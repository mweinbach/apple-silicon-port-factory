use super::{Finding, Package};
use crate::stub_inventory::{parse_inventory_tsv, StubInventoryEntry};
use crate::win32_db::{ImplKind, Win32ApiEntry, Win32Category, Win32Database};
use anyhow::{ensure, Context, Result};
use pe_image::{normalize_dll_name, ExportTarget, ImportName};
use port_manifest::DllRoutingTable;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

pub(super) const FORWARDER_LIMIT: usize = 8;

pub(super) struct Catalog {
    apis: Win32Database,
    sites: BTreeMap<String, Vec<StubInventoryEntry>>,
    routes: DllRoutingTable,
}

pub(super) struct Binding<'a> {
    pub finding: Finding,
    pub provider: String,
    pub sites: &'a [StubInventoryEntry],
}

impl Catalog {
    pub fn read(root: &Path) -> Result<Self> {
        let apis =
            fs::read_to_string(root.join("win32_apis.tsv")).context("reading win32_apis.tsv")?;
        let mut lines = apis.lines();
        ensure!(
            lines.next()
                == Some("dll\tsymbol\tordinal\tcategory\timpl_kind\trust_path\tsource_file"),
            "invalid win32_apis.tsv header"
        );
        let mut entries = Vec::new();
        for (i, line) in lines.enumerate() {
            let fields: Vec<_> = line.split('\t').collect();
            let [dll, symbol, ordinal, _, kind, rust_path, source_file] = fields[..] else {
                anyhow::bail!("invalid win32_apis.tsv row {}", i + 2);
            };
            entries.push(Win32ApiEntry {
                dll: dll.into(),
                symbol: symbol.into(),
                ordinal: if ordinal == "-" {
                    None
                } else {
                    Some(ordinal.parse()?)
                },
                category: Win32Category::for_dll(dll),
                impl_kind: ImplKind::parse(kind).context("unknown Win32 implementation kind")?,
                rust_path: rust_path.into(),
                source_file: source_file.into(),
            });
        }
        let inventory = fs::read_to_string(root.join("stub_inventory.tsv"))
            .context("reading stub_inventory.tsv")?;
        ensure!(
            inventory.lines().next() == Some("dll\tapi\tkind\tfile\tline\tdetail"),
            "invalid stub_inventory.tsv header"
        );
        // Several DLLs re-export the same Rust implementation. Join the inventory
        // against that target once so aliases retain the original stub-site evidence.
        let targets: BTreeMap<_, _> = entries
            .iter()
            .map(|e| ((e.dll.as_str(), e.symbol.as_str()), e.rust_path.as_str()))
            .collect();
        let mut sites: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for site in parse_inventory_tsv(&inventory) {
            if let Some(target) = targets.get(&(site.dll.as_str(), site.api.as_str())) {
                sites.entry((*target).to_owned()).or_default().push(site);
            }
        }
        Ok(Self {
            apis: Win32Database::from_entries(entries),
            sites,
            routes: DllRoutingTable::new(),
        })
    }

    pub fn shipped_path<'p>(
        &self,
        package: &'p Package,
        requester: &Path,
        dll: &str,
    ) -> Option<&'p PathBuf> {
        let dll = normalize_dll_name(dll);
        if dll.starts_with("api-ms-win-")
            || dll.starts_with("ext-ms-win-")
            || self.routes.is_builtin_override(&dll)
        {
            return None;
        }
        package.neighbor(requester, &dll)
    }

    pub fn resolve(
        &self,
        package: &Package,
        requester: &Path,
        dll: &str,
        name: &ImportName,
    ) -> Binding<'_> {
        self.resolve_at(package, requester, dll, name, 0)
    }

    fn resolve_at(
        &self,
        package: &Package,
        requester: &Path,
        dll: &str,
        name: &ImportName,
        depth: usize,
    ) -> Binding<'_> {
        let missing = |reason: String| Binding {
            finding: Finding::Missing,
            provider: reason,
            sites: &[],
        };
        if depth > FORWARDER_LIMIT {
            return missing("forwarder chain exceeded loader limit".into());
        }
        if let Some(path) = self.shipped_path(package, requester, dll) {
            let Some(module) = package.modules.get(path) else {
                return missing(format!("unreadable shipped module {}", path.display()));
            };
            return match module.export(name) {
                Some(ExportTarget::Rva(rva)) if *rva != 0 => Binding {
                    finding: Finding::Shipped,
                    provider: format!("{}!{name}", path.display()),
                    sites: &[],
                },
                Some(ExportTarget::Forwarder(target)) => match forwarder(target) {
                    Some((dll, next)) => self.resolve_at(package, path, dll, &next, depth + 1),
                    None => missing(format!("invalid forwarder {target}")),
                },
                _ => missing(format!("absent from {}", path.display())),
            };
        }
        match self.apis.resolve(dll, name) {
            Some(hit) => Binding {
                finding: match hit.impl_kind {
                    ImplKind::Failing => Finding::Failing,
                    ImplKind::Stub => Finding::Stub,
                    ImplKind::Partial => Finding::Partial,
                    ImplKind::Real => Finding::Real,
                },
                provider: format!("{}!{} ({})", hit.dll, hit.symbol, hit.source_file),
                sites: self.sites.get(&hit.rust_path).map_or(&[], Vec::as_slice),
            },
            None => missing("no catalog or package binding".into()),
        }
    }
}

pub(super) fn forwarder(s: &str) -> Option<(&str, ImportName)> {
    let (dll, symbol) = s.rsplit_once('.')?;
    let name = match symbol.strip_prefix('#') {
        Some(n) => ImportName::Ordinal(n.parse().ok()?),
        None => ImportName::Name {
            hint: 0,
            name: symbol.into(),
        },
    };
    Some((dll, name))
}
