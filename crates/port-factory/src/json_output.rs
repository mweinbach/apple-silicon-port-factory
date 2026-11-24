//! Shared versioned JSON output types, RFC 8259 serializer/parser, and canonical
//! exit-code table for all `port-factory` and `port-loader` CLI commands.
//!
//! Zero external dependencies: designed to be reused directly by every `port-factory`
//! audit/verify command, `port-loader` (`port-runner`), and higher-level aggregators
//! such as `audit-game`.
//!
//! Every `--json` command emits a single top-level [`JsonEnvelope`]:
//! ```json
//! {
//!   "schema_version": 1,
//!   "command": "<canonical-command-name>",
//!   "ok": true,
//!   "exit_code": 0,
//!   "data": { ... },
//!   "error": null
//! }
//! ```

use crate::anticheat::{lookup_ssn_name, AntiCheatAuditReport, AntiCheatVerdict};
use crate::diag_toolkit::RuntimeHazardReport;
use crate::doctor::{DoctorDbReport, TsvCheckItem, TsvSyncStatus};
use crate::dotnet::{DotNetAuditSummary, DotNetBinaryReport};
use crate::game_profiles::GameProfileAudit;
use crate::gpu_shaders::AppCapabilityProfile;
use crate::ios_bundle::IosBundleVerifyReport;
use crate::ops_db::{CatalogStats, OpRecord};
use crate::scanner::BinaryProfile;
use crate::steam_library::{
    BundleCheckStatus, BundleVerifyReport, ExternalVolumeInfo, PackagedBundleInfo,
    SteamLibraryScanReport,
};
use crate::win32_db::{Win32ApiEntry, Win32Category, Win32ImportAudit};
use dxil_metal::formats::{ShaderFormatSpec, ShaderInspection};
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// Current stable JSON envelope schema version.
pub const SCHEMA_VERSION: u32 = 1;

/// Canonical exit-code table followed by every `port-factory` and `port-loader` command.
///
/// Documented in `docs/DIAGNOSTICS.md#exit-code-table`.
pub mod exit_codes {
    /// Success: clean audit, verification passed, `--no-run` resolved all imports,
    /// guest exited with 0, or `--max-frames` reached cleanly.
    pub const OK: i32 = 0;
    /// General runtime, I/O, PE parse, or internal failure (or non-zero guest exit).
    pub const RUNTIME_ERROR: i32 = 1;
    /// Invalid CLI arguments, unknown flag/subcommand, missing path, or no matching inputs.
    pub const USAGE_ERROR: i32 = 2;
    /// Unresolved static or Win32 imports detected (`port-runner --no-run`, `win32-audit`).
    pub const UNRESOLVED_IMPORTS: i32 = 3;
    /// Verification failure, database drift, or differential mismatch
    /// (`doctor`, `bundle-verify`, `ios-verify`, `verify-x86`, `oracle`, `port-runner --diff-test`).
    pub const VERIFICATION_FAILED: i32 = 4;
    /// Hard compatibility blocker detected (e.g. Ring-0 kernel driver imports in `anticheat-audit`).
    pub const INCOMPATIBLE: i32 = 5;
    /// Execution exceeded `--timeout-secs` limit (`port-runner`, `live-probe`, `bisect-runtime`).
    pub const TIMEOUT: i32 = 124;

    /// Human-readable constant name for a canonical exit code.
    pub const fn name(code: i32) -> &'static str {
        match code {
            OK => "OK",
            RUNTIME_ERROR => "RUNTIME_ERROR",
            USAGE_ERROR => "USAGE_ERROR",
            UNRESOLVED_IMPORTS => "UNRESOLVED_IMPORTS",
            VERIFICATION_FAILED => "VERIFICATION_FAILED",
            INCOMPATIBLE => "INCOMPATIBLE",
            TIMEOUT => "TIMEOUT",
            _ => "GUEST_EXIT",
        }
    }
}

/// Self-contained JSON AST value (RFC 8259).
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            JsonValue::Int(i) => Some(*i),
            JsonValue::UInt(u) => i64::try_from(*u).ok(),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            JsonValue::UInt(u) => Some(*u),
            JsonValue::Int(i) if *i >= 0 => Some(*i as u64),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Float(f) => Some(*f),
            JsonValue::Int(i) => Some(*i as f64),
            JsonValue::UInt(u) => Some(*u as f64),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[JsonValue]> {
        match self {
            JsonValue::Array(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&[(String, JsonValue)]> {
        match self {
            JsonValue::Object(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    /// Serialize to compact single-line JSON.
    pub fn to_json_compact(&self) -> String {
        let mut out = String::new();
        self.write_json(&mut out, None, 0);
        out
    }

    /// Serialize to 2-space pretty-printed JSON.
    pub fn to_json_pretty(&self) -> String {
        let mut out = String::new();
        self.write_json(&mut out, Some(2), 0);
        out
    }

    fn write_json(&self, out: &mut String, indent: Option<usize>, depth: usize) {
        match self {
            JsonValue::Null => out.push_str("null"),
            JsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            JsonValue::Int(i) => {
                let _ = write!(out, "{i}");
            }
            JsonValue::UInt(u) => {
                let _ = write!(out, "{u}");
            }
            JsonValue::Float(f) => {
                if f.is_finite() {
                    if *f == 0.0 {
                        out.push_str("0.0");
                    } else {
                        let s = format!("{f}");
                        out.push_str(&s);
                        if !s.contains('.') && !s.contains('e') && !s.contains('E') {
                            out.push_str(".0");
                        }
                    }
                } else {
                    out.push_str("null");
                }
            }
            JsonValue::Str(s) => write_json_string(out, s),
            JsonValue::Array(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push('[');
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        out.push(',');
                    }
                    if let Some(step) = indent {
                        out.push('\n');
                        push_spaces(out, (depth + 1) * step);
                    }
                    item.write_json(out, indent, depth + 1);
                }
                if let Some(step) = indent {
                    out.push('\n');
                    push_spaces(out, depth * step);
                }
                out.push(']');
            }
            JsonValue::Object(entries) => {
                if entries.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push('{');
                for (idx, (k, v)) in entries.iter().enumerate() {
                    if idx > 0 {
                        out.push(',');
                    }
                    if let Some(step) = indent {
                        out.push('\n');
                        push_spaces(out, (depth + 1) * step);
                    }
                    write_json_string(out, k);
                    if indent.is_some() {
                        out.push_str(": ");
                    } else {
                        out.push(':');
                    }
                    v.write_json(out, indent, depth + 1);
                }
                if let Some(step) = indent {
                    out.push('\n');
                    push_spaces(out, depth * step);
                }
                out.push('}');
            }
        }
    }
}

fn push_spaces(out: &mut String, n: usize) {
    for _ in 0..n {
        out.push(' ');
    }
}

/// Escape and quote a UTF-8 string per RFC 8259.
pub fn write_json_string(out: &mut String, s: &str) {
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
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
}

/// Ergonomic builder for [`JsonValue::Object`].
#[derive(Debug, Default)]
pub struct JsonObj {
    entries: Vec<(String, JsonValue)>,
}

impl JsonObj {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn val(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.entries.push((key.into(), value));
        self
    }

    pub fn str(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.val(key, JsonValue::Str(value.into()))
    }

    pub fn opt_str(self, key: impl Into<String>, value: Option<String>) -> Self {
        match value {
            Some(s) => self.val(key, JsonValue::Str(s)),
            None => self.val(key, JsonValue::Null),
        }
    }

    pub fn bool(self, key: impl Into<String>, value: bool) -> Self {
        self.val(key, JsonValue::Bool(value))
    }

    pub fn i64(self, key: impl Into<String>, value: i64) -> Self {
        self.val(key, JsonValue::Int(value))
    }

    pub fn u64(self, key: impl Into<String>, value: u64) -> Self {
        self.val(key, JsonValue::UInt(value))
    }

    pub fn usize(self, key: impl Into<String>, value: usize) -> Self {
        self.val(key, JsonValue::UInt(value as u64))
    }

    pub fn f64(self, key: impl Into<String>, value: f64) -> Self {
        self.val(key, JsonValue::Float(value))
    }

    pub fn str_array<I, S>(self, key: impl Into<String>, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.val(
            key,
            JsonValue::Array(items.into_iter().map(|s| JsonValue::Str(s.into())).collect()),
        )
    }

    pub fn build(self) -> JsonValue {
        JsonValue::Object(self.entries)
    }
}

/// Standard top-level versioned JSON envelope emitted by all `--json` commands.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonEnvelope {
    pub schema_version: u32,
    pub command: String,
    pub ok: bool,
    pub exit_code: i32,
    pub data: JsonValue,
    pub error: Option<String>,
}

impl JsonEnvelope {
    pub fn ok(command: impl Into<String>, data: JsonValue) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            command: command.into(),
            ok: true,
            exit_code: exit_codes::OK,
            data,
            error: None,
        }
    }

    pub fn with_status(
        command: impl Into<String>,
        exit_code: i32,
        data: JsonValue,
        error: Option<String>,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            command: command.into(),
            ok: exit_code == exit_codes::OK,
            exit_code,
            data,
            error,
        }
    }

    pub fn error(command: impl Into<String>, exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            command: command.into(),
            ok: false,
            exit_code,
            data: JsonValue::Null,
            error: Some(message.into()),
        }
    }

    pub fn to_json_value(&self) -> JsonValue {
        JsonObj::new()
            .u64("schema_version", self.schema_version as u64)
            .str("command", self.command.clone())
            .bool("ok", self.ok)
            .i64("exit_code", self.exit_code as i64)
            .val("data", self.data.clone())
            .opt_str("error", self.error.clone())
            .build()
    }

    pub fn to_json_pretty(&self) -> String {
        self.to_json_value().to_json_pretty()
    }

    pub fn to_json_compact(&self) -> String {
        self.to_json_value().to_json_compact()
    }
}

