# Stubs 4: crypt32, Schannel and WinVerifyTrust

Scope: 27 stub sites across `crates/win32-rt/src/misc/crypt32.rs`, `crypt32/asn1.rs`, `crypt32/chain.rs`, `secur32/schannel.rs`, and `wintrust.rs`.

- **Sites in scope before**: 27 (12 `failing!` + 15 `stub!`)
- **Sites in scope after**: 1 (1 `failing!`, 0 `stub!`)
- **Disposition**: 24 made real, 2 native-unsupported (matching Windows without TPM hardware), 1 honest leftover (blocked on parallel branch `stubs/crypto-cng-capi`)

## DPAPI Key Derivation Audit Note

The earlier review flag about DPAPI deriving keys from public constants in `crates/win32-rt/src/misc/crypt32/asn1.rs` was verified to be **already resolved on `origin/main`**: `dpapi_host_secret()` generates a 32-byte random key via `arc4random_buf`, atomically persists it with `0o600` permissions in per-user (`dpapi_user_<user>.key`) or machine (`dpapi_machine.key`) state files, never falls back to hardcoded/public constants, and is covered by `dpapi_uses_0600_host_secrets_and_isolates_secrets_and_machine_scope`.

---

## Remaining Sites in Scope (1)

| DLL!API | Kind | Why Not Implemented Yet | Next Step |
|---|---|---|---|
| `crypt32.dll!CryptImportPublicKeyInfo` | `failing` | Returns a CryptoAPI `HCRYPTKEY` handle associated with an `advapi32.dll` `HCRYPTPROV`, which must be usable by `advapi32.dll!CryptDestroyKey`, `CryptVerifySignature`, `CryptEncrypt`, and `CryptExportKey`. In `crates/win32-rt/src/advapi32.rs` (owned by parallel branch `stubs/crypto-cng-capi`, outside this branch's scope), `CryptImportKey`/`CryptDestroyKey`/`CryptExportKey` are currently 0-arg `failing!` macros. The `CERT_PUBLIC_KEY_INFO` -> CAPI `PUBLICKEYBLOB` (`RSA1` `BLOBHEADER` + `RSAPUBKEY` + little-endian modulus) converter (`crypt32::decode::spki_to_capi_public_key_blob`) is already implemented and unit-tested. | Once `stubs/crypto-cng-capi` lands real `advapi32::CryptImportKey`, replace the `failing!` line in `crates/win32-rt/src/misc/crypt32.rs` with a call to `decode::spki_to_capi_public_key_blob` followed by `advapi32::CryptImportKey(h_crypt_prov, blob.as_ptr(), blob.len() as u32, 0, 0, ph_key)`. |

---

## Native-Unsupported (Matches Windows Without the Feature) (2)

| DLL!API | Return Value / Error | Capability Absent on Apple Silicon | Microsoft Learn Reference |
|---|---|---|---|
| `wintrust.dll!Tbsip_Context_Close` | `TBS_E_INVALID_CONTEXT` (`0x80284004`) | Apple Silicon Macs have no TCG TPM 1.2/2.0 hardware device and no Windows TPM Base Services (TBS) daemon. Because `Tbsi_Context_Create` returns `TBS_E_TPM_NOT_FOUND` and never issues a valid `TBS_HCONTEXT`, any handle passed to `Tbsip_Context_Close` is invalid, matching Windows on a machine without a TPM. | [Tbsip_Context_Close (tbs.h)](https://learn.microsoft.com/en-us/windows/win32/api/tbs/nf-tbs-tbsip_context_close) |
| `wintrust.dll!Tbsip_Submit_Command` | `TBS_E_TPM_NOT_FOUND` (`0x8028400F`, zeroes `*pResultBufSize`) | No TPM device is present to receive TPM command buffers; matches Windows TBS when no compatible TPM is found on the host. | [Tbsip_Submit_Command (tbs.h)](https://learn.microsoft.com/en-us/windows/win32/api/tbs/nf-tbs-tbsip_submit_command) |

---

## Sites Made Real (24)

| DLL!API | File | Implementation Summary |
|---|---|---|
| `crypt32.dll!CryptFindOIDInfo` | `misc/crypt32/decode.rs` | Win64 ABI-exact 64-byte static `CRYPT_OID_INFO` catalog supporting all 6 key types (`OID`, `NAME`, `ALGID`, `SIGN`, `CNG_ALGID`, `CNG_SIGN`) and all 10 OID groups. |
| `crypt32.dll!CertStrToNameA` | `misc/crypt32/decode.rs` | RFC 1779/2253 X.500 DN string parser (`CN`, `O`, `OU`, `C`, `L`, `S`/`ST`, `E`, `DC`, `SN`, `SERIALNUMBER`, dotted OIDs, quotes/escapes, `+` multi-valued RDNs, `#hex` DER, `CERT_NAME_STR_REVERSE_FLAG`, `ppszError` + `CRYPT_E_INVALID_X500_STRING`). |
| `crypt32.dll!CertStrToNameW` | `misc/crypt32/decode.rs` | UTF-16 counterpart to `CertStrToNameA` with UTF-16 character error offset calculation. |
| `crypt32.dll!CryptDecodeObject` | `misc/crypt32/decode.rs` | Single-buffer relocating `DecodeArena` decoder for 25+ X.509/PKCS#7 struct types and OID aliases with Windows size-query and `ERROR_MORE_DATA` semantics. |
| `crypt32.dll!CryptDecodeObjectEx` | `misc/crypt32/decode.rs` | Extended decoder adding `CRYPT_DECODE_ALLOC_FLAG` (`LocalAlloc`), `CRYPT_DECODE_NOCOPY_FLAG`, `CRYPT_DECODE_SHARE_OID_STRING_FLAG`, and `ERROR_FILE_NOT_FOUND` for unknown struct types. |
| `crypt32.dll!CryptEncodeObjectEx` (`encode_x509_object`) | `misc/crypt32/asn1.rs` | Added `X509_ENUMERATED`, `PKCS_UTC_TIME`, `X509_ALGORITHM_IDENTIFIER`, `PKCS_CONTENT_INFO`, `PKCS_ATTRIBUTE`, `PKCS_ATTRIBUTES`, `X509_AUTHORITY_KEY_ID2` (`2.5.29.35`), `CNG_RSA_PUBLIC_KEY_BLOB`, `PKCS_7_ASN_ENCODING`, and `ERROR_FILE_NOT_FOUND` for unknown struct types. |
| `crypt32.dll!CryptStringToBinaryA/W` (`crypt_string_to_binary_impl`) | `misc/crypt32/asn1.rs` | Added `CRYPT_STRING_PERCENTESCAPE` (`0x0E`) decoder and returns `ERROR_INVALID_DATA` for unsupported flag values. |
| `crypt32.dll!CertGetCertificateContextProperty` | `misc/crypt32/store.rs` | Real computed and stored properties (`SHA1_HASH`, `MD5_HASH`, `SHA256_HASH`, `SIGNATURE_HASH`, `KEY_IDENTIFIER`, `SUBJECT_PUBLIC_KEY_MD5_HASH`, `ISSUER_PUBLIC_KEY_MD5_HASH`, `SUBJECT_NAME_MD5_HASH`, `ENHKEY_USAGE`, `PUBKEY_ALG_PARA`, `ACCESS_STATE`, `KEY_SPEC`, `KEY_PROV_INFO`, `FRIENDLY_NAME`, custom properties) plus `CertSetCertificateContextProperty`. |
| `crypt32.dll!CertFreeCRLContext` | `misc/crypt32/store.rs` | Real refcounted `CRL_CONTEXT` registry with `CertCreateCRLContext`, `CertDuplicateCRLContext`, and `CertFreeCRLContext` (returns `TRUE` on null/unregistered per Windows). |
| `crypt32.dll!CertFreeCTLContext` | `misc/crypt32/store.rs` | Real refcounted `CTL_CONTEXT` registry with `CertCreateCTLContext`, `CertDuplicateCTLContext`, and `CertFreeCTLContext`. |
| `crypt32.dll!CryptMsgOpenToDecode` | `misc/crypt32/pkcs.rs` | Real PKCS#7/CMS decode message handle creation (attached & detached `CMSG_DETACHED_FLAG`). |
| `crypt32.dll!CryptMsgUpdate` | `misc/crypt32/pkcs.rs` | Streaming/chunked DER `ContentInfo` accumulation and finalization with `CryptMsgGetParam` support (`TYPE`, `VERSION`, `CONTENT`, `BARE_CONTENT`, `INNER_CONTENT_TYPE`, `ENCODED_MESSAGE`, `SIGNER_COUNT`, `SIGNER_INFO`, `SIGNER_CERT_INFO`, `CERT_COUNT`, `CERT`, `SIGNER_AUTH_ATTR`, `SIGNER_UNAUTH_ATTR`). |
| `crypt32.dll!CryptVerifyMessageSignature` | `misc/crypt32/pkcs.rs` | Real RSA PKCS#1 v1.5, ECDSA, and Ed25519 signature verification over `signedAttrs` (SET-retagged `0x31` + `messageDigest` match) or direct content, with payload decoding and signer `PCCERT_CONTEXT` output. |
| `crypt32.dll!PFXImportCertStore` | `misc/crypt32/pkcs.rs` | Real RFC 7292 PKCS#12 parser backed by Apple CommonCrypto (`CCCrypt`, `CCKeyDerivationPBKDF`, `Digest::hmac`) supporting PBES2 (PBKDF2 + AES-128/192/256-CBC, 3DES-CBC) and legacy PBES1 (SHA1-3DES-CBC, SHA1-RC2-128/40-CBC), MAC verification (`ERROR_INVALID_PASSWORD`), `friendlyName`/`localKeyId` attributes, and PKCS#8 private key attachment. |
| `crypt32.dll!CertVerifySubjectCertificateContext` | `misc/crypt32/chain.rs` | Real RSA/ECDSA/Ed25519 signature verification against issuer SPKI, time validity check (`NotBefore`/`NotAfter`), and revocation/flag masking semantics. |
| `crypt32.dll!CertFreeCertificateChainEngine` | `misc/crypt32/chain.rs` | Proper handling of `NULL`, `HCCE_LOCAL_MACHINE` (`1`), and custom engine handles without stub warning. |
| `crypt32.dll!CertFreeCertificateChain` | `misc/crypt32/chain.rs` | Refcounted chain handles with `CertDuplicateCertificateChain` and safe cleanup. |
| `crypt32.dll!CertVerifyCertificateChainPolicy` | `misc/crypt32/chain.rs` | Added Apple `Security.framework` macOS root store integration (`SecTrustCopyAnchorCertificates` + `SecTrustSettingsCopyCertificates` + `SecTrustEvaluateWithError`) and implemented policies `BASIC_CONSTRAINTS` (5), `NT_AUTH` (6), `MICROSOFT_ROOT` (7), `EV` (8), `SSL_F12` (9), `SSL_HPKP_HEADER` (10), `THIRD_PARTY_ROOT` (11), `SSL_KEY_PIN` (12), returning `ERROR_FILE_NOT_FOUND` for unknown policy OIDs. |
| `secur32.dll!AcquireCredentialsHandleA/W` | `misc/secur32/schannel.rs` | Real inbound (`SECPKG_CRED_INBOUND`) and bidirectional (`SECPKG_CRED_BOTH`) Schannel credentials reading cert chain + PKCS#8 private key from `SCHANNEL_CRED`/`SCH_CREDENTIALS` `paCred`, returning `SEC_E_NO_CREDENTIALS` if missing, paired with `AcceptSecurityContext` (`rustls::ServerConnection`). |
| `secur32.dll!ApplyControlToken` | `misc/secur32/schannel.rs` | Real `SCHANNEL_SHUTDOWN` (1), `SCHANNEL_ALERT` (2, warning and fatal), and `SCHANNEL_SESSION` (3, enable/disable reconnects); returns `SEC_E_INVALID_TOKEN` for unknown token types. |
| `secur32.dll!QueryContextAttributesA/W` | `misc/secur32/schannel.rs` | Added `LIFESPAN`, `KEY_INFO`, `AUTHORITY`, `PROTO_INFO`, `NEGOTIATION_INFO`, `FLAGS`, `UNIQUE_BINDINGS`/`ENDPOINT_BINDINGS` (RFC 5929 `tls-server-end-point:` SHA-256), `CLIENT_SPECIFIED_TARGET`, `LOCAL_CERT_CONTEXT`, `ROOT_STORE`, `SUPPORTED_ALGS`, `CIPHER_STRENGTHS`, `SUPPORTED_PROTOCOLS`, `EAP_KEY_BLOCK` (`export_keying_material`), `SESSION_INFO`, `REMOTE_CERT_CHAIN`, `LOCAL_CERT_CHAIN`. |
| `secur32.dll!QueryCredentialsAttributesA/W` | `misc/secur32/schannel.rs` | Added `SUPPORTED_ALGS`, `CIPHER_STRENGTHS`, and server protocol flags in `SUPPORTED_PROTOCOLS`. |
| `wintrust.dll!WinVerifyTrust` (action dispatch) | `misc/wintrust.rs` | Implemented all 6 standard action GUIDs (`GENERIC_VERIFY_V2`, `TRUSTPROVIDER_TEST`, `GENERIC_CERT_VERIFY`, `GENERIC_CHAIN_VERIFY`, `DRIVER_ACTION_VERIFY`, `HTTPSPROV_ACTION`), returning `TRUST_E_ACTION_UNKNOWN` for unknown GUIDs. |
| `wintrust.dll!WinVerifyTrust` (union choice) | `misc/wintrust.rs` | Implemented all 5 `WINTRUST_DATA.dwUnionChoice` modes (`FILE`, `CATALOG`, `BLOB`, `SIGNER`, `CERT`) backed by real Authenticode/PKCS#7/cert-chain verification, returning `TRUST_E_SUBJECT_FORM_UNKNOWN` for invalid choices. |
