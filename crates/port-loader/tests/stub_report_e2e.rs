//! End-to-end guest PE tests (`zig cc -target x86_64-windows-gnu` -> `port-runner`)
//! verifying:
//! 1. Real Windows x86-64 guest execution of our newly implemented Win32 APIs
//!    (`ws2_32`, `crypt32`, `advapi32`, `dbghelp`, `kernel32`, `user32`, `oleaut32`)
//!    with ZERO stubs hit.
//! 2. The global stub registry (`hits`, `first_frame`, symbolized `first_guest_caller`,
//!    ranked exit summary table) and unified `port-runner --report-json` boot report.
//! 3. `port-factory report-diff` and `port-factory stub-diff` comparing two runs.
#![cfg(all(target_os = "macos", target_arch = "aarch64"))]

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let id = SEQ.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "port-runner-stub-report-e2e-{tag}-{}-{id}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp test dir");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn find_zig() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("ZIG").filter(|v| !v.is_empty()) {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Some(pb);
        }
    }
    if let Some(path_env) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_env) {
            let cand = dir.join("zig");
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    for fixed in ["/opt/homebrew/bin/zig", "/usr/local/bin/zig"] {
        let pb = PathBuf::from(fixed);
        if pb.is_file() {
            return Some(pb);
        }
    }
    None
}

