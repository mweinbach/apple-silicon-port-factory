//! Proc-macro shim that enables `win32-rt`, `aw2-game`, and `aw2-loader` to
//! compile natively on `aarch64-apple-darwin` without modifying `extern "win64"`
//! signatures in the source tree.
//!
//! * On `x86_64`, [`win64_mods!`] expands to its input tokens unchanged.
//! * On non-`x86_64` targets (e.g. `aarch64`), [`win64_mods!`] mirrors the
//!   crate's `src/` tree into `target/win64-abi-aarch64/<pkg>/src/` with
//!   `extern "win64"` rewritten to `extern "C"`, attaches `#[path = "..."]`
//!   to each top-level module declaration, and registers the original source
//!   files in `rustc` dep-info via `include_bytes!`.

use proc_macro::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use std::fs;
use std::path::{Path, PathBuf};

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn is_narrow_int_type(ty: &str) -> bool {
    matches!(
        ty,
        "u8" | "i8" | "u16" | "i16" | "BYTE" | "BOOLEAN" | "UCHAR" | "USHORT" | "WORD" | "WCHAR" | "SHORT"
    )
}

fn parse_narrow_params(params: &str) -> String {
    let mut guards = String::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    let bytes = params.as_bytes();
    for i in 0..=bytes.len() {
        let at_end = i == bytes.len();
        let ch = if at_end { b',' } else { bytes[i] };
        if !at_end {
            match ch {
                b'<' | b'(' | b'[' => depth += 1,
                b'>' | b')' | b']' => depth -= 1,
                _ => {}
            }
        }
        if (at_end || (ch == b',' && depth == 0)) && start < i {
            let param = params[start..i].trim();
            start = i + 1;
            if let Some(colon) = param.find(':') {
                let lhs = param[..colon].trim();
                let ty = param[colon + 1..].trim();
                let (is_mut, ident) = if let Some(rest) = lhs.strip_prefix("mut") {
                    let r = rest.trim();
                    if !r.is_empty() && r.bytes().all(is_ident_char) {
                        (true, r)
                    } else {
                        (false, "")
                    }
                } else if !lhs.is_empty() && lhs != "_" && lhs.bytes().all(is_ident_char) {
                    (false, lhs)
                } else {
                    (false, "")
                };
                if !ident.is_empty() {
                    let mut_kw = if is_mut { "mut " } else { "" };
                    if is_narrow_int_type(ty) {
                        guards.push_str(&format!(
                            " #[allow(unused_variables, unused_mut)] let {mut_kw}{ident}: {ty} = {{ let mut __v: u32 = {ident} as u32; unsafe {{ core::arch::asm!(\"/* {{0:w}} */\", inlateout(reg) __v, options(pure, nomem, nostack, preserves_flags)); }} __v as {ty} }};"
                        ));
                    } else if ty == "bool" {
                        guards.push_str(&format!(
                            " #[allow(unused_variables, unused_mut)] let {mut_kw}{ident}: bool = {{ let mut __v: u32 = ({ident} as u8) as u32; unsafe {{ core::arch::asm!(\"/* {{0:w}} */\", inlateout(reg) __v, options(pure, nomem, nostack, preserves_flags)); }} (__v as u8) != 0 }};"
                        ));
                    }
                }
            }
        }
    }
    guards
}

