"""Offline unit tests for the Steam backend (`tools/port_acquire/backends/steam.py`).

Uses a self-contained mocked `steamcmd` executable (and mocked `DepotDownloader`)
so CI passes with zero network access and zero real credentials.
"""

from __future__ import annotations

import contextlib
import io
import json
import os
from pathlib import Path
import stat
import sys
import tempfile
import textwrap
import unittest

_TOOLS_DIR = Path(__file__).resolve().parents[2]
if str(_TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOLS_DIR))

from port_acquire import (  # noqa: E402
    AuthRequiredError,
    DownloadOptions,
    UnsupportedOperationError,
    available_backends,
    get_backend,
    read_package_manifest,
)
from port_acquire.backends._steam_vdf import (  # noqa: E402
    extract_appinfo_vdf,
    parse_installscript_vdf,
    parse_licenses_print,
    parse_loginusers_vdf,
    parse_vdf,
)
from port_acquire.backends.steam import SteamBackend, _redact_cmd, _split_windows_args  # noqa: E402
from port_acquire.cli import main as cli_main  # noqa: E402


SAMPLE_APPINFO_VDF = textwrap.dedent(
    """\
    Redirecting stderr to '/tmp/stderr.txt'
    Logging directory: '/tmp/logs'
    Connecting anonymously to Steam Public...OK
    AppID : 1091500, change number : 24501999/0, last change : Thu Nov 26 12:00:00 2026
    "1091500"
    {
        "common"
        {
            "name"      "Sample Action RPG"
            "type"      "Game"
            "oslist"    "windows"
            "osarch"    "64"
            "steam_deck_compatibility"
            {
                "category"  "3"
                "configuration"
                {
                    "recommended_runtime"   "proton-9.0-3"
                }
            }
        }
        "extended"
        {
            "developer" "Sample Studio"
            "publisher" "Sample Publisher"
            "listofdlc" "2138330,2138331"
            "anti_cheat_support_url" "https://example.invalid/anticheat"
        }
        "config"
        {
            "installdir"    "Sample Action RPG"
            "steamcontrollertemplateindex"  "4"
            "launch"
            {
                "0"
                {
                    "executable"    "bin\\x64\\SampleGame.exe"
                    "arguments"     "-skipStartScreen"
                    "workingdir"    "bin\\x64"
                    "type"          "default"
                    "config"
                    {
                        "oslist"    "windows"
                        "osarch"    "64"
                    }
                }
            }
        }
        "ufs"
        {
            "quota" "104857600"
        }
        "depots"
        {
            "228989"
            {
                "config"
                {
                    "oslist"    "windows"
                }
                "depotfromapp"  "228980"
                "sharedinstall" "1"
            }
            "228990"
            {
                "config"
                {
                    "oslist"    "windows"
                }
                "depotfromapp"  "228980"
                "sharedinstall" "1"
            }
            "229007"
            {
                "config"
                {
                    "oslist"    "windows"
                }
                "depotfromapp"  "228980"
                "sharedinstall" "1"
            }
            "1091501"
            {
                "name"  "Sample Action RPG Win64 Content"
                "config"
                {
                    "oslist"    "windows"
                    "osarch"    "64"
                }
                "manifests"
                {
                    "public"
                    {
                        "gid"       "8811223344556677889"
                        "size"      "53687091200"
                        "download"  "42949672960"
                    }
                }
            }
            "1091502"
            {
                "name"  "Sample Action RPG Linux Content"
                "config"
                {
                    "oslist"    "linux"
                }
                "manifests"
                {
                    "public"
                    {
                        "gid"       "1111222233334444555"
                        "size"      "1073741824"
                        "download"  "536870912"
                    }
                }
            }
            "2138332"
            {
                "name"      "Expansion Pack DLC"
                "dlcappid"  "2138330"
                "config"
                {
                    "oslist"    "windows"
                    "osarch"    "64"
                }
                "manifests"
                {
                    "public"
                    {
                        "gid"       "9988776655443322110"
                        "size"      "10737418240"
                        "download"  "8589934592"
                    }
                }
            }
            "branches"
            {
                "public"
                {
                    "buildid"       "15000100"
                    "timeupdated"   "1760000000"
                }
                "beta"
                {
                    "buildid"       "15000250"
                    "timeupdated"   "1761000000"
                    "pwdrequired"   "1"
                    "description"   "Experimental test branch"
                }
            }
        }
    }
    """
)

