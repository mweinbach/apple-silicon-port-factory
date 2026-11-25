//! Drift detection and synchronization for `port-factory`'s canonical TSV databases:
//! - `db/ops_x86_to_arm64.tsv`
//! - `db/win32_apis.tsv`
//! - `db/shader_formats.tsv`
//! - `db/game_profiles.tsv`
//!
//! By default (`fix = false`), [`check_or_fix_db_tsvs`] is strictly read-only: it compares each
//! committed `db/*.tsv` against the freshly generated canonical output and reports a concise diff
//! summary so `cargo opsdb -- doctor` exits non-zero on any drift. Passing `fix = true`
//! (`cargo opsdb -- doctor --fix`) rewrites only the missing or drifted TSV files in place.

use crate::game_profiles::{audit_all_game_profiles, format_game_profiles_tsv, TOP_GAME_PROFILES};
use crate::ops_db::{catalog_stats, format_tsv as format_ops_tsv};
use crate::stub_inventory::{
    check_no_new_stubs_against_file, format_inventory_tsv, scan_workspace_stubs,
};
use crate::win32_db::{catalog_rust_items, ImplKind, Win32Database};
use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Sync status of a single `db/*.tsv` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TsvSyncStatus {
    InSync {
        line_count: usize,
    },
    Missing {
        expected_lines: usize,
    },
    Drifted {
        disk_lines: usize,
        expected_lines: usize,
        added_lines: usize,
        removed_lines: usize,
        first_diff_line: usize,
        sample_diffs: Vec<String>,
    },
    Fixed {
        line_count: usize,
    },
}

impl TsvSyncStatus {
    pub fn is_drift(&self) -> bool {
        matches!(self, Self::Missing { .. } | Self::Drifted { .. })
    }
}

/// Result of checking (or fixing) one canonical `db/*.tsv` database.
#[derive(Debug, Clone)]
pub struct TsvCheckItem {
    pub file_name: &'static str,
    pub abs_path: PathBuf,
    pub label: &'static str,
    pub detail: String,
    pub fix_cmd: &'static str,
    pub status: TsvSyncStatus,
}

/// Aggregated report across the canonical `db/*.tsv` databases.
#[derive(Debug, Clone)]
pub struct DoctorDbReport {
    pub items: Vec<TsvCheckItem>,
    pub catalog_errors: Vec<String>,
}

impl DoctorDbReport {
    pub fn drifted_items(&self) -> Vec<&TsvCheckItem> {
        self.items.iter().filter(|i| i.status.is_drift()).collect()
    }

    pub fn has_drift(&self) -> bool {
        !self.catalog_errors.is_empty() || self.items.iter().any(|i| i.status.is_drift())
    }

    /// Format a clear, human-readable status and mismatch summary for stdout.
    pub fn format_summary(&self) -> String {
        let mut out = String::new();
        for item in &self.items {
            match &item.status {
                TsvSyncStatus::InSync { line_count } => {
                    let _ = writeln!(
                        out,
                        "  [OK]      {:16} : {} -> {} ({line_count} lines, in sync)",
                        item.label,
                        item.detail,
                        item.abs_path.display()
                    );
                }
                TsvSyncStatus::Fixed { line_count } => {
                    let _ = writeln!(
                        out,
                        "  [FIXED]   {:16} : {} -> {} ({line_count} lines updated)",
                        item.label,
                        item.detail,
                        item.abs_path.display()
                    );
                }
                TsvSyncStatus::Missing { expected_lines } => {
                    let _ = writeln!(
                        out,
                        "  [MISSING] {:16} : {} is missing on disk (expected {expected_lines} lines; fix with `{}`)",
                        item.label,
                        item.abs_path.display(),
                        item.fix_cmd
                    );
                }
                TsvSyncStatus::Drifted {
                    disk_lines,
                    expected_lines,
                    added_lines,
                    removed_lines,
                    first_diff_line,
                    sample_diffs,
                } => {
                    let _ = writeln!(
                        out,
                        "  [DRIFT]   {:16} : {} out of sync (disk={disk_lines} lines, expected={expected_lines} lines, +{added_lines}/-{removed_lines}, first diff at line {first_diff_line}; fix with `{}`)",
                        item.label,
                        item.abs_path.display(),
                        item.fix_cmd
                    );
                    for sample in sample_diffs {
                        let _ = writeln!(out, "              {sample}");
                    }
                }
            }
        }
        for error in &self.catalog_errors {
            let _ = writeln!(out, "  [DRIFT]   {error}");
        }
        out
    }

