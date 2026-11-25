//! HTTP/1.1 + TLS transport engine and streaming content decoding for `libcurl.dll`.

use super::easy::{
    apply_head_meta, invoke_header_callback, invoke_write_callback, invoke_xferinfo_callback,
    CallbackConfig, CurlEasy, HeadMeta, TransferOutcome, TransferRequest,
    CURLE_ABORTED_BY_CALLBACK, CURLE_BAD_CONTENT_ENCODING, CURLE_COULDNT_CONNECT,
    CURLE_COULDNT_RESOLVE_HOST, CURLE_FILESIZE_EXCEEDED, CURLE_GOT_NOTHING,
    CURLE_HTTP_RETURNED_ERROR, CURLE_OK, CURLE_OPERATION_TIMEDOUT, CURLE_PARTIAL_FILE,
    CURLE_PEER_FAILED_VERIFICATION, CURLE_RANGE_ERROR, CURLE_RECV_ERROR, CURLE_SEND_ERROR,
    CURLE_SSL_CONNECT_ERROR,
    CURLE_TOO_MANY_REDIRECTS, CURLE_UNSUPPORTED_PROTOCOL, CURLE_URL_MALFORMAT,
    CURLE_WEIRD_SERVER_REPLY, CURLAUTH_BASIC, CURLAUTH_BEARER, CURLAUTH_DIGEST,
    CURLAUTH_NEGOTIATE, CURLAUTH_NTLM, CURLPROTO_HTTP, CURLPROTO_HTTPS, CURL_IPRESOLVE_V4,
    CURL_IPRESOLVE_V6, CURL_REDIR_POST_301, CURL_REDIR_POST_302, CURL_REDIR_POST_303,
};
use crate::misc::zlib1::{InflateStreamError, ZlibStream, Z_DATA_ERROR};
use crate::net_tls::{self, HttpStream, TlsVerifyPolicy};
use rustls::pki_types::ServerName;
use rustls::ClientConnection;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::os::unix::io::AsRawFd;
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

fn parse_url_with_default(raw: &str, default_proto: Option<&str>) -> Result<ParsedUrl, (i32, String)> {
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
        let def = default_proto
            .map(|s| s.trim().to_ascii_lowercase())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "http".to_string());
        if def != "http" && def != "https" {
            return Err((
                CURLE_UNSUPPORTED_PROTOCOL,
                format!("Unsupported default protocol: {def}"),
            ));
        }
        (def, without_frag)
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
    parse_url_with_default(&resolved, None)
}

fn scheme_proto_bit(scheme: &str) -> u32 {
    if scheme.eq_ignore_ascii_case("https") {
        CURLPROTO_HTTPS
    } else {
        CURLPROTO_HTTP
    }
}

fn parse_auth_header_mask(val: &str) -> i32 {
    let mut mask = 0i32;
    let lower = val.to_ascii_lowercase();
    for tok in lower.split([',', ' ', '\t']) {
        match tok.trim() {
            "basic" => mask |= CURLAUTH_BASIC,
            "digest" => mask |= CURLAUTH_DIGEST,
            "negotiate" => mask |= CURLAUTH_NEGOTIATE,
            "ntlm" => mask |= CURLAUTH_NTLM,
            "bearer" => mask |= CURLAUTH_BEARER,
            _ => {}
        }
    }
    mask
}

