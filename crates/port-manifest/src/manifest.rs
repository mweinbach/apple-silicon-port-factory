//! Declarative per-game target manifest (`game.toml`) parser, serializer, and discovery helpers.
//!
//! Zero external TOML crate dependencies so `port-manifest` stays lean and builds
//! identically for macOS, iOS, iOS Simulator, and Rosetta targets.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::dll_map::{DllRoute, DllRoutingTable};

/// Version and routing override for one middleware family in `game.toml`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MiddlewareSelection {
    pub version: Option<String>,
    pub mode: Option<DllRoute>,
}

/// Optional `[storage]` optimization settings in `game.toml`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StorageManifestConfig {
    pub keep_dlls: Vec<String>,
    pub archive_repacker: Option<String>,
    pub archive_dir: Option<String>,
}

/// Declarative per-game manifest loaded from `games/<id>/game.toml` or an `.app` bundle.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GameManifest {
    // [app]
    pub id: Option<String>,
    pub name: Option<String>,
    pub exe: Option<String>,
    pub win_dir: Option<String>,
    pub log_name: Option<String>,
    pub log_dir: Option<String>,
    pub computer_name: Option<String>,
    pub steam_app_id: Option<String>,

    // [runtime]
    pub builtin_overrides: Option<String>,
    pub aot_dylibs: Option<String>,
    pub aot_dir: Option<String>,
    pub preferred_base_retry: Option<bool>,
    pub v8_jitless: Option<bool>,
    pub legacy_remedy_migration: Option<bool>,
    pub args: Option<String>,

    // [dlls]
    pub dlls: BTreeMap<String, DllRoute>,

    // [middleware] & [middleware.<family>]
    pub middleware: BTreeMap<String, MiddlewareSelection>,

    // [storage]
    pub storage: StorageManifestConfig,

    // [ios]
    pub ios_seed_files: Vec<(String, String)>,
}

impl GameManifest {
    /// Load and parse a `game.toml` file from disk.
    pub fn load_file(path: &Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(Self::parse(&text))
    }