    /// Returns `Ok(())` when all TSV files match their canonical generated state, or an `Err`
    /// enumerating every missing/drifted file so CLI callers exit non-zero.
    pub fn ensure_clean(&self) -> Result<()> {
        if !self.catalog_errors.is_empty() {
            bail!("{}", self.catalog_errors.join("\n"));
        }
        let drifted = self.drifted_items();
        if drifted.is_empty() {
            return Ok(());
        }
        let names: Vec<String> = drifted
            .iter()
            .map(|item| match &item.status {
                TsvSyncStatus::Missing { .. } => format!("db/{} (missing)", item.file_name),
                TsvSyncStatus::Drifted {
                    added_lines,
                    removed_lines,
                    ..
                } => format!("db/{} (+{added_lines}/-{removed_lines})", item.file_name),
                _ => format!("db/{}", item.file_name),
            })
            .collect();
        bail!(
            "database drift detected in {} file(s): {}. Run `cargo opsdb -- doctor --fix` to sync.",
            drifted.len(),
            names.join(", ")
        );
    }
}

/// Render `db/shader_formats.tsv` using `dxil_metal::formats::sync_shader_formats_tsv` to guarantee
/// exact byte-for-byte parity with `dxil-metal`'s own serializer.
pub fn render_shader_formats_tsv() -> Result<String> {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    let tmp = std::env::temp_dir().join(format!(
        "port_factory_shader_formats_{}_{}.tsv",
        std::process::id(),
        seq
    ));
    let res = (|| -> Result<String> {
        dxil_metal::formats::sync_shader_formats_tsv(&tmp)
            .with_context(|| format!("rendering shader_formats.tsv via {}", tmp.display()))?;
        fs::read_to_string(&tmp)
            .with_context(|| format!("reading rendered shader_formats.tsv from {}", tmp.display()))
    })();
    let _ = fs::remove_file(&tmp);
    res
}

fn truncate_preview(line: &str, max_chars: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    if chars.len() <= max_chars {
        line.to_string()
    } else {
        let mut s: String = chars[..max_chars.saturating_sub(3)].iter().collect();
        s.push_str("...");
        s
    }
}

fn compute_tsv_diff(disk: &str, expected: &str) -> TsvSyncStatus {
    let disk_lines: Vec<&str> = disk.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();

    let mut disk_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for &l in &disk_lines {
        *disk_counts.entry(l).or_insert(0) += 1;
    }
    let mut exp_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for &l in &expected_lines {
        *exp_counts.entry(l).or_insert(0) += 1;
    }

    let mut added_lines = 0usize;
    let mut removed_lines = 0usize;
    let mut sample_diffs = Vec::new();
    const MAX_SAMPLES: usize = 6;

    for (&line, &ec) in &exp_counts {
        let dc = disk_counts.get(line).copied().unwrap_or(0);
        if ec > dc {
            added_lines += ec - dc;
            if sample_diffs.len() < MAX_SAMPLES {
                sample_diffs.push(format!("+ {}", truncate_preview(line, 110)));
            }
        }
    }
    for (&line, &dc) in &disk_counts {
        let ec = exp_counts.get(line).copied().unwrap_or(0);
        if dc > ec {
            removed_lines += dc - ec;
            if sample_diffs.len() < MAX_SAMPLES {
                sample_diffs.push(format!("- {}", truncate_preview(line, 110)));
            }
        }
    }

    let first_diff_line = disk_lines
        .iter()
        .zip(expected_lines.iter())
        .position(|(a, b)| a != b)
        .map(|i| i + 1)
        .unwrap_or_else(|| disk_lines.len().min(expected_lines.len()) + 1);

    if sample_diffs.is_empty() {
        // Same line multiset but different ordering or trailing newline.
        let disk_sample = disk_lines
            .get(first_diff_line.saturating_sub(1))
            .copied()
            .unwrap_or("<EOF>");
        let exp_sample = expected_lines
            .get(first_diff_line.saturating_sub(1))
            .copied()
            .unwrap_or("<EOF>");
        sample_diffs.push(format!(
            "line {first_diff_line} differs: disk={} vs expected={}",
            truncate_preview(disk_sample, 50),
            truncate_preview(exp_sample, 50)
        ));
    }

    TsvSyncStatus::Drifted {
        disk_lines: disk_lines.len(),
        expected_lines: expected_lines.len(),
        added_lines,
        removed_lines,
        first_diff_line,
        sample_diffs,
    }
}

