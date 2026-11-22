//! wldap32.dll: pure-Rust RFC 4511 BER/ASN.1 LDAPv3 client over `TcpStream`.
//! Imported by ordinal (preserving the Windows `wldap32.dll` ordinal table)
//! and honoring `PORT_OFFLINE=1`.

use crate::types::*;
use std::ffi::c_void;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (22, "ldap_err2stringA"),
    (26, "ldap_first_entry"),
    (27, "ldap_next_entry"),
    (30, "ldap_get_dnA"),
    (32, "ldap_first_attributeA"),
    (33, "ldap_next_attributeA"),
    (35, "ldap_get_values_lenA"),
    (41, "ldap_msgfree"),
    (46, "ldap_unbind_s"),
    (50, "ldap_search_sA"),
    (60, "ldap_simple_bind_sA"),
    (79, "ldap_value_free_len"),
    (143, "ldap_initA"),
    (200, "ldap_memfreeA"),
    (211, "ldap_set_optionA"),
    (301, "ber_free"),
];

// Standard LDAP result / client error codes (winldap.h / RFC 4511)
const LDAP_SUCCESS: u32 = 0x00;
const LDAP_OPERATIONS_ERROR: u32 = 0x01;
const LDAP_PROTOCOL_ERROR: u32 = 0x02;
const LDAP_TIMELIMIT_EXCEEDED: u32 = 0x03;
const LDAP_SIZELIMIT_EXCEEDED: u32 = 0x04;
const LDAP_AUTH_METHOD_NOT_SUPPORTED: u32 = 0x07;
const LDAP_NO_SUCH_OBJECT: u32 = 0x20;
const LDAP_INVALID_DN_SYNTAX: u32 = 0x22;
const LDAP_INVALID_CREDENTIALS: u32 = 0x31;
const LDAP_INSUFFICIENT_RIGHTS: u32 = 0x32;
const LDAP_UNWILLING_TO_PERFORM: u32 = 0x35;
const LDAP_SERVER_DOWN: u32 = 0x51;
const LDAP_LOCAL_ERROR: u32 = 0x52;
const LDAP_ENCODING_ERROR: u32 = 0x53;
const LDAP_DECODING_ERROR: u32 = 0x54;
const LDAP_TIMEOUT: u32 = 0x55;
const LDAP_FILTER_ERROR: u32 = 0x57;
const LDAP_PARAM_ERROR: u32 = 0x59;
const LDAP_NO_MEMORY: u32 = 0x5A;
const LDAP_CONNECT_ERROR: u32 = 0x5B;
const LDAP_NOT_SUPPORTED: u32 = 0x5C;

// Option identifiers (winldap.h)
const LDAP_OPT_DEREF: i32 = 0x02;
const LDAP_OPT_SIZELIMIT: i32 = 0x03;
const LDAP_OPT_TIMELIMIT: i32 = 0x04;
const LDAP_OPT_REFERRALS: i32 = 0x08;
const LDAP_OPT_SSL: i32 = 0x0A;
const LDAP_OPT_PROTOCOL_VERSION: i32 = 0x11;
const LDAP_OPT_SEND_TIMEOUT: i32 = 0x42;
const LDAP_OPT_TCP_KEEPALIVE: i32 = 0x40;
const LDAP_OPT_NETWORK_TIMEOUT: i32 = 0x5005;

// RFC 4511 protocolOp tags
const TAG_BOOLEAN: u8 = 0x01;
const TAG_INTEGER: u8 = 0x02;
const TAG_OCTET_STRING: u8 = 0x04;
const TAG_ENUMERATED: u8 = 0x0A;
const TAG_SEQUENCE: u8 = 0x30;
const TAG_SET: u8 = 0x31;
const TAG_BIND_REQUEST: u8 = 0x60;
const TAG_BIND_RESPONSE: u8 = 0x61;
const TAG_UNBIND_REQUEST: u8 = 0x42;
const TAG_SEARCH_REQUEST: u8 = 0x63;
const TAG_SEARCH_RESULT_ENTRY: u8 = 0x64;
const TAG_SEARCH_RESULT_DONE: u8 = 0x65;
const TAG_SEARCH_RESULT_REF: u8 = 0x73;

const MAX_BER_LEN: usize = 16 * 1024 * 1024;

fn is_offline() -> bool {
    std::env::var("PORT_OFFLINE")
        .is_ok_and(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

// ---------------------------------------------------------------------------
// Minimal RFC 4511 BER encoder / decoder
// ---------------------------------------------------------------------------

fn ber_push_len(out: &mut Vec<u8>, len: usize) {
    if len < 0x80 {
        out.push(len as u8);
    } else {
        let bytes = (len as u64).to_be_bytes();
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(7);
        let n = 8 - first;
        out.push(0x80 | (n as u8));
        out.extend_from_slice(&bytes[first..]);
    }
}

fn ber_tlv(tag: u8, value: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + value.len());
    out.push(tag);
    ber_push_len(&mut out, value.len());
    out.extend_from_slice(value);
    out
}

fn ber_int_bytes(v: i64) -> Vec<u8> {
    let raw = v.to_be_bytes();
    let mut start = 0usize;
    while start < 7 {
        let cur = raw[start];
        let next = raw[start + 1];
        if (cur == 0x00 && (next & 0x80) == 0) || (cur == 0xFF && (next & 0x80) != 0) {
            start += 1;
        } else {
            break;
        }
    }
    raw[start..].to_vec()
}

fn ber_integer(v: i64) -> Vec<u8> {
    ber_tlv(TAG_INTEGER, &ber_int_bytes(v))
}

fn ber_enumerated(v: i64) -> Vec<u8> {
    ber_tlv(TAG_ENUMERATED, &ber_int_bytes(v))
}

fn ber_boolean(v: bool) -> Vec<u8> {
    ber_tlv(TAG_BOOLEAN, &[if v { 0xFF } else { 0x00 }])
}

fn ber_octet_string(s: &[u8]) -> Vec<u8> {
    ber_tlv(TAG_OCTET_STRING, s)
}

#[derive(Debug)]
struct BerElem<'a> {
    tag: u8,
    val: &'a [u8],
}

