//! crypt32.dll: certificate contexts, memory stores, chain verification,
//! SPKI DER encoding, DPAPI, and ASN.1 string/name helpers.
//!
//! Peer certificates returned by Schannel (`secur32.dll`) and certs created via
//! `CertCreateCertificateContext` / `CertOpenStore(CERT_STORE_PROV_MEMORY)` are
//! backed by Win64 ABI-exact `CERT_CONTEXT` / `CERT_INFO` / `CERT_CHAIN_CONTEXT`
//! structures and verified against `crate::net_tls` (`webpki-roots` or caller-supplied
//! exclusive root stores from `CertCreateCertificateChainEngine`).

mod asn1;
mod chain;
mod decode;
mod pkcs;
mod store;

use crate::types::*;
use std::ffi::c_void;
use std::sync::Arc;
use store::ERROR_CALL_NOT_IMPLEMENTED;

pub(crate) use asn1::{cert_eku_oids, dpapi_protect_bytes, dpapi_unprotect_bytes};
pub(crate) use chain::{
    check_chain_revocation_error, verify_cert_signed_by_der, verify_peer_chain_error,
};
pub(crate) use pkcs::verify_cmsg_signer_info_signature;
#[allow(unused_imports)]
pub(crate) use chain::{
    map_rustls_cert_error, CERT_E_CHAINING, CERT_E_CN_NO_MATCH, CERT_E_EXPIRED,
    CERT_E_INVALID_POLICY, CERT_E_REVOKED, CERT_E_UNTRUSTEDROOT, CERT_E_WRONG_USAGE,
    CRYPT_E_NO_REVOCATION_CHECK, CRYPT_E_REVOCATION_OFFLINE,
};
#[allow(unused_imports)]
pub(crate) use store::{
    attach_cert_private_key_pkcs8, copy_store_certs_with_properties,
    duplicate_or_create_cert_context_der, lookup_cert_private_key_and_chain,
    register_peer_cert_chain, store_certs_der, PeerCertVerifyResult,
};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

#[allow(dead_code)]
pub(crate) fn rustls_crypto_provider() -> Arc<rustls::crypto::CryptoProvider> {
    crate::net_tls::crypto_provider()
}

#[allow(dead_code)]
pub(crate) fn default_root_store() -> Arc<rustls::RootCertStore> {
    crate::net_tls::root_cert_store()
}

// Blocked on advapi32::CryptImportKey (owned by parallel branch stubs/crypto-cng-capi);
// SPKI -> Win32 PUBLICKEYBLOB conversion is implemented in `decode::spki_to_capi_public_key_blob`.
// @exports CryptImportPublicKeyInfo
failing!(BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED;
    CryptImportPublicKeyInfo);

// ---------------------------------------------------------------------------
// Certificate Contexts & Stores
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CertCreateCertificateContext(
    dw_cert_encoding_type: u32,
    pb_cert_encoded: *const u8,
    cb_cert_encoded: u32,
) -> *mut c_void {
    store::cert_create_certificate_context_impl(
        dw_cert_encoding_type,
        pb_cert_encoded,
        cb_cert_encoded,
    )
}

pub unsafe extern "win64" fn CertDuplicateCertificateContext(ctx: *mut c_void) -> *mut c_void {
    store::cert_duplicate_certificate_context_impl(ctx)
}

pub unsafe extern "win64" fn CertFreeCertificateContext(ctx: *mut c_void) -> BOOL {
    store::cert_free_certificate_context_impl(ctx)
}

pub unsafe extern "win64" fn CertOpenStore(
    lpsz_store_provider: *const u8,
    _dw_encoding_type: u32,
    _h_crypt_prov: usize,
    dw_flags: u32,
    pv_para: *const c_void,
) -> *mut c_void {
    store::cert_open_store_with_flags_impl(lpsz_store_provider, dw_flags, pv_para)
}

pub unsafe extern "win64" fn CertOpenSystemStoreA(
    _h_prov: usize,
    sz_subsystem_protocol: LPCSTR,
) -> *mut c_void {
    store::cert_open_system_store_a_impl(sz_subsystem_protocol)
}

pub unsafe extern "win64" fn CertOpenSystemStoreW(
    _h_prov: usize,
    sz_subsystem_protocol: LPCWSTR,
) -> *mut c_void {
    store::cert_open_system_store_w_impl(sz_subsystem_protocol)
}

pub unsafe extern "win64" fn CertDeleteCertificateFromStore(p_cert_context: *mut c_void) -> BOOL {
    store::cert_delete_certificate_from_store_impl(p_cert_context)
}

