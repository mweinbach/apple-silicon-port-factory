# Stub Backlog & Zero-Stub Tracker

This index is generated from the checked-in static inventory [`db/stub_inventory.tsv`](../db/stub_inventory.tsv)
and aggregates every per-thread backlog file placed in [`docs/stub-backlog.d/`](stub-backlog.d/).

> **Zero-stub policy:** Do not add new `stub!()`, `failing!()`, or `win64_stubs!()` sites anywhere in `crates/`.
> Both `cargo opsdb -- doctor` and `.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh` fail when a new site appears.
> Regenerate this file and `db/stub_inventory.tsv` after eliminating stubs with `cargo opsdb -- stub-sync`.

## Summary

| Metric | Count |
|---|---:|
| Total remaining stub sites | **195** |
| `stub` (partial Win32/OS/D3D/UCRT branches) | 145 |
| `failing` (honest-failing macro exports) | 31 |
| `middleware_stub` (third-party/engine middleware fallbacks) | 19 |
| DLLs with remaining stubs | 32 |

## Per-Thread Backlog Notes (`docs/stub-backlog.d/`)

| File | Summary |
|---|---|
| [`guest-corpus-bench.md`](stub-backlog.d/guest-corpus-bench.md) | Stub & API Gap Backlog — `toolkit/guest-corpus-bench` |
| [`kernel-files-nls.md`](stub-backlog.d/kernel-files-nls.md) | Stub Backlog — `stubs/kernel-files-nls` |
| [`middleware.md`](stub-backlog.d/middleware.md) | Stub & API Gap Backlog — `stubs/middleware` (Vendor Middleware & Vulkan Extensions) |

### From `docs/stub-backlog.d/guest-corpus-bench.md`

# Stub & API Gap Backlog — `toolkit/guest-corpus-bench`

Exposed while building and running the headless Windows x86-64 guest corpus (`crates/port-loader/tests/os_guest_corpus.rs` and `crates/port-loader/tests/d3d12_e2e.rs`).

| DLL!API | What the test showed | Next step |
|---|---|---|
| `user32.dll!wsprintfW` | Missing export in `crates/win32-rt/src/user32/misc.rs`: guest binaries importing `wsprintfW` hit an unresolved-import trap thunk instead of formatting UTF-16 output. | Implement variadic `wsprintfW` via `win64_varargs_thunk!` backed by `ucrt::stdio` wide formatter (1024-char Windows cap) and run `cargo opsdb -- win32-sync`. |
| `user32.dll!wsprintfA` | Existing implementation in `crates/win32-rt/src/user32/misc.rs` copies the raw format string and ignores variadic arguments (`let _ = (a1, a2);`). | Replace with a real `win64_varargs_thunk!` formatter backed by `ucrt::stdio` narrow printf formatting (1024-char Windows cap). |

### From `docs/stub-backlog.d/kernel-files-nls.md`

# Stub Backlog — `stubs/kernel-files-nls`

Scope: `crates/win32-rt/src/kernel32/file.rs` (8), `crates/win32-rt/src/kernel32/string.rs` (7), and `crates/win32-rt/src/ucrt/{locale,runtime,stdio}.rs` (6). **21 sites before → 0 sites after** (18 made real, 3 native-unsupported matching Windows without the privilege/capability, 0 leftovers).

## Removed Sites

