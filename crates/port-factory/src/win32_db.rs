//! Wine-style Core Windows API Mapping Database (`db/win32_apis.tsv`).
//!
//! Catalogs every Windows DLL and exported API implemented natively in
//! `crates/win32-rt`, performs Windows API-set normalization
//! (`api-ms-win-crt-*` -> `ucrtbase.dll`, `api-ms-win-core-*` -> `kernel32.dll`,
//! `kernel32 <-> ntdll` forwarders), and audits any target `.exe` or `.dll`
//! for static AOT linking and Win32 API coverage.

use anyhow::{Context, Result};
use pe_image::{ImportName, Pe};
pub use port_manifest::canonical_dll;
use port_manifest::{detect_middleware_from_dlls, DetectedMiddleware};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Functional category of a Windows DLL in the Win32 API database.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Win32Category {
    /// Core OS kernel, memory, threads, sync, exceptions, security, registry (`kernel32`, `ntdll`, `advapi32`, `bcrypt`, `crypt32`, `secur32`, `wintrust`, `version`, `dbghelp`, `normaliz`).
    CoreOs,
    /// C/C++ runtime library (`ucrtbase`, `msvcrt`, `vcruntime140*`, `msvcp140*`).
    CrtStl,
    /// Windowing, input, shell, and COM (`user32`, `gdi32`, `shell32`, `ole32`, `oleaut32`, `dwmapi`, `hid`, `setupapi`, `xinput1_4`, `dinput8`).
    WindowInputCom,
    /// Networking, HTTP, LDAP, and RPC (`ws2_32`, `winhttp`, `wldap32`, `iphlpapi`, `rpcrt4`, `libcurl`).
    NetRpc,
    /// Graphics, audio, and media (`d3d12`, `dxgi`, `d3d11`, `winmm`, `mmdevapi`, `avrt`, `dsound`, `mfplat`, `avifil32`, `xaudio2_9redist`).
    GraphicsMediaAudio,
    /// Bundled game/engine middleware overrides (`zlib1`, `v8_zlib`, `libprotobuf`, `physx*`, `pxfoundation*`, `nvcloth*`, `cohtml*`, `bink2w64`, `nrd`, `nri`, `amd_fidelityfx*`, `sl.interposer`, `eossdk*`).
    EngineMiddleware,
}

impl Win32Category {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreOs => "CORE_OS",
            Self::CrtStl => "CRT_STL",
            Self::WindowInputCom => "WINDOW_INPUT_COM",
            Self::NetRpc => "NET_RPC",
            Self::GraphicsMediaAudio => "GRAPHICS_MEDIA_AUDIO",
            Self::EngineMiddleware => "ENGINE_MIDDLEWARE",
        }
    }

    pub fn for_dll(dll: &str) -> Self {
        match dll {
            "kernel32.dll" | "ntdll.dll" | "advapi32.dll" | "bcrypt.dll" | "crypt32.dll"
            | "secur32.dll" | "wintrust.dll" | "version.dll" | "dbghelp.dll" | "normaliz.dll" => {
                Self::CoreOs
            }
            "ucrtbase.dll"
            | "msvcrt.dll"
            | "vcruntime140.dll"
            | "vcruntime140_1.dll"
            | "msvcp140.dll"
            | "msvcp140_codecvt_ids.dll"
            | "mscoree.dll"
            | "hostfxr.dll"
            | "coreclr.dll" => Self::CrtStl,
            "user32.dll" | "gdi32.dll" | "shell32.dll" | "ole32.dll" | "oleaut32.dll"
            | "dwmapi.dll" | "hid.dll" | "setupapi.dll" | "xinput1_4.dll" | "dinput8.dll"
            | "comdlg32.dll" | "uxtheme.dll" | "imm32.dll" => Self::WindowInputCom,
            "ws2_32.dll" | "winhttp.dll" | "wldap32.dll" | "iphlpapi.dll" | "rpcrt4.dll"
            | "libcurl.dll" => Self::NetRpc,
            "d3d12.dll"
            | "dxgi.dll"
            | "d3d11.dll"
            | "d3dcompiler_47.dll"
            | "dwrite.dll"
            | "d2d1.dll"
            | "vulkan_1.dll"
            | "directml.dll"
            | "opencl.dll"
            | "nvcuda.dll"
            | "nvapi64.dll"
            | "amd_ags_x64.dll"
            | "nvngx_dlss.dll"
            | "dstorage.dll"
            | "fmod.dll"
            | "winmm.dll"
            | "mmdevapi.dll"
            | "avrt.dll"
            | "dsound.dll"
            | "mfplat.dll"
            | "avifil32.dll"
            | "xaudio2_9redist.dll"
            | "mediadecoders_windowsdesktop.dll" => Self::GraphicsMediaAudio,
            _ => Self::EngineMiddleware,
        }
    }
}

/// Implementation maturity of an exported Win32/middleware API in `crates/win32-rt`,
/// derived statically from its Rust source definition by `win32-sync`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImplKind {
    /// Full native implementation (no `stub!`, `failing!`, or `win64_stubs!` in body).
    Real,
    /// Handles real cases/branches, with `stub!` fallback(s) for unsupported sub-cases.
    Partial,
    /// Unconditional `stub!` body (or `win64_stubs!` macro), e.g. no-op or unimplemented body.
    Stub,
    /// Explicit `failing!(...)` macro stub that sets last error and returns a failure sentinel.
    Failing,
}

impl ImplKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Real => "real",
            Self::Partial => "partial",
            Self::Stub => "stub",
            Self::Failing => "failing",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "real" => Some(Self::Real),
            "partial" => Some(Self::Partial),
            "stub" => Some(Self::Stub),
            "failing" => Some(Self::Failing),
            _ => None,
        }
    }

    /// True if this export is a pure stub or explicit failing macro stub.
    pub const fn is_stub_or_failing(self) -> bool {
        matches!(self, Self::Stub | Self::Failing)
    }
}

/// One exported Windows API entry in `db/win32_apis.tsv`.
#[derive(Debug, Clone)]
pub struct Win32ApiEntry {
    pub dll: String,
    pub symbol: String,
    pub ordinal: Option<u16>,
    pub category: Win32Category,
    pub impl_kind: ImplKind,
    pub rust_path: String,
    pub source_file: String,
}

/// Complete in-memory index of the Win32 API database.
#[derive(Debug, Clone, Default)]
pub struct Win32Database {
    pub entries: Vec<Win32ApiEntry>,
    by_dll_sym: HashMap<(String, String), usize>,
    by_dll_ord: HashMap<(String, u16), usize>,
}

impl Win32Database {
    /// Scan `crates/win32-rt/src` to build the canonical [`Win32Database`].
    pub fn from_workspace_src(workspace_root: &Path) -> Result<Self> {
        let src_root = workspace_root.join("crates/win32-rt/src");
        let mut entries = Vec::new();

        // 1. Core DLLs scanned just like `crates/win32-rt/build.rs`.
        let mut k32_files = rs_files(&src_root.join("kernel32"));
        k32_files.push(src_root.join("sync.rs"));
        k32_files.push(src_root.join("unwind.rs"));
        scan_dll_table(workspace_root, &src_root, "kernel32.dll", &k32_files, &mut entries);
        scan_dll_table(
            workspace_root,
            &src_root,
            "ntdll.dll",
            &[src_root.join("ntdll.rs")],
            &mut entries,
        );
        scan_dll_table(
            workspace_root,
            &src_root,
            "ucrtbase.dll",
            &rs_files(&src_root.join("ucrt")),
            &mut entries,
        );
        scan_dll_table(
            workspace_root,
            &src_root,
            "msvcrt.dll",
            &[src_root.join("msvcrt.rs")],
            &mut entries,
        );
        scan_dll_table(
            workspace_root,
            &src_root,
            "advapi32.dll",
            &[src_root.join("advapi32.rs")],
            &mut entries,
        );
        scan_dll_table(
            workspace_root,
            &src_root,
            "user32.dll",
            &rs_files(&src_root.join("user32")),
            &mut entries,
        );

        // 2. Explicit `exports![...]` tables (`d3d12`, `dxgi`, `d3d11`, `sl.interposer`, `eossdk`).
        scan_exports_macro_file(
            workspace_root,
            &src_root,
            "d3d12.dll",
            &src_root.join("d3d12/mod.rs"),
            &mut entries,
        );
        scan_exports_macro_file(
            workspace_root,
            &src_root,
            "dxgi.dll",
            &src_root.join("dxgi/mod.rs"),
            &mut entries,
        );
        scan_exports_macro_file(
            workspace_root,
            &src_root,
            "d3d11.dll",
            &src_root.join("d3d11/mod.rs"),
            &mut entries,
        );
        scan_exports_macro_file(
            workspace_root,
            &src_root,
            "sl.interposer.dll",
            &src_root.join("slstub.rs"),
            &mut entries,
        );
        scan_exports_macro_file(
            workspace_root,
            &src_root,
            "eossdk-win64-shipping.dll",
            &src_root.join("eossdk.rs"),
            &mut entries,
        );

        // 3. All `src/misc/<stem>.rs` DLLs (including `EXTRA_EXPORTS` and `ORDINALS`).
        for f in rs_files(&src_root.join("misc")) {
            let stem = f.file_stem().unwrap().to_string_lossy().into_owned();
            if stem == "mod" {
                continue;
            }
            let dll_name = format!("{stem}.dll");
            scan_dll_table(
                workspace_root,
                &src_root,
                &dll_name,
                std::slice::from_ref(&f),
                &mut entries,
            );
            scan_extra_exports_and_ordinals(workspace_root, &src_root, &dll_name, &f, &mut entries);
        }

        // 4. Propagate `impl_kind` across cross-module forwarders (e.g. `sl.interposer.dll`
        //    re-exporting `crate::d3d12::D3D12CreateRootSignatureDeserializer`).
        let mut by_rust_path: HashMap<String, ImplKind> = HashMap::new();
        for e in &entries {
            let prev = by_rust_path.get(&e.rust_path).copied();
            let merged = match prev {
                None => e.impl_kind,
                Some(ImplKind::Real) => e.impl_kind,
                Some(other) => other,
            };
            by_rust_path.insert(e.rust_path.clone(), merged);
        }
        for e in &mut entries {
            if e.impl_kind == ImplKind::Real {
                if let Some(&target_kind) = by_rust_path.get(&e.rust_path) {
                    e.impl_kind = target_kind;
                }
            }
        }

        Ok(Self::from_entries(entries))
    }

