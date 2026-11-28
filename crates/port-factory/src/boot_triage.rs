//! Headless boot-triage primitive (`cargo opsdb -- boot-triage`).
//!
//! Launches `port-runner` headless (`PORT_HEADLESS=1`), captures its combined
//! runtime log, `boot_report.json`, and any `crash-*.json`/`crash-*.txt`
//! sidecars in an isolated temporary directory, classifies the **first
//! blocker**, and attributes it to the likely owning crate using the
//! `AGENTS.md` "Where code belongs" table.
//!
//! Supported blocker classes:
//! - `unresolved_import`: static trap thunk (`UNIMPLEMENTED dll!sym`), COM
//!   trap (`UNIMPLEMENTED IFoo::Method`), or delay-load exception
//!   (`0xc06d007e` module not found / `0xc06d007f` proc not found)
//! - `stub_hit`: `stub!` or `failing!` Win32 API hit
//! - `guest_crash`: unhandled guest exception, access violation, `__fastfail`,
//!   host fault, or unimplemented x86-64 opcode (`EXIT_UNIMPL_OP = 86`),
//!   symbolized as `module+0xRVA` with the faulting instruction
//! - `rust_panic_or_abort`: Rust panic or `SIGABRT` with backtrace frames
//! - `metal_validation_error`: Apple Metal debug/validation layer assertion
//!   or fatal Metal 4 error
//! - `d3d12_dxgi_hresult`: failing `HRESULT` (`0x8xxxxxxx`, `DXGI_ERROR_*`,
//!   `D3D12_ERROR_*`, `E_*`) from a D3D12 / DXGI / D3D11 call
//! - `hang_timeout`: wall-clock timeout or watchdog stall with last activity
//! - `startup_error` / `unknown_failure` / `none` (clean boot)

use crate::bundle_common::{find_or_build_host_port_runner, TempDirGuard};
use crate::json_output::{exit_codes, parse_json, JsonEnvelope, JsonObj, JsonValue};
use anyhow::{bail, Context, Result};
use port_manifest::GameManifest;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Canonical blocker classification returned by `boot-triage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockerClass {
    None,
    UnresolvedImport,
    StubHit,
    GuestCrash,
    RustPanicOrAbort,
    MetalValidationError,
    D3d12DxgiHresult,
    HangTimeout,
    StartupError,
    UnknownFailure,
}

impl BlockerClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::UnresolvedImport => "unresolved_import",
            Self::StubHit => "stub_hit",
            Self::GuestCrash => "guest_crash",
            Self::RustPanicOrAbort => "rust_panic_or_abort",
            Self::MetalValidationError => "metal_validation_error",
            Self::D3d12DxgiHresult => "d3d12_dxgi_hresult",
            Self::HangTimeout => "hang_timeout",
            Self::StartupError => "startup_error",
            Self::UnknownFailure => "unknown_failure",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "CLEAN_BOOT",
            Self::UnresolvedImport => "UNRESOLVED_IMPORT",
            Self::StubHit => "STUB_HIT",
            Self::GuestCrash => "GUEST_CRASH",
            Self::RustPanicOrAbort => "RUST_PANIC_OR_ABORT",
            Self::MetalValidationError => "METAL_VALIDATION_ERROR",
            Self::D3d12DxgiHresult => "D3D12_DXGI_HRESULT",
            Self::HangTimeout => "HANG_TIMEOUT",
            Self::StartupError => "STARTUP_ERROR",
            Self::UnknownFailure => "UNKNOWN_FAILURE",
        }
    }

    /// Canonical Port Factory CLI exit code for this classification.
    pub const fn exit_code(self) -> i32 {
        match self {
            Self::None => exit_codes::OK,
            Self::UnresolvedImport => exit_codes::UNRESOLVED_IMPORTS,
            Self::HangTimeout => exit_codes::TIMEOUT,
            Self::StubHit
            | Self::GuestCrash
            | Self::RustPanicOrAbort
            | Self::MetalValidationError
            | Self::D3d12DxgiHresult
            | Self::StartupError
            | Self::UnknownFailure => exit_codes::RUNTIME_ERROR,
        }
    }
}

/// Ownership recommendation derived from `AGENTS.md`'s "Where code belongs" table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerHint {
    pub crate_name: String,
    pub paths: Vec<String>,
    pub area: String,
    pub reason: String,
    pub next_step: String,
}

/// Detailed description of the classified first blocker.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BlockerDetail {
    pub class: Option<BlockerClass>,
    pub subkind: String,
    pub summary: String,
    pub log_line: Option<usize>,
    pub evidence: Option<String>,
    pub dll: Option<String>,
    pub symbol: Option<String>,
    pub exception_code: Option<String>,
    pub hresult: Option<String>,
    pub location: Option<String>,
    pub faulting_instruction: Option<String>,
    pub backtrace: Vec<String>,
    pub last_activity: Vec<String>,
}

impl BlockerDetail {
    pub fn class(&self) -> BlockerClass {
        self.class.unwrap_or(BlockerClass::None)
    }
}

/// Non-primary signals observed earlier or alongside the primary blocker
/// (for example `stub!`/`failing!` hits or D3D12 HRESULT warnings before a crash or hang).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecondarySignal {
    pub class: BlockerClass,
    pub subkind: String,
    pub log_line: Option<usize>,
    pub summary: String,
    pub detail: String,
}

/// Target metadata recorded in the triage report.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BootTriageTarget {
    pub manifest: Option<String>,
    pub game_dir: Option<String>,
    pub pe: Option<String>,
    pub exe: Option<String>,
    pub args: Vec<String>,
    pub timeout_secs: Option<f64>,
    pub max_frames: Option<u64>,
}

/// Execution telemetry for a live or offline boot-triage run.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BootTriageRun {
    pub mode: String,
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub timed_out: bool,
    pub elapsed_ms: u64,
    pub frames_presented: u64,
    pub graphics_api: String,
    pub log_lines: usize,
    pub log_path: Option<String>,
    pub boot_report_found: bool,
    pub crash_report_found: bool,
}

/// Complete structured boot-triage report.
#[derive(Debug, Clone, PartialEq)]
pub struct BootTriageReport {
    pub target: BootTriageTarget,
    pub run: BootTriageRun,
    pub blocker: BlockerDetail,
    pub owner: Option<OwnerHint>,
    pub secondary_signals: Vec<SecondarySignal>,
    pub missing_probes: Vec<String>,
}

impl BootTriageReport {
    pub fn exit_code(&self) -> i32 {
        self.blocker.class().exit_code()
    }
}

/// Inputs to the pure offline log classifier [`classify_boot_log`].
#[derive(Debug, Clone, Default)]
pub struct BootTriageInput {
    pub target: BootTriageTarget,
    pub mode: String,
    pub log_text: String,
    pub boot_report_json: Option<String>,
    pub crash_report_json: Option<String>,
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub timed_out: bool,
    pub elapsed_ms: u64,
    pub log_path: Option<String>,
}

/// Configuration for a live headless [`run_boot_triage`] execution.
#[derive(Debug, Clone)]
pub struct BootTriageOptions {
    pub workspace_root: PathBuf,
    pub manifest: Option<PathBuf>,
    pub game_dir: Option<PathBuf>,
    pub pe: Option<PathBuf>,
    pub exe: Option<String>,
    pub game_args: Vec<String>,
    pub timeout_secs: f64,
    /// Stop cleanly after N presented frames (`None` disables `--max-frames`).
    pub max_frames: Option<u64>,
    pub runner: Option<PathBuf>,
    pub aot_dir: Option<PathBuf>,
    pub drive_c: Option<PathBuf>,
    pub extra_env: Vec<(String, String)>,
    pub metal_validation: bool,
    pub log_out: Option<PathBuf>,
}

impl Default for BootTriageOptions {
    fn default() -> Self {
        Self {
            workspace_root: PathBuf::from("."),
            manifest: None,
            game_dir: None,
            pe: None,
            exe: None,
            game_args: Vec::new(),
            timeout_secs: 30.0,
            max_frames: Some(5),
            runner: None,
            aot_dir: None,
            drive_c: None,
            extra_env: Vec::new(),
            metal_validation: false,
            log_out: None,
        }
    }
}

/// Split an `--args` string into individual guest CLI tokens (supporting single and double quotes
/// while preserving literal Windows backslashes inside double-quoted paths).
pub fn split_arg_string(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut has_token = false;
    let mut in_double = false;
    let mut in_single = false;
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' if in_double => {
                // Windows CommandLineToArgvW rule: backslashes are literal unless immediately
                // preceding a double quote (`2k` slashes + `"` => `k` slashes + close quote;
                // `2k+1` slashes + `"` => `k` slashes + literal `"`).
                let mut slashes = 1usize;
                while chars.peek() == Some(&'\\') {
                    chars.next();
                    slashes += 1;
                }
                if chars.peek() == Some(&'"') {
                    for _ in 0..(slashes / 2) {
                        cur.push('\\');
                    }
                    if slashes % 2 == 1 {
                        chars.next();
                        cur.push('"');
                    }
                } else {
                    for _ in 0..slashes {
                        cur.push('\\');
                    }
                }
                has_token = true;
            }
            '"' if !in_single => {
                in_double = !in_double;
                has_token = true;
            }
            '\'' if !in_double => {
                in_single = !in_single;
                has_token = true;
            }
            c if c.is_whitespace() && !in_double && !in_single => {
                if has_token {
                    out.push(std::mem::take(&mut cur));
                    has_token = false;
                }
            }
            c => {
                cur.push(c);
                has_token = true;
            }
        }
    }
    if has_token {
        out.push(cur);
    }
    out
}

/// Launch `port-runner` headless, capture its combined log, `boot_report.json`, and any
/// `crash-*.json` / `crash-*.txt` files, and return the classified [`BootTriageReport`].
pub fn run_boot_triage(opts: &BootTriageOptions) -> Result<BootTriageReport> {
    if opts.manifest.is_none() && opts.pe.is_none() && opts.exe.is_none() {
        bail!("boot-triage requires --manifest <TOML>, --pe <EXE>, or --game-dir <DIR> --exe <EXE>");
    }
    if !opts.timeout_secs.is_finite() || opts.timeout_secs <= 0.0 {
        bail!(
            "--timeout must be a positive finite number, got {}",
            opts.timeout_secs
        );
    }

    let runner = match &opts.runner {
        Some(p) => {
            if !p.is_file() {
                bail!("port-runner not found at {}", p.display());
            }
            p.clone()
        }
        None => find_or_build_host_port_runner(&opts.workspace_root)?,
    };

    let tmp = TempDirGuard::create_unique("port_factory_boot_triage")?;
    let log_dir = tmp.path().join("logs");
    let crash_dir = tmp.path().join("crash");
    let stdio_path = tmp.path().join("stdio.log");
    let boot_report_path = tmp.path().join("boot_report.json");
    fs::create_dir_all(&log_dir)?;
    fs::create_dir_all(&crash_dir)?;

    let stdio_file = File::create(&stdio_path)
        .with_context(|| format!("creating {}", stdio_path.display()))?;
    let stderr_file = stdio_file
        .try_clone()
        .context("cloning stdio capture handle")?;

    let mut cmd = Command::new(&runner);
    if let Some(ref m) = opts.manifest {
        cmd.arg("--manifest").arg(m);
    }
    if let Some(ref gd) = opts.game_dir {
        cmd.arg("--game-dir").arg(gd);
    }
    if let Some(ref pe) = opts.pe {
        cmd.arg("--pe").arg(pe);
    }
    if let Some(ref exe) = opts.exe {
        cmd.arg("--exe").arg(exe);
    }
    if let Some(ref aot_dir) = opts.aot_dir {
        cmd.arg("--aot-dir").arg(aot_dir);
    }
    if let Some(ref drive_c) = opts.drive_c {
        cmd.arg("--drive-c").arg(drive_c);
    }
    cmd.arg("--report-json").arg(&boot_report_path);
    cmd.arg("--timeout-secs").arg(format!("{}", opts.timeout_secs));
    if let Some(mf) = opts.max_frames.filter(|&n| n > 0) {
        cmd.arg("--max-frames").arg(mf.to_string());
    }
    if !opts.game_args.is_empty() {
        cmd.arg("--").args(&opts.game_args);
    }

    cmd.env("PORT_HEADLESS", "1");
    cmd.env("AW2_HEADLESS", "1");
    cmd.env("PORT_CHILD_STDIO", "1");
    cmd.env("PORT_CHILD_LOG_STDERR", "1");
    cmd.env("PORT_LOG_DIR", &log_dir);
    cmd.env("AW2_LOG_DIR", &log_dir);
    cmd.env("PORT_CRASH_DIR", &crash_dir);
    cmd.env("AW2_CRASH_DIR", &crash_dir);
    cmd.env("RUST_BACKTRACE", "1");
    if std::env::var_os("RUST_LOG").is_none() {
        cmd.env("RUST_LOG", "info");
    }
    if std::env::var_os("PORT_AUDIO_BACKEND").is_none()
        && std::env::var_os("AW2_AUDIO_BACKEND").is_none()
    {
        cmd.env("PORT_AUDIO_BACKEND", "null");
        cmd.env("AW2_AUDIO_BACKEND", "null");
    }
    if opts.metal_validation {
        cmd.env("MTL_DEBUG_LAYER", "1");
    }
    for (k, v) in &opts.extra_env {
        cmd.env(k, v);
    }
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::from(stdio_file));
    cmd.stderr(Stdio::from(stderr_file));

    let started = Instant::now();
    let mut child = cmd
        .spawn()
        .with_context(|| format!("spawning {}", runner.display()))?;

    // Parent-side hard watchdog in case port-runner stalls before its internal
    // timeout thread starts or wedges inside a signal handler.
    let hard_deadline = Duration::from_secs_f64(opts.timeout_secs + 5.0);
    let mut parent_timed_out = false;
    let status = loop {
        if let Some(st) = child.try_wait().context("polling port-runner child")? {
            break st;
        }
        if started.elapsed() >= hard_deadline {
            parent_timed_out = true;
            #[cfg(unix)]
            {
                let _ = Command::new("/bin/kill")
                    .args(["-TERM", &child.id().to_string()])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
            let grace = Instant::now();
            let mut exited = None;
            while grace.elapsed() < Duration::from_secs(2) {
                if let Some(st) = child.try_wait().context("polling child after SIGTERM")? {
                    exited = Some(st);
                    break;
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            if let Some(st) = exited {
                break st;
            }
            let _ = child.kill();
            break child.wait().context("waiting for killed port-runner")?;
        }
        std::thread::sleep(Duration::from_millis(25));
    };
    let elapsed_ms = started.elapsed().as_millis() as u64;

    let exit_code = status.code();
    #[cfg(unix)]
    let signal = {
        use std::os::unix::process::ExitStatusExt;
        status.signal()
    };
    #[cfg(not(unix))]
    let signal: Option<i32> = None;

    let timed_out = parent_timed_out || exit_code == Some(exit_codes::TIMEOUT);

    // Collect combined log text from stdio capture + any PORT_LOG_DIR file + crash .txt.
    let mut log_text = read_lossy(&stdio_path);
    if let Ok(rd) = fs::read_dir(&log_dir) {
        let mut log_files: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.is_file()
                    && p.extension()
                        .and_then(|s| s.to_str())
                        .is_some_and(|e| e.eq_ignore_ascii_case("log"))
            })
            .collect();
        log_files.sort();
        for lf in log_files {
            let txt = read_lossy(&lf);
            if log_text.trim().is_empty() && !txt.trim().is_empty() {
                log_text = txt;
            } else if !txt.trim().is_empty() {
                // Append any lines from the tee'd boot log not already present (e.g. boot_log_line).
                let existing: BTreeSet<&str> = log_text.lines().collect();
                let missing: Vec<&str> = txt
                    .lines()
                    .filter(|l| !l.trim().is_empty() && !existing.contains(l))
                    .collect();
                if !missing.is_empty() {
                    if !log_text.ends_with('\n') && !log_text.is_empty() {
                        log_text.push('\n');
                    }
                    for m in missing {
                        log_text.push_str(m);
                        log_text.push('\n');
                    }
                }
            }
        }
    }

    let boot_report_json = fs::read_to_string(&boot_report_path).ok();
    let (crash_report_json, crash_report_txt) = read_first_crash_files(&crash_dir);
    if let Some(ref ctxt) = crash_report_txt {
        if !log_text.contains("CRASH REPORT:") && !ctxt.trim().is_empty() {
            if !log_text.ends_with('\n') && !log_text.is_empty() {
                log_text.push('\n');
            }
            log_text.push_str(ctxt);
            if !log_text.ends_with('\n') {
                log_text.push('\n');
            }
        }
    }

    let mut saved_log_path: Option<String> = None;
    if let Some(ref out_p) = opts.log_out {
        if let Some(parent) = out_p.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        fs::write(out_p, &log_text)
            .with_context(|| format!("writing captured log to {}", out_p.display()))?;
        saved_log_path = Some(out_p.display().to_string());
    }

    let manifest_exe = opts
        .manifest
        .as_deref()
        .and_then(|p| GameManifest::load_file(p).ok())
        .and_then(|m| m.exe);
    let exe = opts
        .exe
        .clone()
        .or(manifest_exe)
        .or_else(|| {
            opts.pe
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|s| s.to_string_lossy().into_owned())
        });

    let input = BootTriageInput {
        target: BootTriageTarget {
            manifest: opts.manifest.as_ref().map(|p| p.display().to_string()),
            game_dir: opts.game_dir.as_ref().map(|p| p.display().to_string()),
            pe: opts.pe.as_ref().map(|p| p.display().to_string()),
            exe,
            args: opts.game_args.clone(),
            timeout_secs: Some(opts.timeout_secs),
            max_frames: opts.max_frames,
        },
        mode: "live".to_string(),
        log_text,
        boot_report_json,
        crash_report_json,
        exit_code,
        signal,
        timed_out,
        elapsed_ms,
        log_path: saved_log_path,
    };
    Ok(classify_boot_log(&input))
}

fn read_lossy(path: &Path) -> String {
    fs::read(path)
        .map(|b| String::from_utf8_lossy(&b).into_owned())
        .unwrap_or_default()
}

fn read_first_crash_files(crash_dir: &Path) -> (Option<String>, Option<String>) {
    let Ok(rd) = fs::read_dir(crash_dir) else {
        return (None, None);
    };
    let mut jsons = Vec::new();
    let mut txts = Vec::new();
    for entry in rd.flatten() {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        match p.extension().and_then(|s| s.to_str()) {
            Some(e) if e.eq_ignore_ascii_case("json") => jsons.push(p),
            Some(e) if e.eq_ignore_ascii_case("txt") => txts.push(p),
            _ => {}
        }
    }
    jsons.sort();
    txts.sort();
    let json = jsons.first().and_then(|p| fs::read_to_string(p).ok());
    let txt = txts.first().and_then(|p| fs::read_to_string(p).ok());
    (json, txt)
}

// ---------------------------------------------------------------------------
// Log parsing & classification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct ParsedLine<'a> {
    line_no: usize,
    raw: &'a str,
    level: &'a str,
    target: &'a str,
    msg: &'a str,
}

#[derive(Debug, Clone, Default)]
struct BootReportMeta {
    present: bool,
    exit_kind: String,
    exit_code: Option<i64>,
    exit_detail: String,
    frames_presented: u64,
    graphics_api: String,
    stubs: Vec<StubMeta>,
    unresolved_imports: Vec<String>,
}

#[derive(Debug, Clone)]
struct StubMeta {
    dll: String,
    api: String,
    kind: String,
    hits: u64,
    first_caller: String,
    file: String,
    line: u64,
    detail: String,
}

#[derive(Debug, Clone, Default)]
struct CrashReportMeta {
    present: bool,
    kind: String,
    title: String,
    exception_code: Option<String>,
    exception_desc: Option<String>,
    guest_symbol: Option<String>,
    top_frame_symbol: Option<String>,
    faulting_insn: Option<String>,
    frames: Vec<String>,
}