    /// Discover a manifest path from:
    /// 1. `explicit` (`--manifest`),
    /// 2. `$PORT_MANIFEST`,
    /// 3. `<exe_dir>/game.toml` (flat iOS `.app`),
    /// 4. `<exe_dir>/../Resources/game.toml` (macOS `.app`).
    pub fn discover_path(explicit: Option<&Path>) -> Option<PathBuf> {
        if let Some(p) = explicit {
            return p.is_file().then(|| p.to_path_buf());
        }
        if let Some(p) = std::env::var_os("PORT_MANIFEST").filter(|v| !v.is_empty()) {
            let pb = PathBuf::from(p);
            if pb.is_file() {
                return Some(pb);
            }
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                for rel in ["game.toml", "../Resources/game.toml"] {
                    let bundled = dir.join(rel);
                    if bundled.is_file() {
                        return Some(bundled);
                    }
                }
            }
        }
        None
    }

    /// Resolve a default non-empty `Game` directory inside a self-contained `.app` bundle
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

    /// Build the effective [`DllRoutingTable`] for this manifest.
    pub fn routing_table(
        &self,
        plugin_defaults: &[(&str, &str)],
        env_override: Option<&str>,
    ) -> DllRoutingTable {
        DllRoutingTable::from_manifest(self, plugin_defaults, env_override)
    }

    /// Return the normalized set of DLLs that must be kept during storage optimization,
    /// merging `[storage].keep_dlls` with all `Guest` / `GuestOnly` DLLs from routing.
    pub fn effective_keep_dlls(&self) -> BTreeSet<String> {
        let mut out = self.routing_table(&[], None).guest_protected_dlls();
        for dll in &self.storage.keep_dlls {
            let trimmed = dll.trim();
            if !trimmed.is_empty() {
                out.insert(pe_image::normalize_dll_name(trimmed));
            }
        }
        out
    }

    /// Query the declared version string for a middleware family, if any.
    pub fn middleware_version(&self, family: &str) -> Option<&str> {
        let key = family.trim().to_ascii_lowercase();
        self.middleware
            .get(&key)
            .and_then(|sel| sel.version.as_deref())
    }

    /// Dependency-free parser for `game.toml` manifests.
    ///
    /// Supports:
    /// - `#` comments outside single/double quotes
    /// - Escape sequences in double-quoted strings and literal single-quoted strings
    /// - Quoted or unquoted keys (e.g. `"physx3_x64.dll" = "guest"` or `nrd.dll = "guest"`)
    /// - Single-line and multi-line `[ ... ]` string arrays (e.g. `[storage].keep_dlls`)
    /// - Dotted `[middleware.<family>]` sections and compact `[middleware]` entries
    /// - Legacy flat (unsectioned) keys for backwards compatibility
    pub fn parse(text: &str) -> Self {
        let mut m = Self::default();
        let mut section = String::new();
        let mut pending_array: Option<(String, String, String)> = None;

        for raw_line in text.lines() {
            let line = strip_comment(raw_line).trim();
            if line.is_empty() {
                continue;
            }

            if let Some((arr_sec, arr_key, mut buf)) = pending_array.take() {
                buf.push(' ');
                buf.push_str(line);
                if has_unquoted_char(&buf, ']') {
                    m.apply_entry(&arr_sec, &arr_key, &buf);
                } else {
                    pending_array = Some((arr_sec, arr_key, buf));
                }
                continue;
            }

            if let Some(sec) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                section = normalize_section(sec);
                continue;
            }

            let Some((raw_key, raw_val)) = split_key_value(line) else {
                continue;
            };
            let key = unquote(raw_key.trim());
            let val_trimmed = raw_val.trim();

            if val_trimmed.starts_with('[') && !has_unquoted_char(val_trimmed, ']') {
                pending_array = Some((section.clone(), key, val_trimmed.to_string()));
                continue;
            }

            m.apply_entry(&section, &key, val_trimmed);
        }

        if let Some((arr_sec, arr_key, buf)) = pending_array {
            m.apply_entry(&arr_sec, &arr_key, &buf);
        }

        m
    }

    fn apply_entry(&mut self, section: &str, raw_key: &str, raw_val: &str) {
        if section == "dlls" {
            let dll = pe_image::normalize_dll_name(raw_key);
            let val = unquote(raw_val);
            if let Some(route) = DllRoute::parse(&val) {
                self.dlls.insert(dll, route);
            }
            return;
        }

        if let Some(family) = section.strip_prefix("middleware.") {
            let fam_key = family.trim().to_ascii_lowercase();
            if fam_key.is_empty() {
                return;
            }
            let key_lower = raw_key.trim().to_ascii_lowercase();
            let val = unquote(raw_val);
            let entry = self.middleware.entry(fam_key).or_default();
            match key_lower.as_str() {
                "version" => {
                    if !val.is_empty() {
                        entry.version = Some(val);
                    }
                }
                "mode" => {
                    if let Some(route) = DllRoute::parse(&val) {
                        entry.mode = Some(route);
                    }
                }
                _ => {}
            }
            return;
        }

        if section == "middleware" {
            let key_lower = raw_key.trim().to_ascii_lowercase();
            let val = unquote(raw_val);
            if let Some((fam, field)) = key_lower.split_once('.') {
                let entry = self.middleware.entry(fam.trim().to_string()).or_default();
                match field.trim() {
                    "version" if !val.is_empty() => entry.version = Some(val),
                    "mode" => {
                        if let Some(route) = DllRoute::parse(&val) {
                            entry.mode = Some(route);
                        }
                    }
                    _ => {}
                }
            } else if !key_lower.is_empty() {
                let entry = self.middleware.entry(key_lower).or_default();
                apply_compact_middleware_value(entry, &val);
            }
            return;
        }

        let key = raw_key.trim().to_ascii_lowercase();
        let qualified = if section.is_empty() {
            key
        } else {
            format!("{section}.{key}")
        };

        if qualified == "storage.keep_dlls" {
            if raw_val.starts_with('[') {
                self.storage.keep_dlls = parse_string_array(raw_val);
            } else {
                let val = unquote(raw_val);
                self.storage.keep_dlls = val
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            return;
        }

        let val = unquote(raw_val);
        match qualified.as_str() {
            "id" | "app.id" => self.id = Some(val),
            "name" | "app.name" => self.name = Some(val),
            "exe" | "app.exe" => self.exe = Some(val),
            "win_dir" | "app.win_dir" => self.win_dir = Some(val),
            "log_name" | "app.log_name" => self.log_name = Some(val),
            "log_dir" | "app.log_dir" => self.log_dir = Some(val),
            "computer_name" | "app.computer_name" => self.computer_name = Some(val),
            "steam_app_id" | "app.steam_app_id" => self.steam_app_id = Some(val),
            "builtin_overrides" | "runtime.builtin_overrides" => self.builtin_overrides = Some(val),
            "aot_dylibs" | "runtime.aot_dylibs" => self.aot_dylibs = Some(val),
            "aot_dir" | "runtime.aot_dir" => self.aot_dir = Some(val),
            "preferred_base_retry" | "runtime.preferred_base_retry" => {
                self.preferred_base_retry = parse_bool(&val)
            }
            "v8_jitless" | "runtime.v8_jitless" => self.v8_jitless = parse_bool(&val),
            "legacy_remedy_migration" | "runtime.legacy_remedy_migration" => {
                self.legacy_remedy_migration = parse_bool(&val)
            }
            "args" | "runtime.args" | "game_args" | "runtime.game_args" => self.args = Some(val),
            "storage.archive_repacker" => self.storage.archive_repacker = Some(val),
            "storage.archive_dir" => self.storage.archive_dir = Some(val),
            "ios.seed_file" => {
                if let Some((src, dst)) = val.split_once("->") {
                    let s = src.trim();
                    let d = dst.trim();
                    if !s.is_empty() && !d.is_empty() {
                        self.ios_seed_files.push((s.to_string(), d.to_string()));
                    }
                }
            }
            _ => {}
        }
    }

    /// Serialize this manifest into clean, deterministic TOML that round-trips through [`Self::parse`].
    pub fn to_toml(&self) -> String {
        let mut out = String::new();
        let mut wrote_section = false;

        let mut start_section = |out: &mut String, header: &str| {
            if wrote_section {
                out.push('\n');
            }
            out.push('[');
            out.push_str(header);
            out.push_str("]\n");
            wrote_section = true;
        };

        let has_app = self.id.is_some()
            || self.name.is_some()
            || self.exe.is_some()
            || self.win_dir.is_some()
            || self.log_name.is_some()
            || self.log_dir.is_some()
            || self.computer_name.is_some()
            || self.steam_app_id.is_some();
        if has_app {
            start_section(&mut out, "app");
            push_opt_str(&mut out, "id", self.id.as_deref());
            push_opt_str(&mut out, "name", self.name.as_deref());
            push_opt_str(&mut out, "exe", self.exe.as_deref());
            push_opt_str(&mut out, "win_dir", self.win_dir.as_deref());
            push_opt_str(&mut out, "log_name", self.log_name.as_deref());
            push_opt_str(&mut out, "log_dir", self.log_dir.as_deref());
            push_opt_str(&mut out, "computer_name", self.computer_name.as_deref());
            push_opt_str(&mut out, "steam_app_id", self.steam_app_id.as_deref());
        }

        let has_runtime = self.preferred_base_retry.is_some()
            || self.v8_jitless.is_some()
            || self.legacy_remedy_migration.is_some()
            || self.builtin_overrides.is_some()
            || self.aot_dylibs.is_some()
            || self.aot_dir.is_some()
            || self.args.is_some();
        if has_runtime {
            start_section(&mut out, "runtime");
            push_opt_bool(&mut out, "preferred_base_retry", self.preferred_base_retry);
            push_opt_bool(&mut out, "v8_jitless", self.v8_jitless);
            push_opt_bool(
                &mut out,
                "legacy_remedy_migration",
                self.legacy_remedy_migration,
            );
            push_opt_str(
                &mut out,
                "builtin_overrides",
                self.builtin_overrides.as_deref(),
            );
            push_opt_str(&mut out, "aot_dylibs", self.aot_dylibs.as_deref());
            push_opt_str(&mut out, "aot_dir", self.aot_dir.as_deref());
            push_opt_str(&mut out, "args", self.args.as_deref());
        }

        for (family, sel) in &self.middleware {
            if sel.version.is_none() && sel.mode.is_none() {
                continue;
            }
            let header = format!("middleware.{family}");
            start_section(&mut out, &header);
            push_opt_str(&mut out, "version", sel.version.as_deref());
            if let Some(route) = &sel.mode {
                let v = route.to_toml_value();
                push_opt_str(&mut out, "mode", Some(&v));
            }
        }

        if !self.dlls.is_empty() {
            start_section(&mut out, "dlls");
            for (dll, route) in &self.dlls {
                out.push_str(&format!(
                    "\"{}\" = \"{}\"\n",
                    escape_toml_str(dll),
                    escape_toml_str(&route.to_toml_value())
                ));
            }
        }

        let has_storage = self.storage.archive_repacker.is_some()
            || self.storage.archive_dir.is_some()
            || !self.storage.keep_dlls.is_empty();
        if has_storage {
            start_section(&mut out, "storage");
            push_opt_str(
                &mut out,
                "archive_repacker",
                self.storage.archive_repacker.as_deref(),
            );
            push_opt_str(&mut out, "archive_dir", self.storage.archive_dir.as_deref());
            if !self.storage.keep_dlls.is_empty() {
                out.push_str("keep_dlls = [");
                for (idx, dll) in self.storage.keep_dlls.iter().enumerate() {
                    if idx > 0 {
                        out.push_str(", ");
                    }
                    out.push('"');
                    out.push_str(&escape_toml_str(dll));
                    out.push('"');
                }
                out.push_str("]\n");
            }
        }

        if !self.ios_seed_files.is_empty() {
            start_section(&mut out, "ios");
            for (src, dst) in &self.ios_seed_files {
                let combined = format!("{src} -> {dst}");
                push_opt_str(&mut out, "seed_file", Some(&combined));
            }
        }

        out
    }
}

