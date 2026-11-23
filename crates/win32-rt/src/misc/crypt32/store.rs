//! Certificate contexts, stores, name formatting, and PKCS#7 message handles for `crypt32.dll`.

use super::asn1::{
    all_dn_values_lower, encode_der_tlv, parse_cert_der_parts, PKCS_7_ASN_ENCODING,
    X509_ASN_ENCODING,
};
use super::chain::{cert_verify_time_validity_impl, verify_cert_sig_with_spki};
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

#[derive(Default)]
struct StoreState {
    certs: HashMap<usize, RegisteredCert>,
    /// Map `SubjectPublicKeyInfo` field address -> cert handle for exact DER SPKI lookup.
    spki_to_cert: HashMap<usize, usize>,
    stores: HashMap<usize, RegisteredStore>,
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
        },
    );
    ptr
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
) -> Option<(Vec<Vec<u8>>, Option<PeerCertVerifyResult>)> {
    let st = STORE_STATE.lock().unwrap();
    let entry = st.certs.get(&cert_handle)?;
    let certs = if !entry.peer_chain_der.is_empty() {
        entry.peer_chain_der.clone()
    } else {
        vec![entry.owned.raw_der().to_vec()]
    };
    Some((certs, entry.peer_verify.clone()))
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

/// Collect DER bytes of all certificates in a registered `HCERTSTORE`.
pub(crate) fn store_certs_der(h_cert_store: *mut c_void) -> Vec<Vec<u8>> {
    if h_cert_store.is_null() {
        return Vec::new();
    }
    let st = STORE_STATE.lock().unwrap();
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

fn open_named_system_store(store_name: &str) -> *mut c_void {
    let upper = store_name.trim().to_ascii_uppercase();
    let seed_roots = matches!(upper.as_str(), "ROOT" | "AUTHROOT" | "CA");
    let mut st = STORE_STATE.lock().unwrap();
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

pub(super) unsafe fn cert_open_store_impl(
    lpsz_store_provider: *const u8,
    pv_para: *const c_void,
) -> *mut c_void {
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
    let mut st = STORE_STATE.lock().unwrap();
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
    let mut st = STORE_STATE.lock().unwrap();
    let store_key = h_cert_store as usize;
    if !st.stores.contains_key(&store_key) {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }

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
    add_der_to_store(h_cert_store, der, dw_add_disposition, pp_store_context)
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
    let Some(certs) = st
        .stores
        .get(&(h_cert_store as usize))
        .map(|s| s.certs.clone())
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
    let Some(certs) = st
        .stores
        .get(&(h_cert_store as usize))
        .map(|s| s.certs.clone())
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
    if !h_msg.is_null() && MSGS.remove(h_msg as usize).is_none() {
        stub!("CryptMsgClose({h_msg:p}): foreign message ignored");
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
