//! Certificate chain engine, WebPKI chain verification, policy checks, and
//! time-validity evaluation for `crypt32.dll`.

use super::asn1::{
    cert_eku_oids, der_item_children, parse_cert_der_parts, parse_der_item, CertDerParts,
};
use super::store::{lookup_cert_for_chain, store_certs_der};
use crate::handles::HandleTable;
use crate::misc::wintrust::{
    self, CertChainContext, CertChainElement, CertContext, CertInfo, CertSimpleChain,
    CertTrustStatus, OwnedCertContext,
};
use crate::net_tls::{
    self, SECURITY_FLAG_IGNORE_CERT_CN_INVALID, SECURITY_FLAG_IGNORE_CERT_DATE_INVALID,
    SECURITY_FLAG_IGNORE_UNKNOWN_CA,
};
use crate::types::*;
use rustls::client::danger::ServerCertVerifier;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use std::ffi::c_void;
use std::sync::{Arc, LazyLock};

pub(super) const CERT_CHAIN_POLICY_BASE: usize = 1;
pub(super) const CERT_CHAIN_POLICY_AUTHENTICODE: usize = 2;
const CERT_CHAIN_POLICY_AUTHENTICODE_TS: usize = 3;
pub(super) const CERT_CHAIN_POLICY_SSL: usize = 4;
pub(super) const CERT_CHAIN_POLICY_BASIC_CONSTRAINTS: usize = 5;
pub(super) const CERT_CHAIN_POLICY_NT_AUTH: usize = 6;
pub(super) const CERT_CHAIN_POLICY_MICROSOFT_ROOT: usize = 7;
pub(super) const CERT_CHAIN_POLICY_EV: usize = 8;

const TRUST_E_BASIC_CONSTRAINTS: u32 = 0x8009_6019;
const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG: u32 = 0x8000_0000;
const BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG: u32 = 0x4000_0000;

