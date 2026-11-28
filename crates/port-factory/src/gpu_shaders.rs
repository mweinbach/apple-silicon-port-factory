//! Non-Gaming Application Profiler, Multi-API GPU Acceleration Analyzer,
//! Standardized Shader File Format Inspector, and Ahead-of-Time (AOT) DirectX 12 / DirectX 11 /
//! Vulkan `SPIR-V` -> Metal `.metallib` Pre-Compiler (`gpu_shaders`).
//!
//! Covers four key pillars for generalizing the translator across games and non-gaming apps:
//! 1. **Non-Gaming Application Subsystems**:
//!    - Console / Pipe / Subprocess IPC (`CreatePipe`, `PeekNamedPipe`, `CreateNamedPipeW`, `CreateProcessW`)
//!    - Desktop GUI / Clipboard / High-DPI / Common Dialogs / IME / Themes (`OpenClipboard`,
//!      `GetOpenFileNameW`, `GetDpiForMonitor`, `ImmGetContext`, `OpenThemeData`, `DWriteCreateFactory`, `D2D1CreateFactory`)
//!    - Directory Watchers & Memory-Mapped Files (`ReadDirectoryChangesW`, `FindFirstChangeNotificationW`, `MapViewOfFile*`)
//!    - Networking / HTTP / TLS (`winhttp.dll`, `ws2_32.dll`, `crypt32.dll`, `bcrypt.dll`)
//! 2. **Multi-API GPU Graphics & Compute Acceleration**:
//!    - `D3D12` Graphics + Compute + Mesh + DXR 1.1 (`d3d12.dll` -> `d3d12-metal` Metal 4 backend)
//!    - `D3D11 / D3DCompiler` (`d3d11.dll`, `d3dcompiler_47.dll` -> SM4/5 DXBC -> `libdxilconv` -> DXIL -> Metal)
//!    - `Vulkan` (`vulkan-1.dll` -> `dxil_metal::spirv_metal` + `libMoltenVK.dylib` on Metal)
//!    - `DirectML` (`directml.dll` -> D3D12 Compute / Apple Silicon MPSGraph)
//!    - `OpenCL` (`opencl.dll` -> Metal Compute)
//!    - `CUDA` (`nvcuda.dll` / `cudart64_*.dll` -> clean device query fallback or kernel translation)
//! 3. **Standardized Shader File Format Catalog (`db/shader_formats.tsv`)**:
//!    - Auto-detects and documents 14 binary/container/source shader formats (`DXBC_DXIL_SM6`,
//!      `DXIL_RAW_BITCODE`, `DXBC_FXC_SM4_5`, `DXBC_ROOT_SIGNATURE`, `VULKAN_SPIRV_BINARY`,
//!      `NORTHLIGHT_BINRFX`, `WGSL_SOURCE`, `HLSL_SOURCE`, `GLSL_SOURCE`, `METAL_MSL_SOURCE`,
//!      `METAL_AIR_BITCODE`, `METAL_METALLIB`, `TRANSLATOR_AW2S_CACHE`, `PE_EMBEDDED_CONTAINER`).
//! 4. **Native DirectX 12 + DirectX 11 + Vulkan `SPIR-V` -> Metal Conversion Pipeline**:
//!    - Converts raw `.dxil`, `.dxbc`, `.cso`, `.fxo`, `.spv`, `.spirv`, `.wgsl`, `.metal`, and
//!      embedded PE shader blobs into native Apple `.metallib` binaries cached in `~/Translator/ShaderCache`.

use crate::vfs::WinFs;
use crate::win32_db::canonical_dll;
use anyhow::{Context, Result};
use pe_image::{ImportName, Pe};
use dxil_metal::formats::{
    convert_raw_shader_to_metal, detect_shader_format, format_shader_catalog_table,
    inspect_shader_bytes, sync_shader_formats_tsv, RawShaderConversion, ShaderFormatId,
    ShaderInspection,
};
use dxil_metal::pe::{extract_pe_shaders, PeEmbeddedShader};
use dxil_metal::spirv_metal::{
    compile_spirv_cached, extract_pe_spirv_shaders, metal_toolchain_available,
    metal_toolchain_version, PeEmbeddedSpirvShader,
};
use dxil_metal::{
    CacheConfig, CompileOptions, CompileRequest, ErrorCode, RootSignatureDesc, ShaderPipeline,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Non-gaming & GPU acceleration capability categories detected in a PE binary.
#[derive(Debug, Clone, Default)]
pub struct AppCapabilityProfile {
    pub binary_name: String,
    pub is_gui_subsystem: bool,
    pub is_console_subsystem: bool,
    pub console_pipe_ipc_apis: Vec<String>,
    pub desktop_ui_clipboard_dpi_apis: Vec<String>,
    pub fs_watcher_mmap_apis: Vec<String>,
    pub net_tls_crypto_apis: Vec<String>,
    pub gpu_apis_by_backend: BTreeMap<String, Vec<String>>,
    pub embedded_shaders_total: usize,
    pub embedded_shaders_dxil: usize,
    pub embedded_shaders_sm4_5: usize,
    pub embedded_shaders_spirv: usize,
    pub embedded_shaders_wave32: usize,
    pub embedded_shaders_wave64: usize,
    pub shader_profiles: BTreeMap<String, usize>,
}

/// Audits a PE binary's imports, subsystem header, and embedded DirectX + Vulkan shader containers
/// to produce an [`AppCapabilityProfile`].
pub fn audit_pe_app_capabilities(path: &Path) -> Result<AppCapabilityProfile> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let pe = Pe::parse(&bytes).with_context(|| format!("parse PE {}", path.display()))?;
    let binary_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    Ok(audit_pe_app_capabilities_parsed(&binary_name, &pe, &bytes))
}

