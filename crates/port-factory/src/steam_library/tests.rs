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
        store: Some("steam"),
        app_id: "12345",
        steam_app_id: Some("12345"),
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
    assert!(!src.contains("__CLEAN_TITLE_DEBUG__"));
    assert!(!src.contains("__STORE_DEBUG__"));
    assert!(!src.contains("__APP_ID_DEBUG__"));
    assert!(!src.contains("__STEAM_APP_ID_DEBUG__"));
    assert!(!src.contains("__CACHE_KEY_DEBUG__"));
    assert!(!src.contains("__EXE_REL_DEBUG__"));

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

#[cfg(unix)]
#[test]
fn copy_game_payload_produces_bundle_passing_external_symlinks_check() {
    use crate::bundle_common::{codesign_ad_hoc, copy_game_payload};
    use std::os::unix::fs::symlink;

    let root = make_temp_dir("copy_game_symlinks_bundle");
    let outside_dir = root.path().join("outside");
    let game_src = root.path().join("FixtureGame");
    fs::create_dir_all(&outside_dir).unwrap();
    fs::create_dir_all(game_src.join("data")).unwrap();
    fs::create_dir_all(game_src.join("sub")).unwrap();
    fs::create_dir_all(game_src.join("dir_a")).unwrap();
    fs::create_dir_all(game_src.join("dir_b")).unwrap();

    fs::write(outside_dir.join("host_secret.txt"), b"HOST_ONLY").unwrap();
    fs::write(game_src.join("FixtureGame.exe"), b"MZ").unwrap();
    fs::write(game_src.join("data/asset.bin"), b"GAME_ASSET").unwrap();

    // (1) Escaping symlinks
    symlink("../outside/host_secret.txt", game_src.join("escape_rel.txt")).unwrap();
    symlink(&outside_dir, game_src.join("escape_dir")).unwrap();
    // (2) Dangling symlink
    symlink("nonexistent_asset.bin", game_src.join("dangling.bin")).unwrap();
    // (3) Cycles (self, ancestor, ELOOP, mutual directory)
    symlink(".", game_src.join("loop_self")).unwrap();
    symlink("..", game_src.join("sub/loop_parent")).unwrap();
    symlink("cycle_b", game_src.join("cycle_a")).unwrap();
    symlink("cycle_a", game_src.join("cycle_b")).unwrap();
    symlink("../dir_b", game_src.join("dir_a/to_b")).unwrap();
    symlink("../dir_a", game_src.join("dir_b/to_a")).unwrap();
    // (4) Safe interior relative symlinks
    symlink("data/asset.bin", game_src.join("safe_alias.bin")).unwrap();
    symlink(
        "../data/asset.bin",
        game_src.join("sub/safe_parent_alias.bin"),
    )
    .unwrap();

    // Build a minimal `.app` bundle and copy the fixture game payload into `Contents/Resources/Game`.
    let app_dir = root.path().join("FixtureGame.app");
    let contents = app_dir.join("Contents");
    let macos_dir = contents.join("MacOS");
    let frameworks_dir = contents.join("Frameworks");
    let resources_dir = contents.join("Resources");
    let bundle_game_dir = resources_dir.join("Game");
    fs::create_dir_all(&macos_dir).unwrap();
    fs::create_dir_all(&frameworks_dir).unwrap();
    fs::create_dir_all(&resources_dir).unwrap();
    fs::write(
        contents.join("Info.plist"),
        generate_info_plist("FixtureGame", "FixtureGame", "1"),
    )
    .unwrap();
    fs::write(
        resources_dir.join("game.toml"),
        generate_game_manifest_toml("FixtureGame", "FixtureGame", "FixtureGame.exe", "custom"),
    )
    .unwrap();
    // Compile a tiny native Mach-O stub so `codesign --force --sign -` and
    // `codesign --verify --deep --strict` exercise real signing on macOS without hanging.
    let stub_rs = root.path().join("stub_bin.rs");
    let stub_bin = root.path().join("stub_bin");
    fs::write(&stub_rs, "fn main() {}\n").unwrap();
    let compiled = Command::new("rustc")
        .args([
            "--edition=2021",
            &stub_rs.to_string_lossy(),
            "-o",
            &stub_bin.to_string_lossy(),
        ])
        .output()
        .expect("compile tiny stub binary with rustc");
    assert!(
        compiled.status.success(),
        "stub binary compile failed: {}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    fs::copy(&stub_bin, macos_dir.join("FixtureGame")).unwrap();
    fs::copy(&stub_bin, macos_dir.join("port-runner")).unwrap();
    fs::copy(&stub_bin, macos_dir.join("dxil-metald")).unwrap();

    let stub_dylib_rs = root.path().join("stub_dylib.rs");
    let stub_dylib = frameworks_dir.join("libmetalirconverter.dylib");
    fs::write(&stub_dylib_rs, "pub extern \"C\" fn stub() {}\n").unwrap();
    let compiled_dylib = Command::new("rustc")
        .args([
            "--edition=2021",
            "--crate-type",
            "cdylib",
            &stub_dylib_rs.to_string_lossy(),
            "-o",
            &stub_dylib.to_string_lossy(),
        ])
        .output()
        .expect("compile tiny stub dylib with rustc");
    assert!(
        compiled_dylib.status.success(),
        "stub dylib compile failed: {}",
        String::from_utf8_lossy(&compiled_dylib.stderr)
    );

    let copy_report = copy_game_payload(&game_src, &bundle_game_dir, None).unwrap();
    assert!(copy_report.skipped_symlinks() >= 7);

    // Unsafe links must not exist in the bundled Game tree; safe interior relative links stay.
    for skipped in [
        "escape_rel.txt",
        "escape_dir",
        "dangling.bin",
        "loop_self",
        "sub/loop_parent",
        "cycle_a",
        "cycle_b",
        "dir_b/to_a",
    ] {
        assert!(
            fs::symlink_metadata(bundle_game_dir.join(skipped)).is_err(),
            "unsafe symlink {skipped} should not be copied into bundle"
        );
    }
    assert!(
        fs::symlink_metadata(bundle_game_dir.join("safe_alias.bin"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::read(bundle_game_dir.join("safe_alias.bin")).unwrap(),
        b"GAME_ASSET"
    );
    assert!(
        fs::symlink_metadata(bundle_game_dir.join("sub/safe_parent_alias.bin"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::read(bundle_game_dir.join("sub/safe_parent_alias.bin")).unwrap(),
        b"GAME_ASSET"
    );

    codesign_ad_hoc(&stub_dylib);
    codesign_ad_hoc(&macos_dir.join("dxil-metald"));
    codesign_ad_hoc(&macos_dir.join("port-runner"));
    codesign_ad_hoc(&app_dir);

    // Fresh `--copy-game` bundle must pass `game_payload`, `external_symlinks`, and `codesign`.
    let report = verify_app_bundle(&app_dir).unwrap();
    let check_status = |rep: &BundleVerifyReport, name: &str| {
        rep.checks
            .iter()
            .find(|c| c.name == name)
            .map(|c| (c.status, c.detail.clone()))
    };
    assert_eq!(
        check_status(&report, "game_payload").map(|(s, _)| s),
        Some(BundleCheckStatus::Pass)
    );
    assert_eq!(
        check_status(&report, "external_symlinks").map(|(s, _)| s),
        Some(BundleCheckStatus::Pass),
        "external_symlinks failed: {:?}",
        check_status(&report, "external_symlinks")
    );
    #[cfg(target_os = "macos")]
    assert_eq!(
        check_status(&report, "codesign").map(|(s, _)| s),
        Some(BundleCheckStatus::Pass),
        "codesign failed: {:?}",
        check_status(&report, "codesign")
    );

    // `bundle-verify`'s `external_symlinks` check must keep failing if escaping or dangling links
    // make it into the bundle.
    let injected_escape = bundle_game_dir.join("injected_escape.txt");
    symlink(outside_dir.join("host_secret.txt"), &injected_escape).unwrap();
    let report_escape = verify_app_bundle(&app_dir).unwrap();
    assert_eq!(
        check_status(&report_escape, "external_symlinks").map(|(s, _)| s),
        Some(BundleCheckStatus::Fail)
    );
    fs::remove_file(&injected_escape).unwrap();

    let injected_dangling = bundle_game_dir.join("injected_dangling.txt");
    symlink("does_not_exist.txt", &injected_dangling).unwrap();
    let report_dangling = verify_app_bundle(&app_dir).unwrap();
    assert_eq!(
        check_status(&report_dangling, "external_symlinks").map(|(s, _)| s),
        Some(BundleCheckStatus::Fail)
    );
}

#[test]
fn parses_port_acquire_manifest_for_any_store_and_generates_game_toml() {
    use port_manifest::GameManifest;

    // 1. Epic Games Store manifest (with Unicode title, launch args, and an injected secret
    //    flag to prove defense-in-depth redaction).
    let epic_json = r#"{
      "schema_version": 1,
      "generated_by": "port-acquire",
      "store": "epic",
      "app_id": "a1b2c3d4e5f60718293a4b5c6d7e8f90",
      "title": "Hero\u2019s Journey 2",
      "platform": "windows",
      "version": "Build_20260810.0",
      "executable": "Bin\\Hero2.exe",
      "launch_args": [
        "-epicapp=a1b2c3d4e5f60718293a4b5c6d7e8f90",
        "-epicenv=Prod",
        "-EpicPortal",
        "-auth_password=SUPER_SECRET",
        "--token",
        "SECRET_TOK",
        "-epiclocale=en"
      ],
      "compatibility_signals": {
        "requires_eos": true,
        "requires_steamworks": false
      },
      "redistributables": [
        {
          "id": "vcredist:2022",
          "name": "Visual C++ 2022",
          "category": "vcredist",
          "native_in_win32_rt": true
        }
      ]
    }"#;

    let epic = parse_acquire_manifest(epic_json).unwrap();
    assert_eq!(epic.schema_version, 1);
    assert_eq!(epic.store, "epic");
    assert_eq!(epic.app_id, "a1b2c3d4e5f60718293a4b5c6d7e8f90");
    assert_eq!(epic.title.as_deref(), Some("Hero\u{2019}s Journey 2"));
    assert_eq!(epic.version.as_deref(), Some("Build_20260810.0"));
    assert_eq!(epic.executable.as_deref(), Some("Bin/Hero2.exe"));
    assert!(epic.had_unredacted_secrets);
    assert_eq!(epic.steam_app_id(), None);
    assert_eq!(
        epic.launch_args,
        vec![
            "-epicapp=a1b2c3d4e5f60718293a4b5c6d7e8f90",
            "-epicenv=Prod",
            "-EpicPortal",
            "-epiclocale=en",
        ]
    );

    let epic_toml = generate_bundle_game_manifest_toml(&BundleManifestInput {
        display_name: epic.title.as_deref().unwrap(),
        exec_ident: "HerosJourney2",
        exe_rel: "Bin/Hero2.exe",
        app_id: &epic.app_id,
        v8_jitless: true,
        acquire_manifest: Some(&epic),
    });
    assert!(epic_toml.contains("store = \"epic\""), "{epic_toml}");
    assert!(
        epic_toml.contains("app_id = \"a1b2c3d4e5f60718293a4b5c6d7e8f90\""),
        "{epic_toml}"
    );
    assert!(!epic_toml.contains("steam_app_id"), "{epic_toml}");
    assert!(!epic_toml.contains("SUPER_SECRET"), "{epic_toml}");
    assert!(!epic_toml.contains("SECRET_TOK"), "{epic_toml}");
    assert!(epic_toml.contains("v8_jitless = true"), "{epic_toml}");

    let parsed_epic = GameManifest::parse(&epic_toml);
    assert_eq!(parsed_epic.exe.as_deref(), Some("Bin/Hero2.exe"));
    assert_eq!(parsed_epic.steam_app_id, None);
    assert_eq!(parsed_epic.v8_jitless, Some(true));
    assert_eq!(parsed_epic.aot_dir.as_deref(), Some("AotCache"));
    assert_eq!(
        parsed_epic.split_args(),
        vec![
            "-epicapp=a1b2c3d4e5f60718293a4b5c6d7e8f90",
            "-epicenv=Prod",
            "-EpicPortal",
            "-epiclocale=en",
        ]
    );

    // 2. Steam manifest preserves both store/app_id and numeric steam_app_id.
    let steam_json = r#"{
      "schema_version": 1,
      "store": "steam",
      "app_id": "2357570",
      "title": "Steam Demo",
      "executable": "Demo.exe",
      "launch_args": ["-windowed", "-saveDir=My Saves"]
    }"#;
    let steam = parse_acquire_manifest(steam_json).unwrap();
    assert!(!steam.had_unredacted_secrets);
    assert_eq!(steam.steam_app_id(), Some("2357570"));
    let steam_toml = generate_bundle_game_manifest_toml(&BundleManifestInput {
        display_name: "Steam Demo",
        exec_ident: "SteamDemo",
        exe_rel: "Demo.exe",
        app_id: &steam.app_id,
        v8_jitless: false,
        acquire_manifest: Some(&steam),
    });
    assert!(steam_toml.contains("store = \"steam\""), "{steam_toml}");
    assert!(steam_toml.contains("app_id = \"2357570\""), "{steam_toml}");
    assert!(steam_toml.contains("steam_app_id = \"2357570\""), "{steam_toml}");
    let parsed_steam = GameManifest::parse(&steam_toml);
    assert_eq!(parsed_steam.steam_app_id.as_deref(), Some("2357570"));
    assert_eq!(parsed_steam.split_args(), vec!["-windowed", "-saveDir=My Saves"]);

    // 3. Any other storefront (e.g. GOG) preserves store, app_id, and args without polluting steam_app_id.
    let gog_json = r#"{
      "schema_version": 1,
      "store": "gog",
      "app_id": "1207658924",
      "title": "GOG Classic",
      "executable": "Classic.exe",
      "launch_args": ["-skipintro"]
    }"#;
    let gog = parse_acquire_manifest(gog_json).unwrap();
    assert_eq!(gog.steam_app_id(), None);
    let gog_toml = generate_bundle_game_manifest_toml(&BundleManifestInput {
        display_name: "GOG Classic",
        exec_ident: "GOGClassic",
        exe_rel: "Classic.exe",
        app_id: &gog.app_id,
        v8_jitless: false,
        acquire_manifest: Some(&gog),
    });
    assert!(gog_toml.contains("store = \"gog\""), "{gog_toml}");
    assert!(gog_toml.contains("app_id = \"1207658924\""), "{gog_toml}");
    assert!(!gog_toml.contains("steam_app_id"), "{gog_toml}");

    // 4. Local directory without a manifest (`"custom"`) never emits `steam_app_id = "custom"`.
    let custom_toml = generate_game_manifest_toml("LocalApp", "LocalApp", "LocalApp.exe", "custom");
    assert!(!custom_toml.contains("steam_app_id"), "{custom_toml}");

    // 5. Unsupported schema_version fails honestly.
    assert!(parse_acquire_manifest(r#"{"schema_version": 99, "store": "epic"}"#).is_err());

    // 6. `scan_post_install` picks up `native_in_win32_rt: true` redistributables from the manifest.
    let tmp = make_temp_dir("acquire_postinstall");
    fs::write(tmp.path().join(ACQUIRE_MANIFEST_FILENAME), epic_json).unwrap();
    let post = scan_post_install(tmp.path());
    assert!(
        post.bypassable_redistributables
            .iter()
            .any(|s| s.contains("Visual C++ 2022")),
        "expected native_in_win32_rt redist in {:?}",
        post.bypassable_redistributables
    );
}

#[test]
fn bundler_and_audit_game_agree_on_v8_cohtml_detection() {
    use crate::audit_game::{audit_game_dir, detect_v8_or_cohtml_in_game_dir, AuditGameOptions};
    use crate::win32_db::test_pe_fixture::{build_synthetic_pe, build_synthetic_win32_workspace};

    let tmp = make_temp_dir("v8_agreement");
    let fake_ws = tmp.path().join("ws");
    build_synthetic_win32_workspace(&fake_ws);
    let db = Win32Database::from_workspace_src(&fake_ws).unwrap();

    // Case A: ships `cohtml.WindowsDesktop.dll` beside plain EXE.
    let dir_a = tmp.path().join("GameWithCohtmlDll");
    fs::create_dir_all(&dir_a).unwrap();
    let plain_exe = build_synthetic_pe(
        false,
        &[0x31, 0xC0, 0xC3],
        &[("KERNEL32.dll", &["CreateFileW"])],
        false,
    );
    let cohtml_dll = build_synthetic_pe(
        true,
        &[0x31, 0xC0, 0xC3],
        &[("KERNEL32.dll", &["CreateFileW"])],
        false,
    );
    fs::write(dir_a.join("GameA.exe"), &plain_exe).unwrap();
    fs::write(dir_a.join("cohtml.WindowsDesktop.dll"), &cohtml_dll).unwrap();

    let audit_a = audit_game_dir(&dir_a, &db, &AuditGameOptions::default()).unwrap();
    let bundle_a = detect_v8_or_cohtml_in_game_dir(&dir_a);
    assert!(audit_a.runtime_hazards.v8_or_cohtml_detected);
    assert_eq!(bundle_a, audit_a.runtime_hazards.v8_or_cohtml_detected);

    // Case B: EXE statically imports `v8.dll` (even without `v8.dll` file on disk).
    let dir_b = tmp.path().join("GameImportingV8");
    fs::create_dir_all(&dir_b).unwrap();
    let v8_exe = build_synthetic_pe(
        false,
        &[0x31, 0xC0, 0xC3],
        &[("KERNEL32.dll", &["CreateFileW"]), ("v8.dll", &["V8Init"])],
        false,
    );
    fs::write(dir_b.join("GameB.exe"), &v8_exe).unwrap();

    let audit_b = audit_game_dir(&dir_b, &db, &AuditGameOptions::default()).unwrap();
    let bundle_b = detect_v8_or_cohtml_in_game_dir(&dir_b);
    assert!(audit_b.runtime_hazards.v8_or_cohtml_detected);
    assert_eq!(bundle_b, audit_b.runtime_hazards.v8_or_cohtml_detected);

    // Case C: Plain D3D12 game with no V8/Cohtml/CEF.
    let dir_c = tmp.path().join("PlainD3D12Game");
    fs::create_dir_all(&dir_c).unwrap();
    let d3d_exe = build_synthetic_pe(
        false,
        &[0x31, 0xC0, 0xC3],
        &[
            ("KERNEL32.dll", &["CreateFileW"]),
            ("d3d12.dll", &["D3D12CreateDevice"]),
        ],
        false,
    );
    fs::write(dir_c.join("GameC.exe"), &d3d_exe).unwrap();

    let audit_c = audit_game_dir(&dir_c, &db, &AuditGameOptions::default()).unwrap();
    let bundle_c = detect_v8_or_cohtml_in_game_dir(&dir_c);
    assert!(!audit_c.runtime_hazards.v8_or_cohtml_detected);
    assert_eq!(bundle_c, audit_c.runtime_hazards.v8_or_cohtml_detected);
}

#[test]
fn parallel_shader_precompile_defaults_to_all_and_honors_cap() {
    use crate::gpu_shaders::{plan_shader_precompile_jobs, precompile_shaders_for_paths};

    let tmp = make_temp_dir("shader_cap");
    let src_dir = tmp.path().join("shaders_src");
    fs::create_dir_all(&src_dir).unwrap();
    let spv = dxil_metal::spirv_metal::sample_vulkan_compute_spirv();
    for i in 0..5 {
        fs::write(src_dir.join(format!("comp_{i}.spv")), &spv).unwrap();
    }

    // 1. Always verify discovery, uncapped default (`usize::MAX`), explicit cap (`2`), and
    //    deterministic sorted job ordering without requiring `libmetalirconverter.dylib`
    //    (runs on CI runners that do not ship Metal Shader Converter).
    let planned_all = plan_shader_precompile_jobs(&[src_dir.clone()], usize::MAX);
    assert_eq!(planned_all.len(), 5);
    assert_eq!(
        planned_all
            .iter()
            .map(|(bin, _, _)| bin.as_str())
            .collect::<Vec<_>>(),
        vec![
            "comp_0.spv",
            "comp_1.spv",
            "comp_2.spv",
            "comp_3.spv",
            "comp_4.spv"
        ]
    );

    let planned_cap = plan_shader_precompile_jobs(&[src_dir.clone()], 2);
    assert_eq!(planned_cap.len(), 2);
    assert_eq!(
        planned_cap
            .iter()
            .map(|(bin, _, _)| bin.as_str())
            .collect::<Vec<_>>(),
        vec!["comp_0.spv", "comp_1.spv"]
    );

    // 2. Full end-to-end parallel compilation when Metal Shader Converter is installed
    //    (matching `gpu_shaders::tests::audits_and_precompiles_embedded_pe_and_vulkan_spirv_shaders`).
    //    When unavailable (e.g. GitHub Actions macOS runners), assert an honest error instead
    //    of faking a pass.
    let cache_all = tmp.path().join("cache_all");
    if dxil_metal::converter_available() {
        let all = precompile_shaders_for_paths(&[src_dir.clone()], Some(cache_all), usize::MAX)
            .expect("precompile all synthetic .spv shaders");
        assert_eq!(all.active_wave32_exports, 5);

        let cache_cap = tmp.path().join("cache_cap");
        let capped = precompile_shaders_for_paths(&[src_dir], Some(cache_cap), 2)
            .expect("precompile capped synthetic .spv shaders");
        assert_eq!(capped.active_wave32_exports, 2);

        if dxil_metal::spirv_metal::metal_toolchain_available() {
            assert_eq!(all.active_wave32_converted, 5);
            assert_eq!(all.active_wave32_failed, 0);
            assert_eq!(capped.active_wave32_converted, 2);
            assert_eq!(capped.active_wave32_failed, 0);
        }
    } else {
        eprintln!(
            "note: skipping end-to-end precompile_shaders_for_paths (Metal Shader Converter unavailable)"
        );
        assert!(
            precompile_shaders_for_paths(&[src_dir], Some(cache_all), usize::MAX).is_err(),
            "precompile_shaders_for_paths must fail honestly when libmetalirconverter.dylib is absent"
        );
    }
}

#[test]
fn bundle_verify_checks_acquire_manifest_and_flags_custom_steam_app_id() {
    let root = make_temp_dir("verify_acquire_manifest");
    let app = root.path().join("ManifestCheck.app");
    let contents = app.join("Contents");
    let resources = contents.join("Resources");
    fs::create_dir_all(contents.join("MacOS")).unwrap();
    fs::create_dir_all(contents.join("Frameworks")).unwrap();
    fs::create_dir_all(resources.join("Game")).unwrap();
    fs::write(
        contents.join("Info.plist"),
        generate_info_plist("ManifestCheck", "ManifestCheck", "1"),
    )
    .unwrap();

    // Legacy placeholder `steam_app_id = "custom"` must emit a WARN on `game_manifest`.
    fs::write(
        resources.join("game.toml"),
        "[app]\nid = \"mc\"\nexe = \"MC.exe\"\nsteam_app_id = \"custom\"\n",
    )
    .unwrap();
    // Clean `port-acquire-manifest.json` must PASS `acquire_manifest`.
    fs::write(
        resources.join(ACQUIRE_MANIFEST_FILENAME),
        r#"{"schema_version":1,"store":"epic","app_id":"abc123","launch_args":["-EpicPortal"]}"#,
    )
    .unwrap();

    let rep1 = verify_app_bundle(&app).unwrap();
    let find = |r: &BundleVerifyReport, name: &str| {
        r.checks
            .iter()
            .find(|c| c.name == name)
            .map(|c| (c.status, c.detail.clone()))
    };
    assert_eq!(
        find(&rep1, "game_manifest").map(|(s, _)| s),
        Some(BundleCheckStatus::Warn)
    );
    assert_eq!(
        find(&rep1, "acquire_manifest").map(|(s, _)| s),
        Some(BundleCheckStatus::Pass)
    );

    // Unredacted secret in bundled `port-acquire-manifest.json` must FAIL `acquire_manifest`.
    fs::write(
        resources.join(ACQUIRE_MANIFEST_FILENAME),
        r#"{"schema_version":1,"store":"epic","app_id":"abc123","launch_args":["-auth_password=leak"]}"#,
    )
    .unwrap();
    let rep2 = verify_app_bundle(&app).unwrap();
    assert_eq!(
        find(&rep2, "acquire_manifest").map(|(s, _)| s),
        Some(BundleCheckStatus::Fail)
    );
}

#[test]
fn steam_convert_surfaces_prewarm_aot_failure_clearly() {
    use crate::win32_db::test_pe_fixture::{build_synthetic_pe, build_synthetic_win32_workspace};

    let tmp = make_temp_dir("prewarm_fail_surface");
    let fake_ws = tmp.path().join("ws");
    build_synthetic_win32_workspace(&fake_ws);

    // Stage tiny fake `port-runner` (exits 1 with a diagnostic stderr error) and `dxil-metald`
    // in `<fake_ws>/target/aarch64-apple-darwin/release/` so `convert_steam_game` exercises the
    // real packaging + `--prewarm-aot` failure surfacing path quickly without invoking cargo.
    let rel_dir = fake_ws.join("target/aarch64-apple-darwin/release");
    fs::create_dir_all(&rel_dir).unwrap();
    let fake_runner_rs = tmp.path().join("fake_failing_runner.rs");
    fs::write(
        &fake_runner_rs,
        r#"fn main() {
    eprintln!("port-runner fatal startup error: parsing dbghelp.dll: not a PE32+ image (magic 0x10b)");
    std::process::exit(1);
}
"#,
    )
    .unwrap();
    let fake_runner = rel_dir.join("port-runner");
    let st1 = Command::new("rustc")
        .args([
            "--edition=2021",
            &fake_runner_rs.to_string_lossy(),
            "-o",
            &fake_runner.to_string_lossy(),
        ])
        .output()
        .unwrap();
    assert!(st1.status.success());

    let fake_shaderd_rs = tmp.path().join("fake_shaderd.rs");
    fs::write(
        &fake_shaderd_rs,
        "fn main() { println!(\"dxil-metald protocol v1\"); }\n",
    )
    .unwrap();
    let fake_shaderd = rel_dir.join("dxil-metald");
    let st2 = Command::new("rustc")
        .args([
            "--edition=2021",
            &fake_shaderd_rs.to_string_lossy(),
            "-o",
            &fake_shaderd.to_string_lossy(),
        ])
        .output()
        .unwrap();
    assert!(st2.status.success());

    // Synthetic Epic game directory with `port-acquire-manifest.json`, PE32+ EXE importing `v8.dll`,
    // and an isolated `PORT_VERIFY_CACHE_DIR`.
    let game_dir = tmp.path().join("SyntheticEpicGame");
    fs::create_dir_all(&game_dir).unwrap();
    let exe_bytes = build_synthetic_pe(
        false,
        &[0x31, 0xC0, 0xC3],
        &[("KERNEL32.dll", &["CreateFileW"]), ("v8.dll", &["V8Init"])],
        false,
    );
    fs::write(game_dir.join("SynthHero.exe"), &exe_bytes).unwrap();
    fs::write(
        game_dir.join(ACQUIRE_MANIFEST_FILENAME),
        r#"{
          "schema_version": 1,
          "store": "epic",
          "app_id": "deadbeef1234",
          "title": "Synth Hero",
          "version": "Build_1.0",
          "executable": "SynthHero.exe",
          "launch_args": ["-epicapp=deadbeef1234", "-EpicPortal"]
        }"#,
    )
    .unwrap();

    let out_app = tmp.path().join("Synth Hero.app");
    let res = convert_steam_game(
        &fake_ws,
        &SteamConvertOptions {
            target: game_dir.to_string_lossy().into_owned(),
            format: ConversionFormat::MacOsApp,
            output_path: Some(out_app.clone()),
            precompile_shaders: false,
            shader_limit: None,
            max_aot_functions: 0,
            bundle_shader_cache_from: None,
        },
    )
    .unwrap();

    // Clean up any external signing receipt written for this temporary synthetic bundle.
    if let (Some(cache_dir), Some(ref launcher)) = (
        crate::bundle_codesign::verify_receipt_cache_dir(),
        &res.standalone_executable,
    ) {
        if let Ok(cd) = crate::bundle_codesign::read_macho_code_directory_info(launcher) {
            let _ = fs::remove_file(cache_dir.join(format!("{}.tsv", cd.cdhash_full_hex)));
        }
    }

    assert_eq!(res.store.as_deref(), Some("epic"));
    assert_eq!(res.app_id, "deadbeef1234");
    assert!(res.v8_jitless);
    assert_eq!(res.aot_cache_modules, 0);
    let err = res
        .aot_prewarm_error
        .as_deref()
        .expect("prewarm failure must be surfaced in SteamConvertResult.aot_prewarm_error");
    assert!(
        err.contains("magic 0x10b"),
        "expected stderr diagnostic in aot_prewarm_error, got: {err}"
    );
    assert!(
        res.warnings.iter().any(|w| w.contains("magic 0x10b")),
        "expected prewarm failure in res.warnings: {:?}",
        res.warnings
    );

    let generated_toml = fs::read_to_string(out_app.join("Contents/Resources/game.toml")).unwrap();
    assert!(generated_toml.contains("store = \"epic\""), "{generated_toml}");
    assert!(generated_toml.contains("app_id = \"deadbeef1234\""), "{generated_toml}");
    assert!(!generated_toml.contains("steam_app_id"), "{generated_toml}");
    assert!(generated_toml.contains("v8_jitless = true"), "{generated_toml}");
    assert!(
        generated_toml.contains("args = \"-epicapp=deadbeef1234 -EpicPortal\""),
        "{generated_toml}"
    );
    assert!(out_app
        .join("Contents/Resources")
        .join(ACQUIRE_MANIFEST_FILENAME)
        .is_file());
}

#[test]
fn steam_convert_sanitizes_bundled_acquire_manifests_and_gates_non_steam_numeric_ids() {
    use crate::win32_db::test_pe_fixture::{build_synthetic_pe, build_synthetic_win32_workspace};

    let tmp = make_temp_dir("sanitize_and_gog_gate");
    let fake_ws = tmp.path().join("ws");
    build_synthetic_win32_workspace(&fake_ws);

    // Stage tiny fake `port-runner` (prints Steam env vars during launch, exits 0 on --prewarm-aot)
    // and fake `dxil-metald` so `convert_steam_game` runs end-to-end quickly.
    let rel_dir = fake_ws.join("target/aarch64-apple-darwin/release");
    fs::create_dir_all(&rel_dir).unwrap();
    let probe_runner_rs = tmp.path().join("probe_runner.rs");
    fs::write(
        &probe_runner_rs,
        r#"fn main() {
    for k in ["STEAM_APPID", "SteamAppId", "SteamGameId"] {
        println!("{k}={}", std::env::var(k).unwrap_or_default());
    }
}
"#,
    )
    .unwrap();
    let probe_runner = rel_dir.join("port-runner");
    let st1 = Command::new("rustc")
        .args([
            "--edition=2021",
            &probe_runner_rs.to_string_lossy(),
            "-o",
            &probe_runner.to_string_lossy(),
        ])
        .output()
        .unwrap();
    assert!(st1.status.success());

    let fake_shaderd_rs = tmp.path().join("fake_shaderd.rs");
    fs::write(
        &fake_shaderd_rs,
        "fn main() { println!(\"dxil-metald protocol v1\"); }\n",
    )
    .unwrap();
    let fake_shaderd = rel_dir.join("dxil-metald");
    let st2 = Command::new("rustc")
        .args([
            "--edition=2021",
            &fake_shaderd_rs.to_string_lossy(),
            "-o",
            &fake_shaderd.to_string_lossy(),
        ])
        .output()
        .unwrap();
    assert!(st2.status.success());

    // Synthetic GOG game directory with a NUMERIC non-Steam app_id (`1207658924`) AND
    // unredacted secret flags in `port-acquire-manifest.json`.
    let game_dir = tmp.path().join("SyntheticGogGame");
    fs::create_dir_all(&game_dir).unwrap();
    let exe_bytes = build_synthetic_pe(
        false,
        &[0x31, 0xC0, 0xC3],
        &[("KERNEL32.dll", &["CreateFileW"])],
        false,
    );
    fs::write(game_dir.join("Classic.exe"), &exe_bytes).unwrap();
    let raw_manifest = r#"{
  "schema_version": 1,
  "generated_by": "port-acquire",
  "store": "gog",
  "app_id": "1207658924",
  "title": "GOG Classic",
  "version": "1.2.3",
  "executable": "Classic.exe",
  "launch_args": [
    "-skipintro",
    "-auth_password=SUPER_SECRET_PW",
    "--token",
    "LEAKED_TOKEN_VAL",
    "-windowed"
  ]
}
"#;
    fs::write(game_dir.join(ACQUIRE_MANIFEST_FILENAME), raw_manifest).unwrap();

    let out_app = tmp.path().join("GOG Classic.app");
    let res = convert_steam_game(
        &fake_ws,
        &SteamConvertOptions {
            target: game_dir.to_string_lossy().into_owned(),
            format: ConversionFormat::MacOsApp,
            output_path: Some(out_app.clone()),
            precompile_shaders: false,
            shader_limit: None,
            max_aot_functions: 0,
            bundle_shader_cache_from: None,
        },
    )
    .unwrap();

    // Clean up external signing receipt for this synthetic bundle.
    if let (Some(cache_dir), Some(ref launcher)) = (
        crate::bundle_codesign::verify_receipt_cache_dir(),
        &res.standalone_executable,
    ) {
        if let Ok(cd) = crate::bundle_codesign::read_macho_code_directory_info(launcher) {
            let _ = fs::remove_file(cache_dir.join(format!("{}.tsv", cd.cdhash_full_hex)));
        }
    }

    // 1. Source manifest must remain untouched; BOTH bundled copies AND game.toml must strip secrets.
    let src_after = fs::read_to_string(game_dir.join(ACQUIRE_MANIFEST_FILENAME)).unwrap();
    assert_eq!(src_after, raw_manifest);

    let res_manifest =
        fs::read_to_string(out_app.join("Contents/Resources").join(ACQUIRE_MANIFEST_FILENAME))
            .unwrap();
    let game_manifest = fs::read_to_string(
        out_app
            .join("Contents/Resources/Game")
            .join(ACQUIRE_MANIFEST_FILENAME),
    )
    .unwrap();
    let generated_toml = fs::read_to_string(out_app.join("Contents/Resources/game.toml")).unwrap();
    for (label, content) in [
        ("Resources/port-acquire-manifest.json", &res_manifest),
        ("Resources/Game/port-acquire-manifest.json", &game_manifest),
        ("Resources/game.toml", &generated_toml),
    ] {
        assert!(
            !content.contains("SUPER_SECRET_PW") && !content.contains("LEAKED_TOKEN_VAL"),
            "{label} still contained secret launch args:\n{content}"
        );
        assert!(
            content.contains("-skipintro") && content.contains("-windowed"),
            "{label} lost non-secret launch args:\n{content}"
        );
    }
    assert!(res_manifest.contains("\"generated_by\": \"port-acquire\""));
    assert_eq!(res_manifest, game_manifest);

    // `bundle-verify` must PASS `acquire_manifest` on the sanitized bundle it just produced.
    let verify_rep = verify_app_bundle(&out_app).unwrap();
    let acq_check = verify_rep
        .checks
        .iter()
        .find(|c| c.name == "acquire_manifest")
        .expect("acquire_manifest check present");
    assert_eq!(
        acq_check.status,
        BundleCheckStatus::Pass,
        "expected acquire_manifest PASS, got: {:?}",
        acq_check
    );

    // 2. Non-Steam numeric GOG ID must NEVER write `steam_appid.txt` or export Steam env vars.
    assert!(!game_dir.join("steam_appid.txt").exists());
    assert!(!out_app.join("Contents/Resources/steam_appid.txt").exists());
    assert!(!out_app
        .join("Contents/Resources/Game/steam_appid.txt")
        .exists());
    assert!(!generated_toml.contains("steam_app_id"), "{generated_toml}");

    let launcher_bin = res
        .standalone_executable
        .as_ref()
        .expect("standalone launcher built");
    let run_out = Command::new(launcher_bin)
        .env_clear()
        .output()
        .expect("exec GOG launcher");
    assert!(run_out.status.success());
    let stdout = String::from_utf8_lossy(&run_out.stdout);
    assert!(stdout.contains("STEAM_APPID=\n"), "{stdout}");
    assert!(stdout.contains("SteamAppId=\n"), "{stdout}");
    assert!(stdout.contains("SteamGameId=\n"), "{stdout}");

    // 3. Injecting a secret ONLY into `Contents/Resources/Game/port-acquire-manifest.json`
    //    must cause `bundle-verify` to FAIL `acquire_manifest`.
    fs::write(
        out_app
            .join("Contents/Resources/Game")
            .join(ACQUIRE_MANIFEST_FILENAME),
        raw_manifest,
    )
    .unwrap();
    let tampered_rep = verify_app_bundle(&out_app).unwrap();
    let tampered_check = tampered_rep
        .checks
        .iter()
        .find(|c| c.name == "acquire_manifest")
        .unwrap();
    assert_eq!(tampered_check.status, BundleCheckStatus::Fail);
    assert!(
        tampered_check
            .detail
            .contains("Contents/Resources/Game/port-acquire-manifest.json"),
        "{}",
        tampered_check.detail
    );

    // 4. Positive control: a Steam launcher with `store: Some("steam")` and numeric
    //    `steam_app_id: Some("2357570")` MUST still export all three Steam env vars.
    let steam_app = tmp.path().join("SteamProbe.app");
    fs::create_dir_all(steam_app.join("Contents/MacOS")).unwrap();
    fs::create_dir_all(steam_app.join("Contents/Resources/Game")).unwrap();
    fs::create_dir_all(steam_app.join("Contents/Frameworks")).unwrap();
    fs::copy(&probe_runner, steam_app.join("Contents/MacOS/port-runner")).unwrap();
    let steam_src = generate_bundle_launcher_source(&BundleLauncherConfig {
        clean_title: "SteamProbe",
        store: Some("steam"),
        app_id: "2357570",
        steam_app_id: Some("2357570"),
        exe_rel: "Game.exe",
        image_base: 0x1_4000_0000,
        size_of_image: 0x1000,
        aot_fn_count: 0,
        aot_dylib_name: "",
        extra_aot_dylibs: &[],
    });
    let steam_rs = tmp.path().join("steam_probe.rs");
    let steam_bin = steam_app.join("Contents/MacOS/SteamProbe");
    fs::write(&steam_rs, &steam_src).unwrap();
    let st3 = Command::new("rustc")
        .args([
            "--edition=2021",
            &steam_rs.to_string_lossy(),
            "-o",
            &steam_bin.to_string_lossy(),
        ])
        .output()
        .unwrap();
    assert!(
        st3.status.success(),
        "{}",
        String::from_utf8_lossy(&st3.stderr)
    );
    let steam_run = Command::new(&steam_bin).env_clear().output().unwrap();
    assert!(steam_run.status.success());
    let steam_stdout = String::from_utf8_lossy(&steam_run.stdout);
    assert!(steam_stdout.contains("STEAM_APPID=2357570\n"), "{steam_stdout}");
    assert!(steam_stdout.contains("SteamAppId=2357570\n"), "{steam_stdout}");
    assert!(steam_stdout.contains("SteamGameId=2357570\n"), "{steam_stdout}");
}