    /// Count entries matching a specific [`ImplKind`].
    pub fn count_by_impl_kind(&self, kind: ImplKind) -> usize {
        self.entries.iter().filter(|e| e.impl_kind == kind).count()
    }

    /// Return counts grouped by [`ImplKind`] across the full database.
    pub fn impl_kind_counts(&self) -> BTreeMap<ImplKind, usize> {
        let mut counts = BTreeMap::new();
        for kind in [
            ImplKind::Real,
            ImplKind::Partial,
            ImplKind::Stub,
            ImplKind::Failing,
        ] {
            counts.insert(kind, 0);
        }
        for e in &self.entries {
            *counts.entry(e.impl_kind).or_insert(0) += 1;
        }
        counts
    }

    pub fn from_entries(mut entries: Vec<Win32ApiEntry>) -> Self {
        entries.sort_by(|a, b| a.dll.cmp(&b.dll).then_with(|| a.symbol.cmp(&b.symbol)));
        let mut by_dll_sym = HashMap::new();
        let mut by_dll_ord = HashMap::new();
        for (idx, e) in entries.iter().enumerate() {
            by_dll_sym.insert((e.dll.clone(), e.symbol.clone()), idx);
            if let Some(ord) = e.ordinal {
                by_dll_ord.insert((e.dll.clone(), ord), idx);
            }
        }
        Self {
            entries,
            by_dll_sym,
            by_dll_ord,
        }
    }

    /// Resolve an import `(dll, ImportName)` using Wine-style API-set normalization and
    /// `kernel32 <-> ntdll` / `msvcrt -> ucrtbase` forwarding.
    pub fn resolve(&self, dll: &str, name: &ImportName) -> Option<&Win32ApiEntry> {
        let c = canonical_dll(dll);
        match name {
            ImportName::Name { name: sym, .. } => {
                if let Some(&idx) = self.by_dll_sym.get(&(c.clone(), sym.clone())) {
                    return Some(&self.entries[idx]);
                }
                let fallback = match c.as_str() {
                    "kernel32.dll" => Some("ntdll.dll"),
                    "ntdll.dll" => Some("kernel32.dll"),
                    "msvcrt.dll" => Some("ucrtbase.dll"),
                    _ => None,
                };
                if let Some(fb) = fallback {
                    if let Some(&idx) = self.by_dll_sym.get(&(fb.to_string(), sym.clone())) {
                        return Some(&self.entries[idx]);
                    }
                }
                None
            }
            ImportName::Ordinal(ord) => self
                .by_dll_ord
                .get(&(c, *ord))
                .map(|&idx| &self.entries[idx]),
        }
    }

    /// Format the canonical Win32 API database as TSV (`db/win32_apis.tsv`).
    pub fn format_tsv(&self) -> String {
        use std::fmt::Write as _;
        let mut out = String::from(
            "dll\tsymbol\tordinal\tcategory\timpl_kind\trust_path\tsource_file\n",
        );
        for e in &self.entries {
            let ord_str = e
                .ordinal
                .map(|o| o.to_string())
                .unwrap_or_else(|| "-".to_string());
            let _ = writeln!(
                out,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                e.dll,
                e.symbol,
                ord_str,
                e.category.as_str(),
                e.impl_kind.as_str(),
                e.rust_path,
                e.source_file
            );
        }
        out
    }

    /// Write the canonical Win32 API database to `db/win32_apis.tsv`.
    pub fn write_tsv(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut f = fs::File::create(path).with_context(|| format!("create {}", path.display()))?;
        f.write_all(self.format_tsv().as_bytes())?;
        Ok(())
    }
}

/// One resolved import whose builtin implementation is not a full `real` implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonRealImport {
    pub dll: String,
    pub symbol: String,
    pub impl_kind: ImplKind,
    pub source_file: String,
}

/// Summary of auditing a PE binary's imports against [`Win32Database`].
#[derive(Debug, Clone)]
pub struct Win32ImportAudit {
    pub binary_name: String,
    pub total_imports: usize,
    /// Total imports matched by either a builtin export or a sibling PE DLL.
    pub resolved_win32: usize,
    /// Imports satisfied by a sibling PE DLL shipped alongside the binary (executed via x86-runtime).
    pub resolved_sibling: usize,
    /// Builtin-resolved imports grouped by [`ImplKind`] (`real`, `partial`, `stub`, `failing`).
    pub resolved_by_impl_kind: BTreeMap<ImplKind, usize>,
    pub resolved_by_category: BTreeMap<Win32Category, usize>,
    /// Builtin-resolved imports whose `impl_kind` is `stub` or `failing`.
    pub stubbed_imports: Vec<NonRealImport>,
    /// Builtin-resolved imports whose `impl_kind` is `partial`.
    pub partial_imports: Vec<NonRealImport>,
    pub unresolved: Vec<(String, String)>,
    pub detected_middleware: Vec<DetectedMiddleware>,
}

impl Win32ImportAudit {
    /// Percentage of imports that have any symbol binding (including stubs, failing stubs, and sibling PEs).
    pub fn coverage_pct(&self) -> f64 {
        if self.total_imports == 0 {
            100.0
        } else {
            (self.resolved_win32 as f64 * 100.0) / (self.total_imports as f64)
        }
    }

    /// Count of builtin-resolved imports with the given [`ImplKind`].
    pub fn count_by_impl_kind(&self, kind: ImplKind) -> usize {
        self.resolved_by_impl_kind.get(&kind).copied().unwrap_or(0)
    }

    /// Count of imports backed by real code (builtin `real` + sibling PE DLLs).
    pub fn real_resolved(&self) -> usize {
        self.count_by_impl_kind(ImplKind::Real) + self.resolved_sibling
    }

    /// Percentage of imports backed by real code (`real` builtins + sibling PEs).
    pub fn real_coverage_pct(&self) -> f64 {
        if self.total_imports == 0 {
            100.0
        } else {
            (self.real_resolved() as f64 * 100.0) / (self.total_imports as f64)
        }
    }

    pub fn format_report(&self) -> String {
        format_audit_report(self)
    }
}

