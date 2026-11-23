use super::download::parse_qr_cached_username;
use super::*;
use crate::bundle_common::TempDirGuard;
use crate::win32_db::Win32Database;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn make_temp_dir(tag: &str) -> TempDirGuard {
    let dir = std::env::temp_dir().join(format!(
        "port_factory_steam_test_{}_{}",
        tag,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp test dir");
    TempDirGuard::new(dir)
}

#[test]
fn parses_acf_and_installscript_vdf() {
    let acf = r#"
"AppState"
{
    "appid"     "2357570"
    "name"      "Overwatch®"
    "installdir" "Overwatch"
    "SizeOnDisk" "72292658910"
}
"#;
    let vdf = parse_vdf(acf);
    assert_eq!(
        vdf.get_path(&["AppState", "appid"]).and_then(|v| v.as_str()),
        Some("2357570")
    );
    assert_eq!(
        vdf.get_path(&["AppState", "installdir"])
            .and_then(|v| v.as_str()),
        Some("Overwatch")
    );

    let install_vdf = r#"
"InstallScript"
{
    "Registry"
    {
        "HKEY_LOCAL_MACHINE\\Software\\Blizzard Entertainment\\Overwatch"
        {
            "string"
            {
                "InstallPath" "%INSTALLDIR%"
            }
            "dword"
            {
                "Installed" "1"
            }
        }
    }
    "Run Process"
    {
        "VC_Redist_x64"
        {
            "process 1" "%INSTALLDIR%\\_CommonRedist\\vcredist\\2022\\VC_redist.x64.exe"
            "command 1" "/quiet /norestart"
        }
    }
}
"#;
    let summary = parse_installscript_vdf(install_vdf);
    assert_eq!(summary.registry_entries.len(), 2);
    assert_eq!(
        summary.registry_entries[0].key_path,
        "hklm\\software\\blizzard entertainment\\overwatch"
    );
    assert_eq!(summary.registry_entries[0].value_name, "InstallPath");
    assert_eq!(summary.registry_entries[1].value_type, "REG_DWORD");
    assert_eq!(summary.run_processes.len(), 1);
    assert_eq!(summary.bypassable_redistributables.len(), 1);
}

#[test]
fn stages_bundle_frameworks_from_home_vulkan_sdk_and_env_overrides() {
    let root = make_temp_dir("frameworks");
    let fake_home = root.path().join("home");
    let fake_sdk = root.path().join("vulkan_sdk");
    let fake_workspace = root.path().join("ws");

    let home_msc =
        fake_home.join(".local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib");
    let home_dxilconv = fake_home.join(".local/lib/libdxilconv.dylib");
    let sdk_moltenvk = fake_sdk.join("lib/libMoltenVK.dylib");

    fs::create_dir_all(home_msc.parent().unwrap()).unwrap();
    fs::create_dir_all(home_dxilconv.parent().unwrap()).unwrap();
    fs::create_dir_all(sdk_moltenvk.parent().unwrap()).unwrap();
    fs::write(&home_msc, b"FAKE_MSC_HOME").unwrap();
    fs::write(&home_dxilconv, b"FAKE_DXILCONV_HOME").unwrap();
    fs::write(&sdk_moltenvk, b"FAKE_MVK_SDK").unwrap();

    // 1. Auto-discovery from $HOME + $VULKAN_SDK (no explicit dylib env overrides).
    let fw_dir_1 = root.path().join("Bundle1.app/Contents/Frameworks");
    let env1 = FrameworkDiscoveryEnv {
        home: Some(fake_home.clone()),
        vulkan_sdk: Some(fake_sdk.clone()),
        include_system_paths: false,
        ..Default::default()
    };
    let staged1 = stage_bundle_frameworks_with_env(&fw_dir_1, &fake_workspace, &env1).unwrap();
    assert_eq!(
        staged1.staged_names(),
        vec![
            "libmetalirconverter.dylib",
            "libdxilconv.dylib",
            "libMoltenVK.dylib",
        ]
    );
    assert_eq!(
        fs::read(staged1.metal_irconverter.as_ref().unwrap()).unwrap(),
        b"FAKE_MSC_HOME"
    );
    assert_eq!(
        fs::read(staged1.dxilconv.as_ref().unwrap()).unwrap(),
        b"FAKE_DXILCONV_HOME"
    );
    assert_eq!(
        fs::read(staged1.moltenvk.as_ref().unwrap()).unwrap(),
        b"FAKE_MVK_SDK"
    );

    // 2. Explicit env vars (`METAL_IRCONVERTER_DYLIB`, `AW2_DXILCONV`, `PORT_VULKAN_DYLIB`)
    //    take precedence over `$HOME` and `$VULKAN_SDK`.
    let override_dir = root.path().join("overrides");
    fs::create_dir_all(&override_dir).unwrap();
    let env_msc = override_dir.join("libmetalirconverter.dylib");
    let env_dxilconv = override_dir.join("libdxilconv.dylib");
    let env_mvk = override_dir.join("libMoltenVK.dylib");
    fs::write(&env_msc, b"FAKE_MSC_ENV").unwrap();
    fs::write(&env_dxilconv, b"FAKE_DXILCONV_ENV").unwrap();
    fs::write(&env_mvk, b"FAKE_MVK_ENV").unwrap();

    let fw_dir_2 = root.path().join("Bundle2.app/Contents/Frameworks");
    let env2 = FrameworkDiscoveryEnv {
        home: Some(fake_home.clone()),
        metal_irconverter_dylib: Some(env_msc),
        aw2_dxilconv: Some(env_dxilconv),
        port_vulkan_dylib: Some(env_mvk),
        vulkan_sdk: Some(fake_sdk),
        include_system_paths: false,
        ..Default::default()
    };
    let staged2 = stage_bundle_frameworks_with_env(&fw_dir_2, &fake_workspace, &env2).unwrap();
    assert_eq!(
        fs::read(staged2.metal_irconverter.as_ref().unwrap()).unwrap(),
        b"FAKE_MSC_ENV"
    );
    assert_eq!(
        fs::read(staged2.dxilconv.as_ref().unwrap()).unwrap(),
        b"FAKE_DXILCONV_ENV"
    );
    assert_eq!(
        fs::read(staged2.moltenvk.as_ref().unwrap()).unwrap(),
        b"FAKE_MVK_ENV"
    );

    // 3. Optional missing dylibs (`libdxilconv.dylib`, `libMoltenVK.dylib`) stay None.
    let fw_dir_3 = root.path().join("Bundle3.app/Contents/Frameworks");
    let env3 = FrameworkDiscoveryEnv {
        metal_irconverter_dylib: Some(home_msc),
        include_system_paths: false,
        ..Default::default()
    };
    let staged3 = stage_bundle_frameworks_with_env(&fw_dir_3, &fake_workspace, &env3).unwrap();
    assert!(staged3.metal_irconverter.is_some());
    assert!(staged3.dxilconv.is_none());
    assert!(staged3.moltenvk.is_none());
    assert!(!fw_dir_3.join("libdxilconv.dylib").exists());
    assert!(!fw_dir_3.join("libMoltenVK.dylib").exists());
    // 4. Symlinked read-only (0444) source dylibs (like Homebrew Cellar libraries) are copied
    //    as self-contained regular 0755 files so `codesign --force --sign -` and re-staging work.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let cellar_dir = root.path().join("cellar");
        let brew_lib = root.path().join("brew_lib");
        fs::create_dir_all(&cellar_dir).unwrap();
        fs::create_dir_all(&brew_lib).unwrap();
        let ro_target = cellar_dir.join("libMoltenVK.dylib");
        fs::write(&ro_target, b"READONLY_CELLAR_MVK").unwrap();
        fs::set_permissions(&ro_target, fs::Permissions::from_mode(0o444)).unwrap();
        let brew_link = brew_lib.join("libMoltenVK.dylib");
        std::os::unix::fs::symlink(&ro_target, &brew_link).unwrap();

        let fw_dir_4 = root.path().join("Bundle4.app/Contents/Frameworks");
        let env4 = FrameworkDiscoveryEnv {
            port_vulkan_dylib: Some(brew_link),
            include_system_paths: false,
            ..Default::default()
        };
        let staged4 = stage_bundle_frameworks_with_env(&fw_dir_4, &fake_workspace, &env4).unwrap();
        let dst_mvk = staged4.moltenvk.as_ref().unwrap();
        let meta = fs::symlink_metadata(dst_mvk).unwrap();
        assert!(
            !meta.file_type().is_symlink(),
            "staged framework must be a real file copy"
        );
        assert_eq!(meta.permissions().mode() & 0o777, 0o755);
        assert_eq!(fs::read(dst_mvk).unwrap(), b"READONLY_CELLAR_MVK");

        // Re-staging over an already staged bundle must succeed cleanly too.
        let staged4_again =
            stage_bundle_frameworks_with_env(&fw_dir_4, &fake_workspace, &env4).unwrap();
        assert_eq!(staged4_again.moltenvk.as_deref(), Some(dst_mvk.as_path()));
    }
}

#[test]
fn generates_and_executes_self_contained_launcher_with_empty_env() {
    let root = make_temp_dir("launcher");
    let contents_dir = root.path().join("TestGame.app/Contents");
    let macos_dir = contents_dir.join("MacOS");
    let frameworks_dir = contents_dir.join("Frameworks");
    let resources_dir = contents_dir.join("Resources");
    let aot_cache_dir = resources_dir.join("AotCache");
    let shader_cache_dir = resources_dir.join("ShaderCache");
    fs::create_dir_all(&macos_dir).unwrap();
    fs::create_dir_all(&frameworks_dir).unwrap();
    fs::create_dir_all(resources_dir.join("Game")).unwrap();
    fs::create_dir_all(&aot_cache_dir).unwrap();
    fs::create_dir_all(shader_cache_dir.join("manifest")).unwrap();
    fs::create_dir_all(shader_cache_dir.join("pso/pso_manifest")).unwrap();

    // Place fake staged frameworks, manifest, and helper binary inside the bundle.
    let fw_msc = frameworks_dir.join("libmetalirconverter.dylib");
    let fw_dxilconv = frameworks_dir.join("libdxilconv.dylib");
    let fw_moltenvk = frameworks_dir.join("libMoltenVK.dylib");
    let bundled_manifest = resources_dir.join("game.toml");
    let bundled_shaderd = macos_dir.join("dxil-metald");
    fs::write(&fw_msc, b"MSC").unwrap();
    fs::write(&fw_dxilconv, b"DXILCONV").unwrap();
    fs::write(&fw_moltenvk, b"MVK").unwrap();
    fs::write(
        &bundled_manifest,
        generate_game_manifest_toml("TestGame", "TestGame", "Bin/TestGame.exe", "12345"),
    )
    .unwrap();
    fs::write(&bundled_shaderd, b"FAKE_SHADERD").unwrap();

    let cfg = BundleLauncherConfig {
        clean_title: "TestGame",
        app_id: "12345",
        exe_rel: "Bin/TestGame.exe",
        image_base: 0x14000_0000,
        size_of_image: 0x20_0000,
        aot_fn_count: 42,
        aot_dylib_name: "libtestgame_aot.dylib",
        extra_aot_dylibs: &[],
    };
    let src = generate_bundle_launcher_source(&cfg);
    assert!(src.contains("METAL_IRCONVERTER_DYLIB"));
    assert!(src.contains("AW2_DXILCONV"));
    assert!(src.contains("PORT_VULKAN_DYLIB"));
    assert!(src.contains("PORT_MANIFEST"));
    assert!(src.contains("PORT_AOT_DIR"));
    assert!(src.contains("PORT_SHADERD"));
    assert!(src.contains("DYLD_FALLBACK_LIBRARY_PATH"));
    assert!(src.contains("libmetalirconverter.dylib"));
    assert!(src.contains("libdxilconv.dylib"));
    assert!(src.contains("libMoltenVK.dylib"));
    assert!(!src.contains("__CLEAN_TITLE__"));
    assert!(!src.contains("__APP_ID__"));
    assert!(!src.contains("__EXE_REL__"));

    // Compile the generated launcher with rustc and run it with an EMPTY environment
    // (`env_clear()`, matching a cold launch from Finder) against a stub `port-runner`.
    let launcher_rs = root.path().join("launcher.rs");
    let launcher_bin = macos_dir.join("TestGame");
    fs::write(&launcher_rs, &src).unwrap();
    let compile = Command::new("rustc")
        .args([
            "--edition=2021",
            &launcher_rs.to_string_lossy(),
            "-o",
            &launcher_bin.to_string_lossy(),
        ])
        .output()
        .expect("invoke rustc for generated bundle launcher");
    assert!(
        compile.status.success(),
        "generated launcher failed to compile:\n{}",
        String::from_utf8_lossy(&compile.stderr)
    );

    // Compile a tiny native stub `port-runner` (instead of `/bin/sh`, which macOS SIP strips
    // `DYLD_*` env vars from) so we can verify both `PORT_*` and `DYLD_FALLBACK_LIBRARY_PATH`.
    let fake_runner_rs = root.path().join("fake_runner.rs");
    let fake_runner = macos_dir.join("port-runner");
    fs::write(
        &fake_runner_rs,
        r#"fn main() {
    for k in [
        "METAL_IRCONVERTER_DYLIB",
        "AW2_DXILCONV",
        "PORT_VULKAN_DYLIB",
        "DYLD_FALLBACK_LIBRARY_PATH",
        "PATH",
        "PORT_MANIFEST",
        "PORT_SHADERD",
        "AW2_SHADERD",
        "PORT_AOT_DIR",
        "AW2_DLL_AOT_DIR",
        "PORT_AOT_FALLBACK_DIR",
        "PORT_SHADER_CACHE",
        "PORT_SHADER_CACHE_FALLBACK",
        "PORT_SHADER_MANIFEST_FALLBACK",
        "PORT_PSO_MANIFEST_FALLBACK",
    ] {
        println!("{k}={}", std::env::var(k).unwrap_or_default());
    }
}
"#,
    )
    .unwrap();
    let runner_compile = Command::new("rustc")
        .args([
            "--edition=2021",
            &fake_runner_rs.to_string_lossy(),
            "-o",
            &fake_runner.to_string_lossy(),
        ])
        .output()
        .expect("invoke rustc for stub port-runner");
    assert!(
        runner_compile.status.success(),
        "stub port-runner failed to compile:\n{}",
        String::from_utf8_lossy(&runner_compile.stderr)
    );

    let parse_env_dump = |stdout: &str| -> HashMap<String, String> {
        stdout
            .lines()
            .filter_map(|l| l.split_once('='))
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    };

    // 1. Cold launch with ZERO env vars set -> all bundled frameworks, manifest, AotCache,
    //    dxil-metald, PATH, and DYLD_FALLBACK_LIBRARY_PATH resolved!
    let run_cold = Command::new(&launcher_bin)
        .env_clear()
        .output()
        .expect("run compiled launcher with empty env");
    assert!(
        run_cold.status.success(),
        "cold launch failed: {}",
        String::from_utf8_lossy(&run_cold.stderr)
    );
    let out_cold = String::from_utf8_lossy(&run_cold.stdout);
    let env_cold = parse_env_dump(&out_cold);
    let canon_fw = fs::canonicalize(&frameworks_dir).unwrap();
    let canon_macos = fs::canonicalize(&macos_dir).unwrap();
    let canon_res = fs::canonicalize(&resources_dir).unwrap();
    assert_eq!(
        fs::canonicalize(env_cold.get("METAL_IRCONVERTER_DYLIB").unwrap()).unwrap(),
        canon_fw.join("libmetalirconverter.dylib")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("AW2_DXILCONV").unwrap()).unwrap(),
        canon_fw.join("libdxilconv.dylib")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_VULKAN_DYLIB").unwrap()).unwrap(),
        canon_fw.join("libMoltenVK.dylib")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_MANIFEST").unwrap()).unwrap(),
        canon_res.join("game.toml")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_AOT_DIR").unwrap()).unwrap(),
        canon_res.join("AotCache")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("AW2_DLL_AOT_DIR").unwrap()).unwrap(),
        canon_res.join("AotCache")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_AOT_FALLBACK_DIR").unwrap()).unwrap(),
        canon_res.join("AotCache")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_SHADER_CACHE_FALLBACK").unwrap()).unwrap(),
        canon_res.join("ShaderCache")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_SHADER_MANIFEST_FALLBACK").unwrap()).unwrap(),
        canon_res.join("ShaderCache/manifest")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_PSO_MANIFEST_FALLBACK").unwrap()).unwrap(),
        canon_res.join("ShaderCache/pso/pso_manifest")
    );

    // 1b. With HOME set (normal Finder / new-Mac launch): writable caches route to $HOME
    //     while *_FALLBACK* still point read-only into Contents/Resources/!
    let fake_home = root.path().join("fake_home");
    fs::create_dir_all(&fake_home).unwrap();
    let run_home = Command::new(&launcher_bin)
        .env_clear()
        .env("HOME", &fake_home)
        .output()
        .expect("run compiled launcher with HOME");
    assert!(run_home.status.success());
    let env_home = parse_env_dump(&String::from_utf8_lossy(&run_home.stdout));
    assert_eq!(
        PathBuf::from(env_home.get("PORT_AOT_DIR").unwrap()),
        fake_home.join("Library/Caches/port-factory/aot")
    );
    assert_eq!(
        PathBuf::from(env_home.get("PORT_SHADER_CACHE").unwrap()),
        fake_home.join("Translator/ShaderCache")
    );
    assert_eq!(
        fs::canonicalize(env_home.get("PORT_AOT_FALLBACK_DIR").unwrap()).unwrap(),
        canon_res.join("AotCache")
    );
    assert_eq!(
        fs::canonicalize(env_home.get("PORT_SHADER_CACHE_FALLBACK").unwrap()).unwrap(),
        canon_res.join("ShaderCache")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("PORT_SHADERD").unwrap()).unwrap(),
        canon_macos.join("dxil-metald")
    );
    assert_eq!(
        fs::canonicalize(env_cold.get("AW2_SHADERD").unwrap()).unwrap(),
        canon_macos.join("dxil-metald")
    );
    let path_first = env_cold.get("PATH").unwrap().split(':').next().unwrap();
    assert_eq!(fs::canonicalize(path_first).unwrap(), canon_macos);
    let dyld_first = env_cold
        .get("DYLD_FALLBACK_LIBRARY_PATH")
        .unwrap()
        .split(':')
        .next()
        .unwrap();
    assert_eq!(fs::canonicalize(dyld_first).unwrap(), canon_fw);

    // 2. Explicit developer overrides are preserved instead of overwritten.
    let run_override = Command::new(&launcher_bin)
        .env_clear()
        .env("PORT_VULKAN_DYLIB", "/custom/override/libMoltenVK.dylib")
        .env("PORT_MANIFEST", "/custom/override/game.toml")
        .env("PORT_AOT_DIR", "/custom/override/aot")
        .output()
        .expect("run compiled launcher with explicit override");
    assert!(run_override.status.success());
    let env_override = parse_env_dump(&String::from_utf8_lossy(&run_override.stdout));
    assert_eq!(
        env_override.get("PORT_VULKAN_DYLIB").map(String::as_str),
        Some("/custom/override/libMoltenVK.dylib")
    );
    assert_eq!(
        env_override.get("PORT_MANIFEST").map(String::as_str),
        Some("/custom/override/game.toml")
    );
    assert_eq!(
        env_override.get("PORT_AOT_DIR").map(String::as_str),
        Some("/custom/override/aot")
    );

    // 3. When optional libMoltenVK.dylib is removed from Frameworks, PORT_VULKAN_DYLIB stays empty.
    fs::remove_file(&fw_moltenvk).unwrap();
    let run_no_vk = Command::new(&launcher_bin)
        .env_clear()
        .output()
        .expect("run compiled launcher without bundled MoltenVK");
    assert!(run_no_vk.status.success());
    let env_no_vk = parse_env_dump(&String::from_utf8_lossy(&run_no_vk.stdout));
    assert_eq!(env_no_vk.get("PORT_VULKAN_DYLIB").map(String::as_str), Some(""));
    assert_eq!(
        fs::canonicalize(env_no_vk.get("METAL_IRCONVERTER_DYLIB").unwrap()).unwrap(),
        canon_fw.join("libmetalirconverter.dylib")
    );

    // 4. Explicit `--aot-info` prints bundled framework, manifest, AotCache, and helper presence cleanly.
    let info = Command::new(&launcher_bin)
        .arg("--aot-info")
        .env_clear()
        .output()
        .expect("run --aot-info");
    assert!(info.status.success());
    let info_out = String::from_utf8_lossy(&info.stdout);
    assert!(info_out.contains("Bundled Manifest (TOML): true"));
    assert!(info_out.contains("Bundled AotCache Dir   : true"));
    assert!(info_out.contains("Bundled Shader Helper  : true"));
    assert!(info_out.contains("Bundled Metal IR Conv  : true"));
    assert!(info_out.contains("Bundled DXILConv (DX11): true"));
    assert!(info_out.contains("Bundled MoltenVK (VK)  : false"));
}

