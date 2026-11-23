//! Canonical Windows DLL name mapping and declarative DLL routing tables.
//!
//! Centralizes:
//! - [`canonical_dll`]: API-set, forwarder, and versioned-DLL aliasing to builtin module names.
//! - [`DEFAULT_BUILTIN_OVERRIDES`]: Game-dir DLLs replaced by native ARM64 builtins by default.
//! - [`DllRoute`] & [`DllRoutingTable`]: Layered per-DLL routing policy across defaults,
//!   plugins, legacy `builtin_overrides`, `[middleware.*]`, `[dlls]`, and environment overrides.

use std::collections::{BTreeMap, BTreeSet};

use crate::manifest::GameManifest;
use crate::middleware;

/// Map API-set and alias DLL names to the canonical builtin module name that implements them.
///
/// Input may be any case, with or without directory components or a `.dll` suffix.
pub fn canonical_dll(dll: &str) -> String {
    let n = pe_image::normalize_dll_name(dll);
    if n.starts_with("api-ms-win-crt-") {
        return "ucrtbase.dll".into();
    }
    if n.starts_with("api-ms-win-core-registry-")
        || n.starts_with("api-ms-win-service-")
        || n.starts_with("api-ms-win-eventing-")
        || n.starts_with("api-ms-win-security-")
    {
        return "advapi32.dll".into();
    }
    if n.starts_with("api-ms-win-core-com-") || n.starts_with("api-ms-win-core-winrt-") {
        return "ole32.dll".into();
    }
    if n.starts_with("api-ms-win-core-version-") {
        return "version.dll".into();
    }
    if n.starts_with("api-ms-win-shcore-") || n == "shcore.dll" {
        return "user32.dll".into();
    }
    if n.starts_with("api-ms-win-core-")
        || n.starts_with("api-ms-win-appmodel-")
        || n == "kernelbase.dll"
        || n == "psapi.dll"
    {
        return "kernel32.dll".into();
    }
    if n.starts_with("ext-ms-win-") {
        return n;
    }
    if n == "xinput1_3.dll"
        || n == "xinput9_1_0.dll"
        || n == "xinput1_2.dll"
        || n == "xinput1_1.dll"
    {
        return "xinput1_4.dll".into();
    }
    if n == "xaudio2_9.dll" || n == "xaudio2_8.dll" || n == "xaudio2_7.dll" {
        return "xaudio2_9redist.dll".into();
    }
    if n == "mediadecoders.windowsdesktop.dll" {
        return "mediadecoders_windowsdesktop.dll".into();
    }
    if n == "cohtml.windowsdesktop.dll" {
        return "cohtml_windowsdesktop.dll".into();
    }
    if n == "vulkan-1.dll" {
        return "vulkan_1.dll".into();
    }
    if n == "d3dcompiler_43.dll" || n == "d3dcompiler_46.dll" {
        return "d3dcompiler_47.dll".into();
    }
    if n == "mono-2.0-bdwgc.dll" || n == "mono.dll" || n == "mono-2.0-sgen.dll" {
        return "mono_2_0_bdwgc.dll".into();
    }
    if n == "mscoreei.dll" {
        return "mscoree.dll".into();
    }
    if n == "clr.dll" {
        return "coreclr.dll".into();
    }
    if n == "steam_api.dll" {
        return "steam_api64.dll".into();
    }
    if n == "dstoragecore.dll" {
        return "dstorage.dll".into();
    }
    if n == "nvapi.dll" {
        return "nvapi64.dll".into();
    }
    if n == "_nvngx.dll" || n == "nvngx_dlssg.dll" || n == "nvngx_dlssd.dll" {
        return "nvngx_dlss.dll".into();
    }
    if n == "libxess_dx12.dll" {
        return "libxess.dll".into();
    }
    if n == "fmod64.dll" || n == "fmodstudio.dll" || n == "fmodstudio64.dll" {
        return "fmod.dll".into();
    }
    if n.starts_with("oo2core_") && n.ends_with(".dll") {
        return "oo2core_9_win64.dll".into();
    }
    if n == "tbb12.dll" || n == "tbb.dll" {
        return "tbbmalloc.dll".into();
    }
    if n == "libzstd.dll" {
        return "zstd.dll".into();
    }
    if n == "ffx_fsr3_x64.dll" || n == "ffx_fsr2_api_x64.dll" || n == "amd_fidelityfx_dx12.dll" {
        return "amd_fidelityfx_loader_dx12.dll".into();
    }
    if n == "wsock32.dll" {
        return "ws2_32.dll".into();
    }
    if n == "shlwapi.dll" {
        return "shell32.dll".into();
    }
    if n == "comctl32.dll" {
        return "uxtheme.dll".into();
    }
    if n == "sfc.dll" || n == "sfc_os.dll" || n == "tbs.dll" {
        return "wintrust.dll".into();
    }
    if n == "cfgmgr32.dll" || n.starts_with("api-ms-win-devices-config-") {
        return "setupapi.dll".into();
    }
    if n == "nvlowlatencyvk.dll" {
        return "vulkan_1.dll".into();
    }
    if n == "sensapi.dll" || n == "wininet.dll" {
        return "winhttp.dll".into();
    }
    if n == "userenv.dll" {
        return "advapi32.dll".into();
    }
    if n == "sspicli.dll" || n == "security.dll" {
        return "secur32.dll".into();
    }
    if n == "binkw64.dll" {
        return "bink2w64.dll".into();
    }
    n
}

