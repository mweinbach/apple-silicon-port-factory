//! crypt32.dll: certificate contexts, memory stores, chain verification,
//! SPKI DER encoding, DPAPI, and ASN.1 string/name helpers.
//!
//! Peer certificates returned by Schannel (`secur32.dll`) and certs created via
//! `CertCreateCertificateContext` / `CertOpenStore(CERT_STORE_PROV_MEMORY)` are
//! backed by Win64 ABI-exact `CERT_CONTEXT` / `CERT_INFO` / `CERT_CHAIN_CONTEXT`
//! structures and verified against `webpki-roots` (or caller-supplied exclusive
//! root stores from `CertCreateCertificateChainEngine`) via `rustls`'s WebPKI
//! verifier.

use super::wintrust::{
    self, CertChainContext, CertChainElement, CertContext, CertInfo, CertPublicKeyInfo,
    CertSimpleChain, CertTrustStatus, CmsgSignerInfo, CryptAlgId, CryptDataBlob, OwnedCertContext,
    ParsedPkcs7Info,
};
use crate::types::*;
use rustls::client::danger::ServerCertVerifier;
use rustls::client::WebPkiServerVerifier;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Arc, LazyLock, Mutex};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const CRYPT_E_NOT_FOUND: u32 = 0x8009_2004;
const CRYPT_E_NO_MATCH: u32 = 0x8009_2009;
const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

const X509_ASN_ENCODING: u32 = 0x0000_0001;
const PKCS_7_ASN_ENCODING: u32 = 0x0001_0000;
const X509_CERT: usize = 1;
const X509_CERT_TO_BE_SIGNED: usize = 2;
const X509_EXTENSIONS: usize = 5;
const X509_ANY_STRING: usize = 6;
const X509_NAME: usize = 7;
const X509_PUBLIC_KEY_INFO: usize = 8;
const X509_ALTERNATE_NAME: usize = 12;
const X509_KEY_USAGE: usize = 14;
const X509_BASIC_CONSTRAINTS2: usize = 15;
const RSA_CSP_PUBLICKEYBLOB: usize = 19;
const X509_UNICODE_NAME: usize = 20;
const X509_UNICODE_ANY_STRING: usize = 24;
const X509_OCTET_STRING: usize = 25;
const X509_BITS: usize = 26;
const X509_INTEGER: usize = 27;
const X509_MULTI_BYTE_INTEGER: usize = 28;
const X509_CHOICE_OF_TIME: usize = 30;
const X509_SEQUENCE_OF_ANY: usize = 34;
const X509_MULTI_BYTE_UINT: usize = 35;
const X509_ENHANCED_KEY_USAGE: usize = 36;
const X509_ECC_SIGNATURE: usize = 47;
const X509_OBJECT_IDENTIFIER: usize = 73;
const CRYPT_ENCODE_ALLOC_FLAG: u32 = 0x0000_8000;

const CERT_QUERY_OBJECT_FILE: u32 = 1;
const CERT_QUERY_OBJECT_BLOB: u32 = 2;
const CERT_QUERY_CONTENT_CERT: u32 = 1;
const CERT_QUERY_CONTENT_PKCS7_SIGNED: u32 = 8;
const CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED: u32 = 10;
const CERT_QUERY_FORMAT_BINARY: u32 = 1;
const CERT_QUERY_FORMAT_FLAG_BINARY: u32 = 1 << CERT_QUERY_FORMAT_BINARY;

const CMSG_TYPE_PARAM: u32 = 1;
const CMSG_SIGNER_COUNT_PARAM: u32 = 5;
const CMSG_SIGNER_INFO_PARAM: u32 = 6;
const CMSG_CERT_COUNT_PARAM: u32 = 11;
const CMSG_CERT_PARAM: u32 = 12;
const CMSG_VERSION_PARAM: u32 = 30;
const CMSG_SIGNED: u32 = 2;

const CERT_FIND_ANY: u32 = 0x0000_0000;
const CERT_FIND_SHA1_HASH: u32 = 0x0001_0000;
const CERT_FIND_MD5_HASH: u32 = 0x0004_0000;
const CERT_FIND_PUBLIC_KEY: u32 = 0x0006_0000;
const CERT_FIND_ISSUER_NAME: u32 = 0x0002_0004;
const CERT_FIND_SUBJECT_NAME: u32 = 0x0002_0007;
const CERT_FIND_ISSUER_STR_A: u32 = 0x0007_0004;
const CERT_FIND_SUBJECT_STR_A: u32 = 0x0007_0007;
const CERT_FIND_ISSUER_STR_W: u32 = 0x0008_0004;
const CERT_FIND_SUBJECT_STR_W: u32 = 0x0008_0007;
const CERT_FIND_SUBJECT_CERT: u32 = 0x000B_0000;
const CERT_FIND_ISSUER_OF: u32 = 0x000C_0000;
const CERT_FIND_EXISTING: u32 = 0x000D_0000;

const CERT_STORE_PROV_MEMORY: usize = 2;
const CERT_STORE_PROV_SYSTEM_A: usize = 9;
const CERT_STORE_PROV_SYSTEM_W: usize = 10;
const CERT_STORE_PROV_SYSTEM_REGISTRY_A: usize = 12;
const CERT_STORE_PROV_SYSTEM_REGISTRY_W: usize = 13;
const CERT_SYSTEM_STORE_CURRENT_USER: u32 = 0x0001_0000;

const CERT_STORE_ADD_NEW: u32 = 1;
const CERT_STORE_ADD_USE_EXISTING: u32 = 2;
const CERT_STORE_ADD_REPLACE_EXISTING: u32 = 3;
const CERT_STORE_ADD_ALWAYS: u32 = 4;

const CERT_CHAIN_POLICY_BASE: usize = 1;
const CERT_CHAIN_POLICY_AUTHENTICODE: usize = 2;
const CERT_CHAIN_POLICY_AUTHENTICODE_TS: usize = 3;
const CERT_CHAIN_POLICY_SSL: usize = 4;

// CERT_CHAIN_POLICY_PARA.dwFlags
const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG: u32 = 0x0000_0001;
const CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS: u32 = 0x0000_0007;
const CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG: u32 = 0x0000_0010;
const CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG: u32 = 0x0000_0020;
const CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG: u32 = 0x0000_0040;

// SSL_EXTRA_CERT_CHAIN_POLICY_PARA.fdwChecks
const SECURITY_FLAG_IGNORE_UNKNOWN_CA: u32 = 0x0000_0100;
const SECURITY_FLAG_IGNORE_CERT_CN_INVALID: u32 = 0x0000_1000;
const SECURITY_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 0x0000_2000;

// Windows CERT_E_* verification status codes
pub(crate) const CERT_E_EXPIRED: u32 = 0x800B_0101;
pub(crate) const CERT_E_UNTRUSTEDROOT: u32 = 0x800B_0109;
pub(crate) const CERT_E_CHAINING: u32 = 0x800B_010A;
pub(crate) const CERT_E_REVOKED: u32 = 0x800B_010C;
pub(crate) const CERT_E_CN_NO_MATCH: u32 = 0x800B_010F;
pub(crate) const CERT_E_WRONG_USAGE: u32 = 0x800B_0110;

// CERT_TRUST_STATUS.dwErrorStatus bits
const CERT_TRUST_IS_NOT_TIME_VALID: u32 = 0x0000_0001;
const CERT_TRUST_IS_REVOKED: u32 = 0x0000_0004;
const CERT_TRUST_IS_UNTRUSTED_ROOT: u32 = 0x0000_0020;
const CERT_TRUST_IS_PARTIAL_CHAIN: u32 = 0x0001_0000;
const CERT_TRUST_IS_NOT_VALID_FOR_USAGE: u32 = 0x0000_0010;

// @exports PFXImportCertStore CertFindExtension CryptMsgOpenToDecode CryptFindOIDInfo
failing!(*mut c_void = std::ptr::null_mut(), err CRYPT_E_NOT_FOUND;
    PFXImportCertStore,
    CertFindExtension,
    CryptMsgOpenToDecode, CryptFindOIDInfo);

// @exports CertCompareCertificateName CertCompareIntegerBlob CertGetCertificateContextProperty CertStrToNameA CertStrToNameW CertVerifySubjectCertificateContext CryptBinaryToStringA CryptBinaryToStringW CryptDecodeObject CryptDecodeObjectEx CryptImportPublicKeyInfo CryptMsgUpdate CryptProtectData CryptUnprotectData CryptVerifyMessageSignature
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    CertCompareCertificateName, CertCompareIntegerBlob,
    CertGetCertificateContextProperty,
    CertStrToNameA, CertStrToNameW,
    CertVerifySubjectCertificateContext, CryptBinaryToStringA,
    CryptBinaryToStringW, CryptDecodeObject, CryptDecodeObjectEx,
    CryptImportPublicKeyInfo, CryptMsgUpdate,
    CryptProtectData, CryptUnprotectData,
    CryptVerifyMessageSignature);

// ---------------------------------------------------------------------------
// Internal registries for certificates, memory stores, chain engines, chains
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub(crate) struct PeerCertVerifyResult {
    /// Windows `CERT_E_*` status from TLS handshake WebPKI verification (`0` = valid).
    pub cert_error: u32,
    /// Additional trust anchors supplied via `SCHANNEL_CRED.hRootStore`.
    pub extra_roots_der: Vec<Vec<u8>>,
}

struct RegisteredCert {
    owned: OwnedCertContext,
    refcount: usize,
    /// Full peer certificate chain DER (`[leaf, intermediate1, ...]`) when created from TLS.
    peer_chain_der: Vec<Vec<u8>>,
    peer_verify: Option<PeerCertVerifyResult>,
    /// Memory store owned by this peer cert (released when refcount hits 0).
    owned_store: usize,
}

struct RegisteredStore {
    refcount: usize,
    certs: Vec<usize>,
}

struct RegisteredEngine {
    exclusive_roots_der: Option<Vec<Vec<u8>>>,
    additional_certs_der: Vec<Vec<u8>>,
}

struct RegisteredChain {
    _certs: Vec<OwnedCertContext>,
    _elements: Vec<Box<CertChainElement>>,
    _element_ptrs: Box<[*const CertChainElement]>,
    _simple_chain: Box<CertSimpleChain>,
    _simple_chain_ptrs: Box<[*const CertSimpleChain]>,
    _chain_context: Box<CertChainContext>,
    chain_der: Vec<Vec<u8>>,
    element_times: Vec<(u64, u64)>,
    verify_filetime: Option<u64>,
    exclusive_roots_der: Option<Vec<Vec<u8>>>,
    extra_roots_der: Vec<Vec<u8>>,
    default_chain_error: u32,
}

unsafe impl Send for RegisteredChain {}

#[derive(Default)]
struct CryptState {
    certs: HashMap<usize, RegisteredCert>,
    /// Map `SubjectPublicKeyInfo` field address -> cert handle for exact DER SPKI lookup.
    spki_to_cert: HashMap<usize, usize>,
    stores: HashMap<usize, RegisteredStore>,
    engines: HashMap<usize, RegisteredEngine>,
    chains: HashMap<usize, RegisteredChain>,
    msgs: HashMap<usize, ParsedPkcs7Info>,
    next_store_id: usize,
    next_engine_id: usize,
    next_msg_id: usize,
}

static CRYPT_STATE: LazyLock<Mutex<CryptState>> = LazyLock::new(|| {
    Mutex::new(CryptState {
        next_store_id: 0x5100_0001,
        next_engine_id: 0x5200_0001,
        next_msg_id: 0x5300_0001,
        ..Default::default()
    })
});

static DEFAULT_WEB_ROOTS: LazyLock<Arc<rustls::RootCertStore>> = LazyLock::new(|| {
    let mut store = rustls::RootCertStore::empty();
    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    Arc::new(store)
});

/// Synthesize a valid self-signed DER X.509 v3 CA certificate from a `webpki-roots`
/// `TrustAnchor` (preserving its exact Subject DN, SubjectPublicKeyInfo, and any
/// NameConstraints so `CertEnumCertificatesInStore`, `CertFindCertificateInStore`,
/// `CertGetNameString`, `CryptEncodeObjectEx(SPKI)`, and `rustls::RootCertStore::add`
/// all work faithfully).
fn synthesize_root_cert_der(ta: &rustls::pki_types::TrustAnchor<'_>) -> Vec<u8> {
    let subject_der = encode_der_tlv(0x30, ta.subject.as_ref());
    let spki_der = encode_der_tlv(0x30, ta.subject_public_key_info.as_ref());

    // Choose a matching signature algorithm OID (ECDSA-SHA256 vs RSA-SHA256).
    const OID_EC_PUBKEY: &[u8] = &[0x06, 0x07, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01];
    let is_ec = ta
        .subject_public_key_info
        .as_ref()
        .windows(OID_EC_PUBKEY.len())
        .any(|w| w == OID_EC_PUBKEY);
    let sig_alg = if is_ec {
        encode_der_tlv(0x30, &encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02]))
    } else {
        let mut inner = encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B]);
        inner.extend_from_slice(&encode_der_tlv(0x05, &[]));
        encode_der_tlv(0x30, &inner)
    };

    // Deterministic canonical positive 16-byte serial number from SHA-256(subject || spki).
    let hash = super::bcrypt::Digest::Sha256.hash_slices(&[&subject_der, &spki_der]);
    let mut serial = [0u8; 16];
    serial.copy_from_slice(&hash[..16]);
    serial[0] = (serial[0] & 0x7F) | 0x01;
    let serial_der = encode_der_tlv(0x02, &serial);

    let version_der = encode_der_tlv(0xA0, &encode_der_tlv(0x02, &[0x02])); // v3
    let validity_der = {
        let mut v = encode_der_tlv(0x17, b"200101000000Z");
        v.extend_from_slice(&encode_der_tlv(0x18, b"20991231235959Z"));
        encode_der_tlv(0x30, &v)
    };

    // Extensions: BasicConstraints(critical, cA=TRUE), KeyUsage(critical, keyCertSign|cRLSign),
    // and optional NameConstraints if present on the TrustAnchor.
    let mut exts_inner = Vec::new();
    {
        let bc_val = encode_der_tlv(0x30, &encode_der_tlv(0x01, &[0xFF]));
        let mut e = encode_der_tlv(0x06, &[0x55, 0x1D, 0x13]);
        e.extend_from_slice(&encode_der_tlv(0x01, &[0xFF]));
        e.extend_from_slice(&encode_der_tlv(0x04, &bc_val));
        exts_inner.extend_from_slice(&encode_der_tlv(0x30, &e));
    }
    {
        let ku_val = encode_der_tlv(0x03, &[0x01, 0x06]);
        let mut e = encode_der_tlv(0x06, &[0x55, 0x1D, 0x0F]);
        e.extend_from_slice(&encode_der_tlv(0x01, &[0xFF]));
        e.extend_from_slice(&encode_der_tlv(0x04, &ku_val));
        exts_inner.extend_from_slice(&encode_der_tlv(0x30, &e));
    }
    if let Some(ref nc) = ta.name_constraints {
        let nc_val = encode_der_tlv(0x30, nc.as_ref());
        let mut e = encode_der_tlv(0x06, &[0x55, 0x1D, 0x1E]);
        e.extend_from_slice(&encode_der_tlv(0x01, &[0xFF]));
        e.extend_from_slice(&encode_der_tlv(0x04, &nc_val));
        exts_inner.extend_from_slice(&encode_der_tlv(0x30, &e));
    }
    let extensions_der = encode_der_tlv(0xA3, &encode_der_tlv(0x30, &exts_inner));

    let mut tbs_inner = Vec::new();
    tbs_inner.extend_from_slice(&version_der);
    tbs_inner.extend_from_slice(&serial_der);
    tbs_inner.extend_from_slice(&sig_alg);
    tbs_inner.extend_from_slice(&subject_der); // issuer == subject (self-signed root)
    tbs_inner.extend_from_slice(&validity_der);
    tbs_inner.extend_from_slice(&subject_der);
    tbs_inner.extend_from_slice(&spki_der);
    tbs_inner.extend_from_slice(&extensions_der);
    let tbs = encode_der_tlv(0x30, &tbs_inner);

    let mut sig_bits = Vec::with_capacity(1 + 32);
    sig_bits.push(0x00);
    sig_bits.extend_from_slice(&hash);

    let mut cert_inner = Vec::with_capacity(tbs.len() + sig_alg.len() + sig_bits.len() + 8);
    cert_inner.extend_from_slice(&tbs);
    cert_inner.extend_from_slice(&sig_alg);
    cert_inner.extend_from_slice(&encode_der_tlv(0x03, &sig_bits));
    encode_der_tlv(0x30, &cert_inner)
}

static WEBPKI_ROOT_CERTS_DER: LazyLock<Vec<Vec<u8>>> = LazyLock::new(|| {
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .map(synthesize_root_cert_der)
        .collect()
});

// ---------------------------------------------------------------------------
// Minimal DER parser & X.509 time / extension / hostname / signature helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct DerItem<'a> {
    tag: u8,
    raw: &'a [u8],
    val: &'a [u8],
}

fn parse_der_item(buf: &[u8]) -> Option<(DerItem<'_>, &[u8])> {
    if buf.len() < 2 {
        return None;
    }
    let tag = buf[0];
    let lb = buf[1];
    let (hdr, len) = if (lb & 0x80) == 0 {
        (2usize, lb as usize)
    } else {
        let n = (lb & 0x7F) as usize;
        if n == 0 || n > 4 || buf.len() < 2 + n {
            return None;
        }
        let mut v = 0usize;
        for &b in &buf[2..2 + n] {
            v = (v << 8) | (b as usize);
        }
        (2 + n, v)
    };
    let end = hdr.checked_add(len)?;
    if end > buf.len() {
        return None;
    }
    Some((DerItem { tag, raw: &buf[..end], val: &buf[hdr..end] }, &buf[end..]))
}

fn der_item_children(mut buf: &[u8]) -> Option<Vec<DerItem<'_>>> {
    let mut out = Vec::new();
    while !buf.is_empty() {
        let (it, rest) = parse_der_item(buf)?;
        out.push(it);
        buf = rest;
    }
    Some(out)
}

fn now_filetime() -> u64 {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    (dur.as_secs().saturating_add(11_644_473_600))
        .saturating_mul(10_000_000)
        .saturating_add((dur.subsec_nanos() / 100) as u64)
}

fn filetime_to_unix_time(ft: u64) -> Option<UnixTime> {
    const EPOCH_DIFF_100NS: u64 = 11_644_473_600u64 * 10_000_000;
    let unix_100ns = ft.checked_sub(EPOCH_DIFF_100NS)?;
    let secs = unix_100ns / 10_000_000;
    let nanos = ((unix_100ns % 10_000_000) * 100) as u32;
    Some(UnixTime::since_unix_epoch(std::time::Duration::new(secs, nanos)))
}

struct CertDerParts<'a> {
    tbs_raw: &'a [u8],
    sig_alg_inner: &'a [u8],
    sig_bits: &'a [u8],
    issuer_inner: &'a [u8],
    subject_inner: &'a [u8],
    subject_raw: &'a [u8],
    spki_inner: &'a [u8],
    spki_alg_inner: &'a [u8],
    spki_key_bits: &'a [u8],
    extensions: Vec<(&'a [u8], bool, &'a [u8])>,
}

