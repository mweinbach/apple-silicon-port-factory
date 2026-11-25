//! wintrust.dll: PE Authenticode verification (`WinVerifyTrust`), provider state helpers,
//! and catalog/SFC/TBS exports.
//!
//! `WinVerifyTrust` (`WINTRUST_ACTION_GENERIC_VERIFY_V2`, `WTD_CHOICE_FILE`) implements real
//! cryptographic PE Authenticode verification:
//! - computes the Microsoft PE Authenticode digest (excluding the optional header `CheckSum`,
//!   the `IMAGE_DIRECTORY_ENTRY_SECURITY` table entry, and the trailing `WIN_CERTIFICATE` blob),
//! - parses the embedded PKCS#7 `SignedData` / `SpcIndirectDataContent` ASN.1 DER structure,
//! - verifies the computed PE digest matches `SpcIndirectDataContent.messageDigest`,
//! - verifies the `SignerInfo` `messageDigest` authenticated attribute against the
//!   `SpcIndirectDataContent` value bytes,
//! - verifies the RSA PKCS#1 v1.5 signature on the SET-encoded authenticated attributes
//!   against the matching signer certificate's public key, and
//! - verifies RSA signatures along any certificate chain present in the PKCS#7 bundle.

use super::bcrypt::{Digest, RsaPublicKey};
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{LazyLock, Mutex};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_SUCCESS: i32 = 0;
const TRUST_E_NOSIGNATURE: i32 = 0x800B_0100u32 as i32;
const TRUST_E_SUBJECT_FORM_UNKNOWN: i32 = 0x800B_0003u32 as i32;
const TRUST_E_PROVIDER_UNKNOWN: i32 = 0x800B_0001u32 as i32;
const TRUST_E_ACTION_UNKNOWN: i32 = 0x800B_0002u32 as i32;
const TRUST_E_BAD_DIGEST: i32 = 0x8009_6010u32 as i32;
const CRYPT_E_FILE_ERROR: i32 = 0x8009_2003u32 as i32;
const CERT_E_CHAINING: i32 = 0x800B_010Au32 as i32;

const WINTRUST_ACTION_GENERIC_VERIFY_V2: [u8; 16] = [
    0x6B, 0xC5, 0xAA, 0x00, 0x44, 0xCD, 0xD0, 0x11, 0x8C, 0xC2, 0x00, 0xC0, 0x4F, 0xC2, 0x95, 0xEE,
];
const WINTRUST_ACTION_TRUSTPROVIDER_TEST: [u8; 16] = [
    0xF8, 0x31, 0x3E, 0x57, 0xBA, 0xDD, 0xD0, 0x11, 0x8C, 0xCB, 0x00, 0xC0, 0x4F, 0xC2, 0x95, 0xEE,
];
const WINTRUST_ACTION_GENERIC_CERT_VERIFY: [u8; 16] = [
    0x42, 0x38, 0x9A, 0x18, 0x41, 0x30, 0xD1, 0x11, 0x85, 0xE1, 0x00, 0xC0, 0x4F, 0xC2, 0x95, 0xEE,
];
const WINTRUST_ACTION_GENERIC_CHAIN_VERIFY: [u8; 16] = [
    0x16, 0x1C, 0x45, 0xFC, 0x75, 0xAC, 0xD1, 0x11, 0xB4, 0xB8, 0x00, 0xC0, 0x4F, 0xB6, 0x6E, 0xA0,
];
const DRIVER_ACTION_VERIFY: [u8; 16] = [
    0xC3, 0xE6, 0x50, 0xF7, 0xEE, 0x38, 0xD1, 0x11, 0x85, 0xE5, 0x00, 0xC0, 0x4F, 0xC2, 0x95, 0xEE,
];
const HTTPSPROV_ACTION: [u8; 16] = [
    0xF8, 0x31, 0x3E, 0x57, 0xBA, 0xAA, 0xD0, 0x11, 0x8C, 0xCB, 0x00, 0xC0, 0x4F, 0xC2, 0x95, 0xEE,
];

const WTD_CHOICE_FILE: u32 = 1;
const WTD_CHOICE_CATALOG: u32 = 2;
const WTD_CHOICE_BLOB: u32 = 3;
const WTD_CHOICE_SIGNER: u32 = 4;
const WTD_CHOICE_CERT: u32 = 5;
const WTD_STATEACTION_VERIFY: u32 = 1;
const WTD_STATEACTION_CLOSE: u32 = 2;

// WINTRUST_DATA.fdwRevocationChecks (+0x1C)
const WTD_REVOKE_NONE: u32 = 0x0000_0000;
const WTD_REVOKE_WHOLECHAIN: u32 = 0x0000_0001;

// WINTRUST_DATA.dwProvFlags (+0x48, when cbStruct >= 0x4C)
const WTD_REVOCATION_CHECK_NONE: u32 = 0x0000_0010;
const WTD_REVOCATION_CHECK_END_CERT: u32 = 0x0000_0020;
const WTD_REVOCATION_CHECK_CHAIN: u32 = 0x0000_0040;
const WTD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 0x0000_0080;
const WTD_CACHE_ONLY_URL_RETRIEVAL: u32 = 0x0000_1000;

// CertGetCertificateChain dwFlags mapped from WINTRUST_DATA revocation fields
const CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: u32 = 0x0000_0004;
const CERT_CHAIN_REVOCATION_CHECK_END_CERT: u32 = 0x1000_0000;
const CERT_CHAIN_REVOCATION_CHECK_CHAIN: u32 = 0x2000_0000;
const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 0x4000_0000;
const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 0x8000_0000;

const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u16 = 0x0002;
const X509_ASN_ENCODING: u32 = 1;
const MAX_CMS_CERTS: usize = 128;
const MAX_CMS_SIGNERS: usize = 32;
const MAX_CMS_AUTH_ATTRS: usize = 64;

// DER OID value bytes
const OID_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x01];
const OID_SIGNED_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02];
const OID_SPC_INDIRECT_DATA: &[u8] = &[0x2B, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x02, 0x01, 0x04];
const OID_CTL: &[u8] = &[0x2B, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x0A, 0x01];
const OID_CONTENT_TYPE: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x03];
const OID_MESSAGE_DIGEST: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x04];
const OID_RSA_ENCRYPTION: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01];

const OID_MD5: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x02, 0x05];
const OID_SHA1: &[u8] = &[0x2B, 0x0E, 0x03, 0x02, 0x1A];
const OID_SHA256: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01];
const OID_SHA384: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02];
const OID_SHA512: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03];

// Signature algorithm OIDs -> digest for RSA PKCS#1 v1.5
const OID_MD5_WITH_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x04];
const OID_SHA1_WITH_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x05];
const OID_SHA256_WITH_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B];
const OID_SHA384_WITH_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0C];
const OID_SHA512_WITH_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0D];

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

fn rsa_sig_digest_from_oid(oid: &[u8]) -> Option<Digest> {
    match oid {
        OID_MD5_WITH_RSA => Some(Digest::Md5),
        OID_SHA1_WITH_RSA => Some(Digest::Sha1),
        OID_SHA256_WITH_RSA => Some(Digest::Sha256),
        OID_SHA384_WITH_RSA => Some(Digest::Sha384),
        OID_SHA512_WITH_RSA => Some(Digest::Sha512),
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct DerElem<'a> {
    tag: u8,
    raw: &'a [u8],
    val: &'a [u8],
}

fn parse_der_elem(buf: &[u8]) -> Option<(DerElem<'_>, &[u8])> {
    if buf.len() < 2 {
        return None;
    }
    let tag = buf[0];
    let lb = buf[1];
    let (hdr_len, val_len) = if (lb & 0x80) == 0 {
        (2usize, lb as usize)
    } else {
        let n = (lb & 0x7F) as usize;
        if n == 0 || n > 4 || buf.len() < 2 + n {
            return None;
        }
        let mut len = 0usize;
        for &b in &buf[2..2 + n] {
            len = (len << 8) | (b as usize);
        }
        (2 + n, len)
    };
    let end = hdr_len.checked_add(val_len)?;
    if end > buf.len() {
        return None;
    }
    Some((DerElem { tag, raw: &buf[..end], val: &buf[hdr_len..end] }, &buf[end..]))
}

fn der_children(mut buf: &[u8]) -> Option<Vec<DerElem<'_>>> {
    let mut out = Vec::new();
    while !buf.is_empty() {
        let (e, rest) = parse_der_elem(buf)?;
        out.push(e);
        buf = rest;
    }
    Some(out)
}

fn der_children_bounded(mut buf: &[u8], max_items: usize) -> Option<Vec<DerElem<'_>>> {
    let mut out = Vec::new();
    while !buf.is_empty() {
        if out.len() >= max_items {
            return None;
        }
        let (e, rest) = parse_der_elem(buf)?;
        out.push(e);
        buf = rest;
    }
    Some(out)
}

fn parse_alg_id_oid(elem: DerElem<'_>) -> Option<&'_ [u8]> {
    if elem.tag != 0x30 {
        return None;
    }
    let (oid, _) = parse_der_elem(elem.val)?;
    (oid.tag == 0x06).then_some(oid.val)
}

fn strip_der_int_sign(mut b: &[u8]) -> &[u8] {
    while b.len() > 1 && b[0] == 0 {
        b = &b[1..];
    }
    b
}

pub(crate) fn decode_oid_str(oid: &[u8]) -> String {
    if oid.is_empty() {
        return String::new();
    }
    let first = (oid[0] / 40) as u32;
    let second = (oid[0] % 40) as u32;
    let mut parts = vec![first.to_string(), second.to_string()];
    let mut i = 1usize;
    while i < oid.len() {
        let mut val: u64 = 0;
        loop {
            let b = oid[i];
            i += 1;
            val = (val << 7) | ((b & 0x7F) as u64);
            if (b & 0x80) == 0 || i >= oid.len() {
                break;
            }
        }
        parts.push(val.to_string());
    }
    parts.join(".")
}

pub(crate) fn parse_asn1_time_filetime_raw(tag: u8, val: &[u8]) -> u64 {
    parse_asn1_time_filetime(DerElem { tag, raw: val, val })
}

fn parse_asn1_time_filetime(elem: DerElem<'_>) -> u64 {
    let s = std::str::from_utf8(elem.val).unwrap_or("");
    let (year, rest) = match elem.tag {
        0x17 if s.len() >= 12 => {
            let yy: i64 = s[0..2].parse().unwrap_or(0);
            (if yy >= 50 { 1900 + yy } else { 2000 + yy }, &s[2..])
        }
        0x18 if s.len() >= 14 => (s[0..4].parse().unwrap_or(1970), &s[4..]),
        _ => return 0,
    };
    let month: i64 = rest.get(0..2).and_then(|x| x.parse().ok()).unwrap_or(1);
    let day: i64 = rest.get(2..4).and_then(|x| x.parse().ok()).unwrap_or(1);
    let hour: i64 = rest.get(4..6).and_then(|x| x.parse().ok()).unwrap_or(0);
    let min: i64 = rest.get(6..8).and_then(|x| x.parse().ok()).unwrap_or(0);
    let sec: i64 = rest.get(8..10).and_then(|x| x.parse().ok()).unwrap_or(0);
    // Howard Hinnant's civil_from_days inverse: days since 1970-01-01
    let y = year - if month <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days_since_unix = era * 146097 + doe - 719468;
    let unix_secs = days_since_unix * 86400 + hour * 3600 + min * 60 + sec;
    // FILETIME epoch is 1601-01-01 (11644473600 seconds before 1970-01-01).
    let win_secs = unix_secs + 11_644_473_600;
    if win_secs <= 0 {
        0
    } else {
        (win_secs as u64).saturating_mul(10_000_000)
    }
}

struct ParsedCert<'a> {
    version: u32,
    raw_der: &'a [u8],
    tbs_raw: &'a [u8],
    sig_alg_oid: &'a [u8],
    sig_bytes: &'a [u8],
    serial_val: &'a [u8],
    issuer_raw: &'a [u8],
    not_before: u64,
    not_after: u64,
    subject_raw: &'a [u8],
    spki_raw: &'a [u8],
    spki_alg_oid: &'a [u8],
    spki_alg_params: &'a [u8],
    spki_key_bytes: &'a [u8],
    spki_unused_bits: u32,
    rsa_pub: Option<RsaPublicKey>,
}

