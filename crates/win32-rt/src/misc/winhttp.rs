//! `winhttp.dll` / `wininet.dll` / `sensapi.dll`: real HTTP/1.1 + HTTPS client
//! (TLS 1.2/1.3 via `rustls` + embedded `webpki-roots`), URL cracking/building,
//! proxy configuration queries, and POSIX interface network-liveness detection.
//!
//! Works identically on macOS (`aarch64-apple-darwin`) and iOS
//! (`aarch64-apple-ios`) with zero platform-specific UI frameworks.

use crate::types::*;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::client::WebPkiServerVerifier;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{
    CertificateError, ClientConfig, ClientConnection, DigitallySignedStruct, RootCertStore,
    SignatureScheme, StreamOwned,
};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Win32 / WinHTTP / WinINet / SENSAPI constants
// ---------------------------------------------------------------------------

const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

const ERROR_WINHTTP_TIMEOUT: u32 = 12002;
const ERROR_WINHTTP_INVALID_URL: u32 = 12005;
const ERROR_WINHTTP_UNRECOGNIZED_SCHEME: u32 = 12006;
const ERROR_WINHTTP_NAME_NOT_RESOLVED: u32 = 12007;
const ERROR_WINHTTP_INVALID_OPTION: u32 = 12009;
const ERROR_WINHTTP_OPERATION_CANCELLED: u32 = 12017;
const ERROR_WINHTTP_INCORRECT_HANDLE_TYPE: u32 = 12018;
const ERROR_WINHTTP_INCORRECT_HANDLE_STATE: u32 = 12019;
const ERROR_WINHTTP_CANNOT_CONNECT: u32 = 12029;
const ERROR_WINHTTP_CONNECTION_ERROR: u32 = 12030;
const ERROR_WINHTTP_SECURE_CERT_DATE_INVALID: u32 = 12037;
const ERROR_WINHTTP_SECURE_CERT_CN_INVALID: u32 = 12038;
const ERROR_WINHTTP_SECURE_INVALID_CA: u32 = 12045;
const ERROR_WINHTTP_HEADER_NOT_FOUND: u32 = 12150;
const ERROR_WINHTTP_INVALID_SERVER_RESPONSE: u32 = 12152;
const ERROR_WINHTTP_INVALID_HEADER: u32 = 12153;
const ERROR_WINHTTP_INVALID_QUERY_REQUEST: u32 = 12154;
const ERROR_WINHTTP_HEADER_ALREADY_EXISTS: u32 = 12155;
const ERROR_WINHTTP_REDIRECT_FAILED: u32 = 12156;
const ERROR_WINHTTP_SECURE_CHANNEL_ERROR: u32 = 12157;
const ERROR_WINHTTP_AUTODETECTION_FAILED: u32 = 12180;

const WINHTTP_ACCESS_TYPE_NO_PROXY: u32 = 1;

const WINHTTP_FLAG_SECURE: u32 = 0x0080_0000;

const WINHTTP_HANDLE_TYPE_SESSION: u32 = 1;
const WINHTTP_HANDLE_TYPE_CONNECT: u32 = 2;
const WINHTTP_HANDLE_TYPE_REQUEST: u32 = 3;

const WINHTTP_SCHEME_HTTP: i32 = 1;
const WINHTTP_SCHEME_HTTPS: i32 = 2;
const WININET_SCHEME_HTTP: i32 = 3;
const WININET_SCHEME_HTTPS: i32 = 4;

const INTERNET_DEFAULT_HTTP_PORT: u16 = 80;
const INTERNET_DEFAULT_HTTPS_PORT: u16 = 443;

// WinHttpAddRequestHeaders / HttpAddRequestHeaders modifiers
const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: u32 = 0x0100_0000;
const WINHTTP_ADDREQ_FLAG_ADD_IF_NEW: u32 = 0x1000_0000;
const WINHTTP_ADDREQ_FLAG_ADD: u32 = 0x2000_0000;
const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: u32 = 0x4000_0000;
const WINHTTP_ADDREQ_FLAG_REPLACE: u32 = 0x8000_0000;

// WinHttpSetOption / WinHttpQueryOption identifiers
const WINHTTP_OPTION_CALLBACK: u32 = 1;
const WINHTTP_OPTION_RESOLVE_TIMEOUT: u32 = 2;
const WINHTTP_OPTION_CONNECT_TIMEOUT: u32 = 3;
const WINHTTP_OPTION_CONNECT_RETRIES: u32 = 4;
const WINHTTP_OPTION_SEND_TIMEOUT: u32 = 5;
const WINHTTP_OPTION_RECEIVE_TIMEOUT: u32 = 6;
const WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT: u32 = 7;
const WINHTTP_OPTION_HANDLE_TYPE: u32 = 9;
const WINHTTP_OPTION_READ_BUFFER_SIZE: u32 = 12;
const WINHTTP_OPTION_WRITE_BUFFER_SIZE: u32 = 13;
const WINHTTP_OPTION_PARENT_HANDLE: u32 = 21;
const WINHTTP_OPTION_SECURITY_FLAGS: u32 = 31;
const WINHTTP_OPTION_URL: u32 = 34;
const WINHTTP_OPTION_PROXY: u32 = 38;
const WINHTTP_OPTION_USER_AGENT: u32 = 41;
const WINHTTP_OPTION_CONTEXT_VALUE: u32 = 45;
const WINHTTP_OPTION_DISABLE_FEATURE: u32 = 63;
const WINHTTP_OPTION_ENABLE_FEATURE: u32 = 79;
const WINHTTP_OPTION_SECURE_PROTOCOLS: u32 = 84;
const WINHTTP_OPTION_REDIRECT_POLICY: u32 = 88;
const WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS: u32 = 89;
const WINHTTP_OPTION_MAX_CONNS_PER_SERVER: u32 = 103;
const WINHTTP_OPTION_DECOMPRESSION: u32 = 118;
const WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 133;
const WINHTTP_OPTION_HTTP_PROTOCOL_USED: u32 = 134;

// WinINet-specific option overrides where ID differs from WinHTTP
const INTERNET_OPTION_CONNECT_TIMEOUT: u32 = 2;
const INTERNET_OPTION_CONNECT_RETRIES: u32 = 3;

const WINHTTP_DISABLE_REDIRECTS: u32 = 0x0000_0002;

const WINHTTP_OPTION_REDIRECT_POLICY_NEVER: u32 = 0;
const WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP: u32 = 1;
const WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS: u32 = 2;

// Security flags
const SECURITY_FLAG_SECURE: u32 = 0x0000_0001;
const SECURITY_FLAG_IGNORE_UNKNOWN_CA: u32 = 0x0000_0100;
const SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE: u32 = 0x0000_0200;
const SECURITY_FLAG_IGNORE_CERT_CN_INVALID: u32 = 0x0000_1000;
const SECURITY_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 0x0000_2000;
const SECURITY_FLAG_STRENGTH_STRONG: u32 = 0x2000_0000;
const SECURITY_IGNORE_MASK: u32 = SECURITY_FLAG_IGNORE_UNKNOWN_CA
    | SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE
    | SECURITY_FLAG_IGNORE_CERT_CN_INVALID
    | SECURITY_FLAG_IGNORE_CERT_DATE_INVALID;

// WinINet request flags
const INTERNET_FLAG_NO_AUTO_REDIRECT: u32 = 0x0020_0000;
const INTERNET_FLAG_IGNORE_CERT_CN_INVALID: u32 = 0x0000_1000;
const INTERNET_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 0x0000_2000;

// WinHttpQueryHeaders / HttpQueryInfo flags & IDs
const WINHTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 0x8000_0000;
const WINHTTP_QUERY_FLAG_SYSTEMTIME: u32 = 0x4000_0000;
const WINHTTP_QUERY_FLAG_NUMBER: u32 = 0x2000_0000;
const WINHTTP_QUERY_FLAG_COALESCE: u32 = 0x1000_0000;
const WINHTTP_QUERY_FLAG_NUMBER64: u32 = 0x0800_0000;

const WINHTTP_QUERY_VERSION: u32 = 18;
const WINHTTP_QUERY_STATUS_CODE: u32 = 19;
const WINHTTP_QUERY_STATUS_TEXT: u32 = 20;
const WINHTTP_QUERY_RAW_HEADERS: u32 = 21;
const WINHTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22;
const WINHTTP_QUERY_REQUEST_METHOD: u32 = 45;
const WINHTTP_QUERY_CUSTOM: u32 = 65535;

// URL crack / create flags
const ICU_DECODE: u32 = 0x1000_0000;
const ICU_ESCAPE: u32 = 0x8000_0000;

// Auth constants
const WINHTTP_AUTH_TARGET_SERVER: u32 = 0;
const WINHTTP_AUTH_TARGET_PROXY: u32 = 1;
const WINHTTP_AUTH_SCHEME_BASIC: u32 = 0x0000_0001;
const WINHTTP_AUTH_SCHEME_NTLM: u32 = 0x0000_0002;
const WINHTTP_AUTH_SCHEME_DIGEST: u32 = 0x0000_0008;
const WINHTTP_AUTH_SCHEME_NEGOTIATE: u32 = 0x0000_0010;

// SENSAPI / WinINet connection flags
const NETWORK_ALIVE_LAN: u32 = 0x0000_0001;
const INTERNET_CONNECTION_LAN: u32 = 0x0000_0002;

const WINHTTP_INVALID_STATUS_CALLBACK: usize = usize::MAX;

const DEFAULT_MAX_REDIRECTS: u32 = 10;
const MAX_HEADER_BYTES: usize = 512 * 1024;
const MAX_CHUNK_SIZE: usize = 256 * 1024 * 1024;

// ---------------------------------------------------------------------------
// Win64 ABI structures
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct WINHTTP_PROXY_INFO {
    pub access_type: u32,
    pub proxy: LPWSTR,
    pub bypass: LPWSTR,
}

#[repr(C)]
pub struct WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
    pub auto_detect: BOOL,
    pub auto_config_url: LPWSTR,
    pub proxy: LPWSTR,
    pub bypass: LPWSTR,
}

