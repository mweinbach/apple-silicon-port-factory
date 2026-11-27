"""Unit tests for the Epic Games Store backend (`tools/port_acquire/backends/epic.py`).

Uses a self-contained mocked `legendary` executable in a temporary directory.
Requires zero network access, zero real credentials, and zero game files.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import stat
import subprocess
import sys
import tempfile
import unittest

_REPO_ROOT = Path(__file__).resolve().parents[3]
_TOOLS_DIR = _REPO_ROOT / "tools"
_CLI_PATH = _TOOLS_DIR / "port-acquire"
if str(_TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOLS_DIR))

from port_acquire.backend import (  # noqa: E402
    AppNotFoundError,
    AuthRequiredError,
    BackendUnavailableError,
    CommandFailedError,
    DownloadOptions,
)
from port_acquire.backends.epic import EpicBackend  # noqa: E402
from port_acquire.manifest import MANIFEST_FILENAME, read_package_manifest  # noqa: E402


_MOCK_LEGENDARY_SCRIPT = r'''#!/usr/bin/env python3
"""Stateful mock of Heroic's `legendary` CLI for offline unit tests."""
import json
import os
from pathlib import Path
import sys

cfg_dir = Path(os.environ.get("LEGENDARY_CONFIG_PATH", "."))
cfg_dir.mkdir(parents=True, exist_ok=True)
state_file = cfg_dir / "mock_state.json"
inv_file = cfg_dir / "invocations.jsonl"

DEFAULT_STATE = {
    "logged_in": True,
    "account": "ExamplePlayer",
    "verify_fail": False,
    "verify_silent_zero": False,
    "install_fail_zero": False,
    "install_silent_zero": False,
    "uninstall_fail_zero": False,
    "uninstall_lock_zero": False,
    "omit_remote_version": False,
    "warn_on_stdout": False,
    "installed": {},
    "updates": {
        "ArachnidTwo": "1.2.0",
    },
    "catalog": [
        {
            "app_name": "ArachnidTwo",
            "app_title": "Marvel's Spider-Man 2",
            "asset_infos": {
                "Windows": {"build_version": "1.1.0", "app_name": "ArachnidTwo"},
            },
            "metadata": {
                "id": "cat_arachnid_2",
                "customAttributes": {
                    "CanRunOffline": {"type": "STRING", "value": "true"},
                    "PresenceId": {"type": "STRING", "value": "pres_arachnid_2"},
                },
            },
            # Real Legendary `list --json` nests add-ons under `dlcs` (not `dlc_list` or top-level rows).
            "dlcs": [
                {
                    "app_name": "ArachnidTwoPreorderDLC",
                    "app_title": "Suit Pack DLC",
                    "asset_infos": {
                        "Windows": {"build_version": "1.1.0", "app_name": "ArachnidTwoPreorderDLC"},
                    },
                    "download_size": 524288000,
                    "disk_size": 1073741824,
                },
                {
                    "app_name": "ArachnidTwoExpansion",
                    "app_title": "Story Expansion Add-On",
                    "asset_infos": {
                        "Windows": {"build_version": "1.1.0", "app_name": "ArachnidTwoExpansion"},
                    },
                    "download_size": 2147483648,
                    "disk_size": 4294967296,
                },
            ],
        },
        {
            "app_name": "ArachnidRemastered",
            "app_title": "Marvel's Spider-Man Remastered",
            "asset_infos": {
                "Windows": {"build_version": "2.0.1", "app_name": "ArachnidRemastered"},
            },
            "metadata": {
                "id": "cat_arachnid_1",
                "customAttributes": {
                    "CanRunOffline": {"type": "STRING", "value": "true"},
                },
            },
            "dlcs": [],
        },
        {
            "app_name": "OnlineShooter",
            "app_title": "Competitive Arena Shooter",
            "asset_infos": {
                "Windows": {"build_version": "4.5.0", "app_name": "OnlineShooter"},
            },
            "metadata": {
                "id": "cat_shooter",
                "customAttributes": {
                    "CanRunOffline": {"type": "STRING", "value": "false"},
                    "UseAccessControl": {"type": "STRING", "value": "true"},
                    "MonitorPresence": {"type": "STRING", "value": "true"},
                    "ThirdPartyManagedApp": {"type": "STRING", "value": "Ubisoft Connect"},
                },
            },
            "dlcs": [],
        },
        {
            "app_name": "ExternalLauncherGame",
            "app_title": "External Launcher RPG",
            "external_activation": "Ubisoft Connect",
            "asset_infos": {
                "Windows": {"build_version": "1.0.0", "app_name": "ExternalLauncherGame"},
            },
            "dlcs": [],
        },
    ],
    "manifest_files": {
        "ArachnidTwo": [
            {"filename": "Spider-Man2.exe", "size": 100000000},
            {"filename": "Engine/Plugins/EOS/EOSSDK-Win64-Shipping.dll", "size": 12000000},
            {"filename": "Redist/VC_redist.x64.exe", "size": 24000000},
            {"filename": "Redist/DirectX/DXSETUP.exe", "size": 500000},
        ],
        "OnlineShooter": [
            {"filename": "start_protected_game.exe", "size": 2000000},
            {"filename": "EasyAntiCheat/EasyAntiCheat_EOS_Setup.exe", "size": 5000000},
            {"filename": "Game/EOSSDK-Win64-Shipping.dll", "size": 11000000},
        ],
    },
}


