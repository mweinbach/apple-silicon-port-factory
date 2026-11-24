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
        assert_eq!(count, webpki_roots::TLS_SERVER_ROOTS.len());
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
