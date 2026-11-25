#[path = "test_vectors.rs"]
pub(crate) mod test_vectors;

use super::asn1::{
    encode_der_tlv, CertAltNameEntry, CertAltNameInfo, CertBasicConstraints2Info, CertEccSignature,
    CertEnhkeyUsage, CertNameInfo, CertRdn, CertRdnAttr, CRYPT_STRING_BASE64,
    CRYPT_STRING_BASE64HEADER, CRYPT_STRING_BASE64REQUESTHEADER, CRYPT_STRING_BASE64URI,
    CRYPT_STRING_BASE64X509CRLHEADER, CRYPT_STRING_BASE64_ANY, CRYPT_STRING_HEX,
    CRYPT_STRING_HEXADDR, CRYPT_STRING_HEXASCII, CRYPT_STRING_HEXASCIIADDR, CRYPT_STRING_HEXRAW,
    CRYPT_STRING_HEX_ANY, PKCS_7_ASN_ENCODING, X509_ALTERNATE_NAME, X509_ASN_ENCODING,
    X509_BASIC_CONSTRAINTS2, X509_ECC_SIGNATURE, X509_ENHANCED_KEY_USAGE, X509_NAME,
};
use super::chain::{
    CertChainPolicyPara, CertChainPolicyStatus, SslExtraCertChainPolicyPara,
    CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG, CERT_CHAIN_POLICY_AUTHENTICODE,
    CERT_CHAIN_POLICY_BASE, CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG, CERT_CHAIN_POLICY_SSL,
};
use super::store::{
    CERT_FIND_ANY, CERT_FIND_EXISTING, CERT_FIND_ISSUER_STR_A, CERT_FIND_ISSUER_STR_W,
    CERT_FIND_MD5_HASH, CERT_FIND_SHA1_HASH, CERT_FIND_SUBJECT_CERT, CERT_FIND_SUBJECT_STR_A,
    CERT_FIND_SUBJECT_STR_W, CERT_QUERY_CONTENT_PKCS7_SIGNED_EMBED, CERT_QUERY_FORMAT_BINARY,
    CERT_QUERY_FORMAT_FLAG_BINARY, CERT_QUERY_OBJECT_BLOB, CERT_STORE_ADD_NEW,
    CERT_STORE_PROV_SYSTEM_A, CERT_STORE_PROV_SYSTEM_REGISTRY_W, CERT_STORE_PROV_SYSTEM_W,
    CERT_SYSTEM_STORE_CURRENT_USER, CMSG_SIGNER_COUNT_PARAM, CMSG_SIGNER_INFO_PARAM,
};
use super::*;
use crate::misc::wintrust::{self, CertContext, CertInfo, CmsgSignerInfo, CryptDataBlob};
use crate::net_tls::{
    SECURITY_FLAG_IGNORE_CERT_DATE_INVALID, SECURITY_FLAG_IGNORE_UNKNOWN_CA,
};

#[test]
fn crypt_string_to_binary_base64_and_pem() {
    unsafe {
        let b64 = b"SGVsbG8gV29ybGQ=\0";
        let mut len = 0u32;
        assert_eq!(
            CryptStringToBinaryA(
                b64.as_ptr(),
                0,
                CRYPT_STRING_BASE64,
                std::ptr::null_mut(),
                &mut len,
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            TRUE
        );
        assert_eq!(len, 11);
        let mut out = vec![0u8; len as usize];
        assert_eq!(
            CryptStringToBinaryA(
                b64.as_ptr(),
                0,
                CRYPT_STRING_BASE64,
                out.as_mut_ptr(),
                &mut len,
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            TRUE
        );
        assert_eq!(&out, b"Hello World");

        let pem = b"-----BEGIN CERTIFICATE-----\nSGVsbG8=\n-----END CERTIFICATE-----\0";
        let mut plen = 0u32;
        let mut pflags = 0u32;
        assert_eq!(
            CryptStringToBinaryA(
                pem.as_ptr(),
                0,
                CRYPT_STRING_BASE64_ANY,
                std::ptr::null_mut(),
                &mut plen,
                std::ptr::null_mut(),
                &mut pflags
            ),
            TRUE
        );
        assert_eq!(plen, 5);
        assert_eq!(pflags, CRYPT_STRING_BASE64HEADER);
        let mut pout = vec![0u8; plen as usize];
        assert_eq!(
            CryptStringToBinaryA(
                pem.as_ptr(),
                0,
                CRYPT_STRING_BASE64_ANY,
                pout.as_mut_ptr(),
                &mut plen,
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            TRUE
        );
        assert_eq!(&pout, b"Hello");
    }
}

#[test]
fn abi_layouts_match_win64() {
    assert_eq!(std::mem::size_of::<CertChainPolicyPara>(), 16);
    assert_eq!(
        std::mem::offset_of!(CertChainPolicyPara, pv_extra_policy_para),
        8
    );
    assert_eq!(std::mem::size_of::<CertChainPolicyStatus>(), 24);
    assert_eq!(std::mem::offset_of!(CertChainPolicyStatus, dw_error), 4);
    assert_eq!(std::mem::size_of::<SslExtraCertChainPolicyPara>(), 24);
    assert_eq!(
        std::mem::offset_of!(SslExtraCertChainPolicyPara, fdw_checks),
        8
    );
    assert_eq!(
        std::mem::offset_of!(SslExtraCertChainPolicyPara, pwsz_server_name),
        16
    );
    assert_eq!(std::mem::size_of::<CmsgSignerInfo>(), 0x88);
}

#[test]
fn crypt_query_object_and_signer_lookup() {
    fn der(tag: u8, val: &[u8]) -> Vec<u8> {
        encode_der_tlv(tag, val)
    }
    // Build a synthetic X.509 cert (CN="Test Publisher", serial=0x01020304)
    let cn_atv = der(
        0x30,
        &[der(0x06, &[0x55, 0x04, 0x03]), der(0x0C, b"Test Publisher")].concat(),
    );
    let dn = der(0x30, &der(0x31, &cn_atv));
    let alg_sha256_rsa = der(
        0x30,
        &[
            der(
                0x06,
                &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B],
            ),
            der(0x05, &[]),
        ]
        .concat(),
    );
    let validity = der(
        0x30,
        &[der(0x17, b"250101000000Z"), der(0x17, b"350101000000Z")].concat(),
    );
    let spki = der(
        0x30,
        &[
            der(
                0x30,
                &[
                    der(
                        0x06,
                        &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01],
                    ),
                    der(0x05, &[]),
                ]
                .concat(),
            ),
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
    let cert_der = der(
        0x30,
        &[tbs, alg_sha256_rsa, der(0x03, &[0x00, 0xAA, 0xBB])].concat(),
    );

    // Build PKCS#7 SignedData ContentInfo containing cert_der + SignerInfo
    let sha256_alg = der(
        0x30,
        &der(
            0x06,
            &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01],
        ),
    );
    let rsa_alg = der(
        0x30,
        &der(
            0x06,
            &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01],
        ),
    );
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
            der(
                0x30,
                &der(
                    0x06,
                    &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x01],
                ),
            ),
            der(0xA0, &cert_der),
            der(0x31, &signer_info),
        ]
        .concat(),
    );
    let pkcs7 = der(
        0x30,
        &[
            der(
                0x06,
                &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02],
            ),
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
            CryptMsgGetParam(
                h_msg,
                CMSG_SIGNER_INFO_PARAM,
                0,
                std::ptr::null_mut(),
                &mut si_len
            ),
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
        assert_eq!(
            crate::strings::from_ansi(si.hash_algorithm.psz_obj_id),
            "2.16.840.1.101.3.4.2.1"
        );

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
        let unsigned = CryptDataBlob {
            cb_data: 16,
            pb_data: b"unsigned payload".as_ptr(),
        };
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

fn build_synthetic_cert(
    cn: &str,
    issuer_cn: &str,
    not_before: &[u8],
    not_after: &[u8],
    san_dns: Option<&str>,
    eku_oids: &[&[u8]],
) -> Vec<u8> {
    let mk_dn = |s: &str| {
        let atv = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x55, 0x04, 0x03]),
                encode_der_tlv(0x0C, s.as_bytes()),
            ]
            .concat(),
        );
        encode_der_tlv(0x30, &encode_der_tlv(0x31, &atv))
    };
    let alg = encode_der_tlv(
        0x30,
        &[
            encode_der_tlv(
                0x06,
                &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B],
            ),
            encode_der_tlv(0x05, &[]),
        ]
        .concat(),
    );
    let validity = encode_der_tlv(
        0x30,
        &[
            encode_der_tlv(0x17, not_before),
            encode_der_tlv(0x17, not_after),
        ]
        .concat(),
    );
    let spki = encode_der_tlv(
        0x30,
        &[
            encode_der_tlv(
                0x30,
                &[
                    encode_der_tlv(
                        0x06,
                        &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01],
                    ),
                    encode_der_tlv(0x05, &[]),
                ]
                .concat(),
            ),
            encode_der_tlv(
                0x03,
                &[0x00, 0x30, 0x06, 0x02, 0x01, 0x05, 0x02, 0x01, 0x03],
            ),
        ]
        .concat(),
    );
    let mut exts = Vec::new();
    if let Some(dns) = san_dns {
        let san_val = encode_der_tlv(0x30, &encode_der_tlv(0x82, dns.as_bytes()));
        let ext = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x55, 0x1D, 0x11]),
                encode_der_tlv(0x04, &san_val),
            ]
            .concat(),
        );
        exts.extend_from_slice(&ext);
    }
    if !eku_oids.is_empty() {
        let mut inner = Vec::new();
        for oid in eku_oids {
            inner.extend_from_slice(&encode_der_tlv(0x06, oid));
        }
        let eku_val = encode_der_tlv(0x30, &inner);
        let ext = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x55, 0x1D, 0x25]),
                encode_der_tlv(0x04, &eku_val),
            ]
            .concat(),
        );
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
    encode_der_tlv(
        0x30,
        &[tbs, alg, encode_der_tlv(0x03, &[0x00, 0x11, 0x22, 0x33])].concat(),
    )
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
                first_der =
                    std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize)
                        .to_vec();
            }
            count += 1;
        }
        // Verify ROOT includes at least all embedded webpki-roots (plus macOS system roots on macOS)
        assert!(count >= webpki_roots::TLS_SERVER_ROOTS.len());
        assert!(count > 100);

        // CERT_FIND_ANY
        let any_c = CertFindCertificateInStore(
            root_a,
            X509_ASN_ENCODING,
            0,
            CERT_FIND_ANY,
            std::ptr::null(),
            std::ptr::null_mut(),
        );
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
        let sha1_blob = CryptDataBlob {
            cb_data: sha1.len() as u32,
            pb_data: sha1.as_ptr(),
        };
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
        let md5_blob = CryptDataBlob {
            cb_data: md5.len() as u32,
            pb_data: md5.as_ptr(),
        };
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
        assert_eq!(
            CertAddCertificateContextToStore(
                my_store,
                by_sha1,
                CERT_STORE_ADD_NEW,
                &mut stored_ctx
            ),
            TRUE
        );
        assert!(!stored_ctx.is_null());
        assert_eq!(CertDeleteCertificateFromStore(stored_ctx), TRUE);
        assert!(CertEnumCertificatesInStore(my_store, std::ptr::null_mut()).is_null());

        for h in [
            any_c,
            by_subj_a,
            by_subj_w,
            by_iss_a,
            by_iss_w,
            by_sha1,
            by_md5,
            by_existing,
        ] {
            assert_eq!(CertFreeCertificateContext(h), TRUE);
        }
        for s in [root_a, ca_w, root_reg, my_store] {
            assert_eq!(CertCloseStore(s, 0), TRUE);
        }
    }
}

