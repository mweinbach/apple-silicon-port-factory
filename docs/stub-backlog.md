# Stub Backlog & Zero-Stub Tracker

This index is generated from the checked-in static inventory [`db/stub_inventory.tsv`](../db/stub_inventory.tsv)
and aggregates every per-thread backlog file placed in [`docs/stub-backlog.d/`](stub-backlog.d/).

> **Zero-stub policy:** Do not add new `stub!()`, `failing!()`, or `win64_stubs!()` sites anywhere in `crates/`.
> Both `cargo opsdb -- doctor` and `.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh` fail when a new site appears.
> Regenerate this file and `db/stub_inventory.tsv` after eliminating stubs with `cargo opsdb -- stub-sync`.

## Summary

| Metric | Count |
|---|---:|
| Total remaining stub sites | **42** |
| `stub` (partial Win32/OS/D3D/UCRT branches) | 26 |
| `failing` (honest-failing macro exports) | 1 |
| `middleware_stub` (third-party/engine middleware fallbacks) | 15 |
| DLLs with remaining stubs | 8 |

## Per-Thread Backlog Notes (`docs/stub-backlog.d/`)

| File | Summary |
|---|---|
| [`com-audio-net.md`](stub-backlog.d/com-audio-net.md) | Stubs 7: COM/OLE, Audio, Winsock, libcurl and DbgHelp (`stubs/com-audio-net`) |
| [`crypto-certs.md`](stub-backlog.d/crypto-certs.md) | Stubs 4: crypt32, Schannel and WinVerifyTrust |
| [`crypto-cng-capi.md`](stub-backlog.d/crypto-cng-capi.md) | Stub & API Gap Backlog — `stubs/crypto-cng-capi` |
| [`graphics.md`](stub-backlog.d/graphics.md) | Stub Backlog — D3D11, D3D12 and DXGI (`stubs/graphics`) |
| [`guest-corpus-bench.md`](stub-backlog.d/guest-corpus-bench.md) | Stub & API Gap Backlog — `toolkit/guest-corpus-bench` |
| [`kernel-core.md`](stub-backlog.d/kernel-core.md) | Stub Backlog — `stubs/kernel-core` |
| [`middleware.md`](stub-backlog.d/middleware.md) | Stub & API Gap Backlog — `stubs/middleware` (Vendor Middleware & Vulkan Extensions) |
| [`window-input-ui.md`](stub-backlog.d/window-input-ui.md) | Stub Backlog — Window, Input, UI & Text DLLs (`stubs/window-input-ui`) |

### From `docs/stub-backlog.d/com-audio-net.md`

# Stubs 7: COM/OLE, Audio, Winsock, libcurl and DbgHelp (`stubs/com-audio-net`)

Scope: **37** stub sites across 7 DLLs (`ole32.dll`, `dbghelp.dll`, `mmdevapi.dll`, `dsound.dll`, `xaudio2_9redist.dll`, `ws2_32.dll`, `libcurl.dll`) reduced from **37 → 0** (**34 made real**, **3 native-unsupported matching Windows without the feature**, **0 leftovers**).

## Made Real (34 sites)

| DLL!API | File | Implementation Summary |
|---|---|---|
| `ole32.dll!IPropertyValue::GetDateTime` | `crates/win32-rt/src/misc/ole32.rs` | Stores and returns WinRT `Windows.Foundation.DateTime` (`i64` UniversalTime). |
| `ole32.dll!IPropertyValue::GetTimeSpan` | `crates/win32-rt/src/misc/ole32.rs` | Stores and returns WinRT `Windows.Foundation.TimeSpan` (`i64` Duration). |
| `ole32.dll!IPropertyValue::GetPoint` | `crates/win32-rt/src/misc/ole32.rs` | Stores and returns WinRT `Windows.Foundation.Point` (`{ f32, f32 }`). |
| `ole32.dll!IPropertyValue::GetSize` | `crates/win32-rt/src/misc/ole32.rs` | Stores and returns WinRT `Windows.Foundation.Size` (`{ f32, f32 }`). |
| `ole32.dll!IPropertyValue::GetRect` | `crates/win32-rt/src/misc/ole32.rs` | Stores and returns WinRT `Windows.Foundation.Rect` (`{ f32, f32, f32, f32 }`). |
| `ole32.dll!IPropertyValue::Get*Array` (19 types) | `crates/win32-rt/src/misc/ole32.rs` | CoTaskMem-allocated array getters for all 19 WinRT `PropertyType` array variants (`UInt8Array`..`RectArray`), duplicating `HSTRING`s and `AddRef`ing `IInspectable*` elements. |
| `ole32.dll!IPropertyValueStatics::CreateDateTime` | `crates/win32-rt/src/misc/ole32.rs` | Constructs `IPropertyValue` with `PropertyType::DateTime`. |
| `ole32.dll!IPropertyValueStatics::CreateTimeSpan` | `crates/win32-rt/src/misc/ole32.rs` | Constructs `IPropertyValue` with `PropertyType::TimeSpan`. |
| `ole32.dll!IPropertyValueStatics::CreatePoint` | `crates/win32-rt/src/misc/ole32.rs` | Constructs `IPropertyValue` with `PropertyType::Point` (with Win64 XMM ABI bridge). |
| `ole32.dll!IPropertyValueStatics::CreateSize` | `crates/win32-rt/src/misc/ole32.rs` | Constructs `IPropertyValue` with `PropertyType::Size` (with Win64 XMM ABI bridge). |
| `ole32.dll!IPropertyValueStatics::CreateRect` | `crates/win32-rt/src/misc/ole32.rs` | Constructs `IPropertyValue` with `PropertyType::Rect`. |
| `ole32.dll!IPropertyValueStatics::Create*Array` (19 types) | `crates/win32-rt/src/misc/ole32.rs` | Copies input slices into retained `PropertyValue` array variants (`BooleanArray` normalized, `StringArray` duplicated, `InspectableArray` `AddRef`/`Release`-managed). |
| `dbghelp.dll!StackWalk64` | `crates/win32-rt/src/misc/dbghelp.rs` | Unwinds x64 frames via `crate::unwind::safe_virtual_unwind` (`.pdata`), supporting `ReadMemoryRoutine`, `FunctionTableAccessRoutine`, `GetModuleBaseRoutine`, and leaf-frame `[RSP]` fallback. |
| `dbghelp.dll!MiniDumpWriteDump` | `crates/win32-rt/src/misc/dbghelp.rs` | Emits valid Microsoft `MDMP` (`0x504D444D`) files via `WriteFile` with `SystemInfoStream`, `MiscInfoStream`, `ModuleListStream` (PE timestamp/checksum/RSDS CodeView), `ThreadListStream`, `MemoryListStream`, optional `ExceptionStream`, and user streams. |
| `dbghelp.dll!SymGetLineFromAddr64` | `crates/win32-rt/src/misc/dbghelp.rs` | Pure-Rust MSF 7.0 + `/names` + DBI + CodeView C13 (`DEBUG_S_FILECHKSMS` / `DEBUG_S_LINES`) PDB reader; returns `ERROR_MOD_NOT_FOUND` (126) / `ERROR_INVALID_ADDRESS` (487) when absent. |
| `dbghelp.dll!SymGetLineFromAddrW64` | `crates/win32-rt/src/misc/dbghelp.rs` | Wide UTF-16 wrapper backed by the same MSF 7.0 / C13 PDB reader. |
| `mmdevapi.dll!IMMDevice::Activate` | `crates/win32-rt/src/audio/mmdevapi.rs` | Implemented `IAudioEndpointVolume` (21-slot COM vtable backed by CoreAudio HAL `VirtualMainVolume`/`VolumeScalar`/`Mute`/`VolumeRangeDecibels` on macOS, atomic fallback on Null/iOS); unknown IIDs return standard COM `E_NOINTERFACE`. |
| `mmdevapi.dll!IAudioClient::GetService` | `crates/win32-rt/src/audio/audioclient.rs` | Implemented `IAudioClock2`, `ISimpleAudioVolume`, `IAudioStreamVolume` (per-channel gain applied live in `submit()`), and `IAudioSessionControl`/`2`; unknown IIDs return `E_NOINTERFACE`. |
| `mmdevapi.dll!IAudioClient2::GetBufferSizeLimits` | `crates/win32-rt/src/audio/audioclient.rs` | Validates format and out-pointers and computes real shared-mode min/max buffer duration bounds (`MIN_PERIOD_HNS` / `DEFAULT_PERIOD_HNS` to `20_000_000` hns). |
| `mmdevapi.dll!IAudioClient::Initialize(LOOPBACK)` | `crates/win32-rt/src/audio/audioclient.rs` | Real loopback capture via macOS 14.2+ CoreAudio Process Tap (`AudioHardwareCreateProcessTap` + private aggregate device + `IOProc`, dynamically resolved) with automatic fallback to an in-process per-endpoint WASAPI render loopback bus. |
| `mmdevapi.dll!ISpatialAudioObjectRenderStream::GetService` | `crates/win32-rt/src/audio/spatial.rs` | Implemented `IAudioClock`, `IAudioClock2`, `ISimpleAudioVolume`, `IAudioStreamVolume` (applied live in `EndUpdatingAudioObjects`), and `IAudioSessionControl`/`2`; unknown IIDs return `E_NOINTERFACE`. |
| `dsound.dll!IDirectSoundBuffer8::SetFX` | `crates/win32-rt/src/misc/dsound/com_vtbl.rs` | Full DirectSound 8 software DSP chain (`SetFX`, `AcquireResources`, `GetObjectInPath`) for all 9 standard effects (`Gargle`, `Chorus`, `Flanger`, `Echo`, `Distortion`, `Compressor`, `ParamEq`, `I3DL2Reverb`, `WavesReverb`) with live stereo DSP in `mix_dsound_frames` and real `IDirectSoundFX*` COM objects. |
| `xaudio2_9redist.dll!parse_wave_format` (Extensible SubFormat) | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs` | Decodes `KSDATAFORMAT_SUBTYPE_{ADPCM,ALAW,MULAW}` in addition to PCM/IEEE_FLOAT; rejects invalid/unsupported subtypes with `XAUDIO2_E_INVALID_CALL` matching desktop Windows `xaudio2_9redist.dll`. |
| `xaudio2_9redist.dll!parse_wave_format` (Format Tag) | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs` | Real block/sample decoders for MS-ADPCM (`0x0002`), IMA/DVI ADPCM (`0x0011`), ITU G.711 A-law (`0x0006`), and μ-law (`0x0007`); rejects invalid tags and Xbox-only XMA2 with `XAUDIO2_E_INVALID_CALL`. |
| `ws2_32.dll!WSAAsyncSelect` | `crates/win32-rt/src/misc/ws2_32.rs` | Reuses the background `poll(2)` reactor to post real Win32 window messages (`PostMessageW(hwnd, wmsg, sock, WSAMAKESELECTREPLY(ev, err))`), sets guest nonblocking mode, and inherits across `accept`. |
| `ws2_32.dll!WSACancelAsyncRequest` | `crates/win32-rt/src/misc/ws2_32.rs` | Backed by an async-task state registry returning `0` on pending cancellation, `WSAEALREADY` on completed tasks, and `WSAEINVAL` on invalid/null handles. |
| `ws2_32.dll!WSAConnectByNameA` | `crates/win32-rt/src/misc/ws2_32.rs` | Resolves host/service via `getaddrinfo` (including dual-stack IPv4-mapped IPv6), connects with `WinTimeVal` timeout, populates `LocalAddress`/`RemoteAddress`, and returns `BOOL`. |
| `ws2_32.dll!WSAConnectByNameW` | `crates/win32-rt/src/misc/ws2_32.rs` | UTF-16 variant sharing `connect_by_name_impl`. |
| `ws2_32.dll!WSAJoinLeaf` | `crates/win32-rt/src/misc/ws2_32.rs` | Real IPv4 (`IP_ADD_MEMBERSHIP`) and IPv6 (`IPV6_JOIN_GROUP`) multicast group membership join plus optional `connect` for `JL_SENDER_ONLY`/`JL_BOTH`. |
| `ws2_32.dll!WSARecvMsg` | `crates/win32-rt/src/misc/ws2_32.rs` | Real POSIX `recvmsg(2)` with Win64 `WSAMSG`/`WSACMSGHDR` ↔ Darwin `cmsghdr` translation (`IP_PKTINFO`, `IPV6_PKTINFO`, `IP_TTL`, `IP_TOS`, `IPV6_HOPLIMIT`, `IPV6_TCLASS`), sync + overlapped `ASYNC_IO` reactor support, and `WSAID_WSARECVMSG` extension thunk. |
| `ws2_32.dll!WSASendMsg` | `crates/win32-rt/src/misc/ws2_32.rs` | Real POSIX `sendmsg(2)` with Win64 `WSAMSG`/`WSACMSGHDR` translation, sync + overlapped `ASYNC_IO` reactor support, and `WSAID_WSASENDMSG` extension thunk. |
| `ws2_32.dll!WSAIoctl` | `crates/win32-rt/src/misc/ws2_32.rs` | Implemented `SIOCATMARK`, `SIO_KEEPALIVE_VALS`, `SIO_GET_INTERFACE_LIST`, `SIO_ADDRESS_LIST_QUERY`, `SIO_ROUTING_INTERFACE_QUERY`, plus `WSAID_WSARECVMSG`/`WSAID_WSASENDMSG`; unknown IOCTLs return standard `WSAEOPNOTSUPP`. |
| `libcurl.dll!curl_easy_setopt` | `crates/win32-rt/src/misc/libcurl/easy.rs` | Wired `CURLOPT_IPRESOLVE`, `CURLOPT_PROTOCOLS`/`_REDIR_PROTOCOLS`/`_STR`, `CURLOPT_RESOLVE`, `CURLOPT_XOAUTH2_BEARER`, `CURLOPT_UNRESTRICTED_AUTH`, `CURLOPT_RESUME_FROM`/`_LARGE`, `CURLOPT_MAXFILESIZE`/`_LARGE`, `CURLOPT_DEFAULT_PROTOCOL`, `CURLOPT_REQUEST_TARGET`, `CURLOPT_TCP_KEEPALIVE`/`_KEEPIDLE`/`_KEEPINTVL`, `CURLOPT_BUFFERSIZE`, and `CURLOPT_XFERINFOFUNCTION`/`_NOPROGRESS`; disabled protocols return `CURLE_NOT_BUILT_IN` and unknown options return `CURLE_UNKNOWN_OPTION`. |
| `libcurl.dll!curl_easy_getinfo` | `crates/win32-rt/src/misc/libcurl/easy.rs` | Implemented full HTTP/HTTPS timing (`APPCONNECT`/`PRETRANSFER`/`STARTTRANSFER`/`REDIRECT`/`NAMELOOKUP`/`CONNECT` double and `_T`), `PRIMARY_PORT`, `LOCAL_IP`, `LOCAL_PORT`, `SCHEME`, `PROTOCOL`, `EFFECTIVE_METHOD`, `FILETIME`/`_T`, `CONDITION_UNMET`, `HTTPAUTH_AVAIL`, `PROXYAUTH_AVAIL`, `RETRY_AFTER`, `ACTIVESOCKET`/`LASTSOCKET`; unknown info IDs return `CURLE_UNKNOWN_OPTION`. |

