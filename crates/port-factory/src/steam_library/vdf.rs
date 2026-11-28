//! Valve KeyValues (`.vdf` / `.acf`) recursive parser and Steam post-install
//! (`installscript.vdf` / `_CommonRedist`) provisioning helpers.

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

// ============================================================================
// 1. Valve KeyValues (.vdf / .acf) Recursive Parser
// ============================================================================

/// A parsed node in a Valve KeyValues (`.vdf` / `.acf`) document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VdfValue {
    Str(String),
    Map(Vec<(String, VdfValue)>),
}

impl VdfValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s.as_str()),
            Self::Map(_) => None,
        }
    }

    pub fn as_map(&self) -> Option<&[(String, VdfValue)]> {
        match self {
            Self::Map(m) => Some(m.as_slice()),
            Self::Str(_) => None,
        }
    }

    /// Case-insensitive lookup in a `Map` node.
    pub fn get(&self, key: &str) -> Option<&VdfValue> {
        let m = self.as_map()?;
        m.iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    }

    /// Case-insensitive string value lookup.
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }

    /// Deep case-insensitive path lookup, e.g. `&["AppState", "name"]`.
    pub fn get_path(&self, path: &[&str]) -> Option<&VdfValue> {
        let mut cur = self;
        for seg in path {
            cur = cur.get(seg)?;
        }
        Some(cur)
    }
}

/// Tokenize and parse a Valve KeyValues (`.vdf` / `.acf`) text document.
pub fn parse_vdf(input: &str) -> VdfValue {
    let tokens = tokenize_vdf(input);
    let mut pos = 0;
    VdfValue::Map(parse_vdf_map(&tokens, &mut pos))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum VdfToken {
    LBrace,
    RBrace,
    Text(String),
}

fn tokenize_vdf(input: &str) -> Vec<VdfToken> {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let n = bytes.len();
    let mut i = 0;
    while i < n {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            while i < n && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b == b'{' {
            out.push(VdfToken::LBrace);
            i += 1;
            continue;
        }
        if b == b'}' {
            out.push(VdfToken::RBrace);
            i += 1;
            continue;
        }
        if b == b'"' {
            i += 1;
            let mut s = String::new();
            while i < n {
                if bytes[i] == b'\\' && i + 1 < n {
                    let esc = bytes[i + 1];
                    match esc {
                        b'n' => s.push('\n'),
                        b't' => s.push('\t'),
                        b'\\' => s.push('\\'),
                        b'"' => s.push('"'),
                        other => s.push(other as char),
                    }
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                let ch = input[i..].chars().next().unwrap_or(' ');
                s.push(ch);
                i += ch.len_utf8();
            }
            out.push(VdfToken::Text(s));
            continue;
        }
        // Unquoted token
        let start = i;
        while i < n
            && !bytes[i].is_ascii_whitespace()
            && bytes[i] != b'{'
            && bytes[i] != b'}'
            && bytes[i] != b'"'
        {
            i += 1;
        }
        if i > start {
            out.push(VdfToken::Text(input[start..i].to_string()));
        } else {
            i += 1;
        }
    }
    out
}

fn parse_vdf_map(tokens: &[VdfToken], pos: &mut usize) -> Vec<(String, VdfValue)> {
    let mut entries = Vec::new();
    while *pos < tokens.len() {
        match &tokens[*pos] {
            VdfToken::RBrace => {
                *pos += 1;
                break;
            }
            VdfToken::LBrace => {
                *pos += 1;
                let _ = parse_vdf_map(tokens, pos);
            }
            VdfToken::Text(key) => {
                let key = key.clone();
                *pos += 1;
                if *pos >= tokens.len() {
                    break;
                }
                match &tokens[*pos] {
                    VdfToken::LBrace => {
                        *pos += 1;
                        let child = parse_vdf_map(tokens, pos);
                        entries.push((key, VdfValue::Map(child)));
                    }
                    VdfToken::Text(val) => {
                        let val = val.clone();
                        *pos += 1;
                        entries.push((key, VdfValue::Str(val)));
                    }
                    VdfToken::RBrace => {
                        break;
                    }
                }
            }
        }
    }
    entries
}

// ============================================================================
// 2. Post-Download / Post-Install (`installscript.vdf` & `_CommonRedist`) Handler
// ============================================================================

/// A registry key extracted from a Steam `installscript.vdf` post-install file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteamRegistryEntry {
    pub key_path: String,
    pub value_name: String,
    pub value_type: String,
    pub value_data: String,
}

/// Summary of post-download / post-install requirements (`installscript.vdf` & `_CommonRedist`).
#[derive(Debug, Clone, Default)]
pub struct PostInstallSummary {
    pub installscript_files: Vec<PathBuf>,
    pub common_redist_dirs: Vec<PathBuf>,
    pub bypassable_redistributables: Vec<String>,
    pub registry_entries: Vec<SteamRegistryEntry>,
    pub run_processes: Vec<String>,
}

/// Scan a game directory for `installscript.vdf` / `*install*.vdf` and `_CommonRedist`
/// post-download installers, extracting all required virtual registry keys and
/// identifying redistributables natively satisfied by `win32-rt`.
pub fn scan_post_install(game_dir: &Path) -> PostInstallSummary {
    let mut summary = PostInstallSummary::default();
    if !game_dir.is_dir() {
        return summary;
    }

    let mut stack = vec![(game_dir.to_path_buf(), 0usize)];
    while let Some((dir, depth)) = stack.pop() {
        if depth > 3 {
            continue;
        }
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(ft) = entry.file_type() else {
                continue;
            };
            if ft.is_symlink() {
                continue;
            }
            let path = entry.path();
            let fname = entry.file_name().to_string_lossy().into_owned();
            let lower = fname.to_ascii_lowercase();
            if ft.is_dir() {
                if lower == "_commonredist" || lower == "redist" || lower == "support" {
                    summary.common_redist_dirs.push(path.clone());
                }
                if depth < 3 && !lower.starts_with('.') && lower != "data" && lower != "data_pack2"
                {
                    stack.push((path, depth + 1));
                }
            } else if lower.ends_with(".vdf") && lower.contains("install") {
                if let Ok(raw) = fs::read_to_string(&path) {
                    let parsed = parse_installscript_vdf(&raw);
                    summary.installscript_files.push(path);
                    summary.registry_entries.extend(parsed.registry_entries);
                    summary.run_processes.extend(parsed.run_processes);
                    for b in parsed.bypassable_redistributables {
                        if !summary.bypassable_redistributables.contains(&b) {
                            summary.bypassable_redistributables.push(b);
                        }
                    }
                }
            } else if (lower.contains("vc_redist")
                || lower.contains("vcredist")
                || lower.contains("dxsetup")
                || lower.contains("dotnet")
                || lower.contains("ndp4")
                || lower.contains("oalinst")
                || lower.contains("physx"))
                && lower.ends_with(".exe")
            {
                let label = format!("{fname} (Natively provided by win32-rt runtime)");
                if !summary.bypassable_redistributables.contains(&label) {
                    summary.bypassable_redistributables.push(label);
                }
            }
        }
    }

    // Also incorporate redistributable metadata from `port-acquire-manifest.json` when present
    // (any storefront: Epic, Steam, GOG, etc.).
    if let Ok(Some(acq)) = super::acquire_manifest::load_acquire_manifest_in_dir(game_dir) {
        for r in &acq.redistributables {
            if r.native_in_win32_rt {
                let label = format!("{} (Natively provided by win32-rt runtime)", r.name);
                if !summary.bypassable_redistributables.contains(&label) {
                    summary.bypassable_redistributables.push(label);
                }
            }
        }
    }

    summary
}

