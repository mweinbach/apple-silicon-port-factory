//! Declarative per-game target manifest (`game.toml`).
//!
//! Allows any Windows x86-64 game (e.g. Overwatch, Alan Wake 2, Steam titles) to configure
//! display name, primary executable, virtual Windows install dir, log filename, DLL override
//! policy, and quirks without writing Rust code or recompiling the universal `port-runner`.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct TargetManifest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub exe: Option<String>,
    pub win_dir: Option<String>,
    pub log_name: Option<String>,
    pub computer_name: Option<String>,
    pub steam_app_id: Option<String>,
    pub builtin_overrides: Option<String>,
    pub aot_dylibs: Option<String>,
    pub preferred_base_retry: Option<bool>,
    pub v8_jitless: Option<bool>,
    pub legacy_remedy_migration: Option<bool>,
    /// Optional default Windows guest command-line arguments (`[runtime] args = "..."`).
    pub args: Option<String>,
    /// Optional iOS first-launch seed files (`[ios] seed_file = "<bundle_rel> -> <drive_c_rel>"`),
    /// copied into `drive_c` only when the destination does not exist yet.
    pub ios_seed_files: Vec<(String, String)>,
}

impl TargetManifest {
    /// Discover a manifest from `--manifest`, `$PORT_MANIFEST`, `<bundle>/game.toml` (iOS flat
    /// `.app`), or `<bundle>/../Resources/game.toml` (macOS `.app`).
    pub fn discover(explicit: Option<&Path>) -> Option<Self> {
        let found = Self::discover_raw(explicit)?;
        let default_args = found.split_args();
        if !default_args.is_empty() {
            win32_rt::process::set_manifest_game_args(default_args);
        }
        Some(found)
    }

    fn discover_raw(explicit: Option<&Path>) -> Option<Self> {
        if let Some(p) = explicit {
            return Self::load_file(p).ok();
        }
        if let Some(p) = std::env::var_os("PORT_MANIFEST").filter(|v| !v.is_empty()) {
            return Self::load_file(Path::new(&p)).ok();
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                for rel in ["game.toml", "../Resources/game.toml"] {
                    let bundled = dir.join(rel);
                    if bundled.is_file() {
                        return Self::load_file(&bundled).ok();
                    }
                }
            }
        }
        None
    }

    /// Split `[runtime] args` into individual Windows CLI tokens (supports double quotes).
    pub fn split_args(&self) -> Vec<String> {
        let Some(raw) = self.args.as_deref() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        let mut cur = String::new();
        let mut in_quotes = false;
        for ch in raw.chars() {
            match ch {
                '"' => in_quotes = !in_quotes,
                c if c.is_whitespace() && !in_quotes => {
                    if !cur.is_empty() {
                        out.push(std::mem::take(&mut cur));
                    }
                }
                c => cur.push(c),
            }
        }
        if !cur.is_empty() {
            out.push(cur);
        }
        out
    }

    pub fn load_file(path: &Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(Self::parse(&text))
    }

    /// Minimal, dependency-free parser for flat/sectioned `key = value` TOML manifests.
    pub fn parse(text: &str) -> Self {
        let mut m = Self::default();
        let mut section = String::new();
        for raw_line in text.lines() {
            let line = strip_comment(raw_line).trim();
            if line.is_empty() {
                continue;
            }
            if let Some(sec) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                section = sec.trim().to_ascii_lowercase();
                continue;
            }
            let Some((k, v)) = line.split_once('=') else { continue };
            let key = k.trim().to_ascii_lowercase();
            let val = unquote(v.trim());
            let qualified = if section.is_empty() { key.clone() } else { format!("{section}.{key}") };
            match qualified.as_str() {
                "id" | "app.id" => m.id = Some(val),
                "name" | "app.name" => m.name = Some(val),
                "exe" | "app.exe" => m.exe = Some(val),
                "win_dir" | "app.win_dir" => m.win_dir = Some(val),
                "log_name" | "app.log_name" => m.log_name = Some(val),
                "computer_name" | "app.computer_name" => m.computer_name = Some(val),
                "steam_app_id" | "app.steam_app_id" => m.steam_app_id = Some(val),
                "builtin_overrides" | "runtime.builtin_overrides" => m.builtin_overrides = Some(val),
                "aot_dylibs" | "runtime.aot_dylibs" => m.aot_dylibs = Some(val),
                "preferred_base_retry" | "runtime.preferred_base_retry" => m.preferred_base_retry = parse_bool(&val),
                "v8_jitless" | "runtime.v8_jitless" => m.v8_jitless = parse_bool(&val),
                "legacy_remedy_migration" | "runtime.legacy_remedy_migration" => {
                    m.legacy_remedy_migration = parse_bool(&val)
                }
                "args" | "runtime.args" | "game_args" | "runtime.game_args" => m.args = Some(val),
                "ios.seed_file" => {
                    if let Some((src, dst)) = val.split_once("->") {
                        let s = src.trim();
                        let d = dst.trim();
                        if !s.is_empty() && !d.is_empty() {
                            m.ios_seed_files.push((s.to_string(), d.to_string()));
                        }
                    }
                }
                _ => {}
            }
        }
        m
    }

    /// Resolve a default game directory next to a manifest inside a self-contained `.app` bundle
    /// (`<exe_dir>/Game` on flat iOS bundles, `<exe_dir>/../Resources/Game` on macOS `.app` bundles).
    pub fn bundled_game_dir() -> Option<PathBuf> {
        let non_empty_dir = |p: &Path| -> bool {
            p.is_dir()
                && std::fs::read_dir(p)
                    .ok()
                    .and_then(|mut rd| rd.next())
                    .is_some()
        };
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                for rel in ["Game", "../Resources/Game"] {
                    let bundled = parent.join(rel);
                    if non_empty_dir(&bundled) {
                        return Some(bundled);
                    }
                }
            }
        }
        None
    }
}

