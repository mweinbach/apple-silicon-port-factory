//! Interactive Local Web Launcher & macOS `.app` Manager (`port-factory launcher`).
//!
//! Zero extra crate dependencies: serves a single-page dark-mode management UI bound
//! strictly to `127.0.0.1` with a per-session CSRF token (`X-PF-Token`), allowing you to:
//! - Install/verify native Apple Silicon `SteamRE/DepotDownloader` in one click
//! - Browse installed games & your owned Steam library (or enter any Steam AppID)
//! - Download, update, or repair Windows x64 Steam depots (via Steam Mobile App QR code
//!   or an interactive macOS Terminal session for password/2FA)
//! - Convert any game into a self-contained SSD-portable `.app` bundle
//! - Run `bundle-verify` inline, launch `.app` bundles, reveal in Finder, and export to
//!   any mounted external SSD under `/Volumes/`

use crate::bundle_common::{codesign_ad_hoc, set_executable_mode};
use crate::steam_library::{
    default_apps_dir, find_depot_downloader, generate_info_plist, list_external_volumes,
    list_packaged_bundles, sanitize_game_title, scan_all_steam_libraries, verify_app_bundle,
    BundleCheckStatus,
};
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Configuration for the local interactive launcher server.
#[derive(Debug, Clone)]
pub struct LauncherOptions {
    pub workspace_root: PathBuf,
    pub port: u16,
    pub open_browser: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JobStatus {
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl JobStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

struct JobEntry {
    id: u64,
    title: String,
    command_preview: String,
    status: JobStatus,
    exit_code: Option<i32>,
    logs: String,
    child_pid: Option<u32>,
    started_unix: u64,
}

struct SharedState {
    workspace_root: PathBuf,
    self_exe: PathBuf,
    token: String,
    next_job_id: AtomicU64,
    jobs: Mutex<Vec<JobEntry>>,
    shutdown: AtomicBool,
}

/// Handle to a running background launcher HTTP server (used by both CLI and tests).
pub struct RunningLauncher {
    pub addr: SocketAddr,
    pub url: String,
    pub token: String,
    state: Arc<SharedState>,
    handle: Option<thread::JoinHandle<()>>,
}

impl RunningLauncher {
    pub fn shutdown(&mut self) {
        self.state.shutdown.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.addr);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

impl Drop for RunningLauncher {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn generate_session_token() -> String {
    let mut buf = [0u8; 16];
    if let Ok(mut f) = fs::File::open("/dev/urandom") {
        if f.read_exact(&mut buf).is_ok() {
            let mut s = String::with_capacity(32);
            for b in buf {
                use std::fmt::Write as _;
                let _ = write!(s, "{b:02x}");
            }
            return s;
        }
    }
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{:032x}", nanos ^ (std::process::id() as u128).rotate_left(48))
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Start the local `127.0.0.1` launcher HTTP server on a background thread.
pub fn start_launcher_server(opts: LauncherOptions) -> Result<RunningLauncher> {
    let listener = TcpListener::bind(("127.0.0.1", opts.port))
        .with_context(|| format!("binding launcher server on 127.0.0.1:{}", opts.port))?;
    let addr = listener.local_addr()?;
    listener
        .set_nonblocking(false)
        .context("configuring launcher TcpListener")?;

    let self_exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("port-factory"));
    let token = generate_session_token();
    let url = format!("http://127.0.0.1:{}/", addr.port());

    let state = Arc::new(SharedState {
        workspace_root: opts.workspace_root,
        self_exe,
        token: token.clone(),
        next_job_id: AtomicU64::new(1),
        jobs: Mutex::new(Vec::new()),
        shutdown: AtomicBool::new(false),
    });

    let thread_state = Arc::clone(&state);
    let handle = thread::spawn(move || {
        for incoming in listener.incoming() {
            if thread_state.shutdown.load(Ordering::SeqCst) {
                break;
            }
            let Ok(stream) = incoming else {
                continue;
            };
            let st = Arc::clone(&thread_state);
            thread::spawn(move || {
                let _ = handle_connection(stream, &st);
            });
        }
    });

    Ok(RunningLauncher {
        addr,
        url,
        token,
        state,
        handle: Some(handle),
    })
}

/// Run the interactive Port Factory Launcher in the foreground (opens browser unless `--no-open`).
pub fn run_launcher(opts: LauncherOptions) -> Result<()> {
    let open_browser = opts.open_browser;
    let launcher = start_launcher_server(opts)?;

    // Record live URL so double-clicking `Port Factory.app` again reuses the running server.
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        let cache_dir = home.join("Library/Caches/port-factory");
        let _ = fs::create_dir_all(&cache_dir);
        let _ = fs::write(cache_dir.join("launcher.url"), &launcher.url);
    }

    println!("================================================================================");
    println!("  Port Factory Launcher running at {}", launcher.url);
    println!("  Workspace : {}", launcher.state.workspace_root.display());
    println!("  Press Ctrl-C to stop.");
    println!("================================================================================");

    if open_browser {
        #[cfg(target_os = "macos")]
        {
            let _ = Command::new("/usr/bin/open").arg(&launcher.url).status();
        }
    }

    loop {
        if launcher.state.shutdown.load(Ordering::SeqCst) {
            break;
        }
        thread::sleep(Duration::from_millis(250));
    }
    Ok(())
}

/// Package a double-clickable `Port Factory.app` bundle that launches this interactive UI.
pub fn build_launcher_app_bundle(workspace_root: &Path, out_app: &Path) -> Result<PathBuf> {
    let build = Command::new("cargo")
        .current_dir(workspace_root)
        .args([
            "build",
            "--release",
            "--target",
            "aarch64-apple-darwin",
            "-p",
            "port-factory",
            "--bin",
            "port-factory",
        ])
        .status()
        .context("building release port-factory binary")?;
    if !build.success() {
        bail!("cargo build --release -p port-factory failed (status {build})");
    }

    let pf_bin = [
        workspace_root.join("target/aarch64-apple-darwin/release/port-factory"),
        workspace_root.join("target/release/port-factory"),
    ]
    .into_iter()
    .find(|p| p.is_file())
    .context("built port-factory release binary not found")?;

    let contents = out_app.join("Contents");
    let macos = contents.join("MacOS");
    let resources = contents.join("Resources");
    let _ = fs::remove_dir_all(contents.join("_CodeSignature"));
    let _ = fs::remove_dir_all(&macos);
    fs::create_dir_all(&macos)?;
    fs::create_dir_all(&resources)?;

    fs::write(
        contents.join("Info.plist"),
        generate_info_plist("Port Factory", "PortFactoryLauncher", "1"),
    )?;
    fs::copy(&pf_bin, macos.join("port-factory"))?;

    // Compile a lightweight native ARM64 launcher that reuses an already-running local
    // launcher server or starts `port-factory launcher` cleanly in the background.
    let ws_canon = fs::canonicalize(workspace_root).unwrap_or_else(|_| workspace_root.to_path_buf());
    let launcher_rs = resources.join("launcher_main.rs");
    let launcher_src = format!(
        r#"fn main() {{
    use std::io::{{Read, Write}};
    use std::net::TcpStream;
    use std::path::PathBuf;
    use std::process::{{Command, Stdio}};
    use std::time::Duration;

    let current_exe = std::env::current_exe().expect("current_exe");
    let macos_dir = current_exe.parent().expect("Contents/MacOS");
    let pf_bin = macos_dir.join("port-factory");
    let ws = std::env::var("PORT_FACTORY_WORKSPACE").unwrap_or_else(|_| {ws_str:?}.to_string());

    let port: u16 = std::env::var("PORT_FACTORY_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(7420);
    let url = format!("http://127.0.0.1:{{port}}/");

    let is_alive = |p: u16| -> bool {{
        let Ok(mut s) = TcpStream::connect_timeout(
            &std::net::SocketAddr::from(([127, 0, 0, 1], p)),
            Duration::from_millis(200),
        ) else {{
            return false;
        }};
        let _ = s.set_read_timeout(Some(Duration::from_millis(300)));
        if s.write_all(b"GET /api/ping HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n").is_err() {{
            return false;
        }}
        let mut resp = String::new();
        let _ = s.read_to_string(&mut resp);
        resp.contains("port-factory-ok")
    }};

    if is_alive(port) {{
        let _ = Command::new("/usr/bin/open").arg(&url).status();
        return;
    }}

    let log_dir = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("Library/Logs/PortFactory");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_file = std::fs::File::create(log_dir.join("launcher.log")).ok();

    let mut cmd = Command::new(&pf_bin);
    cmd.args(["launcher", "--workspace", &ws, "--port", &port.to_string()]);
    if let Some(f) = log_file {{
        if let Ok(f2) = f.try_clone() {{
            cmd.stdout(Stdio::from(f)).stderr(Stdio::from(f2));
        }}
    }}
    let _ = cmd.spawn();
}}
"#,
        ws_str = ws_canon.to_string_lossy()
    );
    fs::write(&launcher_rs, launcher_src)?;

    let launcher_bin = macos.join("PortFactoryLauncher");
    let compile = Command::new("rustc")
        .args([
            "--edition=2021",
            "-C",
            "opt-level=2",
            &launcher_rs.to_string_lossy(),
            "-o",
            &launcher_bin.to_string_lossy(),
        ])
        .status()
        .context("compiling PortFactoryLauncher executable")?;
    if !compile.success() {
        bail!("rustc failed compiling {}", launcher_bin.display());
    }

    for bin in [&launcher_bin, &macos.join("port-factory")] {
        let _ = set_executable_mode(bin);
    }

    #[cfg(target_os = "macos")]
    {
        // Sign nested helper binaries first, then sign the `.app` bundle once (signing the
        // bundle signs `CFBundleExecutable` and seals `CodeResources` in a single pass).
        codesign_ad_hoc(&macos.join("port-factory"));
        codesign_ad_hoc(out_app);
    }

    Ok(out_app.to_path_buf())
}

// ============================================================================
// Minimal HTTP/1.1 Router & JSON Helpers (Zero Extra Crate Dependencies)
// ============================================================================

struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

fn read_http_request(stream: &mut TcpStream) -> Result<HttpRequest> {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        bail!("empty connection");
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_string();
    let raw_target = parts.next().unwrap_or("/");
    let path = raw_target.split('?').next().unwrap_or("/").to_string();

    let mut headers = HashMap::new();
    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        if let Some((k, v)) = trimmed.split_once(':') {
            let key = k.trim().to_ascii_lowercase();
            let val = v.trim().to_string();
            if key == "content-length" {
                content_length = val.parse().unwrap_or(0);
            }
            headers.insert(key, val);
        }
    }

    let capped_len = content_length.min(256 * 1024);
    let mut body_bytes = vec![0u8; capped_len];
    if capped_len > 0 {
        reader.read_exact(&mut body_bytes)?;
    }
    let body = String::from_utf8_lossy(&body_bytes).into_owned();
    Ok(HttpRequest {
        method,
        path,
        headers,
        body,
    })
}

fn write_http_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &str,
) -> Result<()> {
    let resp = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(resp.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn handle_connection(mut stream: TcpStream, state: &Arc<SharedState>) -> Result<()> {
    let req = read_http_request(&mut stream)?;

    if req.method == "GET" && req.path == "/api/ping" {
        return write_http_response(
            &mut stream,
            "200 OK",
            "application/json; charset=utf-8",
            "{\"ok\":true,\"service\":\"port-factory-ok\"}",
        );
    }

    if req.method == "GET" && (req.path == "/" || req.path == "/index.html") {
        let html = LAUNCHER_HTML_TEMPLATE.replace("__PF_TOKEN__", &state.token);
        return write_http_response(&mut stream, "200 OK", "text/html; charset=utf-8", &html);
    }

    if req.method == "GET" && req.path == "/api/state" {
        let payload = build_state_json(state);
        return write_http_response(
            &mut stream,
            "200 OK",
            "application/json; charset=utf-8",
            &payload,
        );
    }

    if req.method == "GET" && req.path.starts_with("/api/jobs/") {
        let id_str = req.path.trim_start_matches("/api/jobs/");
        if let Ok(id) = id_str.parse::<u64>() {
            let jobs = state.jobs.lock().unwrap();
            if let Some(job) = jobs.iter().find(|j| j.id == id) {
                let json = format_job_json(job, true);
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    &json,
                );
            }
        }
        return write_http_response(
            &mut stream,
            "404 Not Found",
            "application/json; charset=utf-8",
            "{\"error\":\"job not found\"}",
        );
    }

    // All mutating POST endpoints require the per-session CSRF token.
    if req.method == "POST" {
        let got_token = req
            .headers
            .get("x-pf-token")
            .map(String::as_str)
            .unwrap_or("");
        if got_token != state.token {
            return write_http_response(
                &mut stream,
                "403 Forbidden",
                "application/json; charset=utf-8",
                "{\"error\":\"missing or invalid session token\"}",
            );
        }

        let fields = parse_flat_json(&req.body);
        match req.path.as_str() {
            "/api/setup-steam" => {
                let id = spawn_pf_job(
                    state,
                    "Install Native ARM64 DepotDownloader",
                    vec!["steam-setup".to_string()],
                );
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    &format!("{{\"ok\":true,\"job_id\":{id}}}"),
                );
            }
            "/api/download" => {
                let target = fields.get("target").cloned().unwrap_or_default();
                if target.trim().is_empty() {
                    return write_http_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        "{\"error\":\"missing target AppID or game name\"}",
                    );
                }
                let use_qr = json_bool(&fields, "qr", true);
                let validate = json_bool(&fields, "validate", false);
                let terminal = json_bool(&fields, "terminal", false);
                let username = fields.get("username").cloned().unwrap_or_default();
                let dir = fields.get("dir").cloned().unwrap_or_default();

                let mut args = vec!["steam-download".to_string(), target.clone()];
                if !dir.trim().is_empty() {
                    args.push("--dir".to_string());
                    args.push(dir);
                }
                if use_qr {
                    args.push("--qr".to_string());
                } else if !username.trim().is_empty() {
                    args.push("--user".to_string());
                    args.push(username);
                }
                if validate {
                    args.push("--validate".to_string());
                }

                if terminal {
                    match launch_in_macos_terminal(&state.self_exe, &state.workspace_root, &args) {
                        Ok(msg) => {
                            return write_http_response(
                                &mut stream,
                                "200 OK",
                                "application/json; charset=utf-8",
                                &format!("{{\"ok\":true,\"terminal\":true,\"message\":{}}}", json_str(&msg)),
                            );
                        }
                        Err(e) => {
                            return write_http_response(
                                &mut stream,
                                "500 Internal Server Error",
                                "application/json; charset=utf-8",
                                &format!("{{\"error\":{}}}", json_str(&e.to_string())),
                            );
                        }
                    }
                }

                let label = if validate {
                    format!("Verify / Update Steam Game: {target}")
                } else {
                    format!("Download Steam Game: {target}")
                };
                let id = spawn_pf_job(state, &label, args);
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    &format!("{{\"ok\":true,\"job_id\":{id}}}"),
                );
            }
            "/api/convert" => {
                let target = fields.get("target").cloned().unwrap_or_default();
                let title = fields.get("title").cloned().unwrap_or_else(|| target.clone());
                if target.trim().is_empty() {
                    return write_http_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        "{\"error\":\"missing target\"}",
                    );
                }
                let out_app = fields
                    .get("out")
                    .filter(|s| !s.trim().is_empty())
                    .cloned()
                    .unwrap_or_else(|| {
                        let clean = sanitize_game_title(&title);
                        default_apps_dir(&state.workspace_root)
                            .join(format!("{clean}.app"))
                            .to_string_lossy()
                            .into_owned()
                    });
                let args = vec![
                    "steam-convert".to_string(),
                    target,
                    "--format".to_string(),
                    "app".to_string(),
                    "--copy-game".to_string(),
                    "--max-fns".to_string(),
                    "0".to_string(),
                    "--out".to_string(),
                    out_app,
                ];
                let id = spawn_pf_job(state, &format!("Convert to .app: {title}"), args);
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    &format!("{{\"ok\":true,\"job_id\":{id}}}"),
                );
            }
            "/api/port" => {
                let target = fields.get("target").cloned().unwrap_or_default();
                if target.trim().is_empty() {
                    return write_http_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        "{\"error\":\"missing target\"}",
                    );
                }
                let use_qr = json_bool(&fields, "qr", true);
                let validate = json_bool(&fields, "validate", false);
                let skip_download = json_bool(&fields, "skip_download", false);
                let terminal = json_bool(&fields, "terminal", false);
                let username = fields.get("username").cloned().unwrap_or_default();
                let copy_to = fields.get("copy_to").cloned().unwrap_or_default();

                let mut args = vec!["steam-port".to_string(), target.clone()];
                if skip_download {
                    args.push("--skip-download".to_string());
                } else if use_qr {
                    args.push("--qr".to_string());
                } else if !username.trim().is_empty() {
                    args.push("--user".to_string());
                    args.push(username);
                }
                if validate {
                    args.push("--validate".to_string());
                }
                if !copy_to.trim().is_empty() {
                    args.push("--copy-to".to_string());
                    args.push(copy_to);
                }

                if terminal && !skip_download {
                    match launch_in_macos_terminal(&state.self_exe, &state.workspace_root, &args) {
                        Ok(msg) => {
                            return write_http_response(
                                &mut stream,
                                "200 OK",
                                "application/json; charset=utf-8",
                                &format!("{{\"ok\":true,\"terminal\":true,\"message\":{}}}", json_str(&msg)),
                            );
                        }
                        Err(e) => {
                            return write_http_response(
                                &mut stream,
                                "500 Internal Server Error",
                                "application/json; charset=utf-8",
                                &format!("{{\"error\":{}}}", json_str(&e.to_string())),
                            );
                        }
                    }
                }

                let id = spawn_pf_job(state, &format!("Download & Port to .app: {target}"), args);
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    &format!("{{\"ok\":true,\"job_id\":{id}}}"),
                );
            }
            "/api/verify" => {
                let app_path_str = fields.get("path").cloned().unwrap_or_default();
                let app_path = PathBuf::from(&app_path_str);
                match verify_app_bundle(&app_path) {
                    Ok(report) => {
                        let checks_json: Vec<String> = report
                            .checks
                            .iter()
                            .map(|c| {
                                let st = match c.status {
                                    BundleCheckStatus::Pass => "pass",
                                    BundleCheckStatus::Warn => "warn",
                                    BundleCheckStatus::Fail => "fail",
                                };
                                format!(
                                    "{{\"name\":{},\"status\":{},\"detail\":{}}}",
                                    json_str(&c.name),
                                    json_str(st),
                                    json_str(&c.detail)
                                )
                            })
                            .collect();
                        let body = format!(
                            "{{\"ok\":true,\"passed\":{},\"fail_count\":{},\"warn_count\":{},\"path\":{},\"checks\":[{}]}}",
                            report.passed(),
                            report.fail_count(),
                            report.warn_count(),
                            json_str(&report.app_path.to_string_lossy()),
                            checks_json.join(",")
                        );
                        return write_http_response(
                            &mut stream,
                            "200 OK",
                            "application/json; charset=utf-8",
                            &body,
                        );
                    }
                    Err(e) => {
                        return write_http_response(
                            &mut stream,
                            "500 Internal Server Error",
                            "application/json; charset=utf-8",
                            &format!("{{\"error\":{}}}", json_str(&e.to_string())),
                        );
                    }
                }
            }
            "/api/export" => {
                let app_path = fields.get("path").cloned().unwrap_or_default();
                let dest = fields.get("dest").cloned().unwrap_or_default();
                if app_path.trim().is_empty() || dest.trim().is_empty() {
                    return write_http_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        "{\"error\":\"missing bundle path or destination SSD\"}",
                    );
                }
                let args = vec!["bundle-export".to_string(), app_path.clone(), dest.clone()];
                let id = spawn_pf_job(state, &format!("Export to SSD: {dest}"), args);
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    &format!("{{\"ok\":true,\"job_id\":{id}}}"),
                );
            }
            "/api/launch" => {
                let app_path = PathBuf::from(fields.get("path").cloned().unwrap_or_default());
                if !app_path.is_dir()
                    || !app_path
                        .extension()
                        .and_then(|s| s.to_str())
                        .is_some_and(|e| e.eq_ignore_ascii_case("app"))
                {
                    return write_http_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        "{\"error\":\"path must be an existing .app bundle\"}",
                    );
                }
                #[cfg(target_os = "macos")]
                let _ = Command::new("/usr/bin/open").arg(&app_path).spawn();
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    "{\"ok\":true}",
                );
            }
            "/api/reveal" => {
                let p = PathBuf::from(fields.get("path").cloned().unwrap_or_default());
                if !p.exists() {
                    return write_http_response(
                        &mut stream,
                        "400 Bad Request",
                        "application/json; charset=utf-8",
                        "{\"error\":\"path does not exist\"}",
                    );
                }
                #[cfg(target_os = "macos")]
                let _ = Command::new("/usr/bin/open").args(["-R"]).arg(&p).spawn();
                return write_http_response(
                    &mut stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    "{\"ok\":true}",
                );
            }
            other if other.starts_with("/api/jobs/") && other.ends_with("/cancel") => {
                let id_str = other
                    .trim_start_matches("/api/jobs/")
                    .trim_end_matches("/cancel");
                if let Ok(id) = id_str.parse::<u64>() {
                    let mut jobs = state.jobs.lock().unwrap();
                    if let Some(job) = jobs.iter_mut().find(|j| j.id == id) {
                        if let Some(pid) = job.child_pid {
                            let _ = Command::new("/bin/kill")
                                .args(["-TERM", &pid.to_string()])
                                .status();
                        }
                        if job.status == JobStatus::Running {
                            job.status = JobStatus::Cancelled;
                            job.logs.push_str("\n[cancelled by user]\n");
                        }
                        return write_http_response(
                            &mut stream,
                            "200 OK",
                            "application/json; charset=utf-8",
                            "{\"ok\":true}",
                        );
                    }
                }
                return write_http_response(
                    &mut stream,
                    "404 Not Found",
                    "application/json; charset=utf-8",
                    "{\"error\":\"job not found\"}",
                );
            }
            _ => {}
        }
    }

    write_http_response(
        &mut stream,
        "404 Not Found",
        "application/json; charset=utf-8",
        "{\"error\":\"not found\"}",
    )
}

