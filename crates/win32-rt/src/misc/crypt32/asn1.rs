//! ASN.1 DER parsing, X.509 encoding (`CryptEncodeObjectEx`), and string-to-binary
//! decoding (`CryptStringToBinaryA/W`) for `crypt32.dll`.

use super::decode::{
    CNG_RSA_PUBLIC_KEY_BLOB, PKCS_ATTRIBUTE, PKCS_ATTRIBUTES, PKCS_CONTENT_INFO, PKCS_UTC_TIME,
    X509_ALGORITHM_IDENTIFIER, X509_AUTHORITY_KEY_ID2, X509_ENUMERATED,
};
use crate::misc::wintrust::{
    self, CertInfo, CertPublicKeyInfo, CryptAlgId, CryptBitBlob, CryptDataBlob,
};
use crate::types::*;
use std::ffi::c_void;

pub(super) const X509_ASN_ENCODING: u32 = 0x0000_0001;
pub(super) const PKCS_7_ASN_ENCODING: u32 = 0x0001_0000;

pub(super) const X509_CERT: usize = 1;
pub(super) const X509_CERT_TO_BE_SIGNED: usize = 2;
pub(super) const X509_EXTENSIONS: usize = 5;
pub(super) const X509_ANY_STRING: usize = 6;
pub(super) const X509_NAME: usize = 7;
pub(super) const X509_PUBLIC_KEY_INFO: usize = 8;
pub(super) const X509_ALTERNATE_NAME: usize = 12;
pub(super) const X509_KEY_USAGE: usize = 14;
pub(super) const X509_BASIC_CONSTRAINTS2: usize = 15;
pub(super) const RSA_CSP_PUBLICKEYBLOB: usize = 19;
pub(super) const X509_UNICODE_NAME: usize = 20;
pub(super) const X509_UNICODE_ANY_STRING: usize = 24;
pub(super) const X509_OCTET_STRING: usize = 25;
pub(super) const X509_BITS: usize = 26;
pub(super) const X509_INTEGER: usize = 27;
pub(super) const X509_MULTI_BYTE_INTEGER: usize = 28;
pub(super) const X509_CHOICE_OF_TIME: usize = 30;
pub(super) const X509_SEQUENCE_OF_ANY: usize = 34;
pub(super) const X509_ENHANCED_KEY_USAGE: usize = 36;
pub(super) const X509_MULTI_BYTE_UINT: usize = 38;
pub(super) const X509_ECC_SIGNATURE: usize = 47;
pub(super) const X509_OBJECT_IDENTIFIER: usize = 73;
const CRYPT_ENCODE_ALLOC_FLAG: u32 = 0x0000_8000;

pub(super) const CRYPT_STRING_BASE64HEADER: u32 = 0x0000_0000;
pub(super) const CRYPT_STRING_BASE64: u32 = 0x0000_0001;
pub(super) const CRYPT_STRING_BINARY: u32 = 0x0000_0002;
pub(super) const CRYPT_STRING_BASE64REQUESTHEADER: u32 = 0x0000_0003;
pub(super) const CRYPT_STRING_HEX: u32 = 0x0000_0004;
pub(super) const CRYPT_STRING_HEXASCII: u32 = 0x0000_0005;
pub(super) const CRYPT_STRING_BASE64_ANY: u32 = 0x0000_0006;
pub(super) const CRYPT_STRING_ANY: u32 = 0x0000_0007;
pub(super) const CRYPT_STRING_HEX_ANY: u32 = 0x0000_0008;
pub(super) const CRYPT_STRING_BASE64X509CRLHEADER: u32 = 0x0000_0009;
pub(super) const CRYPT_STRING_HEXADDR: u32 = 0x0000_000A;
pub(super) const CRYPT_STRING_HEXASCIIADDR: u32 = 0x0000_000B;
pub(super) const CRYPT_STRING_HEXRAW: u32 = 0x0000_000C;
pub(super) const CRYPT_STRING_BASE64URI: u32 = 0x0000_000D;
pub(super) const CRYPT_STRING_PERCENTESCAPE: u32 = 0x0000_000E;
const CRYPT_STRING_STRICT: u32 = 0x2000_0000;

// ---------------------------------------------------------------------------
// Minimal DER parser & X.509 extension / DN helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub(super) struct DerItem<'a> {
    pub(super) tag: u8,
    pub(super) raw: &'a [u8],
    pub(super) val: &'a [u8],
}

pub(super) fn parse_der_item(buf: &[u8]) -> Option<(DerItem<'_>, &[u8])> {
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
    Some((
        DerItem {
            tag,
            raw: &buf[..end],
            val: &buf[hdr..end],
        },
        &buf[end..],
    ))
}

pub(super) fn der_item_children(mut buf: &[u8]) -> Option<Vec<DerItem<'_>>> {
    let mut out = Vec::new();
    while !buf.is_empty() {
        let (it, rest) = parse_der_item(buf)?;
        out.push(it);
        buf = rest;
    }
    Some(out)
}

pub(super) struct CertDerParts<'a> {
    pub(super) tbs_raw: &'a [u8],
    pub(super) sig_alg_inner: &'a [u8],
    pub(super) sig_bits: &'a [u8],
    pub(super) issuer_inner: &'a [u8],
    pub(super) subject_inner: &'a [u8],
    pub(super) subject_raw: &'a [u8],
    pub(super) spki_inner: &'a [u8],
    pub(super) spki_alg_inner: &'a [u8],
    pub(super) spki_key_bits: &'a [u8],
    pub(super) extensions: Vec<(&'a [u8], bool, &'a [u8])>,
}