fn check_or_fix_single(
    file_name: &'static str,
    abs_path: PathBuf,
    label: &'static str,
    detail: String,
    fix_cmd: &'static str,
    expected: &str,
    fix: bool,
) -> Result<TsvCheckItem> {
    let expected_line_count = expected.lines().count();
    let status = match fs::read_to_string(&abs_path) {
        Ok(disk) if disk == expected => TsvSyncStatus::InSync {
            line_count: expected_line_count,
        },
        Ok(disk) => {
            if fix {
                if let Some(parent) = abs_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&abs_path, expected)
                    .with_context(|| format!("writing {}", abs_path.display()))?;
                TsvSyncStatus::Fixed {
                    line_count: expected_line_count,
                }
            } else {
                compute_tsv_diff(&disk, expected)
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if fix {
                if let Some(parent) = abs_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&abs_path, expected)
                    .with_context(|| format!("writing {}", abs_path.display()))?;
                TsvSyncStatus::Fixed {
                    line_count: expected_line_count,
                }
            } else {
                TsvSyncStatus::Missing {
                    expected_lines: expected_line_count,
                }
            }
        }
        Err(e) => {
            return Err(e).with_context(|| format!("reading {}", abs_path.display()));
        }
    };

    Ok(TsvCheckItem {
        file_name,
        abs_path,
        label,
        detail,
        fix_cmd,
        status,
    })
}

/// Verify source links even when the generator and the committed TSV agree on a stale path.
fn catalog_source_errors<'a>(
    root: &Path,
    source_files: impl Iterator<Item = &'a str>,
    rust_paths: impl Iterator<Item = &'a str>,
) -> Vec<String> {
    let mut errors = Vec::new();
    for file in source_files.collect::<BTreeSet<_>>() {
        let path = Path::new(file);
        if path.is_absolute()
            || path
                .components()
                .any(|c| c == std::path::Component::ParentDir)
            || !root.join(path).is_file()
        {
            errors.push(format!(
                "catalog source_file does not resolve in the workspace: {file}"
            ));
        }
    }

    // rust_path is a qualified Rust item, including cross-DLL forwarders. An
    // EXTRA_EXPORTS[decorated name] row names an element of a generated table.
    let mut modules = BTreeMap::<&str, BTreeSet<String>>::new();
    for path in rust_paths.collect::<BTreeSet<_>>() {
        // An indexed export can contain punctuation, including double colons.
        let item = path.split('[').next().unwrap_or(path);
        let Some((module, _)) = item
            .strip_prefix("win32_rt::")
            .and_then(|p| p.rsplit_once("::"))
        else {
            errors.push(format!("invalid Win32 rust_path: {path}"));
            continue;
        };
        let identifiers = modules.entry(module).or_insert_with(|| {
            let base = root
                .join("crates/win32-rt/src")
                .join(module.replace("::", "/"));
            let src = fs::read_to_string(base.with_extension("rs"))
                .or_else(|_| fs::read_to_string(base.join("mod.rs")))
                .unwrap_or_default();
            catalog_rust_items(src.split("#[cfg(test)]").next().unwrap_or(""))
        });
        let name = &path["win32_rt::".len() + module.len() + 2..];
        if !identifiers.contains(name) {
            errors.push(format!(
                "Win32 rust_path has no matching source item: {path}"
            ));
        }
    }
    errors
}

const COUNTS_START: &str = "<!-- opsdb:catalog -->";
const COUNTS_END: &str = "<!-- /opsdb:catalog -->";

