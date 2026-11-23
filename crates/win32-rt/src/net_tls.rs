//! Shared TLS (`rustls` + `webpki-roots`) and HTTP/1.1 framing primitives used by
//! `winhttp.rs`, `libcurl.rs`, `secur32.rs`, and `crypt32.rs`.
//!
//! * Process-wide singleton `ring` [`CryptoProvider`] and embedded `webpki-roots`
//!   [`RootCertStore`].
//! * Unified [`TlsVerifyPolicy`] + [`SelectiveCertVerifier`] covering WinHTTP
//!   `SECURITY_FLAG_IGNORE_*`, libcurl `CURLOPT_SSL_VERIFYPEER` / `VERIFYHOST`,
//!   and Schannel certificate verification modes.
//! * Cached [`build_tls_config`] returning shared `Arc<ClientConfig>`.
//! * [`HttpStream`] (`Plain(TcpStream)` / `Tls(...)`) with `Read`, `Write`,
//!   timeout setters, and `shutdown`.
//! * HTTP/1.1 framing helpers: [`find_header_end`], [`is_chunked_encoding`],
//!   [`parse_content_length`], [`read_line_limited`], [`read_chunked_body`],
//!   [`read_chunked_body_streaming`], [`decode_chunked_body`],
//!   [`normalize_url_path`], [`resolve_redirect_url`], and [`base64_encode`].

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::client::WebPkiServerVerifier;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{
    CertificateError, ClientConfig, ClientConnection, DigitallySignedStruct, RootCertStore,
    SignatureScheme, StreamOwned,
};
use std::io::{self, BufRead, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

// WinHTTP / WinINet certificate ignore flags (winhttp.h / wininet.h)
#[allow(dead_code)]
pub const SECURITY_FLAG_IGNORE_UNKNOWN_CA: u32 = 0x0000_0100;
#[allow(dead_code)]
pub const SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE: u32 = 0x0000_0200;
#[allow(dead_code)]
pub const SECURITY_FLAG_IGNORE_CERT_CN_INVALID: u32 = 0x0000_1000;
#[allow(dead_code)]
pub const SECURITY_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 0x0000_2000;

/// Default maximum chunk size accepted by [`decode_chunked_body`] (256 MiB).
#[allow(dead_code)]
pub const DEFAULT_MAX_CHUNK_SIZE: usize = 256 * 1024 * 1024;
#[allow(dead_code)]
const MAX_CHUNK_LINE_BYTES: usize = 64 * 1024;

// ---------------------------------------------------------------------------
// Shared `rustls` crypto provider & embedded WebPKI root store
// ---------------------------------------------------------------------------

#[allow(dead_code)]
static TLS_CRYPTO_PROVIDER: LazyLock<Arc<rustls::crypto::CryptoProvider>> =
    LazyLock::new(|| Arc::new(rustls::crypto::ring::default_provider()));

#[allow(dead_code)]
static TLS_ROOT_STORE: LazyLock<Arc<RootCertStore>> = LazyLock::new(|| {
    let mut store = RootCertStore::empty();
    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    Arc::new(store)
});

#[allow(dead_code)]
static DEFAULT_STRICT_HTTP11_CONFIG: LazyLock<Arc<ClientConfig>> = LazyLock::new(|| {
    let mut cfg = ClientConfig::builder_with_provider(Arc::clone(&TLS_CRYPTO_PROVIDER))
        .with_safe_default_protocol_versions()
        .expect("rustls ring provider supports default TLS 1.2/1.3 versions")
        .with_root_certificates(Arc::clone(&TLS_ROOT_STORE))
        .with_no_client_auth();
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    Arc::new(cfg)
});

#[allow(dead_code)]
static DEFAULT_STRICT_NO_ALPN_CONFIG: LazyLock<Arc<ClientConfig>> = LazyLock::new(|| {
    let cfg = ClientConfig::builder_with_provider(Arc::clone(&TLS_CRYPTO_PROVIDER))
        .with_safe_default_protocol_versions()
        .expect("rustls ring provider supports default TLS 1.2/1.3 versions")
        .with_root_certificates(Arc::clone(&TLS_ROOT_STORE))
        .with_no_client_auth();
    Arc::new(cfg)
});

/// Shared process-wide `ring` crypto provider for `rustls`.
#[allow(dead_code)]
#[inline]
pub fn crypto_provider() -> Arc<rustls::crypto::CryptoProvider> {
    Arc::clone(&TLS_CRYPTO_PROVIDER)
}

/// Shared process-wide embedded `webpki-roots` trust anchor store.
#[allow(dead_code)]
#[inline]
pub fn root_cert_store() -> Arc<RootCertStore> {
    Arc::clone(&TLS_ROOT_STORE)
}

/// Build a `WebPkiServerVerifier` backed by the shared `ring` provider and `roots`.
#[allow(dead_code)]
pub fn build_webpki_verifier(roots: Arc<RootCertStore>) -> Result<Arc<WebPkiServerVerifier>, String> {
    WebPkiServerVerifier::builder_with_provider(roots, crypto_provider())
        .build()
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Configurable TLS certificate verification policy
// ---------------------------------------------------------------------------

/// Certificate validation relaxation policy shared across WinHTTP, libcurl, and Schannel.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TlsVerifyPolicy {
    pub ignore_unknown_ca: bool,
    pub ignore_expired: bool,
    pub ignore_wrong_usage: bool,
    pub ignore_hostname: bool,
    pub ignore_all: bool,
}

#[allow(dead_code)]
impl TlsVerifyPolicy {
    /// Full WebPKI certificate chain, date, usage, and hostname verification.
    #[inline]
    pub const fn strict() -> Self {
        Self {
            ignore_unknown_ca: false,
            ignore_expired: false,
            ignore_wrong_usage: false,
            ignore_hostname: false,
            ignore_all: false,
        }
    }

    /// Accept any server certificate (e.g. `CURLOPT_SSL_VERIFYPEER = 0` or manual validation).
    #[inline]
    pub const fn ignore_all() -> Self {
        Self {
            ignore_unknown_ca: true,
            ignore_expired: true,
            ignore_wrong_usage: true,
            ignore_hostname: true,
            ignore_all: true,
        }
    }

    /// True if no verification check is relaxed.
    #[inline]
    pub const fn is_strict(&self) -> bool {
        !self.ignore_all
            && !self.ignore_unknown_ca
            && !self.ignore_expired
            && !self.ignore_wrong_usage
            && !self.ignore_hostname
    }

    /// Build a policy from WinHTTP / WinINet `SECURITY_FLAG_IGNORE_*` bits.
    #[inline]
    pub const fn from_winhttp_flags(security_flags: u32) -> Self {
        Self {
            ignore_unknown_ca: (security_flags & SECURITY_FLAG_IGNORE_UNKNOWN_CA) != 0,
            ignore_expired: (security_flags & SECURITY_FLAG_IGNORE_CERT_DATE_INVALID) != 0,
            ignore_wrong_usage: (security_flags & SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE) != 0,
            ignore_hostname: (security_flags & SECURITY_FLAG_IGNORE_CERT_CN_INVALID) != 0,
            ignore_all: false,
        }
    }

    /// Build a policy from libcurl `CURLOPT_SSL_VERIFYPEER` and `CURLOPT_SSL_VERIFYHOST`.
    #[inline]
    pub const fn from_curl(verify_peer: bool, verify_host: bool) -> Self {
        if !verify_peer {
            Self::ignore_all()
        } else if !verify_host {
            Self {
                ignore_hostname: true,
                ..Self::strict()
            }
        } else {
            Self::strict()
        }
    }

    /// Build a policy from Schannel credential / context flags.
    #[inline]
    pub const fn from_schannel(manual_validation: bool, no_servername_check: bool) -> Self {
        if manual_validation {
            Self::ignore_all()
        } else {
            Self {
                ignore_hostname: no_servername_check,
                ..Self::strict()
            }
        }
    }

    /// Whether `self` permits ignoring a specific `rustls::CertificateError`.
    pub fn allows_cert_error(&self, cert_err: &CertificateError) -> bool {
        if self.ignore_all {
            return true;
        }
        match cert_err {
            CertificateError::Expired
            | CertificateError::NotValidYet
            | CertificateError::ExpiredContext { .. }
            | CertificateError::NotValidYetContext { .. } => self.ignore_expired,
            CertificateError::NotValidForName
            | CertificateError::NotValidForNameContext { .. } => self.ignore_hostname,
            CertificateError::InvalidPurpose
            | CertificateError::InvalidPurposeContext { .. } => self.ignore_wrong_usage,
            _ => self.ignore_unknown_ca,
        }
    }
}

/// `rustls` certificate verifier that delegates to [`WebPkiServerVerifier`] and
/// selectively suppresses certificate errors permitted by [`TlsVerifyPolicy`].
#[allow(dead_code)]
#[derive(Debug)]
pub struct SelectiveCertVerifier {
    inner: Arc<WebPkiServerVerifier>,
    policy: TlsVerifyPolicy,
}

#[allow(dead_code)]
impl SelectiveCertVerifier {
    pub fn new(policy: TlsVerifyPolicy) -> Result<Self, String> {
        Self::with_roots(root_cert_store(), policy)
    }

    pub fn with_roots(roots: Arc<RootCertStore>, policy: TlsVerifyPolicy) -> Result<Self, String> {
        let inner = build_webpki_verifier(roots)?;
        Ok(Self { inner, policy })
    }

    #[inline]
    pub fn policy(&self) -> TlsVerifyPolicy {
        self.policy
    }
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
        if self.policy.ignore_all {
            return Ok(ServerCertVerified::assertion());
        }
        match self
            .inner
            .verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now)
        {
            Ok(v) => Ok(v),
            Err(rustls::Error::InvalidCertificate(ref cert_err))
                if self.policy.allows_cert_error(cert_err) =>
            {
                Ok(ServerCertVerified::assertion())
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

/// Build (or return a cached) `rustls::ClientConfig` for `policy` and `alpn` protocols.
#[allow(dead_code)]
pub fn build_tls_config(policy: TlsVerifyPolicy, alpn: &[&[u8]]) -> Arc<ClientConfig> {
    if policy.is_strict() {
        if alpn.is_empty() {
            return Arc::clone(&DEFAULT_STRICT_NO_ALPN_CONFIG);
        }
        if alpn.len() == 1 && alpn[0] == b"http/1.1" {
            return Arc::clone(&DEFAULT_STRICT_HTTP11_CONFIG);
        }
    }

    let builder = ClientConfig::builder_with_provider(crypto_provider())
        .with_safe_default_protocol_versions()
        .expect("rustls ring provider supports default TLS 1.2/1.3 versions");

    let mut cfg = if policy.is_strict() {
        builder
            .with_root_certificates(root_cert_store())
            .with_no_client_auth()
    } else {
        let verifier = Arc::new(
            SelectiveCertVerifier::new(policy)
                .expect("WebPkiServerVerifier from embedded webpki-roots"),
        );
        builder
            .dangerous()
            .with_custom_certificate_verifier(verifier)
            .with_no_client_auth()
    };
    cfg.alpn_protocols = alpn.iter().map(|p| p.to_vec()).collect();
    Arc::new(cfg)
}

// ---------------------------------------------------------------------------
// Unified plain TCP / rustls TLS transport stream
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub enum HttpStream {
    Plain(TcpStream),
    Tls(Box<StreamOwned<ClientConnection, TcpStream>>),
}

#[allow(dead_code)]
impl HttpStream {
    #[inline]
    pub fn plain(tcp: TcpStream) -> Self {
        Self::Plain(tcp)
    }

    #[inline]
    pub fn tls(conn: ClientConnection, tcp: TcpStream) -> Self {
        Self::Tls(Box::new(StreamOwned::new(conn, tcp)))
    }

    /// Wrap an already-connected `TcpStream` in a `rustls` client TLS stream for `host`.
    pub fn connect_tls(
        tcp: TcpStream,
        host: &str,
        config: Arc<ClientConfig>,
    ) -> io::Result<Self> {
        let clean = host.trim_start_matches('[').trim_end_matches(']');
        let server_name = ServerName::try_from(clean.to_string()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid TLS server name: {host}"),
            )
        })?;
        let conn = ClientConnection::new(config, server_name).map_err(io::Error::other)?;
        Ok(Self::tls(conn, tcp))
    }

    #[inline]
    pub fn is_tls(&self) -> bool {
        matches!(self, Self::Tls(_))
    }

    #[inline]
    pub fn tcp(&self) -> &TcpStream {
        match self {
            Self::Plain(s) => s,
            Self::Tls(s) => &s.sock,
        }
    }

    #[inline]
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.tcp().set_read_timeout(dur)
    }

    #[inline]
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.tcp().set_write_timeout(dur)
    }

    #[inline]
    pub fn set_timeouts(
        &self,
        read_to: Option<Duration>,
        write_to: Option<Duration>,
    ) -> io::Result<()> {
        let r = self.tcp().set_read_timeout(read_to);
        let w = self.tcp().set_write_timeout(write_to);
        r.and(w)
    }

    #[inline]
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        self.tcp().set_nodelay(nodelay)
    }

    #[inline]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.tcp().peer_addr()
    }

    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        self.tcp().shutdown(how)
    }

    /// Send a TLS `close_notify` alert if this is a TLS stream.
    pub fn send_close_notify(&mut self) {
        if let Self::Tls(s) = self {
            s.conn.send_close_notify();
            let _ = s.flush();
        }
    }
}