fn apply_compact_middleware_value(entry: &mut MiddlewareSelection, val: &str) {
    let trimmed = val.trim();
    if trimmed.is_empty() {
        return;
    }
    if let Some(route) = DllRoute::parse(trimmed) {
        entry.mode = Some(route);
        return;
    }
    if let Some((ver, mode_str)) = trimmed.split_once(':') {
        let ver = ver.trim();
        let mode_str = mode_str.trim();
        if !ver.is_empty() {
            entry.version = Some(ver.to_string());
        }
        if let Some(route) = DllRoute::parse(mode_str) {
            entry.mode = Some(route);
        }
    } else {
        entry.version = Some(trimmed.to_string());
    }
}

fn normalize_section(sec: &str) -> String {
    sec.split('.')
        .map(|part| unquote(part.trim()).to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(".")
}

fn strip_comment(line: &str) -> &str {
    let mut in_double = false;
    let mut in_single = false;
    let mut escaped = false;
    for (i, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_double {
            escaped = true;
            continue;
        }
        if ch == '"' && !in_single {
            in_double = !in_double;
        } else if ch == '\'' && !in_double {
            in_single = !in_single;
        } else if ch == '#' && !in_double && !in_single {
            return &line[..i];
        }
    }
    line
}

fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let mut in_double = false;
    let mut in_single = false;
    let mut escaped = false;
    for (i, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_double {
            escaped = true;
            continue;
        }
        if ch == '"' && !in_single {
            in_double = !in_double;
        } else if ch == '\'' && !in_double {
            in_single = !in_single;
        } else if ch == '=' && !in_double && !in_single {
            return Some((&line[..i], &line[i + 1..]));
        }
    }
    None
}