fn ber_decode_one(buf: &[u8]) -> Result<(BerElem<'_>, &[u8]), u32> {
    if buf.len() < 2 {
        return Err(LDAP_DECODING_ERROR);
    }
    let tag = buf[0];
    let mut pos = 1usize;
    // Skip extended tag octets if present
    if (tag & 0x1F) == 0x1F {
        while pos < buf.len() && (buf[pos] & 0x80) != 0 {
            pos += 1;
        }
        pos += 1;
        if pos >= buf.len() {
            return Err(LDAP_DECODING_ERROR);
        }
    }
    let first_len = buf[pos];
    pos += 1;
    let len = if (first_len & 0x80) == 0 {
        first_len as usize
    } else {
        let n = (first_len & 0x7F) as usize;
        if n == 0 || n > 4 || pos + n > buf.len() {
            return Err(LDAP_DECODING_ERROR);
        }
        let mut l = 0usize;
        for &b in &buf[pos..pos + n] {
            l = (l << 8) | (b as usize);
        }
        pos += n;
        l
    };
    if len > MAX_BER_LEN || pos.checked_add(len).is_none_or(|e| e > buf.len()) {
        return Err(LDAP_DECODING_ERROR);
    }
    Ok((BerElem { tag, val: &buf[pos..pos + len] }, &buf[pos + len..]))
}

fn ber_decode_int(val: &[u8]) -> Result<i64, u32> {
    if val.is_empty() || val.len() > 8 {
        return Err(LDAP_DECODING_ERROR);
    }
    let mut acc: i64 = if (val[0] & 0x80) != 0 { -1 } else { 0 };
    for &b in val {
        acc = (acc << 8) | (b as i64);
    }
    Ok(acc)
}

fn read_ber_message_from_stream(stream: &mut TcpStream) -> Result<Vec<u8>, u32> {
    let mut header = Vec::with_capacity(8);
    let mut b = [0u8; 1];
    stream.read_exact(&mut b).map_err(|e| map_io_err(&e))?;
    header.push(b[0]);
    if (b[0] & 0x1F) == 0x1F {
        loop {
            stream.read_exact(&mut b).map_err(|e| map_io_err(&e))?;
            header.push(b[0]);
            if (b[0] & 0x80) == 0 {
                break;
            }
            if header.len() > 8 {
                return Err(LDAP_DECODING_ERROR);
            }
        }
    }
    stream.read_exact(&mut b).map_err(|e| map_io_err(&e))?;
    header.push(b[0]);
    let len = if (b[0] & 0x80) == 0 {
        b[0] as usize
    } else {
        let n = (b[0] & 0x7F) as usize;
        if n == 0 || n > 4 {
            return Err(LDAP_DECODING_ERROR);
        }
        let mut lbuf = [0u8; 4];
        stream.read_exact(&mut lbuf[..n]).map_err(|e| map_io_err(&e))?;
        header.extend_from_slice(&lbuf[..n]);
        let mut l = 0usize;
        for &lb in &lbuf[..n] {
            l = (l << 8) | (lb as usize);
        }
        l
    };
    if len > MAX_BER_LEN {
        return Err(LDAP_DECODING_ERROR);
    }
    let mut body = vec![0u8; len];
    if len > 0 {
        stream.read_exact(&mut body).map_err(|e| map_io_err(&e))?;
    }
    header.extend_from_slice(&body);
    Ok(header)
}

fn map_io_err(e: &io::Error) -> u32 {
    match e.kind() {
        io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => LDAP_TIMEOUT,
        _ => LDAP_SERVER_DOWN,
    }
}

// ---------------------------------------------------------------------------
// RFC 4515 LDAP Search Filter parser -> RFC 4511 BER Filter
// ---------------------------------------------------------------------------

fn unescape_ldap_value(s: &str) -> Result<Vec<u8>, u32> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            if i + 2 >= bytes.len() {
                return Err(LDAP_FILTER_ERROR);
            }
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).map_err(|_| LDAP_FILTER_ERROR)?;
            let v = u8::from_str_radix(hex, 16).map_err(|_| LDAP_FILTER_ERROR)?;
            out.push(v);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    Ok(out)
}

fn encode_filter_str(raw: &str) -> Result<Vec<u8>, u32> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return encode_filter_str("(objectClass=*)");
    }
    let (ber, rest) = parse_filter_node(trimmed)?;
    if !rest.trim().is_empty() {
        return Err(LDAP_FILTER_ERROR);
    }
    Ok(ber)
}

fn parse_filter_node(s: &str) -> Result<(Vec<u8>, &str), u32> {
    let s = s.trim_start();
    if !s.starts_with('(') {
        // Tolerate bare `attr=val` filters by wrapping them.
        let inner = encode_simple_filter(s)?;
        return Ok((inner, ""));
    }
    // Find matching closing paren
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut end = None;
    for (idx, &b) in bytes.iter().enumerate() {
        if b == b'(' {
            depth += 1;
        } else if b == b')' {
            depth -= 1;
            if depth == 0 {
                end = Some(idx);
                break;
            }
        }
    }
    let end = end.ok_or(LDAP_FILTER_ERROR)?;
    let inner = &s[1..end];
    let rest = &s[end + 1..];
    if inner.is_empty() {
        return Err(LDAP_FILTER_ERROR);
    }
    let first = inner.as_bytes()[0];
    if first == b'&' || first == b'|' {
        let tag = if first == b'&' { 0xA0 } else { 0xA1 };
        let mut children = Vec::new();
        let mut cur = inner[1..].trim();
        if cur.is_empty() {
            return Err(LDAP_FILTER_ERROR);
        }
        while !cur.is_empty() {
            let (child, rem) = parse_filter_node(cur)?;
            children.extend_from_slice(&child);
            cur = rem.trim();
        }
        return Ok((ber_tlv(tag, &children), rest));
    }
    if first == b'!' {
        let (child, rem) = parse_filter_node(&inner[1..])?;
        if !rem.trim().is_empty() {
            return Err(LDAP_FILTER_ERROR);
        }
        return Ok((ber_tlv(0xA2, &child), rest));
    }
    Ok((encode_simple_filter(inner)?, rest))
}

