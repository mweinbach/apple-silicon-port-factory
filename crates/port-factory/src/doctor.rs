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

use crate::game_profiles::{
    audit_all_game_profiles, format_game_profiles_tsv, TOP_GAME_PROFILES,
};
use crate::ops_db::{catalog_stats, format_tsv as format_ops_tsv};
use crate::win32_db::Win32Database;
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

/// Aggregated report across all four canonical `db/*.tsv` databases.
#[derive(Debug, Clone)]
pub struct DoctorDbReport {
    pub items: Vec<TsvCheckItem>,
}

impl DoctorDbReport {
    pub fn drifted_items(&self) -> Vec<&TsvCheckItem> {
        self.items.iter().filter(|i| i.status.is_drift()).collect()
    }

    pub fn has_drift(&self) -> bool {
        self.items.iter().any(|i| i.status.is_drift())
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
        out
    }

    /// Returns `Ok(())` when all TSV files match their canonical generated state, or an `Err`
    /// enumerating every missing/drifted file so CLI callers exit non-zero.
    pub fn ensure_clean(&self) -> Result<()> {
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

/// Check (when `fix == false`) or synchronize (when `fix == true`) all four canonical TSV
/// databases in `db_dir`:
/// - `ops_x86_to_arm64.tsv`
/// - `win32_apis.tsv`
/// - `shader_formats.tsv`
/// - `game_profiles.tsv`
pub fn check_or_fix_db_tsvs(
    workspace_root: &Path,
    db_dir: &Path,
    fix: bool,
) -> Result<DoctorDbReport> {
    // 1. x86-64 -> ARM64 Ops Database
    let ops_expected = format_ops_tsv();
    let st = catalog_stats();
    let ops_item = check_or_fix_single(
        "ops_x86_to_arm64.tsv",
        db_dir.join("ops_x86_to_arm64.tsv"),
        "Ops Database",
        format!(
            "{} families, {} mnemonics ({} JIT_ARM64, {} vectors)",
            st.total_records, st.total_mnemonics, st.jit_arm64_records, st.total_verified_vectors
        ),
        "cargo opsdb -- sync",
        &ops_expected,
        fix,
    )?;

    // 2. Win32 API Database
    let win_db = Win32Database::from_workspace_src(workspace_root)?;
    let win_expected = win_db.format_tsv();
    let dll_count: BTreeSet<&str> = win_db.entries.iter().map(|e| e.dll.as_str()).collect();
    let win_item = check_or_fix_single(
        "win32_apis.tsv",
        db_dir.join("win32_apis.tsv"),
        "Win32 Database",
        format!("{} APIs across {} DLLs", win_db.entries.len(), dll_count.len()),
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
    let gp_ready = gp_audits.iter().filter(|a| a.missing_dlls.is_empty()).count();
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

    Ok(DoctorDbReport {
        items: vec![ops_item, win_item, shader_item, gp_item],
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
    fn doctor_detects_missing_fixes_and_verifies_in_sync() {
        let root = workspace_root();
        let tmp = make_temp_db_dir("missing_and_sync");

        // 1. Empty db_dir -> all 4 TSVs reported missing, check mode fails without creating files.
        let missing_rep = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap();
        assert_eq!(missing_rep.items.len(), 4);
        assert!(missing_rep.has_drift());
        assert_eq!(missing_rep.drifted_items().len(), 4);
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

        // 2. Fix mode writes all 4 canonical TSVs cleanly.
        let fixed_rep = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap();
        assert!(!fixed_rep.has_drift());
        assert!(fixed_rep.ensure_clean().is_ok());
        for item in &fixed_rep.items {
            assert!(matches!(item.status, TsvSyncStatus::Fixed { .. }));
            assert!(item.abs_path.is_file());
        }

        // 3. Subsequent check mode passes with all 4 InSync.
        let clean_rep = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap();
        assert!(!clean_rep.has_drift());
        assert!(clean_rep.ensure_clean().is_ok());
        for item in &clean_rep.items {
            assert!(matches!(item.status, TsvSyncStatus::InSync { .. }));
        }
    }

    #[test]
    fn doctor_detects_drift_in_each_of_the_four_tsvs_individually() {
        let root = workspace_root();
        let tmp = make_temp_db_dir("individual_drift");

        // Seed all 4 canonical TSVs cleanly first.
        let seeded = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap();
        assert!(seeded.ensure_clean().is_ok());

        for target_file in [
            "ops_x86_to_arm64.tsv",
            "win32_apis.tsv",
            "shader_formats.tsv",
            "game_profiles.tsv",
        ] {
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

            // Fix mode repairs only `target_file`, leaving the other 3 untouched/InSync.
            let fix_rep = check_or_fix_db_tsvs(&root, tmp.path(), true).unwrap();
            assert!(fix_rep.ensure_clean().is_ok());
            assert_eq!(fs::read_to_string(&p).unwrap(), orig);
            let check_after = check_or_fix_db_tsvs(&root, tmp.path(), false).unwrap();
            assert!(check_after.ensure_clean().is_ok());
        }
    }
}