pub unsafe extern "win64" fn CertDuplicateStore(store_h: *mut c_void) -> *mut c_void {
    store::cert_duplicate_store_impl(store_h)
}

pub unsafe extern "win64" fn CertCloseStore(store_h: *mut c_void, _flags: u32) -> BOOL {
    store::cert_close_store_impl(store_h)
}

pub unsafe extern "win64" fn CertAddCertificateContextToStore(
    h_cert_store: *mut c_void,
    p_cert_context: *const c_void,
    dw_add_disposition: u32,
    pp_store_context: *mut *mut c_void,
) -> BOOL {
    store::cert_add_certificate_context_to_store_impl(
        h_cert_store,
        p_cert_context,
        dw_add_disposition,
        pp_store_context,
    )
}

pub unsafe extern "win64" fn CertAddEncodedCertificateToStore(
    h_cert_store: *mut c_void,
    dw_cert_encoding_type: u32,
    pb_cert_encoded: *const u8,
    cb_cert_encoded: u32,
    dw_add_disposition: u32,
    pp_cert_context: *mut *mut c_void,
) -> BOOL {
    store::cert_add_encoded_certificate_to_store_impl(
        h_cert_store,
        dw_cert_encoding_type,
        pb_cert_encoded,
        cb_cert_encoded,
        dw_add_disposition,
        pp_cert_context,
    )
}

pub unsafe extern "win64" fn CertEnumCertificatesInStore(
    h_cert_store: *mut c_void,
    p_prev_cert_context: *mut c_void,
) -> *mut c_void {
    store::cert_enum_certificates_in_store_impl(h_cert_store, p_prev_cert_context)
}

// ---------------------------------------------------------------------------
// Certificate Chain Engine & Policy Verification
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CertCreateCertificateChainEngine(
    p_config: *const c_void,
    ph_chain_engine: *mut *mut c_void,
) -> BOOL {
    chain::cert_create_certificate_chain_engine_impl(p_config, ph_chain_engine)
}

pub unsafe extern "win64" fn CertFreeCertificateChainEngine(engine: *mut c_void) {
    chain::cert_free_certificate_chain_engine_impl(engine);
}

pub unsafe extern "win64" fn CertGetCertificateChain(
    h_chain_engine: *mut c_void,
    p_cert_context: *const c_void,
    p_time: *const c_void,
    h_additional_store: *mut c_void,
    p_chain_para: *const c_void,
    dw_flags: u32,
    _pv_reserved: *mut c_void,
    pp_chain_context: *mut *mut c_void,
) -> BOOL {
    chain::cert_get_certificate_chain_impl(
        h_chain_engine,
        p_cert_context,
        p_time,
        h_additional_store,
        p_chain_para,
        dw_flags,
        pp_chain_context,
    )
}

pub unsafe extern "win64" fn CertDuplicateCertificateChain(chain_h: *const c_void) -> *mut c_void {
    chain::cert_duplicate_certificate_chain_impl(chain_h)
}

pub unsafe extern "win64" fn CertFreeCertificateChain(chain_h: *mut c_void) {
    chain::cert_free_certificate_chain_impl(chain_h);
}

pub unsafe extern "win64" fn CertVerifyCertificateChainPolicy(
    psz_policy_oid: *const u8,
    p_chain_context: *const c_void,
    p_policy_para: *const c_void,
    p_policy_status: *mut c_void,
) -> BOOL {
    chain::cert_verify_certificate_chain_policy_impl(
        psz_policy_oid,
        p_chain_context,
        p_policy_para,
        p_policy_status,
    )
}

// ---------------------------------------------------------------------------
// ASN.1 DER Encoding, Name Formatting, Messages & String<->Binary Conversions
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CryptEncodeObjectEx(
    dw_cert_encoding_type: u32,
    lpsz_struct_type: *const u8,
    pv_struct_info: *const c_void,
    dw_flags: u32,
    _p_encode_para: *const c_void,
    pv_encoded: *mut c_void,
    pcb_encoded: *mut u32,
) -> BOOL {
    asn1::crypt_encode_object_ex_impl(
        dw_cert_encoding_type,
        lpsz_struct_type,
        pv_struct_info,
        dw_flags,
        pv_encoded,
        pcb_encoded,
    )
}