/// Game-dir DLLs replaced by native ARM64 builtins by default (when a builtin exists).
///
/// Matches `crates/port-loader/src/loader.rs::BUILTIN_OVERRIDES` 1:1.
pub const DEFAULT_BUILTIN_OVERRIDES: &[&str] = &[
    "sl.interposer.dll",
    "eossdk-win64-shipping.dll",
    "steam_api64.dll",
    "steam_api.dll",
    "dstorage.dll",
    "dstoragecore.dll",
    "nvapi64.dll",
    "nvapi.dll",
    "amd_ags_x64.dll",
    "nvngx_dlss.dll",
    "_nvngx.dll",
    "nvngx_dlssg.dll",
    "nvngx_dlssd.dll",
    "libxess.dll",
    "libxess_dx12.dll",
    "libxess_dx11.dll",
    "bink2w64.dll",
    "mediadecoders.windowsdesktop.dll",
    "tbb12.dll",
    "tbb.dll",
    "tbbmalloc.dll",
    "lz4.dll",
    "gameinput.dll",
    "mscoree.dll",
    "hostfxr.dll",
    "coreclr.dll",
    "mono-2.0-bdwgc.dll",
    "libcurl.dll",
    "zlib1.dll",
    "v8_zlib.dll",
    "amd_fidelityfx_loader_dx12.dll",
    "cohtml_icuuc.dll",
    "xaudio2_9redist.dll",
    "nri.dll",
    "nvcloth_x64.dll",
    "libprotobuf.dll",
];

/// Routing directive for a single Windows DLL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DllRoute {
    /// Serve from a native ARM64 builtin (optionally redirecting to a specific provider DLL).
    Builtin { provider: Option<String> },
    /// Load the game-dir PE DLL via x86-runtime if present; fall back to builtin if absent.
    Guest,
    /// Strictly require loading the game-dir PE DLL; never fall back to a builtin.
    GuestOnly,
    /// Refuse to load this DLL (emulates module-not-found / disabled feature).
    Disabled,
    /// Redirect lookup to another DLL name before resolving routing/canonical target.
    Alias(String),
}

