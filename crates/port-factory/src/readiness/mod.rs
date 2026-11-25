//! Read-only, per-image readiness for AMD64 Windows packages. Inspect one image at a
//! time, retaining only import/export metadata and grouped instruction findings.
mod cpu;
mod imports;

pub(crate) use cpu::advertised as cpuid_advertised;

use crate::json_output::{exit_codes, JsonObj, JsonValue};
use anyhow::{ensure, Context, Result};
use cpu::CpuScan;
use imports::Catalog;
use pe_image::{normalize_dll_name, ExportTarget, Exports, ImportDll, ImportName, Pe};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

// Declaration order is the ranking order. Frequencies only break ties within a class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Finding {
    Unimplemented,
    Missing,
    Failing,
    Stub,
    Partial,
    UnverifiedForm,
    Interpreter,
    GuestBoundary,
    Shipped,
    Real,
}

impl Finding {
    fn label(self) -> &'static str {
        match self {
            Self::Unimplemented => "unimplemented",
            Self::Missing => "missing",
            Self::Failing => "failing",
            Self::Stub => "stub",
            Self::Partial => "partial",
            Self::UnverifiedForm => "unverified_form",
            Self::Interpreter => "interpreter_only",
            Self::GuestBoundary => "guest_boundary",
            Self::Shipped => "shipped",
            Self::Real => "real",
        }
    }
}

struct Module {
    cpu: CpuScan,
    imports: Vec<ImportDll>,
    exports: Exports,
}

impl Module {
    fn read(file: &Path) -> Result<Self> {
        let bytes = fs::read(file)?;
        let pe = Pe::parse(&bytes)?;
        let mut imports = pe.imports()?;
        imports.extend(pe.delay_imports()?);
        let exports = pe.exports()?;
        Ok(Self {
            cpu: CpuScan::scan(&pe, &exports)?,
            imports,
            exports,
        })
    }

    fn export(&self, name: &ImportName) -> Option<&ExportTarget> {
        match name {
            ImportName::Name { name, .. } => self.exports.by_name(name),
            ImportName::Ordinal(n) => self.exports.by_ordinal(u32::from(*n)),
        }
        .map(|e| &e.target)
    }
}

#[derive(Default)]
struct Package {
    // Keys retain the directory and fold only the DLL basename, as Windows lookup does.
    files: BTreeMap<(PathBuf, String), PathBuf>,
    modules: BTreeMap<PathBuf, Module>,
    skipped: BTreeMap<PathBuf, String>,
}

impl Package {
    fn neighbor(&self, requester: &Path, dll: &str) -> Option<&PathBuf> {
        let name = normalize_dll_name(dll);
        let dir = requester.parent()?.to_owned();
        self.files
            .get(&(dir, name.clone()))
            .or_else(|| self.files.get(&(PathBuf::new(), name)))
    }

