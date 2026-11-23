//! `libcurl.dll` easy/slist/share state, constants, ABI structures, and helpers.

use crate::handles::HandleTable;
use std::ffi::c_void;
use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// libcurl constants (exact upstream curl/curl.h numeric IDs)
// ---------------------------------------------------------------------------

// CURLcode
pub(super) const CURLE_OK: i32 = 0;
pub(super) const CURLE_UNSUPPORTED_PROTOCOL: i32 = 1;
pub(super) const CURLE_FAILED_INIT: i32 = 2;
pub(super) const CURLE_URL_MALFORMAT: i32 = 3;
pub(super) const CURLE_NOT_BUILT_IN: i32 = 4;
pub(super) const CURLE_COULDNT_RESOLVE_HOST: i32 = 6;
pub(super) const CURLE_COULDNT_CONNECT: i32 = 7;
pub(super) const CURLE_WEIRD_SERVER_REPLY: i32 = 8;
pub(super) const CURLE_PARTIAL_FILE: i32 = 18;
pub(super) const CURLE_HTTP_RETURNED_ERROR: i32 = 22;
pub(super) const CURLE_WRITE_ERROR: i32 = 23;
pub(super) const CURLE_READ_ERROR: i32 = 26;
pub(super) const CURLE_OUT_OF_MEMORY: i32 = 27;
pub(super) const CURLE_OPERATION_TIMEDOUT: i32 = 28;
pub(super) const CURLE_SSL_CONNECT_ERROR: i32 = 35;
pub(super) const CURLE_ABORTED_BY_CALLBACK: i32 = 42;
pub(super) const CURLE_BAD_FUNCTION_ARGUMENT: i32 = 43;
pub(super) const CURLE_TOO_MANY_REDIRECTS: i32 = 47;
pub(super) const CURLE_UNKNOWN_OPTION: i32 = 48;
pub(super) const CURLE_GOT_NOTHING: i32 = 52;
pub(super) const CURLE_SEND_ERROR: i32 = 55;
pub(super) const CURLE_RECV_ERROR: i32 = 56;
pub(super) const CURLE_PEER_FAILED_VERIFICATION: i32 = 60;
pub(super) const CURLE_BAD_CONTENT_ENCODING: i32 = 61;

// CURLSHcode
pub(super) const CURLSHE_OK: i32 = 0;
pub(super) const CURLSHE_BAD_OPTION: i32 = 1;
pub(super) const CURLSHE_INVALID: i32 = 3;

// Special callback return sentinel & buffer sizes
pub(super) const CURL_READFUNC_ABORT: usize = 0x1000_0000;
pub(super) const CURL_ERROR_SIZE: usize = 256;

// HTTP version reported by CURLINFO_HTTP_VERSION
pub(super) const CURL_HTTP_VERSION_1_1: i32 = 2;

// POSTREDIR bitflags
pub(super) const CURL_REDIR_POST_301: i32 = 1;
pub(super) const CURL_REDIR_POST_302: i32 = 2;
pub(super) const CURL_REDIR_POST_303: i32 = 4;

