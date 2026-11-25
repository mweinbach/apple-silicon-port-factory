//! ASN.1 DER object decoding (`CryptDecodeObject` / `CryptDecodeObjectEx`),
//! X.500 DN string parsing (`CertStrToNameA/W`), OID metadata lookup (`CryptFindOIDInfo`),
//! and SPKI-to-CAPI `PUBLICKEYBLOB` conversion for `crypt32.dll`.

use super::asn1::{
    der_item_children, encode_der_tlv, parse_der_item, DerItem, PKCS_7_ASN_ENCODING,
    X509_ALTERNATE_NAME, X509_ANY_STRING, X509_ASN_ENCODING, X509_BASIC_CONSTRAINTS2, X509_BITS,
    X509_CERT, X509_CERT_TO_BE_SIGNED, X509_CHOICE_OF_TIME, X509_ECC_SIGNATURE,
    X509_ENHANCED_KEY_USAGE, X509_EXTENSIONS, X509_INTEGER, X509_KEY_USAGE,
    X509_MULTI_BYTE_INTEGER, X509_MULTI_BYTE_UINT, X509_NAME, X509_OBJECT_IDENTIFIER,
    X509_OCTET_STRING, X509_PUBLIC_KEY_INFO, X509_SEQUENCE_OF_ANY, X509_UNICODE_ANY_STRING,
    X509_UNICODE_NAME, RSA_CSP_PUBLICKEYBLOB,
};
use crate::misc::wintrust::{
    self, CertInfo, CertPublicKeyInfo, CmsgSignerInfo, CryptDataBlob,
};
use crate::types::*;
use std::ffi::c_void;
use std::sync::OnceLock;

// Additional wincrypt.h struct-type numeric identifiers
pub(super) const X509_CERT_CRL_TO_BE_SIGNED: usize = 3;
pub(super) const X509_AUTHORITY_KEY_ID: usize = 9;
pub(super) const X509_BASIC_CONSTRAINTS: usize = 13;
pub(super) const X509_CERT_POLICIES: usize = 16;
pub(super) const PKCS_UTC_TIME: usize = 17;
pub(super) const PKCS_ATTRIBUTE: usize = 22;
pub(super) const X509_ENUMERATED: usize = 29;
pub(super) const X509_AUTHORITY_KEY_ID2: usize = 31;
pub(super) const X509_AUTHORITY_INFO_ACCESS: usize = 32;
pub(super) const PKCS_CONTENT_INFO: usize = 33;
pub(super) const X509_CRL_DIST_POINTS: usize = 35;
pub(super) const X509_DSS_SIGNATURE: usize = 40;
pub(super) const PKCS_PRIVATE_KEY_INFO: usize = 44;
pub(super) const PKCS_ENCRYPTED_PRIVATE_KEY_INFO: usize = 45;
pub(super) const PKCS_ATTRIBUTES: usize = 48;
pub(super) const CNG_RSA_PUBLIC_KEY_BLOB: usize = 72;
pub(super) const X509_ALGORITHM_IDENTIFIER: usize = 74;
pub(super) const PKCS7_SIGNER_INFO: usize = 500;

pub(super) const CRYPT_DECODE_TO_BE_SIGNED_FLAG: u32 = 0x0000_0002;
pub(super) const CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG: u32 = 0x0000_0008;
const CRYPT_DECODE_ALLOC_FLAG: u32 = 0x0000_8000;
const CRYPT_E_ASN1_BADTAG: u32 = 0x8009_310B;
const CRYPT_E_INVALID_X500_STRING: u32 = 0x8009_2023;

// ---------------------------------------------------------------------------
// Single-buffer relocating DecodeArena for CryptDecodeObject / Ex
// ---------------------------------------------------------------------------

struct DecodeArena {
    buf: Vec<u8>,
    /// Byte offsets within `buf` holding a 64-bit relative offset to rebase onto `dest_base`.
    relocs: Vec<usize>,
}

impl DecodeArena {
    fn new() -> Self {
        Self {
            buf: Vec::with_capacity(256),
            relocs: Vec::with_capacity(16),
        }
    }

    fn alloc_aligned(&mut self, size: usize, align: usize) -> usize {
        let a = align.max(1);
        let rem = self.buf.len() % a;
        if rem != 0 {
            self.buf.resize(self.buf.len() + (a - rem), 0);
        }
        let off = self.buf.len();
        self.buf.resize(off + size, 0);
        off
    }

    fn push_bytes(&mut self, data: &[u8]) -> Option<usize> {
        if data.is_empty() {
            return None;
        }
        let off = self.buf.len();
        self.buf.extend_from_slice(data);
        Some(off)
    }

    fn push_cstr(&mut self, s: &str) -> usize {
        let off = self.buf.len();
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(0);
        off
    }

    fn push_wstr(&mut self, s: &str) -> usize {
        let units: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();
        let off = self.alloc_aligned(units.len() * 2, 2);
        for (i, u) in units.iter().enumerate() {
            let b = u.to_le_bytes();
            self.buf[off + i * 2] = b[0];
            self.buf[off + i * 2 + 1] = b[1];
        }
        off
    }