#[test]
fn cert_chain_revocation_flags_fail_closed_and_respect_ignore_flags() {
    use super::chain::{CRYPT_E_REVOCATION_OFFLINE, CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG};
    const CERT_CHAIN_REVOCATION_CHECK_END_CERT: u32 = 0x1000_0000;
    const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 0x4000_0000;
    const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 0x8000_0000;
    const CERT_TRUST_REVOCATION_STATUS_UNKNOWN: u32 = 0x0000_0040;
    const CERT_TRUST_IS_OFFLINE_REVOCATION: u32 = 0x0100_0000;
    const CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG: u32 = 0x0000_0100;
    const CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG: u32 = 0x0000_0400;

    unsafe {
        let cs_der = build_synthetic_cert(
            "Revocation Test Signer",
            "Revocation Test Signer",
            b"200101000000Z",
            b"400101000000Z",
            None,
            &[&[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x03]],
        );
        let ctx = CertCreateCertificateContext(
            X509_ASN_ENCODING,
            cs_der.as_ptr(),
            cs_der.len() as u32,
        );
        assert!(!ctx.is_null());

        let mut chain: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            CertGetCertificateChain(
                std::ptr::null_mut(),
                ctx,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT
                    | CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY,
                std::ptr::null_mut(),
                &mut chain,
            ),
            TRUE
        );
        assert!(!chain.is_null());
        let cc = std::ptr::read_unaligned(chain as *const crate::misc::wintrust::CertChainContext);
        assert_ne!(
            cc.trust_status.dw_error_status
                & (CERT_TRUST_REVOCATION_STATUS_UNKNOWN | CERT_TRUST_IS_OFFLINE_REVOCATION),
            0,
            "chain status must record unknown/offline revocation when positive check fails"
        );

        // Even with ALLOW_UNKNOWN_CA, AUTHENTICODE and BASE must fail closed on offline revocation
        let mut st: CertChainPolicyStatus = std::mem::zeroed();
        st.cb_size = std::mem::size_of::<CertChainPolicyStatus>() as u32;
        let para_ca_only = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG,
            pv_extra_policy_para: std::ptr::null(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_AUTHENTICODE as *const u8,
                chain,
                &para_ca_only as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CRYPT_E_REVOCATION_OFFLINE);

        // Ignoring only END_REV_UNKNOWN is insufficient for CHAIN_EXCLUDE_ROOT (needs CA too)
        let para_end_only = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG
                | CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG,
            pv_extra_policy_para: std::ptr::null(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_AUTHENTICODE as *const u8,
                chain,
                &para_end_only as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CRYPT_E_REVOCATION_OFFLINE);

        // Ignoring both END and CA revocation unknown clears the offline revocation error
        let para_both = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG
                | CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG
                | CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG,
            pv_extra_policy_para: std::ptr::null(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_AUTHENTICODE as *const u8,
                chain,
                &para_both as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(st.dw_error, 0);
        CertFreeCertificateChain(chain);

        // END_CERT mode fails closed until IGNORE_END_REV_UNKNOWN is explicitly passed
        let mut chain_end: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            CertGetCertificateChain(
                std::ptr::null_mut(),
                ctx,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                CERT_CHAIN_REVOCATION_CHECK_END_CERT | CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY,
                std::ptr::null_mut(),
                &mut chain_end,
            ),
            TRUE
        );
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_BASE as *const u8,
                chain_end,
                &para_ca_only as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CRYPT_E_REVOCATION_OFFLINE);
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_BASE as *const u8,
                chain_end,
                &para_end_only as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(st.dw_error, 0);
        CertFreeCertificateChain(chain_end);
        assert_eq!(CertFreeCertificateContext(ctx), TRUE);
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
        let exp_ctx = CertCreateCertificateContext(
            X509_ASN_ENCODING,
            expired_der.as_ptr(),
            expired_der.len() as u32,
        );
        assert!(!exp_ctx.is_null());
        let cc = std::ptr::read_unaligned(exp_ctx as *const CertContext);
        assert_eq!(
            CertVerifyTimeValidity(std::ptr::null(), cc.p_cert_info as *const c_void),
            1
        );

        let mut chain_exp = std::ptr::null_mut();
        assert_eq!(
            CertGetCertificateChain(
                std::ptr::null_mut(),
                exp_ctx,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                &mut chain_exp
            ),
            TRUE
        );

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
        let mut st = CertChainPolicyStatus {
            cb_size: 24,
            dw_error: 0,
            l_chain_index: 0,
            l_element_index: 0,
            pv_extra_policy_status: std::ptr::null_mut(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_SSL as *const u8,
                chain_exp,
                &para_allow_ca as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CERT_E_EXPIRED);

        // SSL policy with IGNORE_UNKNOWN_CA + IGNORE_DATE + matching wildcard passes!
        let ssl_allow_both = SslExtraCertChainPolicyPara {
            cb_size: 24,
            dw_auth_type: 2,
            fdw_checks: SECURITY_FLAG_IGNORE_UNKNOWN_CA | SECURITY_FLAG_IGNORE_CERT_DATE_INVALID,
            pwsz_server_name: host_ok.as_ptr(),
        };
        let para_both = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: 0,
            pv_extra_policy_para: &ssl_allow_both as *const _ as *const c_void,
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_SSL as *const u8,
                chain_exp,
                &para_both as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert_eq!(st.dw_error, 0);

        // SSL policy with IGNORE_UNKNOWN_CA + IGNORE_DATE + WRONG hostname MUST fail with CERT_E_CN_NO_MATCH!
        let host_bad = crate::strings::to_wide_z("a.b.example.org"); // wildcard is single-label only
        let ssl_bad_host = SslExtraCertChainPolicyPara {
            cb_size: 24,
            dw_auth_type: 2,
            fdw_checks: SECURITY_FLAG_IGNORE_UNKNOWN_CA | SECURITY_FLAG_IGNORE_CERT_DATE_INVALID,
            pwsz_server_name: host_bad.as_ptr(),
        };
        let para_bad_host = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: 0,
            pv_extra_policy_para: &ssl_bad_host as *const _ as *const c_void,
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_SSL as *const u8,
                chain_exp,
                &para_bad_host as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CERT_E_CN_NO_MATCH);

        // AUTHENTICODE policy on serverAuth-only cert MUST fail with CERT_E_WRONG_USAGE!
        let para_auth = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG
                | CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG,
            pv_extra_policy_para: std::ptr::null(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_AUTHENTICODE as *const u8,
                chain_exp,
                &para_auth as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CERT_E_WRONG_USAGE);

        // BASE policy fails with CERT_E_EXPIRED when date not ignored, passes when both CA & date ignored
        let para_base_ca_only = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG,
            pv_extra_policy_para: std::ptr::null(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_BASE as *const u8,
                chain_exp,
                &para_base_ca_only as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CERT_E_EXPIRED);
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_BASE as *const u8,
                chain_exp,
                &para_auth as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
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
        let cs_ctx = CertCreateCertificateContext(
            X509_ASN_ENCODING,
            cs_der.as_ptr(),
            cs_der.len() as u32,
        );
        let cs_cc = std::ptr::read_unaligned(cs_ctx as *const CertContext);
        assert_eq!(
            CertVerifyTimeValidity(std::ptr::null(), cs_cc.p_cert_info as *const c_void),
            0
        );
        let mut chain_cs = std::ptr::null_mut();
        assert_eq!(
            CertGetCertificateChain(
                std::ptr::null_mut(),
                cs_ctx,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                &mut chain_cs
            ),
            TRUE
        );

        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_AUTHENTICODE as *const u8,
                chain_cs,
                std::ptr::null(),
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert_eq!(st.dw_error, CERT_E_UNTRUSTEDROOT);

        let para_ca = CertChainPolicyPara {
            cb_size: 16,
            dw_flags: CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG,
            pv_extra_policy_para: std::ptr::null(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_AUTHENTICODE as *const u8,
                chain_cs,
                &para_ca as *const _ as *const c_void,
                &mut st as *mut _ as *mut c_void
            ),
            TRUE
        );
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
                CryptStringToBinaryA(
                    s.as_ptr(),
                    s.len() as u32,
                    flags,
                    std::ptr::null_mut(),
                    &mut cb,
                    &mut skip,
                    &mut actual
                ),
                TRUE
            );
            let mut out = vec![0u8; cb as usize];
            assert_eq!(
                CryptStringToBinaryA(
                    s.as_ptr(),
                    s.len() as u32,
                    flags,
                    out.as_mut_ptr(),
                    &mut cb,
                    &mut skip,
                    &mut actual
                ),
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
        assert_eq!(
            (b.as_slice(), skip, f),
            (&b"Hello"[..], 7, CRYPT_STRING_BASE64REQUESTHEADER)
        );

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
        assert_eq!(
            CryptStringToBinaryW(
                w_hex.as_ptr(),
                0,
                CRYPT_STRING_HEX,
                std::ptr::null_mut(),
                &mut wlen,
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            TRUE
        );
        let mut wout = vec![0u8; wlen as usize];
        assert_eq!(
            CryptStringToBinaryW(
                w_hex.as_ptr(),
                0,
                CRYPT_STRING_HEX,
                wout.as_mut_ptr(),
                &mut wlen,
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            TRUE
        );
        assert_eq!(wout, vec![0xDE, 0xAD, 0xBE, 0xEF]);

        // CryptEncodeObjectEx: X509_BASIC_CONSTRAINTS2 (both integer and OID string!)
        let bc = CertBasicConstraints2Info {
            f_ca: 1,
            f_path_len_constraint: 1,
            dw_path_len_constraint: 2,
        };
        let mut len = 0u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                b"2.5.29.19\0".as_ptr(),
                &bc as *const _ as *const c_void,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut len
            ),
            TRUE
        );
        let mut buf = vec![0u8; len as usize];
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_BASIC_CONSTRAINTS2 as *const u8,
                &bc as *const _ as *const c_void,
                0,
                std::ptr::null(),
                buf.as_mut_ptr() as *mut c_void,
                &mut len
            ),
            TRUE
        );
        assert_eq!(buf, vec![0x30, 0x06, 0x01, 0x01, 0xFF, 0x02, 0x01, 0x02]);

        // X509_ENHANCED_KEY_USAGE
        let oids = [
            b"1.3.6.1.5.5.7.3.1\0".as_ptr(),
            b"1.3.6.1.5.5.7.3.3\0".as_ptr(),
        ];
        let eku = CertEnhkeyUsage {
            c_usage_identifier: 2,
            rgpsz_usage_identifier: oids.as_ptr(),
        };
        let mut elen = 0u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ENHANCED_KEY_USAGE as *const u8,
                &eku as *const _ as *const c_void,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut elen
            ),
            TRUE
        );
        let mut ebuf = vec![0u8; elen as usize];
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ENHANCED_KEY_USAGE as *const u8,
                &eku as *const _ as *const c_void,
                0,
                std::ptr::null(),
                ebuf.as_mut_ptr() as *mut c_void,
                &mut elen
            ),
            TRUE
        );
        assert_eq!(ebuf[0], 0x30);
        assert!(ebuf
            .windows(8)
            .any(|w| w == [0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x03]));

        // X509_ALTERNATE_NAME (DNS + IPv4)
        let dns_w = crate::strings::to_wide_z("example.com");
        let ip4 = [127u8, 0, 0, 1];
        let mut e1 = CertAltNameEntry {
            dw_alt_name_choice: 3,
            payload: [0; 16],
        };
        std::ptr::write_unaligned(e1.payload.as_mut_ptr() as *mut *const u16, dns_w.as_ptr());
        let mut e2 = CertAltNameEntry {
            dw_alt_name_choice: 8,
            payload: [0; 16],
        };
        std::ptr::write_unaligned(
            e2.payload.as_mut_ptr() as *mut CryptDataBlob,
            CryptDataBlob {
                cb_data: 4,
                pb_data: ip4.as_ptr(),
            },
        );
        let entries = [e1, e2];
        let san = CertAltNameInfo {
            c_alt_entry: 2,
            rg_alt_entry: entries.as_ptr(),
        };
        let mut slen = 0u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ALTERNATE_NAME as *const u8,
                &san as *const _ as *const c_void,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut slen
            ),
            TRUE
        );
        let mut sbuf = vec![0u8; slen as usize];
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ALTERNATE_NAME as *const u8,
                &san as *const _ as *const c_void,
                0,
                std::ptr::null(),
                sbuf.as_mut_ptr() as *mut c_void,
                &mut slen
            ),
            TRUE
        );
        assert!(sbuf.windows(11).any(|w| w == b"example.com"));
        assert!(sbuf.windows(6).any(|w| w == [0x87, 0x04, 127, 0, 0, 1]));

        // X509_NAME -> format_dn roundtrip!
        let cn_str = b"Encode Test";
        let attr = CertRdnAttr {
            psz_obj_id: b"2.5.4.3\0".as_ptr(),
            dw_value_type: 13, // UTF8_STRING
            value: CryptDataBlob {
                cb_data: cn_str.len() as u32,
                pb_data: cn_str.as_ptr(),
            },
        };
        let rdn = CertRdn {
            c_rdn_attr: 1,
            rg_rdn_attr: &attr,
        };
        let ni = CertNameInfo {
            c_rdn: 1,
            rg_rdn: &rdn,
        };
        let mut nlen = 0u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_NAME as *const u8,
                &ni as *const _ as *const c_void,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut nlen
            ),
            TRUE
        );
        let mut nbuf = vec![0u8; nlen as usize];
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_NAME as *const u8,
                &ni as *const _ as *const c_void,
                0,
                std::ptr::null(),
                nbuf.as_mut_ptr() as *mut c_void,
                &mut nlen
            ),
            TRUE
        );
        assert_eq!(wintrust::format_dn(&nbuf).as_deref(), Some("CN=Encode Test"));

        // X509_ECC_SIGNATURE (LE -> DER SEQUENCE { INT r, INT s })
        let r_le = [0x01u8, 0x02];
        let s_le = [0x03u8, 0x80]; // BE = 0x8003 -> needs leading 0x00 in DER!
        let ecc_sig = CertEccSignature {
            r: CryptDataBlob {
                cb_data: 2,
                pb_data: r_le.as_ptr(),
            },
            s: CryptDataBlob {
                cb_data: 2,
                pb_data: s_le.as_ptr(),
            },
        };
        let mut eclen = 0u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ECC_SIGNATURE as *const u8,
                &ecc_sig as *const _ as *const c_void,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut eclen
            ),
            TRUE
        );
        let mut ecbuf = vec![0u8; eclen as usize];
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ECC_SIGNATURE as *const u8,
                &ecc_sig as *const _ as *const c_void,
                0,
                std::ptr::null(),
                ecbuf.as_mut_ptr() as *mut c_void,
                &mut eclen
            ),
            TRUE
        );
        assert_eq!(
            ecbuf,
            vec![0x30, 0x09, 0x02, 0x02, 0x02, 0x01, 0x02, 0x03, 0x00, 0x80, 0x03]
        );
    }
}

