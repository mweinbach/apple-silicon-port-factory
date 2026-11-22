//! Symbol tables for crash reports and `symbolize()`.
//!
//! * PE images: `out/symbols/<image>.syms` files written by
//!   `tools/debug/export_symbols.py` from the function DB (fndb names,
//!   owner and decomp status). Override the directory with `AW2_SYMBOLS=<dir>`
//!   or point it at a single file; `AW2_SYMBOLS=0` disables loading.
//! * Host code: `dladdr` names, demangled here (Rust legacy `_ZN...E`
//!   mangling; v0 `_R` names are left as is).
//!
//! The tables are parsed on a background thread at startup ([`preload`]) so
//! that symbolization never stalls boot; [`get`] is non-blocking (returns
//! `None` until parsing finishes) and [`wait`] blocks (crash reports).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Condvar, Mutex, OnceLock};

/// One function from a `.syms` file.
pub struct Sym {
    pub rva: u32,
    pub size: u32,
    owner: u16,
    status: u16,
    pub name: Box<str>,
}

/// Symbols of one image (sorted by RVA).
pub struct SymTable {
    pub image: String,
    pub base: u64,
    syms: Vec<Sym>,
    strings: Vec<Box<str>>,
}

/// A resolved symbol: `name+off {owner, status}`.
pub struct Hit<'a> {
    pub table: &'a SymTable,
    pub sym: &'a Sym,
    pub off: u32,
}

impl Hit<'_> {
    pub fn owner(&self) -> &str {
        &self.table.strings[self.sym.owner as usize]
    }
    pub fn status(&self) -> &str {
        &self.table.strings[self.sym.status as usize]
    }
}

impl std::fmt::Display for Hit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.sym.name.is_empty() {
            write!(f, "sub_{:x}", self.table.base + self.sym.rva as u64)?;
        } else {
            f.write_str(&self.sym.name)?;
        }
        if self.off != 0 {
            write!(f, "+0x{:x}", self.off)?;
        }
        match self.status() {
            "-" | "" => write!(f, " {{{}}}", self.owner()),
            s => write!(f, " {{{}, {s}}}", self.owner()),
        }
    }
}

impl SymTable {
    pub fn parse(text: &str) -> Result<SymTable, String> {
        let mut lines = text.lines();
        let header = lines.next().ok_or("empty symbol file")?;
        if !header.starts_with("# aw2-syms v1") {
            return Err(format!("bad header {header:?}"));
        }
        let field = |k: &str| header.split_whitespace().find_map(|w| w.strip_prefix(k)).map(str::to_string);
        let image = field("image=").ok_or("header lacks image=")?;
        let base = field("base=")
            .and_then(|b| u64::from_str_radix(b.trim_start_matches("0x"), 16).ok())
            .ok_or("header lacks base=")?;
        let count: usize = field("count=").and_then(|c| c.parse().ok()).unwrap_or(0);
        let mut strings: Vec<Box<str>> = Vec::new();
        let mut interned: HashMap<String, u16> = HashMap::new();
        let mut intern = |s: &str| -> u16 {
            if let Some(&i) = interned.get(s) {
                return i;
            }
            let i = strings.len().min(u16::MAX as usize) as u16;
            strings.push(s.into());
            interned.insert(s.to_string(), i);
            i
        };
        let mut syms = Vec::with_capacity(count);
        for (n, line) in lines.enumerate() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut it = line.splitn(5, '\t');
            let mut next = || it.next().ok_or_else(|| format!("line {}: too few fields", n + 2));
            let rva = u32::from_str_radix(next()?, 16).map_err(|e| format!("line {}: {e}", n + 2))?;
            let size = u32::from_str_radix(next()?, 16).map_err(|e| format!("line {}: {e}", n + 2))?;
            let owner = intern(next()?);
            let status = intern(next()?);
            let name = next().unwrap_or("").into();
            syms.push(Sym { rva, size, owner, status, name });
        }
        syms.sort_by_key(|s| s.rva);
        Ok(SymTable { image, base, syms, strings })
    }

    pub fn len(&self) -> usize {
        self.syms.len()
    }

    /// Function containing `rva` (functions without a size match up to
    /// 64 KiB past their start).
    pub fn lookup(&self, rva: u32) -> Option<Hit<'_>> {
        let i = self.syms.partition_point(|s| s.rva <= rva).checked_sub(1)?;
        let sym = &self.syms[i];
        let off = rva - sym.rva;
        let limit = if sym.size != 0 { sym.size } else { 0x10000 };
        (off < limit).then_some(Hit { table: self, sym, off })
    }
}

