//! URL parsing, cracking, and formatting (`WinHttpCrackUrl`, `WinHttpCreateUrl`,
//! `InternetCrackUrlW`, `InternetCrackUrlA`).

use super::state::{
    ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER, ERROR_WINHTTP_INVALID_URL,
    ERROR_WINHTTP_UNRECOGNIZED_SCHEME, ICU_DECODE, ICU_ESCAPE, INTERNET_DEFAULT_HTTPS_PORT,
    INTERNET_DEFAULT_HTTP_PORT, URL_COMPONENTSA, URL_COMPONENTSW, WINHTTP_SCHEME_HTTP,
    WINHTTP_SCHEME_HTTPS, WININET_SCHEME_HTTP, WININET_SCHEME_HTTPS,
};
use crate::strings;
use crate::types::*;

pub(super) struct ParsedUrl {
    pub scheme_start: usize,
    pub scheme_end: usize,
    pub is_https: bool,
    pub user_start: usize,
    pub user_end: usize,
    pub pass_start: usize,
    pub pass_end: usize,
    pub host_start: usize,
    pub host_end: usize,
    pub port: u16,
    pub path_start: usize,
    pub path_end: usize,
    pub extra_start: usize,
    pub extra_end: usize,
}

pub(super) fn parse_url_parts(url: &str) -> Result<ParsedUrl, u32> {
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
    let auth_rel_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
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
        if b <= 0x20
            || b >= 0x7F
            || matches!(
                b,
                b'<' | b'>' | b'"' | b'{' | b'}' | b'|' | b'\\' | b'^' | b'`'
            )
        {
            out.push('%');
            let hi = b >> 4;
            let lo = b & 0x0F;
            out.push(char::from(if hi < 10 {
                b'0' + hi
            } else {
                b'A' + hi - 10
            }));
            out.push(char::from(if lo < 10 {
                b'0' + lo
            } else {
                b'A' + lo - 10
            }));
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
    let needed_units = val.encode_utf16().count();
    let cap = *out_len as usize;
    if cap <= needed_units {
        *out_len = (needed_units + 1) as u32;
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    let written = strings::write_wide_z(*out_ptr, cap, &val);
    *out_len = written as u32;
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
    let cap = *out_len as usize;
    if cap <= val.len() {
        *out_len = (val.len() + 1) as u32;
        return Err(ERROR_INSUFFICIENT_BUFFER);
    }
    let written = strings::write_ansi_z(*out_ptr, cap, &val);
    *out_len = written as u32;
    Ok(())
}

pub(super) unsafe fn crack_url_w_impl(
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
    let mut comp = strings::rd(components);
    if (comp.dw_struct_size as usize) < std::mem::size_of::<URL_COMPONENTSW>() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(url_str) = strings::read_wide_zero_term(url, url_len as usize) else {
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

    strings::wr(components, comp);
    if let Some(e) = first_err {
        crate::teb::set_last_error(e);
        return FALSE;
    }
    TRUE
}

pub(super) unsafe fn crack_url_a_impl(
    url: LPCSTR,
    url_len: u32,
    flags: u32,
    components: *mut URL_COMPONENTSA,
) -> BOOL {
    if url.is_null() || components.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut comp = strings::rd(components);
    if (comp.dw_struct_size as usize) < std::mem::size_of::<URL_COMPONENTSA>() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(url_str) = strings::read_ansi_zero_term(url, url_len as usize) else {
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

    strings::wr(components, comp);
    if let Some(e) = first_err {
        crate::teb::set_last_error(e);
        return FALSE;
    }
    TRUE
}

pub(super) unsafe fn create_url_w_impl(
    components: *const URL_COMPONENTSW,
    flags: u32,
    url: LPWSTR,
    url_len: *mut u32,
) -> BOOL {
    if components.is_null() || url_len.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let comp = strings::rd(components);
    if (comp.dw_struct_size as usize) < std::mem::size_of::<URL_COMPONENTSW>() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let scheme = match strings::read_wide_zero_term(comp.lpsz_scheme, comp.dw_scheme_length as usize)
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
    let user = strings::read_wide_zero_term(comp.lpsz_user_name, comp.dw_user_name_length as usize)
        .unwrap_or_default();
    let pass = strings::read_wide_zero_term(comp.lpsz_password, comp.dw_password_length as usize)
        .unwrap_or_default();
    let host = strings::read_wide_zero_term(comp.lpsz_host_name, comp.dw_host_name_length as usize)
        .unwrap_or_default();
    let path = strings::read_wide_zero_term(comp.lpsz_url_path, comp.dw_url_path_length as usize)
        .unwrap_or_default();
    let extra =
        strings::read_wide_zero_term(comp.lpsz_extra_info, comp.dw_extra_info_length as usize)
            .unwrap_or_default();

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

    let needed_units = built.encode_utf16().count();
    let needed = needed_units + 1;
    let cap = strings::rd(url_len) as usize;
    if url.is_null() || cap < needed {
        strings::wr(url_len, needed as u32);
        crate::teb::set_last_error(ERROR_INSUFFICIENT_BUFFER);
        return FALSE;
    }
    let written = strings::write_wide_z(url, cap, &built);
    strings::wr(url_len, written as u32);
    TRUE
}