fn check_status_counts(doc: &Path, counts: &str, fix: bool) -> Result<Option<String>> {
    let text = fs::read_to_string(doc).with_context(|| format!("read {}", doc.display()))?;
    let region = text.split_once(COUNTS_START).and_then(|(before, rest)| {
        rest.split_once(COUNTS_END)
            .map(|(old, after)| (before, old, after))
    });
    let Some((before, old, after)) = region else {
        return Ok(Some(
            "docs/STATUS.md is missing its opsdb:catalog markers".into(),
        ));
    };
    if [before, old, after]
        .iter()
        .any(|s| s.contains(COUNTS_START) || s.contains(COUNTS_END))
    {
        return Ok(Some(
            "docs/STATUS.md has ambiguous opsdb:catalog markers".into(),
        ));
    }
    if old == counts {
        return Ok(None);
    }
    if fix {
        fs::write(
            doc,
            format!("{before}{COUNTS_START}{counts}{COUNTS_END}{after}"),
        )?;
        Ok(None)
    } else {
        Ok(Some(
            "docs/STATUS.md catalog counts drifted; run `cargo opsdb -- doctor --fix`".into(),
        ))
    }
}

/// Check the canonical databases and the published catalog totals.
pub fn check_workspace_catalogs(root: &Path, fix: bool) -> Result<DoctorDbReport> {
    check_catalogs(
        root,
        &root.join("db"),
        Some(&root.join("docs/STATUS.md")),
        fix,
    )
}

/// Check or regenerate just the databases in the supplied directory.
pub fn check_or_fix_db_tsvs(root: &Path, db_dir: &Path, fix: bool) -> Result<DoctorDbReport> {
    check_catalogs(root, db_dir, None, fix)
}