fn transform_win64_source(raw: &str) -> String {
    const NEEDLE: &str = "extern \"win64\"";
    let mut out = String::with_capacity(raw.len() + 1024);
    let mut pos = 0usize;
    let bytes = raw.as_bytes();
    while let Some(rel) = raw[pos..].find(NEEDLE) {
        let idx = pos + rel;
        out.push_str(&raw[pos..idx]);
        out.push_str("extern \"C\"");
        let mut cur = idx + NEEDLE.len();

        let mut scan = cur;
        while scan < bytes.len() && bytes[scan].is_ascii_whitespace() {
            scan += 1;
        }
        if raw[scan..].starts_with("fn") && scan + 2 < bytes.len() && bytes[scan + 2].is_ascii_whitespace() {
            scan += 2;
            while scan < bytes.len() && bytes[scan].is_ascii_whitespace() {
                scan += 1;
            }
            let name_start = scan;
            while scan < bytes.len() && is_ident_char(bytes[scan]) {
                scan += 1;
            }
            if scan > name_start {
                while scan < bytes.len() && bytes[scan].is_ascii_whitespace() {
                    scan += 1;
                }
                if scan < bytes.len() && bytes[scan] == b'<' {
                    let mut gdepth = 1i32;
                    scan += 1;
                    while scan < bytes.len() && gdepth > 0 {
                        match bytes[scan] {
                            b'<' => gdepth += 1,
                            b'>' => gdepth -= 1,
                            _ => {}
                        }
                        scan += 1;
                    }
                    while scan < bytes.len() && bytes[scan].is_ascii_whitespace() {
                        scan += 1;
                    }
                }
                if scan < bytes.len() && bytes[scan] == b'(' {
                    let p_start = scan + 1;
                    let mut pdepth = 1i32;
                    scan += 1;
                    while scan < bytes.len() && pdepth > 0 {
                        match bytes[scan] {
                            b'(' => pdepth += 1,
                            b')' => pdepth -= 1,
                            _ => {}
                        }
                        scan += 1;
                    }
                    if pdepth == 0 {
                        let p_end = scan - 1;
                        let guards = parse_narrow_params(&raw[p_start..p_end]);
                        if !guards.is_empty() {
                            let mut bscan = scan;
                            let mut rdepth = 0i32;
                            while bscan < bytes.len() {
                                match bytes[bscan] {
                                    b'(' | b'[' => rdepth += 1,
                                    b')' | b']' => rdepth -= 1,
                                    b';' if rdepth == 0 => break,
                                    b'{' if rdepth == 0 => {
                                        out.push_str(&raw[cur..=bscan]);
                                        out.push_str(&guards);
                                        cur = bscan + 1;
                                        break;
                                    }
                                    _ => {}
                                }
                                bscan += 1;
                            }
                        }
                    }
                }
            }
        }
        pos = cur;
    }
    out.push_str(&raw[pos..]);
    out
}

fn sync_dir(src_dir: &Path, dst_dir: &Path, tracked: &mut Vec<PathBuf>) {
    let _ = fs::create_dir_all(dst_dir);
    if let Ok(dst_entries) = fs::read_dir(dst_dir) {
        for entry in dst_entries.flatten() {
            let dst_path = entry.path();
            let src_path = src_dir.join(entry.file_name());
            if !src_path.exists() || src_path.is_dir() != dst_path.is_dir() {
                if dst_path.is_dir() {
                    let _ = fs::remove_dir_all(&dst_path);
                } else {
                    let _ = fs::remove_file(&dst_path);
                }
            }
        }
    }
    let Ok(entries) = fs::read_dir(src_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let dst = dst_dir.join(entry.file_name());
        if path.is_dir() {
            sync_dir(&path, &dst, tracked);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let pstr = path.to_string_lossy();
            let is_generated = pstr.contains("/shards/") || pstr.contains("/dll_native/lift/");
            if !is_generated && path.metadata().map_or(false, |m| m.len() < 2_000_000) {
                tracked.push(path.clone());
            }
            if is_generated {
                if let (Ok(sm), Ok(dm)) = (path.metadata(), dst.metadata()) {
                    if let (Ok(st), Ok(dt)) = (sm.modified(), dm.modified()) {
                        if dt >= st {
                            continue;
                        }
                    }
                }
            }
            let Ok(raw) = fs::read_to_string(&path) else {
                continue;
            };
            let transformed = transform_win64_source(&raw);
            let needs_write = match fs::read_to_string(&dst) {
                Ok(existing) => existing != transformed,
                Err(_) => true,
            };
            if needs_write {
                let _ = fs::write(&dst, transformed);
            }
            if let Ok(sm) = path.metadata().and_then(|m| m.modified()) {
                let needs_mtime = dst
                    .metadata()
                    .and_then(|m| m.modified())
                    .map_or(true, |dm| dm != sm);
                if needs_mtime {
                    if let Ok(f) = fs::OpenOptions::new().write(true).open(&dst) {
                        let _ = f.set_modified(sm);
                    }
                }
            }
        }
    }
}

