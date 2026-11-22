//! Builtin `libcurl.dll`: HTTP/1.1 + HTTPS (TLS 1.2/1.3 via `rustls` + `webpki-roots`)
//! easy and multi transfer client.
//!
//! * Supports the full `curl_easy_*`, `curl_multi_*`, `curl_slist_*`, `curl_share_*`,
//!   `curl_global_*`, `curl_version*`, and escaping/freeing API surface with Win64 LLP64 ABI.
//! * Invokes guest `CURLOPT_WRITEFUNCTION`, `CURLOPT_HEADERFUNCTION`, and
//!   `CURLOPT_READFUNCTION` callbacks via [`crate::call_guest8`] strictly on the caller's
//!   thread (both in `curl_easy_perform` and in `curl_multi_perform`).
//! * `curl_multi_*` executes network I/O concurrently on background host threads and streams
//!   response metadata, headers, and body chunks back to `curl_multi_perform`, waking
//!   `curl_multi_poll` / `curl_multi_wait` via a condition variable.
//! * Honors `PORT_OFFLINE=1` by failing transfers with `CURLE_COULDNT_CONNECT` (`7`).

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::client::WebPkiServerVerifier;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, ClientConnection, DigitallySignedStruct, RootCertStore, SignatureScheme};
use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, LazyLock, Mutex};
use std::time::{Duration, Instant};

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

// ---------------------------------------------------------------------------
// libcurl constants (exact upstream curl/curl.h & curl/multi.h numeric IDs)
// ---------------------------------------------------------------------------

// CURLcode
const CURLE_OK: i32 = 0;
const CURLE_UNSUPPORTED_PROTOCOL: i32 = 1;
const CURLE_FAILED_INIT: i32 = 2;
const CURLE_URL_MALFORMAT: i32 = 3;
const CURLE_NOT_BUILT_IN: i32 = 4;
const CURLE_COULDNT_RESOLVE_HOST: i32 = 6;
const CURLE_COULDNT_CONNECT: i32 = 7;
const CURLE_WEIRD_SERVER_REPLY: i32 = 8;
const CURLE_PARTIAL_FILE: i32 = 18;
const CURLE_HTTP_RETURNED_ERROR: i32 = 22;
const CURLE_WRITE_ERROR: i32 = 23;
const CURLE_READ_ERROR: i32 = 26;
const CURLE_OUT_OF_MEMORY: i32 = 27;
const CURLE_OPERATION_TIMEDOUT: i32 = 28;
const CURLE_SSL_CONNECT_ERROR: i32 = 35;
const CURLE_ABORTED_BY_CALLBACK: i32 = 42;
const CURLE_BAD_FUNCTION_ARGUMENT: i32 = 43;
const CURLE_TOO_MANY_REDIRECTS: i32 = 47;
const CURLE_UNKNOWN_OPTION: i32 = 48;
const CURLE_GOT_NOTHING: i32 = 52;
const CURLE_SEND_ERROR: i32 = 55;
const CURLE_RECV_ERROR: i32 = 56;
const CURLE_PEER_FAILED_VERIFICATION: i32 = 60;
const CURLE_BAD_CONTENT_ENCODING: i32 = 61;

// CURLMcode
const CURLM_OK: i32 = 0;
const CURLM_BAD_HANDLE: i32 = 1;
const CURLM_BAD_EASY_HANDLE: i32 = 2;
const CURLM_OUT_OF_MEMORY: i32 = 3;
const CURLM_INTERNAL_ERROR: i32 = 4;
const CURLM_BAD_SOCKET: i32 = 5;
const CURLM_UNKNOWN_OPTION: i32 = 6;
const CURLM_ADDED_ALREADY: i32 = 7;
const CURLM_BAD_FUNCTION_ARGUMENT: i32 = 10;

// CURLSHcode
const CURLSHE_OK: i32 = 0;
const CURLSHE_BAD_OPTION: i32 = 1;
const CURLSHE_INVALID: i32 = 3;

// CURLMSG
const CURLMSG_DONE: i32 = 1;

// Special callback return sentinel
const CURL_READFUNC_ABORT: usize = 0x1000_0000;
const CURL_ERROR_SIZE: usize = 256;

// HTTP version reported by CURLINFO_HTTP_VERSION
const CURL_HTTP_VERSION_1_1: i32 = 2;

// POSTREDIR bitflags
const CURL_REDIR_POST_301: i32 = 1;
const CURL_REDIR_POST_302: i32 = 2;
const CURL_REDIR_POST_303: i32 = 4;

// CURLOPT_* option IDs
const CURLOPT_WRITEDATA: i32 = 10001;
const CURLOPT_URL: i32 = 10002;
const CURLOPT_PORT: i32 = 3;
const CURLOPT_PROXY: i32 = 10004;
const CURLOPT_USERPWD: i32 = 10005;
const CURLOPT_PROXYUSERPWD: i32 = 10006;
const CURLOPT_RANGE: i32 = 10007;
const CURLOPT_READDATA: i32 = 10009;
const CURLOPT_ERRORBUFFER: i32 = 10010;
const CURLOPT_WRITEFUNCTION: i32 = 20011;
const CURLOPT_READFUNCTION: i32 = 20012;
const CURLOPT_TIMEOUT: i32 = 13;
const CURLOPT_INFILESIZE: i32 = 14;
const CURLOPT_POSTFIELDS: i32 = 10015;
const CURLOPT_REFERER: i32 = 10016;
const CURLOPT_USERAGENT: i32 = 10018;
const CURLOPT_LOW_SPEED_LIMIT: i32 = 19;
const CURLOPT_LOW_SPEED_TIME: i32 = 20;
const CURLOPT_COOKIE: i32 = 10022;
const CURLOPT_HTTPHEADER: i32 = 10023;
const CURLOPT_HEADERDATA: i32 = 10029;
const CURLOPT_COOKIEFILE: i32 = 10031;
const CURLOPT_SSLVERSION: i32 = 32;
const CURLOPT_CUSTOMREQUEST: i32 = 10036;
const CURLOPT_VERBOSE: i32 = 41;
const CURLOPT_HEADER: i32 = 42;
const CURLOPT_NOPROGRESS: i32 = 43;
const CURLOPT_NOBODY: i32 = 44;
const CURLOPT_FAILONERROR: i32 = 45;
const CURLOPT_UPLOAD: i32 = 46;
const CURLOPT_POST: i32 = 47;
const CURLOPT_FOLLOWLOCATION: i32 = 52;
const CURLOPT_PUT: i32 = 54;
const CURLOPT_PROGRESSFUNCTION: i32 = 20056;
const CURLOPT_XFERINFODATA: i32 = 10057;
const CURLOPT_AUTOREFERER: i32 = 58;
const CURLOPT_PROXYPORT: i32 = 59;
const CURLOPT_POSTFIELDSIZE: i32 = 60;
const CURLOPT_HTTPPROXYTUNNEL: i32 = 61;
const CURLOPT_SSL_VERIFYPEER: i32 = 64;
const CURLOPT_CAINFO: i32 = 10065;
const CURLOPT_MAXREDIRS: i32 = 68;
const CURLOPT_MAXCONNECTS: i32 = 71;
const CURLOPT_FRESH_CONNECT: i32 = 74;
const CURLOPT_FORBID_REUSE: i32 = 75;
const CURLOPT_CONNECTTIMEOUT: i32 = 78;
const CURLOPT_HEADERFUNCTION: i32 = 20079;
const CURLOPT_HTTPGET: i32 = 80;
const CURLOPT_SSL_VERIFYHOST: i32 = 81;
const CURLOPT_COOKIEJAR: i32 = 10082;
const CURLOPT_HTTP_VERSION: i32 = 84;
const CURLOPT_DNS_CACHE_TIMEOUT: i32 = 92;
const CURLOPT_DEBUGFUNCTION: i32 = 20094;
const CURLOPT_DEBUGDATA: i32 = 10095;
const CURLOPT_CAPATH: i32 = 10097;
const CURLOPT_BUFFERSIZE: i32 = 98;
const CURLOPT_NOSIGNAL: i32 = 99;
const CURLOPT_SHARE: i32 = 10100;
const CURLOPT_ACCEPT_ENCODING: i32 = 10102;
const CURLOPT_PRIVATE: i32 = 10103;
const CURLOPT_SSL_CTX_FUNCTION: i32 = 20108;
const CURLOPT_SSL_CTX_DATA: i32 = 10109;
const CURLOPT_IPRESOLVE: i32 = 113;
const CURLOPT_INFILESIZE_LARGE: i32 = 30115;
const CURLOPT_POSTFIELDSIZE_LARGE: i32 = 30120;
const CURLOPT_TCP_NODELAY: i32 = 121;
const CURLOPT_MAX_SEND_SPEED_LARGE: i32 = 30145;
const CURLOPT_MAX_RECV_SPEED_LARGE: i32 = 30146;
const CURLOPT_SOCKOPTFUNCTION: i32 = 20148;
const CURLOPT_SOCKOPTDATA: i32 = 10149;
const CURLOPT_SSL_SESSIONID_CACHE: i32 = 150;
const CURLOPT_TIMEOUT_MS: i32 = 155;
const CURLOPT_CONNECTTIMEOUT_MS: i32 = 156;
const CURLOPT_HTTP_TRANSFER_DECODING: i32 = 157;
const CURLOPT_HTTP_CONTENT_DECODING: i32 = 158;
const CURLOPT_POSTREDIR: i32 = 161;
const CURLOPT_OPENSOCKETFUNCTION: i32 = 20163;
const CURLOPT_OPENSOCKETDATA: i32 = 10164;
const CURLOPT_COPYPOSTFIELDS: i32 = 10165;
const CURLOPT_SEEKFUNCTION: i32 = 20167;
const CURLOPT_SEEKDATA: i32 = 10168;
const CURLOPT_USERNAME: i32 = 10173;
const CURLOPT_PASSWORD: i32 = 10174;
const CURLOPT_NOPROXY: i32 = 10177;
const CURLOPT_PROTOCOLS: i32 = 181;
const CURLOPT_REDIR_PROTOCOLS: i32 = 182;
const CURLOPT_CLOSESOCKETFUNCTION: i32 = 20208;
const CURLOPT_CLOSESOCKETDATA: i32 = 10209;
const CURLOPT_TCP_KEEPALIVE: i32 = 213;
const CURLOPT_TCP_KEEPIDLE: i32 = 214;
const CURLOPT_TCP_KEEPINTVL: i32 = 215;
const CURLOPT_SSL_OPTIONS: i32 = 216;
const CURLOPT_XFERINFOFUNCTION: i32 = 20219;
const CURLOPT_EXPECT_100_TIMEOUT_MS: i32 = 227;
const CURLOPT_PATH_AS_IS: i32 = 234;
const CURLOPT_PIPEWAIT: i32 = 237;
const CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS: i32 = 271;
const CURLOPT_PROTOCOLS_STR: i32 = 10318;
const CURLOPT_REDIR_PROTOCOLS_STR: i32 = 10319;

// CURLINFO_* IDs
const CURLINFO_EFFECTIVE_URL: i32 = 0x10_0001;
const CURLINFO_CONTENT_TYPE: i32 = 0x10_0012;
const CURLINFO_PRIVATE: i32 = 0x10_0015;
const CURLINFO_REDIRECT_URL: i32 = 0x10_001f;
const CURLINFO_PRIMARY_IP: i32 = 0x10_0020;

const CURLINFO_RESPONSE_CODE: i32 = 0x20_0002;
const CURLINFO_HEADER_SIZE: i32 = 0x20_000b;
const CURLINFO_REQUEST_SIZE: i32 = 0x20_000c;
const CURLINFO_SSL_VERIFYRESULT: i32 = 0x20_000d;
const CURLINFO_REDIRECT_COUNT: i32 = 0x20_0014;
const CURLINFO_HTTP_CONNECTCODE: i32 = 0x20_0016;
const CURLINFO_OS_ERRNO: i32 = 0x20_0019;
const CURLINFO_NUM_CONNECTS: i32 = 0x20_001a;
const CURLINFO_HTTP_VERSION: i32 = 0x20_002e;

const CURLINFO_TOTAL_TIME: i32 = 0x30_0003;
const CURLINFO_NAMELOOKUP_TIME: i32 = 0x30_0004;
const CURLINFO_CONNECT_TIME: i32 = 0x30_0005;
const CURLINFO_SIZE_UPLOAD: i32 = 0x30_0007;
const CURLINFO_SIZE_DOWNLOAD: i32 = 0x30_0008;
const CURLINFO_SPEED_DOWNLOAD: i32 = 0x30_0009;
const CURLINFO_SPEED_UPLOAD: i32 = 0x30_000a;
const CURLINFO_CONTENT_LENGTH_DOWNLOAD: i32 = 0x30_000f;
const CURLINFO_CONTENT_LENGTH_UPLOAD: i32 = 0x30_0010;

const CURLINFO_SIZE_UPLOAD_T: i32 = 0x60_0007;
const CURLINFO_SIZE_DOWNLOAD_T: i32 = 0x60_0008;
const CURLINFO_SPEED_DOWNLOAD_T: i32 = 0x60_0009;
const CURLINFO_SPEED_UPLOAD_T: i32 = 0x60_000a;
const CURLINFO_CONTENT_LENGTH_DOWNLOAD_T: i32 = 0x60_000f;
const CURLINFO_CONTENT_LENGTH_UPLOAD_T: i32 = 0x60_0010;
const CURLINFO_TOTAL_TIME_T: i32 = 0x60_0032;

// Version feature bits
const CURL_VERSION_IPV6: i32 = 1 << 0;
const CURL_VERSION_SSL: i32 = 1 << 2;
const CURL_VERSION_LIBZ: i32 = 1 << 3;
const CURL_VERSION_LARGEFILE: i32 = 1 << 9;

// ---------------------------------------------------------------------------
// Win64 ABI structures
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct CurlSlist {
    pub data: *mut u8,
    pub next: *mut CurlSlist,
}

#[repr(C)]
pub struct CurlMsg {
    pub msg: i32,
    pub _pad: u32,
    pub easy_handle: usize,
    pub data: usize,
}
const _: () = assert!(std::mem::size_of::<CurlMsg>() == 24);