fn spawn_pf_job(state: &Arc<SharedState>, title: &str, args: Vec<String>) -> u64 {
    let id = state.next_job_id.fetch_add(1, Ordering::SeqCst);
    let preview = format!("port-factory {}", args.join(" "));
    {
        let mut jobs = state.jobs.lock().unwrap();
        jobs.push(JobEntry {
            id,
            title: title.to_string(),
            command_preview: preview.clone(),
            status: JobStatus::Running,
            exit_code: None,
            logs: format!("$ {preview}\n"),
            child_pid: None,
            started_unix: now_unix(),
        });
    }

    let st = Arc::clone(state);
    thread::spawn(move || {
        let mut cmd = Command::new(&st.self_exe);
        cmd.current_dir(&st.workspace_root)
            .env("PORT_FACTORY_WORKSPACE", &st.workspace_root)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child: Child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                let mut jobs = st.jobs.lock().unwrap();
                if let Some(j) = jobs.iter_mut().find(|j| j.id == id) {
                    j.status = JobStatus::Failed;
                    j.logs.push_str(&format!("\nfailed to spawn: {e}\n"));
                }
                return;
            }
        };

        let pid = child.id();
        {
            let mut jobs = st.jobs.lock().unwrap();
            if let Some(j) = jobs.iter_mut().find(|j| j.id == id) {
                j.child_pid = Some(pid);
            }
        }

        let mut child = child;
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let st_out = Arc::clone(&st);
        let t_out = thread::spawn(move || {
            if let Some(out) = stdout {
                stream_pipe_to_job(out, &st_out, id);
            }
        });
        let st_err = Arc::clone(&st);
        let t_err = thread::spawn(move || {
            if let Some(err) = stderr {
                stream_pipe_to_job(err, &st_err, id);
            }
        });

        let wait_res = child.wait();
        let _ = t_out.join();
        let _ = t_err.join();

        let mut jobs = st.jobs.lock().unwrap();
        if let Some(j) = jobs.iter_mut().find(|j| j.id == id) {
            j.child_pid = None;
            match wait_res {
                Ok(status) => {
                    j.exit_code = status.code();
                    if j.status != JobStatus::Cancelled {
                        j.status = if status.success() {
                            JobStatus::Succeeded
                        } else {
                            JobStatus::Failed
                        };
                    }
                    j.logs.push_str(&format!("\n[exited with {status}]\n"));
                }
                Err(e) => {
                    j.status = JobStatus::Failed;
                    j.logs.push_str(&format!("\n[wait error: {e}]\n"));
                }
            }
        }
    });

    id
}