impl Read for HttpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Plain(s) => s.read(buf),
            Self::Tls(s) => s.read(buf),
        }
    }
}

impl Write for HttpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Plain(s) => s.write(buf),
            Self::Tls(s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Plain(s) => s.flush(),
            Self::Tls(s) => s.flush(),
        }
    }
}

// ---------------------------------------------------------------------------
// Shared HTTP/1.1 framing & URL helpers
// ---------------------------------------------------------------------------

/// Find the earliest `\r\n\r\n` or `\n\n` header terminator in `buf`.
/// Returns `Some((header_len, separator_len))`.
#[allow(dead_code)]
pub fn find_header_end(buf: &[u8]) -> Option<(usize, usize)> {
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

/// True if `headers` contains a `Transfer-Encoding` header with a `chunked` token.
#[allow(dead_code)]
pub fn is_chunked_encoding(headers: &[(String, String)]) -> bool {
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

/// Parse the first valid decimal `Content-Length` header value, if present.
#[allow(dead_code)]
pub fn parse_content_length(headers: &[(String, String)]) -> Option<usize> {
    for (k, v) in headers {
        if k.eq_ignore_ascii_case("Content-Length") {
            if let Ok(n) = v.trim().parse::<usize>() {
                return Some(n);
            }
        }
    }
    None
}

/// Read a single LF-delimited line from `reader` into `out` (cleared first),
/// enforcing a maximum byte limit `max_len` and retrying on `Interrupted`.
/// Returns total bytes read (`0` on clean EOF).
#[allow(dead_code)]
pub fn read_line_limited<R: BufRead>(
    reader: &mut R,
    out: &mut Vec<u8>,
    max_len: usize,
) -> io::Result<usize> {
    out.clear();
    let mut total = 0usize;
    loop {
        let available = match reader.fill_buf() {
            Ok(b) => b,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        if available.is_empty() {
            return Ok(total);
        }
        if let Some(pos) = available.iter().position(|&b| b == b'\n') {
            let take = pos + 1;
            if total + take > max_len {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "HTTP line exceeds maximum length",
                ));
            }
            out.extend_from_slice(&available[..take]);
            reader.consume(take);
            return Ok(total + take);
        }
        let len = available.len();
        if total + len > max_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "HTTP line exceeds maximum length",
            ));
        }
        out.extend_from_slice(available);
        reader.consume(len);
        total += len;
    }
}

