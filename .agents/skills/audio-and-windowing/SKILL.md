---
name: audio-and-windowing
description: Develop or debug Win32 windowing, AppKit/UIKit main-thread messaging, DPI/Retina scaling, raw mouse/keyboard, XInput/DirectInput/DualSense HID, WASAPI-on-CoreAudio, or the universal crates/wwise-audio 2022.1 runtime. Use when fixing UI deadlocks, window resize/focus bugs, mouse/gamepad/haptics issues, audio glitches/hotplug/capture, or Wwise .bnk/.wem/DSP parity bugs.
---

# Windowing, Input, WASAPI Audio & Universal Wwise

## 1. Windowing & Input (`crates/win32-rt/src/{user32,macos,misc/{xinput1_4,dinput8,hid,setupapi}}.rs`)

### Non-Negotiable Threading & Deadlock Rules
1. **AppKit (macOS) / UIKit (iOS) owns the process main thread.** Guest code and `WndProc`
   **NEVER** execute on the main thread; `WndProc` runs strictly on the creating Win32 thread
   inside `GetMessage`/`PeekMessage`/`SendMessage`/`DispatchMessage`.
2. **THE DEADLOCK RULE:** `window_create` is the **only** `dispatch_sync` onto the main queue.
   Every other UI operation (destroy, frame/resize, style, show/hide, title, cursor, warp) MUST be
   FIFO `dispatch_async` capturing plain values only. **Never hold a window lock on a Win32 thread
   while synchronously waiting on the main thread.**
3. **Never call guest code from AppKit delegates/views.** Translate events into queued Win32 input
   or internal sent messages on the owning Win32 thread's queue (integrated with the kernel wait
   lock for `MsgWaitForMultipleObjects[Ex]`).
4. **Cross-platform guard:** Gate AppKit/IOKit code under `#[cfg(target_os = "macos")]` and keep
   UIKit counterparts in `macos/uikit.rs`. Always support `PORT_HEADLESS=1` (no `NSApplication`,
   synthetic 1920×1080 display — used by CI and unit tests).

### Key Windowing & Input Invariants
- **Coordinates & mixed-DPI:** Screen origin `(0, 0)` is top-left of `NSScreen.screens[0]`, `+y`
  down. Each `NSWindow` hosts a `CAMetalLayer` returned by `macos::metal_layer_for_hwnd(hwnd)` for
  D3D12, D3D11, and Vulkan. Mixed-DPI displays propagate `WM_DPICHANGED` via `macos/display.rs` +
  `user32/{monitor,window}.rs`.
- **Keyboard & clipboard:** Non-US layouts use HIToolbox `'uchr'` / `UCKeyTranslate`
  (`macos/keyboard.rs`); clipboard uses `NSPasteboard` with test-isolated fallback.
- **Raw mouse & relative FPS mode:** `AW2_RAW_MOUSE=auto|gc|nsevent` prefers `GCMouse` with
  >500 ms silence fallback to `NSEvent`. Relative mode disassociates cursor when foreground hides
  cursor while raw mouse is registered.
- **Gamepads & DualSense HID:**
  - `xinput1_4` (aliases `xinput1_3`, `xinput9_1_0`) + `dinput8` expose `GameController` pads
    (with CoreHaptics rumble).
  - Sony DualSense/DS4 (`VID 054C`) exposed via `IOHIDManager` + `SetupDi*` (`hid.rs`,
    `setupapi.rs`) are **hidden from XInput** so games never see duplicate controllers.
  - DualSense USB audio-haptics pairing links `SPDRP_BASE_CONTAINERID` /
    `DEVPKEY_Device_ContainerId` in `setupapi.rs` with `PKEY_Device_ContainerId` (`VT_CLSID`) in
    `audio/{mmdevapi,propsys}.rs`.

## 2. WASAPI → CoreAudio (`crates/win32-rt/src/audio/`, `misc/{mmdevapi,avrt}.rs`)

- **Render + Capture + Spatial:** Shared-mode float32 (default 48 kHz stereo; AUHAL resamples),
  read-only-probed `eCapture`/`eAll` + `IAudioCaptureClient` (paced silent PCM with **zero TCC mic
  permission prompt**), `ISpatialAudioClient` (static beds + up to 128 dynamic 3D objects),
  `IAudioSessionManager[2]` / `ISimpleAudioVolume`, `ActivateAudioInterfaceAsync` (`mmdevapi` #17
  for `xaudio2_9redist.dll`), and `avrt.dll` Mach real-time thread constraints.
- **Device hotplug (`audio/devnotify.rs`):** CoreAudio listeners (`dev#`, `dOut`, `livn`) deliver
  `IMMNotificationClient` events in Windows order on a Win32 worker thread. Removed/format-changed
  endpoints invalidate active streams (`AUDCLNT_E_DEVICE_INVALIDATED`, `0x88890004`); default-device
  switches do **not** invalidate running streams (matching Windows).
- **Audio debug env vars:** `AW2_AUDIO_BACKEND=null`, `AW2_AUDIO_TAP=<out.wav>`,
  `AW2_AUDIO_DEVICE=<substr|UID>`, `AW2_AUDIO_LOW_LATENCY=0`, `AW2_AUDIO_DEVICE_EVENTS=0`,
  `AW2_AUDIO_SIMULATE_UNPLUG=<s[,replug_s]>`, `AW2_AUDIO_RT=0`, `AW2_AUDIO_TEST_DEVICES=1`.

## 3. Universal Wwise 2022.1 Runtime (`crates/wwise-audio`)

- **Game-agnostic boundary:** Consumes any `AudioDataSource`. Keep Alan Wake 2 Northlight VFS
  strictly behind `--features northlight`, and the `win32-rt` `AK::` hook bridge behind
  `--features wwise-bridge`.
- **Key subsystems:** Bank v145 parser (`src/bnk/`), `.wem` Vorbis (`FloorSynthesis::AkAvx2`
  default vs `AW2_VORBIS_FLOOR=libvorbis`), 3D spatialization + interactive music + 21 DSP plugins
  (`src/engine/`), `CAkBus::IsMixingBus` pass-through folding (`params.rs`/`render.rs`), and
  partitioned overlap-add FFT convolver decoding `00 04 02 00` BFloat16 / `00 04 00 00` Float32 IRs
  (`engine/convolver.rs`).
- **Alan Wake 2 deterministic audio oracle:** `AW2_AUDIO=original|native|both`,
  `tools/audio_harness.sh`, `tools/audio_oracle.sh`, `tools/audio_dc_{analyze,bank}.py`.

## Verification Checklist

```sh
# Windowing, input, gamepads & HID
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- \
  user32 macos misc::dinput8 misc::hid misc::setupapi misc::xinput1_4 --test-threads=1

# WASAPI audio (verified deadlock-free in parallel too, plus --features wwise-bridge)
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- audio misc::setupapi
cargo test -p win32-rt --lib --features wwise-bridge --target aarch64-apple-darwin -- audio

# Universal Wwise 2022.1 suite (+ optional Northlight real_data when AW2_GAME_DIR is set)
cargo test -p wwise-audio --lib --target aarch64-apple-darwin
# Live physical hardware opt-in tests:
# AW2_TEST_GAMEPAD=1 cargo test -p win32-rt --test gamepad_device --target aarch64-apple-darwin
# AW2_AUDIO_TEST_DEVICES=1 cargo test -p win32-rt --lib --target aarch64-apple-darwin -- audio::devnotify --test-threads=1

cargo runner-build
```

See `docs/WINDOWING.md` and `docs/AUDIO.md` for full reference.