// ---------------------------------------------------------------------------
// Apple Security.framework system trust store (macOS & iOS)
// ---------------------------------------------------------------------------

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(super) mod apple_trust {
    use std::ffi::c_void;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFAllocatorDefault: *const c_void;
        static kCFTypeArrayCallBacks: c_void;
        fn CFRelease(cf: *const c_void);
        #[cfg(target_os = "macos")]
        fn CFArrayGetCount(the_array: *const c_void) -> isize;
        #[cfg(target_os = "macos")]
        fn CFArrayGetValueAtIndex(the_array: *const c_void, idx: isize) -> *const c_void;
        fn CFArrayCreate(
            allocator: *const c_void,
            values: *const *const c_void,
            num_values: isize,
            callbacks: *const u8,
        ) -> *const c_void;
        fn CFDataCreate(allocator: *const c_void, bytes: *const u8, length: isize) -> *const c_void;
        #[cfg(target_os = "macos")]
        fn CFDataGetLength(the_data: *const c_void) -> isize;
        #[cfg(target_os = "macos")]
        fn CFDataGetBytePtr(the_data: *const c_void) -> *const u8;
        fn CFDictionaryGetValue(the_dict: *const c_void, key: *const c_void) -> *const c_void;
        #[cfg(target_os = "macos")]
        fn CFDictionaryGetCount(the_dict: *const c_void) -> isize;
        fn CFBooleanGetValue(boolean: *const c_void) -> u8;
        #[cfg(target_os = "macos")]
        fn CFNumberGetValue(number: *const c_void, the_type: isize, value_ptr: *mut c_void) -> u8;
        fn CFDateCreate(allocator: *const c_void, at: f64) -> *const c_void;
        fn CFErrorGetCode(err: *const c_void) -> isize;
        fn CFStringCreateWithBytes(
            alloc: *const c_void,
            bytes: *const u8,
            num_bytes: isize,
            encoding: u32,
            is_external_representation: u8,
        ) -> *const c_void;
    }

    #[link(name = "Security", kind = "framework")]
    extern "C" {
        static kSecTrustRevocationChecked: *const c_void;
        fn SecCertificateCreateWithData(
            allocator: *const c_void,
            data: *const c_void,
        ) -> *const c_void;
        #[cfg(target_os = "macos")]
        fn SecCertificateCopyData(certificate: *const c_void) -> *const c_void;
        fn SecPolicyCreateBasicX509() -> *const c_void;
        fn SecPolicyCreateSSL(server: u8, hostname: *const c_void) -> *const c_void;
        fn SecPolicyCreateRevocation(revocation_flags: usize) -> *const c_void;
        fn SecTrustCreateWithCertificates(
            certificates: *const c_void,
            policies: *const c_void,
            trust: *mut *const c_void,
        ) -> i32;
        fn SecTrustSetVerifyDate(trust: *const c_void, verify_date: *const c_void) -> i32;
        fn SecTrustSetNetworkFetchAllowed(trust: *const c_void, allow_fetch: u8) -> i32;
        fn SecTrustSetAnchorCertificates(
            trust: *const c_void,
            anchor_certificates: *const c_void,
        ) -> i32;
        fn SecTrustSetAnchorCertificatesOnly(trust: *const c_void, anchor_certs_only: u8) -> i32;
        fn SecTrustEvaluateWithError(trust: *const c_void, error: *mut *const c_void) -> u8;
        fn SecTrustCopyResult(trust: *const c_void) -> *const c_void;

        #[cfg(target_os = "macos")]
        fn SecTrustCopyAnchorCertificates(anchors: *mut *const c_void) -> i32;
        #[cfg(target_os = "macos")]
        fn SecTrustSettingsCopyCertificates(domain: u32, certs_out: *mut *const c_void) -> i32;
        #[cfg(target_os = "macos")]
        fn SecTrustSettingsCopyTrustSettings(
            cert_ref: *const c_void,
            domain: u32,
            trust_settings: *mut *const c_void,
        ) -> i32;
    }

    struct CfRef(*const c_void);
    impl Drop for CfRef {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    #[cfg(target_os = "macos")]
    unsafe fn cert_ref_to_der(cert_ref: *const c_void) -> Option<Vec<u8>> {
        if cert_ref.is_null() {
            return None;
        }
        let data = SecCertificateCopyData(cert_ref);
        if data.is_null() {
            return None;
        }
        let _g = CfRef(data);
        let len = CFDataGetLength(data);
        if len <= 0 {
            return None;
        }
        let ptr = CFDataGetBytePtr(data);
        if ptr.is_null() {
            return None;
        }
        Some(std::slice::from_raw_parts(ptr, len as usize).to_vec())
    }

    #[cfg(target_os = "macos")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum TrustSettingDecision {
        NotSet,
        UnconditionalTrustRoot,
        Denied,
        PolicyRestrictedOrUnspecified,
    }

    /// Classify a macOS `SecTrustSettings` `CFArrayRef` of usage-constraint dictionaries.
    /// Per Apple's `SecTrustSettings.h` specification, ALL specified usage-constraint components
    /// (`kSecTrustSettingsPolicy`, `kSecTrustSettingsPolicyString`, `kSecTrustSettingsApplication`,
    /// `kSecTrustSettingsKeyUsage`, `kSecTrustSettingsAllowedError`, or any unrecognized key) must
    /// match for a rule to apply:
    /// - An empty `trustSettings` array means "Always Trust" unconditionally for all policies.
    /// - A dictionary is unconditional ONLY when every entry in it is either `kSecTrustSettingsResult`
    ///   or `kSecTrustSettingsKeyUsage == kSecTrustSettingsKeyUseAny` (`-1` / `0xFFFF_FFFF`).
    /// - Any dictionary carrying `kSecTrustSettingsPolicy`, `kSecTrustSettingsPolicyString`,
    ///   `kSecTrustSettingsApplication`, a restrictive `kSecTrustSettingsKeyUsage`,
    ///   `kSecTrustSettingsAllowedError`, or any unknown constraint key is NOT flattened into
    ///   unrestricted roots; `evaluate_sec_trust()` evaluates it with Apple's policy engine instead.
    #[cfg(target_os = "macos")]
    pub(crate) unsafe fn classify_trust_settings_array(
        settings: *const c_void,
    ) -> TrustSettingDecision {
        const K_CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;
        const K_CF_NUMBER_SINT32_TYPE: isize = 3;
        const K_SEC_TRUST_SETTINGS_RESULT_TRUST_ROOT: i32 = 1;
        const K_SEC_TRUST_SETTINGS_RESULT_TRUST_AS_ROOT: i32 = 2;
        const K_SEC_TRUST_SETTINGS_RESULT_DENY: i32 = 3;
        const K_SEC_TRUST_SETTINGS_KEY_USE_ANY: i32 = -1; // 0xFFFF_FFFF

        if settings.is_null() {
            return TrustSettingDecision::NotSet;
        }
        let count = CFArrayGetCount(settings);
        if count == 0 {
            return TrustSettingDecision::UnconditionalTrustRoot;
        }

        let k_result = CFStringCreateWithBytes(
            kCFAllocatorDefault,
            b"kSecTrustSettingsResult".as_ptr(),
            23,
            K_CF_STRING_ENCODING_UTF8,
            0,
        );
        let _krg = CfRef(k_result);
        let k_key_usage = CFStringCreateWithBytes(
            kCFAllocatorDefault,
            b"kSecTrustSettingsKeyUsage".as_ptr(),
            25,
            K_CF_STRING_ENCODING_UTF8,
            0,
        );
        let _kkug = CfRef(k_key_usage);
        if k_result.is_null() || k_key_usage.is_null() {
            return TrustSettingDecision::PolicyRestrictedOrUnspecified;
        }

        let mut saw_unconditional_trust = false;
        let mut saw_unconditional_deny = false;
        let mut saw_constrained_rule = false;
        for i in 0..count {
            let dict = CFArrayGetValueAtIndex(settings, i);
            if dict.is_null() {
                continue;
            }
            let total_keys = CFDictionaryGetCount(dict);
            let mut unconstrained_keys: isize = 0;

            let result_num = CFDictionaryGetValue(dict, k_result);
            let result_val: i32 = if !result_num.is_null() {
                unconstrained_keys += 1;
                let mut v: i32 = 0;
                if CFNumberGetValue(
                    result_num,
                    K_CF_NUMBER_SINT32_TYPE,
                    &mut v as *mut i32 as *mut c_void,
                ) != 0
                {
                    v
                } else {
                    0 // malformed kSecTrustSettingsResult -> do not default to TrustRoot
                }
            } else {
                K_SEC_TRUST_SETTINGS_RESULT_TRUST_ROOT
            };

            let ku_num = CFDictionaryGetValue(dict, k_key_usage);
            if !ku_num.is_null() {
                let mut ku: i32 = 0;
                if CFNumberGetValue(
                    ku_num,
                    K_CF_NUMBER_SINT32_TYPE,
                    &mut ku as *mut i32 as *mut c_void,
                ) != 0
                    && ku == K_SEC_TRUST_SETTINGS_KEY_USE_ANY
                {
                    unconstrained_keys += 1;
                }
            }

            // If ANY other key is present (kSecTrustSettingsPolicy, kSecTrustSettingsPolicyString,
            // kSecTrustSettingsApplication, kSecTrustSettingsAllowedError, restrictive
            // kSecTrustSettingsKeyUsage, or any future constraint key), total_keys > unconstrained_keys
            // and this rule is constrained. A trust-settings array containing ANY constrained rule
            // (e.g. a scoped Deny before an unconditional TrustRoot fallback) MUST NOT be promoted
            // to unconditional trust — defer the root to policy-aware SecTrust evaluation.
            if total_keys == unconstrained_keys {
                if result_val == K_SEC_TRUST_SETTINGS_RESULT_DENY {
                    saw_unconditional_deny = true;
                } else if result_val == K_SEC_TRUST_SETTINGS_RESULT_TRUST_ROOT
                    || result_val == K_SEC_TRUST_SETTINGS_RESULT_TRUST_AS_ROOT
                {
                    saw_unconditional_trust = true;
                } else {
                    saw_constrained_rule = true;
                }
            } else {
                saw_constrained_rule = true;
            }
        }
        if saw_constrained_rule {
            TrustSettingDecision::PolicyRestrictedOrUnspecified
        } else if saw_unconditional_deny {
            TrustSettingDecision::Denied
        } else if saw_unconditional_trust {
            TrustSettingDecision::UnconditionalTrustRoot
        } else {
            TrustSettingDecision::PolicyRestrictedOrUnspecified
        }
    }

    #[cfg(target_os = "macos")]
    unsafe fn inspect_trust_settings(cert: *const c_void, domain: u32) -> TrustSettingDecision {
        let mut settings: *const c_void = std::ptr::null();
        if SecTrustSettingsCopyTrustSettings(cert, domain, &mut settings) != 0 || settings.is_null()
        {
            return TrustSettingDecision::NotSet;
        }
        let _sg = CfRef(settings);
        classify_trust_settings_array(settings)
    }

    /// Split macOS system/custom anchors into `(unconditional_trusted, excluded_from_unconditional)`.
    /// Any root with `Denied` or `PolicyRestrictedOrUnspecified` trust settings is excluded from
    /// the unconditional set so policy- or application-scoped distrust is preserved and deferred
    /// to policy-aware `SecTrust` evaluation rather than being bypassed by rustls.
    #[cfg(any(target_os = "macos", test))]
    pub(crate) fn filter_unconditional_anchors(
        system_anchors: Vec<Vec<u8>>,
        domain_decisions: &std::collections::HashMap<Vec<u8>, TrustSettingDecision>,
    ) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        use std::collections::HashSet;

        let mut excluded: HashSet<Vec<u8>> = HashSet::new();
        let mut custom_trusted: Vec<Vec<u8>> = Vec::new();
        for (der, &decision) in domain_decisions {
            match decision {
                TrustSettingDecision::Denied
                | TrustSettingDecision::PolicyRestrictedOrUnspecified => {
                    excluded.insert(der.clone());
                }
                TrustSettingDecision::UnconditionalTrustRoot => {
                    custom_trusted.push(der.clone());
                }
                TrustSettingDecision::NotSet => {}
            }
        }

        let mut out = Vec::new();
        for der in system_anchors {
            if !excluded.contains(&der) {
                out.push(der);
            }
        }
        for der in custom_trusted {
            if !excluded.contains(&der) && !out.iter().any(|existing| existing == &der) {
                out.push(der);
            }
        }
        (out, excluded.into_iter().collect())
    }

    /// Copy unconditionally trusted root certificates from the macOS System/Admin/User trust
    /// stores while honoring `SecTrustSettingsCopyTrustSettings` (excluding both unconditionally
    /// denied and policy/application-restricted entries from unconditional roots).
    #[cfg(target_os = "macos")]
    pub fn load_macos_trust_anchors_and_exclusions() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        use std::collections::HashMap;

        let mut system_anchors = Vec::new();
        let mut domain_decisions: HashMap<Vec<u8>, TrustSettingDecision> = HashMap::new();
        unsafe {
            // Evaluate Admin (1) then User (0) so User trust settings override Admin.
            for domain in [1u32, 0u32] {
                let mut arr: *const c_void = std::ptr::null();
                if SecTrustSettingsCopyCertificates(domain, &mut arr) == 0 && !arr.is_null() {
                    let _g = CfRef(arr);
                    let count = CFArrayGetCount(arr);
                    for i in 0..count {
                        let cert = CFArrayGetValueAtIndex(arr, i);
                        let decision = inspect_trust_settings(cert, domain);
                        if decision == TrustSettingDecision::NotSet {
                            continue;
                        }
                        if let Some(der) = cert_ref_to_der(cert) {
                            domain_decisions.insert(der, decision);
                        }
                    }
                }
            }

            let mut anchors: *const c_void = std::ptr::null();
            if SecTrustCopyAnchorCertificates(&mut anchors) == 0 && !anchors.is_null() {
                let _g = CfRef(anchors);
                let count = CFArrayGetCount(anchors);
                for i in 0..count {
                    let cert = CFArrayGetValueAtIndex(anchors, i);
                    if let Some(der) = cert_ref_to_der(cert) {
                        system_anchors.push(der);
                    }
                }
            }
        }
        filter_unconditional_anchors(system_anchors, &domain_decisions)
    }

    #[cfg(not(target_os = "macos"))]
    pub fn load_macos_trust_anchors_and_exclusions() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        (Vec::new(), Vec::new())
    }

    /// Evaluate a certificate chain against the Apple Security.framework trust engine.
    /// `exclusive_anchors`: if `Some`, trust ONLY those anchors; otherwise combine `extra_anchors`
    /// with the macOS/iOS system trust store.
    pub fn evaluate_sec_trust(
        chain_der: &[Vec<u8>],
        exclusive_anchors: Option<&[Vec<u8>]>,
        extra_anchors: &[Vec<u8>],
        ssl_hostname: Option<&str>,
        verify_unix_secs: Option<f64>,
    ) -> Option<bool> {
        if chain_der.is_empty() {
            return Some(false);
        }
        unsafe {
            let mut cert_guards = Vec::with_capacity(chain_der.len());
            let mut cert_ptrs = Vec::with_capacity(chain_der.len());
            for der in chain_der {
                let d = CFDataCreate(kCFAllocatorDefault, der.as_ptr(), der.len() as isize);
                if d.is_null() {
                    continue;
                }
                let _dg = CfRef(d);
                let c = SecCertificateCreateWithData(kCFAllocatorDefault, d);
                if !c.is_null() {
                    cert_ptrs.push(c);
                    cert_guards.push(CfRef(c));
                }
            }
            if cert_ptrs.is_empty() {
                return Some(false);
            }
            let cf_certs = CFArrayCreate(
                kCFAllocatorDefault,
                cert_ptrs.as_ptr(),
                cert_ptrs.len() as isize,
                std::ptr::addr_of!(kCFTypeArrayCallBacks) as *const u8,
            );
            if cf_certs.is_null() {
                return None;
            }
            let _certs_g = CfRef(cf_certs);

            let policy = if let Some(host) = ssl_hostname {
                let cf_host = CFStringCreateWithBytes(
                    kCFAllocatorDefault,
                    host.as_ptr(),
                    host.len() as isize,
                    0x0800_0100, // kCFStringEncodingUTF8
                    0,
                );
                let _hg = CfRef(cf_host);
                SecPolicyCreateSSL(1, cf_host)
            } else {
                SecPolicyCreateBasicX509()
            };
            if policy.is_null() {
                return None;
            }
            let _pol_g = CfRef(policy);

            let mut trust: *const c_void = std::ptr::null();
            if SecTrustCreateWithCertificates(cf_certs, policy, &mut trust) != 0 || trust.is_null() {
                return None;
            }
            let _trust_g = CfRef(trust);

            if let Some(unix_s) = verify_unix_secs {
                // CFAbsoluteTime epoch is 2001-01-01 (978307200 seconds after Unix epoch).
                let cf_abs = unix_s - 978_307_200.0;
                let date = CFDateCreate(kCFAllocatorDefault, cf_abs);
                if !date.is_null() {
                    let _dg = CfRef(date);
                    let _ = SecTrustSetVerifyDate(trust, date);
                }
            }

            let anchor_slice = exclusive_anchors.unwrap_or(extra_anchors);
            if exclusive_anchors.is_some() || !anchor_slice.is_empty() {
                let mut a_guards = Vec::with_capacity(anchor_slice.len());
                let mut a_ptrs = Vec::with_capacity(anchor_slice.len());
                for der in anchor_slice {
                    let d = CFDataCreate(kCFAllocatorDefault, der.as_ptr(), der.len() as isize);
                    if d.is_null() {
                        continue;
                    }
                    let _dg = CfRef(d);
                    let c = SecCertificateCreateWithData(kCFAllocatorDefault, d);
                    if !c.is_null() {
                        a_ptrs.push(c);
                        a_guards.push(CfRef(c));
                    }
                }
                let cf_anchors = CFArrayCreate(
                    kCFAllocatorDefault,
                    a_ptrs.as_ptr(),
                    a_ptrs.len() as isize,
                    std::ptr::addr_of!(kCFTypeArrayCallBacks) as *const u8,
                );
                if !cf_anchors.is_null() {
                    let _ag = CfRef(cf_anchors);
                    let _ = SecTrustSetAnchorCertificates(trust, cf_anchors);
                    let _ = SecTrustSetAnchorCertificatesOnly(
                        trust,
                        u8::from(exclusive_anchors.is_some()),
                    );
                }
            }

            let mut err: *const c_void = std::ptr::null();
            let ok = SecTrustEvaluateWithError(trust, &mut err);
            let _eg = CfRef(err);
            Some(ok != 0)
        }
    }

    /// Evaluate Extended Validation (EV) using Apple's Security.framework EV root & policy table
    /// (`SecTrustCopyResult` -> `"TrustExtendedValidation"`). Ordinary DV/OV certificates return
    /// `false` even when they carry a `certificatePolicies` extension.
    pub fn evaluate_sec_trust_ev(
        chain_der: &[Vec<u8>],
        exclusive_anchors: Option<&[Vec<u8>]>,
        extra_anchors: &[Vec<u8>],
        verify_unix_secs: Option<f64>,
    ) -> bool {
        if chain_der.is_empty() {
            return false;
        }
        unsafe {
            let mut cert_guards = Vec::with_capacity(chain_der.len());
            let mut cert_ptrs = Vec::with_capacity(chain_der.len());
            for der in chain_der {
                let d = CFDataCreate(kCFAllocatorDefault, der.as_ptr(), der.len() as isize);
                if d.is_null() {
                    continue;
                }
                let _dg = CfRef(d);
                let c = SecCertificateCreateWithData(kCFAllocatorDefault, d);
                if !c.is_null() {
                    cert_ptrs.push(c);
                    cert_guards.push(CfRef(c));
                }
            }
            if cert_ptrs.is_empty() {
                return false;
            }
            let cf_certs = CFArrayCreate(
                kCFAllocatorDefault,
                cert_ptrs.as_ptr(),
                cert_ptrs.len() as isize,
                std::ptr::addr_of!(kCFTypeArrayCallBacks) as *const u8,
            );
            if cf_certs.is_null() {
                return false;
            }
            let _certs_g = CfRef(cf_certs);
            let policy = SecPolicyCreateSSL(1, std::ptr::null());
            if policy.is_null() {
                return false;
            }
            let _pol_g = CfRef(policy);
            let mut trust: *const c_void = std::ptr::null();
            if SecTrustCreateWithCertificates(cf_certs, policy, &mut trust) != 0 || trust.is_null() {
                return false;
            }
            let _trust_g = CfRef(trust);
            if let Some(unix_s) = verify_unix_secs {
                let cf_abs = unix_s - 978_307_200.0;
                let date = CFDateCreate(kCFAllocatorDefault, cf_abs);
                if !date.is_null() {
                    let _dg = CfRef(date);
                    let _ = SecTrustSetVerifyDate(trust, date);
                }
            }
            let anchor_slice = exclusive_anchors.unwrap_or(extra_anchors);
            if exclusive_anchors.is_some() || !anchor_slice.is_empty() {
                let mut a_guards = Vec::with_capacity(anchor_slice.len());
                let mut a_ptrs = Vec::with_capacity(anchor_slice.len());
                for der in anchor_slice {
                    let d = CFDataCreate(kCFAllocatorDefault, der.as_ptr(), der.len() as isize);
                    if d.is_null() {
                        continue;
                    }
                    let _dg = CfRef(d);
                    let c = SecCertificateCreateWithData(kCFAllocatorDefault, d);
                    if !c.is_null() {
                        a_ptrs.push(c);
                        a_guards.push(CfRef(c));
                    }
                }
                let cf_anchors = CFArrayCreate(
                    kCFAllocatorDefault,
                    a_ptrs.as_ptr(),
                    a_ptrs.len() as isize,
                    std::ptr::addr_of!(kCFTypeArrayCallBacks) as *const u8,
                );
                if !cf_anchors.is_null() {
                    let _ag = CfRef(cf_anchors);
                    let _ = SecTrustSetAnchorCertificates(trust, cf_anchors);
                    let _ = SecTrustSetAnchorCertificatesOnly(
                        trust,
                        u8::from(exclusive_anchors.is_some()),
                    );
                }
            }
            let mut err: *const c_void = std::ptr::null();
            let ok = SecTrustEvaluateWithError(trust, &mut err);
            let _eg = CfRef(err);
            if ok == 0 {
                return false;
            }
            let result_dict = SecTrustCopyResult(trust);
            if result_dict.is_null() {
                return false;
            }
            let _rdg = CfRef(result_dict);
            let ev_key = CFStringCreateWithBytes(
                kCFAllocatorDefault,
                b"TrustExtendedValidation".as_ptr(),
                23,
                0x0800_0100,
                0,
            );
            if ev_key.is_null() {
                return false;
            }
            let _ekg = CfRef(ev_key);
            let ev_val = CFDictionaryGetValue(result_dict, ev_key);
            !ev_val.is_null() && CFBooleanGetValue(ev_val) != 0
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum RevocationCheckResult {
        VerifiedGood,
        Revoked,
        OfflineOrUnknown,
    }

    /// Select `(certs, anchors, anchors_only)` for Apple Security.framework revocation checking.
    /// When `end_cert_only` is true (`CERT_CHAIN_REVOCATION_CHECK_END_CERT`), pass only the leaf
    /// certificate anchored at its direct issuer (if present in the chain/anchors) so Security.framework
    /// verifies ONLY the end certificate's revocation using its issuer to authenticate OCSP/CRL,
    /// without failing offline when an intermediate CA's own revocation response is unavailable.
    pub(crate) fn revocation_eval_inputs<'a>(
        selected_path: &[&'a [u8]],
        exclusive_anchors: Option<&'a [Vec<u8>]>,
        extra_anchors: &'a [Vec<u8>],
        end_cert_only: bool,
    ) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, bool) {
        if end_cert_only {
            if let Some(&leaf) = selected_path.first() {
                if let Some(&direct_issuer) = selected_path.get(1) {
                    return (vec![leaf.to_vec()], vec![direct_issuer.to_vec()], true);
                }
                let anchors = exclusive_anchors.unwrap_or(extra_anchors).to_vec();
                return (vec![leaf.to_vec()], anchors, exclusive_anchors.is_some());
            }
        }
        let certs: Vec<Vec<u8>> = selected_path.iter().map(|c| c.to_vec()).collect();
        let anchors = exclusive_anchors.unwrap_or(extra_anchors).to_vec();
        (certs, anchors, exclusive_anchors.is_some())
    }

    /// Evaluate certificate revocation via Apple Security.framework (`SecPolicyCreateRevocation`
    /// with `kSecRevocationUseAnyAvailableMethod | kSecRevocationRequirePositiveResponse`).
    /// Fails closed (`OfflineOrUnknown` or `Revoked`) unless Apple's trust engine positively
    /// confirms non-revoked status AND sets `kSecTrustRevocationChecked = true`.
    pub fn evaluate_sec_trust_revocation(
        chain_der: &[Vec<u8>],
        exclusive_anchors: Option<&[Vec<u8>]>,
        extra_anchors: &[Vec<u8>],
        verify_unix_secs: Option<f64>,
        cache_only: bool,
        end_cert_only: bool,
    ) -> RevocationCheckResult {
        const K_SEC_REVOCATION_USE_ANY: usize = (1 << 0) | (1 << 1);
        const K_SEC_REVOCATION_REQUIRE_POSITIVE: usize = 1 << 3;
        const K_SEC_REVOCATION_NETWORK_DISABLED: usize = 1 << 4;
        const ERR_SEC_CERTIFICATE_REVOKED: isize = -67820;

        if chain_der.is_empty() {
            return RevocationCheckResult::OfflineOrUnknown;
        }
        let root_pool = exclusive_anchors.unwrap_or(extra_anchors);
        let selected_path = super::resolve_selected_issuer_path(chain_der, root_pool);
        let (eval_certs, eval_anchors, anchors_only) =
            revocation_eval_inputs(&selected_path, exclusive_anchors, extra_anchors, end_cert_only);
        if eval_certs.is_empty() {
            return RevocationCheckResult::OfflineOrUnknown;
        }
        unsafe {
            let mut cert_guards = Vec::with_capacity(eval_certs.len());
            let mut cert_ptrs = Vec::with_capacity(eval_certs.len());
            for der in &eval_certs {
                let d = CFDataCreate(kCFAllocatorDefault, der.as_ptr(), der.len() as isize);
                if d.is_null() {
                    continue;
                }
                let _dg = CfRef(d);
                let c = SecCertificateCreateWithData(kCFAllocatorDefault, d);
                if !c.is_null() {
                    cert_ptrs.push(c);
                    cert_guards.push(CfRef(c));
                }
            }
            if cert_ptrs.is_empty() {
                return RevocationCheckResult::OfflineOrUnknown;
            }
            let cf_certs = CFArrayCreate(
                kCFAllocatorDefault,
                cert_ptrs.as_ptr(),
                cert_ptrs.len() as isize,
                std::ptr::addr_of!(kCFTypeArrayCallBacks) as *const u8,
            );
            if cf_certs.is_null() {
                return RevocationCheckResult::OfflineOrUnknown;
            }
            let _certs_g = CfRef(cf_certs);

            let basic_policy = SecPolicyCreateBasicX509();
            if basic_policy.is_null() {
                return RevocationCheckResult::OfflineOrUnknown;
            }
            let _bp_g = CfRef(basic_policy);

            let mut rev_flags = K_SEC_REVOCATION_USE_ANY | K_SEC_REVOCATION_REQUIRE_POSITIVE;
            if cache_only {
                rev_flags |= K_SEC_REVOCATION_NETWORK_DISABLED;
            }
            let rev_policy = SecPolicyCreateRevocation(rev_flags);
            if rev_policy.is_null() {
                return RevocationCheckResult::OfflineOrUnknown;
            }
            let _rp_g = CfRef(rev_policy);

            let pol_ptrs = [basic_policy, rev_policy];
            let cf_policies = CFArrayCreate(
                kCFAllocatorDefault,
                pol_ptrs.as_ptr(),
                pol_ptrs.len() as isize,
                std::ptr::addr_of!(kCFTypeArrayCallBacks) as *const u8,
            );
            if cf_policies.is_null() {
                return RevocationCheckResult::OfflineOrUnknown;
            }
            let _pols_g = CfRef(cf_policies);

            let mut trust: *const c_void = std::ptr::null();
            if SecTrustCreateWithCertificates(cf_certs, cf_policies, &mut trust) != 0
                || trust.is_null()
            {
                return RevocationCheckResult::OfflineOrUnknown;
            }
            let _trust_g = CfRef(trust);

            let _ = SecTrustSetNetworkFetchAllowed(trust, u8::from(!cache_only));
            if let Some(unix_s) = verify_unix_secs {
                let cf_abs = unix_s - 978_307_200.0;
                let date = CFDateCreate(kCFAllocatorDefault, cf_abs);
                if !date.is_null() {
                    let _dg = CfRef(date);
                    let _ = SecTrustSetVerifyDate(trust, date);
                }
            }

            if anchors_only || !eval_anchors.is_empty() {
                let mut a_guards = Vec::with_capacity(eval_anchors.len());
                let mut a_ptrs = Vec::with_capacity(eval_anchors.len());
                for der in &eval_anchors {
                    let d = CFDataCreate(kCFAllocatorDefault, der.as_ptr(), der.len() as isize);
                    if d.is_null() {
                        continue;
                    }
                    let _dg = CfRef(d);
                    let c = SecCertificateCreateWithData(kCFAllocatorDefault, d);
                    if !c.is_null() {
                        a_ptrs.push(c);
                        a_guards.push(CfRef(c));
                    }
                }
                let cf_anchors = CFArrayCreate(
                    kCFAllocatorDefault,
                    a_ptrs.as_ptr(),
                    a_ptrs.len() as isize,
                    std::ptr::addr_of!(kCFTypeArrayCallBacks) as *const u8,
                );
                if !cf_anchors.is_null() {
                    let _ag = CfRef(cf_anchors);
                    let _ = SecTrustSetAnchorCertificates(trust, cf_anchors);
                    let _ = SecTrustSetAnchorCertificatesOnly(trust, u8::from(anchors_only));
                }
            }

            let mut cf_err: *const c_void = std::ptr::null();
            let ok = SecTrustEvaluateWithError(trust, &mut cf_err);
            let _eg = CfRef(cf_err);
            if ok != 0 {
                let result_dict = SecTrustCopyResult(trust);
                if result_dict.is_null() {
                    return RevocationCheckResult::OfflineOrUnknown;
                }
                let _rdg = CfRef(result_dict);
                let checked = CFDictionaryGetValue(result_dict, kSecTrustRevocationChecked);
                if !checked.is_null() && CFBooleanGetValue(checked) != 0 {
                    RevocationCheckResult::VerifiedGood
                } else {
                    RevocationCheckResult::OfflineOrUnknown
                }
            } else if !cf_err.is_null() && CFErrorGetCode(cf_err) == ERR_SEC_CERTIFICATE_REVOKED {
                RevocationCheckResult::Revoked
            } else {
                RevocationCheckResult::OfflineOrUnknown
            }
        }
    }
}

