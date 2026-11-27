"""Unit tests for the shared store-backend interface, fuzzy matching, manifest & redaction."""

from __future__ import annotations

import json
from pathlib import Path
import sys
import tempfile
import unittest

_TOOLS_DIR = Path(__file__).resolve().parents[2]
if str(_TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOLS_DIR))

from port_acquire.backend import (  # noqa: E402
    AnticheatReport,
    AuthStatus,
    BackendUnavailableError,
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
    fuzzy_search_owned,
    normalize_title,
    scan_dir_redistributables,
    score_title_match,
)
from port_acquire.cli import main as cli_main  # noqa: E402
from port_acquire.manifest import (  # noqa: E402
    MANIFEST_SCHEMA_VERSION,
    build_package_manifest,
    read_package_manifest,
    redact_env,
    redact_launch_args,
    render_starter_game_toml,
    write_package_manifest,
)
from port_acquire.registry import available_backends, get_backend, register_backend  # noqa: E402


class _DummySteamBackend(StoreBackend):
    """Minimal second backend proving pluggability without touching shared files."""

    name = "test-steam"
    display_name = "Test Steam"

    def login(self, **kwargs):
        return AuthStatus(store=self.name, logged_in=True, display_name="TestSteamUser")

    def status(self, *, offline: bool = False):
        return AuthStatus(store=self.name, logged_in=True, display_name="TestSteamUser")

    def list_owned(self, *, platform: str = "windows", refresh: bool = False):
        return [
            OwnedGame(store=self.name, app_id="2651280", title="Marvel's Spider-Man 2"),
            OwnedGame(store=self.name, app_id="1817070", title="Marvel's Spider-Man Remastered"),
        ]

    def info(self, app_id: str, *, platform: str = "windows", offline: bool = False):
        return GameInfo(store=self.name, app_id=app_id, title="Marvel's Spider-Man 2")

    def download(self, options: DownloadOptions):
        return DownloadResult(
            store=self.name,
            app_id=options.app_id,
            title="Marvel's Spider-Man 2",
            platform=options.platform,
            install_path=str(options.dest),
        )

    def verify(self, app_id: str, *, repair: bool = False):
        return VerifyResult(store=self.name, app_id=app_id, ok=True, repaired=repair)

    def update(self, app_id: str, *, check_only: bool = False, with_dlcs: bool = False):
        return UpdateResult(store=self.name, app_id=app_id, updated=False)

    def uninstall(self, app_id: str, *, keep_files: bool = False, skip_uninstaller: bool = False):
        return UninstallResult(store=self.name, app_id=app_id, uninstalled=True, kept_files=keep_files)

    def launch_args(self, app_id: str, *, offline: bool = True):
        return LaunchPlan(
            store=self.name,
            app_id=app_id,
            title="Marvel's Spider-Man 2",
            install_dir="/tmp/sm2",
            executable="Spider-Man2.exe",
            working_dir="/tmp/sm2",
        )


