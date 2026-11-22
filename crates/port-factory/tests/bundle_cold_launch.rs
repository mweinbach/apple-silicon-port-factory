//! End-to-end verification of self-contained macOS `.app` bundles under an empty
//! Finder-like environment (`env_clear()`).
//!
//! Stages a temporary `.app` bundle (deleted unconditionally on drop), compiles its
//! generated native ARM64 launcher, and verifies:
//! 1. `Contents/Info.plist` is valid XML (`plutil -lint`) with all required Finder keys.
//! 2. `Contents/Resources/game.toml`, `Contents/Resources/AotCache`, `Contents/MacOS/dxil-metald`,
//!    and `Contents/Frameworks/*.dylib` all resolve cleanly under `env_clear()`.
//! 3. When real `dxil-metald` and `libmetalirconverter.dylib` are present on the host, the bundled
//!    `Contents/MacOS/dxil-metald --version` cold-launches under `env_clear()` and loads
//!    `Contents/Frameworks/libmetalirconverter.dylib`.
//! 4. When real `port-runner` is built in workspace `target/`, cold-launching
//!    `Contents/MacOS/<Launcher>` under `env_clear()` loads `Contents/Resources/game.toml`,
//!    prewarms a real `.aot` block cache into `Contents/Resources/AotCache`, and executes an
//!    exported x86-64 function from the bundled PE via ARM64 translation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use port_factory::steam_library::{
    format_bundle_verify_report, generate_bundle_launcher_source, generate_game_manifest_toml,
    generate_info_plist, stage_bundle_frameworks, verify_app_bundle, BundleLauncherConfig,
};

struct TempDirGuard(PathBuf);

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn make_temp_dir(tag: &str) -> TempDirGuard {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "port_factory_cold_launch_{tag}_{}_{}",
        std::process::id(),
        seq
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp test dir");
    TempDirGuard(dir)
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(Path::to_path_buf)
        .expect("resolve workspace root")
}

fn find_workspace_binary(ws_root: &Path, bin_name: &str) -> Option<PathBuf> {
    let mut cands = Vec::new();
    for rel in [
        format!("target/aarch64-apple-darwin/release/{bin_name}"),
        format!("target/release/{bin_name}"),
        format!("target/aarch64-apple-darwin/debug/{bin_name}"),
        format!("target/debug/{bin_name}"),
    ] {
        let p = ws_root.join(rel);
        if p.is_file() {
            cands.push(p);
        }
    }
    cands.sort_by_key(|p| std::cmp::Reverse(p.metadata().and_then(|m| m.modified()).ok()));
    cands.into_iter().next()
}

