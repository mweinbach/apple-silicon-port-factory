//! secur32.dll: Windows SSPI Schannel client-side TLS 1.2 / 1.3 implementation
//! backed by `rustls` (`ring` crypto provider) + embedded `webpki-roots`.
//!
//! Supports the full SSPI lifecycle used by `libcurl`, WinHTTP, and native Windows
//! HTTPS clients:
//! - `InitSecurityInterfaceA/W` (full `SecurityFunctionTableA/W` dispatch table)
//! - `AcquireCredentialsHandleA/W` (`SCHANNEL_CRED` v4 & `SCH_CREDENTIALS` v5)
//! - `InitializeSecurityContextA/W` (TLS 1.2 & TLS 1.3 handshake, SNI, ALPN,
//!   fragmented `SEC_E_INCOMPLETE_MESSAGE`, `SECBUFFER_EXTRA`, `SCHANNEL_SHUTDOWN`)
//! - `EncryptMessage` / `DecryptMessage` (stream framing, TLS 1.3 `NewSessionTicket`,
//!   `SECBUFFER_EXTRA`, `SEC_I_CONTEXT_EXPIRED`)
//! - `QueryContextAttributesA/W` (`STREAM_SIZES`, `REMOTE_CERT_CONTEXT`,
//!   `CONNECTION_INFO`, `APPLICATION_PROTOCOL`, `ISSUER_LIST_EX`, `SIZES`)
//! - `ApplyControlToken`, `CompleteAuthToken`, `QuerySecurityPackageInfoA/W`,
//!   `QueryCredentialsAttributesA/W`, `DeleteSecurityContext`, `FreeCredentialsHandle`,
//!   `FreeContextBuffer`

use super::crypt32::{self, PeerCertVerifyResult};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::client::WebPkiServerVerifier;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, ClientConnection, DigitallySignedStruct, ProtocolVersion, SignatureScheme};
use std::collections::HashMap;
use std::ffi::c_void;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock, Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// SSPI / Schannel constants (exact Windows SDK sspi.h / schannel.h / winerror.h)
// ---------------------------------------------------------------------------

const SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION: u32 = 1;

const SEC_E_OK: i32 = 0;
const SEC_I_CONTINUE_NEEDED: i32 = 0x0009_0312;
const SEC_I_CONTEXT_EXPIRED: i32 = 0x0009_0317;

const SEC_E_INSUFFICIENT_MEMORY: i32 = 0x8009_0300u32 as i32;
const SEC_E_INVALID_HANDLE: i32 = 0x8009_0301u32 as i32;
const SEC_E_UNSUPPORTED_FUNCTION: i32 = 0x8009_0302u32 as i32;
const SEC_E_INTERNAL_ERROR: i32 = 0x8009_0304u32 as i32;
const SEC_E_SECPKG_NOT_FOUND: i32 = 0x8009_0305u32 as i32;
const SEC_E_INVALID_TOKEN: i32 = 0x8009_0308u32 as i32;
const SEC_E_NO_CREDENTIALS: i32 = 0x8009_030Eu32 as i32;
const SEC_E_INCOMPLETE_MESSAGE: i32 = 0x8009_0318u32 as i32;
const SEC_E_CONTEXT_EXPIRED: i32 = 0x8009_0317u32 as i32;
const SEC_E_BUFFER_TOO_SMALL: i32 = 0x8009_0321u32 as i32;
const SEC_E_WRONG_PRINCIPAL: i32 = 0x8009_0322u32 as i32;
const SEC_E_UNTRUSTED_ROOT: i32 = 0x8009_0325u32 as i32;
const SEC_E_ILLEGAL_MESSAGE: i32 = 0x8009_0326u32 as i32;
const SEC_E_CERT_UNKNOWN: i32 = 0x8009_0327u32 as i32;
const SEC_E_CERT_EXPIRED: i32 = 0x8009_0328u32 as i32;
const SEC_E_ENCRYPT_FAILURE: i32 = 0x8009_0329u32 as i32;
const SEC_E_DECRYPT_FAILURE: i32 = 0x8009_0330u32 as i32;
const SEC_E_ALGORITHM_MISMATCH: i32 = 0x8009_0331u32 as i32;

const SECPKG_CRED_OUTBOUND: u32 = 0x0000_0002;

const SCHANNEL_CRED_VERSION: u32 = 4;
const SCH_CREDENTIALS_VERSION: u32 = 5;

const SCH_CRED_NO_SERVERNAME_CHECK: u32 = 0x0000_0004;
const SCH_CRED_MANUAL_CRED_VALIDATION: u32 = 0x0000_0008;

const SP_PROT_TLS1_2_CLIENT: u32 = 0x0000_0800;
const SP_PROT_TLS1_3_CLIENT: u32 = 0x0000_2000;

const ISC_REQ_REPLAY_DETECT: u32 = 0x0000_0004;
const ISC_REQ_SEQUENCE_DETECT: u32 = 0x0000_0008;
const ISC_REQ_CONFIDENTIALITY: u32 = 0x0000_0010;
const ISC_REQ_USE_SUPPLIED_CREDS: u32 = 0x0000_0080;
const ISC_REQ_ALLOCATE_MEMORY: u32 = 0x0000_0100;
const ISC_REQ_EXTENDED_ERROR: u32 = 0x0000_4000;
const ISC_REQ_STREAM: u32 = 0x0000_8000;
const ISC_REQ_MANUAL_CRED_VALIDATION: u32 = 0x0008_0000;

const SECBUFFER_ATTRMASK: u32 = 0xF000_0000;
const SECBUFFER_EMPTY: u32 = 0;
const SECBUFFER_DATA: u32 = 1;
const SECBUFFER_TOKEN: u32 = 2;
const SECBUFFER_MISSING: u32 = 4;
const SECBUFFER_EXTRA: u32 = 5;
const SECBUFFER_STREAM_TRAILER: u32 = 6;
const SECBUFFER_STREAM_HEADER: u32 = 7;
const SECBUFFER_APPLICATION_PROTOCOLS: u32 = 18;

const SECPKG_ATTR_SIZES: u32 = 0;
const SECPKG_ATTR_NAMES: u32 = 1;
const SECPKG_ATTR_STREAM_SIZES: u32 = 4;
const SECPKG_ATTR_PACKAGE_INFO: u32 = 10;
const SECPKG_ATTR_APPLICATION_PROTOCOL: u32 = 0x23;
const SECPKG_ATTR_REMOTE_CERT_CONTEXT: u32 = 0x53;
const SECPKG_ATTR_ISSUER_LIST_EX: u32 = 0x59;
const SECPKG_ATTR_CONNECTION_INFO: u32 = 0x5A;
const SECPKG_ATTR_CIPHER_INFO: u32 = 0x64;

const SECPKG_CRED_ATTR_NAMES: u32 = 1;
const SECPKG_CRED_ATTR_SUPPORTED_PROTOCOLS: u32 = 0x11;

const SECPKGCONTEXT_CIPHERINFO_V1: u32 = 1;
const SZ_ALG_MAX_SIZE: usize = 64;

const SCHANNEL_SHUTDOWN: u32 = 1;
const SCHANNEL_ALERT: u32 = 2;

