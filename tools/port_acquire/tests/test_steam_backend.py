"""Offline unit tests for the native arm64 Steam backend (`tools/port_acquire/backends/steam.py`).

Uses a self-contained mocked `DepotDownloader` executable so CI passes with zero
network access and zero real credentials.
"""

from __future__ import annotations

import contextlib
import hashlib
import io
import json
import os
from pathlib import Path
import stat
import struct
import sys
import tempfile
import textwrap
import unittest

_TOOLS_DIR = Path(__file__).resolve().parents[2]
if str(_TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOLS_DIR))

from port_acquire import (  # noqa: E402
    AcquireError,
    AuthRequiredError,
    CommandFailedError,
    DownloadOptions,
    available_backends,
    get_backend,
    read_package_manifest,
)
from port_acquire.backends._steam_vdf import (  # noqa: E402
    parse_appinfo_vdf_dlc_depots,
    parse_depot_downloader_stdout,
    parse_depot_manifest_txt,
    parse_installscript_vdf,
    parse_local_owned_app_ids,
    parse_loginusers_vdf,
    parse_vdf,
)
from port_acquire.backends.steam import SteamBackend, _redact_cmd, _split_windows_args  # noqa: E402
from port_acquire.cli import main as cli_main  # noqa: E402


SAMPLE_MANIFEST_TXT = textwrap.dedent(
    """\
    Content Manifest for Depot 1091501

    Manifest ID / date       : 8811223344556677889 / 11/26/2026 12:00:00
    Total number of files    : 5
    Total number of chunks   : 42
    Total bytes on disk      : 53687091200
    Total bytes compressed   : 42949672960

              Size Chunks File SHA                                 Flags Name
          10485760      4 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa     0 bin\\x64\\SampleGame.exe
           2097152      1 bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     0 bin\\x64\\steam_api64.dll
           1048576      1 cccccccccccccccccccccccccccccccccccccccc     0 EasyAntiCheat\\EasyAntiCheat_EOS_Setup.exe
          25165824      2 dddddddddddddddddddddddddddddddddddddddd     0 _CommonRedist\\vcredist\\2022\\vc_redist.x64.exe
           8388608      1 eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee     0 _CommonRedist\\DirectX\\Jun2010\\DXSETUP.exe
    """
)

SAMPLE_UPDATED_MANIFEST_TXT = SAMPLE_MANIFEST_TXT.replace(
    "8811223344556677889", "9922334455667788990"
)


def _write_mock_appinfo_vdf(
    steam_home: Path,
    app_id: int,
    depots: dict[str, str | None],
    *,
    v29: bool = False,
) -> Path:
    """Write a minimal valid Steam binary `appcache/appinfo.vdf` (v28 or v29)."""
    strings: list[str] = []
    string_idx: dict[str, int] = {}

    def _key_bytes(name: str) -> bytes:
        if not v29:
            return name.encode("utf-8") + b"\x00"
        if name not in string_idx:
            string_idx[name] = len(strings)
            strings.append(name)
        return struct.pack("<I", string_idx[name])

    def _encode_map(d: dict) -> bytes:
        buf = bytearray()
        for k, v in d.items():
            if isinstance(v, dict):
                buf.append(0x00)
                buf.extend(_key_bytes(k))
                buf.extend(_encode_map(v))
            else:
                buf.append(0x01)
                buf.extend(_key_bytes(k))
                buf.extend(str(v).encode("utf-8") + b"\x00")
        buf.append(0x08)
        return bytes(buf)

    depots_node: dict = {}
    for did, dlc_app_id in depots.items():
        node: dict[str, str] = {"name": f"Depot {did}"}
        if dlc_app_id is not None:
            node["dlcappid"] = dlc_app_id
        depots_node[did] = node

    payload = _encode_map({"appinfo": {"depots": depots_node}})
    entry_hdr = struct.pack("<IIQ20sI20s", 2, 1700000000, 0, b"\x00" * 20, 1, b"\x00" * 20)
    entry_body = entry_hdr + payload
    entry = struct.pack("<II", app_id, len(entry_body)) + entry_body + struct.pack("<I", 0)

    out = bytearray()
    if v29:
        str_off = 16 + len(entry)
        out.extend(struct.pack("<IIq", 0x07564429, 1, str_off))
        out.extend(entry)
        out.extend(struct.pack("<I", len(strings)))
        for s in strings:
            out.extend(s.encode("utf-8") + b"\x00")
    else:
        out.extend(struct.pack("<II", 0x07564428, 1))
        out.extend(entry)

    vdf_path = steam_home / "appcache" / "appinfo.vdf"
    vdf_path.parent.mkdir(parents=True, exist_ok=True)
    vdf_path.write_bytes(bytes(out))
    return vdf_path


def _write_mock_depot_downloader(
    bin_path: Path,
    log_path: Path,
    *,
    manifest_file_path: Path,
    fail_login: bool = False,
    empty_manifest: bool = False,
) -> Path:
    script = textwrap.dedent(
        f"""\
        #!/usr/bin/env python3
        import json, os, pathlib, sys

        argv = sys.argv[1:]
        if argv == ["--version"] or argv == ["-V"]:
            print("DepotDownloader v3.4.0+mock-arm64")
            sys.exit(0)

        log_file = pathlib.Path({str(log_path)!r})
        with log_file.open("a", encoding="utf-8") as fh:
            fh.write(json.dumps(argv) + "\\n")

        if {fail_login!r}:
            print("Unable to login to Steam3: InvalidPassword / Steam Guard required")
            sys.exit(1)

        app_id = argv[argv.index("-app") + 1] if "-app" in argv else "0"
        depot_args = []
        if "-depot" in argv:
            idx = argv.index("-depot") + 1
            while idx < len(argv) and not argv[idx].startswith("-"):
                depot_args.append(argv[idx])
                idx += 1
        branch = argv[argv.index("-branch") + 1] if "-branch" in argv else "public"
        dest = pathlib.Path(argv[argv.index("-dir") + 1]) if "-dir" in argv else None

        # Match real DepotDownloader 3.4.0: DLC AppIDs have no standalone depot list;
        # all DLC depots live under the base game's AppID.
        if app_id in ("2138330", "2138331", "2138332"):
            print(f"Error: Couldn't find any depots to download for app {{app_id}}")
            sys.exit(1)

        if "-manifest-only" in argv:
            if app_id == "228980":
                print("Using app branch: 'public'.")
                print("Got depot key for 228989 result: OK")
                print("Processing depot 228989 - Visual C++ 2022 Redistributable")
                print("Manifest 1111111111111111111 (11/26/2026 12:00:00)")
                sys.exit(0)
            if {empty_manifest!r} or os.environ.get("MOCK_EMPTY_MANIFEST") == "1":
                print("No depots available for manifest-only query")
                sys.exit(0)
            manifest_txt = pathlib.Path({str(manifest_file_path)!r}).read_text(encoding="utf-8")
            gid = "9922334455667788990" if "9922334455667788990" in manifest_txt else "8811223344556677889"
            if dest is not None:
                dest.mkdir(parents=True, exist_ok=True)
                (dest / f"manifest_1091501_{{gid}}.txt").write_text(manifest_txt, encoding="utf-8")
            print(f"Using app branch: '{{branch}}'.")
            print("Got depot key for 1091501 result: OK")
            print("Processing depot 1091501 - Sample Action RPG Win64 Content")
            print(f"Manifest {{gid}} (11/26/2026 12:00:00)")
            sys.exit(0)

        # Download or validate mode
        gid = os.environ.get("MOCK_MANIFEST_GID", "8811223344556677889")
        print(f"Using app branch: '{{branch}}'.")
        if dest is not None:
            dest.mkdir(parents=True, exist_ok=True)
            if app_id == "228980" and depot_args:
                for dep in depot_args:
                    (dest / f"redist_{{dep}}.bin").write_bytes(b"MOCK_REDIST")
                    print(f"Processing depot {{dep}} - Common Redist")
                    print("Manifest 1111111111111111111 (11/26/2026 12:00:00)")
                print("Total downloaded: 512 bytes")
                sys.exit(0)
            selected = depot_args or ["1091501"]
            if "1091501" in selected:
                (dest / "bin" / "x64").mkdir(parents=True, exist_ok=True)
                (dest / "bin" / "x64" / "SampleGame.exe").write_bytes(b"MZ\\x90\\x00MOCKPE")
                (dest / "bin" / "x64" / "steam_api64.dll").write_bytes(b"MZ\\x90\\x00STEAM")
                (dest / "installscript.vdf").write_text(
                    '"InstallScript" {{"Run Process" {{"VCRedist" {{"process 1" "%INSTALLDIR%\\\\_CommonRedist\\\\vcredist\\\\vc_redist.x64.exe" "command 1" "/quiet"}}}}}}\\n',
                    encoding="utf-8",
                )
                print("Processing depot 1091501 - Sample Action RPG Win64 Content")
                print(f"Manifest {{gid}} (11/26/2026 12:00:00)")
            if "999001" in selected:
                (dest / "dlc_2138330.pak").write_bytes(b"MOCK_DLC")
                print("Processing depot 999001 - Expansion Pack DLC Content")
                print("Manifest 770011 (11/26/2026 12:00:00)")
        print("Total downloaded: 4096 bytes")
        sys.exit(0)
        """
    )
    bin_path.write_text(script, encoding="utf-8")
    bin_path.chmod(bin_path.stat().st_mode | stat.S_IXUSR)
    return bin_path


