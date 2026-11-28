//! Store-agnostic `port-acquire-manifest.json` (schema v1) reader, secret redactor,
//! and `game.toml` argument formatter.
//!
//! Consumed by `steam-convert` / `macos_bundle` (and validated by `bundle-verify` when
//! bundled in `Contents/Resources/port-acquire-manifest.json`) so installs acquired from
//! any storefront (`epic`, `steam`, `gog`, etc.) preserve their store, app ID, deterministic
//! primary executable, build version, non-secret launch arguments, and redistributable metadata.

use crate::json_output::{parse_json, JsonValue};
use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::Path;

/// Standard filename written by `tools/port-acquire` in the game install directory
/// and mirrored into `<App>.app/Contents/Resources/port-acquire-manifest.json`.
pub const ACQUIRE_MANIFEST_FILENAME: &str = "port-acquire-manifest.json";

/// Supported schema version for `port-acquire-manifest.json`.
pub const ACQUIRE_MANIFEST_SCHEMA_VERSION: u64 = 1;

/// Exact/prefix CLI flags that can carry store authentication secrets or tokens.
/// Matches `tools/port_acquire/manifest.py::_SECRET_ARG_PREFIXES`.
const SECRET_ARG_PREFIXES: &[&str] = &[
    "-auth_password=",
    "-auth_login=",
    "-auth_type=",
    "-auth_exchangecode=",
    "-epicuserid=",
    "-epicusername=",
    "-betapassword=",
    "-branchpassword=",
    "-password=",
    "--token=",
    "--password=",
    "--secret=",
    "--sid=",
    "--auth-code=",
    "--api-key=",
    "--branch-password=",
];

/// Two-token secret flags (`<flag> <value>`) whose flag and following argument must be stripped.
/// Matches `tools/port_acquire/manifest.py::_SECRET_ARG_FLAGS_WITH_VALUE`.
const SECRET_ARG_FLAGS_WITH_VALUE: &[&str] = &[
    "-auth_password",
    "-auth_login",
    "-auth_type",
    "-auth_exchangecode",
    "-epicuserid",
    "-betapassword",
    "-branchpassword",
    "-password",
    "--token",
    "--password",
    "--secret",
    "--sid",
    "--auth-code",
    "--api-key",
    "--branch-password",
];

/// One redistributable entry recorded in `port-acquire-manifest.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AcquireRedistItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub relative_path: Option<String>,
    pub native_in_win32_rt: bool,
    pub notes: String,
}

/// Parsed schema-v1 `port-acquire-manifest.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AcquireManifest {
    pub schema_version: u64,
    pub store: String,
    pub app_id: String,
    pub title: Option<String>,
    pub version: Option<String>,
    pub executable: Option<String>,
    /// Launch arguments after defense-in-depth secret redaction.
    pub launch_args: Vec<String>,
    /// `true` if `launch_args` in the file contained any unredacted secret flags that were stripped.
    pub had_unredacted_secrets: bool,
    pub requires_eos: bool,
    pub requires_steamworks: bool,
    pub redistributables: Vec<AcquireRedistItem>,
}

impl AcquireManifest {
    /// Returns `true` when `store` identifies Steam (case-insensitive).
    pub fn is_steam_store(&self) -> bool {
        self.store.trim().eq_ignore_ascii_case("steam")
    }

    /// Returns the numeric Steam AppID if and only if this manifest belongs to Steam
    /// and `app_id` is a valid positive integer ID.
    pub fn steam_app_id(&self) -> Option<&str> {
        if self.is_steam_store() && is_numeric_steam_app_id(&self.app_id) {
            Some(self.app_id.trim())
        } else {
            None
        }
    }

    /// Format sanitized `launch_args` into a `[runtime].args` string compatible with
    /// `port_manifest::GameManifest::split_args()`.
    pub fn formatted_runtime_args(&self) -> Option<String> {
        format_game_toml_args(&self.launch_args)
    }
}

/// Returns `true` for valid positive numeric Steam AppIDs (rejecting `"custom"`, `"0"`,
/// empty strings, and non-numeric storefront IDs such as Epic hex IDs).
pub fn is_numeric_steam_app_id(raw: &str) -> bool {
    let t = raw.trim();
    !t.is_empty() && t != "0" && t.chars().all(|c| c.is_ascii_digit())
}

/// Defense-in-depth secret/token flag redaction for `launch_args`.
/// Returns `(sanitized_args, redacted_flag_names)`.
pub fn redact_launch_args(args: &[String]) -> (Vec<String>, Vec<String>) {
    let mut sanitized = Vec::with_capacity(args.len());
    let mut redacted = Vec::new();
    let mut skip_next = false;

    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        let low = arg.trim().to_ascii_lowercase();
        if let Some(prefix) = SECRET_ARG_PREFIXES
            .iter()
            .find(|p| low.starts_with(**p))
        {
            redacted.push((*prefix).trim_end_matches('=').to_string());
            continue;
        }
        if let Some(flag) = SECRET_ARG_FLAGS_WITH_VALUE.iter().find(|f| low == **f) {
            skip_next = true;
            redacted.push((*flag).to_string());
            continue;
        }
        sanitized.push(arg.clone());
    }
    (sanitized, redacted)
}

