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

const WTD_CHOICE_FILE: u32 = 1;
const WTD_STATEACTION_VERIFY: u32 = 1;
const WTD_STATEACTION_CLOSE: u32 = 2;

const WIN_CERT_TYPE_PKCS_SIGNED_DATA: u16 = 0x0002;
const X509_ASN_ENCODING: u32 = 1;

// DER OID value bytes
const OID_SIGNED_DATA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02];
const OID_SPC_INDIRECT_DATA: &[u8] = &[0x2B, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x02, 0x01, 0x04];
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

fn decode_oid_str(oid: &[u8]) -> String {
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

    // Parse certificates ([0] IMPLICIT) and signerInfos (SET).
    let mut certs: Vec<ParsedCert<'_>> = Vec::new();
    let mut signer_infos_elem: Option<DerElem<'_>> = None;
    for &elem in &sd_kids[3..] {
        if elem.tag == 0xA0 {
            for ce in der_children(elem.val).ok_or(TRUST_E_BAD_DIGEST)? {
                if let Some(c) = parse_x509_cert(ce) {
                    certs.push(c);
                }
            }
        } else if elem.tag == 0x31 {
            signer_infos_elem = Some(elem);
        }
    }
    let si_set = signer_infos_elem.ok_or(TRUST_E_BAD_DIGEST)?;
    let si_list = der_children(si_set.val).ok_or(TRUST_E_BAD_DIGEST)?;
    if si_list.is_empty() || certs.is_empty() {
        return Err(TRUST_E_BAD_DIGEST);
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
        let serial_val = ias[1].val;
        let Some(cert_idx) = certs.iter().position(|c| c.issuer_raw == issuer_raw && c.serial_val == serial_val) else {
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
            let mut found_md = false;
            for attr in der_children(aa.val).ok_or(TRUST_E_BAD_DIGEST)? {
                if attr.tag != 0x30 {
                    continue;
                }
                let ak = der_children(attr.val).ok_or(TRUST_E_BAD_DIGEST)?;
                if ak.len() >= 2 && ak[0].tag == 0x06 && ak[0].val == OID_MESSAGE_DIGEST && ak[1].tag == 0x31 {
                    let vals = der_children(ak[1].val).ok_or(TRUST_E_BAD_DIGEST)?;
                    if vals.len() == 1 && vals[0].tag == 0x04 {
                        let expected_content_md = si_digest.hash(spc_seq.val);
                        if vals[0].val != expected_content_md.as_slice() {
                            return Err(TRUST_E_BAD_DIGEST);
                        }
                        found_md = true;
                    }
                }
            }
            if !found_md {
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
            let content_hash = si_digest.hash(spc_seq.val);
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
    for _ in 0..certs.len() {
        let cur = &certs[cur_idx];
        if cur.issuer_raw == cur.subject_raw {
            break; // self-signed root
        }
        // Prefer a self-signed root cert over a cross-signed cert when both share the issuer DN.
        let parent_idx = certs
            .iter()
            .position(|c| c.subject_raw == cur.issuer_raw && c.issuer_raw == c.subject_raw && c.raw_der != cur.raw_der)
            .or_else(|| certs.iter().position(|c| c.subject_raw == cur.issuer_raw && c.raw_der != cur.raw_der));
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
    let mut signers = Vec::new();
    for &elem in &sd_kids[3..] {
        if elem.tag == 0xA0 {
            if let Some(cert_elems) = der_children(elem.val) {
                for ce in cert_elems {
                    if parse_x509_cert(ce).is_some() {
                        certs_der.push(ce.raw.to_vec());
                    }
                }
            }
        } else if elem.tag == 0x31 {
            if let Some(si_list) = der_children(elem.val) {
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

pub unsafe extern "win64" fn WinVerifyTrust(_hwnd: *mut c_void, action: *const u8, data: *mut c_void) -> i32 {
    if action.is_null() || data.is_null() {
        return TRUST_E_PROVIDER_UNKNOWN;
    }
    let mut guid = [0u8; 16];
    std::ptr::copy_nonoverlapping(action, guid.as_mut_ptr(), 16);
    if guid != WINTRUST_ACTION_GENERIC_VERIFY_V2 {
        let a = super::guid_string(action);
        stub!("WinVerifyTrust(action={a}) -> TRUST_E_ACTION_UNKNOWN");
        return TRUST_E_ACTION_UNKNOWN;
    }

    let base = data as *const u8;
    let cb_struct = std::ptr::read_unaligned(base as *const u32);
    if cb_struct < 0x40 {
        return TRUST_E_PROVIDER_UNKNOWN;
    }
    let union_choice = std::ptr::read_unaligned(base.add(0x20) as *const u32);
    let p_file = std::ptr::read_unaligned(base.add(0x28) as *const *const u8);
    let state_action = std::ptr::read_unaligned(base.add(0x30) as *const u32);
    let state_data_slot = (data as *mut u8).add(0x38) as *mut *mut c_void;
    let cur_state = std::ptr::read_unaligned(state_data_slot);

    if state_action == WTD_STATEACTION_CLOSE {
        if !cur_state.is_null() {
            STATE_MAP.lock().unwrap().remove(&(cur_state as usize));
            std::ptr::write_unaligned(state_data_slot, std::ptr::null_mut());
        }
        return ERROR_SUCCESS;
    }

    if union_choice != WTD_CHOICE_FILE || p_file.is_null() {
        stub!("WinVerifyTrust(choice={union_choice}) -> TRUST_E_NOSIGNATURE");
        return TRUST_E_NOSIGNATURE;
    }

    let pcwsz_path = std::ptr::read_unaligned(p_file.add(0x08) as *const *const u16);
    let h_file = std::ptr::read_unaligned(p_file.add(0x10) as *const crate::types::HANDLE);

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
            log::warn!("WinVerifyTrust({label:?}): read {}: {e} -> CRYPT_E_FILE_ERROR", host_path.display());
            return CRYPT_E_FILE_ERROR;
        }
    };

    match verify_pe_authenticode(&bytes) {
        Ok(verified) => {
            let ncerts = verified.certs.len();
            if state_action == WTD_STATEACTION_VERIFY {
                let h = alloc_provider_state(verified, data);
                std::ptr::write_unaligned(state_data_slot, h);
            }
            log::info!("WinVerifyTrust({label:?}): verified PE Authenticode digest + PKCS#7 RSA signature ({ncerts} cert(s))");
            ERROR_SUCCESS
        }
        Err(code) => {
            log::warn!("WinVerifyTrust({label:?}): Authenticode verification failed ({code:#010x})");
            code
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
    if h_context.is_null() {
        TBS_E_INVALID_CONTEXT
    } else {
        stub!("Tbsip_Context_Close({h_context:p}) -> TBS_E_INVALID_CONTEXT");
        TBS_E_INVALID_CONTEXT
    }
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
    stub!("Tbsip_Submit_Command -> TBS_E_TPM_NOT_FOUND");
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
}

