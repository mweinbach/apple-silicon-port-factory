//! HLSL preprocessor (`#define` / `#undef` / `#if` / `#elif` / `#ifdef` / `#include` / `#pragma once`).

use super::reflect::make_cstring;
use crate::types::{HRESULT, LPCSTR};
use std::collections::{HashMap, HashSet};
use std::ffi::c_void;
use std::path::{Path, PathBuf};

// D3D_COMPILE_STANDARD_FILE_INCLUDE ((ID3DInclude*)(UINT_PTR)1)
pub(super) const D3D_COMPILE_STANDARD_FILE_INCLUDE: usize = 1;

// D3D_INCLUDE_TYPE
const D3D_INCLUDE_LOCAL: u32 = 0;
const D3D_INCLUDE_SYSTEM: u32 = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct D3dShaderMacro {
    pub(super) name: LPCSTR,
    pub(super) definition: LPCSTR,
}

#[derive(Clone, Debug)]
struct MacroDef {
    params: Option<Vec<String>>,
    body: String,
}

enum IncludeMode {
    None,
    StandardFile,
    GuestCallback(*mut c_void),
}

struct Preprocessor {
    macros: HashMap<String, MacroDef>,
    include_mode: IncludeMode,
    pragma_once: HashSet<String>,
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_cont(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn splice_and_strip_comments(src: &str) -> String {
    // 1. Line splicing: `\` followed by `\r\n` or `\n`
    let mut spliced = String::with_capacity(src.len());
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 2;
                continue;
            }
            if i + 2 < bytes.len() && bytes[i + 1] == b'\r' && bytes[i + 2] == b'\n' {
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'\r' {
            if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                i += 1;
                continue;
            }
            spliced.push('\n');
            i += 1;
            continue;
        }
        let ch = src[i..].chars().next().unwrap();
        spliced.push(ch);
        i += ch.len_utf8();
    }

    // 2. Strip // and /* */ comments while preserving strings & line breaks
    let mut out = String::with_capacity(spliced.len());
    let sb = spliced.as_bytes();
    let mut j = 0;
    while j < sb.len() {
        if sb[j] == b'"' || sb[j] == b'\'' {
            let quote = sb[j];
            out.push(quote as char);
            j += 1;
            while j < sb.len() {
                let c = spliced[j..].chars().next().unwrap();
                out.push(c);
                j += c.len_utf8();
                if c == '\\' && j < sb.len() {
                    let esc = spliced[j..].chars().next().unwrap();
                    out.push(esc);
                    j += esc.len_utf8();
                } else if c as u32 == quote as u32 {
                    break;
                }
            }
            continue;
        }
        if j + 1 < sb.len() && sb[j] == b'/' && sb[j + 1] == b'/' {
            j += 2;
            while j < sb.len() && sb[j] != b'\n' {
                j += 1;
            }
            continue;
        }
        if j + 1 < sb.len() && sb[j] == b'/' && sb[j + 1] == b'*' {
            out.push(' ');
            j += 2;
            while j < sb.len() {
                if j + 1 < sb.len() && sb[j] == b'*' && sb[j + 1] == b'/' {
                    j += 2;
                    break;
                }
                if sb[j] == b'\n' {
                    out.push('\n');
                }
                j += 1;
            }
            continue;
        }
        let ch = spliced[j..].chars().next().unwrap();
        out.push(ch);
        j += ch.len_utf8();
    }
    out
}

fn parse_macro_call_args(s: &str, open_paren: usize) -> Option<(Vec<String>, usize)> {
    let bytes = s.as_bytes();
    if bytes.get(open_paren) != Some(&b'(') {
        return None;
    }
    let mut args = Vec::new();
    let mut depth = 0usize;
    let mut i = open_paren + 1;
    let mut cur = String::new();
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'"' || b == b'\'' {
            let q = b;
            cur.push(q as char);
            i += 1;
            while i < bytes.len() {
                let c = s[i..].chars().next().unwrap();
                cur.push(c);
                i += c.len_utf8();
                if c == '\\' && i < bytes.len() {
                    let esc = s[i..].chars().next().unwrap();
                    cur.push(esc);
                    i += esc.len_utf8();
                } else if c as u32 == q as u32 {
                    break;
                }
            }
            continue;
        }
        if b == b'(' {
            depth += 1;
            cur.push('(');
            i += 1;
        } else if b == b')' {
            if depth == 0 {
                let trimmed = cur.trim().to_string();
                if !(args.is_empty() && trimmed.is_empty()) {
                    args.push(trimmed);
                }
                return Some((args, i + 1));
            }
            depth -= 1;
            cur.push(')');
            i += 1;
        } else if b == b',' && depth == 0 {
            args.push(cur.trim().to_string());
            cur.clear();
            i += 1;
        } else {
            let ch = s[i..].chars().next().unwrap();
            cur.push(ch);
            i += ch.len_utf8();
        }
    }
    None
}

