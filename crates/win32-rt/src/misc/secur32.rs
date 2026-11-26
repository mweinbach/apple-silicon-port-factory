//! secur32.dll: Windows SSPI Schannel client-side TLS 1.2 / 1.3 implementation
//! backed by `crate::net_tls` (`rustls` + `ring` + embedded `webpki-roots`).
//!
//! Supports the full SSPI lifecycle used by `libcurl`, WinHTTP, and native Windows
//! HTTPS clients:
//! - `InitSecurityInterfaceA/W` (full `SecurityFunctionTableA/W` dispatch table)
//! - `AcquireCredentialsHandleA/W` (`SCHANNEL_CRED` v4 & `SCH_CREDENTIALS` v5)
//! - `InitializeSecurityContextA/W` (TLS 1.2 & TLS 1.3 handshake, SNI, ALPN,
//!   fragmented `SEC_E_INCOMPLETE_MESSAGE`, `SECBUFFER_EXTRA`, `SCHANNEL_SHUTDOWN`)
//! - `EncryptMessage` / `DecryptMessage` (stream framing, TLS 1.3 `NewSessionTicket`,
//!   `SECBUFFER_EXTRA`, `SEC_I_CONTEXT_EXPIRED`)
//! - `QueryContextAttributesA/W` (`STREAM_SIZES`, `REMOTE_CERT_CONTEXT`,
//!   `CONNECTION_INFO`, `APPLICATION_PROTOCOL`, `ISSUER_LIST_EX`, `SIZES`, `CIPHER_INFO`, `NAMES`)
//! - `ApplyControlToken`, `CompleteAuthToken`, `QuerySecurityPackageInfoA/W`,
//!   `QueryCredentialsAttributesA/W`, `DeleteSecurityContext`, `FreeCredentialsHandle`,
//!   `FreeContextBuffer`

mod schannel;

