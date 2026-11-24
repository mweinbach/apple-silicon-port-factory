//! Static stub inventory (`db/stub_inventory.tsv`), zero-new-stub regression guard,
//! `docs/stub-backlog.md` generator (aggregating `docs/stub-backlog.d/*.md`), and
//! runtime stub-snapshot diffing (`cargo opsdb -- stub-diff`).

use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// One static `stub!`, `failing!`, or `win64_stubs!` site discovered in `crates/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StubInventoryEntry {
    pub dll: String,
    pub api: String,
    pub kind: String,
    pub file: String,
    pub line: usize,
    pub detail: String,
}

/// Returns `true` for third-party/engine middleware DLLs whose builtin modules are
/// honest fallback stubs or partial compatibility shims.
pub fn is_middleware_stub_dll(dll: &str) -> bool {
    matches!(
        dll,
        "zstd.dll"
            | "oo2core_9_win64.dll"
            | "tbbmalloc.dll"
            | "lz4.dll"
            | "openal32.dll"
            | "libxess.dll"
            | "libxess_dx11.dll"
            | "sl.interposer.dll"
            | "eossdk-win64-shipping.dll"
            | "physx_64.dll"
            | "physxcooking_64.dll"
            | "physx3common_x64.dll"
            | "nvngx_dlss.dll"
            | "amd_fidelityfx_loader_dx12.dll"
            | "amd_ags_x64.dll"
            | "nri.dll"
            | "nrd.dll"
            | "dstorage.dll"
            | "directml.dll"
            | "libcurl.dll"
            | "xaudio2_9redist.dll"
    )
}

/// Map a workspace-relative Rust source path under `crates/` to its canonical DLL name.
pub fn source_file_to_dll(rel_path: &str) -> String {
    let Some(rest) = rel_path.strip_prefix("crates/win32-rt/src/") else {
        // Non-win32-rt crate
        let crate_name = rel_path
            .strip_prefix("crates/")
            .and_then(|s| s.split('/').next())
            .unwrap_or("unknown");
        return format!("{crate_name}");
    };
    let mut parts = rest.split('/');
    let first = parts.next().unwrap_or("");
    let stem = first.strip_suffix(".rs").unwrap_or(first);
    match stem {
        "kernel32" | "sync" | "unwind" | "thread" | "process" | "heap" | "path" | "handles" => {
            "kernel32.dll".to_string()
        }
        "ntdll" | "ntsyscall" => "ntdll.dll".to_string(),
        "ucrt" => "ucrtbase.dll".to_string(),
        "msvcrt" => "msvcrt.dll".to_string(),
        "user32" | "macos" => "user32.dll".to_string(),
        "advapi32" => "advapi32.dll".to_string(),
        "d3d12" => "d3d12.dll".to_string(),
        "dxgi" => "dxgi.dll".to_string(),
        "d3d11" => "d3d11.dll".to_string(),
        "audio" => "mmdevapi.dll".to_string(),
        "slstub" => "sl.interposer.dll".to_string(),
        "eossdk" => "eossdk-win64-shipping.dll".to_string(),
        "misc" => {
            let second = parts.next().unwrap_or("misc");
            let dll_stem = second.strip_suffix(".rs").unwrap_or(second);
            format!("{dll_stem}.dll")
        }
        other => format!("{other}.dll"),
    }
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("reading dir {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            collect_rs_files(&p, out)?;
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
    Ok(())
}

/// Strip line comments (`//...`) outside of string literals so commented-out examples
/// never match as live macro invocations.
fn strip_line_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let mut in_str = false;
    let mut esc = false;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if esc {
                esc = false;
            } else if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_str = false;
            }
        } else if b == b'"' {
            in_str = true;
        } else if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            return &line[..i];
        }
        i += 1;
    }
    line
}

