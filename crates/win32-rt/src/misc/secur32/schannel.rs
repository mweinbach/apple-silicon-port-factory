//! Schannel SSPI client- and server-side TLS 1.2 / 1.3 implementation backed by `crate::net_tls`.

use crate::handles::HandleTable;
use crate::misc::crypt32::{self, PeerCertVerifyResult};
use crate::net_tls::{self, SelectiveCertVerifier, TlsVerifyPolicy};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::{ClientConfig, ClientConnection, Connection, ProtocolVersion, ServerConfig, ServerConnection};
use std::ffi::c_void;
use std::io::{Read, Write};
use std::sync::{Arc, OnceLock};

// ---------------------------------------------------------------------------
// SSPI / Schannel constants (exact Windows SDK sspi.h / schannel.h / winerror.h)
// ---------------------------------------------------------------------------

pub(super) const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION: u32 = 1;

pub(super) const SEC_E_OK: i32 = 0;
pub(super) const SEC_I_CONTINUE_NEEDED: i32 = 0x0009_0312;
pub(super) const SEC_I_CONTEXT_EXPIRED: i32 = 0x0009_0317;

pub(super) const SEC_E_INSUFFICIENT_MEMORY: i32 = 0x8009_0300u32 as i32;
pub(super) const SEC_E_INVALID_HANDLE: i32 = 0x8009_0301u32 as i32;
pub(super) const SEC_E_UNSUPPORTED_FUNCTION: i32 = 0x8009_0302u32 as i32;
pub(super) const SEC_E_INTERNAL_ERROR: i32 = 0x8009_0304u32 as i32;
pub(super) const SEC_E_SECPKG_NOT_FOUND: i32 = 0x8009_0305u32 as i32;
pub(super) const SEC_E_INVALID_TOKEN: i32 = 0x8009_0308u32 as i32;
pub(super) const SEC_E_NO_CREDENTIALS: i32 = 0x8009_030Eu32 as i32;
pub(super) const SEC_E_INCOMPLETE_MESSAGE: i32 = 0x8009_0318u32 as i32;
pub(super) const SEC_E_CONTEXT_EXPIRED: i32 = 0x8009_0317u32 as i32;
pub(super) const SEC_E_BUFFER_TOO_SMALL: i32 = 0x8009_0321u32 as i32;
pub(super) const SEC_E_WRONG_PRINCIPAL: i32 = 0x8009_0322u32 as i32;
pub(super) const SEC_E_UNTRUSTED_ROOT: i32 = 0x8009_0325u32 as i32;
pub(super) const SEC_E_ILLEGAL_MESSAGE: i32 = 0x8009_0326u32 as i32;
pub(super) const SEC_E_CERT_UNKNOWN: i32 = 0x8009_0327u32 as i32;
pub(super) const SEC_E_CERT_EXPIRED: i32 = 0x8009_0328u32 as i32;
pub(super) const SEC_E_ENCRYPT_FAILURE: i32 = 0x8009_0329u32 as i32;
pub(super) const SEC_E_DECRYPT_FAILURE: i32 = 0x8009_0330u32 as i32;
pub(super) const SEC_E_ALGORITHM_MISMATCH: i32 = 0x8009_0331u32 as i32;

pub(super) const SECPKG_CRED_INBOUND: u32 = 0x0000_0001;
pub(super) const SECPKG_CRED_OUTBOUND: u32 = 0x0000_0002;
pub(super) const SECPKG_CRED_BOTH: u32 = 0x0000_0003;

pub(super) const SCHANNEL_CRED_VERSION: u32 = 4;
pub(super) const SCH_CREDENTIALS_VERSION: u32 = 5;

pub(super) const SCH_CRED_NO_SERVERNAME_CHECK: u32 = 0x0000_0004;
pub(super) const SCH_CRED_MANUAL_CRED_VALIDATION: u32 = 0x0000_0008;
pub(super) const SCH_CRED_DISABLE_RECONNECTS: u32 = 0x0000_0080;
pub(super) const SCH_CRED_REVOCATION_CHECK_END_CERT: u32 = 0x0000_0100;
pub(super) const SCH_CRED_REVOCATION_CHECK_CHAIN: u32 = 0x0000_0200;
pub(super) const SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 0x0000_0400;
pub(super) const SCH_CRED_IGNORE_NO_REVOCATION_CHECK: u32 = 0x0000_0800;
pub(super) const SCH_CRED_IGNORE_REVOCATION_OFFLINE: u32 = 0x0000_1000;

pub(super) const SSL_SESSION_RECONNECT: u32 = 1;

pub(super) const SP_PROT_TLS1_2_SERVER: u32 = 0x0000_0400;
pub(super) const SP_PROT_TLS1_2_CLIENT: u32 = 0x0000_0800;
pub(super) const SP_PROT_TLS1_3_SERVER: u32 = 0x0000_1000;
pub(super) const SP_PROT_TLS1_3_CLIENT: u32 = 0x0000_2000;

pub(super) const ISC_REQ_REPLAY_DETECT: u32 = 0x0000_0004;
pub(super) const ISC_REQ_SEQUENCE_DETECT: u32 = 0x0000_0008;
pub(super) const ISC_REQ_CONFIDENTIALITY: u32 = 0x0000_0010;
pub(super) const ISC_REQ_USE_SUPPLIED_CREDS: u32 = 0x0000_0080;
pub(super) const ISC_REQ_ALLOCATE_MEMORY: u32 = 0x0000_0100;
pub(super) const ISC_REQ_EXTENDED_ERROR: u32 = 0x0000_4000;
pub(super) const ISC_REQ_STREAM: u32 = 0x0000_8000;
pub(super) const ISC_REQ_MANUAL_CRED_VALIDATION: u32 = 0x0008_0000;

// Server-side AcceptSecurityContext request/return flags (sspi.h).
// Note: ASC_REQ_EXTENDED_ERROR (0x8000) and ASC_REQ_STREAM (0x10000) differ from ISC_REQ_*!
pub(super) const ASC_REQ_MUTUAL_AUTH: u32 = 0x0000_0002;
pub(super) const ASC_REQ_REPLAY_DETECT: u32 = 0x0000_0004;
pub(super) const ASC_REQ_SEQUENCE_DETECT: u32 = 0x0000_0008;
pub(super) const ASC_REQ_CONFIDENTIALITY: u32 = 0x0000_0010;
pub(super) const ASC_REQ_ALLOCATE_MEMORY: u32 = 0x0000_0100;
pub(super) const ASC_REQ_EXTENDED_ERROR: u32 = 0x0000_8000;
pub(super) const ASC_REQ_STREAM: u32 = 0x0001_0000;

const SECBUFFER_ATTRMASK: u32 = 0xF000_0000;
pub(super) const SECBUFFER_EMPTY: u32 = 0;
pub(super) const SECBUFFER_DATA: u32 = 1;
pub(super) const SECBUFFER_TOKEN: u32 = 2;
pub(super) const SECBUFFER_MISSING: u32 = 4;
pub(super) const SECBUFFER_EXTRA: u32 = 5;
pub(super) const SECBUFFER_STREAM_TRAILER: u32 = 6;
pub(super) const SECBUFFER_STREAM_HEADER: u32 = 7;
pub(super) const SECBUFFER_APPLICATION_PROTOCOLS: u32 = 18;

pub(super) const SECPKG_ATTR_SIZES: u32 = 0;
pub(super) const SECPKG_ATTR_NAMES: u32 = 1;
pub(super) const SECPKG_ATTR_LIFESPAN: u32 = 2;
pub(super) const SECPKG_ATTR_STREAM_SIZES: u32 = 4;
pub(super) const SECPKG_ATTR_KEY_INFO: u32 = 5;
pub(super) const SECPKG_ATTR_AUTHORITY: u32 = 6;
pub(super) const SECPKG_ATTR_PROTO_INFO: u32 = 7;
pub(super) const SECPKG_ATTR_PACKAGE_INFO: u32 = 10;
pub(super) const SECPKG_ATTR_NEGOTIATION_INFO: u32 = 12;
pub(super) const SECPKG_ATTR_FLAGS: u32 = 14;
pub(super) const SECPKG_ATTR_UNIQUE_BINDINGS: u32 = 25;
pub(super) const SECPKG_ATTR_ENDPOINT_BINDINGS: u32 = 26;
pub(super) const SECPKG_ATTR_CLIENT_SPECIFIED_TARGET: u32 = 27;
pub(super) const SECPKG_ATTR_APPLICATION_PROTOCOL: u32 = 0x23;
pub(super) const SECPKG_ATTR_REMOTE_CERT_CONTEXT: u32 = 0x53;
pub(super) const SECPKG_ATTR_LOCAL_CERT_CONTEXT: u32 = 0x54;
pub(super) const SECPKG_ATTR_ROOT_STORE: u32 = 0x55;
pub(super) const SECPKG_ATTR_SUPPORTED_ALGS: u32 = 0x56;
pub(super) const SECPKG_ATTR_CIPHER_STRENGTHS: u32 = 0x57;
pub(super) const SECPKG_ATTR_SUPPORTED_PROTOCOLS: u32 = 0x58;
pub(super) const SECPKG_ATTR_ISSUER_LIST_EX: u32 = 0x59;
pub(super) const SECPKG_ATTR_CONNECTION_INFO: u32 = 0x5A;
pub(super) const SECPKG_ATTR_EAP_KEY_BLOCK: u32 = 0x5B;
pub(super) const SECPKG_ATTR_SESSION_INFO: u32 = 0x5D;
pub(super) const SECPKG_ATTR_REMOTE_CERTIFICATES: u32 = 0x5F;
pub(super) const SECPKG_ATTR_CIPHER_INFO: u32 = 0x64;
pub(super) const SECPKG_ATTR_REMOTE_CERT_CHAIN: u32 = 0x67;

pub(super) const SECPKG_CRED_ATTR_NAMES: u32 = 1;
pub(super) const SECPKG_CRED_ATTR_SUPPORTED_ALGS: u32 = 0x0F;
pub(super) const SECPKG_CRED_ATTR_CIPHER_STRENGTHS: u32 = 0x10;
pub(super) const SECPKG_CRED_ATTR_SUPPORTED_PROTOCOLS: u32 = 0x11;

pub(super) const SECPKGCONTEXT_CIPHERINFO_V1: u32 = 1;
pub(super) const SZ_ALG_MAX_SIZE: usize = 64;

pub(super) const SCHANNEL_SHUTDOWN: u32 = 1;
pub(super) const SCHANNEL_ALERT: u32 = 2;
pub(super) const SCHANNEL_SESSION: u32 = 3;

const CALG_RSA_SIGN: u32 = 0x0000_2400;
const CALG_ECDSA: u32 = 0x0000_2203;

pub(super) const STREAM_HEADER_SIZE: u32 = 5;
pub(super) const STREAM_TRAILER_SIZE: u32 = 64;
pub(super) const STREAM_MAX_MESSAGE: u32 = 16384;

// WinCrypt ALG_ID constants for SecPkgContext_ConnectionInfo
const CALG_AES_128: u32 = 0x0000_660E;
const CALG_AES_256: u32 = 0x0000_6610;
const CALG_SHA_256: u32 = 0x0000_800C;
const CALG_SHA_384: u32 = 0x0000_800D;
const CALG_ECDH_EPHEM: u32 = 0x0000_AE06;

const MAGIC_CRED: usize = 0x5343_4843_0001_0001; // "SCHC"
const MAGIC_CTXT: usize = 0x5343_4858_0002_0002; // "SCHX"