fn check_catalogs(
    workspace_root: &Path,
    db_dir: &Path,
    status_doc: Option<&Path>,
    fix: bool,
) -> Result<DoctorDbReport> {
    let win_db = Win32Database::from_workspace_src(workspace_root)?;
    let stubs = scan_workspace_stubs(workspace_root)?;
    let mut catalog_errors = catalog_source_errors(
        workspace_root,
        crate::ops_db::OPS_CATALOG
            .iter()
            .map(|r| r.source_file)
            .chain(win_db.entries.iter().map(|e| e.source_file.as_str()))
            .chain(stubs.iter().map(|e| e.file.as_str())),
        win_db.entries.iter().map(|e| e.rust_path.as_str()),
    );
    if fix && !catalog_errors.is_empty() {
        return Ok(DoctorDbReport {
            items: Vec::new(),
            catalog_errors,
        });
    }

    // 1. x86-64 -> ARM64 Ops Database
    let ops_expected = format_ops_tsv();
    let st = catalog_stats();
    let mut tiers = BTreeMap::new();
    for row in crate::ops_db::OPS_CATALOG {
        *tiers.entry(row.tier.as_str()).or_insert(0usize) += 1;
    }
    let tiers = tiers
        .iter()
        .map(|(tier, n)| format!("{n} {tier}"))
        .collect::<Vec<_>>()
        .join(", ");
    let ops_item = check_or_fix_single(
        "ops_x86_to_arm64.tsv",
        db_dir.join("ops_x86_to_arm64.tsv"),
        "Ops Database",
        format!(
            "{} families ({tiers}), {} distinct mnemonics, {} cataloged vectors",
            st.total_records, st.total_mnemonics, st.total_verified_vectors
        ),
        "cargo opsdb -- sync",
        &ops_expected,
        fix,
    )?;

    // 2. Win32 API Database
    let win_expected = win_db.format_tsv();
    let dll_count: BTreeSet<&str> = win_db.entries.iter().map(|e| e.dll.as_str()).collect();
    let win_item = check_or_fix_single(
        "win32_apis.tsv",
        db_dir.join("win32_apis.tsv"),
        "Win32 Database",
        format!(
            "{} exports across {} DLLs ({} real, {} partial, {} stub, {} failing)",
            win_db.entries.len(),
            dll_count.len(),
            win_db.count_by_impl_kind(ImplKind::Real),
            win_db.count_by_impl_kind(ImplKind::Partial),
            win_db.count_by_impl_kind(ImplKind::Stub),
            win_db.count_by_impl_kind(ImplKind::Failing),
        ),
        "cargo opsdb -- win32-sync",
        &win_expected,
        fix,
    )?;

    // 3. Standardized Shader Formats Database
    let shader_expected = render_shader_formats_tsv()?;
    let shader_item = check_or_fix_single(
        "shader_formats.tsv",
        db_dir.join("shader_formats.tsv"),
        "Shader Formats",
        format!(
            "{} standardized formats (DX12 DXIL, DX11 DXBC, Vulkan SPIR-V, WGSL, Metal)",
            dxil_metal::formats::SHADER_FORMAT_CATALOG.len()
        ),
        "cargo opsdb -- shader-formats --sync",
        &shader_expected,
        fix,
    )?;

    // 4. Top PC Games & Engines Compatibility Catalog
    let gp_expected = format_game_profiles_tsv();
    let gp_audits = audit_all_game_profiles(&win_db);
    let gp_ready = gp_audits
        .iter()
        .filter(|a| a.missing_dlls.is_empty())
        .count();
    let gp_item = check_or_fix_single(
        "game_profiles.tsv",
        db_dir.join("game_profiles.tsv"),
        "Game Profiles",
        format!(
            "{}/{} engine profiles 100% DLL-ready",
            gp_ready,
            TOP_GAME_PROFILES.len()
        ),
        "cargo opsdb -- game-profiles --sync",
        &gp_expected,
        fix,
    )?;

    // 5. Static Stub Inventory + zero-new-stub regression guard
    let stub_tsv_path = db_dir.join("stub_inventory.tsv");
    // Always enforce the zero-new-stub policy (even in --fix mode!) so `doctor --fix`
    // can never silently bless newly added `stub!`/`failing!` sites.
    check_no_new_stubs_against_file(&stub_tsv_path, &stubs)?;
    let stub_expected = format_inventory_tsv(&stubs);
    let stub_dlls: BTreeSet<&str> = stubs.iter().map(|s| s.dll.as_str()).collect();
    let stub_item = check_or_fix_single(
        "stub_inventory.tsv",
        stub_tsv_path,
        "Stub Inventory",
        format!("{} sites across {} DLLs", stubs.len(), stub_dlls.len()),
        "cargo opsdb -- stub-sync",
        &stub_expected,
        fix,
    )?;

    if let Some(doc) = status_doc {
        let a64_rows = |file: &str| -> Result<usize> {
            let path = db_dir.join(file);
            Ok(fs::read_to_string(&path)
                .with_context(|| format!("read {}", path.display()))?
                .lines()
                .skip(1)
                .count())
        };
        let counts = format!(
            "\n\n- x86-64 operations — {}.\n- Win32 — {}.\n- Stub inventory — {}.\n- A64 emitter — {} test words checked against {} encoding forms.\n\n",
            ops_item.detail, win_item.detail, stub_item.detail,
            a64_rows("a64_emitter_matches.tsv")?, a64_rows("a64_encodings.tsv")?,
        );
        catalog_errors.extend(check_status_counts(doc, &counts, fix)?);
    }
    let mut items = vec![ops_item, win_item, shader_item, gp_item, stub_item];

    // 6. Exhaustive x86-64 ISA Coverage Ledger (Apple Silicon macOS lowerer)
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
        let isa_rep = crate::isa_coverage::analyze_isa_coverage();
        let isa_expected = crate::isa_coverage::format_isa_coverage_tsv(&isa_rep);
        let isa_item = check_or_fix_single(
            "isa_coverage.tsv",
            db_dir.join("isa_coverage.tsv"),
            "ISA Coverage",
            format!(
                "{} forms across {} codes ({} Arm64 {:.1}%, {} advertised gaps)",
                isa_rep.forms.len(),
                isa_rep.included_codes,
                isa_rep.totals.arm64,
                isa_rep.totals.arm64_pct(),
                isa_rep.advertised_but_unimplemented.len(),
            ),
            "cargo opsdb -- isa-coverage --sync",
            &isa_expected,
            fix,
        )?;
        items.push(isa_item);
    }

    Ok(DoctorDbReport {
        catalog_errors,
        items,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle_common::TempDirGuard;

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(Path::to_path_buf)
            .expect("resolve workspace root")
    }

    fn make_temp_db_dir(tag: &str) -> TempDirGuard {
        let dir = std::env::temp_dir().join(format!(
            "port_factory_doctor_test_{}_{}",
            tag,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create temp db dir");
        TempDirGuard::new(dir)
    }

    #[test]
    fn source_and_qualified_item_must_exist_even_if_the_tsv_is_in_sync() {
        let tmp = make_temp_db_dir("references");
        let src = "crates/win32-rt/src/example/mod.rs";
        let file = tmp.path().join(src);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(
            file,
            "fn Working() { Removed(); } // Removed\nexports![Removed];\nstatic EXTRA_EXPORTS: &[Export] = &[Export { name: \"?decorated\", ptr: Working }];",
        )
        .unwrap();
        let errors = catalog_source_errors(
            tmp.path(),
            [src, "crates/deleted.rs"].into_iter(),
            [
                "win32_rt::example::Working",
                "win32_rt::example::EXTRA_EXPORTS[?decorated]",
                "win32_rt::example::EXTRA_EXPORTS[missing]",
                "win32_rt::example::Removed",
                "win32_rt::old_module::Working",
            ]
            .into_iter(),
        );
        assert_eq!(errors.len(), 4, "{errors:?}");
        for missing in [
            "deleted.rs",
            "example::Removed",
            "old_module::Working",
            "EXTRA_EXPORTS[missing]",
        ] {
            assert!(errors.iter().any(|e| e.contains(missing)));
        }
    }

    #[test]
    fn status_fix_preserves_prose_and_refuses_ambiguous_markers() {
        let tmp = make_temp_db_dir("counts");
        let doc = tmp.path().join("STATUS.md");
        let original = format!("before\n{COUNTS_START}\nstale\n{COUNTS_END}\nafter\n");
        fs::write(&doc, &original).unwrap();
        let counts = "\ncurrent\n";
        assert!(check_status_counts(&doc, counts, false).unwrap().is_some());
        assert_eq!(fs::read_to_string(&doc).unwrap(), original);
        assert_eq!(check_status_counts(&doc, counts, true).unwrap(), None);
        assert_eq!(
            fs::read_to_string(&doc).unwrap(),
            original.replace("stale", "current")
        );
        assert_eq!(check_status_counts(&doc, counts, false).unwrap(), None);
        for broken in [original.replace(COUNTS_END, ""), original.repeat(2)] {
            fs::write(&doc, &broken).unwrap();
            assert!(check_status_counts(&doc, counts, true).unwrap().is_some());
            assert_eq!(fs::read_to_string(&doc).unwrap(), broken);
        }
    }

    #[test]
    fn doctor_detects_missing_fixes_and_verifies_in_sync() {
        let root = workspace_root();
        let tmp = make_temp_db_dir("missing_and_sync");

        let expected_tsv_count = if cfg!(all(target_arch = "aarch64", target_os = "macos")) {
            6
        } else {
            5
        };

        // 1. Empty db_dir -> all canonical TSVs reported missing, check mode fails without creating files.
        let missing_rep = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap();
        assert_eq!(missing_rep.items.len(), expected_tsv_count);
        assert!(missing_rep.has_drift());
        assert_eq!(missing_rep.drifted_items().len(), expected_tsv_count);
        for item in &missing_rep.items {
            assert!(matches!(item.status, TsvSyncStatus::Missing { .. }));
            assert!(
                !item.abs_path.exists(),
                "check mode must not create {}",
                item.abs_path.display()
            );
        }
        let err = missing_rep.ensure_clean().unwrap_err();
        let err_msg = err.to_string();
        assert!(err_msg.contains("ops_x86_to_arm64.tsv"));
        assert!(err_msg.contains("win32_apis.tsv"));
        assert!(err_msg.contains("shader_formats.tsv"));
        assert!(err_msg.contains("game_profiles.tsv"));
        assert!(err_msg.contains("stub_inventory.tsv"));
        if cfg!(all(target_arch = "aarch64", target_os = "macos")) {
            assert!(err_msg.contains("isa_coverage.tsv"));
        }

        // 2. Fix mode writes all five canonical TSVs cleanly.
        let fixed_rep = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap();
        assert!(!fixed_rep.has_drift());
        assert!(fixed_rep.ensure_clean().is_ok());
        for item in &fixed_rep.items {
            assert!(matches!(item.status, TsvSyncStatus::Fixed { .. }));
            assert!(item.abs_path.is_file());
        }

        // 3. Subsequent check mode passes with all five InSync.
        let clean_rep = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap();
        assert!(!clean_rep.has_drift());
        assert!(clean_rep.ensure_clean().is_ok());
        for item in &clean_rep.items {
            assert!(matches!(item.status, TsvSyncStatus::InSync { .. }));
        }
    }

    #[test]
    fn doctor_detects_drift_in_each_of_the_tsvs_individually() {
        let root = workspace_root();
        let tmp = make_temp_db_dir("individual_drift");

        // Seed all five canonical TSVs cleanly first.
        let seeded = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap();
        assert!(seeded.ensure_clean().is_ok());

        let mut target_files = vec![
            "ops_x86_to_arm64.tsv",
            "win32_apis.tsv",
            "shader_formats.tsv",
            "game_profiles.tsv",
            "stub_inventory.tsv",
        ];
        if cfg!(all(target_arch = "aarch64", target_os = "macos")) {
            target_files.push("isa_coverage.tsv");
        }
        for target_file in target_files {
            let p = tmp.path().join(target_file);
            let orig = fs::read_to_string(&p).unwrap();
            let mutated = format!("{orig}STALE_DRIFT_SENTINEL\tdummy\n");
            fs::write(&p, &mutated).unwrap();

            // Check mode must fail and flag ONLY `target_file`, without mutating disk!
            let rep = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap();
            assert!(rep.has_drift(), "expected drift for {target_file}");
            let drifted = rep.drifted_items();
            assert_eq!(
                drifted.len(),
                1,
                "expected exactly 1 drifted file for {target_file}, got {}",
                drifted.len()
            );
            assert_eq!(drifted[0].file_name, target_file);
            assert!(matches!(
                drifted[0].status,
                TsvSyncStatus::Drifted {
                    removed_lines: 1,
                    ..
                }
            ));
            let err = rep.ensure_clean().unwrap_err();
            assert!(err.to_string().contains(target_file));
            let summary = rep.format_summary();
            assert!(summary.contains("[DRIFT]"));
            assert!(summary.contains("STALE_DRIFT_SENTINEL"));

            // Ensure check mode did not silently overwrite the drifted file.
            assert_eq!(fs::read_to_string(&p).unwrap(), mutated);

            // Fix mode repairs only `target_file`, leaving the other files untouched/InSync.
            let fix_rep = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap();
            assert!(fix_rep.ensure_clean().is_ok());
            assert_eq!(fs::read_to_string(&p).unwrap(), orig);
            let check_after = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap();
            assert!(check_after.ensure_clean().is_ok());
        }
    }

    #[test]
    fn doctor_rejects_new_stub_sites_even_in_fix_mode() {
        let root = workspace_root();
        let tmp = make_temp_db_dir("reject_new_stubs");

        let seeded = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap();
        assert!(seeded.ensure_clean().is_ok());

        // Remove one existing row from stub_inventory.tsv to simulate a newly added stub in source.
        let stub_tsv = tmp.path().join("stub_inventory.tsv");
        let lines: Vec<String> = fs::read_to_string(&stub_tsv)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();
        assert!(lines.len() > 2);
        let truncated = format!("{}\n", lines[..lines.len() - 1].join("\n"));
        fs::write(&stub_tsv, truncated).unwrap();

        let err_check = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap_err();
        assert!(
            err_check.to_string().contains("zero-stub policy violation"),
            "unexpected error: {err_check}"
        );
        let err_fix = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap_err();
        assert!(
            err_fix.to_string().contains("zero-stub policy violation"),
            "fix mode must also reject new stubs: {err_fix}"
        );
    }
}