def load_state():
    if state_file.is_file():
        return json.loads(state_file.read_text(encoding="utf-8"))
    state_file.write_text(json.dumps(DEFAULT_STATE, indent=2), encoding="utf-8")
    return json.loads(state_file.read_text(encoding="utf-8"))


def save_state(state):
    state_file.write_text(json.dumps(state, indent=2), encoding="utf-8")


argv = sys.argv[1:]
with inv_file.open("a", encoding="utf-8") as fh:
    fh.write(json.dumps(argv) + "\n")

state = load_state()

# Strip global flags (-y, -v, -J)
args = []
i = 0
while i < len(argv):
    if argv[i] in ("-y", "--yes", "-v", "--debug", "-J", "--pretty-json"):
        i += 1
        continue
    if argv[i] in ("-A", "--api-timeout") and i + 1 < len(argv):
        i += 2
        continue
    args.append(argv[i])
    i += 1

if not args or args[0] in ("-V", "--version"):
    print('legendary version "0.20.42", codename "Mocked Test"')
    sys.exit(0)

cmd = args[0]
rest = args[1:]


def require_auth():
    if not state.get("logged_in"):
        print("[cli] ERROR: No saved credentials found, please authenticate via `legendary auth`.", file=sys.stderr)
        sys.exit(1)


def get_flag(flag, default=None):
    if flag in rest:
        idx = rest.index(flag)
        if idx + 1 < len(rest):
            return rest[idx + 1]
    return default


if cmd == "auth":
    if "--delete" in rest:
        state["logged_in"] = False
        state["account"] = None
        save_state(state)
        sys.exit(0)
    code = get_flag("--code")
    if code == "BAD_CODE_SECRET_999":
        print(f"ERROR: invalid authorization_code={code}", file=sys.stderr)
        sys.exit(1)
    state["logged_in"] = True
    state["account"] = "ExamplePlayer"
    save_state(state)
    sys.exit(0)

if cmd == "status":
    payload = {
        "account": state["account"] if state.get("logged_in") else "<not logged in>",
        "games_available": len(state["catalog"]) if state.get("logged_in") else 0,
        "games_installed": len(state["installed"]),
        "egl_sync_enabled": False,
        "config_directory": str(cfg_dir),
        # Deliberately include mock internal secret to assert port-acquire never forwards it
        "internal_mock_token": "MUST_NEVER_LEAK_TOKEN_ABC",
    }
    if state.get("warn_on_stdout"):
        print("[Core] WARNING: mock warning on stdout")
    print(json.dumps(payload))
    sys.exit(0)

if cmd == "list":
    require_auth()
    print(json.dumps(state["catalog"]))
    sys.exit(0)

if cmd == "list-installed":
    rows = []
    check_updates = "--check-updates" in rest
    for app_name, entry in state["installed"].items():
        row = dict(entry)
        if check_updates and not state.get("omit_remote_version") and app_name in state.get("updates", {}):
            row["remote_version"] = state["updates"][app_name]
            row["update_available"] = row["remote_version"] != row.get("version")
        rows.append(row)
    print(json.dumps(rows))
    sys.exit(0)

if cmd == "list-files":
    require_auth()
    app_name = rest[0]
    files = state["manifest_files"].get(app_name)
    if files is None:
        print(f"ERROR: Could not find app {app_name}", file=sys.stderr)
        sys.exit(1)
    print(json.dumps(files))
    sys.exit(0)

