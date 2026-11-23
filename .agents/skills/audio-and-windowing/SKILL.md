---
name: audio-and-windowing
description: Develop or debug Win32 windowing, AppKit/UIKit main-thread messaging, DPI/Retina scaling, borderless fullscreen, raw mouse/keyboard/IME/TSF, XInput/DirectInput/GameInput/DualSense HID, WASAPI/ISpatialAudioClient/XAudio2 XAPO/FMOD/DirectSound/OpenAL/Bink Audio on CoreAudio, or the universal crates/wwise-audio 2022.1 runtime. Use when fixing UI deadlocks, window resize/focus/fullscreen bugs, input/gamepad/haptics issues, audio glitches/hotplug/capture, or Wwise/FMOD/XAudio2 bugs.
---

# Windowing, Input, Native Audio Engines & Universal Wwise

See [audio_and_input_matrix.md](./references/audio_and_input_matrix.md) for the complete source map
across all windowing, TSF/IME, gamepad/HID, WASAPI, XAudio2/XAPO, FMOD Core+Studio, DirectSound 8,
OpenAL 1.1, Bink Audio, and Wwise 2022.1 modules plus all debug environment knobs.

## 1. Windowing & Input (`crates/win32-rt/src/{user32,macos,misc/*}`)

### Non-Negotiable Threading & Deadlock Rules
1. **AppKit (macOS) / UIKit (iOS) owns the process main thread.** Guest code and `WndProc`
   **NEVER** run on the main thread; `WndProc` runs strictly on the creating Win32 thread inside
   `GetMessage`/`PeekMessage`/`SendMessage`/`DispatchMessage`.
2. **THE DEADLOCK RULE:** `window_create` is the **only** `dispatch_sync` onto the main queue.
   Every other UI mutation (destroy, frame/resize, style, fullscreen toggle, show/hide, title,
   cursor, warp) MUST be FIFO `dispatch_async` capturing plain values only. **Never hold a window
   lock on a Win32 thread while synchronously waiting on the main thread.**
3. **Never call guest code from AppKit/UIKit delegates.** Queue events onto the owning Win32
   thread's message queue (integrated with the kernel wait lock for `MsgWaitForMultipleObjects[Ex]`).
4. **Cross-platform & headless guards:** Gate AppKit/IOKit under `#[cfg(target_os = "macos")]`
   with UIKit counterparts in `macos/uikit.rs`. Keep `PORT_HEADLESS=1` working cleanly (synthetic
   1920×1080 display, zero `NSApplication` — required for CI and unit tests).

### Key Windowing, IME & Controller Invariants
- **Fullscreen & mixed-DPI:** `window_set_dxgi_fullscreen` (`macos/appkit.rs`) saves/restores
  frame, `styleMask`, `level`, and `collectionBehavior` and guards `window_set_frame`/`style`
  against clobbering active borderless fullscreen. Mixed-DPI displays propagate `WM_DPICHANGED`.
- **Hooks, icons, clipboard & TSF/IME:** `SetWindowsHookEx*`, PE `LoadIcon` resources,
  `EnumDisplayMonitors`, `NSPasteboard` clipboard, HIToolbox `'uchr'`/`UCKeyTranslate`, `imm32`,
  and `msctf.rs` (`CLSID_TF_ThreadMgr`) are implemented natively.
- **Raw mouse & controllers:** `AW2_RAW_MOUSE=auto|gc|nsevent` (`GCMouse` with >500 ms fallback to
  `NSEvent`). `GameController` + CoreHaptics drives `xinput1_4`, `dinput8`, and `gameinput`. Sony
  DS4/DualSense (`VID 054C`) exposed via `IOHIDManager` + `SetupDi*` are **hidden from XInput** to
  prevent duplicate controllers, and pair USB audio-haptics via `SPDRP_BASE_CONTAINERID` ↔
  `PKEY_Device_ContainerId`.

## 2. Native Audio Stack (`win32-rt/src/{audio,misc/*}` & `crates/wwise-audio`)

- **WASAPI → CoreAudio (`win32-rt/src/audio/`):** Shared-mode float32 render (48 kHz stereo
  default), read-only-probed silent `eCapture`/`eAll` + `IAudioCaptureClient` (**zero TCC mic
  prompt**), real `ISpatialAudioClient` (static beds + up to 128 3D objects), `IAudioSessionManager2`,
  `ActivateAudioInterfaceAsync` (#17), Mach RT scheduling (`avrt`), and `IMMNotificationClient`
  hotplug (`AUDCLNT_E_DEVICE_INVALIDATED` on removal/format change; default switch does **not**
  invalidate running streams).
- **Native Windows Audio Middleware (all CoreAudio/Null-backed):**
  - `xaudio2_9redist.rs` — XAudio2.9 + X3DAudio + real dual-interface COM `IXAPO`/`IXAPOParameters`
    (`CreateAudioReverb`, `CreateAudioVolumeMeter`, `CreateFX` `FXEQ`/`FXMasteringLimiter`/
    `FXReverb`/`FXEcho`) + arbitrary guest COM `IXAPO` chains (always drop locks before `call_guest8`!).
  - `fmod.rs` — Pure-Rust `FSB5` (PCM/IMA-ADPCM/FADPCM) & RIFF `FEV ` `.bank` parser + live FMOD
    Core & Studio mixer.
  - `dsound.rs`, `openal32.rs`, and `bink2w64.rs` (Bink Audio DCT/RDFT).
- **Universal Wwise 2022.1 (`crates/wwise-audio`):** Decoupled via `AudioDataSource` (keep AW2
  Northlight VFS behind `--features northlight` and `win32-rt` `AK::` hooks behind
  `--features wwise-bridge`). Includes `FloorSynthesis::AkAvx2`/`Libvorbis`, `CAkBus::IsMixingBus`
  pass-through folding, 21 DSP plugins, and partitioned overlap-add FFT convolver (`00 04 02 00`
  BFloat16 / `00 04 00 00` Float32 IRs). AW2 offline oracle: `tools/audio_harness.sh` &
  `tools/audio_oracle.sh`.

## Verification Checklist

```sh
# Windowing, input, TSF, gamepads & HID
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- \
  user32 macos misc::dinput8 misc::gameinput misc::hid misc::setupapi misc::xinput1_4 misc::msctf --test-threads=1

# WASAPI (verified deadlock-free in parallel too) + wwise-bridge
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- audio misc::setupapi
cargo test -p win32-rt --lib --features wwise-bridge --target aarch64-apple-darwin -- audio

# Native audio middleware (XAudio2/XAPO, FMOD Studio, DirectSound, OpenAL, Bink2)
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- \
  misc::xaudio2_9redist misc::fmod misc::dsound misc::openal32 misc::bink2w64 --test-threads=1

# Universal Wwise 2022.1 + iOS cross-check + runner build
cargo test -p wwise-audio --lib --target aarch64-apple-darwin
cargo check -p win32-rt --lib --target aarch64-apple-ios
cargo runner-build
```
