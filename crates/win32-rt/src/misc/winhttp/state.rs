//! WinHTTP / WinINet / SENSAPI constants, Win64 ABI structures, handle registry,
//! header/option/request state management, and POSIX network-liveness detection.

use crate::handles::HandleTable;
use crate::net_tls::{
    self, SECURITY_FLAG_IGNORE_CERT_CN_INVALID, SECURITY_FLAG_IGNORE_CERT_DATE_INVALID,
};
use crate::strings::{self, DEFAULT_MAX_STR_LEN};
use crate::types::*;

// ---------------------------------------------------------------------------
// Win32 / WinHTTP / WinINet / SENSAPI constants
// ---------------------------------------------------------------------------

pub(super) const ERROR_INVALID_HANDLE: u32 = 6;
pub(super) const ERROR_INVALID_PARAMETER: u32 = 87;
pub(super) const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

pub(super) const ERROR_WINHTTP_TIMEOUT: u32 = 12002;
pub(super) const ERROR_WINHTTP_INVALID_URL: u32 = 12005;
pub(super) const ERROR_WINHTTP_UNRECOGNIZED_SCHEME: u32 = 12006;
pub(super) const ERROR_WINHTTP_NAME_NOT_RESOLVED: u32 = 12007;
pub(super) const ERROR_WINHTTP_INVALID_OPTION: u32 = 12009;
pub(super) const ERROR_WINHTTP_OPERATION_CANCELLED: u32 = 12017;
pub(super) const ERROR_WINHTTP_INCORRECT_HANDLE_TYPE: u32 = 12018;
pub(super) const ERROR_WINHTTP_INCORRECT_HANDLE_STATE: u32 = 12019;
pub(super) const ERROR_WINHTTP_CANNOT_CONNECT: u32 = 12029;
pub(super) const ERROR_WINHTTP_CONNECTION_ERROR: u32 = 12030;
pub(super) const ERROR_WINHTTP_SECURE_CERT_DATE_INVALID: u32 = 12037;
pub(super) const ERROR_WINHTTP_SECURE_CERT_CN_INVALID: u32 = 12038;
pub(super) const ERROR_WINHTTP_SECURE_INVALID_CA: u32 = 12045;
pub(super) const ERROR_WINHTTP_HEADER_NOT_FOUND: u32 = 12150;
pub(super) const ERROR_WINHTTP_INVALID_SERVER_RESPONSE: u32 = 12152;
pub(super) const ERROR_WINHTTP_INVALID_HEADER: u32 = 12153;
pub(super) const ERROR_WINHTTP_INVALID_QUERY_REQUEST: u32 = 12154;
pub(super) const ERROR_WINHTTP_HEADER_ALREADY_EXISTS: u32 = 12155;
pub(super) const ERROR_WINHTTP_REDIRECT_FAILED: u32 = 12156;
pub(super) const ERROR_WINHTTP_SECURE_CHANNEL_ERROR: u32 = 12157;
pub(super) const ERROR_WINHTTP_AUTODETECTION_FAILED: u32 = 12180;

pub(super) const WINHTTP_ACCESS_TYPE_NO_PROXY: u32 = 1;

pub(super) const WINHTTP_FLAG_SECURE: u32 = 0x0080_0000;

pub(super) const WINHTTP_HANDLE_TYPE_SESSION: u32 = 1;
pub(super) const WINHTTP_HANDLE_TYPE_CONNECT: u32 = 2;
pub(super) const WINHTTP_HANDLE_TYPE_REQUEST: u32 = 3;

pub(super) const WINHTTP_SCHEME_HTTP: i32 = 1;
pub(super) const WINHTTP_SCHEME_HTTPS: i32 = 2;
pub(super) const WININET_SCHEME_HTTP: i32 = 3;
pub(super) const WININET_SCHEME_HTTPS: i32 = 4;

pub(super) const INTERNET_DEFAULT_HTTP_PORT: u16 = 80;
pub(super) const INTERNET_DEFAULT_HTTPS_PORT: u16 = 443;