fn encode_simple_filter(expr: &str) -> Result<Vec<u8>, u32> {
    let (attr, op, val) = if let Some((a, v)) = expr.split_once(">=") {
        (a.trim(), ">=", v)
    } else if let Some((a, v)) = expr.split_once("<=") {
        (a.trim(), "<=", v)
    } else if let Some((a, v)) = expr.split_once("~=") {
        (a.trim(), "~=", v)
    } else if let Some((a, v)) = expr.split_once('=') {
        (a.trim(), "=", v)
    } else {
        return Err(LDAP_FILTER_ERROR);
    };
    if attr.is_empty() {
        return Err(LDAP_FILTER_ERROR);
    }
    if op == "=" && val == "*" {
        // present [7] AttributeDescription
        return Ok(ber_tlv(0x87, attr.as_bytes()));
    }
    if op == "=" && val.contains('*') {
        // substrings [4] SEQUENCE { type, SEQUENCE OF CHOICE { [0] initial, [1] any, [2] final } }
        let parts: Vec<&str> = val.split('*').collect();
        let mut seq_body = Vec::new();
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }
            let unesc = unescape_ldap_value(part)?;
            let tag = if i == 0 {
                0x80 // initial
            } else if i + 1 == parts.len() {
                0x82 // final
            } else {
                0x81 // any
            };
            seq_body.extend_from_slice(&ber_tlv(tag, &unesc));
        }
        let mut body = ber_octet_string(attr.as_bytes());
        body.extend_from_slice(&ber_tlv(TAG_SEQUENCE, &seq_body));
        return Ok(ber_tlv(0xA4, &body));
    }
    let tag = match op {
        "=" => 0xA3,  // equalityMatch
        ">=" => 0xA5, // greaterOrEqual
        "<=" => 0xA6, // lessOrEqual
        "~=" => 0xA8, // approxMatch
        _ => return Err(LDAP_FILTER_ERROR),
    };
    let val_bytes = unescape_ldap_value(val)?;
    let mut ava = ber_octet_string(attr.as_bytes());
    ava.extend_from_slice(&ber_octet_string(&val_bytes));
    Ok(ber_tlv(tag, &ava))
}

// ---------------------------------------------------------------------------
// LDAP Session, Messages, Entries, Attributes, and BerVal ABI
// ---------------------------------------------------------------------------

struct LdapSession {
    host: String,
    port: u16,
    protocol_version: u32,
    size_limit: u32,
    time_limit: u32,
    deref_aliases: u32,
    io_timeout_secs: u64,
    next_msg_id: i64,
    stream: Option<TcpStream>,
}

impl LdapSession {
    fn ensure_connected(&mut self) -> Result<&mut TcpStream, u32> {
        if is_offline() {
            return Err(LDAP_SERVER_DOWN);
        }
        if self.stream.is_none() {
            let host = if self.host.is_empty() { "127.0.0.1" } else { self.host.as_str() };
            let addr_str = format!("{host}:{}", self.port);
            let mut addrs = addr_str.to_socket_addrs().map_err(|_| LDAP_SERVER_DOWN)?;
            let timeout = Duration::from_secs(self.io_timeout_secs.max(1));
            let mut last_err = LDAP_SERVER_DOWN;
            let mut connected = None;
            for addr in addrs.by_ref() {
                match TcpStream::connect_timeout(&addr, timeout) {
                    Ok(s) => {
                        let _ = s.set_read_timeout(Some(timeout));
                        let _ = s.set_write_timeout(Some(timeout));
                        connected = Some(s);
                        break;
                    }
                    Err(e) => last_err = map_io_err(&e),
                }
            }
            let stream = connected.ok_or(last_err)?;
            self.stream = Some(stream);
        }
        Ok(self.stream.as_mut().unwrap())
    }

    fn alloc_msg_id(&mut self) -> i64 {
        let id = self.next_msg_id;
        self.next_msg_id = if id >= i32::MAX as i64 { 1 } else { id + 1 };
        id
    }
}

struct LdapEntryNode {
    dn: String,
    attrs: Vec<(String, Vec<Vec<u8>>)>,
    next: *mut LdapEntryNode,
}

struct LdapMessage {
    first: *mut LdapEntryNode,
    msg_type: u32,
}

impl Drop for LdapMessage {
    fn drop(&mut self) {
        let mut cur = self.first;
        while !cur.is_null() {
            unsafe {
                let node = Box::from_raw(cur);
                cur = node.next;
            }
        }
    }
}

struct BerIter {
    index: usize,
}

#[repr(C)]
pub struct BerVal {
    pub bv_len: u32,
    pub bv_val: *mut u8,
}
const _: () = assert!(std::mem::size_of::<BerVal>() == 16);

fn parse_host_and_port(raw_host: &str, default_port: u32) -> (String, u16) {
    let port_fallback = if default_port == 0 { 389 } else { default_port.min(65535) as u16 };
    let first = raw_host.split_whitespace().next().unwrap_or("").trim();
    if first.is_empty() {
        return ("127.0.0.1".into(), port_fallback);
    }
    let stripped = first
        .strip_prefix("ldap://")
        .or_else(|| first.strip_prefix("LDAP://"))
        .unwrap_or(first)
        .trim_end_matches('/');
    if let Some(rest) = stripped.strip_prefix('[') {
        if let Some((ip, after)) = rest.split_once(']') {
            if let Some(pstr) = after.strip_prefix(':') {
                if let Ok(p) = pstr.parse::<u16>() {
                    return (ip.to_string(), p);
                }
            }
            return (ip.to_string(), port_fallback);
        }
    }
    if let Some((h, pstr)) = stripped.rsplit_once(':') {
        if !h.contains(':') {
            if let Ok(p) = pstr.parse::<u16>() {
                return (h.to_string(), p);
            }
        }
    }
    (stripped.to_string(), port_fallback)
}

unsafe fn alloc_heap_cstr(s: &str) -> *mut u8 {
    let b = s.as_bytes();
    let p = crate::heap::alloc(b.len() + 1, false) as *mut u8;
    if p.is_null() {
        return std::ptr::null_mut();
    }
    if !b.is_empty() {
        std::ptr::copy_nonoverlapping(b.as_ptr(), p, b.len());
    }
    std::ptr::write_unaligned(p.add(b.len()), 0);
    p
}