/// Match `CURLOPT_RESOLVE` entries (`[+]host:port:addr[,addr...]`) for `(host, port)`.
fn lookup_resolve_override(entries: &[String], host: &str, port: u16) -> Option<Vec<SocketAddr>> {
    let mut matched: Option<Vec<SocketAddr>> = None;
    for raw in entries {
        let s = raw.trim();
        if let Some(removed) = s.strip_prefix('-') {
            if let Some((h, pstr)) = removed.rsplit_once(':') {
                if h.eq_ignore_ascii_case(host) && pstr.parse::<u16>().ok() == Some(port) {
                    matched = None;
                }
            }
            continue;
        }
        let s = s.strip_prefix('+').unwrap_or(s);
        let mut parts = s.splitn(3, ':');
        let (Some(h), Some(pstr), Some(addrs_str)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        if !h.eq_ignore_ascii_case(host) || pstr.parse::<u16>().ok() != Some(port) {
            continue;
        }
        let mut out = Vec::new();
        for tok in addrs_str.split(',') {
            let clean = tok.trim().trim_start_matches('[').trim_end_matches(']');
            if let Ok(ip) = clean.parse::<IpAddr>() {
                out.push(SocketAddr::new(ip, port));
            }
        }
        if !out.is_empty() {
            matched = Some(out);
        }
    }
    matched
}

// Streaming gzip/deflate decoder backed by `zlib1::ZlibStream`.
struct StreamingInflater {
    stream: ZlibStream,
    finished: bool,
    produced_any: bool,
    raw_deflate_fallback: bool,
    total_decoded: i64,
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
            total_decoded: 0,
        })
    }

    /// Stream decoded output through a fixed `CURL_MAX_WRITE_SIZE` (16 KiB) buffer,
    /// enforcing `max_decoded` (`CURLOPT_MAXFILESIZE`) before allocating or delivering
    /// any output slice to `on_chunk`. Returns `Ok(true)` if at least one decoded slice
    /// was emitted during this call.
    fn decompress_streaming(
        &mut self,
        input: &[u8],
        max_decoded: i64,
        mut on_chunk: impl FnMut(&[u8]) -> Result<(), (i32, String, i32)>,
    ) -> Result<bool, (i32, String, i32)> {
        if input.is_empty() || self.finished {
            return Ok(false);
        }
        let mut scratch = [0u8; CURL_MAX_WRITE_SIZE];
        let mut emitted = false;
        let mut total_decoded = self.total_decoded;
        let mut produced_any = self.produced_any;

        let res = self.stream.inflate_chunk_streaming(input, &mut scratch, |slice| {
            let next_decoded = total_decoded.saturating_add(slice.len() as i64);
            if max_decoded > 0 && next_decoded > max_decoded {
                return Err((
                    CURLE_FILESIZE_EXCEEDED,
                    format!("Maximum file size ({max_decoded}) exceeded"),
                    0,
                ));
            }
            total_decoded = next_decoded;
            produced_any = true;
            emitted = true;
            on_chunk(slice)
        });
        self.total_decoded = total_decoded;
        self.produced_any = produced_any;

        match res {
            Ok(done) => {
                if done {
                    self.finished = true;
                }
                Ok(emitted)
            }
            Err(InflateStreamError::Callback(err)) => Err(err),
            Err(InflateStreamError::Zlib(Z_DATA_ERROR))
                if self.raw_deflate_fallback && !self.produced_any =>
            {
                // Retry as raw RFC 1951 deflate (-15) when zlib header parsing fails before output.
                self.raw_deflate_fallback = false;
                self.stream = ZlibStream::new_inflate(-15).map_err(|_| {
                    (
                        CURLE_BAD_CONTENT_ENCODING,
                        "Content-Encoding decompression failed".to_string(),
                        0,
                    )
                })?;
                let mut total_decoded = self.total_decoded;
                let mut produced_any = self.produced_any;
                let retry = self.stream.inflate_chunk_streaming(input, &mut scratch, |slice| {
                    let next_decoded = total_decoded.saturating_add(slice.len() as i64);
                    if max_decoded > 0 && next_decoded > max_decoded {
                        return Err((
                            CURLE_FILESIZE_EXCEEDED,
                            format!("Maximum file size ({max_decoded}) exceeded"),
                            0,
                        ));
                    }
                    total_decoded = next_decoded;
                    produced_any = true;
                    emitted = true;
                    on_chunk(slice)
                });
                self.total_decoded = total_decoded;
                self.produced_any = produced_any;
                match retry {
                    Ok(done) => {
                        if done {
                            self.finished = true;
                        }
                        Ok(emitted)
                    }
                    Err(InflateStreamError::Callback(err)) => Err(err),
                    Err(InflateStreamError::Zlib(_)) => Err((
                        CURLE_BAD_CONTENT_ENCODING,
                        "Content-Encoding decompression failed".to_string(),
                        0,
                    )),
                }
            }
            Err(InflateStreamError::Zlib(_)) => Err((
                CURLE_BAD_CONTENT_ENCODING,
                "Content-Encoding decompression failed".to_string(),
                0,
            )),
        }
    }
}

