# Audio: WASAPI on CoreAudio & Native Wwise 2022.1 (`win32-rt`, `wwise-audio`)

Audio is split into two cleanly separated layers:

1. **Universal Windows audio (`crates/win32-rt/src/audio/`, `misc/{mmdevapi,avrt}.rs`)** — A
   CoreAudio-backed WASAPI shared-mode implementation, real-time Mach thread scheduling, and
   `mmdevapi`/`avrt` support so any Windows game using WASAPI or XAudio2 plays natively on macOS.
2. **Universal Wwise 2022.1 runtime (`crates/wwise-audio`)** — A pure-Rust reimplementation of
   Audiokinetic Wwise 2022.1 (bank format v145): `.bnk` parser, `.wem` Wwise Vorbis/PCM decoder,
   3D spatializer, interactive music engine, and 21 source/effect DSP plugins. It takes any
   `AudioDataSource` (Alan Wake 2's Northlight VFS is behind an optional `northlight` feature, and
   the `AK::` hook bridge in `win32-rt` is behind the optional `wwise-bridge` feature).

---

## 1. Universal WASAPI → CoreAudio Layer (`crates/win32-rt/src/audio/`)

### Implemented COM & DLL surface
- **WASAPI / MMDevice (`audio/{mmdevapi,audioclient,propsys,backend,ring,tap}.rs`, `misc/ole32.rs`):**
  `IMMDeviceEnumerator`, `IMMDeviceCollection`, `IMMDevice`, `IMMEndpoint`, `IPropertyStore`,
  `IAudioClient` / `2` / `3`, `IAudioRenderClient`, and `IAudioClock`.
- **Mix format:** Shared-mode float32, defaulting to 48 kHz stereo (CoreAudio AUHAL handles sample
  rate conversion and channel mapping to the physical output device). No capture endpoints yet.
- **Adaptive low-latency headroom:** Reported buffer padding includes virtual headroom so the
  steady-state queue stays near ~2,048 frames instead of filling an ~8,736-frame buffer. Silent
  excess frames are dropped and accounted for in `IAudioClock`; after an underrun the queue refills
  once. Disable with `AW2_AUDIO_LOW_LATENCY=0`.
- **XAudio2 / Bink support (`misc/mmdevapi.rs`, `audio/activate.rs`, `misc/avrt.rs`):**
  - `mmdevapi.dll!#17` (`ActivateAudioInterfaceAsync`) activates `DEVINTERFACE_AUDIO_RENDER` /
    `IAudioClient3` asynchronously on a Win32 worker thread and invokes the client's completion
    handler. This lets Microsoft's `xaudio2_9redist.dll` (used by `bink2w64.dll!BinkOpenXAudio2`)
    run cleanly on top of our WASAPI shim.
  - `avrt.dll` (`AvSetMmThreadCharacteristics*` / `AvRevertMmThreadCharacteristics`) promotes
    `"Audio"` / `"Pro Audio"` threads to Mach real-time constraints (10 ms period / 5 ms
    computation / 10 ms constraint) and `"Games"` / `"Playback"` / `"Capture"` to `HIGHEST`.

### Device hotplug & default switching (`audio/devnotify.rs`)
CoreAudio property listeners watch the device list (`dev#`), default output (`dOut`), and device
liveness (`livn`) and signal a dedicated Win32 worker thread (100 ms debounce):

- Removed or format-changed endpoints invalidate active streams (`AUDCLNT_E_DEVICE_INVALIDATED`
  `0x88890004` on all `IAudioClient` / `IAudioRenderClient` calls, waking the client event once)
  and mark stale `IMMDevice`s as `DEVICE_STATE_NOTPRESENT`.
- Registered `IMMNotificationClient`s fire in Windows order: `Added` + `StateChanged(ACTIVE)`,
  `StateChanged(NOTPRESENT)` + `Removed`, `PropertyValueChanged(DeviceFormat)`, then
  `DefaultDeviceChanged` (`eConsole` / `eMultimedia` / `eCommunications`).
- Default-device switches do not invalidate running streams (matching Windows — the app decides
  whether to rebuild its sink).
- Verified with unit tests, live CoreAudio aggregate-device hotplug tests
  (`AW2_AUDIO_TEST_DEVICES=1`), simulated unplugs (`AW2_AUDIO_SIMULATE_UNPLUG=3,3`), and external
  macOS Sound-setting switches.