// WinHttpAddRequestHeaders / HttpAddRequestHeaders modifiers
pub(super) const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_SEMICOLON: u32 = 0x0100_0000;
pub(super) const WINHTTP_ADDREQ_FLAG_ADD_IF_NEW: u32 = 0x1000_0000;
pub(super) const WINHTTP_ADDREQ_FLAG_ADD: u32 = 0x2000_0000;
pub(super) const WINHTTP_ADDREQ_FLAG_COALESCE_WITH_COMMA: u32 = 0x4000_0000;
pub(super) const WINHTTP_ADDREQ_FLAG_REPLACE: u32 = 0x8000_0000;

// WinHttpSetOption / WinHttpQueryOption identifiers
pub(super) const WINHTTP_OPTION_CALLBACK: u32 = 1;
pub(super) const WINHTTP_OPTION_RESOLVE_TIMEOUT: u32 = 2;
pub(super) const WINHTTP_OPTION_CONNECT_TIMEOUT: u32 = 3;
pub(super) const WINHTTP_OPTION_CONNECT_RETRIES: u32 = 4;
pub(super) const WINHTTP_OPTION_SEND_TIMEOUT: u32 = 5;
pub(super) const WINHTTP_OPTION_RECEIVE_TIMEOUT: u32 = 6;
pub(super) const WINHTTP_OPTION_RECEIVE_RESPONSE_TIMEOUT: u32 = 7;
pub(super) const WINHTTP_OPTION_HANDLE_TYPE: u32 = 9;
pub(super) const WINHTTP_OPTION_READ_BUFFER_SIZE: u32 = 12;
pub(super) const WINHTTP_OPTION_WRITE_BUFFER_SIZE: u32 = 13;
pub(super) const WINHTTP_OPTION_PARENT_HANDLE: u32 = 21;
pub(super) const WINHTTP_OPTION_SECURITY_FLAGS: u32 = 31;
pub(super) const WINHTTP_OPTION_URL: u32 = 34;
pub(super) const WINHTTP_OPTION_PROXY: u32 = 38;
pub(super) const WINHTTP_OPTION_USER_AGENT: u32 = 41;
pub(super) const WINHTTP_OPTION_CONTEXT_VALUE: u32 = 45;
pub(super) const WINHTTP_OPTION_DISABLE_FEATURE: u32 = 63;
pub(super) const WINHTTP_OPTION_ENABLE_FEATURE: u32 = 79;
pub(super) const WINHTTP_OPTION_SECURE_PROTOCOLS: u32 = 84;
pub(super) const WINHTTP_OPTION_REDIRECT_POLICY: u32 = 88;
pub(super) const WINHTTP_OPTION_MAX_HTTP_AUTOMATIC_REDIRECTS: u32 = 89;
pub(super) const WINHTTP_OPTION_MAX_CONNS_PER_SERVER: u32 = 103;
pub(super) const WINHTTP_OPTION_DECOMPRESSION: u32 = 118;
pub(super) const WINHTTP_OPTION_ENABLE_HTTP_PROTOCOL: u32 = 133;
pub(super) const WINHTTP_OPTION_HTTP_PROTOCOL_USED: u32 = 134;

// WinINet-specific option overrides where ID differs from WinHTTP
const INTERNET_OPTION_CONNECT_TIMEOUT: u32 = 2;
const INTERNET_OPTION_CONNECT_RETRIES: u32 = 3;

pub(super) const WINHTTP_DISABLE_REDIRECTS: u32 = 0x0000_0002;

pub(super) const WINHTTP_OPTION_REDIRECT_POLICY_NEVER: u32 = 0;
pub(super) const WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP: u32 = 1;
pub(super) const WINHTTP_OPTION_REDIRECT_POLICY_ALWAYS: u32 = 2;

// Security flags
const SECURITY_FLAG_SECURE: u32 = 0x0000_0001;
const SECURITY_FLAG_STRENGTH_STRONG: u32 = 0x2000_0000;

// WinINet request flags
const INTERNET_FLAG_NO_AUTO_REDIRECT: u32 = 0x0020_0000;
const INTERNET_FLAG_IGNORE_CERT_CN_INVALID: u32 = 0x0000_1000;
const INTERNET_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 0x0000_2000;

