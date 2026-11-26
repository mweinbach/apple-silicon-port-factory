//! Certificate contexts, stores, name formatting, and PKCS#7 message handles for `crypt32.dll`.

use super::asn1::{
    all_dn_values_lower, encode_der_tlv, parse_cert_der_parts, parse_eku_der_oids,
    PKCS_7_ASN_ENCODING, X509_ASN_ENCODING,
};
use super::chain::{
    cert_verify_time_validity_impl, verify_cert_sig_with_spki, verify_cert_signed_by_der,
};
use crate::handles::HandleTable;
use crate::misc::wintrust::{
    self, CertContext, CertInfo, CertPublicKeyInfo, CmsgSignerInfo, CryptAlgId, CryptDataBlob,
    OwnedCertContext, ParsedPkcs7Info,
};
use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{LazyLock, Mutex};

pub(super) const CRYPT_E_NOT_FOUND: u32 = 0x8009_2004;
const CRYPT_E_NO_MATCH: u32 = 0x8009_2009;
const NTE_BAD_ALGID: u32 = 0x8009_0008;
pub(super) const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

pub(super) const CERT_QUERY_OBJECT_FILE: u32 = 1;
pub(super) const CERT_QUERY_OBJECT_BLOB: u32 = 2;
pub(super) const CERT_QUERY_CONTENT_CERT: u32 = 1;
pub(super) const CERT_QUERY_CONTENT_PKCS7_SIGNED: u32 = 8;
pub(super) const CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED: u32 = 10;
pub(super) const CERT_QUERY_FORMAT_BINARY: u32 = 1;
pub(super) const CERT_QUERY_FORMAT_FLAG_BINARY: u32 = 1 << CERT_QUERY_FORMAT_BINARY;

pub(super) const CMSG_TYPE_PARAM: u32 = 1;
pub(super) const CMSG_SIGNER_COUNT_PARAM: u32 = 5;
pub(super) const CMSG_SIGNER_INFO_PARAM: u32 = 6;
pub(super) const CMSG_CERT_COUNT_PARAM: u32 = 11;
pub(super) const CMSG_CERT_PARAM: u32 = 12;
pub(super) const CMSG_VERSION_PARAM: u32 = 30;
const CMSG_SIGNED: u32 = 2;

pub(super) const CERT_FIND_ANY: u32 = 0x0000_0000;
pub(super) const CERT_FIND_SHA1_HASH: u32 = 0x0001_0000;
pub(super) const CERT_FIND_MD5_HASH: u32 = 0x0004_0000;
pub(super) const CERT_FIND_PUBLIC_KEY: u32 = 0x0006_0000;
pub(super) const CERT_FIND_ISSUER_NAME: u32 = 0x0002_0004;
pub(super) const CERT_FIND_SUBJECT_NAME: u32 = 0x0002_0007;
pub(super) const CERT_FIND_ISSUER_STR_A: u32 = 0x0007_0004;
pub(super) const CERT_FIND_SUBJECT_STR_A: u32 = 0x0007_0007;
pub(super) const CERT_FIND_ISSUER_STR_W: u32 = 0x0008_0004;
pub(super) const CERT_FIND_SUBJECT_STR_W: u32 = 0x0008_0007;
pub(super) const CERT_FIND_SUBJECT_CERT: u32 = 0x000B_0000;
pub(super) const CERT_FIND_ISSUER_OF: u32 = 0x000C_0000;
pub(super) const CERT_FIND_EXISTING: u32 = 0x000D_0000;

pub(super) const CERT_STORE_PROV_MEMORY: usize = 2;
pub(super) const CERT_STORE_PROV_SYSTEM_A: usize = 9;
pub(super) const CERT_STORE_PROV_SYSTEM_W: usize = 10;
pub(super) const CERT_STORE_PROV_SYSTEM_REGISTRY_A: usize = 12;
pub(super) const CERT_STORE_PROV_SYSTEM_REGISTRY_W: usize = 13;
#[cfg(test)]
pub(super) const CERT_SYSTEM_STORE_CURRENT_USER: u32 = 0x0001_0000;

pub(super) const CERT_STORE_ADD_NEW: u32 = 1;
const CERT_STORE_ADD_USE_EXISTING: u32 = 2;
const CERT_STORE_ADD_REPLACE_EXISTING: u32 = 3;
const CERT_STORE_ADD_ALWAYS: u32 = 4;

// Certificate context property IDs (wincrypt.h)
pub(super) const CERT_KEY_PROV_HANDLE_PROP_ID: u32 = 1;
pub(super) const CERT_KEY_PROV_INFO_PROP_ID: u32 = 2;
pub(super) const CERT_SHA1_HASH_PROP_ID: u32 = 3;
pub(super) const CERT_MD5_HASH_PROP_ID: u32 = 4;
pub(super) const CERT_KEY_CONTEXT_PROP_ID: u32 = 5;
pub(super) const CERT_KEY_SPEC_PROP_ID: u32 = 6;
pub(super) const CERT_ENHKEY_USAGE_PROP_ID: u32 = 9;
pub(super) const CERT_FRIENDLY_NAME_PROP_ID: u32 = 11;
pub(super) const CERT_DESCRIPTION_PROP_ID: u32 = 13;
pub(super) const CERT_ACCESS_STATE_PROP_ID: u32 = 14;
pub(super) const CERT_SIGNATURE_HASH_PROP_ID: u32 = 15;
pub(super) const CERT_KEY_IDENTIFIER_PROP_ID: u32 = 20;
pub(super) const CERT_PUBKEY_ALG_PARA_PROP_ID: u32 = 22;
pub(super) const CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 24;
pub(super) const CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID: u32 = 25;
pub(super) const CERT_DATE_STAMP_PROP_ID: u32 = 27;
pub(super) const CERT_SUBJECT_NAME_MD5_HASH_PROP_ID: u32 = 29;
pub(super) const CERT_STORE_ENUM_ARCHIVED_FLAG: u32 = 0x0000_0200;
pub(super) const CERT_ARCHIVED_PROP_ID: u32 = 19;
pub(super) const CERT_NCRYPT_KEY_HANDLE_PROP_ID: u32 = 78;
pub(super) const CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID: u32 = 99;
pub(super) const CERT_SHA256_HASH_PROP_ID: u32 = 107;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct CertKeyContext {
    pub(super) cb_size: u32,
    pub(super) h_crypt_prov: usize,
    pub(super) dw_key_spec: u32,
}
const _: () = assert!(std::mem::size_of::<CertKeyContext>() == 24);

// ---------------------------------------------------------------------------
// Internal registries for certificates, stores, and PKCS#7 messages
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub(crate) struct PeerCertVerifyResult {
    /// Windows `CERT_E_*` status from TLS handshake WebPKI verification (`0` = valid).
    pub cert_error: u32,
    /// Additional trust anchors supplied via `SCHANNEL_CRED.hRootStore`.
    pub extra_roots_der: Vec<Vec<u8>>,
}

#[derive(Clone, Debug)]
struct StoredKeyProvParam {
    dw_param: u32,
    data: Vec<u8>,
    dw_flags: u32,
}

#[derive(Clone, Debug)]
struct StoredKeyProvInfo {
    container_name: Option<Vec<u16>>,
    prov_name: Option<Vec<u16>>,
    dw_prov_type: u32,
    dw_flags: u32,
    params: Vec<StoredKeyProvParam>,
    dw_key_spec: u32,
}

struct RegisteredCert {
    owned: OwnedCertContext,
    refcount: usize,
    /// Full peer certificate chain DER (`[leaf, intermediate1, ...]`) when created from TLS.
    peer_chain_der: Vec<Vec<u8>>,
    peer_verify: Option<PeerCertVerifyResult>,
    /// Memory store owned by this peer cert (released when refcount hits 0).
    owned_store: usize,
    /// Arbitrary certificate context properties (`CertSetCertificateContextProperty` / PFX).
    properties: HashMap<u32, Vec<u8>>,
    /// Deep-copied `CRYPT_KEY_PROV_INFO` when set via `CERT_KEY_PROV_INFO_PROP_ID`.
    key_prov_info: Option<StoredKeyProvInfo>,
    /// Associated PKCS#8 DER private key (from `PFXImportCertStore` or `attach_cert_private_key`).
    private_key_pkcs8: Option<Vec<u8>>,
}

struct RegisteredStore {
    refcount: usize,
    certs: Vec<usize>,
    is_ntauth: bool,
    enum_archived: bool,
}

#[derive(Default)]
struct StoreState {
    certs: HashMap<usize, RegisteredCert>,
    /// Map `SubjectPublicKeyInfo` field address -> cert handle for exact DER SPKI lookup.
    spki_to_cert: HashMap<usize, usize>,
    stores: HashMap<usize, RegisteredStore>,
    /// Enterprise `NTAuth` issuing CA certificates (empty by default on non-domain macOS;
    /// populated when a caller opens the `"NTAUTH"` store and adds authorized issuing CAs).
    ntauth_certs_der: Vec<Vec<u8>>,
    next_store_id: usize,
}

static STORE_STATE: LazyLock<Mutex<StoreState>> = LazyLock::new(|| {
    Mutex::new(StoreState {
        next_store_id: 0x5100_0001,
        ..Default::default()
    })
});

static MSGS: HandleTable<ParsedPkcs7Info> = HandleTable::new(0x5300_0001, 1);

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
        encode_der_tlv(
            0x30,
            &encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02]),
        )
    } else {
        let mut inner = encode_der_tlv(
            0x06,
            &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B],
        );
        inner.extend_from_slice(&encode_der_tlv(0x05, &[]));
        encode_der_tlv(0x30, &inner)
    };

    // Deterministic canonical positive 16-byte serial number from SHA-256(subject || spki).
    let hash = crate::misc::bcrypt::Digest::Sha256.hash_slices(&[&subject_der, &spki_der]);
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