#[test]
fn parses_vtool_macos_versions_and_adopts_repo_manifests() {
    let vtool_out = r#"/tmp/libdxilconv.dylib (architecture x86_64):
Load command 10
      cmd LC_BUILD_VERSION
  cmdsize 32
 platform MACOS
    minos 13.0
      sdk 26.4
/tmp/libdxilconv.dylib (architecture arm64):
Load command 10
      cmd LC_BUILD_VERSION
  cmdsize 32
 platform MACOS
    minos 27.2
      sdk 27.2
"#;
    let parsed = parse_vtool_macos_versions(vtool_out);
    assert_eq!(
        parsed,
        vec![
            ("13.0".to_string(), "26.4".to_string()),
            ("27.2".to_string(), "27.2".to_string()),
        ]
    );

    let root = make_temp_dir("repo_manifest");
    let game_dir = root.path().join("games/sample");
    fs::create_dir_all(&game_dir).unwrap();
    fs::write(
        game_dir.join("game.toml"),
        r#"[app]
id = "sample"
name = "Sample"
exe = "Sample.exe"
steam_app_id = "424242"

[runtime]
builtin_overrides = "-bink2w64.dll"

[middleware.physx]
version = "4.1"
mode = "guest"

[dlls]
"nrd.dll" = "guest"

[storage]
keep_dlls = ["v8.dll"]

[ios]
seed_file = "ios.ini -> AppData/Local/Sample/ios.ini"
"#,
    )
    .unwrap();

    let adopted = find_repo_game_manifest(root.path(), "424242", "Bin/Sample.exe")
        .expect("should match repo manifest by steam_app_id");
    assert!(adopted.contains("exe = \"Bin/Sample.exe\""));
    assert!(adopted.contains("builtin_overrides = \"-bink2w64.dll\""));
    assert!(adopted.contains("aot_dir = \"AotCache\""));
    assert!(adopted.contains("[middleware.physx]"));
    assert!(adopted.contains("version = \"4.1\""));
    assert!(adopted.contains("\"nrd.dll\" = \"guest\""));
    assert!(adopted.contains("[storage]"));
    assert!(adopted.contains("\"v8.dll\""));
    assert!(adopted.contains("[ios]"));
    assert!(adopted.contains("ios.ini -> AppData/Local/Sample/ios.ini"));

    let by_exe = find_repo_game_manifest(root.path(), "custom", "Sample.exe")
        .expect("should match repo manifest by exe basename");
    assert!(by_exe.contains("builtin_overrides = \"-bink2w64.dll\""));
    assert!(find_repo_game_manifest(root.path(), "999999", "Other.exe").is_none());
}

