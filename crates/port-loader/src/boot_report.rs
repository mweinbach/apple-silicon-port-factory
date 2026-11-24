//! Unified single-file boot report (`port-runner --report-json <PATH>` or `PORT_REPORT_JSON`).
//!
//! Written automatically from the process exit hooks (clean exit, non-zero exit,
//! crash, unimplemented-import trap, `--no-run`, or fatal startup error) so every
//! run produces one structured JSON file containing:
//! - exit kind, exit code, detail, elapsed time, frames presented, graphics API
//! - ranked blockers (fatal fault / unimplemented import, unresolved static imports,
//!   top hit stubs, shader/PSO compile failures)
//! - full stub registry snapshot (`hits`, `first_frame`, `first_guest_caller`)
//! - execution tier counters (interpreter, JIT, dynamic AOT, static AOT)
//! - Tier-1 shader converter & Tier-2 Metal 4 PSO cache stats.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::Instant;

struct ReportConfig {
    path: PathBuf,
    exe: String,
    game_dir: String,
    manifest: String,
    started_at: Instant,
}

static REPORT_CFG: Mutex<Option<ReportConfig>> = Mutex::new(None);
static BOOT_START: OnceLock<Instant> = OnceLock::new();

pub fn mark_boot_start() {
    let _ = BOOT_START.set(Instant::now());
}

/// Configure the `--report-json` output path and target metadata, and register the exit hook.
pub fn configure(path: PathBuf, exe: String, game_dir: String, manifest: Option<String>) {
    let started_at = *BOOT_START.get_or_init(Instant::now);
    let mut g = REPORT_CFG.lock().unwrap_or_else(|e| e.into_inner());
    *g = Some(ReportConfig {
        path,
        exe,
        game_dir,
        manifest: manifest.unwrap_or_default(),
        started_at,
    });
    drop(g);
    win32_rt::process::register_exit_hook(write_exit_report);
}

/// Update target metadata after manifest/PE paths are canonicalized.
pub fn update_target(exe: String, game_dir: String, manifest: Option<String>) {
    if let Ok(mut g) = REPORT_CFG.lock() {
        if let Some(ref mut cfg) = *g {
            cfg.exe = exe;
            cfg.game_dir = game_dir;
            if let Some(m) = manifest {
                cfg.manifest = m;
            }
        }
    }
}

/// Registered process exit hook: writes the unified boot report JSON if configured.
pub fn write_exit_report() {
    write_report_with_override(None, None);
}