### Thread scheduling (`crates/win32-rt/src/thread.rs`)
- `SetThreadPriority` maps Win32 priorities (`IDLE`..`TIME_CRITICAL`) to Mach
  `THREAD_PRECEDENCE_POLICY` importance (`-16`..`+15`). Disable with `AW2_THREAD_PRIORITY=0`.
- The first thread calling `IAudioRenderClient::GetBuffer` is promoted once to Mach
  `THREAD_TIME_CONSTRAINT_POLICY` (period = 512 frames = 10.67 ms at 48 kHz, skipped during offline
  oracle rendering; disable with `AW2_AUDIO_RT=0`).

### Audio environment knobs
| Variable | Effect |
|---|---|
| `AW2_AUDIO_BACKEND=null` | Clocked headless pump (no CoreAudio device) |
| `AW2_AUDIO_DEVICE=<substr\|UID>` | Pin a specific CoreAudio output device |
| `AW2_AUDIO_TAP=<path.wav>` | Record raw float32 PCM submitted by the client |
| `AW2_AUDIO_RATE`, `AW2_AUDIO_CHANNELS`, `AW2_AUDIO_MIN_BUFFER_FRAMES` | Override mix format / buffer floor |
| `AW2_AUDIO_LOW_LATENCY=0` | Disable adaptive headroom |
| `AW2_AUDIO_DEVICE_EVENTS=0` | Disable CoreAudio hotplug listeners |
| `AW2_AUDIO_SIMULATE_UNPLUG=<s[,replug_s]>` | Temporarily hide the default endpoint for hotplug testing |
| `AW2_AUDIO_RT=0` / `AW2_THREAD_PRIORITY=0` | Disable Mach RT / thread-priority promotion |
| `AW2_AUDIO_TEST_DEVICES=1` | Enable the live CoreAudio hotplug test (briefly switches system default) |

**Test caveat:** Always run `win32-rt` audio tests with `-- --test-threads=1` (they share
process-global CoreAudio / recorder state).

---

## 2. Universal Wwise 2022.1 Runtime (`crates/wwise-audio`)