#[proc_macro]
pub fn win64_mods(input: TokenStream) -> TokenStream {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "crate".into());
    let src_dir = manifest_dir.join("src");
    let target_root = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let mut dir = manifest_dir.as_path();
            while let Some(parent) = dir.parent() {
                if parent.join("Cargo.lock").is_file() {
                    return parent.join("target");
                }
                dir = parent;
            }
            manifest_dir.join("../../target")
        });
    let dst_src_dir = target_root
        .join("win64-abi-aarch64")
        .join(&pkg_name)
        .join("src");
    let mut tracked = Vec::new();
    sync_dir(&src_dir, &dst_src_dir, &mut tracked);
    let dst_src_dir = dst_src_dir.canonicalize().unwrap_or(dst_src_dir);

    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut out: Vec<TokenTree> = Vec::with_capacity(tokens.len() * 3 + 64);

    let cfg_x86: Vec<TokenTree> = "#[cfg(target_arch = \"x86_64\")]"
        .parse::<TokenStream>()
        .unwrap()
        .into_iter()
        .collect();
    let cfg_non_x86: Vec<TokenTree> = "#[cfg(not(target_arch = \"x86_64\"))]"
        .parse::<TokenStream>()
        .unwrap()
        .into_iter()
        .collect();

    let mut stmt: Vec<TokenTree> = Vec::new();
    for tok in tokens {
        let is_semi = matches!(&tok, TokenTree::Punct(p) if p.as_char() == ';');
        stmt.push(tok);
        if !is_semi {
            continue;
        }
        let mod_idx = stmt.iter().position(|t| matches!(t, TokenTree::Ident(id) if id.to_string() == "mod"));
        if let Some(mi) = mod_idx {
            if mi + 1 < stmt.len() {
                if let TokenTree::Ident(mod_ident) = &stmt[mi + 1] {
                    let mod_name = mod_ident.to_string();
                    let src_file_cand = src_dir.join(format!("{mod_name}.rs"));
                    let file_cand = dst_src_dir.join(format!("{mod_name}.rs"));
                    let dir_cand = dst_src_dir.join(&mod_name).join("mod.rs");
                    let chosen = if src_file_cand.is_file() { file_cand } else { dir_cand };
                    let chosen_str = chosen.to_string_lossy();

                    // 1) #[cfg(target_arch = "x86_64")] original item
                    out.extend(cfg_x86.iter().cloned());
                    out.extend(stmt.iter().cloned());

                    // 2) #[cfg(not(target_arch = "x86_64"))] #[path = "..."] item
                    out.extend(cfg_non_x86.iter().cloned());
                    let mut attr_inner = TokenStream::new();
                    attr_inner.extend([
                        TokenTree::Ident(proc_macro::Ident::new("path", mod_ident.span())),
                        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                        TokenTree::Literal(Literal::string(&chosen_str)),
                    ]);
                    out.push(TokenTree::Punct(Punct::new('#', Spacing::Alone)));
                    out.push(TokenTree::Group(Group::new(Delimiter::Bracket, attr_inner)));
                    out.extend(stmt.iter().cloned());
                    stmt.clear();
                    continue;
                }
            }
        }
        out.extend(stmt.drain(..));
    }
    out.extend(stmt);

    let mut dep_code = String::from("#[cfg(not(target_arch = \"x86_64\"))]\nconst _: () = {\n");
    for p in tracked {
        let s = p.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\"");
        dep_code.push_str(&format!("    let _ = include_bytes!(\"{s}\");\n"));
    }
    dep_code.push_str("};\n");
    if let Ok(dep_ts) = dep_code.parse::<TokenStream>() {
        out.extend(dep_ts);
    }

    out.into_iter().collect()
}