/// Find a macro token like `stub!(` or `failing!(` outside of string literals on a single line.
fn find_macro_call_on_line(code_line: &str, name: &str) -> Option<usize> {
    let bytes = code_line.as_bytes();
    let pat = format!("{name}!(");
    let pat_b = pat.as_bytes();
    let mut in_str = false;
    let mut esc = false;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if esc {
                esc = false;
            } else if b == b'\\' {
                esc = true;
            } else if b == b'"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if b == b'"' {
            in_str = true;
            i += 1;
            continue;
        }
        if bytes[i..].starts_with(pat_b) {
            // Ensure word boundary before `name` (not e.g. `my_stub!(` or `__record_stub!(`).
            let prev_ok = i == 0 || {
                let prev = bytes[i - 1];
                !(prev.is_ascii_alphanumeric() || prev == b'_')
            };
            if prev_ok {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn parse_fn_name(code_line: &str) -> Option<String> {
    let trimmed = code_line.trim_start();
    // Match `fn <ident>(` preceded only by visibility/qualifiers.
    let idx = trimmed.find("fn ")?;
    let prefix = &trimmed[..idx];
    // Reject closures/types like `type F = fn(...)` or `let f: fn(...)`.
    if prefix.contains('=') || prefix.contains(':') {
        return None;
    }
    // Every token in prefix must be a valid fn modifier (`pub`, `pub(crate)`, `pub(super)`, `unsafe`, etc.).
    for tok in prefix.split_whitespace() {
        if !(tok == "pub"
            || (tok.starts_with("pub(") && tok.ends_with(')'))
            || tok == "unsafe"
            || tok == "const"
            || tok == "async"
            || tok == "extern"
            || tok.starts_with('"'))
        {
            return None;
        }
    }
    let after = &trimmed[idx + 3..];
    let ident: String = after
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    if ident.is_empty() {
        None
    } else {
        Some(ident)
    }
}

fn sanitize_detail(s: &str) -> String {
    let collapsed: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let clean = collapsed.replace('\t', " ");
    if clean.len() > 140 {
        let mut cut = 140;
        while cut > 0 && !clean.is_char_boundary(cut) {
            cut -= 1;
        }
        format!("{}...", &clean[..cut])
    } else {
        clean
    }
}

fn extract_balanced_parens(lines: &[&str], start_line_idx: usize, start_col: usize) -> String {
    let mut buf = String::new();
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    for (offset, raw_line) in lines[start_line_idx..].iter().enumerate() {
        let line = strip_line_comment(raw_line);
        let slice = if offset == 0 { &line[start_col..] } else { line };
        for ch in slice.chars() {
            if in_str {
                buf.push(ch);
                if esc {
                    esc = false;
                } else if ch == '\\' {
                    esc = true;
                } else if ch == '"' {
                    in_str = false;
                }
                continue;
            }
            if ch == '"' {
                in_str = true;
                buf.push(ch);
                continue;
            }
            if ch == '(' {
                depth += 1;
                if depth > 1 {
                    buf.push(ch);
                }
            } else if ch == ')' {
                depth -= 1;
                if depth == 0 {
                    return buf;
                }
                buf.push(ch);
            } else if depth >= 1 {
                buf.push(ch);
            }
        }
        if depth >= 1 {
            buf.push(' ');
        }
    }
    buf
}

fn scan_single_file(workspace_root: &Path, abs_path: &Path, out: &mut Vec<StubInventoryEntry>) -> Result<()> {
    let rel = abs_path
        .strip_prefix(workspace_root)
        .unwrap_or(abs_path)
        .to_string_lossy()
        .replace('\\', "/");

    // Skip macro definitions file itself.
    if rel == "crates/win32-rt/src/macros.rs" {
        return Ok(());
    }

    let raw = fs::read_to_string(abs_path)
        .with_context(|| format!("reading {}", abs_path.display()))?;
    // Match win32-rt/build.rs: ignore everything after the first `#[cfg(test)]`.
    let code = raw.split("#[cfg(test)]").next().unwrap_or(&raw);
    let lines: Vec<&str> = code.lines().collect();
    let dll = source_file_to_dll(&rel);
    let mut cur_fn = String::from("<module>");

    for (idx, &raw_line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let code_line = strip_line_comment(raw_line);
        if let Some(f) = parse_fn_name(code_line) {
            cur_fn = f;
        }

        if let Some(col) = find_macro_call_on_line(code_line, "stub") {
            let paren_col = col + "stub!".len();
            let args = extract_balanced_parens(&lines, idx, paren_col);
            let kind = if is_middleware_stub_dll(&dll) {
                "middleware_stub"
            } else {
                "stub"
            };
            out.push(StubInventoryEntry {
                dll: dll.clone(),
                api: cur_fn.clone(),
                kind: kind.to_string(),
                file: rel.clone(),
                line: line_no,
                detail: sanitize_detail(&args),
            });
        }

        for macro_name in ["failing", "win64_stubs"] {
            if let Some(col) = find_macro_call_on_line(code_line, macro_name) {
                let paren_col = col + macro_name.len() + 1;
                let args = extract_balanced_parens(&lines, idx, paren_col);
                let (header, list_part) = match args.split_once(';') {
                    Some((h, l)) => (h.trim(), l),
                    None => ("usize = 0, err 0", args.as_str()),
                };
                for raw_ident in list_part.split(',') {
                    let ident: String = raw_ident
                        .trim()
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect();
                    if ident.is_empty() {
                        continue;
                    }
                    out.push(StubInventoryEntry {
                        dll: dll.clone(),
                        api: ident,
                        kind: "failing".to_string(),
                        file: rel.clone(),
                        line: line_no,
                        detail: sanitize_detail(header),
                    });
                }
            }
        }
    }

    Ok(())
}

/// Scan every non-test `.rs` file under `crates/*/src/` for live `stub!`, `failing!`,
/// and `win64_stubs!` invocations.
pub fn scan_workspace_stubs(workspace_root: &Path) -> Result<Vec<StubInventoryEntry>> {
    let crates_dir = workspace_root.join("crates");
    let mut files = Vec::new();
    if crates_dir.is_dir() {
        let mut crate_dirs: Vec<PathBuf> = fs::read_dir(&crates_dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();
        crate_dirs.sort();
        for cdir in crate_dirs {
            collect_rs_files(&cdir.join("src"), &mut files)?;
        }
    }
    let mut entries = Vec::new();
    for f in files {
        scan_single_file(workspace_root, &f, &mut entries)?;
    }
    entries.sort_by(|a, b| {
        a.dll
            .cmp(&b.dll)
            .then_with(|| a.api.cmp(&b.api))
            .then_with(|| a.kind.cmp(&b.kind))
            .then_with(|| a.file.cmp(&b.file))
            .then_with(|| a.line.cmp(&b.line))
    });
    Ok(entries)
}

/// Format the canonical `db/stub_inventory.tsv`.
pub fn format_inventory_tsv(entries: &[StubInventoryEntry]) -> String {
    let mut out = String::from("dll\tapi\tkind\tfile\tline\tdetail\n");
    for e in entries {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}",
            e.dll, e.api, e.kind, e.file, e.line, e.detail
        );
    }
    out
}

/// Parse an existing `db/stub_inventory.tsv`.
pub fn parse_inventory_tsv(content: &str) -> Vec<StubInventoryEntry> {
    let mut entries = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        if idx == 0 && line.starts_with("dll\t") {
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 5 {
            continue;
        }
        entries.push(StubInventoryEntry {
            dll: cols[0].to_string(),
            api: cols[1].to_string(),
            kind: cols[2].to_string(),
            file: cols[3].to_string(),
            line: cols[4].parse().unwrap_or(0),
            detail: cols.get(5).copied().unwrap_or("").to_string(),
        });
    }
    entries
}

/// Identify any newly added `stub!`/`failing!`/`win64_stubs!` sites in `current`
/// compared to the checked-in `baseline`.
///
/// Identity is `(dll, api, kind, file)` with multiplicity so pure line-number shifts
/// from unrelated edits do not trigger false positives, while adding a new site
/// (even in a function that already had one) is always caught.
pub fn find_new_stubs(
    baseline: &[StubInventoryEntry],
    current: &[StubInventoryEntry],
) -> Vec<StubInventoryEntry> {
    type Key<'a> = (&'a str, &'a str, &'a str, &'a str);
    let mut base_counts: BTreeMap<Key<'_>, usize> = BTreeMap::new();
    for e in baseline {
        *base_counts
            .entry((&e.dll, &e.api, &e.kind, &e.file))
            .or_insert(0) += 1;
    }
    let mut seen_counts: BTreeMap<Key<'_>, usize> = BTreeMap::new();
    let mut new_sites = Vec::new();
    for e in current {
        let key: Key<'_> = (&e.dll, &e.api, &e.kind, &e.file);
        let c = seen_counts.entry(key).or_insert(0);
        *c += 1;
        let allowed = base_counts.get(&key).copied().unwrap_or(0);
        if *c > allowed {
            new_sites.push(e.clone());
        }
    }
    new_sites
}

/// Verify that `current` introduces zero new `stub!`/`failing!`/`win64_stubs!` sites
/// relative to `baseline_path` (`db/stub_inventory.tsv`).
pub fn check_no_new_stubs_against_file(
    baseline_path: &Path,
    current: &[StubInventoryEntry],
) -> Result<()> {
    let Ok(raw) = fs::read_to_string(baseline_path) else {
        return Ok(());
    };
    let baseline = parse_inventory_tsv(&raw);
    let added = find_new_stubs(&baseline, current);
    if added.is_empty() {
        return Ok(());
    }
    let mut msg = format!(
        "zero-stub policy violation: {} new stub!/failing! site(s) detected beyond {}:\n",
        added.len(),
        baseline_path.display()
    );
    for e in &added {
        let _ = writeln!(
            msg,
            "  + {}:{}  [{}]  {}!{}  ({})",
            e.file, e.line, e.kind, e.dll, e.api, e.detail
        );
    }
    msg.push_str(
        "Implement the API cleanly or document the gap in docs/stub-backlog.d/ instead of adding new stub!/failing! sites.",
    );
    bail!("{msg}");
}

// ---------------------------------------------------------------------------
// Backlog documentation generator (`docs/stub-backlog.md`)
// ---------------------------------------------------------------------------

/// Classify why a remaining stub exists and what the concrete next step is.
pub fn stub_reason_and_next_step(entry: &StubInventoryEntry) -> (&'static str, &'static str) {
    match entry.dll.as_str() {
        "oo2core_9_win64.dll" => (
            "Proprietary RAD Oodle compression (cannot be clean-room reimplemented legally without a license)",
            "Always run the game's shipping oo2core_*_win64.dll PE via x86-runtime ARM64 JIT; keep this honest fallback returning 0 only when no shipping DLL exists.",
        ),
        "sl.interposer.dll" => (
            "Proprietary NVIDIA Streamline plugin host (requires NVIDIA GPU + signed NGX plugins)",
            "Keep honest eErrorFeatureNotSupported reporting; route DLSS/upscaling requests to MetalFX via nvngx_dlss/libxess/FFX bridges.",
        ),
        "physx_64.dll" | "physxcooking_64.dll" => (
            "NVIDIA PhysX 4.x/5.x C++ ABI (vtable layout varies by build; shipping DLL runs natively under ARM64 JIT)",
            "Run shipping PhysX_64.dll / PhysXCooking_64.dll under ARM64 JIT via [middleware.physx] mode = \"guest\".",
        ),
        "physx3common_x64.dll" => (
            "PhysX 3.4 geometry/mesh query path without cooked mesh payload",
            "Prefer shipping PhysX3*.dll under ARM64 JIT, or decode cooked BVH/triangle mesh payloads in physx3common_x64.",
        ),
        "nvngx_dlss.dll" => (
            "Proprietary NVIDIA NGX DLSS / Vulkan NGX extension",
            "Route D3D12 NGX features through MetalFX temporal/spatial scalers; implement Vulkan NGX -> MetalFX or MoltenVK bridge when needed.",
        ),
        "amd_fidelityfx_loader_dx12.dll" => (
            "AMD FidelityFX Frame Generation / unsupported FFX context variant",
            "Keep upscaler contexts routed to MetalFX; report FFX_API_RETURN_NO_PROVIDER honestly for frame-gen until Metal 4 frame interpolation is wired.",
        ),
        "amd_ags_x64.dll" => (
            "AMD GPU Services vendor extension on non-D3D12 command list",
            "Forward recognized D3D12 debug markers to Metal command-buffer labels; return failure on foreign pointers.",
        ),
        "libxess.dll" | "libxess_dx11.dll" => (
            "Intel XeSS debug dump / Intel GPU crash-dump vendor extension (INTC_D3D12_*)",
            "Keep XeSS execute routed through MetalFX; leave proprietary Intel GPA/crash-dump callbacks as documented unsupported hooks.",
        ),
        "zstd.dll" => (
            "Open-source Zstandard compression fallback (no native zstd crate linked yet)",
            "Run shipping zstd.dll via ARM64 JIT, or add a clean-room / zstd-safe native backend with unit round-trip tests.",
        ),
        "vulkan_1.dll" => (
            "Proprietary NVIDIA Reflex Low Latency Vulkan extension (NvLL_VK_*)",
            "Return NVLL_VK_NO_IMPLEMENTATION honestly on Apple Silicon, or map sleep/marker semantics to Metal frame pacing.",
        ),
        "d3d11.dll" => (
            "Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop)",
            "Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them.",
        ),
        "d3d12.dll" => (
            "D3D12 root-signature deserializer COM object not yet implemented",
            "Parse DXBC/RTS0 root-signature blobs in win32-rt/src/d3d12 and expose ID3D12RootSignatureDeserializer / VersionedDeserializer.",
        ),
        "dxgi.dll" => (
            "Unsupported Windows-only DXGI path (WARP software rasterizer, UWP CoreWindow, exclusive fullscreen ownership, event notifications)",
            "Implement VSync-synchronized WaitForVBlank via CVDisplayLink/CADisplayLink and shared-handle LUID lookup; keep WARP/CoreWindow honestly unsupported.",
        ),
        "mmdevapi.dll" => (
            "Optional WASAPI mode/service (exclusive mode, loopback capture, hardware offload, unknown COM IID)",
            "Add CoreAudio loopback capture / tap support when a game requests AUDCLNT_STREAMFLAGS_LOOPBACK; keep exclusive/offload honestly rejected.",
        ),
        "kernel32.dll" => (
            "Partial Windows kernel/file/memory/locale/job/threadpool sub-feature",
            "Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives.",
        ),
        "ntdll.dll" => (
            "Unimplemented NT info class, TxF transaction API, or direct kernel-to-user dispatcher export",
            "Add missing Process/Thread/System information classes on demand; TxF transactions and direct KiUser* calls remain honestly unsupported.",
        ),
        "advapi32.dll" => (
            "Legacy CryptoAPI (CAPI) key/sign/encrypt or uncommon token information class",
            "Back remaining CAPI key/cipher calls with CommonCrypto/ring and implement missing TOKEN_INFORMATION_CLASS variants.",
        ),
        "bcrypt.dll" => (
            "Unimplemented CNG algorithm, key-wrap blob, or padding mode",
            "Add remaining CNG algorithms/padding modes (OAEP/PSS/ECDH/3DES) backed by CommonCrypto/ring as games import them.",
        ),
        "crypt32.dll" => (
            "Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property",
            "Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring.",
        ),
        "secur32.dll" => (
            "Server-side Schannel (INBOUND) or uncommon context/credential attribute",
            "Add server-side TLS or missing SECPKG_ATTR_* queries if a game hosts local TLS sockets.",
        ),
        "wintrust.dll" => (
            "Unimplemented WinVerifyTrust action/choice or TPM Base Services (TBS)",
            "Keep TPM TBS returning TBS_E_TPM_NOT_FOUND honestly; extend WinVerifyTrust for additional catalog/blob choices as needed.",
        ),
        "ws2_32.dll" => (
            "Uncommon Winsock2 async/QOS/control-message API or cross-process socket duplication",
            "Implement WSARecvMsg/WSASendMsg via POSIX recvmsg/sendmsg and remaining WSAIoctl codes as games use them.",
        ),
        "dbghelp.dll" => (
            "Unimplemented DbgHelp stack walk or minidump writer",
            "Wire StackWalk64 to win32-rt's .pdata unwinder and write valid MDMP headers for MiniDumpWriteDump.",
        ),
        "user32.dll" => (
            "Uncommon Win32 window/metric/SPI/hotkey/cursor/display-config sub-feature",
            "Fill in remaining GetSystemMetrics / SystemParametersInfo / DisplayConfig cases and custom bitmap NSCursor creation.",
        ),
        "ucrtbase.dll" => (
            "Uncommon CRT locale, multibyte codepage, UTF-16 stream encoding, or interactive console/shell call",
            "Add UTF-16 FILE stream translation and locale/codepage tables; keep system() safely sandboxed/refused.",
        ),
        "ole32.dll" | "oleaut32.dll" => (
            "Unimplemented WinRT IPropertyValue scalar/array type or VARIANT/SAFEARRAY coercion",
            "Implement remaining IPropertyValue* types and VariantChangeType/SafeArrayDestroy coercions.",
        ),
        _ => (
            "Partial or unimplemented Windows/middleware sub-feature",
            "Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.",
        ),
    }
}

/// Generate the full contents of `docs/stub-backlog.md`, indexing both the checked-in
/// static stub inventory (`db/stub_inventory.tsv`) and every per-thread markdown file
/// in `docs/stub-backlog.d/`.
pub fn render_stub_backlog_md(
    workspace_root: &Path,
    entries: &[StubInventoryEntry],
) -> Result<String> {
    let mut out = String::new();
    let total = entries.len();
    let mut by_kind: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_dll: BTreeMap<&str, Vec<&StubInventoryEntry>> = BTreeMap::new();
    for e in entries {
        *by_kind.entry(e.kind.as_str()).or_insert(0) += 1;
        by_dll.entry(e.dll.as_str()).or_default().push(e);
    }

    let _ = writeln!(out, "# Stub Backlog & Zero-Stub Tracker");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "This index is generated from the checked-in static inventory [`db/stub_inventory.tsv`](../db/stub_inventory.tsv)"
    );
    let _ = writeln!(
        out,
        "and aggregates every per-thread backlog file placed in [`docs/stub-backlog.d/`](stub-backlog.d/)."
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "> **Zero-stub policy:** Do not add new `stub!()`, `failing!()`, or `win64_stubs!()` sites anywhere in `crates/`."
    );
    let _ = writeln!(
        out,
        "> Both `cargo opsdb -- doctor` and `.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh` fail when a new site appears."
    );
    let _ = writeln!(
        out,
        "> Regenerate this file and `db/stub_inventory.tsv` after eliminating stubs with `cargo opsdb -- stub-sync`."
    );
    let _ = writeln!(out);

    let _ = writeln!(out, "## Summary");
    let _ = writeln!(out);
    let _ = writeln!(out, "| Metric | Count |");
    let _ = writeln!(out, "|---|---:|");
    let _ = writeln!(out, "| Total remaining stub sites | **{total}** |");
    let _ = writeln!(
        out,
        "| `stub` (partial Win32/OS/D3D/UCRT branches) | {} |",
        by_kind.get("stub").copied().unwrap_or(0)
    );
    let _ = writeln!(
        out,
        "| `failing` (honest-failing macro exports) | {} |",
        by_kind.get("failing").copied().unwrap_or(0)
    );
    let _ = writeln!(
        out,
        "| `middleware_stub` (third-party/engine middleware fallbacks) | {} |",
        by_kind.get("middleware_stub").copied().unwrap_or(0)
    );
    let _ = writeln!(out, "| DLLs with remaining stubs | {} |", by_dll.len());
    let _ = writeln!(out);

    // Collect per-thread backlog files in docs/stub-backlog.d/
    let backlog_d = workspace_root.join("docs/stub-backlog.d");
    let mut thread_files: Vec<PathBuf> = Vec::new();
    if backlog_d.is_dir() {
        for ent in fs::read_dir(&backlog_d)? {
            let p = ent?.path();
            if p.is_file()
                && p.extension().and_then(|s| s.to_str()) == Some("md")
                && p.file_name().and_then(|s| s.to_str()) != Some("README.md")
            {
                thread_files.push(p);
            }
        }
        thread_files.sort();
    }

    let _ = writeln!(out, "## Per-Thread Backlog Notes (`docs/stub-backlog.d/`)");
    let _ = writeln!(out);
    if thread_files.is_empty() {
        let _ = writeln!(
            out,
            "_No per-thread backlog files present yet. Parallel threads should add `docs/stub-backlog.d/<thread-or-topic>.md` (see [`docs/stub-backlog.d/README.md`](stub-backlog.d/README.md)) and run `cargo opsdb -- stub-backlog --write`._"
        );
        let _ = writeln!(out);
    } else {
        let _ = writeln!(out, "| File | Summary |");
        let _ = writeln!(out, "|---|---|");
        for tf in &thread_files {
            let fname = tf.file_name().unwrap().to_string_lossy();
            let raw = fs::read_to_string(tf).unwrap_or_default();
            let heading = raw
                .lines()
                .find(|l| !l.trim().is_empty())
                .unwrap_or("(empty)")
                .trim_start_matches('#')
                .trim();
            let _ = writeln!(out, "| [`{fname}`](stub-backlog.d/{fname}) | {heading} |");
        }
        let _ = writeln!(out);
        for tf in &thread_files {
            let fname = tf.file_name().unwrap().to_string_lossy();
            let raw = fs::read_to_string(tf).unwrap_or_default();
            let _ = writeln!(out, "### From `docs/stub-backlog.d/{fname}`");
            let _ = writeln!(out);
            out.push_str(raw.trim());
            let _ = writeln!(out);
            let _ = writeln!(out);
        }
    }

    // Proprietary / non-reimplementable middleware section
    let proprietary_dlls: BTreeSet<&str> = [
        "oo2core_9_win64.dll",
        "sl.interposer.dll",
        "physx_64.dll",
        "physxcooking_64.dll",
        "nvngx_dlss.dll",
        "amd_fidelityfx_loader_dx12.dll",
        "amd_ags_x64.dll",
        "libxess.dll",
        "libxess_dx11.dll",
    ]
    .into_iter()
    .collect();

    let _ = writeln!(out, "## Proprietary & Vendor Middleware (Legal / Architectural Limits)");
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "These modules wrap proprietary third-party SDKs or GPU-vendor-specific extensions. Where a game ships its own Windows x86-64 PE DLL (e.g., Oodle `oo2core_*_win64.dll`, PhysX 4/5 `PhysX_64.dll`, `zstd.dll`), `port-runner` executes the real shipping PE through the ARM64 JIT rather than overriding it."
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "| DLL | Remaining Sites | Reason | Next Step |");
    let _ = writeln!(out, "|---|---:|---|---|");
    for &dll in &proprietary_dlls {
        if let Some(list) = by_dll.get(dll) {
            let (reason, next) = stub_reason_and_next_step(list[0]);
            let _ = writeln!(out, "| `{dll}` | {} | {reason} | {next} |", list.len());
        }
    }
    let _ = writeln!(out);

    // Remaining stubs grouped by DLL (ranked by count descending)
    let mut ranked_dlls: Vec<(&str, &Vec<&StubInventoryEntry>)> =
        by_dll.iter().map(|(&d, v)| (d, v)).collect();
    ranked_dlls.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(b.0)));

    let _ = writeln!(out, "## Remaining Stubs by DLL");
    let _ = writeln!(out);
    let _ = writeln!(out, "| DLL | Count | Reason | Next Step |");
    let _ = writeln!(out, "|---|---:|---|---|");
    for (dll, list) in &ranked_dlls {
        let (reason, next) = stub_reason_and_next_step(list[0]);
        let _ = writeln!(out, "| `{dll}` | {} | {reason} | {next} |", list.len());
    }
    let _ = writeln!(out);

    for (dll, list) in &ranked_dlls {
        let (reason, next) = stub_reason_and_next_step(list[0]);
        let _ = writeln!(out, "### `{dll}` ({})", list.len());
        let _ = writeln!(out);
        let _ = writeln!(out, "- **Reason:** {reason}");
        let _ = writeln!(out, "- **Next step:** {next}");
        let _ = writeln!(out);
        let _ = writeln!(out, "| API | Kind | Location | Detail |");
        let _ = writeln!(out, "|---|---|---|---|");
        for e in *list {
            let escaped_detail = e.detail.replace('|', "\\|");
            let _ = writeln!(
                out,
                "| `{}` | `{}` | `{}:{}` | `{}` |",
                e.api, e.kind, e.file, e.line, escaped_detail
            );
        }
        let _ = writeln!(out);
    }

    Ok(out)
}