fn macos_roots_and_exclusions() -> &'static (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    static ROOTS: LazyLock<(Vec<Vec<u8>>, Vec<Vec<u8>>)> = LazyLock::new(|| {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            apple_trust::load_macos_trust_anchors_and_exclusions()
        }
        #[cfg(not(any(target_os = "macos", target_os = "ios")))]
        {
            (Vec::new(), Vec::new())
        }
    });
    &ROOTS
}

pub(super) fn macos_system_roots_der() -> &'static [Vec<u8>] {
    &macos_roots_and_exclusions().0
}

pub(super) fn macos_excluded_root_spkis() -> &'static [Vec<u8>] {
    static EXCLUDED_SPKIS: LazyLock<Vec<Vec<u8>>> = LazyLock::new(|| {
        macos_roots_and_exclusions()
            .1
            .iter()
            .filter_map(|der| parse_cert_der_parts(der).map(|p| p.spki_inner.to_vec()))
            .collect()
    });
    &EXCLUDED_SPKIS
}

#[inline]
pub(super) fn spki_is_excluded(spki_inner: &[u8], excluded_spkis: &[Vec<u8>]) -> bool {
    !excluded_spkis.is_empty()
        && excluded_spkis
            .iter()
            .any(|spki| spki.as_slice() == spki_inner)
}

