"""Valve KeyValues (VDF/ACF) and DepotDownloader manifest parsers.

Pure Python stdlib helpers used by :mod:`port_acquire.backends.steam`.
Leading underscore keeps this module invisible to :mod:`port_acquire.registry`
backend auto-discovery.
"""

from __future__ import annotations

from pathlib import Path
import re
import struct
from typing import Any


# ---------------------------------------------------------------------------
# Steamworks Common Redistributables (AppID 228980) shared depot catalog
# ---------------------------------------------------------------------------

COMMON_REDIST_APP_ID = "228980"

COMMON_REDIST_DEPOTS: dict[str, dict[str, Any]] = {
    "228981": {
        "name": "Visual C++ 2005 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228982": {
        "name": "Visual C++ 2008 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228983": {
        "name": "Visual C++ 2010 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228984": {
        "name": "Visual C++ 2012 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228985": {
        "name": "Visual C++ 2013 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228986": {
        "name": "Visual C++ 2015 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228987": {
        "name": "Visual C++ 2017 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228988": {
        "name": "Visual C++ 2019 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228989": {
        "name": "Visual C++ 2022 Redistributable",
        "category": "vcredist",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt UCRT/MSVC runtime",
    },
    "228990": {
        "name": "DirectX June 2010 Redistributable",
        "category": "directx",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt D3D11/D3D12 + crates/d3d12-metal",
    },
    "229000": {
        "name": ".NET Framework 3.5 Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229001": {
        "name": ".NET Framework 3.5 Client Profile Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229002": {
        "name": ".NET Framework 4.0 Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229003": {
        "name": ".NET Framework 4.0 Client Profile Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229004": {
        "name": ".NET Framework 4.5.2 Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229005": {
        "name": ".NET Framework 4.6.2 Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229006": {
        "name": ".NET Framework 4.7.2 Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229007": {
        "name": ".NET Framework 4.8 Redistributable",
        "category": "dotnet",
        "native_in_win32_rt": False,
        "notes": "Inspect with `cargo opsdb -- dotnet-audit`",
    },
    "229010": {
        "name": "Microsoft XNA Framework 3.0 Redistributable",
        "category": "other",
        "native_in_win32_rt": False,
        "notes": "Legacy XNA runtime; inspect managed assemblies with `dotnet-audit`",
    },
    "229011": {
        "name": "Microsoft XNA Framework 3.1 Redistributable",
        "category": "other",
        "native_in_win32_rt": False,
        "notes": "Legacy XNA runtime; inspect managed assemblies with `dotnet-audit`",
    },
    "229012": {
        "name": "Microsoft XNA Framework 4.0 Redistributable",
        "category": "other",
        "native_in_win32_rt": False,
        "notes": "Legacy XNA runtime; inspect managed assemblies with `dotnet-audit`",
    },
    "229020": {
        "name": "OpenAL 2.0.7.0 Redistributable",
        "category": "other",
        "native_in_win32_rt": True,
        "notes": "Satisfied natively by crates/win32-rt OpenAL CoreAudio shim",
    },
    "229030": {
        "name": "NVIDIA PhysX System Software",
        "category": "physx",
        "native_in_win32_rt": False,
        "notes": "Shipped PhysX DLLs typically run via ARM64 JIT; configure [middleware.physx] if needed",
    },
    "229033": {
        "name": "Steamworks Shared Runtime Component",
        "category": "other",
        "native_in_win32_rt": True,
        "notes": "Shared Steamworks runtime component",
    },
}


# ---------------------------------------------------------------------------
# Valve KeyValues (.vdf / .acf) Parser
# ---------------------------------------------------------------------------


def _tokenize_vdf(text: str) -> list[tuple[str, str]]:
    tokens: list[tuple[str, str]] = []
    n = len(text)
    i = 0
    while i < n:
        ch = text[i]
        if ch.isspace():
            i += 1
            continue
        if ch == "/" and i + 1 < n and text[i + 1] == "/":
            nl = text.find("\n", i)
            if nl == -1:
                break
            i = nl + 1
            continue
        if ch == "{":
            tokens.append(("LBRACE", "{"))
            i += 1
            continue
        if ch == "}":
            tokens.append(("RBRACE", "}"))
            i += 1
            continue
        if ch == '"':
            i += 1
            buf: list[str] = []
            while i < n:
                c = text[i]
                if c == "\\" and i + 1 < n:
                    nxt = text[i + 1]
                    if nxt == "n":
                        buf.append("\n")
                    elif nxt == "t":
                        buf.append("\t")
                    elif nxt in ('"', "\\"):
                        buf.append(nxt)
                    else:
                        # Preserve Windows path backslashes (e.g. "bin\x64\Game.exe").
                        buf.append("\\")
                        buf.append(nxt)
                    i += 2
                    continue
                if c == '"':
                    i += 1
                    break
                buf.append(c)
                i += 1
            tokens.append(("STR", "".join(buf)))
            continue
        start = i
        while i < n and not text[i].isspace() and text[i] not in ("{", "}", '"'):
            i += 1
        if i > start:
            tokens.append(("STR", text[start:i]))
        else:
            i += 1
    return tokens


def _parse_vdf_map(tokens: list[tuple[str, str]], pos: list[int]) -> dict[str, Any]:
    out: dict[str, Any] = {}
    total = len(tokens)
    while pos[0] < total:
        kind, val = tokens[pos[0]]
        if kind == "RBRACE":
            pos[0] += 1
            break
        if kind == "LBRACE":
            pos[0] += 1
            _parse_vdf_map(tokens, pos)
            continue
        key = val
        pos[0] += 1
        if pos[0] >= total:
            break
        nxt_kind, nxt_val = tokens[pos[0]]
        if nxt_kind == "LBRACE":
            pos[0] += 1
            child = _parse_vdf_map(tokens, pos)
            existing = out.get(key)
            if isinstance(existing, dict):
                existing.update(child)
            else:
                out[key] = child
        elif nxt_kind == "STR":
            pos[0] += 1
            out[key] = nxt_val
        elif nxt_kind == "RBRACE":
            break
    return out


def parse_vdf(text: str) -> dict[str, Any]:
    """Parse a Valve KeyValues (.vdf / .acf) text document into nested dicts/strings."""
    tokens = _tokenize_vdf(text)
    pos = [0]
    return _parse_vdf_map(tokens, pos)


def vdf_get(node: Any, *keys: str, default: Any = None) -> Any:
    """Case-insensitive nested lookup inside a parsed VDF dictionary."""
    cur = node
    for key in keys:
        if not isinstance(cur, dict):
            return default
        if key in cur:
            cur = cur[key]
            continue
        low = key.lower()
        matched = None
        for k, v in cur.items():
            if k.lower() == low:
                matched = v
                break
        if matched is None:
            return default
        cur = matched
    return cur


# ---------------------------------------------------------------------------
# DepotDownloader `-manifest-only` Parsers
# ---------------------------------------------------------------------------

_MANIFEST_HEADER_DEPOT_RE = re.compile(r"Content\s+Manifest\s+for\s+Depot\s+(\d+)", re.I)
_MANIFEST_ID_DATE_RE = re.compile(r"Manifest\s+ID\s*/\s*date\s*:\s*(\d+)(?:\s*/\s*(.+))?", re.I)
_MANIFEST_NUM_FILES_RE = re.compile(r"Total\s+number\s+of\s+files\s*:\s*(\d+)", re.I)
_MANIFEST_DISK_BYTES_RE = re.compile(r"Total\s+bytes\s+on\s+disk\s*:\s*(\d+)", re.I)
_MANIFEST_COMP_BYTES_RE = re.compile(r"Total\s+bytes\s+compressed\s*:\s*(\d+)", re.I)
# File rows: "<size> <chunks> <sha1_hex> <flags_hex> <relative_path>"
# DepotDownloader formats `flags` as lowercase hex (`:x`, e.g. `40` for Directory = 0x40).
_MANIFEST_FILE_ROW_RE = re.compile(
    r"^\s*(\d+)\s+(\d+)\s+([0-9a-fA-F]{40})\s+([0-9a-fA-F]+)\s+(.+?)\s*$"
)
_MANIFEST_FNAME_RE = re.compile(r"manifest_(\d+)_(\d+)\.txt$", re.I)


def parse_depot_manifest_txt(text: str, *, filename: str = "") -> dict[str, Any]:
    """Parse a human-readable `DepotDownloader -manifest-only` text file."""
    depot_id: str | None = None
    manifest_gid: str | None = None
    creation_date: str | None = None
    file_count: int | None = None
    disk_bytes: int | None = None
    compressed_bytes: int | None = None
    files: list[dict[str, Any]] = []

    if filename:
        fm = _MANIFEST_FNAME_RE.search(filename)
        if fm:
            depot_id = fm.group(1)
            manifest_gid = fm.group(2)

    for line in text.splitlines():
        m = _MANIFEST_HEADER_DEPOT_RE.search(line)
        if m:
            depot_id = m.group(1)
            continue
        m = _MANIFEST_ID_DATE_RE.search(line)
        if m:
            manifest_gid = m.group(1)
            creation_date = (m.group(2) or "").strip() or None
            continue
        m = _MANIFEST_NUM_FILES_RE.search(line)
        if m:
            file_count = int(m.group(1))
            continue
        m = _MANIFEST_DISK_BYTES_RE.search(line)
        if m:
            disk_bytes = int(m.group(1))
            continue
        m = _MANIFEST_COMP_BYTES_RE.search(line)
        if m:
            compressed_bytes = int(m.group(1))
            continue
        m = _MANIFEST_FILE_ROW_RE.match(line)
        if m:
            rel_path = m.group(5).replace("\\", "/")
            files.append(
                {
                    "size_bytes": int(m.group(1)),
                    "chunks": int(m.group(2)),
                    "sha1": m.group(3).lower(),
                    "flags": int(m.group(4), 16),
                    "path": rel_path,
                }
            )

    if disk_bytes is None and files:
        disk_bytes = sum(f["size_bytes"] for f in files)

    return {
        "depot_id": depot_id,
        "manifest_gid": manifest_gid,
        "creation_date": creation_date,
        "file_count": file_count if file_count is not None else len(files),
        "disk_size_bytes": disk_bytes,
        "download_size_bytes": compressed_bytes,
        "has_file_manifest": True,
        "files": files,
    }


_DD_BRANCH_RE = re.compile(r"Using\s+app\s+branch\s*:\s*'([^']+)'", re.I)
_DD_PROCESSING_DEPOT_RE = re.compile(r"Processing\s+depot\s+(\d+)(?:\s*-\s*(.+))?", re.I)
_DD_MANIFEST_RE = re.compile(r"Manifest\s+(\d+)\s+\(([^)]+)\)", re.I)
_DD_TOTAL_DL_RE = re.compile(r"Total\s+downloaded\s*:\s*(\d+)\s+bytes", re.I)


def parse_depot_downloader_stdout(output: str) -> dict[str, Any]:
    """Extract branch, depot IDs, manifest IDs, and download summary from DepotDownloader stdout."""
    branch: str | None = None
    depots: list[dict[str, Any]] = []
    cur_depot: dict[str, Any] | None = None
    total_downloaded_bytes: int | None = None

    for line in output.splitlines():
        bm = _DD_BRANCH_RE.search(line)
        if bm:
            branch = bm.group(1).strip()
        pm = _DD_PROCESSING_DEPOT_RE.search(line)
        if pm:
            cur_depot = {
                "depot_id": pm.group(1),
                "name": (pm.group(2) or "").strip(),
                "manifest_gid": None,
                "date": None,
            }
            depots.append(cur_depot)
            continue
        mm = _DD_MANIFEST_RE.search(line)
        if mm and cur_depot is not None:
            cur_depot["manifest_gid"] = mm.group(1)
            cur_depot["date"] = mm.group(2).strip()
            continue
        tm = _DD_TOTAL_DL_RE.search(line)
        if tm:
            total_downloaded_bytes = int(tm.group(1))

    return {
        "branch": branch,
        "depots": depots,
        "total_downloaded_bytes": total_downloaded_bytes,
    }


# ---------------------------------------------------------------------------
# Local Steam Library VDF / ACF Parsers (Strictly Non-Secret Metadata)
# ---------------------------------------------------------------------------


def parse_loginusers_vdf(path: Path) -> list[dict[str, Any]]:
    """Parse Steam's `config/loginusers.vdf` returning account names and SteamID64 only.

    Never reads or returns any password hashes, tokens, or sentry data.
    """
    if not path.is_file():
        return []
    try:
        raw = path.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        return []
    parsed = parse_vdf(raw)
    users_node = vdf_get(parsed, "users")
    if not isinstance(users_node, dict):
        return []
    users: list[dict[str, Any]] = []
    for steam_id64, entry in users_node.items():
        if not isinstance(entry, dict):
            continue
        account_name = str(vdf_get(entry, "AccountName", default="") or "").strip()
        persona_name = str(vdf_get(entry, "PersonaName", default="") or "").strip()
        most_recent = str(vdf_get(entry, "MostRecent", default="0")) == "1"
        allow_auto_login = str(vdf_get(entry, "AllowAutoLogin", default="0")) == "1"
        remember_password = str(vdf_get(entry, "RememberPassword", default="0")) == "1"
        if not account_name and not persona_name:
            continue
        users.append(
            {
                "steam_id64": str(steam_id64),
                "account_name": account_name,
                "persona_name": persona_name,
                "most_recent": most_recent,
                "allow_auto_login": allow_auto_login,
                "remember_password": remember_password,
            }
        )
    users.sort(key=lambda u: (not u["most_recent"], u["account_name"].lower()))
    return users


_STEAM_ID64_BASE = 76561197960265728


def steam_id64_to_account_id32(steam_id64: str | None) -> str | None:
    """Convert a 64-bit SteamID string to Steam's 32-bit `userdata/<account_id>` folder name."""
    if not steam_id64:
        return None
    raw = str(steam_id64).strip()
    if not raw.isdigit():
        return None
    val = int(raw)
    if val <= _STEAM_ID64_BASE:
        return None
    return str(val - _STEAM_ID64_BASE)


def parse_local_owned_app_ids(
    steam_home: Path,
    *,
    steam_id64: str | None = None,
) -> set[str]:
    """Collect owned Steam AppIDs from local Steam `userdata` caches scoped to one account.

    When `steam_id64` is supplied, only `userdata/<account_id32>` is scanned.
    When `steam_id64` is omitted, a single numeric `userdata/<id>` directory is
    scanned; multiple `userdata/*` directories are never merged together.
    """
    owned: set[str] = set()
    userdata = steam_home / "userdata"
    if not userdata.is_dir():
        return owned

    user_dirs: list[Path] = []
    if steam_id64 is not None:
        account_id32 = steam_id64_to_account_id32(steam_id64)
        if account_id32 is None:
            return owned
        cand = userdata / account_id32
        if cand.is_dir():
            user_dirs = [cand]
    else:
        numeric_dirs = [p for p in sorted(userdata.iterdir()) if p.is_dir() and p.name.isdigit()]
        if len(numeric_dirs) == 1:
            user_dirs = numeric_dirs

    for user_dir in user_dirs:
        cfg_dir = user_dir / "config"
        if not cfg_dir.is_dir():
            continue
        cache_dir = cfg_dir / "librarycache"
        if cache_dir.is_dir():
            for cf in cache_dir.iterdir():
                if cf.stem.isdigit():
                    owned.add(cf.stem)
        localcfg = cfg_dir / "localconfig.vdf"
        if localcfg.is_file():
            try:
                parsed = parse_vdf(localcfg.read_text(encoding="utf-8", errors="ignore"))
                apps = vdf_get(parsed, "UserLocalConfigStore", "Software", "Valve", "Steam", "apps")
                if isinstance(apps, dict):
                    for aid in apps:
                        if str(aid).isdigit():
                            owned.add(str(aid))
            except OSError:
                pass
    return owned


_APPINFO_MAGIC_V27 = 0x07564427
_APPINFO_MAGIC_V28 = 0x07564428
_APPINFO_MAGIC_V29 = 0x07564429


def _read_cstr(buf: bytes, pos: int) -> tuple[str, int]:
    end = buf.find(b"\x00", pos)
    if end == -1:
        raise ValueError("unterminated binary VDF string")
    return buf[pos:end].decode("utf-8", errors="replace"), end + 1


def _parse_binary_vdf_map(
    buf: bytes,
    pos: int,
    string_table: list[str] | None,
) -> tuple[dict[str, Any], int]:
    out: dict[str, Any] = {}
    n = len(buf)
    while pos < n:
        type_byte = buf[pos]
        pos += 1
        if type_byte == 0x08:
            break
        if string_table is not None:
            if pos + 4 > n:
                break
            idx = struct.unpack_from("<I", buf, pos)[0]
            pos += 4
            if idx >= len(string_table):
                break
            key = string_table[idx]
        else:
            key, pos = _read_cstr(buf, pos)

        if type_byte == 0x00:
            child, pos = _parse_binary_vdf_map(buf, pos, string_table)
            out[key] = child
        elif type_byte == 0x01:
            val, pos = _read_cstr(buf, pos)
            out[key] = val
        elif type_byte == 0x02:
            if pos + 4 > n:
                break
            out[key] = str(struct.unpack_from("<i", buf, pos)[0])
            pos += 4
        elif type_byte in (0x03, 0x04, 0x06):
            pos += 4
        elif type_byte == 0x05:
            while pos + 1 < n and buf[pos : pos + 2] != b"\x00\x00":
                pos += 2
            pos += 2
        elif type_byte == 0x07:
            if pos + 8 > n:
                break
            out[key] = str(struct.unpack_from("<Q", buf, pos)[0])
            pos += 8
        else:
            break
    return out, pos


def _normalize_steam_os(platform: str) -> str:
    low = platform.strip().lower()
    if low in ("win32", "win64", "windows", "win"):
        return "windows"
    if low in ("osx", "macos", "darwin", "mac"):
        return "macos"
    return low


def _depot_config_matches(
    config: Any,
    *,
    platform: str | None = None,
    osarch: str | None = None,
    language: str | None = None,
    lowviolence: bool = False,
) -> bool:
    """Apply Steamworks depot `config` filters mirroring DepotDownloader's selection logic."""
    if not isinstance(config, dict):
        return True
    if platform is not None:
        raw_os = str(vdf_get(config, "oslist", default="") or "").strip()
        if raw_os:
            target_os = _normalize_steam_os(platform)
            allowed = {_normalize_steam_os(part) for part in raw_os.split(",") if part.strip()}
            if allowed and target_os not in allowed:
                return False
    if osarch is not None:
        raw_arch = str(vdf_get(config, "osarch", default="") or "").strip().lower()
        if raw_arch and raw_arch != osarch.strip().lower():
            return False
    if language is not None:
        raw_lang = str(vdf_get(config, "language", default="") or "").strip().lower()
        if raw_lang and raw_lang != language.strip().lower():
            return False
    raw_lv = str(vdf_get(config, "lowviolence", default="") or "").strip().lower()
    if raw_lv in ("1", "true") and not lowviolence:
        return False
    return True


def parse_appinfo_vdf_dlc_depots(
    steam_home: Path,
    app_id: str,
    *,
    platform: str | None = None,
    osarch: str | None = None,
    language: str | None = None,
    lowviolence: bool = False,
) -> dict[str, list[str]]:
    """Read Steam's local binary `appcache/appinfo.vdf` to map `dlc_app_id -> [depot_ids]`.

    Supports binary `appinfo.vdf` v27, v28, and v29 (read-only, zero credentials).
    When `platform`, `osarch`, or `language` is supplied, depots whose `config`
    excludes the requested target are filtered out so explicit `-depot` selection
    never downloads another platform/architecture/language's payload.
    Returns an empty dict when `appinfo.vdf` is absent or does not contain `app_id`.
    """
    if not str(app_id).isdigit():
        return {}
    path = steam_home / "appcache" / "appinfo.vdf"
    if not path.is_file():
        return {}
    try:
        data = path.read_bytes()
    except OSError:
        return {}
    if len(data) < 8:
        return {}

    magic, _universe = struct.unpack_from("<II", data, 0)
    if magic not in (_APPINFO_MAGIC_V27, _APPINFO_MAGIC_V28, _APPINFO_MAGIC_V29):
        return {}

    pos = 8
    string_table: list[str] | None = None
    if magic == _APPINFO_MAGIC_V29:
        if len(data) < 16:
            return {}
        str_off = struct.unpack_from("<q", data, 8)[0]
        pos = 16
        if str_off <= 0 or str_off + 4 > len(data):
            return {}
        str_count = struct.unpack_from("<I", data, str_off)[0]
        st_pos = str_off + 4
        string_table = []
        try:
            for _ in range(str_count):
                s, st_pos = _read_cstr(data, st_pos)
                string_table.append(s)
        except ValueError:
            return {}

    entry_hdr_len = 40 if magic == _APPINFO_MAGIC_V27 else 60
    target_id = int(app_id)
    n = len(data)
    apply_filters = any(x is not None for x in (platform, osarch, language))

    while pos + 8 <= n:
        cur_app_id, entry_size = struct.unpack_from("<II", data, pos)
        pos += 8
        if cur_app_id == 0:
            break
        entry_end = pos + entry_size
        if entry_end > n or entry_size < entry_hdr_len:
            break
        if cur_app_id == target_id:
            payload = data[pos + entry_hdr_len : entry_end]
            try:
                parsed, _ = _parse_binary_vdf_map(payload, 0, string_table)
            except (ValueError, IndexError, struct.error):
                return {}
            depots = vdf_get(parsed, "appinfo", "depots")
            if not isinstance(depots, dict):
                return {}
            dlc_map: dict[str, list[str]] = {}
            for depot_id, depot_body in depots.items():
                if not str(depot_id).isdigit() or not isinstance(depot_body, dict):
                    continue
                if apply_filters and not _depot_config_matches(
                    vdf_get(depot_body, "config"),
                    platform=platform,
                    osarch=osarch,
                    language=language,
                    lowviolence=lowviolence,
                ):
                    continue
                dlc_app_id = str(vdf_get(depot_body, "dlcappid", default="") or "").strip()
                if dlc_app_id.isdigit():
                    dlc_map.setdefault(dlc_app_id, [])
                    if str(depot_id) not in dlc_map[dlc_app_id]:
                        dlc_map[dlc_app_id].append(str(depot_id))
            return dlc_map
        pos = entry_end
    return {}


def parse_libraryfolders_vdf(steam_home: Path) -> list[Path]:
    """Return all existing `steamapps` directories from `steam_home` and `libraryfolders.vdf`."""
    steamapps_dirs: list[Path] = []
    seen: set[Path] = set()

    def _add_candidate(raw_dir: Path) -> None:
        sa = raw_dir if raw_dir.name.lower() == "steamapps" else raw_dir / "steamapps"
        if not sa.is_dir():
            return
        try:
            key = sa.resolve()
        except OSError:
            key = sa
        if key not in seen:
            seen.add(key)
            steamapps_dirs.append(sa)

    _add_candidate(steam_home)

    for vdf_path in (
        steam_home / "steamapps" / "libraryfolders.vdf",
        steam_home / "config" / "libraryfolders.vdf",
    ):
        if not vdf_path.is_file():
            continue
        try:
            parsed = parse_vdf(vdf_path.read_text(encoding="utf-8", errors="ignore"))
        except OSError:
            continue
        folders = vdf_get(parsed, "libraryfolders")
        if not isinstance(folders, dict):
            continue
        for key, val in folders.items():
            raw_path: str | None = None
            if isinstance(val, dict):
                p = vdf_get(val, "path", default="")
                if isinstance(p, str) and p.strip():
                    raw_path = p.strip()
            elif isinstance(val, str) and str(key).isdigit() and val.strip():
                raw_path = val.strip()
            if raw_path:
                _add_candidate(Path(raw_path).expanduser())

    return steamapps_dirs


def parse_appmanifest_acf(path: Path) -> dict[str, Any] | None:
    """Parse a Steam `appmanifest_<appid>.acf` file."""
    if not path.is_file():
        return None
    try:
        raw = path.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        return None
    parsed = parse_vdf(raw)
    state = vdf_get(parsed, "AppState")
    if not isinstance(state, dict):
        return None
    app_id = str(vdf_get(state, "appid", default="") or "").strip()
    if not app_id:
        return None
    mounted = vdf_get(state, "MountedDepots")
    depots = (
        {str(k): str(v) for k, v in mounted.items() if isinstance(v, str)}
        if isinstance(mounted, dict)
        else {}
    )
    user_cfg = vdf_get(state, "UserConfig")
    branch = str(vdf_get(user_cfg, "betakey", default="") or "").strip() or "public"
    size_str = str(vdf_get(state, "SizeOnDisk", default="") or "").strip()
    last_owner = str(vdf_get(state, "LastOwner", default="") or "").strip() or None
    return {
        "app_id": app_id,
        "name": str(vdf_get(state, "name", default=app_id) or app_id),
        "installdir": str(vdf_get(state, "installdir", default="") or ""),
        "build_id": str(vdf_get(state, "buildid", default="") or "") or None,
        "size_on_disk_bytes": int(size_str) if size_str.isdigit() else None,
        "branch": branch,
        "last_owner": last_owner,
        "mounted_depots": depots,
        "manifest_path": str(path),
    }


def parse_installscript_vdf(text: str) -> dict[str, Any]:
    """Extract `Registry` keys and `Run Process` entries from Steam's `installscript.vdf`."""
    parsed = parse_vdf(text)
    run_processes: list[dict[str, str]] = []
    registry_keys: list[str] = []

    for _, root_val in parsed.items():
        if not isinstance(root_val, dict):
            continue
        for sec_name, sec_val in root_val.items():
            if not isinstance(sec_val, dict):
                continue
            low = sec_name.lower()
            if low == "run process":
                for proc_name, proc_body in sec_val.items():
                    if not isinstance(proc_body, dict):
                        continue
                    proc_exe = str(
                        vdf_get(proc_body, "process 1", default="")
                        or vdf_get(proc_body, "process", default="")
                        or ""
                    ).strip()
                    proc_args = str(
                        vdf_get(proc_body, "command 1", default="")
                        or vdf_get(proc_body, "command", default="")
                        or ""
                    ).strip()
                    run_processes.append(
                        {
                            "name": str(proc_name),
                            "process": proc_exe,
                            "args": proc_args,
                        }
                    )
            elif low == "registry":
                for reg_path in sec_val:
                    registry_keys.append(str(reg_path))
    return {
        "run_processes": run_processes,
        "registry_keys": registry_keys,
    }


def infer_common_redist_depots_from_paths(paths: list[str]) -> list[str]:
    """Map discovered `_CommonRedist` / runtime file paths to App `228980` shared depot IDs."""
    inferred: list[str] = []
    joined = "\n".join(p.lower() for p in paths)

    def _add(did: str) -> None:
        if did not in inferred:
            inferred.append(did)

    if re.search(r"vc_?redist|vcredist|msvcp140|vcruntime140", joined):
        if "2010" in joined:
            _add("228983")
        if "2012" in joined:
            _add("228984")
        if "2013" in joined:
            _add("228985")
        if "2015" in joined or "2017" in joined or "2019" in joined or "2022" in joined or not inferred:
            _add("228989")
    if re.search(r"dxsetup|dxwebsetup|directx|d3dx9_|d3dcompiler_", joined):
        _add("228990")
    if re.search(r"dotnet|ndp4|mscoree", joined):
        _add("229007")
    if re.search(r"oalinst|openal", joined):
        _add("229020")
    if re.search(r"physx", joined):
        _add("229030")
    return inferred


def extract_proton_redist_info(
    *,
    depot_ids: list[str],
    manifest_paths: list[str],
    install_dir: Path | None = None,
) -> dict[str, Any]:
    """Extract Steamworks Common Redistributables (`228980`) and Proton-adjacent redist metadata."""
    shared_ids: list[str] = []
    for did in depot_ids:
        if did in COMMON_REDIST_DEPOTS and did not in shared_ids:
            shared_ids.append(did)
    for did in infer_common_redist_depots_from_paths(manifest_paths):
        if did not in shared_ids:
            shared_ids.append(did)

    shared_depots: list[dict[str, Any]] = []
    for did in shared_ids:
        meta = COMMON_REDIST_DEPOTS.get(did, {})
        shared_depots.append(
            {
                "depot_id": did,
                "depot_from_app": COMMON_REDIST_APP_ID,
                "shared_install": True,
                "name": meta.get("name", f"Steamworks Common Redistributable ({did})"),
                "category": meta.get("category", "other"),
                "native_in_win32_rt": bool(meta.get("native_in_win32_rt", False)),
                "notes": meta.get("notes", ""),
            }
        )

    common_redist_dirs: list[str] = []
    for p in manifest_paths:
        parts = Path(p.replace("\\", "/")).parts
        if parts and parts[0].lower() in ("_commonredist", "redist", "support"):
            if parts[0] not in common_redist_dirs:
                common_redist_dirs.append(parts[0])

    install_scripts: list[dict[str, Any]] = []
    if install_dir is not None and install_dir.is_dir():
        for entry in sorted(install_dir.iterdir(), key=lambda p: p.name.lower()):
            if entry.is_symlink():
                continue
            low = entry.name.lower()
            if entry.is_dir() and low in ("_commonredist", "redist", "support"):
                rel = entry.relative_to(install_dir).as_posix()
                if rel not in common_redist_dirs:
                    common_redist_dirs.append(rel)
            elif entry.is_file() and low.endswith(".vdf") and "install" in low:
                try:
                    raw = entry.read_text(encoding="utf-8", errors="ignore")
                    parsed_script = parse_installscript_vdf(raw)
                    install_scripts.append(
                        {
                            "relative_path": entry.relative_to(install_dir).as_posix(),
                            **parsed_script,
                        }
                    )
                except OSError:
                    pass

    proton_notes: list[str] = []
    if any(d["native_in_win32_rt"] for d in shared_depots):
        proton_notes.append(
            "Shared VC++/DirectX/OpenAL redistributables (App 228980) are satisfied natively by crates/win32-rt."
        )
    if any(d["category"] == "dotnet" for d in shared_depots):
        proton_notes.append(
            ".NET redistributable detected; run `cargo opsdb -- dotnet-audit <game-dir>`."
        )
    if any(d["category"] == "physx" for d in shared_depots):
        proton_notes.append(
            "PhysX redistributable detected; configure [middleware.physx] if needed."
        )

    return {
        "shared_redist_app_id": COMMON_REDIST_APP_ID if shared_depots else None,
        "shared_depots": shared_depots,
        "common_redist_dirs": common_redist_dirs,
        "install_scripts": install_scripts,
        "proton_notes": proton_notes,
    }