class SteamVdfAndManifestParserTests(unittest.TestCase):
    def test_parse_depot_manifest_txt_and_stdout(self) -> None:
        parsed = parse_depot_manifest_txt(
            SAMPLE_MANIFEST_TXT,
            filename="manifest_1091501_8811223344556677889.txt",
        )
        self.assertEqual(parsed["depot_id"], "1091501")
        self.assertEqual(parsed["manifest_gid"], "8811223344556677889")
        self.assertEqual(parsed["disk_size_bytes"], 53687091200)
        self.assertEqual(parsed["download_size_bytes"], 42949672960)
        self.assertEqual(len(parsed["files"]), 5)
        self.assertEqual(parsed["files"][0]["path"], "bin/x64/SampleGame.exe")

        # DepotDownloader formats flags as lowercase hexadecimal (`:x`): 40 == 0x40 (Directory), a4 == 0xa4
        hex_manifest = textwrap.dedent(
            """\
            Content Manifest for Depot 1091501

            Manifest ID / date       : 8811223344556677889 / 11/26/2026 12:00:00
            Total number of files    : 2
            Total bytes on disk      : 100
            Total bytes compressed   : 80

                      Size Chunks File SHA                                 Flags Name
                         0      0 0000000000000000000000000000000000000000    40 bin
                       100      1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa    a4 bin\\tool.exe
            """
        )
        parsed_hex = parse_depot_manifest_txt(hex_manifest)
        self.assertEqual(len(parsed_hex["files"]), 2)
        self.assertEqual(parsed_hex["files"][0]["flags"], 0x40)
        self.assertEqual(parsed_hex["files"][1]["flags"], 0xA4)

        out = parse_depot_downloader_stdout(
            "Using app branch: 'beta'.\n"
            "Processing depot 1091501 - Sample Win64\n"
            "Manifest 8811223344556677889 (11/26/2026 12:00:00)\n"
            "Total downloaded: 123456 bytes\n"
        )
        self.assertEqual(out["branch"], "beta")
        self.assertEqual(out["depots"][0]["depot_id"], "1091501")
        self.assertEqual(out["depots"][0]["manifest_gid"], "8811223344556677889")
        self.assertEqual(out["total_downloaded_bytes"], 123456)

    def test_parse_loginusers_localconfig_and_installscript(self) -> None:
        with tempfile.TemporaryDirectory() as td:
            root = Path(td)
            vdf_path = root / "loginusers.vdf"
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

            cfg_dir = root / "userdata" / "12345" / "config"
            (cfg_dir / "librarycache").mkdir(parents=True)
            (cfg_dir / "librarycache" / "367520.json").write_text("{}", encoding="utf-8")
            (cfg_dir / "localconfig.vdf").write_text(
                '"UserLocalConfigStore" { "Software" { "Valve" { "Steam" { "apps" { "1091500" { "Playtime" "120" } "2138330" { "Playtime" "0" } } } } } }\n',
                encoding="utf-8",
            )
            local_ids = parse_local_owned_app_ids(root)
            self.assertEqual(local_ids, {"367520", "1091500", "2138330"})

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

    def test_parse_appinfo_vdf_dlc_depots_v28_and_v29(self) -> None:
        for v29 in (False, True):
            with tempfile.TemporaryDirectory() as td:
                root = Path(td)
                _write_mock_appinfo_vdf(
                    root,
                    1091500,
                    {"1091501": None, "999001": "2138330", "999002": "2138331"},
                    v29=v29,
                )
                mapping = parse_appinfo_vdf_dlc_depots(root, "1091500")
                self.assertEqual(mapping, {"2138330": ["999001"], "2138331": ["999002"]})
                self.assertEqual(parse_appinfo_vdf_dlc_depots(root, "9999999"), {})


