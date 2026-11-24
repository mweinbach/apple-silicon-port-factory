//! Guest workload benchmark suite and chip-keyed baseline comparator
//! (`cargo opsdb -- bench`).
//!
//! Compiles four self-checking Windows x86-64 guest workloads from embedded C
//! source using `zig cc -target x86_64-windows-gnu -O2`:
//! 1. `compression_loop` — LZ77-style hash-chain compress/decompress + checksum
//! 2. `simd_kernel` — AVX2/SSE2 float32 blocked GEMM + dot-product kernel
//! 3. `lock_heavy_job_queue` — multi-threaded SRWLOCK/CONDITION_VARIABLE MPMC queue
//! 4. `d3d12_draw_heavy_frame` — headless D3D12 500-draw-call frame + fence sync + readback
//!
//! Each workload runs inside `port-runner` (headless), writes structured JSON
//! results, and compares median steady-state times against a committed baseline
//! (`db/bench_baselines.json`) keyed by host chip (`sysctl -n machdep.cpu.brand_string`)
//! with a configurable regression tolerance and clear PASS/FAIL status.

use crate::bundle_common::{find_or_build_host_port_runner, TempDirGuard};
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub const DEFAULT_BASELINE_PATH: &str = "db/bench_baselines.json";
pub const DEFAULT_OUT_PATH: &str = "out/bench_results.json";
pub const DEFAULT_TOLERANCE: f64 = 0.25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchMode {
    Aot,
    Jit,
}

impl BenchMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Aot => "aot",
            Self::Jit => "jit",
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "aot" => Ok(Self::Aot),
            "jit" => Ok(Self::Jit),
            other => bail!("invalid bench mode {other:?} (expected 'aot' or 'jit')"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkloadSpec {
    pub id: &'static str,
    pub description: &'static str,
    pub source: &'static str,
    pub extra_cflags: &'static [&'static str],
    pub libs: &'static [&'static str],
    pub requires_gpu_shaders: bool,
}

pub const WORKLOADS: &[WorkloadSpec] = &[
    WorkloadSpec {
        id: "compression_loop",
        description: "LZ77 hash-chain sliding-window compress + decompress verification loop",
        source: COMPRESSION_LOOP_C,
        extra_cflags: &[],
        libs: &["kernel32"],
        requires_gpu_shaders: false,
    },
    WorkloadSpec {
        id: "simd_kernel",
        description: "AVX2+FMA / SSE2 float32 blocked GEMM + vector reduction kernel",
        source: SIMD_KERNEL_C,
        extra_cflags: &["-mavx2", "-mfma"],
        libs: &["kernel32"],
        requires_gpu_shaders: false,
    },
    WorkloadSpec {
        id: "lock_heavy_job_queue",
        description: "4-worker SRWLOCK + CONDITION_VARIABLE MPMC ring-buffer job queue",
        source: LOCK_HEAVY_JOB_QUEUE_C,
        extra_cflags: &[],
        libs: &["kernel32"],
        requires_gpu_shaders: false,
    },
    WorkloadSpec {
        id: "d3d12_draw_heavy_frame",
        description: "Headless D3D12 500-draw-call frame submission + GPU fence sync + pixel readback",
        source: D3D12_DRAW_HEAVY_C,
        extra_cflags: &[],
        libs: &["d3d12", "kernel32"],
        requires_gpu_shaders: true,
    },
];

#[derive(Debug, Clone)]
pub struct WorkloadMeasurement {
    pub id: String,
    pub mode: BenchMode,
    pub median_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub host_wall_ms: f64,
    pub iters: u32,
    pub unit: String,
    pub throughput: f64,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareStatus {
    Pass,
    Fail,
    NoBaseline,
}

impl CompareStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::NoBaseline => "NO_BASELINE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkloadComparison {
    pub measurement: WorkloadMeasurement,
    pub baseline_ms: Option<f64>,
    pub delta_pct: Option<f64>,
    pub tolerance_pct: f64,
    pub status: CompareStatus,
}

#[derive(Debug, Clone)]
pub struct BenchReport {
    pub chip: String,
    pub os_version: String,
    pub runner_profile: String,
    pub mode: BenchMode,
    pub tolerance: f64,
    pub comparisons: Vec<WorkloadComparison>,
    pub overall_pass: bool,
}

/// Baseline database keyed by chip -> mode ("aot"|"jit") -> workload_id -> baseline_ms.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BaselineDb {
    pub chips: BTreeMap<String, BTreeMap<String, BTreeMap<String, f64>>>,
}

#[derive(Debug, Clone)]
pub struct BenchOptions {
    pub workspace_root: PathBuf,
    pub mode: BenchMode,
    pub only: Vec<String>,
    pub tolerance: f64,
    pub baseline_path: PathBuf,
    pub out_path: Option<PathBuf>,
    pub runner_override: Option<PathBuf>,
    pub update_baseline: bool,
    pub no_compare: bool,
    pub print_json: bool,
}

pub fn detect_host_chip() -> String {
    if let Ok(v) = std::env::var("PORT_BENCH_CHIP") {
        let trimmed = v.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return s;
                }
            }
        }
    }
    format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS)
}

fn detect_os_version() -> String {
    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = Command::new("sw_vers").arg("-productVersion").output() {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    return format!("macOS {s}");
                }
            }
        }
    }
    std::env::consts::OS.to_string()
}

fn find_zig() -> Result<PathBuf> {
    if let Some(p) = std::env::var_os("ZIG").filter(|v| !v.is_empty()) {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Ok(pb);
        }
    }
    if let Some(path_env) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&path_env) {
            let cand = dir.join("zig");
            if cand.is_file() {
                return Ok(cand);
            }
        }
    }
    for fixed in ["/opt/homebrew/bin/zig", "/usr/local/bin/zig"] {
        let pb = PathBuf::from(fixed);
        if pb.is_file() {
            return Ok(pb);
        }
    }
    bail!("zig compiler not found (install zig or set ZIG=/path/to/zig)")
}

fn has_shader_converters() -> bool {
    let has_dxilconv = std::env::var_os("AW2_DXILCONV")
        .filter(|v| !v.is_empty())
        .is_some_and(|p| Path::new(&p).is_file())
        || std::env::var_os("HOME")
            .is_some_and(|h| PathBuf::from(h).join(".local/lib/libdxilconv.dylib").is_file())
        || Path::new("/opt/homebrew/lib/libdxilconv.dylib").is_file()
        || Path::new("/usr/local/lib/libdxilconv.dylib").is_file();
    let has_ir = std::env::var_os("METAL_IRCONVERTER_DYLIB")
        .filter(|v| !v.is_empty())
        .is_some_and(|p| Path::new(&p).is_file())
        || std::env::var_os("HOME").is_some_and(|h| {
            PathBuf::from(h)
                .join(".local/opt/metal-shaderconverter/lib/libmetalirconverter.dylib")
                .is_file()
        })
        || Path::new("/usr/local/lib/libmetalirconverter.dylib").is_file()
        || Path::new("/opt/metal-shaderconverter/lib/libmetalirconverter.dylib").is_file();
    has_dxilconv && has_ir
}