#[test]
fn copies_only_portable_shader_cache_and_flags_nonportable_bundles() {
    let root = make_temp_dir("verify_bundle");
    let src_cache = root.path().join("host_shader_cache");
    fs::create_dir_all(src_cache.join("ir_abc123")).unwrap();
    fs::create_dir_all(src_cache.join("manifest")).unwrap();
    fs::create_dir_all(src_cache.join("pso/pso_manifest")).unwrap();
    fs::create_dir_all(src_cache.join("pso/apple_m3_ultra_26C5047e")).unwrap();
    fs::write(src_cache.join("ir_abc123/shader1.aw2s"), b"IR").unwrap();
    fs::write(src_cache.join("manifest/records.aw2m"), b"MAN").unwrap();
    fs::write(src_cache.join("pso/pso_manifest/records.p2m"), b"PSO").unwrap();
    fs::write(
        src_cache.join("pso/apple_m3_ultra_26C5047e/pso.mtl4a"),
        b"HOST_SPECIFIC",
    )
    .unwrap();

    let dst_cache = root.path().join("dst_shader_cache");
    let copied = copy_portable_shader_cache(&src_cache, &dst_cache).unwrap();
    assert_eq!(copied, 3);
    assert!(dst_cache.join("ir_abc123/shader1.aw2s").is_file());
    assert!(dst_cache.join("manifest/records.aw2m").is_file());
    assert!(dst_cache.join("pso/pso_manifest/records.p2m").is_file());
    assert!(!dst_cache.join("pso/apple_m3_ultra_26C5047e/pso.mtl4a").exists());

    // Build a broken .app bundle (symlinked Game, missing libmetalirconverter, stray .mtl4a)
    // and ensure `verify_app_bundle` catches every single portability violation.
    let external_game = root.path().join("external_game");
    fs::create_dir_all(&external_game).unwrap();
    let bad_app = root.path().join("Bad.app");
    let contents = bad_app.join("Contents");
    fs::create_dir_all(contents.join("MacOS")).unwrap();
    fs::create_dir_all(contents.join("Frameworks")).unwrap();
    fs::create_dir_all(contents.join("Resources/ShaderCache/pso")).unwrap();
    fs::write(
        contents.join("Info.plist"),
        generate_info_plist("Bad", "Bad", "1"),
    )
    .unwrap();
    std::os::unix::fs::symlink(&external_game, contents.join("Resources/Game")).unwrap();
    fs::write(
        contents.join("Resources/ShaderCache/pso/host.mtl4a"),
        b"BLOAT",
    )
    .unwrap();

    let report = verify_app_bundle(&bad_app).unwrap();
    assert!(!report.passed());
    let failed_names: BTreeSet<&str> = report
        .checks
        .iter()
        .filter(|c| c.status == BundleCheckStatus::Fail)
        .map(|c| c.name.as_str())
        .collect();
    assert!(failed_names.contains("game_payload"));
    assert!(failed_names.contains("external_symlinks"));
    assert!(failed_names.contains("frameworks"));
    assert!(failed_names.contains("shader_cache_portability"));

    // `export_app_bundle` must refuse to export a failing bundle unless `force == true`.
    let export_dest = root.path().join("FakeSSD");
    assert!(export_app_bundle(&bad_app, &export_dest, false).is_err());
}