// ---------------------------------------------------------------------------
// HTTP/1.1 + TLS transport engine
// ---------------------------------------------------------------------------

pub(super) trait TransferSink {
    fn is_cancelled(&self) -> bool;
    fn on_head(&self, meta: HeadMeta);
    fn on_header_line(&self, line: Vec<u8>) -> Result<(), i32>;
    fn on_body_chunk(&self, data: Vec<u8>) -> Result<(), i32>;
    fn on_progress(&self, _dltotal: i64, _dlnow: i64, _ultotal: i64, _ulnow: i64) -> Result<(), i32> {
        Ok(())
    }
}

pub(super) struct DirectSink {
    pub(super) easy: Arc<Mutex<CurlEasy>>,
    pub(super) cb: CallbackConfig,
}

impl TransferSink for DirectSink {
    fn is_cancelled(&self) -> bool {
        false
    }

    fn on_head(&self, meta: HeadMeta) {
        apply_head_meta(&self.easy, meta);
    }

    fn on_header_line(&self, line: Vec<u8>) -> Result<(), i32> {
        unsafe { invoke_header_callback(self.cb, &line) }
    }

    fn on_body_chunk(&self, data: Vec<u8>) -> Result<(), i32> {
        unsafe { invoke_write_callback(self.cb, &data) }
    }

    fn on_progress(&self, dltotal: i64, dlnow: i64, ultotal: i64, ulnow: i64) -> Result<(), i32> {
        unsafe { invoke_xferinfo_callback(self.cb, dltotal, dlnow, ultotal, ulnow) }
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

/// Maximum single socket read wait before re-checking cancellation and dispatching
/// periodic progress callbacks during idle network I/O (`CURLOPT_XFERINFOFUNCTION`
/// contract: periodic invocation even when stalled waiting for headers or body).
const IDLE_PROGRESS_POLL: Duration = Duration::from_millis(200);

fn bounded_read_timeout(deadline: Option<Instant>) -> Result<Duration, (i32, String, i32)> {
    let rem = remaining_timeout(deadline)?;
    Ok(match rem {
        Some(r) => r.min(IDLE_PROGRESS_POLL),
        None => IDLE_PROGRESS_POLL,
    })
}

struct DeadlineBufReader<'a> {
    inner: &'a mut BufReader<HttpStream>,
    sink: &'a dyn TransferSink,
    deadline: Option<Instant>,
    dltotal: i64,
    dlnow: &'a std::cell::Cell<i64>,
    ultotal: i64,
    ulnow: i64,
    aborted_code: Option<i32>,
}

impl DeadlineBufReader<'_> {
    fn prepare_io(&mut self) -> io::Result<()> {
        if self.sink.is_cancelled() {
            self.aborted_code = Some(CURLE_ABORTED_BY_CALLBACK);
            return Err(io::Error::other("Transfer cancelled"));
        }
        let slice = bounded_read_timeout(self.deadline)
            .map_err(|(_, msg, _)| io::Error::new(io::ErrorKind::TimedOut, msg))?;
        let _ = self.inner.get_ref().set_timeouts(Some(slice), Some(slice));
        Ok(())
    }

    fn on_idle_timeout(&mut self) -> io::Result<()> {
        let _ = remaining_timeout(self.deadline)
            .map_err(|(_, msg, _)| io::Error::new(io::ErrorKind::TimedOut, msg))?;
        if self.sink.is_cancelled() {
            self.aborted_code = Some(CURLE_ABORTED_BY_CALLBACK);
            return Err(io::Error::other("Transfer cancelled"));
        }
        if let Err(code) = self
            .sink
            .on_progress(self.dltotal, self.dlnow.get(), self.ultotal, self.ulnow)
        {
            self.aborted_code = Some(code);
            return Err(io::Error::other("Progress callback aborted transfer"));
        }
        Ok(())
    }
}

