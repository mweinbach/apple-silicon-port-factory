//! `.NET` / CLR / Unity Mono / `IL2CPP` Dependency & Runtime Analyzer.
//!
//! Classifies how Windows games and desktop applications use `.NET` / C# and
//! selects the optimal execution path on macOS Apple Silicon (`aarch64`):
//!
//! 1. **Pure-IL (`AnyCPU` / `x64` `ILONLY`)**:
//!    Runs directly on native macOS ARM64 `libcoreclr.dylib` / `libmonobdwgc-2.0.dylib`
//!    via our `mscoree.dll` / `hostfxr.dll` / `coreclr.dll` bridge with zero x86-64
//!    instruction emulation.
//! 2. **ReadyToRun (`R2R`) Hybrid (`CIL` + precompiled `x86-64` `ManagedNativeHeader`)**:
//!    Can either strip `ManagedNativeHeader` (`pe_image::strip_ready_to_run_header`) so
//!    native ARM64 CoreCLR JITs the portable CIL bytecode, or AOT-translate the x86-64
//!    R2R code via `port-factory`.
//! 3. **Mixed-Mode C++/CLI (`IJW` / `VTableFixups`)**:
//!    Contains both native x86-64 functions and CIL metadata; runs under our x86-64->ARM64
//!    AOT translator + `mscoree.dll` / `coreclr.dll` bridge.
//! 4. **Unity Mono (`mono-2.0-bdwgc.dll` + `<Game>_Data/Managed/Assembly-CSharp.dll`)**:
//!    Bridged via `crates/win32-rt/src/misc/mono_2_0_bdwgc.rs` (or native ARM64 Mono).
//! 5. **Unity `IL2CPP` (`GameAssembly.dll` + `il2cpp_init`) / `.NET NativeAOT`**:
//!    Already compiled from C# to native x86-64 machine code — translated 100% by our
//!    static x86-64->ARM64 AOT compiler (`crates/port-factory/src/aot`) with no CLR needed!

use anyhow::{Context, Result};
use pe_image::{inspect_clr_bytes, strip_ready_to_run_header, ClrExecutionMode, Pe};
use std::path::{Path, PathBuf};

/// High-level `.NET` / C# runtime architecture detected for a binary or game directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DotNetRuntimeKind {
    /// Native C/C++/Rust PE binary with no CLR header or `.NET` host imports.
    NativeNonDotNet,
    /// Classic `.NET Framework` (`v2.0.50727` / `v4.0.30319`) managed assembly (`mscoree.dll`).
    NetFrameworkManaged,
    /// Modern `.NET Core` / `.NET 5/6/7/8/9` managed assembly (`System.Runtime` / `BSJB`).
    ModernCoreClrManaged,
    /// Modern `.NET` native apphost / launcher (`hostfxr.dll` / `coreclr.dll` importer).
    DotNetAppHostLauncher,
    /// Unity Mono embedding runtime (`mono-2.0-bdwgc.dll` / `mono.dll` or Unity Mono host).
    UnityMonoRuntime,
    /// Unity `IL2CPP` AOT-compiled C# binary (`GameAssembly.dll` exporting `il2cpp_*`).
    UnityIl2CppNative,
}

impl DotNetRuntimeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NativeNonDotNet => "Native (Non-.NET)",
            Self::NetFrameworkManaged => ".NET Framework Managed Assembly (CLR v2/v4)",
            Self::ModernCoreClrManaged => "Modern .NET Core / .NET 5-9 Managed Assembly",
            Self::DotNetAppHostLauncher => ".NET AppHost / Native Host (hostfxr/coreclr)",
            Self::UnityMonoRuntime => "Unity Mono Embedding Runtime (mono-2.0-bdwgc)",
            Self::UnityIl2CppNative => "Unity IL2CPP Native x86-64 (GameAssembly.dll -> ARM64 AOT)",
        }
    }
}

/// Audit report for a single PE binary (`.exe` or `.dll`).
#[derive(Debug, Clone)]
pub struct DotNetBinaryReport {
    pub path: PathBuf,
    pub file_name: String,
    pub is_pe32_plus: bool,
    pub machine: u16,
    pub kind: DotNetRuntimeKind,
    pub clr_mode: Option<ClrExecutionMode>,
    pub metadata_version: Option<String>,
    pub metadata_streams: Vec<String>,
    pub clr_flags: u32,
    pub is_ready_to_run: bool,
    pub is_strong_name_signed: bool,
    pub imports_mscoree: bool,
    pub imports_hostfxr: bool,
    pub imports_coreclr: bool,
    pub imports_mono: bool,
    pub il2cpp_exports: usize,
    pub runtime_config_json: Option<PathBuf>,
    pub recommendation: String,
}

