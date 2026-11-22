//! Generates the per-DLL export tables of win32-rt.
//!
//! Every `pub [unsafe] extern "win64" fn NAME` (and `context_capturing_export!`
//! / `win64_varargs_thunk!` definition) in a DLL's source files becomes an
//! export named `NAME`. Code after the first `#[cfg(test)]` is ignored.
//!
//! Tables (`$OUT_DIR/<table>_exports.rs`, each an `&[Export]` expression):
//! * `kernel32` <- src/kernel32/*.rs, src/sync.rs, src/unwind.rs
//! * `ntdll`    <- src/ntdll.rs
//! * `ucrt`     <- src/ucrt/*.rs
//! * `msvcrt`, `advapi32` <- src/<name>.rs
//! * `user32`   <- src/user32/*.rs
//! * `misc_<dll>` <- src/misc/<dll>.rs (one builtin DLL per file)

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

fn module_path(src_root: &Path, file: &Path) -> String {
    let rel = file.strip_prefix(src_root).unwrap().with_extension("");
    let mut parts: Vec<String> = rel.iter().map(|c| c.to_string_lossy().into_owned()).collect();
    if parts.last().is_some_and(|l| l == "mod") {
        parts.pop();
    }
    let mut s = String::from("crate");
    for p in parts {
        s.push_str("::");
        s.push_str(&p);
    }
    s
}

fn parse_macro_ident_list(block: &str, names: &mut Vec<String>) {
    let list_part = block.split_once(';').map(|(_, rhs)| rhs).unwrap_or(block);
    for raw in list_part.split(',') {
        let ident: String = raw
            .trim()
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if !ident.is_empty()
            && !ident.starts_with("dispatch_")
            && !ident.starts_with("aw2_")
            && !names.contains(&ident)
        {
            names.push(ident);
        }
    }
}

fn scan(file: &Path) -> Vec<String> {
    let src = fs::read_to_string(file).unwrap_or_default();
    let src = src.split("#[cfg(test)]").next().unwrap_or("");
    let mut names = Vec::new();
    for line in src.lines() {
        let t = line.trim_start();
        // Macro-generated exports can be listed via `// @exports a b c` or extracted
        // automatically from `failing!(...)` / `win64_stubs!(...)` below.
        if let Some(r) = t.strip_prefix("// @exports ") {
            for s in r.split_whitespace() {
                let sym = s.to_string();
                if !names.contains(&sym) {
                    names.push(sym);
                }
            }
            continue;
        }
        // Only top-level items (no indentation) are exports.
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
        let name: String = rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
        if !name.is_empty() && !name.starts_with("dispatch_") && !name.starts_with("aw2_") && !names.contains(&name) {
            names.push(name);
        }
    }
    // Also auto-extract identifiers from top-level `failing!(...)` and `win64_stubs!(...)` blocks.
    for prefix in ["failing!(", "win64_stubs!("] {
        let mut rem = src;
        while let Some(pos) = rem.find(prefix) {
            let after = &rem[pos + prefix.len()..];
            if let Some(end) = after.find(");") {
                parse_macro_ident_list(&after[..end], &mut names);
                rem = &after[end + 2..];
            } else {
                break;
            }
        }
    }
    names
}

fn write_if_changed(path: PathBuf, content: String) {
    if fs::read_to_string(&path).ok().as_ref() != Some(&content) {
        fs::write(path, content).unwrap();
    }
}

fn table(out: &Path, src_root: &Path, name: &str, files: &[PathBuf]) {
    let mut seen = std::collections::HashSet::new();
    let mut s = String::from("&[\n");
    for f in files {
        println!("cargo:rerun-if-changed={}", f.display());
        let m = module_path(src_root, f);
        for n in scan(f) {
            if !seen.insert(n.clone()) {
                panic!("duplicate export {n} in table {name} ({})", f.display());
            }
            writeln!(s, "    crate::Export {{ name: \"{n}\", ptr: crate::Ptr({m}::{n} as *const ()) }},").unwrap();
        }
    }
    s.push(']');
    write_if_changed(out.join(format!("{name}_exports.rs")), s);
}

fn rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "rs")).collect())
        .unwrap_or_default();
    v.sort();
    v
}

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", root.join("kernel32").display());
    println!("cargo:rerun-if-changed={}", root.join("ucrt").display());
    println!("cargo:rerun-if-changed={}", root.join("misc").display());

    let mut k32 = rs_files(&root.join("kernel32"));
    k32.push(root.join("sync.rs"));
    k32.push(root.join("unwind.rs"));
    table(&out, &root, "kernel32", &k32);
    table(&out, &root, "ntdll", &[root.join("ntdll.rs")]);
    table(&out, &root, "ucrt", &rs_files(&root.join("ucrt")));
    for n in ["msvcrt", "advapi32"] {
        table(&out, &root, n, &[root.join(format!("{n}.rs"))]);
    }
    println!("cargo:rerun-if-changed={}", root.join("user32").display());
    table(&out, &root, "user32", &rs_files(&root.join("user32")));

    // misc/<dll>.rs -> one table per DLL plus a list for misc::dlls().
    let mut list = String::from("&[\n");
    for f in rs_files(&root.join("misc")) {
        let stem = f.file_stem().unwrap().to_string_lossy().into_owned();
        if stem == "mod" {
            continue;
        }
        table(&out, &root, &format!("misc_{stem}"), &[f.clone()]);
        writeln!(
            list,
            "    (\"{stem}.dll\", include!(concat!(env!(\"OUT_DIR\"), \"/misc_{stem}_exports.rs\")) as &'static [crate::Export], crate::misc::{stem}::EXTRA_EXPORTS, crate::misc::{stem}::ORDINALS),"
        )
        .unwrap();
    }
    list.push(']');
    write_if_changed(out.join("misc_dlls.rs"), list);
}