fn stringify_arg(arg: &str) -> String {
    let mut out = String::with_capacity(arg.len() + 2);
    out.push('"');
    for c in arg.chars() {
        if c == '\\' || c == '"' {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('"');
    out
}

fn substitute_fn_macro_body(body: &str, params: &[String], args: &[String]) -> String {
    // First substitute `#param` stringification and parameter tokens, preserving `##`.
    let mut out = String::with_capacity(body.len() + 16);
    let b = body.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'"' || b[i] == b'\'' {
            let q = b[i];
            out.push(q as char);
            i += 1;
            while i < b.len() {
                let c = body[i..].chars().next().unwrap();
                out.push(c);
                i += c.len_utf8();
                if c == '\\' && i < b.len() {
                    let esc = body[i..].chars().next().unwrap();
                    out.push(esc);
                    i += esc.len_utf8();
                } else if c as u32 == q as u32 {
                    break;
                }
            }
            continue;
        }
        // Token paste `##`: trim trailing whitespace from `out`, skip `##` and following whitespace
        if i + 1 < b.len() && b[i] == b'#' && b[i + 1] == b'#' {
            while out.ends_with(' ') || out.ends_with('\t') {
                out.pop();
            }
            i += 2;
            while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
                i += 1;
            }
            continue;
        }
        // Stringification `#param`
        if b[i] == b'#' {
            let mut k = i + 1;
            while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                k += 1;
            }
            if k < b.len() && is_ident_start(b[k]) {
                let start = k;
                while k < b.len() && is_ident_cont(b[k]) {
                    k += 1;
                }
                let ident = &body[start..k];
                if let Some(pidx) = params.iter().position(|p| p == ident) {
                    let val = args.get(pidx).map(String::as_str).unwrap_or("");
                    out.push_str(&stringify_arg(val));
                    i = k;
                    continue;
                }
            }
            out.push('#');
            i += 1;
            continue;
        }
        if is_ident_start(b[i]) {
            let start = i;
            i += 1;
            while i < b.len() && is_ident_cont(b[i]) {
                i += 1;
            }
            let ident = &body[start..i];
            if let Some(pidx) = params.iter().position(|p| p == ident) {
                let val = args.get(pidx).map(String::as_str).unwrap_or("");
                out.push_str(val);
            } else {
                out.push_str(ident);
            }
            continue;
        }
        let ch = body[i..].chars().next().unwrap();
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}

impl Preprocessor {
    fn new(defines: &[(String, String)], include_mode: IncludeMode) -> Self {
        let mut macros = HashMap::new();
        for (name, def) in defines {
            macros.insert(
                name.clone(),
                MacroDef {
                    params: None,
                    body: def.clone(),
                },
            );
        }
        Self {
            macros,
            include_mode,
            pragma_once: HashSet::new(),
        }
    }