impl Read for DeadlineBufReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            self.prepare_io()?;
            match self.inner.read(buf) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e)
                    if e.kind() == io::ErrorKind::WouldBlock
                        || e.kind() == io::ErrorKind::TimedOut =>
                {
                    self.on_idle_timeout()?;
                }
                other => return other,
            }
        }
    }
}

impl BufRead for DeadlineBufReader<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        loop {
            self.prepare_io()?;
            let res = match self.inner.fill_buf() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
            match res {
                Ok(()) => return Ok(self.inner.buffer()),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e)
                    if e.kind() == io::ErrorKind::WouldBlock
                        || e.kind() == io::ErrorKind::TimedOut =>
                {
                    self.on_idle_timeout()?;
                }
                Err(e) => return Err(e),
            }
        }
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
    let target = req
        .request_target
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(&url.path_and_query);
    let _ = write!(wire, "{method} {target} HTTP/1.1\r\n");

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
        if let Some(ref bearer) = req.bearer_token {
            let _ = write!(wire, "Authorization: Bearer {bearer}\r\n");
        } else if req.allow_basic_auth {
            if let Some(creds) = req.basic_auth.as_deref().or(url.userinfo.as_deref()) {
                let encoded = net_tls::base64_encode(creds.as_bytes());
                let _ = write!(wire, "Authorization: Basic {encoded}\r\n");
            }
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
    content_range: Option<String>,
    chunked: bool,
    content_type: Option<String>,
    content_encoding: Option<String>,
    location: Option<String>,
    httpauth_avail: i32,
    proxyauth_avail: i32,
    retry_after: i64,
}

/// Parse RFC 9110 `Content-Range: bytes <start>-<end>/<total>` and return `<start>`.
fn parse_content_range_start(val: &str) -> Option<i64> {
    let trimmed = val.trim();
    let rest = trimmed
        .strip_prefix("bytes ")
        .or_else(|| {
            let prefix = trimmed.get(..6)?;
            prefix.eq_ignore_ascii_case("bytes ").then(|| &trimmed[6..])
        })?
        .trim_start();
    let dash = rest.find('-')?;
    rest[..dash].trim().parse::<i64>().ok().filter(|&n| n >= 0)
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
    let mut content_range: Option<String> = None;
    let mut chunked = false;
    let mut content_type: Option<String> = None;
    let mut content_encoding: Option<String> = None;
    let mut location: Option<String> = None;
    let mut httpauth_avail: i32 = 0;
    let mut proxyauth_avail: i32 = 0;
    let mut retry_after: i64 = 0;

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
                "content-range" => {
                    content_range = Some(val);
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
                "www-authenticate" => {
                    httpauth_avail |= parse_auth_header_mask(&val);
                }
                "proxy-authenticate" => {
                    proxyauth_avail |= parse_auth_header_mask(&val);
                }
                "retry-after" => {
                    if let Ok(secs) = val.parse::<i64>() {
                        if secs >= 0 {
                            retry_after = secs;
                        }
                    }
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
        content_range,
        chunked,
        content_type,
        content_encoding,
        location,
        httpauth_avail,
        proxyauth_avail,
        retry_after,
    })
}

/// Maximum bytes delivered in a single `CURLOPT_WRITEFUNCTION` invocation per
/// libcurl's documented contract (`CURL_MAX_WRITE_SIZE`, 16 KiB), independent
/// of `CURLOPT_BUFFERSIZE`.
pub(super) const CURL_MAX_WRITE_SIZE: usize = 16 * 1024;