fn compile_workload(zig: &Path, work_dir: &Path, spec: &WorkloadSpec) -> Result<PathBuf> {
    let src_path = work_dir.join(format!("{}.c", spec.id));
    let exe_path = work_dir.join(format!("{}.exe", spec.id));
    fs::write(&src_path, spec.source)
        .with_context(|| format!("writing benchmark source {}", src_path.display()))?;
    let zig_cache = work_dir.join("zig-cache");
    let mut cmd = Command::new(zig);
    cmd.args(["cc", "-target", "x86_64-windows-gnu", "-O2", "-Wall", "-Wextra"])
        .args(spec.extra_cflags)
        .arg(&src_path)
        .arg("-o")
        .arg(&exe_path);
    for lib in spec.libs {
        cmd.arg(format!("-l{lib}"));
    }
    cmd.env("ZIG_LOCAL_CACHE_DIR", &zig_cache);
    let out = cmd
        .output()
        .with_context(|| format!("compiling benchmark {} with zig cc", spec.id))?;
    if !out.status.success() {
        bail!(
            "zig cc failed for {} (status={:?}):\nstdout:\n{}\nstderr:\n{}",
            spec.id,
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(exe_path)
}

fn prewarm_workload(runner: &Path, work_dir: &Path, exe_path: &Path, aot_dir: &Path) -> Result<()> {
    let log_dir = work_dir.join("logs-prewarm");
    let drive_c = work_dir.join("drive_c-prewarm");
    fs::create_dir_all(&log_dir)?;
    fs::create_dir_all(&drive_c)?;
    fs::create_dir_all(aot_dir)?;

    let out = Command::new(runner)
        .arg("--pe")
        .arg(exe_path)
        .arg("--prewarm-aot")
        .arg("--aot-dir")
        .arg(aot_dir)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", aot_dir)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("PORT_HEADLESS", "1")
        .env("AW2_HEADLESS", "1")
        .env("RUST_LOG", "warn")
        .output()
        .with_context(|| format!("prewarming AOT for {}", exe_path.display()))?;
    if !out.status.success() {
        bail!(
            "prewarm-aot failed for {} (status={:?}):\n{}",
            exe_path.display(),
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(())
}

pub fn parse_bench_result_line(
    line: &str,
    mode: BenchMode,
    host_wall_ms: f64,
) -> Option<WorkloadMeasurement> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("BENCH_RESULT ")?;
    let mut id = None;
    let mut median_ms = None;
    let mut min_ms = None;
    let mut max_ms = None;
    let mut iters = None;
    let mut unit = None;
    let mut throughput = None;
    let mut checksum = None;
    for tok in rest.split_whitespace() {
        let Some((k, v)) = tok.split_once('=') else {
            continue;
        };
        match k {
            "name" => id = Some(v.to_string()),
            "median_ms" => median_ms = v.parse::<f64>().ok(),
            "min_ms" => min_ms = v.parse::<f64>().ok(),
            "max_ms" => max_ms = v.parse::<f64>().ok(),
            "iters" => iters = v.parse::<u32>().ok(),
            "unit" => unit = Some(v.to_string()),
            "throughput" => throughput = v.parse::<f64>().ok(),
            "checksum" => checksum = Some(v.to_string()),
            _ => {}
        }
    }
    Some(WorkloadMeasurement {
        id: id?,
        mode,
        median_ms: median_ms?,
        min_ms: min_ms?,
        max_ms: max_ms?,
        host_wall_ms,
        iters: iters?,
        unit: unit?,
        throughput: throughput?,
        checksum: checksum?,
    })
}

fn run_workload(
    runner: &Path,
    work_dir: &Path,
    spec: &WorkloadSpec,
    exe_path: &Path,
    aot_dir: &Path,
    mode: BenchMode,
) -> Result<WorkloadMeasurement> {
    if mode == BenchMode::Aot {
        prewarm_workload(runner, work_dir, exe_path, aot_dir)?;
    }

    let log_dir = work_dir.join(format!("logs-{}", spec.id));
    let drive_c = work_dir.join(format!("drive_c-{}", spec.id));
    fs::create_dir_all(&log_dir)?;
    fs::create_dir_all(&drive_c)?;

    let mut cmd = Command::new(runner);
    cmd.arg("--pe")
        .arg(exe_path)
        .env("PORT_LOG_DIR", &log_dir)
        .env("PORT_AOT_DIR", aot_dir)
        .env("AW2_DRIVE_C", &drive_c)
        .env("TRANSLATOR_HOME", &drive_c)
        .env("PORT_HEADLESS", "1")
        .env("AW2_HEADLESS", "1")
        .env("PORT_AOT_GUARD", "0")
        .env("RUST_LOG", "warn")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    match mode {
        BenchMode::Aot => {
            cmd.env("PORT_AOT", "1").env("AW2_DLL_AOT", "1");
        }
        BenchMode::Jit => {
            cmd.env("PORT_AOT", "0").env("AW2_DLL_AOT", "0");
        }
    }

    let t0 = Instant::now();
    let mut child = cmd
        .spawn()
        .with_context(|| format!("spawning port-runner for {}", spec.id))?;
    let out_pipe = child.stdout.take();
    let err_pipe = child.stderr.take();
    let out_reader = std::thread::spawn(move || {
        use std::io::Read;
        let mut buf = Vec::new();
        if let Some(mut p) = out_pipe {
            let _ = p.read_to_end(&mut buf);
        }
        String::from_utf8_lossy(&buf).into_owned()
    });
    let err_reader = std::thread::spawn(move || {
        use std::io::Read;
        let mut buf = Vec::new();
        if let Some(mut p) = err_pipe {
            let _ = p.read_to_end(&mut buf);
        }
        String::from_utf8_lossy(&buf).into_owned()
    });
    let timeout = Duration::from_secs(120);
    let code = loop {
        if let Some(st) = child.try_wait()? {
            break st.code();
        }
        if t0.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            bail!("benchmark {} timed out after {}s", spec.id, timeout.as_secs());
        }
        std::thread::sleep(Duration::from_millis(15));
    };
    let host_wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let stdout = out_reader.join().unwrap_or_default();
    let stderr = err_reader.join().unwrap_or_default();
    let log_file = fs::read_to_string(log_dir.join("port-runner.log")).unwrap_or_default();

    if code != Some(0) {
        bail!(
            "benchmark {} failed (exit={code:?}):\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}\n--- log ---\n{log_file}",
            spec.id
        );
    }

    for line in stdout.lines().chain(log_file.lines()) {
        if let Some(m) = parse_bench_result_line(line, mode, host_wall_ms) {
            return Ok(m);
        }
    }
    bail!(
        "benchmark {} exited 0 but emitted no BENCH_RESULT line:\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
        spec.id
    )
}

pub fn compare_against_baseline(
    measurements: &[WorkloadMeasurement],
    db: &BaselineDb,
    chip: &str,
    mode: BenchMode,
    tolerance: f64,
    no_compare: bool,
) -> (Vec<WorkloadComparison>, bool) {
    let chip_mode_map = db
        .chips
        .get(chip)
        .and_then(|modes| modes.get(mode.as_str()));
    let mut comparisons = Vec::with_capacity(measurements.len());
    let mut overall_pass = true;

    for m in measurements {
        let baseline_ms = chip_mode_map.and_then(|wl| wl.get(&m.id).copied());
        let (delta_pct, status) = match baseline_ms {
            Some(base) if base > 0.0 => {
                let delta = (m.median_ms - base) / base;
                let st = if delta <= tolerance {
                    CompareStatus::Pass
                } else {
                    overall_pass = false;
                    CompareStatus::Fail
                };
                (Some(delta * 100.0), st)
            }
            _ => {
                if !no_compare {
                    overall_pass = false;
                }
                (None, CompareStatus::NoBaseline)
            }
        };
        comparisons.push(WorkloadComparison {
            measurement: m.clone(),
            baseline_ms,
            delta_pct,
            tolerance_pct: tolerance * 100.0,
            status,
        });
    }
    (comparisons, overall_pass)
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub fn format_report_json(report: &BenchReport) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    let _ = writeln!(s, "  \"chip\": {},", json_escape(&report.chip));
    let _ = writeln!(s, "  \"os_version\": {},", json_escape(&report.os_version));
    let _ = writeln!(
        s,
        "  \"runner_profile\": {},",
        json_escape(&report.runner_profile)
    );
    let _ = writeln!(s, "  \"mode\": {},", json_escape(report.mode.as_str()));
    let _ = writeln!(s, "  \"tolerance\": {:.4},", report.tolerance);
    let _ = writeln!(s, "  \"overall_pass\": {},", report.overall_pass);
    s.push_str("  \"workloads\": [\n");
    for (idx, c) in report.comparisons.iter().enumerate() {
        let m = &c.measurement;
        s.push_str("    {\n");
        let _ = writeln!(s, "      \"id\": {},", json_escape(&m.id));
        let _ = writeln!(s, "      \"mode\": {},", json_escape(m.mode.as_str()));
        let _ = writeln!(s, "      \"median_ms\": {:.3},", m.median_ms);
        let _ = writeln!(s, "      \"min_ms\": {:.3},", m.min_ms);
        let _ = writeln!(s, "      \"max_ms\": {:.3},", m.max_ms);
        let _ = writeln!(s, "      \"host_wall_ms\": {:.3},", m.host_wall_ms);
        let _ = writeln!(s, "      \"iters\": {},", m.iters);
        let _ = writeln!(s, "      \"throughput\": {:.3},", m.throughput);
        let _ = writeln!(s, "      \"unit\": {},", json_escape(&m.unit));
        let _ = writeln!(s, "      \"checksum\": {},", json_escape(&m.checksum));
        match c.baseline_ms {
            Some(b) => {
                let _ = writeln!(s, "      \"baseline_ms\": {:.3},", b);
            }
            None => {
                s.push_str("      \"baseline_ms\": null,\n");
            }
        }
        match c.delta_pct {
            Some(d) => {
                let _ = writeln!(s, "      \"delta_pct\": {:.2},", d);
            }
            None => {
                s.push_str("      \"delta_pct\": null,\n");
            }
        }
        let _ = writeln!(s, "      \"status\": {}", json_escape(c.status.as_str()));
        if idx + 1 < report.comparisons.len() {
            s.push_str("    },\n");
        } else {
            s.push_str("    }\n");
        }
    }
    s.push_str("  ]\n");
    s.push_str("}\n");
    s
}

pub fn format_baseline_db_json(db: &BaselineDb) -> String {
    let mut s = String::new();
    s.push_str("{\n");
    s.push_str("  \"schema_version\": 1,\n");
    s.push_str("  \"chips\": {\n");
    let chip_len = db.chips.len();
    for (ci, (chip, modes)) in db.chips.iter().enumerate() {
        let _ = writeln!(s, "    {}: {{", json_escape(chip));
        let mode_len = modes.len();
        for (mi, (mode, workloads)) in modes.iter().enumerate() {
            let _ = writeln!(s, "      {}: {{", json_escape(mode));
            let wl_len = workloads.len();
            for (wi, (wl, ms)) in workloads.iter().enumerate() {
                let comma = if wi + 1 < wl_len { "," } else { "" };
                let _ = writeln!(s, "        {}: {:.3}{comma}", json_escape(wl), ms);
            }
            let comma = if mi + 1 < mode_len { "," } else { "" };
            let _ = writeln!(s, "      }}{comma}");
        }
        let comma = if ci + 1 < chip_len { "," } else { "" };
        let _ = writeln!(s, "    }}{comma}");
    }
    s.push_str("  }\n");
    s.push_str("}\n");
    s
}

// Minimal recursive-descent JSON parser for `db/bench_baselines.json` (zero extra deps).
#[derive(Debug, Clone)]
enum JsonVal {
    Object(BTreeMap<String, JsonVal>),
    Number(f64),
    Scalar,
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            bytes: input.as_bytes(),
            pos: 0,
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.skip_ws();
        self.bytes.get(self.pos).copied()
    }

    fn expect(&mut self, b: u8) -> Result<()> {
        self.skip_ws();
        if self.bytes.get(self.pos) == Some(&b) {
            self.pos += 1;
            Ok(())
        } else {
            bail!(
                "JSON syntax error at byte {}: expected '{}'",
                self.pos,
                b as char
            )
        }
    }

    fn parse_string(&mut self) -> Result<String> {
        self.expect(b'"')?;
        let mut out = String::new();
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            self.pos += 1;
            match b {
                b'"' => return Ok(out),
                b'\\' => {
                    if self.pos >= self.bytes.len() {
                        bail!("unterminated escape in JSON string");
                    }
                    let esc = self.bytes[self.pos];
                    self.pos += 1;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        _ => bail!("unsupported JSON escape '\\{}'", esc as char),
                    }
                }
                other => out.push(other as char),
            }
        }
        bail!("unterminated JSON string")
    }

    fn parse_number(&mut self) -> Result<f64> {
        self.skip_ws();
        let start = self.pos;
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b.is_ascii_digit() || matches!(b, b'-' | b'+' | b'.' | b'e' | b'E') {
                self.pos += 1;
            } else {
                break;
            }
        }
        let s = std::str::from_utf8(&self.bytes[start..self.pos])
            .context("non-UTF8 JSON number")?;
        s.parse::<f64>()
            .with_context(|| format!("invalid JSON number {s:?}"))
    }

    fn parse_object(&mut self) -> Result<BTreeMap<String, JsonVal>> {
        self.expect(b'{')?;
        let mut map = BTreeMap::new();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(map);
        }
        loop {
            let key = self.parse_string()?;
            self.expect(b':')?;
            let val = self.parse_value()?;
            map.insert(key, val);
            match self.peek() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(map);
                }
                _ => bail!("JSON syntax error at byte {}: expected ',' or '}}'", self.pos),
            }
        }
    }

    fn parse_value(&mut self) -> Result<JsonVal> {
        match self.peek() {
            Some(b'{') => Ok(JsonVal::Object(self.parse_object()?)),
            Some(b'"') => {
                let _ = self.parse_string()?;
                Ok(JsonVal::Scalar)
            }
            Some(b't') if self.bytes[self.pos..].starts_with(b"true") => {
                self.pos += 4;
                Ok(JsonVal::Scalar)
            }
            Some(b'f') if self.bytes[self.pos..].starts_with(b"false") => {
                self.pos += 5;
                Ok(JsonVal::Scalar)
            }
            Some(b'n') if self.bytes[self.pos..].starts_with(b"null") => {
                self.pos += 4;
                Ok(JsonVal::Scalar)
            }
            Some(b) if b.is_ascii_digit() || b == b'-' => Ok(JsonVal::Number(self.parse_number()?)),
            other => bail!("unexpected JSON token {:?} at byte {}", other, self.pos),
        }
    }
}