/// Build a minimal valid x86-64 PE32+ executable (`ColdTest.exe`) with:
/// - `.text` @ RVA 0x1000:
///   - `0x1000` (EntryPoint): `xor eax, eax; ret`
///   - `0x1040` (exported `add_and_tag`): `lea rax, [rcx + rdx + 7]; ret`
/// - `.rdata` @ RVA 0x2000:
///   - Export Directory @ 0x2000 exporting `add_and_tag` -> RVA 0x1040
///   - `.pdata` @ 0x2200 covering both `0x1000` and `0x1040` for offline `--prewarm-aot`
fn build_synthetic_pe32plus() -> Vec<u8> {
    let mut img = vec![0u8; 0x800];
    let w16 = |buf: &mut [u8], off: usize, v: u16| buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
    let w32 = |buf: &mut [u8], off: usize, v: u32| buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    let w64 = |buf: &mut [u8], off: usize, v: u64| buf[off..off + 8].copy_from_slice(&v.to_le_bytes());

    // DOS header
    img[0..2].copy_from_slice(b"MZ");
    let pe = 0x80usize;
    w32(&mut img, 0x3C, pe as u32); // e_lfanew (> 0x40 required by goblin)

    // PE signature + COFF header (20 bytes @ pe + 4)
    w32(&mut img, pe, 0x0000_4550); // "PE\0\0"
    w16(&mut img, pe + 4, 0x8664); // IMAGE_FILE_MACHINE_AMD64
    w16(&mut img, pe + 6, 2); // NumberOfSections
    w16(&mut img, pe + 20, 240); // SizeOfOptionalHeader
    w16(&mut img, pe + 22, 0x0022); // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE

    // Optional header PE32+ (240 bytes @ pe + 24)
    let opt = pe + 24;
    w16(&mut img, opt, 0x20B); // PE32+ magic
    w32(&mut img, opt + 16, 0x1000); // AddressOfEntryPoint
    w64(&mut img, opt + 24, 0x1_4000_0000); // ImageBase
    w32(&mut img, opt + 32, 0x1000); // SectionAlignment
    w32(&mut img, opt + 36, 0x200); // FileAlignment
    w16(&mut img, opt + 40, 6); // MajorOperatingSystemVersion
    w16(&mut img, opt + 48, 6); // MajorSubsystemVersion
    w32(&mut img, opt + 56, 0x3000); // SizeOfImage
    w32(&mut img, opt + 60, 0x200); // SizeOfHeaders
    w16(&mut img, opt + 68, 3); // Subsystem = CUI
    w16(&mut img, opt + 70, 0x0040); // DllCharacteristics = DYNAMIC_BASE
    w64(&mut img, opt + 72, 0x0010_0000); // SizeOfStackReserve
    w64(&mut img, opt + 80, 0x0001_0000); // SizeOfStackCommit
    w64(&mut img, opt + 88, 0x0010_0000); // SizeOfHeapReserve
    w64(&mut img, opt + 96, 0x0001_0000); // SizeOfHeapCommit
    w32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

    // DataDirectory[0]: Export Directory @ RVA 0x2000 (size 0x80)
    w32(&mut img, opt + 112, 0x2000);
    w32(&mut img, opt + 112 + 4, 0x80);
    // DataDirectory[3]: Exception Directory (.pdata) @ RVA 0x2200 (2 * 12 = 24 bytes)
    w32(&mut img, opt + 112 + 3 * 8, 0x2200);
    w32(&mut img, opt + 112 + 3 * 8 + 4, 24);

    // Section 0: `.text` (RVA 0x1000, raw 0x200, size 0x200)
    let sec0 = opt + 240;
    img[sec0..sec0 + 5].copy_from_slice(b".text");
    w32(&mut img, sec0 + 8, 0x200); // VirtualSize
    w32(&mut img, sec0 + 12, 0x1000); // VirtualAddress
    w32(&mut img, sec0 + 16, 0x200); // SizeOfRawData
    w32(&mut img, sec0 + 20, 0x200); // PointerToRawData
    w32(&mut img, sec0 + 36, 0x6000_0020); // MEM_EXECUTE | MEM_READ | CNT_CODE

    // Section 1: `.rdata` (RVA 0x2000, raw 0x400, size 0x400)
    let sec1 = sec0 + 40;
    img[sec1..sec1 + 6].copy_from_slice(b".rdata");
    w32(&mut img, sec1 + 8, 0x400); // VirtualSize
    w32(&mut img, sec1 + 12, 0x2000); // VirtualAddress
    w32(&mut img, sec1 + 16, 0x400); // SizeOfRawData
    w32(&mut img, sec1 + 20, 0x400); // PointerToRawData
    w32(&mut img, sec1 + 36, 0x4000_0040); // MEM_READ | CNT_INITIALIZED_DATA

    // `.text` @ file offset 0x200:
    // EntryPoint (RVA 0x1000): xor eax, eax; ret
    let entry_code: &[u8] = &[0x31, 0xC0, 0xC3];
    img[0x200..0x200 + entry_code.len()].copy_from_slice(entry_code);
    // Export `add_and_tag` (RVA 0x1040): lea rax, [rcx + rdx + 7]; ret
    let export_code: &[u8] = &[0x48, 0x8D, 0x44, 0x11, 0x07, 0xC3];
    img[0x240..0x240 + export_code.len()].copy_from_slice(export_code);

    // `.rdata` @ file offset 0x400 (maps to RVA 0x2000):
    // IMAGE_EXPORT_DIRECTORY (40 bytes @ RVA 0x2000 / file 0x400)
    let exp = 0x400usize;
    w32(&mut img, exp + 12, 0x2050); // Name RVA
    w32(&mut img, exp + 16, 1); // Base ordinal
    w32(&mut img, exp + 20, 1); // NumberOfFunctions
    w32(&mut img, exp + 24, 1); // NumberOfNames
    w32(&mut img, exp + 28, 0x2030); // AddressOfFunctions RVA
    w32(&mut img, exp + 32, 0x2034); // AddressOfNames RVA
    w32(&mut img, exp + 36, 0x2038); // AddressOfNameOrdinals RVA

    w32(&mut img, exp + 0x30, 0x1040); // Function[0] -> add_and_tag
    w32(&mut img, exp + 0x34, 0x2060); // Name[0] -> "add_and_tag"
    w16(&mut img, exp + 0x38, 0); // Ordinal[0] -> index 0
    img[exp + 0x50..exp + 0x50 + 13].copy_from_slice(b"ColdTest.exe\0");
    img[exp + 0x60..exp + 0x60 + 12].copy_from_slice(b"add_and_tag\0");

    // `.pdata` @ RVA 0x2200 / file 0x600
    let pdata = 0x600usize;
    w32(&mut img, pdata, 0x1000);
    w32(&mut img, pdata + 4, 0x1003);
    w32(&mut img, pdata + 8, 0x2300);
    w32(&mut img, pdata + 12, 0x1040);
    w32(&mut img, pdata + 16, 0x1046);
    w32(&mut img, pdata + 20, 0x2300);
    img[0x700] = 0x01; // UNWIND_INFO version 1, flags 0 @ RVA 0x2300

    // Sanity check with pe-image parser
    let pe = pe_image::Pe::parse(&img).expect("synthetic PE32+ must parse cleanly");
    let exports = pe.exports().expect("parse exports");
    assert!(exports.by_name("add_and_tag").is_some());
    img
}

