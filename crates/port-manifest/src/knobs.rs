//! Unified environment-knob registry for Apple Silicon Port Factory.
//!
//! Every `PORT_*`, legacy `AW2_*`, and project-specific environment knob read across
//! `crates/` is declared once in [`KNOBS`] with its canonical name, legacy aliases,
//! type, default value, description, AOT cache-key impact, and reading crates.
//!
//! Precedence everywhere is strictly:
//! `process env (canonical) > process env (alias) > game.toml [knobs] > default`.
//!
//! Unit tests in this module scan all Rust sources under `crates/` and fail if any
//! unregistered environment variable is read.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::RwLock;

use crate::manifest::GameManifest;

/// Value category for a registered runtime knob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnobType {
    Bool,
    U32,
    U64,
    I32,
    Usize,
    F64,
    String,
    Path,
    List,
    Enum(&'static [&'static str]),
}

impl KnobType {
    /// Short type label for CLI tables and JSON output.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I32 => "i32",
            Self::Usize => "usize",
            Self::F64 => "f64",
            Self::String => "string",
            Self::Path => "path",
            Self::List => "list",
            Self::Enum(_) => "enum",
        }
    }

    /// Allowed values for [`Self::Enum`] knobs, if any.
    pub fn enum_values(self) -> Option<&'static [&'static str]> {
        match self {
            Self::Enum(vals) => Some(vals),
            _ => None,
        }
    }
}

/// Static declaration of one Port Factory runtime knob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KnobSpec {
    /// Canonical environment variable name (universal `PORT_*` or cross-crate name).
    pub name: &'static str,
    /// Legacy or alternate environment variable names (e.g. `AW2_*`).
    pub aliases: &'static [&'static str],
    /// Parsed value type.
    pub kind: KnobType,
    /// Effective default value when unset (empty string means unset / auto).
    pub default: &'static str,
    /// Concise description of what the knob controls.
    pub description: &'static str,
    /// Whether changing this knob alters generated ARM64 AOT/JIT code or stubs
    /// and therefore must partition the `.aot` cache key.
    pub affects_aot_key: bool,
    /// Workspace crates under `crates/` that read this knob.
    pub crates: &'static [&'static str],
}

/// Provenance of a knob's effective value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnobSource {
    /// Set in process environment via canonical [`KnobSpec::name`].
    Env,
    /// Set in process environment via a legacy/alternate alias.
    EnvAlias(&'static str),
    /// Set in `game.toml`'s `[knobs]` section.
    Manifest,
    /// Neither process env nor `game.toml` set this knob; using [`KnobSpec::default`].
    Default,
}

impl KnobSource {
    /// Machine-readable source tag for JSON output.
    pub fn tag(&self) -> &'static str {
        match self {
            Self::Env => "env",
            Self::EnvAlias(_) => "env_alias",
            Self::Manifest => "game.toml",
            Self::Default => "default",
        }
    }

    /// Human-readable source label for `--knobs` table output.
    pub fn display_label(&self) -> String {
        match self {
            Self::Env => "env".to_string(),
            Self::EnvAlias(alias) => format!("env ({alias})"),
            Self::Manifest => "game.toml".to_string(),
            Self::Default => "default".to_string(),
        }
    }
}

/// Resolved effective value and provenance for a registered knob.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedKnob {
    pub spec: &'static KnobSpec,
    pub value: String,
    pub source: KnobSource,
}

/// Process-wide `[knobs]` table loaded from the active `game.toml` (if any).
static ACTIVE_MANIFEST_KNOBS: RwLock<Option<BTreeMap<String, String>>> = RwLock::new(None);

/// Look up a knob specification by canonical name or any registered alias.
pub fn lookup(name_or_alias: &str) -> Option<&'static KnobSpec> {
    let trimmed = name_or_alias.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(idx) = KNOBS.binary_search_by_key(&trimmed, |k| k.name) {
        return Some(&KNOBS[idx]);
    }
    KNOBS
        .iter()
        .find(|k| k.aliases.iter().any(|&a| a == trimmed))
}

/// Look up a knob specification case-insensitively by canonical name or alias.
pub fn lookup_ignore_ascii_case(name_or_alias: &str) -> Option<&'static KnobSpec> {
    if let Some(exact) = lookup(name_or_alias) {
        return Some(exact);
    }
    let trimmed = name_or_alias.trim();
    KNOBS.iter().find(|k| {
        k.name.eq_ignore_ascii_case(trimmed)
            || k.aliases.iter().any(|a| a.eq_ignore_ascii_case(trimmed))
    })
}

fn read_env_nonempty(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.is_empty())
}

fn find_manifest_value(
    spec: &KnobSpec,
    manifest_knobs: &BTreeMap<String, String>,
) -> Option<String> {
    if let Some(v) = manifest_knobs.get(spec.name) {
        return Some(v.clone());
    }
    for (k, v) in manifest_knobs {
        if k.eq_ignore_ascii_case(spec.name) {
            return Some(v.clone());
        }
    }
    for &alias in spec.aliases {
        if let Some(v) = manifest_knobs.get(alias) {
            return Some(v.clone());
        }
        for (k, v) in manifest_knobs {
            if k.eq_ignore_ascii_case(alias) {
                return Some(v.clone());
            }
        }
    }
    None
}

/// Normalize boolean strings from `game.toml` into `"1"` / `"0"` so legacy env readers
/// checking `v == "1"` or `v != "0"` behave consistently.
pub fn normalize_value_for_env(kind: KnobType, raw: &str) -> String {
    let trimmed = raw.trim();
    if matches!(kind, KnobType::Bool) {
        match trimmed.to_ascii_lowercase().as_str() {
            "true" | "yes" | "on" | "1" => return "1".to_string(),
            "false" | "no" | "off" | "0" => return "0".to_string(),
            _ => {}
        }
    }
    trimmed.to_string()
}

/// Resolve a single knob with precedence `env (canonical) > env (alias) > game.toml > default`.
pub fn resolve_spec(
    spec: &'static KnobSpec,
    manifest_knobs: Option<&BTreeMap<String, String>>,
) -> ResolvedKnob {
    if let Some(v) = read_env_nonempty(spec.name) {
        return ResolvedKnob {
            spec,
            value: v,
            source: KnobSource::Env,
        };
    }
    for &alias in spec.aliases {
        if let Some(v) = read_env_nonempty(alias) {
            return ResolvedKnob {
                spec,
                value: v,
                source: KnobSource::EnvAlias(alias),
            };
        }
    }
    if let Some(mk) = manifest_knobs {
        if let Some(v) = find_manifest_value(spec, mk) {
            return ResolvedKnob {
                spec,
                value: normalize_value_for_env(spec.kind, &v),
                source: KnobSource::Manifest,
            };
        }
    }
    ResolvedKnob {
        spec,
        value: spec.default.to_string(),
        source: KnobSource::Default,
    }
}

/// Resolve every registered knob against the current process environment and optional
/// `game.toml` `[knobs]` table.
pub fn resolve_all(manifest_knobs: Option<&BTreeMap<String, String>>) -> Vec<ResolvedKnob> {
    KNOBS
        .iter()
        .map(|spec| resolve_spec(spec, manifest_knobs))
        .collect()
}

/// Return any keys in `manifest_knobs` that do not match a registered knob name or alias.
pub fn validate_manifest_knobs(manifest_knobs: &BTreeMap<String, String>) -> Vec<String> {
    manifest_knobs
        .keys()
        .filter(|k| lookup_ignore_ascii_case(k).is_none())
        .cloned()
        .collect()
}

/// Apply `[knobs]` from `manifest` (if present) and synchronize canonical `PORT_*` names
/// with their legacy `AW2_*` aliases before the runtime starts.
///
/// Precedence is strictly `env (canonical) > env (alias) > game.toml [knobs] > default`:
/// - Fails loudly with an error if `[knobs]` contains any unregistered knob name.
/// - Mirrors any env-set canonical knob to all of its aliases (and vice versa) so
///   unmigrated legacy call sites immediately honor canonical `PORT_*` overrides.
/// - Materializes non-overridden `game.toml` `[knobs]` entries into both the canonical
///   env var and all of its aliases.
/// - Leaves unset default knobs untouched in process `std::env`.
pub fn apply_manifest_and_aliases(
    manifest: Option<&GameManifest>,
) -> Result<Vec<ResolvedKnob>, String> {
    let manifest_knobs = manifest.map(|m| &m.knobs);
    if let Some(mk) = manifest_knobs {
        let unknown = validate_manifest_knobs(mk);
        if !unknown.is_empty() {
            return Err(format!(
                "unknown knob(s) in game.toml [knobs]: {} (run `port-runner --knobs` to list registered knobs)",
                unknown.join(", ")
            ));
        }
    }

    let resolved = resolve_all(manifest_knobs);

    if let Ok(mut guard) = ACTIVE_MANIFEST_KNOBS.write() {
        *guard = manifest_knobs.cloned();
    }

    for entry in &resolved {
        match entry.source {
            KnobSource::Env => {
                for &alias in entry.spec.aliases {
                    std::env::set_var(alias, &entry.value);
                }
            }
            KnobSource::EnvAlias(set_alias) => {
                std::env::set_var(entry.spec.name, &entry.value);
                for &alias in entry.spec.aliases {
                    if alias != set_alias {
                        std::env::set_var(alias, &entry.value);
                    }
                }
            }
            KnobSource::Manifest => {
                std::env::set_var(entry.spec.name, &entry.value);
                for &alias in entry.spec.aliases {
                    std::env::set_var(alias, &entry.value);
                }
            }
            KnobSource::Default => {}
        }
    }

    Ok(resolved)
}

/// Read the effective string value of a registered knob (checking env, aliases, active
/// manifest `[knobs]`, and default). Panics in debug builds if `name` is unregistered.
pub fn get_str(name: &str) -> String {
    let Some(spec) = lookup(name) else {
        debug_assert!(false, "unregistered knob requested: {name}");
        return std::env::var(name).unwrap_or_default();
    };
    let guard = ACTIVE_MANIFEST_KNOBS.read().ok();
    let mk = guard.as_ref().and_then(|g| g.as_ref());
    resolve_spec(spec, mk).value
}

/// Read a registered boolean knob (`"1"`, `"true"`, `"yes"`, `"on"` => `true`; `"0"`,
/// `"false"`, `"no"`, `"off"` => `false`).
pub fn get_bool(name: &str) -> bool {
    let v = get_str(name);
    let t = v.trim();
    if t.is_empty() {
        return false;
    }
    !matches!(t.to_ascii_lowercase().as_str(), "0" | "false" | "no" | "off")
}

/// Read a registered `u32` knob, falling back to its parsed default or `0`.
pub fn get_u32(name: &str) -> u32 {
    let v = get_str(name);
    if let Ok(n) = v.trim().parse::<u32>() {
        return n;
    }
    lookup(name)
        .and_then(|s| s.default.trim().parse::<u32>().ok())
        .unwrap_or(0)
}

/// Read a registered `u64` knob, falling back to its parsed default or `0`.
pub fn get_u64(name: &str) -> u64 {
    let v = get_str(name);
    if let Ok(n) = v.trim().parse::<u64>() {
        return n;
    }
    lookup(name)
        .and_then(|s| s.default.trim().parse::<u64>().ok())
        .unwrap_or(0)
}

/// Compute a deterministic 64-bit FNV-1a hash over the effective values of every
/// registered knob with `affects_aot_key == true`, using `manifest_knobs` as the
/// fallback layer beneath process env.
pub fn aot_key_hash_with(manifest_knobs: Option<&BTreeMap<String, String>>) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut h = FNV_OFFSET;
    let mut mix = |bytes: &[u8]| {
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        h ^= 0xff;
        h = h.wrapping_mul(FNV_PRIME);
    };
    for spec in KNOBS {
        if !spec.affects_aot_key {
            continue;
        }
        let resolved = resolve_spec(spec, manifest_knobs);
        let norm = normalize_value_for_env(spec.kind, &resolved.value);
        mix(spec.name.as_bytes());
        mix(norm.as_bytes());
    }
    h
}

/// Compute the deterministic 64-bit hash over all AOT-key-affecting knobs using the
/// active process environment and any applied `game.toml` `[knobs]`.
pub fn aot_key_hash() -> u64 {
    let guard = ACTIVE_MANIFEST_KNOBS.read().ok();
    let mk = guard.as_ref().and_then(|g| g.as_ref());
    aot_key_hash_with(mk)
}