/// All loaded tables, keyed by lowercase image name.
pub struct Symbols {
    pub tables: HashMap<String, SymTable>,
}

impl Symbols {
    pub fn for_image(&self, image: &str) -> Option<&SymTable> {
        self.tables.get(&image.to_ascii_lowercase())
    }
}

static SYMS: OnceLock<Symbols> = OnceLock::new();
static STARTED: Mutex<bool> = Mutex::new(false);
static DONE: Condvar = Condvar::new();

fn default_dir() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for rel in ["symbols", "../Resources/symbols"] {
                let bundled = dir.join(rel);
                if bundled.is_dir() {
                    return bundled;
                }
            }
        }
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../out/symbols")
}

fn load_all() -> Symbols {
    let mut tables = HashMap::new();
    let spec = std::env::var_os("AW2_SYMBOLS").map(PathBuf::from);
    if spec.as_deref() == Some(Path::new("0")) {
        return Symbols { tables };
    }
    let path = spec.unwrap_or_else(default_dir);
    let files: Vec<PathBuf> = if path.is_file() {
        vec![path.clone()]
    } else {
        std::fs::read_dir(&path)
            .map(|d| {
                d.flatten()
                    .map(|e| e.path())
                    .filter(|p| {
                        p.extension().is_some_and(|e| e == "syms")
                            && !p
                                .file_name()
                                .is_some_and(|n| n.to_string_lossy().starts_with("._"))
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let t0 = std::time::Instant::now();
    for f in files {
        match std::fs::read_to_string(&f).map_err(|e| e.to_string()).and_then(|t| SymTable::parse(&t)) {
            Ok(t) => {
                log::debug!("symbols: {} ({} functions) from {}", t.image, t.len(), f.display());
                tables.insert(t.image.to_ascii_lowercase(), t);
            }
            Err(e) => log::warn!("symbols: {}: {e}", f.display()),
        }
    }
    if tables.is_empty() {
        log::debug!("symbols: none under {} (run tools/debug/export_symbols.py)", path.display());
    } else {
        log::info!("symbols: loaded {} table(s) in {:?}", tables.len(), t0.elapsed());
    }
    Symbols { tables }
}

/// Start parsing the symbol files on a background thread.
pub fn preload() {
    let mut started = STARTED.lock().unwrap();
    if *started {
        return;
    }
    *started = true;
    std::thread::Builder::new()
        .name("aw2-symbols".into())
        .spawn(|| {
            let _ = SYMS.set(load_all());
            let _g = STARTED.lock().unwrap();
            DONE.notify_all();
        })
        .ok();
}

/// Non-blocking: the tables if parsing has finished.
pub fn get() -> Option<&'static Symbols> {
    SYMS.get()
}

/// Blocking: parse now (or wait for the preload thread, at most 10 s).
pub fn wait() -> Option<&'static Symbols> {
    if let Some(s) = SYMS.get() {
        return Some(s);
    }
    let started = STARTED.lock().unwrap();
    if !*started {
        drop(started);
        return Some(SYMS.get_or_init(load_all));
    }
    let _ = DONE.wait_timeout_while(started, std::time::Duration::from_secs(10), |_| SYMS.get().is_none());
    SYMS.get()
}

// ---------------------------------------------------------------------------
// Rust symbol demangling (v0 and legacy schemes)
// ---------------------------------------------------------------------------

/// Demangle a Rust symbol: v0 (`_RNvNtCs..._9aw2_win326thread11thread_main`
/// -> `win32_rt::thread::thread_main`) or legacy
/// (`_ZN3foo3bar17h0123456789abcdefE` -> `foo::bar`). Crate disambiguators
/// and hashes are dropped. Other names are returned unchanged.
pub fn demangle(sym: &str) -> String {
    demangle_v0(sym).or_else(|| demangle_legacy(sym)).unwrap_or_else(|| sym.to_string())
}

fn demangle_v0(sym: &str) -> Option<String> {
    let s = sym.strip_prefix("__R").or_else(|| sym.strip_prefix("_R"))?;
    let b = s.as_bytes();
    if !b.first()?.is_ascii_uppercase() && !b[0].is_ascii_digit() {
        return None;
    }
    let mut d = V0 { s: b, pos: 0, out: String::new(), skip: 0, depth: 0 };
    // Optional encoding version (decimal) before the path.
    while d.peek().is_some_and(|c| c.is_ascii_digit()) {
        d.pos += 1;
    }
    d.path().ok()?;
    Some(d.out)
}

/// Minimal v0 printer (RFC 2603). Backrefs are offsets from just after `_R`.
struct V0<'a> {
    s: &'a [u8],
    pos: usize,
    out: String,
    /// >0 while parsing parts that are not printed (impl paths).
    skip: u32,
    depth: u32,
}

type V0R = Result<(), ()>;

impl V0<'_> {
    fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }
    fn next(&mut self) -> Result<u8, ()> {
        let c = self.peek().ok_or(())?;
        self.pos += 1;
        Ok(c)
    }
    fn eat(&mut self, c: u8) -> bool {
        if self.peek() == Some(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn print(&mut self, s: &str) {
        if self.skip == 0 {
            self.out.push_str(s);
        }
    }
    fn b62(&mut self) -> Result<u64, ()> {
        if self.eat(b'_') {
            return Ok(0);
        }
        let mut v: u64 = 0;
        loop {
            let c = self.next()?;
            let d = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'z' => c - b'a' + 10,
                b'A'..=b'Z' => c - b'A' + 36,
                b'_' => return v.checked_add(1).ok_or(()),
                _ => return Err(()),
            } as u64;
            v = v.checked_mul(62).and_then(|v| v.checked_add(d)).ok_or(())?;
        }
    }
    fn opt_b62(&mut self, tag: u8) -> Result<u64, ()> {
        if self.eat(tag) {
            Ok(self.b62()? + 1)
        } else {
            Ok(0)
        }
    }
    fn undis_ident(&mut self) -> Result<String, ()> {
        let puny = self.eat(b'u');
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        let len: usize = std::str::from_utf8(&self.s[start..self.pos]).map_err(|_| ())?.parse().map_err(|_| ())?;
        self.eat(b'_');
        let bytes = self.s.get(self.pos..self.pos + len).ok_or(())?;
        self.pos += len;
        let name = String::from_utf8_lossy(bytes).into_owned();
        Ok(if puny { format!("{{punycode {name}}}") } else { name })
    }
    fn backref(&mut self, f: fn(&mut Self) -> V0R) -> V0R {
        let target = self.b62()? as usize;
        if target >= self.pos || self.depth > 64 {
            return Err(());
        }
        let saved = self.pos;
        self.pos = target;
        self.depth += 1;
        let r = f(self);
        self.depth -= 1;
        self.pos = saved;
        r
    }
    fn path(&mut self) -> V0R {
        match self.next()? {
            b'C' => {
                self.opt_b62(b's')?;
                let n = self.undis_ident()?;
                self.print(&n);
            }
            b'N' => {
                let ns = self.next()?;
                self.path()?;
                let dis = self.opt_b62(b's')?;
                let n = self.undis_ident()?;
                if ns.is_ascii_uppercase() {
                    let kind = match ns {
                        b'C' => "closure".to_string(),
                        b'S' => "shim".to_string(),
                        c => (c as char).to_string(),
                    };
                    let name = if n.is_empty() { String::new() } else { format!(":{n}") };
                    self.print(&format!("::{{{kind}{name}#{dis}}}"));
                } else if !n.is_empty() {
                    self.print("::");
                    self.print(&n);
                }
            }
            b'M' => {
                self.impl_path()?;
                self.print("<");
                self.ty()?;
                self.print(">");
            }
            b'X' => {
                self.impl_path()?;
                self.print("<");
                self.ty()?;
                self.print(" as ");
                self.path()?;
                self.print(">");
            }
            b'Y' => {
                self.print("<");
                self.ty()?;
                self.print(" as ");
                self.path()?;
                self.print(">");
            }
            b'I' => {
                self.path()?;
                self.print("<");
                let mut first = true;
                while !self.eat(b'E') {
                    if !first {
                        self.print(", ");
                    }
                    first = false;
                    self.generic_arg()?;
                }
                self.print(">");
            }
            b'B' => self.backref(Self::path)?,
            _ => return Err(()),
        }
        Ok(())
    }
    fn impl_path(&mut self) -> V0R {
        self.skip += 1;
        let r = self.opt_b62(b's').and_then(|_| self.path());
        self.skip -= 1;
        r
    }
    fn generic_arg(&mut self) -> V0R {
        if self.eat(b'L') {
            self.b62()?;
            self.print("'_");
            Ok(())
        } else if self.eat(b'K') {
            self.konst()
        } else {
            self.ty()
        }
    }
    fn basic(c: u8) -> Option<&'static str> {
        Some(match c {
            b'a' => "i8",
            b'b' => "bool",
            b'c' => "char",
            b'd' => "f64",
            b'e' => "str",
            b'f' => "f32",
            b'h' => "u8",
            b'i' => "isize",
            b'j' => "usize",
            b'l' => "i32",
            b'm' => "u32",
            b'n' => "i128",
            b'o' => "u128",
            b's' => "i16",
            b't' => "u16",
            b'u' => "()",
            b'v' => "...",
            b'x' => "i64",
            b'y' => "u64",
            b'z' => "!",
            b'p' => "_",
            _ => return None,
        })
    }
    fn ty(&mut self) -> V0R {
        let c = self.next()?;
        if let Some(b) = Self::basic(c) {
            self.print(b);
            return Ok(());
        }
        match c {
            b'A' | b'S' => {
                self.print("[");
                self.ty()?;
                if c == b'A' {
                    self.print("; ");
                    self.konst()?;
                }
                self.print("]");
            }
            b'T' => {
                self.print("(");
                let mut n = 0;
                while !self.eat(b'E') {
                    if n > 0 {
                        self.print(", ");
                    }
                    self.ty()?;
                    n += 1;
                }
                if n == 1 {
                    self.print(",");
                }
                self.print(")");
            }
            b'R' | b'Q' => {
                if self.eat(b'L') {
                    self.b62()?;
                }
                self.print(if c == b'R' { "&" } else { "&mut " });
                self.ty()?;
            }
            b'P' | b'O' => {
                self.print(if c == b'P' { "*const " } else { "*mut " });
                self.ty()?;
            }
            b'F' => {
                self.opt_b62(b'G')?;
                if self.eat(b'U') {
                    self.print("unsafe ");
                }
                if self.eat(b'K') {
                    let abi = if self.eat(b'C') { "C".to_string() } else { self.undis_ident()?.replace('_', "-") };
                    self.print(&format!("extern \"{abi}\" "));
                }
                self.print("fn(");
                let mut first = true;
                while !self.eat(b'E') {
                    if !first {
                        self.print(", ");
                    }
                    first = false;
                    self.ty()?;
                }
                self.print(")");
                if self.peek() == Some(b'u') {
                    self.pos += 1;
                } else {
                    self.print(" -> ");
                    self.ty()?;
                }
            }
            b'D' => {
                self.opt_b62(b'G')?;
                self.print("dyn ");
                let mut first = true;
                while !self.eat(b'E') {
                    if !first {
                        self.print(" + ");
                    }
                    first = false;
                    self.path()?;
                    while self.eat(b'p') {
                        let n = self.undis_ident()?;
                        self.print(&format!("<{n} = "));
                        self.ty()?;
                        self.print(">");
                    }
                }
                if self.eat(b'L') {
                    self.b62()?;
                }
            }
            b'B' => self.backref(Self::ty)?,
            _ => {
                self.pos -= 1;
                self.path()?;
            }
        }
        Ok(())
    }
    fn konst(&mut self) -> V0R {
        if self.eat(b'B') {
            return self.backref(Self::konst);
        }
        if self.eat(b'p') {
            self.print("_");
            return Ok(());
        }
        let t = self.next()?;
        let neg = self.eat(b'n');
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
            self.pos += 1;
        }
        let hex = std::str::from_utf8(&self.s[start..self.pos]).map_err(|_| ())?.to_string();
        if !self.eat(b'_') {
            return Err(());
        }
        let v = u128::from_str_radix(if hex.is_empty() { "0" } else { &hex }, 16).unwrap_or(0);
        let text = match t {
            b'b' => (v != 0).to_string(),
            b'c' => char::from_u32(v as u32).map_or(format!("0x{hex}"), |c| format!("{c:?}")),
            _ => format!("{}{v}", if neg { "-" } else { "" }),
        };
        self.print(&text);
        Ok(())
    }
}