impl DllRoute {
    /// Parse a route string from `game.toml` or `PORT_BUILTIN_OVERRIDES`.
    pub fn parse(raw: &str) -> Option<Self> {
        let s = raw.trim();
        if s.is_empty() {
            return None;
        }
        if let Some((prefix, rest)) = s.split_once(':') {
            let target = rest.trim();
            if target.is_empty() {
                return None;
            }
            if prefix.eq_ignore_ascii_case("builtin") {
                return Some(Self::Builtin {
                    provider: Some(pe_image::normalize_dll_name(target)),
                });
            }
            if prefix.eq_ignore_ascii_case("alias") {
                return Some(Self::Alias(pe_image::normalize_dll_name(target)));
            }
            return None;
        }
        match s.to_ascii_lowercase().as_str() {
            "builtin" => Some(Self::Builtin { provider: None }),
            "guest" | "pe" | "jit" | "fallback" => Some(Self::Guest),
            "guest-only" | "strict-guest" => Some(Self::GuestOnly),
            "disabled" | "deny" | "block" => Some(Self::Disabled),
            _ => None,
        }
    }

    /// Serialize this route back to its canonical TOML string representation.
    pub fn to_toml_value(&self) -> String {
        match self {
            Self::Builtin { provider: None } => "builtin".to_string(),
            Self::Builtin { provider: Some(p) } => format!("builtin:{p}"),
            Self::Guest => "guest".to_string(),
            Self::GuestOnly => "guest-only".to_string(),
            Self::Disabled => "disabled".to_string(),
            Self::Alias(target) => format!("alias:{target}"),
        }
    }
}

/// Resolved per-DLL routing policy across all configuration layers.
#[derive(Debug, Clone, Default)]
pub struct DllRoutingTable {
    routes: BTreeMap<String, DllRoute>,
    force_all_builtins: bool,
    disable_all_builtin_overrides: bool,
}

impl DllRoutingTable {
    /// Construct the default routing table seeded with [`DEFAULT_BUILTIN_OVERRIDES`].
    pub fn new() -> Self {
        let mut routes = BTreeMap::new();
        for &dll in DEFAULT_BUILTIN_OVERRIDES {
            routes.insert(dll.to_string(), DllRoute::Builtin { provider: None });
        }
        Self {
            routes,
            force_all_builtins: false,
            disable_all_builtin_overrides: false,
        }
    }

    /// Construct a routing table from defaults plus a legacy `PORT_BUILTIN_OVERRIDES` CSV spec.
    pub fn from_legacy_override_spec(extra: Option<&str>) -> Self {
        let mut table = Self::new();
        if let Some(spec) = extra {
            table.apply_override_spec(spec);
        }
        table
    }

    /// Construct a routing table by layering configuration in strict precedence order:
    /// 1. [`DEFAULT_BUILTIN_OVERRIDES`]
    /// 2. `plugin_defaults` (`(dll, route_str)` pairs)
    /// 3. `manifest.builtin_overrides` (legacy CSV spec)
    /// 4. `manifest.middleware` (`[middleware.*]` selections with explicit `mode`)
    /// 5. `manifest.dlls` (explicit per-DLL `[dlls]` overrides)
    /// 6. `env_override` (`PORT_BUILTIN_OVERRIDES` / `AW2_BUILTIN_OVERRIDES`)
    pub fn from_manifest(
        manifest: &GameManifest,
        plugin_defaults: &[(&str, &str)],
        env_override: Option<&str>,
    ) -> Self {
        let mut table = Self::new();

        // Layer 1 (custom): plugin defaults
        for &(dll, route_str) in plugin_defaults {
            if let Some(route) = DllRoute::parse(route_str) {
                table
                    .routes
                    .insert(pe_image::normalize_dll_name(dll), route);
            }
        }

        // Layer 2: legacy manifest.builtin_overrides
        if let Some(spec) = manifest.builtin_overrides.as_deref() {
            table.apply_override_spec(spec);
        }

        // Layer 3: [middleware.*]
        for (family_id, sel) in &manifest.middleware {
            let Some(route) = sel.mode.as_ref() else {
                continue;
            };
            if let Some(ver) = sel.version.as_deref() {
                if let Some(spec) = middleware::find_version(family_id, ver) {
                    for &dll in spec.dlls {
                        table.routes.insert(dll.to_string(), route.clone());
                    }
                    continue;
                }
            }
            if let Some(family) = middleware::find_family(family_id) {
                for dll in middleware::family_all_dlls(family) {
                    table.routes.insert(dll.to_string(), route.clone());
                }
            }
        }

        // Layer 4: explicit per-DLL [dlls] overrides
        for (dll, route) in &manifest.dlls {
            table
                .routes
                .insert(pe_image::normalize_dll_name(dll), route.clone());
        }

        // Layer 5 (highest): environment variable override
        if let Some(spec) = env_override {
            table.apply_override_spec(spec);
        }

        table
    }

