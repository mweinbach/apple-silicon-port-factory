//! HTTP/1.1 + TLS transport engine and streaming content decoding for `libcurl.dll`.

use super::easy::{
    apply_head_meta, invoke_header_callback, invoke_write_callback, CallbackConfig, CurlEasy,
    HeadMeta, TransferOutcome, TransferRequest, CURLE_ABORTED_BY_CALLBACK,
    CURLE_BAD_CONTENT_ENCODING, CURLE_COULDNT_CONNECT, CURLE_COULDNT_RESOLVE_HOST,
    CURLE_GOT_NOTHING, CURLE_HTTP_RETURNED_ERROR, CURLE_OK, CURLE_OPERATION_TIMEDOUT,
    CURLE_PARTIAL_FILE, CURLE_PEER_FAILED_VERIFICATION, CURLE_RECV_ERROR, CURLE_SEND_ERROR,
    CURLE_SSL_CONNECT_ERROR, CURLE_TOO_MANY_REDIRECTS, CURLE_UNSUPPORTED_PROTOCOL,
    CURLE_URL_MALFORMAT, CURLE_WEIRD_SERVER_REPLY, CURL_REDIR_POST_301, CURL_REDIR_POST_302,
    CURL_REDIR_POST_303,
};
use crate::misc::zlib1::{ZlibStream, Z_DATA_ERROR};
use crate::net_tls::{self, HttpStream, TlsVerifyPolicy};
use rustls::pki_types::ServerName;
use rustls::ClientConnection;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Offline check, URL parsing, redirect resolution, streaming zlib inflater
// ---------------------------------------------------------------------------