/// Stream-decode an HTTP/1.1 `Transfer-Encoding: chunked` body from `reader`,
/// invoking `on_chunk` for each decoded payload slice.
#[allow(dead_code)]
pub fn read_chunked_body_streaming<R: BufRead, F>(
    reader: &mut R,
    max_chunk_size: usize,
    mut on_chunk: F,
) -> io::Result<()>
where
    F: FnMut(&[u8]) -> io::Result<()>,
{
    let mut line = Vec::with_capacity(64);
    let mut scratch = [0u8; 16 * 1024];
    loop {
        let n = read_line_limited(reader, &mut line, MAX_CHUNK_LINE_BYTES)?;
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "truncated chunked body (missing terminating chunk)",
            ));
        }
        let trimmed = line
            .strip_suffix(b"\r\n")
            .or_else(|| line.strip_suffix(b"\n"))
            .unwrap_or(&line);
        let size_line = std::str::from_utf8(trimmed)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "non-UTF8 chunk size"))?;
        let size_hex = size_line.split(';').next().unwrap_or("").trim();
        if size_hex.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "empty chunk size",
            ));
        }
        let chunk_size = usize::from_str_radix(size_hex, 16)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid hex chunk size"))?;
        if chunk_size > max_chunk_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "chunk size exceeds limit",
            ));
        }
        if chunk_size == 0 {
            // Drain optional trailer headers until an empty line or EOF.
            loop {
                let tn = read_line_limited(reader, &mut line, MAX_CHUNK_LINE_BYTES)?;
                if tn == 0 {
                    break;
                }
                let t = line
                    .strip_suffix(b"\r\n")
                    .or_else(|| line.strip_suffix(b"\n"))
                    .unwrap_or(&line);
                if t.is_empty() {
                    break;
                }
            }
            return Ok(());
        }

        let mut remaining = chunk_size;
        while remaining > 0 {
            let want = remaining.min(scratch.len());
            let got = match reader.read(&mut scratch[..want]) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "unexpected EOF inside chunk payload",
                    ));
                }
                Ok(g) => g,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            on_chunk(&scratch[..got])?;
            remaining -= got;
        }

        // Consume trailing CRLF (or LF) terminating the chunk payload.
        let _ = read_line_limited(reader, &mut line, MAX_CHUNK_LINE_BYTES)?;
    }
}