| DLL!API | Disposition | File | Summary |
|---|---|---|---|
| `kernel32.dll!AllocConsole` | Made real | `crates/win32-rt/src/kernel32/file.rs` | Allocates a real POSIX PTY-backed console session on macOS (dup'd stdio on iOS) with background master-to-stderr drain, binds `FileKind::Console` std handles, tracks console mode/codepages/title, and rejects a second `AllocConsole` with `ERROR_ACCESS_DENIED` until `FreeConsole`. |
| `kernel32.dll!GetFileInformationByHandleEx` | Made real | `crates/win32-rt/src/kernel32/file.rs` | Implemented all documented read classes: `FileStreamInfo` (7, `::$DATA`), `FileCompressionInfo` (8), `FileAttributeTagInfo` (9 with symlink tag), `FileIdBothDirectoryInfo`/`Restart` (10/11), `FileRemoteProtocolInfo` (13 via `fstatfs` `MNT_LOCAL`), `FileFullDirectoryInfo`/`Restart` (14/15), `FileStorageInfo` (16 via `statfs`/IOKit), `FileIdExtdDirectoryInfo`/`Restart` (19/20), `FileCaseSensitiveInfo` (23 via `fpathconf(_PC_CASE_SENSITIVE)`), and `FileNormalizedNameInfo` (24). |
| `kernel32.dll!SetFileInformationByHandle` | Made real | `crates/win32-rt/src/kernel32/file.rs` | Implemented all documented write classes: `FileBasicInfo` (0 with read-only attribute update), `FileRenameInfo` (3) & `FileRenameInfoEx` (22 with `RootDirectory` resolution), `FileDispositionInfo`/`Ex` (4/21), `FileAllocationInfo` (5 with `F_PREALLOCATE` / shrink), `FileEndOfFileInfo` (6), `FileIoPriorityHintInfo` (12), and `FileCaseSensitiveInfo` (23). |
| `kernel32.dll!MoveFileExW` | Native-unsupported | `crates/win32-rt/src/kernel32/file.rs` | Validates `MOVEFILE_DELAY_UNTIL_REBOOT` / `MOVEFILE_COPY_ALLOWED` conflict (`ERROR_INVALID_PARAMETER`) and non-null `dst` requirement; returns `FALSE` + `ERROR_ACCESS_DENIED` for `MOVEFILE_DELAY_UNTIL_REBOOT` matching non-admin Windows behavior; adds `MOVEFILE_WRITE_THROUGH` `sync_all`. |
| `kernel32.dll!MoveFileExA` | Native-unsupported | `crates/win32-rt/src/kernel32/file.rs` | Delegates to shared `move_file_ex` with identical `MOVEFILE_DELAY_UNTIL_REBOOT` and `NULL` destination validation. |
| `kernel32.dll!MapViewOfFile3` | Native-unsupported | `crates/win32-rt/src/kernel32/file.rs` | Allows `NULL`, `CURRENT_PROCESS`, and `ProcessObj` (own process from `OpenProcess`); returns `ERROR_INVALID_HANDLE` for invalid handles and `ERROR_ACCESS_DENIED` for another OS process (`ChildProcessObj`), matching Windows without `PROCESS_VM_OPERATION`. |
| `kernel32.dll!DeviceIoControl` (`IOCTL_STORAGE_QUERY_PROPERTY`) | Made real | `crates/win32-rt/src/kernel32/file.rs` | Queries real volume/disk properties via `statfs` + macOS IOKit (`IOBSDNameMatching` / `IORegistryEntrySearchCFProperty`), implementing property IDs `0, 1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 14, 17` and `PropertyExistsQuery`, returning `ERROR_NOT_SUPPORTED` for properties not exposed by the hardware. |
| `kernel32.dll!DeviceIoControl` (control codes) | Made real | `crates/win32-rt/src/kernel32/file.rs` | Implemented `IOCTL_STORAGE_GET_DEVICE_NUMBER`, `IOCTL_STORAGE_CHECK_VERIFY`/`2`, `IOCTL_STORAGE_GET_MEDIA_TYPES_EX`, `IOCTL_STORAGE_GET_HOTPLUG_INFO`, `IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS`, `IOCTL_VOLUME_IS_OFFLINE`, `IOCTL_DISK_GET_DRIVE_GEOMETRY`/`EX`, `IOCTL_DISK_GET_LENGTH_INFO`, `IOCTL_DISK_IS_WRITABLE`, `IOCTL_DISK_GET_PARTITION_INFO_EX`, `FSCTL_GET_REPARSE_POINT` (POSIX symlinks), `FSCTL_SET_SPARSE`, `FSCTL_QUERY_ALLOCATED_RANGES` (`SEEK_DATA`/`SEEK_HOLE`), `FSCTL_SET_ZERO_DATA`, `FSCTL_GET_COMPRESSION`/`SET_COMPRESSION`, `FSCTL_IS_VOLUME_MOUNTED`, `FSCTL_GET_NTFS_VOLUME_DATA`, and `FSCTL_LOCK_VOLUME`/`UNLOCK_VOLUME`; unrecognized codes return `ERROR_INVALID_FUNCTION`. |
| `kernel32.dll!LCMapStringEx` (`LCMAP_HASH`) | Made real | `crates/win32-rt/src/kernel32/string.rs` | Computes a deterministic 32-bit FNV-1a hash over ICU locale-aware sort-weight bytes (`cchDest` in bytes = 4), matching case/norm equivalence. |
| `kernel32.dll!LCMapStringEx` (flags) | Made real | `crates/win32-rt/src/kernel32/string.rs` | Implements full ICU-backed casing (`LCMAP_LINGUISTIC_CASING`), `LCMAP_SORTKEY`, `LCMAP_HIRAGANA`/`KATAKANA`, `LCMAP_HALFWIDTH`/`FULLWIDTH`, `LCMAP_SIMPLIFIED_CHINESE`/`TRADITIONAL_CHINESE`, `LCMAP_BYTEREV`, and strict `ERROR_INVALID_FLAGS` validation. |
| `kernel32.dll!get_locale_info` (`LCTYPE`) | Made real | `crates/win32-rt/src/kernel32/string.rs` | Implements full Windows `LCTYPE` catalog (`0x01..=0x7F`, `0x1001..=0x1018`, including `LOCALE_FONTSIGNATURE`) via Apple `/usr/lib/libicucore.A.dylib`; invalid `LCTYPE` returns `ERROR_INVALID_FLAGS`. |
| `kernel32.dll!GetLocaleInfoW` | Made real | `crates/win32-rt/src/kernel32/string.rs` | Resolves real Windows LCIDs and BCP-47 locale names through ICU (`uloc_getLocaleForLCID` / `uloc_forLanguageTag`) instead of answering en-US; invalid LCIDs fail with `ERROR_INVALID_PARAMETER`. |
| `kernel32.dll!GetGeoInfoW` | Made real | `crates/win32-rt/src/kernel32/string.rs` | Maps Windows GeoIDs to ISO-3166-1 countries and answers `GEO_*` queries (localized names, ISO2/ISO3, UN numeric, dialing code, lat/lon, currency, LCID) via ICU; invalid GeoIDs return `ERROR_INVALID_PARAMETER`. |
| `kernel32.dll!GetNumberFormatEx` | Made real | `crates/win32-rt/src/kernel32/string.rs` | Honors custom `NUMBERFMTW` (`NumDigits`, `LeadingZero`, multi-level `Grouping`, `lpDecimalSep`, `lpThousandSep`, `NegativeOrder` 0–4, half-up rounding, flag validation) and ICU locale defaults when `lpFormat == NULL`. |
| `kernel32.dll!GetCalendarInfoEx` | Made real | `crates/win32-rt/src/kernel32/string.rs` | Implements all documented `CALTYPE` constants (`0x01..=0x3C`, `CAL_RETURN_NUMBER`) backed by ICU calendar/date symbols for the requested locale. |
| `ucrtbase.dll!setlocale` | Made real | `crates/win32-rt/src/ucrt/locale.rs` | Per-category (`LC_ALL..LC_TIME`) locale state supporting BCP-47, legacy CRT (`German_Germany.1252`), `.codepage`, and composite locale strings via ICU, with dynamic `localeconv()` and `___lc_*` queries; invalid locales return `NULL`. |
| `ucrtbase.dll!_setmbcp` | Made real | `crates/win32-rt/src/ucrt/locale.rs` | Tracks active multibyte codepage supporting `_MB_CP_SBCS`, `_MB_CP_OEM`, `_MB_CP_ANSI`, `_MB_CP_LOCALE`, and all valid Windows SBCS/DBCS/UTF-8 codepages via ICU; invalid codepages fail with `-1` and `EINVAL`. |
| `ucrtbase.dll!_getch` | Made real | `crates/win32-rt/src/ucrt/runtime.rs` | Reads 1 character directly from the active console input fd using raw `termios` (`~ICANON & ~ECHO`) when attached to a TTY, or POSIX `read(1)` on pipes/files, returning `-1` on EOF (paired with `poll`-based `_kbhit`). |
| `ucrtbase.dll!system` | Made real | `crates/win32-rt/src/ucrt/runtime.rs` | Flushes all `FILE` streams, checks `%COMSPEC%` (`C:\Windows\System32\cmd.exe`), executes `%COMSPEC% /c <cmd>` via `CreateProcessW` if a real guest interpreter exists, and returns `0` (for `NULL`) or `-1` + `ENOENT` (for non-null) when no Windows command interpreter is installed without host shell escape. |
| `ucrtbase.dll!open_host_path` | Made real | `crates/win32-rt/src/ucrt/stdio.rs` | Full `_O_WTEXT`, `_O_U16TEXT`, and `_O_U8TEXT` translation in `_read`/`_write` (UTF-16LE and UTF-8 ↔ UTF-16LE with CRLF ↔ LF, BOM handling for `_O_WTEXT`, even-byte `EINVAL`, exact `_setmode` tracking). |
| `ucrtbase.dll!parse_mode` | Made real | `crates/win32-rt/src/ucrt/stdio.rs` | Full `fopen(..., "..., ccs=UTF-16LE|UTF-8|UNICODE")` support with MSVC BOM precedence, BOM emission on new/truncated files, and UTF-16 wide stream I/O (`fgetwc`, `fputwc`, `fgetws`, `fputws`, `ungetwc`). |

## Native-Unsupported (Matches Windows Without the Feature)

| DLL!API | Trigger / Condition | Returned Behavior | Windows Documentation Citation |
|---|---|---|---|
| `kernel32.dll!MoveFileExW` | `dwFlags & MOVEFILE_DELAY_UNTIL_REBOOT` | Returns `FALSE` with `ERROR_ACCESS_DENIED` (5), or `ERROR_INVALID_PARAMETER` (87) if combined with `MOVEFILE_COPY_ALLOWED`. | Microsoft Learn, `MoveFileExW`: `MOVEFILE_DELAY_UNTIL_REBOOT` writes `HKLM\...\Session Manager\PendingFileRenameOperations` and "can be used only if the process is in the context of a user who belongs to the administrators group or the LocalSystem account" (non-admin callers receive `ERROR_ACCESS_DENIED`). |
| `kernel32.dll!MoveFileExA` | `dwFlags & MOVEFILE_DELAY_UNTIL_REBOOT` | Returns `FALSE` with `ERROR_ACCESS_DENIED` (5), or `ERROR_INVALID_PARAMETER` (87) if combined with `MOVEFILE_COPY_ALLOWED`. | Microsoft Learn, `MoveFileExA`: identical non-admin privilege rule as `MoveFileExW`. |
| `kernel32.dll!MapViewOfFile3` | `Process` handle refers to a different OS process (`ChildProcessObj`) | Returns `NULL` with `ERROR_ACCESS_DENIED` (5) (`ERROR_INVALID_HANDLE` for invalid handles; own `ProcessObj` succeeds). | Microsoft Learn, `MapViewOfFile3`: requires `PROCESS_VM_OPERATION` on `Process` and fails with `ERROR_ACCESS_DENIED` when mapping into another process is denied (macOS SIP / hardened runtime and iOS forbid `task_for_pid` foreign address-space mapping). |

## Remaining Sites in Scope

None — 0 stub sites remain across all 5 files in scope.

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

## Proprietary & Vendor Middleware (Legal / Architectural Limits)

These modules wrap proprietary third-party SDKs or GPU-vendor-specific extensions. Where a game ships its own Windows x86-64 PE DLL (e.g., Oodle `oo2core_*_win64.dll`, PhysX 4/5 `PhysX_64.dll`, `zstd.dll`), `port-runner` executes the real shipping PE through the ARM64 JIT rather than overriding it.

| DLL | Remaining Sites | Reason | Next Step |
|---|---:|---|---|

## Remaining Stubs by DLL

| DLL | Count | Reason | Next Step |
|---|---:|---|---|
| `bcrypt.dll` | 20 | Unimplemented CNG algorithm, key-wrap blob, or padding mode | Add remaining CNG algorithms/padding modes (OAEP/PSS/ECDH/3DES) backed by CommonCrypto/ring as games import them. |
| `crypt32.dll` | 19 | Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property | Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring. |
| `kernel32.dll` | 19 | Partial Windows kernel/file/memory/locale/job/threadpool sub-feature | Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives. |
| `d3d11.dll` | 14 | Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop) | Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them. |
| `user32.dll` | 13 | Uncommon Win32 window/metric/SPI/hotkey/cursor/display-config sub-feature | Fill in remaining GetSystemMetrics / SystemParametersInfo / DisplayConfig cases and custom bitmap NSCursor creation. |
| `ole32.dll` | 12 | Unimplemented WinRT IPropertyValue scalar/array type or VARIANT/SAFEARRAY coercion | Implement remaining IPropertyValue* types and VariantChangeType/SafeArrayDestroy coercions. |
| `physx3common_x64.dll` | 12 | PhysX 3.4 geometry/mesh query path without cooked mesh payload | Prefer shipping PhysX3*.dll under ARM64 JIT, or decode cooked BVH/triangle mesh payloads in physx3common_x64. |
| `dxgi.dll` | 10 | Unsupported Windows-only DXGI path (WARP software rasterizer, UWP CoreWindow, exclusive fullscreen ownership, event notifications) | Implement VSync-synchronized WaitForVBlank via CVDisplayLink/CADisplayLink and shared-handle LUID lookup; keep WARP/CoreWindow honestly unsupported. |
| `ws2_32.dll` | 10 | Uncommon Winsock2 async/QOS/control-message API or cross-process socket duplication | Implement WSARecvMsg/WSASendMsg via POSIX recvmsg/sendmsg and remaining WSAIoctl codes as games use them. |
| `ntdll.dll` | 9 | Unimplemented NT info class, TxF transaction API, or direct kernel-to-user dispatcher export | Add missing Process/Thread/System information classes on demand; TxF transactions and direct KiUser* calls remain honestly unsupported. |
| `advapi32.dll` | 8 | Legacy CryptoAPI (CAPI) key/sign/encrypt or uncommon token information class | Back remaining CAPI key/cipher calls with CommonCrypto/ring and implement missing TOKEN_INFORMATION_CLASS variants. |
| `mmdevapi.dll` | 6 | Optional WASAPI mode/service (exclusive mode, loopback capture, hardware offload, unknown COM IID) | Add CoreAudio loopback capture / tap support when a game requests AUDCLNT_STREAMFLAGS_LOOPBACK; keep exclusive/offload honestly rejected. |
| `dinput8.dll` | 5 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `dbghelp.dll` | 4 | Unimplemented DbgHelp stack walk or minidump writer | Wire StackWalk64 to win32-rt's .pdata unwinder and write valid MDMP headers for MiniDumpWriteDump. |
| `dwrite.dll` | 4 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `secur32.dll` | 4 | Server-side Schannel (INBOUND) or uncommon context/credential attribute | Add server-side TLS or missing SECPKG_ATTR_* queries if a game hosts local TLS sockets. |
| `wintrust.dll` | 4 | Unimplemented WinVerifyTrust action/choice or TPM Base Services (TBS) | Keep TPM TBS returning TBS_E_TPM_NOT_FOUND honestly; extend WinVerifyTrust for additional catalog/blob choices as needed. |
| `powrprof.dll` | 3 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `d3d12.dll` | 2 | D3D12 root-signature deserializer COM object not yet implemented | Parse DXBC/RTS0 root-signature blobs in win32-rt/src/d3d12 and expose ID3D12RootSignatureDeserializer / VersionedDeserializer. |
| `libcurl.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `nri.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `uxtheme.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `xaudio2_9redist.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `d2d1.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `directml.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `dsound.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `gameinput.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `gdi32.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `hid.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `msctf.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `shell32.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `xinput1_4.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |

### `bcrypt.dll` (20)

- **Reason:** Unimplemented CNG algorithm, key-wrap blob, or padding mode
- **Next step:** Add remaining CNG algorithms/padding modes (OAEP/PSS/ECDH/3DES) backed by CommonCrypto/ring as games import them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `BCryptExportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2447` | `"BCryptExportKey(wrapped h_export_key) -> STATUS_NOT_SUPPORTED"` |
| `BCryptExportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2505` | `"BCryptExportKey({t:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptGenerateKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2614` | `"BCryptGenerateKeyPair({}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptGenerateSymmetricKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2528` | `"BCryptGenerateSymmetricKey({}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptGetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1810` | `"BCryptGetProperty(alg={}, {p:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptGetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1824` | `"BCryptGetProperty(hash, {p:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptGetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1859` | `"BCryptGetProperty(key, {p:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptImportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2372` | `"BCryptImportKey({}, wrapped h_import_key) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2429` | `"BCryptImportKey({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1941` | `"BCryptImportKeyPair({}, wrapped import_key) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1949` | `"BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1964` | `"BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1979` | `"BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptOpenAlgorithmProvider` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1720` | `"BCryptOpenAlgorithmProvider({name:?}, flags 0x{flags:x}) -> STATUS_NOT_FOUND (algorithm not implemented)"` |
| `BCryptSetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2670` | `"BCryptSetProperty(ChainingMode={mode_str:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptSetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2718` | `"BCryptSetProperty(ECCCurveName={curve_str:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptSetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2756` | `"BCryptSetProperty({p:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptSignHash` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2057` | `"BCryptSignHash(RSA, flags 0x{flags:x}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptVerifySignature` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2122` | `"BCryptVerifySignature(RSA, flags 0x{flags:x}) -> STATUS_NOT_SUPPORTED (padding)"` |
| `bcrypt_aes_crypt` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2173` | `"BCrypt{} on non-AES key -> STATUS_NOT_SUPPORTED", if encrypt { "Encrypt" } else { "Decrypt" }` |

### `crypt32.dll` (19)

- **Reason:** Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property
- **Next step:** Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CertFreeCRLContext` | `stub` | `crates/win32-rt/src/misc/crypt32.rs:355` | `"CertFreeCRLContext({ctx:p}): foreign CRL context ignored"` |
| `CertFreeCTLContext` | `stub` | `crates/win32-rt/src/misc/crypt32.rs:362` | `"CertFreeCTLContext({ctx:p}): foreign CTL context ignored"` |
| `CertGetCertificateContextProperty` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CertStrToNameA` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CertStrToNameW` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CertVerifySubjectCertificateContext` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptDecodeObject` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptDecodeObjectEx` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptFindOIDInfo` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:42` | `*mut c_void = std::ptr::null_mut(), err CRYPT_E_NOT_FOUND` |
| `CryptImportPublicKeyInfo` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptMsgOpenToDecode` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:42` | `*mut c_void = std::ptr::null_mut(), err CRYPT_E_NOT_FOUND` |
| `CryptMsgUpdate` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptVerifyMessageSignature` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `PFXImportCertStore` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:42` | `*mut c_void = std::ptr::null_mut(), err CRYPT_E_NOT_FOUND` |
| `cert_free_certificate_chain_engine_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/chain.rs:593` | `"CertFreeCertificateChainEngine({engine:p}): foreign engine ignored"` |
| `cert_free_certificate_chain_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/chain.rs:787` | `"CertFreeCertificateChain({chain:p}): foreign chain ignored"` |
| `cert_verify_certificate_chain_policy_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/chain.rs:807` | `"CertVerifyCertificateChainPolicy(oid={policy_id:#x}) -> FALSE"` |
| `crypt_string_to_binary_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/asn1.rs:1276` | `"CryptStringToBinary unsupported flags {flags:#x}"` |
| `encode_x509_object` | `stub` | `crates/win32-rt/src/misc/crypt32/asn1.rs:941` | `"CryptEncodeObjectEx(struct_type={raw_stype:#x}) -> FALSE"` |

### `kernel32.dll` (19)

- **Reason:** Partial Windows kernel/file/memory/locale/job/threadpool sub-feature
- **Next step:** Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives.

| API | Kind | Location | Detail |
|---|---|---|---|
| `AssignProcessToJobObject` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1655` | `"job objects have no effect"` |
| `CloseThreadpoolCleanupGroupMembers` | `stub` | `crates/win32-rt/src/kernel32/threadpool.rs:808` | `"cleanup groups do not track members"` |
| `CreateProcessW` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1283` | `"CreateProcessW unavailable on iOS: app={:?} cmd={:?}", from_wide_opt(app), from_wide_opt(cmd)` |
| `CreateRemoteThread` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:60` | `"CreateRemoteThread into another process"` |
| `FlushProcessWriteBuffers` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:1028` | `"FlushProcessWriteBuffers approximated by a local fence"` |
| `InitOnceBeginInitialize` | `stub` | `crates/win32-rt/src/sync.rs:744` | `"INIT_ONCE_ASYNC treated as synchronous"` |
| `SetInformationJobObject` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1659` | `"SetInformationJobObject class {class} ignored"` |
| `SetThreadAffinityMask` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:244` | `"thread affinity not applied"` |
| `SetThreadGroupAffinity` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:250` | `"thread group affinity not applied"` |
| `TerminateThread` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:173` | `"TerminateThread unsupported"` |
| `VirtualAlloc2` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:575` | `"VirtualAlloc2 on another process"` |
| `VirtualAllocEx` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:566` | `"VirtualAllocEx on another process"` |
| `VirtualProtect` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:678` | `"PAGE_GUARD not emulated"` |
| `format_message` | `stub` | `crates/win32-rt/src/kernel32/process.rs:2180` | `"message tables in modules are not supported (id {id:#x})"` |
| `format_message` | `stub` | `crates/win32-rt/src/kernel32/process.rs:2189` | `"FormatMessage from module message table (id {id:#x})"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:362` | `"MEM_LARGE_PAGES ignored"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:365` | `"MEM_WRITE_WATCH not tracked"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:368` | `"PAGE_GUARD not emulated (treated as normal pages)"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:375` | `"MEM_RESET_UNDO: contents may have been discarded"` |

### `d3d11.dll` (14)

- **Reason:** Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop)
- **Next step:** Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `check_counter` | `stub` | `crates/win32-rt/src/d3d11/device.rs:1074` | `"ID3D11Device::CheckCounter -> DXGI_ERROR_UNSUPPORTED"` |
| `create_class_linkage` | `stub` | `crates/win32-rt/src/d3d11/device.rs:749` | `"ID3D11Device::CreateClassLinkage -> E_NOTIMPL"` |
| `create_counter` | `stub` | `crates/win32-rt/src/d3d11/device.rs:958` | `"ID3D11Device::CreateCounter -> DXGI_ERROR_UNSUPPORTED"` |
| `create_device_context_state` | `stub` | `crates/win32-rt/src/d3d11/device.rs:1038` | `"ID3D11Device1::CreateDeviceContextState -> E_NOTIMPL"` |
| `create_domain_shader` | `stub` | `crates/win32-rt/src/d3d11/device.rs:743` | `"ID3D11Device::CreateDomainShader -> E_NOTIMPL"` |
| `create_geometry_shader_so` | `stub` | `crates/win32-rt/src/d3d11/device.rs:727` | `"ID3D11Device::CreateGeometryShaderWithStreamOutput -> E_NOTIMPL"` |
| `create_hull_shader` | `stub` | `crates/win32-rt/src/d3d11/device.rs:735` | `"ID3D11Device::CreateHullShader -> E_NOTIMPL"` |
| `draw_auto` | `stub` | `crates/win32-rt/src/d3d11/context.rs:2732` | `"ID3D11DeviceContext::DrawAuto not implemented"` |
| `ds_set_shader` | `stub` | `crates/win32-rt/src/d3d11/context.rs:1972` | `"DSSetShader(non-null) ignored"` |
| `dxgi_dev_create_surface` | `stub` | `crates/win32-rt/src/d3d11/device.rs:1308` | `"IDXGIDevice::CreateSurface -> E_NOTIMPL"` |
| `get_data` | `stub` | `crates/win32-rt/src/d3d11/context.rs:2908` | `"GetData(PIPELINE_STATISTICS) returning zeroes"` |
| `hs_set_shader` | `stub` | `crates/win32-rt/src/d3d11/context.rs:1969` | `"HSSetShader(non-null) ignored"` |
| `surf_get_dc` | `stub` | `crates/win32-rt/src/d3d11/resources.rs:1308` | `"IDXGISurface1::GetDC -> E_NOTIMPL"` |
| `swap_device_context_state` | `stub` | `crates/win32-rt/src/d3d11/context.rs:3013` | `"ID3D11DeviceContext1::SwapDeviceContextState not implemented"` |

### `user32.dll` (13)

- **Reason:** Uncommon Win32 window/metric/SPI/hotkey/cursor/display-config sub-feature
- **Next step:** Fill in remaining GetSystemMetrics / SystemParametersInfo / DisplayConfig cases and custom bitmap NSCursor creation.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CreateCursor` | `stub` | `crates/win32-rt/src/user32/cursor.rs:987` | `"CreateCursor {w}x{h}: custom bitmap cursors are shown as the arrow"` |
| `DisplayConfigGetDeviceInfo` | `stub` | `crates/win32-rt/src/user32/monitor.rs:684` | `"DisplayConfigGetDeviceInfo(type {ty}) -> ERROR_NOT_SUPPORTED"` |
| `DisplayConfigSetDeviceInfo` | `stub` | `crates/win32-rt/src/user32/monitor.rs:692` | `"DisplayConfigSetDeviceInfo(type {ty}) -> ERROR_NOT_SUPPORTED"` |
| `GetSystemMetrics` | `stub` | `crates/win32-rt/src/user32/misc.rs:234` | `"GetSystemMetrics({index}) -> 0"` |
| `RegisterHotKey` | `stub` | `crates/win32-rt/src/user32/misc.rs:759` | `"RegisterHotKey(hwnd {:#x}, id {id}, mods {mods:#x}, vk {vk:#x}): never fires", hwnd as usize` |
| `RegisterRawInputDevices` | `stub` | `crates/win32-rt/src/user32/input.rs:1363` | `"RegisterRawInputDevices: RIDEV_NOLEGACY ignored (legacy messages still sent)"` |
| `ReplyMessage` | `stub` | `crates/win32-rt/src/user32/message.rs:883` | `"ReplyMessage: early reply not supported"` |
| `def_window_proc` | `stub` | `crates/win32-rt/src/user32/defproc.rs:154` | `"DefWindowProc(WM_SYSCOMMAND {other:#x}) ignored"` |
| `load_cursor` | `stub` | `crates/win32-rt/src/user32/cursor.rs:87` | `"LoadCursor({inst:#x}, {n:?}): cursor resources not supported -> NULL"` |
| `message_box` | `stub` | `crates/win32-rt/src/user32/misc.rs:1876` | `"MessageBox type {ty:#x}: headless, cannot ask the user -> 0 (failure)"` |
| `spi` | `stub` | `crates/win32-rt/src/user32/misc.rs:565` | `"SystemParametersInfo{}({action:#x}, {uparam}) not supported -> FALSE", if wide { "W" } else { "A" }` |
| `user_object_info` | `stub` | `crates/win32-rt/src/user32/misc.rs:600` | `"GetUserObjectInformation on handle {h:#x} (only the window station is known)"` |
| `user_object_info` | `stub` | `crates/win32-rt/src/user32/misc.rs:622` | `"GetUserObjectInformation index {index} not supported"` |

### `ole32.dll` (12)

- **Reason:** Unimplemented WinRT IPropertyValue scalar/array type or VARIANT/SAFEARRAY coercion
- **Next step:** Implement remaining IPropertyValue* types and VariantChangeType/SafeArrayDestroy coercions.

| API | Kind | Location | Detail |
|---|---|---|---|
| `pv_get_date_time` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2167` | `"IPropertyValue::GetDateTime -> E_NOTIMPL"` |
| `pv_get_point` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2185` | `"IPropertyValue::GetPoint -> E_NOTIMPL"` |
| `pv_get_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2203` | `"IPropertyValue::GetRect -> E_NOTIMPL"` |
| `pv_get_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2223` | `concat!("IPropertyValue::", $method, " -> E_NOTIMPL")` |
| `pv_get_size` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2194` | `"IPropertyValue::GetSize -> E_NOTIMPL"` |
| `pv_get_time_span` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2176` | `"IPropertyValue::GetTimeSpan -> E_NOTIMPL"` |
| `pvs_create_date_time` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2582` | `"IPropertyValueStatics::CreateDateTime -> E_NOTIMPL"` |
| `pvs_create_point` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2609` | `"IPropertyValueStatics::CreatePoint -> E_NOTIMPL"` |
| `pvs_create_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2640` | `"IPropertyValueStatics::CreateRect -> E_NOTIMPL"` |
| `pvs_create_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2659` | `concat!("IPropertyValueStatics::", $method, " -> E_NOTIMPL")` |
| `pvs_create_size` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2623` | `"IPropertyValueStatics::CreateSize -> E_NOTIMPL"` |
| `pvs_create_time_span` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2595` | `"IPropertyValueStatics::CreateTimeSpan -> E_NOTIMPL"` |

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

### `dxgi.dll` (10)

- **Reason:** Unsupported Windows-only DXGI path (WARP software rasterizer, UWP CoreWindow, exclusive fullscreen ownership, event notifications)
- **Next step:** Implement VSync-synchronized WaitForVBlank via CVDisplayLink/CADisplayLink and shared-handle LUID lookup; keep WARP/CoreWindow honestly unsupported.

| API | Kind | Location | Detail |
|---|---|---|---|
| `DXGIGetDebugInterface1` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:114` | `"DXGIGetDebugInterface1({:?}) -> E_NOINTERFACE (no debug layer)", Guid::read(riid).unwrap_or_default()` |
| `create_software_adapter` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:248` | `"CreateSoftwareAdapter -> DXGI_ERROR_UNSUPPORTED"` |
| `create_swap_chain_unsupported` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:275` | `"CreateSwapChainForCoreWindow/Composition -> DXGI_ERROR_UNSUPPORTED"` |
| `enum_warp_adapter` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:240` | `"EnumWarpAdapter: no WARP software rasterizer -> DXGI_ERROR_NOT_FOUND"` |
| `get_shared_resource_adapter_luid` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:283` | `"GetSharedResourceAdapterLuid: no shared handles -> E_INVALIDARG"` |
| `register_event_stub` | `stub` | `crates/win32-rt/src/dxgi/adapter.rs:298` | `"IDXGIAdapter3 notification event registered; it is never signaled"` |
| `register_status_event` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:298` | `"Register*Event (stereo/occlusion/adapters-changed): the event is never signaled"` |
| `register_status_window` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:290` | `"Register{{Stereo,Occlusion}}StatusWindow: status changes are never posted"` |
| `take_ownership` | `stub` | `crates/win32-rt/src/dxgi/adapter.rs:669` | `"TakeOwnership: exclusive fullscreen is not supported"` |
| `wait_for_vblank` | `stub` | `crates/win32-rt/src/dxgi/adapter.rs:663` | `"WaitForVBlank: sleeps one refresh period ({hz} Hz), not synchronized to the display"` |

### `ws2_32.dll` (10)

- **Reason:** Uncommon Winsock2 async/QOS/control-message API or cross-process socket duplication
- **Next step:** Implement WSARecvMsg/WSASendMsg via POSIX recvmsg/sendmsg and remaining WSAIoctl codes as games use them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `WSAAsyncSelect` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSACancelAsyncRequest` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAConnectByNameA` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAConnectByNameW` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAGetQOSByName` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAIoctl` | `stub` | `crates/win32-rt/src/misc/ws2_32.rs:1782` | `"WSAIoctl(code={io_control_code:#x}) -> WSAEOPNOTSUPP"` |
| `WSAJoinLeaf` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSARecvMsg` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSASendMsg` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `duplicate_socket_into_info` | `stub` | `crates/win32-rt/src/misc/ws2_32.rs:1319` | `"WSADuplicateSocket cross-process target pid={pid} unsupported"` |

### `ntdll.dll` (9)

- **Reason:** Unimplemented NT info class, TxF transaction API, or direct kernel-to-user dispatcher export
- **Next step:** Add missing Process/Thread/System information classes on demand; TxF transactions and direct KiUser* calls remain honestly unsupported.

| API | Kind | Location | Detail |
|---|---|---|---|
| `KiUserApcDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1662` | `"KiUserApcDispatcher called directly"` |
| `KiUserCallbackDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1666` | `"KiUserCallbackDispatcher called directly"` |
| `KiUserExceptionDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1658` | `"KiUserExceptionDispatcher called directly"` |
| `LdrInitializeThunk` | `stub` | `crates/win32-rt/src/ntdll.rs:2404` | `"LdrInitializeThunk called directly"` |
| `NtCommitTransaction` | `stub` | `crates/win32-rt/src/ntdll.rs:731` | `"NtCommitTransaction -> STATUS_NOT_IMPLEMENTED"` |
| `NtCreateTransaction` | `stub` | `crates/win32-rt/src/ntdll.rs:726` | `"NtCreateTransaction -> STATUS_NOT_IMPLEMENTED"` |
| `NtQueryInformationProcess` | `stub` | `crates/win32-rt/src/ntdll.rs:239` | `"NtQueryInformationProcess class {class}"` |
| `NtQueryInformationThread` | `stub` | `crates/win32-rt/src/ntdll.rs:303` | `"NtQueryInformationThread class {class}"` |
| `NtQuerySystemInformation` | `stub` | `crates/win32-rt/src/ntdll.rs:422` | `"NtQuerySystemInformation class {class}"` |

### `advapi32.dll` (8)

- **Reason:** Legacy CryptoAPI (CAPI) key/sign/encrypt or uncommon token information class
- **Next step:** Back remaining CAPI key/cipher calls with CommonCrypto/ring and implement missing TOKEN_INFORMATION_CLASS variants.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CryptDecrypt` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptDestroyKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptEncrypt` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptExportKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptGetUserKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptImportKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptSignHashW` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `GetTokenInformation` | `stub` | `crates/win32-rt/src/advapi32.rs:1361` | `"GetTokenInformation(class={info_class}) -> ERROR_CALL_NOT_IMPLEMENTED"` |

### `mmdevapi.dll` (6)

- **Reason:** Optional WASAPI mode/service (exclusive mode, loopback capture, hardware offload, unknown COM IID)
- **Next step:** Add CoreAudio loopback capture / tap support when a game requests AUDCLNT_STREAMFLAGS_LOOPBACK; keep exclusive/offload honestly rejected.

| API | Kind | Location | Detail |
|---|---|---|---|
| `activate` | `stub` | `crates/win32-rt/src/audio/mmdevapi.rs:592` | `"IMMDevice::Activate({iid:?} = {what}) -> E_NOINTERFACE"` |
| `get_buffer_size_limits` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:919` | `"IAudioClient2::GetBufferSizeLimits -> E_NOTIMPL (no offload)"` |
| `get_service` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:885` | `"IAudioClient::GetService({iid:?} = {what}) -> E_NOINTERFACE"` |
| `get_service` | `stub` | `crates/win32-rt/src/audio/spatial.rs:846` | `"ISpatialAudioObjectRenderStream::GetService({iid:?}) -> E_NOINTERFACE"` |
| `initialize` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:427` | `"IAudioClient::Initialize: exclusive mode is not supported -> AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED"` |
| `initialize` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:440` | `"IAudioClient::Initialize: loopback capture is not supported -> E_NOTIMPL"` |

### `dinput8.dll` (5)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `configure_devices` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1862` | `"IDirectInput8::ConfigureDevices -> DIERR_UNSUPPORTED"` |
| `dev_get_property` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1239` | `"IDirectInputDevice8::GetProperty({other:#x}) -> DIERR_UNSUPPORTED"` |
| `dev_run_control_panel` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1557` | `"IDirectInputDevice8::RunControlPanel -> DIERR_UNSUPPORTED"` |
| `dev_set_property` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1331` | `"IDirectInputDevice8::SetProperty({other:#x}) -> DIERR_UNSUPPORTED"` |
| `run_control_panel` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1822` | `"IDirectInput8::RunControlPanel -> DIERR_UNSUPPORTED"` |

### `dbghelp.dll` (4)

- **Reason:** Unimplemented DbgHelp stack walk or minidump writer
- **Next step:** Wire StackWalk64 to win32-rt's .pdata unwinder and write valid MDMP headers for MiniDumpWriteDump.

| API | Kind | Location | Detail |
|---|---|---|---|
| `MiniDumpWriteDump` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `StackWalk64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `SymGetLineFromAddr64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `SymGetLineFromAddrW64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |

### `dwrite.dll` (4)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `fac_create_custom_font_collection` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:76` | `"IDWriteFactory::CreateCustomFontCollection -> E_NOTIMPL"` |
| `fac_create_custom_font_file_reference` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:119` | `"IDWriteFactory::CreateCustomFontFileReference -> E_NOTIMPL"` |
| `fac_create_glyph_run_analysis` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:365` | `"IDWriteFactory::CreateGlyphRunAnalysis -> E_NOTIMPL"` |
| `fac_create_text_analyzer` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:336` | `"IDWriteFactory::CreateTextAnalyzer -> E_NOTIMPL"` |

### `secur32.dll` (4)

- **Reason:** Server-side Schannel (INBOUND) or uncommon context/credential attribute
- **Next step:** Add server-side TLS or missing SECPKG_ATTR_* queries if a game hosts local TLS sockets.

| API | Kind | Location | Detail |
|---|---|---|---|
| `acquire_credentials_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:594` | `"AcquireCredentialsHandle(pkg={package_name:?}, use={f_credential_use:#x}): inbound unsupported"` |
| `apply_control_token_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:1025` | `"ApplyControlToken(ctx={id}, token={token_val:?}) -> SEC_E_UNSUPPORTED_FUNCTION"` |
| `query_context_attributes_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:1787` | `"QueryContextAttributes(ctx={ctx_id}, attr={other:#x}) -> SEC_E_UNSUPPORTED_FUNCTION"` |
| `query_credentials_attributes_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:1849` | `"QueryCredentialsAttributes{}(attr={ul_attribute:#x}) -> SEC_E_UNSUPPORTED_FUNCTION", if wide { "W" } else { "A" }` |

### `wintrust.dll` (4)

- **Reason:** Unimplemented WinVerifyTrust action/choice or TPM Base Services (TBS)
- **Next step:** Keep TPM TBS returning TBS_E_TPM_NOT_FOUND honestly; extend WinVerifyTrust for additional catalog/blob choices as needed.

| API | Kind | Location | Detail |
|---|---|---|---|
| `Tbsip_Context_Close` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1644` | `"Tbsip_Context_Close({h_context:p}) -> TBS_E_INVALID_CONTEXT"` |
| `Tbsip_Submit_Command` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1661` | `"Tbsip_Submit_Command -> TBS_E_TPM_NOT_FOUND"` |
| `WinVerifyTrust` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1415` | `"WinVerifyTrust(action={a}) -> TRUST_E_ACTION_UNKNOWN"` |
| `WinVerifyTrust` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1439` | `"WinVerifyTrust(choice={union_choice}) -> TRUST_E_NOSIGNATURE"` |

### `powrprof.dll` (3)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CallNtPowerInformation` | `stub` | `crates/win32-rt/src/misc/powrprof.rs:468` | `"CallNtPowerInformation(level={info_level}) -> STATUS_NOT_IMPLEMENTED"` |
| `PowerRegisterSuspendResumeNotification` | `stub` | `crates/win32-rt/src/misc/powrprof.rs:533` | `"PowerRegisterSuspendResumeNotification(flags={flags:#x}) -> {handle:#x} (handle tracked; OS wake callback not armed)"` |
| `PowerSettingRegisterNotification` | `stub` | `crates/win32-rt/src/misc/powrprof.rs:604` | `"PowerSettingRegisterNotification(flags={flags:#x}) -> {handle:#x} (handle tracked; OS setting callback not armed)"` |

### `d3d12.dll` (2)

- **Reason:** D3D12 root-signature deserializer COM object not yet implemented
- **Next step:** Parse DXBC/RTS0 root-signature blobs in win32-rt/src/d3d12 and expose ID3D12RootSignatureDeserializer / VersionedDeserializer.

| API | Kind | Location | Detail |
|---|---|---|---|
| `D3D12CreateRootSignatureDeserializer` | `stub` | `crates/win32-rt/src/d3d12/mod.rs:191` | `"D3D12CreateRootSignatureDeserializer({size} bytes, {:?}) -> E_NOTIMPL", Guid::read(riid).unwrap_or_default()` |
| `D3D12CreateVersionedRootSignatureDeserializer` | `stub` | `crates/win32-rt/src/d3d12/mod.rs:199` | `"D3D12CreateVersionedRootSignatureDeserializer({size} bytes, {:?}) -> E_NOTIMPL", Guid::read(riid).unwrap_or_default()` |

### `libcurl.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `easy_getinfo` | `middleware_stub` | `crates/win32-rt/src/misc/libcurl/easy.rs:1344` | `"libcurl: unsupported curl_easy_getinfo(info={other:#x})"` |
| `easy_setopt` | `middleware_stub` | `crates/win32-rt/src/misc/libcurl/easy.rs:1158` | `"libcurl: unsupported curl_easy_setopt(option={other}, param={param:#x})"` |

### `nri.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `core_cmd_set_sample_positions` | `middleware_stub` | `crates/win32-rt/src/misc/nri/cmdlist.rs:965` | `"CmdSetSamplePositions: programmable sample positions not supported on Metal"` |
| `core_create_query_pool` | `middleware_stub` | `crates/win32-rt/src/misc/nri/device.rs:1383` | `"CreateQueryPool: queryType {} is not supported on Metal", qd.query_type` |

### `uxtheme.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `DrawThemeBackground` | `stub` | `crates/win32-rt/src/misc/uxtheme.rs:111` | `"DrawThemeBackground(htheme={htheme:#x}) -> E_NOTIMPL"` |
| `TaskDialogIndirect` | `stub` | `crates/win32-rt/src/misc/uxtheme.rs:203` | `"TaskDialogIndirect -> E_NOTIMPL (native task dialogs not implemented)"` |

### `xaudio2_9redist.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `parse_wave_format` | `middleware_stub` | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs:216` | `"XAudio2 CreateSourceVoice: unsupported WAVE_FORMAT_EXTENSIBLE SubFormat {sub:?}"` |
| `parse_wave_format` | `middleware_stub` | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs:233` | `"XAudio2 CreateSourceVoice: unsupported format tag {tag:#x} bits {bits}"` |

### `d2d1.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `geom_tessellate` | `stub` | `crates/win32-rt/src/misc/d2d1/geometry.rs:1038` | `"ID2D1Geometry::Tessellate -> E_NOTIMPL"` |

### `directml.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `parse` | `middleware_stub` | `crates/win32-rt/src/misc/directml/ops.rs:1048` | `"DirectML CreateOperator: unsupported operator type {other}"` |

### `dsound.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `dsb_set_fx` | `stub` | `crates/win32-rt/src/misc/dsound/com_vtbl.rs:1458` | `"IDirectSoundBuffer8::SetFX({dw_effects_count} effects) -> DSERR_FXUNAVAILABLE"` |

### `gameinput.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `gi_get_temporal_reading` | `stub` | `crates/win32-rt/src/misc/gameinput.rs:1968` | `"IGameInput::GetTemporalReading -> E_NOTIMPL"` |

### `gdi32.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `GetDeviceCaps` | `stub` | `crates/win32-rt/src/misc/gdi32.rs:860` | `"GetDeviceCaps({index}) -> 0"` |

### `hid.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `HidD_SetNumInputBuffers` | `stub` | `crates/win32-rt/src/misc/hid.rs:887` | `"ring size stays {INPUT_BUFFERS} (asked {n})"` |

### `msctf.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `ctx_track_properties` | `stub` | `crates/win32-rt/src/misc/msctf/context.rs:809` | `"ITfContext::TrackProperties -> E_NOTIMPL"` |

### `shell32.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `SHCreateItemFromParsingName` | `stub` | `crates/win32-rt/src/misc/shell32.rs:1733` | `"SHCreateItemFromParsingName: unsupported shell namespace {raw:?}"` |

### `xinput1_4.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `XInputGetKeystroke` | `stub` | `crates/win32-rt/src/misc/xinput1_4.rs:608` | `"XInputGetKeystroke: keystroke events not generated (ERROR_EMPTY)"` |