fn is_offline() -> bool {
    std::env::var("PORT_OFFLINE")
        .ok()
        .is_some_and(|v| matches!(v.trim(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedUrl {
    scheme: String,
    userinfo: Option<String>,
    host: String,
    is_ipv6: bool,
    port: Option<u16>,
    path_and_query: String,
}

impl ParsedUrl {
    fn default_port(&self) -> u16 {
        if self.scheme == "https" {
            443
        } else {
            80
        }
    }

    fn effective_port(&self, port_override: u16) -> u16 {
        if port_override != 0 {
            port_override
        } else {
            self.port.unwrap_or_else(|| self.default_port())
        }
    }

    fn formatted_host(&self) -> String {
        if self.is_ipv6 {
            format!("[{}]", self.host)
        } else {
            self.host.clone()
        }
    }

    fn host_header(&self, port_override: u16) -> String {
        let port = self.effective_port(port_override);
        let host = self.formatted_host();
        if port == self.default_port() {
            host
        } else {
            format!("{host}:{port}")
        }
    }

    fn authority_for_redirect(&self) -> String {
        let host = self.formatted_host();
        match self.port {
            Some(p) if p != self.default_port() => format!("{host}:{p}"),
            _ => host,
        }
    }

    fn socket_addr_str(&self, port_override: u16) -> String {
        format!(
            "{}:{}",
            self.formatted_host(),
            self.effective_port(port_override)
        )
    }

    fn origin_key(&self, port_override: u16) -> (String, String, u16) {
        (
            self.scheme.clone(),
            self.host.to_ascii_lowercase(),
            self.effective_port(port_override),
        )
    }

    fn to_url_string(&self) -> String {
        format!(
            "{}://{}{}",
            self.scheme,
            self.authority_for_redirect(),
            self.path_and_query
        )
    }
}

fn parse_url(raw: &str) -> Result<ParsedUrl, (i32, String)> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err((CURLE_URL_MALFORMAT, "Empty URL".to_string()));
    }
    let without_frag = trimmed.split('#').next().unwrap_or(trimmed);
    let (scheme, rest) = if let Some((s, r)) = without_frag.split_once("://") {
        let sl = s.to_ascii_lowercase();
        if sl != "http" && sl != "https" {
            return Err((
                CURLE_UNSUPPORTED_PROTOCOL,
                format!("Unsupported protocol: {sl}"),
            ));
        }
        (sl, r)
    } else {
        ("http".to_string(), without_frag)
    };

    let (authority, path_and_query) = match rest.find(['/', '?']) {
        Some(idx) => {
            let auth = &rest[..idx];
            let pq = &rest[idx..];
            if pq.starts_with('?') {
                (auth, format!("/{pq}"))
            } else {
                (auth, pq.to_string())
            }
        }
        None => (rest, "/".to_string()),
    };

    if authority.is_empty() {
        return Err((CURLE_URL_MALFORMAT, format!("Missing host in URL: {raw}")));
    }

    let (userinfo, hostport) = match authority.rfind('@') {
        Some(idx) => (Some(authority[..idx].to_string()), &authority[idx + 1..]),
        None => (None, authority),
    };

    let (host, is_ipv6, port) = if let Some(after_open) = hostport.strip_prefix('[') {
        let Some(close_idx) = after_open.find(']') else {
            return Err((CURLE_URL_MALFORMAT, format!("Unclosed IPv6 literal: {raw}")));
        };
        let ipv6 = &after_open[..close_idx];
        let tail = &after_open[close_idx + 1..];
        let port = if tail.is_empty() {
            None
        } else if let Some(pstr) = tail.strip_prefix(':') {
            let p = pstr
                .parse::<u16>()
                .map_err(|_| (CURLE_URL_MALFORMAT, format!("Invalid port in URL: {raw}")))?;
            Some(p)
        } else {
            return Err((CURLE_URL_MALFORMAT, format!("Malformed IPv6 host: {raw}")));
        };
        (ipv6.to_string(), true, port)
    } else if let Some((h, pstr)) = hostport.rsplit_once(':') {
        let p = pstr
            .parse::<u16>()
            .map_err(|_| (CURLE_URL_MALFORMAT, format!("Invalid port in URL: {raw}")))?;
        (h.to_string(), false, Some(p))
    } else {
        (hostport.to_string(), false, None)
    };

    if host.is_empty() {
        return Err((CURLE_URL_MALFORMAT, format!("Empty host in URL: {raw}")));
    }

    Ok(ParsedUrl {
        scheme,
        userinfo,
        host,
        is_ipv6,
        port,
        path_and_query,
    })
}

fn resolve_redirect(base: &ParsedUrl, location: &str) -> Result<ParsedUrl, (i32, String)> {
    let Some(resolved) = net_tls::resolve_redirect_url(
        &base.scheme,
        &base.host,
        base.port,
        &base.path_and_query,
        location,
    ) else {
        return Err((CURLE_URL_MALFORMAT, "Empty redirect Location".to_string()));
    };
    parse_url(&resolved)
}

// Streaming gzip/deflate decoder backed by `zlib1::ZlibStream`.
struct StreamingInflater {
    stream: ZlibStream,
    finished: bool,
    produced_any: bool,
    raw_deflate_fallback: bool,
}

impl StreamingInflater {
    fn new(encoding: &str) -> Result<Self, i32> {
        let (window_bits, raw_deflate_fallback) = match encoding {
            "gzip" | "x-gzip" => (15 + 16, false),
            "deflate" => (15, true),
            _ => return Err(CURLE_BAD_CONTENT_ENCODING),
        };
        let stream = ZlibStream::new_inflate(window_bits).map_err(|_| CURLE_BAD_CONTENT_ENCODING)?;
        Ok(Self {
            stream,
            finished: false,
            produced_any: false,
            raw_deflate_fallback,
        })
    }

    fn decompress(&mut self, input: &[u8], out: &mut Vec<u8>) -> Result<(), i32> {
        if input.is_empty() || self.finished {
            return Ok(());
        }
        let out_start = out.len();
        match self.stream.inflate_chunk(input, out) {
            Ok(done) => {
                if out.len() > out_start {
                    self.produced_any = true;
                }
                if done {
                    self.finished = true;
                }
                Ok(())
            }
            Err(Z_DATA_ERROR)
                if self.raw_deflate_fallback && !self.produced_any && out.len() == out_start =>
            {
                // Retry as raw RFC 1951 deflate (-15).
                self.raw_deflate_fallback = false;
                self.stream =
                    ZlibStream::new_inflate(-15).map_err(|_| CURLE_BAD_CONTENT_ENCODING)?;
                let done = self
                    .stream
                    .inflate_chunk(input, out)
                    .map_err(|_| CURLE_BAD_CONTENT_ENCODING)?;
                if out.len() > out_start {
                    self.produced_any = true;
                }
                if done {
                    self.finished = true;
                }
                Ok(())
            }
            Err(_) => Err(CURLE_BAD_CONTENT_ENCODING),
        }
    }
}

// ---------------------------------------------------------------------------
// HTTP/1.1 + TLS transport engine
// ---------------------------------------------------------------------------

pub(super) trait TransferSink {
    fn is_cancelled(&self) -> bool;
    fn cancel_token(&self) -> Option<Arc<AtomicBool>> {
        None
    }
    fn on_head(&mut self, meta: HeadMeta);
    fn on_header_line(&mut self, line: Vec<u8>) -> Result<(), i32>;
    fn on_body_chunk(&mut self, data: Vec<u8>) -> Result<(), i32>;
}

pub(super) struct DirectSink {
    pub(super) easy: Arc<Mutex<CurlEasy>>,
    pub(super) cb: CallbackConfig,
}

impl TransferSink for DirectSink {
    fn is_cancelled(&self) -> bool {
        false
    }

    fn on_head(&mut self, meta: HeadMeta) {
        apply_head_meta(&self.easy, meta);
    }

    fn on_header_line(&mut self, line: Vec<u8>) -> Result<(), i32> {
        unsafe { invoke_header_callback(self.cb, &line) }
    }

    fn on_body_chunk(&mut self, data: Vec<u8>) -> Result<(), i32> {
        unsafe { invoke_write_callback(self.cb, &data) }
    }
}

fn map_io_err(e: &io::Error, default_code: i32) -> (i32, String, i32) {
    let os_err = e.raw_os_error().unwrap_or(0);
    let code = match e.kind() {
        io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => CURLE_OPERATION_TIMEDOUT,
        io::ErrorKind::ConnectionRefused
        | io::ErrorKind::ConnectionAborted
        | io::ErrorKind::NetworkUnreachable
        | io::ErrorKind::HostUnreachable => CURLE_COULDNT_CONNECT,
        _ => default_code,
    };
    (code, e.to_string(), os_err)
}

fn remaining_timeout(deadline: Option<Instant>) -> Result<Option<Duration>, (i32, String, i32)> {
    let Some(dl) = deadline else {
        return Ok(None);
    };
    let now = Instant::now();
    if now >= dl {
        return Err((
            CURLE_OPERATION_TIMEDOUT,
            "Operation timed out".to_string(),
            0,
        ));
    }
    Ok(Some(dl.saturating_duration_since(now)))
}

struct DeadlineBufReader<'a> {
    inner: &'a mut BufReader<HttpStream>,
    deadline: Option<Instant>,
    cancel: Option<Arc<AtomicBool>>,
    cancelled: bool,
}

impl DeadlineBufReader<'_> {
    fn prepare_io(&mut self) -> io::Result<()> {
        if self
            .cancel
            .as_ref()
            .is_some_and(|c| c.load(Ordering::Acquire))
        {
            self.cancelled = true;
            return Err(io::Error::other("Transfer cancelled"));
        }
        let rem = remaining_timeout(self.deadline)
            .map_err(|(_, msg, _)| io::Error::new(io::ErrorKind::TimedOut, msg))?;
        let _ = self.inner.get_ref().set_timeouts(rem, rem);
        Ok(())
    }
}

