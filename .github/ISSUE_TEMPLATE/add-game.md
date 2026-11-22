---
name: New Game Compatibility Report / Target Request
about: Report bring-up results for a Windows x86-64 game you own or track missing blockers
title: "[Game] <Game Name>"
labels: game-compat
---

> **Reminder:** Never attach copyrighted game executables, DLLs, or assets, and redact any personal paths or Steam IDs before posting logs.

### Game Information
- **Game title:**
- **Store / edition:** (Steam / Epic / GOG / standalone; AppID if applicable)
- **Graphics API:** (D3D12 / D3D11 / Vulkan)
- **Engine (if known):**

### Hardware & Environment
- **Apple Silicon chip:** (e.g. M2 Pro, M3 Max, M4)
- **macOS / iOS version:**
- **Commit SHA:**

### Audit Output
Paste the output of:
```sh
cargo opsdb -- win32-audit /path/to/Game
cargo opsdb -- anticheat-audit /path/to/Game/Game.exe
```

### Runtime Result & Blockers
- What happens when launching via `cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game`?
- List any `UNIMPLEMENTED dll!Export`, unimplemented x86-64 opcodes, shader errors, or crashes observed.