/// Summary report across one or more PE files or an entire game/application directory.
#[derive(Debug, Clone, Default)]
pub struct DotNetAuditSummary {
    pub binaries: Vec<DotNetBinaryReport>,
    pub runtime_configs: Vec<PathBuf>,
    pub global_metadata_dat: Vec<PathBuf>,
    pub pure_il_count: usize,
    pub r2r_count: usize,
    pub mixed_mode_count: usize,
    pub apphost_count: usize,
    pub unity_mono_count: usize,
    pub unity_il2cpp_count: usize,
    pub native_non_dotnet_count: usize,
    pub host_coreclr_dylib: Option<PathBuf>,
    pub host_hostfxr_dylib: Option<PathBuf>,
    pub host_mono_dylib: Option<PathBuf>,
}

/// Inspect a single PE file (`.exe` or `.dll`) for `.NET` CLR headers, ECMA-335 `BSJB`
/// metadata, ReadyToRun (`R2R`) native sections, `hostfxr`/`coreclr`/`mscoree`/`mono`
/// imports, and Unity `IL2CPP` exports.
pub fn inspect_pe_dotnet(path: &Path) -> Result<DotNetBinaryReport> {
    let data = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    inspect_pe_dotnet_bytes(path, &data)
}

/// Inspect raw PE bytes for `.NET` / CLR / Mono / `IL2CPP` characteristics.
pub fn inspect_pe_dotnet_bytes(path: &Path, data: &[u8]) -> Result<DotNetBinaryReport> {
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let lower_name = file_name.to_ascii_lowercase();

    let clr_opt = inspect_clr_bytes(data)?;

    let mut imports_mscoree = false;
    let mut imports_hostfxr = false;
    let mut imports_coreclr = false;
    let mut imports_mono = false;
    let mut il2cpp_exports = 0usize;
    let mut is_pe32_plus = clr_opt.as_ref().map_or(true, |c| c.is_pe32_plus);
    let mut machine = clr_opt
        .as_ref()
        .map_or(pe_image::consts::IMAGE_FILE_MACHINE_AMD64, |c| c.machine);

    if let Ok(pe) = Pe::parse(data) {
        is_pe32_plus = true;
        machine = pe.headers.machine;
        if let Ok(imps) = pe.imports() {
            for dll in &imps {
                let d = dll.dll.to_ascii_lowercase();
                if d.contains("mscoree") || d.contains("mscoreei") {
                    imports_mscoree = true;
                }
                if d.contains("hostfxr") {
                    imports_hostfxr = true;
                }
                if d.contains("coreclr") {
                    imports_coreclr = true;
                }
                if d.contains("mono-2.0") || d == "mono.dll" || d.contains("monobdwgc") {
                    imports_mono = true;
                }
            }
        }
        if let Ok(dels) = pe.delay_imports() {
            for dll in &dels {
                let d = dll.dll.to_ascii_lowercase();
                if d.contains("mscoree") {
                    imports_mscoree = true;
                }
                if d.contains("hostfxr") {
                    imports_hostfxr = true;
                }
                if d.contains("coreclr") {
                    imports_coreclr = true;
                }
                if d.contains("mono-2.0") || d == "mono.dll" {
                    imports_mono = true;
                }
            }
        }
        if let Ok(exps) = pe.exports() {
            for e in &exps.entries {
                if let Some(n) = &e.name {
                    if n.starts_with("il2cpp_") {
                        il2cpp_exports += 1;
                    }
                    if n.starts_with("mono_jit_") || n == "mono_domain_assembly_open" {
                        imports_mono = true;
                    }
                }
            }
        }
    }

    // Check for sibling <stem>.runtimeconfig.json
    let runtime_config_json = path.parent().and_then(|dir| {
        let stem = path.file_stem()?.to_string_lossy();
        let cand = dir.join(format!("{stem}.runtimeconfig.json"));
        cand.is_file().then_some(cand)
    });

    if let Some(clr) = clr_opt {
        let ver = clr
            .metadata
            .as_ref()
            .map(|m| m.version_string.clone())
            .unwrap_or_else(|| {
                format!(
                    "CLR {}.{}",
                    clr.header.major_runtime_version, clr.header.minor_runtime_version
                )
            });
        let streams = clr
            .metadata
            .as_ref()
            .map(|m| m.streams.clone())
            .unwrap_or_default();

        let is_modern = runtime_config_json.is_some()
            || imports_hostfxr
            || imports_coreclr
            || clr.header.is_ready_to_run()
            || !imports_mscoree;
        let kind = if is_modern {
            DotNetRuntimeKind::ModernCoreClrManaged
        } else {
            DotNetRuntimeKind::NetFrameworkManaged
        };

        let recommendation = match clr.mode {
            ClrExecutionMode::PureIlAnyCpu => {
                "Pure CIL bytecode (ILONLY): execute directly via native ARM64 CoreCLR/Mono or mscoree/hostfxr bridge".to_string()
            }
            ClrExecutionMode::ReadyToRunHybrid => {
                "ReadyToRun (CIL + x86-64 ManagedNativeHeader): strip R2R header (`dotnet-audit --strip-r2r`) for native ARM64 CoreCLR JIT or AOT-translate x86-64 R2R".to_string()
            }
            ClrExecutionMode::MixedModeX64 => {
                "Mixed-Mode C++/CLI (x86-64 + CIL): run native code through port-factory ARM64 AOT + mscoree/coreclr bridge".to_string()
            }
            ClrExecutionMode::Il32BitOnly => {
                "32-bit ILONLY assembly: clear COMIMAGE_FLAGS_32BITREQUIRED to run in 64-bit CoreCLR/Mono unless 32-bit P/Invoke DLLs are required".to_string()
            }
        };

        return Ok(DotNetBinaryReport {
            path: path.to_path_buf(),
            file_name,
            is_pe32_plus,
            machine,
            kind,
            clr_mode: Some(clr.mode),
            metadata_version: Some(ver),
            metadata_streams: streams,
            clr_flags: clr.header.flags,
            is_ready_to_run: clr.header.is_ready_to_run(),
            is_strong_name_signed: clr.header.is_strong_name_signed(),
            imports_mscoree,
            imports_hostfxr,
            imports_coreclr,
            imports_mono,
            il2cpp_exports,
            runtime_config_json,
            recommendation,
        });
    }

    let (kind, recommendation) = if il2cpp_exports > 0 || lower_name == "gameassembly.dll" {
        (
            DotNetRuntimeKind::UnityIl2CppNative,
            format!(
                "Unity IL2CPP native x86-64 binary ({il2cpp_exports} il2cpp_* exports): compile directly to ARM64 via `opsdb.sh aot-crate` (no CLR JIT needed)"
            ),
        )
    } else if imports_mono
        || lower_name.contains("mono-2.0")
        || lower_name == "mono.dll"
        || lower_name == "unityplayer.dll"
    {
        (
            DotNetRuntimeKind::UnityMonoRuntime,
            "Unity / Mono host or runtime: bridged via builtin `mono_2_0_bdwgc.dll` + native ARM64 `libmonobdwgc-2.0.dylib`".to_string(),
        )
    } else if imports_hostfxr || imports_coreclr || runtime_config_json.is_some() {
        (
            DotNetRuntimeKind::DotNetAppHostLauncher,
            "Modern .NET AppHost launcher: bridged via builtin `hostfxr.dll` / `coreclr.dll`".to_string(),
        )
    } else if imports_mscoree {
        (
            DotNetRuntimeKind::NetFrameworkManaged,
            "Native binary importing `mscoree.dll` (COM CLR host): bridged via builtin `mscoree.dll` (`CLRCreateInstance` / `CorBindToRuntimeEx`)".to_string(),
        )
    } else {
        (
            DotNetRuntimeKind::NativeNonDotNet,
            "Pure native x86-64 PE: translate directly via `port-factory` ARM64 AOT".to_string(),
        )
    };

    Ok(DotNetBinaryReport {
        path: path.to_path_buf(),
        file_name,
        is_pe32_plus,
        machine,
        kind,
        clr_mode: None,
        metadata_version: None,
        metadata_streams: Vec::new(),
        clr_flags: 0,
        is_ready_to_run: false,
        is_strong_name_signed: false,
        imports_mscoree,
        imports_hostfxr,
        imports_coreclr,
        imports_mono,
        il2cpp_exports,
        runtime_config_json,
        recommendation,
    })
}