#[repr(C)]
pub struct WINHTTP_AUTOPROXY_OPTIONS {
    pub dw_flags: u32,
    pub dw_auto_detect_flags: u32,
    pub lpsz_auto_config_url: LPCWSTR,
    pub lpv_reserved: LPVOID,
    pub dw_reserved: u32,
    pub f_auto_logon_if_challenged: BOOL,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct URL_COMPONENTSW {
    pub dw_struct_size: u32,
    pub lpsz_scheme: LPWSTR,
    pub dw_scheme_length: u32,
    pub n_scheme: i32,
    pub lpsz_host_name: LPWSTR,
    pub dw_host_name_length: u32,
    pub n_port: u16,
    pub lpsz_user_name: LPWSTR,
    pub dw_user_name_length: u32,
    pub lpsz_password: LPWSTR,
    pub dw_password_length: u32,
    pub lpsz_url_path: LPWSTR,
    pub dw_url_path_length: u32,
    pub lpsz_extra_info: LPWSTR,
    pub dw_extra_info_length: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct URL_COMPONENTSA {
    pub dw_struct_size: u32,
    pub lpsz_scheme: LPSTR,
    pub dw_scheme_length: u32,
    pub n_scheme: i32,
    pub lpsz_host_name: LPSTR,
    pub dw_host_name_length: u32,
    pub n_port: u16,
    pub lpsz_user_name: LPSTR,
    pub dw_user_name_length: u32,
    pub lpsz_password: LPSTR,
    pub dw_password_length: u32,
    pub lpsz_url_path: LPSTR,
    pub dw_url_path_length: u32,
    pub lpsz_extra_info: LPSTR,
    pub dw_extra_info_length: u32,
}

// ---------------------------------------------------------------------------
// Unaligned guest memory & string helpers
// ---------------------------------------------------------------------------

#[inline]
unsafe fn read_u32(p: *const u32) -> u32 {
    std::ptr::read_unaligned(p)
}

#[inline]
unsafe fn write_u32(p: *mut u32, v: u32) {
    std::ptr::write_unaligned(p, v);
}

#[inline]
unsafe fn read_usize(p: *const usize) -> usize {
    std::ptr::read_unaligned(p)
}

#[inline]
unsafe fn write_usize(p: *mut usize, v: usize) {
    std::ptr::write_unaligned(p, v);
}

unsafe fn read_wide_z(p: LPCWSTR) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let mut units = Vec::new();
    let mut i = 0usize;
    loop {
        let u = std::ptr::read_unaligned(p.add(i));
        if u == 0 {
            break;
        }
        units.push(u);
        i += 1;
    }
    Some(String::from_utf16_lossy(&units))
}

/// Read UTF-16 string where `len == u32::MAX` (`-1`) means NUL-terminated and `len == 0` means empty.
unsafe fn read_wide_len_neg1(p: LPCWSTR, len: u32) -> Option<String> {
    if p.is_null() {
        return None;
    }
    if len == u32::MAX {
        return read_wide_z(p);
    }
    let mut units = Vec::with_capacity((len as usize).min(4096));
    for i in 0..len as usize {
        let u = std::ptr::read_unaligned(p.add(i));
        if u == 0 {
            break;
        }
        units.push(u);
    }
    Some(String::from_utf16_lossy(&units))
}

/// Read UTF-16 string where `len == 0` means NUL-terminated.
unsafe fn read_wide_len_zero(p: LPCWSTR, len: u32) -> Option<String> {
    if p.is_null() {
        return None;
    }
    if len == 0 {
        return read_wide_z(p);
    }
    let mut units = Vec::with_capacity((len as usize).min(4096));
    for i in 0..len as usize {
        let u = std::ptr::read_unaligned(p.add(i));
        if u == 0 {
            break;
        }
        units.push(u);
    }
    Some(String::from_utf16_lossy(&units))
}

unsafe fn read_ansi_z(p: LPCSTR) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let mut bytes = Vec::new();
    let mut i = 0usize;
    loop {
        let b = std::ptr::read_unaligned(p.add(i));
        if b == 0 {
            break;
        }
        bytes.push(b);
        i += 1;
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

unsafe fn read_ansi_len_neg1(p: LPCSTR, len: u32) -> Option<String> {
    if p.is_null() {
        return None;
    }
    if len == u32::MAX {
        return read_ansi_z(p);
    }
    let mut bytes = Vec::with_capacity((len as usize).min(4096));
    for i in 0..len as usize {
        let b = std::ptr::read_unaligned(p.add(i));
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

unsafe fn read_ansi_len_zero(p: LPCSTR, len: u32) -> Option<String> {
    if p.is_null() {
        return None;
    }
    if len == 0 {
        return read_ansi_z(p);
    }
    let mut bytes = Vec::with_capacity((len as usize).min(4096));
    for i in 0..len as usize {
        let b = std::ptr::read_unaligned(p.add(i));
        if b == 0 {
            break;
        }
        bytes.push(b);
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

unsafe fn write_wide_units(dst: *mut u16, units: &[u16]) {
    for (i, &u) in units.iter().enumerate() {
        std::ptr::write_unaligned(dst.add(i), u);
    }
}

/// Write a UTF-16 string (plus NUL) into a byte buffer whose size in bytes is tracked by `buf_len`.
/// On `ERROR_INSUFFICIENT_BUFFER`, writes required byte size (including NUL) to `*buf_len`.
/// On success, writes byte size *excluding* NUL to `*buf_len`.
unsafe fn write_wide_bytes_out(s: &str, buf: *mut u8, buf_len: *mut u32) -> BOOL {
    if buf_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let units: Vec<u16> = s.encode_utf16().collect();
    let needed_bytes = (units.len() + 1) * 2;
    let cap_bytes = read_u32(buf_len) as usize;
    if buf.is_null() || cap_bytes < needed_bytes {
        write_u32(buf_len, needed_bytes as u32);
        crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    let dst = buf as *mut u16;
    write_wide_units(dst, &units);
    std::ptr::write_unaligned(dst.add(units.len()), 0u16);
    write_u32(buf_len, (units.len() * 2) as u32);
    TRUE
}

/// Same as `write_wide_bytes_out`, for ANSI/UTF-8 strings.
unsafe fn write_ansi_bytes_out(s: &str, buf: *mut u8, buf_len: *mut u32) -> BOOL {
    if buf_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let bytes = s.as_bytes();
    let needed = bytes.len() + 1;
    let cap = read_u32(buf_len) as usize;
    if buf.is_null() || cap < needed {
        write_u32(buf_len, needed as u32);
        crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
    }
    std::ptr::write_unaligned(buf.add(bytes.len()), 0u8);
    write_u32(buf_len, bytes.len() as u32);
    TRUE
}

// ---------------------------------------------------------------------------
// Offline mode & network liveness helpers
// ---------------------------------------------------------------------------

fn is_offline_mode() -> bool {
    match std::env::var("PORT_OFFLINE") {
        Ok(v) => {
            let t = v.trim();
            !t.is_empty() && t != "0" && !t.eq_ignore_ascii_case("false")
        }
        Err(_) => false,
    }
}

/// Inspect POSIX network interfaces via `libc::getifaddrs` (macOS + iOS).
/// Returns `true` if at least one non-loopback `IFF_UP | IFF_RUNNING` interface
/// has an assigned IPv4 or IPv6 address.
fn has_active_network_interface() -> bool {
    unsafe {
        let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) != 0 || ifap.is_null() {
            return false;
        }
        let mut alive = false;
        let mut cur = ifap;
        let want_flags = (libc::IFF_UP | libc::IFF_RUNNING) as libc::c_uint;
        let loopback_flag = libc::IFF_LOOPBACK as libc::c_uint;
        while !cur.is_null() {
            let ifa = &*cur;
            let flags = ifa.ifa_flags;
            if (flags & want_flags) == want_flags
                && (flags & loopback_flag) == 0
                && !ifa.ifa_addr.is_null()
            {
                let family = (*ifa.ifa_addr).sa_family;
                if family == libc::AF_INET as libc::sa_family_t
                    || family == libc::AF_INET6 as libc::sa_family_t
                {
                    alive = true;
                    break;
                }
            }
            cur = ifa.ifa_next;
        }
        libc::freeifaddrs(ifap);
        alive
    }
}

// ---------------------------------------------------------------------------
// Handle hierarchy & global table
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct Timeouts {
    resolve_ms: i32,
    connect_ms: i32,
    send_ms: i32,
    receive_ms: i32,
}

impl Default for Timeouts {
    fn default() -> Self {
        Self {
            resolve_ms: 0,
            connect_ms: 60_000,
            send_ms: 30_000,
            receive_ms: 30_000,
        }
    }
}

#[derive(Clone)]
struct SessionState {
    user_agent: String,
    timeouts: Timeouts,
    redirect_policy: u32,
    max_redirects: u32,
    disable_flags: u32,
    security_flags: u32,
    context: usize,
    status_callback: usize,
}

#[derive(Clone)]
struct ConnectionState {
    parent_session: usize,
    host: String,
    port: u16,
    user_agent: String,
    timeouts: Timeouts,
    redirect_policy: u32,
    max_redirects: u32,
    disable_flags: u32,
    security_flags: u32,
    context: usize,
    status_callback: usize,
}

struct HttpResponse {
    version: String,
    status_code: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    read_pos: usize,
}

struct RequestState {
    parent_connect: usize,
    host: String,
    port: u16,
    is_secure: bool,
    verb: String,
    path: String,
    version: String,
    user_agent: String,
    headers: Vec<(String, String)>,
    send_headers: Vec<(String, String)>,
    request_body: Vec<u8>,
    sent: bool,
    timeouts: Timeouts,
    redirect_policy: u32,
    max_redirects: u32,
    disable_flags: u32,
    security_flags: u32,
    context: usize,
    status_callback: usize,
    effective_url: String,
    response: Option<HttpResponse>,
}

enum WinHttpHandle {
    Session(SessionState),
    Connection(ConnectionState),
    Request(RequestState),
}

static NEXT_HANDLE: AtomicUsize = AtomicUsize::new(0x5748_0001);
static HANDLES: LazyLock<Mutex<HashMap<usize, WinHttpHandle>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn alloc_handle(h: WinHttpHandle) -> usize {
    let id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    HANDLES.lock().unwrap().insert(id, h);
    id
}

fn format_url(is_secure: bool, host: &str, port: u16, path: &str) -> String {
    let scheme = if is_secure { "https" } else { "http" };
    let default_port = if is_secure {
        INTERNET_DEFAULT_HTTPS_PORT
    } else {
        INTERNET_DEFAULT_HTTP_PORT
    };
    let host_disp = if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]")
    } else {
        host.to_string()
    };
    let path_disp = if path.is_empty() {
        "/"
    } else if path.starts_with('/') || path == "*" {
        path
    } else {
        // Prepend '/' when building full URL
        return if port == default_port || port == 0 {
            format!("{scheme}://{host_disp}/{path}")
        } else {
            format!("{scheme}://{host_disp}:{port}/{path}")
        };
    };
    if port == default_port || port == 0 {
        format!("{scheme}://{host_disp}{path_disp}")
    } else {
        format!("{scheme}://{host_disp}:{port}{path_disp}")
    }
}

// ---------------------------------------------------------------------------
// Header manipulation
// ---------------------------------------------------------------------------

fn apply_header_block(
    headers: &mut Vec<(String, String)>,
    raw: &str,
    modifiers: u32,
) -> Result<(), u32> {
    let add_if_new = (modifiers & WINHTTP_ADDREQ_FLAG_ADD_IF_NEW) != 0;
    let replace = (modifiers & WINHTTP_ADDREQ_FLAG_REPLACE) != 0;
    let add = (modifiers & WINHTTP_ADDREQ_FLAG_ADD) != 0;
    let coalesce_comma = (modifiers & WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA) != 0;
    let coalesce_semi = (modifiers & WINHTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON) != 0;

    for line in raw.split('\n') {
        let line = line.trim_end_matches('\r').trim();
        if line.is_empty() {
            continue;
        }
        let Some((name_raw, val_raw)) = line.split_once(':') else {
            return Err(ERROR_WINHTTP_INVALID_HEADER);
        };
        let name = name_raw.trim();
        if name.is_empty() {
            return Err(ERROR_WINHTTP_INVALID_HEADER);
        }
        let value = val_raw.trim();

        let existing_idx = headers.iter().position(|(k, _)| k.eq_ignore_ascii_case(name));

        if add_if_new && !replace {
            if existing_idx.is_some() {
                return Err(ERROR_WINHTTP_HEADER_ALREADY_EXISTS);
            }
            if !value.is_empty() {
                headers.push((name.to_string(), value.to_string()));
            }
            continue;
        }

        if replace {
            if value.is_empty() {
                headers.retain(|(k, _)| !k.eq_ignore_ascii_case(name));
            } else if existing_idx.is_some() {
                headers.retain(|(k, _)| !k.eq_ignore_ascii_case(name));
                headers.push((name.to_string(), value.to_string()));
            } else if add || add_if_new {
                headers.push((name.to_string(), value.to_string()));
            } else {
                return Err(ERROR_WINHTTP_HEADER_NOT_FOUND);
            }
            continue;
        }

        if coalesce_comma || coalesce_semi {
            let sep = if coalesce_semi { "; " } else { ", " };
            if let Some(idx) = existing_idx {
                if !value.is_empty() {
                    if !headers[idx].1.is_empty() {
                        headers[idx].1.push_str(sep);
                    }
                    headers[idx].1.push_str(value);
                }
            } else if !value.is_empty() {
                headers.push((name.to_string(), value.to_string()));
            }
            continue;
        }

        // Default or WINHTTP_ADDREQ_FLAG_ADD: append header entry.
        headers.push((name.to_string(), value.to_string()));
    }
    Ok(())
}

fn has_header(headers: &[(String, String)], name: &str) -> bool {
    headers.iter().any(|(k, _)| k.eq_ignore_ascii_case(name))
}

fn find_header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.as_str())
}

fn query_id_to_header_name(id: u32) -> Option<&'static str> {
    match id {
        0 => Some("MIME-Version"),
        1 => Some("Content-Type"),
        2 => Some("Content-Transfer-Encoding"),
        3 => Some("Content-ID"),
        4 => Some("Content-Description"),
        5 => Some("Content-Length"),
        6 => Some("Content-Language"),
        7 => Some("Allow"),
        8 => Some("Public"),
        9 => Some("Date"),
        10 => Some("Expires"),
        11 => Some("Last-Modified"),
        12 => Some("Message-ID"),
        13 => Some("URI"),
        14 => Some("Derived-From"),
        15 => Some("Cost"),
        17 => Some("Pragma"),
        23 => Some("Connection"),
        24 => Some("Accept"),
        25 => Some("Accept-Charset"),
        26 => Some("Accept-Encoding"),
        27 => Some("Accept-Language"),
        28 => Some("Authorization"),
        29 => Some("Content-Encoding"),
        30 => Some("Forwarded"),
        31 => Some("From"),
        32 => Some("If-Modified-Since"),
        33 => Some("Location"),
        34 => Some("Orig-URI"),
        35 => Some("Referer"),
        36 => Some("Retry-After"),
        37 => Some("Server"),
        38 => Some("Title"),
        39 => Some("User-Agent"),
        40 => Some("WWW-Authenticate"),
        41 => Some("Proxy-Authenticate"),
        42 => Some("Accept-Ranges"),
        43 => Some("Set-Cookie"),
        44 => Some("Cookie"),
        46 => Some("Refresh"),
        47 => Some("Content-Disposition"),
        48 => Some("Age"),
        49 => Some("Cache-Control"),
        50 => Some("Content-Base"),
        51 => Some("Content-Location"),
        52 => Some("Content-MD5"),
        53 => Some("Content-Range"),
        54 => Some("ETag"),
        55 => Some("Host"),
        56 => Some("If-Match"),
        57 => Some("If-None-Match"),
        58 => Some("If-Range"),
        59 => Some("If-Unmodified-Since"),
        60 => Some("Max-Forwards"),
        61 => Some("Proxy-Authorization"),
        62 => Some("Range"),
        63 => Some("Transfer-Encoding"),
        64 => Some("Upgrade"),
        65 => Some("Vary"),
        66 => Some("Via"),
        67 => Some("Warning"),
        68 => Some("Expect"),
        69 => Some("Proxy-Connection"),
        70 => Some("Unless-Modified-Since"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// TLS configuration (`rustls` 0.23 + `webpki-roots`)
// ---------------------------------------------------------------------------

static TLS_CRYPTO_PROVIDER: LazyLock<Arc<rustls::crypto::CryptoProvider>> =
    LazyLock::new(|| Arc::new(rustls::crypto::ring::default_provider()));

static TLS_ROOT_STORE: LazyLock<Arc<RootCertStore>> = LazyLock::new(|| {
    let mut store = RootCertStore::empty();
    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    Arc::new(store)
});

static DEFAULT_TLS_CONFIG: LazyLock<Arc<ClientConfig>> = LazyLock::new(|| {
    let mut cfg = ClientConfig::builder_with_provider(Arc::clone(&TLS_CRYPTO_PROVIDER))
        .with_safe_default_protocol_versions()
        .expect("rustls ring provider supports default TLS versions")
        .with_root_certificates(Arc::clone(&TLS_ROOT_STORE))
        .with_no_client_auth();
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    Arc::new(cfg)
});

#[derive(Debug)]
struct SelectiveCertVerifier {
    inner: Arc<WebPkiServerVerifier>,
    ignore_flags: u32,
}

impl ServerCertVerifier for SelectiveCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        match self
            .inner
            .verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now)
        {
            Ok(v) => Ok(v),
            Err(rustls::Error::InvalidCertificate(ref cert_err)) => {
                let allowed = match cert_err {
                    CertificateError::Expired
                    | CertificateError::NotValidYet
                    | CertificateError::ExpiredContext { .. }
                    | CertificateError::NotValidYetContext { .. } => {
                        (self.ignore_flags & SECURITY_FLAG_IGNORE_CERT_DATE_INVALID) != 0
                    }
                    CertificateError::NotValidForName
                    | CertificateError::NotValidForNameContext { .. } => {
                        (self.ignore_flags & SECURITY_FLAG_IGNORE_CERT_CN_INVALID) != 0
                    }
                    CertificateError::InvalidPurpose
                    | CertificateError::InvalidPurposeContext { .. } => {
                        (self.ignore_flags & SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE) != 0
                    }
                    _ => (self.ignore_flags & SECURITY_FLAG_IGNORE_UNKNOWN_CA) != 0,
                };
                if allowed {
                    Ok(ServerCertVerified::assertion())
                } else {
                    Err(rustls::Error::InvalidCertificate(cert_err.clone()))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls12_signature(message, cert, dss)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls13_signature(message, cert, dss)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.inner.supported_verify_schemes()
    }
}

fn tls_config_for(security_flags: u32) -> Result<Arc<ClientConfig>, u32> {
    let ignore = security_flags & SECURITY_IGNORE_MASK;
    if ignore == 0 {
        return Ok(Arc::clone(&DEFAULT_TLS_CONFIG));
    }
    let inner = WebPkiServerVerifier::builder_with_provider(
        Arc::clone(&TLS_ROOT_STORE),
        Arc::clone(&TLS_CRYPTO_PROVIDER),
    )
    .build()
    .map_err(|_| ERROR_WINHTTP_SECURE_CHANNEL_ERROR)?;
    let verifier = Arc::new(SelectiveCertVerifier {
        inner,
        ignore_flags: ignore,
    });
    let mut cfg = ClientConfig::builder_with_provider(Arc::clone(&TLS_CRYPTO_PROVIDER))
        .with_safe_default_protocol_versions()
        .map_err(|_| ERROR_WINHTTP_SECURE_CHANNEL_ERROR)?
        .dangerous()
        .with_custom_certificate_verifier(verifier)
        .with_no_client_auth();
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    Ok(Arc::new(cfg))
}

fn map_io_error(err: &io::Error) -> u32 {
    if let Some(tls_err) = err.get_ref().and_then(|e| e.downcast_ref::<rustls::Error>()) {
        return match tls_err {
            rustls::Error::InvalidCertificate(cert_err) => match cert_err {
                CertificateError::Expired
                | CertificateError::NotValidYet
                | CertificateError::ExpiredContext { .. }
                | CertificateError::NotValidYetContext { .. } => {
                    ERROR_WINHTTP_SECURE_CERT_DATE_INVALID
                }
                CertificateError::NotValidForName
                | CertificateError::NotValidForNameContext { .. } => {
                    ERROR_WINHTTP_SECURE_CERT_CN_INVALID
                }
                CertificateError::UnknownIssuer | CertificateError::BadSignature => {
                    ERROR_WINHTTP_SECURE_INVALID_CA
                }
                _ => ERROR_WINHTTP_SECURE_CHANNEL_ERROR,
            },
            _ => ERROR_WINHTTP_SECURE_CHANNEL_ERROR,
        };
    }
    match err.kind() {
        io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => ERROR_WINHTTP_TIMEOUT,
        io::ErrorKind::ConnectionRefused => ERROR_WINHTTP_CANNOT_CONNECT,
        _ => ERROR_WINHTTP_CONNECTION_ERROR,
    }
}

// ---------------------------------------------------------------------------
// HTTP/1.1 + HTTPS transport & wire framing
// ---------------------------------------------------------------------------

enum HttpStream {
    Plain(TcpStream),
    Tls(Box<StreamOwned<ClientConnection, TcpStream>>),
}

impl HttpStream {
    fn is_tls(&self) -> bool {
        matches!(self, HttpStream::Tls(_))
    }
}

impl Read for HttpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            HttpStream::Plain(s) => s.read(buf),
            HttpStream::Tls(s) => s.read(buf),
        }
    }
}

impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            HttpStream::Plain(s) => s.write(buf),
            HttpStream::Tls(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            HttpStream::Plain(s) => s.flush(),
            HttpStream::Tls(s) => s.flush(),
        }
    }
}

fn open_http_stream(
    host: &str,
    port: u16,
    is_secure: bool,
    timeouts: Timeouts,
    security_flags: u32,
) -> Result<HttpStream, u32> {
    let clean_host = host.trim_start_matches('[').trim_end_matches(']');
    let addrs: Vec<SocketAddr> = (clean_host, port)
        .to_socket_addrs()
        .map_err(|_| ERROR_WINHTTP_NAME_NOT_RESOLVED)?
        .collect();
    if addrs.is_empty() {
        return Err(ERROR_WINHTTP_NAME_NOT_RESOLVED);
    }

    let connect_timeout = if timeouts.connect_ms > 0 {
        Some(Duration::from_millis(timeouts.connect_ms as u64))
    } else {
        None
    };

    let mut connected: Option<TcpStream> = None;
    let mut last_err: Option<io::Error> = None;
    for addr in addrs {
        let attempt = match connect_timeout {
            Some(dur) => TcpStream::connect_timeout(&addr, dur),
            None => TcpStream::connect(addr),
        };
        match attempt {
            Ok(s) => {
                connected = Some(s);
                break;
            }
            Err(e) => last_err = Some(e),
        }
    }

    let tcp = match connected {
        Some(s) => s,
        None => {
            if let Some(ref e) = last_err {
                if e.kind() == io::ErrorKind::TimedOut {
                    return Err(ERROR_WINHTTP_TIMEOUT);
                }
            }
            return Err(ERROR_WINHTTP_CANNOT_CONNECT);
        }
    };

    let _ = tcp.set_nodelay(true);
    if timeouts.send_ms > 0 {
        let _ = tcp.set_write_timeout(Some(Duration::from_millis(timeouts.send_ms as u64)));
    }
    if timeouts.receive_ms > 0 {
        let _ = tcp.set_read_timeout(Some(Duration::from_millis(timeouts.receive_ms as u64)));
    }

    if !is_secure {
        return Ok(HttpStream::Plain(tcp));
    }

    let tls_cfg = tls_config_for(security_flags)?;
    let server_name =
        ServerName::try_from(clean_host.to_string()).map_err(|_| ERROR_WINHTTP_INVALID_URL)?;
    let conn =
        ClientConnection::new(tls_cfg, server_name).map_err(|_| ERROR_WINHTTP_SECURE_CHANNEL_ERROR)?;
    Ok(HttpStream::Tls(Box::new(StreamOwned::new(conn, tcp))))
}

struct BufferedReader {
    stream: HttpStream,
    buf: Vec<u8>,
    pos: usize,
}

impl BufferedReader {
    fn new(stream: HttpStream) -> Self {
        Self {
            stream,
            buf: Vec::with_capacity(8192),
            pos: 0,
        }
    }

    fn buffered(&self) -> &[u8] {
        &self.buf[self.pos..]
    }

    fn consume(&mut self, n: usize) {
        self.pos += n;
        if self.pos >= self.buf.len() {
            self.buf.clear();
            self.pos = 0;
        } else if self.pos >= 8192 {
            self.buf.drain(..self.pos);
            self.pos = 0;
        }
    }

