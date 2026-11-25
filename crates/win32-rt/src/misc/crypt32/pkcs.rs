//! PKCS#7 / CMS message decoding & verification (`CryptMsgOpenToDecode`, `CryptMsgUpdate`,
//! `CryptVerifyMessageSignature`) and RFC 7292 PKCS#12 / PFX import (`PFXImportCertStore`)
//! backed by Apple CommonCrypto, `bcrypt`, and `rustls::crypto::ring`.

use super::asn1::{
    ber_item_children, der_item_children, encode_der_tlv, parse_ber_item, parse_cert_der_parts,
    parse_der_item, DerItem, PKCS_7_ASN_ENCODING, X509_ASN_ENCODING,
};
use super::store::{
    add_der_with_meta_to_store, cert_open_store_impl, create_cert_context_from_der,
    CERT_FRIENDLY_NAME_PROP_ID, CERT_STORE_PROV_MEMORY, CRYPT_E_NOT_FOUND,
};
use crate::handles::HandleTable;
use crate::misc::bcrypt::{Digest, RsaPublicKey};
use crate::misc::wintrust::{self, CryptDataBlob};
use crate::types::*;
use std::ffi::c_void;

const CRYPT_E_ASN1_BADTAG: u32 = 0x8009_310B;
const CRYPT_E_SIGNER_NOT_FOUND: u32 = 0x8009_100E;
const CRYPT_E_INVALID_MSG_TYPE: u32 = 0x8009_200A;
const NTE_BAD_SIGNATURE: u32 = 0x8009_0006;
const ERROR_INVALID_PASSWORD: u32 = 86;

pub(super) const CMSG_DATA: u32 = 1;
const CMSG_SIGNED: u32 = 2;
const CMSG_ENVELOPED: u32 = 3;
const CMSG_DETACHED_FLAG: u32 = 0x0000_0004;

pub(super) const CMSG_TYPE_PARAM: u32 = 1;
pub(super) const CMSG_CONTENT_PARAM: u32 = 2;
pub(super) const CMSG_BARE_CONTENT_PARAM: u32 = 3;
pub(super) const CMSG_INNER_CONTENT_TYPE_PARAM: u32 = 4;
pub(super) const CMSG_SIGNER_COUNT_PARAM: u32 = 5;
pub(super) const CMSG_SIGNER_INFO_PARAM: u32 = 6;
pub(super) const CMSG_SIGNER_CERT_INFO_PARAM: u32 = 7;
pub(super) const CMSG_SIGNER_AUTH_ATTR_PARAM: u32 = 9;
pub(super) const CMSG_SIGNER_UNAUTH_ATTR_PARAM: u32 = 10;
pub(super) const CMSG_CERT_COUNT_PARAM: u32 = 11;
pub(super) const CMSG_CERT_PARAM: u32 = 12;
pub(super) const CMSG_ENCODED_MESSAGE: u32 = 29;
pub(super) const CMSG_VERSION_PARAM: u32 = 30;

// Common OID DER value bytes
const OID_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x01];
const OID_SIGNED_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02];
const OID_ENVELOPED_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x03];
const OID_ENCRYPTED_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x06];
const OID_CONTENT_TYPE: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x03];
const OID_MESSAGE_DIGEST: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x04];
const OID_RSA_ENCRYPTION: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01];

const OID_MD5: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x02, 0x05];
const OID_SHA1: &[u8] = &[0x2B, 0x0E, 0x03, 0x02, 0x1A];
const OID_SHA256: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01];
const OID_SHA384: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02];
const OID_SHA512: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03];

// PKCS#12 / PBES1 / PBES2 OIDs
const OID_PBE_SHA1_3DES_CBC: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x01, 0x03];
const OID_PBE_SHA1_RC2_128_CBC: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x01, 0x05];
const OID_PBE_SHA1_RC2_40_CBC: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x01, 0x06];
const OID_PBES2: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x05, 0x0D];
const OID_PBKDF2: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x05, 0x0C];
const OID_HMAC_SHA1: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x02, 0x07];
const OID_HMAC_SHA256: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x02, 0x09];
const OID_HMAC_SHA384: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x02, 0x0A];
const OID_HMAC_SHA512: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x02, 0x0B];
const OID_AES_128_CBC: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x01, 0x02];
const OID_AES_192_CBC: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x01, 0x16];
const OID_AES_256_CBC: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x01, 0x2A];
const OID_3DES_CBC: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x03, 0x07];

// PKCS#12 SafeBag & attribute OIDs
const OID_KEY_BAG: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x0A, 0x01, 0x01];
const OID_PKCS8_SHROUDED_KEY_BAG: &[u8] =
    &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x0A, 0x01, 0x02];
const OID_CERT_BAG: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x0A, 0x01, 0x03];
const OID_SAFE_CONTENTS_BAG: &[u8] =
    &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x0C, 0x0A, 0x01, 0x06];
const OID_X509_CERT_TYPE: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x16, 0x01];
const OID_FRIENDLY_NAME: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x14];
const OID_LOCAL_KEY_ID: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x15];

// CommonCrypto FFI (available on both macOS and iOS)
extern "C" {
    fn CCCrypt(
        op: u32,
        alg: u32,
        options: u32,
        key: *const c_void,
        key_length: usize,
        iv: *const c_void,
        data_in: *const c_void,
        data_in_length: usize,
        data_out: *mut c_void,
        data_out_available: usize,
        data_out_moved: *mut usize,
    ) -> i32;

    fn CCKeyDerivationPBKDF(
        algorithm: u32,
        password: *const u8,
        password_len: usize,
        salt: *const u8,
        salt_len: usize,
        prf: u32,
        rounds: u32,
        derived_key: *mut u8,
        derived_key_len: usize,
    ) -> i32;
}

const K_CC_DECRYPT: u32 = 1;
const K_CC_ALGORITHM_AES: u32 = 0;
const K_CC_ALGORITHM_3DES: u32 = 2;
const K_CC_ALGORITHM_RC2: u32 = 5;
const K_CC_OPTION_PKCS7_PADDING: u32 = 0x0001;
const K_CC_PBKDF2: u32 = 2;
const K_CC_PRF_HMAC_SHA1: u32 = 1;
const K_CC_PRF_HMAC_SHA256: u32 = 3;
const K_CC_PRF_HMAC_SHA384: u32 = 4;
const K_CC_PRF_HMAC_SHA512: u32 = 5;

fn digest_from_oid(oid: &[u8]) -> Option<Digest> {
    match oid {
        OID_MD5 => Some(Digest::Md5),
        OID_SHA1 => Some(Digest::Sha1),
        OID_SHA256 => Some(Digest::Sha256),
        OID_SHA384 => Some(Digest::Sha384),
        OID_SHA512 => Some(Digest::Sha512),
        _ => None,
    }
}

fn parse_alg_id(elem: DerItem<'_>) -> Option<(&[u8], Option<DerItem<'_>>)> {
    if elem.tag != 0x30 {
        return None;
    }
    let kids = der_item_children(elem.val)?;
    if kids.is_empty() || kids[0].tag != 0x06 {
        return None;
    }
    Some((kids[0].val, kids.get(1).copied()))
}

fn parse_der_u32_val(val: &[u8]) -> Option<u32> {
    let mut s = val;
    while s.len() > 1 && s[0] == 0 {
        s = &s[1..];
    }
    if s.is_empty() || s.len() > 4 {
        return None;
    }
    let mut v = 0u32;
    for &b in s {
        v = (v << 8) | (b as u32);
    }
    Some(v)
}

fn strip_leading_zero(mut b: &[u8]) -> &[u8] {
    while b.len() > 1 && b[0] == 0 {
        b = &b[1..];
    }
    b
}

/// Extract concatenated OCTET STRING bytes (handles both primitive 0x04 and BER constructed 0x24).
/// Maximum nesting depth allowed for recursive BER constructed OCTET STRINGs and
/// PKCS#12 `safeContentsBag` containers. Prevents stack exhaustion on crafted inputs.
pub(super) const MAX_PFX_NESTING_DEPTH: usize = 32;
pub(super) const MAX_PFX_SAFE_BAGS: usize = 512;

fn extract_octet_string(elem: DerItem<'_>) -> Option<Vec<u8>> {
    fn inner(elem: DerItem<'_>, depth: usize, out: &mut Vec<u8>) -> Option<()> {
        if depth > MAX_PFX_NESTING_DEPTH {
            return None;
        }
        if elem.tag == 0x04 {
            out.extend_from_slice(elem.val);
            return Some(());
        }
        if elem.tag == 0x24 {
            for child in ber_item_children(elem.val)? {
                inner(child, depth + 1, out)?;
            }
            return Some(());
        }
        None
    }
    let mut out = Vec::new();
    inner(elem, 0, &mut out)?;
    Some(out)
}

// ---------------------------------------------------------------------------
// Full PKCS#7 / CMS SignedData parser & signature verifier
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub(super) struct CmsSigner {
    pub signer_info_raw: Vec<u8>,
    pub issuer_raw: Vec<u8>,
    pub serial_be: Vec<u8>,
    pub subject_key_id: Option<Vec<u8>>,
    pub hash_alg_oid: Vec<u8>,
    pub auth_attrs_raw: Option<Vec<u8>>,
    pub unauth_attrs_raw: Option<Vec<u8>>,
    pub encrypted_hash: Vec<u8>,
}

#[derive(Clone)]
pub(super) struct CmsSignedMessage {
    pub msg_type: u32,
    pub version: u32,
    pub raw_encoded: Vec<u8>,
    pub inner_content_type_oid: String,
    pub inner_content_type_oid_der: Vec<u8>,
    /// Outer `ContentInfo.content` DER encoding with only the `ContentInfo` wrapper removed
    /// (returned by `CMSG_BARE_CONTENT_PARAM` — e.g. the DER-encoded `SignedData` SEQUENCE).
    pub bare_content: Vec<u8>,
    /// Inner `encapContentInfo.eContent` bytes hashed for `messageDigest` verification.
    pub digest_content: Vec<u8>,
    /// Application payload returned by `CMSG_CONTENT_PARAM` and `CryptVerifyMessageSignature`.
    pub content: Vec<u8>,
    pub certs_der: Vec<Vec<u8>>,
    pub signers: Vec<CmsSigner>,
}