#[repr(C)]
pub struct CurlVersionInfoData {
    pub age: i32,
    pub _pad0: u32,
    pub version: *const u8,
    pub version_num: u32,
    pub _pad1: u32,
    pub host: *const u8,
    pub features: i32,
    pub _pad2: u32,
    pub ssl_version: *const u8,
    pub ssl_version_num: i32,
    pub _pad3: u32,
    pub libz_version: *const u8,
    pub protocols: *const *const u8,
    pub ares: *const u8,
    pub ares_num: i32,
    pub _pad4: u32,
    pub libidn: *const u8,
    pub iconv_ver_num: i32,
    pub _pad5: u32,
    pub libssh_version: *const u8,
    pub brotli_ver_num: u32,
    pub _pad6: u32,
    pub brotli_version: *const u8,
    pub nghttp2_ver_num: u32,
    pub _pad7: u32,
    pub nghttp2_version: *const u8,
    pub quic_version: *const u8,
    pub cainfo: *const u8,
    pub capath: *const u8,
    pub zstd_ver_num: u32,
    pub _pad8: u32,
    pub zstd_version: *const u8,
    pub hyper_version: *const u8,
    pub gsasl_version: *const u8,
    pub feature_names: *const *const u8,
}
unsafe impl Sync for CurlVersionInfoData {}

#[repr(transparent)]
struct SyncCStrPtr(#[allow(dead_code)] *const u8);
unsafe impl Sync for SyncCStrPtr {}

static VERSION_PROTOCOLS: [SyncCStrPtr; 3] = [
    SyncCStrPtr(b"http\0".as_ptr()),
    SyncCStrPtr(b"https\0".as_ptr()),
    SyncCStrPtr(std::ptr::null()),
];

static VERSION_FEATURE_NAMES: [SyncCStrPtr; 5] = [
    SyncCStrPtr(b"IPv6\0".as_ptr()),
    SyncCStrPtr(b"SSL\0".as_ptr()),
    SyncCStrPtr(b"libz\0".as_ptr()),
    SyncCStrPtr(b"Largefile\0".as_ptr()),
    SyncCStrPtr(std::ptr::null()),
];

static VERSION_INFO: CurlVersionInfoData = CurlVersionInfoData {
    age: 10, // CURLVERSION_ELEVENTH
    _pad0: 0,
    version: b"8.7.1\0".as_ptr(),
    version_num: 0x08_07_01,
    _pad1: 0,
    host: b"x86_64-pc-win32\0".as_ptr(),
    features: CURL_VERSION_IPV6 | CURL_VERSION_SSL | CURL_VERSION_LIBZ | CURL_VERSION_LARGEFILE,
    _pad2: 0,
    ssl_version: b"rustls/0.23\0".as_ptr(),
    ssl_version_num: 0,
    _pad3: 0,
    libz_version: b"1.2.12\0".as_ptr(),
    protocols: VERSION_PROTOCOLS.as_ptr() as *const *const u8,
    ares: std::ptr::null(),
    ares_num: 0,
    _pad4: 0,
    libidn: std::ptr::null(),
    iconv_ver_num: 0,
    _pad5: 0,
    libssh_version: std::ptr::null(),
    brotli_ver_num: 0,
    _pad6: 0,
    brotli_version: std::ptr::null(),
    nghttp2_ver_num: 0,
    _pad7: 0,
    nghttp2_version: std::ptr::null(),
    quic_version: std::ptr::null(),
    cainfo: std::ptr::null(),
    capath: std::ptr::null(),
    zstd_ver_num: 0,
    _pad8: 0,
    zstd_version: std::ptr::null(),
    hyper_version: std::ptr::null(),
    gsasl_version: std::ptr::null(),
    feature_names: VERSION_FEATURE_NAMES.as_ptr() as *const *const u8,
};

// ---------------------------------------------------------------------------
// Global handle tables & TLS configuration
// ---------------------------------------------------------------------------

static NEXT_HANDLE: AtomicUsize = AtomicUsize::new(0xC081_0001);

static EASY_HANDLES: LazyLock<Mutex<HashMap<usize, Arc<Mutex<CurlEasy>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static MULTI_HANDLES: LazyLock<Mutex<HashMap<usize, Arc<CurlMulti>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static SHARE_HANDLES: LazyLock<Mutex<HashMap<usize, ()>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static RUSTLS_ROOTS: LazyLock<Arc<RootCertStore>> = LazyLock::new(|| {
    let mut store = RootCertStore::empty();
    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    Arc::new(store)
});

static RUSTLS_PROVIDER: LazyLock<Arc<rustls::crypto::CryptoProvider>> =
    LazyLock::new(|| Arc::new(rustls::crypto::ring::default_provider()));

static TLS_CONFIG_STRICT: LazyLock<Arc<ClientConfig>> = LazyLock::new(|| {
    build_tls_config(true, true)
});

static TLS_CONFIG_NO_HOST: LazyLock<Arc<ClientConfig>> = LazyLock::new(|| {
    build_tls_config(true, false)
});

static TLS_CONFIG_INSECURE: LazyLock<Arc<ClientConfig>> = LazyLock::new(|| {
    build_tls_config(false, false)
});

#[derive(Debug)]
struct CurlCertVerifier {
    inner: Arc<WebPkiServerVerifier>,
    verify_peer: bool,
    verify_host: bool,
}

impl ServerCertVerifier for CurlCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        if !self.verify_peer {
            return Ok(ServerCertVerified::assertion());
        }
        match self
            .inner
            .verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now)
        {
            Ok(v) => Ok(v),
            Err(rustls::Error::InvalidCertificate(
                rustls::CertificateError::NotValidForName
                | rustls::CertificateError::NotValidForNameContext { .. },
            )) if !self.verify_host => Ok(ServerCertVerified::assertion()),
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

fn build_tls_config(verify_peer: bool, verify_host: bool) -> Arc<ClientConfig> {
    let provider = Arc::clone(&RUSTLS_PROVIDER);
    let roots = Arc::clone(&RUSTLS_ROOTS);
    let builder = ClientConfig::builder_with_provider(Arc::clone(&provider))
        .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
        .expect("rustls TLS 1.2/1.3 protocol versions");

    let mut cfg = if verify_peer && verify_host {
        builder.with_root_certificates(roots).with_no_client_auth()
    } else {
        let inner = WebPkiServerVerifier::builder_with_provider(roots, provider)
            .build()
            .expect("WebPkiServerVerifier");
        let verifier = Arc::new(CurlCertVerifier {
            inner,
            verify_peer,
            verify_host,
        });
        builder
            .dangerous()
            .with_custom_certificate_verifier(verifier)
            .with_no_client_auth()
    };
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    Arc::new(cfg)
}

fn tls_config_for(verify_peer: bool, verify_host: bool) -> Arc<ClientConfig> {
    match (verify_peer, verify_host) {
        (true, true) => Arc::clone(&TLS_CONFIG_STRICT),
        (true, false) => Arc::clone(&TLS_CONFIG_NO_HOST),
        (false, _) => Arc::clone(&TLS_CONFIG_INSECURE),
    }
}

// ---------------------------------------------------------------------------
// Easy handle state & configuration snapshot
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct CurlEasyOptions {
    url: Option<String>,
    port: u16,
    user_agent: Option<String>,
    httpheader_ptr: usize,
    post: bool,
    postfields_ptr: usize,
    postfields_copied: Option<Vec<u8>>,
    postfieldsize: i64,
    custom_request: Option<String>,
    nobody: bool,
    upload: bool,
    infilesize: i64,
    follow_location: bool,
    max_redirs: i32,
    postredir: i32,
    timeout_ms: u64,
    connect_timeout_ms: u64,
    write_fn: usize,
    writedata: usize,
    header_fn: usize,
    headerdata: usize,
    read_fn: usize,
    readdata: usize,
    error_buffer: usize,
    private_ptr: usize,
    ssl_verifypeer: bool,
    ssl_verifyhost: bool,
    cainfo: Option<String>,
    accept_encoding: Option<String>,
    http_content_decoding: bool,
    include_header_in_body: bool,
    fail_on_error: bool,
    verbose: bool,
    referer: Option<String>,
    autoreferer: bool,
    cookie: Option<String>,
    range: Option<String>,
    userpwd: Option<String>,
    username: Option<String>,
    password: Option<String>,
    tcp_nodelay: bool,
}

impl Default for CurlEasyOptions {
    fn default() -> Self {
        Self {
            url: None,
            port: 0,
            user_agent: None,
            httpheader_ptr: 0,
            post: false,
            postfields_ptr: 0,
            postfields_copied: None,
            postfieldsize: -1,
            custom_request: None,
            nobody: false,
            upload: false,
            infilesize: -1,
            follow_location: false,
            max_redirs: -1,
            postredir: 0,
            timeout_ms: 0,
            connect_timeout_ms: 0,
            write_fn: 0,
            writedata: 0,
            header_fn: 0,
            headerdata: 0,
            read_fn: 0,
            readdata: 0,
            error_buffer: 0,
            private_ptr: 0,
            ssl_verifypeer: true,
            ssl_verifyhost: true,
            cainfo: None,
            accept_encoding: None,
            http_content_decoding: true,
            include_header_in_body: false,
            fail_on_error: false,
            verbose: false,
            referer: None,
            autoreferer: false,
            cookie: None,
            range: None,
            userpwd: None,
            username: None,
            password: None,
            tcp_nodelay: true,
        }
    }
}

struct CurlEasyInfo {
    response_code: i32,
    header_size: i32,
    request_size: i32,
    redirect_count: i32,
    http_connectcode: i32,
    ssl_verifyresult: i32,
    os_errno: i32,
    num_connects: i32,
    size_download: i64,
    size_upload: i64,
    content_length_download: i64,
    content_length_upload: i64,
    total_time: f64,
    namelookup_time: f64,
    connect_time: f64,
    effective_url_cstr: Option<Box<[u8]>>,
    content_type_cstr: Option<Box<[u8]>>,
    redirect_url_cstr: Option<Box<[u8]>>,
    primary_ip_cstr: Option<Box<[u8]>>,
}

impl Default for CurlEasyInfo {
    fn default() -> Self {
        Self {
            response_code: 0,
            header_size: 0,
            request_size: 0,
            redirect_count: 0,
            http_connectcode: 0,
            ssl_verifyresult: 0,
            os_errno: 0,
            num_connects: 0,
            size_download: 0,
            size_upload: 0,
            content_length_download: -1,
            content_length_upload: -1,
            total_time: 0.0,
            namelookup_time: 0.0,
            connect_time: 0.0,
            effective_url_cstr: None,
            content_type_cstr: None,
            redirect_url_cstr: None,
            primary_ip_cstr: None,
        }
    }
}

struct CurlEasy {
    opts: CurlEasyOptions,
    info: CurlEasyInfo,
    attached_multi: Option<usize>,
}

impl CurlEasy {
    fn new() -> Self {
        Self {
            opts: CurlEasyOptions::default(),
            info: CurlEasyInfo::default(),
            attached_multi: None,
        }
    }

    fn reset(&mut self) {
        self.opts = CurlEasyOptions::default();
        self.info = CurlEasyInfo::default();
    }
}

#[derive(Clone, Copy)]
struct CallbackConfig {
    write_fn: usize,
    writedata: usize,
    header_fn: usize,
    headerdata: usize,
    include_header_in_body: bool,
    error_buffer: usize,
}

struct TransferRequest {
    url: String,
    port_override: u16,
    method: String,
    explicit_custom_method: bool,
    nobody: bool,
    body: Vec<u8>,
    is_post_default_ct: bool,
    custom_headers: Vec<String>,
    user_agent: Option<String>,
    referer: Option<String>,
    autoreferer: bool,
    cookie: Option<String>,
    range: Option<String>,
    basic_auth: Option<String>,
    accept_encoding: Option<String>,
    http_content_decoding: bool,
    follow_location: bool,
    max_redirs: i32,
    postredir: i32,
    fail_on_error: bool,
    verbose: bool,
    timeout_ms: u64,
    connect_timeout_ms: u64,
    ssl_verifypeer: bool,
    ssl_verifyhost: bool,
    tcp_nodelay: bool,
}

#[derive(Clone)]
struct HeadMeta {
    response_code: i32,
    header_size: i32,
    request_size: i32,
    redirect_count: i32,
    num_connects: i32,
    content_length_download: i64,
    effective_url: String,
    content_type: Option<String>,
    redirect_url: Option<String>,
    primary_ip: Option<String>,
    namelookup_time: f64,
    connect_time: f64,
}

struct TransferOutcome {
    code: i32,
    error_msg: Option<String>,
    size_download: i64,
    size_upload: i64,
    content_length_upload: i64,
    total_time: f64,
    os_errno: i32,
    ssl_verifyresult: i32,
}

fn make_cstr_box(s: &str) -> Box<[u8]> {
    let mut v = Vec::with_capacity(s.len() + 1);
    v.extend_from_slice(s.as_bytes());
    v.push(0);
    v.into_boxed_slice()
}

fn apply_head_meta(easy: &Arc<Mutex<CurlEasy>>, meta: HeadMeta) {
    let mut e = easy.lock().unwrap();
    e.info.response_code = meta.response_code;
    e.info.header_size = meta.header_size;
    e.info.request_size = meta.request_size;
    e.info.redirect_count = meta.redirect_count;
    e.info.num_connects = meta.num_connects;
    e.info.content_length_download = meta.content_length_download;
    e.info.effective_url_cstr = Some(make_cstr_box(&meta.effective_url));
    e.info.content_type_cstr = meta.content_type.as_deref().map(make_cstr_box);
    e.info.redirect_url_cstr = meta.redirect_url.as_deref().map(make_cstr_box);
    e.info.primary_ip_cstr = meta.primary_ip.as_deref().map(make_cstr_box);
    e.info.namelookup_time = meta.namelookup_time;
    e.info.connect_time = meta.connect_time;
}

unsafe fn write_error_buffer(buf: usize, msg: &str) {
    if buf == 0 {
        return;
    }
    let dst = buf as *mut u8;
    let bytes = msg.as_bytes();
    let n = bytes.len().min(CURL_ERROR_SIZE - 1);
    if n > 0 {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, n);
    }
    std::ptr::write_unaligned(dst.add(n), 0);
}