pub(super) fn parse_cert_der_parts(der: &[u8]) -> Option<CertDerParts<'_>> {
    let (cert_seq, _) = parse_der_item(der)?;
    if cert_seq.tag != 0x30 {
        return None;
    }
    let top = der_item_children(cert_seq.val)?;
    if top.len() < 3
        || top[0].tag != 0x30
        || top[1].tag != 0x30
        || top[2].tag != 0x03
        || top[2].val.is_empty()
    {
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
    if spki_kids.len() < 2
        || spki_kids[0].tag != 0x30
        || spki_kids[1].tag != 0x03
        || spki_kids[1].val.is_empty()
    {
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
                            let Some(ek) = der_item_children(ext.val) else {
                                continue;
                            };
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

/// Extract EKU OIDs from extension `2.5.29.37`, or `None` if the cert has no EKU extension.
pub(super) fn cert_eku_oids(der: &[u8]) -> Option<Vec<Vec<u8>>> {
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
        let oids = kids
            .into_iter()
            .filter(|k| k.tag == 0x06)
            .map(|k| k.val.to_vec())
            .collect();
        return Some(oids);
    }
    None
}

/// Extract all RDN attribute strings from a DER `Name` (handles every OID, not just CN/O/OU/C/L/S/E).
pub(super) fn all_dn_values_lower(dn_der: &[u8]) -> Vec<String> {
    let mut vals = Vec::new();
    let Some((seq, _)) = parse_der_item(dn_der) else {
        return vals;
    };
    if seq.tag != 0x30 {
        return vals;
    }
    let Some(rdns) = der_item_children(seq.val) else {
        return vals;
    };
    for rdn in rdns {
        if rdn.tag != 0x31 {
            continue;
        }
        let Some(atvs) = der_item_children(rdn.val) else {
            continue;
        };
        for atv in atvs {
            if atv.tag != 0x30 {
                continue;
            }
            let Some(kids) = der_item_children(atv.val) else {
                continue;
            };
            if kids.len() >= 2 {
                let s = match kids[1].tag {
                    0x0C | 0x13 | 0x16 | 0x14 | 0x12 => {
                        Some(String::from_utf8_lossy(kids[1].val).into_owned())
                    }
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

// ---------------------------------------------------------------------------
// ASN.1 DER Encoding (`CryptEncodeObjectEx`)
// ---------------------------------------------------------------------------

pub(super) fn encode_der_tlv(tag: u8, val: &[u8]) -> Vec<u8> {
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
    if let Some(raw) = super::store::lookup_spki_raw_der(pv_struct_info as usize) {
        return Some(raw);
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
        let s = format!(
            "{:02}{:02}{:02}{:02}{:02}{:02}Z",
            year % 100,
            month,
            day,
            hour,
            min,
            sec
        );
        Some(encode_der_tlv(0x17, s.as_bytes()))
    } else if (0..=9999).contains(&year) {
        let s = format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}Z",
            year, month, day, hour, min, sec
        );
        Some(encode_der_tlv(0x18, s.as_bytes()))
    } else {
        None
    }
}

#[repr(C)]
pub(super) struct CertBasicConstraints2Info {
    pub(super) f_ca: i32,
    pub(super) f_path_len_constraint: i32,
    pub(super) dw_path_len_constraint: u32,
}

#[repr(C)]
pub(super) struct CertEnhkeyUsage {
    pub(super) c_usage_identifier: u32,
    pub(super) rgpsz_usage_identifier: *const *const u8,
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
pub(super) struct CertRdnAttr {
    pub(super) psz_obj_id: *const u8,
    pub(super) dw_value_type: u32,
    pub(super) value: CryptDataBlob,
}

#[repr(C)]
pub(super) struct CertRdn {
    pub(super) c_rdn_attr: u32,
    pub(super) rg_rdn_attr: *const CertRdnAttr,
}

#[repr(C)]
pub(super) struct CertNameInfo {
    pub(super) c_rdn: u32,
    pub(super) rg_rdn: *const CertRdn,
}

#[repr(C)]
struct CertNameValue {
    dw_value_type: u32,
    value: CryptDataBlob,
}

#[repr(C)]
pub(super) struct CertAltNameEntry {
    pub(super) dw_alt_name_choice: u32,
    pub(super) payload: [u8; 16],
}

#[repr(C)]
pub(super) struct CertAltNameInfo {
    pub(super) c_alt_entry: u32,
    pub(super) rg_alt_entry: *const CertAltNameEntry,
}

#[repr(C)]
struct CryptSequenceOfAny {
    c_value: u32,
    rg_value: *const CryptDataBlob,
}

#[repr(C)]
pub(super) struct CertEccSignature {
    pub(super) r: CryptDataBlob,
    pub(super) s: CryptDataBlob,
}

#[repr(C)]
struct CertSignedContentInfo {
    to_be_signed: CryptDataBlob,
    signature_algorithm: CryptAlgId,
    signature: CryptBitBlob,
}

unsafe fn encode_alg_id(alg: &CryptAlgId) -> Option<Vec<u8>> {
    if alg.psz_obj_id.is_null() {
        return None;
    }
    let oid_s = crate::strings::from_ansi(alg.psz_obj_id);
    let oid_val = wintrust::encode_oid_str(&oid_s)?;
    let mut inner = encode_der_tlv(0x06, &oid_val);
    if !alg.parameters.pb_data.is_null() && alg.parameters.cb_data > 0 {
        let params =
            std::slice::from_raw_parts(alg.parameters.pb_data, alg.parameters.cb_data as usize);
        inner.extend_from_slice(params);
    }
    Some(encode_der_tlv(0x30, &inner))
}

unsafe fn encode_bit_blob(bb: &CryptBitBlob) -> Vec<u8> {
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

unsafe fn encode_rdn_value(
    dw_value_type: u32,
    blob: &CryptDataBlob,
    unicode: bool,
) -> Option<Vec<u8>> {
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
            let units = crate::strings::read_wide_slice(
                blob.pb_data as *const u16,
                (blob.cb_data as usize) / 2,
            );
            String::from_utf16_lossy(&units)
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

unsafe fn encode_x509_object(
    lpsz_struct_type: *const u8,
    pv_struct_info: *const c_void,
) -> Result<Vec<u8>, u32> {
    let raw_stype = lpsz_struct_type as usize;
    let stype: usize = if raw_stype <= 0xFFFF {
        raw_stype
    } else {
        match crate::strings::from_ansi(lpsz_struct_type).as_str() {
            "2.5.29.14" => X509_OCTET_STRING,
            "2.5.29.15" => X509_KEY_USAGE,
            "2.5.29.17" | "2.5.29.18" => X509_ALTERNATE_NAME,
            "2.5.29.19" => X509_BASIC_CONSTRAINTS2,
            "2.5.29.35" => X509_AUTHORITY_KEY_ID2,
            "2.5.29.37" => X509_ENHANCED_KEY_USAGE,
            _ => 0,
        }
    };

    match stype {
        X509_PUBLIC_KEY_INFO => {
            encode_spki_from_struct(pv_struct_info).ok_or(err::ERROR_INVALID_DATA)
        }
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
                        let blob =
                            std::ptr::read_unaligned(e.payload.as_ptr() as *const CryptDataBlob);
                        if blob.pb_data.is_null() || blob.cb_data == 0 {
                            return Err(err::ERROR_INVALID_DATA);
                        }
                        let b = std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize);
                        inner.extend_from_slice(&encode_der_tlv(0xA4, b));
                    }
                    8 => {
                        // IP_ADDRESS -> [7] IMPLICIT OCTET STRING (0x87)
                        let blob =
                            std::ptr::read_unaligned(e.payload.as_ptr() as *const CryptDataBlob);
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
                        let oid_val =
                            wintrust::encode_oid_str(&oid_s).ok_or(err::ERROR_INVALID_DATA)?;
                        inner.extend_from_slice(&encode_der_tlv(0x88, &oid_val));
                    }
                    _ => return Err(err::ERROR_INVALID_DATA),
                }
            }
            Ok(encode_der_tlv(0x30, &inner))
        }
        X509_NAME => encode_cert_name_info(pv_struct_info, false).ok_or(err::ERROR_INVALID_DATA),
        X509_UNICODE_NAME => {
            encode_cert_name_info(pv_struct_info, true).ok_or(err::ERROR_INVALID_DATA)
        }
        X509_ANY_STRING | X509_UNICODE_ANY_STRING => {
            let nv = std::ptr::read_unaligned(pv_struct_info as *const CertNameValue);
            encode_rdn_value(nv.dw_value_type, &nv.value, stype == X509_UNICODE_ANY_STRING)
                .ok_or(err::ERROR_INVALID_DATA)
        }
        X509_EXTENSIONS => {
            let exts = std::ptr::read_unaligned(pv_struct_info as *const CertExtensions);
            encode_extensions_list(exts.c_extension, exts.rg_extension)
                .ok_or(err::ERROR_INVALID_DATA)
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
            let bb = std::ptr::read_unaligned(pv_struct_info as *const CryptBitBlob);
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
                    inner.extend_from_slice(std::slice::from_raw_parts(
                        b.pb_data,
                        b.cb_data as usize,
                    ));
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
            if sig.r.pb_data.is_null()
                || sig.r.cb_data == 0
                || sig.s.pb_data.is_null()
                || sig.s.cb_data == 0
            {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut r_be =
                std::slice::from_raw_parts(sig.r.pb_data, sig.r.cb_data as usize).to_vec();
            let mut s_be =
                std::slice::from_raw_parts(sig.s.pb_data, sig.s.cb_data as usize).to_vec();
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
            let tbs = std::slice::from_raw_parts(
                sci.to_be_signed.pb_data,
                sci.to_be_signed.cb_data as usize,
            );
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
            let mut ser_be =
                if !ci.serial_number.pb_data.is_null() && ci.serial_number.cb_data > 0 {
                    std::slice::from_raw_parts(
                        ci.serial_number.pb_data,
                        ci.serial_number.cb_data as usize,
                    )
                    .to_vec()
                } else {
                    vec![1]
                };
            ser_be.reverse();
            tbs.extend_from_slice(&encode_der_uint_be(&ser_be));
            tbs.extend_from_slice(
                &encode_alg_id(&ci.signature_algorithm).ok_or(err::ERROR_INVALID_DATA)?,
            );
            if ci.issuer.pb_data.is_null() || ci.subject.pb_data.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            tbs.extend_from_slice(std::slice::from_raw_parts(
                ci.issuer.pb_data,
                ci.issuer.cb_data as usize,
            ));
            let nb = encode_der_filetime(ci.not_before).ok_or(err::ERROR_INVALID_DATA)?;
            let na = encode_der_filetime(ci.not_after).ok_or(err::ERROR_INVALID_DATA)?;
            tbs.extend_from_slice(&encode_der_tlv(0x30, &[nb, na].concat()));
            tbs.extend_from_slice(std::slice::from_raw_parts(
                ci.subject.pb_data,
                ci.subject.cb_data as usize,
            ));
            let spki_der =
                encode_spki_from_struct(&ci.subject_public_key_info as *const _ as *const c_void)
                    .ok_or(err::ERROR_INVALID_DATA)?;
            tbs.extend_from_slice(&spki_der);
            if ci.c_extension > 0 && !ci.rg_extension.is_null() {
                let exts =
                    encode_extensions_list(ci.c_extension, ci.rg_extension as *const CertExtension)
                        .ok_or(err::ERROR_INVALID_DATA)?;
                tbs.extend_from_slice(&encode_der_tlv(0xA3, &exts));
            }
            Ok(encode_der_tlv(0x30, &tbs))
        }
        X509_ENUMERATED => {
            let v = std::ptr::read_unaligned(pv_struct_info as *const i32);
            let mut der = encode_der_i32(v);
            der[0] = 0x0A; // ASN.1 ENUMERATED tag
            Ok(der)
        }
        PKCS_UTC_TIME => {
            let ft = std::ptr::read_unaligned(pv_struct_info as *const u64);
            encode_der_filetime(ft).ok_or(err::ERROR_INVALID_DATA)
        }
        X509_ALGORITHM_IDENTIFIER => {
            let alg = std::ptr::read_unaligned(pv_struct_info as *const CryptAlgId);
            encode_alg_id(&alg).ok_or(err::ERROR_INVALID_DATA)
        }
        PKCS_CONTENT_INFO => {
            #[repr(C)]
            struct CryptContentInfo {
                psz_obj_id: *const u8,
                content: CryptDataBlob,
            }
            let ci = std::ptr::read_unaligned(pv_struct_info as *const CryptContentInfo);
            if ci.psz_obj_id.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            let oid_s = crate::strings::from_ansi(ci.psz_obj_id);
            let oid_val = wintrust::encode_oid_str(&oid_s).ok_or(err::ERROR_INVALID_DATA)?;
            let mut inner = encode_der_tlv(0x06, &oid_val);
            if !ci.content.pb_data.is_null() && ci.content.cb_data > 0 {
                let c = std::slice::from_raw_parts(ci.content.pb_data, ci.content.cb_data as usize);
                inner.extend_from_slice(&encode_der_tlv(0xA0, c));
            }
            Ok(encode_der_tlv(0x30, &inner))
        }
        PKCS_ATTRIBUTE => {
            #[repr(C)]
            struct CryptAttr {
                psz_obj_id: *const u8,
                c_value: u32,
                rg_value: *const CryptDataBlob,
            }
            let a = std::ptr::read_unaligned(pv_struct_info as *const CryptAttr);
            if a.psz_obj_id.is_null() || (a.c_value > 0 && a.rg_value.is_null()) {
                return Err(err::ERROR_INVALID_DATA);
            }
            let oid_s = crate::strings::from_ansi(a.psz_obj_id);
            let oid_val = wintrust::encode_oid_str(&oid_s).ok_or(err::ERROR_INVALID_DATA)?;
            let mut vals: Vec<&[u8]> = Vec::with_capacity(a.c_value as usize);
            for i in 0..a.c_value as usize {
                let b = std::ptr::read_unaligned(a.rg_value.add(i));
                if !b.pb_data.is_null() && b.cb_data > 0 {
                    vals.push(std::slice::from_raw_parts(b.pb_data, b.cb_data as usize));
                }
            }
            // X.690 §11.6 / RFC 5652 §5.4: DER SET OF elements must be sorted lexicographically.
            vals.sort_unstable();
            let mut set_inner = Vec::new();
            for v in vals {
                set_inner.extend_from_slice(v);
            }
            let mut inner = encode_der_tlv(0x06, &oid_val);
            inner.extend_from_slice(&encode_der_tlv(0x31, &set_inner));
            Ok(encode_der_tlv(0x30, &inner))
        }
        PKCS_ATTRIBUTES => {
            #[repr(C)]
            struct CryptAttr {
                psz_obj_id: *const u8,
                c_value: u32,
                rg_value: *const CryptDataBlob,
            }
            #[repr(C)]
            struct CryptAttrs {
                c_attr: u32,
                rg_attr: *const CryptAttr,
            }
            let list = std::ptr::read_unaligned(pv_struct_info as *const CryptAttrs);
            if list.c_attr > 0 && list.rg_attr.is_null() {
                return Err(err::ERROR_INVALID_DATA);
            }
            let mut encoded_attrs: Vec<Vec<u8>> = Vec::with_capacity(list.c_attr as usize);
            for i in 0..list.c_attr as usize {
                let attr_ptr = list.rg_attr.add(i) as *const c_void;
                encoded_attrs.push(encode_x509_object(PKCS_ATTRIBUTE as *const u8, attr_ptr)?);
            }
            // X.690 §11.6 / RFC 5652 §5.4: DER SET OF Attribute must be sorted lexicographically.
            encoded_attrs.sort_unstable();
            let mut inner = Vec::new();
            for enc in encoded_attrs {
                inner.extend_from_slice(&enc);
            }
            Ok(encode_der_tlv(0x31, &inner))
        }
        X509_AUTHORITY_KEY_ID2 => {
            #[repr(C)]
            struct CertAuthorityKeyId2Info {
                key_id: CryptDataBlob,
                authority_cert_issuer: CertAltNameInfo,
                authority_cert_serial_number: CryptDataBlob,
            }
            let aki = std::ptr::read_unaligned(pv_struct_info as *const CertAuthorityKeyId2Info);
            let mut inner = Vec::new();
            if !aki.key_id.pb_data.is_null() && aki.key_id.cb_data > 0 {
                let b = std::slice::from_raw_parts(aki.key_id.pb_data, aki.key_id.cb_data as usize);
                inner.extend_from_slice(&encode_der_tlv(0x80, b));
            }
            if aki.authority_cert_issuer.c_alt_entry > 0
                && !aki.authority_cert_issuer.rg_alt_entry.is_null()
            {
                let san_der = encode_x509_object(
                    X509_ALTERNATE_NAME as *const u8,
                    &aki.authority_cert_issuer as *const _ as *const c_void,
                )?;
                if let Some((seq, _)) = parse_der_item(&san_der) {
                    inner.extend_from_slice(&encode_der_tlv(0xA1, seq.val));
                }
            }
            if !aki.authority_cert_serial_number.pb_data.is_null()
                && aki.authority_cert_serial_number.cb_data > 0
            {
                let mut be = std::slice::from_raw_parts(
                    aki.authority_cert_serial_number.pb_data,
                    aki.authority_cert_serial_number.cb_data as usize,
                )
                .to_vec();
                be.reverse();
                inner.extend_from_slice(&encode_der_tlv(0x82, &be));
            }
            Ok(encode_der_tlv(0x30, &inner))
        }
        CNG_RSA_PUBLIC_KEY_BLOB => {
            // BCRYPT_RSAKEY_BLOB (24B) + BE exponent + BE modulus
            let p = pv_struct_info as *const u8;
            let magic = std::ptr::read_unaligned(p as *const u32);
            let cb_exp = std::ptr::read_unaligned(p.add(8) as *const u32) as usize;
            let cb_mod = std::ptr::read_unaligned(p.add(12) as *const u32) as usize;
            if (magic != 0x3141_5352 && magic != 0x3241_5352) || cb_exp == 0 || cb_mod == 0 {
                return Err(err::ERROR_INVALID_DATA);
            }
            let exp_be = std::slice::from_raw_parts(p.add(24), cb_exp);
            let mod_be = std::slice::from_raw_parts(p.add(24 + cb_exp), cb_mod);
            let mut inner = encode_der_uint_be(mod_be);
            inner.extend_from_slice(&encode_der_uint_be(exp_be));
            Ok(encode_der_tlv(0x30, &inner))
        }
        _ => {
            log::debug!("CryptEncodeObjectEx: unknown struct_type={raw_stype:#x} -> ERROR_FILE_NOT_FOUND");
            Err(err::ERROR_FILE_NOT_FOUND)
        }
    }
}

pub(super) unsafe fn crypt_encode_object_ex_impl(
    dw_cert_encoding_type: u32,
    lpsz_struct_type: *const u8,
    pv_struct_info: *const c_void,
    dw_flags: u32,
    pv_encoded: *mut c_void,
    pcb_encoded: *mut u32,
) -> BOOL {
    if (dw_cert_encoding_type & (X509_ASN_ENCODING | PKCS_7_ASN_ENCODING)) == 0
        || pv_struct_info.is_null()
        || pcb_encoded.is_null()
    {
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
// String -> Binary Decoding (`CryptStringToBinaryA/W`)
// ---------------------------------------------------------------------------

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
    Some(
        nibbles
            .chunks_exact(2)
            .map(|c| (c[0] << 4) | c[1])
            .collect(),
    )
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
        CRYPT_STRING_BASE64_ANY => try_pem(None).or_else(|| {
            decode_b64_bytes_ext(text, false, strict).map(|b| (b, 0, CRYPT_STRING_BASE64))
        }),
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
            .or_else(|| {
                decode_b64_bytes_ext(text, false, strict).map(|b| (b, 0, CRYPT_STRING_BASE64))
            })
            .or_else(|| {
                decode_hex_dump(text, true, true).map(|b| (b, 0, CRYPT_STRING_HEXASCIIADDR))
            })
            .or_else(|| decode_hex_dump(text, true, false).map(|b| (b, 0, CRYPT_STRING_HEXADDR)))
            .or_else(|| decode_hex_bytes(text, false, false).map(|b| (b, 0, CRYPT_STRING_HEX)))
            .or_else(|| decode_hex_dump(text, false, true).map(|b| (b, 0, CRYPT_STRING_HEXASCII))),
        CRYPT_STRING_PERCENTESCAPE => {
            // Percent-decode UTF-8/ASCII bytes per Windows CRYPT_STRING_PERCENTESCAPE (0x0E)
            let bytes = text.as_bytes();
            let mut out = Vec::with_capacity(bytes.len());
            let mut i = 0usize;
            let mut ok = true;
            while i < bytes.len() {
                if bytes[i] == b'%' {
                    if i + 2 >= bytes.len() {
                        ok = false;
                        break;
                    }
                    let Some(hi) = hex_nibble(bytes[i + 1]) else {
                        ok = false;
                        break;
                    };
                    let Some(lo) = hex_nibble(bytes[i + 2]) else {
                        ok = false;
                        break;
                    };
                    out.push((hi << 4) | lo);
                    i += 3;
                } else {
                    out.push(bytes[i]);
                    i += 1;
                }
            }
            ok.then_some((out, 0, CRYPT_STRING_PERCENTESCAPE))
        }
        _ => {
            // Invalid dwFlags value outside wincrypt.h CRYPT_STRING_* constants -> ERROR_INVALID_DATA.
            log::debug!("CryptStringToBinary: invalid flags {flags:#x} -> ERROR_INVALID_DATA");
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

pub(super) unsafe fn crypt_string_to_binary_a_impl(
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
    crypt_string_to_binary_impl(
        &s,
        &raw,
        dw_flags,
        pb_binary,
        pcb_binary,
        pdw_skip,
        pdw_flags,
    )
}

pub(super) unsafe fn crypt_string_to_binary_w_impl(
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
        let units = crate::strings::read_wide_slice(psz_string, cch_string as usize);
        String::from_utf16_lossy(&units)
    };
    let raw = s.as_bytes().to_vec();
    crypt_string_to_binary_impl(
        &s,
        &raw,
        dw_flags,
        pb_binary,
        pcb_binary,
        pdw_skip,
        pdw_flags,
    )
}

// ---------------------------------------------------------------------------
// CryptBinaryToStringA / W
// ---------------------------------------------------------------------------

const CRYPT_STRING_NOCRLF: u32 = 0x4000_0000;
const CRYPT_STRING_NOCR: u32 = 0x8000_0000;

fn encode_base64_str(data: &[u8], url_safe: bool) -> String {
    const STD: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    const URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let tab = if url_safe { URL } else { STD };
    let mut out = String::with_capacity(((data.len() + 2) / 3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(tab[((triple >> 18) & 0x3F) as usize] as char);
        out.push(tab[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(tab[((triple >> 6) & 0x3F) as usize] as char);
        } else if !url_safe {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(tab[(triple & 0x3F) as usize] as char);
        } else if !url_safe {
            out.push('=');
        }
    }
    out
}

fn format_binary_to_string(data: &[u8], flags: u32) -> Option<String> {
    let eol = if (flags & CRYPT_STRING_NOCRLF) != 0 {
        ""
    } else if (flags & CRYPT_STRING_NOCR) != 0 {
        "\n"
    } else {
        "\r\n"
    };
    let mode = flags & 0x0FFF_FFFF;
    let wrap_64 = |raw: &str, nl: &str| -> String {
        if nl.is_empty() {
            return raw.to_string();
        }
        let mut s = String::with_capacity(raw.len() + (raw.len() / 64 + 1) * nl.len());
        for chunk in raw.as_bytes().chunks(64) {
            s.push_str(std::str::from_utf8(chunk).unwrap_or(""));
            s.push_str(nl);
        }
        s
    };
    match mode {
        CRYPT_STRING_BASE64 => Some(wrap_64(&encode_base64_str(data, false), eol)),
        CRYPT_STRING_BASE64URI => {
            let b = encode_base64_str(data, true);
            Some(if eol.is_empty() { b } else { format!("{b}{eol}") })
        }
        CRYPT_STRING_BASE64HEADER => {
            let body = wrap_64(&encode_base64_str(data, false), eol);
            Some(format!(
                "-----BEGIN CERTIFICATE-----{eol}{body}-----END CERTIFICATE-----{eol}"
            ))
        }
        CRYPT_STRING_BASE64REQUESTHEADER => {
            let body = wrap_64(&encode_base64_str(data, false), eol);
            Some(format!(
                "-----BEGIN NEW CERTIFICATE REQUEST-----{eol}{body}-----END NEW CERTIFICATE REQUEST-----{eol}"
            ))
        }
        CRYPT_STRING_BASE64X509CRLHEADER => {
            let body = wrap_64(&encode_base64_str(data, false), eol);
            Some(format!(
                "-----BEGIN X509 CRL-----{eol}{body}-----END X509 CRL-----{eol}"
            ))
        }
        CRYPT_STRING_HEXRAW => {
            let mut s = String::with_capacity(data.len() * 2 + eol.len());
            for &b in data {
                use std::fmt::Write as _;
                let _ = write!(s, "{b:02x}");
            }
            s.push_str(eol);
            Some(s)
        }
        CRYPT_STRING_HEX
        | CRYPT_STRING_HEXASCII
        | CRYPT_STRING_HEXADDR
        | CRYPT_STRING_HEXASCIIADDR => {
            let with_addr = matches!(mode, CRYPT_STRING_HEXADDR | CRYPT_STRING_HEXASCIIADDR);
            let with_ascii = matches!(mode, CRYPT_STRING_HEXASCII | CRYPT_STRING_HEXASCIIADDR);
            let mut s = String::new();
            for (row, chunk) in data.chunks(16).enumerate() {
                use std::fmt::Write as _;
                if with_addr {
                    let _ = write!(s, "{:04x}  ", row * 16);
                }
                for (i, &b) in chunk.iter().enumerate() {
                    if i > 0 {
                        s.push(' ');
                    }
                    let _ = write!(s, "{b:02x}");
                }
                if with_ascii {
                    for _ in chunk.len()..16 {
                        s.push_str("   ");
                    }
                    s.push_str("  ");
                    for &b in chunk {
                        let ch = if (0x20..=0x7E).contains(&b) { b as char } else { '.' };
                        s.push(ch);
                    }
                }
                s.push_str(eol);
            }
            Some(s)
        }
        _ => None,
    }
}

pub(super) unsafe fn crypt_binary_to_string_a_impl(
    pb_binary: *const u8,
    cb_binary: u32,
    dw_flags: u32,
    psz_string: LPSTR,
    pcch_string: *mut u32,
) -> BOOL {
    if pcch_string.is_null() || (cb_binary > 0 && pb_binary.is_null()) {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let slice = if cb_binary == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(pb_binary, cb_binary as usize)
    };
    let Some(encoded) = format_binary_to_string(slice, dw_flags) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let bytes = encoded.as_bytes();
    let needed_with_nul = (bytes.len() + 1) as u32;
    if psz_string.is_null() {
        std::ptr::write_unaligned(pcch_string, needed_with_nul);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pcch_string);
    if cap < needed_with_nul {
        std::ptr::write_unaligned(pcch_string, needed_with_nul);
        crate::teb::set_last_error(err::ERROR_MORE_DATA);
        return FALSE;
    }
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), psz_string, bytes.len());
    }
    std::ptr::write_unaligned(psz_string.add(bytes.len()), 0);
    std::ptr::write_unaligned(pcch_string, bytes.len() as u32);
    TRUE
}

pub(super) unsafe fn crypt_binary_to_string_w_impl(
    pb_binary: *const u8,
    cb_binary: u32,
    dw_flags: u32,
    psz_string: LPWSTR,
    pcch_string: *mut u32,
) -> BOOL {
    if pcch_string.is_null() || (cb_binary > 0 && pb_binary.is_null()) {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let slice = if cb_binary == 0 {
        &[][..]
    } else {
        std::slice::from_raw_parts(pb_binary, cb_binary as usize)
    };
    let Some(encoded) = format_binary_to_string(slice, dw_flags) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    let units: Vec<u16> = encoded.encode_utf16().collect();
    let needed_with_nul = (units.len() + 1) as u32;
    if psz_string.is_null() {
        std::ptr::write_unaligned(pcch_string, needed_with_nul);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pcch_string);
    if cap < needed_with_nul {
        std::ptr::write_unaligned(pcch_string, needed_with_nul);
        crate::teb::set_last_error(err::ERROR_MORE_DATA);
        return FALSE;
    }
    for (i, &u) in units.iter().enumerate() {
        std::ptr::write_unaligned(psz_string.add(i), u);
    }
    std::ptr::write_unaligned(psz_string.add(units.len()), 0);
    std::ptr::write_unaligned(pcch_string, units.len() as u32);
    TRUE
}

// ---------------------------------------------------------------------------
// CertCompareIntegerBlob / CertCompareCertificateName / CertFindExtension
// ---------------------------------------------------------------------------

unsafe fn read_blob_slice<'a>(blob_ptr: *const c_void) -> Option<&'a [u8]> {
    if blob_ptr.is_null() {
        return None;
    }
    let blob = std::ptr::read_unaligned(blob_ptr as *const CryptDataBlob);
    if blob.cb_data == 0 {
        Some(&[])
    } else if blob.pb_data.is_null() {
        None
    } else {
        Some(std::slice::from_raw_parts(blob.pb_data, blob.cb_data as usize))
    }
}

fn trim_le_signed_int(mut b: &[u8]) -> &[u8] {
    while b.len() > 1 {
        let last = b[b.len() - 1];
        let prev = b[b.len() - 2];
        if (last == 0x00 && (prev & 0x80) == 0) || (last == 0xFF && (prev & 0x80) != 0) {
            b = &b[..b.len() - 1];
        } else {
            break;
        }
    }
    b
}

pub(super) unsafe fn cert_compare_integer_blob_impl(
    p_int1: *const c_void,
    p_int2: *const c_void,
) -> BOOL {
    let (Some(a), Some(b)) = (read_blob_slice(p_int1), read_blob_slice(p_int2)) else {
        return FALSE;
    };
    (trim_le_signed_int(a) == trim_le_signed_int(b)) as BOOL
}

pub(super) unsafe fn cert_compare_certificate_name_impl(
    _dw_cert_encoding_type: u32,
    p_cert_name1: *const c_void,
    p_cert_name2: *const c_void,
) -> BOOL {
    let (Some(a), Some(b)) = (read_blob_slice(p_cert_name1), read_blob_slice(p_cert_name2)) else {
        return FALSE;
    };
    (a == b) as BOOL
}

#[repr(C)]
struct WinCertExtension {
    psz_obj_id: *const u8,
    f_critical: BOOL,
    _pad: u32,
    value: CryptDataBlob,
}
const _: () = assert!(std::mem::size_of::<WinCertExtension>() == 32);

pub(super) unsafe fn cert_find_extension_impl(
    psz_obj_id: LPCSTR,
    c_extensions: u32,
    rg_extensions: *mut c_void,
) -> *mut c_void {
    if psz_obj_id.is_null() || rg_extensions.is_null() || c_extensions == 0 {
        return std::ptr::null_mut();
    }
    let want = crate::strings::from_ansi(psz_obj_id);
    let base = rg_extensions as *mut WinCertExtension;
    for i in 0..c_extensions as usize {
        let ext_ptr = base.add(i);
        let ext = std::ptr::read_unaligned(ext_ptr);
        if !ext.psz_obj_id.is_null() && crate::strings::from_ansi(ext.psz_obj_id) == want {
            return ext_ptr as *mut c_void;
        }
    }
    std::ptr::null_mut()
}

// ---------------------------------------------------------------------------
// DPAPI: CryptProtectData / CryptUnprotectData (AES-256-CTR + HMAC-SHA256)
// ---------------------------------------------------------------------------

const DPAPI_MAGIC: &[u8; 8] = b"PFDPAPI2";
const CRYPTPROTECT_LOCAL_MACHINE: u32 = 0x4;

extern "C" {
    fn CCHmac(
        alg: u32,
        key: *const c_void,
        key_len: usize,
        data: *const c_void,
        data_len: usize,
        mac: *mut u8,
    );
    fn CCCryptorCreateWithMode(
        op: u32,
        mode: u32,
        alg: u32,
        padding: u32,
        iv: *const c_void,
        key: *const c_void,
        key_len: usize,
        tweak: *const c_void,
        tweak_len: usize,
        num_rounds: i32,
        options: u32,
        cryptor_ref: *mut *mut c_void,
    ) -> i32;
    fn CCCryptorUpdate(
        cryptor_ref: *mut c_void,
        data_in: *const c_void,
        data_in_len: usize,
        data_out: *mut c_void,
        data_out_avail: usize,
        data_out_moved: *mut usize,
    ) -> i32;
    fn CCCryptorRelease(cryptor_ref: *mut c_void) -> i32;
}

fn sanitize_profile_user(raw: &str) -> String {
    let clean: String = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect();
    if clean.is_empty() {
        "player".to_string()
    } else {
        clean
    }
}

fn current_dpapi_user() -> String {
    let raw = crate::process::get_env("USERNAME")
        .filter(|s| !s.trim().is_empty())
        .or_else(|| std::env::var("USER").ok().filter(|s| !s.trim().is_empty()))
        .unwrap_or_else(|| "player".to_string());
    sanitize_profile_user(&raw)
}

pub(super) fn dpapi_secret_path(local_machine: bool) -> std::path::PathBuf {
    let root = crate::path::default_translator_root();
    if local_machine {
        root.join("ProgramData")
            .join("Microsoft")
            .join("Protect")
            .join("dpapi_machine.key")
    } else {
        let user = current_dpapi_user();
        root.join("AppData")
            .join("Roaming")
            .join("Microsoft")
            .join("Protect")
            .join(format!("dpapi_user_{user}.key"))
    }
}

/// Load or atomically generate a 32-byte random host secret stored with `0600`
/// permissions under the virtual profile. Never falls back to a constant key.
fn load_or_create_dpapi_secret(local_machine: bool) -> std::io::Result<[u8; 32]> {
    use std::io::Write;
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    let path = dpapi_secret_path(local_machine);
    if let Ok(bytes) = std::fs::read(&path) {
        if bytes.len() == 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            return Ok(key);
        }
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "DPAPI host secret file has invalid length",
        ));
    }

    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "DPAPI secret path has no parent")
    })?;
    std::fs::create_dir_all(parent)?;
    let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));

    let mut secret = [0u8; 32];
    unsafe {
        libc::arc4random_buf(secret.as_mut_ptr() as *mut c_void, secret.len());
    }
    let mut rnd = [0u8; 8];
    unsafe {
        libc::arc4random_buf(rnd.as_mut_ptr() as *mut c_void, rnd.len());
    }
    let suffix = u64::from_le_bytes(rnd);
    let tmp_name = format!(
        ".{}.tmp.{}.{suffix:016x}",
        path.file_name().and_then(|n| n.to_str()).unwrap_or("dpapi.key"),
        std::process::id()
    );
    let tmp_path = parent.join(tmp_name);

    {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&tmp_path)?;
        f.write_all(&secret)?;
        f.sync_all()?;
    }

    // Use hard_link so racing processes never overwrite an already-created secret.
    match std::fs::hard_link(&tmp_path, &path) {
        Ok(()) => {
            let _ = std::fs::remove_file(&tmp_path);
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
            Ok(secret)
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            let _ = std::fs::remove_file(&tmp_path);
            let bytes = std::fs::read(&path)?;
            if bytes.len() != 32 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "DPAPI host secret file has invalid length",
                ));
            }
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            Ok(key)
        }
        Err(_) => {
            // Fallback for filesystems without hard_link: create_new directly.
            let _ = std::fs::remove_file(&tmp_path);
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&path)
            {
                Ok(mut f) => {
                    f.write_all(&secret)?;
                    f.sync_all()?;
                    Ok(secret)
                }
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    let bytes = std::fs::read(&path)?;
                    if bytes.len() != 32 {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "DPAPI host secret file has invalid length",
                        ));
                    }
                    let mut key = [0u8; 32];
                    key.copy_from_slice(&bytes);
                    Ok(key)
                }
                Err(e) => Err(e),
            }
        }
    }
}