    fn write_u32(&mut self, off: usize, v: u32) {
        self.buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    fn write_i32(&mut self, off: usize, v: i32) {
        self.buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    fn write_u64(&mut self, off: usize, v: u64) {
        self.buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }

    fn write_ptr(&mut self, field_off: usize, target_off: Option<usize>) {
        if let Some(t) = target_off {
            self.buf[field_off..field_off + 8].copy_from_slice(&(t as u64).to_le_bytes());
            self.relocs.push(field_off);
        } else {
            self.buf[field_off..field_off + 8].copy_from_slice(&0u64.to_le_bytes());
        }
    }

    fn write_blob(&mut self, blob_off: usize, data: &[u8]) {
        self.write_u32(blob_off, data.len() as u32);
        let ptr_off = self.push_bytes(data);
        self.write_ptr(blob_off + 8, ptr_off);
    }

    fn write_bit_blob(&mut self, blob_off: usize, bytes: &[u8], unused_bits: u32) {
        self.write_u32(blob_off, bytes.len() as u32);
        let ptr_off = self.push_bytes(bytes);
        self.write_ptr(blob_off + 8, ptr_off);
        self.write_u32(blob_off + 16, unused_bits);
    }

    fn write_alg_id(&mut self, alg_off: usize, alg_elem: DerItem<'_>) -> bool {
        if alg_elem.tag != 0x30 {
            return false;
        }
        let Some(kids) = der_item_children(alg_elem.val) else {
            return false;
        };
        if kids.is_empty() || kids[0].tag != 0x06 {
            return false;
        }
        let oid_s = wintrust::decode_oid_str(kids[0].val);
        let oid_off = self.push_cstr(&oid_s);
        self.write_ptr(alg_off, Some(oid_off));
        if kids.len() >= 2 {
            self.write_blob(alg_off + 8, kids[1].raw);
        }
        true
    }

    unsafe fn finish(
        &self,
        dw_flags: u32,
        p_decode_para: *const c_void,
        pv_struct_info: *mut c_void,
        pcb_struct_info: *mut u32,
    ) -> BOOL {
        let need = self.buf.len() as u32;
        if (dw_flags & CRYPT_DECODE_ALLOC_FLAG) != 0 {
            if pv_struct_info.is_null() {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            // Per MSDN CryptDecodeObjectEx: when pDecodePara->pfnAlloc is supplied, allocate
            // through that callback so the caller's matching pfnFree releases it properly.
            // CRYPT_DECODE_PARA (Win64): cbSize @ +0x00 (u32), pfnAlloc @ +0x08 (usize), pfnFree @ +0x10 (usize)
            let mut custom_alloc: usize = 0;
            if !p_decode_para.is_null() {
                let para_base = p_decode_para as *const u8;
                let cb_para = std::ptr::read_unaligned(para_base as *const u32);
                if cb_para >= 16 {
                    custom_alloc = std::ptr::read_unaligned(para_base.add(8) as *const usize);
                }
            }
            let alloc_sz = self.buf.len().max(1);
            let mem = if custom_alloc != 0 {
                crate::call_guest8(custom_alloc, alloc_sz as u64, 0, 0, 0, 0, 0, 0, 0) as *mut u8
            } else {
                crate::kernel32::memory::LocalAlloc(0, alloc_sz) as *mut u8
            };
            if mem.is_null() {
                crate::teb::set_last_error(err::ERROR_NOT_ENOUGH_MEMORY);
                return FALSE;
            }
            self.copy_relocated_into(mem);
            std::ptr::write_unaligned(pv_struct_info as *mut *mut u8, mem);
            std::ptr::write_unaligned(pcb_struct_info, need);
            return TRUE;
        }
        if pv_struct_info.is_null() {
            std::ptr::write_unaligned(pcb_struct_info, need);
            return TRUE;
        }
        let cap = std::ptr::read_unaligned(pcb_struct_info);
        if cap < need {
            std::ptr::write_unaligned(pcb_struct_info, need);
            crate::teb::set_last_error(err::ERROR_MORE_DATA);
            return FALSE;
        }
        self.copy_relocated_into(pv_struct_info as *mut u8);
        std::ptr::write_unaligned(pcb_struct_info, need);
        TRUE
    }

    unsafe fn copy_relocated_into(&self, dst: *mut u8) {
        if !self.buf.is_empty() {
            std::ptr::copy_nonoverlapping(self.buf.as_ptr(), dst, self.buf.len());
        }
        let base = dst as usize;
        for &off in &self.relocs {
            let rel = std::ptr::read_unaligned(dst.add(off) as *const u64) as usize;
            std::ptr::write_unaligned(dst.add(off) as *mut u64, (base + rel) as u64);
        }
    }
}

// ---------------------------------------------------------------------------
// DER -> Windows struct decoders
// ---------------------------------------------------------------------------

fn decode_der_i32(val: &[u8]) -> Option<i32> {
    if val.is_empty() || val.len() > 4 {
        return None;
    }
    let sign_ext = if (val[0] & 0x80) != 0 { -1i32 } else { 0i32 };
    let mut acc = sign_ext;
    for &b in val {
        acc = (acc << 8) | (b as i32);
    }
    Some(acc)
}

fn decode_der_u32(val: &[u8]) -> Option<u32> {
    let mut s = val;
    while s.len() > 1 && s[0] == 0 {
        s = &s[1..];
    }
    if s.is_empty() || s.len() > 4 {
        return None;
    }
    let mut acc = 0u32;
    for &b in s {
        acc = (acc << 8) | (b as u32);
    }
    Some(acc)
}

fn asn1_tag_to_rdn_type(tag: u8) -> u32 {
    match tag {
        0x04 => 2,  // CERT_RDN_OCTET_STRING
        0x12 => 3,  // CERT_RDN_NUMERIC_STRING
        0x13 => 4,  // CERT_RDN_PRINTABLE_STRING
        0x14 => 5,  // CERT_RDN_TELETEX_STRING
        0x16 => 7,  // CERT_RDN_IA5_STRING
        0x1A => 9,  // CERT_RDN_VISIBLE_STRING
        0x1E => 12, // CERT_RDN_BMP_STRING
        0x0C => 13, // CERT_RDN_UTF8_STRING
        _ => 1,     // CERT_RDN_ENCODED_BLOB
    }
}

fn decode_rdn_value_into(arena: &mut DecodeArena, field_off: usize, item: DerItem<'_>, unicode: bool) {
    let rdn_ty = asn1_tag_to_rdn_type(item.tag);
    arena.write_u32(field_off, rdn_ty);
    // field_off + 8 is CryptDataBlob (16 bytes)
    match item.tag {
        0x0C | 0x12 | 0x13 | 0x14 | 0x16 | 0x1A => {
            let s = String::from_utf8_lossy(item.val);
            if unicode {
                let units: Vec<u16> = s.encode_utf16().collect();
                let mut bytes = Vec::with_capacity(units.len() * 2 + 2);
                for u in &units {
                    bytes.extend_from_slice(&u.to_le_bytes());
                }
                bytes.extend_from_slice(&[0, 0]);
                let ptr_off = arena.alloc_aligned(bytes.len(), 2);
                arena.buf[ptr_off..ptr_off + bytes.len()].copy_from_slice(&bytes);
                arena.write_u32(field_off + 8, (units.len() * 2) as u32);
                arena.write_ptr(field_off + 16, Some(ptr_off));
            } else {
                let mut bytes = s.as_bytes().to_vec();
                let len = bytes.len();
                bytes.push(0);
                let ptr_off = arena.push_bytes(&bytes);
                arena.write_u32(field_off + 8, len as u32);
                arena.write_ptr(field_off + 16, ptr_off);
            }
        }
        0x1E if item.val.len() % 2 == 0 => {
            let units: Vec<u16> = item
                .val
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            if unicode {
                let mut bytes = Vec::with_capacity(units.len() * 2 + 2);
                for u in &units {
                    bytes.extend_from_slice(&u.to_le_bytes());
                }
                bytes.extend_from_slice(&[0, 0]);
                let ptr_off = arena.alloc_aligned(bytes.len(), 2);
                arena.buf[ptr_off..ptr_off + bytes.len()].copy_from_slice(&bytes);
                arena.write_u32(field_off + 8, (units.len() * 2) as u32);
                arena.write_ptr(field_off + 16, Some(ptr_off));
            } else {
                let s = String::from_utf16_lossy(&units);
                let mut bytes = s.into_bytes();
                let len = bytes.len();
                bytes.push(0);
                let ptr_off = arena.push_bytes(&bytes);
                arena.write_u32(field_off + 8, len as u32);
                arena.write_ptr(field_off + 16, ptr_off);
            }
        }
        0x04 => {
            arena.write_blob(field_off + 8, item.val);
        }
        _ => {
            arena.write_blob(field_off + 8, item.raw);
        }
    }
}

fn decode_alt_name_entries_into(arena: &mut DecodeArena, info_off: usize, entries: &[DerItem<'_>]) -> bool {
    // CERT_ALT_NAME_INFO { cAltEntry: u32, _pad: u32, rgAltEntry: *mut CertAltNameEntry (24B each) }
    arena.write_u32(info_off, entries.len() as u32);
    if entries.is_empty() {
        return true;
    }
    let arr_off = arena.alloc_aligned(entries.len() * 24, 8);
    arena.write_ptr(info_off + 8, Some(arr_off));
    for (i, e) in entries.iter().enumerate() {
        let eo = arr_off + i * 24;
        // CertAltNameEntry { dwAltNameChoice: u32 @ 0, pad @ 4, union payload @ 8 (16 bytes) }
        match e.tag {
            0x81 => {
                // RFC822_NAME = 2 (LPWSTR)
                arena.write_u32(eo, 2);
                let s = String::from_utf8_lossy(e.val);
                let woff = arena.push_wstr(&s);
                arena.write_ptr(eo + 8, Some(woff));
            }
            0x82 => {
                // DNS_NAME = 3 (LPWSTR)
                arena.write_u32(eo, 3);
                let s = String::from_utf8_lossy(e.val);
                let woff = arena.push_wstr(&s);
                arena.write_ptr(eo + 8, Some(woff));
            }
            0xA4 => {
                // DIRECTORY_NAME = 5 (CERT_NAME_BLOB)
                arena.write_u32(eo, 5);
                arena.write_blob(eo + 8, e.val);
            }
            0x86 => {
                // URL = 7 (LPWSTR)
                arena.write_u32(eo, 7);
                let s = String::from_utf8_lossy(e.val);
                let woff = arena.push_wstr(&s);
                arena.write_ptr(eo + 8, Some(woff));
            }
            0x87 => {
                // IP_ADDRESS = 8 (CRYPT_DATA_BLOB)
                arena.write_u32(eo, 8);
                arena.write_blob(eo + 8, e.val);
            }
            0x88 => {
                // REGISTERED_ID = 9 (LPSTR OID)
                arena.write_u32(eo, 9);
                let oid_s = wintrust::decode_oid_str(e.val);
                let coff = arena.push_cstr(&oid_s);
                arena.write_ptr(eo + 8, Some(coff));
            }
            0xA0 => {
                // OTHER_NAME = 1 (PCERT_OTHER_NAME -> { pszObjId, Value: CRYPT_OBJID_BLOB })
                arena.write_u32(eo, 1);
                if let Some(okids) = der_item_children(e.val) {
                    if okids.len() >= 2 && okids[0].tag == 0x06 {
                        let on_off = arena.alloc_aligned(24, 8);
                        let oid_s = wintrust::decode_oid_str(okids[0].val);
                        let coff = arena.push_cstr(&oid_s);
                        arena.write_ptr(on_off, Some(coff));
                        let val_slice = if okids[1].tag == 0xA0 {
                            okids[1].val
                        } else {
                            okids[1].raw
                        };
                        arena.write_blob(on_off + 8, val_slice);
                        arena.write_ptr(eo + 8, Some(on_off));
                    }
                }
            }
            _ => {
                // Unknown GeneralName tag -> preserve raw as blob choice 0
                arena.write_u32(eo, 0);
                arena.write_blob(eo + 8, e.raw);
            }
        }
    }
    true
}

fn decode_extensions_into(arena: &mut DecodeArena, base_off: usize, ext_list: &[DerItem<'_>]) -> bool {
    // CERT_EXTENSIONS { cExtension: u32, _pad: u32, rgExtension: *mut CERT_EXTENSION (32B each) }
    let mut parsed = Vec::with_capacity(ext_list.len());
    for ext in ext_list {
        if ext.tag != 0x30 {
            return false;
        }
        let Some(ek) = der_item_children(ext.val) else {
            return false;
        };
        if ek.len() < 2 || ek[0].tag != 0x06 {
            return false;
        }
        let (crit, val_item) = if ek.len() >= 3 && ek[1].tag == 0x01 {
            (ek[1].val.first().copied().unwrap_or(0) != 0, ek[2])
        } else {
            (false, ek[1])
        };
        if val_item.tag != 0x04 {
            return false;
        }
        parsed.push((ek[0].val, crit, val_item.val));
    }
    arena.write_u32(base_off, parsed.len() as u32);
    if parsed.is_empty() {
        return true;
    }
    let arr_off = arena.alloc_aligned(parsed.len() * 32, 8);
    arena.write_ptr(base_off + 8, Some(arr_off));
    for (i, (oid_b, crit, val_b)) in parsed.into_iter().enumerate() {
        let eo = arr_off + i * 32;
        let oid_s = wintrust::decode_oid_str(oid_b);
        let coff = arena.push_cstr(&oid_s);
        arena.write_ptr(eo, Some(coff));
        arena.write_i32(eo + 8, i32::from(crit));
        arena.write_blob(eo + 16, val_b);
    }
    true
}

fn decode_spki_into(arena: &mut DecodeArena, spki_off: usize, spki_elem: DerItem<'_>) -> bool {
    // CertPublicKeyInfo (48B): algorithm (24B) + public_key (24B)
    if spki_elem.tag != 0x30 {
        return false;
    }
    let Some(kids) = der_item_children(spki_elem.val) else {
        return false;
    };
    if kids.len() < 2 || kids[1].tag != 0x03 || kids[1].val.is_empty() {
        return false;
    }
    if !arena.write_alg_id(spki_off, kids[0]) {
        return false;
    }
    let unused = kids[1].val[0] as u32;
    let key_bytes = &kids[1].val[1..];
    arena.write_bit_blob(spki_off + 24, key_bytes, unused);
    true
}

fn decode_attribute_into(arena: &mut DecodeArena, attr_off: usize, attr_elem: DerItem<'_>) -> bool {
    // CRYPT_ATTRIBUTE (24B): pszObjId (*const u8 @ 0), cValue (u32 @ 8), rgValue (*mut CryptDataBlob @ 16)
    if attr_elem.tag != 0x30 {
        return false;
    }
    let Some(kids) = der_item_children(attr_elem.val) else {
        return false;
    };
    if kids.len() < 2 || kids[0].tag != 0x06 || kids[1].tag != 0x31 {
        return false;
    }
    let oid_s = wintrust::decode_oid_str(kids[0].val);
    let coff = arena.push_cstr(&oid_s);
    arena.write_ptr(attr_off, Some(coff));
    let vals = der_item_children(kids[1].val).unwrap_or_default();
    arena.write_u32(attr_off + 8, vals.len() as u32);
    if !vals.is_empty() {
        let arr_off = arena.alloc_aligned(vals.len() * 16, 8);
        arena.write_ptr(attr_off + 16, Some(arr_off));
        for (i, v) in vals.iter().enumerate() {
            arena.write_blob(arr_off + i * 16, v.raw);
        }
    }
    true
}

fn decode_attributes_list_into(arena: &mut DecodeArena, base_off: usize, elems: &[DerItem<'_>]) -> bool {
    // CRYPT_ATTRIBUTES (16B): cAttr (u32 @ 0), rgAttr (*mut CRYPT_ATTRIBUTE @ 8)
    arena.write_u32(base_off, elems.len() as u32);
    if elems.is_empty() {
        return true;
    }
    let arr_off = arena.alloc_aligned(elems.len() * 24, 8);
    arena.write_ptr(base_off + 8, Some(arr_off));
    for (i, e) in elems.iter().enumerate() {
        if !decode_attribute_into(arena, arr_off + i * 24, *e) {
            return false;
        }
    }
    true
}

fn decode_x509_object(lpsz_struct_type: *const u8, der: &[u8], dw_flags: u32) -> Result<DecodeArena, u32> {
    let raw_stype = lpsz_struct_type as usize;
    let stype: usize = if raw_stype <= 0xFFFF {
        raw_stype
    } else {
        let s = unsafe { crate::strings::from_ansi(lpsz_struct_type) };
        match s.as_str() {
            "2.5.29.14" => X509_OCTET_STRING,
            "2.5.29.15" => X509_KEY_USAGE,
            "2.5.29.17" | "2.5.29.18" => X509_ALTERNATE_NAME,
            "2.5.29.19" => X509_BASIC_CONSTRAINTS2,
            "2.5.29.31" => X509_CRL_DIST_POINTS,
            "2.5.29.32" => X509_CERT_POLICIES,
            "2.5.29.35" => X509_AUTHORITY_KEY_ID2,
            "2.5.29.37" => X509_ENHANCED_KEY_USAGE,
            "1.3.6.1.5.5.7.1.1" | "1.3.6.1.5.5.7.1.11" => X509_AUTHORITY_INFO_ACCESS,
            _ => 0,
        }
    };

    let (root, _) = parse_der_item(der).ok_or(CRYPT_E_ASN1_BADTAG)?;
    let mut arena = DecodeArena::new();

    match stype {
        X509_PUBLIC_KEY_INFO => {
            let off = arena.alloc_aligned(std::mem::size_of::<CertPublicKeyInfo>(), 8);
            if !decode_spki_into(&mut arena, off, root) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            Ok(arena)
        }
        X509_BASIC_CONSTRAINTS2 => {
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let mut f_ca = 0i32;
            let mut f_path = 0i32;
            let mut path_len = 0u32;
            for k in kids {
                if k.tag == 0x01 {
                    f_ca = i32::from(k.val.first().copied().unwrap_or(0) != 0);
                } else if k.tag == 0x02 {
                    f_path = 1;
                    path_len = decode_der_u32(k.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
                }
            }
            let off = arena.alloc_aligned(12, 4);
            arena.write_i32(off, f_ca);
            arena.write_i32(off + 4, f_path);
            arena.write_u32(off + 8, path_len);
            Ok(arena)
        }
        X509_BASIC_CONSTRAINTS => {
            // CERT_BASIC_CONSTRAINTS_INFO (48B):
            // SubjectType (CryptBitBlob 24B @ 0), fPathLenConstraint (BOOL @ 24),
            // dwPathLenConstraint (u32 @ 28), cSubtreesConstraint (u32 @ 32), rgSubtreesConstraint (*const CryptDataBlob @ 40)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(48, 8);
            if let Some(first) = kids.first() {
                if first.tag == 0x03 && !first.val.is_empty() {
                    arena.write_bit_blob(off, &first.val[1..], first.val[0] as u32);
                }
            }
            for k in kids.iter().skip(1) {
                if k.tag == 0x02 {
                    arena.write_i32(off + 24, 1);
                    arena.write_u32(off + 28, decode_der_u32(k.val).unwrap_or(0));
                }
            }
            Ok(arena)
        }
        X509_ENHANCED_KEY_USAGE => {
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            // CERT_ENHKEY_USAGE (16B): cUsageIdentifier (u32 @ 0), rgpszUsageIdentifier (*mut *const u8 @ 8)
            let off = arena.alloc_aligned(16, 8);
            arena.write_u32(off, kids.len() as u32);
            if !kids.is_empty() {
                let arr_off = arena.alloc_aligned(kids.len() * 8, 8);
                arena.write_ptr(off + 8, Some(arr_off));
                for (i, k) in kids.iter().enumerate() {
                    if k.tag != 0x06 {
                        return Err(CRYPT_E_ASN1_BADTAG);
                    }
                    let oid_s = wintrust::decode_oid_str(k.val);
                    let coff = arena.push_cstr(&oid_s);
                    arena.write_ptr(arr_off + i * 8, Some(coff));
                }
            }
            Ok(arena)
        }
        X509_ALTERNATE_NAME => {
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let entries = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(16, 8);
            decode_alt_name_entries_into(&mut arena, off, &entries);
            Ok(arena)
        }
        X509_NAME | X509_UNICODE_NAME => {
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let unicode = stype == X509_UNICODE_NAME;
            let rdns = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            // CERT_NAME_INFO (16B): cRDN (u32 @ 0), rgRDN (*mut CERT_RDN @ 8)
            let off = arena.alloc_aligned(16, 8);
            arena.write_u32(off, rdns.len() as u32);
            if !rdns.is_empty() {
                let rdn_arr = arena.alloc_aligned(rdns.len() * 16, 8);
                arena.write_ptr(off + 8, Some(rdn_arr));
                for (i, rdn) in rdns.iter().enumerate() {
                    if rdn.tag != 0x31 {
                        return Err(CRYPT_E_ASN1_BADTAG);
                    }
                    let atvs = der_item_children(rdn.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
                    let ro = rdn_arr + i * 16;
                    arena.write_u32(ro, atvs.len() as u32);
                    if !atvs.is_empty() {
                        // CERT_RDN_ATTR (32B): pszObjId (*const u8 @ 0), dwValueType (u32 @ 8), Value (CryptDataBlob 16B @ 16)
                        let attr_arr = arena.alloc_aligned(atvs.len() * 32, 8);
                        arena.write_ptr(ro + 8, Some(attr_arr));
                        for (j, atv) in atvs.iter().enumerate() {
                            if atv.tag != 0x30 {
                                return Err(CRYPT_E_ASN1_BADTAG);
                            }
                            let ak = der_item_children(atv.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
                            if ak.len() < 2 || ak[0].tag != 0x06 {
                                return Err(CRYPT_E_ASN1_BADTAG);
                            }
                            let ao = attr_arr + j * 32;
                            let oid_s = wintrust::decode_oid_str(ak[0].val);
                            let coff = arena.push_cstr(&oid_s);
                            arena.write_ptr(ao, Some(coff));
                            decode_rdn_value_into(&mut arena, ao + 8, ak[1], unicode);
                        }
                    }
                }
            }
            Ok(arena)
        }
        X509_ANY_STRING | X509_UNICODE_ANY_STRING => {
            // CERT_NAME_VALUE (24B): dwValueType (u32 @ 0), Value (CryptDataBlob 16B @ 8)
            let off = arena.alloc_aligned(24, 8);
            decode_rdn_value_into(&mut arena, off, root, stype == X509_UNICODE_ANY_STRING);
            Ok(arena)
        }
        X509_EXTENSIONS => {
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let ext_list = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(16, 8);
            if !decode_extensions_into(&mut arena, off, &ext_list) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            Ok(arena)
        }
        X509_OCTET_STRING => {
            if root.tag != 0x04 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(16, 8);
            arena.write_blob(off, root.val);
            Ok(arena)
        }
        X509_BITS | X509_KEY_USAGE => {
            if root.tag != 0x03 || root.val.is_empty() {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(24, 8);
            arena.write_bit_blob(off, &root.val[1..], root.val[0] as u32);
            Ok(arena)
        }
        X509_INTEGER | X509_ENUMERATED => {
            let want_tag = if stype == X509_ENUMERATED { 0x0A } else { 0x02 };
            if root.tag != want_tag {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let v = decode_der_i32(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(4, 4);
            arena.write_i32(off, v);
            Ok(arena)
        }
        X509_MULTI_BYTE_INTEGER | X509_MULTI_BYTE_UINT => {
            if root.tag != 0x02 || root.val.is_empty() {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let mut le = if stype == X509_MULTI_BYTE_UINT {
                let mut s = root.val;
                while s.len() > 1 && s[0] == 0 {
                    s = &s[1..];
                }
                s.to_vec()
            } else {
                root.val.to_vec()
            };
            le.reverse();
            let off = arena.alloc_aligned(16, 8);
            arena.write_blob(off, &le);
            Ok(arena)
        }
        X509_CHOICE_OF_TIME | PKCS_UTC_TIME => {
            if root.tag != 0x17 && root.tag != 0x18 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let ft = wintrust::parse_asn1_time_filetime_raw(root.tag, root.val);
            if ft == 0 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(8, 8);
            arena.write_u64(off, ft);
            Ok(arena)
        }
        X509_SEQUENCE_OF_ANY => {
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(16, 8);
            arena.write_u32(off, kids.len() as u32);
            if !kids.is_empty() {
                let arr_off = arena.alloc_aligned(kids.len() * 16, 8);
                arena.write_ptr(off + 8, Some(arr_off));
                for (i, k) in kids.iter().enumerate() {
                    arena.write_blob(arr_off + i * 16, k.raw);
                }
            }
            Ok(arena)
        }
        X509_OBJECT_IDENTIFIER => {
            if root.tag != 0x06 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            // Output is a struct containing a single LPSTR (8B) pointing to the NUL-terminated OID.
            let off = arena.alloc_aligned(8, 8);
            let oid_s = wintrust::decode_oid_str(root.val);
            let coff = arena.push_cstr(&oid_s);
            arena.write_ptr(off, Some(coff));
            Ok(arena)
        }
        X509_ALGORITHM_IDENTIFIER => {
            let off = arena.alloc_aligned(24, 8);
            if !arena.write_alg_id(off, root) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            Ok(arena)
        }
        X509_ECC_SIGNATURE => {
            // CERT_ECC_SIGNATURE (32B): r (CryptDataBlob 16B @ 0), s (CryptDataBlob 16B @ 16), LE
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() != 2 || kids[0].tag != 0x02 || kids[1].tag != 0x02 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let strip = |mut b: &[u8]| -> Vec<u8> {
                while b.len() > 1 && b[0] == 0 {
                    b = &b[1..];
                }
                let mut v = b.to_vec();
                v.reverse();
                v
            };
            let r_le = strip(kids[0].val);
            let s_le = strip(kids[1].val);
            let off = arena.alloc_aligned(32, 8);
            arena.write_blob(off, &r_le);
            arena.write_blob(off + 16, &s_le);
            Ok(arena)
        }
        X509_DSS_SIGNATURE => {
            // 40-byte raw r || s (each 20 bytes little-endian)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() != 2 || kids[0].tag != 0x02 || kids[1].tag != 0x02 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(40, 1);
            for (idx, comp) in kids.iter().enumerate() {
                let mut b = comp.val;
                while b.len() > 1 && b[0] == 0 {
                    b = &b[1..];
                }
                if b.len() > 20 {
                    return Err(CRYPT_E_ASN1_BADTAG);
                }
                for (j, &byte) in b.iter().rev().enumerate() {
                    arena.buf[off + idx * 20 + j] = byte;
                }
            }
            Ok(arena)
        }
        RSA_CSP_PUBLICKEYBLOB => {
            // DER RSAPublicKey SEQUENCE { modulus INTEGER, publicExponent INTEGER }
            // -> PUBLICKEYSTRUC (8B) + RSAPUBKEY (12B) + LE modulus
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() != 2 || kids[0].tag != 0x02 || kids[1].tag != 0x02 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let mut m = kids[0].val;
            while m.len() > 1 && m[0] == 0 {
                m = &m[1..];
            }
            let exp = decode_der_u32(kids[1].val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let mod_len = m.len();
            let off = arena.alloc_aligned(20 + mod_len, 4);
            // PUBLICKEYSTRUC: bType=PUBLICKEYBLOB(6), bVersion=2, reserved=0, aiKeyAlg=CALG_RSA_KEYX(0x0000a400)
            arena.buf[off] = 6;
            arena.buf[off + 1] = 2;
            arena.write_u32(off + 4, 0x0000_A400);
            // RSAPUBKEY: magic="RSA1" (0x31415352), bitlen, pubexp
            arena.write_u32(off + 8, 0x3141_5352);
            arena.write_u32(off + 12, (mod_len * 8) as u32);
            arena.write_u32(off + 16, exp);
            for (i, &b) in m.iter().rev().enumerate() {
                arena.buf[off + 20 + i] = b;
            }
            Ok(arena)
        }
        CNG_RSA_PUBLIC_KEY_BLOB => {
            // BCRYPT_RSAKEY_BLOB (24B) + BE exponent + BE modulus
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() != 2 || kids[0].tag != 0x02 || kids[1].tag != 0x02 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let mut m = kids[0].val;
            while m.len() > 1 && m[0] == 0 {
                m = &m[1..];
            }
            let mut e = kids[1].val;
            while e.len() > 1 && e[0] == 0 {
                e = &e[1..];
            }
            let off = arena.alloc_aligned(24 + e.len() + m.len(), 4);
            arena.write_u32(off, 0x3141_5352); // BCRYPT_RSAPUBLIC_MAGIC "RSA1"
            arena.write_u32(off + 4, (m.len() * 8) as u32);
            arena.write_u32(off + 8, e.len() as u32);
            arena.write_u32(off + 12, m.len() as u32);
            arena.write_u32(off + 16, 0);
            arena.write_u32(off + 20, 0);
            arena.buf[off + 24..off + 24 + e.len()].copy_from_slice(e);
            arena.buf[off + 24 + e.len()..off + 24 + e.len() + m.len()].copy_from_slice(m);
            Ok(arena)
        }
        X509_AUTHORITY_KEY_ID => {
            // CERT_AUTHORITY_KEY_ID_INFO (48B): KeyId (16B @ 0), CertIssuer (16B @ 16), CertSerialNumber (16B @ 32)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(48, 8);
            for k in kids {
                match k.tag {
                    0x80 => arena.write_blob(off, k.val),
                    0xA1 => arena.write_blob(off + 16, k.val),
                    0x82 => {
                        let mut le = k.val.to_vec();
                        le.reverse();
                        arena.write_blob(off + 32, &le);
                    }
                    _ => {}
                }
            }
            Ok(arena)
        }
        X509_AUTHORITY_KEY_ID2 => {
            // CERT_AUTHORITY_KEY_ID2_INFO (48B):
            // KeyId (CryptDataBlob 16B @ 0), AuthorityCertIssuer (CertAltNameInfo 16B @ 16),
            // AuthorityCertSerialNumber (CryptIntegerBlob 16B @ 32)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(48, 8);
            for k in kids {
                match k.tag {
                    0x80 => arena.write_blob(off, k.val),
                    0xA1 => {
                        if let Some(entries) = der_item_children(k.val) {
                            decode_alt_name_entries_into(&mut arena, off + 16, &entries);
                        }
                    }
                    0x82 => {
                        let mut le = k.val.to_vec();
                        le.reverse();
                        arena.write_blob(off + 32, &le);
                    }
                    _ => {}
                }
            }
            Ok(arena)
        }
        X509_AUTHORITY_INFO_ACCESS => {
            // CERT_AUTHORITY_INFO_ACCESS (16B): cAccDescr (u32 @ 0), rgAccDescr (*mut CERT_ACCESS_DESCRIPTION @ 8)
            // CERT_ACCESS_DESCRIPTION (32B): pszAccessMethod (*const u8 @ 0), AccessLocation (CertAltNameEntry 24B @ 8)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(16, 8);
            arena.write_u32(off, kids.len() as u32);
            if !kids.is_empty() {
                let arr_off = arena.alloc_aligned(kids.len() * 32, 8);
                arena.write_ptr(off + 8, Some(arr_off));
                for (i, k) in kids.iter().enumerate() {
                    if k.tag != 0x30 {
                        return Err(CRYPT_E_ASN1_BADTAG);
                    }
                    let ak = der_item_children(k.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
                    if ak.len() < 2 || ak[0].tag != 0x06 {
                        return Err(CRYPT_E_ASN1_BADTAG);
                    }
                    let ao = arr_off + i * 32;
                    let oid_s = wintrust::decode_oid_str(ak[0].val);
                    let coff = arena.push_cstr(&oid_s);
                    arena.write_ptr(ao, Some(coff));
                    // Single CertAltNameEntry at ao + 8
                    let loc = ak[1];
                    match loc.tag {
                        0x86 => {
                            arena.write_u32(ao + 8, 7); // URL
                            let woff = arena.push_wstr(&String::from_utf8_lossy(loc.val));
                            arena.write_ptr(ao + 16, Some(woff));
                        }
                        0x82 => {
                            arena.write_u32(ao + 8, 3); // DNS
                            let woff = arena.push_wstr(&String::from_utf8_lossy(loc.val));
                            arena.write_ptr(ao + 16, Some(woff));
                        }
                        0x81 => {
                            arena.write_u32(ao + 8, 2); // RFC822
                            let woff = arena.push_wstr(&String::from_utf8_lossy(loc.val));
                            arena.write_ptr(ao + 16, Some(woff));
                        }
                        0xA4 => {
                            arena.write_u32(ao + 8, 5); // DIRECTORY_NAME
                            arena.write_blob(ao + 16, loc.val);
                        }
                        0x87 => {
                            arena.write_u32(ao + 8, 8); // IP_ADDRESS
                            arena.write_blob(ao + 16, loc.val);
                        }
                        _ => {
                            arena.write_u32(ao + 8, 0);
                            arena.write_blob(ao + 16, loc.raw);
                        }
                    }
                }
            }
            Ok(arena)
        }
        X509_CERT_POLICIES => {
            // CERT_POLICIES_INFO (16B): cPolicyInfo (u32 @ 0), rgPolicyInfo (*mut CERT_POLICY_INFO @ 8)
            // CERT_POLICY_INFO (24B): pszPolicyIdentifier (*const u8 @ 0), cPolicyQualifier (u32 @ 8), rgPolicyQualifier (*mut CERT_POLICY_QUALIFIER_INFO @ 16)
            // CERT_POLICY_QUALIFIER_INFO (24B): pszPolicyQualifierId (*const u8 @ 0), Qualifier (CryptDataBlob 16B @ 8)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(16, 8);
            arena.write_u32(off, kids.len() as u32);
            if !kids.is_empty() {
                let arr_off = arena.alloc_aligned(kids.len() * 24, 8);
                arena.write_ptr(off + 8, Some(arr_off));
                for (i, k) in kids.iter().enumerate() {
                    if k.tag != 0x30 {
                        return Err(CRYPT_E_ASN1_BADTAG);
                    }
                    let pk = der_item_children(k.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
                    if pk.is_empty() || pk[0].tag != 0x06 {
                        return Err(CRYPT_E_ASN1_BADTAG);
                    }
                    let po = arr_off + i * 24;
                    let oid_s = wintrust::decode_oid_str(pk[0].val);
                    let coff = arena.push_cstr(&oid_s);
                    arena.write_ptr(po, Some(coff));
                    if pk.len() >= 2 && pk[1].tag == 0x30 {
                        if let Some(quals) = der_item_children(pk[1].val) {
                            arena.write_u32(po + 8, quals.len() as u32);
                            if !quals.is_empty() {
                                let qarr = arena.alloc_aligned(quals.len() * 24, 8);
                                arena.write_ptr(po + 16, Some(qarr));
                                for (j, q) in quals.iter().enumerate() {
                                    if let Some(qk) = der_item_children(q.val) {
                                        if !qk.is_empty() && qk[0].tag == 0x06 {
                                            let qo = qarr + j * 24;
                                            let qoid = wintrust::decode_oid_str(qk[0].val);
                                            let qcoff = arena.push_cstr(&qoid);
                                            arena.write_ptr(qo, Some(qcoff));
                                            if qk.len() >= 2 {
                                                arena.write_blob(qo + 8, qk[1].raw);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(arena)
        }
        X509_CRL_DIST_POINTS => {
            // CRL_DIST_POINTS_INFO (16B): cDistPoint (u32 @ 0), rgDistPoint (*mut CRL_DIST_POINT @ 8)
            // CRL_DIST_POINT (64B):
            //   DistPointName (24B @ 0: dwDistPointNameChoice u32 @ 0, FullName CertAltNameInfo 16B @ 8)
            //   ReasonFlags (CryptBitBlob 24B @ 24)
            //   CRLIssuer (CertAltNameInfo 16B @ 48)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(16, 8);
            arena.write_u32(off, kids.len() as u32);
            if !kids.is_empty() {
                let arr_off = arena.alloc_aligned(kids.len() * 64, 8);
                arena.write_ptr(off + 8, Some(arr_off));
                for (i, dp) in kids.iter().enumerate() {
                    if dp.tag != 0x30 {
                        return Err(CRYPT_E_ASN1_BADTAG);
                    }
                    let dpk = der_item_children(dp.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
                    let dpo = arr_off + i * 64;
                    for item in dpk {
                        match item.tag {
                            0xA0 => {
                                // DistributionPointName [0] EXPLICIT
                                if let Some((inner, _)) = parse_der_item(item.val) {
                                    if inner.tag == 0xA0 {
                                        // CRL_DIST_POINT_FULL_NAME = 1
                                        arena.write_u32(dpo, 1);
                                        if let Some(entries) = der_item_children(inner.val) {
                                            decode_alt_name_entries_into(&mut arena, dpo + 8, &entries);
                                        }
                                    }
                                }
                            }
                            0x81 if !item.val.is_empty() => {
                                // reasons [1] IMPLICIT BIT STRING
                                arena.write_bit_blob(dpo + 24, &item.val[1..], item.val[0] as u32);
                            }
                            0xA2 => {
                                // cRLIssuer [2] IMPLICIT GeneralNames
                                if let Some(entries) = der_item_children(item.val) {
                                    decode_alt_name_entries_into(&mut arena, dpo + 48, &entries);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(arena)
        }
        PKCS_CONTENT_INFO => {
            // CRYPT_CONTENT_INFO (24B): pszObjId (*const u8 @ 0), Content (CryptDataBlob 16B @ 8)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.is_empty() || kids[0].tag != 0x06 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(24, 8);
            let oid_s = wintrust::decode_oid_str(kids[0].val);
            let coff = arena.push_cstr(&oid_s);
            arena.write_ptr(off, Some(coff));
            if kids.len() >= 2 && kids[1].tag == 0xA0 {
                arena.write_blob(off + 8, kids[1].val);
            }
            Ok(arena)
        }
        PKCS_ATTRIBUTE => {
            let off = arena.alloc_aligned(24, 8);
            if !decode_attribute_into(&mut arena, off, root) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            Ok(arena)
        }
        PKCS_ATTRIBUTES => {
            if root.tag != 0x30 && root.tag != 0x31 && root.tag != 0xA0 && root.tag != 0xA1 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let off = arena.alloc_aligned(16, 8);
            if !decode_attributes_list_into(&mut arena, off, &kids) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            Ok(arena)
        }
        PKCS_PRIVATE_KEY_INFO => {
            // CRYPT_PRIVATE_KEY_INFO (64B):
            // Version (u32 @ 0), Algorithm (CryptAlgId 24B @ 8), PrivateKey (CryptDataBlob 16B @ 32), pAttributes (*mut CRYPT_ATTRIBUTES @ 48)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() < 3 || kids[0].tag != 0x02 || kids[2].tag != 0x04 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(56, 8);
            arena.write_u32(off, decode_der_u32(kids[0].val).unwrap_or(0));
            if !arena.write_alg_id(off + 8, kids[1]) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            arena.write_blob(off + 32, kids[2].val);
            if kids.len() >= 4 && kids[3].tag == 0xA0 {
                if let Some(attrs) = der_item_children(kids[3].val) {
                    let aoff = arena.alloc_aligned(16, 8);
                    if decode_attributes_list_into(&mut arena, aoff, &attrs) {
                        arena.write_ptr(off + 48, Some(aoff));
                    }
                }
            }
            Ok(arena)
        }
        PKCS_ENCRYPTED_PRIVATE_KEY_INFO => {
            // CRYPT_ENCRYPTED_PRIVATE_KEY_INFO (40B): EncryptionAlgorithm (CryptAlgId 24B @ 0), EncryptedPrivateKey (CryptDataBlob 16B @ 24)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() < 2 || kids[1].tag != 0x04 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(40, 8);
            if !arena.write_alg_id(off, kids[0]) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            arena.write_blob(off + 24, kids[1].val);
            Ok(arena)
        }
        X509_CERT => {
            // CERT_SIGNED_CONTENT_INFO (64B): ToBeSigned (16B @ 0), SignatureAlgorithm (24B @ 16), Signature (24B @ 40)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() < 3 || kids[2].tag != 0x03 || kids[2].val.is_empty() {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(64, 8);
            arena.write_blob(off, kids[0].raw);
            if !arena.write_alg_id(off + 16, kids[1]) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let sig_bytes = &kids[2].val[1..];
            if (dw_flags & CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG) == 0 {
                let mut rev = sig_bytes.to_vec();
                rev.reverse();
                arena.write_bit_blob(off + 40, &rev, kids[2].val[0] as u32);
            } else {
                arena.write_bit_blob(off + 40, sig_bytes, kids[2].val[0] as u32);
            }
            Ok(arena)
        }
        X509_CERT_TO_BE_SIGNED => {
            // CERT_INFO (208B, matches wintrust::CertInfo exactly).
            // Per MSDN CryptDecodeObjectEx: by default (dwFlags without CRYPT_DECODE_TO_BE_SIGNED_FLAG)
            // pbEncoded is a signed Certificate wrapper SEQUENCE { tbsCertificate, sigAlg, sigVal };
            // unwrap tbsCertificate unless CRYPT_DECODE_TO_BE_SIGNED_FLAG indicates unsigned TBS input.
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let outer_kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let kids = if (dw_flags & CRYPT_DECODE_TO_BE_SIGNED_FLAG) == 0
                && outer_kids.len() == 3
                && outer_kids[0].tag == 0x30
                && outer_kids[1].tag == 0x30
                && outer_kids[2].tag == 0x03
            {
                der_item_children(outer_kids[0].val).ok_or(CRYPT_E_ASN1_BADTAG)?
            } else {
                outer_kids
            };
            let off = arena.alloc_aligned(std::mem::size_of::<CertInfo>(), 8);
            let mut idx = 0usize;
            let mut ver = 0u32;
            if kids.get(idx).is_some_and(|k| k.tag == 0xA0) {
                if let Some((ve, _)) = parse_der_item(kids[idx].val) {
                    if ve.tag == 0x02 {
                        ver = decode_der_u32(ve.val).unwrap_or(0);
                    }
                }
                idx += 1;
            }
            let serial = *kids.get(idx).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let sig_alg = *kids.get(idx + 1).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let issuer = *kids.get(idx + 2).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let validity = *kids.get(idx + 3).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let subject = *kids.get(idx + 4).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let spki = *kids.get(idx + 5).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if serial.tag != 0x02 || issuer.tag != 0x30 || validity.tag != 0x30 || subject.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            arena.write_u32(off + std::mem::offset_of!(CertInfo, dw_version), ver);
            let mut ser_le = serial.val.to_vec();
            ser_le.reverse();
            arena.write_blob(off + std::mem::offset_of!(CertInfo, serial_number), &ser_le);
            if !arena.write_alg_id(off + std::mem::offset_of!(CertInfo, signature_algorithm), sig_alg) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            arena.write_blob(off + std::mem::offset_of!(CertInfo, issuer), issuer.raw);
            let vk = der_item_children(validity.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if vk.len() < 2 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let nb = wintrust::parse_asn1_time_filetime_raw(vk[0].tag, vk[0].val);
            let na = wintrust::parse_asn1_time_filetime_raw(vk[1].tag, vk[1].val);
            arena.write_u64(off + std::mem::offset_of!(CertInfo, not_before), nb);
            arena.write_u64(off + std::mem::offset_of!(CertInfo, not_after), na);
            arena.write_blob(off + std::mem::offset_of!(CertInfo, subject), subject.raw);
            if !decode_spki_into(&mut arena, off + std::mem::offset_of!(CertInfo, subject_public_key_info), spki) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            for extra in kids.iter().skip(idx + 6) {
                if extra.tag == 0xA3 {
                    if let Some((ext_seq, _)) = parse_der_item(extra.val) {
                        if ext_seq.tag == 0x30 {
                            if let Some(ext_list) = der_item_children(ext_seq.val) {
                                decode_extensions_into(
                                    &mut arena,
                                    off + std::mem::offset_of!(CertInfo, c_extension),
                                    &ext_list,
                                );
                            }
                        }
                    }
                }
            }
            Ok(arena)
        }
        X509_CERT_CRL_TO_BE_SIGNED => {
            // CRL_INFO (96B): dwVersion(u32 @ 0), SignatureAlgorithm(24B @ 8), Issuer(16B @ 32),
            // ThisUpdate(u64 @ 48), NextUpdate(u64 @ 56), cCRLEntry(u32 @ 64), rgCRLEntry(*mut CRL_ENTRY @ 72),
            // cExtension(u32 @ 80), rgExtension(*mut CERT_EXTENSION @ 88)
            // Unwrap signed CertificateList wrapper unless CRYPT_DECODE_TO_BE_SIGNED_FLAG is set.
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let outer_kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let kids = if (dw_flags & CRYPT_DECODE_TO_BE_SIGNED_FLAG) == 0
                && outer_kids.len() == 3
                && outer_kids[0].tag == 0x30
                && outer_kids[1].tag == 0x30
                && outer_kids[2].tag == 0x03
            {
                der_item_children(outer_kids[0].val).ok_or(CRYPT_E_ASN1_BADTAG)?
            } else {
                outer_kids
            };
            let off = arena.alloc_aligned(96, 8);
            let mut idx = 0usize;
            let mut ver = 0u32;
            if kids.get(idx).is_some_and(|k| k.tag == 0x02) {
                ver = decode_der_u32(kids[idx].val).unwrap_or(0);
                idx += 1;
            }
            arena.write_u32(off, ver);
            let sig_alg = *kids.get(idx).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let issuer = *kids.get(idx + 1).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let this_upd = *kids.get(idx + 2).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if !arena.write_alg_id(off + 8, sig_alg) || issuer.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            arena.write_blob(off + 32, issuer.raw);
            arena.write_u64(off + 48, wintrust::parse_asn1_time_filetime_raw(this_upd.tag, this_upd.val));
            let mut pos = idx + 3;
            if kids.get(pos).is_some_and(|k| k.tag == 0x17 || k.tag == 0x18) {
                arena.write_u64(off + 56, wintrust::parse_asn1_time_filetime_raw(kids[pos].tag, kids[pos].val));
                pos += 1;
            }
            if kids.get(pos).is_some_and(|k| k.tag == 0x30) {
                // revokedCertificates SEQUENCE OF SEQUENCE { userCertificate INTEGER, revocationDate Time, crlEntryExtensions OPTIONAL }
                let entries = der_item_children(kids[pos].val).unwrap_or_default();
                arena.write_u32(off + 64, entries.len() as u32);
                if !entries.is_empty() {
                    // CRL_ENTRY (40B): SerialNumber (16B @ 0), RevocationDate (u64 @ 16), cExtension (u32 @ 24), rgExtension (*mut @ 32)
                    let earr = arena.alloc_aligned(entries.len() * 40, 8);
                    arena.write_ptr(off + 72, Some(earr));
                    for (i, re) in entries.iter().enumerate() {
                        if let Some(rk) = der_item_children(re.val) {
                            if rk.len() >= 2 && rk[0].tag == 0x02 {
                                let eo = earr + i * 40;
                                let mut ser_le = rk[0].val.to_vec();
                                ser_le.reverse();
                                arena.write_blob(eo, &ser_le);
                                arena.write_u64(eo + 16, wintrust::parse_asn1_time_filetime_raw(rk[1].tag, rk[1].val));
                                if rk.len() >= 3 && rk[2].tag == 0x30 {
                                    if let Some(el) = der_item_children(rk[2].val) {
                                        decode_extensions_into(&mut arena, eo + 24, &el);
                                    }
                                }
                            }
                        }
                    }
                }
                pos += 1;
            }
            if kids.get(pos).is_some_and(|k| k.tag == 0xA0) {
                if let Some((ext_seq, _)) = parse_der_item(kids[pos].val) {
                    if ext_seq.tag == 0x30 {
                        if let Some(ext_list) = der_item_children(ext_seq.val) {
                            decode_extensions_into(&mut arena, off + 80, &ext_list);
                        }
                    }
                }
            }
            Ok(arena)
        }
        PKCS7_SIGNER_INFO => {
            // CMSG_SIGNER_INFO (0x88 = 136B)
            if root.tag != 0x30 {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let kids = der_item_children(root.val).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if kids.len() < 5 || kids[0].tag != 0x02 || (kids[1].tag != 0x30 && kids[1].tag != 0x80) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let off = arena.alloc_aligned(std::mem::size_of::<CmsgSignerInfo>(), 8);
            arena.write_u32(
                off + std::mem::offset_of!(CmsgSignerInfo, dw_version),
                decode_der_u32(kids[0].val).unwrap_or(1),
            );
            if kids[1].tag == 0x30 {
                let ias = der_item_children(kids[1].val).ok_or(CRYPT_E_ASN1_BADTAG)?;
                if ias.len() < 2 || ias[0].tag != 0x30 || ias[1].tag != 0x02 {
                    return Err(CRYPT_E_ASN1_BADTAG);
                }
                arena.write_blob(off + std::mem::offset_of!(CmsgSignerInfo, issuer), ias[0].raw);
                let mut ser_le = ias[1].val.to_vec();
                ser_le.reverse();
                arena.write_blob(off + std::mem::offset_of!(CmsgSignerInfo, serial_number), &ser_le);
            }
            if !arena.write_alg_id(off + std::mem::offset_of!(CmsgSignerInfo, hash_algorithm), kids[2]) {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            let mut pos = 3usize;
            if kids.get(pos).is_some_and(|k| k.tag == 0xA0) {
                if let Some(aa) = der_item_children(kids[pos].val) {
                    decode_attributes_list_into(
                        &mut arena,
                        off + std::mem::offset_of!(CmsgSignerInfo, auth_attrs_count),
                        &aa,
                    );
                }
                pos += 1;
            }
            let enc_alg = *kids.get(pos).ok_or(CRYPT_E_ASN1_BADTAG)?;
            let enc_hash = *kids.get(pos + 1).ok_or(CRYPT_E_ASN1_BADTAG)?;
            if !arena.write_alg_id(
                off + std::mem::offset_of!(CmsgSignerInfo, hash_encryption_algorithm),
                enc_alg,
            ) || enc_hash.tag != 0x04
            {
                return Err(CRYPT_E_ASN1_BADTAG);
            }
            arena.write_blob(
                off + std::mem::offset_of!(CmsgSignerInfo, encrypted_hash),
                enc_hash.val,
            );
            if kids.get(pos + 2).is_some_and(|k| k.tag == 0xA1) {
                if let Some(ua) = der_item_children(kids[pos + 2].val) {
                    decode_attributes_list_into(
                        &mut arena,
                        off + std::mem::offset_of!(CmsgSignerInfo, unauth_attrs_count),
                        &ua,
                    );
                }
            }
            Ok(arena)
        }
        _ => {
            log::debug!("CryptDecodeObjectEx: unknown struct_type={raw_stype:#x} -> ERROR_FILE_NOT_FOUND");
            Err(err::ERROR_FILE_NOT_FOUND)
        }
    }
}

pub(super) unsafe fn crypt_decode_object_ex_impl(
    dw_cert_encoding_type: u32,
    lpsz_struct_type: *const u8,
    pb_encoded: *const u8,
    cb_encoded: u32,
    dw_flags: u32,
    p_decode_para: *const c_void,
    pv_struct_info: *mut c_void,
    pcb_struct_info: *mut u32,
) -> BOOL {
    if (dw_cert_encoding_type & (X509_ASN_ENCODING | PKCS_7_ASN_ENCODING)) == 0
        || pb_encoded.is_null()
        || cb_encoded == 0
        || pcb_struct_info.is_null()
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let der = std::slice::from_raw_parts(pb_encoded, cb_encoded as usize);
    let arena = match decode_x509_object(lpsz_struct_type, der, dw_flags) {
        Ok(a) => a,
        Err(code) => {
            crate::teb::set_last_error(code);
            return FALSE;
        }
    };
    arena.finish(dw_flags, p_decode_para, pv_struct_info, pcb_struct_info)
}

pub(super) unsafe fn crypt_decode_object_impl(
    dw_cert_encoding_type: u32,
    lpsz_struct_type: *const u8,
    pb_encoded: *const u8,
    cb_encoded: u32,
    dw_flags: u32,
    pv_struct_info: *mut c_void,
    pcb_struct_info: *mut u32,
) -> BOOL {
    crypt_decode_object_ex_impl(
        dw_cert_encoding_type,
        lpsz_struct_type,
        pb_encoded,
        cb_encoded,
        dw_flags & !CRYPT_DECODE_ALLOC_FLAG,
        std::ptr::null(),
        pv_struct_info,
        pcb_struct_info,
    )
}

/// Decode a DER `CertificateTrustList` into a single-buffer `LocalAlloc`-allocated Win64 `CTL_INFO`
/// (128B: `dwVersion @ 0`, `SubjectUsage @ 8`, `ListIdentifier @ 24`, `SequenceNumber @ 40`,
/// `ThisUpdate @ 56`, `NextUpdate @ 64`, `SubjectAlgorithm @ 72`, `cCTLEntry @ 96`,
/// `rgCTLEntry @ 104`, `cExtension @ 112`, `rgExtension @ 120`).
pub(super) unsafe fn decode_ctl_info_alloc(ctl_der: &[u8]) -> Option<*mut c_void> {
    let (root, _) = parse_der_item(ctl_der)?;
    if root.tag != 0x30 {
        return None;
    }
    let kids = der_item_children(root.val)?;
    if kids.len() < 3 {
        return None;
    }
    let mut arena = DecodeArena::new();
    let off = arena.alloc_aligned(128, 8);
    let mut idx = 0usize;
    let mut ver = 0u32;
    if kids.get(idx).is_some_and(|k| k.tag == 0x02) {
        ver = decode_der_u32(kids[idx].val).unwrap_or(0);
        idx += 1;
    }
    arena.write_u32(off, ver);

    // SubjectUsage ::= SEQUENCE OF OID -> CTL_USAGE (16B @ +8: cUsageIdentifier u32, rgpszUsageIdentifier *mut *mut u8)
    let usage_seq = *kids.get(idx)?;
    if usage_seq.tag != 0x30 {
        return None;
    }
    let usage_oids = der_item_children(usage_seq.val)?;
    arena.write_u32(off + 8, usage_oids.len() as u32);
    if !usage_oids.is_empty() {
        let arr = arena.alloc_aligned(usage_oids.len() * 8, 8);
        arena.write_ptr(off + 16, Some(arr));
        for (i, o) in usage_oids.iter().enumerate() {
            if o.tag != 0x06 {
                return None;
            }
            let s = wintrust::decode_oid_str(o.val);
            let coff = arena.push_cstr(&s);
            arena.write_ptr(arr + i * 8, Some(coff));
        }
    }
    idx += 1;

    // Optional listIdentifier OCTET STRING -> +24
    if kids.get(idx).is_some_and(|k| k.tag == 0x04) {
        arena.write_blob(off + 24, kids[idx].val);
        idx += 1;
    }
    // Optional sequenceNumber INTEGER (LE) -> +40
    if kids.get(idx).is_some_and(|k| k.tag == 0x02) {
        let mut seq_le = kids[idx].val.to_vec();
        seq_le.reverse();
        arena.write_blob(off + 40, &seq_le);
        idx += 1;
    }
    // ctlThisUpdate Time -> +56
    let this_upd = *kids.get(idx)?;
    if this_upd.tag != 0x17 && this_upd.tag != 0x18 {
        return None;
    }
    arena.write_u64(
        off + 56,
        wintrust::parse_asn1_time_filetime_raw(this_upd.tag, this_upd.val),
    );
    idx += 1;
    // Optional ctlNextUpdate Time -> +64
    if kids.get(idx).is_some_and(|k| k.tag == 0x17 || k.tag == 0x18) {
        arena.write_u64(
            off + 64,
            wintrust::parse_asn1_time_filetime_raw(kids[idx].tag, kids[idx].val),
        );
        idx += 1;
    }
    // subjectAlgorithm AlgorithmIdentifier -> +72 (24B)
    let subj_alg = *kids.get(idx)?;
    if !arena.write_alg_id(off + 72, subj_alg) {
        return None;
    }
    idx += 1;
    // Optional trustedSubjects SEQUENCE OF TrustedSubject -> cCTLEntry @ +96, rgCTLEntry @ +104
    if kids.get(idx).is_some_and(|k| k.tag == 0x30) {
        let subjects = der_item_children(kids[idx].val)?;
        arena.write_u32(off + 96, subjects.len() as u32);
        if !subjects.is_empty() {
            // CTL_ENTRY (32B): SubjectIdentifier (16B @ 0), cAttribute (u32 @ 16), rgAttribute (*mut CRYPT_ATTRIBUTE @ 24)
            let earr = arena.alloc_aligned(subjects.len() * 32, 8);
            arena.write_ptr(off + 104, Some(earr));
            for (i, subj) in subjects.iter().enumerate() {
                if subj.tag != 0x30 {
                    return None;
                }
                let sk = der_item_children(subj.val)?;
                if sk.is_empty() || sk[0].tag != 0x04 {
                    return None;
                }
                let eo = earr + i * 32;
                arena.write_blob(eo, sk[0].val);
                if let Some(attrs) = sk.get(1).filter(|e| e.tag == 0x31) {
                    if let Some(al) = der_item_children(attrs.val) {
                        decode_attributes_list_into(&mut arena, eo + 16, &al);
                    }
                }
            }
        }
        idx += 1;
    }
    // Optional ctlExtensions [0] EXPLICIT Extensions -> cExtension @ +112, rgExtension @ +120
    if kids.get(idx).is_some_and(|k| k.tag == 0xA0) {
        if let Some((ext_seq, _)) = parse_der_item(kids[idx].val) {
            if ext_seq.tag == 0x30 {
                if let Some(ext_list) = der_item_children(ext_seq.val) {
                    decode_extensions_into(&mut arena, off + 112, &ext_list);
                }
            }
        }
    }

    let mem = crate::kernel32::memory::LocalAlloc(0, arena.buf.len().max(1)) as *mut u8;
    if mem.is_null() {
        return None;
    }
    arena.copy_relocated_into(mem);
    Some(mem as *mut c_void)
}

// ---------------------------------------------------------------------------
// SPKI -> Win32 CAPI PUBLICKEYBLOB helper (ready for CryptImportPublicKeyInfo)
// ---------------------------------------------------------------------------

/// Decode a Windows `CERT_PUBLIC_KEY_INFO` (RSA) into a CAPI `PUBLICKEYBLOB`
/// (`PUBLICKEYSTRUC` + `RSAPUBKEY` + little-endian modulus), ready to pass to
/// `advapi32::CryptImportKey`.
#[allow(dead_code)]
pub(crate) unsafe fn spki_to_capi_public_key_blob(p_info: *const c_void) -> Result<Vec<u8>, u32> {
    if p_info.is_null() {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    let spki = std::ptr::read_unaligned(p_info as *const CertPublicKeyInfo);
    if spki.algorithm.psz_obj_id.is_null()
        || spki.public_key.pb_data.is_null()
        || spki.public_key.cb_data == 0
    {
        return Err(err::ERROR_INVALID_PARAMETER);
    }
    let oid = crate::strings::from_ansi(spki.algorithm.psz_obj_id);
    if !matches!(
        oid.as_str(),
        "1.2.840.113549.1.1.1" | "1.2.840.113549.1.1.7" | "1.2.840.113549.1.1.10"
    ) {
        return Err(err::ERROR_NOT_SUPPORTED);
    }
    let key_der = std::slice::from_raw_parts(spki.public_key.pb_data, spki.public_key.cb_data as usize);
    let arena = decode_x509_object(RSA_CSP_PUBLICKEYBLOB as *const u8, key_der, 0)?;
    Ok(arena.buf)
}

// ---------------------------------------------------------------------------
// CryptFindOIDInfo (Win64 ABI-exact static CRYPT_OID_INFO catalog)
// ---------------------------------------------------------------------------

pub(super) const CRYPT_OID_INFO_OID_KEY: u32 = 1;
pub(super) const CRYPT_OID_INFO_NAME_KEY: u32 = 2;
pub(super) const CRYPT_OID_INFO_ALGID_KEY: u32 = 3;
pub(super) const CRYPT_OID_INFO_SIGN_KEY: u32 = 4;
pub(super) const CRYPT_OID_INFO_CNG_ALGID_KEY: u32 = 5;
pub(super) const CRYPT_OID_INFO_CNG_SIGN_KEY: u32 = 6;

pub(super) const CRYPT_HASH_ALG_OID_GROUP_ID: u32 = 1;
pub(super) const CRYPT_ENCRYPT_ALG_OID_GROUP_ID: u32 = 2;
pub(super) const CRYPT_PUBKEY_ALG_OID_GROUP_ID: u32 = 3;
pub(super) const CRYPT_SIGN_ALG_OID_GROUP_ID: u32 = 4;
pub(super) const CRYPT_RDN_ATTR_OID_GROUP_ID: u32 = 5;
pub(super) const CRYPT_EXT_OR_ATTR_OID_GROUP_ID: u32 = 6;
pub(super) const CRYPT_ENHKEY_USAGE_OID_GROUP_ID: u32 = 7;
pub(super) const CRYPT_POLICY_OID_GROUP_ID: u32 = 8;
pub(super) const CRYPT_KDF_OID_GROUP_ID: u32 = 10;

const CALG_MD5: u32 = 0x0000_8003;
const CALG_SHA1: u32 = 0x0000_8004;
const CALG_SHA_256: u32 = 0x0000_800C;
const CALG_SHA_384: u32 = 0x0000_800D;
const CALG_SHA_512: u32 = 0x0000_800E;
const CALG_RSA_SIGN: u32 = 0x0000_2400;
const CALG_RSA_KEYX: u32 = 0x0000_A400;
const CALG_DSS_SIGN: u32 = 0x0000_2200;
const CALG_DES: u32 = 0x0000_6601;
const CALG_3DES: u32 = 0x0000_6603;
const CALG_RC2: u32 = 0x0000_6602;
const CALG_RC4: u32 = 0x0000_6801;
const CALG_AES_128: u32 = 0x0000_660E;
const CALG_AES_192: u32 = 0x0000_660F;
const CALG_AES_256: u32 = 0x0000_6610;
const CALG_OID_INFO_CNG_ONLY: u32 = 0xFFFF_FFFF;

#[repr(C)]
pub(super) struct CryptOidInfo {
    pub(super) cb_size: u32,
    pub(super) psz_oid: *const u8,
    pub(super) pwsz_name: *const u16,
    pub(super) dw_group_id: u32,
    pub(super) dw_value: u32,
    pub(super) extra_info: CryptDataBlob,
    pub(super) pwsz_cng_algid: *const u16,
    pub(super) pwsz_cng_extra_algid: *const u16,
}
const _: () = assert!(std::mem::size_of::<CryptOidInfo>() == 64);

struct OwnedOidEntry {
    oid: String,
    name: String,
    cng_alg: String,
    cng_extra: String,
    _oid_cstr: Box<[u8]>,
    _name_wstr: Box<[u16]>,
    _extra_bytes: Box<[u8]>,
    _cng_alg_wstr: Box<[u16]>,
    _cng_extra_wstr: Box<[u16]>,
    info: Box<CryptOidInfo>,
}

unsafe impl Send for OwnedOidEntry {}
unsafe impl Sync for OwnedOidEntry {}

impl OwnedOidEntry {
    fn new(
        oid: &str,
        name: &str,
        group: u32,
        val: u32,
        extra: &[u8],
        cng_alg: &str,
        cng_extra: &str,
    ) -> Self {
        let mut oid_v = oid.as_bytes().to_vec();
        oid_v.push(0);
        let oid_cstr = oid_v.into_boxed_slice();
        let name_wstr: Box<[u16]> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let extra_bytes = extra.to_vec().into_boxed_slice();
        let cng_alg_wstr: Box<[u16]> = if cng_alg.is_empty() {
            Box::new([])
        } else {
            cng_alg.encode_utf16().chain(std::iter::once(0)).collect()
        };
        let cng_extra_wstr: Box<[u16]> = if cng_extra.is_empty() {
            Box::new([])
        } else {
            cng_extra.encode_utf16().chain(std::iter::once(0)).collect()
        };
        let info = Box::new(CryptOidInfo {
            cb_size: std::mem::size_of::<CryptOidInfo>() as u32,
            psz_oid: oid_cstr.as_ptr(),
            pwsz_name: name_wstr.as_ptr(),
            dw_group_id: group,
            dw_value: val,
            extra_info: CryptDataBlob {
                cb_data: extra_bytes.len() as u32,
                pb_data: if extra_bytes.is_empty() {
                    std::ptr::null()
                } else {
                    extra_bytes.as_ptr()
                },
            },
            pwsz_cng_algid: if cng_alg_wstr.is_empty() {
                std::ptr::null()
            } else {
                cng_alg_wstr.as_ptr()
            },
            pwsz_cng_extra_algid: if cng_extra_wstr.is_empty() {
                std::ptr::null()
            } else {
                cng_extra_wstr.as_ptr()
            },
        });
        Self {
            oid: oid.to_string(),
            name: name.to_string(),
            cng_alg: cng_alg.to_string(),
            cng_extra: cng_extra.to_string(),
            _oid_cstr: oid_cstr,
            _name_wstr: name_wstr,
            _extra_bytes: extra_bytes,
            _cng_alg_wstr: cng_alg_wstr,
            _cng_extra_wstr: cng_extra_wstr,
            info,
        }
    }
}

fn oid_table() -> &'static [OwnedOidEntry] {
    static TABLE: OnceLock<Vec<OwnedOidEntry>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let rsa_sign_extra = CALG_RSA_SIGN.to_le_bytes();
        let dss_sign_extra = CALG_DSS_SIGN.to_le_bytes();
        let cng_extra_b = CALG_OID_INFO_CNG_ONLY.to_le_bytes();
        let aes128_extra = 128u32.to_le_bytes();
        let aes192_extra = 192u32.to_le_bytes();
        let aes256_extra = 256u32.to_le_bytes();

        vec![
            // Hash algorithms (group 1)
            OwnedOidEntry::new("1.2.840.113549.2.5", "md5", CRYPT_HASH_ALG_OID_GROUP_ID, CALG_MD5, &[], "MD5", ""),
            OwnedOidEntry::new("1.3.14.3.2.26", "sha1", CRYPT_HASH_ALG_OID_GROUP_ID, CALG_SHA1, &[], "SHA1", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.2.1", "sha256", CRYPT_HASH_ALG_OID_GROUP_ID, CALG_SHA_256, &[], "SHA256", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.2.2", "sha384", CRYPT_HASH_ALG_OID_GROUP_ID, CALG_SHA_384, &[], "SHA384", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.2.3", "sha512", CRYPT_HASH_ALG_OID_GROUP_ID, CALG_SHA_512, &[], "SHA512", ""),
            // Symmetric encryption algorithms (group 2)
            OwnedOidEntry::new("1.3.14.3.2.7", "des", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_DES, &[], "DES", ""),
            OwnedOidEntry::new("1.2.840.113549.3.7", "3des", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_3DES, &[], "3DES", ""),
            OwnedOidEntry::new("1.2.840.113549.3.2", "rc2", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_RC2, &[], "RC2", ""),
            OwnedOidEntry::new("1.2.840.113549.3.4", "rc4", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_RC4, &[], "RC4", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.1.2", "aes128", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_AES_128, &aes128_extra, "AES", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.1.22", "aes192", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_AES_192, &aes192_extra, "AES", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.1.42", "aes256", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_AES_256, &aes256_extra, "AES", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.1.6", "aes128GCM", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &aes128_extra, "AES", ""),
            OwnedOidEntry::new("2.16.840.1.101.3.4.1.46", "aes256GCM", CRYPT_ENCRYPT_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &aes256_extra, "AES", ""),
            // Public key algorithms (group 3)
            OwnedOidEntry::new("1.2.840.113549.1.1.1", "RSA", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_RSA_KEYX, &[], "RSA", ""),
            OwnedOidEntry::new("1.2.840.113549.1.1.1", "RSA_SIGN", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_RSA_SIGN, &[], "RSA", ""),
            OwnedOidEntry::new("1.2.840.10040.4.1", "DSA", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_DSS_SIGN, &[], "DSA", ""),
            OwnedOidEntry::new("1.2.840.10045.2.1", "ECC", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &[], "ECDSA", ""),
            OwnedOidEntry::new("1.2.840.10045.3.1.7", "nistP256", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &[], "ECDSA_P256", ""),
            OwnedOidEntry::new("1.3.132.0.34", "nistP384", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &[], "ECDSA_P384", ""),
            OwnedOidEntry::new("1.3.132.0.35", "nistP521", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &[], "ECDSA_P521", ""),
            OwnedOidEntry::new("1.3.101.112", "Ed25519", CRYPT_PUBKEY_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &[], "ED25519", ""),
            // Signature algorithms (group 4)
            OwnedOidEntry::new("1.2.840.113549.1.1.4", "md5RSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_MD5, &rsa_sign_extra, "MD5", "RSA"),
            OwnedOidEntry::new("1.2.840.113549.1.1.5", "sha1RSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_SHA1, &rsa_sign_extra, "SHA1", "RSA"),
            OwnedOidEntry::new("1.2.840.113549.1.1.11", "sha256RSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_SHA_256, &rsa_sign_extra, "SHA256", "RSA"),
            OwnedOidEntry::new("1.2.840.113549.1.1.12", "sha384RSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_SHA_384, &rsa_sign_extra, "SHA384", "RSA"),
            OwnedOidEntry::new("1.2.840.113549.1.1.13", "sha512RSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_SHA_512, &rsa_sign_extra, "SHA512", "RSA"),
            OwnedOidEntry::new("1.2.840.113549.1.1.10", "RSASSA-PSS", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &cng_extra_b, "RSA", ""),
            OwnedOidEntry::new("1.2.840.10040.4.3", "sha1DSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_SHA1, &dss_sign_extra, "SHA1", "DSA"),
            OwnedOidEntry::new("1.2.840.10045.4.1", "sha1ECDSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &cng_extra_b, "SHA1", "ECDSA"),
            OwnedOidEntry::new("1.2.840.10045.4.3.2", "sha256ECDSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &cng_extra_b, "SHA256", "ECDSA"),
            OwnedOidEntry::new("1.2.840.10045.4.3.3", "sha384ECDSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &cng_extra_b, "SHA384", "ECDSA"),
            OwnedOidEntry::new("1.2.840.10045.4.3.4", "sha512ECDSA", CRYPT_SIGN_ALG_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &cng_extra_b, "SHA512", "ECDSA"),
            // RDN attributes (group 5)
            OwnedOidEntry::new("2.5.4.3", "CN", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.4", "SN", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.5", "SERIALNUMBER", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.6", "C", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.7", "L", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.8", "S", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.9", "STREET", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.10", "O", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.11", "OU", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.12", "T", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.4.42", "G", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("1.2.840.113549.1.9.1", "E", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("0.9.2342.19200300.100.1.25", "DC", CRYPT_RDN_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            // Extensions & Attributes (group 6)
            OwnedOidEntry::new("2.5.29.14", "Subject Key Identifier", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.15", "Key Usage", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.17", "Subject Alternative Name", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.18", "Issuer Alternative Name", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.19", "Basic Constraints", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.31", "CRL Distribution Points", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.32", "Certificate Policies", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.35", "Authority Key Identifier", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("2.5.29.37", "Enhanced Key Usage", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("1.3.6.1.5.5.7.1.1", "Authority Information Access", CRYPT_EXT_OR_ATTR_OID_GROUP_ID, 0, &[], "", ""),
            // Enhanced Key Usage (group 7)
            OwnedOidEntry::new("1.3.6.1.5.5.7.3.1", "Server Authentication", CRYPT_ENHKEY_USAGE_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("1.3.6.1.5.5.7.3.2", "Client Authentication", CRYPT_ENHKEY_USAGE_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("1.3.6.1.5.5.7.3.3", "Code Signing", CRYPT_ENHKEY_USAGE_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("1.3.6.1.5.5.7.3.4", "Secure Email", CRYPT_ENHKEY_USAGE_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("1.3.6.1.5.5.7.3.8", "Time Stamping", CRYPT_ENHKEY_USAGE_OID_GROUP_ID, 0, &[], "", ""),
            OwnedOidEntry::new("1.3.6.1.5.5.7.3.9", "OCSP Signing", CRYPT_ENHKEY_USAGE_OID_GROUP_ID, 0, &[], "", ""),
            // Policy (group 8)
            OwnedOidEntry::new("2.5.29.32.0", "All issuance policies", CRYPT_POLICY_OID_GROUP_ID, 0, &[], "", ""),
            // KDF (group 10)
            OwnedOidEntry::new("1.2.840.113549.1.5.12", "PBKDF2", CRYPT_KDF_OID_GROUP_ID, CALG_OID_INFO_CNG_ONLY, &[], "PBKDF2", ""),
        ]
    })
}

pub(super) unsafe fn crypt_find_oid_info_impl(
    dw_key_type: u32,
    pv_key: *const c_void,
    dw_group_id: u32,
) -> *mut c_void {
    if pv_key.is_null() {
        return std::ptr::null_mut();
    }
    const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK: u32 = 0x0FFF_0000;
    const CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT: u32 = 16;

    let key_type = dw_key_type & 0x0000_FFFF;
    let group_filter = dw_group_id & 0x0000_FFFF;
    let req_bit_len =
        (dw_group_id & CRYPT_OID_INFO_OID_GROUP_BIT_LEN_MASK) >> CRYPT_OID_INFO_OID_GROUP_BIT_LEN_SHIFT;
    let table = oid_table();

    let matches_group = |e: &OwnedOidEntry| -> bool {
        if group_filter != 0 && e.info.dw_group_id != group_filter {
            return false;
        }
        if req_bit_len != 0 {
            if e._extra_bytes.len() < 4 {
                return false;
            }
            let entry_bits = u32::from_le_bytes(e._extra_bytes[..4].try_into().unwrap());
            if entry_bits != req_bit_len {
                return false;
            }
        }
        true
    };

    let found = match key_type {
        CRYPT_OID_INFO_OID_KEY => {
            let want = crate::strings::from_ansi(pv_key as *const u8);
            table
                .iter()
                .find(|e| matches_group(e) && e.oid == want)
        }
        CRYPT_OID_INFO_NAME_KEY => {
            let want = crate::strings::from_wide(pv_key as *const u16);
            table
                .iter()
                .find(|e| matches_group(e) && e.name.eq_ignore_ascii_case(&want))
        }
        CRYPT_OID_INFO_ALGID_KEY => {
            let want = std::ptr::read_unaligned(pv_key as *const u32);
            table
                .iter()
                .find(|e| matches_group(e) && e.info.dw_value == want)
        }
        CRYPT_OID_INFO_SIGN_KEY => {
            let pair = std::ptr::read_unaligned(pv_key as *const [u32; 2]);
            let want_hash = pair[0];
            let want_pub = pair[1];
            table.iter().find(|e| {
                matches_group(e)
                    && e.info.dw_group_id == CRYPT_SIGN_ALG_OID_GROUP_ID
                    && e.info.dw_value == want_hash
                    && e._extra_bytes.len() >= 4
                    && u32::from_le_bytes(e._extra_bytes[..4].try_into().unwrap()) == want_pub
            })
        }
        CRYPT_OID_INFO_CNG_ALGID_KEY => {
            let want = crate::strings::from_wide(pv_key as *const u16);
            table
                .iter()
                .find(|e| matches_group(e) && !e.cng_alg.is_empty() && e.cng_alg.eq_ignore_ascii_case(&want))
        }
        CRYPT_OID_INFO_CNG_SIGN_KEY => {
            let pair = std::ptr::read_unaligned(pv_key as *const [*const u16; 2]);
            if pair[0].is_null() || pair[1].is_null() {
                None
            } else {
                let want_hash = crate::strings::from_wide(pair[0]);
                let want_pub = crate::strings::from_wide(pair[1]);
                table.iter().find(|e| {
                    matches_group(e)
                        && e.info.dw_group_id == CRYPT_SIGN_ALG_OID_GROUP_ID
                        && e.cng_alg.eq_ignore_ascii_case(&want_hash)
                        && e.cng_extra.eq_ignore_ascii_case(&want_pub)
                })
            }
        }
        _ => None,
    };

    match found {
        Some(e) => &*e.info as *const CryptOidInfo as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

// ---------------------------------------------------------------------------
// CertStrToNameA / CertStrToNameW (RFC 1779 / 2253 X.500 DN -> DER)
// ---------------------------------------------------------------------------

const CERT_NAME_STR_REVERSE_FLAG: u32 = 0x0200_0000;
const CERT_NAME_STR_CRLF_FLAG: u32 = 0x0800_0000;
const CERT_NAME_STR_NO_QUOTING_FLAG: u32 = 0x1000_0000;
const CERT_NAME_STR_NO_PLUS_FLAG: u32 = 0x2000_0000;
const CERT_NAME_STR_SEMICOLON_FLAG: u32 = 0x4000_0000;

fn rdn_key_to_oid(key: &str) -> Option<&'static str> {
    match key.trim().to_ascii_uppercase().as_str() {
        "CN" => Some("2.5.4.3"),
        "SN" | "SURNAME" => Some("2.5.4.4"),
        "SERIALNUMBER" => Some("2.5.4.5"),
        "C" | "COUNTRY" => Some("2.5.4.6"),
        "L" | "LOCALITY" => Some("2.5.4.7"),
        "S" | "ST" | "STATEORPROVINCENAME" => Some("2.5.4.8"),
        "STREET" => Some("2.5.4.9"),
        "O" | "ORG" | "ORGANIZATION" => Some("2.5.4.10"),
        "OU" | "ORGUNIT" | "ORGANIZATIONALUNIT" => Some("2.5.4.11"),
        "T" | "TITLE" => Some("2.5.4.12"),
        "DESCRIPTION" => Some("2.5.4.13"),
        "G" | "GN" | "GIVENNAME" => Some("2.5.4.42"),
        "I" | "INITIALS" => Some("2.5.4.43"),
        "E" | "EMAIL" | "EMAILADDRESS" => Some("1.2.840.113549.1.9.1"),
        "DC" | "DOMAINCOMPONENT" => Some("0.9.2342.19200300.100.1.25"),
        "UID" => Some("0.9.2342.19200300.100.1.1"),
        _ => None,
    }
}

fn is_printable_string(s: &str) -> bool {
    s.bytes().all(|b| {
        b.is_ascii_alphanumeric()
            || matches!(
                b,
                b' ' | b'\'' | b'(' | b')' | b'+' | b',' | b'-' | b'.' | b'/' | b':' | b'=' | b'?'
            )
    })
}

fn encode_dn_attr_value(oid_str: &str, value: &str, is_hex_der: bool) -> Option<Vec<u8>> {
    if is_hex_der {
        if value.len() % 2 != 0 || value.is_empty() {
            return None;
        }
        let mut raw = Vec::with_capacity(value.len() / 2);
        let bytes = value.as_bytes();
        for pair in bytes.chunks_exact(2) {
            let hi = (pair[0] as char).to_digit(16)? as u8;
            let lo = (pair[1] as char).to_digit(16)? as u8;
            raw.push((hi << 4) | lo);
        }
        return Some(raw);
    }
    // Country (C) must be PrintableString; Email (E) and DC use IA5String when ASCII.
    let tag = if oid_str == "2.5.4.6" {
        0x13u8
    } else if (oid_str == "1.2.840.113549.1.9.1" || oid_str == "0.9.2342.19200300.100.1.25")
        && value.is_ascii()
    {
        0x16u8
    } else if is_printable_string(value) {
        0x13u8
    } else {
        0x0Cu8
    };
    Some(encode_der_tlv(tag, value.as_bytes()))
}

/// Parse an X.500 string into DER `Name` bytes. On syntax error returns `Err(char_index)`.
fn parse_x500_name_to_der(input: &str, dw_str_type: u32) -> Result<Vec<u8>, usize> {
    let allow_semicolon = (dw_str_type & CERT_NAME_STR_SEMICOLON_FLAG) != 0
        || (dw_str_type & 0xFFFF) == 3; // X500 accepts ';' in Windows as well as ','
    let allow_crlf = (dw_str_type & CERT_NAME_STR_CRLF_FLAG) != 0;
    let no_plus = (dw_str_type & CERT_NAME_STR_NO_PLUS_FLAG) != 0;
    let no_quoting = (dw_str_type & CERT_NAME_STR_NO_QUOTING_FLAG) != 0;
    let reverse = (dw_str_type & CERT_NAME_STR_REVERSE_FLAG) != 0;

    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let n = chars.len();
    let mut pos = 0usize;

    let skip_ws = |p: &mut usize| {
        while *p < n && matches!(chars[*p].1, ' ' | '\t') {
            *p += 1;
        }
    };
    let is_rdn_sep = |c: char| -> bool {
        c == ',' || (allow_semicolon && c == ';') || (allow_crlf && (c == '\r' || c == '\n'))
    };

    let mut rdns: Vec<Vec<(Vec<u8>, Vec<u8>)>> = Vec::new();
    skip_ws(&mut pos);
    while pos < n {
        if is_rdn_sep(chars[pos].1) {
            pos += 1;
            skip_ws(&mut pos);
            continue;
        }
        let mut rdn_attrs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        loop {
            skip_ws(&mut pos);
            let key_start_idx = pos;
            if pos >= n {
                return Err(key_start_idx);
            }
            // Read key up to '='
            let mut key = String::new();
            while pos < n && chars[pos].1 != '=' {
                if is_rdn_sep(chars[pos].1) || (!no_plus && chars[pos].1 == '+') {
                    return Err(key_start_idx);
                }
                key.push(chars[pos].1);
                pos += 1;
            }
            if pos >= n || chars[pos].1 != '=' {
                return Err(key_start_idx);
            }
            pos += 1; // consume '='
            let key_trimmed = key.trim();
            let oid_owned: String = if let Some(known) = rdn_key_to_oid(key_trimmed) {
                known.to_string()
            } else {
                let stripped = key_trimmed
                    .strip_prefix("OID.")
                    .or_else(|| key_trimmed.strip_prefix("oid."))
                    .unwrap_or(key_trimmed);
                if wintrust::encode_oid_str(stripped).is_none() {
                    return Err(key_start_idx);
                }
                stripped.to_string()
            };
            let oid_bytes = wintrust::encode_oid_str(&oid_owned).ok_or(key_start_idx)?;

            skip_ws(&mut pos);
            let val_start_idx = pos;
            let mut val = String::new();
            let mut is_hex_der = false;
            if !no_quoting && pos < n && chars[pos].1 == '"' {
                pos += 1; // opening quote
                let mut closed = false;
                while pos < n {
                    let c = chars[pos].1;
                    if c == '"' {
                        if pos + 1 < n && chars[pos + 1].1 == '"' {
                            val.push('"');
                            pos += 2;
                        } else {
                            pos += 1;
                            closed = true;
                            break;
                        }
                    } else if c == '\\' && pos + 1 < n {
                        val.push(chars[pos + 1].1);
                        pos += 2;
                    } else {
                        val.push(c);
                        pos += 1;
                    }
                }
                if !closed {
                    return Err(val_start_idx);
                }
                skip_ws(&mut pos);
            } else if pos < n && chars[pos].1 == '#' {
                is_hex_der = true;
                pos += 1;
                while pos < n && !is_rdn_sep(chars[pos].1) && (no_plus || chars[pos].1 != '+') {
                    let c = chars[pos].1;
                    if !c.is_ascii_whitespace() {
                        val.push(c);
                    }
                    pos += 1;
                }
            } else {
                while pos < n && !is_rdn_sep(chars[pos].1) && (no_plus || chars[pos].1 != '+') {
                    let c = chars[pos].1;
                    if !no_quoting && c == '\\' && pos + 1 < n {
                        val.push(chars[pos + 1].1);
                        pos += 2;
                    } else {
                        val.push(c);
                        pos += 1;
                    }
                }
                let trimmed_len = val.trim_end_matches([' ', '\t']).len();
                val.truncate(trimmed_len);
            }

            let val_der = encode_dn_attr_value(&oid_owned, &val, is_hex_der).ok_or(val_start_idx)?;
            rdn_attrs.push((oid_bytes, val_der));

            if pos < n && !no_plus && chars[pos].1 == '+' {
                pos += 1;
                continue;
            }
            if pos < n && is_rdn_sep(chars[pos].1) {
                pos += 1;
            }
            break;
        }
        if !rdn_attrs.is_empty() {
            rdns.push(rdn_attrs);
        }
        skip_ws(&mut pos);
    }

    if reverse {
        rdns.reverse();
    }

    let mut seq_inner = Vec::new();
    for rdn in rdns {
        let mut set_inner = Vec::new();
        for (oid_bytes, val_der) in rdn {
            let mut atv = encode_der_tlv(0x06, &oid_bytes);
            atv.extend_from_slice(&val_der);
            set_inner.extend_from_slice(&encode_der_tlv(0x30, &atv));
        }
        seq_inner.extend_from_slice(&encode_der_tlv(0x31, &set_inner));
    }
    Ok(encode_der_tlv(0x30, &seq_inner))
}

pub(super) unsafe fn cert_str_to_name_a_impl(
    dw_cert_encoding_type: u32,
    psz_x500: LPCSTR,
    dw_str_type: u32,
    _pv_reserved: *mut c_void,
    pb_encoded: *mut u8,
    pcb_encoded: *mut u32,
    ppsz_error: *mut LPCSTR,
) -> BOOL {
    if !ppsz_error.is_null() {
        std::ptr::write_unaligned(ppsz_error, std::ptr::null());
    }
    if (dw_cert_encoding_type & X509_ASN_ENCODING) == 0
        || psz_x500.is_null()
        || pcb_encoded.is_null()
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let s = crate::strings::from_ansi(psz_x500);
    let der = match parse_x500_name_to_der(&s, dw_str_type) {
        Ok(d) => d,
        Err(char_idx) => {
            if !ppsz_error.is_null() {
                let byte_off = s.char_indices().nth(char_idx).map(|(b, _)| b).unwrap_or(s.len());
                std::ptr::write_unaligned(ppsz_error, psz_x500.add(byte_off));
            }
            crate::teb::set_last_error(CRYPT_E_INVALID_X500_STRING);
            return FALSE;
        }
    };
    let need = der.len() as u32;
    if pb_encoded.is_null() {
        std::ptr::write_unaligned(pcb_encoded, need);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pcb_encoded);
    if cap < need {
        std::ptr::write_unaligned(pcb_encoded, need);
        crate::teb::set_last_error(err::ERROR_MORE_DATA);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(der.as_ptr(), pb_encoded, der.len());
    std::ptr::write_unaligned(pcb_encoded, need);
    TRUE
}

pub(super) unsafe fn cert_str_to_name_w_impl(
    dw_cert_encoding_type: u32,
    psz_x500: LPCWSTR,
    dw_str_type: u32,
    _pv_reserved: *mut c_void,
    pb_encoded: *mut u8,
    pcb_encoded: *mut u32,
    ppsz_error: *mut LPCWSTR,
) -> BOOL {
    if !ppsz_error.is_null() {
        std::ptr::write_unaligned(ppsz_error, std::ptr::null());
    }
    if (dw_cert_encoding_type & X509_ASN_ENCODING) == 0
        || psz_x500.is_null()
        || pcb_encoded.is_null()
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let s = crate::strings::from_wide(psz_x500);
    let der = match parse_x500_name_to_der(&s, dw_str_type) {
        Ok(d) => d,
        Err(char_idx) => {
            if !ppsz_error.is_null() {
                let utf16_off: usize = s.chars().take(char_idx).map(|c| c.len_utf16()).sum();
                std::ptr::write_unaligned(ppsz_error, psz_x500.add(utf16_off));
            }
            crate::teb::set_last_error(CRYPT_E_INVALID_X500_STRING);
            return FALSE;
        }
    };
    let need = der.len() as u32;
    if pb_encoded.is_null() {
        std::ptr::write_unaligned(pcb_encoded, need);
        return TRUE;
    }
    let cap = std::ptr::read_unaligned(pcb_encoded);
    if cap < need {
        std::ptr::write_unaligned(pcb_encoded, need);
        crate::teb::set_last_error(err::ERROR_MORE_DATA);
        return FALSE;
    }
    std::ptr::copy_nonoverlapping(der.as_ptr(), pb_encoded, der.len());
    std::ptr::write_unaligned(pcb_encoded, need);
    TRUE
}