// CURLOPT_* option IDs
pub(super) const CURLOPT_WRITEDATA: i32 = 10001;
pub(super) const CURLOPT_URL: i32 = 10002;
pub(super) const CURLOPT_PORT: i32 = 3;
pub(super) const CURLOPT_PROXY: i32 = 10004;
pub(super) const CURLOPT_USERPWD: i32 = 10005;
pub(super) const CURLOPT_PROXYUSERPWD: i32 = 10006;
pub(super) const CURLOPT_RANGE: i32 = 10007;
pub(super) const CURLOPT_READDATA: i32 = 10009;
pub(super) const CURLOPT_ERRORBUFFER: i32 = 10010;
pub(super) const CURLOPT_WRITEFUNCTION: i32 = 20011;
pub(super) const CURLOPT_READFUNCTION: i32 = 20012;
pub(super) const CURLOPT_TIMEOUT: i32 = 13;
pub(super) const CURLOPT_INFILESIZE: i32 = 14;
pub(super) const CURLOPT_POSTFIELDS: i32 = 10015;
pub(super) const CURLOPT_REFERER: i32 = 10016;
pub(super) const CURLOPT_USERAGENT: i32 = 10018;
pub(super) const CURLOPT_LOW_SPEED_LIMIT: i32 = 19;
pub(super) const CURLOPT_LOW_SPEED_TIME: i32 = 20;
pub(super) const CURLOPT_COOKIE: i32 = 10022;
pub(super) const CURLOPT_HTTPHEADER: i32 = 10023;
pub(super) const CURLOPT_HEADERDATA: i32 = 10029;
pub(super) const CURLOPT_COOKIEFILE: i32 = 10031;
pub(super) const CURLOPT_SSLVERSION: i32 = 32;
pub(super) const CURLOPT_CUSTOMREQUEST: i32 = 10036;
pub(super) const CURLOPT_VERBOSE: i32 = 41;
pub(super) const CURLOPT_HEADER: i32 = 42;
pub(super) const CURLOPT_NOPROGRESS: i32 = 43;
pub(super) const CURLOPT_NOBODY: i32 = 44;
pub(super) const CURLOPT_FAILONERROR: i32 = 45;
pub(super) const CURLOPT_UPLOAD: i32 = 46;
pub(super) const CURLOPT_POST: i32 = 47;
pub(super) const CURLOPT_FOLLOWLOCATION: i32 = 52;
pub(super) const CURLOPT_PUT: i32 = 54;
pub(super) const CURLOPT_PROGRESSFUNCTION: i32 = 20056;
pub(super) const CURLOPT_XFERINFODATA: i32 = 10057;
pub(super) const CURLOPT_AUTOREFERER: i32 = 58;
pub(super) const CURLOPT_PROXYPORT: i32 = 59;
pub(super) const CURLOPT_POSTFIELDSIZE: i32 = 60;
pub(super) const CURLOPT_HTTPPROXYTUNNEL: i32 = 61;
pub(super) const CURLOPT_SSL_VERIFYPEER: i32 = 64;
pub(super) const CURLOPT_CAINFO: i32 = 10065;
pub(super) const CURLOPT_MAXREDIRS: i32 = 68;
pub(super) const CURLOPT_MAXCONNECTS: i32 = 71;
pub(super) const CURLOPT_FRESH_CONNECT: i32 = 74;
pub(super) const CURLOPT_FORBID_REUSE: i32 = 75;
pub(super) const CURLOPT_CONNECTTIMEOUT: i32 = 78;
pub(super) const CURLOPT_HEADERFUNCTION: i32 = 20079;
pub(super) const CURLOPT_HTTPGET: i32 = 80;
pub(super) const CURLOPT_SSL_VERIFYHOST: i32 = 81;
pub(super) const CURLOPT_COOKIEJAR: i32 = 10082;
pub(super) const CURLOPT_HTTP_VERSION: i32 = 84;
pub(super) const CURLOPT_DNS_CACHE_TIMEOUT: i32 = 92;
pub(super) const CURLOPT_DEBUGFUNCTION: i32 = 20094;
pub(super) const CURLOPT_DEBUGDATA: i32 = 10095;
pub(super) const CURLOPT_CAPATH: i32 = 10097;
pub(super) const CURLOPT_BUFFERSIZE: i32 = 98;
pub(super) const CURLOPT_NOSIGNAL: i32 = 99;
pub(super) const CURLOPT_SHARE: i32 = 10100;
pub(super) const CURLOPT_ACCEPT_ENCODING: i32 = 10102;
pub(super) const CURLOPT_PRIVATE: i32 = 10103;
pub(super) const CURLOPT_SSL_CTX_FUNCTION: i32 = 20108;
pub(super) const CURLOPT_SSL_CTX_DATA: i32 = 10109;
pub(super) const CURLOPT_IPRESOLVE: i32 = 113;
pub(super) const CURLOPT_INFILESIZE_LARGE: i32 = 30115;
pub(super) const CURLOPT_POSTFIELDSIZE_LARGE: i32 = 30120;
pub(super) const CURLOPT_TCP_NODELAY: i32 = 121;
pub(super) const CURLOPT_MAX_SEND_SPEED_LARGE: i32 = 30145;
pub(super) const CURLOPT_MAX_RECV_SPEED_LARGE: i32 = 30146;
pub(super) const CURLOPT_SOCKOPTFUNCTION: i32 = 20148;
pub(super) const CURLOPT_SOCKOPTDATA: i32 = 10149;
pub(super) const CURLOPT_SSL_SESSIONID_CACHE: i32 = 150;
pub(super) const CURLOPT_TIMEOUT_MS: i32 = 155;
pub(super) const CURLOPT_CONNECTTIMEOUT_MS: i32 = 156;
pub(super) const CURLOPT_HTTP_TRANSFER_DECODING: i32 = 157;
pub(super) const CURLOPT_HTTP_CONTENT_DECODING: i32 = 158;
pub(super) const CURLOPT_POSTREDIR: i32 = 161;
pub(super) const CURLOPT_OPENSOCKETFUNCTION: i32 = 20163;
pub(super) const CURLOPT_OPENSOCKETDATA: i32 = 10164;
pub(super) const CURLOPT_COPYPOSTFIELDS: i32 = 10165;
pub(super) const CURLOPT_SEEKFUNCTION: i32 = 20167;
pub(super) const CURLOPT_SEEKDATA: i32 = 10168;
pub(super) const CURLOPT_USERNAME: i32 = 10173;
pub(super) const CURLOPT_PASSWORD: i32 = 10174;
pub(super) const CURLOPT_NOPROXY: i32 = 10177;
pub(super) const CURLOPT_PROTOCOLS: i32 = 181;
pub(super) const CURLOPT_REDIR_PROTOCOLS: i32 = 182;
pub(super) const CURLOPT_CLOSESOCKETFUNCTION: i32 = 20208;
pub(super) const CURLOPT_CLOSESOCKETDATA: i32 = 10209;
pub(super) const CURLOPT_TCP_KEEPALIVE: i32 = 213;
pub(super) const CURLOPT_TCP_KEEPIDLE: i32 = 214;
pub(super) const CURLOPT_TCP_KEEPINTVL: i32 = 215;
pub(super) const CURLOPT_SSL_OPTIONS: i32 = 216;
pub(super) const CURLOPT_XFERINFOFUNCTION: i32 = 20219;
pub(super) const CURLOPT_EXPECT_100_TIMEOUT_MS: i32 = 227;
pub(super) const CURLOPT_PATH_AS_IS: i32 = 234;
pub(super) const CURLOPT_PIPEWAIT: i32 = 237;
pub(super) const CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS: i32 = 271;
pub(super) const CURLOPT_PROTOCOLS_STR: i32 = 10318;
pub(super) const CURLOPT_REDIR_PROTOCOLS_STR: i32 = 10319;