fn parse_ldap_result_code(body: &[u8]) -> Result<u32, u32> {
    let (res_code_elem, _) = ber_decode_one(body)?;
    if res_code_elem.tag != TAG_ENUMERATED {
        return Err(LDAP_DECODING_ERROR);
    }
    let code = ber_decode_int(res_code_elem.val)?;
    Ok(code.max(0) as u32)
}

fn parse_search_result_entry(body: &[u8]) -> Result<LdapEntryNode, u32> {
    let (dn_elem, rest) = ber_decode_one(body)?;
    if dn_elem.tag != TAG_OCTET_STRING {
        return Err(LDAP_DECODING_ERROR);
    }
    let dn = String::from_utf8_lossy(dn_elem.val).into_owned();
    let (attrs_seq, _) = ber_decode_one(rest)?;
    if attrs_seq.tag != TAG_SEQUENCE {
        return Err(LDAP_DECODING_ERROR);
    }

    let mut attrs = Vec::new();
    let mut cur = attrs_seq.val;
    while !cur.is_empty() {
        let (pa_elem, next_cur) = ber_decode_one(cur)?;
        cur = next_cur;
        if pa_elem.tag != TAG_SEQUENCE {
            return Err(LDAP_DECODING_ERROR);
        }
        let (name_elem, pa_rest) = ber_decode_one(pa_elem.val)?;
        if name_elem.tag != TAG_OCTET_STRING {
            return Err(LDAP_DECODING_ERROR);
        }
        let attr_name = String::from_utf8_lossy(name_elem.val).into_owned();
        let (vals_set, _) = ber_decode_one(pa_rest)?;
        if vals_set.tag != TAG_SET {
            return Err(LDAP_DECODING_ERROR);
        }
        let mut vals = Vec::new();
        let mut vcur = vals_set.val;
        while !vcur.is_empty() {
            let (v_elem, next_v) = ber_decode_one(vcur)?;
            vcur = next_v;
            if v_elem.tag != TAG_OCTET_STRING {
                return Err(LDAP_DECODING_ERROR);
            }
            vals.push(v_elem.val.to_vec());
        }
        attrs.push((attr_name, vals));
    }

    Ok(LdapEntryNode {
        dn,
        attrs,
        next: std::ptr::null_mut(),
    })
}

// ---------------------------------------------------------------------------
// Exported wldap32.dll functions
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn ldap_initA(host_name: LPCSTR, port_number: u32) -> usize {
    let raw_host = crate::strings::from_ansi_opt(host_name).unwrap_or_default();
    let (host, port) = parse_host_and_port(&raw_host, port_number);
    let session = Box::new(LdapSession {
        host,
        port,
        protocol_version: 3,
        size_limit: 0,
        time_limit: 0,
        deref_aliases: 0,
        io_timeout_secs: 10,
        next_msg_id: 1,
        stream: None,
    });
    Box::into_raw(session) as usize
}

pub unsafe extern "win64" fn ldap_set_optionA(
    ld: usize,
    option: i32,
    invalue: *const c_void,
) -> u32 {
    if ld == 0 {
        return LDAP_PARAM_ERROR;
    }
    let session = &mut *(ld as *mut LdapSession);
    match option {
        LDAP_OPT_PROTOCOL_VERSION => {
            if invalue.is_null() {
                return LDAP_PARAM_ERROR;
            }
            let ver = std::ptr::read_unaligned(invalue as *const u32);
            if ver != 2 && ver != 3 {
                return LDAP_PARAM_ERROR;
            }
            session.protocol_version = ver;
            LDAP_SUCCESS
        }
        LDAP_OPT_SIZELIMIT => {
            if !invalue.is_null() {
                session.size_limit = std::ptr::read_unaligned(invalue as *const u32);
            }
            LDAP_SUCCESS
        }
        LDAP_OPT_TIMELIMIT | LDAP_OPT_SEND_TIMEOUT => {
            if !invalue.is_null() {
                let secs = std::ptr::read_unaligned(invalue as *const u32);
                session.time_limit = secs;
                if secs > 0 {
                    session.io_timeout_secs = secs as u64;
                }
            }
            LDAP_SUCCESS
        }
        LDAP_OPT_NETWORK_TIMEOUT => {
            if !invalue.is_null() {
                // timeval { tv_sec: i32/i64, tv_usec: i32 }
                let secs = std::ptr::read_unaligned(invalue as *const i32);
                if secs > 0 {
                    session.io_timeout_secs = secs as u64;
                }
            }
            LDAP_SUCCESS
        }
        LDAP_OPT_DEREF => {
            if !invalue.is_null() {
                session.deref_aliases = std::ptr::read_unaligned(invalue as *const u32);
            }
            LDAP_SUCCESS
        }
        LDAP_OPT_REFERRALS | LDAP_OPT_TCP_KEEPALIVE => LDAP_SUCCESS,
        LDAP_OPT_SSL => {
            let on = (invalue as usize) == 1
                || (!invalue.is_null()
                    && (invalue as usize) > 0x1000
                    && std::ptr::read_unaligned(invalue as *const u32) != 0);
            if on {
                LDAP_NOT_SUPPORTED
            } else {
                LDAP_SUCCESS
            }
        }
        _ => LDAP_SUCCESS,
    }
}