/// Parse a Valve `installscript.vdf` file to extract `Registry` entries and `Run Process` steps.
pub fn parse_installscript_vdf(raw: &str) -> PostInstallSummary {
    let vdf = parse_vdf(raw);
    let mut summary = PostInstallSummary::default();

    let Some(root_entries) = vdf.as_map() else {
        return summary;
    };
    for (_, root_val) in root_entries {
        let Some(sections) = root_val.as_map() else {
            continue;
        };
        for (sec_name, sec_val) in sections {
            if sec_name.eq_ignore_ascii_case("Registry") {
                if let Some(reg_keys) = sec_val.as_map() {
                    for (key_path, key_body) in reg_keys {
                        collect_vdf_registry_entries(
                            key_path,
                            key_body,
                            &mut summary.registry_entries,
                        );
                    }
                }
            } else if sec_name.eq_ignore_ascii_case("Run Process") {
                if let Some(procs) = sec_val.as_map() {
                    for (proc_name, proc_body) in procs {
                        let process = proc_body
                            .get_str("process 1")
                            .or_else(|| proc_body.get_str("process"))
                            .unwrap_or(proc_name);
                        let args = proc_body
                            .get_str("command 1")
                            .or_else(|| proc_body.get_str("command"))
                            .unwrap_or("");
                        let desc = if args.is_empty() {
                            format!("{proc_name}: {process}")
                        } else {
                            format!("{proc_name}: {process} {args}")
                        };
                        summary.run_processes.push(desc);

                        let lower = process.to_ascii_lowercase();
                        if lower.contains("vcredist")
                            || lower.contains("vc_redist")
                            || lower.contains("dxsetup")
                            || lower.contains("dotnet")
                            || lower.contains("physx")
                        {
                            summary.bypassable_redistributables.push(format!(
                                "{proc_name} ({process} -> bypassed, native in win32-rt)"
                            ));
                        }
                    }
                }
            }
        }
    }
    summary
}