## Native-unsupported (matches Windows without the feature) (3 sites)

| DLL!API | Returned Code | Windows Behavior & Documentation Citation |
|---|---|---|
| `mmdevapi.dll!IAudioClient::Initialize` (`AUDCLNT_SHAREMODE_EXCLUSIVE`) | `AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED` (`0x8889000E`) | Matches [Microsoft Learn `IAudioClient::Initialize`](https://learn.microsoft.com/en-us/windows/win32/api/audioclient/nf-audioclient-iaudioclient-initialize): when an audio endpoint does not allow exclusive-mode hardware takeover (Windows sound setting "Allow applications to take exclusive control of this device" unchecked, Bluetooth/virtual endpoints, or macOS/iOS shared CoreAudio/`AVAudioSession` routing), Windows returns `AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED` so callers fall back to `AUDCLNT_SHAREMODE_SHARED`. |
| `ws2_32.dll!WSADuplicateSocketA` / `WSADuplicateSocketW` (cross-process `pid != current_pid`) | `SOCKET_ERROR` with `WSAEOPNOTSUPP` (`10045`) | Matches [Microsoft Learn `WSADuplicateSocketW`](https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsaduplicatesocketw): same-process duplication works via `dup(2)` + protocol-info token; cross-process fd injection into an external macOS process we do not host cannot exist natively, so the provider returns `WSAEOPNOTSUPP`. |
| `ws2_32.dll!WSAGetQOSByName` | `FALSE` with `WSAEOPNOTSUPP` (`10045`) | Matches [Microsoft Learn `WSAGetQOSByName`](https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsagetqosbyname): Winsock2 RSVP/QOS templates were deprecated in Windows Vista and the built-in Windows TCP/IP service provider (`MSAFD Tcpip`) does not advertise `XP1_QOS_SUPPORTED`, returning `FALSE` with `WSAEOPNOTSUPP` after parameter validation. |

## Leftovers

None — **0** remaining stub sites in scope.

### From `docs/stub-backlog.d/crypto-certs.md`

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

### From `docs/stub-backlog.d/crypto-cng-capi.md`

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
| `bcrypt.dll!BCryptVerifySignature` (RSA padding) | Made real | Added `BCRYPT_PAD_PKCS1_OPTIONAL_HASH_OID` (0x10) and MD2/MD4 DigestInfo OIDs alongside PKCS#1 v1.5 and PSS. |
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

### From `docs/stub-backlog.d/graphics.md`

# Stub Backlog — D3D11, D3D12 and DXGI (`stubs/graphics`)

Scope: 26 stub sites across `d3d12/mod.rs` (2), `dxgi/mod.rs` (7), `dxgi/adapter.rs` (3),
`d3d11/device.rs` (8), `d3d11/context.rs` (5), and `d3d11/resources.rs` (1).

- **Before:** 26 sites
- **Made real:** 16 sites
- **Native-unsupported (matches Windows without the hardware/OS feature):** 6 sites
- **Honest leftovers remaining in `db/stub_inventory.tsv`:** 4 sites

## Remaining Leftovers in Scope (4 sites)

| DLL!API | Location | Why Not Implemented Yet | Next Step |
|---|---|---|---|
| `d3d11.dll!ID3D11Device::CreateClassLinkage` | `crates/win32-rt/src/d3d11/device.rs` | Returning `S_OK` without implementing dynamic shader interface dispatch (`SHEX` `IFCE` / `dcl_interface` / `dcl_function_table` / `fcall`) would let apps pass feature detection and then execute the wrong shader implementations. | Parse DXBC `IFCE` interface/type/instance tables, wire `pClassLinkage` at `Create*Shader` and `ppClassInstances` at `{VS,HS,DS,GS,PS,CS}SetShader` into PSO specialization or `dxilconv`/`dxil-metal` `fcall` lowering, then return `S_OK` with an end-to-end GPU test verifying distinct class instances produce distinct shader output. |
| `d3d11.dll!ID3D11Device::CreateGeometryShaderWithStreamOutput` | `crates/win32-rt/src/d3d11/device.rs` | Apple Metal Shader Converter does not expose stream-out synthesis in its public C header (`metal_irconverter.h`; `IRCompilerSetStreamOutEnabled` / `IRCreateStreamOutFunction` are private undocumented C++ symbols that must not be called). Lowering arbitrary DXBC/DXIL stream-out shaders cleanly requires a DXIL LLVM 3.7 bitcode rewriter that translates `dx.op.storeOutput` / `dx.op.emitStream` into `RWByteAddressBuffer` UAV writes with per-stream atomic vertex counters according to `D3D11_SO_DECLARATION_ENTRY` strides. | Implement a clean-room DXIL stream-out bitcode lowering pass in `crates/dxil-metal` (rewriting stream-out outputs to internal UAV slots + atomic counter buffers), bind `so_targets` UAVs in `d3d11/context.rs`, and verify with a Metal 4 GPU E2E readback test. |
| `d3d11.dll!ID3D11DeviceContext::DrawAuto` | `crates/win32-rt/src/d3d11/context.rs` | Depends on `CreateGeometryShaderWithStreamOutput` populating per-stream GPU vertex-count atomic buffers for bound `SOSetTargets` buffers. | Once stream-out writes atomic vertex counts per SO target buffer, dispatch a tiny compute helper (or copy directly into a 16-byte `DrawInstancedIndirect` argument buffer `{vertexCount, 1, 0, 0}`) and execute via `CommandList::execute_indirect`. |
| `d3d11.dll!ID3D11DeviceContext::GetData` (`D3D11_QUERY_PIPELINE_STATISTICS`) | `crates/win32-rt/src/d3d11/context.rs` | CPU-submitted counts (`IAVertices`, `IAPrimitives`, `VSInvocations`, `GSInvocations`, `HSInvocations`, `CSInvocations`) are now accurately accumulated between `Begin` and `End`, but GPU-side counters (`CInvocations`, `CPrimitives`, `PSInvocations`, `GSPrimitives`, `DSInvocations`, and indirect draw counts) cannot be sampled on Apple Silicon because `MTLDevice.counterSets` only exposes `timestamp` (`GPUTimestamp`) and `MTL4CounterHeapType` only defines `Invalid` and `Timestamp` (no `MTLCommonCounterSetStatistic`). The `stub!` stays in place so GPU-side zeroes are never reported silently. | When Apple Metal exposes stage-invocation/clipper hardware counter sets on Apple Silicon, wire them through `d3d12-metal::metal::query` (`MTL4CounterHeap`), or add an opt-in shader-instrumentation pass for `PSInvocations`/`GSPrimitives`/`DSInvocations`. |

## Native-Unsupported (Matches Windows Without the Feature — 6 sites)

| DLL!API | Location | Return Code & Behavior | Microsoft Learn Reference |
|---|---|---|---|
| `dxgi.dll!DXGIGetDebugInterface1` | `crates/win32-rt/src/dxgi/mod.rs` | Validates `ppDebug` (`E_INVALIDARG` if null; zeroes `*ppDebug`) and returns `E_NOINTERFACE` because Windows Graphics Tools (`dxgidebug.dll` / `IDXGIDebug` / `IDXGIInfoQueue`) is not installed. | [DXGIGetDebugInterface1 function](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-dxgigetdebuginterface1) ("If the Graphics Tools optional feature is not installed, `DXGIGetDebugInterface1` returns `E_NOINTERFACE`") |
| `dxgi.dll!IDXGIFactory4::EnumWarpAdapter` | `crates/win32-rt/src/dxgi/mod.rs` | Validates `ppvAdapter` (`E_INVALIDARG` if null; zeroes `*ppvAdapter`) and returns `DXGI_ERROR_NOT_FOUND` because Microsoft's WARP (`d3d10warp.dll`) software rasterizer is not installed. | [IDXGIFactory4::EnumWarpAdapter method](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nf-dxgi1_4-idxgifactory4-enumwarpadapter) & [DXGI_ERROR](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-error) |
| `dxgi.dll!IDXGIFactory::CreateSoftwareAdapter` | `crates/win32-rt/src/dxgi/mod.rs` | Validates `Module` and `ppAdapter` (`E_INVALIDARG` if null; zeroes `*ppAdapter`) and returns `DXGI_ERROR_UNSUPPORTED` because Windows user-mode software display driver DLLs cannot execute as native Metal adapters. | [IDXGIFactory::CreateSoftwareAdapter method](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-createsoftwareadapter) |
| `dxgi.dll!IDXGIFactory2::CreateSwapChainForCoreWindow` & `CreateSwapChainForComposition` | `crates/win32-rt/src/dxgi/mod.rs` | Split into distinct Win64 methods validating `pDevice`/`pWindow`/`pDesc`/`ppSwapChain` (`E_INVALIDARG` on null, zeroes `*ppSwapChain`) and returning `DXGI_ERROR_UNSUPPORTED` because UWP `CoreWindow` and DirectComposition swap chains do not exist outside WinRT/DComp. | [IDXGIFactory2::CreateSwapChainForCoreWindow](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforcorewindow) & [CreateSwapChainForComposition](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforcomposition) |
| `d3d11.dll!ID3D11Device::CheckCounter` | `crates/win32-rt/src/d3d11/device.rs` | Validates `pDesc` (`E_INVALIDARG` if null) and returns `DXGI_ERROR_UNSUPPORTED`, matching Windows D3D11 drivers whose `CheckCounterInfo` reports `LastDeviceDependentCounter == 0` (Apple Silicon exposes no device-dependent performance counters). | [ID3D11Device::CheckCounter method](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11device-checkcounter) & [ID3D11Device::CheckCounterInfo](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11device-checkcounterinfo) |
| `d3d11.dll!ID3D11Device::CreateCounter` | `crates/win32-rt/src/d3d11/device.rs` | Zeroes `*ppCounter`, validates `pCounterDesc` (`E_INVALIDARG` if null), and returns `DXGI_ERROR_UNSUPPORTED`, matching Windows D3D11 drivers that expose zero device-dependent counters. | [ID3D11Device::CreateCounter method](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11device-createcounter) |

## Resolved Sites — Made Real (16 sites)

| DLL!API | Location | Implementation Summary |
|---|---|---|
| `d3d12.dll!D3D12CreateRootSignatureDeserializer` | `crates/win32-rt/src/d3d12/mod.rs` | Parses DXBC/`RTS0` blobs via `dxil_metal::root_signature::RootSignatureDesc::from_blob` and returns a heap-stable `ID3D12RootSignatureDeserializer` COM object exposing `GetRootSignatureDesc` in exact Windows x64 ABI layout. |
| `d3d12.dll!D3D12CreateVersionedRootSignatureDeserializer` | `crates/win32-rt/src/d3d12/mod.rs` | Returns a real `ID3D12VersionedRootSignatureDeserializer` supporting `GetUnconvertedRootSignatureDesc` and `GetRootSignatureDescAtVersion` with lossless v1.0 ↔ v1.1 ↔ v1.2 conversion (rejecting lossy v1.2 static-sampler-flag down-conversions with `E_INVALIDARG`). |
| `dxgi.dll!IDXGIFactory2::GetSharedResourceAdapterLuid` | `crates/win32-rt/src/dxgi/mod.rs` | Resolves live D3D11 NT shared handles and legacy KMT handles in `d3d11::resources` and writes the active adapter LUID (`DXGI_ERROR_INVALID_CALL` / `E_INVALIDARG` on invalid/null inputs). |
| `dxgi.dll!IDXGIFactory2::Register{Occlusion,Stereo}StatusWindow` | `crates/win32-rt/src/dxgi/mod.rs` | Real cookie-keyed window notification registries wired to AppKit `windowDidChangeOcclusionState:` (`PostMessageW(hwnd, wMsg, 0, 0)`) with matching `Unregister*Status`. |
| `dxgi.dll!IDXGIFactory2::Register{Occlusion,Stereo}StatusEvent` & `IDXGIFactory7::RegisterAdaptersChangedEvent` | `crates/win32-rt/src/dxgi/mod.rs` | Real cookie-keyed Win32 `HANDLE` event notification registries wired to AppKit `windowDidChangeOcclusionState:` and `applicationDidChangeScreenParameters:` (`SetEvent`) with matching `Unregister*`. |
| `dxgi.dll!IDXGIAdapter3::RegisterHardwareContentProtectionTeardownStatusEvent` | `crates/win32-rt/src/dxgi/adapter.rs` | Real cookie-keyed HCP teardown event registry with `UnregisterHardwareContentProtectionTeardownStatusEvent` and `notify_hcp_teardown()`. |
| `dxgi.dll!IDXGIOutput::WaitForVBlank` | `crates/win32-rt/src/dxgi/adapter.rs` | Synchronizes to the macOS display refresh via CoreVideo `CVDisplayLink` (`CVDisplayLinkCreateWithCGDisplay` + callback condvar wake + headless/timeout monotonic fallback; iOS-safe cfg gating). |
| `dxgi.dll!IDXGIOutput::TakeOwnership` / `ReleaseOwnership` | `crates/win32-rt/src/dxgi/adapter.rs` | Per-output exclusive ownership tracking with `DXGI_ERROR_NOT_CURRENTLY_AVAILABLE` conflict reporting, `E_INVALIDARG` on null device, and automatic cleanup on `Drop`. |
| `d3d11.dll!IDXGIDevice::CreateSurface` | `crates/win32-rt/src/d3d11/device.rs` | Creates `NumSurfaces` real Metal-backed `IDXGISurface2` objects translating `DXGI_USAGE_*` flags into `D3D11_BIND_*` / `D3D11_CPU_ACCESS_*` flags, with rollback on failure. |
| `d3d11.dll!IDXGISurface1::GetDC` / `ReleaseDC` | `crates/win32-rt/src/d3d11/resources.rs` | Validates `D3D11_RESOURCE_MISC_GDI_COMPATIBLE`, non-MSAA, and `B8G8R8A8_UNORM(_SRGB)`; optionally reads back GPU texture when `Discard == FALSE`; creates a real `gdi32` top-down 32bpp BGRA `HDC` backed by staging memory; uploads dirty rect or full surface to Metal on `ReleaseDC`. |
| `d3d11.dll!ID3D11Device1::CreateDeviceContextState` | `crates/win32-rt/src/d3d11/device.rs` | Validates flags, feature-level array, SDK version, and emulated interface IID (`ID3D10Device` / `ID3D10Device1` / `ID3D11Device` / `ID3D11Device1`), selects the highest supported feature level, and creates a real `ID3DDeviceContextState` holding full `ContextState`. |
| `d3d11.dll!ID3D11DeviceContext1::SwapDeviceContextState` | `crates/win32-rt/src/d3d11/context.rs` | Lazily materializes the initial context state, swaps the entire immediate `ContextState` atomically with the target `ID3DDeviceContextState`, and returns the previous state in `ppPreviousState`. |
| `d3d11.dll!ID3D11Device::CreateHullShader` | `crates/win32-rt/src/d3d11/device.rs` | Validates DXBC container bytecode (`E_INVALIDARG` on invalid input, `S_FALSE` when `ppHullShader == NULL`) and creates a real `ID3D11HullShader` COM object. |
| `d3d11.dll!ID3D11Device::CreateDomainShader` | `crates/win32-rt/src/d3d11/device.rs` | Validates DXBC container bytecode (`E_INVALIDARG` on invalid input, `S_FALSE` when `ppDomainShader == NULL`) and creates a real `ID3D11DomainShader` COM object. |
| `d3d11.dll!ID3D11DeviceContext::HSSetShader` | `crates/win32-rt/src/d3d11/context.rs` | Binds `ID3D11HullShader` + class instances onto context state (round-tripped via `HSGetShader`), populates HS CBV/SRV/Sampler root tables (params 6 & 7), and compiles Metal 4 object+mesh tessellation emulation pipelines via `GraphicsPipeline::new_with_tessellation` (with MSC Apache-2.0 tessellator lookup tables at bind point 7). |
| `d3d11.dll!ID3D11DeviceContext::DSSetShader` | `crates/win32-rt/src/d3d11/context.rs` | Binds `ID3D11DomainShader` + class instances onto context state (round-tripped via `DSGetShader`), populates DS CBV/SRV/Sampler root tables (params 8 & 9), and executes full VS+HS+DS(+optional GS)+PS tessellation draws on Metal 4. |

### From `docs/stub-backlog.d/guest-corpus-bench.md`

# Stub & API Gap Backlog — `toolkit/guest-corpus-bench`

Exposed while building and running the headless Windows x86-64 guest corpus (`crates/port-loader/tests/os_guest_corpus.rs` and `crates/port-loader/tests/d3d12_e2e.rs`).

| DLL!API | What the test showed | Next step |
|---|---|---|
| `user32.dll!wsprintfW` | Missing export in `crates/win32-rt/src/user32/misc.rs`: guest binaries importing `wsprintfW` hit an unresolved-import trap thunk instead of formatting UTF-16 output. | Implement variadic `wsprintfW` via `win64_varargs_thunk!` backed by `ucrt::stdio` wide formatter (1024-char Windows cap) and run `cargo opsdb -- win32-sync`. |
| `user32.dll!wsprintfA` | Existing implementation in `crates/win32-rt/src/user32/misc.rs` copies the raw format string and ignores variadic arguments (`let _ = (a1, a2);`). | Replace with a real `win64_varargs_thunk!` formatter backed by `ucrt::stdio` narrow printf formatting (1024-char Windows cap). |

### From `docs/stub-backlog.d/kernel-core.md`

# Stub Backlog — `stubs/kernel-core`

Scope: 28 stub sites across `crates/win32-rt/src/kernel32/{memory,threads,process,threadpool}.rs`, `crates/win32-rt/src/sync.rs`, and `crates/win32-rt/src/ntdll.rs`.

- **Sites before:** 28
- **Sites after:** 0
- **Made real:** 17
- **Native-unsupported (matches Windows without the feature/capability):** 11
- **Leftovers:** 0

## Removed Sites

| DLL!API | Location | Disposition | Summary |
|---|---|---|---|
| `kernel32.dll!VirtualAlloc` (`MEM_WRITE_WATCH`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Hardware write-watch tracking via `PROT_READ` + `try_handle_write_watch_fault` in `signals.rs`, real `GetWriteWatch` (`WRITE_WATCH_FLAG_RESET` partial-reset semantics) and `ResetWriteWatch`. |
| `kernel32.dll!VirtualAlloc` (`PAGE_GUARD`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Maps guard pages `PROT_NONE`, validates against `PAGE_NOACCESS`, and delivers one-shot `STATUS_GUARD_PAGE_VIOLATION` (`0x80000001`) via `consume_guard_page` + `signals.rs` before clearing `PAGE_GUARD`. |
| `kernel32.dll!VirtualAlloc` (`MEM_RESET_UNDO`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Uses Darwin `MADV_FREE_REUSABLE` (7) / `MADV_FREE_REUSE` (8) + `mincore` (`MINCORE_INCORE`) residency tracking to verify pages were not purged before succeeding, failing with `ERROR_INVALID_ADDRESS` if discarded. |
| `kernel32.dll!VirtualProtect` (`PAGE_GUARD`) | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Real `PAGE_GUARD` arming (`PROT_NONE`), validation against `PAGE_NOACCESS`, and one-shot `STATUS_GUARD_PAGE_VIOLATION` delivery through `signals.rs`. |
| `kernel32.dll!FlushProcessWriteBuffers` | `crates/win32-rt/src/kernel32/memory.rs` | Made real | Real process-wide IPI barrier on Apple platforms via Mach `task_threads` + serialized per-thread `thread_suspend`/`thread_resume` (XNU `thread_wait` cross-CPU AST/IPI) bracketed by `SeqCst` fences. |
| `kernel32.dll!TerminateThread` | `crates/win32-rt/src/kernel32/threads.rs` | Made real | Real thread termination for current thread, `CREATE_SUSPENDED` threads, and live sibling threads (`SIGUSR1` async-signal-safe `pthread_exit` + exit code recording + waiter signaling + `pthread_join` stack reaping). |
| `kernel32.dll!SetThreadAffinityMask` | `crates/win32-rt/src/kernel32/threads.rs` | Made real | Tracks per-thread affinity mask against process affinity mask, validates mask bounds, returns exact previous mask, and applies best-effort Darwin `THREAD_AFFINITY_POLICY` tag without lowering QoS. |
| `kernel32.dll!SetThreadGroupAffinity` | `crates/win32-rt/src/kernel32/threads.rs` | Made real | Validates group/mask, tracks per-thread group affinity, writes previous `GROUP_AFFINITY`, applies `THREAD_AFFINITY_POLICY`, and pairs with new `GetThreadGroupAffinity`. |
| `kernel32.dll!AssignProcessToJobObject` | `crates/win32-rt/src/kernel32/process.rs` | Made real | Tracks assigned self and child processes on `JobObj`, enforces `JOB_OBJECT_LIMIT_ACTIVE_PROCESS` (`ERROR_NOT_ENOUGH_QUOTA`), updates `IsProcessInJob`, and terminates child members on `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`. |
| `kernel32.dll!SetInformationJobObject` | `crates/win32-rt/src/kernel32/process.rs` | Made real | Validates struct sizes (`ERROR_BAD_LENGTH`) and flags, stores basic/extended limits, UI restrictions, end-of-job action, completion port, CPU rate control, notification limits, and group affinity; reports them back via `QueryInformationJobObject`. |
| `kernel32.dll!FormatMessageW/A` (`FROM_HMODULE` + `FROM_SYSTEM`) | `crates/win32-rt/src/kernel32/process.rs` | Made real | Parses PE `RT_MESSAGETABLE` (type 11) `MESSAGE_RESOURCE_DATA` / `MESSAGE_RESOURCE_BLOCK` / `MESSAGE_RESOURCE_ENTRY` (Unicode & ANSI), searching module before system fallback. |
| `kernel32.dll!FormatMessageW/A` (`FROM_HMODULE` only) | `crates/win32-rt/src/kernel32/process.rs` | Made real | Same real PE `RT_MESSAGETABLE` parser returning `ERROR_MR_MID_NOT_FOUND` (317) when an ID is absent and `ERROR_INVALID_PARAMETER` when no source flag is given. |
| `kernel32.dll!CloseThreadpoolCleanupGroupMembers` | `crates/win32-rt/src/kernel32/threadpool.rs` | Made real | Parses `TP_CALLBACK_ENVIRON` V1/V3 in `CreateThreadpool{Work,Timer,Wait,Io}`, tracks live members in `TpCleanupGroup`, cancels/waits callbacks (including queued `TpWork` cancellation), invokes `CleanupGroupCancelCallback`, and releases member refs without double-close. |
| `kernel32.dll!InitOnceBeginInitialize` (`INIT_ONCE_ASYNC`) | `crates/win32-rt/src/sync.rs` | Made real | Full 4-state `RTL_RUN_ONCE` machine (`0` uninit, `1` sync, `2` done, `3` async): concurrent non-blocking `INIT_ONCE_ASYNC` racers, single winning `InitOnceComplete(INIT_ONCE_ASYNC)` (`ERROR_GEN_FAILURE` for losers), and Windows-matching `INIT_ONCE_CHECK_ONLY` (`FALSE` + `ERROR_GEN_FAILURE` before completion). |
| `ntdll.dll!NtQueryInformationProcess` | `crates/win32-rt/src/ntdll.rs` | Made real | Validates handle (`STATUS_INVALID_HANDLE`), adds real `ProcessVmCounters` (Mach `task_info`), `ProcessTimes` (`getrusage`), `ProcessPriorityClass`, `ProcessHandleCount` (`handles::live_count`), `ProcessAffinityMask`, `ProcessSessionInformation`, `ProcessBreakOnTermination`, random `ProcessCookie`, `ProcessSubsystemInformation`, `ProcessProtectionInformation`; returns `STATUS_INVALID_INFO_CLASS` for unsupported/unknown classes. |
| `ntdll.dll!NtQueryInformationThread` | `crates/win32-rt/src/ntdll.rs` | Made real | Validates handle, fills full `ThreadBasicInformation`, adds real `ThreadTimes` (Mach `thread_info`), `ThreadQuerySetWin32StartAddress`, `ThreadAmILastThread`, tracked `ThreadHideFromDebugger`, `ThreadGroupInformation`, `ThreadNameInformation` (unaligned byte copy); returns `STATUS_INVALID_INFO_CLASS` for unsupported/unknown classes. |
| `ntdll.dll!NtQuerySystemInformation` | `crates/win32-rt/src/ntdll.rs` | Made real | Adds real `SystemProcessorPerformanceInformation` (Mach `host_processor_info`), `SystemLogicalProcessorInformation` / `Ex`, `SystemKernelDebuggerInformationEx`; returns `STATUS_INVALID_INFO_CLASS` for unsupported/unknown classes (no fabricated telemetry). |
| `kernel32.dll!VirtualAlloc` (`MEM_LARGE_PAGES`) | `crates/win32-rt/src/kernel32/memory.rs` | Native-unsupported | See table below. |
| `kernel32.dll!VirtualAllocEx` (foreign process) | `crates/win32-rt/src/kernel32/memory.rs` | Native-unsupported | Current-process pseudo-handle and `OpenProcess(self)` work; foreign processes return `ERROR_ACCESS_DENIED` (invalid/NULL handles return `ERROR_INVALID_HANDLE`). |
| `kernel32.dll!VirtualAlloc2` (foreign process) | `crates/win32-rt/src/kernel32/memory.rs` | Native-unsupported | `NULL`, `CURRENT_PROCESS`, and `OpenProcess(self)` work; foreign processes return `ERROR_ACCESS_DENIED` (invalid handles return `ERROR_INVALID_HANDLE`). |
| `kernel32.dll!CreateRemoteThread` (foreign process) | `crates/win32-rt/src/kernel32/threads.rs` | Native-unsupported | `CURRENT_PROCESS` and `OpenProcess(self)` work; foreign processes return `ERROR_ACCESS_DENIED` (invalid/NULL handles return `ERROR_INVALID_HANDLE`). |
| `kernel32.dll!CreateProcessW` (iOS target) | `crates/win32-rt/src/kernel32/process.rs` | Native-unsupported | macOS spawns real child processes via `posix_spawn`; iOS sandbox forbids child process spawning (`ERROR_NOT_SUPPORTED`). |
| `ntdll.dll!NtCreateTransaction` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!NtCommitTransaction` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!KiUserExceptionDispatcher` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!KiUserApcDispatcher` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!KiUserCallbackDispatcher` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |
| `ntdll.dll!LdrInitializeThunk` | `crates/win32-rt/src/ntdll.rs` | Native-unsupported | See table below. |

## Native-unsupported (matches Windows without the feature)

| DLL!API | Returned status / error | Reason on Apple Silicon / macOS / iOS | Windows Documentation Citation |
|---|---|---|---|
| `kernel32.dll!VirtualAlloc` (`MEM_LARGE_PAGES`) | `NULL`, `ERROR_PRIVILEGE_NOT_HELD` (1314) after parameter validation (`ERROR_INVALID_PARAMETER`) | Apple Silicon XNU does not expose 2 MiB user-mode superpages (`GetLargePageMinimum() == 0`), and standard unprivileged tokens do not hold `SeLockMemoryPrivilege`. | Microsoft Learn: *Large-Page Support* & *VirtualAlloc* — processes without `SeLockMemoryPrivilege` fail with `ERROR_PRIVILEGE_NOT_HELD`; invalid flag/alignment combinations fail with `ERROR_INVALID_PARAMETER`. |
| `kernel32.dll!VirtualAllocEx` (foreign process handle) | `NULL`, `ERROR_ACCESS_DENIED` (5) for foreign `ChildProcessObj`; `ERROR_INVALID_HANDLE` (6) for `NULL`/invalid handle | macOS SIP and hardened-runtime sandbox block `task_for_pid` / cross-process `mach_vm_allocate` without `com.apple.security.cs.debugger` entitlement. | Microsoft Learn: *VirtualAllocEx* — requires `PROCESS_VM_OPERATION` on target process; lacking access fails with `ERROR_ACCESS_DENIED`, invalid handle fails with `ERROR_INVALID_HANDLE`. |
| `kernel32.dll!VirtualAlloc2` (foreign process handle) | `NULL`, `ERROR_ACCESS_DENIED` (5) for foreign `ChildProcessObj`; `ERROR_INVALID_HANDLE` (6) for invalid handle (`NULL` targets current process) | Same macOS SIP/sandbox restriction on foreign `task_for_pid`. | Microsoft Learn: *VirtualAlloc2* — `NULL` process handle targets current process; foreign process without `PROCESS_VM_OPERATION` fails with `ERROR_ACCESS_DENIED`. |
| `kernel32.dll!CreateRemoteThread` (foreign process handle) | `NULL`, `ERROR_ACCESS_DENIED` (5) for foreign `ChildProcessObj`; `ERROR_INVALID_HANDLE` (6) for `NULL`/invalid handle | macOS SIP/sandbox blocks cross-process `thread_create_running` without debugger entitlements. | Microsoft Learn: *CreateRemoteThread* — requires `PROCESS_CREATE_THREAD`, `PROCESS_VM_OPERATION`, etc.; denied access fails with `ERROR_ACCESS_DENIED`. |
| `kernel32.dll!CreateProcessW` (`target_os = "ios"` only) | `FALSE`, `ERROR_NOT_SUPPORTED` (50) | iOS App Sandbox and kernel policy prohibit `posix_spawn` / child process execution (macOS uses real `posix_spawn`). | Microsoft Learn: *CreateProcessW* & Apple iOS Sandbox documentation (child processes disallowed on iOS). |
| `ntdll.dll!NtCreateTransaction` | `STATUS_NOT_IMPLEMENTED` (`0xC0000002`), `*out = NULL` | APFS/Darwin has no Kernel Transaction Manager (KTM) or Transactional NTFS (TxF). | Microsoft Learn: *Transactional NTFS (TxF)* (deprecated; unsupported on non-NTFS volumes / systems without KTM). |
| `ntdll.dll!NtCommitTransaction` | `STATUS_INVALID_HANDLE` (`0xC0000008`) for `NULL`; `STATUS_NOT_IMPLEMENTED` (`0xC0000002`) otherwise | Same absence of KTM/TxF on Darwin/APFS. | Microsoft Learn: *NtCommitTransaction* & *Transactional NTFS (TxF)*. |
| `ntdll.dll!KiUserExceptionDispatcher` | Exported symbol; logs warning if called directly | Kernel-to-user trap-frame entry point invoked solely by `ntoskrnl.exe` with a kernel-synthesized `CONTEXT`/`EXCEPTION_RECORD` stack frame; no user-mode call ABI. Exceptions dispatch natively via `signals.rs` -> `unwind::dispatch_exception`. | *Windows Internals* / Microsoft Learn: `ntoskrnl!PspLookupKernelUserEntryPoints` (user-mode code resolves address only for hooks/inspection). |
| `ntdll.dll!KiUserApcDispatcher` | Exported symbol; logs warning if called directly | Kernel-to-user APC trap-frame entry point; user APCs dispatch natively via `thread::run_pending_apcs`. | *Windows Internals* (`PspLookupKernelUserEntryPoints`). |
| `ntdll.dll!KiUserCallbackDispatcher` | Exported symbol; logs warning if called directly | Kernel-to-user win32k callback trap-frame entry point; window callbacks dispatch natively via `user32/`. | *Windows Internals* (`PspLookupKernelUserEntryPoints`). |
| `ntdll.dll!LdrInitializeThunk` | Exported symbol; logs warning if called directly | Kernel-to-user initial thread startup entry point; thread initialization runs natively via `thread::thread_main` -> `BaseThreadInitThunk`. | *Windows Internals* (`PspLookupKernelUserEntryPoints`). |

## Leftovers

None — all 28 stub sites in scope have been resolved.

### From `docs/stub-backlog.d/middleware.md`

# Stub & API Gap Backlog — `stubs/middleware` (Vendor Middleware & Vulkan Extensions)

Scope: 61 static stub sites across vendor middleware DLLs (`amd_ags_x64`, `amd_fidelityfx_loader_dx12`, `libxess`, `libxess_dx11`, `nvngx_dlss`, `sl.interposer`, `oo2core_9_win64`, `physx_64`, `physxcooking_64`, `physx3common_x64`, `zstd`, `directml`, `dstorage`, `nri`) and `vulkan_1.dll` (`nvlowlatencyvk.dll`).

- **Before:** 61 sites in scope
- **After:** 15 sites remaining in scope (46 sites eliminated)
  - **Made real:** 6 sites
  - **Removed fake proprietary overrides (Rule 4 — shipping PE runs via ARM64 JIT; absent DLL returns `ERROR_MOD_NOT_FOUND`):** 7 sites
  - **Native-unsupported (matches Windows without vendor hardware/feature):** 33 sites
  - **Leftovers (honest partial implementations kept in place):** 15 sites

## Removed Sites — Made Real (6)

| DLL!API | Disposition | Implementation |
|---|---|---|
| `zstd.dll!ZSTD_compress` | Made real | Backed by vendored BSD-3-Clause `zstd-sys` (Meta Zstandard 1.6.0) with null-pointer safety and full unit round-trip/error tests in `crates/win32-rt/src/misc/zstd.rs`. |
| `zstd.dll!ZSTD_decompress` | Made real | Backed by vendored BSD-3-Clause `zstd-sys` (Meta Zstandard 1.6.0) with null-pointer safety and corrupt/small-buffer error tests in `crates/win32-rt/src/misc/zstd.rs`. |
| `dstorage.dll!DStorageCreateCompressionCodec` | Made real | Complete DirectStorage 1.2 built-in codec dispatch (`DSTORAGE_COMPRESSION_FORMAT_GDEFLATE` -> real GDeflate v1 codec; `NONE` -> `E_INVALIDARG`; custom `>= 0x80` & reserved `2..=0x7F` -> `E_NOTIMPL` per Microsoft Learn `DStorageCreateCompressionCodec`). |
| `amd_fidelityfx_loader_dx12.dll!ffxDispatch` (`GENERATEREACTIVEMASK`) | Made real | Implemented real Metal GPU compute shader `port_ffx_gen_reactive_2d` in `crates/d3d12-metal` (`clear.rs`, `metalfx_rt.rs`) matching FidelityFX FSR3 autogen reactive mask (`APPLY_TONEMAP`, `APPLY_INVERSETONEMAP`, `APPLY_THRESHOLD`, `USE_COMPONENTS_MAX`, `scale`, `cutoff_threshold`, `binary_value`), verified via GPU readback test. |
| `libxess.dll!xessD3D12Execute` | Made real | Implemented non-zero input/output subrect base offsets (`inputColorBase`, `inputDepthBase`, `inputMotionVectorBase`, `inputResponsiveMaskBase`, `outputColorBase`) in `crates/d3d12-metal` via dedicated-slot Metal staging blits (zero overhead at `[0, 0]`), verified via GPU readback test. |
| `libxess_dx11.dll!xessD3D11Execute` | Made real | Wired non-zero subrect base offsets through `d3d12-metal` `TemporalUpscaleParams` and `upscale_resource_subrect`, verified via D3D11 GPU readback test. |

## Removed Sites — Fake Proprietary Middleware Overrides Removed (7)

Per Rule 4, games shipping these proprietary DLLs execute their real Windows x86-64 PE via the ARM64 JIT (`DefaultDllMode::Guest`), and when no shipping DLL is present `LoadLibrary` fails with `ERROR_MOD_NOT_FOUND` (126), matching Windows.

| DLL!API | Disposition | Reason |
|---|---|---|
| `oo2core_9_win64.dll!OodleLZ_Compress` | Removed fake override | Proprietary RAD Oodle codec; deleted `crates/win32-rt/src/misc/oo2core_9_win64.rs` so shipping `oo2core_*_win64.dll` runs via ARM64 JIT and absent DLL fails with `ERROR_MOD_NOT_FOUND`. |
| `oo2core_9_win64.dll!OodleLZ_Decompress` | Removed fake override | Proprietary RAD Oodle codec; deleted `crates/win32-rt/src/misc/oo2core_9_win64.rs`. |
| `physx_64.dll!PxCreateBasePhysics` | Removed fake override | NVIDIA PhysX 4.x/5.x core simulation; deleted `crates/win32-rt/src/misc/physx_64.rs` so shipping `PhysX_64.dll` runs via ARM64 JIT (`[middleware.physx] mode = "guest"`). |
| `physx_64.dll!PxCreatePhysics` | Removed fake override | NVIDIA PhysX 4.x/5.x core simulation; deleted `crates/win32-rt/src/misc/physx_64.rs`. |
| `physx_64.dll!PxRegisterPhysicsSerializers` | Removed fake override | NVIDIA PhysX 4.x/5.x serializers; deleted `crates/win32-rt/src/misc/physx_64.rs`. |
| `physx_64.dll!PxUnregisterPhysicsSerializers` | Removed fake override | NVIDIA PhysX 4.x/5.x serializers; deleted `crates/win32-rt/src/misc/physx_64.rs`. |
| `physxcooking_64.dll!PxCreateCooking` | Removed fake override | NVIDIA PhysX 4.x/5.x cooking; deleted `crates/win32-rt/src/misc/physxcooking_64.rs` so shipping `PhysXCooking_64.dll` runs via ARM64 JIT. |

## Native-unsupported (matches Windows without the feature) (33)

| DLL!API | Behavior on Apple Silicon | Windows / Vendor SDK Citation |
|---|---|---|
| `amd_ags_x64.dll!agsDriverExtensionsDX12_PushMarker` | Forwards recognized D3D12 command lists to Metal debug markers (`AGS_SUCCESS`); returns `AGS_EXTENSION_NOT_SUPPORTED` (`7`) on foreign/non-AMD command lists. | AMD GPUOpen AGS SDK `amd_ags.h`: on non-AMD GPUs `extensionsSupported == 0` and DX12 driver-extension calls return `AGS_EXTENSION_NOT_SUPPORTED` (`7`). |
| `amd_ags_x64.dll!agsDriverExtensionsDX12_PopMarker` | Forwards recognized D3D12 command lists to Metal debug markers (`AGS_SUCCESS`); returns `AGS_EXTENSION_NOT_SUPPORTED` (`7`) on foreign/non-AMD command lists. | AMD GPUOpen AGS SDK `amd_ags.h`: `AGS_EXTENSION_NOT_SUPPORTED` (`7`) without AMD driver extension context. |
| `amd_ags_x64.dll!agsDriverExtensionsDX12_SetMarker` | Forwards recognized D3D12 command lists to Metal debug markers (`AGS_SUCCESS`); returns `AGS_EXTENSION_NOT_SUPPORTED` (`7`) on foreign/non-AMD command lists. | AMD GPUOpen AGS SDK `amd_ags.h`: `AGS_EXTENSION_NOT_SUPPORTED` (`7`) without AMD driver extension context. |
| `amd_fidelityfx_loader_dx12.dll!ffxCreateContext` (`FRAMEGENERATION`) | Zeroes `*context = 0` and returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp` / `ffx_api_types.h`: returns `FFX_API_RETURN_NO_PROVIDER` (`4`) when no frame-generation provider is installed (upscaler contexts remain routed to MetalFX; future Metal 4 `MTL4FXFrameInterpolator` optional). |
| `amd_fidelityfx_loader_dx12.dll!ffxConfigure` (`FRAMEGENERATION`) | Returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation provider. |
| `amd_fidelityfx_loader_dx12.dll!ffxDispatch` (`FRAMEGENERATION`) | Returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation provider. |
| `amd_fidelityfx_loader_dx12.dll!ffxQuery` (`INTERPOLATIONCOMMANDLIST_DX12`) | Zeroes `*out_cmd = 0` and returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation swapchain provider. |
| `amd_fidelityfx_loader_dx12.dll!ffxQuery` (`INTERPOLATIONTEXTURE_DX12`) | Zeroes `*out_tex = [0; 6]` and returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation swapchain provider. |
| `libxess_dx11.dll!xessSetContextParameterF` | Validates context and finite value, returns `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). | Intel XeSS SDK `xess.h`: retail non-Intel XeSS runtimes expose no Intel-driver-internal float context parameters and return `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). |
| `libxess_dx11.dll!xessStartDump` | Validates context and non-null params/path, returns `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). | Intel XeSS SDK `xess_debug.h`: retail `libxess_dx11.dll` builds omit Intel debug frame-dump capture and return `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). |
| `libxess_dx11.dll!INTC_D3D12_GetApplicationInfo` | Returns `E_NOINTERFACE` (`0x80004002`). | Intel Graphics Extensions (`igdext64.dll` / `IntelExtensionsFramework`): no Intel `INTCExtensionContext` exists on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GetGpuCrashDumpBufferInfo` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GpuCrashDumpOnCommandListCreate` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GpuCrashDumpOnCommandListDestroy` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GpuCrashDumpOnDeviceDestroy` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_RegisterDriverCallbacks` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD callback hook has no Intel GPU driver on non-Intel GPUs. |
| `nvngx_dlss.dll!NVSDK_NGX_D3D12_CreateFeature` (non-SR/ISP features) | Zeroes `*out_handle = 0` and returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). (`SuperSampling` and `ImageSignalProcessing` run on MetalFX.) | NVIDIA NGX SDK `nvsdk_ngx_defs.h`: GPUs lacking Tensor Core / Optical Flow features (`FrameGeneration`, `DeepDVC`, `RayReconstruction`) return `NVSDK_NGX_Result_FAIL_FeatureNotSupported`. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_Init` | Returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h` / `nvsdk_ngx_defs.h`: requires NVIDIA Vulkan driver (`VK_NVX_binary_import`); non-NVIDIA GPUs return `NVSDK_NGX_Result_FAIL_FeatureNotSupported`. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_CreateFeature` | Zeroes `*out_handle = 0` and returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h`: returns `NVSDK_NGX_Result_FAIL_FeatureNotSupported` on non-NVIDIA GPUs. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_EvaluateFeature` | Returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h`: returns `NVSDK_NGX_Result_FAIL_FeatureNotSupported` on non-NVIDIA GPUs. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_ReleaseFeature` | Returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h`: returns `NVSDK_NGX_Result_FAIL_FeatureNotSupported` on non-NVIDIA GPUs. |
| `sl.interposer.dll!slSetTag` | Returns `eErrorInvalidParameter` (`25`) if `tags == NULL \|\| n == 0`, else `eErrorFeatureNotSupported` (`32`). | NVIDIA Streamline SDK `source/core/sl.api/sl.cpp` (`slSetTag` -> `slValidateFeatureContext(kFeatureCommon, ctx)`): returns `eErrorFeatureNotSupported` when no Streamline plugins are supported on non-NVIDIA GPUs. |
| `sl.interposer.dll!slSetConstants` | Returns `eErrorInvalidParameter` (`25`) if `values == NULL \|\| frame == NULL`, else `eErrorFeatureNotSupported` (`32`). | NVIDIA Streamline SDK `source/core/sl.api/sl.cpp` (`slSetConstants` -> `slValidateFeatureContext(kFeatureCommon, ctx)`): returns `eErrorFeatureNotSupported` when no Streamline plugins are supported on non-NVIDIA GPUs. |
| `vulkan_1.dll!NvLL_VK_Initialize` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h` (`nvlowlatencyvk.dll`): returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) on non-NVIDIA GPUs without `VK_NV_low_latency`. |
| `vulkan_1.dll!NvLL_VK_Unload` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_InitLowLatencyDevice` | Zeroes `*signal_sem = 0` when non-null and returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_DestroyLowLatencyDevice` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_SetSleepMode` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_Sleep` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_GetSleepStatus` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_SetLatencyMarker` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_GetLatency` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_NotifyOutOfBandQueue` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |

## Remaining Leftovers in Scope (15)

| DLL!API | Kind | Location | What Is Missing | Concrete Next Step |
|---|---|---|---|---|
| `physx3common_x64.dll!geom_get_world_bounds` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry (`ConvexMesh`, `TriangleMesh`, `HeightField`) bounds not parsed (analytic Sphere/Plane/Capsule/Box implemented). | Prefer shipping `PhysX3Common_x64.dll` via ARM64 JIT (`DefaultDllMode::Guest`), or parse PhysX 3.4 cooked `Gu::ConvexMeshData` / `Gu::TriangleMeshData` / `Gu::HeightFieldData` AABBs. |
| `physx3common_x64.dll!geom_point_distance` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry point-distance query not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement GJK/BVH closest-point over cooked PhysX 3.4 mesh payloads. |
| `physx3common_x64.dll!geom_compute_penetration` (2 sites: geom0, geom1) | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry penetration depth/direction (MTD) not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement EPA/SAT penetration over cooked convex/triangle/heightfield meshes. |
| `physx3common_x64.dll!geom_overlap` (2 sites: geom0, geom1) | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry overlap test not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement BVH/SAT overlap against cooked meshes. |
| `physx3common_x64.dll!geom_raycast` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry raycast not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement BVH33/BVH34 ray-triangle and ray-convex-hull intersection. |
| `physx3common_x64.dll!geom_sweep` (2 sites: geom0, geom1) | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry shape sweep not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement GJK-raycast/TOI sweep over cooked meshes. |
| `physx3common_x64.dll!mesh_find_overlap_triangle_mesh` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | `PxMeshQuery::findOverlapTriangleMesh` requires cooked triangle mesh BVH/index payload decoding. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or decode `Gu::TriangleMesh` vertex/index/BVH buffers. |
| `physx3common_x64.dll!mesh_get_triangle` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | `PxMeshQuery::getTriangle` requires cooked triangle mesh vertex/index/adjacency decoding. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or decode `Gu::TriangleMesh` vertex/index/adjacency tables. |
| `physx3common_x64.dll!mesh_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | `PxMeshQuery::sweep` over triangle array not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement analytic shape-vs-triangle sweep loop in `physx3common_x64.rs`. |
| `directml.dll!parse` | `middleware_stub` | `crates/win32-rt/src/misc/directml/ops.rs` | Only ~35 core DirectML operators are implemented; remaining DML 1.x / FL 5.0 operators (`Gather`, `Scatter`, `Resample`, `Resize`, `TopK`, `Pad`, `Tile`, `Cast`, `RNN`/`LSTM`/`GRU`, `QuantizeLinear`/`DequantizeLinear`, `MultiheadAttention`, etc.) return `E_NOTIMPL`. | Implement missing `DML_OPERATOR_*` descriptors and CPU/Metal compute kernels as games request them, with per-operator unit tests. |
| `nri.dll!core_cmd_set_sample_positions` | `middleware_stub` | `crates/win32-rt/src/misc/nri/cmdlist.rs` | `ID3D12GraphicsCommandList1::SetSamplePositions` is not yet wired to Metal's `MTLRenderPassDescriptor::setSamplePositions:count:` (`D3D12_OPTIONS2.ProgrammableSamplePositionsTier` currently reports `0`). | Wire `SetSamplePositions` in `d3d12-metal` (`MTL4RenderPassDescriptor` / `MTLRenderPassDescriptor` programmable sample positions on Apple GPU family 7+) and `win32-rt/src/d3d12`, then forward `CmdSetSamplePositions` to it. |
| `nri.dll!core_create_query_pool` | `middleware_stub` | `crates/win32-rt/src/misc/nri/device.rs` | `QueryType::PIPELINE_STATISTICS` (`2`) and `QueryType::ACCELERATION_STRUCTURE_COMPACTED_SIZE` (`3`) return `NRI_RESULT_UNSUPPORTED` (`TIMESTAMP` and `OCCLUSION` are implemented). | Back `ACCELERATION_STRUCTURE_COMPACTED_SIZE` with a readback buffer + `EmitRaytracingAccelerationStructurePostbuildInfo` (`COMPACTED_SIZE`), and wire `PIPELINE_STATISTICS` once `d3d12-metal` exposes `MTL4CounterHeap` pipeline-statistics queries. |

### From `docs/stub-backlog.d/window-input-ui.md`

# Stub Backlog — Window, Input, UI & Text DLLs (`stubs/window-input-ui`)

Scope: 34 `stub!` sites in `db/stub_inventory.tsv` across `user32`, `dinput8`, `xinput1_4`, `gameinput`, `hid`, `uxtheme`, `msctf`, `dwrite`, `d2d1`, `gdi32`, `shell32`, and `powrprof`, plus 2 `user32` guest-corpus gaps (`wsprintfA` varargs and `wsprintfW` export, plus `NONCLIENTMETRICS` layout verification).

## Summary

| Category | Count |
|---|---|
| `stub!` sites in scope (before) | 34 |
| `stub!` sites in scope (after) | 1 |
| Removed — made real (incl. 2 `wsprintf` guest-corpus gaps) | 30 (28 `stub!` sites + `wsprintfA`/`wsprintfW`) |
| Removed — native-unsupported (matches Windows without feature) | 5 |
| Remaining honest leftovers | 1 |

---

## Removed Sites (Made Real)

| DLL!API | File | What was implemented |
|---|---|---|
| `user32.dll!LoadCursorA/W` | `crates/win32-rt/src/user32/cursor.rs`, `macos/appkit.rs` | Parses PE `RT_GROUP_CURSOR` (12) + `RT_CURSOR` (1) resources with 4-byte `(xHotSpot, yHotSpot)` prefix and DIB/PNG decoding; renders custom `Shape::Custom` cursors natively via CoreGraphics + `NSImage` + `NSCursor`. |
| `user32.dll!CreateCursor` | `crates/win32-rt/src/user32/cursor.rs`, `macos/appkit.rs` | Decodes 1bpp AND/XOR monochrome cursor masks with hotspot into RGBA/BGRA `IconData` and custom `NSCursor`. |
| `user32.dll!DefWindowProcA/W` (`WM_SYSCOMMAND`) | `crates/win32-rt/src/user32/defproc.rs` | Handles all documented `SC_*` codes (`SC_HOTKEY`, `SC_NEXTWINDOW`/`SC_PREVWINDOW`, standard WindowServer-owned no-ops) and ignores unknown codes as Windows does. |
| `user32.dll!RegisterRawInputDevices` (`RIDEV_NOLEGACY`) | `crates/win32-rt/src/user32/input.rs` | Enforces `RIDEV_NOLEGACY` for mouse `(1,2)` and keyboard `(1,6)` (suppressing legacy `WM_MOUSE*`/`WM_KEY*` while continuing to deliver `WM_INPUT` and update async key state); rejects `RIDEV_NOLEGACY` on non-legacy usages with `ERROR_INVALID_PARAMETER`. |
| `user32.dll!ReplyMessage` | `crates/win32-rt/src/user32/message.rs` | Unblocks waiting cross-thread `SendMessage` callers early via thread-local reply stack and RAII `Drop` guard. |
| `user32.dll!GetSystemMetrics` / `GetSystemMetricsForDpi` | `crates/win32-rt/src/user32/misc.rs` | Complete `SM_*` table with DPI scaling; returns `0` for undefined indices matching Windows. |
| `user32.dll!SystemParametersInfoA/W/ForDpi` | `crates/win32-rt/src/user32/misc.rs` | Persistent `SpiState` across all standard SPI getters/setters, DPI-scaled `NONCLIENTMETRICS` with compile-time layout assertions (`344B`/`504B`), and `FALSE` + `ERROR_INVALID_PARAMETER` for unknown actions. |
| `user32.dll!GetUserObjectInformationA/W` (handles & indices) | `crates/win32-rt/src/user32/misc.rs` | Added desktop handle APIs (`GetThreadDesktop`, `OpenInputDesktop`, `OpenDesktopA/W`, `CloseDesktop`, `CloseWindowStation`) and all 6 `UOI_*` queries (`UOI_FLAGS`, `UOI_NAME`, `UOI_TYPE`, `UOI_USER_SID`, `UOI_HEAPSIZE`, `UOI_IO`). |
| `user32.dll!RegisterHotKey` / `UnregisterHotKey` | `crates/win32-rt/src/user32/misc.rs`, `input.rs`, `window.rs` | Per-thread/per-HWND global hotkey table with duplicate detection (`ERROR_HOTKEY_ALREADY_REGISTERED`), `WM_HOTKEY` dispatch on keydown, and automatic cleanup on window destruction. |
| `user32.dll!wsprintfA` / `wsprintfW` | `crates/win32-rt/src/user32/misc.rs`, `ucrt/printf.rs` | Column-0 `win64_varargs_thunk!` varargs formatting backed by `ucrt::printf` with Windows 1024-char cap (1023 + NUL) and `LEGACY_WIDE_SPECIFIERS` for `wsprintfW`; un-ignored `guest_user32_wsprintf_backlog`. |
| `user32.dll!DisplayConfigGetDeviceInfo` | `crates/win32-rt/src/user32/monitor.rs` | Implemented remaining CCD GET packets (`GET_TARGET_BASE_TYPE`, `GET_SUPPORT_VIRTUAL_RESOLUTION`, `GET_MONITOR_SPECIALIZATION`, `GET_ADVANCED_COLOR_INFO_2`, `GET_DPI_SCALE`); unknown types return `ERROR_INVALID_PARAMETER`. |
| `dinput8.dll!IDirectInputDevice8::GetProperty` / `SetProperty` | `crates/win32-rt/src/misc/dinput8.rs` | Implemented `DIPROP_FFGAIN`, `DIPROP_FFLOAD`, `DIPROP_AUTOCENTER`, `DIPROP_CALIBRATIONMODE`, `DIPROP_CALIBRATION`, `DIPROP_GUIDANDPATH` (Xbox `IG_` path), `DIPROP_GETPORTDISPLAYNAME`, `DIPROP_KEYNAME`, `DIPROP_APPDATA` (wired to `DIDEVICEOBJECTDATA.uAppData`), `DIPROP_TYPENAME`, `DIPROP_USERNAME`. |
| `xinput1_4.dll!XInputGetKeystroke` | `crates/win32-rt/src/misc/xinput1_4.rs` | Real per-slot and `XUSER_INDEX_ANY` (`0xFF`) `XINPUT_KEYSTROKE` ring queue populated on button, trigger, and thumbstick edge transitions. |
| `gameinput.dll!IGameInput::GetTemporalReading` | `crates/win32-rt/src/misc/gameinput.rs` | Searches device reading history for the newest snapshot with `timestamp_us <= timestamp`, returning `GAMEINPUT_E_READING_NOT_FOUND` when none precedes it. |
| `hid.dll!HidD_SetNumInputBuffers` | `crates/win32-rt/src/misc/hid.rs` | Per-handle input report ring-buffer capacity (`2..=512`), enforced on report delivery, trimmed on shrink, and returned by `HidD_GetNumInputBuffers`. |
| `uxtheme.dll!DrawThemeBackground` | `crates/win32-rt/src/misc/uxtheme.rs` | Matches Windows Classic / unthemed desktop (`IsThemeActive() == FALSE`, zero open `HTHEME` handles): `ValidateHandle` rejects any unopened handle with `E_HANDLE`. |
| `uxtheme.dll!TaskDialogIndirect` | `crates/win32-rt/src/misc/uxtheme.rs` | Parses 1-byte-packed x64 `TASKDIALOGCONFIG` (160B) and `TASKDIALOG_BUTTON` (12B), resolves `IS_INTRESOURCE` strings via `LoadStringW`, supports custom/common buttons, radio defaults, verification flag, and callback notifications, and displays via native `NSAlert`/`UIAlertController` (with honest headless handling). |
| `msctf.dll!ITfContext::TrackProperties` | `crates/win32-rt/src/misc/msctf/{context,range,abi}.rs` | Returns a composite `ITfReadOnlyProperty` whose `GetValue` yields `VT_UNKNOWN` -> `IEnumTfPropertyValue` enumerating `TF_PROPERTYVAL` entries for all tracked properties. |
| `dwrite.dll!IDWriteFactory::CreateCustomFontFileReference` | `crates/win32-rt/src/misc/dwrite{,.rs,/font.rs,/sfnt.rs}` | Invokes `IDWriteFontFileLoader::CreateStreamFromKey` + `IDWriteFontFileStream::{GetFileSize, ReadFileFragment, ReleaseFileFragment}` to build a memory-backed SFNT `IDWriteFontFile` usable with `Analyze` and `CreateFontFace`. |
| `dwrite.dll!IDWriteFactory::CreateCustomFontCollection` | `crates/win32-rt/src/misc/dwrite{,.rs,/font.rs}` | Invokes `IDWriteFontCollectionLoader::CreateEnumeratorFromKey` + `IDWriteFontFileEnumerator::{MoveNext, GetCurrentFontFile}` to assemble a real `IDWriteFontCollection`. |
| `dwrite.dll!IDWriteFactory::CreateTextAnalyzer` | `crates/win32-rt/src/misc/dwrite/analysis.rs` | Full 10-slot `IDWriteTextAnalyzer` (`AnalyzeScript`, `AnalyzeBidi`, `AnalyzeNumberSubstitution`, `AnalyzeLineBreakpoints`, `GetGlyphs`, `GetGlyphPlacements`, `GetGdiCompatibleGlyphPlacements`). |
| `dwrite.dll!IDWriteFactory::CreateGlyphRunAnalysis` | `crates/win32-rt/src/misc/dwrite/analysis.rs` | Full 6-slot `IDWriteGlyphRunAnalysis` (`GetAlphaTextureBounds`, `CreateAlphaTexture` for `ALIASED_1x1` and `CLEARTYPE_3x1`, `GetAlphaBlendParams`). |
| `d2d1.dll!ID2D1Geometry::Tessellate` | `crates/win32-rt/src/misc/d2d1/{geometry,abi}.rs` | Flattens geometry with transform/tolerance, triangulates convex and concave filled subpaths via ear-clipping, and emits `D2D1_TRIANGLE` arrays to `ID2D1TessellationSink::AddTriangles`. |
| `gdi32.dll!GetDeviceCaps` | `crates/win32-rt/src/misc/gdi32.rs` | Covers all standard `wingdi.h` capability indices, honoring selected bitmap dimensions/depth on memory DCs and returning `0` for unknown indices. |
| `powrprof.dll!CallNtPowerInformation` | `crates/win32-rt/src/misc/powrprof.rs` | Implements standard read-only `POWER_INFORMATION_LEVEL`s (including `LastWakeTime`/`LastSleepTime` via macOS `sysctlbyname`), returns `STATUS_ACCESS_DENIED` for privileged admin levels and `STATUS_INVALID_PARAMETER` for unknown levels. |
| `powrprof.dll!PowerRegisterSuspendResumeNotification` | `crates/win32-rt/src/misc/powrprof.rs` | Arms a dedicated background macOS IOKit `IORegisterForSystemPower` `CFRunLoop` delivering `PBT_APMSUSPEND`/`PBT_APMRESUME*` to `DEVICE_NOTIFY_CALLBACK` and `WM_POWERBROADCAST` recipients. |
| `powrprof.dll!PowerSettingRegisterNotification` | `crates/win32-rt/src/misc/powrprof.rs` | Arms macOS IOKit `IOPSNotificationCreateRunLoopSource`, delivers the immediate initial `PBT_POWERSETTINGCHANGE` value upon registration, and dispatches `POWERBROADCAST_SETTING` packets on power-source/battery changes. |

---

## Native-unsupported (matches Windows without the feature)

| DLL!API | File | Behavior Returned | Windows Documentation Citation |
|---|---|---|---|
| `user32.dll!MessageBoxA/W` (headless multi-button branch) | `crates/win32-rt/src/user32/misc.rs` | Returns `0` with `SetLastError(ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION)` (`1459`) when no interactive desktop session is active (interactive macOS/iOS displays a real `NSAlert`/`UIAlertController`). | Microsoft Learn — [MessageBox function](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox) & Windows Services / non-interactive window station behavior (`ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION`, `1459`). |
| `user32.dll!DisplayConfigSetDeviceInfo` | `crates/win32-rt/src/user32/monitor.rs` | Validates packet pointer/size/type (`ERROR_INVALID_PARAMETER`) and returns `ERROR_NOT_SUPPORTED` (`50`) for valid WDDM/CCD `SET_*` packets (`5, 8, 10, 13, 16`), because Apple Silicon macOS has no Windows WDDM/CCD display driver. | Microsoft Learn — [DisplayConfigSetDeviceInfo function](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-displayconfigsetdeviceinfo) (`ERROR_NOT_SUPPORTED` when the display driver/system does not support the requested configuration change). |
| `dinput8.dll!IDirectInputDevice8::RunControlPanel` | `crates/win32-rt/src/misc/dinput8.rs` | Validates `dwFlags == 0` (`DIERR_INVALIDPARAM`) and returns `DIERR_UNSUPPORTED` (`E_NOTIMPL`), because Windows `joy.cpl` does not exist on macOS/iOS. | Microsoft Learn — [IDirectInputDevice8::RunControlPanel](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee417909(v=vs.85)) (optional control-panel UI; returns `DIERR_UNSUPPORTED` when unsupported). |
| `dinput8.dll!IDirectInput8::RunControlPanel` | `crates/win32-rt/src/misc/dinput8.rs` | Validates `dwFlags == 0` (`DIERR_INVALIDPARAM`) and returns `DIERR_UNSUPPORTED` (`E_NOTIMPL`), because Windows `joy.cpl` does not exist on macOS/iOS. | Microsoft Learn — [IDirectInput8::RunControlPanel](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee417816(v=vs.85)) (returns `DIERR_UNSUPPORTED` when control panel launch is unsupported). |
| `dinput8.dll!IDirectInput8::ConfigureDevices` | `crates/win32-rt/src/misc/dinput8.rs` | Returns `DIERR_UNSUPPORTED` (`E_NOTIMPL`), matching Windows Vista and all later Windows releases where the DirectInput action-mapper UI was removed. | Microsoft Learn — [IDirectInput8::ConfigureDevices](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee417799(v=vs.85)) (deprecated and unsupported starting with Windows Vista). |

---

## Remaining Honest Leftovers

| DLL!API | File | What works now | What is still missing & next step |
|---|---|---|---|
| `shell32.dll!SHCreateItemFromParsingName` | `crates/win32-rt/src/misc/shell32.rs` | Local VFS paths (including `\\?\` extended paths), `file:///` URLs (percent-decoded), `shell:<KnownFolderName>`, and filesystem-backed `::{<FOLDERID>}` / `shell:::{<FOLDERID>}` parsing names now resolve to real `IShellItem` COM objects. | Non-filesystem virtual Explorer shell namespaces (e.g. `shell:::{20D04FE0-...}` This PC, `::{645FF040-...}` Recycle Bin), `ms-settings:` URIs, `http://` URLs, and SMB/UNC network shares (`\\server\share\...`) still hit `stub!` and return `E_NOTIMPL`. Next step: implement a virtual `IShellFolder`/`IShellItem` namespace tree for `CLSID_MyComputer` / `CLSID_RecycleBin` (backed by mounted drives / `~/.Trash`) and an SMB/UNC resolver if a title requests them. |

## Proprietary & Vendor Middleware (Legal / Architectural Limits)

These modules wrap proprietary third-party SDKs or GPU-vendor-specific extensions. Where a game ships its own Windows x86-64 PE DLL (e.g., Oodle `oo2core_*_win64.dll`, PhysX 4/5 `PhysX_64.dll`, `zstd.dll`), `port-runner` executes the real shipping PE through the ARM64 JIT rather than overriding it.

| DLL | Remaining Sites | Reason | Next Step |
|---|---:|---|---|

## Remaining Stubs by DLL

| DLL | Count | Reason | Next Step |
|---|---:|---|---|
| `kernel32.dll` | 15 | Partial Windows kernel/file/memory/locale/job/threadpool sub-feature | Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives. |
| `physx3common_x64.dll` | 12 | PhysX 3.4 geometry/mesh query path without cooked mesh payload | Prefer shipping PhysX3*.dll under ARM64 JIT, or decode cooked BVH/triangle mesh payloads in physx3common_x64. |
| `ucrtbase.dll` | 6 | Uncommon CRT locale, multibyte codepage, UTF-16 stream encoding, or interactive console/shell call | Add UTF-16 FILE stream translation and locale/codepage tables; keep system() safely sandboxed/refused. |
| `d3d11.dll` | 4 | Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop) | Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them. |
| `nri.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `crypt32.dll` | 1 | Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property | Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring. |
| `directml.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `shell32.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |

### `kernel32.dll` (15)

- **Reason:** Partial Windows kernel/file/memory/locale/job/threadpool sub-feature
- **Next step:** Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives.

| API | Kind | Location | Detail |
|---|---|---|---|
| `AllocConsole` | `stub` | `crates/win32-rt/src/kernel32/file.rs:477` | `"no separate console window; stdout/stderr of the loader act as the console"` |
| `DeviceIoControl` | `stub` | `crates/win32-rt/src/kernel32/file.rs:7291` | `"IOCTL_STORAGE_QUERY_PROPERTY property {prop} unsupported"` |
| `DeviceIoControl` | `stub` | `crates/win32-rt/src/kernel32/file.rs:7319` | `"DeviceIoControl code {code:#x} unsupported"` |
| `GetCalendarInfoEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1354` | `"GetCalendarInfoEx type {ty:#x}"` |
| `GetFileInformationByHandleEx` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2287` | `"GetFileInformationByHandleEx class {class} not supported"` |
| `GetGeoInfoW` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1125` | `"GetGeoInfo for geo id {geo} answered with US data"` |
| `GetLocaleInfoW` | `stub` | `crates/win32-rt/src/kernel32/string.rs:918` | `"locale {lcid:#x} not available; answering with en-US data"` |
| `GetNumberFormatEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1323` | `"custom NUMBERFMT ignored"` |
| `LCMapStringEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:552` | `"LCMAP_HASH unsupported"` |
| `LCMapStringEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:578` | `"LCMapString flags {flags:#x} partially honoured"` |
| `MapViewOfFile3` | `stub` | `crates/win32-rt/src/kernel32/file.rs:7082` | `"MapViewOfFile3 on another process"` |
| `MoveFileExA` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2746` | `"MoveFileExA delete-on-reboot ignored"` |
| `MoveFileExW` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2739` | `"MoveFileExW delete-on-reboot ignored"` |
| `SetFileInformationByHandle` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2418` | `"SetFileInformationByHandle class {class} not supported"` |
| `get_locale_info` | `stub` | `crates/win32-rt/src/kernel32/string.rs:865` | `"GetLocaleInfo LCTYPE {lctype:#x} unknown"` |

### `physx3common_x64.dll` (12)

- **Reason:** PhysX 3.4 geometry/mesh query path without cooked mesh payload
- **Next step:** Prefer shipping PhysX3*.dll under ARM64 JIT, or decode cooked BVH/triangle mesh payloads in physx3common_x64.

| API | Kind | Location | Detail |
|---|---|---|---|
| `geom_compute_penetration` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2336` | `"PxGeometryQuery::computePenetration unsupported geometry0 {}", geom_type_name(t)` |
| `geom_compute_penetration` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2346` | `"PxGeometryQuery::computePenetration unsupported geometry1 {}", geom_type_name(t)` |
| `geom_get_world_bounds` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2243` | `"PxGeometryQuery::getWorldBounds unsupported cooked geometry {}", geom_type_name(gtype)` |
| `geom_overlap` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2378` | `"PxGeometryQuery::overlap unsupported geometry0 {}", geom_type_name(t)` |
| `geom_overlap` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2388` | `"PxGeometryQuery::overlap unsupported geometry1 {}", geom_type_name(t)` |
| `geom_point_distance` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2316` | `"PxGeometryQuery::pointDistance unsupported cooked geometry {}", geom_type_name(gtype)` |
| `geom_raycast` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2426` | `"PxGeometryQuery::raycast unsupported cooked geometry {}", geom_type_name(t)` |
| `geom_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2566` | `"PxGeometryQuery::sweep unsupported geometry0 {}", geom_type_name(t)` |
| `geom_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2576` | `"PxGeometryQuery::sweep unsupported geometry1 {}", geom_type_name(t)` |
| `mesh_find_overlap_triangle_mesh` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2614` | `"PxMeshQuery::findOverlapTriangleMesh without cooked triangle mesh payload"` |
| `mesh_get_triangle` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2629` | `"PxMeshQuery::getTriangle without cooked triangle mesh payload"` |
| `mesh_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2642` | `"PxMeshQuery::sweep without cooked triangle mesh payload"` |

### `ucrtbase.dll` (6)

- **Reason:** Uncommon CRT locale, multibyte codepage, UTF-16 stream encoding, or interactive console/shell call
- **Next step:** Add UTF-16 FILE stream translation and locale/codepage tables; keep system() safely sandboxed/refused.

| API | Kind | Location | Detail |
|---|---|---|---|
| `_getch` | `stub` | `crates/win32-rt/src/ucrt/runtime.rs:953` | `"_getch returns EOF"` |
| `_setmbcp` | `stub` | `crates/win32-rt/src/ucrt/locale.rs:274` | `"_setmbcp({cp}) unsupported"` |
| `open_host_path` | `stub` | `crates/win32-rt/src/ucrt/stdio.rs:210` | `"Unicode text modes (0x{oflag:x}) treated as plain text mode"` |
| `parse_mode` | `stub` | `crates/win32-rt/src/ucrt/stdio.rs:1097` | `"fopen ccs={c}: UTF-16 stream encoding not implemented; byte stream instead"` |
| `setlocale` | `stub` | `crates/win32-rt/src/ucrt/locale.rs:64` | `"setlocale({cat}, {n:?}): locale not available"` |
| `system` | `stub` | `crates/win32-rt/src/ucrt/runtime.rs:965` | `"system({:?}) refused", from_ansi(cmd)` |

### `d3d11.dll` (4)

- **Reason:** Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop)
- **Next step:** Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `create_class_linkage` | `stub` | `crates/win32-rt/src/d3d11/device.rs:794` | `"ID3D11Device::CreateClassLinkage -> E_NOTIMPL"` |
| `create_geometry_shader_so` | `stub` | `crates/win32-rt/src/d3d11/device.rs:748` | `"ID3D11Device::CreateGeometryShaderWithStreamOutput -> E_NOTIMPL"` |
| `draw_auto` | `stub` | `crates/win32-rt/src/d3d11/context.rs:2913` | `"ID3D11DeviceContext::DrawAuto not implemented"` |
| `get_data` | `stub` | `crates/win32-rt/src/d3d11/context.rs:3094` | `"GetData(PIPELINE_STATISTICS): GPU-side counters (PS/Clipper/GSPrimitives/DS/indirect) unavailable on Apple Silicon"` |

### `nri.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `core_cmd_set_sample_positions` | `middleware_stub` | `crates/win32-rt/src/misc/nri/cmdlist.rs:965` | `"CmdSetSamplePositions: programmable sample positions not supported on Metal"` |
| `core_create_query_pool` | `middleware_stub` | `crates/win32-rt/src/misc/nri/device.rs:1383` | `"CreateQueryPool: queryType {} is not supported on Metal", qd.query_type` |

### `crypt32.dll` (1)

- **Reason:** Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property
- **Next step:** Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CryptImportPublicKeyInfo` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:55` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |

### `directml.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `parse` | `middleware_stub` | `crates/win32-rt/src/misc/directml/ops.rs:1048` | `"DirectML CreateOperator: unsupported operator type {other}"` |

### `shell32.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `SHCreateItemFromParsingName` | `stub` | `crates/win32-rt/src/misc/shell32.rs:1863` | `"SHCreateItemFromParsingName: unsupported shell namespace {raw:?}"` |