fn demangle_legacy(sym: &str) -> Option<String> {
    // macOS prefixes C symbols with '_' (dladdr strips it, but be lenient).
    let s = sym.strip_prefix("__ZN").or_else(|| sym.strip_prefix("_ZN"))?;
    let b = s.as_bytes();
    let mut i = 0;
    let mut parts: Vec<String> = Vec::new();
    while i < b.len() && b[i] != b'E' {
        let start = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        let len: usize = s[start..i].parse().ok()?;
        let ident = s.get(i..i + len)?;
        i += len;
        parts.push(ident.to_string());
    }
    if i >= b.len() || parts.is_empty() {
        return None;
    }
    // Drop the trailing hash segment (h + 16 hex digits).
    if let Some(last) = parts.last() {
        if last.len() == 17 && last.starts_with('h') && last[1..].bytes().all(|c| c.is_ascii_hexdigit()) {
            parts.pop();
        }
    }
    let parts: Vec<String> = parts.iter().map(|p| unescape(p)).collect();
    Some(parts.join("::"))
}

fn unescape(p: &str) -> String {
    let p = p.strip_prefix("_$").map(|r| format!("${r}")).unwrap_or_else(|| p.to_string());
    let mut out = String::with_capacity(p.len());
    let mut rest = p.as_str();
    while !rest.is_empty() {
        if let Some(r) = rest.strip_prefix("..") {
            out.push_str("::");
            rest = r;
        } else if rest.starts_with('$') {
            let Some(end) = rest[1..].find('$') else {
                out.push_str(rest);
                break;
            };
            let esc = &rest[1..1 + end];
            let rep = match esc {
                "SP" => Some("@".to_string()),
                "BP" => Some("*".into()),
                "RF" => Some("&".into()),
                "LT" => Some("<".into()),
                "GT" => Some(">".into()),
                "LP" => Some("(".into()),
                "RP" => Some(")".into()),
                "C" => Some(",".into()),
                _ => esc
                    .strip_prefix('u')
                    .and_then(|h| u32::from_str_radix(h, 16).ok())
                    .and_then(char::from_u32)
                    .map(|c| c.to_string()),
            };
            match rep {
                Some(r) => {
                    out.push_str(&r);
                    rest = &rest[2 + end..];
                }
                None => {
                    out.push('$');
                    rest = &rest[1..];
                }
            }
        } else {
            let c = rest.chars().next().unwrap();
            out.push(c);
            rest = &rest[c.len_utf8()..];
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# aw2-syms v1 image=AlanWake2.exe base=0x140000000 count=4\n\
        1000\te\tunknown\t-\t\n\
        1020\t26\tremedy\t-\tCSingletonCriticalSection::vfunc_0\n\
        2000\t100\tremedy\tverified\tr::scratchState\n\
        3000\t0\tcrt\t-\tmemcpy\n";

    #[test]
    fn parse_and_lookup() {
        let t = SymTable::parse(SAMPLE).unwrap();
        assert_eq!(t.image, "AlanWake2.exe");
        assert_eq!(t.base, 0x1_4000_0000);
        assert_eq!(t.len(), 4);
        assert_eq!(t.lookup(0x1000).unwrap().to_string(), "sub_140001000 {unknown}");
        assert_eq!(t.lookup(0x1025).unwrap().to_string(), "CSingletonCriticalSection::vfunc_0+0x5 {remedy}");
        assert!(t.lookup(0x1046).is_none(), "past the end of a sized function");
        assert!(t.lookup(0xfff).is_none());
        assert_eq!(t.lookup(0x20ff).unwrap().to_string(), "r::scratchState+0xff {remedy, verified}");
        assert_eq!(t.lookup(0x3400).unwrap().to_string(), "memcpy+0x400 {crt}");
    }

    #[test]
    fn rejects_bad_header() {
        assert!(SymTable::parse("hello\n").is_err());
    }

    fn mangle(parts: &[&str]) -> String {
        let mut s = "_ZN".to_string();
        for p in parts {
            s += &format!("{}{p}", p.len());
        }
        s + "E"
    }

    #[test]
    fn demangles_legacy_rust() {
        assert_eq!(
            demangle("_ZN8win32_rt6unwind18dispatch_exception17h0123456789abcdefE"),
            "win32_rt::unwind::dispatch_exception"
        );
        let m = mangle(&[
            "core",
            "ptr",
            "drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$",
            "h1f0a1b2c3d4e5f60",
        ]);
        assert_eq!(demangle(&m), "core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>");
        assert_eq!(demangle(&mangle(&["foo", "_$LT$impl$GT$", "bar", "h0000000000000000"])), "foo::<impl>::bar");
        assert_eq!(demangle("__psynch_cvwait"), "__psynch_cvwait");
        assert_eq!(demangle("_ZN3foo"), "_ZN3foo");
    }

    #[test]
    fn demangles_v0_rust() {
        assert_eq!(
            demangle("_RNvNtNtCs6aqRWpZFILx_8win32_rt8kernel327threads21WaitForSingleObjectEx"),
            "win32_rt::kernel32::threads::WaitForSingleObjectEx"
        );
        assert_eq!(
            demangle("_RNSNvYNCNvNtNtCs6aqRWpZFILx_8win32_rt8kernel327threads12CreateThread0INtNtNtCsbAqs9W1eE8G_4core3ops8function6FnOnceuE9call_once6vtableBc_"),
            "<win32_rt::kernel32::threads::CreateThread::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}"
        );
        // Inherent impl + generic args + backref.
        assert_eq!(demangle("_RNvMNtCs1_3foo3barINtB2_3BazmE3new"), "<foo::bar::Baz<u32>>::new");
        assert_eq!(demangle("_RCs1_3foo"), "foo");
        assert_eq!(demangle("_Rfoo"), "_Rfoo");
    }
}