fn parse_x509_cert(elem: DerElem<'_>) -> Option<ParsedCert<'_>> {
    if elem.tag != 0x30 {
        return None;
    }
    let top = der_children(elem.val)?;
    if top.len() < 3 || top[0].tag != 0x30 || top[2].tag != 0x03 || top[2].val.is_empty() {
        return None;
    }
    let tbs = top[0];
    let sig_alg_oid = parse_alg_id_oid(top[1])?;
    if top[2].val[0] != 0 {
        return None;
    }
    let sig_bytes = &top[2].val[1..];

    let tbs_kids = der_children(tbs.val)?;
    let mut idx = 0usize;
    let mut version = 0u32;
    if tbs_kids.get(idx)?.tag == 0xA0 {
        if let Some((ver_e, _)) = parse_der_elem(tbs_kids[idx].val) {
            if ver_e.tag == 0x02 && !ver_e.val.is_empty() {
                version = ver_e.val[ver_e.val.len() - 1] as u32;
            }
        }
        idx += 1;
    }
    // serialNumber, signature, issuer, validity, subject, subjectPublicKeyInfo
    let serial = *tbs_kids.get(idx)?;
    let issuer = *tbs_kids.get(idx + 2)?;
    let validity = *tbs_kids.get(idx + 3)?;
    let subject = *tbs_kids.get(idx + 4)?;
    let spki = *tbs_kids.get(idx + 5)?;
    if serial.tag != 0x02 || issuer.tag != 0x30 || subject.tag != 0x30 || spki.tag != 0x30 {
        return None;
    }
    let (not_before, not_after) = if validity.tag == 0x30 {
        if let Some(vk) = der_children(validity.val) {
            if vk.len() >= 2 {
                (parse_asn1_time_filetime(vk[0]), parse_asn1_time_filetime(vk[1]))
            } else {
                (0, u64::MAX / 2)
            }
        } else {
            (0, u64::MAX / 2)
        }
    } else {
        (0, u64::MAX / 2)
    };
    let mut rsa_pub = None;
    let mut spki_alg_oid: &[u8] = &[];
    let mut spki_alg_params: &[u8] = &[];
    let mut spki_key_bytes: &[u8] = &[];
    let mut spki_unused_bits = 0u32;
    if let Some(spki_kids) = der_children(spki.val) {
        if spki_kids.len() >= 2 {
            if let Some(oid) = parse_alg_id_oid(spki_kids[0]) {
                spki_alg_oid = oid;
            }
            if spki_kids[0].tag == 0x30 {
                if let Some(alg_kids) = der_children(spki_kids[0].val) {
                    if alg_kids.len() >= 2 {
                        spki_alg_params = alg_kids[1].raw;
                    }
                }
            }
            if spki_kids[1].tag == 0x03 && !spki_kids[1].val.is_empty() {
                spki_unused_bits = spki_kids[1].val[0] as u32;
                spki_key_bytes = &spki_kids[1].val[1..];
                if spki_alg_oid == OID_RSA_ENCRYPTION && spki_unused_bits == 0 {
                    if let Some((pub_seq, _)) = parse_der_elem(spki_key_bytes) {
                        if pub_seq.tag == 0x30 {
                            if let Some(pk) = der_children(pub_seq.val) {
                                if pk.len() >= 2 && pk[0].tag == 0x02 && pk[1].tag == 0x02 {
                                    rsa_pub = Some(RsaPublicKey {
                                        n: strip_der_int_sign(pk[0].val).to_vec(),
                                        e: strip_der_int_sign(pk[1].val).to_vec(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Some(ParsedCert {
        version,
        raw_der: elem.raw,
        tbs_raw: tbs.raw,
        sig_alg_oid,
        sig_bytes,
        serial_val: serial.val,
        issuer_raw: issuer.raw,
        not_before,
        not_after,
        subject_raw: subject.raw,
        spki_raw: spki.raw,
        spki_alg_oid,
        spki_alg_params,
        spki_key_bytes,
        spki_unused_bits,
        rsa_pub,
    })
}

struct VerifiedCert {
    version: u32,
    raw_der: Vec<u8>,
    serial_val: Vec<u8>,
    sig_alg_oid_str: String,
    issuer_raw: Vec<u8>,
    not_before: u64,
    not_after: u64,
    subject_raw: Vec<u8>,
    spki_alg_oid_str: String,
    spki_key_bytes: Vec<u8>,
    spki_unused_bits: u32,
    self_signed: bool,
}

struct VerifiedSigner {
    hash_alg_oid_str: String,
    enc_alg_oid_str: String,
    encrypted_hash: Vec<u8>,
}

struct VerifiedAuthenticode {
    certs: Vec<VerifiedCert>,
    signer: VerifiedSigner,
}

/// Extract PE Authenticode slices + embedded PKCS#7 blob and cryptographically verify:
/// 1. PE Authenticode digest == `SpcIndirectDataContent.messageDigest`,
/// 2. `SignerInfo` `messageDigest` attribute == hash of `SpcIndirectDataContent` inner value,
/// 3. `SignerInfo` RSA PKCS#1 v1.5 signature over SET-tagged authenticated attributes,
/// 4. RSA signatures of any certificate chain links present in `SignedData.certificates`.
fn verify_pe_authenticode(bytes: &[u8]) -> Result<VerifiedAuthenticode, i32> {
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return Err(TRUST_E_SUBJECT_FORM_UNKNOWN);
    }
    let pe_off = u32::from_le_bytes(bytes[0x3C..0x40].try_into().unwrap()) as usize;
    if pe_off + 24 + 96 > bytes.len() || &bytes[pe_off..pe_off + 4] != b"PE\0\0" {
        return Err(TRUST_E_SUBJECT_FORM_UNKNOWN);
    }
    let opt = pe_off + 24;
    let magic = u16::from_le_bytes(bytes[opt..opt + 2].try_into().unwrap());
    let dd_off = match magic {
        0x10B => opt + 96,
        0x20B => opt + 112,
        _ => return Err(TRUST_E_SUBJECT_FORM_UNKNOWN),
    };
    let checksum_off = opt + 64;
    let sec_dir_off = dd_off + 4 * 8;
    if sec_dir_off + 8 > bytes.len() {
        return Err(TRUST_E_SUBJECT_FORM_UNKNOWN);
    }
    let sec_off = u32::from_le_bytes(bytes[sec_dir_off..sec_dir_off + 4].try_into().unwrap()) as usize;
    let sec_sz = u32::from_le_bytes(bytes[sec_dir_off + 4..sec_dir_off + 8].try_into().unwrap()) as usize;
    if sec_off == 0 || sec_sz < 8 || sec_off < sec_dir_off + 8 || sec_off.saturating_add(sec_sz) > bytes.len() {
        return Err(TRUST_E_NOSIGNATURE);
    }

    let dw_len = u32::from_le_bytes(bytes[sec_off..sec_off + 4].try_into().unwrap()) as usize;
    let w_cert_type = u16::from_le_bytes(bytes[sec_off + 6..sec_off + 8].try_into().unwrap());
    if w_cert_type != WIN_CERT_TYPE_PKCS_SIGNED_DATA || dw_len < 8 || dw_len > sec_sz {
        return Err(TRUST_E_NOSIGNATURE);
    }
    let pkcs7_raw = &bytes[sec_off + 8..sec_off + dw_len];

    // ContentInfo SEQUENCE { contentType OID(signedData), [0] EXPLICIT SignedData }
    let (ci, _) = parse_der_elem(pkcs7_raw).ok_or(TRUST_E_BAD_DIGEST)?;
    if ci.tag != 0x30 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    let ci_kids = der_children(ci.val).ok_or(TRUST_E_BAD_DIGEST)?;
    if ci_kids.len() < 2 || ci_kids[0].tag != 0x06 || ci_kids[0].val != OID_SIGNED_DATA || ci_kids[1].tag != 0xA0 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    let (sd_seq, _) = parse_der_elem(ci_kids[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
    if sd_seq.tag != 0x30 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    let sd_kids = der_children(sd_seq.val).ok_or(TRUST_E_BAD_DIGEST)?;
    if sd_kids.len() < 4 {
        return Err(TRUST_E_BAD_DIGEST);
    }

    // encapContentInfo: SEQUENCE { contentType OID(SPC_INDIRECT_DATA), [0] EXPLICIT SpcIndirectDataContent }
    let eci_kids = der_children(sd_kids[2].val).ok_or(TRUST_E_BAD_DIGEST)?;
    if eci_kids.len() < 2 || eci_kids[0].tag != 0x06 || eci_kids[0].val != OID_SPC_INDIRECT_DATA || eci_kids[1].tag != 0xA0 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    let (spc_seq, _) = parse_der_elem(eci_kids[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
    if spc_seq.tag != 0x30 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    let spc_kids = der_children(spc_seq.val).ok_or(TRUST_E_BAD_DIGEST)?;
    if spc_kids.len() < 2 || spc_kids[1].tag != 0x30 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    // DigestInfo ::= SEQUENCE { digestAlgorithm AlgorithmIdentifier, digest OCTET STRING }
    let di_kids = der_children(spc_kids[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
    if di_kids.len() < 2 || di_kids[1].tag != 0x04 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    let spc_alg_oid = parse_alg_id_oid(di_kids[0]).ok_or(TRUST_E_BAD_DIGEST)?;
    let spc_digest_alg = digest_from_oid(spc_alg_oid).ok_or(TRUST_E_BAD_DIGEST)?;
    let expected_pe_hash = di_kids[1].val;

    // Step 1: Compute and compare the real PE Authenticode hash.
    let computed_pe_hash = spc_digest_alg.hash_slices(&[
        &bytes[..checksum_off],
        &bytes[checksum_off + 4..sec_dir_off],
        &bytes[sec_dir_off + 8..sec_off],
        &bytes[sec_off + sec_sz..],
    ]);
    if computed_pe_hash.as_slice() != expected_pe_hash {
        return Err(TRUST_E_BAD_DIGEST);
    }

    // Parse certificates ([0] IMPLICIT) and signerInfos (SET) with hard bounds.
    let mut certs: Vec<ParsedCert<'_>> = Vec::new();
    let mut raw_cert_count = 0usize;
    let mut signer_infos_elem: Option<DerElem<'_>> = None;
    for &elem in &sd_kids[3..] {
        if elem.tag == 0xA0 {
            let remaining = MAX_CMS_CERTS.saturating_sub(raw_cert_count);
            let cert_elems = der_children_bounded(elem.val, remaining).ok_or(TRUST_E_BAD_DIGEST)?;
            raw_cert_count += cert_elems.len();
            for ce in cert_elems {
                if let Some(c) = parse_x509_cert(ce) {
                    certs.push(c);
                }
            }
        } else if elem.tag == 0x31 {
            signer_infos_elem = Some(elem);
        }
    }
    let si_set = signer_infos_elem.ok_or(TRUST_E_BAD_DIGEST)?;
    let si_list = der_children_bounded(si_set.val, MAX_CMS_SIGNERS).ok_or(TRUST_E_BAD_DIGEST)?;
    if si_list.is_empty() || certs.is_empty() {
        return Err(TRUST_E_BAD_DIGEST);
    }

    // Index parsed certificates by (issuer DER, normalized serial) for O(1) signer matching.
    let mut certs_by_ias: HashMap<(&[u8], &[u8]), usize> = HashMap::with_capacity(certs.len());
    for (idx, c) in certs.iter().enumerate() {
        certs_by_ias
            .entry((c.issuer_raw, strip_der_int_sign(c.serial_val)))
            .or_insert(idx);
    }

    // Step 2 & 3: Verify at least one SignerInfo against its matching certificate.
    let mut verified_signer: Option<(usize, VerifiedSigner)> = None;
    for si in si_list {
        if si.tag != 0x30 {
            continue;
        }
        let kids = der_children(si.val).ok_or(TRUST_E_BAD_DIGEST)?;
        if kids.len() < 5 || kids[1].tag != 0x30 {
            continue;
        }
        // issuerAndSerialNumber
        let ias = der_children(kids[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
        if ias.len() < 2 || ias[0].tag != 0x30 || ias[1].tag != 0x02 {
            continue;
        }
        let issuer_raw = ias[0].raw;
        let serial_val = strip_der_int_sign(ias[1].val);
        let Some(&cert_idx) = certs_by_ias.get(&(issuer_raw, serial_val)) else {
            continue;
        };
        let Some(ref rsa_pub) = certs[cert_idx].rsa_pub else {
            continue;
        };
        let hash_alg_oid = parse_alg_id_oid(kids[2]).ok_or(TRUST_E_BAD_DIGEST)?;
        let si_digest = digest_from_oid(hash_alg_oid).ok_or(TRUST_E_BAD_DIGEST)?;

        let mut pos = 3usize;
        let auth_attrs = if kids.get(pos).is_some_and(|e| e.tag == 0xA0) {
            pos += 1;
            Some(kids[pos - 1])
        } else {
            None
        };
        // digestEncryptionAlgorithm, encryptedDigest
        let enc_alg = *kids.get(pos).ok_or(TRUST_E_BAD_DIGEST)?;
        let enc_alg_oid = parse_alg_id_oid(enc_alg).unwrap_or(OID_RSA_ENCRYPTION);
        let enc_digest = *kids.get(pos + 1).ok_or(TRUST_E_BAD_DIGEST)?;
        if enc_digest.tag != 0x04 {
            return Err(TRUST_E_BAD_DIGEST);
        }

        if let Some(aa) = auth_attrs {
            let mut md_count = 0usize;
            let mut ct_count = 0usize;
            for attr in der_children_bounded(aa.val, MAX_CMS_AUTH_ATTRS).ok_or(TRUST_E_BAD_DIGEST)? {
                if attr.tag != 0x30 {
                    continue;
                }
                let ak = der_children(attr.val).ok_or(TRUST_E_BAD_DIGEST)?;
                if ak.len() >= 2 && ak[0].tag == 0x06 && ak[1].tag == 0x31 {
                    if ak[0].val == OID_MESSAGE_DIGEST {
                        let vals = der_children(ak[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
                        let expected_content_md = si_digest.hash(spc_seq.val);
                        if vals.len() != 1
                            || vals[0].tag != 0x04
                            || vals[0].val != expected_content_md.as_slice()
                        {
                            return Err(TRUST_E_BAD_DIGEST);
                        }
                        md_count += 1;
                    } else if ak[0].val == OID_CONTENT_TYPE {
                        let vals = der_children(ak[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
                        if vals.len() != 1
                            || vals[0].tag != 0x06
                            || vals[0].val != OID_SPC_INDIRECT_DATA
                        {
                            return Err(TRUST_E_BAD_DIGEST);
                        }
                        ct_count += 1;
                    }
                }
            }
            if md_count != 1 || ct_count != 1 {
                return Err(TRUST_E_BAD_DIGEST);
            }
            // Re-encode authenticatedAttributes with EXPLICIT SET tag (0x31) per RFC 2315 §9.3.
            let mut set_der = Vec::with_capacity(aa.raw.len());
            set_der.push(0x31);
            set_der.extend_from_slice(&aa.raw[1..]);
            let attr_hash = si_digest.hash(&set_der);
            if !rsa_pub.verify_pkcs1(Some(si_digest), &attr_hash, enc_digest.val) {
                return Err(TRUST_E_BAD_DIGEST);
            }
        } else {
            // Per RFC 5652 §5.3 & Authenticode spec, signedAttrs MUST be present for non-id-data
            // content types (Authenticode always uses OID_SPC_INDIRECT_DATA).
            return Err(TRUST_E_BAD_DIGEST);
        }
        verified_signer = Some((
            cert_idx,
            VerifiedSigner {
                hash_alg_oid_str: decode_oid_str(hash_alg_oid),
                enc_alg_oid_str: decode_oid_str(enc_alg_oid),
                encrypted_hash: enc_digest.val.to_vec(),
            },
        ));
        break;
    }

    let Some((mut cur_idx, signer)) = verified_signer else {
        return Err(TRUST_E_BAD_DIGEST);
    };

    // Step 4: Verify certificate chain links present in the CMS bundle, ordering leaf first.
    let to_verified = |c: &ParsedCert<'_>| VerifiedCert {
        version: c.version,
        raw_der: c.raw_der.to_vec(),
        serial_val: c.serial_val.to_vec(),
        sig_alg_oid_str: decode_oid_str(c.sig_alg_oid),
        issuer_raw: c.issuer_raw.to_vec(),
        not_before: c.not_before,
        not_after: c.not_after,
        subject_raw: c.subject_raw.to_vec(),
        spki_alg_oid_str: decode_oid_str(c.spki_alg_oid),
        spki_key_bytes: c.spki_key_bytes.to_vec(),
        spki_unused_bits: c.spki_unused_bits,
        self_signed: c.issuer_raw == c.subject_raw,
    };
    let mut chain = vec![to_verified(&certs[cur_idx])];
    let mut visited = vec![cur_idx];
    for _ in 0..16 {
        let cur = &certs[cur_idx];
        if cur.issuer_raw == cur.subject_raw {
            break; // self-signed root
        }
        // Prefer a self-signed root cert over a cross-signed cert when both share the issuer DN.
        let parent_idx = certs
            .iter()
            .enumerate()
            .position(|(i, c)| {
                !visited.contains(&i)
                    && c.subject_raw == cur.issuer_raw
                    && c.issuer_raw == c.subject_raw
                    && c.raw_der != cur.raw_der
            })
            .or_else(|| {
                certs.iter().enumerate().position(|(i, c)| {
                    !visited.contains(&i) && c.subject_raw == cur.issuer_raw && c.raw_der != cur.raw_der
                })
            });
        let Some(parent_idx) = parent_idx else {
            break; // stops at external root CA (not embedded in PKCS#7 bundle)
        };
        let parent = &certs[parent_idx];
        if let (Some(ref pub_key), Some(sig_dig)) = (&parent.rsa_pub, rsa_sig_digest_from_oid(cur.sig_alg_oid)) {
            let tbs_hash = sig_dig.hash(cur.tbs_raw);
            if !pub_key.verify_pkcs1(Some(sig_dig), &tbs_hash, cur.sig_bytes) {
                return Err(CERT_E_CHAINING);
            }
        }
        visited.push(parent_idx);
        chain.push(to_verified(parent));
        cur_idx = parent_idx;
    }

    Ok(VerifiedAuthenticode { certs: chain, signer })
}

pub(crate) struct ParsedSignerInfo {
    pub version: u32,
    pub issuer_raw: Vec<u8>,
    pub serial_be: Vec<u8>,
    pub hash_alg_oid_str: String,
    pub enc_alg_oid_str: String,
    pub encrypted_hash: Vec<u8>,
}

pub(crate) struct ParsedPkcs7Info {
    pub certs_der: Vec<Vec<u8>>,
    pub signers: Vec<ParsedSignerInfo>,
    pub embedded_in_pe: bool,
}

fn extract_pe_pkcs7_slice(bytes: &[u8]) -> Option<&[u8]> {
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return None;
    }
    let pe_off = u32::from_le_bytes(bytes[0x3C..0x40].try_into().ok()?) as usize;
    if pe_off + 24 + 96 > bytes.len() || &bytes[pe_off..pe_off + 4] != b"PE\0\0" {
        return None;
    }
    let opt = pe_off + 24;
    let magic = u16::from_le_bytes(bytes[opt..opt + 2].try_into().ok()?);
    let dd_off = match magic {
        0x10B => opt + 96,
        0x20B => opt + 112,
        _ => return None,
    };
    let sec_dir_off = dd_off + 4 * 8;
    if sec_dir_off + 8 > bytes.len() {
        return None;
    }
    let sec_off = u32::from_le_bytes(bytes[sec_dir_off..sec_dir_off + 4].try_into().ok()?) as usize;
    let sec_sz = u32::from_le_bytes(bytes[sec_dir_off + 4..sec_dir_off + 8].try_into().ok()?) as usize;
    if sec_off == 0 || sec_sz < 8 || sec_off < sec_dir_off + 8 || sec_off.saturating_add(sec_sz) > bytes.len() {
        return None;
    }
    let dw_len = u32::from_le_bytes(bytes[sec_off..sec_off + 4].try_into().ok()?) as usize;
    let w_cert_type = u16::from_le_bytes(bytes[sec_off + 6..sec_off + 8].try_into().ok()?);
    if w_cert_type != WIN_CERT_TYPE_PKCS_SIGNED_DATA || dw_len < 8 || dw_len > sec_sz {
        return None;
    }
    Some(&bytes[sec_off + 8..sec_off + dw_len])
}

fn parse_pkcs7_signed_data(pkcs7_raw: &[u8], embedded_in_pe: bool) -> Option<ParsedPkcs7Info> {
    let (ci, _) = parse_der_elem(pkcs7_raw)?;
    if ci.tag != 0x30 {
        return None;
    }
    let ci_kids = der_children(ci.val)?;
    if ci_kids.len() < 2 || ci_kids[0].tag != 0x06 || ci_kids[0].val != OID_SIGNED_DATA || ci_kids[1].tag != 0xA0 {
        return None;
    }
    let (sd_seq, _) = parse_der_elem(ci_kids[1].val)?;
    if sd_seq.tag != 0x30 {
        return None;
    }
    let sd_kids = der_children(sd_seq.val)?;
    if sd_kids.len() < 4 {
        return None;
    }
    let mut certs_der = Vec::new();
    let mut raw_cert_count = 0usize;
    let mut raw_signer_count = 0usize;
    let mut signers = Vec::new();
    for &elem in &sd_kids[3..] {
        if elem.tag == 0xA0 {
            let remaining = MAX_CMS_CERTS.saturating_sub(raw_cert_count);
            let cert_elems = der_children_bounded(elem.val, remaining)?;
            raw_cert_count += cert_elems.len();
            for ce in cert_elems {
                if parse_x509_cert(ce).is_some() {
                    certs_der.push(ce.raw.to_vec());
                }
            }
        } else if elem.tag == 0x31 {
            let remaining = MAX_CMS_SIGNERS.saturating_sub(raw_signer_count);
            let si_list = der_children_bounded(elem.val, remaining)?;
            raw_signer_count += si_list.len();
            for si in si_list {
                if si.tag != 0x30 {
                    continue;
                }
                let Some(kids) = der_children(si.val) else { continue };
                if kids.len() < 5 || kids[1].tag != 0x30 {
                    continue;
                }
                let version = if kids[0].tag == 0x02 && !kids[0].val.is_empty() {
                    kids[0].val[kids[0].val.len() - 1] as u32
                } else {
                    1
                };
                let Some(ias) = der_children(kids[1].val) else { continue };
                if ias.len() < 2 || ias[0].tag != 0x30 || ias[1].tag != 0x02 {
                    continue;
                }
                let issuer_raw = ias[0].raw.to_vec();
                let serial_be = ias[1].val.to_vec();
                let hash_alg_oid = parse_alg_id_oid(kids[2]).unwrap_or(OID_SHA256);
                let mut pos = 3usize;
                if kids.get(pos).is_some_and(|e| e.tag == 0xA0) {
                    pos += 1;
                }
                let enc_alg_oid = kids
                    .get(pos)
                    .and_then(|&e| parse_alg_id_oid(e))
                    .unwrap_or(OID_RSA_ENCRYPTION);
                let encrypted_hash = kids
                    .get(pos + 1)
                    .filter(|e| e.tag == 0x04)
                    .map(|e| e.val.to_vec())
                    .unwrap_or_default();
                signers.push(ParsedSignerInfo {
                    version,
                    issuer_raw,
                    serial_be,
                    hash_alg_oid_str: decode_oid_str(hash_alg_oid),
                    enc_alg_oid_str: decode_oid_str(enc_alg_oid),
                    encrypted_hash,
                });
            }
        }
    }
    Some(ParsedPkcs7Info {
        certs_der,
        signers,
        embedded_in_pe,
    })
}

/// Extract embedded PE Authenticode PKCS#7 `SignedData` or parse a standalone DER PKCS#7 `SignedData` blob.
pub(crate) fn parse_pkcs7_or_pe_authenticode(bytes: &[u8]) -> Option<ParsedPkcs7Info> {
    if let Some(pkcs7) = extract_pe_pkcs7_slice(bytes) {
        return parse_pkcs7_signed_data(pkcs7, true);
    }
    parse_pkcs7_signed_data(bytes, false)
}

/// Extract an X.509 attribute string (e.g. CN = `2.5.4.3`, O = `2.5.4.10`, OU = `2.5.4.11`)
/// from a DER-encoded `Name` (`SEQUENCE OF RelativeDistinguishedName`).
pub(crate) fn extract_dn_attr(dn_der: &[u8], target_oid: &[u8]) -> Option<String> {
    let (seq, _) = parse_der_elem(dn_der)?;
    if seq.tag != 0x30 {
        return None;
    }
    for rdn in der_children(seq.val)? {
        if rdn.tag != 0x31 {
            continue;
        }
        for atv in der_children(rdn.val)? {
            if atv.tag != 0x30 {
                continue;
            }
            let kids = der_children(atv.val)?;
            if kids.len() >= 2 && kids[0].tag == 0x06 && kids[0].val == target_oid {
                return decode_asn1_string(kids[1]);
            }
        }
    }
    None
}

/// Format a DER-encoded X.509 `Name` into a comma-separated string (e.g. `CN=..., O=...`).
pub(crate) fn format_dn(dn_der: &[u8]) -> Option<String> {
    let (seq, _) = parse_der_elem(dn_der)?;
    if seq.tag != 0x30 {
        return None;
    }
    let mut parts = Vec::new();
    for rdn in der_children(seq.val)? {
        if rdn.tag != 0x31 {
            continue;
        }
        for atv in der_children(rdn.val)? {
            if atv.tag != 0x30 {
                continue;
            }
            let kids = der_children(atv.val)?;
            if kids.len() >= 2 && kids[0].tag == 0x06 {
                if let Some(val) = decode_asn1_string(kids[1]) {
                    let key = match kids[0].val {
                        [0x55, 0x04, 0x03] => "CN",
                        [0x55, 0x04, 0x06] => "C",
                        [0x55, 0x04, 0x07] => "L",
                        [0x55, 0x04, 0x08] => "S",
                        [0x55, 0x04, 0x0A] => "O",
                        [0x55, 0x04, 0x0B] => "OU",
                        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x01] => "E",
                        _ => continue,
                    };
                    parts.push(format!("{key}={val}"));
                }
            }
        }
    }
    (!parts.is_empty()).then(|| parts.join(", "))
}

/// Parse an ASCII dotted OID (e.g. `"2.5.4.3"`) into DER OID value bytes.
pub(crate) fn encode_oid_str(s: &str) -> Option<Vec<u8>> {
    let mut nums = s.split('.').map(|p| p.parse::<u32>().ok());
    let first = nums.next()??;
    let second = nums.next()??;
    if first > 2 || (first < 2 && second >= 40) {
        return None;
    }
    let mut out = vec![(first * 40 + second) as u8];
    for n in nums {
        let mut v = n?;
        let mut buf = [0u8; 5];
        let mut k = 0usize;
        buf[k] = (v & 0x7F) as u8;
        k += 1;
        v >>= 7;
        while v > 0 {
            buf[k] = ((v & 0x7F) as u8) | 0x80;
            k += 1;
            v >>= 7;
        }
        while k > 0 {
            k -= 1;
            out.push(buf[k]);
        }
    }
    Some(out)
}

fn decode_asn1_string(elem: DerElem<'_>) -> Option<String> {
    match elem.tag {
        0x0C | 0x13 | 0x16 | 0x14 => Some(String::from_utf8_lossy(elem.val).into_owned()),
        0x1E if elem.val.len() % 2 == 0 => {
            // BMPString (UTF-16BE)
            let units: Vec<u16> = elem.val.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
            Some(String::from_utf16_lossy(&units))
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Windows-ABI CERT_CONTEXT / CERT_INFO / CERT_CHAIN_CONTEXT / CRYPT_PROVIDER_SGNR
// ---------------------------------------------------------------------------

#[repr(C)]
pub(crate) struct CryptDataBlob {
    pub cb_data: u32,
    pub pb_data: *const u8,
}

#[repr(C)]
pub(crate) struct CryptAlgId {
    pub psz_obj_id: *const u8,
    pub parameters: CryptDataBlob,
}

#[repr(C)]
pub(crate) struct CryptBitBlob {
    pub cb_data: u32,
    pub pb_data: *const u8,
    pub c_unused_bits: u32,
}

#[repr(C)]
pub(crate) struct CertPublicKeyInfo {
    pub algorithm: CryptAlgId,
    pub public_key: CryptBitBlob,
}

#[repr(C)]
pub(crate) struct CertInfo {
    pub dw_version: u32,
    pub serial_number: CryptDataBlob,
    pub signature_algorithm: CryptAlgId,
    pub issuer: CryptDataBlob,
    pub not_before: u64,
    pub not_after: u64,
    pub subject: CryptDataBlob,
    pub subject_public_key_info: CertPublicKeyInfo,
    pub issuer_unique_id: CryptBitBlob,
    pub subject_unique_id: CryptBitBlob,
    pub c_extension: u32,
    pub rg_extension: *mut c_void,
}

#[repr(C)]
pub(crate) struct CertContext {
    pub dw_cert_encoding_type: u32,
    pub pb_cert_encoded: *const u8,
    pub cb_cert_encoded: u32,
    pub p_cert_info: *mut CertInfo,
    pub h_cert_store: *mut c_void,
}

#[repr(C)]
pub(crate) struct CertTrustStatus {
    pub dw_error_status: u32,
    pub dw_info_status: u32,
}

#[repr(C)]
pub(crate) struct CertChainElement {
    pub cb_size: u32,
    pub p_cert_context: *const CertContext,
    pub trust_status: CertTrustStatus,
    pub p_revocation_info: *mut c_void,
    pub p_issuance_usage: *mut c_void,
    pub p_application_usage: *mut c_void,
    pub pwsz_extended_error_info: *const u16,
}

#[repr(C)]
pub(crate) struct CertSimpleChain {
    pub cb_size: u32,
    pub trust_status: CertTrustStatus,
    pub c_element: u32,
    pub rgp_element: *const *const CertChainElement,
    pub p_trust_list_info: *mut c_void,
    pub f_has_revocation_freshness_time: i32,
    pub dw_revocation_freshness_time: u32,
}

#[repr(C)]
pub(crate) struct CertChainContext {
    pub cb_size: u32,
    pub trust_status: CertTrustStatus,
    pub c_chain: u32,
    pub rgp_chain: *const *const CertSimpleChain,
    pub c_lower_quality_chain_context: u32,
    pub rgp_lower_quality_chain_context: *mut c_void,
    pub f_has_revocation_freshness_time: i32,
    pub dw_revocation_freshness_time: u32,
    pub dw_create_flags: u32,
    pub chain_id: [u8; 16],
}

/// Heap-pinned Windows `CERT_CONTEXT` + `CERT_INFO` + backing DER slices.
///
/// All inner pointers reference heap allocations (`Box<[u8]>`, `Box<CertInfo>`) so moving
/// `OwnedCertContext` never invalidates the exposed `*const CertContext` pointer.
pub(crate) struct OwnedCertContext {
    der_blob: Box<[u8]>,
    _serial_le: Box<[u8]>,
    issuer_blob: Box<[u8]>,
    subject_blob: Box<[u8]>,
    _sig_oid_cstr: Box<[u8]>,
    _spki_oid_cstr: Box<[u8]>,
    _spki_alg_params: Box<[u8]>,
    _spki_key_blob: Box<[u8]>,
    spki_raw_der: Box<[u8]>,
    cert_info: Box<CertInfo>,
    cert_context: Box<CertContext>,
}

unsafe impl Send for OwnedCertContext {}
unsafe impl Sync for OwnedCertContext {}

impl OwnedCertContext {
    pub fn as_ptr(&self) -> *const CertContext {
        &*self.cert_context as *const CertContext
    }

    pub fn as_mut_ptr(&self) -> *mut CertContext {
        &*self.cert_context as *const CertContext as *mut CertContext
    }

    pub fn spki_ptr(&self) -> usize {
        (&self.cert_info.subject_public_key_info) as *const CertPublicKeyInfo as usize
    }

    pub fn spki_raw_der(&self) -> &[u8] {
        &self.spki_raw_der
    }

    pub fn raw_der(&self) -> &[u8] {
        &self.der_blob
    }

    pub fn is_self_signed(&self) -> bool {
        !self.issuer_blob.is_empty() && self.issuer_blob == self.subject_blob
    }
}

/// Parse a DER-encoded X.509 certificate into a standalone, Win64 ABI-exact `OwnedCertContext`.
pub(crate) fn build_owned_cert_context(der: &[u8], h_cert_store: *mut c_void) -> Option<OwnedCertContext> {
    let (elem, _) = parse_der_elem(der)?;
    let parsed = parse_x509_cert(elem)?;

    let der_blob = parsed.raw_der.to_vec().into_boxed_slice();
    // Windows CERT_INFO.SerialNumber is little-endian bytes.
    let mut ser_le = parsed.serial_val.to_vec();
    ser_le.reverse();
    let serial_le = ser_le.into_boxed_slice();
    let issuer_blob = parsed.issuer_raw.to_vec().into_boxed_slice();
    let subject_blob = parsed.subject_raw.to_vec().into_boxed_slice();

    let mut sig_oid_v = decode_oid_str(parsed.sig_alg_oid).into_bytes();
    sig_oid_v.push(0);
    let sig_oid_cstr = sig_oid_v.into_boxed_slice();

    let mut spki_oid_v = decode_oid_str(parsed.spki_alg_oid).into_bytes();
    spki_oid_v.push(0);
    let spki_oid_cstr = spki_oid_v.into_boxed_slice();

    let spki_alg_params = parsed.spki_alg_params.to_vec().into_boxed_slice();
    let spki_key_blob = parsed.spki_key_bytes.to_vec().into_boxed_slice();
    let spki_raw_der = parsed.spki_raw.to_vec().into_boxed_slice();

    let params_ptr = if spki_alg_params.is_empty() {
        std::ptr::null()
    } else {
        spki_alg_params.as_ptr()
    };

    let cert_info = Box::new(CertInfo {
        dw_version: parsed.version,
        serial_number: CryptDataBlob {
            cb_data: serial_le.len() as u32,
            pb_data: serial_le.as_ptr(),
        },
        signature_algorithm: CryptAlgId {
            psz_obj_id: sig_oid_cstr.as_ptr(),
            parameters: CryptDataBlob { cb_data: 0, pb_data: std::ptr::null() },
        },
        issuer: CryptDataBlob {
            cb_data: issuer_blob.len() as u32,
            pb_data: issuer_blob.as_ptr(),
        },
        not_before: parsed.not_before,
        not_after: parsed.not_after,
        subject: CryptDataBlob {
            cb_data: subject_blob.len() as u32,
            pb_data: subject_blob.as_ptr(),
        },
        subject_public_key_info: CertPublicKeyInfo {
            algorithm: CryptAlgId {
                psz_obj_id: spki_oid_cstr.as_ptr(),
                parameters: CryptDataBlob {
                    cb_data: spki_alg_params.len() as u32,
                    pb_data: params_ptr,
                },
            },
            public_key: CryptBitBlob {
                cb_data: spki_key_blob.len() as u32,
                pb_data: spki_key_blob.as_ptr(),
                c_unused_bits: parsed.spki_unused_bits,
            },
        },
        issuer_unique_id: CryptBitBlob { cb_data: 0, pb_data: std::ptr::null(), c_unused_bits: 0 },
        subject_unique_id: CryptBitBlob { cb_data: 0, pb_data: std::ptr::null(), c_unused_bits: 0 },
        c_extension: 0,
        rg_extension: std::ptr::null_mut(),
    });

    let cert_context = Box::new(CertContext {
        dw_cert_encoding_type: X509_ASN_ENCODING,
        pb_cert_encoded: der_blob.as_ptr(),
        cb_cert_encoded: der_blob.len() as u32,
        p_cert_info: &*cert_info as *const CertInfo as *mut CertInfo,
        h_cert_store,
    });

    Some(OwnedCertContext {
        der_blob,
        _serial_le: serial_le,
        issuer_blob,
        subject_blob,
        _sig_oid_cstr: sig_oid_cstr,
        _spki_oid_cstr: spki_oid_cstr,
        _spki_alg_params: spki_alg_params,
        _spki_key_blob: spki_key_blob,
        spki_raw_der,
        cert_info,
        cert_context,
    })
}

#[repr(C)]
struct CryptProviderCert {
    cb_struct: u32,
    p_cert: *const CertContext,
    f_commercial: i32,
    f_trusted_root: i32,
    f_self_signed: i32,
    f_test_cert: i32,
    dw_revoked_reason: u32,
    dw_confidence: u32,
    dw_error: u32,
    p_trust_list_context: *mut c_void,
    f_trust_list_signer_cert: i32,
    p_ctl_context: *mut c_void,
    dw_ctl_error: u32,
    f_is_cyclic: i32,
    p_chain_element: *const CertChainElement,
}

#[repr(C)]
struct CryptProviderSgnr {
    cb_struct: u32,
    sft_verify_as_of: [u32; 2],
    cs_cert_chain: u32,
    pas_cert_chain: *const CryptProviderCert,
    dw_signer_type: u32,
    ps_signer: *mut c_void,
    dw_error: u32,
    cs_counter_signers: u32,
    pas_counter_signers: *mut c_void,
    p_chain_context: *const CertChainContext,
}

#[repr(C)]
pub(crate) struct CmsgSignerInfo {
    pub dw_version: u32,
    pub issuer: CryptDataBlob,
    pub serial_number: CryptDataBlob,
    pub hash_algorithm: CryptAlgId,
    pub hash_encryption_algorithm: CryptAlgId,
    pub encrypted_hash: CryptDataBlob,
    pub auth_attrs_count: u32,
    pub auth_attrs_ptr: *mut c_void,
    pub unauth_attrs_count: u32,
    pub unauth_attrs_ptr: *mut c_void,
}

#[repr(C)]
struct CryptProviderData {
    cb_struct: u32,
    p_wintrust_data: *mut c_void,
    f_opened_file: i32,
    h_wnd_parent: *mut c_void,
    pg_action_id: *const u8,
    h_prov: usize,
    dw_error: u32,
    dw_reg_security_settings: u32,
    dw_reg_policy_settings: u32,
    ps_pfns: *mut c_void,
    cdw_trust_step_errors: u32,
    padw_trust_step_errors: *mut u32,
    ch_stores: u32,
    pah_stores: *mut c_void,
    dw_encoding: u32,
    h_msg: *mut c_void,
    cs_signers: u32,
    pas_signers: *const CryptProviderSgnr,
    cs_prov_priv_data: u32,
    pas_prov_priv_data: *mut c_void,
    dw_subject_choice: u32,
    p_pdsip: *mut c_void,
    psz_usage_oid: *const u8,
    f_recall_with_state: i32,
    sft_system_time: [u32; 2],
    psz_ctl_signer_usage_oid: *const u8,
    dw_prov_flags: u32,
    dw_final_error: u32,
    p_request_usage: *mut c_void,
    dw_trust_pub_settings: u32,
    dw_ui_state_flags: u32,
}

struct ProviderState {
    _der_blobs: Vec<Box<[u8]>>,
    _serial_blobs: Vec<Box<[u8]>>,
    _issuer_blobs: Vec<Box<[u8]>>,
    _subject_blobs: Vec<Box<[u8]>>,
    _oid_cstrs: Vec<Box<[u8]>>,
    _spki_key_blobs: Vec<Box<[u8]>>,
    _enc_hash_blob: Box<[u8]>,
    _cert_infos: Vec<Box<CertInfo>>,
    _cert_contexts: Vec<Box<CertContext>>,
    _chain_elements: Vec<Box<CertChainElement>>,
    _chain_element_ptrs: Box<[*const CertChainElement]>,
    _simple_chain: Box<CertSimpleChain>,
    _simple_chain_ptrs: Box<[*const CertSimpleChain]>,
    _chain_context: Box<CertChainContext>,
    _provider_certs: Box<[CryptProviderCert]>,
    _cmsg_signer: Box<CmsgSignerInfo>,
    _signer: Box<CryptProviderSgnr>,
    provider_data: Box<CryptProviderData>,
}

unsafe impl Send for ProviderState {}

static STATE_MAP: LazyLock<Mutex<HashMap<usize, Box<ProviderState>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn alloc_provider_state(verified: VerifiedAuthenticode, wintrust_data: *mut c_void) -> *mut c_void {
    let n = verified.certs.len();
    let mut der_blobs = Vec::with_capacity(n);
    let mut serial_blobs = Vec::with_capacity(n);
    let mut issuer_blobs = Vec::with_capacity(n);
    let mut subject_blobs = Vec::with_capacity(n);
    let mut oid_cstrs = Vec::with_capacity(n * 2 + 2);
    let mut spki_key_blobs = Vec::with_capacity(n);
    let mut cert_infos = Vec::with_capacity(n);
    let mut cert_contexts = Vec::with_capacity(n);
    let mut chain_elements = Vec::with_capacity(n);
    let mut chain_element_ptrs = Vec::with_capacity(n);
    let mut provider_certs_vec = Vec::with_capacity(n);

    let mut push_oid = |s: &str| -> *const u8 {
        let mut v = s.as_bytes().to_vec();
        v.push(0);
        let b = v.into_boxed_slice();
        let p = b.as_ptr();
        oid_cstrs.push(b);
        p
    };

    for (idx, c) in verified.certs.into_iter().enumerate() {
        let boxed_der = c.raw_der.into_boxed_slice();
        // Windows CERT_INFO.SerialNumber is little-endian bytes.
        let mut ser_le = c.serial_val;
        ser_le.reverse();
        let boxed_ser = ser_le.into_boxed_slice();
        let boxed_iss = c.issuer_raw.into_boxed_slice();
        let boxed_sub = c.subject_raw.into_boxed_slice();
        let boxed_spki = c.spki_key_bytes.into_boxed_slice();
        let sig_oid_ptr = push_oid(&c.sig_alg_oid_str);
        let spki_oid_ptr = push_oid(&c.spki_alg_oid_str);

        let ci = Box::new(CertInfo {
            dw_version: c.version,
            serial_number: CryptDataBlob { cb_data: boxed_ser.len() as u32, pb_data: boxed_ser.as_ptr() },
            signature_algorithm: CryptAlgId {
                psz_obj_id: sig_oid_ptr,
                parameters: CryptDataBlob { cb_data: 0, pb_data: std::ptr::null() },
            },
            issuer: CryptDataBlob { cb_data: boxed_iss.len() as u32, pb_data: boxed_iss.as_ptr() },
            not_before: c.not_before,
            not_after: c.not_after,
            subject: CryptDataBlob { cb_data: boxed_sub.len() as u32, pb_data: boxed_sub.as_ptr() },
            subject_public_key_info: CertPublicKeyInfo {
                algorithm: CryptAlgId {
                    psz_obj_id: spki_oid_ptr,
                    parameters: CryptDataBlob { cb_data: 0, pb_data: std::ptr::null() },
                },
                public_key: CryptBitBlob {
                    cb_data: boxed_spki.len() as u32,
                    pb_data: boxed_spki.as_ptr(),
                    c_unused_bits: c.spki_unused_bits,
                },
            },
            issuer_unique_id: CryptBitBlob { cb_data: 0, pb_data: std::ptr::null(), c_unused_bits: 0 },
            subject_unique_id: CryptBitBlob { cb_data: 0, pb_data: std::ptr::null(), c_unused_bits: 0 },
            c_extension: 0,
            rg_extension: std::ptr::null_mut(),
        });
        let cc = Box::new(CertContext {
            dw_cert_encoding_type: X509_ASN_ENCODING,
            pb_cert_encoded: boxed_der.as_ptr(),
            cb_cert_encoded: boxed_der.len() as u32,
            p_cert_info: &*ci as *const CertInfo as *mut CertInfo,
            h_cert_store: std::ptr::null_mut(),
        });
        let info_status = if c.self_signed { 0x0000_0108 } else { 0x0000_0100 };
        let ce = Box::new(CertChainElement {
            cb_size: std::mem::size_of::<CertChainElement>() as u32,
            p_cert_context: &*cc as *const CertContext,
            trust_status: CertTrustStatus { dw_error_status: 0, dw_info_status: info_status },
            p_revocation_info: std::ptr::null_mut(),
            p_issuance_usage: std::ptr::null_mut(),
            p_application_usage: std::ptr::null_mut(),
            pwsz_extended_error_info: std::ptr::null(),
        });
        let ce_ptr = &*ce as *const CertChainElement;
        let pc = CryptProviderCert {
            cb_struct: std::mem::size_of::<CryptProviderCert>() as u32,
            p_cert: &*cc as *const CertContext,
            f_commercial: 1,
            f_trusted_root: i32::from(c.self_signed || idx + 1 == n),
            f_self_signed: i32::from(c.self_signed),
            f_test_cert: 0,
            dw_revoked_reason: 0,
            dw_confidence: 0x1111_0000,
            dw_error: 0,
            p_trust_list_context: std::ptr::null_mut(),
            f_trust_list_signer_cert: 0,
            p_ctl_context: std::ptr::null_mut(),
            dw_ctl_error: 0,
            f_is_cyclic: 0,
            p_chain_element: ce_ptr,
        };
        der_blobs.push(boxed_der);
        serial_blobs.push(boxed_ser);
        issuer_blobs.push(boxed_iss);
        subject_blobs.push(boxed_sub);
        spki_key_blobs.push(boxed_spki);
        cert_infos.push(ci);
        cert_contexts.push(cc);
        chain_elements.push(ce);
        chain_element_ptrs.push(ce_ptr);
        provider_certs_vec.push(pc);
    }

    let hash_oid_ptr = push_oid(&verified.signer.hash_alg_oid_str);
    let enc_oid_ptr = push_oid(&verified.signer.enc_alg_oid_str);
    let enc_hash_blob = verified.signer.encrypted_hash.into_boxed_slice();
    let cmsg_signer = Box::new(CmsgSignerInfo {
        dw_version: 1,
        issuer: CryptDataBlob {
            cb_data: issuer_blobs[0].len() as u32,
            pb_data: issuer_blobs[0].as_ptr(),
        },
        serial_number: CryptDataBlob {
            cb_data: serial_blobs[0].len() as u32,
            pb_data: serial_blobs[0].as_ptr(),
        },
        hash_algorithm: CryptAlgId {
            psz_obj_id: hash_oid_ptr,
            parameters: CryptDataBlob { cb_data: 0, pb_data: std::ptr::null() },
        },
        hash_encryption_algorithm: CryptAlgId {
            psz_obj_id: enc_oid_ptr,
            parameters: CryptDataBlob { cb_data: 0, pb_data: std::ptr::null() },
        },
        encrypted_hash: CryptDataBlob {
            cb_data: enc_hash_blob.len() as u32,
            pb_data: enc_hash_blob.as_ptr(),
        },
        auth_attrs_count: 0,
        auth_attrs_ptr: std::ptr::null_mut(),
        unauth_attrs_count: 0,
        unauth_attrs_ptr: std::ptr::null_mut(),
    });

    let chain_element_ptrs = chain_element_ptrs.into_boxed_slice();
    let simple_chain = Box::new(CertSimpleChain {
        cb_size: std::mem::size_of::<CertSimpleChain>() as u32,
        trust_status: CertTrustStatus { dw_error_status: 0, dw_info_status: 0 },
        c_element: n as u32,
        rgp_element: chain_element_ptrs.as_ptr(),
        p_trust_list_info: std::ptr::null_mut(),
        f_has_revocation_freshness_time: 0,
        dw_revocation_freshness_time: 0,
    });
    let simple_chain_ptrs = vec![&*simple_chain as *const CertSimpleChain].into_boxed_slice();
    let chain_context = Box::new(CertChainContext {
        cb_size: std::mem::size_of::<CertChainContext>() as u32,
        trust_status: CertTrustStatus { dw_error_status: 0, dw_info_status: 0 },
        c_chain: 1,
        rgp_chain: simple_chain_ptrs.as_ptr(),
        c_lower_quality_chain_context: 0,
        rgp_lower_quality_chain_context: std::ptr::null_mut(),
        f_has_revocation_freshness_time: 0,
        dw_revocation_freshness_time: 0,
        dw_create_flags: 0,
        chain_id: [0; 16],
    });
    let provider_certs = provider_certs_vec.into_boxed_slice();
    let signer = Box::new(CryptProviderSgnr {
        cb_struct: std::mem::size_of::<CryptProviderSgnr>() as u32,
        sft_verify_as_of: [0, 0],
        cs_cert_chain: n as u32,
        pas_cert_chain: provider_certs.as_ptr(),
        dw_signer_type: 0,
        ps_signer: &*cmsg_signer as *const CmsgSignerInfo as *mut c_void,
        dw_error: 0,
        cs_counter_signers: 0,
        pas_counter_signers: std::ptr::null_mut(),
        p_chain_context: &*chain_context as *const CertChainContext,
    });
    let provider_data = Box::new(CryptProviderData {
        cb_struct: std::mem::size_of::<CryptProviderData>() as u32,
        p_wintrust_data: wintrust_data,
        f_opened_file: 1,
        h_wnd_parent: std::ptr::null_mut(),
        pg_action_id: WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
        h_prov: 0,
        dw_error: 0,
        dw_reg_security_settings: 0,
        dw_reg_policy_settings: 0,
        ps_pfns: std::ptr::null_mut(),
        cdw_trust_step_errors: 0,
        padw_trust_step_errors: std::ptr::null_mut(),
        ch_stores: 0,
        pah_stores: std::ptr::null_mut(),
        dw_encoding: X509_ASN_ENCODING | 0x0001_0000,
        h_msg: std::ptr::null_mut(),
        cs_signers: 1,
        pas_signers: &*signer as *const CryptProviderSgnr,
        cs_prov_priv_data: 0,
        pas_prov_priv_data: std::ptr::null_mut(),
        dw_subject_choice: WTD_CHOICE_FILE,
        p_pdsip: std::ptr::null_mut(),
        psz_usage_oid: std::ptr::null(),
        f_recall_with_state: 0,
        sft_system_time: [0, 0],
        psz_ctl_signer_usage_oid: std::ptr::null(),
        dw_prov_flags: 0,
        dw_final_error: 0,
        p_request_usage: std::ptr::null_mut(),
        dw_trust_pub_settings: 0,
        dw_ui_state_flags: 0,
    });
    let state = Box::new(ProviderState {
        _der_blobs: der_blobs,
        _serial_blobs: serial_blobs,
        _issuer_blobs: issuer_blobs,
        _subject_blobs: subject_blobs,
        _oid_cstrs: oid_cstrs,
        _spki_key_blobs: spki_key_blobs,
        _enc_hash_blob: enc_hash_blob,
        _cert_infos: cert_infos,
        _cert_contexts: cert_contexts,
        _chain_elements: chain_elements,
        _chain_element_ptrs: chain_element_ptrs,
        _simple_chain: simple_chain,
        _simple_chain_ptrs: simple_chain_ptrs,
        _chain_context: chain_context,
        _provider_certs: provider_certs,
        _cmsg_signer: cmsg_signer,
        _signer: signer,
        provider_data,
    });
    let handle = &*state as *const ProviderState as usize;
    STATE_MAP.lock().unwrap().insert(handle, state);
    handle as *mut c_void
}

/// Result of cryptographically verifying a standalone DER PKCS#7 SignedData blob.
struct StandalonePkcs7Result {
    verified: VerifiedAuthenticode,
    encap_content_type_oid: Vec<u8>,
    encap_content_der: Vec<u8>,
    all_certs_der: Vec<Vec<u8>>,
}

/// Verify a standalone DER PKCS#7 SignedData blob (and optional detached payload) for
/// `WTD_CHOICE_BLOB` / `WTD_CHOICE_CATALOG`.
///
/// NOTE: Callers MUST also validate the signer certificate chain via
/// `verify_pkcs7_signer_chain` before reporting `ERROR_SUCCESS`.
fn verify_standalone_pkcs7(
    pkcs7_raw: &[u8],
    detached_content: Option<&[u8]>,
) -> Result<StandalonePkcs7Result, i32> {
    let (ci, _) = parse_der_elem(pkcs7_raw).ok_or(TRUST_E_NOSIGNATURE)?;
    if ci.tag != 0x30 {
        return Err(TRUST_E_NOSIGNATURE);
    }
    let ci_kids = der_children(ci.val).ok_or(TRUST_E_NOSIGNATURE)?;
    if ci_kids.len() < 2
        || ci_kids[0].tag != 0x06
        || ci_kids[0].val != OID_SIGNED_DATA
        || ci_kids[1].tag != 0xA0
    {
        return Err(TRUST_E_NOSIGNATURE);
    }
    let (sd_seq, _) = parse_der_elem(ci_kids[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
    if sd_seq.tag != 0x30 {
        return Err(TRUST_E_BAD_DIGEST);
    }
    let sd_kids = der_children(sd_seq.val).ok_or(TRUST_E_BAD_DIGEST)?;
    if sd_kids.len() < 4 || sd_kids[2].tag != 0x30 {
        return Err(TRUST_E_BAD_DIGEST);
    }

    let eci_kids = der_children(sd_kids[2].val).ok_or(TRUST_E_BAD_DIGEST)?;
    let encap_content_type_oid = eci_kids
        .first()
        .filter(|e| e.tag == 0x06)
        .map(|e| e.val.to_vec())
        .unwrap_or_default();
    let mut encap_content_der = Vec::new();
    let content_bytes: Vec<u8> = if let Some(d) = detached_content {
        d.to_vec()
    } else if let Some(ec) = eci_kids.get(1).filter(|e| e.tag == 0xA0) {
        if let Some((inner, _)) = parse_der_elem(ec.val) {
            // In RFC 5652 CMS, eContent is wrapped in OCTET STRING (0x04) whose value octets are
            // both the hashed content and the inner DER (e.g. CertificateTrustList). In legacy
            // PKCS#7 (e.g. makecat), structured eContent is placed directly as a SEQUENCE (0x30)
            // where messageDigest hashes inner.val while the DER object is inner.raw.
            encap_content_der = if inner.tag == 0x04 {
                inner.val.to_vec()
            } else {
                inner.raw.to_vec()
            };
            inner.val.to_vec()
        } else {
            encap_content_der = ec.val.to_vec();
            ec.val.to_vec()
        }
    } else {
        Vec::new()
    };

    let mut certs: Vec<ParsedCert<'_>> = Vec::new();
    let mut raw_cert_count = 0usize;
    let mut signer_infos_elem: Option<DerElem<'_>> = None;
    for &elem in &sd_kids[3..] {
        if elem.tag == 0xA0 {
            let remaining = MAX_CMS_CERTS.saturating_sub(raw_cert_count);
            let cert_elems = der_children_bounded(elem.val, remaining).ok_or(TRUST_E_BAD_DIGEST)?;
            raw_cert_count += cert_elems.len();
            for ce in cert_elems {
                if let Some(c) = parse_x509_cert(ce) {
                    certs.push(c);
                }
            }
        } else if elem.tag == 0x31 {
            signer_infos_elem = Some(elem);
        }
    }
    let si_set = signer_infos_elem.ok_or(TRUST_E_BAD_DIGEST)?;
    let si_list = der_children_bounded(si_set.val, MAX_CMS_SIGNERS).ok_or(TRUST_E_BAD_DIGEST)?;
    if si_list.is_empty() || certs.is_empty() {
        return Err(TRUST_E_BAD_DIGEST);
    }

    // Index parsed certificates by (issuer DER, normalized serial) for O(1) signer matching.
    let mut certs_by_ias: HashMap<(&[u8], &[u8]), usize> = HashMap::with_capacity(certs.len());
    for (idx, c) in certs.iter().enumerate() {
        certs_by_ias
            .entry((c.issuer_raw, strip_der_int_sign(c.serial_val)))
            .or_insert(idx);
    }

    let mut verified_signer: Option<(usize, VerifiedSigner)> = None;
    for si in si_list {
        if si.tag != 0x30 {
            continue;
        }
        let kids = der_children(si.val).ok_or(TRUST_E_BAD_DIGEST)?;
        if kids.len() < 5 || kids[1].tag != 0x30 {
            continue;
        }
        let ias = der_children(kids[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
        if ias.len() < 2 || ias[0].tag != 0x30 || ias[1].tag != 0x02 {
            continue;
        }
        let Some(&cert_idx) = certs_by_ias.get(&(ias[0].raw, strip_der_int_sign(ias[1].val))) else {
            continue;
        };
        let Some(ref rsa_pub) = certs[cert_idx].rsa_pub else {
            continue;
        };
        let hash_alg_oid = parse_alg_id_oid(kids[2]).ok_or(TRUST_E_BAD_DIGEST)?;
        let si_digest = digest_from_oid(hash_alg_oid).ok_or(TRUST_E_BAD_DIGEST)?;
        let mut pos = 3usize;
        let auth_attrs = if kids.get(pos).is_some_and(|e| e.tag == 0xA0) {
            pos += 1;
            Some(kids[pos - 1])
        } else {
            None
        };
        let enc_alg = *kids.get(pos).ok_or(TRUST_E_BAD_DIGEST)?;
        let enc_alg_oid = parse_alg_id_oid(enc_alg).unwrap_or(OID_RSA_ENCRYPTION);
        let enc_digest = *kids.get(pos + 1).ok_or(TRUST_E_BAD_DIGEST)?;
        if enc_digest.tag != 0x04 {
            return Err(TRUST_E_BAD_DIGEST);
        }

        if let Some(aa) = auth_attrs {
            // Per RFC 5652 §5.3 & §5.6: require exactly one content-type attribute matching
            // encapContentInfo.eContentType and exactly one message-digest matching hash(content).
            let mut md_count = 0usize;
            let mut ct_count = 0usize;
            let expected_md = si_digest.hash(&content_bytes);
            for attr in der_children_bounded(aa.val, MAX_CMS_AUTH_ATTRS).ok_or(TRUST_E_BAD_DIGEST)? {
                if attr.tag != 0x30 {
                    continue;
                }
                let ak = der_children(attr.val).ok_or(TRUST_E_BAD_DIGEST)?;
                if ak.len() >= 2 && ak[0].tag == 0x06 && ak[1].tag == 0x31 {
                    if ak[0].val == OID_MESSAGE_DIGEST {
                        let vals = der_children(ak[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
                        if vals.len() != 1
                            || vals[0].tag != 0x04
                            || vals[0].val != expected_md.as_slice()
                        {
                            return Err(TRUST_E_BAD_DIGEST);
                        }
                        md_count += 1;
                    } else if ak[0].val == OID_CONTENT_TYPE {
                        let vals = der_children(ak[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
                        if vals.len() != 1
                            || vals[0].tag != 0x06
                            || vals[0].val != encap_content_type_oid.as_slice()
                        {
                            return Err(TRUST_E_BAD_DIGEST);
                        }
                        ct_count += 1;
                    }
                }
            }
            if md_count != 1 || ct_count != 1 {
                return Err(TRUST_E_BAD_DIGEST);
            }
            let mut set_der = Vec::with_capacity(aa.raw.len());
            set_der.push(0x31);
            set_der.extend_from_slice(&aa.raw[1..]);
            let attr_hash = si_digest.hash(&set_der);
            if !rsa_pub.verify_pkcs1(Some(si_digest), &attr_hash, enc_digest.val) {
                return Err(TRUST_E_BAD_DIGEST);
            }
        } else {
            // Per RFC 5652 §5.3, signedAttrs MUST be present whenever eContentType != id-data.
            if encap_content_type_oid.as_slice() != OID_DATA {
                return Err(TRUST_E_BAD_DIGEST);
            }
            let content_hash = si_digest.hash(&content_bytes);
            if !rsa_pub.verify_pkcs1(Some(si_digest), &content_hash, enc_digest.val) {
                return Err(TRUST_E_BAD_DIGEST);
            }
        }
        verified_signer = Some((
            cert_idx,
            VerifiedSigner {
                hash_alg_oid_str: decode_oid_str(hash_alg_oid),
                enc_alg_oid_str: decode_oid_str(enc_alg_oid),
                encrypted_hash: enc_digest.val.to_vec(),
            },
        ));
        break;
    }

    let Some((signer_idx, signer)) = verified_signer else {
        return Err(TRUST_E_BAD_DIGEST);
    };

    // Preserve ONLY the signer certificate followed by cryptographically verified issuer chain
    // links present in SignedData — never pass unrelated trailing certificates into the chain.
    let mut chain_indices = vec![signer_idx];
    let mut cur = signer_idx;
    for _ in 0..8 {
        if certs[cur].issuer_raw == certs[cur].subject_raw
            && super::crypt32::verify_cert_signed_by_der(certs[cur].raw_der, certs[cur].raw_der)
        {
            break;
        }
        // Prefer a cryptographically verified self-signed root if multiple certs share the DN.
        let next_idx = certs
            .iter()
            .enumerate()
            .position(|(i, c)| {
                !chain_indices.contains(&i)
                    && c.subject_raw == certs[cur].issuer_raw
                    && c.issuer_raw == c.subject_raw
                    && super::crypt32::verify_cert_signed_by_der(certs[cur].raw_der, c.raw_der)
            })
            .or_else(|| {
                certs.iter().enumerate().position(|(i, c)| {
                    !chain_indices.contains(&i)
                        && c.subject_raw == certs[cur].issuer_raw
                        && super::crypt32::verify_cert_signed_by_der(certs[cur].raw_der, c.raw_der)
                })
            });
        if let Some(next_idx) = next_idx {
            chain_indices.push(next_idx);
            cur = next_idx;
        } else {
            break;
        }
    }
    let chain_certs: Vec<VerifiedCert> = chain_indices
        .into_iter()
        .map(|idx| {
            let c = &certs[idx];
            VerifiedCert {
                version: c.version,
                raw_der: c.raw_der.to_vec(),
                serial_val: c.serial_val.to_vec(),
                sig_alg_oid_str: decode_oid_str(c.sig_alg_oid),
                issuer_raw: c.issuer_raw.to_vec(),
                not_before: c.not_before,
                not_after: c.not_after,
                subject_raw: c.subject_raw.to_vec(),
                spki_alg_oid_str: decode_oid_str(c.spki_alg_oid),
                spki_key_bytes: c.spki_key_bytes.to_vec(),
                spki_unused_bits: c.spki_unused_bits,
                self_signed: c.issuer_raw == c.subject_raw,
            }
        })
        .collect();
    let all_certs_der: Vec<Vec<u8>> = chain_certs.iter().map(|c| c.raw_der.clone()).collect();
    Ok(StandalonePkcs7Result {
        verified: VerifiedAuthenticode { certs: chain_certs, signer },
        encap_content_type_oid,
        encap_content_der,
        all_certs_der,
    })
}

/// Validate the signer certificate of a cryptographically verified PKCS#7 message against
/// Windows trust chain & policy rules, supplying all embedded PKCS#7 certificates as
/// intermediate candidates.
/// Map `WINTRUST_DATA.fdwRevocationChecks` (+0x1C) and `WINTRUST_DATA.dwProvFlags` (+0x48)
/// into the corresponding `CertGetCertificateChain` `dwFlags` revocation bits.
fn map_wintrust_revocation_flags(fdw_revocation_checks: u32, dw_prov_flags: u32) -> u32 {
    let mut chain_flags = 0u32;
    if (dw_prov_flags & WTD_REVOCATION_CHECK_END_CERT) != 0 {
        chain_flags |= CERT_CHAIN_REVOCATION_CHECK_END_CERT;
    }
    if (dw_prov_flags & WTD_REVOCATION_CHECK_CHAIN) != 0 {
        chain_flags |= CERT_CHAIN_REVOCATION_CHECK_CHAIN;
    }
    if (dw_prov_flags & WTD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT) != 0 {
        chain_flags |= CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT;
    }
    if chain_flags == 0
        && (dw_prov_flags & WTD_REVOCATION_CHECK_NONE) == 0
        && (fdw_revocation_checks == WTD_REVOKE_WHOLECHAIN
            || fdw_revocation_checks != WTD_REVOKE_NONE)
    {
        // WTD_REVOKE_WHOLECHAIN (or any non-zero fdwRevocationChecks) checks the whole chain
        // excluding the trusted root, matching Windows WinTrust semantics.
        chain_flags |= CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT;
    }
    if (dw_prov_flags & WTD_CACHE_ONLY_URL_RETRIEVAL) != 0 {
        chain_flags |=
            CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL | CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY;
    }
    chain_flags
}

#[derive(Clone, Copy)]
struct GenericChainPolicy {
    h_chain_engine: *mut c_void,
    p_chain_para: *const c_void,
    dw_flags: u32,
    pfn_policy_callback: usize,
    pv_policy_arg: *mut c_void,
    wintrust_data: *mut c_void,
    action_guid: *const u8,
    prov_flags: u32,
    subject_choice: u32,
}

#[repr(C)]
struct WtdGenericChainPolicySignerInfo {
    cb_size: u32,
    _pad0: u32,
    p_chain_context: *const c_void,
    dw_signer_type: u32,
    _pad1: u32,
    p_msg_signer_info: *const c_void,
    dw_error: u32,
    c_counter_signer: u32,
    rgp_counter_signer: *const *const WtdGenericChainPolicySignerInfo,
}

unsafe fn verify_pkcs7_signer_chain(
    verified: &VerifiedAuthenticode,
    all_certs_der: &[Vec<u8>],
    policy_oid: usize,
    policy_extra: *const c_void,
    chain_flags: u32,
    generic_chain: Option<GenericChainPolicy>,
) -> i32 {
    let Some(leaf) = verified.certs.first() else {
        return TRUST_E_NOSIGNATURE;
    };
    let leaf_ctx = super::crypt32::CertCreateCertificateContext(
        X509_ASN_ENCODING,
        leaf.raw_der.as_ptr(),
        leaf.raw_der.len() as u32,
    );
    if leaf_ctx.is_null() {
        return CERT_E_CHAINING;
    }
    let extra_store = super::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
    if !extra_store.is_null() {
        for der in all_certs_der {
            let _ = super::crypt32::CertAddEncodedCertificateToStore(
                extra_store,
                X509_ASN_ENCODING,
                der.as_ptr(),
                der.len() as u32,
                4,
                std::ptr::null_mut(),
            );
        }
    }
    let stores = [extra_store];
    let rc = verify_cert_via_chain_policy(
        leaf_ctx,
        if extra_store.is_null() { 0 } else { 1 },
        if extra_store.is_null() { std::ptr::null() } else { stores.as_ptr() },
        std::ptr::null(),
        policy_oid,
        policy_extra,
        chain_flags,
        generic_chain,
    );
    if !extra_store.is_null() {
        super::crypt32::CertCloseStore(extra_store, 0);
    }
    super::crypt32::CertFreeCertificateContext(leaf_ctx);
    rc
}

/// Compute the PE Authenticode digest of `bytes` using `alg`, or `None` if `bytes` is not a PE.
fn compute_pe_authenticode_digest(bytes: &[u8], alg: Digest) -> Option<Vec<u8>> {
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return None;
    }
    let pe_off = u32::from_le_bytes(bytes[0x3C..0x40].try_into().ok()?) as usize;
    if pe_off + 24 + 96 > bytes.len() || &bytes[pe_off..pe_off + 4] != b"PE\0\0" {
        return None;
    }
    let opt = pe_off + 24;
    let magic = u16::from_le_bytes(bytes[opt..opt + 2].try_into().ok()?);
    let dd_off = match magic {
        0x10B => opt + 96,
        0x20B => opt + 112,
        _ => return None,
    };
    let checksum_off = opt + 64;
    let sec_dir_off = dd_off + 4 * 8;
    if sec_dir_off + 8 > bytes.len() {
        return None;
    }
    let sec_off = u32::from_le_bytes(bytes[sec_dir_off..sec_dir_off + 4].try_into().ok()?) as usize;
    let sec_sz = u32::from_le_bytes(bytes[sec_dir_off + 4..sec_dir_off + 8].try_into().ok()?) as usize;
    if sec_off != 0 && sec_sz > 0 && sec_off >= sec_dir_off + 8 && sec_off.saturating_add(sec_sz) <= bytes.len() {
        Some(alg.hash_slices(&[
            &bytes[..checksum_off],
            &bytes[checksum_off + 4..sec_dir_off],
            &bytes[sec_dir_off + 8..sec_off],
            &bytes[sec_off + sec_sz..],
        ]))
    } else {
        Some(alg.hash_slices(&[
            &bytes[..checksum_off],
            &bytes[checksum_off + 4..sec_dir_off],
            &bytes[sec_dir_off + 8..],
        ]))
    }
}

struct CtlMemberEntry<'a> {
    subject_id: &'a [u8],
    digests: Vec<(Digest, &'a [u8])>,
}

/// Parse a DER-encoded Microsoft `CertificateTrustList` (`szOID_CTL`) into its `TrustedSubject`
/// entries and any signed `SPC_INDIRECT_DATA_OBJID` member digests.
fn parse_ctl_entries(ctl_der: &[u8]) -> Option<Vec<CtlMemberEntry<'_>>> {
    let (seq, _) = parse_der_elem(ctl_der)?;
    if seq.tag != 0x30 {
        return None;
    }
    let kids = der_children(seq.val)?;
    // CertificateTrustList ::= SEQUENCE { subjectUsage, [listId], [seqNum], thisUpdate, [nextUpdate],
    //   subjectAlgorithm, [trustedSubjects SEQUENCE OF TrustedSubject], [ctlExtensions [0]] }
    // Locate `trustedSubjects`: the SEQUENCE immediately after `subjectAlgorithm` (which itself is a
    // SEQUENCE starting with an OID, appearing after `thisUpdate` UTCTime/GeneralizedTime).
    let mut after_time = false;
    let mut after_alg = false;
    let mut subjects_seq: Option<DerElem<'_>> = None;
    for &elem in &kids {
        if elem.tag == 0x17 || elem.tag == 0x18 {
            after_time = true;
            continue;
        }
        if after_time && !after_alg && elem.tag == 0x30 {
            after_alg = true;
            continue;
        }
        if after_alg && elem.tag == 0x30 {
            subjects_seq = Some(elem);
            break;
        }
    }
    let mut entries = Vec::new();
    let Some(subj_seq) = subjects_seq else {
        return Some(entries);
    };
    for subj in der_children(subj_seq.val)? {
        if subj.tag != 0x30 {
            continue;
        }
        let sk = der_children(subj.val)?;
        if sk.is_empty() || sk[0].tag != 0x04 {
            continue;
        }
        let subject_id = sk[0].val;
        let mut digests = Vec::new();
        if let Some(attrs) = sk.get(1).filter(|e| e.tag == 0x31) {
            if let Some(attr_list) = der_children(attrs.val) {
                for attr in attr_list {
                    if attr.tag != 0x30 {
                        continue;
                    }
                    let Some(ak) = der_children(attr.val) else { continue };
                    if ak.len() < 2 || ak[0].tag != 0x06 || ak[0].val != OID_SPC_INDIRECT_DATA || ak[1].tag != 0x31 {
                        continue;
                    }
                    let Some(vals) = der_children(ak[1].val) else { continue };
                    for v in vals {
                        if v.tag != 0x30 {
                            continue;
                        }
                        let Some(spc_kids) = der_children(v.val) else { continue };
                        if spc_kids.len() < 2 || spc_kids[1].tag != 0x30 {
                            continue;
                        }
                        let Some(di_kids) = der_children(spc_kids[1].val) else { continue };
                        if di_kids.len() < 2 || di_kids[1].tag != 0x04 {
                            continue;
                        }
                        if let Some(alg_oid) = parse_alg_id_oid(di_kids[0]) {
                            if let Some(alg) = digest_from_oid(alg_oid) {
                                digests.push((alg, di_kids[1].val));
                            }
                        }
                    }
                }
            }
        }
        entries.push(CtlMemberEntry { subject_id, digests });
    }
    Some(entries)
}

fn decode_hex_tag(tag: &str) -> Option<Vec<u8>> {
    let s = tag.trim();
    if s.is_empty() || s.len() % 2 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    for pair in bytes.chunks_exact(2) {
        let hi = (pair[0] as char).to_digit(16)?;
        let lo = (pair[1] as char).to_digit(16)?;
        out.push(((hi << 4) | lo) as u8);
    }
    Some(out)
}

fn ctl_subject_id_matches_tag(subject_id: &[u8], digests: &[(Digest, &[u8])], tag: &str) -> bool {
    // Windows catalogs encode subjectIdentifier as:
    // 1) UTF-16LE text tag (with or without NUL),
    // 2) ASCII/UTF-8 text tag, or
    // 3) raw binary digest bytes (e.g. 20-byte SHA-1 / 32-byte SHA-256) referenced by a hex
    //    `pcwszMemberTag` string (or matching an SPC_INDIRECT_DATA member digest).
    if let Some(hex_bytes) = decode_hex_tag(tag) {
        if subject_id == hex_bytes.as_slice()
            || digests.iter().any(|&(_, d)| d == hex_bytes.as_slice())
        {
            return true;
        }
    }
    if subject_id.len() >= 2 && subject_id.len() % 2 == 0 {
        let mut u16s: Vec<u16> = subject_id
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        while u16s.last() == Some(&0) {
            u16s.pop();
        }
        if let Ok(s) = String::from_utf16(&u16s) {
            if s.eq_ignore_ascii_case(tag) {
                return true;
            }
        }
    }
    if let Ok(s) = std::str::from_utf8(subject_id) {
        if s.trim_end_matches('\0').eq_ignore_ascii_case(tag) {
            return true;
        }
    }
    false
}

/// Verify that a catalog member (identified by `member_tag` and either `calculated_hash` or
/// `member_bytes`) matches a signed entry in `ctl_der`.
fn verify_ctl_membership(
    ctl_der: &[u8],
    member_tag: Option<&str>,
    calculated_hash: Option<&[u8]>,
    member_bytes: Option<&[u8]>,
) -> Result<(), i32> {
    if calculated_hash.is_none() && member_bytes.is_none() {
        return Err(TRUST_E_NOSIGNATURE);
    }
    let entries = parse_ctl_entries(ctl_der).ok_or(TRUST_E_SUBJECT_FORM_UNKNOWN)?;
    let mut matched_tag = false;
    for entry in &entries {
        if let Some(tag) = member_tag {
            if !ctl_subject_id_matches_tag(entry.subject_id, &entry.digests, tag) {
                continue;
            }
            matched_tag = true;
        }
        // Check SPC_INDIRECT_DATA DigestInfo entries first.
        for &(alg, expected_digest) in &entry.digests {
            if let Some(calc) = calculated_hash {
                if calc == expected_digest {
                    return Ok(());
                }
            }
            if let Some(bytes) = member_bytes {
                if let Some(pe_hash) = compute_pe_authenticode_digest(bytes, alg) {
                    if pe_hash.as_slice() == expected_digest {
                        return Ok(());
                    }
                }
                if alg.hash(bytes).as_slice() == expected_digest {
                    return Ok(());
                }
            }
        }
        // Fallback: some catalogs store raw member digest bytes directly in subjectIdentifier.
        if entry.digests.is_empty() {
            if let Some(calc) = calculated_hash {
                if calc == entry.subject_id {
                    return Ok(());
                }
            }
        }
    }
    if matched_tag {
        Err(TRUST_E_BAD_DIGEST)
    } else {
        Err(TRUST_E_NOSIGNATURE)
    }
}

unsafe fn verify_cert_via_chain_policy(
    cert_ctx: *const c_void,
    ch_stores: u32,
    pah_stores: *const *mut c_void,
    p_time: *const c_void,
    policy_oid: usize,
    policy_extra: *const c_void,
    chain_flags: u32,
    generic_chain: Option<GenericChainPolicy>,
) -> i32 {
    if cert_ctx.is_null() {
        return TRUST_E_NOSIGNATURE;
    }
    let extra_store = if ch_stores > 0 && !pah_stores.is_null() {
        let mem = super::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
        if !mem.is_null() {
            for i in 0..ch_stores as usize {
                let s = std::ptr::read_unaligned(pah_stores.add(i));
                for der in super::crypt32::store_certs_der(s) {
                    let _ = super::crypt32::CertAddEncodedCertificateToStore(
                        mem,
                        X509_ASN_ENCODING,
                        der.as_ptr(),
                        der.len() as u32,
                        4,
                        std::ptr::null_mut(),
                    );
                }
            }
        }
        mem
    } else {
        std::ptr::null_mut()
    };

    let h_chain_engine = generic_chain.map_or(std::ptr::null_mut(), |g| g.h_chain_engine);
    let p_chain_para = generic_chain.map_or(std::ptr::null(), |g| g.p_chain_para);
    let effective_chain_flags = chain_flags | generic_chain.map_or(0, |g| g.dw_flags);
    let mut chain_ctx: *mut c_void = std::ptr::null_mut();
    let ok = super::crypt32::CertGetCertificateChain(
        h_chain_engine,
        cert_ctx,
        p_time,
        extra_store,
        p_chain_para,
        effective_chain_flags,
        std::ptr::null_mut(),
        &mut chain_ctx,
    );
    if !extra_store.is_null() {
        super::crypt32::CertCloseStore(extra_store, 0);
    }
    if ok == 0 || chain_ctx.is_null() {
        return CERT_E_CHAINING;
    }

    // Per MSDN WINTRUST_ACTION_GENERIC_CHAIN_VERIFY / WTD_GENERIC_CHAIN_POLICY_DATA:
    // if the caller supplied a custom final-policy callback, build a valid CRYPT_PROVIDER_DATA
    // context (even when WTD_STATEACTION_VERIFY was not requested), invoke the callback with
    // `pProvData` and the built signer chain context, and return its HRESULT.
    if let Some(gc) = generic_chain {
        if gc.pfn_policy_callback != 0 {
            let mut trust_step_errors = [0u32; 8];
            let prov_cert = CryptProviderCert {
                cb_struct: std::mem::size_of::<CryptProviderCert>() as u32,
                p_cert: cert_ctx as *const CertContext,
                f_commercial: 1,
                f_trusted_root: 0,
                f_self_signed: 0,
                f_test_cert: 0,
                dw_revoked_reason: 0,
                dw_confidence: 0x1111_0000,
                dw_error: 0,
                p_trust_list_context: std::ptr::null_mut(),
                f_trust_list_signer_cert: 0,
                p_ctl_context: std::ptr::null_mut(),
                dw_ctl_error: 0,
                f_is_cyclic: 0,
                p_chain_element: std::ptr::null(),
            };
            let prov_signer = CryptProviderSgnr {
                cb_struct: std::mem::size_of::<CryptProviderSgnr>() as u32,
                sft_verify_as_of: [0, 0],
                cs_cert_chain: 1,
                pas_cert_chain: &prov_cert as *const CryptProviderCert,
                dw_signer_type: 0,
                ps_signer: std::ptr::null_mut(),
                dw_error: 0,
                cs_counter_signers: 0,
                pas_counter_signers: std::ptr::null_mut(),
                p_chain_context: chain_ctx as *const CertChainContext,
            };
            let mut prov_data = CryptProviderData {
                cb_struct: std::mem::size_of::<CryptProviderData>() as u32,
                p_wintrust_data: gc.wintrust_data,
                f_opened_file: 0,
                h_wnd_parent: std::ptr::null_mut(),
                pg_action_id: gc.action_guid,
                h_prov: 0,
                dw_error: 0,
                dw_reg_security_settings: 0,
                dw_reg_policy_settings: 0,
                ps_pfns: std::ptr::null_mut(),
                cdw_trust_step_errors: trust_step_errors.len() as u32,
                padw_trust_step_errors: trust_step_errors.as_mut_ptr(),
                ch_stores,
                pah_stores: pah_stores as *mut c_void,
                dw_encoding: X509_ASN_ENCODING | 0x0001_0000,
                h_msg: std::ptr::null_mut(),
                cs_signers: 1,
                pas_signers: &prov_signer as *const CryptProviderSgnr,
                cs_prov_priv_data: 0,
                pas_prov_priv_data: std::ptr::null_mut(),
                dw_subject_choice: gc.subject_choice,
                p_pdsip: std::ptr::null_mut(),
                psz_usage_oid: std::ptr::null(),
                f_recall_with_state: 0,
                sft_system_time: [0, 0],
                psz_ctl_signer_usage_oid: std::ptr::null(),
                dw_prov_flags: gc.prov_flags,
                dw_final_error: 0,
                p_request_usage: std::ptr::null_mut(),
                dw_trust_pub_settings: 0,
                dw_ui_state_flags: 0,
            };
            let signer_info = WtdGenericChainPolicySignerInfo {
                cb_size: std::mem::size_of::<WtdGenericChainPolicySignerInfo>() as u32,
                _pad0: 0,
                p_chain_context: chain_ctx as *const c_void,
                dw_signer_type: 0,
                _pad1: 0,
                p_msg_signer_info: std::ptr::null(),
                dw_error: 0,
                c_counter_signer: 0,
                rgp_counter_signer: std::ptr::null(),
            };
            let rgp_signer = [&signer_info as *const WtdGenericChainPolicySignerInfo];
            let cb_rc = crate::call_guest8(
                gc.pfn_policy_callback,
                &mut prov_data as *mut CryptProviderData as u64,
                0,
                prov_data.dw_reg_policy_settings as u64,
                1,
                rgp_signer.as_ptr() as u64,
                gc.pv_policy_arg as u64,
                0,
                0,
            ) as i32;
            super::crypt32::CertFreeCertificateChain(chain_ctx);
            return cb_rc;
        }
    }

    #[repr(C)]
    struct PolicyPara {
        cb_size: u32,
        dw_flags: u32,
        pv_extra_policy_para: *const c_void,
    }
    #[repr(C)]
    struct PolicyStatus {
        cb_size: u32,
        dw_error: u32,
        l_chain_index: i32,
        l_element_index: i32,
        pv_extra: *mut c_void,
    }
    let policy_para = PolicyPara {
        cb_size: std::mem::size_of::<PolicyPara>() as u32,
        dw_flags: 0,
        pv_extra_policy_para: policy_extra,
    };
    let p_para = if policy_extra.is_null() {
        std::ptr::null()
    } else {
        &policy_para as *const PolicyPara as *const c_void
    };
    let mut status = PolicyStatus {
        cb_size: std::mem::size_of::<PolicyStatus>() as u32,
        dw_error: 0,
        l_chain_index: -1,
        l_element_index: -1,
        pv_extra: std::ptr::null_mut(),
    };
    let pok = super::crypt32::CertVerifyCertificateChainPolicy(
        policy_oid as *const u8,
        chain_ctx,
        p_para,
        &mut status as *mut PolicyStatus as *mut c_void,
    );
    super::crypt32::CertFreeCertificateChain(chain_ctx);
    if pok == 0 {
        return CERT_E_CHAINING;
    }
    status.dw_error as i32
}

pub unsafe extern "win64" fn WinVerifyTrust(_hwnd: *mut c_void, action: *const u8, data: *mut c_void) -> i32 {
    if action.is_null() || data.is_null() {
        return TRUST_E_PROVIDER_UNKNOWN;
    }
    let mut guid = [0u8; 16];
    std::ptr::copy_nonoverlapping(action, guid.as_mut_ptr(), 16);

    let is_known_action = guid == WINTRUST_ACTION_GENERIC_VERIFY_V2
        || guid == WINTRUST_ACTION_TRUSTPROVIDER_TEST
        || guid == WINTRUST_ACTION_GENERIC_CERT_VERIFY
        || guid == WINTRUST_ACTION_GENERIC_CHAIN_VERIFY
        || guid == DRIVER_ACTION_VERIFY
        || guid == HTTPSPROV_ACTION;
    if !is_known_action {
        // Per MSDN: unknown trust action GUIDs return TRUST_E_ACTION_UNKNOWN.
        let a = super::guid_string(action);
        log::debug!("WinVerifyTrust(unknown action={a}) -> TRUST_E_ACTION_UNKNOWN");
        return TRUST_E_ACTION_UNKNOWN;
    }

    if guid == DRIVER_ACTION_VERIFY {
        // WHQL driver verification (`DRIVER_ACTION_VERIFY` / `DRIVER_VER_INFO`) enforces
        // Windows WHQL driver policy rather than ordinary Authenticode code signing; fail
        // closed honestly instead of accepting non-WHQL code-signing chains.
        log::debug!("WinVerifyTrust(DRIVER_ACTION_VERIFY) -> TRUST_E_PROVIDER_UNKNOWN");
        return TRUST_E_PROVIDER_UNKNOWN;
    }

    let base = data as *const u8;
    let cb_struct = std::ptr::read_unaligned(base as *const u32);
    if cb_struct < 0x40 {
        return TRUST_E_PROVIDER_UNKNOWN;
    }
    let fdw_revocation_checks = std::ptr::read_unaligned(base.add(0x1C) as *const u32);
    let union_choice = std::ptr::read_unaligned(base.add(0x20) as *const u32);
    let p_union = std::ptr::read_unaligned(base.add(0x28) as *const *const u8);
    let state_action = std::ptr::read_unaligned(base.add(0x30) as *const u32);
    let state_data_slot = (data as *mut u8).add(0x38) as *mut *mut c_void;
    let cur_state = std::ptr::read_unaligned(state_data_slot);
    let dw_prov_flags = if cb_struct >= 0x4C {
        std::ptr::read_unaligned(base.add(0x48) as *const u32)
    } else {
        0
    };
    let chain_flags = map_wintrust_revocation_flags(fdw_revocation_checks, dw_prov_flags);

    if state_action == WTD_STATEACTION_CLOSE {
        if !cur_state.is_null() {
            STATE_MAP.lock().unwrap().remove(&(cur_state as usize));
            std::ptr::write_unaligned(state_data_slot, std::ptr::null_mut());
        }
        return ERROR_SUCCESS;
    }

    if p_union.is_null() {
        return TRUST_E_NOSIGNATURE;
    }

    let p_policy_callback_data = std::ptr::read_unaligned(base.add(0x08) as *const *const c_void);
    let chain_policy_oid: usize = if guid == HTTPSPROV_ACTION {
        4 // CERT_CHAIN_POLICY_SSL
    } else if guid == WINTRUST_ACTION_GENERIC_CERT_VERIFY
        || guid == WINTRUST_ACTION_GENERIC_CHAIN_VERIFY
    {
        1 // CERT_CHAIN_POLICY_BASE
    } else {
        2 // CERT_CHAIN_POLICY_AUTHENTICODE
    };
    let generic_chain: Option<GenericChainPolicy> =
        if guid == WINTRUST_ACTION_GENERIC_CHAIN_VERIFY && !p_policy_callback_data.is_null() {
            // Windows SDK SoftPub.h WTD_GENERIC_CHAIN_POLICY_DATA (Win64, 0x28 bytes):
            //   cbSize                  @ +0x00 (DWORD + 4B pad)
            //   pSignerChainInfo        @ +0x08 (PWTD_GENERIC_CHAIN_POLICY_CREATE_INFO)
            //   pCounterSignerChainInfo @ +0x10 (PWTD_GENERIC_CHAIN_POLICY_CREATE_INFO)
            //   pfnPolicyCallback       @ +0x18 (PFN_WTD_GENERIC_CHAIN_POLICY_CALLBACK)
            //   pvPolicyArg             @ +0x20 (void *)
            let p = p_policy_callback_data as *const u8;
            let cb_gc = std::ptr::read_unaligned(p as *const u32);
            if cb_gc < 0x28 {
                return 0x8007_0057u32 as i32; // E_INVALIDARG
            }
            let p_signer_chain_info = std::ptr::read_unaligned(p.add(0x08) as *const *const u8);
            let (h_chain_engine, p_chain_para, dw_flags) = if !p_signer_chain_info.is_null() {
                // SoftPub.h WTD_GENERIC_CHAIN_POLICY_CREATE_INFO (Win64, 0x28 bytes):
                //   cbSize       @ +0x00 (DWORD + 4B pad)
                //   hChainEngine @ +0x08 (HCERTCHAINENGINE)
                //   pChainPara   @ +0x10 (PCERT_CHAIN_PARA)
                //   dwFlags      @ +0x18 (DWORD + 4B pad)
                //   pvReserved   @ +0x20 (void *)
                let cb_ci = std::ptr::read_unaligned(p_signer_chain_info as *const u32);
                if cb_ci < 0x28 {
                    return 0x8007_0057u32 as i32; // E_INVALIDARG
                }
                (
                    std::ptr::read_unaligned(p_signer_chain_info.add(0x08) as *const *mut c_void),
                    std::ptr::read_unaligned(p_signer_chain_info.add(0x10) as *const *const c_void),
                    std::ptr::read_unaligned(p_signer_chain_info.add(0x18) as *const u32),
                )
            } else {
                (std::ptr::null_mut(), std::ptr::null(), 0)
            };
            Some(GenericChainPolicy {
                h_chain_engine,
                p_chain_para,
                dw_flags,
                pfn_policy_callback: std::ptr::read_unaligned(p.add(0x18) as *const usize),
                pv_policy_arg: std::ptr::read_unaligned(p.add(0x20) as *const *mut c_void),
                wintrust_data: data,
                action_guid: action,
                prov_flags: dw_prov_flags,
                subject_choice: union_choice,
            })
        } else {
            None
        };
    let policy_extra: *const c_void = if guid == HTTPSPROV_ACTION && !p_policy_callback_data.is_null() {
        // Validate HTTPSPolicyCallbackData / SSL_EXTRA_CERT_CHAIN_POLICY_PARA (24B on Win64):
        // cbStruct @ +0x00, dwAuthType @ +0x04 (1=CLIENT, 2=SERVER), fdwChecks @ +0x08, pwszServerName @ +0x10
        let p = p_policy_callback_data as *const u8;
        let cb_ssl = std::ptr::read_unaligned(p as *const u32);
        if cb_ssl < 0x18 {
            return 0x8007_0057u32 as i32; // E_INVALIDARG
        }
        let dw_auth_type = std::ptr::read_unaligned(p.add(0x04) as *const u32);
        if dw_auth_type != 1 && dw_auth_type != 2 {
            return 0x8007_0057u32 as i32; // E_INVALIDARG
        }
        let fdw_checks = std::ptr::read_unaligned(p.add(0x08) as *const u32);
        let pwsz_server = std::ptr::read_unaligned(p.add(0x10) as *const *const u16);
        let ignore_cn = (fdw_checks & 0x0000_1000) != 0; // SECURITY_FLAG_IGNORE_CERT_CN_INVALID
        if dw_auth_type == 2 && !ignore_cn {
            let empty_name = pwsz_server.is_null() || crate::strings::from_wide(pwsz_server).is_empty();
            if empty_name {
                return 0x800B_010Fu32 as i32; // CERT_E_CN_NO_MATCH
            }
        }
        p_policy_callback_data
    } else {
        std::ptr::null()
    };

    match union_choice {
        WTD_CHOICE_FILE => {
            let pcwsz_path = std::ptr::read_unaligned(p_union.add(0x08) as *const *const u16);
            let h_file = std::ptr::read_unaligned(p_union.add(0x10) as *const crate::types::HANDLE);

            let (label, host_path) = if !pcwsz_path.is_null() {
                let win = crate::strings::from_wide(pcwsz_path);
                let abs = crate::path::absolute(&win);
                let hp = crate::path::to_host(&abs);
                (win, hp)
            } else if !h_file.is_null() && h_file != crate::types::INVALID_HANDLE_VALUE {
                match crate::kernel32::file::get_file(h_file) {
                    Some(f) => (f.win_path.clone(), f.host_path.clone()),
                    None => (String::from("<handle>"), None),
                }
            } else {
                return CRYPT_E_FILE_ERROR;
            };

            let Some(host_path) = host_path else {
                log::warn!("WinVerifyTrust({label:?}): cannot map path -> CRYPT_E_FILE_ERROR");
                return CRYPT_E_FILE_ERROR;
            };

            let bytes = match std::fs::read(&host_path) {
                Ok(b) => b,
                Err(e) => {
                    log::warn!(
                        "WinVerifyTrust({label:?}): read {}: {e} -> CRYPT_E_FILE_ERROR",
                        host_path.display()
                    );
                    return CRYPT_E_FILE_ERROR;
                }
            };

            match verify_pe_authenticode(&bytes) {
                Ok(verified) => {
                    let ncerts = verified.certs.len();
                    let all_der: Vec<Vec<u8>> =
                        verified.certs.iter().map(|c| c.raw_der.clone()).collect();
                    let chain_rc = verify_pkcs7_signer_chain(
                        &verified,
                        &all_der,
                        chain_policy_oid,
                        policy_extra,
                        chain_flags,
                        generic_chain,
                    );
                    if state_action == WTD_STATEACTION_VERIFY {
                        let h = alloc_provider_state(verified, data);
                        std::ptr::write_unaligned(state_data_slot, h);
                    }
                    if chain_rc == ERROR_SUCCESS {
                        log::info!("WinVerifyTrust({label:?}): verified PE Authenticode digest + PKCS#7 signature + signer chain ({ncerts} cert(s))");
                    } else {
                        log::warn!("WinVerifyTrust({label:?}): PE Authenticode signer chain rejected ({chain_rc:#010x})");
                    }
                    chain_rc
                }
                Err(code) => {
                    log::warn!("WinVerifyTrust({label:?}): Authenticode verification failed ({code:#010x})");
                    code
                }
            }
        }
        WTD_CHOICE_CATALOG => {
            // WINTRUST_CATALOG_INFO (Win64):
            //   cbStruct @ +0x00, dwCatalogVersion @ +0x04,
            //   pcwszCatalogFilePath @ +0x08, pcwszMemberTag @ +0x10,
            //   pcwszMemberFilePath @ +0x18, hMemberFile @ +0x20,
            //   pbCalculatedFileHash @ +0x28, cbCalculatedFileHash @ +0x30
            let cb_cat = std::ptr::read_unaligned(p_union as *const u32);
            if cb_cat < 0x20 {
                return TRUST_E_SUBJECT_FORM_UNKNOWN;
            }
            let cat_path_w = std::ptr::read_unaligned(p_union.add(0x08) as *const *const u16);
            if cat_path_w.is_null() {
                return TRUST_E_NOSIGNATURE;
            }
            let member_tag_w = std::ptr::read_unaligned(p_union.add(0x10) as *const *const u16);
            let member_path_w = std::ptr::read_unaligned(p_union.add(0x18) as *const *const u16);
            let member_handle = if cb_cat >= 0x28 {
                std::ptr::read_unaligned(p_union.add(0x20) as *const crate::types::HANDLE)
            } else {
                std::ptr::null_mut()
            };
            let (calc_hash_ptr, calc_hash_len) = if cb_cat >= 0x34 {
                (
                    std::ptr::read_unaligned(p_union.add(0x28) as *const *const u8),
                    std::ptr::read_unaligned(p_union.add(0x30) as *const u32) as usize,
                )
            } else {
                (std::ptr::null(), 0)
            };

            let win = crate::strings::from_wide(cat_path_w);
            let abs = crate::path::absolute(&win);
            let Some(host) = crate::path::to_host(&abs) else {
                return CRYPT_E_FILE_ERROR;
            };
            let Ok(cat_bytes) = std::fs::read(&host) else {
                return CRYPT_E_FILE_ERROR;
            };

            // Step 1: Verify catalog PKCS#7 cryptographic signature and require szOID_CTL content.
            let p7 = match verify_standalone_pkcs7(&cat_bytes, None) {
                Ok(v) => v,
                Err(code) => return code,
            };
            if p7.encap_content_type_oid.as_slice() != OID_CTL {
                return TRUST_E_SUBJECT_FORM_UNKNOWN;
            }

            // Step 2: Obtain the specified catalog member's hash or file bytes and verify
            // membership against the signed CertificateTrustList entries.
            let member_tag = (!member_tag_w.is_null()).then(|| crate::strings::from_wide(member_tag_w));
            let calc_hash = (calc_hash_len > 0 && !calc_hash_ptr.is_null())
                .then(|| std::slice::from_raw_parts(calc_hash_ptr, calc_hash_len));
            let member_bytes = if calc_hash.is_none() {
                if !member_path_w.is_null() {
                    let mw = crate::strings::from_wide(member_path_w);
                    let mabs = crate::path::absolute(&mw);
                    let Some(mhost) = crate::path::to_host(&mabs) else {
                        return CRYPT_E_FILE_ERROR;
                    };
                    let Ok(b) = std::fs::read(&mhost) else {
                        return CRYPT_E_FILE_ERROR;
                    };
                    Some(b)
                } else if !member_handle.is_null() && member_handle != crate::types::INVALID_HANDLE_VALUE {
                    let Some(mhost) = crate::kernel32::file::get_file(member_handle).and_then(|f| f.host_path.clone()) else {
                        return CRYPT_E_FILE_ERROR;
                    };
                    let Ok(b) = std::fs::read(&mhost) else {
                        return CRYPT_E_FILE_ERROR;
                    };
                    Some(b)
                } else {
                    None
                }
            } else {
                None
            };
            if let Err(code) = verify_ctl_membership(
                &p7.encap_content_der,
                member_tag.as_deref(),
                calc_hash,
                member_bytes.as_deref(),
            ) {
                return code;
            }

            // Step 3: Validate the catalog signer's certificate chain & action policy.
            let chain_rc = verify_pkcs7_signer_chain(
                &p7.verified,
                &p7.all_certs_der,
                chain_policy_oid,
                policy_extra,
                chain_flags,
                generic_chain,
            );
            if state_action == WTD_STATEACTION_VERIFY {
                let h = alloc_provider_state(p7.verified, data);
                std::ptr::write_unaligned(state_data_slot, h);
            }
            chain_rc
        }
        WTD_CHOICE_BLOB => {
            // WINTRUST_BLOB_INFO (Win64): cbMemObject @ +0x20, pbMemObject @ +0x28,
            // cbMemSignedMsg @ +0x30, pbMemSignedMsg @ +0x38
            let cb_struct_b = std::ptr::read_unaligned(p_union as *const u32);
            if cb_struct_b < 0x30 {
                return TRUST_E_SUBJECT_FORM_UNKNOWN;
            }
            let cb_obj = std::ptr::read_unaligned(p_union.add(0x20) as *const u32) as usize;
            let pb_obj = std::ptr::read_unaligned(p_union.add(0x28) as *const *const u8);
            let (cb_sig, pb_sig) = if cb_struct_b >= 0x40 {
                (
                    std::ptr::read_unaligned(p_union.add(0x30) as *const u32) as usize,
                    std::ptr::read_unaligned(p_union.add(0x38) as *const *const u8),
                )
            } else {
                (0, std::ptr::null())
            };
            if cb_sig > 0 && !pb_sig.is_null() {
                let sig_slice = std::slice::from_raw_parts(pb_sig, cb_sig);
                let obj_opt = (cb_obj > 0 && !pb_obj.is_null())
                    .then(|| std::slice::from_raw_parts(pb_obj, cb_obj));
                match verify_standalone_pkcs7(sig_slice, obj_opt) {
                    Ok(p7) => {
                        let chain_rc = verify_pkcs7_signer_chain(
                            &p7.verified,
                            &p7.all_certs_der,
                            chain_policy_oid,
                            policy_extra,
                            chain_flags,
                            generic_chain,
                        );
                        if state_action == WTD_STATEACTION_VERIFY {
                            let h = alloc_provider_state(p7.verified, data);
                            std::ptr::write_unaligned(state_data_slot, h);
                        }
                        chain_rc
                    }
                    Err(code) => code,
                }
            } else if cb_obj > 0 && !pb_obj.is_null() {
                let obj_slice = std::slice::from_raw_parts(pb_obj, cb_obj);
                if obj_slice.len() >= 2 && &obj_slice[..2] == b"MZ" {
                    match verify_pe_authenticode(obj_slice) {
                        Ok(verified) => {
                            let all_der: Vec<Vec<u8>> =
                                verified.certs.iter().map(|c| c.raw_der.clone()).collect();
                            let chain_rc = verify_pkcs7_signer_chain(
                                &verified,
                                &all_der,
                                chain_policy_oid,
                                policy_extra,
                                chain_flags,
                                generic_chain,
                            );
                            if state_action == WTD_STATEACTION_VERIFY {
                                let h = alloc_provider_state(verified, data);
                                std::ptr::write_unaligned(state_data_slot, h);
                            }
                            chain_rc
                        }
                        Err(code) => code,
                    }
                } else {
                    match verify_standalone_pkcs7(obj_slice, None) {
                        Ok(p7) => {
                            let chain_rc = verify_pkcs7_signer_chain(
                                &p7.verified,
                                &p7.all_certs_der,
                                chain_policy_oid,
                                policy_extra,
                                chain_flags,
                                generic_chain,
                            );
                            if state_action == WTD_STATEACTION_VERIFY {
                                let h = alloc_provider_state(p7.verified, data);
                                std::ptr::write_unaligned(state_data_slot, h);
                            }
                            chain_rc
                        }
                        Err(code) => code,
                    }
                }
            } else {
                TRUST_E_NOSIGNATURE
            }
        }
        WTD_CHOICE_SIGNER => {
            // WINTRUST_SGNR_INFO (Win64): psSignerInfo @ +0x10, chStores @ +0x18, pahStores @ +0x20
            let ps_signer = std::ptr::read_unaligned(p_union.add(0x10) as *const *const CmsgSignerInfo);
            let ch_stores = std::ptr::read_unaligned(p_union.add(0x18) as *const u32);
            let pah_stores = std::ptr::read_unaligned(p_union.add(0x20) as *const *const *mut c_void);
            if ps_signer.is_null() || ch_stores == 0 || pah_stores.is_null() {
                return TRUST_E_NOSIGNATURE;
            }
            let si = std::ptr::read_unaligned(ps_signer);
            let mut query_ci: CertInfo = std::mem::zeroed();
            query_ci.issuer = CryptDataBlob {
                cb_data: si.issuer.cb_data,
                pb_data: si.issuer.pb_data,
            };
            query_ci.serial_number = CryptDataBlob {
                cb_data: si.serial_number.cb_data,
                pb_data: si.serial_number.pb_data,
            };
            let mut found_cert: *mut c_void = std::ptr::null_mut();
            for i in 0..ch_stores as usize {
                let st = std::ptr::read_unaligned(pah_stores.add(i));
                let c = super::crypt32::CertGetSubjectCertificateFromStore(
                    st,
                    X509_ASN_ENCODING,
                    &query_ci as *const CertInfo as *const c_void,
                );
                if !c.is_null() {
                    found_cert = c;
                    break;
                }
            }
            if found_cert.is_null() {
                return TRUST_E_NOSIGNATURE;
            }
            // Cryptographically verify the supplied CMSG_SIGNER_INFO signature against the
            // matched signer certificate before evaluating its chain policy (fail closed on
            // empty, forged, or unverifiable signatures).
            let cc = std::ptr::read_unaligned(found_cert as *const CertContext);
            let sig_ok = !cc.pb_cert_encoded.is_null()
                && cc.cb_cert_encoded > 0
                && super::crypt32::verify_cmsg_signer_info_signature(
                    &si,
                    std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize),
                );
            if !sig_ok {
                super::crypt32::CertFreeCertificateContext(found_cert);
                return TRUST_E_BAD_DIGEST;
            }
            let rc = verify_cert_via_chain_policy(
                found_cert,
                ch_stores,
                pah_stores,
                std::ptr::null(),
                chain_policy_oid,
                policy_extra,
                chain_flags,
                generic_chain,
            );
            super::crypt32::CertFreeCertificateContext(found_cert);
            rc
        }
        WTD_CHOICE_CERT => {
            // WINTRUST_CERT_INFO (Win64): psCertContext @ +0x10, chStores @ +0x18, pahStores @ +0x20,
            // dwFlags @ +0x28, psftVerifyAsOf @ +0x30
            let cb_ci = std::ptr::read_unaligned(p_union as *const u32);
            if cb_ci < 0x18 {
                return TRUST_E_SUBJECT_FORM_UNKNOWN;
            }
            let cert_ctx = std::ptr::read_unaligned(p_union.add(0x10) as *const *const c_void);
            let ch_stores = if cb_ci >= 0x20 {
                std::ptr::read_unaligned(p_union.add(0x18) as *const u32)
            } else {
                0
            };
            let pah_stores = if cb_ci >= 0x28 {
                std::ptr::read_unaligned(p_union.add(0x20) as *const *const *mut c_void)
            } else {
                std::ptr::null()
            };
            let p_time = if cb_ci >= 0x38 {
                std::ptr::read_unaligned(p_union.add(0x30) as *const *const c_void)
            } else {
                std::ptr::null()
            };
            verify_cert_via_chain_policy(
                cert_ctx,
                ch_stores,
                pah_stores,
                p_time,
                chain_policy_oid,
                policy_extra,
                chain_flags,
                generic_chain,
            )
        }
        other => {
            // Per MSDN: unrecognized dwUnionChoice returns TRUST_E_SUBJECT_FORM_UNKNOWN.
            log::debug!("WinVerifyTrust: invalid dwUnionChoice={other} -> TRUST_E_SUBJECT_FORM_UNKNOWN");
            TRUST_E_SUBJECT_FORM_UNKNOWN
        }
    }
}

pub unsafe extern "win64" fn WinVerifyTrustEx(hwnd: *mut c_void, action: *const u8, data: *mut c_void) -> i32 {
    WinVerifyTrust(hwnd, action, data)
}

pub unsafe extern "win64" fn WTHelperProvDataFromStateData(state_data: *mut c_void) -> *mut c_void {
    if state_data.is_null() {
        return std::ptr::null_mut();
    }
    let map = STATE_MAP.lock().unwrap();
    let Some(st) = map.get(&(state_data as usize)) else {
        return std::ptr::null_mut();
    };
    &*st.provider_data as *const CryptProviderData as *mut c_void
}

pub unsafe extern "win64" fn WTHelperGetProvSignerFromChain(
    prov_data: *mut c_void,
    idx_signer: u32,
    is_counter_signer: i32,
    _idx_counter_signer: u32,
) -> *mut c_void {
    if prov_data.is_null() || is_counter_signer != 0 {
        return std::ptr::null_mut();
    }
    let pd = &*(prov_data as *const CryptProviderData);
    if pd.pas_signers.is_null() || idx_signer >= pd.cs_signers {
        return std::ptr::null_mut();
    }
    pd.pas_signers.add(idx_signer as usize) as *mut c_void
}

pub unsafe extern "win64" fn WTHelperGetProvCertFromChain(
    signer: *mut c_void,
    idx_cert: u32,
) -> *mut c_void {
    if signer.is_null() {
        return std::ptr::null_mut();
    }
    let s = &*(signer as *const CryptProviderSgnr);
    if s.pas_cert_chain.is_null() || idx_cert >= s.cs_cert_chain {
        return std::ptr::null_mut();
    }
    s.pas_cert_chain.add(idx_cert as usize) as *mut c_void
}

pub unsafe extern "win64" fn CryptCATAdminAcquireContext(
    ph_cat_admin: *mut *mut c_void,
    _pg_subsystem: *const u8,
    _flags: u32,
) -> i32 {
    if !ph_cat_admin.is_null() {
        std::ptr::write_unaligned(ph_cat_admin, std::ptr::null_mut());
    }
    crate::teb::set_last_error(crate::types::err::ERROR_CALL_NOT_IMPLEMENTED);
    0
}

pub unsafe extern "win64" fn CryptCATAdminAcquireContext2(
    ph_cat_admin: *mut *mut c_void,
    _pg_subsystem: *const u8,
    _hash_alg: *const u16,
    _strong_hash_policy: *const c_void,
    _flags: u32,
) -> i32 {
    if !ph_cat_admin.is_null() {
        std::ptr::write_unaligned(ph_cat_admin, std::ptr::null_mut());
    }
    crate::teb::set_last_error(crate::types::err::ERROR_CALL_NOT_IMPLEMENTED);
    0
}

pub unsafe extern "win64" fn CryptCATAdminReleaseContext(_h: *mut c_void, _flags: u32) -> i32 {
    1
}

pub unsafe extern "win64" fn CryptCATAdminCalcHashFromFileHandle(
    _h_file: *mut c_void,
    pcb_hash: *mut u32,
    _pb_hash: *mut u8,
    _flags: u32,
) -> i32 {
    if !pcb_hash.is_null() {
        std::ptr::write_unaligned(pcb_hash, 0);
    }
    crate::teb::set_last_error(crate::types::err::ERROR_CALL_NOT_IMPLEMENTED);
    0
}

pub unsafe extern "win64" fn CryptCATAdminCalcHashFromFileHandle2(
    _h_cat_admin: *mut c_void,
    _h_file: *mut c_void,
    pcb_hash: *mut u32,
    _pb_hash: *mut u8,
    _flags: u32,
) -> i32 {
    if !pcb_hash.is_null() {
        std::ptr::write_unaligned(pcb_hash, 0);
    }
    crate::teb::set_last_error(crate::types::err::ERROR_CALL_NOT_IMPLEMENTED);
    0
}

pub unsafe extern "win64" fn CryptCATAdminEnumCatalogFromHash(
    _h_cat_admin: *mut c_void,
    _pb_hash: *const u8,
    _cb_hash: u32,
    _flags: u32,
    _ph_prev: *mut *mut c_void,
) -> *mut c_void {
    crate::teb::set_last_error(crate::types::err::ERROR_NOT_FOUND);
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn CryptCATAdminReleaseCatalogContext(
    _h_cat_admin: *mut c_void,
    _h_cat_info: *mut c_void,
    _flags: u32,
) -> i32 {
    1
}

pub unsafe extern "win64" fn CryptCATCatalogInfoFromContext(
    _h_cat_info: *mut c_void,
    _ps_cat_info: *mut c_void,
    _flags: u32,
) -> i32 {
    crate::teb::set_last_error(crate::types::err::ERROR_INVALID_HANDLE);
    0
}

pub unsafe extern "win64" fn SfcIsFileProtected(
    _rpc_handle: *mut c_void,
    _prot_file_name: *const u16,
) -> i32 {
    0 // FALSE (file is not protected by Windows Resource Protection)
}

const TBS_E_INVALID_CONTEXT: u32 = 0x8028_4004;
const TBS_E_TPM_NOT_FOUND: u32 = 0x8028_400F;

// TPM Base Services (TBS, `tbs.h`): Apple Silicon has no TCG TPM 2.0 device or Windows
// TBS kernel driver (`tbs.sys`). Per Microsoft Learn (`Tbsi_Context_Create`,
// `Tbsip_Context_Close`, `Tbsip_Submit_Command`, `Tbsi_GetDeviceInfo`), Windows machines
// without a TPM return `TBS_E_TPM_NOT_FOUND` (0x8028400F) on context creation, device
// queries, and command submission, and `TBS_E_INVALID_CONTEXT` (0x80284004) when closing
// any handle since no valid TBS context can exist.
pub unsafe extern "win64" fn Tbsi_Context_Create(
    _context_params: *const c_void,
    ph_context: *mut *mut c_void,
) -> u32 {
    if !ph_context.is_null() {
        std::ptr::write_unaligned(ph_context, std::ptr::null_mut());
    }
    TBS_E_TPM_NOT_FOUND
}

pub unsafe extern "win64" fn Tbsip_Context_Close(h_context: *mut c_void) -> u32 {
    log::debug!("Tbsip_Context_Close({h_context:p}) -> TBS_E_INVALID_CONTEXT (no TPM on Apple Silicon)");
    TBS_E_INVALID_CONTEXT
}

pub unsafe extern "win64" fn Tbsip_Submit_Command(
    _h_context: *mut c_void,
    _locality: u32,
    _priority: u32,
    _command_buf: *const u8,
    _command_buf_len: u32,
    _result_buf: *mut u8,
    result_buf_len: *mut u32,
) -> u32 {
    if !result_buf_len.is_null() {
        std::ptr::write_unaligned(result_buf_len, 0);
    }
    log::debug!("Tbsip_Submit_Command -> TBS_E_TPM_NOT_FOUND (no TPM on Apple Silicon)");
    TBS_E_TPM_NOT_FOUND
}

pub unsafe extern "win64" fn Tbsi_GetDeviceInfo(_size: u32, _info: *mut c_void) -> u32 {
    TBS_E_TPM_NOT_FOUND
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tbs_reports_tpm_not_found() {
        unsafe {
            let mut ctx = 0x1234usize as *mut c_void;
            assert_eq!(Tbsi_Context_Create(std::ptr::null(), &mut ctx), TBS_E_TPM_NOT_FOUND);
            assert!(ctx.is_null());
            assert_eq!(Tbsip_Context_Close(ctx), TBS_E_INVALID_CONTEXT);
            let mut out_len = 99u32;
            assert_eq!(
                Tbsip_Submit_Command(
                    std::ptr::null_mut(),
                    0,
                    0,
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                    &mut out_len
                ),
                TBS_E_TPM_NOT_FOUND
            );
            assert_eq!(out_len, 0);
        }
    }

    #[test]
    fn wintrust_abi_offsets_match_win64() {
        assert_eq!(std::mem::offset_of!(CryptProviderSgnr, cs_cert_chain), 0x0C);
        assert_eq!(std::mem::offset_of!(CryptProviderSgnr, pas_cert_chain), 0x10);
        assert_eq!(std::mem::offset_of!(CryptProviderSgnr, ps_signer), 0x20);
        assert_eq!(std::mem::offset_of!(CryptProviderSgnr, p_chain_context), 0x38);
        assert_eq!(std::mem::offset_of!(CertChainContext, c_chain), 0x0C);
        assert_eq!(std::mem::offset_of!(CertChainContext, rgp_chain), 0x10);
        assert_eq!(std::mem::offset_of!(CertSimpleChain, c_element), 0x0C);
        assert_eq!(std::mem::offset_of!(CertSimpleChain, rgp_element), 0x10);
        assert_eq!(std::mem::offset_of!(CertChainElement, p_cert_context), 0x08);
        assert_eq!(std::mem::offset_of!(CertContext, p_cert_info), 0x18);
        assert_eq!(std::mem::offset_of!(CertInfo, issuer), 0x30);
        assert_eq!(std::mem::offset_of!(CertInfo, subject), 0x50);
    }

    #[test]
    fn oid_and_dn_parsing() {
        let cn_oid = encode_oid_str("2.5.4.3").unwrap();
        assert_eq!(cn_oid, vec![0x55, 0x04, 0x03]);
        assert_eq!(decode_oid_str(&cn_oid), "2.5.4.3");
        assert_eq!(decode_oid_str(OID_SHA256_WITH_RSA), "1.2.840.113549.1.1.11");

        // Minimal DER Name: SEQUENCE { SET { SEQUENCE { OID 2.5.4.3, UTF8String "Test Signer" } } }
        let dn: &[u8] = &[
            0x30, 0x16, 0x31, 0x14, 0x30, 0x12, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x0B,
            b'T', b'e', b's', b't', b' ', b'S', b'i', b'g', b'n', b'e', b'r',
        ];
        assert_eq!(extract_dn_attr(dn, &cn_oid).as_deref(), Some("Test Signer"));
        assert_eq!(format_dn(dn).as_deref(), Some("CN=Test Signer"));
    }

    #[test]
    fn authenticode_rejects_non_pe_and_unsigned_pe() {
        assert_eq!(verify_pe_authenticode(b"not a pe").err(), Some(TRUST_E_SUBJECT_FORM_UNKNOWN));
        // Minimal unsigned PE32+ image with zeroed security directory.
        let mut pe = vec![0u8; 0x200];
        pe[0..2].copy_from_slice(b"MZ");
        pe[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        pe[0x80..0x84].copy_from_slice(b"PE\0\0");
        pe[0x98..0x9A].copy_from_slice(&0x20Bu16.to_le_bytes());
        assert_eq!(verify_pe_authenticode(&pe).err(), Some(TRUST_E_NOSIGNATURE));
    }

    #[test]
    fn win_verify_trust_actions_and_choices_and_tbs() {
        unsafe {
            // Non-null TBS handle still returns TBS_E_INVALID_CONTEXT (no TPM on Apple Silicon)
            assert_eq!(Tbsip_Context_Close(0x1234usize as *mut c_void), TBS_E_INVALID_CONTEXT);
            assert_eq!(Tbsi_GetDeviceInfo(0, std::ptr::null_mut()), TBS_E_TPM_NOT_FOUND);

            // Unknown action GUID returns TRUST_E_ACTION_UNKNOWN
            let unknown_guid = [0xAAu8; 16];
            let mut wtd = [0u8; 0x58];
            std::ptr::write_unaligned(wtd.as_mut_ptr() as *mut u32, wtd.len() as u32);
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    unknown_guid.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                TRUST_E_ACTION_UNKNOWN
            );

            // WTD_CHOICE_BLOB with real OpenSSL PKCS#7 SignedData signed by a self-signed untrusted
            // cert: verifies PKCS#7 cryptography, enforces signer chain policy -> CERT_E_UNTRUSTEDROOT,
            // and still populates hWVTStateData when WTD_STATEACTION_VERIFY is requested.
            let pkcs7 = crate::misc::crypt32::tests::test_vectors::PKCS7_SIGNED_DER;
            let mut blob_info = [0u8; 0x40];
            std::ptr::write_unaligned(blob_info.as_mut_ptr() as *mut u32, blob_info.len() as u32);
            std::ptr::write_unaligned(blob_info.as_mut_ptr().add(0x20) as *mut u32, pkcs7.len() as u32);
            std::ptr::write_unaligned(blob_info.as_mut_ptr().add(0x28) as *mut *const u8, pkcs7.as_ptr());

            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x20) as *mut u32, WTD_CHOICE_BLOB);
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x28) as *mut *const u8, blob_info.as_ptr());
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x30) as *mut u32, WTD_STATEACTION_VERIFY);
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                crate::misc::crypt32::CERT_E_UNTRUSTEDROOT as i32
            );
            let st_handle = std::ptr::read_unaligned(wtd.as_ptr().add(0x38) as *const *mut c_void);
            assert!(!st_handle.is_null());
            let prov = WTHelperProvDataFromStateData(st_handle);
            assert!(!prov.is_null());
            let sgnr = WTHelperGetProvSignerFromChain(prov, 0, 0, 0);
            assert!(!sgnr.is_null());
            let cert_el = WTHelperGetProvCertFromChain(sgnr, 0);
            assert!(!cert_el.is_null());
            // Close provider state
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x30) as *mut u32, WTD_STATEACTION_CLOSE);
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS
            );

            // Tampered blob fails earlier at cryptographic check with TRUST_E_BAD_DIGEST
            let mut bad_p7 = pkcs7.to_vec();
            let last = bad_p7.len() - 1;
            bad_p7[last] ^= 0xFF;
            std::ptr::write_unaligned(blob_info.as_mut_ptr().add(0x28) as *mut *const u8, bad_p7.as_ptr());
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x30) as *mut u32, 0);
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                TRUST_E_BAD_DIGEST
            );

            // DRIVER_ACTION_VERIFY (WHQL driver verification) fails closed with
            // TRUST_E_PROVIDER_UNKNOWN instead of falling back to ordinary Authenticode.
            std::ptr::write_unaligned(blob_info.as_mut_ptr().add(0x28) as *mut *const u8, pkcs7.as_ptr());
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    DRIVER_ACTION_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                TRUST_E_PROVIDER_UNKNOWN
            );

            // WTD_CHOICE_CERT with WINTRUST_ACTION_GENERIC_CERT_VERIFY, GENERIC_CHAIN_VERIFY, and HTTPSPROV_ACTION
            let pfx_blob = CryptDataBlob {
                cb_data: crate::misc::crypt32::tests::test_vectors::PFX_PBES2_AES256.len() as u32,
                pb_data: crate::misc::crypt32::tests::test_vectors::PFX_PBES2_AES256.as_ptr(),
            };
            let pw = crate::strings::to_wide_z("testpass123");
            let st = crate::misc::crypt32::PFXImportCertStore(&pfx_blob as *const _ as *const c_void, pw.as_ptr(), 0);
            let cert = crate::misc::crypt32::CertEnumCertificatesInStore(st, std::ptr::null_mut());
            let mut cert_info = [0u8; 0x38];
            std::ptr::write_unaligned(cert_info.as_mut_ptr() as *mut u32, cert_info.len() as u32);
            std::ptr::write_unaligned(cert_info.as_mut_ptr().add(0x10) as *mut *const c_void, cert);
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x20) as *mut u32, WTD_CHOICE_CERT);
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x28) as *mut *const u8, cert_info.as_ptr());
            for action in [
                &WINTRUST_ACTION_GENERIC_CERT_VERIFY,
                &WINTRUST_ACTION_GENERIC_CHAIN_VERIFY,
                &HTTPSPROV_ACTION,
            ] {
                assert_eq!(
                    WinVerifyTrust(
                        std::ptr::null_mut(),
                        action.as_ptr(),
                        wtd.as_mut_ptr() as *mut c_void,
                    ),
                    crate::misc::crypt32::CERT_E_UNTRUSTEDROOT as i32
                );
            }

            // HTTPSPROV_ACTION forwards WINTRUST_DATA.pPolicyCallbackData (HTTPSPolicyCallbackData)
            // into CERT_CHAIN_POLICY_SSL so mismatched or missing server names are rejected.
            let tls_leaf_der = crate::misc::secur32::tests::TEST_LEAF_DER;
            let tls_cert = crate::misc::crypt32::CertCreateCertificateContext(
                X509_ASN_ENCODING,
                tls_leaf_der.as_ptr(),
                tls_leaf_der.len() as u32,
            );
            assert!(!tls_cert.is_null());
            std::ptr::write_unaligned(cert_info.as_mut_ptr().add(0x10) as *mut *const c_void, tls_cert);
            let wrong_host_w = crate::strings::to_wide_z("wrong.example.com");
            let right_host_w = crate::strings::to_wide_z("test.example.com");
            let mut https_para = [0u8; 0x18];
            std::ptr::write_unaligned(https_para.as_mut_ptr() as *mut u32, 0x18u32); // cbStruct
            std::ptr::write_unaligned(https_para.as_mut_ptr().add(0x04) as *mut u32, 2u32); // AUTHTYPE_SERVER
            std::ptr::write_unaligned(https_para.as_mut_ptr().add(0x08) as *mut u32, 0x0100u32); // SECURITY_FLAG_IGNORE_UNKNOWN_CA
            std::ptr::write_unaligned(
                https_para.as_mut_ptr().add(0x10) as *mut *const u16,
                wrong_host_w.as_ptr(),
            );
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x08) as *mut *const c_void,
                https_para.as_ptr() as *const c_void,
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                0x800B_010Fu32 as i32, // CERT_E_CN_NO_MATCH
            );
            // Missing server name with AUTHTYPE_SERVER (without IGNORE_CN) is also rejected
            std::ptr::write_unaligned(
                https_para.as_mut_ptr().add(0x10) as *mut *const u16,
                std::ptr::null(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                0x800B_010Fu32 as i32, // CERT_E_CN_NO_MATCH
            );
            // Matching SAN hostname passes when revocation check is WTD_REVOKE_NONE
            std::ptr::write_unaligned(
                https_para.as_mut_ptr().add(0x10) as *mut *const u16,
                right_host_w.as_ptr(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS,
            );

            // HTTPSPROV_ACTION with AUTHTYPE_CLIENT (1) MUST enforce clientAuth EKU:
            // TEST_LEAF_DER (serverAuth-only) is rejected with CERT_E_WRONG_USAGE even with
            // SECURITY_FLAG_IGNORE_UNKNOWN_CA, while TEST_MTLS_CLIENT_DER (clientAuth EKU) passes.
            std::ptr::write_unaligned(https_para.as_mut_ptr().add(0x04) as *mut u32, 1u32); // AUTHTYPE_CLIENT
            std::ptr::write_unaligned(
                https_para.as_mut_ptr().add(0x10) as *mut *const u16,
                std::ptr::null(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                crate::misc::crypt32::CERT_E_WRONG_USAGE as i32,
                "AUTHTYPE_CLIENT must reject serverAuth-only certificate"
            );
            let client_auth_cert = crate::misc::crypt32::CertCreateCertificateContext(
                X509_ASN_ENCODING,
                crate::misc::secur32::tests::TEST_MTLS_CLIENT_DER.as_ptr(),
                crate::misc::secur32::tests::TEST_MTLS_CLIENT_DER.len() as u32,
            );
            assert!(!client_auth_cert.is_null());
            std::ptr::write_unaligned(
                cert_info.as_mut_ptr().add(0x10) as *mut *const c_void,
                client_auth_cert,
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS,
                "AUTHTYPE_CLIENT should accept clientAuth certificate when unknown CA is ignored"
            );
            crate::misc::crypt32::CertFreeCertificateContext(client_auth_cert);
            // Restore AUTHTYPE_SERVER + tls_cert for subsequent checks below.
            std::ptr::write_unaligned(https_para.as_mut_ptr().add(0x04) as *mut u32, 2u32);
            std::ptr::write_unaligned(
                https_para.as_mut_ptr().add(0x10) as *mut *const u16,
                right_host_w.as_ptr(),
            );
            std::ptr::write_unaligned(
                cert_info.as_mut_ptr().add(0x10) as *mut *const c_void,
                tls_cert,
            );

            // WINTRUST_ACTION_GENERIC_CHAIN_VERIFY with WTD_GENERIC_CHAIN_POLICY_DATA MUST invoke
            // pfnPolicyCallback with the built signer chain and return its HRESULT (rather than
            // bypassing it with CERT_CHAIN_POLICY_BASE).
            struct GenericPolicyCtx {
                called: usize,
                saw_chain: bool,
                saw_prov_data: bool,
                expected_wtd: *mut c_void,
                return_hr: i32,
            }
            unsafe extern "C" fn test_generic_chain_policy_cb(
                prov_data: u64,
                _step_err: u64,
                _reg_settings: u64,
                c_signer: u64,
                rgp_signer: u64,
                pv_policy_arg: u64,
                _a7: u64,
                _a8: u64,
            ) -> u64 {
                let ctx = &mut *(pv_policy_arg as *mut GenericPolicyCtx);
                ctx.called += 1;
                if prov_data != 0 {
                    let pd = &*(prov_data as *const CryptProviderData);
                    ctx.saw_prov_data = pd.cb_struct == std::mem::size_of::<CryptProviderData>() as u32
                        && pd.p_wintrust_data == ctx.expected_wtd
                        && !pd.pg_action_id.is_null()
                        && pd.cs_signers == 1
                        && !pd.pas_signers.is_null()
                        && !(*pd.pas_signers).p_chain_context.is_null();
                }
                if c_signer == 1 && rgp_signer != 0 {
                    let sptr = *(rgp_signer as *const *const WtdGenericChainPolicySignerInfo);
                    if !sptr.is_null() {
                        let sinfo = std::ptr::read_unaligned(sptr);
                        ctx.saw_chain = sinfo.cb_size == 0x30 && !sinfo.p_chain_context.is_null();
                    }
                }
                ctx.return_hr as u32 as u64
            }
            // Undersized WTD_GENERIC_CHAIN_POLICY_DATA (< 0x28 on Win64) returns E_INVALIDARG.
            let mut bad_gc = [0u8; 0x28];
            std::ptr::write_unaligned(bad_gc.as_mut_ptr() as *mut u32, 0x10u32);
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x08) as *mut *const c_void,
                bad_gc.as_ptr() as *const c_void,
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                0x8007_0057u32 as i32, // E_INVALIDARG
            );
            // SDK-sized (0x28 bytes) WTD_GENERIC_CHAIN_POLICY_DATA with pSignerChainInfo
            // (WTD_GENERIC_CHAIN_POLICY_CREATE_INFO, 0x28 bytes), pfnPolicyCallback @ +0x18,
            // and pvPolicyArg @ +0x20.
            let mut signer_chain_info = [0u8; 0x28];
            std::ptr::write_unaligned(signer_chain_info.as_mut_ptr() as *mut u32, 0x28u32);
            std::ptr::write_unaligned(
                signer_chain_info.as_mut_ptr().add(0x18) as *mut u32,
                CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT,
            );
            let mut cb_ctx = GenericPolicyCtx {
                called: 0,
                saw_chain: false,
                saw_prov_data: false,
                expected_wtd: wtd.as_mut_ptr() as *mut c_void,
                return_hr: 0x800B_0111u32 as i32, // TRUST_E_EXPLICIT_DISTRUST
            };
            let mut gc_para = [0u8; 0x28];
            std::ptr::write_unaligned(gc_para.as_mut_ptr() as *mut u32, 0x28u32);
            std::ptr::write_unaligned(
                gc_para.as_mut_ptr().add(0x08) as *mut *const u8,
                signer_chain_info.as_ptr(),
            );
            std::ptr::write_unaligned(
                gc_para.as_mut_ptr().add(0x18) as *mut usize,
                test_generic_chain_policy_cb as *const () as usize,
            );
            std::ptr::write_unaligned(
                gc_para.as_mut_ptr().add(0x20) as *mut *mut c_void,
                &mut cb_ctx as *mut GenericPolicyCtx as *mut c_void,
            );
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x08) as *mut *const c_void,
                gc_para.as_ptr() as *const c_void,
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                0x800B_0111u32 as i32,
                "restrictive WTD_GENERIC_CHAIN_POLICY_DATA callback must be invoked and honored"
            );
            assert_eq!(cb_ctx.called, 1);
            assert!(cb_ctx.saw_chain, "callback must receive non-null PCCERT_CHAIN_CONTEXT");
            assert!(
                cb_ctx.saw_prov_data,
                "callback must receive valid non-null PCRYPT_PROVIDER_DATA even without WTD_STATEACTION_VERIFY"
            );
            // Accepting callback returns ERROR_SUCCESS.
            cb_ctx.return_hr = ERROR_SUCCESS;
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS,
            );
            assert_eq!(cb_ctx.called, 2);
            // Without pfnPolicyCallback, pSignerChainInfo->dwFlags (revocation) is still applied.
            std::ptr::write_unaligned(gc_para.as_mut_ptr().add(0x18) as *mut usize, 0usize);
            assert_ne!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS,
                "pSignerChainInfo revocation flags must fail closed when callback is NULL"
            );
            // With an exclusive-root hChainEngine trusting TEST_CA_DER (so the chain itself is
            // trusted), pSignerChainInfo->pChainPara MUST still enforce RequestedUsage,
            // RequestedIssuancePolicy, and pStrongSignPara!
            let excl_root_st = crate::misc::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
            assert_eq!(
                crate::misc::crypt32::CertAddEncodedCertificateToStore(
                    excl_root_st,
                    X509_ASN_ENCODING,
                    crate::misc::secur32::tests::TEST_CA_DER.as_ptr(),
                    crate::misc::secur32::tests::TEST_CA_DER.len() as u32,
                    4,
                    std::ptr::null_mut(),
                ),
                1
            );
            let mut eng_cfg = [0u8; 0x60];
            std::ptr::write_unaligned(eng_cfg.as_mut_ptr() as *mut u32, eng_cfg.len() as u32);
            std::ptr::write_unaligned(
                eng_cfg.as_mut_ptr().add(0x40) as *mut *mut c_void,
                excl_root_st,
            );
            let mut excl_eng: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::crypt32::CertCreateCertificateChainEngine(
                    eng_cfg.as_ptr() as *const c_void,
                    &mut excl_eng,
                ),
                1
            );
            std::ptr::write_unaligned(
                signer_chain_info.as_mut_ptr().add(0x08) as *mut *mut c_void,
                excl_eng,
            );
            std::ptr::write_unaligned(signer_chain_info.as_mut_ptr().add(0x18) as *mut u32, 0u32);
            // Baseline without pChainPara passes via exclusive-root hChainEngine.
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS
            );
            // 1. RequestedUsage requiring codeSigning (1.3.6.1.5.5.7.3.3) on serverAuth-only
            //    TEST_LEAF_DER MUST fail with CERT_E_WRONG_USAGE.
            let code_sign_oid = b"1.3.6.1.5.5.7.3.3\0";
            let usage_ptrs = [code_sign_oid.as_ptr()];
            let mut chain_para = [0u8; 0x60];
            std::ptr::write_unaligned(chain_para.as_mut_ptr() as *mut u32, 0x60u32);
            std::ptr::write_unaligned(chain_para.as_mut_ptr().add(0x08) as *mut u32, 0u32); // AND
            std::ptr::write_unaligned(chain_para.as_mut_ptr().add(0x10) as *mut u32, 1u32);
            std::ptr::write_unaligned(
                chain_para.as_mut_ptr().add(0x18) as *mut *const *const u8,
                usage_ptrs.as_ptr(),
            );
            std::ptr::write_unaligned(
                signer_chain_info.as_mut_ptr().add(0x10) as *mut *const u8,
                chain_para.as_ptr(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                crate::misc::crypt32::CERT_E_WRONG_USAGE as i32,
                "pChainPara RequestedUsage must reject disallowed EKU"
            );
            // Matching RequestedUsage (serverAuth 1.3.6.1.5.5.7.3.1) passes.
            let srv_auth_oid = b"1.3.6.1.5.5.7.3.1\0";
            let srv_usage_ptrs = [srv_auth_oid.as_ptr()];
            std::ptr::write_unaligned(
                chain_para.as_mut_ptr().add(0x18) as *mut *const *const u8,
                srv_usage_ptrs.as_ptr(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS
            );
            // 2. RequestedIssuancePolicy missing on leaf MUST fail with CERT_E_INVALID_POLICY.
            let req_pol_oid = b"1.2.3.4.5.6\0";
            let pol_ptrs = [req_pol_oid.as_ptr()];
            std::ptr::write_unaligned(chain_para.as_mut_ptr().add(0x20) as *mut u32, 0u32);
            std::ptr::write_unaligned(chain_para.as_mut_ptr().add(0x28) as *mut u32, 1u32);
            std::ptr::write_unaligned(
                chain_para.as_mut_ptr().add(0x30) as *mut *const *const u8,
                pol_ptrs.as_ptr(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                crate::misc::crypt32::CERT_E_INVALID_POLICY as i32,
                "pChainPara RequestedIssuancePolicy must reject missing policy"
            );
            // 3. pStrongSignPara: SDK CERT_STRONG_SIGN_PARA_OS_CURRENT
            //    (dwInfoChoice = CERT_STRONG_SIGN_OID_INFO_CHOICE = 2,
            //     pszOID = szOID_CERT_STRONG_SIGN_OS_CURRENT = "1.3.6.1.4.1.311.72.1.2")
            //    must succeed on our SHA-256-signed chain.
            std::ptr::write_unaligned(chain_para.as_mut_ptr().add(0x28) as *mut u32, 0u32);
            let strong_oid = b"1.3.6.1.4.1.311.72.1.2\0";
            let mut strong_ss = [0u8; 0x10];
            std::ptr::write_unaligned(strong_ss.as_mut_ptr() as *mut u32, 0x10u32);
            std::ptr::write_unaligned(strong_ss.as_mut_ptr().add(4) as *mut u32, 2u32); // CERT_STRONG_SIGN_OID_INFO_CHOICE
            std::ptr::write_unaligned(
                strong_ss.as_mut_ptr().add(8) as *mut *const u8,
                strong_oid.as_ptr(),
            );
            std::ptr::write_unaligned(
                chain_para.as_mut_ptr().add(0x50) as *mut *const u8,
                strong_ss.as_ptr(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS,
                "CERT_STRONG_SIGN_PARA_OS_CURRENT (dwInfoChoice=2) must succeed on SHA-256 chain"
            );
            // Unknown strong-sign OID under CERT_STRONG_SIGN_OID_INFO_CHOICE (2) must fail closed.
            let bad_oid = b"1.2.3.4.5.999\0";
            std::ptr::write_unaligned(
                strong_ss.as_mut_ptr().add(8) as *mut *const u8,
                bad_oid.as_ptr(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                CERT_E_CHAINING,
                "unrecognized strong-sign OID must fail closed"
            );
            // CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE (dwInfoChoice=1) must fail closed
            // without treating pSerializedInfo as an ANSI OID string.
            let fake_serialized_struct = [0xDEu8, 0xAD, 0xBE, 0xEF, 0x01, 0x02, 0x03, 0x04];
            std::ptr::write_unaligned(strong_ss.as_mut_ptr().add(4) as *mut u32, 1u32); // CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE
            std::ptr::write_unaligned(
                strong_ss.as_mut_ptr().add(8) as *mut *const u8,
                fake_serialized_struct.as_ptr(),
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                CERT_E_CHAINING,
                "CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE (dwInfoChoice=1) must fail closed"
            );
            crate::misc::crypt32::CertFreeCertificateChainEngine(excl_eng);
            crate::misc::crypt32::CertCloseStore(excl_root_st, 0);
            // Undersized pSignerChainInfo (< 0x28) returns E_INVALIDARG.
            std::ptr::write_unaligned(signer_chain_info.as_mut_ptr() as *mut u32, 0x10u32);
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                0x8007_0057u32 as i32,
            );
            // Restore HTTPS policy callback pointer for subsequent revocation checks below.
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x08) as *mut *const c_void,
                https_para.as_ptr() as *const c_void,
            );
            // Requested revocation checks (via either fdwRevocationChecks or dwProvFlags) MUST
            // fail closed when positive revocation status cannot be obtained, rather than returning
            // ERROR_SUCCESS.
            assert_eq!(map_wintrust_revocation_flags(WTD_REVOKE_NONE, 0), 0);
            assert_eq!(
                map_wintrust_revocation_flags(WTD_REVOKE_WHOLECHAIN, 0),
                CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT
            );
            assert_eq!(
                map_wintrust_revocation_flags(WTD_REVOKE_WHOLECHAIN, WTD_REVOCATION_CHECK_NONE),
                0
            );
            assert_eq!(
                map_wintrust_revocation_flags(
                    WTD_REVOKE_NONE,
                    WTD_REVOCATION_CHECK_END_CERT | WTD_CACHE_ONLY_URL_RETRIEVAL
                ),
                CERT_CHAIN_REVOCATION_CHECK_END_CERT
                    | CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL
                    | CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY
            );
            assert_eq!(
                map_wintrust_revocation_flags(WTD_REVOKE_NONE, WTD_REVOCATION_CHECK_CHAIN),
                CERT_CHAIN_REVOCATION_CHECK_CHAIN
            );
            assert_eq!(
                map_wintrust_revocation_flags(
                    WTD_REVOKE_NONE,
                    WTD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT
                ),
                CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT
            );

            // 1) fdwRevocationChecks = WTD_REVOKE_WHOLECHAIN -> fails closed
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x1C) as *mut u32,
                WTD_REVOKE_WHOLECHAIN,
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                crate::misc::crypt32::CRYPT_E_REVOCATION_OFFLINE as i32,
            );
            // 2) dwProvFlags = WTD_REVOCATION_CHECK_NONE disables revocation checking
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x48) as *mut u32,
                WTD_REVOCATION_CHECK_NONE,
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS,
            );
            // 3) Each WTD_REVOCATION_CHECK_* mode in dwProvFlags fails closed when offline/unknown
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x1C) as *mut u32, WTD_REVOKE_NONE);
            for prov_flag in [
                WTD_REVOCATION_CHECK_END_CERT,
                WTD_REVOCATION_CHECK_CHAIN,
                WTD_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT,
                WTD_REVOCATION_CHECK_END_CERT | WTD_CACHE_ONLY_URL_RETRIEVAL,
            ] {
                std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x48) as *mut u32, prov_flag);
                assert_eq!(
                    WinVerifyTrust(
                        std::ptr::null_mut(),
                        HTTPSPROV_ACTION.as_ptr(),
                        wtd.as_mut_ptr() as *mut c_void,
                    ),
                    crate::misc::crypt32::CRYPT_E_REVOCATION_OFFLINE as i32,
                    "dwProvFlags={prov_flag:#x} must fail closed when revocation is unavailable"
                );
            }
            // Reset revocation fields before subsequent tests
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x1C) as *mut u32, WTD_REVOKE_NONE);
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x48) as *mut u32, 0u32);
            // Truncated HTTPSPolicyCallbackData -> E_INVALIDARG
            std::ptr::write_unaligned(https_para.as_mut_ptr() as *mut u32, 8u32);
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    HTTPSPROV_ACTION.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                0x8007_0057u32 as i32,
            );
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x08) as *mut *const c_void,
                std::ptr::null(),
            );
            std::ptr::write_unaligned(cert_info.as_mut_ptr().add(0x10) as *mut *const c_void, cert);
            crate::misc::crypt32::CertFreeCertificateContext(tls_cert);

            // WTD_CHOICE_SIGNER: an unsigned/forged CMSG_SIGNER_INFO (only issuer + serial,
            // empty EncryptedHash) MUST fail cryptographically with TRUST_E_BAD_DIGEST before
            // reaching chain-policy evaluation.
            let cctx = std::ptr::read_unaligned(cert as *const CertContext);
            let ci = std::ptr::read_unaligned(cctx.p_cert_info);
            let mut forged_si: CmsgSignerInfo = std::mem::zeroed();
            forged_si.issuer = ci.issuer;
            forged_si.serial_number = ci.serial_number;
            let stores = [st];
            let mut sgnr_info = [0u8; 0x28];
            std::ptr::write_unaligned(sgnr_info.as_mut_ptr() as *mut u32, sgnr_info.len() as u32);
            std::ptr::write_unaligned(sgnr_info.as_mut_ptr().add(0x10) as *mut *const CmsgSignerInfo, &forged_si);
            std::ptr::write_unaligned(sgnr_info.as_mut_ptr().add(0x18) as *mut u32, 1);
            std::ptr::write_unaligned(sgnr_info.as_mut_ptr().add(0x20) as *mut *const *mut c_void, stores.as_ptr());
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x20) as *mut u32, WTD_CHOICE_SIGNER);
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x28) as *mut *const u8, sgnr_info.as_ptr());
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                TRUST_E_BAD_DIGEST
            );

            // Real cryptographically signed CMSG_SIGNER_INFO extracted from PKCS7_SIGNED_DER
            // passes signature verification and reaches chain-policy check (CERT_E_UNTRUSTEDROOT),
            // while flipping a bit in its EncryptedHash fails earlier with TRUST_E_BAD_DIGEST.
            let h_si_msg = crate::misc::crypt32::CryptMsgOpenToDecode(
                X509_ASN_ENCODING | 0x0001_0000,
                0,
                0,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!h_si_msg.is_null());
            assert_eq!(
                crate::misc::crypt32::CryptMsgUpdate(h_si_msg, pkcs7.as_ptr(), pkcs7.len() as u32, 1),
                1
            );
            let mut si_sz = 0u32;
            assert_eq!(
                crate::misc::crypt32::CryptMsgGetParam(h_si_msg, 6, 0, std::ptr::null_mut(), &mut si_sz),
                1
            );
            let mut real_si_buf = vec![0u8; si_sz as usize];
            assert_eq!(
                crate::misc::crypt32::CryptMsgGetParam(
                    h_si_msg,
                    6,
                    0,
                    real_si_buf.as_mut_ptr() as *mut c_void,
                    &mut si_sz,
                ),
                1
            );
            std::ptr::write_unaligned(
                sgnr_info.as_mut_ptr().add(0x10) as *mut *const CmsgSignerInfo,
                real_si_buf.as_ptr() as *const CmsgSignerInfo,
            );
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                crate::misc::crypt32::CERT_E_UNTRUSTEDROOT as i32
            );
            // Flip 1 byte in real EncryptedHash -> TRUST_E_BAD_DIGEST
            let real_si_hdr = std::ptr::read_unaligned(real_si_buf.as_ptr() as *const CmsgSignerInfo);
            let hash_off = (real_si_hdr.encrypted_hash.pb_data as usize) - (real_si_buf.as_ptr() as usize);
            real_si_buf[hash_off] ^= 0x01;
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                TRUST_E_BAD_DIGEST
            );
            crate::misc::crypt32::CryptMsgClose(h_si_msg);

            // WTD_CHOICE_CATALOG: validates CTL structure, member tag & member digest, AND signer chain
            let tmp_dir = std::env::temp_dir().join(format!("wintrust_cat_{}", std::process::id()));
            std::fs::create_dir_all(&tmp_dir).unwrap();
            let tmp_non_ctl = tmp_dir.join("notctl.cat");
            std::fs::write(&tmp_non_ctl, pkcs7).unwrap();
            let tmp_ctl = tmp_dir.join("real.cat");
            let ctl_signed = crate::misc::crypt32::tests::test_vectors::CATALOG_CTL_SIGNED_DER;
            let member_payload = crate::misc::crypt32::tests::test_vectors::CATALOG_MEMBER_PAYLOAD;
            std::fs::write(&tmp_ctl, ctl_signed).unwrap();
            let tmp_member = tmp_dir.join("member.bin");
            std::fs::write(&tmp_member, member_payload).unwrap();
            let tmp_bad_member = tmp_dir.join("tampered.bin");
            std::fs::write(&tmp_bad_member, b"tampered-payload").unwrap();
            crate::path::add_mount("T:\\", &tmp_dir);

            let non_ctl_w = crate::strings::to_wide_z("T:\\notctl.cat");
            let ctl_w = crate::strings::to_wide_z("T:\\real.cat");
            let tag_ok_w = crate::strings::to_wide_z("MEMBER01");
            let tag_bad_w = crate::strings::to_wide_z("OTHER99");
            let member_w = crate::strings::to_wide_z("T:\\member.bin");
            let bad_member_w = crate::strings::to_wide_z("T:\\tampered.bin");
            let good_hash = Digest::Sha256.hash(member_payload);
            let mut wrong_hash = good_hash.clone();
            wrong_hash[0] ^= 0xFF;

            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x20) as *mut u32, WTD_CHOICE_CATALOG);
            // 1) Non-CTL PKCS#7 is rejected with TRUST_E_SUBJECT_FORM_UNKNOWN
            let mut cat_info = [0u8; 0x38];
            std::ptr::write_unaligned(cat_info.as_mut_ptr() as *mut u32, cat_info.len() as u32);
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x08) as *mut *const u16, non_ctl_w.as_ptr());
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x28) as *mut *const u8, cat_info.as_ptr());
            assert_eq!(
                WinVerifyTrust(std::ptr::null_mut(), WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(), wtd.as_mut_ptr() as *mut c_void),
                TRUST_E_SUBJECT_FORM_UNKNOWN
            );

            // 2) Real CTL with NO member hash/file -> TRUST_E_NOSIGNATURE
            cat_info.fill(0);
            std::ptr::write_unaligned(cat_info.as_mut_ptr() as *mut u32, cat_info.len() as u32);
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x08) as *mut *const u16, ctl_w.as_ptr());
            assert_eq!(
                WinVerifyTrust(std::ptr::null_mut(), WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(), wtd.as_mut_ptr() as *mut c_void),
                TRUST_E_NOSIGNATURE
            );

            // 3) Real CTL with matching member tag + WRONG hash -> TRUST_E_BAD_DIGEST
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x10) as *mut *const u16, tag_ok_w.as_ptr());
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x28) as *mut *const u8, wrong_hash.as_ptr());
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x30) as *mut u32, wrong_hash.len() as u32);
            assert_eq!(
                WinVerifyTrust(std::ptr::null_mut(), WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(), wtd.as_mut_ptr() as *mut c_void),
                TRUST_E_BAD_DIGEST
            );

            // 4) Real CTL with unknown member tag -> TRUST_E_NOSIGNATURE
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x10) as *mut *const u16, tag_bad_w.as_ptr());
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x28) as *mut *const u8, good_hash.as_ptr());
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x30) as *mut u32, good_hash.len() as u32);
            assert_eq!(
                WinVerifyTrust(std::ptr::null_mut(), WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(), wtd.as_mut_ptr() as *mut c_void),
                TRUST_E_NOSIGNATURE
            );

            // 5) Real CTL with matching member tag + GOOD hash -> passes membership check and
            // reaches signer chain trust check (CERT_E_UNTRUSTEDROOT for self-signed test cert)
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x10) as *mut *const u16, tag_ok_w.as_ptr());
            assert_eq!(
                WinVerifyTrust(std::ptr::null_mut(), WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(), wtd.as_mut_ptr() as *mut c_void),
                crate::misc::crypt32::CERT_E_UNTRUSTEDROOT as i32
            );

            // 6) Real CTL with pcwszMemberFilePath: tampered member file -> TRUST_E_BAD_DIGEST,
            // genuine member file -> passes membership check (reaches CERT_E_UNTRUSTEDROOT)
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x28) as *mut *const u8, std::ptr::null());
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x30) as *mut u32, 0);
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x18) as *mut *const u16, bad_member_w.as_ptr());
            assert_eq!(
                WinVerifyTrust(std::ptr::null_mut(), WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(), wtd.as_mut_ptr() as *mut c_void),
                TRUST_E_BAD_DIGEST
            );
            std::ptr::write_unaligned(cat_info.as_mut_ptr().add(0x18) as *mut *const u16, member_w.as_ptr());
            assert_eq!(
                WinVerifyTrust(std::ptr::null_mut(), WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(), wtd.as_mut_ptr() as *mut c_void),
                crate::misc::crypt32::CERT_E_UNTRUSTEDROOT as i32
            );
            let _ = std::fs::remove_dir_all(&tmp_dir);

            crate::misc::crypt32::CertFreeCertificateContext(cert);
            crate::misc::crypt32::CertCloseStore(st, 0);

            // Binary 20-byte SHA-1 subjectIdentifier matches uppercase/lowercase hex member tag
            let bin_sha1: [u8; 20] = [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x10, 0x20,
                0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0,
            ];
            assert!(ctl_subject_id_matches_tag(
                &bin_sha1,
                &[],
                "0123456789ABCDEF102030405060708090A0B0C0"
            ));
            assert!(ctl_subject_id_matches_tag(
                &bin_sha1,
                &[],
                "0123456789abcdef102030405060708090a0b0c0"
            ));
            assert!(!ctl_subject_id_matches_tag(
                &bin_sha1,
                &[],
                "FF23456789ABCDEF102030405060708090A0B0C0"
            ));

            // RFC 5652 §5.6: mutating encapContentInfo.eContentType without changing the
            // content or signature MUST fail verify_standalone_pkcs7 (content-type mismatch).
            let mut tampered_ctl = ctl_signed.to_vec();
            let oid_data: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x01];
            let pos = tampered_ctl
                .windows(OID_CTL.len())
                .position(|w| w == OID_CTL)
                .expect("CATALOG_CTL_SIGNED_DER must contain OID_CTL in encapContentInfo");
            tampered_ctl[pos..pos + OID_CTL.len()].copy_from_slice(oid_data);
            assert_eq!(
                verify_standalone_pkcs7(&tampered_ctl, None).err(),
                Some(TRUST_E_BAD_DIGEST)
            );

            // RFC 5652 §5.3: attribute-free SignedData succeeds for id-data, but MUST be
            // rejected when relabeled as a non-DATA content type (e.g. szOID_CTL).
            let noattr = crate::misc::crypt32::tests::test_vectors::PKCS7_NOATTR_DATA_DER;
            assert!(verify_standalone_pkcs7(noattr, None).is_ok());
            let mut relabeled_noattr = noattr.to_vec();
            let dpos = relabeled_noattr
                .windows(OID_DATA.len())
                .position(|w| w == OID_DATA)
                .expect("PKCS7_NOATTR_DATA_DER must contain OID_DATA");
            relabeled_noattr[dpos..dpos + OID_DATA.len()].copy_from_slice(OID_CTL);
            assert_eq!(
                verify_standalone_pkcs7(&relabeled_noattr, None).err(),
                Some(TRUST_E_BAD_DIGEST)
            );

            // Invalid union choice returns TRUST_E_SUBJECT_FORM_UNKNOWN
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x20) as *mut u32, 99);
            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_TRUSTPROVIDER_TEST.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                TRUST_E_SUBJECT_FORM_UNKNOWN
            );
        }
    }

    #[test]
    fn generic_chain_verify_callback_excludes_unverified_embedded_certs() {
        fn der_tlv(tag: u8, body: &[u8]) -> Vec<u8> {
            let mut out = vec![tag];
            if body.len() < 0x80 {
                out.push(body.len() as u8);
            } else if body.len() < 0x100 {
                out.push(0x81);
                out.push(body.len() as u8);
            } else {
                out.push(0x82);
                out.push((body.len() >> 8) as u8);
                out.push((body.len() & 0xFF) as u8);
            }
            out.extend_from_slice(body);
            out
        }

        // Take a genuinely signed standalone PKCS#7 blob and append an unrelated CA certificate
        // (`TEST_CA_DER`) into SignedData.certificates (tag 0xA0) without touching the signature.
        let orig = crate::misc::crypt32::tests::test_vectors::PKCS7_SIGNED_DER;
        let injected_ca = crate::misc::secur32::tests::TEST_CA_DER;
        let ci = der_children(orig).unwrap()[0];
        let ci_kids = der_children(ci.val).unwrap();
        let sd = der_children(ci_kids[1].val).unwrap()[0];
        let sd_kids = der_children(sd.val).unwrap();
        let mut rebuilt_sd = Vec::new();
        let mut found_certs = false;
        for child in sd_kids {
            if child.tag == 0xA0 {
                found_certs = true;
                let mut new_certs = child.val.to_vec();
                new_certs.extend_from_slice(injected_ca);
                rebuilt_sd.extend_from_slice(&der_tlv(0xA0, &new_certs));
            } else {
                rebuilt_sd.extend_from_slice(child.raw);
            }
        }
        assert!(found_certs);
        let mut rebuilt_ci = Vec::new();
        rebuilt_ci.extend_from_slice(ci_kids[0].raw);
        rebuilt_ci.extend_from_slice(&der_tlv(0xA0, &der_tlv(0x30, &rebuilt_sd)));
        let stuffed_p7 = der_tlv(0x30, &rebuilt_ci);

        // Callback inspects every element in `rgpChain[0]->rgpElement[0..cElement]`.
        struct PinCbCtx {
            called: usize,
            seen_elements: Vec<Vec<u8>>,
        }
        unsafe extern "C" fn inspect_chain_cb(
            _prov_data: u64,
            _step_err: u64,
            _reg_settings: u64,
            c_signer: u64,
            rgp_signer: u64,
            pv_policy_arg: u64,
            _a7: u64,
            _a8: u64,
        ) -> u64 {
            let ctx = &mut *(pv_policy_arg as *mut PinCbCtx);
            ctx.called += 1;
            if c_signer == 1 && rgp_signer != 0 {
                let sptr = *(rgp_signer as *const *const WtdGenericChainPolicySignerInfo);
                if !sptr.is_null() {
                    let sinfo = std::ptr::read_unaligned(sptr);
                    if !sinfo.p_chain_context.is_null() {
                        let cc = &*(sinfo.p_chain_context as *const CertChainContext);
                        if cc.c_chain > 0 && !cc.rgp_chain.is_null() {
                            let simple = &*(*cc.rgp_chain);
                            for i in 0..simple.c_element as usize {
                                let el = &*(*simple.rgp_element.add(i));
                                let cert = &*el.p_cert_context;
                                let der = std::slice::from_raw_parts(
                                    cert.pb_cert_encoded,
                                    cert.cb_cert_encoded as usize,
                                );
                                ctx.seen_elements.push(der.to_vec());
                            }
                        }
                    }
                }
            }
            ERROR_SUCCESS as u32 as u64
        }

        unsafe {
            let mut cb_ctx = PinCbCtx {
                called: 0,
                seen_elements: Vec::new(),
            };
            let mut gc_para = [0u8; 0x28];
            std::ptr::write_unaligned(gc_para.as_mut_ptr() as *mut u32, 0x28u32);
            std::ptr::write_unaligned(
                gc_para.as_mut_ptr().add(0x18) as *mut usize,
                inspect_chain_cb as *const () as usize,
            );
            std::ptr::write_unaligned(
                gc_para.as_mut_ptr().add(0x20) as *mut *mut c_void,
                &mut cb_ctx as *mut PinCbCtx as *mut c_void,
            );

            let mut blob_info = [0u8; 0x38];
            std::ptr::write_unaligned(blob_info.as_mut_ptr() as *mut u32, 0x38u32);
            std::ptr::write_unaligned(
                blob_info.as_mut_ptr().add(0x20) as *mut u32,
                stuffed_p7.len() as u32,
            );
            std::ptr::write_unaligned(
                blob_info.as_mut_ptr().add(0x28) as *mut *const u8,
                stuffed_p7.as_ptr(),
            );

            let mut wtd = [0u8; 0x58];
            std::ptr::write_unaligned(wtd.as_mut_ptr() as *mut u32, 0x58u32);
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x08) as *mut *const c_void,
                gc_para.as_ptr() as *const c_void,
            );
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x20) as *mut u32, WTD_CHOICE_BLOB);
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x28) as *mut *const u8,
                blob_info.as_ptr(),
            );

            assert_eq!(
                WinVerifyTrust(
                    std::ptr::null_mut(),
                    WINTRUST_ACTION_GENERIC_CHAIN_VERIFY.as_ptr(),
                    wtd.as_mut_ptr() as *mut c_void,
                ),
                ERROR_SUCCESS
            );
            assert_eq!(cb_ctx.called, 1);
            assert_eq!(
                cb_ctx.seen_elements.len(),
                1,
                "callback chain must contain only the verified signer path, not unrelated embedded certs"
            );
            assert!(
                !cb_ctx.seen_elements.iter().any(|d| d.as_slice() == injected_ca),
                "unrelated CA appended to SignedData.certificates must never appear in callback chain"
            );

            // Direct CertGetCertificateChain with an unrelated CA in hAdditionalStore must emit
            // ONLY [TEST_LEAF_DER, TEST_CA_DER] (verified issuer path), never the unrelated CA.
            let leaf_ctx = crate::misc::crypt32::CertCreateCertificateContext(
                X509_ASN_ENCODING,
                crate::misc::secur32::tests::TEST_LEAF_DER.as_ptr(),
                crate::misc::secur32::tests::TEST_LEAF_DER.len() as u32,
            );
            let add_st = crate::misc::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
            let unrelated_ca = crate::misc::crypt32::tests::test_vectors::RSA2048_CA_DER;
            assert_eq!(
                crate::misc::crypt32::CertAddEncodedCertificateToStore(
                    add_st,
                    X509_ASN_ENCODING,
                    unrelated_ca.as_ptr(),
                    unrelated_ca.len() as u32,
                    4,
                    std::ptr::null_mut(),
                ),
                1
            );
            let root_st = crate::misc::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
            assert_eq!(
                crate::misc::crypt32::CertAddEncodedCertificateToStore(
                    root_st,
                    X509_ASN_ENCODING,
                    injected_ca.as_ptr(),
                    injected_ca.len() as u32,
                    4,
                    std::ptr::null_mut(),
                ),
                1
            );
            let mut eng_cfg = [0u8; 0x60];
            std::ptr::write_unaligned(eng_cfg.as_mut_ptr() as *mut u32, eng_cfg.len() as u32);
            std::ptr::write_unaligned(eng_cfg.as_mut_ptr().add(0x40) as *mut *mut c_void, root_st);
            let mut eng: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::crypt32::CertCreateCertificateChainEngine(
                    eng_cfg.as_ptr() as *const c_void,
                    &mut eng,
                ),
                1
            );
            let mut ch: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::crypt32::CertGetCertificateChain(
                    eng,
                    leaf_ctx,
                    std::ptr::null(),
                    add_st,
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                    &mut ch,
                ),
                1
            );
            let cc = &*(ch as *const CertChainContext);
            let simple = &*(*cc.rgp_chain);
            assert_eq!(simple.c_element, 2, "verified path must contain [leaf, verified root]");
            let el0 = &*((*(*simple.rgp_element.add(0))).p_cert_context);
            let el1 = &*((*(*simple.rgp_element.add(1))).p_cert_context);
            assert_eq!(
                std::slice::from_raw_parts(el0.pb_cert_encoded, el0.cb_cert_encoded as usize),
                crate::misc::secur32::tests::TEST_LEAF_DER
            );
            assert_eq!(
                std::slice::from_raw_parts(el1.pb_cert_encoded, el1.cb_cert_encoded as usize),
                injected_ca
            );
            crate::misc::crypt32::CertFreeCertificateChain(ch);
            crate::misc::crypt32::CertFreeCertificateChainEngine(eng);
            crate::misc::crypt32::CertCloseStore(root_st, 0);
            crate::misc::crypt32::CertCloseStore(add_st, 0);

            // Same-SPKI forged issuer attack: copy `TEST_CA_DER`'s exact Subject DN and
            // SubjectPublicKeyInfo into a forged certificate with an attacker-chosen serial and
            // invalid signature. Even though single-step `verify_cert_signed_by_der(leaf, fake_ca)`
            // returns true, `resolve_verified_issuer_path` MUST bind emitted elements to the
            // authenticated path and NEVER expose `fake_ca` (neither when trusted nor untrusted).
            let ca_top = der_children(der_children(injected_ca).unwrap()[0].val).unwrap();
            let ca_tbs_kids = der_children(ca_top[0].val).unwrap();
            let mut fake_tbs = Vec::new();
            let mut ca_idx = 0usize;
            if ca_tbs_kids[0].tag == 0xA0 {
                fake_tbs.extend_from_slice(ca_tbs_kids[0].raw);
                ca_idx = 1;
            }
            fake_tbs.extend_from_slice(&der_tlv(0x02, &[0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE])); // forged serial
            fake_tbs.extend_from_slice(ca_tbs_kids[ca_idx + 1].raw); // sigAlg
            fake_tbs.extend_from_slice(ca_tbs_kids[ca_idx + 2].raw); // issuer
            fake_tbs.extend_from_slice(ca_tbs_kids[ca_idx + 3].raw); // validity
            fake_tbs.extend_from_slice(ca_tbs_kids[ca_idx + 4].raw); // copied subject
            fake_tbs.extend_from_slice(ca_tbs_kids[ca_idx + 5].raw); // copied SPKI
            let mut fake_cert_body = der_tlv(0x30, &fake_tbs);
            fake_cert_body.extend_from_slice(ca_top[1].raw); // sigAlg
            fake_cert_body.extend_from_slice(&der_tlv(0x03, &[0x00; 65])); // invalid dummy signature
            let fake_ca_der = der_tlv(0x30, &fake_cert_body);
            assert_ne!(fake_ca_der.as_slice(), injected_ca);
            assert!(
                crate::misc::crypt32::verify_cert_signed_by_der(
                    crate::misc::secur32::tests::TEST_LEAF_DER,
                    &fake_ca_der,
                ),
                "precondition: same-SPKI forgery verifies child signature on a naive single-step check"
            );

            // (a) Trusted case: `fake_ca_der` in hAdditionalStore + genuine `TEST_CA_DER` in exclusive
            //     root store -> chain MUST emit [TEST_LEAF_DER, TEST_CA_DER], never `fake_ca_der`.
            let fake_add_st =
                crate::misc::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
            assert_eq!(
                crate::misc::crypt32::CertAddEncodedCertificateToStore(
                    fake_add_st,
                    X509_ASN_ENCODING,
                    fake_ca_der.as_ptr(),
                    fake_ca_der.len() as u32,
                    4,
                    std::ptr::null_mut(),
                ),
                1
            );
            let genuine_root_st =
                crate::misc::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
            assert_eq!(
                crate::misc::crypt32::CertAddEncodedCertificateToStore(
                    genuine_root_st,
                    X509_ASN_ENCODING,
                    injected_ca.as_ptr(),
                    injected_ca.len() as u32,
                    4,
                    std::ptr::null_mut(),
                ),
                1
            );
            let mut eng2_cfg = [0u8; 0x60];
            std::ptr::write_unaligned(eng2_cfg.as_mut_ptr() as *mut u32, eng2_cfg.len() as u32);
            std::ptr::write_unaligned(
                eng2_cfg.as_mut_ptr().add(0x40) as *mut *mut c_void,
                genuine_root_st,
            );
            let mut eng2: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::crypt32::CertCreateCertificateChainEngine(
                    eng2_cfg.as_ptr() as *const c_void,
                    &mut eng2,
                ),
                1
            );
            let mut ch2: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::crypt32::CertGetCertificateChain(
                    eng2,
                    leaf_ctx,
                    std::ptr::null(),
                    fake_add_st,
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                    &mut ch2,
                ),
                1
            );
            let cc2 = &*(ch2 as *const CertChainContext);
            assert_eq!(cc2.trust_status.dw_error_status, 0);
            let simple2 = &*(*cc2.rgp_chain);
            assert_eq!(simple2.c_element, 2);
            let e2_1 = &*((*(*simple2.rgp_element.add(1))).p_cert_context);
            let e2_1_der =
                std::slice::from_raw_parts(e2_1.pb_cert_encoded, e2_1.cb_cert_encoded as usize);
            assert_eq!(
                e2_1_der, injected_ca,
                "same-SPKI forged issuer in hAdditionalStore must never replace the genuine trust-store root"
            );
            crate::misc::crypt32::CertFreeCertificateChain(ch2);
            crate::misc::crypt32::CertFreeCertificateChainEngine(eng2);
            crate::misc::crypt32::CertCloseStore(genuine_root_st, 0);

            // (b) Untrusted case: `fake_ca_der` in hAdditionalStore, no trusted root ->
            //     `fake_ca_der` is NOT self-signed, so chain MUST emit ONLY [TEST_LEAF_DER].
            let empty_root_st =
                crate::misc::crypt32::CertOpenStore(2 as *const u8, 0, 0, 0, std::ptr::null());
            let mut eng3_cfg = [0u8; 0x60];
            std::ptr::write_unaligned(eng3_cfg.as_mut_ptr() as *mut u32, eng3_cfg.len() as u32);
            std::ptr::write_unaligned(
                eng3_cfg.as_mut_ptr().add(0x40) as *mut *mut c_void,
                empty_root_st,
            );
            let mut eng3: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::crypt32::CertCreateCertificateChainEngine(
                    eng3_cfg.as_ptr() as *const c_void,
                    &mut eng3,
                ),
                1
            );
            let mut ch3: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::misc::crypt32::CertGetCertificateChain(
                    eng3,
                    leaf_ctx,
                    std::ptr::null(),
                    fake_add_st,
                    std::ptr::null(),
                    0,
                    std::ptr::null_mut(),
                    &mut ch3,
                ),
                1
            );
            let cc3 = &*(ch3 as *const CertChainContext);
            let simple3 = &*(*cc3.rgp_chain);
            assert_eq!(
                simple3.c_element, 1,
                "unauthenticated same-SPKI forged issuer must not be emitted on untrusted chains either"
            );
            crate::misc::crypt32::CertFreeCertificateChain(ch3);
            crate::misc::crypt32::CertFreeCertificateChainEngine(eng3);
            crate::misc::crypt32::CertCloseStore(empty_root_st, 0);
            crate::misc::crypt32::CertCloseStore(fake_add_st, 0);
            crate::misc::crypt32::CertFreeCertificateContext(leaf_ctx);
        }
    }

    #[test]
    fn standalone_cms_bounds_certs_and_signers_against_quadratic_dos() {
        fn der_tlv(tag: u8, body: &[u8]) -> Vec<u8> {
            let mut out = vec![tag];
            if body.len() < 0x80 {
                out.push(body.len() as u8);
            } else if body.len() < 0x100 {
                out.push(0x81);
                out.push(body.len() as u8);
            } else if body.len() < 0x1_0000 {
                out.push(0x82);
                out.push((body.len() >> 8) as u8);
                out.push((body.len() & 0xFF) as u8);
            } else {
                out.push(0x83);
                out.push((body.len() >> 16) as u8);
                out.push(((body.len() >> 8) & 0xFF) as u8);
                out.push((body.len() & 0xFF) as u8);
            }
            out.extend_from_slice(body);
            out
        }

        // Build a standalone CMS SignedData repeating a parseable certificate `cert_count` times
        // and an unmatched syntactically valid SignerInfo `signer_count` times.
        let build_cms = |cert_count: usize, signer_count: usize| -> Vec<u8> {
            let cert_der = crate::misc::secur32::tests::TEST_LEAF_DER;
            let mut certs_body = Vec::with_capacity(cert_der.len() * cert_count);
            for _ in 0..cert_count {
                certs_body.extend_from_slice(cert_der);
            }
            // Syntactically valid SignerInfo whose issuer+serial does NOT match `cert_der`.
            let unmatched_ias = der_tlv(
                0x30,
                &[
                    der_tlv(0x30, &[]),       // empty Issuer DN
                    der_tlv(0x02, &[0x7F]),   // serial 0x7F
                ]
                .concat(),
            );
            let sha256_alg = der_tlv(0x30, &[der_tlv(0x06, OID_SHA256), der_tlv(0x05, &[])].concat());
            let rsa_alg = der_tlv(0x30, &[der_tlv(0x06, OID_RSA_ENCRYPTION), der_tlv(0x05, &[])].concat());
            let one_si = der_tlv(
                0x30,
                &[
                    der_tlv(0x02, &[1]),
                    unmatched_ias,
                    sha256_alg.clone(),
                    rsa_alg,
                    der_tlv(0x04, &[0u8; 32]),
                ]
                .concat(),
            );
            let mut signers_body = Vec::with_capacity(one_si.len() * signer_count);
            for _ in 0..signer_count {
                signers_body.extend_from_slice(&one_si);
            }
            let eci = der_tlv(
                0x30,
                &[
                    der_tlv(0x06, OID_DATA),
                    der_tlv(0xA0, &der_tlv(0x04, b"payload")),
                ]
                .concat(),
            );
            let sd = der_tlv(
                0x30,
                &[
                    der_tlv(0x02, &[1]),
                    der_tlv(0x31, &sha256_alg),
                    eci,
                    der_tlv(0xA0, &certs_body),
                    der_tlv(0x31, &signers_body),
                ]
                .concat(),
            );
            der_tlv(
                0x30,
                &[der_tlv(0x06, OID_SIGNED_DATA), der_tlv(0xA0, &sd)].concat(),
            )
        };

        let check_blob_rejected = |blob: &[u8]| unsafe {
            let mut blob_info = [0u8; 0x38];
            std::ptr::write_unaligned(blob_info.as_mut_ptr() as *mut u32, 0x38u32);
            std::ptr::write_unaligned(
                blob_info.as_mut_ptr().add(0x20) as *mut u32,
                blob.len() as u32,
            );
            std::ptr::write_unaligned(
                blob_info.as_mut_ptr().add(0x28) as *mut *const u8,
                blob.as_ptr(),
            );
            let mut wtd = [0u8; 0x58];
            std::ptr::write_unaligned(wtd.as_mut_ptr() as *mut u32, 0x58u32);
            std::ptr::write_unaligned(wtd.as_mut_ptr().add(0x20) as *mut u32, WTD_CHOICE_BLOB);
            std::ptr::write_unaligned(
                wtd.as_mut_ptr().add(0x28) as *mut *const u8,
                blob_info.as_ptr(),
            );
            let t0 = std::time::Instant::now();
            let rc = WinVerifyTrust(
                std::ptr::null_mut(),
                WINTRUST_ACTION_GENERIC_VERIFY_V2.as_ptr(),
                wtd.as_mut_ptr() as *mut c_void,
            );
            assert_eq!(rc, TRUST_E_BAD_DIGEST);
            assert!(
                t0.elapsed() < std::time::Duration::from_secs(2),
                "CMS verification must reject unbounded/repeated certs & signers quickly"
            );
        };

        // 1. Over-cap certificate list is rejected immediately before parsing.
        let too_many_certs = build_cms(MAX_CMS_CERTS + 1, 1);
        check_blob_rejected(&too_many_certs);
        assert!(parse_pkcs7_signed_data(&too_many_certs, false).is_none());

        // 2. Over-cap SignerInfo list is rejected immediately.
        let too_many_signers = build_cms(1, MAX_CMS_SIGNERS + 1);
        check_blob_rejected(&too_many_signers);
        assert!(parse_pkcs7_signed_data(&too_many_signers, false).is_none());

        // 3. Worst-case within-cap (MAX_CMS_CERTS x MAX_CMS_SIGNERS) matches in O(C + S).
        let at_cap = build_cms(MAX_CMS_CERTS, MAX_CMS_SIGNERS);
        check_blob_rejected(&at_cap);

        // 4. Legitimate standalone signed blob still verifies cleanly via indexed lookup.
        assert!(verify_standalone_pkcs7(
            crate::misc::crypt32::tests::test_vectors::PKCS7_SIGNED_DER,
            None,
        )
        .is_ok());
    }
}