fn collect_vdf_registry_entries(
    key_path: &str,
    key_body: &VdfValue,
    out: &mut Vec<SteamRegistryEntry>,
) {
    let Some(items) = key_body.as_map() else {
        return;
    };
    let normalized_key = normalize_vdf_reg_path(key_path);
    for (type_or_sub, val_node) in items {
        if type_or_sub.eq_ignore_ascii_case("string") {
            if let Some(kv_list) = val_node.as_map() {
                for (k, v) in kv_list {
                    if let Some(s) = v.as_str() {
                        out.push(SteamRegistryEntry {
                            key_path: normalized_key.clone(),
                            value_name: k.clone(),
                            value_type: "REG_SZ".to_string(),
                            value_data: s.to_string(),
                        });
                    }
                }
            }
        } else if type_or_sub.eq_ignore_ascii_case("dword") {
            if let Some(kv_list) = val_node.as_map() {
                for (k, v) in kv_list {
                    if let Some(s) = v.as_str() {
                        out.push(SteamRegistryEntry {
                            key_path: normalized_key.clone(),
                            value_name: k.clone(),
                            value_type: "REG_DWORD".to_string(),
                            value_data: s.to_string(),
                        });
                    }
                }
            }
        }
    }
}

fn normalize_vdf_reg_path(raw: &str) -> String {
    let mut s = raw.trim().replace('/', "\\");
    for (long, short) in [
        ("HKEY_LOCAL_MACHINE\\", "hklm\\"),
        ("HKEY_CURRENT_USER\\", "hkcu\\"),
        ("HKEY_CLASSES_ROOT\\", "hkcr\\"),
        ("HKEY_USERS\\", "hku\\"),
        ("HKEY_CURRENT_CONFIG\\", "hkcc\\"),
    ] {
        if s.len() >= long.len() && s[..long.len()].eq_ignore_ascii_case(long) {
            s = format!("{short}{}", &s[long.len()..]);
            break;
        }
    }
    s.to_ascii_lowercase()
}

/// Apply post-download / post-install steps for a Steam game directory:
/// 1. Writes `steam_appid.txt` so `steam_api64.dll` knows the exact AppID.
/// 2. Extracts all `installscript.vdf` registry entries + standard Steam/Redistributable
///    completion flags (`HasRunKey`) into `<out_dir>/steam_registry.tsv`.
pub fn apply_post_install(
    game_dir: &Path,
    app_id: &str,
    out_dir: &Path,
) -> Result<PostInstallSummary> {
    fs::create_dir_all(out_dir)?;
    let mut summary = scan_post_install(game_dir);

    // 1. Write steam_appid.txt if numeric AppID
    if app_id.chars().all(|c| c.is_ascii_digit()) && !app_id.is_empty() && app_id != "0" {
        let appid_file = out_dir.join("steam_appid.txt");
        let _ = fs::write(&appid_file, format!("{app_id}\n"));
        if game_dir.is_dir() {
            let _ = fs::write(game_dir.join("steam_appid.txt"), format!("{app_id}\n"));
        }
        // Also seed Steam's HasRunKey in registry so Steam never re-runs _CommonRedist installers
        summary.registry_entries.push(SteamRegistryEntry {
            key_path: format!("hkcu\\software\\valve\\steam\\apps\\{app_id}"),
            value_name: "Installed".to_string(),
            value_type: "REG_DWORD".to_string(),
            value_data: "1".to_string(),
        });
    }

    // 2. Write steam_registry.tsv for `win32-rt::advapi32::Registry`
    let reg_tsv_path = out_dir.join("steam_registry.tsv");
    let mut f = fs::File::create(&reg_tsv_path)
        .with_context(|| format!("creating {}", reg_tsv_path.display()))?;
    writeln!(
        f,
        "# Virtual Registry manifest generated by port-factory steam-postinstall"
    )?;
    writeln!(f, "key_path\tvalue_name\tvalue_type\tvalue_data")?;
    for entry in &summary.registry_entries {
        writeln!(
            f,
            "{}\t{}\t{}\t{}",
            entry.key_path, entry.value_name, entry.value_type, entry.value_data
        )?;
    }

    Ok(summary)
}
