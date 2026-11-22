# `dxil-metal` Developer Tools

Files in this directory are developer helpers rather than part of the Rust crate itself — with one
exception: `dxilconv/build.sh` builds the optional runtime `libdxilconv.dylib` needed to translate
legacy DirectX SM 4/5 (`SHEX`/`SHDR`) shaders into DXIL.

---

## 1. Shader Test Fixtures (`build_fixtures.sh`, `dxc_compile.cpp`)

The GPU tests (`tests/gpu_*.rs`) use tiny HLSL shaders written by us — never game shaders. Their
compiled DXIL (`tests/fixtures/*.dxil`) is committed so `cargo test` needs no HLSL compiler on the
machine. Run `build_fixtures.sh` only after editing a fixture `.hlsl`.

Compiler discovery order:
1. `$DXC` or `dxc` on `PATH` from open-source
   [microsoft/DirectXShaderCompiler](https://github.com/microsoft/DirectXShaderCompiler) (preferred;
   build from source with CMake + Ninja or install via vcpkg `directx-dxc`).
2. `$DXCOMPILER` pointing to an open-source `libdxcompiler.dylib` (driven by `dxc_compile.cpp`).
3. An open-source `libdxcompiler.dylib` in `/opt/homebrew/lib`, `/usr/local/lib`, or `~/.local`.
4. Fallback (with warning): an x86-64 `libdxcompiler.dylib` under Rosetta for regenerating fixtures
   offline only — never loaded at runtime or depended on by tests/builds.

macOS has no Microsoft DXIL validator, so fixtures are emitted unsigned with a zero hash; tests
recompute the DXBC checksum via `dxil_metal::hash::dxbc_checksum` before converting.

---

## 2. SM 4/5 Converter: `dxilconv/` (`build.sh`, `dxilconv-macos.patch`)

Apple's Metal Shader Converter reads **DXIL** (SM 6.x) only. Legacy D3D11 games and older embedded
shaders (such as BinkGPU's 23 SM 5.0 shaders) ship FXC `SHEX`/`SHDR` bytecode instead.

`crates/dxil-metal/src/dxilconv.rs` converts any DXBC container with `SHEX`/`SHDR` and no `DXIL`
into signed DXIL 1.0 using Microsoft's open-source **dxilconv** (DXC `projects/dxilconv`, NCSA
license — the same code behind `dxbc2dxil.exe`):

- Cache keys still hash the original DXBC input bytes.
- Loaded lazily via `dlopen`. Search order: `$AW2_DXILCONV`, next to the executable (or
  `../Frameworks`), `~/.local/lib`, `/opt/homebrew/lib`, `/usr/local/lib`.
- If missing, SM 4/5 compilation returns `Error::Load` and is never negative-cached, so installing
  `libdxilconv.dylib` works immediately on the next run.

Upstream DXC only builds `dxilconv` on Windows. `build.sh` clones the pinned DXC commit, applies
`dxilconv-macos.patch` (pure CMake/portability glue: `CROSS_PLATFORM_UUIDOF`, `DXNonWin.h`, and
constructor/destructor instead of `DllMain`), and installs `libdxilconv.dylib` into `~/.local/lib`:

```sh
crates/dxil-metal/tools/dxilconv/build.sh
cargo test -p dxil-metal --test dxbc_sm5
```

If you bump `DXC_COMMIT`, regenerate the patch from the patched checkout:
```sh
git -C "$SRC" add -N projects/dxilconv/include/Support/DXNonWin.h
git -C "$SRC" diff -- projects > dxilconv-macos.patch
```