/// Read an entire HTTP/1.1 chunked body from `reader` into a `Vec<u8>`.
#[allow(dead_code)]
pub fn read_chunked_body<R: BufRead>(
    reader: &mut R,
    max_chunk_size: usize,
) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    read_chunked_body_streaming(reader, max_chunk_size, |chunk| {
        out.extend_from_slice(chunk);
        Ok(())
    })?;
    Ok(out)
}

/// Decode an in-memory HTTP/1.1 chunked body slice into a `Vec<u8>`.
#[allow(dead_code)]
pub fn decode_chunked_body(raw: &[u8]) -> io::Result<Vec<u8>> {
    let mut cursor = io::Cursor::new(raw);
    read_chunked_body(&mut cursor, DEFAULT_MAX_CHUNK_SIZE)
}

/// Normalize dot segments (`.` and `..`) in a URL path while preserving any
/// trailing slash and `?query` suffix.
#[allow(dead_code)]
pub fn normalize_url_path(path: &str) -> String {
    let (p, q) = match path.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (path, None),
    };
    let trailing_slash = p.ends_with('/');
    let mut stack: Vec<&str> = Vec::new();
    for seg in p.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            s => stack.push(s),
        }
    }
    let mut out = String::from("/");
    out.push_str(&stack.join("/"));
    if trailing_slash && !out.ends_with('/') {
        out.push('/');
    }
    if let Some(q) = q {
        out.push('?');
        out.push_str(q);
    }
    out
}