// WinHttpQueryHeaders / HttpQueryInfo flags & IDs
const WINHTTP_QUERY_FLAG_REQUEST_HEADERS: u32 = 0x8000_0000;
const WINHTTP_QUERY_FLAG_SYSTEMTIME: u32 = 0x4000_0000;
pub(super) const WINHTTP_QUERY_FLAG_NUMBER: u32 = 0x2000_0000;
const WINHTTP_QUERY_FLAG_COALESCE: u32 = 0x1000_0000;
const WINHTTP_QUERY_FLAG_NUMBER64: u32 = 0x0800_0000;

const WINHTTP_QUERY_VERSION: u32 = 18;
pub(super) const WINHTTP_QUERY_STATUS_CODE: u32 = 19;
pub(super) const WINHTTP_QUERY_STATUS_TEXT: u32 = 20;
pub(super) const WINHTTP_QUERY_RAW_HEADERS: u32 = 21;
pub(super) const WINHTTP_QUERY_RAW_HEADERS_CRLF: u32 = 22;
const WINHTTP_QUERY_REQUEST_METHOD: u32 = 45;
pub(super) const WINHTTP_QUERY_CUSTOM: u32 = 65535;

// URL crack / create flags
pub(super) const ICU_DECODE: u32 = 0x1000_0000;
pub(super) const ICU_ESCAPE: u32 = 0x8000_0000;

// Auth constants
pub(super) const WINHTTP_AUTH_TARGET_SERVER: u32 = 0;
pub(super) const WINHTTP_AUTH_TARGET_PROXY: u32 = 1;
pub(super) const WINHTTP_AUTH_SCHEME_BASIC: u32 = 0x0000_0001;
pub(super) const WINHTTP_AUTH_SCHEME_NTLM: u32 = 0x0000_0002;
pub(super) const WINHTTP_AUTH_SCHEME_DIGEST: u32 = 0x0000_0008;
pub(super) const WINHTTP_AUTH_SCHEME_NEGOTIATE: u32 = 0x0000_0010;

// SENSAPI / WinINet connection flags
pub(super) const NETWORK_ALIVE_LAN: u32 = 0x0000_0001;
pub(super) const INTERNET_CONNECTION_LAN: u32 = 0x0000_0002;

pub(super) const WINHTTP_INVALID_STATUS_CALLBACK: usize = usize::MAX;

const DEFAULT_MAX_REDIRECTS: u32 = 10;

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
#[derive(Clone, Copy)]
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
// Thin WinHTTP-specific string adapters over `crate::strings`
// ---------------------------------------------------------------------------

/// Write a UTF-16 string (plus NUL) into a byte buffer whose size in bytes is tracked by `buf_len`.
/// On `ERROR_INSUFFICIENT_BUFFER`, writes required byte size (including NUL) to `*buf_len`.
/// On success, writes byte size *excluding* NUL to `*buf_len`.
pub(super) unsafe fn write_wide_bytes_out_winhttp(
    s: &str,
    buf: *mut u8,
    buf_len: *mut u32,
) -> BOOL {
    if buf_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cap_bytes = strings::rd(buf_len) as usize;
    let (bytes, ok) = strings::write_wide_bytes_out(buf, cap_bytes, s);
    strings::wr(buf_len, bytes as u32);
    if ok {
        TRUE
    } else {
        crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
        FALSE
    }
}

/// Same as `write_wide_bytes_out_winhttp`, for ANSI/UTF-8 strings.
pub(super) unsafe fn write_ansi_bytes_out_winhttp(
    s: &str,
    buf: *mut u8,
    buf_len: *mut u32,
) -> BOOL {
    if buf_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cap = strings::rd(buf_len) as usize;
    let (bytes, ok) = strings::write_ansi_bytes_out(buf, cap, s);
    strings::wr(buf_len, bytes as u32);
    if ok {
        TRUE
    } else {
        crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
        FALSE
    }
}