    fn expand_text(
        &self,
        text: &str,
        file_name: &str,
        line_no: usize,
        hide: &HashSet<String>,
        depth: u32,
    ) -> String {
        if depth > 32 {
            return text.to_string();
        }
        let mut out = String::with_capacity(text.len());
        let b = text.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if b[i] == b'"' || b[i] == b'\'' {
                let q = b[i];
                out.push(q as char);
                i += 1;
                while i < b.len() {
                    let c = text[i..].chars().next().unwrap();
                    out.push(c);
                    i += c.len_utf8();
                    if c == '\\' && i < b.len() {
                        let esc = text[i..].chars().next().unwrap();
                        out.push(esc);
                        i += esc.len_utf8();
                    } else if c as u32 == q as u32 {
                        break;
                    }
                }
                continue;
            }
            if is_ident_start(b[i]) {
                let start = i;
                i += 1;
                while i < b.len() && is_ident_cont(b[i]) {
                    i += 1;
                }
                let ident = &text[start..i];
                if ident == "__LINE__" {
                    out.push_str(&line_no.to_string());
                    continue;
                }
                if ident == "__FILE__" {
                    out.push_str(&stringify_arg(file_name));
                    continue;
                }
                if !hide.contains(ident) {
                    if let Some(m) = self.macros.get(ident) {
                        if let Some(ref params) = m.params {
                            let mut k = i;
                            while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                                k += 1;
                            }
                            if let Some((raw_args, end_pos)) = parse_macro_call_args(text, k) {
                                let expanded_args: Vec<String> = raw_args
                                    .iter()
                                    .map(|a| {
                                        self.expand_text(a, file_name, line_no, hide, depth + 1)
                                    })
                                    .collect();
                                let sub = substitute_fn_macro_body(&m.body, params, &expanded_args);
                                let mut next_hide = hide.clone();
                                next_hide.insert(ident.to_string());
                                let res = self.expand_text(
                                    &sub,
                                    file_name,
                                    line_no,
                                    &next_hide,
                                    depth + 1,
                                );
                                out.push_str(&res);
                                i = end_pos;
                                continue;
                            }
                        } else {
                            let mut next_hide = hide.clone();
                            next_hide.insert(ident.to_string());
                            let res = self.expand_text(
                                &m.body,
                                file_name,
                                line_no,
                                &next_hide,
                                depth + 1,
                            );
                            out.push_str(&res);
                            continue;
                        }
                    }
                }
                out.push_str(ident);
                continue;
            }
            let ch = text[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
        out
    }