fn strip_comment(line: &str) -> &str {
    let mut in_quotes = false;
    let mut escaped = false;
    for (i, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_quotes {
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quotes = !in_quotes;
        } else if ch == '#' && !in_quotes {
            return &line[..i];
        }
    }
    line
}

fn unquote(v: &str) -> String {
    if let Some(inner) = v.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        let mut out = String::with_capacity(inner.len());
        let mut chars = inner.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('\\') => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => out.push('\\'),
                }
            } else {
                out.push(c);
            }
        }
        out
    } else if let Some(inner) = v.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        inner.to_string()
    } else {
        v.to_string()
    }
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" => Some(true),
        "false" | "0" | "no" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_game_toml_manifest() {
        let src = r#"
# Example target manifest
[app]
id = "overwatch"
name = "Overwatch"
exe = "Overwatch.exe"
win_dir = "C:\\Games\\Overwatch"
steam_app_id = "2357570"

[runtime]
preferred_base_retry = false
v8_jitless = false # comment after value
builtin_overrides = "steam_api64.dll,-bink2w64.dll"
args = "-epicuserid=Default-Epic-User -lang \"en US\""

[ios]
seed_file = "ios-config.ini -> AppData/Local/Game/config.ini"
"#;
        let m = TargetManifest::parse(src);
        assert_eq!(m.id.as_deref(), Some("overwatch"));
        assert_eq!(m.name.as_deref(), Some("Overwatch"));
        assert_eq!(m.exe.as_deref(), Some("Overwatch.exe"));
        assert_eq!(m.win_dir.as_deref(), Some("C:\\Games\\Overwatch"));
        assert_eq!(m.steam_app_id.as_deref(), Some("2357570"));
        assert_eq!(m.preferred_base_retry, Some(false));
        assert_eq!(m.v8_jitless, Some(false));
        assert_eq!(m.builtin_overrides.as_deref(), Some("steam_api64.dll,-bink2w64.dll"));
        assert_eq!(m.split_args(), vec!["-epicuserid=Default-Epic-User", "-lang", "en US"]);
        assert_eq!(
            m.ios_seed_files,
            vec![("ios-config.ini".to_string(), "AppData/Local/Game/config.ini".to_string())]
        );
    }
}
