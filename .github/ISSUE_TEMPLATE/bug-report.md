---
name: Bug Report
about: Report a bug in x86-runtime, win32-rt, d3d12-metal, dxil-metal, port-loader, or port-factory
title: "[Bug] "
labels: bug
---

> **Reminder:** Never attach copyrighted game files or assets, and scrub personal paths (`/Users/<name>`) from logs.

### Component
- [ ] `x86-runtime` (decoder / JIT / AOT / SEH)
- [ ] `win32-rt` (Win32 / CRT / D3D11 / Vulkan / audio / input)
- [ ] `d3d12-metal` / `dxil-metal` (Metal 4 / MetalFX / shaders)
- [ ] `port-loader` / `port-factory` / `ios/PortHost`
- [ ] `games/alan-wake-2` / `games/overwatch`

### Environment
- **Apple Silicon chip & macOS/iOS version:**
- **Commit SHA:**

### Reproduction Steps & Expected vs. Actual Behavior

### Logs / Diagnostics
(Output from `RUST_LOG=info,stub=warn`, `cargo opsdb -- verify-x86`, or `MTL_DEBUG_LAYER=1` if relevant.)
