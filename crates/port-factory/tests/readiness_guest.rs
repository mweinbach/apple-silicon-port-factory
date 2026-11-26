//! CI supplies Zig. Cargo supplies both executable paths. Only these test-owned guests execute.
#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use port_factory::bench::{aot_live_block_count, build_host_port_runner, WORKLOADS};
use port_factory::bundle_common::TempDirGuard;
use port_factory::json_output::{parse_json, JsonValue};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn succeeded(out: Output) {
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn compile(dir: &Path, name: &str, source: &str, flags: &[&str]) {
    let file = dir.join(format!("{name}.c"));
    fs::write(&file, source).unwrap();
    succeeded(
        Command::new(std::env::var_os("ZIG").unwrap_or_else(|| "zig".into()))
            .args(["cc", "-target", "x86_64-windows-gnu", "-O2"])
            .args(flags)
            .arg(file)
            .arg("-lkernel32")
            .arg("-o")
            .arg(dir.join(format!("{name}.exe")))
            .env("ZIG_LOCAL_CACHE_DIR", dir.join("zig-cache"))
            .output()
            .expect("Zig required"),
    );
}

fn report(exe: &Path) -> (i32, JsonValue) {
    let out = Command::new(env!("CARGO_BIN_EXE_port-factory"))
        .arg("readiness")
        .arg(exe)
        .arg("--json")
        .output()
        .unwrap();
    let data = parse_json(std::str::from_utf8(&out.stdout).unwrap()).unwrap();
    (
        out.status.code().unwrap(),
        data.get("data").expect("readiness report").clone(),
    )
}

#[test]
#[ignore = "requires Zig; exercised by CI"]
fn readiness_matches_the_guest_in_live_jit_and_prewarmed_aot() {
    let tmp = TempDirGuard::create_unique("readiness-guest").unwrap();
    let dir = tmp.path();
    // The existing SIMD benchmark exercises a realistically linked Windows PE.
    let spec = WORKLOADS.iter().find(|w| w.id == "simd_kernel").unwrap();
    compile(dir, spec.id, spec.source, spec.extra_cflags);
    let (_, data) = report(&dir.join("simd_kernel.exe"));
    let binary = &data.get("binaries").unwrap().as_array().unwrap()[0];
    assert!(binary.get("decoded").unwrap().as_u64().unwrap() > 100);
    assert!(binary
        .get("imports")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .any(
            |imp| imp.get("symbol").unwrap().as_str() == Some("QueryPerformanceCounter")
                && imp.get("kind").unwrap().as_str() == Some("real")
        ));

    let runner = build_host_port_runner(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    for abort in [false, true] {
        let name = if abort { "unsupported" } else { "x87" };
        let mut flags = vec![
            "-nostdlib",
            "-funwind-tables",
            "-fno-omit-frame-pointer",
            "-Wl,-e,mainCRTStartup",
        ];
        if x86_runtime::simd_ops::cpuid_x64(7, 0).1 & (1 << 5) != 0 {
            flags.push("-DEXPECT_AVX2");
        }
        if abort {
            flags.push("-DEXERCISE_ABORT");
        }
        compile(
            dir,
            name,
            r#"
__declspec(dllimport) void ExitProcess(unsigned);
__attribute__((noinline)) static double one(void) {
    double result;
    __asm__ volatile ("fld1; fstpl %0" : "=m"(result));
    return result;
}
void mainCRTStartup(void) {
    unsigned ebx;
    unsigned eax = 7, ecx = 0, edx;
    __asm__ volatile ("cpuid" : "+a"(eax), "=b"(ebx), "+c"(ecx), "=d"(edx));
#ifdef EXPECT_AVX2
    if (!(ebx & (1u << 5))) ExitProcess(41);
#else
    if (ebx & (1u << 5)) ExitProcess(41);
#endif
    if (one() != 1.0) ExitProcess(42);
    unsigned converted;
    __asm__ volatile ("movd %1, %%mm0; cvtpi2ps %%mm0, %%xmm0; movd %%xmm0, %0; emms"
        : "=r"(converted) : "r"(2) : "mm0", "xmm0");
    if (converted != 0x40000000u) ExitProcess(43); // float32 2.0
    unsigned shld_mem = 0x12345678u;
    __asm__ volatile ("shldl $4, %1, %0" : "+m"(shld_mem) : "r"(0x9ABCDEF0u) : "cc");
    if (shld_mem != 0x23456789u) ExitProcess(44);
#ifdef EXERCISE_ABORT
    __asm__ volatile ("rdpkru" : : "c"(0) : "eax", "edx");
#endif
    ExitProcess(0);
}
"#,
            &flags,
        );
        let exe = dir.join(format!("{name}.exe"));
        let (code, data) = report(&exe);
        assert_eq!(code, if abort { 5 } else { 0 }, "{data:?}");
        let ops = data.get("binaries").unwrap().as_array().unwrap()[0]
            .get("instructions")
            .unwrap()
            .as_array()
            .unwrap();
        let reports = |mn, kind| {
            ops.iter().any(|op| {
                op.get("mnemonic").unwrap().as_str() == Some(mn)
                    && op.get("kind").unwrap().as_str() == Some(kind)
            })
        };
        assert!(!reports("fld1", "interpreter_only"));
        assert!(!reports("fld1", "unimplemented"));
        assert!(!ops
            .iter()
            .any(|op| op.get("mnemonic").and_then(|v| v.as_str()) == Some("cvtpi2ps")));
        assert!(reports("shld", "interpreter_only"));
        assert_eq!(reports("rdpkru", "unimplemented"), abort);
        let aot = dir.join(format!("{name}-aot"));
        for mode in ["jit", "prewarm", "aot"] {
            let logs = dir.join(format!("{name}-{mode}"));
            let mut cmd = Command::new(&runner);
            cmd.arg("--pe")
                .arg(&exe)
                .arg("--timeout-secs")
                .arg("30")
                .env("PORT_HEADLESS", "1")
                .env("PORT_LOG_DIR", &logs)
                .env("AW2_DRIVE_C", dir.join("drive"))
                .env("TRANSLATOR_HOME", dir.join("drive"))
                .env("PORT_AOT_DIR", &aot)
                .env("RUST_LOG", "info");
            if mode == "prewarm" {
                cmd.arg("--prewarm-aot");
            }
            cmd.env("PORT_AOT", if mode == "jit" { "0" } else { "1" });
            let out = cmd.output().unwrap();
            let log = format!(
                "{}{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
                fs::read_to_string(logs.join("port-runner.log")).unwrap_or_default()
            );
            let exit = if abort && mode != "prewarm" { 86 } else { 0 };
            assert_eq!(out.status.code(), Some(exit), "{name}/{mode}: {log}");
            if mode == "aot" {
                assert!(aot_live_block_count(&log).is_some(), "{log}");
            }
            if exit == 86 {
                assert!(log.contains("UNIMPLEMENTED mnemonic Rdpkru"), "{log}");
            }
        }
    }
}