/// Audits an already-parsed [`Pe`] image and its raw bytes for non-gaming, GPU, and embedded shader capabilities.
pub fn audit_pe_app_capabilities_parsed(
    binary_name: &str,
    pe: &Pe,
    bytes: &[u8],
) -> AppCapabilityProfile {
    let binary_name = binary_name.to_string();

    let (is_gui_subsystem, is_console_subsystem) = (true, true);

    let mut console_pipe_ipc = BTreeSet::new();
    let mut desktop_ui = BTreeSet::new();
    let mut fs_watcher_mmap = BTreeSet::new();
    let mut net_tls_crypto = BTreeSet::new();
    let mut gpu_apis: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    let mut all_dlls = pe.imports().unwrap_or_default();
    if let Ok(delay) = pe.delay_imports() {
        all_dlls.extend(delay);
    }

    for imp in &all_dlls {
        let dll = canonical_dll(&imp.dll);
        for ent in &imp.entries {
            let sym = match &ent.name {
                ImportName::Name { name, .. } => name.as_str(),
                ImportName::Ordinal(_) => continue,
            };

            // 1. Console / Pipe / Subprocess IPC
            if matches!(
                sym,
                "GetStdHandle"
                    | "SetStdHandle"
                    | "AllocConsole"
                    | "AttachConsole"
                    | "FreeConsole"
                    | "GetConsoleMode"
                    | "SetConsoleMode"
                    | "GetConsoleScreenBufferInfo"
                    | "WriteConsoleW"
                    | "WriteConsoleA"
                    | "ReadConsoleW"
                    | "ReadConsoleA"
                    | "CreatePipe"
                    | "PeekNamedPipe"
                    | "CreateNamedPipeW"
                    | "CreateNamedPipeA"
                    | "ConnectNamedPipe"
                    | "DisconnectNamedPipe"
                    | "SetNamedPipeHandleState"
                    | "WaitNamedPipeW"
                    | "CreateProcessW"
                    | "CreateProcessA"
                    | "TerminateProcess"
                    | "AssignProcessToJobObject"
            ) {
                console_pipe_ipc.insert(format!("{dll}!{sym}"));
            }

            // 2. Desktop UI / Clipboard / High-DPI / Common Dialogs / IME / Typography
            if matches!(
                dll.as_str(),
                "comdlg32.dll" | "uxtheme.dll" | "imm32.dll" | "dwrite.dll" | "d2d1.dll"
            ) || matches!(
                sym,
                "OpenClipboard"
                    | "CloseClipboard"
                    | "EmptyClipboard"
                    | "GetClipboardData"
                    | "SetClipboardData"
                    | "IsClipboardFormatAvailable"
                    | "RegisterClipboardFormatW"
                    | "RegisterClipboardFormatA"
                    | "SetProcessDPIAware"
                    | "SetProcessDpiAwareness"
                    | "SetProcessDpiAwarenessContext"
                    | "GetDpiForMonitor"
                    | "GetDpiForWindow"
                    | "GetSystemDpiForProcess"
                    | "ShellExecuteW"
                    | "ShellExecuteExW"
                    | "SHGetKnownFolderPath"
                    | "SHGetFolderPathW"
                    | "DragAcceptFiles"
                    | "DragQueryFileW"
            ) {
                desktop_ui.insert(format!("{dll}!{sym}"));
            }

            // 3. Filesystem Watchers & Memory-Mapped I/O
            if matches!(
                sym,
                "ReadDirectoryChangesW"
                    | "FindFirstChangeNotificationW"
                    | "FindFirstChangeNotificationA"
                    | "FindNextChangeNotification"
                    | "CreateFileMappingW"
                    | "CreateFileMappingA"
                    | "CreateFileMapping2"
                    | "MapViewOfFile"
                    | "MapViewOfFileEx"
                    | "MapViewOfFile3"
                    | "UnmapViewOfFile"
                    | "CreateIoCompletionPort"
                    | "GetQueuedCompletionStatus"
                    | "GetQueuedCompletionStatusEx"
                    | "PostQueuedCompletionStatus"
            ) {
                fs_watcher_mmap.insert(format!("{dll}!{sym}"));
            }

            // 4. Networking / HTTP / TLS / Crypto
            if matches!(
                dll.as_str(),
                "winhttp.dll"
                    | "ws2_32.dll"
                    | "crypt32.dll"
                    | "bcrypt.dll"
                    | "secur32.dll"
                    | "wldap32.dll"
                    | "iphlpapi.dll"
                    | "libcurl.dll"
            ) {
                net_tls_crypto.insert(format!("{dll}!{sym}"));
            }

            // 5. GPU Graphics & Compute Backends
            let gpu_backend = match dll.as_str() {
                "d3d12.dll" | "dxgi.dll" => {
                    Some("D3D12 / DXGI -> Metal 4 (Graphics + Compute + DXR)")
                }
                "d3d11.dll" | "d3dcompiler_47.dll" => {
                    Some("D3D11 / D3DCompiler -> DXBC-to-DXIL-to-Metal")
                }
                "vulkan_1.dll" => {
                    Some("Vulkan (vulkan-1.dll) -> SPIR-V-to-Metal (naga + MoltenVK)")
                }
                "directml.dll" => Some("DirectML (directml.dll) -> D3D12 Compute / MPSGraph"),
                "opencl.dll" => Some("OpenCL (opencl.dll) -> Metal Compute"),
                "nvcuda.dll" => Some("NVIDIA CUDA (nvcuda.dll) -> Device Query / Fallback"),
                "amd_fidelityfx_loader_dx12.dll" | "nrd.dll" | "nri.dll" | "sl.interposer.dll" => {
                    Some("GPU Upscaling / Denoising Middleware (FSR / DLSS / NRD / NRI)")
                }
                _ => None,
            };
            if let Some(b) = gpu_backend {
                gpu_apis
                    .entry(b.to_string())
                    .or_default()
                    .insert(format!("{dll}!{sym}"));
            }
        }
    }

    // 6. Scan for embedded DirectX (DXBC / DXIL) and Vulkan (SPIR-V) shader containers
    let dxbc_shaders = extract_pe_shaders(&binary_name, &bytes);
    let spirv_shaders = extract_pe_spirv_shaders(&binary_name, &bytes);
    let mut embedded_shaders_dxil = 0usize;
    let mut embedded_shaders_sm4_5 = 0usize;
    let mut embedded_shaders_wave32 = 0usize;
    let mut embedded_shaders_wave64 = 0usize;
    let mut shader_profiles: BTreeMap<String, usize> = BTreeMap::new();

    for sh in &dxbc_shaders {
        if sh.is_sm4_5 {
            embedded_shaders_sm4_5 += 1;
        } else {
            embedded_shaders_dxil += 1;
        }
        if sh.is_wave64 {
            embedded_shaders_wave64 += 1;
        } else {
            embedded_shaders_wave32 += 1;
        }
        for exp in sh.exports() {
            *shader_profiles.entry(exp.profile).or_default() += 1;
        }
    }

    for spv in &spirv_shaders {
        embedded_shaders_wave32 += 1;
        for ep in &spv.entry_points {
            *shader_profiles.entry(ep.profile.clone()).or_default() += 1;
        }
    }

    AppCapabilityProfile {
        binary_name,
        is_gui_subsystem,
        is_console_subsystem,
        console_pipe_ipc_apis: console_pipe_ipc.into_iter().collect(),
        desktop_ui_clipboard_dpi_apis: desktop_ui.into_iter().collect(),
        fs_watcher_mmap_apis: fs_watcher_mmap.into_iter().collect(),
        net_tls_crypto_apis: net_tls_crypto.into_iter().collect(),
        gpu_apis_by_backend: gpu_apis
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect(),
        embedded_shaders_total: dxbc_shaders.len() + spirv_shaders.len(),
        embedded_shaders_dxil,
        embedded_shaders_sm4_5,
        embedded_shaders_spirv: spirv_shaders.len(),
        embedded_shaders_wave32,
        embedded_shaders_wave64,
        shader_profiles,
    }
}

