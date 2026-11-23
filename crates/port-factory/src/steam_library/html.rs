//! Interactive HTML Steam Library Browser (`out/steam_library_browser.html`) and
//! human-readable CLI scan report formatter.

use super::discovery::SteamLibraryScanReport;
use anyhow::Result;
use std::fs;
use std::path::Path;

const STEAM_BROWSER_HTML_TEMPLATE: &str = include_str!("../../assets/steam_browser.html");

/// Export a rich, interactive dark-mode HTML Steam Library Browser & One-Click Converter UI.
pub fn write_steam_browser_html(report: &SteamLibraryScanReport, out_path: &Path) -> Result<()> {
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let user_banner = report
        .users
        .first()
        .map(|u| {
            format!(
                "{} ({}) — SteamID64: {}",
                u.persona_name, u.account_name, u.steam_id64
            )
        })
        .unwrap_or_else(|| "Local macOS Steam User".to_string());

    let mut installed_rows = String::new();
    for g in &report.installed_games {
        let size_gb = (g.size_on_disk_bytes as f64) / 1_000_000_000.0;
        let readiness = g.readiness_pct();
        let badge_color = if readiness >= 99.0 {
            "#10b981"
        } else if readiness >= 85.0 {
            "#3b82f6"
        } else {
            "#f59e0b"
        };
        let exe_name = g
            .primary_exe
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("macOS Bundle");
        let cmd_app = format!(
            "cargo opsdb -- steam-convert {} --format app",
            g.app_id
        );
        let cmd_aot = format!(
            "cargo opsdb -- steam-convert {} --format aot-crate",
            g.app_id
        );
        installed_rows.push_str(&format!(
            r#"<div class="card installed-card" data-name="{}" data-id="{}">
                <div class="card-header">
                    <div>
                        <span class="source-pill">{}</span>
                        <span class="appid-pill">AppID: {}</span>
                    </div>
                    <span class="readiness-pill" style="background:{badge_color}22;color:{badge_color};border:1px solid {badge_color}55">
                        {:.1}% Win32 Ready ({}/{})
                    </span>
                </div>
                <h3>{}</h3>
                <div class="meta-grid">
                    <div><strong>Executable:</strong> <code>{}</code> ({} PE modules)</div>
                    <div><strong>Engine Profile:</strong> {}</div>
                    <div><strong>Platform:</strong> {} ({:.2} GB)</div>
                    <div><strong>Install Path:</strong> <code>{}</code></div>
                </div>
                <div class="action-bar">
                    <button class="btn-primary" onclick="copyCmd('{}', this)">One-Click Convert (.app)</button>
                    <button class="btn-secondary" onclick="copyCmd('{}', this)">Convert to Rust AOT Crate</button>
                </div>
            </div>"#,
            html_escape(&g.name.to_ascii_lowercase()),
            html_escape(&g.app_id),
            html_escape(&g.source.label()),
            html_escape(&g.app_id),
            readiness,
            g.resolved_imports,
            g.total_imports,
            html_escape(&g.name),
            html_escape(exe_name),
            g.pe_count,
            html_escape(&g.engine_profile),
            html_escape(&g.platform),
            size_gb,
            html_escape(&g.install_dir.to_string_lossy()),
            html_escape(&cmd_app),
            html_escape(&cmd_aot),
        ));
    }

    let mut owned_rows = String::new();
    for o in &report.owned_games {
        let status_badge = if o.installed {
            format!(
                r#"<span class="status-installed">Installed ({})</span>"#,
                html_escape(o.install_source.as_deref().unwrap_or("Local"))
            )
        } else {
            r#"<span class="status-cloud">Owned in Steam Cloud</span>"#.to_string()
        };
        let hours = (o.playtime_minutes as f64) / 60.0;
        let dl_cmd = format!(
            "cargo opsdb -- steam-download {} && cargo opsdb -- steam-convert {} --format app",
            o.app_id, o.app_id
        );
        owned_rows.push_str(&format!(
            r#"<tr class="owned-row" data-name="{}" data-id="{}">
                <td><code>{}</code></td>
                <td><strong>{}</strong></td>
                <td>{}</td>
                <td>{}</td>
                <td>{:.1} hrs ({} cached achievements)</td>
                <td>
                    <button class="btn-small" onclick="copyCmd('{}', this)">Download Win64 &amp; Convert</button>
                </td>
            </tr>"#,
            html_escape(&o.title.to_ascii_lowercase()),
            o.app_id,
            o.app_id,
            html_escape(&o.title),
            html_escape(&o.engine_hint),
            status_badge,
            hours,
            o.achievements_cached,
            html_escape(&dl_cmd),
        ));
    }

    let html = STEAM_BROWSER_HTML_TEMPLATE
        .replace("__USER_BANNER__", &html_escape(&user_banner))
        .replace("__INSTALLED_COUNT__", &report.installed_games.len().to_string())
        .replace("__OWNED_COUNT__", &report.owned_games.len().to_string())
        .replace("__INSTALLED_ROWS__", &installed_rows)
        .replace("__OWNED_ROWS__", &owned_rows);

    fs::write(out_path, html)?;
    Ok(())
}