#[test]
fn info_plist_and_manifest_are_valid_for_finder_cold_launch() {
    let tmp = make_temp_dir("plist");
    let plist_xml = generate_info_plist("Cold & Test", "ColdTest", "");
    assert!(plist_xml.contains("<key>CFBundleExecutable</key>"));
    assert!(plist_xml.contains("<string>ColdTest</string>"));
    assert!(plist_xml.contains("<key>CFBundleIdentifier</key>"));
    assert!(plist_xml.contains("<string>com.portfactory.steam.coldtest</string>"));
    assert!(plist_xml.contains("<key>CFBundleName</key>"));
    assert!(plist_xml.contains("<string>Cold &amp; Test</string>"));
    assert!(plist_xml.contains("<key>NSHighResolutionCapable</key>"));
    assert!(plist_xml.contains("<true/>"));
    assert!(plist_xml.contains("<key>LSMinimumSystemVersion</key>"));
    assert!(plist_xml.contains("<key>CFBundleVersion</key>\n    <string>1</string>"));

    let plist_path = tmp.0.join("Info.plist");
    fs::write(&plist_path, &plist_xml).unwrap();

    #[cfg(target_os = "macos")]
    {
        let lint = Command::new("/usr/bin/plutil")
            .args(["-lint", &plist_path.to_string_lossy()])
            .output()
            .expect("run plutil -lint");
        assert!(
            lint.status.success(),
            "Info.plist failed plutil -lint: {}\n{}",
            String::from_utf8_lossy(&lint.stdout),
            String::from_utf8_lossy(&lint.stderr)
        );
    }

    let toml = generate_game_manifest_toml("ColdTest", "ColdTest", "ColdTest.exe", "999999");
    assert!(toml.contains("id = \"coldtest\""));
    assert!(toml.contains("name = \"ColdTest\""));
    assert!(toml.contains("exe = \"ColdTest.exe\""));
    assert!(toml.contains("win_dir = \"C:\\\\Games\\\\ColdTest\""));
    assert!(toml.contains("steam_app_id = \"999999\""));
    assert!(toml.contains("aot_dir = \"AotCache\""));
}