    fn discover(root: &Path, single: Option<&Path>, catalog: &Catalog) -> Result<Self> {
        let mut pkg = Self::default();
        let mut dirs = vec![PathBuf::new()];
        let mut pending = Vec::new();
        while let Some(dir) = dirs.pop() {
            for entry in fs::read_dir(root.join(&dir))? {
                let entry = entry?;
                let rel = dir.join(entry.file_name());
                let ty = entry.file_type()?;
                if ty.is_dir() && single.is_none() {
                    dirs.push(rel);
                } else if ty.is_file() {
                    // Folder scans include extensionless images and PE plugins. For a
                    // single executable only index names, then read its dependency closure.
                    if single.is_none() {
                        if !port_manifest::has_pe_extension(&rel) {
                            let mut magic = [0; 2];
                            if fs::File::open(entry.path())?.read(&mut magic)? != 2
                                || &magic != b"MZ"
                            {
                                continue;
                            }
                        }
                        pending.push((rel.clone(), None));
                    }
                    let key = (
                        dir.clone(),
                        entry.file_name().to_string_lossy().to_ascii_lowercase(),
                    );
                    ensure!(
                        pkg.files.insert(key, rel.clone()).is_none(),
                        "ambiguous case-insensitive filename {}",
                        rel.display()
                    );
                }
            }
        }
        if let Some(file) = single {
            pending.push((file.to_owned(), None));
        }
        ensure!(!pending.is_empty(), "no PE candidates found");
        while let Some((rel, request)) = pending.pop() {
            if pkg.skipped.contains_key(&rel) {
                continue;
            }
            if !pkg.modules.contains_key(&rel) {
                let module = match Module::read(&root.join(&rel)) {
                    Ok(module) => module,
                    Err(e) => {
                        pkg.skipped.insert(rel, format!("{e:#}"));
                        continue;
                    }
                };
                if single.is_some() {
                    for imports in &module.imports {
                        if let Some(path) = catalog.shipped_path(&pkg, &rel, &imports.dll) {
                            for entry in &imports.entries {
                                pending.push((path.clone(), Some((entry.name.clone(), 0))));
                            }
                        }
                    }
                }
                pkg.modules.insert(rel.clone(), module);
            }
            // Only requesting a particular export makes its forwarder a dependency.
            // A loaded DLL may receive new symbol requests after its imports have been read.
            let Some((symbol, depth)) = request else {
                continue;
            };
            if depth >= imports::FORWARDER_LIMIT {
                continue;
            }
            let Some(ExportTarget::Forwarder(target)) = pkg.modules[&rel].export(&symbol) else {
                continue;
            };
            let Some((dll, next)) = imports::forwarder(target) else {
                continue;
            };
            if let Some(path) = catalog.shipped_path(&pkg, &rel, dll) {
                pending.push((path.clone(), Some((next, depth + 1))));
            }
        }
        Ok(pkg)
    }
}

pub struct ReadinessReport {
    pub data: JsonValue,
    pub text: String,
    pub exit_code: i32,
}

// A finding can occur in many modules. Count static instruction sites or import slots
// and retain the affected package-relative paths so the ranking is actionable.
#[derive(Default)]
struct Hits {
    count: u64,
    files: BTreeSet<String>,
}