/// Resolve an HTTP `Location` redirect value against a base URL's components
/// into a canonical absolute URL string. Returns `None` if `location` is empty.
#[allow(dead_code)]
pub fn resolve_redirect_url(
    base_scheme: &str,
    base_host: &str,
    base_port: Option<u16>,
    base_path: &str,
    location: &str,
) -> Option<String> {
    let loc = location.trim();
    if loc.is_empty() {
        return None;
    }
    if loc.contains("://") {
        return Some(loc.to_string());
    }
    let scheme = if base_scheme.is_empty() {
        "http"
    } else {
        base_scheme
    };
    if let Some(rest) = loc.strip_prefix("//") {
        return Some(format!("{scheme}://{rest}"));
    }

    let host_disp = if base_host.contains(':') && !base_host.starts_with('[') {
        format!("[{base_host}]")
    } else {
        base_host.to_string()
    };
    let default_port = if scheme.eq_ignore_ascii_case("https") {
        443u16
    } else {
        80u16
    };
    let authority = match base_port {
        Some(p) if p != 0 && p != default_port => format!("{host_disp}:{p}"),
        _ => host_disp,
    };

    if loc.starts_with('/') {
        return Some(format!("{scheme}://{authority}{loc}"));
    }
    if loc.starts_with('?') {
        let base_no_query = base_path.split(['?', '#']).next().unwrap_or("/");
        let p = if base_no_query.is_empty() {
            "/"
        } else {
            base_no_query
        };
        return Some(format!("{scheme}://{authority}{p}{loc}"));
    }
    let base_no_query = base_path.split(['?', '#']).next().unwrap_or("/");
    let dir = match base_no_query.rfind('/') {
        Some(idx) => &base_no_query[..=idx],
        None => "/",
    };
    let joined = normalize_url_path(&format!("{dir}{loc}"));
    Some(format!("{scheme}://{authority}{joined}"))
}