const STREAM_HEADER_SIZE: u32 = 5;
const STREAM_TRAILER_SIZE: u32 = 64;
const STREAM_MAX_MESSAGE: u32 = 16384;

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
struct SecHandle {
    dw_lower: usize,
    dw_upper: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SecBuffer {
    cb_buffer: u32,
    buffer_type: u32,
    pv_buffer: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SecBufferDesc {
    ul_version: u32,
    c_buffers: u32,
    p_buffers: *mut SecBuffer,
}

#[repr(C)]
struct SecPkgContextStreamSizes {
    cb_header: u32,
    cb_trailer: u32,
    cb_maximum_message: u32,
    c_buffers: u32,
    cb_block_size: u32,
}

#[repr(C)]
struct SecPkgContextSizes {
    cb_max_token: u32,
    cb_max_signature: u32,
    cb_block_size: u32,
    cb_security_trailer: u32,
}

#[repr(C)]
struct SecPkgContextConnectionInfo {
    dw_protocol: u32,
    ai_cipher: u32,
    dw_cipher_strength: u32,
    ai_hash: u32,
    dw_hash_strength: u32,
    ai_exch: u32,
    dw_exch_strength: u32,
}

#[repr(C)]
struct SecPkgContextApplicationProtocol {
    proto_nego_status: u32,
    proto_nego_ext: u32,
    protocol_id_size: u8,
    protocol_id: [u8; 255],
}

#[repr(C)]
struct SecPkgContextIssuerListInfoEx {
    a_issuers: *mut c_void,
    c_issuers: u32,
}

#[repr(C)]
struct SecPkgContextCipherInfo {
    dw_version: u32,
    dw_protocol: u32,
    dw_cipher_suite: u32,
    dw_base_cipher_suite: u32,
    sz_cipher_suite: [u16; SZ_ALG_MAX_SIZE],
    sz_cipher: [u16; SZ_ALG_MAX_SIZE],
    dw_cipher_len: u32,
    dw_cipher_block_len: u32,
    sz_hash: [u16; SZ_ALG_MAX_SIZE],
    dw_hash_len: u32,
    sz_exchange: [u16; SZ_ALG_MAX_SIZE],
    dw_min_exchange_len: u32,
    dw_max_exchange_len: u32,
    sz_certificate: [u16; SZ_ALG_MAX_SIZE],
    dw_key_type: u32,
}

#[repr(C)]
struct SecPkgContextNamesA {
    s_user_name: *mut u8,
}

#[repr(C)]
struct SecPkgContextNamesW {
    s_user_name: *mut u16,
}

#[repr(C)]
struct SecPkgCredSupportedProtocols {
    grbit_protocol: u32,
}

#[repr(C)]
struct SecPkgInfoA {
    f_capabilities: u32,
    w_version: u16,
    w_rpcid: u16,
    cb_max_token: u32,
    name: *const u8,
    comment: *const u8,
}

#[repr(C)]
struct SecPkgInfoW {
    f_capabilities: u32,
    w_version: u16,
    w_rpcid: u16,
    cb_max_token: u32,
    name: *const u16,
    comment: *const u16,
}

// ---------------------------------------------------------------------------
// Internal credential / context tables & rustls ServerCertVerifier bridge
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct SchannelCred {
    principal: String,
    flags: u32,
    enable_tls12: bool,
    enable_tls13: bool,
    extra_roots_der: Vec<Vec<u8>>,
}

#[derive(Default, Debug)]
struct CapturedPeerCert {
    chain_der: Vec<Vec<u8>>,
    verify_error: u32,
}

#[derive(Debug)]
struct SchannelCertVerifier {
    inner: Arc<WebPkiServerVerifier>,
    manual_validation: bool,
    no_servername_check: bool,
    captured: Arc<Mutex<CapturedPeerCert>>,
}

impl ServerCertVerifier for SchannelCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let mut chain = Vec::with_capacity(1 + intermediates.len());
        chain.push(end_entity.as_ref().to_vec());
        for c in intermediates {
            chain.push(c.as_ref().to_vec());
        }

        let res = self
            .inner
            .verify_server_cert(end_entity, intermediates, server_name, ocsp_response, now);

        let verify_error = match &res {
            Ok(_) => 0,
            Err(e) => crypt32::map_rustls_cert_error(e, self.no_servername_check),
        };

        {
            let mut cap = self.captured.lock().unwrap();
            cap.chain_der = chain;
            cap.verify_error = verify_error;
        }

        if self.manual_validation {
            // Caller requested SCH_CRED_MANUAL_CRED_VALIDATION / ISC_REQ_MANUAL_CRED_VALIDATION
            // (standard libcurl Schannel path: caller validates via CertGetCertificateChain +
            // CertVerifyCertificateChainPolicy using the captured chain & recorded WebPKI error).
            return Ok(ServerCertVerified::assertion());
        }

        if verify_error == 0 {
            Ok(ServerCertVerified::assertion())
        } else {
            res
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

struct SchannelContext {
    conn: ClientConnection,
    captured_cert: Arc<Mutex<CapturedPeerCert>>,
    extra_roots_der: Vec<Vec<u8>>,
    target_name: String,
    context_req: u32,
    shutdown_pending: bool,
    shutdown_sent: bool,
    pending_shutdown_bytes: Vec<u8>,
    pending_eof: bool,
    pending_plaintext: Vec<u8>,
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
static CRED_TABLE: LazyLock<Mutex<HashMap<usize, SchannelCred>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static CTXT_TABLE: LazyLock<Mutex<HashMap<usize, SchannelContext>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

fn map_rustls_error_to_sec(err: &rustls::Error) -> i32 {
    match err {
        rustls::Error::InvalidCertificate(cert_err) => match cert_err {
            rustls::CertificateError::Expired
            | rustls::CertificateError::NotValidYet
            | rustls::CertificateError::ExpiredContext { .. }
            | rustls::CertificateError::NotValidYetContext { .. } => SEC_E_CERT_EXPIRED,
            rustls::CertificateError::NotValidForName
            | rustls::CertificateError::NotValidForNameContext { .. } => SEC_E_WRONG_PRINCIPAL,
            rustls::CertificateError::UnknownIssuer
            | rustls::CertificateError::BadSignature
            | rustls::CertificateError::UnknownRevocationStatus => SEC_E_UNTRUSTED_ROOT,
            _ => SEC_E_CERT_UNKNOWN,
        },
        rustls::Error::AlertReceived(alert) => match alert {
            rustls::AlertDescription::HandshakeFailure
            | rustls::AlertDescription::ProtocolVersion
            | rustls::AlertDescription::InsufficientSecurity => SEC_E_ALGORITHM_MISMATCH,
            rustls::AlertDescription::CertificateExpired => SEC_E_CERT_EXPIRED,
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

fn drain_tls_out(conn: &mut ClientConnection) -> Vec<u8> {
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

unsafe fn acquire_credentials_impl(
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
    if (f_credential_use & SECPKG_CRED_OUTBOUND) == 0 {
        stub!("AcquireCredentialsHandle(pkg={package_name:?}, use={f_credential_use:#x}): inbound unsupported");
        return SEC_E_UNSUPPORTED_FUNCTION;
    }

    let mut flags = 0u32;
    let mut enable_tls12 = true;
    let mut enable_tls13 = true;
    let mut extra_roots_der = Vec::new();

    if !p_auth_data.is_null() {
        let base = p_auth_data as *const u8;
        let dw_version = std::ptr::read_unaligned(base as *const u32);
        match dw_version {
            SCHANNEL_CRED_VERSION => {
                // SCHANNEL_CRED (Win64): hRootStore @ +0x10, grbitEnabledProtocols @ +0x38, dwFlags @ +0x48
                let h_root_store = std::ptr::read_unaligned(base.add(0x10) as *const *mut c_void);
                let enabled = std::ptr::read_unaligned(base.add(0x38) as *const u32);
                flags = std::ptr::read_unaligned(base.add(0x48) as *const u32);
                if !h_root_store.is_null() {
                    extra_roots_der = crypt32::store_certs_der(h_root_store);
                }
                if enabled != 0 {
                    let want_12 = (enabled & SP_PROT_TLS1_2_CLIENT) != 0;
                    let want_13 = (enabled & SP_PROT_TLS1_3_CLIENT) != 0;
                    if want_12 || want_13 {
                        enable_tls12 = want_12;
                        enable_tls13 = want_13;
                    }
                }
            }
            SCH_CREDENTIALS_VERSION => {
                // SCH_CREDENTIALS (Win64): hRootStore @ +0x18, dwFlags @ +0x34,
                // cTlsParameters @ +0x38, pTlsParameters @ +0x40
                let h_root_store = std::ptr::read_unaligned(base.add(0x18) as *const *mut c_void);
                flags = std::ptr::read_unaligned(base.add(0x34) as *const u32);
                let c_params = std::ptr::read_unaligned(base.add(0x38) as *const u32);
                let p_params = std::ptr::read_unaligned(base.add(0x40) as *const *const u8);
                if !h_root_store.is_null() {
                    extra_roots_der = crypt32::store_certs_der(h_root_store);
                }
                if c_params > 0 && !p_params.is_null() {
                    // TLS_PARAMETERS.grbitDisabledProtocols @ +0x10
                    let disabled = std::ptr::read_unaligned(p_params.add(0x10) as *const u32);
                    enable_tls12 = (disabled & SP_PROT_TLS1_2_CLIENT) == 0;
                    enable_tls13 = (disabled & SP_PROT_TLS1_3_CLIENT) == 0;
                    if !enable_tls12 && !enable_tls13 {
                        return SEC_E_ALGORITHM_MISMATCH;
                    }
                }
            }
            other => {
                log::warn!("AcquireCredentialsHandle: unknown SCHANNEL auth data version {other}");
            }
        }
    }

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    CRED_TABLE.lock().unwrap().insert(
        id,
        SchannelCred {
            principal,
            flags,
            enable_tls12,
            enable_tls13,
            extra_roots_der,
        },
    );
    write_handle(ph_credential, MAGIC_CRED, id);
    if !pts_expiry.is_null() {
        std::ptr::write_unaligned(pts_expiry, i64::MAX);
    }
    log::debug!(
        "AcquireCredentialsHandle(pkg={package_name:?}, tls12={enable_tls12}, tls13={enable_tls13}, flags={flags:#x}) -> id={id}"
    );
    SEC_E_OK
}

unsafe fn initialize_security_context_impl(
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
        let Some(cred) = CRED_TABLE.lock().unwrap().get(&cred_id).cloned() else {
            return SEC_E_INVALID_HANDLE;
        };
        if ph_new_context.is_null() {
            return SEC_E_INVALID_HANDLE;
        }

        let manual_validation = (cred.flags & SCH_CRED_MANUAL_CRED_VALIDATION) != 0
            || (f_context_req & ISC_REQ_MANUAL_CRED_VALIDATION) != 0;
        let no_servername_check = (cred.flags & SCH_CRED_NO_SERVERNAME_CHECK) != 0;

        let (server_name, enable_sni) = match target_name.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            Some(host) => {
                let clean = host.trim_start_matches('[').trim_end_matches(']');
                match ServerName::try_from(clean.to_string()) {
                    Ok(sn) => {
                        let is_dns = matches!(sn, ServerName::DnsName(_));
                        (sn, is_dns)
                    }
                    Err(_) => {
                        if manual_validation || no_servername_check {
                            (ServerName::try_from("localhost".to_string()).unwrap(), false)
                        } else {
                            return SEC_E_WRONG_PRINCIPAL;
                        }
                    }
                }
            }
            None => (ServerName::try_from("localhost".to_string()).unwrap(), false),
        };

        let mut versions: Vec<&'static rustls::SupportedProtocolVersion> = Vec::with_capacity(2);
        if cred.enable_tls13 {
            versions.push(&rustls::version::TLS13);
        }
        if cred.enable_tls12 {
            versions.push(&rustls::version::TLS12);
        }
        if versions.is_empty() {
            return SEC_E_ALGORITHM_MISMATCH;
        }

        let roots = if cred.extra_roots_der.is_empty() {
            crypt32::default_root_store()
        } else {
            let mut s = (*crypt32::default_root_store()).clone();
            for der in &cred.extra_roots_der {
                let _ = s.add(CertificateDer::from(der.as_slice()));
            }
            Arc::new(s)
        };

        let provider = crypt32::rustls_crypto_provider();
        let inner_verifier = match WebPkiServerVerifier::builder_with_provider(roots, Arc::clone(&provider)).build() {
            Ok(v) => v,
            Err(_) => return SEC_E_INTERNAL_ERROR,
        };
        let captured_cert = Arc::new(Mutex::new(CapturedPeerCert::default()));
        let verifier = Arc::new(SchannelCertVerifier {
            inner: inner_verifier,
            manual_validation,
            no_servername_check,
            captured: Arc::clone(&captured_cert),
        });

        let mut config = match ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(&versions)
        {
            Ok(b) => b
                .dangerous()
                .with_custom_certificate_verifier(verifier)
                .with_no_client_auth(),
            Err(_) => return SEC_E_ALGORITHM_MISMATCH,
        };
        config.enable_sni = enable_sni;
        let alpn = parse_alpn_protocols(p_input);
        if !alpn.is_empty() {
            config.alpn_protocols = alpn;
        }

        let mut conn = match ClientConnection::new(Arc::new(config), server_name) {
            Ok(c) => c,
            Err(ref e) => return map_rustls_error_to_sec(e),
        };

        let client_hello = drain_tls_out(&mut conn);
        if let Err(code) = emit_token_output(p_output, &client_hello, allocate_mem) {
            return code;
        }

        let stored_target = target_name.clone().unwrap_or_default();
        let ctx_id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        CTXT_TABLE.lock().unwrap().insert(
            ctx_id,
            SchannelContext {
                conn,
                captured_cert,
                extra_roots_der: cred.extra_roots_der,
                target_name: stored_target,
                context_req: f_context_req,
                shutdown_pending: false,
                shutdown_sent: false,
                pending_shutdown_bytes: Vec::new(),
                pending_eof: false,
                pending_plaintext: Vec::new(),
            },
        );
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

    let mut table = CTXT_TABLE.lock().unwrap();
    let Some(ctx) = table.get_mut(&ctx_id) else {
        return SEC_E_INVALID_HANDLE;
    };

    if ctx.shutdown_pending {
        if ctx.pending_shutdown_bytes.is_empty() && !ctx.shutdown_sent {
            ctx.conn.send_close_notify();
            ctx.pending_shutdown_bytes = drain_tls_out(&mut ctx.conn);
        }
        let alert_bytes = ctx.pending_shutdown_bytes.clone();
        if let Err(code) = emit_token_output(p_output, &alert_bytes, allocate_mem) {
            return code;
        }
        ctx.shutdown_pending = false;
        ctx.shutdown_sent = true;
        ctx.pending_shutdown_bytes.clear();
        if !pf_context_attr.is_null() {
            std::ptr::write_unaligned(pf_context_attr, ret_attr);
        }
        log::debug!("InitializeSecurityContext(ctx={ctx_id}, shutdown) -> close_notify={}B", alert_bytes.len());
        return SEC_E_OK;
    }

    // Find SECBUFFER_TOKEN in p_input.
    let mut token_ptr: *const u8 = std::ptr::null();
    let mut token_len: usize = 0;
    if !p_input.is_null() {
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

    if token_ptr.is_null() || token_len == 0 {
        let out = drain_tls_out(&mut ctx.conn);
        if !out.is_empty() {
            if let Err(code) = emit_token_output(p_output, &out, allocate_mem) {
                return code;
            }
            if !pf_context_attr.is_null() {
                std::ptr::write_unaligned(pf_context_attr, ret_attr);
            }
            return if ctx.conn.is_handshaking() {
                SEC_I_CONTINUE_NEEDED
            } else {
                SEC_E_OK
            };
        }
        let _ = emit_token_output(p_output, &[], allocate_mem);
        set_input_aux_buffer(p_input, SECBUFFER_MISSING, std::ptr::null_mut(), 5);
        return SEC_E_INCOMPLETE_MESSAGE;
    }

    let input = std::slice::from_raw_parts(token_ptr, token_len);
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
                let mut rec = &input[consumed..consumed + rec_total];
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
                            let _ = emit_token_output(p_output, &alert, allocate_mem);
                        }
                    }
                    log::debug!("InitializeSecurityContext(ctx={ctx_id}) handshake error: {e} -> {sec_err:#010x}");
                    return sec_err;
                }
            }
        }
    }

    if consumed == 0 && ctx.conn.is_handshaking() && !ctx.conn.wants_write() {
        let _ = emit_token_output(p_output, &[], allocate_mem);
        set_input_aux_buffer(
            p_input,
            SECBUFFER_MISSING,
            std::ptr::null_mut(),
            missing_bytes.max(1) as u32,
        );
        return SEC_E_INCOMPLETE_MESSAGE;
    }

    let out_bytes = drain_tls_out(&mut ctx.conn);
    if let Err(code) = emit_token_output(p_output, &out_bytes, allocate_mem) {
        return code;
    }

    if consumed < input.len() {
        let extra = input.len() - consumed;
        set_input_aux_buffer(
            p_input,
            SECBUFFER_EXTRA,
            token_ptr.add(consumed) as *mut c_void,
            extra as u32,
        );
    }

    if !pf_context_attr.is_null() {
        std::ptr::write_unaligned(pf_context_attr, ret_attr);
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
}

// ---------------------------------------------------------------------------
// Exported Win64 SSPI APIs
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn AcquireCredentialsHandleA(
    psz_principal: *const u8,
    psz_package: *const u8,
    f_credential_use: u32,
    _pv_logon_id: *mut c_void,
    p_auth_data: *mut c_void,
    _p_get_key_fn: *mut c_void,
    _pv_get_key_argument: *mut c_void,
    ph_credential: *mut c_void,
    pts_expiry: *mut i64,
) -> i32 {
    if psz_package.is_null() {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    let principal = if psz_principal.is_null() {
        String::new()
    } else {
        crate::strings::from_ansi(psz_principal)
    };
    let pkg = crate::strings::from_ansi(psz_package);
    acquire_credentials_impl(
        principal,
        &pkg,
        f_credential_use,
        p_auth_data,
        ph_credential as *mut SecHandle,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn AcquireCredentialsHandleW(
    psz_principal: *const u16,
    psz_package: *const u16,
    f_credential_use: u32,
    _pv_logon_id: *mut c_void,
    p_auth_data: *mut c_void,
    _p_get_key_fn: *mut c_void,
    _pv_get_key_argument: *mut c_void,
    ph_credential: *mut c_void,
    pts_expiry: *mut i64,
) -> i32 {
    if psz_package.is_null() {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    let principal = if psz_principal.is_null() {
        String::new()
    } else {
        crate::strings::from_wide(psz_principal)
    };
    let pkg = crate::strings::from_wide(psz_package);
    acquire_credentials_impl(
        principal,
        &pkg,
        f_credential_use,
        p_auth_data,
        ph_credential as *mut SecHandle,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn FreeCredentialsHandle(ph_credential: *mut c_void) -> i32 {
    let Some(id) = read_handle(ph_credential as *const SecHandle, MAGIC_CRED) else {
        return SEC_E_INVALID_HANDLE;
    };
    if CRED_TABLE.lock().unwrap().remove(&id).is_some() {
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn InitializeSecurityContextA(
    ph_credential: *mut c_void,
    ph_context: *mut c_void,
    psz_target_name: *const u8,
    f_context_req: u32,
    _reserved1: u32,
    _target_data_rep: u32,
    p_input: *mut c_void,
    _reserved2: u32,
    ph_new_context: *mut c_void,
    p_output: *mut c_void,
    pf_context_attr: *mut u32,
    pts_expiry: *mut i64,
) -> i32 {
    let target = if psz_target_name.is_null() {
        None
    } else {
        Some(crate::strings::from_ansi(psz_target_name))
    };
    initialize_security_context_impl(
        ph_credential as *mut SecHandle,
        ph_context as *mut SecHandle,
        target,
        f_context_req,
        p_input as *mut SecBufferDesc,
        ph_new_context as *mut SecHandle,
        p_output as *mut SecBufferDesc,
        pf_context_attr,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn InitializeSecurityContextW(
    ph_credential: *mut c_void,
    ph_context: *mut c_void,
    psz_target_name: *const u16,
    f_context_req: u32,
    _reserved1: u32,
    _target_data_rep: u32,
    p_input: *mut c_void,
    _reserved2: u32,
    ph_new_context: *mut c_void,
    p_output: *mut c_void,
    pf_context_attr: *mut u32,
    pts_expiry: *mut i64,
) -> i32 {
    let target = if psz_target_name.is_null() {
        None
    } else {
        Some(crate::strings::from_wide(psz_target_name))
    };
    initialize_security_context_impl(
        ph_credential as *mut SecHandle,
        ph_context as *mut SecHandle,
        target,
        f_context_req,
        p_input as *mut SecBufferDesc,
        ph_new_context as *mut SecHandle,
        p_output as *mut SecBufferDesc,
        pf_context_attr,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn CompleteAuthToken(
    ph_context: *mut c_void,
    _p_token: *mut c_void,
) -> i32 {
    let Some(id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if CTXT_TABLE.lock().unwrap().contains_key(&id) {
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn DeleteSecurityContext(ph_context: *mut c_void) -> i32 {
    let Some(id) = read_handle(ph_context as *const SecHandle, MAGIC_CTXT) else {
        return SEC_E_INVALID_HANDLE;
    };
    if CTXT_TABLE.lock().unwrap().remove(&id).is_some() {
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn ApplyControlToken(
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
            if tv == SCHANNEL_ALERT && sb.cb_buffer >= 12 {
                // SCHANNEL_ALERT_TOKEN { dwTokenType(0), dwAlertType(4), dwAlertNumber(8) }
                alert_num = Some(std::ptr::read_unaligned((sb.pv_buffer as *const u8).add(8) as *const u32));
            }
            break;
        }
    }
    let is_close_notify = token_val == Some(SCHANNEL_SHUTDOWN)
        || (token_val == Some(SCHANNEL_ALERT) && alert_num == Some(0));
    if !is_close_notify {
        stub!("ApplyControlToken(ctx={id}, token={token_val:?}) -> SEC_E_UNSUPPORTED_FUNCTION");
        return SEC_E_UNSUPPORTED_FUNCTION;
    }
    let mut table = CTXT_TABLE.lock().unwrap();
    let Some(ctx) = table.get_mut(&id) else {
        return SEC_E_INVALID_HANDLE;
    };
    if !ctx.shutdown_sent && !ctx.shutdown_pending {
        ctx.conn.send_close_notify();
        ctx.pending_shutdown_bytes = drain_tls_out(&mut ctx.conn);
        ctx.shutdown_pending = true;
    }
    SEC_E_OK
}

pub unsafe extern "win64" fn FreeContextBuffer(pv_context_buffer: *mut c_void) -> i32 {
    if pv_context_buffer.is_null() {
        return SEC_E_OK;
    }
    if crate::heap::is_ours(pv_context_buffer) {
        crate::heap::free(pv_context_buffer);
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn EncryptMessage(
    ph_context: *mut c_void,
    _f_qop: u32,
    p_message: *mut c_void,
    _message_seq_no: u32,
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

    if hdr.pv_buffer.is_null() || trl.pv_buffer.is_null() || (data.cb_buffer > 0 && data.pv_buffer.is_null()) {
        return SEC_E_INVALID_TOKEN;
    }

    // Copy plaintext BEFORE modifying any output buffer (they are often contiguous in memory).
    let plaintext = if data.cb_buffer > 0 {
        std::slice::from_raw_parts(data.pv_buffer as *const u8, data.cb_buffer as usize).to_vec()
    } else {
        Vec::new()
    };

    let mut table = CTXT_TABLE.lock().unwrap();
    let Some(ctx) = table.get_mut(&ctx_id) else {
        return SEC_E_INVALID_HANDLE;
    };
    if ctx.shutdown_pending || ctx.shutdown_sent {
        return SEC_E_CONTEXT_EXPIRED;
    }
    if ctx.conn.is_handshaking() {
        return SEC_E_ENCRYPT_FAILURE;
    }

    if ctx.conn.writer().write_all(&plaintext).is_err() || ctx.conn.writer().flush().is_err() {
        return SEC_E_ENCRYPT_FAILURE;
    }
    let ct = drain_tls_out(&mut ctx.conn);
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

pub unsafe extern "win64" fn DecryptMessage(
    ph_context: *mut c_void,
    p_message: *mut c_void,
    _message_seq_no: u32,
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
        set_input_aux_buffer(p_message as *mut SecBufferDesc, SECBUFFER_MISSING, std::ptr::null_mut(), 5);
        return SEC_E_INCOMPLETE_MESSAGE;
    }

    let base_ptr = db.pv_buffer as *mut u8;
    let total_len = db.cb_buffer as usize;

    let mut table = CTXT_TABLE.lock().unwrap();
    let Some(ctx) = table.get_mut(&ctx_id) else {
        return SEC_E_INVALID_HANDLE;
    };

    if ctx.pending_eof && ctx.pending_plaintext.is_empty() {
        ctx.pending_eof = false;
        write_decrypt_buffers(desc, base_ptr, 0, 0, total_len);
        return SEC_I_CONTEXT_EXPIRED;
    }

    let input = std::slice::from_raw_parts(base_ptr as *const u8, total_len);
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
        set_input_aux_buffer(
            p_message as *mut SecBufferDesc,
            SECBUFFER_MISSING,
            std::ptr::null_mut(),
            missing_bytes.max(1) as u32,
        );
        return SEC_E_INCOMPLETE_MESSAGE;
    }

    if !pf_qop.is_null() {
        std::ptr::write_unaligned(pf_qop, 0u32);
    }

    if plaintext.is_empty() && saw_eof {
        write_decrypt_buffers(desc, base_ptr, 0, consumed, total_len);
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
    if copy_len > 0 {
        std::ptr::copy_nonoverlapping(plaintext.as_ptr(), base_ptr, copy_len);
    }

    write_decrypt_buffers(desc, base_ptr, copy_len, consumed, total_len);
    SEC_E_OK
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
        let units: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
        let mem = crate::heap::alloc(units.len() * 2, false) as *mut u16;
        if mem.is_null() {
            return std::ptr::null_mut();
        }
        for (i, &u) in units.iter().enumerate() {
            std::ptr::write_unaligned(mem.add(i), u);
        }
        mem as *mut c_void
    } else {
        let bytes = s.as_bytes();
        let mem = crate::heap::alloc(bytes.len() + 1, false) as *mut u8;
        if mem.is_null() {
            return std::ptr::null_mut();
        }
        if !bytes.is_empty() {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), mem, bytes.len());
        }
        std::ptr::write_unaligned(mem.add(bytes.len()), 0);
        mem as *mut c_void
    }
}

fn kx_strength(conn: &ClientConnection) -> u32 {
    match conn.negotiated_key_exchange_group().map(|g| g.name()) {
        Some(rustls::NamedGroup::secp384r1) => 384,
        Some(rustls::NamedGroup::secp521r1) => 521,
        _ => 256,
    }
}

fn cipher_suite_metadata(
    suite: Option<rustls::CipherSuite>,
) -> (u32, &'static str, &'static str, u32, u32, &'static str, u32, &'static str, &'static str) {
    // Returns: (iana_code, suite_name, cipher_name, cipher_bits, block_bytes, hash_name, hash_bits, exch_name, cert_name)
    match suite {
        Some(rustls::CipherSuite::TLS13_AES_256_GCM_SHA384) => (
            0x1302, "TLS_AES_256_GCM_SHA384", "AES", 256, 16, "SHA384", 384, "ECDHE", "RSA",
        ),
        Some(rustls::CipherSuite::TLS13_AES_128_GCM_SHA256) => (
            0x1301, "TLS_AES_128_GCM_SHA256", "AES", 128, 16, "SHA256", 256, "ECDHE", "RSA",
        ),
        Some(rustls::CipherSuite::TLS13_CHACHA20_POLY1305_SHA256) => (
            0x1303, "TLS_CHACHA20_POLY1305_SHA256", "CHACHA20", 256, 1, "SHA256", 256, "ECDHE", "RSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384) => (
            0xC02C, "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384", "AES", 256, 16, "SHA384", 384, "ECDHE", "ECDSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256) => (
            0xC02B, "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256", "AES", 128, 16, "SHA256", 256, "ECDHE", "ECDSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256) => (
            0xCCA9, "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256", "CHACHA20", 256, 1, "SHA256", 256, "ECDHE", "ECDSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384) => (
            0xC030, "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384", "AES", 256, 16, "SHA384", 384, "ECDHE", "RSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256) => (
            0xC02F, "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256", "AES", 128, 16, "SHA256", 256, "ECDHE", "RSA",
        ),
        Some(rustls::CipherSuite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256) => (
            0xCCA8, "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256", "CHACHA20", 256, 1, "SHA256", 256, "ECDHE", "RSA",
        ),
        Some(other) => (
            u16::from(other) as u32, "TLS_AES_128_GCM_SHA256", "AES", 128, 16, "SHA256", 256, "ECDHE", "RSA",
        ),
        None => (
            0, "", "", 0, 0, "", 0, "", "",
        ),
    }
}

unsafe fn query_context_attributes_impl(
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
    let table = CTXT_TABLE.lock().unwrap();
    let Some(ctx) = table.get(&ctx_id) else {
        return SEC_E_INVALID_HANDLE;
    };

    match ul_attribute {
        SECPKG_ATTR_SIZES => {
            std::ptr::write_unaligned(
                p_buffer as *mut SecPkgContextSizes,
                SecPkgContextSizes {
                    cb_max_token: STREAM_MAX_MESSAGE,
                    cb_max_signature: 0,
                    cb_block_size: 1,
                    cb_security_trailer: STREAM_TRAILER_SIZE,
                },
            );
            SEC_E_OK
        }
        SECPKG_ATTR_NAMES => {
            let name = if !ctx.target_name.is_empty() {
                ctx.target_name.clone()
            } else {
                "Schannel".to_string()
            };
            let s_ptr = alloc_sec_string(&name, wide);
            if s_ptr.is_null() {
                return SEC_E_INSUFFICIENT_MEMORY;
            }
            if wide {
                std::ptr::write_unaligned(
                    p_buffer as *mut SecPkgContextNamesW,
                    SecPkgContextNamesW { s_user_name: s_ptr as *mut u16 },
                );
            } else {
                std::ptr::write_unaligned(
                    p_buffer as *mut SecPkgContextNamesA,
                    SecPkgContextNamesA { s_user_name: s_ptr as *mut u8 },
                );
            }
            SEC_E_OK
        }
        SECPKG_ATTR_STREAM_SIZES => {
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
            SEC_E_OK
        }
        SECPKG_ATTR_PACKAGE_INFO => {
            let mut pkg_ptr: *mut c_void = std::ptr::null_mut();
            let rc = if wide {
                let w: Vec<u16> = "Schannel\0".encode_utf16().collect();
                QuerySecurityPackageInfoW(w.as_ptr(), &mut pkg_ptr)
            } else {
                QuerySecurityPackageInfoA(b"Schannel\0".as_ptr(), &mut pkg_ptr)
            };
            if rc != SEC_E_OK {
                return rc;
            }
            std::ptr::write_unaligned(p_buffer as *mut *mut c_void, pkg_ptr);
            SEC_E_OK
        }
        SECPKG_ATTR_REMOTE_CERT_CONTEXT => {
            let cap = ctx.captured_cert.lock().unwrap();
            if cap.chain_der.is_empty() {
                return SEC_E_NO_CREDENTIALS;
            }
            let cert_ptr = crypt32::register_peer_cert_chain(
                &cap.chain_der,
                PeerCertVerifyResult {
                    cert_error: cap.verify_error,
                    extra_roots_der: ctx.extra_roots_der.clone(),
                },
            );
            if cert_ptr.is_null() {
                return SEC_E_INTERNAL_ERROR;
            }
            std::ptr::write_unaligned(p_buffer as *mut *mut c_void, cert_ptr);
            SEC_E_OK
        }
        SECPKG_ATTR_CONNECTION_INFO => {
            let dw_protocol = match ctx.conn.protocol_version() {
                Some(ProtocolVersion::TLSv1_3) => SP_PROT_TLS1_3_CLIENT,
                Some(ProtocolVersion::TLSv1_2) => SP_PROT_TLS1_2_CLIENT,
                _ => SP_PROT_TLS1_2_CLIENT,
            };
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
            SEC_E_OK
        }
        SECPKG_ATTR_CIPHER_INFO => {
            let dw_protocol = match ctx.conn.protocol_version() {
                Some(ProtocolVersion::TLSv1_3) => SP_PROT_TLS1_3_CLIENT,
                Some(ProtocolVersion::TLSv1_2) => SP_PROT_TLS1_2_CLIENT,
                _ => SP_PROT_TLS1_2_CLIENT,
            };
            let suite = ctx.conn.negotiated_cipher_suite().map(|s| s.suite());
            let (code, suite_name, cipher_name, cipher_len, block_len, hash_name, hash_len, exch_name, cert_name) =
                cipher_suite_metadata(suite);
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
            std::ptr::write_unaligned(p_buffer as *mut SecPkgContextCipherInfo, info);
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
            std::ptr::write_unaligned(p_buffer as *mut SecPkgContextApplicationProtocol, info);
            SEC_E_OK
        }
        SECPKG_ATTR_ISSUER_LIST_EX => {
            std::ptr::write_unaligned(
                p_buffer as *mut SecPkgContextIssuerListInfoEx,
                SecPkgContextIssuerListInfoEx {
                    a_issuers: std::ptr::null_mut(),
                    c_issuers: 0,
                },
            );
            SEC_E_OK
        }
        other => {
            stub!("QueryContextAttributes(ctx={ctx_id}, attr={other:#x}) -> SEC_E_UNSUPPORTED_FUNCTION");
            SEC_E_UNSUPPORTED_FUNCTION
        }
    }
}

pub unsafe extern "win64" fn QueryContextAttributesA(
    ph_context: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    query_context_attributes_impl(ph_context, ul_attribute, p_buffer, false)
}

pub unsafe extern "win64" fn QueryContextAttributesW(
    ph_context: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    query_context_attributes_impl(ph_context, ul_attribute, p_buffer, true)
}

unsafe fn query_credentials_attributes_impl(
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
    let Some(cred) = CRED_TABLE.lock().unwrap().get(&cred_id).cloned() else {
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
                    SecPkgContextNamesW { s_user_name: s_ptr as *mut u16 },
                );
            } else {
                std::ptr::write_unaligned(
                    p_buffer as *mut SecPkgContextNamesA,
                    SecPkgContextNamesA { s_user_name: s_ptr as *mut u8 },
                );
            }
            SEC_E_OK
        }
        SECPKG_CRED_ATTR_SUPPORTED_PROTOCOLS => {
            let mut prot = 0u32;
            if cred.enable_tls12 {
                prot |= SP_PROT_TLS1_2_CLIENT;
            }
            if cred.enable_tls13 {
                prot |= SP_PROT_TLS1_3_CLIENT;
            }
            std::ptr::write_unaligned(
                p_buffer as *mut SecPkgCredSupportedProtocols,
                SecPkgCredSupportedProtocols { grbit_protocol: prot },
            );
            SEC_E_OK
        }
        _ => {
            stub!("QueryCredentialsAttributes{}(attr={ul_attribute:#x}) -> SEC_E_UNSUPPORTED_FUNCTION", if wide { "W" } else { "A" });
            SEC_E_UNSUPPORTED_FUNCTION
        }
    }
}

pub unsafe extern "win64" fn QueryCredentialsAttributesA(
    ph_credential: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    query_credentials_attributes_impl(ph_credential, ul_attribute, p_buffer, false)
}

pub unsafe extern "win64" fn QueryCredentialsAttributesW(
    ph_credential: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    query_credentials_attributes_impl(ph_credential, ul_attribute, p_buffer, true)
}

pub unsafe extern "win64" fn QuerySecurityPackageInfoA(
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

pub unsafe extern "win64" fn QuerySecurityPackageInfoW(
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

unsafe extern "win64" fn sspi_unsupported() -> i32 {
    SEC_E_UNSUPPORTED_FUNCTION
}

#[repr(C)]
struct SecurityFunctionTable {
    dw_version: u32,
    slots: [usize; 27],
}

unsafe fn build_table(wide: bool) -> *mut c_void {
    let unsup = sspi_unsupported as *const () as usize;
    let query_cred = if wide {
        QueryCredentialsAttributesW as *const () as usize
    } else {
        QueryCredentialsAttributesA as *const () as usize
    };
    let acq = if wide {
        AcquireCredentialsHandleW as *const () as usize
    } else {
        AcquireCredentialsHandleA as *const () as usize
    };
    let init_ctx = if wide {
        InitializeSecurityContextW as *const () as usize
    } else {
        InitializeSecurityContextA as *const () as usize
    };
    let query_ctx = if wide {
        QueryContextAttributesW as *const () as usize
    } else {
        QueryContextAttributesA as *const () as usize
    };
    let query_pkg = if wide {
        QuerySecurityPackageInfoW as *const () as usize
    } else {
        QuerySecurityPackageInfoA as *const () as usize
    };
    let free_cred = FreeCredentialsHandle as *const () as usize;
    let complete = CompleteAuthToken as *const () as usize;
    let del_ctx = DeleteSecurityContext as *const () as usize;
    let apply_ctl = ApplyControlToken as *const () as usize;
    let free_buf = FreeContextBuffer as *const () as usize;
    let enc = EncryptMessage as *const () as usize;
    let dec = DecryptMessage as *const () as usize;

    for f in [
        unsup, query_cred, acq, init_ctx, query_ctx, query_pkg, free_cred, complete, del_ctx,
        apply_ctl, free_buf, enc, dec,
    ] {
        crate::register_native_va(f as u64, f);
    }

    let mut slots = [unsup; 27];
    slots[1] = query_cred; // QueryCredentialsAttributes
    slots[2] = acq;        // AcquireCredentialsHandle
    slots[3] = free_cred;  // FreeCredentialsHandle
    slots[4] = 0;          // Reserved2
    slots[5] = init_ctx;   // InitializeSecurityContext
    slots[7] = complete;   // CompleteAuthToken
    slots[8] = del_ctx;    // DeleteSecurityContext
    slots[9] = apply_ctl;  // ApplyControlToken
    slots[10] = query_ctx; // QueryContextAttributes
    slots[15] = free_buf;  // FreeContextBuffer
    slots[16] = query_pkg; // QuerySecurityPackageInfo
    slots[17] = 0;         // Reserved3
    slots[18] = 0;         // Reserved4
    slots[22] = 0;         // Reserved8
    slots[24] = enc;       // EncryptMessage
    slots[25] = dec;       // DecryptMessage

    Box::into_raw(Box::new(SecurityFunctionTable {
        dw_version: SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION,
        slots,
    })) as *mut c_void
}

pub unsafe extern "win64" fn InitSecurityInterfaceA() -> *mut c_void {
    static TBL: OnceLock<usize> = OnceLock::new();
    *TBL.get_or_init(|| build_table(false) as usize) as *mut c_void
}

pub unsafe extern "win64" fn InitSecurityInterfaceW() -> *mut c_void {
    static TBL: OnceLock<usize> = OnceLock::new();
    *TBL.get_or_init(|| build_table(true) as usize) as *mut c_void
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::wintrust::CertContext;
    use rustls::pki_types::PrivateKeyDer;
    use rustls::{ServerConfig, ServerConnection};

    // Deterministic Ed25519 Test CA + Leaf cert (CN=test.example.com, SAN=DNS:test.example.com, IP:127.0.0.1,
    // valid 2020-01-01 to 2095-01-01) for offline loopback Schannel <-> rustls::ServerConnection tests.
    const TEST_CA_DER: &[u8] = &[
        0x30, 0x82, 0x01, 0x2F, 0x30, 0x81, 0xE2, 0xA0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x02, 0x10, 0x00,
        0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70, 0x30, 0x1E, 0x31, 0x1C, 0x30, 0x1A, 0x06, 0x03, 0x55,
        0x04, 0x03, 0x0C, 0x13, 0x50, 0x6F, 0x72, 0x74, 0x46, 0x61, 0x63, 0x74, 0x6F, 0x72, 0x79, 0x20,
        0x54, 0x65, 0x73, 0x74, 0x20, 0x43, 0x41, 0x30, 0x20, 0x17, 0x0D, 0x32, 0x30, 0x30, 0x31, 0x30,
        0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x18, 0x0F, 0x32, 0x30, 0x39, 0x35, 0x30, 0x31,
        0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x30, 0x1E, 0x31, 0x1C, 0x30, 0x1A, 0x06,
        0x03, 0x55, 0x04, 0x03, 0x0C, 0x13, 0x50, 0x6F, 0x72, 0x74, 0x46, 0x61, 0x63, 0x74, 0x6F, 0x72,
        0x79, 0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x43, 0x41, 0x30, 0x2A, 0x30, 0x05, 0x06, 0x03, 0x2B,
        0x65, 0x70, 0x03, 0x21, 0x00, 0xC9, 0xDF, 0xC5, 0xF9, 0x98, 0x74, 0x8D, 0xAC, 0x1F, 0xD1, 0x1A,
        0x1E, 0xEB, 0x8B, 0x3D, 0x7C, 0xB7, 0x3E, 0x2D, 0x94, 0xCC, 0x7B, 0x36, 0xD6, 0xC5, 0x3F, 0x5E,
        0xD3, 0x6D, 0x62, 0x5B, 0x03, 0xA3, 0x42, 0x30, 0x40, 0x30, 0x0F, 0x06, 0x03, 0x55, 0x1D, 0x13,
        0x01, 0x01, 0xFF, 0x04, 0x05, 0x30, 0x03, 0x01, 0x01, 0xFF, 0x30, 0x0E, 0x06, 0x03, 0x55, 0x1D,
        0x0F, 0x01, 0x01, 0xFF, 0x04, 0x04, 0x03, 0x02, 0x01, 0x06, 0x30, 0x1D, 0x06, 0x03, 0x55, 0x1D,
        0x0E, 0x04, 0x16, 0x04, 0x14, 0xDE, 0x3C, 0xDD, 0x00, 0xB6, 0xC3, 0xE0, 0x57, 0x91, 0xA7, 0xBA,
        0x51, 0x58, 0xFA, 0xD7, 0xEC, 0x4B, 0x22, 0x85, 0x9C, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70,
        0x03, 0x41, 0x00, 0x30, 0x82, 0x52, 0xB8, 0x65, 0x95, 0x78, 0x8F, 0x16, 0xCC, 0x90, 0x9B, 0x53,
        0xAD, 0x3F, 0x06, 0xA0, 0xB8, 0xD3, 0xC3, 0xF1, 0x88, 0xBC, 0xAB, 0xC1, 0xE1, 0x7B, 0xC3, 0x49,
        0x45, 0xBD, 0x18, 0x87, 0x68, 0x69, 0x53, 0x74, 0x9A, 0xB1, 0xA1, 0xF3, 0x3E, 0xBC, 0x1E, 0x29,
        0xFA, 0x49, 0x6D, 0x5E, 0xCD, 0x19, 0x92, 0xB6, 0x4E, 0xE5, 0xEE, 0x68, 0x9C, 0xB4, 0x5F, 0x9E,
        0x0F, 0x5B, 0x04,
    ];

    const TEST_LEAF_DER: &[u8] = &[
        0x30, 0x82, 0x01, 0x82, 0x30, 0x82, 0x01, 0x34, 0xA0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x02, 0x10,
        0x01, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70, 0x30, 0x1E, 0x31, 0x1C, 0x30, 0x1A, 0x06, 0x03,
        0x55, 0x04, 0x03, 0x0C, 0x13, 0x50, 0x6F, 0x72, 0x74, 0x46, 0x61, 0x63, 0x74, 0x6F, 0x72, 0x79,
        0x20, 0x54, 0x65, 0x73, 0x74, 0x20, 0x43, 0x41, 0x30, 0x20, 0x17, 0x0D, 0x32, 0x30, 0x30, 0x31,
        0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x18, 0x0F, 0x32, 0x30, 0x39, 0x35, 0x30,
        0x31, 0x30, 0x31, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5A, 0x30, 0x1B, 0x31, 0x19, 0x30, 0x17,
        0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x10, 0x74, 0x65, 0x73, 0x74, 0x2E, 0x65, 0x78, 0x61, 0x6D,
        0x70, 0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x30, 0x2A, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70,
        0x03, 0x21, 0x00, 0x64, 0x12, 0x69, 0x0A, 0x76, 0x96, 0x67, 0x9F, 0x8D, 0x7E, 0x53, 0xCE, 0x59,
        0xA0, 0x95, 0x62, 0x95, 0xED, 0xFE, 0xBF, 0xBA, 0xD1, 0xB8, 0xCE, 0x98, 0x10, 0x3C, 0xE7, 0xFE,
        0x3D, 0x15, 0x04, 0xA3, 0x81, 0x96, 0x30, 0x81, 0x93, 0x30, 0x09, 0x06, 0x03, 0x55, 0x1D, 0x13,
        0x04, 0x02, 0x30, 0x00, 0x30, 0x0E, 0x06, 0x03, 0x55, 0x1D, 0x0F, 0x01, 0x01, 0xFF, 0x04, 0x04,
        0x03, 0x02, 0x07, 0x80, 0x30, 0x13, 0x06, 0x03, 0x55, 0x1D, 0x25, 0x04, 0x0C, 0x30, 0x0A, 0x06,
        0x08, 0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x01, 0x30, 0x21, 0x06, 0x03, 0x55, 0x1D, 0x11,
        0x04, 0x1A, 0x30, 0x18, 0x82, 0x10, 0x74, 0x65, 0x73, 0x74, 0x2E, 0x65, 0x78, 0x61, 0x6D, 0x70,
        0x6C, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x87, 0x04, 0x7F, 0x00, 0x00, 0x01, 0x30, 0x1D, 0x06, 0x03,
        0x55, 0x1D, 0x0E, 0x04, 0x16, 0x04, 0x14, 0x13, 0xAF, 0xD3, 0x84, 0xCA, 0xC2, 0x1E, 0xC9, 0x60,
        0x11, 0xE3, 0x10, 0x7F, 0x67, 0x39, 0x27, 0xB5, 0xCF, 0x48, 0x10, 0x30, 0x1F, 0x06, 0x03, 0x55,
        0x1D, 0x23, 0x04, 0x18, 0x30, 0x16, 0x80, 0x14, 0xDE, 0x3C, 0xDD, 0x00, 0xB6, 0xC3, 0xE0, 0x57,
        0x91, 0xA7, 0xBA, 0x51, 0x58, 0xFA, 0xD7, 0xEC, 0x4B, 0x22, 0x85, 0x9C, 0x30, 0x05, 0x06, 0x03,
        0x2B, 0x65, 0x70, 0x03, 0x41, 0x00, 0x6E, 0x85, 0x65, 0xC4, 0x03, 0x91, 0xBC, 0x68, 0x8A, 0x98,
        0xFD, 0xC1, 0xC0, 0x3F, 0x33, 0xA2, 0xEF, 0x34, 0x3F, 0x39, 0xB7, 0x15, 0x6B, 0xFF, 0xD3, 0xEB,
        0xC9, 0xB4, 0xE1, 0x7F, 0xD7, 0x3D, 0x21, 0xC1, 0x27, 0x5C, 0xE9, 0xE2, 0xD5, 0x0C, 0xED, 0xAF,
        0x45, 0x39, 0x22, 0x06, 0x8C, 0x32, 0x4C, 0x68, 0xFD, 0xFB, 0xF5, 0x62, 0xA9, 0x6F, 0x73, 0x38,
        0xBA, 0x01, 0x47, 0x14, 0x74, 0x03,
    ];

    const TEST_LEAF_KEY_PKCS8_DER: &[u8] = &[
        0x30, 0x2E, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2B, 0x65, 0x70, 0x04, 0x22, 0x04, 0x20,
        0xA6, 0xD8, 0x4E, 0xD4, 0x73, 0x80, 0x73, 0x52, 0xB6, 0x85, 0x95, 0x39, 0xA7, 0x4E, 0x3C, 0x72,
        0x3B, 0x17, 0xDC, 0xB7, 0x50, 0x23, 0xA5, 0x51, 0x8A, 0x09, 0x95, 0x99, 0xEB, 0xE0, 0x91, 0x9A,
    ];

    fn make_test_server(versions: &[&'static rustls::SupportedProtocolVersion], alpn: Vec<Vec<u8>>) -> ServerConnection {
        let chain = vec![CertificateDer::from(TEST_LEAF_DER.to_vec())];
        let key = PrivateKeyDer::Pkcs8(TEST_LEAF_KEY_PKCS8_DER.to_vec().into());
        let mut cfg = ServerConfig::builder_with_provider(crypt32::rustls_crypto_provider())
            .with_protocol_versions(versions)
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(chain, key)
            .unwrap();
        cfg.alpn_protocols = alpn;
        ServerConnection::new(Arc::new(cfg)).unwrap()
    }

    fn server_step(srv: &mut ServerConnection, incoming: &[u8]) -> Vec<u8> {
        let mut slice = incoming;
        while !slice.is_empty() {
            let n = srv.read_tls(&mut slice).unwrap();
            if n == 0 {
                break;
            }
            srv.process_new_packets().unwrap();
        }
        let mut out = Vec::new();
        while srv.wants_write() {
            if srv.write_tls(&mut out).unwrap() == 0 {
                break;
            }
        }
        out
    }

    unsafe fn make_ca_memory_store() -> *mut c_void {
        let store = crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
        assert!(!store.is_null());
        assert_eq!(
            crypt32::CertAddEncodedCertificateToStore(
                store,
                1,
                TEST_CA_DER.as_ptr(),
                TEST_CA_DER.len() as u32,
                4,
                std::ptr::null_mut(),
            ),
            1
        );
        store
    }

    #[test]
    fn init_security_interface_returns_valid_function_tables() {
        unsafe {
            let ta = InitSecurityInterfaceA() as *const SecurityFunctionTable;
            let tw = InitSecurityInterfaceW() as *const SecurityFunctionTable;
            assert!(!ta.is_null());
            assert!(!tw.is_null());
            assert_eq!((*ta).dw_version, SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION);
            assert_eq!((*tw).dw_version, SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION);
            for slot in [1, 2, 3, 5, 7, 8, 9, 10, 15, 16, 24, 25] {
                assert_ne!((*ta).slots[slot], 0, "A slot {slot} empty");
                assert_ne!((*tw).slots[slot], 0, "W slot {slot} empty");
            }
        }
    }

    #[test]
    fn schannel_full_tls13_handshake_alpn_data_cert_chain_spki_and_shutdown() {
        unsafe {
            let ca_store = make_ca_memory_store();

            // Build SCHANNEL_CRED (v4) with SCH_CRED_MANUAL_CRED_VALIDATION + hRootStore = ca_store
            let mut cred_cfg = [0u8; 0x50];
            std::ptr::write_unaligned(cred_cfg.as_mut_ptr() as *mut u32, SCHANNEL_CRED_VERSION);
            std::ptr::write_unaligned(cred_cfg.as_mut_ptr().add(0x10) as *mut *mut c_void, ca_store);
            std::ptr::write_unaligned(
                cred_cfg.as_mut_ptr().add(0x48) as *mut u32,
                SCH_CRED_MANUAL_CRED_VALIDATION,
            );

            let pkg = b"Microsoft Unified Security Protocol Provider\0";
            let mut cred = SecHandle::default();
            let mut expiry = 0i64;
            assert_eq!(
                AcquireCredentialsHandleA(
                    std::ptr::null(),
                    pkg.as_ptr(),
                    SECPKG_CRED_OUTBOUND,
                    std::ptr::null_mut(),
                    cred_cfg.as_mut_ptr() as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut cred as *mut SecHandle as *mut c_void,
                    &mut expiry,
                ),
                SEC_E_OK
            );

            // Prepare ALPN input buffer: ["h2", "http/1.1"]
            let mut alpn_payload = Vec::new();
            let proto_list: &[u8] = &[2, b'h', b'2', 8, b'h', b't', b't', b'p', b'/', b'1', b'.', b'1'];
            let lists_size = (4 + 2 + proto_list.len()) as u32;
            alpn_payload.extend_from_slice(&lists_size.to_le_bytes());
            alpn_payload.extend_from_slice(&2u32.to_le_bytes()); // ALPN ext
            alpn_payload.extend_from_slice(&(proto_list.len() as u16).to_le_bytes());
            alpn_payload.extend_from_slice(proto_list);

            let mut alpn_in = SecBuffer {
                cb_buffer: alpn_payload.len() as u32,
                buffer_type: SECBUFFER_APPLICATION_PROTOCOLS,
                pv_buffer: alpn_payload.as_mut_ptr() as *mut c_void,
            };
            let mut alpn_desc = SecBufferDesc {
                ul_version: 0,
                c_buffers: 1,
                p_buffers: &mut alpn_in,
            };

            let mut out_buf = SecBuffer {
                cb_buffer: 0,
                buffer_type: SECBUFFER_TOKEN,
                pv_buffer: std::ptr::null_mut(),
            };
            let mut out_desc = SecBufferDesc {
                ul_version: 0,
                c_buffers: 1,
                p_buffers: &mut out_buf,
            };

            let target = b"test.example.com\0";
            let mut ctx = SecHandle::default();
            let mut attrs = 0u32;
            let req = ISC_REQ_SEQUENCE_DETECT
                | ISC_REQ_REPLAY_DETECT
                | ISC_REQ_CONFIDENTIALITY
                | ISC_REQ_ALLOCATE_MEMORY
                | ISC_REQ_STREAM;

            let st = InitializeSecurityContextA(
                &mut cred as *mut SecHandle as *mut c_void,
                std::ptr::null_mut(),
                target.as_ptr(),
                req,
                0,
                0,
                &mut alpn_desc as *mut SecBufferDesc as *mut c_void,
                0,
                &mut ctx as *mut SecHandle as *mut c_void,
                &mut out_desc as *mut SecBufferDesc as *mut c_void,
                &mut attrs,
                &mut expiry,
            );
            assert_eq!(st, SEC_I_CONTINUE_NEEDED);
            assert!(!out_buf.pv_buffer.is_null() && out_buf.cb_buffer > 0);

            let mut srv = make_test_server(&[&rustls::version::TLS13], vec![b"h2".to_vec()]);
            let ch = std::slice::from_raw_parts(out_buf.pv_buffer as *const u8, out_buf.cb_buffer as usize).to_vec();
            assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);

            // Server responds to ClientHello; intentionally feed fragmented first to exercise SEC_E_INCOMPLETE_MESSAGE!
            let srv_flight = server_step(&mut srv, &ch);
            assert!(srv_flight.len() > 20);

            let mut partial = srv_flight[..7].to_vec();
            let mut in_bufs = [
                SecBuffer {
                    cb_buffer: partial.len() as u32,
                    buffer_type: SECBUFFER_TOKEN,
                    pv_buffer: partial.as_mut_ptr() as *mut c_void,
                },
                SecBuffer {
                    cb_buffer: 0,
                    buffer_type: SECBUFFER_EMPTY,
                    pv_buffer: std::ptr::null_mut(),
                },
            ];
            let mut in_desc = SecBufferDesc {
                ul_version: 0,
                c_buffers: 2,
                p_buffers: in_bufs.as_mut_ptr(),
            };
            out_buf = SecBuffer {
                cb_buffer: 0,
                buffer_type: SECBUFFER_TOKEN,
                pv_buffer: std::ptr::null_mut(),
            };
            let st_inc = InitializeSecurityContextA(
                &mut cred as *mut SecHandle as *mut c_void,
                &mut ctx as *mut SecHandle as *mut c_void,
                target.as_ptr(),
                req,
                0,
                0,
                &mut in_desc as *mut SecBufferDesc as *mut c_void,
                0,
                &mut ctx as *mut SecHandle as *mut c_void,
                &mut out_desc as *mut SecBufferDesc as *mut c_void,
                &mut attrs,
                &mut expiry,
            );
            assert_eq!(st_inc, SEC_E_INCOMPLETE_MESSAGE);
            assert_eq!(in_bufs[1].buffer_type, SECBUFFER_MISSING);
            assert!(in_bufs[1].cb_buffer > 0);

            // Feed full server flight + 5 trailing dummy bytes to verify SECBUFFER_EXTRA preservation
            let mut full_in = srv_flight.clone();
            full_in.extend_from_slice(&[0x17, 0x03, 0x03, 0x00, 0x01]);
            in_bufs[0] = SecBuffer {
                cb_buffer: full_in.len() as u32,
                buffer_type: SECBUFFER_TOKEN,
                pv_buffer: full_in.as_mut_ptr() as *mut c_void,
            };
            in_bufs[1] = SecBuffer {
                cb_buffer: 0,
                buffer_type: SECBUFFER_EMPTY,
                pv_buffer: std::ptr::null_mut(),
            };
            let st2 = InitializeSecurityContextA(
                &mut cred as *mut SecHandle as *mut c_void,
                &mut ctx as *mut SecHandle as *mut c_void,
                target.as_ptr(),
                req,
                0,
                0,
                &mut in_desc as *mut SecBufferDesc as *mut c_void,
                0,
                &mut ctx as *mut SecHandle as *mut c_void,
                &mut out_desc as *mut SecBufferDesc as *mut c_void,
                &mut attrs,
                &mut expiry,
            );
            assert_eq!(st2, SEC_E_OK);
            assert_eq!(in_bufs[1].buffer_type, SECBUFFER_EXTRA);
            assert_eq!(in_bufs[1].cb_buffer, 5);

            // Send Client Finished to server
            let cfin = std::slice::from_raw_parts(out_buf.pv_buffer as *const u8, out_buf.cb_buffer as usize).to_vec();
            assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);
            let post_hs = server_step(&mut srv, &cfin);
            assert!(!srv.is_handshaking());

            // QueryContextAttributes: STREAM_SIZES, CONNECTION_INFO, APPLICATION_PROTOCOL, REMOTE_CERT_CONTEXT
            let mut sizes = std::mem::zeroed::<SecPkgContextStreamSizes>();
            assert_eq!(
                QueryContextAttributesA(
                    &mut ctx as *mut SecHandle as *mut c_void,
                    SECPKG_ATTR_STREAM_SIZES,
                    &mut sizes as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert_eq!(sizes.cb_header, STREAM_HEADER_SIZE);
            assert_eq!(sizes.cb_trailer, STREAM_TRAILER_SIZE);

            let mut cinfo = std::mem::zeroed::<SecPkgContextConnectionInfo>();
            assert_eq!(
                QueryContextAttributesA(
                    &mut ctx as *mut SecHandle as *mut c_void,
                    SECPKG_ATTR_CONNECTION_INFO,
                    &mut cinfo as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert_eq!(cinfo.dw_protocol, SP_PROT_TLS1_3_CLIENT);

            let mut alpn_out = std::mem::zeroed::<SecPkgContextApplicationProtocol>();
            assert_eq!(
                QueryContextAttributesA(
                    &mut ctx as *mut SecHandle as *mut c_void,
                    SECPKG_ATTR_APPLICATION_PROTOCOL,
                    &mut alpn_out as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert_eq!(alpn_out.proto_nego_status, 1);
            assert_eq!(&alpn_out.protocol_id[..alpn_out.protocol_id_size as usize], b"h2");

            // Peer cert context + CertGetCertificateChain + CertVerifyCertificateChainPolicy + CryptEncodeObjectEx(SPKI)
            let mut cert_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                QueryContextAttributesA(
                    &mut ctx as *mut SecHandle as *mut c_void,
                    SECPKG_ATTR_REMOTE_CERT_CONTEXT,
                    &mut cert_ptr as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert!(!cert_ptr.is_null());

            let mut chain_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crypt32::CertGetCertificateChain(
                    std::ptr::null_mut(),
                    cert_ptr,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                    &mut chain_ptr,
                ),
                1
            );
            assert!(!chain_ptr.is_null());

            // Verify SSL policy passes for matching hostname "test.example.com" (CA was in hRootStore)
            let host_ok: Vec<u16> = "test.example.com\0".encode_utf16().collect();
            #[repr(C)]
            struct SslPara { cb: u32, auth: u32, checks: u32, host: *const u16 }
            #[repr(C)]
            struct PolPara { cb: u32, flags: u32, extra: *const c_void }
            #[repr(C)]
            struct PolStatus { cb: u32, err: u32, ci: i32, ei: i32, extra: *mut c_void }

            let ssl_ok = SslPara { cb: 24, auth: 2, checks: 0, host: host_ok.as_ptr() };
            let para_ok = PolPara { cb: 16, flags: 0, extra: &ssl_ok as *const _ as *const c_void };
            let mut status_ok = PolStatus { cb: 24, err: 99, ci: 0, ei: 0, extra: std::ptr::null_mut() };
            assert_eq!(
                crypt32::CertVerifyCertificateChainPolicy(
                    4 as *const u8,
                    chain_ptr,
                    &para_ok as *const _ as *const c_void,
                    &mut status_ok as *mut _ as *mut c_void,
                ),
                1
            );
            assert_eq!(status_ok.err, 0);

            // Verify SSL policy FAILS with CERT_E_CN_NO_MATCH for wrong hostname
            let host_bad: Vec<u16> = "evil.example.com\0".encode_utf16().collect();
            let ssl_bad = SslPara { cb: 24, auth: 2, checks: 0, host: host_bad.as_ptr() };
            let para_bad = PolPara { cb: 16, flags: 0, extra: &ssl_bad as *const _ as *const c_void };
            let mut status_bad = PolStatus { cb: 24, err: 0, ci: 0, ei: 0, extra: std::ptr::null_mut() };
            assert_eq!(
                crypt32::CertVerifyCertificateChainPolicy(
                    4 as *const u8,
                    chain_ptr,
                    &para_bad as *const _ as *const c_void,
                    &mut status_bad as *mut _ as *mut c_void,
                ),
                1
            );
            assert_eq!(status_bad.err, crypt32::CERT_E_CN_NO_MATCH);

            // CryptEncodeObjectEx(X509_PUBLIC_KEY_INFO) matches leaf cert SPKI DER
            let cc = std::ptr::read_unaligned(cert_ptr as *const CertContext);
            let spki_ptr = &(*cc.p_cert_info).subject_public_key_info as *const _ as *const c_void;
            let mut spki_len = 0u32;
            assert_eq!(
                crypt32::CryptEncodeObjectEx(1, 8 as *const u8, spki_ptr, 0, std::ptr::null(), std::ptr::null_mut(), &mut spki_len),
                1
            );
            assert_eq!(spki_len, 44); // Ed25519 SPKI is exactly 44 bytes DER
            let mut spki_der = vec![0u8; spki_len as usize];
            assert_eq!(
                crypt32::CryptEncodeObjectEx(1, 8 as *const u8, spki_ptr, 0, std::ptr::null(), spki_der.as_mut_ptr() as *mut c_void, &mut spki_len),
                1
            );
            assert!(TEST_LEAF_DER.windows(spki_der.len()).any(|w| w == spki_der.as_slice()));

            crypt32::CertFreeCertificateChain(chain_ptr);
            assert_eq!(crypt32::CertFreeCertificateContext(cert_ptr), 1);

            // EncryptMessage -> Server decrypts exact application request
            let msg = b"GET /health HTTP/1.1\r\nHost: test.example.com\r\n\r\n";
            let mut wire = vec![0u8; sizes.cb_header as usize + msg.len() + sizes.cb_trailer as usize];
            let hdr_len = sizes.cb_header as usize;
            wire[hdr_len..hdr_len + msg.len()].copy_from_slice(msg);
            let mut enc_bufs = [
                SecBuffer { cb_buffer: sizes.cb_header, buffer_type: SECBUFFER_STREAM_HEADER, pv_buffer: wire.as_mut_ptr() as *mut c_void },
                SecBuffer { cb_buffer: msg.len() as u32, buffer_type: SECBUFFER_DATA, pv_buffer: wire.as_mut_ptr().add(hdr_len) as *mut c_void },
                SecBuffer { cb_buffer: sizes.cb_trailer, buffer_type: SECBUFFER_STREAM_TRAILER, pv_buffer: wire.as_mut_ptr().add(hdr_len + msg.len()) as *mut c_void },
                SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_EMPTY, pv_buffer: std::ptr::null_mut() },
            ];
            let mut enc_desc = SecBufferDesc { ul_version: 0, c_buffers: 4, p_buffers: enc_bufs.as_mut_ptr() };
            assert_eq!(EncryptMessage(&mut ctx as *mut SecHandle as *mut c_void, 0, &mut enc_desc as *mut _ as *mut c_void, 0), SEC_E_OK);
            let total_enc = (enc_bufs[0].cb_buffer + enc_bufs[1].cb_buffer + enc_bufs[2].cb_buffer) as usize;
            let _ = server_step(&mut srv, &wire[..total_enc]);
            let mut srv_plain = vec![0u8; msg.len()];
            srv.reader().read_exact(&mut srv_plain).unwrap();
            assert_eq!(&srv_plain, msg);

            // Server sends response (+ TLS 1.3 post-handshake NewSessionTicket in post_hs!) + close_notify
            let resp = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
            srv.writer().write_all(resp).unwrap();
            srv.writer().flush().unwrap();
            srv.send_close_notify();
            let mut srv_wire = post_hs;
            while srv.wants_write() {
                if srv.write_tls(&mut srv_wire).unwrap() == 0 {
                    break;
                }
            }

            // DecryptMessage handles post-handshake NewSessionTicket, extracts exact resp, then SEC_I_CONTEXT_EXPIRED
            let mut recv_buf = srv_wire;
            let mut got_plain = Vec::new();
            let mut saw_expired = false;
            for _ in 0..8 {
                let mut dec_bufs = [
                    SecBuffer { cb_buffer: recv_buf.len() as u32, buffer_type: SECBUFFER_DATA, pv_buffer: recv_buf.as_mut_ptr() as *mut c_void },
                    SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_EMPTY, pv_buffer: std::ptr::null_mut() },
                    SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_EMPTY, pv_buffer: std::ptr::null_mut() },
                    SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_EMPTY, pv_buffer: std::ptr::null_mut() },
                ];
                let mut dec_desc = SecBufferDesc { ul_version: 0, c_buffers: 4, p_buffers: dec_bufs.as_mut_ptr() };
                let ds = DecryptMessage(&mut ctx as *mut SecHandle as *mut c_void, &mut dec_desc as *mut _ as *mut c_void, 0, std::ptr::null_mut());
                if ds == SEC_I_CONTEXT_EXPIRED {
                    saw_expired = true;
                    break;
                }
                assert_eq!(ds, SEC_E_OK);
                for b in &dec_bufs {
                    if b.buffer_type == SECBUFFER_DATA && b.cb_buffer > 0 {
                        got_plain.extend_from_slice(std::slice::from_raw_parts(b.pv_buffer as *const u8, b.cb_buffer as usize));
                    }
                }
                let next = dec_bufs
                    .iter()
                    .find(|b| b.buffer_type == SECBUFFER_EXTRA && b.cb_buffer > 0)
                    .map(|b| std::slice::from_raw_parts(b.pv_buffer as *const u8, b.cb_buffer as usize).to_vec())
                    .unwrap_or_default();
                recv_buf = next;
                if recv_buf.is_empty() {
                    break;
                }
            }
            assert_eq!(&got_plain, resp);
            assert!(saw_expired);

            // Client shutdown via ApplyControlToken(SCHANNEL_SHUTDOWN) + InitializeSecurityContextA
            let mut shut_tok = SCHANNEL_SHUTDOWN;
            let mut ctl_buf = SecBuffer { cb_buffer: 4, buffer_type: SECBUFFER_TOKEN, pv_buffer: &mut shut_tok as *mut u32 as *mut c_void };
            let mut ctl_desc = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut ctl_buf };
            assert_eq!(ApplyControlToken(&mut ctx as *mut SecHandle as *mut c_void, &mut ctl_desc as *mut _ as *mut c_void), SEC_E_OK);
            out_buf = SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_TOKEN, pv_buffer: std::ptr::null_mut() };
            assert_eq!(
                InitializeSecurityContextA(
                    &mut cred as *mut SecHandle as *mut c_void,
                    &mut ctx as *mut SecHandle as *mut c_void,
                    std::ptr::null(),
                    req,
                    0,
                    0,
                    std::ptr::null_mut(),
                    0,
                    &mut ctx as *mut SecHandle as *mut c_void,
                    &mut out_desc as *mut SecBufferDesc as *mut c_void,
                    &mut attrs,
                    std::ptr::null_mut(),
                ),
                SEC_E_OK
            );
            assert!(!out_buf.pv_buffer.is_null() && out_buf.cb_buffer > 0);
            assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);

            assert_eq!(DeleteSecurityContext(&mut ctx as *mut SecHandle as *mut c_void), SEC_E_OK);
            assert_eq!(FreeCredentialsHandle(&mut cred as *mut SecHandle as *mut c_void), SEC_E_OK);
            assert_eq!(crypt32::CertCloseStore(ca_store, 0), 1);
        }
    }

    #[test]
    fn auto_validation_rejects_untrusted_root_honestly() {
        unsafe {
            // Acquire default Schannel creds WITHOUT manual validation and WITHOUT test CA -> must reject!
            let pkg = b"Schannel\0";
            let mut cred = SecHandle::default();
            assert_eq!(
                AcquireCredentialsHandleA(
                    std::ptr::null(),
                    pkg.as_ptr(),
                    SECPKG_CRED_OUTBOUND,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut cred as *mut SecHandle as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SEC_E_OK
            );
            let mut out_buf = SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_TOKEN, pv_buffer: std::ptr::null_mut() };
            let mut out_desc = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut out_buf };
            let mut ctx = SecHandle::default();
            let target = b"test.example.com\0";
            let req = ISC_REQ_ALLOCATE_MEMORY | ISC_REQ_STREAM;

            assert_eq!(
                InitializeSecurityContextA(
                    &mut cred as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    target.as_ptr(),
                    req,
                    0,
                    0,
                    std::ptr::null_mut(),
                    0,
                    &mut ctx as *mut _ as *mut c_void,
                    &mut out_desc as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SEC_I_CONTINUE_NEEDED
            );
            let ch = std::slice::from_raw_parts(out_buf.pv_buffer as *const u8, out_buf.cb_buffer as usize).to_vec();
            assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);

            let mut srv = make_test_server(&[&rustls::version::TLS12], vec![]);
            let mut srv_flight = server_step(&mut srv, &ch);
            let mut in_buf = SecBuffer {
                cb_buffer: srv_flight.len() as u32,
                buffer_type: SECBUFFER_TOKEN,
                pv_buffer: srv_flight.as_mut_ptr() as *mut c_void,
            };
            let mut in_desc = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut in_buf };
            let mut out_buf2 = SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_TOKEN, pv_buffer: std::ptr::null_mut() };
            let mut out_desc2 = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut out_buf2 };
            let res = InitializeSecurityContextA(
                &mut cred as *mut _ as *mut c_void,
                &mut ctx as *mut _ as *mut c_void,
                target.as_ptr(),
                req,
                0,
                0,
                &mut in_desc as *mut _ as *mut c_void,
                0,
                &mut ctx as *mut _ as *mut c_void,
                &mut out_desc2 as *mut _ as *mut c_void,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert_eq!(res, SEC_E_UNTRUSTED_ROOT);
            assert_eq!(DeleteSecurityContext(&mut ctx as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(FreeCredentialsHandle(&mut cred as *mut _ as *mut c_void), SEC_E_OK);
        }
    }

    #[test]
    fn sch_credentials_v5_tls12_and_custom_chain_engine_exclusive_root() {
        unsafe {
            // Build SCH_CREDENTIALS (v5) disabling TLS 1.3 via TLS_PARAMETERS.grbitDisabledProtocols
            // and enabling manual cert validation (verifying via CertCreateCertificateChainEngine!).
            let mut tls_params = [0u8; 0x28];
            std::ptr::write_unaligned(tls_params.as_mut_ptr().add(0x10) as *mut u32, SP_PROT_TLS1_3_CLIENT);

            let mut cred_v5 = [0u8; 0x48];
            std::ptr::write_unaligned(cred_v5.as_mut_ptr() as *mut u32, SCH_CREDENTIALS_VERSION);
            std::ptr::write_unaligned(cred_v5.as_mut_ptr().add(0x34) as *mut u32, SCH_CRED_MANUAL_CRED_VALIDATION);
            std::ptr::write_unaligned(cred_v5.as_mut_ptr().add(0x38) as *mut u32, 1u32);
            std::ptr::write_unaligned(cred_v5.as_mut_ptr().add(0x40) as *mut *const u8, tls_params.as_ptr());

            let pkg_w: Vec<u16> = "Microsoft Unified Security Protocol Provider\0".encode_utf16().collect();
            let mut cred = SecHandle::default();
            assert_eq!(
                AcquireCredentialsHandleW(
                    std::ptr::null(),
                    pkg_w.as_ptr(),
                    SECPKG_CRED_OUTBOUND,
                    std::ptr::null_mut(),
                    cred_v5.as_mut_ptr() as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut cred as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SEC_E_OK
            );

            let target_w: Vec<u16> = "127.0.0.1\0".encode_utf16().collect();
            let mut ctx = SecHandle::default();
            let mut srv = make_test_server(&[&rustls::version::TLS12], vec![]);
            let req = ISC_REQ_ALLOCATE_MEMORY | ISC_REQ_STREAM;

            let mut out_buf = SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_TOKEN, pv_buffer: std::ptr::null_mut() };
            let mut out_desc = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut out_buf };
            assert_eq!(
                InitializeSecurityContextW(
                    &mut cred as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    target_w.as_ptr(),
                    req,
                    0,
                    0,
                    std::ptr::null_mut(),
                    0,
                    &mut ctx as *mut _ as *mut c_void,
                    &mut out_desc as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SEC_I_CONTINUE_NEEDED
            );

            // Drive TLS 1.2 2-RTT handshake to completion
            let mut to_server = std::slice::from_raw_parts(out_buf.pv_buffer as *const u8, out_buf.cb_buffer as usize).to_vec();
            assert_eq!(FreeContextBuffer(out_buf.pv_buffer), SEC_E_OK);
            let mut done = false;
            for _ in 0..6 {
                let mut from_server = server_step(&mut srv, &to_server);
                if from_server.is_empty() && !srv.is_handshaking() {
                    break;
                }
                let mut in_buf = SecBuffer {
                    cb_buffer: from_server.len() as u32,
                    buffer_type: SECBUFFER_TOKEN,
                    pv_buffer: from_server.as_mut_ptr() as *mut c_void,
                };
                let mut in_desc = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut in_buf };
                let mut ob = SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_TOKEN, pv_buffer: std::ptr::null_mut() };
                let mut od = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut ob };
                let st = InitializeSecurityContextW(
                    &mut cred as *mut _ as *mut c_void,
                    &mut ctx as *mut _ as *mut c_void,
                    target_w.as_ptr(),
                    req,
                    0,
                    0,
                    &mut in_desc as *mut _ as *mut c_void,
                    0,
                    &mut ctx as *mut _ as *mut c_void,
                    &mut od as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                to_server = if !ob.pv_buffer.is_null() && ob.cb_buffer > 0 {
                    let v = std::slice::from_raw_parts(ob.pv_buffer as *const u8, ob.cb_buffer as usize).to_vec();
                    assert_eq!(FreeContextBuffer(ob.pv_buffer), SEC_E_OK);
                    v
                } else {
                    Vec::new()
                };
                if st == SEC_E_OK {
                    done = true;
                    break;
                }
                assert_eq!(st, SEC_I_CONTINUE_NEEDED);
            }
            assert!(done, "TLS 1.2 handshake did not complete");

            let mut cinfo = std::mem::zeroed::<SecPkgContextConnectionInfo>();
            assert_eq!(
                QueryContextAttributesW(
                    &mut ctx as *mut _ as *mut c_void,
                    SECPKG_ATTR_CONNECTION_INFO,
                    &mut cinfo as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert_eq!(cinfo.dw_protocol, SP_PROT_TLS1_2_CLIENT);

            // Verify peer cert via custom CertCreateCertificateChainEngine(hExclusiveRoot = ca_store)
            let mut cert_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                QueryContextAttributesW(
                    &mut ctx as *mut _ as *mut c_void,
                    SECPKG_ATTR_REMOTE_CERT_CONTEXT,
                    &mut cert_ptr as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            let ca_store = make_ca_memory_store();
            let mut eng_cfg = [0u8; 0x58];
            std::ptr::write_unaligned(eng_cfg.as_mut_ptr() as *mut u32, 0x58u32);
            std::ptr::write_unaligned(eng_cfg.as_mut_ptr().add(0x40) as *mut *mut c_void, ca_store);
            let mut engine: *mut c_void = std::ptr::null_mut();
            assert_eq!(crypt32::CertCreateCertificateChainEngine(eng_cfg.as_ptr() as *const c_void, &mut engine), 1);

            let mut chain_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crypt32::CertGetCertificateChain(
                    engine,
                    cert_ptr,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                    &mut chain_ptr,
                ),
                1
            );

            #[repr(C)]
            struct SslPara { cb: u32, auth: u32, checks: u32, host: *const u16 }
            #[repr(C)]
            struct PolPara { cb: u32, flags: u32, extra: *const c_void }
            #[repr(C)]
            struct PolStatus { cb: u32, err: u32, ci: i32, ei: i32, extra: *mut c_void }
            let ssl = SslPara { cb: 24, auth: 2, checks: 0, host: target_w.as_ptr() };
            let para = PolPara { cb: 16, flags: 0, extra: &ssl as *const _ as *const c_void };
            let mut status = PolStatus { cb: 24, err: 99, ci: 0, ei: 0, extra: std::ptr::null_mut() };
            assert_eq!(
                crypt32::CertVerifyCertificateChainPolicy(
                    4 as *const u8,
                    chain_ptr,
                    &para as *const _ as *const c_void,
                    &mut status as *mut _ as *mut c_void,
                ),
                1
            );
            assert_eq!(status.err, 0);

            crypt32::CertFreeCertificateChain(chain_ptr);
            crypt32::CertFreeCertificateChainEngine(engine);
            assert_eq!(crypt32::CertFreeCertificateContext(cert_ptr), 1);
            assert_eq!(crypt32::CertCloseStore(ca_store, 0), 1);
            assert_eq!(DeleteSecurityContext(&mut ctx as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(FreeCredentialsHandle(&mut cred as *mut _ as *mut c_void), SEC_E_OK);
        }
    }

    #[test]
    fn query_context_cipher_info_names_creds_and_shutdown_close_notify() {
        assert_eq!(std::mem::size_of::<SecPkgContextCipherInfo>(), 0x2A8);
        assert_eq!(std::mem::offset_of!(SecPkgContextCipherInfo, sz_cipher_suite), 0x10);
        assert_eq!(std::mem::offset_of!(SecPkgContextCipherInfo, sz_cipher), 0x90);
        assert_eq!(std::mem::offset_of!(SecPkgContextCipherInfo, dw_cipher_len), 0x110);
        assert_eq!(std::mem::offset_of!(SecPkgContextCipherInfo, sz_hash), 0x118);
        assert_eq!(std::mem::offset_of!(SecPkgContextCipherInfo, sz_exchange), 0x19C);
        assert_eq!(std::mem::offset_of!(SecPkgContextCipherInfo, sz_certificate), 0x224);

        unsafe {
            let ca_store = make_ca_memory_store();
            let mut cred_cfg = [0u8; 0x50];
            std::ptr::write_unaligned(cred_cfg.as_mut_ptr() as *mut u32, SCHANNEL_CRED_VERSION);
            std::ptr::write_unaligned(cred_cfg.as_mut_ptr().add(0x10) as *mut *mut c_void, ca_store);

            let principal = b"client@test.example.com\0";
            let pkg = b"Schannel\0";
            let mut cred = SecHandle::default();
            assert_eq!(
                AcquireCredentialsHandleA(
                    principal.as_ptr(),
                    pkg.as_ptr(),
                    SECPKG_CRED_OUTBOUND,
                    std::ptr::null_mut(),
                    cred_cfg.as_mut_ptr() as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut cred as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                ),
                SEC_E_OK
            );

            // QueryCredentialsAttributesA & W (SECPKG_CRED_ATTR_NAMES + SUPPORTED_PROTOCOLS)
            let mut cred_names_a = SecPkgContextNamesA { s_user_name: std::ptr::null_mut() };
            assert_eq!(
                QueryCredentialsAttributesA(
                    &mut cred as *mut _ as *mut c_void,
                    SECPKG_CRED_ATTR_NAMES,
                    &mut cred_names_a as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert!(!cred_names_a.s_user_name.is_null());
            assert_eq!(crate::strings::from_ansi(cred_names_a.s_user_name), "client@test.example.com");
            assert_eq!(FreeContextBuffer(cred_names_a.s_user_name as *mut c_void), SEC_E_OK);

            let mut cred_names_w = SecPkgContextNamesW { s_user_name: std::ptr::null_mut() };
            assert_eq!(
                QueryCredentialsAttributesW(
                    &mut cred as *mut _ as *mut c_void,
                    SECPKG_CRED_ATTR_NAMES,
                    &mut cred_names_w as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert!(!cred_names_w.s_user_name.is_null());
            assert_eq!(crate::strings::from_wide(cred_names_w.s_user_name), "client@test.example.com");
            assert_eq!(FreeContextBuffer(cred_names_w.s_user_name as *mut c_void), SEC_E_OK);

            let mut sup_prot = SecPkgCredSupportedProtocols { grbit_protocol: 0 };
            assert_eq!(
                QueryCredentialsAttributesA(
                    &mut cred as *mut _ as *mut c_void,
                    SECPKG_CRED_ATTR_SUPPORTED_PROTOCOLS,
                    &mut sup_prot as *mut _ as *mut c_void,
                ),
                SEC_E_OK
            );
            assert_eq!(sup_prot.grbit_protocol, SP_PROT_TLS1_2_CLIENT | SP_PROT_TLS1_3_CLIENT);

            // Complete a TLS 1.3 handshake
            let target = b"test.example.com\0";
            let mut ctx = SecHandle::default();
            let req = ISC_REQ_ALLOCATE_MEMORY | ISC_REQ_STREAM;
            let mut ob = SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_TOKEN, pv_buffer: std::ptr::null_mut() };
            let mut od = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut ob };
            assert_eq!(
                InitializeSecurityContextA(
                    &mut cred as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    target.as_ptr(),
                    req,
                    0,
                    0,
                    std::ptr::null_mut(),
                    0,
                    &mut ctx as *mut _ as *mut c_void,
                    &mut od as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SEC_I_CONTINUE_NEEDED
            );
            let ch = std::slice::from_raw_parts(ob.pv_buffer as *const u8, ob.cb_buffer as usize).to_vec();
            assert_eq!(FreeContextBuffer(ob.pv_buffer), SEC_E_OK);

            let mut srv = make_test_server(&[&rustls::version::TLS13], vec![]);
            let mut sf = server_step(&mut srv, &ch);
            let mut ib = SecBuffer { cb_buffer: sf.len() as u32, buffer_type: SECBUFFER_TOKEN, pv_buffer: sf.as_mut_ptr() as *mut c_void };
            let mut id = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut ib };
            ob = SecBuffer { cb_buffer: 0, buffer_type: SECBUFFER_TOKEN, pv_buffer: std::ptr::null_mut() };
            assert_eq!(
                InitializeSecurityContextA(
                    &mut cred as *mut _ as *mut c_void,
                    &mut ctx as *mut _ as *mut c_void,
                    target.as_ptr(),
                    req,
                    0,
                    0,
                    &mut id as *mut _ as *mut c_void,
                    0,
                    &mut ctx as *mut _ as *mut c_void,
                    &mut od as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SEC_E_OK
            );
            let cfin = std::slice::from_raw_parts(ob.pv_buffer as *const u8, ob.cb_buffer as usize).to_vec();
            assert_eq!(FreeContextBuffer(ob.pv_buffer), SEC_E_OK);
            let _ = server_step(&mut srv, &cfin);
            assert!(!srv.is_handshaking());

            // QueryContextAttributes: SIZES, STREAM_SIZES, CONNECTION_INFO, CIPHER_INFO, ISSUER_LIST_EX, NAMES (A & W)
            let mut ctx_sizes = std::mem::zeroed::<SecPkgContextSizes>();
            assert_eq!(QueryContextAttributesA(&mut ctx as *mut _ as *mut c_void, SECPKG_ATTR_SIZES, &mut ctx_sizes as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(ctx_sizes.cb_max_token, STREAM_MAX_MESSAGE);

            let mut ci = std::mem::zeroed::<SecPkgContextCipherInfo>();
            assert_eq!(QueryContextAttributesW(&mut ctx as *mut _ as *mut c_void, SECPKG_ATTR_CIPHER_INFO, &mut ci as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(ci.dw_version, SECPKGCONTEXT_CIPHERINFO_V1);
            assert_eq!(ci.dw_protocol, SP_PROT_TLS1_3_CLIENT);
            assert!(matches!(ci.dw_cipher_suite, 0x1301 | 0x1302 | 0x1303));
            let suite_str = crate::strings::from_wide(ci.sz_cipher_suite.as_ptr());
            assert!(suite_str.starts_with("TLS_"), "unexpected suite {suite_str}");
            assert!(ci.dw_cipher_len == 128 || ci.dw_cipher_len == 256);
            assert!(ci.dw_hash_len == 256 || ci.dw_hash_len == 384);

            let mut issuers = SecPkgContextIssuerListInfoEx { a_issuers: 1 as *mut c_void, c_issuers: 99 };
            assert_eq!(QueryContextAttributesA(&mut ctx as *mut _ as *mut c_void, SECPKG_ATTR_ISSUER_LIST_EX, &mut issuers as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(issuers.c_issuers, 0);
            assert!(issuers.a_issuers.is_null());

            let mut names_a = SecPkgContextNamesA { s_user_name: std::ptr::null_mut() };
            assert_eq!(QueryContextAttributesA(&mut ctx as *mut _ as *mut c_void, SECPKG_ATTR_NAMES, &mut names_a as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(crate::strings::from_ansi(names_a.s_user_name), "test.example.com");
            assert_eq!(FreeContextBuffer(names_a.s_user_name as *mut c_void), SEC_E_OK);

            let mut names_w = SecPkgContextNamesW { s_user_name: std::ptr::null_mut() };
            assert_eq!(QueryContextAttributesW(&mut ctx as *mut _ as *mut c_void, SECPKG_ATTR_NAMES, &mut names_w as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(crate::strings::from_wide(names_w.s_user_name), "test.example.com");
            assert_eq!(FreeContextBuffer(names_w.s_user_name as *mut c_void), SEC_E_OK);

            // ApplyControlToken(SCHANNEL_SHUTDOWN) -> retry after BUFFER_TOO_SMALL -> server observes genuine close_notify EOF!
            let mut shut_tok = SCHANNEL_SHUTDOWN;
            let mut ctl_buf = SecBuffer { cb_buffer: 4, buffer_type: SECBUFFER_TOKEN, pv_buffer: &mut shut_tok as *mut u32 as *mut c_void };
            let mut ctl_desc = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut ctl_buf };
            assert_eq!(ApplyControlToken(&mut ctx as *mut _ as *mut c_void, &mut ctl_desc as *mut _ as *mut c_void), SEC_E_OK);

            // First try with tiny caller-supplied buffer -> SEC_E_BUFFER_TOO_SMALL, preserving close_notify bytes
            let mut tiny = [0u8; 2];
            let mut tiny_buf = SecBuffer { cb_buffer: 2, buffer_type: SECBUFFER_TOKEN, pv_buffer: tiny.as_mut_ptr() as *mut c_void };
            let mut tiny_desc = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut tiny_buf };
            assert_eq!(
                InitializeSecurityContextA(
                    &mut cred as *mut _ as *mut c_void,
                    &mut ctx as *mut _ as *mut c_void,
                    std::ptr::null(),
                    ISC_REQ_STREAM,
                    0,
                    0,
                    std::ptr::null_mut(),
                    0,
                    &mut ctx as *mut _ as *mut c_void,
                    &mut tiny_desc as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SEC_E_BUFFER_TOO_SMALL
            );

            // Retry with adequate buffer -> emits close_notify TLS record; feed it to server and verify clean EOF!
            let mut alert_buf = [0u8; 128];
            let mut ab = SecBuffer { cb_buffer: alert_buf.len() as u32, buffer_type: SECBUFFER_TOKEN, pv_buffer: alert_buf.as_mut_ptr() as *mut c_void };
            let mut ad = SecBufferDesc { ul_version: 0, c_buffers: 1, p_buffers: &mut ab };
            assert_eq!(
                InitializeSecurityContextA(
                    &mut cred as *mut _ as *mut c_void,
                    &mut ctx as *mut _ as *mut c_void,
                    std::ptr::null(),
                    ISC_REQ_STREAM,
                    0,
                    0,
                    std::ptr::null_mut(),
                    0,
                    &mut ctx as *mut _ as *mut c_void,
                    &mut ad as *mut _ as *mut c_void,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                SEC_E_OK
            );
            assert!(ab.cb_buffer > 5);
            let mut wire = &alert_buf[..ab.cb_buffer as usize];
            srv.read_tls(&mut wire).unwrap();
            let io_st = srv.process_new_packets().unwrap();
            assert!(io_st.peer_has_closed(), "server did not receive TLS close_notify");

            assert_eq!(DeleteSecurityContext(&mut ctx as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(FreeCredentialsHandle(&mut cred as *mut _ as *mut c_void), SEC_E_OK);
            assert_eq!(crypt32::CertCloseStore(ca_store, 0), 1);
        }
    }
}