pub fn analyze(input: &Path, db: &Path) -> Result<ReadinessReport> {
    let input = input.canonicalize().context("open readiness input")?;
    let (root, single) = if input.is_file() {
        (input.parent().unwrap(), input.file_name().map(Path::new))
    } else {
        (input.as_path(), None)
    };
    let catalog = Catalog::read(db)?;
    let pkg = Package::discover(root, single, &catalog)?;
    let mut ranked: BTreeMap<(Finding, String), Hits> = BTreeMap::new();
    let mut binaries = Vec::new();
    let mut details = String::new();
    for (path, module) in &pkg.modules {
        let file = path.to_string_lossy();
        let mut record = |kind, label: String, count| {
            let hits = ranked.entry((kind, label)).or_default();
            hits.count += count;
            hits.files.insert(file.to_string());
        };
        let mut instructions = Vec::new();
        let _ = writeln!(
            details,
            "\n{file} ({} instructions, {} undecodable bytes)",
            module.cpu.decoded, module.cpu.invalid_bytes
        );
        for (&(kind, code), hit) in &module.cpu.findings {
            let form = format!("{code:?}");
            if kind != Finding::GuestBoundary {
                record(kind, form.clone(), hit.count);
            }
            let _ = writeln!(
                details,
                "  {} {} x{} at RVA {:#x}",
                kind.label(),
                form,
                hit.count,
                hit.first_rva
            );
            if !hit.advertised_features.is_empty() {
                let _ = writeln!(
                    details,
                    "    Guest CPUID advertises {}",
                    hit.advertised_features.join(", ")
                );
            }
            instructions.push(
                JsonObj::new()
                    .str("mnemonic", &hit.mnemonic)
                    .str("form", form)
                    .str("kind", kind.label())
                    .u64("count", hit.count)
                    .u64("first_rva", hit.first_rva.into())
                    .str_array(
                        "advertised_but_unimplemented",
                        hit.advertised_features.iter(),
                    )
                    .build(),
            );
        }
        let mut imports = Vec::new();
        for dll in &module.imports {
            let delay = dll.delay.is_some();
            for entry in &dll.entries {
                let binding = catalog.resolve(&pkg, path, &dll.dll, &entry.name);
                let symbol = entry.name.to_string();
                let label = format!("{}!{symbol}", normalize_dll_name(&dll.dll));
                if !matches!(binding.finding, Finding::Shipped | Finding::Real) {
                    record(binding.finding, label.clone(), 1);
                }
                let _ = writeln!(
                    details,
                    "  {} {label}{} -> {}",
                    binding.finding.label(),
                    if delay { " (delay)" } else { "" },
                    binding.provider
                );
                let sites = binding
                    .sites
                    .iter()
                    .map(|s| {
                        JsonObj::new()
                            .str("kind", &s.kind)
                            .str("file", &s.file)
                            .usize("line", s.line)
                            .str("detail", &s.detail)
                            .build()
                    })
                    .collect();
                imports.push(
                    JsonObj::new()
                        .str("dll", &dll.dll)
                        .str("symbol", symbol)
                        .u64("iat_rva", entry.iat_rva.into())
                        .bool("delay", delay)
                        .str("kind", binding.finding.label())
                        .str("provider", binding.provider)
                        .val("stub_sites", JsonValue::Array(sites))
                        .build(),
                );
            }
        }
        binaries.push(
            JsonObj::new()
                .str("path", file)
                .u64("decoded", module.cpu.decoded)
                .u64("undecodable_bytes", module.cpu.invalid_bytes)
                .val("instructions", JsonValue::Array(instructions))
                .val("imports", JsonValue::Array(imports))
                .build(),
        );
    }
    let mut ranked: Vec<_> = ranked.into_iter().collect();
    ranked.sort_by(|((a, an), ah), ((b, bn), bh)| {
        a.cmp(b).then(bh.count.cmp(&ah.count)).then(an.cmp(bn))
    });
    let mut blockers = Vec::new();
    let mut text = String::from("Ranked static findings (conditional on execution)\n");
    for ((kind, name), hits) in &ranked {
        let _ = writeln!(
            text,
            "  {} {name} ({} sites in {} images)",
            kind.label(),
            hits.count,
            hits.files.len()
        );
        blockers.push(
            JsonObj::new()
                .str("kind", kind.label())
                .str("item", name)
                .u64("sites", hits.count)
                .str_array("images", hits.files.iter())
                .build(),
        );
    }
    if ranked.is_empty() {
        text.push_str("  None in the decoded code and import tables.\n");
    }
    text.push_str(&details);
    let mut skipped = Vec::new();
    for (path, error) in &pkg.skipped {
        let _ = writeln!(text, "  Unscanned {}: {error}", path.display());
        skipped.push(
            JsonObj::new()
                .str("path", path.to_string_lossy())
                .str("reason", error)
                .build(),
        );
    }
    let exit_code = if !skipped.is_empty() {
        exit_codes::RUNTIME_ERROR
    } else if ranked
        .iter()
        .any(|((k, _), _)| *k == Finding::Unimplemented)
    {
        exit_codes::INCOMPATIBLE
    } else if ranked.iter().any(|((k, _), _)| *k == Finding::Missing) {
        exit_codes::UNRESOLVED_IMPORTS
    } else {
        exit_codes::OK
    };
    let assessment = match exit_code {
        exit_codes::RUNTIME_ERROR => "incomplete",
        exit_codes::INCOMPATIBLE | exit_codes::UNRESOLVED_IMPORTS => "potential_blockers",
        _ if !ranked.is_empty() => "review_findings",
        _ => "no_static_findings",
    };
    let data = JsonObj::new()
        .str("assessment", assessment)
        .val("binaries", JsonValue::Array(binaries))
        .val("ranked_findings", JsonValue::Array(blockers))
        .val("unscanned", JsonValue::Array(skipped))
        .build();
    Ok(ReadinessReport {
        data,
        text,
        exit_code,
    })
}