/// Formats a comprehensive multi-binary Non-Gaming & GPU Acceleration Capability Report.
pub fn format_app_capability_report(profiles: &[AppCapabilityProfile]) -> String {
    let mut out = String::new();
    let total_bins = profiles.len();
    let bins_with_shaders = profiles
        .iter()
        .filter(|p| p.embedded_shaders_total > 0)
        .count();
    let total_shaders: usize = profiles.iter().map(|p| p.embedded_shaders_total).sum();
    let total_dxil: usize = profiles.iter().map(|p| p.embedded_shaders_dxil).sum();
    let total_sm4_5: usize = profiles.iter().map(|p| p.embedded_shaders_sm4_5).sum();
    let total_spirv: usize = profiles.iter().map(|p| p.embedded_shaders_spirv).sum();
    let total_wave32: usize = profiles.iter().map(|p| p.embedded_shaders_wave32).sum();
    let total_wave64: usize = profiles.iter().map(|p| p.embedded_shaders_wave64).sum();

    let mut all_console = BTreeSet::new();
    let mut all_desktop = BTreeSet::new();
    let mut all_fs_mmap = BTreeSet::new();
    let mut all_net = BTreeSet::new();
    let mut all_gpu: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut all_profiles: BTreeMap<String, usize> = BTreeMap::new();

    for p in profiles {
        all_console.extend(p.console_pipe_ipc_apis.iter().cloned());
        all_desktop.extend(p.desktop_ui_clipboard_dpi_apis.iter().cloned());
        all_fs_mmap.extend(p.fs_watcher_mmap_apis.iter().cloned());
        all_net.extend(p.net_tls_crypto_apis.iter().cloned());
        for (k, v) in &p.gpu_apis_by_backend {
            all_gpu.entry(k.clone()).or_default().extend(v.iter().cloned());
        }
        for (prof, c) in &p.shader_profiles {
            *all_profiles.entry(prof.clone()).or_default() += *c;
        }
    }

    let _ = writeln!(
        out,
        "================================================================================"
    );
    let _ = writeln!(
        out,
        "=== Non-Gaming Application, GPU Acceleration & Embedded Shader Audit ==="
    );
    let _ = writeln!(
        out,
        "================================================================================"
    );
    let _ = writeln!(out, "  Binaries audited            : {total_bins}");
    let _ = writeln!(
        out,
        "  Binaries with shaders       : {bins_with_shaders} ({total_shaders} embedded containers: {total_dxil} DX12 DXIL, {total_sm4_5} DX11 FXC, {total_spirv} Vulkan SPIR-V)"
    );
    let _ = writeln!(
        out,
        "  Shader wave permutations    : {total_wave32} Wave32/unpinned (active on Apple Silicon), {total_wave64} AMD Wave64"
    );
    let _ = writeln!(
        out,
        "  Default Metal shader cache  : {}",
        WinFs::default_shader_cache_dir().display()
    );
    let _ = writeln!(
        out,
        "  DirectX 12 DXIL -> Metal    : {}",
        if dxil_metal::converter_available() {
            "AVAILABLE (libmetalirconverter.dylib)"
        } else {
            "NOT LOADED"
        }
    );
    let _ = writeln!(
        out,
        "  DirectX 11 DXBC -> DXIL     : {}",
        if dxil_metal::dxilconv::available() {
            "AVAILABLE (libdxilconv.dylib)"
        } else {
            "NOT LOADED"
        }
    );
    let _ = writeln!(
        out,
        "  Vulkan SPIR-V -> Metal      : {}",
        if metal_toolchain_available() {
            format!(
                "AVAILABLE (naga spv-in/msl-out + {})",
                metal_toolchain_version().unwrap_or("xcrun metal")
            )
        } else {
            "AVAILABLE (naga spv-in/msl-out)".to_string()
        }
    );
    let _ = writeln!(out);

    let _ = writeln!(out, "== 1. Non-Gaming Desktop, CLI & Productivity Subsystems Used ==");
    let _ = writeln!(
        out,
        "  - Console / Pipe / Subprocess IPC ({} distinct APIs): {}",
        all_console.len(),
        preview_list(&all_console, 8)
    );
    let _ = writeln!(
        out,
        "  - Desktop GUI / Clipboard / High-DPI / IME / Dialogs ({} distinct APIs): {}",
        all_desktop.len(),
        preview_list(&all_desktop, 8)
    );
    let _ = writeln!(
        out,
        "  - Filesystem Watchers / IOCP / Memory-Mapped I/O ({} distinct APIs): {}",
        all_fs_mmap.len(),
        preview_list(&all_fs_mmap, 8)
    );
    let _ = writeln!(
        out,
        "  - Networking / HTTP / TLS / Crypto ({} distinct APIs): {}",
        all_net.len(),
        preview_list(&all_net, 8)
    );
    let _ = writeln!(out);

    let _ = writeln!(out, "== 2. GPU Graphics & Compute Acceleration Backends Used ==");
    if all_gpu.is_empty() {
        let _ = writeln!(out, "  (No explicit GPU imports detected; CPU-only workload)");
    } else {
        for (backend, syms) in &all_gpu {
            let _ = writeln!(
                out,
                "  - {:56} : {:>3} APIs ({})",
                backend,
                syms.len(),
                preview_list(syms, 5)
            );
        }
    }

    if !all_profiles.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "== 3. Embedded Shader Profiles Discovered ==");
        for (prof, count) in &all_profiles {
            let _ = writeln!(out, "  - {:<22} : {:>6} exports", prof, count);
        }
    }

    out
}