fn insert_cert_locked(
    st: &mut StoreState,
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
            properties: HashMap::new(),
            key_prov_info: None,
            private_key_pkcs8: None,
        },
    );
    ptr
}

pub(super) fn create_cert_context_from_der(der: &[u8]) -> *mut c_void {
    let Some(owned) = wintrust::build_owned_cert_context(der, std::ptr::null_mut()) else {
        return std::ptr::null_mut();
    };
    let mut st = STORE_STATE.lock().unwrap();
    insert_cert_locked(&mut st, owned, vec![der.to_vec()], None, 0) as *mut c_void
}

pub(super) fn add_der_with_meta_to_store(
    h_cert_store: *mut c_void,
    der: &[u8],
    props: Vec<(u32, Vec<u8>)>,
    private_key_pkcs8: Option<Vec<u8>>,
) -> *mut c_void {
    let Some(owned) = wintrust::build_owned_cert_context(der, h_cert_store) else {
        return std::ptr::null_mut();
    };
    let mut st = STORE_STATE.lock().unwrap();
    let store_key = h_cert_store as usize;
    if !st.stores.contains_key(&store_key) {
        return std::ptr::null_mut();
    }
    let h = insert_cert_locked(&mut st, owned, vec![der.to_vec()], None, 0);
    if let Some(entry) = st.certs.get_mut(&h) {
        for (id, val) in props {
            entry.properties.insert(id, val);
        }
        entry.private_key_pkcs8 = private_key_pkcs8;
    }
    st.stores.get_mut(&store_key).unwrap().certs.push(h);
    h as *mut c_void
}

/// Associate a PKCS#8 DER private key with a registered `PCCERT_CONTEXT` (for server-side Schannel).
#[allow(dead_code)]
pub(crate) fn attach_cert_private_key_pkcs8(cert_ctx: *mut c_void, pkcs8_der: Vec<u8>) -> bool {
    if cert_ctx.is_null() {
        return false;
    }
    let mut st = STORE_STATE.lock().unwrap();
    if let Some(entry) = st.certs.get_mut(&(cert_ctx as usize)) {
        entry.private_key_pkcs8 = Some(pkcs8_der);
        true
    } else {
        false
    }
}

/// Order a TLS local credential's certification path per RFC 5246 §7.4.2 (leaf first, each
/// subsequent certificate cryptographically certifying its predecessor), excluding unrelated
/// certificates present in the same `HCERTSTORE`.
fn build_ordered_tls_cert_chain(leaf_der: Vec<u8>, candidates: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut chain = vec![leaf_der];
    for _ in 0..16 {
        let cur = chain.last().unwrap().as_slice();
        let Some(cur_parts) = parse_cert_der_parts(cur) else {
            break;
        };
        if cur_parts.subject_inner == cur_parts.issuer_inner && verify_cert_signed_by_der(cur, cur)
        {
            break;
        }
        let verified_issuers: Vec<usize> = candidates
            .iter()
            .enumerate()
            .filter(|(_, cand)| {
                !chain.iter().any(|seen| seen == *cand)
                    && parse_cert_der_parts(cand)
                        .is_some_and(|p| p.subject_inner == cur_parts.issuer_inner)
                    && verify_cert_signed_by_der(cur, cand)
            })
            .map(|(idx, _)| idx)
            .collect();
        let next_idx = verified_issuers
            .iter()
            .copied()
            .find(|&idx| {
                parse_cert_der_parts(&candidates[idx]).is_some_and(|p| {
                    p.subject_inner != p.issuer_inner
                        && candidates.iter().any(|parent| {
                            parent != &candidates[idx]
                                && parse_cert_der_parts(parent)
                                    .is_some_and(|pp| pp.subject_inner == p.issuer_inner)
                                && verify_cert_signed_by_der(&candidates[idx], parent)
                        })
                })
            })
            .or_else(|| {
                verified_issuers.iter().copied().find(|&idx| {
                    parse_cert_der_parts(&candidates[idx]).is_some_and(|p| {
                        p.subject_inner == p.issuer_inner
                            && verify_cert_signed_by_der(&candidates[idx], &candidates[idx])
                    })
                })
            })
            .or_else(|| verified_issuers.first().copied());
        let Some(idx) = next_idx else {
            break;
        };
        chain.push(candidates[idx].clone());
    }
    chain
}

/// Look up the ordered RFC 5246 §7.4.2 certificate chain (`[leaf, issuer1, ...]`) and attached
/// PKCS#8 private key for a `PCCERT_CONTEXT` passed to Schannel (`SCHANNEL_CRED.paCred`),
/// excluding unrelated certificates present in the same store.
pub(crate) fn lookup_cert_private_key_and_chain(
    cert_ctx: *const c_void,
) -> Option<(Vec<Vec<u8>>, Vec<u8>)> {
    if cert_ctx.is_null() {
        return None;
    }
    let (leaf_der, key, candidates) = {
        let st = STORE_STATE.lock().unwrap();
        let entry = st.certs.get(&(cert_ctx as usize))?;
        let key = entry.private_key_pkcs8.clone()?;
        let leaf_der = entry.owned.raw_der().to_vec();
        let mut candidates = Vec::new();
        let store_h = unsafe { (*entry.owned.as_ptr()).h_cert_store as usize };
        if store_h != 0 {
            if let Some(store) = st.stores.get(&store_h) {
                for &ch in &store.certs {
                    if ch == cert_ctx as usize {
                        continue;
                    }
                    if let Some(other) = st.certs.get(&ch) {
                        let der = other.owned.raw_der().to_vec();
                        if der != leaf_der && !candidates.iter().any(|c| c == &der) {
                            candidates.push(der);
                        }
                    }
                }
            }
        }
        (leaf_der, key, candidates)
    };
    let chain = build_ordered_tls_cert_chain(leaf_der, candidates);
    Some((chain, key))
}

/// Duplicate a registered `PCCERT_CONTEXT` by raw DER bytes (or register a fresh context).
pub(crate) fn duplicate_or_create_cert_context_der(der: &[u8]) -> *mut c_void {
    create_cert_context_from_der(der)
}