/// Strip ANSI CSI escape sequences so pattern matching works even on colorized logs.
fn strip_ansi(s: &str) -> String {
    if !s.contains('\x1b') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for ch in chars.by_ref() {
                if ('@'..='~').contains(&ch) {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Parse an `env_logger` (`[ts LEVEL target] msg`) or `boot_log_line` (`[secs pid=... boot] msg`) line.
fn parse_log_line<'a>(line_no: usize, raw: &'a str) -> ParsedLine<'a> {
    let trimmed = raw.trim_end_matches('\r');
    if let Some(rest) = trimmed.strip_prefix('[') {
        if let Some(close) = rest.find(']') {
            let header = &rest[..close];
            let msg = rest[close + 1..].trim_start();
            let parts: Vec<&str> = header.split_whitespace().collect();
            if parts.len() >= 3
                && matches!(parts[1], "ERROR" | "WARN" | "INFO" | "DEBUG" | "TRACE")
            {
                return ParsedLine {
                    line_no,
                    raw: trimmed,
                    level: parts[1],
                    target: parts[2],
                    msg,
                };
            }
            if parts.len() >= 3 && parts.last() == Some(&"boot") {
                return ParsedLine {
                    line_no,
                    raw: trimmed,
                    level: "BOOT",
                    target: "boot",
                    msg,
                };
            }
        }
    }
    ParsedLine {
        line_no,
        raw: trimmed,
        level: "",
        target: "",
        msg: trimmed,
    }
}

fn parse_boot_report_meta(raw: Option<&str>) -> BootReportMeta {
    let mut meta = BootReportMeta::default();
    let Some(raw) = raw else {
        return meta;
    };
    let Ok(val) = parse_json(raw) else {
        return meta;
    };
    meta.present = true;
    if let Some(exit) = val.get("exit") {
        meta.exit_kind = exit
            .get("kind")
            .and_then(JsonValue::as_str)
            .unwrap_or("")
            .to_string();
        meta.exit_code = exit
            .get("code")
            .and_then(|v| v.as_i64().or_else(|| v.as_u64().map(|u| u as i64)));
        meta.exit_detail = exit
            .get("detail")
            .and_then(JsonValue::as_str)
            .unwrap_or("")
            .to_string();
        meta.frames_presented = exit
            .get("frames_presented")
            .and_then(JsonValue::as_u64)
            .unwrap_or(0);
        meta.graphics_api = exit
            .get("graphics_api")
            .and_then(JsonValue::as_str)
            .unwrap_or("unknown")
            .to_string();
    }
    if let Some(entries) = val
        .get("stubs")
        .and_then(|s| s.get("entries"))
        .and_then(JsonValue::as_array)
    {
        for e in entries {
            let api = e
                .get("api")
                .and_then(JsonValue::as_str)
                .unwrap_or("")
                .to_string();
            if api.is_empty() {
                continue;
            }
            meta.stubs.push(StubMeta {
                dll: e
                    .get("dll")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("")
                    .to_string(),
                api,
                kind: e
                    .get("kind")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("stub")
                    .to_string(),
                hits: e.get("hits").and_then(JsonValue::as_u64).unwrap_or(1),
                first_caller: e
                    .get("first_guest_caller")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("")
                    .to_string(),
                file: e
                    .get("file")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("")
                    .to_string(),
                line: e.get("line").and_then(JsonValue::as_u64).unwrap_or(0),
                detail: e
                    .get("first_detail")
                    .and_then(JsonValue::as_str)
                    .unwrap_or("")
                    .to_string(),
            });
        }
    }
    if let Some(arr) = val.get("unresolved_imports").and_then(JsonValue::as_array) {
        for item in arr {
            if let Some(s) = item.as_str() {
                meta.unresolved_imports.push(s.to_string());
            }
        }
    }
    meta
}

fn parse_crash_report_meta(raw: Option<&str>) -> CrashReportMeta {
    let mut meta = CrashReportMeta::default();
    let Some(raw) = raw else {
        return meta;
    };
    let Ok(val) = parse_json(raw) else {
        return meta;
    };
    meta.present = true;
    meta.kind = val
        .get("kind")
        .and_then(JsonValue::as_str)
        .unwrap_or("")
        .to_string();
    meta.title = val
        .get("title")
        .and_then(JsonValue::as_str)
        .unwrap_or("")
        .to_string();
    if let Some(ex) = val.get("exception") {
        meta.exception_code = ex
            .get("code")
            .and_then(JsonValue::as_str)
            .map(str::to_string);
        meta.exception_desc = ex
            .get("description")
            .and_then(JsonValue::as_str)
            .map(str::to_string);
    }
    let mut guest_frames: Vec<String> = Vec::new();
    if let Some(gs) = val.get("guest_state") {
        meta.guest_symbol = gs
            .get("symbol")
            .and_then(JsonValue::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        if let Some(frames) = gs.get("frames").and_then(JsonValue::as_array) {
            for f in frames.iter().take(16) {
                if let Some(sym) = f.get("symbol").and_then(JsonValue::as_str) {
                    guest_frames.push(sym.to_string());
                }
            }
        }
    }
    let mut bt_frames: Vec<String> = Vec::new();
    if let Some(bt) = val.get("backtrace") {
        if let Some(frames) = bt.get("frames").and_then(JsonValue::as_array) {
            if meta.top_frame_symbol.is_none() {
                meta.top_frame_symbol = frames
                    .first()
                    .and_then(|f| f.get("symbol"))
                    .and_then(JsonValue::as_str)
                    .map(str::to_string);
            }
            for f in frames.iter().take(16) {
                if let Some(sym) = f.get("symbol").and_then(JsonValue::as_str) {
                    bt_frames.push(sym.to_string());
                }
            }
        }
    }
    let has_sym = |fs: &[String]| fs.iter().any(|s| extract_module_rva(s).is_some());
    meta.frames = if has_sym(&guest_frames) {
        guest_frames
    } else if has_sym(&bt_frames) {
        bt_frames
    } else if !guest_frames.is_empty() {
        guest_frames
    } else {
        bt_frames
    };
    if let Some(code) = val.get("code_at_rip") {
        if let Some(dis) = code.get("disassembly").and_then(JsonValue::as_array) {
            if let Some(first) = dis.first().and_then(JsonValue::as_str) {
                meta.faulting_insn = Some(clean_disasm_instruction(first));
            }
        }
    }
    meta
}

/// Classify a captured log (and optional `boot_report.json` / `crash-*.json`) into a
/// structured [`BootTriageReport`]. Pure & deterministic — suitable for unit tests on
/// synthetic fixture logs as well as live `run_boot_triage`.
pub fn classify_boot_log(input: &BootTriageInput) -> BootTriageReport {
    let cleaned_log = strip_ansi(&input.log_text);
    let raw_lines: Vec<&str> = cleaned_log.lines().collect();
    let lines: Vec<ParsedLine<'_>> = raw_lines
        .iter()
        .enumerate()
        .map(|(idx, l)| parse_log_line(idx + 1, l))
        .collect();

    let boot_meta = parse_boot_report_meta(input.boot_report_json.as_deref());
    let crash_meta = parse_crash_report_meta(input.crash_report_json.as_deref());

    let mut frames_presented = boot_meta.frames_presented;
    if frames_presented == 0 {
        frames_presented = infer_frames_presented_from_log(&lines);
    }
    let graphics_api = if !boot_meta.graphics_api.is_empty() && boot_meta.graphics_api != "unknown"
    {
        boot_meta.graphics_api.clone()
    } else {
        infer_graphics_api_from_log(&lines)
    };

    let missing_probes = collect_missing_probes(&lines);

    // Fall back to `boot_report.json` `exit.code` when no explicit process status was supplied
    // (e.g. offline `--log-file` + `--boot-report` without `--exit-code`).
    let boot_exit_code = boot_meta.exit_code.map(|c| {
        if c == 0 {
            0
        } else if let Ok(v) = i32::try_from(c) {
            v
        } else {
            let low = c as i32;
            if low != 0 {
                low
            } else {
                1
            }
        }
    });
    let effective_exit_code = input.exit_code.or(boot_exit_code);

    let log_timed_out = lines.iter().any(|l| {
        l.msg.contains("[run-limit] timed out after")
            || l.msg.contains("port-runner: timed out after")
            || l.msg.contains("CRASH REPORT:") && (l.msg.contains("stall") || l.msg.contains("hang"))
    });
    let is_timeout = input.timed_out
        || effective_exit_code == Some(exit_codes::TIMEOUT)
        || log_timed_out
        || matches!(boot_meta.exit_kind.as_str(), "timeout" | "stall")
        || (crash_meta.present && crash_meta.kind == "stall");

    // Collect candidates across all classes.
    let unresolved = detect_unresolved_import(&lines, &boot_meta, &crash_meta);
    let metal_val = detect_metal_validation(&lines);
    let panic_abort = detect_rust_panic_or_abort(&lines, effective_exit_code, input.signal);
    let (guest_crash, nonfatal_guest_signals) = detect_guest_crash(
        &lines,
        &crash_meta,
        effective_exit_code,
        input.signal,
        is_timeout,
        frames_presented,
    );
    let hresult = detect_d3d12_dxgi_hresult(&lines);
    let stub_hit = detect_stub_hit(&lines, &boot_meta);
    let startup_err = detect_startup_error(&lines, &boot_meta);

    // Pick the first fatal (Tier 1) blocker, respecting causal dominance:
    //  - Unresolved import / delay-load 0xc06d007e/7f wins over its own crash report.
    //  - Metal validation error wins over the SIGABRT it triggers.
    let mut primary: Option<BlockerDetail> = None;

    // Gather Tier 1 fatal candidates with causal suppression.
    let mut tier1: Vec<(usize, u8, BlockerDetail)> = Vec::new();
    if let Some(u) = unresolved.clone() {
        tier1.push((u.log_line.unwrap_or(usize::MAX), 0, u));
    }
    if let Some(m) = metal_val.clone() {
        tier1.push((m.log_line.unwrap_or(usize::MAX), 1, m));
    }
    if let Some(p) = panic_abort.clone() {
        // If Metal validation fired, don't let the resulting SIGABRT overshadow it.
        let shadowed_by_metal = metal_val.is_some() && p.subkind == "sigabrt";
        if !shadowed_by_metal {
            tier1.push((p.log_line.unwrap_or(usize::MAX), 2, p));
        }
    }
    if let Some(g) = guest_crash.clone() {
        // Don't let a generic crash report shadow an unresolved import or delay-load exception
        // or a Rust panic / Metal validation abort.
        let shadowed_by_unresolved = unresolved.is_some();
        let shadowed_by_metal = metal_val.is_some() && g.subkind == "host_fault";
        if !shadowed_by_unresolved && !shadowed_by_metal {
            tier1.push((g.log_line.unwrap_or(usize::MAX), 3, g));
        }
    }

    if !tier1.is_empty() {
        tier1.sort_by_key(|(line, prio, _)| (*line, *prio));
        primary = Some(tier1.remove(0).2);
    }

    // Tier 2: Timeout / hang (when no Tier 1 fatal blocker terminated the process).
    if primary.is_none() && is_timeout {
        let last_activity = extract_last_activity(&lines, 8);
        let timeout_line = lines
            .iter()
            .find(|l| {
                l.msg.contains("[run-limit] timed out after")
                    || l.msg.contains("port-runner: timed out after")
                    || (l.msg.contains("CRASH REPORT:")
                        && (l.msg.contains("stall") || l.msg.contains("hang")))
            })
            .map(|l| (l.line_no, l.raw.to_string()));
        let subkind = if boot_meta.exit_kind == "stall"
            || (crash_meta.present && crash_meta.kind == "stall")
            || lines.iter().any(|l| {
                l.msg.contains("CRASH REPORT:")
                    && (l.msg.contains("stall") || l.msg.contains("hang"))
            }) {
            "watchdog_stall"
        } else {
            "timeout"
        };
        let timeout_desc = input
            .target
            .timeout_secs
            .map(|s| format!("{s:.0}s"))
            .unwrap_or_else(|| "timeout".to_string());
        let last_hint = last_activity
            .last()
            .map(|s| truncate_chars(s, 120))
            .unwrap_or_else(|| "<no log output>".to_string());
        primary = Some(BlockerDetail {
            class: Some(BlockerClass::HangTimeout),
            subkind: subkind.to_string(),
            summary: format!(
                "Boot timed out after {timeout_desc} ({frames_presented} frame(s) presented); last activity: {last_hint}"
            ),
            log_line: timeout_line.as_ref().map(|(n, _)| *n),
            evidence: timeout_line
                .map(|(_, s)| s)
                .or_else(|| last_activity.last().cloned()),
            last_activity,
            ..BlockerDetail::default()
        });
    }

    // Tier 3: Non-crashing boot failures (D3D12/DXGI HRESULT failure or stub!/failing! hit).
    // Applies when the run did not present frames (or exited non-zero). Pick whichever happened first.
    let clean_rendered = frames_presented > 0
        && effective_exit_code.unwrap_or(0) == 0
        && input.signal.is_none()
        && !is_timeout;
    if primary.is_none() && !clean_rendered {
        let mut tier3: Vec<(usize, u8, BlockerDetail)> = Vec::new();
        if let Some(h) = hresult.clone() {
            tier3.push((h.log_line.unwrap_or(usize::MAX), 0, h));
        }
        if let Some(s) = stub_hit.clone() {
            tier3.push((s.log_line.unwrap_or(usize::MAX), 1, s));
        }
        if !tier3.is_empty() {
            tier3.sort_by_key(|(line, prio, _)| (*line, *prio));
            primary = Some(tier3.remove(0).2);
        }
    }

    // Tier 4: Startup error, unknown non-zero failure, or clean boot.
    let blocker = if let Some(b) = primary {
        b
    } else if let Some(se) = startup_err {
        se
    } else if input.signal.is_some() || effective_exit_code.is_some_and(|c| c != 0) {
        let last_activity = extract_last_activity(&lines, 8);
        let desc = match (effective_exit_code, input.signal) {
            (_, Some(sig)) => format!("port-runner terminated by signal {sig}"),
            (Some(c), None) => format!("port-runner exited with non-zero code {c}"),
            (None, None) => "port-runner failed with unknown status".to_string(),
        };
        BlockerDetail {
            class: Some(BlockerClass::UnknownFailure),
            subkind: "nonzero_exit".to_string(),
            summary: desc,
            log_line: lines.last().map(|l| l.line_no),
            evidence: last_activity.last().cloned(),
            last_activity,
            ..BlockerDetail::default()
        }
    } else {
        BlockerDetail {
            class: Some(BlockerClass::None),
            subkind: if frames_presented > 0 {
                "rendered_frames".to_string()
            } else {
                "clean_exit".to_string()
            },
            summary: if frames_presented > 0 {
                format!(
                    "No blocker detected; booted cleanly and presented {frames_presented} frame(s) ({graphics_api})"
                )
            } else {
                "No blocker detected; process exited cleanly (code 0)".to_string()
            },
            ..BlockerDetail::default()
        }
    };

    // Collect non-primary secondary signals in chronological order so root-cause chains stay visible.
    let primary_class = blocker.class();
    let mut secondary_signals: Vec<SecondarySignal> = Vec::new();
    for cand in [
        unresolved,
        metal_val,
        panic_abort,
        guest_crash,
        hresult,
        stub_hit,
    ]
    .into_iter()
    .flatten()
    {
        if cand.class() == primary_class && cand.log_line == blocker.log_line {
            continue;
        }
        // Skip crash report secondary when it is simply the crash dump for our unresolved import.
        if primary_class == BlockerClass::UnresolvedImport
            && cand.class() == BlockerClass::GuestCrash
        {
            continue;
        }
        // Skip SIGABRT secondary when Metal validation caused the abort.
        if primary_class == BlockerClass::MetalValidationError
            && cand.class() == BlockerClass::RustPanicOrAbort
            && cand.subkind == "sigabrt"
        {
            continue;
        }
        secondary_signals.push(SecondarySignal {
            class: cand.class(),
            subkind: cand.subkind,
            log_line: cand.log_line,
            summary: cand.summary,
            detail: cand.evidence.unwrap_or_default(),
        });
    }
    secondary_signals.extend(nonfatal_guest_signals);
    secondary_signals.sort_by_key(|s| s.log_line.unwrap_or(usize::MAX));

    let owner = attribute_owner(&blocker, &boot_meta);

    BootTriageReport {
        target: input.target.clone(),
        run: BootTriageRun {
            mode: if input.mode.is_empty() {
                "offline".to_string()
            } else {
                input.mode.clone()
            },
            exit_code: effective_exit_code,
            signal: input.signal,
            timed_out: is_timeout,
            elapsed_ms: input.elapsed_ms,
            frames_presented,
            graphics_api,
            log_lines: lines.len(),
            log_path: input.log_path.clone(),
            boot_report_found: boot_meta.present,
            crash_report_found: crash_meta.present,
        },
        blocker,
        owner,
        secondary_signals,
        missing_probes,
    }
}

// ---------------------------------------------------------------------------
// Per-class detectors
// ---------------------------------------------------------------------------

fn detect_unresolved_import(
    lines: &[ParsedLine<'_>],
    boot_meta: &BootReportMeta,
    crash_meta: &CrashReportMeta,
) -> Option<BlockerDetail> {
    for (idx, l) in lines.iter().enumerate() {
        let msg = l.msg;

        // 1. Static import trap thunk or COM method trap (NEVER match `emu: UNIMPLEMENTED mnemonic`).
        if let Some(pos) = msg.find("UNIMPLEMENTED ") {
            if msg.contains("UNIMPLEMENTED mnemonic ") {
                continue;
            }
            let after = msg[pos + "UNIMPLEMENTED ".len()..].trim();
            let (head, caller) = split_called_from(after);
            let head_clean = head
                .split(" (imported by ")
                .next()
                .unwrap_or(head)
                .split(" (COM method)")
                .next()
                .unwrap_or(head)
                .trim();

            if let Some((dll, sym)) = head_clean.split_once('!') {
                let dll = dll.trim().to_string();
                let sym = sym.trim().to_string();
                let bt = extract_crash_backtrace_after(lines, idx);
                let summary = match &caller {
                    Some(c) => format!("Unresolved import {dll}!{sym} called from {c}"),
                    None => format!("Unresolved import {dll}!{sym}"),
                };
                return Some(BlockerDetail {
                    class: Some(BlockerClass::UnresolvedImport),
                    subkind: "static_trap".to_string(),
                    summary,
                    log_line: Some(l.line_no),
                    evidence: Some(l.raw.to_string()),
                    dll: Some(dll),
                    symbol: Some(sym),
                    exception_code: Some("0xc0000139".to_string()),
                    location: caller,
                    backtrace: bt,
                    ..BlockerDetail::default()
                });
            }

            if !head_clean.is_empty() {
                let is_com = msg.contains("(COM method)") || head_clean.contains("::");
                let bt = extract_crash_backtrace_after(lines, idx);
                let summary = match &caller {
                    Some(c) => format!("Unimplemented COM method {head_clean} called from {c}"),
                    None => format!("Unimplemented COM method {head_clean}"),
                };
                return Some(BlockerDetail {
                    class: Some(BlockerClass::UnresolvedImport),
                    subkind: if is_com {
                        "com_trap".to_string()
                    } else {
                        "static_trap".to_string()
                    },
                    summary,
                    log_line: Some(l.line_no),
                    evidence: Some(l.raw.to_string()),
                    symbol: Some(head_clean.to_string()),
                    location: caller,
                    backtrace: bt,
                    ..BlockerDetail::default()
                });
            }
        }

        // 2. Delay-load exceptions: 0xc06d007e (module not found) / 0xc06d007f (proc not found).
        // Ignore DEBUG-level `RaiseException` traces unless no other signal exists.
        if l.level == "DEBUG" {
            continue;
        }
        let lower = msg.to_ascii_lowercase();
        let is_mod_not_found =
            lower.contains("0xc06d007e") || lower.contains("delayload module not found");
        let is_proc_not_found =
            lower.contains("0xc06d007f") || lower.contains("delayload proc not found");
        if is_mod_not_found || is_proc_not_found {
            let code = if is_mod_not_found {
                "0xc06d007e"
            } else {
                "0xc06d007f"
            };
            let subkind = if is_mod_not_found {
                "delay_load_module_not_found"
            } else {
                "delay_load_proc_not_found"
            };
            let loc = extract_module_rva(msg)
                .or_else(|| find_nearby_guest_location(lines, idx));

            // Back-correlate nearest prior LoadLibrary failure or GetProcAddress -> NULL.
            let (dll, sym) = if is_mod_not_found {
                (find_recent_loadlibrary_failure(lines, idx), None)
            } else {
                find_recent_proc_not_found(lines, idx)
                    .map(|(d, s)| (Some(d), Some(s)))
                    .unwrap_or((None, None))
            };
            let target_desc = match (&dll, &sym) {
                (Some(d), Some(s)) => format!(": {d}!{s}"),
                (Some(d), None) => format!(": {d}"),
                (None, Some(s)) => format!(": {s}"),
                (None, None) => String::new(),
            };
            let at_desc = loc
                .as_ref()
                .map(|c| format!(" at {c}"))
                .unwrap_or_default();
            let kind_label = if is_mod_not_found {
                "Delay-load module not found"
            } else {
                "Delay-load procedure not found"
            };
            let bt = extract_crash_backtrace_after(lines, idx);
            return Some(BlockerDetail {
                class: Some(BlockerClass::UnresolvedImport),
                subkind: subkind.to_string(),
                summary: format!("{kind_label} ({code}){target_desc}{at_desc}"),
                log_line: Some(l.line_no),
                evidence: Some(l.raw.to_string()),
                dll,
                symbol: sym,
                exception_code: Some(code.to_string()),
                location: loc,
                backtrace: bt,
                ..BlockerDetail::default()
            });
        }
    }

    // Fallback to structured crash/boot report JSON if log lines were truncated.
    if let Some(ref code) = crash_meta.exception_code {
        let lower = code.to_ascii_lowercase();
        if lower == "0xc06d007e" || lower == "0xc06d007f" {
            let is_mod = lower == "0xc06d007e";
            let (dll, sym) = if is_mod {
                (find_recent_loadlibrary_failure(lines, lines.len()), None)
            } else {
                find_recent_proc_not_found(lines, lines.len())
                    .map(|(d, s)| (Some(d), Some(s)))
                    .unwrap_or((None, None))
            };
            let loc = crash_meta
                .guest_symbol
                .clone()
                .or_else(|| crash_meta.top_frame_symbol.clone());
            return Some(BlockerDetail {
                class: Some(BlockerClass::UnresolvedImport),
                subkind: if is_mod {
                    "delay_load_module_not_found".to_string()
                } else {
                    "delay_load_proc_not_found".to_string()
                },
                summary: format!(
                    "Delay-load {} ({lower})",
                    if is_mod {
                        "module not found"
                    } else {
                        "procedure not found"
                    }
                ),
                evidence: crash_meta.exception_desc.clone(),
                dll,
                symbol: sym,
                exception_code: Some(lower),
                location: loc,
                ..BlockerDetail::default()
            });
        }
    }

    if crash_meta.present
        && (crash_meta.kind == "unimplemented_import"
            || crash_meta
                .title
                .to_ascii_lowercase()
                .starts_with("unimplemented import "))
    {
        let prefix = "unimplemented import ";
        let raw_target = if crash_meta
            .title
            .get(..prefix.len())
            .is_some_and(|p| p.eq_ignore_ascii_case(prefix))
        {
            &crash_meta.title[prefix.len()..]
        } else {
            &crash_meta.title
        }
        .trim();
        let (head, caller_in_title) = split_called_from(raw_target);
        let head = head.trim_end_matches(" (").trim();
        let loc = caller_in_title
            .or_else(|| {
                crash_meta
                    .top_frame_symbol
                    .as_deref()
                    .and_then(extract_module_rva)
            })
            .or_else(|| {
                crash_meta
                    .guest_symbol
                    .as_deref()
                    .and_then(extract_module_rva)
            })
            .or_else(|| crash_meta.top_frame_symbol.clone())
            .or_else(|| crash_meta.guest_symbol.clone());
        let (dll, sym) = match head.split_once('!') {
            Some((d, s)) => (Some(d.trim().to_string()), Some(s.trim().to_string())),
            None => (None, (!head.is_empty()).then(|| head.to_string())),
        };
        let is_com = dll.is_none() && sym.as_deref().is_some_and(|s| s.contains("::"));
        let target_str = if head.is_empty() { "<unknown>" } else { head };
        let summary = match &loc {
            Some(c) if is_com => format!("Unimplemented COM method {target_str} called from {c}"),
            None if is_com => format!("Unimplemented COM method {target_str}"),
            Some(c) => format!("Unresolved import {target_str} called from {c}"),
            None => format!("Unresolved import {target_str}"),
        };
        return Some(BlockerDetail {
            class: Some(BlockerClass::UnresolvedImport),
            subkind: if is_com {
                "com_trap".to_string()
            } else {
                "static_trap".to_string()
            },
            summary,
            evidence: Some(crash_meta.title.clone()),
            dll,
            symbol: sym,
            exception_code: (!is_com).then(|| "0xc0000139".to_string()),
            location: loc,
            backtrace: crash_meta.frames.clone(),
            ..BlockerDetail::default()
        });
    }

    if boot_meta.exit_kind == "unimplemented_import" && !boot_meta.exit_detail.is_empty() {
        let (head, caller) = split_called_from(&boot_meta.exit_detail);
        let head = head.trim_end_matches(" (").trim();
        let (dll, sym) = match head.split_once('!') {
            Some((d, s)) => (Some(d.trim().to_string()), Some(s.trim().to_string())),
            None => (None, Some(head.to_string())),
        };
        return Some(BlockerDetail {
            class: Some(BlockerClass::UnresolvedImport),
            subkind: "static_trap".to_string(),
            summary: format!("Unresolved import {}", boot_meta.exit_detail),
            evidence: Some(boot_meta.exit_detail.clone()),
            dll,
            symbol: sym,
            exception_code: Some("0xc0000139".to_string()),
            location: caller,
            ..BlockerDetail::default()
        });
    }

    None
}

fn detect_metal_validation(lines: &[ParsedLine<'_>]) -> Option<BlockerDetail> {
    for (idx, l) in lines.iter().enumerate() {
        let msg = l.msg;
        let low = msg.to_ascii_lowercase();
        let has_failure_word = low.contains("error")
            || low.contains("failed")
            || low.contains("assertion")
            || low.contains("abort")
            || low.contains("invalid")
            || low.contains("violation")
            || low.contains("fatal");
        // Exclude Apple's informational startup banners printed whenever `--metal-validation`
        // (`MTL_DEBUG_LAYER=1`) or GPU shader validation is enabled (e.g. `Metal API Validation Enabled`).
        if (low.contains("validation enabled") || low.contains("validation: enabled"))
            && !has_failure_word
        {
            continue;
        }
        let is_mtl_debug = msg.contains("MTLDebug")
            || msg.contains("MTL4Debug")
            || msg.contains("MTLGPUDebug")
            || msg.contains("MTLReportFailure")
            || (msg.contains("Metal API Validation") && has_failure_word)
            || msg.contains("validateWithDevice")
            || msg.contains("validateCommonDrawErrors")
            || msg.contains("validateVertexFunction")
            || msg.contains("validateFragmentFunction")
            || msg.contains("Execution of the command buffer was aborted due to an error during execution")
            || msg.contains("Metal 4 not supported")
            || ((msg.contains("failed assertion") || msg.contains("Validation Error"))
                && (msg.contains("MTL") || msg.contains("Metal") || msg.contains("AGX")));

        if is_mtl_debug {
            let bt = collect_backtrace_after(lines, idx);
            let subkind = if msg.contains("failed assertion") || msg.contains("MTLDebug") || msg.contains("MTL4Debug") {
                "mtl_debug_assertion"
            } else if msg.contains("command buffer was aborted") {
                "gpu_command_buffer_abort"
            } else {
                "metal_validation"
            };
            return Some(BlockerDetail {
                class: Some(BlockerClass::MetalValidationError),
                subkind: subkind.to_string(),
                summary: format!("Metal validation error: {}", truncate_chars(msg.trim(), 160)),
                log_line: Some(l.line_no),
                evidence: Some(l.raw.to_string()),
                backtrace: bt,
                ..BlockerDetail::default()
            });
        }
    }
    None
}

fn detect_rust_panic_or_abort(
    lines: &[ParsedLine<'_>],
    exit_code: Option<i32>,
    signal: Option<i32>,
) -> Option<BlockerDetail> {
    // 1. Explicit Rust panic (custom port-loader panic hook or standard libstd panic format).
    for (idx, l) in lines.iter().enumerate() {
        let msg = l.msg;
        let is_custom_panic = msg.starts_with("PANIC on thread ");
        let is_std_panic = msg.starts_with("thread '") && msg.contains(" panicked at ");
        let is_fatal_rt = msg.starts_with("fatal runtime error:");
        if is_custom_panic || is_std_panic || is_fatal_rt {
            let mut summary_line = msg.trim().to_string();
            // In Rust 1.73+, the panic payload is on the line immediately after `thread '...' panicked at file:line:col:`.
            if is_std_panic && summary_line.ends_with(':') {
                if let Some(next) = lines.get(idx + 1) {
                    let next_msg = next.msg.trim();
                    if !next_msg.is_empty()
                        && !next_msg.starts_with("note:")
                        && !next_msg.starts_with("stack backtrace:")
                    {
                        summary_line.push(' ');
                        summary_line.push_str(next_msg);
                    }
                }
            }
            let bt = collect_backtrace_after(lines, idx);
            let location = extract_rust_file_loc(&summary_line)
                .or_else(|| bt.first().cloned());
            return Some(BlockerDetail {
                class: Some(BlockerClass::RustPanicOrAbort),
                subkind: if is_fatal_rt {
                    "fatal_runtime_error".to_string()
                } else {
                    "rust_panic".to_string()
                },
                summary: format!("Rust panic: {}", truncate_chars(&summary_line, 180)),
                log_line: Some(l.line_no),
                evidence: Some(l.raw.to_string()),
                location,
                backtrace: bt,
                ..BlockerDetail::default()
            });
        }
    }

    // 2. SIGABRT / C++ / ObjC abort lines or signal 6 / exit code 134.
    for (idx, l) in lines.iter().enumerate() {
        let msg = l.msg;
        let is_abort_line = msg.contains("SIGABRT")
            || msg.contains("Abort trap: 6")
            || msg.contains("libc++abi: terminating")
            || msg.contains("Terminating app due to uncaught exception")
            || msg.starts_with("Assertion failed:");
        if is_abort_line {
            let bt = collect_backtrace_after(lines, idx);
            return Some(BlockerDetail {
                class: Some(BlockerClass::RustPanicOrAbort),
                subkind: "sigabrt".to_string(),
                summary: format!("Process aborted (SIGABRT): {}", truncate_chars(msg.trim(), 160)),
                log_line: Some(l.line_no),
                evidence: Some(l.raw.to_string()),
                location: bt.first().cloned(),
                backtrace: bt,
                ..BlockerDetail::default()
            });
        }
    }

    if signal == Some(6) || exit_code == Some(134) {
        let bt = collect_any_backtrace(lines);
        let last = extract_last_activity(lines, 6);
        let detail_hint = last
            .last()
            .map(|s| format!(": {}", truncate_chars(s, 140)))
            .unwrap_or_default();
        return Some(BlockerDetail {
            class: Some(BlockerClass::RustPanicOrAbort),
            subkind: "sigabrt".to_string(),
            summary: format!("Process terminated with SIGABRT (signal 6){detail_hint}"),
            log_line: lines.last().map(|l| l.line_no),
            evidence: last.last().cloned(),
            location: bt.first().cloned(),
            backtrace: bt,
            last_activity: last,
            ..BlockerDetail::default()
        });
    }

    None
}

fn is_guest_crash_trigger(msg: &str) -> Option<(&'static str, bool)> {
    if msg.contains("CRASH REPORT:") {
        if msg.contains("unimplemented import ") || msg.contains("stall") || msg.contains("hang") {
            return None;
        }
        if msg.contains("host fault") {
            return Some(("host_fault", false));
        }
        if msg.contains("__fastfail") {
            return Some(("fastfail", false));
        }
        let provisional = msg.to_ascii_lowercase().contains("unhandled");
        return Some(("unhandled_exception", provisional));
    }
    if msg.starts_with("UNHANDLED ") || msg.starts_with("emu: UNHANDLED guest ") {
        let low = msg.to_ascii_lowercase();
        if low.contains("0xc06d007e") || low.contains("0xc06d007f") {
            return None;
        }
        return Some(("unhandled_exception", true));
    }
    if msg.starts_with("FAST FAIL ") {
        return Some(("fastfail", false));
    }
    if msg.starts_with("FATAL: ") {
        let sub = if msg.contains("host fault") {
            "host_fault"
        } else if msg.contains("__fastfail") {
            "fastfail"
        } else {
            "unhandled_exception"
        };
        return Some((sub, false));
    }
    None
}

fn detect_guest_crash(
    lines: &[ParsedLine<'_>],
    crash_meta: &CrashReportMeta,
    exit_code: Option<i32>,
    signal: Option<i32>,
    is_timeout: bool,
    frames_presented: u64,
) -> (Option<BlockerDetail>, Vec<SecondarySignal>) {
    // 1. Unimplemented x86-64 opcode (`EXIT_UNIMPL_OP = 86` when fatal; secondary signal when
    //    `PORT_FAIL_ON_UNIMPL_OP=0` logged the opcode and continued).
    let has_unimpl_crash_report = (crash_meta.present
        && crash_meta
            .title
            .to_ascii_lowercase()
            .contains("unimplemented x86 instruction"))
        || lines.iter().any(|l| {
            l.msg.contains("CRASH REPORT:")
                && l.msg
                    .to_ascii_lowercase()
                    .contains("unimplemented x86 instruction")
        });

    let mut nonfatal_signals: Vec<SecondarySignal> = Vec::new();
    for (idx, l) in lines.iter().enumerate() {
        if let Some(pos) = l.msg.find("UNIMPLEMENTED mnemonic ") {
            let rest = &l.msg[pos + "UNIMPLEMENTED mnemonic ".len()..];
            let mn = rest.split_whitespace().next().unwrap_or("unknown");
            let loc = extract_module_rva(rest);
            let insn = extract_backtick_span(rest);
            let at_str = loc
                .as_deref()
                .map(|s| format!(" at {s}"))
                .unwrap_or_default();
            let insn_str = insn
                .as_deref()
                .map(|s| format!(" (`{s}`)"))
                .unwrap_or_default();
            let is_fatal = l.msg.contains("aborting")
                || l.msg.contains("exit 86")
                || exit_code == Some(86)
                || has_unimpl_crash_report;
            if is_fatal {
                let bt = extract_crash_backtrace_after(lines, idx);
                return (
                    Some(BlockerDetail {
                        class: Some(BlockerClass::GuestCrash),
                        subkind: "unimplemented_x86_op".to_string(),
                        summary: format!("Unimplemented x86-64 opcode {mn}{at_str}{insn_str}"),
                        log_line: Some(l.line_no),
                        evidence: Some(l.raw.to_string()),
                        symbol: Some(mn.to_string()),
                        exception_code: Some("EXIT_UNIMPL_OP(86)".to_string()),
                        location: loc,
                        faulting_instruction: insn,
                        backtrace: bt,
                        ..BlockerDetail::default()
                    }),
                    nonfatal_signals,
                );
            }
            if nonfatal_signals.len() < 4 {
                nonfatal_signals.push(SecondarySignal {
                    class: BlockerClass::GuestCrash,
                    subkind: "unimplemented_x86_op_nonfatal".to_string(),
                    log_line: Some(l.line_no),
                    summary: format!(
                        "Nonfatal unimplemented x86-64 opcode {mn}{at_str}{insn_str} (continued)"
                    ),
                    detail: l.raw.to_string(),
                });
            }
        }
    }

    // 2. Crash report / unhandled exception / FATAL / FAST FAIL blocks.
    //    `fatal_unhandled` and emulated unhandled dispatch in `win32-rt/src/unwind.rs` log
    //    `UNHANDLED` and write a provisional `CRASH REPORT:` *before* invoking the application's
    //    unhandled exception filter. If the filter returns `EXCEPTION_CONTINUE_EXECUTION` (-1),
    //    execution resumes (with an explicit `continued execution` log on the emulated path, or
    //    `restore_context` followed by continued execution / clean exit / a later event on the
    //    native path). Demote recovered exceptions to secondary signals rather than Tier 1 crashes.
    let clean_exit = exit_code == Some(0) && signal.is_none() && !is_timeout;
    let reached_max_frames = lines
        .iter()
        .any(|l| l.msg.contains("[run-limit] reached --max-frames"));

    let mut recovered_any = false;
    let mut scan_idx = 0usize;
    let mut fatal_trigger: Option<(usize, &'static str)> = None;

    while scan_idx < lines.len() {
        let Some((mut subkind, provisional)) = is_guest_crash_trigger(lines[scan_idx].msg) else {
            scan_idx += 1;
            continue;
        };
        let tidx = scan_idx;
        let max_end = (tidx + 140).min(lines.len());
        let mut block_end = tidx + 1;
        while block_end < max_end {
            let m = lines[block_end].msg;
            if m.contains("end of crash report") {
                block_end += 1;
                break;
            }
            // Do not merge a second separate `UNHANDLED` / `FATAL` trigger into the same block.
            if block_end > tidx && !m.contains("CRASH REPORT:") && is_guest_crash_trigger(m).is_some()
            {
                break;
            }
            block_end += 1;
        }
        // If no `end of crash report` appeared, don't jump too far past a 1-line trigger.
        if block_end == max_end
            && !lines[tidx..block_end]
                .iter()
                .any(|l| l.msg.contains("CRASH REPORT:"))
        {
            block_end = (tidx + 1).min(lines.len());
        }

        // Inspect lines immediately following this crash block up to any next crash/fatal event.
        let mut saw_filter_call = false;
        let mut saw_continued = false;
        let mut has_later_fatal_event = false;
        for l in &lines[block_end..] {
            let m = l.msg;
            if m.contains("unhandled exception filter continued execution") {
                saw_continued = true;
                saw_filter_call = true;
                continue;
            }
            if m.contains("calling the application's unhandled exception filter") {
                saw_filter_call = true;
                continue;
            }
            if is_guest_crash_trigger(m).is_some()
                || m.starts_with("PANIC on thread ")
                || (m.starts_with("thread '") && m.contains(" panicked at "))
                || (m.contains("UNIMPLEMENTED ") && !m.contains("UNIMPLEMENTED mnemonic "))
            {
                has_later_fatal_event = true;
                break;
            }
        }

        let is_recovered = provisional
            && (saw_continued
                || (saw_filter_call
                    && (has_later_fatal_event || is_timeout || clean_exit || reached_max_frames))
                || (clean_exit && (frames_presented > 0 || reached_max_frames)));

        if is_recovered {
            recovered_any = true;
            let ex_desc = lines[tidx..block_end]
                .iter()
                .find_map(|l| extract_exception_code_and_name(l.msg))
                .map(|(c, n)| if n.is_empty() { c } else { format!("{n} ({c})") })
                .unwrap_or_else(|| "unhandled_exception".to_string());
            let loc_desc = lines[tidx..block_end]
                .iter()
                .find_map(|l| extract_module_rva(l.msg))
                .map(|s| format!(" at {s}"))
                .unwrap_or_default();
            if nonfatal_signals.len() < 4 {
                nonfatal_signals.push(SecondarySignal {
                    class: BlockerClass::GuestCrash,
                    subkind: "recovered_exception".to_string(),
                    log_line: Some(lines[tidx].line_no),
                    summary: format!(
                        "Recovered guest exception {ex_desc}{loc_desc} (continued execution)"
                    ),
                    detail: lines[tidx].raw.to_string(),
                });
            }
            scan_idx = block_end;
            continue;
        }

        // Non-recovered fatal crash trigger found.
        if lines[tidx].msg.starts_with("FAST FAIL ") || lines[tidx].msg.contains("__fastfail") {
            subkind = "fastfail";
        }
        fatal_trigger = Some((tidx, subkind));
        break;
    }

    if let Some((tidx, mut subkind)) = fatal_trigger {
        let window_end = (tidx + 120).min(lines.len());
        let slice = &lines[tidx..window_end];

        let mut exception_code: Option<String> = None;
        let mut exception_name: Option<String> = None;
        let mut guest_loc: Option<String> = None;
        let mut fallback_loc: Option<String> = None;
        let mut faulting_insn: Option<String> = None;
        let mut av_detail: Option<String> = None;
        let mut in_disasm = false;

        for l in slice {
            let m = l.msg;
            if m.contains("end of crash report") {
                break;
            }
            if exception_code.is_none() {
                if let Some((code, name)) = extract_exception_code_and_name(m) {
                    exception_code = Some(code);
                    if !name.is_empty() {
                        exception_name = Some(name);
                    }
                }
            }
            if av_detail.is_none() && m.starts_with("access violation:") {
                av_detail = Some(m.to_string());
                subkind = "access_violation";
            }
            if m.starts_with("guest interpreter state") {
                if let Some(loc) = extract_module_rva(m) {
                    guest_loc = Some(loc);
                }
            }
            if fallback_loc.is_none() {
                if let Some(loc) = extract_module_rva(m) {
                    fallback_loc = Some(loc);
                }
            }
            if m.starts_with("disassembly @rip:") {
                in_disasm = true;
                continue;
            }
            if in_disasm {
                let t = m.trim_start();
                if let Some(after_arrow) = t.strip_prefix("=>") {
                    if faulting_insn.is_none() {
                        faulting_insn = Some(clean_disasm_instruction(after_arrow.trim()));
                    }
                    in_disasm = false;
                    continue;
                } else if !t.is_empty() && !t.chars().next().is_some_and(|c| c.is_ascii_hexdigit())
                {
                    in_disasm = false;
                }
            }
        }

        let mut bt = extract_crash_backtrace_after(lines, tidx);
        if guest_loc.is_none() {
            guest_loc = bt.iter().find_map(|f| extract_module_rva(f));
        }

        // Enrich from crash JSON sidecar if available.
        if exception_code.is_none() {
            exception_code = crash_meta.exception_code.clone();
        }
        if guest_loc.is_none() {
            guest_loc = crash_meta
                .guest_symbol
                .as_deref()
                .and_then(extract_module_rva)
                .or_else(|| {
                    crash_meta
                        .top_frame_symbol
                        .as_deref()
                        .and_then(extract_module_rva)
                });
        }
        if faulting_insn.is_none() {
            faulting_insn = crash_meta.faulting_insn.clone();
        }
        if bt.is_empty() {
            bt = crash_meta.frames.clone();
        }

        if let Some(ref ec) = exception_code {
            match ec.as_str() {
                "0xc0000005" => subkind = "access_violation",
                "0xc000001d" => subkind = "illegal_instruction",
                "0xc0000409" => subkind = "fastfail",
                "0xc00000fd" => subkind = "stack_overflow",
                "0xc0000094" => subkind = "integer_divide_by_zero",
                "0xe06d7363" => subkind = "unhandled_cxx_exception",
                _ => {}
            }
        }

        let location = guest_loc
            .or(fallback_loc)
            .or_else(|| crash_meta.guest_symbol.clone())
            .or_else(|| crash_meta.top_frame_symbol.clone());

        let ex_str = match (&exception_code, &exception_name) {
            (Some(c), Some(n)) => format!("{n} ({c})"),
            (Some(c), None) => c.clone(),
            (None, Some(n)) => n.clone(),
            (None, None) => subkind.to_string(),
        };
        let loc_str = location
            .as_deref()
            .map(|s| format!(" at {s}"))
            .unwrap_or_default();
        let insn_str = faulting_insn
            .as_deref()
            .map(|s| format!(" [`{s}`]"))
            .unwrap_or_default();
        let av_str = av_detail
            .as_deref()
            .map(|s| format!(" ({s})"))
            .unwrap_or_default();

        return (
            Some(BlockerDetail {
                class: Some(BlockerClass::GuestCrash),
                subkind: subkind.to_string(),
                summary: format!("Guest crash: {ex_str}{loc_str}{insn_str}{av_str}"),
                log_line: Some(lines[tidx].line_no),
                evidence: Some(lines[tidx].raw.to_string()),
                exception_code,
                location,
                faulting_instruction: faulting_insn,
                backtrace: bt,
                ..BlockerDetail::default()
            }),
            nonfatal_signals,
        );
    }

    // Fallback: crash-*.json present even without log markers, or exit code 86.
    // Skip if a provisional crash report in the log was already recognized as recovered, or if
    // the process exited cleanly after presenting frames / reaching --max-frames.
    let sidecar_is_provisional_recovered = recovered_any
        || (clean_exit
            && (frames_presented > 0
                || reached_max_frames
                || lines.iter().any(|l| {
                    l.msg.contains("calling the application's unhandled exception filter")
                        || l.msg.contains("unhandled exception filter continued execution")
                })));
    if crash_meta.present && crash_meta.kind == "crash" {
        if sidecar_is_provisional_recovered {
            if !recovered_any && nonfatal_signals.len() < 4 {
                let loc = crash_meta
                    .guest_symbol
                    .as_deref()
                    .and_then(extract_module_rva)
                    .or_else(|| crash_meta.top_frame_symbol.as_deref().and_then(extract_module_rva));
                let ex = crash_meta
                    .exception_code
                    .as_deref()
                    .unwrap_or("unhandled_exception");
                let loc_str = loc.map(|s| format!(" at {s}")).unwrap_or_default();
                nonfatal_signals.push(SecondarySignal {
                    class: BlockerClass::GuestCrash,
                    subkind: "recovered_exception".to_string(),
                    log_line: None,
                    summary: format!(
                        "Recovered guest exception {ex}{loc_str} (continued execution)"
                    ),
                    detail: crash_meta
                        .exception_desc
                        .clone()
                        .unwrap_or_else(|| crash_meta.title.clone()),
                });
            }
            return (None, nonfatal_signals);
        }
        let loc = crash_meta
            .guest_symbol
            .as_deref()
            .and_then(extract_module_rva)
            .or_else(|| crash_meta.guest_symbol.clone())
            .or_else(|| crash_meta.top_frame_symbol.clone());
        let insn = crash_meta.faulting_insn.clone();
        let ex = crash_meta
            .exception_code
            .clone()
            .unwrap_or_else(|| "crash".to_string());
        return (
            Some(BlockerDetail {
                class: Some(BlockerClass::GuestCrash),
                subkind: "crash_report".to_string(),
                summary: format!(
                    "Guest crash ({ex}) at {}{}",
                    loc.as_deref().unwrap_or("<unknown>"),
                    insn.as_deref()
                        .map(|s| format!(" [`{s}`]"))
                        .unwrap_or_default()
                ),
                evidence: crash_meta
                    .exception_desc
                    .clone()
                    .or_else(|| Some(crash_meta.title.clone())),
                exception_code: crash_meta.exception_code.clone(),
                location: loc,
                faulting_instruction: insn,
                backtrace: crash_meta.frames.clone(),
                ..BlockerDetail::default()
            }),
            nonfatal_signals,
        );
    }

    if exit_code == Some(86) {
        return (
            Some(BlockerDetail {
                class: Some(BlockerClass::GuestCrash),
                subkind: "unimplemented_x86_op".to_string(),
                summary: "Unimplemented x86-64 opcode (exit 86)".to_string(),
                exception_code: Some("EXIT_UNIMPL_OP(86)".to_string()),
                ..BlockerDetail::default()
            }),
            nonfatal_signals,
        );
    }

    (None, nonfatal_signals)
}

fn detect_d3d12_dxgi_hresult(lines: &[ParsedLine<'_>]) -> Option<BlockerDetail> {
    for l in lines {
        let msg = l.msg;
        // Skip benign INFO/DEBUG capability probes (e.g. D3D12GetDebugInterface -> E_NOINTERFACE,
        // EnumWarpAdapter -> DXGI_ERROR_NOT_FOUND, CheckInterfaceSupport -> DXGI_ERROR_UNSUPPORTED).
        if matches!(l.level, "INFO" | "DEBUG" | "TRACE") {
            continue;
        }
        // `stub!` lines belong strictly to `detect_stub_hit` (avoid double-classifying e.g.
        // `STUB create_geometry_shader_so: ID3D11Device::... -> E_NOTIMPL`).
        if l.target == "stub" || msg.starts_with("STUB ") {
            continue;
        }
        // Skip benign DXGI_STATUS_OCCLUDED (non-error status code 0x087A0001).
        if msg.contains("DXGI_STATUS_") {
            continue;
        }
        // Skip benign non-ERROR COM QueryInterface version probes (games routinely probe optional
        // newer ID3D11DeviceN / ID3D12DeviceN / IDXGI* interfaces and fall back on E_NOINTERFACE).
        if l.level != "ERROR"
            && msg.contains("QueryInterface(")
            && (msg.contains("E_NOINTERFACE") || msg.to_ascii_lowercase().contains("0x80004002"))
        {
            continue;
        }

        let is_d3d_target = l.target == "d3d"
            || l.target.starts_with("d3d12")
            || l.target.starts_with("d3d11")
            || l.target.starts_with("dxgi");
        let mentions_d3d_api = msg.contains("D3D12")
            || msg.contains("ID3D12")
            || msg.contains("DXGI")
            || msg.contains("IDXGI")
            || msg.contains("D3D11")
            || msg.contains("ID3D11")
            || msg.contains("CreateSwapChain")
            || msg.contains("CreateCommittedResource")
            || msg.contains("CreateGraphicsPipelineState")
            || msg.contains("CreateComputePipelineState")
            || msg.contains("CreateQueryHeap")
            || msg.contains("CreateDescriptorHeap")
            || msg.contains("CreateCommandQueue")
            || msg.contains("CreateCommandList")
            || msg.contains("CreateRootSignature")
            || msg.contains("ResizeBuffers");

        if !is_d3d_target && !mentions_d3d_api {
            continue;
        }

        if let Some((hr_hex, hr_name)) = extract_failing_hresult(msg) {
            let hr_display = match hr_name {
                Some(n) => format!("{hr_hex} ({n})"),
                None => hr_hex.clone(),
            };
            let call = msg
                .split("->")
                .next()
                .unwrap_or(msg)
                .trim()
                .to_string();
            let subkind = if msg.contains("d3d11:") || msg.contains("D3D11") || msg.contains("ID3D11") {
                "d3d11_hresult"
            } else if msg.contains("DXGI")
                || msg.contains("IDXGI")
                || msg.contains("SwapChain")
                || msg.contains("ResizeBuffers")
                || hr_hex.starts_with("0x887a")
            {
                "dxgi_hresult"
            } else {
                "d3d12_hresult"
            };
            return Some(BlockerDetail {
                class: Some(BlockerClass::D3d12DxgiHresult),
                subkind: subkind.to_string(),
                summary: format!(
                    "D3D12/DXGI HRESULT failure {hr_display}: {}",
                    truncate_chars(&call, 120)
                ),
                log_line: Some(l.line_no),
                evidence: Some(l.raw.to_string()),
                symbol: Some(truncate_chars(&call, 120)),
                hresult: Some(hr_display),
                ..BlockerDetail::default()
            });
        }
    }
    None
}

fn detect_stub_hit(lines: &[ParsedLine<'_>], boot_meta: &BootReportMeta) -> Option<BlockerDetail> {
    for l in lines {
        let msg = l.msg;
        if let Some(pos) = msg.find("STUB ") {
            // Ensure `STUB ` starts the message or follows whitespace (not a substring of another word).
            if pos > 0
                && msg.as_bytes()[pos - 1].is_ascii_alphanumeric()
            {
                continue;
            }
            let after = &msg[pos + "STUB ".len()..];
            let (api_raw, detail_raw) = match after.split_once(':') {
                Some((a, d)) => (a.trim(), d.trim()),
                None => (after.split_whitespace().next().unwrap_or(""), after.trim()),
            };
            if api_raw.is_empty() {
                continue;
            }
            let is_failing = detail_raw.contains(" -> ") && detail_raw.contains("(last error ");
            let matched_meta = boot_meta
                .stubs
                .iter()
                .find(|s| s.api.eq_ignore_ascii_case(api_raw));
            let subkind = matched_meta
                .map(|m| m.kind.as_str())
                .unwrap_or(if is_failing { "failing" } else { "stub" });
            let dll = matched_meta
                .map(|m| m.dll.clone())
                .filter(|s| !s.is_empty());
            let caller = matched_meta
                .map(|m| m.first_caller.clone())
                .filter(|s| !s.is_empty());
            let qual = match &dll {
                Some(d) => format!("{d}!{api_raw}"),
                None => api_raw.to_string(),
            };
            let caller_note = caller
                .as_deref()
                .map(|c| format!(" (called from {c})"))
                .unwrap_or_default();
            return Some(BlockerDetail {
                class: Some(BlockerClass::StubHit),
                subkind: subkind.to_string(),
                summary: format!(
                    "{subkind}! hit on {qual}{caller_note}: {}",
                    truncate_chars(detail_raw, 120)
                ),
                log_line: Some(l.line_no),
                evidence: Some(l.raw.to_string()),
                dll,
                symbol: Some(api_raw.to_string()),
                location: caller,
                ..BlockerDetail::default()
            });
        }
    }

    // Fallback to boot_report.json stub registry snapshot if log filtered `stub` target.
    if let Some(first) = boot_meta.stubs.first() {
        let qual = if first.dll.is_empty() {
            first.api.clone()
        } else {
            format!("{}!{}", first.dll, first.api)
        };
        return Some(BlockerDetail {
            class: Some(BlockerClass::StubHit),
            subkind: first.kind.clone(),
            summary: format!(
                "{}! hit on {qual} ({} hit(s))",
                first.kind, first.hits
            ),
            evidence: Some(format!(
                "{qual} ({}:{}) -> {}",
                first.file, first.line, first.detail
            )),
            dll: (!first.dll.is_empty()).then(|| first.dll.clone()),
            symbol: Some(first.api.clone()),
            location: (!first.first_caller.is_empty()).then(|| first.first_caller.clone()),
            ..BlockerDetail::default()
        });
    }

    None
}

fn detect_startup_error(
    lines: &[ParsedLine<'_>],
    boot_meta: &BootReportMeta,
) -> Option<BlockerDetail> {
    for l in lines {
        if l.msg.contains("port-runner fatal startup error") {
            return Some(BlockerDetail {
                class: Some(BlockerClass::StartupError),
                subkind: "startup_error".to_string(),
                summary: truncate_chars(l.msg.trim(), 180),
                log_line: Some(l.line_no),
                evidence: Some(l.raw.to_string()),
                ..BlockerDetail::default()
            });
        }
    }
    if boot_meta.exit_kind == "startup_error" {
        return Some(BlockerDetail {
            class: Some(BlockerClass::StartupError),
            subkind: "startup_error".to_string(),
            summary: format!("port-runner startup error: {}", boot_meta.exit_detail),
            evidence: Some(boot_meta.exit_detail.clone()),
            ..BlockerDetail::default()
        });
    }
    None
}

// ---------------------------------------------------------------------------
// Ownership routing (`AGENTS.md` "Where code belongs" table)
// ---------------------------------------------------------------------------

fn attribute_owner(blocker: &BlockerDetail, boot_meta: &BootReportMeta) -> Option<OwnerHint> {
    match blocker.class() {
        BlockerClass::None => None,
        BlockerClass::UnresolvedImport => Some(owner_for_dll_or_symbol(
            blocker.dll.as_deref(),
            blocker.symbol.as_deref(),
            &blocker.subkind,
        )),
        BlockerClass::StubHit => {
            if let Some(sym) = blocker.symbol.as_deref() {
                if let Some(meta) = boot_meta
                    .stubs
                    .iter()
                    .find(|s| s.api.eq_ignore_ascii_case(sym))
                {
                    if !meta.file.is_empty() {
                        let path = if meta.line > 0 {
                            format!("{}:{}", meta.file, meta.line)
                        } else {
                            meta.file.clone()
                        };
                        return Some(OwnerHint {
                            crate_name: crate_from_path(&meta.file),
                            paths: vec![path],
                            area: area_for_dll(Some(&meta.dll), Some(sym)).to_string(),
                            reason: format!(
                                "Guest hit {}! stub {}!{} ({} hit(s))",
                                meta.kind, meta.dll, meta.api, meta.hits
                            ),
                            next_step: "Replace the stub!/failing! with real macOS behavior, add a unit test, and run `cargo opsdb -- win32-sync`.".to_string(),
                        });
                    }
                }
            }
            Some(owner_for_dll_or_symbol(
                blocker.dll.as_deref(),
                blocker.symbol.as_deref(),
                "stub_hit",
            ))
        }
        BlockerClass::MetalValidationError => Some(OwnerHint {
            crate_name: "crates/d3d12-metal".to_string(),
            paths: vec![
                "crates/d3d12-metal/src/metal/".to_string(),
                "crates/win32-rt/src/d3d12/".to_string(),
            ],
            area: "Direct3D 12 COM shim vs Metal 4 implementation".to_string(),
            reason: "Apple Metal validation layer reported an invalid Metal 4 call/state".to_string(),
            next_step: "Reproduce under `MTL_DEBUG_LAYER=1` and inspect the offending encoder/pipeline/resource state in `crates/d3d12-metal/`.".to_string(),
        }),
        BlockerClass::D3d12DxgiHresult => {
            let sym_low = blocker
                .symbol
                .as_deref()
                .unwrap_or("")
                .to_ascii_lowercase();
            if blocker.subkind == "d3d11_hresult" {
                Some(OwnerHint {
                    crate_name: "crates/win32-rt".to_string(),
                    paths: vec![
                        "crates/win32-rt/src/d3d11/".to_string(),
                        "crates/d3d12-metal/src/".to_string(),
                    ],
                    area: "Direct3D 11.0/11.1 runtime".to_string(),
                    reason: format!(
                        "D3D11 call returned failing HRESULT {}",
                        blocker.hresult.as_deref().unwrap_or("")
                    ),
                    next_step: "Inspect the D3D11 COM shim in `crates/win32-rt/src/d3d11/` and underlying Metal 4 mapping in `crates/d3d12-metal/`.".to_string(),
                })
            } else if sym_low.contains("pipelinestate")
                || sym_low.contains("shader")
                || sym_low.contains("d3dcompile")
            {
                Some(OwnerHint {
                    crate_name: "crates/dxil-metal".to_string(),
                    paths: vec![
                        "crates/dxil-metal/src/".to_string(),
                        "crates/d3d12-metal/src/metal/pipeline.rs".to_string(),
                        "crates/win32-rt/src/d3d12/".to_string(),
                    ],
                    area: "Shader translation or PSO cache / Metal 4 pipeline".to_string(),
                    reason: format!(
                        "Shader or pipeline state creation returned {}",
                        blocker.hresult.as_deref().unwrap_or("error HRESULT")
                    ),
                    next_step: "Check DXIL/FXC conversion in `crates/dxil-metal/` and PSO descriptor validation in `crates/d3d12-metal/`.".to_string(),
                })
            } else {
                Some(OwnerHint {
                    crate_name: "crates/win32-rt + crates/d3d12-metal".to_string(),
                    paths: vec![
                        "crates/win32-rt/src/d3d12/".to_string(),
                        "crates/win32-rt/src/dxgi/".to_string(),
                        "crates/d3d12-metal/src/".to_string(),
                    ],
                    area: "Direct3D 12 COM shim vs Metal 4 implementation".to_string(),
                    reason: format!(
                        "D3D12/DXGI call returned failing HRESULT {}",
                        blocker.hresult.as_deref().unwrap_or("")
                    ),
                    next_step: "Check the COM validation in `crates/win32-rt/src/{d3d12,dxgi}/` and backend implementation in `crates/d3d12-metal/`.".to_string(),
                })
            }
        }
        BlockerClass::RustPanicOrAbort => Some(owner_from_backtrace(
            &blocker.backtrace,
            blocker.location.as_deref(),
            &blocker.summary,
        )),
        BlockerClass::GuestCrash => {
            if blocker.subkind == "unimplemented_x86_op" {
                return Some(OwnerHint {
                    crate_name: "crates/x86-runtime".to_string(),
                    paths: vec![
                        "crates/x86-runtime/src/emu/".to_string(),
                        "crates/port-factory/src/ops_db/".to_string(),
                    ],
                    area: "x86-64 decoder, ARM64/NEON block JIT & interpreter".to_string(),
                    reason: format!(
                        "Guest executed unimplemented x86-64 instruction {}",
                        blocker
                            .faulting_instruction
                            .as_deref()
                            .or(blocker.symbol.as_deref())
                            .unwrap_or("")
                    ),
                    next_step: "Implement the opcode in `crates/x86-runtime/src/emu/` (generic interpreter + FastOp/JIT), add a differential test, and run `cargo opsdb -- sync`.".to_string(),
                });
            }
            if blocker.subkind == "host_fault" {
                return Some(owner_from_backtrace(
                    &blocker.backtrace,
                    blocker.location.as_deref(),
                    &blocker.summary,
                ));
            }
            Some(OwnerHint {
                crate_name: "crates/x86-runtime".to_string(),
                paths: vec![
                    "crates/x86-runtime/src/".to_string(),
                    "crates/win32-rt/src/".to_string(),
                ],
                area: "x86-64 CPU translation / FH4-SEH unwinding (or upstream Win32 return)".to_string(),
                reason: format!(
                    "Guest crashed ({}) at {}",
                    blocker.exception_code.as_deref().unwrap_or(&blocker.subkind),
                    blocker.location.as_deref().unwrap_or("<unknown>")
                ),
                next_step: "Re-test with `PORT_AOT=0` / `AW2_JIT=0` or `cargo opsdb -- bisect-runtime` to distinguish x86-runtime translation from an earlier Win32 return/struct bug.".to_string(),
            })
        }
        BlockerClass::HangTimeout => Some(owner_from_last_activity(&blocker.last_activity)),
        BlockerClass::StartupError => Some(OwnerHint {
            crate_name: "crates/port-loader".to_string(),
            paths: vec!["crates/port-loader/src/".to_string()],
            area: "Universal runner CLI (`port-runner`), `game.toml` loading, PE mapping".to_string(),
            reason: "port-runner failed during startup before guest execution".to_string(),
            next_step: "Check `--manifest` / `--game-dir` / `--exe` paths and PE mapping logic in `crates/port-loader/src/`.".to_string(),
        }),
        BlockerClass::UnknownFailure => Some(OwnerHint {
            crate_name: "crates/port-loader".to_string(),
            paths: vec![
                "crates/port-loader/src/".to_string(),
                "crates/win32-rt/src/".to_string(),
            ],
            area: "Universal runner / Win32 runtime".to_string(),
            reason: "Process exited non-zero without a known blocker signature".to_string(),
            next_step: "Re-run with `--log-out out/triage.log --env RUST_LOG=debug` to inspect the full log before exit.".to_string(),
        }),
    }
}

fn owner_for_dll_or_symbol(
    dll: Option<&str>,
    symbol: Option<&str>,
    subkind: &str,
) -> OwnerHint {
    let dll_low = dll.unwrap_or("").to_ascii_lowercase();
    let sym_low = symbol.unwrap_or("").to_ascii_lowercase();

    if dll_low.contains("d3d12")
        || dll_low.contains("dxgi")
        || sym_low.starts_with("id3d12")
        || sym_low.starts_with("idxgi")
        || sym_low.starts_with("d3d12")
    {
        return OwnerHint {
            crate_name: "crates/win32-rt + crates/d3d12-metal".to_string(),
            paths: vec![
                "crates/win32-rt/src/d3d12/".to_string(),
                "crates/win32-rt/src/dxgi/".to_string(),
                "crates/d3d12-metal/src/".to_string(),
            ],
            area: "Direct3D 12 COM shim vs Metal 4 implementation".to_string(),
            reason: format!(
                "Missing or stubbed D3D12/DXGI API ({})",
                format_dll_sym(dll, symbol)
            ),
            next_step: "Implement the COM/export entry in `crates/win32-rt/src/{d3d12,dxgi}/` and any Metal 4 backend support in `crates/d3d12-metal/`.".to_string(),
        };
    }

    if dll_low.contains("d3d11") || sym_low.starts_with("id3d11") || sym_low.starts_with("d3d11") {
        return OwnerHint {
            crate_name: "crates/win32-rt".to_string(),
            paths: vec![
                "crates/win32-rt/src/d3d11/".to_string(),
                "crates/d3d12-metal/src/".to_string(),
            ],
            area: "Direct3D 11.0/11.1 runtime".to_string(),
            reason: format!("Missing or stubbed D3D11 API ({})", format_dll_sym(dll, symbol)),
            next_step: "Implement the D3D11 entry in `crates/win32-rt/src/d3d11/` and run `cargo opsdb -- win32-sync`.".to_string(),
        };
    }

    if dll_low.contains("vulkan") || sym_low.starts_with("vk") {
        return OwnerHint {
            crate_name: "crates/win32-rt".to_string(),
            paths: vec!["crates/win32-rt/src/misc/vulkan_1.rs".to_string()],
            area: "Vulkan loader & VK_KHR_win32_surface bridge".to_string(),
            reason: format!("Missing or stubbed Vulkan API ({})", format_dll_sym(dll, symbol)),
            next_step: "Update the MoltenVK bridge in `crates/win32-rt/src/misc/vulkan_1.rs` and run `cargo opsdb -- win32-sync`.".to_string(),
        };
    }

    if dll_low.contains("d3dcompiler") || sym_low.starts_with("d3dcompile") || sym_low.starts_with("d3dreflect") {
        return OwnerHint {
            crate_name: "crates/dxil-metal + crates/win32-rt".to_string(),
            paths: vec![
                "crates/dxil-metal/src/".to_string(),
                "crates/win32-rt/src/misc/d3dcompiler_47.rs".to_string(),
            ],
            area: "Shader translation (DXIL, FXC SM 4/5, SPIR-V, WGSL) or PSO cache".to_string(),
            reason: format!("Missing or stubbed D3DCompiler API ({})", format_dll_sym(dll, symbol)),
            next_step: "Implement the compiler/reflection entry in `crates/win32-rt/src/misc/d3dcompiler_47.rs` / `crates/dxil-metal/`.".to_string(),
        };
    }

    if dll_low.contains("user32")
        || dll_low.contains("gdi32")
        || dll_low.contains("dwmapi")
        || dll_low.contains("imm32")
        || dll_low.contains("xinput")
        || dll_low.contains("dinput")
        || dll_low.contains("hid")
        || dll_low.contains("setupapi")
        || dll_low.contains("gameinput")
    {
        return OwnerHint {
            crate_name: "crates/win32-rt".to_string(),
            paths: vec![
                "crates/win32-rt/src/user32/".to_string(),
                "crates/win32-rt/src/macos/".to_string(),
                "crates/win32-rt/src/misc/".to_string(),
            ],
            area: "Windowing, AppKit, mouse/keyboard/raw input, XInput, DualSense HID".to_string(),
            reason: format!(
                "Missing or stubbed windowing/input API ({})",
                format_dll_sym(dll, symbol)
            ),
            next_step: "Implement the API in `crates/win32-rt/src/{user32,macos,misc}/` and run `cargo opsdb -- win32-sync`.".to_string(),
        };
    }

    if dll_low.contains("mmdevapi")
        || dll_low.contains("avrt")
        || dll_low.contains("xaudio")
        || dll_low.contains("fmod")
        || dll_low.contains("dsound")
        || dll_low.contains("winmm")
        || dll_low.contains("openal")
    {
        return OwnerHint {
            crate_name: "crates/win32-rt".to_string(),
            paths: vec![
                "crates/win32-rt/src/audio/".to_string(),
                "crates/win32-rt/src/misc/".to_string(),
            ],
            area: "WASAPI / CoreAudio / mmdevapi / avrt / audio middleware".to_string(),
            reason: format!("Missing or stubbed audio API ({})", format_dll_sym(dll, symbol)),
            next_step: "Implement the API in `crates/win32-rt/src/{audio,misc}/` and run `cargo opsdb -- win32-sync`.".to_string(),
        };
    }

    let action = if subkind == "delay_load_module_not_found" {
        "Add or register the missing DLL in `crates/win32-rt/src/` (or check `[dlls]`/`[middleware.*]` in `game.toml`) and run `cargo opsdb -- win32-sync`."
    } else {
        "Implement the export in `crates/win32-rt/src/`, add a unit test, and run `cargo opsdb -- win32-sync`."
    };
    OwnerHint {
        crate_name: "crates/win32-rt".to_string(),
        paths: vec!["crates/win32-rt/src/".to_string()],
        area: "Missing or broken Win32/CRT/COM/middleware export".to_string(),
        reason: format!("Unresolved or stubbed Win32 API ({})", format_dll_sym(dll, symbol)),
        next_step: action.to_string(),
    }
}

fn owner_from_backtrace(
    frames: &[String],
    location: Option<&str>,
    summary: &str,
) -> OwnerHint {
    let mut search_haystacks: Vec<&str> = Vec::new();
    if let Some(loc) = location {
        search_haystacks.push(loc);
    }
    search_haystacks.push(summary);
    for f in frames {
        // Skip panic/backtrace/crash-reporter boilerplate frames.
        if f.contains("std::backtrace")
            || f.contains("std::panicking")
            || f.contains("core::panicking")
            || f.contains("rust_begin_unwind")
            || f.contains("win32_rt::crash::")
            || f.contains("port_loader::init_boot_log")
        {
            continue;
        }
        search_haystacks.push(f.as_str());
    }

    for h in search_haystacks {
        if h.contains("d3d12_metal") || h.contains("crates/d3d12-metal") {
            return OwnerHint {
                crate_name: "crates/d3d12-metal".to_string(),
                paths: vec!["crates/d3d12-metal/src/".to_string()],
                area: "Metal 4 D3D12 / MetalFX GPU backend".to_string(),
                reason: "Backtrace/panic points into `d3d12-metal`".to_string(),
                next_step: "Inspect the top `d3d12_metal` frame in the backtrace.".to_string(),
            };
        }
        if h.contains("dxil_metal") || h.contains("crates/dxil-metal") {
            return OwnerHint {
                crate_name: "crates/dxil-metal".to_string(),
                paths: vec!["crates/dxil-metal/src/".to_string()],
                area: "Shader translation (DXIL, FXC SM 4/5, SPIR-V, WGSL) or PSO cache".to_string(),
                reason: "Backtrace/panic points into `dxil-metal`".to_string(),
                next_step: "Inspect the top `dxil_metal` frame in the backtrace.".to_string(),
            };
        }
        if h.contains("x86_runtime") || h.contains("crates/x86-runtime") {
            return OwnerHint {
                crate_name: "crates/x86-runtime".to_string(),
                paths: vec!["crates/x86-runtime/src/".to_string()],
                area: "x86-64 decoder, ARM64/NEON block JIT, .aot cache, FH4/SEH unwinding".to_string(),
                reason: "Backtrace/panic points into `x86-runtime`".to_string(),
                next_step: "Inspect the top `x86_runtime` frame in the backtrace.".to_string(),
            };
        }
        if h.contains("win32_rt") || h.contains("crates/win32-rt") {
            let sub = if h.contains("d3d12") {
                "crates/win32-rt/src/d3d12/"
            } else if h.contains("d3d11") {
                "crates/win32-rt/src/d3d11/"
            } else if h.contains("dxgi") {
                "crates/win32-rt/src/dxgi/"
            } else if h.contains("audio") {
                "crates/win32-rt/src/audio/"
            } else if h.contains("user32") || h.contains("macos") {
                "crates/win32-rt/src/user32/"
            } else {
                "crates/win32-rt/src/"
            };
            return OwnerHint {
                crate_name: "crates/win32-rt".to_string(),
                paths: vec![sub.to_string()],
                area: "Win32 / CRT / COM / middleware runtime".to_string(),
                reason: "Backtrace/panic points into `win32-rt`".to_string(),
                next_step: format!("Inspect the top `win32_rt` frame under `{sub}`."),
            };
        }
        if h.contains("wwise_audio") || h.contains("crates/wwise-audio") {
            return OwnerHint {
                crate_name: "crates/wwise-audio".to_string(),
                paths: vec!["crates/wwise-audio/src/".to_string()],
                area: "Universal Wwise 2022.1 audio engine".to_string(),
                reason: "Backtrace/panic points into `wwise-audio`".to_string(),
                next_step: "Inspect the top `wwise_audio` frame in the backtrace.".to_string(),
            };
        }
        if h.contains("pe_image") || h.contains("crates/pe-image") {
            return OwnerHint {
                crate_name: "crates/pe-image".to_string(),
                paths: vec!["crates/pe-image/src/".to_string()],
                area: "PE32+ / .NET metadata parsing".to_string(),
                reason: "Backtrace/panic points into `pe-image`".to_string(),
                next_step: "Inspect the top `pe_image` frame in the backtrace.".to_string(),
            };
        }
        if h.contains("port_loader") || h.contains("crates/port-loader") {
            return OwnerHint {
                crate_name: "crates/port-loader".to_string(),
                paths: vec!["crates/port-loader/src/".to_string()],
                area: "Universal runner CLI (`port-runner`), `game.toml` loading, PE mapping".to_string(),
                reason: "Backtrace/panic points into `port-loader`".to_string(),
                next_step: "Inspect the top `port_loader` frame in the backtrace.".to_string(),
            };
        }
    }

    OwnerHint {
        crate_name: "crates/port-loader".to_string(),
        paths: vec![
            "crates/port-loader/src/".to_string(),
            "crates/win32-rt/src/".to_string(),
        ],
        area: "Runtime host panic / abort".to_string(),
        reason: "Host process panicked or aborted without a resolved workspace frame".to_string(),
        next_step: "Re-run with `RUST_BACKTRACE=full` or inspect the captured crash report.".to_string(),
    }
}

fn owner_from_last_activity(last_activity: &[String]) -> OwnerHint {
    for line in last_activity.iter().rev() {
        let low = line.to_ascii_lowercase();
        if low.contains("dxil_metal")
            || low.contains("pso cache")
            || low.contains(" shaders]")
            || low.contains("dxil-metald")
        {
            return OwnerHint {
                crate_name: "crates/dxil-metal".to_string(),
                paths: vec!["crates/dxil-metal/src/".to_string()],
                area: "Shader translation (DXIL, FXC SM 4/5, SPIR-V, WGSL) or PSO cache".to_string(),
                reason: "Last activity before timeout was shader/PSO compilation".to_string(),
                next_step: "Check if shader compilation or `dxil-metald` stalled (try increasing `--timeout` or prewarming shaders).".to_string(),
            };
        }
        if low.contains(" d3d]")
            || low.contains("d3d12")
            || low.contains("dxgi")
            || low.contains("metal")
            || low.contains("fence")
            || low.contains("swapchain")
            || low.contains("present")
        {
            return OwnerHint {
                crate_name: "crates/d3d12-metal + crates/win32-rt".to_string(),
                paths: vec![
                    "crates/d3d12-metal/src/".to_string(),
                    "crates/win32-rt/src/d3d12/".to_string(),
                ],
                area: "Direct3D 12 COM shim vs Metal 4 implementation".to_string(),
                reason: "Last activity before timeout was inside D3D12/DXGI/Metal 4".to_string(),
                next_step: "Inspect fence waits, command queue submission, or headless swapchain presentation in `crates/d3d12-metal/` and `crates/win32-rt/src/d3d12/`.".to_string(),
            };
        }
        if low.contains("audio") || low.contains("wasapi") || low.contains("xaudio") || low.contains("wwise") {
            return OwnerHint {
                crate_name: "crates/win32-rt".to_string(),
                paths: vec![
                    "crates/win32-rt/src/audio/".to_string(),
                    "crates/wwise-audio/src/".to_string(),
                ],
                area: "WASAPI / CoreAudio / audio middleware".to_string(),
                reason: "Last activity before timeout was in the audio subsystem".to_string(),
                next_step: "Inspect audio thread startup/wait loops in `crates/win32-rt/src/audio/`.".to_string(),
            };
        }
        if low.contains("user32") || low.contains("macos") || low.contains("createwindow") || low.contains("msg") {
            return OwnerHint {
                crate_name: "crates/win32-rt".to_string(),
                paths: vec![
                    "crates/win32-rt/src/user32/".to_string(),
                    "crates/win32-rt/src/macos/".to_string(),
                ],
                area: "Windowing, AppKit, mouse/keyboard/raw input".to_string(),
                reason: "Last activity before timeout was in windowing/message pump".to_string(),
                next_step: "Check headless window/dialog or AppKit main-thread message pump behavior in `crates/win32-rt/src/{user32,macos}/` (e.g. a launcher dialog needing `-nolauncher`).".to_string(),
            };
        }
        if low.contains("x86_runtime") || low.contains("emu jit") || low.contains("emu aot") {
            return OwnerHint {
                crate_name: "crates/x86-runtime".to_string(),
                paths: vec!["crates/x86-runtime/src/".to_string()],
                area: "x86-64 decoder, ARM64/NEON block JIT, .aot cache".to_string(),
                reason: "Last activity before timeout was inside x86-runtime JIT/AOT execution".to_string(),
                next_step: "Prewarm `.aot` caches (`--prewarm-aot`) or test with `PORT_AOT=0` / larger `--timeout` to separate slow JIT warmup from a guest deadlock.".to_string(),
            };
        }
        if low.contains("port_loader::loader") || low.contains("loadlibrary") || low.contains("dllmain") {
            return OwnerHint {
                crate_name: "crates/port-loader + crates/win32-rt".to_string(),
                paths: vec![
                    "crates/port-loader/src/loader.rs".to_string(),
                    "crates/win32-rt/src/".to_string(),
                ],
                area: "PE loading / DLL initialization / Win32 runtime".to_string(),
                reason: "Last activity before timeout was module loading or DllMain init".to_string(),
                next_step: "Run with `--env RUST_LOG=info,trace=info` to see which Win32 call blocked after module load.".to_string(),
            };
        }
    }

    OwnerHint {
        crate_name: "crates/win32-rt + crates/x86-runtime".to_string(),
        paths: vec![
            "crates/win32-rt/src/".to_string(),
            "crates/x86-runtime/src/".to_string(),
        ],
        area: "Win32 synchronization/waits or guest execution loop".to_string(),
        reason: "Process timed out without presenting frames".to_string(),
        next_step: "Re-run with a larger `--timeout`, prewarmed `.aot` caches, or `--env RUST_LOG=debug` to identify what the guest is waiting on.".to_string(),
    }
}

fn crate_from_path(file: &str) -> String {
    if let Some(rest) = file.strip_prefix("crates/") {
        if let Some(name) = rest.split('/').next() {
            return format!("crates/{name}");
        }
    }
    "crates/win32-rt".to_string()
}

fn area_for_dll(dll: Option<&str>, sym: Option<&str>) -> &'static str {
    let d = dll.unwrap_or("").to_ascii_lowercase();
    let s = sym.unwrap_or("").to_ascii_lowercase();
    if d.contains("d3d12") || d.contains("dxgi") || s.starts_with("id3d12") {
        "Direct3D 12 COM shim vs Metal 4 implementation"
    } else if d.contains("d3d11") {
        "Direct3D 11.0/11.1 runtime"
    } else if d.contains("vulkan") {
        "Vulkan loader & VK_KHR_win32_surface bridge"
    } else if d.contains("user32") || d.contains("xinput") || d.contains("hid") {
        "Windowing, AppKit, mouse/keyboard/raw input, XInput, DualSense HID"
    } else if d.contains("mmdevapi") || d.contains("xaudio") || d.contains("fmod") {
        "WASAPI / CoreAudio / audio middleware"
    } else {
        "Win32 / CRT / COM / middleware export"
    }
}

fn format_dll_sym(dll: Option<&str>, sym: Option<&str>) -> String {
    match (dll, sym) {
        (Some(d), Some(s)) => format!("{d}!{s}"),
        (Some(d), None) => d.to_string(),
        (None, Some(s)) => s.to_string(),
        (None, None) => "unknown".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Extraction helpers
// ---------------------------------------------------------------------------

fn split_called_from(s: &str) -> (&str, Option<String>) {
    if let Some((head, tail)) = s.split_once(" called from ") {
        let loc = extract_module_rva(tail).or_else(|| {
            let tok = tail.trim().trim_end_matches(')');
            (!tok.is_empty()).then(|| tok.to_string())
        });
        (head.trim(), loc)
    } else {
        (s.trim(), None)
    }
}

/// Extract a canonical `module+0x<hex>` token (e.g. `spider-man2.exe+0x5081964`) from a string.
pub fn extract_module_rva(s: &str) -> Option<String> {
    let bytes = s.as_bytes();
    let mut search_from = 0usize;
    while let Some(rel) = s[search_from..].find("+0x") {
        let plus = search_from + rel;
        // Walk backward for module filename.
        let mut start = plus;
        while start > 0 {
            let b = bytes[start - 1];
            if b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-') {
                start -= 1;
            } else {
                break;
            }
        }
        // Walk forward for hex digits.
        let hex_start = plus + 3;
        let mut hex_end = hex_start;
        while hex_end < bytes.len() && bytes[hex_end].is_ascii_hexdigit() {
            hex_end += 1;
        }
        if start < plus && hex_end > hex_start {
            let module = &s[start..plus];
            let lower = module.to_ascii_lowercase();
            let looks_like_module = lower.ends_with(".exe")
                || lower.ends_with(".dll")
                || lower.ends_with(".pyd")
                || lower.ends_with(".node")
                || lower.ends_with(".ocx")
                || lower.ends_with(".sys")
                || lower.ends_with(".drv");
            if looks_like_module {
                return Some(s[start..hex_end].to_string());
            }
        }
        search_from = plus + 3;
    }
    None
}

fn find_nearby_guest_location(lines: &[ParsedLine<'_>], idx: usize) -> Option<String> {
    let end = (idx + 60).min(lines.len());
    for l in &lines[idx..end] {
        if l.msg.contains("end of crash report") {
            break;
        }
        if l.msg.starts_with("guest interpreter state") {
            if let Some(loc) = extract_module_rva(l.msg) {
                return Some(loc);
            }
        }
    }
    for l in &lines[idx..end] {
        if l.msg.contains("end of crash report") {
            break;
        }
        if let Some(loc) = extract_module_rva(l.msg) {
            return Some(loc);
        }
    }
    None
}

/// Extract the best symbolized backtrace from a crash report immediately following `start_idx`.
///
/// Crash reports can contain up to three sections (`guest backtrace`, `backtrace (faulting thread)`,
/// and `host backtrace`). We parse each section independently so headers never bleed across
/// sections, preferring whichever of `guest backtrace` / `backtrace (faulting thread)` contains
/// symbolized `module+0xRVA` guest frames before falling back to raw guest or host frames.
fn extract_crash_backtrace_after(lines: &[ParsedLine<'_>], start_idx: usize) -> Vec<String> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum BtSection {
        None,
        Guest,
        Faulting,
        Host,
    }

    let end = (start_idx + 120).min(lines.len());
    let mut section = BtSection::None;
    let mut guest_bt: Vec<String> = Vec::new();
    let mut faulting_bt: Vec<String> = Vec::new();
    let mut host_bt: Vec<String> = Vec::new();

    for l in &lines[start_idx..end] {
        let m = l.msg;
        if m.contains("end of crash report") || m.starts_with("other threads") {
            break;
        }
        if m.starts_with("guest backtrace") {
            section = BtSection::Guest;
            continue;
        }
        if m.starts_with("backtrace (") {
            section = BtSection::Faulting;
            continue;
        }
        if m.starts_with("host backtrace") {
            section = BtSection::Host;
            continue;
        }
        if section != BtSection::None {
            let t = m.trim_start();
            if t.starts_with('#') {
                let dst = match section {
                    BtSection::Guest => &mut guest_bt,
                    BtSection::Faulting => &mut faulting_bt,
                    BtSection::Host => &mut host_bt,
                    BtSection::None => continue,
                };
                if dst.len() < 16 {
                    dst.push(t.to_string());
                }
            } else if !t.is_empty() {
                section = BtSection::None;
            }
        }
    }

    let has_sym = |frames: &[String]| frames.iter().any(|f| extract_module_rva(f).is_some());
    if has_sym(&guest_bt) {
        guest_bt
    } else if has_sym(&faulting_bt) {
        faulting_bt
    } else if !guest_bt.is_empty() {
        guest_bt
    } else if !faulting_bt.is_empty() {
        faulting_bt
    } else {
        host_bt
    }
}

fn find_recent_loadlibrary_failure(lines: &[ParsedLine<'_>], before_idx: usize) -> Option<String> {
    let end = before_idx.min(lines.len());
    for l in lines[..end].iter().rev().take(60) {
        if (l.msg.contains("LoadLibrary(") || l.msg.contains("LoadLibraryEx("))
            && l.msg.contains(" failed")
        {
            if let Some(q1) = l.msg.find('"') {
                if let Some(q2) = l.msg[q1 + 1..].find('"') {
                    let name = &l.msg[q1 + 1..q1 + 1 + q2];
                    if !name.is_empty() {
                        return Some(basename_win(name).to_string());
                    }
                }
            }
        }
    }
    None
}

fn find_recent_proc_not_found(
    lines: &[ParsedLine<'_>],
    before_idx: usize,
) -> Option<(String, String)> {
    let end = before_idx.min(lines.len());
    for l in lines[..end].iter().rev().take(60) {
        if let Some(pair) = parse_proc_not_found_line(l.msg) {
            return Some(pair);
        }
    }
    None
}

fn parse_proc_not_found_line(msg: &str) -> Option<(String, String)> {
    let start = msg
        .find("GetProcAddress(")
        .map(|p| p + "GetProcAddress(".len())
        .or_else(|| {
            msg.find("LdrGetProcedureAddress(")
                .map(|p| p + "LdrGetProcedureAddress(".len())
        })?;
    if !msg.contains("-> NULL") {
        return None;
    }
    let rest = &msg[start..];
    let close = rest.find(')')?;
    let inside = &rest[..close];
    let (raw_mod, raw_proc) = inside.split_once(',')?;
    let dll = basename_win(raw_mod.trim()).to_string();
    let sym = raw_proc.trim().trim_matches('"').to_string();
    if dll.is_empty() || sym.is_empty() {
        return None;
    }
    Some((dll, sym))
}

fn collect_missing_probes(lines: &[ParsedLine<'_>]) -> Vec<String> {
    let mut set = BTreeSet::new();
    for l in lines {
        if let Some((dll, sym)) = parse_proc_not_found_line(l.msg) {
            set.insert(format!("{dll}!{sym}"));
        }
    }
    set.into_iter().collect()
}

fn basename_win(path: &str) -> &str {
    path.rsplit(['\\', '/']).next().unwrap_or(path)
}

fn extract_backtick_span(s: &str) -> Option<String> {
    let a = s.find('`')?;
    let b = s[a + 1..].find('`')?;
    let inner = s[a + 1..a + 1 + b].trim();
    (!inner.is_empty()).then(|| inner.to_string())
}

/// Convert `140012ab0: 488b00                   mov rax,[rax]` -> `mov rax,[rax]`.
fn clean_disasm_instruction(raw: &str) -> String {
    let trimmed = raw.trim();
    if let Some((addr_part, rest)) = trimmed.split_once(':') {
        let a = addr_part
            .trim()
            .trim_start_matches("0x")
            .trim_start_matches("0X");
        if !a.is_empty() && a.bytes().all(|b| b.is_ascii_hexdigit()) {
            let after_colon = rest.trim_start();
            // Skip leading hex opcode bytes token(s) separated from the mnemonic by spaces.
            let mut parts = after_colon.split_whitespace().peekable();
            let mut skipped_hex = false;
            while let Some(&tok) = parts.peek() {
                if tok.len() >= 2
                    && tok.len() % 2 == 0
                    && tok.bytes().all(|b| b.is_ascii_hexdigit())
                    && !is_x86_mnemonic_word(tok)
                {
                    parts.next();
                    skipped_hex = true;
                } else {
                    break;
                }
            }
            if skipped_hex {
                let mn: Vec<&str> = parts.collect();
                if !mn.is_empty() {
                    return mn.join(" ");
                }
            }
            if !after_colon.is_empty() {
                return after_colon.to_string();
            }
        }
    }
    trimmed.to_string()
}

fn is_x86_mnemonic_word(tok: &str) -> bool {
    matches!(
        tok.to_ascii_lowercase().as_str(),
        "add" | "adc" | "and" | "call" | "daa" | "das" | "dec" | "div" | "cdqe" | "cbw"
    )
}

fn extract_exception_code_and_name(msg: &str) -> Option<(String, String)> {
    let low = msg.to_ascii_lowercase();
    let pos = low.find("exception 0x")?;
    let hex_start = pos + "exception ".len();
    let rest = &msg[hex_start..];
    let code_end = rest
        .find(|c: char| !c.is_ascii_hexdigit() && c != 'x' && c != 'X')
        .unwrap_or(rest.len());
    let code = rest[..code_end].to_ascii_lowercase();
    if code.len() < 4 {
        return None;
    }
    let after = rest[code_end..].trim_start();
    let name = after
        .split(" flags=")
        .next()
        .unwrap_or(after)
        .split(" at ")
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    Some((code, name))
}

fn extract_failing_hresult(msg: &str) -> Option<(String, Option<&'static str>)> {
    if let Some((_, rhs)) = msg.rsplit_once("->") {
        if let Some(hit) = extract_failing_hresult_in_slice(rhs) {
            return Some(hit);
        }
    }
    extract_failing_hresult_in_slice(msg)
}

fn extract_failing_hresult_in_slice(msg: &str) -> Option<(String, Option<&'static str>)> {
    // 1. Named failing HRESULT constants.
    const NAMED: &[(&str, &str)] = &[
        ("DXGI_ERROR_INVALID_CALL", "0x887a0001"),
        ("DXGI_ERROR_NOT_FOUND", "0x887a0002"),
        ("DXGI_ERROR_MORE_DATA", "0x887a0003"),
        ("DXGI_ERROR_UNSUPPORTED", "0x887a0004"),
        ("DXGI_ERROR_DEVICE_REMOVED", "0x887a0005"),
        ("DXGI_ERROR_DEVICE_HUNG", "0x887a0006"),
        ("DXGI_ERROR_DEVICE_RESET", "0x887a0007"),
        ("DXGI_ERROR_WAS_STILL_DRAWING", "0x887a000a"),
        ("DXGI_ERROR_DRIVER_INTERNAL_ERROR", "0x887a0020"),
        ("DXGI_ERROR_ACCESS_DENIED", "0x887a0022"),
        ("DXGI_ERROR_SESSION_DISCONNECTED", "0x887a0026"),
        ("DXGI_ERROR_WAIT_TIMEOUT", "0x887a0027"),
        ("D3D12_ERROR_ADAPTER_NOT_FOUND", "0x887e0001"),
        ("D3D12_ERROR_DRIVER_VERSION_MISMATCH", "0x887e0002"),
        ("E_NOTIMPL", "0x80004001"),
        ("E_NOINTERFACE", "0x80004002"),
        ("E_POINTER", "0x80004003"),
        ("E_ABORT", "0x80004004"),
        ("E_FAIL", "0x80004005"),
        ("E_ACCESSDENIED", "0x80070005"),
        ("E_HANDLE", "0x80070006"),
        ("E_OUTOFMEMORY", "0x8007000e"),
        ("E_INVALIDARG", "0x80070057"),
    ];
    for &(name, hex) in NAMED {
        if msg.contains(name) {
            return Some((hex.to_string(), Some(name)));
        }
    }

    // 2. Hex 32-bit HRESULT with failure bit set (`0x8xxxxxxx`, `0x887a....`, `0x8007....`, `0x8000....`).
    let bytes = msg.as_bytes();
    let mut i = 0usize;
    while i + 10 <= bytes.len() {
        if bytes[i] == b'0' && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
            let mut end = i + 2;
            while end < bytes.len() && bytes[end].is_ascii_hexdigit() {
                end += 1;
            }
            if end - (i + 2) == 8 {
                let hex_digits = &msg[i + 2..end];
                if let Ok(val) = u32::from_str_radix(hex_digits, 16) {
                    if (val & 0x8000_0000) != 0 {
                        let canonical = format!("0x{val:08x}");
                        return Some((canonical, hresult_name(val)));
                    }
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }
    None
}

fn hresult_name(hr: u32) -> Option<&'static str> {
    match hr {
        0x8000_4001 => Some("E_NOTIMPL"),
        0x8000_4002 => Some("E_NOINTERFACE"),
        0x8000_4003 => Some("E_POINTER"),
        0x8000_4004 => Some("E_ABORT"),
        0x8000_4005 => Some("E_FAIL"),
        0x8007_0005 => Some("E_ACCESSDENIED"),
        0x8007_0006 => Some("E_HANDLE"),
        0x8007_000E => Some("E_OUTOFMEMORY"),
        0x8007_0057 => Some("E_INVALIDARG"),
        0x887A_0001 => Some("DXGI_ERROR_INVALID_CALL"),
        0x887A_0002 => Some("DXGI_ERROR_NOT_FOUND"),
        0x887A_0003 => Some("DXGI_ERROR_MORE_DATA"),
        0x887A_0004 => Some("DXGI_ERROR_UNSUPPORTED"),
        0x887A_0005 => Some("DXGI_ERROR_DEVICE_REMOVED"),
        0x887A_0006 => Some("DXGI_ERROR_DEVICE_HUNG"),
        0x887A_0007 => Some("DXGI_ERROR_DEVICE_RESET"),
        0x887A_000A => Some("DXGI_ERROR_WAS_STILL_DRAWING"),
        0x887A_0020 => Some("DXGI_ERROR_DRIVER_INTERNAL_ERROR"),
        0x887A_0022 => Some("DXGI_ERROR_ACCESS_DENIED"),
        0x887A_0026 => Some("DXGI_ERROR_SESSION_DISCONNECTED"),
        0x887A_0027 => Some("DXGI_ERROR_WAIT_TIMEOUT"),
        0x887E_0001 => Some("D3D12_ERROR_ADAPTER_NOT_FOUND"),
        0x887E_0002 => Some("D3D12_ERROR_DRIVER_VERSION_MISMATCH"),
        _ => None,
    }
}

fn extract_rust_file_loc(msg: &str) -> Option<String> {
    for tok in msg.split_whitespace() {
        let clean = tok.trim_matches(|c| matches!(c, '\'' | '"' | ':' | ',' | '(' | ')'));
        if clean.contains(".rs:") {
            return Some(clean.to_string());
        }
    }
    None
}

fn collect_backtrace_after(lines: &[ParsedLine<'_>], start_idx: usize) -> Vec<String> {
    let mut frames: Vec<String> = Vec::new();
    let end = (start_idx + 120).min(lines.len());
    for l in &lines[start_idx + 1..end] {
        let t = l.msg.trim();
        if t.is_empty() {
            if !frames.is_empty() {
                break;
            }
            continue;
        }
        // Rust backtrace frame: `0: foo::bar` or `12: ...`
        if is_numbered_rust_frame(t) {
            if frames.len() < 16 {
                frames.push(t.to_string());
            }
            continue;
        }
        // Rust backtrace location line: `at crates/foo/src/bar.rs:42:10`
        if let Some(at_rest) = t.strip_prefix("at ") {
            if let Some(last) = frames.last_mut() {
                if !last.contains(" @ ") {
                    last.push_str(" @ ");
                    last.push_str(at_rest.trim());
                }
            }
            continue;
        }
        // `crash.rs` frame: `#0  foo::bar`
        if t.starts_with('#') && t[1..].chars().next().is_some_and(|c| c.is_ascii_digit()) {
            if frames.len() < 16 {
                frames.push(t.to_string());
            }
            continue;
        }
        // macOS NSException / backtrace_symbols frame: `0   port-runner   0x00000001... sym + 12`
        if is_apple_backtrace_frame(t) {
            if frames.len() < 16 {
                frames.push(t.to_string());
            }
            continue;
        }
        if t == "Backtrace:"
            || t == "stack backtrace:"
            || t == "*** First throw call stack:"
            || t == "("
            || t.starts_with("note:")
            || t.starts_with("host backtrace")
            || t.starts_with("backtrace (")
        {
            continue;
        }
        if !frames.is_empty() {
            break;
        }
    }
    frames
}

fn collect_any_backtrace(lines: &[ParsedLine<'_>]) -> Vec<String> {
    for (idx, l) in lines.iter().enumerate() {
        let t = l.msg.trim();
        if t == "Backtrace:"
            || t == "stack backtrace:"
            || t.starts_with("host backtrace")
            || t.starts_with("backtrace (")
            || t == "*** First throw call stack:"
        {
            let bt = collect_backtrace_after(lines, idx);
            if !bt.is_empty() {
                return bt;
            }
        }
    }
    Vec::new()
}

fn is_numbered_rust_frame(t: &str) -> bool {
    let Some((num, rest)) = t.split_once(':') else {
        return false;
    };
    !num.is_empty()
        && num.bytes().all(|b| b.is_ascii_digit())
        && rest.starts_with(' ')
        && !rest.trim().is_empty()
}

fn is_apple_backtrace_frame(t: &str) -> bool {
    let parts: Vec<&str> = t.split_whitespace().collect();
    parts.len() >= 4
        && parts[0].bytes().all(|b| b.is_ascii_digit())
        && parts[2].starts_with("0x")
}

fn extract_last_activity(lines: &[ParsedLine<'_>], limit: usize) -> Vec<String> {
    let mut out = Vec::new();
    for l in lines.iter().rev() {
        let m = l.msg.trim();
        if m.is_empty() {
            continue;
        }
        if m.contains("[run-limit] timed out after")
            || m.contains("port-runner: timed out after")
            || m.starts_with("boot_report: wrote ")
            || m.starts_with("crash report written to ")
            || m.starts_with("====================")
        {
            continue;
        }
        out.push(l.raw.to_string());
        if out.len() >= limit {
            break;
        }
    }
    out.reverse();
    out
}

fn infer_frames_presented_from_log(lines: &[ParsedLine<'_>]) -> u64 {
    let mut max_frame = 0u64;
    for l in lines {
        if let Some(pos) = l.msg.find("frames_presented=") {
            let rest = &l.msg[pos + "frames_presented=".len()..];
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = digits.parse::<u64>() {
                max_frame = max_frame.max(n);
            }
        }
        if let Some(pos) = l.msg.find("Present #") {
            let rest = &l.msg[pos + "Present #".len()..];
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(n) = digits.parse::<u64>() {
                max_frame = max_frame.max(n);
            }
        }
    }
    max_frame
}

fn infer_graphics_api_from_log(lines: &[ParsedLine<'_>]) -> String {
    for l in lines.iter().rev() {
        if l.msg.contains("D3D12CreateDevice") || l.msg.contains("ID3D12Device") {
            return "d3d12".to_string();
        }
        if l.msg.contains("D3D11CreateDevice") || l.msg.contains("d3d11:") {
            return "d3d11".to_string();
        }
        if l.msg.contains("vkCreateInstance") || l.msg.contains("MoltenVK") {
            return "vulkan".to_string();
        }
    }
    "unknown".to_string()
}

fn truncate_chars(s: &str, max: usize) -> String {
    let mut chars = s.chars();
    let head: String = chars.by_ref().take(max).collect();
    if chars.next().is_some() {
        format!("{}...", head.chars().take(max.saturating_sub(3)).collect::<String>())
    } else {
        head
    }
}

// ---------------------------------------------------------------------------
// Formatting (human summary + Schema v1 JSON)
// ---------------------------------------------------------------------------

/// Render a short, human-readable triage summary pointing at the likely owning crate
/// from `AGENTS.md`'s "Where code belongs" table.
pub fn format_human_summary(report: &BootTriageReport) -> String {
    let mut out = String::new();
    let cls = report.blocker.class();
    let _ = writeln!(
        out,
        "=== Boot Triage: {} ({}) ===",
        cls.label(),
        report.blocker.subkind
    );
    if let Some(ref exe) = report.target.exe {
        let args_str = if report.target.args.is_empty() {
            String::new()
        } else {
            format!(" {}", report.target.args.join(" "))
        };
        let _ = writeln!(out, "  Target       : {exe}{args_str}");
    }
    let _ = writeln!(
        out,
        "  Run          : mode={}, exit={:?}, signal={:?}, frames={}, elapsed={}ms, log_lines={}",
        report.run.mode,
        report.run.exit_code,
        report.run.signal,
        report.run.frames_presented,
        report.run.elapsed_ms,
        report.run.log_lines
    );
    let _ = writeln!(out, "  Blocker      : {}", report.blocker.summary);
    if let Some(ref loc) = report.blocker.location {
        let _ = writeln!(out, "  Location     : {loc}");
    }
    if let Some(ref insn) = report.blocker.faulting_instruction {
        let _ = writeln!(out, "  Instruction  : {insn}");
    }
    if let Some(ref hr) = report.blocker.hresult {
        let _ = writeln!(out, "  HRESULT      : {hr}");
    }
    if !report.blocker.backtrace.is_empty() {
        let _ = writeln!(out, "  Backtrace    :");
        for frame in report.blocker.backtrace.iter().take(6) {
            let _ = writeln!(out, "    {frame}");
        }
    }
    if !report.blocker.last_activity.is_empty() {
        let _ = writeln!(out, "  Last Activity:");
        for line in report.blocker.last_activity.iter().rev().take(4).rev() {
            let _ = writeln!(out, "    {}", truncate_chars(line, 140));
        }
    }
    if let Some(ref owner) = report.owner {
        let _ = writeln!(
            out,
            "  Likely Owner : {} [{}] ({})",
            owner.crate_name,
            owner.paths.join(", "),
            owner.area
        );
        let _ = writeln!(out, "  Next Step    : {}", owner.next_step);
    }
    if !report.secondary_signals.is_empty() {
        let _ = writeln!(
            out,
            "  Other Signals: {} ({})",
            report.secondary_signals.len(),
            report
                .secondary_signals
                .iter()
                .take(3)
                .map(|s| format!("{}: {}", s.class.as_str(), truncate_chars(&s.summary, 60)))
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    out
}

/// Build the Schema v1 `data` payload for `port-factory boot-triage`.
pub fn boot_triage_data_json(report: &BootTriageReport) -> JsonValue {
    let target_obj = JsonObj::new()
        .opt_str("manifest", report.target.manifest.clone())
        .opt_str("game_dir", report.target.game_dir.clone())
        .opt_str("pe", report.target.pe.clone())
        .opt_str("exe", report.target.exe.clone())
        .str_array("args", report.target.args.iter().cloned())
        .val(
            "timeout_secs",
            report
                .target
                .timeout_secs
                .map(JsonValue::Float)
                .unwrap_or(JsonValue::Null),
        )
        .val(
            "max_frames",
            report
                .target
                .max_frames
                .map(JsonValue::UInt)
                .unwrap_or(JsonValue::Null),
        )
        .build();

    let run_obj = JsonObj::new()
        .str("mode", report.run.mode.clone())
        .val(
            "exit_code",
            report
                .run
                .exit_code
                .map(|c| JsonValue::Int(c as i64))
                .unwrap_or(JsonValue::Null),
        )
        .val(
            "signal",
            report
                .run
                .signal
                .map(|s| JsonValue::Int(s as i64))
                .unwrap_or(JsonValue::Null),
        )
        .bool("timed_out", report.run.timed_out)
        .u64("elapsed_ms", report.run.elapsed_ms)
        .u64("frames_presented", report.run.frames_presented)
        .str("graphics_api", report.run.graphics_api.clone())
        .usize("log_lines", report.run.log_lines)
        .opt_str("log_path", report.run.log_path.clone())
        .bool("boot_report_found", report.run.boot_report_found)
        .bool("crash_report_found", report.run.crash_report_found)
        .build();

    let b = &report.blocker;
    let blocker_obj = JsonObj::new()
        .str("class", b.class().as_str())
        .str("subkind", b.subkind.clone())
        .str("summary", b.summary.clone())
        .val(
            "log_line",
            b.log_line
                .map(|n| JsonValue::UInt(n as u64))
                .unwrap_or(JsonValue::Null),
        )
        .opt_str("evidence", b.evidence.clone())
        .opt_str("dll", b.dll.clone())
        .opt_str("symbol", b.symbol.clone())
        .opt_str("exception_code", b.exception_code.clone())
        .opt_str("hresult", b.hresult.clone())
        .opt_str("location", b.location.clone())
        .opt_str("faulting_instruction", b.faulting_instruction.clone())
        .str_array("backtrace", b.backtrace.iter().cloned())
        .str_array("last_activity", b.last_activity.iter().cloned())
        .build();

    let owner_val = match &report.owner {
        Some(o) => JsonObj::new()
            .str("crate", o.crate_name.clone())
            .str_array("paths", o.paths.iter().cloned())
            .str("area", o.area.clone())
            .str("reason", o.reason.clone())
            .str("next_step", o.next_step.clone())
            .build(),
        None => JsonValue::Null,
    };

    let secondary_arr = JsonValue::Array(
        report
            .secondary_signals
            .iter()
            .map(|s| {
                JsonObj::new()
                    .str("class", s.class.as_str())
                    .str("subkind", s.subkind.clone())
                    .val(
                        "log_line",
                        s.log_line
                            .map(|n| JsonValue::UInt(n as u64))
                            .unwrap_or(JsonValue::Null),
                    )
                    .str("summary", s.summary.clone())
                    .str("detail", s.detail.clone())
                    .build()
            })
            .collect(),
    );

    JsonObj::new()
        .val("target", target_obj)
        .val("run", run_obj)
        .val("blocker", blocker_obj)
        .val("owner", owner_val)
        .val("secondary_signals", secondary_arr)
        .str_array("missing_probes", report.missing_probes.iter().cloned())
        .str("human_summary", format_human_summary(report))
        .build()
}

/// Build the full Schema v1 [`JsonEnvelope`] for `boot-triage`.
pub fn boot_triage_envelope(report: &BootTriageReport) -> JsonEnvelope {
    let code = report.exit_code();
    let error = (code != exit_codes::OK).then(|| report.blocker.summary.clone());
    JsonEnvelope::with_status("boot-triage", code, boot_triage_data_json(report), error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_input(log: &str, exit_code: Option<i32>) -> BootTriageInput {
        BootTriageInput {
            target: BootTriageTarget {
                exe: Some("TestGame.exe".to_string()),
                timeout_secs: Some(30.0),
                ..BootTriageTarget::default()
            },
            mode: "offline".to_string(),
            log_text: log.to_string(),
            exit_code,
            ..BootTriageInput::default()
        }
    }

    #[test]
    fn classifies_unresolved_static_import_over_crash_report() {
        let log = "\
[2026-11-28T04:00:01.000Z INFO  port_loader] mapped testgame.exe at 0x140000000-0x140200000
[2026-11-28T04:00:01.100Z WARN  stub] STUB SetThreadDescription: SetThreadDescription -> 0 (last error 0)
[2026-11-28T04:00:01.200Z ERROR port_loader::thunks] UNIMPLEMENTED advapi32.dll!CredWriteW (imported by testgame.exe) called from testgame.exe+0x4120 [va 0x140004120]
[2026-11-28T04:00:01.201Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unimplemented import advapi32.dll!CredWriteW ====================
[2026-11-28T04:00:01.202Z ERROR win32_rt::crash] ==================== end of crash report (1.2ms) ====================";
        let rep = classify_boot_log(&synthetic_input(log, Some(57)));
        assert_eq!(rep.blocker.class(), BlockerClass::UnresolvedImport);
        assert_eq!(rep.blocker.subkind, "static_trap");
        assert_eq!(rep.blocker.dll.as_deref(), Some("advapi32.dll"));
        assert_eq!(rep.blocker.symbol.as_deref(), Some("CredWriteW"));
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x4120"));
        assert_eq!(rep.exit_code(), exit_codes::UNRESOLVED_IMPORTS);
        let owner = rep.owner.as_ref().expect("owner");
        assert_eq!(owner.crate_name, "crates/win32-rt");
        assert_eq!(rep.secondary_signals.len(), 1);
        assert_eq!(rep.secondary_signals[0].class, BlockerClass::StubHit);
    }

    #[test]
    fn classifies_com_unimplemented_method_to_d3d12_owner() {
        let log = "\
[2026-11-28T04:00:01.000Z ERROR win32_rt::d3d12::com] UNIMPLEMENTED ID3D12Device8::CreateSamplerFeedbackUnorderedAccessView (COM method) called from testgame.exe+0x8820";
        let rep = classify_boot_log(&synthetic_input(log, Some(57)));
        assert_eq!(rep.blocker.class(), BlockerClass::UnresolvedImport);
        assert_eq!(rep.blocker.subkind, "com_trap");
        assert_eq!(
            rep.blocker.symbol.as_deref(),
            Some("ID3D12Device8::CreateSamplerFeedbackUnorderedAccessView")
        );
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x8820"));
        let owner = rep.owner.as_ref().expect("owner");
        assert!(owner.crate_name.contains("d3d12-metal"));
    }

    #[test]
    fn classifies_delay_load_module_not_found_0xc06d007e() {
        let log = "\
[2026-11-28T04:00:01.050Z WARN  port_loader::loader] LoadLibrary(\"xinput_custom.dll\", 0x0) failed (err=126): not found
[2026-11-28T04:00:01.060Z ERROR win32_rt::unwind] UNHANDLED exception 0xc06d007e DELAYLOAD module not found (VcppException ERROR_MOD_NOT_FOUND) flags=0x0 at testgame.exe+0x22100 params=[0x140500000]
[2026-11-28T04:00:01.061Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled exception ====================";
        let rep = classify_boot_log(&synthetic_input(log, Some(126)));
        assert_eq!(rep.blocker.class(), BlockerClass::UnresolvedImport);
        assert_eq!(rep.blocker.subkind, "delay_load_module_not_found");
        assert_eq!(rep.blocker.exception_code.as_deref(), Some("0xc06d007e"));
        assert_eq!(rep.blocker.dll.as_deref(), Some("xinput_custom.dll"));
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x22100"));
        assert_eq!(rep.exit_code(), exit_codes::UNRESOLVED_IMPORTS);
        assert_eq!(rep.owner.as_ref().unwrap().crate_name, "crates/win32-rt");
    }

    #[test]
    fn classifies_delay_load_proc_not_found_0xc06d007f() {
        let log = "\
[2026-11-28T04:00:01.050Z WARN  win32_rt::kernel32::module] GetProcAddress(C:\\Windows\\System32\\kernel32.dll, CreateFile2Ex) -> NULL (ERROR_PROC_NOT_FOUND)
[2026-11-28T04:00:01.060Z ERROR win32_rt::unwind] UNHANDLED exception 0xc06d007f DELAYLOAD proc not found (VcppException ERROR_PROC_NOT_FOUND) flags=0x0 at testgame.exe+0x33400 params=[0x140600000]";
        let rep = classify_boot_log(&synthetic_input(log, Some(127)));
        assert_eq!(rep.blocker.class(), BlockerClass::UnresolvedImport);
        assert_eq!(rep.blocker.subkind, "delay_load_proc_not_found");
        assert_eq!(rep.blocker.exception_code.as_deref(), Some("0xc06d007f"));
        assert_eq!(rep.blocker.dll.as_deref(), Some("kernel32.dll"));
        assert_eq!(rep.blocker.symbol.as_deref(), Some("CreateFile2Ex"));
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x33400"));
        assert_eq!(rep.missing_probes, vec!["kernel32.dll!CreateFile2Ex"]);
    }

    #[test]
    fn classifies_stub_and_failing_hits_with_boot_report_enrichment() {
        let log = "\
[2026-11-28T04:00:01.000Z INFO  port_loader] calling exe entry point testgame.exe+0x1000
[2026-11-28T04:00:01.100Z WARN  stub] STUB CredEnumerateW: CredEnumerateW -> 0 (last error 120)
[2026-11-28T04:00:01.200Z INFO  port_loader] entry point returned 1";
        let boot_report = r#"{
          "schema_version": 1,
          "exit": { "kind": "clean_exit", "code": 1, "detail": "", "elapsed_ms": 200, "frames_presented": 0, "graphics_api": "unknown" },
          "stubs": {
            "total_hits": 2,
            "unique_sites": 1,
            "entries": [
              { "dll": "advapi32.dll", "api": "CredEnumerateW", "kind": "failing", "hits": 2, "first_frame": 0, "first_guest_caller_va": 4608, "first_guest_caller": "testgame.exe+0x1200", "file": "crates/win32-rt/src/advapi32.rs", "line": 412, "first_detail": "CredEnumerateW -> 0 (last error 120)" }
            ]
          },
          "unresolved_imports": []
        }"#;
        let mut inp = synthetic_input(log, Some(1));
        inp.boot_report_json = Some(boot_report.to_string());
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::StubHit);
        assert_eq!(rep.blocker.subkind, "failing");
        assert_eq!(rep.blocker.dll.as_deref(), Some("advapi32.dll"));
        assert_eq!(rep.blocker.symbol.as_deref(), Some("CredEnumerateW"));
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x1200"));
        let owner = rep.owner.as_ref().unwrap();
        assert_eq!(owner.crate_name, "crates/win32-rt");
        assert_eq!(owner.paths, vec!["crates/win32-rt/src/advapi32.rs:412"]);
    }

    #[test]
    fn classifies_guest_crash_with_module_rva_and_faulting_instruction() {
        let log = "\
[2026-11-28T04:00:01.000Z WARN  win32_rt::kernel32::module] GetProcAddress(C:\\Windows\\System32\\kernel32.dll, FlsGetValue2) -> NULL (ERROR_PROC_NOT_FOUND)
[2026-11-28T04:00:02.000Z ERROR win32_rt::unwind] UNHANDLED exception 0xc0000005 ACCESS_VIOLATION flags=0x0 at testgame.exe+0x12ab0 [va 0x140012ab0] params=[0x0, 0x0]
[2026-11-28T04:00:02.001Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled exception ====================
[2026-11-28T04:00:02.001Z ERROR win32_rt::crash] exception 0xc0000005 ACCESS_VIOLATION flags=0x0 at testgame.exe+0x12ab0 params=[0x0, 0x0]
[2026-11-28T04:00:02.001Z ERROR win32_rt::crash] access violation: read from 0x0
[2026-11-28T04:00:02.002Z ERROR win32_rt::crash] disassembly @rip:
[2026-11-28T04:00:02.002Z ERROR win32_rt::crash]   => 140012ab0: 488b00                   mov rax,[rax]
[2026-11-28T04:00:02.002Z ERROR win32_rt::crash]      140012ab3: 4885c0                   test rax,rax
[2026-11-28T04:00:02.003Z ERROR win32_rt::crash] guest interpreter state (innermost emu frame; rip = after current instruction): rip=0x140012ab0 testgame.exe+0x12ab0 [va 0x140012ab0]
[2026-11-28T04:00:02.003Z ERROR win32_rt::crash] guest backtrace (pdata, 2 frame(s)):
[2026-11-28T04:00:02.003Z ERROR win32_rt::crash]   #0  testgame.exe+0x12ab0 [pdata]
[2026-11-28T04:00:02.003Z ERROR win32_rt::crash]   #1  testgame.exe+0x10100 [pdata]
[2026-11-28T04:00:02.004Z ERROR win32_rt::crash] ==================== end of crash report (3.1ms) ====================";
        let rep = classify_boot_log(&synthetic_input(log, Some(5)));
        assert_eq!(rep.blocker.class(), BlockerClass::GuestCrash);
        assert_eq!(rep.blocker.subkind, "access_violation");
        assert_eq!(rep.blocker.exception_code.as_deref(), Some("0xc0000005"));
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x12ab0"));
        assert_eq!(
            rep.blocker.faulting_instruction.as_deref(),
            Some("mov rax,[rax]")
        );
        assert_eq!(rep.blocker.backtrace.len(), 2);
        assert_eq!(rep.owner.as_ref().unwrap().crate_name, "crates/x86-runtime");
        // Benign GetProcAddress probe must NOT trigger UnresolvedImport.
        assert_eq!(rep.missing_probes, vec!["kernel32.dll!FlsGetValue2"]);
    }

    #[test]
    fn classifies_unimplemented_x86_opcode_as_guest_crash_owned_by_x86_runtime() {
        let log = "\
[2026-11-28T04:00:02.000Z ERROR x86_runtime::emu::interp] emu: UNIMPLEMENTED mnemonic Vprold at 0x140050100 (testgame.exe+0x50100) `vprold zmm0, zmm1, 1` — aborting (exit 86; disable with --no-fail-on-unimpl-op)";
        let rep = classify_boot_log(&synthetic_input(log, Some(86)));
        assert_eq!(rep.blocker.class(), BlockerClass::GuestCrash);
        assert_eq!(rep.blocker.subkind, "unimplemented_x86_op");
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x50100"));
        assert_eq!(
            rep.blocker.faulting_instruction.as_deref(),
            Some("vprold zmm0, zmm1, 1")
        );
        assert_eq!(rep.owner.as_ref().unwrap().crate_name, "crates/x86-runtime");
    }

    #[test]
    fn classifies_rust_panic_with_backtrace_and_crate_attribution() {
        let log = "\
[1764304504.198 pid=1234 boot] PANIC on thread 'win32-main': panicked at crates/d3d12-metal/src/metal/query.rs:88:13:
index out of bounds: the len is 1024 but the index is 1024
Backtrace:
   0: std::backtrace::Backtrace::force_capture
             at /rustc/hash/library/std/src/backtrace.rs:330:18
   1: port_loader::init_boot_log::{{closure}}
             at crates/port-loader/src/lib.rs:871:18
   2: d3d12_metal::metal::query::QueryHeap::resolve_range
             at crates/d3d12-metal/src/metal/query.rs:88:13
   3: win32_rt::d3d12::cmdlist::resolve_query_data
             at crates/win32-rt/src/d3d12/cmdlist.rs:910:9";
        let rep = classify_boot_log(&synthetic_input(log, Some(101)));
        assert_eq!(rep.blocker.class(), BlockerClass::RustPanicOrAbort);
        assert_eq!(rep.blocker.subkind, "rust_panic");
        assert!(rep.blocker.backtrace.len() >= 4);
        assert!(rep.blocker.backtrace[2].contains("d3d12_metal::metal::query::QueryHeap::resolve_range"));
        let owner = rep.owner.as_ref().unwrap();
        assert_eq!(owner.crate_name, "crates/d3d12-metal");
    }

    #[test]
    fn classifies_sigabrt_with_backtrace_frames() {
        let log = "\
libc++abi: terminating due to uncaught exception of type std::runtime_error
*** First throw call stack:
(
  0   CoreFoundation  0x0000000180100000 __exceptionPreprocess + 176
  1   port-runner     0x0000000100234560 dxil_metal::helper::compile_shader + 212
)";
        let mut inp = synthetic_input(log, None);
        inp.signal = Some(6);
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::RustPanicOrAbort);
        assert_eq!(rep.blocker.subkind, "sigabrt");
        assert_eq!(rep.blocker.backtrace.len(), 2);
        assert_eq!(rep.owner.as_ref().unwrap().crate_name, "crates/dxil-metal");
    }

    #[test]
    fn classifies_metal_validation_error_ahead_of_resulting_sigabrt() {
        let log = "\
[2026-11-28T04:00:03.100Z INFO  d3d] Metal device: Apple M3 Ultra
-[MTLDebugRenderCommandEncoder validateCommonDrawErrors:]:5775: failed assertion `Draw Errors Validation
Vertex Function(main_vs): argument buffer at index 0 is not bound'
Abort trap: 6";
        let mut inp = synthetic_input(log, Some(134));
        inp.signal = Some(6);
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::MetalValidationError);
        assert_eq!(rep.blocker.subkind, "mtl_debug_assertion");
        assert_eq!(rep.owner.as_ref().unwrap().crate_name, "crates/d3d12-metal");
        assert!(rep.secondary_signals.is_empty());
    }

    #[test]
    fn classifies_d3d12_dxgi_hresult_failure_and_ignores_benign_probes() {
        let log = "\
[2026-11-28T04:00:01.000Z INFO  d3d] D3D12GetDebugInterface({344488b7}) -> E_NOINTERFACE (no debug layer)
[2026-11-28T04:00:01.010Z DEBUG d3d] IDXGIFactory4::EnumWarpAdapter -> DXGI_ERROR_NOT_FOUND (no WARP on Apple Silicon)
[2026-11-28T04:00:01.050Z WARN  d3d] CreateQueryHeap(type=0, count=65536): exceeds pool -> 0x80070057
[2026-11-28T04:00:01.100Z INFO  port_loader] entry point returned 1";
        let rep = classify_boot_log(&synthetic_input(log, Some(1)));
        assert_eq!(rep.blocker.class(), BlockerClass::D3d12DxgiHresult);
        assert_eq!(rep.blocker.subkind, "d3d12_hresult");
        assert_eq!(
            rep.blocker.hresult.as_deref(),
            Some("0x80070057 (E_INVALIDARG)")
        );
        assert!(rep.owner.as_ref().unwrap().crate_name.contains("d3d12-metal"));
    }

    #[test]
    fn classifies_hang_timeout_with_last_activity() {
        let log = "\
[2026-11-28T04:00:01.000Z INFO  port_loader] calling exe entry point testgame.exe+0x1000
[2026-11-28T04:00:02.000Z INFO  d3d] Metal device: Apple M3 Ultra
[2026-11-28T04:00:03.500Z INFO  d3d] ID3D12CommandQueue::ExecuteCommandLists waiting on fence 42
[2026-11-28T04:00:31.000Z WARN  port_loader::run_limits] [run-limit] timed out after 30s (30001 ms, frames_presented=0); exiting with code 124
[2026-11-28T04:00:31.002Z INFO  port_loader::boot_report] boot_report: wrote /tmp/r.json";
        let mut inp = synthetic_input(log, Some(124));
        inp.timed_out = true;
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::HangTimeout);
        assert_eq!(rep.blocker.subkind, "timeout");
        assert_eq!(rep.exit_code(), exit_codes::TIMEOUT);
        assert_eq!(rep.blocker.last_activity.len(), 3);
        assert!(rep
            .blocker
            .last_activity
            .last()
            .unwrap()
            .contains("waiting on fence 42"));
        assert!(rep.owner.as_ref().unwrap().crate_name.contains("d3d12-metal"));
    }

    #[test]
    fn classifies_clean_boot_and_roundtrips_json_envelope() {
        let log = "\
[2026-11-28T04:00:01.000Z INFO  d3d] D3D12GetDebugInterface({344488b7}) -> E_NOINTERFACE (no debug layer)
[2026-11-28T04:00:01.100Z WARN  win32_rt::kernel32::module] GetProcAddress(kernel32.dll, FlsGetValue2) -> NULL (ERROR_PROC_NOT_FOUND)
[2026-11-28T04:00:03.000Z INFO  port_loader::run_limits] [run-limit] reached --max-frames 5 (frames_presented=5, elapsed=2100ms); exiting cleanly";
        let rep = classify_boot_log(&synthetic_input(log, Some(0)));
        assert_eq!(rep.blocker.class(), BlockerClass::None);
        assert_eq!(rep.run.frames_presented, 5);
        assert_eq!(rep.exit_code(), exit_codes::OK);
        assert!(rep.owner.is_none());

        let env = boot_triage_envelope(&rep);
        let parsed = parse_json(&env.to_json_pretty()).expect("valid RFC 8259 JSON");
        assert_eq!(parsed.get("command").and_then(JsonValue::as_str), Some("boot-triage"));
        assert_eq!(parsed.get("ok").and_then(JsonValue::as_bool), Some(true));
        assert_eq!(parsed.get("exit_code").and_then(JsonValue::as_i64), Some(0));
        let summary = format_human_summary(&rep);
        assert!(summary.contains("CLEAN_BOOT"));
    }

    #[test]
    fn split_arg_string_handles_quotes_and_multiple_flags() {
        assert_eq!(split_arg_string("-nolauncher"), vec!["-nolauncher"]);
        assert_eq!(
            split_arg_string(r#"-nolauncher -lang "en US" -foo='bar baz'"#),
            vec!["-nolauncher", "-lang", "en US", "-foo=bar baz"]
        );
        assert_eq!(
            split_arg_string(r#"-config "C:\Game Data\settings.ini" -share "\\server\share\dir""#),
            vec![
                "-config",
                r"C:\Game Data\settings.ini",
                "-share",
                r"\\server\share\dir",
            ]
        );
        assert_eq!(
            split_arg_string(r#"-dir "C:\Game Data\\" -msg "say \"hi\"""#),
            vec!["-dir", r"C:\Game Data\", "-msg", r#"say "hi""#]
        );
    }

    #[test]
    fn crash_json_sidecar_enriches_truncated_log() {
        let crash_json = r#"{
          "schema_version": 1,
          "kind": "crash",
          "title": "unhandled exception",
          "exception": { "code": "0xc000001d", "description": "exception 0xc000001d ILLEGAL_INSTRUCTION", "address": "0x140099900" },
          "code_at_rip": { "address": "0x140099900", "readable": true, "disassembly": ["140099900: 0f0b  ud2"] },
          "guest_state": { "rip": "0x140099900", "symbol": "testgame.exe+0x99900 [va 0x140099900]", "frames": [ { "index": 0, "symbol": "testgame.exe+0x99900" } ] },
          "backtrace": { "frames": [] }
        }"#;
        let mut inp = synthetic_input("[2026-11-28T04:00:01.000Z INFO  port_loader] starting initial thread", Some(29));
        inp.crash_report_json = Some(crash_json.to_string());
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::GuestCrash);
        assert_eq!(rep.blocker.location.as_deref(), Some("testgame.exe+0x99900"));
        assert_eq!(rep.blocker.faulting_instruction.as_deref(), Some("ud2"));
    }

    #[test]
    fn ignores_benign_query_interface_probes_and_does_not_duplicate_stub_hresult() {
        let log = "\
[2026-11-28T04:00:01.000Z WARN  com] ID3D11Device1::QueryInterface({9D06DFFA-D1E5-4D07-83A8-1BB123F2F841}) -> E_NOINTERFACE
[2026-11-28T04:00:01.100Z WARN  stub] STUB create_geometry_shader_so: ID3D11Device::CreateGeometryShaderWithStreamOutput -> E_NOTIMPL
[2026-11-28T04:00:01.200Z INFO  port_loader] entry point returned 1";
        let rep = classify_boot_log(&synthetic_input(log, Some(1)));
        assert_eq!(rep.blocker.class(), BlockerClass::StubHit);
        assert_eq!(rep.blocker.symbol.as_deref(), Some("create_geometry_shader_so"));
        assert!(
            rep.secondary_signals.is_empty(),
            "QueryInterface E_NOINTERFACE probe and stub! E_NOTIMPL must not duplicate as d3d12_dxgi_hresult: {:?}",
            rep.secondary_signals
        );
    }

    #[test]
    fn prefers_symbolized_faulting_thread_backtrace_over_leaf_guest_backtrace() {
        let log = "\
[2026-11-28T04:00:01.000Z ERROR port_loader::thunks] UNIMPLEMENTED kernel32.dll!ExampleMissingApi (imported by testgame.exe) called from testgame.exe+0x318575a [va 0x14318575a]
[2026-11-28T04:00:01.001Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unimplemented import kernel32.dll!ExampleMissingApi ====================
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash] guest backtrace (none, 1 frame(s)):
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash]   #0  0x105340fbd [leaf]
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash] backtrace (faulting thread, stack 0x1000..0x2000):
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash]   #0  testgame.exe+0x318575a [va 0x14318575a] [pdata]
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash]   #1  testgame.exe+0x3186fe0 [va 0x143186fe0] [pdata]
[2026-11-28T04:00:01.003Z ERROR win32_rt::crash] host backtrace (libSystem unwinder):
[2026-11-28T04:00:01.003Z ERROR win32_rt::crash]   #0  0x1039111d4 [host port-runner!win32_rt::crash::report+0x23d4]
[2026-11-28T04:00:01.004Z ERROR win32_rt::crash] ==================== end of crash report (4.0ms) ====================";
        let rep = classify_boot_log(&synthetic_input(log, Some(57)));
        assert_eq!(rep.blocker.class(), BlockerClass::UnresolvedImport);
        assert_eq!(rep.blocker.backtrace.len(), 2);
        assert!(rep.blocker.backtrace[0].contains("testgame.exe+0x318575a"));
        assert!(rep.blocker.backtrace[1].contains("testgame.exe+0x3186fe0"));
    }

    #[test]
    fn ignores_metal_validation_enabled_banner_on_clean_boot() {
        let clean_log = "\
2026-11-28 04:00:01.000 port-runner[1234:5678] Metal API Validation Enabled
2026-11-28 04:00:01.001 port-runner[1234:5678] Metal GPU Validation Enabled
[2026-11-28T04:00:03.000Z INFO  port_loader::run_limits] [run-limit] reached --max-frames 5 (frames_presented=5, elapsed=1800ms); exiting cleanly";
        let rep = classify_boot_log(&synthetic_input(clean_log, Some(0)));
        assert_eq!(rep.blocker.class(), BlockerClass::None);
        assert_eq!(rep.blocker.subkind, "rendered_frames");
        assert_eq!(rep.exit_code(), exit_codes::OK);
        assert!(rep.secondary_signals.is_empty());

        // Real Metal API Validation failure still classifies as MetalValidationError.
        let fail_log = "\
2026-11-28 04:00:01.000 port-runner[1234:5678] Metal API Validation Enabled
2026-11-28 04:00:02.000 port-runner[1234:5678] Metal API Validation Error: texture usage does not permit render target";
        let fail_rep = classify_boot_log(&synthetic_input(fail_log, Some(1)));
        assert_eq!(fail_rep.blocker.class(), BlockerClass::MetalValidationError);
        assert_eq!(fail_rep.blocker.log_line, Some(2));
    }

    #[test]
    fn offline_classification_honors_boot_report_nonzero_exit_code() {
        let log = "[2026-11-28T04:00:01.000Z INFO  port_loader] calling exe entry point testgame.exe+0x1000";
        let boot_report = r#"{
          "schema_version": 1,
          "exit": { "kind": "clean_exit", "code": 1, "detail": "", "elapsed_ms": 250, "frames_presented": 0, "graphics_api": "unknown" },
          "stubs": { "total_hits": 0, "unique_sites": 0, "entries": [] },
          "unresolved_imports": []
        }"#;
        let mut inp = synthetic_input(log, None);
        inp.boot_report_json = Some(boot_report.to_string());
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::UnknownFailure);
        assert_eq!(rep.run.exit_code, Some(1));
        assert_eq!(rep.exit_code(), exit_codes::RUNTIME_ERROR);
    }

    #[test]
    fn offline_boot_report_nonzero_exit_does_not_suppress_stub_when_frames_presented() {
        let log = "\
[2026-11-28T04:00:01.000Z INFO  port_loader] calling exe entry point testgame.exe+0x1000
[2026-11-28T04:00:02.000Z WARN  stub] STUB CreateQueryHeapEx: CreateQueryHeapEx -> 0x80004001 (last error 120)";
        let boot_report = r#"{
          "schema_version": 1,
          "exit": { "kind": "clean_exit", "code": 2, "detail": "", "elapsed_ms": 900, "frames_presented": 2, "graphics_api": "d3d12" },
          "stubs": { "total_hits": 1, "unique_sites": 1, "entries": [] },
          "unresolved_imports": []
        }"#;
        let mut inp = synthetic_input(log, None);
        inp.boot_report_json = Some(boot_report.to_string());
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::StubHit);
        assert_eq!(rep.run.exit_code, Some(2));
        assert_eq!(rep.exit_code(), exit_codes::RUNTIME_ERROR);
    }

    #[test]
    fn nonfatal_unimplemented_x86_op_does_not_fail_clean_boot_or_hide_real_crash() {
        // PORT_FAIL_ON_UNIMPL_OP=0 logs the opcode and continues to a clean boot.
        let clean_log = "\
[2026-11-28T04:00:01.000Z ERROR x86_runtime::emu::interp] emu: UNIMPLEMENTED mnemonic Xgetbv at 0x140010000 `xgetbv`
[2026-11-28T04:00:03.000Z INFO  port_loader::run_limits] [run-limit] reached --max-frames 5 (frames_presented=5, elapsed=1500ms); exiting cleanly";
        let clean_rep = classify_boot_log(&synthetic_input(clean_log, Some(0)));
        assert_eq!(clean_rep.blocker.class(), BlockerClass::None);
        assert_eq!(clean_rep.exit_code(), exit_codes::OK);
        assert_eq!(clean_rep.secondary_signals.len(), 1);
        assert_eq!(
            clean_rep.secondary_signals[0].subkind,
            "unimplemented_x86_op_nonfatal"
        );

        // Nonfatal opcode earlier in the log must not hide a later real guest crash.
        let crash_log = "\
[2026-11-28T04:00:01.000Z ERROR x86_runtime::emu::interp] emu: UNIMPLEMENTED mnemonic Xgetbv at 0x140010000 `xgetbv`
[2026-11-28T04:00:02.000Z ERROR win32_rt::unwind] UNHANDLED exception 0xc0000005 ACCESS_VIOLATION flags=0x0 at testgame.exe+0x20000 [va 0x140020000]
[2026-11-28T04:00:02.001Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled exception ====================
[2026-11-28T04:00:02.001Z ERROR win32_rt::crash] exception 0xc0000005 ACCESS_VIOLATION flags=0x0 at testgame.exe+0x20000
[2026-11-28T04:00:02.002Z ERROR win32_rt::crash] ==================== end of crash report (1.0ms) ====================";
        let crash_rep = classify_boot_log(&synthetic_input(crash_log, Some(5)));
        assert_eq!(crash_rep.blocker.class(), BlockerClass::GuestCrash);
        assert_eq!(crash_rep.blocker.subkind, "access_violation");
        assert_eq!(
            crash_rep.blocker.location.as_deref(),
            Some("testgame.exe+0x20000")
        );
        assert_eq!(crash_rep.secondary_signals.len(), 1);
        assert_eq!(
            crash_rep.secondary_signals[0].subkind,
            "unimplemented_x86_op_nonfatal"
        );
    }

    #[test]
    fn crash_json_sidecar_recognizes_unimplemented_import_and_stall() {
        let import_crash_json = r#"{
          "schema_version": 1,
          "kind": "unimplemented_import",
          "title": "unimplemented import kernel32.dll!GetGeoInfoA",
          "exception": null,
          "guest_state": { "rip": "0x105340fbd", "symbol": "0x105340fbd", "frames": [ { "index": 0, "symbol": "0x105340fbd [leaf]" } ] },
          "backtrace": { "frames": [ { "index": 0, "symbol": "testgame.exe+0x318575a [va 0x14318575a]" }, { "index": 1, "symbol": "testgame.exe+0x3186fe0 [va 0x143186fe0]" } ] }
        }"#;
        let mut inp = synthetic_input(
            "[2026-11-28T04:00:01.000Z INFO  port_loader] starting initial thread",
            None,
        );
        inp.crash_report_json = Some(import_crash_json.to_string());
        let rep = classify_boot_log(&inp);
        assert_eq!(rep.blocker.class(), BlockerClass::UnresolvedImport);
        assert_eq!(rep.blocker.subkind, "static_trap");
        assert_eq!(rep.blocker.dll.as_deref(), Some("kernel32.dll"));
        assert_eq!(rep.blocker.symbol.as_deref(), Some("GetGeoInfoA"));
        assert_eq!(
            rep.blocker.location.as_deref(),
            Some("testgame.exe+0x318575a")
        );
        assert_eq!(rep.blocker.backtrace.len(), 2);
        assert_eq!(rep.exit_code(), exit_codes::UNRESOLVED_IMPORTS);
        assert_eq!(rep.owner.as_ref().unwrap().crate_name, "crates/win32-rt");

        let stall_crash_json = r#"{
          "schema_version": 1,
          "kind": "stall",
          "title": "watchdog stall detected",
          "exception": null
        }"#;
        let mut stall_inp = synthetic_input(
            "[2026-11-28T04:00:01.000Z INFO  d3d] waiting on fence",
            None,
        );
        stall_inp.crash_report_json = Some(stall_crash_json.to_string());
        let stall_rep = classify_boot_log(&stall_inp);
        assert_eq!(stall_rep.blocker.class(), BlockerClass::HangTimeout);
        assert_eq!(stall_rep.blocker.subkind, "watchdog_stall");
        assert_eq!(stall_rep.exit_code(), exit_codes::TIMEOUT);
    }

    #[test]
    fn recovered_unhandled_exception_is_demoted_to_secondary_signal() {
        let provisional_crash_json = r#"{
          "schema_version": 1,
          "kind": "crash",
          "title": "unhandled exception",
          "exception": { "code": "0xc0000005", "description": "exception 0xc0000005 ACCESS_VIOLATION" },
          "guest_state": { "rip": "0x140012ab0", "symbol": "testgame.exe+0x12ab0", "frames": [] },
          "backtrace": { "frames": [] }
        }"#;

        // 1. Native fatal_unhandled restores context via EXCEPTION_CONTINUE_EXECUTION, then exits cleanly after 5 frames.
        let clean_after_filter = "\
[2026-11-28T04:00:01.000Z ERROR win32_rt::unwind] UNHANDLED exception 0xc0000005 ACCESS_VIOLATION flags=0x0 at testgame.exe+0x12ab0
[2026-11-28T04:00:01.001Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled exception ====================
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash] ==================== end of crash report (1.0ms) ====================
[2026-11-28T04:00:01.003Z ERROR win32_rt::unwind] calling the application's unhandled exception filter at testgame.exe+0x1000
[2026-11-28T04:00:03.000Z INFO  port_loader::run_limits] [run-limit] reached --max-frames 5 (frames_presented=5, elapsed=2000ms); exiting cleanly";
        let mut clean_inp = synthetic_input(clean_after_filter, Some(0));
        clean_inp.crash_report_json = Some(provisional_crash_json.to_string());
        let clean_rep = classify_boot_log(&clean_inp);
        assert_eq!(clean_rep.blocker.class(), BlockerClass::None);
        assert_eq!(clean_rep.blocker.subkind, "rendered_frames");
        assert_eq!(clean_rep.exit_code(), exit_codes::OK);
        assert_eq!(clean_rep.secondary_signals.len(), 1);
        assert_eq!(
            clean_rep.secondary_signals[0].subkind,
            "recovered_exception"
        );

        // 2. Emulated exception continued by filter, then process later times out.
        let timeout_after_continue = "\
[2026-11-28T04:00:01.000Z ERROR win32_rt::unwind] emu: UNHANDLED guest exception 0xc0000005 ACCESS_VIOLATION at rip 0x140012ab0 testgame.exe+0x12ab0
[2026-11-28T04:00:01.001Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled emulated guest exception ====================
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash] ==================== end of crash report (1.0ms) ====================
[2026-11-28T04:00:01.003Z ERROR win32_rt::unwind] calling the application's unhandled exception filter at testgame.exe+0x1000
[2026-11-28T04:00:01.004Z ERROR win32_rt::unwind] emu: unhandled exception filter continued execution at 0x140012ab3
[2026-11-28T04:00:31.000Z WARN  port_loader::run_limits] [run-limit] timed out after 30s (30001 ms, frames_presented=0); exiting with code 124";
        let mut to_inp = synthetic_input(timeout_after_continue, Some(124));
        to_inp.timed_out = true;
        to_inp.crash_report_json = Some(provisional_crash_json.to_string());
        let to_rep = classify_boot_log(&to_inp);
        assert_eq!(to_rep.blocker.class(), BlockerClass::HangTimeout);
        assert_eq!(to_rep.exit_code(), exit_codes::TIMEOUT);
        assert_eq!(to_rep.secondary_signals.len(), 1);
        assert_eq!(to_rep.secondary_signals[0].subkind, "recovered_exception");

        // 3. Recovered exception followed by a second genuinely fatal crash keeps the second crash as primary.
        let second_crash = "\
[2026-11-28T04:00:01.000Z ERROR win32_rt::unwind] UNHANDLED exception 0xc0000005 ACCESS_VIOLATION flags=0x0 at testgame.exe+0x12ab0
[2026-11-28T04:00:01.001Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled exception ====================
[2026-11-28T04:00:01.002Z ERROR win32_rt::crash] ==================== end of crash report (1.0ms) ====================
[2026-11-28T04:00:01.003Z ERROR win32_rt::unwind] calling the application's unhandled exception filter at testgame.exe+0x1000
[2026-11-28T04:00:02.000Z ERROR win32_rt::unwind] UNHANDLED exception 0xc000001d ILLEGAL_INSTRUCTION flags=0x0 at testgame.exe+0x99900
[2026-11-28T04:00:02.001Z ERROR win32_rt::crash] ==================== AW2 CRASH REPORT: unhandled exception ====================
[2026-11-28T04:00:02.001Z ERROR win32_rt::crash] exception 0xc000001d ILLEGAL_INSTRUCTION flags=0x0 at testgame.exe+0x99900
[2026-11-28T04:00:02.002Z ERROR win32_rt::crash] ==================== end of crash report (1.0ms) ====================";
        let second_rep = classify_boot_log(&synthetic_input(second_crash, Some(29)));
        assert_eq!(second_rep.blocker.class(), BlockerClass::GuestCrash);
        assert_eq!(second_rep.blocker.subkind, "illegal_instruction");
        assert_eq!(
            second_rep.blocker.location.as_deref(),
            Some("testgame.exe+0x99900")
        );
        assert_eq!(second_rep.secondary_signals.len(), 1);
        assert_eq!(
            second_rep.secondary_signals[0].subkind,
            "recovered_exception"
        );
    }
}