pub unsafe extern "win64" fn ldap_simple_bind_sA(
    ld: usize,
    dn: LPCSTR,
    passwd: LPCSTR,
) -> u32 {
    if ld == 0 {
        return LDAP_PARAM_ERROR;
    }
    let session = &mut *(ld as *mut LdapSession);
    let dn_str = crate::strings::from_ansi_opt(dn).unwrap_or_default();
    let pw_str = crate::strings::from_ansi_opt(passwd).unwrap_or_default();

    let msg_id = session.alloc_msg_id();
    let version = session.protocol_version as i64;
    let mut bind_body = ber_integer(version);
    bind_body.extend_from_slice(&ber_octet_string(dn_str.as_bytes()));
    bind_body.extend_from_slice(&ber_tlv(0x80, pw_str.as_bytes())); // simple auth [0]

    let mut msg_body = ber_integer(msg_id);
    msg_body.extend_from_slice(&ber_tlv(TAG_BIND_REQUEST, &bind_body));
    let packet = ber_tlv(TAG_SEQUENCE, &msg_body);

    let stream = match session.ensure_connected() {
        Ok(s) => s,
        Err(e) => return e,
    };
    if let Err(e) = stream.write_all(&packet) {
        session.stream = None;
        return map_io_err(&e);
    }

    let raw = match read_ber_message_from_stream(stream) {
        Ok(r) => r,
        Err(e) => {
            session.stream = None;
            return e;
        }
    };
    let (seq, _) = match ber_decode_one(&raw) {
        Ok(v) if v.0.tag == TAG_SEQUENCE => v,
        _ => return LDAP_DECODING_ERROR,
    };
    let (id_elem, rest) = match ber_decode_one(seq.val) {
        Ok(v) if v.0.tag == TAG_INTEGER => v,
        _ => return LDAP_DECODING_ERROR,
    };
    if ber_decode_int(id_elem.val).ok() != Some(msg_id) {
        return LDAP_PROTOCOL_ERROR;
    }
    let (op, _) = match ber_decode_one(rest) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if op.tag != TAG_BIND_RESPONSE {
        return LDAP_PROTOCOL_ERROR;
    }
    parse_ldap_result_code(op.val).unwrap_or(LDAP_DECODING_ERROR)
}

pub unsafe extern "win64" fn ldap_search_sA(
    ld: usize,
    base: LPCSTR,
    scope: u32,
    filter: LPCSTR,
    attrs: *const LPCSTR,
    attrsonly: u32,
    res: *mut usize,
) -> u32 {
    if !res.is_null() {
        std::ptr::write_unaligned(res, 0);
    }
    if ld == 0 || res.is_null() {
        return LDAP_PARAM_ERROR;
    }
    let session = &mut *(ld as *mut LdapSession);
    let base_str = crate::strings::from_ansi_opt(base).unwrap_or_default();
    let filter_str = crate::strings::from_ansi_opt(filter).unwrap_or_else(|| "(objectClass=*)".into());
    let filter_ber = match encode_filter_str(&filter_str) {
        Ok(b) => b,
        Err(e) => return e,
    };

    let mut attr_list_ber = Vec::new();
    if !attrs.is_null() {
        let mut i = 0usize;
        loop {
            let ap = std::ptr::read_unaligned(attrs.add(i));
            if ap.is_null() {
                break;
            }
            let a = crate::strings::from_ansi(ap);
            attr_list_ber.extend_from_slice(&ber_octet_string(a.as_bytes()));
            i += 1;
        }
    }

    let msg_id = session.alloc_msg_id();
    let mut req_body = ber_octet_string(base_str.as_bytes());
    req_body.extend_from_slice(&ber_enumerated(scope.min(2) as i64));
    req_body.extend_from_slice(&ber_enumerated(session.deref_aliases.min(3) as i64));
    req_body.extend_from_slice(&ber_integer(session.size_limit as i64));
    req_body.extend_from_slice(&ber_integer(session.time_limit as i64));
    req_body.extend_from_slice(&ber_boolean(attrsonly != 0));
    req_body.extend_from_slice(&filter_ber);
    req_body.extend_from_slice(&ber_tlv(TAG_SEQUENCE, &attr_list_ber));

    let mut msg_body = ber_integer(msg_id);
    msg_body.extend_from_slice(&ber_tlv(TAG_SEARCH_REQUEST, &req_body));
    let packet = ber_tlv(TAG_SEQUENCE, &msg_body);

    let stream = match session.ensure_connected() {
        Ok(s) => s,
        Err(e) => return e,
    };
    if let Err(e) = stream.write_all(&packet) {
        session.stream = None;
        return map_io_err(&e);
    }

    let mut first: *mut LdapEntryNode = std::ptr::null_mut();
    let mut tail: *mut LdapEntryNode = std::ptr::null_mut();

    let final_code = loop {
        let raw = match read_ber_message_from_stream(stream) {
            Ok(r) => r,
            Err(e) => {
                session.stream = None;
                let _ = Box::new(LdapMessage {
                    first,
                    msg_type: TAG_SEARCH_RESULT_DONE as u32,
                });
                return e;
            }
        };
        let (seq, _) = match ber_decode_one(&raw) {
            Ok(v) if v.0.tag == TAG_SEQUENCE => v,
            _ => {
                let _ = Box::new(LdapMessage {
                    first,
                    msg_type: TAG_SEARCH_RESULT_DONE as u32,
                });
                return LDAP_DECODING_ERROR;
            }
        };
        let (id_elem, rest) = match ber_decode_one(seq.val) {
            Ok(v) if v.0.tag == TAG_INTEGER => v,
            _ => {
                let _ = Box::new(LdapMessage {
                    first,
                    msg_type: TAG_SEARCH_RESULT_DONE as u32,
                });
                return LDAP_DECODING_ERROR;
            }
        };
        if ber_decode_int(id_elem.val).ok() != Some(msg_id) {
            continue;
        }
        let (op, _) = match ber_decode_one(rest) {
            Ok(v) => v,
            Err(e) => {
                let _ = Box::new(LdapMessage {
                    first,
                    msg_type: TAG_SEARCH_RESULT_DONE as u32,
                });
                return e;
            }
        };
        match op.tag {
            TAG_SEARCH_RESULT_ENTRY => match parse_search_result_entry(op.val) {
                Ok(node) => {
                    let raw_node = Box::into_raw(Box::new(node));
                    if first.is_null() {
                        first = raw_node;
                        tail = raw_node;
                    } else {
                        (&mut *tail).next = raw_node;
                        tail = raw_node;
                    }
                }
                Err(e) => {
                    let _ = Box::new(LdapMessage {
                        first,
                        msg_type: TAG_SEARCH_RESULT_DONE as u32,
                    });
                    return e;
                }
            },
            TAG_SEARCH_RESULT_REF => {
                // Skip search continuation references.
            }
            TAG_SEARCH_RESULT_DONE => {
                break parse_ldap_result_code(op.val).unwrap_or(LDAP_DECODING_ERROR);
            }
            _ => {
                let _ = Box::new(LdapMessage {
                    first,
                    msg_type: TAG_SEARCH_RESULT_DONE as u32,
                });
                return LDAP_PROTOCOL_ERROR;
            }
        }
    };

    let msg = Box::into_raw(Box::new(LdapMessage {
        first,
        msg_type: TAG_SEARCH_RESULT_DONE as u32,
    }));
    std::ptr::write_unaligned(res, msg as usize);
    final_code
}