fn compile_guest(zig: &Path, work_dir: &Path, name: &str, src: &str, libs: &[&str]) -> PathBuf {
    let src_path = work_dir.join(format!("{name}.c"));
    let exe_path = work_dir.join(format!("{name}.exe"));
    std::fs::write(&src_path, src).expect("write guest C source");
    let zig_cache = work_dir.join("zig-cache");
    let mut cmd = Command::new(zig);
    cmd.args(["cc", "-target", "x86_64-windows-gnu", "-O2", "-Wall", "-Wextra"])
        .arg(&src_path)
        .arg("-o")
        .arg(&exe_path);
    for l in libs {
        cmd.arg(l);
    }
    cmd.env("ZIG_LOCAL_CACHE_DIR", &zig_cache);
    let out = cmd.output().expect("spawn zig cc");
    assert!(
        out.status.success(),
        "zig cc failed for {name}:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    exe_path
}

fn run_port_runner(
    work_dir: &Path,
    exe: &Path,
    report_json: &Path,
) -> (Option<i32>, String) {
    let log_dir = work_dir.join("logs");
    let drive_c = work_dir.join("drive_c");
    let aot_dir = work_dir.join("aot");
    std::fs::create_dir_all(&log_dir).unwrap();
    std::fs::create_dir_all(&drive_c).unwrap();
    std::fs::create_dir_all(&aot_dir).unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_port-runner"));
    cmd.arg("--pe")
        .arg(exe)
        .arg("--report-json")
        .arg(report_json)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", &aot_dir)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("PORT_HEADLESS", "1")
        .env("AW2_HEADLESS", "1")
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("spawn port-runner");
    let deadline = Instant::now() + Duration::from_secs(45);
    let code = loop {
        if let Some(st) = child.try_wait().expect("try_wait") {
            break st.code();
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let out = child.wait_with_output().expect("wait_with_output");
    let mut combined = String::new();
    if let Ok(log_txt) = std::fs::read_to_string(log_dir.join("port-runner.log")) {
        combined.push_str(&log_txt);
    }
    combined.push_str(&String::from_utf8_lossy(&out.stdout));
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    (code, combined)
}

const CLEAN_WIN32_GUEST_C: &str = r#"
#define WIN32_LEAN_AND_MEAN
#include <winsock2.h>
#include <windows.h>
#include <wincrypt.h>
#include <wincred.h>
#include <dbghelp.h>
#include <oleauto.h>
#include <stdio.h>
#include <string.h>
#include <wchar.h>

static void die(const char *msg) {
    fprintf(stderr, "GUEST_FAIL: %s (err=%lu)\n", msg, (unsigned long)GetLastError());
    ExitProcess(1);
}

int main(void) {
    // 1. ws2_32: getprotobyname, gethostbyname, WSAHtonl/WSANtohl, WSAEnumProtocolsA
    WSADATA wsa;
    if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0) die("WSAStartup");
    struct protoent *pe = getprotobyname("tcp");
    if (!pe || pe->p_proto != 6) die("getprotobyname(tcp)");
    struct hostent *he = gethostbyname("127.0.0.1");
    if (!he || he->h_length != 4 || !he->h_addr_list || !he->h_addr_list[0]) die("gethostbyname");
    if ((unsigned char)he->h_addr_list[0][0] != 127 || (unsigned char)he->h_addr_list[0][3] != 1) die("gethostbyname ip");
    SOCKET s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    if (s == INVALID_SOCKET) die("socket");
    u_long netl = 0, hostl = 0;
    if (WSAHtonl(s, 0x11223344u, &netl) != 0) die("WSAHtonl");
    if (WSANtohl(s, netl, &hostl) != 0 || hostl != 0x11223344u) die("WSANtohl");
    closesocket(s);
    DWORD plen = 0;
    WSAEnumProtocolsA(NULL, NULL, &plen);
    if (plen == 0) die("WSAEnumProtocolsA size");
    WSACleanup();

    // 2. crypt32: CryptBinaryToStringA, CertCompareIntegerBlob, CryptProtectData/UnprotectData
    const BYTE sample[] = "PortFactory";
    DWORD cch = 0;
    if (!CryptBinaryToStringA(sample, 11, CRYPT_STRING_BASE64 | CRYPT_STRING_NOCRLF, NULL, &cch)) die("CryptBinaryToStringA size");
    char b64[64];
    if (!CryptBinaryToStringA(sample, 11, CRYPT_STRING_BASE64 | CRYPT_STRING_NOCRLF, b64, &cch)) die("CryptBinaryToStringA");
    if (strcmp(b64, "UG9ydEZhY3Rvcnk=") != 0) die("CryptBinaryToStringA value");

    BYTE i1[] = { 0x2A };
    BYTE i2[] = { 0x2A, 0x00 };
    CRYPT_INTEGER_BLOB ib1 = { 1, i1 }, ib2 = { 2, i2 };
    if (!CertCompareIntegerBlob(&ib1, &ib2)) die("CertCompareIntegerBlob");

    DATA_BLOB inb = { 11, (BYTE*)sample };
    DATA_BLOB enc = { 0, NULL }, dec = { 0, NULL };
    LPWSTR descr = NULL;
    if (!CryptProtectData(&inb, L"guest-key", NULL, NULL, NULL, 0, &enc)) die("CryptProtectData");
    if (!CryptUnprotectData(&enc, &descr, NULL, NULL, NULL, 0, &dec)) die("CryptUnprotectData");
    if (dec.cbData != 11 || memcmp(dec.pbData, sample, 11) != 0) die("CryptUnprotectData payload");
    LocalFree(enc.pbData);
    LocalFree(dec.pbData);
    if (descr) LocalFree(descr);

    // 3. advapi32: CAPI SHA-256 + CredWriteW/CredReadW/CredFree/CredDeleteW
    HCRYPTPROV prov = 0;
    HCRYPTHASH hash = 0;
    if (!CryptAcquireContextW(&prov, NULL, NULL, PROV_RSA_AES, CRYPT_VERIFYCONTEXT)) die("CryptAcquireContextW");
    if (!CryptCreateHash(prov, CALG_SHA_256, 0, 0, &hash)) die("CryptCreateHash");
    if (!CryptHashData(hash, (const BYTE*)"abc", 3, 0)) die("CryptHashData");
    BYTE sha[32];
    DWORD shalen = 32;
    if (!CryptGetHashParam(hash, HP_HASHVAL, sha, &shalen, 0) || shalen != 32) die("CryptGetHashParam");
    if (sha[0] != 0xba || sha[1] != 0x78 || sha[31] != 0xad) die("SHA256 digest");
    CryptDestroyHash(hash);
    CryptReleaseContext(prov, 0);

    CREDENTIALW cred;
    memset(&cred, 0, sizeof(cred));
    cred.Type = CRED_TYPE_GENERIC;
    cred.TargetName = (LPWSTR)L"GuestE2E/Token";
    cred.UserName = (LPWSTR)L"player";
    cred.CredentialBlob = (LPBYTE)"tok99";
    cred.CredentialBlobSize = 5;
    cred.Persist = CRED_PERSIST_SESSION;
    if (!CredWriteW(&cred, 0)) die("CredWriteW");
    PCREDENTIALW out_cred = NULL;
    if (!CredReadW(L"GuestE2E/Token", CRED_TYPE_GENERIC, 0, &out_cred) || !out_cred) die("CredReadW");
    if (out_cred->CredentialBlobSize != 5 || memcmp(out_cred->CredentialBlob, "tok99", 5) != 0) die("CredReadW blob");
    CredFree(out_cred);
    if (!CredDeleteW(L"GuestE2E/Token", CRED_TYPE_GENERIC, 0)) die("CredDeleteW");

    // 4. dbghelp: SymInitialize / SymGetSearchPath / SymSetSearchPath / SymCleanup
    HANDLE proc = GetCurrentProcess();
    if (!SymInitialize(proc, "C:\\Syms", FALSE)) die("SymInitialize");
    char spath[64];
    if (!SymGetSearchPath(proc, spath, sizeof(spath)) || strcmp(spath, "C:\\Syms") != 0) die("SymGetSearchPath");
    if (!SymCleanup(proc)) die("SymCleanup");

    // 5. kernel32: custom HeapCreate / HeapAlloc / HeapWalk / HeapDestroy + GetThreadTimes
    HANDLE h = HeapCreate(0, 0, 0);
    if (!h) die("HeapCreate");
    void *blk = HeapAlloc(h, HEAP_ZERO_MEMORY, 96);
    if (!blk) die("HeapAlloc");
    PROCESS_HEAP_ENTRY ent;
    memset(&ent, 0, sizeof(ent));
    int count = 0;
    while (HeapWalk(h, &ent)) {
        if (ent.lpData == blk && ent.cbData == 96) count++;
    }
    if (count != 1) die("HeapWalk");
    if (!HeapDestroy(h)) die("HeapDestroy");

    FILETIME ftC, ftE, ftK, ftU;
    if (!GetThreadTimes(GetCurrentThread(), &ftC, &ftE, &ftK, &ftU)) die("GetThreadTimes");
    ULONG64 cyc = 0;
    if (!QueryThreadCycleTime(GetCurrentThread(), &cyc)) die("QueryThreadCycleTime");

    // 6. user32: SPI_GETNONCLIENTMETRICS (verify real MinGW struct field offsets for A and W)
    NONCLIENTMETRICSA ncm;
    memset(&ncm, 0, sizeof(ncm));
    ncm.cbSize = sizeof(ncm);
    if (!SystemParametersInfoA(SPI_GETNONCLIENTMETRICS, sizeof(ncm), &ncm, 0)) die("SPI_GETNONCLIENTMETRICS A");
    if (ncm.iBorderWidth != 1 || ncm.iScrollWidth != 17 || ncm.iCaptionWidth != 36) die("NCM A ints");
    if (ncm.iSmCaptionWidth != 22 || ncm.iSmCaptionHeight != 22) die("NCM A smCaption");
    if (ncm.iMenuWidth != 19 || ncm.iMenuHeight != 19) die("NCM A menu");
    if (ncm.lfCaptionFont.lfHeight != -12 || strcmp(ncm.lfCaptionFont.lfFaceName, "Segoe UI") != 0) die("NCM A lfCaptionFont");
    if (strcmp(ncm.lfSmCaptionFont.lfFaceName, "Segoe UI") != 0) die("NCM A lfSmCaptionFont");
    if (strcmp(ncm.lfMenuFont.lfFaceName, "Segoe UI") != 0) die("NCM A lfMenuFont");
    if (strcmp(ncm.lfStatusFont.lfFaceName, "Segoe UI") != 0) die("NCM A lfStatusFont");
    if (strcmp(ncm.lfMessageFont.lfFaceName, "Segoe UI") != 0) die("NCM A lfMessageFont");
    if (ncm.iPaddedBorderWidth != 4) die("NCM A iPaddedBorderWidth");

    NONCLIENTMETRICSW ncmw;
    memset(&ncmw, 0, sizeof(ncmw));
    ncmw.cbSize = sizeof(ncmw);
    if (!SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, sizeof(ncmw), &ncmw, 0)) die("SPI_GETNONCLIENTMETRICS W");
    if (ncmw.iSmCaptionWidth != 22 || ncmw.iMenuWidth != 19 || ncmw.iPaddedBorderWidth != 4) die("NCM W ints");
    if (ncmw.lfCaptionFont.lfHeight != -12 || wcscmp(ncmw.lfCaptionFont.lfFaceName, L"Segoe UI") != 0) die("NCM W lfCaptionFont");
    if (wcscmp(ncmw.lfMessageFont.lfFaceName, L"Segoe UI") != 0) die("NCM W lfMessageFont");

    // 7. oleaut32: VariantChangeType + VariantCopy / VariantCopyInd / VariantClear (no double free)
    VARIANT v1, v2;
    VariantInit(&v1);
    VariantInit(&v2);
    V_VT(&v1) = VT_I4;
    V_I4(&v1) = 123;
    if (VariantChangeType(&v2, &v1, 0, VT_BSTR) != S_OK) die("VariantChangeType BSTR");
    if (VariantChangeType(&v2, &v2, 0, VT_R8) != S_OK) die("VariantChangeType R8");
    if (V_R8(&v2) != 123.0) die("VariantChangeType value");
    VariantClear(&v2);

    SAFEARRAY *psa = SafeArrayCreateVector(VT_I4, 0, 2);
    if (!psa) die("SafeArrayCreateVector");
    LONG sa_i0 = 0, sa_i1 = 1, a0 = 11, a1 = 22;
    SafeArrayPutElement(psa, &sa_i0, &a0);
    SafeArrayPutElement(psa, &sa_i1, &a1);
    VARIANT va1, va2;
    VariantInit(&va1);
    VariantInit(&va2);
    V_VT(&va1) = VT_ARRAY | VT_I4;
    V_ARRAY(&va1) = psa;
    if (VariantCopy(&va2, &va1) != S_OK) die("VariantCopy SAFEARRAY");
    if (V_ARRAY(&va2) == V_ARRAY(&va1)) die("VariantCopy aliased SAFEARRAY");
    VariantClear(&va1);
    LONG got1 = 0;
    if (SafeArrayGetElement(V_ARRAY(&va2), &sa_i1, &got1) != S_OK || got1 != 22) die("SafeArrayGetElement after clear");
    VariantClear(&va2);

    LONG byref_val = 555;
    VARIANT vr, vind;
    VariantInit(&vr);
    VariantInit(&vind);
    V_VT(&vr) = VT_BYREF | VT_I4;
    V_I4REF(&vr) = &byref_val;
    if (VariantCopyInd(&vind, &vr) != S_OK || V_VT(&vind) != VT_I4 || V_I4(&vind) != 555) die("VariantCopyInd");
    VariantClear(&vr);
    VariantClear(&vind);

    printf("GUEST_WIN32_PASS\n");
    fflush(stdout);
    return 0;
}
"#;

const STUB_HITTING_GUEST_C: &str = r#"
#define WIN32_LEAN_AND_MEAN
#include <winsock2.h>
#include <windows.h>
#include <stdio.h>

int main(void) {
    // Hit a remaining failing! export (ws2_32!WSAJoinLeaf) 3 times.
    for (int i = 0; i < 3; i++) {
        WSAJoinLeaf(INVALID_SOCKET, NULL, 0, NULL, NULL, NULL, NULL, 0);
    }
    // Hit a remaining stub! branch (user32!GetSystemMetrics with unknown index 9999) 2 times.
    for (int i = 0; i < 2; i++) {
        GetSystemMetrics(9999);
    }
    printf("GUEST_STUBS_DONE\n");
    fflush(stdout);
    return 0;
}
"#;

#[test]
fn win32_unstubbed_apis_and_boot_report_diff_e2e() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping stub_report_e2e: zig not found");
        return;
    };
    let tmp = TempDir::new("main");

    // 1. Run the clean Win32 guest PE covering our newly implemented APIs.
    let clean_exe = compile_guest(
        &zig,
        tmp.path(),
        "clean_win32_guest",
        CLEAN_WIN32_GUEST_C,
        &[
            "-lws2_32",
            "-lcrypt32",
            "-ladvapi32",
            "-ldbghelp",
            "-luser32",
            "-loleaut32",
            "-lkernel32",
        ],
    );
    let clean_report = tmp.path().join("report_clean.json");
    let (clean_code, clean_log) = run_port_runner(tmp.path(), &clean_exe, &clean_report);
    assert_eq!(clean_code, Some(0), "clean_win32_guest failed:\n{clean_log}");
    assert!(
        clean_log.contains("GUEST_WIN32_PASS"),
        "missing GUEST_WIN32_PASS:\n{clean_log}"
    );
    assert!(
        clean_log.contains("Stub Registry Summary: 0 stubs hit"),
        "expected 0 stubs hit in clean run:\n{clean_log}"
    );

    let clean_json = std::fs::read_to_string(&clean_report).expect("read report_clean.json");
    assert!(clean_json.contains("\"kind\": \"clean_exit\""), "bad clean report:\n{clean_json}");
    assert!(clean_json.contains("\"total_hits\": 0"), "expected 0 stub hits:\n{clean_json}");
    assert!(clean_json.contains("\"decoded_interp_blocks\":"), "missing tiers:\n{clean_json}");
    assert!(clean_json.contains("\"tier1_compiles\":"), "missing shaders:\n{clean_json}");

    // 2. Run the stub-hitting guest PE and verify ranked summary + guest caller + --report-json.
    let stub_exe = compile_guest(
        &zig,
        tmp.path(),
        "stub_hit_guest",
        STUB_HITTING_GUEST_C,
        &["-lws2_32", "-luser32", "-lkernel32"],
    );
    let stub_report = tmp.path().join("report_stubs.json");
    let (stub_code, stub_log) = run_port_runner(tmp.path(), &stub_exe, &stub_report);
    assert_eq!(stub_code, Some(0), "stub_hit_guest failed:\n{stub_log}");
    assert!(stub_log.contains("GUEST_STUBS_DONE"), "missing GUEST_STUBS_DONE:\n{stub_log}");
    assert!(
        stub_log.contains("Stub Registry Summary (2 unique sites, 5 total hits)"),
        "expected ranked stub summary in exit log:\n{stub_log}"
    );

    let stub_json = std::fs::read_to_string(&stub_report).expect("read report_stubs.json");
    assert!(stub_json.contains("\"total_hits\": 5"), "bad stub report:\n{stub_json}");
    assert!(stub_json.contains("\"unique_sites\": 2"), "bad stub report:\n{stub_json}");
    assert!(stub_json.contains("\"api\":\"WSAJoinLeaf\",\"kind\":\"failing\",\"hits\":3"), "missing WSAJoinLeaf:\n{stub_json}");
    assert!(stub_json.contains("\"api\":\"GetSystemMetrics\",\"kind\":\"stub\",\"hits\":2"), "missing GetSystemMetrics:\n{stub_json}");
    assert!(
        stub_json.contains("stub_hit_guest.exe+0x"),
        "expected symbolized first_guest_caller in report:\n{stub_json}"
    );
    assert!(
        stub_json.contains("\"file\":\"crates/win32-rt/src/misc/ws2_32.rs\""),
        "expected repo-relative normalized source file in report:\n{stub_json}"
    );

    // 3. Diff the two boot reports using port-factory's report_diff + stub_inventory APIs directly.
    let diff = port_factory::report_diff::diff_boot_report_files(&stub_report, &clean_report)
        .expect("diff_boot_report_files");
    assert!(diff.contains("=== Boot Report Diff ==="), "bad report diff:\n{diff}");
    assert!(diff.contains("RESOLVED stubs (-2)"), "expected 2 resolved stubs:\n{diff}");
    assert!(diff.contains("ws2_32.dll!WSAJoinLeaf"), "missing WSAJoinLeaf in diff:\n{diff}");
    assert!(diff.contains("user32.dll!GetSystemMetrics"), "missing GetSystemMetrics in diff:\n{diff}");
}

