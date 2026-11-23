# MSVC x64 C++ ABI, COM Vtables & `win32-rt` Middleware Reference

Battle-tested lessons learned from binary-verifying retail Windows x64 DLLs (NvCloth 1.1.6,
DirectStorage 1.2, DirectML, XAudio2/XAPO, FMOD Studio, D3DReflect, Bink2, XeSS, CLR/Mono).

## 1. MSVC x64 C++ ABI Pitfalls (Verify Against Retail Binary, Never Guess!)

1. **MSVC reverses overloaded virtual methods in vtable slot order.**
   - When a C++ class declares overloaded virtual methods (especially `const` vs non-`const`, e.g.,
     `getCurrentParticles()` and `getCurrentParticles() const`), MSVC places them in **reverse**
     declaration order in the vtable!
   - Always dump the retail PE vtable / disassembly before locking down slot indices.
2. **WinMD / Media Foundation alphabetical vtable ordering.**
   - Some WinRT/MF COM interfaces (`IMFDXGIDeviceManager`) sort methods alphabetically rather than
     in conceptual groups. Check official WinMD/Windows SDK headers.
3. **Hidden `sret` struct return convention.**
   - Any C++ return struct whose size is **not** 1, 2, 4, or 8 bytes (e.g., `PxVec3` = 12 B,
     `MappedRange` = 16 B, `DML_BINDING_PROPERTIES`) is returned via a hidden `sret` pointer:
     - Member method: `(this: rcx, sret: rdx, arg1: r8, arg2: r9, ...)` → **must return `sret` in `rax`**!
     - Free function: `(sret: rcx, arg1: rdx, ...)` → **must return `sret` in `rax`**!
4. **Mixed integer + floating-point Win64 arguments on ARM64 (`win64_bridge!`).**
   - In Win64 ABI, argument slot `k` (`0..3`) maps positionally to `(rcx/xmm0, rdx/xmm1, r8/xmm2, r9/xmm3)`.
   - Example: `void setSolverFrequency(this, float)` puts `this` in `rcx` (slot 0) and `float` in
     **`xmm1`** (slot 1 — NOT `xmm0`!).
   - On `aarch64`, `extern "C" fn(*mut T, f32)` expects `f32` in **`s0`**, NOT `s1`!
   - **Fix:** Wrap EVERY mixed int+float Win64 function in `win64_bridge!` + `bridge_entry!` (from
     `crates/win32-rt/src/macros.rs`, exposed via `EXTRA_EXPORTS` or COM vtables), OR for pure-math
     functions in `ucrt/math.rs` (`frexp`, `ldexp`, `scalbn`, `modf`), add `#[cfg(target_arch = "aarch64")]`
     dummy GPR slot parameters so `rdx` (`x1`) and `xmm0` (`s0`/`d0`) align.
5. **MSVC virtual scalar deleting destructor.**
   - Slot 0 of a polymorphic C++ class is `virtual ~Class()`, whose MSVC x64 signature is
     `unsafe extern "win64" fn(this: *mut T, flags: u32) -> *mut T` (frees memory when `(flags & 1) != 0`).
6. **Dual-interface COM & invalid sentinel objects.**
   - Multi-interface COM objects (e.g., `IXAPO` + `IXAPOParameters`, `IMFMediaBuffer` + `IMF2DBuffer2` + `IMFDXGIBuffer`)
     must preserve canonical `IUnknown` pointer identity across `QueryInterface` and share an atomic refcount.
   - Shader reflection lookup methods (`ID3D12ShaderReflection::GetConstantBufferByName`) return
     non-null **invalid sentinel objects** whose `GetDesc` returns `E_FAIL` (callers dereference the
     returned pointer without null checks!).
7. **Always release internal locks before `call_guest8`.**
   - Guest callbacks (`WNDPROC`, `ID3DInclude::Open`, guest `IXAPO::Process`, Mono internal calls,
     P/Invoke, `qsort`, `EnumWindows`) can re-enter `win32-rt`. Always drop `Mutex`/`RwLock` guards
     before calling `crate::call_guest8(...)`.
8. **Mach `SuspendThread` self-suspension rule.**
   - Never call Mach `thread_suspend` on the current thread (permanent deadlock!). Park self-suspends
     on a `Condvar` until another thread calls `ResumeThread`, and use a lifecycle guard (`mark_exiting`)
     so exiting threads never leak Mach suspend counts.

## 2. Fast-Path Dispatch Tables (`crates/win32-rt/src/lib.rs`)