impl Read for DeadlineBufReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.prepare_io()?;
        self.inner.read(buf)
    }
}

impl BufRead for DeadlineBufReader<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.prepare_io()?;
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
    }
}

fn build_http_request(
    req: &TransferRequest,
    url: &ParsedUrl,
    port_override: u16,
    method: &str,
    body: &[u8],
    referer_override: Option<&str>,
    include_credentials: bool,
) -> Vec<u8> {
    let mut suppressed: Vec<String> = Vec::new();
    let mut provided: Vec<String> = Vec::new();
    let mut custom_lines: Vec<String> = Vec::new();

    for raw in &req.custom_headers {
        let trimmed = raw.trim_end_matches(['\r', '\n']);
        if let Some((name, val)) = trimmed.split_once(':') {
            let name_trim = name.trim();
            if name_trim.is_empty() {
                continue;
            }
            let lower = name_trim.to_ascii_lowercase();
            let val_trim = val.trim();
            if val_trim.is_empty() {
                suppressed.push(lower);
            } else {
                provided.push(lower);
                custom_lines.push(format!("{name_trim}: {val_trim}"));
            }
        } else if let Some(name) = trimmed.strip_suffix(';') {
            let name_trim = name.trim();
            if !name_trim.is_empty() {
                provided.push(name_trim.to_ascii_lowercase());
                custom_lines.push(format!("{name_trim}:"));
            }
        }
    }

    let has_custom =
        |k: &str| provided.iter().any(|x| x == k) || suppressed.iter().any(|x| x == k);

    let mut wire = Vec::with_capacity(512 + body.len());
    let _ = write!(wire, "{method} {} HTTP/1.1\r\n", url.path_and_query);

    if !has_custom("host") {
        let _ = write!(wire, "Host: {}\r\n", url.host_header(port_override));
    }
    if !has_custom("user-agent") {
        if let Some(ref ua) = req.user_agent {
            let _ = write!(wire, "User-Agent: {ua}\r\n");
        }
    }
    if !has_custom("accept") {
        wire.extend_from_slice(b"Accept: */*\r\n");
    }
    if !has_custom("accept-encoding") {
        if let Some(ref enc) = req.accept_encoding {
            let val = if enc.is_empty() {
                "gzip, deflate"
            } else {
                enc.as_str()
            };
            let _ = write!(wire, "Accept-Encoding: {val}\r\n");
        }
    }
    if !has_custom("referer") {
        if let Some(r) = referer_override.or(req.referer.as_deref()) {
            let _ = write!(wire, "Referer: {r}\r\n");
        }
    }
    if include_credentials && !has_custom("cookie") {
        if let Some(ref c) = req.cookie {
            let _ = write!(wire, "Cookie: {c}\r\n");
        }
    }
    if !has_custom("range") {
        if let Some(ref rng) = req.range {
            let _ = write!(wire, "Range: bytes={rng}\r\n");
        }
    }
    if include_credentials && !has_custom("authorization") {
        if let Some(creds) = req.basic_auth.as_deref().or(url.userinfo.as_deref()) {
            let encoded = net_tls::base64_encode(creds.as_bytes());
            let _ = write!(wire, "Authorization: Basic {encoded}\r\n");
        }
    }
    if !has_custom("connection") {
        wire.extend_from_slice(b"Connection: close\r\n");
    }
    if req.is_post_default_ct && method == "POST" && !has_custom("content-type") {
        wire.extend_from_slice(b"Content-Type: application/x-www-form-urlencoded\r\n");
    }
    if (!body.is_empty() || matches!(method, "POST" | "PUT" | "PATCH"))
        && !has_custom("content-length")
        && !has_custom("transfer-encoding")
    {
        let _ = write!(wire, "Content-Length: {}\r\n", body.len());
    }

    for line in &custom_lines {
        if !include_credentials {
            if let Some((name, _)) = line.split_once(':') {
                let lower = name.trim().to_ascii_lowercase();
                if lower == "authorization" || lower == "cookie" {
                    continue;
                }
            }
        }
        wire.extend_from_slice(line.as_bytes());
        wire.extend_from_slice(b"\r\n");
    }
    wire.extend_from_slice(b"\r\n");
    wire.extend_from_slice(body);
    wire
}

