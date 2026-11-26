//! End-to-end CLI integration tests for:
//! 1. `win32-audit` honest `impl_kind` counts (`real`, `partial`, `stub`, `failing`) and stub listing
//! 2. Single-pass `audit-game <dir> --json --init-manifest` against synthetic PE fixtures

use port_factory::json_output::{exit_codes, parse_json, JsonValue, SCHEMA_VERSION};
use port_manifest::GameManifest;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

fn unique_temp_dir(tag: &str) -> PathBuf {
    let n = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let p = std::env::temp_dir().join(format!(
        "pf_audit_game_cli_{tag}_{}_{}",
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

fn put_u16(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
}
fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}
fn put_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

/// Build a valid synthetic x86-64 PE32+ executable or DLL with custom `.text` machine code
/// and a real `IMAGE_DIRECTORY_ENTRY_IMPORT` table.
fn build_synthetic_pe(
    is_dll: bool,
    code: &[u8],
    imports: &[(&str, &[&str])],
    rwx_text: bool,
) -> Vec<u8> {
    let text_raw_size = code.len().max(1).div_ceil(0x200) * 0x200;
    let text_rva: u32 = 0x1000;
    let text_raw_off: usize = 0x200;
    let rdata_rva: u32 = text_rva + ((text_raw_size as u32).div_ceil(0x1000) * 0x1000);
    let rdata_raw_off: usize = text_raw_off + text_raw_size;

    let mut rdata = Vec::<u8>::new();
    let import_dir_size: u32;
    if !imports.is_empty() {
        let desc_bytes = (imports.len() + 1) * 20;
        rdata.resize(desc_bytes, 0);
        import_dir_size = desc_bytes as u32;

        for (idx, (dll, syms)) in imports.iter().enumerate() {
            while rdata.len() % 8 != 0 {
                rdata.push(0);
            }
            let int_rva = rdata_rva + rdata.len() as u32;
            let int_off = rdata.len();
            rdata.resize(int_off + (syms.len() + 1) * 8, 0);

            let iat_rva = rdata_rva + rdata.len() as u32;
            let iat_off = rdata.len();
            rdata.resize(iat_off + (syms.len() + 1) * 8, 0);

            let name_rva = rdata_rva + rdata.len() as u32;
            rdata.extend_from_slice(dll.as_bytes());
            rdata.push(0);

            for (s_idx, sym) in syms.iter().enumerate() {
                if rdata.len() % 2 != 0 {
                    rdata.push(0);
                }
                let ibn_rva = rdata_rva + rdata.len() as u32;
                rdata.extend_from_slice(&0u16.to_le_bytes());
                rdata.extend_from_slice(sym.as_bytes());
                rdata.push(0);

                put_u64(&mut rdata, int_off + s_idx * 8, ibn_rva as u64);
                put_u64(&mut rdata, iat_off + s_idx * 8, ibn_rva as u64);
            }

            let d = idx * 20;
            put_u32(&mut rdata, d, int_rva);
            put_u32(&mut rdata, d + 12, name_rva);
            put_u32(&mut rdata, d + 16, iat_rva);
        }
    } else {
        import_dir_size = 0;
    }

    let rdata_raw_size = rdata.len().max(0x200).div_ceil(0x200) * 0x200;
    let rdata_vsize_aligned = (rdata_raw_size as u32).div_ceil(0x1000) * 0x1000;
    let size_of_image = rdata_rva + rdata_vsize_aligned;
    let total_file_size = rdata_raw_off + rdata_raw_size;

    let mut img = vec![0u8; total_file_size];
    img[0..2].copy_from_slice(b"MZ");
    let pe = 0x80usize;
    put_u32(&mut img, 0x3C, pe as u32);
    put_u32(&mut img, pe, 0x0000_4550);
    put_u16(&mut img, pe + 4, 0x8664); // AMD64
    put_u16(&mut img, pe + 6, 2);
    put_u16(&mut img, pe + 20, 240);
    put_u16(&mut img, pe + 22, if is_dll { 0x2022 } else { 0x0022 });

    let opt = pe + 24;
    put_u16(&mut img, opt, 0x20B); // PE32+
    put_u32(&mut img, opt + 16, text_rva);
    put_u64(&mut img, opt + 24, 0x1_4000_0000);
    put_u32(&mut img, opt + 32, 0x1000);
    put_u32(&mut img, opt + 36, 0x200);
    put_u32(&mut img, opt + 56, size_of_image);
    put_u32(&mut img, opt + 60, 0x200);
    put_u16(&mut img, opt + 68, 2); // GUI
    put_u32(&mut img, opt + 108, 16);

    if import_dir_size > 0 {
        let imp_dd = opt + 112 + 8; // IMAGE_DIRECTORY_ENTRY_IMPORT = 1
        put_u32(&mut img, imp_dd, rdata_rva);
        put_u32(&mut img, imp_dd + 4, import_dir_size);
    }

    let sec0 = opt + 240;
    img[sec0..sec0 + 5].copy_from_slice(b".text");
    put_u32(&mut img, sec0 + 8, code.len().max(1) as u32);
    put_u32(&mut img, sec0 + 12, text_rva);
    put_u32(&mut img, sec0 + 16, text_raw_size as u32);
    put_u32(&mut img, sec0 + 20, text_raw_off as u32);
    let mut text_chars = 0x6000_0020u32; // CODE | EXEC | READ
    if rwx_text {
        text_chars |= 0x8000_0000; // WRITE
    }
    put_u32(&mut img, sec0 + 36, text_chars);

    let sec1 = sec0 + 40;
    img[sec1..sec1 + 6].copy_from_slice(b".rdata");
    put_u32(&mut img, sec1 + 8, rdata.len().max(1) as u32);
    put_u32(&mut img, sec1 + 12, rdata_rva);
    put_u32(&mut img, sec1 + 16, rdata_raw_size as u32);
    put_u32(&mut img, sec1 + 20, rdata_raw_off as u32);
    put_u32(&mut img, sec1 + 36, 0x4000_0040); // INITIALIZED_DATA | READ

    img[text_raw_off..text_raw_off + code.len()].copy_from_slice(code);
    img[rdata_raw_off..rdata_raw_off + rdata.len()].copy_from_slice(&rdata);
    img
}

fn build_synthetic_win32_workspace(ws_root: &std::path::Path) {
    let src = ws_root.join("crates/win32-rt/src");
    for sub in ["kernel32", "d3d12", "misc"] {
        std::fs::create_dir_all(src.join(sub)).unwrap();
    }
    std::fs::write(
        src.join("kernel32/file.rs"),
        r#"
pub unsafe extern "win64" fn CreateFileW() -> usize { 1 }
pub unsafe extern "win64" fn CloseHandle() -> i32 { 1 }
"#,
    )
    .unwrap();
    std::fs::write(
        src.join("ntdll.rs"),
        r#"
pub unsafe extern "win64" fn NtQueryInformationProcess(class: u32) -> i32 {
    match class {
        0 => 0,
        _ => {
            stub!("NtQueryInformationProcess({class})");
            -1
        }
    }
}
"#,
    )
    .unwrap();
    std::fs::write(
        src.join("advapi32.rs"),
        r#"
pub unsafe extern "win64" fn CreateServiceW() -> usize { 1 }

// @exports SynthFailApi
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    SynthFailApi);
"#,
    )
    .unwrap();
    std::fs::write(
        src.join("d3d12/mod.rs"),
        r#"
pub static EXPORTS: &[crate::Export] = exports![D3D12CreateDevice];
pub unsafe extern "win64" fn D3D12CreateDevice() -> i32 { 0 }
"#,
    )
    .unwrap();
    std::fs::write(
        src.join("misc/zstd.rs"),
        r#"
pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];
pub extern "win64" fn SynthZstdStub() -> usize {
    crate::stub!("zstd!SynthZstdStub");
    usize::MAX
}
"#,
    )
    .unwrap();
}