pub(super) unsafe fn read_wide_z_list(list: *const LPCWSTR) -> Vec<String> {
    let mut out = Vec::new();
    if list.is_null() {
        return out;
    }
    let mut i = 0usize;
    loop {
        let p = strings::rd(list.add(i));
        if p.is_null() {
            break;
        }
        if let Some(s) = strings::read_wide_z_opt(p, DEFAULT_MAX_STR_LEN) {
            if !s.is_empty() {
                out.push(s);
            }
        }
        i += 1;
    }
    out
}

pub(super) unsafe fn read_ansi_z_list(list: *const LPCSTR) -> Vec<String> {
    let mut out = Vec::new();
    if list.is_null() {
        return out;
    }
    let mut i = 0usize;
    loop {
        let p = strings::rd(list.add(i));
        if p.is_null() {
            break;
        }
        if let Some(s) = strings::read_ansi_z_opt(p, DEFAULT_MAX_STR_LEN) {
            if !s.is_empty() {
                out.push(s);
            }
        }
        i += 1;
    }
    out
}

// ---------------------------------------------------------------------------
// Offline mode & network liveness helpers
// ---------------------------------------------------------------------------

pub(super) fn is_offline_mode() -> bool {
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
pub(super) fn has_active_network_interface() -> bool {
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
// Handle hierarchy & global HandleTable
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub(super) struct Timeouts {
    pub resolve_ms: i32,
    pub connect_ms: i32,
    pub send_ms: i32,
    pub receive_ms: i32,
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
pub(super) struct SessionState {
    pub user_agent: String,
    pub timeouts: Timeouts,
    pub redirect_policy: u32,
    pub max_redirects: u32,
    pub disable_flags: u32,
    pub security_flags: u32,
    pub context: usize,
    pub status_callback: usize,
}

#[derive(Clone)]
pub(super) struct ConnectionState {
    pub parent_session: usize,
    pub host: String,
    pub port: u16,
    pub user_agent: String,
    pub timeouts: Timeouts,
    pub redirect_policy: u32,
    pub max_redirects: u32,
    pub disable_flags: u32,
    pub security_flags: u32,
    pub context: usize,
    pub status_callback: usize,
}

pub(super) struct HttpResponse {
    pub version: String,
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub read_pos: usize,
}

pub(super) struct RequestState {
    pub parent_connect: usize,
    pub host: String,
    pub port: u16,
    pub is_secure: bool,
    pub verb: String,
    pub path: String,
    pub version: String,
    pub user_agent: String,
    pub headers: Vec<(String, String)>,
    pub send_headers: Vec<(String, String)>,
    pub request_body: Vec<u8>,
    pub sent: bool,
    pub timeouts: Timeouts,
    pub redirect_policy: u32,
    pub max_redirects: u32,
    pub disable_flags: u32,
    pub security_flags: u32,
    pub context: usize,
    pub status_callback: usize,
    pub effective_url: String,
    pub response: Option<HttpResponse>,
}

pub(super) enum WinHttpHandle {
    Session(SessionState),
    Connection(ConnectionState),
    Request(RequestState),
}

pub(super) static HANDLES: HandleTable<WinHttpHandle> = HandleTable::new(0x5748_0001, 1);

pub(super) fn format_url(is_secure: bool, host: &str, port: u16, path: &str) -> String {
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
// Header manipulation & HTTP-date parser
// ---------------------------------------------------------------------------

pub(super) fn apply_header_block(
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

pub(super) fn has_header(headers: &[(String, String)], name: &str) -> bool {
    headers.iter().any(|(k, _)| k.eq_ignore_ascii_case(name))
}

pub(super) fn find_header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
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

fn parse_http_date(s: &str) -> Option<SYSTEMTIME> {
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

// ---------------------------------------------------------------------------
// Core internal operations shared by WinHTTP and WinINet
// ---------------------------------------------------------------------------

pub(super) fn open_session_impl(user_agent: Option<String>) -> usize {
    let ua = user_agent
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "WinHTTP/1.1".to_string());
    HANDLES.insert(WinHttpHandle::Session(SessionState {
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

pub(super) fn connect_impl(session: usize, host: String, server_port: u16, context: usize) -> usize {
    if host.is_empty() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let parent = match HANDLES.with(session, |h| match h {
        WinHttpHandle::Session(s) => Ok(s.clone()),
        _ => Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE),
    }) {
        Some(Ok(s)) => s,
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            return 0;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return 0;
        }
    };
    HANDLES.insert(WinHttpHandle::Connection(ConnectionState {
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

pub(super) fn open_request_impl(
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
    let conn = match HANDLES.with(connect, |h| match h {
        WinHttpHandle::Connection(c) => Ok(c.clone()),
        _ => Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE),
    }) {
        Some(Ok(c)) => c,
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            return 0;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return 0;
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
    HANDLES.insert(WinHttpHandle::Request(RequestState {
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

pub(super) fn add_request_headers_impl(request: usize, raw: &str, modifiers: u32) -> BOOL {
    let res = HANDLES.with_mut(request, |h| match h {
        WinHttpHandle::Request(r) => apply_header_block(&mut r.headers, raw, modifiers),
        _ => Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE),
    });
    match res {
        Some(Ok(())) => TRUE,
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            FALSE
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub(super) fn set_timeouts_impl(
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
    let t = Timeouts {
        resolve_ms: resolve_timeout.max(0),
        connect_ms: connect_timeout.max(0),
        send_ms: send_timeout.max(0),
        receive_ms: receive_timeout.max(0),
    };
    let ok = HANDLES.with_mut(h_internet, |handle| match handle {
        WinHttpHandle::Session(s) => s.timeouts = t,
        WinHttpHandle::Connection(c) => c.timeouts = t,
        WinHttpHandle::Request(r) => r.timeouts = t,
    });
    if ok.is_some() {
        TRUE
    } else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        FALSE
    }
}

pub(super) fn set_status_callback_impl(h_internet: usize, callback: usize) -> usize {
    match HANDLES.with_mut(h_internet, |handle| {
        let slot = match handle {
            WinHttpHandle::Session(s) => &mut s.status_callback,
            WinHttpHandle::Connection(c) => &mut c.status_callback,
            WinHttpHandle::Request(r) => &mut r.status_callback,
        };
        let prev = *slot;
        *slot = callback;
        prev
    }) {
        Some(prev) => prev,
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            WINHTTP_INVALID_STATUS_CALLBACK
        }
    }
}

pub(super) unsafe fn set_credentials_impl(
    request: usize,
    auth_targets: u32,
    auth_scheme: u32,
    user_name: LPCWSTR,
    password: LPCWSTR,
) -> BOOL {
    if auth_scheme != WINHTTP_AUTH_SCHEME_BASIC {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let user = strings::read_wide_z_opt(user_name, DEFAULT_MAX_STR_LEN).unwrap_or_default();
    let pass = strings::read_wide_z_opt(password, DEFAULT_MAX_STR_LEN).unwrap_or_default();
    let encoded = net_tls::base64_encode(format!("{user}:{pass}").as_bytes());
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

pub(super) unsafe fn query_auth_schemes_impl(
    request: usize,
    supported_schemes: *mut u32,
    first_scheme: *mut u32,
    auth_target: *mut u32,
) -> BOOL {
    if supported_schemes.is_null() || first_scheme.is_null() || auth_target.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let res = HANDLES.with(request, |h| {
        let WinHttpHandle::Request(req) = h else {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
        };
        let Some(resp) = req.response.as_ref() else {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
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
            Err(ERROR_WINHTTP_HEADER_NOT_FOUND)
        } else {
            Ok((mask, first, target))
        }
    });
    match res {
        Some(Ok((mask, first, target))) => {
            strings::wr(supported_schemes, mask);
            strings::wr(first_scheme, first);
            strings::wr(auth_target, target);
            TRUE
        }
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            FALSE
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub(super) unsafe fn send_request_impl(
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
    let res = HANDLES.with_mut(request, |h| {
        let WinHttpHandle::Request(req) = h else {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
        };
        req.send_headers.clear();
        if let Some(hdrs) = extra_headers.filter(|s| !s.is_empty()) {
            apply_header_block(&mut req.send_headers, &hdrs, WINHTTP_ADDREQ_FLAG_ADD)?;
        }
        req.request_body.clear();
        let reserve = (total_len as usize).max(optional_len as usize).min(1024 * 1024);
        req.request_body.reserve(reserve);
        if optional_len > 0 {
            let slice = unsafe { std::slice::from_raw_parts(optional, optional_len as usize) };
            req.request_body.extend_from_slice(slice);
        }
        if context != 0 {
            req.context = context;
        }
        req.response = None;
        req.sent = true;
        Ok(())
    });
    match res {
        Some(Ok(())) => TRUE,
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            FALSE
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub(super) unsafe fn write_data_impl(
    request: usize,
    buffer: *const u8,
    bytes_to_write: u32,
    bytes_written: *mut u32,
) -> BOOL {
    if bytes_to_write > 0 && buffer.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let res = HANDLES.with_mut(request, |h| {
        let WinHttpHandle::Request(req) = h else {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
        };
        if !req.sent {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        }
        if bytes_to_write > 0 {
            let slice = unsafe { std::slice::from_raw_parts(buffer, bytes_to_write as usize) };
            req.request_body.extend_from_slice(slice);
        }
        Ok(())
    });
    match res {
        Some(Ok(())) => {
            if !bytes_written.is_null() {
                strings::wr(bytes_written, bytes_to_write);
            }
            TRUE
        }
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            FALSE
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub(super) fn receive_response_impl(request: usize) -> BOOL {
    let snap = match HANDLES.with(request, |h| {
        let WinHttpHandle::Request(req) = h else {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
        };
        if !req.sent {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        }
        let mut combined_headers = req.headers.clone();
        combined_headers.extend(req.send_headers.iter().cloned());
        Ok(super::http::RequestSnapshot {
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
        })
    }) {
        Some(Ok(s)) => s,
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return FALSE;
        }
    };

    // Perform network I/O without holding the global handle table lock.
    let (resp, final_url) = match super::http::execute_http_request(snap) {
        Ok(res) => res,
        Err(e) => {
            crate::teb::set_last_error(e);
            return FALSE;
        }
    };

    let updated = HANDLES.with_mut(request, |h| {
        if let WinHttpHandle::Request(req) = h {
            req.effective_url = final_url;
            req.response = Some(resp);
            true
        } else {
            false
        }
    });
    if updated == Some(true) {
        TRUE
    } else {
        crate::teb::set_last_error(ERROR_WINHTTP_OPERATION_CANCELLED);
        FALSE
    }
}

pub(super) unsafe fn query_data_available_impl(
    request: usize,
    bytes_available: *mut u32,
) -> BOOL {
    let res = HANDLES.with(request, |h| {
        let WinHttpHandle::Request(req) = h else {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
        };
        let Some(resp) = req.response.as_ref() else {
            return Err(ERROR_WINHTTP_INCORRECT_HANDLE_STATE);
        };
        let rem = resp.body.len().saturating_sub(resp.read_pos).min(u32::MAX as usize) as u32;
        Ok(rem)
    });
    match res {
        Some(Ok(rem)) => {
            if !bytes_available.is_null() {
                strings::wr(bytes_available, rem);
            }
            TRUE
        }
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            FALSE
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub(super) unsafe fn read_data_impl(
    request: usize,
    buffer: *mut u8,
    bytes_to_read: u32,
    bytes_read: *mut u32,
) -> BOOL {
    if bytes_to_read > 0 && buffer.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let res = HANDLES.with_mut(request, |h| {
        let WinHttpHandle::Request(req) = h else {
            return Err((ERROR_WINHTTP_INCORRECT_HANDLE_TYPE, false));
        };
        let Some(resp) = req.response.as_mut() else {
            return Err((ERROR_WINHTTP_INCORRECT_HANDLE_STATE, true));
        };
        let avail = resp.body.len().saturating_sub(resp.read_pos);
        let n = avail.min(bytes_to_read as usize);
        if n > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(resp.body[resp.read_pos..].as_ptr(), buffer, n);
            }
            resp.read_pos += n;
        }
        Ok(n as u32)
    });
    match res {
        Some(Ok(n)) => {
            if !bytes_read.is_null() {
                strings::wr(bytes_read, n);
            }
            TRUE
        }
        Some(Err((e, zero_out))) => {
            if zero_out && !bytes_read.is_null() {
                strings::wr(bytes_read, 0);
            }
            crate::teb::set_last_error(e);
            FALSE
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub(super) unsafe fn query_headers_impl(
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
        strings::rd(index) as usize
    };

    let Some(res) = HANDLES.with(request, |h| -> BOOL {
        let WinHttpHandle::Request(req) = h else {
            crate::teb::set_last_error(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE);
            return FALSE;
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
                let cap = unsafe { strings::rd(buffer_len) } as usize;
                if buffer.is_null() || cap < bytes.len() {
                    unsafe { strings::wr(buffer_len, bytes.len() as u32) };
                    crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
                    return FALSE;
                }
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, bytes.len());
                    strings::wr(buffer_len, (bytes.len() - 1) as u32);
                }
                return TRUE;
            } else {
                let mut units = Vec::new();
                for l in &lines {
                    units.extend(l.encode_utf16());
                    units.push(0);
                }
                units.push(0);
                let needed_bytes = units.len() * 2;
                let cap_bytes = unsafe { strings::rd(buffer_len) } as usize;
                if buffer.is_null() || cap_bytes < needed_bytes {
                    unsafe { strings::wr(buffer_len, needed_bytes as u32) };
                    crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
                    return FALSE;
                }
                unsafe {
                    let wdst = buffer as *mut u16;
                    for (i, &u) in units.iter().enumerate() {
                        strings::wr(wdst.add(i), u);
                    }
                    strings::wr(buffer_len, ((units.len() - 1) * 2) as u32);
                }
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
                    match custom_name.as_deref().filter(|s| !s.trim().is_empty()) {
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
            let cap = unsafe { strings::rd(buffer_len) } as usize;
            if buffer.is_null() || cap < 4 {
                unsafe { strings::wr(buffer_len, 4) };
                crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            unsafe {
                strings::wr(buffer as *mut u32, num);
                strings::wr(buffer_len, 4);
            }
            TRUE
        } else if flag_number64 {
            let Ok(num) = value.trim().parse::<u64>() else {
                crate::teb::set_last_error(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
                return FALSE;
            };
            let cap = unsafe { strings::rd(buffer_len) } as usize;
            if buffer.is_null() || cap < 8 {
                unsafe { strings::wr(buffer_len, 8) };
                crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            unsafe {
                strings::wr(buffer as *mut u64, num);
                strings::wr(buffer_len, 8);
            }
            TRUE
        } else if flag_systemtime {
            let Some(st) = parse_http_date(&value) else {
                crate::teb::set_last_error(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
                return FALSE;
            };
            let needed = std::mem::size_of::<SYSTEMTIME>();
            let cap = unsafe { strings::rd(buffer_len) } as usize;
            if buffer.is_null() || cap < needed {
                unsafe { strings::wr(buffer_len, needed as u32) };
                crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
                return FALSE;
            }
            unsafe {
                strings::wr(buffer as *mut SYSTEMTIME, st);
                strings::wr(buffer_len, needed as u32);
            }
            TRUE
        } else if ansi {
            unsafe { write_ansi_bytes_out_winhttp(&value, buffer, buffer_len) }
        } else {
            unsafe { write_wide_bytes_out_winhttp(&value, buffer, buffer_len) }
        };

        if ok == TRUE && advance_index && !index.is_null() {
            unsafe { strings::wr(index, (start_idx + 1) as u32) };
        }
        ok
    }) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    res
}

pub(super) unsafe fn set_option_impl(
    h_internet: usize,
    option: u32,
    buffer: *const u8,
    buffer_len: u32,
    ansi: bool,
) -> BOOL {
    if h_internet == 0 {
        log::debug!("WinHttpSetOption(global, opt={option}) -> TRUE");
        return TRUE;
    }

    let read_dword = || -> Result<u32, u32> {
        if buffer.is_null() || buffer_len < 4 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        Ok(unsafe { strings::rd(buffer as *const u32) })
    };

    let read_ptr_val = || -> Result<usize, u32> {
        if buffer.is_null() || (buffer_len as usize) < std::mem::size_of::<usize>() {
            return Err(ERROR_INVALID_PARAMETER);
        }
        Ok(unsafe { strings::rd(buffer as *const usize) })
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

    let Some(res) = HANDLES.with_mut(h_internet, |handle| match option {
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
                unsafe { strings::read_ansi_opt_len(buffer, buffer_len as i32) }.unwrap_or_default()
            } else {
                let wlen = if buffer_len == u32::MAX {
                    -1i32
                } else {
                    (buffer_len / 2) as i32
                };
                unsafe { strings::read_wide_opt_len(buffer as LPCWSTR, wlen) }.unwrap_or_default()
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
    }) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    res
}

pub(super) unsafe fn query_option_impl(
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

    let write_dword_out = |val: u32| -> BOOL {
        let cap = unsafe { strings::rd(buffer_len) } as usize;
        if buffer.is_null() || cap < 4 {
            unsafe { strings::wr(buffer_len, 4) };
            crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        unsafe {
            strings::wr(buffer as *mut u32, val);
            strings::wr(buffer_len, 4);
        }
        TRUE
    };

    let write_usize_out = |val: usize| -> BOOL {
        let needed = std::mem::size_of::<usize>();
        let cap = unsafe { strings::rd(buffer_len) } as usize;
        if buffer.is_null() || cap < needed {
            unsafe { strings::wr(buffer_len, needed as u32) };
            crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
            return FALSE;
        }
        unsafe {
            strings::wr(buffer as *mut usize, val);
            strings::wr(buffer_len, needed as u32);
        }
        TRUE
    };

    let Some(res) = HANDLES.with(h_internet, |handle| {
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
                        unsafe { write_ansi_bytes_out_winhttp(&r.effective_url, buffer, buffer_len) }
                    } else {
                        unsafe { write_wide_bytes_out_winhttp(&r.effective_url, buffer, buffer_len) }
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
                    unsafe { write_ansi_bytes_out_winhttp(ua, buffer, buffer_len) }
                } else {
                    unsafe { write_wide_bytes_out_winhttp(ua, buffer, buffer_len) }
                }
            }
            WINHTTP_OPTION_HTTP_PROTOCOL_USED => write_dword_out(0), // HTTP/1.1
            _ => {
                crate::teb::set_last_error(ERROR_WINHTTP_INVALID_OPTION);
                FALSE
            }
        }
    }) else {
        crate::teb::set_last_error(ERROR_INVALID_HANDLE);
        return FALSE;
    };
    res
}

pub(super) fn translate_wininet_request_flags(flags: u32) -> (u32, u32) {
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

pub(super) fn map_wininet_option(option: u32) -> u32 {
    match option {
        INTERNET_OPTION_CONNECT_TIMEOUT => WINHTTP_OPTION_CONNECT_TIMEOUT,
        INTERNET_OPTION_CONNECT_RETRIES => WINHTTP_OPTION_CONNECT_RETRIES,
        other => other,
    }
}

pub(super) fn internet_open_url_impl(
    session: usize,
    url_str: &str,
    headers: Option<String>,
    flags: u32,
    context: usize,
) -> usize {
    let parsed = match super::url::parse_url_parts(url_str) {
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
    let parent = match HANDLES.with(session, |h| match h {
        WinHttpHandle::Session(s) => Ok(s.clone()),
        _ => Err(ERROR_WINHTTP_INCORRECT_HANDLE_TYPE),
    }) {
        Some(Ok(s)) => s,
        Some(Err(e)) => {
            crate::teb::set_last_error(e);
            return 0;
        }
        None => {
            crate::teb::set_last_error(ERROR_INVALID_HANDLE);
            return 0;
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
    let req_handle = HANDLES.insert(WinHttpHandle::Request(RequestState {
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
        HANDLES.remove(req_handle);
        return 0;
    }
    req_handle
}
