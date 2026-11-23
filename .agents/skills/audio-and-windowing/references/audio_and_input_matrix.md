# Windowing, Input, TSF & Multi-API Audio Stack Reference

## 1. Windowing, Display, Hooks & Input Subsystems (`crates/win32-rt`)

| Subsystem | Source Files | Key Implemented Details |
|---|---|---|
| **AppKit / UIKit host** | `macos/{appkit,uikit,display,events,keyboard,mod}.rs` | Main-thread `NSApplication` / `UIApplication`; `window_create` is the **only** `dispatch_sync`; all other mutations are FIFO `dispatch_async`. `window_set_dxgi_fullscreen` implements borderless fullscreen save/restore guarded against frame/style clobbering. `PORT_HEADLESS=1` disables AppKit cleanly. |
| **Win32 windows & hooks** | `user32/{window,message,class,defproc,monitor,cursor,input,keymap,misc}.rs` | Per-thread lazy queues integrated with kernel wait lock (`MsgWaitForMultipleObjects[Ex]`), per-monitor mixed-DPI (`WM_DPICHANGED`), `EnumDisplayMonitors`, `SetWindowsHookEx*`, PE `LoadIcon` resource loading, `NSPasteboard` clipboard, HIToolbox `'uchr'`/`UCKeyTranslate` non-US keyboard layout, `GCMouse` + `NSEvent` raw mouse & relative FPS lock. |
| **Text Services (IME/TSF)** | `misc/msctf.rs`, `misc/imm32.rs` | Windows Text Services Framework (`CLSID_TF_ThreadMgr`, document managers, contexts, sinks) + IMM32 IME bridge. |
| **Gamepads & HID** | `macos/{gamepad,hidmgr}.rs`, `misc/{xinput1_4,dinput8,gameinput,hid,setupapi}.rs` | `GameController` + CoreHaptics rumble powering XInput (`xinput1_4`/`1_3`/`9_1_0`), DirectInput 8 (`IDirectInputDevice8W/A`), and live `GameInput`. Sony DS4/DualSense (`VID 054C`) use `IOHIDManager` + real parsed HID report descriptors + overlapped `ReadFile`/`WriteFile` + `SetupDi*` and are **hidden from XInput** to prevent double controllers. USB audio-haptics pair via `SPDRP_BASE_CONTAINERID` ↔ `PKEY_Device_ContainerId` (`VT_CLSID`). |

## 2. Full Native Audio Stack (All CoreAudio / Null-Backend Powered)

| Windows Audio API | Source | Capabilities |
|---|---|---|
| **WASAPI & MMDevice** | `win32-rt/src/audio/{mmdevapi,audioclient,propsys,spatial,backend,devnotify,activate,ring,format,tap}.rs`, `misc/{mmdevapi,avrt}.rs` | Shared-mode float32 render (default 48 kHz stereo, adaptive low-latency headroom), read-only-probed `eCapture`/`eAll` + `IAudioCaptureClient` (paced silent PCM, **zero TCC mic prompt**), real `ISpatialAudioClient` (static beds + up to 128 dynamic 3D objects), `IAudioSessionManager[2]`/`ISimpleAudioVolume`, `ActivateAudioInterfaceAsync` (#17), Mach RT scheduling (`avrt` + `GetBuffer`), and `IMMNotificationClient` hotplug (`AUDCLNT_E_DEVICE_INVALIDATED` on removal/format change; default-device switch does **not** invalidate running streams). |
| **XAudio2.9 + X3DAudio + XAPO** | `win32-rt/src/misc/xaudio2_9redist.rs` | CoreAudio-backed source/submix/mastering voices, X3DAudio 3D math, real dual-interface COM `IXAPO` + `IXAPOParameters` builtin DSP (`CreateAudioReverb` 2.7/2.8/2.9, `CreateAudioVolumeMeter`, `CreateFX` `FXEQ`/`FXMasteringLimiter`/`FXReverb`/`FXEcho`), plus arbitrary guest COM `IXAPO` callbacks via `call_guest8` with locks dropped! |
| **FMOD Core & Studio** | `win32-rt/src/misc/fmod.rs` | Pure-Rust `FSB5` v0/v1 (PCM8/16/24/32/Float, IMA-ADPCM, FADPCM) & RIFF `FEV ` `.bank` parser (`strings.bank`, `BUSS`, `VCAS`, `EVNT`, `PARM`, `SND `) driving a live CoreAudio/Null mixer with 3D spatialization, buses, VCAs, parameters, and fire-and-forget event reaping. |
| **DirectSound 8 & OpenAL 1.1** | `win32-rt/src/misc/{dsound,openal32}.rs` | Real CoreAudio-backed DirectSound 8 secondary/primary mixing buffers and OpenAL 1.1 context/source/buffer playback. |
| **Bink Audio** | `win32-rt/src/misc/bink2w64.rs` | Real Bink Audio DCT/RDFT decoder & stereo playback (`BinkOpenXAudio27/28/29`, `BinkSetSpeakerVolumes`). |
| **Universal Wwise 2022.1** | `crates/wwise-audio` (`--features northlight` for AW2 VFS; `win32-rt/wwise-bridge` for `AK::` hooks) | Bank v145 parser, Wwise Vorbis (`FloorSynthesis::AkAvx2` default vs `AW2_VORBIS_FLOOR=libvorbis`), 3D spatializer, interactive music, 21 DSP plugins, exact `CAkBus::IsMixingBus` pass-through folding, and partitioned overlap-add FFT convolver decoding `00 04 02 00` BFloat16 & `00 04 00 00` Float32 IRs. |

## 3. Audio & Windowing Environment Knobs

| Variable | Effect |
|---|---|
| `PORT_HEADLESS=1` / `AW2_HEADLESS=1` | Disable AppKit; synthetic 1920×1080 display |
| `AW2_RETINA=0` / `AW2_DPI=<n>` | Force 1:1 pixel-to-point scaling / override DPI |
| `AW2_RAW_MOUSE=auto\|gc\|nsevent` | Raw mouse delta source |
| `AW2_GAMEPAD=0` / `AW2_HID=0` | Disable GameController XInput / IOHIDManager (Sony pads fall back to XInput) |
| `AW2_AUDIO_BACKEND=null` | Clocked headless audio pump (no CoreAudio output) |
| `AW2_AUDIO_TAP=<path.wav>` | Record raw float32 PCM submitted by WASAPI render client (+ `.tsv`) |
| `AW2_AUDIO_DEVICE=<substr\|UID>` | Pin specific CoreAudio output device |
| `AW2_AUDIO_LOW_LATENCY=0` / `AW2_AUDIO_RT=0` | Disable adaptive headroom / Mach RT thread promotion |
| `AW2_AUDIO_DEVICE_EVENTS=0` / `AW2_AUDIO_SIMULATE_UNPLUG=<s>[,replug]` | Disable hotplug listeners / simulate endpoint unplug |
| `AW2_AUDIO=original\|native\|both` | Switch Alan Wake 2 between original Wwise over WASAPI and native `wwise-audio` |