fn has_unquoted_char(s: &str, needle: char) -> bool {
    let mut in_double = false;
    let mut in_single = false;
    let mut escaped = false;
    for ch in s.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_double {
            escaped = true;
            continue;
        }
        if ch == '"' && !in_single {
            in_double = !in_double;
        } else if ch == '\'' && !in_double {
            in_single = !in_single;
        } else if ch == needle && !in_double && !in_single {
            return true;
        }
    }
    false
}

fn parse_string_array(raw: &str) -> Vec<String> {
    let inner = raw
        .trim()
        .strip_prefix('[')
        .unwrap_or(raw)
        .trim();
    // Strip at the first unquoted `]`.
    let mut end_idx = inner.len();
    {
        let mut in_double = false;
        let mut in_single = false;
        let mut escaped = false;
        for (i, ch) in inner.char_indices() {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' && in_double {
                escaped = true;
                continue;
            }
            if ch == '"' && !in_single {
                in_double = !in_double;
            } else if ch == '\'' && !in_double {
                in_single = !in_single;
            } else if ch == ']' && !in_double && !in_single {
                end_idx = i;
                break;
            }
        }
    }
    let body = &inner[..end_idx];

    let mut items = Vec::new();
    let mut start = 0usize;
    let mut in_double = false;
    let mut in_single = false;
    let mut escaped = false;

    for (i, ch) in body.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' && in_double {
            escaped = true;
            continue;
        }
        if ch == '"' && !in_single {
            in_double = !in_double;
        } else if ch == '\'' && !in_double {
            in_single = !in_single;
        } else if ch == ',' && !in_double && !in_single {
            let tok = body[start..i].trim();
            if !tok.is_empty() {
                items.push(unquote(tok));
            }
            start = i + 1;
        }
    }
    let tail = body[start..].trim();
    if !tail.is_empty() {
        items.push(unquote(tail));
    }
    items
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
                    Some('r') => out.push('\r'),
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