unsafe fn apply_outcome(easy: &Arc<Mutex<CurlEasy>>, cb: CallbackConfig, outcome: &TransferOutcome) {
    {
        let mut e = easy.lock().unwrap();
        e.info.size_download = outcome.size_download;
        e.info.size_upload = outcome.size_upload;
        e.info.content_length_upload = outcome.content_length_upload;
        e.info.total_time = outcome.total_time;
        e.info.os_errno = outcome.os_errno;
        e.info.ssl_verifyresult = outcome.ssl_verifyresult;
    }
    if outcome.code != CURLE_OK {
        if let Some(ref msg) = outcome.error_msg {
            write_error_buffer(cb.error_buffer, msg);
        }
    }
}

unsafe fn read_slist_strings(mut list: *const CurlSlist) -> Vec<String> {
    let mut out = Vec::new();
    while !list.is_null() {
        let data = std::ptr::read_unaligned(std::ptr::addr_of!((*list).data));
        let next = std::ptr::read_unaligned(std::ptr::addr_of!((*list).next));
        if !data.is_null() {
            out.push(crate::strings::from_ansi(data));
        }
        list = next;
    }
    out
}

unsafe fn prepare_transfer(
    easy: &Arc<Mutex<CurlEasy>>,
) -> Result<(TransferRequest, CallbackConfig), (i32, String, CallbackConfig)> {
    let opts = {
        let mut e = easy.lock().unwrap();
        e.info = CurlEasyInfo::default();
        if let Some(ref u) = e.opts.url {
            e.info.effective_url_cstr = Some(make_cstr_box(u));
        }
        e.opts.clone()
    };

    let cb = CallbackConfig {
        write_fn: opts.write_fn,
        writedata: opts.writedata,
        header_fn: opts.header_fn,
        headerdata: opts.headerdata,
        include_header_in_body: opts.include_header_in_body,
        error_buffer: opts.error_buffer,
    };

    if cb.error_buffer != 0 {
        std::ptr::write_unaligned(cb.error_buffer as *mut u8, 0);
    }

    if let Some(ref ca) = opts.cainfo {
        log::debug!("libcurl: CURLOPT_CAINFO={ca:?} (using embedded webpki-roots bundle)");
    }

    let Some(url) = opts.url.filter(|u| !u.is_empty()) else {
        return Err((CURLE_URL_MALFORMAT, "No URL set".to_string(), cb));
    };

    let custom_headers = if opts.httpheader_ptr != 0 {
        read_slist_strings(opts.httpheader_ptr as *const CurlSlist)
    } else {
        Vec::new()
    };

    let (method, explicit_custom_method) = if let Some(ref custom) = opts.custom_request {
        if !custom.is_empty() {
            (custom.clone(), true)
        } else if opts.nobody {
            ("HEAD".to_string(), false)
        } else if opts.upload {
            ("PUT".to_string(), false)
        } else if opts.post {
            ("POST".to_string(), false)
        } else {
            ("GET".to_string(), false)
        }
    } else if opts.nobody {
        ("HEAD".to_string(), false)
    } else if opts.upload {
        ("PUT".to_string(), false)
    } else if opts.post {
        ("POST".to_string(), false)
    } else {
        ("GET".to_string(), false)
    };

    // Snapshot upload body on the caller's thread (so guest pointers and READFUNCTION
    // are always accessed on the calling Windows thread).
    let body = if let Some(ref copied) = opts.postfields_copied {
        copied.clone()
    } else if opts.postfields_ptr != 0 {
        let len = if opts.postfieldsize >= 0 {
            opts.postfieldsize as usize
        } else {
            crate::strings::strlen(opts.postfields_ptr as *const u8)
        };
        if len == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(opts.postfields_ptr as *const u8, len).to_vec()
        }
    } else if opts.read_fn != 0
        && (opts.upload
            || opts.post
            || matches!(method.as_str(), "PUT" | "POST" | "PATCH"))
    {
        match pull_read_callback(opts.read_fn, opts.readdata, opts.infilesize) {
            Ok(data) => data,
            Err((code, msg)) => return Err((code, msg, cb)),
        }
    } else {
        Vec::new()
    };

    let basic_auth = if let Some(ref up) = opts.userpwd {
        Some(up.clone())
    } else if opts.username.is_some() || opts.password.is_some() {
        Some(format!(
            "{}:{}",
            opts.username.as_deref().unwrap_or(""),
            opts.password.as_deref().unwrap_or("")
        ))
    } else {
        None
    };

    let is_post_default_ct = opts.post && !opts.upload && !opts.nobody;

    Ok((
        TransferRequest {
            url,
            port_override: opts.port,
            method,
            explicit_custom_method,
            nobody: opts.nobody,
            body,
            is_post_default_ct,
            custom_headers,
            user_agent: opts.user_agent,
            referer: opts.referer,
            autoreferer: opts.autoreferer,
            cookie: opts.cookie,
            range: opts.range,
            basic_auth,
            accept_encoding: opts.accept_encoding,
            http_content_decoding: opts.http_content_decoding,
            follow_location: opts.follow_location,
            max_redirs: opts.max_redirs,
            postredir: opts.postredir,
            fail_on_error: opts.fail_on_error,
            verbose: opts.verbose,
            timeout_ms: opts.timeout_ms,
            connect_timeout_ms: opts.connect_timeout_ms,
            ssl_verifypeer: opts.ssl_verifypeer,
            ssl_verifyhost: opts.ssl_verifyhost,
            tcp_nodelay: opts.tcp_nodelay,
        },
        cb,
    ))
}

unsafe fn pull_read_callback(
    read_fn: usize,
    readdata: usize,
    infilesize: i64,
) -> Result<Vec<u8>, (i32, String)> {
    let mut out = if infilesize > 0 {
        Vec::with_capacity((infilesize as usize).min(1024 * 1024))
    } else {
        Vec::new()
    };
    let mut chunk = vec![0u8; 16384];
    loop {
        let cap = if infilesize >= 0 {
            let rem = (infilesize as usize).saturating_sub(out.len());
            if rem == 0 {
                break;
            }
            rem.min(chunk.len())
        } else {
            chunk.len()
        };
        let n = crate::call_guest8(
            read_fn,
            chunk.as_mut_ptr() as u64,
            1,
            cap as u64,
            readdata as u64,
            0,
            0,
            0,
            0,
        ) as usize;
        if n == 0 {
            break;
        }
        if n == CURL_READFUNC_ABORT {
            return Err((
                CURLE_ABORTED_BY_CALLBACK,
                "Operation aborted by read callback".to_string(),
            ));
        }
        if n > cap {
            return Err((
                CURLE_READ_ERROR,
                format!("Read callback returned {n} > buffer capacity {cap}"),
            ));
        }
        out.extend_from_slice(&chunk[..n]);
    }
    Ok(out)
}

unsafe fn invoke_header_callback(cb: CallbackConfig, line: &[u8]) -> Result<(), i32> {
    if line.is_empty() {
        return Ok(());
    }
    if cb.header_fn != 0 {
        let written = crate::call_guest8(
            cb.header_fn,
            line.as_ptr() as u64,
            1,
            line.len() as u64,
            cb.headerdata as u64,
            0,
            0,
            0,
            0,
        ) as usize;
        if written != line.len() {
            return Err(CURLE_WRITE_ERROR);
        }
    }
    if cb.include_header_in_body && cb.write_fn != 0 {
        let written = crate::call_guest8(
            cb.write_fn,
            line.as_ptr() as u64,
            1,
            line.len() as u64,
            cb.writedata as u64,
            0,
            0,
            0,
            0,
        ) as usize;
        if written != line.len() {
            return Err(CURLE_WRITE_ERROR);
        }
    }
    Ok(())
}

unsafe fn invoke_write_callback(cb: CallbackConfig, data: &[u8]) -> Result<(), i32> {
    if data.is_empty() || cb.write_fn == 0 {
        return Ok(());
    }
    let written = crate::call_guest8(
        cb.write_fn,
        data.as_ptr() as u64,
        1,
        data.len() as u64,
        cb.writedata as u64,
        0,
        0,
        0,
        0,
    ) as usize;
    if written != data.len() {
        return Err(CURLE_WRITE_ERROR);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Offline check, URL parsing, redirect resolution, Base64, zlib decompression
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
        format!("{}:{}", self.formatted_host(), self.effective_port(port_override))
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

fn normalize_path(path: &str) -> String {
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

fn resolve_redirect(base: &ParsedUrl, location: &str) -> Result<ParsedUrl, (i32, String)> {
    let loc = location.trim();
    if loc.is_empty() {
        return Err((CURLE_URL_MALFORMAT, "Empty redirect Location".to_string()));
    }
    if loc.contains("://") {
        return parse_url(loc);
    }
    if let Some(rest) = loc.strip_prefix("//") {
        return parse_url(&format!("{}://{rest}", base.scheme));
    }
    if loc.starts_with('/') {
        return parse_url(&format!(
            "{}://{}{loc}",
            base.scheme,
            base.authority_for_redirect()
        ));
    }
    if loc.starts_with('?') {
        let base_path = base
            .path_and_query
            .split('?')
            .next()
            .unwrap_or("/");
        return parse_url(&format!(
            "{}://{}{base_path}{loc}",
            base.scheme,
            base.authority_for_redirect()
        ));
    }
    let base_path = base.path_and_query.split('?').next().unwrap_or("/");
    let dir = match base_path.rfind('/') {
        Some(idx) => &base_path[..=idx],
        None => "/",
    };
    let joined = normalize_path(&format!("{dir}{loc}"));
    parse_url(&format!(
        "{}://{}{joined}",
        base.scheme,
        base.authority_for_redirect()
    ))
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

// Streaming gzip/deflate decoder backed by Apple system libz (shared with zlib1.rs).
struct StreamingInflater {
    strm: super::zlib1::MacZStream,
    initialized: bool,
    finished: bool,
    produced_any: bool,
    raw_deflate_fallback: bool,
}

impl StreamingInflater {
    fn new(encoding: &str) -> Result<Self, i32> {
        let (window_bits, raw_fallback) = match encoding {
            "gzip" | "x-gzip" => (15 + 16, false),
            "deflate" => (15, true),
            _ => return Err(CURLE_BAD_CONTENT_ENCODING),
        };
        let mut strm: super::zlib1::MacZStream = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            super::zlib1::host_inflateInit2_(
                &mut strm,
                window_bits,
                super::zlib1::host_zlibVersion(),
                std::mem::size_of::<super::zlib1::MacZStream>() as i32,
            )
        };
        if rc != 0 {
            return Err(CURLE_BAD_CONTENT_ENCODING);
        }
        Ok(Self {
            strm,
            initialized: true,
            finished: false,
            produced_any: false,
            raw_deflate_fallback: raw_fallback,
        })
    }

    fn decompress(&mut self, input: &[u8], out: &mut Vec<u8>) -> Result<(), i32> {
        const Z_OK: i32 = 0;
        const Z_STREAM_END: i32 = 1;
        const Z_NO_FLUSH: i32 = 0;
        const Z_DATA_ERROR: i32 = -3;
        const Z_BUF_ERROR: i32 = -5;

        if input.is_empty() || self.finished {
            return Ok(());
        }

        let mut offset = 0usize;
        let mut scratch = [0u8; 16384];
        while offset < input.len() {
            let slice_len = (input.len() - offset).min(u32::MAX as usize);
            self.strm.next_in = input[offset..].as_ptr();
            self.strm.avail_in = slice_len as u32;

            loop {
                self.strm.next_out = scratch.as_mut_ptr();
                self.strm.avail_out = scratch.len() as u32;
                let rc = unsafe { super::zlib1::host_inflate(&mut self.strm, Z_NO_FLUSH) };
                let produced = scratch.len() - self.strm.avail_out as usize;
                if produced > 0 {
                    out.extend_from_slice(&scratch[..produced]);
                    self.produced_any = true;
                }
                if rc == Z_STREAM_END {
                    self.finished = true;
                    break;
                }
                if rc == Z_DATA_ERROR && self.raw_deflate_fallback && !self.produced_any {
                    // Retry as raw RFC 1951 deflate (-15).
                    unsafe {
                        super::zlib1::host_inflateEnd(&mut self.strm);
                        self.strm = std::mem::zeroed();
                        let init_rc = super::zlib1::host_inflateInit2_(
                            &mut self.strm,
                            -15,
                            super::zlib1::host_zlibVersion(),
                            std::mem::size_of::<super::zlib1::MacZStream>() as i32,
                        );
                        if init_rc != Z_OK {
                            self.initialized = false;
                            return Err(CURLE_BAD_CONTENT_ENCODING);
                        }
                    }
                    self.raw_deflate_fallback = false;
                    self.strm.next_in = input[offset..].as_ptr();
                    self.strm.avail_in = slice_len as u32;
                    continue;
                }
                if rc == Z_BUF_ERROR && self.strm.avail_in == 0 {
                    break;
                }
                if rc != Z_OK && rc != Z_BUF_ERROR {
                    self.strm.next_in = std::ptr::null();
                    self.strm.avail_in = 0;
                    self.strm.next_out = std::ptr::null_mut();
                    self.strm.avail_out = 0;
                    return Err(CURLE_BAD_CONTENT_ENCODING);
                }
                if self.strm.avail_in == 0 && produced == 0 {
                    break;
                }
            }

            let consumed = slice_len - self.strm.avail_in as usize;
            offset += consumed;
            if self.finished || consumed == 0 {
                break;
            }
        }

        self.strm.next_in = std::ptr::null();
        self.strm.avail_in = 0;
        self.strm.next_out = std::ptr::null_mut();
        self.strm.avail_out = 0;
        Ok(())
    }
}

impl Drop for StreamingInflater {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                super::zlib1::host_inflateEnd(&mut self.strm);
            }
            self.initialized = false;
        }
    }
}

// ---------------------------------------------------------------------------
// HTTP/1.1 + TLS transport engine
// ---------------------------------------------------------------------------

enum HttpStream {
    Plain(TcpStream),
    Tls(rustls::StreamOwned<ClientConnection, TcpStream>),
}

impl HttpStream {
    fn is_tls(&self) -> bool {
        matches!(self, Self::Tls(_))
    }