fn release_cert_locked(st: &mut StoreState, handle: usize) -> bool {
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

fn release_store_locked(st: &mut StoreState, handle: usize) -> bool {
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

pub(super) fn lookup_spki_raw_der(spki_addr: usize) -> Option<Vec<u8>> {
    let st = STORE_STATE.lock().unwrap();
    let &cert_h = st.spki_to_cert.get(&spki_addr)?;
    let c = st.certs.get(&cert_h)?;
    let raw = c.owned.spki_raw_der();
    (!raw.is_empty()).then(|| raw.to_vec())
}

pub(super) fn lookup_cert_for_chain(
    cert_handle: usize,
) -> Option<(
    Vec<Vec<u8>>,
    Option<PeerCertVerifyResult>,
    HashMap<Vec<u8>, Vec<Vec<u8>>>,
)> {
    let st = STORE_STATE.lock().unwrap();
    let entry = st.certs.get(&cert_handle)?;
    let certs = if !entry.peer_chain_der.is_empty() {
        entry.peer_chain_der.clone()
    } else {
        vec![entry.owned.raw_der().to_vec()]
    };
    let mut eku_props = HashMap::new();
    let mut collect_store = |store_id: usize| {
        if store_id != 0 {
            if let Some(store) = st.stores.get(&store_id) {
                for h in &store.certs {
                    if let Some(c) = st.certs.get(h) {
                        if let Some(raw) = c.properties.get(&CERT_ENHKEY_USAGE_PROP_ID) {
                            if let Some(oids) = parse_eku_der_oids(raw) {
                                eku_props.insert(c.owned.raw_der().to_vec(), oids);
                            }
                        }
                    }
                }
            }
        }
    };
    collect_store(entry.owned_store);
    let cert_store_h = unsafe { (*entry.owned.as_ptr()).h_cert_store as usize };
    if cert_store_h != entry.owned_store {
        collect_store(cert_store_h);
    }
    if let Some(raw) = entry.properties.get(&CERT_ENHKEY_USAGE_PROP_ID) {
        if let Some(oids) = parse_eku_der_oids(raw) {
            eku_props.insert(entry.owned.raw_der().to_vec(), oids);
        }
    }
    Some((certs, entry.peer_verify.clone(), eku_props))
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
    let mut st = STORE_STATE.lock().unwrap();
    let store_handle = st.next_store_id;
    st.next_store_id += 1;

    let mut store_certs = Vec::with_capacity(chain_der.len());
    for der in chain_der {
        if let Some(owned) = wintrust::build_owned_cert_context(der, store_handle as *mut c_void) {
            let h = insert_cert_locked(&mut st, owned, Vec::new(), None, 0);
            store_certs.push(h);
        }
    }
    st.stores.insert(
        store_handle,
        RegisteredStore {
            refcount: 1,
            certs: store_certs,
            is_ntauth: false,
            enum_archived: false,
        },
    );

    let Some(leaf_owned) = wintrust::build_owned_cert_context(leaf_der, store_handle as *mut c_void)
    else {
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

/// Collect DER bytes and any stored `CERT_ENHKEY_USAGE_PROP_ID` restrictions for all
/// certificates in a registered `HCERTSTORE`.
pub(super) fn store_certs_with_eku_props(
    h_cert_store: *mut c_void,
) -> (Vec<Vec<u8>>, HashMap<Vec<u8>, Vec<Vec<u8>>>) {
    if h_cert_store.is_null() {
        return (Vec::new(), HashMap::new());
    }
    let st = STORE_STATE.lock().unwrap();
    let Some(store) = st.stores.get(&(h_cert_store as usize)) else {
        return (Vec::new(), HashMap::new());
    };
    let mut ders = Vec::with_capacity(store.certs.len());
    let mut eku_props = HashMap::new();
    for h in &store.certs {
        if let Some(c) = st.certs.get(h) {
            let der = c.owned.raw_der().to_vec();
            if let Some(raw) = c.properties.get(&CERT_ENHKEY_USAGE_PROP_ID) {
                if let Some(oids) = parse_eku_der_oids(raw) {
                    eku_props.insert(der.clone(), oids);
                }
            }
            ders.push(der);
        }
    }
    (ders, eku_props)
}

/// Collect DER bytes of all certificates in a registered `HCERTSTORE`.
pub(crate) fn store_certs_der(h_cert_store: *mut c_void) -> Vec<Vec<u8>> {
    store_certs_with_eku_props(h_cert_store).0
}

// ---------------------------------------------------------------------------
// Certificate Context & Store implementations
// ---------------------------------------------------------------------------

pub(super) unsafe fn cert_create_certificate_context_impl(
    dw_cert_encoding_type: u32,
    pb_cert_encoded: *const u8,
    cb_cert_encoded: u32,
) -> *mut c_void {
    if (dw_cert_encoding_type & X509_ASN_ENCODING) == 0
        || pb_cert_encoded.is_null()
        || cb_cert_encoded == 0
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let der = std::slice::from_raw_parts(pb_cert_encoded, cb_cert_encoded as usize);
    let Some(owned) = wintrust::build_owned_cert_context(der, std::ptr::null_mut()) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    let mut st = STORE_STATE.lock().unwrap();
    insert_cert_locked(&mut st, owned, vec![der.to_vec()], None, 0) as *mut c_void
}

pub(super) fn cert_duplicate_certificate_context_impl(ctx: *mut c_void) -> *mut c_void {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let mut st = STORE_STATE.lock().unwrap();
    if let Some(entry) = st.certs.get_mut(&(ctx as usize)) {
        entry.refcount += 1;
    }
    ctx
}

pub(super) fn cert_free_certificate_context_impl(ctx: *mut c_void) -> BOOL {
    if ctx.is_null() {
        return TRUE;
    }
    let mut st = STORE_STATE.lock().unwrap();
    if !release_cert_locked(&mut st, ctx as usize) {
        stub!("CertFreeCertificateContext({ctx:p}): foreign context ignored");
    }
    TRUE
}

fn open_named_system_store(store_name: &str, enum_archived: bool) -> *mut c_void {
    let upper = store_name.trim().to_ascii_uppercase();
    let seed_roots = matches!(upper.as_str(), "ROOT" | "AUTHROOT" | "CA");
    let is_ntauth = upper == "NTAUTH";
    let mut st = STORE_STATE.lock().unwrap();
    let id = st.next_store_id;
    st.next_store_id += 1;
    let mut certs = Vec::new();
    if is_ntauth {
        let existing: Vec<Vec<u8>> = st.ntauth_certs_der.clone();
        for der in existing {
            if let Some(owned) = wintrust::build_owned_cert_context(&der, id as *mut c_void) {
                let h = insert_cert_locked(&mut st, owned, vec![der], None, 0);
                certs.push(h);
            }
        }
    }
    if seed_roots {
        let mut seen_subjects: std::collections::HashSet<Vec<u8>> = std::collections::HashSet::new();
        // Seed real macOS Security.framework system/admin/user roots first, then webpki-roots.
        for der in super::chain::macos_system_roots_der() {
            if let Some(owned) = wintrust::build_owned_cert_context(der, id as *mut c_void) {
                let key = owned.spki_raw_der().to_vec();
                if seen_subjects.insert(key) {
                    let h = insert_cert_locked(&mut st, owned, vec![der.clone()], None, 0);
                    certs.push(h);
                }
            }
        }
        let excluded_spkis = super::chain::macos_excluded_root_spkis();
        for der in WEBPKI_ROOT_CERTS_DER.iter() {
            if super::asn1::parse_cert_der_parts(der)
                .is_some_and(|p| super::chain::spki_is_excluded(p.spki_inner, excluded_spkis))
            {
                continue;
            }
            if let Some(owned) = wintrust::build_owned_cert_context(der, id as *mut c_void) {
                let key = owned.spki_raw_der().to_vec();
                if seen_subjects.insert(key) {
                    let h = insert_cert_locked(&mut st, owned, vec![der.clone()], None, 0);
                    certs.push(h);
                }
            }
        }
    }
    st.stores.insert(
        id,
        RegisteredStore {
            refcount: 1,
            certs,
            is_ntauth,
            enum_archived,
        },
    );
    log::debug!("CertOpenStore(system={store_name:?}) -> {id:#x} (seeded={seed_roots})");
    id as *mut c_void
}

/// Snapshot of DER certificates currently registered in the Enterprise `NTAuth` store.
pub(super) fn ntauth_store_certs_der() -> Vec<Vec<u8>> {
    STORE_STATE.lock().unwrap().ntauth_certs_der.clone()
}

pub(super) unsafe fn cert_open_store_impl(
    lpsz_store_provider: *const u8,
    pv_para: *const c_void,
) -> *mut c_void {
    cert_open_store_with_flags_impl(lpsz_store_provider, 0, pv_para)
}

pub(super) unsafe fn cert_open_store_with_flags_impl(
    lpsz_store_provider: *const u8,
    dw_flags: u32,
    pv_para: *const c_void,
) -> *mut c_void {
    let enum_archived = (dw_flags & CERT_STORE_ENUM_ARCHIVED_FLAG) != 0;
    let prov = lpsz_store_provider as usize;
    let prov_str = if prov > 0xFFFF {
        Some(crate::strings::from_ansi(lpsz_store_provider))
    } else {
        None
    };

    let is_memory = prov == CERT_STORE_PROV_MEMORY
        || prov_str
            .as_deref()
            .is_some_and(|s| s.eq_ignore_ascii_case("Memory"));
    if is_memory {
        let mut st = STORE_STATE.lock().unwrap();
        let id = st.next_store_id;
        st.next_store_id += 1;
        st.stores.insert(
            id,
            RegisteredStore {
                refcount: 1,
                certs: Vec::new(),
                is_ntauth: false,
                enum_archived,
            },
        );
        return id as *mut c_void;
    }

    let is_system_a = prov == CERT_STORE_PROV_SYSTEM_A
        || prov == CERT_STORE_PROV_SYSTEM_REGISTRY_A
        || prov_str.as_deref().is_some_and(|s| {
            s.eq_ignore_ascii_case("SystemA") || s.eq_ignore_ascii_case("SystemRegistryA")
        });
    if is_system_a {
        if pv_para.is_null() {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return std::ptr::null_mut();
        }
        let name = crate::strings::from_ansi(pv_para as *const u8);
        return open_named_system_store(&name, enum_archived);
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
        return open_named_system_store(&name, enum_archived);
    }

    stub!("CertOpenStore(provider={prov:#x}) -> NULL");
    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
    std::ptr::null_mut()
}

pub(super) unsafe fn cert_open_system_store_a_impl(sz_subsystem_protocol: LPCSTR) -> *mut c_void {
    if sz_subsystem_protocol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    cert_open_store_impl(
        CERT_STORE_PROV_SYSTEM_A as *const u8,
        sz_subsystem_protocol as *const c_void,
    )
}

pub(super) unsafe fn cert_open_system_store_w_impl(sz_subsystem_protocol: LPCWSTR) -> *mut c_void {
    if sz_subsystem_protocol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    cert_open_store_impl(
        CERT_STORE_PROV_SYSTEM_W as *const u8,
        sz_subsystem_protocol as *const c_void,
    )
}

pub(super) unsafe fn cert_delete_certificate_from_store_impl(p_cert_context: *mut c_void) -> BOOL {
    if p_cert_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cc = std::ptr::read_unaligned(p_cert_context as *const CertContext);
    let cert_h = p_cert_context as usize;
    let store_h = cc.h_cert_store as usize;
    let cert_der = if !cc.pb_cert_encoded.is_null() && cc.cb_cert_encoded > 0 {
        Some(std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize).to_vec())
    } else {
        None
    };
    let mut st = STORE_STATE.lock().unwrap();
    let mut removed_from_store = false;
    let mut removed_from_ntauth = false;
    if store_h != 0 {
        if let Some(store) = st.stores.get_mut(&store_h) {
            if let Some(pos) = store.certs.iter().position(|&h| h == cert_h) {
                store.certs.remove(pos);
                removed_from_ntauth = store.is_ntauth;
                // Release the store's own reference to this cert.
                release_cert_locked(&mut st, cert_h);
                removed_from_store = true;
            }
        }
    }
    if removed_from_ntauth {
        if let Some(ref der) = cert_der {
            st.ntauth_certs_der.retain(|existing| existing != der);
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

pub(super) fn cert_duplicate_store_impl(store: *mut c_void) -> *mut c_void {
    if store.is_null() {
        return std::ptr::null_mut();
    }
    let mut st = STORE_STATE.lock().unwrap();
    if let Some(s) = st.stores.get_mut(&(store as usize)) {
        s.refcount += 1;
    }
    store
}

pub(super) fn cert_close_store_impl(store: *mut c_void) -> BOOL {
    if store.is_null() {
        return TRUE;
    }
    let mut st = STORE_STATE.lock().unwrap();
    if !release_store_locked(&mut st, store as usize) {
        stub!("CertCloseStore({store:p}): foreign store ignored");
    }
    TRUE
}

#[derive(Clone, Default)]
struct CopiedCertMeta {
    properties: HashMap<u32, Vec<u8>>,
    key_prov_info: Option<StoredKeyProvInfo>,
    private_key_pkcs8: Option<Vec<u8>>,
}

fn is_non_copyable_cert_prop(prop_id: u32) -> bool {
    matches!(
        prop_id,
        CERT_KEY_PROV_HANDLE_PROP_ID
            | CERT_KEY_CONTEXT_PROP_ID
            | CERT_NCRYPT_KEY_HANDLE_PROP_ID
            | CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID
    )
}

fn extract_copyable_cert_meta(src: &RegisteredCert) -> CopiedCertMeta {
    let properties = src
        .properties
        .iter()
        .filter(|(&id, _)| !is_non_copyable_cert_prop(id))
        .map(|(&id, v)| (id, v.clone()))
        .collect();
    CopiedCertMeta {
        properties,
        key_prov_info: src.key_prov_info.clone(),
        private_key_pkcs8: src.private_key_pkcs8.clone(),
    }
}

unsafe fn add_der_to_store(
    h_cert_store: *mut c_void,
    der: &[u8],
    dw_add_disposition: u32,
    pp_store_context: *mut *mut c_void,
) -> BOOL {
    add_der_to_store_with_source(h_cert_store, der, dw_add_disposition, pp_store_context, None)
}

unsafe fn add_der_to_store_with_source(
    h_cert_store: *mut c_void,
    der: &[u8],
    dw_add_disposition: u32,
    pp_store_context: *mut *mut c_void,
    src_cert_handle: Option<usize>,
) -> BOOL {
    let Some(owned) = wintrust::build_owned_cert_context(der, h_cert_store) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    };
    let mut st = STORE_STATE.lock().unwrap();
    let store_key = h_cert_store as usize;
    if !st.stores.contains_key(&store_key) {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }

    let copied_meta = src_cert_handle
        .and_then(|src_h| st.certs.get(&src_h))
        .map(extract_copyable_cert_meta);

    let existing_idx = st.stores[&store_key].certs.iter().position(|&h| {
        st.certs
            .get(&h)
            .is_some_and(|c| c.owned.raw_der() == der)
    });

    if let Some(idx) = existing_idx {
        match dw_add_disposition {
            CERT_STORE_ADD_NEW => {
                crate::teb::set_last_error(0x8009_2005); // CRYPT_E_EXISTS
                return FALSE;
            }
            CERT_STORE_ADD_USE_EXISTING => {
                let h = st.stores[&store_key].certs[idx];
                if let Some(c) = st.certs.get_mut(&h) {
                    if let Some(meta) = copied_meta {
                        for (id, val) in meta.properties {
                            c.properties.entry(id).or_insert(val);
                        }
                        if c.key_prov_info.is_none() && meta.key_prov_info.is_some() {
                            c.key_prov_info = meta.key_prov_info;
                        }
                        if c.private_key_pkcs8.is_none() && meta.private_key_pkcs8.is_some() {
                            c.private_key_pkcs8 = meta.private_key_pkcs8;
                        }
                    }
                    if !pp_store_context.is_null() {
                        c.refcount += 1;
                    }
                }
                if !pp_store_context.is_null() {
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
    if let (Some(meta), Some(entry)) = (copied_meta, st.certs.get_mut(&h)) {
        entry.properties = meta.properties;
        entry.key_prov_info = meta.key_prov_info;
        entry.private_key_pkcs8 = meta.private_key_pkcs8;
    }
    let is_ntauth = {
        let store = st.stores.get_mut(&store_key).unwrap();
        store.certs.push(h);
        store.is_ntauth
    };
    if is_ntauth && !st.ntauth_certs_der.iter().any(|existing| existing.as_slice() == der) {
        st.ntauth_certs_der.push(der.to_vec());
    }
    if !pp_store_context.is_null() {
        st.certs.get_mut(&h).unwrap().refcount += 1;
        std::ptr::write_unaligned(pp_store_context, h as *mut c_void);
    }
    TRUE
}

pub(super) unsafe fn cert_add_certificate_context_to_store_impl(
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
    add_der_to_store_with_source(
        h_cert_store,
        der,
        dw_add_disposition,
        pp_store_context,
        Some(p_cert_context as usize),
    )
}

pub(super) unsafe fn cert_add_encoded_certificate_to_store_impl(
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

pub(super) fn cert_enum_certificates_in_store_impl(
    h_cert_store: *mut c_void,
    p_prev_cert_context: *mut c_void,
) -> *mut c_void {
    let mut st = STORE_STATE.lock().unwrap();
    let Some((certs, enum_archived)) = st
        .stores
        .get(&(h_cert_store as usize))
        .map(|s| (s.certs.clone(), s.enum_archived))
    else {
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
    for &cert_h in &certs[next_idx..] {
        if let Some(c) = st.certs.get_mut(&cert_h) {
            if !enum_archived && c.properties.contains_key(&CERT_ARCHIVED_PROP_ID) {
                continue;
            }
            c.refcount += 1;
            return cert_h as *mut c_void;
        }
    }
    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
    std::ptr::null_mut()
}

// ---------------------------------------------------------------------------
// Name Formatting & Certificate Store Queries
// ---------------------------------------------------------------------------

unsafe fn write_wide_out(s: &str, buf: *mut u16, cch: u32) -> u32 {
    if buf.is_null() || cch == 0 {
        return (s.encode_utf16().count() + 1) as u32;
    }
    (crate::strings::write_wide_z(buf, cch as usize, s) + 1) as u32
}

unsafe fn write_ansi_out(s: &str, buf: *mut u8, cch: u32) -> u32 {
    if buf.is_null() || cch == 0 {
        return (s.len() + 1) as u32;
    }
    (crate::strings::write_ansi_z(buf, cch as usize, s) + 1) as u32
}

pub(super) unsafe fn cert_name_to_str_w_impl(
    name: *const c_void,
    buf: *mut u16,
    cch: u32,
) -> u32 {
    let s = if !name.is_null() {
        let blob = std::ptr::read_unaligned(name as *const CryptDataBlob);
        if !blob.pb_data.is_null() && blob.cb_data > 0 {
            let der = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            wintrust::format_dn(der).unwrap_or_default()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    write_wide_out(&s, buf, cch)
}

pub(super) unsafe fn cert_name_to_str_a_impl(
    name: *const c_void,
    buf: *mut u8,
    cch: u32,
) -> u32 {
    let s = if !name.is_null() {
        let blob = std::ptr::read_unaligned(name as *const CryptDataBlob);
        if !blob.pb_data.is_null() && blob.cb_data > 0 {
            let der = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
            wintrust::format_dn(der).unwrap_or_default()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    write_ansi_out(&s, buf, cch)
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
    let cc = std::ptr::read_unaligned(ctx as *const CertContext);
    if cc.p_cert_info.is_null() {
        return String::new();
    }
    let ci = std::ptr::read_unaligned(cc.p_cert_info);
    let blob = if (flags & CERT_NAME_ISSUER_FLAG) != 0 {
        ci.issuer
    } else {
        ci.subject
    };
    if blob.pb_data.is_null() || blob.cb_data == 0 {
        return String::new();
    }
    let der = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
    match ty {
        CERT_NAME_RDN_TYPE => wintrust::format_dn(der).unwrap_or_default(),
        CERT_NAME_ATTR_TYPE => {
            if para.is_null() {
                wintrust::extract_dn_attr(der, &[0x55, 0x04, 0x03]).unwrap_or_default()
            } else {
                let oid_s = crate::strings::from_ansi(para as *const u8);
                wintrust::encode_oid_str(&oid_s)
                    .and_then(|oid| wintrust::extract_dn_attr(der, &oid))
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
                if let Some(v) = wintrust::extract_dn_attr(der, oid) {
                    return v;
                }
            }
            String::new()
        }
        _ => String::new(),
    }
}

pub(super) unsafe fn cert_get_name_string_w_impl(
    ctx: *mut c_void,
    ty: u32,
    flags: u32,
    para: *mut c_void,
    buf: *mut u16,
    cch: u32,
) -> u32 {
    let s = cert_get_name(ctx, ty, flags, para);
    write_wide_out(&s, buf, cch)
}

pub(super) unsafe fn cert_get_name_string_a_impl(
    ctx: *mut c_void,
    ty: u32,
    flags: u32,
    para: *mut c_void,
    buf: *mut u8,
    cch: u32,
) -> u32 {
    let s = cert_get_name(ctx, ty, flags, para);
    write_ansi_out(&s, buf, cch)
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
            let q_serial =
                if query.serial_number.pb_data.is_null() || query.serial_number.cb_data == 0 {
                    &[][..]
                } else {
                    std::slice::from_raw_parts(
                        query.serial_number.pb_data,
                        query.serial_number.cb_data as usize,
                    )
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
            let needle =
                crate::strings::from_wide(pv_find_para as *const u16).to_ascii_lowercase();
            let dn = if dw_find_type == CERT_FIND_ISSUER_STR_W {
                issuer_slice
            } else {
                subject_slice
            };
            let hay = wintrust::format_dn(dn)
                .unwrap_or_default()
                .to_ascii_lowercase();
            hay.contains(&needle) || all_dn_values_lower(dn).iter().any(|v| v.contains(&needle))
        }
        CERT_FIND_SUBJECT_STR_A | CERT_FIND_ISSUER_STR_A => {
            if pv_find_para.is_null() {
                return false;
            }
            let needle = crate::strings::from_ansi(pv_find_para as *const u8).to_ascii_lowercase();
            let dn = if dw_find_type == CERT_FIND_ISSUER_STR_A {
                issuer_slice
            } else {
                subject_slice
            };
            let hay = wintrust::format_dn(dn)
                .unwrap_or_default()
                .to_ascii_lowercase();
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
            crate::misc::bcrypt::Digest::Sha1
                .hash(owned.raw_der())
                .as_slice()
                == q
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
            crate::misc::bcrypt::Digest::Md5
                .hash(owned.raw_der())
                .as_slice()
                == q
        }
        CERT_FIND_PUBLIC_KEY => {
            if pv_find_para.is_null() {
                return false;
            }
            let qspki = std::ptr::read_unaligned(pv_find_para as *const CertPublicKeyInfo);
            let q_key = if qspki.public_key.pb_data.is_null() || qspki.public_key.cb_data == 0 {
                &[][..]
            } else {
                std::slice::from_raw_parts(
                    qspki.public_key.pb_data,
                    qspki.public_key.cb_data as usize,
                )
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
            if !qspki.algorithm.psz_obj_id.is_null()
                && !ci.subject_public_key_info.algorithm.psz_obj_id.is_null()
            {
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
            let q_issuer =
                std::slice::from_raw_parts(qci.issuer.pb_data, qci.issuer.cb_data as usize);
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

pub(super) unsafe fn cert_find_certificate_in_store_impl(
    h_cert_store: *mut c_void,
    dw_find_type: u32,
    pv_find_para: *const c_void,
    p_prev_cert_context: *mut c_void,
) -> *mut c_void {
    let mut st = STORE_STATE.lock().unwrap();
    let Some((certs, enum_archived)) = st
        .stores
        .get(&(h_cert_store as usize))
        .map(|s| (s.certs.clone(), s.enum_archived))
    else {
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
            // Per MSDN CERT_ARCHIVED_PROP_ID: skip archived certs during CERT_FIND_ANY
            // enumeration unless CERT_STORE_ENUM_ARCHIVED_FLAG was set, while keeping them
            // discoverable via explicit searches.
            if dw_find_type == CERT_FIND_ANY
                && !enum_archived
                && entry.properties.contains_key(&CERT_ARCHIVED_PROP_ID)
            {
                continue;
            }
            if cert_matches_find(&entry.owned, dw_find_type, pv_find_para) {
                st.certs.get_mut(&cert_h).unwrap().refcount += 1;
                return cert_h as *mut c_void;
            }
        }
    }
    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
    std::ptr::null_mut()
}

pub(super) unsafe fn cert_get_issuer_certificate_from_store_impl(
    h_cert_store: *mut c_void,
    p_subject_context: *const c_void,
    p_prev_issuer_context: *mut c_void,
    pdw_flags: *mut u32,
) -> *mut c_void {
    if p_subject_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let found = cert_find_certificate_in_store_impl(
        h_cert_store,
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
            let sig_ok = if !subj_cc.pb_cert_encoded.is_null() && !iss_cc.pb_cert_encoded.is_null()
            {
                let s_der = std::slice::from_raw_parts(
                    subj_cc.pb_cert_encoded,
                    subj_cc.cb_cert_encoded as usize,
                );
                let i_der = std::slice::from_raw_parts(
                    iss_cc.pb_cert_encoded,
                    iss_cc.cb_cert_encoded as usize,
                );
                match (parse_cert_der_parts(s_der), parse_cert_der_parts(i_der)) {
                    (Some(sp), Some(ip)) => {
                        verify_cert_sig_with_spki(&sp, ip.spki_alg_inner, ip.spki_key_bits)
                    }
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
            if cert_verify_time_validity_impl(std::ptr::null(), iss_cc.p_cert_info as *const c_void)
                != 0
            {
                out_flags |= 0x0000_0002;
            }
        }
        std::ptr::write_unaligned(pdw_flags, out_flags);
    }
    found
}

pub(super) unsafe fn cert_get_subject_certificate_from_store_impl(
    h_cert_store: *mut c_void,
    p_cert_id: *const c_void,
) -> *mut c_void {
    cert_find_certificate_in_store_impl(
        h_cert_store,
        CERT_FIND_SUBJECT_CERT,
        p_cert_id,
        std::ptr::null_mut(),
    )
}

// ---------------------------------------------------------------------------
// PKCS#7 Query & Message Handle APIs
// ---------------------------------------------------------------------------

pub(super) fn crypt_msg_close_impl(h_msg: *mut c_void) -> BOOL {
    if !h_msg.is_null() {
        let key = h_msg as usize;
        if MSGS.remove(key).is_none() && !super::pkcs::try_close_decode_msg(key) {
            stub!("CryptMsgClose({h_msg:p}): foreign message ignored");
        }
    }
    TRUE
}

pub(super) unsafe fn crypt_query_object_impl(
    dw_object_type: u32,
    pv_object: *const c_void,
    dw_expected_content_type_flags: u32,
    dw_expected_format_type_flags: u32,
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
            let store =
                cert_open_store_impl(CERT_STORE_PROV_MEMORY as *const u8, std::ptr::null());
            for der in &parsed.certs_der {
                let _ = add_der_to_store(store, der, CERT_STORE_ADD_ALWAYS, std::ptr::null_mut());
            }
            std::ptr::write_unaligned(ph_cert_store, store);
        }
        if !ph_msg.is_null() {
            let id = MSGS.insert(parsed);
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
                let store =
                    cert_open_store_impl(CERT_STORE_PROV_MEMORY as *const u8, std::ptr::null());
                let _ =
                    add_der_to_store(store, &bytes, CERT_STORE_ADD_ALWAYS, std::ptr::null_mut());
                std::ptr::write_unaligned(ph_cert_store, store);
            }
            if !ppv_context.is_null() {
                let mut st = STORE_STATE.lock().unwrap();
                let h = insert_cert_locked(&mut st, owned, vec![bytes], None, 0);
                std::ptr::write_unaligned(ppv_context, h as *mut c_void);
            }
            return TRUE;
        }
    }

    crate::teb::set_last_error(CRYPT_E_NO_MATCH);
    FALSE
}

pub(super) unsafe fn crypt_msg_get_param_impl(
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
    if let Some(res) = super::pkcs::try_get_decode_msg_param(
        h_crypt_msg as usize,
        dw_param_type,
        dw_index,
        pv_data,
        pcb_data,
    ) {
        return res;
    }
    let write_bytes = |src: &[u8]| -> BOOL {
        let need = src.len() as u32;
        if pv_data.is_null() {
            unsafe { std::ptr::write_unaligned(pcb_data, need) };
            return TRUE;
        }
        let cap = unsafe { std::ptr::read_unaligned(pcb_data) };
        if cap < need {
            unsafe { std::ptr::write_unaligned(pcb_data, need) };
            crate::teb::set_last_error(err::ERROR_MORE_DATA);
            return FALSE;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), pv_data as *mut u8, src.len());
            std::ptr::write_unaligned(pcb_data, need);
        }
        TRUE
    };

    match MSGS.with(h_crypt_msg as usize, |msg| match dw_param_type {
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
                unsafe { std::ptr::write_unaligned(pcb_data, need) };
                return TRUE;
            }
            let cap = unsafe { std::ptr::read_unaligned(pcb_data) };
            if cap < need {
                unsafe { std::ptr::write_unaligned(pcb_data, need) };
                crate::teb::set_last_error(err::ERROR_MORE_DATA);
                return FALSE;
            }
            unsafe {
                let base = pv_data as *mut u8;
                let mut off = hdr_sz;

                let issuer_ptr = base.add(off);
                std::ptr::copy_nonoverlapping(
                    signer.issuer_raw.as_ptr(),
                    issuer_ptr,
                    signer.issuer_raw.len(),
                );
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
            }
            TRUE
        }
        _ => {
            stub!("CryptMsgGetParam(param={dw_param_type}, idx={dw_index}) -> FALSE");
            crate::teb::set_last_error(ERROR_CALL_NOT_IMPLEMENTED);
            FALSE
        }
    }) {
        Some(b) => b,
        None => {
            crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

// ---------------------------------------------------------------------------
// Certificate Context Properties (CertGet/SetCertificateContextProperty)
// ---------------------------------------------------------------------------

#[repr(C)]
struct CryptKeyProvInfo {
    pwsz_container_name: *mut u16,
    pwsz_prov_name: *mut u16,
    dw_prov_type: u32,
    dw_flags: u32,
    c_prov_param: u32,
    rg_prov_param: *mut c_void,
    dw_key_spec: u32,
}
const _: () = assert!(std::mem::size_of::<CryptKeyProvInfo>() == 48);

#[repr(C)]
struct CryptKeyProvParam {
    dw_param: u32,
    pb_data: *mut u8,
    cb_data: u32,
    dw_flags: u32,
}
const _: () = assert!(std::mem::size_of::<CryptKeyProvParam>() == 24);

/// Map an X.509 certificate's `signatureAlgorithm` AlgorithmIdentifier inner bytes to its
/// exact hash digest per `CryptHashToBeSigned` / `CERT_SIGNATURE_HASH_PROP_ID`, or `None`
/// when the signature algorithm has no supported single hash digest (causing the caller to
/// fail with `NTE_BAD_ALGID` rather than silently returning a mismatched SHA-256 hash).
fn cert_signature_hash_digest(sig_alg_inner: &[u8]) -> Option<crate::misc::bcrypt::Digest> {
    use crate::misc::bcrypt::Digest;

    const OID_RSASSA_PSS: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0A];
    const OID_HASH_SHA1: &[u8] = &[0x2B, 0x0E, 0x03, 0x02, 0x1A];
    const OID_HASH_SHA256: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01];
    const OID_HASH_SHA384: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02];
    const OID_HASH_SHA512: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03];

    let (oid, params_rest) = super::asn1::parse_der_item(sig_alg_inner)?;
    if oid.tag != 0x06 {
        return None;
    }
    match oid.val {
        // md5WithRSAEncryption (1.2.840.113549.1.1.4), md5WithRSA (1.3.14.3.2.3)
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x04]
        | [0x2B, 0x0E, 0x03, 0x02, 0x03] => Some(Digest::Md5),
        // sha1WithRSAEncryption, sha1WithRSA, ecdsa-with-SHA1, id-dsa-with-sha1, dsaWithSHA1
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x05]
        | [0x2B, 0x0E, 0x03, 0x02, 0x1D]
        | [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x01]
        | [0x2A, 0x86, 0x48, 0xCE, 0x38, 0x04, 0x03]
        | [0x2B, 0x0E, 0x03, 0x02, 0x1B] => Some(Digest::Sha1),
        // sha256WithRSAEncryption, ecdsa-with-SHA256, id-dsa-with-sha256
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B]
        | [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02]
        | [0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x03, 0x02] => Some(Digest::Sha256),
        // sha384WithRSAEncryption, ecdsa-with-SHA384, id-dsa-with-sha384
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0C]
        | [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x03]
        | [0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x03, 0x03] => Some(Digest::Sha384),
        // sha512WithRSAEncryption, ecdsa-with-SHA512, id-dsa-with-sha512
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0D]
        | [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x04]
        | [0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x03, 0x04] => Some(Digest::Sha512),
        // RSASSA-PSS (1.2.840.113549.1.1.10): RFC 4055 §3.1 DEFAULT hashAlgorithm is SHA-1.
        OID_RSASSA_PSS => {
            if params_rest.is_empty() {
                return Some(Digest::Sha1);
            }
            let (params_seq, trailing) = super::asn1::parse_der_item(params_rest)?;
            if params_seq.tag != 0x30 || !trailing.is_empty() {
                return None;
            }
            let fields = super::asn1::der_item_children_bounded(params_seq.val, 8)?;
            for f in fields {
                if f.tag == 0xA0 {
                    let (alg_seq, rem) = super::asn1::parse_der_item(f.val)?;
                    if alg_seq.tag != 0x30 || !rem.is_empty() {
                        return None;
                    }
                    let (hash_oid, _) = super::asn1::parse_der_item(alg_seq.val)?;
                    if hash_oid.tag != 0x06 {
                        return None;
                    }
                    return match hash_oid.val {
                        OID_HASH_SHA1 => Some(Digest::Sha1),
                        OID_HASH_SHA256 => Some(Digest::Sha256),
                        OID_HASH_SHA384 => Some(Digest::Sha384),
                        OID_HASH_SHA512 => Some(Digest::Sha512),
                        _ => None,
                    };
                }
            }
            Some(Digest::Sha1)
        }
        _ => None,
    }
}

pub(super) unsafe fn cert_get_certificate_context_property_impl(
    p_cert_context: *const c_void,
    dw_prop_id: u32,
    pv_data: *mut c_void,
    pcb_data: *mut u32,
) -> BOOL {
    if p_cert_context.is_null() || pcb_data.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let cc = std::ptr::read_unaligned(p_cert_context as *const CertContext);
    let der_bytes: Vec<u8> = if !cc.pb_cert_encoded.is_null() && cc.cb_cert_encoded > 0 {
        std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize).to_vec()
    } else {
        Vec::new()
    };

    // Check explicit stored properties and private key state on registered contexts first.
    let (stored_prop, stored_kpi, has_priv_key) = {
        let st = STORE_STATE.lock().unwrap();
        if let Some(entry) = st.certs.get(&(p_cert_context as usize)) {
            (
                entry.properties.get(&dw_prop_id).cloned(),
                entry.key_prov_info.clone(),
                entry.private_key_pkcs8.is_some(),
            )
        } else {
            (None, None, false)
        }
    };

    // Special case: CERT_KEY_PROV_INFO_PROP_ID (2) contains internal pointers into the caller buffer.
    if dw_prop_id == CERT_KEY_PROV_INFO_PROP_ID {
        let kpi = if let Some(s) = stored_kpi {
            Some(s)
        } else if has_priv_key {
            Some(StoredKeyProvInfo {
                container_name: Some(
                    format!("PFX-{:x}\0", p_cert_context as usize)
                        .encode_utf16()
                        .collect(),
                ),
                prov_name: Some(
                    "Microsoft Enhanced RSA and AES Cryptographic Provider\0"
                        .encode_utf16()
                        .collect(),
                ),
                dw_prov_type: 24, // PROV_RSA_AES
                dw_flags: 0,
                params: Vec::new(),
                dw_key_spec: 1, // AT_KEYEXCHANGE
            })
        } else {
            None
        };
        let Some(kpi) = kpi else {
            crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
            return FALSE;
        };
        let hdr_sz = std::mem::size_of::<CryptKeyProvInfo>();
        let param_sz = std::mem::size_of::<CryptKeyProvParam>();
        let params_off = hdr_sz; // 48 is 8-byte aligned
        let mut cursor = params_off + kpi.params.len() * param_sz;
        let cont_off = if let Some(ref c) = kpi.container_name {
            let o = cursor;
            cursor += c.len() * 2;
            Some(o)
        } else {
            None
        };
        let prov_off = if let Some(ref p) = kpi.prov_name {
            let o = cursor;
            cursor += p.len() * 2;
            Some(o)
        } else {
            None
        };
        let mut param_data_offs = Vec::with_capacity(kpi.params.len());
        for pm in &kpi.params {
            if pm.data.is_empty() {
                param_data_offs.push(None);
            } else {
                let o = cursor;
                cursor += pm.data.len();
                param_data_offs.push(Some(o));
            }
        }
        let need = cursor as u32;
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
        let cont_ptr = if let (Some(o), Some(ref c)) = (cont_off, &kpi.container_name) {
            let p = base.add(o) as *mut u16;
            for (i, &u) in c.iter().enumerate() {
                std::ptr::write_unaligned(p.add(i), u);
            }
            p
        } else {
            std::ptr::null_mut()
        };
        let prov_ptr = if let (Some(o), Some(ref pr)) = (prov_off, &kpi.prov_name) {
            let p = base.add(o) as *mut u16;
            for (i, &u) in pr.iter().enumerate() {
                std::ptr::write_unaligned(p.add(i), u);
            }
            p
        } else {
            std::ptr::null_mut()
        };
        let rg_param_ptr = if kpi.params.is_empty() {
            std::ptr::null_mut()
        } else {
            let arr = base.add(params_off) as *mut CryptKeyProvParam;
            for (i, pm) in kpi.params.iter().enumerate() {
                let data_ptr = if let Some(doff) = param_data_offs[i] {
                    let dp = base.add(doff);
                    std::ptr::copy_nonoverlapping(pm.data.as_ptr(), dp, pm.data.len());
                    dp
                } else {
                    std::ptr::null_mut()
                };
                std::ptr::write_unaligned(
                    arr.add(i),
                    CryptKeyProvParam {
                        dw_param: pm.dw_param,
                        pb_data: data_ptr,
                        cb_data: pm.data.len() as u32,
                        dw_flags: pm.dw_flags,
                    },
                );
            }
            arr as *mut c_void
        };
        std::ptr::write_unaligned(
            pv_data as *mut CryptKeyProvInfo,
            CryptKeyProvInfo {
                pwsz_container_name: cont_ptr,
                pwsz_prov_name: prov_ptr,
                dw_prov_type: kpi.dw_prov_type,
                dw_flags: kpi.dw_flags,
                c_prov_param: kpi.params.len() as u32,
                rg_prov_param: rg_param_ptr,
                dw_key_spec: kpi.dw_key_spec,
            },
        );
        std::ptr::write_unaligned(pcb_data, need);
        return TRUE;
    }

    if dw_prop_id == CERT_SIGNATURE_HASH_PROP_ID && stored_prop.is_none() {
        let Some(bytes) = parse_cert_der_parts(&der_bytes)
            .and_then(|parts| {
                cert_signature_hash_digest(parts.sig_alg_inner).map(|dig| dig.hash(parts.tbs_raw))
            })
        else {
            crate::teb::set_last_error(NTE_BAD_ALGID);
            return FALSE;
        };
        let need = bytes.len() as u32;
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
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), pv_data as *mut u8, bytes.len());
        std::ptr::write_unaligned(pcb_data, need);
        return TRUE;
    }

    let computed: Option<Vec<u8>> = if let Some(p) = stored_prop {
        Some(p)
    } else {
        match dw_prop_id {
            CERT_SHA1_HASH_PROP_ID => (!der_bytes.is_empty())
                .then(|| crate::misc::bcrypt::Digest::Sha1.hash(&der_bytes)),
            CERT_MD5_HASH_PROP_ID => (!der_bytes.is_empty())
                .then(|| crate::misc::bcrypt::Digest::Md5.hash(&der_bytes)),
            CERT_SHA256_HASH_PROP_ID => (!der_bytes.is_empty())
                .then(|| crate::misc::bcrypt::Digest::Sha256.hash(&der_bytes)),
            CERT_KEY_IDENTIFIER_PROP_ID => parse_cert_der_parts(&der_bytes).map(|parts| {
                const OID_SKI: &[u8] = &[0x55, 0x1D, 0x0E];
                for (oid, _, val) in &parts.extensions {
                    if *oid == OID_SKI {
                        if let Some((oct, _)) = super::asn1::parse_der_item(val) {
                            if oct.tag == 0x04 {
                                return oct.val.to_vec();
                            }
                        }
                    }
                }
                crate::misc::bcrypt::Digest::Sha1.hash(parts.spki_key_bits)
            }),
            CERT_SUBJECT_PUBLIC_KEY_MD5_HASH_PROP_ID => parse_cert_der_parts(&der_bytes)
                .map(|parts| crate::misc::bcrypt::Digest::Md5.hash(parts.spki_key_bits)),
            CERT_ISSUER_PUBLIC_KEY_MD5_HASH_PROP_ID => parse_cert_der_parts(&der_bytes).and_then(|parts| {
                (parts.issuer_inner == parts.subject_inner)
                    .then(|| crate::misc::bcrypt::Digest::Md5.hash(parts.spki_key_bits))
            }),
            CERT_SUBJECT_NAME_MD5_HASH_PROP_ID => parse_cert_der_parts(&der_bytes)
                .map(|parts| crate::misc::bcrypt::Digest::Md5.hash(parts.subject_raw)),
            CERT_ENHKEY_USAGE_PROP_ID => parse_cert_der_parts(&der_bytes).and_then(|parts| {
                const OID_EKU: &[u8] = &[0x55, 0x1D, 0x25];
                parts
                    .extensions
                    .iter()
                    .find(|(oid, _, _)| *oid == OID_EKU)
                    .map(|(_, _, val)| val.to_vec())
            }),
            CERT_PUBKEY_ALG_PARA_PROP_ID => {
                if !cc.p_cert_info.is_null() {
                    let ci = std::ptr::read_unaligned(cc.p_cert_info);
                    let params = ci.subject_public_key_info.algorithm.parameters;
                    if !params.pb_data.is_null() && params.cb_data > 0 {
                        Some(
                            std::slice::from_raw_parts(params.pb_data, params.cb_data as usize)
                                .to_vec(),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            CERT_ACCESS_STATE_PROP_ID => Some(0u32.to_le_bytes().to_vec()),
            CERT_KEY_SPEC_PROP_ID if has_priv_key => Some(1u32.to_le_bytes().to_vec()),
            CERT_KEY_PROV_HANDLE_PROP_ID | CERT_DESCRIPTION_PROP_ID | CERT_FRIENDLY_NAME_PROP_ID => None,
            _ => None,
        }
    };

    let Some(bytes) = computed else {
        crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
        return FALSE;
    };
    let need = bytes.len() as u32;
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
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), pv_data as *mut u8, bytes.len());
    }
    std::ptr::write_unaligned(pcb_data, need);
    TRUE
}

pub(super) unsafe fn cert_set_certificate_context_property_impl(
    p_cert_context: *const c_void,
    dw_prop_id: u32,
    _dw_flags: u32,
    pv_data: *const c_void,
) -> BOOL {
    if p_cert_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mut st = STORE_STATE.lock().unwrap();
    let Some(entry) = st.certs.get_mut(&(p_cert_context as usize)) else {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    };
    const CERT_NCRYPT_KEY_SPEC: u32 = 0xFFFF_FFFF;
    if pv_data.is_null() {
        if dw_prop_id == CERT_KEY_PROV_INFO_PROP_ID {
            entry.key_prov_info = None;
        }
        if dw_prop_id == CERT_KEY_CONTEXT_PROP_ID {
            entry.properties.remove(&CERT_KEY_PROV_HANDLE_PROP_ID);
            entry.properties.remove(&CERT_NCRYPT_KEY_HANDLE_PROP_ID);
            entry.properties.remove(&CERT_KEY_SPEC_PROP_ID);
        } else if matches!(
            dw_prop_id,
            CERT_KEY_PROV_HANDLE_PROP_ID
                | CERT_NCRYPT_KEY_HANDLE_PROP_ID
                | CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID
        ) {
            entry.properties.remove(&CERT_KEY_CONTEXT_PROP_ID);
            entry.properties.remove(&CERT_KEY_PROV_HANDLE_PROP_ID);
            entry.properties.remove(&CERT_NCRYPT_KEY_HANDLE_PROP_ID);
        }
        entry.properties.remove(&dw_prop_id);
        return TRUE;
    }
    if dw_prop_id == CERT_KEY_CONTEXT_PROP_ID {
        // Per MSDN CertSetCertificateContextProperty: CERT_KEY_CONTEXT_PROP_ID receives a
        // CERT_KEY_CONTEXT* (24B on Win64: cbSize, hCryptProv/hNCryptKey, dwKeySpec) — NEVER a
        // CRYPT_DATA_BLOB* and NEVER dereference the opaque provider handle.
        let kc = std::ptr::read_unaligned(pv_data as *const CertKeyContext);
        if kc.cb_size < std::mem::size_of::<CertKeyContext>() as u32 {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        let normalized = CertKeyContext {
            cb_size: std::mem::size_of::<CertKeyContext>() as u32,
            h_crypt_prov: kc.h_crypt_prov,
            dw_key_spec: kc.dw_key_spec,
        };
        let mut raw = vec![0u8; std::mem::size_of::<CertKeyContext>()];
        std::ptr::write_unaligned(raw.as_mut_ptr() as *mut CertKeyContext, normalized);
        entry.properties.insert(CERT_KEY_CONTEXT_PROP_ID, raw);
        if kc.dw_key_spec == CERT_NCRYPT_KEY_SPEC {
            entry.properties.remove(&CERT_KEY_PROV_HANDLE_PROP_ID);
            entry
                .properties
                .insert(CERT_NCRYPT_KEY_HANDLE_PROP_ID, kc.h_crypt_prov.to_le_bytes().to_vec());
        } else {
            entry.properties.remove(&CERT_NCRYPT_KEY_HANDLE_PROP_ID);
            entry
                .properties
                .insert(CERT_KEY_PROV_HANDLE_PROP_ID, kc.h_crypt_prov.to_le_bytes().to_vec());
        }
        entry
            .properties
            .insert(CERT_KEY_SPEC_PROP_ID, kc.dw_key_spec.to_le_bytes().to_vec());
        return TRUE;
    }
    if matches!(
        dw_prop_id,
        CERT_KEY_PROV_HANDLE_PROP_ID
            | CERT_NCRYPT_KEY_HANDLE_PROP_ID
            | CERT_NCRYPT_KEY_HANDLE_TRANSFER_PROP_ID
    ) {
        // Per Windows SDK wincrypt.h / MSDN CertSetCertificateContextProperty:
        // pvData IS the opaque HCRYPTPROV / NCRYPT_KEY_HANDLE value itself — NEVER dereference it!
        let handle = pv_data as usize;
        let is_ncrypt = dw_prop_id != CERT_KEY_PROV_HANDLE_PROP_ID;
        let dw_key_spec = if is_ncrypt {
            CERT_NCRYPT_KEY_SPEC
        } else {
            entry
                .properties
                .get(&CERT_KEY_SPEC_PROP_ID)
                .and_then(|b| b.get(..4))
                .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                .unwrap_or(0)
        };
        let kc = CertKeyContext {
            cb_size: std::mem::size_of::<CertKeyContext>() as u32,
            h_crypt_prov: handle,
            dw_key_spec,
        };
        let mut raw = vec![0u8; std::mem::size_of::<CertKeyContext>()];
        std::ptr::write_unaligned(raw.as_mut_ptr() as *mut CertKeyContext, kc);
        entry.properties.insert(CERT_KEY_CONTEXT_PROP_ID, raw);
        if is_ncrypt {
            entry.properties.remove(&CERT_KEY_PROV_HANDLE_PROP_ID);
            entry
                .properties
                .insert(CERT_NCRYPT_KEY_HANDLE_PROP_ID, handle.to_le_bytes().to_vec());
            entry
                .properties
                .insert(CERT_KEY_SPEC_PROP_ID, dw_key_spec.to_le_bytes().to_vec());
        } else {
            entry.properties.remove(&CERT_NCRYPT_KEY_HANDLE_PROP_ID);
            entry
                .properties
                .insert(CERT_KEY_PROV_HANDLE_PROP_ID, handle.to_le_bytes().to_vec());
        }
        return TRUE;
    }
    if dw_prop_id == CERT_KEY_PROV_INFO_PROP_ID {
        // Per MSDN CertSetCertificateContextProperty: CERT_KEY_PROV_INFO_PROP_ID receives a
        // CRYPT_KEY_PROV_INFO* (NOT a CRYPT_DATA_BLOB*). Deep-copy all pointed-to strings & params.
        let raw = std::ptr::read_unaligned(pv_data as *const CryptKeyProvInfo);
        if raw.c_prov_param > 0 && raw.rg_prov_param.is_null() {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        let read_wide_z = |p: *const u16| -> Option<Vec<u16>> {
            if p.is_null() {
                return None;
            }
            let mut out = Vec::new();
            let mut i = 0usize;
            loop {
                let u = std::ptr::read_unaligned(p.add(i));
                out.push(u);
                if u == 0 || i >= 4096 {
                    if u != 0 {
                        out.push(0);
                    }
                    break;
                }
                i += 1;
            }
            Some(out)
        };
        let container_name = read_wide_z(raw.pwsz_container_name);
        let prov_name = read_wide_z(raw.pwsz_prov_name);
        let mut params = Vec::with_capacity(raw.c_prov_param as usize);
        for i in 0..raw.c_prov_param as usize {
            let pm = std::ptr::read_unaligned(
                (raw.rg_prov_param as *const CryptKeyProvParam).add(i),
            );
            if pm.cb_data > 0 && pm.pb_data.is_null() {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let data = if pm.cb_data == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(pm.pb_data, pm.cb_data as usize).to_vec()
            };
            params.push(StoredKeyProvParam {
                dw_param: pm.dw_param,
                data,
                dw_flags: pm.dw_flags,
            });
        }
        entry.key_prov_info = Some(StoredKeyProvInfo {
            container_name,
            prov_name,
            dw_prov_type: raw.dw_prov_type,
            dw_flags: raw.dw_flags,
            params,
            dw_key_spec: raw.dw_key_spec,
        });
        return TRUE;
    }
    let val = match dw_prop_id {
        CERT_KEY_SPEC_PROP_ID | CERT_ACCESS_STATE_PROP_ID => {
            std::ptr::read_unaligned(pv_data as *const u32).to_le_bytes().to_vec()
        }
        CERT_DATE_STAMP_PROP_ID => {
            std::ptr::read_unaligned(pv_data as *const u64).to_le_bytes().to_vec()
        }
        _ => {
            // CRYPT_DATA_BLOB pointer
            let blob = std::ptr::read_unaligned(pv_data as *const CryptDataBlob);
            if blob.cb_data > 0 && blob.pb_data.is_null() {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            if blob.cb_data == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize).to_vec()
            }
        }
    };
    if dw_prop_id == CERT_ENHKEY_USAGE_PROP_ID && parse_eku_der_oids(&val).is_none() {
        crate::teb::set_last_error(0x8009_310B); // CRYPT_E_ASN1_BADTAG
        return FALSE;
    }
    entry.properties.insert(dw_prop_id, val);
    TRUE
}

// ---------------------------------------------------------------------------
// CRL & CTL Contexts (CertCreate/Duplicate/Free CRL & CTL Context)
// ---------------------------------------------------------------------------

#[repr(C)]
pub(super) struct CrlContext {
    pub(super) dw_cert_encoding_type: u32,
    pub(super) pb_crl_encoded: *const u8,
    pub(super) cb_crl_encoded: u32,
    pub(super) p_crl_info: *mut c_void,
    pub(super) h_cert_store: *mut c_void,
}
const _: () = assert!(std::mem::size_of::<CrlContext>() == 40);

#[repr(C)]
pub(super) struct CtlContext {
    pub(super) dw_msg_and_cert_encoding_type: u32,
    pub(super) pb_ctl_encoded: *const u8,
    pub(super) cb_ctl_encoded: u32,
    pub(super) p_ctl_info: *mut c_void,
    pub(super) h_cert_store: *mut c_void,
    pub(super) h_crypt_msg: *mut c_void,
    pub(super) pb_ctl_content: *const u8,
    pub(super) cb_ctl_content: u32,
}
const _: () = assert!(std::mem::size_of::<CtlContext>() == 64);

struct RegisteredCrl {
    _der: Box<[u8]>,
    _info_buf: *mut c_void,
    _ctx: Box<CrlContext>,
    refcount: usize,
}
unsafe impl Send for RegisteredCrl {}
impl Drop for RegisteredCrl {
    fn drop(&mut self) {
        if !self._info_buf.is_null() {
            unsafe {
                crate::kernel32::memory::LocalFree(self._info_buf);
            }
        }
    }
}

struct RegisteredCtl {
    _der: Box<[u8]>,
    _content_der: Box<[u8]>,
    _info_buf: *mut c_void,
    _h_crypt_msg: *mut c_void,
    _ctx: Box<CtlContext>,
    refcount: usize,
}
unsafe impl Send for RegisteredCtl {}
impl Drop for RegisteredCtl {
    fn drop(&mut self) {
        if !self._info_buf.is_null() {
            unsafe {
                crate::kernel32::memory::LocalFree(self._info_buf);
            }
        }
        if !self._h_crypt_msg.is_null() {
            let _ = super::pkcs::try_close_decode_msg(self._h_crypt_msg as usize);
        }
    }
}

static CRLS: LazyLock<Mutex<HashMap<usize, RegisteredCrl>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static CTLS: LazyLock<Mutex<HashMap<usize, RegisteredCtl>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub(super) unsafe fn cert_create_crl_context_impl(
    dw_cert_encoding_type: u32,
    pb_crl_encoded: *const u8,
    cb_crl_encoded: u32,
) -> *mut c_void {
    if (dw_cert_encoding_type & X509_ASN_ENCODING) == 0
        || pb_crl_encoded.is_null()
        || cb_crl_encoded == 0
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let slice = std::slice::from_raw_parts(pb_crl_encoded, cb_crl_encoded as usize);
    let Some((seq, _)) = super::asn1::parse_der_item(slice) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if seq.tag != 0x30 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    // Require a valid signed CertificateList SEQUENCE { tbsCertList, sigAlg, sigVal } and
    // successful CRL_INFO decoding; per MSDN CertCreateCRLContext returns NULL on decode failure.
    let Some(kids) = super::asn1::der_item_children(seq.val) else {
        crate::teb::set_last_error(0x8009_310B); // CRYPT_E_ASN1_BADTAG
        return std::ptr::null_mut();
    };
    if kids.len() != 3 || kids[0].tag != 0x30 || kids[1].tag != 0x30 || kids[2].tag != 0x03 {
        crate::teb::set_last_error(0x8009_310B); // CRYPT_E_ASN1_BADTAG
        return std::ptr::null_mut();
    }
    let mut info_ptr: *mut c_void = std::ptr::null_mut();
    let mut info_sz = 0u32;
    let decoded_ok = super::decode::crypt_decode_object_ex_impl(
        X509_ASN_ENCODING,
        super::decode::X509_CERT_CRL_TO_BE_SIGNED as *const u8,
        kids[0].raw.as_ptr(),
        kids[0].raw.len() as u32,
        0x8000 | super::decode::CRYPT_DECODE_TO_BE_SIGNED_FLAG,
        std::ptr::null(),
        &mut info_ptr as *mut *mut c_void as *mut c_void,
        &mut info_sz,
    );
    if decoded_ok == FALSE || info_ptr.is_null() {
        if !info_ptr.is_null() {
            crate::kernel32::memory::LocalFree(info_ptr);
        }
        return std::ptr::null_mut();
    }
    let der = slice.to_vec().into_boxed_slice();
    let ctx = Box::new(CrlContext {
        dw_cert_encoding_type: X509_ASN_ENCODING,
        pb_crl_encoded: der.as_ptr(),
        cb_crl_encoded: der.len() as u32,
        p_crl_info: info_ptr,
        h_cert_store: std::ptr::null_mut(),
    });
    let ptr = &*ctx as *const CrlContext as usize;
    CRLS.lock().unwrap().insert(
        ptr,
        RegisteredCrl {
            _der: der,
            _info_buf: info_ptr,
            _ctx: ctx,
            refcount: 1,
        },
    );
    ptr as *mut c_void
}

pub(super) fn cert_duplicate_crl_context_impl(ctx: *const c_void) -> *mut c_void {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let mut map = CRLS.lock().unwrap();
    if let Some(e) = map.get_mut(&(ctx as usize)) {
        e.refcount += 1;
    }
    ctx as *mut c_void
}

pub(super) fn cert_free_crl_context_impl(ctx: *mut c_void) -> BOOL {
    if ctx.is_null() {
        return TRUE;
    }
    let mut map = CRLS.lock().unwrap();
    let key = ctx as usize;
    if let Some(e) = map.get_mut(&key) {
        if e.refcount > 1 {
            e.refcount -= 1;
        } else {
            map.remove(&key);
        }
    } else {
        log::debug!("CertFreeCRLContext({ctx:p}): unknown/already-freed context");
    }
    TRUE
}

pub(super) unsafe fn cert_create_ctl_context_impl(
    dw_msg_and_cert_encoding_type: u32,
    pb_ctl_encoded: *const u8,
    cb_ctl_encoded: u32,
) -> *mut c_void {
    if (dw_msg_and_cert_encoding_type & (X509_ASN_ENCODING | PKCS_7_ASN_ENCODING)) == 0
        || pb_ctl_encoded.is_null()
        || cb_ctl_encoded == 0
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let slice = std::slice::from_raw_parts(pb_ctl_encoded, cb_ctl_encoded as usize);
    // Extract inner CertificateTrustList DER content from the signed PKCS#7 ContentInfo wrapper.
    let Some((ci, _)) = super::asn1::parse_der_item(slice) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if ci.tag != 0x30 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    let Some(ci_kids) = super::asn1::der_item_children(ci.val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    const OID_SIGNED_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02];
    const OID_CTL: &[u8] = &[0x2B, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x0A, 0x01];
    if ci_kids.len() < 2
        || ci_kids[0].tag != 0x06
        || ci_kids[0].val != OID_SIGNED_DATA
        || ci_kids[1].tag != 0xA0
    {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    let Some((sd, _)) = super::asn1::parse_der_item(ci_kids[1].val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    let Some(sd_kids) = super::asn1::der_item_children(sd.val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if sd_kids.len() < 3 || sd_kids[2].tag != 0x30 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    let Some(eci_kids) = super::asn1::der_item_children(sd_kids[2].val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if eci_kids.len() < 2
        || eci_kids[0].tag != 0x06
        || eci_kids[0].val != OID_CTL
        || eci_kids[1].tag != 0xA0
    {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    let Some((inner, _)) = super::asn1::parse_der_item(eci_kids[1].val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    let ctl_content_slice = if inner.tag == 0x04 { inner.val } else { inner.raw };
    let Some(p_ctl_info) = super::decode::decode_ctl_info_alloc(ctl_content_slice) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };

    // Open a real PKCS#7 decode message handle for hCryptMsg so callers can inspect CTL signers.
    let h_crypt_msg = super::pkcs::crypt_msg_open_to_decode_impl(
        dw_msg_and_cert_encoding_type,
        0,
        0,
        0,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    );
    if h_crypt_msg.is_null()
        || super::pkcs::crypt_msg_update_impl(h_crypt_msg, pb_ctl_encoded, cb_ctl_encoded, TRUE)
            == FALSE
    {
        if !h_crypt_msg.is_null() {
            let _ = super::pkcs::try_close_decode_msg(h_crypt_msg as usize);
        }
        crate::kernel32::memory::LocalFree(p_ctl_info);
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }

    let der = slice.to_vec().into_boxed_slice();
    let content_der = ctl_content_slice.to_vec().into_boxed_slice();
    let ctx = Box::new(CtlContext {
        dw_msg_and_cert_encoding_type,
        pb_ctl_encoded: der.as_ptr(),
        cb_ctl_encoded: der.len() as u32,
        p_ctl_info,
        h_cert_store: std::ptr::null_mut(),
        h_crypt_msg,
        pb_ctl_content: content_der.as_ptr(),
        cb_ctl_content: content_der.len() as u32,
    });
    let ptr = &*ctx as *const CtlContext as usize;
    CTLS.lock().unwrap().insert(
        ptr,
        RegisteredCtl {
            _der: der,
            _content_der: content_der,
            _info_buf: p_ctl_info,
            _h_crypt_msg: h_crypt_msg,
            _ctx: ctx,
            refcount: 1,
        },
    );
    ptr as *mut c_void
}

pub(super) fn cert_duplicate_ctl_context_impl(ctx: *const c_void) -> *mut c_void {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let mut map = CTLS.lock().unwrap();
    if let Some(e) = map.get_mut(&(ctx as usize)) {
        e.refcount += 1;
    }
    ctx as *mut c_void
}

pub(super) fn cert_free_ctl_context_impl(ctx: *mut c_void) -> BOOL {
    if ctx.is_null() {
        return TRUE;
    }
    let mut map = CTLS.lock().unwrap();
    let key = ctx as usize;
    if let Some(e) = map.get_mut(&key) {
        if e.refcount > 1 {
            e.refcount -= 1;
        } else {
            map.remove(&key);
        }
    } else {
        log::debug!("CertFreeCTLContext({ctx:p}): unknown/already-freed context");
    }
    TRUE
}