/// Format a [`Win32ImportAudit`] report, including separate `impl_kind` counts
/// (`real`, `partial`, `stub`, `failing`), resolved category counts, detected
/// multi-version middleware, stubbed/failing imports used, and unresolved imports.
pub fn format_audit_report(audit: &Win32ImportAudit) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let real_cnt = audit.count_by_impl_kind(ImplKind::Real);
    let partial_cnt = audit.count_by_impl_kind(ImplKind::Partial);
    let stub_cnt = audit.count_by_impl_kind(ImplKind::Stub);
    let failing_cnt = audit.count_by_impl_kind(ImplKind::Failing);
    let _ = writeln!(
        out,
        "=== Win32 Import Audit: {} ({}/{} resolved = {:.2}%, {}/{} real = {:.2}%) ===",
        audit.binary_name,
        audit.resolved_win32,
        audit.total_imports,
        audit.coverage_pct(),
        audit.real_resolved(),
        audit.total_imports,
        audit.real_coverage_pct()
    );
    let _ = writeln!(out, "  Implementation Breakdown:");
    if audit.resolved_sibling > 0 {
        let _ = writeln!(
            out,
            "    {:22} : {:>5} (+ {} sibling PE)",
            "real", real_cnt, audit.resolved_sibling
        );
    } else {
        let _ = writeln!(out, "    {:22} : {:>5}", "real", real_cnt);
    }
    let _ = writeln!(out, "    {:22} : {:>5}", "partial", partial_cnt);
    let _ = writeln!(out, "    {:22} : {:>5}", "stub", stub_cnt);
    let _ = writeln!(out, "    {:22} : {:>5}", "failing", failing_cnt);
    let _ = writeln!(out, "    {:22} : {:>5}", "unresolved", audit.unresolved.len());
    for (cat, cnt) in &audit.resolved_by_category {
        let _ = writeln!(out, "  {:24} : {:>5}", cat.as_str(), cnt);
    }
    if !audit.detected_middleware.is_empty() {
        let _ = writeln!(
            out,
            "  Detected Middleware ({}):",
            audit.detected_middleware.len()
        );
        for mw in &audit.detected_middleware {
            let _ = writeln!(
                out,
                "    {} ({}) v{} [default: {}] <- {}",
                mw.family.display_name,
                mw.family.id,
                mw.version.version,
                mw.family.default_mode.as_str(),
                mw.matched_dlls.join(", ")
            );
        }
    }
    if !audit.stubbed_imports.is_empty() {
        let _ = writeln!(
            out,
            "  Stubbed / Failing Imports ({}):",
            audit.stubbed_imports.len()
        );
        for imp in audit.stubbed_imports.iter().take(200) {
            let _ = writeln!(
                out,
                "    {}!{} [{}] ({})",
                imp.dll,
                imp.symbol,
                imp.impl_kind.as_str(),
                imp.source_file
            );
        }
    }
    if !audit.partial_imports.is_empty() {
        let _ = writeln!(out, "  Partial Imports ({}):", audit.partial_imports.len());
        for imp in audit.partial_imports.iter().take(200) {
            let _ = writeln!(
                out,
                "    {}!{} [partial] ({})",
                imp.dll, imp.symbol, imp.source_file
            );
        }
    }
    if !audit.unresolved.is_empty() {
        let _ = writeln!(out, "  Unresolved ({}):", audit.unresolved.len());
        for (dll, sym) in audit.unresolved.iter().take(200) {
            let _ = writeln!(out, "    {dll}!{sym}");
        }
    }
    out
}

/// Audit all regular and delay-load imports of an already-parsed [`Pe`] image against [`Win32Database`].
pub fn audit_pe_imports_parsed(
    binary_name: &str,
    pe: &Pe,
    db: &Win32Database,
    sibling_exports: Option<&HashMap<(String, String), u32>>,
) -> Win32ImportAudit {
    let mut audit = Win32ImportAudit {
        binary_name: binary_name.to_string(),
        total_imports: 0,
        resolved_win32: 0,
        resolved_sibling: 0,
        resolved_by_impl_kind: BTreeMap::new(),
        resolved_by_category: BTreeMap::new(),
        stubbed_imports: Vec::new(),
        partial_imports: Vec::new(),
        unresolved: Vec::new(),
        detected_middleware: Vec::new(),
    };

    let mut all_dlls = pe.imports().unwrap_or_default();
    if let Ok(delay) = pe.delay_imports() {
        all_dlls.extend(delay);
    }

    let mut imported_dlls = Vec::with_capacity(all_dlls.len());
    for imp in all_dlls {
        let norm_dll = pe_image::normalize_dll_name(&imp.dll);
        imported_dlls.push(norm_dll.clone());
        for ent in &imp.entries {
            audit.total_imports += 1;
            if let Some(hit) = db.resolve(&norm_dll, &ent.name) {
                audit.resolved_win32 += 1;
                *audit.resolved_by_impl_kind.entry(hit.impl_kind).or_insert(0) += 1;
                *audit.resolved_by_category.entry(hit.category).or_insert(0) += 1;
                match hit.impl_kind {
                    ImplKind::Stub | ImplKind::Failing => {
                        audit.stubbed_imports.push(NonRealImport {
                            dll: norm_dll.clone(),
                            symbol: ent.name.to_string(),
                            impl_kind: hit.impl_kind,
                            source_file: hit.source_file.clone(),
                        });
                    }
                    ImplKind::Partial => {
                        audit.partial_imports.push(NonRealImport {
                            dll: norm_dll.clone(),
                            symbol: ent.name.to_string(),
                            impl_kind: hit.impl_kind,
                            source_file: hit.source_file.clone(),
                        });
                    }
                    ImplKind::Real => {}
                }
                continue;
            }
            if let Some(sib) = sibling_exports {
                let sym_str = ent.name.to_string();
                if sib.contains_key(&(norm_dll.clone(), sym_str)) {
                    audit.resolved_win32 += 1;
                    audit.resolved_sibling += 1;
                    *audit
                        .resolved_by_category
                        .entry(Win32Category::EngineMiddleware)
                        .or_insert(0) += 1;
                    continue;
                }
            }
            audit.unresolved.push((norm_dll.clone(), ent.name.to_string()));
        }
    }
    audit.detected_middleware =
        detect_middleware_from_dlls(imported_dlls.iter().map(String::as_str));
    audit
}

/// Audit all regular and delay-load imports of a PE image against [`Win32Database`].
pub fn audit_pe_imports(
    binary_name: &str,
    data: &[u8],
    db: &Win32Database,
    sibling_exports: Option<&HashMap<(String, String), u32>>,
) -> Result<Win32ImportAudit> {
    let pe = Pe::parse(data).with_context(|| format!("parse PE {binary_name}"))?;
    Ok(audit_pe_imports_parsed(binary_name, &pe, db, sibling_exports))
}

fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "rs"))
                .collect()
        })
        .unwrap_or_default();
    v.sort();
    v
}