/// Write the boot report immediately with an explicit exit kind/code override
/// (used for `--no-run` or fatal startup errors before `exit_process`).
pub fn write_report_with_override(
    exit_kind_override: Option<(&str, &str)>,
    exit_code_override: Option<u32>,
) {
    let cfg = match REPORT_CFG.try_lock() {
        Ok(g) => match g.as_ref() {
            Some(c) => (
                c.path.clone(),
                c.exe.clone(),
                c.game_dir.clone(),
                c.manifest.clone(),
                c.started_at,
            ),
            None => return,
        },
        Err(_) => return,
    };
    let (path, exe, game_dir, manifest, started_at) = cfg;

    let (exit_kind, exit_detail) = match exit_kind_override {
        Some((k, d)) => (k.to_string(), d.to_string()),
        None => win32_rt::process::exit_reason(),
    };
    let exit_code = exit_code_override.or_else(win32_rt::process::exit_code);
    let elapsed_ms = started_at.elapsed().as_millis() as u64;
    let frames_presented = win32_rt::present::present_count() as u64;
    let graphics_api = win32_rt::present::last_present_api().label();

    let stub_snap = win32_rt::stub_registry::snapshot();
    let tiers = x86_runtime::emu::tier_stats();
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    let shaders = d3d12_metal::metal::pipeline::shader_report_stats_if_initialized();
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    let shaders = d3d12_metal::metal::pipeline::ShaderReportStats::default();

    let unresolved_imports: Vec<String> = crate::loader::try_get()
        .and_then(|l| l.unresolved.try_lock().ok().map(|u| u.iter().cloned().collect()))
        .unwrap_or_default();

    // Build ranked blockers (fatal > shader/pso errors > top runtime stubs > unresolved static imports).
    struct Blocker {
        severity: &'static str,
        category: String,
        symbol: String,
        count: u64,
        first_frame: u64,
        first_caller: String,
        detail: String,
    }
    let mut blockers: Vec<Blocker> = Vec::new();
    if matches!(
        exit_kind.as_str(),
        "unimplemented_import" | "crash" | "stall" | "startup_error"
    ) {
        blockers.push(Blocker {
            severity: "fatal",
            category: exit_kind.clone(),
            symbol: exit_detail.clone(),
            count: 1,
            first_frame: frames_presented,
            first_caller: String::new(),
            detail: exit_detail.clone(),
        });
    }
    if shaders.tier1_failures > 0 {
        blockers.push(Blocker {
            severity: "error",
            category: "shader_compile_failure".into(),
            symbol: "dxil-metal".into(),
            count: shaders.tier1_failures,
            first_frame: 0,
            first_caller: String::new(),
            detail: format!("{} Tier-1 shader conversion failure(s)", shaders.tier1_failures),
        });
    }
    if shaders.pso_failures > 0 {
        blockers.push(Blocker {
            severity: "error",
            category: "pso_compile_failure".into(),
            symbol: "metal4-pso".into(),
            count: shaders.pso_failures,
            first_frame: 0,
            first_caller: String::new(),
            detail: format!("{} Metal 4 PSO compile failure(s)", shaders.pso_failures),
        });
    }
    for e in &stub_snap.entries {
        blockers.push(Blocker {
            severity: "stub",
            category: e.kind.as_str().to_string(),
            symbol: format!("{}!{}", e.dll, e.api),
            count: e.hits,
            first_frame: e.first_frame,
            first_caller: e.first_guest_caller.clone(),
            detail: e.first_detail.clone(),
        });
    }
    for sym in &unresolved_imports {
        blockers.push(Blocker {
            severity: "warning",
            category: "unresolved_import".into(),
            symbol: sym.clone(),
            count: 1,
            first_frame: 0,
            first_caller: String::new(),
            detail: "static import bound to trap thunk".into(),
        });
    }

    let mut out = String::new();
    let _ = writeln!(out, "{{");
    let _ = writeln!(out, "  \"schema_version\": 1,");
    let _ = writeln!(out, "  \"target\": {{");
    let _ = writeln!(out, "    \"exe\": {},", json_quote(&exe));
    let _ = writeln!(out, "    \"game_dir\": {},", json_quote(&game_dir));
    let _ = writeln!(out, "    \"manifest\": {}", json_quote(&manifest));
    let _ = writeln!(out, "  }},");
    let _ = writeln!(out, "  \"exit\": {{");
    let _ = writeln!(out, "    \"kind\": {},", json_quote(&exit_kind));
    match exit_code {
        Some(c) => { let _ = writeln!(out, "    \"code\": {c},"); }
        None => { let _ = writeln!(out, "    \"code\": null,"); }
    }
    let _ = writeln!(out, "    \"detail\": {},", json_quote(&exit_detail));
    let _ = writeln!(out, "    \"elapsed_ms\": {elapsed_ms},");
    let _ = writeln!(out, "    \"frames_presented\": {frames_presented},");
    let _ = writeln!(out, "    \"graphics_api\": {}", json_quote(graphics_api));
    let _ = writeln!(out, "  }},");

    // Ranked blockers
    let _ = writeln!(out, "  \"ranked_blockers\": [");
    for (idx, b) in blockers.iter().enumerate() {
        let comma = if idx + 1 < blockers.len() { "," } else { "" };
        let _ = writeln!(
            out,
            "    {{\"rank\":{},\"severity\":{},\"category\":{},\"symbol\":{},\"count\":{},\"first_frame\":{},\"first_caller\":{},\"detail\":{}}}{comma}",
            idx + 1,
            json_quote(b.severity),
            json_quote(&b.category),
            json_quote(&b.symbol),
            b.count,
            b.first_frame,
            json_quote(&b.first_caller),
            json_quote(&b.detail),
        );
    }
    let _ = writeln!(out, "  ],");

    // Stubs snapshot (indent embedded JSON cleanly)
    let stub_json = stub_snap.to_json();
    let mut indented_stubs = String::new();
    for (i, line) in stub_json.lines().enumerate() {
        if i == 0 {
            indented_stubs.push_str(line);
        } else {
            indented_stubs.push('\n');
            indented_stubs.push_str("  ");
            indented_stubs.push_str(line);
        }
    }
    let _ = writeln!(out, "  \"stubs\": {indented_stubs},");

    // Execution tier stats
    let _ = writeln!(out, "  \"tiers\": {{");
    let _ = writeln!(out, "    \"exec_mode\": {},", json_quote(tiers.exec_mode));
    let _ = writeln!(out, "    \"decoded_interp_blocks\": {},", tiers.decoded_interp_blocks);
    let _ = writeln!(out, "    \"jit_blocks_compiled\": {},", tiers.jit_blocks_compiled);
    let _ = writeln!(out, "    \"aot_active\": {},", tiers.aot_active);
    let _ = writeln!(out, "    \"aot_kills\": {},", tiers.aot_kills);
    let _ = writeln!(out, "    \"aot_revived\": {},", tiers.aot_revived);
    let _ = writeln!(out, "    \"static_aot_modules\": {},", tiers.static_aot_modules);
    let _ = writeln!(out, "    \"static_aot_blocks\": {},", tiers.static_aot_blocks);
    let _ = writeln!(out, "    \"static_aot_bytes\": {}", tiers.static_aot_bytes);
    let _ = writeln!(out, "  }},");

    // Shader & PSO stats
    let _ = writeln!(out, "  \"shaders\": {{");
    let _ = writeln!(out, "    \"tier1_initialized\": {},", shaders.tier1_initialized);
    let _ = writeln!(out, "    \"tier1_memory_hits\": {},", shaders.tier1_memory_hits);
    let _ = writeln!(out, "    \"tier1_disk_hits\": {},", shaders.tier1_disk_hits);
    let _ = writeln!(out, "    \"tier1_compiles\": {},", shaders.tier1_compiles);
    let _ = writeln!(out, "    \"tier1_failures\": {},", shaders.tier1_failures);
    let _ = writeln!(out, "    \"tier1_compile_ms\": {},", shaders.tier1_compile_ms);
    let _ = writeln!(out, "    \"pso_initialized\": {},", shaders.pso_initialized);
    let _ = writeln!(out, "    \"pso_memory_hits\": {},", shaders.pso_memory_hits);
    let _ = writeln!(out, "    \"pso_archive_hits\": {},", shaders.pso_archive_hits);
    let _ = writeln!(out, "    \"pso_compiles\": {},", shaders.pso_compiles);
    let _ = writeln!(out, "    \"pso_failures\": {},", shaders.pso_failures);
    let _ = writeln!(out, "    \"pso_compile_ms\": {}", shaders.pso_compile_ms);
    let _ = writeln!(out, "  }},");

    // Unresolved static imports
    let _ = writeln!(out, "  \"unresolved_imports\": [");
    for (idx, sym) in unresolved_imports.iter().enumerate() {
        let comma = if idx + 1 < unresolved_imports.len() { "," } else { "" };
        let _ = writeln!(out, "    {}{comma}", json_quote(sym));
    }
    let _ = writeln!(out, "  ]");
    out.push_str("}\n");

    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(&path, out) {
        log::warn!("boot_report: failed to write {}: {e}", path.display());
    } else {
        log::info!("boot_report: wrote {}", path.display());
    }
}

fn json_quote(s: &str) -> String {
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