    fn apply_override_spec(&mut self, spec: &str) {
        let trimmed = spec.trim();
        if trimmed.is_empty() {
            return;
        }
        if trimmed == "0"
            || trimmed.eq_ignore_ascii_case("none")
            || trimmed.eq_ignore_ascii_case("false")
        {
            self.disable_all_builtin_overrides = true;
            self.force_all_builtins = false;
            self.routes
                .retain(|_, r| !matches!(r, DllRoute::Builtin { .. }));
            return;
        }
        if trimmed == "1" || trimmed.eq_ignore_ascii_case("all") {
            self.force_all_builtins = true;
            self.disable_all_builtin_overrides = false;
            self.routes
                .retain(|_, r| !matches!(r, DllRoute::Guest | DllRoute::GuestOnly));
            return;
        }
        for tok in trimmed.split(',').map(str::trim).filter(|s| !s.is_empty()) {
            if let Some((lhs, rhs)) = tok.split_once('=') {
                let dll = pe_image::normalize_dll_name(lhs.trim());
                if let Some(route) = DllRoute::parse(rhs.trim()) {
                    self.routes.insert(dll, route);
                }
            } else if let Some(neg) = tok.strip_prefix('-').or_else(|| tok.strip_prefix('!')) {
                let dll = pe_image::normalize_dll_name(neg.trim());
                self.routes.insert(dll, DllRoute::Guest);
            } else {
                let dll = pe_image::normalize_dll_name(tok);
                self.routes
                    .insert(dll, DllRoute::Builtin { provider: None });
            }
        }
    }

    /// Look up the effective [`DllRoute`] for `dll_name`.
    pub fn route(&self, dll_name: &str) -> DllRoute {
        let norm = pe_image::normalize_dll_name(dll_name);
        if let Some(r) = self.routes.get(&norm) {
            return r.clone();
        }
        if self.force_all_builtins {
            return DllRoute::Builtin { provider: None };
        }
        DllRoute::Guest
    }

    /// True if `dll_name` should be overridden by a native ARM64 builtin even when present in game-dir.
    pub fn is_builtin_override(&self, dll_name: &str) -> bool {
        matches!(self.route(dll_name), DllRoute::Builtin { .. })
    }

    /// True if `dll_name` is explicitly blocked from loading.
    pub fn is_disabled(&self, dll_name: &str) -> bool {
        matches!(self.route(dll_name), DllRoute::Disabled)
    }

    /// True if `dll_name` must come strictly from game-dir PE with no builtin fallback.
    pub fn is_guest_only(&self, dll_name: &str) -> bool {
        matches!(self.route(dll_name), DllRoute::GuestOnly)
    }

    /// Resolve the canonical target DLL name, following up to 4 `Alias` hops and honoring
    /// `Builtin { provider: Some(...) }` redirections.
    pub fn resolve_target_dll(&self, dll_name: &str) -> String {
        let mut current = pe_image::normalize_dll_name(dll_name);
        for _ in 0..4 {
            match self.routes.get(&current) {
                Some(DllRoute::Alias(target)) => {
                    current = target.clone();
                }
                Some(DllRoute::Builtin { provider: Some(p) }) => {
                    return canonical_dll(p);
                }
                _ => break,
            }
        }
        if let Some(DllRoute::Builtin { provider: Some(p) }) = self.routes.get(&current) {
            return canonical_dll(p);
        }
        canonical_dll(&current)
    }