#[test]
fn crypt_binary_to_string_dpapi_and_cert_compare_work() {
    use crate::misc::wintrust::CryptDataBlob;
    unsafe {
        // 1. CryptBinaryToStringA/W round-trip with CryptStringToBinaryA
        let sample = b"Hello Crypt32 BinaryToString!";
        let mut cch = 0u32;
        assert_eq!(
            CryptBinaryToStringA(
                sample.as_ptr(),
                sample.len() as u32,
                0x0000_0001 | 0x4000_0000, // CRYPT_STRING_BASE64 | CRYPT_STRING_NOCRLF
                std::ptr::null_mut(),
                &mut cch
            ),
            TRUE
        );
        assert!(cch > 1);
        let mut out_a = vec![0u8; cch as usize];
        assert_eq!(
            CryptBinaryToStringA(
                sample.as_ptr(),
                sample.len() as u32,
                0x0000_0001 | 0x4000_0000,
                out_a.as_mut_ptr(),
                &mut cch
            ),
            TRUE
        );
        let b64 = std::str::from_utf8(&out_a[..cch as usize]).unwrap();
        assert_eq!(b64, "SGVsbG8gQ3J5cHQzMiBCaW5hcnlUb1N0cmluZyE=");

        // Wide hex raw
        let raw_hex = [0xDEu8, 0xAD, 0xBE, 0xEF];
        let mut wcch = 0u32;
        assert_eq!(
            CryptBinaryToStringW(
                raw_hex.as_ptr(),
                4,
                0x0000_000C | 0x4000_0000, // HEXRAW | NOCRLF
                std::ptr::null_mut(),
                &mut wcch
            ),
            TRUE
        );
        let mut out_w = vec![0u16; wcch as usize];
        assert_eq!(
            CryptBinaryToStringW(
                raw_hex.as_ptr(),
                4,
                0x0000_000C | 0x4000_0000,
                out_w.as_mut_ptr(),
                &mut wcch
            ),
            TRUE
        );
        assert_eq!(String::from_utf16_lossy(&out_w[..wcch as usize]), "deadbeef");

        // 2. CertCompareIntegerBlob (redundant sign-extension bytes match)
        let i1 = [0x2Au8];
        let i2 = [0x2Au8, 0x00, 0x00];
        let b1 = CryptDataBlob { cb_data: 1, pb_data: i1.as_ptr() as *mut u8 };
        let b2 = CryptDataBlob { cb_data: 3, pb_data: i2.as_ptr() as *mut u8 };
        assert_eq!(CertCompareIntegerBlob(&b1 as *const _ as *const c_void, &b2 as *const _ as *const c_void), TRUE);

        // 3. CertCompareCertificateName
        let n1 = b"\x30\x05\x31\x03test";
        let nb1 = CryptDataBlob { cb_data: n1.len() as u32, pb_data: n1.as_ptr() as *mut u8 };
        assert_eq!(CertCompareCertificateName(1, &nb1 as *const _ as *const c_void, &nb1 as *const _ as *const c_void), TRUE);

        // 4. CryptProtectData / CryptUnprotectData with entropy + tamper rejection
        let secret = b"top secret game token 12345";
        let ent = b"app-entropy";
        let descr = crate::strings::to_wide_z("session-key");
        let in_blob = CryptDataBlob { cb_data: secret.len() as u32, pb_data: secret.as_ptr() as *mut u8 };
        let ent_blob = CryptDataBlob { cb_data: ent.len() as u32, pb_data: ent.as_ptr() as *mut u8 };
        let mut protected = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        assert_eq!(
            CryptProtectData(
                &in_blob as *const _ as *const c_void,
                descr.as_ptr(),
                &ent_blob as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut protected as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert!(protected.cb_data > secret.len() as u32);

        let mut recovered = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        let mut out_descr: LPWSTR = std::ptr::null_mut();
        assert_eq!(
            CryptUnprotectData(
                &protected as *const _ as *const c_void,
                &mut out_descr,
                &ent_blob as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut recovered as *mut _ as *mut c_void
            ),
            TRUE
        );
        assert_eq!(
            std::slice::from_raw_parts(recovered.pb_data, recovered.cb_data as usize),
            secret
        );
        assert_eq!(crate::strings::from_wide(out_descr), "session-key");

        // Wrong entropy fails cleanly with ERROR_INVALID_DATA
        let bad_ent = b"wrong-entropy";
        let bad_blob = CryptDataBlob { cb_data: bad_ent.len() as u32, pb_data: bad_ent.as_ptr() as *mut u8 };
        let mut fail_out = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        assert_eq!(
            CryptUnprotectData(
                &protected as *const _ as *const c_void,
                std::ptr::null_mut(),
                &bad_blob as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut fail_out as *mut _ as *mut c_void
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_DATA);

        crate::kernel32::memory::LocalFree(protected.pb_data as *mut c_void);
        crate::kernel32::memory::LocalFree(recovered.pb_data as *mut c_void);
        crate::kernel32::memory::LocalFree(out_descr as *mut c_void);
    }
}

#[test]
fn dpapi_uses_0600_host_secrets_and_isolates_secrets_and_machine_scope() {
    use std::os::unix::fs::PermissionsExt;

    struct EnvGuard {
        prev_home: Option<std::ffi::OsString>,
        prev_drive: Option<std::ffi::OsString>,
        prev_user: Option<String>,
        dirs: Vec<std::path::PathBuf>,
    }
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.prev_home {
                Some(v) => std::env::set_var("TRANSLATOR_HOME", v),
                None => std::env::remove_var("TRANSLATOR_HOME"),
            }
            match &self.prev_drive {
                Some(v) => std::env::set_var("AW2_DRIVE_C", v),
                None => std::env::remove_var("AW2_DRIVE_C"),
            }
            crate::process::set_env("USERNAME", self.prev_user.as_deref());
            for d in &self.dirs {
                let _ = std::fs::remove_dir_all(d);
            }
        }
    }

    let base = std::env::temp_dir().join(format!("pf_dpapi_test_{}", std::process::id()));
    let dir_a = base.join("profile_a");
    let dir_b = base.join("profile_b");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&dir_a).unwrap();
    std::fs::create_dir_all(&dir_b).unwrap();

    let _guard = EnvGuard {
        prev_home: std::env::var_os("TRANSLATOR_HOME"),
        prev_drive: std::env::var_os("AW2_DRIVE_C"),
        prev_user: crate::process::get_env("USERNAME"),
        dirs: vec![base.clone()],
    };

    std::env::set_var("TRANSLATOR_HOME", &dir_a);
    std::env::remove_var("AW2_DRIVE_C");
    crate::process::set_env("USERNAME", Some("alice"));

    unsafe {
        let payload = b"dpapi-secret-payload-v2";
        let in_blob = CryptDataBlob {
            cb_data: payload.len() as u32,
            pb_data: payload.as_ptr() as *mut u8,
        };

        // 1. User-scoped protect creates a 32-byte 0600 secret file under profile_a.
        let mut user_ct = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        assert_eq!(
            CryptProtectData(
                &in_blob as *const _ as *const c_void,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut user_ct as *mut _ as *mut c_void,
            ),
            TRUE
        );
        let user_key_path = asn1::dpapi_secret_path(false);
        let meta = std::fs::metadata(&user_key_path).expect("user secret file must exist");
        assert_eq!(meta.len(), 32);
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);

        // 2. Same user & same secret decrypts cleanly.
        let mut out = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        assert_eq!(
            CryptUnprotectData(
                &user_ct as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut out as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(std::slice::from_raw_parts(out.pb_data, out.cb_data as usize), payload);
        crate::kernel32::memory::LocalFree(out.pb_data as *mut c_void);

        // 3. Different user ("bob") on same machine CANNOT decrypt alice's user-scoped blob.
        crate::process::set_env("USERNAME", Some("bob"));
        let mut fail_out = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        assert_eq!(
            CryptUnprotectData(
                &user_ct as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut fail_out as *mut _ as *mut c_void,
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_DATA);

        // 4. Same user ("alice") under a DIFFERENT host secret (profile_b) CANNOT decrypt it.
        crate::process::set_env("USERNAME", Some("alice"));
        std::env::set_var("TRANSLATOR_HOME", &dir_b);
        assert_eq!(
            CryptUnprotectData(
                &user_ct as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut fail_out as *mut _ as *mut c_void,
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_DATA);

        // 5. CRYPTPROTECT_LOCAL_MACHINE (0x4): distinct 0600 machine secret; decryptable by
        //    another user ("bob") on the same machine, NOT decryptable under profile_b's secret.
        std::env::set_var("TRANSLATOR_HOME", &dir_a);
        crate::process::set_env("USERNAME", Some("alice"));
        let mut mach_ct = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        assert_eq!(
            CryptProtectData(
                &in_blob as *const _ as *const c_void,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0x4, // CRYPTPROTECT_LOCAL_MACHINE
                &mut mach_ct as *mut _ as *mut c_void,
            ),
            TRUE
        );
        let mach_key_path = asn1::dpapi_secret_path(true);
        assert_ne!(mach_key_path, user_key_path);
        let mmeta = std::fs::metadata(&mach_key_path).expect("machine secret file must exist");
        assert_eq!(mmeta.len(), 32);
        assert_eq!(mmeta.permissions().mode() & 0o777, 0o600);

        // Another user on the same machine succeeds for CRYPTPROTECT_LOCAL_MACHINE.
        crate::process::set_env("USERNAME", Some("bob"));
        let mut mach_out = CryptDataBlob { cb_data: 0, pb_data: std::ptr::null_mut() };
        assert_eq!(
            CryptUnprotectData(
                &mach_ct as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut mach_out as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(std::slice::from_raw_parts(mach_out.pb_data, mach_out.cb_data as usize), payload);
        crate::kernel32::memory::LocalFree(mach_out.pb_data as *mut c_void);

        // Switching to profile_b's different machine secret fails cleanly.
        std::env::set_var("TRANSLATOR_HOME", &dir_b);
        assert_eq!(
            CryptUnprotectData(
                &mach_ct as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut fail_out as *mut _ as *mut c_void,
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_DATA);

        // Flipping the scope flag in the envelope fails MAC verification even back on profile_a.
        std::env::set_var("TRANSLATOR_HOME", &dir_a);
        let mut tampered = std::slice::from_raw_parts(mach_ct.pb_data, mach_ct.cb_data as usize).to_vec();
        tampered[8] ^= 0x4;
        let tampered_blob = CryptDataBlob {
            cb_data: tampered.len() as u32,
            pb_data: tampered.as_mut_ptr(),
        };
        assert_eq!(
            CryptUnprotectData(
                &tampered_blob as *const _ as *const c_void,
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut fail_out as *mut _ as *mut c_void,
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_DATA);

        crate::kernel32::memory::LocalFree(user_ct.pb_data as *mut c_void);
        crate::kernel32::memory::LocalFree(mach_ct.pb_data as *mut c_void);
    }
}

#[test]
fn crypt_find_oid_info_all_key_types_and_groups() {
    use super::decode::{
        CryptOidInfo, CRYPT_HASH_ALG_OID_GROUP_ID, CRYPT_OID_INFO_ALGID_KEY,
        CRYPT_OID_INFO_CNG_ALGID_KEY, CRYPT_OID_INFO_CNG_SIGN_KEY, CRYPT_OID_INFO_NAME_KEY,
        CRYPT_OID_INFO_OID_KEY, CRYPT_OID_INFO_SIGN_KEY, CRYPT_RDN_ATTR_OID_GROUP_ID,
        CRYPT_SIGN_ALG_OID_GROUP_ID,
    };
    unsafe {
        // 1. Lookup by OID string
        let p = CryptFindOIDInfo(
            CRYPT_OID_INFO_OID_KEY,
            b"1.2.840.113549.1.1.11\0".as_ptr() as *const c_void,
            0,
        ) as *const CryptOidInfo;
        assert!(!p.is_null());
        let info = std::ptr::read_unaligned(p);
        assert_eq!(info.cb_size, 64);
        assert_eq!(info.dw_group_id, CRYPT_SIGN_ALG_OID_GROUP_ID);
        assert_eq!(info.dw_value, 0x800C); // CALG_SHA_256
        assert_eq!(crate::strings::from_wide(info.pwsz_name), "sha256RSA");
        assert_eq!(crate::strings::from_wide(info.pwsz_cng_algid), "SHA256");
        assert_eq!(crate::strings::from_wide(info.pwsz_cng_extra_algid), "RSA");

        // 2. Lookup by wide friendly name (case-insensitive)
        let name_w = crate::strings::to_wide_z("cn");
        let p2 = CryptFindOIDInfo(
            CRYPT_OID_INFO_NAME_KEY,
            name_w.as_ptr() as *const c_void,
            CRYPT_RDN_ATTR_OID_GROUP_ID,
        ) as *const CryptOidInfo;
        assert!(!p2.is_null());
        assert_eq!(crate::strings::from_ansi((*p2).psz_oid), "2.5.4.3");

        // 3. Lookup by ALGID
        let algid = 0x800Cu32; // CALG_SHA_256
        let p3 = CryptFindOIDInfo(
            CRYPT_OID_INFO_ALGID_KEY,
            &algid as *const u32 as *const c_void,
            CRYPT_HASH_ALG_OID_GROUP_ID,
        ) as *const CryptOidInfo;
        assert!(!p3.is_null());
        assert_eq!(crate::strings::from_ansi((*p3).psz_oid), "2.16.840.1.101.3.4.2.1");

        // 4. Lookup by SIGN key [CALG_SHA_256, CALG_RSA_SIGN]
        let sign_pair = [0x800Cu32, 0x2400u32];
        let p4 = CryptFindOIDInfo(
            CRYPT_OID_INFO_SIGN_KEY,
            sign_pair.as_ptr() as *const c_void,
            0,
        ) as *const CryptOidInfo;
        assert!(!p4.is_null());
        assert_eq!(crate::strings::from_ansi((*p4).psz_oid), "1.2.840.113549.1.1.11");

        // 5. Lookup by CNG ALGID and CNG SIGN key
        let sha384_w = crate::strings::to_wide_z("SHA384");
        let ecdsa_w = crate::strings::to_wide_z("ECDSA");
        let p5 = CryptFindOIDInfo(
            CRYPT_OID_INFO_CNG_ALGID_KEY,
            sha384_w.as_ptr() as *const c_void,
            CRYPT_HASH_ALG_OID_GROUP_ID,
        ) as *const CryptOidInfo;
        assert!(!p5.is_null());
        assert_eq!(crate::strings::from_ansi((*p5).psz_oid), "2.16.840.1.101.3.4.2.2");

        let cng_pair = [sha384_w.as_ptr(), ecdsa_w.as_ptr()];
        let p6 = CryptFindOIDInfo(
            CRYPT_OID_INFO_CNG_SIGN_KEY,
            cng_pair.as_ptr() as *const c_void,
            0,
        ) as *const CryptOidInfo;
        assert!(!p6.is_null());
        assert_eq!(crate::strings::from_ansi((*p6).psz_oid), "1.2.840.10045.4.3.3");

        // 6. Unknown OID returns NULL
        assert!(CryptFindOIDInfo(
            CRYPT_OID_INFO_OID_KEY,
            b"1.2.3.4.5.99999\0".as_ptr() as *const c_void,
            0,
        )
        .is_null());
    }
}

#[test]
fn cert_str_to_name_roundtrip_and_errors() {
    unsafe {
        // 1. ANSI X.500 DN with quoted comma & roundtrip via CertNameToStrA
        let dn_in = b"CN=\"Acme, Inc.\", O=PortFactory, C=US\0";
        let mut cb = 0u32;
        assert_eq!(
            CertStrToNameA(
                X509_ASN_ENCODING,
                dn_in.as_ptr(),
                3, // CERT_X500_NAME_STR
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cb,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        assert!(cb > 20);
        // Too-small buffer -> ERROR_MORE_DATA
        let mut small = [0u8; 4];
        let mut small_cb = 4u32;
        assert_eq!(
            CertStrToNameA(
                X509_ASN_ENCODING,
                dn_in.as_ptr(),
                3,
                std::ptr::null_mut(),
                small.as_mut_ptr(),
                &mut small_cb,
                std::ptr::null_mut(),
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_MORE_DATA);
        assert_eq!(small_cb, cb);

        let mut der = vec![0u8; cb as usize];
        assert_eq!(
            CertStrToNameA(
                X509_ASN_ENCODING,
                dn_in.as_ptr(),
                3,
                std::ptr::null_mut(),
                der.as_mut_ptr(),
                &mut cb,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        let blob = CryptDataBlob {
            cb_data: cb,
            pb_data: der.as_ptr(),
        };
        let mut out_a = [0u8; 128];
        let written = CertNameToStrA(
            X509_ASN_ENCODING,
            &blob as *const _ as *const c_void,
            3,
            out_a.as_mut_ptr(),
            out_a.len() as u32,
        );
        assert!(written > 10);
        let formatted = crate::strings::from_ansi(out_a.as_ptr());
        assert!(formatted.contains("CN=Acme, Inc."));
        assert!(formatted.contains("O=PortFactory"));
        assert!(formatted.contains("C=US"));

        // 2. Wide version + CERT_NAME_STR_REVERSE_FLAG
        let dn_w = crate::strings::to_wide_z("CN=Leaf, O=Root");
        let mut cb_w = 0u32;
        assert_eq!(
            CertStrToNameW(
                X509_ASN_ENCODING,
                dn_w.as_ptr(),
                3 | 0x0200_0000, // REVERSE
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cb_w,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        let mut der_w = vec![0u8; cb_w as usize];
        assert_eq!(
            CertStrToNameW(
                X509_ASN_ENCODING,
                dn_w.as_ptr(),
                3 | 0x0200_0000,
                std::ptr::null_mut(),
                der_w.as_mut_ptr(),
                &mut cb_w,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        let rev_str = wintrust::format_dn(&der_w).unwrap();
        assert_eq!(rev_str, "O=Root, CN=Leaf");

        // 3. Syntax error reports exact ppszError pointer + CRYPT_E_INVALID_X500_STRING
        let bad = b"CN=Ok, BADKEY=foo\0";
        let mut err_ptr: LPCSTR = std::ptr::null();
        let mut bad_cb = 0u32;
        assert_eq!(
            CertStrToNameA(
                X509_ASN_ENCODING,
                bad.as_ptr(),
                3,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut bad_cb,
                &mut err_ptr,
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), 0x8009_2023);
        assert_eq!(err_ptr as usize - bad.as_ptr() as usize, 7);
    }
}

#[test]
fn crypt_decode_object_and_ex_roundtrip_all_types() {
    use super::asn1::{
        CRYPT_STRING_PERCENTESCAPE, X509_INTEGER, X509_MULTI_BYTE_UINT, X509_OBJECT_IDENTIFIER,
        X509_OCTET_STRING, X509_PUBLIC_KEY_INFO, X509_UNICODE_NAME, RSA_CSP_PUBLICKEYBLOB,
    };
    use super::decode::{CNG_RSA_PUBLIC_KEY_BLOB, X509_AUTHORITY_KEY_ID2, X509_ENUMERATED};
    unsafe {
        // 1. Encode -> Decode BasicConstraints2 via CryptDecodeObjectEx(ALLOC)
        let bc_in = CertBasicConstraints2Info {
            f_ca: 1,
            f_path_len_constraint: 1,
            dw_path_len_constraint: 3,
        };
        let mut der_ptr: *mut u8 = std::ptr::null_mut();
        let mut der_len = 0u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_BASIC_CONSTRAINTS2 as *const u8,
                &bc_in as *const _ as *const c_void,
                0x8000,
                std::ptr::null(),
                &mut der_ptr as *mut _ as *mut c_void,
                &mut der_len,
            ),
            TRUE
        );
        let mut bc_out_ptr: *mut CertBasicConstraints2Info = std::ptr::null_mut();
        let mut bc_out_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                b"2.5.29.19\0".as_ptr(),
                der_ptr,
                der_len,
                0x8000,
                std::ptr::null(),
                &mut bc_out_ptr as *mut _ as *mut c_void,
                &mut bc_out_len,
            ),
            TRUE
        );
        assert_eq!((*bc_out_ptr).f_ca, 1);
        assert_eq!((*bc_out_ptr).f_path_len_constraint, 1);
        assert_eq!((*bc_out_ptr).dw_path_len_constraint, 3);
        crate::kernel32::memory::LocalFree(der_ptr as *mut c_void);
        crate::kernel32::memory::LocalFree(bc_out_ptr as *mut c_void);

        // 2. X509_UNICODE_NAME via CryptDecodeObject (caller buffer + two-call sizing)
        let dn_w = crate::strings::to_wide_z("CN=DecodeTest, O=PortFactory");
        let mut dn_der_len = 0u32;
        assert_eq!(
            CertStrToNameW(
                X509_ASN_ENCODING,
                dn_w.as_ptr(),
                3,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut dn_der_len,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        let mut dn_der = vec![0u8; dn_der_len as usize];
        assert_eq!(
            CertStrToNameW(
                X509_ASN_ENCODING,
                dn_w.as_ptr(),
                3,
                std::ptr::null_mut(),
                dn_der.as_mut_ptr(),
                &mut dn_der_len,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        let mut need = 0u32;
        assert_eq!(
            CryptDecodeObject(
                X509_ASN_ENCODING,
                X509_UNICODE_NAME as *const u8,
                dn_der.as_ptr(),
                dn_der_len,
                0,
                std::ptr::null_mut(),
                &mut need,
            ),
            TRUE
        );
        let mut buf = vec![0u8; need as usize];
        assert_eq!(
            CryptDecodeObject(
                X509_ASN_ENCODING,
                X509_UNICODE_NAME as *const u8,
                dn_der.as_ptr(),
                dn_der_len,
                0,
                buf.as_mut_ptr() as *mut c_void,
                &mut need,
            ),
            TRUE
        );
        let ni = std::ptr::read_unaligned(buf.as_ptr() as *const CertNameInfo);
        assert_eq!(ni.c_rdn, 2);
        let rdn0 = std::ptr::read_unaligned(ni.rg_rdn);
        assert_eq!(rdn0.c_rdn_attr, 1);
        let attr0 = std::ptr::read_unaligned(rdn0.rg_rdn_attr);
        assert_eq!(crate::strings::from_ansi(attr0.psz_obj_id), "2.5.4.3");
        assert_eq!(
            crate::strings::from_wide(attr0.value.pb_data as *const u16),
            "DecodeTest"
        );

        // 3. X509_ENHANCED_KEY_USAGE + X509_INTEGER + X509_ENUMERATED + X509_OBJECT_IDENTIFIER + X509_OCTET_STRING
        let eku_oids = [b"1.3.6.1.5.5.7.3.1\0".as_ptr(), b"1.3.6.1.5.5.7.3.3\0".as_ptr()];
        let eku_in = CertEnhkeyUsage {
            c_usage_identifier: 2,
            rgpsz_usage_identifier: eku_oids.as_ptr(),
        };
        let mut eku_der = [0u8; 64];
        let mut eku_der_len = eku_der.len() as u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ENHANCED_KEY_USAGE as *const u8,
                &eku_in as *const _ as *const c_void,
                0,
                std::ptr::null(),
                eku_der.as_mut_ptr() as *mut c_void,
                &mut eku_der_len,
            ),
            TRUE
        );
        let mut eku_dec: *mut CertEnhkeyUsage = std::ptr::null_mut();
        let mut eku_dec_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                b"2.5.29.37\0".as_ptr(),
                eku_der.as_ptr(),
                eku_der_len,
                0x8000,
                std::ptr::null(),
                &mut eku_dec as *mut _ as *mut c_void,
                &mut eku_dec_len,
            ),
            TRUE
        );
        assert_eq!((*eku_dec).c_usage_identifier, 2);
        assert_eq!(
            crate::strings::from_ansi(*(*eku_dec).rgpsz_usage_identifier),
            "1.3.6.1.5.5.7.3.1"
        );
        assert_eq!(
            crate::strings::from_ansi(*(*eku_dec).rgpsz_usage_identifier.add(1)),
            "1.3.6.1.5.5.7.3.3"
        );
        crate::kernel32::memory::LocalFree(eku_dec as *mut c_void);

        // CryptDecodeObjectEx with CRYPT_DECODE_ALLOC_FLAG + custom CRYPT_DECODE_PARA.pfnAlloc
        static CUSTOM_ALLOC_CALLS: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(0);
        static CUSTOM_ALLOC_LAST_PTR: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(0);
        unsafe extern "C" fn custom_crypt_alloc(cb: usize) -> *mut c_void {
            CUSTOM_ALLOC_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let p = crate::heap::alloc(cb, true);
            CUSTOM_ALLOC_LAST_PTR.store(p as usize, std::sync::atomic::Ordering::SeqCst);
            p
        }
        #[repr(C)]
        struct CryptDecodePara {
            cb_size: u32,
            _pad: u32,
            pfn_alloc: usize,
            pfn_free: usize,
        }
        let decode_para = CryptDecodePara {
            cb_size: std::mem::size_of::<CryptDecodePara>() as u32,
            _pad: 0,
            pfn_alloc: custom_crypt_alloc as *const () as usize,
            pfn_free: 0,
        };
        let before_calls = CUSTOM_ALLOC_CALLS.load(std::sync::atomic::Ordering::SeqCst);
        let mut custom_eku: *mut CertEnhkeyUsage = std::ptr::null_mut();
        let mut custom_eku_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                b"2.5.29.37\0".as_ptr(),
                eku_der.as_ptr(),
                eku_der_len,
                0x8000,
                &decode_para as *const _ as *const c_void,
                &mut custom_eku as *mut _ as *mut c_void,
                &mut custom_eku_len,
            ),
            TRUE
        );
        assert_eq!(
            CUSTOM_ALLOC_CALLS.load(std::sync::atomic::Ordering::SeqCst),
            before_calls + 1
        );
        assert_eq!(
            custom_eku as usize,
            CUSTOM_ALLOC_LAST_PTR.load(std::sync::atomic::Ordering::SeqCst)
        );
        assert_eq!((*custom_eku).c_usage_identifier, 2);
        assert_eq!(
            crate::strings::from_ansi(*(*custom_eku).rgpsz_usage_identifier),
            "1.3.6.1.5.5.7.3.1"
        );
        crate::heap::free(custom_eku as *mut c_void);

        // X509_CERT_TO_BE_SIGNED: unwraps full signed certificate by default (dwFlags = 0),
        // and rejects signed wrapper / decodes raw TBS when CRYPT_DECODE_TO_BE_SIGNED_FLAG is set.
        let pfx_blob_dec = CryptDataBlob {
            cb_data: self::test_vectors::PFX_PBES2_AES256.len() as u32,
            pb_data: self::test_vectors::PFX_PBES2_AES256.as_ptr(),
        };
        let pfx_pw = crate::strings::to_wide_z("testpass123");
        let pfx_st = PFXImportCertStore(&pfx_blob_dec as *const _ as *const c_void, pfx_pw.as_ptr(), 0);
        let pfx_cert = CertEnumCertificatesInStore(pfx_st, std::ptr::null_mut());
        let pfx_cc = std::ptr::read_unaligned(pfx_cert as *const crate::misc::wintrust::CertContext);
        let signed_cert_der = std::slice::from_raw_parts(pfx_cc.pb_cert_encoded, pfx_cc.cb_cert_encoded as usize);

        let mut ci_ptr: *mut crate::misc::wintrust::CertInfo = std::ptr::null_mut();
        let mut ci_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                super::asn1::X509_CERT_TO_BE_SIGNED as *const u8,
                signed_cert_der.as_ptr(),
                signed_cert_der.len() as u32,
                0x8000, // CRYPT_DECODE_ALLOC_FLAG only (dwFlags without CRYPT_DECODE_TO_BE_SIGNED_FLAG)
                std::ptr::null(),
                &mut ci_ptr as *mut _ as *mut c_void,
                &mut ci_len,
            ),
            TRUE
        );
        assert!(!ci_ptr.is_null());
        assert_eq!((*ci_ptr).dw_version, 2); // X.509 v3
        assert!((*ci_ptr).serial_number.cb_data > 0);
        assert!((*ci_ptr).subject.cb_data > 0);
        assert!((*ci_ptr).subject_public_key_info.public_key.cb_data > 0);
        crate::kernel32::memory::LocalFree(ci_ptr as *mut c_void);

        // Passing signed cert with CRYPT_DECODE_TO_BE_SIGNED_FLAG (0x2) must NOT unwrap it -> fails
        let mut bad_ci_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                super::asn1::X509_CERT_TO_BE_SIGNED as *const u8,
                signed_cert_der.as_ptr(),
                signed_cert_der.len() as u32,
                super::decode::CRYPT_DECODE_TO_BE_SIGNED_FLAG,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut bad_ci_len,
            ),
            FALSE
        );

        // Extract raw TBS from signed cert via X509_CERT and decode it with CRYPT_DECODE_TO_BE_SIGNED_FLAG
        let mut signed_info_ptr: *mut u8 = std::ptr::null_mut();
        let mut signed_info_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                super::asn1::X509_CERT as *const u8,
                signed_cert_der.as_ptr(),
                signed_cert_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut signed_info_ptr as *mut _ as *mut c_void,
                &mut signed_info_len,
            ),
            TRUE
        );
        let tbs_blob = std::ptr::read_unaligned(signed_info_ptr as *const CryptDataBlob);
        // Verify X509_CERT reverses signature bytes by default and preserves DER order with
        // CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG.
        let (_, _) = super::asn1::parse_der_item(&signed_cert_der).unwrap();
        let outer_cert_kids =
            super::asn1::der_item_children(super::asn1::parse_der_item(&signed_cert_der).unwrap().0.val)
                .unwrap();
        let der_sig = &outer_cert_kids[2].val[1..];
        let mut expected_rev_sig = der_sig.to_vec();
        expected_rev_sig.reverse();
        let dec_sig_blob =
            std::ptr::read_unaligned(signed_info_ptr.add(40) as *const crate::misc::wintrust::CryptBitBlob);
        assert_eq!(
            std::slice::from_raw_parts(dec_sig_blob.pb_data, dec_sig_blob.cb_data as usize),
            expected_rev_sig.as_slice()
        );
        let mut signed_norev_ptr: *mut u8 = std::ptr::null_mut();
        let mut signed_norev_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                super::asn1::X509_CERT as *const u8,
                signed_cert_der.as_ptr(),
                signed_cert_der.len() as u32,
                0x8000 | super::decode::CRYPT_DECODE_NO_SIGNATURE_BYTE_REVERSAL_FLAG,
                std::ptr::null(),
                &mut signed_norev_ptr as *mut _ as *mut c_void,
                &mut signed_norev_len,
            ),
            TRUE
        );
        let norev_sig_blob =
            std::ptr::read_unaligned(signed_norev_ptr.add(40) as *const crate::misc::wintrust::CryptBitBlob);
        assert_eq!(
            std::slice::from_raw_parts(norev_sig_blob.pb_data, norev_sig_blob.cb_data as usize),
            der_sig
        );
        crate::kernel32::memory::LocalFree(signed_norev_ptr as *mut c_void);

        let mut ci_tbs_ptr: *mut crate::misc::wintrust::CertInfo = std::ptr::null_mut();
        let mut ci_tbs_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                super::asn1::X509_CERT_TO_BE_SIGNED as *const u8,
                tbs_blob.pb_data,
                tbs_blob.cb_data,
                0x8000 | super::decode::CRYPT_DECODE_TO_BE_SIGNED_FLAG,
                std::ptr::null(),
                &mut ci_tbs_ptr as *mut _ as *mut c_void,
                &mut ci_tbs_len,
            ),
            TRUE
        );
        assert_eq!((*ci_tbs_ptr).dw_version, 2);
        crate::kernel32::memory::LocalFree(ci_tbs_ptr as *mut c_void);
        crate::kernel32::memory::LocalFree(signed_info_ptr as *mut c_void);
        CertFreeCertificateContext(pfx_cert);
        CertCloseStore(pfx_st, 0);

        // X509_CERT_CRL_TO_BE_SIGNED: unwraps signed CertificateList by default and supports CRYPT_DECODE_TO_BE_SIGNED_FLAG
        let sha256_rsa_alg = encode_der_tlv(
            0x30,
            &[
                0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B, 0x05, 0x00,
            ],
        );
        let crl_issuer = encode_der_tlv(
            0x30,
            &encode_der_tlv(
                0x31,
                &encode_der_tlv(0x30, &[0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x02, b'C', b'A']),
            ),
        );
        let mut tbs_crl_body = Vec::new();
        tbs_crl_body.extend_from_slice(&encode_der_tlv(0x02, &[0x01])); // version v2
        tbs_crl_body.extend_from_slice(&sha256_rsa_alg);
        tbs_crl_body.extend_from_slice(&crl_issuer);
        tbs_crl_body.extend_from_slice(&encode_der_tlv(0x17, b"260101000000Z"));
        let tbs_crl = encode_der_tlv(0x30, &tbs_crl_body);
        let mut signed_crl_body = Vec::new();
        signed_crl_body.extend_from_slice(&tbs_crl);
        signed_crl_body.extend_from_slice(&sha256_rsa_alg);
        signed_crl_body.extend_from_slice(&encode_der_tlv(0x03, &[0x00, 0xAA, 0xBB]));
        let signed_crl = encode_der_tlv(0x30, &signed_crl_body);

        let mut crl_info_buf = [0u8; 256];
        let mut crl_info_len = crl_info_buf.len() as u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                super::decode::X509_CERT_CRL_TO_BE_SIGNED as *const u8,
                signed_crl.as_ptr(),
                signed_crl.len() as u32,
                0,
                std::ptr::null(),
                crl_info_buf.as_mut_ptr() as *mut c_void,
                &mut crl_info_len,
            ),
            TRUE
        );
        assert_eq!(std::ptr::read_unaligned(crl_info_buf.as_ptr() as *const u32), 1);
        let mut crl_tbs_len = crl_info_buf.len() as u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                super::decode::X509_CERT_CRL_TO_BE_SIGNED as *const u8,
                tbs_crl.as_ptr(),
                tbs_crl.len() as u32,
                super::decode::CRYPT_DECODE_TO_BE_SIGNED_FLAG,
                std::ptr::null(),
                crl_info_buf.as_mut_ptr() as *mut c_void,
                &mut crl_tbs_len,
            ),
            TRUE
        );
        assert_eq!(std::ptr::read_unaligned(crl_info_buf.as_ptr() as *const u32), 1);

        // X509_ENUMERATED & X509_INTEGER
        let enum_in = 5i32;
        let mut enum_der = [0u8; 16];
        let mut enum_der_len = 16u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING,
                X509_ENUMERATED as *const u8,
                &enum_in as *const _ as *const c_void,
                0,
                std::ptr::null(),
                enum_der.as_mut_ptr() as *mut c_void,
                &mut enum_der_len,
            ),
            TRUE
        );
        assert_eq!(enum_der[0], 0x0A);
        let mut enum_out = 0i32;
        let mut enum_out_len = 4u32;
        assert_eq!(
            CryptDecodeObject(
                X509_ASN_ENCODING,
                X509_ENUMERATED as *const u8,
                enum_der.as_ptr(),
                enum_der_len,
                0,
                &mut enum_out as *mut _ as *mut c_void,
                &mut enum_out_len,
            ),
            TRUE
        );
        assert_eq!(enum_out, 5);

        let int_der = encode_der_tlv(0x02, &[0xFF, 0x80]); // -128
        let mut int_out = 0i32;
        let mut int_out_len = 4u32;
        assert_eq!(
            CryptDecodeObject(
                X509_ASN_ENCODING,
                X509_INTEGER as *const u8,
                int_der.as_ptr(),
                int_der.len() as u32,
                0,
                &mut int_out as *mut _ as *mut c_void,
                &mut int_out_len,
            ),
            TRUE
        );
        assert_eq!(int_out, -128);

        // RSA_CSP_PUBLICKEYBLOB & CNG_RSA_PUBLIC_KEY_BLOB roundtrip + SPKI conversion
        let rsa_pub_der = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x02, &[0x00, 0xC1, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]),
                encode_der_tlv(0x02, &[0x01, 0x00, 0x01]),
            ]
            .concat(),
        );
        let mut csp_ptr: *mut u8 = std::ptr::null_mut();
        let mut csp_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                RSA_CSP_PUBLICKEYBLOB as *const u8,
                rsa_pub_der.as_ptr(),
                rsa_pub_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut csp_ptr as *mut _ as *mut c_void,
                &mut csp_len,
            ),
            TRUE
        );
        assert_eq!(csp_len, 28);
        assert_eq!(std::ptr::read_unaligned(csp_ptr.add(8) as *const u32), 0x3141_5352);
        assert_eq!(std::ptr::read_unaligned(csp_ptr.add(12) as *const u32), 64);
        assert_eq!(std::ptr::read_unaligned(csp_ptr.add(16) as *const u32), 65537);
        assert_eq!(*csp_ptr.add(20), 0x08); // LE modulus
        crate::kernel32::memory::LocalFree(csp_ptr as *mut c_void);

        let mut cng_ptr: *mut u8 = std::ptr::null_mut();
        let mut cng_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                CNG_RSA_PUBLIC_KEY_BLOB as *const u8,
                rsa_pub_der.as_ptr(),
                rsa_pub_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut cng_ptr as *mut _ as *mut c_void,
                &mut cng_len,
            ),
            TRUE
        );
        assert_eq!(cng_len, 24 + 3 + 8);
        crate::kernel32::memory::LocalFree(cng_ptr as *mut c_void);

        // X509_OCTET_STRING, X509_MULTI_BYTE_UINT, X509_OBJECT_IDENTIFIER, X509_AUTHORITY_KEY_ID2, X509_PUBLIC_KEY_INFO
        let oct_der = encode_der_tlv(0x04, b"hello");
        let mut oct_out: *mut CryptDataBlob = std::ptr::null_mut();
        let mut oct_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                X509_OCTET_STRING as *const u8,
                oct_der.as_ptr(),
                oct_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut oct_out as *mut _ as *mut c_void,
                &mut oct_len,
            ),
            TRUE
        );
        assert_eq!(
            std::slice::from_raw_parts((*oct_out).pb_data, (*oct_out).cb_data as usize),
            b"hello"
        );
        crate::kernel32::memory::LocalFree(oct_out as *mut c_void);

        // Verify Windows SDK numeric identifiers directly (wincrypt.h):
        // 35 = X509_CRL_DIST_POINTS, 38 = X509_MULTI_BYTE_UINT, 48 = PKCS_ATTRIBUTES
        assert_eq!(X509_MULTI_BYTE_UINT, 38);
        let uint_der = encode_der_tlv(0x02, &[0x00, 0xAB, 0xCD]);
        let mut uint_out: *mut CryptDataBlob = std::ptr::null_mut();
        let mut uint_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                38 as *const u8, // X509_MULTI_BYTE_UINT
                uint_der.as_ptr(),
                uint_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut uint_out as *mut _ as *mut c_void,
                &mut uint_len,
            ),
            TRUE
        );
        assert_eq!(
            std::slice::from_raw_parts((*uint_out).pb_data, (*uint_out).cb_data as usize),
            &[0xCD, 0xAB]
        );
        crate::kernel32::memory::LocalFree(uint_out as *mut c_void);

        let cdp_uri = encode_der_tlv(0x86, b"http://crl.example.com/root.crl");
        let cdp_der = encode_der_tlv(
            0x30,
            &encode_der_tlv(0x30, &encode_der_tlv(0xA0, &encode_der_tlv(0xA0, &cdp_uri))),
        );
        let mut cdp_out: *mut u8 = std::ptr::null_mut();
        let mut cdp_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                35 as *const u8, // X509_CRL_DIST_POINTS
                cdp_der.as_ptr(),
                cdp_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut cdp_out as *mut _ as *mut c_void,
                &mut cdp_len,
            ),
            TRUE
        );
        assert_eq!(std::ptr::read_unaligned(cdp_out as *const u32), 1);
        crate::kernel32::memory::LocalFree(cdp_out as *mut c_void);

        let oid_der = encode_der_tlv(0x06, &[0x55, 0x04, 0x03]);
        let mut oid_out: *mut *const u8 = std::ptr::null_mut();
        let mut oid_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                X509_OBJECT_IDENTIFIER as *const u8,
                oid_der.as_ptr(),
                oid_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut oid_out as *mut _ as *mut c_void,
                &mut oid_len,
            ),
            TRUE
        );
        assert_eq!(crate::strings::from_ansi(*oid_out), "2.5.4.3");
        crate::kernel32::memory::LocalFree(oid_out as *mut c_void);

        let aki_der = encode_der_tlv(0x30, &encode_der_tlv(0x80, &[0x11, 0x22, 0x33, 0x44]));
        let mut aki_out: *mut CryptDataBlob = std::ptr::null_mut();
        let mut aki_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                X509_AUTHORITY_KEY_ID2 as *const u8,
                aki_der.as_ptr(),
                aki_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut aki_out as *mut _ as *mut c_void,
                &mut aki_len,
            ),
            TRUE
        );
        assert_eq!(
            std::slice::from_raw_parts((*aki_out).pb_data, (*aki_out).cb_data as usize),
            &[0x11, 0x22, 0x33, 0x44]
        );
        crate::kernel32::memory::LocalFree(aki_out as *mut c_void);

        // X509_PUBLIC_KEY_INFO decode + spki_to_capi_public_key_blob
        let spki_der = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(
                    0x30,
                    &[
                        encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01]),
                        encode_der_tlv(0x05, &[]),
                    ]
                    .concat(),
                ),
                encode_der_tlv(0x03, &[&[0x00u8][..], rsa_pub_der.as_slice()].concat()),
            ]
            .concat(),
        );
        let mut spki_out: *mut c_void = std::ptr::null_mut();
        let mut spki_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                X509_PUBLIC_KEY_INFO as *const u8,
                spki_der.as_ptr(),
                spki_der.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut spki_out as *mut _ as *mut c_void,
                &mut spki_len,
            ),
            TRUE
        );
        let capi_blob = super::decode::spki_to_capi_public_key_blob(spki_out).unwrap();
        assert_eq!(capi_blob.len(), 28);
        assert_eq!(u32::from_le_bytes(capi_blob[8..12].try_into().unwrap()), 0x3141_5352);
        crate::kernel32::memory::LocalFree(spki_out);

        // PKCS_ATTRIBUTES (SDK ID 48) accepts implicit unsigned-attributes tag 0xA1 and round-trips via CryptEncodeObjectEx
        let unauth_attr_item = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x06]), // countersignature
                encode_der_tlv(0x31, &encode_der_tlv(0x04, &[0xAA, 0xBB])),
            ]
            .concat(),
        );
        let unauth_attrs_a1 = encode_der_tlv(0xA1, &unauth_attr_item);
        let mut unauth_out: *mut u8 = std::ptr::null_mut();
        let mut unauth_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
                48 as *const u8, // PKCS_ATTRIBUTES
                unauth_attrs_a1.as_ptr(),
                unauth_attrs_a1.len() as u32,
                0x8000,
                std::ptr::null(),
                &mut unauth_out as *mut _ as *mut c_void,
                &mut unauth_len,
            ),
            TRUE
        );
        assert_eq!(std::ptr::read_unaligned(unauth_out as *const u32), 1);
        let mut enc_attrs_ptr: *mut u8 = std::ptr::null_mut();
        let mut enc_attrs_len = 0u32;
        assert_eq!(
            CryptEncodeObjectEx(
                X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
                48 as *const u8, // PKCS_ATTRIBUTES
                unauth_out as *const c_void,
                0x8000,
                std::ptr::null(),
                &mut enc_attrs_ptr as *mut _ as *mut c_void,
                &mut enc_attrs_len,
            ),
            TRUE
        );
        assert!(enc_attrs_len > 0 && !enc_attrs_ptr.is_null());
        crate::kernel32::memory::LocalFree(enc_attrs_ptr as *mut c_void);
        crate::kernel32::memory::LocalFree(unauth_out as *mut c_void);

        // Unknown struct type -> ERROR_FILE_NOT_FOUND
        let mut dummy_len = 0u32;
        assert_eq!(
            CryptDecodeObjectEx(
                X509_ASN_ENCODING,
                9999 as *const u8,
                oct_der.as_ptr(),
                oct_der.len() as u32,
                0,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut dummy_len,
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_FILE_NOT_FOUND);

        // CryptStringToBinaryA: CRYPT_STRING_PERCENTESCAPE (0x0E) + invalid flag -> ERROR_INVALID_DATA
        let pct = b"Hi%20There%21\0";
        let mut pout = [0u8; 32];
        let mut plen = 32u32;
        assert_eq!(
            CryptStringToBinaryA(
                pct.as_ptr(),
                0,
                CRYPT_STRING_PERCENTESCAPE,
                pout.as_mut_ptr(),
                &mut plen,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            TRUE
        );
        assert_eq!(&pout[..plen as usize], b"Hi There!");

        assert_eq!(
            CryptStringToBinaryA(
                pct.as_ptr(),
                0,
                0x9999,
                pout.as_mut_ptr(),
                &mut plen,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_DATA);
    }
}

#[test]
fn pfx_import_crypt_msg_verify_properties_and_crl_ctl() {
    use super::pkcs::{
        CMSG_CERT_COUNT_PARAM, CMSG_CONTENT_PARAM, CMSG_SIGNER_AUTH_ATTR_PARAM,
        CMSG_SIGNER_CERT_INFO_PARAM, CMSG_TYPE_PARAM,
    };
    use super::store::{
        CERT_FRIENDLY_NAME_PROP_ID, CERT_KEY_IDENTIFIER_PROP_ID, CERT_KEY_PROV_INFO_PROP_ID,
        CERT_MD5_HASH_PROP_ID, CERT_SHA1_HASH_PROP_ID, CERT_SHA256_HASH_PROP_ID,
        CERT_SUBJECT_NAME_MD5_HASH_PROP_ID,
    };
    use self::test_vectors::{
        PFX_CA_FIRST_NO_KEY_ID, PFX_PBES1_LEGACY, PFX_PBES2_AES256, PKCS7_SIGNED_DER,
    };

    unsafe {
        // 1. PFXImportCertStore: modern PBES2 AES-256-CBC + SHA-256 MAC
        let blob_aes = CryptDataBlob {
            cb_data: PFX_PBES2_AES256.len() as u32,
            pb_data: PFX_PBES2_AES256.as_ptr(),
        };
        let wrong_pw = crate::strings::to_wide_z("wrongpass");
        assert!(PFXImportCertStore(&blob_aes as *const _ as *const c_void, wrong_pw.as_ptr(), 0).is_null());
        assert_eq!(crate::teb::last_error(), 86); // ERROR_INVALID_PASSWORD

        let good_pw = crate::strings::to_wide_z("testpass123");
        let st_aes = PFXImportCertStore(&blob_aes as *const _ as *const c_void, good_pw.as_ptr(), 0);
        assert!(!st_aes.is_null(), "PBES2 AES-256 PFX import must succeed");
        let cert = CertEnumCertificatesInStore(st_aes, std::ptr::null_mut());
        assert!(!cert.is_null());

        // Verify CertGetCertificateContextProperty: friendlyName, SHA1, MD5, SHA256, SKI, KEY_PROV_INFO
        let mut fname_buf = [0u16; 64];
        let mut fname_cb = (fname_buf.len() * 2) as u32;
        assert_eq!(
            CertGetCertificateContextProperty(
                cert,
                CERT_FRIENDLY_NAME_PROP_ID,
                fname_buf.as_mut_ptr() as *mut c_void,
                &mut fname_cb,
            ),
            TRUE
        );
        assert_eq!(crate::strings::from_wide(fname_buf.as_ptr()), "PortFactory Friendly");

        let mut sha1 = [0u8; 20];
        let mut sha1_len = 20u32;
        assert_eq!(
            CertGetCertificateContextProperty(cert, CERT_SHA1_HASH_PROP_ID, sha1.as_mut_ptr() as *mut c_void, &mut sha1_len),
            TRUE
        );
        assert_eq!(sha1_len, 20);
        let mut md5 = [0u8; 16];
        let mut md5_len = 16u32;
        assert_eq!(
            CertGetCertificateContextProperty(cert, CERT_MD5_HASH_PROP_ID, md5.as_mut_ptr() as *mut c_void, &mut md5_len),
            TRUE
        );
        assert_eq!(md5_len, 16);
        let mut sha256 = [0u8; 32];
        let mut sha256_len = 32u32;
        assert_eq!(
            CertGetCertificateContextProperty(cert, CERT_SHA256_HASH_PROP_ID, sha256.as_mut_ptr() as *mut c_void, &mut sha256_len),
            TRUE
        );
        assert_eq!(sha256_len, 32);

        let mut ski_len = 0u32;
        assert_eq!(
            CertGetCertificateContextProperty(cert, CERT_KEY_IDENTIFIER_PROP_ID, std::ptr::null_mut(), &mut ski_len),
            TRUE
        );
        assert_eq!(ski_len, 20);

        let mut subj_md5_len = 0u32;
        assert_eq!(
            CertGetCertificateContextProperty(cert, CERT_SUBJECT_NAME_MD5_HASH_PROP_ID, std::ptr::null_mut(), &mut subj_md5_len),
            TRUE
        );
        assert_eq!(subj_md5_len, 16);

        // Private key was matched -> CERT_KEY_PROV_INFO_PROP_ID succeeds
        let mut kpi_len = 0u32;
        assert_eq!(
            CertGetCertificateContextProperty(cert, CERT_KEY_PROV_INFO_PROP_ID, std::ptr::null_mut(), &mut kpi_len),
            TRUE
        );
        assert!(kpi_len >= 48);
        assert!(lookup_cert_private_key_and_chain(cert).is_some());

        // CertVerifySubjectCertificateContext: self-signed cert verifies its own signature & validity
        let mut flags = 0x1 | 0x2 | 0x4; // SIG | TIME | REVOCATION
        assert_eq!(CertVerifySubjectCertificateContext(cert, cert, &mut flags), TRUE);
        assert_eq!(flags & 0x1, 0, "valid self-signed signature must clear SIGNATURE_FLAG");
        assert_eq!(flags & 0x2, 0, "valid cert time must clear TIME_VALIDITY_FLAG");
        assert_eq!(flags & 0x0001_0004, 0x0001_0004, "no CRL sets REVOCATION | NO_CRL");

        CertFreeCertificateContext(cert);
        CertCloseStore(st_aes, 0);

        // 2. PFXImportCertStore: legacy PBES1 3DES-CBC + RC2-40-CBC + SHA-1 MAC
        let blob_leg = CryptDataBlob {
            cb_data: PFX_PBES1_LEGACY.len() as u32,
            pb_data: PFX_PBES1_LEGACY.as_ptr(),
        };
        let leg_pw = crate::strings::to_wide_z("legacy456");
        let st_leg = PFXImportCertStore(&blob_leg as *const _ as *const c_void, leg_pw.as_ptr(), 0);
        assert!(!st_leg.is_null(), "Legacy PBES1 PFX import must succeed");
        let c_leg = CertEnumCertificatesInStore(st_leg, std::ptr::null_mut());
        assert!(!c_leg.is_null());
        let mut lf_buf = [0u16; 64];
        let mut lf_cb = (lf_buf.len() * 2) as u32;
        assert_eq!(
            CertGetCertificateContextProperty(c_leg, CERT_FRIENDLY_NAME_PROP_ID, lf_buf.as_mut_ptr() as *mut c_void, &mut lf_cb),
            TRUE
        );
        assert_eq!(crate::strings::from_wide(lf_buf.as_ptr()), "Legacy Friendly");
        assert!(lookup_cert_private_key_and_chain(c_leg).is_some());
        CertFreeCertificateContext(c_leg);
        CertCloseStore(st_leg, 0);

        // 2b. PFXImportCertStore: CA-first PFX with zero localKeyId bag attributes must match
        //     the private key to the leaf certificate by SPKI, never blindly to the first cert.
        let blob_noid = CryptDataBlob {
            cb_data: PFX_CA_FIRST_NO_KEY_ID.len() as u32,
            pb_data: PFX_CA_FIRST_NO_KEY_ID.as_ptr(),
        };
        let empty_pw = crate::strings::to_wide_z("");
        let st_noid = PFXImportCertStore(&blob_noid as *const _ as *const c_void, empty_pw.as_ptr(), 0);
        assert!(!st_noid.is_null(), "CA-first attribute-free PFX import must succeed");
        let c_first = CertEnumCertificatesInStore(st_noid, std::ptr::null_mut());
        assert!(!c_first.is_null(), "first cert (CA) must be present");
        let mut cn1 = [0u8; 64];
        CertGetNameStringA(c_first, 4, 0, std::ptr::null_mut(), cn1.as_mut_ptr(), cn1.len() as u32);
        assert_eq!(crate::strings::from_ansi(cn1.as_ptr()), "NoIdTestCA");
        assert!(
            lookup_cert_private_key_and_chain(c_first).is_none(),
            "CA cert must NOT receive leaf private key when localKeyId is omitted"
        );
        let mut ca_kpi_len = 0u32;
        assert_eq!(
            CertGetCertificateContextProperty(c_first, CERT_KEY_PROV_INFO_PROP_ID, std::ptr::null_mut(), &mut ca_kpi_len),
            FALSE
        );

        let c_second = CertEnumCertificatesInStore(st_noid, c_first);
        assert!(!c_second.is_null(), "second cert (leaf) must be present");
        let mut cn2 = [0u8; 64];
        CertGetNameStringA(c_second, 4, 0, std::ptr::null_mut(), cn2.as_mut_ptr(), cn2.len() as u32);
        assert_eq!(crate::strings::from_ansi(cn2.as_ptr()), "NoIdTestLeaf");
        let (leaf_chain, leaf_key) = lookup_cert_private_key_and_chain(c_second)
            .expect("leaf cert must match its private key by SPKI without localKeyId");
        assert_eq!(leaf_chain.len(), 2, "leaf chain should include leaf + CA from store");
        assert!(!leaf_key.is_empty());
        let mut leaf_kpi_len = 0u32;
        assert_eq!(
            CertGetCertificateContextProperty(c_second, CERT_KEY_PROV_INFO_PROP_ID, std::ptr::null_mut(), &mut leaf_kpi_len),
            TRUE
        );
        assert!(CertEnumCertificatesInStore(st_noid, c_second).is_null());
        CertCloseStore(st_noid, 0);

        // 3. CryptMsgOpenToDecode + chunked CryptMsgUpdate + CryptMsgGetParam
        let h_msg = CryptMsgOpenToDecode(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert!(!h_msg.is_null());
        let mid = PKCS7_SIGNED_DER.len() / 2;
        assert_eq!(CryptMsgUpdate(h_msg, PKCS7_SIGNED_DER.as_ptr(), mid as u32, FALSE), TRUE);
        assert_eq!(
            CryptMsgUpdate(
                h_msg,
                PKCS7_SIGNED_DER[mid..].as_ptr(),
                (PKCS7_SIGNED_DER.len() - mid) as u32,
                TRUE,
            ),
            TRUE
        );
        let mut mtype = 0u32;
        let mut mtype_len = 4u32;
        assert_eq!(
            CryptMsgGetParam(h_msg, CMSG_TYPE_PARAM, 0, &mut mtype as *mut _ as *mut c_void, &mut mtype_len),
            TRUE
        );
        assert_eq!(mtype, 2); // CMSG_SIGNED
        let mut ccnt = 0u32;
        let mut ccnt_len = 4u32;
        assert_eq!(
            CryptMsgGetParam(h_msg, CMSG_CERT_COUNT_PARAM, 0, &mut ccnt as *mut _ as *mut c_void, &mut ccnt_len),
            TRUE
        );
        assert_eq!(ccnt, 1);
        let mut content = [0u8; 64];
        let mut clen = 64u32;
        assert_eq!(
            CryptMsgGetParam(h_msg, CMSG_CONTENT_PARAM, 0, content.as_mut_ptr() as *mut c_void, &mut clen),
            TRUE
        );
        assert_eq!(&content[..clen as usize], b"Hello PKCS7 Signed World!");
        let mut si_len = 0u32;
        assert_eq!(
            CryptMsgGetParam(h_msg, CMSG_SIGNER_INFO_PARAM, 0, std::ptr::null_mut(), &mut si_len),
            TRUE
        );
        assert!(si_len > std::mem::size_of::<crate::misc::wintrust::CmsgSignerInfo>() as u32);
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
        let si_val = std::ptr::read_unaligned(
            si_buf.as_ptr() as *const crate::misc::wintrust::CmsgSignerInfo,
        );
        assert!(si_val.auth_attrs_count > 0, "CMSG_SIGNER_INFO_PARAM must include AuthAttrs");
        assert!(!si_val.auth_attrs_ptr.is_null());
        let first_attr_oid_ptr =
            std::ptr::read_unaligned(si_val.auth_attrs_ptr as *const *const u8);
        assert!(!first_attr_oid_ptr.is_null());
        assert!(!crate::strings::from_ansi(first_attr_oid_ptr).is_empty());

        let mut sci_len = 0u32;
        assert_eq!(
            CryptMsgGetParam(h_msg, CMSG_SIGNER_CERT_INFO_PARAM, 0, std::ptr::null_mut(), &mut sci_len),
            TRUE
        );
        assert!(sci_len >= std::mem::size_of::<CertInfo>() as u32);
        let mut aa_len = 0u32;
        assert_eq!(
            CryptMsgGetParam(h_msg, CMSG_SIGNER_AUTH_ATTR_PARAM, 0, std::ptr::null_mut(), &mut aa_len),
            TRUE
        );
        assert!(aa_len > 16);
        assert_eq!(CryptMsgClose(h_msg), TRUE);

        // Verify CMSG_SIGNER_INFO_PARAM and CMSG_SIGNER_UNAUTH_ATTR_PARAM preserve unauthenticated
        // attributes (e.g. countersignature 1.2.840.113549.1.9.6 under implicit tag 0xA1).
        let sha256_alg_der = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01]),
                encode_der_tlv(0x05, &[]),
            ]
            .concat(),
        );
        let rsa_alg_der = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01]),
                encode_der_tlv(0x05, &[]),
            ]
            .concat(),
        );
        let dn_der = encode_der_tlv(
            0x30,
            &encode_der_tlv(
                0x31,
                &encode_der_tlv(
                    0x30,
                    &[encode_der_tlv(0x06, &[0x55, 0x04, 0x03]), encode_der_tlv(0x0C, b"Test")].concat(),
                ),
            ),
        );
        let cs_attr = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x09, 0x06]),
                encode_der_tlv(0x31, &encode_der_tlv(0x04, &[0x11, 0x22, 0x33])),
            ]
            .concat(),
        );
        let signer_with_unauth = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x02, &[0x01]),
                encode_der_tlv(0x30, &[dn_der.clone(), encode_der_tlv(0x02, &[0x07])].concat()),
                sha256_alg_der.clone(),
                rsa_alg_der,
                encode_der_tlv(0x04, &[0xAA, 0xBB]),
                encode_der_tlv(0xA1, &cs_attr),
            ]
            .concat(),
        );
        let sd_with_unauth = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x02]),
                encode_der_tlv(
                    0xA0,
                    &encode_der_tlv(
                        0x30,
                        &[
                            encode_der_tlv(0x02, &[0x01]),
                            encode_der_tlv(0x31, &sha256_alg_der),
                            encode_der_tlv(
                                0x30,
                                &encode_der_tlv(
                                    0x06,
                                    &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x01],
                                ),
                            ),
                            encode_der_tlv(0x31, &signer_with_unauth),
                        ]
                        .concat(),
                    ),
                ),
            ]
            .concat(),
        );
        let h_umsg = CryptMsgOpenToDecode(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(
            CryptMsgUpdate(h_umsg, sd_with_unauth.as_ptr(), sd_with_unauth.len() as u32, TRUE),
            TRUE
        );
        let mut usi_len = 0u32;
        assert_eq!(
            CryptMsgGetParam(h_umsg, CMSG_SIGNER_INFO_PARAM, 0, std::ptr::null_mut(), &mut usi_len),
            TRUE
        );
        let mut usi_buf = vec![0u8; usi_len as usize];
        assert_eq!(
            CryptMsgGetParam(
                h_umsg,
                CMSG_SIGNER_INFO_PARAM,
                0,
                usi_buf.as_mut_ptr() as *mut c_void,
                &mut usi_len,
            ),
            TRUE
        );
        let usi_val = std::ptr::read_unaligned(
            usi_buf.as_ptr() as *const crate::misc::wintrust::CmsgSignerInfo,
        );
        assert_eq!(usi_val.unauth_attrs_count, 1);
        assert!(!usi_val.unauth_attrs_ptr.is_null());
        let ua_oid_ptr = std::ptr::read_unaligned(usi_val.unauth_attrs_ptr as *const *const u8);
        assert_eq!(crate::strings::from_ansi(ua_oid_ptr), "1.2.840.113549.1.9.6");
        let mut ua_len = 0u32;
        assert_eq!(
            CryptMsgGetParam(
                h_umsg,
                super::pkcs::CMSG_SIGNER_UNAUTH_ATTR_PARAM,
                0,
                std::ptr::null_mut(),
                &mut ua_len,
            ),
            TRUE
        );
        assert!(ua_len >= 16);
        assert_eq!(CryptMsgClose(h_umsg), TRUE);

        // 3b. CryptMsgOpenToDecode with CMSG_STREAM_INFO invokes pfnStreamOutput and propagates failure
        #[repr(C)]
        struct TestStreamInfo {
            cb_content: u32,
            pfn_stream_output: usize,
            pv_arg: *mut c_void,
        }
        unsafe extern "C" fn collect_stream_cb(
            pv_arg: *mut c_void,
            pb_data: *const u8,
            cb_data: u32,
            f_final: BOOL,
        ) -> BOOL {
            let out = &mut *(pv_arg as *mut (Vec<u8>, bool));
            if !pb_data.is_null() && cb_data > 0 {
                out.0.extend_from_slice(std::slice::from_raw_parts(pb_data, cb_data as usize));
            }
            if f_final != 0 {
                out.1 = true;
            }
            TRUE
        }
        unsafe extern "C" fn fail_stream_cb(
            _pv_arg: *mut c_void,
            _pb_data: *const u8,
            _cb_data: u32,
            _f_final: BOOL,
        ) -> BOOL {
            crate::teb::set_last_error(0x8009_0020);
            FALSE
        }
        let mut streamed: (Vec<u8>, bool) = (Vec::new(), false);
        let mut sinfo_ok = TestStreamInfo {
            cb_content: 0xFFFF_FFFF,
            pfn_stream_output: collect_stream_cb as *const () as usize,
            pv_arg: &mut streamed as *mut _ as *mut c_void,
        };
        let h_smsg = CryptMsgOpenToDecode(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            0,
            0,
            0,
            std::ptr::null_mut(),
            &mut sinfo_ok as *mut _ as *mut c_void,
        );
        assert!(!h_smsg.is_null());
        assert_eq!(
            CryptMsgUpdate(h_smsg, PKCS7_SIGNED_DER.as_ptr(), PKCS7_SIGNED_DER.len() as u32, TRUE),
            TRUE
        );
        assert!(streamed.1, "pfnStreamOutput must receive fFinal=TRUE");
        assert_eq!(streamed.0.as_slice(), b"Hello PKCS7 Signed World!");
        assert_eq!(CryptMsgClose(h_smsg), TRUE);

        let mut sinfo_fail = TestStreamInfo {
            cb_content: 0xFFFF_FFFF,
            pfn_stream_output: fail_stream_cb as *const () as usize,
            pv_arg: std::ptr::null_mut(),
        };
        let h_fmsg = CryptMsgOpenToDecode(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            0,
            0,
            0,
            std::ptr::null_mut(),
            &mut sinfo_fail as *mut _ as *mut c_void,
        );
        assert!(!h_fmsg.is_null());
        assert_eq!(
            CryptMsgUpdate(h_fmsg, PKCS7_SIGNED_DER.as_ptr(), PKCS7_SIGNED_DER.len() as u32, TRUE),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), 0x8009_0020);
        assert_eq!(CryptMsgClose(h_fmsg), TRUE);

        // 3c. Headerless SignedData decoding when caller supplies dwMsgType = CMSG_SIGNED (2)
        let (ci_item, _) = super::asn1::parse_der_item(PKCS7_SIGNED_DER).unwrap();
        let ci_kids = super::asn1::der_item_children(ci_item.val).unwrap();
        let (bare_sd, _) = super::asn1::parse_der_item(ci_kids[1].val).unwrap();
        let h_bare = CryptMsgOpenToDecode(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            0,
            2, // CMSG_SIGNED
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert!(!h_bare.is_null());
        assert_eq!(
            CryptMsgUpdate(h_bare, bare_sd.raw.as_ptr(), bare_sd.raw.len() as u32, TRUE),
            TRUE
        );
        let mut bare_type = 0u32;
        let mut bare_type_len = 4u32;
        assert_eq!(
            CryptMsgGetParam(
                h_bare,
                CMSG_TYPE_PARAM,
                0,
                &mut bare_type as *mut _ as *mut c_void,
                &mut bare_type_len,
            ),
            TRUE
        );
        assert_eq!(bare_type, 2);
        let mut bare_content = [0u8; 64];
        let mut bare_clen = 64u32;
        assert_eq!(
            CryptMsgGetParam(
                h_bare,
                CMSG_CONTENT_PARAM,
                0,
                bare_content.as_mut_ptr() as *mut c_void,
                &mut bare_clen,
            ),
            TRUE
        );
        assert_eq!(&bare_content[..bare_clen as usize], b"Hello PKCS7 Signed World!");
        assert_eq!(CryptMsgClose(h_bare), TRUE);

        // 4. CryptVerifyMessageSignature (valid + tampered rejection)
        #[repr(C)]
        struct VerifyPara {
            cb_size: u32,
            encoding: u32,
            h_prov: usize,
            pfn: usize,
            arg: *mut c_void,
        }
        let vp = VerifyPara {
            cb_size: std::mem::size_of::<VerifyPara>() as u32,
            encoding: X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            h_prov: 0,
            pfn: 0,
            arg: std::ptr::null_mut(),
        };
        let mut dec = [0u8; 64];
        let mut dec_len = 64u32;
        let mut signer_cert: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp as *const _ as *const c_void,
                0,
                PKCS7_SIGNED_DER.as_ptr(),
                PKCS7_SIGNED_DER.len() as u32,
                dec.as_mut_ptr(),
                &mut dec_len,
                &mut signer_cert,
            ),
            TRUE
        );
        assert_eq!(&dec[..dec_len as usize], b"Hello PKCS7 Signed World!");
        assert!(!signer_cert.is_null());

        // 4a. CertSetCertificateContextProperty + CertGetCertificateContextProperty for CERT_KEY_PROV_INFO_PROP_ID
        #[repr(C)]
        struct TestKeyProvParam {
            dw_param: u32,
            pb_data: *const u8,
            cb_data: u32,
            dw_flags: u32,
        }
        #[repr(C)]
        struct TestKeyProvInfo {
            pwsz_container_name: *const u16,
            pwsz_prov_name: *const u16,
            dw_prov_type: u32,
            dw_flags: u32,
            c_prov_param: u32,
            rg_prov_param: *const TestKeyProvParam,
            dw_key_spec: u32,
        }
        let cont_w = crate::strings::to_wide_z("MyKeyContainer");
        let prov_w = crate::strings::to_wide_z("MyCryptoProvider");
        let param_bytes = [0xDEu8, 0xAD, 0xBE, 0xEF];
        let kpp = TestKeyProvParam {
            dw_param: 7,
            pb_data: param_bytes.as_ptr(),
            cb_data: param_bytes.len() as u32,
            dw_flags: 0,
        };
        let kpi_in = TestKeyProvInfo {
            pwsz_container_name: cont_w.as_ptr(),
            pwsz_prov_name: prov_w.as_ptr(),
            dw_prov_type: 24,
            dw_flags: 0x20,
            c_prov_param: 1,
            rg_prov_param: &kpp,
            dw_key_spec: 2,
        };
        assert_eq!(
            CertSetCertificateContextProperty(
                signer_cert,
                CERT_KEY_PROV_INFO_PROP_ID,
                0,
                &kpi_in as *const _ as *const c_void,
            ),
            TRUE
        );
        let mut kpi_sz = 0u32;
        assert_eq!(
            CertGetCertificateContextProperty(
                signer_cert,
                CERT_KEY_PROV_INFO_PROP_ID,
                std::ptr::null_mut(),
                &mut kpi_sz,
            ),
            TRUE
        );
        let mut kpi_buf = vec![0u8; kpi_sz as usize];
        assert_eq!(
            CertGetCertificateContextProperty(
                signer_cert,
                CERT_KEY_PROV_INFO_PROP_ID,
                kpi_buf.as_mut_ptr() as *mut c_void,
                &mut kpi_sz,
            ),
            TRUE
        );
        let kpi_out = std::ptr::read_unaligned(kpi_buf.as_ptr() as *const TestKeyProvInfo);
        assert_eq!(crate::strings::from_wide(kpi_out.pwsz_container_name), "MyKeyContainer");
        assert_eq!(crate::strings::from_wide(kpi_out.pwsz_prov_name), "MyCryptoProvider");
        assert_eq!(kpi_out.dw_prov_type, 24);
        assert_eq!(kpi_out.dw_flags, 0x20);
        assert_eq!(kpi_out.dw_key_spec, 2);
        assert_eq!(kpi_out.c_prov_param, 1);
        let out_param = std::ptr::read_unaligned(kpi_out.rg_prov_param);
        assert_eq!(out_param.dw_param, 7);
        assert_eq!(
            std::slice::from_raw_parts(out_param.pb_data, out_param.cb_data as usize),
            &param_bytes
        );

        // 4b. CryptVerifyMessageSignature honors pfnGetSignerCertificate callback
        unsafe extern "C" fn reject_signer_cb(
            _arg: *mut c_void,
            _enc: u32,
            _signer_id: *const c_void,
            _store: *mut c_void,
        ) -> *mut c_void {
            std::ptr::null_mut()
        }
        unsafe extern "C" fn accept_signer_cb(
            arg: *mut c_void,
            enc: u32,
            signer_id: *const c_void,
            msg_store: *mut c_void,
        ) -> *mut c_void {
            assert_eq!(arg as usize, 0xCAFE);
            CertGetSubjectCertificateFromStore(msg_store, enc, signer_id)
        }
        let vp_reject = VerifyPara {
            cb_size: std::mem::size_of::<VerifyPara>() as u32,
            encoding: X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            h_prov: 0,
            pfn: reject_signer_cb as *const () as usize,
            arg: std::ptr::null_mut(),
        };
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp_reject as *const _ as *const c_void,
                0,
                PKCS7_SIGNED_DER.as_ptr(),
                PKCS7_SIGNED_DER.len() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), 0x8009_100E); // CRYPT_E_SIGNER_NOT_FOUND

        let vp_accept = VerifyPara {
            cb_size: std::mem::size_of::<VerifyPara>() as u32,
            encoding: X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            h_prov: 0,
            pfn: accept_signer_cb as *const () as usize,
            arg: 0xCAFEusize as *mut c_void,
        };
        let mut cb_signer_out: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp_accept as *const _ as *const c_void,
                0,
                PKCS7_SIGNED_DER.as_ptr(),
                PKCS7_SIGNED_DER.len() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut cb_signer_out,
            ),
            TRUE
        );
        assert!(!cb_signer_out.is_null());
        CertFreeCertificateContext(cb_signer_out);
        CertFreeCertificateContext(signer_cert);

        // Tamper signature byte -> fails with NTE_BAD_SIGNATURE
        let mut bad_sig = PKCS7_SIGNED_DER.to_vec();
        let last = bad_sig.len() - 1;
        bad_sig[last] ^= 0xFF;
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp as *const _ as *const c_void,
                0,
                bad_sig.as_ptr(),
                bad_sig.len() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), 0x8009_0006);

        // 5. Real CRL & CTL context lifecycle (reject malformed CRLs; require decoded pCrlInfo)
        let minimal_seq = encode_der_tlv(0x30, &[]);
        assert!(
            CertCreateCRLContext(X509_ASN_ENCODING, minimal_seq.as_ptr(), minimal_seq.len() as u32)
                .is_null(),
            "CertCreateCRLContext must reject malformed DER sequence with NULL"
        );
        let crl_alg = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B]),
                encode_der_tlv(0x05, &[]),
            ]
            .concat(),
        );
        let crl_revoked = encode_der_tlv(
            0x30,
            &encode_der_tlv(
                0x30,
                &[encode_der_tlv(0x02, &[0x42]), encode_der_tlv(0x17, b"260101000000Z")].concat(),
            ),
        );
        let crl_tbs = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x02, &[0x01]),
                crl_alg.clone(),
                dn_der,
                encode_der_tlv(0x17, b"260101000000Z"),
                crl_revoked,
            ]
            .concat(),
        );
        let valid_crl_der = encode_der_tlv(
            0x30,
            &[crl_tbs, crl_alg, encode_der_tlv(0x03, &[0x00, 0xDE, 0xAD])].concat(),
        );
        let crl = CertCreateCRLContext(
            X509_ASN_ENCODING,
            valid_crl_der.as_ptr(),
            valid_crl_der.len() as u32,
        );
        assert!(!crl.is_null());
        // CRL_CONTEXT.pCrlInfo is at offset 24 (after dwCertEncodingType + pbCrlEncoded + cbCrlEncoded)
        let p_crl_info = std::ptr::read_unaligned((crl as *const u8).add(24) as *const *const u8);
        assert!(!p_crl_info.is_null(), "CertCreateCRLContext must populate pCrlInfo");
        let c_crl_entry = std::ptr::read_unaligned(p_crl_info.add(64) as *const u32);
        assert_eq!(c_crl_entry, 1);
        let crl_dup = CertDuplicateCRLContext(crl);
        assert_eq!(crl_dup, crl);
        assert_eq!(CertFreeCRLContext(crl_dup), TRUE);
        assert_eq!(CertFreeCRLContext(crl), TRUE);

        // Invalid/non-CTL input MUST fail CertCreateCTLContext
        assert!(CertCreateCTLContext(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            minimal_seq.as_ptr(),
            minimal_seq.len() as u32,
        )
        .is_null());

        let ctl_der = self::test_vectors::CATALOG_CTL_SIGNED_DER;
        let ctl = CertCreateCTLContext(
            X509_ASN_ENCODING | PKCS_7_ASN_ENCODING,
            ctl_der.as_ptr(),
            ctl_der.len() as u32,
        );
        assert!(!ctl.is_null());
        let ctl_ctx = &*(ctl as *const super::store::CtlContext);
        assert!(!ctl_ctx.p_ctl_info.is_null(), "pCtlInfo must be decoded");
        assert!(!ctl_ctx.h_crypt_msg.is_null(), "hCryptMsg must be opened");
        assert!(ctl_ctx.cb_ctl_content > 0 && ctl_ctx.cb_ctl_content < ctl_ctx.cb_ctl_encoded);
        // CTL_INFO: SubjectUsage.cUsageIdentifier @ +8, cCTLEntry @ +96
        let usage_cnt = std::ptr::read_unaligned((ctl_ctx.p_ctl_info as *const u8).add(8) as *const u32);
        let entry_cnt = std::ptr::read_unaligned((ctl_ctx.p_ctl_info as *const u8).add(96) as *const u32);
        assert_eq!(usage_cnt, 1);
        assert_eq!(entry_cnt, 1);
        // hCryptMsg is a live message handle supporting CryptMsgGetParam(CMSG_SIGNER_COUNT_PARAM)
        let mut scnt = 0u32;
        let mut scnt_len = 4u32;
        assert_eq!(
            CryptMsgGetParam(
                ctl_ctx.h_crypt_msg,
                5, // CMSG_SIGNER_COUNT_PARAM
                0,
                &mut scnt as *mut _ as *mut c_void,
                &mut scnt_len,
            ),
            TRUE
        );
        assert_eq!(scnt, 1);

        // RFC 5652 §5.2.1: CMSG_CONTENT_PARAM & CryptVerifyMessageSignature unwrap the CMS
        // OCTET STRING for non-DATA content types (szOID_CTL -> raw 0x30 CertificateTrustList DER).
        let mut ctl_c_len = 0u32;
        assert_eq!(
            CryptMsgGetParam(
                ctl_ctx.h_crypt_msg,
                CMSG_CONTENT_PARAM,
                0,
                std::ptr::null_mut(),
                &mut ctl_c_len,
            ),
            TRUE
        );
        assert_eq!(ctl_c_len, ctl_ctx.cb_ctl_content);
        let mut ctl_c_buf = vec![0u8; ctl_c_len as usize];
        assert_eq!(
            CryptMsgGetParam(
                ctl_ctx.h_crypt_msg,
                CMSG_CONTENT_PARAM,
                0,
                ctl_c_buf.as_mut_ptr() as *mut c_void,
                &mut ctl_c_len,
            ),
            TRUE
        );
        assert_eq!(ctl_c_buf[0], 0x30, "CMSG_CONTENT_PARAM must unwrap CMS OCTET STRING");

        let mut ctl_dec = vec![0u8; ctl_ctx.cb_ctl_content as usize];
        let mut ctl_dec_len = ctl_dec.len() as u32;
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp as *const _ as *const c_void,
                0,
                ctl_der.as_ptr(),
                ctl_der.len() as u32,
                ctl_dec.as_mut_ptr(),
                &mut ctl_dec_len,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        assert_eq!(ctl_dec_len, ctl_ctx.cb_ctl_content);
        assert_eq!(ctl_dec[0], 0x30);

        // RFC 5652 §5.6: mutating encapContentInfo.eContentType (leaving signedAttrs + signature
        // intact) MUST fail CryptVerifyMessageSignature with NTE_BAD_SIGNATURE.
        let oid_ctl: &[u8] = &[0x2B, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x0A, 0x01];
        let oid_data: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x07, 0x01];
        let mut bad_ct_ctl = ctl_der.to_vec();
        let ct_pos = bad_ct_ctl
            .windows(oid_ctl.len())
            .position(|w| w == oid_ctl)
            .unwrap();
        bad_ct_ctl[ct_pos..ct_pos + oid_ctl.len()].copy_from_slice(oid_data);
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp as *const _ as *const c_void,
                0,
                bad_ct_ctl.as_ptr(),
                bad_ct_ctl.len() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), 0x8009_0006);

        // RFC 5652 §5.3: attribute-free SignedData passes CryptVerifyMessageSignature for id-data,
        // but MUST fail when encapContentInfo.eContentType is relabeled to non-DATA (szOID_CTL).
        let noattr_der = self::test_vectors::PKCS7_NOATTR_DATA_DER;
        let mut noattr_out = [0u8; 64];
        let mut noattr_len = noattr_out.len() as u32;
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp as *const _ as *const c_void,
                0,
                noattr_der.as_ptr(),
                noattr_der.len() as u32,
                noattr_out.as_mut_ptr(),
                &mut noattr_len,
                std::ptr::null_mut(),
            ),
            TRUE
        );
        assert_eq!(&noattr_out[..noattr_len as usize], b"noattr-data-payload");
        let mut relabeled_noattr = noattr_der.to_vec();
        let dpos = relabeled_noattr
            .windows(oid_data.len())
            .position(|w| w == oid_data)
            .unwrap();
        relabeled_noattr[dpos..dpos + oid_data.len()].copy_from_slice(oid_ctl);
        assert_eq!(
            CryptVerifyMessageSignature(
                &vp as *const _ as *const c_void,
                0,
                relabeled_noattr.as_ptr(),
                relabeled_noattr.len() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), 0x8009_0006);

        let ctl_dup = CertDuplicateCTLContext(ctl);
        assert_eq!(ctl_dup, ctl);
        assert_eq!(CertFreeCTLContext(ctl_dup), TRUE);
        assert_eq!(CertFreeCTLContext(ctl), TRUE);
    }
}