#[test]
fn self_contained_app_bundle_cold_launch_with_empty_env() {
    let ws_root = workspace_root();
    let tmp = make_temp_dir("e2e");
    let app_dir = tmp.0.join("ColdTest.app");
    let contents_dir = app_dir.join("Contents");
    let macos_dir = contents_dir.join("MacOS");
    let frameworks_dir = contents_dir.join("Frameworks");
    let resources_dir = contents_dir.join("Resources");
    let game_dir = resources_dir.join("Game");
    let aot_cache_dir = resources_dir.join("AotCache");
    let shader_cache_dir = resources_dir.join("ShaderCache");
    let drive_c_dir = tmp.0.join("drive_c");

    for d in [
        &macos_dir,
        &frameworks_dir,
        &game_dir,
        &aot_cache_dir,
        &shader_cache_dir,
        &drive_c_dir,
    ] {
        fs::create_dir_all(d).unwrap();
    }

    // 1. Write Info.plist, game.toml, and synthetic PE32+ executable.
    let plist_path = contents_dir.join("Info.plist");
    fs::write(&plist_path, generate_info_plist("ColdTest", "ColdTest", "100")).unwrap();
    let manifest_path = resources_dir.join("game.toml");
    fs::write(
        &manifest_path,
        generate_game_manifest_toml("ColdTest", "ColdTest", "ColdTest.exe", "999999"),
    )
    .unwrap();
    let pe_path = game_dir.join("ColdTest.exe");
    fs::write(&pe_path, build_synthetic_pe32plus()).unwrap();

    // 2. Stage real host frameworks into Contents/Frameworks/ (using temporary mock markers only
    //    during Phase A for any optional framework not installed on this machine).
    let real_staged = stage_bundle_frameworks(&frameworks_dir, &ws_root).unwrap();
    let has_real_msc = real_staged.metal_irconverter.is_some();
    let fw_msc = frameworks_dir.join("libmetalirconverter.dylib");
    let fw_dxilconv = frameworks_dir.join("libdxilconv.dylib");
    let fw_moltenvk = frameworks_dir.join("libMoltenVK.dylib");
    let mut mock_frameworks = Vec::new();
    if !fw_msc.is_file() {
        fs::write(&fw_msc, b"MOCK_MSC").unwrap();
        mock_frameworks.push(fw_msc.clone());
    }
    if !fw_dxilconv.is_file() {
        fs::write(&fw_dxilconv, b"MOCK_DXILCONV").unwrap();
        mock_frameworks.push(fw_dxilconv.clone());
    }
    if !fw_moltenvk.is_file() {
        fs::write(&fw_moltenvk, b"MOCK_MVK").unwrap();
        mock_frameworks.push(fw_moltenvk.clone());
    }

    // 3. Stage `dxil-metald` into Contents/MacOS/ (real binary when built, else placeholder).
    let real_shaderd = find_workspace_binary(&ws_root, "dxil-metald");
    let bundled_shaderd = macos_dir.join("dxil-metald");
    if let Some(ref src) = real_shaderd {
        fs::copy(src, &bundled_shaderd).unwrap();
    } else {
        fs::write(&bundled_shaderd, b"MOCK_DXIL_METALD").unwrap();
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&bundled_shaderd, fs::Permissions::from_mode(0o755)).unwrap();
    }

    // 4. Compile the generated native ARM64 bundle launcher (`Contents/MacOS/ColdTest`).
    let launcher_cfg = BundleLauncherConfig {
        clean_title: "ColdTest",
        app_id: "999999",
        exe_rel: "ColdTest.exe",
        image_base: 0x1_4000_0000,
        size_of_image: 0x3000,
        aot_fn_count: 2,
        aot_dylib_name: "libcoldtest_aot.dylib",
        extra_aot_dylibs: &[],
    };
    let launcher_rs = tmp.0.join("coldtest_launcher.rs");
    let launcher_bin = macos_dir.join("ColdTest");
    fs::write(&launcher_rs, generate_bundle_launcher_source(&launcher_cfg)).unwrap();
    let rustc_launcher = Command::new("rustc")
        .args([
            "--edition=2021",
            &launcher_rs.to_string_lossy(),
            "-o",
            &launcher_bin.to_string_lossy(),
        ])
        .output()
        .expect("compile bundle launcher");
    assert!(
        rustc_launcher.status.success(),
        "bundle launcher compile failed:\n{}",
        String::from_utf8_lossy(&rustc_launcher.stderr)
    );

    // 5. Phase A: Verify exact empty-env (`env_clear()`) resolution via a compiled probe runner.
    let probe_rs = tmp.0.join("probe_runner.rs");
    let bundled_runner = macos_dir.join("port-runner");
    fs::write(
        &probe_rs,
        r#"fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("ARGV={}", args.join("|"));
    for k in [
        "METAL_IRCONVERTER_DYLIB",
        "AW2_DXILCONV",
        "PORT_VULKAN_DYLIB",
        "DYLD_FALLBACK_LIBRARY_PATH",
        "PATH",
        "PORT_GAME_DIR",
        "PORT_MANIFEST",
        "PORT_AOT_DIR",
        "AW2_DLL_AOT_DIR",
        "PORT_AOT_FALLBACK_DIR",
        "PORT_SHADER_CACHE",
        "AW2_SHADER_CACHE",
        "PORT_SHADER_CACHE_FALLBACK",
        "PORT_SHADER_MANIFEST_FALLBACK",
        "PORT_PSO_MANIFEST_FALLBACK",
        "PORT_SHADERD",
        "AW2_SHADERD",
        "STEAM_APPID",
    ] {
        println!("{k}={}", std::env::var(k).unwrap_or_default());
    }
}
"#,
    )
    .unwrap();
    let rustc_probe = Command::new("rustc")
        .args([
            "--edition=2021",
            &probe_rs.to_string_lossy(),
            "-o",
            &bundled_runner.to_string_lossy(),
        ])
        .output()
        .expect("compile probe port-runner");
    assert!(rustc_probe.status.success());

    let probe_out = Command::new(&launcher_bin)
        .env_clear()
        .output()
        .expect("cold-launch bundle with empty env");
    assert!(
        probe_out.status.success(),
        "cold launch probe failed: {}",
        String::from_utf8_lossy(&probe_out.stderr)
    );
    let env_map: HashMap<String, String> = String::from_utf8_lossy(&probe_out.stdout)
        .lines()
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let canon_fw = fs::canonicalize(&frameworks_dir).unwrap();
    let canon_macos = fs::canonicalize(&macos_dir).unwrap();
    let canon_res = fs::canonicalize(&resources_dir).unwrap();
    assert_eq!(
        fs::canonicalize(env_map.get("METAL_IRCONVERTER_DYLIB").unwrap()).unwrap(),
        canon_fw.join("libmetalirconverter.dylib")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("AW2_DXILCONV").unwrap()).unwrap(),
        canon_fw.join("libdxilconv.dylib")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("PORT_VULKAN_DYLIB").unwrap()).unwrap(),
        canon_fw.join("libMoltenVK.dylib")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("PORT_MANIFEST").unwrap()).unwrap(),
        canon_res.join("game.toml")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("PORT_AOT_DIR").unwrap()).unwrap(),
        canon_res.join("AotCache")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("AW2_DLL_AOT_DIR").unwrap()).unwrap(),
        canon_res.join("AotCache")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("PORT_SHADER_CACHE").unwrap()).unwrap(),
        canon_res.join("ShaderCache")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("PORT_AOT_FALLBACK_DIR").unwrap()).unwrap(),
        canon_res.join("AotCache")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("PORT_SHADER_CACHE_FALLBACK").unwrap()).unwrap(),
        canon_res.join("ShaderCache")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("PORT_SHADERD").unwrap()).unwrap(),
        canon_macos.join("dxil-metald")
    );
    assert_eq!(
        fs::canonicalize(env_map.get("AW2_SHADERD").unwrap()).unwrap(),
        canon_macos.join("dxil-metald")
    );
    assert_eq!(env_map.get("STEAM_APPID").map(String::as_str), Some("999999"));
    assert_eq!(
        fs::canonicalize(env_map.get("PATH").unwrap().split(':').next().unwrap()).unwrap(),
        canon_macos
    );
    assert_eq!(
        fs::canonicalize(
            env_map
                .get("DYLD_FALLBACK_LIBRARY_PATH")
                .unwrap()
                .split(':')
                .next()
                .unwrap()
        )
        .unwrap(),
        canon_fw
    );
    let argv = env_map.get("ARGV").unwrap();
    assert!(argv.contains("--game-dir|") && argv.contains("--pe|"), "argv={argv}");

    // Remove temporary non-Mach-O mock markers before real Mach-O codesigning and execution.
    for mock in &mock_frameworks {
        let _ = fs::remove_file(mock);
    }

    // 6. Phase B: Real `dxil-metald` inside `Contents/MacOS/` cold-launched under `env_clear()`
    //    must find and dlopen `Contents/Frameworks/libmetalirconverter.dylib` with ZERO env vars.
    if real_shaderd.is_some() && has_real_msc {
        #[cfg(target_os = "macos")]
        {
            for entry in fs::read_dir(&frameworks_dir).unwrap().flatten() {
                let p = entry.path();
                if p.is_file() {
                    let sign = Command::new("codesign")
                        .args(["--force", "--sign", "-", &p.to_string_lossy()])
                        .output()
                        .expect("codesign staged framework");
                    assert!(
                        sign.status.success(),
                        "codesign failed on {}: {}",
                        p.display(),
                        String::from_utf8_lossy(&sign.stderr)
                    );
                }
            }
            let sign = Command::new("codesign")
                .args(["--force", "--sign", "-", &bundled_shaderd.to_string_lossy()])
                .output()
                .expect("codesign bundled dxil-metald");
            assert!(
                sign.status.success(),
                "codesign failed on {}: {}",
                bundled_shaderd.display(),
                String::from_utf8_lossy(&sign.stderr)
            );
        }
        let ver = Command::new(&bundled_shaderd)
            .arg("--version")
            .env_clear()
            .output()
            .expect("run bundled dxil-metald --version with empty env");
        assert!(
            ver.status.success(),
            "bundled dxil-metald --version failed under env_clear(): {}",
            String::from_utf8_lossy(&ver.stderr)
        );
        let ver_out = String::from_utf8_lossy(&ver.stdout);
        assert!(
            ver_out.starts_with("dxil-metald protocol v"),
            "unexpected dxil-metald --version output: {ver_out:?}"
        );
    }

    // 7. Phase C: Real `port-runner` end-to-end cold launch under `env_clear()`:
    //    - `--prewarm-aot` writes a real `.aot` block cache into `Contents/Resources/AotCache`
    //    - `--call-export add_and_tag` loads `Contents/Resources/game.toml` + prewarmed `AotCache`
    //      and executes the x86-64 function natively on ARM64!
    let Some(real_runner) = find_workspace_binary(&ws_root, "port-runner") else {
        eprintln!("note: skipping real port-runner Phase C (port-runner not built in target/)");
        return;
    };
    let _ = fs::remove_file(&bundled_runner);
    fs::copy(&real_runner, &bundled_runner).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&bundled_runner, fs::Permissions::from_mode(0o755)).unwrap();
    }
    #[cfg(target_os = "macos")]
    {
        if real_shaderd.is_none() {
            let _ = fs::remove_file(&bundled_shaderd);
        }
        for entry in fs::read_dir(&frameworks_dir).unwrap().flatten() {
            let p = entry.path();
            if p.is_file() {
                let sign = Command::new("codesign")
                    .args(["--force", "--sign", "-", &p.to_string_lossy()])
                    .output()
                    .expect("codesign framework");
                assert!(
                    sign.status.success(),
                    "codesign failed on {}: {}",
                    p.display(),
                    String::from_utf8_lossy(&sign.stderr)
                );
            }
        }
        for target in [&bundled_runner, &launcher_bin, &app_dir] {
            let sign = Command::new("codesign")
                .args(["--force", "--sign", "-", &target.to_string_lossy()])
                .output()
                .expect("codesign bundle binary");
            assert!(
                sign.status.success(),
                "codesign failed on {}: {}",
                target.display(),
                String::from_utf8_lossy(&sign.stderr)
            );
        }
        let verify = Command::new("codesign")
            .args(["--verify", "--deep", "--strict", &app_dir.to_string_lossy()])
            .output()
            .expect("verify .app bundle code signature");
        assert!(
            verify.status.success(),
            "codesign --verify failed on {}: {}",
            app_dir.display(),
            String::from_utf8_lossy(&verify.stderr)
        );
    }

    // Step C1: Cold-launch `--prewarm-aot` with completely empty environment.
    let prewarm = Command::new(&launcher_bin)
        .args(["--prewarm-aot", "--drive-c"])
        .arg(&drive_c_dir)
        .env_clear()
        .output()
        .expect("cold-launch --prewarm-aot");
    assert!(
        prewarm.status.success(),
        "cold-launch --prewarm-aot failed (status {:?}):\nstdout:\n{}\nstderr:\n{}",
        prewarm.status,
        String::from_utf8_lossy(&prewarm.stdout),
        String::from_utf8_lossy(&prewarm.stderr)
    );
    let aot_files: Vec<PathBuf> = fs::read_dir(&aot_cache_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("aot"))
        .collect();
    assert!(
        !aot_files.is_empty(),
        "expected prewarmed .aot cache file in {}, stdout:\n{}\nstderr:\n{}",
        aot_cache_dir.display(),
        String::from_utf8_lossy(&prewarm.stdout),
        String::from_utf8_lossy(&prewarm.stderr)
    );
    assert!(
        fs::metadata(&aot_files[0]).unwrap().len() > 0,
        "prewarmed .aot cache file is empty"
    );

    // Step C2: Cold-launch `--call-export add_and_tag` with completely empty environment.
    let call = Command::new(&launcher_bin)
        .args(["--call-export", "add_and_tag", "--drive-c"])
        .arg(&drive_c_dir)
        .args(["--", "100", "23"])
        .env_clear()
        .output()
        .expect("cold-launch --call-export add_and_tag");
    let call_stdout = String::from_utf8_lossy(&call.stdout);
    let call_stderr = String::from_utf8_lossy(&call.stderr);
    assert!(
        call.status.success(),
        "cold-launch --call-export failed (status {:?}):\nstdout:\n{call_stdout}\nstderr:\n{call_stderr}",
        call.status
    );
    assert!(
        call_stdout.contains("returned 0x82"),
        "expected add_and_tag(100, 23) == 130 (0x82), got:\nstdout:\n{call_stdout}\nstderr:\n{call_stderr}"
    );
    // Verify `port-runner` loaded `win_dir = "C:\\Games\\ColdTest"` from `Contents/Resources/game.toml`.
    assert!(
        call_stderr.contains("C:\\Games\\ColdTest ->"),
        "expected port-runner to mount win_dir from Contents/Resources/game.toml, stderr:\n{call_stderr}"
    );
    // Verify `port-runner` activated the bundled `Contents/Resources/AotCache` directory under `env_clear()`.
    assert!(
        call_stderr.contains("emu aot:") && call_stderr.contains("Contents/Resources/AotCache"),
        "expected port-runner to activate AOT cache from Contents/Resources/AotCache, stderr:\n{call_stderr}"
    );

    // Step C3: Simulate copying the signed `.app` bundle into read-only `/Applications` on a
    // brand-new Mac (`HOME=<fresh_home>`, `Contents/Resources` locked `a-w`). Verify:
    //   - the bundle executes cleanly without writing inside `Contents/Resources`
    //   - `port-runner` loads the bundled `.aot` cache via `PORT_AOT_FALLBACK_DIR`
    //   - the `.app` code signature remains 100% valid afterwards!
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fn set_tree_writable(root: &Path, writable: bool) {
            let mut dirs = vec![root.to_path_buf()];
            let mut all_dirs = Vec::new();
            while let Some(d) = dirs.pop() {
                if writable {
                    let _ = fs::set_permissions(&d, fs::Permissions::from_mode(0o755));
                }
                all_dirs.push(d.clone());
                if let Ok(rd) = fs::read_dir(&d) {
                    for e in rd.flatten() {
                        let Ok(ft) = e.file_type() else { continue };
                        let p = e.path();
                        if ft.is_dir() {
                            dirs.push(p);
                        } else if ft.is_file() {
                            let mode = if writable { 0o644 } else { 0o444 };
                            let _ = fs::set_permissions(&p, fs::Permissions::from_mode(mode));
                        }
                    }
                }
            }
            if !writable {
                for d in all_dirs.into_iter().rev() {
                    let _ = fs::set_permissions(&d, fs::Permissions::from_mode(0o555));
                }
            }
        }

        struct RestoreWritableGuard(PathBuf);
        impl Drop for RestoreWritableGuard {
            fn drop(&mut self) {
                set_tree_writable(&self.0, true);
            }
        }

        // Re-sign the bundle after Step C1 populated `Contents/Resources/AotCache/*.aot`.
        #[cfg(target_os = "macos")]
        {
            let resign = Command::new("codesign")
                .args(["--force", "--sign", "-", &app_dir.to_string_lossy()])
                .output()
                .expect("re-sign .app bundle with prewarmed AotCache");
            assert!(resign.status.success(), "codesign failed: {}", String::from_utf8_lossy(&resign.stderr));
        }

        let new_mac_home = tmp.0.join("new_mac_user_home");
        fs::create_dir_all(&new_mac_home).unwrap();
        let aot_entries_before: Vec<PathBuf> = fs::read_dir(&aot_cache_dir)
            .unwrap()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();

        let _ro_guard = RestoreWritableGuard(resources_dir.clone());
        set_tree_writable(&resources_dir, false);

        let ro_call = Command::new(&launcher_bin)
            .args(["--call-export", "add_and_tag", "--drive-c"])
            .arg(new_mac_home.join("Translator"))
            .args(["--", "100", "23"])
            .env_clear()
            .env("HOME", &new_mac_home)
            .output()
            .expect("launch read-only .app with fresh user HOME");
        let ro_stdout = String::from_utf8_lossy(&ro_call.stdout);
        let ro_stderr = String::from_utf8_lossy(&ro_call.stderr);
        assert!(
            ro_call.status.success(),
            "read-only .app launch failed (status {:?}):\nstdout:\n{ro_stdout}\nstderr:\n{ro_stderr}",
            ro_call.status
        );
        assert!(
            ro_stdout.contains("returned 0x82"),
            "expected add_and_tag(100, 23) == 130 (0x82) on read-only .app, stdout:\n{ro_stdout}\nstderr:\n{ro_stderr}"
        );
        assert!(
            ro_stderr.contains("loading cache from read-only fallback")
                && ro_stderr.contains("Contents/Resources/AotCache"),
            "expected port-runner to load bundled .aot from read-only fallback, stderr:\n{ro_stderr}"
        );
        let aot_entries_after: Vec<PathBuf> = fs::read_dir(&aot_cache_dir)
            .unwrap()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        assert_eq!(aot_entries_before, aot_entries_after, "read-only Contents/Resources/AotCache must stay untouched");

        #[cfg(target_os = "macos")]
        {
            let verify_after = Command::new("codesign")
                .args(["--verify", "--deep", "--strict", &app_dir.to_string_lossy()])
                .output()
                .expect("verify .app bundle signature after read-only launch");
            assert!(
                verify_after.status.success(),
                "codesign --verify failed after read-only launch: {}",
                String::from_utf8_lossy(&verify_after.stderr)
            );

            // Also run the full `verify_app_bundle` SSD-portability audit when all real
            // components (`dxil-metald` + `libmetalirconverter.dylib`) were staged!
            if real_shaderd.is_some() && has_real_msc {
                let report = verify_app_bundle(&app_dir).expect("verify_app_bundle");
                assert!(
                    report.passed(),
                    "verify_app_bundle failed on ColdTest.app:\n{}",
                    format_bundle_verify_report(&report)
                );
            }
        }
    }
}