/// Audit a list of PE files or directories (recursively scanning `.exe`, `.dll`,
/// `.runtimeconfig.json`, and `global-metadata.dat`).
pub fn audit_dotnet_paths(paths: &[PathBuf]) -> Result<DotNetAuditSummary> {
    let mut summary = DotNetAuditSummary {
        host_coreclr_dylib: find_host_dylib_coreclr(),
        host_hostfxr_dylib: find_host_dylib_hostfxr(),
        host_mono_dylib: find_host_dylib_mono(),
        ..Default::default()
    };

    let mut pe_files = Vec::new();
    for p in paths {
        collect_dotnet_artifacts(p, &mut pe_files, &mut summary.runtime_configs, &mut summary.global_metadata_dat)?;
    }
    pe_files.sort();
    pe_files.dedup();

    for pe_path in pe_files {
        let Ok(rep) = inspect_pe_dotnet(&pe_path) else {
            continue;
        };
        if let Some(mode) = rep.clr_mode {
            match mode {
                ClrExecutionMode::PureIlAnyCpu | ClrExecutionMode::Il32BitOnly => {
                    summary.pure_il_count += 1;
                }
                ClrExecutionMode::ReadyToRunHybrid => {
                    summary.r2r_count += 1;
                }
                ClrExecutionMode::MixedModeX64 => {
                    summary.mixed_mode_count += 1;
                }
            }
        } else {
            match rep.kind {
                DotNetRuntimeKind::DotNetAppHostLauncher => summary.apphost_count += 1,
                DotNetRuntimeKind::UnityMonoRuntime => summary.unity_mono_count += 1,
                DotNetRuntimeKind::UnityIl2CppNative => summary.unity_il2cpp_count += 1,
                DotNetRuntimeKind::NetFrameworkManaged | DotNetRuntimeKind::ModernCoreClrManaged => {
                    summary.mixed_mode_count += 1;
                }
                DotNetRuntimeKind::NativeNonDotNet => summary.native_non_dotnet_count += 1,
            }
        }
        summary.binaries.push(rep);
    }

    Ok(summary)
}