pub unsafe extern "win64" fn ldap_first_entry(_ld: usize, res: usize) -> usize {
    if res == 0 {
        return 0;
    }
    let msg = &*(res as *const LdapMessage);
    msg.first as usize
}

pub unsafe extern "win64" fn ldap_next_entry(_ld: usize, entry: usize) -> usize {
    if entry == 0 {
        return 0;
    }
    let node = &*(entry as *const LdapEntryNode);
    node.next as usize
}

pub unsafe extern "win64" fn ldap_get_dnA(_ld: usize, entry: usize) -> *mut u8 {
    if entry == 0 {
        return std::ptr::null_mut();
    }
    let node = &*(entry as *const LdapEntryNode);
    alloc_heap_cstr(&node.dn)
}

pub unsafe extern "win64" fn ldap_first_attributeA(
    _ld: usize,
    entry: usize,
    ptr: *mut usize,
) -> *mut u8 {
    if !ptr.is_null() {
        std::ptr::write_unaligned(ptr, 0);
    }
    if entry == 0 || ptr.is_null() {
        return std::ptr::null_mut();
    }
    let node = &*(entry as *const LdapEntryNode);
    if node.attrs.is_empty() {
        return std::ptr::null_mut();
    }
    let iter = Box::into_raw(Box::new(BerIter { index: 1 }));
    std::ptr::write_unaligned(ptr, iter as usize);
    alloc_heap_cstr(&node.attrs[0].0)
}

pub unsafe extern "win64" fn ldap_next_attributeA(
    _ld: usize,
    entry: usize,
    ber: usize,
) -> *mut u8 {
    if entry == 0 || ber == 0 {
        return std::ptr::null_mut();
    }
    let node = &*(entry as *const LdapEntryNode);
    let iter = &mut *(ber as *mut BerIter);
    if iter.index >= node.attrs.len() {
        return std::ptr::null_mut();
    }
    let name = &node.attrs[iter.index].0;
    iter.index += 1;
    alloc_heap_cstr(name)
}

pub unsafe extern "win64" fn ldap_get_values_lenA(
    _ld: usize,
    entry: usize,
    attr: LPCSTR,
) -> *mut *mut BerVal {
    if entry == 0 || attr.is_null() {
        return std::ptr::null_mut();
    }
    let target = crate::strings::from_ansi(attr);
    let node = &*(entry as *const LdapEntryNode);
    let Some((_, values)) = node
        .attrs
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(&target))
    else {
        return std::ptr::null_mut();
    };
    if values.is_empty() {
        return std::ptr::null_mut();
    }
    let arr = crate::heap::alloc((values.len() + 1) * std::mem::size_of::<*mut BerVal>(), true)
        as *mut *mut BerVal;
    if arr.is_null() {
        return std::ptr::null_mut();
    }
    for (i, v) in values.iter().enumerate() {
        let bv = crate::heap::alloc(std::mem::size_of::<BerVal>(), true) as *mut BerVal;
        let data = crate::heap::alloc(v.len() + 1, false) as *mut u8;
        if bv.is_null() || data.is_null() {
            if !bv.is_null() {
                crate::heap::free(bv as *mut c_void);
            }
            if !data.is_null() {
                crate::heap::free(data as *mut c_void);
            }
            ldap_value_free_len(arr);
            return std::ptr::null_mut();
        }
        if !v.is_empty() {
            std::ptr::copy_nonoverlapping(v.as_ptr(), data, v.len());
        }
        std::ptr::write_unaligned(data.add(v.len()), 0);
        std::ptr::write_unaligned(
            bv,
            BerVal {
                bv_len: v.len() as u32,
                bv_val: data,
            },
        );
        std::ptr::write_unaligned(arr.add(i), bv);
    }
    arr
}

pub unsafe extern "win64" fn ldap_value_free_len(vals: *mut *mut BerVal) -> u32 {
    if vals.is_null() {
        return LDAP_SUCCESS;
    }
    let mut i = 0usize;
    loop {
        let bv = std::ptr::read_unaligned(vals.add(i));
        if bv.is_null() {
            break;
        }
        let val = std::ptr::read_unaligned(bv);
        if !val.bv_val.is_null() {
            crate::heap::free(val.bv_val as *mut c_void);
        }
        crate::heap::free(bv as *mut c_void);
        i += 1;
    }
    crate::heap::free(vals as *mut c_void);
    LDAP_SUCCESS
}

pub unsafe extern "win64" fn ldap_msgfree(msg: usize) -> u32 {
    if msg == 0 {
        return 0;
    }
    let boxed = Box::from_raw(msg as *mut LdapMessage);
    boxed.msg_type
}

pub unsafe extern "win64" fn ldap_memfreeA(p: *mut u8) {
    if !p.is_null() {
        crate::heap::free(p as *mut c_void);
    }
}

pub unsafe extern "win64" fn ber_free(ber: usize, _freebuf: i32) {
    if ber != 0 {
        drop(Box::from_raw(ber as *mut BerIter));
    }
}

pub unsafe extern "win64" fn ldap_unbind_s(ld: usize) -> u32 {
    if ld == 0 {
        return LDAP_PARAM_ERROR;
    }
    let mut session = Box::from_raw(ld as *mut LdapSession);
    if let Some(ref mut stream) = session.stream {
        let msg_id = session.next_msg_id;
        let mut body = ber_integer(msg_id);
        body.extend_from_slice(&ber_tlv(TAG_UNBIND_REQUEST, &[]));
        let pkt = ber_tlv(TAG_SEQUENCE, &body);
        let _ = stream.write_all(&pkt);
    }
    LDAP_SUCCESS
}