// CURLINFO_* IDs
pub(super) const CURLINFO_EFFECTIVE_URL: i32 = 0x10_0001;
pub(super) const CURLINFO_CONTENT_TYPE: i32 = 0x10_0012;
pub(super) const CURLINFO_PRIVATE: i32 = 0x10_0015;
pub(super) const CURLINFO_REDIRECT_URL: i32 = 0x10_001f;
pub(super) const CURLINFO_PRIMARY_IP: i32 = 0x10_0020;

pub(super) const CURLINFO_RESPONSE_CODE: i32 = 0x20_0002;
pub(super) const CURLINFO_HEADER_SIZE: i32 = 0x20_000b;
pub(super) const CURLINFO_REQUEST_SIZE: i32 = 0x20_000c;
pub(super) const CURLINFO_SSL_VERIFYRESULT: i32 = 0x20_000d;
pub(super) const CURLINFO_REDIRECT_COUNT: i32 = 0x20_0014;
pub(super) const CURLINFO_HTTP_CONNECTCODE: i32 = 0x20_0016;
pub(super) const CURLINFO_OS_ERRNO: i32 = 0x20_0019;
pub(super) const CURLINFO_NUM_CONNECTS: i32 = 0x20_001a;
pub(super) const CURLINFO_HTTP_VERSION: i32 = 0x20_002e;