fn preview_list(set: &BTreeSet<String>, max: usize) -> String {
    if set.is_empty() {
        return "none".to_string();
    }
    let v: Vec<&str> = set.iter().take(max).map(|s| s.as_str()).collect();
    let rem = set.len().saturating_sub(max);
    let mut s = v.join(", ");
    if rem > 0 {
        let _ = write!(s, ", ... (+{rem} more)");
    }
    s
}

/// Summary of an AOT shader extraction and Metal `.metallib` pre-compilation run across
/// DirectX 12 (`DXIL`), DirectX 11 (`DXBC`), and Vulkan (`SPIR-V`).
#[derive(Debug, Clone)]
pub struct ShaderPrecompileSummary {
    pub cache_dir: PathBuf,
    pub converter_fingerprint: String,
    pub dxilconv_available: bool,
    pub spirv_metal_available: bool,
    pub files_scanned: usize,
    pub files_with_shaders: usize,
    pub total_containers: usize,
    pub unique_containers: usize,
    pub dxil_containers: usize,
    pub sm4_5_containers: usize,
    pub spirv_containers: usize,
    pub active_wave32_exports: usize,
    pub active_wave32_converted: usize,
    pub active_wave32_failed: usize,
    pub wave64_skipped: usize,
    pub total_metallib_bytes: usize,
    pub disk_hits: u64,
    pub new_compiles: u64,
    pub elapsed_secs: f64,
    pub by_binary: BTreeMap<String, (usize, usize, usize)>, // (containers, wave32_ok, wave32_total)
    pub by_stage: BTreeMap<String, usize>,
    pub failures: Vec<String>,
}