fn stream_pipe_to_job<R: Read>(mut pipe: R, state: &Arc<SharedState>, job_id: u64) {
    let mut buf = [0u8; 4096];
    loop {
        match pipe.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buf[..n]);
                let mut jobs = state.jobs.lock().unwrap();
                if let Some(j) = jobs.iter_mut().find(|j| j.id == job_id) {
                    j.logs.push_str(&chunk);
                    // Cap in-memory job log to ~512 KiB while preserving the tail.
                    if j.logs.len() > 512 * 1024 {
                        let cut = j.logs.len() - 384 * 1024;
                        let safe = j
                            .logs
                            .char_indices()
                            .find(|(idx, _)| *idx >= cut)
                            .map(|(idx, _)| idx)
                            .unwrap_or(0);
                        j.logs.drain(..safe);
                    }
                }
            }
            Err(_) => break,
        }
    }
}

fn shell_quote(arg: &str) -> String {
    format!("'{}'", arg.replace('\'', "'\\''"))
}

/// Open a native macOS Terminal window running `port-factory <args...>` so interactive
/// Steam password / Steam Guard 2FA prompts stay strictly inside the local Terminal.
fn launch_in_macos_terminal(
    self_exe: &Path,
    workspace_root: &Path,
    args: &[String],
) -> Result<String> {
    let script_path = std::env::temp_dir().join(format!(
        "port_factory_steam_{}_{}.command",
        std::process::id(),
        now_unix()
    ));
    let mut cmd_line = format!(
        "PORT_FACTORY_WORKSPACE={} {}",
        shell_quote(&workspace_root.to_string_lossy()),
        shell_quote(&self_exe.to_string_lossy())
    );
    for a in args {
        cmd_line.push(' ');
        cmd_line.push_str(&shell_quote(a));
    }
    let script = format!(
        "#!/bin/sh\nclear\ncd {} || exit 1\necho \"=== Port Factory Interactive Steam Session ===\"\n{cmd_line}\necho \"\"\necho \"Done. You may close this window and refresh the Port Factory Launcher.\"\n",
        shell_quote(&workspace_root.to_string_lossy())
    );
    fs::write(&script_path, script)?;
    set_executable_mode(&script_path)?;
    #[cfg(target_os = "macos")]
    {
        let st = Command::new("/usr/bin/open")
            .args(["-a", "Terminal"])
            .arg(&script_path)
            .status()
            .context("opening Terminal.app")?;
        if !st.success() {
            bail!("failed to open Terminal.app (status {st})");
        }
    }
    Ok(format!(
        "Opened interactive Terminal session ({})",
        script_path.display()
    ))
}