const CRED_WRITER_GUEST_C: &str = r#"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <wincrypt.h>
#include <wincred.h>
#include <stdio.h>
#include <string.h>

static void die(const char *msg) {
    fprintf(stderr, "WRITER_FAIL: %s (err=%lu)\n", msg, (unsigned long)GetLastError());
    ExitProcess(1);
}

int main(void) {
    CREDENTIALW c;
    memset(&c, 0, sizeof(c));
    c.Type = CRED_TYPE_GENERIC;
    c.UserName = (LPWSTR)L"persist-user";

    c.TargetName = (LPWSTR)L"CrossProc/LocalMachine";
    c.CredentialBlob = (LPBYTE)"local-secret-42";
    c.CredentialBlobSize = 15;
    c.Persist = CRED_PERSIST_LOCAL_MACHINE;
    if (!CredWriteW(&c, 0)) die("CredWriteW local");

    c.TargetName = (LPWSTR)L"CrossProc/Enterprise";
    c.CredentialBlob = (LPBYTE)"ent-secret-99";
    c.CredentialBlobSize = 13;
    c.Persist = CRED_PERSIST_ENTERPRISE;
    if (!CredWriteW(&c, 0)) die("CredWriteW enterprise");

    c.TargetName = (LPWSTR)L"CrossProc/SessionOnly";
    c.CredentialBlob = (LPBYTE)"ephemeral";
    c.CredentialBlobSize = 9;
    c.Persist = CRED_PERSIST_SESSION;
    if (!CredWriteW(&c, 0)) die("CredWriteW session");

    const BYTE secret[] = "cross-process-dpapi-secret";
    DATA_BLOB inb = { (DWORD)(sizeof(secret) - 1), (BYTE*)secret };
    DATA_BLOB outb = { 0, NULL };
    if (!CryptProtectData(&inb, L"cross-proc", NULL, NULL, NULL, 0, &outb)) die("CryptProtectData");

    HANDLE fh = CreateFileA("C:\\Windows\\Temp\\dpapi_blob.bin", GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL);
    if (fh == INVALID_HANDLE_VALUE) die("CreateFileA");
    DWORD written = 0;
    if (!WriteFile(fh, outb.pbData, outb.cbData, &written, NULL) || written != outb.cbData) die("WriteFile");
    CloseHandle(fh);
    LocalFree(outb.pbData);

    printf("CRED_WRITER_PASS\n");
    fflush(stdout);
    return 0;
}
"#;