fn parse_cms_signed_data(sd_seq: DerItem<'_>, raw_encoded: &[u8]) -> Option<CmsSignedMessage> {
    if sd_seq.tag != 0x30 {
        return None;
    }
    let sd_kids = ber_item_children(sd_seq.val)?;
    if sd_kids.len() < 4 || sd_kids[0].tag != 0x02 || sd_kids[2].tag != 0x30 {
        return None;
    }
    let version = parse_der_u32_val(sd_kids[0].val).unwrap_or(1);

    // encapContentInfo ::= SEQUENCE { eContentType OID, [0] EXPLICIT eContent OPTIONAL }
    let eci_kids = ber_item_children(sd_kids[2].val)?;
    if eci_kids.is_empty() || eci_kids[0].tag != 0x06 {
        return None;
    }
    let inner_oid = eci_kids[0].val;
    let inner_content_type_oid = wintrust::decode_oid_str(inner_oid);
    let (content, digest_content) = if let Some(ec) = eci_kids.get(1).filter(|e| e.tag == 0xA0) {
        if let Some((inner, _)) = parse_ber_item(ec.val) {
            if inner.tag == 0x04 || inner.tag == 0x24 {
                // RFC 5652 §5.2.1 CMS EncapsulatedContentInfo wraps eContent in an OCTET STRING
                // regardless of eContentType (both DATA and non-DATA types such as szOID_CTL).
                let bytes = extract_octet_string(inner).unwrap_or_else(|| inner.val.to_vec());
                (bytes.clone(), bytes)
            } else {
                // Legacy PKCS#7 places structured non-DATA content (e.g. SpcIndirectDataContent
                // SEQUENCE) directly under [0]: CMSG_CONTENT_PARAM returns the full TLV while
                // messageDigest hashes its inner value bytes.
                (inner.raw.to_vec(), inner.val.to_vec())
            }
        } else {
            (ec.val.to_vec(), ec.val.to_vec())
        }
    } else {
        (Vec::new(), Vec::new())
    };

    let mut certs_der = Vec::new();
    let mut signers = Vec::new();
    for &elem in &sd_kids[3..] {
        if elem.tag == 0xA0 {
            if let Some(list) = ber_item_children(elem.val) {
                for ce in list {
                    if ce.tag == 0x30 {
                        certs_der.push(ce.raw.to_vec());
                    }
                }
            }
        } else if elem.tag == 0x31 {
            if let Some(si_list) = ber_item_children(elem.val) {
                for si in si_list {
                    if si.tag != 0x30 {
                        continue;
                    }
                    let Some(kids) = der_item_children(si.val) else {
                        continue;
                    };
                    if kids.len() < 5 || kids[0].tag != 0x02 {
                        continue;
                    }
                    let _si_ver = parse_der_u32_val(kids[0].val).unwrap_or(1);
                    let (issuer_raw, serial_be, subject_key_id) = if kids[1].tag == 0x30 {
                        let Some(ias) = der_item_children(kids[1].val) else {
                            continue;
                        };
                        if ias.len() < 2 || ias[0].tag != 0x30 || ias[1].tag != 0x02 {
                            continue;
                        }
                        (ias[0].raw.to_vec(), ias[1].val.to_vec(), None)
                    } else if kids[1].tag == 0x80 && !kids[1].val.is_empty() {
                        // RFC 5652 §5.3: SignerIdentifier ::= CHOICE {
                        //   issuerAndSerialNumber, subjectKeyIdentifier [0] IMPLICIT OCTET STRING }
                        (Vec::new(), Vec::new(), Some(kids[1].val.to_vec()))
                    } else {
                        continue;
                    };
                    let Some((hash_oid, _)) = parse_alg_id(kids[2]) else {
                        continue;
                    };
                    let mut pos = 3usize;
                    let auth_attrs_raw = if kids.get(pos).is_some_and(|e| e.tag == 0xA0) {
                        pos += 1;
                        Some(kids[pos - 1].raw.to_vec())
                    } else {
                        None
                    };
                    let Some(&enc_alg_elem) = kids.get(pos) else {
                        continue;
                    };
                    if parse_alg_id(enc_alg_elem).is_none() {
                        continue;
                    }
                    let Some(&sig_elem) = kids.get(pos + 1) else {
                        continue;
                    };
                    if sig_elem.tag != 0x04 {
                        continue;
                    }
                    let unauth_attrs_raw = if kids.get(pos + 2).is_some_and(|e| e.tag == 0xA1) {
                        Some(kids[pos + 2].raw.to_vec())
                    } else {
                        None
                    };
                    signers.push(CmsSigner {
                        signer_info_raw: si.raw.to_vec(),
                        issuer_raw,
                        serial_be,
                        subject_key_id,
                        hash_alg_oid: hash_oid.to_vec(),
                        auth_attrs_raw,
                        unauth_attrs_raw,
                        encrypted_hash: sig_elem.val.to_vec(),
                    });
                }
            }
        }
    }

    Some(CmsSignedMessage {
        msg_type: CMSG_SIGNED,
        version,
        raw_encoded: raw_encoded.to_vec(),
        inner_content_type_oid,
        inner_content_type_oid_der: inner_oid.to_vec(),
        bare_content: sd_seq.raw.to_vec(),
        digest_content,
        content,
        certs_der,
        signers,
    })
}

pub(super) fn parse_cms_content_info(der: &[u8]) -> Option<CmsSignedMessage> {
    let (ci, _) = parse_ber_item(der)?;
    if ci.tag != 0x30 {
        return None;
    }
    let ci_kids = ber_item_children(ci.val)?;
    if ci_kids.is_empty() || ci_kids[0].tag != 0x06 {
        return None;
    }
    let ctype = ci_kids[0].val;
    if ctype == OID_DATA {
        let (bare, data) = if let Some(c) = ci_kids.get(1).filter(|e| e.tag == 0xA0) {
            let (inner, _) = parse_ber_item(c.val)?;
            (inner.raw.to_vec(), extract_octet_string(inner)?)
        } else {
            (Vec::new(), Vec::new())
        };
        return Some(CmsSignedMessage {
            msg_type: CMSG_DATA,
            version: 1,
            raw_encoded: ci.raw.to_vec(),
            inner_content_type_oid: wintrust::decode_oid_str(OID_DATA),
            inner_content_type_oid_der: OID_DATA.to_vec(),
            bare_content: bare,
            digest_content: data.clone(),
            content: data,
            certs_der: Vec::new(),
            signers: Vec::new(),
        });
    }
    if ctype == OID_ENVELOPED_DATA {
        let bare = ci_kids
            .get(1)
            .filter(|e| e.tag == 0xA0)
            .map(|e| e.val.to_vec())
            .unwrap_or_default();
        return Some(CmsSignedMessage {
            msg_type: CMSG_ENVELOPED,
            version: 0,
            raw_encoded: ci.raw.to_vec(),
            inner_content_type_oid: wintrust::decode_oid_str(OID_ENVELOPED_DATA),
            inner_content_type_oid_der: OID_ENVELOPED_DATA.to_vec(),
            bare_content: bare,
            digest_content: Vec::new(),
            content: Vec::new(),
            certs_der: Vec::new(),
            signers: Vec::new(),
        });
    }
    if ctype != OID_SIGNED_DATA || ci_kids.len() < 2 || ci_kids[1].tag != 0xA0 {
        return None;
    }
    let (sd_seq, _) = parse_ber_item(ci_kids[1].val)?;
    parse_cms_signed_data(sd_seq, ci.raw)
}

/// Decode either a full PKCS#7/CMS `ContentInfo` or, when `expected_msg_type != 0`, a headerless
/// bare message of that type (`CMSG_SIGNED`, `CMSG_DATA`, `CMSG_ENVELOPED`) per MSDN
/// `CryptMsgOpenToDecode`.
fn parse_cms_message(der: &[u8], expected_msg_type: u32) -> Result<CmsSignedMessage, u32> {
    if let Some(parsed) = parse_cms_content_info(der) {
        if expected_msg_type != 0 && parsed.msg_type != expected_msg_type {
            return Err(CRYPT_E_INVALID_MSG_TYPE);
        }
        return Ok(parsed);
    }
    match expected_msg_type {
        CMSG_SIGNED => {
            let (sd_seq, _) = parse_ber_item(der).ok_or(CRYPT_E_ASN1_BADTAG)?;
            parse_cms_signed_data(sd_seq, sd_seq.raw).ok_or(CRYPT_E_ASN1_BADTAG)
        }
        CMSG_DATA => {
            let data = if let Some((item, _)) = parse_ber_item(der) {
                if item.tag == 0x04 || item.tag == 0x24 {
                    extract_octet_string(item).unwrap_or_else(|| der.to_vec())
                } else {
                    der.to_vec()
                }
            } else {
                der.to_vec()
            };
            Ok(CmsSignedMessage {
                msg_type: CMSG_DATA,
                version: 1,
                raw_encoded: der.to_vec(),
                inner_content_type_oid: wintrust::decode_oid_str(OID_DATA),
                inner_content_type_oid_der: OID_DATA.to_vec(),
                bare_content: der.to_vec(),
                digest_content: data.clone(),
                content: data,
                certs_der: Vec::new(),
                signers: Vec::new(),
            })
        }
        CMSG_ENVELOPED => {
            let (seq, _) = parse_der_item(der).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if seq.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            Ok(CmsSignedMessage {
                msg_type: CMSG_ENVELOPED,
                version: 0,
                raw_encoded: seq.raw.to_vec(),
                inner_content_type_oid: wintrust::decode_oid_str(OID_ENVELOPED_DATA),
                inner_content_type_oid_der: OID_ENVELOPED_DATA.to_vec(),
                bare_content: seq.raw.to_vec(),
                digest_content: Vec::new(),
                content: Vec::new(),
                certs_der: Vec::new(),
                signers: Vec::new(),
            })
        }
        _ => Err(CRYPT_E_ASN1_BADTAG),
    }
}

