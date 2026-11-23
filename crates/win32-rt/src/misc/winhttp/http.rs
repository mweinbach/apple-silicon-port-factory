//! HTTP/1.1 + HTTPS transport, redirect handling, and wire framing for WinHTTP / WinINet,
//! backed by shared [`crate::net_tls`] primitives.

use super::state::{
    find_header, format_url, has_header, is_offline_mode, HttpResponse, Timeouts,
    ERROR_WINHTTP_CANNOT_CONNECT, ERROR_WINHTTP_CONNECTION_ERROR,
    ERROR_WINHTTP_INVALID_SERVER_RESPONSE, ERROR_WINHTTP_INVALID_URL,
    ERROR_WINHTTP_NAME_NOT_RESOLVED, ERROR_WINHTTP_REDIRECT_FAILED,
    ERROR_WINHTTP_SECURE_CERT_CN_INVALID, ERROR_WINHTTP_SECURE_CERT_DATE_INVALID,
    ERROR_WINHTTP_SECURE_CHANNEL_ERROR, ERROR_WINHTTP_SECURE_INVALID_CA, ERROR_WINHTTP_TIMEOUT,
    INTERNET_DEFAULT_HTTPS_PORT, INTERNET_DEFAULT_HTTP_PORT, WINHTTP_DISABLE_REDIRECTS,
    WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP, WINHTTP_OPTION_REDIRECT_POLICY_NEVER,
};
use super::url::parse_url_parts;
use crate::net_tls::{self, HttpStream, TlsVerifyPolicy};
use rustls::CertificateError;
use std::io::{self, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

const MAX_HEADER_BYTES: usize = 512 * 1024;

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

fn map_chunked_error(err: &io::Error) -> u32 {
    if err.get_ref().and_then(|e| e.downcast_ref::<rustls::Error>()).is_some() {
        return map_io_error(err);
    }
    match err.kind() {
        io::ErrorKind::InvalidData | io::ErrorKind::UnexpectedEof => {
            ERROR_WINHTTP_INVALID_SERVER_RESPONSE
        }
        _ => map_io_error(err),
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
        return Ok(HttpStream::plain(tcp));
    }

    let policy = TlsVerifyPolicy::from_winhttp_flags(security_flags);
    let tls_cfg = net_tls::build_tls_config(policy, &[b"http/1.1"]);
    HttpStream::connect_tls(tcp, clean_host, tls_cfg).map_err(|e| {
        if e.kind() == io::ErrorKind::InvalidInput {
            ERROR_WINHTTP_INVALID_URL
        } else {
            ERROR_WINHTTP_SECURE_CHANNEL_ERROR
        }
    })
}

fn read_response_header_block(reader: &mut BufReader<HttpStream>) -> Result<Vec<u8>, u32> {
    let mut block = Vec::with_capacity(1024);
    let mut line = Vec::with_capacity(256);
    loop {
        let n = net_tls::read_line_limited(reader, &mut line, MAX_HEADER_BYTES).map_err(|e| {
            if e.kind() == io::ErrorKind::InvalidData {
                ERROR_WINHTTP_INVALID_SERVER_RESPONSE
            } else {
                map_io_error(&e)
            }
        })?;
        if n == 0 {
            return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
        }
        let trimmed = line
            .strip_suffix(b"\r\n")
            .or_else(|| line.strip_suffix(b"\n"))
            .unwrap_or(&line);
        if trimmed.is_empty() {
            if block.is_empty() {
                return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
            }
            return Ok(block);
        }
        if block.len() + line.len() > MAX_HEADER_BYTES {
            return Err(ERROR_WINHTTP_INVALID_SERVER_RESPONSE);
        }
        block.extend_from_slice(&line);
    }
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

fn read_exact_bytes(reader: &mut BufReader<HttpStream>, len: usize) -> Result<Vec<u8>, u32> {
    let mut out = Vec::with_capacity(len.min(64 * 1024));
    let mut remaining = len;
    let mut scratch = [0u8; 8192];
    while remaining > 0 {
        let want = remaining.min(scratch.len());
        match reader.read(&mut scratch[..want]) {
            Ok(0) => return Err(ERROR_WINHTTP_CONNECTION_ERROR),
            Ok(n) => {
                out.extend_from_slice(&scratch[..n]);
                remaining -= n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(ref e) => return Err(map_io_error(e)),
        }
    }
    Ok(out)
}

fn read_to_eof(reader: &mut BufReader<HttpStream>) -> Result<Vec<u8>, u32> {
    let is_tls = reader.get_ref().is_tls();
    let mut out = Vec::new();
    let mut scratch = [0u8; 8192];
    loop {
        match reader.read(&mut scratch) {
            Ok(0) => return Ok(out),
            Ok(n) => out.extend_from_slice(&scratch[..n]),
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(ref e) if is_tls && e.kind() == io::ErrorKind::UnexpectedEof => return Ok(out),
            Err(ref e) => return Err(map_io_error(e)),
        }
    }
}

fn sanitize_header_token(s: &str) -> String {
    s.replace(['\r', '\n'], " ")
}

pub(super) struct RequestSnapshot {
    pub host: String,
    pub port: u16,
    pub is_secure: bool,
    pub verb: String,
    pub path: String,
    pub version: String,
    pub user_agent: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub timeouts: Timeouts,
    pub redirect_policy: u32,
    pub max_redirects: u32,
    pub disable_flags: u32,
    pub security_flags: u32,
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

    let mut reader = BufReader::with_capacity(8192, stream);
    let (resp_ver, status_code, status_text, resp_headers) = loop {
        let block = read_response_header_block(&mut reader)?;
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
    } else if net_tls::is_chunked_encoding(&resp_headers) {
        net_tls::read_chunked_body(&mut reader, net_tls::DEFAULT_MAX_CHUNK_SIZE)
            .map_err(|e| map_chunked_error(&e))?
    } else if let Some(cl) = net_tls::parse_content_length(&resp_headers) {
        read_exact_bytes(&mut reader, cl)?
    } else {
        read_to_eof(&mut reader)?
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

pub(super) fn execute_http_request(snap: RequestSnapshot) -> Result<(HttpResponse, String), u32> {
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
    let base_scheme = if cur_secure { "https" } else { "http" };
    let resolved = net_tls::resolve_redirect_url(
        base_scheme,
        cur_host,
        Some(cur_port),
        cur_path,
        location,
    )
    .ok_or(ERROR_WINHTTP_INVALID_URL)?;
    let parsed = parse_url_parts(&resolved)?;
    let scheme_str = &resolved[parsed.scheme_start..parsed.scheme_end];
    let secure = scheme_str.eq_ignore_ascii_case("https");
    let host = resolved[parsed.host_start..parsed.host_end].to_string();
    let port = parsed.port;
    let path_part = &resolved[parsed.path_start..parsed.path_end];
    let extra_part = &resolved[parsed.extra_start..parsed.extra_end];
    let mut full_path = if path_part.is_empty() {
        "/".to_string()
    } else {
        path_part.to_string()
    };
    full_path.push_str(extra_part);
    Ok((secure, host, port, full_path))
}