    /// Read more bytes from the underlying stream. Returns `Ok(0)` on clean EOF.
    fn fill(&mut self, allow_tls_unclean_eof: bool) -> Result<usize, u32> {
        let mut tmp = [0u8; 8192];
        loop {
            match self.stream.read(&mut tmp) {
                Ok(n) => {
                    if n > 0 {
                        self.buf.extend_from_slice(&tmp[..n]);
                    }
                    return Ok(n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(ref e)
                    if allow_tls_unclean_eof
                        && self.stream.is_tls()
                        && e.kind() == io::ErrorKind::UnexpectedEof =>
                {
                    return Ok(0);
                }
                Err(ref e) => return Err(map_io_error(e)),
            }
        }
    }

    fn read_header_block(&mut self) -> Result<Vec<u8>, u32> {
        loop {
            let slice = self.buffered();
            if let Some(idx) = find_header_end(slice) {
                let (end, sep_len) = idx;
                let block = slice[..end].to_vec();
                self.consume(end + sep_len);
                return Ok(block);
            }
            if slice.len() > MAX_HEADER_BYTES {
                return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
            }
            if self.fill(false)? == 0 {
                return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
            }
        }
    }

    fn read_line(&mut self) -> Result<Option<String>, u32> {
        loop {
            let slice = self.buffered();
            if let Some(nl) = slice.iter().position(|&b| b == b'\n') {
                let raw = &slice[..nl];
                let line = raw.strip_suffix(b"\r").unwrap_or(raw);
                let s = String::from_utf8_lossy(line).into_owned();
                self.consume(nl + 1);
                return Ok(Some(s));
            }
            if slice.len() > MAX_HEADER_BYTES {
                return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
            }
            if self.fill(true)? == 0 {
                if self.buffered().is_empty() {
                    return Ok(None);
                }
                let s = String::from_utf8_lossy(self.buffered()).into_owned();
                self.buf.clear();
                self.pos = 0;
                return Ok(Some(s));
            }
        }
    }

    fn read_exact_bytes(&mut self, len: usize, out: &mut Vec<u8>) -> Result<(), u32> {
        let mut remaining = len;
        while remaining > 0 {
            let avail = self.buffered().len();
            if avail > 0 {
                let take = avail.min(remaining);
                out.extend_from_slice(&self.buffered()[..take]);
                self.consume(take);
                remaining -= take;
            } else if self.fill(false)? == 0 {
                return Err(ERROR_WINHTTP_CONNECTION_ERROR);
            }
        }
        Ok(())
    }

    fn read_to_eof(&mut self, out: &mut Vec<u8>) -> Result<(), u32> {
        if !self.buffered().is_empty() {
            out.extend_from_slice(self.buffered());
            self.buf.clear();
            self.pos = 0;
        }
        while self.fill(true)? > 0 {
            out.extend_from_slice(self.buffered());
            self.buf.clear();
            self.pos = 0;
        }
        Ok(())
    }
}

/// Find `\r\n\r\n` or `\n\n` in `buf`. Returns `(header_len, separator_len)`.
fn find_header_end(buf: &[u8]) -> Option<(usize, usize)> {
    for i in 0..buf.len() {
        if i + 4 <= buf.len() && &buf[i..i + 4] == b"\r\n\r\n" {
            return Some((i, 4));
        }
        if i + 2 <= buf.len() && &buf[i..i + 2] == b"\n\n" {
            return Some((i, 2));
        }
    }
    None
}

fn parse_response_headers(block: &[u8]) -> Result<(String, u16, String, Vec<(String, String)>), u32> {
    let text = String::from_utf8_lossy(block);
    let mut lines = text.split('\n');
    let first_line = lines
        .next()
        .ok_or(ERROR_WINHTTP_INVALID_SERVER_RESPONSE)?
        .trim_end_matches('\r');

    // Status-Line: HTTP-Version SP Status-Code [SP Reason-Phrase]
    let mut sp = first_line.splitn(3, ' ');
    let version = sp.next().unwrap_or("").trim();
    let code_str = sp.next().unwrap_or("").trim();
    let reason = sp.next().unwrap_or("").trim();
    if !version.starts_with("HTTP/") {
        return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
    }
    let status_code: u16 = code_str
        .parse()
        .map_err(|_| ERROR_WINHTTP_INVALID_SERVER_RESPONSE)?;
    if !(100..=599).contains(&status_code) {
        return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
    }

    let mut headers: Vec<(String, String)> = Vec::new();
    for raw_line in lines {
        let line = raw_line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        if line.starts_with(' ') || line.starts_with('\t') {
            if let Some(last) = headers.last_mut() {
                last.1.push(' ');
                last.1.push_str(line.trim());
            }
            continue;
        }
        if let Some((name, val)) = line.split_once(':') {
            let n = name.trim();
            if !n.is_empty() {
                headers.push((n.to_string(), val.trim().to_string()));
            }
        }
    }
    Ok((version.to_string(), status_code, reason.to_string(), headers))
}

fn is_chunked_encoding(headers: &[(String, String)]) -> bool {
    for (k, v) in headers {
        if k.eq_ignore_ascii_case("Transfer-Encoding") {
            for token in v.split(',') {
                if token.trim().eq_ignore_ascii_case("chunked") {
                    return true;
                }
            }
        }
    }
    false
}

fn parse_content_length(headers: &[(String, String)]) -> Option<usize> {
    for (k, v) in headers {
        if k.eq_ignore_ascii_case("Content-Length") {
            if let Ok(n) = v.trim().parse::<usize>() {
                return Some(n);
            }
        }
    }
    None
}

fn read_chunked_body(reader: &mut BufferedReader) -> Result<Vec<u8>, u32> {
    let mut body = Vec::new();
    loop {
        let Some(size_line) = reader.read_line()? else {
            return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
        };
        let size_hex = size_line.split(';').next().unwrap_or("").trim();
        if size_hex.is_empty() {
            return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
        }
        let chunk_size = usize::from_str_radix(size_hex, 16)
            .map_err(|_| ERROR_WINHTTP_INVALID_SERVER_RESPONSE)?;
        if chunk_size > MAX_CHUNK_SIZE {
            return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
        }
        if chunk_size == 0 {
            // Drain optional trailer headers until empty line or EOF.
            loop {
                match reader.read_line()? {
                    Some(l) if l.is_empty() => break,
                    Some(_) => continue,
                    None => break,
                }
            }
            break;
        }
        reader.read_exact_bytes(chunk_size, &mut body)?;
        // Consume CRLF (or LF) terminating the chunk payload.
        let _ = reader.read_line()?;
    }
    Ok(body)
}

fn sanitize_header_token(s: &str) -> String {
    s.replace(['\r', '\n'], " ")
}

struct RequestSnapshot {
    host: String,
    port: u16,
    is_secure: bool,
    verb: String,
    path: String,
    version: String,
    user_agent: String,
    headers: Vec<(String, String)>,
    body: Vec<u8>,
    timeouts: Timeouts,
    redirect_policy: u32,
    max_redirects: u32,
    disable_flags: u32,
    security_flags: u32,
}

fn send_single_http_request(
    host: &str,
    port: u16,
    is_secure: bool,
    verb: &str,
    path: &str,
    version: &str,
    user_agent: &str,
    headers: &[(String, String)],
    body: &[u8],
    timeouts: Timeouts,
    security_flags: u32,
) -> Result<HttpResponse, u32> {
    let mut stream = open_http_stream(host, port, is_secure, timeouts, security_flags)?;

    let req_path = if path.is_empty() {
        "/".to_string()
    } else if path.starts_with('/') || path == "*" {
        path.to_string()
    } else {
        format!("/{path}")
    };
    let req_ver = if version.is_empty() { "HTTP/1.1" } else { version };

    let mut wire = Vec::with_capacity(512 + body.len());
    let _ = write!(wire, "{verb} {req_path} {req_ver}\r\n");

    if !has_header(headers, "Host") {
        let default_port = if is_secure {
            INTERNET_DEFAULT_HTTPS_PORT
        } else {
            INTERNET_DEFAULT_HTTP_PORT
        };
        let host_hdr = if host.contains(':') && !host.starts_with('[') {
            format!("[{host}]")
        } else {
            host.to_string()
        };
        if port == default_port || port == 0 {
            let _ = write!(wire, "Host: {host_hdr}\r\n");
        } else {
            let _ = write!(wire, "Host: {host_hdr}:{port}\r\n");
        }
    }

    if !has_header(headers, "User-Agent") && !user_agent.is_empty() {
        let _ = write!(wire, "User-Agent: {}\r\n", sanitize_header_token(user_agent));
    }

    if !has_header(headers, "Connection") {
        let _ = write!(wire, "Connection: close\r\n");
    }

    if !has_header(headers, "Content-Length")
        && !has_header(headers, "Transfer-Encoding")
        && (!body.is_empty()
            || verb.eq_ignore_ascii_case("POST")
            || verb.eq_ignore_ascii_case("PUT")
            || verb.eq_ignore_ascii_case("PATCH"))
    {
        let _ = write!(wire, "Content-Length: {}\r\n", body.len());
    }

    for (k, v) in headers {
        let _ = write!(
            wire,
            "{}: {}\r\n",
            sanitize_header_token(k),
            sanitize_header_token(v)
        );
    }
    wire.extend_from_slice(b"\r\n");
    wire.extend_from_slice(body);

    stream.write_all(&wire).map_err(|e| map_io_error(&e))?;
    stream.flush().map_err(|e| map_io_error(&e))?;

    let mut reader = BufferedReader::new(stream);
    let (resp_ver, status_code, status_text, resp_headers) = loop {
        let block = reader.read_header_block()?;
        let parsed = parse_response_headers(&block)?;
        if (100..200).contains(&parsed.1) && parsed.1 != 101 {
            // Skip 1xx interim responses (e.g., 100 Continue).
            continue;
        }
        break parsed;
    };

    let no_body = verb.eq_ignore_ascii_case("HEAD")
        || status_code == 204
        || status_code == 304
        || (100..200).contains(&status_code);

    let resp_body = if no_body {
        Vec::new()
    } else if is_chunked_encoding(&resp_headers) {
        read_chunked_body(&mut reader)?
    } else if let Some(cl) = parse_content_length(&resp_headers) {
        let mut b = Vec::with_capacity(cl.min(64 * 1024));
        reader.read_exact_bytes(cl, &mut b)?;
        b
    } else {
        let mut b = Vec::new();
        reader.read_to_eof(&mut b)?;
        b
    };

    Ok(HttpResponse {
        version: resp_ver,
        status_code,
        status_text,
        headers: resp_headers,
        body: resp_body,
        read_pos: 0,
    })
}

fn execute_http_request(snap: RequestSnapshot) -> Result<(HttpResponse, String), u32> {
    if is_offline_mode() {
        log::debug!("WinHttpReceiveResponse: blocked by PORT_OFFLINE=1");
        return Err(ERROR_WINHTTP_CANNOT_CONNECT);
    }

    let mut host = snap.host;
    let mut port = snap.port;
    let mut is_secure = snap.is_secure;
    let mut verb = snap.verb;
    let mut path = snap.path;
    let mut headers = snap.headers;
    let mut body = snap.body;

    let allow_redirects = (snap.disable_flags & WINHTTP_DISABLE_REDIRECTS) == 0
        && snap.redirect_policy != WINHTTP_OPTION_REDIRECT_POLICY_NEVER;
    let max_hops = snap.max_redirects;
    let mut hops: u32 = 0;

    loop {
        let resp = send_single_http_request(
            &host,
            port,
            is_secure,
            &verb,
            &path,
            &snap.version,
            &snap.user_agent,
            &headers,
            &body,
            snap.timeouts,
            snap.security_flags,
        )?;

        let is_redirect = matches!(resp.status_code, 301 | 302 | 303 | 307 | 308);
        if !is_redirect || !allow_redirects {
            let final_url = format_url(is_secure, &host, port, &path);
            return Ok((resp, final_url));
        }

        let Some(location) = find_header(&resp.headers, "Location") else {
            let final_url = format_url(is_secure, &host, port, &path);
            return Ok((resp, final_url));
        };

        if hops >= max_hops {
            return Err(ERROR_WINHTTP_REDIRECT_FAILED);
        }

        let (next_secure, next_host, next_port, next_path) =
            resolve_redirect_target(is_secure, &host, port, &path, location)?;

        if is_secure
            && !next_secure
            && snap.redirect_policy == WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP
        {
            // HTTPS -> HTTP downgrade disallowed by policy: return the 3xx response.
            let final_url = format_url(is_secure, &host, port, &path);
            return Ok((resp, final_url));
        }

        let origin_changed =
            is_secure != next_secure || !host.eq_ignore_ascii_case(&next_host) || port != next_port;
        if origin_changed {
            headers.retain(|(k, _)| {
                !k.eq_ignore_ascii_case("Authorization") && !k.eq_ignore_ascii_case("Host")
            });
        }

        let downgrade_to_get = resp.status_code == 303
            || ((resp.status_code == 301 || resp.status_code == 302)
                && verb.eq_ignore_ascii_case("POST"));
        if downgrade_to_get {
            verb = "GET".to_string();
            body.clear();
            headers.retain(|(k, _)| {
                !k.eq_ignore_ascii_case("Content-Length")
                    && !k.eq_ignore_ascii_case("Content-Type")
                    && !k.eq_ignore_ascii_case("Transfer-Encoding")
            });
        }

        is_secure = next_secure;
        host = next_host;
        port = next_port;
        path = next_path;
        hops += 1;
    }
}

fn resolve_redirect_target(
    cur_secure: bool,
    cur_host: &str,
    cur_port: u16,
    cur_path: &str,
    location: &str,
) -> Result<(bool, String, u16, String), u32> {
    let loc = location.trim();
    if loc.is_empty() {
        return Err(ERROR_WINHTTP_INVALID_URL);
    }

    if loc.contains("://") {
        let parsed = parse_url_parts(loc)?;
        let scheme_str = &loc[parsed.scheme_start..parsed.scheme_end];
        let secure = scheme_str.eq_ignore_ascii_case("https");
        let host = loc[parsed.host_start..parsed.host_end].to_string();
        let port = parsed.port;
        let path_part = &loc[parsed.path_start..parsed.path_end];
        let extra_part = &loc[parsed.extra_start..parsed.extra_end];
        let mut full_path = if path_part.is_empty() {
            "/".to_string()
        } else {
            path_part.to_string()
        };
        full_path.push_str(extra_part);
        return Ok((secure, host, port, full_path));
    }

    if let Some(rest) = loc.strip_prefix("//") {
        let synthetic = format!(
            "{}://{rest}",
            if cur_secure { "https" } else { "http" }
        );
        return resolve_redirect_target(cur_secure, cur_host, cur_port, cur_path, &synthetic);
    }

    if loc.starts_with('/') {
        return Ok((cur_secure, cur_host.to_string(), cur_port, loc.to_string()));
    }

    if loc.starts_with('?') {
        let base_path = cur_path.split(['?', '#']).next().unwrap_or("/");
        let p = if base_path.is_empty() { "/" } else { base_path };
        return Ok((cur_secure, cur_host.to_string(), cur_port, format!("{p}{loc}")));
    }

    let base_no_query = cur_path.split(['?', '#']).next().unwrap_or("/");
    let dir = match base_no_query.rfind('/') {
        Some(idx) => &base_no_query[..=idx],
        None => "/",
    };
    Ok((cur_secure, cur_host.to_string(), cur_port, format!("{dir}{loc}")))
}

// ---------------------------------------------------------------------------
// URL parsing & formatting (`WinHttpCrackUrl`, `WinHttpCreateUrl`, `InternetCrackUrl*`)
// ---------------------------------------------------------------------------

struct ParsedUrl {
    scheme_start: usize,
    scheme_end: usize,
    is_https: bool,
    user_start: usize,
    user_end: usize,
    pass_start: usize,
    pass_end: usize,
    host_start: usize,
    host_end: usize,
    port: u16,
    path_start: usize,
    path_end: usize,
    extra_start: usize,
    extra_end: usize,
}

fn parse_url_parts(url: &str) -> Result<ParsedUrl, u32> {
    let Some(scheme_sep) = url.find("://") else {
        return Err(ERROR_WINHTTP_INVALID_URL);
    };
    let scheme = &url[..scheme_sep];
    let is_https = if scheme.eq_ignore_ascii_case("http") {
        false
    } else if scheme.eq_ignore_ascii_case("https") {
        true
    } else {
        return Err(ERROR_WINHTTP_UNRECOGNIZED_SCHEME);
    };

    let after_scheme = scheme_sep + 3;
    let rest = &url[after_scheme..];

    // Authority ends at first '/', '?', or '#'
    let auth_rel_end = rest
        .find(['/', '?', '#'])
        .unwrap_or(rest.len());
    let auth_start = after_scheme;
    let auth_end = after_scheme + auth_rel_end;
    let authority = &url[auth_start..auth_end];

    let (user_start, user_end, pass_start, pass_end, hostport_start) =
        if let Some(at_rel) = authority.rfind('@') {
            let userinfo = &authority[..at_rel];
            if let Some(colon_rel) = userinfo.find(':') {
                (
                    auth_start,
                    auth_start + colon_rel,
                    auth_start + colon_rel + 1,
                    auth_start + at_rel,
                    auth_start + at_rel + 1,
                )
            } else {
                (
                    auth_start,
                    auth_start + at_rel,
                    auth_start + at_rel,
                    auth_start + at_rel,
                    auth_start + at_rel + 1,
                )
            }
        } else {
            (auth_start, auth_start, auth_start, auth_start, auth_start)
        };

    let hostport = &url[hostport_start..auth_end];
    if hostport.is_empty() {
        return Err(ERROR_WINHTTP_INVALID_URL);
    }

    let (host_start, host_end, port) = if hostport.starts_with('[') {
        let Some(close_rel) = hostport.find(']') else {
            return Err(ERROR_WINHTTP_INVALID_URL);
        };
        let h_start = hostport_start;
        let h_end = hostport_start + close_rel + 1;
        let after_bracket = &hostport[close_rel + 1..];
        let port = if after_bracket.is_empty() {
            if is_https {
                INTERNET_DEFAULT_HTTPS_PORT
            } else {
                INTERNET_DEFAULT_HTTP_PORT
            }
        } else if let Some(port_str) = after_bracket.strip_prefix(':') {
            port_str.parse::<u16>().map_err(|_| ERROR_WINHTTP_INVALID_URL)?
        } else {
            return Err(ERROR_WINHTTP_INVALID_URL);
        };
        (h_start, h_end, port)
    } else if let Some(colon_rel) = hostport.rfind(':') {
        let h_start = hostport_start;
        let h_end = hostport_start + colon_rel;
        if h_start == h_end {
            return Err(ERROR_WINHTTP_INVALID_URL);
        }
        let port_str = &hostport[colon_rel + 1..];
        let port = if port_str.is_empty() {
            if is_https {
                INTERNET_DEFAULT_HTTPS_PORT
            } else {
                INTERNET_DEFAULT_HTTP_PORT
            }
        } else {
            port_str.parse::<u16>().map_err(|_| ERROR_WINHTTP_INVALID_URL)?
        };
        (h_start, h_end, port)
    } else {
        let port = if is_https {
            INTERNET_DEFAULT_HTTPS_PORT
        } else {
            INTERNET_DEFAULT_HTTP_PORT
        };
        (hostport_start, auth_end, port)
    };

    let path_and_extra = &url[auth_end..];
    let (path_start, path_end, extra_start, extra_end) =
        if let Some(q_rel) = path_and_extra.find(['?', '#']) {
            (auth_end, auth_end + q_rel, auth_end + q_rel, url.len())
        } else {
            (auth_end, url.len(), url.len(), url.len())
        };

    Ok(ParsedUrl {
        scheme_start: 0,
        scheme_end: scheme_sep,
        is_https,
        user_start,
        user_end,
        pass_start,
        pass_end,
        host_start,
        host_end,
        port,
        path_start,
        path_end,
        extra_start,
        extra_end,
    })
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                out.push((h << 4) | l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn percent_escape_url(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b <= 0x20 || b >= 0x7F || matches!(b, b'<' | b'>' | b'"' | b'{' | b'}' | b'|' | b'\\' | b'^' | b'`') {
            out.push('%');
            let hi = b >> 4;
            let lo = b & 0x0F;
            out.push(char::from(if hi < 10 { b'0' + hi } else { b'A' + hi - 10 }));
            out.push(char::from(if lo < 10 { b'0' + lo } else { b'A' + lo - 10 }));
        } else {
            out.push(b as char);
        }
    }
    out
}

unsafe fn fill_url_component_w(
    url_ptr: LPCWSTR,
    url_str: &str,
    byte_start: usize,
    byte_end: usize,
    decode: bool,
    out_ptr: &mut LPWSTR,
    out_len: &mut u32,
) -> Result<(), u32> {
    if out_ptr.is_null() && *out_len == 0 {
        return Ok(());
    }
    let slice = &url_str[byte_start..byte_end];
    if out_ptr.is_null() {
        // In-place pointer mode into caller's UTF-16 URL buffer.
        let utf16_off = url_str[..byte_start].encode_utf16().count();
        let utf16_len = slice.encode_utf16().count();
        *out_ptr = if utf16_len == 0 {
            std::ptr::null_mut()
        } else {
            url_ptr.add(utf16_off) as LPWSTR
        };
        *out_len = utf16_len as u32;
        return Ok(());
    }
    let val = if decode {
        percent_decode(slice)
    } else {
        slice.to_string()
    };
    let units: Vec<u16> = val.encode_utf16().collect();
    let cap = *out_len as usize;
    if cap <= units.len() {
        *out_len = (units.len() + 1) as u32;
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    write_wide_units(*out_ptr, &units);
    std::ptr::write_unaligned((*out_ptr).add(units.len()), 0u16);
    *out_len = units.len() as u32;
    Ok(())
}

unsafe fn fill_url_component_a(
    url_ptr: LPCSTR,
    url_str: &str,
    byte_start: usize,
    byte_end: usize,
    decode: bool,
    out_ptr: &mut LPSTR,
    out_len: &mut u32,
) -> Result<(), u32> {
    if out_ptr.is_null() && *out_len == 0 {
        return Ok(());
    }
    let slice = &url_str[byte_start..byte_end];
    if out_ptr.is_null() {
        let len = byte_end - byte_start;
        *out_ptr = if len == 0 {
            std::ptr::null_mut()
        } else {
            url_ptr.add(byte_start) as LPSTR
        };
        *out_len = len as u32;
        return Ok(());
    }
    let val = if decode {
        percent_decode(slice)
    } else {
        slice.to_string()
    };
    let bytes = val.as_bytes();
    let cap = *out_len as usize;
    if cap <= bytes.len() {
        *out_len = (bytes.len() + 1) as u32;
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), *out_ptr, bytes.len());
    }
    std::ptr::write_unaligned((*out_ptr).add(bytes.len()), 0u8);
    *out_len = bytes.len() as u32;
    Ok(())
}

unsafe fn crack_url_w_impl(
    url: LPCWSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSW,
    wininet_schemes: bool,
) -> BOOL {
    if url.is_null() || components.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut comp = std::ptr::read_unaligned(components);
    if (comp.dw_struct_size as usize) < std::mem::size_of::<URL_COMPONENTSW>() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(url_str) = read_wide_len_zero(url, url_len) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let parsed = match parse_url_parts(&url_str) {
        Ok(p) => p,
        Err(e) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    };

    let decode = (flags & ICU_DECODE) != 0;
    let extra_requested = !comp.lpsz_extra_info.is_null() || comp.dw_extra_info_length != 0;
    let path_end = if extra_requested {
        parsed.path_end
    } else {
        parsed.extra_end
    };

    comp.n_scheme = match (parsed.is_https, wininet_schemes) {
        (false, false) => WINHTTP_SCHEME_HTTP,
        (true, false) => WINHTTP_SCHEME_HTTPS,
        (false, true) => WININET_SCHEME_HTTP,
        (true, true) => WININET_SCHEME_HTTPS,
    };
    comp.n_port = parsed.port;

    let mut first_err: Option<u32> = None;
    let mut record = |res: Result<(), u32>| {
        if let Err(e) = res {
            if first_err.is_none() {
                first_err = Some(e);
            }
        }
    };

    record(fill_url_component_w(
        url,
        &url_str,
        parsed.scheme_start,
        parsed.scheme_end,
        false,
        &mut comp.lpsz_scheme,
        &mut comp.dw_scheme_length,
    ));
    record(fill_url_component_w(
        url,
        &url_str,
        parsed.host_start,
        parsed.host_end,
        false,
        &mut comp.lpsz_host_name,
        &mut comp.dw_host_name_length,
    ));
    record(fill_url_component_w(
        url,
        &url_str,
        parsed.user_start,
        parsed.user_end,
        decode,
        &mut comp.lpsz_user_name,
        &mut comp.dw_user_name_length,
    ));
    record(fill_url_component_w(
        url,
        &url_str,
        parsed.pass_start,
        parsed.pass_end,
        decode,
        &mut comp.lpsz_password,
        &mut comp.dw_password_length,
    ));
    record(fill_url_component_w(
        url,
        &url_str,
        parsed.path_start,
        path_end,
        decode,
        &mut comp.lpsz_url_path,
        &mut comp.dw_url_path_length,
    ));
    record(fill_url_component_w(
        url,
        &url_str,
        parsed.extra_start,
        parsed.extra_end,
        decode,
        &mut comp.lpsz_extra_info,
        &mut comp.dw_extra_info_length,
    ));

    std::ptr::write_unaligned(components, comp);
    if let Some(e) = first_err {
        crate::teb::set_last_error(e);
        return FALSE;
    }
    TRUE
}

// ---------------------------------------------------------------------------
// HTTP-date parser (IMF-fixdate -> SYSTEMTIME) & Base64 encoder (Basic auth)
// ---------------------------------------------------------------------------

fn parse_http_date(s: &str) -> Option<SYSTEMTIME> {
    // Example: "Sun, 06 Nov 1994 08:49:37 GMT"
    let s = s.trim();
    let (dow_str, rest) = s.split_once(',')?;
    let day_of_week: u16 = match dow_str.trim().to_ascii_lowercase().as_str() {
        "sun" | "sunday" => 0,
        "mon" | "monday" => 1,
        "tue" | "tuesday" => 2,
        "wed" | "wednesday" => 3,
        "thu" | "thursday" => 4,
        "fri" | "friday" => 5,
        "sat" | "saturday" => 6,
        _ => return None,
    };
    let mut parts = rest.split_whitespace();
    let day: u16 = parts.next()?.parse().ok()?;
    let month: u16 = match parts.next()?.to_ascii_lowercase().as_str() {
        "jan" => 1,
        "feb" => 2,
        "mar" => 3,
        "apr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "aug" => 8,
        "sep" => 9,
        "oct" => 10,
        "nov" => 11,
        "dec" => 12,
        _ => return None,
    };
    let year: u16 = parts.next()?.parse().ok()?;
    let time_str = parts.next()?;
    let mut tparts = time_str.split(':');
    let hour: u16 = tparts.next()?.parse().ok()?;
    let minute: u16 = tparts.next()?.parse().ok()?;
    let second: u16 = tparts.next()?.parse().ok()?;
    Some(SYSTEMTIME {
        wYear: year,
        wMonth: month,
        wDayOfWeek: day_of_week,
        wDay: day,
        wHour: hour,
        wMinute: minute,
        wSecond: second,
        wMilliseconds: 0,
    })
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Core internal operations shared by WinHTTP and WinINet
// ---------------------------------------------------------------------------

fn open_session_impl(user_agent: Option<String>) -> usize {
    let ua = user_agent
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "WinHTTP/1.1".to_string());
    alloc_handle(WinHttpHandle::Session(SessionState {
        user_agent: ua,
        timeouts: Timeouts::default(),
        redirect_policy: WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP,
        max_redirects: DEFAULT_MAX_REDIRECTS,
        disable_flags: 0,
        security_flags: 0,
        context: 0,
        status_callback: 0,
    }))
}

fn connect_impl(session: usize, host: String, server_port: u16, context: usize) -> usize {
    if host.is_empty() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let parent = {
        let map = HANDLES.lock().unwrap();
        match map.get(&session) {
            Some(WinHttpHandle::Session(s)) => s.clone(),
            Some(_) => {
                crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
                return 0;
            }
            None => {
                crate::teb::set_last_error(ERROR_INVALID_HANDLE);
                return 0;
            }
        }
    };
    alloc_handle(WinHttpHandle::Connection(ConnectionState {
        parent_session: session,
        host,
        port: server_port,
        user_agent: parent.user_agent,
        timeouts: parent.timeouts,
        redirect_policy: parent.redirect_policy,
        max_redirects: parent.max_redirects,
        disable_flags: parent.disable_flags,
        security_flags: parent.security_flags,
        context,
        status_callback: parent.status_callback,
    }))
}

fn open_request_impl(
    connect: usize,
    verb: Option<String>,
    object_name: Option<String>,
    version: Option<String>,
    referrer: Option<String>,
    accept_types: Vec<String>,
    flags: u32,
    extra_disable_flags: u32,
    extra_security_flags: u32,
    context: usize,
) -> usize {
    let conn = {
        let map = HANDLES.lock().unwrap();
        match map.get(&connect) {
            Some(WinHttpHandle::Connection(c)) => c.clone(),
            Some(_) => {
                crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
                return 0;
            }
            None => {
                crate::teb::set_last_error(ERROR_INVALID_HANDLE);
                return 0;
            }
        }
    };

    let flag_secure = (flags & WINHTTP_FLAG_SECURE) != 0;
    let resolved_port = if conn.port != 0 {
        conn.port
    } else if flag_secure {
        INTERNET_DEFAULT_HTTPS_PORT
    } else {
        INTERNET_DEFAULT_HTTP_PORT
    };
    let is_secure = flag_secure || resolved_port == INTERNET_DEFAULT_HTTPS_PORT;

    let verb = verb
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "GET".to_string());
    let raw_path = object_name.unwrap_or_default();
    let path = if raw_path.is_empty() {
        "/".to_string()
    } else if raw_path.starts_with('/') || raw_path == "*" {
        raw_path
    } else {
        format!("/{raw_path}")
    };
    let version = version
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "HTTP/1.1".to_string());