JIT and static-AOT `lookup_call`/`lookup_jmp` bypass `jit_host_xfer` spills for two strictly
**disjoint** tables in `crates/win32-rt/src/lib.rs`:

- **`plain_builtin_fns()`**: Pure-integer leaf functions that **NEVER** invoke `call_guest8` or
  throw/unwind (CRT string/mem/ctype `*_s`, RTTI helpers, pointer-taking float tests).
- **`pure_float_math_fns()`**: Pure floating-point `ucrt::math` functions (any entry mixing ints
  and floats MUST have `#[cfg(target_arch = "aarch64")]` dummy GPR slot alignment).

Verify additions with:
```sh
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- plain_and_pure_math_tables_cover_crt_exports --test-threads=1
```

## 3. Native Middleware Builtins Already Implemented (`crates/win32-rt/src/misc/`)

Before writing a new shim or assuming a DLL is a stub, check the real native runtimes already built:

| Domain | Modules | Real Capability |
|---|---|---|
| **GPU Upscalers & Vendor** | `amd_fidelityfx_loader_dx12.rs`, `nvngx_dlss.rs`, `libxess.rs`, `libxess_dx11.rs`, `amd_ags_x64.rs` | MetalFX Temporal/Spatial/bilinear upscale (D3D12 & D3D11), honest refusal of FrameGen/RayRecon/DeepDVC, real Apple Silicon `AGSGPUInfo` |
| **GPU Compute / Streaming / Shaders** | `dstorage.rs`, `directml.rs`, `d3dcompiler_47.rs`, `nri.rs`, `nrd.rs`, `opencl.rs` | GDeflate v1 + D3D12 CopyQueue uploads, CPU-backed D3D12 DirectML ops, pure-Rust `D3DReflect`/`D3DPreprocess` + DXC `D3DCompile`, D3D12 NRI, 7-shader DXIL NRD, macOS OpenCL framework |
| **Video, 2D & Text** | `bink2w64.rs`, `mediadecoders_windowsdesktop.rs`, `mfplat.rs`, `dwrite.rs`, `d2d1.rs`, `msctf.rs`, `cohtml_icuuc.rs` | 80-export Bink2 + Bink Audio DCT/RDFT + `KB2v` VideoToolbox HEVC, Media Foundation buffers/samples/DXGI manager, DirectWrite + Direct2D software rasterizer, TSF (`CLSID_TF_ThreadMgr`), Apple `libicucore` |
| **Audio** | `audio/`, `xaudio2_9redist.rs`, `fmod.rs`, `dsound.rs`, `openal32.rs`, `mmdevapi.rs`, `avrt.rs` | WASAPI render/capture/`ISpatialAudioClient`, XAudio2.9 + X3DAudio + live XAPO chains, FMOD Core + Studio `.bank`/`FSB5` live CoreAudio mixer, DirectSound 8, OpenAL 1.1 |
| **Physics & Cloth** | `nvcloth_x64.rs` | Native ARM64 NvCloth 1.1.6 CPU solver (`PhysX3*` runs retail PE via ARM64 JIT) |
| **.NET / Mono CLR** | `clr/{metadata,vm,mod}.rs`, `hostfxr.rs`, `coreclr.rs`, `mscoree.rs`, `mono_2_0_bdwgc.rs` | Host `dlopen` first, seamless pure-Rust ECMA-335 CLR/CIL interpreter fallback on macOS & iOS |
| **Networking & Crypto** | `ws2_32.rs`, `secur32.rs`, `crypt32.rs`, `bcrypt.rs`, `wintrust.rs`, `winhttp.rs`, `libcurl.rs` | POSIX Winsock2, statically linked `rustls` Schannel TLS 1.2/1.3, CommonCrypto CNG AES/PBKDF2, CryptoAPI chains, PE Authenticode `WinVerifyTrust`, `rustls` WinHTTP & libcurl |
| **Input & Platform** | `xinput1_4.rs`, `dinput8.rs`, `gameinput.rs`, `hid.rs`, `setupapi.rs`, `steam_api64.rs` | `GameController` + CoreHaptics, DirectInput 8, live `GameInput`, DualSense `IOHIDManager` + USB container pairing, native arm64 `steamclient.dylib` IPC |

## 4. Rust 2024 & Multi-Target Hygiene
- Avoid `dangerous_implicit_autorefs` (`(*ptr).field.method()` → `(&mut (*ptr).field).method()`)
  and `static mut` references (`rustc 1.97+` enforces these).
- Always check `aarch64-apple-darwin`, `aarch64-apple-ios`, and `x86_64-apple-darwin` compilation
  with `-D warnings` clean on files you touch.