fn combined_default_root_store() -> Arc<rustls::RootCertStore> {
    static COMBINED: LazyLock<Arc<rustls::RootCertStore>> = LazyLock::new(|| {
        let macos_roots = macos_system_roots_der();
        let excluded_spkis = macos_excluded_root_spkis();
        let mut s = rustls::RootCertStore::empty();
        for ta in webpki_roots::TLS_SERVER_ROOTS.iter() {
            if spki_is_excluded(ta.subject_public_key_info.as_ref(), excluded_spkis) {
                continue;
            }
            s.roots.push(ta.clone());
        }
        for der in macos_roots {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    });
    Arc::clone(&COMBINED)
}

// CERT_CHAIN_POLICY_PARA.dwFlags
pub(super) const CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG: u32 = 0x0000_0001;
const CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS: u32 = 0x0000_0007;
pub(super) const CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG: u32 = 0x0000_0010;
const CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG: u32 = 0x0000_0020;
const CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG: u32 = 0x0000_0040;
const CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG: u32 = 0x0000_0100;
const CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG: u32 = 0x0000_0400;
const CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG: u32 = 0x0000_0800;
const SECURITY_FLAG_IGNORE_REVOCATION: u32 = 0x0000_0080;

// CertGetCertificateChain dwFlags
const CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL: u32 = 0x0000_0004;
const CERT_CHAIN_REVOCATION_CHECK_END_CERT: u32 = 0x1000_0000;
const CERT_CHAIN_REVOCATION_CHECK_CHAIN: u32 = 0x2000_0000;
const CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT: u32 = 0x4000_0000;
const CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY: u32 = 0x8000_0000;
const CERT_CHAIN_REVOCATION_CHECK_MASK: u32 = CERT_CHAIN_REVOCATION_CHECK_END_CERT
    | CERT_CHAIN_REVOCATION_CHECK_CHAIN
    | CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT;

// Windows CERT_E_* / CRYPT_E_* verification status codes
pub(crate) const CERT_E_EXPIRED: u32 = 0x800B_0101;
pub(crate) const CERT_E_UNTRUSTEDROOT: u32 = 0x800B_0109;
pub(crate) const CERT_E_CHAINING: u32 = 0x800B_010A;
pub(crate) const CERT_E_REVOKED: u32 = 0x800B_010C;
pub(crate) const CERT_E_CN_NO_MATCH: u32 = 0x800B_010F;
pub(crate) const CERT_E_WRONG_USAGE: u32 = 0x800B_0110;
pub(crate) const CERT_E_UNTRUSTEDCA: u32 = 0x800B_0112;
pub(crate) const CERT_E_INVALID_POLICY: u32 = 0x800B_0113;
pub(crate) const CRYPT_E_NO_REVOCATION_CHECK: u32 = 0x8009_2012;
pub(crate) const CRYPT_E_REVOCATION_OFFLINE: u32 = 0x8009_2013;

// CERT_TRUST_STATUS.dwErrorStatus bits
const CERT_TRUST_IS_NOT_TIME_VALID: u32 = 0x0000_0001;
const CERT_TRUST_IS_REVOKED: u32 = 0x0000_0004;
const CERT_TRUST_IS_NOT_SIGNATURE_VALID: u32 = 0x0000_0008;
const CERT_TRUST_IS_NOT_VALID_FOR_USAGE: u32 = 0x0000_0010;
const CERT_TRUST_IS_UNTRUSTED_ROOT: u32 = 0x0000_0020;
const CERT_TRUST_REVOCATION_STATUS_UNKNOWN: u32 = 0x0000_0040;
const CERT_TRUST_INVALID_POLICY_CONSTRAINTS: u32 = 0x0000_0400;
const CERT_TRUST_IS_PARTIAL_CHAIN: u32 = 0x0001_0000;
const CERT_TRUST_IS_OFFLINE_REVOCATION: u32 = 0x0100_0000;

// ---------------------------------------------------------------------------
// Chain engine & chain registries (backed by `HandleTable`)
// ---------------------------------------------------------------------------

struct RegisteredEngine {
    exclusive_roots_der: Option<Vec<Vec<u8>>>,
    additional_certs_der: Vec<Vec<u8>>,
}

struct RegisteredChain {
    refcount: usize,
    _certs: Vec<OwnedCertContext>,
    _elements: Vec<Box<CertChainElement>>,
    _element_ptrs: Box<[*const CertChainElement]>,
    _simple_chain: Box<CertSimpleChain>,
    _simple_chain_ptrs: Box<[*const CertSimpleChain]>,
    _chain_context: Box<CertChainContext>,
    chain_der: Vec<Vec<u8>>,
    element_times: Vec<(u64, u64)>,
    verify_filetime: Option<u64>,
    exclusive_roots_der: Option<Vec<Vec<u8>>>,
    extra_roots_der: Vec<Vec<u8>>,
    default_chain_error: u32,
    revocation_flags: u32,
    revocation_error: u32,
    chain_para_error: u32,
}

unsafe impl Send for RegisteredChain {}

static ENGINES: HandleTable<RegisteredEngine> = HandleTable::new(0x5200_0001, 1);
static CHAINS: HandleTable<RegisteredChain> = HandleTable::new(0x5400_0001, 1);

// ---------------------------------------------------------------------------
// Time, hostname, signature, and WebPKI verification helpers
// ---------------------------------------------------------------------------

fn now_filetime() -> u64 {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    (dur.as_secs().saturating_add(11_644_473_600))
        .saturating_mul(10_000_000)
        .saturating_add((dur.subsec_nanos() / 100) as u64)
}

fn filetime_to_unix_time(ft: u64) -> Option<UnixTime> {
    const EPOCH_DIFF_100NS: u64 = 11_644_473_600u64 * 10_000_000;
    let unix_100ns = ft.checked_sub(EPOCH_DIFF_100NS)?;
    let secs = unix_100ns / 10_000_000;
    let nanos = ((unix_100ns % 10_000_000) * 100) as u32;
    Some(UnixTime::since_unix_epoch(std::time::Duration::new(
        secs, nanos,
    )))
}

fn dns_pattern_matches(pattern: &str, host: &str) -> bool {
    let pat = pattern.trim_end_matches('.').to_ascii_lowercase();
    let h = host.trim_end_matches('.').to_ascii_lowercase();
    if pat.is_empty() || h.is_empty() {
        return false;
    }
    if pat == h {
        return true;
    }
    // RFC 6125 leftmost single-label wildcard (*.example.com)
    if let Some(suffix) = pat.strip_prefix("*.") {
        if suffix.contains('*') || !suffix.contains('.') {
            return false;
        }
        if let Some(prefix) = h.strip_suffix(suffix).and_then(|p| p.strip_suffix('.')) {
            return !prefix.is_empty() && !prefix.contains('.');
        }
    }
    false
}

/// Check whether `leaf_der` matches `server_name` (DNS or IP) per RFC 6125:
/// checks SAN (`2.5.29.17`) `dNSName` / `iPAddress`, falling back to Subject CN
/// only when no SAN extension is present.
fn cert_matches_hostname(leaf_der: &[u8], server_name: &str) -> bool {
    let Some(parts) = parse_cert_der_parts(leaf_der) else {
        return false;
    };
    let clean = server_name
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']');
    if clean.is_empty() {
        return false;
    }
    let ip_query: Option<std::net::IpAddr> = clean.parse().ok();

    const OID_SAN: &[u8] = &[0x55, 0x1D, 0x11];
    let mut saw_san = false;
    for (oid, _, val) in &parts.extensions {
        if *oid != OID_SAN {
            continue;
        }
        saw_san = true;
        let Some((seq, _)) = parse_der_item(val) else {
            continue;
        };
        if seq.tag != 0x30 {
            continue;
        }
        let Some(entries) = der_item_children(seq.val) else {
            continue;
        };
        for entry in entries {
            match entry.tag {
                0x82 if ip_query.is_none() => {
                    if let Ok(dns) = std::str::from_utf8(entry.val) {
                        if dns_pattern_matches(dns, clean) {
                            return true;
                        }
                    }
                }
                0x87 => {
                    if let Some(ip) = ip_query {
                        let matched = match (ip, entry.val.len()) {
                            (std::net::IpAddr::V4(v4), 4) => v4.octets() == entry.val,
                            (std::net::IpAddr::V6(v6), 16) => v6.octets() == entry.val,
                            _ => false,
                        };
                        if matched {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    if saw_san {
        return false;
    }
    // Fallback to Subject CN when no SAN extension exists.
    if let Some(cn) = wintrust::extract_dn_attr(parts.subject_raw, &[0x55, 0x04, 0x03]) {
        if ip_query.is_some() {
            return cn.trim().eq_ignore_ascii_case(clean);
        }
        return dns_pattern_matches(&cn, clean);
    }
    false
}

pub(super) fn verify_cert_sig_with_spki(
    child: &CertDerParts<'_>,
    spki_alg_inner: &[u8],
    spki_key_bits: &[u8],
) -> bool {
    let provider = net_tls::crypto_provider();
    for alg in provider.signature_verification_algorithms.all {
        if alg.public_key_alg_id().as_ref() == spki_alg_inner
            && alg.signature_alg_id().as_ref() == child.sig_alg_inner
            && alg
                .verify_signature(spki_key_bits, child.tbs_raw, child.sig_bits)
                .is_ok()
        {
            return true;
        }
    }
    false
}

/// Verify cryptographic chain signatures from `chain_der[0]` up to a trusted root
/// without requiring TLS `serverAuth` EKU (used by `CERT_CHAIN_POLICY_BASE` and
/// `CERT_CHAIN_POLICY_AUTHENTICODE`).
/// Verified cryptographic chain endpoints returned by `verify_chain_to_root`.
pub(super) struct VerifiedChainEndpoints {
    /// Inner DER bytes of the terminating trusted root's `SubjectPublicKeyInfo` (without outer 0x30 tag/len).
    root_spki_inner: Vec<u8>,
    /// Inner DER bytes of the issuing CA's `SubjectPublicKeyInfo` that cryptographically signed
    /// the end-entity leaf `chain_der[0]` (or the leaf's own SPKI if self-signed root).
    issuing_ca_spki_inner: Vec<u8>,
}

/// Official Microsoft Root Certificate Authority `SubjectPublicKeyInfo` SHA-256 public-key
/// fingerprints (full DER SEQUENCE `0x30 ...` of SPKI) checked by `CERT_CHAIN_POLICY_MICROSOFT_ROOT`.
/// Derived directly from Microsoft's published root CAs (not subject display names).
const MICROSOFT_ROOT_SPKI_SHA256: &[[u8; 32]] = &[
    // Microsoft RSA Root Certificate Authority 2017
    [
        0xB2, 0xF7, 0x29, 0x8B, 0x52, 0xBF, 0x2C, 0x3C, 0xAC, 0x4D, 0xDF, 0xE7, 0x2D, 0xE4, 0xD6,
        0x82, 0xAC, 0x58, 0x95, 0x75, 0x95, 0x98, 0x2F, 0x2B, 0x62, 0x30, 0x1A, 0xF5, 0x97, 0xC6,
        0x99, 0xC5,
    ],
    // Microsoft ECC Root Certificate Authority 2017
    [
        0x35, 0xF5, 0x3C, 0xE1, 0x26, 0x46, 0x11, 0xE0, 0x33, 0x40, 0xFE, 0x37, 0xE1, 0xEC, 0x7D,
        0x4C, 0xC9, 0x86, 0xC5, 0x61, 0x3D, 0xCA, 0x70, 0xFD, 0x04, 0xAA, 0x44, 0x54, 0x5F, 0x2D,
        0xAF, 0x28,
    ],
];

fn spki_inner_sha256(spki_inner: &[u8]) -> [u8; 32] {
    let full = super::asn1::encode_der_tlv(0x30, spki_inner);
    let h = crate::misc::bcrypt::Digest::Sha256.hash(&full);
    let mut out = [0u8; 32];
    if h.len() == 32 {
        out.copy_from_slice(&h);
    }
    out
}

/// Parse X.509 `BasicConstraints` (`2.5.29.19`) extension from a DER certificate into
/// `(is_ca, optional_path_len_constraint)`. Returns `None` if the extension is absent or invalid.
fn parse_basic_constraints(der: &[u8]) -> Option<(bool, Option<u32>)> {
    const OID_BC: &[u8] = &[0x55, 0x1D, 0x13];
    let parts = parse_cert_der_parts(der)?;
    let (_, _, val) = parts.extensions.iter().find(|(oid, _, _)| *oid == OID_BC)?;
    let (seq, _) = parse_der_item(val)?;
    if seq.tag != 0x30 {
        return None;
    }
    let kids = der_item_children(seq.val)?;
    let mut is_ca = false;
    let mut path_len = None;
    for k in kids {
        if k.tag == 0x01 && !k.val.is_empty() {
            is_ca = k.val[0] != 0;
        } else if k.tag == 0x02 && !k.val.is_empty() && k.val.len() <= 4 {
            let mut n = 0u32;
            for &b in k.val {
                n = (n << 8) | b as u32;
            }
            path_len = Some(n);
        }
    }
    Some((is_ca, path_len))
}

/// Returns `true` if `parts` carries `NameConstraints` (`2.5.29.30`) or any critical extension
/// not enforced by the manual signature-walking fallback, per RFC 5280 §4.2 & §4.2.1.10.
fn cert_has_unhandled_constraints(parts: &CertDerParts<'_>) -> bool {
    const OID_SKI: &[u8] = &[0x55, 0x1D, 0x0E];
    const OID_KEY_USAGE: &[u8] = &[0x55, 0x1D, 0x0F];
    const OID_SAN: &[u8] = &[0x55, 0x1D, 0x11];
    const OID_BC: &[u8] = &[0x55, 0x1D, 0x13];
    const OID_NAME_CONSTRAINTS: &[u8] = &[0x55, 0x1D, 0x1E];
    const OID_AKI: &[u8] = &[0x55, 0x1D, 0x23];
    const OID_EKU: &[u8] = &[0x55, 0x1D, 0x25];

    for &(oid, is_critical, _) in &parts.extensions {
        if oid == OID_NAME_CONSTRAINTS {
            return true;
        }
        if is_critical
            && !matches!(
                oid,
                OID_BC | OID_KEY_USAGE | OID_EKU | OID_SKI | OID_AKI | OID_SAN
            )
        {
            return true;
        }
    }
    false
}

/// Enforce RFC 5280 CA constraints on an intermediate certificate acting as an issuer:
/// - `BasicConstraints` (`2.5.29.19`) MUST be present with `cA = TRUE`.
/// - If `pathLenConstraint` is present, `intermediates_below` must be `<= path_len`.
/// - If `KeyUsage` (`2.5.29.15`) is present, bit 5 (`keyCertSign`, mask `0x04` in byte 0) MUST be set.
/// - Per RFC 5280 §4.2 & §4.2.1.10, reject any intermediate carrying `NameConstraints` (`2.5.29.30`)
///   or any unsupported critical extension so the fallback path validator never overrides a
///   name-constraint or critical-extension failure from WebPKI / SecTrust.
/// - If `required_eku` is specified and the intermediate carries a non-empty EKU extension,
///   it must permit `required_eku` or `anyExtendedKeyUsage`.
pub(super) fn intermediate_is_valid_ca(
    der: &[u8],
    intermediates_below: usize,
    required_eku: Option<&[u8]>,
) -> bool {
    const OID_KEY_USAGE: &[u8] = &[0x55, 0x1D, 0x0F];
    const OID_ANY_EKU: &[u8] = &[0x55, 0x1D, 0x25, 0x00];

    let Some((is_ca, path_len)) = parse_basic_constraints(der) else {
        return false;
    };
    if !is_ca {
        return false;
    }
    if let Some(max_len) = path_len {
        if intermediates_below > max_len as usize {
            return false;
        }
    }
    let Some(parts) = parse_cert_der_parts(der) else {
        return false;
    };
    if cert_has_unhandled_constraints(&parts) {
        return false;
    }
    if let Some((_, _, val)) = parts.extensions.iter().find(|(oid, _, _)| *oid == OID_KEY_USAGE) {
        if let Some((bs, _)) = parse_der_item(val) {
            // KeyUsage BIT STRING: byte 0 = unused_bits, byte 1 = bits 0..7 (bit 5 = keyCertSign = 0x04)
            if bs.tag != 0x03 || bs.val.len() < 2 || (bs.val[1] & 0x04) == 0 {
                return false;
            }
        } else {
            return false;
        }
    }
    if let Some(req_eku) = required_eku {
        if let Some(oids) = cert_eku_oids(der) {
            if !oids.is_empty()
                && !oids
                    .iter()
                    .any(|o| o.as_slice() == req_eku || o.as_slice() == OID_ANY_EKU)
            {
                return false;
            }
        }
    }
    true
}

fn verify_chain_to_root(
    chain_der: &[Vec<u8>],
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
) -> Option<VerifiedChainEndpoints> {
    verify_chain_to_root_with_eku(chain_der, exclusive_roots_der, extra_roots_der, None)
}

fn verify_chain_to_root_with_eku(
    chain_der: &[Vec<u8>],
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
    required_eku: Option<&[u8]>,
) -> Option<VerifiedChainEndpoints> {
    verify_chain_to_root_with_eku_and_exclusions(
        chain_der,
        exclusive_roots_der,
        extra_roots_der,
        required_eku,
        macos_excluded_root_spkis(),
    )
}

pub(super) fn verify_chain_to_root_with_eku_and_exclusions(
    chain_der: &[Vec<u8>],
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
    required_eku: Option<&[u8]>,
    excluded_spkis: &[Vec<u8>],
) -> Option<VerifiedChainEndpoints> {
    if chain_der.is_empty() {
        return None;
    }
    // Collect candidate trust anchors as (subject_inner, spki_inner, spki_alg_inner, spki_key_bits).
    let mut root_parts: Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)> = Vec::new();
    let push_root_der = |dst: &mut Vec<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>,
                         der: &[u8],
                         filter_excluded: bool| {
        if let Some(p) = parse_cert_der_parts(der) {
            // Do not bypass NameConstraints or unknown critical extensions on trust anchors.
            if cert_has_unhandled_constraints(&p) {
                return;
            }
            if filter_excluded && spki_is_excluded(p.spki_inner, excluded_spkis) {
                return;
            }
            dst.push((
                p.subject_inner.to_vec(),
                p.spki_inner.to_vec(),
                p.spki_alg_inner.to_vec(),
                p.spki_key_bits.to_vec(),
            ));
        }
    };
    if let Some(excl) = exclusive_roots_der {
        for der in excl {
            push_root_der(&mut root_parts, der, false);
        }
    } else {
        for der in extra_roots_der {
            push_root_der(&mut root_parts, der, false);
        }
        for der in macos_system_roots_der() {
            push_root_der(&mut root_parts, der, true);
        }
        for ta in webpki_roots::TLS_SERVER_ROOTS {
            if ta.name_constraints.is_some()
                || spki_is_excluded(ta.subject_public_key_info.as_ref(), excluded_spkis)
            {
                continue;
            }
            if let Ok(kids) = der_item_children(ta.subject_public_key_info.as_ref()).ok_or(()) {
                if kids.len() >= 2
                    && kids[0].tag == 0x30
                    && kids[1].tag == 0x03
                    && !kids[1].val.is_empty()
                {
                    root_parts.push((
                        ta.subject.as_ref().to_vec(),
                        ta.subject_public_key_info.as_ref().to_vec(),
                        kids[0].val.to_vec(),
                        kids[1].val[1..].to_vec(),
                    ));
                }
            }
        }
    }
    if root_parts.is_empty() {
        return None;
    }

    let parsed: Vec<Option<CertDerParts<'_>>> = chain_der
        .iter()
        .map(|d| parse_cert_der_parts(d))
        .collect();
    let mut cur = parsed[0].as_ref()?;
    // Per RFC 5280 §4.2, do not bypass unknown critical extensions on the leaf certificate.
    if cert_has_unhandled_constraints(cur) {
        return None;
    }
    let mut issuing_ca_spki: Option<Vec<u8>> = None;

    // Walk up to 16 chain links looking for a trusted root.
    // Track non-self-issued intermediates separately per RFC 5280 §4.2.1.9 & §6.1.4(l)
    // so self-issued CA rollover certificates do not consume pathLenConstraint.
    let mut visited = vec![false; chain_der.len()];
    visited[0] = true;
    let mut non_self_issued_intermediates = 0usize;
    for step in 0..16 {
        // Direct match: cur itself is a trusted root (same subject + SPKI).
        if let Some((_, spki, _, _)) = root_parts.iter().find(|(subj, spki, _, _)| {
            subj.as_slice() == cur.subject_inner && spki.as_slice() == cur.spki_inner
        }) {
            return Some(VerifiedChainEndpoints {
                root_spki_inner: spki.clone(),
                issuing_ca_spki_inner: issuing_ca_spki.unwrap_or_else(|| spki.clone()),
            });
        }
        // Signed directly by a trusted root?
        if let Some((_, spki, _, _)) = root_parts.iter().find(|(subj, _, alg, key)| {
            subj.as_slice() == cur.issuer_inner && verify_cert_sig_with_spki(cur, alg, key)
        }) {
            if step == 0 {
                issuing_ca_spki = Some(spki.clone());
            }
            return Some(VerifiedChainEndpoints {
                root_spki_inner: spki.clone(),
                issuing_ca_spki_inner: issuing_ca_spki.unwrap_or_else(|| spki.clone()),
            });
        }
        // Otherwise look for an intermediate CA issuer in chain_der.
        // Reject end-entity certificates (cA=FALSE / missing BasicConstraints / missing keyCertSign /
        // violated pathLenConstraint / incompatible intermediate EKU) so a leaf cert can never forge
        // a subordinate chain.
        let mut next_idx = None;
        for (i, cand) in parsed.iter().enumerate() {
            if visited[i] {
                continue;
            }
            if let Some(p) = cand.as_ref() {
                if p.subject_inner == cur.issuer_inner
                    && intermediate_is_valid_ca(
                        &chain_der[i],
                        non_self_issued_intermediates,
                        required_eku,
                    )
                    && verify_cert_sig_with_spki(cur, p.spki_alg_inner, p.spki_key_bits)
                {
                    next_idx = Some(i);
                    break;
                }
            }
        }
        let ni = next_idx?;
        let next_cert = parsed[ni].as_ref().unwrap();
        if step == 0 {
            issuing_ca_spki = Some(next_cert.spki_inner.to_vec());
        }
        if next_cert.subject_inner != next_cert.issuer_inner {
            non_self_issued_intermediates += 1;
        }
        visited[ni] = true;
        cur = next_cert;
    }
    None
}

pub(super) fn verify_chain_signatures_to_roots(
    chain_der: &[Vec<u8>],
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
) -> bool {
    verify_chain_to_root(chain_der, exclusive_roots_der, extra_roots_der).is_some()
}

/// Map a `rustls::Error` from `WebPkiServerVerifier::verify_server_cert` to a Windows `CERT_E_*` code.
pub(crate) fn map_rustls_cert_error(err: &rustls::Error, ignore_hostname: bool) -> u32 {
    match err {
        rustls::Error::InvalidCertificate(cert_err) => match cert_err {
            rustls::CertificateError::Expired
            | rustls::CertificateError::NotValidYet
            | rustls::CertificateError::ExpiredContext { .. }
            | rustls::CertificateError::NotValidYetContext { .. } => CERT_E_EXPIRED,
            rustls::CertificateError::NotValidForName
            | rustls::CertificateError::NotValidForNameContext { .. } => {
                if ignore_hostname {
                    0
                } else {
                    CERT_E_CN_NO_MATCH
                }
            }
            rustls::CertificateError::Revoked => CERT_E_REVOKED,
            rustls::CertificateError::InvalidPurpose
            | rustls::CertificateError::InvalidPurposeContext { .. } => CERT_E_WRONG_USAGE,
            _ => CERT_E_UNTRUSTEDROOT,
        },
        _ => CERT_E_CHAINING,
    }
}

fn cert_error_to_trust_status(cert_error: u32) -> u32 {
    match cert_error {
        0 => 0,
        CERT_E_EXPIRED => CERT_TRUST_IS_NOT_TIME_VALID,
        CERT_E_REVOKED => CERT_TRUST_IS_REVOKED,
        CERT_E_UNTRUSTEDROOT => CERT_TRUST_IS_UNTRUSTED_ROOT,
        CERT_E_WRONG_USAGE => CERT_TRUST_IS_NOT_VALID_FOR_USAGE,
        CERT_E_CHAINING => CERT_TRUST_IS_PARTIAL_CHAIN,
        CRYPT_E_NO_REVOCATION_CHECK => CERT_TRUST_REVOCATION_STATUS_UNKNOWN,
        CRYPT_E_REVOCATION_OFFLINE => {
            CERT_TRUST_REVOCATION_STATUS_UNKNOWN | CERT_TRUST_IS_OFFLINE_REVOCATION
        }
        // Hostname mismatch is evaluated at SSL policy time, not raw chain status.
        CERT_E_CN_NO_MATCH => 0,
        _ => CERT_TRUST_IS_UNTRUSTED_ROOT,
    }
}

/// Verify an X.509 certificate chain (`[leaf, intermediates...]`) using `rustls`'s WebPKI
/// verifier against either an exclusive root store or the embedded `webpki-roots` bundle.
/// Returns `0` on success or a Windows `CERT_E_*` error code on failure.
fn verify_chain_webpki_client_at(
    chain_der: &[Vec<u8>],
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
    at_time: UnixTime,
) -> u32 {
    let Some((leaf, intermediates)) = chain_der.split_first() else {
        return CERT_E_CHAINING;
    };
    let roots = if let Some(excl) = exclusive_roots_der {
        let mut s = rustls::RootCertStore::empty();
        for der in excl {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    } else if extra_roots_der.is_empty() {
        combined_default_root_store()
    } else {
        let mut s = (*combined_default_root_store()).clone();
        for der in extra_roots_der {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    };
    if roots.is_empty() {
        return CERT_E_UNTRUSTEDROOT;
    }
    let verifier = match rustls::server::WebPkiClientVerifier::builder_with_provider(
        roots,
        net_tls::crypto_provider(),
    )
    .build()
    {
        Ok(v) => v,
        Err(_) => return CERT_E_UNTRUSTEDROOT,
    };
    let ee = CertificateDer::from(leaf.as_slice());
    let inters: Vec<CertificateDer<'_>> = intermediates
        .iter()
        .map(|d| CertificateDer::from(d.as_slice()))
        .collect();
    match verifier.verify_client_cert(&ee, &inters, at_time) {
        Ok(_) => 0,
        Err(ref e) => map_rustls_cert_error(e, true),
    }
}

fn verify_chain_webpki_at(
    chain_der: &[Vec<u8>],
    server_name: Option<&str>,
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
    ignore_hostname: bool,
    at_time: UnixTime,
) -> u32 {
    let Some((leaf, intermediates)) = chain_der.split_first() else {
        return CERT_E_CHAINING;
    };
    let roots = if let Some(excl) = exclusive_roots_der {
        let mut s = rustls::RootCertStore::empty();
        for der in excl {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    } else if extra_roots_der.is_empty() {
        combined_default_root_store()
    } else {
        let mut s = (*combined_default_root_store()).clone();
        for der in extra_roots_der {
            let _ = s.add(CertificateDer::from(der.as_slice()));
        }
        Arc::new(s)
    };
    if roots.is_empty() {
        return CERT_E_UNTRUSTEDROOT;
    }
    let verifier = match net_tls::build_webpki_verifier(roots) {
        Ok(v) => v,
        Err(_) => return CERT_E_UNTRUSTEDROOT,
    };

    let (sname, effective_ignore_host) = match server_name.map(str::trim).filter(|s| !s.is_empty())
    {
        Some(host) => {
            let clean = host.trim_start_matches('[').trim_end_matches(']');
            match ServerName::try_from(clean.to_string()) {
                Ok(sn) => (sn, ignore_hostname),
                Err(_) => {
                    if ignore_hostname {
                        (
                            ServerName::try_from("localhost".to_string()).unwrap(),
                            true,
                        )
                    } else {
                        return CERT_E_CN_NO_MATCH;
                    }
                }
            }
        }
        None => (
            ServerName::try_from("localhost".to_string()).unwrap(),
            true,
        ),
    };

    let end_entity = CertificateDer::from(leaf.as_slice());
    let inters: Vec<CertificateDer<'_>> = intermediates
        .iter()
        .map(|d| CertificateDer::from(d.as_slice()))
        .collect();
    match verifier.verify_server_cert(&end_entity, &inters, &sname, &[], at_time) {
        Ok(_) => 0,
        Err(ref e) => map_rustls_cert_error(e, effective_ignore_host),
    }
}

/// Evaluate WebPKI chain trust for a manually-validated TLS peer chain against
/// the embedded `webpki-roots` bundle plus optional `SCHANNEL_CRED.hRootStore` roots.
pub(crate) fn verify_peer_chain_error(
    chain_der: &[Vec<u8>],
    server_name: Option<&str>,
    extra_roots_der: &[Vec<u8>],
    ignore_hostname: bool,
) -> u32 {
    verify_chain_webpki_at(
        chain_der,
        server_name,
        None,
        extra_roots_der,
        ignore_hostname,
        UnixTime::now(),
    )
}

// ---------------------------------------------------------------------------
// Certificate Chain Engine & Policy Verification implementations
// ---------------------------------------------------------------------------

#[repr(C)]
#[allow(dead_code)]
pub(super) struct CertChainPolicyPara {
    pub(super) cb_size: u32,
    pub(super) dw_flags: u32,
    pub(super) pv_extra_policy_para: *const c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct CertChainPolicyStatus {
    pub(super) cb_size: u32,
    pub(super) dw_error: u32,
    pub(super) l_chain_index: i32,
    pub(super) l_element_index: i32,
    pub(super) pv_extra_policy_status: *mut c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub(super) struct SslExtraCertChainPolicyPara {
    pub(super) cb_size: u32,
    pub(super) dw_auth_type: u32,
    pub(super) fdw_checks: u32,
    pub(super) pwsz_server_name: *const u16,
}

fn trust_status_to_cert_error(dw_error_status: u32) -> u32 {
    if dw_error_status == 0 {
        0
    } else if (dw_error_status & CERT_TRUST_IS_REVOKED) != 0 {
        CERT_E_REVOKED
    } else if (dw_error_status & CERT_TRUST_IS_NOT_TIME_VALID) != 0 {
        CERT_E_EXPIRED
    } else if (dw_error_status & CERT_TRUST_IS_UNTRUSTED_ROOT) != 0 {
        CERT_E_UNTRUSTEDROOT
    } else if (dw_error_status & CERT_TRUST_IS_NOT_VALID_FOR_USAGE) != 0 {
        CERT_E_WRONG_USAGE
    } else if (dw_error_status & CERT_TRUST_IS_PARTIAL_CHAIN) != 0 {
        CERT_E_CHAINING
    } else if (dw_error_status & CERT_TRUST_IS_OFFLINE_REVOCATION) != 0 {
        CRYPT_E_REVOCATION_OFFLINE
    } else if (dw_error_status & CERT_TRUST_REVOCATION_STATUS_UNKNOWN) != 0 {
        CRYPT_E_NO_REVOCATION_CHECK
    } else {
        CERT_E_UNTRUSTEDROOT
    }
}

unsafe fn extract_external_chain(
    p_chain_context: *const c_void,
) -> Option<(Vec<Vec<u8>>, Vec<(u64, u64)>, u32)> {
    let ctx = std::ptr::read_unaligned(p_chain_context as *const CertChainContext);
    if ctx.c_chain == 0 || ctx.rgp_chain.is_null() {
        return None;
    }
    let simple_ptr = std::ptr::read_unaligned(ctx.rgp_chain);
    if simple_ptr.is_null() {
        return None;
    }
    let simple = std::ptr::read_unaligned(simple_ptr);
    if simple.c_element == 0 || simple.rgp_element.is_null() {
        return None;
    }
    let mut ders = Vec::with_capacity(simple.c_element as usize);
    let mut times = Vec::with_capacity(simple.c_element as usize);
    for i in 0..simple.c_element as usize {
        let el_ptr = std::ptr::read_unaligned(simple.rgp_element.add(i));
        if el_ptr.is_null() {
            continue;
        }
        let el = std::ptr::read_unaligned(el_ptr);
        if el.p_cert_context.is_null() {
            continue;
        }
        let cc = std::ptr::read_unaligned(el.p_cert_context);
        if cc.pb_cert_encoded.is_null() || cc.cb_cert_encoded == 0 {
            continue;
        }
        let der =
            std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize).to_vec();
        let (nb, na) = if !cc.p_cert_info.is_null() {
            let ci = std::ptr::read_unaligned(cc.p_cert_info);
            (ci.not_before, ci.not_after)
        } else {
            (0, 0)
        };
        ders.push(der);
        times.push((nb, na));
    }
    if ders.is_empty() {
        return None;
    }
    Some((
        ders,
        times,
        trust_status_to_cert_error(ctx.trust_status.dw_error_status),
    ))
}

pub(super) unsafe fn cert_create_certificate_chain_engine_impl(
    p_config: *const c_void,
    ph_chain_engine: *mut *mut c_void,
) -> BOOL {
    if p_config.is_null() || ph_chain_engine.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let base = p_config as *const u8;
    let cb_size = std::ptr::read_unaligned(base as *const u32);
    if cb_size < 0x40 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let h_restricted_root = std::ptr::read_unaligned(base.add(0x08) as *const *mut c_void);
    let c_additional = std::ptr::read_unaligned(base.add(0x20) as *const u32);
    let rgh_additional = std::ptr::read_unaligned(base.add(0x28) as *const *const *mut c_void);
    let h_exclusive_root = if cb_size >= 0x48 {
        std::ptr::read_unaligned(base.add(0x40) as *const *mut c_void)
    } else {
        std::ptr::null_mut()
    };

    let exclusive_roots_der = if !h_exclusive_root.is_null() {
        Some(store_certs_der(h_exclusive_root))
    } else if !h_restricted_root.is_null() {
        Some(store_certs_der(h_restricted_root))
    } else {
        None
    };

    let mut additional_certs_der = Vec::new();
    if c_additional > 0 && !rgh_additional.is_null() {
        for i in 0..c_additional as usize {
            let store_h = std::ptr::read_unaligned(rgh_additional.add(i));
            additional_certs_der.extend(store_certs_der(store_h));
        }
    }

    let id = ENGINES.insert(RegisteredEngine {
        exclusive_roots_der,
        additional_certs_der,
    });
    std::ptr::write_unaligned(ph_chain_engine, id as *mut c_void);
    TRUE
}

pub(super) fn cert_free_certificate_chain_engine_impl(engine: *mut c_void) {
    // HCCE_CURRENT_USER (NULL / 0) and HCCE_LOCAL_MACHINE (1) are predefined pseudo-handles.
    let raw = engine as usize;
    if raw == 0 || raw == 1 {
        return;
    }
    if ENGINES.remove(raw).is_none() {
        log::debug!("CertFreeCertificateChainEngine({engine:p}): unknown/already-freed engine");
    }
}

/// Evaluate certificate revocation for a DER chain (`[leaf, intermediates...]`) using macOS/iOS
/// `Security.framework` revocation policy. Returns `0` on positive non-revoked verification,
/// `CERT_E_REVOKED`, `CRYPT_E_NO_REVOCATION_CHECK`, or `CRYPT_E_REVOCATION_OFFLINE`.
pub(crate) fn check_chain_revocation_error(
    chain_der: &[Vec<u8>],
    exclusive_roots_der: Option<&[Vec<u8>]>,
    extra_roots_der: &[Vec<u8>],
    verify_unix_secs: Option<f64>,
    revocation_flags: u32,
) -> u32 {
    if (revocation_flags & CERT_CHAIN_REVOCATION_CHECK_MASK) == 0 {
        return 0;
    }
    let cache_only = (revocation_flags
        & (CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY | CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL))
        != 0;
    let end_cert_only = (revocation_flags & CERT_CHAIN_REVOCATION_CHECK_END_CERT) != 0
        && (revocation_flags
            & (CERT_CHAIN_REVOCATION_CHECK_CHAIN | CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT))
            == 0;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    let outcome = apple_trust::evaluate_sec_trust_revocation(
        chain_der,
        exclusive_roots_der,
        extra_roots_der,
        verify_unix_secs,
        cache_only,
        end_cert_only,
    );
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    let outcome = {
        let _ = (
            chain_der,
            exclusive_roots_der,
            extra_roots_der,
            verify_unix_secs,
            cache_only,
            end_cert_only,
        );
        0u8
    };
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    match outcome {
        apple_trust::RevocationCheckResult::VerifiedGood => {
            // Apple Security.framework checks leaf + intermediate CAs, not self-signed roots.
            // CERT_CHAIN_REVOCATION_CHECK_CHAIN explicitly requires checking the root as well.
            if (revocation_flags & CERT_CHAIN_REVOCATION_CHECK_CHAIN) != 0 {
                CRYPT_E_NO_REVOCATION_CHECK
            } else {
                0
            }
        }
        apple_trust::RevocationCheckResult::Revoked => CERT_E_REVOKED,
        apple_trust::RevocationCheckResult::OfflineOrUnknown => CRYPT_E_REVOCATION_OFFLINE,
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _ = outcome;
        CRYPT_E_REVOCATION_OFFLINE
    }
}

/// Extract Certificate Policies extension (`2.5.29.32`) OID strings from a DER certificate.
fn cert_policy_oids(der: &[u8]) -> Vec<String> {
    const OID_CERT_POLICIES: &[u8] = &[0x55, 0x1D, 0x20];
    let Some(parts) = parse_cert_der_parts(der) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (oid, _, val) in &parts.extensions {
        if *oid != OID_CERT_POLICIES {
            continue;
        }
        if let Some((seq, _)) = parse_der_item(val) {
            if let Some(items) = der_item_children(seq.val) {
                for pi in items {
                    if pi.tag == 0x30 {
                        if let Some(kids) = der_item_children(pi.val) {
                            if let Some(first) = kids.first() {
                                if first.tag == 0x06 {
                                    out.push(wintrust::decode_oid_str(first.val));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

/// Returns true if the certificate uses a strong signature algorithm accepted by Windows
/// `szOID_CERT_STRONG_SIGN_OS_1` / `OS_CURRENT` / `OS_SERVER_1` (SHA-256/384/512 with RSA,
/// RSASSA-PSS, or ECDSA, or Ed25519/Ed448).
fn cert_has_strong_signature_alg(der: &[u8]) -> bool {
    const OID_RSASSA_PSS: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0A];
    const OID_SHA256: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01];
    const OID_SHA384: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x02];
    const OID_SHA512: &[u8] = &[0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03];

    let Some(parts) = parse_cert_der_parts(der) else {
        return false;
    };
    let Some((sig_oid, rest)) = parse_der_item(parts.sig_alg_inner) else {
        return false;
    };
    if sig_oid.tag != 0x06 {
        return false;
    }
    if sig_oid.val == OID_RSASSA_PSS {
        // RFC 4055 §3.1: omitted hashAlgorithm defaults to SHA-1 (weak).
        let Some((params_seq, _)) = parse_der_item(rest) else {
            return false;
        };
        if params_seq.tag != 0x30 {
            return false;
        }
        let Some(fields) = der_item_children(params_seq.val) else {
            return false;
        };
        for f in fields {
            if f.tag == 0xA0 {
                if let Some((alg_seq, _)) = parse_der_item(f.val) {
                    if alg_seq.tag == 0x30 {
                        if let Some((hash_oid, _)) = parse_der_item(alg_seq.val) {
                            if hash_oid.tag == 0x06 {
                                return matches!(
                                    hash_oid.val,
                                    OID_SHA256 | OID_SHA384 | OID_SHA512
                                );
                            }
                        }
                    }
                }
            }
        }
        return false;
    }
    matches!(
        sig_oid.val,
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B] // sha256WithRSAEncryption
            | [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0C] // sha384WithRSAEncryption
            | [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0D] // sha512WithRSAEncryption
            | [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02] // ecdsa-with-SHA256
            | [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x03] // ecdsa-with-SHA384
            | [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x04] // ecdsa-with-SHA512
            | [0x2B, 0x65, 0x70] // id-Ed25519
            | [0x2B, 0x65, 0x71] // id-Ed448
    )
}

/// Returns true if the certificate's SubjectPublicKeyInfo meets Windows strong-sign key-strength
/// requirements (`szOID_CERT_STRONG_SIGN_OS_1` / `OS_CURRENT` / `OS_SERVER_1`):
/// RSA >= 2047 bits, ECDSA on P-256 / P-384 / P-521, or Ed25519 / Ed448.
fn cert_has_strong_public_key(der: &[u8]) -> bool {
    const OID_RSA_ENCRYPTION: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01];
    const OID_RSASSA_PSS: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0A];
    const OID_EC_PUBLIC_KEY: &[u8] = &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01];
    const OID_SECP256R1: &[u8] = &[0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07];
    const OID_SECP384R1: &[u8] = &[0x2B, 0x81, 0x04, 0x00, 0x22];
    const OID_SECP521R1: &[u8] = &[0x2B, 0x81, 0x04, 0x00, 0x23];
    const OID_ED25519: &[u8] = &[0x2B, 0x65, 0x70];
    const OID_ED448: &[u8] = &[0x2B, 0x65, 0x71];

    let Some(parts) = parse_cert_der_parts(der) else {
        return false;
    };
    let Some((alg_oid, alg_rest)) = parse_der_item(parts.spki_alg_inner) else {
        return false;
    };
    if alg_oid.tag != 0x06 {
        return false;
    }
    match alg_oid.val {
        OID_RSA_ENCRYPTION | OID_RSASSA_PSS => {
            let Some((rsa_seq, _)) = parse_der_item(parts.spki_key_bits) else {
                return false;
            };
            if rsa_seq.tag != 0x30 {
                return false;
            }
            let Some(kids) = der_item_children(rsa_seq.val) else {
                return false;
            };
            if kids.len() < 2 || kids[0].tag != 0x02 || kids[1].tag != 0x02 {
                return false;
            }
            let mut modulus = kids[0].val;
            if modulus.is_empty() || (modulus[0] & 0x80) != 0 {
                return false;
            }
            while modulus.len() > 1 && modulus[0] == 0 {
                modulus = &modulus[1..];
            }
            if modulus[0] == 0 {
                return false;
            }
            let bit_len = (modulus.len() - 1) * 8 + (8 - modulus[0].leading_zeros() as usize);
            bit_len >= 2047
        }
        OID_EC_PUBLIC_KEY => {
            let Some((curve_oid, _)) = parse_der_item(alg_rest) else {
                return false;
            };
            curve_oid.tag == 0x06
                && matches!(curve_oid.val, OID_SECP256R1 | OID_SECP384R1 | OID_SECP521R1)
        }
        OID_ED25519 | OID_ED448 => true,
        _ => false,
    }
}

/// Windows SDK `wincrypt.h` `CERT_STRONG_SIGN_PARA.dwInfoChoice` discriminators and flags.
const CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE: u32 = 1;
const CERT_STRONG_SIGN_OID_INFO_CHOICE: u32 = 2;
const CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG: u32 = 0x0000_0001;
const CERT_TRUST_HAS_WEAK_SIGNATURE: u32 = 0x0010_0000;
const TRUST_E_CERT_SIGNATURE: u32 = 0x8009_6004;

fn cert_signed_by_parts(child: &CertDerParts<'_>, parent: &CertDerParts<'_>) -> bool {
    verify_cert_sig_with_spki(child, parent.spki_alg_inner, parent.spki_key_bits)
        || verify_rsa_cert_sig_fallback(child, parent.spki_alg_inner, parent.spki_key_bits)
}

pub(crate) fn verify_cert_signed_by_der(child_der: &[u8], parent_der: &[u8]) -> bool {
    match (parse_cert_der_parts(child_der), parse_cert_der_parts(parent_der)) {
        (Some(c), Some(p)) => cert_signed_by_parts(&c, &p),
        _ => false,
    }
}

/// Resolve ONLY cryptographically signature-verified issuers `[leaf, intermediate..., optional_root]`
/// for building `CERT_CHAIN_CONTEXT.rgpChain[0]->rgpElement`. Unrelated or unverified DN-only
/// certificates from `hAdditionalStore` or PKCS#7 certificate bags are never emitted as chain elements.
fn resolve_verified_issuer_path(
    chain_der: &[Vec<u8>],
    root_pool: &[Vec<u8>],
    allow_default_macos: bool,
) -> Vec<Vec<u8>> {
    let Some(leaf) = chain_der.first() else {
        return Vec::new();
    };
    let mut path: Vec<Vec<u8>> = vec![leaf.clone()];
    let excluded_spkis = macos_excluded_root_spkis();
    for _ in 0..16 {
        let cur_der = path.last().unwrap().clone();
        let Some(cur_parts) = parse_cert_der_parts(&cur_der) else {
            break;
        };
        if cur_parts.subject_inner == cur_parts.issuer_inner
            && cert_signed_by_parts(&cur_parts, &cur_parts)
        {
            break;
        }
        let find_verified = |pool: &[Vec<u8>], filter_excluded: bool| -> Option<Vec<u8>> {
            for cand in pool {
                if path.iter().any(|p| p == cand) {
                    continue;
                }
                let Some(cp) = parse_cert_der_parts(cand) else {
                    continue;
                };
                if filter_excluded && spki_is_excluded(cp.spki_inner, excluded_spkis) {
                    continue;
                }
                if cp.subject_inner == cur_parts.issuer_inner
                    && cert_signed_by_parts(&cur_parts, &cp)
                {
                    return Some(cand.clone());
                }
            }
            None
        };
        if let Some(next) = find_verified(&chain_der[1..], false) {
            path.push(next);
            continue;
        }
        if let Some(root) = find_verified(root_pool, false) {
            path.push(root);
            break;
        }
        if allow_default_macos {
            if let Some(root) = find_verified(macos_system_roots_der(), true) {
                path.push(root);
                break;
            }
        }
        break;
    }
    path
}

/// Resolve the actual certification path `[leaf, intermediate..., optional_root]` starting from
/// `chain_der[0]`, selecting issuers from `chain_der[1..]` and `root_pool` by cryptographic
/// signature match (falling back to issuer/subject DN match if no signature-verified issuer
/// exists in the candidate pool). Unrelated certificates in `hAdditionalStore` are excluded.
fn resolve_selected_issuer_path<'a>(
    chain_der: &'a [Vec<u8>],
    root_pool: &'a [Vec<u8>],
) -> Vec<&'a [u8]> {
    let Some(leaf) = chain_der.first() else {
        return Vec::new();
    };
    let mut path: Vec<&'a [u8]> = vec![leaf.as_slice()];
    for _ in 0..16 {
        let cur = *path.last().unwrap();
        let Some(cur_parts) = parse_cert_der_parts(cur) else {
            break;
        };
        // Stop at a genuinely self-signed root, while allowing self-issued key-rollover
        // intermediates (subject == issuer but signed by a distinct parent cert) to keep walking.
        if cur_parts.subject_inner == cur_parts.issuer_inner
            && verify_cert_sig_with_spki(&cur_parts, cur_parts.spki_alg_inner, cur_parts.spki_key_bits)
        {
            break;
        }
        let find_verified = |pool: &'a [Vec<u8>]| -> Option<&'a [u8]> {
            for cand in pool {
                let cand_slice = cand.as_slice();
                if path.iter().any(|&p| p == cand_slice) {
                    continue;
                }
                let Some(cp) = parse_cert_der_parts(cand_slice) else {
                    continue;
                };
                if cp.subject_inner == cur_parts.issuer_inner
                    && verify_cert_sig_with_spki(&cur_parts, cp.spki_alg_inner, cp.spki_key_bits)
                {
                    return Some(cand_slice);
                }
            }
            None
        };
        let find_dn_only = |pool: &'a [Vec<u8>]| -> Option<&'a [u8]> {
            for cand in pool {
                let cand_slice = cand.as_slice();
                if path.iter().any(|&p| p == cand_slice) {
                    continue;
                }
                let Some(cp) = parse_cert_der_parts(cand_slice) else {
                    continue;
                };
                if cp.subject_inner == cur_parts.issuer_inner {
                    return Some(cand_slice);
                }
            }
            None
        };
        // Search BOTH pools for a cryptographically verified issuer before considering any
        // name-only fallback, so a DN-colliding unrelated cert in hAdditionalStore can never
        // shadow the genuine issuer in root_pool.
        if let Some(next) = find_verified(&chain_der[1..]) {
            path.push(next);
            continue;
        }
        if let Some(root) = find_verified(root_pool) {
            path.push(root);
            break;
        }
        if let Some(next) = find_dn_only(&chain_der[1..]) {
            path.push(next);
            continue;
        }
        if let Some(root) = find_dn_only(root_pool) {
            path.push(root);
        }
        break;
    }
    path
}

/// Evaluate `CERT_CHAIN_PARA` (`RequestedUsage`, `RequestedIssuancePolicy`, and `pStrongSignPara`)
/// against the selected certification path resolved from `chain_der` and `root_pool`.
/// Returns `Ok((trust_status_bits, chain_para_error))` or `Err(win32_err)`.
unsafe fn evaluate_cert_chain_para(
    p_chain_para: *const c_void,
    chain_der: &[Vec<u8>],
    root_pool: &[Vec<u8>],
) -> Result<(u32, u32), u32> {
    if p_chain_para.is_null() {
        return Ok((0, 0));
    }
    let base = p_chain_para as *const u8;
    let cb_size = std::ptr::read_unaligned(base as *const u32);
    if cb_size < 0x20 {
        return Err(err::ERROR_INVALID_PARAMETER);
    }

    let selected_path = resolve_selected_issuer_path(chain_der, root_pool);

    let read_usage_match = |off: usize| -> Result<Option<(u32, Vec<String>)>, u32> {
        let dw_type = std::ptr::read_unaligned(base.add(off) as *const u32);
        let c_usage = std::ptr::read_unaligned(base.add(off + 0x08) as *const u32) as usize;
        let rgpsz = std::ptr::read_unaligned(base.add(off + 0x10) as *const *const *const u8);
        if c_usage == 0 {
            return Ok(None);
        }
        if (dw_type != 0 && dw_type != 1) || rgpsz.is_null() {
            return Err(err::ERROR_INVALID_PARAMETER);
        }
        let mut oids = Vec::with_capacity(c_usage.min(64));
        for i in 0..c_usage {
            let p_oid = std::ptr::read_unaligned(rgpsz.add(i));
            if p_oid.is_null() {
                return Err(err::ERROR_INVALID_PARAMETER);
            }
            oids.push(crate::strings::from_ansi(p_oid));
        }
        Ok(Some((dw_type, oids)))
    };

    let mut trust_bits = 0u32;
    let mut para_err = 0u32;

    // 1. RequestedUsage (CERT_USAGE_MATCH @ +0x08) — intersect permitted EKUs across the
    //    entire selected issuer path before applying AND/OR, so an OR request requires at
    //    least one requested OID to be permitted end-to-end on every cert along the path.
    if let Some((match_type, req_ekus)) = read_usage_match(0x08)? {
        let cert_eku_sets: Vec<Option<Vec<String>>> = selected_path
            .iter()
            .map(|&der| {
                cert_eku_oids(der).and_then(|raw_oids| {
                    if raw_oids.is_empty() {
                        None
                    } else {
                        Some(
                            raw_oids
                                .iter()
                                .map(|o| wintrust::decode_oid_str(o))
                                .collect(),
                        )
                    }
                })
            })
            .collect();
        let path_authorizes_eku = |req: &str| -> bool {
            cert_eku_sets.iter().all(|opt_oids| match opt_oids {
                None => true,
                Some(oids) => {
                    oids.iter().any(|o| o == "2.5.29.37.0") || oids.iter().any(|o| o == req)
                }
            })
        };
        let ok = if match_type == 0 {
            // USAGE_MATCH_TYPE_AND
            req_ekus.iter().all(|r| path_authorizes_eku(r))
        } else {
            // USAGE_MATCH_TYPE_OR
            req_ekus.iter().any(|r| path_authorizes_eku(r))
        };
        if !ok {
            trust_bits |= CERT_TRUST_IS_NOT_VALID_FOR_USAGE;
            para_err = CERT_E_WRONG_USAGE;
        }
    }

    // 2. RequestedIssuancePolicy (CERT_USAGE_MATCH @ +0x20 when cbSize >= 0x38)
    //    Per RFC 5280 §6.1 / MSDN CERT_CHAIN_PARA: every non-root certificate along the selected
    //    issuer path must authorize the requested policy (explicitly or via anyPolicy 2.5.29.32.0
    //    unless inhibited). Policy mappings are not expanded; any unmapped/inhibited path fails closed.
    if cb_size >= 0x38 {
        if let Some((match_type, req_policies)) = read_usage_match(0x20)? {
            const OID_POLICY_MAPPINGS: &[u8] = &[0x55, 0x1D, 0x21]; // 2.5.29.33
            const OID_INHIBIT_ANY_POLICY: &[u8] = &[0x55, 0x1D, 0x36]; // 2.5.29.54

            // Exclude a trailing self-signed trust-anchor root (trust anchors are not required
            // to carry certificatePolicies per RFC 5280 §6.1).
            let non_root_len = if selected_path.len() > 1 {
                let last = *selected_path.last().unwrap();
                if parse_cert_der_parts(last).is_some_and(|p| p.subject_inner == p.issuer_inner) {
                    selected_path.len() - 1
                } else {
                    selected_path.len()
                }
            } else {
                selected_path.len()
            };
            let non_root_path = &selected_path[..non_root_len];

            let mut any_policy_inhibited = false;
            let mut has_unsupported_policy_ext = false;
            let mut cert_policy_sets: Vec<Vec<String>> = Vec::with_capacity(non_root_path.len());
            for &der in non_root_path {
                if let Some(parts) = parse_cert_der_parts(der) {
                    for (oid, _, _) in &parts.extensions {
                        if *oid == OID_POLICY_MAPPINGS {
                            has_unsupported_policy_ext = true;
                        } else if *oid == OID_INHIBIT_ANY_POLICY {
                            any_policy_inhibited = true;
                        }
                    }
                }
                cert_policy_sets.push(cert_policy_oids(der));
            }

            let path_authorizes_policy = |req: &str| -> bool {
                if cert_policy_sets.is_empty() || has_unsupported_policy_ext {
                    return false;
                }
                cert_policy_sets.iter().all(|oids| {
                    oids.iter().any(|o| o == req)
                        || (!any_policy_inhibited && oids.iter().any(|o| o == "2.5.29.32.0"))
                })
            };

            let ok = if match_type == 0 {
                // USAGE_MATCH_TYPE_AND
                req_policies.iter().all(|r| path_authorizes_policy(r))
            } else {
                // USAGE_MATCH_TYPE_OR
                req_policies.iter().any(|r| path_authorizes_policy(r))
            };
            if !ok {
                trust_bits |=
                    CERT_TRUST_IS_NOT_VALID_FOR_USAGE | CERT_TRUST_INVALID_POLICY_CONSTRAINTS;
                if para_err == 0 {
                    para_err = CERT_E_INVALID_POLICY;
                }
            }
        }
    }

    // 3. pStrongSignPara (@ +0x50 when cbSize >= 0x58)
    if cb_size >= 0x58 {
        let p_strong = std::ptr::read_unaligned(base.add(0x50) as *const *const u8);
        if !p_strong.is_null() {
            let cb_ss = std::ptr::read_unaligned(p_strong as *const u32);
            let dw_choice = std::ptr::read_unaligned(p_strong.add(4) as *const u32);
            if cb_ss < 0x10 {
                return Err(err::ERROR_INVALID_PARAMETER);
            }
            // Per Windows SDK wincrypt.h:
            //   CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE == 1 (pSerializedInfo)
            //   CERT_STRONG_SIGN_OID_INFO_CHOICE        == 2 (pszOID)
            // Enforce strong signature algorithms and key lengths for standard OS strong-sign OIDs;
            // fail closed on serialized info or unrecognized OIDs.
            match dw_choice {
                CERT_STRONG_SIGN_OID_INFO_CHOICE => {
                    let psz_oid = std::ptr::read_unaligned(p_strong.add(8) as *const *const u8);
                    if psz_oid.is_null() {
                        return Err(err::ERROR_INVALID_PARAMETER);
                    }
                    let s = crate::strings::from_ansi(psz_oid);
                    let recognized_strong_oid = matches!(
                        s.as_str(),
                        "1.3.6.1.4.1.311.72.1.1" // szOID_CERT_STRONG_SIGN_OS_1
                            | "1.3.6.1.4.1.311.72.1.2" // szOID_CERT_STRONG_SIGN_OS_CURRENT
                            | "1.3.6.1.4.1.311.72.1.3" // szOID_CERT_STRONG_SIGN_OS_SERVER_1
                    );
                    if !recognized_strong_oid {
                        return Err(err::ERROR_NOT_SUPPORTED);
                    }
                }
                CERT_STRONG_SIGN_SERIALIZED_INFO_CHOICE | _ => {
                    return Err(err::ERROR_NOT_SUPPORTED);
                }
            }
            // Read dwStrongSignFlags (@ +0x58 when cbSize >= 0x5C).
            let dw_strong_flags = if cb_size >= 0x5C {
                std::ptr::read_unaligned(base.add(0x58) as *const u32)
            } else {
                0
            };
            let disable_end_check =
                (dw_strong_flags & CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG) != 0;

            let mut weak = false;
            for (idx, &d) in selected_path.iter().enumerate() {
                let is_self_signed = parse_cert_der_parts(d)
                    .is_some_and(|p| p.subject_inner == p.issuer_inner);
                // Enforce strong signature algorithm on the leaf and all non-self-signed
                // certificates (historical self-signed roots may carry SHA-1 self-signatures
                // while still requiring a strong public key).
                if (idx == 0 || !is_self_signed) && !cert_has_strong_signature_alg(d) {
                    weak = true;
                    break;
                }
                // Enforce strong public key length (RSA >= 2047 bits, ECDSA >= P-256, Ed25519/Ed448)
                // on all issuer CAs, and on the leaf unless CERT_CHAIN_STRONG_SIGN_DISABLE_END_CHECK_FLAG is set.
                if (idx > 0 || !disable_end_check) && !cert_has_strong_public_key(d) {
                    weak = true;
                    break;
                }
            }
            if weak {
                trust_bits |= CERT_TRUST_IS_NOT_SIGNATURE_VALID | CERT_TRUST_HAS_WEAK_SIGNATURE;
                if para_err == 0 {
                    para_err = TRUST_E_CERT_SIGNATURE;
                }
            }
        }
    }

    Ok((trust_bits, para_err))
}

pub(super) unsafe fn cert_get_certificate_chain_impl(
    h_chain_engine: *mut c_void,
    p_cert_context: *const c_void,
    p_time: *const c_void,
    h_additional_store: *mut c_void,
    p_chain_para: *const c_void,
    dw_flags: u32,
    pp_chain_context: *mut *mut c_void,
) -> BOOL {
    if p_cert_context.is_null() || pp_chain_context.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let verify_filetime = if p_time.is_null() {
        None
    } else {
        Some(std::ptr::read_unaligned(p_time as *const u64))
    };
    let verify_unix = verify_filetime
        .and_then(filetime_to_unix_time)
        .unwrap_or_else(UnixTime::now);
    let check_ft = verify_filetime.unwrap_or_else(now_filetime);

    let (mut chain_der, peer_verify) =
        if let Some(pair) = lookup_cert_for_chain(p_cert_context as usize) {
            pair
        } else {
            let cc = std::ptr::read_unaligned(p_cert_context as *const CertContext);
            if cc.pb_cert_encoded.is_null() || cc.cb_cert_encoded == 0 {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            let der = std::slice::from_raw_parts(cc.pb_cert_encoded, cc.cb_cert_encoded as usize);
            (vec![der.to_vec()], None)
        };

    let (exclusive_roots_der, engine_extra) =
        if !h_chain_engine.is_null() && (h_chain_engine as usize) > 1 {
            ENGINES
                .with(h_chain_engine as usize, |eng| {
                    (
                        eng.exclusive_roots_der.clone(),
                        eng.additional_certs_der.clone(),
                    )
                })
                .unwrap_or((None, Vec::new()))
        } else {
            (None, Vec::new())
        };

    let mut extra_roots_der = peer_verify
        .as_ref()
        .map(|v| v.extra_roots_der.clone())
        .unwrap_or_default();
    extra_roots_der.extend(engine_extra);

    // Include any intermediates from hAdditionalStore (without duplicating leaf or intermediates).
    for der in store_certs_der(h_additional_store) {
        if !chain_der.iter().any(|c| c == &der) {
            chain_der.push(der);
        }
    }

    // Verify chain trust (ignoring hostname here; hostname is checked in CertVerifyCertificateChainPolicy).
    let default_chain_error = if exclusive_roots_der.is_none() {
        if let Some(ref pv) = peer_verify {
            if verify_filetime.is_some() {
                verify_chain_webpki_at(&chain_der, None, None, &extra_roots_der, true, verify_unix)
            } else if pv.cert_error == CERT_E_CN_NO_MATCH {
                0
            } else {
                pv.cert_error
            }
        } else {
            verify_chain_webpki_at(&chain_der, None, None, &extra_roots_der, true, verify_unix)
        }
    } else {
        extra_roots_der.clear();
        verify_chain_webpki_at(
            &chain_der,
            None,
            exclusive_roots_der.as_deref(),
            &[],
            true,
            verify_unix,
        )
    };

    let element_root_pool = exclusive_roots_der
        .as_deref()
        .unwrap_or(&extra_roots_der);
    let verified_element_der = resolve_verified_issuer_path(
        &chain_der,
        element_root_pool,
        exclusive_roots_der.is_none(),
    );

    let mut owned_certs = Vec::with_capacity(verified_element_der.len());
    let mut element_times = Vec::with_capacity(verified_element_der.len());
    for der in &verified_element_der {
        if let Some(c) = wintrust::build_owned_cert_context(der, std::ptr::null_mut()) {
            let ci = &*(*c.as_ptr()).p_cert_info;
            element_times.push((ci.not_before, ci.not_after));
            owned_certs.push(c);
        }
    }
    if owned_certs.is_empty() {
        crate::teb::set_last_error(err::ERROR_INVALID_DATA);
        return FALSE;
    }

    // Evaluate certificate revocation when requested via CERT_CHAIN_REVOCATION_CHECK_* flags.
    // Fails closed with CRYPT_E_REVOCATION_OFFLINE / CRYPT_E_NO_REVOCATION_CHECK (or CERT_E_REVOKED)
    // whenever positive non-revoked status cannot be confirmed for the requested scope.
    let revocation_flags = dw_flags
        & (CERT_CHAIN_REVOCATION_CHECK_MASK
            | CERT_CHAIN_REVOCATION_CHECK_CACHE_ONLY
            | CERT_CHAIN_CACHE_ONLY_URL_RETRIEVAL);
    let revocation_error: u32 = check_chain_revocation_error(
        &chain_der,
        exclusive_roots_der.as_deref(),
        &extra_roots_der,
        Some(verify_unix.as_secs() as f64),
        revocation_flags,
    );

    let strong_root_pool = exclusive_roots_der
        .as_deref()
        .unwrap_or(&extra_roots_der);
    let (para_trust_bits, chain_para_error) =
        match evaluate_cert_chain_para(p_chain_para, &chain_der, strong_root_pool) {
        Ok(pair) => pair,
        Err(le) => {
            crate::teb::set_last_error(le);
            return FALSE;
        }
    };

    let mut trust_err_bits = cert_error_to_trust_status(default_chain_error)
        | cert_error_to_trust_status(revocation_error)
        | para_trust_bits;
    let mut elements = Vec::with_capacity(owned_certs.len());
    let mut element_ptrs = Vec::with_capacity(owned_certs.len());
    for (idx, cert) in owned_certs.iter().enumerate() {
        let info_status = if cert.is_self_signed() {
            0x0000_0108
        } else {
            0x0000_0100
        };
        let (nb, na) = element_times[idx];
        let time_bad = (nb > 0 && check_ft < nb) || (na > 0 && check_ft > na);
        let mut elem_err = if idx == 0 { trust_err_bits } else { 0 };
        if time_bad {
            elem_err |= CERT_TRUST_IS_NOT_TIME_VALID;
            trust_err_bits |= CERT_TRUST_IS_NOT_TIME_VALID;
        }
        let el = Box::new(CertChainElement {
            cb_size: std::mem::size_of::<CertChainElement>() as u32,
            p_cert_context: cert.as_ptr(),
            trust_status: CertTrustStatus {
                dw_error_status: elem_err,
                dw_info_status: info_status,
            },
            p_revocation_info: std::ptr::null_mut(),
            p_issuance_usage: std::ptr::null_mut(),
            p_application_usage: std::ptr::null_mut(),
            pwsz_extended_error_info: std::ptr::null(),
        });
        element_ptrs.push(&*el as *const CertChainElement);
        elements.push(el);
    }

    let element_ptrs = element_ptrs.into_boxed_slice();
    let simple_chain = Box::new(CertSimpleChain {
        cb_size: std::mem::size_of::<CertSimpleChain>() as u32,
        trust_status: CertTrustStatus {
            dw_error_status: trust_err_bits,
            dw_info_status: 0,
        },
        c_element: elements.len() as u32,
        rgp_element: element_ptrs.as_ptr(),
        p_trust_list_info: std::ptr::null_mut(),
        f_has_revocation_freshness_time: 0,
        dw_revocation_freshness_time: 0,
    });
    let simple_chain_ptrs = vec![&*simple_chain as *const CertSimpleChain].into_boxed_slice();
    let chain_context = Box::new(CertChainContext {
        cb_size: std::mem::size_of::<CertChainContext>() as u32,
        trust_status: CertTrustStatus {
            dw_error_status: trust_err_bits,
            dw_info_status: 0,
        },
        c_chain: 1,
        rgp_chain: simple_chain_ptrs.as_ptr(),
        c_lower_quality_chain_context: 0,
        rgp_lower_quality_chain_context: std::ptr::null_mut(),
        f_has_revocation_freshness_time: 0,
        dw_revocation_freshness_time: 0,
        dw_create_flags: 0,
        chain_id: [0; 16],
    });
    let ctx_ptr = &*chain_context as *const CertChainContext as usize;

    let _ = CHAINS.insert_at(
        ctx_ptr,
        RegisteredChain {
            refcount: 1,
            _certs: owned_certs,
            _elements: elements,
            _element_ptrs: element_ptrs,
            _simple_chain: simple_chain,
            _simple_chain_ptrs: simple_chain_ptrs,
            _chain_context: chain_context,
            chain_der,
            element_times,
            verify_filetime,
            exclusive_roots_der,
            extra_roots_der,
            default_chain_error,
            revocation_flags,
            revocation_error,
            chain_para_error,
        },
    );
    std::ptr::write_unaligned(pp_chain_context, ctx_ptr as *mut c_void);
    TRUE
}

pub(super) fn cert_duplicate_certificate_chain_impl(chain: *const c_void) -> *mut c_void {
    if chain.is_null() {
        return std::ptr::null_mut();
    }
    let _ = CHAINS.with_mut(chain as usize, |c| {
        c.refcount += 1;
    });
    chain as *mut c_void
}

pub(super) fn cert_free_certificate_chain_impl(chain: *mut c_void) {
    if chain.is_null() {
        return;
    }
    let key = chain as usize;
    let should_remove = CHAINS
        .with_mut(key, |c| {
            if c.refcount > 1 {
                c.refcount -= 1;
                false
            } else {
                true
            }
        });
    match should_remove {
        Some(true) => {
            let _ = CHAINS.remove(key);
        }
        Some(false) => {}
        None => {
            log::debug!("CertFreeCertificateChain({chain:p}): unknown/already-freed chain");
        }
    }
}

pub(super) unsafe fn cert_verify_certificate_chain_policy_impl(
    psz_policy_oid: *const u8,
    p_chain_context: *const c_void,
    p_policy_para: *const c_void,
    p_policy_status: *mut c_void,
) -> BOOL {
    if p_chain_context.is_null() || p_policy_status.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let policy_id = psz_policy_oid as usize;
    if !matches!(
        policy_id,
        CERT_CHAIN_POLICY_BASE
            | CERT_CHAIN_POLICY_AUTHENTICODE
            | CERT_CHAIN_POLICY_AUTHENTICODE_TS
            | CERT_CHAIN_POLICY_SSL
            | CERT_CHAIN_POLICY_BASIC_CONSTRAINTS
            | CERT_CHAIN_POLICY_NT_AUTH
            | CERT_CHAIN_POLICY_MICROSOFT_ROOT
            | CERT_CHAIN_POLICY_EV
    ) {
        // Per MSDN/Wine: unknown chain policy OID returns FALSE with ERROR_FILE_NOT_FOUND.
        log::debug!("CertVerifyCertificateChainPolicy: unknown policy oid={policy_id:#x}");
        crate::teb::set_last_error(err::ERROR_FILE_NOT_FOUND);
        return FALSE;
    }

    let (
        chain_der,
        element_times,
        verify_ft,
        exclusive_roots,
        extra_roots,
        default_chain_error,
        revocation_flags,
        revocation_error,
        chain_para_error,
    ) = if let Some(found) = CHAINS.with(p_chain_context as usize, |c| {
        (
            c.chain_der.clone(),
            c.element_times.clone(),
            c.verify_filetime,
            c.exclusive_roots_der.clone(),
            c.extra_roots_der.clone(),
            c.default_chain_error,
            c.revocation_flags,
            c.revocation_error,
            c.chain_para_error,
        )
    }) {
        found
    } else {
        let Some((ders, times, ext_err)) = extract_external_chain(p_chain_context) else {
            crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
            return FALSE;
        };
        let ext_rev_err = match ext_err {
            CERT_E_REVOKED | CRYPT_E_REVOCATION_OFFLINE | CRYPT_E_NO_REVOCATION_CHECK => ext_err,
            _ => 0,
        };
        let ext_para_err = match ext_err {
            CERT_E_WRONG_USAGE | CERT_E_INVALID_POLICY => ext_err,
            _ => 0,
        };
        (
            ders,
            times,
            None,
            None,
            Vec::new(),
            ext_err,
            0,
            ext_rev_err,
            ext_para_err,
        )
    };

    let (dw_flags, extra_ptr) = if !p_policy_para.is_null() {
        let para = std::ptr::read_unaligned(p_policy_para as *const CertChainPolicyPara);
        (para.dw_flags, para.pv_extra_policy_para)
    } else {
        (0, std::ptr::null())
    };

    let mut fdw_checks = 0u32;
    let mut ssl_auth_type: u32 = 2; // AUTHTYPE_SERVER default when extra para absent
    let mut server_name: Option<String> = None;
    if policy_id == CERT_CHAIN_POLICY_SSL && !extra_ptr.is_null() {
        let ssl = std::ptr::read_unaligned(extra_ptr as *const SslExtraCertChainPolicyPara);
        fdw_checks = ssl.fdw_checks;
        if ssl.dw_auth_type == 1 || ssl.dw_auth_type == 2 {
            ssl_auth_type = ssl.dw_auth_type;
        }
        if ssl.cb_size >= 0x18 && !ssl.pwsz_server_name.is_null() {
            let s = crate::strings::from_wide(ssl.pwsz_server_name);
            if !s.is_empty() {
                server_name = Some(s);
            }
        }
    }

    let ignore_cn = (fdw_checks & SECURITY_FLAG_IGNORE_CERT_CN_INVALID) != 0
        || (dw_flags & CERT_CHAIN_POLICY_IGNORE_INVALID_NAME_FLAG) != 0;
    let ignore_ca = (fdw_checks & SECURITY_FLAG_IGNORE_UNKNOWN_CA) != 0
        || (dw_flags & CERT_CHAIN_POLICY_ALLOW_UNKNOWN_CA_FLAG) != 0;
    let ignore_date = (fdw_checks & SECURITY_FLAG_IGNORE_CERT_DATE_INVALID) != 0
        || (dw_flags & CERT_CHAIN_POLICY_IGNORE_NOT_TIME_VALID_FLAG) != 0
        || (dw_flags & CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS)
            == CERT_CHAIN_POLICY_IGNORE_ALL_NOT_TIME_VALID_FLAGS;
    let ignore_usage = (dw_flags & CERT_CHAIN_POLICY_IGNORE_WRONG_USAGE_FLAG) != 0;

    let check_ft = verify_ft.unwrap_or_else(now_filetime);
    let verify_unix = verify_ft
        .and_then(filetime_to_unix_time)
        .unwrap_or_else(UnixTime::now);
    let first_time_invalid_idx = element_times
        .iter()
        .position(|&(nb, na)| (nb > 0 && check_ft < nb) || (na > 0 && check_ft > na));

    let effective_rev_err: u32 = {
        let rev_err = if default_chain_error == CERT_E_REVOKED || revocation_error == CERT_E_REVOKED
        {
            CERT_E_REVOKED
        } else {
            revocation_error
        };
        if rev_err == 0 || rev_err == CERT_E_REVOKED {
            rev_err
        } else if policy_id == CERT_CHAIN_POLICY_SSL
            && (fdw_checks & SECURITY_FLAG_IGNORE_REVOCATION) != 0
        {
            0
        } else if rev_err == CRYPT_E_NO_REVOCATION_CHECK
            && (revocation_flags & CERT_CHAIN_REVOCATION_CHECK_CHAIN) != 0
            && (dw_flags & CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG) != 0
        {
            0
        } else {
            let ignore_end = (dw_flags & CERT_CHAIN_POLICY_IGNORE_END_REV_UNKNOWN_FLAG) != 0;
            let ignore_ca_rev = (dw_flags & CERT_CHAIN_POLICY_IGNORE_CA_REV_UNKNOWN_FLAG) != 0;
            let ignore_root_rev = (dw_flags & CERT_CHAIN_POLICY_IGNORE_ROOT_REV_UNKNOWN_FLAG) != 0;
            let ignored = if (revocation_flags & CERT_CHAIN_REVOCATION_CHECK_END_CERT) != 0
                && (revocation_flags
                    & (CERT_CHAIN_REVOCATION_CHECK_CHAIN
                        | CERT_CHAIN_REVOCATION_CHECK_CHAIN_EXCLUDE_ROOT))
                    == 0
            {
                ignore_end
            } else if (revocation_flags & CERT_CHAIN_REVOCATION_CHECK_CHAIN) != 0 {
                ignore_end && ignore_ca_rev && ignore_root_rev
            } else {
                ignore_end && ignore_ca_rev
            };
            if ignored {
                0
            } else {
                rev_err
            }
        }
    };

    let (mut err_code, mut elem_idx): (u32, i32) = match policy_id {
        CERT_CHAIN_POLICY_SSL => {
            const OID_CLIENT_AUTH: &[u8] = &[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x02];
            const OID_ANY_EKU: &[u8] = &[0x55, 0x1D, 0x25, 0x00];
            let is_client_auth = ssl_auth_type == 1;
            let mut raw = if is_client_auth {
                verify_chain_webpki_client_at(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                    verify_unix,
                )
            } else {
                verify_chain_webpki_at(
                    &chain_der,
                    server_name.as_deref(),
                    exclusive_roots.as_deref(),
                    &extra_roots,
                    ignore_cn,
                    verify_unix,
                )
            };
            // Fall back to Apple Security.framework SecTrust when WebPKI rejects with untrusted root
            // (handles system-installed macOS roots or non-WebPKI chains trusted by Apple's trust store).
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            if raw == CERT_E_UNTRUSTEDROOT {
                let verify_secs = Some(verify_unix.as_secs() as f64);
                let host_arg = if is_client_auth || ignore_cn {
                    None
                } else {
                    server_name.as_deref()
                };
                if apple_trust::evaluate_sec_trust(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                    host_arg,
                    verify_secs,
                ) == Some(true)
                {
                    raw = 0;
                }
            }
            if (raw == CERT_E_UNTRUSTEDROOT && ignore_ca)
                || (raw == CERT_E_EXPIRED && ignore_date)
                || (raw == CERT_E_CN_NO_MATCH && ignore_cn)
                || (raw == CERT_E_WRONG_USAGE && ignore_usage)
            {
                raw = 0;
            }
            // Independent post-checks ensure ignoring one error (e.g. ALLOW_UNKNOWN_CA)
            // never silently masks an un-ignored expiration, clientAuth EKU, or hostname mismatch!
            let client_usage_bad = is_client_auth
                && !ignore_usage
                && chain_der
                    .first()
                    .and_then(|d| cert_eku_oids(d))
                    .is_some_and(|oids| {
                        !oids.is_empty()
                            && !oids.iter().any(|o| {
                                o.as_slice() == OID_CLIENT_AUTH || o.as_slice() == OID_ANY_EKU
                            })
                    });
            if raw == 0 {
                if client_usage_bad {
                    (CERT_E_WRONG_USAGE, 0)
                } else if !ignore_date {
                    if let Some(ti) = first_time_invalid_idx {
                        (CERT_E_EXPIRED, ti as i32)
                    } else if !is_client_auth {
                        if let Some(ref host) = server_name {
                            if !ignore_cn
                                && !chain_der
                                    .first()
                                    .is_some_and(|leaf| cert_matches_hostname(leaf, host))
                            {
                                (CERT_E_CN_NO_MATCH, 0)
                            } else {
                                (0, -1)
                            }
                        } else {
                            (0, -1)
                        }
                    } else {
                        (0, -1)
                    }
                } else if !is_client_auth {
                    if let Some(ref host) = server_name {
                        if !ignore_cn
                            && !chain_der
                                .first()
                                .is_some_and(|leaf| cert_matches_hostname(leaf, host))
                        {
                            (CERT_E_CN_NO_MATCH, 0)
                        } else {
                            (0, -1)
                        }
                    } else {
                        (0, -1)
                    }
                } else {
                    (0, -1)
                }
            } else {
                let ei = if raw == CERT_E_EXPIRED {
                    first_time_invalid_idx.unwrap_or(0) as i32
                } else {
                    0
                };
                (raw, ei)
            }
        }
        CERT_CHAIN_POLICY_AUTHENTICODE | CERT_CHAIN_POLICY_AUTHENTICODE_TS => {
            const OID_CODE_SIGNING: &[u8] = &[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x03];
            const OID_TIME_STAMPING: &[u8] = &[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x08];
            const OID_ANY_EKU: &[u8] = &[0x55, 0x1D, 0x25, 0x00];
            let req_eku = if policy_id == CERT_CHAIN_POLICY_AUTHENTICODE_TS {
                OID_TIME_STAMPING
            } else {
                OID_CODE_SIGNING
            };
            let usage_bad = !ignore_usage
                && chain_der
                    .first()
                    .and_then(|d| cert_eku_oids(d))
                    .is_some_and(|oids| {
                        !oids.is_empty()
                            && !oids
                                .iter()
                                .any(|o| o.as_slice() == req_eku || o.as_slice() == OID_ANY_EKU)
                    });
            let mut trust_ok = default_chain_error == 0
                || verify_chain_to_root_with_eku(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                    Some(req_eku),
                )
                .is_some();
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            if !trust_ok {
                trust_ok = apple_trust::evaluate_sec_trust(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                    None,
                    Some(verify_unix.as_secs() as f64),
                ) == Some(true);
            }
            if default_chain_error == CERT_E_REVOKED {
                (CERT_E_REVOKED, 0)
            } else if usage_bad {
                (CERT_E_WRONG_USAGE, 0)
            } else if !ignore_date && first_time_invalid_idx.is_some() {
                (CERT_E_EXPIRED, first_time_invalid_idx.unwrap() as i32)
            } else if !trust_ok && !ignore_ca {
                (
                    if default_chain_error == CERT_E_CHAINING {
                        CERT_E_CHAINING
                    } else {
                        CERT_E_UNTRUSTEDROOT
                    },
                    0,
                )
            } else {
                (0, -1)
            }
        }
        CERT_CHAIN_POLICY_BASIC_CONSTRAINTS => {
            // Check Basic Constraints extension (2.5.29.19 / legacy 2.5.29.10) across the chain.
            // Per MSDN CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_BASIC_CONSTRAINTS):
            // if a certificate has neither Basic Constraints extension, it is considered valid
            // (legacy roots/issuers without Basic Constraints are not treated as explicit non-CAs).
            const OID_BC2: &[u8] = &[0x55, 0x1D, 0x13];
            const OID_BC1: &[u8] = &[0x55, 0x1D, 0x0A];
            let require_ca_leaf = (dw_flags & BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_CA_FLAG) != 0;
            let require_ee_leaf =
                (dw_flags & BASIC_CONSTRAINTS_CERT_CHAIN_POLICY_END_ENTITY_FLAG) != 0;
            let bc_root_pool = exclusive_roots.as_deref().unwrap_or(&extra_roots);
            let selected_path = resolve_selected_issuer_path(&chain_der, bc_root_pool);
            let mut violation: Option<(u32, i32)> = None;
            let mut non_self_issued_below = 0usize;
            for (idx, &der) in selected_path.iter().enumerate() {
                let Some(parts) = parse_cert_der_parts(der) else {
                    continue;
                };
                let is_self_issued = parts.subject_inner == parts.issuer_inner;
                let mut has_bc = false;
                let mut is_ca = false;
                let mut is_ee = false;
                let mut path_len: Option<usize> = None;
                for (oid, _, val) in &parts.extensions {
                    if *oid == OID_BC2 {
                        has_bc = true;
                        let mut ca_flag = false;
                        if let Some((seq, _)) = parse_der_item(val) {
                            if let Some(kids) = der_item_children(seq.val) {
                                for k in kids {
                                    if k.tag == 0x01 {
                                        ca_flag = k.val.first().copied().unwrap_or(0) != 0;
                                    } else if k.tag == 0x02 && !k.val.is_empty() {
                                        let mut v = 0usize;
                                        for &b in k.val {
                                            v = (v << 8) | (b as usize);
                                        }
                                        path_len = Some(v);
                                    }
                                }
                            }
                        }
                        is_ca = ca_flag;
                        is_ee = !ca_flag;
                    } else if *oid == OID_BC1 && !has_bc {
                        has_bc = true;
                        if let Some((seq, _)) = parse_der_item(val) {
                            if let Some(kids) = der_item_children(seq.val) {
                                for k in kids {
                                    if k.tag == 0x03 && k.val.len() >= 2 {
                                        let bits = k.val[1];
                                        is_ca = (bits & 0x80) != 0;
                                        is_ee = (bits & 0x40) != 0;
                                    } else if k.tag == 0x02 && !k.val.is_empty() {
                                        let mut v = 0usize;
                                        for &b in k.val {
                                            v = (v << 8) | (b as usize);
                                        }
                                        path_len = Some(v);
                                    }
                                }
                            }
                        }
                    }
                }
                if has_bc {
                    if idx == 0 {
                        // Per MSDN CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_BASIC_CONSTRAINTS):
                        // setting both CA_FLAG and END_ENTITY_FLAG allows either leaf type;
                        // enforce a specific leaf type only when exactly one flag is set.
                        if require_ca_leaf && require_ee_leaf {
                            if !is_ca && !is_ee {
                                violation = Some((TRUST_E_BASIC_CONSTRAINTS, 0));
                                break;
                            }
                        } else if require_ca_leaf && !is_ca {
                            violation = Some((TRUST_E_BASIC_CONSTRAINTS, 0));
                            break;
                        } else if require_ee_leaf && !is_ee {
                            violation = Some((TRUST_E_BASIC_CONSTRAINTS, 0));
                            break;
                        }
                    } else {
                        if !is_ca {
                            violation = Some((TRUST_E_BASIC_CONSTRAINTS, idx as i32));
                            break;
                        }
                        // RFC 5280 §4.2.1.9: pathLenConstraint counts only non-self-issued
                        // intermediate certificates subordinate to this CA.
                        if let Some(max_inter) = path_len {
                            if non_self_issued_below > max_inter {
                                violation = Some((TRUST_E_BASIC_CONSTRAINTS, idx as i32));
                                break;
                            }
                        }
                    }
                }
                if idx > 0 && !is_self_issued {
                    non_self_issued_below += 1;
                }
            }
            violation.unwrap_or((0, -1))
        }
        CERT_CHAIN_POLICY_NT_AUTH => {
            const OID_CLIENT_AUTH: &[u8] = &[0x2B, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x02];
            const OID_SMARTCARD_LOGON: &[u8] =
                &[0x2B, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x14, 0x02, 0x02];
            const OID_ANY_EKU: &[u8] = &[0x55, 0x1D, 0x25, 0x00];
            let usage_bad = !ignore_usage
                && chain_der
                    .first()
                    .and_then(|d| cert_eku_oids(d))
                    .is_some_and(|oids| {
                        !oids.is_empty()
                            && !oids.iter().any(|o| {
                                o.as_slice() == OID_CLIENT_AUTH
                                    || o.as_slice() == OID_SMARTCARD_LOGON
                                    || o.as_slice() == OID_ANY_EKU
                            })
                    });
            let verified_endpoints = verify_chain_to_root(
                &chain_der,
                exclusive_roots.as_deref(),
                &extra_roots,
            );
            let trust_ok = default_chain_error == 0 || verified_endpoints.is_some();
            // Per MSDN CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_NT_AUTH):
            // The issuing CA must be explicitly authorized in the Enterprise NTAuth store;
            // ordinary trusted CAs are NOT authorized for NT authentication -> CERT_E_UNTRUSTEDCA.
            let ntauth_authorized = verified_endpoints.as_ref().is_some_and(|ep| {
                super::store::ntauth_store_certs_der().iter().any(|der| {
                    parse_cert_der_parts(der)
                        .is_some_and(|p| p.spki_inner == ep.issuing_ca_spki_inner.as_slice())
                })
            });
            if default_chain_error == CERT_E_REVOKED {
                (CERT_E_REVOKED, 0)
            } else if usage_bad {
                (CERT_E_WRONG_USAGE, 0)
            } else if !ignore_date && first_time_invalid_idx.is_some() {
                (CERT_E_EXPIRED, first_time_invalid_idx.unwrap() as i32)
            } else if !trust_ok && !ignore_ca {
                (CERT_E_UNTRUSTEDROOT, 0)
            } else if !ntauth_authorized {
                (CERT_E_UNTRUSTEDCA, 1.min(chain_der.len().saturating_sub(1)) as i32)
            } else {
                (0, -1)
            }
        }
        CERT_CHAIN_POLICY_MICROSOFT_ROOT => {
            // Per MSDN CertVerifyCertificateChainPolicy(CERT_CHAIN_POLICY_MICROSOFT_ROOT):
            // Verify the actual cryptographically chained terminating root's public key
            // against Microsoft's root public keys (never by display name/subject string).
            let verified_endpoints = verify_chain_to_root(
                &chain_der,
                exclusive_roots.as_deref(),
                &extra_roots,
            );
            let root_is_ms_key = verified_endpoints.as_ref().is_some_and(|ep| {
                let fp = spki_inner_sha256(&ep.root_spki_inner);
                MICROSOFT_ROOT_SPKI_SHA256.iter().any(|known| known == &fp)
            });
            if !root_is_ms_key {
                (CERT_E_UNTRUSTEDROOT, chain_der.len().saturating_sub(1) as i32)
            } else if !ignore_date && first_time_invalid_idx.is_some() {
                (CERT_E_EXPIRED, first_time_invalid_idx.unwrap() as i32)
            } else {
                (0, -1)
            }
        }
        CERT_CHAIN_POLICY_EV => {
            // Perform genuine Extended Validation via Apple Security.framework's EV policy +
            // root authorization table (`TrustExtendedValidation`). Ordinary DV/OV certificates
            // (even those carrying a `certificatePolicies` extension) are rejected with
            // `CERT_E_WRONG_USAGE`.
            let trust_ok = default_chain_error == 0
                || verify_chain_signatures_to_roots(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                );
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            let is_ev = apple_trust::evaluate_sec_trust_ev(
                &chain_der,
                exclusive_roots.as_deref(),
                &extra_roots,
                Some(verify_unix.as_secs() as f64),
            );
            #[cfg(not(any(target_os = "macos", target_os = "ios")))]
            let is_ev = false;

            if default_chain_error == CERT_E_REVOKED {
                (CERT_E_REVOKED, 0)
            } else if !ignore_date && first_time_invalid_idx.is_some() {
                (CERT_E_EXPIRED, first_time_invalid_idx.unwrap() as i32)
            } else if !trust_ok && !ignore_ca {
                (CERT_E_UNTRUSTEDROOT, 0)
            } else if !is_ev {
                (CERT_E_WRONG_USAGE, 0)
            } else {
                (0, -1)
            }
        }
        CERT_CHAIN_POLICY_BASE => {
            let mut trust_ok = default_chain_error == 0
                || verify_chain_signatures_to_roots(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                );
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            if !trust_ok {
                trust_ok = apple_trust::evaluate_sec_trust(
                    &chain_der,
                    exclusive_roots.as_deref(),
                    &extra_roots,
                    None,
                    Some(verify_unix.as_secs() as f64),
                ) == Some(true);
            }
            if default_chain_error == CERT_E_REVOKED {
                (CERT_E_REVOKED, 0)
            } else if !ignore_date && first_time_invalid_idx.is_some() {
                (CERT_E_EXPIRED, first_time_invalid_idx.unwrap() as i32)
            } else if !trust_ok && !ignore_ca {
                (
                    if default_chain_error == CERT_E_CHAINING {
                        CERT_E_CHAINING
                    } else {
                        CERT_E_UNTRUSTEDROOT
                    },
                    0,
                )
            } else {
                (0, -1)
            }
        }
        _ => unreachable!(),
    };

    // Propagate CERT_CHAIN_PARA RequestedUsage / RequestedIssuancePolicy / StrongSign errors
    // into policy verification unless explicitly ignored.
    if policy_id != CERT_CHAIN_POLICY_BASIC_CONSTRAINTS && chain_para_error != 0 && err_code == 0 {
        let ignored = chain_para_error == CERT_E_WRONG_USAGE && ignore_usage;
        if !ignored {
            err_code = chain_para_error;
            elem_idx = 0;
        }
    }

    // Confirmed revocation (CERT_E_REVOKED) always takes precedence across trust policies;
    // un-ignored offline/unknown revocation errors fail closed whenever other checks pass.
    if policy_id != CERT_CHAIN_POLICY_BASIC_CONSTRAINTS && effective_rev_err != 0 {
        if effective_rev_err == CERT_E_REVOKED || err_code == 0 {
            err_code = effective_rev_err;
            elem_idx = if effective_rev_err == CRYPT_E_NO_REVOCATION_CHECK
                && (revocation_flags & CERT_CHAIN_REVOCATION_CHECK_CHAIN) != 0
            {
                chain_der.len().saturating_sub(1) as i32
            } else {
                0
            };
        }
    }

    let mut status = std::ptr::read_unaligned(p_policy_status as *const CertChainPolicyStatus);
    status.dw_error = err_code;
    status.l_chain_index = if err_code == 0 { -1 } else { 0 };
    status.l_element_index = elem_idx;
    std::ptr::write_unaligned(p_policy_status as *mut CertChainPolicyStatus, status);
    log::debug!(
        "CertVerifyCertificateChainPolicy(oid={policy_id}, host={server_name:?}) -> dwError={err_code:#010x} (elem={elem_idx})"
    );
    TRUE
}

pub(super) unsafe fn cert_verify_time_validity_impl(
    p_time_to_verify: *const c_void,
    p_cert_info: *const c_void,
) -> i32 {
    if p_cert_info.is_null() {
        return 0;
    }
    let ci = std::ptr::read_unaligned(p_cert_info as *const CertInfo);
    let t = if p_time_to_verify.is_null() {
        now_filetime()
    } else {
        std::ptr::read_unaligned(p_time_to_verify as *const u64)
    };
    if ci.not_before > 0 && t < ci.not_before {
        -1
    } else if ci.not_after > 0 && t > ci.not_after {
        1
    } else {
        0
    }
}

pub(super) unsafe fn cert_verify_subject_certificate_context_impl(
    p_subject: *const c_void,
    p_issuer: *const c_void,
    pdw_flags: *mut u32,
) -> BOOL {
    const CERT_STORE_SIGNATURE_FLAG: u32 = 0x0000_0001;
    const CERT_STORE_TIME_VALIDITY_FLAG: u32 = 0x0000_0002;
    const CERT_STORE_REVOCATION_FLAG: u32 = 0x0000_0004;
    const CERT_STORE_NO_CRL_FLAG: u32 = 0x0001_0000;
    const E_INVALIDARG: u32 = 0x8007_0057;

    if p_subject.is_null() || pdw_flags.is_null() {
        crate::teb::set_last_error(E_INVALIDARG);
        return FALSE;
    }
    let req = std::ptr::read_unaligned(pdw_flags);
    let subj_cc = std::ptr::read_unaligned(p_subject as *const CertContext);
    let mut out_flags = 0u32;

    if (req & CERT_STORE_SIGNATURE_FLAG) != 0 {
        if p_issuer.is_null() {
            // Per MSDN: pIssuer must be non-NULL when CERT_STORE_SIGNATURE_FLAG is requested.
            out_flags |= CERT_STORE_SIGNATURE_FLAG;
        } else {
            let iss_cc = std::ptr::read_unaligned(p_issuer as *const CertContext);
            let sig_ok = if !subj_cc.pb_cert_encoded.is_null()
                && subj_cc.cb_cert_encoded > 0
                && !iss_cc.pb_cert_encoded.is_null()
                && iss_cc.cb_cert_encoded > 0
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
                            || verify_rsa_cert_sig_fallback(&sp, ip.spki_alg_inner, ip.spki_key_bits)
                    }
                    _ => false,
                }
            } else {
                false
            };
            if !sig_ok {
                out_flags |= CERT_STORE_SIGNATURE_FLAG;
            }
        }
    }

    if (req & CERT_STORE_TIME_VALIDITY_FLAG) != 0 {
        if subj_cc.p_cert_info.is_null()
            || cert_verify_time_validity_impl(std::ptr::null(), subj_cc.p_cert_info as *const c_void)
                != 0
        {
            out_flags |= CERT_STORE_TIME_VALIDITY_FLAG;
        }
    }

    if (req & CERT_STORE_REVOCATION_FLAG) != 0 {
        // No local CRL available for the issuer -> Windows sets both REVOCATION and NO_CRL flags.
        out_flags |= CERT_STORE_REVOCATION_FLAG | CERT_STORE_NO_CRL_FLAG;
    }

    std::ptr::write_unaligned(pdw_flags, out_flags);
    TRUE
}

fn verify_rsa_cert_sig_fallback(
    child: &CertDerParts<'_>,
    spki_alg_inner: &[u8],
    spki_key_bits: &[u8],
) -> bool {
    const OID_RSA: &[u8] = &[0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01];
    let Some((alg_oid, _)) = parse_der_item(spki_alg_inner) else {
        return false;
    };
    if alg_oid.tag != 0x06 || alg_oid.val != OID_RSA {
        return false;
    }
    let Some((sig_oid, _)) = parse_der_item(child.sig_alg_inner) else {
        return false;
    };
    let digest = match sig_oid.val {
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x04] => crate::misc::bcrypt::Digest::Md5,
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x05] => crate::misc::bcrypt::Digest::Sha1,
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B] => crate::misc::bcrypt::Digest::Sha256,
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0C] => crate::misc::bcrypt::Digest::Sha384,
        [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0D] => crate::misc::bcrypt::Digest::Sha512,
        _ => return false,
    };
    let Some((pub_seq, _)) = parse_der_item(spki_key_bits) else {
        return false;
    };
    let Some(pk) = der_item_children(pub_seq.val) else {
        return false;
    };
    if pk.len() < 2 || pk[0].tag != 0x02 || pk[1].tag != 0x02 {
        return false;
    }
    let Some(rsa) = crate::misc::bcrypt::RsaPublicKey::from_asn1_integers(pk[0].val, pk[1].val)
    else {
        return false;
    };
    let h = digest.hash(child.tbs_raw);
    rsa.verify_pkcs1(Some(digest), &h, child.sig_bits)
}