pub(super) const CURLINFO_TOTAL_TIME: i32 = 0x30_0003;
pub(super) const CURLINFO_NAMELOOKUP_TIME: i32 = 0x30_0004;
pub(super) const CURLINFO_CONNECT_TIME: i32 = 0x30_0005;
pub(super) const CURLINFO_SIZE_UPLOAD: i32 = 0x30_0007;
pub(super) const CURLINFO_SIZE_DOWNLOAD: i32 = 0x30_0008;
pub(super) const CURLINFO_SPEED_DOWNLOAD: i32 = 0x30_0009;
pub(super) const CURLINFO_SPEED_UPLOAD: i32 = 0x30_000a;
pub(super) const CURLINFO_CONTENT_LENGTH_DOWNLOAD: i32 = 0x30_000f;
pub(super) const CURLINFO_CONTENT_LENGTH_UPLOAD: i32 = 0x30_0010;

pub(super) const CURLINFO_SIZE_UPLOAD_T: i32 = 0x60_0007;
pub(super) const CURLINFO_SIZE_DOWNLOAD_T: i32 = 0x60_0008;
pub(super) const CURLINFO_SPEED_DOWNLOAD_T: i32 = 0x60_0009;
pub(super) const CURLINFO_SPEED_UPLOAD_T: i32 = 0x60_000a;
pub(super) const CURLINFO_CONTENT_LENGTH_DOWNLOAD_T: i32 = 0x60_000f;
pub(super) const CURLINFO_CONTENT_LENGTH_UPLOAD_T: i32 = 0x60_0010;
pub(super) const CURLINFO_TOTAL_TIME_T: i32 = 0x60_0032;

// Version feature bits
pub(super) const CURL_VERSION_IPV6: i32 = 1 << 0;
pub(super) const CURL_VERSION_SSL: i32 = 1 << 2;
pub(super) const CURL_VERSION_LIBZ: i32 = 1 << 3;
pub(super) const CURL_VERSION_LARGEFILE: i32 = 1 << 9;

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

pub(super) fn version_info_ptr() -> *const CurlVersionInfoData {
    &VERSION_INFO
}

// ---------------------------------------------------------------------------
// Global handle tables (stride-4 disjoint handle spaces)
// ---------------------------------------------------------------------------

pub(super) static EASY_HANDLES: HandleTable<Arc<Mutex<CurlEasy>>> =
    HandleTable::new(0xC081_0001, 4);

pub(super) static SHARE_HANDLES: HandleTable<()> = HandleTable::new(0xC081_0003, 4);