if cmd == "info":
    require_auth()
    app_name = rest[0]
    if "--offline" in rest and app_name not in state["installed"]:
        # Real Legendary `info --offline` cannot load manifest for unregistered apps.
        print(f'[cli] ERROR: Game "{app_name}" is not installed', file=sys.stderr)
        sys.exit(1)
    game = next((g for g in state["catalog"] if g["app_name"] == app_name), None)
    if game is None:
        print(f"ERROR: Could not find {app_name} in library", file=sys.stderr)
        sys.exit(1)
    exe = "Spider-Man2.exe" if app_name == "ArachnidTwo" else "start_protected_game.exe"
    prereq = (
        {
            "name": "Visual C++ 2022 Redistributable",
            "path": "Redist\\VC_redist.x64.exe",
            "args": "/install /quiet /norestart",
        }
        if app_name == "ArachnidTwo"
        else None
    )
    # Real Legendary `info --json`: `manifest.version` is the integer manifest format
    # revision (e.g. 18), while `manifest.build_version` is the game build version.
    # For entitlement-only DLCs, Legendary returns `app_name: None, installable: False, id: ...`.
    # For entitlement-backed downloadable DLCs, Legendary can return `app_name: None` with a
    # non-empty `installable` release-info array containing `appId`.
    # For external-launcher titles, Legendary exposes `game.external_activation` (without raw metadata).
    build_ver = state.get("updates", {}).get(
        app_name,
        game.get("asset_infos", {}).get("Windows", {}).get("build_version", "1.1.0"),
    )
    owned_dlc = list(game.get("dlcs", []))
    if app_name == "ArachnidTwo":
        owned_dlc.append(
            {
                "app_name": None,
                "installable": [
                    {"appId": "ArachnidTwoEntitlementDLC", "platform": ["Windows"]},
                ],
                "id": "cat_entitlement_installable",
                "title": "Entitlement Downloadable DLC",
            }
        )
        owned_dlc.append(
            {
                "app_name": None,
                "installable": False,
                "id": "cat_entitlement_only_bonus",
                "title": "Digital Bonus Entitlement",
            }
        )
    game_payload = {
        "app_name": game["app_name"],
        "title": game["app_title"],
        "external_activation": game.get("external_activation"),
        "asset_infos": game.get("asset_infos", {}),
        "owned_dlc": owned_dlc,
    }
    if "metadata" in game:
        game_payload["metadata"] = game["metadata"]
    payload = {
        "game": game_payload,
        "manifest": {
            "version": 18,
            "build_version": build_ver,
            "download_size": 48_000_000_000,
            "disk_size": 92_000_000_000,
            "launch_exe": exe,
            "launch_command": "-dx12",
            "prerequisites": prereq,
        },
        "install": state["installed"].get(app_name, {}),
    }
    print(json.dumps(payload))
    sys.exit(0)

if cmd in ("install", "download", "update"):
    require_auth()
    app_name = rest[0]
    if state.get("install_fail_zero"):
        # Real Legendary catches downloader exceptions, logs Installation failed, and exits 0.
        print("[cli] ERROR: Installation failed: simulated downloader exception", file=sys.stderr)
        sys.exit(0)
    if state.get("install_silent_zero"):
        sys.exit(0)
    if "--repair" in rest:
        if app_name not in state["installed"]:
            print(f'[cli] ERROR: Game "{app_name}" is not installed', file=sys.stderr)
            sys.exit(0)
        state["verify_fail"] = False
        save_state(state)
        print("[cli] INFO: Finished installation process in 0.10 seconds.")
        sys.exit(0)

    if "--update-only" in rest:
        if app_name not in state["installed"]:
            print(f'[cli] ERROR: Game "{app_name}" is not installed', file=sys.stderr)
            sys.exit(0)
        new_ver = state.get("updates", {}).get(app_name, state["installed"][app_name]["version"])
        state["installed"][app_name]["version"] = new_ver
        save_state(state)
        print("[cli] INFO: Finished installation process in 0.10 seconds.")
        sys.exit(0)

    game = next((g for g in state["catalog"] if g["app_name"] == app_name), None)
    # Also allow installing an inline DLC app_id
    dlc_title = None
    is_dlc = False
    if game is None:
        for g in state["catalog"]:
            for d in g.get("dlcs", []):
                if d["app_name"] == app_name:
                    dlc_title = d["app_title"]
                    is_dlc = True
                    break
        if not is_dlc and app_name == "ArachnidTwoEntitlementDLC":
            dlc_title = "Entitlement Downloadable DLC"
            is_dlc = True
    if game is None and not is_dlc:
        print(f"ERROR: Could not find {app_name}", file=sys.stderr)
        sys.exit(1)

    folder = get_flag("--game-folder") or get_flag("--base-path") or str(cfg_dir / app_name)
    install_dir = Path(folder)
    install_dir.mkdir(parents=True, exist_ok=True)
    if not is_dlc:
        (install_dir / "Spider-Man2.exe").write_bytes(b"MZ_MOCK")
        (install_dir / "Redist").mkdir(exist_ok=True)
        (install_dir / "Redist" / "VC_redist.x64.exe").write_bytes(b"MZ_MOCK")
    else:
        (install_dir / f"{app_name}.dat").write_bytes(b"DLC_MOCK")

    if "--download-only" not in rest:
        state["installed"][app_name] = {
            "app_name": app_name,
            "title": dlc_title or game["app_title"],
            "version": "1.1.0",
            "install_path": str(install_dir),
            "executable": "Spider-Man2.exe" if not is_dlc else "",
            "install_size": 92_000_000_000 if not is_dlc else 1_073_741_824,
            "is_dlc": is_dlc,
            "can_run_offline": True,
            "requires_ot": False,
        }
        if "--with-dlcs" in rest and game and game.get("dlcs"):
            for d in game["dlcs"]:
                d_id = d["app_name"]
                (install_dir / f"{d_id}.dat").write_bytes(b"DLC_MOCK")
                state["installed"][d_id] = {
                    "app_name": d_id,
                    "title": d["app_title"],
                    "version": "1.1.0",
                    "install_path": str(install_dir),
                    "executable": "",
                    "install_size": d.get("disk_size", 1024),
                    "is_dlc": True,
                }
        save_state(state)
    print("[cli] INFO: Finished installation process in 0.10 seconds.")
    sys.exit(0)