fn escape_toml_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn push_opt_str(out: &mut String, key: &str, val: Option<&str>) {
    if let Some(v) = val {
        out.push_str(&format!("{key} = \"{}\"\n", escape_toml_str(v)));
    }
}

fn push_opt_bool(out: &mut String, key: &str, val: Option<bool>) {
    if let Some(v) = val {
        out.push_str(&format!("{key} = {v}\n"));
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

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn parses_repo_game_manifests() {
        let root = workspace_root();

        let aw2 = GameManifest::load_file(&root.join("games/alan-wake-2/game.toml")).unwrap();
        assert_eq!(aw2.id.as_deref(), Some("alan-wake-2"));
        assert_eq!(aw2.name.as_deref(), Some("Alan Wake 2"));
        assert_eq!(aw2.exe.as_deref(), Some("AlanWake2.exe"));
        assert_eq!(aw2.win_dir.as_deref(), Some("C:\\Games\\AlanWake2"));
        assert_eq!(aw2.log_name.as_deref(), Some("AlanWake2.log"));
        assert_eq!(aw2.computer_name.as_deref(), Some("AW2PORT"));
        assert_eq!(aw2.preferred_base_retry, Some(true));
        assert_eq!(aw2.v8_jitless, Some(true));
        assert_eq!(aw2.legacy_remedy_migration, Some(true));
        assert_eq!(aw2.split_args(), vec!["-epicuserid=Default-Epic-User"]);
        assert_eq!(aw2.storage.archive_repacker.as_deref(), Some("aw2-data"));
        assert_eq!(aw2.storage.archive_dir.as_deref(), Some("data_pack2"));
        assert!(aw2.storage.keep_dlls.iter().any(|d| d == "NRD.dll"));
        assert!(aw2.storage.keep_dlls.iter().any(|d| d == "PhysX3_x64.dll"));
        assert_eq!(
            aw2.ios_seed_files,
            vec![(
                "ios-renderer.ini".to_string(),
                "AppData/Local/Remedy/AlanWake2/renderer.ini".to_string()
            )]
        );
        let keep = aw2.effective_keep_dlls();
        assert!(keep.contains("nrd.dll"));
        assert!(keep.contains("physx3_x64.dll"));
        assert!(keep.contains("pxfoundation_x64.dll"));

        let ow = GameManifest::load_file(&root.join("games/overwatch/game.toml")).unwrap();
        assert_eq!(ow.id.as_deref(), Some("overwatch"));
        assert_eq!(ow.exe.as_deref(), Some("Overwatch.exe"));
        assert_eq!(ow.steam_app_id.as_deref(), Some("2357570"));
        assert_eq!(ow.preferred_base_retry, Some(false));
        assert_eq!(ow.v8_jitless, Some(false));
        assert_eq!(ow.legacy_remedy_migration, Some(false));

        let tpl = GameManifest::load_file(&root.join("games/_template/game.toml")).unwrap();
        assert_eq!(tpl.id.as_deref(), Some("my-game"));
        assert_eq!(tpl.name.as_deref(), Some("My Game"));
        assert_eq!(tpl.exe.as_deref(), Some("MyGame.exe"));
        assert_eq!(tpl.win_dir.as_deref(), Some("C:\\Games\\MyGame"));
    }

    #[test]
    fn parses_multiline_arrays_and_compact_middleware() {
        let src = r#"
[app]
id = "sample"
name = "Sample Game"

[middleware]
physx = "4.1:guest"
fmod = "1.10:builtin:fmod.dll"
nrd = "guest-only"
oodle = "8"

[dlls]
"custom_foo.dll" = "alias:bar.dll"
nrd.dll = "guest"

[storage]
keep_dlls = [
    "NRD.dll", # inline comment
    "PhysX_64.dll",
    'oo2core_8_win64.dll',
]
"#;
        let m = GameManifest::parse(src);
        assert_eq!(m.middleware_version("physx"), Some("4.1"));
        assert_eq!(m.middleware["physx"].mode, Some(DllRoute::Guest));
        assert_eq!(m.middleware_version("fmod"), Some("1.10"));
        assert_eq!(
            m.middleware["fmod"].mode,
            Some(DllRoute::Builtin {
                provider: Some("fmod.dll".into())
            })
        );
        assert_eq!(m.middleware_version("nrd"), None);
        assert_eq!(m.middleware["nrd"].mode, Some(DllRoute::GuestOnly));
        assert_eq!(m.middleware_version("oodle"), Some("8"));
        assert_eq!(m.middleware["oodle"].mode, None);

        assert_eq!(
            m.dlls.get("custom_foo.dll"),
            Some(&DllRoute::Alias("bar.dll".into()))
        );
        assert_eq!(m.dlls.get("nrd.dll"), Some(&DllRoute::Guest));
        assert_eq!(
            m.storage.keep_dlls,
            vec!["NRD.dll", "PhysX_64.dll", "oo2core_8_win64.dll"]
        );

        let effective = m.effective_keep_dlls();
        assert!(effective.contains("physx_64.dll"));
        assert!(effective.contains("physxcommon_64.dll"));
        assert!(effective.contains("physxcooking_64.dll"));
        assert!(effective.contains("physxfoundation_64.dll"));
        assert!(effective.contains("nrd.dll"));
        assert!(effective.contains("oo2core_8_win64.dll"));
    }

    #[test]
    fn to_toml_roundtrip() {
        let mut original = GameManifest {
            id: Some("roundtrip-test".into()),
            name: Some("Roundtrip \"Game\"".into()),
            exe: Some("Game.exe".into()),
            win_dir: Some("C:\\Games\\Roundtrip".into()),
            log_name: Some("Game.log".into()),
            log_dir: Some("Logs/Game".into()),
            computer_name: Some("WIN64PORT".into()),
            steam_app_id: Some("999999".into()),
            builtin_overrides: Some("-bink2w64.dll,steam_api64.dll".into()),
            aot_dylibs: Some("foo.dylib".into()),
            aot_dir: Some("aot-cache".into()),
            preferred_base_retry: Some(true),
            v8_jitless: Some(false),
            legacy_remedy_migration: Some(false),
            args: Some("-lang \"en US\" -flag=1".into()),
            dlls: BTreeMap::new(),
            middleware: BTreeMap::new(),
            storage: StorageManifestConfig {
                keep_dlls: vec!["NRD.dll".into(), "PhysX3_x64.dll".into()],
                archive_repacker: Some("custom-repacker".into()),
                archive_dir: Some("paks".into()),
            },
            ios_seed_files: vec![(
                "ios.ini".into(),
                "AppData/Local/Test/ios.ini".into(),
            )],
        };
        original.dlls.insert("nrd.dll".into(), DllRoute::Guest);
        original.dlls.insert(
            "old_xinput.dll".into(),
            DllRoute::Alias("xinput1_4.dll".into()),
        );
        original.dlls.insert(
            "custom_vid.dll".into(),
            DllRoute::Builtin {
                provider: Some("bink2w64.dll".into()),
            },
        );
        original
            .dlls
            .insert("telemetry.dll".into(), DllRoute::Disabled);
        original.middleware.insert(
            "physx".into(),
            MiddlewareSelection {
                version: Some("3.4".into()),
                mode: Some(DllRoute::GuestOnly),
            },
        );
        original.middleware.insert(
            "fmod".into(),
            MiddlewareSelection {
                version: Some("2.02".into()),
                mode: Some(DllRoute::Guest),
            },
        );

        let serialized = original.to_toml();
        let reparsed = GameManifest::parse(&serialized);
        assert_eq!(reparsed, original);
        assert_eq!(reparsed.split_args(), vec!["-lang", "en US", "-flag=1"]);
    }
}
