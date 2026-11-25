# What's next

## Take a second offline game past menus

Pick a legally owned single-player x86-64 Windows title that works offline and has no
anti-tamper or kernel anti-cheat requirement. Overwatch 2 reached its login screen. The
next milestone requires interactive gameplay in both the universal runner and a packaged
app, with all per-game settings in `games/<id>/game.toml`.

1. Run `cargo opsdb -- audit-game /path/to/game --json` on the install. Check the main
   executable and its shipped DLLs. Review unresolved imports, stubbed APIs and instruction
   coverage before choosing the first guest test.
2. Prewarm the manifest and boot with `--timeout-secs`, `--max-frames` and `--report-json`.
   A pre-present stall report gives the last heartbeat and thread stacks. For a fatal
   exception, inspect the crash JSON and the guest frames unwound from `.pdata`.
3. Reproduce each blocker with a redistributable guest fixture. Verify the fix on the live
   JIT and a prewarmed AOT run. Compare boot reports with `cargo opsdb -- report-diff`.
4. Enter gameplay, exercise input and audio, then save, quit and resume. Record the game
   build, manifest and observed result in [Status](STATUS.md).

## Package and relaunch

Run `cargo opsdb -- steam-port <AppID>` for the selected Steam title, verify the resulting
bundle with `bundle-verify`, then launch it cold from Finder and repeat the gameplay check.
The signed synthetic bundle and headless/windowed graphics fixtures exercise separate parts
of this path.

## Remaining hardware checks

- Exercise D3D11 and Vulkan on shipping titles, and Metal barriers and residency on a second
  D3D12 engine with the validation layer enabled.
- Complete Return 0 and Chapter 1 in Alan Wake 2, including resize, audio, input and save/load.
- Verify offline SPIR-V/WGSL compilation with the Metal toolchain on the target machine.

[Engineering backlog](TODOS.md) · [Prerequisites](WHATS_NEEDED.md) · [End goal](END_GOAL.md)