pub fn parse_baseline_db_json(input: &str) -> Result<BaselineDb> {
    let mut parser = JsonParser::new(input);
    let root = match parser.parse_value()? {
        JsonVal::Object(o) => o,
        _ => bail!("baseline JSON root must be an object"),
    };
    let mut db = BaselineDb::default();
    let Some(JsonVal::Object(chips_obj)) = root.get("chips") else {
        return Ok(db);
    };
    for (chip_name, modes_val) in chips_obj {
        let JsonVal::Object(modes_obj) = modes_val else {
            bail!("chip entry {chip_name:?} must be an object");
        };
        let mut mode_map = BTreeMap::new();
        for (mode_name, wl_val) in modes_obj {
            let JsonVal::Object(wl_obj) = wl_val else {
                bail!("mode entry {mode_name:?} under {chip_name:?} must be an object");
            };
            let mut wl_map = BTreeMap::new();
            for (wl_name, num_val) in wl_obj {
                let JsonVal::Number(ms) = num_val else {
                    bail!("workload {wl_name:?} baseline must be a number");
                };
                wl_map.insert(wl_name.clone(), *ms);
            }
            mode_map.insert(mode_name.clone(), wl_map);
        }
        db.chips.insert(chip_name.clone(), mode_map);
    }
    Ok(db)
}

pub fn load_baseline_db(path: &Path) -> Result<BaselineDb> {
    if !path.is_file() {
        return Ok(BaselineDb::default());
    }
    let text = fs::read_to_string(path)
        .with_context(|| format!("reading baseline file {}", path.display()))?;
    parse_baseline_db_json(&text)
        .with_context(|| format!("parsing baseline file {}", path.display()))
}

pub fn format_summary_table(report: &BenchReport) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Port Factory Benchmark ({} | {} | mode={} | runner={} | tolerance=+{:.0}%)",
        report.chip,
        report.os_version,
        report.mode.as_str(),
        report.runner_profile,
        report.tolerance * 100.0
    );
    let _ = writeln!(
        out,
        "{:<24} {:>10} {:>10} {:>9} {:>16} {:>12}",
        "WORKLOAD", "MEDIAN(ms)", "BASE(ms)", "DELTA", "THROUGHPUT", "STATUS"
    );
    let _ = writeln!(out, "{}", "-".repeat(86));
    for c in &report.comparisons {
        let m = &c.measurement;
        let base_s = c
            .baseline_ms
            .map(|b| format!("{b:.3}"))
            .unwrap_or_else(|| "-".to_string());
        let delta_s = c
            .delta_pct
            .map(|d| format!("{d:+.1}%"))
            .unwrap_or_else(|| "n/a".to_string());
        let tp_s = format!("{:.1} {}", m.throughput, m.unit);
        let _ = writeln!(
            out,
            "{:<24} {:>10.3} {:>10} {:>9} {:>16} {:>12}",
            m.id,
            m.median_ms,
            base_s,
            delta_s,
            tp_s,
            c.status.as_str()
        );
    }
    let _ = writeln!(out, "{}", "-".repeat(86));
    let _ = writeln!(
        out,
        "OVERALL: {}",
        if report.overall_pass { "PASS" } else { "FAIL" }
    );
    out
}