pub(super) fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Format a human-readable CLI summary of the [`SteamLibraryScanReport`].
pub fn format_steam_scan_report(report: &SteamLibraryScanReport) -> String {
    let mut out = String::new();
    out.push_str("=== Port Factory Steam & PC Game Library Scanner ===\n");
    if !report.users.is_empty() {
        out.push_str("  Steam Accounts:\n");
        for u in &report.users {
            out.push_str(&format!(
                "    - {} ({}) [SteamID64: {}, AccountID: {}{}]\n",
                u.persona_name,
                u.account_name,
                u.steam_id64,
                u.account_id32,
                if u.most_recent { ", MostRecent" } else { "" }
            ));
        }
    }
    out.push_str(&format!(
        "  Scanned Library Folders: {}\n",
        report.library_folders.len()
    ));
    for dir in &report.library_folders {
        out.push_str(&format!("    - {}\n", dir.display()));
    }

    out.push_str(&format!(
        "\n=== Installed Games & Clients ({}) ===\n",
        report.installed_games.len()
    ));
    out.push_str(&format!(
        "  {:18} {:28} {:28} {:>9} {:>12} {}\n",
        "APPID", "TITLE", "SOURCE", "SIZE", "WIN32 READY", "ENGINE / EXECUTABLE"
    ));
    out.push_str(&format!("  {}\n", "-".repeat(130)));
    for g in &report.installed_games {
        let size_gb = (g.size_on_disk_bytes as f64) / 1_000_000_000.0;
        let exe_label = g
            .primary_exe
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("macOS.app");
        out.push_str(&format!(
            "  {:18} {:28} {:28} {:>6.1} GB {:>6.1}% ({}/{}) {} [{}]\n",
            g.app_id,
            truncate_str(&g.name, 27),
            truncate_str(&g.source.label(), 27),
            size_gb,
            g.readiness_pct(),
            g.resolved_imports,
            g.total_imports,
            exe_label,
            g.engine_profile
        ));
    }

    out.push_str(&format!(
        "\n=== Owned Steam Library ({} Games Discovered in Local Cache) ===\n",
        report.owned_games.len()
    ));
    out.push_str(&format!(
        "  {:>10} {:34} {:16} {:>10} {}\n",
        "APPID", "TITLE", "STATUS", "PLAYTIME", "ENGINE / PIPELINE"
    ));
    out.push_str(&format!("  {}\n", "-".repeat(115)));
    for o in report.owned_games.iter().take(35) {
        let status = if o.installed { "INSTALLED" } else { "CLOUD" };
        let hrs = (o.playtime_minutes as f64) / 60.0;
        out.push_str(&format!(
            "  {:>10} {:34} {:16} {:>7.1} hr {}\n",
            o.app_id,
            truncate_str(&o.title, 33),
            status,
            hrs,
            o.engine_hint
        ));
    }
    if report.owned_games.len() > 35 {
        out.push_str(&format!(
            "  ... and {} more owned Steam games (see out/steam_library_browser.html)\n",
            report.owned_games.len() - 35
        ));
    }
    out
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        let mut t: String = chars[..max_chars.saturating_sub(1)].iter().collect();
        t.push('…');
        t
    }
}
