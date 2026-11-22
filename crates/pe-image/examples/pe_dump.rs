//! Dump the transitive import closure of a PE inside a game directory.
//!
//! `cargo run -p pe-image --example pe_dump -- <game_dir> [AlanWake2.exe]`
//!
//! Prints every system (non game-dir) DLL import needed by the closure, which
//! is the to-do list for `win32-rt`.

use pe_image::{normalize_dll_name, Pe};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let dir = match args
        .get(1)
        .cloned()
        .or_else(|| std::env::var("PORT_GAME_DIR").ok())
        .or_else(|| std::env::var("AW2_GAME_DIR").ok())
    {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => anyhow::bail!("pass <game_dir> or set PORT_GAME_DIR / AW2_GAME_DIR"),
    };
    let root = args.get(2).cloned().unwrap_or("AlanWake2.exe".into());
    let include_delay = std::env::var_os("PE_DUMP_DELAY").is_some();
    let mut files: HashMap<String, PathBuf> = HashMap::new();
    for e in std::fs::read_dir(&dir)? {
        let p = e?.path();
        if let Some(n) = p.file_name().and_then(|n| n.to_str()) {
            files.insert(n.to_ascii_lowercase(), p.clone());
        }
    }
    let mut queue = VecDeque::from([root.to_ascii_lowercase()]);
    let mut seen = BTreeSet::new();
    let mut system: BTreeMap<String, BTreeMap<String, BTreeSet<String>>> = BTreeMap::new();
    while let Some(m) = queue.pop_front() {
        if !seen.insert(m.clone()) {
            continue;
        }
        let data = std::fs::read(&files[&m])?;
        let pe = Pe::parse(&data)?;
        let tls = pe.tls()?;
        println!(
            "# {m}: base=0x{:x} size=0x{:x} stack=0x{:x} tls={} tls_cbs={}",
            pe.headers.image_base,
            pe.headers.size_of_image,
            pe.headers.stack_reserve,
            tls.map_or(0, |t| t.block_size()),
            pe.tls_callback_rvas()?.len()
        );
        let mut all = pe.imports()?;
        let delay = pe.delay_imports()?;
        for d in &delay {
            println!("#   delay: {} ({} fns)", d.dll, d.entries.len());
        }
        if include_delay {
            all.extend(delay);
        }
        for imp in all {
            let n = normalize_dll_name(&imp.dll);
            if files.contains_key(&n) {
                queue.push_back(n);
            } else {
                let e = system.entry(n).or_default();
                for ent in imp.entries {
                    e.entry(ent.name.to_string()).or_default().insert(m.clone());
                }
            }
        }
    }
    for (dll, fns) in &system {
        println!("{dll} ({})", fns.len());
        for (f, users) in fns {
            println!("    {f}    <- {}", users.iter().cloned().collect::<Vec<_>>().join(","));
        }
    }
    Ok(())
}