SAMPLE_LICENSES_OUTPUT = textwrap.dedent(
    """\
    Logging in user 'testplayer' to Steam Public...OK
    Waiting for client config...OK
    Waiting for user info...OK
    License packageID 100001:
     - State   : Active( flags 0 ) - Purchased : Sat Jun 2 12:43:06 2024 in "US", Wallet
     - Apps    : 1091500, 2138330, (2 in total)
     - Depots  : 1091501, 2138332, (2 in total)
    License packageID 100002: - State : Active( flags 0 ) - Purchased : Mon Jan 1 00:00:00 2025 in "US", CD Key - Apps : 367520, (1 in total) - Depots : 367521, (1 in total)
    License packageID 100003: - State : Revoked( flags 0 ) - Purchased : Mon Jan 1 00:00:00 2024 in "US", Refund - Apps : 999999, (1 in total) - Depots : 999998, (1 in total)
    """
)


def _write_mock_steamcmd(
    bin_path: Path,
    log_path: Path,
    *,
    appinfo_path: Path,
    steam_home: Path,
    fail_login: bool = False,
) -> Path:
    script = textwrap.dedent(
        f"""\
        #!/usr/bin/env python3
        import json, os, pathlib, sys

        argv = sys.argv[1:]
        log_file = pathlib.Path({str(log_path)!r})
        with log_file.open("a", encoding="utf-8") as fh:
            fh.write(json.dumps(argv) + "\\n")

        if {fail_login!r}:
            print("FAILED (Invalid Password / Steam Guard required)")
            sys.exit(5)

        joined = " ".join(argv)
        if "+licenses_print" in argv:
            print({SAMPLE_LICENSES_OUTPUT!r})
            sys.exit(0)

        if "+app_info_print" in argv:
            print(pathlib.Path({str(appinfo_path)!r}).read_text(encoding="utf-8"))
            sys.exit(0)

        if "+app_update" in argv:
            dest = None
            if "+force_install_dir" in argv:
                idx = argv.index("+force_install_dir")
                dest = pathlib.Path(argv[idx + 1])
            app_idx = argv.index("+app_update")
            app_id = argv[app_idx + 1]
            if dest is not None:
                (dest / "bin" / "x64").mkdir(parents=True, exist_ok=True)
                (dest / "bin" / "x64" / "SampleGame.exe").write_bytes(b"MZ\\x90\\x00MOCKPE")
                (dest / "bin" / "x64" / "steam_api64.dll").write_bytes(b"MZ\\x90\\x00STEAM")
                (dest / "installscript.vdf").write_text(
                    '"InstallScript" {{"Run Process" {{"VCRedist" {{"process 1" "%INSTALLDIR%\\\\_CommonRedist\\\\vcredist\\\\vc_redist.x64.exe" "command 1" "/quiet"}}}}}}\\n',
                    encoding="utf-8",
                )
                sa = dest / "steamapps"
                sa.mkdir(parents=True, exist_ok=True)
                build_id = os.environ.get("MOCK_BUILD_ID", "15000100")
                (sa / f"appmanifest_{{app_id}}.acf").write_text(
                    f'"AppState" {{"appid" "{{app_id}}" "name" "Sample Action RPG" "buildid" "{{build_id}}"}}\\n',
                    encoding="utf-8",
                )
            if "validate" in argv:
                print(f"Success! App '{{app_id}}' fully installed.")
            else:
                print(f"Success! App '{{app_id}}' fully installed.")
            sys.exit(0)

        if "+download_depot" in argv:
            idx = argv.index("+download_depot")
            app_id = argv[idx + 1]
            depot_id = argv[idx + 2]
            out_dir = pathlib.Path({str(steam_home)!r}) / "steamapps" / "content" / f"app_{{app_id}}" / f"depot_{{depot_id}}"
            out_dir.mkdir(parents=True, exist_ok=True)
            (out_dir / f"depot_{{depot_id}}.bin").write_bytes(b"MOCK_DEPOT")
            print(f"Depot download complete : {{out_dir}}")
            sys.exit(0)

        # Plain `+login <user> +quit`
        print("Logging in user 'testplayer' to Steam Public...OK")
        sys.exit(0)
        """
    )
    bin_path.write_text(script, encoding="utf-8")
    bin_path.chmod(bin_path.stat().st_mode | stat.S_IXUSR)
    return bin_path