fn parse_cert_der_parts(der: &[u8]) -> Option<CertDerParts<'_>> {
    let (cert_seq, _) = parse_der_item(der)?;
    if cert_seq.tag != 0x30 {
        return None;
    }
    let top = der_item_children(cert_seq.val)?;
    if top.len() < 3 || top[0].tag != 0x30 || top[1].tag != 0x30 || top[2].tag != 0x03 || top[2].val.is_empty() {
        return None;
    }
    let tbs = top[0];
    let sig_alg_inner = top[1].val;
    let sig_bits = &top[2].val[1..];

    let kids = der_item_children(tbs.val)?;
    let mut idx = 0usize;
    if kids.get(idx)?.tag == 0xA0 {
        idx += 1;
    }
    let issuer = *kids.get(idx + 2)?;
    let subject = *kids.get(idx + 4)?;
    let spki = *kids.get(idx + 5)?;
    if issuer.tag != 0x30 || subject.tag != 0x30 || spki.tag != 0x30 {
        return None;
    }
    let spki_kids = der_item_children(spki.val)?;
    if spki_kids.len() < 2 || spki_kids[0].tag != 0x30 || spki_kids[1].tag != 0x03 || spki_kids[1].val.is_empty() {
        return None;
    }

    let mut extensions = Vec::new();
    for item in kids.iter().skip(idx + 6) {
        if item.tag == 0xA3 {
            if let Some((ext_seq, _)) = parse_der_item(item.val) {
                if ext_seq.tag == 0x30 {
                    if let Some(ext_list) = der_item_children(ext_seq.val) {
                        for ext in ext_list {
                            if ext.tag != 0x30 {
                                continue;
                            }
                            let Some(ek) = der_item_children(ext.val) else { continue };
                            if ek.len() >= 2 && ek[0].tag == 0x06 {
                                let (crit, val_item) = if ek.len() >= 3 && ek[1].tag == 0x01 {
                                    (ek[1].val.first().copied().unwrap_or(0) != 0, ek[2])
                                } else {
                                    (false, ek[1])
                                };
                                if val_item.tag == 0x04 {
                                    extensions.push((ek[0].val, crit, val_item.val));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Some(CertDerParts {
        tbs_raw: tbs.raw,
        sig_alg_inner,
        sig_bits,
        issuer_inner: issuer.val,
        subject_inner: subject.val,
        subject_raw: subject.raw,
        spki_inner: spki.val,
        spki_alg_inner: spki_kids[0].val,
        spki_key_bits: &spki_kids[1].val[1..],
        extensions,
    })
}

fn dns_pattern_matches(pattern: &str, host: &str) -> bool {
    let pat = pattern.trim_end_matches('.').to_ascii_lowercase();
    let h = host.trim_end_matches('.').to_ascii_lowercase();
    if pat.is_empty() || h.is_empty() {
        return false;
    }
    if pat == h {
        return true;
    }
    // RFC 6125 leftmost single-label wildcard (*.example.com)
    if let Some(suffix) = pat.strip_prefix("*.") {
        if suffix.contains('*') || !suffix.contains('.') {
            return false;
        }
        if let Some(prefix) = h.strip_suffix(suffix).and_then(|p| p.strip_suffix('.')) {
            return !prefix.is_empty() && !prefix.contains('.');
        }
    }
    false
}

/// Check whether `leaf_der` matches `server_name` (DNS or IP) per RFC 6125:
/// checks SAN (`2.5.29.17`) `dNSName` / `iPAddress`, falling back to Subject CN
/// only when no SAN extension is present.
fn cert_matches_hostname(leaf_der: &[u8], server_name: &str) -> bool {
    let Some(parts) = parse_cert_der_parts(leaf_der) else {
        return false;
    };
    let clean = server_name.trim().trim_start_matches('[').trim_end_matches(']');
    if clean.is_empty() {
        return false;
    }
    let ip_query: Option<std::net::IpAddr> = clean.parse().ok();

    const OID_SAN: &[u8] = &[0x55, 0x1D, 0x11];
    let mut saw_san = false;
    for (oid, _, val) in &parts.extensions {
        if *oid != OID_SAN {
            continue;
        }
        saw_san = true;
        let Some((seq, _)) = parse_der_item(val) else { continue };
        if seq.tag != 0x30 {
            continue;
        }
        let Some(entries) = der_item_children(seq.val) else { continue };
        for entry in entries {
            match entry.tag {
                0x82 if ip_query.is_none() => {
                    if let Ok(dns) = std::str::from_utf8(entry.val) {
                        if dns_pattern_matches(dns, clean) {
                            return true;
                        }
                    }
                }
                0x87 => {
                    if let Some(ip) = ip_query {
                        let matched = match (ip, entry.val.len()) {
                            (std::net::IpAddr::V4(v4), 4) => v4.octets() == entry.val,
                            (std::net::IpAddr::V6(v6), 16) => v6.octets() == entry.val,
                            _ => false,
                        };
                        if matched {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if saw_san {
        return false;
    }
    // Fallback to Subject CN when no SAN extension exists.
    if let Some(cn) = wintrust::extract_dn_attr(parts.subject_raw, &[0x55, 0x04, 0x03]) {
        if ip_query.is_some() {
            return cn.trim().eq_ignore_ascii_case(clean);
        }
        return dns_pattern_matches(&cn, clean);
    }
    false
}

/// Extract EKU OIDs from extension `2.5.29.37`, or `None` if the cert has no EKU extension.
fn cert_eku_oids(der: &[u8]) -> Option<Vec<Vec<u8>>> {
    let parts = parse_cert_der_parts(der)?;
    const OID_EKU: &[u8] = &[0x55, 0x1D, 0x25];
    for (oid, _, val) in &parts.extensions {
        if *oid != OID_EKU {
            continue;
        }
        let (seq, _) = parse_der_item(val)?;
        if seq.tag != 0x30 {
            return Some(Vec::new());
        }
        let kids = der_item_children(seq.val)?;
        let oids = kids.into_iter().filter(|k| k.tag == 0x06).map(|k| k.val.to_vec()).collect();
        return Some(oids);
    }
    None
}

fn verify_cert_sig_with_spki(child: &CertDerParts<'_>, spki_alg_inner: &[u8], spki_key_bits: &[u8]) -> bool {
    let provider = rustls_crypto_provider();
    for alg in provider.signature_verification_algorithms.all {
        if alg.public_key_alg_id().as_ref() == spki_alg_inner
            && alg.signature_alg_id().as_ref() == child.sig_alg_inner
            && alg.verify_signature(spki_key_bits, child.tbs_raw, child.sig_bits).is_ok()
        {
            return true;
        }
    }
    false
}

/// Verify cryptographic chain signatures from `chain_der[0]` up to a trusted root
/// without requiring TLS `serverAuth` EKU (used by `CERT_CHAIN_POLICY_BASE` and
/// `CERT_CHAIN_POLICY_AUTHENTICODE`).
fn verify_chain_signatures_to_roots(
    chain_der: &[Vec<u8>],
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
) -> bool {
    if chain_der.is_empty() {
        return false;
    }
    // Collect candidate trust anchors as (subject_inner, spki_inner, spki_alg_inner, spki_key_bits).
    let mut root_parts: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = Vec::new();
    let push_root_der = |dst: &mut Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>, der: &[u8]| {
        if let Some(p) = parse_cert_der_parts(der) {
            dst.push((
                p.subject_inner.to_vec(),
                p.spki_inner.to_vec(),
                p.spki_alg_inner.to_vec(),
                p.spki_key_bits.to_vec(),
            ));
        }
    };
    if let Some(excl) = exclusive_roots_der {
        for der in excl {
            push_root_der(&mut root_parts, der);
        }
    } else {
        for der in extra_roots_der {
            push_root_der(&mut root_parts, der);
        }
        for ta in webpki_roots::TLS_SERVER_ROOTS {
            if let Ok(kids) = der_item_children(ta.subject_public_key_info.as_ref()).ok_or(()) {
                if kids.len() >= 2 && kids[0].tag == 0x30 && kids[1].tag == 0x03 && !kids[1].val.is_empty() {
                    root_parts.push((
                        ta.subject.as_ref().to_vec(),
                        ta.subject_public_key_info.as_ref().to_vec(),
                        kids[0].val.to_vec(),
                        kids[1].val[1..].to_vec(),
                    ));
                }
            }
        }
    }
    if root_parts.is_empty() {
        return false;
    }

    let parsed: Vec<Option<CertDerParts<'_>>> = chain_der.iter().map(|d| parse_cert_der_parts(d)).collect();
    let Some(mut cur) = parsed[0].as_ref() else {
        return false;
    };

    // Walk up to 16 chain links looking for a trusted root.
    let mut visited = vec![false; chain_der.len()];
    visited[0] = true;
    for _ in 0..16 {
        // Direct match: cur itself is a trusted root (same subject + SPKI).
        if root_parts
            .iter()
            .any(|(subj, spki, _, _)| subj.as_slice() == cur.subject_inner && spki.as_slice() == cur.spki_inner)
        {
            return true;
        }
        // Signed directly by a trusted root?
        if root_parts.iter().any(|(subj, _, alg, key)| {
            subj.as_slice() == cur.issuer_inner && verify_cert_sig_with_spki(cur, alg, key)
        }) {
            return true;
        }
        // Otherwise look for an intermediate issuer in chain_der.
        let mut next_idx = None;
        for (i, cand) in parsed.iter().enumerate() {
            if visited[i] {
                continue;
            }
            if let Some(p) = cand.as_ref() {
                if p.subject_inner == cur.issuer_inner
                    && verify_cert_sig_with_spki(cur, p.spki_alg_inner, p.spki_key_bits)
                {
                    next_idx = Some(i);
                    break;
                }
            }
        }
        let Some(ni) = next_idx else {
            return false;
        };
        visited[ni] = true;
        cur = parsed[ni].as_ref().unwrap();
    }
    false
}

/// Extract all RDN attribute strings from a DER `Name` (handles every OID, not just CN/O/OU/C/L/S/E).
fn all_dn_values_lower(dn_der: &[u8]) -> Vec<String> {
    let mut vals = Vec::new();
    let Some((seq, _)) = parse_der_item(dn_der) else { return vals };
    if seq.tag != 0x30 {
        return vals;
    }
    let Some(rdns) = der_item_children(seq.val) else { return vals };
    for rdn in rdns {
        if rdn.tag != 0x31 {
            continue;
        }
        let Some(atvs) = der_item_children(rdn.val) else { continue };
        for atv in atvs {
            if atv.tag != 0x30 {
                continue;
            }
            let Some(kids) = der_item_children(atv.val) else { continue };
            if kids.len() >= 2 {
                let s = match kids[1].tag {
                    0x0C | 0x13 | 0x16 | 0x14 | 0x12 => Some(String::from_utf8_lossy(kids[1].val).into_owned()),
                    0x1E if kids[1].val.len() % 2 == 0 => {
                        let units: Vec<u16> = kids[1]
                            .val
                            .chunks_exact(2)
                            .map(|c| u16::from_be_bytes([c[0], c[1]]))
                            .collect();
                        Some(String::from_utf16_lossy(&units))
                    }
                    _ => None,
                };
                if let Some(v) = s {
                    vals.push(v.to_ascii_lowercase());
                }
            }
        }
    }
    vals
}

pub(crate) fn rustls_crypto_provider() -> Arc<rustls::crypto::CryptoProvider> {
    static PROV: LazyLock<Arc<rustls::crypto::CryptoProvider>> =
        LazyLock::new(|| Arc::new(rustls::crypto::ring::default_provider()));
    Arc::clone(&PROV)
}

pub(crate) fn default_root_store() -> Arc<rustls::RootCertStore> {
    Arc::clone(&DEFAULT_WEB_ROOTS)
}

/// Map a `rustls::Error` from `WebPkiServerVerifier::verify_server_cert` to a Windows `CERT_E_*` code.
pub(crate) fn map_rustls_cert_error(err: &rustls::Error, ignore_hostname: bool) -> u32 {
    match err {
        rustls::Error::InvalidCertificate(cert_err) => match cert_err {
            rustls::CertificateError::Expired
            | rustls::CertificateError::NotValidYet
            | rustls::CertificateError::ExpiredContext { .. }
            | rustls::CertificateError::NotValidYetContext { .. } => CERT_E_EXPIRED,
            rustls::CertificateError::NotValidForName
            | rustls::CertificateError::NotValidForNameContext { .. } => {
                if ignore_hostname {
                    0
                } else {
                    CERT_E_CN_NO_MATCH
                }
            }
            rustls::CertificateError::Revoked => CERT_E_REVOKED,
            rustls::CertificateError::InvalidPurpose
            | rustls::CertificateError::InvalidPurposeContext { .. } => CERT_E_WRONG_USAGE,
            _ => CERT_E_UNTRUSTEDROOT,
        },
        _ => CERT_E_CHAINING,
    }
}

fn cert_error_to_trust_status(cert_error: u32) -> u32 {
    match cert_error {
        0 => 0,
        CERT_E_EXPIRED => CERT_TRUST_IS_NOT_TIME_VALID,
        CERT_E_REVOKED => CERT_TRUST_IS_REVOKED,
        CERT_E_UNTRUSTEDROOT => CERT_TRUST_IS_UNTRUSTED_ROOT,
        CERT_E_WRONG_USAGE => CERT_TRUST_IS_NOT_VALID_FOR_USAGE,
        CERT_E_CHAINING => CERT_TRUST_IS_PARTIAL_CHAIN,
        // Hostname mismatch is evaluated at SSL policy time, not raw chain status.
        CERT_E_CN_NO_MATCH => 0,
        _ => CERT_TRUST_IS_UNTRUSTED_ROOT,
    }
}

/// Verify an X.509 certificate chain (`[leaf, intermediates...]`) using `rustls`'s WebPKI
/// verifier against either an exclusive root store or the embedded `webpki-roots` bundle.
/// Returns `0` on success or a Windows `CERT_E_*` error code on failure.
fn verify_chain_webpki_at(
    chain_der: &[Vec<u8>],
    server_name: Option<&str>,
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
    ignore_hostname: bool,
    at_time: UnixTime,
) -> u32 {
    let Some((leaf, intermediates)) = chain_der.split_first() else {
        return CERT_E_CHAINING;
    };
    let roots = if let Some(excl) = exclusive_roots_der {
        let mut s = rustls::RootCertStore::empty();
        for der in excl {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    } else if extra_roots_der.is_empty() {
        default_root_store()
    } else {
        let mut s = (*default_root_store()).clone();
        for der in extra_roots_der {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    };
    if roots.is_empty() {
        return CERT_E_UNTRUSTEDROOT;
    }
    let verifier = match WebPkiServerVerifier::builder_with_provider(roots, rustls_crypto_provider()).build() {
        Ok(v) => v,
        Err(_) => return CERT_E_UNTRUSTEDROOT,
    };

    let (sname, effective_ignore_host) = match server_name.map(str::trim).filter(|s| !s.is_empty()) {
        Some(host) => {
            let clean = host.trim_start_matches('[').trim_end_matches(']');
            match ServerName::try_from(clean.to_string()) {
                Ok(sn) => (sn, ignore_hostname),
                Err(_) => {
                    if ignore_hostname {
                        (ServerName::try_from("localhost".to_string()).unwrap(), true)
                    } else {
                        return CERT_E_CN_NO_MATCH;
                    }
                }
            }
        }
        None => (ServerName::try_from("localhost".to_string()).unwrap(), true),
    };

    let end_entity = CertificateDer::from(leaf.as_slice());
    let inters: Vec<CertificateDer<'_>> = intermediates.iter().map(|d| CertificateDer::from(d.as_slice())).collect();
    match verifier.verify_server_cert(&end_entity, &inters, &sname, &[], at_time) {
        Ok(_) => 0,
        Err(ref e) => map_rustls_cert_error(e, effective_ignore_host),
    }
}

fn insert_cert_locked(
    st: &mut CryptState,
    owned: OwnedCertContext,
    peer_chain_der: Vec<Vec<u8>>,
    peer_verify: Option<PeerCertVerifyResult>,
    owned_store: usize,
) -> usize {
    let ptr = owned.as_mut_ptr() as usize;
    let spki = owned.spki_ptr();
    st.spki_to_cert.insert(spki, ptr);
    st.certs.insert(
        ptr,
        RegisteredCert {
            owned,
            refcount: 1,
            peer_chain_der,
            peer_verify,
            owned_store,
        },
    );
    ptr
}

fn release_cert_locked(st: &mut CryptState, handle: usize) -> bool {
    let Some(entry) = st.certs.get_mut(&handle) else {
        return false;
    };
    if entry.refcount > 1 {
        entry.refcount -= 1;
        return true;
    }
    let removed = st.certs.remove(&handle).unwrap();
    st.spki_to_cert.remove(&removed.owned.spki_ptr());
    if removed.owned_store != 0 {
        release_store_locked(st, removed.owned_store);
    }
    true
}

fn release_store_locked(st: &mut CryptState, handle: usize) -> bool {
    let Some(store) = st.stores.get_mut(&handle) else {
        return false;
    };
    if store.refcount > 1 {
        store.refcount -= 1;
        return true;
    }
    let removed = st.stores.remove(&handle).unwrap();
    for cert_h in removed.certs {
        release_cert_locked(st, cert_h);
    }
    true
}

/// Register a TLS peer's certificate chain and return a newly referenced leaf `PCCERT_CONTEXT`
/// (`CertFreeCertificateContext` releases it). Its `hCertStore` points to an in-memory store
/// containing all certificates presented by the peer.
pub(crate) fn register_peer_cert_chain(
    chain_der: &[Vec<u8>],
    verify: PeerCertVerifyResult,
) -> *mut c_void {
    let Some(leaf_der) = chain_der.first() else {
        return std::ptr::null_mut();
    };
    let mut st = CRYPT_STATE.lock().unwrap();
    let store_handle = st.next_store_id;
    st.next_store_id += 1;

    let mut store_certs = Vec::with_capacity(chain_der.len());
    for der in chain_der {
        if let Some(owned) = wintrust::build_owned_cert_context(der, store_handle as *mut c_void) {
            let h = insert_cert_locked(&mut st, owned, Vec::new(), None, 0);
            store_certs.push(h);
        }
    }
    st.stores.insert(store_handle, RegisteredStore { refcount: 1, certs: store_certs });

    let Some(leaf_owned) = wintrust::build_owned_cert_context(leaf_der, store_handle as *mut c_void) else {
        release_store_locked(&mut st, store_handle);
        return std::ptr::null_mut();
    };
    let leaf_handle = insert_cert_locked(
        &mut st,
        leaf_owned,
        chain_der.to_vec(),
        Some(verify),
        store_handle,
    );
    leaf_handle as *mut c_void
}

/// Collect DER bytes of all certificates in a registered `HCERTSTORE`.
pub(crate) fn store_certs_der(h_cert_store: *mut c_void) -> Vec<Vec<u8>> {
    if h_cert_store.is_null() {
        return Vec::new();
    }
    let st = CRYPT_STATE.lock().unwrap();
    let Some(store) = st.stores.get(&(h_cert_store as usize)) else {
        return Vec::new();
    };
    store
        .certs
        .iter()
        .filter_map(|h| st.certs.get(h).map(|c| c.owned.raw_der().to_vec()))
        .collect()
}

// ---------------------------------------------------------------------------
// Certificate Contexts & Stores
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CertCreateCertificateContext(
    dw_cert_encoding_type: u32,
    pb_cert_encoded: *const u8,
    cb_cert_encoded: u32,
) -> *mut c_void {
    if (dw_cert_encoding_type & X509_ASN_ENCODING) == 0 || pb_cert_encoded.is_null() || cb_cert_encoded == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let der = std::slice::from_raw_parts(pb_cert_encoded, cb_cert_encoded as usize);
    let Some(owned) = wintrust::build_owned_cert_context(der, std::ptr::null_mut()) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    let mut st = CRYPT_STATE.lock().unwrap();
    insert_cert_locked(&mut st, owned, vec![der.to_vec()], None, 0) as *mut c_void
}

pub unsafe extern "win64" fn CertDuplicateCertificateContext(ctx: *mut c_void) -> *mut c_void {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let mut st = CRYPT_STATE.lock().unwrap();
    if let Some(entry) = st.certs.get_mut(&(ctx as usize)) {
        entry.refcount += 1;
    }
    ctx
}

pub unsafe extern "win64" fn CertFreeCertificateContext(ctx: *mut c_void) -> BOOL {
    if ctx.is_null() {
        return TRUE;
    }
    let mut st = CRYPT_STATE.lock().unwrap();
    if !release_cert_locked(&mut st, ctx as usize) {
        stub!("CertFreeCertificateContext({ctx:p}): foreign context ignored");
    }
    TRUE
}

fn open_named_system_store(store_name: &str) -> *mut c_void {
    let upper = store_name.trim().to_ascii_uppercase();
    let seed_roots = matches!(upper.as_str(), "ROOT" | "AUTHROOT" | "CA");
    let mut st = CRYPT_STATE.lock().unwrap();
    let id = st.next_store_id;
    st.next_store_id += 1;
    let mut certs = Vec::new();
    if seed_roots {
        for der in WEBPKI_ROOT_CERTS_DER.iter() {
            if let Some(owned) = wintrust::build_owned_cert_context(der, id as *mut c_void) {
                let h = insert_cert_locked(&mut st, owned, vec![der.clone()], None, 0);
                certs.push(h);
            }
        }
    }
    st.stores.insert(id, RegisteredStore { refcount: 1, certs });
    log::debug!("CertOpenStore(system={store_name:?}) -> {id:#x} (seeded={seed_roots})");
    id as *mut c_void
}

pub unsafe extern "win64" fn CertOpenStore(
    lpsz_store_provider: *const u8,
    _dw_encoding_type: u32,
    _h_crypt_prov: usize,
    _dw_flags: u32,
    pv_para: *const c_void,
) -> *mut c_void {
    let prov = lpsz_store_provider as usize;
    let prov_str = if prov > 0xFFFF {
        Some(crate::strings::from_ansi(lpsz_store_provider))
    } else {
        None
    };

    let is_memory = prov == CERT_STORE_PROV_MEMORY
        || prov_str.as_deref().is_some_and(|s| s.eq_ignore_ascii_case("Memory"));
    if is_memory {
        let mut st = CRYPT_STATE.lock().unwrap();
        let id = st.next_store_id;
        st.next_store_id += 1;
        st.stores.insert(id, RegisteredStore { refcount: 1, certs: Vec::new() });
        return id as *mut c_void;
    }

    let is_system_a = prov == CERT_STORE_PROV_SYSTEM_A
        || prov == CERT_STORE_PROV_SYSTEM_REGISTRY_A
        || prov_str
            .as_deref()
            .is_some_and(|s| s.eq_ignore_ascii_case("SystemA") || s.eq_ignore_ascii_case("SystemRegistryA"));
    if is_system_a {
        if pv_para.is_null() {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return std::ptr::null_mut();
        }
        let name = crate::strings::from_ansi(pv_para as *const u8);
        return open_named_system_store(&name);
    }

    let is_system_w = prov == CERT_STORE_PROV_SYSTEM_W
        || prov == CERT_STORE_PROV_SYSTEM_REGISTRY_W
        || prov_str.as_deref().is_some_and(|s| {
            s.eq_ignore_ascii_case("System")
                || s.eq_ignore_ascii_case("SystemW")
                || s.eq_ignore_ascii_case("SystemRegistry")
                || s.eq_ignore_ascii_case("SystemRegistryW")
        });
    if is_system_w {
        if pv_para.is_null() {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return std::ptr::null_mut();
        }
        let name = crate::strings::from_wide(pv_para as *const u16);
        return open_named_system_store(&name);
    }

    stub!("CertOpenStore(provider={prov:#x}) -> NULL");
    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn CertOpenSystemStoreA(
    h_prov: usize,
    sz_subsystem_protocol: LPCSTR,
) -> *mut c_void {
    if sz_subsystem_protocol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    CertOpenStore(
        CERT_STORE_PROV_SYSTEM_A as *const u8,
        X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
        h_prov,
        CERT_SYSTEM_STORE_CURRENT_USER,
        sz_subsystem_protocol as *const c_void,
    )
}

pub unsafe extern "win64" fn CertOpenSystemStoreW(
    h_prov: usize,
    sz_subsystem_protocol: LPCWSTR,
) -> *mut c_void {
    if sz_subsystem_protocol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    CertOpenStore(
        CERT_STORE_PROV_SYSTEM_W as *const u8,
        X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
        h_prov,
        CERT_SYSTEM_STORE_CURRENT_USER,
        sz_subsystem_protocol as *const c_void,
    )
}

pub unsafe extern "win64" fn CertDeleteCertificateFromStore(p_cert_context: *mut c_void) -> BOOL {
    if p_cert_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cc = std::ptr::read_unaligned(p_cert_context as *const CertContext);
    let cert_h = p_cert_context as usize;
    let store_h = cc.h_cert_store as usize;
    let mut st = CRYPT_STATE.lock().unwrap();
    let mut removed_from_store = false;
    if store_h != 0 {
        if let Some(store) = st.stores.get_mut(&store_h) {
            if let Some(pos) = store.certs.iter().position(|&h| h == cert_h) {
                store.certs.remove(pos);
                // Release the store's own reference to this cert.
                release_cert_locked(&mut st, cert_h);
                removed_from_store = true;
            }
        }
    }
    // Windows CertDeleteCertificateFromStore always frees the caller's context reference.
    release_cert_locked(&mut st, cert_h);
    if removed_from_store {
        TRUE
    } else {
        crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
        FALSE
    }
}

pub unsafe extern "win64" fn CertDuplicateStore(store: *mut c_void) -> *mut c_void {
    if store.is_null() {
        return std::ptr::null_mut();
    }
    let mut st = CRYPT_STATE.lock().unwrap();
    if let Some(s) = st.stores.get_mut(&(store as usize)) {
        s.refcount += 1;
    }
    store
}

pub unsafe extern "win64" fn CertCloseStore(store: *mut c_void, _flags: u32) -> BOOL {
    if store.is_null() {
        return TRUE;
    }
    let mut st = CRYPT_STATE.lock().unwrap();
    if !release_store_locked(&mut st, store as usize) {
        stub!("CertCloseStore({store:p}): foreign store ignored");
    }
    TRUE
}

unsafe fn add_der_to_store(
    h_cert_store: *mut c_void,
    der: &[u8],
    dw_add_disposition: u32,
    pp_store_context: *mut *mut c_void,
) -> BOOL {
    let Some(owned) = wintrust::build_owned_cert_context(der, h_cert_store) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    };
    let mut st = CRYPT_STATE.lock().unwrap();
    let store_key = h_cert_store as usize;
    if !st.stores.contains_key(&store_key) {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }

    let existing_idx = st.stores[&store_key].certs.iter().position(|&h| {
        st.certs.get(&h).is_some_and(|c| c.owned.raw_der() == der)
    });

    if let Some(idx) = existing_idx {
        match dw_add_disposition {
            CERT_STORE_ADD_NEW => {
                crate::teb::set_last_error(0x8009_2005); // CRYPT_E_EXISTS
                return FALSE;
            }
            CERT_STORE_ADD_USE_EXISTING => {
                let h = st.stores[&store_key].certs[idx];
                if !pp_store_context.is_null() {
                    if let Some(c) = st.certs.get_mut(&h) {
                        c.refcount += 1;
                    }
                    std::ptr::write_unaligned(pp_store_context, h as *mut c_void);
                }
                return TRUE;
            }
            CERT_STORE_ADD_REPLACE_EXISTING => {
                let old_h = st.stores.get_mut(&store_key).unwrap().certs.remove(idx);
                release_cert_locked(&mut st, old_h);
            }
            CERT_STORE_ADD_ALWAYS | _ => {}
        }
    }

    let h = insert_cert_locked(&mut st, owned, vec![der.to_vec()], None, 0);
    st.stores.get_mut(&store_key).unwrap().certs.push(h);
    if !pp_store_context.is_null() {
        st.certs.get_mut(&h).unwrap().refcount += 1;
        std::ptr::write_unaligned(pp_store_context, h as *mut c_void);
    }
    TRUE
}

pub unsafe extern "win64" fn CertAddCertificateContextToStore(
    h_cert_store: *mut c_void,
    p_cert_context: *const c_void,
    dw_add_disposition: u32,
    pp_store_context: *mut *mut c_void,
) -> BOOL {
    if h_cert_store.is_null() || p_cert_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cc = std::ptr::read_unaligned(p_cert_context as *const CertContext);
    if cc.pb_cert_encoded.is_null() || cc.cb_cert_encoded == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    }
    let der = std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize);
    add_der_to_store(h_cert_store, der, dw_add_disposition, pp_store_context)
}

pub unsafe extern "win64" fn CertAddEncodedCertificateToStore(
    h_cert_store: *mut c_void,
    dw_cert_encoding_type: u32,
    pb_cert_encoded: *const u8,
    cb_cert_encoded: u32,
    dw_add_disposition: u32,
    pp_cert_context: *mut *mut c_void,
) -> BOOL {
    if h_cert_store.is_null()
        || (dw_cert_encoding_type & X509_ASN_ENCODING) == 0
        || pb_cert_encoded.is_null()
        || cb_cert_encoded == 0
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let der = std::slice::from_raw_parts(pb_cert_encoded, cb_cert_encoded as usize);
    add_der_to_store(h_cert_store, der, dw_add_disposition, pp_cert_context)
}

pub unsafe extern "win64" fn CertEnumCertificatesInStore(
    h_cert_store: *mut c_void,
    p_prev_cert_context: *mut c_void,
) -> *mut c_void {
    let mut st = CRYPT_STATE.lock().unwrap();
    let Some(certs) = st.stores.get(&(h_cert_store as usize)).map(|s| s.certs.clone()) else {
        if !p_prev_cert_context.is_null() {
            release_cert_locked(&mut st, p_prev_cert_context as usize);
        }
        crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
        return std::ptr::null_mut();
    };
    let next_idx = if p_prev_cert_context.is_null() {
        0
    } else {
        let prev = p_prev_cert_context as usize;
        let pos = certs.iter().position(|&h| h == prev).map(|i| i + 1);
        // Windows CertEnumCertificatesInStore always frees pPrevCertContext.
        release_cert_locked(&mut st, prev);
        match pos {
            Some(i) => i,
            None => {
                crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                return std::ptr::null_mut();
            }
        }
    };
    let Some(&next_h) = certs.get(next_idx) else {
        crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
        return std::ptr::null_mut();
    };
    if let Some(c) = st.certs.get_mut(&next_h) {
        c.refcount += 1;
    }
    next_h as *mut c_void
}

// ---------------------------------------------------------------------------
// Certificate Chain Engine & Policy Verification
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CertCreateCertificateChainEngine(
    p_config: *const c_void,
    ph_chain_engine: *mut *mut c_void,
) -> BOOL {
    if p_config.is_null() || ph_chain_engine.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let base = p_config as *const u8;
    let cb_size = std::ptr::read_unaligned(base as *const u32);
    if cb_size < 0x40 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let h_restricted_root = std::ptr::read_unaligned(base.add(0x08) as *const *mut c_void);
    let c_additional = std::ptr::read_unaligned(base.add(0x20) as *const u32);
    let rgh_additional = std::ptr::read_unaligned(base.add(0x28) as *const *const *mut c_void);
    let h_exclusive_root = if cb_size >= 0x48 {
        std::ptr::read_unaligned(base.add(0x40) as *const *mut c_void)
    } else {
        std::ptr::null_mut()
    };

    let exclusive_roots_der = if !h_exclusive_root.is_null() {
        Some(store_certs_der(h_exclusive_root))
    } else if !h_restricted_root.is_null() {
        Some(store_certs_der(h_restricted_root))
    } else {
        None
    };

    let mut additional_certs_der = Vec::new();
    if c_additional > 0 && !rgh_additional.is_null() {
        for i in 0..c_additional as usize {
            let store_h = std::ptr::read_unaligned(rgh_additional.add(i));
            additional_certs_der.extend(store_certs_der(store_h));
        }
    }

    let mut st = CRYPT_STATE.lock().unwrap();
    let id = st.next_engine_id;
    st.next_engine_id += 1;
    st.engines.insert(
        id,
        RegisteredEngine {
            exclusive_roots_der,
            additional_certs_der,
        },
    );
    std::ptr::write_unaligned(ph_chain_engine, id as *mut c_void);
    TRUE
}

pub unsafe extern "win64" fn CertFreeCertificateChainEngine(engine: *mut c_void) {
    if engine.is_null() {
        return;
    }
    let mut st = CRYPT_STATE.lock().unwrap();
    if st.engines.remove(&(engine as usize)).is_none() {
        stub!("CertFreeCertificateChainEngine({engine:p}): foreign engine ignored");
    }
}

pub unsafe extern "win64" fn CertGetCertificateChain(
    h_chain_engine: *mut c_void,
    p_cert_context: *const c_void,
    p_time: *const c_void,
    h_additional_store: *mut c_void,
    _p_chain_para: *const c_void,
    _dw_flags: u32,
    _pv_reserved: *mut c_void,
    pp_chain_context: *mut *mut c_void,
) -> BOOL {
    if p_cert_context.is_null() || pp_chain_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let verify_filetime = if p_time.is_null() {
        None
    } else {
        Some(std::ptr::read_unaligned(p_time as *const u64))
    };
    let verify_unix = verify_filetime
        .and_then(filetime_to_unix_time)
        .unwrap_or_else(UnixTime::now);
    let check_ft = verify_filetime.unwrap_or_else(now_filetime);

    let (mut chain_der, peer_verify, exclusive_roots_der, mut extra_roots_der) = {
        let st = CRYPT_STATE.lock().unwrap();
        let (certs, verify) = if let Some(entry) = st.certs.get(&(p_cert_context as usize)) {
            let c = if !entry.peer_chain_der.is_empty() {
                entry.peer_chain_der.clone()
            } else {
                vec![entry.owned.raw_der().to_vec()]
            };
            (c, entry.peer_verify.clone())
        } else {
            let cc = std::ptr::read_unaligned(p_cert_context as *const CertContext);
            if cc.pb_cert_encoded.is_null() || cc.cb_cert_encoded == 0 {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let der = std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize);
            (vec![der.to_vec()], None)
        };

        let (excl, engine_extra) = if !h_chain_engine.is_null() && (h_chain_engine as usize) > 1 {
            if let Some(eng) = st.engines.get(&(h_chain_engine as usize)) {
                (eng.exclusive_roots_der.clone(), eng.additional_certs_der.clone())
            } else {
                (None, Vec::new())
            }
        } else {
            (None, Vec::new())
        };

        let mut extra = verify.as_ref().map(|v| v.extra_roots_der.clone()).unwrap_or_default();
        extra.extend(engine_extra);
        (certs, verify, excl, extra)
    };

    // Include any intermediates from hAdditionalStore (without duplicating leaf or intermediates).
    for der in store_certs_der(h_additional_store) {
        if !chain_der.iter().any(|c| c == &der) {
            chain_der.push(der);
        }
    }

    // Verify chain trust (ignoring hostname here; hostname is checked in CertVerifyCertificateChainPolicy).
    let default_chain_error = if exclusive_roots_der.is_none() {
        if let Some(ref pv) = peer_verify {
            if verify_filetime.is_some() {
                verify_chain_webpki_at(&chain_der, None, None, &extra_roots_der, true, verify_unix)
            } else if pv.cert_error == CERT_E_CN_NO_MATCH {
                0
            } else {
                pv.cert_error
            }
        } else {
            verify_chain_webpki_at(&chain_der, None, None, &extra_roots_der, true, verify_unix)
        }
    } else {
        extra_roots_der.clear();
        verify_chain_webpki_at(&chain_der, None, exclusive_roots_der.as_deref(), &[], true, verify_unix)
    };

    let mut owned_certs = Vec::with_capacity(chain_der.len());
    let mut element_times = Vec::with_capacity(chain_der.len());
    for der in &chain_der {
        if let Some(c) = wintrust::build_owned_cert_context(der, std::ptr::null_mut()) {
            let ci = &*(*c.as_ptr()).p_cert_info;
            element_times.push((ci.not_before, ci.not_after));
            owned_certs.push(c);
        }
    }
    if owned_certs.is_empty() {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    }

    let mut trust_err_bits = cert_error_to_trust_status(default_chain_error);
    let mut elements = Vec::with_capacity(owned_certs.len());
    let mut element_ptrs = Vec::with_capacity(owned_certs.len());
    for (idx, cert) in owned_certs.iter().enumerate() {
        let info_status = if cert.is_self_signed() { 0x0000_0108 } else { 0x0000_0100 };
        let (nb, na) = element_times[idx];
        let time_bad = (nb > 0 && check_ft < nb) || (na > 0 && check_ft > na);
        let mut elem_err = if idx == 0 { trust_err_bits } else { 0 };
        if time_bad {
            elem_err |= CERT_TRUST_IS_NOT_TIME_VALID;
            trust_err_bits |= CERT_TRUST_IS_NOT_TIME_VALID;
        }
        let el = Box::new(CertChainElement {
            cb_size: std::mem::size_of::<CertChainElement>() as u32,
            p_cert_context: cert.as_ptr(),
            trust_status: CertTrustStatus {
                dw_error_status: elem_err,
                dw_info_status: info_status,
            },
            p_revocation_info: std::ptr::null_mut(),
            p_issuance_usage: std::ptr::null_mut(),
            p_application_usage: std::ptr::null_mut(),
            pwsz_extended_error_info: std::ptr::null(),
        });
        element_ptrs.push(&*el as *const CertChainElement);
        elements.push(el);
    }

    let element_ptrs = element_ptrs.into_boxed_slice();
    let simple_chain = Box::new(CertSimpleChain {
        cb_size: std::mem::size_of::<CertSimpleChain>() as u32,
        trust_status: CertTrustStatus {
            dw_error_status: trust_err_bits,
            dw_info_status: 0,
        },
        c_element: elements.len() as u32,
        rgp_element: element_ptrs.as_ptr(),
        p_trust_list_info: std::ptr::null_mut(),
        f_has_revocation_freshness_time: 0,
        dw_revocation_freshness_time: 0,
    });
    let simple_chain_ptrs = vec![&*simple_chain as *const CertSimpleChain].into_boxed_slice();
    let chain_context = Box::new(CertChainContext {
        cb_size: std::mem::size_of::<CertChainContext>() as u32,
        trust_status: CertTrustStatus {
            dw_error_status: trust_err_bits,
            dw_info_status: 0,
        },
        c_chain: 1,
        rgp_chain: simple_chain_ptrs.as_ptr(),
        c_lower_quality_chain_context: 0,
        rgp_lower_quality_chain_context: std::ptr::null_mut(),
        f_has_revocation_freshness_time: 0,
        dw_revocation_freshness_time: 0,
        dw_create_flags: 0,
        chain_id: [0; 16],
    });
    let ctx_ptr = &*chain_context as *const CertChainContext as usize;

    let mut st = CRYPT_STATE.lock().unwrap();
    st.chains.insert(
        ctx_ptr,
        RegisteredChain {
            _certs: owned_certs,
            _elements: elements,
            _element_ptrs: element_ptrs,
            _simple_chain: simple_chain,
            _simple_chain_ptrs: simple_chain_ptrs,
            _chain_context: chain_context,
            chain_der,
            element_times,
            verify_filetime,
            exclusive_roots_der,
            extra_roots_der,
            default_chain_error,
        },
    );
    std::ptr::write_unaligned(pp_chain_context, ctx_ptr as *mut c_void);
    TRUE
}

pub unsafe extern "win64" fn CertFreeCertificateChain(chain: *mut c_void) {
    if chain.is_null() {
        return;
    }
    let mut st = CRYPT_STATE.lock().unwrap();
    if st.chains.remove(&(chain as usize)).is_none() {
        stub!("CertFreeCertificateChain({chain:p}): foreign chain ignored");
    }
}

#[repr(C)]
struct CertChainPolicyPara {
    cb_size: u32,
    dw_flags: u32,
    pv_extra_policy_para: *const c_void,
}

#[repr(C)]
struct CertChainPolicyStatus {
    cb_size: u32,
    dw_error: u32,
    l_chain_index: i32,
    l_element_index: i32,
    pv_extra_policy_status: *mut c_void,
}

#[repr(C)]
struct SslExtraCertChainPolicyPara {
    cb_size: u32,
    dw_auth_type: u32,
    fdw_checks: u32,
    pwsz_server_name: *const u16,
}

fn trust_status_to_cert_error(dw_error_status: u32) -> u32 {
    if dw_error_status == 0 {
        0
    } else if (dw_error_status & CERT_TRUST_IS_REVOKED) != 0 {
        CERT_E_REVOKED
    } else if (dw_error_status & CERT_TRUST_IS_NOT_TIME_VALID) != 0 {
        CERT_E_EXPIRED
    } else if (dw_error_status & CERT_TRUST_IS_UNTRUSTED_ROOT) != 0 {
        CERT_E_UNTRUSTEDROOT
    } else if (dw_error_status & CERT_TRUST_IS_NOT_VALID_FOR_USAGE) != 0 {
        CERT_E_WRONG_USAGE
    } else if (dw_error_status & CERT_TRUST_IS_PARTIAL_CHAIN) != 0 {
        CERT_E_CHAINING
    } else {
        CERT_E_UNTRUSTEDROOT
    }
}

unsafe fn extract_external_chain(
    p_chain_context: *const c_void,
) -> Option<(Vec<Vec<u8>>, Vec<(u64, u64)>, u32)> {
    let ctx = std::ptr::read_unaligned(p_chain_context as *const CertChainContext);
    if ctx.c_chain == 0 || ctx.rgp_chain.is_null() {
        return None;
    }
    let simple_ptr = std::ptr::read_unaligned(ctx.rgp_chain);
    if simple_ptr.is_null() {
        return None;
    }
    let simple = std::ptr::read_unaligned(simple_ptr);
    if simple.c_element == 0 || simple.rgp_element.is_null() {
        return None;
    }
    let mut ders = Vec::with_capacity(simple.c_element as usize);
    let mut times = Vec::with_capacity(simple.c_element as usize);
    for i in 0..simple.c_element as usize {
        let el_ptr = std::ptr::read_unaligned(simple.rgp_element.add(i));
        if el_ptr.is_null() {
            continue;
        }
        let el = std::ptr::read_unaligned(el_ptr);
        if el.p_cert_context.is_null() {
            continue;
        }
        let cc = std::ptr::read_unaligned(el.p_cert_context);
        if cc.pb_cert_encoded.is_null() || cc.cb_cert_encoded == 0 {
            continue;
        }
        let der = std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize).to_vec();
        let (nb, na) = if !cc.p_cert_info.is_null() {
            let ci = std::ptr::read_unaligned(cc.p_cert_info);
            (ci.not_before, ci.not_after)
        } else {
            (0, 0)
        };
        ders.push(der);
        times.push((nb, na));
    }
    if ders.is_empty() {
        return None;
    }
    Some((ders, times, trust_status_to_cert_error(ctx.trust_status.dw_error_status)))
}

pub unsafe extern "win64" fn CertVerifyCertificateChainPolicy(
    psz_policy_oid: *const u8,
    p_chain_context: *const c_void,
    p_policy_para: *const c_void,
    p_policy_status: *mut c_void,
) -> BOOL {
    if p_chain_context.is_null() || p_policy_status.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let policy_id = psz_policy_oid as usize;
    if policy_id != CERT_CHAIN_POLICY_BASE
        && policy_id != CERT_CHAIN_POLICY_AUTHENTICODE
        && policy_id != CERT_CHAIN_POLICY_AUTHENTICODE_TS
        && policy_id != CERT_CHAIN_POLICY_SSL
    {
        stub!("CertVerifyCertificateChainPolicy(oid={policy_id:#x}) -> FALSE");
        crate::teb::set_last_error(ERROR_CALL_NOT_IMPLEMENTED);
        return FALSE;
    }

    let (chain_der, element_times, verify_ft, exclusive_roots, extra_roots, default_chain_error) = {
        let st = CRYPT_STATE.lock().unwrap();
        if let Some(c) = st.chains.get(&(p_chain_context as usize)) {
            (
                c.chain_der.clone(),
                c.element_times.clone(),
                c.verify_filetime,
                c.exclusive_roots_der.clone(),
                c.extra_roots_der.clone(),
                c.default_chain_error,
            )
        } else {
            drop(st);
            let Some((ders, times, ext_err)) = extract_external_chain(p_chain_context) else {
                crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
                return FALSE;
            };
            (ders, times, None, None, Vec::new(), ext_err)
        }
    };

    let (dw_flags, extra_ptr) = if !p_policy_para.is_null() {
        let para = std::ptr::read_unaligned(p_policy_para as *const CertChainPolicyPara);
        (para.dw_flags, para.pv_extra_policy_para)
    } else {
        (0, std::ptr::null())
    };

    let mut fdw_checks = 0u32;
    let mut server_name: Option<String> = None;
    if policy_id == CERT_CHAIN_POLICY_SSL && !extra_ptr.is_null() {
        let ssl = std::ptr::read_unaligned(extra_ptr as *const SslExtraCertChainPolicyPara);
        fdw_checks = ssl.fdw_checks;
        if ssl.cb_size >= 0x18 && !ssl.pwsz_server_name.is_null() {
            let s = crate::strings::from_wide(ssl.pwsz_server_name);
            if !s.is_empty() {
                server_name = Some(s);
            }
        }
    }

    let ignore_cn = (fdw_checks & SECURITY_FLAG_IGNORE_CERT_CN_INVALID) != 0
        || (dw_flags & CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG) != 0;
    let ignore_ca = (fdw_checks & SECURITY_FLAG_IGNORE_UNKNOWN_CA) != 0
        || (dw_flags & CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG) != 0;
    let ignore_date = (fdw_checks & SECURITY_FLAG_IGNORE_CERT_DATE_INVALID) != 0
        || (dw_flags & CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG) != 0
        || (dw_flags & CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS) == CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS;
    let ignore_usage = (dw_flags & CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG) != 0;

    let check_ft = verify_ft.unwrap_or_else(now_filetime);
    let verify_unix = verify_ft
        .and_then(filetime_to_unix_time)
        .unwrap_or_else(UnixTime::now);
    let first_time_invalid_idx = element_times
        .iter()
        .position(|&(nb, na)| (nb > 0 && check_ft < nb) || (na > 0 && check_ft > na));

    let (err_code, elem_idx): (u32, i32) = match policy_id {
        CERT_CHAIN_POLICY_SSL => {
            let mut raw = verify_chain_webpki_at(
                &chain_der,
                server_name.as_deref(),
                exclusive_roots.as_deref(),
                &extra_roots,
                ignore_cn,
                verify_unix,
            );
            if (raw == CERT_E_UNTRUSTEDROOT && ignore_ca)
                || (raw == CERT_E_EXPIRED && ignore_date)
                || (raw == CERT_E_CN_NO_MATCH && ignore_cn)
                || (raw == CERT_E_WRONG_USAGE && ignore_usage)
            {
                raw = 0;
            }
            // Independent post-checks ensure ignoring one error (e.g. ALLOW_UNKNOWN_CA)
            // never silently masks an un-ignored expiration or hostname mismatch!
            if raw == 0 {
                if !ignore_date {
                    if let Some(ti) = first_time_invalid_idx {
                        (CERT_E_EXPIRED, ti as i32)
                    } else if let Some(ref host) = server_name {
                        if !ignore_cn && !chain_der.first().is_some_and(|leaf| cert_matches_hostname(leaf, host)) {
                            (CERT_E_CN_NO_MATCH, 0)
                        } else {
                            (0, -1)
                        }
                    } else {
                        (0, -1)
                    }
                } else if let Some(ref host) = server_name {
                    if !ignore_cn && !chain_der.first().is_some_and(|leaf| cert_matches_hostname(leaf, host)) {
                        (CERT_E_CN_NO_MATCH, 0)
                    } else {
                        (0, -1)
                    }
                } else {
                    (0, -1)
                }
            } else {
                let ei = if raw == CERT_E_EXPIRED {
                    first_time_invalid_idx.unwrap_or(0) as i32
                } else {
                    0
                };
                (raw, ei)
            }
        }
        CERT_CHAIN_POLICY_AUTHENTICODE | CERT_CHAIN_POLICY_AUTHENTICODE_TS => {
            const OID_CODE_SIGNING: &[u8] = &[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x03];
            const OID_TIME_STAMPING: &[u8] = &[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x08];
            const OID_ANY_EKU: &[u8] = &[0x55, 0x1D, 0x25, 0x00];
            let req_eku = if policy_id == CERT_CHAIN_POLICY_AUTHENTICODE_TS {
                OID_TIME_STAMPING
            } else {
                OID_CODE_SIGNING
            };
            let usage_bad = !ignore_usage
                && chain_der.first().and_then(|d| cert_eku_oids(d)).is_some_and(|oids| {
                    !oids.is_empty()
                        && !oids
                            .iter()
                            .any(|o| o.as_slice() == req_eku || o.as_slice() == OID_ANY_EKU)
                });
            let trust_ok = default_chain_error == 0
                || verify_chain_signatures_to_roots(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                );
            if default_chain_error == CERT_E_REVOKED {
                (CERT_E_REVOKED, 0)
            } else if usage_bad {
                (CERT_E_WRONG_USAGE, 0)
            } else if !ignore_date && first_time_invalid_idx.is_some() {
                (CERT_E_EXPIRED, first_time_invalid_idx.unwrap() as i32)
            } else if !trust_ok && !ignore_ca {
                (
                    if default_chain_error == CERT_E_CHAINING {
                        CERT_E_CHAINING
                    } else {
                        CERT_E_UNTRUSTEDROOT
                    },
                    0,
                )
            } else {
                (0, -1)
            }
        }
        CERT_CHAIN_POLICY_BASE => {
            let trust_ok = default_chain_error == 0
                || verify_chain_signatures_to_roots(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                );
            if default_chain_error == CERT_E_REVOKED {
                (CERT_E_REVOKED, 0)
            } else if !ignore_date && first_time_invalid_idx.is_some() {
                (CERT_E_EXPIRED, first_time_invalid_idx.unwrap() as i32)
            } else if !trust_ok && !ignore_ca {
                (
                    if default_chain_error == CERT_E_CHAINING {
                        CERT_E_CHAINING
                    } else {
                        CERT_E_UNTRUSTEDROOT
                    },
                    0,
                )
            } else {
                (0, -1)
            }
        }
        _ => unreachable!(),
    };

    let mut status = std::ptr::read_unaligned(p_policy_status as *const CertChainPolicyStatus);
    status.dw_error = err_code;
    status.l_chain_index = if err_code == 0 { -1 } else { 0 };
    status.l_element_index = elem_idx;
    std::ptr::write_unaligned(p_policy_status as *mut CertChainPolicyStatus, status);
    log::debug!(
        "CertVerifyCertificateChainPolicy(oid={policy_id}, host={server_name:?}) -> dwError={err_code:#010x} (elem={elem_idx})"
    );
    TRUE
}

// ---------------------------------------------------------------------------
// ASN.1 DER Encoding (`CryptEncodeObjectEx`)
// ---------------------------------------------------------------------------

fn encode_der_tlv(tag: u8, val: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + 4 + val.len());
    out.push(tag);
    let len = val.len();
    if len < 0x80 {
        out.push(len as u8);
    } else if len <= 0xFF {
        out.push(0x81);
        out.push(len as u8);
    } else if len <= 0xFFFF {
        out.push(0x82);
        out.push((len >> 8) as u8);
        out.push(len as u8);
    } else if len <= 0xFF_FFFF {
        out.push(0x83);
        out.push((len >> 16) as u8);
        out.push((len >> 8) as u8);
        out.push(len as u8);
    } else {
        out.push(0x84);
        out.extend_from_slice(&(len as u32).to_be_bytes());
    }
    out.extend_from_slice(val);
    out
}

unsafe fn encode_spki_from_struct(pv_struct_info: *const c_void) -> Option<Vec<u8>> {
    // Fast path: if pv_struct_info points into a registered OwnedCertContext, return exact X.509 DER.
    {
        let st = CRYPT_STATE.lock().unwrap();
        if let Some(&cert_h) = st.spki_to_cert.get(&(pv_struct_info as usize)) {
            if let Some(c) = st.certs.get(&cert_h) {
                if !c.owned.spki_raw_der().is_empty() {
                    return Some(c.owned.spki_raw_der().to_vec());
                }
            }
        }
    }
    let spki = std::ptr::read_unaligned(pv_struct_info as *const CertPublicKeyInfo);
    if spki.algorithm.psz_obj_id.is_null() {
        return None;
    }
    let oid_s = crate::strings::from_ansi(spki.algorithm.psz_obj_id);
    let oid_val = wintrust::encode_oid_str(&oid_s)?;
    let mut alg_inner = encode_der_tlv(0x06, &oid_val);
    if !spki.algorithm.parameters.pb_data.is_null() && spki.algorithm.parameters.cb_data > 0 {
        let params = std::slice::from_raw_parts(
            spki.algorithm.parameters.pb_data,
            spki.algorithm.parameters.cb_data as usize,
        );
        alg_inner.extend_from_slice(params);
    }
    let alg_seq = encode_der_tlv(0x30, &alg_inner);

    let key_bytes = if !spki.public_key.pb_data.is_null() && spki.public_key.cb_data > 0 {
        std::slice::from_raw_parts(spki.public_key.pb_data, spki.public_key.cb_data as usize)
    } else {
        &[]
    };
    let mut bit_val = Vec::with_capacity(1 + key_bytes.len());
    bit_val.push((spki.public_key.c_unused_bits & 0xFF) as u8);
    bit_val.extend_from_slice(key_bytes);
    let bit_str = encode_der_tlv(0x03, &bit_val);

    let mut spki_inner = Vec::with_capacity(alg_seq.len() + bit_str.len());
    spki_inner.extend_from_slice(&alg_seq);
    spki_inner.extend_from_slice(&bit_str);
    Some(encode_der_tlv(0x30, &spki_inner))
}

fn encode_der_signed_int_be(be: &[u8]) -> Vec<u8> {
    if be.is_empty() {
        return encode_der_tlv(0x02, &[0]);
    }
    let neg = (be[0] & 0x80) != 0;
    let mut start = 0usize;
    if neg {
        while start + 1 < be.len() && be[start] == 0xFF && (be[start + 1] & 0x80) != 0 {
            start += 1;
        }
    } else {
        while start + 1 < be.len() && be[start] == 0x00 && (be[start + 1] & 0x80) == 0 {
            start += 1;
        }
    }
    encode_der_tlv(0x02, &be[start..])
}

fn encode_der_uint_be(be: &[u8]) -> Vec<u8> {
    let mut start = 0usize;
    while start + 1 < be.len() && be[start] == 0 {
        start += 1;
    }
    let slice = &be[start..];
    if slice.is_empty() {
        return encode_der_tlv(0x02, &[0]);
    }
    if (slice[0] & 0x80) != 0 {
        let mut v = Vec::with_capacity(slice.len() + 1);
        v.push(0);
        v.extend_from_slice(slice);
        encode_der_tlv(0x02, &v)
    } else {
        encode_der_tlv(0x02, slice)
    }
}

fn encode_der_u32(v: u32) -> Vec<u8> {
    encode_der_uint_be(&v.to_be_bytes())
}

fn encode_der_i32(v: i32) -> Vec<u8> {
    encode_der_signed_int_be(&v.to_be_bytes())
}

fn encode_der_filetime(ft: u64) -> Option<Vec<u8>> {
    const EPOCH_DIFF_SECS: i64 = 11_644_473_600;
    let total_secs = (ft / 10_000_000) as i64 - EPOCH_DIFF_SECS;
    let days = total_secs.div_euclid(86400);
    let secs_of_day = total_secs.rem_euclid(86400);
    let hour = secs_of_day / 3600;
    let min = (secs_of_day % 3600) / 60;
    let sec = secs_of_day % 60;
    // Howard Hinnant civil_from_days
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    if (1950..=2049).contains(&year) {
        let s = format!("{:02}{:02}{:02}{:02}{:02}{:02}Z", year % 100, month, day, hour, min, sec);
        Some(encode_der_tlv(0x17, s.as_bytes()))
    } else if (0..=9999).contains(&year) {
        let s = format!("{:04}{:02}{:02}{:02}{:02}{:02}Z", year, month, day, hour, min, sec);
        Some(encode_der_tlv(0x18, s.as_bytes()))
    } else {
        None
    }
}

#[repr(C)]
struct CertBasicConstraints2Info {
    f_ca: i32,
    f_path_len_constraint: i32,
    dw_path_len_constraint: u32,
}

#[repr(C)]
struct CertEnhkeyUsage {
    c_usage_identifier: u32,
    rgpsz_usage_identifier: *const *const u8,
}

#[repr(C)]
struct CertExtension {
    psz_obj_id: *const u8,
    f_critical: i32,
    value: CryptDataBlob,
}

#[repr(C)]
struct CertExtensions {
    c_extension: u32,
    rg_extension: *const CertExtension,
}

#[repr(C)]
struct CertRdnAttr {
    psz_obj_id: *const u8,
    dw_value_type: u32,
    value: CryptDataBlob,
}

#[repr(C)]
struct CertRdn {
    c_rdn_attr: u32,
    rg_rdn_attr: *const CertRdnAttr,
}

#[repr(C)]
struct CertNameInfo {
    c_rdn: u32,
    rg_rdn: *const CertRdn,
}

#[repr(C)]
struct CertNameValue {
    dw_value_type: u32,
    value: CryptDataBlob,
}

#[repr(C)]
struct CertAltNameEntry {
    dw_alt_name_choice: u32,
    payload: [u8; 16],
}

#[repr(C)]
struct CertAltNameInfo {
    c_alt_entry: u32,
    rg_alt_entry: *const CertAltNameEntry,
}

#[repr(C)]
struct CryptSequenceOfAny {
    c_value: u32,
    rg_value: *const CryptDataBlob,
}

#[repr(C)]
struct CertEccSignature {
    r: CryptDataBlob,
    s: CryptDataBlob,
}

#[repr(C)]
struct CertSignedContentInfo {
    to_be_signed: CryptDataBlob,
    signature_algorithm: CryptAlgId,
    signature: super::wintrust::CryptBitBlob,
}

unsafe fn encode_alg_id(alg: &CryptAlgId) -> Option<Vec<u8>> {
    if alg.psz_obj_id.is_null() {
        return None;
    }
    let oid_s = crate::strings::from_ansi(alg.psz_obj_id);
    let oid_val = wintrust::encode_oid_str(&oid_s)?;
    let mut inner = encode_der_tlv(0x06, &oid_val);
    if !alg.parameters.pb_data.is_null() && alg.parameters.cb_data > 0 {
        let params = std::slice::from_raw_parts(alg.parameters.pb_data, alg.parameters.cb_data as usize);
        inner.extend_from_slice(params);
    }
    Some(encode_der_tlv(0x30, &inner))
}

unsafe fn encode_bit_blob(bb: &super::wintrust::CryptBitBlob) -> Vec<u8> {
    let key_bytes = if !bb.pb_data.is_null() && bb.cb_data > 0 {
        std::slice::from_raw_parts(bb.pb_data, bb.cb_data as usize)
    } else {
        &[]
    };
    let mut bit_val = Vec::with_capacity(1 + key_bytes.len());
    bit_val.push((bb.c_unused_bits & 0x07) as u8);
    bit_val.extend_from_slice(key_bytes);
    encode_der_tlv(0x03, &bit_val)
}

unsafe fn encode_rdn_value(dw_value_type: u32, blob: &CryptDataBlob, unicode: bool) -> Option<Vec<u8>> {
    const CERT_RDN_ENCODED_BLOB: u32 = 1;
    const CERT_RDN_OCTET_STRING: u32 = 2;
    const CERT_RDN_NUMERIC_STRING: u32 = 3;
    const CERT_RDN_PRINTABLE_STRING: u32 = 4;
    const CERT_RDN_TELETEX_STRING: u32 = 5;
    const CERT_RDN_IA5_STRING: u32 = 7;
    const CERT_RDN_VISIBLE_STRING: u32 = 9;
    const CERT_RDN_BMP_STRING: u32 = 12;
    const CERT_RDN_UTF8_STRING: u32 = 13;

    let raw_ty = dw_value_type & 0x0000_FFFF;
    if raw_ty == CERT_RDN_ENCODED_BLOB {
        if blob.pb_data.is_null() {
            return None;
        }
        return Some(std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize).to_vec());
    }
    if raw_ty == CERT_RDN_OCTET_STRING {
        let data = if blob.pb_data.is_null() || blob.cb_data == 0 {
            &[][..]
        } else {
            std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize)
        };
        return Some(encode_der_tlv(0x04, data));
    }

    let text: String = if unicode {
        if blob.pb_data.is_null() {
            String::new()
        } else if blob.cb_data == 0 {
            crate::strings::from_wide(blob.pb_data as *const u16)
        } else {
            let n_u16 = (blob.cb_data as usize) / 2;
            let mut u = Vec::with_capacity(n_u16);
            for i in 0..n_u16 {
                u.push(std::ptr::read_unaligned((blob.pb_data as *const u16).add(i)));
            }
            String::from_utf16_lossy(&u)
        }
    } else if blob.pb_data.is_null() {
        String::new()
    } else if blob.cb_data == 0 {
        crate::strings::from_ansi(blob.pb_data)
    } else {
        let s = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
        String::from_utf8_lossy(s).into_owned()
    };

    match raw_ty {
        CERT_RDN_NUMERIC_STRING => Some(encode_der_tlv(0x12, text.as_bytes())),
        CERT_RDN_PRINTABLE_STRING => Some(encode_der_tlv(0x13, text.as_bytes())),
        CERT_RDN_TELETEX_STRING => Some(encode_der_tlv(0x14, text.as_bytes())),
        CERT_RDN_IA5_STRING => Some(encode_der_tlv(0x16, text.as_bytes())),
        CERT_RDN_VISIBLE_STRING => Some(encode_der_tlv(0x1A, text.as_bytes())),
        CERT_RDN_BMP_STRING => {
            let be: Vec<u8> = text.encode_utf16().flat_map(u16::to_be_bytes).collect();
            Some(encode_der_tlv(0x1E, &be))
        }
        CERT_RDN_UTF8_STRING | _ => Some(encode_der_tlv(0x0C, text.as_bytes())),
    }
}

unsafe fn encode_cert_name_info(pv_struct_info: *const c_void, unicode: bool) -> Option<Vec<u8>> {
    let info = std::ptr::read_unaligned(pv_struct_info as *const CertNameInfo);
    let mut seq_inner = Vec::new();
    if info.c_rdn > 0 && info.rg_rdn.is_null() {
        return None;
    }
    for i in 0..info.c_rdn as usize {
        let rdn = std::ptr::read_unaligned(info.rg_rdn.add(i));
        if rdn.c_rdn_attr > 0 && rdn.rg_rdn_attr.is_null() {
            return None;
        }
        let mut set_inner = Vec::new();
        for j in 0..rdn.c_rdn_attr as usize {
            let attr = std::ptr::read_unaligned(rdn.rg_rdn_attr.add(j));
            if attr.psz_obj_id.is_null() {
                return None;
            }
            let oid_s = crate::strings::from_ansi(attr.psz_obj_id);
            let oid_bytes = wintrust::encode_oid_str(&oid_s)?;
            let val_der = encode_rdn_value(attr.dw_value_type, &attr.value, unicode)?;
            let mut atv = encode_der_tlv(0x06, &oid_bytes);
            atv.extend_from_slice(&val_der);
            set_inner.extend_from_slice(&encode_der_tlv(0x30, &atv));
        }
        seq_inner.extend_from_slice(&encode_der_tlv(0x31, &set_inner));
    }
    Some(encode_der_tlv(0x30, &seq_inner))
}

unsafe fn encode_extensions_list(c_ext: u32, rg_ext: *const CertExtension) -> Option<Vec<u8>> {
    if c_ext > 0 && rg_ext.is_null() {
        return None;
    }
    let mut inner = Vec::new();
    for i in 0..c_ext as usize {
        let ext = std::ptr::read_unaligned(rg_ext.add(i));
        if ext.psz_obj_id.is_null() {
            return None;
        }
        let oid_s = crate::strings::from_ansi(ext.psz_obj_id);
        let oid_bytes = wintrust::encode_oid_str(&oid_s)?;
        let mut e = encode_der_tlv(0x06, &oid_bytes);
        if ext.f_critical != 0 {
            e.extend_from_slice(&encode_der_tlv(0x01, &[0xFF]));
        }
        let val = if ext.value.pb_data.is_null() || ext.value.cb_data == 0 {
            &[][..]
        } else {
            std::slice::from_raw_parts(ext.value.pb_data, ext.value.cb_data as usize)
        };
        e.extend_from_slice(&encode_der_tlv(0x04, val));
        inner.extend_from_slice(&encode_der_tlv(0x30, &e));
    }
    Some(encode_der_tlv(0x30, &inner))
}

unsafe fn encode_x509_object(lpsz_struct_type: *const u8, pv_struct_info: *const c_void) -> Result<Vec<u8>, u32> {
    let raw_stype = lpsz_struct_type as usize;
    let stype: usize = if raw_stype <= 0xFFFF {
        raw_stype
    } else {
        match crate::strings::from_ansi(lpsz_struct_type).as_str() {
            "2.5.29.14" => X509_OCTET_STRING,
            "2.5.29.15" => X509_KEY_USAGE,
            "2.5.29.17" | "2.5.29.18" => X509_ALTERNATE_NAME,
            "2.5.29.19" => X509_BASIC_CONSTRAINTS2,
            "2.5.29.37" => X509_ENHANCED_KEY_USAGE,
            _ => 0,
        }
    };

    match stype {
        X509_PUBLIC_KEY_INFO => encode_spki_from_struct(pv_struct_info).ok_or(err::ERROR_INVALID_DATA),
        X509_BASIC_CONSTRAINTS2 => {
            let bc = std::ptr::read_unaligned(pv_struct_info as *const CertBasicConstraints2Info);
            let mut inner = Vec::new();
            if bc.f_ca != 0 {
                inner.extend_from_slice(&encode_der_tlv(0x01, &[0xFF]));
            }
            if bc.f_path_len_constraint != 0 {
                inner.extend_from_slice(&encode_der_u32(bc.dw_path_len_constraint));
            }
            Ok(encode_der_tlv(0x30, &inner))
        }
        X509_ENHANCED_KEY_USAGE => {
            let eku = std::ptr::read_unaligned(pv_struct_info as *const CertEnhkeyUsage);
            if eku.c_usage_identifier > 0 && eku.rgpsz_usage_identifier.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut inner = Vec::new();
            for i in 0..eku.c_usage_identifier as usize {
                let p = std::ptr::read_unaligned(eku.rgpsz_usage_identifier.add(i));
                if p.is_null() {
                    return Err(err::ERROR_INVALID_DATA);
                }
                let oid_s = crate::strings::from_ansi(p);
                let oid_val = wintrust::encode_oid_str(&oid_s).ok_or(err::ERROR_INVALID_DATA)?;
                inner.extend_from_slice(&encode_der_tlv(0x06, &oid_val));
            }
            Ok(encode_der_tlv(0x30, &inner))
        }
        X509_ALTERNATE_NAME => {
            let san = std::ptr::read_unaligned(pv_struct_info as *const CertAltNameInfo);
            if san.c_alt_entry > 0 && san.rg_alt_entry.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut inner = Vec::new();
            for i in 0..san.c_alt_entry as usize {
                let e = std::ptr::read_unaligned(san.rg_alt_entry.add(i));
                let ptr = std::ptr::read_unaligned(e.payload.as_ptr() as *const *const u8);
                match e.dw_alt_name_choice {
                    2 | 3 | 7 => {
                        // RFC822_NAME (2 -> 0x81), DNS_NAME (3 -> 0x82), URL (7 -> 0x86)
                        if ptr.is_null() {
                            return Err(err::ERROR_INVALID_DATA);
                        }
                        let s = crate::strings::from_wide(ptr as *const u16);
                        let tag = match e.dw_alt_name_choice {
                            2 => 0x81,
                            3 => 0x82,
                            _ => 0x86,
                        };
                        inner.extend_from_slice(&encode_der_tlv(tag, s.as_bytes()));
                    }
                    5 => {
                        // DIRECTORY_NAME -> [4] EXPLICIT Name
                        let blob = std::ptr::read_unaligned(e.payload.as_ptr() as *const CryptDataBlob);
                        if blob.pb_data.is_null() || blob.cb_data == 0 {
                            return Err(err::ERROR_INVALID_DATA);
                        }
                        let b = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
                        inner.extend_from_slice(&encode_der_tlv(0xA4, b));
                    }
                    8 => {
                        // IP_ADDRESS -> [7] IMPLICIT OCTET STRING (0x87)
                        let blob = std::ptr::read_unaligned(e.payload.as_ptr() as *const CryptDataBlob);
                        if blob.pb_data.is_null() || blob.cb_data == 0 {
                            return Err(err::ERROR_INVALID_DATA);
                        }
                        let b = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
                        inner.extend_from_slice(&encode_der_tlv(0x87, b));
                    }
                    9 => {
                        // REGISTERED_ID -> [8] IMPLICIT OID (0x88)
                        if ptr.is_null() {
                            return Err(err::ERROR_INVALID_DATA);
                        }
                        let oid_s = crate::strings::from_ansi(ptr);
                        let oid_val = wintrust::encode_oid_str(&oid_s).ok_or(err::ERROR_INVALID_DATA)?;
                        inner.extend_from_slice(&encode_der_tlv(0x88, &oid_val));
                    }
                    _ => return Err(err::ERROR_INVALID_DATA),
                }
            }
            Ok(encode_der_tlv(0x30, &inner))
        }
        X509_NAME => encode_cert_name_info(pv_struct_info, false).ok_or(err::ERROR_INVALID_DATA),
        X509_UNICODE_NAME => encode_cert_name_info(pv_struct_info, true).ok_or(err::ERROR_INVALID_DATA),
        X509_ANY_STRING | X509_UNICODE_ANY_STRING => {
            let nv = std::ptr::read_unaligned(pv_struct_info as *const CertNameValue);
            encode_rdn_value(nv.dw_value_type, &nv.value, stype == X509_UNICODE_ANY_STRING)
                .ok_or(err::ERROR_INVALID_DATA)
        }
        X509_EXTENSIONS => {
            let exts = std::ptr::read_unaligned(pv_struct_info as *const CertExtensions);
            encode_extensions_list(exts.c_extension, exts.rg_extension).ok_or(err::ERROR_INVALID_DATA)
        }
        X509_OCTET_STRING => {
            let blob = std::ptr::read_unaligned(pv_struct_info as *const CryptDataBlob);
            let data = if blob.pb_data.is_null() || blob.cb_data == 0 {
                &[][..]
            } else {
                std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize)
            };
            Ok(encode_der_tlv(0x04, data))
        }
        X509_BITS | X509_KEY_USAGE => {
            let bb = std::ptr::read_unaligned(pv_struct_info as *const super::wintrust::CryptBitBlob);
            Ok(encode_bit_blob(&bb))
        }
        X509_INTEGER => {
            let v = std::ptr::read_unaligned(pv_struct_info as *const i32);
            Ok(encode_der_i32(v))
        }
        X509_MULTI_BYTE_INTEGER | X509_MULTI_BYTE_UINT => {
            let blob = std::ptr::read_unaligned(pv_struct_info as *const CryptDataBlob);
            if blob.pb_data.is_null() || blob.cb_data == 0 {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut be = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize).to_vec();
            be.reverse(); // Windows CRYPT_INTEGER_BLOB / CRYPT_UINT_BLOB is little-endian
            if stype == X509_MULTI_BYTE_UINT {
                Ok(encode_der_uint_be(&be))
            } else {
                Ok(encode_der_signed_int_be(&be))
            }
        }
        X509_CHOICE_OF_TIME => {
            let ft = std::ptr::read_unaligned(pv_struct_info as *const u64);
            encode_der_filetime(ft).ok_or(err::ERROR_INVALID_DATA)
        }
        X509_SEQUENCE_OF_ANY => {
            let sq = std::ptr::read_unaligned(pv_struct_info as *const CryptSequenceOfAny);
            if sq.c_value > 0 && sq.rg_value.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut inner = Vec::new();
            for i in 0..sq.c_value as usize {
                let b = std::ptr::read_unaligned(sq.rg_value.add(i));
                if !b.pb_data.is_null() && b.cb_data > 0 {
                    inner.extend_from_slice(std::slice::from_raw_parts(b.pb_data, b.cb_data as usize));
                }
            }
            Ok(encode_der_tlv(0x30, &inner))
        }
        X509_OBJECT_IDENTIFIER => {
            let oid_ptr = std::ptr::read_unaligned(pv_struct_info as *const *const u8);
            if oid_ptr.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            let oid_s = crate::strings::from_ansi(oid_ptr);
            let oid_val = wintrust::encode_oid_str(&oid_s).ok_or(err::ERROR_INVALID_DATA)?;
            Ok(encode_der_tlv(0x06, &oid_val))
        }
        X509_ECC_SIGNATURE => {
            let sig = std::ptr::read_unaligned(pv_struct_info as *const CertEccSignature);
            if sig.r.pb_data.is_null() || sig.r.cb_data == 0 || sig.s.pb_data.is_null() || sig.s.cb_data == 0 {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut r_be = std::slice::from_raw_parts(sig.r.pb_data, sig.r.cb_data as usize).to_vec();
            let mut s_be = std::slice::from_raw_parts(sig.s.pb_data, sig.s.cb_data as usize).to_vec();
            r_be.reverse(); // Windows CERT_ECC_SIGNATURE stores r, s little-endian
            s_be.reverse();
            let mut inner = encode_der_uint_be(&r_be);
            inner.extend_from_slice(&encode_der_uint_be(&s_be));
            Ok(encode_der_tlv(0x30, &inner))
        }
        RSA_CSP_PUBLICKEYBLOB => {
            // PUBLICKEYSTRUC (8B) + RSAPUBKEY (12B: magic "RSA1", bitlen u32, pubexp u32) + modulus (LE)
            let p = pv_struct_info as *const u8;
            let magic = std::ptr::read_unaligned(p.add(8) as *const u32);
            let bitlen = std::ptr::read_unaligned(p.add(12) as *const u32) as usize;
            let pubexp = std::ptr::read_unaligned(p.add(16) as *const u32);
            if magic != 0x3141_5352 || bitlen == 0 || bitlen % 8 != 0 {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut mod_be = std::slice::from_raw_parts(p.add(20), bitlen / 8).to_vec();
            mod_be.reverse();
            let mut inner = encode_der_uint_be(&mod_be);
            inner.extend_from_slice(&encode_der_u32(pubexp));
            Ok(encode_der_tlv(0x30, &inner))
        }
        X509_CERT => {
            let sci = std::ptr::read_unaligned(pv_struct_info as *const CertSignedContentInfo);
            if sci.to_be_signed.pb_data.is_null() || sci.to_be_signed.cb_data == 0 {
                return Err(err::ERROR_INVALID_DATA);
            }
            let tbs = std::slice::from_raw_parts(sci.to_be_signed.pb_data, sci.to_be_signed.cb_data as usize);
            let alg = encode_alg_id(&sci.signature_algorithm).ok_or(err::ERROR_INVALID_DATA)?;
            let sig = encode_bit_blob(&sci.signature);
            let mut inner = Vec::with_capacity(tbs.len() + alg.len() + sig.len());
            inner.extend_from_slice(tbs);
            inner.extend_from_slice(&alg);
            inner.extend_from_slice(&sig);
            Ok(encode_der_tlv(0x30, &inner))
        }
        X509_CERT_TO_BE_SIGNED => {
            let ci = std::ptr::read_unaligned(pv_struct_info as *const CertInfo);
            let mut tbs = Vec::new();
            if ci.dw_version > 0 {
                tbs.extend_from_slice(&encode_der_tlv(0xA0, &encode_der_u32(ci.dw_version)));
            }
            let mut ser_be = if !ci.serial_number.pb_data.is_null() && ci.serial_number.cb_data > 0 {
                std::slice::from_raw_parts(ci.serial_number.pb_data, ci.serial_number.cb_data as usize).to_vec()
            } else {
                vec![1]
            };
            ser_be.reverse();
            tbs.extend_from_slice(&encode_der_uint_be(&ser_be));
            tbs.extend_from_slice(&encode_alg_id(&ci.signature_algorithm).ok_or(err::ERROR_INVALID_DATA)?);
            if ci.issuer.pb_data.is_null() || ci.subject.pb_data.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            tbs.extend_from_slice(std::slice::from_raw_parts(ci.issuer.pb_data, ci.issuer.cb_data as usize));
            let nb = encode_der_filetime(ci.not_before).ok_or(err::ERROR_INVALID_DATA)?;
            let na = encode_der_filetime(ci.not_after).ok_or(err::ERROR_INVALID_DATA)?;
            tbs.extend_from_slice(&encode_der_tlv(0x30, &[nb, na].concat()));
            tbs.extend_from_slice(std::slice::from_raw_parts(ci.subject.pb_data, ci.subject.cb_data as usize));
            let spki_der = encode_spki_from_struct(&ci.subject_public_key_info as *const _ as *const c_void)
                .ok_or(err::ERROR_INVALID_DATA)?;
            tbs.extend_from_slice(&spki_der);
            if ci.c_extension > 0 && !ci.rg_extension.is_null() {
                let exts = encode_extensions_list(ci.c_extension, ci.rg_extension as *const CertExtension)
                    .ok_or(err::ERROR_INVALID_DATA)?;
                tbs.extend_from_slice(&encode_der_tlv(0xA3, &exts));
            }
            Ok(encode_der_tlv(0x30, &tbs))
        }
        _ => {
            stub!("CryptEncodeObjectEx(struct_type={raw_stype:#x}) -> FALSE");
            Err(ERROR_CALL_NOT_IMPLEMENTED)
        }
    }
}

pub unsafe extern "win64" fn CryptEncodeObjectEx(
    dw_cert_encoding_type: u32,
    lpsz_struct_type: *const u8,
    pv_struct_info: *const c_void,
    dw_flags: u32,
    _p_encode_para: *const c_void,
    pv_encoded: *mut c_void,
    pcb_encoded: *mut u32,
) -> BOOL {
    if (dw_cert_encoding_type & X509_ASN_ENCODING) == 0 || pv_struct_info.is_null() || pcb_encoded.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let der = match encode_x509_object(lpsz_struct_type, pv_struct_info) {
        Ok(d) => d,
        Err(code) => {
            crate::teb::set_last_error(code);
            return FALSE;
        }
    };
    let need = der.len() as u32;
    if (dw_flags & CRYPT_ENCODE_ALLOC_FLAG) != 0 {
        if pv_encoded.is_null() {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        let buf = crate::kernel32::memory::LocalAlloc(0, der.len()) as *mut u8;
        if buf.is_null() {
            crate::teb::set_last_error(err::ERROR_NOT_ENOUGH_MEMORY);
            return FALSE;
        }
        std::ptr::copy_nonoverlapping(der.as_ptr(), buf, der.len());
        std::ptr::write_unaligned(pv_encoded as *mut *mut u8, buf);
        std::ptr::write_unaligned(pcb_encoded, need);
        return TRUE;
    }
    if pv_encoded.is_null() {
        std::ptr::write_unaligned(pcb_encoded, need);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pcb_encoded);
    if cap < need {
        std::ptr::write_unaligned(pcb_encoded, need);
        crate::teb::set_last_error(err::ERROR_MORE_DATA);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(der.as_ptr(), pv_encoded as *mut u8, der.len());
    std::ptr::write_unaligned(pcb_encoded, need);
    TRUE
}

// ---------------------------------------------------------------------------
// Name Formatting, Memory Helpers, and String<->Binary Conversions
// ---------------------------------------------------------------------------

unsafe fn write_wide_out(s: &str, buf: *mut u16, cch: u32) -> u32 {
    let units: Vec<u16> = s.encode_utf16().collect();
    let needed = units.len() + 1;
    if buf.is_null() || cch == 0 {
        return needed as u32;
    }
    let cap = cch as usize;
    let copy = units.len().min(cap - 1);
    for (i, &u) in units[..copy].iter().enumerate() {
        std::ptr::write_unaligned(buf.add(i), u);
    }
    std::ptr::write_unaligned(buf.add(copy), 0);
    (copy + 1) as u32
}

unsafe fn write_ansi_out(s: &str, buf: *mut u8, cch: u32) -> u32 {
    let bytes = s.as_bytes();
    let needed = bytes.len() + 1;
    if buf.is_null() || cch == 0 {
        return needed as u32;
    }
    let cap = cch as usize;
    let copy = bytes.len().min(cap - 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, copy);
    std::ptr::write_unaligned(buf.add(copy), 0);
    (copy + 1) as u32
}

pub unsafe extern "win64" fn CertNameToStrW(_enc: u32, name: *const c_void, _ty: u32, buf: *mut u16, cch: u32) -> u32 {
    let s = if !name.is_null() {
        let blob = std::ptr::read_unaligned(name as *const super::wintrust::CryptDataBlob);
        if !blob.pb_data.is_null() && blob.cb_data > 0 {
            let der = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            super::wintrust::format_dn(der).unwrap_or_default()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    write_wide_out(&s, buf, cch)
}

pub unsafe extern "win64" fn CertNameToStrA(_enc: u32, name: *const c_void, _ty: u32, buf: *mut u8, cch: u32) -> u32 {
    let s = if !name.is_null() {
        let blob = std::ptr::read_unaligned(name as *const super::wintrust::CryptDataBlob);
        if !blob.pb_data.is_null() && blob.cb_data > 0 {
            let der = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            super::wintrust::format_dn(der).unwrap_or_default()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    write_ansi_out(&s, buf, cch)
}

pub unsafe extern "win64" fn CryptMsgClose(h_msg: *mut c_void) -> BOOL {
    if !h_msg.is_null() {
        let mut st = CRYPT_STATE.lock().unwrap();
        if st.msgs.remove(&(h_msg as usize)).is_none() {
            stub!("CryptMsgClose({h_msg:p}): foreign message ignored");
        }
    }
    TRUE
}

fn trim_le_serial_zeros(b: &[u8]) -> &[u8] {
    let mut end = b.len();
    while end > 1 && b[end - 1] == 0 {
        end -= 1;
    }
    &b[..end]
}

unsafe fn cert_matches_find(
    owned: &OwnedCertContext,
    dw_find_type: u32,
    pv_find_para: *const c_void,
) -> bool {
    let cc = &*owned.as_ptr();
    let ci = &*cc.p_cert_info;
    let issuer_slice = if ci.issuer.pb_data.is_null() || ci.issuer.cb_data == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(ci.issuer.pb_data, ci.issuer.cb_data as usize)
    };
    let subject_slice = if ci.subject.pb_data.is_null() || ci.subject.cb_data == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(ci.subject.pb_data, ci.subject.cb_data as usize)
    };
    let serial_slice = if ci.serial_number.pb_data.is_null() || ci.serial_number.cb_data == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(ci.serial_number.pb_data, ci.serial_number.cb_data as usize)
    };

    match dw_find_type {
        CERT_FIND_ANY => true,
        CERT_FIND_SUBJECT_CERT => {
            if pv_find_para.is_null() {
                return false;
            }
            let query = std::ptr::read_unaligned(pv_find_para as *const CertInfo);
            let q_issuer = if query.issuer.pb_data.is_null() || query.issuer.cb_data == 0 {
                &[][..]
            } else {
                std::slice::from_raw_parts(query.issuer.pb_data, query.issuer.cb_data as usize)
            };
            let q_serial = if query.serial_number.pb_data.is_null() || query.serial_number.cb_data == 0 {
                &[][..]
            } else {
                std::slice::from_raw_parts(query.serial_number.pb_data, query.serial_number.cb_data as usize)
            };
            issuer_slice == q_issuer
                && trim_le_serial_zeros(serial_slice) == trim_le_serial_zeros(q_serial)
        }
        CERT_FIND_ISSUER_NAME => {
            if pv_find_para.is_null() {
                return false;
            }
            let blob = std::ptr::read_unaligned(pv_find_para as *const CryptDataBlob);
            if blob.pb_data.is_null() {
                return false;
            }
            let q = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            issuer_slice == q
        }
        CERT_FIND_SUBJECT_NAME => {
            if pv_find_para.is_null() {
                return false;
            }
            let blob = std::ptr::read_unaligned(pv_find_para as *const CryptDataBlob);
            if blob.pb_data.is_null() {
                return false;
            }
            let q = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            subject_slice == q
        }
        CERT_FIND_SUBJECT_STR_W | CERT_FIND_ISSUER_STR_W => {
            if pv_find_para.is_null() {
                return false;
            }
            let needle = crate::strings::from_wide(pv_find_para as *const u16).to_ascii_lowercase();
            let dn = if dw_find_type == CERT_FIND_ISSUER_STR_W { issuer_slice } else { subject_slice };
            let hay = wintrust::format_dn(dn).unwrap_or_default().to_ascii_lowercase();
            hay.contains(&needle) || all_dn_values_lower(dn).iter().any(|v| v.contains(&needle))
        }
        CERT_FIND_SUBJECT_STR_A | CERT_FIND_ISSUER_STR_A => {
            if pv_find_para.is_null() {
                return false;
            }
            let needle = crate::strings::from_ansi(pv_find_para as *const u8).to_ascii_lowercase();
            let dn = if dw_find_type == CERT_FIND_ISSUER_STR_A { issuer_slice } else { subject_slice };
            let hay = wintrust::format_dn(dn).unwrap_or_default().to_ascii_lowercase();
            hay.contains(&needle) || all_dn_values_lower(dn).iter().any(|v| v.contains(&needle))
        }
        CERT_FIND_SHA1_HASH => {
            if pv_find_para.is_null() {
                return false;
            }
            let blob = std::ptr::read_unaligned(pv_find_para as *const CryptDataBlob);
            if blob.pb_data.is_null() || blob.cb_data == 0 {
                return false;
            }
            let q = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            super::bcrypt::Digest::Sha1.hash(owned.raw_der()).as_slice() == q
        }
        CERT_FIND_MD5_HASH => {
            if pv_find_para.is_null() {
                return false;
            }
            let blob = std::ptr::read_unaligned(pv_find_para as *const CryptDataBlob);
            if blob.pb_data.is_null() || blob.cb_data == 0 {
                return false;
            }
            let q = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            super::bcrypt::Digest::Md5.hash(owned.raw_der()).as_slice() == q
        }
        CERT_FIND_PUBLIC_KEY => {
            if pv_find_para.is_null() {
                return false;
            }
            let qspki = std::ptr::read_unaligned(pv_find_para as *const CertPublicKeyInfo);
            let q_key = if qspki.public_key.pb_data.is_null() || qspki.public_key.cb_data == 0 {
                &[][..]
            } else {
                std::slice::from_raw_parts(qspki.public_key.pb_data, qspki.public_key.cb_data as usize)
            };
            let c_key = if ci.subject_public_key_info.public_key.pb_data.is_null()
                || ci.subject_public_key_info.public_key.cb_data == 0
            {
                &[][..]
            } else {
                std::slice::from_raw_parts(
                    ci.subject_public_key_info.public_key.pb_data,
                    ci.subject_public_key_info.public_key.cb_data as usize,
                )
            };
            if c_key != q_key {
                return false;
            }
            if !qspki.algorithm.psz_obj_id.is_null() && !ci.subject_public_key_info.algorithm.psz_obj_id.is_null() {
                crate::strings::from_ansi(qspki.algorithm.psz_obj_id)
                    == crate::strings::from_ansi(ci.subject_public_key_info.algorithm.psz_obj_id)
            } else {
                true
            }
        }
        CERT_FIND_ISSUER_OF => {
            if pv_find_para.is_null() {
                return false;
            }
            let qcc = std::ptr::read_unaligned(pv_find_para as *const CertContext);
            if qcc.p_cert_info.is_null() {
                return false;
            }
            let qci = std::ptr::read_unaligned(qcc.p_cert_info);
            if qci.issuer.pb_data.is_null() || qci.issuer.cb_data == 0 {
                return false;
            }
            let q_issuer = std::slice::from_raw_parts(qci.issuer.pb_data, qci.issuer.cb_data as usize);
            subject_slice == q_issuer
        }
        CERT_FIND_EXISTING => {
            if pv_find_para.is_null() {
                return false;
            }
            let qcc = std::ptr::read_unaligned(pv_find_para as *const CertContext);
            if qcc.pb_cert_encoded.is_null() || qcc.cb_cert_encoded == 0 {
                return false;
            }
            let q = std::slice::from_raw_parts(qcc.pb_cert_encoded, qcc.cb_cert_encoded as usize);
            owned.raw_der() == q
        }
        _ => {
            stub!("CertFindCertificateInStore(find_type={dw_find_type:#x}) unsupported");
            false
        }
    }
}

pub unsafe extern "win64" fn CertGetIssuerCertificateFromStore(
    h_cert_store: *mut c_void,
    p_subject_context: *const c_void,
    p_prev_issuer_context: *mut c_void,
    pdw_flags: *mut u32,
) -> *mut c_void {
    if p_subject_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let found = CertFindCertificateInStore(
        h_cert_store,
        X509_ASN_ENCODING,
        0,
        CERT_FIND_ISSUER_OF,
        p_subject_context,
        p_prev_issuer_context,
    );
    if !found.is_null() && !pdw_flags.is_null() {
        let req = std::ptr::read_unaligned(pdw_flags);
        let mut out_flags = 0u32;
        let subj_cc = std::ptr::read_unaligned(p_subject_context as *const CertContext);
        let iss_cc = std::ptr::read_unaligned(found as *const CertContext);
        if (req & 0x0000_0001) != 0 {
            // CERT_STORE_SIGNATURE_FLAG
            let sig_ok = if !subj_cc.pb_cert_encoded.is_null() && !iss_cc.pb_cert_encoded.is_null() {
                let s_der = std::slice::from_raw_parts(subj_cc.pb_cert_encoded, subj_cc.cb_cert_encoded as usize);
                let i_der = std::slice::from_raw_parts(iss_cc.pb_cert_encoded, iss_cc.cb_cert_encoded as usize);
                match (parse_cert_der_parts(s_der), parse_cert_der_parts(i_der)) {
                    (Some(sp), Some(ip)) => verify_cert_sig_with_spki(&sp, ip.spki_alg_inner, ip.spki_key_bits),
                    _ => false,
                }
            } else {
                false
            };
            if !sig_ok {
                out_flags |= 0x0000_0001;
            }
        }
        if (req & 0x0000_0002) != 0 {
            // CERT_STORE_TIME_VALIDITY_FLAG
            if CertVerifyTimeValidity(std::ptr::null(), iss_cc.p_cert_info as *const c_void) != 0 {
                out_flags |= 0x0000_0002;
            }
        }
        std::ptr::write_unaligned(pdw_flags, out_flags);
    }
    found
}

pub unsafe extern "win64" fn CertFindCertificateInStore(
    h_cert_store: *mut c_void,
    _dw_cert_encoding_type: u32,
    _dw_find_flags: u32,
    dw_find_type: u32,
    pv_find_para: *const c_void,
    p_prev_cert_context: *mut c_void,
) -> *mut c_void {
    let mut st = CRYPT_STATE.lock().unwrap();
    let Some(certs) = st.stores.get(&(h_cert_store as usize)).map(|s| s.certs.clone()) else {
        if !p_prev_cert_context.is_null() {
            release_cert_locked(&mut st, p_prev_cert_context as usize);
        }
        crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
        return std::ptr::null_mut();
    };
    let start_idx = if p_prev_cert_context.is_null() {
        0
    } else {
        let prev = p_prev_cert_context as usize;
        let pos = certs.iter().position(|&h| h == prev).map(|i| i + 1);
        release_cert_locked(&mut st, prev);
        match pos {
            Some(i) => i,
            None => {
                crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                return std::ptr::null_mut();
            }
        }
    };
    for &cert_h in &certs[start_idx..] {
        if let Some(entry) = st.certs.get(&cert_h) {
            if cert_matches_find(&entry.owned, dw_find_type, pv_find_para) {
                st.certs.get_mut(&cert_h).unwrap().refcount += 1;
                return cert_h as *mut c_void;
            }
        }
    }
    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn CertGetSubjectCertificateFromStore(
    h_cert_store: *mut c_void,
    dw_cert_encoding_type: u32,
    p_cert_id: *const c_void,
) -> *mut c_void {
    CertFindCertificateInStore(
        h_cert_store,
        dw_cert_encoding_type,
        0,
        CERT_FIND_SUBJECT_CERT,
        p_cert_id,
        std::ptr::null_mut(),
    )
}

pub unsafe extern "win64" fn CryptQueryObject(
    dw_object_type: u32,
    pv_object: *const c_void,
    dw_expected_content_type_flags: u32,
    dw_expected_format_type_flags: u32,
    _dw_flags: u32,
    pdw_msg_and_cert_encoding_type: *mut u32,
    pdw_content_type: *mut u32,
    pdw_format_type: *mut u32,
    ph_cert_store: *mut *mut c_void,
    ph_msg: *mut *mut c_void,
    ppv_context: *mut *mut c_void,
) -> BOOL {
    if !ph_cert_store.is_null() {
        std::ptr::write_unaligned(ph_cert_store, std::ptr::null_mut());
    }
    if !ph_msg.is_null() {
        std::ptr::write_unaligned(ph_msg, std::ptr::null_mut());
    }
    if !ppv_context.is_null() {
        std::ptr::write_unaligned(ppv_context, std::ptr::null_mut());
    }
    if pv_object.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let bytes = match dw_object_type {
        CERT_QUERY_OBJECT_FILE => {
            let win = crate::strings::from_wide(pv_object as *const u16);
            let abs = crate::path::absolute(&win);
            let Some(host) = crate::path::to_host(&abs) else {
                crate::teb::set_last_error(err::ERROR_FILE_NOT_FOUND);
                return FALSE;
            };
            match std::fs::read(&host) {
                Ok(b) => b,
                Err(_) => {
                    crate::teb::set_last_error(err::ERROR_FILE_NOT_FOUND);
                    return FALSE;
                }
            }
        }
        CERT_QUERY_OBJECT_BLOB => {
            let blob = std::ptr::read_unaligned(pv_object as *const CryptDataBlob);
            if blob.pb_data.is_null() || blob.cb_data == 0 {
                crate::teb::set_last_error(CRYPT_E_NO_MATCH);
                return FALSE;
            }
            std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize).to_vec()
        }
        _ => {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
    };

    if (dw_expected_format_type_flags & CERT_QUERY_FORMAT_FLAG_BINARY) == 0 {
        crate::teb::set_last_error(CRYPT_E_NO_MATCH);
        return FALSE;
    }

    if let Some(parsed) = wintrust::parse_pkcs7_or_pe_authenticode(&bytes) {
        let content_type = if parsed.embedded_in_pe {
            CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED
        } else {
            CERT_QUERY_CONTENT_PKCS7_SIGNED
        };
        if (dw_expected_content_type_flags & (1u32 << content_type)) == 0 {
            crate::teb::set_last_error(CRYPT_E_NO_MATCH);
            return FALSE;
        }
        if !pdw_msg_and_cert_encoding_type.is_null() {
            std::ptr::write_unaligned(
                pdw_msg_and_cert_encoding_type,
                X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            );
        }
        if !pdw_content_type.is_null() {
            std::ptr::write_unaligned(pdw_content_type, content_type);
        }
        if !pdw_format_type.is_null() {
            std::ptr::write_unaligned(pdw_format_type, CERT_QUERY_FORMAT_BINARY);
        }
        if !ph_cert_store.is_null() {
            let store = CertOpenStore(
                CERT_STORE_PROV_MEMORY as *const u8,
                X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
                0,
                0,
                std::ptr::null(),
            );
            for der in &parsed.certs_der {
                let _ = add_der_to_store(store, der, CERT_STORE_ADD_ALWAYS, std::ptr::null_mut());
            }
            std::ptr::write_unaligned(ph_cert_store, store);
        }
        if !ph_msg.is_null() {
            let mut st = CRYPT_STATE.lock().unwrap();
            let id = st.next_msg_id;
            st.next_msg_id += 1;
            st.msgs.insert(id, parsed);
            std::ptr::write_unaligned(ph_msg, id as *mut c_void);
        }
        return TRUE;
    }

    if (dw_expected_content_type_flags & (1u32 << CERT_QUERY_CONTENT_CERT)) != 0 {
        if let Some(owned) = wintrust::build_owned_cert_context(&bytes, std::ptr::null_mut()) {
            if !pdw_msg_and_cert_encoding_type.is_null() {
                std::ptr::write_unaligned(pdw_msg_and_cert_encoding_type, X509_ASN_ENCODING);
            }
            if !pdw_content_type.is_null() {
                std::ptr::write_unaligned(pdw_content_type, CERT_QUERY_CONTENT_CERT);
            }
            if !pdw_format_type.is_null() {
                std::ptr::write_unaligned(pdw_format_type, CERT_QUERY_FORMAT_BINARY);
            }
            if !ph_cert_store.is_null() {
                let store = CertOpenStore(
                    CERT_STORE_PROV_MEMORY as *const u8,
                    X509_ASN_ENCODING,
                    0,
                    0,
                    std::ptr::null(),
                );
                let _ = add_der_to_store(store, &bytes, CERT_STORE_ADD_ALWAYS, std::ptr::null_mut());
                std::ptr::write_unaligned(ph_cert_store, store);
            }
            if !ppv_context.is_null() {
                let mut st = CRYPT_STATE.lock().unwrap();
                let h = insert_cert_locked(&mut st, owned, vec![bytes], None, 0);
                std::ptr::write_unaligned(ppv_context, h as *mut c_void);
            }
            return TRUE;
        }
    }

    crate::teb::set_last_error(CRYPT_E_NO_MATCH);
    FALSE
}

pub unsafe extern "win64" fn CryptMsgGetParam(
    h_crypt_msg: *mut c_void,
    dw_param_type: u32,
    dw_index: u32,
    pv_data: *mut c_void,
    pcb_data: *mut u32,
) -> BOOL {
    if h_crypt_msg.is_null() || pcb_data.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let st = CRYPT_STATE.lock().unwrap();
    let Some(msg) = st.msgs.get(&(h_crypt_msg as usize)) else {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };

    let write_bytes = |src: &[u8]| -> BOOL {
        let need = src.len() as u32;
        if pv_data.is_null() {
            std::ptr::write_unaligned(pcb_data, need);
            return TRUE;
        }
        let cap = std::ptr::read_unaligned(pcb_data);
        if cap < need {
            std::ptr::write_unaligned(pcb_data, need);
            crate::teb::set_last_error(err::ERROR_MORE_DATA);
            return FALSE;
        }
        std::ptr::copy_nonoverlapping(src.as_ptr(), pv_data as *mut u8, src.len());
        std::ptr::write_unaligned(pcb_data, need);
        TRUE
    };

    match dw_param_type {
        CMSG_TYPE_PARAM => write_bytes(&CMSG_SIGNED.to_le_bytes()),
        CMSG_VERSION_PARAM => write_bytes(&1u32.to_le_bytes()),
        CMSG_SIGNER_COUNT_PARAM => write_bytes(&(msg.signers.len() as u32).to_le_bytes()),
        CMSG_CERT_COUNT_PARAM => write_bytes(&(msg.certs_der.len() as u32).to_le_bytes()),
        CMSG_CERT_PARAM => {
            let Some(der) = msg.certs_der.get(dw_index as usize) else {
                crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                return FALSE;
            };
            write_bytes(der)
        }
        CMSG_SIGNER_INFO_PARAM => {
            let Some(signer) = msg.signers.get(dw_index as usize) else {
                crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                return FALSE;
            };
            let mut serial_le = signer.serial_be.clone();
            serial_le.reverse();
            let hash_oid = signer.hash_alg_oid_str.as_bytes();
            let enc_oid = signer.enc_alg_oid_str.as_bytes();
            let hdr_sz = std::mem::size_of::<CmsgSignerInfo>();
            let total = hdr_sz
                + signer.issuer_raw.len()
                + serial_le.len()
                + hash_oid.len()
                + 1
                + enc_oid.len()
                + 1
                + signer.encrypted_hash.len();
            let need = total as u32;
            if pv_data.is_null() {
                std::ptr::write_unaligned(pcb_data, need);
                return TRUE;
            }
            let cap = std::ptr::read_unaligned(pcb_data);
            if cap < need {
                std::ptr::write_unaligned(pcb_data, need);
                crate::teb::set_last_error(err::ERROR_MORE_DATA);
                return FALSE;
            }
            let base = pv_data as *mut u8;
            let mut off = hdr_sz;

            let issuer_ptr = base.add(off);
            std::ptr::copy_nonoverlapping(signer.issuer_raw.as_ptr(), issuer_ptr, signer.issuer_raw.len());
            off += signer.issuer_raw.len();

            let serial_ptr = base.add(off);
            std::ptr::copy_nonoverlapping(serial_le.as_ptr(), serial_ptr, serial_le.len());
            off += serial_le.len();

            let hash_oid_ptr = base.add(off);
            std::ptr::copy_nonoverlapping(hash_oid.as_ptr(), hash_oid_ptr, hash_oid.len());
            std::ptr::write_unaligned(hash_oid_ptr.add(hash_oid.len()), 0);
            off += hash_oid.len() + 1;

            let enc_oid_ptr = base.add(off);
            std::ptr::copy_nonoverlapping(enc_oid.as_ptr(), enc_oid_ptr, enc_oid.len());
            std::ptr::write_unaligned(enc_oid_ptr.add(enc_oid.len()), 0);
            off += enc_oid.len() + 1;

            let enc_hash_ptr = base.add(off);
            if !signer.encrypted_hash.is_empty() {
                std::ptr::copy_nonoverlapping(
                    signer.encrypted_hash.as_ptr(),
                    enc_hash_ptr,
                    signer.encrypted_hash.len(),
                );
            }

            let info = CmsgSignerInfo {
                dw_version: signer.version,
                issuer: CryptDataBlob {
                    cb_data: signer.issuer_raw.len() as u32,
                    pb_data: issuer_ptr,
                },
                serial_number: CryptDataBlob {
                    cb_data: serial_le.len() as u32,
                    pb_data: serial_ptr,
                },
                hash_algorithm: CryptAlgId {
                    psz_obj_id: hash_oid_ptr,
                    parameters: CryptDataBlob {
                        cb_data: 0,
                        pb_data: std::ptr::null(),
                    },
                },
                hash_encryption_algorithm: CryptAlgId {
                    psz_obj_id: enc_oid_ptr,
                    parameters: CryptDataBlob {
                        cb_data: 0,
                        pb_data: std::ptr::null(),
                    },
                },
                encrypted_hash: CryptDataBlob {
                    cb_data: signer.encrypted_hash.len() as u32,
                    pb_data: if signer.encrypted_hash.is_empty() {
                        std::ptr::null()
                    } else {
                        enc_hash_ptr
                    },
                },
                auth_attrs_count: 0,
                auth_attrs_ptr: std::ptr::null_mut(),
                unauth_attrs_count: 0,
                unauth_attrs_ptr: std::ptr::null_mut(),
            };
            std::ptr::write_unaligned(pv_data as *mut CmsgSignerInfo, info);
            std::ptr::write_unaligned(pcb_data, need);
            TRUE
        }
        _ => {
            stub!("CryptMsgGetParam(param={dw_param_type}, idx={dw_index}) -> FALSE");
            crate::teb::set_last_error(ERROR_CALL_NOT_IMPLEMENTED);
            FALSE
        }
    }
}

pub unsafe extern "win64" fn CryptMemAlloc(cb: u32) -> *mut c_void {
    crate::heap::alloc(cb as usize, false)
}

pub unsafe extern "win64" fn CryptMemRealloc(p: *mut c_void, cb: u32) -> *mut c_void {
    if p.is_null() {
        crate::heap::alloc(cb as usize, false)
    } else {
        crate::heap::realloc(p, cb as usize, false, false)
    }
}

pub unsafe extern "win64" fn CryptMemFree(p: *mut c_void) {
    if !p.is_null() {
        crate::heap::free(p);
    }
}

pub unsafe extern "win64" fn CertVerifyTimeValidity(p_time_to_verify: *const c_void, p_cert_info: *const c_void) -> i32 {
    if p_cert_info.is_null() {
        return 0;
    }
    let ci = std::ptr::read_unaligned(p_cert_info as *const CertInfo);
    let t = if p_time_to_verify.is_null() {
        now_filetime()
    } else {
        std::ptr::read_unaligned(p_time_to_verify as *const u64)
    };
    if ci.not_before > 0 && t < ci.not_before {
        -1
    } else if ci.not_after > 0 && t > ci.not_after {
        1
    } else {
        0
    }
}

pub unsafe extern "win64" fn CertFreeCRLContext(ctx: *mut c_void) -> BOOL {
    if !ctx.is_null() {
        stub!("CertFreeCRLContext({ctx:p}): foreign CRL context ignored");
    }
    TRUE
}

pub unsafe extern "win64" fn CertFreeCTLContext(ctx: *mut c_void) -> BOOL {
    if !ctx.is_null() {
        stub!("CertFreeCTLContext({ctx:p}): foreign CTL context ignored");
    }
    TRUE
}

unsafe fn cert_get_name(ctx: *mut c_void, ty: u32, flags: u32, para: *mut c_void) -> String {
    const CERT_NAME_RDN_TYPE: u32 = 2;
    const CERT_NAME_ATTR_TYPE: u32 = 3;
    const CERT_NAME_SIMPLE_DISPLAY_TYPE: u32 = 4;
    const CERT_NAME_FRIENDLY_DISPLAY_TYPE: u32 = 5;
    const CERT_NAME_ISSUER_FLAG: u32 = 0x1;

    if ctx.is_null() {
        return String::new();
    }
    let cc = std::ptr::read_unaligned(ctx as *const super::wintrust::CertContext);
    if cc.p_cert_info.is_null() {
        return String::new();
    }
    let ci = std::ptr::read_unaligned(cc.p_cert_info);
    let blob = if (flags & CERT_NAME_ISSUER_FLAG) != 0 { ci.issuer } else { ci.subject };
    if blob.pb_data.is_null() || blob.cb_data == 0 {
        return String::new();
    }
    let der = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
    match ty {
        CERT_NAME_RDN_TYPE => super::wintrust::format_dn(der).unwrap_or_default(),
        CERT_NAME_ATTR_TYPE => {
            if para.is_null() {
                super::wintrust::extract_dn_attr(der, &[0x55, 0x04, 0x03]).unwrap_or_default()
            } else {
                let oid_s = crate::strings::from_ansi(para as *const u8);
                super::wintrust::encode_oid_str(&oid_s)
                    .and_then(|oid| super::wintrust::extract_dn_attr(der, &oid))
                    .unwrap_or_default()
            }
        }
        CERT_NAME_SIMPLE_DISPLAY_TYPE | CERT_NAME_FRIENDLY_DISPLAY_TYPE => {
            // CN (2.5.4.3), then OU (2.5.4.11), then O (2.5.4.10), then E (1.2.840.113549.1.9.1)
            for oid in [
                &[0x55u8, 0x04, 0x03][..],
                &[0x55, 0x04, 0x0B],
                &[0x55, 0x04, 0x0A],
                &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x01],
            ] {
                if let Some(v) = super::wintrust::extract_dn_attr(der, oid) {
                    return v;
                }
            }
            String::new()
        }
        _ => String::new(),
    }
}

pub unsafe extern "win64" fn CertGetNameStringW(ctx: *mut c_void, ty: u32, flags: u32, para: *mut c_void, buf: *mut u16, cch: u32) -> u32 {
    let s = cert_get_name(ctx, ty, flags, para);
    write_wide_out(&s, buf, cch)
}

pub unsafe extern "win64" fn CertGetNameStringA(ctx: *mut c_void, ty: u32, flags: u32, para: *mut c_void, buf: *mut u8, cch: u32) -> u32 {
    let s = cert_get_name(ctx, ty, flags, para);
    write_ansi_out(&s, buf, cch)
}

const CRYPT_STRING_BASE64HEADER: u32 = 0x0000_0000;
const CRYPT_STRING_BASE64: u32 = 0x0000_0001;
const CRYPT_STRING_BINARY: u32 = 0x0000_0002;
const CRYPT_STRING_BASE64REQUESTHEADER: u32 = 0x0000_0003;
const CRYPT_STRING_HEX: u32 = 0x0000_0004;
const CRYPT_STRING_HEXASCII: u32 = 0x0000_0005;
const CRYPT_STRING_BASE64_ANY: u32 = 0x0000_0006;
const CRYPT_STRING_ANY: u32 = 0x0000_0007;
const CRYPT_STRING_HEX_ANY: u32 = 0x0000_0008;
const CRYPT_STRING_BASE64X509CRLHEADER: u32 = 0x0000_0009;
const CRYPT_STRING_HEXADDR: u32 = 0x0000_000A;
const CRYPT_STRING_HEXASCIIADDR: u32 = 0x0000_000B;
const CRYPT_STRING_HEXRAW: u32 = 0x0000_000C;
const CRYPT_STRING_BASE64URI: u32 = 0x0000_000D;
const CRYPT_STRING_STRICT: u32 = 0x2000_0000;

fn decode_b64_bytes_ext(s: &str, url_safe: bool, strict: bool) -> Option<Vec<u8>> {
    let val = |b: u8| -> Option<u8> {
        match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' if !url_safe => Some(62),
            b'/' if !url_safe => Some(63),
            b'-' if url_safe => Some(62),
            b'_' if url_safe => Some(63),
            _ => None,
        }
    };
    let mut out = Vec::new();
    let mut buf = [0u8; 4];
    let mut n = 0usize;
    let mut pads = 0usize;
    for b in s.bytes() {
        if b.is_ascii_whitespace() {
            if strict && url_safe {
                return None;
            }
            continue;
        }
        if b == b'=' {
            if pads >= 2 || n + pads >= 4 || n < 2 {
                return None;
            }
            buf[n + pads] = 0;
            pads += 1;
        } else if let Some(v) = val(b) {
            if pads != 0 {
                return None;
            }
            buf[n] = v;
            n += 1;
        } else {
            return None;
        }
        if n + pads == 4 {
            let word = ((buf[0] as u32) << 18)
                | ((buf[1] as u32) << 12)
                | ((buf[2] as u32) << 6)
                | (buf[3] as u32);
            out.push((word >> 16) as u8);
            if pads < 2 {
                out.push((word >> 8) as u8);
            }
            if pads == 0 {
                out.push(word as u8);
            }
            n = 0;
            pads = 0;
        }
    }
    if n == 0 && pads == 0 {
        return Some(out);
    }
    // Base64url (and non-strict Base64) allows omitted trailing '=' padding.
    if (url_safe || !strict) && pads == 0 && (n == 2 || n == 3) {
        let word = ((buf[0] as u32) << 18)
            | ((buf[1] as u32) << 12)
            | (if n == 3 { (buf[2] as u32) << 6 } else { 0 });
        out.push((word >> 16) as u8);
        if n == 3 {
            out.push((word >> 8) as u8);
        }
        return Some(out);
    }
    None
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn decode_hex_bytes(s: &str, raw_only: bool, strict: bool) -> Option<Vec<u8>> {
    let mut nibbles = Vec::new();
    for b in s.bytes() {
        if b.is_ascii_whitespace() {
            if raw_only && strict {
                return None;
            }
            continue;
        }
        if !raw_only && (b == b'-' || b == b':') {
            continue;
        }
        nibbles.push(hex_nibble(b)?);
    }
    if nibbles.is_empty() || nibbles.len() % 2 != 0 {
        return None;
    }
    Some(nibbles.chunks_exact(2).map(|c| (c[0] << 4) | c[1]).collect())
}

fn decode_hex_dump(s: &str, has_addr: bool, has_ascii: bool) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut any_line = false;
    for raw_line in s.lines() {
        let line = raw_line.trim_end_matches('\r');
        if line.trim().is_empty() {
            continue;
        }
        any_line = true;
        let mut work = line.trim_start();
        if has_addr {
            // Strip leading hex address token (at least 4 hex digits, optional trailing ':', followed by whitespace)
            let addr_end = work
                .find(|c: char| c.is_ascii_whitespace() || c == ':')
                .unwrap_or(0);
            if addr_end < 4 || !work[..addr_end].bytes().all(|b| hex_nibble(b).is_some()) {
                return None;
            }
            work = &work[addr_end..];
            if let Some(rest) = work.strip_prefix(':') {
                work = rest;
            }
            if !work.starts_with(|c: char| c.is_ascii_whitespace()) {
                return None;
            }
            work = work.trim_start();
        }
        if has_ascii {
            // Split off trailing ASCII column separated by >= 2 spaces or a tab after hex bytes.
            let bytes_part = if let Some(pos) = work.find("   ") {
                &work[..pos]
            } else if let Some(pos) = work.find("  ") {
                &work[..pos]
            } else if let Some(pos) = work.find('\t') {
                &work[..pos]
            } else {
                work
            };
            work = bytes_part;
        }
        let mut line_bytes = 0usize;
        for tok in work.split(|c: char| c.is_ascii_whitespace() || c == '-') {
            if tok.is_empty() {
                continue;
            }
            if tok.len() % 2 != 0 {
                return None;
            }
            let tb = tok.as_bytes();
            for pair in tb.chunks_exact(2) {
                let hi = hex_nibble(pair[0])?;
                let lo = hex_nibble(pair[1])?;
                out.push((hi << 4) | lo);
                line_bytes += 1;
            }
        }
        if line_bytes == 0 {
            return None;
        }
    }
    (any_line && !out.is_empty()).then_some(out)
}

fn extract_pem_body_with_label(s: &str) -> Option<(usize, &str, &str)> {
    let start_hdr = s.find("-----BEGIN ")?;
    let label_start = start_hdr + "-----BEGIN ".len();
    let label_rel_end = s[label_start..].find("-----")?;
    let label = s[label_start..label_start + label_rel_end].trim();
    let body_start = label_start + label_rel_end + 5;
    let end_marker_rel = s[body_start..].find("-----END ")?;
    let end_after = body_start + end_marker_rel + "-----END ".len();
    let end_close_rel = s[end_after..].find("-----")?;
    let end_label = s[end_after..end_after + end_close_rel].trim();
    if !end_label.eq_ignore_ascii_case(label) {
        return None;
    }
    Some((start_hdr, label, &s[body_start..body_start + end_marker_rel]))
}

fn pem_flag_for_label(label: &str) -> u32 {
    let up = label.to_ascii_uppercase();
    if up.contains("REQUEST") {
        CRYPT_STRING_BASE64REQUESTHEADER
    } else if up.contains("CRL") {
        CRYPT_STRING_BASE64X509CRLHEADER
    } else {
        CRYPT_STRING_BASE64HEADER
    }
}

unsafe fn crypt_string_to_binary_impl(
    text: &str,
    raw_bytes_for_binary: &[u8],
    flags: u32,
    pb_binary: *mut u8,
    pcb_binary: *mut u32,
    pdw_skip: *mut u32,
    pdw_flags: *mut u32,
) -> BOOL {
    if pcb_binary.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let strict = (flags & CRYPT_STRING_STRICT) != 0;
    let base_flags = flags & 0xFFFF;
    let try_pem = |expected: Option<u32>| -> Option<(Vec<u8>, u32, u32)> {
        let (skip, label, body) = extract_pem_body_with_label(text)?;
        let kind = pem_flag_for_label(label);
        if let Some(exp) = expected {
            if exp == CRYPT_STRING_BASE64REQUESTHEADER && kind != CRYPT_STRING_BASE64REQUESTHEADER {
                return None;
            }
            if exp == CRYPT_STRING_BASE64X509CRLHEADER && kind != CRYPT_STRING_BASE64X509CRLHEADER {
                return None;
            }
        }
        let b = decode_b64_bytes_ext(body, false, strict)?;
        Some((b, skip as u32, expected.unwrap_or(kind)))
    };

    let decoded: Option<(Vec<u8>, u32, u32)> = match base_flags {
        CRYPT_STRING_BASE64HEADER => try_pem(Some(CRYPT_STRING_BASE64HEADER)),
        CRYPT_STRING_BASE64REQUESTHEADER => try_pem(Some(CRYPT_STRING_BASE64REQUESTHEADER)),
        CRYPT_STRING_BASE64X509CRLHEADER => try_pem(Some(CRYPT_STRING_BASE64X509CRLHEADER)),
        CRYPT_STRING_BASE64 => {
            decode_b64_bytes_ext(text, false, strict).map(|b| (b, 0, CRYPT_STRING_BASE64))
        }
        CRYPT_STRING_BASE64URI => {
            decode_b64_bytes_ext(text, true, strict).map(|b| (b, 0, CRYPT_STRING_BASE64URI))
        }
        CRYPT_STRING_BINARY => Some((raw_bytes_for_binary.to_vec(), 0, CRYPT_STRING_BINARY)),
        CRYPT_STRING_BASE64_ANY => try_pem(None)
            .or_else(|| decode_b64_bytes_ext(text, false, strict).map(|b| (b, 0, CRYPT_STRING_BASE64))),
        CRYPT_STRING_HEX => {
            decode_hex_bytes(text, false, strict).map(|b| (b, 0, CRYPT_STRING_HEX))
        }
        CRYPT_STRING_HEXRAW => {
            decode_hex_bytes(text, true, strict).map(|b| (b, 0, CRYPT_STRING_HEXRAW))
        }
        CRYPT_STRING_HEXASCII => {
            decode_hex_dump(text, false, true).map(|b| (b, 0, CRYPT_STRING_HEXASCII))
        }
        CRYPT_STRING_HEXADDR => {
            decode_hex_dump(text, true, false).map(|b| (b, 0, CRYPT_STRING_HEXADDR))
        }
        CRYPT_STRING_HEXASCIIADDR => {
            decode_hex_dump(text, true, true).map(|b| (b, 0, CRYPT_STRING_HEXASCIIADDR))
        }
        CRYPT_STRING_HEX_ANY => decode_hex_dump(text, true, true)
            .map(|b| (b, 0, CRYPT_STRING_HEXASCIIADDR))
            .or_else(|| decode_hex_dump(text, true, false).map(|b| (b, 0, CRYPT_STRING_HEXADDR)))
            .or_else(|| decode_hex_bytes(text, true, false).map(|b| (b, 0, CRYPT_STRING_HEXRAW)))
            .or_else(|| decode_hex_bytes(text, false, false).map(|b| (b, 0, CRYPT_STRING_HEX)))
            .or_else(|| decode_hex_dump(text, false, true).map(|b| (b, 0, CRYPT_STRING_HEXASCII))),
        CRYPT_STRING_ANY => try_pem(None)
            .or_else(|| decode_b64_bytes_ext(text, false, strict).map(|b| (b, 0, CRYPT_STRING_BASE64)))
            .or_else(|| decode_hex_dump(text, true, true).map(|b| (b, 0, CRYPT_STRING_HEXASCIIADDR)))
            .or_else(|| decode_hex_dump(text, true, false).map(|b| (b, 0, CRYPT_STRING_HEXADDR)))
            .or_else(|| decode_hex_bytes(text, false, false).map(|b| (b, 0, CRYPT_STRING_HEX)))
            .or_else(|| decode_hex_dump(text, false, true).map(|b| (b, 0, CRYPT_STRING_HEXASCII))),
        _ => {
            stub!("CryptStringToBinary unsupported flags {flags:#x}");
            None
        }
    };
    let Some((bytes, skip, actual_flags)) = decoded else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    };
    let need = bytes.len() as u32;
    if pb_binary.is_null() {
        std::ptr::write_unaligned(pcb_binary, need);
        if !pdw_skip.is_null() {
            std::ptr::write_unaligned(pdw_skip, skip);
        }
        if !pdw_flags.is_null() {
            std::ptr::write_unaligned(pdw_flags, actual_flags);
        }
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pcb_binary);
    if cap < need {
        std::ptr::write_unaligned(pcb_binary, need);
        crate::teb::set_last_error(err::ERROR_MORE_DATA);
        return FALSE;
    }
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), pb_binary, bytes.len());
    }
    std::ptr::write_unaligned(pcb_binary, need);
    if !pdw_skip.is_null() {
        std::ptr::write_unaligned(pdw_skip, skip);
    }
    if !pdw_flags.is_null() {
        std::ptr::write_unaligned(pdw_flags, actual_flags);
    }
    TRUE
}

pub unsafe extern "win64" fn CryptStringToBinaryA(
    psz_string: LPCSTR,
    cch_string: u32,
    dw_flags: u32,
    pb_binary: *mut u8,
    pcb_binary: *mut u32,
    pdw_skip: *mut u32,
    pdw_flags: *mut u32,
) -> BOOL {
    if psz_string.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (s, raw): (String, Vec<u8>) = if cch_string == 0 {
        let s = crate::strings::from_ansi(psz_string);
        let raw = s.as_bytes().to_vec();
        (s, raw)
    } else {
        let slice = std::slice::from_raw_parts(psz_string, cch_string as usize);
        (String::from_utf8_lossy(slice).into_owned(), slice.to_vec())
    };
    crypt_string_to_binary_impl(&s, &raw, dw_flags, pb_binary, pcb_binary, pdw_skip, pdw_flags)
}

pub unsafe extern "win64" fn CryptStringToBinaryW(
    psz_string: LPCWSTR,
    cch_string: u32,
    dw_flags: u32,
    pb_binary: *mut u8,
    pcb_binary: *mut u32,
    pdw_skip: *mut u32,
    pdw_flags: *mut u32,
) -> BOOL {
    if psz_string.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let s = if cch_string == 0 {
        crate::strings::from_wide(psz_string)
    } else {
        let mut tmp = Vec::with_capacity(cch_string as usize);
        for i in 0..cch_string as usize {
            tmp.push(std::ptr::read_unaligned(psz_string.add(i)));
        }
        String::from_utf16_lossy(&tmp)
    };
    let raw = s.as_bytes().to_vec();
    crypt_string_to_binary_impl(&s, &raw, dw_flags, pb_binary, pcb_binary, pdw_skip, pdw_flags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypt_string_to_binary_base64_and_pem() {
        unsafe {
            let b64 = b"SGVsbG8gV29ybGQ=\0";
            let mut len = 0u32;
            assert_eq!(
                CryptStringToBinaryA(b64.as_ptr(), 0, CRYPT_STRING_BASE64, std::ptr::null_mut(), &mut len, std::ptr::null_mut(), std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(len, 11);
            let mut out = vec![0u8; len as usize];
            assert_eq!(
                CryptStringToBinaryA(b64.as_ptr(), 0, CRYPT_STRING_BASE64, out.as_mut_ptr(), &mut len, std::ptr::null_mut(), std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(&out, b"Hello World");

            let pem = b"-----BEGIN CERTIFICATE-----\nSGVsbG8=\n-----END CERTIFICATE-----\0";
            let mut plen = 0u32;
            let mut pflags = 0u32;
            assert_eq!(
                CryptStringToBinaryA(pem.as_ptr(), 0, CRYPT_STRING_BASE64_ANY, std::ptr::null_mut(), &mut plen, std::ptr::null_mut(), &mut pflags),
                TRUE
            );
            assert_eq!(plen, 5);
            assert_eq!(pflags, CRYPT_STRING_BASE64HEADER);
            let mut pout = vec![0u8; plen as usize];
            assert_eq!(
                CryptStringToBinaryA(pem.as_ptr(), 0, CRYPT_STRING_BASE64_ANY, pout.as_mut_ptr(), &mut plen, std::ptr::null_mut(), std::ptr::null_mut()),
                TRUE
            );
            assert_eq!(&pout, b"Hello");
        }
    }

    #[test]
    fn abi_layouts_match_win64() {
        assert_eq!(std::mem::size_of::<CertChainPolicyPara>(), 16);
        assert_eq!(std::mem::offset_of!(CertChainPolicyPara, pv_extra_policy_para), 8);
        assert_eq!(std::mem::size_of::<CertChainPolicyStatus>(), 24);
        assert_eq!(std::mem::offset_of!(CertChainPolicyStatus, dw_error), 4);
        assert_eq!(std::mem::size_of::<SslExtraCertChainPolicyPara>(), 24);
        assert_eq!(std::mem::offset_of!(SslExtraCertChainPolicyPara, fdw_checks), 8);
        assert_eq!(std::mem::offset_of!(SslExtraCertChainPolicyPara, pwsz_server_name), 16);
        assert_eq!(std::mem::size_of::<CmsgSignerInfo>(), 0x88);
    }

    #[test]
    fn crypt_query_object_and_signer_lookup() {
        fn der(tag: u8, val: &[u8]) -> Vec<u8> {
            encode_der_tlv(tag, val)
        }
        // Build a synthetic X.509 cert (CN="Test Publisher", serial=0x01020304)
        let cn_atv = der(0x30, &[der(0x06, &[0x55, 0x04, 0x03]), der(0x0C, b"Test Publisher")].concat());
        let dn = der(0x30, &der(0x31, &cn_atv));
        let alg_sha256_rsa = der(
            0x30,
            &[der(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B]), der(0x05, &[])].concat(),
        );
        let validity = der(0x30, &[der(0x17, b"250101000000Z"), der(0x17, b"350101000000Z")].concat());
        let spki = der(
            0x30,
            &[
                der(0x30, &[der(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01]), der(0x05, &[])].concat()),
                der(0x03, &[0x00, 0x30, 0x06, 0x02, 0x01, 0x05, 0x02, 0x01, 0x03]),
            ]
            .concat(),
        );
        let serial_der = der(0x02, &[0x01, 0x02, 0x03, 0x04]);
        let tbs = der(
            0x30,
            &[
                der(0xA0, &der(0x02, &[0x02])),
                serial_der.clone(),
                alg_sha256_rsa.clone(),
                dn.clone(),
                validity,
                dn.clone(),
                spki,
            ]
            .concat(),
        );
        let cert_der = der(0x30, &[tbs, alg_sha256_rsa, der(0x03, &[0x00, 0xAA, 0xBB])].concat());

        // Build PKCS#7 SignedData ContentInfo containing cert_der + SignerInfo
        let sha256_alg = der(0x30, &der(0x06, &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01]));
        let rsa_alg = der(0x30, &der(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01]));
        let signer_info = der(
            0x30,
            &[
                der(0x02, &[0x01]),
                der(0x30, &[dn.clone(), serial_der].concat()),
                sha256_alg.clone(),
                rsa_alg,
                der(0x04, &[0xDE, 0xAD, 0xBE, 0xEF]),
            ]
            .concat(),
        );
        let signed_data = der(
            0x30,
            &[
                der(0x02, &[0x01]),
                der(0x31, &sha256_alg),
                der(0x30, &der(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x01])),
                der(0xA0, &cert_der),
                der(0x31, &signer_info),
            ]
            .concat(),
        );
        let pkcs7 = der(
            0x30,
            &[
                der(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02]),
                der(0xA0, &signed_data),
            ]
            .concat(),
        );

        // Embed PKCS#7 inside a minimal PE32+ image security directory
        let mut pe = vec![0u8; 0x200];
        pe[0..2].copy_from_slice(b"MZ");
        pe[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        pe[0x80..0x84].copy_from_slice(b"PE\0\0");
        pe[0x98..0x9A].copy_from_slice(&0x20Bu16.to_le_bytes());
        let win_cert_len = (8 + pkcs7.len()) as u32;
        let sec_off = 0x200u32;
        let sec_dir_off = 0x98 + 112 + 4 * 8;
        pe[sec_dir_off..sec_dir_off + 4].copy_from_slice(&sec_off.to_le_bytes());
        pe[sec_dir_off + 4..sec_dir_off + 8].copy_from_slice(&win_cert_len.to_le_bytes());
        pe.extend_from_slice(&win_cert_len.to_le_bytes());
        pe.extend_from_slice(&0x0200u16.to_le_bytes()); // WIN_CERT_REVISION_2_0
        pe.extend_from_slice(&0x0002u16.to_le_bytes()); // WIN_CERT_TYPE_PKCS_SIGNED_DATA
        pe.extend_from_slice(&pkcs7);

        unsafe {
            let blob = CryptDataBlob {
                cb_data: pe.len() as u32,
                pb_data: pe.as_ptr(),
            };
            let mut enc = 0u32;
            let mut content = 0u32;
            let mut format = 0u32;
            let mut h_store = std::ptr::null_mut();
            let mut h_msg = std::ptr::null_mut();
            assert_eq!(
                CryptQueryObject(
                    CERT_QUERY_OBJECT_BLOB,
                    &blob as *const CryptDataBlob as *const c_void,
                    1u32 << CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED,
                    CERT_QUERY_FORMAT_FLAG_BINARY,
                    0,
                    &mut enc,
                    &mut content,
                    &mut format,
                    &mut h_store,
                    &mut h_msg,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            assert_eq!(enc, X509_ASN_ENCODING | PKCS_7_ASN_ENCODING);
            assert_eq!(content, CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED);
            assert_eq!(format, CERT_QUERY_FORMAT_BINARY);
            assert!(!h_store.is_null());
            assert!(!h_msg.is_null());

            let mut signer_count = 0u32;
            let mut cb = 4u32;
            assert_eq!(
                CryptMsgGetParam(
                    h_msg,
                    CMSG_SIGNER_COUNT_PARAM,
                    0,
                    &mut signer_count as *mut u32 as *mut c_void,
                    &mut cb,
                ),
                TRUE
            );
            assert_eq!(signer_count, 1);

            let mut si_len = 0u32;
            assert_eq!(
                CryptMsgGetParam(h_msg, CMSG_SIGNER_INFO_PARAM, 0, std::ptr::null_mut(), &mut si_len),
                TRUE
            );
            assert!(si_len as usize > std::mem::size_of::<CmsgSignerInfo>());
            let mut si_buf = vec![0u8; si_len as usize];
            assert_eq!(
                CryptMsgGetParam(
                    h_msg,
                    CMSG_SIGNER_INFO_PARAM,
                    0,
                    si_buf.as_mut_ptr() as *mut c_void,
                    &mut si_len,
                ),
                TRUE
            );
            let si = std::ptr::read_unaligned(si_buf.as_ptr() as *const CmsgSignerInfo);
            assert_eq!(crate::strings::from_ansi(si.hash_algorithm.psz_obj_id), "2.16.840.1.101.3.4.2.1");

            // KB 323809 lookup via CertFindCertificateInStore(CERT_FIND_SUBJECT_CERT)
            let mut query_ci: CertInfo = std::mem::zeroed();
            query_ci.issuer = si.issuer;
            query_ci.serial_number = si.serial_number;
            let found = CertFindCertificateInStore(
                h_store,
                enc,
                0,
                CERT_FIND_SUBJECT_CERT,
                &query_ci as *const CertInfo as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!found.is_null());
            let mut wbuf = [0u16; 64];
            let n = CertGetNameStringW(found, 4, 0, std::ptr::null_mut(), wbuf.as_mut_ptr(), 64);
            assert!(n > 1);
            assert_eq!(crate::strings::from_wide(wbuf.as_ptr()), "Test Publisher");

            // Also verify subject string search
            let needle = crate::strings::to_wide_z("publisher");
            let found2 = CertFindCertificateInStore(
                h_store,
                enc,
                0,
                CERT_FIND_SUBJECT_STR_W,
                needle.as_ptr() as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!found2.is_null());
            assert_eq!(CertFreeCertificateContext(found2), TRUE);
            assert_eq!(CertFreeCertificateContext(found), TRUE);
            assert_eq!(CertCloseStore(h_store, 0), TRUE);
            assert_eq!(CryptMsgClose(h_msg), TRUE);

            // Honest failure on unsigned blob
            let unsigned = CryptDataBlob { cb_data: 16, pb_data: b"unsigned payload".as_ptr() };
            assert_eq!(
                CryptQueryObject(
                    CERT_QUERY_OBJECT_BLOB,
                    &unsigned as *const CryptDataBlob as *const c_void,
                    1u32 << CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED,
                    CERT_QUERY_FORMAT_FLAG_BINARY,
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                FALSE
            );
        }
    }

    fn build_synthetic_cert(cn: &str, issuer_cn: &str, not_before: &[u8], not_after: &[u8], san_dns: Option<&str>, eku_oids: &[&[u8]]) -> Vec<u8> {
        let mk_dn = |s: &str| {
            let atv = encode_der_tlv(0x30, &[encode_der_tlv(0x06, &[0x55, 0x04, 0x03]), encode_der_tlv(0x0C, s.as_bytes())].concat());
            encode_der_tlv(0x30, &encode_der_tlv(0x31, &atv))
        };
        let alg = encode_der_tlv(
            0x30,
            &[encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B]), encode_der_tlv(0x05, &[])].concat(),
        );
        let validity = encode_der_tlv(0x30, &[encode_der_tlv(0x17, not_before), encode_der_tlv(0x17, not_after)].concat());
        let spki = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x30, &[encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01]), encode_der_tlv(0x05, &[])].concat()),
                encode_der_tlv(0x03, &[0x00, 0x30, 0x06, 0x02, 0x01, 0x05, 0x02, 0x01, 0x03]),
            ]
            .concat(),
        );
        let mut exts = Vec::new();
        if let Some(dns) = san_dns {
            let san_val = encode_der_tlv(0x30, &encode_der_tlv(0x82, dns.as_bytes()));
            let ext = encode_der_tlv(0x30, &[encode_der_tlv(0x06, &[0x55, 0x1D, 0x11]), encode_der_tlv(0x04, &san_val)].concat());
            exts.extend_from_slice(&ext);
        }
        if !eku_oids.is_empty() {
            let mut inner = Vec::new();
            for oid in eku_oids {
                inner.extend_from_slice(&encode_der_tlv(0x06, oid));
            }
            let eku_val = encode_der_tlv(0x30, &inner);
            let ext = encode_der_tlv(0x30, &[encode_der_tlv(0x06, &[0x55, 0x1D, 0x25]), encode_der_tlv(0x04, &eku_val)].concat());
            exts.extend_from_slice(&ext);
        }
        let mut tbs_parts = vec![
            encode_der_tlv(0xA0, &encode_der_tlv(0x02, &[0x02])),
            encode_der_tlv(0x02, &[0x01, 0x23, 0x45]),
            alg.clone(),
            mk_dn(issuer_cn),
            validity,
            mk_dn(cn),
            spki,
        ];
        if !exts.is_empty() {
            tbs_parts.push(encode_der_tlv(0xA3, &encode_der_tlv(0x30, &exts)));
        }
        let tbs = encode_der_tlv(0x30, &tbs_parts.concat());
        encode_der_tlv(0x30, &[tbs, alg, encode_der_tlv(0x03, &[0x00, 0x11, 0x22, 0x33])].concat())
    }

    #[test]
    fn cert_open_store_system_and_find_all_types() {
        unsafe {
            // Open SYSTEM_A("ROOT"), SYSTEM_W("CA"), SYSTEM_REGISTRY_W("ROOT"), and SYSTEM_A("MY")
            let root_a = CertOpenStore(
                CERT_STORE_PROV_SYSTEM_A as *const u8,
                X509_ASN_ENCODING,
                0,
                CERT_SYSTEM_STORE_CURRENT_USER,
                b"ROOT\0".as_ptr() as *const c_void,
            );
            assert!(!root_a.is_null());

            let ca_w_name = crate::strings::to_wide_z("CA");
            let ca_w = CertOpenStore(
                CERT_STORE_PROV_SYSTEM_W as *const u8,
                X509_ASN_ENCODING,
                0,
                CERT_SYSTEM_STORE_CURRENT_USER,
                ca_w_name.as_ptr() as *const c_void,
            );
            assert!(!ca_w.is_null());

            let root_w_name = crate::strings::to_wide_z("ROOT");
            let root_reg = CertOpenStore(
                CERT_STORE_PROV_SYSTEM_REGISTRY_W as *const u8,
                X509_ASN_ENCODING,
                0,
                CERT_SYSTEM_STORE_CURRENT_USER,
                root_w_name.as_ptr() as *const c_void,
            );
            assert!(!root_reg.is_null());

            let my_store = CertOpenSystemStoreA(0, b"MY\0".as_ptr());
            assert!(!my_store.is_null());
            assert!(CertEnumCertificatesInStore(my_store, std::ptr::null_mut()).is_null());

            // Verify all webpki-roots are seeded & enumerable in ROOT
            let mut count = 0usize;
            let mut cur = std::ptr::null_mut();
            let mut first_der = Vec::new();
            loop {
                cur = CertEnumCertificatesInStore(root_a, cur);
                if cur.is_null() {
                    break;
                }
                if first_der.is_empty() {
                    let cc = std::ptr::read_unaligned(cur as *const CertContext);
                    first_der = std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize).to_vec();
                }
                count += 1;
            }
            assert_eq!(count, webpki_roots::TLS_SERVER_ROOTS.len());
            assert!(count > 100);

            // CERT_FIND_ANY
            let any_c = CertFindCertificateInStore(root_a, X509_ASN_ENCODING, 0, CERT_FIND_ANY, std::ptr::null(), std::ptr::null_mut());
            assert!(!any_c.is_null());

            // CERT_FIND_SUBJECT_STR_A & CERT_FIND_SUBJECT_STR_W (DigiCert is always in webpki-roots)
            let by_subj_a = CertFindCertificateInStore(
                root_a,
                X509_ASN_ENCODING,
                0,
                CERT_FIND_SUBJECT_STR_A,
                b"DigiCert\0".as_ptr() as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!by_subj_a.is_null());
            let digi_w = crate::strings::to_wide_z("DigiCert");
            let by_subj_w = CertFindCertificateInStore(
                ca_w,
                X509_ASN_ENCODING,
                0,
                CERT_FIND_SUBJECT_STR_W,
                digi_w.as_ptr() as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!by_subj_w.is_null());

            // CERT_FIND_ISSUER_STR_A & CERT_FIND_ISSUER_STR_W
            let by_iss_a = CertFindCertificateInStore(
                root_reg,
                X509_ASN_ENCODING,
                0,
                CERT_FIND_ISSUER_STR_A,
                b"DigiCert\0".as_ptr() as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!by_iss_a.is_null());
            let by_iss_w = CertFindCertificateInStore(
                root_reg,
                X509_ASN_ENCODING,
                0,
                CERT_FIND_ISSUER_STR_W,
                digi_w.as_ptr() as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!by_iss_w.is_null());

            // CERT_FIND_SHA1_HASH & CERT_FIND_MD5_HASH & CERT_FIND_EXISTING
            let sha1 = crate::misc::bcrypt::Digest::Sha1.hash(&first_der);
            let sha1_blob = CryptDataBlob { cb_data: sha1.len() as u32, pb_data: sha1.as_ptr() };
            let by_sha1 = CertFindCertificateInStore(
                root_a,
                X509_ASN_ENCODING,
                0,
                CERT_FIND_SHA1_HASH,
                &sha1_blob as *const _ as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!by_sha1.is_null());

            let md5 = crate::misc::bcrypt::Digest::Md5.hash(&first_der);
            let md5_blob = CryptDataBlob { cb_data: md5.len() as u32, pb_data: md5.as_ptr() };
            let by_md5 = CertFindCertificateInStore(
                root_a,
                X509_ASN_ENCODING,
                0,
                CERT_FIND_MD5_HASH,
                &md5_blob as *const _ as *const c_void,
                std::ptr::null_mut(),
            );
            assert!(!by_md5.is_null());

            let by_existing = CertFindCertificateInStore(
                root_a,
                X509_ASN_ENCODING,
                0,
                CERT_FIND_EXISTING,
                by_sha1,
                std::ptr::null_mut(),
            );
            assert!(!by_existing.is_null());

            // Add & delete from MY store
            let mut stored_ctx = std::ptr::null_mut();
            assert_eq!(CertAddCertificateContextToStore(my_store, by_sha1, CERT_STORE_ADD_NEW, &mut stored_ctx), TRUE);
            assert!(!stored_ctx.is_null());
            assert_eq!(CertDeleteCertificateFromStore(stored_ctx), TRUE);
            assert!(CertEnumCertificatesInStore(my_store, std::ptr::null_mut()).is_null());

            for h in [any_c, by_subj_a, by_subj_w, by_iss_a, by_iss_w, by_sha1, by_md5, by_existing] {
                assert_eq!(CertFreeCertificateContext(h), TRUE);
            }
            for s in [root_a, ca_w, root_reg, my_store] {
                assert_eq!(CertCloseStore(s, 0), TRUE);
            }
        }
    }

    #[test]
    fn cert_verify_policy_base_ssl_authenticode_validity_and_hostname() {
        unsafe {
            // 1. Expired leaf cert with wildcard SAN "*.example.org"
            let expired_der = build_synthetic_cert(
                "expired.example.org",
                "Test Untrusted CA",
                b"150101000000Z",
                b"200101000000Z",
                Some("*.example.org"),
                &[&[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x01]], // serverAuth
            );
            let exp_ctx = CertCreateCertificateContext(X509_ASN_ENCODING, expired_der.as_ptr(), expired_der.len() as u32);
            assert!(!exp_ctx.is_null());
            let cc = std::ptr::read_unaligned(exp_ctx as *const CertContext);
            assert_eq!(CertVerifyTimeValidity(std::ptr::null(), cc.p_cert_info as *const c_void), 1);

            let mut chain_exp = std::ptr::null_mut();
            assert_eq!(CertGetCertificateChain(std::ptr::null_mut(), exp_ctx, std::ptr::null(), std::ptr::null_mut(), std::ptr::null(), 0, std::ptr::null_mut(), &mut chain_exp), TRUE);

            // SSL policy with ALLOW_UNKNOWN_CA MUST still reject expired cert with CERT_E_EXPIRED!
            let host_ok = crate::strings::to_wide_z("api.example.org");
            let ssl_allow_ca = SslExtraCertChainPolicyPara {
                cb_size: std::mem::size_of::<SslExtraCertChainPolicyPara>() as u32,
                dw_auth_type: 2,
                fdw_checks: SECURITY_FLAG_IGNORE_UNKNOWN_CA,
                pwsz_server_name: host_ok.as_ptr(),
            };
            let para_allow_ca = CertChainPolicyPara {
                cb_size: std::mem::size_of::<CertChainPolicyPara>() as u32,
                dw_flags: 0,
                pv_extra_policy_para: &ssl_allow_ca as *const _ as *const c_void,
            };
            let mut st = CertChainPolicyStatus { cb_size: 24, dw_error: 0, l_chain_index: 0, l_element_index: 0, pv_extra_policy_status: std::ptr::null_mut() };
            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_SSL as *const u8, chain_exp, &para_allow_ca as *const _ as *const c_void, &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, CERT_E_EXPIRED);

            // SSL policy with IGNORE_UNKNOWN_CA + IGNORE_DATE + matching wildcard passes!
            let ssl_allow_both = SslExtraCertChainPolicyPara {
                cb_size: 24,
                dw_auth_type: 2,
                fdw_checks: SECURITY_FLAG_IGNORE_UNKNOWN_CA | SECURITY_FLAG_IGNORE_CERT_DATE_INVALID,
                pwsz_server_name: host_ok.as_ptr(),
            };
            let para_both = CertChainPolicyPara { cb_size: 16, dw_flags: 0, pv_extra_policy_para: &ssl_allow_both as *const _ as *const c_void };
            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_SSL as *const u8, chain_exp, &para_both as *const _ as *const c_void, &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, 0);

            // SSL policy with IGNORE_UNKNOWN_CA + IGNORE_DATE + WRONG hostname MUST fail with CERT_E_CN_NO_MATCH!
            let host_bad = crate::strings::to_wide_z("a.b.example.org"); // wildcard is single-label only
            let ssl_bad_host = SslExtraCertChainPolicyPara {
                cb_size: 24,
                dw_auth_type: 2,
                fdw_checks: SECURITY_FLAG_IGNORE_UNKNOWN_CA | SECURITY_FLAG_IGNORE_CERT_DATE_INVALID,
                pwsz_server_name: host_bad.as_ptr(),
            };
            let para_bad_host = CertChainPolicyPara { cb_size: 16, dw_flags: 0, pv_extra_policy_para: &ssl_bad_host as *const _ as *const c_void };
            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_SSL as *const u8, chain_exp, &para_bad_host as *const _ as *const c_void, &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, CERT_E_CN_NO_MATCH);

            // AUTHENTICODE policy on serverAuth-only cert MUST fail with CERT_E_WRONG_USAGE!
            let para_auth = CertChainPolicyPara { cb_size: 16, dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG | CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG, pv_extra_policy_para: std::ptr::null() };
            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_AUTHENTICODE as *const u8, chain_exp, &para_auth as *const _ as *const c_void, &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, CERT_E_WRONG_USAGE);

            // BASE policy fails with CERT_E_EXPIRED when date not ignored, passes when both CA & date ignored
            let para_base_ca_only = CertChainPolicyPara { cb_size: 16, dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG, pv_extra_policy_para: std::ptr::null() };
            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_BASE as *const u8, chain_exp, &para_base_ca_only as *const _ as *const c_void, &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, CERT_E_EXPIRED);
            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_BASE as *const u8, chain_exp, &para_auth as *const _ as *const c_void, &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, 0);

            CertFreeCertificateChain(chain_exp);
            assert_eq!(CertFreeCertificateContext(exp_ctx), TRUE);

            // 2. Valid Authenticode (codeSigning EKU) cert passes AUTHENTICODE with ALLOW_UNKNOWN_CA, fails without it
            let cs_der = build_synthetic_cert(
                "Code Signer",
                "Code Signer",
                b"200101000000Z",
                b"400101000000Z",
                None,
                &[&[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x03]], // id-kp-codeSigning
            );
            let cs_ctx = CertCreateCertificateContext(X509_ASN_ENCODING, cs_der.as_ptr(), cs_der.len() as u32);
            let cs_cc = std::ptr::read_unaligned(cs_ctx as *const CertContext);
            assert_eq!(CertVerifyTimeValidity(std::ptr::null(), cs_cc.p_cert_info as *const c_void), 0);
            let mut chain_cs = std::ptr::null_mut();
            assert_eq!(CertGetCertificateChain(std::ptr::null_mut(), cs_ctx, std::ptr::null(), std::ptr::null_mut(), std::ptr::null(), 0, std::ptr::null_mut(), &mut chain_cs), TRUE);

            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_AUTHENTICODE as *const u8, chain_cs, std::ptr::null(), &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, CERT_E_UNTRUSTEDROOT);

            let para_ca = CertChainPolicyPara { cb_size: 16, dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG, pv_extra_policy_para: std::ptr::null() };
            assert_eq!(CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_AUTHENTICODE as *const u8, chain_cs, &para_ca as *const _ as *const c_void, &mut st as *mut _ as *mut c_void), TRUE);
            assert_eq!(st.dw_error, 0);

            CertFreeCertificateChain(chain_cs);
            assert_eq!(CertFreeCertificateContext(cs_ctx), TRUE);
        }
    }

    #[test]
    fn crypt_string_to_binary_all_flags_and_encode_object_ex_structures() {
        unsafe {
            let decode_a = |s: &str, flags: u32| -> (Vec<u8>, u32, u32) {
                let mut cb = 0u32;
                let mut skip = 0u32;
                let mut actual = 0u32;
                assert_eq!(
                    CryptStringToBinaryA(s.as_ptr(), s.len() as u32, flags, std::ptr::null_mut(), &mut cb, &mut skip, &mut actual),
                    TRUE
                );
                let mut out = vec![0u8; cb as usize];
                assert_eq!(
                    CryptStringToBinaryA(s.as_ptr(), s.len() as u32, flags, out.as_mut_ptr(), &mut cb, &mut skip, &mut actual),
                    TRUE
                );
                out.truncate(cb as usize);
                (out, skip, actual)
            };

            // BASE64URI without padding
            let (b, _, f) = decode_a("SGVsbG8-Pz8_", CRYPT_STRING_BASE64URI);
            assert_eq!(b, b"Hello>??\x3f");
            assert_eq!(f, CRYPT_STRING_BASE64URI);

            // BASE64REQUESTHEADER & BASE64X509CRLHEADER
            let csr = "prefix\n-----BEGIN CERTIFICATE REQUEST-----\nSGVsbG8=\n-----END CERTIFICATE REQUEST-----\n";
            let (b, skip, f) = decode_a(csr, CRYPT_STRING_BASE64REQUESTHEADER);
            assert_eq!((b.as_slice(), skip, f), (&b"Hello"[..], 7, CRYPT_STRING_BASE64REQUESTHEADER));

            let crl = "-----BEGIN X509 CRL-----\nV29ybGQ=\n-----END X509 CRL-----";
            let (b, _, f) = decode_a(crl, CRYPT_STRING_BASE64X509CRLHEADER);
            assert_eq!((b.as_slice(), f), (&b"World"[..], CRYPT_STRING_BASE64X509CRLHEADER));

            // HEX, HEXRAW, HEXASCII, HEXADDR, HEXASCIIADDR
            let (b, _, _) = decode_a("48 65 6c 6c 6f", CRYPT_STRING_HEX);
            assert_eq!(b, b"Hello");
            let (b, _, _) = decode_a("48656c6c6f", CRYPT_STRING_HEXRAW);
            assert_eq!(b, b"Hello");
            let (b, _, _) = decode_a("48 65 6c 6c 6f   Hello\n", CRYPT_STRING_HEXASCII);
            assert_eq!(b, b"Hello");
            let (b, _, _) = decode_a("0000  48 65 6c 6c 6f\n", CRYPT_STRING_HEXADDR);
            assert_eq!(b, b"Hello");
            let (b, _, f) = decode_a("0000  48 65 6c 6c 6f   Hello\n", CRYPT_STRING_HEX_ANY);
            assert_eq!((b.as_slice(), f), (&b"Hello"[..], CRYPT_STRING_HEXASCIIADDR));

            // CryptStringToBinaryW
            let w_hex = crate::strings::to_wide_z("DE AD BE EF");
            let mut wlen = 0u32;
            assert_eq!(CryptStringToBinaryW(w_hex.as_ptr(), 0, CRYPT_STRING_HEX, std::ptr::null_mut(), &mut wlen, std::ptr::null_mut(), std::ptr::null_mut()), TRUE);
            let mut wout = vec![0u8; wlen as usize];
            assert_eq!(CryptStringToBinaryW(w_hex.as_ptr(), 0, CRYPT_STRING_HEX, wout.as_mut_ptr(), &mut wlen, std::ptr::null_mut(), std::ptr::null_mut()), TRUE);
            assert_eq!(wout, vec![0xDE, 0xAD, 0xBE, 0xEF]);

            // CryptEncodeObjectEx: X509_BASIC_CONSTRAINTS2 (both integer and OID string!)
            let bc = CertBasicConstraints2Info { f_ca: 1, f_path_len_constraint: 1, dw_path_len_constraint: 2 };
            let mut len = 0u32;
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, b"2.5.29.19\0".as_ptr(), &bc as *const _ as *const c_void, 0, std::ptr::null(), std::ptr::null_mut(), &mut len), TRUE);
            let mut buf = vec![0u8; len as usize];
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_BASIC_CONSTRAINTS2 as *const u8, &bc as *const _ as *const c_void, 0, std::ptr::null(), buf.as_mut_ptr() as *mut c_void, &mut len), TRUE);
            assert_eq!(buf, vec![0x30, 0x06, 0x01, 0x01, 0xFF, 0x02, 0x01, 0x02]);

            // X509_ENHANCED_KEY_USAGE
            let oids = [b"1.3.6.1.5.5.7.3.1\0".as_ptr(), b"1.3.6.1.5.5.7.3.3\0".as_ptr()];
            let eku = CertEnhkeyUsage { c_usage_identifier: 2, rgpsz_usage_identifier: oids.as_ptr() };
            let mut elen = 0u32;
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_ENHANCED_KEY_USAGE as *const u8, &eku as *const _ as *const c_void, 0, std::ptr::null(), std::ptr::null_mut(), &mut elen), TRUE);
            let mut ebuf = vec![0u8; elen as usize];
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_ENHANCED_KEY_USAGE as *const u8, &eku as *const _ as *const c_void, 0, std::ptr::null(), ebuf.as_mut_ptr() as *mut c_void, &mut elen), TRUE);
            assert_eq!(ebuf[0], 0x30);
            assert!(ebuf.windows(8).any(|w| w == [0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x03]));

            // X509_ALTERNATE_NAME (DNS + IPv4)
            let dns_w = crate::strings::to_wide_z("example.com");
            let ip4 = [127u8, 0, 0, 1];
            let mut e1 = CertAltNameEntry { dw_alt_name_choice: 3, payload: [0; 16] };
            std::ptr::write_unaligned(e1.payload.as_mut_ptr() as *mut *const u16, dns_w.as_ptr());
            let mut e2 = CertAltNameEntry { dw_alt_name_choice: 8, payload: [0; 16] };
            std::ptr::write_unaligned(e2.payload.as_mut_ptr() as *mut CryptDataBlob, CryptDataBlob { cb_data: 4, pb_data: ip4.as_ptr() });
            let entries = [e1, e2];
            let san = CertAltNameInfo { c_alt_entry: 2, rg_alt_entry: entries.as_ptr() };
            let mut slen = 0u32;
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_ALTERNATE_NAME as *const u8, &san as *const _ as *const c_void, 0, std::ptr::null(), std::ptr::null_mut(), &mut slen), TRUE);
            let mut sbuf = vec![0u8; slen as usize];
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_ALTERNATE_NAME as *const u8, &san as *const _ as *const c_void, 0, std::ptr::null(), sbuf.as_mut_ptr() as *mut c_void, &mut slen), TRUE);
            assert!(sbuf.windows(11).any(|w| w == b"example.com"));
            assert!(sbuf.windows(6).any(|w| w == [0x87, 0x04, 127, 0, 0, 1]));

            // X509_NAME -> format_dn roundtrip!
            let cn_str = b"Encode Test";
            let attr = CertRdnAttr {
                psz_obj_id: b"2.5.4.3\0".as_ptr(),
                dw_value_type: 13, // UTF8_STRING
                value: CryptDataBlob { cb_data: cn_str.len() as u32, pb_data: cn_str.as_ptr() },
            };
            let rdn = CertRdn { c_rdn_attr: 1, rg_rdn_attr: &attr };
            let ni = CertNameInfo { c_rdn: 1, rg_rdn: &rdn };
            let mut nlen = 0u32;
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_NAME as *const u8, &ni as *const _ as *const c_void, 0, std::ptr::null(), std::ptr::null_mut(), &mut nlen), TRUE);
            let mut nbuf = vec![0u8; nlen as usize];
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_NAME as *const u8, &ni as *const _ as *const c_void, 0, std::ptr::null(), nbuf.as_mut_ptr() as *mut c_void, &mut nlen), TRUE);
            assert_eq!(wintrust::format_dn(&nbuf).as_deref(), Some("CN=Encode Test"));

            // X509_ECC_SIGNATURE (LE -> DER SEQUENCE { INT r, INT s })
            let r_le = [0x01u8, 0x02];
            let s_le = [0x03u8, 0x80]; // BE = 0x8003 -> needs leading 0x00 in DER!
            let ecc_sig = CertEccSignature {
                r: CryptDataBlob { cb_data: 2, pb_data: r_le.as_ptr() },
                s: CryptDataBlob { cb_data: 2, pb_data: s_le.as_ptr() },
            };
            let mut eclen = 0u32;
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_ECC_SIGNATURE as *const u8, &ecc_sig as *const _ as *const c_void, 0, std::ptr::null(), std::ptr::null_mut(), &mut eclen), TRUE);
            let mut ecbuf = vec![0u8; eclen as usize];
            assert_eq!(CryptEncodeObjectEx(X509_ASN_ENCODING, X509_ECC_SIGNATURE as *const u8, &ecc_sig as *const _ as *const c_void, 0, std::ptr::null(), ecbuf.as_mut_ptr() as *mut c_void, &mut eclen), TRUE);
            assert_eq!(ecbuf, vec![0x30, 0x09, 0x02, 0x02, 0x02, 0x01, 0x02, 0x03, 0x00, 0x80, 0x03]);
        }
    }
}