pub fn run_benchmarks(opts: &BenchOptions) -> Result<BenchReport> {
    let zig = find_zig()?;
    let runner = match &opts.runner_override {
        Some(p) => {
            if !p.is_file() {
                bail!("specified --runner not found: {}", p.display());
            }
            p.clone()
        }
        None => find_or_build_host_port_runner(&opts.workspace_root)?,
    };
    let runner_profile = if runner.to_string_lossy().contains("/release/") {
        "release".to_string()
    } else {
        "debug".to_string()
    };

    let selected: Vec<&WorkloadSpec> = WORKLOADS
        .iter()
        .filter(|w| {
            opts.only.is_empty()
                || opts
                    .only
                    .iter()
                    .any(|f| w.id.eq_ignore_ascii_case(f) || w.id.contains(f.as_str()))
        })
        .collect();
    if selected.is_empty() {
        bail!(
            "no benchmark workloads matched {:?} (available: {})",
            opts.only,
            WORKLOADS
                .iter()
                .map(|w| w.id)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let tmp = TempDirGuard::create_unique("port-factory-bench")?;
    let aot_dir = tmp.path().join("aot-cache");
    fs::create_dir_all(&aot_dir)?;

    let shaders_ok = has_shader_converters();
    let mut measurements = Vec::with_capacity(selected.len());
    for spec in &selected {
        if spec.requires_gpu_shaders && !shaders_ok {
            bail!(
                "workload {} requires libdxilconv.dylib and libmetalirconverter.dylib (set AW2_DXILCONV / METAL_IRCONVERTER_DYLIB or filter with --only)",
                spec.id
            );
        }
        let exe = compile_workload(&zig, tmp.path(), spec)?;
        let m = run_workload(&runner, tmp.path(), spec, &exe, &aot_dir, opts.mode)?;
        measurements.push(m);
    }

    let chip = detect_host_chip();
    let os_version = detect_os_version();
    let mut db = load_baseline_db(&opts.baseline_path)?;

    if opts.update_baseline {
        let mode_map = db
            .chips
            .entry(chip.clone())
            .or_default()
            .entry(opts.mode.as_str().to_string())
            .or_default();
        for m in &measurements {
            mode_map.insert(m.id.clone(), (m.median_ms * 1000.0).round() / 1000.0);
        }
        if let Some(parent) = opts.baseline_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&opts.baseline_path, format_baseline_db_json(&db))
            .with_context(|| format!("writing updated baseline {}", opts.baseline_path.display()))?;
    }

    let (comparisons, overall_pass) = compare_against_baseline(
        &measurements,
        &db,
        &chip,
        opts.mode,
        opts.tolerance,
        opts.no_compare || opts.update_baseline,
    );

    let report = BenchReport {
        chip,
        os_version,
        runner_profile,
        mode: opts.mode,
        tolerance: opts.tolerance,
        comparisons,
        overall_pass,
    };

    if let Some(ref out_path) = opts.out_path {
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(out_path, format_report_json(&report))
            .with_context(|| format!("writing benchmark report {}", out_path.display()))?;
    }

    Ok(report)
}

// ---------------------------------------------------------------------------
// Embedded Windows x86-64 guest benchmark sources
// ---------------------------------------------------------------------------

const COMPRESSION_LOOP_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Simple deterministic LZ77 hash-chain compressor + decompressor. */
#define BUF_SIZE (256 * 1024)
#define HASH_BITS 14
#define HASH_SIZE (1 << HASH_BITS)

static uint32_t hash3(const uint8_t *p) {
    uint32_t v = ((uint32_t)p[0]) | ((uint32_t)p[1] << 8) | ((uint32_t)p[2] << 16);
    return (v * 2654435761u) >> (32 - HASH_BITS);
}

static size_t lz_compress(const uint8_t *src, size_t len, uint8_t *dst) {
    static int32_t head[HASH_SIZE];
    memset(head, -1, sizeof(head));
    size_t sp = 0, dp = 0;
    while (sp < len) {
        int32_t match_dist = 0;
        size_t match_len = 0;
        if (sp + 4 <= len) {
            uint32_t h = hash3(src + sp);
            int32_t cand = head[h];
            head[h] = (int32_t)sp;
            if (cand >= 0 && (sp - (size_t)cand) <= 4095) {
                size_t dist = sp - (size_t)cand;
                size_t max_l = len - sp;
                if (max_l > 10) max_l = 10;
                size_t l = 0;
                while (l < max_l && src[cand + l] == src[sp + l]) l++;
                if (l >= 3) {
                    match_dist = (int32_t)dist;
                    match_len = l;
                }
            }
        }
        if (match_len >= 3) {
            uint16_t tok = (uint16_t)(((match_len - 3) << 12) | (size_t)match_dist);
            dst[dp++] = 0x80u | (uint8_t)(tok >> 8);
            dst[dp++] = (uint8_t)(tok & 0xFFu);
            sp += match_len;
        } else {
            size_t lit_start = sp++;
            size_t lit_len = 1;
            while (sp < len && lit_len < 127) {
                if (sp + 4 <= len) {
                    uint32_t h = hash3(src + sp);
                    int32_t cand = head[h];
                    if (cand >= 0 && (sp - (size_t)cand) <= 4095 &&
                        src[cand] == src[sp] && src[cand + 1] == src[sp + 1] && src[cand + 2] == src[sp + 2]) {
                        break;
                    }
                    head[h] = (int32_t)sp;
                }
                sp++;
                lit_len++;
            }
            dst[dp++] = (uint8_t)lit_len;
            memcpy(dst + dp, src + lit_start, lit_len);
            dp += lit_len;
        }
    }
    return dp;
}

static size_t lz_decompress(const uint8_t *src, size_t slen, uint8_t *dst, size_t cap) {
    size_t sp = 0, dp = 0;
    while (sp < slen) {
        uint8_t tag = src[sp++];
        if ((tag & 0x80u) == 0) {
            size_t lit = tag;
            if (sp + lit > slen || dp + lit > cap) return 0;
            memcpy(dst + dp, src + sp, lit);
            sp += lit;
            dp += lit;
        } else {
            if (sp >= slen) return 0;
            uint16_t tok = (uint16_t)(((uint16_t)(tag & 0x7Fu) << 8) | src[sp++]);
            size_t mlen = ((tok >> 12) & 0x07u) + 3;
            size_t dist = tok & 0x0FFFu;
            if (dist == 0 || dist > dp || dp + mlen > cap) return 0;
            for (size_t i = 0; i < mlen; ++i) {
                dst[dp + i] = dst[dp - dist + i];
            }
            dp += mlen;
        }
    }
    return dp;
}

static int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a, db = *(const double *)b;
    return (da > db) - (da < db);
}