class TestInterfaceAndManifest(unittest.TestCase):
    def test_title_normalization_and_roman_numerals(self) -> None:
        self.assertEqual(normalize_title("Marvel's Spider-Man™ 2"), "marvels spider man 2")
        self.assertEqual(normalize_title("Alan Wake II"), "alan wake 2")
        self.assertEqual(normalize_title("Control: Ultimate Edition®"), "control ultimate edition")

    def test_fuzzy_search_ranks_exact_sequel_first(self) -> None:
        games = [
            OwnedGame(store="epic", app_id="RemasteredApp", title="Marvel's Spider-Man Remastered"),
            OwnedGame(store="epic", app_id="MilesApp", title="Marvel's Spider-Man: Miles Morales"),
            OwnedGame(store="epic", app_id="ArachnidTwo", title="Marvel's Spider-Man 2"),
            OwnedGame(store="epic", app_id="WakeTwo", title="Alan Wake II"),
        ]
        matches = fuzzy_search_owned(games, "Spider-Man 2")
        self.assertGreaterEqual(len(matches), 1)
        self.assertEqual(matches[0].app_id, "ArachnidTwo")
        self.assertGreater(matches[0].score, 0.85)
        if len(matches) > 1:
            self.assertGreater(matches[0].score, matches[1].score)

        aw2_matches = fuzzy_search_owned(games, "Alan Wake 2")
        self.assertEqual(aw2_matches[0].app_id, "WakeTwo")
        self.assertEqual(aw2_matches[0].score, 1.0)

        id_score = score_title_match("ArachnidTwo", "Marvel's Spider-Man 2", "ArachnidTwo")
        self.assertEqual(id_score, 1.0)

    def test_registry_discovers_epic_and_accepts_plugins(self) -> None:
        self.assertIn("epic", available_backends())
        register_backend("test-steam", _DummySteamBackend)
        backend = get_backend("test-steam")
        self.assertEqual(backend.name, "test-steam")
        matches = backend.search("Spider-Man 2")
        self.assertEqual(matches[0].app_id, "2651280")

        with self.assertRaises(BackendUnavailableError):
            get_backend("nonexistent-store-xyz")

    def test_redact_launch_args_and_env_strip_all_secrets(self) -> None:
        raw_args = [
            "-epicapp=ArachnidTwo",
            "-epicenv=Prod",
            "-AUTH_LOGIN=secret@example.com",
            "-AUTH_PASSWORD=super_secret_exchange_token",
            "-AUTH_TYPE=exchangecode",
            "-epicuserid=abcdef0123456789",
            "--token",
            "tok_value_should_vanish",
            "-windowed",
        ]
        safe, redacted = redact_launch_args(raw_args)
        self.assertTrue(redacted)
        self.assertEqual(safe, ["-epicapp=ArachnidTwo", "-epicenv=Prod", "-windowed"])

        safe_env, env_redacted = redact_env(
            {
                "PORT_METALFX": "1",
                "EPIC_AUTH_TOKEN": "should_not_appear",
                "USER_PASSWORD": "hidden",
                "SDL_VIDEODRIVER": "windows",
            }
        )
        self.assertTrue(env_redacted)
        self.assertEqual(safe_env, {"PORT_METALFX": "1", "SDL_VIDEODRIVER": "windows"})

    def test_package_manifest_roundtrip_and_starter_toml(self) -> None:
        info = GameInfo(
            store="epic",
            app_id="ArachnidTwo",
            title="Example Action Game 2",
            platform="windows",
            version="1.2.3",
            download_size_bytes=50_000_000_000,
            disk_size_bytes=95_000_000_000,
            executable="Binaries/Win64/ExampleGame.exe",
            can_run_offline=True,
            requires_eos=True,
            eos_signals=["manifest_file:EOSSDK-Win64-Shipping.dll"],
            anticheat=AnticheatReport(detected=False, systems=[], signals=[], report_only=True),
            drm=DrmReport(detected=False, report_only=True),
            dlcs=[DlcItem(app_id="ArachnidTwoDLC1", title="Bonus Pack", owned=True, installed=True)],
            redistributables=[
                RedistributableItem(
                    id="vcredist:vc_redist.x64.exe",
                    name="Visual C++ Redistributable",
                    category="vcredist",
                    source="prereq_info",
                    relative_path="Redist/VC_redist.x64.exe",
                    native_in_win32_rt=True,
                )
            ],
        )
        plan = LaunchPlan(
            store="epic",
            app_id="ArachnidTwo",
            title="Example Action Game 2",
            install_dir="/tmp/ExampleGame",
            executable="Binaries/Win64/ExampleGame.exe",
            working_dir="/tmp/ExampleGame/Binaries/Win64",
            args=["-epicapp=ArachnidTwo", "-AUTH_PASSWORD=leaked_secret", "-windowed"],
            env={"EPIC_TOKEN": "leaked_env_token", "SAFE_VAR": "ok"},
            offline=True,
        )
        manifest = build_package_manifest(info=info, install_dir="/tmp/ExampleGame", launch_plan=plan)
        serialized = json.dumps(manifest)
        self.assertNotIn("leaked_secret", serialized)
        self.assertNotIn("leaked_env_token", serialized)
        self.assertTrue(manifest["redacted_auth_args"])
        self.assertEqual(manifest["launch_args"], ["-epicapp=ArachnidTwo", "-windowed"])
        self.assertEqual(manifest["schema_version"], MANIFEST_SCHEMA_VERSION)

        with tempfile.TemporaryDirectory() as tmp:
            mpath = Path(tmp) / "port-acquire-manifest.json"
            write_package_manifest(manifest, mpath)
            loaded = read_package_manifest(mpath)
            self.assertEqual(loaded["app_id"], "ArachnidTwo")
            self.assertEqual(len(loaded["dlcs"]), 1)
            self.assertEqual(loaded["packaging_hook"]["format"], "port-acquire-manifest-v1")

        toml = render_starter_game_toml(manifest)
        self.assertIn('id = "example-action-game-2"', toml)
        self.assertIn('exe = "Binaries/Win64/ExampleGame.exe"', toml)
        self.assertIn('args = "-epicapp=ArachnidTwo -windowed"', toml)
        self.assertNotIn("leaked_secret", toml)

    def test_render_starter_game_toml_normalizes_prequoted_args(self) -> None:
        manifest = {
            "app_id": "QuotedApp",
            "title": "Quoted App",
            "executable": "Game.exe",
            "launch_args": [
                '"Test Game"',
                "'Single Quoted'",
                '-DataPath="C:\\My Games\\Data"',
                "Plain With Space",
                "-DataDir=Data\\Content",
            ],
        }
        toml = render_starter_game_toml(manifest)
        # Must NOT double-quote `""Test Game""` (which would split in GameManifest::split_args).
        self.assertNotIn('\\"\\"Test Game\\"\\"', toml)
        self.assertIn(
            'args = "\\"Test Game\\" \\"Single Quoted\\" -DataPath=\\"C:\\\\My Games\\\\Data\\" '
            '\\"Plain With Space\\" -DataDir=Data\\\\Content"',
            toml,
        )

    def test_redistributable_classification_and_disk_scan(self) -> None:
        vc = classify_redistributable_path("_CommonRedist/vcredist/VC_redist.x64.exe")
        self.assertIsNotNone(vc)
        assert vc is not None
        self.assertEqual(vc.category, "vcredist")
        self.assertTrue(vc.native_in_win32_rt)

        dx = classify_redistributable_path("Redist/DirectX/DXSETUP.exe")
        self.assertIsNotNone(dx)
        assert dx is not None
        self.assertEqual(dx.category, "directx")
        self.assertTrue(dx.native_in_win32_rt)

        eos = classify_redistributable_path("Engine/Binaries/EOSSDK-Win64-Shipping.dll")
        self.assertIsNotNone(eos)
        assert eos is not None
        self.assertEqual(eos.category, "eos_sdk")
        self.assertFalse(eos.native_in_win32_rt)

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "Redist").mkdir()
            (root / "Redist" / "VC_redist.x64.exe").write_bytes(b"MZ")
            (root / "EOSSDK-Win64-Shipping.dll").write_bytes(b"MZ")
            found = scan_dir_redistributables(root)
            cats = {item.category for item in found}
            self.assertEqual(cats, {"vcredist", "eos_sdk"})

    def test_cli_global_and_subcommand_flags(self) -> None:
        import contextlib
        import io

        register_backend("test-steam", _DummySteamBackend)
        # Ensure `--store` before subcommand isn't clobbered by subparser defaults.
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            rc = cli_main(["--store", "test-steam", "--json", "status"])
        self.assertEqual(rc, 0)
        payload = json.loads(buf.getvalue())
        self.assertEqual(payload["store"], "test-steam")
        self.assertEqual(payload["display_name"], "TestSteamUser")


if __name__ == "__main__":
    unittest.main()