Pure Rust, decoupled from any single game via `AudioDataSource` (enable `--features northlight`
for Alan Wake 2's `.rmdtoc` VFS integration). Codebook tables are generated at build time from
`third_party/vgmstream`.

### What's implemented
- **Bank parser (`src/bnk/`):** Full v145 parser for `BKHD`, `DIDX`, `DATA`, `HIRC` (all 22 object
  types, strict byte consumption: 0 errors across all 131,478 HIRC objects in Alan Wake 2's 5,009
  banks), `STID`, `INIT`, `STMG`, `ENVS`, and `PLAT`.
- **Media & Wwise Vorbis decoder (`src/wem.rs`, `src/codec/`):** RIFF/extensible `.wem` parser
  (`smpl` loops, `akd ` loudness gain chunks, prefetch detection), PCM16, and a forked packet-level
  Vorbis decoder (`codec::lewton`) with two floor1 modes:
  - `FloorSynthesis::AkAvx2` *(default)* — Bit-exact reproduction of Audiokinetic's AVX2/FMA
    continuous-float floor1 synthesis and 2048-entry `rcpps` table (eliminating a −31 dB residual
    against Wwise's own decoder).
  - `FloorSynthesis::Libvorbis` (`AW2_VORBIS_FLOOR=libvorbis`) — Spec-identical to libvorbis /
    `vgmstream` (verified across 52 stream classes / 63 full decodes with max `|Δ| ≤ 4.9e-7`).
- **Core engine (`src/engine/`):**
  - Event/action scheduling (all 13 action kinds, delays, fades, scopes, `SeekOnEvent`,
    `ExecuteActionOnEvent`, `EndOfEvent`/`Duration` callbacks).
  - Containers: `RanSeqCntr` (random/shuffle/avoid-repeat/sequence, all 6 transition modes),
    `SwitchCntr`, `LayerCntr`, using AK's exact LCG (`state = state * 0x5851f42d4c957f2d + 1`,
    `state >> 33`).
  - 3D spatialization (`engine/spatial.rs`): built-in params (Distance, Azimuth, Elevation, Cone,
    Obstruction, Occlusion), Attenuation Sharesets, 2D/3D VBAP stereo panner, RTPCs, States,
    Switches, Triggers, and Obstruction/Occlusion `ENVS` curves.
  - Voice management & bus graph (`engine/render.rs`, `engine/filter.rs`): volume threshold
    (`-67 dB`), priority + distance offset, `MaxNumInstance`, virtual voice behaviors, LFO /
    Envelope / Time modulators, AK Butterworth LPF/HPF biquad filter, aux sends, and HDR buses.
  - Interactive music (`engine/music.rs`): `MusicSegment`, `MusicTrack`, `MusicRanSeqCntr`,
    `MusicSwitchCntr` decision trees, clip automation, transition rules, stingers, and
    `AK_MusicSync*` callbacks.
- **DSP plugins (`src/engine/fx.rs`):** All 3 source plugins (`Silence`, `Tone Generator`,
  `SoundSeed Grain`) and all 18 effect plugins (`Gain`, `Parametric EQ`, `Compressor`, `Expander`,
  `Peak Limiter`, `Delay`, `RoomVerb`, `Matrix Reverb`, `Stereo Delay`, `Flanger`,
  `Guitar Distortion`, `Meter`, `Pitch Shifter`, `Harmonizer`, `Time Stretch`,
  `Convolution Reverb`, `McDSP FutzBox`, `iZotope Trash`, `Mastering Suite`, `QAnalyzer`) with zero
  passthrough fallbacks across all 693 AW2 bank FX instances.
- **Realtime runtime (`src/runtime.rs`):** `NativeSoundEngine` renders 512-sample 48 kHz frames
  directly into `win32-rt`'s CoreAudio output ring buffer.

### CLI & tests
```sh
cargo test -p wwise-audio --lib --target aarch64-apple-darwin
# With Alan Wake 2 VFS enabled:
cargo run --release -p wwise-audio --features northlight --bin aw2-audio -- validate --per-class 3
```

---

## 3. Alan Wake 2 Integration & Oracle Harness (`win32-rt/wwise-bridge`)

Alan Wake 2 statically links Wwise 2022.1.4 and exports 330 `AK::` symbols. `aw2-loader` supports
switching modes via `AW2_AUDIO`:

- **`AW2_AUDIO=original`** *(default)* — Runs the game's original Wwise engine through our
  CoreAudio WASAPI shim. Verified sample-exact against Wwise's built-in `StartOutputCapture`
  across sink rebuilds and device switches.
- **`AW2_AUDIO=native`** — Hooks 155 `AK::` exports (`crates/win32-rt/src/audio/akrec.rs`) into
  `wwise-audio::NativeSoundEngine` + native heap/stream shims. Zero original Wwise DSP, voice,
  music, or WASAPI threads start.
- **`AW2_AUDIO=both`** — Runs both side by side for diagnostic comparison.

### Deterministic offline oracle (`tools/audio_harness.sh`, `tools/audio_oracle.sh`)
`cargo loader -- --audio-harness` boots original Wwise with `SetOfflineRendering(true)`,
`SetOfflineRenderingFrameTime(512/48000)`, `SetRandomSeed`, and `RegisterCaptureCallback`, letting
us record real gameplay `AK::` call traces (`AW2_AK_RECORD=trace.akt`, magic `AW2AKT01`) and replay
them deterministically to byte-identical WAVs (verified 5/5 replays, 509,952/509,952 samples
exact). This is the A/B null-test oracle for `wwise-audio`.

### Honest open items in `wwise-audio`
- **A3 fine gain-staging parity:** On 212 isolated 2D events (`patch-2d` banks), 199/212 pass
  ≤ −90 dBFS in PCM-isolated mode; 11 events on filtered bus `1134869537` still have a static
  `1 / FastPow10(0)^2` gain offset, and a handful of Vorbis events show a tiny `0.99998×` decode
  scale difference vs AK's SSE split-radix IMDCT (intentionally not ported; ~−92 dB relative rms).
- **Binary plugin media formats:** Preprocessed Convolution Reverb IR media (`00 04 02 00`, 50
  embedded entries) and Wwise MIDI media (`03 c0 00 00`, 5 entries) are not reverse-engineered yet;
  Convolution Reverb currently uses a parameterized FDN approximation.
- **Fade-out handoff:** Starting a fade-out while a fade-in is still active currently starts from
  `1.0` instead of the current envelope ratio.
- **Proprietary FX parameter fidelity:** `McDSP FutzBox`, `iZotope Trash`, and `Mastering Suite`
  are clean approximations rather than bit-exact clones of the proprietary plugins.
