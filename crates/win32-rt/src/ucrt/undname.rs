//! MSVC C++ symbol demangler (`__unDName`, `__unDNameEx`).
//!
//! Decodes MSVC decorated names — both RTTI `TypeDescriptor` names (`.?AV...`,
//! `.?AU...`, `.PEAV...`, primitives) and decorated function/method/data symbols
//! (`?foo@ns@@YAXH@Z`, `??0Cls@@QEAA@XZ`, `?$Vec@H@std@@`, back-references `0..9`).
//! Unparseable or plain C names are copied verbatim, matching Microsoft `undname.cxx`.

use std::ffi::c_void;

const UNDNAME_NO_LEADING_UNDERSCORES: u32 = 0x0001;
const UNDNAME_NO_MS_KEYWORDS: u32 = 0x0002;
const UNDNAME_NO_FUNCTION_RETURNS: u32 = 0x0004;
const UNDNAME_NO_ALLOCATION_LANGUAGE: u32 = 0x0010;
const UNDNAME_NO_ACCESS_SPECIFIERS: u32 = 0x0080;
const UNDNAME_NO_MEMBER_TYPE: u32 = 0x0200;
const UNDNAME_NAME_ONLY: u32 = 0x1000;
const UNDNAME_TYPE_ONLY: u32 = 0x2000;
const UNDNAME_NO_ECSU: u32 = 0x8000;

const MAX_DEPTH: usize = 64;
const MAX_OUT: usize = 65_536;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpecialKind {
    Normal,
    CtorOrDtor,
    Conversion,
}