#[test]
fn parses_depot_downloader_release_and_scans_managed_downloads() {
    let release_json = r#"{
  "tag_name": "DepotDownloader_3.4.0",
  "assets": [
    {"name": "DepotDownloader-linux-x64.zip", "browser_download_url": "https://example.invalid/DepotDownloader-linux-x64.zip"},
    {"name": "DepotDownloader-macos-arm64.zip", "browser_download_url": "https://example.invalid/DepotDownloader-macos-arm64.zip"},
    {"name": "DepotDownloader-macos-x64.zip", "browser_download_url": "https://example.invalid/DepotDownloader-macos-x64.zip"}
  ]
}"#;
    let (tag, url) = parse_depot_downloader_release_asset(release_json, "arm64").unwrap();
    assert_eq!(tag, "DepotDownloader_3.4.0");
    assert_eq!(
        url,
        "https://example.invalid/DepotDownloader-macos-arm64.zip"
    );

    let root = make_temp_dir("managed_dl");
    let game_dir = root.path().join("Control");
    fs::create_dir_all(&game_dir).unwrap();
    fs::write(
        game_dir.join(".port-factory-game.tsv"),
        "app_id\t870780\ntitle\tControl Ultimate Edition\n",
    )
    .unwrap();
    fs::write(game_dir.join("steam_appid.txt"), "870780\n").unwrap();

    let db = Win32Database::default();
    let scanned = scan_port_factory_downloads_dir(root.path(), &db);
    assert_eq!(scanned.len(), 1);
    assert_eq!(scanned[0].app_id, "870780");
    assert_eq!(scanned[0].name, "Control Ultimate Edition");
    assert_eq!(scanned[0].source, GameLibrarySource::PortFactoryLibrary);

    // Verify `resolve_steam_target_app_id` matches both numeric AppID and case-insensitive title.
    let report = SteamLibraryScanReport {
        users: Vec::new(),
        library_folders: Vec::new(),
        installed_games: Vec::new(),
        owned_games: vec![OwnedSteamGame {
            app_id: 870780,
            title: "Control Ultimate Edition".to_string(),
            engine_hint: "NORTHLIGHT_DX12".to_string(),
            installed: false,
            install_source: None,
            playtime_minutes: 120,
            last_played_unix: 0,
            achievements_cached: 0,
        }],
    };
    let (id1, name1, _) = resolve_steam_target_app_id(&report, "control").unwrap();
    assert_eq!(id1, 870780);
    assert_eq!(name1, "Control Ultimate Edition");
    let (id2, name2, _) = resolve_steam_target_app_id(&report, "870780").unwrap();
    assert_eq!(id2, 870780);
    assert_eq!(name2, "Control Ultimate Edition");
    assert_eq!(
        parse_qr_cached_username(
            "Success! Next time you can login with -username alice123 -remember-password instead of -qr."
        ),
        Some("alice123".to_string())
    );
    assert_eq!(parse_qr_cached_username("no login line here"), None);
}