/// Strip the x86-64 ReadyToRun (`ManagedNativeHeader`) from a `.NET` assembly file and
/// write the pure-IL output to `output_path`.
pub fn strip_ready_to_run_file(input_path: &Path, output_path: &Path) -> Result<bool> {
    let mut bytes =
        std::fs::read(input_path).with_context(|| format!("read {}", input_path.display()))?;
    let stripped = strip_ready_to_run_header(&mut bytes)?;
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(output_path, &bytes)
        .with_context(|| format!("write {}", output_path.display()))?;
    Ok(stripped)
}

fn collect_dotnet_artifacts(
    path: &Path,
    pe_files: &mut Vec<PathBuf>,
    runtime_configs: &mut Vec<PathBuf>,
    global_metadata: &mut Vec<PathBuf>,
) -> Result<()> {
    if path.is_file() {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();
        if name.ends_with(".exe") || name.ends_with(".dll") {
            pe_files.push(path.to_path_buf());
        } else if name.ends_with(".runtimeconfig.json") {
            runtime_configs.push(path.to_path_buf());
        } else if name == "global-metadata.dat" {
            global_metadata.push(path.to_path_buf());
        }
        return Ok(());
    }
    if path.is_dir() {
        let mut entries: Vec<(PathBuf, std::fs::FileType)> = std::fs::read_dir(path)
            .with_context(|| format!("read_dir {}", path.display()))?
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_type().ok().map(|ft| (e.path(), ft)))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        for (entry, ft) in entries {
            if ft.is_symlink() {
                continue;
            }
            collect_dotnet_artifacts(&entry, pe_files, runtime_configs, global_metadata)?;
        }
    }
    Ok(())
}

fn find_host_dylib_coreclr() -> Option<PathBuf> {
    let mut roots = vec![
        PathBuf::from("/usr/local/share/dotnet/shared/Microsoft.NETCore.App"),
        PathBuf::from("/opt/homebrew/share/dotnet/shared/Microsoft.NETCore.App"),
    ];
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".dotnet/shared/Microsoft.NETCore.App"));
    }
    for root in roots {
        let Ok(rd) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut versions: Vec<PathBuf> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.join("libcoreclr.dylib").is_file())
            .collect();
        versions.sort();
        if let Some(best) = versions.pop() {
            return Some(best.join("libcoreclr.dylib"));
        }
    }
    None
}