// ---------------------------------------------------------------------------
// Runtime stub snapshot JSON parser & diff (`cargo opsdb -- stub-diff`)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStubEntry {
    pub dll: String,
    pub api: String,
    pub kind: String,
    pub hits: u64,
    pub first_frame: u64,
    pub first_guest_caller: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuntimeStubSnapshot {
    pub total_hits: u64,
    pub unique_sites: usize,
    pub entries: Vec<RuntimeStubEntry>,
}

/// Minimal RFC 8259 JSON value used across `stub-diff` and `report-diff`.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<JsonValue>),
    Obj(BTreeMap<String, JsonValue>),
}

impl JsonValue {
    pub fn as_obj(&self) -> Option<&BTreeMap<String, JsonValue>> {
        match self {
            Self::Obj(m) => Some(m),
            _ => None,
        }
    }
    pub fn as_arr(&self) -> Option<&[JsonValue]> {
        match self {
            Self::Arr(v) => Some(v),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Num(n) if *n >= 0.0 => Some(*n as u64),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

pub fn parse_json(input: &str) -> Result<JsonValue> {
    struct P<'a> {
        b: &'a [u8],
        i: usize,
    }
    impl<'a> P<'a> {
        fn ws(&mut self) {
            while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\n' | b'\r' | b'\t') {
                self.i += 1;
            }
        }
        fn peek(&mut self) -> Option<u8> {
            self.ws();
            self.b.get(self.i).copied()
        }
        fn expect(&mut self, ch: u8) -> Result<()> {
            self.ws();
            if self.b.get(self.i) == Some(&ch) {
                self.i += 1;
                Ok(())
            } else {
                bail!("JSON syntax error: expected '{}' at byte {}", ch as char, self.i)
            }
        }
        fn val(&mut self) -> Result<JsonValue> {
            match self.peek().ok_or_else(|| anyhow::anyhow!("unexpected JSON EOF"))? {
                b'{' => self.obj(),
                b'[' => self.arr(),
                b'"' => self.str().map(JsonValue::Str),
                b't' => self.lit(b"true", JsonValue::Bool(true)),
                b'f' => self.lit(b"false", JsonValue::Bool(false)),
                b'n' => self.lit(b"null", JsonValue::Null),
                b'-' | b'0'..=b'9' => self.num(),
                c => bail!("unexpected JSON byte '{}' at {}", c as char, self.i),
            }
        }
        fn lit(&mut self, pat: &[u8], out: JsonValue) -> Result<JsonValue> {
            if self.b[self.i..].starts_with(pat) {
                self.i += pat.len();
                Ok(out)
            } else {
                bail!("invalid JSON literal at byte {}", self.i)
            }
        }
        fn num(&mut self) -> Result<JsonValue> {
            let start = self.i;
            if self.b.get(self.i) == Some(&b'-') {
                self.i += 1;
            }
            while self.i < self.b.len()
                && matches!(self.b[self.i], b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-')
            {
                self.i += 1;
            }
            let s = std::str::from_utf8(&self.b[start..self.i])?;
            let n: f64 = s.parse().with_context(|| format!("invalid JSON number {s:?}"))?;
            Ok(JsonValue::Num(n))
        }
        fn str(&mut self) -> Result<String> {
            self.expect(b'"')?;
            let mut out = String::new();
            while self.i < self.b.len() {
                let c = self.b[self.i];
                self.i += 1;
                if c == b'"' {
                    return Ok(out);
                }
                if c == b'\\' {
                    let esc = *self
                        .b
                        .get(self.i)
                        .ok_or_else(|| anyhow::anyhow!("unterminated JSON escape"))?;
                    self.i += 1;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\x08'),
                        b'f' => out.push('\x0c'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            if self.i + 4 > self.b.len() {
                                bail!("truncated \\u escape");
                            }
                            let hex = std::str::from_utf8(&self.b[self.i..self.i + 4])?;
                            let code = u32::from_str_radix(hex, 16)?;
                            self.i += 4;
                            if let Some(ch) = char::from_u32(code) {
                                out.push(ch);
                            }
                        }
                        other => out.push(other as char),
                    }
                } else {
                    let start = self.i - 1;
                    while self.i < self.b.len() && (self.b[self.i] & 0xC0) == 0x80 {
                        self.i += 1;
                    }
                    out.push_str(std::str::from_utf8(&self.b[start..self.i])?);
                }
            }
            bail!("unterminated JSON string")
        }
        fn arr(&mut self) -> Result<JsonValue> {
            self.expect(b'[')?;
            let mut v = Vec::new();
            if self.peek() == Some(b']') {
                self.i += 1;
                return Ok(JsonValue::Arr(v));
            }
            loop {
                v.push(self.val()?);
                match self.peek() {
                    Some(b',') => self.i += 1,
                    Some(b']') => {
                        self.i += 1;
                        return Ok(JsonValue::Arr(v));
                    }
                    _ => bail!("expected ',' or ']' at byte {}", self.i),
                }
            }
        }
        fn obj(&mut self) -> Result<JsonValue> {
            self.expect(b'{')?;
            let mut m = BTreeMap::new();
            if self.peek() == Some(b'}') {
                self.i += 1;
                return Ok(JsonValue::Obj(m));
            }
            loop {
                let k = self.str()?;
                self.expect(b':')?;
                let v = self.val()?;
                m.insert(k, v);
                match self.peek() {
                    Some(b',') => self.i += 1,
                    Some(b'}') => {
                        self.i += 1;
                        return Ok(JsonValue::Obj(m));
                    }
                    _ => bail!("expected ',' or '}}' at byte {}", self.i),
                }
            }
        }
    }
    let mut p = P {
        b: input.as_bytes(),
        i: 0,
    };
    p.val()
}

/// Parse a runtime stub snapshot from either a standalone `StubSnapshot` JSON
/// or a full `--report-json` boot report (extracting its `"stubs"` field).
pub fn parse_runtime_stub_snapshot(input: &str) -> Result<RuntimeStubSnapshot> {
    let root = parse_json(input)?;
    let obj = root
        .as_obj()
        .ok_or_else(|| anyhow::anyhow!("expected top-level JSON object"))?;
    let stubs_obj = if let Some(nested) = obj.get("stubs").and_then(|v| v.as_obj()) {
        nested
    } else {
        obj
    };
    let arr = stubs_obj
        .get("entries")
        .and_then(|v| v.as_arr())
        .ok_or_else(|| anyhow::anyhow!("JSON missing 'entries' array in stub snapshot"))?;
    let mut entries = Vec::with_capacity(arr.len());
    for item in arr {
        let o = item
            .as_obj()
            .ok_or_else(|| anyhow::anyhow!("expected object in stub entries"))?;
        entries.push(RuntimeStubEntry {
            dll: o.get("dll").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            api: o.get("api").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            kind: o
                .get("kind")
                .and_then(|v| v.as_str())
                .unwrap_or("stub")
                .to_string(),
            hits: o.get("hits").and_then(|v| v.as_u64()).unwrap_or(0),
            first_frame: o.get("first_frame").and_then(|v| v.as_u64()).unwrap_or(0),
            first_guest_caller: o
                .get("first_guest_caller")
                .and_then(|v| v.as_str())
                .unwrap_or("<host>")
                .to_string(),
        });
    }
    let total_hits = stubs_obj
        .get("total_hits")
        .and_then(|v| v.as_u64())
        .unwrap_or_else(|| entries.iter().map(|e| e.hits).sum());
    let unique_sites = stubs_obj
        .get("unique_sites")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize)
        .unwrap_or(entries.len());
    Ok(RuntimeStubSnapshot {
        total_hits,
        unique_sites,
        entries,
    })
}