def _write_mock_depot_downloader(bin_path: Path, log_path: Path) -> Path:
    script = textwrap.dedent(
        f"""\
        #!/usr/bin/env python3
        import json, pathlib, sys

        argv = sys.argv[1:]
        with pathlib.Path({str(log_path)!r}).open("a", encoding="utf-8") as fh:
            fh.write(json.dumps(argv) + "\\n")
        if "-dir" in argv:
            dest = pathlib.Path(argv[argv.index("-dir") + 1])
            dest.mkdir(parents=True, exist_ok=True)
            (dest / "Game.exe").write_bytes(b"MZ\\x90\\x00DD")
        print("Total downloaded: 1024 bytes")
        sys.exit(0)
        """
    )
    bin_path.write_text(script, encoding="utf-8")
    bin_path.chmod(bin_path.stat().st_mode | stat.S_IXUSR)
    return bin_path


class SteamVdfParserTests(unittest.TestCase):
    def test_parse_vdf_and_extract_appinfo(self) -> None:
        appinfo, change_num = extract_appinfo_vdf(SAMPLE_APPINFO_VDF, "1091500")
        self.assertEqual(change_num, "24501999")
        self.assertEqual(appinfo["common"]["name"], "Sample Action RPG")
        self.assertEqual(appinfo["depots"]["branches"]["beta"]["pwdrequired"], "1")
        self.assertEqual(appinfo["depots"]["228989"]["depotfromapp"], "228980")

    def test_parse_licenses_print_filters_active_and_revoked(self) -> None:
        licenses = parse_licenses_print(SAMPLE_LICENSES_OUTPUT)
        self.assertEqual(len(licenses), 3)
        self.assertTrue(licenses[0]["active"])
        self.assertEqual(licenses[0]["app_ids"], ["1091500", "2138330"])
        self.assertTrue(licenses[1]["active"])
        self.assertEqual(licenses[1]["app_ids"], ["367520"])
        self.assertFalse(licenses[2]["active"])
        self.assertEqual(licenses[2]["app_ids"], ["999999"])

    def test_parse_loginusers_and_installscript(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            vdf_path = Path(td) / "loginusers.vdf"
            vdf_path.write_text(
                textwrap.dedent(
                    """\
                    "users"
                    {
                        "76561198000000001"
                        {
                            "AccountName"       "olderuser"
                            "PersonaName"       "Older"
                            "MostRecent"        "0"
                            "RememberPassword"  "1"
                        }
                        "76561198000000002"
                        {
                            "AccountName"       "testplayer"
                            "PersonaName"       "Test Player"
                            "MostRecent"        "1"
                            "RememberPassword"  "1"
                        }
                    }
                    """
                ),
                encoding="utf-8",
            )
            users = parse_loginusers_vdf(vdf_path)
            self.assertEqual(len(users), 2)
            self.assertEqual(users[0]["account_name"], "testplayer")
            self.assertEqual(users[0]["steam_id64"], "76561198000000002")

        parsed_script = parse_installscript_vdf(
            '"InstallScript" { "Run Process" { "VC2022" { "process 1" "_CommonRedist\\\\vc_redist.x64.exe" "command 1" "/quiet" } } "Registry" { "HKEY_LOCAL_MACHINE\\\\SOFTWARE\\\\Sample" { "string" { "Lang" "en" } } } }'
        )
        self.assertEqual(len(parsed_script["run_processes"]), 1)
        self.assertEqual(parsed_script["run_processes"][0]["name"], "VC2022")
        self.assertEqual(len(parsed_script["registry_keys"]), 1)

    def test_parse_vdf_handles_comments_and_escapes(self) -> None:
        parsed = parse_vdf(
            '// comment\n"root" { "path" "C:\\\\Games\\\\\\"Test\\"" "winrel" "bin\\x64\\Game.exe" }'
        )
        self.assertEqual(parsed["root"]["path"], 'C:\\Games\\"Test"')
        self.assertEqual(parsed["root"]["winrel"], "bin\\x64\\Game.exe")


class SteamBackendTests(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        self.steam_home = self.root / "steam_home"
        self.state_dir = self.root / "state"
        self.log_path = self.root / "steamcmd.log"
        self.appinfo_path = self.root / "appinfo.vdf"
        self.appinfo_path.write_text(SAMPLE_APPINFO_VDF, encoding="utf-8")

        (self.steam_home / "config").mkdir(parents=True, exist_ok=True)
        (self.steam_home / "config" / "loginusers.vdf").write_text(
            textwrap.dedent(
                """\
                "users"
                {
                    "76561198000000002"
                    {
                        "AccountName"       "testplayer"
                        "PersonaName"       "Test Player"
                        "MostRecent"        "1"
                        "RememberPassword"  "1"
                    }
                }
                """
            ),
            encoding="utf-8",
        )
        (self.steam_home / "config" / "config.vdf").write_text(
            '"InstallConfigStore" { "Software" { "Valve" { "Steam" { "ConnectCache" { "abc" "DO_NOT_LEAK_TOKEN_999" } } } } }\n',
            encoding="utf-8",
        )

        self.mock_scmd = _write_mock_steamcmd(
            self.root / "mock-steamcmd",
            self.log_path,
            appinfo_path=self.appinfo_path,
            steam_home=self.steam_home,
        )

    def tearDown(self) -> None:
        self._tmp.cleanup()

    def _make_backend(self, **kwargs) -> SteamBackend:
        defaults = {
            "steamcmd_bin": self.mock_scmd,
            "steam_home": self.steam_home,
            "state_dir": self.state_dir,
            "allow_network": False,
            "rosetta_check": lambda: True,
        }
        defaults.update(kwargs)
        return SteamBackend(**defaults)

    def _read_invocations(self) -> list[list[str]]:
        if not self.log_path.is_file():
            return []
        return [json.loads(line) for line in self.log_path.read_text(encoding="utf-8").splitlines() if line.strip()]

    def test_registry_discovers_steam_only_not_helpers(self) -> None:
        backends = available_backends()
        self.assertIn("steam", backends)
        self.assertNotIn("_steam_vdf", backends)
        self.assertIsInstance(get_backend("steam"), SteamBackend)

    def test_login_status_logout_and_zero_secret_leaks(self) -> None:
        backend = self._make_backend()
        auth = backend.login(username="testplayer")
        self.assertTrue(auth.logged_in)
        self.assertEqual(auth.display_name, "testplayer")

        session_text = (self.state_dir / "steam-session.json").read_text(encoding="utf-8")
        self.assertIn("testplayer", session_text)
        self.assertNotIn("DO_NOT_LEAK_TOKEN_999", session_text)

        st = backend.status(offline=False)
        st_json = json.dumps(st.to_dict())
        self.assertTrue(st.logged_in)
        self.assertEqual(st.display_name, "testplayer")
        self.assertNotIn("DO_NOT_LEAK_TOKEN_999", st_json)
        self.assertNotIn("password", st_json.lower())
        self.assertNotIn("token", st_json.lower())

        logged_out = backend.login(logout=True)
        self.assertFalse(logged_out.logged_in)
        self.assertIsNone(logged_out.display_name)

        # Logout must persist even when `loginusers.vdf` / `config.vdf` still exist on disk
        st_after_logout = backend.status(offline=True)
        self.assertFalse(st_after_logout.logged_in)
        self.assertIsNone(st_after_logout.display_name)
        with self.assertRaises(AuthRequiredError):
            backend.download(DownloadOptions(app_id="1091500", dest=self.root / "after_logout"))

        # Explicit re-import restores the logged-in session
        reimported = backend.login(import_existing=True)
        self.assertTrue(reimported.logged_in)
        self.assertEqual(reimported.display_name, "testplayer")

    def test_status_reports_logged_out_on_steam_guard_failure(self) -> None:
        failing_scmd = _write_mock_steamcmd(
            self.root / "fail-steamcmd",
            self.log_path,
            appinfo_path=self.appinfo_path,
            steam_home=self.steam_home,
            fail_login=True,
        )
        backend = self._make_backend(steamcmd_bin=failing_scmd)
        st = backend.status(offline=False)
        self.assertFalse(st.logged_in)
        self.assertIsNone(st.display_name)

    def test_list_owned_via_licenses_print_and_search_enrichment(self) -> None:
        def fake_http(url: str):
            if "storesearch" in url:
                return {
                    "items": [
                        {"id": 367520, "name": "Hollow Knight"},
                        {"id": 888888, "name": "Unowned Hollow Game"},
                    ]
                }
            return {}

        backend = self._make_backend(allow_network=True, http_get_json=fake_http)
        backend.login(username="testplayer")

        owned = backend.list_owned(platform="windows")
        owned_ids = [g.app_id for g in owned]
        self.assertIn("1091500", owned_ids)
        self.assertIn("367520", owned_ids)
        self.assertNotIn("999999", owned_ids)  # Revoked license excluded

        matches = backend.search("Hollow Knight")
        self.assertEqual(len(matches), 1)
        self.assertEqual(matches[0].app_id, "367520")
        self.assertEqual(matches[0].title, "Hollow Knight")
        # Must never return unowned Store games
        self.assertTrue(all(m.app_id != "888888" for m in matches))

    def test_list_owned_via_steam_web_api_never_leaks_key(self) -> None:
        seen_urls: list[str] = []

        def fake_http(url: str):
            seen_urls.append(url)
            return {
                "response": {
                    "game_count": 1,
                    "games": [{"appid": 1091500, "name": "Sample Action RPG"}],
                }
            }

        prev = os.environ.get("PORT_STEAM_API_KEY")
        os.environ["PORT_STEAM_API_KEY"] = "SECRET_WEB_API_KEY_12345"
        try:
            backend = self._make_backend(allow_network=True, http_get_json=fake_http)
            backend.login(username="testplayer")
            owned = backend.list_owned()
            self.assertEqual(len(owned), 1)
            self.assertEqual(owned[0].title, "Sample Action RPG")
            self.assertTrue(any("GetOwnedGames" in u for u in seen_urls))
            serialized = json.dumps([g.to_dict() for g in owned])
            self.assertNotIn("SECRET_WEB_API_KEY_12345", serialized)
        finally:
            if prev is None:
                os.environ.pop("PORT_STEAM_API_KEY", None)
            else:
                os.environ["PORT_STEAM_API_KEY"] = prev

    def test_info_reports_depots_sizes_dlcs_redists_proton_drm_anticheat(self) -> None:
        def fake_http(url: str):
            if "appdetails" in url:
                return {
                    "1091500": {
                        "success": True,
                        "data": {
                            "name": "Sample Action RPG",
                            "drm_notice": "Incorporates 3rd-party DRM: Denuvo Anti-Tamper",
                            "categories": [{"description": "Steam Achievements"}],
                        },
                    }
                }
            return {}

        backend = self._make_backend(allow_network=True, http_get_json=fake_http)
        info = backend.info("1091500", platform="windows", offline=False)

        self.assertEqual(info.title, "Sample Action RPG")
        self.assertEqual(info.version, "15000100")
        self.assertEqual(info.download_size_bytes, 42949672960)
        self.assertEqual(info.disk_size_bytes, 53687091200)
        self.assertEqual(info.executable, "bin/x64/SampleGame.exe")

        depot_ids = {d.depot_id for d in info.depots}
        self.assertIn("1091501", depot_ids)
        self.assertIn("2138332", depot_ids)
        self.assertIn("228989", depot_ids)
        self.assertIn("228990", depot_ids)

        branch_names = {b["name"] for b in info.branches}
        self.assertEqual(branch_names, {"public", "beta"})

        dlc_ids = [d.app_id for d in info.dlcs]
        self.assertEqual(dlc_ids, ["2138330", "2138331"])

        redist_ids = {r.id for r in info.redistributables}
        self.assertIn("steam_redist:228989", redist_ids)
        self.assertIn("steam_redist:228990", redist_ids)
        self.assertIn("steam_redist:229007", redist_ids)

        self.assertEqual(info.proton_redist_info["shared_redist_app_id"], "228980")
        self.assertEqual(
            info.proton_redist_info["steam_deck_compatibility"]["recommended_runtime"],
            "proton-9.0-3",
        )

        # Report-only Steamworks / DRM / anti-cheat checks
        self.assertTrue(info.requires_steamworks)
        self.assertTrue(info.drm.detected)
        self.assertTrue(info.drm.report_only)
        self.assertTrue(info.anticheat.detected)
        self.assertTrue(info.anticheat.report_only)

    def test_download_branch_dlcs_redists_verify_update_uninstall_and_manifest(self) -> None:
        backend = self._make_backend()
        backend.login(username="testplayer")
        dest = self.root / "games" / "SampleActionRPG"

        dl = backend.download(
            DownloadOptions(
                app_id="1091500",
                dest=dest,
                platform="windows",
                with_dlcs=True,
                skip_dlcs=False,
                with_redists=True,
                branch="beta",
                branch_password="TOP_SECRET_BETA_PW",
                validate=True,
            )
        )
        self.assertEqual(dl.app_id, "1091500")
        self.assertEqual(dl.branch, "beta")
        self.assertEqual(dl.downloaded_dlcs, ["2138330"])
        self.assertIn("228989", dl.downloaded_redists)
        self.assertIn("228990", dl.downloaded_redists)
        self.assertTrue((dest / "steam_appid.txt").is_file())
        self.assertTrue((dest / "_CommonRedist" / "228989" / "depot_228989.bin").is_file())

        invocations = self._read_invocations()
        app_update_calls = [c for c in invocations if "+app_update" in c]
        self.assertEqual(len(app_update_calls), 1)
        cmd = app_update_calls[0]
        self.assertIn("+@sSteamCmdForcePlatformType", cmd)
        self.assertIn("windows", cmd)
        self.assertIn("-beta", cmd)
        self.assertIn("beta", cmd)
        self.assertIn("validate", cmd)
        self.assertNotIn("TOP_SECRET_BETA_PW", " ".join(_redact_cmd(cmd)))

        # Verify
        ver = backend.verify("1091500", repair=True)
        self.assertTrue(ver.ok)

        # Update check-only detects newer build on beta branch (15000100 -> 15000250)
        upd_check = backend.update("1091500", check_only=True)
        self.assertTrue(upd_check.update_available)
        self.assertFalse(upd_check.updated)
        self.assertEqual(upd_check.old_version, "15000100")
        self.assertEqual(upd_check.new_version, "15000250")

        # Launch args
        plan = backend.launch_args("1091500", offline=True)
        self.assertEqual(plan.executable, "bin/x64/SampleGame.exe")
        self.assertEqual(plan.args, ["-skipStartScreen"])

        # Uninstall keeping files first, then re-register and delete
        uninst_keep = backend.uninstall("1091500", keep_files=True)
        self.assertTrue(uninst_keep.uninstalled)
        self.assertTrue(dest.is_dir())

        # Re-download and full uninstall removes files
        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))
        uninst_del = backend.uninstall("1091500", keep_files=False)
        self.assertTrue(uninst_del.uninstalled)
        self.assertFalse(dest.exists())

    def test_depot_downloader_fallback_when_steamcmd_or_rosetta_absent(self) -> None:
        dd_log = self.root / "dd.log"
        dd_bin = _write_mock_depot_downloader(self.root / "mock-dd", dd_log)
        backend = self._make_backend(
            steamcmd_bin=self.root / "missing-steamcmd",
            depot_downloader_bin=dd_bin,
            rosetta_check=lambda: False,
        )
        backend._save_session("testplayer", "76561198000000002")
        dest = self.root / "games" / "FallbackGame"
        res = backend.download(
            DownloadOptions(app_id="1091500", dest=dest, platform="windows", branch="beta", validate=True)
        )
        self.assertEqual(res.app_id, "1091500")

        # Verify and update must also reach DepotDownloader fallback without requiring steamcmd
        ver = backend.verify("1091500", repair=True)
        self.assertTrue(ver.ok)

        with self.assertRaises(UnsupportedOperationError):
            backend.update("1091500", check_only=True)

        upd = backend.update("1091500", check_only=False)
        self.assertTrue(upd.updated)

        calls = [json.loads(ln) for ln in dd_log.read_text(encoding="utf-8").splitlines() if ln.strip()]
        self.assertEqual(len(calls), 3)
        self.assertIn("-os", calls[0])
        self.assertIn("windows", calls[0])
        self.assertIn("-branch", calls[0])
        self.assertIn("beta", calls[0])
        self.assertIn("-validate", calls[0])
        self.assertIn("-validate", calls[1])

    def test_quoted_launch_args_and_secret_redaction(self) -> None:
        parsed = _split_windows_args(
            '-config "My Settings.ini" -path "C:\\Games\\Sample" -password "my secret pass" --flag'
        )
        self.assertEqual(
            parsed,
            ["-config", "My Settings.ini", "-path", "C:\\Games\\Sample", "-password", "my secret pass", "--flag"],
        )

        # Verify launch_args() preserves quoted paths and redacts entire quoted secret values
        quoted_appinfo = SAMPLE_APPINFO_VDF.replace(
            '"-skipStartScreen"',
            '"-config \\"My Settings.ini\\" -password \\"my secret pass\\" -windowed"',
        )
        self.appinfo_path.write_text(quoted_appinfo, encoding="utf-8")
        backend = self._make_backend()
        backend.login(username="testplayer")
        dest = self.root / "games" / "QuotedArgsGame"
        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))
        plan = backend.launch_args("1091500", offline=True)
        self.assertEqual(plan.args, ["-config", "My Settings.ini", "-windowed"])
        self.assertTrue(plan.redacted_auth_args)
        self.assertNotIn("secret", " ".join(plan.args).lower())

    def test_cli_end_to_end_json_and_package_manifest(self) -> None:
        dest = self.root / "cli_dest" / "SampleRPG"
        toml_out = self.root / "starter_game.toml"
        env_updates = {
            "PORT_STEAMCMD": str(self.mock_scmd),
            "PORT_STEAM_HOME": str(self.steam_home),
            "PORT_STEAM_USER": "testplayer",
            "MOCK_BETA_PW": "NEVER_WRITE_THIS_PW",
        }
        old_env = {k: os.environ.get(k) for k in env_updates}
        os.environ.update(env_updates)
        try:
            def run_cli(argv: list[str]) -> tuple[int, dict]:
                buf = io.StringIO()
                with contextlib.redirect_stdout(buf):
                    rc = cli_main(argv)
                return rc, json.loads(buf.getvalue())

            rc, st = run_cli(["--store", "steam", "--json", "status"])
            self.assertEqual(rc, 0)
            self.assertTrue(st["ok"])
            self.assertTrue(st["logged_in"])
            self.assertEqual(st["display_name"], "testplayer")

            rc, owned = run_cli(["--store", "steam", "--json", "list-owned"])
            self.assertEqual(rc, 0)
            self.assertTrue(owned["ok"])
            self.assertGreaterEqual(owned["count"], 2)

            rc, dlcs = run_cli(["--store", "steam", "--json", "list-dlcs", "1091500"])
            self.assertEqual(rc, 0)
            self.assertEqual(dlcs["count"], 2)

            rc, redists = run_cli(["--store", "steam", "--json", "list-redists", "1091500"])
            self.assertEqual(rc, 0)
            self.assertGreaterEqual(redists["count"], 3)

            rc, info = run_cli(["--store", "steam", "--json", "info", "1091500"])
            self.assertEqual(rc, 0)
            self.assertEqual(info["title"], "Sample Action RPG")
            self.assertTrue(info["requires_steamworks"])
            self.assertTrue(len(info["depots"]) >= 5)

            rc, dl = run_cli(
                [
                    "--store",
                    "steam",
                    "--json",
                    "download",
                    "1091500",
                    "--dest",
                    str(dest),
                    "--platform",
                    "windows",
                    "--branch",
                    "beta",
                    "--branch-password-env",
                    "MOCK_BETA_PW",
                    "--with-dlcs",
                    "--with-redists",
                    "--validate",
                ]
            )
            self.assertEqual(rc, 0)
            self.assertTrue(dl["ok"])
            manifest_path = Path(dl["manifest_path"])
            self.assertTrue(manifest_path.is_file())

            manifest_raw = manifest_path.read_text(encoding="utf-8")
            self.assertNotIn("NEVER_WRITE_THIS_PW", manifest_raw)
            manifest = read_package_manifest(manifest_path)
            self.assertEqual(manifest["store"], "steam")
            self.assertEqual(manifest["branch"], "beta")
            self.assertEqual(manifest["executable"], "bin/x64/SampleGame.exe")
            self.assertEqual(manifest["proton_redist_info"]["shared_redist_app_id"], "228980")
            self.assertTrue(manifest["compatibility_signals"]["requires_steamworks"])
            self.assertEqual(manifest["packaging_hook"]["format"], "port-acquire-manifest-v1")

            rc, found = run_cli(["--store", "steam", "--json", "search", "Sample Action"])
            self.assertEqual(rc, 0)
            self.assertEqual(found["matches"][0]["app_id"], "1091500")

            rc, ver = run_cli(["--store", "steam", "--json", "verify", "1091500", "--repair"])
            self.assertEqual(rc, 0)
            self.assertTrue(ver["ok"])

            rc, upd = run_cli(["--store", "steam", "--json", "update", "1091500", "--check-only"])
            self.assertEqual(rc, 0)
            self.assertTrue(upd["update_available"])

            rc, launch = run_cli(
                [
                    "--store",
                    "steam",
                    "--json",
                    "launch-args",
                    "1091500",
                    "--export-game-toml",
                    str(toml_out),
                ]
            )
            self.assertEqual(rc, 0)
            self.assertTrue(launch["ok"])
            toml_text = toml_out.read_text(encoding="utf-8")
            self.assertIn('steam_app_id = "1091500"', toml_text)
            self.assertIn('exe = "bin/x64/SampleGame.exe"', toml_text)

            rc, uninst = run_cli(["--store", "steam", "--json", "uninstall", "1091500"])
            self.assertEqual(rc, 0)
            self.assertTrue(uninst["uninstalled"])
            self.assertFalse(dest.exists())
        finally:
            for k, v in old_env.items():
                if v is None:
                    os.environ.pop(k, None)
                else:
                    os.environ[k] = v

    def test_unauthenticated_download_raises_auth_required(self) -> None:
        empty_home = self.root / "empty_steam_home"
        empty_home.mkdir()
        backend = self._make_backend(steam_home=empty_home)
        prev_user = os.environ.pop("PORT_STEAM_USER", None)
        try:
            with self.assertRaises(AuthRequiredError):
                backend.download(DownloadOptions(app_id="1091500", dest=self.root / "out"))
        finally:
            if prev_user is not None:
                os.environ["PORT_STEAM_USER"] = prev_user


if __name__ == "__main__":
    unittest.main()