    fn expand_if_expr(&self, expr: &str, file_name: &str, line_no: usize) -> String {
        // Handle `defined(NAME)` / `defined NAME` before macro expansion.
        let mut pre = String::with_capacity(expr.len());
        let b = expr.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if is_ident_start(b[i]) {
                let start = i;
                i += 1;
                while i < b.len() && is_ident_cont(b[i]) {
                    i += 1;
                }
                let ident = &expr[start..i];
                if ident == "defined" {
                    let mut k = i;
                    while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                        k += 1;
                    }
                    let has_paren = k < b.len() && b[k] == b'(';
                    if has_paren {
                        k += 1;
                        while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                            k += 1;
                        }
                    }
                    let mstart = k;
                    while k < b.len() && is_ident_cont(b[k]) {
                        k += 1;
                    }
                    let mname = &expr[mstart..k];
                    if has_paren {
                        while k < b.len() && (b[k] == b' ' || b[k] == b'\t') {
                            k += 1;
                        }
                        if k < b.len() && b[k] == b')' {
                            k += 1;
                        }
                    }
                    pre.push(if self.macros.contains_key(mname) { '1' } else { '0' });
                    i = k;
                    continue;
                }
                pre.push_str(ident);
                continue;
            }
            let ch = expr[i..].chars().next().unwrap();
            pre.push(ch);
            i += ch.len_utf8();
        }
        self.expand_text(&pre, file_name, line_no, &HashSet::new(), 0)
    }

    fn eval_if_expr(&self, expr: &str, file_name: &str, line_no: usize) -> Result<bool, String> {
        let expanded = self.expand_if_expr(expr, file_name, line_no);
        let tokens = tokenize_expr(&expanded);
        let mut parser = ExprParser {
            tokens: &tokens,
            pos: 0,
        };
        let val = parser
            .parse_ternary()
            .map_err(|e| format!("{file_name}({line_no}): {e}"))?;
        Ok(val != 0)
    }

    fn resolve_standard_file(&self, current_file: &str, inc_path: &str) -> Option<(String, String)> {
        let inc_norm_slash = inc_path.replace('\\', "/");
        let mut candidates: Vec<PathBuf> = Vec::new();

        // 1. Relative to current file's directory (host path and virtual Windows path)
        if !current_file.is_empty() {
            if let Some(host_cur) = crate::path::to_host(current_file) {
                if let Some(dir) = host_cur.parent() {
                    candidates.push(dir.join(&inc_norm_slash));
                }
            }
            let p_cur = Path::new(current_file);
            if let Some(dir) = p_cur.parent() {
                if !dir.as_os_str().is_empty() {
                    candidates.push(dir.join(&inc_norm_slash));
                }
            }
            // Virtual Windows path join
            let win_dir = current_file
                .rsplit_once(['\\', '/'])
                .map(|(d, _)| d)
                .unwrap_or("");
            if !win_dir.is_empty() {
                let win_joined = format!("{win_dir}\\{inc_path}");
                if let Some(h) = crate::path::to_host(&win_joined) {
                    candidates.push(h);
                }
            }
        }

        // 2. Direct virtual Windows path or relative to virtual CWD
        if let Some(h) = crate::path::to_host(inc_path) {
            candidates.push(h);
        }

        // 3. Direct host path
        candidates.push(PathBuf::from(&inc_norm_slash));

        for cand in candidates {
            if cand.is_file() {
                if let Ok(bytes) = std::fs::read(&cand) {
                    let key = cand
                        .canonicalize()
                        .unwrap_or(cand)
                        .to_string_lossy()
                        .into_owned();
                    let text = String::from_utf8_lossy(&bytes).into_owned();
                    return Some((key, text));
                }
            }
        }
        None
    }

    unsafe fn preprocess_unit(
        &mut self,
        src: &str,
        file_name: &str,
        parent_data: *const c_void,
        depth: u32,
        out: &mut String,
    ) -> Result<(), String> {
        if depth > 64 {
            return Err(format!(
                "{file_name}(1): #include nesting depth exceeded limit (64)"
            ));
        }

        let cleaned = splice_and_strip_comments(src);

        // Conditional stack: (parent_active, this_branch_active, any_branch_taken, seen_else)
        let mut cond_stack: Vec<(bool, bool, bool, bool)> = Vec::new();
        let is_active = |st: &[(bool, bool, bool, bool)]| -> bool {
            st.last().map_or(true, |x| x.1)
        };

        for (idx, raw_line) in cleaned.split('\n').enumerate() {
            let line_no = idx + 1;
            let trimmed = raw_line.trim_start();

            if let Some(after_hash) = trimmed.strip_prefix('#') {
                let after_hash = after_hash.trim_start();
                let dir_end = after_hash
                    .bytes()
                    .take_while(|&b| is_ident_cont(b))
                    .count();
                let directive = &after_hash[..dir_end];
                let rest = after_hash[dir_end..].trim();

                match directive {
                    "ifdef" => {
                        let parent = is_active(&cond_stack);
                        let name = rest.split_whitespace().next().unwrap_or("");
                        let cond = parent && self.macros.contains_key(name);
                        cond_stack.push((parent, cond, cond, false));
                        out.push('\n');
                        continue;
                    }
                    "ifndef" => {
                        let parent = is_active(&cond_stack);
                        let name = rest.split_whitespace().next().unwrap_or("");
                        let cond = parent && !self.macros.contains_key(name);
                        cond_stack.push((parent, cond, cond, false));
                        out.push('\n');
                        continue;
                    }
                    "if" => {
                        let parent = is_active(&cond_stack);
                        let cond = if parent {
                            self.eval_if_expr(rest, file_name, line_no)?
                        } else {
                            false
                        };
                        cond_stack.push((parent, cond, cond, false));
                        out.push('\n');
                        continue;
                    }
                    "elif" => {
                        let Some((parent, active, taken, seen_else)) = cond_stack.last_mut() else {
                            return Err(format!("{file_name}({line_no}): #elif without #if"));
                        };
                        if *seen_else {
                            return Err(format!("{file_name}({line_no}): #elif after #else"));
                        }
                        if *parent && !*taken {
                            let cond = self.eval_if_expr(rest, file_name, line_no)?;
                            *active = cond;
                            if cond {
                                *taken = true;
                            }
                        } else {
                            *active = false;
                        }
                        out.push('\n');
                        continue;
                    }
                    "else" => {
                        let Some((parent, active, taken, seen_else)) = cond_stack.last_mut() else {
                            return Err(format!("{file_name}({line_no}): #else without #if"));
                        };
                        if *seen_else {
                            return Err(format!("{file_name}({line_no}): duplicate #else"));
                        }
                        *seen_else = true;
                        *active = *parent && !*taken;
                        *taken = true;
                        out.push('\n');
                        continue;
                    }
                    "endif" => {
                        if cond_stack.pop().is_none() {
                            return Err(format!("{file_name}({line_no}): #endif without #if"));
                        }
                        out.push('\n');
                        continue;
                    }
                    _ => {}
                }

                if !is_active(&cond_stack) {
                    out.push('\n');
                    continue;
                }

                match directive {
                    "" => {
                        // Null directive `#`
                        out.push('\n');
                    }
                    "define" => {
                        let b = rest.as_bytes();
                        let name_len = b.iter().take_while(|&&c| is_ident_cont(c)).count();
                        if name_len == 0 {
                            return Err(format!("{file_name}({line_no}): malformed #define"));
                        }
                        let name = rest[..name_len].to_string();
                        let after_name = &rest[name_len..];
                        if let Some(after_lparen) = after_name.strip_prefix('(') {
                            let Some(rparen) = after_lparen.find(')') else {
                                return Err(format!(
                                    "{file_name}({line_no}): unterminated #define parameter list"
                                ));
                            };
                            let plist = &after_lparen[..rparen];
                            let params: Vec<String> = if plist.trim().is_empty() {
                                Vec::new()
                            } else {
                                plist.split(',').map(|p| p.trim().to_string()).collect()
                            };
                            let body = after_lparen[rparen + 1..].trim().to_string();
                            self.macros.insert(
                                name,
                                MacroDef {
                                    params: Some(params),
                                    body,
                                },
                            );
                        } else {
                            let body = after_name.trim().to_string();
                            self.macros.insert(name, MacroDef { params: None, body });
                        }
                        out.push('\n');
                    }
                    "undef" => {
                        let name = rest.split_whitespace().next().unwrap_or("");
                        self.macros.remove(name);
                        out.push('\n');
                    }
                    "include" => {
                        let inc_expr = if rest.starts_with('"') || rest.starts_with('<') {
                            rest.to_string()
                        } else {
                            self.expand_text(rest, file_name, line_no, &HashSet::new(), 0)
                        };
                        let inc_trim = inc_expr.trim();
                        let (inc_type, inc_name) = if let Some(s) = inc_trim.strip_prefix('"') {
                            let Some(end) = s.find('"') else {
                                return Err(format!(
                                    "{file_name}({line_no}): unterminated #include filename"
                                ));
                            };
                            (D3D_INCLUDE_LOCAL, &s[..end])
                        } else if let Some(s) = inc_trim.strip_prefix('<') {
                            let Some(end) = s.find('>') else {
                                return Err(format!(
                                    "{file_name}({line_no}): unterminated #include <filename>"
                                ));
                            };
                            (D3D_INCLUDE_SYSTEM, &s[..end])
                        } else {
                            return Err(format!(
                                "{file_name}({line_no}): invalid #include syntax: {inc_trim}"
                            ));
                        };

                        self.handle_include(
                            inc_type,
                            inc_name,
                            file_name,
                            line_no,
                            parent_data,
                            depth,
                            out,
                        )?;
                    }
                    "pragma" => {
                        if rest.split_whitespace().next() == Some("once") {
                            self.pragma_once.insert(file_name.to_string());
                            out.push('\n');
                        } else {
                            out.push_str(raw_line);
                            out.push('\n');
                        }
                    }
                    "error" => {
                        return Err(format!("{file_name}({line_no}): #error {rest}"));
                    }
                    "line" => {
                        out.push_str(raw_line);
                        out.push('\n');
                    }
                    _ => {
                        out.push_str(raw_line);
                        out.push('\n');
                    }
                }
                continue;
            }

            if !is_active(&cond_stack) {
                out.push('\n');
                continue;
            }

            let expanded = self.expand_text(raw_line, file_name, line_no, &HashSet::new(), 0);
            out.push_str(&expanded);
            out.push('\n');
        }

        if !cond_stack.is_empty() {
            return Err(format!("{file_name}: unterminated #if/#ifdef directive"));
        }

        Ok(())
    }

    unsafe fn handle_include(
        &mut self,
        inc_type: u32,
        inc_name: &str,
        current_file: &str,
        line_no: usize,
        parent_data: *const c_void,
        depth: u32,
        out: &mut String,
    ) -> Result<(), String> {
        match self.include_mode {
            IncludeMode::None => Err(format!(
                "{current_file}({line_no}): cannot open include file '{inc_name}': no ID3DInclude handler"
            )),
            IncludeMode::StandardFile => {
                let Some((resolved_key, text)) = self.resolve_standard_file(current_file, inc_name)
                else {
                    return Err(format!(
                        "{current_file}({line_no}): cannot open include file '{inc_name}'"
                    ));
                };
                if self.pragma_once.contains(&resolved_key) {
                    out.push('\n');
                    return Ok(());
                }
                self.preprocess_unit(&text, &resolved_key, std::ptr::null(), depth + 1, out)
            }
            IncludeMode::GuestCallback(inc_obj) => {
                if self.pragma_once.contains(inc_name) {
                    out.push('\n');
                    return Ok(());
                }
                let vtbl = std::ptr::read_unaligned(inc_obj as *const usize);
                if vtbl == 0 {
                    return Err(format!(
                        "{current_file}({line_no}): invalid ID3DInclude vtable"
                    ));
                }
                let open_fn = std::ptr::read_unaligned(vtbl as *const usize);
                let close_fn = std::ptr::read_unaligned((vtbl + 8) as *const usize);
                let cname = make_cstring(inc_name);
                let mut data_ptr: *const c_void = std::ptr::null();
                let mut data_bytes: u32 = 0;

                let hr = crate::call_guest8(
                    open_fn,
                    inc_obj as u64,
                    inc_type as u64,
                    cname.as_ptr() as u64,
                    parent_data as u64,
                    (&mut data_ptr as *mut *const c_void) as u64,
                    (&mut data_bytes as *mut u32) as u64,
                    0,
                    0,
                ) as HRESULT;

                if hr < 0 {
                    return Err(format!(
                        "{current_file}({line_no}): cannot open include file '{inc_name}' (hr={hr:#010x})"
                    ));
                }

                let inc_text = if !data_ptr.is_null() && data_bytes > 0 {
                    let slice =
                        std::slice::from_raw_parts(data_ptr as *const u8, data_bytes as usize);
                    String::from_utf8_lossy(slice).into_owned()
                } else {
                    String::new()
                };

                let res = self.preprocess_unit(&inc_text, inc_name, data_ptr, depth + 1, out);

                if !data_ptr.is_null() && close_fn != 0 {
                    let _ = crate::call_guest8(
                        close_fn,
                        inc_obj as u64,
                        data_ptr as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    );
                }

                res
            }
        }
    }
}