/// Format a human-readable diff between two runtime stub snapshots.
pub fn format_runtime_stub_diff(
    before: &RuntimeStubSnapshot,
    after: &RuntimeStubSnapshot,
) -> String {
    type Key = (String, String, String);
    let agg = |snap: &RuntimeStubSnapshot| -> BTreeMap<Key, (u64, u64, String)> {
        let mut m: BTreeMap<Key, (u64, u64, String)> = BTreeMap::new();
        for e in &snap.entries {
            let k = (e.dll.clone(), e.api.clone(), e.kind.clone());
            m.entry(k)
                .and_modify(|(h, f, c)| {
                    *h = h.saturating_add(e.hits);
                    if e.first_frame < *f {
                        *f = e.first_frame;
                        *c = e.first_guest_caller.clone();
                    }
                })
                .or_insert((e.hits, e.first_frame, e.first_guest_caller.clone()));
        }
        m
    };

    let b = agg(before);
    let a = agg(after);
    let mut added: Vec<(Key, u64, u64, String)> = Vec::new();
    let mut removed: Vec<(Key, u64)> = Vec::new();
    let mut changed: Vec<(Key, u64, u64)> = Vec::new();
    let mut unchanged = 0usize;

    for (k, (ah, af, ac)) in &a {
        match b.get(k) {
            None => added.push((k.clone(), *ah, *af, ac.clone())),
            Some((bh, _, _)) if bh != ah => changed.push((k.clone(), *bh, *ah)),
            Some(_) => unchanged += 1,
        }
    }
    for (k, (bh, _, _)) in &b {
        if !a.contains_key(k) {
            removed.push((k.clone(), *bh));
        }
    }

    added.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| x.0.cmp(&y.0)));
    removed.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| x.0.cmp(&y.0)));
    changed.sort_by(|x, y| {
        let dx = (x.2 as i128 - x.1 as i128).abs();
        let dy = (y.2 as i128 - y.1 as i128).abs();
        dy.cmp(&dx).then_with(|| x.0.cmp(&y.0))
    });

    let mut out = String::new();
    let hit_delta = after.total_hits as i128 - before.total_hits as i128;
    let api_delta = a.len() as isize - b.len() as isize;
    let _ = writeln!(
        out,
        "=== Stub Run Diff (APIs: {} -> {} [{:+}], hits: {} -> {} [{:+}]) ===",
        b.len(),
        a.len(),
        api_delta,
        before.total_hits,
        after.total_hits,
        hit_delta
    );
    if added.is_empty() && removed.is_empty() && changed.is_empty() {
        let _ = writeln!(out, "  (no changes; {unchanged} unchanged APIs)");
        return out;
    }
    if !added.is_empty() {
        let _ = writeln!(out, "  NEW stubs hit (+{}):", added.len());
        for ((dll, api, kind), hits, frame, caller) in &added {
            let _ = writeln!(
                out,
                "    + {:>8}  {:<15}  {}!{}  (frame {}, caller {})",
                hits, kind, dll, api, frame, caller
            );
        }
    }
    if !removed.is_empty() {
        let _ = writeln!(out, "  RESOLVED stubs (-{}):", removed.len());
        for ((dll, api, kind), hits) in &removed {
            let _ = writeln!(out, "    - {:>8}  {:<15}  {}!{}", hits, kind, dll, api);
        }
    }
    if !changed.is_empty() {
        let _ = writeln!(out, "  CHANGED hit counts ({}):", changed.len());
        for ((dll, api, kind), bh, ah) in &changed {
            let d = *ah as i128 - *bh as i128;
            let _ = writeln!(
                out,
                "    ~ {:>8} -> {:>8} ({:+})  {:<15}  {}!{}",
                bh, ah, d, kind, dll, api
            );
        }
    }
    if unchanged > 0 {
        let _ = writeln!(out, "  ({unchanged} unchanged APIs)");
    }
    out
}

