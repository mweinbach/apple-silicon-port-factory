"""Command-line interface for `tools/port-acquire`."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import sys
from typing import Any, Sequence

from .backend import (
    AcquireError,
    DownloadOptions,
    GameInfo,
    StoreBackend,
    sanitize_dirname,
    scan_dir_redistributables,
)
from .manifest import (
    MANIFEST_FILENAME,
    build_package_manifest,
    render_starter_game_toml,
    write_package_manifest,
)
from .registry import available_backends, get_backend


def _format_bytes(size: int | None) -> str:
    if size is None:
        return "unknown"
    val = float(size)
    for unit in ("B", "KiB", "MiB", "GiB", "TiB"):
        if val < 1024.0 or unit == "TiB":
            return f"{val:.1f} {unit}" if unit != "B" else f"{int(val)} B"
        val /= 1024.0
    return f"{size} B"


def _default_store() -> str:
    env_store = os.environ.get("PORT_ACQUIRE_STORE", "").strip().lower()
    if env_store:
        return env_store
    avail = available_backends()
    if "epic" in avail:
        return "epic"
    return avail[0] if avail else "epic"


def _default_dest(store: str, app_id: str, title: str | None = None) -> Path:
    base_env = os.environ.get("PORT_ACQUIRE_DEST", "").strip()
    if base_env:
        base = Path(base_env).expanduser()
    else:
        base = Path.home() / "Games" / "PortFactory" / store
    folder = sanitize_dirname(title) if title else sanitize_dirname(app_id)
    return base / folder


def _emit(payload: dict[str, Any], *, as_json: bool, human_lines: list[str]) -> None:
    if as_json:
        print(json.dumps(payload, indent=2))
    else:
        for line in human_lines:
            print(line)


def _add_shared_flags(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--store",
        default=argparse.SUPPRESS,
        help="Store backend name (default: $PORT_ACQUIRE_STORE or epic)",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        default=argparse.SUPPRESS,
        help="Emit machine-readable JSON output",
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="port-acquire",
        description=(
            "Acquire legally owned Windows PC games from supported stores for "
            "Apple Silicon Port Factory (zero DRM/anti-cheat bypass)."
        ),
    )
    parser.add_argument(
        "--store",
        default=None,
        help="Store backend name (default: $PORT_ACQUIRE_STORE or epic)",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        default=False,
        help="Emit machine-readable JSON output",
    )

    sub = parser.add_subparsers(dest="command", required=True)

    # stores (helper to inspect plugged-in backends)
    p_stores = sub.add_parser("stores", help="List available store backends")
    _add_shared_flags(p_stores)

    # login
    p_login = sub.add_parser(
        "login",
        help="Authenticate interactively with the store (run by the account owner)",
    )
    _add_shared_flags(p_login)
    p_login.add_argument(
        "--no-webview",
        action="store_true",
        help="Use system browser + authorization code prompt instead of embedded webview",
    )
    p_login.add_argument(
        "--code",
        default=None,
        help="Pass an official authorization code non-interactively (never logged)",
    )
    p_login.add_argument(
        "--import-existing",
        action="store_true",
        help="Import authentication from an existing official launcher install if supported",
    )
    p_login.add_argument(
        "--logout",
        action="store_true",
        help="Remove saved credentials from the backend config (log out)",
    )

    # status
    p_status = sub.add_parser(
        "status",
        help="Report login state and account display name only (never tokens)",
    )
    _add_shared_flags(p_status)
    p_status.add_argument(
        "--offline",
        action="store_true",
        help="Inspect local session state without contacting store servers",
    )

    # list-owned
    p_list = sub.add_parser("list-owned", help="List games owned by the logged-in account")
    _add_shared_flags(p_list)
    p_list.add_argument(
        "--platform",
        default="windows",
        choices=("windows", "macos", "all"),
        help="Target platform filter (default: windows)",
    )
    p_list.add_argument(
        "--refresh",
        action="store_true",
        help="Force refresh metadata from store servers",
    )

    # search
    p_search = sub.add_parser(
        "search",
        help="Fuzzy-match a game title in your owned library to an app_id",
    )
    _add_shared_flags(p_search)
    p_search.add_argument("query", help="Game title or partial name to search for")
    p_search.add_argument(
        "--platform",
        default="windows",
        choices=("windows", "macos", "all"),
        help="Target platform filter (default: windows)",
    )
    p_search.add_argument("--limit", type=int, default=10, help="Maximum matches to return")
    p_search.add_argument(
        "--min-score",
        type=float,
        default=0.35,
        help="Minimum fuzzy match score in [0.0, 1.0] (default: 0.35)",
    )
    p_search.add_argument("--refresh", action="store_true", help="Force refresh store library")

    # info
    p_info = sub.add_parser(
        "info",
        help="Inspect size, EOS/anti-cheat/DRM signals (report-only), DLCs, and redistributables",
    )
    _add_shared_flags(p_info)
    p_info.add_argument("app", help="Store app_id (e.g. Epic app_name)")
    p_info.add_argument(
        "--platform",
        default="windows",
        choices=("windows", "macos"),
        help="Target platform (default: windows)",
    )
    p_info.add_argument(
        "--offline",
        action="store_true",
        help="Use cached local metadata only",
    )

    # list-dlcs / dlcs
    p_dlcs = sub.add_parser(
        "list-dlcs",
        aliases=["dlcs"],
        help="List owned/available DLCs and add-ons for an app",
    )
    _add_shared_flags(p_dlcs)
    p_dlcs.add_argument("app", help="Store app_id")
    p_dlcs.add_argument(
        "--platform",
        default="windows",
        choices=("windows", "macos"),
        help="Target platform (default: windows)",
    )
    p_dlcs.add_argument("--offline", action="store_true", help="Use cached local metadata only")

    # list-redists / redists
    p_redists = sub.add_parser(
        "list-redists",
        aliases=["redists"],
        help="List redistributables (VC++, DirectX, EOS SDK, etc.) needed by an app",
    )
    _add_shared_flags(p_redists)
    p_redists.add_argument("app", help="Store app_id")
    p_redists.add_argument(
        "--platform",
        default="windows",
        choices=("windows", "macos"),
        help="Target platform (default: windows)",
    )
    p_redists.add_argument("--offline", action="store_true", help="Use cached local metadata only")

    # download
    p_dl = sub.add_parser(
        "download",
        help="Download an owned build (and optional DLCs/EOS overlay) and write package manifest",
    )
    _add_shared_flags(p_dl)
    p_dl.add_argument("app", help="Store app_id to download")
    p_dl.add_argument(
        "--dest",
        default=None,
        help="Destination directory (default: $PORT_ACQUIRE_DEST/<title> or ~/Games/PortFactory/<store>/<title>)",
    )
    p_dl.add_argument(
        "--platform",
        default="windows",
        choices=("windows", "macos"),
        help="Target platform build (default: windows)",
    )
    p_dl.add_argument(
        "--with-dlcs",
        action="store_true",
        help="Also download all owned DLCs/add-ons for this title",
    )
    p_dl.add_argument(
        "--dlc",
        action="append",
        default=[],
        help="Download specific owned DLC app_id (repeatable)",
    )
    p_dl.add_argument(
        "--with-eos-overlay",
        action="store_true",
        help="Also download official EOS Overlay redistributable bits into the destination",
    )
    p_dl.add_argument(
        "--no-register",
        action="store_true",
        help="Download files without registering the install in the backend database",
    )
    p_dl.add_argument(
        "--max-workers",
        type=int,
        default=None,
        help="Override maximum concurrent download workers",
    )
    p_dl.add_argument(
        "--manifest-out",
        default=None,
        help=f"Custom path for {MANIFEST_FILENAME} (default: <dest>/{MANIFEST_FILENAME})",
    )
    p_dl.add_argument(
        "--no-manifest",
        action="store_true",
        help=f"Skip writing {MANIFEST_FILENAME}",
    )

    # verify
    p_verify = sub.add_parser("verify", help="Verify installed game files against store manifest")
    _add_shared_flags(p_verify)
    p_verify.add_argument("app", help="Installed app_id")
    p_verify.add_argument(
        "--repair",
        action="store_true",
        help="Re-download any missing or corrupted files",
    )

    # update
    p_update = sub.add_parser("update", help="Check for or apply updates to an installed game")
    _add_shared_flags(p_update)
    p_update.add_argument("app", help="Installed app_id")
    p_update.add_argument(
        "--check-only",
        action="store_true",
        help="Only check if an update is available without downloading",
    )
    p_update.add_argument(
        "--with-dlcs",
        action="store_true",
        help="Also update installed DLCs",
    )

    # uninstall
    p_uninst = sub.add_parser("uninstall", help="Uninstall an installed game")
    _add_shared_flags(p_uninst)
    p_uninst.add_argument("app", help="Installed app_id")
    p_uninst.add_argument(
        "--keep-files",
        action="store_true",
        help="Remove from backend registry while keeping files on disk",
    )
    p_uninst.add_argument(
        "--skip-uninstaller",
        action="store_true",
        help="Skip any store uninstaller hooks",
    )

    # launch-args
    p_launch = sub.add_parser(
        "launch-args",
        help="Export sanitized launch arguments and/or package manifest (never logs auth tokens)",
    )
    _add_shared_flags(p_launch)
    p_launch.add_argument("app", help="Installed app_id")
    p_launch.add_argument(
        "--online",
        action="store_true",
        help="Query online launch arguments (auth tokens are still redacted from output)",
    )
    p_launch.add_argument(
        "--export-manifest",
        default=None,
        help=f"Write schema-v1 package manifest JSON (e.g. <game-dir>/{MANIFEST_FILENAME})",
    )
    p_launch.add_argument(
        "--export-game-toml",
        default=None,
        help="Write a starter game.toml file populated from the manifest",
    )

    return parser


def _write_manifest_for_install(
    backend: StoreBackend,
    *,
    app_id: str,
    install_dir: Path,
    platform: str,
    manifest_out: Path,
    eos_overlay_path: str | None = None,
    offline: bool = True,
) -> dict[str, Any]:
    info: GameInfo | None = None
    try:
        info = backend.info(app_id, platform=platform, offline=offline)
    except AcquireError:
        info = None
    if (info is None or not info.executable) and offline:
        try:
            online_info = backend.info(app_id, platform=platform, offline=False)
            if info is None or online_info.executable:
                info = online_info
        except AcquireError:
            pass
    if info is None:
        info = GameInfo(store=backend.name, app_id=app_id, title=app_id, platform=platform)

    launch_plan = None
    try:
        launch_plan = backend.launch_args(app_id, offline=True)
    except AcquireError:
        launch_plan = None

    disk_redists = scan_dir_redistributables(install_dir)
    manifest = build_package_manifest(
        info=info,
        install_dir=install_dir,
        launch_plan=launch_plan,
        eos_overlay_path=eos_overlay_path,
        extra_redists=disk_redists,
    )
    write_package_manifest(manifest, manifest_out)
    return manifest


def _dispatch(args: argparse.Namespace) -> int:
    as_json = bool(getattr(args, "json", False))
    cmd = args.command

    if cmd == "stores":
        stores = available_backends()
        _emit(
            {"ok": True, "stores": stores},
            as_json=as_json,
            human_lines=[f"Available store backends: {', '.join(stores) or '(none)'}"],
        )
        return 0

    store_name = (args.store or _default_store()).strip().lower()
    backend = get_backend(store_name)

    if cmd == "login":
        status = backend.login(
            no_webview=bool(args.no_webview),
            auth_code=args.code,
            import_existing=bool(args.import_existing),
            logout=bool(args.logout),
        )
        lines = [
            f"Store: {status.store}",
            f"Logged in: {status.logged_in}",
            f"Account: {status.display_name or '<not logged in>'}",
        ]
        _emit({"ok": True, **status.to_dict()}, as_json=as_json, human_lines=lines)
        return 0

    if cmd == "status":
        status = backend.status(offline=bool(args.offline))
        lines = [
            f"Store: {status.store}",
            f"Backend available: {status.backend_available}"
            + (f" ({status.backend_version})" if status.backend_version else ""),
            f"Logged in: {status.logged_in}",
            f"Account: {status.display_name or '<not logged in>'}",
        ]
        _emit({"ok": True, **status.to_dict()}, as_json=as_json, human_lines=lines)
        return 0

    if cmd == "list-owned":
        games = backend.list_owned(platform=args.platform, refresh=bool(args.refresh))
        lines = [f"Owned {store_name} games ({len(games)}, platform={args.platform}):"]
        for g in games:
            inst = " [installed]" if g.installed else ""
            dlc_note = f" (+{len(g.dlcs)} DLC)" if g.dlcs else ""
            lines.append(f"  {g.app_id:24} {g.title}{dlc_note}{inst}")
        _emit(
            {
                "ok": True,
                "store": store_name,
                "platform": args.platform,
                "count": len(games),
                "games": [g.to_dict() for g in games],
            },
            as_json=as_json,
            human_lines=lines,
        )
        return 0

    if cmd == "search":
        matches = backend.search(
            args.query,
            platform=args.platform,
            limit=args.limit,
            min_score=args.min_score,
            refresh=bool(args.refresh),
        )
        lines = [f"Search matches for {args.query!r} in {store_name} ({len(matches)}):"]
        for m in matches:
            inst = " [installed]" if m.installed else ""
            lines.append(f"  {m.score:0.3f}  {m.app_id:24} {m.title}{inst}")
        _emit(
            {
                "ok": True,
                "store": store_name,
                "query": args.query,
                "platform": args.platform,
                "count": len(matches),
                "matches": [m.to_dict() for m in matches],
            },
            as_json=as_json,
            human_lines=lines,
        )
        return 0

    if cmd == "info":
        info = backend.info(args.app, platform=args.platform, offline=bool(args.offline))
        lines = [
            f"Title:      {info.title} ({info.app_id})",
            f"Store:      {info.store} ({info.platform})",
            f"Version:    {info.version or 'unknown'}",
            f"Download:   {_format_bytes(info.download_size_bytes)}",
            f"Disk size:  {_format_bytes(info.disk_size_bytes)}",
            f"Installed:  {info.installed}"
            + (f" ({info.install_path})" if info.install_path else ""),
            f"Executable: {info.executable or 'unknown'}",
            f"Offline OK: {info.can_run_offline}",
            f"EOS needed: {info.requires_eos}"
            + (f" ({', '.join(info.eos_signals)})" if info.eos_signals else ""),
            f"Anti-cheat: {'DETECTED: ' + ', '.join(info.anticheat.systems) if info.anticheat.detected else 'none detected'} (report-only)",
            f"DRM/Auth:   {'DETECTED' if info.drm.detected else 'none detected'}"
            + (f" (signals: {', '.join(info.drm.signals)})" if info.drm.signals else "")
            + " (report-only)",
            f"DLCs:       {len(info.dlcs)}",
            f"Redists:    {len(info.redistributables)}",
        ]
        _emit({"ok": True, **info.to_dict()}, as_json=as_json, human_lines=lines)
        return 0

    if cmd in ("list-dlcs", "dlcs"):
        dlcs = backend.list_dlcs(args.app, platform=args.platform, offline=bool(args.offline))
        lines = [f"DLCs for {args.app} ({len(dlcs)}):"]
        for d in dlcs:
            inst = " [installed]" if d.installed else ("" if d.installable else " [entitlement-only]")
            label = d.app_id or d.catalog_id or "<entitlement>"
            lines.append(
                f"  {label:24} {d.title} ({_format_bytes(d.disk_size_bytes)}){inst}"
            )
        _emit(
            {
                "ok": True,
                "store": store_name,
                "app_id": args.app,
                "count": len(dlcs),
                "dlcs": [d.to_dict() for d in dlcs],
            },
            as_json=as_json,
            human_lines=lines,
        )
        return 0

    if cmd in ("list-redists", "redists"):
        redists = backend.list_redists(args.app, platform=args.platform, offline=bool(args.offline))
        lines = [f"Redistributables for {args.app} ({len(redists)}):"]
        for r in redists:
            native = " [native in win32-rt]" if r.native_in_win32_rt else ""
            lines.append(f"  [{r.category:8}] {r.name} ({r.relative_path or r.source}){native}")
        _emit(
            {
                "ok": True,
                "store": store_name,
                "app_id": args.app,
                "count": len(redists),
                "redistributables": [r.to_dict() for r in redists],
            },
            as_json=as_json,
            human_lines=lines,
        )
        return 0

    if cmd == "download":
        title_hint: str | None = None
        if args.dest is None:
            try:
                title_hint = backend.info(args.app, platform=args.platform, offline=True).title
            except AcquireError:
                title_hint = None
            dest = _default_dest(store_name, args.app, title=title_hint)
        else:
            dest = Path(args.dest).expanduser().resolve()

        opts = DownloadOptions(
            app_id=args.app,
            dest=dest,
            platform=args.platform,
            with_dlcs=bool(args.with_dlcs),
            skip_dlcs=not bool(args.with_dlcs) and not bool(args.dlc),
            dlcs=tuple(args.dlc or ()),
            with_eos_overlay=bool(args.with_eos_overlay),
            no_register=bool(args.no_register),
            max_workers=args.max_workers,
        )
        result = backend.download(opts)

        if not args.no_manifest:
            install_path = Path(result.install_path)
            manifest_path = (
                Path(args.manifest_out).expanduser().resolve()
                if args.manifest_out
                else install_path / MANIFEST_FILENAME
            )
            _write_manifest_for_install(
                backend,
                app_id=result.app_id,
                install_dir=install_path,
                platform=result.platform,
                manifest_out=manifest_path,
                eos_overlay_path=result.eos_overlay_path,
                offline=not bool(args.no_register),
            )
            result.manifest_path = str(manifest_path)

        lines = [
            f"Downloaded {result.title} ({result.app_id}) [{result.platform}]",
            f"  Install path: {result.install_path}",
        ]
        if result.downloaded_dlcs:
            lines.append(f"  DLCs:         {', '.join(result.downloaded_dlcs)}")
        if result.eos_overlay_path:
            lines.append(f"  EOS overlay:  {result.eos_overlay_path}")
        if result.manifest_path:
            lines.append(f"  Manifest:     {result.manifest_path}")
        _emit({"ok": True, **result.to_dict()}, as_json=as_json, human_lines=lines)
        return 0

    if cmd == "verify":
        res = backend.verify(args.app, repair=bool(args.repair))
        lines = [
            f"Verify {res.app_id}: {'OK' if res.ok else 'FAILED'}"
            + (" (repaired)" if res.repaired else ""),
        ]
        if res.message:
            lines.append(f"  {res.message}")
        _emit({"ok": res.ok, **res.to_dict()}, as_json=as_json, human_lines=lines)
        return 0 if res.ok else 1

    if cmd == "update":
        res = backend.update(
            args.app,
            check_only=bool(args.check_only),
            with_dlcs=bool(args.with_dlcs),
        )
        if res.updated and res.install_path:
            existing_manifest = Path(res.install_path) / MANIFEST_FILENAME
            if existing_manifest.is_file():
                _write_manifest_for_install(
                    backend,
                    app_id=res.app_id,
                    install_dir=Path(res.install_path),
                    platform="windows",
                    manifest_out=existing_manifest,
                    offline=True,
                )
        lines = [
            f"Update {res.app_id}: updated={res.updated}, available={res.update_available}",
            f"  Version: {res.old_version or 'unknown'} -> {res.new_version or 'unknown'}",
        ]
        if res.message:
            lines.append(f"  {res.message}")
        _emit({"ok": True, **res.to_dict()}, as_json=as_json, human_lines=lines)
        return 0

    if cmd == "uninstall":
        res = backend.uninstall(
            args.app,
            keep_files=bool(args.keep_files),
            skip_uninstaller=bool(args.skip_uninstaller),
        )
        lines = [
            f"Uninstalled {res.app_id} (kept_files={res.kept_files})"
            + (f" from {res.install_path}" if res.install_path else ""),
        ]
        _emit({"ok": True, **res.to_dict()}, as_json=as_json, human_lines=lines)
        return 0

    if cmd == "launch-args":
        plan = backend.launch_args(args.app, offline=not bool(args.online))
        exported_manifest_path: str | None = None
        exported_toml_path: str | None = None
        manifest_obj: dict[str, Any] | None = None

        if args.export_manifest or args.export_game_toml:
            try:
                info = backend.info(args.app, platform="windows", offline=True)
            except AcquireError:
                info = GameInfo(
                    store=store_name,
                    app_id=plan.app_id,
                    title=plan.title,
                    platform="windows",
                    installed=True,
                    install_path=plan.install_dir,
                    executable=plan.executable,
                )
            disk_redists = scan_dir_redistributables(Path(plan.install_dir))
            manifest_obj = build_package_manifest(
                info=info,
                install_dir=plan.install_dir,
                launch_plan=plan,
                extra_redists=disk_redists,
            )
            if args.export_manifest:
                out_p = Path(args.export_manifest).expanduser().resolve()
                write_package_manifest(manifest_obj, out_p)
                exported_manifest_path = str(out_p)
            if args.export_game_toml:
                toml_p = Path(args.export_game_toml).expanduser().resolve()
                toml_p.parent.mkdir(parents=True, exist_ok=True)
                toml_p.write_text(render_starter_game_toml(manifest_obj), encoding="utf-8")
                exported_toml_path = str(toml_p)

        payload: dict[str, Any] = {"ok": True, **plan.to_dict()}
        if exported_manifest_path:
            payload["exported_manifest"] = exported_manifest_path
        if exported_toml_path:
            payload["exported_game_toml"] = exported_toml_path
        lines = [
            f"Launch plan for {plan.title} ({plan.app_id}):",
            f"  Install dir:  {plan.install_dir}",
            f"  Executable:   {plan.executable}",
            f"  Working dir:  {plan.working_dir}",
            f"  Args:         {' '.join(plan.args) or '(none)'}",
            f"  Offline mode: {plan.offline}",
        ]
        if exported_manifest_path:
            lines.append(f"  Manifest:     {exported_manifest_path}")
        if exported_toml_path:
            lines.append(f"  game.toml:    {exported_toml_path}")
        _emit(payload, as_json=as_json, human_lines=lines)
        return 0

    raise AcquireError(f"unknown command: {cmd}")


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    as_json = bool(getattr(args, "json", False))
    try:
        return _dispatch(args)
    except AcquireError as exc:
        if as_json:
            print(json.dumps(exc.to_dict(), indent=2))
        else:
            print(f"error [{exc.code}]: {exc}", file=sys.stderr)
        return exc.exit_code
    except KeyboardInterrupt:
        if as_json:
            print(
                json.dumps(
                    {"ok": False, "error": {"code": "interrupted", "message": "Interrupted"}},
                    indent=2,
                )
            )
        else:
            print("Interrupted.", file=sys.stderr)
        return 130