// --- Preprocessor `#if` constant integer expression evaluator ---

#[derive(Clone, Debug, PartialEq, Eq)]
enum ExprTok {
    Num(i64),
    LParen,
    RParen,
    Question,
    Colon,
    OrOr,
    AndAnd,
    BitOr,
    BitXor,
    BitAnd,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Shl,
    Shr,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Tilde,
}

fn tokenize_expr(s: &str) -> Vec<ExprTok> {
    let mut toks = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b[i].is_ascii_digit() {
            let start = i;
            if b[i] == b'0' && i + 1 < b.len() && matches!(b[i + 1], b'x' | b'X') {
                i += 2;
                while i < b.len() && b[i].is_ascii_hexdigit() {
                    i += 1;
                }
                let val = i64::from_str_radix(&s[start + 2..i], 16).unwrap_or(0);
                while i < b.len() && matches!(b[i], b'u' | b'U' | b'l' | b'L') {
                    i += 1;
                }
                toks.push(ExprTok::Num(val));
            } else {
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
                let raw = &s[start..i];
                let val = if raw.len() > 1 && raw.starts_with('0') {
                    i64::from_str_radix(raw, 8).unwrap_or(0)
                } else {
                    raw.parse::<i64>().unwrap_or(0)
                };
                while i < b.len() && matches!(b[i], b'u' | b'U' | b'l' | b'L') {
                    i += 1;
                }
                toks.push(ExprTok::Num(val));
            }
            continue;
        }
        if b[i] == b'\'' {
            i += 1;
            let mut val = 0i64;
            if i < b.len() {
                if b[i] == b'\\' && i + 1 < b.len() {
                    i += 1;
                    val = match b[i] {
                        b'n' => b'\n' as i64,
                        b'r' => b'\r' as i64,
                        b't' => b'\t' as i64,
                        b'0' => 0,
                        c => c as i64,
                    };
                } else {
                    val = b[i] as i64;
                }
                while i < b.len() && b[i] != b'\'' {
                    i += 1;
                }
                if i < b.len() {
                    i += 1;
                }
            }
            toks.push(ExprTok::Num(val));
            continue;
        }
        if is_ident_start(b[i]) {
            let start = i;
            i += 1;
            while i < b.len() && is_ident_cont(b[i]) {
                i += 1;
            }
            let ident = &s[start..i];
            toks.push(ExprTok::Num(if ident == "true" { 1 } else { 0 }));
            continue;
        }
        let two = b.get(i..i + 2);
        if let Some(pair) = two {
            let matched = match pair {
                b"||" => Some(ExprTok::OrOr),
                b"&&" => Some(ExprTok::AndAnd),
                b"==" => Some(ExprTok::Eq),
                b"!=" => Some(ExprTok::Ne),
                b"<=" => Some(ExprTok::Le),
                b">=" => Some(ExprTok::Ge),
                b"<<" => Some(ExprTok::Shl),
                b">>" => Some(ExprTok::Shr),
                _ => None,
            };
            if let Some(t) = matched {
                toks.push(t);
                i += 2;
                continue;
            }
        }
        let one = match b[i] {
            b'(' => Some(ExprTok::LParen),
            b')' => Some(ExprTok::RParen),
            b'?' => Some(ExprTok::Question),
            b':' => Some(ExprTok::Colon),
            b'|' => Some(ExprTok::BitOr),
            b'^' => Some(ExprTok::BitXor),
            b'&' => Some(ExprTok::BitAnd),
            b'<' => Some(ExprTok::Lt),
            b'>' => Some(ExprTok::Gt),
            b'+' => Some(ExprTok::Plus),
            b'-' => Some(ExprTok::Minus),
            b'*' => Some(ExprTok::Star),
            b'/' => Some(ExprTok::Slash),
            b'%' => Some(ExprTok::Percent),
            b'!' => Some(ExprTok::Bang),
            b'~' => Some(ExprTok::Tilde),
            _ => None,
        };
        if let Some(t) = one {
            toks.push(t);
        }
        i += 1;
    }
    toks
}

