# Runtime Diagnostic Environment Knobs & `PORT_CONTROL_DIR` Reference

## 1. Execution-Tier & Codegen Isolation Knobs (`x86-runtime`)

| Env Var | Effect | When to Use |
|---|---|---|
| `PORT_AOT=0` | Disable `.aot` disk cache (100% Live JIT) | Test whether a bug is AOT-only (check logs for `no cache for this build/knobs` to confirm!) |
| `PORT_AOT_VERIFY=1` | Re-translate & compare `.aot` blocks word-for-word on load | Catch AOT encoding, relocation, or link-pass mismatches |
| `PORT_AOT_NO_LINK=1` / `PORT_AOT_NO_COND_PATCH=1` | Disable AOT direct branch / conditional branch patching | Isolate AOT cross-block linking bugs |
| `PORT_AOT_SKIP=mod:lo..hi` | Exclude RVA span(s) from AOT (falls back to Live JIT) | Binary-search guilty AOT basic block |
| `PORT_JIT_SKIP=mod:lo..hi` / `PORT_JIT_ALLOW=...` | Force RVA span(s) onto interpreter | Binary-search guilty JIT basic block (`bisect-runtime`) |
| `PORT_JIT_FAST_NAN=0` | Exact IEEE-754 x86-64 NaN propagation & sign-bit parity | Isolate NaN payload/sign divergences (`x86-verify` default) |
| `PORT_JIT_LSE2_TRAP=0` | Disable LSE2 alignment-fault self-patching (`SIGBUS` VEH) | Isolate unaligned atomic/TSO trap patching bugs |
| `PORT_JIT_STACK_TSO=1` | Enforce TSO ordering on stack-slot loads/stores | Isolate multi-thread stack visibility races |
| `PORT_JIT_TSO=0` | Disable TSO `ldapr`/`stlr` barriers | Measure TSO overhead or check barrier interaction |
| `PORT_JIT_XMM_PIN=0` / `AW2_JIT_XMMREG=0` | Disable XMM register pinning / XMM cache | Isolate XMM register allocator / spill bugs |
| `AW2_JIT_FLAGLAZY=0` | Disable cross-block lazy flag liveness (`succ_need`) | Isolate lazy-flag or obfuscated dead-fallthrough bugs |
| `PORT_JIT_LAST_BITS=0` | Disable `Cpu::last_bits` optimization | Isolate partial-register / ABI register bugs |
| `AW2_JIT_RAS=0` | Disable shadow return-address stack | Isolate return-prediction / mispredicted `ret` bugs |
| `AW2_JIT_RO_PLAIN=0` / `PORT_JIT_HW_RANGE=0` | Disable RO plain loads / HW range-check folding | Isolate memory range-check optimizations |
| `AW2_JIT_PLAINCALL=0 AW2_JIT_HOSTCALL=0` | Bypass **all** inline JIT host calls (`plain_call` & `host_xfer`) | Single zero-rebuild test to rule out Win64↔ARM64 host-call bridges! |
| `AW2_JIT=0 PORT_AOT=0` | Disable JIT & AOT (runs `FastOp` interpreter) | Separate JIT bugs from interpreter/Win32 bugs |
| `AW2_JIT=0 AW2_EMU_FAST=0 PORT_AOT=0` | Pure Generic `iced-x86` interpreter | Separate `FastOp` bugs from Generic/MXCSR/Win32 bugs |
| `AW2_EMU_VERIFY=1` | Shadow-check every `FastOp` against Generic live | Catch live `FastOp` vs Generic divergences |

## 2. Diagnostic Decision Tree: When `x86-verify` Says 0 Mismatches!

If a bug:
1. Reproduces under **100% Live JIT** (`PORT_AOT=0`, confirmed 0 `.aot` caches loaded),
2. Reproduces under `PORT_JIT_FAST_NAN=0`, `PORT_JIT_LSE2_TRAP=0`, `PORT_JIT_STACK_TSO=1`, AND
3. Passes offline single-threaded `x86-verify` with 0 mismatches,

then **JIT and the Rust reference interpreter agree with each other while diverging from real
Windows x86-64 hardware**, or the bug is in host-call/Win32/multi-thread code! Run these 3 sharp
discriminators immediately:
1. **Check `MXCSR` / `FPCR` parity** (`ldmxcsr` `0x9FC0` FTZ|DAZ vs `0x1F80`) via `runtime-audit` +
   `oracle --mxcsr 0x9fc0` vs `--mxcsr 0x1f80`.
2. **Capture exact first `NaN`-birth `RIP`** via `PORT_NAN_TRACE` armed right before the repro
   moment using `PORT_CONTROL_DIR` (`nantrace on`) so boot runs at full speed!
3. **Eliminate host-call bridges in one run** via `AW2_JIT_PLAINCALL=0 AW2_JIT_HOSTCALL=0`.

## 3. `PORT_NAN_TRACE` & `PORT_NAN_BOUNDARY` Spec

- `PORT_NAN_BOUNDARY=1`: logs `[diag-boundary]` + guest stack trace whenever non-finite floats
  cross instrumented `win32-rt` middleware boundaries (e.g., `NvCloth::setSpheres`).
- `PORT_NAN_TRACE="<k=v;...>"` (zero overhead when unset):
  - `module=a.exe,b.dll` — modules to trace
  - `range=mod:lo..hi` (or `ranges=mod1:lo..hi,mod2:lo..hi`) — RVA ranges forced through tracer
  - `watch=lo..hi` — memory watchpoint address range
  - `host=sym` — trace calls to a host symbol
  - `dump_dir=DIR` — write pre/post CPU + `MXCSR` `snap_*.json` files for `cargo opsdb -- oracle`
  - `log=FILE` — JSONL trace log
  - `stores=1` — also check floating-point memory stores for NaNs
  - `max=N` — stop dumping after `N` events
  - `armed=0|1` (or `on=0|1`) — start disarmed (`0`) for fast boot, arm mid-run via `nantrace on`!

## 4. `PORT_CONTROL_DIR` Command Grammar (`crates/port-loader/src/live_ctl.rs`)

Set `PORT_CONTROL_DIR=/path/to/dir`; write commands atomically via `cmd.tmp` → rename to `cmd.txt`
(polled on every `Present`; reads `<dir>/status.json` for frame count / elapsed time):

| Command | Action |
|---|---|
| `shot <path.png>` | Capture next presented Metal 4 back buffer atomically to PNG |
| `key <name> <frames>` | Hold Win32 key `<name>` for `<frames>` frames (default 8) |
| `mouse <dx> <dy> <frames>` | Inject smooth relative raw-mouse deltas across `<frames>` frames |
| `moveto <x> <y>` | Move cursor to client pixel `(x, y)` |
| `click <left\|right\|middle> <frames>` | Press & hold mouse button |
| `wheel <delta> [frames]` | Send vertical `WM_MOUSEWHEEL` (`120` per notch) |
| `pad <lx> <ly> <rx> <ry> <lt> <rt> <buttons_csv\|none> <frames>` | Hold XInput gamepad state (e.g. `pad 0 32000 0 0 0 0 lthumb 120`) |
| `nantrace on\|off` | Arm or disarm `PORT_NAN_TRACE` dynamically mid-run |
| `release` | Release all held keys, mouse buttons, and gamepad state |
| `quit` | Terminate guest cleanly (`ExitProcess(0)`) |