    fn set_timeouts(&self, read_to: Option<Duration>, write_to: Option<Duration>) {
        let tcp = match self {
            Self::Plain(s) => s,
            Self::Tls(s) => &s.sock,
        };
        let _ = tcp.set_read_timeout(read_to);
        let _ = tcp.set_write_timeout(write_to);
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

trait TransferSink {
    fn is_cancelled(&self) -> bool;
    fn on_head(&mut self, meta: HeadMeta);
    fn on_header_line(&mut self, line: Vec<u8>) -> Result<(), i32>;
    fn on_body_chunk(&mut self, data: Vec<u8>) -> Result<(), i32>;
}

struct DirectSink {
    easy: Arc<Mutex<CurlEasy>>,
    cb: CallbackConfig,
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

fn read_line_limited<R: BufRead>(
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
                    "HTTP header line exceeds maximum length",
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
                "HTTP header line exceeds maximum length",
            ));
        }
        out.extend_from_slice(available);
        reader.consume(len);
        total += len;
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

    let has_custom = |k: &str| {
        provided.iter().any(|x| x == k) || suppressed.iter().any(|x| x == k)
    };

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
            let encoded = base64_encode(creds.as_bytes());
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
    let n = read_line_limited(reader, &mut line_buf, MAX_HEADER_LINE)
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
        let n = read_line_limited(reader, &mut line_buf, MAX_HEADER_LINE)
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

fn execute_http_transfer(req: TransferRequest, sink: &mut dyn TransferSink) -> TransferOutcome {
    let start = Instant::now();
    let deadline = if req.timeout_ms > 0 {
        Some(start + Duration::from_millis(req.timeout_ms))
    } else {
        None
    };

    if is_offline() {
        log::debug!("libcurl: PORT_OFFLINE=1 -> CURLE_COULDNT_CONNECT for {}", req.url);
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

        // 3. Optional TLS handshake
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
            let tls_cfg = tls_config_for(req.ssl_verifypeer, req.ssl_verifyhost);
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
            HttpStream::Tls(rustls::StreamOwned::new(conn, tcp))
        } else {
            HttpStream::Plain(tcp)
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
        stream.set_timeouts(io_timeout, io_timeout);

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
            read_chunked_body(&mut reader, sink, &mut inflater, &mut total_size_download, deadline)
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
    let mut line_buf = Vec::with_capacity(64);
    let mut data_buf = vec![0u8; 16384];

    loop {
        if sink.is_cancelled() {
            return Err((CURLE_ABORTED_BY_CALLBACK, "Transfer cancelled".to_string(), 0));
        }
        let rem = remaining_timeout(deadline)?;
        reader.get_ref().set_timeouts(rem, rem);

        let n = read_line_limited(reader, &mut line_buf, 8192)
            .map_err(|e| map_io_err(&e, CURLE_RECV_ERROR))?;
        if n == 0 {
            return Err((
                CURLE_PARTIAL_FILE,
                "Unexpected EOF reading chunk size".to_string(),
                0,
            ));
        }

        let line_str = String::from_utf8_lossy(&line_buf);
        let size_part = line_str
            .trim_end_matches(['\r', '\n'])
            .split(';')
            .next()
            .unwrap_or("")
            .trim();
        let chunk_size = usize::from_str_radix(size_part, 16).map_err(|_| {
            (
                CURLE_RECV_ERROR,
                format!("Invalid chunk size line: {size_part:?}"),
                0,
            )
        })?;

        if chunk_size == 0 {
            // Drain optional trailer headers until terminating blank line or EOF.
            loop {
                match read_line_limited(reader, &mut line_buf, 8192) {
                    Ok(0) => break,
                    Ok(_) => {
                        let t = line_buf
                            .strip_suffix(b"\r\n")
                            .or_else(|| line_buf.strip_suffix(b"\n"))
                            .unwrap_or(&line_buf);
                        if t.is_empty() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            return Ok(());
        }

        let mut remaining = chunk_size;
        while remaining > 0 {
            if sink.is_cancelled() {
                return Err((CURLE_ABORTED_BY_CALLBACK, "Transfer cancelled".to_string(), 0));
            }
            let rem = remaining_timeout(deadline)?;
            reader.get_ref().set_timeouts(rem, rem);

            let want = remaining.min(data_buf.len());
            let r = match reader.read(&mut data_buf[..want]) {
                Ok(0) => {
                    return Err((
                        CURLE_PARTIAL_FILE,
                        "Unexpected EOF inside HTTP chunk".to_string(),
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

        // Consume trailing CRLF after chunk payload
        let _ = read_line_limited(reader, &mut line_buf, 64)
            .map_err(|e| map_io_err(&e, CURLE_RECV_ERROR))?;
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
            return Err((CURLE_ABORTED_BY_CALLBACK, "Transfer cancelled".to_string(), 0));
        }
        let rem = remaining_timeout(deadline)?;
        reader.get_ref().set_timeouts(rem, rem);

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
            return Err((CURLE_ABORTED_BY_CALLBACK, "Transfer cancelled".to_string(), 0));
        }
        let rem = remaining_timeout(deadline)?;
        reader.get_ref().set_timeouts(rem, rem);

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

// ---------------------------------------------------------------------------
// CURLM multi handle runtime
// ---------------------------------------------------------------------------

enum TransferEvent {
    Head(HeadMeta),
    HeaderLine(Vec<u8>),
    BodyChunk(Vec<u8>),
}

#[derive(Default)]
struct TransferChannel {
    events: VecDeque<TransferEvent>,
    finished: Option<TransferOutcome>,
}

struct MultiNotify {
    woken: bool,
    seq: u64,
}

struct CurlMulti {
    inner: Mutex<CurlMultiInner>,
    notify_lock: Mutex<MultiNotify>,
    notify_cv: Condvar,
}

impl CurlMulti {
    fn new() -> Self {
        Self {
            inner: Mutex::new(CurlMultiInner {
                entries: HashMap::new(),
                done_queue: VecDeque::new(),
                delivered_msgs: Vec::new(),
            }),
            notify_lock: Mutex::new(MultiNotify { woken: false, seq: 0 }),
            notify_cv: Condvar::new(),
        }
    }

    fn signal_activity(&self) {
        let mut n = self.notify_lock.lock().unwrap();
        n.seq = n.seq.wrapping_add(1);
        self.notify_cv.notify_all();
    }

    fn wakeup(&self) {
        let mut n = self.notify_lock.lock().unwrap();
        n.woken = true;
        n.seq = n.seq.wrapping_add(1);
        self.notify_cv.notify_all();
    }
}

enum MultiEntryState {
    Pending,
    Starting,
    Running {
        channel: Arc<Mutex<TransferChannel>>,
        cancel: Arc<AtomicBool>,
        cb: CallbackConfig,
    },
    Done,
}

struct MultiEntry {
    easy: Arc<Mutex<CurlEasy>>,
    state: MultiEntryState,
}

struct CurlMultiInner {
    entries: HashMap<usize, MultiEntry>,
    done_queue: VecDeque<Box<CurlMsg>>,
    delivered_msgs: Vec<Box<CurlMsg>>,
}

struct MultiWorkerSink {
    channel: Arc<Mutex<TransferChannel>>,
    cancel: Arc<AtomicBool>,
    multi: Arc<CurlMulti>,
}

impl TransferSink for MultiWorkerSink {
    fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Acquire)
    }

    fn on_head(&mut self, meta: HeadMeta) {
        {
            let mut ch = self.channel.lock().unwrap();
            ch.events.push_back(TransferEvent::Head(meta));
        }
        self.multi.signal_activity();
    }

    fn on_header_line(&mut self, line: Vec<u8>) -> Result<(), i32> {
        if self.is_cancelled() {
            return Err(CURLE_ABORTED_BY_CALLBACK);
        }
        {
            let mut ch = self.channel.lock().unwrap();
            ch.events.push_back(TransferEvent::HeaderLine(line));
        }
        self.multi.signal_activity();
        Ok(())
    }

    fn on_body_chunk(&mut self, data: Vec<u8>) -> Result<(), i32> {
        if self.is_cancelled() {
            return Err(CURLE_ABORTED_BY_CALLBACK);
        }
        {
            let mut ch = self.channel.lock().unwrap();
            ch.events.push_back(TransferEvent::BodyChunk(data));
        }
        self.multi.signal_activity();
        Ok(())
    }
}

fn multi_wait_impl(
    multi_id: usize,
    extra_nfds: u32,
    timeout_ms: i32,
    numfds: *mut i32,
    is_poll: bool,
) -> i32 {
    let Some(multi) = MULTI_HANDLES.lock().unwrap().get(&multi_id).cloned() else {
        if !numfds.is_null() {
            unsafe { std::ptr::write_unaligned(numfds, 0) };
        }
        return CURLM_BAD_HANDLE;
    };

    let start_seq = multi.notify_lock.lock().unwrap().seq;

    let (has_active, has_ready) = {
        let inner = multi.inner.lock().unwrap();
        let mut active = false;
        // Only active transfer readiness (Pending start or undrained worker events/completion)
        // wakes curl_multi_poll/wait immediately; unread messages already in done_queue must NOT
        // busy-spin callers looping `while running > 0 { perform(); poll(); }`.
        let mut ready = false;
        for entry in inner.entries.values() {
            match &entry.state {
                MultiEntryState::Pending => {
                    active = true;
                    ready = true;
                }
                MultiEntryState::Starting => {
                    active = true;
                }
                MultiEntryState::Running { channel, .. } => {
                    active = true;
                    let ch = channel.lock().unwrap();
                    if !ch.events.is_empty() || ch.finished.is_some() {
                        ready = true;
                    }
                }
                MultiEntryState::Done => {}
            }
        }
        (active, ready)
    };

    if has_ready {
        if !numfds.is_null() {
            unsafe { std::ptr::write_unaligned(numfds, 1) };
        }
        return CURLM_OK;
    }

    // Documented curl_multi_wait behavior: returns immediately when no active transfers
    // and no extra_fds exist, whereas curl_multi_poll still sleeps up to timeout_ms.
    if !is_poll && !has_active && extra_nfds == 0 {
        if !numfds.is_null() {
            unsafe { std::ptr::write_unaligned(numfds, 0) };
        }
        return CURLM_OK;
    }

    let timeout = if timeout_ms <= 0 {
        Duration::ZERO
    } else {
        Duration::from_millis(timeout_ms as u64)
    };
    let deadline = Instant::now() + timeout;

    let mut guard = multi.notify_lock.lock().unwrap();
    let mut activity = false;
    loop {
        if guard.woken {
            guard.woken = false;
            activity = true;
            break;
        }
        if guard.seq != start_seq {
            activity = true;
            break;
        }
        let now = Instant::now();
        if now >= deadline {
            break;
        }
        let (new_guard, wait_res) = multi
            .notify_cv
            .wait_timeout(guard, deadline.saturating_duration_since(now))
            .unwrap();
        guard = new_guard;
        if wait_res.timed_out() && !guard.woken && guard.seq == start_seq {
            break;
        }
    }

    if !numfds.is_null() {
        unsafe { std::ptr::write_unaligned(numfds, i32::from(activity)) };
    }
    CURLM_OK
}

// ---------------------------------------------------------------------------
// Exported Win64 `libcurl.dll` API
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn curl_global_init(flags: i64) -> i32 {
    log::info!("libcurl (builtin): curl_global_init(flags={flags:#x}) -> CURLE_OK");
    CURLE_OK
}

pub unsafe extern "win64" fn curl_global_init_mem(
    flags: i64,
    _m: usize,
    _f: usize,
    _r: usize,
    _s: usize,
    _c: usize,
) -> i32 {
    log::info!("libcurl (builtin): curl_global_init_mem(flags={flags:#x}) -> CURLE_OK");
    CURLE_OK
}

pub unsafe extern "win64" fn curl_global_cleanup() {}

pub unsafe extern "win64" fn curl_version() -> *const u8 {
    b"libcurl/8.7.1 rustls/0.23 zlib/1.2.12\0".as_ptr()
}

pub unsafe extern "win64" fn curl_version_info(_age: i32) -> *const CurlVersionInfoData {
    &VERSION_INFO
}

pub unsafe extern "win64" fn curl_easy_init() -> usize {
    let id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    EASY_HANDLES
        .lock()
        .unwrap()
        .insert(id, Arc::new(Mutex::new(CurlEasy::new())));
    id
}

pub unsafe extern "win64" fn curl_easy_duphandle(easy: usize) -> usize {
    let Some(src) = EASY_HANDLES.lock().unwrap().get(&easy).cloned() else {
        return 0;
    };
    let opts = src.lock().unwrap().opts.clone();
    let new_id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    let mut dup = CurlEasy::new();
    dup.opts = opts;
    EASY_HANDLES
        .lock()
        .unwrap()
        .insert(new_id, Arc::new(Mutex::new(dup)));
    new_id
}

pub unsafe extern "win64" fn curl_easy_cleanup(easy: usize) {
    if easy == 0 {
        return;
    }
    let Some(handle) = EASY_HANDLES.lock().unwrap().remove(&easy) else {
        return;
    };
    let attached = handle.lock().unwrap().attached_multi.take();
    if let Some(multi_id) = attached {
        if let Some(multi) = MULTI_HANDLES.lock().unwrap().get(&multi_id).cloned() {
            let mut inner = multi.inner.lock().unwrap();
            if let Some(entry) = inner.entries.remove(&easy) {
                if let MultiEntryState::Running { cancel, .. } = entry.state {
                    cancel.store(true, Ordering::Release);
                }
            }
            inner.done_queue.retain(|m| m.easy_handle != easy);
            inner.delivered_msgs.retain(|m| m.easy_handle != easy);
        }
    }
}

pub unsafe extern "win64" fn curl_easy_reset(easy: usize) {
    if let Some(handle) = EASY_HANDLES.lock().unwrap().get(&easy).cloned() {
        handle.lock().unwrap().reset();
    }
}

pub unsafe extern "win64" fn curl_easy_setopt(easy: usize, option: i32, param: usize) -> i32 {
    let Some(handle) = EASY_HANDLES.lock().unwrap().get(&easy).cloned() else {
        return CURLE_BAD_FUNCTION_ARGUMENT;
    };
    let long_val = param as i32;
    let off_val = param as i64;
    let mut guard = handle.lock().unwrap();
    let opts = &mut guard.opts;

    match option {
        CURLOPT_URL => {
            opts.url = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_PORT => {
            opts.port = if long_val > 0 && long_val <= u16::MAX as i32 {
                long_val as u16
            } else {
                0
            };
        }
        CURLOPT_USERAGENT => {
            opts.user_agent = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_HTTPHEADER => {
            opts.httpheader_ptr = param;
        }
        CURLOPT_POST => {
            opts.post = long_val != 0;
            if opts.post {
                opts.nobody = false;
                opts.upload = false;
            }
        }
        CURLOPT_POSTFIELDS => {
            opts.postfields_copied = None;
            opts.postfields_ptr = param;
            if param != 0 {
                opts.post = true;
                opts.nobody = false;
                opts.upload = false;
            }
        }
        CURLOPT_COPYPOSTFIELDS => {
            opts.postfields_ptr = 0;
            if param == 0 {
                opts.postfields_copied = None;
            } else {
                let len = if opts.postfieldsize >= 0 {
                    opts.postfieldsize as usize
                } else {
                    crate::strings::strlen(param as *const u8)
                };
                let data = if len == 0 {
                    Vec::new()
                } else {
                    std::slice::from_raw_parts(param as *const u8, len).to_vec()
                };
                opts.postfields_copied = Some(data);
                opts.post = true;
                opts.nobody = false;
                opts.upload = false;
            }
        }
        CURLOPT_POSTFIELDSIZE => {
            opts.postfieldsize = long_val as i64;
        }
        CURLOPT_POSTFIELDSIZE_LARGE => {
            opts.postfieldsize = off_val;
        }
        CURLOPT_CUSTOMREQUEST => {
            opts.custom_request = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_NOBODY => {
            opts.nobody = long_val != 0;
        }
        CURLOPT_UPLOAD | CURLOPT_PUT => {
            opts.upload = long_val != 0;
            if opts.upload {
                opts.post = false;
                opts.nobody = false;
            }
        }
        CURLOPT_HTTPGET => {
            if long_val != 0 {
                opts.post = false;
                opts.nobody = false;
                opts.upload = false;
                opts.custom_request = None;
            }
        }
        CURLOPT_INFILESIZE => {
            opts.infilesize = long_val as i64;
        }
        CURLOPT_INFILESIZE_LARGE => {
            opts.infilesize = off_val;
        }
        CURLOPT_FOLLOWLOCATION => {
            opts.follow_location = long_val != 0;
        }
        CURLOPT_MAXREDIRS => {
            opts.max_redirs = long_val;
        }
        CURLOPT_POSTREDIR => {
            opts.postredir = long_val;
        }
        CURLOPT_TIMEOUT => {
            opts.timeout_ms = if long_val > 0 {
                (long_val as u64).saturating_mul(1000)
            } else {
                0
            };
        }
        CURLOPT_TIMEOUT_MS => {
            opts.timeout_ms = if long_val > 0 { long_val as u64 } else { 0 };
        }
        CURLOPT_CONNECTTIMEOUT => {
            opts.connect_timeout_ms = if long_val > 0 {
                (long_val as u64).saturating_mul(1000)
            } else {
                0
            };
        }
        CURLOPT_CONNECTTIMEOUT_MS => {
            opts.connect_timeout_ms = if long_val > 0 { long_val as u64 } else { 0 };
        }
        CURLOPT_WRITEFUNCTION => {
            opts.write_fn = param;
        }
        CURLOPT_WRITEDATA => {
            opts.writedata = param;
        }
        CURLOPT_HEADERFUNCTION => {
            opts.header_fn = param;
        }
        CURLOPT_HEADERDATA => {
            opts.headerdata = param;
        }
        CURLOPT_READFUNCTION => {
            opts.read_fn = param;
        }
        CURLOPT_READDATA => {
            opts.readdata = param;
        }
        CURLOPT_ERRORBUFFER => {
            opts.error_buffer = param;
        }
        CURLOPT_PRIVATE => {
            opts.private_ptr = param;
        }
        CURLOPT_SSL_VERIFYPEER => {
            opts.ssl_verifypeer = long_val != 0;
        }
        CURLOPT_SSL_VERIFYHOST => {
            opts.ssl_verifyhost = long_val != 0;
        }
        CURLOPT_CAINFO => {
            opts.cainfo = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_ACCEPT_ENCODING => {
            opts.accept_encoding = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_HTTP_CONTENT_DECODING => {
            opts.http_content_decoding = long_val != 0;
        }
        CURLOPT_HEADER => {
            opts.include_header_in_body = long_val != 0;
        }
        CURLOPT_FAILONERROR => {
            opts.fail_on_error = long_val != 0;
        }
        CURLOPT_VERBOSE => {
            opts.verbose = long_val != 0;
        }
        CURLOPT_REFERER => {
            opts.referer = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_AUTOREFERER => {
            opts.autoreferer = long_val != 0;
        }
        CURLOPT_COOKIE => {
            opts.cookie = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_RANGE => {
            opts.range = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_USERPWD => {
            opts.userpwd = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_USERNAME => {
            opts.username = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_PASSWORD => {
            opts.password = crate::strings::from_ansi_opt(param as *const u8);
        }
        CURLOPT_TCP_NODELAY => {
            opts.tcp_nodelay = long_val != 0;
        }
        CURLOPT_SSL_CTX_FUNCTION => {
            // Matches Windows Schannel libcurl: SSL_CTX_FUNCTION is OpenSSL-only.
            return CURLE_NOT_BUILT_IN;
        }
        CURLOPT_NOSIGNAL
        | CURLOPT_NOPROGRESS
        | CURLOPT_HTTP_VERSION
        | CURLOPT_DNS_CACHE_TIMEOUT
        | CURLOPT_BUFFERSIZE
        | CURLOPT_FRESH_CONNECT
        | CURLOPT_FORBID_REUSE
        | CURLOPT_MAXCONNECTS
        | CURLOPT_LOW_SPEED_LIMIT
        | CURLOPT_LOW_SPEED_TIME
        | CURLOPT_SSLVERSION
        | CURLOPT_IPRESOLVE
        | CURLOPT_SSL_OPTIONS
        | CURLOPT_SSL_SESSIONID_CACHE
        | CURLOPT_TCP_KEEPALIVE
        | CURLOPT_TCP_KEEPIDLE
        | CURLOPT_TCP_KEEPINTVL
        | CURLOPT_EXPECT_100_TIMEOUT_MS
        | CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS
        | CURLOPT_PIPEWAIT
        | CURLOPT_PATH_AS_IS
        | CURLOPT_HTTP_TRANSFER_DECODING
        | CURLOPT_PROTOCOLS
        | CURLOPT_REDIR_PROTOCOLS
        | CURLOPT_PROTOCOLS_STR
        | CURLOPT_REDIR_PROTOCOLS_STR
        | CURLOPT_SHARE
        | CURLOPT_CAPATH
        | CURLOPT_COOKIEFILE
        | CURLOPT_COOKIEJAR
        | CURLOPT_PROXY
        | CURLOPT_PROXYPORT
        | CURLOPT_PROXYUSERPWD
        | CURLOPT_NOPROXY
        | CURLOPT_HTTPPROXYTUNNEL
        | CURLOPT_MAX_SEND_SPEED_LARGE
        | CURLOPT_MAX_RECV_SPEED_LARGE
        | CURLOPT_DEBUGFUNCTION
        | CURLOPT_DEBUGDATA
        | CURLOPT_PROGRESSFUNCTION
        | CURLOPT_XFERINFOFUNCTION
        | CURLOPT_XFERINFODATA
        | CURLOPT_SSL_CTX_DATA
        | CURLOPT_SOCKOPTFUNCTION
        | CURLOPT_SOCKOPTDATA
        | CURLOPT_OPENSOCKETFUNCTION
        | CURLOPT_OPENSOCKETDATA
        | CURLOPT_CLOSESOCKETFUNCTION
        | CURLOPT_CLOSESOCKETDATA
        | CURLOPT_SEEKFUNCTION
        | CURLOPT_SEEKDATA => {
            log::trace!("libcurl: accepted no-op/tuning curl_easy_setopt({option}, {param:#x})");
        }
        other => {
            stub!("libcurl: unsupported curl_easy_setopt(option={other}, param={param:#x})");
            return CURLE_UNKNOWN_OPTION;
        }
    }

    CURLE_OK
}

pub unsafe extern "win64" fn curl_easy_perform(easy: usize) -> i32 {
    let Some(handle) = EASY_HANDLES.lock().unwrap().get(&easy).cloned() else {
        return CURLE_BAD_FUNCTION_ARGUMENT;
    };

    let (req, cb) = match prepare_transfer(&handle) {
        Ok(pair) => pair,
        Err((code, msg, cb)) => {
            write_error_buffer(cb.error_buffer, &msg);
            return code;
        }
    };

    let mut sink = DirectSink {
        easy: Arc::clone(&handle),
        cb,
    };
    let outcome = execute_http_transfer(req, &mut sink);
    apply_outcome(&handle, cb, &outcome);
    outcome.code
}

pub unsafe extern "win64" fn curl_easy_getinfo(easy: usize, info: i32, out: *mut c_void) -> i32 {
    if out.is_null() {
        return CURLE_BAD_FUNCTION_ARGUMENT;
    }
    let Some(handle) = EASY_HANDLES.lock().unwrap().get(&easy).cloned() else {
        return CURLE_BAD_FUNCTION_ARGUMENT;
    };
    let guard = handle.lock().unwrap();
    let st = &guard.info;

    match info {
        // String / pointer infos (8 bytes on Win64)
        CURLINFO_EFFECTIVE_URL => {
            let p = st
                .effective_url_cstr
                .as_ref()
                .map(|b| b.as_ptr() as usize)
                .unwrap_or_else(|| b"\0".as_ptr() as usize);
            std::ptr::write_unaligned(out as *mut usize, p);
        }
        CURLINFO_CONTENT_TYPE => {
            let p = st
                .content_type_cstr
                .as_ref()
                .map(|b| b.as_ptr() as usize)
                .unwrap_or(0);
            std::ptr::write_unaligned(out as *mut usize, p);
        }
        CURLINFO_PRIVATE => {
            std::ptr::write_unaligned(out as *mut usize, guard.opts.private_ptr);
        }
        CURLINFO_REDIRECT_URL => {
            let p = st
                .redirect_url_cstr
                .as_ref()
                .map(|b| b.as_ptr() as usize)
                .unwrap_or(0);
            std::ptr::write_unaligned(out as *mut usize, p);
        }
        CURLINFO_PRIMARY_IP => {
            let p = st
                .primary_ip_cstr
                .as_ref()
                .map(|b| b.as_ptr() as usize)
                .unwrap_or_else(|| b"\0".as_ptr() as usize);
            std::ptr::write_unaligned(out as *mut usize, p);
        }

        // Long infos (4-byte `long` = `i32` on Win64 LLP64)
        CURLINFO_RESPONSE_CODE => {
            std::ptr::write_unaligned(out as *mut i32, st.response_code);
        }
        CURLINFO_HEADER_SIZE => {
            std::ptr::write_unaligned(out as *mut i32, st.header_size);
        }
        CURLINFO_REQUEST_SIZE => {
            std::ptr::write_unaligned(out as *mut i32, st.request_size);
        }
        CURLINFO_SSL_VERIFYRESULT => {
            std::ptr::write_unaligned(out as *mut i32, st.ssl_verifyresult);
        }
        CURLINFO_REDIRECT_COUNT => {
            std::ptr::write_unaligned(out as *mut i32, st.redirect_count);
        }
        CURLINFO_HTTP_CONNECTCODE => {
            std::ptr::write_unaligned(out as *mut i32, st.http_connectcode);
        }
        CURLINFO_OS_ERRNO => {
            std::ptr::write_unaligned(out as *mut i32, st.os_errno);
        }
        CURLINFO_NUM_CONNECTS => {
            std::ptr::write_unaligned(out as *mut i32, st.num_connects);
        }
        CURLINFO_HTTP_VERSION => {
            let v = if st.response_code != 0 {
                CURL_HTTP_VERSION_1_1
            } else {
                0
            };
            std::ptr::write_unaligned(out as *mut i32, v);
        }

        // Double infos (8-byte `f64`)
        CURLINFO_TOTAL_TIME => {
            std::ptr::write_unaligned(out as *mut f64, st.total_time);
        }
        CURLINFO_NAMELOOKUP_TIME => {
            std::ptr::write_unaligned(out as *mut f64, st.namelookup_time);
        }
        CURLINFO_CONNECT_TIME => {
            std::ptr::write_unaligned(out as *mut f64, st.connect_time);
        }
        CURLINFO_SIZE_UPLOAD => {
            std::ptr::write_unaligned(out as *mut f64, st.size_upload as f64);
        }
        CURLINFO_SIZE_DOWNLOAD => {
            std::ptr::write_unaligned(out as *mut f64, st.size_download as f64);
        }
        CURLINFO_SPEED_DOWNLOAD => {
            let spd = if st.total_time > 0.0 {
                st.size_download as f64 / st.total_time
            } else {
                0.0
            };
            std::ptr::write_unaligned(out as *mut f64, spd);
        }
        CURLINFO_SPEED_UPLOAD => {
            let spd = if st.total_time > 0.0 {
                st.size_upload as f64 / st.total_time
            } else {
                0.0
            };
            std::ptr::write_unaligned(out as *mut f64, spd);
        }
        CURLINFO_CONTENT_LENGTH_DOWNLOAD => {
            std::ptr::write_unaligned(out as *mut f64, st.content_length_download as f64);
        }
        CURLINFO_CONTENT_LENGTH_UPLOAD => {
            std::ptr::write_unaligned(out as *mut f64, st.content_length_upload as f64);
        }

        // curl_off_t infos (8-byte `i64`)
        CURLINFO_SIZE_UPLOAD_T => {
            std::ptr::write_unaligned(out as *mut i64, st.size_upload);
        }
        CURLINFO_SIZE_DOWNLOAD_T => {
            std::ptr::write_unaligned(out as *mut i64, st.size_download);
        }
        CURLINFO_SPEED_DOWNLOAD_T => {
            let spd = if st.total_time > 0.0 {
                (st.size_download as f64 / st.total_time) as i64
            } else {
                0
            };
            std::ptr::write_unaligned(out as *mut i64, spd);
        }
        CURLINFO_SPEED_UPLOAD_T => {
            let spd = if st.total_time > 0.0 {
                (st.size_upload as f64 / st.total_time) as i64
            } else {
                0
            };
            std::ptr::write_unaligned(out as *mut i64, spd);
        }
        CURLINFO_CONTENT_LENGTH_DOWNLOAD_T => {
            std::ptr::write_unaligned(out as *mut i64, st.content_length_download);
        }
        CURLINFO_CONTENT_LENGTH_UPLOAD_T => {
            std::ptr::write_unaligned(out as *mut i64, st.content_length_upload);
        }
        CURLINFO_TOTAL_TIME_T => {
            let us = (st.total_time * 1_000_000.0) as i64;
            std::ptr::write_unaligned(out as *mut i64, us);
        }

        other => {
            stub!("libcurl: unsupported curl_easy_getinfo(info={other:#x})");
            return CURLE_UNKNOWN_OPTION;
        }
    }

    CURLE_OK
}

pub unsafe extern "win64" fn curl_easy_strerror(code: i32) -> *const u8 {
    match code {
        CURLE_OK => b"No error\0".as_ptr(),
        CURLE_UNSUPPORTED_PROTOCOL => b"Unsupported protocol\0".as_ptr(),
        CURLE_FAILED_INIT => b"Failed initialization\0".as_ptr(),
        CURLE_URL_MALFORMAT => b"URL using bad/illegal format or missing URL\0".as_ptr(),
        CURLE_NOT_BUILT_IN => b"A requested feature, protocol or option was not found built-in\0".as_ptr(),
        CURLE_COULDNT_RESOLVE_HOST => b"Couldn't resolve host name\0".as_ptr(),
        CURLE_COULDNT_CONNECT => b"Couldn't connect to server\0".as_ptr(),
        CURLE_WEIRD_SERVER_REPLY => b"Weird server reply\0".as_ptr(),
        CURLE_PARTIAL_FILE => b"Transferred a partial file\0".as_ptr(),
        CURLE_HTTP_RETURNED_ERROR => b"HTTP response code said error\0".as_ptr(),
        CURLE_WRITE_ERROR => b"Failed writing received data to disk/application\0".as_ptr(),
        CURLE_READ_ERROR => b"Failed to open/read local data from file/application\0".as_ptr(),
        CURLE_OUT_OF_MEMORY => b"Out of memory\0".as_ptr(),
        CURLE_OPERATION_TIMEDOUT => b"Timeout was reached\0".as_ptr(),
        CURLE_SSL_CONNECT_ERROR => b"SSL connect error\0".as_ptr(),
        CURLE_ABORTED_BY_CALLBACK => b"Operation was aborted by an application callback\0".as_ptr(),
        CURLE_BAD_FUNCTION_ARGUMENT => b"A libcurl function was given a bad argument\0".as_ptr(),
        CURLE_TOO_MANY_REDIRECTS => b"Number of redirects hit maximum amount\0".as_ptr(),
        CURLE_UNKNOWN_OPTION => b"An unknown option was passed in to libcurl\0".as_ptr(),
        CURLE_GOT_NOTHING => b"Server returned nothing (no headers, no data)\0".as_ptr(),
        CURLE_SEND_ERROR => b"Failed sending data to the peer\0".as_ptr(),
        CURLE_RECV_ERROR => b"Failure when receiving data from the peer\0".as_ptr(),
        CURLE_PEER_FAILED_VERIFICATION => b"SSL peer certificate or SSH remote key was not OK\0".as_ptr(),
        CURLE_BAD_CONTENT_ENCODING => b"Unrecognized or bad HTTP Content or Transfer-Encoding\0".as_ptr(),
        _ => b"Unknown CURL error\0".as_ptr(),
    }
}

pub unsafe extern "win64" fn curl_easy_escape(
    _easy: usize,
    string: *const u8,
    length: i32,
) -> *mut u8 {
    if string.is_null() {
        return std::ptr::null_mut();
    }
    let len = if length > 0 {
        length as usize
    } else {
        crate::strings::strlen(string)
    };
    let input = if len == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(string, len)
    };
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = Vec::with_capacity(len);
    for &b in input {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~') {
            encoded.push(b);
        } else {
            encoded.push(b'%');
            encoded.push(HEX[(b >> 4) as usize]);
            encoded.push(HEX[(b & 0x0F) as usize]);
        }
    }
    let mem = crate::heap::alloc(encoded.len() + 1, true) as *mut u8;
    if mem.is_null() {
        return std::ptr::null_mut();
    }
    if !encoded.is_empty() {
        std::ptr::copy_nonoverlapping(encoded.as_ptr(), mem, encoded.len());
    }
    mem
}

pub unsafe extern "win64" fn curl_easy_unescape(
    _easy: usize,
    string: *const u8,
    length: i32,
    outlength: *mut i32,
) -> *mut u8 {
    if string.is_null() {
        if !outlength.is_null() {
            std::ptr::write_unaligned(outlength, 0);
        }
        return std::ptr::null_mut();
    }
    let len = if length > 0 {
        length as usize
    } else {
        crate::strings::strlen(string)
    };
    let input = if len == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(string, len)
    };
    let hex_val = |b: u8| -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    };
    let mut decoded = Vec::with_capacity(len);
    let mut i = 0usize;
    while i < input.len() {
        if input[i] == b'%' && i + 2 < input.len() {
            if let (Some(hi), Some(lo)) = (hex_val(input[i + 1]), hex_val(input[i + 2])) {
                decoded.push((hi << 4) | lo);
                i += 3;
                continue;
            }
        }
        decoded.push(input[i]);
        i += 1;
    }
    let mem = crate::heap::alloc(decoded.len() + 1, true) as *mut u8;
    if mem.is_null() {
        if !outlength.is_null() {
            std::ptr::write_unaligned(outlength, 0);
        }
        return std::ptr::null_mut();
    }
    if !decoded.is_empty() {
        std::ptr::copy_nonoverlapping(decoded.as_ptr(), mem, decoded.len());
    }
    if !outlength.is_null() {
        std::ptr::write_unaligned(outlength, decoded.len() as i32);
    }
    mem
}

pub unsafe extern "win64" fn curl_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        crate::heap::free(ptr);
    }
}

pub unsafe extern "win64" fn curl_slist_append(
    list: *mut CurlSlist,
    s: *const u8,
) -> *mut CurlSlist {
    let len = if s.is_null() {
        0
    } else {
        crate::strings::strlen(s)
    };
    let copy = crate::heap::alloc(len + 1, true) as *mut u8;
    if copy.is_null() {
        return std::ptr::null_mut();
    }
    if !s.is_null() && len > 0 {
        std::ptr::copy_nonoverlapping(s, copy, len);
    }
    let node = crate::heap::alloc(std::mem::size_of::<CurlSlist>(), true) as *mut CurlSlist;
    if node.is_null() {
        crate::heap::free(copy as *mut c_void);
        return std::ptr::null_mut();
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*node).data), copy);
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*node).next), std::ptr::null_mut());
    if list.is_null() {
        return node;
    }
    let mut cur = list;
    loop {
        let next = std::ptr::read_unaligned(std::ptr::addr_of!((*cur).next));
        if next.is_null() {
            break;
        }
        cur = next;
    }
    std::ptr::write_unaligned(std::ptr::addr_of_mut!((*cur).next), node);
    list
}

pub unsafe extern "win64" fn curl_slist_free_all(mut list: *mut CurlSlist) {
    while !list.is_null() {
        let data = std::ptr::read_unaligned(std::ptr::addr_of!((*list).data));
        let next = std::ptr::read_unaligned(std::ptr::addr_of!((*list).next));
        if !data.is_null() {
            crate::heap::free(data as *mut c_void);
        }
        crate::heap::free(list as *mut c_void);
        list = next;
    }
}

// ---------------------------------------------------------------------------
// Exported Win64 `curl_multi_*` & `curl_share_*` API
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn curl_multi_init() -> usize {
    let id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    MULTI_HANDLES
        .lock()
        .unwrap()
        .insert(id, Arc::new(CurlMulti::new()));
    id
}

pub unsafe extern "win64" fn curl_multi_cleanup(multi: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.lock().unwrap().remove(&multi) else {
        return CURLM_BAD_HANDLE;
    };
    let entries: Vec<MultiEntry> = {
        let mut inner = m.inner.lock().unwrap();
        inner.done_queue.clear();
        inner.delivered_msgs.clear();
        inner.entries.drain().map(|(_, e)| e).collect()
    };
    for entry in entries {
        if let MultiEntryState::Running { cancel, .. } = entry.state {
            cancel.store(true, Ordering::Release);
        }
        let mut e = entry.easy.lock().unwrap();
        if e.attached_multi == Some(multi) {
            e.attached_multi = None;
        }
    }
    m.wakeup();
    CURLM_OK
}

pub unsafe extern "win64" fn curl_multi_add_handle(multi: usize, easy: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.lock().unwrap().get(&multi).cloned() else {
        return CURLM_BAD_HANDLE;
    };
    let Some(e) = EASY_HANDLES.lock().unwrap().get(&easy).cloned() else {
        return CURLM_BAD_EASY_HANDLE;
    };
    {
        let mut eg = e.lock().unwrap();
        if eg.attached_multi.is_some() {
            return CURLM_ADDED_ALREADY;
        }
        eg.attached_multi = Some(multi);
    }
    let mut inner = m.inner.lock().unwrap();
    if inner.entries.contains_key(&easy) {
        return CURLM_ADDED_ALREADY;
    }
    inner.entries.insert(
        easy,
        MultiEntry {
            easy: e,
            state: MultiEntryState::Pending,
        },
    );
    drop(inner);
    m.signal_activity();
    CURLM_OK
}

pub unsafe extern "win64" fn curl_multi_remove_handle(multi: usize, easy: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.lock().unwrap().get(&multi).cloned() else {
        return CURLM_BAD_HANDLE;
    };
    let Some(e) = EASY_HANDLES.lock().unwrap().get(&easy).cloned() else {
        return CURLM_BAD_EASY_HANDLE;
    };
    {
        let mut inner = m.inner.lock().unwrap();
        if let Some(entry) = inner.entries.remove(&easy) {
            if let MultiEntryState::Running { cancel, .. } = entry.state {
                cancel.store(true, Ordering::Release);
            }
        }
        inner.done_queue.retain(|msg| msg.easy_handle != easy);
        inner.delivered_msgs.retain(|msg| msg.easy_handle != easy);
    }
    let mut eg = e.lock().unwrap();
    if eg.attached_multi == Some(multi) {
        eg.attached_multi = None;
    }
    CURLM_OK
}

pub unsafe extern "win64" fn curl_multi_perform(multi: usize, running_handles: *mut i32) -> i32 {
    let Some(m) = MULTI_HANDLES.lock().unwrap().get(&multi).cloned() else {
        if !running_handles.is_null() {
            std::ptr::write_unaligned(running_handles, 0);
        }
        return CURLM_BAD_HANDLE;
    };

    // Phase 1: transition Pending handles -> Starting so we can prepare transfers
    // (including guest READFUNCTION) without holding multi.inner.
    let to_start: Vec<(usize, Arc<Mutex<CurlEasy>>)> = {
        let mut inner = m.inner.lock().unwrap();
        let mut v = Vec::new();
        for (&easy_id, entry) in inner.entries.iter_mut() {
            if matches!(entry.state, MultiEntryState::Pending) {
                entry.state = MultiEntryState::Starting;
                v.push((easy_id, Arc::clone(&entry.easy)));
            }
        }
        v
    };

    // Phase 2: prepare & launch background I/O threads for each newly started handle.
    for (easy_id, easy_arc) in to_start {
        match prepare_transfer(&easy_arc) {
            Ok((req, cb)) => {
                let channel = Arc::new(Mutex::new(TransferChannel::default()));
                let cancel = Arc::new(AtomicBool::new(false));

                let still_present = {
                    let mut inner = m.inner.lock().unwrap();
                    if let Some(entry) = inner.entries.get_mut(&easy_id) {
                        entry.state = MultiEntryState::Running {
                            channel: Arc::clone(&channel),
                            cancel: Arc::clone(&cancel),
                            cb,
                        };
                        true
                    } else {
                        false
                    }
                };

                if still_present {
                    let worker_multi = Arc::clone(&m);
                    let worker_channel = Arc::clone(&channel);
                    let worker_cancel = Arc::clone(&cancel);
                    let spawn_res = std::thread::Builder::new()
                        .name("libcurl-io".to_string())
                        .spawn(move || {
                            let mut sink = MultiWorkerSink {
                                channel: Arc::clone(&worker_channel),
                                cancel: worker_cancel,
                                multi: Arc::clone(&worker_multi),
                            };
                            let outcome = execute_http_transfer(req, &mut sink);
                            {
                                let mut ch = worker_channel.lock().unwrap();
                                ch.finished = Some(outcome);
                            }
                            worker_multi.signal_activity();
                        });
                    if let Err(err) = spawn_res {
                        let outcome = TransferOutcome {
                            code: CURLE_FAILED_INIT,
                            error_msg: Some(format!("Failed to spawn I/O thread: {err}")),
                            size_download: 0,
                            size_upload: 0,
                            content_length_upload: 0,
                            total_time: 0.0,
                            os_errno: err.raw_os_error().unwrap_or(0),
                            ssl_verifyresult: 0,
                        };
                        channel.lock().unwrap().finished = Some(outcome);
                        m.signal_activity();
                    }
                }
            }
            Err((code, msg, cb)) => {
                write_error_buffer(cb.error_buffer, &msg);
                let mut inner = m.inner.lock().unwrap();
                if let Some(entry) = inner.entries.get_mut(&easy_id) {
                    entry.state = MultiEntryState::Done;
                    inner.done_queue.push_back(Box::new(CurlMsg {
                        msg: CURLMSG_DONE,
                        _pad: 0,
                        easy_handle: easy_id,
                        data: (code as u32) as usize,
                    }));
                }
            }
        }
    }

    // Phase 3: snapshot active Running handles so we can invoke guest callbacks
    // strictly on the calling thread with zero locks held.
    let active: Vec<(
        usize,
        Arc<Mutex<CurlEasy>>,
        Arc<Mutex<TransferChannel>>,
        Arc<AtomicBool>,
        CallbackConfig,
    )> = {
        let inner = m.inner.lock().unwrap();
        let mut v = Vec::new();
        for (&easy_id, entry) in &inner.entries {
            if let MultiEntryState::Running {
                channel,
                cancel,
                cb,
            } = &entry.state
            {
                v.push((
                    easy_id,
                    Arc::clone(&entry.easy),
                    Arc::clone(channel),
                    Arc::clone(cancel),
                    *cb,
                ));
            }
        }
        v
    };

    for (easy_id, easy_arc, channel, cancel, cb) in active {
        let (events, finished) = {
            let mut ch = channel.lock().unwrap();
            let evs: Vec<TransferEvent> = ch.events.drain(..).collect();
            let fin = ch.finished.take();
            (evs, fin)
        };

        let mut callback_err: Option<i32> = None;
        for ev in events {
            match ev {
                TransferEvent::Head(meta) => apply_head_meta(&easy_arc, meta),
                TransferEvent::HeaderLine(line) => {
                    if callback_err.is_none() {
                        if let Err(code) = invoke_header_callback(cb, &line) {
                            callback_err = Some(code);
                            cancel.store(true, Ordering::Release);
                        }
                    }
                }
                TransferEvent::BodyChunk(data) => {
                    if callback_err.is_none() {
                        if let Err(code) = invoke_write_callback(cb, &data) {
                            callback_err = Some(code);
                            cancel.store(true, Ordering::Release);
                        }
                    }
                }
            }
        }

        if let Some(code) = callback_err {
            let outcome = finished.map_or_else(
                || TransferOutcome {
                    code,
                    error_msg: Some("Write/header callback aborted transfer".to_string()),
                    size_download: 0,
                    size_upload: 0,
                    content_length_upload: 0,
                    total_time: 0.0,
                    os_errno: 0,
                    ssl_verifyresult: 0,
                },
                |mut o| {
                    o.code = code;
                    o.error_msg = Some("Write/header callback aborted transfer".to_string());
                    o
                },
            );
            apply_outcome(&easy_arc, cb, &outcome);
            let mut inner = m.inner.lock().unwrap();
            if let Some(entry) = inner.entries.get_mut(&easy_id) {
                entry.state = MultiEntryState::Done;
                inner.done_queue.push_back(Box::new(CurlMsg {
                    msg: CURLMSG_DONE,
                    _pad: 0,
                    easy_handle: easy_id,
                    data: (code as u32) as usize,
                }));
            }
        } else if let Some(outcome) = finished {
            apply_outcome(&easy_arc, cb, &outcome);
            let mut inner = m.inner.lock().unwrap();
            if let Some(entry) = inner.entries.get_mut(&easy_id) {
                entry.state = MultiEntryState::Done;
                inner.done_queue.push_back(Box::new(CurlMsg {
                    msg: CURLMSG_DONE,
                    _pad: 0,
                    easy_handle: easy_id,
                    data: (outcome.code as u32) as usize,
                }));
            }
        }
    }

    let still_running = {
        let inner = m.inner.lock().unwrap();
        inner
            .entries
            .values()
            .filter(|e| !matches!(e.state, MultiEntryState::Done))
            .count() as i32
    };
    if !running_handles.is_null() {
        std::ptr::write_unaligned(running_handles, still_running);
    }
    CURLM_OK
}

pub unsafe extern "win64" fn curl_multi_poll(
    multi: usize,
    _extra_fds: usize,
    extra_nfds: u32,
    timeout_ms: i32,
    numfds: *mut i32,
) -> i32 {
    multi_wait_impl(multi, extra_nfds, timeout_ms, numfds, true)
}

pub unsafe extern "win64" fn curl_multi_wait(
    multi: usize,
    _extra_fds: usize,
    extra_nfds: u32,
    timeout_ms: i32,
    numfds: *mut i32,
) -> i32 {
    multi_wait_impl(multi, extra_nfds, timeout_ms, numfds, false)
}

pub unsafe extern "win64" fn curl_multi_wakeup(multi: usize) -> i32 {
    let Some(m) = MULTI_HANDLES.lock().unwrap().get(&multi).cloned() else {
        return CURLM_BAD_HANDLE;
    };
    m.wakeup();
    CURLM_OK
}

pub unsafe extern "win64" fn curl_multi_info_read(
    multi: usize,
    msgs_in_queue: *mut i32,
) -> usize {
    let Some(m) = MULTI_HANDLES.lock().unwrap().get(&multi).cloned() else {
        if !msgs_in_queue.is_null() {
            std::ptr::write_unaligned(msgs_in_queue, 0);
        }
        return 0;
    };
    let mut inner = m.inner.lock().unwrap();
    let msg = inner.done_queue.pop_front();
    let remaining = inner.done_queue.len() as i32;
    if !msgs_in_queue.is_null() {
        std::ptr::write_unaligned(msgs_in_queue, remaining);
    }
    if let Some(boxed) = msg {
        let ptr = (&*boxed as *const CurlMsg) as usize;
        inner.delivered_msgs.push(boxed);
        ptr
    } else {
        0
    }
}

pub unsafe extern "win64" fn curl_multi_setopt(
    multi: usize,
    option: i32,
    param: usize,
) -> i32 {
    if !MULTI_HANDLES.lock().unwrap().contains_key(&multi) {
        return CURLM_BAD_HANDLE;
    }
    log::trace!("libcurl: curl_multi_setopt(option={option}, param={param:#x}) -> CURLM_OK");
    CURLM_OK
}

pub unsafe extern "win64" fn curl_multi_strerror(code: i32) -> *const u8 {
    match code {
        CURLM_OK => b"No error\0".as_ptr(),
        CURLM_BAD_HANDLE => b"Invalid multi handle\0".as_ptr(),
        CURLM_BAD_EASY_HANDLE => b"Invalid easy handle\0".as_ptr(),
        CURLM_OUT_OF_MEMORY => b"Out of memory\0".as_ptr(),
        CURLM_INTERNAL_ERROR => b"Internal error\0".as_ptr(),
        CURLM_BAD_SOCKET => b"Invalid socket argument\0".as_ptr(),
        CURLM_UNKNOWN_OPTION => b"Unknown option\0".as_ptr(),
        CURLM_ADDED_ALREADY => b"The easy handle is already added to a multi handle\0".as_ptr(),
        CURLM_BAD_FUNCTION_ARGUMENT => b"A libcurl function was given a bad argument\0".as_ptr(),
        _ => b"Unknown CURLM error\0".as_ptr(),
    }
}

pub unsafe extern "win64" fn curl_share_init() -> usize {
    let id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    SHARE_HANDLES.lock().unwrap().insert(id, ());
    id
}

pub unsafe extern "win64" fn curl_share_setopt(
    share: usize,
    option: i32,
    param: usize,
) -> i32 {
    if !SHARE_HANDLES.lock().unwrap().contains_key(&share) {
        return CURLSHE_INVALID;
    }
    log::trace!("libcurl: curl_share_setopt(option={option}, param={param:#x}) -> CURLSHE_OK");
    CURLSHE_OK
}

pub unsafe extern "win64" fn curl_share_cleanup(share: usize) -> i32 {
    if SHARE_HANDLES.lock().unwrap().remove(&share).is_some() {
        CURLSHE_OK
    } else {
        CURLSHE_INVALID
    }
}

pub unsafe extern "win64" fn curl_share_strerror(code: i32) -> *const u8 {
    match code {
        CURLSHE_OK => b"No error\0".as_ptr(),
        CURLSHE_BAD_OPTION => b"Unknown share option\0".as_ptr(),
        CURLSHE_INVALID => b"Invalid share handle\0".as_ptr(),
        _ => b"Unknown CURLSH error\0".as_ptr(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    unsafe extern "win64" fn vec_write_cb(
        ptr: *const u8,
        size: usize,
        nmemb: usize,
        userdata: usize,
    ) -> usize {
        let total = size.saturating_mul(nmemb);
        if total > 0 && !ptr.is_null() && userdata != 0 {
            let dst = &mut *(userdata as *mut Vec<u8>);
            dst.extend_from_slice(std::slice::from_raw_parts(ptr, total));
        }
        total
    }

    unsafe extern "win64" fn vec_header_cb(
        ptr: *const u8,
        size: usize,
        nmemb: usize,
        userdata: usize,
    ) -> usize {
        let total = size.saturating_mul(nmemb);
        if total > 0 && !ptr.is_null() && userdata != 0 {
            let dst = &mut *(userdata as *mut Vec<String>);
            let line = String::from_utf8_lossy(std::slice::from_raw_parts(ptr, total)).into_owned();
            dst.push(line);
        }
        total
    }

    struct ReadSource {
        data: Vec<u8>,
        pos: usize,
    }

    unsafe extern "win64" fn slice_read_cb(
        buf: *mut u8,
        size: usize,
        nitems: usize,
        userdata: usize,
    ) -> usize {
        let cap = size.saturating_mul(nitems);
        if cap == 0 || buf.is_null() || userdata == 0 {
            return 0;
        }
        let src = &mut *(userdata as *mut ReadSource);
        let rem = src.data.len().saturating_sub(src.pos);
        let n = rem.min(cap).min(4); // Intentionally stream in small chunks
        if n > 0 {
            std::ptr::copy_nonoverlapping(src.data.as_ptr().add(src.pos), buf, n);
            src.pos += n;
        }
        n
    }

    fn read_http_request(stream: &mut TcpStream) -> (String, Vec<u8>) {
        let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
        let mut buf = Vec::new();
        let mut tmp = [0u8; 1024];
        let header_end = loop {
            let n = stream.read(&mut tmp).unwrap_or(0);
            if n == 0 {
                break buf.len();
            }
            buf.extend_from_slice(&tmp[..n]);
            if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                break pos + 4;
            }
        };
        let head = String::from_utf8_lossy(&buf[..header_end]).into_owned();
        let content_len = head
            .lines()
            .find_map(|l| {
                let (k, v) = l.split_once(':')?;
                if k.trim().eq_ignore_ascii_case("content-length") {
                    v.trim().parse::<usize>().ok()
                } else {
                    None
                }
            })
            .unwrap_or(0);
        while buf.len() < header_end + content_len {
            let n = stream.read(&mut tmp).unwrap_or(0);
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&tmp[..n]);
        }
        let body = buf[header_end..buf.len().min(header_end + content_len)].to_vec();
        (head, body)
    }

    struct EnvGuard {
        key: &'static str,
        prev: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, val: &str) -> Self {
            let prev = std::env::var(key).ok();
            std::env::set_var(key, val);
            Self { key, prev }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }

    #[test]
    fn easy_get_headers_and_getinfo() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let (req_head, _) = read_http_request(&mut sock);
            assert!(req_head.starts_with("GET /hello?x=1 HTTP/1.1\r\n"));
            assert!(req_head.contains("User-Agent: TestAgent/1.0\r\n"));
            let resp = concat!(
                "HTTP/1.1 200 OK\r\n",
                "Content-Type: text/plain; charset=utf-8\r\n",
                "Content-Length: 13\r\n",
                "X-Reply: ok\r\n",
                "\r\n",
                "Hello, World!"
            );
            sock.write_all(resp.as_bytes()).unwrap();
        });

        unsafe {
            let easy = curl_easy_init();
            assert_ne!(easy, 0);

            let url = format!("http://{addr}/hello?x=1\0");
            let ua = b"TestAgent/1.0\0";
            let mut body: Vec<u8> = Vec::new();
            let mut headers: Vec<String> = Vec::new();
            let priv_cookie: usize = 0x1234_5678_9ABC_DEF0;

            assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
            assert_eq!(curl_easy_setopt(easy, CURLOPT_USERAGENT, ua.as_ptr() as usize), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_HEADERFUNCTION, vec_header_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(
                    easy,
                    CURLOPT_HEADERDATA,
                    (&mut headers as *mut Vec<String>) as usize
                ),
                CURLE_OK
            );
            assert_eq!(curl_easy_setopt(easy, CURLOPT_PRIVATE, priv_cookie), CURLE_OK);

            assert_eq!(curl_easy_perform(easy), CURLE_OK);
            assert_eq!(body, b"Hello, World!");
            assert!(headers.iter().any(|h| h.starts_with("HTTP/1.1 200 OK")));
            assert!(headers.iter().any(|h| h.starts_with("X-Reply: ok")));

            let mut code: i32 = 0;
            assert_eq!(
                curl_easy_getinfo(easy, CURLINFO_RESPONSE_CODE, &mut code as *mut i32 as *mut c_void),
                CURLE_OK
            );
            assert_eq!(code, 200);

            let mut cl: i64 = 0;
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
                    &mut cl as *mut i64 as *mut c_void
                ),
                CURLE_OK
            );
            assert_eq!(cl, 13);

            let mut dl: i64 = 0;
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_SIZE_DOWNLOAD_T,
                    &mut dl as *mut i64 as *mut c_void
                ),
                CURLE_OK
            );
            assert_eq!(dl, 13);

            let mut total_time: f64 = -1.0;
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_TOTAL_TIME,
                    &mut total_time as *mut f64 as *mut c_void
                ),
                CURLE_OK
            );
            assert!(total_time >= 0.0);