// ---------------------------------------------------------------------------
// Win64 ABI structures
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct SecHandle {
    pub(super) dw_lower: usize,
    pub(super) dw_upper: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct SecBuffer {
    pub(super) cb_buffer: u32,
    pub(super) buffer_type: u32,
    pub(super) pv_buffer: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct SecBufferDesc {
    pub(super) ul_version: u32,
    pub(super) c_buffers: u32,
    pub(super) p_buffers: *mut SecBuffer,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextStreamSizes {
    pub(super) cb_header: u32,
    pub(super) cb_trailer: u32,
    pub(super) cb_maximum_message: u32,
    pub(super) c_buffers: u32,
    pub(super) cb_block_size: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextSizes {
    pub(super) cb_max_token: u32,
    pub(super) cb_max_signature: u32,
    pub(super) cb_block_size: u32,
    pub(super) cb_security_trailer: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextConnectionInfo {
    pub(super) dw_protocol: u32,
    pub(super) ai_cipher: u32,
    pub(super) dw_cipher_strength: u32,
    pub(super) ai_hash: u32,
    pub(super) dw_hash_strength: u32,
    pub(super) ai_exch: u32,
    pub(super) dw_exch_strength: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextApplicationProtocol {
    pub(super) proto_nego_status: u32,
    pub(super) proto_nego_ext: u32,
    pub(super) protocol_id_size: u8,
    pub(super) protocol_id: [u8; 255],
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextIssuerListInfoEx {
    pub(super) a_issuers: *mut c_void,
    pub(super) c_issuers: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextCipherInfo {
    pub(super) dw_version: u32,
    pub(super) dw_protocol: u32,
    pub(super) dw_cipher_suite: u32,
    pub(super) dw_base_cipher_suite: u32,
    pub(super) sz_cipher_suite: [u16; SZ_ALG_MAX_SIZE],
    pub(super) sz_cipher: [u16; SZ_ALG_MAX_SIZE],
    pub(super) dw_cipher_len: u32,
    pub(super) dw_cipher_block_len: u32,
    pub(super) sz_hash: [u16; SZ_ALG_MAX_SIZE],
    pub(super) dw_hash_len: u32,
    pub(super) sz_exchange: [u16; SZ_ALG_MAX_SIZE],
    pub(super) dw_min_exchange_len: u32,
    pub(super) dw_max_exchange_len: u32,
    pub(super) sz_certificate: [u16; SZ_ALG_MAX_SIZE],
    pub(super) dw_key_type: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextNamesA {
    pub(super) s_user_name: *mut u8,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextNamesW {
    pub(super) s_user_name: *mut u16,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgCredSupportedProtocols {
    pub(super) grbit_protocol: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgCredSupportedAlgs {
    pub(super) c_supported_algs: u32,
    pub(super) palg_supported_algs: *mut u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgCredCipherStrengths {
    pub(super) dw_minimum_cipher_strength: u32,
    pub(super) dw_maximum_cipher_strength: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextCertificates {
    pub(super) c_certs: u32,
    pub(super) cb_cert_chain: u32,
    pub(super) pb_certificate_chain: *mut u8,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextLifespan {
    pub(super) ts_start: i64,
    pub(super) ts_expiry: i64,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextKeyInfoA {
    pub(super) s_signature_algorithm_name: *mut u8,
    pub(super) s_encrypt_algorithm_name: *mut u8,
    pub(super) key_size: u32,
    pub(super) signature_algorithm: u32,
    pub(super) encrypt_algorithm: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextKeyInfoW {
    pub(super) s_signature_algorithm_name: *mut u16,
    pub(super) s_encrypt_algorithm_name: *mut u16,
    pub(super) key_size: u32,
    pub(super) signature_algorithm: u32,
    pub(super) encrypt_algorithm: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextProtoInfoA {
    pub(super) s_protocol_name: *mut u8,
    pub(super) major_version: u32,
    pub(super) minor_version: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextProtoInfoW {
    pub(super) s_protocol_name: *mut u16,
    pub(super) major_version: u32,
    pub(super) minor_version: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextNegotiationInfo {
    pub(super) package_info: *mut c_void,
    pub(super) negotiation_state: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextFlags {
    pub(super) flags: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextBindings {
    pub(super) bindings_length: u32,
    pub(super) bindings: *mut c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextEapKeyBlock {
    pub(super) rgb_keys: [u8; 128],
    pub(super) rgb_ivs: [u8; 64],
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SecPkgContextSessionInfo {
    pub(super) dw_flags: u32,
    pub(super) cb_session_id: u32,
    pub(super) rgb_session_id: [u8; 32],
}

#[repr(C)]
#[allow(dead_code)]
struct SecPkgInfoA {
    f_capabilities: u32,
    w_version: u16,
    w_rpcid: u16,
    cb_max_token: u32,
    name: *const u8,
    comment: *const u8,
}

#[repr(C)]
#[allow(dead_code)]
struct SecPkgInfoW {
    f_capabilities: u32,
    w_version: u16,
    w_rpcid: u16,
    cb_max_token: u32,
    name: *const u16,
    comment: *const u16,
}

// ---------------------------------------------------------------------------
// Internal credential / context tables & shared rustls TLS configuration
// ---------------------------------------------------------------------------

type ClientConfigCacheKey = (bool, bool, bool, Vec<Vec<u8>>);

#[derive(Clone)]
struct SchannelCred {
    principal: String,
    cred_use: u32,
    flags: u32,
    client_enable_tls12: bool,
    client_enable_tls13: bool,
    server_enable_tls12: bool,
    server_enable_tls13: bool,
    has_root_store: bool,
    extra_roots_der: Vec<Vec<u8>>,
    server_cert_chain: Vec<Vec<u8>>,
    server_private_key_pkcs8: Option<Vec<u8>>,
    local_cert_der: Option<Vec<u8>>,
    client_configs: Arc<std::sync::Mutex<Vec<(ClientConfigCacheKey, Arc<ClientConfig>)>>>,
    // Partition server resumption state by client-authentication policy so a session or ticket
    // issued without ASC_REQ_MUTUAL_AUTH can never be resumed into an mTLS context.
    server_session_cache_no_auth: Arc<rustls::server::ServerSessionMemoryCache>,
    server_ticketer_no_auth: Option<Arc<dyn rustls::server::ProducesTickets>>,
    server_session_cache_mtls: Arc<rustls::server::ServerSessionMemoryCache>,
    server_ticketer_mtls: Option<Arc<dyn rustls::server::ProducesTickets>>,
}

struct SchannelContext {
    conn: Connection,
    is_server: bool,
    manual_validation: bool,
    no_servername_check: bool,
    extra_roots_der: Vec<Vec<u8>>,
    local_cert_der: Option<Vec<u8>>,
    target_name: String,
    context_req: u32,
    context_attr: u32,
    shutdown_pending: bool,
    shutdown_sent: bool,
    pending_shutdown_bytes: Vec<u8>,
    pending_handshake_out: Vec<u8>,
    pending_eof: bool,
    pending_plaintext: Vec<u8>,
    server_hello_hs_buf: Vec<u8>,
    server_hello_parsed: bool,
    tls12_session_id: Vec<u8>,
}

static CRED_TABLE: HandleTable<SchannelCred> = HandleTable::new(1, 2);
static CTXT_TABLE: HandleTable<SchannelContext> = HandleTable::new(2, 2);

fn is_schannel_package(name: &str) -> bool {
    let lower = name.trim().to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "microsoft unified security protocol provider"
            | "schannel"
            | "unified security protocol provider"
            | "default tls ssp"
    )
}

unsafe fn read_handle(p: *const SecHandle, expected_magic: usize) -> Option<usize> {
    if p.is_null() {
        return None;
    }
    let h = std::ptr::read_unaligned(p);
    if h.dw_upper == expected_magic && h.dw_lower != 0 && h.dw_lower != usize::MAX {
        Some(h.dw_lower)
    } else {
        None
    }
}

unsafe fn write_handle(p: *mut SecHandle, magic: usize, id: usize) {
    if !p.is_null() {
        std::ptr::write_unaligned(
            p,
            SecHandle {
                dw_lower: id,
                dw_upper: magic,
            },
        );
    }
}

fn build_schannel_tls_config(
    cred: &SchannelCred,
    manual_validation: bool,
    no_servername_check: bool,
    enable_sni: bool,
    alpn: Vec<Vec<u8>>,
) -> Result<Arc<ClientConfig>, i32> {
    let policy = TlsVerifyPolicy::from_schannel(manual_validation, no_servername_check);

    let has_client_cert =
        !cred.server_cert_chain.is_empty() && cred.server_private_key_pkcs8.is_some();

    // Fast path onto shared cached `crate::net_tls::build_tls_config` when using
    // default roots, both TLS 1.2 + 1.3, standard SNI, reconnects enabled, and no client cert.
    let disable_reconnects = (cred.flags & SCH_CRED_DISABLE_RECONNECTS) != 0;
    if cred.extra_roots_der.is_empty()
        && cred.client_enable_tls12
        && cred.client_enable_tls13
        && enable_sni
        && !has_client_cert
        && !disable_reconnects
    {
        let alpn_refs: Vec<&[u8]> = alpn.iter().map(Vec::as_slice).collect();
        return Ok(net_tls::build_tls_config(policy, &alpn_refs));
    }

    // Reuse a cached ClientConfig on this credential when parameters match so rustls keeps the
    // same ServerCertVerifier / ResolvesClientCert Arc identity required for session resumption.
    let cache_key: ClientConfigCacheKey =
        (manual_validation, no_servername_check, enable_sni, alpn.clone());
    if !disable_reconnects {
        if let Ok(guard) = cred.client_configs.lock() {
            if let Some((_, existing)) = guard.iter().find(|(k, _)| *k == cache_key) {
                return Ok(existing.clone());
            }
        }
    }

    let mut versions: Vec<&'static rustls::SupportedProtocolVersion> = Vec::with_capacity(2);
    if cred.client_enable_tls13 {
        versions.push(&rustls::version::TLS13);
    }
    if cred.client_enable_tls12 {
        versions.push(&rustls::version::TLS12);
    }
    if versions.is_empty() {
        return Err(SEC_E_ALGORITHM_MISMATCH);
    }

    let roots = if cred.extra_roots_der.is_empty() {
        net_tls::root_cert_store()
    } else {
        let mut s = (*net_tls::root_cert_store()).clone();
        for der in &cred.extra_roots_der {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    };

    let builder = ClientConfig::builder_with_provider(net_tls::crypto_provider())
        .with_protocol_versions(&versions)
        .map_err(|_| SEC_E_ALGORITHM_MISMATCH)?;

    let client_auth_pair = if has_client_cert {
        let chain: Vec<CertificateDer<'static>> = cred
            .server_cert_chain
            .iter()
            .map(|d| CertificateDer::from(d.clone()))
            .collect();
        let key = PrivateKeyDer::Pkcs8(cred.server_private_key_pkcs8.clone().unwrap().into());
        Some((chain, key))
    } else {
        None
    };

    let mut config = if policy.is_strict() {
        let wants_auth = builder.with_root_certificates(roots);
        if let Some((chain, key)) = client_auth_pair {
            wants_auth
                .with_client_auth_cert(chain, key)
                .map_err(|_| SEC_E_NO_CREDENTIALS)?
        } else {
            wants_auth.with_no_client_auth()
        }
    } else {
        let verifier = Arc::new(
            SelectiveCertVerifier::with_roots(roots, policy)
                .map_err(|_| SEC_E_INTERNAL_ERROR)?,
        );
        let wants_auth = builder.dangerous().with_custom_certificate_verifier(verifier);
        if let Some((chain, key)) = client_auth_pair {
            wants_auth
                .with_client_auth_cert(chain, key)
                .map_err(|_| SEC_E_NO_CREDENTIALS)?
        } else {
            wants_auth.with_no_client_auth()
        }
    };
    config.enable_sni = enable_sni;
    if disable_reconnects {
        config.resumption = rustls::client::Resumption::disabled();
    } else {
        config.resumption = rustls::client::Resumption::in_memory_sessions(64);
    }
    if !alpn.is_empty() {
        config.alpn_protocols = alpn;
    }
    let cfg = Arc::new(config);
    if !disable_reconnects {
        if let Ok(mut guard) = cred.client_configs.lock() {
            if guard.len() < 16 {
                guard.push((cache_key, cfg.clone()));
            }
        }
    }
    Ok(cfg)
}

fn map_rustls_error_to_sec(err: &rustls::Error) -> i32 {
    match err {
        rustls::Error::InvalidCertificate(cert_err) => match cert_err {
            rustls::CertificateError::Expired
            | rustls::CertificateError::NotValidYet
            | rustls::CertificateError::ExpiredContext { .. }
            | rustls::CertificateError::NotValidYetContext { .. } => SEC_E_CERT_EXPIRED,
            rustls::CertificateError::NotValidForName
            | rustls::CertificateError::NotValidForNameContext { .. } => SEC_E_WRONG_PRINCIPAL,
            rustls::CertificateError::UnknownIssuer | rustls::CertificateError::BadSignature => {
                SEC_E_UNTRUSTED_ROOT
            }
            rustls::CertificateError::Revoked => crypt32::CERT_E_REVOKED as i32,
            rustls::CertificateError::UnknownRevocationStatus => {
                crypt32::CRYPT_E_NO_REVOCATION_CHECK as i32
            }
            _ => SEC_E_CERT_UNKNOWN,
        },
        rustls::Error::NoCertificatesPresented => SEC_E_NO_CREDENTIALS,
        rustls::Error::AlertReceived(alert) => match alert {
            rustls::AlertDescription::HandshakeFailure
            | rustls::AlertDescription::ProtocolVersion
            | rustls::AlertDescription::InsufficientSecurity => SEC_E_ALGORITHM_MISMATCH,
            rustls::AlertDescription::CertificateExpired => SEC_E_CERT_EXPIRED,
            rustls::AlertDescription::CertificateRequired => SEC_E_NO_CREDENTIALS,
            rustls::AlertDescription::UnknownCA
            | rustls::AlertDescription::BadCertificate
            | rustls::AlertDescription::UnsupportedCertificate => SEC_E_UNTRUSTED_ROOT,
            _ => SEC_E_ILLEGAL_MESSAGE,
        },
        rustls::Error::InappropriateMessage { .. } | rustls::Error::InvalidMessage(_) => {
            SEC_E_INVALID_TOKEN
        }
        rustls::Error::DecryptError => SEC_E_DECRYPT_FAILURE,
        _ => SEC_E_INTERNAL_ERROR,
    }
}

/// Inspect the 5-byte TLS record header at `buf[0..]`.
/// Returns:
/// - `Ok(Some(total_len))` if a complete TLS record is available,
/// - `Ok(None)` if the header is invalid (not a TLS record),
/// - `Err(missing_bytes)` if more bytes are needed to complete the record.
fn inspect_tls_record(buf: &[u8]) -> Result<Option<usize>, usize> {
    if buf.len() < 5 {
        return Err(5 - buf.len());
    }
    let content_type = buf[0];
    let major = buf[1];
    if !(20..=23).contains(&content_type) || major != 3 {
        return Ok(None);
    }
    let rec_len = u16::from_be_bytes([buf[3], buf[4]]) as usize;
    let total = 5 + rec_len;
    if buf.len() < total {
        return Err(total - buf.len());
    }
    Ok(Some(total))
}

/// Sniff the `ServerHello.session_id` (up to 32 bytes) from TLS handshake records so
/// `SECPKG_ATTR_SESSION_INFO` can report the negotiated TLS 1.2 session ID alongside
/// `rustls::HandshakeKind`. Skips TLS 1.3 HelloRetryRequest messages.
fn capture_server_hello_session_id(ctx: &mut SchannelContext, tls_records: &[u8]) {
    if ctx.server_hello_parsed {
        return;
    }
    const HRR_RANDOM: [u8; 32] = [
        0xCF, 0x21, 0xAD, 0x74, 0xE5, 0x9A, 0x61, 0x11, 0xBE, 0x1D, 0x8C, 0x02, 0x1E, 0x65, 0xB8,
        0x91, 0xC2, 0xA2, 0x11, 0x16, 0x7A, 0xBB, 0x8C, 0x5E, 0x07, 0x9E, 0x09, 0xE2, 0xC8, 0xA8,
        0x33, 0x9C,
    ];
    let mut off = 0usize;
    while off + 5 <= tls_records.len() {
        let ctype = tls_records[off];
        let rec_len = u16::from_be_bytes([tls_records[off + 3], tls_records[off + 4]]) as usize;
        let end = off + 5 + rec_len;
        if end > tls_records.len() {
            break;
        }
        if ctype == 22 {
            let payload = &tls_records[off + 5..end];
            let cap = 512usize.saturating_sub(ctx.server_hello_hs_buf.len());
            if cap > 0 {
                let take = payload.len().min(cap);
                ctx.server_hello_hs_buf.extend_from_slice(&payload[..take]);
            }
        }
        off = end;
    }
    let mut pos = 0usize;
    while pos + 4 <= ctx.server_hello_hs_buf.len() {
        let hs_type = ctx.server_hello_hs_buf[pos];
        let hs_len = u32::from_be_bytes([
            0,
            ctx.server_hello_hs_buf[pos + 1],
            ctx.server_hello_hs_buf[pos + 2],
            ctx.server_hello_hs_buf[pos + 3],
        ]) as usize;
        let body_start = pos + 4;
        if hs_type == 0x02 {
            if body_start + 35 > ctx.server_hello_hs_buf.len() {
                return;
            }
            let random = &ctx.server_hello_hs_buf[body_start + 2..body_start + 34];
            let sid_len = ctx.server_hello_hs_buf[body_start + 34] as usize;
            if sid_len > 32 {
                ctx.server_hello_parsed = true;
                ctx.server_hello_hs_buf.clear();
                return;
            }
            if body_start + 35 + sid_len > ctx.server_hello_hs_buf.len() {
                return;
            }
            if random != HRR_RANDOM {
                ctx.tls12_session_id =
                    ctx.server_hello_hs_buf[body_start + 35..body_start + 35 + sid_len].to_vec();
                ctx.server_hello_parsed = true;
                ctx.server_hello_hs_buf.clear();
                return;
            }
        }
        let next = body_start.saturating_add(hs_len);
        if next > ctx.server_hello_hs_buf.len() {
            return;
        }
        pos = next;
    }
}

fn drain_tls_out(conn: &mut Connection) -> Vec<u8> {
    let mut out = Vec::new();
    while conn.wants_write() {
        match conn.write_tls(&mut out) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }
    out
}

pub(super) fn build_inbound_client_root_store(
    has_root_store: bool,
    extra_roots_der: &[Vec<u8>],
) -> Arc<rustls::RootCertStore> {
    if has_root_store {
        // When the server supplies a private hRootStore, scope mutual-TLS client trust
        // strictly to the CAs in that store (never merge public WebPKI roots, and never
        // fall back to public roots if the supplied store is empty).
        let mut s = rustls::RootCertStore::empty();
        for der in extra_roots_der {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    } else {
        net_tls::root_cert_store()
    }
}

fn schannel_revocation_chain_flags(cred_flags: u32) -> u32 {
    let mut out = 0u32;
    if (cred_flags & SCH_CRED_REVOCATION_CHECK_END_CERT) != 0 {
        out |= 0x1000_0000; // CERT_CHAIN_REVOCATION_CHECK_END_CERT
    }
    if (cred_flags & SCH_CRED_REVOCATION_CHECK_CHAIN) != 0 {
        out |= 0x2000_0000; // CERT_CHAIN_REVOCATION_CHECK_CHAIN
    }
    if (cred_flags & SCH_CRED_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT) != 0 {
        out |= 0x4000_0000; // CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT
    }
    out
}

/// Wraps `WebPkiClientVerifier` to enforce `SCH_CRED_REVOCATION_CHECK_*` on inbound mTLS
/// client certificates via macOS `Security.framework`, failing closed when revocation status
/// cannot be positively verified (unless explicitly ignored via `SCH_CRED_IGNORE_*`).
#[derive(Debug)]
struct RevocationClientCertVerifier {
    inner: Arc<dyn rustls::server::danger::ClientCertVerifier>,
    has_root_store: bool,
    extra_roots_der: Vec<Vec<u8>>,
    cred_flags: u32,
    chain_revocation_flags: u32,
}

impl rustls::server::danger::ClientCertVerifier for RevocationClientCertVerifier {
    fn offer_client_auth(&self) -> bool {
        self.inner.offer_client_auth()
    }

    fn client_auth_mandatory(&self) -> bool {
        self.inner.client_auth_mandatory()
    }

    fn root_hint_subjects(&self) -> &[rustls::DistinguishedName] {
        self.inner.root_hint_subjects()
    }

    fn verify_client_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::server::danger::ClientCertVerified, rustls::Error> {
        let verified = self.inner.verify_client_cert(end_entity, intermediates, now)?;
        let mut chain_der = Vec::with_capacity(1 + intermediates.len());
        chain_der.push(end_entity.as_ref().to_vec());
        for ic in intermediates {
            chain_der.push(ic.as_ref().to_vec());
        }
        let rev_err = crypt32::check_chain_revocation_error(
            &chain_der,
            self.has_root_store.then_some(self.extra_roots_der.as_slice()),
            &[],
            Some(now.as_secs() as f64),
            self.chain_revocation_flags,
        );
        if rev_err == 0 {
            return Ok(verified);
        }
        if rev_err == crypt32::CERT_E_REVOKED {
            return Err(rustls::Error::InvalidCertificate(
                rustls::CertificateError::Revoked,
            ));
        }
        let ignore_no_rev = (self.cred_flags & SCH_CRED_IGNORE_NO_REVOCATION_CHECK) != 0;
        let ignore_offline = (self.cred_flags & SCH_CRED_IGNORE_REVOCATION_OFFLINE) != 0;
        if (rev_err == crypt32::CRYPT_E_NO_REVOCATION_CHECK && ignore_no_rev)
            || (rev_err == crypt32::CRYPT_E_REVOCATION_OFFLINE && ignore_offline)
        {
            return Ok(verified);
        }
        Err(rustls::Error::InvalidCertificate(
            rustls::CertificateError::UnknownRevocationStatus,
        ))
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls12_signature(message, cert, dss)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        self.inner.verify_tls13_signature(message, cert, dss)
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        self.inner.supported_verify_schemes()
    }
}

fn build_schannel_server_config(
    cred: &SchannelCred,
    alpn: Vec<Vec<u8>>,
    require_client_auth: bool,
) -> Result<Arc<ServerConfig>, i32> {
    let Some(ref key_pkcs8) = cred.server_private_key_pkcs8 else {
        return Err(SEC_E_NO_CREDENTIALS);
    };
    if cred.server_cert_chain.is_empty() {
        return Err(SEC_E_NO_CREDENTIALS);
    }
    let mut versions: Vec<&'static rustls::SupportedProtocolVersion> = Vec::with_capacity(2);
    if cred.server_enable_tls13 {
        versions.push(&rustls::version::TLS13);
    }
    if cred.server_enable_tls12 {
        versions.push(&rustls::version::TLS12);
    }
    if versions.is_empty() {
        return Err(SEC_E_ALGORITHM_MISMATCH);
    }
    let chain: Vec<CertificateDer<'static>> = cred
        .server_cert_chain
        .iter()
        .map(|d| CertificateDer::from(d.clone()))
        .collect();
    let key = PrivateKeyDer::Pkcs8(key_pkcs8.clone().into());
    let provider = net_tls::crypto_provider();
    let builder = ServerConfig::builder_with_provider(provider.clone())
        .with_protocol_versions(&versions)
        .map_err(|_| SEC_E_ALGORITHM_MISMATCH)?;
    let mut cfg = if require_client_auth {
        let roots = build_inbound_client_root_store(cred.has_root_store, &cred.extra_roots_der);
        let base_verifier =
            rustls::server::WebPkiClientVerifier::builder_with_provider(roots, provider)
                .build()
                .map_err(|_| SEC_E_NO_CREDENTIALS)?;
        let rev_flags = schannel_revocation_chain_flags(cred.flags);
        let verifier: Arc<dyn rustls::server::danger::ClientCertVerifier> = if rev_flags != 0 {
            Arc::new(RevocationClientCertVerifier {
                inner: base_verifier,
                has_root_store: cred.has_root_store,
                extra_roots_der: cred.extra_roots_der.clone(),
                cred_flags: cred.flags,
                chain_revocation_flags: rev_flags,
            })
        } else {
            base_verifier
        };
        builder
            .with_client_cert_verifier(verifier)
            .with_single_cert(chain, key)
            .map_err(|_| SEC_E_NO_CREDENTIALS)?
    } else {
        builder
            .with_no_client_auth()
            .with_single_cert(chain, key)
            .map_err(|_| SEC_E_NO_CREDENTIALS)?
    };
    if (cred.flags & SCH_CRED_DISABLE_RECONNECTS) != 0 {
        cfg.session_storage = Arc::new(rustls::server::NoServerSessionStorage {});
        cfg.send_tls13_tickets = 0;
    } else if require_client_auth {
        cfg.session_storage = cred.server_session_cache_mtls.clone();
        if let Some(ref ticketer) = cred.server_ticketer_mtls {
            cfg.ticketer = ticketer.clone();
        }
    } else {
        cfg.session_storage = cred.server_session_cache_no_auth.clone();
        if let Some(ref ticketer) = cred.server_ticketer_no_auth {
            cfg.ticketer = ticketer.clone();
        }
    }
    if !alpn.is_empty() {
        cfg.alpn_protocols = alpn;
    }
    Ok(Arc::new(cfg))
}

unsafe fn emit_token_output(
    p_output: *mut SecBufferDesc,
    data: &[u8],
    allocate_mem: bool,
) -> Result<(), i32> {
    if p_output.is_null() {
        return if data.is_empty() {
            Ok(())
        } else {
            Err(SEC_E_BUFFER_TOO_SMALL)
        };
    }
    let desc = std::ptr::read_unaligned(p_output);
    if desc.p_buffers.is_null() || desc.c_buffers == 0 {
        return if data.is_empty() {
            Ok(())
        } else {
            Err(SEC_E_BUFFER_TOO_SMALL)
        };
    }
    // Find SECBUFFER_TOKEN (or first SECBUFFER_EMPTY as fallback).
    let mut target_idx: Option<usize> = None;
    for i in 0..desc.c_buffers as usize {
        let b = std::ptr::read_unaligned(desc.p_buffers.add(i));
        let kind = b.buffer_type & !SECBUFFER_ATTRMASK;
        if kind == SECBUFFER_TOKEN {
            target_idx = Some(i);
            break;
        }
        if kind == SECBUFFER_EMPTY && target_idx.is_none() {
            target_idx = Some(i);
        }
    }
    let Some(idx) = target_idx else {
        return if data.is_empty() {
            Ok(())
        } else {
            Err(SEC_E_BUFFER_TOO_SMALL)
        };
    };

    let slot = desc.p_buffers.add(idx);
    let mut sb = std::ptr::read_unaligned(slot);
    sb.buffer_type = SECBUFFER_TOKEN;

    if data.is_empty() {
        sb.cb_buffer = 0;
        if allocate_mem {
            sb.pv_buffer = std::ptr::null_mut();
        }
        std::ptr::write_unaligned(slot, sb);
        return Ok(());
    }

    if allocate_mem {
        let mem = crate::heap::alloc(data.len(), false) as *mut u8;
        if mem.is_null() {
            return Err(SEC_E_INSUFFICIENT_MEMORY);
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), mem, data.len());
        sb.pv_buffer = mem as *mut c_void;
        sb.cb_buffer = data.len() as u32;
        std::ptr::write_unaligned(slot, sb);
        Ok(())
    } else {
        if sb.pv_buffer.is_null() || (sb.cb_buffer as usize) < data.len() {
            return Err(SEC_E_BUFFER_TOO_SMALL);
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), sb.pv_buffer as *mut u8, data.len());
        sb.cb_buffer = data.len() as u32;
        std::ptr::write_unaligned(slot, sb);
        Ok(())
    }
}

unsafe fn set_input_aux_buffer(
    p_input: *mut SecBufferDesc,
    buf_type: u32,
    pv: *mut c_void,
    cb: u32,
) {
    if p_input.is_null() {
        return;
    }
    let desc = std::ptr::read_unaligned(p_input);
    if desc.p_buffers.is_null() {
        return;
    }
    for i in 0..desc.c_buffers as usize {
        let slot = desc.p_buffers.add(i);
        let mut sb = std::ptr::read_unaligned(slot);
        let kind = sb.buffer_type & !SECBUFFER_ATTRMASK;
        if kind == SECBUFFER_EMPTY || kind == buf_type {
            sb.buffer_type = buf_type;
            sb.pv_buffer = pv;
            sb.cb_buffer = cb;
            std::ptr::write_unaligned(slot, sb);
            return;
        }
    }
}

unsafe fn parse_alpn_protocols(p_input: *mut SecBufferDesc) -> Vec<Vec<u8>> {
    let mut protos = Vec::new();
    if p_input.is_null() {
        return protos;
    }
    let desc = std::ptr::read_unaligned(p_input);
    if desc.p_buffers.is_null() {
        return protos;
    }
    for i in 0..desc.c_buffers as usize {
        let sb = std::ptr::read_unaligned(desc.p_buffers.add(i));
        let kind = sb.buffer_type & !SECBUFFER_ATTRMASK;
        if kind != SECBUFFER_APPLICATION_PROTOCOLS || sb.pv_buffer.is_null() || sb.cb_buffer < 10 {
            continue;
        }
        let bytes = std::slice::from_raw_parts(sb.pv_buffer as *const u8, sb.cb_buffer as usize);
        let lists_size = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;
        let end = (4 + lists_size).min(bytes.len());
        let mut off = 4usize;
        while off + 6 <= end {
            let ext = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            let list_len = u16::from_le_bytes(bytes[off + 4..off + 6].try_into().unwrap()) as usize;
            off += 6;
            let list_end = (off + list_len).min(end);
            if ext == 2 {
                // SecApplicationProtocolNegotiationExt_ALPN
                let mut k = off;
                while k < list_end {
                    let plen = bytes[k] as usize;
                    k += 1;
                    if plen > 0 && k + plen <= list_end {
                        protos.push(bytes[k..k + plen].to_vec());
                    }
                    k += plen;
                }
            }
            off = list_end;
        }
    }
    protos
}

// ---------------------------------------------------------------------------
// SSPI implementation helpers called by `secur32.rs` exports
// ---------------------------------------------------------------------------

pub(super) unsafe fn acquire_credentials_impl(
    principal: String,
    package_name: &str,
    f_credential_use: u32,
    p_auth_data: *mut c_void,
    ph_credential: *mut SecHandle,
    pts_expiry: *mut i64,
) -> i32 {
    if ph_credential.is_null() {
        return SEC_E_INVALID_HANDLE;
    }
    if !is_schannel_package(package_name) {
        log::debug!("AcquireCredentialsHandle(pkg={package_name:?}) -> SEC_E_SECPKG_NOT_FOUND");
        return SEC_E_SECPKG_NOT_FOUND;
    }
    let want_inbound = (f_credential_use & SECPKG_CRED_INBOUND) != 0;
    let want_outbound = (f_credential_use & SECPKG_CRED_OUTBOUND) != 0;
    if (f_credential_use & SECPKG_CRED_BOTH) == 0 {
        log::debug!("AcquireCredentialsHandle(pkg={package_name:?}, use={f_credential_use:#x}) -> SEC_E_NO_CREDENTIALS");
        return SEC_E_NO_CREDENTIALS;
    }

    let mut flags = 0u32;
    let mut client_enable_tls12 = true;
    let mut client_enable_tls13 = true;
    let mut server_enable_tls12 = true;
    let mut server_enable_tls13 = true;
    let mut has_root_store = false;
    let mut extra_roots_der = Vec::new();
    let mut server_cert_chain = Vec::new();
    let mut server_private_key_pkcs8 = None;
    let mut local_cert_der = None;

    if !p_auth_data.is_null() {
        let base = p_auth_data as *const u8;
        let dw_version = std::ptr::read_unaligned(base as *const u32);
        let mut extract_creds = |c_creds: u32, pa_cred: *const *const c_void| {
            if c_creds > 0 && !pa_cred.is_null() {
                for i in 0..c_creds as usize {
                    let cert_ctx = std::ptr::read_unaligned(pa_cred.add(i));
                    if cert_ctx.is_null() {
                        continue;
                    }
                    if let Some((chain, key)) = crypt32::lookup_cert_private_key_and_chain(cert_ctx)
                    {
                        if local_cert_der.is_none() {
                            local_cert_der = chain.first().cloned();
                        }
                        server_cert_chain = chain;
                        server_private_key_pkcs8 = Some(key);
                        break;
                    } else if local_cert_der.is_none() {
                        let cc = std::ptr::read_unaligned(
                            cert_ctx as *const crate::misc::wintrust::CertContext,
                        );
                        if !cc.pb_cert_encoded.is_null() && cc.cb_cert_encoded > 0 {
                            local_cert_der = Some(
                                std::slice::from_raw_parts(
                                    cc.pb_cert_encoded,
                                    cc.cb_cert_encoded as usize,
                                )
                                .to_vec(),
                            );
                        }
                    }
                }
            }
        };

        match dw_version {
            SCHANNEL_CRED_VERSION => {
                // SCHANNEL_CRED (Win64): cCreds @ +0x04, paCred @ +0x08, hRootStore @ +0x10,
                // grbitEnabledProtocols @ +0x38, dwFlags @ +0x48
                let c_creds = std::ptr::read_unaligned(base.add(0x04) as *const u32);
                let pa_cred = std::ptr::read_unaligned(base.add(0x08) as *const *const *const c_void);
                let h_root_store = std::ptr::read_unaligned(base.add(0x10) as *const *mut c_void);
                let enabled = std::ptr::read_unaligned(base.add(0x38) as *const u32);
                flags = std::ptr::read_unaligned(base.add(0x48) as *const u32);
                extract_creds(c_creds, pa_cred);
                if !h_root_store.is_null() {
                    has_root_store = true;
                    extra_roots_der = crypt32::store_certs_der(h_root_store);
                }
                if enabled != 0 {
                    let want_c12 = (enabled & SP_PROT_TLS1_2_CLIENT) != 0;
                    let want_c13 = (enabled & SP_PROT_TLS1_3_CLIENT) != 0;
                    let want_s12 = (enabled & SP_PROT_TLS1_2_SERVER) != 0;
                    let want_s13 = (enabled & SP_PROT_TLS1_3_SERVER) != 0;
                    if want_c12 || want_c13 {
                        client_enable_tls12 = want_c12;
                        client_enable_tls13 = want_c13;
                    }
                    if want_s12 || want_s13 {
                        server_enable_tls12 = want_s12;
                        server_enable_tls13 = want_s13;
                    }
                }
            }
            SCH_CREDENTIALS_VERSION => {
                // SCH_CREDENTIALS (Win64): cCreds @ +0x08, paCred @ +0x10, hRootStore @ +0x18,
                // dwFlags @ +0x34, cTlsParameters @ +0x38, pTlsParameters @ +0x40
                let c_creds = std::ptr::read_unaligned(base.add(0x08) as *const u32);
                let pa_cred = std::ptr::read_unaligned(base.add(0x10) as *const *const *const c_void);
                let h_root_store = std::ptr::read_unaligned(base.add(0x18) as *const *mut c_void);
                flags = std::ptr::read_unaligned(base.add(0x34) as *const u32);
                let c_params = std::ptr::read_unaligned(base.add(0x38) as *const u32);
                let p_params = std::ptr::read_unaligned(base.add(0x40) as *const *const u8);
                extract_creds(c_creds, pa_cred);
                if !h_root_store.is_null() {
                    has_root_store = true;
                    extra_roots_der = crypt32::store_certs_der(h_root_store);
                }
                if c_params > 0 && !p_params.is_null() {
                    // TLS_PARAMETERS.grbitDisabledProtocols @ +0x10
                    let disabled = std::ptr::read_unaligned(p_params.add(0x10) as *const u32);
                    client_enable_tls12 = (disabled & SP_PROT_TLS1_2_CLIENT) == 0;
                    client_enable_tls13 = (disabled & SP_PROT_TLS1_3_CLIENT) == 0;
                    server_enable_tls12 = (disabled & SP_PROT_TLS1_2_SERVER) == 0;
                    server_enable_tls13 = (disabled & SP_PROT_TLS1_3_SERVER) == 0;
                    if (want_outbound && !client_enable_tls12 && !client_enable_tls13)
                        || (want_inbound && !server_enable_tls12 && !server_enable_tls13)
                    {
                        return SEC_E_ALGORITHM_MISMATCH;
                    }
                }
            }
            other => {
                log::warn!("AcquireCredentialsHandle: unknown SCHANNEL auth data version {other}");
            }
        }
    }

    if want_inbound && !want_outbound && server_private_key_pkcs8.is_none() {
        log::debug!("AcquireCredentialsHandle(INBOUND): no server cert/key in paCred -> SEC_E_NO_CREDENTIALS");
        return SEC_E_NO_CREDENTIALS;
    }

    let id = CRED_TABLE.insert(SchannelCred {
        principal,
        cred_use: f_credential_use,
        flags,
        client_enable_tls12,
        client_enable_tls13,
        server_enable_tls12,
        server_enable_tls13,
        has_root_store,
        extra_roots_der,
        server_cert_chain,
        server_private_key_pkcs8,
        local_cert_der,
        client_configs: Arc::new(std::sync::Mutex::new(Vec::new())),
        server_session_cache_no_auth: rustls::server::ServerSessionMemoryCache::new(64),
        server_ticketer_no_auth: rustls::crypto::ring::Ticketer::new().ok(),
        server_session_cache_mtls: rustls::server::ServerSessionMemoryCache::new(64),
        server_ticketer_mtls: rustls::crypto::ring::Ticketer::new().ok(),
    });
    write_handle(ph_credential, MAGIC_CRED, id);
    if !pts_expiry.is_null() {
        std::ptr::write_unaligned(pts_expiry, i64::MAX);
    }
    log::debug!(
        "AcquireCredentialsHandle(pkg={package_name:?}, cli=({client_enable_tls12},{client_enable_tls13}), srv=({server_enable_tls12},{server_enable_tls13}), flags={flags:#x}) -> id={id}"
    );
    SEC_E_OK
}

pub(super) unsafe fn free_credentials_handle_impl(ph_credential: *mut c_void) -> i32 {
    let Some(id) = read_handle(ph_credential as *const SecHandle, MAGIC_CRED) else {
        return SEC_E_INVALID_HANDLE;
    };
    if CRED_TABLE.remove(id).is_some() {
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub(super) unsafe fn initialize_security_context_impl(
    ph_credential: *mut SecHandle,
    ph_context: *mut SecHandle,
    target_name: Option<String>,
    f_context_req: u32,
    p_input: *mut SecBufferDesc,
    ph_new_context: *mut SecHandle,
    p_output: *mut SecBufferDesc,
    pf_context_attr: *mut u32,
    pts_expiry: *mut i64,
) -> i32 {
    let allocate_mem = (f_context_req & ISC_REQ_ALLOCATE_MEMORY) != 0;

    if !pts_expiry.is_null() {
        std::ptr::write_unaligned(pts_expiry, i64::MAX);
    }
    let ret_attr = ISC_REQ_REPLAY_DETECT
        | ISC_REQ_SEQUENCE_DETECT
        | ISC_REQ_CONFIDENTIALITY
        | ISC_REQ_STREAM
        | (f_context_req
            & (ISC_REQ_ALLOCATE_MEMORY
                | ISC_REQ_EXTENDED_ERROR
                | ISC_REQ_USE_SUPPLIED_CREDS
                | ISC_REQ_MANUAL_CRED_VALIDATION));

    // ---- Initial handshake call (ph_context == NULL) ----
    if ph_context.is_null() {
        let Some(cred_id) = read_handle(ph_credential, MAGIC_CRED) else {
            return SEC_E_INVALID_HANDLE;
        };
        let Some(cred) = CRED_TABLE.get_cloned(cred_id) else {
            return SEC_E_INVALID_HANDLE;
        };
        if ph_new_context.is_null() {
            return SEC_E_INVALID_HANDLE;
        }

        let manual_validation = (cred.flags & SCH_CRED_MANUAL_CRED_VALIDATION) != 0
            || (f_context_req & ISC_REQ_MANUAL_CRED_VALIDATION) != 0;
        let no_servername_check = (cred.flags & SCH_CRED_NO_SERVERNAME_CHECK) != 0;

        let (server_name, enable_sni) = match target_name
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Some(host) => {
                let clean = host.trim_start_matches('[').trim_end_matches(']');
                match ServerName::try_from(clean.to_string()) {
                    Ok(sn) => {
                        let is_dns = matches!(sn, ServerName::DnsName(_));
                        (sn, is_dns)
                    }
                    Err(_) => {
                        if manual_validation || no_servername_check {
                            (
                                ServerName::try_from("localhost".to_string()).unwrap(),
                                false,
                            )
                        } else {
                            return SEC_E_WRONG_PRINCIPAL;
                        }
                    }
                }
            }
            None => (
                ServerName::try_from("localhost".to_string()).unwrap(),
                false,
            ),
        };

        let alpn = parse_alpn_protocols(p_input);
        let config = match build_schannel_tls_config(
            &cred,
            manual_validation,
            no_servername_check,
            enable_sni,
            alpn,
        ) {
            Ok(cfg) => cfg,
            Err(code) => return code,
        };

        let mut conn = match ClientConnection::new(config, server_name) {
            Ok(c) => Connection::Client(c),
            Err(ref e) => return map_rustls_error_to_sec(e),
        };

        let client_hello = drain_tls_out(&mut conn);
        if let Err(code) = emit_token_output(p_output, &client_hello, allocate_mem) {
            return code;
        }

        let stored_target = target_name.clone().unwrap_or_default();
        let ctx_id = CTXT_TABLE.insert(SchannelContext {
            conn,
            is_server: false,
            manual_validation,
            no_servername_check,
            extra_roots_der: cred.extra_roots_der,
            local_cert_der: cred.local_cert_der,
            target_name: stored_target,
            context_req: f_context_req,
            context_attr: ret_attr,
            shutdown_pending: false,
            shutdown_sent: false,
            pending_shutdown_bytes: Vec::new(),
            pending_handshake_out: Vec::new(),
            pending_eof: false,
            pending_plaintext: Vec::new(),
            server_hello_hs_buf: Vec::new(),
            server_hello_parsed: false,
            tls12_session_id: Vec::new(),
        });
        write_handle(ph_new_context, MAGIC_CTXT, ctx_id);
        if !pf_context_attr.is_null() {
            std::ptr::write_unaligned(pf_context_attr, ret_attr);
        }
        log::debug!(
            "InitializeSecurityContext(init, target={target_name:?}, manual={manual_validation}) -> ctx={ctx_id}, ClientHello={}B",
            client_hello.len()
        );
        return SEC_I_CONTINUE_NEEDED;
    }

    // ---- Subsequent handshake / shutdown calls (ph_context != NULL) ----
    let Some(ctx_id) = read_handle(ph_context, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if !ph_new_context.is_null() {
        write_handle(ph_new_context, MAGIC_CTXT, ctx_id);
    }

    CTXT_TABLE
        .with_mut(ctx_id, |ctx| {
            if ctx.shutdown_pending {
                if ctx.pending_shutdown_bytes.is_empty() && !ctx.shutdown_sent {
                    ctx.conn.send_close_notify();
                    ctx.pending_shutdown_bytes = drain_tls_out(&mut ctx.conn);
                }
                let alert_bytes = ctx.pending_shutdown_bytes.clone();
                if let Err(code) = unsafe { emit_token_output(p_output, &alert_bytes, allocate_mem) }
                {
                    return code;
                }
                ctx.shutdown_pending = false;
                ctx.shutdown_sent = true;
                ctx.pending_shutdown_bytes.clear();
                if !pf_context_attr.is_null() {
                    unsafe { std::ptr::write_unaligned(pf_context_attr, ret_attr) };
                }
                log::debug!(
                    "InitializeSecurityContext(ctx={ctx_id}, shutdown) -> close_notify={}B",
                    alert_bytes.len()
                );
                return SEC_E_OK;
            }

            // Flush any previously drained handshake flight that failed to fit in an earlier
            // caller-provided output buffer (SEC_E_BUFFER_TOO_SMALL) before reading new input.
            if !ctx.pending_handshake_out.is_empty() {
                if let Err(code) =
                    unsafe { emit_token_output(p_output, &ctx.pending_handshake_out, allocate_mem) }
                {
                    return code;
                }
                ctx.pending_handshake_out.clear();
                if !pf_context_attr.is_null() {
                    unsafe { std::ptr::write_unaligned(pf_context_attr, ret_attr) };
                }
                return if ctx.conn.is_handshaking() {
                    SEC_I_CONTINUE_NEEDED
                } else {
                    SEC_E_OK
                };
            }

            // Find SECBUFFER_TOKEN in p_input.
            let mut token_ptr: *const u8 = std::ptr::null();
            let mut token_len: usize = 0;
            if !p_input.is_null() {
                unsafe {
                    let desc = std::ptr::read_unaligned(p_input);
                    if !desc.p_buffers.is_null() {
                        for i in 0..desc.c_buffers as usize {
                            let sb = std::ptr::read_unaligned(desc.p_buffers.add(i));
                            if (sb.buffer_type & !SECBUFFER_ATTRMASK) == SECBUFFER_TOKEN
                                && !sb.pv_buffer.is_null()
                                && sb.cb_buffer > 0
                            {
                                token_ptr = sb.pv_buffer as *const u8;
                                token_len = sb.cb_buffer as usize;
                                break;
                            }
                        }
                    }
                }
            }

            if token_ptr.is_null() || token_len == 0 {
                let out = drain_tls_out(&mut ctx.conn);
                if !out.is_empty() {
                    if let Err(code) = unsafe { emit_token_output(p_output, &out, allocate_mem) } {
                        ctx.pending_handshake_out = out;
                        return code;
                    }
                    if !pf_context_attr.is_null() {
                        unsafe { std::ptr::write_unaligned(pf_context_attr, ret_attr) };
                    }
                    return if ctx.conn.is_handshaking() {
                        SEC_I_CONTINUE_NEEDED
                    } else {
                        SEC_E_OK
                    };
                }
                unsafe {
                    let _ = emit_token_output(p_output, &[], allocate_mem);
                    set_input_aux_buffer(p_input, SECBUFFER_MISSING, std::ptr::null_mut(), 5);
                }
                return SEC_E_INCOMPLETE_MESSAGE;
            }

            let input = unsafe { std::slice::from_raw_parts(token_ptr, token_len) };
            let mut consumed = 0usize;
            let mut missing_bytes = 0usize;

            while consumed < input.len() && ctx.conn.is_handshaking() {
                match inspect_tls_record(&input[consumed..]) {
                    Err(missing) => {
                        missing_bytes = missing;
                        break;
                    }
                    Ok(None) => {
                        return SEC_E_INVALID_TOKEN;
                    }
                    Ok(Some(rec_total)) => {
                        let rec_slice = &input[consumed..consumed + rec_total];
                        if !ctx.is_server {
                            capture_server_hello_session_id(ctx, rec_slice);
                        }
                        let mut rec = rec_slice;
                        while !rec.is_empty() {
                            match ctx.conn.read_tls(&mut rec) {
                                Ok(0) => break,
                                Ok(_) => {}
                                Err(_) => return SEC_E_INVALID_TOKEN,
                            }
                        }
                        consumed += rec_total;
                        if let Err(ref e) = ctx.conn.process_new_packets() {
                            let sec_err = map_rustls_error_to_sec(e);
                            if (ctx.context_req & ISC_REQ_EXTENDED_ERROR) != 0 {
                                let alert = drain_tls_out(&mut ctx.conn);
                                if !alert.is_empty() {
                                    unsafe {
                                        let _ = emit_token_output(p_output, &alert, allocate_mem);
                                    }
                                }
                            }
                            log::debug!(
                                "InitializeSecurityContext(ctx={ctx_id}) handshake error: {e} -> {sec_err:#010x}"
                            );
                            return sec_err;
                        }
                    }
                }
            }

            if consumed == 0 && ctx.conn.is_handshaking() && !ctx.conn.wants_write() {
                unsafe {
                    let _ = emit_token_output(p_output, &[], allocate_mem);
                    set_input_aux_buffer(
                        p_input,
                        SECBUFFER_MISSING,
                        std::ptr::null_mut(),
                        missing_bytes.max(1) as u32,
                    );
                }
                return SEC_E_INCOMPLETE_MESSAGE;
            }

            let out_bytes = drain_tls_out(&mut ctx.conn);
            if ctx.is_server && !out_bytes.is_empty() {
                capture_server_hello_session_id(ctx, &out_bytes);
            }
            if let Err(code) = unsafe { emit_token_output(p_output, &out_bytes, allocate_mem) } {
                if !out_bytes.is_empty() {
                    ctx.pending_handshake_out = out_bytes;
                }
                return code;
            }

            if consumed < input.len() {
                let extra = input.len() - consumed;
                unsafe {
                    set_input_aux_buffer(
                        p_input,
                        SECBUFFER_EXTRA,
                        token_ptr.add(consumed) as *mut c_void,
                        extra as u32,
                    );
                }
            }

            if !pf_context_attr.is_null() {
                unsafe { std::ptr::write_unaligned(pf_context_attr, ret_attr) };
            }

            if ctx.conn.is_handshaking() {
                SEC_I_CONTINUE_NEEDED
            } else {
                log::debug!(
                    "InitializeSecurityContext(ctx={ctx_id}) -> SEC_E_OK (proto={:?}, alpn={:?}, out={}B, extra={}B)",
                    ctx.conn.protocol_version(),
                    ctx.conn.alpn_protocol().map(|b| String::from_utf8_lossy(b).into_owned()),
                    out_bytes.len(),
                    input.len() - consumed
                );
                SEC_E_OK
            }
        })
        .unwrap_or(SEC_E_INVALID_HANDLE)
}

pub(super) unsafe fn accept_security_context_impl(
    ph_credential: *mut SecHandle,
    ph_context: *mut SecHandle,
    p_input: *mut SecBufferDesc,
    f_context_req: u32,
    ph_new_context: *mut SecHandle,
    p_output: *mut SecBufferDesc,
    pf_context_attr: *mut u32,
    pts_expiry: *mut i64,
) -> i32 {
    let allocate_mem = (f_context_req & ASC_REQ_ALLOCATE_MEMORY) != 0;
    let require_client_auth = (f_context_req & ASC_REQ_MUTUAL_AUTH) != 0;
    if !pts_expiry.is_null() {
        std::ptr::write_unaligned(pts_expiry, i64::MAX);
    }
    let ret_attr = ASC_REQ_REPLAY_DETECT
        | ASC_REQ_SEQUENCE_DETECT
        | ASC_REQ_CONFIDENTIALITY
        | ASC_REQ_STREAM
        | (f_context_req & (ASC_REQ_MUTUAL_AUTH | ASC_REQ_ALLOCATE_MEMORY | ASC_REQ_EXTENDED_ERROR));

    // Create a fresh server context on initial call (ph_context == NULL).
    let (ctx_id, created_new) = if ph_context.is_null() {
        let Some(cred_id) = read_handle(ph_credential, MAGIC_CRED) else {
            return SEC_E_INVALID_HANDLE;
        };
        let Some(cred) = CRED_TABLE.get_cloned(cred_id) else {
            return SEC_E_INVALID_HANDLE;
        };
        if (cred.cred_use & SECPKG_CRED_INBOUND) == 0 {
            return SEC_E_NO_CREDENTIALS;
        }
        if ph_new_context.is_null() {
            return SEC_E_INVALID_HANDLE;
        }
        let alpn = parse_alpn_protocols(p_input);
        let cfg = match build_schannel_server_config(&cred, alpn, require_client_auth) {
            Ok(c) => c,
            Err(code) => return code,
        };
        let srv = match ServerConnection::new(cfg) {
            Ok(s) => Connection::Server(s),
            Err(ref e) => return map_rustls_error_to_sec(e),
        };
        let id = CTXT_TABLE.insert(SchannelContext {
            conn: srv,
            is_server: true,
            manual_validation: false,
            no_servername_check: false,
            extra_roots_der: cred.extra_roots_der,
            local_cert_der: cred.local_cert_der,
            target_name: String::new(),
            context_req: f_context_req,
            context_attr: ret_attr,
            shutdown_pending: false,
            shutdown_sent: false,
            pending_shutdown_bytes: Vec::new(),
            pending_handshake_out: Vec::new(),
            pending_eof: false,
            pending_plaintext: Vec::new(),
            server_hello_hs_buf: Vec::new(),
            server_hello_parsed: false,
            tls12_session_id: Vec::new(),
        });
        (id, true)
    } else {
        let Some(id) = read_handle(ph_context, MAGIC_CTXT) else {
            return SEC_E_INVALID_HANDLE;
        };
        (id, false)
    };

    let status = CTXT_TABLE
        .with_mut(ctx_id, |ctx| {
            if ctx.shutdown_pending {
                if ctx.pending_shutdown_bytes.is_empty() && !ctx.shutdown_sent {
                    ctx.conn.send_close_notify();
                    ctx.pending_shutdown_bytes = drain_tls_out(&mut ctx.conn);
                }
                let alert_bytes = ctx.pending_shutdown_bytes.clone();
                if let Err(code) = unsafe { emit_token_output(p_output, &alert_bytes, allocate_mem) }
                {
                    return code;
                }
                ctx.shutdown_pending = false;
                ctx.shutdown_sent = true;
                ctx.pending_shutdown_bytes.clear();
                if !pf_context_attr.is_null() {
                    unsafe { std::ptr::write_unaligned(pf_context_attr, ret_attr) };
                }
                return SEC_E_OK;
            }

            // Flush any previously drained server handshake flight that did not fit in an earlier
            // caller-provided output buffer (SEC_E_BUFFER_TOO_SMALL) before reading new input.
            if !ctx.pending_handshake_out.is_empty() {
                if let Err(code) =
                    unsafe { emit_token_output(p_output, &ctx.pending_handshake_out, allocate_mem) }
                {
                    return code;
                }
                ctx.pending_handshake_out.clear();
                if !pf_context_attr.is_null() {
                    unsafe { std::ptr::write_unaligned(pf_context_attr, ret_attr) };
                }
                return if ctx.conn.is_handshaking() {
                    SEC_I_CONTINUE_NEEDED
                } else {
                    SEC_E_OK
                };
            }

            let mut token_ptr: *const u8 = std::ptr::null();
            let mut token_len: usize = 0;
            if !p_input.is_null() {
                unsafe {
                    let desc = std::ptr::read_unaligned(p_input);
                    if !desc.p_buffers.is_null() {
                        for i in 0..desc.c_buffers as usize {
                            let sb = std::ptr::read_unaligned(desc.p_buffers.add(i));
                            if (sb.buffer_type & !SECBUFFER_ATTRMASK) == SECBUFFER_TOKEN
                                && !sb.pv_buffer.is_null()
                                && sb.cb_buffer > 0
                            {
                                token_ptr = sb.pv_buffer as *const u8;
                                token_len = sb.cb_buffer as usize;
                                break;
                            }
                        }
                    }
                }
            }
            if token_ptr.is_null() || token_len == 0 {
                unsafe {
                    let _ = emit_token_output(p_output, &[], allocate_mem);
                    set_input_aux_buffer(p_input, SECBUFFER_MISSING, std::ptr::null_mut(), 5);
                }
                return SEC_E_INCOMPLETE_MESSAGE;
            }

            let input = unsafe { std::slice::from_raw_parts(token_ptr, token_len) };
            let mut consumed = 0usize;
            let mut missing_bytes = 0usize;
            while consumed < input.len() && ctx.conn.is_handshaking() {
                match inspect_tls_record(&input[consumed..]) {
                    Err(missing) => {
                        missing_bytes = missing;
                        break;
                    }
                    Ok(None) => return SEC_E_INVALID_TOKEN,
                    Ok(Some(rec_total)) => {
                        let rec_slice = &input[consumed..consumed + rec_total];
                        if !ctx.is_server {
                            capture_server_hello_session_id(ctx, rec_slice);
                        }
                        let mut rec = rec_slice;
                        while !rec.is_empty() {
                            match ctx.conn.read_tls(&mut rec) {
                                Ok(0) => break,
                                Ok(_) => {}
                                Err(_) => return SEC_E_INVALID_TOKEN,
                            }
                        }
                        consumed += rec_total;
                        if let Err(ref e) = ctx.conn.process_new_packets() {
                            let sec_err = map_rustls_error_to_sec(e);
                            if (ctx.context_req & ASC_REQ_EXTENDED_ERROR) != 0 {
                                let alert = drain_tls_out(&mut ctx.conn);
                                if !alert.is_empty() {
                                    unsafe {
                                        let _ = emit_token_output(p_output, &alert, allocate_mem);
                                    }
                                }
                            }
                            return sec_err;
                        }
                        if ctx.target_name.is_empty() {
                            if let Connection::Server(ref sc) = ctx.conn {
                                if let Some(sni) = sc.server_name() {
                                    ctx.target_name = sni.to_string();
                                }
                            }
                        }
                    }
                }
            }

            if consumed == 0 && ctx.conn.is_handshaking() && !ctx.conn.wants_write() {
                unsafe {
                    let _ = emit_token_output(p_output, &[], allocate_mem);
                    set_input_aux_buffer(
                        p_input,
                        SECBUFFER_MISSING,
                        std::ptr::null_mut(),
                        missing_bytes.max(1) as u32,
                    );
                }
                return SEC_E_INCOMPLETE_MESSAGE;
            }

            let out_bytes = drain_tls_out(&mut ctx.conn);
            if ctx.is_server && !out_bytes.is_empty() {
                capture_server_hello_session_id(ctx, &out_bytes);
            }
            if let Err(code) = unsafe { emit_token_output(p_output, &out_bytes, allocate_mem) } {
                if !out_bytes.is_empty() {
                    ctx.pending_handshake_out = out_bytes;
                }
                return code;
            }
            if consumed < input.len() {
                let extra = input.len() - consumed;
                unsafe {
                    set_input_aux_buffer(
                        p_input,
                        SECBUFFER_EXTRA,
                        token_ptr.add(consumed) as *mut c_void,
                        extra as u32,
                    );
                }
            }
            if !pf_context_attr.is_null() {
                unsafe { std::ptr::write_unaligned(pf_context_attr, ret_attr) };
            }
            if ctx.conn.is_handshaking() {
                SEC_I_CONTINUE_NEEDED
            } else {
                SEC_E_OK
            }
        })
        .unwrap_or(SEC_E_INVALID_HANDLE);

    if created_new {
        if status == SEC_E_OK || status == SEC_I_CONTINUE_NEEDED {
            write_handle(ph_new_context, MAGIC_CTXT, ctx_id);
        } else {
            let _ = CTXT_TABLE.remove(ctx_id);
        }
    } else if !ph_new_context.is_null() {
        write_handle(ph_new_context, MAGIC_CTXT, ctx_id);
    }
    status
}

pub(super) unsafe fn complete_auth_token_impl(ph_context: *mut c_void) -> i32 {
    let Some(id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if CTXT_TABLE.contains(id) {
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub(super) unsafe fn delete_security_context_impl(ph_context: *mut c_void) -> i32 {
    let Some(id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if CTXT_TABLE.remove(id).is_some() {
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub(super) unsafe fn apply_control_token_impl(
    ph_context: *mut c_void,
    p_input: *mut c_void,
) -> i32 {
    let Some(id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if p_input.is_null() {
        return SEC_E_INVALID_TOKEN;
    }
    let desc = std::ptr::read_unaligned(p_input as *const SecBufferDesc);
    if desc.p_buffers.is_null() {
        return SEC_E_INVALID_TOKEN;
    }
    let mut token_val: Option<u32> = None;
    let mut second_word: Option<u32> = None;
    let mut alert_num: Option<u32> = None;
    for i in 0..desc.c_buffers as usize {
        let sb = std::ptr::read_unaligned(desc.p_buffers.add(i));
        let kind = sb.buffer_type & !SECBUFFER_ATTRMASK;
        if (kind == SECBUFFER_TOKEN || kind == SECBUFFER_DATA)
            && !sb.pv_buffer.is_null()
            && sb.cb_buffer >= 4
        {
            let tv = std::ptr::read_unaligned(sb.pv_buffer as *const u32);
            token_val = Some(tv);
            if sb.cb_buffer >= 8 {
                second_word = Some(std::ptr::read_unaligned(
                    (sb.pv_buffer as *const u8).add(4) as *const u32,
                ));
            }
            if sb.cb_buffer >= 12 {
                // SCHANNEL_ALERT_TOKEN { dwTokenType(0), dwAlertType(4), dwAlertNumber(8) }
                alert_num = Some(std::ptr::read_unaligned(
                    (sb.pv_buffer as *const u8).add(8) as *const u32,
                ));
            }
            break;
        }
    }
    match token_val {
        Some(SCHANNEL_SHUTDOWN) => CTXT_TABLE
            .with_mut(id, |ctx| {
                if !ctx.shutdown_sent && !ctx.shutdown_pending {
                    ctx.conn.send_close_notify();
                    ctx.pending_shutdown_bytes = drain_tls_out(&mut ctx.conn);
                    ctx.shutdown_pending = true;
                }
                SEC_E_OK
            })
            .unwrap_or(SEC_E_INVALID_HANDLE),
        Some(SCHANNEL_ALERT) => {
            // SCHANNEL_ALERT_TOKEN { dwTokenType(2), dwAlertType(1=WARNING|2=FATAL), dwAlertNumber }
            let (Some(alert_type), Some(num)) = (second_word, alert_num) else {
                return SEC_E_INVALID_TOKEN;
            };
            if alert_type != 1 && alert_type != 2 {
                return SEC_E_INVALID_TOKEN;
            }
            // rustls only exposes emitting a TLS warning close_notify (type=1, number=0);
            // honestly reject fatal or other custom alerts rather than silently emitting
            // close_notify or discarding the requested alert.
            if alert_type != 1 || num != 0 {
                return SEC_E_UNSUPPORTED_FUNCTION;
            }
            CTXT_TABLE
                .with_mut(id, |ctx| {
                    if !ctx.shutdown_sent && !ctx.shutdown_pending {
                        ctx.conn.send_close_notify();
                        ctx.pending_shutdown_bytes = drain_tls_out(&mut ctx.conn);
                        ctx.shutdown_pending = true;
                    }
                    SEC_E_OK
                })
                .unwrap_or(SEC_E_INVALID_HANDLE)
        }
        Some(SCHANNEL_SESSION) => {
            // SCHANNEL_SESSION_TOKEN { dwTokenType(3), dwFlags(1=enable, 2=disable reconnects) }
            // Per MSDN this controls TLS session-ID resumption cache invalidation; until per-context
            // session-cache control is wired into rustls, honestly report SEC_E_UNSUPPORTED_FUNCTION
            // rather than claiming success without disabling reconnections.
            let Some(flags) = second_word else {
                return SEC_E_INVALID_TOKEN;
            };
            if flags != 1 && flags != 2 {
                return SEC_E_INVALID_TOKEN;
            }
            SEC_E_UNSUPPORTED_FUNCTION
        }
        other => {
            // Per MSDN: unrecognized Schannel control token types return SEC_E_UNSUPPORTED_FUNCTION.
            log::debug!("ApplyControlToken(ctx={id}, token={other:?}) -> SEC_E_UNSUPPORTED_FUNCTION");
            SEC_E_UNSUPPORTED_FUNCTION
        }
    }
}

pub(super) unsafe fn encrypt_message_impl(
    ph_context: *mut c_void,
    p_message: *mut c_void,
) -> i32 {
    let Some(ctx_id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if p_message.is_null() {
        return SEC_E_INVALID_TOKEN;
    }
    let desc = std::ptr::read_unaligned(p_message as *const SecBufferDesc);
    if desc.p_buffers.is_null() || desc.c_buffers == 0 {
        return SEC_E_INVALID_TOKEN;
    }

    let mut hdr_idx: Option<usize> = None;
    let mut data_idx: Option<usize> = None;
    let mut trl_idx: Option<usize> = None;

    for i in 0..desc.c_buffers as usize {
        let sb = std::ptr::read_unaligned(desc.p_buffers.add(i));
        match sb.buffer_type & !SECBUFFER_ATTRMASK {
            SECBUFFER_STREAM_HEADER if hdr_idx.is_none() => hdr_idx = Some(i),
            SECBUFFER_DATA if data_idx.is_none() => data_idx = Some(i),
            SECBUFFER_STREAM_TRAILER if trl_idx.is_none() => trl_idx = Some(i),
            _ => {}
        }
    }

    let (Some(hi), Some(di), Some(ti)) = (hdr_idx, data_idx, trl_idx) else {
        return SEC_E_INVALID_TOKEN;
    };

    let mut hdr = std::ptr::read_unaligned(desc.p_buffers.add(hi));
    let mut data = std::ptr::read_unaligned(desc.p_buffers.add(di));
    let mut trl = std::ptr::read_unaligned(desc.p_buffers.add(ti));

    if hdr.pv_buffer.is_null()
        || trl.pv_buffer.is_null()
        || (data.cb_buffer > 0 && data.pv_buffer.is_null())
    {
        return SEC_E_INVALID_TOKEN;
    }

    // Copy plaintext BEFORE modifying any output buffer (they are often contiguous in memory).
    let plaintext = if data.cb_buffer > 0 {
        std::slice::from_raw_parts(data.pv_buffer as *const u8, data.cb_buffer as usize).to_vec()
    } else {
        Vec::new()
    };

    let ct = match CTXT_TABLE.with_mut(ctx_id, |ctx| {
        if ctx.shutdown_pending || ctx.shutdown_sent {
            return Err(SEC_E_CONTEXT_EXPIRED);
        }
        if ctx.conn.is_handshaking() {
            return Err(SEC_E_ENCRYPT_FAILURE);
        }
        if ctx.conn.writer().write_all(&plaintext).is_err() || ctx.conn.writer().flush().is_err() {
            return Err(SEC_E_ENCRYPT_FAILURE);
        }
        Ok(drain_tls_out(&mut ctx.conn))
    }) {
        Some(Ok(ct)) => ct,
        Some(Err(code)) => return code,
        None => return SEC_E_INVALID_HANDLE,
    };

    if ct.is_empty() {
        // Empty plaintext write: rustls emits no record for 0 bytes.
        hdr.cb_buffer = 0;
        data.cb_buffer = 0;
        trl.cb_buffer = 0;
        std::ptr::write_unaligned(desc.p_buffers.add(hi), hdr);
        std::ptr::write_unaligned(desc.p_buffers.add(di), data);
        std::ptr::write_unaligned(desc.p_buffers.add(ti), trl);
        return SEC_E_OK;
    }

    let hdr_cap = hdr.cb_buffer as usize;
    let data_cap = data.cb_buffer as usize;
    let trl_cap = trl.cb_buffer as usize;
    if ct.len() > hdr_cap + data_cap + trl_cap {
        return SEC_E_BUFFER_TOO_SMALL;
    }

    // Fill HEADER -> DATA -> TRAILER sequentially so both contiguous buffers (libcurl)
    // and scatter-gather buffers (WSASend) have zero gaps and exact ciphertext bytes.
    let n_hdr = ct.len().min(hdr_cap);
    std::ptr::copy_nonoverlapping(ct.as_ptr(), hdr.pv_buffer as *mut u8, n_hdr);
    hdr.cb_buffer = n_hdr as u32;

    let rem1 = &ct[n_hdr..];
    let n_data = rem1.len().min(data_cap);
    if n_data > 0 {
        std::ptr::copy_nonoverlapping(rem1.as_ptr(), data.pv_buffer as *mut u8, n_data);
    }
    data.cb_buffer = n_data as u32;

    let rem2 = &rem1[n_data..];
    if !rem2.is_empty() {
        std::ptr::copy_nonoverlapping(rem2.as_ptr(), trl.pv_buffer as *mut u8, rem2.len());
    }
    trl.cb_buffer = rem2.len() as u32;

    std::ptr::write_unaligned(desc.p_buffers.add(hi), hdr);
    std::ptr::write_unaligned(desc.p_buffers.add(di), data);
    std::ptr::write_unaligned(desc.p_buffers.add(ti), trl);
    SEC_E_OK
}

pub(super) unsafe fn decrypt_message_impl(
    ph_context: *mut c_void,
    p_message: *mut c_void,
    pf_qop: *mut u32,
) -> i32 {
    let Some(ctx_id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if p_message.is_null() {
        return SEC_E_INVALID_TOKEN;
    }
    let desc = std::ptr::read_unaligned(p_message as *const SecBufferDesc);
    if desc.p_buffers.is_null() || desc.c_buffers == 0 {
        return SEC_E_INVALID_TOKEN;
    }

    let mut data_buf: Option<SecBuffer> = None;
    for i in 0..desc.c_buffers as usize {
        let sb = std::ptr::read_unaligned(desc.p_buffers.add(i));
        if (sb.buffer_type & !SECBUFFER_ATTRMASK) == SECBUFFER_DATA {
            data_buf = Some(sb);
            break;
        }
    }
    let Some(db) = data_buf else {
        return SEC_E_INVALID_TOKEN;
    };
    if db.pv_buffer.is_null() || db.cb_buffer == 0 {
        set_input_aux_buffer(
            p_message as *mut SecBufferDesc,
            SECBUFFER_MISSING,
            std::ptr::null_mut(),
            5,
        );
        return SEC_E_INCOMPLETE_MESSAGE;
    }

    let base_ptr = db.pv_buffer as *mut u8;
    let total_len = db.cb_buffer as usize;

    CTXT_TABLE
        .with_mut(ctx_id, |ctx| {
            if ctx.pending_eof && ctx.pending_plaintext.is_empty() {
                ctx.pending_eof = false;
                unsafe { write_decrypt_buffers(desc, base_ptr, 0, 0, total_len) };
                return SEC_I_CONTEXT_EXPIRED;
            }

            let input = unsafe { std::slice::from_raw_parts(base_ptr as *const u8, total_len) };
            let mut consumed = 0usize;
            let mut missing_bytes = 0usize;
            let mut plaintext = std::mem::take(&mut ctx.pending_plaintext);
            let mut saw_eof = false;
            let mut tmp = [0u8; 4096];

            while consumed < total_len {
                match inspect_tls_record(&input[consumed..]) {
                    Err(missing) => {
                        missing_bytes = missing;
                        break;
                    }
                    Ok(None) => {
                        return SEC_E_DECRYPT_FAILURE;
                    }
                    Ok(Some(rec_total)) => {
                        let mut rec = &input[consumed..consumed + rec_total];
                        while !rec.is_empty() {
                            match ctx.conn.read_tls(&mut rec) {
                                Ok(0) => break,
                                Ok(_) => {}
                                Err(_) => return SEC_E_DECRYPT_FAILURE,
                            }
                        }
                        consumed += rec_total;

                        let io_state = match ctx.conn.process_new_packets() {
                            Ok(st) => st,
                            Err(ref e) => {
                                log::debug!("DecryptMessage(ctx={ctx_id}) error: {e}");
                                return map_rustls_error_to_sec(e);
                            }
                        };

                        loop {
                            match ctx.conn.reader().read(&mut tmp) {
                                Ok(0) => {
                                    saw_eof = true;
                                    break;
                                }
                                Ok(n) => plaintext.extend_from_slice(&tmp[..n]),
                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                                Err(_) => return SEC_E_DECRYPT_FAILURE,
                            }
                        }
                        if io_state.peer_has_closed() {
                            saw_eof = true;
                        }

                        // Stop after delivering one decrypted payload (or EOF) so trailing records
                        // stay cleanly in SECBUFFER_EXTRA. Continue past 0-plaintext TLS 1.3
                        // NewSessionTicket records when another complete record is already buffered.
                        if !plaintext.is_empty() || saw_eof {
                            break;
                        }
                    }
                }
            }

            if consumed == 0 && plaintext.is_empty() {
                unsafe {
                    set_input_aux_buffer(
                        p_message as *mut SecBufferDesc,
                        SECBUFFER_MISSING,
                        std::ptr::null_mut(),
                        missing_bytes.max(1) as u32,
                    );
                }
                return SEC_E_INCOMPLETE_MESSAGE;
            }

            if !pf_qop.is_null() {
                unsafe { std::ptr::write_unaligned(pf_qop, 0u32) };
            }

            if plaintext.is_empty() && saw_eof {
                unsafe { write_decrypt_buffers(desc, base_ptr, 0, consumed, total_len) };
                return SEC_I_CONTEXT_EXPIRED;
            }

            if saw_eof {
                // Deliver final plaintext chunk now; report SEC_I_CONTEXT_EXPIRED on the next call.
                ctx.pending_eof = true;
            }

            let copy_len = plaintext.len().min(consumed);
            if copy_len < plaintext.len() {
                ctx.pending_plaintext = plaintext.split_off(copy_len);
            }
            unsafe {
                if copy_len > 0 {
                    std::ptr::copy_nonoverlapping(plaintext.as_ptr(), base_ptr, copy_len);
                }
                write_decrypt_buffers(desc, base_ptr, copy_len, consumed, total_len);
            }
            SEC_E_OK
        })
        .unwrap_or(SEC_E_INVALID_HANDLE)
}

unsafe fn write_decrypt_buffers(
    desc: SecBufferDesc,
    base_ptr: *mut u8,
    plain_len: usize,
    consumed: usize,
    total_len: usize,
) {
    let extra = total_len.saturating_sub(consumed);
    let c = desc.c_buffers as usize;
    if c >= 4 {
        std::ptr::write_unaligned(
            desc.p_buffers.add(0),
            SecBuffer {
                cb_buffer: 0,
                buffer_type: SECBUFFER_STREAM_HEADER,
                pv_buffer: base_ptr as *mut c_void,
            },
        );
        std::ptr::write_unaligned(
            desc.p_buffers.add(1),
            SecBuffer {
                cb_buffer: plain_len as u32,
                buffer_type: SECBUFFER_DATA,
                pv_buffer: base_ptr as *mut c_void,
            },
        );
        std::ptr::write_unaligned(
            desc.p_buffers.add(2),
            SecBuffer {
                cb_buffer: consumed.saturating_sub(plain_len) as u32,
                buffer_type: SECBUFFER_STREAM_TRAILER,
                pv_buffer: base_ptr.add(plain_len) as *mut c_void,
            },
        );
        if extra > 0 {
            std::ptr::write_unaligned(
                desc.p_buffers.add(3),
                SecBuffer {
                    cb_buffer: extra as u32,
                    buffer_type: SECBUFFER_EXTRA,
                    pv_buffer: base_ptr.add(consumed) as *mut c_void,
                },
            );
        } else {
            std::ptr::write_unaligned(
                desc.p_buffers.add(3),
                SecBuffer {
                    cb_buffer: 0,
                    buffer_type: SECBUFFER_EMPTY,
                    pv_buffer: std::ptr::null_mut(),
                },
            );
        }
    } else if c >= 1 {
        std::ptr::write_unaligned(
            desc.p_buffers.add(0),
            SecBuffer {
                cb_buffer: plain_len as u32,
                buffer_type: SECBUFFER_DATA,
                pv_buffer: base_ptr as *mut c_void,
            },
        );
        if c >= 2 {
            if extra > 0 {
                std::ptr::write_unaligned(
                    desc.p_buffers.add(1),
                    SecBuffer {
                        cb_buffer: extra as u32,
                        buffer_type: SECBUFFER_EXTRA,
                        pv_buffer: base_ptr.add(consumed) as *mut c_void,
                    },
                );
            } else {
                std::ptr::write_unaligned(
                    desc.p_buffers.add(1),
                    SecBuffer {
                        cb_buffer: 0,
                        buffer_type: SECBUFFER_EMPTY,
                        pv_buffer: std::ptr::null_mut(),
                    },
                );
            }
        }
    }
}

fn write_wide_fixed(dst: &mut [u16; SZ_ALG_MAX_SIZE], s: &str) {
    dst.fill(0);
    for (slot, u) in dst.iter_mut().take(SZ_ALG_MAX_SIZE - 1).zip(s.encode_utf16()) {
        *slot = u;
    }
}

unsafe fn alloc_sec_string(s: &str, wide: bool) -> *mut c_void {
    if wide {
        let cap = s.encode_utf16().count() + 1;
        let mem = crate::heap::alloc(cap * 2, false) as *mut u16;
        if mem.is_null() {
            return std::ptr::null_mut();
        }
        crate::strings::write_wide_z(mem, cap, s);
        mem as *mut c_void
    } else {
        let cap = s.len() + 1;
        let mem = crate::heap::alloc(cap, false) as *mut u8;
        if mem.is_null() {
            return std::ptr::null_mut();
        }
        crate::strings::write_ansi_z(mem, cap, s);
        mem as *mut c_void
    }
}

fn kx_strength(conn: &Connection) -> u32 {
    match conn.negotiated_key_exchange_group().map(|g| g.name()) {
        Some(rustls::NamedGroup::secp384r1) => 384,
        Some(rustls::NamedGroup::secp521r1) => 521,
        _ => 256,
    }
}

fn negotiated_protocol_bits(conn: &Connection, is_server: bool) -> u32 {
    match (conn.protocol_version(), is_server) {
        (Some(ProtocolVersion::TLSv1_3), true) => SP_PROT_TLS1_3_SERVER,
        (Some(ProtocolVersion::TLSv1_3), false) => SP_PROT_TLS1_3_CLIENT,
        (_, true) => SP_PROT_TLS1_2_SERVER,
        (_, false) => SP_PROT_TLS1_2_CLIENT,
    }
}

const SUPPORTED_SCHANNEL_ALGS: &[u32] = &[
    CALG_AES_128,
    CALG_AES_256,
    CALG_SHA_256,
    CALG_SHA_384,
    CALG_ECDH_EPHEM,
    CALG_RSA_SIGN,
    CALG_ECDSA,
];

fn cipher_suite_metadata(
    suite: Option<rustls::CipherSuite>,
) -> (
    u32,
    &'static str,
    &'static str,
    u32,
    u32,
    &'static str,
    u32,
    &'static str,
    &'static str,
) {
    // Returns: (iana_code, suite_name, cipher_name, cipher_bits, block_bytes, hash_name, hash_bits, exch_name, cert_name)
    match suite {
        Some(rustls::CipherSuite::TLS13_AES_256_GCM_SHA384) => (
            0x1302,
            "TLS_AES_256_GCM_SHA384",
            "AES",
            256,
            16,
            "SHA384",
            384,
            "ECDHE",
            "RSA",
        ),
        Some(rustls::CipherSuite::TLS13_AES_128_GCM_SHA256) => (
            0x1301,
            "TLS_AES_128_GCM_SHA256",
            "AES",
            128,
            16,
            "SHA256",
            256,
            "ECDHE",
            "RSA",
        ),
        Some(rustls::CipherSuite::TLS13_CHACHA20_POLY1305_SHA256) => (
            0x1303,
            "TLS_CHACHA20_POLY1305_SHA256",
            "CHACHA20",
            256,
            1,
            "SHA256",
            256,
            "ECDHE",
            "RSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384) => (
            0xC02C,
            "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
            "AES",
            256,
            16,
            "SHA384",
            384,
            "ECDHE",
            "ECDSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256) => (
            0xC02B,
            "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
            "AES",
            128,
            16,
            "SHA256",
            256,
            "ECDHE",
            "ECDSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256) => (
            0xCCA9,
            "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
            "CHACHA20",
            256,
            1,
            "SHA256",
            256,
            "ECDHE",
            "ECDSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384) => (
            0xC030,
            "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
            "AES",
            256,
            16,
            "SHA384",
            384,
            "ECDHE",
            "RSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256) => (
            0xC02F,
            "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
            "AES",
            128,
            16,
            "SHA256",
            256,
            "ECDHE",
            "RSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256) => (
            0xCCA8,
            "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
            "CHACHA20",
            256,
            1,
            "SHA256",
            256,
            "ECDHE",
            "RSA",
        ),
        Some(other) => (
            u16::from(other) as u32,
            "TLS_AES_128_GCM_SHA256",
            "AES",
            128,
            16,
            "SHA256",
            256,
            "ECDHE",
            "RSA",
        ),
        None => (0, "", "", 0, 0, "", 0, "", ""),
    }
}

pub(super) unsafe fn query_context_attributes_impl(
    ph_context: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
    wide: bool,
) -> i32 {
    if p_buffer.is_null() {
        return SEC_E_INVALID_HANDLE;
    }
    let Some(ctx_id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };

    CTXT_TABLE
        .with(ctx_id, |ctx| match ul_attribute {
            SECPKG_ATTR_SIZES => {
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextSizes,
                        SecPkgContextSizes {
                            cb_max_token: STREAM_MAX_MESSAGE,
                            cb_max_signature: 0,
                            cb_block_size: 1,
                            cb_security_trailer: STREAM_TRAILER_SIZE,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_NAMES => {
                let name = if !ctx.target_name.is_empty() {
                    ctx.target_name.clone()
                } else {
                    "Schannel".to_string()
                };
                let s_ptr = unsafe { alloc_sec_string(&name, wide) };
                if s_ptr.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                unsafe {
                    if wide {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextNamesW,
                            SecPkgContextNamesW {
                                s_user_name: s_ptr as *mut u16,
                            },
                        );
                    } else {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextNamesA,
                            SecPkgContextNamesA {
                                s_user_name: s_ptr as *mut u8,
                            },
                        );
                    }
                }
                SEC_E_OK
            }
            SECPKG_ATTR_STREAM_SIZES => {
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextStreamSizes,
                        SecPkgContextStreamSizes {
                            cb_header: STREAM_HEADER_SIZE,
                            cb_trailer: STREAM_TRAILER_SIZE,
                            cb_maximum_message: STREAM_MAX_MESSAGE,
                            c_buffers: 4,
                            cb_block_size: 1,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_PACKAGE_INFO => {
                let mut pkg_ptr: *mut c_void = std::ptr::null_mut();
                let rc = unsafe {
                    if wide {
                        let w: Vec<u16> = "Schannel\0".encode_utf16().collect();
                        query_security_package_info_w_impl(w.as_ptr(), &mut pkg_ptr)
                    } else {
                        query_security_package_info_a_impl(b"Schannel\0".as_ptr(), &mut pkg_ptr)
                    }
                };
                if rc != SEC_E_OK {
                    return rc;
                }
                unsafe { std::ptr::write_unaligned(p_buffer as *mut *mut c_void, pkg_ptr) };
                SEC_E_OK
            }
            SECPKG_ATTR_REMOTE_CERT_CONTEXT | SECPKG_ATTR_REMOTE_CERT_CHAIN => {
                // Both SECPKG_ATTR_REMOTE_CERT_CONTEXT (0x53) and SECPKG_ATTR_REMOTE_CERT_CHAIN
                // (0x67) return a PCCERT_CONTEXT whose hCertStore contains the peer's certificate
                // chain (freed via CertFreeCertificateContext).
                let Some(certs) = ctx.conn.peer_certificates() else {
                    return SEC_E_NO_CREDENTIALS;
                };
                if certs.is_empty() {
                    return SEC_E_NO_CREDENTIALS;
                }
                let chain_der: Vec<Vec<u8>> = certs.iter().map(|c| c.as_ref().to_vec()).collect();
                let verify_error = if ctx.manual_validation {
                    crypt32::verify_peer_chain_error(
                        &chain_der,
                        (!ctx.target_name.is_empty()).then_some(ctx.target_name.as_str()),
                        &ctx.extra_roots_der,
                        ctx.no_servername_check,
                    )
                } else {
                    0
                };
                let cert_ptr = crypt32::register_peer_cert_chain(
                    &chain_der,
                    PeerCertVerifyResult {
                        cert_error: verify_error,
                        extra_roots_der: ctx.extra_roots_der.clone(),
                    },
                );
                if cert_ptr.is_null() {
                    return SEC_E_INTERNAL_ERROR;
                }
                unsafe { std::ptr::write_unaligned(p_buffer as *mut *mut c_void, cert_ptr) };
                SEC_E_OK
            }
            SECPKG_ATTR_LIFESPAN => {
                let mut start = 0i64;
                let mut expiry = i64::MAX;
                if let Some(cert_der) = ctx
                    .conn
                    .peer_certificates()
                    .and_then(|c| c.first())
                    .map(|c| c.as_ref())
                    .or(ctx.local_cert_der.as_deref())
                {
                    if let Some(owned) =
                        crate::misc::wintrust::build_owned_cert_context(cert_der, std::ptr::null_mut())
                    {
                        unsafe {
                            let ci = &*(*owned.as_ptr()).p_cert_info;
                            start = ci.not_before as i64;
                            expiry = ci.not_after as i64;
                        }
                    }
                }
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextLifespan,
                        SecPkgContextLifespan {
                            ts_start: start,
                            ts_expiry: expiry,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_KEY_INFO => {
                let suite = ctx.conn.negotiated_cipher_suite().map(|s| s.suite());
                let (_, _, cipher_name, cipher_len, _, hash_name, _, _, _) =
                    cipher_suite_metadata(suite);
                let sig_ptr = unsafe { alloc_sec_string(hash_name, wide) };
                let enc_ptr = unsafe { alloc_sec_string(cipher_name, wide) };
                if sig_ptr.is_null() || enc_ptr.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                let ai_cipher = if cipher_len >= 256 { CALG_AES_256 } else { CALG_AES_128 };
                let ai_hash = if hash_name == "SHA384" { CALG_SHA_384 } else { CALG_SHA_256 };
                unsafe {
                    if wide {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextKeyInfoW,
                            SecPkgContextKeyInfoW {
                                s_signature_algorithm_name: sig_ptr as *mut u16,
                                s_encrypt_algorithm_name: enc_ptr as *mut u16,
                                key_size: cipher_len,
                                signature_algorithm: ai_hash,
                                encrypt_algorithm: ai_cipher,
                            },
                        );
                    } else {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextKeyInfoA,
                            SecPkgContextKeyInfoA {
                                s_signature_algorithm_name: sig_ptr as *mut u8,
                                s_encrypt_algorithm_name: enc_ptr as *mut u8,
                                key_size: cipher_len,
                                signature_algorithm: ai_hash,
                                encrypt_algorithm: ai_cipher,
                            },
                        );
                    }
                }
                SEC_E_OK
            }
            SECPKG_ATTR_AUTHORITY => {
                let auth_name = ctx
                    .conn
                    .peer_certificates()
                    .and_then(|c| c.first())
                    .and_then(|c| {
                        let owned = crate::misc::wintrust::build_owned_cert_context(
                            c.as_ref(),
                            std::ptr::null_mut(),
                        )?;
                        unsafe {
                            let ci = &*(*owned.as_ptr()).p_cert_info;
                            if ci.issuer.pb_data.is_null() || ci.issuer.cb_data == 0 {
                                return None;
                            }
                            let der = std::slice::from_raw_parts(
                                ci.issuer.pb_data,
                                ci.issuer.cb_data as usize,
                            );
                            crate::misc::wintrust::format_dn(der)
                        }
                    })
                    .unwrap_or_else(|| "Schannel".to_string());
                let s_ptr = unsafe { alloc_sec_string(&auth_name, wide) };
                if s_ptr.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                unsafe {
                    if wide {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextNamesW,
                            SecPkgContextNamesW {
                                s_user_name: s_ptr as *mut u16,
                            },
                        );
                    } else {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextNamesA,
                            SecPkgContextNamesA {
                                s_user_name: s_ptr as *mut u8,
                            },
                        );
                    }
                }
                SEC_E_OK
            }
            SECPKG_ATTR_PROTO_INFO => {
                let (pname, minor) = match ctx.conn.protocol_version() {
                    Some(ProtocolVersion::TLSv1_3) => ("TLS 1.3", 4u32),
                    _ => ("TLS 1.2", 3u32),
                };
                let s_ptr = unsafe { alloc_sec_string(pname, wide) };
                if s_ptr.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                unsafe {
                    if wide {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextProtoInfoW,
                            SecPkgContextProtoInfoW {
                                s_protocol_name: s_ptr as *mut u16,
                                major_version: 3,
                                minor_version: minor,
                            },
                        );
                    } else {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextProtoInfoA,
                            SecPkgContextProtoInfoA {
                                s_protocol_name: s_ptr as *mut u8,
                                major_version: 3,
                                minor_version: minor,
                            },
                        );
                    }
                }
                SEC_E_OK
            }
            SECPKG_ATTR_NEGOTIATION_INFO => {
                let mut pkg_ptr: *mut c_void = std::ptr::null_mut();
                let rc = unsafe {
                    if wide {
                        let w: Vec<u16> = "Schannel\0".encode_utf16().collect();
                        query_security_package_info_w_impl(w.as_ptr(), &mut pkg_ptr)
                    } else {
                        query_security_package_info_a_impl(b"Schannel\0".as_ptr(), &mut pkg_ptr)
                    }
                };
                if rc != SEC_E_OK {
                    return rc;
                }
                let state = if ctx.conn.is_handshaking() { 2u32 } else { 0u32 };
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextNegotiationInfo,
                        SecPkgContextNegotiationInfo {
                            package_info: pkg_ptr,
                            negotiation_state: state,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_FLAGS => {
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextFlags,
                        SecPkgContextFlags {
                            flags: ctx.context_attr,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_UNIQUE_BINDINGS => {
                // RFC 5929 §3 `tls-unique` requires the raw TLS 1.2 first Finished verify_data
                // bytes and is undefined in TLS 1.3 (RFC 8446 / RFC 9266, where Windows Schannel
                // returns SEC_E_UNSUPPORTED_FUNCTION). rustls intentionally does not expose TLS 1.2
                // Finished bytes, so report SEC_E_UNSUPPORTED_FUNCTION rather than returning
                // endpoint-binding data under the unique-bindings attribute.
                SEC_E_UNSUPPORTED_FUNCTION
            }
            SECPKG_ATTR_ENDPOINT_BINDINGS => {
                // RFC 5929 §4.1 tls-server-end-point channel binding: hash the server leaf cert
                // DER with its signature algorithm's digest (substituting SHA-256 for MD5/SHA-1).
                let server_cert_der = if ctx.is_server {
                    ctx.local_cert_der.as_deref()
                } else {
                    ctx.conn
                        .peer_certificates()
                        .and_then(|c| c.first())
                        .map(|c| c.as_ref())
                };
                let Some(cert_der) = server_cert_der else {
                    return SEC_E_NO_CREDENTIALS;
                };
                let hash = rfc5929_endpoint_binding_digest(cert_der).hash(cert_der);
                let mut app_data = Vec::with_capacity(21 + hash.len());
                app_data.extend_from_slice(b"tls-server-end-point:");
                app_data.extend_from_slice(&hash);
                let total = 32 + app_data.len();
                let mem = crate::heap::alloc(total, true) as *mut u8;
                if mem.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                unsafe {
                    // SEC_CHANNEL_BINDINGS: cbApplicationDataLength @ +24, dwApplicationDataOffset @ +28
                    std::ptr::write_unaligned(mem.add(24) as *mut u32, app_data.len() as u32);
                    std::ptr::write_unaligned(mem.add(28) as *mut u32, 32u32);
                    std::ptr::copy_nonoverlapping(app_data.as_ptr(), mem.add(32), app_data.len());
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextBindings,
                        SecPkgContextBindings {
                            bindings_length: total as u32,
                            bindings: mem as *mut c_void,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_CLIENT_SPECIFIED_TARGET => {
                let s_ptr = unsafe { alloc_sec_string(&ctx.target_name, wide) };
                if s_ptr.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                unsafe {
                    if wide {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextNamesW,
                            SecPkgContextNamesW {
                                s_user_name: s_ptr as *mut u16,
                            },
                        );
                    } else {
                        std::ptr::write_unaligned(
                            p_buffer as *mut SecPkgContextNamesA,
                            SecPkgContextNamesA {
                                s_user_name: s_ptr as *mut u8,
                            },
                        );
                    }
                }
                SEC_E_OK
            }
            SECPKG_ATTR_LOCAL_CERT_CONTEXT => {
                let Some(ref der) = ctx.local_cert_der else {
                    return SEC_E_NO_CREDENTIALS;
                };
                let cert_ptr = crypt32::duplicate_or_create_cert_context_der(der);
                if cert_ptr.is_null() {
                    return SEC_E_INTERNAL_ERROR;
                }
                unsafe { std::ptr::write_unaligned(p_buffer as *mut *mut c_void, cert_ptr) };
                SEC_E_OK
            }
            SECPKG_ATTR_ROOT_STORE => {
                let store = unsafe { crypt32::CertOpenSystemStoreA(0, b"ROOT\0".as_ptr()) };
                if store.is_null() {
                    return SEC_E_INTERNAL_ERROR;
                }
                unsafe { std::ptr::write_unaligned(p_buffer as *mut *mut c_void, store) };
                SEC_E_OK
            }
            SECPKG_ATTR_SUPPORTED_ALGS => {
                let bytes = SUPPORTED_SCHANNEL_ALGS.len() * 4;
                let mem = crate::heap::alloc(bytes, false) as *mut u32;
                if mem.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                unsafe {
                    for (i, &alg) in SUPPORTED_SCHANNEL_ALGS.iter().enumerate() {
                        std::ptr::write_unaligned(mem.add(i), alg);
                    }
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgCredSupportedAlgs,
                        SecPkgCredSupportedAlgs {
                            c_supported_algs: SUPPORTED_SCHANNEL_ALGS.len() as u32,
                            palg_supported_algs: mem,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_CIPHER_STRENGTHS => {
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgCredCipherStrengths,
                        SecPkgCredCipherStrengths {
                            dw_minimum_cipher_strength: 128,
                            dw_maximum_cipher_strength: 256,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_SUPPORTED_PROTOCOLS => {
                let prot = if ctx.is_server {
                    SP_PROT_TLS1_2_SERVER | SP_PROT_TLS1_3_SERVER
                } else {
                    SP_PROT_TLS1_2_CLIENT | SP_PROT_TLS1_3_CLIENT
                };
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgCredSupportedProtocols,
                        SecPkgCredSupportedProtocols {
                            grbit_protocol: prot,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_EAP_KEY_BLOCK => {
                let mut kb = SecPkgContextEapKeyBlock {
                    rgb_keys: [0u8; 128],
                    rgb_ivs: [0u8; 64],
                };
                match ctx.conn.protocol_version() {
                    Some(ProtocolVersion::TLSv1_3) => {
                        // RFC 9190 §2.3 (EAP-TLS 1.3):
                        // Key_Material = TLS-Exporter("EXPORTER_EAP_TLS_Key_Material", 0x0D, 128)
                        if ctx
                            .conn
                            .export_keying_material(
                                &mut kb.rgb_keys,
                                b"EXPORTER_EAP_TLS_Key_Material",
                                Some(&[0x0Du8]),
                            )
                            .is_err()
                        {
                            return SEC_E_INVALID_HANDLE;
                        }
                    }
                    Some(ProtocolVersion::TLSv1_2) => {
                        // RFC 5216 §2.3 / RFC 5705 (EAP-TLS 1.2): 128B keys + 64B IVs
                        let mut mat = [0u8; 192];
                        if ctx
                            .conn
                            .export_keying_material(&mut mat, b"client EAP encryption", None)
                            .is_err()
                        {
                            return SEC_E_INVALID_HANDLE;
                        }
                        kb.rgb_keys.copy_from_slice(&mat[..128]);
                        kb.rgb_ivs.copy_from_slice(&mat[128..192]);
                    }
                    _ => return SEC_E_INVALID_HANDLE,
                }
                unsafe {
                    std::ptr::write_unaligned(p_buffer as *mut SecPkgContextEapKeyBlock, kb);
                }
                SEC_E_OK
            }
            SECPKG_ATTR_SESSION_INFO => {
                let Some(kind) = ctx.conn.handshake_kind() else {
                    return SEC_E_INVALID_HANDLE;
                };
                let dw_flags = match kind {
                    rustls::HandshakeKind::Resumed => SSL_SESSION_RECONNECT,
                    rustls::HandshakeKind::Full
                    | rustls::HandshakeKind::FullWithHelloRetryRequest => 0,
                };
                let mut rgb_session_id = [0u8; 32];
                let cb_session_id = if ctx.conn.protocol_version() == Some(ProtocolVersion::TLSv1_2)
                    && !ctx.tls12_session_id.is_empty()
                {
                    let n = ctx.tls12_session_id.len().min(32);
                    rgb_session_id[..n].copy_from_slice(&ctx.tls12_session_id[..n]);
                    n as u32
                } else {
                    0
                };
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextSessionInfo,
                        SecPkgContextSessionInfo {
                            dw_flags,
                            cb_session_id,
                            rgb_session_id,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_REMOTE_CERTIFICATES => {
                // Windows SDK schannel.h: 0x5F returns SecPkgContext_Certificates where
                // pbCertificateChain (freed via FreeContextBuffer) packs each DER cert as
                // <DWORD length (LE)> <cert DER bytes>.
                let Some(certs) = ctx.conn.peer_certificates() else {
                    return SEC_E_NO_CREDENTIALS;
                };
                if certs.is_empty() {
                    return SEC_E_NO_CREDENTIALS;
                }
                let total_bytes: usize = certs.iter().map(|c| 4 + c.as_ref().len()).sum();
                let mem = crate::heap::alloc(total_bytes, false) as *mut u8;
                if mem.is_null() {
                    return SEC_E_INSUFFICIENT_MEMORY;
                }
                let mut off = 0usize;
                for c in certs {
                    let der = c.as_ref();
                    let len_le = (der.len() as u32).to_le_bytes();
                    unsafe {
                        std::ptr::copy_nonoverlapping(len_le.as_ptr(), mem.add(off), 4);
                        off += 4;
                        std::ptr::copy_nonoverlapping(der.as_ptr(), mem.add(off), der.len());
                        off += der.len();
                    }
                }
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextCertificates,
                        SecPkgContextCertificates {
                            c_certs: certs.len() as u32,
                            cb_cert_chain: total_bytes as u32,
                            pb_certificate_chain: mem,
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_CONNECTION_INFO => {
                let dw_protocol = negotiated_protocol_bits(&ctx.conn, ctx.is_server);
                let (ai_cipher, dw_cipher_strength, ai_hash, dw_hash_strength) =
                    match ctx.conn.negotiated_cipher_suite().map(|s| s.suite()) {
                        Some(
                            rustls::CipherSuite::TLS13_AES_256_GCM_SHA384
                            | rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
                            | rustls::CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                        ) => (CALG_AES_256, 256, CALG_SHA_384, 384),
                        Some(
                            rustls::CipherSuite::TLS13_CHACHA20_POLY1305_SHA256
                            | rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256
                            | rustls::CipherSuite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
                        ) => (CALG_AES_256, 256, CALG_SHA_256, 256),
                        _ => (CALG_AES_128, 128, CALG_SHA_256, 256),
                    };
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextConnectionInfo,
                        SecPkgContextConnectionInfo {
                            dw_protocol,
                            ai_cipher,
                            dw_cipher_strength,
                            ai_hash,
                            dw_hash_strength,
                            ai_exch: CALG_ECDH_EPHEM,
                            dw_exch_strength: kx_strength(&ctx.conn),
                        },
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_CIPHER_INFO => {
                let dw_protocol = negotiated_protocol_bits(&ctx.conn, ctx.is_server);
                let suite = ctx.conn.negotiated_cipher_suite().map(|s| s.suite());
                let (
                    code,
                    suite_name,
                    cipher_name,
                    cipher_len,
                    block_len,
                    hash_name,
                    hash_len,
                    exch_name,
                    cert_name,
                ) = cipher_suite_metadata(suite);
                let exch_bits = kx_strength(&ctx.conn);
                let mut info = SecPkgContextCipherInfo {
                    dw_version: SECPKGCONTEXT_CIPHERINFO_V1,
                    dw_protocol,
                    dw_cipher_suite: code,
                    dw_base_cipher_suite: code,
                    sz_cipher_suite: [0; SZ_ALG_MAX_SIZE],
                    sz_cipher: [0; SZ_ALG_MAX_SIZE],
                    dw_cipher_len: cipher_len,
                    dw_cipher_block_len: block_len,
                    sz_hash: [0; SZ_ALG_MAX_SIZE],
                    dw_hash_len: hash_len,
                    sz_exchange: [0; SZ_ALG_MAX_SIZE],
                    dw_min_exchange_len: exch_bits,
                    dw_max_exchange_len: exch_bits,
                    sz_certificate: [0; SZ_ALG_MAX_SIZE],
                    dw_key_type: 0,
                };
                write_wide_fixed(&mut info.sz_cipher_suite, suite_name);
                write_wide_fixed(&mut info.sz_cipher, cipher_name);
                write_wide_fixed(&mut info.sz_hash, hash_name);
                write_wide_fixed(&mut info.sz_exchange, exch_name);
                write_wide_fixed(&mut info.sz_certificate, cert_name);
                unsafe {
                    std::ptr::write_unaligned(p_buffer as *mut SecPkgContextCipherInfo, info);
                }
                SEC_E_OK
            }
            SECPKG_ATTR_APPLICATION_PROTOCOL => {
                let mut info = SecPkgContextApplicationProtocol {
                    proto_nego_status: 0,
                    proto_nego_ext: 0,
                    protocol_id_size: 0,
                    protocol_id: [0u8; 255],
                };
                if let Some(alpn) = ctx.conn.alpn_protocol() {
                    let n = alpn.len().min(255);
                    info.proto_nego_status = 1; // SecApplicationProtocolNegotiationStatus_Success
                    info.proto_nego_ext = 2; // SecApplicationProtocolNegotiationExt_ALPN
                    info.protocol_id_size = n as u8;
                    info.protocol_id[..n].copy_from_slice(&alpn[..n]);
                }
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextApplicationProtocol,
                        info,
                    );
                }
                SEC_E_OK
            }
            SECPKG_ATTR_ISSUER_LIST_EX => {
                unsafe {
                    std::ptr::write_unaligned(
                        p_buffer as *mut SecPkgContextIssuerListInfoEx,
                        SecPkgContextIssuerListInfoEx {
                            a_issuers: std::ptr::null_mut(),
                            c_issuers: 0,
                        },
                    );
                }
                SEC_E_OK
            }
            other => {
                // Per MSDN: attributes not supported by Schannel return SEC_E_UNSUPPORTED_FUNCTION.
                log::debug!("QueryContextAttributes(ctx={ctx_id}, attr={other:#x}) -> SEC_E_UNSUPPORTED_FUNCTION");
                SEC_E_UNSUPPORTED_FUNCTION
            }
        })
        .unwrap_or(SEC_E_INVALID_HANDLE)
}

pub(super) unsafe fn query_credentials_attributes_impl(
    ph_credential: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
    wide: bool,
) -> i32 {
    if p_buffer.is_null() {
        return SEC_E_INVALID_HANDLE;
    }
    let Some(cred_id) = read_handle(ph_credential as *const SecHandle, MAGIC_CRED) else {
        return SEC_E_INVALID_HANDLE;
    };
    let Some(cred) = CRED_TABLE.get_cloned(cred_id) else {
        return SEC_E_INVALID_HANDLE;
    };
    match ul_attribute {
        SECPKG_CRED_ATTR_NAMES => {
            let s_ptr = alloc_sec_string(&cred.principal, wide);
            if s_ptr.is_null() {
                return SEC_E_INSUFFICIENT_MEMORY;
            }
            if wide {
                std::ptr::write_unaligned(
                    p_buffer as *mut SecPkgContextNamesW,
                    SecPkgContextNamesW {
                        s_user_name: s_ptr as *mut u16,
                    },
                );
            } else {
                std::ptr::write_unaligned(
                    p_buffer as *mut SecPkgContextNamesA,
                    SecPkgContextNamesA {
                        s_user_name: s_ptr as *mut u8,
                    },
                );
            }
            SEC_E_OK
        }
        SECPKG_CRED_ATTR_SUPPORTED_ALGS => {
            let bytes = SUPPORTED_SCHANNEL_ALGS.len() * 4;
            let mem = crate::heap::alloc(bytes, false) as *mut u32;
            if mem.is_null() {
                return SEC_E_INSUFFICIENT_MEMORY;
            }
            for (i, &alg) in SUPPORTED_SCHANNEL_ALGS.iter().enumerate() {
                std::ptr::write_unaligned(mem.add(i), alg);
            }
            std::ptr::write_unaligned(
                p_buffer as *mut SecPkgCredSupportedAlgs,
                SecPkgCredSupportedAlgs {
                    c_supported_algs: SUPPORTED_SCHANNEL_ALGS.len() as u32,
                    palg_supported_algs: mem,
                },
            );
            SEC_E_OK
        }
        SECPKG_CRED_ATTR_CIPHER_STRENGTHS => {
            std::ptr::write_unaligned(
                p_buffer as *mut SecPkgCredCipherStrengths,
                SecPkgCredCipherStrengths {
                    dw_minimum_cipher_strength: 128,
                    dw_maximum_cipher_strength: 256,
                },
            );
            SEC_E_OK
        }
        SECPKG_CRED_ATTR_SUPPORTED_PROTOCOLS => {
            let mut prot = 0u32;
            if (cred.cred_use & SECPKG_CRED_OUTBOUND) != 0 {
                if cred.client_enable_tls12 {
                    prot |= SP_PROT_TLS1_2_CLIENT;
                }
                if cred.client_enable_tls13 {
                    prot |= SP_PROT_TLS1_3_CLIENT;
                }
            }
            if (cred.cred_use & SECPKG_CRED_INBOUND) != 0 {
                if cred.server_enable_tls12 {
                    prot |= SP_PROT_TLS1_2_SERVER;
                }
                if cred.server_enable_tls13 {
                    prot |= SP_PROT_TLS1_3_SERVER;
                }
            }
            std::ptr::write_unaligned(
                p_buffer as *mut SecPkgCredSupportedProtocols,
                SecPkgCredSupportedProtocols {
                    grbit_protocol: prot,
                },
            );
            SEC_E_OK
        }
        _ => {
            // Per MSDN: attributes not supported by Schannel credentials return SEC_E_UNSUPPORTED_FUNCTION.
            log::debug!(
                "QueryCredentialsAttributes{}(attr={ul_attribute:#x}) -> SEC_E_UNSUPPORTED_FUNCTION",
                if wide { "W" } else { "A" }
            );
            SEC_E_UNSUPPORTED_FUNCTION
        }
    }
}

pub(super) unsafe fn query_security_package_info_a_impl(
    psz_package_name: *const u8,
    pp_package_info: *mut *mut c_void,
) -> i32 {
    if psz_package_name.is_null() || pp_package_info.is_null() {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    let name = crate::strings::from_ansi(psz_package_name);
    if !is_schannel_package(&name) {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    static PKG_NAME_A: &[u8] = b"Schannel\0";
    static PKG_COMMENT_A: &[u8] = b"Schannel Security Package\0";
    let mem = crate::heap::alloc(std::mem::size_of::<SecPkgInfoA>(), true) as *mut SecPkgInfoA;
    if mem.is_null() {
        return SEC_E_INSUFFICIENT_MEMORY;
    }
    std::ptr::write_unaligned(
        mem,
        SecPkgInfoA {
            f_capabilities: 0x0001_07B3, // INTEGRITY | PRIVACY | CONNECTION | STREAM | MUTUAL_AUTH
            w_version: 1,
            w_rpcid: 14,
            cb_max_token: STREAM_MAX_MESSAGE,
            name: PKG_NAME_A.as_ptr(),
            comment: PKG_COMMENT_A.as_ptr(),
        },
    );
    std::ptr::write_unaligned(pp_package_info, mem as *mut c_void);
    SEC_E_OK
}

pub(super) unsafe fn query_security_package_info_w_impl(
    psz_package_name: *const u16,
    pp_package_info: *mut *mut c_void,
) -> i32 {
    if psz_package_name.is_null() || pp_package_info.is_null() {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    let name = crate::strings::from_wide(psz_package_name);
    if !is_schannel_package(&name) {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    static PKG_NAME_W: OnceLock<Vec<u16>> = OnceLock::new();
    static PKG_COMMENT_W: OnceLock<Vec<u16>> = OnceLock::new();
    let nw = PKG_NAME_W.get_or_init(|| "Schannel\0".encode_utf16().collect());
    let cw = PKG_COMMENT_W.get_or_init(|| "Schannel Security Package\0".encode_utf16().collect());
    let mem = crate::heap::alloc(std::mem::size_of::<SecPkgInfoW>(), true) as *mut SecPkgInfoW;
    if mem.is_null() {
        return SEC_E_INSUFFICIENT_MEMORY;
    }
    std::ptr::write_unaligned(
        mem,
        SecPkgInfoW {
            f_capabilities: 0x0001_07B3,
            w_version: 1,
            w_rpcid: 14,
            cb_max_token: STREAM_MAX_MESSAGE,
            name: nw.as_ptr(),
            comment: cw.as_ptr(),
        },
    );
    std::ptr::write_unaligned(pp_package_info, mem as *mut c_void);
    SEC_E_OK
}

fn parse_der_tlv(input: &[u8]) -> Option<(u8, &[u8], &[u8])> {
    if input.len() < 2 {
        return None;
    }
    let tag = input[0];
    let b1 = input[1];
    let (len, hdr) = if b1 < 0x80 {
        (b1 as usize, 2usize)
    } else {
        let n = (b1 & 0x7F) as usize;
        if n == 0 || n > 4 || 2 + n > input.len() {
            return None;
        }
        let mut l = 0usize;
        for i in 0..n {
            l = (l << 8) | input[2 + i] as usize;
        }
        (l, 2 + n)
    };
    if hdr.checked_add(len)? > input.len() {
        return None;
    }
    Some((tag, &input[hdr..hdr + len], &input[hdr + len..]))
}

/// Select the channel-binding hash algorithm for `tls-server-end-point` per RFC 5929 §4.1:
/// use the hash algorithm from the server certificate's outer `signatureAlgorithm`, substituting
/// SHA-256 for MD5 or SHA-1 (and for signature schemes without a single legacy digest).
pub(super) fn rfc5929_endpoint_binding_digest(cert_der: &[u8]) -> crate::misc::bcrypt::Digest {
    use crate::misc::bcrypt::Digest;

    // Certificate ::= SEQUENCE { tbsCertificate, signatureAlgorithm, signatureValue }
    let Some((0x30, cert_body, _)) = parse_der_tlv(cert_der) else {
        return Digest::Sha256;
    };
    // Skip tbsCertificate
    let Some((_, _, after_tbs)) = parse_der_tlv(cert_body) else {
        return Digest::Sha256;
    };
    // signatureAlgorithm ::= SEQUENCE { algorithm OID, parameters ANY OPTIONAL }
    let Some((0x30, alg_body, _)) = parse_der_tlv(after_tbs) else {
        return Digest::Sha256;
    };
    let Some((0x06, alg_oid, params_rest)) = parse_der_tlv(alg_body) else {
        return Digest::Sha256;
    };

    const OID_SHA384_WITH_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0C];
    const OID_SHA512_WITH_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0D];
    const OID_ECDSA_WITH_SHA384: &[u8] = &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x03];
    const OID_ECDSA_WITH_SHA512: &[u8] = &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x04];
    const OID_RSASSA_PSS: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0A];
    const OID_SHA384: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02];
    const OID_SHA512: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03];

    match alg_oid {
        OID_SHA384_WITH_RSA | OID_ECDSA_WITH_SHA384 => Digest::Sha384,
        OID_SHA512_WITH_RSA | OID_ECDSA_WITH_SHA512 => Digest::Sha512,
        OID_RSASSA_PSS => {
            // RSASSA-PSS-params ::= SEQUENCE { [0] hashAlgorithm AlgorithmIdentifier DEFAULT sha1, ... }
            if let Some((0x30, pss_body, _)) = parse_der_tlv(params_rest) {
                if let Some((0xA0, hash_ctx, _)) = parse_der_tlv(pss_body) {
                    if let Some((0x30, hash_alg_seq, _)) = parse_der_tlv(hash_ctx) {
                        if let Some((0x06, hash_oid, _)) = parse_der_tlv(hash_alg_seq) {
                            return match hash_oid {
                                OID_SHA384 => Digest::Sha384,
                                OID_SHA512 => Digest::Sha512,
                                _ => Digest::Sha256,
                            };
                        }
                    }
                }
            }
            Digest::Sha256
        }
        // MD5, SHA-1 (RFC 5929 §4.1 substitute SHA-256), SHA-256, or default
        _ => Digest::Sha256,
    }
}