pub unsafe extern "win64" fn CryptDecodeObject(
    dw_cert_encoding_type: u32,
    lpsz_struct_type: *const u8,
    pb_encoded: *const u8,
    cb_encoded: u32,
    dw_flags: u32,
    pv_struct_info: *mut c_void,
    pcb_struct_info: *mut u32,
) -> BOOL {
    decode::crypt_decode_object_impl(
        dw_cert_encoding_type,
        lpsz_struct_type,
        pb_encoded,
        cb_encoded,
        dw_flags,
        pv_struct_info,
        pcb_struct_info,
    )
}

pub unsafe extern "win64" fn CryptDecodeObjectEx(
    dw_cert_encoding_type: u32,
    lpsz_struct_type: *const u8,
    pb_encoded: *const u8,
    cb_encoded: u32,
    dw_flags: u32,
    p_decode_para: *const c_void,
    pv_struct_info: *mut c_void,
    pcb_struct_info: *mut u32,
) -> BOOL {
    decode::crypt_decode_object_ex_impl(
        dw_cert_encoding_type,
        lpsz_struct_type,
        pb_encoded,
        cb_encoded,
        dw_flags,
        p_decode_para,
        pv_struct_info,
        pcb_struct_info,
    )
}

pub unsafe extern "win64" fn CryptFindOIDInfo(
    dw_key_type: u32,
    pv_key: *const c_void,
    dw_group_id: u32,
) -> *mut c_void {
    decode::crypt_find_oid_info_impl(dw_key_type, pv_key, dw_group_id)
}

pub unsafe extern "win64" fn CertStrToNameA(
    dw_cert_encoding_type: u32,
    psz_x500: LPCSTR,
    dw_str_type: u32,
    pv_reserved: *mut c_void,
    pb_encoded: *mut u8,
    pcb_encoded: *mut u32,
    ppsz_error: *mut LPCSTR,
) -> BOOL {
    decode::cert_str_to_name_a_impl(
        dw_cert_encoding_type,
        psz_x500,
        dw_str_type,
        pv_reserved,
        pb_encoded,
        pcb_encoded,
        ppsz_error,
    )
}

pub unsafe extern "win64" fn CertStrToNameW(
    dw_cert_encoding_type: u32,
    psz_x500: LPCWSTR,
    dw_str_type: u32,
    pv_reserved: *mut c_void,
    pb_encoded: *mut u8,
    pcb_encoded: *mut u32,
    ppsz_error: *mut LPCWSTR,
) -> BOOL {
    decode::cert_str_to_name_w_impl(
        dw_cert_encoding_type,
        psz_x500,
        dw_str_type,
        pv_reserved,
        pb_encoded,
        pcb_encoded,
        ppsz_error,
    )
}

pub unsafe extern "win64" fn CertNameToStrW(
    _enc: u32,
    name: *const c_void,
    _ty: u32,
    buf: *mut u16,
    cch: u32,
) -> u32 {
    store::cert_name_to_str_w_impl(name, buf, cch)
}

pub unsafe extern "win64" fn CertNameToStrA(
    _enc: u32,
    name: *const c_void,
    _ty: u32,
    buf: *mut u8,
    cch: u32,
) -> u32 {
    store::cert_name_to_str_a_impl(name, buf, cch)
}

pub unsafe extern "win64" fn CryptMsgClose(h_msg: *mut c_void) -> BOOL {
    store::crypt_msg_close_impl(h_msg)
}

pub unsafe extern "win64" fn CertGetIssuerCertificateFromStore(
    h_cert_store: *mut c_void,
    p_subject_context: *const c_void,
    p_prev_issuer_context: *mut c_void,
    pdw_flags: *mut u32,
) -> *mut c_void {
    store::cert_get_issuer_certificate_from_store_impl(
        h_cert_store,
        p_subject_context,
        p_prev_issuer_context,
        pdw_flags,
    )
}

pub unsafe extern "win64" fn CertFindCertificateInStore(
    h_cert_store: *mut c_void,
    _dw_cert_encoding_type: u32,
    _dw_find_flags: u32,
    dw_find_type: u32,
    pv_find_para: *const c_void,
    p_prev_cert_context: *mut c_void,
) -> *mut c_void {
    store::cert_find_certificate_in_store_impl(
        h_cert_store,
        dw_find_type,
        pv_find_para,
        p_prev_cert_context,
    )
}

pub unsafe extern "win64" fn CertGetSubjectCertificateFromStore(
    h_cert_store: *mut c_void,
    _dw_cert_encoding_type: u32,
    p_cert_id: *const c_void,
) -> *mut c_void {
    store::cert_get_subject_certificate_from_store_impl(h_cert_store, p_cert_id)
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
    store::crypt_query_object_impl(
        dw_object_type,
        pv_object,
        dw_expected_content_type_flags,
        dw_expected_format_type_flags,
        pdw_msg_and_cert_encoding_type,
        pdw_content_type,
        pdw_format_type,
        ph_cert_store,
        ph_msg,
        ppv_context,
    )
}