fn emit_body_bytes(
    sink: &dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    raw: &[u8],
    size_download: &mut i64,
    max_filesize: i64,
    dltotal: i64,
    ultotal: i64,
) -> Result<(), (i32, String, i32)> {
    if raw.is_empty() {
        return Ok(());
    }
    if let Some(ref mut inf) = inflater {
        // Track transferred (wire/encoded) bytes separately from decompressed output
        // so `dlnow` matches `dltotal` (Content-Length) and `CURLINFO_SIZE_DOWNLOAD_T`
        // per libcurl's progress and transfer-size contracts.
        *size_download += raw.len() as i64;
        if max_filesize > 0 && *size_download > max_filesize {
            return Err((
                CURLE_FILESIZE_EXCEEDED,
                format!("Maximum file size ({max_filesize}) exceeded"),
                0,
            ));
        }
        let dlnow = *size_download;
        let emitted = inf.decompress_streaming(raw, max_filesize, |chunk| {
            if sink.is_cancelled() {
                return Err((
                    CURLE_ABORTED_BY_CALLBACK,
                    "Transfer cancelled".to_string(),
                    0,
                ));
            }
            sink.on_body_chunk(chunk.to_vec())
                .map_err(|code| (code, "Write callback aborted transfer".to_string(), 0))?;
            sink.on_progress(dltotal, dlnow, ultotal, ultotal)
                .map_err(|code| (code, "Progress callback aborted transfer".to_string(), 0))?;
            Ok(())
        })?;
        if !emitted {
            sink.on_progress(dltotal, dlnow, ultotal, ultotal)
                .map_err(|code| (code, "Progress callback aborted transfer".to_string(), 0))?;
        }
        return Ok(());
    }
    for chunk in raw.chunks(CURL_MAX_WRITE_SIZE) {
        *size_download += chunk.len() as i64;
        if max_filesize > 0 && *size_download > max_filesize {
            return Err((
                CURLE_FILESIZE_EXCEEDED,
                format!("Maximum file size ({max_filesize}) exceeded"),
                0,
            ));
        }
        sink.on_body_chunk(chunk.to_vec())
            .map_err(|code| (code, "Write callback aborted transfer".to_string(), 0))?;
        sink.on_progress(dltotal, *size_download, ultotal, ultotal)
            .map_err(|code| (code, "Progress callback aborted transfer".to_string(), 0))?;
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

    let mut initial_url = match parse_url_with_default(&req.url, req.default_protocol.as_deref()) {
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
    if (req.allowed_protocols & scheme_proto_bit(&initial_url.scheme)) == 0 {
        return TransferOutcome {
            code: CURLE_UNSUPPORTED_PROTOCOL,
            error_msg: Some(format!("Protocol {:?} disabled by CURLOPT_PROTOCOLS", initial_url.scheme)),
            size_download: 0,
            size_upload: 0,
            content_length_upload: req.body.len() as i64,
            total_time: start.elapsed().as_secs_f64(),
            os_errno: 0,
            ssl_verifyresult: 0,
        };
    }
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
    let mut redirect_time: f64 = 0.0;
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

        // 1. DNS resolution (honoring CURLOPT_RESOLVE and CURLOPT_IPRESOLVE)
        let eff_port = current_url.effective_port(current_port_override);
        let addr_str = current_url.socket_addr_str(current_port_override);
        let mut addrs: Vec<SocketAddr> = if let Some(ov) =
            lookup_resolve_override(&req.resolve_entries, &current_url.host, eff_port)
        {
            ov
        } else {
            match addr_str.to_socket_addrs() {
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
            }
        };
        if req.ipresolve == CURL_IPRESOLVE_V4 {
            addrs.retain(|a| a.is_ipv4());
        } else if req.ipresolve == CURL_IPRESOLVE_V6 {
            addrs.retain(|a| a.is_ipv6());
        }
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
        let mut primary_port: i32 = eff_port as i32;
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
                    primary_port = sock_addr.port() as i32;
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
        if req.tcp_keepalive {
            let fd = tcp.as_raw_fd();
            let one: i32 = 1;
            let idle = req.tcp_keepidle.max(1) as i32;
            let intvl = req.tcp_keepintvl.max(1) as i32;
            unsafe {
                let _ = libc::setsockopt(
                    fd,
                    libc::SOL_SOCKET,
                    libc::SO_KEEPALIVE,
                    &one as *const i32 as *const libc::c_void,
                    std::mem::size_of::<i32>() as libc::socklen_t,
                );
                let _ = libc::setsockopt(
                    fd,
                    libc::IPPROTO_TCP,
                    libc::TCP_KEEPALIVE,
                    &idle as *const i32 as *const libc::c_void,
                    std::mem::size_of::<i32>() as libc::socklen_t,
                );
                let _ = libc::setsockopt(
                    fd,
                    libc::IPPROTO_TCP,
                    libc::TCP_KEEPINTVL,
                    &intvl as *const i32 as *const libc::c_void,
                    std::mem::size_of::<i32>() as libc::socklen_t,
                );
            }
        }
        let (local_ip, local_port) = match tcp.local_addr() {
            Ok(la) => (Some(la.ip().to_string()), la.port() as i32),
            Err(_) => (None, 0),
        };
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

        let appconnect_time = if stream.is_tls() {
            start.elapsed().as_secs_f64()
        } else {
            0.0
        };
        let pretransfer_time = start.elapsed().as_secs_f64();

        // 4. Send HTTP/1.1 request
        let same_origin = current_url.origin_key(current_port_override) == initial_origin;
        let wire_req = build_http_request(
            &req,
            &current_url,
            current_port_override,
            &current_method,
            &current_body,
            current_referer.as_deref(),
            same_origin || req.unrestricted_auth,
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
        let head_dlnow = std::cell::Cell::new(total_size_download);
        let head_ultotal = current_body.len() as i64;

        let head = loop {
            let mut hreader = DeadlineBufReader {
                inner: &mut reader,
                sink,
                deadline,
                dltotal: 0,
                dlnow: &head_dlnow,
                ultotal: head_ultotal,
                ulnow: head_ultotal,
                aborted_code: None,
            };
            let h = match read_response_head(&mut hreader) {
                Ok(h) => h,
                Err((code, msg, os_errno)) => {
                    let (final_code, final_msg, final_errno) = if let Some(aborted) = hreader.aborted_code {
                        (aborted, "Progress callback aborted transfer".to_string(), 0)
                    } else {
                        (code, msg, os_errno)
                    };
                    return TransferOutcome {
                        code: final_code,
                        error_msg: Some(final_msg),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno: final_errno,
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

        let starttransfer_time = start.elapsed().as_secs_f64();
        let proto_bit = scheme_proto_bit(&current_url.scheme);
        // Publish response metadata BEFORE callbacks so curl_easy_getinfo works inside callbacks.
        sink.on_head(HeadMeta {
            response_code: head.status_code,
            header_size: total_header_size,
            request_size: total_request_size,
            redirect_count,
            num_connects,
            primary_port,
            local_port,
            protocol: proto_bit as i32,
            condition_unmet: i32::from(head.status_code == 304),
            httpauth_avail: head.httpauth_avail,
            proxyauth_avail: head.proxyauth_avail,
            retry_after: head.retry_after,
            content_length_download: if head.chunked {
                -1
            } else {
                head.content_length.unwrap_or(-1)
            },
            effective_url: current_url.to_url_string(),
            content_type: head.content_type.clone(),
            redirect_url: redirect_url_str,
            primary_ip,
            local_ip,
            scheme: current_url.scheme.to_ascii_uppercase(),
            effective_method: current_method.clone(),
            namelookup_time,
            connect_time,
            appconnect_time,
            pretransfer_time,
            starttransfer_time,
            redirect_time,
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
                let next_proto_bit = scheme_proto_bit(&next_url.scheme);
                if (req.allowed_protocols & next_proto_bit) == 0
                    || (req.redir_protocols & next_proto_bit) == 0
                {
                    return TransferOutcome {
                        code: CURLE_UNSUPPORTED_PROTOCOL,
                        error_msg: Some(format!(
                            "Redirect protocol {:?} disabled by CURLOPT_PROTOCOLS / CURLOPT_REDIR_PROTOCOLS",
                            next_url.scheme
                        )),
                        size_download: total_size_download,
                        size_upload: total_size_upload,
                        content_length_upload: current_body.len() as i64,
                        total_time: start.elapsed().as_secs_f64(),
                        os_errno: 0,
                        ssl_verifyresult: 0,
                    };
                }
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
                redirect_time = start.elapsed().as_secs_f64();
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

        // 7b. Max file size check on declared Content-Length
        if req.max_filesize > 0 {
            if let Some(cl) = head.content_length {
                if cl > req.max_filesize {
                    return TransferOutcome {
                        code: CURLE_FILESIZE_EXCEEDED,
                        error_msg: Some(format!(
                            "Maximum file size ({}) exceeded (Content-Length: {cl})",
                            req.max_filesize
                        )),
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

        // 8b. Validate resumed download response before delivering any body bytes:
        // when CURLOPT_RESUME_FROM > 0, the server must reply 206 Partial Content with a
        // matching Content-Range start offset; otherwise return CURLE_RANGE_ERROR rather
        // than delivering a full 200 body that would corrupt the caller's partial file.
        if req.resume_from > 0 {
            let range_matches = head.status_code == 206
                && head
                    .content_range
                    .as_deref()
                    .and_then(parse_content_range_start)
                    == Some(req.resume_from);
            if !range_matches {
                return TransferOutcome {
                    code: CURLE_RANGE_ERROR,
                    error_msg: Some(format!(
                        "HTTP server did not honor resume offset {} (status={}, Content-Range={:?})",
                        req.resume_from, head.status_code, head.content_range
                    )),
                    size_download: total_size_download,
                    size_upload: total_size_upload,
                    content_length_upload: current_body.len() as i64,
                    total_time: start.elapsed().as_secs_f64(),
                    os_errno: 0,
                    ssl_verifyresult: 0,
                };
            }
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
        let dltotal = if head.chunked {
            0
        } else {
            head.content_length.unwrap_or(0)
        };
        let ultotal = current_body.len() as i64;
        if let Err(code) = sink.on_progress(dltotal, total_size_download, ultotal, ultotal) {
            return TransferOutcome {
                code,
                error_msg: Some("Progress callback aborted transfer".to_string()),
                size_download: total_size_download,
                size_upload: total_size_upload,
                content_length_upload: ultotal,
                total_time: start.elapsed().as_secs_f64(),
                os_errno: 0,
                ssl_verifyresult: 0,
            };
        }
        let body_res: Result<(), (i32, String, i32)> = if head.chunked {
            read_chunked_body(
                &mut reader,
                sink,
                &mut inflater,
                &mut total_size_download,
                req.max_filesize,
                dltotal,
                ultotal,
                deadline,
            )
        } else if let Some(cl) = head.content_length {
            read_exact_body(
                &mut reader,
                cl as usize,
                req.buffer_size,
                sink,
                &mut inflater,
                &mut total_size_download,
                req.max_filesize,
                dltotal,
                ultotal,
                deadline,
            )
        } else {
            read_eof_body(
                &mut reader,
                is_tls,
                req.buffer_size,
                sink,
                &mut inflater,
                &mut total_size_download,
                req.max_filesize,
                dltotal,
                ultotal,
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

#[allow(clippy::too_many_arguments)]
fn read_chunked_body(
    reader: &mut BufReader<HttpStream>,
    sink: &dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    size_download: &mut i64,
    max_filesize: i64,
    dltotal: i64,
    ultotal: i64,
    deadline: Option<Instant>,
) -> Result<(), (i32, String, i32)> {
    let dlnow_cell = std::cell::Cell::new(*size_download);
    let mut dreader = DeadlineBufReader {
        inner: reader,
        sink,
        deadline,
        dltotal,
        dlnow: &dlnow_cell,
        ultotal,
        ulnow: ultotal,
        aborted_code: None,
    };
    let mut emit_err: Option<(i32, String, i32)> = None;
    let res = net_tls::read_chunked_body_streaming(
        &mut dreader,
        net_tls::DEFAULT_MAX_CHUNK_SIZE,
        |chunk| {
            let r = emit_body_bytes(
                sink,
                inflater,
                chunk,
                size_download,
                max_filesize,
                dltotal,
                ultotal,
            );
            dlnow_cell.set(*size_download);
            if let Err(err) = r {
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
            if let Some(code) = dreader.aborted_code {
                return Err((code, "Progress callback aborted transfer".to_string(), 0));
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

#[allow(clippy::too_many_arguments)]
fn read_exact_body(
    reader: &mut BufReader<HttpStream>,
    mut remaining: usize,
    buffer_size: usize,
    sink: &dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    size_download: &mut i64,
    max_filesize: i64,
    dltotal: i64,
    ultotal: i64,
    deadline: Option<Instant>,
) -> Result<(), (i32, String, i32)> {
    let mut data_buf = vec![0u8; buffer_size.max(1024)];
    while remaining > 0 {
        if sink.is_cancelled() {
            return Err((
                CURLE_ABORTED_BY_CALLBACK,
                "Transfer cancelled".to_string(),
                0,
            ));
        }
        let slice = bounded_read_timeout(deadline)?;
        let _ = reader.get_ref().set_timeouts(Some(slice), Some(slice));

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
            Err(e)
                if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::TimedOut =>
            {
                let _ = remaining_timeout(deadline)?;
                if sink.is_cancelled() {
                    return Err((
                        CURLE_ABORTED_BY_CALLBACK,
                        "Transfer cancelled".to_string(),
                        0,
                    ));
                }
                sink.on_progress(dltotal, *size_download, ultotal, ultotal)
                    .map_err(|code| (code, "Progress callback aborted transfer".to_string(), 0))?;
                continue;
            }
            Err(e) => return Err(map_io_err(&e, CURLE_RECV_ERROR)),
        };
        remaining -= r;
        emit_body_bytes(
            sink,
            inflater,
            &data_buf[..r],
            size_download,
            max_filesize,
            dltotal,
            ultotal,
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn read_eof_body(
    reader: &mut BufReader<HttpStream>,
    is_tls: bool,
    buffer_size: usize,
    sink: &dyn TransferSink,
    inflater: &mut Option<StreamingInflater>,
    size_download: &mut i64,
    max_filesize: i64,
    dltotal: i64,
    ultotal: i64,
    deadline: Option<Instant>,
) -> Result<(), (i32, String, i32)> {
    let mut data_buf = vec![0u8; buffer_size.max(1024)];
    loop {
        if sink.is_cancelled() {
            return Err((
                CURLE_ABORTED_BY_CALLBACK,
                "Transfer cancelled".to_string(),
                0,
            ));
        }
        let slice = bounded_read_timeout(deadline)?;
        let _ = reader.get_ref().set_timeouts(Some(slice), Some(slice));

        match reader.read(&mut data_buf) {
            Ok(0) => return Ok(()),
            Ok(k) => emit_body_bytes(
                sink,
                inflater,
                &data_buf[..k],
                size_download,
                max_filesize,
                dltotal,
                ultotal,
            )?,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e)
                if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::TimedOut =>
            {
                let _ = remaining_timeout(deadline)?;
                if sink.is_cancelled() {
                    return Err((
                        CURLE_ABORTED_BY_CALLBACK,
                        "Transfer cancelled".to_string(),
                        0,
                    ));
                }
                sink.on_progress(dltotal, *size_download, ultotal, ultotal)
                    .map_err(|code| (code, "Progress callback aborted transfer".to_string(), 0))?;
                continue;
            }
            // Many HTTPS servers close TCP after Connection: close without TLS close_notify.
            Err(e) if is_tls && e.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(e) => return Err(map_io_err(&e, CURLE_RECV_ERROR)),
        }
    }
}