unsafe fn dpapi_derive_keys(local_machine: bool, entropy: &[u8]) -> Option<([u8; 32], [u8; 32])> {
    let master_secret = load_or_create_dpapi_secret(local_machine).ok()?;
    let mut info = Vec::with_capacity(64 + entropy.len());
    info.extend_from_slice(b"port-factory-dpapi-v2:");
    if local_machine {
        info.extend_from_slice(b"machine");
    } else {
        info.extend_from_slice(b"user:");
        info.extend_from_slice(current_dpapi_user().as_bytes());
    }
    info.extend_from_slice(b":entropy:");
    info.extend_from_slice(&(entropy.len() as u32).to_le_bytes());
    info.extend_from_slice(entropy);

    let mut enc_info = info.clone();
    enc_info.extend_from_slice(b":enc");
    let enc_key = dpapi_hmac_sha256(&master_secret, &enc_info);

    let mut mac_info = info;
    mac_info.extend_from_slice(b":mac");
    let mac_key = dpapi_hmac_sha256(&master_secret, &mac_info);
    Some((enc_key, mac_key))
}

unsafe fn dpapi_aes256_ctr(key: &[u8; 32], iv: &[u8; 16], data: &[u8]) -> Option<Vec<u8>> {
    if data.is_empty() {
        return Some(Vec::new());
    }
    let mut cr: *mut c_void = std::ptr::null_mut();
    let st = CCCryptorCreateWithMode(
        0,
        4,
        0,
        0,
        iv.as_ptr() as *const c_void,
        key.as_ptr() as *const c_void,
        key.len(),
        std::ptr::null(),
        0,
        0,
        2,
        &mut cr,
    );
    if st != 0 || cr.is_null() {
        return None;
    }
    let mut out = vec![0u8; data.len()];
    let mut moved = 0usize;
    let st2 = CCCryptorUpdate(
        cr,
        data.as_ptr() as *const c_void,
        data.len(),
        out.as_mut_ptr() as *mut c_void,
        out.len(),
        &mut moved,
    );
    CCCryptorRelease(cr);
    if st2 != 0 || moved != data.len() {
        return None;
    }
    Some(out)
}