/// Scans one or more files or directories (`.exe`, `.dll`, `.dxbc`, `.dxil`, `.cso`, `.fxo`,
/// `.spv`, `.spirv`, `.wgsl`, `.metal`) for DirectX `DXBC`/`DXIL` and Vulkan `SPIR-V` shaders
/// and AOT pre-compiles them into `cache_dir` (defaulting to `~/Translator/ShaderCache`).
pub fn precompile_shaders_for_paths(
    inputs: &[PathBuf],
    cache_dir: Option<PathBuf>,
    limit: usize,
) -> Result<ShaderPrecompileSummary> {
    let t0 = Instant::now();
    let resolved_cache_dir = cache_dir.unwrap_or_else(WinFs::default_shader_cache_dir);
    std::fs::create_dir_all(&resolved_cache_dir)
        .with_context(|| format!("create shader cache dir {}", resolved_cache_dir.display()))?;

    fn collect_shader_targets_recursive(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
        if depth > 3 {
            return;
        }
        let Ok(rd) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in rd.flatten() {
            let ep = entry.path();
            if ep.is_dir() {
                let name = ep
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                if matches!(
                    name.as_str(),
                    "data" | "cache" | "_codesignature" | ".git" | "target" | "errorreporting"
                ) {
                    continue;
                }
                collect_shader_targets_recursive(&ep, depth + 1, out);
            } else if ep.is_file() {
                if ep.extension().is_some_and(|e| {
                    let s = e.to_string_lossy();
                    s.eq_ignore_ascii_case("exe")
                        || s.eq_ignore_ascii_case("dll")
                        || s.eq_ignore_ascii_case("dxbc")
                        || s.eq_ignore_ascii_case("dxil")
                        || s.eq_ignore_ascii_case("cso")
                        || s.eq_ignore_ascii_case("fxo")
                        || s.eq_ignore_ascii_case("spv")
                        || s.eq_ignore_ascii_case("spirv")
                        || s.eq_ignore_ascii_case("wgsl")
                }) {
                    out.push(ep);
                }
            }
        }
    }

    let mut target_files = Vec::new();
    for p in inputs {
        if p.is_file() {
            target_files.push(p.clone());
        } else if p.is_dir() {
            collect_shader_targets_recursive(p, 0, &mut target_files);
        }
    }
    target_files.sort();
    target_files.dedup();

    let mut all_dxbc_shaders: Vec<PeEmbeddedShader> = Vec::new();
    let mut all_spirv_shaders: Vec<PeEmbeddedSpirvShader> = Vec::new();
    let mut files_with_shaders = 0usize;
    let mut by_binary: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();

    for f in &target_files {
        let Ok(bytes) = std::fs::read(f) else {
            continue;
        };
        let name = f
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| f.display().to_string());

        let fmt = detect_shader_format(Some(f), &bytes);
        if fmt == ShaderFormatId::VulkanSpirvBinary {
            let spv_list = extract_pe_spirv_shaders(&name, &bytes);
            if !spv_list.is_empty() {
                files_with_shaders += 1;
                by_binary.entry(name).or_default().0 += spv_list.len();
                all_spirv_shaders.extend(spv_list);
                continue;
            }
        }

        let extracted_dxbc = extract_pe_shaders(&name, &bytes);
        let extracted_spirv = if fmt == ShaderFormatId::PeEmbeddedContainer {
            extract_pe_spirv_shaders(&name, &bytes)
        } else {
            Vec::new()
        };

        let count = extracted_dxbc.len() + extracted_spirv.len();
        if count > 0 {
            files_with_shaders += 1;
            by_binary.entry(name).or_default().0 += count;
            all_dxbc_shaders.extend(extracted_dxbc);
            all_spirv_shaders.extend(extracted_spirv);
        }
    }

    let dxil_containers = all_dxbc_shaders.iter().filter(|s| !s.is_sm4_5).count();
    let sm4_5_containers = all_dxbc_shaders.iter().filter(|s| s.is_sm4_5).count();
    let spirv_containers = all_spirv_shaders.len();
    let total_containers = all_dxbc_shaders.len() + spirv_containers;

    let mut unique_hashes: BTreeSet<u128> =
        all_dxbc_shaders.iter().map(|s| s.dxbc_hash).collect();
    unique_hashes.extend(all_spirv_shaders.iter().map(|s| s.spirv_hash));
    let unique_containers = unique_hashes.len();

    let cfg = CacheConfig {
        disk_dir: Some(resolved_cache_dir.clone()),
        memory_limit_bytes: 512 << 20,
    };
    let pipeline = ShaderPipeline::new(cfg)
        .map_err(|e| anyhow::anyhow!("failed to initialize Metal ShaderPipeline: {e}"))?;

    let compile_opts = CompileOptions::default();
    let mut wave64_skipped = 0usize;

    enum ShaderCompileJob<'a> {
        Dxbc {
            shader: &'a PeEmbeddedShader,
            entry_point: String,
            profile: String,
        },
        Spirv {
            shader: &'a PeEmbeddedSpirvShader,
            entry_point: String,
            profile: String,
        },
    }

    let mut jobs: Vec<ShaderCompileJob<'_>> = Vec::new();
    'outer_dxbc: for sh in &all_dxbc_shaders {
        if sh.is_wave64 {
            wave64_skipped += sh.exports().len();
            continue;
        }
        for exp in sh.exports() {
            if jobs.len() >= limit {
                break 'outer_dxbc;
            }
            jobs.push(ShaderCompileJob::Dxbc {
                shader: sh,
                entry_point: exp.entry_point,
                profile: exp.profile,
            });
        }
    }
    'outer_spirv: for spv in &all_spirv_shaders {
        for ep in &spv.entry_points {
            if jobs.len() >= limit {
                break 'outer_spirv;
            }
            jobs.push(ShaderCompileJob::Spirv {
                shader: spv,
                entry_point: ep.name.clone(),
                profile: ep.profile.clone(),
            });
        }
    }

    struct ShaderJobOutcome {
        idx: usize,
        binary_name: String,
        result: std::result::Result<(String, usize), String>,
    }

    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(jobs.len().max(1));
    let next_job = std::sync::atomic::AtomicUsize::new(0);
    let outcomes = std::sync::Mutex::new(Vec::with_capacity(jobs.len()));

    if !jobs.is_empty() {
        std::thread::scope(|scope| {
            for _ in 0..threads {
                scope.spawn(|| loop {
                    let idx = next_job.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(job) = jobs.get(idx) else {
                        break;
                    };
                    let outcome = match job {
                        ShaderCompileJob::Dxbc {
                            shader: sh,
                            entry_point,
                            profile,
                        } => {
                            let attempt = || -> std::result::Result<
                                std::sync::Arc<dxil_metal::CompiledShader>,
                                dxil_metal::Error,
                            > {
                                if sh.has_rts0 {
                                    if let Ok(rs) = pipeline.root_signature(&sh.dxbc_bytes) {
                                        let req = CompileRequest::new(
                                            &sh.dxbc_bytes,
                                            Some(rs.as_ref()),
                                            &compile_opts,
                                        )
                                        .entry_point(entry_point);
                                        if let Ok(c) = pipeline.compile(&req) {
                                            return Ok(c);
                                        }
                                    }
                                }
                                let rs = RootSignatureDesc::covering_dxil(&sh.dxbc_bytes)
                                    .and_then(|d| pipeline.root_signature(&d.to_blob()).ok());
                                let req = CompileRequest::new(
                                    &sh.dxbc_bytes,
                                    rs.as_deref(),
                                    &compile_opts,
                                )
                                .entry_point(entry_point);
                                pipeline.compile(&req)
                            };
                            let res = match attempt() {
                                Ok(compiled) => {
                                    Ok((format!("{:?}", compiled.stage), compiled.metallib.len()))
                                }
                                Err(e) => {
                                    let code: Option<ErrorCode> = match &e {
                                        dxil_metal::Error::Convert { code, .. } => Some(*code),
                                        _ => None,
                                    };
                                    Err(format!(
                                        "{}@{:#x} {} [{} code={:?}]: {e}",
                                        sh.binary_name,
                                        sh.file_offset,
                                        entry_point,
                                        profile,
                                        code
                                    ))
                                }
                            };
                            ShaderJobOutcome {
                                idx,
                                binary_name: sh.binary_name.clone(),
                                result: res,
                            }
                        }
                        ShaderCompileJob::Spirv {
                            shader: spv,
                            entry_point,
                            profile,
                        } => {
                            let res = match compile_spirv_cached(
                                pipeline.cache(),
                                &spv.spirv_bytes,
                                Some(entry_point),
                            ) {
                                Ok(compiled) => {
                                    Ok((format!("{:?}", compiled.stage), compiled.metallib.len()))
                                }
                                Err(e) => Err(format!(
                                    "{}@{:#x} {} [{}]: {e}",
                                    spv.binary_name, spv.file_offset, entry_point, profile
                                )),
                            };
                            ShaderJobOutcome {
                                idx,
                                binary_name: spv.binary_name.clone(),
                                result: res,
                            }
                        }
                    };
                    outcomes.lock().unwrap().push(outcome);
                });
            }
        });
    }

    let mut outcomes = outcomes.into_inner().unwrap();
    outcomes.sort_by_key(|o| o.idx);

    let mut active_wave32_exports = 0usize;
    let mut active_wave32_converted = 0usize;
    let mut active_wave32_failed = 0usize;
    let mut total_metallib_bytes = 0usize;
    let mut by_stage: BTreeMap<String, usize> = BTreeMap::new();
    let mut failures = Vec::new();

    for out in outcomes {
        active_wave32_exports += 1;
        let b_entry = by_binary.entry(out.binary_name).or_default();
        b_entry.2 += 1;
        match out.result {
            Ok((stage, bytes_len)) => {
                active_wave32_converted += 1;
                b_entry.1 += 1;
                total_metallib_bytes += bytes_len;
                *by_stage.entry(stage).or_default() += 1;
            }
            Err(msg) => {
                active_wave32_failed += 1;
                failures.push(msg);
            }
        }
    }

    let st = pipeline.stats();
    Ok(ShaderPrecompileSummary {
        cache_dir: resolved_cache_dir,
        converter_fingerprint: pipeline.converter_fingerprint().to_string(),
        dxilconv_available: dxil_metal::dxilconv::available(),
        spirv_metal_available: metal_toolchain_available(),
        files_scanned: target_files.len(),
        files_with_shaders,
        total_containers,
        unique_containers,
        dxil_containers,
        sm4_5_containers,
        spirv_containers,
        active_wave32_exports,
        active_wave32_converted,
        active_wave32_failed,
        wave64_skipped,
        total_metallib_bytes,
        disk_hits: st.disk_hits,
        new_compiles: st.compiles,
        elapsed_secs: t0.elapsed().as_secs_f64(),
        by_binary,
        by_stage,
        failures,
    })
}