struct ParsedResponseHead {
    status_code: i32,
    raw_lines: Vec<Vec<u8>>,
    header_bytes: usize,
    content_length: Option<i64>,
    chunked: bool,
    content_type: Option<String>,
    content_encoding: Option<String>,
    location: Option<String>,
}

fn read_response_head<R: BufRead>(reader: &mut R) -> Result<ParsedResponseHead, (i32, String, i32)> {
    const MAX_HEADER_LINE: usize = 65536;
    const MAX_HEADERS: usize = 1024;

    let mut line_buf = Vec::with_capacity(256);
    let n = net_tls::read_line_limited(reader, &mut line_buf, MAX_HEADER_LINE)
        .map_err(|e| map_io_err(&e, CURLE_RECV_ERROR))?;
    if n == 0 {
        return Err((
            CURLE_GOT_NOTHING,
            "Empty reply from server".to_string(),
            0,
        ));
    }

    let status_str = String::from_utf8_lossy(&line_buf);
    let status_trimmed = status_str.trim_end_matches(['\r', '\n']);
    let mut parts = status_trimmed.split_ascii_whitespace();
    let proto = parts.next().unwrap_or("");
    let code_str = parts.next().unwrap_or("");
    if !proto.starts_with("HTTP/") {
        return Err((
            CURLE_WEIRD_SERVER_REPLY,
            format!("Invalid HTTP status line: {status_trimmed:?}"),
            0,
        ));
    }
    let status_code = code_str.parse::<i32>().map_err(|_| {
        (
            CURLE_WEIRD_SERVER_REPLY,
            format!("Invalid HTTP status code: {code_str:?}"),
            0,
        )
    })?;

    let mut raw_lines = Vec::with_capacity(16);
    let mut header_bytes = line_buf.len();
    raw_lines.push(line_buf.clone());

    let mut content_length: Option<i64> = None;
    let mut chunked = false;
    let mut content_type: Option<String> = None;
    let mut content_encoding: Option<String> = None;
    let mut location: Option<String> = None;

    for _ in 0..MAX_HEADERS {
        let n = net_tls::read_line_limited(reader, &mut line_buf, MAX_HEADER_LINE)
            .map_err(|e| map_io_err(&e, CURLE_RECV_ERROR))?;
        if n == 0 {
            break;
        }
        header_bytes += n;
        raw_lines.push(line_buf.clone());
        let trimmed = line_buf
            .strip_suffix(b"\r\n")
            .or_else(|| line_buf.strip_suffix(b"\n"))
            .unwrap_or(&line_buf);
        if trimmed.is_empty() {
            break;
        }
        if let Some(colon) = trimmed.iter().position(|&b| b == b':') {
            let name = String::from_utf8_lossy(&trimmed[..colon])
                .trim()
                .to_ascii_lowercase();
            let val = String::from_utf8_lossy(&trimmed[colon + 1..])
                .trim()
                .to_string();
            match name.as_str() {
                "content-length" => {
                    if let Ok(cl) = val.parse::<i64>() {
                        if cl >= 0 {
                            content_length = Some(cl);
                        }
                    }
                }
                "transfer-encoding" => {
                    if val.to_ascii_lowercase().contains("chunked") {
                        chunked = true;
                    }
                }
                "content-type" => {
                    content_type = Some(val);
                }
                "content-encoding" => {
                    content_encoding = Some(val.to_ascii_lowercase());
                }
                "location" => {
                    location = Some(val);
                }
                _ => {}
            }
        }
    }

    Ok(ParsedResponseHead {
        status_code,
        raw_lines,
        header_bytes,
        content_length,
        chunked,
        content_type,
        content_encoding,
        location,
    })
}

fn emit_body_bytes(
    sink: &mut dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    raw: &[u8],
    size_download: &mut i64,
) -> Result<(), (i32, String, i32)> {
    if raw.is_empty() {
        return Ok(());
    }
    if let Some(ref mut inf) = inflater {
        let mut decoded = Vec::with_capacity(raw.len().saturating_mul(2));
        inf.decompress(raw, &mut decoded)
            .map_err(|code| (code, "Content-Encoding decompression failed".to_string(), 0))?;
        if !decoded.is_empty() {
            *size_download += decoded.len() as i64;
            sink.on_body_chunk(decoded)
                .map_err(|code| (code, "Write callback aborted transfer".to_string(), 0))?;
        }
    } else {
        *size_download += raw.len() as i64;
        sink.on_body_chunk(raw.to_vec())
            .map_err(|code| (code, "Write callback aborted transfer".to_string(), 0))?;
    }
    Ok(())
}