if cmd == "eos-overlay":
    action = rest[0] if rest else ""
    path = get_flag("--path")
    if action == "install" and path:
        ov = Path(path)
        ov.mkdir(parents=True, exist_ok=True)
        (ov / "EOSOVH-Win64-Shipping.dll").write_bytes(b"MZ_MOCK")
        # Exact upstream Legendary macOS message (no Wine prefix supplied).
        print("[cli] INFO: Overlay has been downloaded.")
    sys.exit(0)

if cmd == "verify":
    app_name = rest[0]
    if app_name not in state["installed"]:
        # Real Legendary logs this error and exits 0.
        print(f'[cli] ERROR: Game "{app_name}" is not installed', file=sys.stderr)
        sys.exit(0)
    if state.get("verify_silent_zero"):
        sys.exit(0)
    if state.get("verify_fail"):
        print('ERROR: File "Spider-Man2.exe" failed hash check (corrupted)!', file=sys.stderr)
        print('ERROR: Missing file "Data/level0.dat"', file=sys.stderr)
        sys.exit(1)
    print("Verification was successful.")
    sys.exit(0)

if cmd == "uninstall":
    app_name = rest[0]
    if state.get("uninstall_fail_zero"):
        print("[cli] ERROR: Removing game failed: PermissionError", file=sys.stderr)
        sys.exit(0)
    if state.get("uninstall_lock_zero"):
        print("[cli] ERROR: Could not acquire lockfile for uninstall", file=sys.stderr)
        sys.exit(0)
    entry = state["installed"].pop(app_name, None)
    if entry is None:
        # Real Legendary logs this error and exits 0.
        print(f"[cli] ERROR: Game {app_name} not installed, cannot uninstall!", file=sys.stderr)
        sys.exit(0)
    if "--keep-files" not in rest:
        p = Path(entry["install_path"])
        if p.is_dir():
            for child in p.iterdir():
                if child.is_file():
                    child.unlink()
    save_state(state)
    print("[cli] INFO: Game has been uninstalled.")
    sys.exit(0)

if cmd == "launch":
    app_name = rest[0]
    entry = state["installed"].get(app_name)
    if entry is None:
        print(f"ERROR: {app_name} is not installed", file=sys.stderr)
        sys.exit(1)
    payload = {
        "game_directory": entry["install_path"],
        "game_executable": entry["executable"],
        "working_directory": entry["install_path"],
        "game_parameters": ["-windowed", "-dx12"],
        "egl_parameters": [
            f"-epicapp={app_name}",
            "-epicenv=Prod",
            "-AUTH_LOGIN=private@example.com",
            "-AUTH_PASSWORD=TOP_SECRET_TOKEN_XYZ",
            "-AUTH_TYPE=exchangecode",
            "-epicuserid=deadbeefcafe1234",
        ],
        "user_parameters": [],
        "environment": {
            "EPIC_ACCESS_TOKEN": "SHOULD_BE_REDACTED_ENV_999",
            "GAME_LOCALE": "en-US",
        },
    }
    print(json.dumps(payload))
    sys.exit(0)