unsafe fn dpapi_hmac_sha256(key: &[u8; 32], data: &[u8]) -> [u8; 32] {
    let mut tag = [0u8; 32];
    CCHmac(
        2,
        key.as_ptr() as *const c_void,
        key.len(),
        data.as_ptr() as *const c_void,
        data.len(),
        tag.as_mut_ptr(),
    );
    tag
}

pub(crate) unsafe fn dpapi_protect_bytes(
    local_machine: bool,
    descr: &str,
    entropy: &[u8],
    plain: &[u8],
) -> Option<Vec<u8>> {
    let descr_bytes = descr.as_bytes();
    let scope_flags = if local_machine { CRYPTPROTECT_LOCAL_MACHINE } else { 0 };
    let (enc_key, mac_key) = dpapi_derive_keys(local_machine, entropy)?;

    let mut nonce = [0u8; 16];
    libc::arc4random_buf(nonce.as_mut_ptr() as *mut c_void, nonce.len());

    let mut combined_plain = Vec::with_capacity(descr_bytes.len() + plain.len());
    combined_plain.extend_from_slice(descr_bytes);
    combined_plain.extend_from_slice(plain);
    let ct = dpapi_aes256_ctr(&enc_key, &nonce, &combined_plain)?;

    let mut env = Vec::with_capacity(8 + 4 + 16 + 4 + 4 + ct.len() + 32);
    env.extend_from_slice(DPAPI_MAGIC);
    env.extend_from_slice(&scope_flags.to_le_bytes());
    env.extend_from_slice(&nonce);
    env.extend_from_slice(&(descr_bytes.len() as u32).to_le_bytes());
    env.extend_from_slice(&(plain.len() as u32).to_le_bytes());
    env.extend_from_slice(&ct);
    let tag = dpapi_hmac_sha256(&mac_key, &env);
    env.extend_from_slice(&tag);
    Some(env)
}