pub unsafe extern "win64" fn ldap_err2stringA(err: u32) -> *const u8 {
    match err {
        LDAP_SUCCESS => c"Success".as_ptr() as *const u8,
        LDAP_OPERATIONS_ERROR => c"Operations Error".as_ptr() as *const u8,
        LDAP_PROTOCOL_ERROR => c"Protocol Error".as_ptr() as *const u8,
        LDAP_TIMELIMIT_EXCEEDED => c"Time Limit Exceeded".as_ptr() as *const u8,
        LDAP_SIZELIMIT_EXCEEDED => c"Size Limit Exceeded".as_ptr() as *const u8,
        LDAP_AUTH_METHOD_NOT_SUPPORTED => c"Authentication Method Not Supported".as_ptr() as *const u8,
        LDAP_NO_SUCH_OBJECT => c"No Such Object".as_ptr() as *const u8,
        LDAP_INVALID_DN_SYNTAX => c"Invalid DN Syntax".as_ptr() as *const u8,
        LDAP_INVALID_CREDENTIALS => c"Invalid Credentials".as_ptr() as *const u8,
        LDAP_INSUFFICIENT_RIGHTS => c"Insufficient Access Rights".as_ptr() as *const u8,
        LDAP_UNWILLING_TO_PERFORM => c"Unwilling To Perform".as_ptr() as *const u8,
        LDAP_SERVER_DOWN => c"Server Down".as_ptr() as *const u8,
        LDAP_LOCAL_ERROR => c"Local Error".as_ptr() as *const u8,
        LDAP_ENCODING_ERROR => c"Encoding Error".as_ptr() as *const u8,
        LDAP_DECODING_ERROR => c"Decoding Error".as_ptr() as *const u8,
        LDAP_TIMEOUT => c"Timeout".as_ptr() as *const u8,
        LDAP_FILTER_ERROR => c"Filter Error".as_ptr() as *const u8,
        LDAP_PARAM_ERROR => c"Parameter Error".as_ptr() as *const u8,
        LDAP_NO_MEMORY => c"No Memory".as_ptr() as *const u8,
        LDAP_CONNECT_ERROR => c"Connect Error".as_ptr() as *const u8,
        LDAP_NOT_SUPPORTED => c"Not Supported".as_ptr() as *const u8,
        _ => c"Unknown LDAP Error".as_ptr() as *const u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    fn build_ldap_result(msg_id: i64, op_tag: u8, result_code: i64) -> Vec<u8> {
        let mut res = ber_enumerated(result_code);
        res.extend_from_slice(&ber_octet_string(b"")); // matchedDN
        res.extend_from_slice(&ber_octet_string(b"")); // diagnosticMessage
        let mut msg = ber_integer(msg_id);
        msg.extend_from_slice(&ber_tlv(op_tag, &res));
        ber_tlv(TAG_SEQUENCE, &msg)
    }

    fn build_search_entry(msg_id: i64, dn: &str, attrs: &[(&str, &[&[u8]])]) -> Vec<u8> {
        let mut attrs_body = Vec::new();
        for &(name, vals) in attrs {
            let mut set_body = Vec::new();
            for &v in vals {
                set_body.extend_from_slice(&ber_octet_string(v));
            }
            let mut pa = ber_octet_string(name.as_bytes());
            pa.extend_from_slice(&ber_tlv(TAG_SET, &set_body));
            attrs_body.extend_from_slice(&ber_tlv(TAG_SEQUENCE, &pa));
        }
        let mut entry = ber_octet_string(dn.as_bytes());
        entry.extend_from_slice(&ber_tlv(TAG_SEQUENCE, &attrs_body));
        let mut msg = ber_integer(msg_id);
        msg.extend_from_slice(&ber_tlv(TAG_SEARCH_RESULT_ENTRY, &entry));
        ber_tlv(TAG_SEQUENCE, &msg)
    }

    #[test]
    fn loopback_ldapv3_bind_search_iterate_and_unbind() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let _ = sock.set_read_timeout(Some(Duration::from_secs(5)));

            // 1) BindRequest
            let raw_bind = read_ber_message_from_stream(&mut sock).unwrap();
            let (seq, _) = ber_decode_one(&raw_bind).unwrap();
            let (id_el, rest) = ber_decode_one(seq.val).unwrap();
            let bind_id = ber_decode_int(id_el.val).unwrap();
            let (op, _) = ber_decode_one(rest).unwrap();
            assert_eq!(op.tag, TAG_BIND_REQUEST);
            let (ver_el, brest) = ber_decode_one(op.val).unwrap();
            assert_eq!(ber_decode_int(ver_el.val).unwrap(), 3);
            let (dn_el, brest2) = ber_decode_one(brest).unwrap();
            assert_eq!(dn_el.val, b"cn=admin,dc=example,dc=com");
            let (pw_el, _) = ber_decode_one(brest2).unwrap();
            assert_eq!(pw_el.tag, 0x80);
            assert_eq!(pw_el.val, b"secret");
            sock.write_all(&build_ldap_result(bind_id, TAG_BIND_RESPONSE, 0))
                .unwrap();

            // 2) SearchRequest
            let raw_search = read_ber_message_from_stream(&mut sock).unwrap();
            let (sseq, _) = ber_decode_one(&raw_search).unwrap();
            let (sid_el, srest) = ber_decode_one(sseq.val).unwrap();
            let search_id = ber_decode_int(sid_el.val).unwrap();
            let (sop, _) = ber_decode_one(srest).unwrap();
            assert_eq!(sop.tag, TAG_SEARCH_REQUEST);

            let large_bio = vec![b'Z'; 320]; // exercises long-form BER length (>127 bytes)
            let e1 = build_search_entry(
                search_id,
                "uid=alice,ou=users,dc=example,dc=com",
                &[
                    ("cn", &[b"Alice"]),
                    ("mail", &[b"alice@example.com", b"a@example.org"]),
                    ("description", &[large_bio.as_slice()]),
                ],
            );
            let e2 = build_search_entry(
                search_id,
                "uid=bob,ou=users,dc=example,dc=com",
                &[("cn", &[b"Bob"])],
            );
            let done = build_ldap_result(search_id, TAG_SEARCH_RESULT_DONE, 0);
            sock.write_all(&e1).unwrap();
            sock.write_all(&e2).unwrap();
            sock.write_all(&done).unwrap();

            // 3) UnbindRequest
            let raw_unbind = read_ber_message_from_stream(&mut sock).unwrap();
            let (useq, _) = ber_decode_one(&raw_unbind).unwrap();
            let (_, urest) = ber_decode_one(useq.val).unwrap();
            let (uop, _) = ber_decode_one(urest).unwrap();
            assert_eq!(uop.tag, TAG_UNBIND_REQUEST);
        });

        unsafe {
            let ld = ldap_initA(b"127.0.0.1\0".as_ptr(), port as u32);
            assert_ne!(ld, 0);

            let version: u32 = 3;
            assert_eq!(
                ldap_set_optionA(ld, LDAP_OPT_PROTOCOL_VERSION, &version as *const u32 as *const c_void),
                LDAP_SUCCESS
            );
            assert_eq!(
                ldap_simple_bind_sA(
                    ld,
                    b"cn=admin,dc=example,dc=com\0".as_ptr(),
                    b"secret\0".as_ptr()
                ),
                LDAP_SUCCESS
            );

            let attrs: [LPCSTR; 3] = [b"cn\0".as_ptr(), b"mail\0".as_ptr(), std::ptr::null()];
            let mut msg: usize = 0;
            assert_eq!(
                ldap_search_sA(
                    ld,
                    b"dc=example,dc=com\0".as_ptr(),
                    2,
                    b"(&(objectClass=person)(cn=A*))\0".as_ptr(),
                    attrs.as_ptr(),
                    0,
                    &mut msg
                ),
                LDAP_SUCCESS
            );
            assert_ne!(msg, 0);

            // Entry 1: Alice
            let e1 = ldap_first_entry(ld, msg);
            assert_ne!(e1, 0);
            let dn1_ptr = ldap_get_dnA(ld, e1);
            assert_eq!(
                crate::strings::from_ansi(dn1_ptr),
                "uid=alice,ou=users,dc=example,dc=com"
            );
            ldap_memfreeA(dn1_ptr);

            let mut ber: usize = 0;
            let a1 = ldap_first_attributeA(ld, e1, &mut ber);
            assert_eq!(crate::strings::from_ansi(a1), "cn");
            ldap_memfreeA(a1);
            let a2 = ldap_next_attributeA(ld, e1, ber);
            assert_eq!(crate::strings::from_ansi(a2), "mail");
            ldap_memfreeA(a2);
            let a3 = ldap_next_attributeA(ld, e1, ber);
            assert_eq!(crate::strings::from_ansi(a3), "description");
            ldap_memfreeA(a3);
            assert!(ldap_next_attributeA(ld, e1, ber).is_null());
            ber_free(ber, 0);

            // Long-form BER attribute value (> 127 bytes) decoded verbatim
            let desc_vals = ldap_get_values_lenA(ld, e1, b"description\0".as_ptr());
            assert!(!desc_vals.is_null());
            let dp0 = std::ptr::read_unaligned(desc_vals.add(0));
            let d0 = std::ptr::read_unaligned(dp0);
            assert_eq!(d0.bv_len, 320);
            assert_eq!(std::slice::from_raw_parts(d0.bv_val, 320), &[b'Z'; 320]);
            assert_eq!(ldap_value_free_len(desc_vals), LDAP_SUCCESS);

            // Values (case-insensitive attribute lookup)
            let vals = ldap_get_values_lenA(ld, e1, b"MAIL\0".as_ptr());
            assert!(!vals.is_null());
            let p0 = std::ptr::read_unaligned(vals.add(0));
            let p1 = std::ptr::read_unaligned(vals.add(1));
            let bv0 = std::ptr::read_unaligned(p0);
            let bv1 = std::ptr::read_unaligned(p1);
            assert!(std::ptr::read_unaligned(vals.add(2)).is_null());
            assert_eq!(
                std::slice::from_raw_parts(bv0.bv_val, bv0.bv_len as usize),
                b"alice@example.com"
            );
            assert_eq!(
                std::slice::from_raw_parts(bv1.bv_val, bv1.bv_len as usize),
                b"a@example.org"
            );
            assert_eq!(ldap_value_free_len(vals), LDAP_SUCCESS);

            // Entry 2: Bob
            let e2 = ldap_next_entry(ld, e1);
            assert_ne!(e2, 0);
            let dn2_ptr = ldap_get_dnA(ld, e2);
            assert_eq!(
                crate::strings::from_ansi(dn2_ptr),
                "uid=bob,ou=users,dc=example,dc=com"
            );
            ldap_memfreeA(dn2_ptr);
            assert_eq!(ldap_next_entry(ld, e2), 0);

            assert_eq!(ldap_msgfree(msg), TAG_SEARCH_RESULT_DONE as u32);
            assert_eq!(ldap_unbind_s(ld), LDAP_SUCCESS);
        }

        server.join().unwrap();
    }

    struct EnvGuard {
        prev: Option<String>,
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(v) => std::env::set_var("PORT_OFFLINE", v),
                None => std::env::remove_var("PORT_OFFLINE"),
            }
        }
    }

    #[test]
    fn offline_mode_blocks_ldap_connections() {
        let _guard = EnvGuard {
            prev: std::env::var("PORT_OFFLINE").ok(),
        };
        std::env::set_var("PORT_OFFLINE", "1");
        unsafe {
            let ld = ldap_initA(b"127.0.0.1\0".as_ptr(), 389);
            assert_ne!(ld, 0);
            assert_eq!(
                ldap_simple_bind_sA(ld, std::ptr::null(), std::ptr::null()),
                LDAP_SERVER_DOWN
            );
            assert_eq!(
                crate::strings::from_ansi(ldap_err2stringA(LDAP_SERVER_DOWN)),
                "Server Down"
            );
            assert_eq!(ldap_unbind_s(ld), LDAP_SUCCESS);
        }
    }
}