// ---------------------------------------------------------------------------
// Easy handle state & configuration snapshot
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub(super) struct CurlEasyOptions {
    pub(super) url: Option<String>,
    pub(super) port: u16,
    pub(super) user_agent: Option<String>,
    pub(super) httpheader_ptr: usize,
    pub(super) post: bool,
    pub(super) postfields_ptr: usize,
    pub(super) postfields_copied: Option<Vec<u8>>,
    pub(super) postfieldsize: i64,
    pub(super) custom_request: Option<String>,
    pub(super) nobody: bool,
    pub(super) upload: bool,
    pub(super) infilesize: i64,
    pub(super) follow_location: bool,
    pub(super) max_redirs: i32,
    pub(super) postredir: i32,
    pub(super) timeout_ms: u64,
    pub(super) connect_timeout_ms: u64,
    pub(super) write_fn: usize,
    pub(super) writedata: usize,
    pub(super) header_fn: usize,
    pub(super) headerdata: usize,
    pub(super) read_fn: usize,
    pub(super) readdata: usize,
    pub(super) error_buffer: usize,
    pub(super) private_ptr: usize,
    pub(super) ssl_verifypeer: bool,
    pub(super) ssl_verifyhost: bool,
    pub(super) cainfo: Option<String>,
    pub(super) accept_encoding: Option<String>,
    pub(super) http_content_decoding: bool,
    pub(super) include_header_in_body: bool,
    pub(super) fail_on_error: bool,
    pub(super) verbose: bool,
    pub(super) referer: Option<String>,
    pub(super) autoreferer: bool,
    pub(super) cookie: Option<String>,
    pub(super) range: Option<String>,
    pub(super) userpwd: Option<String>,
    pub(super) username: Option<String>,
    pub(super) password: Option<String>,
    pub(super) tcp_nodelay: bool,
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

pub(super) struct CurlEasyInfo {
    pub(super) response_code: i32,
    pub(super) header_size: i32,
    pub(super) request_size: i32,
    pub(super) redirect_count: i32,
    pub(super) http_connectcode: i32,
    pub(super) ssl_verifyresult: i32,
    pub(super) os_errno: i32,
    pub(super) num_connects: i32,
    pub(super) size_download: i64,
    pub(super) size_upload: i64,
    pub(super) content_length_download: i64,
    pub(super) content_length_upload: i64,
    pub(super) total_time: f64,
    pub(super) namelookup_time: f64,
    pub(super) connect_time: f64,
    pub(super) effective_url_cstr: Option<Box<[u8]>>,
    pub(super) content_type_cstr: Option<Box<[u8]>>,
    pub(super) redirect_url_cstr: Option<Box<[u8]>>,
    pub(super) primary_ip_cstr: Option<Box<[u8]>>,
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

pub(super) struct CurlEasy {
    pub(super) opts: CurlEasyOptions,
    pub(super) info: CurlEasyInfo,
    pub(super) attached_multi: Option<usize>,
}

impl CurlEasy {
    pub(super) fn new() -> Self {
        Self {
            opts: CurlEasyOptions::default(),
            info: CurlEasyInfo::default(),
            attached_multi: None,
        }
    }

    pub(super) fn reset(&mut self) {
        self.opts = CurlEasyOptions::default();
        self.info = CurlEasyInfo::default();
    }
}

#[derive(Clone, Copy)]
pub(super) struct CallbackConfig {
    pub(super) write_fn: usize,
    pub(super) writedata: usize,
    pub(super) header_fn: usize,
    pub(super) headerdata: usize,
    pub(super) include_header_in_body: bool,
    pub(super) error_buffer: usize,
}

pub(super) struct TransferRequest {
    pub(super) url: String,
    pub(super) port_override: u16,
    pub(super) method: String,
    pub(super) explicit_custom_method: bool,
    pub(super) nobody: bool,
    pub(super) body: Vec<u8>,
    pub(super) is_post_default_ct: bool,
    pub(super) custom_headers: Vec<String>,
    pub(super) user_agent: Option<String>,
    pub(super) referer: Option<String>,
    pub(super) autoreferer: bool,
    pub(super) cookie: Option<String>,
    pub(super) range: Option<String>,
    pub(super) basic_auth: Option<String>,
    pub(super) accept_encoding: Option<String>,
    pub(super) http_content_decoding: bool,
    pub(super) follow_location: bool,
    pub(super) max_redirs: i32,
    pub(super) postredir: i32,
    pub(super) fail_on_error: bool,
    pub(super) verbose: bool,
    pub(super) timeout_ms: u64,
    pub(super) connect_timeout_ms: u64,
    pub(super) ssl_verifypeer: bool,
    pub(super) ssl_verifyhost: bool,
    pub(super) tcp_nodelay: bool,
}

#[derive(Clone)]
pub(super) struct HeadMeta {
    pub(super) response_code: i32,
    pub(super) header_size: i32,
    pub(super) request_size: i32,
    pub(super) redirect_count: i32,
    pub(super) num_connects: i32,
    pub(super) content_length_download: i64,
    pub(super) effective_url: String,
    pub(super) content_type: Option<String>,
    pub(super) redirect_url: Option<String>,
    pub(super) primary_ip: Option<String>,
    pub(super) namelookup_time: f64,
    pub(super) connect_time: f64,
}

pub(super) struct TransferOutcome {
    pub(super) code: i32,
    pub(super) error_msg: Option<String>,
    pub(super) size_download: i64,
    pub(super) size_upload: i64,
    pub(super) content_length_upload: i64,
    pub(super) total_time: f64,
    pub(super) os_errno: i32,
    pub(super) ssl_verifyresult: i32,
}

fn make_cstr_box(s: &str) -> Box<[u8]> {
    let mut v = Vec::with_capacity(s.len() + 1);
    v.extend_from_slice(s.as_bytes());
    v.push(0);
    v.into_boxed_slice()
}

pub(super) fn apply_head_meta(easy: &Arc<Mutex<CurlEasy>>, meta: HeadMeta) {
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

pub(super) unsafe fn write_error_buffer(buf: usize, msg: &str) {
    if buf == 0 {
        return;
    }
    crate::strings::write_ansi_z(buf as *mut u8, CURL_ERROR_SIZE, msg);
}

pub(super) unsafe fn apply_outcome(
    easy: &Arc<Mutex<CurlEasy>>,
    cb: CallbackConfig,
    outcome: &TransferOutcome,
) {
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

pub(super) unsafe fn prepare_transfer(
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
        && (opts.upload || opts.post || matches!(method.as_str(), "PUT" | "POST" | "PATCH"))
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

pub(super) unsafe fn invoke_header_callback(cb: CallbackConfig, line: &[u8]) -> Result<(), i32> {
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

pub(super) unsafe fn invoke_write_callback(cb: CallbackConfig, data: &[u8]) -> Result<(), i32> {
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
// Easy / slist / share implementation helpers
// ---------------------------------------------------------------------------

pub(super) fn easy_init() -> usize {
    EASY_HANDLES.insert(Arc::new(Mutex::new(CurlEasy::new())))
}

pub(super) fn easy_duphandle(easy: usize) -> usize {
    let Some(src) = EASY_HANDLES.get_cloned(easy) else {
        return 0;
    };
    let opts = src.lock().unwrap().opts.clone();
    let mut dup = CurlEasy::new();
    dup.opts = opts;
    EASY_HANDLES.insert(Arc::new(Mutex::new(dup)))
}

pub(super) fn easy_cleanup(easy: usize) {
    if easy == 0 {
        return;
    }
    let Some(handle) = EASY_HANDLES.remove(easy) else {
        return;
    };
    let attached = handle.lock().unwrap().attached_multi.take();
    if let Some(multi_id) = attached {
        super::multi::detach_easy_from_multi(multi_id, easy);
    }
}

pub(super) fn easy_reset(easy: usize) {
    if let Some(handle) = EASY_HANDLES.get_cloned(easy) {
        handle.lock().unwrap().reset();
    }
}

pub(super) unsafe fn easy_setopt(easy: usize, option: i32, param: usize) -> i32 {
    let Some(handle) = EASY_HANDLES.get_cloned(easy) else {
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

pub(super) unsafe fn easy_perform(easy: usize) -> i32 {
    let Some(handle) = EASY_HANDLES.get_cloned(easy) else {
        return CURLE_BAD_FUNCTION_ARGUMENT;
    };

    let (req, cb) = match prepare_transfer(&handle) {
        Ok(pair) => pair,
        Err((code, msg, cb)) => {
            write_error_buffer(cb.error_buffer, &msg);
            return code;
        }
    };

    let mut sink = super::http::DirectSink {
        easy: Arc::clone(&handle),
        cb,
    };
    let outcome = super::http::execute_http_transfer(req, &mut sink);
    apply_outcome(&handle, cb, &outcome);
    outcome.code
}

pub(super) unsafe fn easy_getinfo(easy: usize, info: i32, out: *mut c_void) -> i32 {
    if out.is_null() {
        return CURLE_BAD_FUNCTION_ARGUMENT;
    }
    let Some(handle) = EASY_HANDLES.get_cloned(easy) else {
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

pub(super) fn easy_strerror(code: i32) -> *const u8 {
    match code {
        CURLE_OK => b"No error\0".as_ptr(),
        CURLE_UNSUPPORTED_PROTOCOL => b"Unsupported protocol\0".as_ptr(),
        CURLE_FAILED_INIT => b"Failed initialization\0".as_ptr(),
        CURLE_URL_MALFORMAT => b"URL using bad/illegal format or missing URL\0".as_ptr(),
        CURLE_NOT_BUILT_IN => {
            b"A requested feature, protocol or option was not found built-in\0".as_ptr()
        }
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
        CURLE_ABORTED_BY_CALLBACK => {
            b"Operation was aborted by an application callback\0".as_ptr()
        }
        CURLE_BAD_FUNCTION_ARGUMENT => b"A libcurl function was given a bad argument\0".as_ptr(),
        CURLE_TOO_MANY_REDIRECTS => b"Number of redirects hit maximum amount\0".as_ptr(),
        CURLE_UNKNOWN_OPTION => b"An unknown option was passed in to libcurl\0".as_ptr(),
        CURLE_GOT_NOTHING => b"Server returned nothing (no headers, no data)\0".as_ptr(),
        CURLE_SEND_ERROR => b"Failed sending data to the peer\0".as_ptr(),
        CURLE_RECV_ERROR => b"Failure when receiving data from the peer\0".as_ptr(),
        CURLE_PEER_FAILED_VERIFICATION => {
            b"SSL peer certificate or SSH remote key was not OK\0".as_ptr()
        }
        CURLE_BAD_CONTENT_ENCODING => {
            b"Unrecognized or bad HTTP Content or Transfer-Encoding\0".as_ptr()
        }
        _ => b"Unknown CURL error\0".as_ptr(),
    }
}

pub(super) unsafe fn easy_escape(string: *const u8, length: i32) -> *mut u8 {
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

pub(super) unsafe fn easy_unescape(
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

pub(super) unsafe fn slist_append(list: *mut CurlSlist, s: *const u8) -> *mut CurlSlist {
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

pub(super) unsafe fn slist_free_all(mut list: *mut CurlSlist) {
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

pub(super) fn share_init() -> usize {
    SHARE_HANDLES.insert(())
}

pub(super) fn share_setopt(share: usize, option: i32, param: usize) -> i32 {
    if !SHARE_HANDLES.contains(share) {
        return CURLSHE_INVALID;
    }
    log::trace!("libcurl: curl_share_setopt(option={option}, param={param:#x}) -> CURLSHE_OK");
    CURLSHE_OK
}

pub(super) fn share_cleanup(share: usize) -> i32 {
    if SHARE_HANDLES.remove(share).is_some() {
        CURLSHE_OK
    } else {
        CURLSHE_INVALID
    }
}

pub(super) fn share_strerror(code: i32) -> *const u8 {
    match code {
        CURLSHE_OK => b"No error\0".as_ptr(),
        CURLSHE_BAD_OPTION => b"Unknown share option\0".as_ptr(),
        CURLSHE_INVALID => b"Invalid share handle\0".as_ptr(),
        _ => b"Unknown CURLSH error\0".as_ptr(),
    }
}