/// Format resolved knobs as a human-readable aligned table for `port-runner --knobs`.
pub fn format_table(resolved: &[ResolvedKnob], manifest_path: Option<&Path>) -> String {
    let mut out = String::new();
    let env_count = resolved
        .iter()
        .filter(|r| matches!(r.source, KnobSource::Env | KnobSource::EnvAlias(_)))
        .count();
    let manifest_count = resolved
        .iter()
        .filter(|r| matches!(r.source, KnobSource::Manifest))
        .count();
    let aot_count = resolved.iter().filter(|r| r.spec.affects_aot_key).count();
    out.push_str(&format!(
        "Port Factory Knob Registry ({} knobs; {} from env, {} from game.toml, {} affect AOT key)\n",
        resolved.len(),
        env_count,
        manifest_count,
        aot_count
    ));
    if let Some(p) = manifest_path {
        out.push_str(&format!("Manifest: {}\n", p.display()));
    } else {
        out.push_str("Manifest: (none)\n");
    }
    out.push_str(&format!(
        "{:<34} {:<6} {:<18} {:<24} {:<3} {:<24} {}\n",
        "NAME", "TYPE", "VALUE", "SOURCE", "AOT", "CRATES", "DESCRIPTION"
    ));
    out.push_str(&"-".repeat(132));
    out.push('\n');
    for r in resolved {
        let val_disp = if r.value.is_empty() {
            "<unset>".to_string()
        } else {
            r.value.clone()
        };
        let src_disp = r.source.display_label();
        let aot_disp = if r.spec.affects_aot_key { "yes" } else { "-" };
        let crates_disp = r.spec.crates.join(",");
        let desc = if r.spec.aliases.is_empty() {
            r.spec.description.to_string()
        } else {
            format!("{} [aliases: {}]", r.spec.description, r.spec.aliases.join(", "))
        };
        out.push_str(&format!(
            "{:<34} {:<6} {:<18} {:<24} {:<3} {:<24} {}\n",
            r.spec.name,
            r.spec.kind.as_str(),
            val_disp,
            src_disp,
            aot_disp,
            crates_disp,
            desc
        ));
    }
    out
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
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Format resolved knobs as strict JSON for `port-runner --knobs --json`.
pub fn format_json(
    resolved: &[ResolvedKnob],
    manifest_path: Option<&Path>,
    aot_hash: u64,
) -> String {
    let mut out = String::with_capacity(resolved.len() * 256);
    out.push_str("{\n");
    match manifest_path {
        Some(p) => out.push_str(&format!(
            "  \"manifest\": {},\n",
            json_escape(&p.display().to_string())
        )),
        None => out.push_str("  \"manifest\": null,\n"),
    }
    out.push_str(&format!("  \"aot_key_hash\": \"0x{aot_hash:016x}\",\n"));
    out.push_str(&format!("  \"count\": {},\n", resolved.len()));
    out.push_str("  \"knobs\": [\n");
    for (idx, r) in resolved.iter().enumerate() {
        out.push_str("    {\n");
        out.push_str(&format!("      \"name\": {},\n", json_escape(r.spec.name)));
        out.push_str("      \"aliases\": [");
        for (ai, a) in r.spec.aliases.iter().enumerate() {
            if ai > 0 {
                out.push_str(", ");
            }
            out.push_str(&json_escape(a));
        }
        out.push_str("],\n");
        out.push_str(&format!(
            "      \"type\": {},\n",
            json_escape(r.spec.kind.as_str())
        ));
        out.push_str(&format!(
            "      \"default\": {},\n",
            json_escape(r.spec.default)
        ));
        out.push_str(&format!("      \"value\": {},\n", json_escape(&r.value)));
        out.push_str(&format!(
            "      \"source\": {},\n",
            json_escape(r.source.tag())
        ));
        match (&r.source, manifest_path) {
            (KnobSource::EnvAlias(alias), _) => {
                out.push_str(&format!("      \"source_detail\": {},\n", json_escape(alias)));
            }
            (KnobSource::Manifest, Some(p)) => {
                out.push_str(&format!(
                    "      \"source_detail\": {},\n",
                    json_escape(&p.display().to_string())
                ));
            }
            _ => {
                out.push_str("      \"source_detail\": null,\n");
            }
        }
        out.push_str(&format!(
            "      \"affects_aot_key\": {},\n",
            r.spec.affects_aot_key
        ));
        out.push_str("      \"crates\": [");
        for (ci, c) in r.spec.crates.iter().enumerate() {
            if ci > 0 {
                out.push_str(", ");
            }
            out.push_str(&json_escape(c));
        }
        out.push_str("],\n");
        out.push_str(&format!(
            "      \"description\": {}\n",
            json_escape(r.spec.description)
        ));
        if idx + 1 < resolved.len() {
            out.push_str("    },\n");
        } else {
            out.push_str("    }\n");
        }
    }
    out.push_str("  ]\n");
    out.push_str("}\n");
    out
}

/// Complete sorted registry of all Port Factory environment knobs.
pub const KNOBS: &[KnobSpec] = &[
    KnobSpec {
        name: "DXC_BIN",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to dxc CLI binary for runtime HLSL compilation.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "DXC_DYLIB",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to libdxcompiler.dylib for runtime HLSL compilation.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "METAL_IRCONVERTER_DYLIB",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to Apple Metal Shader Converter (libmetalirconverter.dylib).",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-factory", "port-loader"],
    },
    KnobSpec {
        name: "PORT_A64_ENCODINGS",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Path to Arm A64 XML/JSON encoding catalog for offline emitter validation test.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_A64_MATCHES_OUT",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Output path for matched A64 encoding report in emitter validation test.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_ACTIVATE",
        aliases: &["AW2_ACTIVATE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Force macOS application foreground activation on window creation.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AIR_NO_RES_HEAP",
        aliases: &["AW2_AIR_NO_RES_HEAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Bypass SFI0 RESOURCE_HEAP_INDEXING opacity when metallib RBUF proves heap unused.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_AK_FORCE_SSE",
        aliases: &["AW2_AK_FORCE_SSE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Force SSE mixer SIMD path in Wwise audio harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AK_RECORD",
        aliases: &["AW2_AK_RECORD"],
        kind: KnobType::Path,
        default: "",
        description: "Output trace path to record live Wwise AK:: API calls.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AK_REPLAY_OUT",
        aliases: &["AW2_AK_REPLAY_OUT"],
        kind: KnobType::Path,
        default: "",
        description: "Output WAV path for Wwise AK:: trace replay.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AK_REPLAY_TAIL_FRAMES",
        aliases: &["AW2_AK_REPLAY_TAIL_FRAMES"],
        kind: KnobType::U64,
        default: "94",
        description: "Extra tail frames rendered at end of Wwise trace replay.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AK_REPLAY_TAP",
        aliases: &["AW2_AK_REPLAY_TAP"],
        kind: KnobType::Path,
        default: "",
        description: "Optional tap output path during Wwise trace replay.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AK_SEED",
        aliases: &["AW2_AK_SEED"],
        kind: KnobType::String,
        default: "0x5eed",
        description: "RNG seed (decimal or hex) used by Wwise audio oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AK_TRACE",
        aliases: &["AW2_AK_TRACE"],
        kind: KnobType::Path,
        default: "",
        description: "Input trace path for Wwise AK:: offline replay.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AOT",
        aliases: &["AW2_DLL_AOT"],
        kind: KnobType::U32,
        default: "2",
        description: "AOT translation cache mode: 0=JIT only, 1=load caches, 2=load + background build.",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "win32-rt", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_ALLOW",
        aliases: &[],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated guest VA ranges allowed to load from AOT caches.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_BG_DELAY",
        aliases: &["AW2_DLL_AOT_BG_DELAY"],
        kind: KnobType::U32,
        default: "60",
        description: "Seconds after first AOT activation before background rebuilds and sweeps run.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_BUILD",
        aliases: &["AW2_DLL_AOT_BUILD"],
        kind: KnobType::String,
        default: "",
        description: "Build .aot caches for listed module substrings (1/all for all mapped modules).",
        affects_aot_key: false,
        crates: &["port-loader", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_CLOSURE",
        aliases: &["AW2_DLL_AOT_CLOSURE"],
        kind: KnobType::U32,
        default: "128",
        description: "Static CFG closure BFS depth per discovered root during AOT prewarming.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_DIR",
        aliases: &["AW2_DLL_AOT_DIR"],
        kind: KnobType::Path,
        default: "",
        description: "Writable directory for relocatable ARM64 .aot block caches.",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "win32-rt", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_DYLIB",
        aliases: &["AW2_AOT_DYLIB"],
        kind: KnobType::Path,
        default: "",
        description: "Comma-separated paths to precompiled native AOT dylibs to load.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_AOT_FALLBACK_DIR",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Read-only fallback directory for bundled .aot caches (never written).",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "win32-rt", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_GUARD",
        aliases: &["AW2_DLL_AOT_GUARD"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable AOT crash-loop guard (disables AOT for one run after rapid crash).",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_GUARD_SECS",
        aliases: &["AW2_DLL_AOT_GUARD_SECS"],
        kind: KnobType::U32,
        default: "120",
        description: "Session duration threshold (seconds) for the AOT crash-loop guard.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_JUMPTABLES",
        aliases: &["AW2_DLL_AOT_JUMPTABLES"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable RIP-relative switch/jump-table discovery during AOT CFG closure.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_NO_COND_PATCH",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Disable direct conditional-branch patching between installed AOT blocks.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_NO_LINK",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Disable direct branch linking between installed AOT blocks.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_PDATA",
        aliases: &["AW2_DLL_AOT_PDATA"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use PE .pdata RUNTIME_FUNCTION table for AOT root discovery.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_PROBE",
        aliases: &["AW2_DLL_AOT_PROBE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Emit shared AOT probe-table lookup inside JIT lookup stubs.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_RELOC_ROOTS",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Harvest code roots from PE base relocations during AOT prewarming.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_REVIVE",
        aliases: &["AW2_DLL_AOT_REVIVE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Allow revalidated AOT blocks to be revived after code-page reprotection.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_SKIP",
        aliases: &[],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated guest VA ranges excluded from AOT cache installation.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_STRICT",
        aliases: &[],
        kind: KnobType::Enum(&["off", "report", "fail"]),
        default: "off",
        description: "Strict AOT fallback accounting: 'report' logs & counts runtime JIT compiles, interpreter steps, and software-FP fallbacks with module+RVA; 'fail' aborts (exit 87) on the first fallback.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_THREADS",
        aliases: &["AW2_DLL_AOT_THREADS"],
        kind: KnobType::U32,
        default: "0",
        description: "Worker thread count for AOT prewarming/builds (0 = available parallelism).",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_AOT_VERIFY",
        aliases: &["AW2_DLL_AOT_VERIFY"],
        kind: KnobType::U32,
        default: "0",
        description: "Re-translate and verify every loaded AOT block against the live JIT.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_ASLR_RETRY",
        aliases: &["AW2_ASLR_RETRY"],
        kind: KnobType::U32,
        default: "0",
        description: "Internal counter tracking ASLR preferred-base re-exec attempts.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_AUDIO",
        aliases: &["AW2_AUDIO"],
        kind: KnobType::String,
        default: "",
        description: "Audio routing mode ('native' routes hooked Wwise exports to wwise-audio).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_BACKEND",
        aliases: &["AW2_AUDIO_BACKEND"],
        kind: KnobType::String,
        default: "coreaudio",
        description: "Audio output backend (coreaudio, null, etc.).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_CHANNELS",
        aliases: &["AW2_AUDIO_CHANNELS"],
        kind: KnobType::U32,
        default: "0",
        description: "Override reported WASAPI mix format channel count (0 = device default).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_DEVICE",
        aliases: &["AW2_AUDIO_DEVICE"],
        kind: KnobType::String,
        default: "",
        description: "Substring match to select a specific CoreAudio output device.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_DEVICE_EVENTS",
        aliases: &["AW2_AUDIO_DEVICE_EVENTS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Watch CoreAudio device hotplug and default-device change events.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS",
        aliases: &["AW2_AUDIO_HARNESS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Run the standalone Wwise audio test harness instead of the game.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_BANK",
        aliases: &["AW2_AUDIO_HARNESS_BANK"],
        kind: KnobType::String,
        default: "play_gui_caseboard_hand_up",
        description: "Default Wwise soundbank name loaded by audio harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_BANKS",
        aliases: &["AW2_AUDIO_HARNESS_BANKS"],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated list of Wwise banks loaded by audio oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_BYPASS_FX",
        aliases: &["AW2_AUDIO_HARNESS_BYPASS_FX"],
        kind: KnobType::Bool,
        default: "0",
        description: "Bypass Wwise effect plugins during oracle harness rendering.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_DIR",
        aliases: &["AW2_AUDIO_HARNESS_DIR"],
        kind: KnobType::Path,
        default: "",
        description: "Base directory for audio harness input/output artifacts.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_DRAIN_FRAMES",
        aliases: &["AW2_AUDIO_HARNESS_DRAIN_FRAMES"],
        kind: KnobType::U64,
        default: "938",
        description: "Maximum drain frames between oracle harness events.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_EVENT",
        aliases: &["AW2_AUDIO_HARNESS_EVENT"],
        kind: KnobType::String,
        default: "",
        description: "Wwise event name posted by the audio harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_EVENT_ID",
        aliases: &["AW2_AUDIO_HARNESS_EVENT_ID"],
        kind: KnobType::String,
        default: "",
        description: "Decimal or 0x-hex Wwise event ShortID posted by audio harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_EVENT_IDS",
        aliases: &["AW2_AUDIO_HARNESS_EVENT_IDS"],
        kind: KnobType::String,
        default: "",
        description: "Comma/space-separated Wwise event IDs (or @file) for oracle batch capture.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_MAX_FRAMES",
        aliases: &["AW2_AUDIO_HARNESS_MAX_FRAMES"],
        kind: KnobType::U64,
        default: "2813",
        description: "Maximum audio frames rendered per event in oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_MODE",
        aliases: &["AW2_AUDIO_HARNESS_MODE"],
        kind: KnobType::String,
        default: "realtime",
        description: "Audio harness execution mode (realtime, oracle, replay, etc.).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_OUT",
        aliases: &["AW2_AUDIO_HARNESS_OUT"],
        kind: KnobType::Path,
        default: "",
        description: "Output directory for audio oracle per-event WAV captures.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_RELOAD",
        aliases: &["AW2_AUDIO_HARNESS_RELOAD"],
        kind: KnobType::Bool,
        default: "1",
        description: "Reload Wwise banks between events in oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_REPEAT_MS",
        aliases: &["AW2_AUDIO_HARNESS_REPEAT_MS"],
        kind: KnobType::U64,
        default: "700",
        description: "Repeat interval in milliseconds in realtime audio harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_SECONDS",
        aliases: &["AW2_AUDIO_HARNESS_SECONDS"],
        kind: KnobType::F64,
        default: "4",
        description: "Duration in seconds for realtime audio harness playback.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_STOP_AFTER",
        aliases: &["AW2_AUDIO_HARNESS_STOP_AFTER"],
        kind: KnobType::U64,
        default: "",
        description: "Frame index at which to issue StopPlayingID in oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_STOP_CURVE",
        aliases: &["AW2_AUDIO_HARNESS_STOP_CURVE"],
        kind: KnobType::I32,
        default: "4",
        description: "Wwise AkCurveInterpolation ID for StopPlayingID in oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_STOP_MS",
        aliases: &["AW2_AUDIO_HARNESS_STOP_MS"],
        kind: KnobType::I32,
        default: "0",
        description: "Fade-out duration in milliseconds for StopPlayingID in oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_TAIL_FRAMES",
        aliases: &["AW2_AUDIO_HARNESS_TAIL_FRAMES"],
        kind: KnobType::U64,
        default: "0",
        description: "Extra tail frames rendered after silence in oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_TERM",
        aliases: &["AW2_AUDIO_HARNESS_TERM"],
        kind: KnobType::Bool,
        default: "1",
        description: "Call AK::SoundEngine::Term on audio harness exit.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_HARNESS_WARMUP",
        aliases: &["AW2_AUDIO_HARNESS_WARMUP"],
        kind: KnobType::String,
        default: "first",
        description: "Warm-up render mode in audio oracle harness.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_LOW_LATENCY",
        aliases: &["AW2_AUDIO_LOW_LATENCY"],
        kind: KnobType::Bool,
        default: "1",
        description: "Request minimum-latency CoreAudio IO buffer size.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_MIN_BUFFER_FRAMES",
        aliases: &["AW2_AUDIO_MIN_BUFFER_FRAMES"],
        kind: KnobType::U32,
        default: "0",
        description: "Minimum CoreAudio ring buffer floor in audio frames (0 = auto).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_RATE",
        aliases: &["AW2_AUDIO_RATE"],
        kind: KnobType::U32,
        default: "48000",
        description: "Override reported WASAPI mix sample rate in Hz.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_RT",
        aliases: &["AW2_AUDIO_RT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable Mach real-time workgroup/thread scheduling for audio threads.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_SIMULATE_UNPLUG",
        aliases: &["AW2_AUDIO_SIMULATE_UNPLUG"],
        kind: KnobType::String,
        default: "",
        description: "Test knob: '<secs>[,<replug_secs>]' to simulate audio device unplug.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_TAP",
        aliases: &["AW2_AUDIO_TAP"],
        kind: KnobType::Path,
        default: "",
        description: "Base path to tap mixed WASAPI audio output to WAV file.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_TEST_DEVICES",
        aliases: &["AW2_AUDIO_TEST_DEVICES"],
        kind: KnobType::Bool,
        default: "0",
        description: "Gate hardware CoreAudio device enumeration tests.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_AUDIO_VERBOSE",
        aliases: &["AW2_AUDIO_VERBOSE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable verbose logging in the wwise-audio CLI.",
        affects_aot_key: false,
        crates: &["wwise-audio"],
    },
    KnobSpec {
        name: "PORT_BANKS",
        aliases: &["AW2_BANKS"],
        kind: KnobType::Path,
        default: "",
        description: "Directory containing Wwise .bnk files for wwise-audio tests.",
        affects_aot_key: false,
        crates: &["wwise-audio"],
    },
    KnobSpec {
        name: "PORT_BARRIER_MASKS",
        aliases: &["AW2_BARRIER_MASKS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Track per-stage Metal 4 barrier masks on command lists.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_BARRIER_R2R_ELIDE",
        aliases: &["AW2_BARRIER_R2R_ELIDE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Elide redundant read-to-read resource transition barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_BARRIER_TIGHT_STAGES",
        aliases: &["AW2_BARRIER_TIGHT_STAGES"],
        kind: KnobType::Bool,
        default: "1",
        description: "Tighten Metal 4 barrier producer/consumer stage masks.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_BARRIER_TIGHT_V2",
        aliases: &["AW2_BARRIER_TIGHT_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 tight stage-mask barrier narrowing.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_BARRIER_UAV0_KEEP",
        aliases: &["AW2_BARRIER_UAV0_KEEP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Keep UAV-barrier-all transitions conservative across compute/graphics.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_BENCH_CHIP",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Override detected Apple Silicon chip key for `port-factory bench` baseline lookup.",
        affects_aot_key: false,
        crates: &["port-factory"],
    },
    KnobSpec {
        name: "PORT_BLIT_LIST_NARROW_V2",
        aliases: &["AW2_BLIT_LIST_NARROW_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 narrowing of blit-list boundary barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_BUILTIN_OVERRIDES",
        aliases: &["AW2_BUILTIN_OVERRIDES"],
        kind: KnobType::String,
        default: "",
        description: "Comma-separated builtin DLL routing overrides (0 disables all builtin overrides).",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_CALLVA_PROF",
        aliases: &["AW2_CALLVA_PROF"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable guest call_va profiler.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_CALLVA_PROF_SECS",
        aliases: &["AW2_CALLVA_PROF_SECS"],
        kind: KnobType::U64,
        default: "30",
        description: "Seconds between call_va profiler summaries.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_CALL_PROF",
        aliases: &["AW2_CALL_PROF"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable legacy host-call profiler.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_CHILD_CMDLINE",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Internal Windows command line passed to a guest-spawned child process.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_CHILD_CWD",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Internal Windows working directory passed to a guest-spawned child process.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_CHILD_ENV",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Internal encoded Windows environment block passed to a guest-spawned child process.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_CHILD_EXE_WIN",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Internal Windows executable path passed to a guest-spawned child process.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_CHILD_LOG_STDERR",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Echo runner log lines to stderr even in PORT_CHILD_STDIO=1 mode.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_CHILD_PROCESS",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Internal marker indicating a guest-spawned child runner process (disables disk AOT).",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_CHILD_STDIO",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Internal marker keeping stdout/stderr pipes intact for guest-spawned child processes.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_CLEAR_CS_EXACT_SYNC",
        aliases: &["AW2_CLEAR_CS_EXACT_SYNC"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact sync when compute clears follow prior compute/render work.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_CS_TILE4",
        aliases: &["AW2_CLEAR_CS_TILE4"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use 4x-tiled compute clear kernel for large render targets/UAVs.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_ISOLATE_COALESCE",
        aliases: &["AW2_CLEAR_ISOLATE_COALESCE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Coalesce isolated clear dispatches into batched clear encoders.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_KEEP_DSV",
        aliases: &["AW2_CLEAR_KEEP_DSV"],
        kind: KnobType::Bool,
        default: "0",
        description: "Keep open render pass across ClearDepthStencilView transitions.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_KEEP_SMART",
        aliases: &["AW2_CLEAR_KEEP_SMART"],
        kind: KnobType::Bool,
        default: "1",
        description: "Smartly preserve open render pass when clears do not conflict.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_OVERLAP_EXACT",
        aliases: &["AW2_CLEAR_OVERLAP_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact resource tracking to overlap independent GPU clears.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_PASS_FOLD_V2",
        aliases: &["AW2_CLEAR_PASS_FOLD_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 folding of clears into subsequent render-pass load actions.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_PRELIST_OVERLAP",
        aliases: &["AW2_CLEAR_PRELIST_OVERLAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allow pre-list GPU clears to overlap with preceding queue work.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CLEAR_ZERO_QB_COALESCE",
        aliases: &["AW2_CLEAR_ZERO_QB_COALESCE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Coalesce zero-clear queue barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_COMPACT_ICB",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Use 3 compact ICB vertex-buffer slot tiers (6/14/30) to shrink ICB memory.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CONTROL_DIR",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Directory for live-probe IPC command/telemetry files.",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader"],
    },
    KnobSpec {
        name: "PORT_COPY_QUEUE_LIGHT",
        aliases: &["AW2_COPY_QUEUE_LIGHT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use lightweight synchronization for Copy command-queue submissions.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CORECLR_PATH",
        aliases: &["AW2_CORECLR_PATH"],
        kind: KnobType::Path,
        default: "",
        description: "Override path to host libcoreclr.dylib for .NET hosting.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_CRASH_DIR",
        aliases: &["AW2_CRASH_DIR"],
        kind: KnobType::Path,
        default: "",
        description: "Override directory where crash reports and minidumps are written.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_CRASH_DISASM",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Include x86-64/ARM64 disassembly around faulting PC in crash reports.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_CRASH_DISASM_ADDRS",
        aliases: &[],
        kind: KnobType::List,
        default: "",
        description: "Extra comma-separated hex addresses to disassemble on crash.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_CS_BIND_CACHE",
        aliases: &["AW2_CS_BIND_CACHE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Cache compute pipeline and argument-table bindings across dispatches.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_BIND_INTRA_SAFE",
        aliases: &["AW2_CS_BIND_INTRA_SAFE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Preserve conservative intra-encoder compute bind state invalidation.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_FAST_BIND_EXACT",
        aliases: &["AW2_CS_FAST_BIND_EXACT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental exact fast-bind tracking for compute argument tables.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_INTRA_BARRIER_EXACT",
        aliases: &["AW2_CS_INTRA_BARRIER_EXACT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental exact intra-compute barrier placement.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_INTRA_BARRIER_V2",
        aliases: &["AW2_CS_INTRA_BARRIER_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 intra-compute barrier placement.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_MID_QB_DEFER_V3",
        aliases: &["AW2_CS_MID_QB_DEFER_V3"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v3 mid-compute queue barrier deferral.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_MID_QB_EXACT_V2",
        aliases: &["AW2_CS_MID_QB_EXACT_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Pair MSC compute reflection exactness with stage_gen mid-compute transition deferral.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_MID_QB_RES_DEFER_V5",
        aliases: &["AW2_CS_MID_QB_RES_DEFER_V5"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v5 per-resource mid-compute queue barrier deferral.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_MID_QB_TOUCH_DEFER",
        aliases: &["AW2_CS_MID_QB_TOUCH_DEFER"],
        kind: KnobType::Bool,
        default: "0",
        description: "Defer mid-compute queue barriers until touched resource is bound.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_MID_QB_UNPOISON_V4",
        aliases: &["AW2_CS_MID_QB_UNPOISON_V4"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v4 unpoisoning for deferred mid-compute queue barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_PSO_TG",
        aliases: &["AW2_CS_PSO_TG"],
        kind: KnobType::Bool,
        default: "1",
        description: "Specialize MTL4ComputePipelineDescriptor with DXIL [numthreads] hints.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_TO_PS_OVERLAP",
        aliases: &["AW2_CS_TO_PS_OVERLAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allow compute-to-pixel-shader overlap on disjoint resources.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_TO_PS_OVERLAP_V2",
        aliases: &["AW2_CS_TO_PS_OVERLAP_V2"],
        kind: KnobType::Bool,
        default: "1",
        description: "Allow v2 compute-to-pixel-shader overlap on proven disjoint resources.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_CS_UAV_PRODUCER_EXACT",
        aliases: &["AW2_CS_UAV_PRODUCER_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Track exact compute UAV producers for downstream barrier narrowing.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_D3D11",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable universal D3D11-on-Metal 4 runtime (0 disables).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_D3D12_TRACE",
        aliases: &["AW2_D3D12_TRACE"],
        kind: KnobType::Path,
        default: "",
        description: "Output path to record binary D3D12 call trace for d3d12-replay.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_D3D_CS",
        aliases: &["AW2_D3D_CS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Encode D3D12 command lists on dedicated worker thread (0 = inline).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_DEPOT_DOWNLOADER",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to DepotDownloader executable for Steam downloads.",
        affects_aot_key: false,
        crates: &["port-factory"],
    },
    KnobSpec {
        name: "PORT_DESC_HEAP_CACHED",
        aliases: &["AW2_DESC_HEAP_CACHED"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allocate GPU descriptor heaps with CPU-cached storage mode.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_DESC_TABLE_CS_FAST",
        aliases: &["AW2_DESC_TABLE_CS_FAST"],
        kind: KnobType::Bool,
        default: "0",
        description: "Fast-path compute descriptor-table binding without full rescan.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_DESC_TABLE_RES_EXACT",
        aliases: &["AW2_DESC_TABLE_RES_EXACT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Track exact descriptor-table referenced resources for barrier elision.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_DIAG_GPU_NAN",
        aliases: &[],
        kind: KnobType::U64,
        default: "0",
        description: "Check GPU render-target/buffer readbacks for NaNs every N frames.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_DIFF_LIMIT",
        aliases: &["AW2_DIFF_LIMIT"],
        kind: KnobType::Usize,
        default: "0",
        description: "Maximum number of functions to test during --diff-test (0 = unlimited).",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_DIFF_ONLY",
        aliases: &["AW2_DIFF_ONLY"],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated function name substrings to filter --diff-test.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_DIFF_SHARDS",
        aliases: &["AW2_DIFF_SHARDS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Include lifted shard functions during --diff-test.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_DIRECT_CALLS",
        aliases: &["AW2_DIRECT_CALLS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Allow direct host-to-guest call routing optimizations.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DISPLAY_RES",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Force reported logical Win32 display resolution as WxH.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_DLL_CHKSTK",
        aliases: &["AW2_DLL_CHKSTK"],
        kind: KnobType::Bool,
        default: "1",
        description: "Intercept guest __chkstk probes in DLL route fast path.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_FWD",
        aliases: &["AW2_DLL_FWD"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable native DLL forwarder resolution.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_LIFT",
        aliases: &["AW2_DLL_LIFT"],
        kind: KnobType::String,
        default: "",
        description: "Control statically lifted guest DLL activation.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_LIFT_COV",
        aliases: &["AW2_DLL_LIFT_COV"],
        kind: KnobType::Bool,
        default: "0",
        description: "Record coverage counts for statically lifted guest DLL functions.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_LIFT_SEL",
        aliases: &["AW2_DLL_LIFT_SEL"],
        kind: KnobType::String,
        default: "",
        description: "Diagnostic per-entry selection filter for A/B bisection of lifted DLLs.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_NATIVE",
        aliases: &["AW2_DLL_NATIVE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable native PE DLL loading and lifting hooks.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_DLL_PIN",
        aliases: &["AW2_DLL_PIN"],
        kind: KnobType::Bool,
        default: "1",
        description: "Attempt to map guest DLLs at their registered pinned base addresses.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_ROUTE",
        aliases: &["AW2_DLL_ROUTE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable DLL import-stub fast routing (0 runs stubs as guest code).",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_ROUTE_LOG",
        aliases: &["AW2_DLL_ROUTE_LOG"],
        kind: KnobType::Bool,
        default: "0",
        description: "Log DLL import-stub routing and dispatch decisions.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DLL_STACK_POS",
        aliases: &["AW2_DLL_STACK_POS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Optimize guest stack positioning on DLL route transitions.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_DPI",
        aliases: &["AW2_DPI"],
        kind: KnobType::U32,
        default: "0",
        description: "Override reported Win32 DPI (0 = host screen DPI).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_DXILCONV",
        aliases: &["AW2_DXILCONV"],
        kind: KnobType::Path,
        default: "",
        description: "Override path to libdxilconv.dylib for FXC SM 4/5 -> DXIL conversion.",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-factory", "port-loader"],
    },
    KnobSpec {
        name: "PORT_EH_CHILD",
        aliases: &["AW2_EH_CHILD"],
        kind: KnobType::String,
        default: "",
        description: "Internal child-test selector for SEH/FH4 unwinder subprocess tests.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EI_DISPATCH_EXACT",
        aliases: &["AW2_EI_DISPATCH_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact resource tracking for indirect DispatchIndirect barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_EI_ICB",
        aliases: &["AW2_EI_ICB"],
        kind: KnobType::Bool,
        default: "1",
        description: "Execute D3D12 ExecuteIndirect via GPU Metal Indirect Command Buffers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_EI_INPASS",
        aliases: &["AW2_EI_INPASS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Expand ExecuteIndirect ICBs inside the render pass vertex stage.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_EMPTY_ENC_ELIDE",
        aliases: &["AW2_EMPTY_ENC_ELIDE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Elide opening/closing empty Metal 4 command encoders.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_EMU_BADADDR",
        aliases: &["AW2_EMU_BADADDR"],
        kind: KnobType::String,
        default: "",
        description: "Set to 'ignore' to suppress fatal halt on out-of-range guest memory access.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EMU_CODECHECK",
        aliases: &["AW2_EMU_CODECHECK"],
        kind: KnobType::Bool,
        default: "0",
        description: "Hash guest code pages on block fill to detect self-modifying code.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EMU_FAST",
        aliases: &["AW2_EMU_FAST"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable FastOp decoded-IR interpreter tier (0 = generic iced-x86 interpreter).",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EMU_INT3",
        aliases: &["AW2_EMU_INT3"],
        kind: KnobType::String,
        default: "",
        description: "Set to 'return' to return instead of trapping on guest INT3 instructions.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EMU_PROF",
        aliases: &["AW2_EMU_PROF"],
        kind: KnobType::U32,
        default: "0",
        description: "Enable x86-64 interpreter/JIT execution profiler verbosity level (0 = off).",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EMU_PROF_FROM_FRAME",
        aliases: &["AW2_EMU_PROF_FROM_FRAME"],
        kind: KnobType::U64,
        default: "0",
        description: "Frame number after which emu_prof starts recording samples.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EMU_PROF_INTERVAL_S",
        aliases: &["AW2_EMU_PROF_INTERVAL_S"],
        kind: KnobType::U64,
        default: "10",
        description: "Interval in seconds between periodic emu-prof log dumps.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_EMU_VERIFY",
        aliases: &["AW2_EMU_VERIFY"],
        kind: KnobType::Bool,
        default: "0",
        description: "Run live differential verification comparing JIT output against interpreter.",
        affects_aot_key: false,
        crates: &["port-factory", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_ENABLE_NVAPI",
        aliases: &["AW2_ENABLE_NVAPI"],
        kind: KnobType::Bool,
        default: "0",
        description: "Expose nvapi64.dll stub/bridge instead of reporting unsupported.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_ENC_STATS",
        aliases: &["AW2_ENC_STATS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable Metal 4 encoder and barrier counters logging.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_EXEC_MODE",
        aliases: &[],
        kind: KnobType::Enum(&["auto", "jit", "static-aot", "interp"]),
        default: "auto",
        description: "Executable codegen policy: auto, jit, static-aot, or interp.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EXE_GUEST_LOG",
        aliases: &["AW2_EXE_GUEST_LOG"],
        kind: KnobType::Bool,
        default: "0",
        description: "Warn once per distinct main-exe IP that executes in guest mode.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_EXIT_CODE_FD",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Internal pipe file descriptor carrying exact 32-bit Windows exit code to parent.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_EXIT_HOLD",
        aliases: &["AW2_EXIT_HOLD"],
        kind: KnobType::U32,
        default: "0",
        description: "Seconds to hold process alive before ExitProcess completes.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_F16_SATURATE",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Override narrow-float (F16/RG11B10) overflow saturation sanitizer (1/f16/0).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_FACTORY_PORT",
        aliases: &[],
        kind: KnobType::U32,
        default: "7860",
        description: "HTTP listen port for the local Port Factory launcher UI.",
        affects_aot_key: false,
        crates: &["port-factory"],
    },
    KnobSpec {
        name: "PORT_FACTORY_WORKSPACE",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Workspace root path passed to the Port Factory launcher UI.",
        affects_aot_key: false,
        crates: &["port-factory"],
    },
    KnobSpec {
        name: "PORT_FAIL_ON_UNIMPL_OP",
        aliases: &[],
        kind: KnobType::Bool,
        default: "",
        description: "Abort with exit code 86 on unimplemented x86-64 instructions (default on for CLI bring-up).",
        affects_aot_key: false,
        crates: &["port-loader", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_FAST_BIND_PSO",
        aliases: &["AW2_FAST_BIND_PSO"],
        kind: KnobType::Bool,
        default: "1",
        description: "Dirty-track MTL4ArgumentTable bindings and specialize internal compute PSOs.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_FAST_ENC_STATE",
        aliases: &["AW2_FAST_ENC_STATE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Cache active encoder pipeline/root/descriptor state to skip redundant binds.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_FENCE_ALL",
        aliases: &["AW2_FENCE_ALL"],
        kind: KnobType::Bool,
        default: "0",
        description: "Install memory/register fences around every replacement hook.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_FENCE_SIGNAL_COALESCE",
        aliases: &["AW2_FENCE_SIGNAL_COALESCE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Coalesce mid-frame Direct queue ID3D12CommandQueue::Signal calls.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_FFMPEG_LIB_DIR",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Directory containing FFmpeg shared libraries for MediaDecoders.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_FRAME_CSV",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Write per-frame Present timing CSV (frame, elapsed_ms, dt_ms, present_ms, sync, gpu_busy_ms) to this path.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_FULLSCREEN",
        aliases: &["AW2_FULLSCREEN"],
        kind: KnobType::Bool,
        default: "0",
        description: "Start in borderless fullscreen takeover mode on macOS.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_GAMEPAD",
        aliases: &["AW2_GAMEPAD"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable Apple GameController XInput/DirectInput gamepad bridge.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_GAME_DIR",
        aliases: &["AW2_GAME_DIR"],
        kind: KnobType::Path,
        default: "",
        description: "Host directory containing the target Windows game executable and DLLs.",
        affects_aot_key: false,
        crates: &["d3d12-metal", "dxil-metal", "pe-image", "port-factory", "port-loader", "wwise-audio"],
    },
    KnobSpec {
        name: "PORT_GFX_DESC_TABLE_EXACT",
        aliases: &["AW2_GFX_DESC_TABLE_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact graphics descriptor-table tracking for hazard analysis.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GFX_LAZY_ICB",
        aliases: &["AW2_GFX_LAZY_ICB"],
        kind: KnobType::Bool,
        default: "1",
        description: "Compile graphics PSOs without ICB support until ExecuteIndirect actually uses them.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GFX_OVERLAP_CAPPED",
        aliases: &["AW2_GFX_OVERLAP_CAPPED"],
        kind: KnobType::Bool,
        default: "0",
        description: "Cap overlapping graphics passes to bound tile/memory pressure.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GFX_OVERLAP_FRAG_CAP",
        aliases: &["AW2_GFX_OVERLAP_FRAG_CAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Cap fragment-stage overlap across consecutive graphics encoders.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GFX_TO_PS_OVERLAP",
        aliases: &["AW2_GFX_TO_PS_OVERLAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allow graphics-to-pixel-shader overlap on disjoint resources.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GFX_TO_PS_OVERLAP_V2",
        aliases: &["AW2_GFX_TO_PS_OVERLAP_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 graphics-to-pixel-shader overlap.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GPU_CAPTURE",
        aliases: &["AW2_GPU_CAPTURE"],
        kind: KnobType::Path,
        default: "",
        description: "Output .gputrace path for programmatic Metal GPU frame capture.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GPU_CAPTURE_AT",
        aliases: &["AW2_GPU_CAPTURE_AT"],
        kind: KnobType::U64,
        default: "600",
        description: "Frame index at which to start programmatic Metal GPU capture.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_GPU_CAPTURE_FRAMES",
        aliases: &["AW2_GPU_CAPTURE_FRAMES"],
        kind: KnobType::U64,
        default: "1",
        description: "Number of consecutive frames to include in Metal GPU capture.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_HEADLESS",
        aliases: &["AW2_HEADLESS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Disable AppKit windowing and run headless without GUI.",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_HID",
        aliases: &["AW2_HID"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable IOHIDManager DualSense/DS4 and SetupDi HID device enumeration.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_HITCH_CPU_REPORT",
        aliases: &["AW2_HITCH_CPU_REPORT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable periodic CPU hitch profiler summary reports.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_HITCH_CPU_REPORT_S",
        aliases: &["AW2_HITCH_CPU_REPORT_S"],
        kind: KnobType::U64,
        default: "5",
        description: "Interval in seconds between periodic hitch CPU reports.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_HITCH_LOG",
        aliases: &["AW2_HITCH_LOG"],
        kind: KnobType::Path,
        default: "",
        description: "File path for frame hitch and stall diagnostic logs.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_HITCH_MS",
        aliases: &["AW2_HITCH_MS"],
        kind: KnobType::U64,
        default: "50",
        description: "Frame duration threshold in milliseconds to log a hitch.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_HITCH_OP_MS",
        aliases: &["AW2_HITCH_OP_MS"],
        kind: KnobType::U64,
        default: "20",
        description: "Single operation threshold in milliseconds included in hitch traces.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_HOOK_LINT",
        aliases: &["AW2_HOOK_LINT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Run consistency lint checks on registered replacement hooks at load.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_HOOK_MAX_VA",
        aliases: &["AW2_HOOK_MAX_VA"],
        kind: KnobType::String,
        default: "",
        description: "Upper guest VA bound (hex) for replacement hook bisection.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_HOOK_MIN_VA",
        aliases: &["AW2_HOOK_MIN_VA"],
        kind: KnobType::String,
        default: "",
        description: "Lower guest VA bound (hex) for replacement hook bisection.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_HOOK_SHARDS",
        aliases: &["AW2_HOOK_SHARDS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable compiled-in replacement shard hooks when active.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_HOOK_STATS",
        aliases: &["AW2_HOOK_STATS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Count invocations per replacement hook.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_HOOK_STATS_FILE",
        aliases: &["AW2_HOOK_STATS_FILE"],
        kind: KnobType::Path,
        default: "",
        description: "Output file path for periodic hook invocation statistics.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_HOSTFXR_PATH",
        aliases: &["AW2_HOSTFXR_PATH"],
        kind: KnobType::Path,
        default: "",
        description: "Override path to host libhostfxr.dylib for .NET hosting.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_IB_BINDLESS_WAR",
        aliases: &["AW2_IB_BINDLESS_WAR"],
        kind: KnobType::Bool,
        default: "0",
        description: "Engine-specific bindless WAR intra-barrier shortcut (OFF by default).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_IB_EXACT_OBL",
        aliases: &["AW2_IB_EXACT_OBL"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact oblique/resource tracking for intra-encoder barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_IB_SUBSTAGE_NARROW",
        aliases: &["AW2_IB_SUBSTAGE_NARROW"],
        kind: KnobType::Bool,
        default: "1",
        description: "Narrow intra-encoder barriers using sub-stage producer/consumer sets.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_IB_TIGHT",
        aliases: &["AW2_IB_TIGHT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Narrow intra-pass/intra-encoder barriers based on tracked resource hazards.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_IB_UAV0_UNBOUNDED_EXACT",
        aliases: &["AW2_IB_UAV0_UNBOUNDED_EXACT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental exact tracking for unbounded UAV0 intra-encoder barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_ICB_BATCH",
        aliases: &["AW2_ICB_BATCH"],
        kind: KnobType::Bool,
        default: "1",
        description: "Batch consecutive ExecuteIndirect draws into shared ICB runs.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_ICB_ENC_BARRIER_EXACT",
        aliases: &["AW2_ICB_ENC_BARRIER_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact encoder barriers around ICB execution.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_ICB_FAST_V3",
        aliases: &["AW2_ICB_FAST_V3"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use fragment-aware vectorized ICB expansion with hoisted vertex barrier.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_ICB_PREPASS_COMPUTE",
        aliases: &["AW2_ICB_PREPASS_COMPUTE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Expand batched ICB draws in a compute pre-pass before opening render pass (default 1 on iOS).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_INPUT_SCRIPT",
        aliases: &["AW2_AUTO_PLAY_SCRIPT"],
        kind: KnobType::Path,
        default: "",
        description: "Path to scripted input playback file driven on swapchain Present.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_IOS_GPU_BUDGET_CAP_MB",
        aliases: &[],
        kind: KnobType::U64,
        default: "0",
        description: "Cap reported GPU memory budget on iOS in MiB (0 = auto).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_IOS_ICB_PASS_CAP",
        aliases: &[],
        kind: KnobType::U32,
        default: "65536",
        description: "Maximum indirect commands per open Metal render pass on iOS before splitting.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_IOS_JETSAM_HEADROOM_MB",
        aliases: &[],
        kind: KnobType::U64,
        default: "0",
        description: "Reserved Jetsam memory headroom on iOS in MiB (0 = auto).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_IOS_NORMAL_QOS",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Keep elevated threads at normal QoS on iOS to reduce thermal throttling.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_IOS_PRESSURE_SYNC",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable emergency GPU drain when iOS Jetsam headroom is critical.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_IOS_PRESSURE_SYNC_MB",
        aliases: &[],
        kind: KnobType::U64,
        default: "512",
        description: "Available memory threshold (MiB) triggering iOS GPU pressure drain.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_IOS_PSO_RESET_BATCH",
        aliases: &[],
        kind: KnobType::U64,
        default: "0",
        description: "Reset MTL4Compiler scratch after N cold PSO compiles (default 12 on iOS, 0 on macOS).",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_JIT",
        aliases: &["AW2_JIT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable x86-64 to ARM64 block JIT translator (0 = interpreter only).",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_ALLOW",
        aliases: &[],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated guest VA ranges allowed to run in ARM64 JIT code.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_BENCH_DLL",
        aliases: &["AW2_JIT_BENCH_DLL"],
        kind: KnobType::Path,
        default: "",
        description: "PE DLL path for the JIT benchmark suite.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_BENCH_MAX",
        aliases: &["AW2_JIT_BENCH_MAX"],
        kind: KnobType::Usize,
        default: "400",
        description: "Maximum blocks translated in the JIT benchmark.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_BENCH_OUT",
        aliases: &["AW2_JIT_BENCH_OUT"],
        kind: KnobType::Path,
        default: "/tmp/aw2_jit_bench",
        description: "Output directory for JIT benchmark artifacts.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_BENCH_RVAS",
        aliases: &["AW2_JIT_BENCH_RVAS"],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated hex RVAs for JIT benchmark roots.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_BENCH_TSO",
        aliases: &["AW2_JIT_BENCH_TSO"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable TSO mode during the JIT benchmark.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_BUDGET_BACKEDGE",
        aliases: &["AW2_JIT_BUDGET_BACKEDGE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Check instruction budget only at backedges, RAS rets, and lookup stubs.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_BUDGET_LIGHT",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Elide instruction-budget decrements on forward-only blocks.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_COV_DEPTH",
        aliases: &["AW2_JIT_COV_DEPTH"],
        kind: KnobType::U32,
        default: "128",
        description: "CFG closure depth for the JIT coverage test.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_COV_DLL",
        aliases: &["AW2_JIT_COV_DLL"],
        kind: KnobType::Path,
        default: "",
        description: "PE DLL path for the JIT coverage test.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_COV_RVAS",
        aliases: &["AW2_JIT_COV_RVAS"],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated hex RVAs for the JIT coverage test.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_FAST_NAN",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Use native IEEE-754 ARM64 FP NaN semantics without per-op x86 QNaN tiebreak.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_FLAGLAZY",
        aliases: &["AW2_JIT_FLAGLAZY"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use lazy x86 flag materialization in ARM64 JIT blocks.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_FLAGS_DEAD_AT_CALL",
        aliases: &["AW2_JIT_FLAGS_DEAD_AT_CALL"],
        kind: KnobType::U32,
        default: "3",
        description: "Bitmask of x86 flag deadness at call (bit 0) and ret (bit 1).",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_FUZZ_TRACE",
        aliases: &["AW2_JIT_FUZZ_TRACE"],
        kind: KnobType::Path,
        default: "",
        description: "Trace output path for JIT randomized fuzz tests.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_HOSTCALL",
        aliases: &["AW2_JIT_HOSTCALL"],
        kind: KnobType::Bool,
        default: "1",
        description: "Invoke host functions directly from translated code via host_xfer stub.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_HOT",
        aliases: &["AW2_JIT_HOT"],
        kind: KnobType::U32,
        default: "2",
        description: "Block execution threshold before JIT translation triggers.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_HW_RANGE",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Rely on hardware MMU faults instead of software pointer range checks.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_IC",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable per-site 2-way polymorphic inline caches for indirect call/jmp/ret.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_LAST_BITS",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Maintain Cpu::last_bits for Win64 abi_gpr masking of DF_GAME_REPL hooks.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_LSE2_TRAP",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Rely on FEAT_LSE2 hardware + SIGBUS handler for unaligned TSO accesses.",
        affects_aot_key: true,
        crates: &["port-loader", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_LZ_ONLY",
        aliases: &["AW2_JIT_LZ_ONLY"],
        kind: KnobType::Usize,
        default: "",
        description: "Run only a single lazy-flag fuzz combo index.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_LZ_SEED",
        aliases: &["AW2_JIT_LZ_SEED"],
        kind: KnobType::String,
        default: "",
        description: "Hex RNG seed override for lazy-flag JIT fuzz tests.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_MEMCHK",
        aliases: &["AW2_JIT_MEMCHK"],
        kind: KnobType::U32,
        default: "15",
        description: "Bitmask enabling cheaper guest memory range and alignment checks.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_PLAINCALL",
        aliases: &["AW2_JIT_PLAINCALL"],
        kind: KnobType::Bool,
        default: "1",
        description: "Inline direct calls to DF_PLAIN builtins in translated code.",
        affects_aot_key: true,
        crates: &["port-loader", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_PROFILE",
        aliases: &["AW2_JIT_PROFILE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Record and dump translated block execution profiles for AOT building.",
        affects_aot_key: false,
        crates: &["port-loader", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_PUSHPOP",
        aliases: &["AW2_JIT_PUSHPOP"],
        kind: KnobType::Bool,
        default: "1",
        description: "Emit plain pre/post-indexed push/pop and fused stp/ldp on guest stack.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_RAS",
        aliases: &["AW2_JIT_RAS"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable shadow return-address stack (RAS) for translated guest call/ret.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_RETLIT",
        aliases: &["AW2_JIT_RETLIT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Load 64-bit code/import constants from per-block literal pools instead of movz/movk.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_RO_PLAIN",
        aliases: &["AW2_JIT_RO_PLAIN"],
        kind: KnobType::Bool,
        default: "1",
        description: "Emit plain ldr for reads from read-only non-executable PE sections.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_SKIP",
        aliases: &[],
        kind: KnobType::List,
        default: "",
        description: "Comma-separated guest VA ranges forced onto the interpreter.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_STACK_TSO",
        aliases: &["AW2_JIT_STACK_TSO"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enforce TSO ordering on guest stack-relative [rsp+disp] accesses.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_STATS",
        aliases: &["AW2_JIT_STATS"],
        kind: KnobType::U64,
        default: "0",
        description: "Enable JIT/AOT runtime execution counters and periodic stats reporting.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_SX_ONLY",
        aliases: &["AW2_JIT_SX_ONLY"],
        kind: KnobType::Usize,
        default: "",
        description: "Run only a single SIMD/SSE fuzz combo index.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_SX_SEED",
        aliases: &["AW2_JIT_SX_SEED"],
        kind: KnobType::String,
        default: "",
        description: "Hex RNG seed override for SIMD/SSE JIT fuzz tests.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_THUNK_ELIDE",
        aliases: &["AW2_JIT_THUNK_ELIDE"],
        kind: KnobType::U32,
        default: "0",
        description: "Bitmask allowing guest callers to bypass registered host hook thunks.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_TSO",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Enforce x86 TSO memory ordering (ldapr/stlr) on non-atomic guest loads/stores.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_XMMREG",
        aliases: &["AW2_JIT_XMMREG"],
        kind: KnobType::Bool,
        default: "1",
        description: "Keep hot XMM registers live in ARM64 NEON V-registers across instructions.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JIT_XMM_PIN",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Pin low XMM registers in callee-saved ARM64 V-registers across block entries.",
        affects_aot_key: true,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JOB_MARK",
        aliases: &["AW2_JOB_MARK"],
        kind: KnobType::String,
        default: "",
        description: "Optional marker tag written into job profiler logs.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JOB_PROF",
        aliases: &["AW2_JOB_PROF"],
        kind: KnobType::String,
        default: "",
        description: "Enable worker-thread job profiling output spec.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_JOB_PROF_KINDS",
        aliases: &["AW2_JOB_PROF_KINDS"],
        kind: KnobType::String,
        default: "",
        description: "Filter job profiler to specific comma-separated job kinds.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_LEGACY_BARRIERS",
        aliases: &["AW2_LEGACY_BARRIERS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Use conservative legacy full-stage barriers instead of tracked stage masks.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_LIFT_TRAP_CHILD",
        aliases: &["AW2_LIFT_TRAP_CHILD"],
        kind: KnobType::String,
        default: "",
        description: "Internal child-test selector for lifted-op trap subprocess tests.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_LIST_BARRIER_TIGHT",
        aliases: &["AW2_LIST_BARRIER_TIGHT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Tighten cross-list barrier stage masks at command-list boundaries.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_LIST_BOUNDARY_OVERLAP",
        aliases: &["AW2_LIST_BOUNDARY_OVERLAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allow independent work to overlap across command-list boundaries.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_LIST_START_RENDER_SPLIT",
        aliases: &["AW2_LIST_START_RENDER_SPLIT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Split render pass barriers conservatively at command-list start.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_LIST_START_SUBSTAGE",
        aliases: &["AW2_LIST_START_SUBSTAGE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use sub-stage barrier masks at command-list start.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_LOCK_CURSOR",
        aliases: &["AW2_LOCK_CURSOR"],
        kind: KnobType::Bool,
        default: "0",
        description: "Lock cursor inside the game window.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_LOCK_DISPLAY_MODE",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Lock display mode policy (windowed, fullscreen, borderless, 0/off).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_LOG_APPEND",
        aliases: &["AW2_LOG_APPEND"],
        kind: KnobType::Bool,
        default: "0",
        description: "Internal flag set across re-execs to append to the existing boot log.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_LOG_DIR",
        aliases: &["AW2_LOG_DIR"],
        kind: KnobType::Path,
        default: "",
        description: "Override directory for boot and runtime log files.",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_LSTACK_ARENA",
        aliases: &["AW2_LSTACK_ARENA"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use pooled arena allocator for lifted-function guest shadow stacks.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_LSTACK_CHECK",
        aliases: &["AW2_LSTACK_CHECK"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable guard/canary checks on lifted-function shadow stack frames.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_MANIFEST",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Path to declarative game.toml manifest.",
        affects_aot_key: false,
        crates: &["port-factory", "port-manifest"],
    },
    KnobSpec {
        name: "PORT_MAX_DISPLAY_WIDTH",
        aliases: &["PORT_IOS_MAX_WIDTH"],
        kind: KnobType::U32,
        default: "0",
        description: "Clamp reported display width in pixels (0 = unclamped).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_MAX_PSO_COMPILES",
        aliases: &[],
        kind: KnobType::Usize,
        default: "0",
        description: "Cap concurrent cold MTL4Compiler PSO compilation jobs (0 = auto).",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_MEMDIAG",
        aliases: &["AW2_MEMDIAG"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable verbose Win32 virtual memory allocation diagnostics.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_MESH_SHADERS",
        aliases: &["AW2_MESH_SHADERS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Expose D3D12_MESH_SHADER_TIER_1 support on Metal 4.",
        affects_aot_key: false,
        crates: &["d3d12-metal", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_METALFX",
        aliases: &["AW2_METALFX"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable Metal 4 MetalFX spatial upscaling (0 falls back to bilinear).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_METALFX_TEMPORAL",
        aliases: &["AW2_METALFX_TEMPORAL"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable Metal 4 MetalFX temporal upscaling.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_MID_QB_STAGESYNC",
        aliases: &["AW2_MID_QB_STAGESYNC"],
        kind: KnobType::Bool,
        default: "1",
        description: "Defer mid-queue-barrier stage sync until dependent consumer stage.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_MIN_PHYS_MEM_GB",
        aliases: &[],
        kind: KnobType::U64,
        default: "0",
        description: "Floor on physical RAM (GiB) reported by GlobalMemoryStatusEx.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_MOLTENVK_DYLIB",
        aliases: &["MOLTENVK_DYLIB"],
        kind: KnobType::Path,
        default: "",
        description: "Override path to libMoltenVK.dylib (alias: MOLTENVK_DYLIB).",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_MONO_PATH",
        aliases: &["AW2_MONO_PATH"],
        kind: KnobType::Path,
        default: "",
        description: "Override path to host Mono runtime dylib.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_MOUNTS",
        aliases: &["AW2_MOUNTS"],
        kind: KnobType::List,
        default: "",
        description: "Semicolon-separated extra Windows=host directory mounts.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_MSC_COMPUTE_ACCESS",
        aliases: &["AW2_MSC_COMPUTE_ACCESS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Refine ComputeAccess slots/classes from MSC UsedResources/ResourceUsages JSON.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_MSC_STAGE_USAGE",
        aliases: &["AW2_MSC_STAGE_USAGE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Refine graphics/mesh/compute stage usage from MSC ResourceUsages JSON.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_NAN_BOUNDARY",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Check float arguments/outputs for NaN at native middleware call boundaries.",
        affects_aot_key: false,
        crates: &["port-factory", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_NAN_TRACE",
        aliases: &[],
        kind: KnobType::String,
        default: "",
        description: "Configure universal NaN-birth, memory-watch, and host-boundary tracer.",
        affects_aot_key: false,
        crates: &["port-factory", "x86-runtime"],
    },
    KnobSpec {
        name: "PORT_NO_ACTIVATE",
        aliases: &["AW2_NO_ACTIVATE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Suppress macOS foreground activation when opening windows.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_NO_BASE_RETRY",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Disable preferred ImageBase ASLR re-exec retries.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_NUM_CPUS",
        aliases: &[],
        kind: KnobType::U32,
        default: "0",
        description: "Override logical CPU count reported to Windows (0 = host CPU count).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_OFFLINE",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Force offline networking mode for directory/auth/HTTP middleware.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_PASS_TIMING",
        aliases: &["AW2_PASS_TIMING"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable per-pass GPU timestamp profiling on Metal 4.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PC_SAMPLE",
        aliases: &["AW2_PC_SAMPLE"],
        kind: KnobType::U64,
        default: "0",
        description: "Guest PC sampling interval in microseconds (0 = disabled).",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_PC_SAMPLE_REPORT_S",
        aliases: &["AW2_PC_SAMPLE_REPORT_S"],
        kind: KnobType::U64,
        default: "10",
        description: "Interval in seconds between PC-sample hotspot reports.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_PERF_LOG",
        aliases: &["PORT_IOS_PERF_LOG"],
        kind: KnobType::Path,
        default: "",
        description: "Path to write periodic frame-time and memory performance CSV/log.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_PNG_SELFTEST",
        aliases: &["AW2_PNG_SELFTEST"],
        kind: KnobType::Path,
        default: "",
        description: "Write a synthetic PNG self-test image to this path.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PRECISE_SLEEP",
        aliases: &["AW2_PRECISE_SLEEP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Use high-precision spin-assisted wait for short Win32 Sleep calls.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_PRECISE_TIMESTAMPS",
        aliases: &["AW2_PRECISE_TIMESTAMPS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Request Precise MTL4TimestampGranularity (may split encoders).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PRESENT_BLIT_NARROW",
        aliases: &["AW2_PRESENT_BLIT_NARROW"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use Blit->Blit barrier on dedicated swapchain staging/present queues.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PRESENT_CAPTURE",
        aliases: &["AW2_PRESENT_CAPTURE"],
        kind: KnobType::Path,
        default: "",
        description: "Base path to dump presented swapchain frames as PNG images.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PRESENT_CAPTURE_AT",
        aliases: &["AW2_PRESENT_CAPTURE_AT"],
        kind: KnobType::List,
        default: "1",
        description: "Comma-separated frame indices or ranges to capture with PORT_PRESENT_CAPTURE.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PRESENT_OFF_MAIN",
        aliases: &["AW2_PRESENT_OFF_MAIN"],
        kind: KnobType::Bool,
        default: "1",
        description: "Offload swapchain backbuffer-to-staging blit onto a dedicated queue.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PREWARM_SHADERS",
        aliases: &[],
        kind: KnobType::String,
        default: "auto",
        description: "Cold-device shader/PSO auto-prewarm mode: auto, 1/force, or 0/off.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_PREWARM_SHADER_THREADS",
        aliases: &[],
        kind: KnobType::Usize,
        default: "0",
        description: "Parallel worker count for cold-device shader/PSO prewarming (0 = auto).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PREWARM_TRACE",
        aliases: &["AW2_PREWARM_TRACE"],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable verbose per-shader trace output in dxil-metal prewarm CLI.",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_PROBE",
        aliases: &["AW2_PROBE"],
        kind: KnobType::String,
        default: "",
        description: "Comma-separated guest VAs or hook names to install diagnostic probes on.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_PSO_CACHE",
        aliases: &["AW2_PSO_CACHE"],
        kind: KnobType::Path,
        default: "",
        description: "Writable Tier-2 MTL4Archive PSO cache directory (0/off disables).",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-loader"],
    },
    KnobSpec {
        name: "PORT_PSO_MANIFEST",
        aliases: &["AW2_PSO_MANIFEST"],
        kind: KnobType::Path,
        default: "",
        description: "Writable directory for GPU/OS-independent PSO recipe manifests.",
        affects_aot_key: false,
        crates: &["d3d12-metal", "port-loader"],
    },
    KnobSpec {
        name: "PORT_PSO_MANIFEST_FALLBACK",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Read-only fallback directory for bundled PSO recipe manifests.",
        affects_aot_key: false,
        crates: &["d3d12-metal", "port-factory"],
    },
    KnobSpec {
        name: "PORT_PSO_MAX_ARCHIVES",
        aliases: &["AW2_PSO_MAX_ARCHIVES"],
        kind: KnobType::Usize,
        default: "16",
        description: "Maximum number of rotated MTL4Archive files kept in Tier-2 PSO cache.",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-loader"],
    },
    KnobSpec {
        name: "PORT_PS_SRV_CLASS_EXACT",
        aliases: &["AW2_PS_SRV_CLASS_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Classify pixel-shader SRVs by resource class for tighter barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PS_TO_CS_HK_OVERLAP",
        aliases: &["AW2_PS_TO_CS_HK_OVERLAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allow pixel-to-compute overlap using SM 6.6 handle-kind proofs.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PS_TO_CS_OVERLAP",
        aliases: &["AW2_PS_TO_CS_OVERLAP"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allow pixel-shader-to-compute overlap on disjoint resources.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_PS_TO_CS_OVERLAP_V2",
        aliases: &["AW2_PS_TO_CS_OVERLAP_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 pixel-shader-to-compute overlap.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_QPC_ISB",
        aliases: &["AW2_QPC_ISB"],
        kind: KnobType::Bool,
        default: "0",
        description: "Emit ISB barrier around QueryPerformanceCounter timer reads.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_QUEUE_START_STAGE_EXACT",
        aliases: &["AW2_QUEUE_START_STAGE_EXACT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Use exact producer stage mask at command-queue start boundaries.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RAW_MOUSE",
        aliases: &["AW2_RAW_MOUSE"],
        kind: KnobType::String,
        default: "auto",
        description: "Raw mouse input backend mode (auto, gcmouse, nsevent, off).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_RAYTRACING",
        aliases: &["AW2_RAYTRACING"],
        kind: KnobType::Bool,
        default: "0",
        description: "Expose DXR 1.1 ray tracing tier support on Metal 4.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_REFRESH_HZ",
        aliases: &["AW2_REFRESH_HZ"],
        kind: KnobType::U32,
        default: "0",
        description: "Override reported display refresh rate in Hz (0 = host display rate).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_REGISTRY_TSV",
        aliases: &["AW2_REGISTRY_TSV"],
        kind: KnobType::Path,
        default: "",
        description: "Path to additional Windows registry key/value TSV file to import.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_RENDER_DOM_EXACT",
        aliases: &["AW2_RENDER_DOM_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact render-pass domain tracking for cross-pass hazards.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RENDER_SPLIT_STAGE_SYNC",
        aliases: &["AW2_RENDER_SPLIT_STAGE_SYNC"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact stage sync when splitting a render pass.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_REPORT_JSON",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Path to write a structured JSON boot report on port-runner exit.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_RETINA",
        aliases: &["AW2_RETINA"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use native Retina backing pixels (0 maps 1 Win32 pixel = 1 macOS point).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_REUSE_ICB",
        aliases: &["PORT_IOS_REUSE_ICB"],
        kind: KnobType::Bool,
        default: "0",
        description: "Reuse IcbRing and GPU scratch across closed render passes (default 1 on iOS).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RO_DSV_DONTCARE",
        aliases: &["AW2_RO_DSV_DONTCARE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use DontCare store action for read-only depth/stencil attachments.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RP_DISCARD",
        aliases: &["AW2_RP_DISCARD"],
        kind: KnobType::Bool,
        default: "0",
        description: "Allow DiscardResource to convert render-pass loads/stores to DontCare.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RP_KEEP_WRITE",
        aliases: &["AW2_RP_KEEP_WRITE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Keep render pass open across non-conflicting write transitions.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RP_KEEP_WRITE_EXACT",
        aliases: &["AW2_RP_KEEP_WRITE_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact resource write sets when keeping render passes open.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RP_SPLIT_EXACT_KEEP",
        aliases: &["AW2_RP_SPLIT_EXACT_KEEP"],
        kind: KnobType::Bool,
        default: "1",
        description: "Preserve exact attachment load/store actions across split render passes.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_RUNNER",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to host port-runner binary used by port-factory.",
        affects_aot_key: false,
        crates: &["port-factory"],
    },
    KnobSpec {
        name: "PORT_RUNNER_EXE",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to port-runner executable used when spawning Windows child processes.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_SCREENSHOT",
        aliases: &["AW2_SCREENSHOT"],
        kind: KnobType::Path,
        default: "",
        description: "Save a window screenshot PNG to this path after startup delay.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_SCREENSHOT_DELAY_MS",
        aliases: &["AW2_SCREENSHOT_DELAY_MS"],
        kind: KnobType::U64,
        default: "3000",
        description: "Delay in milliseconds after window show before capturing PORT_SCREENSHOT.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_SEM_CASCADE",
        aliases: &["AW2_SEM_CASCADE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use cascading single-waiter wakeup for Win32 semaphores.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_SHADERD",
        aliases: &["AW2_SHADERD"],
        kind: KnobType::Path,
        default: "",
        description: "Override path to out-of-process dxil-metald shader compiler worker.",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-factory", "port-loader"],
    },
    KnobSpec {
        name: "PORT_SHADERD_TIMEOUT_SECS",
        aliases: &["AW2_SHADERD_TIMEOUT_SECS"],
        kind: KnobType::F64,
        default: "60",
        description: "Per-shader timeout in seconds for dxil-metald worker requests (0 = none).",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_SHADERD_WORKERS",
        aliases: &["AW2_SHADERD_WORKERS"],
        kind: KnobType::Usize,
        default: "0",
        description: "Number of dxil-metald worker processes (0 = available parallelism).",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_SHADER_BACKEND",
        aliases: &["AW2_SHADER_BACKEND"],
        kind: KnobType::Enum(&["auto", "inprocess", "helper", "cache-only"]),
        default: "auto",
        description: "Shader conversion backend: auto, inprocess, helper, or cache-only.",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_SHADER_CACHE",
        aliases: &["AW2_SHADER_CACHE"],
        kind: KnobType::Path,
        default: "",
        description: "Writable Tier-1 .metallib shader disk cache directory (0 disables).",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-factory", "port-loader"],
    },
    KnobSpec {
        name: "PORT_SHADER_CACHE_FALLBACK",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Read-only fallback Tier-1 shader cache directory (e.g. bundled .app).",
        affects_aot_key: false,
        crates: &["d3d12-metal", "dxil-metal", "port-factory"],
    },
    KnobSpec {
        name: "PORT_SHADER_CACHE_MEM_MB",
        aliases: &[],
        kind: KnobType::U64,
        default: "512",
        description: "In-memory shader cache budget in MiB (default 512 macOS, 32 iOS).",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_SHADER_FINGERPRINT",
        aliases: &["AW2_SHADER_FINGERPRINT"],
        kind: KnobType::String,
        default: "",
        description: "Override Tier-1 shader cache namespace fingerprint.",
        affects_aot_key: false,
        crates: &["dxil-metal"],
    },
    KnobSpec {
        name: "PORT_SHADER_IMMUTABLE_OPT",
        aliases: &["AW2_SHADER_IMMUTABLE_OPT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Specialize Metal 4 PSO descriptors (disable validation/reflection/linking, fast math).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SHADER_MANIFEST",
        aliases: &["AW2_SHADER_MANIFEST"],
        kind: KnobType::Path,
        default: "",
        description: "Path to writable Tier-1 shader manifest file.",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-loader"],
    },
    KnobSpec {
        name: "PORT_SHADER_MANIFEST_FALLBACK",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Read-only fallback Tier-1 shader manifest directory/file.",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-factory"],
    },
    KnobSpec {
        name: "PORT_SHADER_PSO_OPT_V2",
        aliases: &["AW2_SHADER_PSO_OPT_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Refine SM 6.6 pipeline usage from MSC reflection and elide dead TBDR blend reads.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SHADOW",
        aliases: &["AW2_SHADOW"],
        kind: KnobType::String,
        default: "",
        description: "Shadow differential checker configuration string.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_SHADOW_TEST_DUMP",
        aliases: &["AW2_SHADOW_TEST_DUMP"],
        kind: KnobType::Path,
        default: "",
        description: "Directory to dump shadow-checker test artifacts.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_SKIP_UNHANDLED_FILTER",
        aliases: &["AW2_SKIP_UNHANDLED_FILTER"],
        kind: KnobType::Bool,
        default: "0",
        description: "Skip invoking registered SetUnhandledExceptionFilter callbacks.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_SM66_CS_STAGE_EXACT",
        aliases: &["AW2_SM66_CS_STAGE_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use zero-salt MSC reflection for exact SM 6.6 compute shader resource usage.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SM66_GFX_STAGE_EXACT",
        aliases: &["AW2_SM66_GFX_STAGE_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use zero-salt MSC reflection for exact SM 6.5/6.6 graphics/mesh stage usage.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SM66_HANDLE_KIND_EXACT",
        aliases: &["AW2_SM66_HANDLE_KIND_EXACT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Classify SM 6.6 bindless handles by exact SRV/UAV kind.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SM66_HANDLE_KIND_V2",
        aliases: &["AW2_SM66_HANDLE_KIND_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 SM 6.6 bindless handle kind classification.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SPLIT_COMPUTE_EXACT",
        aliases: &["AW2_SPLIT_COMPUTE_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use exact dependency tracking when splitting compute encoders.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SPLIT_RES_SOFT_SYNC",
        aliases: &["AW2_SPLIT_RES_SOFT_SYNC"],
        kind: KnobType::Bool,
        default: "1",
        description: "Use soft stage sync for split-barrier resource transitions.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_STAGESYNC_BARRIER_V2",
        aliases: &["AW2_STAGESYNC_BARRIER_V2"],
        kind: KnobType::Bool,
        default: "0",
        description: "Experimental v2 stage-sync barrier placement.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_STALL_MS",
        aliases: &["AW2_STALL_MS"],
        kind: KnobType::U64,
        default: "2000",
        description: "Main/render thread stall threshold in milliseconds for stack dump.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_STEAMCLIENT_DYLIB",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to Valve's native arm64 steamclient.dylib.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_STEAM_NATIVE",
        aliases: &[],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable dynamic native Apple Silicon steamclient.dylib IPC bridge.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_STRICT_HOST_THUNK",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Panic immediately when call_va8/call_va8_xmm/call_va8_caught refuses an unregistered host __TEXT target.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_STUB_BASELINE_JSON",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Path to a previous runtime stub summary JSON to diff against on exit.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_STUB_SUMMARY_JSON",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Path to write a ranked JSON summary of runtime stubs hit during execution.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_SUBMIT_COALESCE",
        aliases: &["AW2_SUBMIT_COALESCE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Elide 0-work command-buffer commits and redundant ExecuteCommandLists events.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SUBRANGE_UAV_BARRIER",
        aliases: &["AW2_SUBRANGE_UAV_BARRIER"],
        kind: KnobType::Bool,
        default: "1",
        description: "Track subresource/subrange UAV barriers instead of whole-resource barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_SYMBOLS",
        aliases: &["AW2_SYMBOLS"],
        kind: KnobType::Path,
        default: "",
        description: "Override path or disable (0) symbol table preloading for crash reports.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_SYNC_CLIPBOARD",
        aliases: &["AW2_SYNC_CLIPBOARD"],
        kind: KnobType::Bool,
        default: "1",
        description: "Synchronize Win32 clipboard with macOS NSPasteboard.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_TEST_GAMEPAD",
        aliases: &["AW2_TEST_GAMEPAD"],
        kind: KnobType::Bool,
        default: "0",
        description: "Gate interactive physical-gamepad integration test.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_TEST_GAMEPAD_SECS",
        aliases: &["AW2_TEST_GAMEPAD_SECS"],
        kind: KnobType::U64,
        default: "10",
        description: "Duration in seconds for interactive physical-gamepad test.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_TEST_MOLTENVK",
        aliases: &[],
        kind: KnobType::Bool,
        default: "0",
        description: "Enable live MoltenVK Vulkan integration tests.",
        affects_aot_key: false,
        crates: &["port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_TEST_ZLIB_DLL",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Optional path to zlib1.dll for port-factory scaffold integration tests.",
        affects_aot_key: false,
        crates: &["port-factory"],
    },
    KnobSpec {
        name: "PORT_THREAD_PRIORITY",
        aliases: &["AW2_THREAD_PRIORITY"],
        kind: KnobType::Bool,
        default: "1",
        description: "Apply Win32 thread priority to macOS pthread QoS mapping.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_TRACE_EAT",
        aliases: &["AW2_TRACE_EAT"],
        kind: KnobType::Bool,
        default: "0",
        description: "Trace export-address-table lookup and routing calls.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_UAV0_CLEAR_OVERLAP",
        aliases: &["AW2_UAV0_CLEAR_OVERLAP"],
        kind: KnobType::Bool,
        default: "1",
        description: "Allow non-overlapping clears across UAV-all barriers.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_V8_JITLESS",
        aliases: &["AW2_V8_JITLESS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Force embedded V8 engines into --jitless mode (no guest RWX JIT).",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_V8_REGFILE",
        aliases: &["AW2_V8_REGFILE"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable DF_REGFILE builtin installation and routing for V8 lifts.",
        affects_aot_key: false,
        crates: &["x86-runtime"],
    },
    KnobSpec {
        name: "PORT_VORBIS_FLOOR",
        aliases: &["AW2_VORBIS_FLOOR"],
        kind: KnobType::Enum(&["ak", "libvorbis"]),
        default: "ak",
        description: "Wwise Vorbis floor synthesis mode: 'ak' or 'libvorbis'.",
        affects_aot_key: false,
        crates: &["wwise-audio"],
    },
    KnobSpec {
        name: "PORT_VRAM_MB",
        aliases: &["AW2_VRAM_MB"],
        kind: KnobType::U64,
        default: "0",
        description: "Override reported dedicated GPU VRAM in MiB (0 = device default).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_VSYNC",
        aliases: &["AW2_VSYNC"],
        kind: KnobType::Bool,
        default: "1",
        description: "Enable display-synced presentation (0/false/off disables VSync).",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_VS_BUF_DEFER_EXACT",
        aliases: &["AW2_VS_BUF_DEFER_EXACT"],
        kind: KnobType::Bool,
        default: "1",
        description: "Defer vertex-buffer transition barriers to exact vertex stage consumption.",
        affects_aot_key: false,
        crates: &["d3d12-metal"],
    },
    KnobSpec {
        name: "PORT_VULKAN_DYLIB",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to Vulkan loader or MoltenVK dylib for vulkan-1.dll.",
        affects_aot_key: false,
        crates: &["port-factory", "port-loader", "win32-rt"],
    },
    KnobSpec {
        name: "PORT_WAIT_DEBUGGER",
        aliases: &["AW2_WAIT_DEBUGGER"],
        kind: KnobType::U64,
        default: "0",
        description: "Seconds to pause after PE mapping so tools/lldb.sh can attach by PID.",
        affects_aot_key: false,
        crates: &["port-loader"],
    },
    KnobSpec {
        name: "PORT_WAIT_SPIN_US",
        aliases: &["AW2_WAIT_SPIN_US"],
        kind: KnobType::U64,
        default: "16",
        description: "Maximum adaptive spin duration in microseconds before parking Win32 wait objects (0 disables spin).",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_WAKE_STATS",
        aliases: &["AW2_WAKE_STATS"],
        kind: KnobType::Bool,
        default: "0",
        description: "Log Win32 wait/wake contention statistics.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_WINDOW_SIZE",
        aliases: &["AW2_WINDOW_SIZE"],
        kind: KnobType::String,
        default: "",
        description: "Override initial Win32 window dimensions as WxH.",
        affects_aot_key: false,
        crates: &["win32-rt"],
    },
    KnobSpec {
        name: "PORT_X86_VERIFY",
        aliases: &[],
        kind: KnobType::Path,
        default: "",
        description: "Override path to x86_verify helper binary for port-factory verify-x86.",
        affects_aot_key: false,
        crates: &["port-factory"],
    },
    KnobSpec {
        name: "TRANSLATOR_HOME",
        aliases: &["AW2_DRIVE_C"],
        kind: KnobType::Path,
        default: "~/Translator",
        description: "Virtual Windows C:\\ root directory (alias: AW2_DRIVE_C).",
        affects_aot_key: false,
        crates: &["dxil-metal", "port-factory", "port-loader", "win32-rt"],
    },

];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeSet, HashSet};
    use std::path::PathBuf;

    /// Standard host OS, Cargo, toolchain, SDK, and subprocess-test harness env vars
    /// that are intentionally external to the Port Factory knob registry.
    const STANDARD_EXTERNAL_ENV_VARS: &[&str] = &[
        "CARGO_MANIFEST_DIR",
        "CARGO_PKG_NAME",
        "CARGO_TARGET_DIR",
        "CHILD_TAG",
        "CLAUDECODE",
        "CONTROLLER_THREAD_ID",
        "DOTNET_ROOT",
        "DXIL_METALD_TEST_ROLE",
        "HOME",
        "HOST_ONLY_GUEST_PROC_MARKER",
        "MTL_CAPTURE_ENABLED",
        "OUT_DIR",
        "PATH",
        "PE_DUMP_DELAY",
        "ROSETTA_ADVERTISE_AVX",
        "STEAM_APPID",
        "STEAM_USER",
        "SteamAppId",
        "SteamGameId",
        "TARGET",
        "TZ",
        "USER",
        "VGMSTREAM_CLI",
        "VULKAN_SDK",
        "ZIG",
    ];

    /// Non-knob `PORT_*` string literals used as prefixes or unit-test sentinels.
    const ALLOWED_NON_KNOB_PORT_LITERALS: &[&str] = &[
        "PORT_ENV_",
        "PORT_LATE_MARKER_TEST",
    ];

    fn workspace_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(rd) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if p.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                collect_rs_files(&p, out);
            } else if p.extension().is_some_and(|e| e == "rs") {
                out.push(p);
            }
        }
    }

    /// Scan a Rust source file (skipping line/block comments) and return:
    /// - `(line, name)` for every `env::var("...")` / `env::var_os("...")` call, and
    /// - `(line, name)` for every `"PORT_..."` / `"AW2_..."` string literal in code.
    fn scan_rust_env_reads(src: &str) -> (Vec<(usize, String)>, Vec<(usize, String)>) {
        let bytes = src.as_bytes();
        let n = bytes.len();
        let mut i = 0usize;
        let mut line = 1usize;

        // Track non-comment code tokens as a flat string alongside parsed string literals.
        let mut env_calls = Vec::new();
        let mut port_lits = Vec::new();
        let mut code_buf = String::with_capacity(n);

        while i < n {
            let b = bytes[i];
            if b == b'\n' {
                line += 1;
                code_buf.push('\n');
                i += 1;
                continue;
            }
            // Line comment `// ...`
            if b == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
                i += 2;
                while i < n && bytes[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            // Block comment `/* ... */` (supports Rust nesting)
            if b == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
                let mut depth = 1usize;
                i += 2;
                while i < n && depth > 0 {
                    if bytes[i] == b'\n' {
                        line += 1;
                    }
                    if bytes[i] == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
                        depth += 1;
                        i += 2;
                    } else if bytes[i] == b'*' && i + 1 < n && bytes[i + 1] == b'/' {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                code_buf.push(' ');
                continue;
            }
            // Raw string literal `r"..."`, `r#"..."#`, `br#"..."#`, `cr#"..."#`
            let raw_start = if b == b'r' {
                Some(i + 1)
            } else if (b == b'b' || b == b'c') && i + 1 < n && bytes[i + 1] == b'r' {
                Some(i + 2)
            } else {
                None
            };
            if let Some(mut j) = raw_start {
                let prev_ident = i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
                if !prev_ident {
                    let mut hashes = 0usize;
                    while j < n && bytes[j] == b'#' {
                        hashes += 1;
                        j += 1;
                    }
                    if j < n && bytes[j] == b'"' {
                        j += 1;
                        let content_start = j;
                        let lit_line = line;
                        while j < n {
                            if bytes[j] == b'\n' {
                                line += 1;
                            }
                            if bytes[j] == b'"' {
                                let mut h = 0usize;
                                while h < hashes && j + 1 + h < n && bytes[j + 1 + h] == b'#' {
                                    h += 1;
                                }
                                if h == hashes {
                                    let lit = &src[content_start..j];
                                    record_string_lit(&code_buf, lit_line, lit, &mut env_calls, &mut port_lits);
                                    i = j + 1 + hashes;
                                    code_buf.push_str("\"\"");
                                    break;
                                }
                            }
                            j += 1;
                        }
                        if j >= n {
                            break;
                        }
                        continue;
                    }
                }
            }
            // Char literal `'a'` or `'\''` (must not confuse lifetime `'static`)
            if b == b'\'' {
                if i + 2 < n && bytes[i + 1] == b'\\' {
                    // escaped char literal like '\'' or '\n' or '\x7f'
                    let mut j = i + 3;
                    while j < n && bytes[j] != b'\'' && bytes[j] != b'\n' && j - i < 12 {
                        j += 1;
                    }
                    if j < n && bytes[j] == b'\'' {
                        code_buf.push_str("''");
                        i = j + 1;
                        continue;
                    }
                } else if i + 2 < n && bytes[i + 2] == b'\'' {
                    code_buf.push_str("''");
                    i += 3;
                    continue;
                }
            }
            // Normal string literal `"..."`
            if b == b'"' {
                let lit_line = line;
                i += 1;
                let mut lit = String::new();
                while i < n {
                    let ch = bytes[i];
                    if ch == b'\n' {
                        line += 1;
                    }
                    if ch == b'\\' && i + 1 < n {
                        lit.push(bytes[i + 1] as char);
                        if bytes[i + 1] == b'\n' {
                            line += 1;
                        }
                        i += 2;
                        continue;
                    }
                    if ch == b'"' {
                        i += 1;
                        break;
                    }
                    lit.push(ch as char);
                    i += 1;
                }
                record_string_lit(&code_buf, lit_line, &lit, &mut env_calls, &mut port_lits);
                code_buf.push_str("\"\"");
                continue;
            }

            code_buf.push(b as char);
            i += 1;
        }

        (env_calls, port_lits)
    }

    fn is_knob_ident(s: &str) -> bool {
        (s.starts_with("PORT_") || s.starts_with("AW2_"))
            && s.len() > 4
            && s.bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
    }

    fn record_string_lit(
        code_before: &str,
        line: usize,
        lit: &str,
        env_calls: &mut Vec<(usize, String)>,
        port_lits: &mut Vec<(usize, String)>,
    ) {
        let trimmed = code_before.trim_end();
        if let Some(before_paren) = trimmed.strip_suffix('(') {
            let fn_head = before_paren.trim_end();
            if fn_head.ends_with("env::var") || fn_head.ends_with("env::var_os") {
                env_calls.push((line, lit.to_string()));
            }
        }
        if is_knob_ident(lit) {
            port_lits.push((line, lit.to_string()));
        }
    }

    #[test]
    fn registry_invariants() {
        let root = workspace_root();
        let mut names = HashSet::new();
        let mut aliases = HashSet::new();
        let mut prev_name: Option<&str> = None;

        for spec in KNOBS {
            assert!(!spec.name.is_empty(), "empty knob name");
            assert!(
                !spec.description.trim().is_empty(),
                "empty description for {}",
                spec.name
            );
            assert!(!spec.crates.is_empty(), "empty crates for {}", spec.name);
            assert!(
                names.insert(spec.name),
                "duplicate canonical knob name: {}",
                spec.name
            );
            if let Some(prev) = prev_name {
                assert!(
                    prev < spec.name,
                    "KNOBS must be sorted by canonical name: {prev} >= {}",
                    spec.name
                );
            }
            prev_name = Some(spec.name);

            for &alias in spec.aliases {
                assert!(!alias.is_empty(), "empty alias on {}", spec.name);
                assert!(
                    aliases.insert(alias),
                    "duplicate alias {alias} (on {})",
                    spec.name
                );
            }
            for &crate_name in spec.crates {
                assert!(
                    root.join("crates").join(crate_name).is_dir(),
                    "knob {} references non-existent crate {crate_name}",
                    spec.name
                );
            }
        }

        for alias in &aliases {
            assert!(
                !names.contains(alias),
                "alias {alias} collides with a canonical knob name"
            );
        }
    }

    #[test]
    fn every_env_read_in_crates_is_registered() {
        let root = workspace_root();
        let crates_dir = root.join("crates");
        let self_path = crates_dir.join("port-manifest/src/knobs.rs");
        let manifest_path = crates_dir.join("port-manifest/src/manifest.rs");
        let mut files = Vec::new();
        collect_rs_files(&crates_dir, &mut files);
        files.sort();

        let ext_allowed: HashSet<&str> = STANDARD_EXTERNAL_ENV_VARS.iter().copied().collect();
        let lit_allowed: HashSet<&str> = ALLOWED_NON_KNOB_PORT_LITERALS.iter().copied().collect();

        let mut violations = Vec::new();

        for file in files {
            if file == self_path {
                continue;
            }
            let rel = file.strip_prefix(&crates_dir).unwrap();
            let crate_name = rel
                .components()
                .next()
                .and_then(|c| c.as_os_str().to_str())
                .unwrap_or("");
            let Ok(src) = std::fs::read_to_string(&file) else {
                continue;
            };
            let (env_calls, port_lits) = scan_rust_env_reads(&src);

            for (line, name) in env_calls {
                if ext_allowed.contains(name.as_str()) {
                    continue;
                }
                match lookup(&name) {
                    None => violations.push(format!(
                        "{}:{line}: unregistered env::var({name:?}) — add it to crates/port-manifest/src/knobs.rs",
                        rel.display()
                    )),
                    Some(spec) => {
                        if !spec.crates.contains(&crate_name) {
                            violations.push(format!(
                                "{}:{line}: knob {} (via {name}) read in crate {crate_name:?}, which is not listed in spec.crates {:?}",
                                rel.display(),
                                spec.name,
                                spec.crates
                            ));
                        }
                    }
                }
            }

            for (line, name) in port_lits {
                if lit_allowed.contains(name.as_str()) {
                    continue;
                }
                match lookup(&name) {
                    None => violations.push(format!(
                        "{}:{line}: unregistered knob literal {name:?} — add it to crates/port-manifest/src/knobs.rs",
                        rel.display()
                    )),
                    Some(spec) => {
                        // `manifest.rs` unit tests reference arbitrary registered knobs as sample `[knobs]` fixtures;
                        // actual `env::var*` reads in `manifest.rs` are still crate-checked in the `env_calls` loop above.
                        if file != manifest_path && !spec.crates.contains(&crate_name) {
                            violations.push(format!(
                                "{}:{line}: knob {} (via {name}) referenced in crate {crate_name:?}, which is not listed in spec.crates {:?}",
                                rel.display(),
                                spec.name,
                                spec.crates
                            ));
                        }
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "found {} unregistered or misattributed knob read(s) across crates/:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }

    #[test]
    fn precedence_env_over_alias_over_manifest_over_default() {
        let spec = lookup("PORT_JIT_RAS").expect("PORT_JIT_RAS registered");
        let prev_p = std::env::var_os("PORT_JIT_RAS");
        let prev_a = std::env::var_os("AW2_JIT_RAS");

        std::env::remove_var("PORT_JIT_RAS");
        std::env::remove_var("AW2_JIT_RAS");

        // 1. Default when unset everywhere.
        let r0 = resolve_spec(spec, None);
        assert_eq!(r0.value, "1");
        assert_eq!(r0.source, KnobSource::Default);

        // 2. Manifest [knobs] overrides default (both canonical and legacy alias accepted).
        let mut mk = BTreeMap::new();
        mk.insert("AW2_JIT_RAS".to_string(), "false".to_string());
        let r1 = resolve_spec(spec, Some(&mk));
        assert_eq!(r1.value, "0");
        assert_eq!(r1.source, KnobSource::Manifest);

        // 3. Env alias overrides manifest [knobs].
        std::env::set_var("AW2_JIT_RAS", "1");
        let r2 = resolve_spec(spec, Some(&mk));
        assert_eq!(r2.value, "1");
        assert_eq!(r2.source, KnobSource::EnvAlias("AW2_JIT_RAS"));

        // 4. Canonical env overrides env alias and manifest [knobs].
        std::env::set_var("PORT_JIT_RAS", "0");
        let r3 = resolve_spec(spec, Some(&mk));
        assert_eq!(r3.value, "0");
        assert_eq!(r3.source, KnobSource::Env);

        match prev_p {
            Some(v) => std::env::set_var("PORT_JIT_RAS", v),
            None => std::env::remove_var("PORT_JIT_RAS"),
        }
        match prev_a {
            Some(v) => std::env::set_var("AW2_JIT_RAS", v),
            None => std::env::remove_var("AW2_JIT_RAS"),
        }
    }

    #[test]
    fn apply_manifest_and_aliases_mirrors_and_validates() {
        let prev_p = std::env::var_os("PORT_METALFX");
        let prev_a = std::env::var_os("AW2_METALFX");
        let prev_ras_p = std::env::var_os("PORT_JIT_RAS");
        let prev_ras_a = std::env::var_os("AW2_JIT_RAS");

        std::env::remove_var("PORT_METALFX");
        std::env::remove_var("AW2_METALFX");
        std::env::set_var("PORT_JIT_RAS", "0");
        std::env::remove_var("AW2_JIT_RAS");

        let mut manifest = GameManifest::default();
        manifest.knobs.insert("PORT_METALFX".to_string(), "false".to_string());
        manifest.knobs.insert("PORT_JIT_RAS".to_string(), "1".to_string());

        let resolved = apply_manifest_and_aliases(Some(&manifest)).expect("valid manifest knobs");

        // Manifest value normalized to "0" and mirrored to both canonical and alias.
        assert_eq!(std::env::var("PORT_METALFX").as_deref(), Ok("0"));
        assert_eq!(std::env::var("AW2_METALFX").as_deref(), Ok("0"));
        // Env value beat manifest ("0" vs "1") and mirrored to legacy alias.
        assert_eq!(std::env::var("PORT_JIT_RAS").as_deref(), Ok("0"));
        assert_eq!(std::env::var("AW2_JIT_RAS").as_deref(), Ok("0"));

        let mfx = resolved.iter().find(|r| r.spec.name == "PORT_METALFX").unwrap();
        assert_eq!(mfx.source, KnobSource::Manifest);
        let ras = resolved.iter().find(|r| r.spec.name == "PORT_JIT_RAS").unwrap();
        assert_eq!(ras.source, KnobSource::Env);

        // Unknown knob fails loudly.
        let mut bad = GameManifest::default();
        bad.knobs.insert("PORT_NOT_A_REAL_KNOB".to_string(), "1".to_string());
        let err = apply_manifest_and_aliases(Some(&bad)).unwrap_err();
        assert!(err.contains("PORT_NOT_A_REAL_KNOB"));

        // Restore env & active manifest state.
        if let Ok(mut g) = ACTIVE_MANIFEST_KNOBS.write() {
            *g = None;
        }
        for (k, prev) in [
            ("PORT_METALFX", prev_p),
            ("AW2_METALFX", prev_a),
            ("PORT_JIT_RAS", prev_ras_p),
            ("AW2_JIT_RAS", prev_ras_a),
        ] {
            match prev {
                Some(v) => std::env::set_var(k, v),
                None => std::env::remove_var(k),
            }
        }
    }

    #[test]
    fn aot_key_hash_tracks_only_aot_affecting_knobs() {
        let base = aot_key_hash_with(None);

        let mut non_aot = BTreeMap::new();
        non_aot.insert("PORT_VRAM_MB".to_string(), "16384".to_string());
        assert_eq!(
            aot_key_hash_with(Some(&non_aot)),
            base,
            "non-AOT knob must not change aot_key_hash"
        );

        let mut aot_override = BTreeMap::new();
        aot_override.insert("PORT_JIT_THUNK_ELIDE".to_string(), "2".to_string());
        assert_ne!(
            aot_key_hash_with(Some(&aot_override)),
            base,
            "AOT-affecting knob must change aot_key_hash"
        );

        // Alias in manifest produces the exact same hash as canonical name.
        let mut aot_alias = BTreeMap::new();
        aot_alias.insert("AW2_JIT_THUNK_ELIDE".to_string(), "2".to_string());
        assert_eq!(
            aot_key_hash_with(Some(&aot_override)),
            aot_key_hash_with(Some(&aot_alias))
        );
    }

    #[test]
    fn table_and_json_formatters_include_all_knobs() {
        let resolved = resolve_all(None);
        let table = format_table(&resolved, None);
        assert!(table.contains("PORT_AOT"));
        assert!(table.contains("PORT_METALFX"));

        let json = format_json(&resolved, Some(Path::new("games/overwatch/game.toml")), 0x1234);
        assert!(json.starts_with("{\n"));
        assert!(json.contains("\"aot_key_hash\": \"0x0000000000001234\""));
        assert!(json.contains("\"name\": \"PORT_AOT\""));
        assert!(json.contains("\"aliases\": [\"AW2_DLL_AOT\"]"));
        assert!(json.ends_with("}\n"));

        // Verify all registered AOT-key knobs exist.
        let aot_names: BTreeSet<&str> = KNOBS
            .iter()
            .filter(|k| k.affects_aot_key)
            .map(|k| k.name)
            .collect();
        assert!(aot_names.contains("PORT_JIT_RAS"));
        assert!(aot_names.contains("PORT_JIT_TSO"));
        assert!(aot_names.contains("PORT_AOT_PROBE"));
    }
}