fn rust_module_path(src_root: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(src_root).unwrap().with_extension("");
    let mut parts: Vec<String> = rel
        .iter()
        .map(|c| c.to_string_lossy().into_owned())
        .collect();
    if parts.last().is_some_and(|l| l == "mod") {
        parts.pop();
    }
    let mut s = String::from("win32_rt");
    for p in parts {
        s.push_str("::");
        s.push_str(&p);
    }
    s
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Produce a same-length byte mask of `src` where comments and string/char literals
/// are replaced with ASCII spaces (preserving newlines and byte offsets).
pub(crate) fn mask_non_code(src: &str) -> Vec<u8> {
    let bytes = src.as_bytes();
    let n = bytes.len();
    let mut out = vec![b' '; n];
    let mut i = 0usize;
    while i < n {
        let b = bytes[i];
        let next = if i + 1 < n { bytes[i + 1] } else { 0 };
        // Line comment
        if b == b'/' && next == b'/' {
            i += 2;
            while i < n && bytes[i] != b'\n' {
                i += 1;
            }
            if i < n {
                out[i] = b'\n';
                i += 1;
            }
            continue;
        }
        // Block comment (supports nesting)
        if b == b'/' && next == b'*' {
            let mut depth = 1usize;
            i += 2;
            while i < n && depth > 0 {
                if bytes[i] == b'\n' {
                    out[i] = b'\n';
                    i += 1;
                } else if i + 1 < n && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if i + 1 < n && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        // Raw string literal: r"..." or r#"..."# (or br"..." / cr"...")
        if (b == b'r' || (b == b'b' && next == b'r') || (b == b'c' && next == b'r'))
            && (i == 0 || !is_ident_byte(bytes[i - 1]))
        {
            let r_pos = if b == b'r' { i } else { i + 1 };
            let mut j = r_pos + 1;
            let mut hashes = 0usize;
            while j < n && bytes[j] == b'#' {
                hashes += 1;
                j += 1;
            }
            if j < n && bytes[j] == b'"' {
                j += 1;
                while j < n {
                    if bytes[j] == b'\n' {
                        out[j] = b'\n';
                    }
                    if bytes[j] == b'"' {
                        let mut k = j + 1;
                        let mut matched = 0usize;
                        while k < n && matched < hashes && bytes[k] == b'#' {
                            matched += 1;
                            k += 1;
                        }
                        if matched == hashes {
                            i = k;
                            break;
                        }
                    }
                    j += 1;
                }
                if j >= n {
                    i = n;
                }
                continue;
            }
        }
        // Normal string literal `"..."`
        if b == b'"' {
            i += 1;
            while i < n {
                if bytes[i] == b'\n' {
                    out[i] = b'\n';
                    i += 1;
                } else if bytes[i] == b'\\' {
                    i += 2;
                } else if bytes[i] == b'"' {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        // Character literal `'a'` or `'\n'` (avoid matching lifetimes like `'static`)
        if b == b'\'' {
            if i + 2 < n && bytes[i + 1] != b'\\' && bytes[i + 2] == b'\'' {
                i += 3;
                continue;
            }
            if i + 2 < n && bytes[i + 1] == b'\\' {
                let mut j = i + 2;
                while j < n && bytes[j] != b'\'' && bytes[j] != b'\n' {
                    j += 1;
                }
                if j < n && bytes[j] == b'\'' {
                    i = j + 1;
                    continue;
                }
            }
        }
        out[i] = b;
        i += 1;
    }
    out
}

/// Match balanced delimiters (`open` / `close`) starting at `open_idx` in a masked code slice.
fn find_matching_delim(masked: &[u8], open_idx: usize, open: u8, close: u8) -> Option<usize> {
    if masked.get(open_idx) != Some(&open) {
        return None;
    }
    let mut depth = 0usize;
    for (idx, &b) in masked.iter().enumerate().skip(open_idx) {
        if b == open {
            depth += 1;
        } else if b == close {
            depth -= 1;
            if depth == 0 {
                return Some(idx);
            }
        }
    }
    None
}

/// Find every `stub!(` invocation in a masked function body (where outer `{` is at index 0, depth 1).
/// Returns `(byte_offset, brace_depth, end_byte_offset_exclusive)` for each call.
fn find_stub_calls(masked_body: &[u8]) -> Vec<(usize, usize, usize)> {
    let mut calls = Vec::new();
    let n = masked_body.len();
    let mut depth = 0usize;
    let mut i = 0usize;
    while i < n {
        match masked_body[i] {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            b's' if i + 5 <= n
                && &masked_body[i..i + 5] == b"stub!"
                && (i == 0 || !is_ident_byte(masked_body[i - 1])) =>
            {
                let mut j = i + 5;
                while j < n && masked_body[j].is_ascii_whitespace() {
                    j += 1;
                }
                if j < n && matches!(masked_body[j], b'(' | b'[' | b'{') {
                    let close = match masked_body[j] {
                        b'(' => b')',
                        b'[' => b']',
                        _ => b'}',
                    };
                    let end = find_matching_delim(masked_body, j, masked_body[j], close)
                        .map(|e| e + 1)
                        .unwrap_or(j + 1);
                    calls.push((i, depth, end));
                    i = end;
                    continue;
                }
                i += 1;
            }
            _ => i += 1,
        }
    }
    calls
}

/// Check if a `return <expr>` expression represents a validation/error guard rather than a real success path.
fn is_error_guard_return(expr: &str) -> bool {
    let t = expr.trim();
    if t.is_empty()
        || matches!(
            t,
            "0" | "-1" | "()" | "FALSE" | "false" | "None" | "std::ptr::null_mut()" | "std::ptr::null()"
        )
    {
        return true;
    }
    // Explicit success tokens override generic substrings like `ERROR_SUCCESS`.
    if t.contains("STATUS_SUCCESS")
        || t.contains("ERROR_SUCCESS")
        || t.contains("S_OK")
        || t.contains("AGS_SUCCESS")
        || t.contains("XESS_RESULT_SUCCESS")
        || t == "TRUE"
        || t == "true"
        || t == "OK"
    {
        return false;
    }
    let upper = t.to_ascii_uppercase();
    upper.contains("INVALID")
        || upper.contains("NOT_SUPPORTED")
        || upper.contains("NOT_IMPLEMENTED")
        || upper.contains("NOTIMPL")
        || upper.contains("NOINTERFACE")
        || upper.contains("UNSUPPORTED")
        || upper.contains("ERROR")
        || upper.contains("FAIL")
        || upper.contains("DENIED")
        || upper.contains("E_POINTER")
        || upper.contains("E_HANDLE")
        || upper.contains("E_OUTOFMEMORY")
        || upper.contains("E_ABORT")
        || upper.contains("FALSE")
        || t.contains("null_mut()")
        || t.contains("null()")
        || t.starts_with("err(")
        || t.starts_with("Err(")
}

/// Returns true if the code prefix before an unconditional `stub!(` already contains a real
/// non-error return path (e.g. `BCryptImportKey`, `BCryptExportKey`, `BCryptSetProperty`).
fn prefix_has_real_success_path(prefix_masked: &str) -> bool {
    // 1. Look for `return <expr>;` whose expression is not a pure validation/error guard.
    let bytes = prefix_masked.as_bytes();
    let n = bytes.len();
    let mut i = 0usize;
    while i + 6 <= n {
        if &bytes[i..i + 6] == b"return"
            && (i == 0 || !is_ident_byte(bytes[i - 1]))
            && (i + 6 == n || !is_ident_byte(bytes[i + 6]))
        {
            if let Some(semi_rel) = prefix_masked[i + 6..].find(';') {
                let expr = &prefix_masked[i + 6..i + 6 + semi_rel];
                if !is_error_guard_return(expr) {
                    return true;
                }
                i += 6 + semi_rel + 1;
                continue;
            }
        }
        i += 1;
    }
    // 2. Also treat an explicit `match` with a success value before the fallback `stub!` as partial.
    prefix_masked.contains("match ")
        && (prefix_masked.contains("STATUS_SUCCESS")
            || prefix_masked.contains("S_OK")
            || prefix_masked.contains("ERROR_SUCCESS"))
}

/// Returns true if a body whose `stub!(` calls are all inside conditional branches performs real
/// work outside those `stub!` calls (as opposed to a pure null-guarded stub like `CredFree`).
fn body_has_real_work_outside_stub(masked_body: &str, stub_calls: &[(usize, usize, usize)]) -> bool {
    let mut stripped = masked_body.as_bytes().to_vec();
    let slen = stripped.len();
    for &(start, _, end) in stub_calls {
        for b in &mut stripped[start..end.min(slen)] {
            *b = b' ';
        }
    }
    let clean = std::str::from_utf8(&stripped).unwrap_or("");
    // Strip `crate::` leftovers immediately before a blanked stub! call.
    for kw in ["match ", "match(", "let ", "for ", "while ", "loop "] {
        if clean.contains(kw) {
            return true;
        }
    }
    // Any function/method call other than `.is_null()` or `.is_empty()` counts as real work.
    let bytes = clean.as_bytes();
    let n = bytes.len();
    let mut i = 0usize;
    while i < n {
        if bytes[i] == b'(' && i > 0 {
            let mut j = i;
            while j > 0 && bytes[j - 1].is_ascii_whitespace() {
                j -= 1;
            }
            let end_ident = j;
            while j > 0 && is_ident_byte(bytes[j - 1]) {
                j -= 1;
            }
            if end_ident > j {
                let ident = &clean[j..end_ident];
                if !matches!(ident, "if" | "is_null" | "is_empty") {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
}

/// Classify a single Rust function body (`{ ... }`) into [`ImplKind`].
pub(crate) fn classify_fn_body(body: &str) -> ImplKind {
    let masked = mask_non_code(body);
    let stub_calls = find_stub_calls(&masked);
    if stub_calls.is_empty() {
        return ImplKind::Real;
    }
    if let Some(&(first_top_offset, _, _)) = stub_calls.iter().find(|&&(_, depth, _)| depth == 1) {
        let prefix = std::str::from_utf8(&masked[..first_top_offset]).unwrap_or("");
        if prefix_has_real_success_path(prefix) {
            ImplKind::Partial
        } else {
            ImplKind::Stub
        }
    } else {
        let masked_str = std::str::from_utf8(&masked).unwrap_or("");
        if body_has_real_work_outside_stub(masked_str, &stub_calls) {
            ImplKind::Partial
        } else {
            ImplKind::Stub
        }
    }
}

/// Extract all `failing!(...)`, `win64_stubs!(...)`, and `fn name(...) { ... }` definitions from
/// one Rust source file (already trimmed before `#[cfg(test)]`) and map each symbol to its [`ImplKind`].
pub(crate) fn analyze_file_impl_kinds(src: &str) -> HashMap<String, ImplKind> {
    let mut map = HashMap::new();
    let masked = mask_non_code(src);
    let masked_str = std::str::from_utf8(&masked).unwrap_or("");

    // 1. Classify macro-generated `failing!(...)` and `win64_stubs!(...)` exports.
    for (prefix, kind) in [
        ("failing!(", ImplKind::Failing),
        ("win64_stubs!(", ImplKind::Stub),
    ] {
        let mut search_from = 0usize;
        while let Some(rel) = masked_str[search_from..].find(prefix) {
            let pos = search_from + rel;
            if pos > 0 && is_ident_byte(masked[pos - 1]) {
                search_from = pos + 1;
                continue;
            }
            let open_paren = pos + prefix.len() - 1;
            let Some(close_paren) = find_matching_delim(&masked, open_paren, b'(', b')') else {
                break;
            };
            let block = &masked_str[open_paren + 1..close_paren];
            let list_part = block.split_once(';').map(|(_, rhs)| rhs).unwrap_or(block);
            for raw_sym in list_part.split(',') {
                let name: String = raw_sym
                    .trim()
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() && !name.starts_with("dispatch_") && !name.starts_with("aw2_") {
                    map.insert(name, kind);
                }
            }
            search_from = close_paren + 1;
        }
    }

    // 2. Classify all `fn <ident>(...) { ... }` definitions in this file.
    let n = masked.len();
    let mut i = 0usize;
    while i + 3 <= n {
        if &masked[i..i + 2] == b"fn"
            && (i == 0 || !is_ident_byte(masked[i - 1]))
            && masked[i + 2].is_ascii_whitespace()
        {
            // Skip `fn(...)` function-pointer types (not followed by an identifier).
            let mut j = i + 2;
            while j < n && masked[j].is_ascii_whitespace() {
                j += 1;
            }
            let name_start = j;
            while j < n && is_ident_byte(masked[j]) {
                j += 1;
            }
            if j > name_start {
                let name = &src[name_start..j];
                // Skip optional whitespace + generics `<...>`
                let mut k = j;
                while k < n && masked[k].is_ascii_whitespace() {
                    k += 1;
                }
                if k < n && masked[k] == b'<' {
                    let mut gdepth = 0usize;
                    while k < n {
                        if masked[k] == b'<' {
                            gdepth += 1;
                        } else if masked[k] == b'>' {
                            gdepth -= 1;
                            if gdepth == 0 {
                                k += 1;
                                break;
                            }
                        }
                        k += 1;
                    }
                    while k < n && masked[k].is_ascii_whitespace() {
                        k += 1;
                    }
                }
                if k < n && masked[k] == b'(' {
                    if let Some(close_p) = find_matching_delim(&masked, k, b'(', b')') {
                        // Advance past return type / where clause to `{` or `;`.
                        let mut bpos = close_p + 1;
                        while bpos < n && masked[bpos] != b'{' && masked[bpos] != b';' {
                            bpos += 1;
                        }
                        if bpos < n && masked[bpos] == b'{' {
                            if let Some(close_b) = find_matching_delim(&masked, bpos, b'{', b'}') {
                                let body = &src[bpos..=close_b];
                                let kind = classify_fn_body(body);
                                map.entry(name.to_string()).or_insert(kind);
                                i = close_b + 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }

    map
}

/// Extract the target Rust function identifier from an `EXTRA_EXPORTS` `ptr:` field expression,
/// handling `Ptr(foo as *const ())`, `crate::Ptr(crate::bridge_entry!(foo))`, etc.
fn extract_ptr_target_ident(ptr_expr: &str) -> Option<&str> {
    if let Some((_, after_bridge)) = ptr_expr.split_once("bridge_entry!(") {
        let ident = after_bridge
            .trim_start()
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .next()?;
        if !ident.is_empty() {
            return Some(ident);
        }
    }
    let inner = ptr_expr
        .split_once("Ptr(")
        .map(|(_, rhs)| rhs)
        .unwrap_or(ptr_expr)
        .trim_start();
    let before_as = inner.split(" as ").next().unwrap_or(inner).trim();
    let last_seg = before_as.rsplit("::").next().unwrap_or(before_as).trim();
    let ident = last_seg
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .next()?;
    (!ident.is_empty()).then_some(ident)
}

fn scan_dll_table(
    workspace_root: &Path,
    src_root: &Path,
    dll_name: &str,
    files: &[PathBuf],
    out: &mut Vec<Win32ApiEntry>,
) {
    let mut seen = BTreeSet::new();
    let cat = Win32Category::for_dll(dll_name);
    for file in files {
        let Ok(raw) = fs::read_to_string(file) else {
            continue;
        };
        let src = raw.split("#[cfg(test)]").next().unwrap_or("");
        let file_kinds = analyze_file_impl_kinds(src);
        let mod_path = rust_module_path(src_root, file);
        let rel_file = file
            .strip_prefix(workspace_root)
            .unwrap_or(file)
            .to_string_lossy()
            .into_owned();
        for line in src.lines() {
            let t = line.trim_start();
            if let Some(r) = t.strip_prefix("// @exports ") {
                for name in r.split_whitespace() {
                    if seen.insert(name.to_string()) {
                        let impl_kind = file_kinds.get(name).copied().unwrap_or(ImplKind::Real);
                        out.push(Win32ApiEntry {
                            dll: dll_name.to_string(),
                            symbol: name.to_string(),
                            ordinal: None,
                            category: cat,
                            impl_kind,
                            rust_path: format!("{mod_path}::{name}"),
                            source_file: rel_file.clone(),
                        });
                    }
                }
                continue;
            }
            if t.len() != line.len() {
                continue;
            }
            let rest = if let Some(r) = t.strip_prefix("pub unsafe extern \"win64\" fn ") {
                r
            } else if let Some(r) = t.strip_prefix("pub extern \"win64\" fn ") {
                r
            } else if let Some(r) = t.strip_prefix("context_capturing_export!(") {
                r
            } else if let Some(r) = t.strip_prefix("win64_varargs_thunk!(") {
                r
            } else {
                continue;
            };
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty()
                && !name.starts_with("dispatch_")
                && !name.starts_with("aw2_")
                && seen.insert(name.clone())
            {
                let impl_kind = file_kinds.get(&name).copied().unwrap_or(ImplKind::Real);
                out.push(Win32ApiEntry {
                    dll: dll_name.to_string(),
                    symbol: name.clone(),
                    ordinal: None,
                    category: cat,
                    impl_kind,
                    rust_path: format!("{mod_path}::{name}"),
                    source_file: rel_file.clone(),
                });
            }
        }
        for (prefix, default_kind) in [
            ("failing!(", ImplKind::Failing),
            ("win64_stubs!(", ImplKind::Stub),
        ] {
            let mut rem = src;
            while let Some(pos) = rem.find(prefix) {
                let after = &rem[pos + prefix.len()..];
                if let Some(end) = after.find(");") {
                    let block = &after[..end];
                    let list_part = block.split_once(';').map(|(_, rhs)| rhs).unwrap_or(block);
                    for raw_sym in list_part.split(',') {
                        let name: String = raw_sym
                            .trim()
                            .chars()
                            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                            .collect();
                        if !name.is_empty()
                            && !name.starts_with("dispatch_")
                            && !name.starts_with("aw2_")
                            && seen.insert(name.clone())
                        {
                            out.push(Win32ApiEntry {
                                dll: dll_name.to_string(),
                                symbol: name.clone(),
                                ordinal: None,
                                category: cat,
                                impl_kind: default_kind,
                                rust_path: format!("{mod_path}::{name}"),
                                source_file: rel_file.clone(),
                            });
                        }
                    }
                    rem = &after[end + 2..];
                } else {
                    break;
                }
            }
        }
    }
}

fn scan_exports_macro_file(
    workspace_root: &Path,
    src_root: &Path,
    dll_name: &str,
    file: &Path,
    out: &mut Vec<Win32ApiEntry>,
) {
    let Ok(raw) = fs::read_to_string(file) else {
        return;
    };
    let src = raw.split("#[cfg(test)]").next().unwrap_or("");
    let file_kinds = analyze_file_impl_kinds(src);
    let mod_path = rust_module_path(src_root, file);
    let rel_file = file
        .strip_prefix(workspace_root)
        .unwrap_or(file)
        .to_string_lossy()
        .into_owned();
    let cat = Win32Category::for_dll(dll_name);
    let ordinals = parse_ordinals(src);

    if let Some((_, after)) = src.split_once("exports![") {
        if let Some((body, _)) = after.split_once(']') {
            for item in body.split([',', ';']) {
                let item = item.trim();
                if item.is_empty() {
                    continue;
                }
                let (sym, rust_path, impl_kind) = if let Some((lhs, rhs)) = item.split_once("=>") {
                    let sym = lhs.trim().trim_matches('"').to_string();
                    let target = rhs.trim().replacen("crate::", "win32_rt::", 1);
                    let local_ident = target.rsplit("::").next().unwrap_or("");
                    let kind = if target.starts_with(&mod_path) {
                        file_kinds.get(local_ident).copied().unwrap_or(ImplKind::Real)
                    } else {
                        // Cross-module forwarders are resolved in pass 4 of `from_workspace_src`.
                        ImplKind::Real
                    };
                    (sym, target, kind)
                } else {
                    let sym = item.to_string();
                    let target = format!("{mod_path}::{sym}");
                    let kind = file_kinds.get(&sym).copied().unwrap_or(ImplKind::Real);
                    (sym, target, kind)
                };
                if sym.is_empty() {
                    continue;
                }
                let ord = ordinals
                    .iter()
                    .find(|(_, n)| *n == sym)
                    .map(|(o, _)| *o);
                out.push(Win32ApiEntry {
                    dll: dll_name.to_string(),
                    symbol: sym,
                    ordinal: ord,
                    category: cat,
                    impl_kind,
                    rust_path,
                    source_file: rel_file.clone(),
                });
            }
        }
    }
}

fn scan_extra_exports_and_ordinals(
    workspace_root: &Path,
    src_root: &Path,
    dll_name: &str,
    file: &Path,
    out: &mut Vec<Win32ApiEntry>,
) {
    let Ok(raw) = fs::read_to_string(file) else {
        return;
    };
    let src = raw.split("#[cfg(test)]").next().unwrap_or("");
    let file_kinds = analyze_file_impl_kinds(src);
    let mod_path = rust_module_path(src_root, file);
    let rel_file = file
        .strip_prefix(workspace_root)
        .unwrap_or(file)
        .to_string_lossy()
        .into_owned();
    let cat = Win32Category::for_dll(dll_name);
    let ordinals = parse_ordinals(src);

    // Attach ordinals to already-scanned exports or insert ordinal aliases.
    for (ord, sym) in &ordinals {
        if let Some(existing) = out
            .iter_mut()
            .find(|e| e.dll == dll_name && e.symbol == *sym && e.ordinal.is_none())
        {
            existing.ordinal = Some(*ord);
        } else {
            let impl_kind = file_kinds.get(sym).copied().unwrap_or(ImplKind::Real);
            out.push(Win32ApiEntry {
                dll: dll_name.to_string(),
                symbol: sym.clone(),
                ordinal: Some(*ord),
                category: cat,
                impl_kind,
                rust_path: format!("{mod_path}::{sym}"),
                source_file: rel_file.clone(),
            });
        }
    }

    // Parse `Export { name: "...", ptr: ... }` inside `pub static EXTRA_EXPORTS: &[...] = &[ ... ];`.
    // Anchor on the `static EXTRA_EXPORTS` declaration and split after `=` so comments or
    // earlier code referencing `EXTRA_EXPORTS` never shadow the table.
    let extra_after = src
        .split_once("pub static EXTRA_EXPORTS")
        .or_else(|| src.split_once("static EXTRA_EXPORTS"))
        .map(|(_, after)| after);
    if let Some(after) = extra_after {
        let init = after.split_once('=').map(|(_, rhs)| rhs).unwrap_or(after);
        let body = init
            .split_once("&[")
            .and_then(|(_, b)| b.split_once("];"))
            .map(|(b, _)| b)
            .unwrap_or(init);
        for part in body.split("name:") {
            let trimmed = part.trim_start();
            if let Some(rest) = trimmed.strip_prefix('"') {
                if let Some((sym, after_sym)) = rest.split_once('"') {
                    if !sym.is_empty()
                        && !out.iter().any(|e| e.dll == dll_name && e.symbol == sym)
                    {
                        let impl_kind = after_sym
                            .split_once("ptr:")
                            .and_then(|(_, ptr_tail)| extract_ptr_target_ident(ptr_tail))
                            .and_then(|ident| file_kinds.get(ident).copied())
                            .unwrap_or(ImplKind::Real);
                        out.push(Win32ApiEntry {
                            dll: dll_name.to_string(),
                            symbol: sym.to_string(),
                            ordinal: None,
                            category: cat,
                            impl_kind,
                            rust_path: format!("{mod_path}::EXTRA_EXPORTS[{sym}]"),
                            source_file: rel_file.clone(),
                        });
                    }
                }
            }
        }
    }
}

fn parse_ordinals(src: &str) -> Vec<(u16, String)> {
    let mut out = Vec::new();
    let Some((_, after)) = src
        .split_once("pub static ORDINALS")
        .or_else(|| src.split_once("static ORDINALS"))
    else {
        return out;
    };
    let init = after.split_once('=').map(|(_, rhs)| rhs).unwrap_or(after);
    let Some((_, list_body)) = init.split_once("&[") else {
        return out;
    };
    let Some((body, _)) = list_body.split_once("];") else {
        return out;
    };
    for chunk in body.split('(').skip(1) {
        let Some((inside, _)) = chunk.split_once(')') else {
            continue;
        };
        let Some((num_s, name_s)) = inside.split_once(',') else {
            continue;
        };
        let Ok(ord) = num_s.trim().parse::<u16>() else {
            continue;
        };
        let name = name_s.trim().trim_matches('"').to_string();
        if !name.is_empty() {
            out.push((ord, name));
        }
    }
    out
}

#[cfg(test)]
pub(crate) mod test_pe_fixture {
    use pe_image::consts::{
        IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_FILE_MACHINE_AMD64, IMAGE_SCN_CNT_CODE,
        IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE,
    };
    const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 0x0000_0040;

    /// Build a minimal valid x86-64 PE32+ image with custom `.text` machine code bytes
    /// and a genuine `IMAGE_DIRECTORY_ENTRY_IMPORT` table for the given `(dll, &[symbol])` list.
    pub(crate) fn build_synthetic_pe(
        is_dll: bool,
        code: &[u8],
        imports: &[(&str, &[&str])],
        rwx_text: bool,
    ) -> Vec<u8> {
        let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
        let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
        let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

        // Layout:
        //   0x000..0x200 : DOS + PE + Optional + 2 section headers (".text", ".rdata")
        //   0x200..      : .text  (RVA 0x1000, raw size rounded to 0x200)
        //   after .text  : .rdata (RVA 0x1000 + text_vsize_aligned, Import Directory + INT/IAT + strings)
        let text_raw_size = code.len().max(1).div_ceil(0x200) * 0x200;
        let text_rva: u32 = 0x1000;
        let text_raw_off: usize = 0x200;

        let rdata_rva: u32 = text_rva + ((text_raw_size as u32).div_ceil(0x1000) * 0x1000);
        let rdata_raw_off: usize = text_raw_off + text_raw_size;

        // Build `.rdata` containing the Import Directory table.
        let mut rdata = Vec::<u8>::new();
        let import_dir_size: u32;
        if !imports.is_empty() {
            // (imports.len() + 1) IMAGE_IMPORT_DESCRIPTOR entries, 20 bytes each.
            let desc_bytes = (imports.len() + 1) * 20;
            rdata.resize(desc_bytes, 0);
            import_dir_size = desc_bytes as u32;

            for (idx, (dll, syms)) in imports.iter().enumerate() {
                // Align to 8 bytes for INT/IAT thunk arrays.
                while rdata.len() % 8 != 0 {
                    rdata.push(0);
                }
                let int_rva = rdata_rva + rdata.len() as u32;
                let thunk_slots = syms.len() + 1;
                let int_off = rdata.len();
                rdata.resize(int_off + thunk_slots * 8, 0);

                let iat_rva = rdata_rva + rdata.len() as u32;
                let iat_off = rdata.len();
                rdata.resize(iat_off + thunk_slots * 8, 0);

                let name_rva = rdata_rva + rdata.len() as u32;
                rdata.extend_from_slice(dll.as_bytes());
                rdata.push(0);

                for (s_idx, sym) in syms.iter().enumerate() {
                    if rdata.len() % 2 != 0 {
                        rdata.push(0);
                    }
                    let ibn_rva = rdata_rva + rdata.len() as u32;
                    rdata.extend_from_slice(&0u16.to_le_bytes()); // Hint
                    rdata.extend_from_slice(sym.as_bytes());
                    rdata.push(0);

                    w64(&mut rdata, int_off + s_idx * 8, ibn_rva as u64);
                    w64(&mut rdata, iat_off + s_idx * 8, ibn_rva as u64);
                }

                let d = idx * 20;
                w32(&mut rdata, d, int_rva); // OriginalFirstThunk
                w32(&mut rdata, d + 12, name_rva); // Name
                w32(&mut rdata, d + 16, iat_rva); // FirstThunk
            }
        } else {
            import_dir_size = 0;
        }

        let rdata_raw_size = rdata.len().max(0x200).div_ceil(0x200) * 0x200;
        let rdata_vsize_aligned = (rdata_raw_size as u32).div_ceil(0x1000) * 0x1000;
        let size_of_image = rdata_rva + rdata_vsize_aligned;
        let total_file_size = rdata_raw_off + rdata_raw_size;

        let mut img = vec![0u8; total_file_size];
        img[0..2].copy_from_slice(b"MZ");
        let pe = 0x80usize;
        w32(&mut img, 0x3C, pe as u32);
        w32(&mut img, pe, 0x0000_4550); // "PE\0\0"
        w16(&mut img, pe + 4, IMAGE_FILE_MACHINE_AMD64);
        w16(&mut img, pe + 6, 2); // 2 sections (.text, .rdata)
        w16(&mut img, pe + 20, 240); // SizeOfOptionalHeader (PE32+)
        let chars = if is_dll { 0x2022 } else { 0x0022 };
        w16(&mut img, pe + 22, chars);

        let opt = pe + 24;
        w16(&mut img, opt, 0x20B); // PE32+
        w32(&mut img, opt + 16, text_rva); // AddressOfEntryPoint
        w64(&mut img, opt + 24, 0x1_4000_0000); // ImageBase
        w32(&mut img, opt + 32, 0x1000); // SectionAlignment
        w32(&mut img, opt + 36, 0x200); // FileAlignment
        w32(&mut img, opt + 56, size_of_image);
        w32(&mut img, opt + 60, 0x200); // SizeOfHeaders
        w16(&mut img, opt + 68, 2); // Subsystem = GUI
        w32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

        if import_dir_size > 0 {
            let imp_dd = opt + 112 + IMAGE_DIRECTORY_ENTRY_IMPORT * 8;
            w32(&mut img, imp_dd, rdata_rva);
            w32(&mut img, imp_dd + 4, import_dir_size);
        }

        // Section 0: .text
        let sec0 = opt + 240;
        img[sec0..sec0 + 5].copy_from_slice(b".text");
        w32(&mut img, sec0 + 8, code.len().max(1) as u32);
        w32(&mut img, sec0 + 12, text_rva);
        w32(&mut img, sec0 + 16, text_raw_size as u32);
        w32(&mut img, sec0 + 20, text_raw_off as u32);
        let mut text_chars = IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ;
        if rwx_text {
            text_chars |= IMAGE_SCN_MEM_WRITE;
        }
        w32(&mut img, sec0 + 36, text_chars);

        // Section 1: .rdata
        let sec1 = sec0 + 40;
        img[sec1..sec1 + 6].copy_from_slice(b".rdata");
        w32(&mut img, sec1 + 8, rdata.len().max(1) as u32);
        w32(&mut img, sec1 + 12, rdata_rva);
        w32(&mut img, sec1 + 16, rdata_raw_size as u32);
        w32(&mut img, sec1 + 20, rdata_raw_off as u32);
        w32(
            &mut img,
            sec1 + 36,
            IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ,
        );

        img[text_raw_off..text_raw_off + code.len()].copy_from_slice(code);
        img[rdata_raw_off..rdata_raw_off + rdata.len()].copy_from_slice(&rdata);
        img
    }

    /// Populate a minimal synthetic `crates/win32-rt/src/` tree under `ws_root` with
    /// deterministic `real`, `partial`, `stub`, and `failing` exports so tests never depend
    /// on which live repo exports happen to be stubs on a given commit.
    pub(crate) fn build_synthetic_win32_workspace(ws_root: &std::path::Path) {
        let src = ws_root.join("crates/win32-rt/src");
        for sub in ["kernel32", "d3d12", "d3d11", "misc"] {
            std::fs::create_dir_all(src.join(sub)).unwrap();
        }
        std::fs::write(
            src.join("kernel32/file.rs"),
            r#"
pub unsafe extern "win64" fn CreateFileW() -> usize { 1 }
pub unsafe extern "win64" fn CloseHandle() -> i32 { 1 }
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("ntdll.rs"),
            r#"
pub unsafe extern "win64" fn NtQueryInformationProcess(class: u32) -> i32 {
    match class {
        0 => 0,
        _ => {
            stub!("NtQueryInformationProcess({class})");
            -1
        }
    }
}
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("advapi32.rs"),
            r#"
pub unsafe extern "win64" fn CreateServiceW() -> usize { 1 }

// @exports SynthFailApi
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    SynthFailApi);
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("d3d12/mod.rs"),
            r#"
pub static EXPORTS: &[crate::Export] = exports![
    D3D12CreateDevice,
    SynthD3D12Stub,
];
pub unsafe extern "win64" fn D3D12CreateDevice() -> i32 { 0 }
pub unsafe extern "win64" fn SynthD3D12Stub() -> i32 {
    stub!("SynthD3D12Stub -> E_NOTIMPL");
    E_NOTIMPL
}
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("d3d11/mod.rs"),
            r#"
pub static EXPORTS: &[crate::Export] = exports![D3D11CreateDevice];
pub unsafe extern "win64" fn D3D11CreateDevice() -> i32 { 0 }
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("slstub.rs"),
            r#"
pub static EXPORTS: &[crate::Export] = exports![
    slInit;
    "SynthD3D12Stub" => crate::d3d12::SynthD3D12Stub,
];
pub unsafe extern "win64" fn slInit() -> i32 { 0 }
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("misc/zstd.rs"),
            r#"
pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];
pub extern "win64" fn SynthZstdStub() -> usize {
    crate::stub!("zstd!SynthZstdStub");
    usize::MAX
}
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("misc/physx3_x64.rs"),
            r#"
pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];
pub extern "win64" fn PxCreateBasePhysics() -> usize { 1 }
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("misc/bcrypt.rs"),
            r#"
pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];
pub unsafe extern "win64" fn SynthEarlyReturnPartial(tag: u32) -> i32 {
    if tag == 0 {
        return STATUS_INVALID_PARAMETER;
    }
    if tag == 1 {
        return STATUS_SUCCESS;
    }
    stub!("SynthEarlyReturnPartial fallback");
    STATUS_NOT_SUPPORTED
}
"#,
        )
        .unwrap();
        std::fs::write(
            src.join("misc/libxess_dx11.rs"),
            r#"
pub static EXTRA_EXPORTS: &[crate::Export] = &[
    crate::Export {
        name: "SynthBridgeStub",
        ptr: crate::Ptr(crate::bridge_entry!(synth_bridge_stub_impl)),
    },
];
pub static ORDINALS: &[(u16, &str)] = &[];
extern "win64" fn synth_bridge_stub_impl() -> i32 {
    crate::stub!("synth_bridge_stub_impl -> NOT_IMPLEMENTED");
    -1
}
"#,
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_dll_matches_port_manifest_truth() {
        assert_eq!(canonical_dll("libxess.dll"), "libxess.dll");
        assert_eq!(canonical_dll("openal32.dll"), "openal32.dll");
        assert_eq!(canonical_dll("tbb12.dll"), "tbbmalloc.dll");
        assert_eq!(canonical_dll("libzstd.dll"), "zstd.dll");
        assert_eq!(canonical_dll("lz4.dll"), "lz4.dll");
        assert_eq!(canonical_dll("sspicli.dll"), "secur32.dll");
    }

    #[test]
    fn format_audit_report_includes_detected_middleware_and_impl_kind_breakdown() {
        let mut resolved_by_category = BTreeMap::new();
        resolved_by_category.insert(Win32Category::CoreOs, 12);
        let mut resolved_by_impl_kind = BTreeMap::new();
        resolved_by_impl_kind.insert(ImplKind::Real, 9);
        resolved_by_impl_kind.insert(ImplKind::Partial, 1);
        resolved_by_impl_kind.insert(ImplKind::Stub, 1);
        resolved_by_impl_kind.insert(ImplKind::Failing, 1);
        let audit = Win32ImportAudit {
            binary_name: "Sample.exe".to_string(),
            total_imports: 14,
            resolved_win32: 12,
            resolved_sibling: 0,
            resolved_by_impl_kind,
            resolved_by_category,
            stubbed_imports: vec![
                NonRealImport {
                    dll: "zstd.dll".to_string(),
                    symbol: "ZSTD_decompress".to_string(),
                    impl_kind: ImplKind::Stub,
                    source_file: "crates/win32-rt/src/misc/zstd.rs".to_string(),
                },
                NonRealImport {
                    dll: "advapi32.dll".to_string(),
                    symbol: "CryptAcquireContextW".to_string(),
                    impl_kind: ImplKind::Failing,
                    source_file: "crates/win32-rt/src/advapi32.rs".to_string(),
                },
            ],
            partial_imports: vec![NonRealImport {
                dll: "ntdll.dll".to_string(),
                symbol: "NtQueryInformationProcess".to_string(),
                impl_kind: ImplKind::Partial,
                source_file: "crates/win32-rt/src/ntdll.rs".to_string(),
            }],
            unresolved: vec![("custom.dll".to_string(), "MissingFn".to_string())],
            detected_middleware: detect_middleware_from_dlls([
                "PhysX3_x64.dll",
                "PxFoundation_x64.dll",
                "nvngx_dlss.dll",
            ]),
        };

        let report = format_audit_report(&audit);
        assert!(report.contains(
            "=== Win32 Import Audit: Sample.exe (12/14 resolved = 85.71%, 9/14 real = 64.29%) ==="
        ));
        assert!(report.contains("Implementation Breakdown:"));
        assert!(report.contains("CORE_OS"));
        assert!(report.contains("Detected Middleware (2):"));
        assert!(report.contains("NVIDIA PhysX (physx) v3.4 [default: guest] <- physx3_x64.dll, pxfoundation_x64.dll"));
        assert!(report.contains("NVIDIA DLSS / Streamline (dlss) v2 [default: builtin] <- nvngx_dlss.dll"));
        assert!(report.contains("Stubbed / Failing Imports (2):"));
        assert!(report.contains("zstd.dll!ZSTD_decompress [stub]"));
        assert!(report.contains("advapi32.dll!CryptAcquireContextW [failing]"));
        assert!(report.contains("Partial Imports (1):"));
        assert!(report.contains("ntdll.dll!NtQueryInformationProcess [partial]"));
        assert!(report.contains("Unresolved (1):"));
        assert!(report.contains("custom.dll!MissingFn"));
    }

    #[test]
    fn classify_source_impl_kinds_distinguishes_all_four_categories() {
        let src = r#"
pub unsafe extern "win64" fn RealFn(x: u32) -> u32 {
    // A comment mentioning stub!("not a real call") must be ignored.
    let s = "stub!(also ignored in string)";
    let _ = s;
    x + 1
}

pub unsafe extern "win64" fn PartialBranchFn(class: u32) -> i32 {
    match class {
        0 => 0,
        _ => {
            crate::stub!("PartialBranchFn({class})");
            -1
        }
    }
}

pub unsafe extern "win64" fn PartialEarlyReturnFn(tag: u32) -> i32 {
    if tag == 0 {
        return STATUS_INVALID_PARAMETER;
    }
    if tag == 1 {
        return STATUS_SUCCESS;
    }
    stub!("PartialEarlyReturnFn fallback");
    STATUS_NOT_SUPPORTED
}

pub unsafe extern "win64" fn PureStubFn(ptr: *mut u8) -> i32 {
    if ptr.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    crate::stub!("PureStubFn -> NOT_IMPLEMENTED");
    XESS_RESULT_ERROR_NOT_IMPLEMENTED
}

pub unsafe extern "win64" fn NullGuardedStubFn(ptr: *mut u8) {
    if !ptr.is_null() {
        stub!("NullGuardedStubFn");
    }
}

// @exports FailMacroFnA FailMacroFnB
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    FailMacroFnA, FailMacroFnB);

win64_stubs!(Win64StubMacroFn);
"#;
        let kinds = analyze_file_impl_kinds(src);
        assert_eq!(kinds.get("RealFn"), Some(&ImplKind::Real));
        assert_eq!(kinds.get("PartialBranchFn"), Some(&ImplKind::Partial));
        assert_eq!(kinds.get("PartialEarlyReturnFn"), Some(&ImplKind::Partial));
        assert_eq!(kinds.get("PureStubFn"), Some(&ImplKind::Stub));
        assert_eq!(kinds.get("NullGuardedStubFn"), Some(&ImplKind::Stub));
        assert_eq!(kinds.get("FailMacroFnA"), Some(&ImplKind::Failing));
        assert_eq!(kinds.get("FailMacroFnB"), Some(&ImplKind::Failing));
        assert_eq!(kinds.get("Win64StubMacroFn"), Some(&ImplKind::Stub));
    }

    #[test]
    fn synthetic_workspace_win32_db_classifies_real_partial_stub_and_failing_exports() {
        let fake_ws = std::env::temp_dir().join(format!(
            "port_factory_win32_db_synth_ws_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&fake_ws);
        test_pe_fixture::build_synthetic_win32_workspace(&fake_ws);

        let db = Win32Database::from_workspace_src(&fake_ws).unwrap();
        let _ = fs::remove_dir_all(&fake_ws);

        let lookup = |dll: &str, sym: &str| {
            db.resolve(
                dll,
                &ImportName::Name {
                    hint: 0,
                    name: sym.to_string(),
                },
            )
            .unwrap_or_else(|| panic!("expected {dll}!{sym} in synthetic Win32Database"))
            .impl_kind
        };

        assert_eq!(lookup("kernel32.dll", "CreateFileW"), ImplKind::Real);
        assert_eq!(lookup("d3d12.dll", "D3D12CreateDevice"), ImplKind::Real);
        assert_eq!(
            lookup("ntdll.dll", "NtQueryInformationProcess"),
            ImplKind::Partial
        );
        assert_eq!(
            lookup("bcrypt.dll", "SynthEarlyReturnPartial"),
            ImplKind::Partial
        );
        assert_eq!(lookup("zstd.dll", "SynthZstdStub"), ImplKind::Stub);
        assert_eq!(lookup("d3d12.dll", "SynthD3D12Stub"), ImplKind::Stub);
        // Cross-module forwarder in sl.interposer.dll inherits Stub from d3d12/mod.rs.
        assert_eq!(
            lookup("sl.interposer.dll", "SynthD3D12Stub"),
            ImplKind::Stub
        );
        // EXTRA_EXPORTS + bridge_entry! inherits Stub from target impl fn.
        assert_eq!(
            lookup("libxess_dx11.dll", "SynthBridgeStub"),
            ImplKind::Stub
        );
        // `// @exports` + `failing!(...)` is classified as Failing.
        assert_eq!(lookup("advapi32.dll", "SynthFailApi"), ImplKind::Failing);

        assert_eq!(db.count_by_impl_kind(ImplKind::Real), 7);
        assert_eq!(db.count_by_impl_kind(ImplKind::Partial), 2);
        assert_eq!(db.count_by_impl_kind(ImplKind::Stub), 4);
        assert_eq!(db.count_by_impl_kind(ImplKind::Failing), 1);

        // Verify TSV header & row column count include `impl_kind`.
        let tsv = db.format_tsv();
        let mut lines = tsv.lines();
        assert_eq!(
            lines.next().unwrap(),
            "dll\tsymbol\tordinal\tcategory\timpl_kind\trust_path\tsource_file"
        );
        for row in lines {
            let cols: Vec<&str> = row.split('\t').collect();
            assert_eq!(cols.len(), 7, "expected 7 columns in row: {row}");
            assert!(
                ImplKind::parse(cols[4]).is_some(),
                "invalid impl_kind column {:?} in row: {row}",
                cols[4]
            );
        }
    }

    #[test]
    fn live_workspace_win32_db_invariants_hold_independent_of_stub_counts() {
        let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .unwrap()
            .to_path_buf();
        let db = Win32Database::from_workspace_src(&ws).unwrap();

        assert!(db.entries.len() > 3000);
        let sum = db.count_by_impl_kind(ImplKind::Real)
            + db.count_by_impl_kind(ImplKind::Partial)
            + db.count_by_impl_kind(ImplKind::Stub)
            + db.count_by_impl_kind(ImplKind::Failing);
        assert_eq!(sum, db.entries.len());
        assert_eq!(
            db.resolve(
                "kernel32.dll",
                &ImportName::Name {
                    hint: 0,
                    name: "CreateFileW".to_string(),
                }
            )
            .unwrap()
            .impl_kind,
            ImplKind::Real
        );

        let tsv = db.format_tsv();
        let mut lines = tsv.lines();
        assert_eq!(
            lines.next().unwrap(),
            "dll\tsymbol\tordinal\tcategory\timpl_kind\trust_path\tsource_file"
        );
        for row in lines {
            let cols: Vec<&str> = row.split('\t').collect();
            assert_eq!(cols.len(), 7, "expected 7 columns in row: {row}");
            assert!(ImplKind::parse(cols[4]).is_some());
        }
    }

    #[test]
    fn synthetic_pe_win32_audit_reports_impl_kinds_and_lists_stubs() {
        let fake_ws = std::env::temp_dir().join(format!(
            "port_factory_win32_audit_synth_ws_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&fake_ws);
        test_pe_fixture::build_synthetic_win32_workspace(&fake_ws);
        let db = Win32Database::from_workspace_src(&fake_ws).unwrap();
        let _ = fs::remove_dir_all(&fake_ws);

        // Synthetic PE importing against our controlled synthetic Win32Database:
        // - 2 real (`kernel32.dll!CreateFileW`, `d3d12.dll!D3D12CreateDevice`)
        // - 1 partial (`ntdll.dll!NtQueryInformationProcess`)
        // - 1 stub (`zstd.dll!SynthZstdStub`)
        // - 1 failing (`advapi32.dll!SynthFailApi`)
        // - 1 unresolved (`custom_engine.dll!MysteryInit`)
        let pe_bytes = test_pe_fixture::build_synthetic_pe(
            false,
            &[0x48, 0x31, 0xC0, 0xC3], // xor rax, rax; ret
            &[
                ("KERNEL32.dll", &["CreateFileW"]),
                ("d3d12.dll", &["D3D12CreateDevice"]),
                ("ntdll.dll", &["NtQueryInformationProcess"]),
                ("zstd.dll", &["SynthZstdStub"]),
                ("ADVAPI32.dll", &["SynthFailApi"]),
                ("custom_engine.dll", &["MysteryInit"]),
            ],
            false,
        );

        let audit = audit_pe_imports("SyntheticGame.exe", &pe_bytes, &db, None).unwrap();
        assert_eq!(audit.total_imports, 6);
        assert_eq!(audit.resolved_win32, 5);
        assert_eq!(audit.count_by_impl_kind(ImplKind::Real), 2);
        assert_eq!(audit.count_by_impl_kind(ImplKind::Partial), 1);
        assert_eq!(audit.count_by_impl_kind(ImplKind::Stub), 1);
        assert_eq!(audit.count_by_impl_kind(ImplKind::Failing), 1);
        assert_eq!(audit.unresolved.len(), 1);
        assert_eq!(audit.stubbed_imports.len(), 2);
        assert_eq!(audit.partial_imports.len(), 1);

        let report = audit.format_report();
        assert!(report.contains("5/6 resolved = 83.33%, 2/6 real = 33.33%"));
        assert!(report.contains("Stubbed / Failing Imports (2):"));
        assert!(report.contains("zstd.dll!SynthZstdStub [stub]"));
        assert!(report.contains("advapi32.dll!SynthFailApi [failing]"));
        assert!(report.contains("Partial Imports (1):"));
        assert!(report.contains("ntdll.dll!NtQueryInformationProcess [partial]"));
        assert!(report.contains("custom_engine.dll!MysteryInit"));
    }

    #[test]
    fn extra_exports_scanner_finds_tables_after_earlier_references() {
        let ws = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .unwrap()
            .to_path_buf();
        let db = Win32Database::from_workspace_src(&ws).unwrap();
        let pb_count = db.entries.iter().filter(|e| e.dll == "libprotobuf.dll").count();
        assert!(
            pb_count >= 122,
            "expected >= 122 libprotobuf.dll EXTRA_EXPORTS, found {pb_count}"
        );
        assert!(db
            .entries
            .iter()
            .any(|e| e.dll == "libxess.dll" && e.symbol == "xessSetJitterScale"));
        assert!(db
            .entries
            .iter()
            .any(|e| e.dll == "d2d1.dll" && e.symbol == "D2D1MakeRotateMatrix"));
    }
}
