"""Acquisition package manifest (`port-acquire-manifest.json`) & secret redaction.

Hook point for `.app` packaging:
`tools/port-acquire download` and `tools/port-acquire launch-args --export-manifest`
write `port-acquire-manifest.json` (schema v1) alongside the downloaded Windows game
files. `crates/port-factory/src/steam_library/{macos_bundle,vdf}.rs` can consume this
JSON file in a future packaging pass to:
  1. Resolve the primary Windows executable without heuristic guessing.
  2. Copy `port-acquire-manifest.json` into `Contents/Resources/`.
  3. Seed non-secret `launch_args` into `Contents/Resources/game.toml` `[runtime].args`.
  4. Skip `native_in_win32_rt: true` redistributables (VC++, DirectX) while preserving
     EOS SDK and DLC records for `bundle-verify`.
"""

from __future__ import annotations

from datetime import datetime, timezone
import json
from pathlib import Path
import re
from typing import Any

from .backend import (
    AnticheatReport,
    DlcItem,
    DrmReport,
    GameInfo,
    LaunchPlan,
    RedistributableItem,
    sanitize_dirname,
)

MANIFEST_FILENAME = "port-acquire-manifest.json"
MANIFEST_SCHEMA_VERSION = 1

# Exact or prefix launch flags that can carry Epic/store auth secrets or tokens.
_SECRET_ARG_PREFIXES = (
    "-auth_password=",
    "-auth_login=",
    "-auth_type=",
    "-auth_exchangecode=",
    "-epicuserid=",
    "-epicusername=",
    "--token=",
    "--password=",
    "--secret=",
    "--sid=",
    "--auth-code=",
)

_SECRET_ARG_FLAGS_WITH_VALUE = {
    "-auth_password",
    "-auth_login",
    "-auth_type",
    "-auth_exchangecode",
    "-epicuserid",
    "--token",
    "--password",
    "--secret",
    "--sid",
    "--auth-code",
}

_SECRET_ENV_PATTERNS = re.compile(
    r"(TOKEN|PASSWORD|SECRET|COOKIE|AUTH|CREDENTIAL|EXCHANGE_CODE|SESSION|SID)",
    re.IGNORECASE,
)


def redact_launch_args(args: list[str]) -> tuple[list[str], bool]:
    """Strip any secret/token-carrying launch arguments so manifests stay secret-free.

    Returns `(sanitized_args, redacted_any)`.
    """
    sanitized: list[str] = []
    redacted = False
    skip_next = False
    for arg in args:
        if skip_next:
            skip_next = False
            redacted = True
            continue
        low = arg.lower()
        if any(low.startswith(prefix) for prefix in _SECRET_ARG_PREFIXES):
            redacted = True
            continue
        if low in _SECRET_ARG_FLAGS_WITH_VALUE:
            skip_next = True
            redacted = True
            continue
        sanitized.append(arg)
    return sanitized, redacted


def redact_env(env: dict[str, str]) -> tuple[dict[str, str], bool]:
    """Remove environment entries whose keys look like credentials or tokens."""
    sanitized: dict[str, str] = {}
    redacted = False
    for key, val in env.items():
        if _SECRET_ENV_PATTERNS.search(key):
            redacted = True
            continue
        sanitized[key] = val
    return sanitized, redacted