#[test]
fn macos_trust_store_and_extended_chain_policies() {
    use super::chain::{
        CERT_CHAIN_POLICY_BASIC_CONSTRAINTS, CERT_CHAIN_POLICY_EV,
        CERT_CHAIN_POLICY_MICROSOFT_ROOT, CERT_CHAIN_POLICY_NT_AUTH,
    };
    #[cfg(target_os = "macos")]
    {
        let sys_roots = super::chain::macos_system_roots_der();
        assert!(!sys_roots.is_empty(), "macOS Security.framework must return system root certs");
    }
    unsafe {
        // HCCE_LOCAL_MACHINE pseudo-handle (1) is a valid no-op on CertFreeCertificateChainEngine.
        CertFreeCertificateChainEngine(1usize as *mut c_void);

        // Build a chain from the PFX self-signed CA cert (which has BasicConstraints cA=TRUE)
        let blob = CryptDataBlob {
            cb_data: self::test_vectors::PFX_PBES2_AES256.len() as u32,
            pb_data: self::test_vectors::PFX_PBES2_AES256.as_ptr(),
        };
        let pw = crate::strings::to_wide_z("testpass123");
        let st = PFXImportCertStore(&blob as *const _ as *const c_void, pw.as_ptr(), 0);
        let cert = CertEnumCertificatesInStore(st, std::ptr::null_mut());
        let mut chain: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            CertGetCertificateChain(
                std::ptr::null_mut(),
                cert,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                &mut chain,
            ),
            TRUE
        );
        let dup = CertDuplicateCertificateChain(chain);
        assert_eq!(dup, chain);

        // BASIC_CONSTRAINTS with END_ENTITY flag fails on CA cert (TRUST_E_BASIC_CONSTRAINTS)
        let para_ee = CertChainPolicyPara {
            cb_size: std::mem::size_of::<CertChainPolicyPara>() as u32,
            dw_flags: 0x4000_0000, // BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG
            pv_extra_policy_para: std::ptr::null(),
        };
        let mut status = CertChainPolicyStatus {
            cb_size: std::mem::size_of::<CertChainPolicyStatus>() as u32,
            dw_error: 0,
            l_chain_index: -1,
            l_element_index: -1,
            pv_extra_policy_status: std::ptr::null_mut(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_BASIC_CONSTRAINTS as *const u8,
                chain,
                &para_ee as *const _ as *const c_void,
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, 0x8009_6019);

        // BASIC_CONSTRAINTS with CA flag passes on CA cert
        let para_ca = CertChainPolicyPara {
            cb_size: std::mem::size_of::<CertChainPolicyPara>() as u32,
            dw_flags: 0x8000_0000,
            pv_extra_policy_para: std::ptr::null(),
        };
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_BASIC_CONSTRAINTS as *const u8,
                chain,
                &para_ca as *const _ as *const c_void,
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, 0);

        // MICROSOFT_ROOT fails with CERT_E_UNTRUSTEDROOT on non-Microsoft cert even when trusted
        // by an exclusive root engine, AND NT_AUTH enforces Enterprise NTAuth issuer authorization
        // (returning CERT_E_UNTRUSTEDCA for ordinary trusted CAs until explicitly added to NTAUTH).
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_MICROSOFT_ROOT as *const u8,
                chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, CERT_E_UNTRUSTEDROOT);

        // Build a trusted chain with an exclusive root engine trusting `cert` itself
        let excl_store = CertOpenStore(
            super::store::CERT_STORE_PROV_MEMORY as *const u8,
            0,
            0,
            0,
            std::ptr::null(),
        );
        assert_eq!(
            CertAddCertificateContextToStore(excl_store, cert, 4, std::ptr::null_mut()),
            TRUE
        );
        let mut engine_cfg = [0u8; 0x60];
        std::ptr::write_unaligned(engine_cfg.as_mut_ptr() as *mut u32, engine_cfg.len() as u32);
        // hExclusiveRoot @ +0x40 in CERT_CHAIN_ENGINE_CONFIG (Win64)
        std::ptr::write_unaligned(engine_cfg.as_mut_ptr().add(0x40) as *mut *mut c_void, excl_store);
        let mut excl_engine: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            CertCreateCertificateChainEngine(
                engine_cfg.as_ptr() as *const c_void,
                &mut excl_engine,
            ),
            TRUE
        );
        let mut trusted_chain: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            CertGetCertificateChain(
                excl_engine,
                cert,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                &mut trusted_chain,
            ),
            TRUE
        );

        // Trusted non-EV cert MUST fail CERT_CHAIN_POLICY_EV with CERT_E_WRONG_USAGE
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_EV as *const u8,
                trusted_chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, CERT_E_WRONG_USAGE);

        // Enforce CA constraints on intermediates: CA cert accepted, end-entity (no cA=TRUE) rejected
        let cert_cc = std::ptr::read_unaligned(cert as *const crate::misc::wintrust::CertContext);
        let ca_der = std::slice::from_raw_parts(cert_cc.pb_cert_encoded, cert_cc.cb_cert_encoded as usize);
        assert!(super::chain::intermediate_is_valid_ca(ca_der, 0, None));
        // CATALOG_CTL_SIGNED_DER signer cert has no BasicConstraints CA:TRUE -> MUST be rejected as intermediate
        let ctl_msg = CryptMsgOpenToDecode(X509_ASN_ENCODING | PKCS_7_ASN_ENCODING, 0, 0, 0, std::ptr::null_mut(), std::ptr::null_mut());
        assert_eq!(CryptMsgUpdate(ctl_msg, self::test_vectors::CATALOG_CTL_SIGNED_DER.as_ptr(), self::test_vectors::CATALOG_CTL_SIGNED_DER.len() as u32, TRUE), TRUE);
        let mut ee_len = 0u32;
        assert_eq!(CryptMsgGetParam(ctl_msg, 12 /* CMSG_CERT_PARAM */, 0, std::ptr::null_mut(), &mut ee_len), TRUE);
        let mut ee_der = vec![0u8; ee_len as usize];
        assert_eq!(CryptMsgGetParam(ctl_msg, 12, 0, ee_der.as_mut_ptr() as *mut c_void, &mut ee_len), TRUE);
        assert!(!super::chain::intermediate_is_valid_ca(&ee_der, 0, None));
        assert_eq!(CryptMsgClose(ctl_msg), TRUE);

        // Intermediate CA with NameConstraints (2.5.29.30) MUST be rejected by fallback validator
        // so WebPKI / SecTrust name-constraint failures are never overridden.
        let alg = encode_der_tlv(
            0x30,
            &[0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B, 0x05, 0x00],
        );
        let dn = encode_der_tlv(
            0x30,
            &encode_der_tlv(
                0x31,
                &encode_der_tlv(0x30, &[0x06, 0x03, 0x55, 0x04, 0x03, 0x0C, 0x02, b'N', b'C']),
            ),
        );
        let validity = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x17, b"200101000000Z"),
                encode_der_tlv(0x17, b"400101000000Z"),
            ]
            .concat(),
        );
        let spki = encode_der_tlv(0x30, &[alg.clone(), encode_der_tlv(0x03, &[0x00, 0x01])].concat());
        let bc_ext = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x55, 0x1D, 0x13]),
                encode_der_tlv(0x01, &[0xFF]),
                encode_der_tlv(0x04, &encode_der_tlv(0x30, &encode_der_tlv(0x01, &[0xFF]))),
            ]
            .concat(),
        );
        let nc_ext = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0x06, &[0x55, 0x1D, 0x1E]), // NameConstraints 2.5.29.30
                encode_der_tlv(0x01, &[0xFF]),
                encode_der_tlv(0x04, &encode_der_tlv(0x30, &[])),
            ]
            .concat(),
        );
        let nc_tbs = encode_der_tlv(
            0x30,
            &[
                encode_der_tlv(0xA0, &encode_der_tlv(0x02, &[0x02])),
                encode_der_tlv(0x02, &[0x01]),
                alg.clone(),
                dn.clone(),
                validity,
                dn,
                spki,
                encode_der_tlv(0xA3, &encode_der_tlv(0x30, &[bc_ext, nc_ext].concat())),
            ]
            .concat(),
        );
        let nc_ca_cert = encode_der_tlv(
            0x30,
            &[nc_tbs, alg, encode_der_tlv(0x03, &[0x00, 0x01])].concat(),
        );
        assert!(!super::chain::intermediate_is_valid_ca(&nc_ca_cert, 0, None));
        assert!(!super::chain::verify_chain_signatures_to_roots(
            std::slice::from_ref(&nc_ca_cert),
            Some(std::slice::from_ref(&nc_ca_cert)),
            &[],
        ));

        // Unsupported specialized policies (SSL_F12=9, HPKP=10, THIRD_PARTY_ROOT=11, KEY_PIN=12)
        // must fail explicitly with FALSE + ERROR_FILE_NOT_FOUND rather than mis-casting parameters
        // or approving ordinary private-root trust.
        for unsupported_oid in [9usize, 10, 11, 12] {
            assert_eq!(
                CertVerifyCertificateChainPolicy(
                    unsupported_oid as *const u8,
                    trusted_chain,
                    std::ptr::null(),
                    &mut status as *mut _ as *mut c_void,
                ),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_FILE_NOT_FOUND);
        }

        // Trusted non-Microsoft root MUST still fail MICROSOFT_ROOT (checked by SPKI key, not name)
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_MICROSOFT_ROOT as *const u8,
                trusted_chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, CERT_E_UNTRUSTEDROOT);

        // Trusted ordinary CA NOT in Enterprise NTAuth store MUST fail NT_AUTH with CERT_E_UNTRUSTEDCA
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_NT_AUTH as *const u8,
                trusted_chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, super::chain::CERT_E_UNTRUSTEDCA);

        // Once the issuing CA is explicitly added to the Enterprise NTAUTH store, NT_AUTH succeeds!
        let ntauth_name = b"NTAUTH\0";
        let ntauth_st = CertOpenSystemStoreA(0, ntauth_name.as_ptr());
        assert!(!ntauth_st.is_null());
        assert_eq!(
            CertAddCertificateContextToStore(ntauth_st, cert, 4, std::ptr::null_mut()),
            TRUE
        );
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_NT_AUTH as *const u8,
                trusted_chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, 0);
        CertCloseStore(ntauth_st, 0);
        CertFreeCertificateChain(trusted_chain);
        CertFreeCertificateChainEngine(excl_engine);
        CertCloseStore(excl_store, 0);

        // Untrusted chain still fails NT_AUTH and EV with CERT_E_UNTRUSTEDROOT
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_NT_AUTH as *const u8,
                chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, CERT_E_UNTRUSTEDROOT);

        assert_eq!(
            CertVerifyCertificateChainPolicy(
                CERT_CHAIN_POLICY_EV as *const u8,
                chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            TRUE
        );
        assert_eq!(status.dw_error, CERT_E_UNTRUSTEDROOT);

        // Unknown policy OID returns FALSE with ERROR_FILE_NOT_FOUND
        assert_eq!(
            CertVerifyCertificateChainPolicy(
                999 as *const u8,
                chain,
                std::ptr::null(),
                &mut status as *mut _ as *mut c_void,
            ),
            FALSE
        );
        assert_eq!(crate::teb::last_error(), err::ERROR_FILE_NOT_FOUND);

        CertFreeCertificateChain(dup);
        CertFreeCertificateChain(chain);
        CertFreeCertificateContext(cert);
        CertCloseStore(st, 0);
    }
}