use schannel::{
    SecBufferDesc, SecHandle, SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION, SEC_E_INVALID_HANDLE,
    SEC_E_OK, SEC_E_SECPKG_NOT_FOUND, SEC_E_UNSUPPORTED_FUNCTION,
};
use std::ffi::c_void;
use std::sync::OnceLock;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Exported Win64 SSPI APIs
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn AcquireCredentialsHandleA(
    psz_principal: *const u8,
    psz_package: *const u8,
    f_credential_use: u32,
    _pv_logon_id: *mut c_void,
    p_auth_data: *mut c_void,
    _p_get_key_fn: *mut c_void,
    _pv_get_key_argument: *mut c_void,
    ph_credential: *mut c_void,
    pts_expiry: *mut i64,
) -> i32 {
    if psz_package.is_null() {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    let principal = if psz_principal.is_null() {
        String::new()
    } else {
        crate::strings::from_ansi(psz_principal)
    };
    let pkg = crate::strings::from_ansi(psz_package);
    schannel::acquire_credentials_impl(
        principal,
        &pkg,
        f_credential_use,
        p_auth_data,
        ph_credential as *mut SecHandle,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn AcquireCredentialsHandleW(
    psz_principal: *const u16,
    psz_package: *const u16,
    f_credential_use: u32,
    _pv_logon_id: *mut c_void,
    p_auth_data: *mut c_void,
    _p_get_key_fn: *mut c_void,
    _pv_get_key_argument: *mut c_void,
    ph_credential: *mut c_void,
    pts_expiry: *mut i64,
) -> i32 {
    if psz_package.is_null() {
        return SEC_E_SECPKG_NOT_FOUND;
    }
    let principal = if psz_principal.is_null() {
        String::new()
    } else {
        crate::strings::from_wide(psz_principal)
    };
    let pkg = crate::strings::from_wide(psz_package);
    schannel::acquire_credentials_impl(
        principal,
        &pkg,
        f_credential_use,
        p_auth_data,
        ph_credential as *mut SecHandle,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn FreeCredentialsHandle(ph_credential: *mut c_void) -> i32 {
    schannel::free_credentials_handle_impl(ph_credential)
}

pub unsafe extern "win64" fn InitializeSecurityContextA(
    ph_credential: *mut c_void,
    ph_context: *mut c_void,
    psz_target_name: *const u8,
    f_context_req: u32,
    _reserved1: u32,
    _target_data_rep: u32,
    p_input: *mut c_void,
    _reserved2: u32,
    ph_new_context: *mut c_void,
    p_output: *mut c_void,
    pf_context_attr: *mut u32,
    pts_expiry: *mut i64,
) -> i32 {
    let target = if psz_target_name.is_null() {
        None
    } else {
        Some(crate::strings::from_ansi(psz_target_name))
    };
    schannel::initialize_security_context_impl(
        ph_credential as *mut SecHandle,
        ph_context as *mut SecHandle,
        target,
        f_context_req,
        p_input as *mut SecBufferDesc,
        ph_new_context as *mut SecHandle,
        p_output as *mut SecBufferDesc,
        pf_context_attr,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn InitializeSecurityContextW(
    ph_credential: *mut c_void,
    ph_context: *mut c_void,
    psz_target_name: *const u16,
    f_context_req: u32,
    _reserved1: u32,
    _target_data_rep: u32,
    p_input: *mut c_void,
    _reserved2: u32,
    ph_new_context: *mut c_void,
    p_output: *mut c_void,
    pf_context_attr: *mut u32,
    pts_expiry: *mut i64,
) -> i32 {
    let target = if psz_target_name.is_null() {
        None
    } else {
        Some(crate::strings::from_wide(psz_target_name))
    };
    schannel::initialize_security_context_impl(
        ph_credential as *mut SecHandle,
        ph_context as *mut SecHandle,
        target,
        f_context_req,
        p_input as *mut SecBufferDesc,
        ph_new_context as *mut SecHandle,
        p_output as *mut SecBufferDesc,
        pf_context_attr,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn AcceptSecurityContext(
    ph_credential: *mut c_void,
    ph_context: *mut c_void,
    p_input: *mut c_void,
    f_context_req: u32,
    _target_data_rep: u32,
    ph_new_context: *mut c_void,
    p_output: *mut c_void,
    pf_context_attr: *mut u32,
    pts_expiry: *mut i64,
) -> i32 {
    schannel::accept_security_context_impl(
        ph_credential as *mut SecHandle,
        ph_context as *mut SecHandle,
        p_input as *mut SecBufferDesc,
        f_context_req,
        ph_new_context as *mut SecHandle,
        p_output as *mut SecBufferDesc,
        pf_context_attr,
        pts_expiry,
    )
}

pub unsafe extern "win64" fn CompleteAuthToken(
    ph_context: *mut c_void,
    _p_token: *mut c_void,
) -> i32 {
    schannel::complete_auth_token_impl(ph_context)
}

pub unsafe extern "win64" fn DeleteSecurityContext(ph_context: *mut c_void) -> i32 {
    schannel::delete_security_context_impl(ph_context)
}

pub unsafe extern "win64" fn ApplyControlToken(
    ph_context: *mut c_void,
    p_input: *mut c_void,
) -> i32 {
    schannel::apply_control_token_impl(ph_context, p_input)
}

pub unsafe extern "win64" fn FreeContextBuffer(pv_context_buffer: *mut c_void) -> i32 {
    if pv_context_buffer.is_null() {
        return SEC_E_OK;
    }
    if crate::heap::is_ours(pv_context_buffer) {
        crate::heap::free(pv_context_buffer);
        SEC_E_OK
    } else {
        SEC_E_INVALID_HANDLE
    }
}

pub unsafe extern "win64" fn EncryptMessage(
    ph_context: *mut c_void,
    _f_qop: u32,
    p_message: *mut c_void,
    _message_seq_no: u32,
) -> i32 {
    schannel::encrypt_message_impl(ph_context, p_message)
}

pub unsafe extern "win64" fn DecryptMessage(
    ph_context: *mut c_void,
    p_message: *mut c_void,
    _message_seq_no: u32,
    pf_qop: *mut u32,
) -> i32 {
    schannel::decrypt_message_impl(ph_context, p_message, pf_qop)
}

pub unsafe extern "win64" fn QueryContextAttributesA(
    ph_context: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    schannel::query_context_attributes_impl(ph_context, ul_attribute, p_buffer, false)
}

pub unsafe extern "win64" fn QueryContextAttributesW(
    ph_context: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    schannel::query_context_attributes_impl(ph_context, ul_attribute, p_buffer, true)
}

pub unsafe extern "win64" fn QueryCredentialsAttributesA(
    ph_credential: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    schannel::query_credentials_attributes_impl(ph_credential, ul_attribute, p_buffer, false)
}

pub unsafe extern "win64" fn QueryCredentialsAttributesW(
    ph_credential: *mut c_void,
    ul_attribute: u32,
    p_buffer: *mut c_void,
) -> i32 {
    schannel::query_credentials_attributes_impl(ph_credential, ul_attribute, p_buffer, true)
}

pub unsafe extern "win64" fn QuerySecurityPackageInfoA(
    psz_package_name: *const u8,
    pp_package_info: *mut *mut c_void,
) -> i32 {
    schannel::query_security_package_info_a_impl(psz_package_name, pp_package_info)
}

pub unsafe extern "win64" fn QuerySecurityPackageInfoW(
    psz_package_name: *const u16,
    pp_package_info: *mut *mut c_void,
) -> i32 {
    schannel::query_security_package_info_w_impl(psz_package_name, pp_package_info)
}

unsafe extern "win64" fn sspi_unsupported() -> i32 {
    SEC_E_UNSUPPORTED_FUNCTION
}

#[repr(C)]
#[allow(dead_code)]
struct SecurityFunctionTable {
    dw_version: u32,
    slots: [usize; 27],
}

unsafe fn build_table(wide: bool) -> *mut c_void {
    let unsup = sspi_unsupported as *const () as usize;
    let query_cred = if wide {
        QueryCredentialsAttributesW as *const () as usize
    } else {
        QueryCredentialsAttributesA as *const () as usize
    };
    let acq = if wide {
        AcquireCredentialsHandleW as *const () as usize
    } else {
        AcquireCredentialsHandleA as *const () as usize
    };
    let init_ctx = if wide {
        InitializeSecurityContextW as *const () as usize
    } else {
        InitializeSecurityContextA as *const () as usize
    };
    let query_ctx = if wide {
        QueryContextAttributesW as *const () as usize
    } else {
        QueryContextAttributesA as *const () as usize
    };
    let query_pkg = if wide {
        QuerySecurityPackageInfoW as *const () as usize
    } else {
        QuerySecurityPackageInfoA as *const () as usize
    };
    let free_cred = FreeCredentialsHandle as *const () as usize;
    let accept_ctx = AcceptSecurityContext as *const () as usize;
    let complete = CompleteAuthToken as *const () as usize;
    let del_ctx = DeleteSecurityContext as *const () as usize;
    let apply_ctl = ApplyControlToken as *const () as usize;
    let free_buf = FreeContextBuffer as *const () as usize;
    let enc = EncryptMessage as *const () as usize;
    let dec = DecryptMessage as *const () as usize;

    for f in [
        unsup, query_cred, acq, init_ctx, accept_ctx, query_ctx, query_pkg, free_cred, complete,
        del_ctx, apply_ctl, free_buf, enc, dec,
    ] {
        crate::register_native_va(f as u64, f);
    }

    let mut slots = [unsup; 27];
    slots[1] = query_cred; // QueryCredentialsAttributes
    slots[2] = acq; // AcquireCredentialsHandle
    slots[3] = free_cred; // FreeCredentialsHandle
    slots[4] = 0; // Reserved2
    slots[5] = init_ctx; // InitializeSecurityContext
    slots[6] = accept_ctx; // AcceptSecurityContext
    slots[7] = complete; // CompleteAuthToken
    slots[8] = del_ctx; // DeleteSecurityContext
    slots[9] = apply_ctl; // ApplyControlToken
    slots[10] = query_ctx; // QueryContextAttributes
    slots[15] = free_buf; // FreeContextBuffer
    slots[16] = query_pkg; // QuerySecurityPackageInfo
    slots[17] = 0; // Reserved3
    slots[18] = 0; // Reserved4
    slots[22] = 0; // Reserved8
    slots[24] = enc; // EncryptMessage
    slots[25] = dec; // DecryptMessage

    Box::into_raw(Box::new(SecurityFunctionTable {
        dw_version: SECURITY_SUPPORT_PROVIDER_INTERFACE_VERSION,
        slots,
    })) as *mut c_void
}

pub unsafe extern "win64" fn InitSecurityInterfaceA() -> *mut c_void {
    static TBL: OnceLock<usize> = OnceLock::new();
    *TBL.get_or_init(|| build_table(false) as usize) as *mut c_void
}

pub unsafe extern "win64" fn InitSecurityInterfaceW() -> *mut c_void {
    static TBL: OnceLock<usize> = OnceLock::new();
    *TBL.get_or_init(|| build_table(true) as usize) as *mut c_void
}

#[cfg(test)]
pub(crate) mod tests;