class SteamBackendTests(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        self.steam_home = self.root / "steam_home"
        self.state_dir = self.root / "state"
        self.log_path = self.root / "depotdownloader.log"
        self.manifest_file = self.root / "sample_manifest.txt"
        self.manifest_file.write_text(SAMPLE_MANIFEST_TXT, encoding="utf-8")

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
        # 76561198000000002 - 76561197960265728 = 39734274
        cfg_dir = self.steam_home / "userdata" / "39734274" / "config"
        (cfg_dir / "librarycache").mkdir(parents=True, exist_ok=True)
        (cfg_dir / "librarycache" / "367520.json").write_text("{}", encoding="utf-8")
        (cfg_dir / "localconfig.vdf").write_text(
            '"UserLocalConfigStore" { "Software" { "Valve" { "Steam" { "apps" { "1091500" { "Playtime" "120" } "2138330" { "Playtime" "0" } } } } } }\n',
            encoding="utf-8",
        )

        self.mock_dd = _write_mock_depot_downloader(
            self.root / "mock-depotdownloader",
            self.log_path,
            manifest_file_path=self.manifest_file,
        )

    def tearDown(self) -> None:
        self._tmp.cleanup()

    def _make_backend(self, **kwargs) -> SteamBackend:
        defaults = {
            "depot_downloader_bin": self.mock_dd,
            "steam_home": self.steam_home,
            "state_dir": self.state_dir,
            "allow_network": False,
        }
        defaults.update(kwargs)
        return SteamBackend(**defaults)

    def _read_invocations(self) -> list[list[str]]:
        if not self.log_path.is_file():
            return []
        return [
            json.loads(line)
            for line in self.log_path.read_text(encoding="utf-8").splitlines()
            if line.strip()
        ]

    def test_registry_discovers_steam_only_not_helpers(self) -> None:
        backends = available_backends()
        self.assertIn("steam", backends)
        self.assertNotIn("_steam_vdf", backends)
        self.assertIsInstance(get_backend("steam"), SteamBackend)

    def test_login_status_logout_persistence_and_zero_secret_leaks(self) -> None:
        backend = self._make_backend()
        auth = backend.login(username="testplayer")
        self.assertTrue(auth.logged_in)
        self.assertEqual(auth.display_name, "testplayer")
        self.assertIn("DepotDownloader", auth.backend_version or "")

        # Verify DepotDownloader login used -username/-remember-password (never -password)
        invocations = self._read_invocations()
        self.assertEqual(len(invocations), 1)
        self.assertIn("-username", invocations[0])
        self.assertIn("testplayer", invocations[0])
        self.assertIn("-remember-password", invocations[0])
        self.assertIn("-manifest-only", invocations[0])
        self.assertNotIn("-password", invocations[0])

        session_text = (self.state_dir / "steam-session.json").read_text(encoding="utf-8")
        self.assertIn("testplayer", session_text)
        self.assertNotIn("token", session_text.lower())

        st = backend.status(offline=False)
        st_json = json.dumps(st.to_dict())
        self.assertTrue(st.logged_in)
        self.assertEqual(st.display_name, "testplayer")
        self.assertNotIn("password", st_json.lower())
        self.assertNotIn("token", st_json.lower())

        logged_out = backend.login(logout=True)
        self.assertFalse(logged_out.logged_in)
        self.assertIsNone(logged_out.display_name)

        # Logout must persist even when `loginusers.vdf` still exists on disk
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
        failing_dd = _write_mock_depot_downloader(
            self.root / "fail-dd",
            self.log_path,
            manifest_file_path=self.manifest_file,
            fail_login=True,
        )
        backend = self._make_backend(depot_downloader_bin=failing_dd)
        st = backend.status(offline=False)
        self.assertFalse(st.logged_in)
        self.assertIsNone(st.display_name)

    def test_list_owned_via_local_cache_and_search_enrichment(self) -> None:
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

        matches = backend.search("Hollow Knight")
        self.assertEqual(len(matches), 1)
        self.assertEqual(matches[0].app_id, "367520")
        self.assertEqual(matches[0].title, "Hollow Knight")
        self.assertTrue(all(m.app_id != "888888" for m in matches))

    def test_list_owned_via_steam_web_api_never_leaks_key(self) -> None:
        seen_urls: list[str] = []

        def fake_http(url: str):
            seen_urls.append(url)
            return {
                "response": {
                    "game_count": 2,
                    "games": [
                        {"appid": 1091500, "name": "Sample Action RPG"},
                        {"appid": 2138330, "name": "Expansion Pack DLC"},
                    ],
                }
            }

        prev = os.environ.get("PORT_STEAM_API_KEY")
        os.environ["PORT_STEAM_API_KEY"] = "SECRET_WEB_API_KEY_12345"
        try:
            backend = self._make_backend(allow_network=True, http_get_json=fake_http)
            backend.login(username="testplayer")
            owned = backend.list_owned()
            self.assertTrue(any(g.app_id == "1091500" and g.title == "Sample Action RPG" for g in owned))
            self.assertTrue(any("GetOwnedGames" in u for u in seen_urls))
            serialized = json.dumps([g.to_dict() for g in owned])
            self.assertNotIn("SECRET_WEB_API_KEY_12345", serialized)
        finally:
            if prev is None:
                os.environ.pop("PORT_STEAM_API_KEY", None)
            else:
                os.environ["PORT_STEAM_API_KEY"] = prev

    def test_info_reports_depots_sizes_owned_vs_unowned_dlcs_redists_drm_anticheat(self) -> None:
        def fake_http(url: str):
            if "appdetails" in url:
                return {
                    "1091500": {
                        "success": True,
                        "data": {
                            "name": "Sample Action RPG",
                            "dlc": [2138330, 2138331],
                            "drm_notice": "Incorporates 3rd-party DRM: Denuvo Anti-Tamper",
                            "categories": [{"description": "Steam Achievements"}],
                        },
                    }
                }
            return {}

        backend = self._make_backend(allow_network=True, http_get_json=fake_http)
        info = backend.info("1091500", platform="windows", offline=False)

        self.assertEqual(info.title, "Sample Action RPG")
        self.assertEqual(info.version, "8811223344556677889")
        self.assertEqual(info.download_size_bytes, 42949672960)
        self.assertEqual(info.disk_size_bytes, 53687091200)
        self.assertEqual(info.executable, "bin/x64/SampleGame.exe")
        self.assertTrue(info.manifest_files_checked)

        self.assertEqual([d.depot_id for d in info.depots], ["1091501"])

        # DLC ownership must be resolved honestly against account entitlements:
        # 2138330 is in localconfig.vdf (owned=True), 2138331 is NOT (owned=False).
        dlc_by_id = {d.app_id: d for d in info.dlcs}
        self.assertEqual(set(dlc_by_id.keys()), {"2138330", "2138331"})
        self.assertTrue(dlc_by_id["2138330"].owned)
        self.assertFalse(dlc_by_id["2138331"].owned)

        # Redistributables & Proton-adjacent info inferred from manifest file list
        redist_ids = {r.id for r in info.redistributables}
        self.assertIn("steam_redist:228989", redist_ids)
        self.assertIn("steam_redist:228990", redist_ids)
        self.assertEqual(info.proton_redist_info["shared_redist_app_id"], "228980")

        # Report-only Steamworks / DRM / anti-cheat checks
        self.assertTrue(info.requires_steamworks)
        self.assertTrue(info.drm.detected)
        self.assertTrue(info.drm.report_only)
        self.assertTrue(info.anticheat.detected)
        self.assertIn("Easy Anti-Cheat", info.anticheat.systems)
        self.assertTrue(info.anticheat.report_only)

    def test_download_branch_owned_dlcs_redists_verify_update_uninstall(self) -> None:
        # Map DLC 2138330 -> depot 999001 inside base AppID 1091500 (matching real Steamworks)
        _write_mock_appinfo_vdf(
            self.steam_home,
            1091500,
            {"1091501": None, "999001": "2138330", "999002": "2138331"},
        )

        def fake_http(url: str):
            if "appdetails" in url:
                return {
                    "1091500": {
                        "success": True,
                        "data": {
                            "name": "Sample Action RPG",
                            "dlc": [2138330, 2138331],
                        },
                    }
                }
            return {}

        backend = self._make_backend(allow_network=True, http_get_json=fake_http)
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
        # Only owned DLC 2138330 is downloaded (via -app 1091500 -depot 999001); unowned 2138331 is NOT
        self.assertEqual(dl.downloaded_dlcs, ["2138330"])
        self.assertTrue((dest / "dlc_2138330.pak").is_file())
        self.assertIn("228989", dl.downloaded_redists)
        self.assertIn("228990", dl.downloaded_redists)
        self.assertTrue((dest / "steam_appid.txt").is_file())
        self.assertTrue((dest / "_CommonRedist" / "228989" / "redist_228989.bin").is_file())

        invocations = self._read_invocations()
        # NEVER invoke DepotDownloader with a DLC AppID as -app
        self.assertTrue(
            all(c[c.index("-app") + 1] != "2138330" for c in invocations if "-app" in c)
        )
        # Base + owned DLC depots are transferred together in ONE invocation (`-app 1091500 -depot 1091501 999001`)
        app_dl_calls = [
            c
            for c in invocations
            if "-app" in c
            and c[c.index("-app") + 1] == "1091500"
            and "-manifest-only" not in c
        ]
        self.assertEqual(len(app_dl_calls), 1)
        cmd = app_dl_calls[0]
        self.assertIn("-depot", cmd)
        dep_idx = cmd.index("-depot") + 1
        self.assertEqual(cmd[dep_idx : dep_idx + 2], ["1091501", "999001"])
        self.assertIn("-os", cmd)
        self.assertIn("windows", cmd)
        self.assertIn("-osarch", cmd)
        self.assertIn("64", cmd)
        self.assertIn("-branch", cmd)
        self.assertIn("beta", cmd)
        self.assertIn("-validate", cmd)
        self.assertNotIn("TOP_SECRET_BETA_PW", " ".join(_redact_cmd(cmd)))

        # Requesting an unowned DLC explicitly must fail honestly
        with self.assertRaises(AcquireError) as unowned_ctx:
            backend.download(
                DownloadOptions(
                    app_id="1091500",
                    dest=dest,
                    platform="windows",
                    dlcs=("2138331",),
                    skip_dlcs=False,
                )
            )
        self.assertEqual(unowned_ctx.exception.code, "dlc_not_owned")

        # Verify & update require the private branch password (never silently falling back to public)
        with self.assertRaises(AcquireError) as missing_pw_ver:
            backend.verify("1091500", repair=True)
        self.assertEqual(missing_pw_ver.exception.code, "missing_branch_password")

        ver = backend.verify("1091500", repair=True, branch_password="TOP_SECRET_BETA_PW")
        self.assertTrue(ver.ok)
        self.assertTrue(ver.repaired)

        # Update check-only detects newer manifest GID
        self.manifest_file.write_text(SAMPLE_UPDATED_MANIFEST_TXT, encoding="utf-8")
        with self.assertRaises(AcquireError) as missing_pw_upd:
            backend.update("1091500", check_only=True)
        self.assertEqual(missing_pw_upd.exception.code, "missing_branch_password")

        upd_check = backend.update("1091500", check_only=True, branch_password="TOP_SECRET_BETA_PW")
        self.assertTrue(upd_check.update_available)
        self.assertFalse(upd_check.updated)
        self.assertEqual(upd_check.old_version, "8811223344556677889")
        self.assertEqual(upd_check.new_version, "9922334455667788990")

        # Launch args
        plan = backend.launch_args("1091500", offline=True)
        self.assertEqual(plan.executable, "bin/x64/SampleGame.exe")
        self.assertEqual(plan.working_dir, "bin/x64")

        # Uninstall keeping files, then re-download and full uninstall
        uninst_keep = backend.uninstall("1091500", keep_files=True)
        self.assertTrue(uninst_keep.uninstalled)
        self.assertTrue(dest.is_dir())

        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))
        uninst_del = backend.uninstall("1091500", keep_files=False)
        self.assertTrue(uninst_del.uninstalled)
        self.assertFalse(dest.exists())

    def test_update_does_not_treat_empty_manifest_as_up_to_date(self) -> None:
        backend = self._make_backend()
        backend.login(username="testplayer")
        dest = self.root / "games" / "EmptyManifestGame"
        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))

        prev = os.environ.get("MOCK_EMPTY_MANIFEST")
        os.environ["MOCK_EMPTY_MANIFEST"] = "1"
        try:
            with self.assertRaises(CommandFailedError):
                backend.update("1091500", check_only=True)

            before_dl = len(
                [c for c in self._read_invocations() if "-app" in c and c[c.index("-app") + 1] == "1091500" and "-manifest-only" not in c]
            )
            upd = backend.update("1091500", check_only=False)
            after_dl = len(
                [c for c in self._read_invocations() if "-app" in c and c[c.index("-app") + 1] == "1091500" and "-manifest-only" not in c]
            )
            self.assertTrue(upd.updated)
            self.assertEqual(after_dl, before_dl + 1)
        finally:
            if prev is None:
                os.environ.pop("MOCK_EMPTY_MANIFEST", None)
            else:
                os.environ["MOCK_EMPTY_MANIFEST"] = prev

    def test_quoted_launch_args_and_secret_redaction(self) -> None:
        parsed = _split_windows_args(
            '-config "My Settings.ini" -path "C:\\Games\\Sample" -password "my secret pass" --flag'
        )
        self.assertEqual(
            parsed,
            ["-config", "My Settings.ini", "-path", "C:\\Games\\Sample", "-password", "my secret pass", "--flag"],
        )

        backend = self._make_backend()
        dest = self.root / "games" / "QuotedArgsGame"
        (dest / "bin" / "x64").mkdir(parents=True, exist_ok=True)
        (dest / "bin" / "x64" / "SampleGame.exe").write_bytes(b"MZ")
        backend._save_installs(
            {
                "1091500": {
                    "app_id": "1091500",
                    "title": "Quoted Args Game",
                    "install_path": str(dest),
                    "launch_args": '-config "My Settings.ini" -password "my secret pass" -windowed',
                }
            }
        )
        plan = backend.launch_args("1091500", offline=True)
        self.assertEqual(plan.args, ["-config", "My Settings.ini", "-windowed"])
        self.assertTrue(plan.redacted_auth_args)
        self.assertNotIn("secret", " ".join(plan.args).lower())

    def test_resolve_prefers_exact_match_and_rejects_ambiguous_uninstall(self) -> None:
        backend = self._make_backend()
        p2_dir = self.root / "games" / "Portal2"
        p1_dir = self.root / "games" / "Portal"
        p2_dir.mkdir(parents=True, exist_ok=True)
        p1_dir.mkdir(parents=True, exist_ok=True)
        (p2_dir / "portal2.exe").write_bytes(b"MZ")
        (p1_dir / "hl2.exe").write_bytes(b"MZ")

        backend._save_installs(
            {
                "620": {"app_id": "620", "title": "Portal 2", "install_path": str(p2_dir)},
                "400": {"app_id": "400", "title": "Portal", "install_path": str(p1_dir)},
            }
        )

        res = backend.uninstall("Portal", keep_files=False)
        self.assertEqual(res.app_id, "400")
        self.assertFalse(p1_dir.exists())
        self.assertTrue(p2_dir.is_dir())

        with self.assertRaises(AcquireError) as ctx_single:
            backend.uninstall("Portal", keep_files=False)
        self.assertEqual(ctx_single.exception.code, "ambiguous_app")
        self.assertTrue(p2_dir.is_dir())

        p3_dir = self.root / "games" / "PortalStories"
        p3_dir.mkdir(parents=True, exist_ok=True)
        backend._save_installs(
            {
                "620": {"app_id": "620", "title": "Portal 2", "install_path": str(p2_dir)},
                "317400": {"app_id": "317400", "title": "Portal Stories: Mel", "install_path": str(p3_dir)},
            }
        )
        with self.assertRaises(AcquireError) as ctx:
            backend.uninstall("Portal", keep_files=False)
        self.assertEqual(ctx.exception.code, "ambiguous_app")
        self.assertTrue(p2_dir.is_dir())
        self.assertTrue(p3_dir.is_dir())

    def test_cli_end_to_end_json_and_unregistered_manifest(self) -> None:
        dest = self.root / "cli_dest" / "SampleRPG"
        unreg_dest = self.root / "cli_dest" / "UnregisteredRPG"
        toml_out = self.root / "starter_game.toml"
        env_updates = {
            "PORT_DEPOT_DOWNLOADER": str(self.mock_dd),
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

            rc, info = run_cli(["--store", "steam", "--json", "info", "1091500", "--offline"])
            self.assertEqual(rc, 0)
            self.assertEqual(info["app_id"], "1091500")

            # Unregistered download (`--no-register`) must still populate executable & working_dir
            rc, unreg_dl = run_cli(
                [
                    "--store",
                    "steam",
                    "--json",
                    "download",
                    "1091500",
                    "--dest",
                    str(unreg_dest),
                    "--platform",
                    "windows",
                    "--no-register",
                ]
            )
            self.assertEqual(rc, 0)
            unreg_manifest = read_package_manifest(Path(unreg_dl["manifest_path"]))
            self.assertEqual(unreg_manifest["executable"], "bin/x64/SampleGame.exe")
            self.assertEqual(unreg_manifest["working_dir"], "bin/x64")

            # Full registered download with branch, password env, redists, and validate
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

            rc, ver = run_cli(
                [
                    "--store",
                    "steam",
                    "--json",
                    "verify",
                    "1091500",
                    "--repair",
                    "--branch-password-env",
                    "MOCK_BETA_PW",
                ]
            )
            self.assertEqual(rc, 0)
            self.assertTrue(ver["ok"])
            self.assertNotIn("NEVER_WRITE_THIS_PW", json.dumps(ver))

            rc, upd = run_cli(
                [
                    "--store",
                    "steam",
                    "--json",
                    "update",
                    "1091500",
                    "--check-only",
                    "--branch-password-env",
                    "MOCK_BETA_PW",
                ]
            )
            self.assertEqual(rc, 0)
            self.assertTrue(upd["ok"])
            self.assertNotIn("NEVER_WRITE_THIS_PW", json.dumps(upd))

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

    def test_verify_readonly_detects_missing_and_corrupt_without_repairing(self) -> None:
        exe_bytes = b"MZ\x90\x00MOCKPE"
        dll_bytes = b"MZ\x90\x00STEAM"
        exe_sha1 = hashlib.sha1(exe_bytes).hexdigest()
        dll_sha1 = hashlib.sha1(dll_bytes).hexdigest()
        real_manifest = textwrap.dedent(
            f"""\
            Content Manifest for Depot 1091501

            Manifest ID / date       : 8811223344556677889 / 11/26/2026 12:00:00
            Total number of files    : 3
            Total number of chunks   : 2
            Total bytes on disk      : {len(exe_bytes) + len(dll_bytes)}
            Total bytes compressed   : {len(exe_bytes) + len(dll_bytes)}

                      Size Chunks File SHA                                 Flags Name
                         0      0 0000000000000000000000000000000000000000    40 bin
                         0      0 0000000000000000000000000000000000000000    40 bin\\x64
            {len(exe_bytes):14d}      1 {exe_sha1}   100 bin\\x64\\SampleGame.exe
            {len(dll_bytes):14d}      1 {dll_sha1}     0 bin\\x64\\steam_api64.dll
            """
        )
        self.manifest_file.write_text(real_manifest, encoding="utf-8")

        backend = self._make_backend()
        backend.login(username="testplayer")
        dest = self.root / "games" / "VerifyReadOnlyGame"
        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))

        # 1. Clean install passes read-only verification without invoking download/validate
        before_count = len(self._read_invocations())
        ok_ver = backend.verify("1091500", repair=False)
        self.assertTrue(ok_ver.ok)
        self.assertFalse(ok_ver.repaired)
        self.assertEqual(ok_ver.missing_files, [])
        self.assertEqual(ok_ver.corrupted_files, [])
        new_calls = self._read_invocations()[before_count:]
        self.assertEqual(len(new_calls), 1)
        self.assertIn("-manifest-only", new_calls[0])
        self.assertNotIn("-validate", new_calls[0])

        # 2. Delete one file and corrupt another; verify(repair=False) reports both and leaves disk untouched
        exe_path = dest / "bin" / "x64" / "SampleGame.exe"
        dll_path = dest / "bin" / "x64" / "steam_api64.dll"
        exe_path.unlink()
        dll_path.write_bytes(b"CORRUPTED!!")

        before_count = len(self._read_invocations())
        bad_ver = backend.verify("1091500", repair=False)
        self.assertFalse(bad_ver.ok)
        self.assertFalse(bad_ver.repaired)
        self.assertEqual(bad_ver.missing_files, ["bin/x64/SampleGame.exe"])
        self.assertEqual(bad_ver.corrupted_files, ["bin/x64/steam_api64.dll"])
        # Must stay strictly read-only: deleted file still absent, corrupted bytes untouched
        self.assertFalse(exe_path.exists())
        self.assertEqual(dll_path.read_bytes(), b"CORRUPTED!!")
        ro_calls = self._read_invocations()[before_count:]
        self.assertEqual(len(ro_calls), 1)
        self.assertIn("-manifest-only", ro_calls[0])
        self.assertNotIn("-validate", ro_calls[0])

        # 3. verify(repair=True) invokes DepotDownloader -validate and restores both files
        rep_ver = backend.verify("1091500", repair=True)
        self.assertTrue(rep_ver.ok)
        self.assertTrue(rep_ver.repaired)
        self.assertEqual(exe_path.read_bytes(), exe_bytes)
        self.assertEqual(dll_path.read_bytes(), dll_bytes)

    def test_verify_readonly_uses_last_depot_wins_for_overlapping_files(self) -> None:
        base_bytes = b"BASE_DEPOT_EXE_V1"
        override_bytes = b"OVERRIDE_DEPOT_EXE_V2_WINNING"
        base_sha1 = hashlib.sha1(base_bytes).hexdigest()
        override_sha1 = hashlib.sha1(override_bytes).hexdigest()

        # Deliberately give the first depot a higher ID (1091502) and the winning override
        # depot a lower ID (1091501) to prove stdout processing order is preserved rather
        # than sorting manifest filenames lexicographically.
        multi_dd = self.root / "multi-depot-dd"
        multi_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-multi")
                    sys.exit(0)
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                if "-manifest-only" in argv:
                    (dest / "manifest_1091502_111.txt").write_text(
                        "Content Manifest for Depot 1091502\\n"
                        "Manifest ID / date : 111 / 11/26/2026\\n"
                        "Size Chunks File SHA Flags Name\\n"
                        "0 0 0000000000000000000000000000000000000000 40 bin\\n"
                        "{len(base_bytes)} 1 {base_sha1} 0 bin\\\\SampleGame.exe\\n",
                        encoding="utf-8",
                    )
                    (dest / "manifest_1091501_222.txt").write_text(
                        "Content Manifest for Depot 1091501\\n"
                        "Manifest ID / date : 222 / 11/26/2026\\n"
                        "Size Chunks File SHA Flags Name\\n"
                        "{len(override_bytes)} 1 {override_sha1} 0 bin\\\\SampleGame.exe\\n",
                        encoding="utf-8",
                    )
                    print("Using app branch: 'public'.")
                    print("Processing depot 1091502 - Base Depot")
                    print("Manifest 111 (11/26/2026 12:00:00)")
                    print("Processing depot 1091501 - Override Depot")
                    print("Manifest 222 (11/26/2026 12:00:00)")
                    sys.exit(0)
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        multi_dd.chmod(multi_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=multi_dd)
        dest = self.root / "games" / "MultiDepotGame"
        (dest / "bin").mkdir(parents=True, exist_ok=True)
        backend._save_installs(
            {
                "1091500": {
                    "app_id": "1091500",
                    "title": "Multi Depot Game",
                    "account_name": "testplayer",
                    "install_path": str(dest),
                    "platform": "windows",
                    "branch": "public",
                }
            }
        )

        # Winning (last-processed) depot's bytes pass verification cleanly
        (dest / "bin" / "SampleGame.exe").write_bytes(override_bytes)
        ok_ver = backend.verify("1091500", repair=False)
        self.assertTrue(ok_ver.ok)
        self.assertEqual(ok_ver.missing_files, [])
        self.assertEqual(ok_ver.corrupted_files, [])

        # Overridden (first-processed) depot's bytes are flagged as corrupted
        (dest / "bin" / "SampleGame.exe").write_bytes(base_bytes)
        bad_ver = backend.verify("1091500", repair=False)
        self.assertFalse(bad_ver.ok)
        self.assertEqual(bad_ver.corrupted_files, ["bin/SampleGame.exe"])

    def test_verify_readonly_rejects_partial_or_failed_manifest_query(self) -> None:
        exe_bytes = b"MZ\x90\x00MOCKPE"
        exe_sha1 = hashlib.sha1(exe_bytes).hexdigest()

        partial_dd = self.root / "partial-fail-dd"
        partial_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import os, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-partial")
                    sys.exit(0)
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                if "-manifest-only" in argv:
                    (dest / "manifest_1091501_111.txt").write_text(
                        "Content Manifest for Depot 1091501\\n"
                        "Manifest ID / date : 111 / 11/26/2026\\n"
                        "Size Chunks File SHA Flags Name\\n"
                        "{len(exe_bytes)} 1 {exe_sha1} 0 bin\\\\SampleGame.exe\\n",
                        encoding="utf-8",
                    )
                    print("Using app branch: 'public'.")
                    print("Processing depot 1091501 - Base Depot")
                    print("Manifest 111 (11/26/2026 12:00:00)")
                    print("Processing depot 1091502 - Missing Depot")
                    sys.exit(int(os.environ.get("MOCK_EXIT_CODE", "1")))
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        partial_dd.chmod(partial_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=partial_dd)
        dest = self.root / "games" / "PartialVerifyGame"
        (dest / "bin").mkdir(parents=True, exist_ok=True)
        (dest / "bin" / "SampleGame.exe").write_bytes(exe_bytes)
        backend._save_installs(
            {
                "1091500": {
                    "app_id": "1091500",
                    "title": "Partial Verify Game",
                    "account_name": "testplayer",
                    "install_path": str(dest),
                    "platform": "windows",
                    "branch": "public",
                }
            }
        )

        # Case 1: DepotDownloader exits nonzero after writing 1 of 2 manifests -> must raise
        with self.assertRaises(CommandFailedError):
            backend.verify("1091500", repair=False)

        # Case 2: DepotDownloader exits 0 but second depot has no parsed manifest .txt -> must raise
        prev = os.environ.get("MOCK_EXIT_CODE")
        os.environ["MOCK_EXIT_CODE"] = "0"
        try:
            with self.assertRaises(CommandFailedError):
                backend.verify("1091500", repair=False)
        finally:
            if prev is None:
                os.environ.pop("MOCK_EXIT_CODE", None)
            else:
                os.environ["MOCK_EXIT_CODE"] = prev

    def test_beta_download_caches_beta_branch_manifest_and_launch_plan(self) -> None:
        branch_log = self.root / "branch_dd.log"
        branch_dd = self.root / "branch-dd"
        branch_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import json, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-branch")
                    sys.exit(0)
                with pathlib.Path({str(branch_log)!r}).open("a", encoding="utf-8") as fh:
                    fh.write(json.dumps(argv) + "\\n")
                branch = argv[argv.index("-branch") + 1] if "-branch" in argv else "public"
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                if "-manifest-only" in argv:
                    if branch == "beta":
                        if "-branchpassword" not in argv or argv[argv.index("-branchpassword") + 1] != "SECRET_BETA":
                            print("Invalid branch password")
                            sys.exit(1)
                        exe_rel = "beta\\\\Game.exe"
                        gid = "222222222"
                    else:
                        exe_rel = "public\\\\Game.exe"
                        gid = "111111111"
                    (dest / f"manifest_1091501_{{gid}}.txt").write_text(
                        f"Content Manifest for Depot 1091501\\n"
                        f"Manifest ID / date : {{gid}} / 11/26/2026\\n"
                        f"Size Chunks File SHA Flags Name\\n"
                        f"10 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0 {{exe_rel}}\\n",
                        encoding="utf-8",
                    )
                    print(f"Using app branch: '{{branch}}'.")
                    print("Processing depot 1091501 - Branch Content")
                    print(f"Manifest {{gid}} (11/26/2026 12:00:00)")
                    sys.exit(0)
                sub = "beta" if branch == "beta" else "public"
                (dest / sub).mkdir(parents=True, exist_ok=True)
                (dest / sub / "Game.exe").write_bytes(b"MZ\\x90\\x00BRANCH")
                print(f"Using app branch: '{{branch}}'.")
                print("Processing depot 1091501 - Branch Content")
                print("Manifest 222222222 (11/26/2026 12:00:00)")
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        branch_dd.chmod(branch_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=branch_dd)
        dest = self.root / "games" / "BetaGame"
        dl = backend.download(
            DownloadOptions(
                app_id="1091500",
                dest=dest,
                platform="windows",
                branch="beta",
                branch_password="SECRET_BETA",
            )
        )
        self.assertEqual(dl.branch, "beta")

        offline_info = backend.info("1091500", platform="windows", offline=True)
        self.assertEqual(offline_info.executable, "beta/Game.exe")
        plan = backend.launch_args("1091500", offline=True)
        self.assertEqual(plan.executable, "beta/Game.exe")
        self.assertEqual(plan.working_dir, "beta")

        # Verify manifest-only pre-fetch used the requested beta branch & password
        logged = [json.loads(ln) for ln in branch_log.read_text(encoding="utf-8").splitlines() if ln.strip()]
        manifest_calls = [c for c in logged if "-manifest-only" in c]
        self.assertEqual(len(manifest_calls), 1)
        self.assertIn("-branch", manifest_calls[0])
        self.assertEqual(manifest_calls[0][manifest_calls[0].index("-branch") + 1], "beta")
        self.assertIn("-branchpassword", manifest_calls[0])

    def test_discovers_installed_games_in_additional_libraryfolders_vdf(self) -> None:
        ext_lib = self.root / "ExternalSSD" / "SteamLibrary"
        ext_sa = ext_lib / "steamapps"
        ext_game_dir = ext_sa / "common" / "ExternalRPG"
        ext_game_dir.mkdir(parents=True, exist_ok=True)
        (ext_game_dir / "ExternalRPG.exe").write_bytes(b"MZ\x90\x00EXT")
        (ext_sa / "appmanifest_444444.acf").write_text(
            textwrap.dedent(
                """\
                "AppState"
                {
                    "appid"        "444444"
                    "name"         "External RPG"
                    "installdir"   "ExternalRPG"
                    "buildid"      "555001"
                    "SizeOnDisk"   "123456"
                    "LastOwner"    "76561198000000002"
                }
                """
            ),
            encoding="utf-8",
        )

        primary_sa = self.steam_home / "steamapps"
        primary_sa.mkdir(parents=True, exist_ok=True)
        (primary_sa / "libraryfolders.vdf").write_text(
            textwrap.dedent(
                f"""\
                "libraryfolders"
                {{
                    "0"
                    {{
                        "path"    "{self.steam_home}"
                    }}
                    "1"
                    {{
                        "path"    "{ext_lib}"
                    }}
                }}
                """
            ),
            encoding="utf-8",
        )

        backend = self._make_backend()
        owned = backend.list_owned()
        ext_owned = [g for g in owned if g.app_id == "444444"]
        self.assertEqual(len(ext_owned), 1)
        self.assertTrue(ext_owned[0].installed)
        self.assertEqual(ext_owned[0].install_path, str(ext_game_dir))

        info = backend.info("444444", platform="windows", offline=True)
        self.assertTrue(info.installed)
        self.assertEqual(info.install_path, str(ext_game_dir))
        self.assertEqual(info.executable, "ExternalRPG.exe")

        plan = backend.launch_args("444444", offline=True)
        self.assertEqual(plan.install_dir, str(ext_game_dir))
        self.assertEqual(plan.executable, "ExternalRPG.exe")

    def test_update_check_only_preserves_installed_launch_metadata(self) -> None:
        backend = self._make_backend()
        backend.login(username="testplayer")
        dest = self.root / "games" / "CheckOnlyPreserveGame"
        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))

        # Simulate a newer remote release renaming SampleGame.exe -> NewGame.exe
        renamed_manifest = SAMPLE_UPDATED_MANIFEST_TXT.replace("SampleGame.exe", "NewGame.exe")
        self.manifest_file.write_text(renamed_manifest, encoding="utf-8")

        upd = backend.update("1091500", check_only=True)
        self.assertTrue(upd.update_available)
        self.assertFalse(upd.updated)

        # Installed launch plan must still select the actually installed SampleGame.exe
        plan = backend.launch_args("1091500", offline=True)
        self.assertEqual(plan.executable, "bin/x64/SampleGame.exe")
        self.assertTrue((dest / plan.executable).is_file())

    def test_update_with_dlcs_installs_and_updates_dlc_when_base_unchanged(self) -> None:
        # Map DLC 2138330 -> depot 999001 under base AppID 1091500 (deliberately non-adjacent ID)
        _write_mock_appinfo_vdf(
            self.steam_home,
            1091500,
            {"1091501": None, "999001": "2138330"},
        )
        dlc_log = self.root / "dlc_update_dd.log"
        dlc_dd = self.root / "dlc-update-dd"
        dlc_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import json, os, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-dlc")
                    sys.exit(0)
                with pathlib.Path({str(dlc_log)!r}).open("a", encoding="utf-8") as fh:
                    fh.write(json.dumps(argv) + "\\n")
                app_id = argv[argv.index("-app") + 1]
                depot_args = []
                if "-depot" in argv:
                    idx = argv.index("-depot") + 1
                    while idx < len(argv) and not argv[idx].startswith("-"):
                        depot_args.append(argv[idx])
                        idx += 1
                # Match real DepotDownloader: DLC AppIDs have no standalone depots
                if app_id != "1091500":
                    print(f"Error: Couldn't find any depots to download for app {{app_id}}")
                    sys.exit(1)
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                base_gid = os.environ.get("MOCK_BASE_GID", "8811223344556677889")
                dlc_gid = os.environ.get("MOCK_DLC_GID", "770011")
                selected = depot_args or ["1091501", "999001"]
                if "-manifest-only" in argv:
                    print("Using app branch: 'public'.")
                    if "1091501" in selected:
                        (dest / f"manifest_1091501_{{base_gid}}.txt").write_text(
                            f"Content Manifest for Depot 1091501\\n"
                            f"Manifest ID / date : {{base_gid}} / 11/26/2026\\n"
                            f"Size Chunks File SHA Flags Name\\n"
                            f"10 1 bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb 0 bin\\\\x64\\\\SampleGame.exe\\n",
                            encoding="utf-8",
                        )
                        print("Processing depot 1091501 - Base Game")
                        print(f"Manifest {{base_gid}} (11/26/2026 12:00:00)")
                    if "999001" in selected:
                        (dest / f"manifest_999001_{{dlc_gid}}.txt").write_text(
                            f"Content Manifest for Depot 999001\\n"
                            f"Manifest ID / date : {{dlc_gid}} / 11/26/2026\\n"
                            f"Size Chunks File SHA Flags Name\\n"
                            f"8 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0 dlc_2138330.pak\\n",
                            encoding="utf-8",
                        )
                        print("Processing depot 999001 - Expansion DLC")
                        print(f"Manifest {{dlc_gid}} (11/26/2026 12:00:00)")
                    sys.exit(0)
                print("Using app branch: 'public'.")
                if "1091501" in selected:
                    (dest / "bin" / "x64").mkdir(parents=True, exist_ok=True)
                    (dest / "bin" / "x64" / "SampleGame.exe").write_bytes(b"MZ\\x90\\x00BASE")
                    print("Processing depot 1091501 - Base Game")
                    print(f"Manifest {{base_gid}} (11/26/2026 12:00:00)")
                if "999001" in selected:
                    (dest / "dlc_2138330.pak").write_bytes(f"DLC_{{dlc_gid}}".encode("utf-8"))
                    print("Processing depot 999001 - Expansion DLC")
                    print(f"Manifest {{dlc_gid}} (11/26/2026 12:00:00)")
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        dlc_dd.chmod(dlc_dd.stat().st_mode | stat.S_IXUSR)

        def fake_http(url: str):
            if "appdetails" in url:
                return {
                    "1091500": {
                        "success": True,
                        "data": {
                            "name": "Sample Action RPG",
                            "dlc": [2138330],
                        },
                    }
                }
            return {}

        prev_base_gid = os.environ.get("MOCK_BASE_GID")
        prev_dlc_gid = os.environ.get("MOCK_DLC_GID")
        os.environ["MOCK_BASE_GID"] = "8811223344556677889"
        os.environ["MOCK_DLC_GID"] = "770011"
        try:
            backend = self._make_backend(
                depot_downloader_bin=dlc_dd,
                allow_network=True,
                http_get_json=fake_http,
            )
            dest = self.root / "games" / "DlcUpdateGame"
            # Initial install WITHOUT DLCs -> downloads only base depot 1091501
            backend.download(
                DownloadOptions(app_id="1091500", dest=dest, platform="windows", with_dlcs=False)
            )
            self.assertFalse((dest / "dlc_2138330.pak").exists())

            # Base manifest is unchanged, but owned DLC 2138330 is not yet installed -> update_available=True
            chk1 = backend.update("1091500", check_only=True, with_dlcs=True)
            self.assertTrue(chk1.update_available)
            self.assertFalse(chk1.updated)

            # Running update --with-dlcs installs the missing owned DLC under base AppID 1091500
            upd1 = backend.update("1091500", check_only=False, with_dlcs=True)
            self.assertTrue(upd1.updated)
            self.assertEqual((dest / "dlc_2138330.pak").read_bytes(), b"DLC_770011")

            # Now both base and DLC are current -> up to date
            upd_same = backend.update("1091500", check_only=False, with_dlcs=True)
            self.assertFalse(upd_same.updated)
            self.assertFalse(upd_same.update_available)

            # Bump DLC depot GID while base stays unchanged -> update --with-dlcs detects & applies it
            os.environ["MOCK_DLC_GID"] = "770022"
            chk2 = backend.update("1091500", check_only=True, with_dlcs=True)
            self.assertTrue(chk2.update_available)

            upd2 = backend.update("1091500", check_only=False, with_dlcs=True)
            self.assertTrue(upd2.updated)
            self.assertEqual((dest / "dlc_2138330.pak").read_bytes(), b"DLC_770022")

            # Base-only update must preserve existing installed DLC IDs, depots, and versions
            os.environ["MOCK_BASE_GID"] = "9911223344556677999"
            upd_base = backend.update("1091500", check_only=False, with_dlcs=False)
            self.assertTrue(upd_base.updated)
            info_after = backend.info("1091500", platform="windows", offline=True)
            dlc_after = {d.app_id: d for d in info_after.dlcs}
            self.assertTrue(dlc_after["2138330"].installed)
            self.assertIn("999001", backend._load_installs()["1091500"]["depots"])
            chk_after = backend.update("1091500", check_only=True, with_dlcs=True)
            self.assertFalse(chk_after.update_available)

            # Deleting the DLC file after a base-only update and running verify(repair=True)
            # must restore the DLC file because depot 999001 was preserved in `depots`.
            (dest / "dlc_2138330.pak").unlink()
            rep = backend.verify("1091500", repair=True)
            self.assertTrue(rep.ok)
            self.assertEqual((dest / "dlc_2138330.pak").read_bytes(), b"DLC_770022")

            # Explicit --dlc download selects base depot + DLC depot via -app 1091500 -depot <id>
            explicit_dest = self.root / "games" / "ExplicitDlcGame"
            dl_explicit = backend.download(
                DownloadOptions(
                    app_id="1091500",
                    dest=explicit_dest,
                    platform="windows",
                    dlcs=("2138330",),
                    skip_dlcs=False,
                )
            )
            self.assertEqual(dl_explicit.downloaded_dlcs, ["2138330"])
            self.assertEqual((explicit_dest / "dlc_2138330.pak").read_bytes(), b"DLC_770022")

            # Verify every single DepotDownloader call used base AppID 1091500, never DLC AppID 2138330,
            # and explicit --dlc passed both base and DLC depots in one single `-depot 1091501 999001` call
            calls = [json.loads(ln) for ln in dlc_log.read_text(encoding="utf-8").splitlines() if ln.strip()]
            self.assertTrue(calls)
            self.assertTrue(all(c[c.index("-app") + 1] == "1091500" for c in calls if "-app" in c))
            self.assertTrue(
                any(
                    "-depot" in c
                    and c[c.index("-depot") + 1 : c.index("-depot") + 3] == ["1091501", "999001"]
                    for c in calls
                )
            )
        finally:
            if prev_base_gid is None:
                os.environ.pop("MOCK_BASE_GID", None)
            else:
                os.environ["MOCK_BASE_GID"] = prev_base_gid
            if prev_dlc_gid is None:
                os.environ.pop("MOCK_DLC_GID", None)
            else:
                os.environ["MOCK_DLC_GID"] = prev_dlc_gid

    def test_multi_base_depots_downloaded_in_single_invocation(self) -> None:
        _write_mock_appinfo_vdf(
            self.steam_home,
            1091500,
            {"1091501": None, "1091502": None, "999001": "2138330"},
        )
        multi_log = self.root / "multi_base_dd.log"
        multi_dd = self.root / "multi-base-dd"
        multi_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import json, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-multi-base")
                    sys.exit(0)
                with pathlib.Path({str(multi_log)!r}).open("a", encoding="utf-8") as fh:
                    fh.write(json.dumps(argv) + "\\n")
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                depot_args = []
                if "-depot" in argv:
                    idx = argv.index("-depot") + 1
                    while idx < len(argv) and not argv[idx].startswith("-"):
                        depot_args.append(argv[idx])
                        idx += 1
                selected = depot_args or ["1091501", "1091502", "999001"]
                if "-manifest-only" in argv:
                    print("Using app branch: 'public'.")
                    for did in selected:
                        (dest / f"manifest_{{did}}_100.txt").write_text(
                            f"Content Manifest for Depot {{did}}\\n"
                            f"Manifest ID / date : 100 / 11/26/2026\\n"
                            f"Size Chunks File SHA Flags Name\\n"
                            f"10 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0 f_{{did}}.bin\\n",
                            encoding="utf-8",
                        )
                        print(f"Processing depot {{did}} - Depot {{did}}")
                        print("Manifest 100 (11/26/2026 12:00:00)")
                    sys.exit(0)
                print("Using app branch: 'public'.")
                for did in selected:
                    (dest / f"f_{{did}}.bin").write_bytes(b"OK")
                    print(f"Processing depot {{did}} - Depot {{did}}")
                    print("Manifest 100 (11/26/2026 12:00:00)")
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        multi_dd.chmod(multi_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=multi_dd)
        dest = self.root / "games" / "MultiBaseGame"
        dl = backend.download(
            DownloadOptions(app_id="1091500", dest=dest, platform="windows", with_dlcs=False)
        )
        self.assertEqual(dl.downloaded_depots, ["1091501", "1091502"])

        logged = [json.loads(ln) for ln in multi_log.read_text(encoding="utf-8").splitlines() if ln.strip()]
        dl_calls = [c for c in logged if "-manifest-only" not in c]
        self.assertEqual(len(dl_calls), 1)
        dep_idx = dl_calls[0].index("-depot") + 1
        self.assertEqual(dl_calls[0][dep_idx : dep_idx + 2], ["1091501", "1091502"])

    def test_download_rejects_skipped_or_unprocessed_depots(self) -> None:
        skip_dd = self.root / "skip-depot-dd"
        skip_dd.write_text(
            textwrap.dedent(
                """\
                #!/usr/bin/env python3
                import os, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-skip")
                    sys.exit(0)
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                if "-manifest-only" in argv:
                    (dest / "manifest_1091501_100.txt").write_text(
                        "Content Manifest for Depot 1091501\\n"
                        "Manifest ID / date : 100 / 11/26/2026\\n"
                        "Size Chunks File SHA Flags Name\\n"
                        "10 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0 Game.exe\\n",
                        encoding="utf-8",
                    )
                    print("Using app branch: 'public'.")
                    print("Processing depot 1091501 - Base Game")
                    print("Manifest 100 (11/26/2026 12:00:00)")
                    sys.exit(0)
                if os.environ.get("MOCK_SKIP_MODE") == "entitlement":
                    print("Using app branch: 'public'.")
                    print("Depot 1091501 is not available from this account.")
                    sys.exit(0)
                print("Using app branch: 'public'.")
                print("Total downloaded: 0 bytes")
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        skip_dd.chmod(skip_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=skip_dd)
        dest = self.root / "games" / "SkippedDepotGame"

        prev_mode = os.environ.get("MOCK_SKIP_MODE")
        try:
            os.environ["MOCK_SKIP_MODE"] = "entitlement"
            with self.assertRaises(AuthRequiredError):
                backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))
            self.assertFalse((dest / "steam_appid.txt").exists())
            self.assertNotIn("1091500", backend._load_installs())

            os.environ["MOCK_SKIP_MODE"] = "empty"
            with self.assertRaises(CommandFailedError):
                backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))
            self.assertFalse((dest / "steam_appid.txt").exists())
            self.assertNotIn("1091500", backend._load_installs())
        finally:
            if prev_mode is None:
                os.environ.pop("MOCK_SKIP_MODE", None)
            else:
                os.environ["MOCK_SKIP_MODE"] = prev_mode

    def test_private_branch_maintenance_requires_and_forwards_branch_password(self) -> None:
        beta_bytes = b"MZ\x90\x00BETA_V1"
        beta_sha1 = hashlib.sha1(beta_bytes).hexdigest()
        priv_dd = self.root / "private-branch-dd"
        priv_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import os, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-priv")
                    sys.exit(0)
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                branch = argv[argv.index("-branch") + 1] if "-branch" in argv else "public"
                pw = argv[argv.index("-branchpassword") + 1] if "-branchpassword" in argv else None
                # Mirror DepotDownloader 3.4.0: missing/wrong private branch password falls back to 'public'
                effective_branch = branch if (branch == "public" or pw == "SECRET_BETA_42") else "public"
                gid = os.environ.get("MOCK_PRIV_GID", "5001") if effective_branch == "beta" else "1001"
                if "-manifest-only" in argv:
                    (dest / f"manifest_1091501_{{gid}}.txt").write_text(
                        f"Content Manifest for Depot 1091501\\n"
                        f"Manifest ID / date : {{gid}} / 11/26/2026\\n"
                        f"Size Chunks File SHA Flags Name\\n"
                        f"{len(beta_bytes)} 1 {beta_sha1} 0 beta\\\\Game.exe\\n",
                        encoding="utf-8",
                    )
                    print(f"Using app branch: '{{effective_branch}}'.")
                    print("Processing depot 1091501 - Private Beta Content")
                    print(f"Manifest {{gid}} (11/26/2026 12:00:00)")
                    sys.exit(0)
                (dest / "beta").mkdir(parents=True, exist_ok=True)
                (dest / "beta" / "Game.exe").write_bytes({beta_bytes!r})
                print(f"Using app branch: '{{effective_branch}}'.")
                print("Processing depot 1091501 - Private Beta Content")
                print(f"Manifest {{gid}} (11/26/2026 12:00:00)")
                print("Total downloaded: 128 bytes")
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        priv_dd.chmod(priv_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=priv_dd)
        dest = self.root / "games" / "PrivateBetaGame"
        backend.download(
            DownloadOptions(
                app_id="1091500",
                dest=dest,
                platform="windows",
                branch="beta",
                branch_password="SECRET_BETA_42",
            )
        )

        # Password must NEVER be persisted in steam-installs.json, only the boolean flag
        installs_raw = (self.state_dir / "steam-installs.json").read_text(encoding="utf-8")
        self.assertNotIn("SECRET_BETA_42", installs_raw)
        self.assertTrue(backend._load_installs()["1091500"]["branch_requires_password"])

        # Omitting branch_password on verify/update raises missing_branch_password
        with self.assertRaises(AcquireError) as ctx_ver:
            backend.verify("1091500", repair=False)
        self.assertEqual(ctx_ver.exception.code, "missing_branch_password")

        with self.assertRaises(AcquireError) as ctx_upd:
            backend.update("1091500", check_only=False)
        self.assertEqual(ctx_upd.exception.code, "missing_branch_password")

        # Supplying the wrong password causes DepotDownloader fallback to 'public', which is rejected
        with self.assertRaises(CommandFailedError):
            backend.update("1091500", check_only=True, branch_password="WRONG_PW")

        # Supplying the right password succeeds for read-only verify, repair, check-only, and update
        ver_ro = backend.verify("1091500", repair=False, branch_password="SECRET_BETA_42")
        self.assertTrue(ver_ro.ok)
        ver_rep = backend.verify("1091500", repair=True, branch_password="SECRET_BETA_42")
        self.assertTrue(ver_rep.ok)

        prev_gid = os.environ.get("MOCK_PRIV_GID")
        os.environ["MOCK_PRIV_GID"] = "5002"
        try:
            chk = backend.update("1091500", check_only=True, branch_password="SECRET_BETA_42")
            self.assertTrue(chk.update_available)
            self.assertEqual(chk.new_version, "5002")
            upd = backend.update("1091500", check_only=False, branch_password="SECRET_BETA_42")
            self.assertTrue(upd.updated)
            self.assertEqual(upd.new_version, "5002")
        finally:
            if prev_gid is None:
                os.environ.pop("MOCK_PRIV_GID", None)
            else:
                os.environ["MOCK_PRIV_GID"] = prev_gid

    def test_verify_readonly_rejects_missing_installed_depot_coverage(self) -> None:
        f1 = b"DEPOT_101_FILE"
        f1_sha1 = hashlib.sha1(f1).hexdigest()
        partial_cov_dd = self.root / "partial-cov-dd"
        partial_cov_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import os, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-cov")
                    sys.exit(0)
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                if "-manifest-only" in argv:
                    (dest / "manifest_101_1000.txt").write_text(
                        "Content Manifest for Depot 101\\n"
                        "Manifest ID / date : 1000 / 11/26/2026\\n"
                        "Size Chunks File SHA Flags Name\\n"
                        "{len(f1)} 1 {f1_sha1} 0 bin\\\\a.bin\\n",
                        encoding="utf-8",
                    )
                    print("Using app branch: 'public'.")
                    print("Processing depot 101 - Depot 101")
                    print("Manifest 1000 (11/26/2026 12:00:00)")
                    if os.environ.get("MOCK_COV_MODE") == "entitlement":
                        print("Depot 102 is not available from this account.")
                    sys.exit(0)
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        partial_cov_dd.chmod(partial_cov_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=partial_cov_dd)
        dest = self.root / "games" / "PartialCovGame"
        (dest / "bin").mkdir(parents=True, exist_ok=True)
        (dest / "bin" / "a.bin").write_bytes(f1)
        backend._save_installs(
            {
                "1091500": {
                    "app_id": "1091500",
                    "title": "Partial Coverage Game",
                    "account_name": "testplayer",
                    "install_path": str(dest),
                    "platform": "windows",
                    "branch": "public",
                    "depots": ["101", "102"],
                }
            }
        )

        prev_cov = os.environ.get("MOCK_COV_MODE")
        try:
            os.environ["MOCK_COV_MODE"] = "entitlement"
            with self.assertRaises(AuthRequiredError):
                backend.verify("1091500", repair=False)

            os.environ["MOCK_COV_MODE"] = "silent_omit"
            with self.assertRaises(CommandFailedError):
                backend.verify("1091500", repair=False)
        finally:
            if prev_cov is None:
                os.environ.pop("MOCK_COV_MODE", None)
            else:
                os.environ["MOCK_COV_MODE"] = prev_cov

    def test_update_compares_mounted_depot_manifests_for_acf_installs(self) -> None:
        sa_dir = self.steam_home / "steamapps"
        game_dir = sa_dir / "common" / "AcfGame"
        game_dir.mkdir(parents=True, exist_ok=True)
        (game_dir / "SampleGame.exe").write_bytes(b"MZ\x90\x00ACF")
        # Steam ACF has buildid "555001" (app buildid) and MountedDepots matching remote manifest GID
        (sa_dir / "appmanifest_1091500.acf").write_text(
            textwrap.dedent(
                """\
                "AppState"
                {
                    "appid"        "1091500"
                    "name"         "Sample Action RPG"
                    "installdir"   "AcfGame"
                    "buildid"      "555001"
                    "SizeOnDisk"   "123456"
                    "LastOwner"    "76561198000000002"
                    "MountedDepots"
                    {
                        "1091501"  "8811223344556677889"
                    }
                }
                """
            ),
            encoding="utf-8",
        )

        backend = self._make_backend()
        # MountedDepots matches remote manifest GID -> must report already up to date
        chk_same = backend.update("1091500", check_only=True)
        self.assertFalse(chk_same.update_available)
        self.assertEqual(chk_same.old_version, "8811223344556677889")
        self.assertEqual(chk_same.new_version, "8811223344556677889")

        # Once remote manifest GID changes -> must report update available
        self.manifest_file.write_text(SAMPLE_UPDATED_MANIFEST_TXT, encoding="utf-8")
        chk_new = backend.update("1091500", check_only=True)
        self.assertTrue(chk_new.update_available)
        self.assertEqual(chk_new.old_version, "8811223344556677889")
        self.assertEqual(chk_new.new_version, "9922334455667788990")

    def test_per_depot_branch_fallback_warning_aborts_before_download(self) -> None:
        fb_log = self.root / "fallback_dd.log"
        fb_dd = self.root / "fallback-dd"
        fb_dd.write_text(
            textwrap.dedent(
                f"""\
                #!/usr/bin/env python3
                import json, pathlib, sys
                argv = sys.argv[1:]
                if argv == ["--version"]:
                    print("DepotDownloader v3.4.0+mock-fallback")
                    sys.exit(0)
                with pathlib.Path({str(fb_log)!r}).open("a", encoding="utf-8") as fh:
                    fh.write(json.dumps(argv) + "\\n")
                dest = pathlib.Path(argv[argv.index("-dir") + 1])
                dest.mkdir(parents=True, exist_ok=True)
                # Mirror DepotDownloader 3.4.0 ContentDownloader.cs#L519-L526:
                # header still prints requested branch, then per-depot warning falls back to public.
                print("Using app branch: 'beta'.")
                print('Warning: Depot 1091501 does not have branch named "beta". Trying public branch.')
                print("Processing depot 1091501 - Public Content")
                print("Manifest 111111 (11/26/2026 12:00:00)")
                if "-manifest-only" in argv:
                    (dest / "manifest_1091501_111111.txt").write_text(
                        "Content Manifest for Depot 1091501\\n"
                        "Manifest ID / date : 111111 / 11/26/2026\\n"
                        "Size Chunks File SHA Flags Name\\n"
                        "10 1 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0 public\\\\Game.exe\\n",
                        encoding="utf-8",
                    )
                sys.exit(0)
                """
            ),
            encoding="utf-8",
        )
        fb_dd.chmod(fb_dd.stat().st_mode | stat.S_IXUSR)

        backend = self._make_backend(depot_downloader_bin=fb_dd)
        dest = self.root / "games" / "FallbackBetaGame"

        with self.assertRaises(CommandFailedError) as ctx:
            backend.download(
                DownloadOptions(
                    app_id="1091500",
                    dest=dest,
                    platform="windows",
                    branch="beta",
                    branch_password="BAD_PASSWORD",
                )
            )
        self.assertEqual(ctx.exception.code, "branch_fallback")
        # Must abort during preflight -manifest-only and NEVER start a download transfer
        calls = [json.loads(ln) for ln in fb_log.read_text(encoding="utf-8").splitlines() if ln.strip()]
        self.assertEqual(len(calls), 1)
        self.assertIn("-manifest-only", calls[0])
        self.assertNotIn("1091500", backend._load_installs())

    def test_repair_synchronizes_installed_version_and_manifest_cache(self) -> None:
        backend = self._make_backend()
        backend.login(username="testplayer")
        dest = self.root / "games" / "RepairSyncGame"
        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))
        self.assertEqual(backend._load_installs()["1091500"]["build_id"], "8811223344556677889")

        # Remote branch advances to manifest 9922334455667788990; repair pulls the new manifest
        self.manifest_file.write_text(SAMPLE_UPDATED_MANIFEST_TXT, encoding="utf-8")
        prev_gid = os.environ.get("MOCK_MANIFEST_GID")
        os.environ["MOCK_MANIFEST_GID"] = "9922334455667788990"
        try:
            ver = backend.verify("1091500", repair=True)
            self.assertTrue(ver.ok)
            self.assertTrue(ver.repaired)
            self.assertEqual(backend._load_installs()["1091500"]["build_id"], "9922334455667788990")
            chk = backend.update("1091500", check_only=True)
            self.assertFalse(chk.update_available)
            self.assertEqual(chk.old_version, "9922334455667788990")
        finally:
            if prev_gid is None:
                os.environ.pop("MOCK_MANIFEST_GID", None)
            else:
                os.environ["MOCK_MANIFEST_GID"] = prev_gid

    def test_cli_repeated_download_without_dest_reuses_registered_install_path(self) -> None:
        default_base = self.root / "default_acquire_dest"
        env_updates = {
            "PORT_DEPOT_DOWNLOADER": str(self.mock_dd),
            "PORT_STEAM_HOME": str(self.steam_home),
            "PORT_STEAM_USER": "testplayer",
            "PORT_ACQUIRE_DEST": str(default_base),
        }
        old_env = {k: os.environ.get(k) for k in env_updates}
        os.environ.update(env_updates)
        try:
            def run_cli(argv: list[str]) -> tuple[int, dict]:
                buf = io.StringIO()
                with contextlib.redirect_stdout(buf):
                    rc = cli_main(argv)
                return rc, json.loads(buf.getvalue())

            rc1, dl1 = run_cli(["--store", "steam", "--json", "download", "1091500"])
            self.assertEqual(rc1, 0)
            first_path = Path(dl1["install_path"])

            # Simulate Store title now cached under store_details (real title differs from placeholder)
            store_cache = self.steam_home / ".port-acquire" / "store_details" / "1091500.json"
            store_cache.parent.mkdir(parents=True, exist_ok=True)
            store_cache.write_text(json.dumps({"name": "Sample Action RPG"}), encoding="utf-8")

            rc2, dl2 = run_cli(["--store", "steam", "--json", "download", "1091500"])
            self.assertEqual(rc2, 0)
            self.assertEqual(Path(dl2["install_path"]), first_path)
            self.assertFalse((default_base / "Sample Action RPG").exists())

            # Unmanaged native macOS Steam-client install must NOT be targeted by a Windows download
            sa_dir = self.steam_home / "steamapps"
            native_dir = sa_dir / "common" / "NativeMacGame"
            native_dir.mkdir(parents=True, exist_ok=True)
            (native_dir / "NativeApp.txt").write_text("MACOS_NATIVE", encoding="utf-8")
            (sa_dir / "appmanifest_555000.acf").write_text(
                textwrap.dedent(
                    """\
                    "AppState"
                    {
                        "appid"        "555000"
                        "name"         "Native Mac Game"
                        "installdir"   "NativeMacGame"
                        "buildid"      "100"
                        "LastOwner"    "76561198000000002"
                    }
                    """
                ),
                encoding="utf-8",
            )
            rc3, dl3 = run_cli(
                ["--store", "steam", "--json", "download", "555000", "--platform", "windows"]
            )
            self.assertEqual(rc3, 0)
            win_path = Path(dl3["install_path"])
            self.assertNotEqual(win_path.resolve(), native_dir.resolve())
            self.assertTrue(str(win_path).startswith(str(default_base.resolve())))
            self.assertEqual((native_dir / "NativeApp.txt").read_text(encoding="utf-8"), "MACOS_NATIVE")
        finally:
            for k, v in old_env.items():
                if v is None:
                    os.environ.pop(k, None)
                else:
                    os.environ[k] = v

    def test_offline_info_and_manifest_retain_cached_store_metadata(self) -> None:
        def fake_http(url: str):
            if "appdetails" in url:
                return {
                    "1091500": {
                        "success": True,
                        "data": {
                            "name": "Sample Action RPG",
                            "dlc": [2138330, 2138331],
                            "drm_notice": "Incorporates 3rd-party DRM: Denuvo Anti-Tamper",
                            "categories": [{"description": "Steam Achievements"}],
                        },
                    }
                }
            return {}

        backend = self._make_backend(allow_network=True, http_get_json=fake_http)
        backend.login(username="testplayer")
        dest = self.root / "games" / "CachedStoreMetaGame"
        backend.download(DownloadOptions(app_id="1091500", dest=dest, platform="windows"))

        # Offline info (the exact path used by CLI when generating port-acquire-manifest.json)
        # must preserve cached Store title, DLCs, and DRM signals.
        offline_info = backend.info("1091500", platform="windows", offline=True)
        self.assertEqual(offline_info.title, "Sample Action RPG")
        self.assertEqual([d.app_id for d in offline_info.dlcs], ["2138330", "2138331"])
        self.assertTrue(offline_info.drm.detected)
        self.assertTrue(offline_info.drm.requires_online_auth)
        self.assertTrue(any("Denuvo" in s for s in offline_info.drm.signals))

    def test_local_ownership_and_cache_scoped_to_active_account(self) -> None:
        # Configure two Steam accounts on the same machine:
        #   testplayer -> 76561198000000002 (account_id32 = 39734274) owns 1091500, 2138330, 367520
        #   otheruser  -> 76561198000000001 (account_id32 = 39734273) owns 555555, 2138331
        (self.steam_home / "config" / "loginusers.vdf").write_text(
            textwrap.dedent(
                """\
                "users"
                {
                    "76561198000000001"
                    {
                        "AccountName"       "otheruser"
                        "PersonaName"       "Other User"
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
        other_cfg = self.steam_home / "userdata" / "39734273" / "config"
        other_cfg.mkdir(parents=True, exist_ok=True)
        (other_cfg / "localconfig.vdf").write_text(
            '"UserLocalConfigStore" { "Software" { "Valve" { "Steam" { "apps" { "555555" { "Playtime" "10" } "2138331" { "Playtime" "5" } } } } } }\n',
            encoding="utf-8",
        )
        # Shared steamapps ACF owned by otheruser
        sa_dir = self.steam_home / "steamapps"
        sa_dir.mkdir(parents=True, exist_ok=True)
        (sa_dir / "appmanifest_777777.acf").write_text(
            '"AppState" { "appid" "777777" "name" "Other User Installed Game" "installdir" "OtherGame" "LastOwner" "76561198000000001" }\n',
            encoding="utf-8",
        )

        # Direct helper test: multi-user directory without steam_id64 must not merge accounts
        self.assertEqual(parse_local_owned_app_ids(self.steam_home), set())
        self.assertEqual(
            parse_local_owned_app_ids(self.steam_home, steam_id64="76561198000000002"),
            {"367520", "1091500", "2138330"},
        )
        self.assertEqual(
            parse_local_owned_app_ids(self.steam_home, steam_id64="76561198000000001"),
            {"555555", "2138331"},
        )

        def fake_http(url: str):
            if "appdetails" in url:
                return {
                    "1091500": {
                        "success": True,
                        "data": {
                            "name": "Sample Action RPG",
                            "dlc": [2138330, 2138331],
                        },
                    }
                }
            return {}

        backend = self._make_backend(allow_network=True, http_get_json=fake_http)

        # Account 1: otheruser sees only its own games, not testplayer's
        backend.login(username="otheruser")
        other_owned = {g.app_id for g in backend.list_owned()}
        self.assertEqual(other_owned, {"555555", "2138331", "777777"})

        # Account 2: testplayer sees only its own games and DLC 2138330 (NOT 2138331, 555555, or 777777)
        backend.login(username="testplayer")
        player_owned = {g.app_id for g in backend.list_owned()}
        self.assertEqual(player_owned, {"367520", "1091500", "2138330"})

        info = backend.info("1091500", platform="windows", offline=False)
        dlc_by_id = {d.app_id: d for d in info.dlcs}
        self.assertTrue(dlc_by_id["2138330"].owned)
        self.assertFalse(dlc_by_id["2138331"].owned)

        # Unlisted account name not in loginusers.vdf must not inherit other accounts' local caches
        backend.login(username="brandnewuser")
        self.assertEqual(backend.list_owned(), [])

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