/// Extract issuer raw DER and serial value bytes from a DER X.509 certificate.
fn cert_issuer_and_serial(cert_der: &[u8]) -> Option<(&[u8], &[u8])> {
    let (seq, _) = parse_der_item(cert_der)?;
    let top = der_item_children(seq.val)?;
    let tbs = top.first()?;
    let kids = der_item_children(tbs.val)?;
    let mut idx = 0;
    if kids.get(idx)?.tag == 0xA0 {
        idx += 1;
    }
    let serial = *kids.get(idx)?;
    let issuer = *kids.get(idx + 2)?;
    (serial.tag == 0x02 && issuer.tag == 0x30).then_some((issuer.raw, serial.val))
}

fn cert_subject_key_id(cert_der: &[u8]) -> Option<Vec<u8>> {
    const OID_SKI: &[u8] = &[0x55, 0x1D, 0x0E];
    let parts = parse_cert_der_parts(cert_der)?;
    for &(oid, _, val) in &parts.extensions {
        if oid == OID_SKI {
            if let Some((oct, _)) = parse_der_item(val) {
                if oct.tag == 0x04 {
                    return Some(oct.val.to_vec());
                }
            }
        }
    }
    Some(Digest::Sha1.hash(parts.spki_key_bits))
}

fn verify_signed_bytes_with_cert_der(
    signer_cert_der: &[u8],
    si_digest: Digest,
    to_verify: &[u8],
    signature: &[u8],
) -> bool {
    if signature.is_empty() {
        return false;
    }
    let Some(parts) = parse_cert_der_parts(signer_cert_der) else {
        return false;
    };
    let Some((spki_alg_oid_item, _)) = parse_der_item(parts.spki_alg_inner) else {
        return false;
    };
    let spki_alg_oid = if spki_alg_oid_item.tag == 0x06 {
        spki_alg_oid_item.val
    } else {
        return false;
    };

    // 1. RSA PKCS#1 v1.5 verification
    if spki_alg_oid == OID_RSA_ENCRYPTION {
        let Some((pub_seq, _)) = parse_der_item(parts.spki_key_bits) else {
            return false;
        };
        if pub_seq.tag != 0x30 {
            return false;
        }
        let Some(pk) = der_item_children(pub_seq.val) else {
            return false;
        };
        if pk.len() < 2 || pk[0].tag != 0x02 || pk[1].tag != 0x02 {
            return false;
        }
        let Some(rsa) = RsaPublicKey::from_asn1_integers(pk[0].val, pk[1].val) else {
            return false;
        };
        let hash = si_digest.hash(to_verify);
        return rsa.verify_pkcs1(Some(si_digest), &hash, signature);
    }

    // 2. ECDSA / Ed25519 verification via rustls::crypto::ring verification algorithms
    let sig_alg_der = match (spki_alg_oid, si_digest) {
        // EC public key (1.2.840.10045.2.1)
        (&[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01], Digest::Sha256) => {
            Some(encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02]))
        }
        (&[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01], Digest::Sha384) => {
            Some(encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x03]))
        }
        (&[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01], Digest::Sha512) => {
            Some(encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x04]))
        }
        // Ed25519 (1.3.101.112)
        (&[0x2B, 0x65, 0x70], _) => Some(encode_der_tlv(0x06, &[0x2B, 0x65, 0x70])),
        _ => None,
    };
    if let Some(sig_alg_inner) = sig_alg_der {
        let provider = crate::net_tls::crypto_provider();
        for alg in provider.signature_verification_algorithms.all {
            if alg.public_key_alg_id().as_ref() == parts.spki_alg_inner
                && alg.signature_alg_id().as_ref() == sig_alg_inner.as_slice()
                && alg
                    .verify_signature(parts.spki_key_bits, to_verify, signature)
                    .is_ok()
            {
                return true;
            }
        }
    }
    false
}