def build_package_manifest(
    *,
    info: GameInfo,
    install_dir: str | Path,
    launch_plan: LaunchPlan | None = None,
    eos_overlay_path: str | None = None,
    extra_redists: list[RedistributableItem] | None = None,
) -> dict[str, Any]:
    """Build a schema-v1 `port-acquire-manifest.json` dictionary (strictly secret-free)."""
    install_dir_str = str(install_dir)
    raw_args = (
        list(launch_plan.args)
        if launch_plan is not None
        else list(info.default_launch_args)
    )
    safe_args, args_redacted = redact_launch_args(raw_args)
    if launch_plan is not None and launch_plan.redacted_auth_args:
        args_redacted = True

    raw_env = dict(launch_plan.env) if launch_plan is not None else {}
    safe_env, env_redacted = redact_env(raw_env)

    executable = (
        launch_plan.executable
        if launch_plan is not None and launch_plan.executable
        else (info.executable or "")
    )
    working_dir = (
        launch_plan.working_dir
        if launch_plan is not None and launch_plan.working_dir
        else (Path(executable).parent.as_posix() if executable else ".")
    )
    if working_dir in ("", "."):
        working_dir = "."

    # Deduplicate redistributables by (id, relative_path)
    redists: list[RedistributableItem] = []
    seen_redists: set[tuple[str, str | None]] = set()
    for item in list(info.redistributables) + list(extra_redists or []):
        key = (item.id, item.relative_path)
        if key not in seen_redists:
            seen_redists.add(key)
            redists.append(item)

    if eos_overlay_path:
        overlay_item = RedistributableItem(
            id="eos_overlay",
            name="Epic Online Services Overlay",
            category="eos_sdk",
            source="eos_overlay",
            relative_path=eos_overlay_path,
            native_in_win32_rt=False,
            present_on_disk=Path(eos_overlay_path).exists(),
            notes="Downloaded via `legendary eos-overlay install`",
        )
        key = (overlay_item.id, overlay_item.relative_path)
        if key not in seen_redists:
            redists.append(overlay_item)

    anticheat: AnticheatReport = info.anticheat
    drm: DrmReport = info.drm
    dlcs: list[DlcItem] = info.dlcs

    return {
        "schema_version": MANIFEST_SCHEMA_VERSION,
        "generated_by": "port-acquire",
        "generated_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "store": info.store,
        "app_id": info.app_id,
        "title": info.title,
        "platform": info.platform,
        "version": info.version,
        "install_dir": install_dir_str,
        "executable": executable,
        "working_dir": working_dir,
        "launch_args": safe_args,
        "launch_env": safe_env,
        "redacted_auth_args": bool(args_redacted or env_redacted),
        "dlcs": [d.to_dict() for d in dlcs],
        "redistributables": [r.to_dict() for r in redists],
        "compatibility_signals": {
            "can_run_offline": info.can_run_offline,
            "requires_eos": info.requires_eos,
            "eos_signals": list(info.eos_signals),
            "anticheat": anticheat.to_dict(),
            "drm": drm.to_dict(),
        },
        "packaging_hook": {
            "format": "port-acquire-manifest-v1",
            "expected_bundle_resource": f"Contents/Resources/{MANIFEST_FILENAME}",
            "notes": (
                "Consumed by .app packaging (`crates/port-factory/src/steam_library/"
                "macos_bundle.rs` & `vdf.rs`) to resolve executable, non-secret launch "
                "args, DLCs, and Win32-native vs bundled redistributables."
            ),
        },
    }


def write_package_manifest(manifest: dict[str, Any], path: Path) -> Path:
    """Atomically write `manifest` as pretty-printed UTF-8 JSON."""
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(json.dumps(manifest, indent=2, sort_keys=False) + "\n", encoding="utf-8")
    tmp.replace(path)
    return path


def read_package_manifest(path: Path) -> dict[str, Any]:
    """Read and validate a schema-v1 `port-acquire-manifest.json` file."""
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or data.get("schema_version") != MANIFEST_SCHEMA_VERSION:
        raise ValueError(
            f"unsupported port-acquire manifest at {path} "
            f"(expected schema_version={MANIFEST_SCHEMA_VERSION})"
        )
    return data


def _toml_escape(value: str) -> str:
    return value.replace("\\", "\\\\").replace('"', '\\"')


def render_starter_game_toml(manifest: dict[str, Any]) -> str:
    """Render a game-agnostic starter `game.toml` from a package manifest."""
    title = str(manifest.get("title") or manifest.get("app_id") or "Acquired Game")
    clean = sanitize_dirname(title)
    game_id = re.sub(r"[^a-z0-9]+", "-", clean.lower()).strip("-") or "acquired-game"
    exec_ident = "".join(ch for ch in clean if ch.isalnum()) or "AcquiredGame"
    exe_rel = str(manifest.get("executable") or "Game.exe").replace("\\", "/")
    safe_args, _ = redact_launch_args(list(manifest.get("launch_args") or []))
    args_str = " ".join(
        f'"{a}"' if any(ch.isspace() for ch in a) else a for a in safe_args
    )

    lines = [
        "# Starter manifest generated by `tools/port-acquire`.",
        "# Review with `cargo opsdb -- audit-game <game-dir>` before committing.",
        "",
        "[app]",
        f'id = "{_toml_escape(game_id)}"',
        f'name = "{_toml_escape(title)}"',
        f'exe = "{_toml_escape(exe_rel)}"',
        f'win_dir = "C:\\\\Games\\\\{_toml_escape(exec_ident)}"',
        f'log_name = "{_toml_escape(exec_ident)}.log"',
        'computer_name = "WIN64PORT"',
        "",
        "[runtime]",
        "preferred_base_retry = false",
        "v8_jitless = false",
        "legacy_remedy_migration = false",
    ]
    if args_str:
        lines.append(f'args = "{_toml_escape(args_str)}"')
    lines.append("")
    return "\n".join(lines)
