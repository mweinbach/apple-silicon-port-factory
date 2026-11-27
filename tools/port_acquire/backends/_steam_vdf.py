"""Valve KeyValues (VDF/ACF) and SteamCMD text output parsers.

Pure Python stdlib helpers used by :mod:`port_acquire.backends.steam`.
Leading underscore keeps this module invisible to :mod:`port_acquire.registry`
backend auto-discovery.
"""

from __future__ import annotations

from pathlib import Path
import re
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

STEAM_DECK_CATEGORIES = {
    "0": "unknown",
    "1": "unsupported",
    "2": "playable",
    "3": "verified",
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
                        # Valve's `steamcmd +app_info_print` writes Windows paths with single
                        # backslashes (e.g. "bin\x64\Game.exe"); preserve the separator.
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
        while i < n and not text[i].isspace() and text[i] not in ('{', '}', '"'):
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
        # STR key
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
# SteamCMD Output Parsers
# ---------------------------------------------------------------------------

_APP_INFO_HEADER_RE = re.compile(
    r"AppID\s*:\s*(\d+)\s*,\s*change number\s*:\s*(\d+)", re.IGNORECASE
)


def extract_appinfo_vdf(steamcmd_output: str, app_id: str) -> tuple[dict[str, Any], str | None]:
    """Extract the parsed VDF dict and optional change number from `steamcmd +app_info_print`."""
    change_number: str | None = None
    m_hdr = _APP_INFO_HEADER_RE.search(steamcmd_output)
    if m_hdr and m_hdr.group(1) == str(app_id):
        change_number = m_hdr.group(2)

    # Locate `"<app_id>"` followed by `{` on the same or next line.
    pattern = re.compile(rf'"{re.escape(str(app_id))}"\s*\{{')
    match = pattern.search(steamcmd_output)
    if not match:
        return {}, change_number

    # Scan forward tracking brace depth (skipping quoted strings) to slice the exact VDF block.
    brace_start = steamcmd_output.find("{", match.start())
    depth = 0
    in_quotes = False
    escaped = False
    end_idx = len(steamcmd_output)
    for idx in range(brace_start, len(steamcmd_output)):
        ch = steamcmd_output[idx]
        if in_quotes:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_quotes = False
            continue
        if ch == '"':
            in_quotes = True
        elif ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                end_idx = idx + 1
                break

    snippet = steamcmd_output[match.start() : end_idx]
    parsed = parse_vdf(snippet)
    app_node = vdf_get(parsed, str(app_id))
    if isinstance(app_node, dict):
        return app_node, change_number
    return {}, change_number


_LICENSE_BLOCK_RE = re.compile(r"License\s+packageID\s+(\d+)\s*:", re.IGNORECASE)
_LICENSE_STATE_RE = re.compile(r"State\s*:\s*([^\n-]+)", re.IGNORECASE)
_LICENSE_APPS_RE = re.compile(r"Apps\s*:\s*([0-9,\s]+)", re.IGNORECASE)
_LICENSE_DEPOTS_RE = re.compile(r"Depots\s*:\s*([0-9,\s]+)", re.IGNORECASE)


def parse_licenses_print(steamcmd_output: str) -> list[dict[str, Any]]:
    """Parse `steamcmd +licenses_print` output into package/app/depot records.

    Handles both single-line and multi-line SteamCMD license formats.
    """
    matches = list(_LICENSE_BLOCK_RE.finditer(steamcmd_output))
    results: list[dict[str, Any]] = []
    for idx, match in enumerate(matches):
        pkg_id = match.group(1)
        start = match.end()
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(steamcmd_output)
        chunk = steamcmd_output[start:end]

        state_m = _LICENSE_STATE_RE.search(chunk)
        state = state_m.group(1).strip() if state_m else "Unknown"
        active = state.lower().startswith("active")

        apps: list[str] = []
        apps_m = _LICENSE_APPS_RE.search(chunk)
        if apps_m:
            apps = re.findall(r"\d+", apps_m.group(1))

        depots: list[str] = []
        depots_m = _LICENSE_DEPOTS_RE.search(chunk)
        if depots_m:
            depots = re.findall(r"\d+", depots_m.group(1))

        results.append(
            {
                "package_id": pkg_id,
                "state": state,
                "active": active,
                "app_ids": apps,
                "depot_ids": depots,
            }
        )
    return results


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


def has_cached_connect_token(config_vdf_path: Path, account_name: str | None = None) -> bool:
    """Return True if Steam's `config.vdf` appears to contain a cached login entry.

    Strictly boolean — never returns or logs token material.
    """
    if not config_vdf_path.is_file():
        return False
    try:
        raw = config_vdf_path.read_text(encoding="utf-8", errors="ignore")
    except OSError:
        return False
    parsed = parse_vdf(raw)
    connect_cache = vdf_get(parsed, "InstallConfigStore", "Software", "Valve", "Steam", "ConnectCache")
    if isinstance(connect_cache, dict) and any(str(v).strip() for v in connect_cache.values()):
        return True
    accounts = vdf_get(parsed, "InstallConfigStore", "Software", "Valve", "Steam", "Accounts")
    if isinstance(accounts, dict):
        if account_name:
            for k in accounts:
                if k.lower() == account_name.lower():
                    return True
        elif accounts:
            return True
    return False


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
    return {
        "app_id": app_id,
        "name": str(vdf_get(state, "name", default=app_id) or app_id),
        "installdir": str(vdf_get(state, "installdir", default="") or ""),
        "build_id": str(vdf_get(state, "buildid", default="") or "") or None,
        "size_on_disk_bytes": int(size_str) if size_str.isdigit() else None,
        "branch": branch,
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


def extract_proton_redist_info(
    appinfo: dict[str, Any],
    install_dir: Path | None = None,
) -> dict[str, Any]:
    """Extract Steamworks Common Redistributables (`228980`) and Proton-adjacent metadata."""
    depots_node = vdf_get(appinfo, "depots")
    shared_depots: list[dict[str, Any]] = []
    if isinstance(depots_node, dict):
        for depot_id, depot_val in depots_node.items():
            if not depot_id.isdigit() or not isinstance(depot_val, dict):
                continue
            depot_from_app = str(vdf_get(depot_val, "depotfromapp", default="") or "").strip()
            shared_install = str(vdf_get(depot_val, "sharedinstall", default="0")) == "1"
            if depot_from_app == COMMON_REDIST_APP_ID or depot_id in COMMON_REDIST_DEPOTS:
                meta = COMMON_REDIST_DEPOTS.get(depot_id, {})
                shared_depots.append(
                    {
                        "depot_id": depot_id,
                        "depot_from_app": depot_from_app or COMMON_REDIST_APP_ID,
                        "shared_install": shared_install or True,
                        "name": meta.get("name", f"Steamworks Common Redistributable ({depot_id})"),
                        "category": meta.get("category", "other"),
                        "native_in_win32_rt": bool(meta.get("native_in_win32_rt", False)),
                        "notes": meta.get("notes", ""),
                    }
                )

    deck_node = vdf_get(appinfo, "common", "steam_deck_compatibility")
    steam_deck_compat: dict[str, Any] = {}
    if isinstance(deck_node, dict):
        cat_raw = str(vdf_get(deck_node, "category", default="0") or "0")
        cfg = vdf_get(deck_node, "configuration")
        rec_runtime = ""
        if isinstance(cfg, dict):
            rec_runtime = str(vdf_get(cfg, "recommended_runtime", default="") or "")
        steam_deck_compat = {
            "category": STEAM_DECK_CATEGORIES.get(cat_raw, "unknown"),
            "category_code": cat_raw,
            "recommended_runtime": rec_runtime or None,
        }

    common_redist_dirs: list[str] = []
    install_scripts: list[dict[str, Any]] = []
    if install_dir is not None and install_dir.is_dir():
        for entry in sorted(install_dir.iterdir(), key=lambda p: p.name.lower()):
            if entry.is_symlink():
                continue
            low = entry.name.lower()
            if entry.is_dir() and low in ("_commonredist", "redist", "support"):
                common_redist_dirs.append(entry.relative_to(install_dir).as_posix())
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
            "Shared VC++/DirectX/OpenAL depots from App 228980 are satisfied natively by crates/win32-rt."
        )
    if any(d["category"] == "dotnet" for d in shared_depots):
        proton_notes.append(
            ".NET redistributable referenced in App 228980; run `cargo opsdb -- dotnet-audit <game-dir>`."
        )
    if steam_deck_compat.get("recommended_runtime"):
        proton_notes.append(
            f"Valve Steam Deck compatibility metadata references runtime: {steam_deck_compat['recommended_runtime']}."
        )

    return {
        "shared_redist_app_id": COMMON_REDIST_APP_ID if shared_depots else None,
        "shared_depots": shared_depots,
        "common_redist_dirs": common_redist_dirs,
        "install_scripts": install_scripts,
        "steam_deck_compatibility": steam_deck_compat,
        "proton_notes": proton_notes,
    }