pub fn diff_stub_json_files(before_path: &Path, after_path: &Path) -> Result<String> {
    let b_raw = fs::read_to_string(before_path)
        .with_context(|| format!("reading {}", before_path.display()))?;
    let a_raw = fs::read_to_string(after_path)
        .with_context(|| format!("reading {}", after_path.display()))?;
    let before = parse_runtime_stub_snapshot(&b_raw)
        .with_context(|| format!("parsing {}", before_path.display()))?;
    let after = parse_runtime_stub_snapshot(&a_raw)
        .with_context(|| format!("parsing {}", after_path.display()))?;
    Ok(format_runtime_stub_diff(&before, &after))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(Path::to_path_buf)
            .expect("resolve workspace root")
    }

    #[test]
    fn scanner_finds_stubs_and_expands_failing_macros() {
        let root = workspace_root();
        let entries = scan_workspace_stubs(&root).expect("scan stubs");
        assert!(!entries.is_empty(), "expected non-empty stub inventory");
        // Middleware stubs classified properly
        assert!(
            entries
                .iter()
                .any(|e| e.dll == "oo2core_9_win64.dll" && e.kind == "middleware_stub"),
            "expected oo2core_9_win64.dll middleware_stub"
        );
        // TSV round-trip preserves exact entries
        let tsv = format_inventory_tsv(&entries);
        let parsed = parse_inventory_tsv(&tsv);
        assert_eq!(parsed, entries);
    }

    #[test]
    fn find_new_stubs_ignores_line_shifts_and_flags_additions() {
        let base = vec![StubInventoryEntry {
            dll: "user32.dll".into(),
            api: "GetSystemMetrics".into(),
            kind: "stub".into(),
            file: "crates/win32-rt/src/user32/misc.rs".into(),
            line: 234,
            detail: "x".into(),
        }];
        // Pure line shift: zero new stubs
        let shifted = vec![StubInventoryEntry {
            line: 260,
            ..base[0].clone()
        }];
        assert!(find_new_stubs(&base, &shifted).is_empty());

        // Second stub! added in same function: flagged!
        let mut doubled = shifted.clone();
        doubled.push(StubInventoryEntry {
            line: 270,
            ..base[0].clone()
        });
        let added = find_new_stubs(&base, &doubled);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0].line, 270);

        // Brand-new failing! export: flagged!
        let mut with_new = shifted;
        with_new.push(StubInventoryEntry {
            dll: "kernel32.dll".into(),
            api: "NewStubApi".into(),
            kind: "failing".into(),
            file: "crates/win32-rt/src/kernel32/misc.rs".into(),
            line: 10,
            detail: "BOOL = FALSE".into(),
        });
        let added2 = find_new_stubs(&base, &with_new);
        assert_eq!(added2.len(), 1);
        assert_eq!(added2[0].api, "NewStubApi");
    }
}
