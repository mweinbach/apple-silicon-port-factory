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