/// Parse an RFC 8259 JSON document into [`JsonValue`].
pub fn parse_json(input: &str) -> Result<JsonValue, String> {
    let mut parser = JsonParser::new(input);
    let val = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos != parser.bytes.len() {
        return Err(format!("trailing characters at byte {}", parser.pos));
    }
    Ok(val)
}

struct JsonParser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.bytes.len()
            && matches!(self.bytes[self.pos], b' ' | b'\t' | b'\r' | b'\n')
        {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_ws();
        let b = self
            .peek()
            .ok_or_else(|| "unexpected end of JSON input".to_string())?;
        match b {
            b'n' => self.expect_lit("null", JsonValue::Null),
            b't' => self.expect_lit("true", JsonValue::Bool(true)),
            b'f' => self.expect_lit("false", JsonValue::Bool(false)),
            b'"' => self.parse_string().map(JsonValue::Str),
            b'[' => self.parse_array(),
            b'{' => self.parse_object(),
            b'-' | b'0'..=b'9' => self.parse_number(),
            other => Err(format!(
                "unexpected byte {:?} at offset {}",
                other as char, self.pos
            )),
        }
    }

    fn expect_lit(&mut self, lit: &str, val: JsonValue) -> Result<JsonValue, String> {
        if self.src[self.pos..].starts_with(lit) {
            self.pos += lit.len();
            Ok(val)
        } else {
            Err(format!("expected {lit} at offset {}", self.pos))
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        if self.peek() != Some(b'"') {
            return Err(format!("expected '\"' at offset {}", self.pos));
        }
        self.pos += 1;
        let mut out = String::new();
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            match b {
                b'"' => {
                    self.pos += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.pos += 1;
                    let esc = self
                        .peek()
                        .ok_or_else(|| "unterminated escape in JSON string".to_string())?;
                    self.pos += 1;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{08}'),
                        b'f' => out.push('\u{0C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let code = self.parse_hex4()?;
                            if (0xD800..=0xDBFF).contains(&code) {
                                if self.src[self.pos..].starts_with("\\u") {
                                    self.pos += 2;
                                    let low = self.parse_hex4()?;
                                    if (0xDC00..=0xDFFF).contains(&low) {
                                        let cp = 0x10000
                                            + (((code as u32 - 0xD800) << 10)
                                                | (low as u32 - 0xDC00));
                                        let ch = char::from_u32(cp).ok_or_else(|| {
                                            format!("invalid surrogate pair at {}", self.pos)
                                        })?;
                                        out.push(ch);
                                    } else {
                                        return Err(format!("invalid low surrogate at {}", self.pos));
                                    }
                                } else {
                                    return Err(format!("missing low surrogate at {}", self.pos));
                                }
                            } else {
                                let ch = char::from_u32(code as u32)
                                    .ok_or_else(|| format!("invalid \\u{code:04x}"))?;
                                out.push(ch);
                            }
                        }
                        other => {
                            return Err(format!(
                                "invalid escape \\{} at offset {}",
                                other as char, self.pos
                            ))
                        }
                    }
                }
                c if c < 0x20 => {
                    return Err(format!(
                        "unescaped control byte 0x{c:02x} at offset {}",
                        self.pos
                    ));
                }
                _ => {
                    let ch = self.src[self.pos..]
                        .chars()
                        .next()
                        .ok_or_else(|| "invalid UTF-8 in JSON".to_string())?;
                    out.push(ch);
                    self.pos += ch.len_utf8();
                }
            }
        }
        Err("unterminated JSON string".to_string())
    }

    fn parse_hex4(&mut self) -> Result<u16, String> {
        if self.pos + 4 > self.bytes.len() {
            return Err("truncated \\uXXXX escape".to_string());
        }
        let s = &self.src[self.pos..self.pos + 4];
        self.pos += 4;
        u16::from_str_radix(s, 16).map_err(|_| format!("invalid \\u escape {s:?}"))
    }

    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        let mut is_float = false;
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        if self.peek() == Some(b'.') {
            is_float = true;
            self.pos += 1;
            let frac_start = self.pos;
            while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            if self.pos == frac_start {
                return Err(format!("invalid fractional number at offset {start}"));
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            if self.pos == exp_start {
                return Err(format!("invalid exponent at offset {start}"));
            }
        }
        let raw = &self.src[start..self.pos];
        if is_float {
            let f: f64 = raw
                .parse()
                .map_err(|_| format!("invalid float {raw:?} at offset {start}"))?;
            Ok(JsonValue::Float(f))
        } else if raw.starts_with('-') {
            let i: i64 = raw
                .parse()
                .map_err(|_| format!("invalid i64 {raw:?} at offset {start}"))?;
            Ok(JsonValue::Int(i))
        } else {
            let u: u64 = raw
                .parse()
                .map_err(|_| format!("invalid u64 {raw:?} at offset {start}"))?;
            Ok(JsonValue::UInt(u))
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.pos += 1; // '['
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(JsonValue::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b']') => {
                    self.pos += 1;
                    return Ok(JsonValue::Array(items));
                }
                _ => return Err(format!("expected ',' or ']' at offset {}", self.pos)),
            }
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.pos += 1; // '{'
        let mut entries = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(JsonValue::Object(entries));
        }
        loop {
            self.skip_ws();
            let key = self.parse_string()?;
            self.skip_ws();
            if self.peek() != Some(b':') {
                return Err(format!("expected ':' after key {key:?} at offset {}", self.pos));
            }
            self.pos += 1;
            let val = self.parse_value()?;
            entries.push((key, val));
            self.skip_ws();
            match self.peek() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(JsonValue::Object(entries));
                }
                _ => return Err(format!("expected ',' or '}}' at offset {}", self.pos)),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Shared structured JSON payload builders (reusable by CLI & `audit-game`)