            let mut eff_url: *const u8 = std::ptr::null();
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_EFFECTIVE_URL,
                    &mut eff_url as *mut *const u8 as *mut c_void
                ),
                CURLE_OK
            );
            assert_eq!(crate::strings::from_ansi(eff_url), format!("http://{addr}/hello?x=1"));

            let mut ct: *const u8 = std::ptr::null();
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_CONTENT_TYPE,
                    &mut ct as *mut *const u8 as *mut c_void
                ),
                CURLE_OK
            );
            assert_eq!(crate::strings::from_ansi(ct), "text/plain; charset=utf-8");

            let mut got_priv: usize = 0;
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_PRIVATE,
                    &mut got_priv as *mut usize as *mut c_void
                ),
                CURLE_OK
            );
            assert_eq!(got_priv, priv_cookie);

            curl_easy_cleanup(easy);
        }

        server.join().unwrap();
    }

    #[test]
    fn easy_post_with_slist_headers_and_copypostfields() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let (req_head, req_body) = read_http_request(&mut sock);
            assert!(req_head.starts_with("POST /submit HTTP/1.1\r\n"));
            assert!(req_head.contains("Content-Type: application/json\r\n"));
            assert!(req_head.contains("X-Token: secret-42\r\n"));
            assert!(!req_head.to_ascii_lowercase().contains("accept:"));
            assert_eq!(req_body, br#"{"ok":true}"#);

            let resp = "HTTP/1.1 201 Created\r\nContent-Length: 7\r\n\r\ncreated";
            sock.write_all(resp.as_bytes()).unwrap();
        });

        unsafe {
            let easy = curl_easy_init();
            let url = format!("http://{addr}/submit\0");
            let mut payload = br#"{"ok":true}"#.to_vec();

            let mut slist = curl_slist_append(
                std::ptr::null_mut(),
                b"Content-Type: application/json\0".as_ptr(),
            );
            slist = curl_slist_append(slist, b"X-Token: secret-42\0".as_ptr());
            slist = curl_slist_append(slist, b"Accept:\0".as_ptr()); // suppress default Accept

            let mut body: Vec<u8> = Vec::new();
            assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
            assert_eq!(curl_easy_setopt(easy, CURLOPT_HTTPHEADER, slist as usize), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_POSTFIELDSIZE, payload.len()),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_COPYPOSTFIELDS, payload.as_ptr() as usize),
                CURLE_OK
            );
            // Mutate original buffer to prove COPYPOSTFIELDS copied it eagerly.
            payload.fill(b'X');

            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize),
                CURLE_OK
            );

            assert_eq!(curl_easy_perform(easy), CURLE_OK);
            assert_eq!(body, b"created");

            let mut code: i32 = 0;
            assert_eq!(
                curl_easy_getinfo(easy, CURLINFO_RESPONSE_CODE, &mut code as *mut i32 as *mut c_void),
                CURLE_OK
            );
            assert_eq!(code, 201);

            curl_slist_free_all(slist);
            curl_easy_cleanup(easy);
        }

        server.join().unwrap();
    }

    #[test]
    fn easy_chunked_transfer_decoding() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let _ = read_http_request(&mut sock);
            let resp = concat!(
                "HTTP/1.1 200 OK\r\n",
                "Transfer-Encoding: chunked\r\n",
                "Content-Type: application/octet-stream\r\n",
                "\r\n",
                "5;ext=1\r\n",
                "Hello\r\n",
                "2\r\n",
                ", \r\n",
                "8\r\n",
                "chunked!\r\n",
                "0\r\n",
                "X-Trailer: done\r\n",
                "\r\n"
            );
            sock.write_all(resp.as_bytes()).unwrap();
        });

        unsafe {
            let easy = curl_easy_init();
            let url = format!("http://{addr}/chunked\0");
            let mut body: Vec<u8> = Vec::new();
            assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize),
                CURLE_OK
            );

            assert_eq!(curl_easy_perform(easy), CURLE_OK);
            assert_eq!(body, b"Hello, chunked!");

            let mut cl: i64 = 0;
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
                    &mut cl as *mut i64 as *mut c_void
                ),
                CURLE_OK
            );
            assert_eq!(cl, -1);

            let mut dl: i64 = 0;
            assert_eq!(
                curl_easy_getinfo(
                    easy,
                    CURLINFO_SIZE_DOWNLOAD_T,
                    &mut dl as *mut i64 as *mut c_void
                ),
                CURLE_OK
            );
            assert_eq!(dl, 15);

            curl_easy_cleanup(easy);
        }

        server.join().unwrap();
    }

    #[test]
    fn easy_follow_302_redirect_and_maxredirs() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let server = std::thread::spawn(move || {
            // Connection 1: 302 redirect from /start -> /final
            let (mut s1, _) = listener.accept().unwrap();
            let (h1, _) = read_http_request(&mut s1);
            assert!(h1.starts_with("POST /start HTTP/1.1\r\n"));
            s1.write_all(
                b"HTTP/1.1 302 Found\r\nLocation: /final\r\nContent-Length: 0\r\n\r\n",
            )
            .unwrap();
            drop(s1);

            // Connection 2: redirected GET /final
            let (mut s2, _) = listener.accept().unwrap();
            let (h2, b2) = read_http_request(&mut s2);
            assert!(h2.starts_with("GET /final HTTP/1.1\r\n"));
            assert!(b2.is_empty());
            s2.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nredirected")
                .unwrap();
            drop(s2);

            // Connection 3: test MAXREDIRS = 0
            let (mut s3, _) = listener.accept().unwrap();
            let _ = read_http_request(&mut s3);
            s3.write_all(
                b"HTTP/1.1 302 Found\r\nLocation: /final\r\nContent-Length: 0\r\n\r\n",
            )
            .unwrap();
        });

        unsafe {
            let easy = curl_easy_init();
            let url = format!("http://{addr}/start\0");
            let mut body: Vec<u8> = Vec::new();
            let post_data = b"a=1\0";

            assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_POSTFIELDS, post_data.as_ptr() as usize),
                CURLE_OK
            );
            assert_eq!(curl_easy_setopt(easy, CURLOPT_FOLLOWLOCATION, 1), CURLE_OK);
            assert_eq!(curl_easy_setopt(easy, CURLOPT_MAXREDIRS, 5), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize),
                CURLE_OK
            );

            assert_eq!(curl_easy_perform(easy), CURLE_OK);
            assert_eq!(body, b"redirected");

            let mut code: i32 = 0;
            curl_easy_getinfo(easy, CURLINFO_RESPONSE_CODE, &mut code as *mut i32 as *mut c_void);
            assert_eq!(code, 200);

            let mut redirs: i32 = 0;
            curl_easy_getinfo(
                easy,
                CURLINFO_REDIRECT_COUNT,
                &mut redirs as *mut i32 as *mut c_void,
            );
            assert_eq!(redirs, 1);

            let mut eff_url: *const u8 = std::ptr::null();
            curl_easy_getinfo(
                easy,
                CURLINFO_EFFECTIVE_URL,
                &mut eff_url as *mut *const u8 as *mut c_void,
            );
            assert_eq!(crate::strings::from_ansi(eff_url), format!("http://{addr}/final"));

            // Now test MAXREDIRS = 0 -> CURLE_TOO_MANY_REDIRECTS
            assert_eq!(curl_easy_setopt(easy, CURLOPT_MAXREDIRS, 0), CURLE_OK);
            assert_eq!(curl_easy_perform(easy), CURLE_TOO_MANY_REDIRECTS);

            curl_easy_cleanup(easy);
        }

        server.join().unwrap();
    }

    #[test]
    fn easy_put_with_readfunction_and_head() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let server = std::thread::spawn(move || {
            let (mut s1, _) = listener.accept().unwrap();
            let (h1, b1) = read_http_request(&mut s1);
            assert!(h1.starts_with("PUT /upload HTTP/1.1\r\n"));
            assert_eq!(b1, b"streamed-body");
            s1.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nok")
                .unwrap();
            drop(s1);

            let (mut s2, _) = listener.accept().unwrap();
            let (h2, _) = read_http_request(&mut s2);
            assert!(h2.starts_with("HEAD /check HTTP/1.1\r\n"));
            s2.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 999\r\n\r\n")
                .unwrap();
        });

        unsafe {
            let easy = curl_easy_init();
            let url = format!("http://{addr}/upload\0");
            let mut src = ReadSource {
                data: b"streamed-body".to_vec(),
                pos: 0,
            };
            let mut body: Vec<u8> = Vec::new();

            assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url.as_ptr() as usize), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_CUSTOMREQUEST, b"PUT\0".as_ptr() as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_READFUNCTION, slice_read_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(
                    easy,
                    CURLOPT_READDATA,
                    (&mut src as *mut ReadSource) as usize
                ),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize),
                CURLE_OK
            );
            assert_eq!(curl_easy_perform(easy), CURLE_OK);
            assert_eq!(body, b"ok");

            // Reset and do HEAD via CURLOPT_NOBODY
            curl_easy_reset(easy);
            body.clear();
            let url2 = format!("http://{addr}/check\0");
            assert_eq!(curl_easy_setopt(easy, CURLOPT_URL, url2.as_ptr() as usize), CURLE_OK);
            assert_eq!(curl_easy_setopt(easy, CURLOPT_NOBODY, 1), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(easy, CURLOPT_WRITEDATA, (&mut body as *mut Vec<u8>) as usize),
                CURLE_OK
            );
            assert_eq!(curl_easy_perform(easy), CURLE_OK);
            assert!(body.is_empty());

            let mut cl: i64 = 0;
            curl_easy_getinfo(
                easy,
                CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
                &mut cl as *mut i64 as *mut c_void,
            );
            assert_eq!(cl, 999);

            curl_easy_cleanup(easy);
        }

        server.join().unwrap();
    }

    #[test]
    fn multi_concurrent_perform_poll_and_info_read() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        let server = std::thread::spawn(move || {
            for _ in 0..2 {
                let (mut sock, _) = listener.accept().unwrap();
                std::thread::spawn(move || {
                    let (req_head, _) = read_http_request(&mut sock);
                    let reply_body = if req_head.starts_with("GET /one ") {
                        "alpha"
                    } else if req_head.starts_with("GET /two ") {
                        "beta-two"
                    } else {
                        "unknown"
                    };
                    let resp = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        reply_body.len(),
                        reply_body
                    );
                    let _ = sock.write_all(resp.as_bytes());
                });
            }
        });

        unsafe {
            let multi = curl_multi_init();
            assert_ne!(multi, 0);

            let e1 = curl_easy_init();
            let e2 = curl_easy_init();
            let url1 = format!("http://{addr}/one\0");
            let url2 = format!("http://{addr}/two\0");
            let mut b1: Vec<u8> = Vec::new();
            let mut b2: Vec<u8> = Vec::new();

            assert_eq!(curl_easy_setopt(e1, CURLOPT_URL, url1.as_ptr() as usize), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(e1, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(e1, CURLOPT_WRITEDATA, (&mut b1 as *mut Vec<u8>) as usize),
                CURLE_OK
            );

            assert_eq!(curl_easy_setopt(e2, CURLOPT_URL, url2.as_ptr() as usize), CURLE_OK);
            assert_eq!(
                curl_easy_setopt(e2, CURLOPT_WRITEFUNCTION, vec_write_cb as *const () as usize),
                CURLE_OK
            );
            assert_eq!(
                curl_easy_setopt(e2, CURLOPT_WRITEDATA, (&mut b2 as *mut Vec<u8>) as usize),
                CURLE_OK
            );

            assert_eq!(curl_multi_add_handle(multi, e1), CURLM_OK);
            assert_eq!(curl_multi_add_handle(multi, e2), CURLM_OK);
            // Adding the same handle twice must return CURLM_ADDED_ALREADY.
            assert_eq!(curl_multi_add_handle(multi, e1), CURLM_ADDED_ALREADY);

            let mut running: i32 = 1;
            let mut iters = 0;
            while running > 0 {
                assert_eq!(curl_multi_perform(multi, &mut running), CURLM_OK);
                if running > 0 {
                    let mut numfds: i32 = 0;
                    assert_eq!(
                        curl_multi_poll(multi, 0, 0, 2000, &mut numfds),
                        CURLM_OK
                    );
                }
                iters += 1;
                assert!(iters < 200, "multi transfer did not complete");
            }

            let mut completed: Vec<(usize, i32)> = Vec::new();
            loop {
                let mut left: i32 = 0;
                let msg_ptr = curl_multi_info_read(multi, &mut left);
                if msg_ptr == 0 {
                    assert_eq!(left, 0);
                    break;
                }
                let msg = std::ptr::read_unaligned(msg_ptr as *const CurlMsg);
                assert_eq!(msg.msg, CURLMSG_DONE);
                completed.push((msg.easy_handle, msg.data as i32));
            }
            assert_eq!(completed.len(), 2);
            assert!(completed.contains(&(e1, CURLE_OK)));
            assert!(completed.contains(&(e2, CURLE_OK)));
            assert_eq!(b1, b"alpha");
            assert_eq!(b2, b"beta-two");

            assert_eq!(curl_multi_remove_handle(multi, e1), CURLM_OK);
            assert_eq!(curl_multi_remove_handle(multi, e2), CURLM_OK);
            curl_easy_cleanup(e1);
            curl_easy_cleanup(e2);
            assert_eq!(curl_multi_cleanup(multi), CURLM_OK);
        }

        server.join().unwrap();
    }

    #[test]
    fn offline_mode_duphandle_escape_and_version() {
        unsafe {
            // PORT_OFFLINE=1 forces CURLE_COULDNT_CONNECT without touching the network.
            {
                let _guard = EnvGuard::set("PORT_OFFLINE", "1");
                let easy = curl_easy_init();
                let mut errbuf = [0u8; CURL_ERROR_SIZE];
                assert_eq!(
                    curl_easy_setopt(easy, CURLOPT_URL, b"http://127.0.0.1:1/test\0".as_ptr() as usize),
                    CURLE_OK
                );
                assert_eq!(
                    curl_easy_setopt(easy, CURLOPT_ERRORBUFFER, errbuf.as_mut_ptr() as usize),
                    CURLE_OK
                );
                assert_eq!(curl_easy_perform(easy), CURLE_COULDNT_CONNECT);
                let err_str = crate::strings::from_ansi(errbuf.as_ptr());
                assert!(err_str.contains("PORT_OFFLINE"));

                // duphandle copies options cleanly
                let dup = curl_easy_duphandle(easy);
                assert_ne!(dup, 0);
                assert_ne!(dup, easy);
                assert_eq!(curl_easy_perform(dup), CURLE_COULDNT_CONNECT);
                curl_easy_cleanup(dup);
                curl_easy_cleanup(easy);
            }

            // Escape / unescape roundtrip + curl_free
            let raw = b"hello world/a+b=c\0";
            let esc = curl_easy_escape(0, raw.as_ptr(), 0);
            assert!(!esc.is_null());
            assert_eq!(crate::strings::from_ansi(esc), "hello%20world%2Fa%2Bb%3Dc");

            let mut out_len: i32 = 0;
            let unesc = curl_easy_unescape(0, esc, 0, &mut out_len);
            assert!(!unesc.is_null());
            assert_eq!(out_len, 17);
            assert_eq!(crate::strings::from_ansi(unesc), "hello world/a+b=c");
            curl_free(esc as *mut c_void);
            curl_free(unesc as *mut c_void);

            // Version & version_info sanity
            let ver = crate::strings::from_ansi(curl_version());
            assert!(ver.contains("libcurl/"));
            assert!(ver.contains("rustls"));
            let vi = curl_version_info(10);
            assert!(!vi.is_null());
            let features = std::ptr::read_unaligned(std::ptr::addr_of!((*vi).features));
            let ver_ptr = std::ptr::read_unaligned(std::ptr::addr_of!((*vi).version));
            assert_ne!(features & CURL_VERSION_SSL, 0);
            assert_eq!(crate::strings::from_ansi(ver_ptr), "8.7.1");
        }
    }
}