fn find_host_dylib_hostfxr() -> Option<PathBuf> {
    let mut roots = vec![
        PathBuf::from("/usr/local/share/dotnet/host/fxr"),
        PathBuf::from("/opt/homebrew/share/dotnet/host/fxr"),
    ];
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".dotnet/host/fxr"));
    }
    for root in roots {
        let Ok(rd) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut versions: Vec<PathBuf> = rd
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.join("libhostfxr.dylib").is_file())
            .collect();
        versions.sort();
        if let Some(best) = versions.pop() {
            return Some(best.join("libhostfxr.dylib"));
        }
    }
    None
}

fn find_host_dylib_mono() -> Option<PathBuf> {
    for candidate in [
        "/opt/homebrew/lib/libmonobdwgc-2.0.dylib",
        "/opt/homebrew/lib/libmono-2.0.dylib",
        "/usr/local/lib/libmonobdwgc-2.0.dylib",
        "/usr/local/lib/libmono-2.0.dylib",
        "/Library/Frameworks/Mono.framework/Versions/Current/lib/libmonobdwgc-2.0.dylib",
    ] {
        let p = PathBuf::from(candidate);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use pe_image::consts::{
        CLR_METADATA_SIGNATURE_BSJB, COMIMAGE_FLAGS_ILONLY, COMIMAGE_FLAGS_IL_LIBRARY,
        IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR, IMAGE_FILE_MACHINE_AMD64,
    };

    /// Build a minimal valid PE32+ image containing an `IMAGE_COR20_HEADER` and `BSJB`
    /// metadata root in `.text`, optionally with a ReadyToRun `ManagedNativeHeader`.
    fn build_synthetic_dotnet_pe(with_r2r: bool) -> Vec<u8> {
        let mut img = vec![0u8; 0x1000];
        // DOS header
        img[0..2].copy_from_slice(b"MZ");
        img[0x3c..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        // PE signature "PE\0\0"
        let pe = 0x80usize;
        img[pe..pe + 4].copy_from_slice(b"PE\0\0");
        // COFF header (20 bytes)
        img[pe + 4..pe + 6].copy_from_slice(&IMAGE_FILE_MACHINE_AMD64.to_le_bytes());
        img[pe + 6..pe + 8].copy_from_slice(&1u16.to_le_bytes()); // 1 section
        img[pe + 20..pe + 22].copy_from_slice(&0xF0u16.to_le_bytes()); // SizeOfOptionalHeader = 240
        img[pe + 22..pe + 24].copy_from_slice(&0x0022u16.to_le_bytes()); // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE

        // Optional header PE32+ (240 bytes)
        let opt = pe + 24;
        img[opt..opt + 2].copy_from_slice(&0x20bu16.to_le_bytes()); // MAGIC_64
        img[opt + 16..opt + 20].copy_from_slice(&0x200u32.to_le_bytes()); // AddressOfEntryPoint
        img[opt + 24..opt + 32].copy_from_slice(&0x1_4000_0000u64.to_le_bytes()); // ImageBase
        img[opt + 32..opt + 36].copy_from_slice(&0x200u32.to_le_bytes()); // SectionAlignment
        img[opt + 36..opt + 40].copy_from_slice(&0x200u32.to_le_bytes()); // FileAlignment
        img[opt + 56..opt + 60].copy_from_slice(&0x1000u32.to_le_bytes()); // SizeOfImage
        img[opt + 60..opt + 64].copy_from_slice(&0x200u32.to_le_bytes()); // SizeOfHeaders
        img[opt + 68..opt + 70].copy_from_slice(&3u16.to_le_bytes()); // Subsystem = CUI
        img[opt + 108..opt + 112].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes = 16

        // DataDirectory[14] = IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR at RVA 0x200, size 72
        let clr_dd = opt + 112 + IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR * 8;
        img[clr_dd..clr_dd + 4].copy_from_slice(&0x200u32.to_le_bytes());
        img[clr_dd + 4..clr_dd + 8].copy_from_slice(&72u32.to_le_bytes());

        // Section 0: ".text" at RVA 0x200, raw offset 0x200, size 0x600
        let sec = opt + 240;
        img[sec..sec + 5].copy_from_slice(b".text");
        img[sec + 8..sec + 12].copy_from_slice(&0x600u32.to_le_bytes()); // VirtualSize
        img[sec + 12..sec + 16].copy_from_slice(&0x200u32.to_le_bytes()); // VirtualAddress
        img[sec + 16..sec + 20].copy_from_slice(&0x600u32.to_le_bytes()); // SizeOfRawData
        img[sec + 20..sec + 24].copy_from_slice(&0x200u32.to_le_bytes()); // PointerToRawData
        img[sec + 36..sec + 40].copy_from_slice(&0x6000_0020u32.to_le_bytes()); // Characteristics

        // IMAGE_COR20_HEADER at 0x200
        let cor = 0x200usize;
        img[cor..cor + 4].copy_from_slice(&72u32.to_le_bytes());
        img[cor + 4..cor + 6].copy_from_slice(&2u16.to_le_bytes());
        img[cor + 6..cor + 8].copy_from_slice(&5u16.to_le_bytes());
        img[cor + 8..cor + 12].copy_from_slice(&0x280u32.to_le_bytes()); // MetaData RVA
        img[cor + 12..cor + 16].copy_from_slice(&0x80u32.to_le_bytes()); // MetaData Size
        let flags = if with_r2r {
            COMIMAGE_FLAGS_ILONLY | COMIMAGE_FLAGS_IL_LIBRARY
        } else {
            COMIMAGE_FLAGS_ILONLY
        };
        img[cor + 16..cor + 20].copy_from_slice(&flags.to_le_bytes());
        img[cor + 20..cor + 24].copy_from_slice(&0x0600_0001u32.to_le_bytes());
        if with_r2r {
            img[cor + 64..cor + 68].copy_from_slice(&0x400u32.to_le_bytes()); // ManagedNativeHeader RVA
            img[cor + 68..cor + 72].copy_from_slice(&0x100u32.to_le_bytes()); // ManagedNativeHeader Size
        }

        // BSJB metadata at 0x280
        let md = 0x280usize;
        img[md..md + 4].copy_from_slice(&CLR_METADATA_SIGNATURE_BSJB.to_le_bytes());
        img[md + 4..md + 6].copy_from_slice(&1u16.to_le_bytes());
        img[md + 6..md + 8].copy_from_slice(&1u16.to_le_bytes());
        let ver = b"v4.0.30319\0\0";
        img[md + 12..md + 16].copy_from_slice(&(ver.len() as u32).to_le_bytes());
        img[md + 16..md + 28].copy_from_slice(ver);
        img[md + 30..md + 32].copy_from_slice(&1u16.to_le_bytes()); // 1 stream
        img[md + 32..md + 36].copy_from_slice(&0x40u32.to_le_bytes());
        img[md + 36..md + 40].copy_from_slice(&0x20u32.to_le_bytes());
        img[md + 40..md + 44].copy_from_slice(b"#~\0\0");

        img
    }

    #[test]
    fn inspect_and_strip_r2r_dotnet_assembly() {
        let mut pe_bytes = build_synthetic_dotnet_pe(true);
        let rep = inspect_pe_dotnet_bytes(Path::new("TestGame.dll"), &pe_bytes).unwrap();
        assert_eq!(rep.kind, DotNetRuntimeKind::ModernCoreClrManaged);
        assert_eq!(rep.clr_mode, Some(ClrExecutionMode::ReadyToRunHybrid));
        assert!(rep.is_ready_to_run);
        assert_eq!(rep.metadata_version.as_deref(), Some("v4.0.30319"));
        assert_eq!(rep.metadata_streams, vec!["#~"]);

        // Now strip the ReadyToRun header and verify it becomes PureIlAnyCpu!
        assert!(strip_ready_to_run_header(&mut pe_bytes).unwrap());
        let rep_after = inspect_pe_dotnet_bytes(Path::new("TestGame.dll"), &pe_bytes).unwrap();
        assert_eq!(rep_after.clr_mode, Some(ClrExecutionMode::PureIlAnyCpu));
        assert!(!rep_after.is_ready_to_run);
    }

    #[test]
    fn collect_dotnet_artifacts_skips_symlink_cycles() {
        let tmp = std::env::temp_dir().join(format!(
            "port_factory_dotnet_symlink_loop_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(tmp.join("bin")).unwrap();
        let dll = tmp.join("bin/Game.dll");
        std::fs::write(&dll, b"stub").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&tmp, tmp.join("loop")).unwrap();

        let mut pes = Vec::new();
        let mut rcs = Vec::new();
        let mut meta = Vec::new();
        collect_dotnet_artifacts(&tmp, &mut pes, &mut rcs, &mut meta).unwrap();
        let _ = std::fs::remove_dir_all(&tmp);

        assert_eq!(pes, vec![dll]);
        assert!(rcs.is_empty());
        assert!(meta.is_empty());
    }
}