#[test]
fn rejects_unsafe_acquire_manifest_ids_and_escapes_all_launcher_template_fields() {
    // 1. `parse_acquire_manifest` must reject code-injection / traversal syntax in `app_id`,
    //    `store`, and `executable`.
    let injected_app_id = r#"{
      "schema_version": 1,
      "store": "epic",
      "app_id": "\"); } std::process::exit(99); if true { println!(\""
    }"#;
    assert!(parse_acquire_manifest(injected_app_id).is_err());
    assert!(parse_acquire_manifest(
        r#"{"schema_version":1,"store":"epic","app_id":"../escape"}"#
    )
    .is_err());
    assert!(parse_acquire_manifest(
        r#"{"schema_version":1,"store":"epic;rm","app_id":"abc123"}"#
    )
    .is_err());
    assert!(parse_acquire_manifest(
        r#"{"schema_version":1,"store":"epic","app_id":"abc123","executable":"../evil.exe"}"#
    )
    .is_err());
    assert!(is_valid_app_id("a1b2c3d4e5f60718293a4b5c6d7e8f90"));
    assert!(is_valid_app_id("cat:item_1.0-rc1"));
    assert!(!is_valid_app_id(r#"" ); std::process::exit(99);"#));
    assert!(!is_valid_app_id("bad/path"));

    // 2. Defense-in-depth: even if adversarial strings are passed directly to
    //    `BundleLauncherConfig`, generated Rust code must compile cleanly and never execute
    //    injected code or expand nested template tokens.
    let tmp = make_temp_dir("launcher_injection_defense");
    let app = tmp.path().join("Safe.app");
    fs::create_dir_all(app.join("Contents/MacOS")).unwrap();
    fs::create_dir_all(app.join("Contents/Resources")).unwrap();
    fs::create_dir_all(app.join("Contents/Frameworks")).unwrap();

    let payload = "\"); } std::process::exit(99); if true { println!(\"__APP_ID_DEBUG__ {0}";
    let src = generate_bundle_launcher_source(&BundleLauncherConfig {
        clean_title: payload,
        store: Some(payload),
        app_id: payload,
        steam_app_id: Some(payload),
        exe_rel: payload,
        image_base: 0x1_4000_0000,
        size_of_image: 0x1000,
        aot_fn_count: 0,
        aot_dylib_name: payload,
        extra_aot_dylibs: &[],
    });
    let rs_path = tmp.path().join("safe_launcher.rs");
    let bin_path = app.join("Contents/MacOS/Safe");
    fs::write(&rs_path, &src).unwrap();
    let st = Command::new("rustc")
        .args([
            "--edition=2021",
            &rs_path.to_string_lossy(),
            "-o",
            &bin_path.to_string_lossy(),
        ])
        .output()
        .unwrap();
    assert!(
        st.status.success(),
        "escaped launcher failed to compile:\n{}\n--- source ---\n{}",
        String::from_utf8_lossy(&st.stderr),
        src
    );

    let out = Command::new(&bin_path)
        .arg("--aot-info")
        .env_clear()
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "injected exit(99) or crash occurred! status={:?}, stderr={}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains(payload), "expected literal payload printed: {stdout}");
    assert!(stdout.contains("Steam AppID            : \n"), "{stdout}");
}