// ---------------------------------------------------------------------------

/// Structured JSON for a single scanned PE binary (`scan`, `port-runner --ops-report`).
pub fn binary_profile_json(prof: &BinaryProfile, top_n: usize) -> JsonValue {
    let mut top_mnemonics: Vec<(&String, &u64)> = prof.by_mnemonic.iter().collect();
    top_mnemonics.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    let mut unmapped_list: Vec<(&String, &u64)> = prof.unmapped_mnemonics.iter().collect();
    unmapped_list.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    JsonObj::new()
        .str("binary_name", prof.binary_name.clone())
        .bool("is_dll", prof.is_dll)
        .str("image_base", format!("0x{:016x}", prof.image_base))
        .u64("size_of_image", prof.size_of_image as u64)
        .usize("functions_count", prof.functions_count)
        .usize("exports_count", prof.exports_count)
        .u64("total_instructions", prof.total_instructions)
        .u64("total_code_bytes", prof.total_code_bytes)
        .u64("jit_arm64_instructions", prof.jit_arm64_instructions)
        .u64("trap_boundary_instructions", prof.trap_boundary_instructions)
        .u64("unmapped_instructions", prof.unmapped_instructions)
        .f64("jit_coverage_pct", prof.jit_coverage_pct())
        .val(
            "imported_dlls",
            JsonValue::Array(
                prof.imported_dlls
                    .iter()
                    .map(|(dll, count)| {
                        JsonObj::new()
                            .str("dll", dll.clone())
                            .usize("import_count", *count)
                            .build()
                    })
                    .collect(),
            ),
        )
        .val(
            "by_category",
            JsonValue::Object(
                prof.by_category
                    .iter()
                    .map(|(cat, count)| (cat.as_str().to_string(), JsonValue::UInt(*count)))
                    .collect(),
            ),
        )
        .val(
            "top_mnemonics",
            JsonValue::Array(
                top_mnemonics
                    .into_iter()
                    .take(top_n)
                    .map(|(mn, count)| {
                        JsonObj::new()
                            .str("mnemonic", mn.clone())
                            .u64("count", *count)
                            .build()
                    })
                    .collect(),
            ),
        )
        .val(
            "unmapped_mnemonics",
            JsonValue::Array(
                unmapped_list
                    .into_iter()
                    .map(|(mn, count)| {
                        JsonObj::new()
                            .str("mnemonic", mn.clone())
                            .u64("count", *count)
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory scan --json`.
pub fn scan_data_json(
    profiles: &[BinaryProfile],
    skipped: &[(String, String)],
    recorded_tsv: Option<&str>,
    top_n: usize,
) -> JsonValue {
    let total_instructions: u64 = profiles.iter().map(|p| p.total_instructions).sum();
    let total_jit: u64 = profiles.iter().map(|p| p.jit_arm64_instructions).sum();
    let total_trap: u64 = profiles.iter().map(|p| p.trap_boundary_instructions).sum();
    let total_unmapped: u64 = profiles.iter().map(|p| p.unmapped_instructions).sum();
    let active = total_instructions.saturating_sub(total_trap);
    let aggregate_jit_pct = if active == 0 {
        100.0
    } else {
        (total_jit as f64 * 100.0) / (active as f64)
    };
    JsonObj::new()
        .usize("binaries_scanned", profiles.len())
        .u64("total_instructions", total_instructions)
        .u64("jit_arm64_instructions", total_jit)
        .u64("unmapped_instructions", total_unmapped)
        .f64("aggregate_jit_coverage_pct", aggregate_jit_pct)
        .opt_str("recorded_tsv", recorded_tsv.map(str::to_string))
        .val(
            "binaries",
            JsonValue::Array(
                profiles
                    .iter()
                    .map(|p| binary_profile_json(p, top_n))
                    .collect(),
            ),
        )
        .val(
            "skipped",
            JsonValue::Array(
                skipped
                    .iter()
                    .map(|(path, reason)| {
                        JsonObj::new()
                            .str("path", path.clone())
                            .str("reason", reason.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON for a single binary's Win32 import audit (`win32-audit`).
pub fn win32_audit_item_json(audit: &Win32ImportAudit) -> JsonValue {
    let mut unresolved_by_dll: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (dll, sym) in &audit.unresolved {
        unresolved_by_dll
            .entry(dll.as_str())
            .or_default()
            .push(sym.as_str());
    }
    JsonObj::new()
        .str("binary_name", audit.binary_name.clone())
        .usize("total_imports", audit.total_imports)
        .usize("resolved_imports", audit.resolved_win32)
        .usize("unresolved_count", audit.unresolved.len())
        .f64("readiness_pct", audit.coverage_pct())
        .val(
            "resolved_by_category",
            JsonValue::Object(
                audit
                    .resolved_by_category
                    .iter()
                    .map(|(cat, count)| (cat.as_str().to_string(), JsonValue::UInt(*count as u64)))
                    .collect(),
            ),
        )
        .val(
            "detected_middleware",
            JsonValue::Array(
                audit
                    .detected_middleware
                    .iter()
                    .map(|mw| {
                        JsonObj::new()
                            .str("id", mw.family.id)
                            .str("display_name", mw.family.display_name)
                            .str("version", mw.version.version)
                            .str("default_mode", mw.family.default_mode.as_str())
                            .str_array("matched_dlls", mw.matched_dlls.iter().cloned())
                            .build()
                    })
                    .collect(),
            ),
        )
        .val(
            "unresolved",
            JsonValue::Array(
                audit
                    .unresolved
                    .iter()
                    .map(|(dll, sym)| {
                        JsonObj::new()
                            .str("dll", dll.clone())
                            .str("symbol", sym.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .val(
            "unresolved_by_dll",
            JsonValue::Object(
                unresolved_by_dll
                    .into_iter()
                    .map(|(dll, syms)| {
                        (
                            dll.to_string(),
                            JsonValue::Array(
                                syms.into_iter()
                                    .map(|s| JsonValue::Str(s.to_string()))
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory win32-audit --json`.
pub fn win32_audit_data_json(audits: &[Win32ImportAudit]) -> JsonValue {
    let total_imports: usize = audits.iter().map(|a| a.total_imports).sum();
    let total_resolved: usize = audits.iter().map(|a| a.resolved_win32).sum();
    let total_unresolved: usize = audits.iter().map(|a| a.unresolved.len()).sum();
    let aggregate_pct = if total_imports == 0 {
        100.0
    } else {
        (total_resolved as f64 * 100.0) / (total_imports as f64)
    };
    JsonObj::new()
        .usize("binaries_audited", audits.len())
        .usize("total_imports", total_imports)
        .usize("resolved_imports", total_resolved)
        .usize("unresolved_imports", total_unresolved)
        .f64("aggregate_readiness_pct", aggregate_pct)
        .val(
            "binaries",
            JsonValue::Array(audits.iter().map(win32_audit_item_json).collect()),
        )
        .build()
}

/// Canonical machine-readable slug for [`AntiCheatVerdict`].
pub const fn anticheat_verdict_code(v: AntiCheatVerdict) -> &'static str {
    match v {
        AntiCheatVerdict::CompatibleClean => "compatible_clean",
        AntiCheatVerdict::CompatibleUserModeAc => "compatible_user_mode_ac",
        AntiCheatVerdict::CompatibleViaRuntimeUnpack => "compatible_via_runtime_unpack",
        AntiCheatVerdict::RequiresKernelDriverFallback => "requires_kernel_driver_fallback",
    }
}

/// Structured JSON for one binary's anti-cheat / packer audit (`anticheat-audit`).
pub fn anticheat_audit_item_json(rep: &AntiCheatAuditReport) -> JsonValue {
    JsonObj::new()
        .str("binary_name", rep.binary_name.clone())
        .str("verdict_code", anticheat_verdict_code(rep.verdict))
        .str("verdict", rep.verdict.as_str())
        .bool(
            "compatible",
            rep.verdict != AntiCheatVerdict::RequiresKernelDriverFallback,
        )
        .u64("direct_syscalls", rep.direct_syscalls)
        .val(
            "decoded_ssns",
            JsonValue::Array(
                rep.decoded_ssns
                    .iter()
                    .map(|(&ssn, &count)| {
                        JsonObj::new()
                            .str("ssn", format!("0x{ssn:04x}"))
                            .opt_str("name", lookup_ssn_name(ssn).map(str::to_string))
                            .u64("count", count)
                            .build()
                    })
                    .collect(),
            ),
        )
        .u64("ud2_count", rep.ud2_count)
        .u64("int3_count", rep.int3_count)
        .u64("cpuid_count", rep.cpuid_count)
        .u64("rdtsc_count", rep.rdtsc_count)
        .u64("xgetbv_count", rep.xgetbv_count)
        .u64("gs_teb_peb_reads", rep.gs_teb_peb_reads)
        .u64("kuser_shared_reads", rep.kuser_shared_reads)
        .val(
            "sections",
            JsonValue::Array(
                rep.sections
                    .iter()
                    .map(|s| {
                        JsonObj::new()
                            .str("name", s.name.clone())
                            .u64("virtual_size", s.virtual_size as u64)
                            .f64("entropy", s.entropy)
                            .bool("is_exec", s.is_exec)
                            .bool("is_write", s.is_write)
                            .build()
                    })
                    .collect(),
            ),
        )
        .str_array("anti_debug_imports", rep.anti_debug_imports.iter().cloned())
        .str_array("veh_seh_imports", rep.veh_seh_imports.iter().cloned())
        .str_array(
            "memory_unpack_imports",
            rep.memory_unpack_imports.iter().cloned(),
        )
        .str_array(
            "kernel_driver_imports",
            rep.kernel_driver_imports.iter().cloned(),
        )
        .build()
}

/// Structured JSON payload for `port-factory anticheat-audit --json`.
pub fn anticheat_audit_data_json(reports: &[AntiCheatAuditReport]) -> JsonValue {
    let blocked = reports
        .iter()
        .filter(|r| r.verdict == AntiCheatVerdict::RequiresKernelDriverFallback)
        .count();
    JsonObj::new()
        .usize("binaries_audited", reports.len())
        .usize("kernel_driver_blockers", blocked)
        .val(
            "binaries",
            JsonValue::Array(reports.iter().map(anticheat_audit_item_json).collect()),
        )
        .build()
}

/// Structured JSON for one binary's static runtime-hazard audit (`runtime-audit`).
pub fn runtime_hazard_item_json(r: &RuntimeHazardReport) -> JsonValue {
    JsonObj::new()
        .str("binary_name", r.binary_name.clone())
        .bool("is_dll", r.is_dll)
        .u64("total_instructions", r.total_instructions)
        .u64("ldmxcsr_count", r.ldmxcsr_count)
        .u64("stmxcsr_count", r.stmxcsr_count)
        .str_array(
            "ldmxcsr_sample_rvas",
            r.ldmxcsr_sample_rvas
                .iter()
                .map(|rva| format!("0x{rva:x}")),
        )
        .bool("has_mxcsr_control", r.has_mxcsr_control())
        .u64("x87_ctrl_count", r.x87_ctrl_count)
        .u64("x87_op_count", r.x87_op_count)
        .u64("avx_narrow_count", r.avx_narrow_count)
        .u64("approx_fp_count", r.approx_fp_count)
        .u64("loop_branch_count", r.loop_branch_count)
        .u64("lock_rmw_count", r.lock_rmw_count)
        .bool("has_wait_on_address", r.has_wait_on_address())
        .str_array("rwx_sections", r.rwx_sections.iter().cloned())
        .str_array("notable_imports", r.notable_imports.iter().cloned())
        .build()
}

/// Structured JSON payload for `port-factory runtime-audit --json`.
pub fn runtime_audit_data_json(
    reports: &[RuntimeHazardReport],
    skipped: &[(String, String)],
) -> JsonValue {
    let total_ldmxcsr: u64 = reports.iter().map(|r| r.ldmxcsr_count).sum();
    let total_x87: u64 = reports.iter().map(|r| r.x87_op_count).sum();
    let total_avx_narrow: u64 = reports.iter().map(|r| r.avx_narrow_count).sum();
    let total_approx_fp: u64 = reports.iter().map(|r| r.approx_fp_count).sum();
    let total_lock_rmw: u64 = reports.iter().map(|r| r.lock_rmw_count).sum();
    let rwx_binaries = reports.iter().filter(|r| !r.rwx_sections.is_empty()).count();
    JsonObj::new()
        .usize("binaries_audited", reports.len())
        .u64("total_ldmxcsr", total_ldmxcsr)
        .u64("total_x87_ops", total_x87)
        .u64("total_avx_narrow", total_avx_narrow)
        .u64("total_approx_fp", total_approx_fp)
        .u64("total_lock_rmw", total_lock_rmw)
        .usize("rwx_binaries", rwx_binaries)
        .val(
            "binaries",
            JsonValue::Array(reports.iter().map(runtime_hazard_item_json).collect()),
        )
        .val(
            "skipped",
            JsonValue::Array(
                skipped
                    .iter()
                    .map(|(path, reason)| {
                        JsonObj::new()
                            .str("path", path.clone())
                            .str("reason", reason.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON for one binary's app & GPU capability profile (`app-audit`).
pub fn app_capability_item_json(p: &AppCapabilityProfile) -> JsonValue {
    JsonObj::new()
        .str("binary_name", p.binary_name.clone())
        .bool("is_gui_subsystem", p.is_gui_subsystem)
        .bool("is_console_subsystem", p.is_console_subsystem)
        .str_array("console_pipe_ipc_apis", p.console_pipe_ipc_apis.iter().cloned())
        .str_array(
            "desktop_ui_clipboard_dpi_apis",
            p.desktop_ui_clipboard_dpi_apis.iter().cloned(),
        )
        .str_array("fs_watcher_mmap_apis", p.fs_watcher_mmap_apis.iter().cloned())
        .str_array("net_tls_crypto_apis", p.net_tls_crypto_apis.iter().cloned())
        .val(
            "gpu_apis_by_backend",
            JsonValue::Object(
                p.gpu_apis_by_backend
                    .iter()
                    .map(|(backend, apis)| {
                        (
                            backend.clone(),
                            JsonValue::Array(apis.iter().cloned().map(JsonValue::Str).collect()),
                        )
                    })
                    .collect(),
            ),
        )
        .usize("embedded_shaders_total", p.embedded_shaders_total)
        .usize("embedded_shaders_dxil", p.embedded_shaders_dxil)
        .usize("embedded_shaders_sm4_5", p.embedded_shaders_sm4_5)
        .usize("embedded_shaders_spirv", p.embedded_shaders_spirv)
        .usize("embedded_shaders_wave32", p.embedded_shaders_wave32)
        .usize("embedded_shaders_wave64", p.embedded_shaders_wave64)
        .val(
            "shader_profiles",
            JsonValue::Object(
                p.shader_profiles
                    .iter()
                    .map(|(k, v)| (k.clone(), JsonValue::UInt(*v as u64)))
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory app-audit --json`.
pub fn app_audit_data_json(profiles: &[AppCapabilityProfile]) -> JsonValue {
    let total_shaders: usize = profiles.iter().map(|p| p.embedded_shaders_total).sum();
    JsonObj::new()
        .usize("binaries_audited", profiles.len())
        .usize("embedded_shaders_total", total_shaders)
        .val(
            "binaries",
            JsonValue::Array(profiles.iter().map(app_capability_item_json).collect()),
        )
        .build()
}

/// Structured JSON for one binary in `.NET / CLR` audit (`dotnet-audit`).
pub fn dotnet_binary_json(b: &DotNetBinaryReport) -> JsonValue {
    JsonObj::new()
        .str("path", b.path.display().to_string())
        .str("file_name", b.file_name.clone())
        .bool("is_pe32_plus", b.is_pe32_plus)
        .str("machine", format!("0x{:04x}", b.machine))
        .str("kind", b.kind.as_str())
        .opt_str("clr_mode", b.clr_mode.map(|m| m.as_str().to_string()))
        .opt_str("metadata_version", b.metadata_version.clone())
        .str_array("metadata_streams", b.metadata_streams.iter().cloned())
        .str("clr_flags", format!("0x{:x}", b.clr_flags))
        .bool("is_ready_to_run", b.is_ready_to_run)
        .bool("is_strong_name_signed", b.is_strong_name_signed)
        .bool("imports_mscoree", b.imports_mscoree)
        .bool("imports_hostfxr", b.imports_hostfxr)
        .bool("imports_coreclr", b.imports_coreclr)
        .bool("imports_mono", b.imports_mono)
        .usize("il2cpp_exports", b.il2cpp_exports)
        .opt_str(
            "runtime_config_json",
            b.runtime_config_json.as_ref().map(|p| p.display().to_string()),
        )
        .str("recommendation", b.recommendation.clone())
        .build()
}

/// Structured JSON payload for `port-factory dotnet-audit --json`.
pub fn dotnet_audit_data_json(s: &DotNetAuditSummary) -> JsonValue {
    JsonObj::new()
        .usize("total_binaries", s.binaries.len())
        .usize("pure_il_count", s.pure_il_count)
        .usize("r2r_count", s.r2r_count)
        .usize("mixed_mode_count", s.mixed_mode_count)
        .usize("apphost_count", s.apphost_count)
        .usize("unity_mono_count", s.unity_mono_count)
        .usize("unity_il2cpp_count", s.unity_il2cpp_count)
        .usize("native_non_dotnet_count", s.native_non_dotnet_count)
        .str_array(
            "runtime_configs",
            s.runtime_configs.iter().map(|p| p.display().to_string()),
        )
        .str_array(
            "global_metadata_dat",
            s.global_metadata_dat.iter().map(|p| p.display().to_string()),
        )
        .opt_str(
            "host_coreclr_dylib",
            s.host_coreclr_dylib.as_ref().map(|p| p.display().to_string()),
        )
        .opt_str(
            "host_hostfxr_dylib",
            s.host_hostfxr_dylib.as_ref().map(|p| p.display().to_string()),
        )
        .opt_str(
            "host_mono_dylib",
            s.host_mono_dylib.as_ref().map(|p| p.display().to_string()),
        )
        .val(
            "binaries",
            JsonValue::Array(s.binaries.iter().map(dotnet_binary_json).collect()),
        )
        .build()
}

/// Structured JSON for one TSV database check in `doctor`.
pub fn doctor_db_item_json(item: &TsvCheckItem) -> JsonValue {
    let mut obj = JsonObj::new()
        .str("file_name", item.file_name)
        .str("path", item.abs_path.display().to_string())
        .str("label", item.label)
        .str("detail", item.detail.clone())
        .str("fix_cmd", item.fix_cmd);

    obj = match &item.status {
        TsvSyncStatus::InSync { line_count } => obj
            .str("status", "in_sync")
            .bool("drifted", false)
            .usize("line_count", *line_count),
        TsvSyncStatus::Fixed { line_count } => obj
            .str("status", "fixed")
            .bool("drifted", false)
            .usize("line_count", *line_count),
        TsvSyncStatus::Missing { expected_lines } => obj
            .str("status", "missing")
            .bool("drifted", true)
            .usize("expected_lines", *expected_lines),
        TsvSyncStatus::Drifted {
            disk_lines,
            expected_lines,
            added_lines,
            removed_lines,
            first_diff_line,
            sample_diffs,
        } => obj
            .str("status", "drifted")
            .bool("drifted", true)
            .usize("disk_lines", *disk_lines)
            .usize("expected_lines", *expected_lines)
            .usize("added_lines", *added_lines)
            .usize("removed_lines", *removed_lines)
            .usize("first_diff_line", *first_diff_line)
            .str_array("sample_diffs", sample_diffs.iter().cloned()),
    };
    obj.build()
}

/// Structured JSON payload for `port-factory doctor --json`.
#[allow(clippy::too_many_arguments)]
pub fn doctor_data_json(
    fix_mode: bool,
    db: &DoctorDbReport,
    vfs_root: &str,
    vfs_mounts: usize,
    shader_cache: &str,
    dx12_converter: bool,
    dx11_converter: bool,
    vulkan_spirv_toolchain: bool,
    pe_audits: Option<JsonValue>,
) -> JsonValue {
    let drifted_count = db.drifted_items().len();
    JsonObj::new()
        .str("mode", if fix_mode { "fix" } else { "check" })
        .bool("db_clean", drifted_count == 0)
        .usize("drifted_count", drifted_count)
        .val(
            "databases",
            JsonValue::Array(db.items.iter().map(doctor_db_item_json).collect()),
        )
        .val(
            "vfs",
            JsonObj::new()
                .str("root", vfs_root)
                .usize("mounts", vfs_mounts)
                .build(),
        )
        .val(
            "shaders",
            JsonObj::new()
                .str("cache_dir", shader_cache)
                .bool("dx12_available", dx12_converter)
                .bool("dx11_available", dx11_converter)
                .bool("vulkan_spirv_available", vulkan_spirv_toolchain)
                .build(),
        )
        .val("pe_audits", pe_audits.unwrap_or(JsonValue::Null))
        .build()
}

/// Structured JSON for one macOS `.app` portability report (`bundle-verify`).
pub fn bundle_verify_item_json(rep: &BundleVerifyReport) -> JsonValue {
    let pass_count = rep
        .checks
        .iter()
        .filter(|c| c.status == BundleCheckStatus::Pass)
        .count();
    JsonObj::new()
        .str("app_path", rep.app_path.display().to_string())
        .bool("passed", rep.passed())
        .usize("pass_count", pass_count)
        .usize("warn_count", rep.warn_count())
        .usize("fail_count", rep.fail_count())
        .val(
            "checks",
            JsonValue::Array(
                rep.checks
                    .iter()
                    .map(|c| {
                        let status = match c.status {
                            BundleCheckStatus::Pass => "pass",
                            BundleCheckStatus::Warn => "warn",
                            BundleCheckStatus::Fail => "fail",
                        };
                        JsonObj::new()
                            .str("name", c.name.clone())
                            .str("status", status)
                            .str("detail", c.detail.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory bundle-verify --json`.
pub fn bundle_verify_data_json(reports: &[BundleVerifyReport]) -> JsonValue {
    let passed_bundles = reports.iter().filter(|r| r.passed()).count();
    let failed_bundles = reports.len().saturating_sub(passed_bundles);
    JsonObj::new()
        .usize("bundles_verified", reports.len())
        .usize("passed_bundles", passed_bundles)
        .usize("failed_bundles", failed_bundles)
        .val(
            "bundles",
            JsonValue::Array(reports.iter().map(bundle_verify_item_json).collect()),
        )
        .build()
}

/// Structured JSON payload for `port-factory ios-verify --json`.
pub fn ios_verify_data_json(rep: &IosBundleVerifyReport) -> JsonValue {
    JsonObj::new()
        .str("bundle_path", rep.bundle_path.display().to_string())
        .bool("passed", rep.ok())
        .bool("is_ipa", rep.is_ipa)
        .str("bundle_id", rep.bundle_id.clone())
        .str("executable_name", rep.executable_name.clone())
        .str("macho_arch", rep.macho_arch.clone())
        .u64("macho_platform_id", rep.macho_platform_id as u64)
        .str("macho_platform_name", rep.macho_platform_name.clone())
        .usize("static_aot_modules", rep.static_aot_modules)
        .bool("has_game_toml", rep.has_game_toml)
        .bool("has_game_dir", rep.has_game_dir)
        .bool("bundled_exe_present", rep.bundled_exe_present)
        .usize("portable_shader_files", rep.portable_shader_files)
        .usize("host_mtl4a_files", rep.host_mtl4a_files)
        .str_array(
            "macos_framework_leaks",
            rep.macos_framework_leaks.iter().cloned(),
        )
        .bool("codesign_ok", rep.codesign_ok)
        .str_array("issues", rep.issues.iter().cloned())
        .build()
}

/// Structured JSON for one shader inspection (`shader-inspect`).
pub fn shader_inspect_item_json(insp: &ShaderInspection) -> JsonValue {
    let spec = insp.format.spec();
    JsonObj::new()
        .str("name", insp.name.clone())
        .usize("byte_len", insp.byte_len)
        .str("format_code", spec.code)
        .str("format_family", spec.family)
        .str("magic_signature", spec.magic_signature)
        .str("shader_model_or_version", insp.shader_model_or_version.clone())
        .str_array("container_parts", insp.container_parts.iter().cloned())
        .val(
            "entry_points",
            JsonValue::Array(
                insp.entry_points
                    .iter()
                    .map(|(name, stage)| {
                        JsonObj::new()
                            .str("name", name.clone())
                            .str("stage", stage.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .usize("resource_count", insp.resource_count)
        .bool(
            "has_root_signature_or_push_constants",
            insp.has_root_signature_or_push_constants,
        )
        .str("wave_or_workgroup_info", insp.wave_or_workgroup_info.clone())
        .usize("embedded_dxbc_count", insp.embedded_dxbc_count)
        .usize("embedded_spirv_count", insp.embedded_spirv_count)
        .build()
}

/// Structured JSON payload for `port-factory shader-inspect --json`.
pub fn shader_inspect_data_json(items: &[ShaderInspection]) -> JsonValue {
    JsonObj::new()
        .usize("shaders_inspected", items.len())
        .val(
            "shaders",
            JsonValue::Array(items.iter().map(shader_inspect_item_json).collect()),
        )
        .build()
}

/// Structured JSON payload for `port-factory game-profiles --json`.
pub fn game_profiles_data_json(
    audits: &[GameProfileAudit],
    synced_tsv: Option<&str>,
    detected: &[(String, String, String)],
) -> JsonValue {
    JsonObj::new()
        .usize("profiles_count", audits.len())
        .opt_str("synced_tsv", synced_tsv.map(str::to_string))
        .val(
            "profiles",
            JsonValue::Array(
                audits
                    .iter()
                    .map(|a| {
                        JsonObj::new()
                            .str("profile_id", a.profile.profile_id)
                            .str("engine_family", a.profile.engine_family)
                            .str("representative_games", a.profile.representative_games)
                            .str("graphics_apis", a.profile.graphics_apis)
                            .str("platform_drm_anticheat", a.profile.platform_drm_anticheat)
                            .usize("total_required_dlls", a.total_required_dlls)
                            .usize("resolved_dlls", a.resolved_dlls)
                            .str_array("missing_dlls", a.missing_dlls.iter().cloned())
                            .build()
                    })
                    .collect(),
            ),
        )
        .val(
            "detected_binaries",
            JsonValue::Array(
                detected
                    .iter()
                    .map(|(file, profile_id, engine)| {
                        JsonObj::new()
                            .str("binary_name", file.clone())
                            .str("profile_id", profile_id.clone())
                            .str("engine_family", engine.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory stats --json`.
pub fn ops_stats_data_json(st: &CatalogStats, records: &[OpRecord]) -> JsonValue {
    JsonObj::new()
        .usize("total_records", st.total_records)
        .usize("jit_arm64_records", st.jit_arm64_records)
        .usize("total_mnemonics", st.total_mnemonics)
        .u64("total_verified_vectors", st.total_verified_vectors as u64)
        .val(
            "records",
            JsonValue::Array(
                records
                    .iter()
                    .map(|r| {
                        JsonObj::new()
                            .str("id", r.id)
                            .u64("fast_kind_id", r.fast_kind_id as u64)
                            .str("fast_kind_name", r.fast_kind_name)
                            .str("category", r.category.as_str())
                            .str("tier", r.tier.as_str())
                            .str("widths", r.widths)
                            .str("operand_forms", r.operand_forms)
                            .str("arm64_pattern", r.arm64_pattern)
                            .u64("verified_vectors", r.verified_vectors as u64)
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory win32-stats --json`.
pub fn win32_stats_data_json(entries: &[Win32ApiEntry], search: Option<&str>) -> JsonValue {
    let mut by_cat: BTreeMap<Win32Category, usize> = BTreeMap::new();
    let mut by_dll: BTreeMap<(&str, Win32Category), usize> = BTreeMap::new();
    for e in entries {
        *by_cat.entry(e.category).or_insert(0) += 1;
        *by_dll.entry((e.dll.as_str(), e.category)).or_insert(0) += 1;
    }
    let matches: Vec<JsonValue> = if let Some(q) = search {
        entries
            .iter()
            .filter(|e| {
                format!("{} {} {}", e.dll, e.symbol, e.rust_path)
                    .to_lowercase()
                    .contains(q)
            })
            .map(|e| {
                JsonObj::new()
                    .str("dll", e.dll.clone())
                    .str("symbol", e.symbol.clone())
                    .str("category", e.category.as_str())
                    .str("rust_path", e.rust_path.clone())
                    .str("source_file", e.source_file.clone())
                    .build()
            })
            .collect()
    } else {
        Vec::new()
    };
    JsonObj::new()
        .usize("total_apis", entries.len())
        .usize("total_dlls", by_dll.len())
        .val(
            "by_category",
            JsonValue::Object(
                by_cat
                    .into_iter()
                    .map(|(cat, count)| (cat.as_str().to_string(), JsonValue::UInt(count as u64)))
                    .collect(),
            ),
        )
        .val(
            "by_dll",
            JsonValue::Array(
                by_dll
                    .into_iter()
                    .map(|((dll, cat), count)| {
                        JsonObj::new()
                            .str("dll", dll)
                            .str("category", cat.as_str())
                            .usize("exports", count)
                            .build()
                    })
                    .collect(),
            ),
        )
        .opt_str("search", search.map(str::to_string))
        .val("matches", JsonValue::Array(matches))
        .build()
}

/// Structured JSON payload for `port-factory shader-formats --json`.
pub fn shader_formats_data_json(
    catalog: &[ShaderFormatSpec],
    synced_tsv: Option<&str>,
) -> JsonValue {
    JsonObj::new()
        .usize("formats_count", catalog.len())
        .opt_str("synced_tsv", synced_tsv.map(str::to_string))
        .val(
            "formats",
            JsonValue::Array(
                catalog
                    .iter()
                    .map(|s| {
                        JsonObj::new()
                            .str("code", s.code)
                            .str("family", s.family)
                            .str("extensions", s.extensions)
                            .str("magic_signature", s.magic_signature)
                            .str("native_metal_pipeline", s.native_metal_pipeline)
                            .str("status", s.status)
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory vfs-resolve --json`.
pub fn vfs_resolve_data_json(
    cwd: &str,
    mounts: &[(String, String, String)],
    resolutions: &[(String, String, Option<(String, String, bool)>)],
) -> JsonValue {
    JsonObj::new()
        .str("cwd", cwd)
        .val(
            "mounts",
            JsonValue::Array(
                mounts
                    .iter()
                    .map(|(win, host, desc)| {
                        JsonObj::new()
                            .str("windows_prefix", win.clone())
                            .str("host_path", host.clone())
                            .str("description", desc.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .val(
            "resolutions",
            JsonValue::Array(
                resolutions
                    .iter()
                    .map(|(query, abs_win, resolved)| match resolved {
                        Some((host, mount, exists)) => JsonObj::new()
                            .str("query", query.clone())
                            .str("absolute_windows_path", abs_win.clone())
                            .bool("mounted", true)
                            .str("host_path", host.clone())
                            .str("mount", mount.clone())
                            .bool("exists", *exists)
                            .build(),
                        None => JsonObj::new()
                            .str("query", query.clone())
                            .str("absolute_windows_path", abs_win.clone())
                            .bool("mounted", false)
                            .val("host_path", JsonValue::Null)
                            .val("mount", JsonValue::Null)
                            .bool("exists", false)
                            .build(),
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory bundles --json`.
pub fn bundles_list_data_json(
    bundles: &[PackagedBundleInfo],
    volumes: &[ExternalVolumeInfo],
) -> JsonValue {
    JsonObj::new()
        .usize("bundles_count", bundles.len())
        .val(
            "bundles",
            JsonValue::Array(
                bundles
                    .iter()
                    .map(|b| {
                        JsonObj::new()
                            .str("name", b.name.clone())
                            .str("path", b.path.display().to_string())
                            .u64("size_bytes", b.size_bytes)
                            .str("runtime", b.runtime.clone())
                            .str("game_exe", b.game_exe.clone())
                            .usize("aot_modules", b.aot_modules)
                            .bool("has_metal_irconverter", b.has_metal_irconverter)
                            .bool("has_dxilconv", b.has_dxilconv)
                            .bool("has_moltenvk", b.has_moltenvk)
                            .bool("is_symlinked_game", b.is_symlinked_game)
                            .build()
                    })
                    .collect(),
            ),
        )
        .usize("external_volumes_count", volumes.len())
        .val(
            "external_volumes",
            JsonValue::Array(
                volumes
                    .iter()
                    .map(|v| {
                        JsonObj::new()
                            .str("name", v.name.clone())
                            .str("path", v.path.display().to_string())
                            .u64("free_bytes", v.free_bytes)
                            .u64("total_bytes", v.total_bytes)
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-factory steam-scan --json`.
pub fn steam_scan_data_json(report: &SteamLibraryScanReport, html_path: Option<&str>) -> JsonValue {
    JsonObj::new()
        .usize("users_count", report.users.len())
        .usize("library_folders_count", report.library_folders.len())
        .usize("installed_games_count", report.installed_games.len())
        .usize("owned_games_count", report.owned_games.len())
        .opt_str("html_path", html_path.map(str::to_string))
        .str_array(
            "library_folders",
            report
                .library_folders
                .iter()
                .map(|p| p.display().to_string()),
        )
        .val(
            "installed_games",
            JsonValue::Array(
                report
                    .installed_games
                    .iter()
                    .map(|g| {
                        JsonObj::new()
                            .str("app_id", g.app_id.clone())
                            .str("name", g.name.clone())
                            .str("source", g.source.label())
                            .str("install_dir", g.install_dir.display().to_string())
                            .u64("size_on_disk_bytes", g.size_on_disk_bytes)
                            .str("build_id", g.build_id.clone())
                            .str("platform", g.platform.clone())
                            .opt_str(
                                "primary_exe",
                                g.primary_exe.as_ref().map(|p| p.display().to_string()),
                            )
                            .usize("pe_count", g.pe_count)
                            .usize("total_imports", g.total_imports)
                            .usize("resolved_imports", g.resolved_imports)
                            .f64("readiness_pct", g.readiness_pct())
                            .str("engine_profile", g.engine_profile.clone())
                            .build()
                    })
                    .collect(),
            ),
        )
        .val(
            "owned_games",
            JsonValue::Array(
                report
                    .owned_games
                    .iter()
                    .map(|g| {
                        JsonObj::new()
                            .u64("app_id", g.app_id as u64)
                            .str("title", g.title.clone())
                            .str("engine_hint", g.engine_hint.clone())
                            .bool("installed", g.installed)
                            .opt_str("install_source", g.install_source.clone())
                            .u64("playtime_minutes", g.playtime_minutes)
                            .build()
                    })
                    .collect(),
            ),
        )
        .build()
}

/// Structured JSON payload for `port-runner --no-run --json` (and optional `--ops-report`).
#[allow(clippy::too_many_arguments)]
pub fn port_runner_no_run_data_json(
    target_exe: &str,
    exe_base: usize,
    preferred_base: u64,
    total_modules: usize,
    pe_modules: usize,
    builtin_modules: usize,
    load_elapsed_ms: u64,
    unresolved: &BTreeMap<String, Vec<String>>,
    ops_report: Option<JsonValue>,
) -> JsonValue {
    let unresolved_count: usize = unresolved.values().map(Vec::len).sum();
    JsonObj::new()
        .str("target", target_exe)
        .str("exe_base", format!("0x{exe_base:x}"))
        .str("preferred_base", format!("0x{preferred_base:x}"))
        .usize("total_modules", total_modules)
        .usize("pe_modules", pe_modules)
        .usize("builtin_modules", builtin_modules)
        .u64("load_elapsed_ms", load_elapsed_ms)
        .usize("unresolved_count", unresolved_count)
        .val(
            "unresolved_by_dll",
            JsonValue::Object(
                unresolved
                    .iter()
                    .map(|(dll, syms)| {
                        (
                            dll.clone(),
                            JsonValue::Array(syms.iter().cloned().map(JsonValue::Str).collect()),
                        )
                    })
                    .collect(),
            ),
        )
        .val("ops_report", ops_report.unwrap_or(JsonValue::Null))
        .build()
}

/// Structured JSON payload for `port-runner --json` live execution / run-limit summary.
pub fn port_runner_run_data_json(
    target_exe: &str,
    stop_reason: &str,
    frames_presented: u64,
    elapsed_ms: u64,
    max_frames: Option<u64>,
    timeout_secs: Option<f64>,
    guest_exit_code: Option<u32>,
) -> JsonValue {
    JsonObj::new()
        .str("target", target_exe)
        .str("stop_reason", stop_reason)
        .u64("frames_presented", frames_presented)
        .u64("elapsed_ms", elapsed_ms)
        .val(
            "max_frames",
            max_frames.map(JsonValue::UInt).unwrap_or(JsonValue::Null),
        )
        .val(
            "timeout_secs",
            timeout_secs.map(JsonValue::Float).unwrap_or(JsonValue::Null),
        )
        .val(
            "guest_exit_code",
            guest_exit_code
                .map(|c| JsonValue::UInt(c as u64))
                .unwrap_or(JsonValue::Null),
        )
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_roundtrip_and_escaping() {
        let env = JsonEnvelope::ok(
            "scan",
            JsonObj::new()
                .str("escaped", "a\"b\\c\nd\te\u{07}")
                .i64("neg", -42)
                .u64("big", u64::MAX)
                .f64("pct", 99.5)
                .f64("nan_becomes_null", f64::NAN)
                .bool("flag", true)
                .str_array("list", ["x", "y"])
                .build(),
        );
        let text = env.to_json_pretty();
        let parsed = parse_json(&text).expect("valid JSON");
        assert_eq!(parsed.get("schema_version").and_then(JsonValue::as_u64), Some(1));
        assert_eq!(parsed.get("command").and_then(JsonValue::as_str), Some("scan"));
        assert_eq!(parsed.get("ok").and_then(JsonValue::as_bool), Some(true));
        assert_eq!(parsed.get("exit_code").and_then(JsonValue::as_i64), Some(0));
        assert!(parsed.get("error").unwrap().is_null());
        let data = parsed.get("data").unwrap();
        assert_eq!(
            data.get("escaped").and_then(JsonValue::as_str),
            Some("a\"b\\c\nd\te\u{07}")
        );
        assert_eq!(data.get("neg").and_then(JsonValue::as_i64), Some(-42));
        assert_eq!(data.get("big").and_then(JsonValue::as_u64), Some(u64::MAX));
        assert_eq!(data.get("pct").and_then(JsonValue::as_f64), Some(99.5));
        assert!(data.get("nan_becomes_null").unwrap().is_null());
        assert_eq!(data.get("flag").and_then(JsonValue::as_bool), Some(true));
        assert_eq!(data.get("list").and_then(JsonValue::as_array).unwrap().len(), 2);
    }

    #[test]
    fn exit_code_names_are_distinct_and_stable() {
        let codes = [
            exit_codes::OK,
            exit_codes::RUNTIME_ERROR,
            exit_codes::USAGE_ERROR,
            exit_codes::UNRESOLVED_IMPORTS,
            exit_codes::VERIFICATION_FAILED,
            exit_codes::INCOMPATIBLE,
            exit_codes::TIMEOUT,
        ];
        for (i, &a) in codes.iter().enumerate() {
            assert_ne!(exit_codes::name(a), "GUEST_EXIT");
            for &b in &codes[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }
}