    /// Return all DLLs explicitly routed `Guest` or `GuestOnly` in this table.
    pub fn guest_protected_dlls(&self) -> BTreeSet<String> {
        self.routes
            .iter()
            .filter_map(|(k, v)| match v {
                DllRoute::Guest | DllRoute::GuestOnly => Some(k.clone()),
                _ => None,
            })
            .collect()
    }

    /// Inspect the explicit per-DLL routing map.
    pub fn routes(&self) -> &BTreeMap<String, DllRoute> {
        &self.routes
    }

    /// True when `"1"` / `"all"` forced all unrouted DLLs to builtins.
    pub fn force_all_builtins(&self) -> bool {
        self.force_all_builtins
    }

    /// True when `"0"` / `"none"` / `"false"` cleared builtin overrides.
    pub fn disable_all_builtin_overrides(&self) -> bool {
        self.disable_all_builtin_overrides
    }

    /// Decide whether a PC `.dll` file can be safely stripped on Apple Silicon during
    /// bundle storage optimization.
    ///
    /// Exact parity with `port-factory/src/storage_opt.rs::should_strip_pc_dll`'s
    /// `EXACT_STRIP` and `PREFIX_STRIP` lists, while also respecting `keep_dlls`,
    /// preserving any DLL routed `Guest` / `GuestOnly` in `self`, and stripping any
    /// DLL routed `Disabled`.
    pub fn should_strip_pc_dll(&self, file_name: &str, keep_dlls: &BTreeSet<String>) -> bool {
        let lower = file_name.to_ascii_lowercase();
        if !lower.ends_with(".dll") {
            return false;
        }
        if keep_dlls.contains(&lower)
            || keep_dlls.iter().any(|k| k.eq_ignore_ascii_case(&lower))
        {
            return false;
        }

        match self.routes.get(&lower) {
            Some(DllRoute::Guest | DllRoute::GuestOnly) => return false,
            Some(DllRoute::Disabled) => return true,
            _ => {}
        }

        // Exact PC-only GPU vendor / debug / runtime-builtin-superseded DLLs.
        const EXACT_STRIP: &[&str] = &[
            "d3d12.dll",
            "d3d12core.dll",
            "d3d12sdklayers.dll",
            "dxcompiler.dll",
            "dxil.dll",
            "dstorage.dll",
            "dstoragecore.dll",
            "nrd.dll",
            "nri.dll",
            "nv_cluster_builder.dll",
            "nvapi64.dll",
            "nvcuda.dll",
            "opencl.dll",
            "bink2w64.dll",
            "eossdk-win64-shipping.dll",
            "xaudio2_9redist.dll",
            "libcurl.dll",
            "libprotobuf.dll",
            "zlib1.dll",
            "v8_zlib.dll",
            "nvcloth_x64.dll",
            "apex_clothing_x64.dll",
            "apexframework_x64.dll",
        ];
        if EXACT_STRIP.contains(&lower.as_str()) {
            return true;
        }

        // Prefix families for NVIDIA DLSS/Streamline/NVTX, AMD FSR/AGS, Razer Chroma, D3DCompiler.
        const PREFIX_STRIP: &[&str] = &[
            "nvngx_",
            "sl.",
            "nvtoolsext",
            "amd_fidelityfx_",
            "amd_ags_",
            "cchromaeditorlibrary",
            "d3dcompiler_",
        ];
        PREFIX_STRIP.iter().any(|p| lower.starts_with(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_dll_parity() {
        // API sets & Windows core forwarders
        assert_eq!(canonical_dll("api-ms-win-crt-runtime-l1-1-0.dll"), "ucrtbase.dll");
        assert_eq!(canonical_dll("api-ms-win-core-registry-l1-1-0.dll"), "advapi32.dll");
        assert_eq!(canonical_dll("api-ms-win-service-core-l1-1-0.dll"), "advapi32.dll");
        assert_eq!(canonical_dll("api-ms-win-eventing-provider-l1-1-0.dll"), "advapi32.dll");
        assert_eq!(canonical_dll("api-ms-win-security-base-l1-1-0.dll"), "advapi32.dll");
        assert_eq!(canonical_dll("api-ms-win-core-com-l1-1-0.dll"), "ole32.dll");
        assert_eq!(canonical_dll("api-ms-win-core-winrt-l1-1-0.dll"), "ole32.dll");
        assert_eq!(canonical_dll("api-ms-win-core-version-l1-1-0.dll"), "version.dll");
        assert_eq!(canonical_dll("api-ms-win-shcore-scaling-l1-1-1.dll"), "user32.dll");
        assert_eq!(canonical_dll("SHCORE"), "user32.dll");
        assert_eq!(canonical_dll("api-ms-win-core-synch-l1-2-0.dll"), "kernel32.dll");
        assert_eq!(canonical_dll("api-ms-win-appmodel-runtime-l1-1-0.dll"), "kernel32.dll");
        assert_eq!(canonical_dll("KernelBase.dll"), "kernel32.dll");
        assert_eq!(canonical_dll("PSAPI.DLL"), "kernel32.dll");
        assert_eq!(canonical_dll("ext-ms-win-ntuser-window-l1-1-0.dll"), "ext-ms-win-ntuser-window-l1-1-0.dll");
        assert_eq!(canonical_dll("api-ms-win-devices-config-l1-1-1.dll"), "setupapi.dll");
        assert_eq!(canonical_dll("cfgmgr32.dll"), "setupapi.dll");

        // Middleware & aliases
        assert_eq!(canonical_dll("xinput1_3.dll"), "xinput1_4.dll");
        assert_eq!(canonical_dll("xinput9_1_0.dll"), "xinput1_4.dll");
        assert_eq!(canonical_dll("xaudio2_7.dll"), "xaudio2_9redist.dll");
        assert_eq!(canonical_dll("MediaDecoders.WindowsDesktop.dll"), "mediadecoders_windowsdesktop.dll");
        assert_eq!(canonical_dll("cohtml.WindowsDesktop.dll"), "cohtml_windowsdesktop.dll");
        assert_eq!(canonical_dll("vulkan-1.dll"), "vulkan_1.dll");
        assert_eq!(canonical_dll("nvlowlatencyvk.dll"), "vulkan_1.dll");
        assert_eq!(canonical_dll("d3dcompiler_43.dll"), "d3dcompiler_47.dll");
        assert_eq!(canonical_dll("mono-2.0-bdwgc.dll"), "mono_2_0_bdwgc.dll");
        assert_eq!(canonical_dll("mscoreei.dll"), "mscoree.dll");
        assert_eq!(canonical_dll("clr.dll"), "coreclr.dll");
        assert_eq!(canonical_dll("steam_api.dll"), "steam_api64.dll");
        assert_eq!(canonical_dll("dstoragecore.dll"), "dstorage.dll");
        assert_eq!(canonical_dll("nvapi.dll"), "nvapi64.dll");
        assert_eq!(canonical_dll("_nvngx.dll"), "nvngx_dlss.dll");
        assert_eq!(canonical_dll("nvngx_dlssg.dll"), "nvngx_dlss.dll");
        assert_eq!(canonical_dll("libxess_dx12.dll"), "libxess.dll");
        assert_eq!(canonical_dll("fmodstudio64.dll"), "fmod.dll");
        assert_eq!(canonical_dll("oo2core_7_win64.dll"), "oo2core_9_win64.dll");
        assert_eq!(canonical_dll("tbb12.dll"), "tbbmalloc.dll");
        assert_eq!(canonical_dll("libzstd.dll"), "zstd.dll");
        assert_eq!(canonical_dll("ffx_fsr3_x64.dll"), "amd_fidelityfx_loader_dx12.dll");
        assert_eq!(canonical_dll("wsock32.dll"), "ws2_32.dll");
        assert_eq!(canonical_dll("shlwapi.dll"), "shell32.dll");
        assert_eq!(canonical_dll("comctl32.dll"), "uxtheme.dll");
        assert_eq!(canonical_dll("sfc_os.dll"), "wintrust.dll");
        assert_eq!(canonical_dll("wininet.dll"), "winhttp.dll");
        assert_eq!(canonical_dll("userenv.dll"), "advapi32.dll");
        assert_eq!(canonical_dll("sspicli.dll"), "secur32.dll");
        assert_eq!(canonical_dll("security.dll"), "secur32.dll");
        assert_eq!(canonical_dll("binkw64.dll"), "bink2w64.dll");

        // DLLs that MUST keep their own names
        for own in [
            "libxess.dll",
            "libxess_dx11.dll",
            "openal32.dll",
            "tbbmalloc.dll",
            "zstd.dll",
            "lz4.dll",
            "physx3_x64.dll",
            "physx3common_x64.dll",
            "physx3cooking_x64.dll",
            "pxfoundation_x64.dll",
            "physx_64.dll",
            "physxcommon_64.dll",
            "physxcooking_64.dll",
            "physxfoundation_64.dll",
        ] {
            assert_eq!(canonical_dll(own), own);
        }
    }

    #[test]
    fn dll_route_parse_and_roundtrip() {
        let cases = [
            ("builtin", DllRoute::Builtin { provider: None }, "builtin"),
            (
                "builtin:Bink2W64.dll",
                DllRoute::Builtin {
                    provider: Some("bink2w64.dll".into()),
                },
                "builtin:bink2w64.dll",
            ),
            ("guest", DllRoute::Guest, "guest"),
            ("pe", DllRoute::Guest, "guest"),
            ("jit", DllRoute::Guest, "guest"),
            ("fallback", DllRoute::Guest, "guest"),
            ("guest-only", DllRoute::GuestOnly, "guest-only"),
            ("strict-guest", DllRoute::GuestOnly, "guest-only"),
            ("disabled", DllRoute::Disabled, "disabled"),
            ("deny", DllRoute::Disabled, "disabled"),
            ("block", DllRoute::Disabled, "disabled"),
            ("alias:XAudio2_9Redist.dll", DllRoute::Alias("xaudio2_9redist.dll".into()), "alias:xaudio2_9redist.dll"),
        ];
        for (input, expected, toml) in cases {
            let parsed = DllRoute::parse(input).unwrap();
            assert_eq!(parsed, expected);
            assert_eq!(parsed.to_toml_value(), toml);
            assert_eq!(DllRoute::parse(&parsed.to_toml_value()).unwrap(), expected);
        }
        assert!(DllRoute::parse("").is_none());
        assert!(DllRoute::parse("builtin:").is_none());
        assert!(DllRoute::parse("alias:").is_none());
        assert!(DllRoute::parse("unknown").is_none());
    }

    #[test]
    fn routing_table_precedence_and_features() {
        let toml = r#"
[runtime]
builtin_overrides = "-bink2w64.dll,custom_legacy.dll"

[middleware.physx]
version = "3.4"
mode = "guest-only"

[middleware.xess]
mode = "disabled"

[dlls]
"libxess.dll" = "builtin"
"physx3_x64.dll" = "guest"
"my_hop1.dll" = "alias:my_hop2.dll"
"my_hop2.dll" = "alias:my_hop3.dll"
"my_hop3.dll" = "builtin:xinput1_3.dll"
"#;
        let manifest = GameManifest::parse(toml);
        let plugin_defaults = &[
            ("plugin_only.dll", "builtin"),
            ("custom_legacy.dll", "disabled"),
        ];

        // Precedence without env override:
        // defaults < plugin < builtin_overrides < [middleware.*] < [dlls]
        let t = DllRoutingTable::from_manifest(&manifest, plugin_defaults, None);
        assert!(t.is_builtin_override("plugin_only.dll"));
        // builtin_overrides beats plugin_defaults
        assert!(t.is_builtin_override("custom_legacy.dll"));
        assert!(!t.is_disabled("custom_legacy.dll"));
        // builtin_overrides disables default builtin bink2w64.dll
        assert!(!t.is_builtin_override("bink2w64.dll"));
        assert_eq!(t.route("bink2w64.dll"), DllRoute::Guest);
        // [middleware.physx] sets 3.4 DLLs to GuestOnly, but [dlls] overrides physx3_x64.dll to Guest
        assert!(t.is_guest_only("physx3common_x64.dll"));
        assert!(!t.is_guest_only("physx3_x64.dll"));
        assert_eq!(t.route("physx3_x64.dll"), DllRoute::Guest);
        // [middleware.xess] disables all XeSS DLLs, but [dlls] re-enables libxess.dll as Builtin
        assert!(t.is_builtin_override("libxess.dll"));
        assert!(t.is_disabled("libxess_dx12.dll"));
        assert!(t.is_disabled("libxess_dx11.dll"));
        // Alias chain + Builtin provider canonicalization
        assert_eq!(t.resolve_target_dll("my_hop1.dll"), "xinput1_4.dll");

        // Env override beats everything
        let t_env = DllRoutingTable::from_manifest(
            &manifest,
            plugin_defaults,
            Some("bink2w64.dll,libxess.dll=disabled,!custom_legacy.dll,my_alias.dll=alias:nvapi.dll"),
        );
        assert!(t_env.is_builtin_override("bink2w64.dll"));
        assert!(t_env.is_disabled("libxess.dll"));
        assert!(!t_env.is_builtin_override("custom_legacy.dll"));
        assert_eq!(t_env.resolve_target_dll("my_alias.dll"), "nvapi64.dll");

        // Env "0" disables all builtin overrides
        let t_zero = DllRoutingTable::from_manifest(&manifest, plugin_defaults, Some("0"));
        assert!(t_zero.disable_all_builtin_overrides());
        assert!(!t_zero.is_builtin_override("steam_api64.dll"));
        assert!(!t_zero.is_builtin_override("libxess.dll"));

        // Env "all" forces builtins
        let t_all = DllRoutingTable::from_manifest(&manifest, plugin_defaults, Some("all"));
        assert!(t_all.force_all_builtins());
        assert!(t_all.is_builtin_override("physx3_x64.dll"));
        assert!(t_all.is_builtin_override("anything_random.dll"));
    }

    #[test]
    fn should_strip_pc_dll_parity_and_routing_integration() {
        let empty = BTreeSet::new();
        let default_table = DllRoutingTable::new();
        assert!(default_table.should_strip_pc_dll("D3D12Core.dll", &empty));
        assert!(default_table.should_strip_pc_dll("nvngx_dlss.dll", &empty));
        assert!(default_table.should_strip_pc_dll("NRD.dll", &empty));
        assert!(!default_table.should_strip_pc_dll("PhysX3_x64.dll", &empty));
        assert!(!default_table.should_strip_pc_dll("readme.txt", &empty));

        // Explicit Guest route protects NRD.dll even with empty keep_dlls
        let m = GameManifest::parse(
            r#"
[dlls]
"nrd.dll" = "guest"
"custom_unused.dll" = "disabled"
"#,
        );
        let t = m.routing_table(&[], None);
        assert!(!t.should_strip_pc_dll("NRD.dll", &empty));
        assert!(t.should_strip_pc_dll("custom_unused.dll", &empty));

        // keep_dlls overrides even Disabled / EXACT_STRIP
        let keep: BTreeSet<String> = ["d3d12core.dll".into(), "custom_unused.dll".into()]
            .into_iter()
            .collect();
        assert!(!t.should_strip_pc_dll("D3D12Core.dll", &keep));
        assert!(!t.should_strip_pc_dll("custom_unused.dll", &keep));
    }
}