int main(void) {
    uint8_t *in_buf = (uint8_t *)VirtualAlloc(NULL, BUF_SIZE, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    uint8_t *comp_buf = (uint8_t *)VirtualAlloc(NULL, BUF_SIZE * 2, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    uint8_t *out_buf = (uint8_t *)VirtualAlloc(NULL, BUF_SIZE, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    if (!in_buf || !comp_buf || !out_buf) return 1;

    const char *dict = "The quick brown fox jumps over the lazy dog. Apple Silicon Port Factory 2026! ";
    size_t dlen = strlen(dict);
    uint32_t rng = 0x12345678u;
    for (size_t i = 0; i < BUF_SIZE; ++i) {
        rng = rng * 1664525u + 1013904223u;
        if ((rng & 7u) == 0) {
            in_buf[i] = (uint8_t)(rng >> 24);
        } else {
            in_buf[i] = (uint8_t)dict[i % dlen];
        }
    }

    LARGE_INTEGER freq;
    QueryPerformanceFrequency(&freq);

    const int PASSES = 16;
    const int SAMPLES = 5;
    double times[5];
    uint64_t final_hash = 0;

    /* Warmup */
    size_t clen = lz_compress(in_buf, BUF_SIZE, comp_buf);
    size_t dsz = lz_decompress(comp_buf, clen, out_buf, BUF_SIZE);
    if (dsz != BUF_SIZE || memcmp(in_buf, out_buf, BUF_SIZE) != 0) {
        fprintf(stderr, "FAIL: lz roundtrip mismatch\n");
        return 2;
    }

    for (int s = 0; s < SAMPLES; ++s) {
        LARGE_INTEGER t0, t1;
        QueryPerformanceCounter(&t0);
        uint64_t h = 1469598103934665603ULL;
        for (int p = 0; p < PASSES; ++p) {
            size_t c = lz_compress(in_buf, BUF_SIZE, comp_buf);
            size_t d = lz_decompress(comp_buf, c, out_buf, BUF_SIZE);
            if (d != BUF_SIZE) return 3;
            h ^= (uint64_t)c + ((uint64_t)out_buf[p * 97] << 16);
            h *= 1099511628211ULL;
        }
        QueryPerformanceCounter(&t1);
        times[s] = (double)(t1.QuadPart - t0.QuadPart) * 1000.0 / (double)freq.QuadPart;
        final_hash = h;
    }

    if (memcmp(in_buf, out_buf, BUF_SIZE) != 0) return 4;
    qsort(times, SAMPLES, sizeof(double), cmp_double);
    double med = times[SAMPLES / 2];
    double mib = ((double)BUF_SIZE * (double)PASSES * 2.0) / (1024.0 * 1024.0);
    double tp = mib / (med / 1000.0);
    printf("BENCH_RESULT name=compression_loop median_ms=%.3f min_ms=%.3f max_ms=%.3f iters=%d unit=MiB/s throughput=%.3f checksum=%016llx\n",
           med, times[0], times[SAMPLES - 1], PASSES, tp, (unsigned long long)final_hash);
    fflush(stdout);
    return 0;
}
"##;

const SIMD_KERNEL_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <immintrin.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define MAT_N 128

static void gemm_avx2(const float *A, const float *B, float *C) {
    for (int i = 0; i < MAT_N; ++i) {
        for (int j = 0; j < MAT_N; j += 8) {
            __m256 acc = _mm256_setzero_ps();
            for (int k = 0; k < MAT_N; ++k) {
                __m256 va = _mm256_set1_ps(A[i * MAT_N + k]);
                __m256 vb = _mm256_loadu_ps(B + k * MAT_N + j);
                acc = _mm256_fmadd_ps(va, vb, acc);
            }
            _mm256_storeu_ps(C + i * MAT_N + j, acc);
        }
    }
}

static int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a, db = *(const double *)b;
    return (da > db) - (da < db);
}

int main(void) {
    float *A = (float *)VirtualAlloc(NULL, MAT_N * MAT_N * sizeof(float), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    float *B = (float *)VirtualAlloc(NULL, MAT_N * MAT_N * sizeof(float), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    float *C = (float *)VirtualAlloc(NULL, MAT_N * MAT_N * sizeof(float), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    if (!A || !B || !C) return 1;

    for (int i = 0; i < MAT_N * MAT_N; ++i) {
        A[i] = (float)((i % 17) - 8) * 0.125f;
        B[i] = (float)(((i * 3) % 13) - 6) * 0.125f;
    }

    LARGE_INTEGER freq;
    QueryPerformanceFrequency(&freq);

    const int REPS = 40;
    const int SAMPLES = 5;
    double times[5];

    /* Warmup + scalar reference check on C[0] and C[last] */
    gemm_avx2(A, B, C);
    float ref0 = 0.0f;
    for (int k = 0; k < MAT_N; ++k) ref0 += A[k] * B[k * MAT_N];
    float diff = C[0] - ref0;
    if (diff < -1e-3f || diff > 1e-3f) {
        fprintf(stderr, "FAIL: GEMM C[0]=%f expected %f\n", C[0], ref0);
        return 2;
    }

    double sum = 0.0;
    for (int s = 0; s < SAMPLES; ++s) {
        LARGE_INTEGER t0, t1;
        QueryPerformanceCounter(&t0);
        for (int r = 0; r < REPS; ++r) {
            gemm_avx2(A, B, C);
            A[0] = A[0] + 0.0f; /* prevent hoisting */
        }
        QueryPerformanceCounter(&t1);
        times[s] = (double)(t1.QuadPart - t0.QuadPart) * 1000.0 / (double)freq.QuadPart;
        sum = 0.0;
        for (int i = 0; i < MAT_N * MAT_N; ++i) sum += (double)C[i];
    }

    qsort(times, SAMPLES, sizeof(double), cmp_double);
    double med = times[SAMPLES / 2];
    double flops = 2.0 * (double)MAT_N * (double)MAT_N * (double)MAT_N * (double)REPS;
    double gflops = (flops / 1e9) / (med / 1000.0);
    int64_t check_i = (int64_t)(sum * 1000.0);
    printf("BENCH_RESULT name=simd_kernel median_ms=%.3f min_ms=%.3f max_ms=%.3f iters=%d unit=GFLOP/s throughput=%.3f checksum=%016llx\n",
           med, times[0], times[SAMPLES - 1], REPS, gflops, (unsigned long long)check_i);
    fflush(stdout);
    return 0;
}
"##;

const LOCK_HEAVY_JOB_QUEUE_C: &str = r##"
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define Q_CAP 256
#define Q_MASK (Q_CAP - 1)
#define NUM_WORKERS 4
#define JOBS_PER_SAMPLE 40000

typedef struct {
    SRWLOCK lock;
    CONDITION_VARIABLE not_empty;
    CONDITION_VARIABLE not_full;
    uint64_t ring[Q_CAP];
    uint32_t head;
    uint32_t tail;
    int shutdown;
    volatile LONG64 completed;
    volatile LONG64 checksum;
} JobQueue;

static DWORD WINAPI queue_worker(LPVOID lp) {
    JobQueue *q = (JobQueue *)lp;
    for (;;) {
        AcquireSRWLockExclusive(&q->lock);
        while (q->head == q->tail && !q->shutdown) {
            SleepConditionVariableSRW(&q->not_empty, &q->lock, INFINITE, 0);
        }
        if (q->head == q->tail && q->shutdown) {
            ReleaseSRWLockExclusive(&q->lock);
            return 0;
        }
        uint64_t item = q->ring[q->head & Q_MASK];
        q->head++;
        ReleaseSRWLockExclusive(&q->lock);
        WakeConditionVariable(&q->not_full);

        /* Tiny deterministic payload mix */
        uint64_t v = (item ^ 0x9E3779B97F4A7C15ULL) * 0xBF58476D1CE4E5B9ULL;
        InterlockedAdd64(&q->checksum, (LONG64)(v & 0xFFFFu));
        InterlockedIncrement64(&q->completed);
    }
}

static int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a, db = *(const double *)b;
    return (da > db) - (da < db);
}

static double run_one_sample(uint64_t *out_checksum) {
    JobQueue q;
    InitializeSRWLock(&q.lock);
    InitializeConditionVariable(&q.not_empty);
    InitializeConditionVariable(&q.not_full);
    q.head = 0;
    q.tail = 0;
    q.shutdown = 0;
    q.completed = 0;
    q.checksum = 0;

    HANDLE workers[NUM_WORKERS];
    for (int i = 0; i < NUM_WORKERS; ++i) {
        workers[i] = CreateThread(NULL, 0, queue_worker, &q, 0, NULL);
        if (!workers[i]) ExitProcess(1);
    }

    LARGE_INTEGER freq, t0, t1;
    QueryPerformanceFrequency(&freq);
    QueryPerformanceCounter(&t0);

    for (uint32_t i = 1; i <= JOBS_PER_SAMPLE; ++i) {
        AcquireSRWLockExclusive(&q.lock);
        while ((q.tail - q.head) >= Q_CAP) {
            SleepConditionVariableSRW(&q.not_full, &q.lock, INFINITE, 0);
        }
        q.ring[q.tail & Q_MASK] = (uint64_t)i;
        q.tail++;
        ReleaseSRWLockExclusive(&q.lock);
        WakeConditionVariable(&q.not_empty);
    }

    AcquireSRWLockExclusive(&q.lock);
    q.shutdown = 1;
    ReleaseSRWLockExclusive(&q.lock);
    WakeAllConditionVariable(&q.not_empty);

    WaitForMultipleObjects(NUM_WORKERS, workers, TRUE, 30000);
    QueryPerformanceCounter(&t1);

    for (int i = 0; i < NUM_WORKERS; ++i) CloseHandle(workers[i]);
    if (q.completed != JOBS_PER_SAMPLE) {
        fprintf(stderr, "FAIL: completed=%lld expected=%d\n", (long long)q.completed, JOBS_PER_SAMPLE);
        ExitProcess(2);
    }
    *out_checksum = (uint64_t)q.checksum;
    return (double)(t1.QuadPart - t0.QuadPart) * 1000.0 / (double)freq.QuadPart;
}

int main(void) {
    uint64_t expected_sum = 0;
    for (uint32_t i = 1; i <= JOBS_PER_SAMPLE; ++i) {
        uint64_t v = ((uint64_t)i ^ 0x9E3779B97F4A7C15ULL) * 0xBF58476D1CE4E5B9ULL;
        expected_sum += (v & 0xFFFFu);
    }

    /* Warmup */
    uint64_t cs = 0;
    run_one_sample(&cs);
    if (cs != expected_sum) {
        fprintf(stderr, "FAIL: checksum mismatch %llu != %llu\n", (unsigned long long)cs, (unsigned long long)expected_sum);
        return 3;
    }

    const int SAMPLES = 5;
    double times[5];
    for (int s = 0; s < SAMPLES; ++s) {
        times[s] = run_one_sample(&cs);
        if (cs != expected_sum) return 4;
    }

    qsort(times, SAMPLES, sizeof(double), cmp_double);
    double med = times[SAMPLES / 2];
    double kjobs_s = ((double)JOBS_PER_SAMPLE / 1000.0) / (med / 1000.0);
    printf("BENCH_RESULT name=lock_heavy_job_queue median_ms=%.3f min_ms=%.3f max_ms=%.3f iters=%d unit=kjobs/s throughput=%.3f checksum=%016llx\n",
           med, times[0], times[SAMPLES - 1], JOBS_PER_SAMPLE, kjobs_s, (unsigned long long)cs);
    fflush(stdout);
    return 0;
}
"##;

const D3D12_DRAW_HEAVY_C: &str = r##"
#define WIDL_C_INLINE_WRAPPERS
#define COBJMACROS
#define CINTERFACE
#define WIN32_LEAN_AND_MEAN
#include <windows.h>
#include <d3d12.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const GUID MY_IID_ID3D12Device =
    {0x189819f1, 0x1db6, 0x4b57, {0xbe, 0x54, 0x18, 0x21, 0x33, 0x9b, 0x85, 0xf7}};
static const GUID MY_IID_ID3D12CommandQueue =
    {0x0ec870a6, 0x5d7e, 0x4c22, {0x8c, 0xfc, 0x5b, 0xaa, 0xe0, 0x76, 0x16, 0xed}};
static const GUID MY_IID_ID3D12CommandAllocator =
    {0x6102dee4, 0xaf59, 0x4b09, {0xb9, 0x99, 0xb4, 0x4d, 0x73, 0xf0, 0x9b, 0x24}};
static const GUID MY_IID_ID3D12GraphicsCommandList =
    {0x5b160d0f, 0xac1b, 0x4185, {0x8b, 0xa8, 0xb3, 0xae, 0x42, 0xa5, 0xa4, 0x55}};
static const GUID MY_IID_ID3D12Fence =
    {0x0a753dcf, 0xc4d8, 0x4b91, {0xad, 0xf6, 0xbe, 0x5a, 0x60, 0xd9, 0x5a, 0x76}};
static const GUID MY_IID_ID3D12DescriptorHeap =
    {0x8efb471d, 0x616c, 0x4f49, {0x90, 0xf7, 0x12, 0x7b, 0xb7, 0x63, 0xfa, 0x51}};
static const GUID MY_IID_ID3D12Resource =
    {0x696442be, 0xa72e, 0x4059, {0xbc, 0x79, 0x5b, 0x5c, 0x98, 0x04, 0x0f, 0xad}};
static const GUID MY_IID_ID3D12RootSignature =
    {0xc54a6b66, 0x72df, 0x4ee8, {0x8b, 0xe5, 0xa9, 0x46, 0xa1, 0x42, 0x92, 0x14}};
static const GUID MY_IID_ID3D12PipelineState =
    {0x765a30f3, 0xf624, 0x4c6f, {0xa8, 0x28, 0xac, 0xe9, 0x48, 0x62, 0x24, 0x45}};

static const uint8_t kVsPassthroughDxbc[216] = {
    0x44, 0x58, 0x42, 0x43, 0x9d, 0x3b, 0x28, 0xb4, 0xb7, 0xe9, 0x69, 0xf4,
    0x0e, 0x49, 0x0a, 0xe5, 0xae, 0x5e, 0xa2, 0x25, 0x01, 0x00, 0x00, 0x00,
    0xd8, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x60, 0x00, 0x00, 0x00, 0x94, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x0f, 0x00, 0x00,
    0x50, 0x4f, 0x53, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x00, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x00, 0x53, 0x48, 0x45, 0x58, 0x3c, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x01, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x5f, 0x00, 0x00, 0x03,
    0xf2, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x67, 0x00, 0x00, 0x04,
    0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x36, 0x00, 0x00, 0x05, 0xf2, 0x20, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x46, 0x1e, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x01,
};

static const uint8_t kPsCb0ColorDxbc[188] = {
    0x44, 0x58, 0x42, 0x43, 0xf4, 0x68, 0x60, 0xac, 0x53, 0x58, 0x0a, 0x02,
    0xf3, 0x2f, 0xb6, 0x13, 0xd5, 0x77, 0x32, 0x24, 0x01, 0x00, 0x00, 0x00,
    0xbc, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00,
    0x3c, 0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x49, 0x53, 0x47, 0x4e,
    0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x4f, 0x53, 0x47, 0x4e, 0x2c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x08, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0x00, 0x00, 0x00, 0x53, 0x56, 0x5f, 0x54, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x00, 0x00, 0x00, 0x53, 0x48, 0x45, 0x58, 0x44, 0x00, 0x00, 0x00,
    0x50, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x6a, 0x08, 0x00, 0x01,
    0x59, 0x00, 0x00, 0x04, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x65, 0x00, 0x00, 0x03, 0xf2, 0x20, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x06, 0xf2, 0x20, 0x10, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x46, 0x8e, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x01,
};

#define CHECK_HR(hr) do { if (FAILED(hr)) { fprintf(stderr, "FAIL hr=0x%08lx line %d\n", (unsigned long)(hr), __LINE__); ExitProcess(1); } } while (0)
#define DRAWS_PER_FRAME 500
#define CB_SLOTS 16

static ID3D12Resource *create_buf(ID3D12Device *dev, D3D12_HEAP_TYPE ht, UINT64 sz, D3D12_RESOURCE_STATES st) {
    D3D12_HEAP_PROPERTIES hp;
    memset(&hp, 0, sizeof(hp));
    hp.Type = ht;
    D3D12_RESOURCE_DESC rd;
    memset(&rd, 0, sizeof(rd));
    rd.Dimension = D3D12_RESOURCE_DIMENSION_BUFFER;
    rd.Width = sz;
    rd.Height = 1;
    rd.DepthOrArraySize = 1;
    rd.MipLevels = 1;
    rd.SampleDesc.Count = 1;
    rd.Layout = D3D12_TEXTURE_LAYOUT_ROW_MAJOR;
    ID3D12Resource *r = NULL;
    CHECK_HR(ID3D12Device_CreateCommittedResource(dev, &hp, D3D12_HEAP_FLAG_NONE, &rd, st, NULL, &MY_IID_ID3D12Resource, (void **)&r));
    return r;
}

static int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a, db = *(const double *)b;
    return (da > db) - (da < db);
}

int main(void) {
    ID3D12Device *dev = NULL;
    CHECK_HR(D3D12CreateDevice(NULL, D3D_FEATURE_LEVEL_11_0, &MY_IID_ID3D12Device, (void **)&dev));

    D3D12_COMMAND_QUEUE_DESC qd;
    memset(&qd, 0, sizeof(qd));
    qd.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
    ID3D12CommandQueue *queue = NULL;
    CHECK_HR(ID3D12Device_CreateCommandQueue(dev, &qd, &MY_IID_ID3D12CommandQueue, (void **)&queue));

    ID3D12CommandAllocator *alloc = NULL;
    CHECK_HR(ID3D12Device_CreateCommandAllocator(dev, D3D12_COMMAND_LIST_TYPE_DIRECT, &MY_IID_ID3D12CommandAllocator, (void **)&alloc));

    ID3D12GraphicsCommandList *cl = NULL;
    CHECK_HR(ID3D12Device_CreateCommandList(dev, 0, D3D12_COMMAND_LIST_TYPE_DIRECT, alloc, NULL, &MY_IID_ID3D12GraphicsCommandList, (void **)&cl));
    CHECK_HR(ID3D12GraphicsCommandList_Close(cl));

    ID3D12Fence *fence = NULL;
    CHECK_HR(ID3D12Device_CreateFence(dev, 0, D3D12_FENCE_FLAG_NONE, &MY_IID_ID3D12Fence, (void **)&fence));
    HANDLE fev = CreateEventW(NULL, FALSE, FALSE, NULL);

    const UINT W = 256, H = 256;
    D3D12_HEAP_PROPERTIES dhp;
    memset(&dhp, 0, sizeof(dhp));
    dhp.Type = D3D12_HEAP_TYPE_DEFAULT;
    D3D12_RESOURCE_DESC rtd;
    memset(&rtd, 0, sizeof(rtd));
    rtd.Dimension = D3D12_RESOURCE_DIMENSION_TEXTURE2D;
    rtd.Width = W;
    rtd.Height = H;
    rtd.DepthOrArraySize = 1;
    rtd.MipLevels = 1;
    rtd.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    rtd.SampleDesc.Count = 1;
    rtd.Flags = D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET;
    ID3D12Resource *rt = NULL;
    CHECK_HR(ID3D12Device_CreateCommittedResource(dev, &dhp, D3D12_HEAP_FLAG_NONE, &rtd, D3D12_RESOURCE_STATE_RENDER_TARGET, NULL, &MY_IID_ID3D12Resource, (void **)&rt));

    D3D12_DESCRIPTOR_HEAP_DESC rtv_hd;
    memset(&rtv_hd, 0, sizeof(rtv_hd));
    rtv_hd.Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
    rtv_hd.NumDescriptors = 1;
    ID3D12DescriptorHeap *rtv_heap = NULL;
    CHECK_HR(ID3D12Device_CreateDescriptorHeap(dev, &rtv_hd, &MY_IID_ID3D12DescriptorHeap, (void **)&rtv_heap));
    D3D12_CPU_DESCRIPTOR_HANDLE rtv_cpu = ID3D12DescriptorHeap_GetCPUDescriptorHandleForHeapStart(rtv_heap);
    ID3D12Device_CreateRenderTargetView(dev, rt, NULL, rtv_cpu);

    D3D12_ROOT_PARAMETER rp;
    memset(&rp, 0, sizeof(rp));
    rp.ParameterType = D3D12_ROOT_PARAMETER_TYPE_CBV;
    rp.ShaderVisibility = D3D12_SHADER_VISIBILITY_ALL;
    D3D12_ROOT_SIGNATURE_DESC rsd;
    memset(&rsd, 0, sizeof(rsd));
    rsd.NumParameters = 1;
    rsd.pParameters = &rp;
    rsd.Flags = D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;
    ID3DBlob *blob = NULL, *errb = NULL;
    CHECK_HR(D3D12SerializeRootSignature(&rsd, D3D_ROOT_SIGNATURE_VERSION_1, &blob, &errb));
    ID3D12RootSignature *rs = NULL;
    CHECK_HR(ID3D12Device_CreateRootSignature(dev, 0, ID3D10Blob_GetBufferPointer(blob), ID3D10Blob_GetBufferSize(blob), &MY_IID_ID3D12RootSignature, (void **)&rs));
    ID3D10Blob_Release(blob);

    D3D12_INPUT_ELEMENT_DESC ie;
    memset(&ie, 0, sizeof(ie));
    ie.SemanticName = "POSITION";
    ie.Format = DXGI_FORMAT_R32G32B32A32_FLOAT;
    D3D12_GRAPHICS_PIPELINE_STATE_DESC pd;
    memset(&pd, 0, sizeof(pd));
    pd.pRootSignature = rs;
    pd.VS.pShaderBytecode = kVsPassthroughDxbc;
    pd.VS.BytecodeLength = sizeof(kVsPassthroughDxbc);
    pd.PS.pShaderBytecode = kPsCb0ColorDxbc;
    pd.PS.BytecodeLength = sizeof(kPsCb0ColorDxbc);
    pd.BlendState.RenderTarget[0].RenderTargetWriteMask = D3D12_COLOR_WRITE_ENABLE_ALL;
    pd.SampleMask = 0xFFFFFFFFu;
    pd.RasterizerState.FillMode = D3D12_FILL_MODE_SOLID;
    pd.RasterizerState.CullMode = D3D12_CULL_MODE_NONE;
    pd.InputLayout.pInputElementDescs = &ie;
    pd.InputLayout.NumElements = 1;
    pd.PrimitiveTopologyType = D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE;
    pd.NumRenderTargets = 1;
    pd.RTVFormats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;
    pd.SampleDesc.Count = 1;
    ID3D12PipelineState *pso = NULL;
    CHECK_HR(ID3D12Device_CreateGraphicsPipelineState(dev, &pd, &MY_IID_ID3D12PipelineState, (void **)&pso));

    const float verts[12] = { -0.8f, -0.8f, 0.0f, 1.0f, 0.0f, 0.8f, 0.0f, 1.0f, 0.8f, -0.8f, 0.0f, 1.0f };
    ID3D12Resource *vb = create_buf(dev, D3D12_HEAP_TYPE_UPLOAD, sizeof(verts), D3D12_RESOURCE_STATE_GENERIC_READ);
    void *mp = NULL;
    CHECK_HR(ID3D12Resource_Map(vb, 0, NULL, &mp));
    memcpy(mp, verts, sizeof(verts));
    ID3D12Resource_Unmap(vb, 0, NULL);

    ID3D12Resource *cb = create_buf(dev, D3D12_HEAP_TYPE_UPLOAD, CB_SLOTS * 256, D3D12_RESOURCE_STATE_GENERIC_READ);
    CHECK_HR(ID3D12Resource_Map(cb, 0, NULL, &mp));
    for (int i = 0; i < CB_SLOTS; ++i) {
        float *f = (float *)((uint8_t *)mp + i * 256);
        f[0] = (float)(i + 1) / (float)CB_SLOTS;
        f[1] = 1.0f - (float)i / (float)CB_SLOTS;
        f[2] = 0.5f;
        f[3] = 1.0f;
    }
    ID3D12Resource_Unmap(cb, 0, NULL);

    D3D12_GPU_VIRTUAL_ADDRESS cb_base = ID3D12Resource_GetGPUVirtualAddress(cb);
    D3D12_VERTEX_BUFFER_VIEW vbv = { ID3D12Resource_GetGPUVirtualAddress(vb), sizeof(verts), 16 };
    D3D12_VIEWPORT vp = { 0.0f, 0.0f, (float)W, (float)H, 0.0f, 1.0f };
    const float clear_col[4] = { 0.0f, 0.0f, 0.0f, 1.0f };

    LARGE_INTEGER freq;
    QueryPerformanceFrequency(&freq);
    UINT64 fence_val = 0;

    const int WARMUP = 2;
    const int SAMPLES = 7;
    double times[7];

    for (int iter = 0; iter < WARMUP + SAMPLES; ++iter) {
        LARGE_INTEGER t0, t1;
        QueryPerformanceCounter(&t0);

        CHECK_HR(ID3D12CommandAllocator_Reset(alloc));
        CHECK_HR(ID3D12GraphicsCommandList_Reset(cl, alloc, pso));
        ID3D12GraphicsCommandList_ClearRenderTargetView(cl, rtv_cpu, clear_col, 0, NULL);
        ID3D12GraphicsCommandList_RSSetViewports(cl, 1, &vp);
        ID3D12GraphicsCommandList_OMSetRenderTargets(cl, 1, &rtv_cpu, FALSE, NULL);
        ID3D12GraphicsCommandList_SetGraphicsRootSignature(cl, rs);
        ID3D12GraphicsCommandList_IASetPrimitiveTopology(cl, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        ID3D12GraphicsCommandList_IASetVertexBuffers(cl, 0, 1, &vbv);

        for (int d = 0; d < DRAWS_PER_FRAME; ++d) {
            D3D12_RECT sc = { (d & 3) * 4, (d & 3) * 4, (LONG)W, (LONG)H };
            ID3D12GraphicsCommandList_RSSetScissorRects(cl, 1, &sc);
            ID3D12GraphicsCommandList_SetGraphicsRootConstantBufferView(
                cl, 0, cb_base + (UINT64)(d % CB_SLOTS) * 256);
            ID3D12GraphicsCommandList_DrawInstanced(cl, 3, 1, 0, 0);
        }

        CHECK_HR(ID3D12GraphicsCommandList_Close(cl));
        ID3D12CommandList *lists[1] = { (ID3D12CommandList *)cl };
        ID3D12CommandQueue_ExecuteCommandLists(queue, 1, lists);
        fence_val++;
        CHECK_HR(ID3D12CommandQueue_Signal(queue, fence, fence_val));
        CHECK_HR(ID3D12Fence_SetEventOnCompletion(fence, fence_val, fev));
        if (WaitForSingleObject(fev, 10000) != WAIT_OBJECT_0) return 2;

        QueryPerformanceCounter(&t1);
        if (iter >= WARMUP) {
            times[iter - WARMUP] = (double)(t1.QuadPart - t0.QuadPart) * 1000.0 / (double)freq.QuadPart;
        }
    }

    /* Verify non-black rendered center pixel via readback */
    D3D12_PLACED_SUBRESOURCE_FOOTPRINT fp;
    UINT64 total_bytes = 0;
    ID3D12Device_GetCopyableFootprints(dev, &rtd, 0, 1, 0, &fp, NULL, NULL, &total_bytes);
    ID3D12Resource *rb = create_buf(dev, D3D12_HEAP_TYPE_READBACK, total_bytes, D3D12_RESOURCE_STATE_COPY_DEST);

    CHECK_HR(ID3D12CommandAllocator_Reset(alloc));
    CHECK_HR(ID3D12GraphicsCommandList_Reset(cl, alloc, NULL));
    D3D12_RESOURCE_BARRIER bar;
    memset(&bar, 0, sizeof(bar));
    bar.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
    bar.Transition.pResource = rt;
    bar.Transition.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
    bar.Transition.StateAfter = D3D12_RESOURCE_STATE_COPY_SOURCE;
    bar.Transition.Subresource = D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES;
    ID3D12GraphicsCommandList_ResourceBarrier(cl, 1, &bar);

    D3D12_TEXTURE_COPY_LOCATION dst_loc, src_loc;
    memset(&dst_loc, 0, sizeof(dst_loc));
    memset(&src_loc, 0, sizeof(src_loc));
    dst_loc.pResource = rb;
    dst_loc.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
    dst_loc.PlacedFootprint = fp;
    src_loc.pResource = rt;
    src_loc.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
    ID3D12GraphicsCommandList_CopyTextureRegion(cl, &dst_loc, 0, 0, 0, &src_loc, NULL);
    CHECK_HR(ID3D12GraphicsCommandList_Close(cl));
    ID3D12CommandList *lists[1] = { (ID3D12CommandList *)cl };
    ID3D12CommandQueue_ExecuteCommandLists(queue, 1, lists);
    fence_val++;
    CHECK_HR(ID3D12CommandQueue_Signal(queue, fence, fence_val));
    CHECK_HR(ID3D12Fence_SetEventOnCompletion(fence, fence_val, fev));
    WaitForSingleObject(fev, 10000);

    const uint8_t *px = NULL;
    CHECK_HR(ID3D12Resource_Map(rb, 0, NULL, (void **)&px));
    const uint8_t *mid = px + (size_t)128 * fp.Footprint.RowPitch + 128 * 4;
    uint32_t rgba = ((uint32_t)mid[0] << 24) | ((uint32_t)mid[1] << 16) | ((uint32_t)mid[2] << 8) | mid[3];
    if (mid[3] != 255 || (mid[0] == 0 && mid[1] == 0 && mid[2] == 0)) {
        fprintf(stderr, "FAIL: center pixel unexpected (%u,%u,%u,%u)\n", mid[0], mid[1], mid[2], mid[3]);
        return 3;
    }
    D3D12_RANGE er = { 0, 0 };
    ID3D12Resource_Unmap(rb, 0, &er);

    qsort(times, SAMPLES, sizeof(double), cmp_double);
    double med = times[SAMPLES / 2];
    double draws_s = (double)DRAWS_PER_FRAME / (med / 1000.0);
    printf("BENCH_RESULT name=d3d12_draw_heavy_frame median_ms=%.3f min_ms=%.3f max_ms=%.3f iters=%d unit=draws/s throughput=%.3f checksum=%08x\n",
           med, times[0], times[SAMPLES - 1], DRAWS_PER_FRAME, draws_s, rgba);
    fflush(stdout);

    CloseHandle(fev);
    ID3D12Resource_Release(rb);
    ID3D12Resource_Release(cb);
    ID3D12Resource_Release(vb);
    ID3D12PipelineState_Release(pso);
    ID3D12RootSignature_Release(rs);
    ID3D12DescriptorHeap_Release(rtv_heap);
    ID3D12Resource_Release(rt);
    ID3D12Fence_Release(fence);
    ID3D12GraphicsCommandList_Release(cl);
    ID3D12CommandAllocator_Release(alloc);
    ID3D12CommandQueue_Release(queue);
    ID3D12Device_Release(dev);
    return 0;
}
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bench_result_line_extracts_all_fields() {
        let line = "BENCH_RESULT name=simd_kernel median_ms=12.345 min_ms=11.900 max_ms=13.100 iters=40 unit=GFLOP/s throughput=13.567 checksum=00001234abcd5678";
        let m = parse_bench_result_line(line, BenchMode::Aot, 45.0).unwrap();
        assert_eq!(m.id, "simd_kernel");
        assert_eq!(m.mode, BenchMode::Aot);
        assert!((m.median_ms - 12.345).abs() < 1e-6);
        assert!((m.min_ms - 11.9).abs() < 1e-6);
        assert!((m.max_ms - 13.1).abs() < 1e-6);
        assert_eq!(m.iters, 40);
        assert_eq!(m.unit, "GFLOP/s");
        assert!((m.throughput - 13.567).abs() < 1e-6);
        assert_eq!(m.checksum, "00001234abcd5678");
    }

    #[test]
    fn baseline_db_json_roundtrip_preserves_chip_keys() {
        let mut db = BaselineDb::default();
        db.chips
            .entry("Apple M3 Ultra".into())
            .or_default()
            .entry("aot".into())
            .or_default()
            .insert("compression_loop".into(), 42.125);
        db.chips
            .entry("Apple M3 Ultra".into())
            .or_default()
            .entry("aot".into())
            .or_default()
            .insert("simd_kernel".into(), 18.5);
        let json = format_baseline_db_json(&db);
        let parsed = parse_baseline_db_json(&json).unwrap();
        assert_eq!(parsed, db);
    }

    #[test]
    fn compare_enforces_tolerance_and_missing_chip_baseline() {
        let mut db = BaselineDb::default();
        db.chips
            .entry("Apple M3 Ultra".into())
            .or_default()
            .entry("aot".into())
            .or_default()
            .insert("compression_loop".into(), 100.0);

        let fast = WorkloadMeasurement {
            id: "compression_loop".into(),
            mode: BenchMode::Aot,
            median_ms: 120.0,
            min_ms: 118.0,
            max_ms: 125.0,
            host_wall_ms: 200.0,
            iters: 16,
            unit: "MiB/s".into(),
            throughput: 66.0,
            checksum: "abc".into(),
        };
        let (cmp_ok, pass_ok) =
            compare_against_baseline(&[fast.clone()], &db, "Apple M3 Ultra", BenchMode::Aot, 0.25, false);
        assert!(pass_ok);
        assert_eq!(cmp_ok[0].status, CompareStatus::Pass);

        let mut slow = fast.clone();
        slow.median_ms = 130.0;
        let (cmp_fail, pass_fail) =
            compare_against_baseline(&[slow], &db, "Apple M3 Ultra", BenchMode::Aot, 0.25, false);
        assert!(!pass_fail);
        assert_eq!(cmp_fail[0].status, CompareStatus::Fail);

        let (cmp_missing, pass_missing) =
            compare_against_baseline(&[fast], &db, "Apple M9 Unknown", BenchMode::Aot, 0.25, false);
        assert!(!pass_missing);
        assert_eq!(cmp_missing[0].status, CompareStatus::NoBaseline);
    }
}