pub(super) fn execute_http_transfer(
    req: TransferRequest,
    sink: &mut dyn TransferSink,
) -> TransferOutcome {
    let start = Instant::now();
    let deadline = if req.timeout_ms > 0 {
        Some(start + Duration::from_millis(req.timeout_ms))
    } else {
        None
    };

    if is_offline() {
        log::debug!(
            "libcurl: PORT_OFFLINE=1 -> CURLE_COULDNT_CONNECT for {}",
            req.url
        );
        return TransferOutcome {
            code: CURLE_COULDNT_CONNECT,
            error_msg: Some("Offline mode (PORT_OFFLINE=1)".to_string()),
            size_download: 0,
            size_upload: 0,
            content_length_upload: req.body.len() as i64,
            total_time: start.elapsed().as_secs_f64(),
            os_errno: 0,
            ssl_verifyresult: 0,
        };
    }

    let mut initial_url = match parse_url(&req.url) {
        Ok(u) => u,
        Err((code, msg)) => {
            return TransferOutcome {
                code,
                error_msg: Some(msg),
                size_download: 0,
                size_upload: 0,
                content_length_upload: req.body.len() as i64,
                total_time: start.elapsed().as_secs_f64(),
                os_errno: 0,
                ssl_verifyresult: 0,
            };
        }
    };
    if req.port_override != 0 {
        initial_url.port = Some(req.port_override);
    }

    let initial_origin = initial_url.origin_key(0);
    let mut current_url = initial_url;
    let current_port_override: u16 = 0;
    let mut current_method = req.method.clone();
    let mut current_body = req.body.clone();
    let mut current_referer: Option<String> = None;
    let mut redirect_count: i32 = 0;
    let mut num_connects: i32 = 0;
    let mut total_size_download: i64 = 0;
    let mut total_size_upload: i64 = 0;
    let mut total_header_size: i32 = 0;
    let mut total_request_size: i32 = 0;

    loop {
        if sink.is_cancelled() {
            return TransferOutcome {
                code: CURLE_ABORTED_BY_CALLBACK,
                error_msg: Some("Transfer cancelled".to_string()),
                size_download: total_size_download,
                size_upload: total_size_upload,
                content_length_upload: current_body.len() as i64,
                total_time: start.elapsed().as_secs_f64(),
                os_errno: 0,
                ssl_verifyresult: 0,
            };
        }

        let rem = match remaining_timeout(deadline) {
            Ok(r) => r,
            Err((code, msg, os_errno)) => {
                return TransferOutcome {
                    code,
                    error_msg: Some(msg),
                    size_download: total_size_download,
                    size_upload: total_size_upload,
                    content_length_upload: current_body.len() as i64,
                    total_time: start.elapsed().as_secs_f64(),
                    os_errno,
                    ssl_verifyresult: 0,
                };
            }
        };

        let connect_timeout = {
            let base = if req.connect_timeout_ms > 0 {
                Duration::from_millis(req.connect_timeout_ms)
            } else {
                Duration::from_secs(30)
            };
            match rem {
                Some(r) => base.min(r),
                None => base,
            }
        };

        // 1. DNS resolution
        let addr_str = current_url.socket_addr_str(current_port_override);
        let addrs: Vec<_> = match addr_str.to_socket_addrs() {
            Ok(iter) => iter.collect(),
            Err(e) => {
                return TransferOutcome {
                    code: CURLE_COULDNT_RESOLVE_HOST,
                    error_msg: Some(format!("Could not resolve host {}: {e}", current_url.host)),
                    size_download: total_size_download,
                    size_upload: total_size_upload,
                    content_length_upload: current_body.len() as i64,
                    total_time: start.elapsed().as_secs_f64(),
                    os_errno: e.raw_os_error().unwrap_or(0),
                    ssl_verifyresult: 0,
                };
            }
        };
        if addrs.is_empty() {
            return TransferOutcome {
                code: CURLE_COULDNT_RESOLVE_HOST,
                error_msg: Some(format!("No addresses resolved for {}", current_url.host)),
                size_download: total_size_download,
                size_upload: total_size_upload,
                content_length_upload: current_body.len() as i64,
                total_time: start.elapsed().as_secs_f64(),
                os_errno: 0,
                ssl_verifyresult: 0,
            };
        }
        let namelookup_time = start.elapsed().as_secs_f64();

        // 2. TCP connect
        let mut connected_tcp = None;
        let mut primary_ip = None;
        let mut last_err: Option<io::Error> = None;
        for sock_addr in &addrs {
            if sink.is_cancelled() {
                return TransferOutcome {
                    code: CURLE_ABORTED_BY_CALLBACK,
                    error_msg: Some("Transfer cancelled".to_string()),
                    size_download: total_size_download,
                    size_upload: total_size_upload,
                    content_length_upload: current_body.len() as i64,
                    total_time: start.elapsed().as_secs_f64(),
                    os_errno: 0,
                    ssl_verifyresult: 0,
                };
            }
            match TcpStream::connect_timeout(sock_addr, connect_timeout) {
                Ok(tcp) => {
                    primary_ip = Some(sock_addr.ip().to_string());
                    connected_tcp = Some(tcp);
                    break;
                }
                Err(e) => {
                    last_err = Some(e);
                }
            }
        }

        let tcp = match connected_tcp {
            Some(s) => s,
            None => {
                let err = last_err
                    .unwrap_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "connect failed"));
                let (code, msg, os_errno) = map_io_err(&err, CURLE_COULDNT_CONNECT);
                return TransferOutcome {
                    code,
                    error_msg: Some(format!("Failed to connect to {addr_str}: {msg}")),
                    size_download: total_size_download,
                    size_upload: total_size_upload,
                    content_length_upload: current_body.len() as i64,
                    total_time: start.elapsed().as_secs_f64(),
                    os_errno,
                    ssl_verifyresult: 0,
                };
            }
        };
        num_connects += 1;
        let _ = tcp.set_nodelay(req.tcp_nodelay);
        let connect_time = start.elapsed().as_secs_f64();

        // 3. Optional TLS handshake (via shared crate::net_tls config & HttpStream)
        let mut stream = if current_url.scheme == "https" {
            let io_timeout = match remaining_timeout(deadline) {
                Ok(r) => r,
                Err((code, msg, os_errno)) => {
                    return TransferOutcome {
                        code,
                        error_msg: Some(msg),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno,
                        ssl_verifyresult: 0,
                    };
                }
            };
            let _ = tcp.set_read_timeout(io_timeout);
            let _ = tcp.set_write_timeout(io_timeout);

            let server_name = match ServerName::try_from(current_url.host.clone()) {
                Ok(sn) => sn,
                Err(_) => {
                    return TransferOutcome {
                        code: CURLE_URL_MALFORMAT,
                        error_msg: Some(format!("Invalid TLS server name: {}", current_url.host)),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno: 0,
                        ssl_verifyresult: 0,
                    };
                }
            };
            let policy = TlsVerifyPolicy::from_curl(req.ssl_verifypeer, req.ssl_verifyhost);
            let tls_cfg = net_tls::build_tls_config(policy, &[b"http/1.1"]);
            let mut conn = match ClientConnection::new(tls_cfg, server_name) {
                Ok(c) => c,
                Err(e) => {
                    return TransferOutcome {
                        code: CURLE_SSL_CONNECT_ERROR,
                        error_msg: Some(format!("TLS client init failed: {e}")),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno: 0,
                        ssl_verifyresult: 1,
                    };
                }
            };
            let mut tcp = tcp;
            while conn.is_handshaking() {
                if let Err(e) = conn.complete_io(&mut tcp) {
                    if e.kind() == io::ErrorKind::Interrupted {
                        continue;
                    }
                    let is_cert_err = e
                        .get_ref()
                        .and_then(|inner| inner.downcast_ref::<rustls::Error>())
                        .is_some_and(|re| matches!(re, rustls::Error::InvalidCertificate(_)));
                    let code = if e.kind() == io::ErrorKind::TimedOut
                        || e.kind() == io::ErrorKind::WouldBlock
                    {
                        CURLE_OPERATION_TIMEDOUT
                    } else if is_cert_err {
                        CURLE_PEER_FAILED_VERIFICATION
                    } else {
                        CURLE_SSL_CONNECT_ERROR
                    };
                    return TransferOutcome {
                        code,
                        error_msg: Some(format!("TLS handshake failed: {e}")),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno: e.raw_os_error().unwrap_or(0),
                        ssl_verifyresult: if is_cert_err { 1 } else { 0 },
                    };
                }
            }
            HttpStream::tls(conn, tcp)
        } else {
            HttpStream::plain(tcp)
        };

        let io_timeout = match remaining_timeout(deadline) {
            Ok(r) => r,
            Err((code, msg, os_errno)) => {
                return TransferOutcome {
                    code,
                    error_msg: Some(msg),
                    size_download: total_size_download,
                    size_upload: total_size_upload,
                    content_length_upload: current_body.len() as i64,
                    total_time: start.elapsed().as_secs_f64(),
                    os_errno,
                    ssl_verifyresult: 0,
                };
            }
        };
        let _ = stream.set_timeouts(io_timeout, io_timeout);

        // 4. Send HTTP/1.1 request
        let same_origin = current_url.origin_key(current_port_override) == initial_origin;
        let wire_req = build_http_request(
            &req,
            &current_url,
            current_port_override,
            &current_method,
            &current_body,
            current_referer.as_deref(),
            same_origin,
        );
        if req.verbose {
            log::debug!(
                "libcurl: > {} {} ({}B body)",
                current_method,
                current_url.to_url_string(),
                current_body.len()
            );
        }
        total_request_size += wire_req.len() as i32;
        total_size_upload = current_body.len() as i64;

        let mut written = 0usize;
        while written < wire_req.len() {
            match stream.write(&wire_req[written..]) {
                Ok(0) => {
                    return TransferOutcome {
                        code: CURLE_SEND_ERROR,
                        error_msg: Some("Failed writing request: connection closed".to_string()),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno: 0,
                        ssl_verifyresult: 0,
                    };
                }
                Ok(n) => written += n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => {
                    let (code, msg, os_errno) = map_io_err(&e, CURLE_SEND_ERROR);
                    return TransferOutcome {
                        code,
                        error_msg: Some(format!("Failed sending HTTP request: {msg}")),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno,
                        ssl_verifyresult: 0,
                    };
                }
            }
        }
        let _ = stream.flush();

        // 5. Read response head (skipping 1xx interim responses except 101)
        let is_tls = stream.is_tls();
        let mut reader = BufReader::with_capacity(16384, stream);

        let head = loop {
            let h = match read_response_head(&mut reader) {
                Ok(h) => h,
                Err((code, msg, os_errno)) => {
                    return TransferOutcome {
                        code,
                        error_msg: Some(msg),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno,
                        ssl_verifyresult: 0,
                    };
                }
            };
            total_header_size += h.header_bytes as i32;
            if (100..200).contains(&h.status_code) && h.status_code != 101 {
                for line in h.raw_lines {
                    if let Err(code) = sink.on_header_line(line) {
                        return TransferOutcome {
                            code,
                            error_msg: Some("Header callback aborted transfer".to_string()),
                            size_download: total_size_download,
                            size_upload: total_size_upload,
                            content_length_upload: current_body.len() as i64,
                            total_time: start.elapsed().as_secs_f64(),
                            os_errno: 0,
                            ssl_verifyresult: 0,
                        };
                    }
                }
                continue;
            }
            break h;
        };

        if req.verbose {
            log::debug!(
                "libcurl: < HTTP {} ({}, chunked={}, cl={:?})",
                head.status_code,
                current_url.to_url_string(),
                head.chunked,
                head.content_length
            );
        }

        let is_redirect_status = matches!(head.status_code, 301 | 302 | 303 | 307 | 308);
        let resolved_redirect = if is_redirect_status {
            head.location
                .as_deref()
                .map(|loc| resolve_redirect(&current_url, loc))
        } else {
            None
        };

        let redirect_url_str = resolved_redirect
            .as_ref()
            .and_then(|r| r.as_ref().ok().map(ParsedUrl::to_url_string));

        // Publish response metadata BEFORE callbacks so curl_easy_getinfo works inside callbacks.
        sink.on_head(HeadMeta {
            response_code: head.status_code,
            header_size: total_header_size,
            request_size: total_request_size,
            redirect_count,
            num_connects,
            content_length_download: if head.chunked {
                -1
            } else {
                head.content_length.unwrap_or(-1)
            },
            effective_url: current_url.to_url_string(),
            content_type: head.content_type.clone(),
            redirect_url: redirect_url_str,
            primary_ip,
            namelookup_time,
            connect_time,
        });

        // Deliver header lines to HEADERFUNCTION
        for line in head.raw_lines {
            if let Err(code) = sink.on_header_line(line) {
                return TransferOutcome {
                    code,
                    error_msg: Some("Header callback aborted transfer".to_string()),
                    size_download: total_size_download,
                    size_upload: total_size_upload,
                    content_length_upload: current_body.len() as i64,
                    total_time: start.elapsed().as_secs_f64(),
                    os_errno: 0,
                    ssl_verifyresult: 0,
                };
            }
        }

        // 6. Follow redirect if enabled
        if req.follow_location && is_redirect_status {
            if let Some(res) = resolved_redirect {
                let limit = if req.max_redirs >= 0 {
                    req.max_redirs
                } else {
                    50
                };
                if redirect_count >= limit {
                    return TransferOutcome {
                        code: CURLE_TOO_MANY_REDIRECTS,
                        error_msg: Some(format!("Maximum ({limit}) redirects followed")),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno: 0,
                        ssl_verifyresult: 0,
                    };
                }
                let next_url = match res {
                    Ok(u) => u,
                    Err((code, msg)) => {
                        return TransferOutcome {
                            code,
                            error_msg: Some(msg),
                            size_download: total_size_download,
                            size_upload: total_size_upload,
                            content_length_upload: current_body.len() as i64,
                            total_time: start.elapsed().as_secs_f64(),
                            os_errno: 0,
                            ssl_verifyresult: 0,
                        };
                    }
                };
                if req.autoreferer {
                    current_referer = Some(current_url.to_url_string());
                }
                let keep_post = match head.status_code {
                    301 => (req.postredir & CURL_REDIR_POST_301) != 0,
                    302 => (req.postredir & CURL_REDIR_POST_302) != 0,
                    303 => (req.postredir & CURL_REDIR_POST_303) != 0,
                    307 | 308 => true,
                    _ => false,
                };
                if !keep_post && !req.explicit_custom_method {
                    if head.status_code == 303 && current_method != "HEAD" {
                        current_method = "GET".to_string();
                        current_body.clear();
                    } else if matches!(head.status_code, 301 | 302) && current_method == "POST" {
                        current_method = "GET".to_string();
                        current_body.clear();
                    }
                }
                current_url = next_url;
                redirect_count += 1;
                continue;
            }
        }

        // 7. Fail-on-error check (>= 400)
        if req.fail_on_error && head.status_code >= 400 {
            return TransferOutcome {
                code: CURLE_HTTP_RETURNED_ERROR,
                error_msg: Some(format!(
                    "The requested URL returned error: {}",
                    head.status_code
                )),
                size_download: total_size_download,
                size_upload: total_size_upload,
                content_length_upload: current_body.len() as i64,
                total_time: start.elapsed().as_secs_f64(),
                os_errno: 0,
                ssl_verifyresult: 0,
            };
        }

        // 8. Bodyless responses
        let no_body = req.nobody
            || current_method.eq_ignore_ascii_case("HEAD")
            || head.status_code == 204
            || head.status_code == 304
            || (100..200).contains(&head.status_code);
        if no_body {
            return TransferOutcome {
                code: CURLE_OK,
                error_msg: None,
                size_download: total_size_download,
                size_upload: total_size_upload,
                content_length_upload: current_body.len() as i64,
                total_time: start.elapsed().as_secs_f64(),
                os_errno: 0,
                ssl_verifyresult: 0,
            };
        }

        // 9. Optional Content-Encoding decompression (when ACCEPT_ENCODING is active)
        let mut inflater = if req.accept_encoding.is_some() && req.http_content_decoding {
            match head.content_encoding.as_deref() {
                Some("gzip") | Some("x-gzip") | Some("deflate") => {
                    match StreamingInflater::new(head.content_encoding.as_deref().unwrap()) {
                        Ok(inf) => Some(inf),
                        Err(code) => {
                            return TransferOutcome {
                                code,
                                error_msg: Some("Failed initializing content decoder".to_string()),
                                size_download: total_size_download,
                                size_upload: total_size_upload,
                                content_length_upload: current_body.len() as i64,
                                total_time: start.elapsed().as_secs_f64(),
                                os_errno: 0,
                                ssl_verifyresult: 0,
                            };
                        }
                    }
                }
                _ => None,
            }
        } else {
            None
        };

        // 10. Read response body (chunked / Content-Length / close-delimited)
        let body_res: Result<(), (i32, String, i32)> = if head.chunked {
            read_chunked_body(
                &mut reader,
                sink,
                &mut inflater,
                &mut total_size_download,
                deadline,
            )
        } else if let Some(cl) = head.content_length {
            read_exact_body(
                &mut reader,
                cl as usize,
                sink,
                &mut inflater,
                &mut total_size_download,
                deadline,
            )
        } else {
            read_eof_body(
                &mut reader,
                is_tls,
                sink,
                &mut inflater,
                &mut total_size_download,
                deadline,
            )
        };

        return match body_res {
            Ok(()) => TransferOutcome {
                code: CURLE_OK,
                error_msg: None,
                size_download: total_size_download,
                size_upload: total_size_upload,
                content_length_upload: current_body.len() as i64,
                total_time: start.elapsed().as_secs_f64(),
                os_errno: 0,
                ssl_verifyresult: 0,
            },
            Err((code, msg, os_errno)) => TransferOutcome {
                code,
                error_msg: Some(msg),
                size_download: total_size_download,
                size_upload: total_size_upload,
                content_length_upload: current_body.len() as i64,
                total_time: start.elapsed().as_secs_f64(),
                os_errno,
                ssl_verifyresult: 0,
            },
        };
    }
}

