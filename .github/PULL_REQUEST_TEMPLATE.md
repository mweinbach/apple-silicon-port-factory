## Summary

<!-- Describe what this PR changes and why. -->

## Category
- [ ] New game target (`games/<id>/game.toml`)
- [ ] Win32 / CRT / COM / middleware API (`crates/win32-rt`)
- [ ] x86-64 → ARM64 translation / JIT / AOT (`crates/x86-runtime`)
- [ ] Graphics / Shaders (`crates/d3d12-metal`, `crates/dxil-metal`, D3D11, Vulkan)
- [ ] Audio / Windowing / Input / iOS host
- [ ] Factory CLI / Diagnostics / Packaging (`crates/port-factory`, `crates/port-loader`)
- [ ] Documentation / Tooling

## Verification

<!-- Report the exact commands you ran and their real outcomes (plus macOS version & Apple Silicon chip if hardware-tested). -->

- [ ] `cargo runner-build` passes with zero new warnings
- [ ] `cargo factory-test -- --test-threads=1` passes
- [ ] `cargo opsdb -- doctor` exits 0 (`cargo opsdb -- win32-sync` run if `win32-rt` exports changed)

## Checklist

- [ ] No copyrighted game binaries, assets, screenshots, or personal dumps included
- [ ] No hardcoded personal paths (e.g. `/Users/<name>`, personal `/Volumes/...`)
- [ ] Universal crates under `crates/` remain game-agnostic (no hardcoded game names or VAs)
- [ ] No anti-cheat or DRM circumvention
- [ ] Unimplemented APIs fail honestly via `stub!` / documented Windows error codes