pub(crate) unsafe fn dpapi_unprotect_bytes(
    entropy: &[u8],
    env: &[u8],
) -> Option<(String, Vec<u8>)> {
    const MIN_LEN: usize = 8 + 4 + 16 + 4 + 4 + 32;
    if env.len() < MIN_LEN || &env[..8] != DPAPI_MAGIC {
        return None;
    }
    let scope_flags = u32::from_le_bytes(env[8..12].try_into().ok()?);
    let mut nonce = [0u8; 16];
    nonce.copy_from_slice(&env[12..28]);
    let descr_len = u32::from_le_bytes(env[28..32].try_into().ok()?) as usize;
    let plain_len = u32::from_le_bytes(env[32..36].try_into().ok()?) as usize;
    let body_end = env.len() - 32;
    if 36usize.saturating_add(descr_len).saturating_add(plain_len) != body_end {
        return None;
    }
    let (enc_key, mac_key) =
        dpapi_derive_keys((scope_flags & CRYPTPROTECT_LOCAL_MACHINE) != 0, entropy)?;
    let expected_tag = dpapi_hmac_sha256(&mac_key, &env[..body_end]);
    let mut diff = 0u8;
    for (a, b) in expected_tag.iter().zip(env[body_end..].iter()) {
        diff |= a ^ b;
    }
    if diff != 0 {
        return None;
    }
    let combined = dpapi_aes256_ctr(&enc_key, &nonce, &env[36..body_end])?;
    let descr = String::from_utf8_lossy(&combined[..descr_len]).into_owned();
    let plain = combined[descr_len..].to_vec();
    Some((descr, plain))
}

