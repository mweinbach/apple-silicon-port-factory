//! Wine-style Core Windows API Mapping Database (`db/win32_apis.tsv`).
//!
//! Catalogs every Windows DLL and exported API implemented natively in
//! `crates/win32-rt`, performs Windows API-set normalization
//! (`api-ms-win-crt-*` -> `ucrtbase.dll`, `api-ms-win-core-*` -> `kernel32.dll`,
//! `kernel32 <-> ntdll` forwarders), and audits any target `.exe` or `.dll`
//! for static AOT linking and Win32 API coverage.

use anyhow::{Context, Result};
use pe_image::{ImportName, Pe};
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

/// One exported Windows API entry in `db/win32_apis.tsv`.
#[derive(Debug, Clone)]
pub struct Win32ApiEntry {
    pub dll: String,
    pub symbol: String,
    pub ordinal: Option<u16>,
    pub category: Win32Category,
    pub rust_path: String,
    pub source_file: String,
}

/// Wine-style DLL & Windows API-set canonicalizer.
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
    if n == "_nvngx.dll"
        || n == "nvngx_dlssg.dll"
        || n == "nvngx_dlssd.dll"
        || n == "libxess.dll"
        || n == "libxess_dx12.dll"
    {
        return "nvngx_dlss.dll".into();
    }
    if n == "fmod64.dll"
        || n == "fmodstudio.dll"
        || n == "fmodstudio64.dll"
        || n == "openal32.dll"
    {
        return "fmod.dll".into();
    }
    if (n.starts_with("oo2core_") && n.ends_with(".dll"))
        || n == "tbb12.dll"
        || n == "tbb.dll"
        || n == "tbbmalloc.dll"
        || n == "zstd.dll"
        || n == "libzstd.dll"
        || n == "lz4.dll"
    {
        return "oo2core_9_win64.dll".into();
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
    n
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

        Ok(Self::from_entries(entries))
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
        let mut out = String::from("dll\tsymbol\tordinal\tcategory\trust_path\tsource_file\n");
        for e in &self.entries {
            let ord_str = e
                .ordinal
                .map(|o| o.to_string())
                .unwrap_or_else(|| "-".to_string());
            let _ = writeln!(
                out,
                "{}\t{}\t{}\t{}\t{}\t{}",
                e.dll,
                e.symbol,
                ord_str,
                e.category.as_str(),
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

/// Summary of auditing a PE binary's imports against [`Win32Database`].
#[derive(Debug, Clone)]
pub struct Win32ImportAudit {
    pub binary_name: String,
    pub total_imports: usize,
    pub resolved_win32: usize,
    pub resolved_by_category: BTreeMap<Win32Category, usize>,
    pub unresolved: Vec<(String, String)>,
}

impl Win32ImportAudit {
    pub fn coverage_pct(&self) -> f64 {
        if self.total_imports == 0 {
            100.0
        } else {
            (self.resolved_win32 as f64 * 100.0) / (self.total_imports as f64)
        }
    }
}

/// Audit all regular and delay-load imports of a PE image against [`Win32Database`].
pub fn audit_pe_imports(
    binary_name: &str,
    data: &[u8],
    db: &Win32Database,
    sibling_exports: Option<&HashMap<(String, String), u32>>,
) -> Result<Win32ImportAudit> {
    let pe = Pe::parse(data).with_context(|| format!("parse PE {binary_name}"))?;
    let mut audit = Win32ImportAudit {
        binary_name: binary_name.to_string(),
        total_imports: 0,
        resolved_win32: 0,
        resolved_by_category: BTreeMap::new(),
        unresolved: Vec::new(),
    };

    let mut all_dlls = pe.imports().unwrap_or_default();
    if let Ok(delay) = pe.delay_imports() {
        all_dlls.extend(delay);
    }

    for imp in all_dlls {
        let norm_dll = pe_image::normalize_dll_name(&imp.dll);
        for ent in &imp.entries {
            audit.total_imports += 1;
            if let Some(hit) = db.resolve(&norm_dll, &ent.name) {
                audit.resolved_win32 += 1;
                *audit.resolved_by_category.entry(hit.category).or_insert(0) += 1;
                continue;
            }
            if let Some(sib) = sibling_exports {
                let sym_str = ent.name.to_string();
                if sib.contains_key(&(norm_dll.clone(), sym_str)) {
                    audit.resolved_win32 += 1;
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
    Ok(audit)
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
                        out.push(Win32ApiEntry {
                            dll: dll_name.to_string(),
                            symbol: name.to_string(),
                            ordinal: None,
                            category: cat,
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
                out.push(Win32ApiEntry {
                    dll: dll_name.to_string(),
                    symbol: name.clone(),
                    ordinal: None,
                    category: cat,
                    rust_path: format!("{mod_path}::{name}"),
                    source_file: rel_file.clone(),
                });
            }
        }
        for prefix in ["failing!(", "win64_stubs!("] {
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
                let (sym, rust_path) = if let Some((lhs, rhs)) = item.split_once("=>") {
                    let sym = lhs.trim().trim_matches('"').to_string();
                    let target = rhs.trim().replacen("crate::", "win32_rt::", 1);
                    (sym, target)
                } else {
                    let sym = item.to_string();
                    let target = format!("{mod_path}::{sym}");
                    (sym, target)
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
            out.push(Win32ApiEntry {
                dll: dll_name.to_string(),
                symbol: sym.clone(),
                ordinal: Some(*ord),
                category: cat,
                rust_path: format!("{mod_path}::{sym}"),
                source_file: rel_file.clone(),
            });
        }
    }

    // Parse `Export { name: "...", ptr: ... }` inside `EXTRA_EXPORTS`.
    if let Some((_, after)) = src.split_once("EXTRA_EXPORTS") {
        let body = after
            .split_once("&[")
            .and_then(|(_, b)| b.split_once("];"))
            .map(|(b, _)| b)
            .unwrap_or(after);
        for part in body.split("name:") {
            let trimmed = part.trim_start();
            if let Some(rest) = trimmed.strip_prefix('"') {
                if let Some((sym, _)) = rest.split_once('"') {
                    if !sym.is_empty()
                        && !out.iter().any(|e| e.dll == dll_name && e.symbol == sym)
                    {
                        out.push(Win32ApiEntry {
                            dll: dll_name.to_string(),
                            symbol: sym.to_string(),
                            ordinal: None,
                            category: cat,
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
    let Some((_, after)) = src.split_once("ORDINALS") else {
        return out;
    };
    let Some((_, list_body)) = after.split_once("&[") else {
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
