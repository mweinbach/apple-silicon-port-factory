# Stub & API Gap Backlog — `stubs/crypto-cng-capi`

Scope: 28 stub sites across `crates/win32-rt/src/misc/bcrypt.rs` (20) and `crates/win32-rt/src/advapi32.rs` (8).

- **Sites in scope before:** 28
- **Sites in scope after:** 0
- **Made real:** 28
- **Native-unsupported (matches Windows without the feature):** 0
- **Leftovers:** 0

## Sites Removed

| DLL!API | Outcome | Notes |
|---|---|---|
| `bcrypt.dll!BCryptOpenAlgorithmProvider` | Made real | Added MD2, MD4, AES-CMAC, AES-GMAC, 3DES, 3DES_112, DES, RC2, RC4, ECDH (`ECDH`, `ECDH_P256`, `ECDH_P384`, `ECDH_P521`), `FIPS186DSARNG`, KDF identifiers, and `BCRYPT_HASH_REUSABLE_FLAG`; unrecognized algorithm names return `STATUS_NOT_FOUND` per Windows CNG. |
| `bcrypt.dll!BCryptGetProperty` (alg) | Made real | Added `PrimitiveType`, `IsKeyedHash`, `IsReusableHash`, `MultiObjectLength`, `EffectiveKeyLength`, `ECCCurveNameList`, CCM/GCM `AuthTagLength`, full RSA `PaddingSchemes` (`0x1E`), and all new symmetric/ECDH/MAC properties. |
| `bcrypt.dll!BCryptGetProperty` (hash) | Made real | Added `IsKeyedHash`, `IsReusableHash`, `PrimitiveType`, and `AES-CMAC` / `AES-GMAC` digest & block lengths. |
| `bcrypt.dll!BCryptGetProperty` (key) | Made real | Added 3DES/3DES_112/DES/RC2/RC4 properties, `IV`, `EffectiveKeyLength`, `PaddingSchemes`, `PublicKeyLength`, `PrimitiveType`, and ECDH key properties. |
| `bcrypt.dll!BCryptImportKeyPair` (wrapped `hImportKey`) | Made real | Per Microsoft Learn (`BCryptImportKeyPair`), `hImportKey` *"is not currently used and should be NULL"*; non-null handles return `STATUS_NOT_SUPPORTED` matching Windows. |
| `bcrypt.dll!BCryptImportKeyPair` (RSA blobs) | Made real | Added `CAPIPUBLICBLOB` / `CAPIPRIVATEBLOB` (little-endian legacy CAPI RSA blobs) alongside CNG RSA blobs. |
| `bcrypt.dll!BCryptImportKeyPair` (ECC blobs) | Made real | Generalized to support both ECDSA and ECDH (`ECK1`..`ECK6`) plus generic ECC blob magics (`ECDP`/`ECDV`/`ECKP`/`ECKV`). |
| `bcrypt.dll!BCryptImportKeyPair` (other alg) | Made real | All CNG asymmetric algorithms (RSA, ECDSA, ECDH) are supported; non-asymmetric algorithms return `STATUS_NOT_SUPPORTED` matching Windows. |
| `bcrypt.dll!BCryptSignHash` (RSA padding) | Made real | Added MD2/MD4 DigestInfo OIDs and strict validation of `BCRYPT_PAD_PKCS1` / `BCRYPT_PAD_PSS` (invalid padding flags return `STATUS_INVALID_PARAMETER` matching Windows). |
| `bcrypt.dll!BCryptVerifySignature` (RSA padding) | Made real | Added `BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID` (0x40) and MD2/MD4 DigestInfo OIDs alongside PKCS#1 v1.5 and PSS. |
| `bcrypt.dll!bcrypt_aes_crypt` (`BCryptEncrypt`/`BCryptDecrypt`) | Made real | Generalized to `bcrypt_crypt`: added AES-CCM (NIST SP 800-38C), DES/3DES/3DES_112 (CommonCrypto), RC2 (RFC 2268), RC4 stream cipher, and RSA encryption/decryption (`BCRYPT_PAD_OAEP` with arbitrary hash & label, `BCRYPT_PAD_PKCS1`, `BCRYPT_PAD_NONE`). |
| `bcrypt.dll!BCryptImportKey` (wrapped `hImportKey`) | Made real | Implemented RFC 3394 / RFC 3565 AES Key Unwrap for `Rfc3565KeyWrapBlob` (`BCRYPT_AES_WRAP_KEY_BLOB`). |
| `bcrypt.dll!BCryptImportKey` (blob types) | Made real | Extended `KeyDataBlob` and `OpaqueKeyBlob` (v1/v2) across all symmetric ciphers (AES, 3DES, 3DES_112, DES, RC2, RC4). |
| `bcrypt.dll!BCryptExportKey` (wrapped `hExportKey`) | Made real | Implemented RFC 3394 / RFC 3565 AES Key Wrap for `Rfc3565KeyWrapBlob`. |
| `bcrypt.dll!BCryptExportKey` (blob types) | Made real | Extended `KeyDataBlob`, `OpaqueKeyBlob`, `CAPIPUBLICBLOB`, `CAPIPRIVATEBLOB`, and ECDH `ECCPUBLICBLOB`/`ECCPRIVATEBLOB`. |
| `bcrypt.dll!BCryptGenerateSymmetricKey` | Made real | Extended to AES, 3DES, 3DES_112, DES, RC2, and RC4. |
| `bcrypt.dll!BCryptGenerateKeyPair` | Made real | Extended to ECDH (`ECDH`, `ECDH_P256`, `ECDH_P384`, `ECDH_P521`) plus companion `BCryptSecretAgreement`, `BCryptDeriveKey` (`TRUNCATE`/`HASH`/`HMAC`/`TLS_PRF`/`SP800_56A_CONCAT`), and `BCryptDestroySecret`. |
| `bcrypt.dll!BCryptSetProperty` (`ChainingMode`) | Made real | Added `ChainingModeCCM` and `ChainingModeN/A` and validated per-cipher chaining mode support across all block/stream ciphers. |
| `bcrypt.dll!BCryptSetProperty` (`ECCCurveName`) | Made real | Extended to generic `ECDH` algorithm and pending key handles alongside `ECDSA`. |
| `bcrypt.dll!BCryptSetProperty` (other properties) | Made real | Added `IV` (symmetric keys & `AES-GMAC` hashes), `EffectiveKeyLength` (RC2), `KeyLength` (pending ECC & RSA keys), and per-cipher `MessageBlockLength`. |
| `advapi32.dll!CryptDecrypt` | Made real | Implemented CAPI AES/3DES/3DES_112/DES/RC2 (ECB/CBC/CFB + PKCS#5 padding + running IV), RC4, RSA (PKCS#1 v1.5 & `CRYPT_OAEP`, little-endian ciphertext), and optional `hHash`. |
| `advapi32.dll!CryptEncrypt` | Made real | Implemented CAPI symmetric & RSA encryption with size query (`pbData == NULL`), `ERROR_MORE_DATA`, PKCS#5 padding on `Final`, little-endian RSA ciphertext, and optional `hHash`. |
| `advapi32.dll!CryptDestroyKey` | Made real | Releases CAPI key handles and returns `NTE_BAD_KEY` on invalid handles. |
| `advapi32.dll!CryptExportKey` | Made real | Implemented `PUBLICKEYBLOB`, `PRIVATEKEYBLOB`, `PLAINTEXTKEYBLOB`, and `SIMPLEBLOB` (RSA PKCS#1 v1.5 / `CRYPT_OAEP` little-endian wrapped session key) with `CRYPT_EXPORTABLE` enforcement. |
| `advapi32.dll!CryptImportKey` | Made real | Implemented `PUBLICKEYBLOB`, `PRIVATEKEYBLOB`, `PLAINTEXTKEYBLOB`, and `SIMPLEBLOB` (unwrapping with RSA private key), updating container user keys on private-key import. |
| `advapi32.dll!CryptGetUserKey` | Made real | Retrieves `AT_KEYEXCHANGE` / `AT_SIGNATURE` RSA keys from ephemeral or named persistent containers, returning `NTE_NO_KEY` when absent. |
| `advapi32.dll!CryptSignHashW` | Made real | Implemented RSA PKCS#1 v1.5 signing (little-endian signature, optional `CRYPT_NOHASHOID`), plus companion `CryptSignHashA`, `CryptVerifySignatureW/A`, `CryptGenKey`, `CryptDeriveKey`, `CryptDuplicateKey`, `CryptDuplicateHash`, `CryptGetKeyParam`, `CryptSetKeyParam`. |
| `advapi32.dll!GetTokenInformation` | Made real | Implemented `TOKEN_INFORMATION_CLASS` 1..=48 with real `TokenObj` handles, Windows 8+ pseudo-handles (`-4`/`-5`/`-6`), impersonation vs primary semantics (`TokenImpersonationLevel`), and documented error codes for restricted/absent classes (`TokenSessionReference` -> `ERROR_INVALID_PARAMETER`, `TokenAuditPolicy` -> `ERROR_PRIVILEGE_NOT_HELD`, `TokenLinkedToken` -> `ERROR_NO_SUCH_LOGON_SESSION`). |

## Native-unsupported (matches Windows without the feature)

None — all 28 sites in scope are natively implemented on Apple CommonCrypto, `Security.framework` (`SecKey`), and POSIX primitives.

## Leftovers

None — 0 stub sites remain in `crates/win32-rt/src/misc/bcrypt.rs` or `crates/win32-rt/src/advapi32.rs`.