pub(super) unsafe fn crypt_protect_data_impl(
    p_data_in: *const c_void,
    sz_data_descr: LPCWSTR,
    p_optional_entropy: *const c_void,
    _pv_reserved: *mut c_void,
    _p_prompt_struct: *const c_void,
    dw_flags: u32,
    p_data_out: *mut c_void,
) -> BOOL {
    let Some(plain) = read_blob_slice(p_data_in) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    if p_data_out.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let entropy = read_blob_slice(p_optional_entropy).unwrap_or(&[]);
    let descr = crate::strings::from_wide_opt(sz_data_descr).unwrap_or_default();
    let local_machine = (dw_flags & CRYPTPROTECT_LOCAL_MACHINE) != 0;
    let Some(env) = dpapi_protect_bytes(local_machine, &descr, entropy, plain) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    };

    let buf = crate::heap::alloc(env.len(), false) as *mut u8;
    if buf.is_null() {
        crate::teb::set_last_error(err::ERROR_NOT_ENOUGH_MEMORY);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(env.as_ptr(), buf, env.len());
    std::ptr::write_unaligned(
        p_data_out as *mut CryptDataBlob,
        CryptDataBlob {
            cb_data: env.len() as u32,
            pb_data: buf,
        },
    );
    TRUE
}

pub(super) unsafe fn crypt_unprotect_data_impl(
    p_data_in: *const c_void,
    ppsz_data_descr: *mut LPWSTR,
    p_optional_entropy: *const c_void,
    _pv_reserved: *mut c_void,
    _p_prompt_struct: *const c_void,
    _dw_flags: u32,
    p_data_out: *mut c_void,
) -> BOOL {
    let Some(env) = read_blob_slice(p_data_in) else {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    };
    if p_data_out.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let entropy = read_blob_slice(p_optional_entropy).unwrap_or(&[]);
    let Some((descr, plain_slice)) = dpapi_unprotect_bytes(entropy, env) else {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    };

    let out_buf = if plain_slice.is_empty() {
        crate::heap::alloc(1, true) as *mut u8
    } else {
        let p = crate::heap::alloc(plain_slice.len(), false) as *mut u8;
        if p.is_null() {
            crate::teb::set_last_error(err::ERROR_NOT_ENOUGH_MEMORY);
            return FALSE;
        }
        std::ptr::copy_nonoverlapping(plain_slice.as_ptr(), p, plain_slice.len());
        p
    };
    if !ppsz_data_descr.is_null() {
        let wide: Vec<u16> = descr.encode_utf16().chain(std::iter::once(0)).collect();
        let dbuf = crate::heap::alloc(wide.len() * 2, false) as *mut u16;
        if !dbuf.is_null() {
            for (i, &u) in wide.iter().enumerate() {
                std::ptr::write_unaligned(dbuf.add(i), u);
            }
        }
        std::ptr::write_unaligned(ppsz_data_descr, dbuf);
    }
    std::ptr::write_unaligned(
        p_data_out as *mut CryptDataBlob,
        CryptDataBlob {
            cb_data: plain_slice.len() as u32,
            pb_data: out_buf,
        },
    );
    TRUE
}