/// Cryptographically verify a Win32 `CMSG_SIGNER_INFO` (used by `WinVerifyTrust(WTD_CHOICE_SIGNER)`)
/// against its matched signer certificate DER. Requires a non-empty `EncryptedHash`, recognized
/// `HashAlgorithm`, and `AuthAttrs` whose DER `SET OF Attribute` verifies under the certificate key.
pub(crate) unsafe fn verify_cmsg_signer_info_signature(
    si: &wintrust::CmsgSignerInfo,
    signer_cert_der: &[u8],
) -> bool {
    if si.encrypted_hash.pb_data.is_null()
        || si.encrypted_hash.cb_data == 0
        || si.hash_algorithm.psz_obj_id.is_null()
        || si.auth_attrs_count == 0
        || si.auth_attrs_ptr.is_null()
    {
        return false;
    }
    let digest = match crate::strings::from_ansi(si.hash_algorithm.psz_obj_id).as_str() {
        "1.2.840.113549.2.5" => Digest::Md5,
        "1.3.14.3.2.26" => Digest::Sha1,
        "2.16.840.1.101.3.4.2.1" => Digest::Sha256,
        "2.16.840.1.101.3.4.2.2" => Digest::Sha384,
        "2.16.840.1.101.3.4.2.3" => Digest::Sha512,
        _ => return false,
    };
    let sig = std::slice::from_raw_parts(
        si.encrypted_hash.pb_data,
        si.encrypted_hash.cb_data as usize,
    );
    #[repr(C)]
    struct CryptAttrs {
        c_attr: u32,
        rg_attr: *mut c_void,
    }
    let attrs = CryptAttrs {
        c_attr: si.auth_attrs_count,
        rg_attr: si.auth_attrs_ptr,
    };
    let mut enc_len = 0u32;
    if super::asn1::crypt_encode_object_ex_impl(
        X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
        super::decode::PKCS_ATTRIBUTES as *const u8,
        &attrs as *const CryptAttrs as *const c_void,
        0,
        std::ptr::null_mut(),
        &mut enc_len,
    ) == FALSE
        || enc_len == 0
    {
        return false;
    }
    let mut set_der = vec![0u8; enc_len as usize];
    if super::asn1::crypt_encode_object_ex_impl(
        X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
        super::decode::PKCS_ATTRIBUTES as *const u8,
        &attrs as *const CryptAttrs as *const c_void,
        0,
        set_der.as_mut_ptr() as *mut c_void,
        &mut enc_len,
    ) == FALSE
    {
        return false;
    }
    set_der.truncate(enc_len as usize);
    if verify_signed_bytes_with_cert_der(signer_cert_der, digest, &set_der, sig) {
        return true;
    }
    // RFC 5652 §5.4 / X.690 §11.6: retry with lexicographically DER-sorted SET OF attributes
    // in case caller supplied AuthAttrs in unsorted order.
    if let Some((set_item, _)) = parse_der_item(&set_der) {
        if set_item.tag == 0x31 {
            if let Some(kids) = der_item_children(set_item.val) {
                if kids.len() > 1 {
                    let mut sorted: Vec<&[u8]> = kids.iter().map(|k| k.raw).collect();
                    sorted.sort();
                    let mut inner = Vec::with_capacity(set_item.val.len());
                    for k in sorted {
                        inner.extend_from_slice(k);
                    }
                    let canonical_der = encode_der_tlv(0x31, &inner);
                    if canonical_der != set_der
                        && verify_signed_bytes_with_cert_der(
                            signer_cert_der,
                            digest,
                            &canonical_der,
                            sig,
                        )
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Verify a CMS `SignerInfo` signature against `signer_cert_der`, `content_bytes`, and
/// `expected_content_type_oid_der` (enforcing RFC 5652 §5.6 signed `content-type` + `message-digest`).
pub(super) fn verify_cms_signer_signature(
    signer: &CmsSigner,
    signer_cert_der: &[u8],
    content_bytes: &[u8],
    expected_content_type_oid_der: &[u8],
) -> bool {
    let Some(si_digest) = digest_from_oid(&signer.hash_alg_oid) else {
        return false;
    };

    // Determine the bytes that are actually signed (SET-retagged signedAttrs or raw content).
    let to_verify: Vec<u8> = if let Some(ref aa_raw) = signer.auth_attrs_raw {
        // Per RFC 5652 §5.3 & §5.6: signedAttrs MUST include exactly one content-type attribute
        // matching encapContentInfo.eContentType and exactly one message-digest matching hash(content).
        let Some((aa, _)) = parse_der_item(aa_raw) else {
            return false;
        };
        let Some(attrs) = der_item_children(aa.val) else {
            return false;
        };
        let mut md_count = 0usize;
        let mut ct_count = 0usize;
        let expected_md = si_digest.hash(content_bytes);
        for attr in attrs {
            if attr.tag != 0x30 {
                continue;
            }
            let Some(ak) = der_item_children(attr.val) else {
                return false;
            };
            if ak.len() >= 2 && ak[0].tag == 0x06 && ak[1].tag == 0x31 {
                if ak[0].val == OID_MESSAGE_DIGEST {
                    let Some(vals) = der_item_children(ak[1].val) else {
                        return false;
                    };
                    if vals.len() != 1 || vals[0].tag != 0x04 || vals[0].val != expected_md.as_slice() {
                        return false;
                    }
                    md_count += 1;
                } else if ak[0].val == OID_CONTENT_TYPE {
                    let Some(vals) = der_item_children(ak[1].val) else {
                        return false;
                    };
                    if vals.len() != 1 || vals[0].tag != 0x06 || vals[0].val != expected_content_type_oid_der {
                        return false;
                    }
                    ct_count += 1;
                }
            }
        }
        if md_count != 1 || ct_count != 1 {
            return false;
        }
        let mut set_der = Vec::with_capacity(aa_raw.len());
        set_der.push(0x31); // EXPLICIT SET tag per RFC 5652 §5.4
        set_der.extend_from_slice(&aa_raw[1..]);
        set_der
    } else {
        // Per RFC 5652 §5.3, signedAttrs MUST be present whenever eContentType != id-data.
        if expected_content_type_oid_der != OID_DATA {
            return false;
        }
        content_bytes.to_vec()
    };

    verify_signed_bytes_with_cert_der(signer_cert_der, si_digest, &to_verify, &signer.encrypted_hash)
}

// ---------------------------------------------------------------------------
// CryptMsgOpenToDecode / CryptMsgUpdate / CryptMsgClose / CryptMsgGetParam
// ---------------------------------------------------------------------------

struct DecodingMsg {
    flags: u32,
    expected_msg_type: u32,
    stream_output: Option<(usize, usize)>,
    buf: Vec<u8>,
    detached_data: Vec<u8>,
    header_done: bool,
    parsed: Option<CmsSignedMessage>,
}

static DECODE_MSGS: HandleTable<DecodingMsg> = HandleTable::new(0x5500_0001, 1);

pub(super) unsafe fn crypt_msg_open_to_decode_impl(
    dw_msg_encoding_type: u32,
    dw_flags: u32,
    dw_msg_type: u32,
    _h_crypt_prov: usize,
    _p_recipient_info: *mut c_void,
    p_stream_info: *mut c_void,
) -> *mut c_void {
    if (dw_msg_encoding_type & (X509_ASN_ENCODING | PKCS_7_ASN_ENCODING)) == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let stream_output = if !p_stream_info.is_null() {
        // CMSG_STREAM_INFO (Win64): cbContent @ +0x00, pfnStreamOutput @ +0x08, pvArg @ +0x10
        let p = p_stream_info as *const u8;
        let pfn = std::ptr::read_unaligned(p.add(0x08) as *const usize);
        let pv_arg = std::ptr::read_unaligned(p.add(0x10) as *const usize);
        if pfn == 0 {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return std::ptr::null_mut();
        }
        Some((pfn, pv_arg))
    } else {
        None
    };
    let id = DECODE_MSGS.insert(DecodingMsg {
        flags: dw_flags,
        expected_msg_type: dw_msg_type,
        stream_output,
        buf: Vec::new(),
        detached_data: Vec::new(),
        header_done: false,
        parsed: None,
    });
    id as *mut c_void
}

pub(super) unsafe fn crypt_msg_update_impl(
    h_crypt_msg: *mut c_void,
    pb_data: *const u8,
    cb_data: u32,
    f_final: BOOL,
) -> BOOL {
    if h_crypt_msg.is_null() || (cb_data > 0 && pb_data.is_null()) {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let chunk = if cb_data == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(pb_data, cb_data as usize)
    };
    let res = DECODE_MSGS.with_mut(h_crypt_msg as usize, |m| {
        if (m.flags & CMSG_DETACHED_FLAG) != 0 && m.header_done {
            m.detached_data.extend_from_slice(chunk);
            if f_final != 0 {
                if let Some(ref mut p) = m.parsed {
                    p.content = m.detached_data.clone();
                    p.digest_content = m.detached_data.clone();
                }
                let deliver = m.stream_output.map(|cb| (cb, m.detached_data.clone()));
                return Ok(deliver);
            }
            return Ok(None);
        }
        m.buf.extend_from_slice(chunk);
        if f_final != 0 {
            let parsed = parse_cms_message(&m.buf, m.expected_msg_type)?;
            let deliver = if (m.flags & CMSG_DETACHED_FLAG) == 0 {
                m.stream_output.map(|cb| (cb, parsed.content.clone()))
            } else {
                None
            };
            m.parsed = Some(parsed);
            m.header_done = true;
            return Ok(deliver);
        }
        Ok(None)
    });
    match res {
        Some(Ok(Some(((pfn, pv_arg), content)))) => {
            let ok = crate::call_guest8(
                pfn,
                pv_arg as u64,
                content.as_ptr() as u64,
                content.len() as u64,
                TRUE as u64,
                0,
                0,
                0,
                0,
            ) as i32;
            if ok == FALSE {
                FALSE
            } else {
                TRUE
            }
        }
        Some(Ok(None)) => TRUE,
        Some(Err(code)) => {
            crate::teb::set_last_error(code);
            FALSE
        }
        None => {
            crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
            FALSE
        }
    }
}

pub(super) fn try_close_decode_msg(h_msg: usize) -> bool {
    DECODE_MSGS.remove(h_msg).is_some()
}

pub(super) unsafe fn try_get_decode_msg_param(
    h_crypt_msg: usize,
    dw_param_type: u32,
    dw_index: u32,
    pv_data: *mut c_void,
    pcb_data: *mut u32,
) -> Option<BOOL> {
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
        if !src.is_empty() {
            std::ptr::copy_nonoverlapping(src.as_ptr(), pv_data as *mut u8, src.len());
        }
        std::ptr::write_unaligned(pcb_data, need);
        TRUE
    };

    DECODE_MSGS.with(h_crypt_msg, |m| {
        let Some(ref msg) = m.parsed else {
            crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
            return FALSE;
        };
        match dw_param_type {
            CMSG_TYPE_PARAM => write_bytes(&msg.msg_type.to_le_bytes()),
            CMSG_VERSION_PARAM => write_bytes(&msg.version.to_le_bytes()),
            CMSG_CONTENT_PARAM => write_bytes(&msg.content),
            CMSG_BARE_CONTENT_PARAM => write_bytes(&msg.bare_content),
            CMSG_INNER_CONTENT_TYPE_PARAM => {
                let mut v = msg.inner_content_type_oid.as_bytes().to_vec();
                v.push(0);
                write_bytes(&v)
            }
            CMSG_ENCODED_MESSAGE => write_bytes(&msg.raw_encoded),
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
                // Decode the full SignerInfo DER (including AuthAttrs and UnauthAttrs with
                // relocated backing storage) via PKCS7_SIGNER_INFO.
                super::decode::crypt_decode_object_ex_impl(
                    X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
                    super::decode::PKCS7_SIGNER_INFO as *const u8,
                    signer.signer_info_raw.as_ptr(),
                    signer.signer_info_raw.len() as u32,
                    0,
                    std::ptr::null(),
                    pv_data,
                    pcb_data,
                )
            }
            CMSG_SIGNER_CERT_INFO_PARAM => {
                let Some(signer) = msg.signers.get(dw_index as usize) else {
                    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                    return FALSE;
                };
                // CERT_INFO with Issuer + SerialNumber populated
                let hdr_sz = std::mem::size_of::<wintrust::CertInfo>();
                let mut serial_le = signer.serial_be.clone();
                serial_le.reverse();
                let need = (hdr_sz + signer.issuer_raw.len() + serial_le.len()) as u32;
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
                std::ptr::write_bytes(base, 0, hdr_sz);
                let iss_ptr = base.add(hdr_sz);
                std::ptr::copy_nonoverlapping(
                    signer.issuer_raw.as_ptr(),
                    iss_ptr,
                    signer.issuer_raw.len(),
                );
                let ser_ptr = iss_ptr.add(signer.issuer_raw.len());
                std::ptr::copy_nonoverlapping(serial_le.as_ptr(), ser_ptr, serial_le.len());
                let ci = base as *mut wintrust::CertInfo;
                std::ptr::write_unaligned(
                    std::ptr::addr_of_mut!((*ci).issuer),
                    CryptDataBlob {
                        cb_data: signer.issuer_raw.len() as u32,
                        pb_data: iss_ptr,
                    },
                );
                std::ptr::write_unaligned(
                    std::ptr::addr_of_mut!((*ci).serial_number),
                    CryptDataBlob {
                        cb_data: serial_le.len() as u32,
                        pb_data: ser_ptr,
                    },
                );
                std::ptr::write_unaligned(pcb_data, need);
                TRUE
            }
            CMSG_SIGNER_AUTH_ATTR_PARAM | CMSG_SIGNER_UNAUTH_ATTR_PARAM => {
                let Some(signer) = msg.signers.get(dw_index as usize) else {
                    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                    return FALSE;
                };
                let raw_opt = if dw_param_type == CMSG_SIGNER_AUTH_ATTR_PARAM {
                    signer.auth_attrs_raw.as_deref()
                } else {
                    signer.unauth_attrs_raw.as_deref()
                };
                let Some(raw) = raw_opt else {
                    crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                    return FALSE;
                };
                super::decode::crypt_decode_object_ex_impl(
                    X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
                    super::decode::PKCS_ATTRIBUTES as *const u8,
                    raw.as_ptr(),
                    raw.len() as u32,
                    0,
                    std::ptr::null(),
                    pv_data,
                    pcb_data,
                )
            }
            _ => {
                crate::teb::set_last_error(CRYPT_E_NOT_FOUND);
                FALSE
            }
        }
    })
}

// ---------------------------------------------------------------------------
// CryptVerifyMessageSignature
// ---------------------------------------------------------------------------

#[repr(C)]
struct CryptVerifyMessagePara {
    cb_size: u32,
    dw_msg_and_cert_encoding_type: u32,
    h_crypt_prov: usize,
    pfn_get_signer_certificate: usize,
    pv_get_arg: *mut c_void,
}

pub(super) unsafe fn crypt_verify_message_signature_impl(
    p_verify_para: *const c_void,
    dw_signer_index: u32,
    pb_signed_blob: *const u8,
    cb_signed_blob: u32,
    pb_decoded: *mut u8,
    pcb_decoded: *mut u32,
    pp_signer_cert: *mut *mut c_void,
) -> BOOL {
    if !pp_signer_cert.is_null() {
        std::ptr::write_unaligned(pp_signer_cert, std::ptr::null_mut());
    }
    if p_verify_para.is_null() || pb_signed_blob.is_null() || cb_signed_blob == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let para = std::ptr::read_unaligned(p_verify_para as *const CryptVerifyMessagePara);
    if para.cb_size < 0x20
        || (para.dw_msg_and_cert_encoding_type & (X509_ASN_ENCODING | PKCS_7_ASN_ENCODING)) == 0
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let blob = std::slice::from_raw_parts(pb_signed_blob, cb_signed_blob as usize);
    let Some(msg) = parse_cms_content_info(blob) else {
        crate::teb::set_last_error(CRYPT_E_ASN1_BADTAG);
        return FALSE;
    };
    if msg.msg_type != CMSG_SIGNED {
        crate::teb::set_last_error(CRYPT_E_INVALID_MSG_TYPE);
        return FALSE;
    }
    let Some(signer) = msg.signers.get(dw_signer_index as usize) else {
        crate::teb::set_last_error(CRYPT_E_SIGNER_NOT_FOUND);
        return FALSE;
    };

    // Per MSDN CRYPT_VERIFY_MESSAGE_PARA: when pfnGetSignerCertificate is supplied, invoke the
    // callback with (pvGetArg, dwCertEncodingType, pSignerId, hMsgCertStore) and use its returned
    // PCCERT_CONTEXT exclusively (if it returns NULL, reject the signer rather than falling back).
    let (signer_der_owned, cb_cert_ctx): (Vec<u8>, *mut c_void) =
        if para.pfn_get_signer_certificate != 0 {
            let msg_store = super::store::cert_open_store_impl(
                super::store::CERT_STORE_PROV_MEMORY as *const u8,
                std::ptr::null(),
            );
            if !msg_store.is_null() {
                for der in &msg.certs_der {
                    let _ = super::store::cert_add_encoded_certificate_to_store_impl(
                        msg_store,
                        X509_ASN_ENCODING,
                        der.as_ptr(),
                        der.len() as u32,
                        4,
                        std::ptr::null_mut(),
                    );
                }
            }
            let mut ser_le = signer.serial_be.clone();
            ser_le.reverse();
            let mut signer_id: crate::misc::wintrust::CertInfo = std::mem::zeroed();
            signer_id.issuer = CryptDataBlob {
                cb_data: signer.issuer_raw.len() as u32,
                pb_data: signer.issuer_raw.as_ptr(),
            };
            signer_id.serial_number = CryptDataBlob {
                cb_data: ser_le.len() as u32,
                pb_data: ser_le.as_ptr(),
            };
            let ctx = crate::call_guest8(
                para.pfn_get_signer_certificate,
                para.pv_get_arg as u64,
                para.dw_msg_and_cert_encoding_type as u64,
                &signer_id as *const _ as u64,
                msg_store as u64,
                0,
                0,
                0,
                0,
            ) as *mut c_void;
            if !msg_store.is_null() {
                let _ = super::store::cert_close_store_impl(msg_store);
            }
            if ctx.is_null() {
                crate::teb::set_last_error(CRYPT_E_SIGNER_NOT_FOUND);
                return FALSE;
            }
            let cc = std::ptr::read_unaligned(ctx as *const crate::misc::wintrust::CertContext);
            if cc.pb_cert_encoded.is_null() || cc.cb_cert_encoded == 0 {
                let _ = super::store::cert_free_certificate_context_impl(ctx);
                crate::teb::set_last_error(CRYPT_E_SIGNER_NOT_FOUND);
                return FALSE;
            }
            let der =
                std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize).to_vec();
            (der, ctx)
        } else {
            // Default lookup: match signer certificate in embedded certificates by
            // SubjectKeyIdentifier (RFC 5652 v3 sid) or IssuerAndSerialNumber (v1 sid).
            let signer_cert_der = if let Some(ref want_ski) = signer.subject_key_id {
                msg.certs_der
                    .iter()
                    .find(|der| cert_subject_key_id(der).as_deref() == Some(want_ski.as_slice()))
            } else {
                msg.certs_der.iter().find(|der| {
                    !signer.issuer_raw.is_empty()
                        && cert_issuer_and_serial(der).is_some_and(|(iss, ser)| {
                            iss == signer.issuer_raw.as_slice()
                                && strip_leading_zero(ser) == strip_leading_zero(&signer.serial_be)
                        })
                })
            };
            let Some(signer_der) = signer_cert_der else {
                crate::teb::set_last_error(CRYPT_E_SIGNER_NOT_FOUND);
                return FALSE;
            };
            (signer_der.clone(), std::ptr::null_mut())
        };

    if !verify_cms_signer_signature(
        signer,
        &signer_der_owned,
        &msg.digest_content,
        &msg.inner_content_type_oid_der,
    ) {
        if !cb_cert_ctx.is_null() {
            let _ = super::store::cert_free_certificate_context_impl(cb_cert_ctx);
        }
        crate::teb::set_last_error(NTE_BAD_SIGNATURE);
        return FALSE;
    }

    // Write decoded content if requested.
    if !pcb_decoded.is_null() {
        let need = msg.content.len() as u32;
        if pb_decoded.is_null() {
            std::ptr::write_unaligned(pcb_decoded, need);
        } else {
            let cap = std::ptr::read_unaligned(pcb_decoded);
            if cap < need {
                std::ptr::write_unaligned(pcb_decoded, need);
                if !cb_cert_ctx.is_null() {
                    let _ = super::store::cert_free_certificate_context_impl(cb_cert_ctx);
                }
                crate::teb::set_last_error(err::ERROR_MORE_DATA);
                return FALSE;
            }
            if !msg.content.is_empty() {
                std::ptr::copy_nonoverlapping(msg.content.as_ptr(), pb_decoded, msg.content.len());
            }
            std::ptr::write_unaligned(pcb_decoded, need);
        }
    }

    if !pp_signer_cert.is_null() {
        let ctx = if !cb_cert_ctx.is_null() {
            cb_cert_ctx
        } else {
            create_cert_context_from_der(&signer_der_owned)
        };
        std::ptr::write_unaligned(pp_signer_cert, ctx);
    } else if !cb_cert_ctx.is_null() {
        let _ = super::store::cert_free_certificate_context_impl(cb_cert_ctx);
    }
    TRUE
}

// ---------------------------------------------------------------------------
// RFC 7292 PKCS#12 / PFXImportCertStore (CommonCrypto PBES1 + PBES2 + HMAC)
// ---------------------------------------------------------------------------

/// Upper bound on a single PFX KDF iteration count across MacData, PBES1, and PBES2.
/// Comfortably above OWASP PBKDF2 recommendations (600,000) while rejecting
/// attacker-crafted 32-bit counts (e.g. `0xffffffff`) before entering any KDF loop.
pub(super) const MAX_PFX_KDF_ITERATIONS: u32 = 2_000_000;

/// Upper bound on cumulative KDF work (total hash/PRF block iterations) across an entire
/// `PFXImportCertStore` import, covering `MacData`, encrypted `SafeContents`, and
/// `pkcs8ShroudedKeyBag` entries (including PBES1 key+IV derivations and multi-block KDF outputs).
pub(super) const MAX_PFX_TOTAL_KDF_ITERATIONS: u64 = 4_000_000;

#[inline]
fn consume_kdf_budget(kdf_budget: &mut u64, work: u64) -> Result<(), u32> {
    if work == 0 || work > *kdf_budget {
        return Err(err::ERROR_INVALID_DATA);
    }
    *kdf_budget -= work;
    Ok(())
}

/// RFC 7292 Appendix B.2 PKCS#12 key/IV/MAC derivation.
fn pkcs12_kdf(
    digest: Digest,
    pass_bmp: &[u8],
    salt: &[u8],
    iters: u32,
    id: u8,
    out_len: usize,
    kdf_budget: &mut u64,
) -> Result<Vec<u8>, u32> {
    if iters == 0 || iters > MAX_PFX_KDF_ITERATIONS || out_len == 0 {
        return Err(err::ERROR_INVALID_DATA);
    }
    let v = match digest {
        Digest::Sha384 | Digest::Sha512 => 128usize,
        _ => 64usize,
    };
    let u = digest.len();
    let c = (out_len + u - 1) / u;
    let work = (iters as u64).saturating_mul(c as u64);
    consume_kdf_budget(kdf_budget, work)?;
    let d = vec![id; v];

    let repeat_pad = |src: &[u8]| -> Vec<u8> {
        if src.is_empty() {
            return Vec::new();
        }
        let target = v * ((src.len() + v - 1) / v);
        let mut out = Vec::with_capacity(target);
        while out.len() < target {
            let take = (target - out.len()).min(src.len());
            out.extend_from_slice(&src[..take]);
        }
        out
    };

    let s_buf = repeat_pad(salt);
    let p_buf = repeat_pad(pass_bmp);
    let mut i_buf = Vec::with_capacity(s_buf.len() + p_buf.len());
    i_buf.extend_from_slice(&s_buf);
    i_buf.extend_from_slice(&p_buf);

    let mut derived = Vec::with_capacity(out_len);
    for block_idx in 0..c {
        let mut a = digest.hash_slices(&[&d, &i_buf]);
        for _ in 1..iters {
            a = digest.hash(&a);
        }
        derived.extend_from_slice(&a);
        if block_idx + 1 == c {
            break;
        }
        // B = concatenate copies of A to length v, then I_j = (I_j + B + 1) mod 2^(8*v)
        let mut b = Vec::with_capacity(v);
        while b.len() < v {
            let take = (v - b.len()).min(a.len());
            b.extend_from_slice(&a[..take]);
        }
        for chunk in i_buf.chunks_exact_mut(v) {
            let mut carry: u16 = 1;
            for k in (0..v).rev() {
                let sum = chunk[k] as u16 + b[k] as u16 + carry;
                chunk[k] = (sum & 0xFF) as u8;
                carry = sum >> 8;
            }
        }
    }
    derived.truncate(out_len);
    Ok(derived)
}

fn password_to_bmp_string(pass: Option<&str>) -> Vec<u8> {
    match pass {
        None => Vec::new(), // NULL password pointer -> 0 bytes per Windows / RFC 7292
        Some(s) => {
            let mut out = Vec::with_capacity((s.encode_utf16().count() + 1) * 2);
            for u in s.encode_utf16() {
                out.extend_from_slice(&u.to_be_bytes());
            }
            out.extend_from_slice(&[0, 0]);
            out
        }
    }
}

unsafe fn cc_decrypt_cbc(alg: u32, key: &[u8], iv: &[u8], ct: &[u8]) -> Option<Vec<u8>> {
    // CommonCrypto `CCCrypt` takes no IV-length parameter and always reads a full cipher block
    // from `iv`. Validate both IV and key lengths before crossing FFI to prevent over-reads.
    let valid_params = match alg {
        K_CC_ALGORITHM_AES => iv.len() == 16 && matches!(key.len(), 16 | 24 | 32),
        K_CC_ALGORITHM_3DES => iv.len() == 8 && key.len() == 24,
        K_CC_ALGORITHM_RC2 => iv.len() == 8 && (1..=128).contains(&key.len()),
        _ => false,
    };
    if !valid_params {
        return None;
    }
    if ct.is_empty() {
        return Some(Vec::new());
    }
    let mut out = vec![0u8; ct.len() + 32];
    let mut moved = 0usize;
    let st = CCCrypt(
        K_CC_DECRYPT,
        alg,
        K_CC_OPTION_PKCS7_PADDING,
        key.as_ptr() as *const c_void,
        key.len(),
        iv.as_ptr() as *const c_void,
        ct.as_ptr() as *const c_void,
        ct.len(),
        out.as_mut_ptr() as *mut c_void,
        out.len(),
        &mut moved,
    );
    if st != 0 {
        return None;
    }
    out.truncate(moved);
    Some(out)
}

unsafe fn decrypt_pbe(
    alg_elem: DerItem<'_>,
    ciphertext: &[u8],
    password_utf8: &str,
    pass_bmp: &[u8],
    kdf_budget: &mut u64,
) -> Result<Vec<u8>, u32> {
    let (alg_oid, params_opt) = parse_alg_id(alg_elem).ok_or(err::ERROR_INVALID_DATA)?;
    let params = params_opt.ok_or(err::ERROR_INVALID_DATA)?;

    // PBES1 (PKCS#12 PBE with SHA-1)
    if matches!(
        alg_oid,
        OID_PBE_SHA1_3DES_CBC | OID_PBE_SHA1_RC2_128_CBC | OID_PBE_SHA1_RC2_40_CBC
    ) {
        if params.tag != 0x30 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let pk = der_item_children(params.val).ok_or(err::ERROR_INVALID_DATA)?;
        if pk.len() < 2 || pk[0].tag != 0x04 || pk[1].tag != 0x02 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let salt = pk[0].val;
        let iters = parse_der_u32_val(pk[1].val).ok_or(err::ERROR_INVALID_DATA)?;
        if iters == 0 || iters > MAX_PFX_KDF_ITERATIONS {
            return Err(err::ERROR_INVALID_DATA);
        }
        let (cc_alg, key_len) = match alg_oid {
            OID_PBE_SHA1_3DES_CBC => (K_CC_ALGORITHM_3DES, 24usize),
            OID_PBE_SHA1_RC2_128_CBC => (K_CC_ALGORITHM_RC2, 16usize),
            OID_PBE_SHA1_RC2_40_CBC => (K_CC_ALGORITHM_RC2, 5usize),
            _ => unreachable!(),
        };
        // Check combined key + IV KDF work upfront before running either derivation.
        let sha1_len = Digest::Sha1.len();
        let key_blocks = (key_len + sha1_len - 1) / sha1_len;
        let iv_blocks = (8 + sha1_len - 1) / sha1_len;
        let combined_work = (iters as u64).saturating_mul((key_blocks + iv_blocks) as u64);
        if combined_work == 0 || combined_work > *kdf_budget {
            return Err(err::ERROR_INVALID_DATA);
        }
        let key = pkcs12_kdf(Digest::Sha1, pass_bmp, salt, iters, 1, key_len, kdf_budget)?;
        let iv = pkcs12_kdf(Digest::Sha1, pass_bmp, salt, iters, 2, 8, kdf_budget)?;
        return cc_decrypt_cbc(cc_alg, &key, &iv, ciphertext).ok_or(ERROR_INVALID_PASSWORD);
    }

    // PBES2 (RFC 8018)
    if alg_oid == OID_PBES2 {
        if params.tag != 0x30 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let pk = der_item_children(params.val).ok_or(err::ERROR_INVALID_DATA)?;
        if pk.len() < 2 {
            return Err(err::ERROR_INVALID_DATA);
        }
        // keyDerivationFunc (PBKDF2)
        let (kdf_oid, kdf_params) = parse_alg_id(pk[0]).ok_or(err::ERROR_INVALID_DATA)?;
        if kdf_oid != OID_PBKDF2 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let kp_elem = kdf_params.ok_or(err::ERROR_INVALID_DATA)?;
        if kp_elem.tag != 0x30 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let kp = der_item_children(kp_elem.val).ok_or(err::ERROR_INVALID_DATA)?;
        if kp.len() < 2 || kp[0].tag != 0x04 || kp[1].tag != 0x02 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let salt = kp[0].val;
        let iters = parse_der_u32_val(kp[1].val).ok_or(err::ERROR_INVALID_DATA)?;
        let mut explicit_key_len: Option<usize> = None;
        let mut prf_cc = K_CC_PRF_HMAC_SHA1;
        let mut prf_len = 20usize;
        for extra in kp.iter().skip(2) {
            if extra.tag == 0x02 {
                let len = parse_der_u32_val(extra.val).ok_or(err::ERROR_INVALID_DATA)?;
                explicit_key_len = Some(len as usize);
            } else if extra.tag == 0x30 {
                let (prf_oid, _) = parse_alg_id(*extra).ok_or(err::ERROR_INVALID_DATA)?;
                (prf_cc, prf_len) = match prf_oid {
                    OID_HMAC_SHA1 => (K_CC_PRF_HMAC_SHA1, 20),
                    OID_HMAC_SHA256 => (K_CC_PRF_HMAC_SHA256, 32),
                    OID_HMAC_SHA384 => (K_CC_PRF_HMAC_SHA384, 48),
                    OID_HMAC_SHA512 => (K_CC_PRF_HMAC_SHA512, 64),
                    _ => return Err(err::ERROR_INVALID_DATA),
                };
            }
        }

        // encryptionScheme
        let (enc_oid, enc_params) = parse_alg_id(pk[1]).ok_or(err::ERROR_INVALID_DATA)?;
        let iv_elem = enc_params.ok_or(err::ERROR_INVALID_DATA)?;
        if iv_elem.tag != 0x04 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let iv = iv_elem.val;
        let (cc_alg, default_key_len, expected_iv_len) = match enc_oid {
            OID_AES_128_CBC => (K_CC_ALGORITHM_AES, 16usize, 16usize),
            OID_AES_192_CBC => (K_CC_ALGORITHM_AES, 24usize, 16usize),
            OID_AES_256_CBC => (K_CC_ALGORITHM_AES, 32usize, 16usize),
            OID_3DES_CBC => (K_CC_ALGORITHM_3DES, 24usize, 8usize),
            _ => return Err(err::ERROR_INVALID_DATA),
        };
        if iv.len() != expected_iv_len {
            return Err(err::ERROR_INVALID_DATA);
        }
        let key_len = explicit_key_len.unwrap_or(default_key_len);
        if key_len != default_key_len || iters == 0 || iters > MAX_PFX_KDF_ITERATIONS {
            return Err(err::ERROR_INVALID_DATA);
        }
        let blocks = (key_len + prf_len - 1) / prf_len;
        let work = (iters as u64).saturating_mul(blocks as u64);
        consume_kdf_budget(kdf_budget, work)?;

        let mut key = vec![0u8; key_len];
        let pw_bytes = password_utf8.as_bytes();
        let st = CCKeyDerivationPBKDF(
            K_CC_PBKDF2,
            pw_bytes.as_ptr(),
            pw_bytes.len(),
            salt.as_ptr(),
            salt.len(),
            prf_cc,
            iters,
            key.as_mut_ptr(),
            key.len(),
        );
        if st != 0 {
            return Err(err::ERROR_INVALID_DATA);
        }
        return cc_decrypt_cbc(cc_alg, &key, iv, ciphertext).ok_or(ERROR_INVALID_PASSWORD);
    }

    Err(err::ERROR_INVALID_DATA)
}

struct PfxBagCert {
    der: Vec<u8>,
    friendly_name: Option<String>,
    local_key_id: Option<Vec<u8>>,
}

struct PfxBagKey {
    pkcs8_der: Vec<u8>,
    local_key_id: Option<Vec<u8>>,
}

fn parse_bag_attributes(attrs_elem: Option<DerItem<'_>>) -> (Option<String>, Option<Vec<u8>>) {
    let mut friendly = None;
    let mut local_key_id = None;
    let Some(ae) = attrs_elem else {
        return (None, None);
    };
    if ae.tag != 0x31 {
        return (None, None);
    }
    let Some(attrs) = der_item_children(ae.val) else {
        return (None, None);
    };
    for attr in attrs {
        if attr.tag != 0x30 {
            continue;
        }
        let Some(ak) = der_item_children(attr.val) else {
            continue;
        };
        if ak.len() < 2 || ak[0].tag != 0x06 || ak[1].tag != 0x31 {
            continue;
        }
        let Some(vals) = der_item_children(ak[1].val) else {
            continue;
        };
        let Some(first) = vals.first() else {
            continue;
        };
        if ak[0].val == OID_FRIENDLY_NAME {
            if first.tag == 0x1E && first.val.len() % 2 == 0 {
                let units: Vec<u16> = first
                    .val
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect();
                friendly = Some(String::from_utf16_lossy(&units));
            } else if matches!(first.tag, 0x0C | 0x13 | 0x16) {
                friendly = Some(String::from_utf8_lossy(first.val).into_owned());
            }
        } else if ak[0].val == OID_LOCAL_KEY_ID && first.tag == 0x04 {
            local_key_id = Some(first.val.to_vec());
        }
    }
    (friendly, local_key_id)
}

unsafe fn parse_safe_contents(
    der: &[u8],
    password_utf8: &str,
    pass_bmp: &[u8],
    certs: &mut Vec<PfxBagCert>,
    keys: &mut Vec<PfxBagKey>,
    bag_count: &mut usize,
    kdf_budget: &mut u64,
) -> Result<(), u32> {
    let mut worklist: Vec<(&[u8], usize)> = vec![(der, 0)];
    while let Some((cur_der, depth)) = worklist.pop() {
        if depth > MAX_PFX_NESTING_DEPTH {
            return Err(err::ERROR_INVALID_DATA);
        }
        let (seq, _) = parse_der_item(cur_der).ok_or(err::ERROR_INVALID_DATA)?;
        if seq.tag != 0x30 {
            return Err(err::ERROR_INVALID_DATA);
        }
        let bags = der_item_children(seq.val).ok_or(err::ERROR_INVALID_DATA)?;
        let mut nested: Vec<(&[u8], usize)> = Vec::new();
        for bag in bags {
            *bag_count = bag_count.saturating_add(1);
            if *bag_count > MAX_PFX_SAFE_BAGS {
                return Err(err::ERROR_INVALID_DATA);
            }
            if bag.tag != 0x30 {
                continue;
            }
            let Some(bk) = der_item_children(bag.val) else {
                continue;
            };
            if bk.len() < 2 || bk[0].tag != 0x06 || bk[1].tag != 0xA0 {
                continue;
            }
            let bag_oid = bk[0].val;
            let bag_val = bk[1].val;
            let (friendly_name, local_key_id) = parse_bag_attributes(bk.get(2).copied());

            if bag_oid == OID_CERT_BAG {
                if let Some((cb_seq, _)) = parse_der_item(bag_val) {
                    if cb_seq.tag == 0x30 {
                        if let Some(cbk) = der_item_children(cb_seq.val) {
                            if cbk.len() >= 2
                                && cbk[0].tag == 0x06
                                && cbk[0].val == OID_X509_CERT_TYPE
                                && cbk[1].tag == 0xA0
                            {
                                if let Some((oct, _)) = parse_der_item(cbk[1].val) {
                                    if let Some(cert_der) = extract_octet_string(oct) {
                                        certs.push(PfxBagCert {
                                            der: cert_der,
                                            friendly_name,
                                            local_key_id,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            } else if bag_oid == OID_KEY_BAG {
                keys.push(PfxBagKey {
                    pkcs8_der: bag_val.to_vec(),
                    local_key_id,
                });
            } else if bag_oid == OID_PKCS8_SHROUDED_KEY_BAG {
                let (epki, _) = parse_der_item(bag_val).ok_or(err::ERROR_INVALID_DATA)?;
                if epki.tag != 0x30 {
                    return Err(err::ERROR_INVALID_DATA);
                }
                let ek = der_item_children(epki.val).ok_or(err::ERROR_INVALID_DATA)?;
                if ek.len() < 2 || ek[1].tag != 0x04 {
                    return Err(err::ERROR_INVALID_DATA);
                }
                let pkcs8 = decrypt_pbe(ek[0], ek[1].val, password_utf8, pass_bmp, kdf_budget)?;
                keys.push(PfxBagKey {
                    pkcs8_der: pkcs8,
                    local_key_id,
                });
            } else if bag_oid == OID_SAFE_CONTENTS_BAG {
                nested.push((bag_val, depth + 1));
            }
        }
        // Push in reverse order so nested SafeContents bags are traversed in document order.
        for item in nested.into_iter().rev() {
            worklist.push(item);
        }
    }
    Ok(())
}

/// Precomputed public-key identifier derived once per PKCS#8 private key (`O(K)`) so
/// certificate-to-key matching without `localKeyId` runs in `O(C + K)` via `HashMap`
/// instead of `O(C * K)` repeated private-key loads.
#[derive(Clone, PartialEq, Eq, Hash)]
enum PfxPublicKeyId {
    /// Inner contents of the ASN.1 `SubjectPublicKeyInfo` SEQUENCE.
    SpkiInner(Vec<u8>),
    /// Stripped `(modulus, publicExponent)` for legacy <2048-bit RSA keys rejected by `ring`.
    RsaModExp(Vec<u8>, Vec<u8>),
}

fn strip_leading_zero_bytes(mut s: &[u8]) -> &[u8] {
    while s.len() > 1 && s[0] == 0 {
        s = &s[1..];
    }
    s
}

/// Derive a `PfxPublicKeyId` once from an RFC 5958 `PrivateKeyInfo` (`pkcs8_der`).
fn extract_pkcs8_public_key_id(pkcs8_der: &[u8]) -> Option<PfxPublicKeyId> {
    // Primary path via rustls::crypto::ring (covers RSA >= 2048, ECDSA P-256/P-384, Ed25519).
    let pk_der = rustls::pki_types::PrivateKeyDer::Pkcs8(
        rustls::pki_types::PrivatePkcs8KeyDer::from(pkcs8_der.to_vec()),
    );
    if let Ok(signing_key) = crate::net_tls::crypto_provider()
        .key_provider
        .load_private_key(pk_der)
    {
        if let Some(spki) = signing_key.public_key() {
            let got = spki.as_ref();
            if let Some((seq, rest)) = parse_der_item(got) {
                if seq.tag == 0x30 && rest.is_empty() {
                    return Some(PfxPublicKeyId::SpkiInner(seq.val.to_vec()));
                }
            }
            return Some(PfxPublicKeyId::SpkiInner(got.to_vec()));
        }
    }

    // Fallback for legacy RSA keys rejected by ring's >=2048-bit policy (e.g. 1024-bit PFXs).
    let (pki_seq, _) = parse_der_item(pkcs8_der)?;
    if pki_seq.tag != 0x30 {
        return None;
    }
    let pki_kids = der_item_children(pki_seq.val)?;
    if pki_kids.len() < 3 || pki_kids[1].tag != 0x30 || pki_kids[2].tag != 0x04 {
        return None;
    }
    let (priv_alg_oid, _) = parse_alg_id(pki_kids[1])?;
    if priv_alg_oid != OID_RSA_ENCRYPTION {
        return None;
    }
    let (rsa_priv_seq, _) = parse_der_item(pki_kids[2].val)?;
    if rsa_priv_seq.tag != 0x30 {
        return None;
    }
    let rsa_priv_kids = der_item_children(rsa_priv_seq.val)?;
    if rsa_priv_kids.len() < 3 || rsa_priv_kids[1].tag != 0x02 || rsa_priv_kids[2].tag != 0x02 {
        return None;
    }
    Some(PfxPublicKeyId::RsaModExp(
        strip_leading_zero_bytes(rsa_priv_kids[1].val).to_vec(),
        strip_leading_zero_bytes(rsa_priv_kids[2].val).to_vec(),
    ))
}

/// Extract O(1) lookup keys (`SpkiInner` and optional `RsaModExp`) from an X.509 certificate DER.
fn extract_cert_public_key_ids(cert_der: &[u8]) -> Option<(PfxPublicKeyId, Option<PfxPublicKeyId>)> {
    let parts = parse_cert_der_parts(cert_der)?;
    let spki_id = PfxPublicKeyId::SpkiInner(parts.spki_inner.to_vec());
    let rsa_id = (|| {
        let spki_kids = der_item_children(parts.spki_inner)?;
        if spki_kids.len() < 2 || spki_kids[0].tag != 0x30 || spki_kids[1].tag != 0x03 {
            return None;
        }
        let (pub_alg_oid, _) = parse_alg_id(spki_kids[0])?;
        if pub_alg_oid != OID_RSA_ENCRYPTION
            || spki_kids[1].val.len() < 2
            || spki_kids[1].val[0] != 0
        {
            return None;
        }
        let (rsa_pub_seq, _) = parse_der_item(&spki_kids[1].val[1..])?;
        if rsa_pub_seq.tag != 0x30 {
            return None;
        }
        let rsa_pub_kids = der_item_children(rsa_pub_seq.val)?;
        if rsa_pub_kids.len() < 2 || rsa_pub_kids[0].tag != 0x02 || rsa_pub_kids[1].tag != 0x02 {
            return None;
        }
        Some(PfxPublicKeyId::RsaModExp(
            strip_leading_zero_bytes(rsa_pub_kids[0].val).to_vec(),
            strip_leading_zero_bytes(rsa_pub_kids[1].val).to_vec(),
        ))
    })();
    Some((spki_id, rsa_id))
}

pub(super) unsafe fn pfx_import_cert_store_impl(
    p_pfx: *const c_void,
    sz_password: LPCWSTR,
    _dw_flags: u32,
) -> *mut c_void {
    if p_pfx.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let blob = std::ptr::read_unaligned(p_pfx as *const CryptDataBlob);
    if blob.pb_data.is_null() || blob.cb_data == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return std::ptr::null_mut();
    }
    let pfx_der = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);

    let pass_opt = crate::strings::from_wide_opt(sz_password);
    let password_utf8 = pass_opt.as_deref().unwrap_or("");
    let mut pass_bmp = password_to_bmp_string(pass_opt.as_deref());

    // PFX ::= SEQUENCE { version INTEGER(3), authSafe ContentInfo, macData MacData OPTIONAL }
    let Some((pfx_seq, _)) = parse_der_item(pfx_der) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if pfx_seq.tag != 0x30 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    let Some(pfx_kids) = der_item_children(pfx_seq.val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if pfx_kids.len() < 2 || pfx_kids[0].tag != 0x02 || pfx_kids[1].tag != 0x30 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }

    // authSafe ContentInfo (must be OID_DATA containing SEQUENCE OF ContentInfo)
    let Some(as_kids) = der_item_children(pfx_kids[1].val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if as_kids.len() < 2 || as_kids[0].tag != 0x06 || as_kids[0].val != OID_DATA || as_kids[1].tag != 0xA0 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    let Some((as_oct, _)) = parse_der_item(as_kids[1].val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    let Some(auth_safe_bytes) = extract_octet_string(as_oct) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };

    // Shared cumulative KDF work budget across MacData, encrypted SafeContents, and shrouded key bags.
    let mut kdf_budget: u64 = MAX_PFX_TOTAL_KDF_ITERATIONS;

    // Verify MacData if present
    if let Some(mac_elem) = pfx_kids.get(2) {
        if mac_elem.tag != 0x30 {
            crate::teb::set_last_error(err::ERROR_INVALID_DATA);
            return std::ptr::null_mut();
        }
        let Some(mk) = der_item_children(mac_elem.val) else {
            crate::teb::set_last_error(err::ERROR_INVALID_DATA);
            return std::ptr::null_mut();
        };
        if mk.len() < 2 || mk[0].tag != 0x30 || mk[1].tag != 0x04 {
            crate::teb::set_last_error(err::ERROR_INVALID_DATA);
            return std::ptr::null_mut();
        }
        let Some(di_kids) = der_item_children(mk[0].val) else {
            crate::teb::set_last_error(err::ERROR_INVALID_DATA);
            return std::ptr::null_mut();
        };
        if di_kids.len() < 2 || di_kids[1].tag != 0x04 {
            crate::teb::set_last_error(err::ERROR_INVALID_DATA);
            return std::ptr::null_mut();
        }
        let Some((mac_oid, _)) = parse_alg_id(di_kids[0]) else {
            crate::teb::set_last_error(err::ERROR_INVALID_DATA);
            return std::ptr::null_mut();
        };
        let Some(mac_digest) = digest_from_oid(mac_oid) else {
            crate::teb::set_last_error(err::ERROR_INVALID_DATA);
            return std::ptr::null_mut();
        };
        let expected_mac = di_kids[1].val;
        let salt = mk[1].val;
        let iters = match mk.get(2) {
            None => 1u32,
            Some(e) if e.tag == 0x02 => match parse_der_u32_val(e.val) {
                Some(n) if (1..=MAX_PFX_KDF_ITERATIONS).contains(&n) => n,
                _ => {
                    crate::teb::set_last_error(err::ERROR_INVALID_DATA);
                    return std::ptr::null_mut();
                }
            },
            Some(_) => {
                crate::teb::set_last_error(err::ERROR_INVALID_DATA);
                return std::ptr::null_mut();
            }
        };

        let check_mac = |bmp: &[u8], budget: &mut u64| -> Result<bool, u32> {
            let mac_key = pkcs12_kdf(mac_digest, bmp, salt, iters, 3, mac_digest.len(), budget)?;
            let computed = mac_digest.hmac(&mac_key, &auth_safe_bytes);
            Ok(computed.as_slice() == expected_mac)
        };
        let mac_ok = match check_mac(&pass_bmp, &mut kdf_budget) {
            Ok(ok) => ok,
            Err(code) => {
                crate::teb::set_last_error(code);
                return std::ptr::null_mut();
            }
        };
        if !mac_ok {
            // Windows / OpenSSL compatibility: empty password may be encoded as 0-byte or 2-byte NUL BMPString.
            let alt = if pass_bmp.is_empty() && pass_opt.is_none() {
                Some(vec![0u8, 0u8])
            } else if password_utf8.is_empty() && pass_bmp == [0u8, 0u8] {
                Some(Vec::new())
            } else {
                None
            };
            if let Some(alt_bmp) = alt {
                match check_mac(&alt_bmp, &mut kdf_budget) {
                    Ok(true) => pass_bmp = alt_bmp,
                    Ok(false) => {
                        crate::teb::set_last_error(ERROR_INVALID_PASSWORD);
                        return std::ptr::null_mut();
                    }
                    Err(code) => {
                        crate::teb::set_last_error(code);
                        return std::ptr::null_mut();
                    }
                }
            } else {
                crate::teb::set_last_error(ERROR_INVALID_PASSWORD);
                return std::ptr::null_mut();
            }
        }
    }

    // Parse AuthenticatedSafe ::= SEQUENCE OF ContentInfo
    let Some((safe_seq, _)) = parse_der_item(&auth_safe_bytes) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };
    if safe_seq.tag != 0x30 {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }
    let Some(content_infos) = der_item_children(safe_seq.val) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    };

    if content_infos.len() > MAX_PFX_SAFE_BAGS {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return std::ptr::null_mut();
    }

    let mut certs: Vec<PfxBagCert> = Vec::new();
    let mut keys: Vec<PfxBagKey> = Vec::new();
    let mut bag_count = 0usize;

    for ci in content_infos {
        if ci.tag != 0x30 {
            continue;
        }
        let Some(ck) = der_item_children(ci.val) else {
            continue;
        };
        if ck.len() < 2 || ck[0].tag != 0x06 || ck[1].tag != 0xA0 {
            continue;
        }
        if ck[0].val == OID_DATA {
            let Some((oct, _)) = parse_der_item(ck[1].val) else {
                continue;
            };
            let Some(sc_bytes) = extract_octet_string(oct) else {
                continue;
            };
            if let Err(code) = parse_safe_contents(
                &sc_bytes,
                password_utf8,
                &pass_bmp,
                &mut certs,
                &mut keys,
                &mut bag_count,
                &mut kdf_budget,
            ) {
                crate::teb::set_last_error(code);
                return std::ptr::null_mut();
            }
        } else if ck[0].val == OID_ENCRYPTED_DATA {
            let Some((ed_seq, _)) = parse_der_item(ck[1].val) else {
                continue;
            };
            if ed_seq.tag != 0x30 {
                continue;
            }
            let Some(edk) = der_item_children(ed_seq.val) else {
                continue;
            };
            // EncryptedData ::= SEQUENCE { version, encryptedContentInfo SEQUENCE { contentType, contentEncryptionAlgorithm, [0] IMPLICIT encryptedContent } }
            if edk.len() < 2 || edk[1].tag != 0x30 {
                continue;
            }
            let Some(ecik) = der_item_children(edk[1].val) else {
                continue;
            };
            if ecik.len() < 3 {
                continue;
            }
            let enc_bytes = if ecik[2].tag == 0x80 {
                ecik[2].val.to_vec()
            } else if ecik[2].tag == 0xA0 {
                let mut acc = Vec::new();
                if let Some(parts) = der_item_children(ecik[2].val) {
                    for p in parts {
                        if let Some(b) = extract_octet_string(p) {
                            acc.extend_from_slice(&b);
                        }
                    }
                }
                acc
            } else {
                continue;
            };
            let plain = match decrypt_pbe(
                ecik[1],
                &enc_bytes,
                password_utf8,
                &pass_bmp,
                &mut kdf_budget,
            ) {
                Ok(p) => p,
                Err(code) => {
                    crate::teb::set_last_error(code);
                    return std::ptr::null_mut();
                }
            };
            if let Err(code) = parse_safe_contents(
                &plain,
                password_utf8,
                &pass_bmp,
                &mut certs,
                &mut keys,
                &mut bag_count,
                &mut kdf_budget,
            ) {
                crate::teb::set_last_error(code);
                return std::ptr::null_mut();
            }
        }
    }

    let store = cert_open_store_impl(CERT_STORE_PROV_MEMORY as *const u8, std::ptr::null());
    if store.is_null() {
        return std::ptr::null_mut();
    }

    // Precompute O(1) localKeyId index and lazily precompute O(K) public-key index once if any
    // certificate needs fallback SPKI matching (bounding matching to O(C + K) instead of O(C * K)).
    let mut keys_by_local_id: std::collections::HashMap<&[u8], usize> =
        std::collections::HashMap::with_capacity(keys.len());
    for (idx, k) in keys.iter().enumerate() {
        if let Some(ref id) = k.local_key_id {
            keys_by_local_id.entry(id.as_slice()).or_insert(idx);
        }
    }
    let mut keys_by_pub_id: Option<std::collections::HashMap<PfxPublicKeyId, usize>> = None;

    for cert in &certs {
        // Match private key by localKeyId first, falling back to precomputed cryptographic SPKI
        // matching when bag attributes (RFC 7292 §4.2) are omitted — never attach blindly by index.
        let matched_key = cert
            .local_key_id
            .as_ref()
            .and_then(|id| keys_by_local_id.get(id.as_slice()).copied())
            .or_else(|| {
                if keys.is_empty() {
                    return None;
                }
                let pub_map = keys_by_pub_id.get_or_insert_with(|| {
                    let mut m = std::collections::HashMap::with_capacity(keys.len());
                    for (idx, k) in keys.iter().enumerate() {
                        if let Some(pub_id) = extract_pkcs8_public_key_id(&k.pkcs8_der) {
                            m.entry(pub_id).or_insert(idx);
                        }
                    }
                    m
                });
                let (spki_id, rsa_id) = extract_cert_public_key_ids(&cert.der)?;
                pub_map
                    .get(&spki_id)
                    .copied()
                    .or_else(|| rsa_id.and_then(|rid| pub_map.get(&rid).copied()))
            })
            .map(|idx| keys[idx].pkcs8_der.clone());

        let mut props: Vec<(u32, Vec<u8>)> = Vec::new();
        if let Some(ref name) = cert.friendly_name {
            let wide: Vec<u8> = name
                .encode_utf16()
                .chain(std::iter::once(0))
                .flat_map(u16::to_le_bytes)
                .collect();
            props.push((CERT_FRIENDLY_NAME_PROP_ID, wide));
        }
        add_der_with_meta_to_store(store, &cert.der, props, matched_key);
    }

    store
}

#[cfg(test)]
pub(super) unsafe fn test_encrypt_pbes2_aes128_sha1(
    password_utf8: &str,
    salt: &[u8],
    iters: u32,
    iv: &[u8; 16],
    plaintext: &[u8],
) -> Vec<u8> {
    let mut key = [0u8; 16];
    let pw_bytes = password_utf8.as_bytes();
    let st = CCKeyDerivationPBKDF(
        K_CC_PBKDF2,
        pw_bytes.as_ptr(),
        pw_bytes.len(),
        salt.as_ptr(),
        salt.len(),
        K_CC_PRF_HMAC_SHA1,
        iters,
        key.as_mut_ptr(),
        key.len(),
    );
    assert_eq!(st, 0, "CCKeyDerivationPBKDF failed in test helper");
    let mut out = vec![0u8; plaintext.len() + 32];
    let mut moved = 0usize;
    let st2 = CCCrypt(
        0, // kCCEncrypt
        K_CC_ALGORITHM_AES,
        K_CC_OPTION_PKCS7_PADDING,
        key.as_ptr() as *const c_void,
        key.len(),
        iv.as_ptr() as *const c_void,
        plaintext.as_ptr() as *const c_void,
        plaintext.len(),
        out.as_mut_ptr() as *mut c_void,
        out.len(),
        &mut moved,
    );
    assert_eq!(st2, 0, "CCCrypt encrypt failed in test helper");
    out.truncate(moved);
    out
}