pub unsafe extern "win64" fn CryptMsgGetParam(
    h_crypt_msg: *mut c_void,
    dw_param_type: u32,
    dw_index: u32,
    pv_data: *mut c_void,
    pcb_data: *mut u32,
) -> BOOL {
    store::crypt_msg_get_param_impl(h_crypt_msg, dw_param_type, dw_index, pv_data, pcb_data)
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

pub unsafe extern "win64" fn CertVerifyTimeValidity(
    p_time_to_verify: *const c_void,
    p_cert_info: *const c_void,
) -> i32 {
    chain::cert_verify_time_validity_impl(p_time_to_verify, p_cert_info)
}

pub unsafe extern "win64" fn CertVerifySubjectCertificateContext(
    p_subject: *const c_void,
    p_issuer: *const c_void,
    pdw_flags: *mut u32,
) -> BOOL {
    chain::cert_verify_subject_certificate_context_impl(p_subject, p_issuer, pdw_flags)
}

pub unsafe extern "win64" fn CertGetCertificateContextProperty(
    p_cert_context: *const c_void,
    dw_prop_id: u32,
    pv_data: *mut c_void,
    pcb_data: *mut u32,
) -> BOOL {
    store::cert_get_certificate_context_property_impl(p_cert_context, dw_prop_id, pv_data, pcb_data)
}

pub unsafe extern "win64" fn CertSetCertificateContextProperty(
    p_cert_context: *const c_void,
    dw_prop_id: u32,
    dw_flags: u32,
    pv_data: *const c_void,
) -> BOOL {
    store::cert_set_certificate_context_property_impl(p_cert_context, dw_prop_id, dw_flags, pv_data)
}

pub unsafe extern "win64" fn CertCreateCRLContext(
    dw_cert_encoding_type: u32,
    pb_crl_encoded: *const u8,
    cb_crl_encoded: u32,
) -> *mut c_void {
    store::cert_create_crl_context_impl(dw_cert_encoding_type, pb_crl_encoded, cb_crl_encoded)
}

pub unsafe extern "win64" fn CertDuplicateCRLContext(ctx: *const c_void) -> *mut c_void {
    store::cert_duplicate_crl_context_impl(ctx)
}

pub unsafe extern "win64" fn CertFreeCRLContext(ctx: *mut c_void) -> BOOL {
    store::cert_free_crl_context_impl(ctx)
}

pub unsafe extern "win64" fn CertCreateCTLContext(
    dw_msg_and_cert_encoding_type: u32,
    pb_ctl_encoded: *const u8,
    cb_ctl_encoded: u32,
) -> *mut c_void {
    store::cert_create_ctl_context_impl(dw_msg_and_cert_encoding_type, pb_ctl_encoded, cb_ctl_encoded)
}

pub unsafe extern "win64" fn CertDuplicateCTLContext(ctx: *const c_void) -> *mut c_void {
    store::cert_duplicate_ctl_context_impl(ctx)
}

pub unsafe extern "win64" fn CertFreeCTLContext(ctx: *mut c_void) -> BOOL {
    store::cert_free_ctl_context_impl(ctx)
}

pub unsafe extern "win64" fn CryptMsgOpenToDecode(
    dw_msg_encoding_type: u32,
    dw_flags: u32,
    dw_msg_type: u32,
    h_crypt_prov: usize,
    p_recipient_info: *mut c_void,
    p_stream_info: *mut c_void,
) -> *mut c_void {
    pkcs::crypt_msg_open_to_decode_impl(
        dw_msg_encoding_type,
        dw_flags,
        dw_msg_type,
        h_crypt_prov,
        p_recipient_info,
        p_stream_info,
    )
}

pub unsafe extern "win64" fn CryptMsgUpdate(
    h_crypt_msg: *mut c_void,
    pb_data: *const u8,
    cb_data: u32,
    f_final: BOOL,
) -> BOOL {
    pkcs::crypt_msg_update_impl(h_crypt_msg, pb_data, cb_data, f_final)
}

pub unsafe extern "win64" fn CryptVerifyMessageSignature(
    p_verify_para: *const c_void,
    dw_signer_index: u32,
    pb_signed_blob: *const u8,
    cb_signed_blob: u32,
    pb_decoded: *mut u8,
    pcb_decoded: *mut u32,
    pp_signer_cert: *mut *mut c_void,
) -> BOOL {
    pkcs::crypt_verify_message_signature_impl(
        p_verify_para,
        dw_signer_index,
        pb_signed_blob,
        cb_signed_blob,
        pb_decoded,
        pcb_decoded,
        pp_signer_cert,
    )
}

pub unsafe extern "win64" fn PFXImportCertStore(
    p_pfx: *const c_void,
    sz_password: LPCWSTR,
    dw_flags: u32,
) -> *mut c_void {
    pkcs::pfx_import_cert_store_impl(p_pfx, sz_password, dw_flags)
}

pub unsafe extern "win64" fn CertGetNameStringW(
    ctx: *mut c_void,
    ty: u32,
    flags: u32,
    para: *mut c_void,
    buf: *mut u16,
    cch: u32,
) -> u32 {
    store::cert_get_name_string_w_impl(ctx, ty, flags, para, buf, cch)
}

pub unsafe extern "win64" fn CertGetNameStringA(
    ctx: *mut c_void,
    ty: u32,
    flags: u32,
    para: *mut c_void,
    buf: *mut u8,
    cch: u32,
) -> u32 {
    store::cert_get_name_string_a_impl(ctx, ty, flags, para, buf, cch)
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
    asn1::crypt_string_to_binary_a_impl(
        psz_string,
        cch_string,
        dw_flags,
        pb_binary,
        pcb_binary,
        pdw_skip,
        pdw_flags,
    )
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
    asn1::crypt_string_to_binary_w_impl(
        psz_string,
        cch_string,
        dw_flags,
        pb_binary,
        pcb_binary,
        pdw_skip,
        pdw_flags,
    )
}

pub unsafe extern "win64" fn CryptBinaryToStringA(
    pb_binary: *const u8,
    cb_binary: u32,
    dw_flags: u32,
    psz_string: LPSTR,
    pcch_string: *mut u32,
) -> BOOL {
    asn1::crypt_binary_to_string_a_impl(pb_binary, cb_binary, dw_flags, psz_string, pcch_string)
}

pub unsafe extern "win64" fn CryptBinaryToStringW(
    pb_binary: *const u8,
    cb_binary: u32,
    dw_flags: u32,
    psz_string: LPWSTR,
    pcch_string: *mut u32,
) -> BOOL {
    asn1::crypt_binary_to_string_w_impl(pb_binary, cb_binary, dw_flags, psz_string, pcch_string)
}

pub unsafe extern "win64" fn CertCompareIntegerBlob(
    p_int1: *const c_void,
    p_int2: *const c_void,
) -> BOOL {
    asn1::cert_compare_integer_blob_impl(p_int1, p_int2)
}

pub unsafe extern "win64" fn CertCompareCertificateName(
    dw_cert_encoding_type: u32,
    p_cert_name1: *const c_void,
    p_cert_name2: *const c_void,
) -> BOOL {
    asn1::cert_compare_certificate_name_impl(dw_cert_encoding_type, p_cert_name1, p_cert_name2)
}

pub unsafe extern "win64" fn CertFindExtension(
    psz_obj_id: LPCSTR,
    c_extensions: u32,
    rg_extensions: *mut c_void,
) -> *mut c_void {
    asn1::cert_find_extension_impl(psz_obj_id, c_extensions, rg_extensions)
}

pub unsafe extern "win64" fn CryptProtectData(
    p_data_in: *const c_void,
    sz_data_descr: LPCWSTR,
    p_optional_entropy: *const c_void,
    pv_reserved: *mut c_void,
    p_prompt_struct: *const c_void,
    dw_flags: u32,
    p_data_out: *mut c_void,
) -> BOOL {
    asn1::crypt_protect_data_impl(
        p_data_in,
        sz_data_descr,
        p_optional_entropy,
        pv_reserved,
        p_prompt_struct,
        dw_flags,
        p_data_out,
    )
}

pub unsafe extern "win64" fn CryptUnprotectData(
    p_data_in: *const c_void,
    ppsz_data_descr: *mut LPWSTR,
    p_optional_entropy: *const c_void,
    pv_reserved: *mut c_void,
    p_prompt_struct: *const c_void,
    dw_flags: u32,
    p_data_out: *mut c_void,
) -> BOOL {
    asn1::crypt_unprotect_data_impl(
        p_data_in,
        ppsz_data_descr,
        p_optional_entropy,
        pv_reserved,
        p_prompt_struct,
        dw_flags,
        p_data_out,
    )
}

#[cfg(test)]
pub(crate) mod tests;