    let mut headers = Vec::new();
    if let Some(r) = referrer.filter(|s| !s.is_empty()) {
        headers.push(("Referer".to_string(), r));
    }
    if !accept_types.is_empty() {
        headers.push(("Accept".to_string(), accept_types.join(", ")));
    }

    let effective_url = format_url(is_secure, &conn.host, resolved_port, &path);
    alloc_handle(WinHttpHandle::Request(RequestState {
        parent_connect: connect,
        host: conn.host,
        port: resolved_port,
        is_secure,
        verb,
        path,
        version,
        user_agent: conn.user_agent,
        headers,
        send_headers: Vec::new(),
        request_body: Vec::new(),
        sent: false,
        timeouts: conn.timeouts,
        redirect_policy: conn.redirect_policy,
        max_redirects: conn.max_redirects,
        disable_flags: conn.disable_flags | extra_disable_flags,
        security_flags: conn.security_flags | extra_security_flags,
        context,
        status_callback: conn.status_callback,
        effective_url,
        response: None,
    }))
}

fn add_request_headers_impl(request: usize, raw: &str, modifiers: u32) -> BOOL {
    let mut map = HANDLES.lock().unwrap();
    let req = match map.get_mut(&request) {
        Some(WinHttpHandle::Request(r)) => r,
        Some(_) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };
    if let Err(e) = apply_header_block(&mut req.headers, raw, modifiers) {
        crate::teb::set_last_error(e);
        return FALSE;
    }
    TRUE
}