/// Normalize one CLI argument token for `GameManifest::split_args()` in `game.toml`.
///
/// `GameManifest::split_args()` splits on whitespace outside `"` quotes and strips `"`
/// delimiters. Avoid double-quoting tokens that already carry surrounding quotes or
/// embedded `"..."` segments.
fn format_game_toml_arg(arg: &str) -> String {
    let mut token = arg;
    let bytes = token.as_bytes();
    if bytes.len() >= 2
        && bytes[0] == bytes[bytes.len() - 1]
        && (bytes[0] == b'"' || bytes[0] == b'\'')
    {
        token = &token[1..token.len() - 1];
    }

    let mut in_quotes = false;
    let mut unquoted_ws = false;
    for ch in token.chars() {
        if ch == '"' {
            in_quotes = !in_quotes;
        } else if ch.is_whitespace() && !in_quotes {
            unquoted_ws = true;
        }
    }

    if !unquoted_ws {
        token.to_string()
    } else {
        let cleaned = token.replace('"', "");
        format!("\"{cleaned}\"")
    }
}

/// Format a list of launch arguments into a single `game.toml` `[runtime].args` string
/// compatible with `GameManifest::split_args()`. Returns `None` when empty.
pub fn format_game_toml_args(args: &[String]) -> Option<String> {
    let parts: Vec<String> = args
        .iter()
        .filter(|a| !a.is_empty())
        .map(|a| format_game_toml_arg(a))
        .filter(|a| !a.is_empty())
        .collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn non_empty_str(val: Option<&JsonValue>) -> Option<String> {
    val.and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Parse a schema-v1 `port-acquire-manifest.json` string.
pub fn parse_acquire_manifest(raw: &str) -> Result<AcquireManifest> {
    let root = parse_json(raw).map_err(|e| anyhow!("invalid JSON: {e}"))?;
    if !matches!(root, JsonValue::Object(_)) {
        bail!("port-acquire manifest root must be a JSON object");
    }

    let schema_version = root
        .get("schema_version")
        .and_then(|v| match v {
            JsonValue::UInt(u) => Some(*u),
            JsonValue::Int(i) if *i >= 0 => Some(*i as u64),
            _ => None,
        })
        .ok_or_else(|| anyhow!("missing or invalid `schema_version` in port-acquire manifest"))?;
    if schema_version != ACQUIRE_MANIFEST_SCHEMA_VERSION {
        bail!(
            "unsupported port-acquire manifest schema_version={schema_version} (expected {ACQUIRE_MANIFEST_SCHEMA_VERSION})"
        );
    }

    let store = non_empty_str(root.get("store"))
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();
    let app_id = non_empty_str(root.get("app_id")).unwrap_or_default();
    let title = non_empty_str(root.get("title"));
    let version = non_empty_str(root.get("version"));
    let executable = non_empty_str(root.get("executable")).map(|s| s.replace('\\', "/"));

    let raw_args: Vec<String> = root
        .get("launch_args")
        .and_then(|v| match v {
            JsonValue::Array(items) => Some(
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect(),
            ),
            _ => None,
        })
        .unwrap_or_default();
    let (launch_args, redacted_flags) = redact_launch_args(&raw_args);

    let compat = root.get("compatibility_signals");
    let requires_eos = compat
        .and_then(|c| c.get("requires_eos"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let requires_steamworks = compat
        .and_then(|c| c.get("requires_steamworks"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let mut redistributables = Vec::new();
    if let Some(JsonValue::Array(items)) = root.get("redistributables") {
        for item in items {
            let id = non_empty_str(item.get("id")).unwrap_or_default();
            let name = non_empty_str(item.get("name")).unwrap_or_else(|| id.clone());
            let category = non_empty_str(item.get("category")).unwrap_or_default();
            let relative_path = non_empty_str(item.get("relative_path"));
            let native_in_win32_rt = item
                .get("native_in_win32_rt")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let notes = non_empty_str(item.get("notes")).unwrap_or_default();
            if !name.is_empty() || !id.is_empty() {
                redistributables.push(AcquireRedistItem {
                    id,
                    name,
                    category,
                    relative_path,
                    native_in_win32_rt,
                    notes,
                });
            }
        }
    }

    Ok(AcquireManifest {
        schema_version,
        store,
        app_id,
        title,
        version,
        executable,
        launch_args,
        had_unredacted_secrets: !redacted_flags.is_empty(),
        requires_eos,
        requires_steamworks,
        redistributables,
    })
}

/// Read and parse a `port-acquire-manifest.json` file from `path`.
pub fn load_acquire_manifest(path: &Path) -> Result<AcquireManifest> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    parse_acquire_manifest(&raw)
        .with_context(|| format!("parsing {}", path.display()))
}

/// Load `<game_dir>/port-acquire-manifest.json` if present.
/// Returns `Ok(None)` when the file does not exist.
pub fn load_acquire_manifest_in_dir(game_dir: &Path) -> Result<Option<AcquireManifest>> {
    let p = game_dir.join(ACQUIRE_MANIFEST_FILENAME);
    if !p.is_file() {
        return Ok(None);
    }
    load_acquire_manifest(&p).map(Some)
}
