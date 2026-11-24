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