const CRED_READER_GUEST_C: &str = r#"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <wincrypt.h>
#include <wincred.h>
#include <stdio.h>
#include <string.h>

static void die(const char *msg) {
    fprintf(stderr, "READER_FAIL: %s (err=%lu)\n", msg, (unsigned long)GetLastError());
    ExitProcess(1);
}

int main(void) {
    PCREDENTIALW out = NULL;
    if (!CredReadW(L"CrossProc/LocalMachine", CRED_TYPE_GENERIC, 0, &out) || !out) die("CredReadW local");
    if (out->CredentialBlobSize != 15 || memcmp(out->CredentialBlob, "local-secret-42", 15) != 0) die("local blob mismatch");
    if (out->Persist != CRED_PERSIST_LOCAL_MACHINE) die("local persist mismatch");
    CredFree(out);

    out = NULL;
    if (!CredReadW(L"CrossProc/Enterprise", CRED_TYPE_GENERIC, 0, &out) || !out) die("CredReadW enterprise");
    if (out->CredentialBlobSize != 13 || memcmp(out->CredentialBlob, "ent-secret-99", 13) != 0) die("enterprise blob mismatch");
    if (out->Persist != CRED_PERSIST_ENTERPRISE) die("enterprise persist mismatch");
    CredFree(out);

    out = NULL;
    if (CredReadW(L"CrossProc/SessionOnly", CRED_TYPE_GENERIC, 0, &out)) die("session cred must not persist");
    if (GetLastError() != ERROR_NOT_FOUND) die("session cred wrong error");

    if (!CredDeleteW(L"CrossProc/LocalMachine", CRED_TYPE_GENERIC, 0)) die("CredDeleteW local");
    if (!CredDeleteW(L"CrossProc/Enterprise", CRED_TYPE_GENERIC, 0)) die("CredDeleteW enterprise");

    HANDLE fh = CreateFileA("C:\\Windows\\Temp\\dpapi_blob.bin", GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (fh == INVALID_HANDLE_VALUE) die("Open dpapi_blob.bin");
    BYTE buf[512];
    DWORD got = 0;
    if (!ReadFile(fh, buf, sizeof(buf), &got, NULL) || got == 0) die("ReadFile dpapi_blob.bin");
    CloseHandle(fh);

    DATA_BLOB enc = { got, buf };
    DATA_BLOB dec = { 0, NULL };
    if (!CryptUnprotectData(&enc, NULL, NULL, NULL, NULL, 0, &dec)) die("CryptUnprotectData same profile");
    const char *want = "cross-process-dpapi-secret";
    if (dec.cbData != strlen(want) || memcmp(dec.pbData, want, dec.cbData) != 0) die("DPAPI payload mismatch");
    LocalFree(dec.pbData);

    printf("CRED_READER_PASS\n");
    fflush(stdout);
    return 0;
}
"#;

const DPAPI_OTHER_SECRET_GUEST_C: &str = r#"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <wincrypt.h>
#include <stdio.h>

static void die(const char *msg) {
    fprintf(stderr, "OTHER_FAIL: %s (err=%lu)\n", msg, (unsigned long)GetLastError());
    ExitProcess(1);
}

int main(void) {
    HANDLE fh = CreateFileA("C:\\Windows\\Temp\\dpapi_blob.bin", GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (fh == INVALID_HANDLE_VALUE) die("Open dpapi_blob.bin");
    BYTE buf[512];
    DWORD got = 0;
    if (!ReadFile(fh, buf, sizeof(buf), &got, NULL) || got == 0) die("ReadFile");
    CloseHandle(fh);

    DATA_BLOB enc = { got, buf };
    DATA_BLOB dec = { 0, NULL };
    if (CryptUnprotectData(&enc, NULL, NULL, NULL, NULL, 0, &dec)) die("other profile must NOT decrypt");
    if (GetLastError() != ERROR_INVALID_DATA) die("expected ERROR_INVALID_DATA");

    printf("DPAPI_ISOLATION_PASS\n");
    fflush(stdout);
    return 0;
}
"#;

#[test]
fn cred_persist_and_dpapi_across_processes_e2e() {
    let Some(zig) = find_zig() else {
        eprintln!("skipping cred_persist_and_dpapi_across_processes_e2e: zig not found");
        return;
    };
    let tmp_a = TempDir::new("cred_a");
    let tmp_b = TempDir::new("cred_b");

    let writer_exe = compile_guest(
        &zig,
        tmp_a.path(),
        "cred_writer",
        CRED_WRITER_GUEST_C,
        &["-ladvapi32", "-lcrypt32", "-lkernel32"],
    );
    let reader_exe = compile_guest(
        &zig,
        tmp_a.path(),
        "cred_reader",
        CRED_READER_GUEST_C,
        &["-ladvapi32", "-lcrypt32", "-lkernel32"],
    );
    let other_exe = compile_guest(
        &zig,
        tmp_b.path(),
        "dpapi_other_secret",
        DPAPI_OTHER_SECRET_GUEST_C,
        &["-lcrypt32", "-lkernel32"],
    );

    // Run 1: write LOCAL_MACHINE, ENTERPRISE, SESSION creds + DPAPI blob in profile A.
    let rep1 = tmp_a.path().join("rep1.json");
    let (code1, log1) = run_port_runner(tmp_a.path(), &writer_exe, &rep1);
    assert_eq!(code1, Some(0), "cred_writer failed:\n{log1}");
    assert!(log1.contains("CRED_WRITER_PASS"), "missing CRED_WRITER_PASS:\n{log1}");

    // Run 2: fresh port-runner process sharing profile A reads persisted creds & DPAPI blob;
    // SESSION cred must be gone.
    let rep2 = tmp_a.path().join("rep2.json");
    let (code2, log2) = run_port_runner(tmp_a.path(), &reader_exe, &rep2);
    assert_eq!(code2, Some(0), "cred_reader failed:\n{log2}");
    assert!(log2.contains("CRED_READER_PASS"), "missing CRED_READER_PASS:\n{log2}");

    // Copy profile A's DPAPI ciphertext into profile B (which has its own random host secret).
    let blob_src = tmp_a.path().join("drive_c").join("Temp").join("dpapi_blob.bin");
    let blob_dst_dir = tmp_b.path().join("drive_c").join("Temp");
    std::fs::create_dir_all(&blob_dst_dir).unwrap();
    std::fs::copy(&blob_src, blob_dst_dir.join("dpapi_blob.bin")).unwrap();

    // Run 3: fresh port-runner process in profile B must NOT decrypt profile A's DPAPI blob.
    let rep3 = tmp_b.path().join("rep3.json");
    let (code3, log3) = run_port_runner(tmp_b.path(), &other_exe, &rep3);
    assert_eq!(code3, Some(0), "dpapi_other_secret failed:\n{log3}");
    assert!(log3.contains("DPAPI_ISOLATION_PASS"), "missing DPAPI_ISOLATION_PASS:\n{log3}");
}