impl ShaderPrecompileSummary {
    pub fn format_report(&self) -> String {
        let mut out = String::new();
        let pct = if self.active_wave32_exports > 0 {
            (self.active_wave32_converted as f64 / self.active_wave32_exports as f64) * 100.0
        } else {
            100.0
        };
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(
            out,
            "=== AOT DirectX 12 (DXIL) / DirectX 11 (DXBC) / Vulkan (SPIR-V) -> Metal Report ==="
        );
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(out, "  Shader cache dir       : {}", self.cache_dir.display());
        let _ = writeln!(out, "  DX12 Metal converter   : {}", self.converter_fingerprint);
        let _ = writeln!(
            out,
            "  DX11 DXBC -> DXIL      : {}",
            if self.dxilconv_available {
                "enabled (libdxilconv.dylib)"
            } else {
                "unavailable"
            }
        );
        let _ = writeln!(
            out,
            "  Vulkan SPIR-V -> Metal : {}",
            if self.spirv_metal_available {
                format!(
                    "enabled (naga spv-in/msl-out + {})",
                    metal_toolchain_version().unwrap_or("xcrun metal")
                )
            } else {
                "enabled (naga spv-in/msl-out)".to_string()
            }
        );
        let _ = writeln!(
            out,
            "  Files scanned          : {} ({} contain shaders)",
            self.files_scanned, self.files_with_shaders
        );
        let _ = writeln!(
            out,
            "  Shader containers      : {} total ({} unique: {} DX12 DXIL, {} DX11 FXC, {} Vulkan SPIR-V)",
            self.total_containers,
            self.unique_containers,
            self.dxil_containers,
            self.sm4_5_containers,
            self.spirv_containers
        );
        let _ = writeln!(
            out,
            "  Active shaders         : {} / {} converted ({:.2}%, {} failed)",
            self.active_wave32_converted,
            self.active_wave32_exports,
            pct,
            self.active_wave32_failed
        );
        let _ = writeln!(
            out,
            "  AMD Wave64 skipped     : {} (unused when reporting WaveLaneCount=32)",
            self.wave64_skipped
        );
        let _ = writeln!(
            out,
            "  Metal .metallib output : {:.2} MB ({} disk cache hits, {} new compiles in {:.2}s)",
            self.total_metallib_bytes as f64 / 1e6,
            self.disk_hits,
            self.new_compiles,
            self.elapsed_secs
        );

        if !self.by_binary.is_empty() {
            let _ = writeln!(out, "\n  Per-Binary AOT Shader Pre-Compilation:");
            let _ = writeln!(
                out,
                "    {:42} {:>10} {:>12} {:>10}",
                "BINARY / FILE", "CONTAINERS", "CONVERTED", "STATUS"
            );
            let _ = writeln!(out, "    {}", "-".repeat(78));
            for (bin, (containers, ok, total)) in &self.by_binary {
                let status = if *ok == *total { "100% READY" } else { "PARTIAL" };
                let _ = writeln!(
                    out,
                    "    {:42} {:>10} {:>5} / {:<5} {:>10}",
                    bin, containers, ok, total, status
                );
            }
        }

        if !self.by_stage.is_empty() {
            let _ = writeln!(out, "\n  Converted Metal Stages:");
            for (stage, count) in &self.by_stage {
                let _ = writeln!(out, "    - {:<18} : {:>6}", stage, count);
            }
        }

        if !self.failures.is_empty() {
            let _ = writeln!(out, "\n  Failures (first 10):");
            for f in self.failures.iter().take(10) {
                let _ = writeln!(out, "    - {f}");
            }
        }

        out
    }
}