#[test]
fn writes_steam_browser_html_from_template() {
    let root = make_temp_dir("browser_html");
    let out_html = root.path().join("steam_library_browser.html");
    let report = SteamLibraryScanReport {
        users: vec![SteamUserAccount {
            steam_id64: "76561198000000000".to_string(),
            account_id32: "39734272".to_string(),
            account_name: "demo".to_string(),
            persona_name: "DemoPlayer".to_string(),
            most_recent: true,
        }],
        library_folders: Vec::new(),
        installed_games: Vec::new(),
        owned_games: vec![OwnedSteamGame {
            app_id: 870780,
            title: "Control Ultimate Edition".to_string(),
            engine_hint: "NORTHLIGHT_DX12".to_string(),
            installed: false,
            install_source: None,
            playtime_minutes: 90,
            last_played_unix: 0,
            achievements_cached: 5,
        }],
    };
    write_steam_browser_html(&report, &out_html).unwrap();
    let html = fs::read_to_string(&out_html).unwrap();
    assert!(html.contains("DemoPlayer (demo)"));
    assert!(html.contains("Control Ultimate Edition"));
    assert!(html.contains("1 Owned Steam Games"));
    assert!(!html.contains("__USER_BANNER__"));
    assert!(!html.contains("__OWNED_ROWS__"));
}