print(f"ERROR: unsupported mock command: {cmd}", file=sys.stderr)
sys.exit(2)
'''


class TestEpicBackendMocked(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        self.mock_bin = self.root / "legendary"
        self.mock_bin.write_text(_MOCK_LEGENDARY_SCRIPT, encoding="utf-8")
        self.mock_bin.chmod(self.mock_bin.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
        self.cfg_dir = self.root / "cfg"
        self.cfg_dir.mkdir()
        self.backend = EpicBackend(legendary_bin=self.mock_bin, config_dir=self.cfg_dir)

    def tearDown(self) -> None:
        self._tmp.cleanup()

    def _cli_env(self) -> dict[str, str]:
        env = dict(os.environ)
        env["PORT_LEGENDARY"] = str(self.mock_bin)
        env["PORT_LEGENDARY_CONFIG"] = str(self.cfg_dir)
        env["PORT_ACQUIRE_STORE"] = "epic"
        return env

    def _run_cli(self, *args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(_CLI_PATH), *args],
            env=self._cli_env(),
            stdin=subprocess.DEVNULL,
            capture_output=True,
            text=True,
            check=False,
        )

    def _read_invocations(self) -> list[list[str]]:
        inv_file = self.cfg_dir / "invocations.jsonl"
        if not inv_file.is_file():
            return []
        return [json.loads(line) for line in inv_file.read_text(encoding="utf-8").splitlines() if line]

    def _patch_state(self, **updates) -> None:
        # Ensure initial state exists
        self.backend.status(offline=True)
        sf = self.cfg_dir / "mock_state.json"
        data = json.loads(sf.read_text(encoding="utf-8"))
        data.update(updates)
        sf.write_text(json.dumps(data, indent=2), encoding="utf-8")

    def test_status_reports_only_login_and_display_name_no_tokens(self) -> None:
        proc = self._run_cli("status", "--json")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        data = json.loads(proc.stdout)
        self.assertTrue(data["ok"])
        self.assertEqual(data["store"], "epic")
        self.assertTrue(data["logged_in"])
        self.assertEqual(data["display_name"], "ExamplePlayer")
        self.assertTrue(data["backend_available"])
        self.assertNotIn("MUST_NEVER_LEAK_TOKEN_ABC", proc.stdout)
        self.assertNotIn("config_directory", data)

        # Also test leading warning line on stdout is tolerated
        self._patch_state(warn_on_stdout=True)
        st = self.backend.status(offline=True)
        self.assertTrue(st.logged_in)
        self.assertEqual(st.display_name, "ExamplePlayer")

    def test_login_logout_and_secret_redaction_on_error(self) -> None:
        out = self.backend.login(logout=True)
        self.assertFalse(out.logged_in)
        self.assertIsNone(out.display_name)

        # Unauthenticated list-owned must fail with AuthRequiredError (exit code 3)
        proc = self._run_cli("list-owned", "--json")
        self.assertEqual(proc.returncode, 3)
        err = json.loads(proc.stdout)
        self.assertFalse(err["ok"])
        self.assertEqual(err["error"]["code"], "auth_required")

        # Non-interactive login without --code must refuse with actionable command
        proc_login = self._run_cli("login", "--json")
        self.assertEqual(proc_login.returncode, 3)
        err_login = json.loads(proc_login.stdout)
        self.assertIn("tools/port-acquire login --store epic", err_login["error"]["message"])

        # Bad auth code must NEVER leak the code in stderr/stdout
        proc_bad = self._run_cli("login", "--code", "BAD_CODE_SECRET_999", "--json")
        self.assertNotEqual(proc_bad.returncode, 0)
        self.assertNotIn("BAD_CODE_SECRET_999", proc_bad.stdout)
        self.assertNotIn("BAD_CODE_SECRET_999", proc_bad.stderr)

        # Valid auth code succeeds and returns status without tokens
        ok_status = self.backend.login(auth_code="VALID_CODE_123")
        self.assertTrue(ok_status.logged_in)
        self.assertEqual(ok_status.display_name, "ExamplePlayer")

    def test_list_owned_and_fuzzy_search(self) -> None:
        proc = self._run_cli("list-owned", "--platform", "windows", "--json")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        data = json.loads(proc.stdout)
        self.assertEqual(data["count"], 4)  # 4 base games; nested DLCs attach to ArachnidTwo
        sm2 = next(g for g in data["games"] if g["app_id"] == "ArachnidTwo")
        dlc_ids = {d["app_id"] for d in sm2["dlcs"]}
        self.assertEqual(dlc_ids, {"ArachnidTwoPreorderDLC", "ArachnidTwoExpansion"})

        proc_search = self._run_cli("search", "Spider-Man 2", "--json")
        self.assertEqual(proc_search.returncode, 0, proc_search.stderr)
        sdata = json.loads(proc_search.stdout)
        self.assertEqual(sdata["matches"][0]["app_id"], "ArachnidTwo")
        self.assertEqual(sdata["matches"][0]["title"], "Marvel's Spider-Man 2")
        self.assertEqual(sdata["matches"][0]["dlcs_count"], 2)

    def test_info_reports_size_eos_anticheat_drm_dlcs_and_redists(self) -> None:
        # 1. Single-player title with EOS SDK + VC++ + DirectX + DLCs, zero anti-cheat
        proc = self._run_cli("info", "ArachnidTwo", "--platform", "windows", "--json")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        info = json.loads(proc.stdout)
        # Must use manifest.build_version ("1.2.0"), never manifest.version format revision (18).
        self.assertEqual(info["version"], "1.2.0")
        self.assertEqual(info["download_size_bytes"], 48_000_000_000)
        self.assertEqual(info["disk_size_bytes"], 92_000_000_000)
        self.assertTrue(info["can_run_offline"])
        self.assertTrue(info["requires_eos"])
        self.assertFalse(info["anticheat"]["detected"])
        self.assertTrue(info["anticheat"]["report_only"])
        self.assertFalse(info["drm"]["detected"])
        self.assertTrue(info["drm"]["report_only"])
        self.assertEqual(len(info["dlcs"]), 4)
        entitlement = next(d for d in info["dlcs"] if d["catalog_id"] == "cat_entitlement_only_bonus")
        self.assertFalse(entitlement["installable"])
        self.assertEqual(entitlement["app_id"], "")
        ent_dl = next(d for d in info["dlcs"] if d["catalog_id"] == "cat_entitlement_installable")
        self.assertTrue(ent_dl["installable"])
        self.assertEqual(ent_dl["app_id"], "ArachnidTwoEntitlementDLC")
        cats = {r["category"] for r in info["redistributables"]}
        self.assertEqual(cats, {"vcredist", "directx", "eos_sdk"})

        # Check dedicated list-dlcs & list-redists commands
        proc_dlcs = self._run_cli("list-dlcs", "ArachnidTwo", "--json")
        self.assertEqual(proc_dlcs.returncode, 0)
        self.assertEqual(json.loads(proc_dlcs.stdout)["count"], 4)

        # 2b. Uninstalled external-launcher title with `game.external_activation` only (no metadata/files)
        proc_ext = self._run_cli("info", "ExternalLauncherGame", "--platform", "windows", "--json")
        self.assertEqual(proc_ext.returncode, 0, proc_ext.stderr)
        ext_info = json.loads(proc_ext.stdout)
        self.assertTrue(ext_info["drm"]["detected"])
        self.assertEqual(ext_info["drm"]["third_party_launcher"], "Ubisoft Connect")
        self.assertIn("external_activation:Ubisoft Connect", ext_info["drm"]["signals"])

        proc_redists = self._run_cli("list-redists", "ArachnidTwo", "--json")
        self.assertEqual(proc_redists.returncode, 0)
        self.assertGreaterEqual(json.loads(proc_redists.stdout)["count"], 3)

        # 2. Online title with EAC + online-only DRM + third-party launcher (report-only)
        proc_ac = self._run_cli("info", "OnlineShooter", "--platform", "windows", "--json")
        self.assertEqual(proc_ac.returncode, 0, proc_ac.stderr)
        ac_info = json.loads(proc_ac.stdout)
        self.assertTrue(ac_info["anticheat"]["detected"])
        self.assertIn("Easy Anti-Cheat (EAC)", ac_info["anticheat"]["systems"])
        self.assertTrue(ac_info["anticheat"]["report_only"])
        self.assertTrue(ac_info["drm"]["detected"])
        self.assertTrue(ac_info["drm"]["requires_online_auth"])
        self.assertTrue(ac_info["drm"]["requires_ownership_token"])
        self.assertEqual(ac_info["drm"]["third_party_launcher"], "Ubisoft Connect")

        # 3. Unknown app raises AppNotFoundError (exit code 4)
        proc_missing = self._run_cli("info", "DoesNotExist", "--json")
        self.assertEqual(proc_missing.returncode, 4)
        self.assertEqual(json.loads(proc_missing.stdout)["error"]["code"], "app_not_found")

    def test_download_verify_update_launch_args_and_uninstall_lifecycle(self) -> None:
        dest = self.root / "games" / "SpiderMan2"

        # Download with DLCs + EOS overlay + package manifest
        proc_dl = self._run_cli(
            "download",
            "ArachnidTwo",
            "--dest",
            str(dest),
            "--platform",
            "windows",
            "--with-dlcs",
            "--with-eos-overlay",
            "--json",
        )
        self.assertEqual(proc_dl.returncode, 0, proc_dl.stderr)
        dl_data = json.loads(proc_dl.stdout)
        self.assertTrue(dl_data["ok"])
        self.assertEqual(dl_data["app_id"], "ArachnidTwo")
        self.assertIn("ArachnidTwoPreorderDLC", dl_data["downloaded_dlcs"])
        self.assertIsNotNone(dl_data["eos_overlay_path"])
        self.assertTrue(Path(dl_data["eos_overlay_path"]).is_dir())

        manifest_path = Path(dl_data["manifest_path"])
        self.assertEqual(manifest_path.name, MANIFEST_FILENAME)
        manifest = read_package_manifest(manifest_path)
        self.assertEqual(manifest["store"], "epic")
        self.assertEqual(manifest["executable"], "Spider-Man2.exe")
        self.assertTrue(manifest["redacted_auth_args"])
        raw_manifest = manifest_path.read_text(encoding="utf-8")
        self.assertNotIn("TOP_SECRET_TOKEN_XYZ", raw_manifest)
        self.assertNotIn("private@example.com", raw_manifest)
        self.assertNotIn("deadbeefcafe1234", raw_manifest)
        self.assertNotIn("SHOULD_BE_REDACTED_ENV_999", raw_manifest)
        self.assertIn("-windowed", manifest["launch_args"])
        self.assertIn("-epicapp=ArachnidTwo", manifest["launch_args"])
        self.assertTrue(any(r["id"] == "eos_overlay" for r in manifest["redistributables"]))

        # Verify command flags passed to legendary
        invocations = self._read_invocations()
        install_inv = next(inv for inv in invocations if "install" in inv and "ArachnidTwo" in inv)
        self.assertIn("--platform", install_inv)
        self.assertEqual(install_inv[install_inv.index("--platform") + 1], "Windows")
        self.assertIn("--game-folder", install_inv)
        self.assertIn("--with-dlcs", install_inv)

        # Verify OK
        proc_ver = self._run_cli("verify", "ArachnidTwo", "--json")
        self.assertEqual(proc_ver.returncode, 0, proc_ver.stderr)
        self.assertTrue(json.loads(proc_ver.stdout)["ok"])

        # Verify failure -> repair
        self._patch_state(verify_fail=True)
        proc_fail = self._run_cli("verify", "ArachnidTwo", "--json")
        self.assertEqual(proc_fail.returncode, 1)
        fail_data = json.loads(proc_fail.stdout)
        self.assertFalse(fail_data["ok"])
        self.assertEqual(len(fail_data["missing_files"]), 1)
        self.assertEqual(len(fail_data["corrupted_files"]), 1)

        proc_rep = self._run_cli("verify", "ArachnidTwo", "--repair", "--json")
        self.assertEqual(proc_rep.returncode, 0, proc_rep.stderr)
        rep_data = json.loads(proc_rep.stdout)
        self.assertTrue(rep_data["ok"])
        self.assertTrue(rep_data["repaired"])

        # Update check-only -> update apply
        proc_chk = self._run_cli("update", "ArachnidTwo", "--check-only", "--json")
        self.assertEqual(proc_chk.returncode, 0, proc_chk.stderr)
        chk_data = json.loads(proc_chk.stdout)
        self.assertTrue(chk_data["update_available"])
        self.assertFalse(chk_data["updated"])
        self.assertEqual(chk_data["old_version"], "1.1.0")
        self.assertEqual(chk_data["new_version"], "1.2.0")

        proc_upd = self._run_cli("update", "ArachnidTwo", "--json")
        self.assertEqual(proc_upd.returncode, 0, proc_upd.stderr)
        upd_data = json.loads(proc_upd.stdout)
        self.assertTrue(upd_data["updated"])
        self.assertEqual(upd_data["new_version"], "1.2.0")

        # Launch-args + explicit manifest & game.toml export
        exported_manifest = self.root / "exported-manifest.json"
        exported_toml = self.root / "game.toml"
        proc_launch = self._run_cli(
            "launch-args",
            "ArachnidTwo",
            "--export-manifest",
            str(exported_manifest),
            "--export-game-toml",
            str(exported_toml),
            "--json",
        )
        self.assertEqual(proc_launch.returncode, 0, proc_launch.stderr)
        launch_data = json.loads(proc_launch.stdout)
        self.assertTrue(launch_data["redacted_auth_args"])
        self.assertNotIn("TOP_SECRET_TOKEN_XYZ", proc_launch.stdout)
        self.assertNotIn("SHOULD_BE_REDACTED_ENV_999", proc_launch.stdout)
        self.assertEqual(launch_data["env"], {"GAME_LOCALE": "en-US"})
        self.assertTrue(exported_manifest.is_file())
        self.assertTrue(exported_toml.is_file())
        self.assertNotIn("TOP_SECRET_TOKEN_XYZ", exported_toml.read_text(encoding="utf-8"))

        # Uninstall with --keep-files
        proc_uninst = self._run_cli("uninstall", "ArachnidTwo", "--keep-files", "--json")
        self.assertEqual(proc_uninst.returncode, 0, proc_uninst.stderr)
        uninst_data = json.loads(proc_uninst.stdout)
        self.assertTrue(uninst_data["uninstalled"])
        self.assertTrue(uninst_data["kept_files"])
        self.assertTrue((dest / "Spider-Man2.exe").is_file())

    def test_selective_dlc_download(self) -> None:
        dest = self.root / "games" / "SelectiveDLC"
        res = self.backend.download(
            DownloadOptions(
                app_id="ArachnidTwo",
                dest=dest,
                platform="windows",
                with_dlcs=False,
                dlcs=("ArachnidTwoPreorderDLC",),
            )
        )
        self.assertEqual(res.downloaded_dlcs, ["ArachnidTwoPreorderDLC"])
        self.assertTrue((dest / "ArachnidTwoPreorderDLC.dat").is_file())

    def test_missing_legendary_binary_raises_clean_error(self) -> None:
        bad_backend = EpicBackend(legendary_bin=self.root / "no-such-legendary")
        st = bad_backend.status(offline=True)
        self.assertFalse(st.backend_available)
        self.assertFalse(st.logged_in)

        with self.assertRaises(BackendUnavailableError):
            bad_backend.list_owned()

        with self.assertRaises(AppNotFoundError):
            self.backend.uninstall("NotInstalledApp")

    def test_info_and_update_use_manifest_build_version_not_format_version(self) -> None:
        dest = self.root / "games" / "CurrentBuild"
        self._patch_state(updates={"ArachnidTwo": "1.1.0"}, omit_remote_version=True)
        self.backend.download(DownloadOptions(app_id="ArachnidTwo", dest=dest, platform="windows"))

        info = self.backend.info("ArachnidTwo", platform="windows")
        self.assertEqual(info.version, "1.1.0")

        # `list-installed --check-updates` omits `remote_version`, forcing fallback to `info --json`.
        # Because `manifest.version` is 18 (format rev) and `manifest.build_version` is "1.1.0",
        # an already-current install must NOT report update_available=True.
        upd = self.backend.update("ArachnidTwo", check_only=True)
        self.assertFalse(upd.update_available)
        self.assertFalse(upd.updated)
        self.assertEqual(upd.old_version, "1.1.0")
        self.assertEqual(upd.new_version, "1.1.0")

    def test_verify_uninstalled_or_silent_zero_does_not_report_ok(self) -> None:
        # Real Legendary exits 0 when verifying an uninstalled app, logging
        # `[cli] ERROR: Game "..." is not installed`.
        proc = self._run_cli("verify", "NeverInstalledApp", "--json")
        self.assertEqual(proc.returncode, 4)
        err = json.loads(proc.stdout)
        self.assertFalse(err["ok"])
        self.assertEqual(err["error"]["code"], "app_not_found")

        # Even if verify exits 0 on an installed app without printing completion confirmation,
        # port-acquire must not claim all files verified cleanly.
        dest = self.root / "games" / "SilentVerify"
        self.backend.download(DownloadOptions(app_id="ArachnidTwo", dest=dest, platform="windows"))
        self._patch_state(verify_silent_zero=True)
        res = self.backend.verify("ArachnidTwo")
        self.assertFalse(res.ok)
        self.assertIn("without reporting verification completion", res.message)

    def test_download_no_register_with_dlcs_downloads_each_dlc_explicitly(self) -> None:
        dest = self.root / "games" / "NoRegisterWithDlcs"
        proc = self._run_cli(
            "download",
            "ArachnidTwo",
            "--dest",
            str(dest),
            "--platform",
            "windows",
            "--no-register",
            "--with-dlcs",
            "--json",
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        data = json.loads(proc.stdout)
        self.assertTrue(data["ok"])
        self.assertEqual(
            set(data["downloaded_dlcs"]),
            {
                "ArachnidTwoPreorderDLC",
                "ArachnidTwoExpansion",
                "ArachnidTwoEntitlementDLC",
            },
        )
        # Base game + all installable DLC payloads (including entitlement-backed `installable` list)
        # landed in the same destination directory; non-installable entitlement was excluded.
        self.assertTrue((dest / "Spider-Man2.exe").is_file())
        self.assertTrue((dest / "ArachnidTwoPreorderDLC.dat").is_file())
        self.assertTrue((dest / "ArachnidTwoExpansion.dat").is_file())
        self.assertTrue((dest / "ArachnidTwoEntitlementDLC.dat").is_file())
        manifest = read_package_manifest(dest / MANIFEST_FILENAME)
        # Unregistered download must still populate executable and default launch args via online info.
        self.assertEqual(manifest["executable"], "Spider-Man2.exe")
        self.assertEqual(manifest["launch_args"], ["-dx12"])
        self.assertEqual(manifest["version"], "1.2.0")

    def test_download_fails_on_exit_zero_installation_failed_even_when_already_registered(self) -> None:
        # First install cleanly so `ArachnidTwo` is already registered in installed.json.
        dest1 = self.root / "games" / "InstalledFirst"
        self.backend.download(DownloadOptions(app_id="ArachnidTwo", dest=dest1, platform="windows"))

        # Now simulate Legendary catching a downloader error, logging `Installation failed`,
        # and exiting 0 while the old registration still exists.
        dest2 = self.root / "games" / "FailedRedownload"
        self._patch_state(install_fail_zero=True)
        proc = self._run_cli(
            "download",
            "ArachnidTwo",
            "--dest",
            str(dest2),
            "--platform",
            "windows",
            "--json",
        )
        self.assertEqual(proc.returncode, 5)
        err = json.loads(proc.stdout)
        self.assertFalse(err["ok"])
        self.assertEqual(err["error"]["code"], "command_failed")
        self.assertFalse((dest2 / MANIFEST_FILENAME).exists())

        # Also fail if Legendary exits 0 without confirming installation completion.
        self._patch_state(install_fail_zero=False, install_silent_zero=True)
        with self.assertRaises(CommandFailedError):
            self.backend.download(
                DownloadOptions(app_id="ArachnidTwo", dest=dest2, platform="windows")
            )

    def test_uninstall_fails_on_exit_zero_removal_or_lock_error(self) -> None:
        dest = self.root / "games" / "UninstallFail"
        self.backend.download(DownloadOptions(app_id="ArachnidTwo", dest=dest, platform="windows"))

        self._patch_state(uninstall_fail_zero=True)
        proc_fail = self._run_cli("uninstall", "ArachnidTwo", "--json")
        self.assertEqual(proc_fail.returncode, 5)
        self.assertEqual(json.loads(proc_fail.stdout)["error"]["code"], "command_failed")

        self._patch_state(uninstall_fail_zero=False, uninstall_lock_zero=True)
        with self.assertRaises(CommandFailedError):
            self.backend.uninstall("ArachnidTwo")


if __name__ == "__main__":
    unittest.main()