struct Parser<'a> {
    src: &'a [u8],
    pos: usize,
    flags: u32,
    depth: usize,
    names: Vec<String>,
    types: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(src: &'a [u8], flags: u32) -> Self {
        Self {
            src,
            pos: 0,
            flags,
            depth: 0,
            names: Vec::new(),
            types: Vec::new(),
        }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn peek_at(&self, off: usize) -> Option<u8> {
        self.src.get(self.pos.checked_add(off)?).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    fn eat(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn enter(&mut self) -> Option<()> {
        if self.depth >= MAX_DEPTH {
            return None;
        }
        self.depth += 1;
        Some(())
    }

    fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    fn push_name_ref(&mut self, name: &str) {
        if self.names.len() < 10 {
            self.names.push(name.to_string());
        }
    }

    fn push_type_ref(&mut self, ty: &str) {
        if self.types.len() < 10 {
            self.types.push(ty.to_string());
        }
    }

    fn kw(&self, s: &str) -> String {
        if (self.flags & UNDNAME_NO_MS_KEYWORDS) != 0 {
            String::new()
        } else if (self.flags & UNDNAME_NO_LEADING_UNDERSCORES) != 0 {
            s.trim_start_matches('_').to_string()
        } else {
            s.to_string()
        }
    }

    fn parse_simple_ident(&mut self) -> Option<String> {
        if let Some(b @ b'0'..=b'9') = self.peek() {
            self.pos += 1;
            let idx = (b - b'0') as usize;
            return self.names.get(idx).cloned();
        }
        let start = self.pos;
        while let Some(b) = self.peek() {
            if b == b'@' {
                break;
            }
            self.pos += 1;
        }
        if self.pos == start || !self.eat(b'@') {
            return None;
        }
        let s = std::str::from_utf8(&self.src[start..self.pos - 1]).ok()?.to_string();
        self.push_name_ref(&s);
        Some(s)
    }

    fn parse_signed_number(&mut self) -> Option<i64> {
        let neg = self.eat(b'?');
        let b = self.bump()?;
        let mag = if b.is_ascii_digit() {
            ((b - b'0') as i64) + 1
        } else if b == b'@' {
            0
        } else if (b'A'..=b'P').contains(&b) {
            let mut v: i64 = (b - b'A') as i64;
            while let Some(c) = self.bump() {
                if c == b'@' {
                    break;
                }
                if !(b'A'..=b'P').contains(&c) {
                    return None;
                }
                v = v.checked_shl(4)?.checked_add((c - b'A') as i64)?;
            }
            v
        } else {
            return None;
        };
        Some(if neg { -mag } else { mag })
    }

    fn parse_template_name(&mut self) -> Option<String> {
        self.enter()?;
        // Template names start a fresh name/type back-reference scope in MSVC mangling.
        let saved_names = std::mem::take(&mut self.names);
        let saved_types = std::mem::take(&mut self.types);
        let res = (|| {
            let base = self.parse_simple_ident()?;
            let mut args = Vec::new();
            while let Some(b) = self.peek() {
                if b == b'@' {
                    self.pos += 1;
                    break;
                }
                if b == b'$' {
                    self.pos += 1;
                    match self.bump()? {
                        b'0' => {
                            let n = self.parse_signed_number()?;
                            args.push(n.to_string());
                        }
                        b'1' | b'H' => {
                            let sym = self.parse_symbol_or_raw()?;
                            args.push(format!("&{sym}"));
                        }
                        _ => return None,
                    }
                } else {
                    let arg = self.parse_type(true)?;
                    args.push(arg);
                }
                if args.len() > 64 {
                    return None;
                }
            }
            let joined = args.join(",");
            if joined.ends_with('>') {
                Some(format!("{base}<{joined} >"))
            } else {
                Some(format!("{base}<{joined}>"))
            }
        })();
        self.names = saved_names;
        self.types = saved_types;
        self.leave();
        if let Some(ref s) = res {
            self.push_name_ref(s);
        }
        res
    }

    fn parse_special_name(&mut self) -> Option<(String, SpecialKind)> {
        // Called after leading `?`, when next char is also `?`.
        self.eat(b'?');
        let c = self.bump()?;
        let (s, kind) = match c {
            b'0' => ("{ctor}", SpecialKind::CtorOrDtor),
            b'1' => ("{dtor}", SpecialKind::CtorOrDtor),
            b'2' => ("operator new", SpecialKind::Normal),
            b'3' => ("operator delete", SpecialKind::Normal),
            b'4' => ("operator=", SpecialKind::Normal),
            b'5' => ("operator>>", SpecialKind::Normal),
            b'6' => ("operator<<", SpecialKind::Normal),
            b'7' => ("operator!", SpecialKind::Normal),
            b'8' => ("operator==", SpecialKind::Normal),
            b'9' => ("operator!=", SpecialKind::Normal),
            b'A' => ("operator[]", SpecialKind::Normal),
            b'B' => ("operator {conv}", SpecialKind::Conversion),
            b'C' => ("operator->", SpecialKind::Normal),
            b'D' => ("operator*", SpecialKind::Normal),
            b'E' => ("operator++", SpecialKind::Normal),
            b'F' => ("operator--", SpecialKind::Normal),
            b'G' => ("operator-", SpecialKind::Normal),
            b'H' => ("operator+", SpecialKind::Normal),
            b'I' => ("operator&", SpecialKind::Normal),
            b'J' => ("operator->*", SpecialKind::Normal),
            b'K' => ("operator/", SpecialKind::Normal),
            b'L' => ("operator%", SpecialKind::Normal),
            b'M' => ("operator<", SpecialKind::Normal),
            b'N' => ("operator<=", SpecialKind::Normal),
            b'O' => ("operator>", SpecialKind::Normal),
            b'P' => ("operator>=", SpecialKind::Normal),
            b'Q' => ("operator,", SpecialKind::Normal),
            b'R' => ("operator()", SpecialKind::Normal),
            b'S' => ("operator~", SpecialKind::Normal),
            b'T' => ("operator^", SpecialKind::Normal),
            b'U' => ("operator|", SpecialKind::Normal),
            b'V' => ("operator&&", SpecialKind::Normal),
            b'W' => ("operator||", SpecialKind::Normal),
            b'X' => ("operator*=", SpecialKind::Normal),
            b'Y' => ("operator+=", SpecialKind::Normal),
            b'Z' => ("operator-=", SpecialKind::Normal),
            b'_' => {
                let c2 = self.bump()?;
                match c2 {
                    b'0' => ("operator/=", SpecialKind::Normal),
                    b'1' => ("operator%=", SpecialKind::Normal),
                    b'2' => ("operator>>=", SpecialKind::Normal),
                    b'3' => ("operator<<=", SpecialKind::Normal),
                    b'4' => ("operator&=", SpecialKind::Normal),
                    b'5' => ("operator|=", SpecialKind::Normal),
                    b'6' => ("operator^=", SpecialKind::Normal),
                    b'7' => ("`vftable'", SpecialKind::Normal),
                    b'8' => ("`vbtable'", SpecialKind::Normal),
                    b'9' => ("`vcall'", SpecialKind::Normal),
                    b'A' => ("`typeof'", SpecialKind::Normal),
                    b'B' => ("`local static guard'", SpecialKind::Normal),
                    b'E' => ("`vector deleting destructor'", SpecialKind::Normal),
                    b'F' => ("`default constructor closure'", SpecialKind::Normal),
                    b'G' => ("`scalar deleting destructor'", SpecialKind::Normal),
                    b'H' => ("`vector constructor iterator'", SpecialKind::Normal),
                    b'I' => ("`vector destructor iterator'", SpecialKind::Normal),
                    b'U' => ("operator new[]", SpecialKind::Normal),
                    b'V' => ("operator delete[]", SpecialKind::Normal),
                    b'R' => {
                        let r = self.bump()?;
                        match r {
                            b'0' => {
                                let ty = self.parse_type(false)?;
                                return Some((
                                    format!("{ty} `RTTI Type Descriptor'"),
                                    SpecialKind::Normal,
                                ));
                            }
                            b'1' => ("`RTTI Base Class Descriptor'", SpecialKind::Normal),
                            b'2' => ("`RTTI Base Class Array'", SpecialKind::Normal),
                            b'3' => ("`RTTI Class Hierarchy Descriptor'", SpecialKind::Normal),
                            b'4' => ("`RTTI Complete Object Locator'", SpecialKind::Normal),
                            _ => return None,
                        }
                    }
                    _ => return None,
                }
            }
            _ => return None,
        };
        Some((s.to_string(), kind))
    }

    fn parse_unqualified_name(&mut self) -> Option<(String, SpecialKind)> {
        if self.peek() == Some(b'?') && self.peek_at(1) == Some(b'$') {
            self.pos += 2;
            return Some((self.parse_template_name()?, SpecialKind::Normal));
        }
        if self.peek() == Some(b'?') {
            return self.parse_special_name();
        }
        Some((self.parse_simple_ident()?, SpecialKind::Normal))
    }

    fn parse_scope_chain(&mut self) -> Option<Vec<String>> {
        let mut scopes = Vec::new();
        while let Some(b) = self.peek() {
            if b == b'@' {
                self.pos += 1;
                return Some(scopes);
            }
            if b == b'?' && self.peek_at(1) == Some(b'$') {
                self.pos += 2;
                scopes.push(self.parse_template_name()?);
            } else if b == b'?' && self.peek_at(1) == Some(b'A') {
                // Anonymous namespace: `?A0x...@`
                let start = self.pos;
                while let Some(c) = self.bump() {
                    if c == b'@' {
                        break;
                    }
                }
                if self.pos == start {
                    return None;
                }
                scopes.push("`anonymous namespace'".to_string());
            } else if b == b'?' {
                // Local function scope or numbered scope (`?1?func@...`)
                self.pos += 1;
                let _ = self.parse_signed_number()?;
            } else {
                scopes.push(self.parse_simple_ident()?);
            }
            if scopes.len() > 64 {
                return None;
            }
        }
        None
    }

    fn parse_qualified_name(&mut self) -> Option<(String, SpecialKind)> {
        let (unqual, kind) = self.parse_unqualified_name()?;
        let mut scopes = self.parse_scope_chain()?;
        scopes.reverse();
        let leaf = if kind == SpecialKind::CtorOrDtor {
            let class_name = scopes.last().cloned().unwrap_or_default();
            let base = class_name.split('<').next().unwrap_or(&class_name);
            if unqual == "{ctor}" {
                base.to_string()
            } else {
                format!("~{base}")
            }
        } else {
            unqual
        };
        if scopes.is_empty() {
            Some((leaf, kind))
        } else {
            scopes.push(leaf);
            Some((scopes.join("::"), kind))
        }
    }

    fn parse_cv(&mut self) -> Option<&'static str> {
        match self.bump()? {
            b'A' => Some(""),
            b'B' => Some("const"),
            b'C' => Some("volatile"),
            b'D' => Some("const volatile"),
            _ => None,
        }
    }

    fn parse_calling_conv(&mut self) -> Option<&'static str> {
        match self.bump()? {
            b'A' | b'B' => Some("__cdecl"),
            b'C' | b'D' => Some("__pascal"),
            b'E' | b'F' => Some("__thiscall"),
            b'G' | b'H' => Some("__stdcall"),
            b'I' | b'J' => Some("__fastcall"),
            b'Q' | b'R' => Some("__vectorcall"),
            _ => None,
        }
    }

    fn parse_type(&mut self, record_backref: bool) -> Option<String> {
        self.enter()?;
        let res = self.parse_type_inner(record_backref);
        self.leave();
        res
    }

    fn parse_type_inner(&mut self, record_backref: bool) -> Option<String> {
        if let Some(b @ b'0'..=b'9') = self.peek() {
            self.pos += 1;
            let idx = (b - b'0') as usize;
            return self.types.get(idx).cloned();
        }
        let start_pos = self.pos;
        let b = self.bump()?;
        let ty = match b {
            b'C' => "signed char".to_string(),
            b'D' => "char".to_string(),
            b'E' => "unsigned char".to_string(),
            b'F' => "short".to_string(),
            b'G' => "unsigned short".to_string(),
            b'H' => "int".to_string(),
            b'I' => "unsigned int".to_string(),
            b'J' => "long".to_string(),
            b'K' => "unsigned long".to_string(),
            b'M' => "float".to_string(),
            b'N' => "double".to_string(),
            b'O' => "long double".to_string(),
            b'X' => "void".to_string(),
            b'Z' => "...".to_string(),
            b'_' => match self.bump()? {
                b'D' => "__int8".to_string(),
                b'E' => "unsigned __int8".to_string(),
                b'F' => "__int16".to_string(),
                b'G' => "unsigned __int16".to_string(),
                b'H' => "__int32".to_string(),
                b'I' => "unsigned __int32".to_string(),
                b'J' => "__int64".to_string(),
                b'K' => "unsigned __int64".to_string(),
                b'N' => "bool".to_string(),
                b'S' => "char16_t".to_string(),
                b'U' => "char32_t".to_string(),
                b'W' => "wchar_t".to_string(),
                _ => return None,
            },
            b'V' | b'U' | b'T' | b'W' => {
                let tag = match b {
                    b'V' => "class ",
                    b'U' => "struct ",
                    b'T' => "union ",
                    b'W' => {
                        let _underlying = self.bump()?;
                        "enum "
                    }
                    _ => unreachable!(),
                };
                let (qname, _) = self.parse_qualified_name()?;
                if (self.flags & UNDNAME_NO_ECSU) != 0 {
                    qname
                } else {
                    format!("{tag}{qname}")
                }
            }
            b'P' | b'Q' | b'R' | b'S' | b'A' | b'B' => {
                let is_ref = matches!(b, b'A' | b'B');
                let ptr_cv = match b {
                    b'Q' => " const",
                    b'R' => " volatile",
                    b'S' => " const volatile",
                    b'B' => " volatile",
                    _ => "",
                };
                let mut ptr64 = false;
                if self.peek() == Some(b'E') {
                    self.pos += 1;
                    ptr64 = true;
                }
                // Function pointer: `P6<cc><ret><args>@Z`
                if !is_ref && self.peek() == Some(b'6') {
                    self.pos += 1;
                    let cc = self.parse_calling_conv()?;
                    let ret = self.parse_type(false)?;
                    let params = self.parse_param_list()?;
                    let cc_str = if (self.flags & UNDNAME_NO_ALLOCATION_LANGUAGE) != 0 {
                        String::new()
                    } else {
                        let k = self.kw(cc);
                        if k.is_empty() { String::new() } else { format!("{k} ") }
                    };
                    format!("{ret} ({cc_str}*)({params})")
                } else {
                    let pointee_cv = self.parse_cv()?;
                    let target = self.parse_type(false)?;
                    let mut s = if pointee_cv.is_empty() {
                        target
                    } else {
                        format!("{target} {pointee_cv}")
                    };
                    s.push_str(if is_ref { " &" } else { " *" });
                    if ptr64 {
                        let k = self.kw("__ptr64");
                        if !k.is_empty() {
                            s.push(' ');
                            s.push_str(&k);
                        }
                    }
                    s.push_str(ptr_cv);
                    s
                }
            }
            b'?' => {
                // Qualified primitive or template parameter (`?A...`, `?B...`, `$$...`)
                let cv = self.parse_cv()?;
                let inner = self.parse_type(false)?;
                if cv.is_empty() { inner } else { format!("{inner} {cv}") }
            }
            b'$' => {
                if self.eat(b'$') {
                    match self.bump()? {
                        b'Q' => {
                            // Rvalue reference `&&`
                            let _ = self.eat(b'E');
                            let cv = self.parse_cv()?;
                            let inner = self.parse_type(false)?;
                            if cv.is_empty() {
                                format!("{inner} &&")
                            } else {
                                format!("{inner} {cv} &&")
                            }
                        }
                        b'T' => "std::nullptr_t".to_string(),
                        b'V' => String::new(),
                        _ => return None,
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        };
        // MSVC records every non-empty parameter type whose mangled encoding is > 1 byte
        // (including 2-byte extended primitives such as `_J`, `_N`, `_W` and `$$T`).
        if record_backref && !ty.is_empty() && (self.pos - start_pos) > 1 {
            self.push_type_ref(&ty);
        }
        Some(ty)
    }

    fn parse_param_list(&mut self) -> Option<String> {
        if self.peek() == Some(b'X') {
            self.pos += 1;
            let _ = self.eat(b'Z');
            return Some("void".to_string());
        }
        let mut params = Vec::new();
        while let Some(b) = self.peek() {
            if b == b'@' {
                self.pos += 1;
                let _ = self.eat(b'Z');
                break;
            }
            if b == b'Z' {
                self.pos += 1;
                params.push("...".to_string());
                let _ = self.eat(b'Z');
                break;
            }
            params.push(self.parse_type(true)?);
            if params.len() > 64 {
                return None;
            }
        }
        if params.is_empty() {
            Some("void".to_string())
        } else {
            Some(params.join(","))
        }
    }

    fn parse_symbol_or_raw(&mut self) -> Option<String> {
        if !self.eat(b'?') {
            return None;
        }
        let (mut qname, kind) = self.parse_qualified_name()?;
        if (self.flags & UNDNAME_NAME_ONLY) != 0 && kind != SpecialKind::Conversion {
            return Some(qname);
        }
        let Some(sc) = self.bump() else {
            return if kind == SpecialKind::Conversion { None } else { Some(qname) };
        };
        // Data members / globals (`0`..`3`) and special tables (`6`, `7`)
        match sc {
            b'0' | b'1' | b'2' | b'3' => {
                let access = match sc {
                    b'0' => "private: static ",
                    b'1' => "protected: static ",
                    b'2' => "public: static ",
                    _ => "",
                };
                let ty = self.parse_type(false)?;
                // Skip optional x64/MS extended storage qualifiers (`E` __ptr64, `F` __unaligned, `I` __restrict)
                // before the trailing storage CV qualifier (`A`/`B`/`C`/`D`).
                while matches!(self.peek(), Some(b'E' | b'F' | b'I')) {
                    self.pos += 1;
                }
                let cv = if self.peek().is_some() {
                    self.parse_cv()?
                } else {
                    ""
                };
                let mut out = String::new();
                if (self.flags & UNDNAME_NO_ACCESS_SPECIFIERS) == 0 {
                    out.push_str(access);
                }
                out.push_str(&ty);
                if !cv.is_empty() {
                    out.push(' ');
                    out.push_str(cv);
                }
                out.push(' ');
                out.push_str(&qname);
                return Some(out);
            }
            b'6' | b'7' => {
                let cv = self.parse_cv().unwrap_or("");
                if cv.is_empty() {
                    return Some(qname);
                }
                return Some(format!("{cv} {qname}"));
            }
            _ => {}
        }

        // Functions and methods.
        let (access, is_static, is_virtual, is_member) = match sc {
            b'A' | b'B' => ("private: ", false, false, true),
            b'C' | b'D' => ("private: ", true, false, true),
            b'E' | b'F' => ("private: ", false, true, true),
            b'I' | b'J' => ("protected: ", false, false, true),
            b'K' | b'L' => ("protected: ", true, false, true),
            b'M' | b'N' => ("protected: ", false, true, true),
            b'Q' | b'R' => ("public: ", false, false, true),
            b'S' | b'T' => ("public: ", true, false, true),
            b'U' | b'V' => ("public: ", false, true, true),
            b'Y' | b'Z' => ("", false, false, false),
            _ => return Some(qname),
        };

        let mut this_cv = "";
        if is_member && !is_static {
            let _ = self.eat(b'E'); // __ptr64 `this`
            this_cv = self.parse_cv()?;
        }
        let cc = self.parse_calling_conv()?;
        let ret = if kind == SpecialKind::CtorOrDtor {
            let _ = self.eat(b'@');
            None
        } else {
            Some(self.parse_type(false)?)
        };
        if kind == SpecialKind::Conversion {
            let target = ret.as_deref()?;
            qname = qname.replace("{conv}", target);
            if (self.flags & UNDNAME_NAME_ONLY) != 0 {
                return Some(qname);
            }
        }
        let params = self.parse_param_list()?;

        let mut out = String::new();
        if (self.flags & UNDNAME_NO_ACCESS_SPECIFIERS) == 0 && !access.is_empty() {
            out.push_str(access);
        }
        if (self.flags & UNDNAME_NO_MEMBER_TYPE) == 0 {
            if is_static {
                out.push_str("static ");
            }
            if is_virtual {
                out.push_str("virtual ");
            }
        }
        if (self.flags & UNDNAME_NO_FUNCTION_RETURNS) == 0 {
            if let Some(r) = ret {
                out.push_str(&r);
                out.push(' ');
            }
        }
        if (self.flags & UNDNAME_NO_ALLOCATION_LANGUAGE) == 0 {
            let k = self.kw(cc);
            if !k.is_empty() {
                out.push_str(&k);
                out.push(' ');
            }
        }
        out.push_str(&qname);
        out.push('(');
        out.push_str(&params);
        out.push(')');
        if !this_cv.is_empty() {
            out.push(' ');
            out.push_str(this_cv);
        }
        Some(out)
    }
}

fn parse_type_only(bytes: &[u8], flags: u32) -> Option<String> {
    let mut p = Parser::new(bytes, flags);
    let ty = if bytes.starts_with(b"?A")
        || bytes.starts_with(b"?B")
        || bytes.starts_with(b"?C")
        || bytes.starts_with(b"?D")
    {
        p.pos = 1;
        let cv = p.parse_cv()?;
        let inner = p.parse_type(false)?;
        if cv.is_empty() { inner } else { format!("{inner} {cv}") }
    } else {
        p.parse_type(false)?
    };
    if p.pos == p.src.len() {
        Some(ty)
    } else {
        None
    }
}

/// Demangle an MSVC C++ decorated name or RTTI `TypeDescriptor` name.
/// Returns the original input unchanged if it is not a recognized MSVC decorated name.
pub(crate) fn demangle_msvc(input: &str, flags: u32) -> String {
    if input.is_empty() {
        return String::new();
    }
    let bytes = input.as_bytes();
    let parsed = if let Some(rest) = bytes.strip_prefix(b".") {
        // RTTI TypeDescriptor names start with `.` (e.g. `.?AVFoo@ns@@`, `.PEAX`, `.H`).
        parse_type_only(rest, flags)
    } else if (flags & UNDNAME_TYPE_ONLY) != 0 {
        // Type-only mode accepts bare MSVC type encodings (`H`, `PEAX`, `VFoo@@`, `?AVFoo@@`)
        // without requiring a leading `?` or `.`.
        parse_type_only(bytes, flags).or_else(|| {
            if bytes.starts_with(b"?") {
                let mut p = Parser::new(bytes, flags);
                p.parse_symbol_or_raw()
            } else {
                None
            }
        })
    } else if bytes.starts_with(b"?") {
        let mut p = Parser::new(bytes, flags);
        p.parse_symbol_or_raw()
    } else {
        None
    };

    let mut out = parsed.unwrap_or_else(|| input.to_string());
    if out.len() > MAX_OUT {
        out.truncate(MAX_OUT);
    }
    out
}

unsafe fn undname_impl(
    output: *mut u8,
    name: *const u8,
    max_len: i32,
    p_alloc: usize,
    flags: u32,
) -> *mut u8 {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    let raw = crate::strings::from_ansi(name);
    let demangled = demangle_msvc(&raw, flags);
    let bytes = demangled.as_bytes();
    if output.is_null() {
        if p_alloc == 0 {
            return std::ptr::null_mut();
        }
        let need = bytes.len() + 1;
        let dst = crate::call_guest8(p_alloc, need as u64, 0, 0, 0, 0, 0, 0, 0) as *mut u8;
        if dst.is_null() {
            return std::ptr::null_mut();
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        *dst.add(bytes.len()) = 0;
        return dst;
    }
    if max_len <= 0 {
        return std::ptr::null_mut();
    }
    let cap = max_len as usize;
    let copy_len = bytes.len().min(cap - 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, copy_len);
    *output.add(copy_len) = 0;
    output
}

pub unsafe extern "win64" fn __unDName(
    output: *mut u8,
    name: *const u8,
    max_len: i32,
    p_alloc: usize,
    _p_free: usize,
    flags: u16,
) -> *mut u8 {
    undname_impl(output, name, max_len, p_alloc, flags as u32)
}

pub unsafe extern "win64" fn __unDNameEx(
    output: *mut u8,
    name: *const u8,
    max_len: i32,
    p_alloc: usize,
    _p_free: usize,
    _get_param: *const c_void,
    flags: u32,
) -> *mut u8 {
    undname_impl(output, name, max_len, p_alloc, flags)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn demangles_rtti_functions_templates_and_flags() {
        // RTTI TypeDescriptors & literal Windows flag values (UNDNAME_NO_ECSU = 0x8000, UNDNAME_32_BIT_DECODE = 0x0800)
        assert_eq!(demangle_msvc(".?AVsystem_error@std@@", 0), "class std::system_error");
        assert_eq!(
            demangle_msvc(".?AVsystem_error@std@@", 0x8000),
            "std::system_error"
        );
        assert_eq!(
            demangle_msvc(".?AVsystem_error@std@@", 0x0800),
            "class std::system_error"
        );
        assert_eq!(demangle_msvc(".?AU_Mbstatet@@", 0), "struct _Mbstatet");
        assert_eq!(demangle_msvc(".?AW4MyEnum@ns@@", 0), "enum ns::MyEnum");
        assert_eq!(demangle_msvc(".PEAVFoo@@", UNDNAME_NO_MS_KEYWORDS), "class Foo *");
        assert_eq!(demangle_msvc(".H", 0), "int");

        // UNDNAME_TYPE_ONLY (0x2000) decodes bare type encodings without requiring leading '?' or '.'.
        assert_eq!(demangle_msvc("H", 0x2000), "int");
        assert_eq!(demangle_msvc("PEAX", 0x2000), "void * __ptr64");
        assert_eq!(demangle_msvc("PEAX", 0x2000 | UNDNAME_NO_MS_KEYWORDS), "void *");
        assert_eq!(demangle_msvc("VFoo@ns@@", 0x2000), "class ns::Foo");
        assert_eq!(demangle_msvc("?AVFoo@ns@@", 0x2000), "class ns::Foo");
        assert_eq!(demangle_msvc("Hello", 0x2000), "Hello");
        assert_eq!(
            demangle_msvc(
                ".?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@",
                0
            ),
            "class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> >"
        );

        // Decorated functions, methods, ctors, dtors
        assert_eq!(
            demangle_msvc("?foo@ns@@YAXH@Z", 0),
            "void __cdecl ns::foo(int)"
        );
        assert_eq!(
            demangle_msvc("?foo@ns@@YAXH@Z", UNDNAME_NAME_ONLY),
            "ns::foo"
        );
        assert_eq!(
            demangle_msvc("??0Widget@ui@@QEAA@XZ", 0),
            "public: __cdecl ui::Widget::Widget(void)"
        );
        assert_eq!(
            demangle_msvc("??1Widget@ui@@UEAA@XZ", 0),
            "public: virtual __cdecl ui::Widget::~Widget(void)"
        );

        // Decorated data symbols preserve trailing const/volatile storage qualifiers.
        assert_eq!(demangle_msvc("?k@A@@2HB", 0), "public: static int const A::k");
        assert_eq!(demangle_msvc("?v@A@@2HC", 0), "public: static int volatile A::v");
        assert_eq!(
            demangle_msvc("?cv@A@@2HD", 0),
            "public: static int const volatile A::cv"
        );
        assert_eq!(demangle_msvc("?g@@3HA", 0), "int g");
        assert_eq!(
            demangle_msvc("?p@A@@2PEBHEB", UNDNAME_NO_MS_KEYWORDS),
            "public: static int const * const A::p"
        );

        // Multi-byte extended primitives (`_J`, `_N`, `$$T`) participate in parameter type back-references.
        assert_eq!(
            demangle_msvc("?mix@@YAX_JPEAH0@Z", 0),
            "void __cdecl mix(__int64,int * __ptr64,__int64)"
        );
        assert_eq!(
            demangle_msvc("?f@@YAX_NPEAH0$$T2@Z", UNDNAME_NO_MS_KEYWORDS),
            "void f(bool,int *,bool,std::nullptr_t,std::nullptr_t)"
        );

        // Variadic function-pointer parameters consume the nested throw-spec terminator cleanly.
        assert_eq!(
            demangle_msvc("?pf@@YAXP6AHHZZ@Z", 0),
            "void __cdecl pf(int (__cdecl *)(int,...))"
        );
        assert_eq!(
            demangle_msvc("?pf2@@YAXP6AHZZ@Z", 0),
            "void __cdecl pf2(int (__cdecl *)(...))"
        );

        // Conversion operators (`??B`) splice the target type into the operator name.
        assert_eq!(
            demangle_msvc("??BA@@QEBAHXZ", 0),
            "public: int __cdecl A::operator int(void) const"
        );
        assert_eq!(
            demangle_msvc("??BA@@QEBAHXZ", UNDNAME_NO_FUNCTION_RETURNS),
            "public: __cdecl A::operator int(void) const"
        );
        assert_eq!(
            demangle_msvc("??BA@@QEBAHXZ", UNDNAME_NAME_ONLY),
            "A::operator int"
        );

        // Non-mangled C names pass through unchanged; deeply nested garbage never panics.
        assert_eq!(demangle_msvc("plain_c_symbol", 0), "plain_c_symbol");
        let deep = format!("?{}@A@@YAXH@Z", "P".repeat(200));
        let _ = demangle_msvc(&deep, 0);
    }

    #[test]
    fn undname_abi_buffer_truncation_and_guest_allocator() {
        static ALLOC_BYTES: AtomicUsize = AtomicUsize::new(0);
        unsafe extern "win64" fn test_alloc(n: usize) -> *mut c_void {
            ALLOC_BYTES.store(n, Ordering::SeqCst);
            crate::ucrt::heap::malloc(n)
        }

        unsafe {
            assert!(crate::lookup("api-ms-win-crt-private-l1-1-0.dll", "__unDName").is_some());
            assert!(crate::lookup("ucrtbase.dll", "__unDNameEx").is_some());

            // Caller-provided buffer + truncation.
            let mut buf = [0xAAu8; 32];
            let p = __unDName(
                buf.as_mut_ptr(),
                b".?AVFoo@Bar@@\0".as_ptr(),
                buf.len() as i32,
                0,
                0,
                0,
            );
            assert_eq!(p, buf.as_mut_ptr());
            assert_eq!(crate::strings::from_ansi(p), "class Bar::Foo");

            let mut tiny = [0xAAu8; 6];
            let p_tiny = __unDName(
                tiny.as_mut_ptr(),
                b".?AVFoo@Bar@@\0".as_ptr(),
                tiny.len() as i32,
                0,
                0,
                0,
            );
            assert_eq!(p_tiny, tiny.as_mut_ptr());
            assert_eq!(&tiny, b"class\0");

            // NULL output allocates via caller-supplied pAlloc callback.
            ALLOC_BYTES.store(0, Ordering::SeqCst);
            let p_alloc = __unDName(
                std::ptr::null_mut(),
                b"?foo@ns@@YAXH@Z\0".as_ptr(),
                0,
                test_alloc as *const () as usize,
                0,
                UNDNAME_NAME_ONLY as u16,
            );
            assert!(!p_alloc.is_null());
            assert_eq!(ALLOC_BYTES.load(Ordering::SeqCst), "ns::foo".len() + 1);
            assert_eq!(crate::strings::from_ansi(p_alloc), "ns::foo");
            crate::ucrt::heap::free(p_alloc.cast());

            // NULL name, NULL output without pAlloc, or non-positive max_len returns NULL.
            assert!(__unDName(buf.as_mut_ptr(), std::ptr::null(), 32, 0, 0, 0).is_null());
            assert!(__unDName(std::ptr::null_mut(), b".H\0".as_ptr(), 0, 0, 0, 0).is_null());
            assert!(__unDName(buf.as_mut_ptr(), b".H\0".as_ptr(), 0, 0, 0, 0).is_null());
        }
    }
}