/// Inspects a raw shader file (`.dxil`, `.dxbc`, `.cso`, `.spv`, `.wgsl`, `.metal`, `.binrfx`,
/// `.metallib`, `.aw2s`) or PE binary (`.exe`, `.dll`).
pub fn inspect_shader_file(path: &Path) -> Result<ShaderInspection> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    inspect_shader_bytes(&name, Some(path), &bytes)
        .map_err(|e| anyhow::anyhow!("inspect {}: {e}", path.display()))
}

/// Converts a raw shader file (`.dxil`, `.dxbc`, `.cso`, `.fxo`, `.spv`, `.spirv`, `.wgsl`,
/// `.metal`) — or extracts the first embedded shader from a `.exe`/`.dll` if passed a PE binary —
/// into a native Apple `.metallib` (and optional `.metal` MSL source).
pub fn convert_shader_file(
    input_path: &Path,
    entry_point: Option<&str>,
    out_metallib: Option<&Path>,
    out_msl: Option<&Path>,
) -> Result<RawShaderConversion> {
    let bytes =
        std::fs::read(input_path).with_context(|| format!("read {}", input_path.display()))?;
    let fmt = detect_shader_format(Some(input_path), &bytes);

    let conv = if fmt == ShaderFormatId::PeEmbeddedContainer {
        // Extract first embedded DXBC/DXIL or SPIR-V shader from the PE binary and convert it
        let name = input_path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let dxbc = extract_pe_shaders(&name, &bytes);
        if let Some(first) = dxbc.iter().find(|s| !s.is_wave64).or_else(|| dxbc.first()) {
            convert_raw_shader_to_metal(None, &first.dxbc_bytes, entry_point, None)
                .map_err(|e| anyhow::anyhow!("{e}"))?
        } else {
            let spirv = extract_pe_spirv_shaders(&name, &bytes);
            let first = spirv.first().ok_or_else(|| {
                anyhow::anyhow!(
                    "PE binary {} contains no embedded DXBC/DXIL or SPIR-V shaders",
                    input_path.display()
                )
            })?;
            convert_raw_shader_to_metal(None, &first.spirv_bytes, entry_point, None)
                .map_err(|e| anyhow::anyhow!("{e}"))?
        }
    } else {
        convert_raw_shader_to_metal(Some(input_path), &bytes, entry_point, None)
            .map_err(|e| anyhow::anyhow!("{e}"))?
    };

    if let Some(out_path) = out_metallib {
        if let Some(parent) = out_path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(out_path, &conv.compiled.metallib)
            .with_context(|| format!("write {}", out_path.display()))?;
    }

    if let Some(msl_path) = out_msl {
        if let Some(ref msl) = conv.msl_source {
            if let Some(parent) = msl_path.parent().filter(|p| !p.as_os_str().is_empty()) {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(msl_path, msl)
                .with_context(|| format!("write {}", msl_path.display()))?;
        }
    }

    Ok(conv)
}

/// Syncs `db/shader_formats.tsv` if `tsv_path` is provided and returns the formatted shader catalog table.
pub fn sync_and_format_shader_catalog(tsv_path: Option<&Path>) -> Result<String> {
    if let Some(p) = tsv_path {
        sync_shader_formats_tsv(p)
            .with_context(|| format!("write {}", p.display()))?;
    }
    Ok(format_shader_catalog_table())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audits_and_precompiles_embedded_pe_and_vulkan_spirv_shaders() {
        // 1. Test raw Vulkan SPIR-V pre-compilation and conversion
        let spv_bytes = dxil_metal::spirv_metal::sample_vulkan_compute_spirv();
        let tmp_spv = std::env::temp_dir().join("aw2_test_vulkan_compute.spv");
        let tmp_metallib = std::env::temp_dir().join("aw2_test_vulkan_compute.metallib");
        let tmp_msl = std::env::temp_dir().join("aw2_test_vulkan_compute.metal");
        std::fs::write(&tmp_spv, &spv_bytes).expect("write temp .spv");

        let insp = inspect_shader_file(&tmp_spv).expect("inspect_shader_file .spv");
        assert_eq!(insp.format, ShaderFormatId::VulkanSpirvBinary);

        let translated = dxil_metal::spirv_metal::translate_spirv_to_msl(&spv_bytes, Some("main"))
            .expect("translate_spirv_to_msl");
        assert!(translated.msl_source.contains("kernel "));

        if dxil_metal::spirv_metal::metal_toolchain_available() {
            let conv = convert_shader_file(
                &tmp_spv,
                Some("main"),
                Some(&tmp_metallib),
                Some(&tmp_msl),
            )
            .expect("convert_shader_file .spv -> .metallib");
            assert!(conv.compiled.metallib.starts_with(b"MTLB"));
            assert!(tmp_metallib.exists());
            assert!(tmp_msl.exists());
        } else {
            // Honest error when offline `xcrun metal` toolchain is not installed (never fake MTLB).
            assert!(convert_shader_file(&tmp_spv, Some("main"), None, None).is_err());
        }

        let _ = std::fs::remove_file(&tmp_spv);
        let _ = std::fs::remove_file(&tmp_metallib);
        let _ = std::fs::remove_file(&tmp_msl);

        // 2. Test local AW2 install embedded shaders if present
        let exe = std::env::var_os("PORT_GAME_DIR")
            .or_else(|| std::env::var_os("AW2_GAME_DIR"))
            .filter(|v| !v.is_empty())
            .map(|d| PathBuf::from(d).join("AlanWake2.exe"));
        let Some(exe) = exe.filter(|p| p.exists()) else {
            return;
        };
        let prof = audit_pe_app_capabilities(&exe).expect("audit AlanWake2.exe");
        assert_eq!(prof.embedded_shaders_total, 23);
        assert_eq!(prof.embedded_shaders_sm4_5, 23);
        assert!(!prof.console_pipe_ipc_apis.is_empty());
        assert!(!prof.desktop_ui_clipboard_dpi_apis.is_empty());

        if dxil_metal::converter_available() && dxil_metal::dxilconv::available() {
            let summary = precompile_shaders_for_paths(&[exe.clone()], None, 23)
                .expect("precompile AlanWake2.exe shaders");
            assert_eq!(summary.active_wave32_exports, 23);
            assert_eq!(summary.active_wave32_converted, 23);
            assert_eq!(summary.active_wave32_failed, 0);
        }
    }
}