fn build_state_json(state: &SharedState) -> String {
    let report = scan_all_steam_libraries(&state.workspace_root).ok();
    let dd_path = find_depot_downloader();
    let bundles = list_packaged_bundles(&state.workspace_root);
    let volumes = list_external_volumes();

    let users_json: Vec<String> = report
        .as_ref()
        .map(|r| {
            r.users
                .iter()
                .map(|u| {
                    format!(
                        "{{\"account_name\":{},\"persona_name\":{},\"steam_id64\":{},\"most_recent\":{}}}",
                        json_str(&u.account_name),
                        json_str(&u.persona_name),
                        json_str(&u.steam_id64),
                        u.most_recent
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let installed_json: Vec<String> = report
        .as_ref()
        .map(|r| {
            r.installed_games
                .iter()
                .map(|g| {
                    let exe_name = g
                        .primary_exe
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    format!(
                        "{{\"app_id\":{},\"name\":{},\"source\":{},\"install_dir\":{},\"size_bytes\":{},\"platform\":{},\"primary_exe\":{},\"is_windows\":{},\"readiness_pct\":{:.1},\"resolved_imports\":{},\"total_imports\":{},\"engine_profile\":{}}}",
                        json_str(&g.app_id),
                        json_str(&g.name),
                        json_str(&g.source.label()),
                        json_str(&g.install_dir.to_string_lossy()),
                        g.size_on_disk_bytes,
                        json_str(&g.platform),
                        json_str(exe_name),
                        g.primary_exe.is_some(),
                        g.readiness_pct(),
                        g.resolved_imports,
                        g.total_imports,
                        json_str(&g.engine_profile),
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let owned_json: Vec<String> = report
        .as_ref()
        .map(|r| {
            r.owned_games
                .iter()
                .map(|o| {
                    format!(
                        "{{\"app_id\":{},\"title\":{},\"engine_hint\":{},\"installed\":{},\"install_source\":{},\"playtime_minutes\":{}}}",
                        o.app_id,
                        json_str(&o.title),
                        json_str(&o.engine_hint),
                        o.installed,
                        o.install_source
                            .as_deref()
                            .map(json_str)
                            .unwrap_or_else(|| "null".to_string()),
                        o.playtime_minutes
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    let bundles_json: Vec<String> = bundles
        .iter()
        .map(|b| {
            format!(
                "{{\"name\":{},\"path\":{},\"runtime\":{},\"game_exe\":{},\"aot_modules\":{},\"has_metal_irconverter\":{},\"has_dxilconv\":{},\"has_moltenvk\":{},\"is_symlinked_game\":{},\"size_bytes\":{}}}",
                json_str(&b.name),
                json_str(&b.path.to_string_lossy()),
                json_str(&b.runtime),
                json_str(&b.game_exe),
                b.aot_modules,
                b.has_metal_irconverter,
                b.has_dxilconv,
                b.has_moltenvk,
                b.is_symlinked_game,
                b.size_bytes
            )
        })
        .collect();

    let volumes_json: Vec<String> = volumes
        .iter()
        .map(|v| {
            format!(
                "{{\"name\":{},\"path\":{},\"free_bytes\":{},\"total_bytes\":{}}}",
                json_str(&v.name),
                json_str(&v.path.to_string_lossy()),
                v.free_bytes,
                v.total_bytes
            )
        })
        .collect();

    let jobs_json: Vec<String> = {
        let jobs = state.jobs.lock().unwrap();
        jobs.iter()
            .rev()
            .take(20)
            .map(|j| format_job_json(j, false))
            .collect()
    };

    format!(
        "{{\"workspace_root\":{},\"depot_downloader\":{},\"users\":[{}],\"installed\":[{}],\"owned\":[{}],\"bundles\":[{}],\"volumes\":[{}],\"jobs\":[{}]}}",
        json_str(&state.workspace_root.to_string_lossy()),
        dd_path
            .as_ref()
            .map(|p| json_str(&p.to_string_lossy()))
            .unwrap_or_else(|| "null".to_string()),
        users_json.join(","),
        installed_json.join(","),
        owned_json.join(","),
        bundles_json.join(","),
        volumes_json.join(","),
        jobs_json.join(","),
    )
}

fn format_job_json(job: &JobEntry, include_logs: bool) -> String {
    let exit = job
        .exit_code
        .map(|c| c.to_string())
        .unwrap_or_else(|| "null".to_string());
    if include_logs {
        format!(
            "{{\"id\":{},\"title\":{},\"command\":{},\"status\":{},\"exit_code\":{},\"started_unix\":{},\"logs\":{}}}",
            job.id,
            json_str(&job.title),
            json_str(&job.command_preview),
            json_str(job.status.as_str()),
            exit,
            job.started_unix,
            json_str(&job.logs)
        )
    } else {
        format!(
            "{{\"id\":{},\"title\":{},\"command\":{},\"status\":{},\"exit_code\":{},\"started_unix\":{}}}",
            job.id,
            json_str(&job.title),
            json_str(&job.command_preview),
            json_str(job.status.as_str()),
            exit,
            job.started_unix
        )
    }
}

fn json_str(s: &str) -> String {
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
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_bool(fields: &HashMap<String, String>, key: &str, default: bool) -> bool {
    match fields.get(key).map(|s| s.trim().to_ascii_lowercase()) {
        Some(ref s) if s == "true" || s == "1" => true,
        Some(ref s) if s == "false" || s == "0" => false,
        _ => default,
    }
}

/// Minimal flat JSON object parser (`{"key":"val","flag":true}`) with string unescaping.
fn parse_flat_json(body: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let bytes = body.as_bytes();
    let mut i = 0usize;
    let skip_ws = |i: &mut usize| {
        while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
            *i += 1;
        }
    };
    let read_str = |i: &mut usize| -> Option<String> {
        if *i >= bytes.len() || bytes[*i] != b'"' {
            return None;
        }
        *i += 1;
        let mut s = String::new();
        while *i < bytes.len() {
            let b = bytes[*i];
            if b == b'"' {
                *i += 1;
                return Some(s);
            }
            if b == b'\\' && *i + 1 < bytes.len() {
                *i += 1;
                match bytes[*i] {
                    b'"' => s.push('"'),
                    b'\\' => s.push('\\'),
                    b'/' => s.push('/'),
                    b'n' => s.push('\n'),
                    b'r' => s.push('\r'),
                    b't' => s.push('\t'),
                    other => s.push(other as char),
                }
                *i += 1;
            } else {
                // Preserve full UTF-8 characters cleanly.
                let rest = &body[*i..];
                let ch = rest.chars().next()?;
                s.push(ch);
                *i += ch.len_utf8();
            }
        }
        None
    };

    skip_ws(&mut i);
    if i >= bytes.len() || bytes[i] != b'{' {
        return out;
    }
    i += 1;
    while i < bytes.len() {
        skip_ws(&mut i);
        if i >= bytes.len() || bytes[i] == b'}' {
            break;
        }
        let Some(key) = read_str(&mut i) else {
            break;
        };
        skip_ws(&mut i);
        if i >= bytes.len() || bytes[i] != b':' {
            break;
        }
        i += 1;
        skip_ws(&mut i);
        if i < bytes.len() && bytes[i] == b'"' {
            if let Some(val) = read_str(&mut i) {
                out.insert(key, val);
            }
        } else {
            let start = i;
            while i < bytes.len() && bytes[i] != b',' && bytes[i] != b'}' {
                i += 1;
            }
            let raw = body[start..i].trim();
            if raw != "null" {
                out.insert(key, raw.to_string());
            }
        }
        skip_ws(&mut i);
        if i < bytes.len() && bytes[i] == b',' {
            i += 1;
        }
    }
    out
}

const LAUNCHER_HTML_TEMPLATE: &str = include_str!("../assets/launcher.html");

#[cfg(test)]
mod tests {
    use super::*;

    fn http_req(addr: SocketAddr, req: &str) -> String {
        let mut s = TcpStream::connect(addr).expect("connect to launcher");
        s.write_all(req.as_bytes()).unwrap();
        s.flush().unwrap();
        let mut resp = String::new();
        s.read_to_string(&mut resp).unwrap();
        resp
    }

    #[test]
    fn launcher_serves_ui_state_and_enforces_csrf_token() {
        let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .unwrap()
            .to_path_buf();
        let mut srv = start_launcher_server(LauncherOptions {
            workspace_root: ws,
            port: 0,
            open_browser: false,
        })
        .expect("start launcher server");

        let ping = http_req(
            srv.addr,
            "GET /api/ping HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n",
        );
        assert!(ping.contains("200 OK") && ping.contains("port-factory-ok"));

        let html = http_req(
            srv.addr,
            "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n",
        );
        assert!(html.contains("200 OK"));
        assert!(html.contains("Port Factory Launcher"));
        assert!(html.contains(&srv.token));

        // POST without CSRF token must be rejected with 403 Forbidden.
        let body = r#"{"path":"/nonexistent/Fake.app"}"#;
        let denied = http_req(
            srv.addr,
            &format!(
                "POST /api/verify HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            ),
        );
        assert!(denied.contains("403 Forbidden"), "denied={denied}");

        // POST with valid CSRF token returns structured verification JSON.
        let allowed = http_req(
            srv.addr,
            &format!(
                "POST /api/verify HTTP/1.1\r\nHost: 127.0.0.1\r\nX-PF-Token: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                srv.token,
                body.len(),
                body
            ),
        );
        assert!(allowed.contains("200 OK"), "allowed={allowed}");
        assert!(allowed.contains("\"passed\":false"));
        assert!(allowed.contains("\"bundle_root\""));

        srv.shutdown();
    }

    #[test]
    fn flat_json_parser_handles_strings_escapes_and_booleans() {
        let parsed = parse_flat_json(
            r#"{"target":"2357570","qr":true,"validate":false,"dir":"/tmp/My \"Game\"\\Dir"}"#,
        );
        assert_eq!(parsed.get("target").map(String::as_str), Some("2357570"));
        assert_eq!(parsed.get("qr").map(String::as_str), Some("true"));
        assert_eq!(parsed.get("validate").map(String::as_str), Some("false"));
        assert_eq!(
            parsed.get("dir").map(String::as_str),
            Some("/tmp/My \"Game\"\\Dir")
        );
    }
}