fn run_pf_json(ws: &std::path::Path, args: &[&str]) -> (i32, JsonValue) {
    let out = Command::new(env!("CARGO_BIN_EXE_port-factory"))
        .env("PORT_FACTORY_WORKSPACE", ws)
        .args(args)
        .output()
        .expect("spawn port-factory");
    let code = out.status.code().expect("exit code");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let parsed = parse_json(stdout.trim()).unwrap_or_else(|e| {
        panic!("invalid JSON for {args:?} (exit {code}): {e}\nstdout:\n{stdout}\nstderr:\n{stderr}")
    });
    assert_eq!(
        parsed.get("schema_version").and_then(JsonValue::as_u64),
        Some(SCHEMA_VERSION as u64)
    );
    assert_eq!(
        parsed.get("exit_code").and_then(JsonValue::as_i64),
        Some(code as i64)
    );
    assert_eq!(
        parsed.get("ok").and_then(JsonValue::as_bool),
        Some(code == exit_codes::OK)
    );
    (code, parsed)
}

#[test]
fn win32_audit_and_audit_game_cli_work_end_to_end_on_synthetic_pe_fixtures() {
    let root = unique_temp_dir("e2e");
    let fake_ws = root.join("fake_ws");
    build_synthetic_win32_workspace(&fake_ws);
    let game_dir = root.join("NebulaDrifter");
    std::fs::create_dir_all(&game_dir).unwrap();

    // Game EXE importing controlled synthetic real, partial, stub, and failing Win32 symbols + D3D12 (0 unresolved).
    let code: &[u8] = &[
        0x48, 0x31, 0xC0, // xor rax, rax
        0x48, 0x01, 0xC8, // add rax, rcx
        0x0F, 0xA2, // cpuid
        0x0F, 0x31, // rdtsc
        0xC3, // ret
    ];
    let exe_bytes = build_synthetic_pe(
        false,
        code,
        &[
            ("KERNEL32.dll", &["CreateFileW", "CloseHandle"]),
            ("d3d12.dll", &["D3D12CreateDevice"]),
            ("ntdll.dll", &["NtQueryInformationProcess"]),
            ("zstd.dll", &["SynthZstdStub"]),
            ("ADVAPI32.dll", &["SynthFailApi"]),
        ],
        false,
    );
    std::fs::write(game_dir.join("NebulaDrifter.exe"), &exe_bytes).unwrap();
    std::fs::write(game_dir.join("steam_appid.txt"), "777001\n").unwrap();

    // 1. `win32-audit` human output reports impl_kind breakdown and lists stub/failing imports.
    let human_out = Command::new(env!("CARGO_BIN_EXE_port-factory"))
        .env("PORT_FACTORY_WORKSPACE", &fake_ws)
        .args(["win32-audit", game_dir.to_str().unwrap()])
        .output()
        .expect("run win32-audit");
    assert_eq!(human_out.status.code(), Some(exit_codes::OK));
    let human_stdout = String::from_utf8_lossy(&human_out.stdout);
    assert!(
        human_stdout.contains("6/6 resolved = 100.00%, 3/6 real = 50.00%"),
        "expected honest real vs resolved header, got:\n{human_stdout}"
    );
    assert!(human_stdout.contains("Stubbed / Failing Imports (2):"));
    assert!(human_stdout.contains("zstd.dll!SynthZstdStub [stub]"));
    assert!(human_stdout.contains("advapi32.dll!SynthFailApi [failing]"));

    // 2. `win32-audit --json` emits impl_kind counts & stubbed_imports list.
    let (code, val) = run_pf_json(&fake_ws, &["win32-audit", game_dir.to_str().unwrap(), "--json"]);
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("win32-audit")
    );
    let data = val.get("data").unwrap();
    let by_kind = data.get("resolved_by_impl_kind").unwrap();
    assert_eq!(by_kind.get("real").and_then(JsonValue::as_u64), Some(3));
    assert_eq!(by_kind.get("partial").and_then(JsonValue::as_u64), Some(1));
    assert_eq!(by_kind.get("stub").and_then(JsonValue::as_u64), Some(1));
    assert_eq!(by_kind.get("failing").and_then(JsonValue::as_u64), Some(1));
    assert_eq!(
        data.get("stubbed_imports_count").and_then(JsonValue::as_u64),
        Some(2)
    );
    let stubbed = data
        .get("stubbed_imports")
        .and_then(JsonValue::as_array)
        .unwrap();
    assert_eq!(stubbed.len(), 2);

    // 3. `audit-game <dir> --json --init-manifest <out_toml>` does one pass & writes game.toml.
    let manifest_out = root.join("generated/nebula-drifter/game.toml");
    let (code, val) = run_pf_json(
        &fake_ws,
        &[
            "audit-game",
            game_dir.to_str().unwrap(),
            "--json",
            "--init-manifest",
            manifest_out.to_str().unwrap(),
        ],
    );
    assert_eq!(code, exit_codes::OK);
    assert_eq!(
        val.get("command").and_then(JsonValue::as_str),
        Some("audit-game")
    );
    let data = val.get("data").unwrap();
    assert_eq!(
        data.get("binaries_scanned").and_then(JsonValue::as_u64),
        Some(1)
    );
    assert_eq!(
        data.get("pe_parse_count").and_then(JsonValue::as_u64),
        Some(1)
    );
    assert_eq!(
        data.get("main_exe").and_then(JsonValue::as_str),
        Some("NebulaDrifter.exe")
    );
    assert_eq!(
        data.get("readiness").and_then(JsonValue::as_str),
        Some("NEEDS_WORK")
    );
    let imp_by_kind = data.get("imports").unwrap().get("by_impl_kind").unwrap();
    assert_eq!(imp_by_kind.get("real").and_then(JsonValue::as_u64), Some(3));
    assert_eq!(
        imp_by_kind.get("partial").and_then(JsonValue::as_u64),
        Some(1)
    );
    assert_eq!(imp_by_kind.get("stub").and_then(JsonValue::as_u64), Some(1));
    assert_eq!(
        imp_by_kind.get("failing").and_then(JsonValue::as_u64),
        Some(1)
    );
    let apis = data
        .get("graphics")
        .unwrap()
        .get("detected_apis")
        .and_then(JsonValue::as_array)
        .unwrap();
    assert!(apis.iter().any(|v| v.as_str() == Some("D3D12")));
    assert!(
        data.get("aot_coverage")
            .unwrap()
            .get("total_instructions")
            .and_then(JsonValue::as_u64)
            .unwrap()
            >= 5
    );
    assert!(
        data.get("anti_tamper")
            .unwrap()
            .get("cpuid_count")
            .and_then(JsonValue::as_u64)
            .unwrap()
            >= 1
    );

    // Pre-filled game.toml exists and round-trips cleanly.
    assert!(manifest_out.is_file());
    let parsed_manifest = GameManifest::load_file(&manifest_out).unwrap();
    assert_eq!(parsed_manifest.id.as_deref(), Some("nebula-drifter"));
    assert_eq!(parsed_manifest.exe.as_deref(), Some("NebulaDrifter.exe"));
    assert_eq!(parsed_manifest.steam_app_id.as_deref(), Some("777001"));
    assert!(parsed_manifest.middleware.contains_key("zstd"));

    // 4. Unresolved import -> exit 3 (UNRESOLVED_IMPORTS).
    let unresolved_dir = root.join("UnresolvedGame");
    std::fs::create_dir_all(&unresolved_dir).unwrap();
    let unresolved_exe = build_synthetic_pe(
        false,
        &[0x31, 0xC0, 0xC3],
        &[
            ("KERNEL32.dll", &["CreateFileW"]),
            ("mystery_dep.dll", &["MissingApiCall"]),
        ],
        false,
    );
    std::fs::write(unresolved_dir.join("Game.exe"), &unresolved_exe).unwrap();
    let (code, val) = run_pf_json(
        &fake_ws,
        &["audit-game", unresolved_dir.to_str().unwrap(), "--json"],
    );
    assert_eq!(code, exit_codes::UNRESOLVED_IMPORTS);
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("imports")
            .unwrap()
            .get("unresolved_count")
            .and_then(JsonValue::as_u64),
        Some(1)
    );

    // 5. Kernel anti-cheat driver loader -> exit 5 (INCOMPATIBLE).
    let kernel_dir = root.join("KernelBlockedGame");
    std::fs::create_dir_all(&kernel_dir).unwrap();
    let kernel_exe = build_synthetic_pe(
        false,
        &[0x31, 0xC0, 0xC3],
        &[("ADVAPI32.dll", &["CreateServiceW"])],
        false,
    );
    std::fs::write(kernel_dir.join("AcGame.exe"), &kernel_exe).unwrap();
    let (code, val) = run_pf_json(
        &fake_ws,
        &["audit-game", kernel_dir.to_str().unwrap(), "--json"],
    );
    assert_eq!(code, exit_codes::INCOMPATIBLE);
    assert_eq!(
        val.get("data")
            .unwrap()
            .get("readiness")
            .and_then(JsonValue::as_str),
        Some("BLOCKED")
    );

    // 6. Missing directory -> exit 2 (USAGE_ERROR) with JSON error envelope.
    let (code, val) = run_pf_json(
        &fake_ws,
        &[
            "audit-game",
            root.join("does_not_exist").to_str().unwrap(),
            "--json",
        ],
    );
    assert_eq!(code, exit_codes::USAGE_ERROR);
    assert!(val.get("error").and_then(JsonValue::as_str).is_some());

    let _ = std::fs::remove_dir_all(&root);
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[test]
fn readiness_uses_real_dispatch_and_classifies_every_import() {
    let tmp = port_factory::bundle_common::TempDirGuard::new(unique_temp_dir("readiness"));
    let db = tmp.path().join("db");
    let game = tmp.path().join("game");
    std::fs::create_dir_all(&db).unwrap();
    std::fs::create_dir_all(game.join("plugins")).unwrap();
    let mut apis =
        String::from("dll\tsymbol\tordinal\tcategory\timpl_kind\trust_path\tsource_file\n");
    for kind in ["real", "partial", "stub", "failing"] {
        apis.push_str(&format!(
            "kernel32.dll\t{kind}\t-\tCORE_OS\t{kind}\twin32_rt::api::{kind}\tapi.rs\n"
        ));
    }
    std::fs::write(db.join("win32_apis.tsv"), apis).unwrap();
    std::fs::write(db.join("stub_inventory.tsv"),
        "dll\tapi\tkind\tfile\tline\tdetail\nkernel32.dll\tpartial\tstub\tapi.rs\t42\tconditional fallback\n").unwrap();
    let instructions = [
        0x48, 0x8b, 0x00, // mov rax,[rax] lowers
        0x64, 0x48, 0x8b, 0x00, // same Code, FS address falls back
        0xd9, 0xe8, // fld1 lowers to native ARM64 x87
        0x0f, 0x2a, 0xc0, // cvtpi2ps xmm0,mm0 lowers to native ARM64 MMX
        0x0f, 0xa4, 0x08, 0x04, // shld dword ptr [rax],ecx,4: generic interpreter-only
        0x0f, 0x01, 0xee, // rdpkru, exit 86 if reached
        0xc4, 0xe2, 0x71, 0x90, 0x04, 0x90, // vpgatherdd: native ARM64 AVX2 gather
        0xcc, 0x0f, 0x0b, 0xc3, // guest traps and ret
    ];
    let image = build_synthetic_pe(
        false,
        &instructions,
        &[(
            "API-MS-WIN-CORE-SYNCH-L1-2-0.DLL",
            &["real", "partial", "stub", "failing", "absent"],
        )],
        false,
    );
    std::fs::write(game.join("play.exe"), &image).unwrap();
    // Keep a leaf outside .pdata in a nonstandard-extension plugin.
    // A padding byte would decode as the start of a two-byte push and swallow
    // the first byte of the exported leaf if the scan ignored its export RVA.
    let mut leaf = build_synthetic_pe(true, &[0xc3, 0x6a, 0x0f, 0x01, 0xee, 0xc3], &[], false);
    let exports = 0x80 + 24 + 112;
    put_u32(&mut leaf, exports, 0x2040);
    put_u32(&mut leaf, exports + 4, 0x60);
    put_u32(&mut leaf, 0x440 + 16, 1);
    put_u32(&mut leaf, 0x440 + 20, 1);
    put_u32(&mut leaf, 0x440 + 28, 0x2080);
    put_u32(&mut leaf, 0x480, 0x1002);
    let exception = 0x80 + 24 + 112 + 3 * 8;
    put_u32(&mut leaf, exception, 0x2000);
    put_u32(&mut leaf, exception + 4, 12);
    put_u32(&mut leaf, 0x400, 0x1000);
    put_u32(&mut leaf, 0x404, 0x1001);
    std::fs::write(game.join("plugins/extra"), leaf).unwrap();
    let (code, env) = run_pf_json(tmp.path(), &["readiness", game.to_str().unwrap(), "--json"]);
    assert_eq!(code, exit_codes::INCOMPATIBLE);
    let data = env.get("data").unwrap();
    let bins = data.get("binaries").unwrap().as_array().unwrap();
    assert_eq!(bins.len(), 2);
    let ins = bins[0].get("instructions").unwrap().as_array().unwrap();
    let op = |mn| {
        ins.iter()
            .find(|x| x.get("mnemonic").unwrap().as_str() == Some(mn))
            .unwrap()
    };
    assert_eq!(op("mov").get("count").unwrap().as_u64(), Some(1));
    assert_eq!(op("mov").get("first_rva").unwrap().as_u64(), Some(0x1003));
    assert!(
        !ins.iter()
            .any(|x| x.get("mnemonic").unwrap().as_str() == Some("fld1")),
        "fld1 now lowers to native ARM64"
    );
    assert!(
        !ins.iter()
            .any(|x| x.get("mnemonic").unwrap().as_str() == Some("cvtpi2ps")),
        "cvtpi2ps now lowers to native ARM64"
    );
    assert_eq!(
        op("shld").get("kind").unwrap().as_str(),
        Some("interpreter_only")
    );
    assert!(op("shld")
        .get("advertised_but_unimplemented")
        .unwrap()
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        op("rdpkru").get("kind").unwrap().as_str(),
        Some("unimplemented")
    );
    assert!(
        ins.iter()
            .all(|x| x.get("mnemonic").and_then(|v| v.as_str()) != Some("vpgatherdd")),
        "vpgatherdd is lowered to native ARM64"
    );
    assert_eq!(
        op("int3").get("kind").unwrap().as_str(),
        Some("guest_boundary")
    );
    assert_eq!(
        op("ud2").get("kind").unwrap().as_str(),
        Some("guest_boundary")
    );
    assert_eq!(ins.len(), 5);
    let imps = bins[0].get("imports").unwrap().as_array().unwrap();
    assert_eq!(
        imps.iter()
            .map(|x| x.get("kind").unwrap().as_str().unwrap())
            .collect::<Vec<_>>(),
        ["real", "partial", "stub", "failing", "missing"]
    );
    assert_eq!(
        imps[1].get("stub_sites").unwrap().as_array().unwrap()[0]
            .get("line")
            .unwrap()
            .as_u64(),
        Some(42)
    );
    let top = &data.get("ranked_findings").unwrap().as_array().unwrap()[0];
    assert_eq!(top.get("kind").unwrap().as_str(), Some("unimplemented"));
    assert_eq!(top.get("sites").unwrap().as_u64(), Some(2));
    assert_eq!(top.get("images").unwrap().as_array().unwrap().len(), 2);

    std::fs::write(game.join("broken.dll"), b"MZ").unwrap();
    let (code, report) = run_pf_json(tmp.path(), &["readiness", game.to_str().unwrap(), "--json"]);
    assert_eq!(code, exit_codes::RUNTIME_ERROR);
    assert_eq!(
        report
            .get("data")
            .unwrap()
            .get("unscanned")
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let (code, report) = run_pf_json(
        tmp.path(),
        &[
            "readiness",
            game.join("play.exe").to_str().unwrap(),
            "--json",
        ],
    );
    assert_eq!(code, exit_codes::INCOMPATIBLE);
    assert_eq!(
        report
            .get("data")
            .unwrap()
            .get("binaries")
            .unwrap()
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
#[test]
fn readiness_resolves_shipped_and_forwarded_delay_ordinals() {
    let tmp = port_factory::bundle_common::TempDirGuard::create_unique("readiness-delay").unwrap();
    let db = tmp.path().join("db");
    let game = tmp.path().join("game");
    std::fs::create_dir_all(&db).unwrap();
    std::fs::create_dir_all(&game).unwrap();
    std::fs::write(db.join("win32_apis.tsv"), "dll\tsymbol\tordinal\tcategory\timpl_kind\trust_path\tsource_file\nkernel32.dll\tExample\t17\tCORE_OS\treal\twin32_rt::api::Example\tapi.rs\n").unwrap();
    std::fs::write(
        db.join("stub_inventory.tsv"),
        "dll\tapi\tkind\tfile\tline\tdetail\n",
    )
    .unwrap();

    let dirs = 0x80 + 24 + 112;
    let raw = 0x400;
    let mut caller = build_synthetic_pe(false, &[0xc3], &[], false);
    put_u32(&mut caller, dirs + 13 * 8, 0x2000);
    put_u32(&mut caller, dirs + 13 * 8 + 4, 64);
    // Delay descriptor, INT with three ordinals, then the DLL name.
    put_u32(&mut caller, raw, 1);
    put_u32(&mut caller, raw + 4, 0x2090);
    put_u32(&mut caller, raw + 12, 0x2070);
    put_u32(&mut caller, raw + 16, 0x2040);
    for i in 0..3 {
        put_u64(
            &mut caller,
            raw + 0x40 + i * 8,
            (1u64 << 63) | (7 + i as u64),
        );
    }
    caller[raw + 0x90..raw + 0x99].copy_from_slice(b"unit.dll\0");
    let exe = game.join("play.exe");
    std::fs::write(&exe, caller).unwrap();

    let mut dll = build_synthetic_pe(true, &[0xc3], &[], false);
    put_u32(&mut dll, dirs, 0x2000);
    put_u32(&mut dll, dirs + 4, 0x80);
    put_u32(&mut dll, raw + 16, 7); // ordinal base
    put_u32(&mut dll, raw + 20, 4); // #7 direct, #8 forwarded, #9 absent, #10 unused
    put_u32(&mut dll, raw + 28, 0x2040);
    put_u32(&mut dll, raw + 0x40, 0x1000); // direct .text export
    put_u32(&mut dll, raw + 0x44, 0x2050); // forwarder in export directory
    put_u32(&mut dll, raw + 0x4c, 0x2060);
    dll[raw + 0x50..raw + 0x58].copy_from_slice(b"link.#7\0");
    dll[raw + 0x60..raw + 0x6a].copy_from_slice(b"unused.#1\0");
    let mut link = dll.clone();
    put_u32(&mut link, raw + 20, 1);
    put_u32(&mut link, raw + 0x40, 0x2050);
    link[raw + 0x50..raw + 0x5d].copy_from_slice(b"KERNEL32.#17\0");
    std::fs::write(game.join("Unit.DLL"), dll).unwrap();
    std::fs::write(game.join("link.dll"), link).unwrap();
    std::fs::write(
        game.join("unused.dll"),
        build_synthetic_pe(true, &[0x0f, 0x01, 0xee], &[], false),
    )
    .unwrap();
    let (code, report) = run_pf_json(tmp.path(), &["readiness", exe.to_str().unwrap(), "--json"]);
    assert_eq!(code, exit_codes::UNRESOLVED_IMPORTS);
    let binaries = report
        .get("data")
        .unwrap()
        .get("binaries")
        .unwrap()
        .as_array()
        .unwrap();
    let paths = binaries
        .iter()
        .map(|b| b.get("path").unwrap().as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(paths, ["Unit.DLL", "link.dll", "play.exe"]);
    let play = binaries
        .iter()
        .find(|b| b.get("path").unwrap().as_str() == Some("play.exe"))
        .unwrap();
    let imports = play.get("imports").unwrap().as_array().unwrap();
    assert!(imports
        .iter()
        .all(|i| i.get("delay").unwrap().as_bool() == Some(true)));
    assert_eq!(
        imports
            .iter()
            .map(|x| x.get("kind").unwrap().as_str().unwrap())
            .collect::<Vec<_>>(),
        ["shipped", "real", "missing"]
    );
    assert_eq!(imports[1].get("symbol").unwrap().as_str(), Some("#8"));
    assert!(imports[1]
        .get("provider")
        .unwrap()
        .as_str()
        .unwrap()
        .contains("kernel32.dll!Example"));
}