unsafe fn send_request_impl(
    request: usize,
    extra_headers: Option<String>,
    optional: *const u8,
    optional_len: u32,
    total_len: u32,
    context: usize,
) -> BOOL {
    if optional_len > 0 && optional.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut map = HANDLES.lock().unwrap();
    let req = match map.get_mut(&request) {
        Some(WinHttpHandle::Request(r)) => r,
        Some(_) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };

    req.send_headers.clear();
    if let Some(hdrs) = extra_headers.filter(|s| !s.is_empty()) {
        if let Err(e) = apply_header_block(&mut req.send_headers, &hdrs, WINHTTP_ADDREQ_FLAG_ADD) {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    }

    req.request_body.clear();
    let reserve = (total_len as usize).max(optional_len as usize).min(1024 * 1024);
    req.request_body.reserve(reserve);
    if optional_len > 0 {
        let slice = std::slice::from_raw_parts(optional, optional_len as usize);
        req.request_body.extend_from_slice(slice);
    }
    if context != 0 {
        req.context = context;
    }
    req.response = None;
    req.sent = true;
    TRUE
}

fn receive_response_impl(request: usize) -> BOOL {
    let snap = {
        let map = HANDLES.lock().unwrap();
        let req = match map.get(&request) {
            Some(WinHttpHandle::Request(r)) => r,
            Some(_) => {
                crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
                return FALSE;
            }
            None => {
                crate::teb::set_last_error(ERROR_INVALID_HANDLE);
                return FALSE;
            }
        };
        if !req.sent {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
            return FALSE;
        }
        let mut combined_headers = req.headers.clone();
        combined_headers.extend(req.send_headers.iter().cloned());
        RequestSnapshot {
            host: req.host.clone(),
            port: req.port,
            is_secure: req.is_secure,
            verb: req.verb.clone(),
            path: req.path.clone(),
            version: req.version.clone(),
            user_agent: req.user_agent.clone(),
            headers: combined_headers,
            body: req.request_body.clone(),
            timeouts: req.timeouts,
            redirect_policy: req.redirect_policy,
            max_redirects: req.max_redirects,
            disable_flags: req.disable_flags,
            security_flags: req.security_flags,
        }
    };

    // Perform network I/O without holding the global handle mutex.
    let (resp, final_url) = match execute_http_request(snap) {
        Ok(res) => res,
        Err(e) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    };

    let mut map = HANDLES.lock().unwrap();
    let Some(WinHttpHandle::Request(req)) = map.get_mut(&request) else {
        crate::teb::set_last_error(ERROR_WINHTTP_OPERATION_CANCELLED);
        return FALSE;
    };
    req.effective_url = final_url;
    req.response = Some(resp);
    TRUE
}

unsafe fn query_headers_impl(
    request: usize,
    info_level: u32,
    custom_name: Option<String>,
    buffer: *mut u8,
    buffer_len: *mut u32,
    index: *mut u32,
    ansi: bool,
) -> BOOL {
    if buffer_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let base = info_level & 0xFFFF;
    let flag_request = (info_level & WINHTTP_QUERY_FLAG_REQUEST_HEADERS) != 0;
    let flag_number = (info_level & WINHTTP_QUERY_FLAG_NUMBER) != 0;
    let flag_number64 = (info_level & WINHTTP_QUERY_FLAG_NUMBER64) != 0;
    let flag_systemtime = (info_level & WINHTTP_QUERY_FLAG_SYSTEMTIME) != 0;
    let flag_coalesce = (info_level & WINHTTP_QUERY_FLAG_COALESCE) != 0;

    let start_idx = if index.is_null() {
        0usize
    } else {
        read_u32(index) as usize
    };

    let map = HANDLES.lock().unwrap();
    let req = match map.get(&request) {
        Some(WinHttpHandle::Request(r)) => r,
        Some(_) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };

    if !flag_request && req.response.is_none() {
        crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        return FALSE;
    }

    // Special handling for WINHTTP_QUERY_RAW_HEADERS (double-NUL delimited lines).
    if base == WINHTTP_QUERY_RAW_HEADERS {
        let mut lines: Vec<String> = Vec::new();
        if flag_request {
            lines.push(format!("{} {} {}", req.verb, req.path, req.version));
            for (k, v) in req.headers.iter().chain(req.send_headers.iter()) {
                lines.push(format!("{k}: {v}"));
            }
        } else {
            let resp = req.response.as_ref().unwrap();
            if resp.status_text.is_empty() {
                lines.push(format!("{} {}", resp.version, resp.status_code));
            } else {
                lines.push(format!("{} {} {}", resp.version, resp.status_code, resp.status_text));
            }
            for (k, v) in &resp.headers {
                lines.push(format!("{k}: {v}"));
            }
        }
        if ansi {
            let mut bytes = Vec::new();
            for l in &lines {
                bytes.extend_from_slice(l.as_bytes());
                bytes.push(0);
            }
            bytes.push(0);
            let cap = read_u32(buffer_len) as usize;
            if buffer.is_null() || cap < bytes.len() {
                write_u32(buffer_len, bytes.len() as u32);
                crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, bytes.len());
            write_u32(buffer_len, (bytes.len() - 1) as u32);
            return TRUE;
        } else {
            let mut units = Vec::new();
            for l in &lines {
                units.extend(l.encode_utf16());
                units.push(0);
            }
            units.push(0);
            let needed_bytes = units.len() * 2;
            let cap_bytes = read_u32(buffer_len) as usize;
            if buffer.is_null() || cap_bytes < needed_bytes {
                write_u32(buffer_len, needed_bytes as u32);
                crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            write_wide_units(buffer as *mut u16, &units);
            write_u32(buffer_len, ((units.len() - 1) * 2) as u32);
            return TRUE;
        }
    }

    let mut advance_index = false;
    let value: String = match base {
        WINHTTP_QUERY_VERSION => {
            if flag_request {
                req.version.clone()
            } else {
                req.response.as_ref().unwrap().version.clone()
            }
        }
        WINHTTP_QUERY_STATUS_CODE => {
            if flag_request {
                crate::teb::set_last_error(ERROR_WINHTTP_INVALID_QUERY_REQUEST);
                return FALSE;
            }
            req.response.as_ref().unwrap().status_code.to_string()
        }
        WINHTTP_QUERY_STATUS_TEXT => {
            if flag_request {
                crate::teb::set_last_error(ERROR_WINHTTP_INVALID_QUERY_REQUEST);
                return FALSE;
            }
            req.response.as_ref().unwrap().status_text.clone()
        }
        WINHTTP_QUERY_REQUEST_METHOD => req.verb.clone(),
        WINHTTP_QUERY_RAW_HEADERS_CRLF => {
            let mut out = String::new();
            if flag_request {
                out.push_str(&format!("{} {} {}\r\n", req.verb, req.path, req.version));
                for (k, v) in req.headers.iter().chain(req.send_headers.iter()) {
                    out.push_str(&format!("{k}: {v}\r\n"));
                }
            } else {
                let resp = req.response.as_ref().unwrap();
                if resp.status_text.is_empty() {
                    out.push_str(&format!("{} {}\r\n", resp.version, resp.status_code));
                } else {
                    out.push_str(&format!(
                        "{} {} {}\r\n",
                        resp.version, resp.status_code, resp.status_text
                    ));
                }
                for (k, v) in &resp.headers {
                    out.push_str(&format!("{k}: {v}\r\n"));
                }
            }
            out.push_str("\r\n");
            out
        }
        _ => {
            let target_name: String = if base == WINHTTP_QUERY_CUSTOM {
                match custom_name.filter(|s| !s.trim().is_empty()) {
                    Some(n) => n.trim().to_string(),
                    None => {
                        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
                        return FALSE;
                    }
                }
            } else {
                match query_id_to_header_name(base) {
                    Some(n) => n.to_string(),
                    None => {
                        crate::teb::set_last_error(ERROR_WINHTTP_INVALID_QUERY_REQUEST);
                        return FALSE;
                    }
                }
            };

            let matches: Vec<&str> = if flag_request {
                req.headers
                    .iter()
                    .chain(req.send_headers.iter())
                    .filter(|(k, _)| k.eq_ignore_ascii_case(&target_name))
                    .map(|(_, v)| v.as_str())
                    .collect()
            } else {
                req.response
                    .as_ref()
                    .unwrap()
                    .headers
                    .iter()
                    .filter(|(k, _)| k.eq_ignore_ascii_case(&target_name))
                    .map(|(_, v)| v.as_str())
                    .collect()
            };

            if matches.is_empty() {
                crate::teb::set_last_error(ERROR_WINHTTP_HEADER_NOT_FOUND);
                return FALSE;
            }
            if flag_coalesce {
                if start_idx > 0 {
                    crate::teb::set_last_error(ERROR_WINHTTP_HEADER_NOT_FOUND);
                    return FALSE;
                }
                advance_index = true;
                matches.join(", ")
            } else if start_idx < matches.len() {
                advance_index = true;
                matches[start_idx].to_string()
            } else {
                crate::teb::set_last_error(ERROR_WINHTTP_HEADER_NOT_FOUND);
                return FALSE;
            }
        }
    };

    let ok = if flag_number {
        let Ok(num) = value.trim().parse::<u32>() else {
            crate::teb::set_last_error(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
            return FALSE;
        };
        let cap = read_u32(buffer_len) as usize;
        if buffer.is_null() || cap < 4 {
            write_u32(buffer_len, 4);
            crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        write_u32(buffer as *mut u32, num);
        write_u32(buffer_len, 4);
        TRUE
    } else if flag_number64 {
        let Ok(num) = value.trim().parse::<u64>() else {
            crate::teb::set_last_error(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
            return FALSE;
        };
        let cap = read_u32(buffer_len) as usize;
        if buffer.is_null() || cap < 8 {
            write_u32(buffer_len, 8);
            crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        std::ptr::write_unaligned(buffer as *mut u64, num);
        write_u32(buffer_len, 8);
        TRUE
    } else if flag_systemtime {
        let Some(st) = parse_http_date(&value) else {
            crate::teb::set_last_error(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
            return FALSE;
        };
        let needed = std::mem::size_of::<SYSTEMTIME>();
        let cap = read_u32(buffer_len) as usize;
        if buffer.is_null() || cap < needed {
            write_u32(buffer_len, needed as u32);
            crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        std::ptr::write_unaligned(buffer as *mut SYSTEMTIME, st);
        write_u32(buffer_len, needed as u32);
        TRUE
    } else if ansi {
        write_ansi_bytes_out(&value, buffer, buffer_len)
    } else {
        write_wide_bytes_out(&value, buffer, buffer_len)
    };

    if ok == TRUE && advance_index && !index.is_null() {
        write_u32(index, (start_idx + 1) as u32);
    }
    ok
}

unsafe fn set_option_impl(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
    ansi: bool,
) -> BOOL {
    if h_internet == 0 {
        // Process-global option (e.g. WINHTTP_OPTION_PROXY / SECURE_PROTOCOLS).
        log::debug!("WinHttpSetOption(global, opt={option}) -> TRUE");
        return TRUE;
    }

    let mut map = HANDLES.lock().unwrap();
    let Some(handle) = map.get_mut(&h_internet) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };

    let read_dword = || -> Result<u32, u32> {
        if buffer.is_null() || buffer_len < 4 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        Ok(read_u32(buffer as *const u32))
    };

    let read_ptr_val = || -> Result<usize, u32> {
        if buffer.is_null() || (buffer_len as usize) < std::mem::size_of::<usize>() {
            return Err(ERROR_INVALID_PARAMETER);
        }
        Ok(read_usize(buffer as *const usize))
    };

    let set_timeouts_field = |h: &mut WinHttpHandle, which: u32, val: i32| {
        let t = match h {
            WinHttpHandle::Session(s) => &mut s.timeouts,
            WinHttpHandle::Connection(c) => &mut c.timeouts,
            WinHttpHandle::Request(r) => &mut r.timeouts,
        };
        match which {
            WINHTTP_OPTION_RESOLVE_TIMEOUT => t.resolve_ms = val,
            WINHTTP_OPTION_CONNECT_TIMEOUT => t.connect_ms = val,
            WINHTTP_OPTION_SEND_TIMEOUT => t.send_ms = val,
            WINHTTP_OPTION_RECEIVE_TIMEOUT | WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT => {
                t.receive_ms = val
            }
            _ => {}
        }
    };

    match option {
        WINHTTP_OPTION_RESOLVE_TIMEOUT
        | WINHTTP_OPTION_CONNECT_TIMEOUT
        | WINHTTP_OPTION_SEND_TIMEOUT
        | WINHTTP_OPTION_RECEIVE_TIMEOUT
        | WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT => match read_dword() {
            Ok(v) => {
                set_timeouts_field(handle, option, v as i32);
                TRUE
            }
            Err(e) => {
                crate::teb::set_last_error(e);
                FALSE
            }
        },
        WINHTTP_OPTION_REDIRECT_POLICY => match read_dword() {
            Ok(v) if v <= WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS => {
                match handle {
                    WinHttpHandle::Session(s) => s.redirect_policy = v,
                    WinHttpHandle::Connection(c) => c.redirect_policy = v,
                    WinHttpHandle::Request(r) => r.redirect_policy = v,
                }
                TRUE
            }
            _ => {
                crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
                FALSE
            }
        },
        WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS => match read_dword() {
            Ok(v) => {
                match handle {
                    WinHttpHandle::Session(s) => s.max_redirects = v,
                    WinHttpHandle::Connection(c) => c.max_redirects = v,
                    WinHttpHandle::Request(r) => r.max_redirects = v,
                }
                TRUE
            }
            Err(e) => {
                crate::teb::set_last_error(e);
                FALSE
            }
        },
        WINHTTP_OPTION_DISABLE_FEATURE => match read_dword() {
            Ok(v) => {
                match handle {
                    WinHttpHandle::Session(s) => s.disable_flags |= v,
                    WinHttpHandle::Connection(c) => c.disable_flags |= v,
                    WinHttpHandle::Request(r) => r.disable_flags |= v,
                }
                TRUE
            }
            Err(e) => {
                crate::teb::set_last_error(e);
                FALSE
            }
        },
        WINHTTP_OPTION_SECURITY_FLAGS => match read_dword() {
            Ok(v) => {
                match handle {
                    WinHttpHandle::Session(s) => s.security_flags = v,
                    WinHttpHandle::Connection(c) => c.security_flags = v,
                    WinHttpHandle::Request(r) => r.security_flags = v,
                }
                TRUE
            }
            Err(e) => {
                crate::teb::set_last_error(e);
                FALSE
            }
        },
        WINHTTP_OPTION_CONTEXT_VALUE => match read_ptr_val() {
            Ok(v) => {
                match handle {
                    WinHttpHandle::Session(s) => s.context = v,
                    WinHttpHandle::Connection(c) => c.context = v,
                    WinHttpHandle::Request(r) => r.context = v,
                }
                TRUE
            }
            Err(e) => {
                crate::teb::set_last_error(e);
                FALSE
            }
        },
        WINHTTP_OPTION_CALLBACK => match read_ptr_val() {
            Ok(v) => {
                match handle {
                    WinHttpHandle::Session(s) => s.status_callback = v,
                    WinHttpHandle::Connection(c) => c.status_callback = v,
                    WinHttpHandle::Request(r) => r.status_callback = v,
                }
                TRUE
            }
            Err(e) => {
                crate::teb::set_last_error(e);
                FALSE
            }
        },
        WINHTTP_OPTION_USER_AGENT => {
            if buffer.is_null() {
                crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let ua = if ansi {
                read_ansi_len_neg1(buffer, buffer_len).unwrap_or_default()
            } else {
                let wlen = if buffer_len == u32::MAX {
                    u32::MAX
                } else {
                    buffer_len / 2
                };
                read_wide_len_neg1(buffer as LPCWSTR, wlen).unwrap_or_default()
            };
            match handle {
                WinHttpHandle::Session(s) => s.user_agent = ua,
                WinHttpHandle::Connection(c) => c.user_agent = ua,
                WinHttpHandle::Request(r) => r.user_agent = ua,
            }
            TRUE
        }
        WINHTTP_OPTION_CONNECT_RETRIES
        | WINHTTP_OPTION_READ_BUFFER_SIZE
        | WINHTTP_OPTION_WRITE_BUFFER_SIZE
        | WINHTTP_OPTION_PROXY
        | WINHTTP_OPTION_ENABLE_FEATURE
        | WINHTTP_OPTION_SECURE_PROTOCOLS
        | WINHTTP_OPTION_MAX_CONNS_PER_SERVER
        | WINHTTP_OPTION_DECOMPRESSION
        | WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL => {
            log::debug!("WinHttpSetOption(h={h_internet:#x}, opt={option}, len={buffer_len}) -> accepted");
            TRUE
        }
        _ => {
            log::debug!("WinHttpSetOption(h={h_internet:#x}, opt={option}, len={buffer_len}) -> ignored/TRUE");
            TRUE
        }
    }
}

unsafe fn query_option_impl(
    h_internet: usize,
    option: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
    ansi: bool,
) -> BOOL {
    if buffer_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let map = HANDLES.lock().unwrap();
    let Some(handle) = map.get(&h_internet) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };

    let write_dword_out = |val: u32| -> BOOL {
        let cap = read_u32(buffer_len) as usize;
        if buffer.is_null() || cap < 4 {
            write_u32(buffer_len, 4);
            crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        write_u32(buffer as *mut u32, val);
        write_u32(buffer_len, 4);
        TRUE
    };

    let write_usize_out = |val: usize| -> BOOL {
        let needed = std::mem::size_of::<usize>();
        let cap = read_u32(buffer_len) as usize;
        if buffer.is_null() || cap < needed {
            write_u32(buffer_len, needed as u32);
            crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        write_usize(buffer as *mut usize, val);
        write_u32(buffer_len, needed as u32);
        TRUE
    };

    let timeouts = match handle {
        WinHttpHandle::Session(s) => s.timeouts,
        WinHttpHandle::Connection(c) => c.timeouts,
        WinHttpHandle::Request(r) => r.timeouts,
    };

    match option {
        WINHTTP_OPTION_RESOLVE_TIMEOUT => write_dword_out(timeouts.resolve_ms as u32),
        WINHTTP_OPTION_CONNECT_TIMEOUT => write_dword_out(timeouts.connect_ms as u32),
        WINHTTP_OPTION_SEND_TIMEOUT => write_dword_out(timeouts.send_ms as u32),
        WINHTTP_OPTION_RECEIVE_TIMEOUT | WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT => {
            write_dword_out(timeouts.receive_ms as u32)
        }
        WINHTTP_OPTION_HANDLE_TYPE => {
            let ty = match handle {
                WinHttpHandle::Session(_) => WINHTTP_HANDLE_TYPE_SESSION,
                WinHttpHandle::Connection(_) => WINHTTP_HANDLE_TYPE_CONNECT,
                WinHttpHandle::Request(_) => WINHTTP_HANDLE_TYPE_REQUEST,
            };
            write_dword_out(ty)
        }
        WINHTTP_OPTION_PARENT_HANDLE => {
            let parent = match handle {
                WinHttpHandle::Session(_) => 0,
                WinHttpHandle::Connection(c) => c.parent_session,
                WinHttpHandle::Request(r) => r.parent_connect,
            };
            write_usize_out(parent)
        }
        WINHTTP_OPTION_REDIRECT_POLICY => {
            let p = match handle {
                WinHttpHandle::Session(s) => s.redirect_policy,
                WinHttpHandle::Connection(c) => c.redirect_policy,
                WinHttpHandle::Request(r) => r.redirect_policy,
            };
            write_dword_out(p)
        }
        WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS => {
            let m = match handle {
                WinHttpHandle::Session(s) => s.max_redirects,
                WinHttpHandle::Connection(c) => c.max_redirects,
                WinHttpHandle::Request(r) => r.max_redirects,
            };
            write_dword_out(m)
        }
        WINHTTP_OPTION_SECURITY_FLAGS => {
            let flags = match handle {
                WinHttpHandle::Session(s) => s.security_flags,
                WinHttpHandle::Connection(c) => c.security_flags,
                WinHttpHandle::Request(r) => {
                    let mut f = r.security_flags;
                    if r.is_secure {
                        f |= SECURITY_FLAG_SECURE | SECURITY_FLAG_STRENGTH_STRONG;
                    }
                    f
                }
            };
            write_dword_out(flags)
        }
        WINHTTP_OPTION_CONTEXT_VALUE => {
            let ctx = match handle {
                WinHttpHandle::Session(s) => s.context,
                WinHttpHandle::Connection(c) => c.context,
                WinHttpHandle::Request(r) => r.context,
            };
            write_usize_out(ctx)
        }
        WINHTTP_OPTION_CALLBACK => {
            let cb = match handle {
                WinHttpHandle::Session(s) => s.status_callback,
                WinHttpHandle::Connection(c) => c.status_callback,
                WinHttpHandle::Request(r) => r.status_callback,
            };
            write_usize_out(cb)
        }
        WINHTTP_OPTION_URL => match handle {
            WinHttpHandle::Request(r) => {
                if ansi {
                    write_ansi_bytes_out(&r.effective_url, buffer, buffer_len)
                } else {
                    write_wide_bytes_out(&r.effective_url, buffer, buffer_len)
                }
            }
            _ => {
                crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
                FALSE
            }
        },
        WINHTTP_OPTION_USER_AGENT => {
            let ua = match handle {
                WinHttpHandle::Session(s) => &s.user_agent,
                WinHttpHandle::Connection(c) => &c.user_agent,
                WinHttpHandle::Request(r) => &r.user_agent,
            };
            if ansi {
                write_ansi_bytes_out(ua, buffer, buffer_len)
            } else {
                write_wide_bytes_out(ua, buffer, buffer_len)
            }
        }
        WINHTTP_OPTION_HTTP_PROTOCOL_USED => write_dword_out(0), // HTTP/1.1
        _ => {
            crate::teb::set_last_error(ERROR_WINHTTP_INVALID_OPTION);
            FALSE
        }
    }
}

// ---------------------------------------------------------------------------
// Exported WinHTTP / SENSAPI functions
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn WinHttpCheckPlatform() -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn WinHttpGetDefaultProxyConfiguration(
    info: *mut WINHTTP_PROXY_INFO,
) -> BOOL {
    if info.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    log::debug!("WinHttpGetDefaultProxyConfiguration -> WINHTTP_ACCESS_TYPE_NO_PROXY");
    std::ptr::write_unaligned(
        info,
        WINHTTP_PROXY_INFO {
            access_type: WINHTTP_ACCESS_TYPE_NO_PROXY,
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        },
    );
    TRUE
}

pub unsafe extern "win64" fn WinHttpGetIEProxyConfigForCurrentUser(
    cfg: *mut WINHTTP_CURRENT_USER_IE_PROXY_CONFIG,
) -> BOOL {
    if cfg.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    log::debug!("WinHttpGetIEProxyConfigForCurrentUser -> no IE/WPAD proxy configured");
    std::ptr::write_unaligned(
        cfg,
        WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
            auto_detect: FALSE,
            auto_config_url: std::ptr::null_mut(),
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        },
    );
    TRUE
}

pub unsafe extern "win64" fn WinHttpGetProxyForUrl(
    session: usize,
    url: LPCWSTR,
    auto_proxy_options: *const WINHTTP_AUTOPROXY_OPTIONS,
    proxy_info: *mut WINHTTP_PROXY_INFO,
) -> BOOL {
    if url.is_null() || auto_proxy_options.is_null() || proxy_info.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    {
        let map = HANDLES.lock().unwrap();
        match map.get(&session) {
            Some(WinHttpHandle::Session(_)) => {}
            Some(_) => {
                crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
                return FALSE;
            }
            None => {
                crate::teb::set_last_error(ERROR_INVALID_HANDLE);
                return FALSE;
            }
        }
    }
    let opts = std::ptr::read_unaligned(auto_proxy_options);
    std::ptr::write_unaligned(
        proxy_info,
        WINHTTP_PROXY_INFO {
            access_type: WINHTTP_ACCESS_TYPE_NO_PROXY,
            proxy: std::ptr::null_mut(),
            bypass: std::ptr::null_mut(),
        },
    );
    log::debug!(
        "WinHttpGetProxyForUrl(flags={:#x}, auto_detect={:#x}) -> ERROR_WINHTTP_AUTODETECTION_FAILED",
        opts.dw_flags,
        opts.dw_auto_detect_flags
    );
    crate::teb::set_last_error(ERROR_WINHTTP_AUTODETECTION_FAILED);
    FALSE
}

pub unsafe extern "win64" fn WinHttpDetectAutoProxyConfigUrl(
    _auto_detect_flags: u32,
    auto_config_url: *mut LPWSTR,
) -> BOOL {
    if !auto_config_url.is_null() {
        std::ptr::write_unaligned(auto_config_url, std::ptr::null_mut());
    }
    crate::teb::set_last_error(ERROR_WINHTTP_AUTODETECTION_FAILED);
    FALSE
}

pub unsafe extern "win64" fn IsNetworkAlive(flags: *mut u32) -> BOOL {
    if is_offline_mode() {
        if !flags.is_null() {
            write_u32(flags, 0);
        }
        return FALSE;
    }
    if has_active_network_interface() {
        if !flags.is_null() {
            write_u32(flags, NETWORK_ALIVE_LAN);
        }
        TRUE
    } else {
        if !flags.is_null() {
            write_u32(flags, 0);
        }
        FALSE
    }
}

pub unsafe extern "win64" fn WinHttpOpen(
    user_agent: LPCWSTR,
    _access_type: u32,
    _proxy_name: LPCWSTR,
    _proxy_bypass: LPCWSTR,
    _flags: u32,
) -> usize {
    let ua = read_wide_z(user_agent);
    open_session_impl(ua)
}

pub unsafe extern "win64" fn WinHttpConnect(
    session: usize,
    server_name: LPCWSTR,
    server_port: u16,
    reserved: u32,
) -> usize {
    if reserved != 0 || server_name.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let Some(host) = read_wide_z(server_name) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    connect_impl(session, host, server_port, 0)
}

pub unsafe extern "win64" fn WinHttpOpenRequest(
    connect: usize,
    verb: LPCWSTR,
    object_name: LPCWSTR,
    version: LPCWSTR,
    referrer: LPCWSTR,
    accept_types: *const LPCWSTR,
    flags: u32,
) -> usize {
    let verb_s = read_wide_z(verb);
    let obj_s = read_wide_z(object_name);
    let ver_s = read_wide_z(version);
    let ref_s = read_wide_z(referrer);
    let mut accepts = Vec::new();
    if !accept_types.is_null() {
        let mut i = 0usize;
        loop {
            let p = std::ptr::read_unaligned(accept_types.add(i));
            if p.is_null() {
                break;
            }
            if let Some(s) = read_wide_z(p) {
                if !s.is_empty() {
                    accepts.push(s);
                }
            }
            i += 1;
        }
    }
    open_request_impl(connect, verb_s, obj_s, ver_s, ref_s, accepts, flags, 0, 0, 0)
}

pub unsafe extern "win64" fn WinHttpAddRequestHeaders(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    modifiers: u32,
) -> BOOL {
    if headers.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(raw) = read_wide_len_neg1(headers, headers_len) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    add_request_headers_impl(request, &raw, modifiers)
}

pub unsafe extern "win64" fn WinHttpSetTimeouts(
    h_internet: usize,
    resolve_timeout: i32,
    connect_timeout: i32,
    send_timeout: i32,
    receive_timeout: i32,
) -> BOOL {
    if resolve_timeout < -1 || connect_timeout < -1 || send_timeout < -1 || receive_timeout < -1 {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut map = HANDLES.lock().unwrap();
    let Some(handle) = map.get_mut(&h_internet) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    let t = Timeouts {
        resolve_ms: resolve_timeout.max(0),
        connect_ms: connect_timeout.max(0),
        send_ms: send_timeout.max(0),
        receive_ms: receive_timeout.max(0),
    };
    match handle {
        WinHttpHandle::Session(s) => s.timeouts = t,
        WinHttpHandle::Connection(c) => c.timeouts = t,
        WinHttpHandle::Request(r) => r.timeouts = t,
    }
    TRUE
}

pub unsafe extern "win64" fn WinHttpSetOption(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
) -> BOOL {
    set_option_impl(h_internet, option, buffer, buffer_len, false)
}

pub unsafe extern "win64" fn WinHttpQueryOption(
    h_internet: usize,
    option: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
) -> BOOL {
    query_option_impl(h_internet, option, buffer, buffer_len, false)
}

pub unsafe extern "win64" fn WinHttpSetStatusCallback(
    h_internet: usize,
    callback: usize,
    _notification_flags: u32,
    _reserved: usize,
) -> usize {
    let mut map = HANDLES.lock().unwrap();
    let Some(handle) = map.get_mut(&h_internet) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return WINHTTP_INVALID_STATUS_CALLBACK;
    };
    let slot = match handle {
        WinHttpHandle::Session(s) => &mut s.status_callback,
        WinHttpHandle::Connection(c) => &mut c.status_callback,
        WinHttpHandle::Request(r) => &mut r.status_callback,
    };
    let prev = *slot;
    *slot = callback;
    prev
}

pub unsafe extern "win64" fn WinHttpSetCredentials(
    request: usize,
    auth_targets: u32,
    auth_scheme: u32,
    user_name: LPCWSTR,
    password: LPCWSTR,
    _auth_params: *mut u8,
) -> BOOL {
    if auth_scheme != WINHTTP_AUTH_SCHEME_BASIC {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let user = read_wide_z(user_name).unwrap_or_default();
    let pass = read_wide_z(password).unwrap_or_default();
    let encoded = base64_encode(format!("{user}:{pass}").as_bytes());
    let hdr_name = if auth_targets == WINHTTP_AUTH_TARGET_PROXY {
        "Proxy-Authorization"
    } else {
        "Authorization"
    };
    add_request_headers_impl(
        request,
        &format!("{hdr_name}: Basic {encoded}"),
        WINHTTP_ADDREQ_FLAG_ADD | WINHTTP_ADDREQ_FLAG_REPLACE,
    )
}

pub unsafe extern "win64" fn WinHttpQueryAuthSchemes(
    request: usize,
    supported_schemes: *mut u32,
    first_scheme: *mut u32,
    auth_target: *mut u32,
) -> BOOL {
    if supported_schemes.is_null() || first_scheme.is_null() || auth_target.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let map = HANDLES.lock().unwrap();
    let req = match map.get(&request) {
        Some(WinHttpHandle::Request(r)) => r,
        Some(_) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };
    let Some(resp) = req.response.as_ref() else {
        crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        return FALSE;
    };

    let (target, hdr_name) = if resp.status_code == 407 {
        (WINHTTP_AUTH_TARGET_PROXY, "Proxy-Authenticate")
    } else {
        (WINHTTP_AUTH_TARGET_SERVER, "WWW-Authenticate")
    };

    let mut mask = 0u32;
    let mut first = 0u32;
    for (k, v) in &resp.headers {
        if k.eq_ignore_ascii_case(hdr_name) {
            let scheme_tok = v.split_whitespace().next().unwrap_or("");
            let bit = if scheme_tok.eq_ignore_ascii_case("Basic") {
                WINHTTP_AUTH_SCHEME_BASIC
            } else if scheme_tok.eq_ignore_ascii_case("Digest") {
                WINHTTP_AUTH_SCHEME_DIGEST
            } else if scheme_tok.eq_ignore_ascii_case("NTLM") {
                WINHTTP_AUTH_SCHEME_NTLM
            } else if scheme_tok.eq_ignore_ascii_case("Negotiate") {
                WINHTTP_AUTH_SCHEME_NEGOTIATE
            } else {
                0
            };
            if bit != 0 {
                if first == 0 {
                    first = bit;
                }
                mask |= bit;
            }
        }
    }
    if mask == 0 {
        crate::teb::set_last_error(ERROR_WINHTTP_HEADER_NOT_FOUND);
        return FALSE;
    }
    write_u32(supported_schemes, mask);
    write_u32(first_scheme, first);
    write_u32(auth_target, target);
    TRUE
}

pub unsafe extern "win64" fn WinHttpSendRequest(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    optional: *const u8,
    optional_len: u32,
    total_len: u32,
    context: usize,
) -> BOOL {
    let extra_headers = read_wide_len_neg1(headers, headers_len);
    send_request_impl(request, extra_headers, optional, optional_len, total_len, context)
}

pub unsafe extern "win64" fn WinHttpWriteData(
    request: usize,
    buffer: *const u8,
    bytes_to_write: u32,
    bytes_written: *mut u32,
) -> BOOL {
    if bytes_to_write > 0 && buffer.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut map = HANDLES.lock().unwrap();
    let req = match map.get_mut(&request) {
        Some(WinHttpHandle::Request(r)) => r,
        Some(_) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };
    if !req.sent {
        crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        return FALSE;
    }
    if bytes_to_write > 0 {
        let slice = std::slice::from_raw_parts(buffer, bytes_to_write as usize);
        req.request_body.extend_from_slice(slice);
    }
    if !bytes_written.is_null() {
        write_u32(bytes_written, bytes_to_write);
    }
    TRUE
}

pub unsafe extern "win64" fn WinHttpReceiveResponse(request: usize, _reserved: *mut u8) -> BOOL {
    receive_response_impl(request)
}

pub unsafe extern "win64" fn WinHttpQueryHeaders(
    request: usize,
    info_level: u32,
    name: LPCWSTR,
    buffer: *mut u8,
    buffer_len: *mut u32,
    index: *mut u32,
) -> BOOL {
    let custom_name = read_wide_z(name);
    query_headers_impl(request, info_level, custom_name, buffer, buffer_len, index, false)
}

pub unsafe extern "win64" fn WinHttpQueryDataAvailable(
    request: usize,
    bytes_available: *mut u32,
) -> BOOL {
    let map = HANDLES.lock().unwrap();
    let req = match map.get(&request) {
        Some(WinHttpHandle::Request(r)) => r,
        Some(_) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };
    let Some(resp) = req.response.as_ref() else {
        crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        return FALSE;
    };
    let rem = resp.body.len().saturating_sub(resp.read_pos).min(u32::MAX as usize) as u32;
    if !bytes_available.is_null() {
        write_u32(bytes_available, rem);
    }
    TRUE
}

pub unsafe extern "win64" fn WinHttpReadData(
    request: usize,
    buffer: *mut u8,
    bytes_to_read: u32,
    bytes_read: *mut u32,
) -> BOOL {
    if bytes_to_read > 0 && buffer.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut map = HANDLES.lock().unwrap();
    let req = match map.get_mut(&request) {
        Some(WinHttpHandle::Request(r)) => r,
        Some(_) => {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };
    let Some(resp) = req.response.as_mut() else {
        if !bytes_read.is_null() {
            write_u32(bytes_read, 0);
        }
        crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        return FALSE;
    };
    let avail = resp.body.len().saturating_sub(resp.read_pos);
    let n = avail.min(bytes_to_read as usize);
    if n > 0 {
        std::ptr::copy_nonoverlapping(resp.body[resp.read_pos..].as_ptr(), buffer, n);
        resp.read_pos += n;
    }
    if !bytes_read.is_null() {
        write_u32(bytes_read, n as u32);
    }
    TRUE
}

pub unsafe extern "win64" fn WinHttpCrackUrl(
    url: LPCWSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSW,
) -> BOOL {
    crack_url_w_impl(url, url_len, flags, components, false)
}

pub unsafe extern "win64" fn WinHttpCreateUrl(
    components: *const URL_COMPONENTSW,
    flags: u32,
    url: LPWSTR,
    url_len: *mut u32,
) -> BOOL {
    if components.is_null() || url_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let comp = std::ptr::read_unaligned(components);
    if (comp.dw_struct_size as usize) < std::mem::size_of::<URL_COMPONENTSW>() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let scheme = match read_wide_len_zero(comp.lpsz_scheme, comp.dw_scheme_length)
        .filter(|s| !s.is_empty())
    {
        Some(s) => s,
        None => match comp.n_scheme {
            WINHTTP_SCHEME_HTTP => "http".to_string(),
            WINHTTP_SCHEME_HTTPS => "https".to_string(),
            _ => {
                crate::teb::set_last_error(ERROR_WINHTTP_UNRECOGNIZED_SCHEME);
                return FALSE;
            }
        },
    };
    let is_https = scheme.eq_ignore_ascii_case("https");
    let user = read_wide_len_zero(comp.lpsz_user_name, comp.dw_user_name_length).unwrap_or_default();
    let pass = read_wide_len_zero(comp.lpsz_password, comp.dw_password_length).unwrap_or_default();
    let host = read_wide_len_zero(comp.lpsz_host_name, comp.dw_host_name_length).unwrap_or_default();
    let path = read_wide_len_zero(comp.lpsz_url_path, comp.dw_url_path_length).unwrap_or_default();
    let extra =
        read_wide_len_zero(comp.lpsz_extra_info, comp.dw_extra_info_length).unwrap_or_default();

    let mut built = String::new();
    built.push_str(&scheme);
    built.push_str("://");
    if !user.is_empty() {
        built.push_str(&user);
        if !pass.is_empty() {
            built.push(':');
            built.push_str(&pass);
        }
        built.push('@');
    }
    if host.contains(':') && !host.starts_with('[') {
        built.push('[');
        built.push_str(&host);
        built.push(']');
    } else {
        built.push_str(&host);
    }
    let default_port = if is_https {
        INTERNET_DEFAULT_HTTPS_PORT
    } else {
        INTERNET_DEFAULT_HTTP_PORT
    };
    if comp.n_port != 0 && comp.n_port != default_port {
        built.push(':');
        built.push_str(&comp.n_port.to_string());
    }
    if !path.is_empty() {
        if !host.is_empty() && !path.starts_with('/') {
            built.push('/');
        }
        built.push_str(&path);
    }
    if !extra.is_empty() {
        built.push_str(&extra);
    }
    if (flags & ICU_ESCAPE) != 0 {
        built = percent_escape_url(&built);
    }

    let units: Vec<u16> = built.encode_utf16().collect();
    let needed = units.len() + 1;
    let cap = read_u32(url_len) as usize;
    if url.is_null() || cap < needed {
        write_u32(url_len, needed as u32);
        crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    write_wide_units(url, &units);
    std::ptr::write_unaligned(url.add(units.len()), 0u16);
    write_u32(url_len, units.len() as u32);
    TRUE
}

pub unsafe extern "win64" fn WinHttpCloseHandle(h_internet: usize) -> BOOL {
    if h_internet == 0 {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if HANDLES.lock().unwrap().remove(&h_internet).is_some() {
        TRUE
    } else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        FALSE
    }
}

// ---------------------------------------------------------------------------
// Exported WinINet functions (`wininet.dll` aliases to `winhttp.dll`)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn InternetGetConnectedState(flags: *mut u32, _reserved: u32) -> BOOL {
    if is_offline_mode() {
        if !flags.is_null() {
            write_u32(flags, 0);
        }
        return FALSE;
    }
    if has_active_network_interface() {
        if !flags.is_null() {
            write_u32(flags, INTERNET_CONNECTION_LAN);
        }
        TRUE
    } else {
        if !flags.is_null() {
            write_u32(flags, 0);
        }
        FALSE
    }
}

pub unsafe extern "win64" fn InternetOpenW(
    agent: LPCWSTR,
    _access_type: u32,
    _proxy: LPCWSTR,
    _proxy_bypass: LPCWSTR,
    _flags: u32,
) -> usize {
    open_session_impl(read_wide_z(agent))
}

pub unsafe extern "win64" fn InternetOpenA(
    agent: LPCSTR,
    _access_type: u32,
    _proxy: LPCSTR,
    _proxy_bypass: LPCSTR,
    _flags: u32,
) -> usize {
    open_session_impl(read_ansi_z(agent))
}

pub unsafe extern "win64" fn InternetConnectW(
    session: usize,
    server_name: LPCWSTR,
    server_port: u16,
    _user_name: LPCWSTR,
    _password: LPCWSTR,
    _service: u32,
    _flags: u32,
    context: usize,
) -> usize {
    let Some(host) = read_wide_z(server_name) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    connect_impl(session, host, server_port, context)
}

pub unsafe extern "win64" fn InternetConnectA(
    session: usize,
    server_name: LPCSTR,
    server_port: u16,
    _user_name: LPCSTR,
    _password: LPCSTR,
    _service: u32,
    _flags: u32,
    context: usize,
) -> usize {
    let Some(host) = read_ansi_z(server_name) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    connect_impl(session, host, server_port, context)
}

fn translate_wininet_request_flags(flags: u32) -> (u32, u32) {
    let mut disable = 0u32;
    let mut sec = 0u32;
    if (flags & INTERNET_FLAG_NO_AUTO_REDIRECT) != 0 {
        disable |= WINHTTP_DISABLE_REDIRECTS;
    }
    if (flags & INTERNET_FLAG_IGNORE_CERT_CN_INVALID) != 0 {
        sec |= SECURITY_FLAG_IGNORE_CERT_CN_INVALID;
    }
    if (flags & INTERNET_FLAG_IGNORE_CERT_DATE_INVALID) != 0 {
        sec |= SECURITY_FLAG_IGNORE_CERT_DATE_INVALID;
    }
    (disable, sec)
}

pub unsafe extern "win64" fn HttpOpenRequestW(
    connect: usize,
    verb: LPCWSTR,
    object_name: LPCWSTR,
    version: LPCWSTR,
    referrer: LPCWSTR,
    accept_types: *const LPCWSTR,
    flags: u32,
    context: usize,
) -> usize {
    let mut accepts = Vec::new();
    if !accept_types.is_null() {
        let mut i = 0usize;
        loop {
            let p = std::ptr::read_unaligned(accept_types.add(i));
            if p.is_null() {
                break;
            }
            if let Some(s) = read_wide_z(p) {
                if !s.is_empty() {
                    accepts.push(s);
                }
            }
            i += 1;
        }
    }
    let (disable, sec) = translate_wininet_request_flags(flags);
    open_request_impl(
        connect,
        read_wide_z(verb),
        read_wide_z(object_name),
        read_wide_z(version),
        read_wide_z(referrer),
        accepts,
        flags,
        disable,
        sec,
        context,
    )
}

pub unsafe extern "win64" fn HttpOpenRequestA(
    connect: usize,
    verb: LPCSTR,
    object_name: LPCSTR,
    version: LPCSTR,
    referrer: LPCSTR,
    accept_types: *const LPCSTR,
    flags: u32,
    context: usize,
) -> usize {
    let mut accepts = Vec::new();
    if !accept_types.is_null() {
        let mut i = 0usize;
        loop {
            let p = std::ptr::read_unaligned(accept_types.add(i));
            if p.is_null() {
                break;
            }
            if let Some(s) = read_ansi_z(p) {
                if !s.is_empty() {
                    accepts.push(s);
                }
            }
            i += 1;
        }
    }
    let (disable, sec) = translate_wininet_request_flags(flags);
    open_request_impl(
        connect,
        read_ansi_z(verb),
        read_ansi_z(object_name),
        read_ansi_z(version),
        read_ansi_z(referrer),
        accepts,
        flags,
        disable,
        sec,
        context,
    )
}

pub unsafe extern "win64" fn HttpAddRequestHeadersW(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    modifiers: u32,
) -> BOOL {
    WinHttpAddRequestHeaders(request, headers, headers_len, modifiers)
}

pub unsafe extern "win64" fn HttpAddRequestHeadersA(
    request: usize,
    headers: LPCSTR,
    headers_len: u32,
    modifiers: u32,
) -> BOOL {
    if headers.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(raw) = read_ansi_len_neg1(headers, headers_len) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    add_request_headers_impl(request, &raw, modifiers)
}

pub unsafe extern "win64" fn HttpSendRequestW(
    request: usize,
    headers: LPCWSTR,
    headers_len: u32,
    optional: *const u8,
    optional_len: u32,
) -> BOOL {
    let extra = read_wide_len_neg1(headers, headers_len);
    if send_request_impl(request, extra, optional, optional_len, optional_len, 0) == FALSE {
        return FALSE;
    }
    receive_response_impl(request)
}

pub unsafe extern "win64" fn HttpSendRequestA(
    request: usize,
    headers: LPCSTR,
    headers_len: u32,
    optional: *const u8,
    optional_len: u32,
) -> BOOL {
    let extra = read_ansi_len_neg1(headers, headers_len);
    if send_request_impl(request, extra, optional, optional_len, optional_len, 0) == FALSE {
        return FALSE;
    }
    receive_response_impl(request)
}

pub unsafe extern "win64" fn HttpQueryInfoW(
    request: usize,
    info_level: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
    index: *mut u32,
) -> BOOL {
    let custom_name = if (info_level & 0xFFFF) == WINHTTP_QUERY_CUSTOM && !buffer.is_null() {
        read_wide_z(buffer as LPCWSTR)
    } else {
        None
    };
    query_headers_impl(request, info_level, custom_name, buffer, buffer_len, index, false)
}

pub unsafe extern "win64" fn HttpQueryInfoA(
    request: usize,
    info_level: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
    index: *mut u32,
) -> BOOL {
    let custom_name = if (info_level & 0xFFFF) == WINHTTP_QUERY_CUSTOM && !buffer.is_null() {
        read_ansi_z(buffer as LPCSTR)
    } else {
        None
    };
    query_headers_impl(request, info_level, custom_name, buffer, buffer_len, index, true)
}

fn internet_open_url_impl(
    session: usize,
    url_str: &str,
    headers: Option<String>,
    flags: u32,
    context: usize,
) -> usize {
    let parsed = match parse_url_parts(url_str) {
        Ok(p) => p,
        Err(e) => {
            crate::teb::set_last_error(e);
            return 0;
        }
    };
    let host = url_str[parsed.host_start..parsed.host_end].to_string();
    let mut path = url_str[parsed.path_start..parsed.extra_end].to_string();
    if path.is_empty() {
        path = "/".to_string();
    }
    let parent = {
        let map = HANDLES.lock().unwrap();
        match map.get(&session) {
            Some(WinHttpHandle::Session(s)) => s.clone(),
            Some(_) => {
                crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
                return 0;
            }
            None => {
                crate::teb::set_last_error(ERROR_INVALID_HANDLE);
                return 0;
            }
        }
    };
    let (disable, sec) = translate_wininet_request_flags(flags);
    let effective_url = format_url(parsed.is_https, &host, parsed.port, &path);
    let mut hdrs = Vec::new();
    if let Some(h) = headers.filter(|s| !s.is_empty()) {
        if let Err(e) = apply_header_block(&mut hdrs, &h, WINHTTP_ADDREQ_FLAG_ADD) {
            crate::teb::set_last_error(e);
            return 0;
        }
    }
    let req_handle = alloc_handle(WinHttpHandle::Request(RequestState {
        parent_connect: session,
        host,
        port: parsed.port,
        is_secure: parsed.is_https,
        verb: "GET".to_string(),
        path,
        version: "HTTP/1.1".to_string(),
        user_agent: parent.user_agent,
        headers: hdrs,
        send_headers: Vec::new(),
        request_body: Vec::new(),
        sent: true,
        timeouts: parent.timeouts,
        redirect_policy: parent.redirect_policy,
        max_redirects: parent.max_redirects,
        disable_flags: parent.disable_flags | disable,
        security_flags: parent.security_flags | sec,
        context,
        status_callback: parent.status_callback,
        effective_url,
        response: None,
    }));
    if receive_response_impl(req_handle) == FALSE {
        HANDLES.lock().unwrap().remove(&req_handle);
        return 0;
    }
    req_handle
}

pub unsafe extern "win64" fn InternetOpenUrlW(
    session: usize,
    url: LPCWSTR,
    headers: LPCWSTR,
    headers_len: u32,
    flags: u32,
    context: usize,
) -> usize {
    let Some(url_str) = read_wide_z(url) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    let hdrs = read_wide_len_neg1(headers, headers_len);
    internet_open_url_impl(session, &url_str, hdrs, flags, context)
}

pub unsafe extern "win64" fn InternetOpenUrlA(
    session: usize,
    url: LPCSTR,
    headers: LPCSTR,
    headers_len: u32,
    flags: u32,
    context: usize,
) -> usize {
    let Some(url_str) = read_ansi_z(url) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    };
    let hdrs = read_ansi_len_neg1(headers, headers_len);
    internet_open_url_impl(session, &url_str, hdrs, flags, context)
}

pub unsafe extern "win64" fn InternetReadFile(
    request: usize,
    buffer: *mut u8,
    bytes_to_read: u32,
    bytes_read: *mut u32,
) -> BOOL {
    WinHttpReadData(request, buffer, bytes_to_read, bytes_read)
}

pub unsafe extern "win64" fn InternetWriteFile(
    request: usize,
    buffer: *const u8,
    bytes_to_write: u32,
    bytes_written: *mut u32,
) -> BOOL {
    WinHttpWriteData(request, buffer, bytes_to_write, bytes_written)
}

pub unsafe extern "win64" fn InternetQueryDataAvailable(
    request: usize,
    bytes_available: *mut u32,
    _flags: u32,
    _context: usize,
) -> BOOL {
    WinHttpQueryDataAvailable(request, bytes_available)
}

pub unsafe extern "win64" fn InternetCloseHandle(h_internet: usize) -> BOOL {
    WinHttpCloseHandle(h_internet)
}

fn map_wininet_option(option: u32) -> u32 {
    match option {
        INTERNET_OPTION_CONNECT_TIMEOUT => WINHTTP_OPTION_CONNECT_TIMEOUT,
        INTERNET_OPTION_CONNECT_RETRIES => WINHTTP_OPTION_CONNECT_RETRIES,
        other => other,
    }
}

pub unsafe extern "win64" fn InternetSetOptionW(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
) -> BOOL {
    set_option_impl(h_internet, map_wininet_option(option), buffer, buffer_len, false)
}

pub unsafe extern "win64" fn InternetSetOptionA(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
) -> BOOL {
    set_option_impl(h_internet, map_wininet_option(option), buffer, buffer_len, true)
}

pub unsafe extern "win64" fn InternetQueryOptionW(
    h_internet: usize,
    option: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
) -> BOOL {
    query_option_impl(h_internet, map_wininet_option(option), buffer, buffer_len, false)
}

pub unsafe extern "win64" fn InternetQueryOptionA(
    h_internet: usize,
    option: u32,
    buffer: *mut u8,
    buffer_len: *mut u32,
) -> BOOL {
    query_option_impl(h_internet, map_wininet_option(option), buffer, buffer_len, true)
}

pub unsafe extern "win64" fn InternetCrackUrlW(
    url: LPCWSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSW,
) -> BOOL {
    crack_url_w_impl(url, url_len, flags, components, true)
}

pub unsafe extern "win64" fn InternetCrackUrlA(
    url: LPCSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSA,
) -> BOOL {
    if url.is_null() || components.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut comp = std::ptr::read_unaligned(components);
    if (comp.dw_struct_size as usize) < std::mem::size_of::<URL_COMPONENTSA>() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(url_str) = read_ansi_len_zero(url, url_len) else {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let parsed = match parse_url_parts(&url_str) {
        Ok(p) => p,
        Err(e) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    };

    let decode = (flags & ICU_DECODE) != 0;
    let extra_requested = !comp.lpsz_extra_info.is_null() || comp.dw_extra_info_length != 0;
    let path_end = if extra_requested {
        parsed.path_end
    } else {
        parsed.extra_end
    };

    comp.n_scheme = if parsed.is_https {
        WININET_SCHEME_HTTPS
    } else {
        WININET_SCHEME_HTTP
    };
    comp.n_port = parsed.port;

    let mut first_err: Option<u32> = None;
    let mut record = |res: Result<(), u32>| {
        if let Err(e) = res {
            if first_err.is_none() {
                first_err = Some(e);
            }
        }
    };

    record(fill_url_component_a(
        url,
        &url_str,
        parsed.scheme_start,
        parsed.scheme_end,
        false,
        &mut comp.lpsz_scheme,
        &mut comp.dw_scheme_length,
    ));
    record(fill_url_component_a(
        url,
        &url_str,
        parsed.host_start,
        parsed.host_end,
        false,
        &mut comp.lpsz_host_name,
        &mut comp.dw_host_name_length,
    ));
    record(fill_url_component_a(
        url,
        &url_str,
        parsed.user_start,
        parsed.user_end,
        decode,
        &mut comp.lpsz_user_name,
        &mut comp.dw_user_name_length,
    ));
    record(fill_url_component_a(
        url,
        &url_str,
        parsed.pass_start,
        parsed.pass_end,
        decode,
        &mut comp.lpsz_password,
        &mut comp.dw_password_length,
    ));
    record(fill_url_component_a(
        url,
        &url_str,
        parsed.path_start,
        path_end,
        decode,
        &mut comp.lpsz_url_path,
        &mut comp.dw_url_path_length,
    ));
    record(fill_url_component_a(
        url,
        &url_str,
        parsed.extra_start,
        parsed.extra_end,
        decode,
        &mut comp.lpsz_extra_info,
        &mut comp.dw_extra_info_length,
    ));

    std::ptr::write_unaligned(components, comp);
    if let Some(e) = first_err {
        crate::teb::set_last_error(e);
        return FALSE;
    }
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::sync::RwLock;

    /// Coordinates tests that read vs temporarily mutate `PORT_OFFLINE`.
    static OFFLINE_ENV_LOCK: LazyLock<RwLock<()>> = LazyLock::new(|| RwLock::new(()));

    struct OfflineGuard {
        prev: Option<String>,
    }

    impl OfflineGuard {
        fn set(val: &str) -> Self {
            let prev = std::env::var("PORT_OFFLINE").ok();
            std::env::set_var("PORT_OFFLINE", val);
            Self { prev }
        }
    }

    impl Drop for OfflineGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(v) => std::env::set_var("PORT_OFFLINE", v),
                None => std::env::remove_var("PORT_OFFLINE"),
            }
        }
    }

    fn wide_z(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn read_http_request(stream: &mut TcpStream) -> (String, Vec<(String, String)>, Vec<u8>) {
        let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
        let mut data = Vec::new();
        let mut tmp = [0u8; 4096];
        let header_end = loop {
            if let Some((end, sep)) = find_header_end(&data) {
                break (end, sep);
            }
            let n = stream.read(&mut tmp).expect("read request headers");
            assert!(n > 0, "unexpected EOF before HTTP request headers");
            data.extend_from_slice(&tmp[..n]);
        };
        let header_text = String::from_utf8_lossy(&data[..header_end.0]).into_owned();
        let mut lines = header_text.split('\n');
        let req_line = lines.next().unwrap_or("").trim_end_matches('\r').to_string();
        let mut headers = Vec::new();
        for l in lines {
            let l = l.trim_end_matches('\r');
            if let Some((k, v)) = l.split_once(':') {
                headers.push((k.trim().to_string(), v.trim().to_string()));
            }
        }
        let content_len = parse_content_length(&headers).unwrap_or(0);
        let mut body = data[header_end.0 + header_end.1..].to_vec();
        while body.len() < content_len {
            let n = stream.read(&mut tmp).expect("read request body");
            assert!(n > 0, "unexpected EOF reading request body");
            body.extend_from_slice(&tmp[..n]);
        }
        body.truncate(content_len);
        (req_line, headers, body)
    }

    unsafe fn query_header_wide(req: usize, info_level: u32, custom: Option<&str>) -> String {
        let custom_w = custom.map(wide_z);
        let name_ptr = custom_w
            .as_ref()
            .map_or(std::ptr::null(), |v| v.as_ptr());
        let mut needed: u32 = 0;
        let ok = WinHttpQueryHeaders(
            req,
            info_level,
            name_ptr,
            std::ptr::null_mut(),
            &mut needed,
            std::ptr::null_mut(),
        );
        assert_eq!(ok, FALSE);
        assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
        assert!(needed >= 2);
        let mut buf = vec![0u8; needed as usize];
        let mut len = needed;
        let ok2 = WinHttpQueryHeaders(
            req,
            info_level,
            name_ptr,
            buf.as_mut_ptr(),
            &mut len,
            std::ptr::null_mut(),
        );
        assert_eq!(ok2, TRUE);
        let units: Vec<u16> = buf[..len as usize]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        String::from_utf16_lossy(&units)
    }

    unsafe fn read_all_body(req: usize, chunk_size: u32) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let mut avail: u32 = 0;
            assert_eq!(WinHttpQueryDataAvailable(req, &mut avail), TRUE);
            if avail == 0 {
                let mut tmp = [0u8; 16];
                let mut got: u32 = 99;
                assert_eq!(WinHttpReadData(req, tmp.as_mut_ptr(), 16, &mut got), TRUE);
                assert_eq!(got, 0);
                break;
            }
            let to_read = avail.min(chunk_size);
            let mut buf = vec![0u8; to_read as usize];
            let mut got: u32 = 0;
            assert_eq!(WinHttpReadData(req, buf.as_mut_ptr(), to_read, &mut got), TRUE);
            assert!(got > 0);
            out.extend_from_slice(&buf[..got as usize]);
        }
        out
    }

    #[test]
    fn http_get_content_length_headers_and_options() {
        let _guard = OFFLINE_ENV_LOCK.read().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let body_payload = b"Hello from WinHTTP HTTP/1.1!";
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let (req_line, headers, body) = read_http_request(&mut stream);
            assert_eq!(req_line, "GET /api/status?v=2 HTTP/1.1");
            assert!(body.is_empty());
            assert_eq!(find_header(&headers, "User-Agent"), Some("WinHttpTest/1.0"));
            assert_eq!(find_header(&headers, "X-Test"), Some("updated"));
            assert_eq!(find_header(&headers, "X-List"), Some("alpha, beta"));
            assert_eq!(find_header(&headers, "X-Extra"), Some("send-hdr"));
            let resp = format!(
                "HTTP/1.1 200 OK\r\n\
                 Content-Type: text/plain; charset=utf-8\r\n\
                 Content-Length: {}\r\n\
                 X-Server-Tag: native-rust\r\n\
                 Connection: close\r\n\r\n",
                body_payload.len()
            );
            stream.write_all(resp.as_bytes()).unwrap();
            stream.write_all(body_payload).unwrap();
        });

        unsafe {
            let ua = wide_z("WinHttpTest/1.0");
            let session = WinHttpOpen(
                ua.as_ptr(),
                WINHTTP_ACCESS_TYPE_NO_PROXY,
                std::ptr::null(),
                std::ptr::null(),
                0,
            );
            assert_ne!(session, 0);
            assert_eq!(WinHttpSetTimeouts(session, 2000, 5000, 5000, 5000), TRUE);

            let host = wide_z("127.0.0.1");
            let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
            assert_ne!(conn, 0);

            let verb = wide_z("GET");
            let path = wide_z("/api/status?v=2");
            let req = WinHttpOpenRequest(
                conn,
                verb.as_ptr(),
                path.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                0,
            );
            assert_ne!(req, 0);

            let h1 = wide_z("X-Test: initial\r\nX-List: alpha");
            assert_eq!(
                WinHttpAddRequestHeaders(req, h1.as_ptr(), u32::MAX, WINHTTP_ADDREQ_FLAG_ADD),
                TRUE
            );
            let h2 = wide_z("X-Test: updated");
            assert_eq!(
                WinHttpAddRequestHeaders(req, h2.as_ptr(), u32::MAX, WINHTTP_ADDREQ_FLAG_REPLACE),
                TRUE
            );
            let h3 = wide_z("X-List: beta");
            assert_eq!(
                WinHttpAddRequestHeaders(
                    req,
                    h3.as_ptr(),
                    u32::MAX,
                    WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA,
                ),
                TRUE
            );

            let extra = wide_z("X-Extra: send-hdr");
            assert_eq!(
                WinHttpSendRequest(req, extra.as_ptr(), u32::MAX, std::ptr::null(), 0, 0, 0x1234),
                TRUE
            );
            assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), TRUE);

            // Status code as DWORD
            let mut status_num: u32 = 0;
            let mut status_len: u32 = 4;
            assert_eq!(
                WinHttpQueryHeaders(
                    req,
                    WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                    std::ptr::null(),
                    &mut status_num as *mut u32 as *mut u8,
                    &mut status_len,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            assert_eq!(status_num, 200);

            // Content-Length as DWORD
            let mut cl_num: u32 = 0;
            let mut cl_len: u32 = 4;
            assert_eq!(
                WinHttpQueryHeaders(
                    req,
                    5 | WINHTTP_QUERY_FLAG_NUMBER,
                    std::ptr::null(),
                    &mut cl_num as *mut u32 as *mut u8,
                    &mut cl_len,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            assert_eq!(cl_num as usize, body_payload.len());

            // String queries (all exercise ERROR_INSUFFICIENT_BUFFER probe first)
            assert_eq!(query_header_wide(req, WINHTTP_QUERY_STATUS_CODE, None), "200");
            assert_eq!(query_header_wide(req, WINHTTP_QUERY_STATUS_TEXT, None), "OK");
            assert_eq!(
                query_header_wide(req, 1, None),
                "text/plain; charset=utf-8"
            );
            assert_eq!(
                query_header_wide(req, WINHTTP_QUERY_CUSTOM, Some("X-Server-Tag")),
                "native-rust"
            );
            let raw_crlf = query_header_wide(req, WINHTTP_QUERY_RAW_HEADERS_CRLF, None);
            assert!(raw_crlf.starts_with("HTTP/1.1 200 OK\r\n"));
            assert!(raw_crlf.contains("X-Server-Tag: native-rust\r\n"));
            assert!(raw_crlf.ends_with("\r\n\r\n"));

            let raw_nul = query_header_wide(req, WINHTTP_QUERY_RAW_HEADERS, None);
            assert!(raw_nul.starts_with("HTTP/1.1 200 OK\0"));
            assert!(raw_nul.contains("X-Server-Tag: native-rust\0"));

            // Query options
            let mut handle_ty: u32 = 0;
            let mut opt_len: u32 = 4;
            assert_eq!(
                WinHttpQueryOption(
                    req,
                    WINHTTP_OPTION_HANDLE_TYPE,
                    &mut handle_ty as *mut u32 as *mut u8,
                    &mut opt_len,
                ),
                TRUE
            );
            assert_eq!(handle_ty, WINHTTP_HANDLE_TYPE_REQUEST);

            let mut url_bytes: u32 = 0;
            assert_eq!(
                WinHttpQueryOption(req, WINHTTP_OPTION_URL, std::ptr::null_mut(), &mut url_bytes),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            let mut url_buf = vec![0u8; url_bytes as usize];
            assert_eq!(
                WinHttpQueryOption(req, WINHTTP_OPTION_URL, url_buf.as_mut_ptr(), &mut url_bytes),
                TRUE
            );
            let url_units: Vec<u16> = url_buf[..url_bytes as usize]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            assert_eq!(
                String::from_utf16_lossy(&url_units),
                format!("http://127.0.0.1:{port}/api/status?v=2")
            );

            // Stream body in small 7-byte slices
            let received = read_all_body(req, 7);
            assert_eq!(received, body_payload);

            assert_eq!(WinHttpCloseHandle(req), TRUE);
            assert_eq!(WinHttpCloseHandle(conn), TRUE);
            assert_eq!(WinHttpCloseHandle(session), TRUE);
            assert_eq!(WinHttpCloseHandle(session), FALSE);
        }

        server.join().unwrap();
    }

    #[test]
    fn http_chunked_transfer_decoding() {
        let _guard = OFFLINE_ENV_LOCK.read().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let _ = read_http_request(&mut stream);
            let resp = concat!(
                "HTTP/1.1 200 OK\r\n",
                "Transfer-Encoding: chunked\r\n",
                "Content-Type: text/plain\r\n",
                "\r\n",
                "5;ext=1\r\nHello\r\n",
                "2\r\n, \r\n",
                "6\r\nWorld!\r\n",
                "0\r\n",
                "X-Trailer: ignored\r\n",
                "\r\n",
            );
            stream.write_all(resp.as_bytes()).unwrap();
        });

        unsafe {
            let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
            let host = wide_z("127.0.0.1");
            let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
            let path = wide_z("/chunked");
            let req = WinHttpOpenRequest(conn, std::ptr::null(), path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
            assert_eq!(WinHttpSendRequest(req, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
            assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), TRUE);

            // Content-Length header was not sent on chunked response.
            let mut cl: u32 = 0;
            let mut cl_len: u32 = 4;
            assert_eq!(
                WinHttpQueryHeaders(req, 5 | WINHTTP_QUERY_FLAG_NUMBER, std::ptr::null(), &mut cl as *mut u32 as *mut u8, &mut cl_len, std::ptr::null_mut()),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_HEADER_NOT_FOUND);

            let body = read_all_body(req, 64);
            assert_eq!(body, b"Hello, World!");

            WinHttpCloseHandle(req);
            WinHttpCloseHandle(conn);
            WinHttpCloseHandle(session);
        }

        server.join().unwrap();
    }

    #[test]
    fn http_redirect_302_follow_and_disable() {
        let _guard = OFFLINE_ENV_LOCK.read().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let server = std::thread::spawn(move || {
            // Serve 3 connections: 1st redirects -> 2nd final; 3rd redirects (client disables redirects).
            for _ in 0..3 {
                let (mut stream, _) = listener.accept().unwrap();
                let (req_line, _, _) = read_http_request(&mut stream);
                if req_line.starts_with("GET /start ") {
                    let resp = "HTTP/1.1 302 Found\r\nLocation: /final?done=1\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                    stream.write_all(resp.as_bytes()).unwrap();
                } else if req_line.starts_with("GET /final?done=1 ") {
                    let body = b"redirected-ok";
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        body.len()
                    );
                    stream.write_all(resp.as_bytes()).unwrap();
                    stream.write_all(body).unwrap();
                } else {
                    panic!("unexpected request line: {req_line}");
                }
            }
        });

        unsafe {
            let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
            let host = wide_z("127.0.0.1");
            let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
            let start_path = wide_z("/start");

            // Case 1: automatic redirect enabled (default)
            let req1 = WinHttpOpenRequest(conn, std::ptr::null(), start_path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
            assert_eq!(WinHttpSendRequest(req1, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
            assert_eq!(WinHttpReceiveResponse(req1, std::ptr::null_mut()), TRUE);
            assert_eq!(query_header_wide(req1, WINHTTP_QUERY_STATUS_CODE, None), "200");
            assert_eq!(read_all_body(req1, 64), b"redirected-ok");
            let mut ulen = 0u32;
            WinHttpQueryOption(req1, WINHTTP_OPTION_URL, std::ptr::null_mut(), &mut ulen);
            let mut ubuf = vec![0u8; ulen as usize];
            assert_eq!(WinHttpQueryOption(req1, WINHTTP_OPTION_URL, ubuf.as_mut_ptr(), &mut ulen), TRUE);
            let u16s: Vec<u16> = ubuf[..ulen as usize].chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
            assert!(String::from_utf16_lossy(&u16s).ends_with("/final?done=1"));
            WinHttpCloseHandle(req1);

            // Case 2: redirects disabled via WINHTTP_OPTION_REDIRECT_POLICY_NEVER
            let req2 = WinHttpOpenRequest(conn, std::ptr::null(), start_path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
            let policy = WINHTTP_OPTION_REDIRECT_POLICY_NEVER;
            assert_eq!(
                WinHttpSetOption(req2, WINHTTP_OPTION_REDIRECT_POLICY, &policy as *const u32 as *const u8, 4),
                TRUE
            );
            assert_eq!(WinHttpSendRequest(req2, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
            assert_eq!(WinHttpReceiveResponse(req2, std::ptr::null_mut()), TRUE);
            assert_eq!(query_header_wide(req2, WINHTTP_QUERY_STATUS_CODE, None), "302");
            assert_eq!(query_header_wide(req2, 33, None), "/final?done=1");
            WinHttpCloseHandle(req2);

            WinHttpCloseHandle(conn);
            WinHttpCloseHandle(session);
        }

        server.join().unwrap();
    }

    #[test]
    fn http_post_body_echo_with_write_data() {
        let _guard = OFFLINE_ENV_LOCK.read().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let (req_line, headers, body) = read_http_request(&mut stream);
            assert_eq!(req_line, "POST /echo HTTP/1.1");
            assert_eq!(find_header(&headers, "Content-Type"), Some("application/octet-stream"));
            let resp = format!(
                "HTTP/1.1 200 OK\r\nX-Method: POST\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(resp.as_bytes()).unwrap();
            stream.write_all(&body).unwrap();
        });

        unsafe {
            let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
            let host = wide_z("127.0.0.1");
            let conn = WinHttpConnect(session, host.as_ptr(), port, 0);
            let verb = wide_z("POST");
            let path = wide_z("/echo");
            let req = WinHttpOpenRequest(conn, verb.as_ptr(), path.as_ptr(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);

            let ct = wide_z("Content-Type: application/octet-stream");
            let part1 = b"first-part:";
            let part2 = b"second-streamed-part";
            let total = (part1.len() + part2.len()) as u32;
            assert_eq!(
                WinHttpSendRequest(
                    req,
                    ct.as_ptr(),
                    u32::MAX,
                    part1.as_ptr(),
                    part1.len() as u32,
                    total,
                    0,
                ),
                TRUE
            );
            let mut written: u32 = 0;
            assert_eq!(
                WinHttpWriteData(req, part2.as_ptr(), part2.len() as u32, &mut written),
                TRUE
            );
            assert_eq!(written as usize, part2.len());

            assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), TRUE);
            assert_eq!(query_header_wide(req, WINHTTP_QUERY_CUSTOM, Some("X-Method")), "POST");
            let echoed = read_all_body(req, 128);
            assert_eq!(echoed, b"first-part:second-streamed-part");

            WinHttpCloseHandle(req);
            WinHttpCloseHandle(conn);
            WinHttpCloseHandle(session);
        }

        server.join().unwrap();
    }

    #[test]
    fn crack_and_create_url_modes() {
        unsafe {
            let url = wide_z("https://user:pass@api.example.com:8443/v1/items?page=2&sort=asc");
            let mut scheme = vec![0u16; 16];
            let mut host = vec![0u16; 64];
            let mut user = vec![0u16; 32];
            let mut pass = vec![0u16; 32];
            let mut path = vec![0u16; 64];
            let mut extra = vec![0u16; 64];

            let mut comps = URL_COMPONENTSW {
                dw_struct_size: std::mem::size_of::<URL_COMPONENTSW>() as u32,
                lpsz_scheme: scheme.as_mut_ptr(),
                dw_scheme_length: scheme.len() as u32,
                n_scheme: 0,
                lpsz_host_name: host.as_mut_ptr(),
                dw_host_name_length: host.len() as u32,
                n_port: 0,
                lpsz_user_name: user.as_mut_ptr(),
                dw_user_name_length: user.len() as u32,
                lpsz_password: pass.as_mut_ptr(),
                dw_password_length: pass.len() as u32,
                lpsz_url_path: path.as_mut_ptr(),
                dw_url_path_length: path.len() as u32,
                lpsz_extra_info: extra.as_mut_ptr(),
                dw_extra_info_length: extra.len() as u32,
            };
            assert_eq!(WinHttpCrackUrl(url.as_ptr(), 0, 0, &mut comps), TRUE);
            assert_eq!(comps.n_scheme, WINHTTP_SCHEME_HTTPS);
            assert_eq!(comps.n_port, 8443);
            assert_eq!(String::from_utf16_lossy(&scheme[..comps.dw_scheme_length as usize]), "https");
            assert_eq!(String::from_utf16_lossy(&host[..comps.dw_host_name_length as usize]), "api.example.com");
            assert_eq!(String::from_utf16_lossy(&user[..comps.dw_user_name_length as usize]), "user");
            assert_eq!(String::from_utf16_lossy(&pass[..comps.dw_password_length as usize]), "pass");
            assert_eq!(String::from_utf16_lossy(&path[..comps.dw_url_path_length as usize]), "/v1/items");
            assert_eq!(String::from_utf16_lossy(&extra[..comps.dw_extra_info_length as usize]), "?page=2&sort=asc");

            // Roundtrip through WinHttpCreateUrl (buffer probe + write)
            let mut out_len: u32 = 0;
            assert_eq!(WinHttpCreateUrl(&comps, 0, std::ptr::null_mut(), &mut out_len), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_INSUFFICIENT_BUFFER);
            let mut out_buf = vec![0u16; out_len as usize];
            assert_eq!(WinHttpCreateUrl(&comps, 0, out_buf.as_mut_ptr(), &mut out_len), TRUE);
            assert_eq!(
                String::from_utf16_lossy(&out_buf[..out_len as usize]),
                "https://user:pass@api.example.com:8443/v1/items?page=2&sort=asc"
            );

            // In-place pointer mode with ExtraInfo omitted -> UrlPath includes query string
            let url2 = wide_z("http://example.com/a/b?k=v");
            let mut ip = URL_COMPONENTSW {
                dw_struct_size: std::mem::size_of::<URL_COMPONENTSW>() as u32,
                lpsz_scheme: std::ptr::null_mut(),
                dw_scheme_length: 0,
                n_scheme: 0,
                lpsz_host_name: std::ptr::null_mut(),
                dw_host_name_length: u32::MAX,
                n_port: 0,
                lpsz_user_name: std::ptr::null_mut(),
                dw_user_name_length: 0,
                lpsz_password: std::ptr::null_mut(),
                dw_password_length: 0,
                lpsz_url_path: std::ptr::null_mut(),
                dw_url_path_length: u32::MAX,
                lpsz_extra_info: std::ptr::null_mut(),
                dw_extra_info_length: 0,
            };
            assert_eq!(WinHttpCrackUrl(url2.as_ptr(), 0, 0, &mut ip), TRUE);
            assert_eq!(ip.n_scheme, WINHTTP_SCHEME_HTTP);
            assert_eq!(ip.n_port, 80);
            let host_slice = std::slice::from_raw_parts(ip.lpsz_host_name, ip.dw_host_name_length as usize);
            let path_slice = std::slice::from_raw_parts(ip.lpsz_url_path, ip.dw_url_path_length as usize);
            assert_eq!(String::from_utf16_lossy(host_slice), "example.com");
            assert_eq!(String::from_utf16_lossy(path_slice), "/a/b?k=v");

            // Unrecognized scheme
            let bad = wide_z("ftp://example.com/file");
            assert_eq!(WinHttpCrackUrl(bad.as_ptr(), 0, 0, &mut ip), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_UNRECOGNIZED_SCHEME);
        }
    }

    #[test]
    fn offline_mode_and_connection_refused_errors() {
        // 1) Connection refused on closed loopback port
        {
            let _guard = OFFLINE_ENV_LOCK.read().unwrap();
            let temp_listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let closed_port = temp_listener.local_addr().unwrap().port();
            drop(temp_listener);

            unsafe {
                let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
                let host = wide_z("127.0.0.1");
                let conn = WinHttpConnect(session, host.as_ptr(), closed_port, 0);
                let req = WinHttpOpenRequest(conn, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
                assert_eq!(WinHttpSendRequest(req, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
                assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), FALSE);
                assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_CANNOT_CONNECT);
                WinHttpCloseHandle(req);
                WinHttpCloseHandle(conn);
                WinHttpCloseHandle(session);
            }
        }

        // 2) PORT_OFFLINE=1 blocks requests honestly with ERROR_WINHTTP_CANNOT_CONNECT
        {
            let _write_guard = OFFLINE_ENV_LOCK.write().unwrap();
            let _offline = OfflineGuard::set("1");

            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let live_port = listener.local_addr().unwrap().port();

            unsafe {
                let mut alive_flags: u32 = 0xDEAD;
                assert_eq!(IsNetworkAlive(&mut alive_flags), FALSE);
                assert_eq!(alive_flags, 0);

                let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
                let host = wide_z("127.0.0.1");
                let conn = WinHttpConnect(session, host.as_ptr(), live_port, 0);
                let req = WinHttpOpenRequest(conn, std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), std::ptr::null(), 0);
                assert_eq!(WinHttpSendRequest(req, std::ptr::null(), 0, std::ptr::null(), 0, 0, 0), TRUE);
                assert_eq!(WinHttpReceiveResponse(req, std::ptr::null_mut()), FALSE);
                assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_CANNOT_CONNECT);
                WinHttpCloseHandle(req);
                WinHttpCloseHandle(conn);
                WinHttpCloseHandle(session);
            }
            drop(listener);
        }
    }

    #[test]
    fn proxy_queries_and_wininet_roundtrip() {
        let _guard = OFFLINE_ENV_LOCK.read().unwrap();
        unsafe {
            let session = WinHttpOpen(std::ptr::null(), WINHTTP_ACCESS_TYPE_NO_PROXY, std::ptr::null(), std::ptr::null(), 0);
            let mut def_proxy = WINHTTP_PROXY_INFO {
                access_type: 99,
                proxy: std::ptr::null_mut(),
                bypass: std::ptr::null_mut(),
            };
            assert_eq!(WinHttpGetDefaultProxyConfiguration(&mut def_proxy), TRUE);
            assert_eq!(def_proxy.access_type, WINHTTP_ACCESS_TYPE_NO_PROXY);

            let mut ie_cfg = WINHTTP_CURRENT_USER_IE_PROXY_CONFIG {
                auto_detect: TRUE,
                auto_config_url: std::ptr::null_mut(),
                proxy: std::ptr::null_mut(),
                bypass: std::ptr::null_mut(),
            };
            assert_eq!(WinHttpGetIEProxyConfigForCurrentUser(&mut ie_cfg), TRUE);
            assert_eq!(ie_cfg.auto_detect, FALSE);

            let url = wide_z("https://example.com/");
            let opts = WINHTTP_AUTOPROXY_OPTIONS {
                dw_flags: 1,
                dw_auto_detect_flags: 3,
                lpsz_auto_config_url: std::ptr::null(),
                lpv_reserved: std::ptr::null_mut(),
                dw_reserved: 0,
                f_auto_logon_if_challenged: FALSE,
            };
            assert_eq!(WinHttpGetProxyForUrl(session, url.as_ptr(), &opts, &mut def_proxy), FALSE);
            assert_eq!(crate::teb::last_error(), ERROR_WINHTTP_AUTODETECTION_FAILED);
            WinHttpCloseHandle(session);
        }

        // WinINet InternetOpenUrlW roundtrip
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let (req_line, _, _) = read_http_request(&mut stream);
            assert_eq!(req_line, "GET /inet HTTP/1.1");
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 7\r\nConnection: close\r\n\r\nwininet")
                .unwrap();
        });

        unsafe {
            let inet = InternetOpenA(b"WinINetTest\0".as_ptr(), 1, std::ptr::null(), std::ptr::null(), 0);
            assert_ne!(inet, 0);
            let url = wide_z(&format!("http://127.0.0.1:{port}/inet"));
            let h = InternetOpenUrlW(inet, url.as_ptr(), std::ptr::null(), 0, 0, 0);
            assert_ne!(h, 0);
            let mut status: u32 = 0;
            let mut slen: u32 = 4;
            assert_eq!(
                HttpQueryInfoA(
                    h,
                    WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                    &mut status as *mut u32 as *mut u8,
                    &mut slen,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            assert_eq!(status, 200);
            let mut buf = [0u8; 32];
            let mut got: u32 = 0;
            assert_eq!(InternetReadFile(h, buf.as_mut_ptr(), 32, &mut got), TRUE);
            assert_eq!(&buf[..got as usize], b"wininet");
            assert_eq!(InternetCloseHandle(h), TRUE);
            assert_eq!(InternetCloseHandle(inet), TRUE);
        }

        server.join().unwrap();
    }
}