/// Standard RFC 4648 Base64 encoding with `=` padding.
#[allow(dead_code)]
pub fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn tls_singletons_and_policy_caching() {
        let roots = root_cert_store();
        assert!(!roots.is_empty(), "embedded webpki roots must be populated");
        assert!(Arc::ptr_eq(&roots, &root_cert_store()));
        assert!(Arc::ptr_eq(&crypto_provider(), &crypto_provider()));

        // Cached strict configs share the same Arc.
        let c1 = build_tls_config(TlsVerifyPolicy::strict(), &[b"http/1.1"]);
        let c2 = build_tls_config(TlsVerifyPolicy::strict(), &[b"http/1.1"]);
        assert!(Arc::ptr_eq(&c1, &c2));
        assert_eq!(c1.alpn_protocols, vec![b"http/1.1".to_vec()]);

        let n1 = build_tls_config(TlsVerifyPolicy::strict(), &[]);
        let n2 = build_tls_config(TlsVerifyPolicy::strict(), &[]);
        assert!(Arc::ptr_eq(&n1, &n2));
        assert!(n1.alpn_protocols.is_empty());

        // Non-strict policies build selective verifier configs cleanly.
        let relaxed = build_tls_config(
            TlsVerifyPolicy::from_winhttp_flags(
                SECURITY_FLAG_IGNORE_CERT_CN_INVALID | SECURITY_FLAG_IGNORE_CERT_DATE_INVALID,
            ),
            &[b"http/1.1"],
        );
        assert!(!Arc::ptr_eq(&c1, &relaxed));
        let insecure = build_tls_config(TlsVerifyPolicy::from_curl(false, false), &[b"h2"]);
        assert_eq!(insecure.alpn_protocols, vec![b"h2".to_vec()]);
    }

    #[test]
    fn tls_verify_policy_error_matching() {
        let strict = TlsVerifyPolicy::strict();
        assert!(strict.is_strict());
        assert!(!strict.allows_cert_error(&CertificateError::Expired));
        assert!(!strict.allows_cert_error(&CertificateError::NotValidForName));
        assert!(!strict.allows_cert_error(&CertificateError::InvalidPurpose));
        assert!(!strict.allows_cert_error(&CertificateError::UnknownIssuer));

        let no_host = TlsVerifyPolicy::from_curl(true, false);
        assert!(!no_host.is_strict());
        assert!(no_host.allows_cert_error(&CertificateError::NotValidForName));
        assert!(!no_host.allows_cert_error(&CertificateError::Expired));
        assert!(!no_host.allows_cert_error(&CertificateError::UnknownIssuer));

        let schannel = TlsVerifyPolicy::from_schannel(false, true);
        assert!(schannel.allows_cert_error(&CertificateError::NotValidForName));
        assert!(!schannel.allows_cert_error(&CertificateError::UnknownIssuer));
        assert!(TlsVerifyPolicy::from_schannel(true, false).ignore_all);

        let winhttp = TlsVerifyPolicy::from_winhttp_flags(
            SECURITY_FLAG_IGNORE_CERT_DATE_INVALID | SECURITY_FLAG_IGNORE_CERT_WRONG_USAGE,
        );
        assert!(winhttp.allows_cert_error(&CertificateError::Expired));
        assert!(winhttp.allows_cert_error(&CertificateError::NotValidYet));
        assert!(winhttp.allows_cert_error(&CertificateError::InvalidPurpose));
        assert!(!winhttp.allows_cert_error(&CertificateError::NotValidForName));
        assert!(!winhttp.allows_cert_error(&CertificateError::UnknownIssuer));

        let all = TlsVerifyPolicy::ignore_all();
        assert!(all.allows_cert_error(&CertificateError::UnknownIssuer));
        assert!(all.allows_cert_error(&CertificateError::BadSignature));
    }

    #[test]
    fn http_header_and_chunked_helpers() {
        assert_eq!(
            find_header_end(b"HTTP/1.1 200 OK\r\nHost: a\r\n\r\nbody"),
            Some((24, 4))
        );
        assert_eq!(find_header_end(b"HTTP/1.1 200 OK\n\nbody"), Some((15, 2)));
        assert_eq!(find_header_end(b"HTTP/1.1 200 OK\r\n"), None);

        let hdrs = vec![
            ("transfer-encoding".to_string(), "gzip, CHUNKED".to_string()),
            ("content-length".to_string(), "  42  ".to_string()),
        ];
        assert!(is_chunked_encoding(&hdrs));
        assert_eq!(parse_content_length(&hdrs), Some(42));
        assert!(!is_chunked_encoding(&[("Host".into(), "example.com".into())]));
        assert_eq!(parse_content_length(&[("Content-Length".into(), "bad".into())]), None);

        // Multi-chunk with extension & trailer headers.
        let wire = b"5;ext=1\r\nHello\r\n6\r\n World\r\n0\r\nX-Trailer: ok\r\n\r\n";
        assert_eq!(decode_chunked_body(wire).unwrap(), b"Hello World");

        // Empty chunked body with LF-only endings.
        assert_eq!(decode_chunked_body(b"0\n\n").unwrap(), b"");

        // Truncated & invalid hex & oversized chunk errors.
        assert!(decode_chunked_body(b"5\r\nHel").is_err());
        assert!(decode_chunked_body(b"zzzz\r\n").is_err());
        let mut cur = io::Cursor::new(&b"10\r\n0123456789abcdef\r\n0\r\n\r\n"[..]);
        assert!(read_chunked_body(&mut cur, 8).is_err());
    }

    #[test]
    fn redirect_resolution_and_base64() {
        assert_eq!(
            resolve_redirect_url("https", "example.com", Some(443), "/a/b/c", "https://other.test/x"),
            Some("https://other.test/x".to_string())
        );
        assert_eq!(
            resolve_redirect_url("https", "example.com", Some(443), "/a/b", "//cdn.test/img.png"),
            Some("https://cdn.test/img.png".to_string())
        );
        assert_eq!(
            resolve_redirect_url("http", "example.com", Some(8080), "/a/b", "/root?k=1"),
            Some("http://example.com:8080/root?k=1".to_string())
        );
        assert_eq!(
            resolve_redirect_url("https", "example.com", Some(443), "/a/b?old=1", "?new=2"),
            Some("https://example.com/a/b?new=2".to_string())
        );
        assert_eq!(
            resolve_redirect_url("https", "example.com", None, "/a/b/c", "../d/./e?q=1"),
            Some("https://example.com/a/d/e?q=1".to_string())
        );
        assert_eq!(
            resolve_redirect_url("http", "::1", Some(9000), "/", "/v2"),
            Some("http://[::1]:9000/v2".to_string())
        );
        assert_eq!(resolve_redirect_url("http", "a", None, "/", "   "), None);

        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
        assert_eq!(base64_encode(b"user:pass"), "dXNlcjpwYXNz");
    }

    #[test]
    fn http_stream_plain_loopback_roundtrip() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4];
            sock.read_exact(&mut buf).unwrap();
            assert_eq!(&buf, b"ping");
            sock.write_all(b"pong").unwrap();
        });

        let tcp = TcpStream::connect(addr).unwrap();
        let mut stream = HttpStream::plain(tcp);
        assert!(!stream.is_tls());
        stream
            .set_timeouts(Some(Duration::from_secs(2)), Some(Duration::from_secs(2)))
            .unwrap();
        stream.set_nodelay(true).unwrap();
        stream.write_all(b"ping").unwrap();
        stream.flush().unwrap();
        let mut resp = [0u8; 4];
        stream.read_exact(&mut resp).unwrap();
        assert_eq!(&resp, b"pong");
        let _ = stream.shutdown(Shutdown::Both);
        server.join().unwrap();
    }
}