struct ExprParser<'a> {
    tokens: &'a [ExprTok],
    pos: usize,
}

impl<'a> ExprParser<'a> {
    fn peek(&self) -> Option<&ExprTok> {
        self.tokens.get(self.pos)
    }
    fn bump(&mut self) -> Option<ExprTok> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }
    fn parse_ternary(&mut self) -> Result<i64, String> {
        let cond = self.parse_or_or()?;
        if self.peek() == Some(&ExprTok::Question) {
            self.bump();
            let t_val = self.parse_ternary()?;
            if self.bump() != Some(ExprTok::Colon) {
                return Err("expected ':' in ternary expression".into());
            }
            let f_val = self.parse_ternary()?;
            Ok(if cond != 0 { t_val } else { f_val })
        } else {
            Ok(cond)
        }
    }
    fn parse_or_or(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_and_and()?;
        while self.peek() == Some(&ExprTok::OrOr) {
            self.bump();
            let rhs = self.parse_and_and()?;
            lhs = i64::from(lhs != 0 || rhs != 0);
        }
        Ok(lhs)
    }
    fn parse_and_and(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_bit_or()?;
        while self.peek() == Some(&ExprTok::AndAnd) {
            self.bump();
            let rhs = self.parse_bit_or()?;
            lhs = i64::from(lhs != 0 && rhs != 0);
        }
        Ok(lhs)
    }
    fn parse_bit_or(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_bit_xor()?;
        while self.peek() == Some(&ExprTok::BitOr) {
            self.bump();
            lhs |= self.parse_bit_xor()?;
        }
        Ok(lhs)
    }
    fn parse_bit_xor(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_bit_and()?;
        while self.peek() == Some(&ExprTok::BitXor) {
            self.bump();
            lhs ^= self.parse_bit_and()?;
        }
        Ok(lhs)
    }
    fn parse_bit_and(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_eq()?;
        while self.peek() == Some(&ExprTok::BitAnd) {
            self.bump();
            lhs &= self.parse_eq()?;
        }
        Ok(lhs)
    }
    fn parse_eq(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_rel()?;
        loop {
            match self.peek() {
                Some(ExprTok::Eq) => {
                    self.bump();
                    lhs = i64::from(lhs == self.parse_rel()?);
                }
                Some(ExprTok::Ne) => {
                    self.bump();
                    lhs = i64::from(lhs != self.parse_rel()?);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_rel(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_shift()?;
        loop {
            match self.peek() {
                Some(ExprTok::Lt) => {
                    self.bump();
                    lhs = i64::from(lhs < self.parse_shift()?);
                }
                Some(ExprTok::Le) => {
                    self.bump();
                    lhs = i64::from(lhs <= self.parse_shift()?);
                }
                Some(ExprTok::Gt) => {
                    self.bump();
                    lhs = i64::from(lhs > self.parse_shift()?);
                }
                Some(ExprTok::Ge) => {
                    self.bump();
                    lhs = i64::from(lhs >= self.parse_shift()?);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_shift(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_add()?;
        loop {
            match self.peek() {
                Some(ExprTok::Shl) => {
                    self.bump();
                    let rhs = self.parse_add()?;
                    lhs = lhs.wrapping_shl((rhs & 63) as u32);
                }
                Some(ExprTok::Shr) => {
                    self.bump();
                    let rhs = self.parse_add()?;
                    lhs = lhs.wrapping_shr((rhs & 63) as u32);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_add(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_mul()?;
        loop {
            match self.peek() {
                Some(ExprTok::Plus) => {
                    self.bump();
                    lhs = lhs.wrapping_add(self.parse_mul()?);
                }
                Some(ExprTok::Minus) => {
                    self.bump();
                    lhs = lhs.wrapping_sub(self.parse_mul()?);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_mul(&mut self) -> Result<i64, String> {
        let mut lhs = self.parse_unary()?;
        loop {
            match self.peek() {
                Some(ExprTok::Star) => {
                    self.bump();
                    lhs = lhs.wrapping_mul(self.parse_unary()?);
                }
                Some(ExprTok::Slash) => {
                    self.bump();
                    let rhs = self.parse_unary()?;
                    lhs = if rhs == 0 { 0 } else { lhs.wrapping_div(rhs) };
                }
                Some(ExprTok::Percent) => {
                    self.bump();
                    let rhs = self.parse_unary()?;
                    lhs = if rhs == 0 { 0 } else { lhs.wrapping_rem(rhs) };
                }
                _ => break,
            }
        }
        Ok(lhs)
    }
    fn parse_unary(&mut self) -> Result<i64, String> {
        match self.peek() {
            Some(ExprTok::Plus) => {
                self.bump();
                self.parse_unary()
            }
            Some(ExprTok::Minus) => {
                self.bump();
                Ok(self.parse_unary()?.wrapping_neg())
            }
            Some(ExprTok::Bang) => {
                self.bump();
                Ok(i64::from(self.parse_unary()? == 0))
            }
            Some(ExprTok::Tilde) => {
                self.bump();
                Ok(!self.parse_unary()?)
            }
            _ => self.parse_primary(),
        }
    }
    fn parse_primary(&mut self) -> Result<i64, String> {
        match self.bump() {
            Some(ExprTok::Num(n)) => Ok(n),
            Some(ExprTok::LParen) => {
                let v = self.parse_ternary()?;
                if self.bump() != Some(ExprTok::RParen) {
                    return Err("missing closing ')' in preprocessor expression".into());
                }
                Ok(v)
            }
            _ => Err("unexpected token in preprocessor #if expression".into()),
        }
    }
}

pub(super) unsafe fn read_guest_defines(defines: *const c_void) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if defines.is_null() {
        return out;
    }
    let mut cur = defines as *const D3dShaderMacro;
    for _ in 0..4096 {
        let entry = std::ptr::read_unaligned(cur);
        if entry.name.is_null() {
            break;
        }
        let name = crate::strings::from_ansi(entry.name);
        let def = if entry.definition.is_null() {
            String::new()
        } else {
            crate::strings::from_ansi(entry.definition)
        };
        if !name.is_empty() {
            out.push((name, def));
        }
        cur = cur.add(1);
    }
    out
}

pub(super) unsafe fn preprocess_hlsl_source(
    src: &str,
    source_name: &str,
    defines: &[(String, String)],
    include: *mut c_void,
) -> Result<String, String> {
    let mode = if include.is_null() {
        IncludeMode::None
    } else if include as usize == D3D_COMPILE_STANDARD_FILE_INCLUDE {
        IncludeMode::StandardFile
    } else {
        IncludeMode::GuestCallback(include)
    };
    let mut pp = Preprocessor::new(defines, mode);
    let mut out = String::with_capacity(src.len() + 64);
    pp.preprocess_unit(src, source_name, std::ptr::null(), 0, &mut out)?;
    Ok(out)
}