fn read_chunked_body(
    reader: &mut BufReader<HttpStream>,
    sink: &mut dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    size_download: &mut i64,
    deadline: Option<Instant>,
) -> Result<(), (i32, String, i32)> {
    let cancel_token = sink.cancel_token();
    let mut dreader = DeadlineBufReader {
        inner: reader,
        deadline,
        cancel: cancel_token,
        cancelled: false,
    };
    let mut emit_err: Option<(i32, String, i32)> = None;
    let res = net_tls::read_chunked_body_streaming(
        &mut dreader,
        net_tls::DEFAULT_MAX_CHUNK_SIZE,
        |chunk| {
            if let Err(err) = emit_body_bytes(sink, inflater, chunk, size_download) {
                emit_err = Some(err);
                return Err(io::Error::other("Transfer body callback failed"));
            }
            Ok(())
        },
    );
    match res {
        Ok(()) => Ok(()),
        Err(e) => {
            if let Some(err) = emit_err {
                return Err(err);
            }
            if dreader.cancelled {
                return Err((
                    CURLE_ABORTED_BY_CALLBACK,
                    "Transfer cancelled".to_string(),
                    0,
                ));
            }
            if e.kind() == io::ErrorKind::UnexpectedEof {
                return Err((
                    CURLE_PARTIAL_FILE,
                    e.to_string(),
                    e.raw_os_error().unwrap_or(0),
                ));
            }
            Err(map_io_err(&e, CURLE_RECV_ERROR))
        }
    }
}

fn read_exact_body(
    reader: &mut BufReader<HttpStream>,
    mut remaining: usize,
    sink: &mut dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    size_download: &mut i64,
    deadline: Option<Instant>,
) -> Result<(), (i32, String, i32)> {
    let mut data_buf = vec![0u8; 16384];
    while remaining > 0 {
        if sink.is_cancelled() {
            return Err((
                CURLE_ABORTED_BY_CALLBACK,
                "Transfer cancelled".to_string(),
                0,
            ));
        }
        let rem = remaining_timeout(deadline)?;
        let _ = reader.get_ref().set_timeouts(rem, rem);

        let want = remaining.min(data_buf.len());
        let r = match reader.read(&mut data_buf[..want]) {
            Ok(0) => {
                return Err((
                    CURLE_PARTIAL_FILE,
                    format!("Transfer closed with {remaining} bytes remaining to read"),
                    0,
                ));
            }
            Ok(k) => k,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(map_io_err(&e, CURLE_RECV_ERROR)),
        };
        remaining -= r;
        emit_body_bytes(sink, inflater, &data_buf[..r], size_download)?;
    }
    Ok(())
}

fn read_eof_body(
    reader: &mut BufReader<HttpStream>,
    is_tls: bool,
    sink: &mut dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    size_download: &mut i64,
    deadline: Option<Instant>,
) -> Result<(), (i32, String, i32)> {
    let mut data_buf = vec![0u8; 16384];
    loop {
        if sink.is_cancelled() {
            return Err((
                CURLE_ABORTED_BY_CALLBACK,
                "Transfer cancelled".to_string(),
                0,
            ));
        }
        let rem = remaining_timeout(deadline)?;
        let _ = reader.get_ref().set_timeouts(rem, rem);

        match reader.read(&mut data_buf) {
            Ok(0) => return Ok(()),
            Ok(k) => emit_body_bytes(sink, inflater, &data_buf[..k], size_download)?,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            // Many HTTPS servers close TCP after Connection: close without TLS close_notify.
            Err(e) if is_tls && e.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(map_io_err(&e, CURLE_RECV_ERROR)),
        }
    }
}
