# ADVICE: C++/SEH exceptions crossing Rust replacement frames

Status: advisory memo (design input, not yet decided). Scope: Phase A, the hybrid
loader under Rosetta, and what carries over to Phase B (native arm64).
Evidence is marked **[V]** (verified in the tree or the exe) or **[A]** (assumed or inferred).
Scratch scripts used for the numbers are `brain/.../scratch/eh.py`, `eh2.py` and `reach.py`.
They parse `.pdata`/`.xdata` and FH4 FuncInfo directly from
`$AW2_GAME_DIR/AlanWake2.exe` and query `db/functions.sqlite`.

---

## 0. Recommendation (TL;DR)

1. **Keep policy (b) as the default, but enforce it with tooling instead of judgement.**
   Add `fndb can-throw <va>` (the reachability below) and a loader startup lint. Any
   `REPLACEMENTS` entry whose original can reach a throw needs an explicit
   `throws:` annotation.
2. **Add "fence frames" (a slimmed-down option a, ~1–2 days).**
   - The hook shim saves *all* Win64 nonvolatiles and gets a real `RUNTIME_FUNCTION`
     (via the existing `RtlAddFunctionTable`).
   - When the unwinder's `step()` meets a pc in host (Rust) code, it skips straight to the
     innermost shim frame by walking the Rust frame-pointer chain.
   - Exceptions thrown below a replacement then reach the catch above it. The Rust frames are
     discarded like a `longjmp`. That is sound as long as they are "plain old frames" (no live
     `Drop` values across throwing calls, see §4).
   - No DWARF translation and no libunwind needed.
3. **Add a catching call thunk, `exe::try_call` (~2–3 days), for porting the 2,947 functions
   that themselves contain `try/catch`.**
   - It converts an MSVC C++ exception from an original callee into `Err(CxxException)` at
     the Rust call site.
   - Rust can then run its own cleanup, handle the error, or rethrow via `_CxxThrowException`.
   - **No option, including (c), solves this case:** Rust cannot catch foreign exceptions
     (`catch_unwind` aborts on them). So this thunk is needed whatever else we choose.
4. **Reject (c), building aw2-game as a PE DLL.** It costs 1–2 weeks plus a permanent tax on
   the toolchain, std, lldb and `cargo test`. The only thing it adds over fences is running
   Rust destructors during foreign unwinds, and only with `panic=unwind` + `win64-unwind`.
   §4's rule makes that unnecessary. It still needs (3). It is pure Phase A scaffolding that
   Phase B throws away.
5. **Keep `panic = "abort"`** in both phases.

---

## 1. How the current dispatcher and unwinder work [V]

- **Hook path** (`crates/aw2-loader/src/hooks.rs`): the original entry gets `jmp rel32` →
  near thunk → `jmp [rip]` → `preserving_shim` (`hooks.rs:156`).
  - The shim does `sub rsp,0xF8` with no pushes. It saves rcx/rdx/r8–r11 at +0x60 and
    xmm0–5 at +0x90, copies stack args 5–12 to +0x20, does `mov rax,imm64; call rax`, then
    restores and does `add rsp,0xF8; ret`.
  - The shim has **no** `RUNTIME_FUNCTION` today.
- **Dispatch** (`crates/aw2-win32/src/unwind.rs`):
  - `dispatch_exception` (`:757`) runs the VEH list, then walks frames with
    `step(UNW_FLAG_EHANDLER)`.
  - `step` (`:640`) looks up `lookup_function_entry(pc)` (`:321`). That checks `DYN_TABLES`
    first (from `RtlAddFunctionTable`, `:297`) and then the loader's PE `.pdata`.
  - If the pc has no entry, it is treated as a leaf *only if* `in_pe_image(pc)` (`:356`).
    Otherwise it returns `None`, and the walk breaks into `fatal_unhandled` (`:835`).
    `fatal_unhandled` logs, calls the game's unhandled filter, then `exit_process`.
- **Unwind** (`do_unwind` `:953`, the second pass of `RtlUnwindEx`): uses the same `step`.
  A `None` result logs "walked out of Windows code" and exits with `0xC0000027`.
- **Language handlers:** these are the real `__CxxFrameHandler4`/`__C_specific_handler` from
  the bundled vcruntime. They call back into our `RtlUnwindEx`/`RtlVirtualUnwind`. Hardware
  faults enter through `dispatch_hardware_exception` (`:1071`) from `aw2-loader/src/signals.rs`.
- **Net effect:** the walk hits a return address inside Rust code (the replacement that called
  the throwing original callee). It returns `None` there, and every exception that would cross
  a replacement becomes fatal.
- **What makes the fix cheap:** `lookup_function_entry` already prefers dynamic tables over the
  PE check. So a registered shim is unwindable *today*. The only missing pieces are the
  Rust-region skip and full nonvolatile restore.

## 2. How much the game relies on EH

All figures below were measured from `.pdata`/`.xdata` and FH4 FuncInfo, then cross-checked
with fndb.

| What | Count |
|---|---|
| Functions in fndb | 244,796 |
| `__CxxFrameHandler4` (thunk `0x14183ac80`) | **40,633** fns: 8,925 EH\|UH, 31,708 UH-only |
| ...of which have a **try-block map** (real catch) | **2,947** fns, 2,991 try blocks, 3,005 catch handlers |
| ...cleanup-only (unwind map, no try) | 36,504 (4,796 of these still set `UNW_FLAG_EHANDLER`, so the flag is *not* a try indicator; the FuncInfo is) |
| ...empty FuncInfo | 1,112 |
| ...`noexcept`-flagged FuncInfo | 5,911 |
| `__C_specific_handler` (`0x14183aca4`) | 22 fns: 6 `__except`, 17 `__finally` scopes |
| `__CxxFrameHandler3` (`0x14183ace6`) | 1 |
| GS-check handlers `0x1418353f0`/`0x141835344`/`0x1418356f4` | 136/52/7 (stack cookies only, not EH) |

**Catch types (3,005 handlers):**

| Caught type | Handlers |
|---|---|
| `r::ObjectStreamException_ErrorsOccurred*` | **1,987** |
| `catch(...)` | 895 |
| `r::FileException_FileOpenFailed*` | 45 |
| `r::Exception*` | 34 |
| `r::FileException*` | 21 |
| Other (Concurrency `task_canceled`/`_Interruption_exception`, `std::exception`, Luau `ParseError`/`CompileError`, `lua_exception`, `r::JSONException`, a few `FileException_*`) | ~20 |

Nearly all typed catches are **by pointer**, so their exception objects are trivially copyable.
That matters for the thunk in §3.3.

**Throw sites:**
- `j__CxxThrowException` (`0x14183acce`) has 327 call sites.
- The dominant thrower is **`0x1413415d0`**. It builds a *thread-local*
  `ObjectStreamException_ErrorsOccurred` (TLS +0xa3478) and throws it with ThrowInfo
  `0x1437fdd10`. It has **1,324 callers**, mostly in Remedy serialization code: content 389,
  ui 54, r 34, rend 27, physics 18, plus 635 not yet attributed.
- msvcp140 throw helpers: `_Xout_of_range`, `_Xlength_error`, `_Xbad_alloc`,
  `_Xbad_function_call`, `_Xinvalid_argument`, `_Xregex_error`, `_Throw_Cpp_error`.
- `RaiseException` ×2 (PhysX) and `__ExceptionPtrRethrow` ×1.

**Reachability:** **52,586 of 244,796 functions (21%)** can reach a throw through *direct*
calls alone (Remedy: 46,281 of 209,848). This is a lower bound, because vcalls are excluded and
serialization goes through vtables.

**Current registry:**
- `REPLACEMENTS` has 39 unique VAs.
- Four originals carry FH4 cleanup: `0x142409850`, `0x142409d70`, `0x14240a160` (UH) and
  `0x14240ab10` (EH\|UH). None of them has a try map.
- Two reach a throw: `scratch_alloc` `0x14241c0a0` and `scratch_free` `0x14241c190`, via
  `0x1424b3910`/`0x1424b3cb0` → `std::_Xbad_function_call`.
  `ns/r/scratch.rs` already accepts this by judgement ("contract violation").
- So policy (b) holds today. It will not hold once decomp moves past leaf utilities into
  `content::`/`ui::`, where ObjectStream reads throw.

**Reading [A]:** EH in AW2 is mainly *one* mechanism: the ObjectStream deserializer reports
errors by throwing a pointer, and loaders catch it. That is a well-defined error channel. In
Rust it should become `Result<_, StreamError>`, not "exceptions through Rust". Whether it
fires during normal loads is unknown. **Measure it first** (§5 step 0).

## 3. Options

### (b) Policy only: never hook functions whose callees can throw

- **Phase A:**
  - Zero runtime cost, and it is what we do now.
  - It blocks roughly 21%+ of Remedy functions, including almost every
    `content::*::deserialize`, and every function with a try map can never be ported.
  - "Can throw" is checked by eye today. One missed vcall path means a fatal crash, and only
    on the error path, so it will not show up in diff tests.
- **Phase B:** irrelevant.
- **Verdict:** keep it as the *default*, but enforce it mechanically. On its own it is not
  sufficient.

### (a) Synthetic unwind info for shims plus a hybrid unwinder

**Full version (translating DWARF/compact unwind to Win64 UNWIND_INFO): don't do it.**

LLVM's Darwin prologues for `extern "win64"` functions are
`push rbp; mov rbp,rsp; push r15..rbx(7); sub rsp,N; movaps xmm6-15,[rbp-..]`. I verified this
in `scratch_free` in `target/x86_64-apple-darwin/debug/aw2-loader`.

Win64 requires the frame pointer to be set *after* allocation, at an offset of at most 240. So
`rsp = rbp - (56+N)` cannot be expressed in general, and an
`RtlInstallFunctionTableCallback` translator would have to fake it per pc. This approach is
fragile and buys nothing over fences.

**Fence frames (recommended):**

1. Extend `preserving_shim` to also save rbx, rbp, rsi, rdi, r12–r15 (`UWOP_SAVE_NONVOL`) and
   xmm6–15 (`UWOP_SAVE_XMM128`).
   - The frame grows to about 0x1F8 (`UWOP_ALLOC_LARGE`).
   - Register one `RUNTIME_FUNCTION` per shim with `RtlAddFunctionTable`, using the shim pool
     base as `BaseAddress` (the pool must sit within 4 GB of that base).
   - This makes the shim a nonvolatile *fence*: unwinding through it restores exactly what
     the original caller had. It does not matter what the Rust frames below did.
     (libunwind would not restore xmm6–15 on x86-64 Darwin anyway [A], so the fence is the
     right place for it.)
2. In `step()`, when `!in_pe_image(pc)` and the pc is in the loader's own `__TEXT`:
   - Follow the Rust rbp chain from `ctx.rbp`. That value is valid because Win64 callees
     preserve rbp and the PE unwind restored it.
   - At each level read `ret = [rbp+8]`. If `ret` is inside the shim pool's post-`call rax`
     range, set `rip = ret, rsp = rbp+16` and continue. The next `step` unwinds the shim from
     its UNWIND_INFO.
   - Otherwise set `rbp = [rbp]`.
   - Bound-check each step against TEB stack limits and require strictly increasing values.
   - If there is no fence, keep today's fatal path, but print a Rust-aware backtrace.
   - Needs `-C force-frame-pointers=yes` for `x86_64-apple-darwin` in `.cargo/config.toml`.
     Debug builds already keep rbp [V]. Release on Apple x86-64 probably does too [A].
   - Alternative that does not depend on the compiler: a per-thread fence stack in a TEB
     slot. It needs lazy pruning, because frames abandoned by an unwind never pop. That is
     more assembly, so use the rbp chain first.
3. Both passes, dispatch and `do_unwind`, get the skip for free because they share `step()`.
   - Nested replacements (Rust → `exe::func` → hook → shim → Rust) just mean several fences
     in a row.
   - Hardware faults in original code called from Rust, caught by one of the 6 game
     `__except` blocks above, also start working.
   - Faults with the **pc inside Rust** stay fatal on purpose.
4. Log `warn!` once per replacement: "exception unwound through Rust replacement X (frames
   discarded)".
5. Cost:
   - About 20 extra stores per hooked call (a few ns under Rosetta [A]).
   - Optional: emit the "fat" shim only for replacements flagged by `can-throw`, and keep the
     current shim for hot leaves like `scratch_free` (2,484 callers).
- **Phase B:** deleted along with the hooks.

### (c) aw2-game as a real Windows PE DLL (`x86_64-pc-windows-msvc`, lld-link)

**What it gives you:**
- LLVM emits `.pdata` for every Rust function, so the existing dispatcher walks through Rust
  frames unchanged. This is the same "discard" semantics as fences under `panic=abort`.
- With `panic=unwind` + `extern "win64-unwind"`, Rust's personality is
  `__CxxFrameHandler3`, so Rust **drops run** during a foreign MSVC unwind. That is the only
  real semantic gain.

**What it does not give you:**
- **Catching.** Rust cannot catch a foreign exception (`catch_unwind` → abort), so the
  2,947 try-functions still need §3.3.
- **Phase B relevance.** Today the crate is `#![cfg(target_arch = "x86_64")]` and empty
  elsewhere [V]. Phase B needs it on `aarch64-apple-darwin` regardless.

**Costs [A, mostly standard knowledge]:**
- **Toolchain:** the msvc target needs Windows SDK/CRT import libs (`xwin`, EULA, several
  hundred MB) and `rust-lld`. gnullvm needs llvm-mingw, and its SEH personality
  (`__gxx_personality_seh0`) is more exotic to support in our dispatcher.
- **std imports:** std-for-Windows pulls in kernel32/ntdll/bcryptprimitives/synch APIs the game
  never uses (`ProcessPrng`, `WaitOnAddress`, `NtCreateFile`, `SetThreadDescription`, ...). We
  would have to implement them all in aw2-win32, or go `no_std`, which hurts decomp ergonomics.
- **Boundary:** `log` would need a forwarding import to the host logger. The difftest harness
  and the registry would cross a PE import boundary. `exe::set_base` and the rest of the glue
  would become exports and imports. The good news: aw2-game depends only on `log` today [V],
  so this part is contained.
- **lldb under Rosetta:** lldb does not know the PE exists. You would need
  `target modules add aw2_game.dll` + `target modules load --file … --slide <base>` after
  every run, via a loader-emitted command file. PDB support through lldb's NativePDB on
  macOS is passable for breakpoints and lines, and poor for Rust variables. Host Rust
  backtraces, `RUST_BACKTRACE` and `symbolize()` would lose these frames.
  Today aw2-game is ordinary Mach-O code inside `aw2-loader`, so everything just works.
- **Workflow:** `cargo hybrid-test` for aw2-game would need a custom runner (our loader) to
  run Windows test binaries. Builds become two targets and two `target/` trees. Each decomp
  iteration gets a link step through lld-link.
- **Effort:** 1–2 weeks to stand up, plus permanent friction. The benefit exists only in
  Phase A.

### 3.3 Catching thunk: `exe::try_call` (needed under every option)

`exe::try_call::<F, R>(va, args…) -> Result<R, CxxException>`:

- **Thunk:** a small loader-generated thunk with `.pdata` registered through
  `RtlAddFunctionTable`. Its language handler is a Rust `extern "win64"` fn.
- **Phase 1:** the handler accepts code `0xE06D7363` with magic `0x19930520..22`, and matches
  the requested type names against `ThrowInfo->pCatchableTypeArray` TypeDescriptor names
  (e.g. `.PEAVObjectStreamException_ErrorsOccurred@r@@`, or "any").
- **On a match:**
  - Copy the exception object. For pointers that is 8 bytes. For class types, use
    `CatchableType::copyFunction` and then the original's `pmfnUnwind`.
  - Copy the ThrowInfo pointer.
  - Call `RtlUnwindEx(establisher, landing_ip, rec, 1)`.
  - The landing pad returns `Err` to Rust, as an ordinary return, so Rust drops run normally.
- **Rethrow** (the `catch(...) { cleanup; throw; }` pattern): call
  `_CxxThrowException(&copy, throwinfo)` again from Rust. That is legal because the
  replacement is behind a fence.
- **Caveat:** semantics differ slightly from MSVC. Our phase 1 claims the exception at the
  call site instead of finding the outermost catch first, so the unhandled-filter or
  terminate timing differs only for exceptions nothing catches. That is acceptable.
- **Phase B:** `try_call` is replaced by native `Result` returns from the ported callee.
  Port the error channel, not the mechanism.

## 4. `panic = "abort"` implications [V: workspace `Cargo.toml`, both profiles]

- **Good:** a Rust panic can never unwind into PE frames. Keep it. With `panic=unwind`, panics
  through `extern "win64"` would abort at the boundary anyway on modern rustc.
- **Rust frames have no landing pads.** Discarding them during a foreign unwind (fences, or
  (c) without `win64-unwind`) is the Rust "forced unwind over plain old frames" case.
  - It is well-defined **only if no frame being discarded has live `Drop` values or borrows**:
    `Vec`, `String`, `Box`, `MutexGuard`, `RefCell` borrows, and so on.
  - **Rule for replacements:** across any call to an original that can throw (`exe::func`
    whose target is in the `can-throw` set), hold only raw pointers or integers. Otherwise use
    `try_call`.
  - Decompiled code is overwhelmingly raw-pointer style already [V, `ns/r/scratch.rs`], so
    this costs little.
  - A *host* lock leaked this way means a deadlock. Leaked game memory just means a leak.
- **Game-side cleanup:** if the original function had an FH4 unwind map (36.5k do), its C++
  locals, for example an `r::String` or a scoped game lock, got destructors on the error path.
  A Rust port that calls those dtors explicitly at the end will skip them when an exception is
  discarded through it.
  - That is a leak, and possibly a deadlock for game locks.
  - So the policy is: replacements of functions with **non-trivial unwind maps whose callees
    can throw** must use `try_call` around those callees and run the cleanup on `Err`, then
    rethrow. The lint in §0.1 should flag exactly this combination: `handler == FH4` and
    `can-throw`.
- **Phase B:** `panic=abort` stays. Errors are `Result`. Add a panic hook that logs and
  crash-dumps.

## 5. Migration path

0. **Measure (½ day).**
   - In `dispatch_exception`, when the code is `0xE06D7363`, log the thrown type (from
     ThrowInfo) and the frame that catches it.
   - Run a new game and a save load. This shows whether ObjectStream throws are a hot path
     or error-only.
   - Treat this as a gate for steps 3 and 4.
1. **Tooling (½–1 day).**
   - Add `fndb can-throw <va>`: transitive closure over `calls` plus vftable-slot
     over-approximation, seeded by `0x14183acce`, the msvcp `_X*`/`_Throw_Cpp_error` imports,
     `RaiseException` and `__ExceptionPtrRethrow`. Cache it as a column.
   - Add a `throws: Throws::{No, Discard, Caught}` field to `Replacement`.
   - At startup the loader cross-checks it against the cached column and the FH4 try-map /
     unwind-map presence. It warns in dev builds and fails in `--diff-test`.
   - Mark `scratch_alloc`/`free` as `Discard` (contract-violation throw).
2. **Fence frames (1–2 days).**
   - Fat shim with unwind info, dynamic table registration, Rust-region skip in `step()`,
     and `force-frame-pointers`.
   - Tests in `cargo hybrid-test`: a hand-assembled PE-style blob with `.pdata` and an
     `__except`-style handler calls the shim → Rust → a blob that raises. Assert that the
     catch is reached, that nonvolatiles including xmm6–15 match, and that a missing fence
     stays fatal.
3. **`exe::try_call` (2–3 days).**
   - Thunk, handler and type matching. Support pointer and trivially-copyable types first,
     which covers more than 2,000 of 3,005 handlers. Then `catch(...)` + rethrow.
   - Diff-test it against an original try-function, e.g. a small `content::` deserializer
     that catches `ObjectStreamException_ErrorsOccurred*`.
4. **Skill update.** Update `.agents/skills/decompile-function/SKILL.md` "Hook safety" and
   `AGENTS.md` §10:
   - **No FH4 handler and not `can-throw`:** hook freely.
   - **`can-throw`, no live drops:** fat shim (Discard).
   - **Try map, or unwind map plus `can-throw`:** `try_call` and an explicit error path
     (Caught).
5. **Phase B.**
   - Model `ObjectStreamException_ErrorsOccurred` as `Result<_, StreamError>` on ported
     ObjectStream readers. Other exception types map to Rust error enums.
   - Fences, shims, `try_call` and `unwind.rs` hooks all disappear with the exe. Nothing
     from (c) would have survived.
   - Separately, remove `#![cfg(target_arch = "x86_64")]` from aw2-game: port logic into
     arch-neutral modules and keep the `extern "win64"` + `exe::` glue in an x86_64-only
     submodule.

## 6. Risks and open questions

- **[A] Release frame pointers.** Release codegen on `x86_64-apple-darwin` might omit frame
  pointers without the flag. The forced flag removes the doubt.
- **Collided unwinds.** An exception raised inside a game catch funclet while a fence is on
  the stack is already handled generically by `EXCEPTION_COLLIDED_UNWIND_DISP` (`:803`). Test
  one nested throw.
- **Game stack walkers.** `RtlLookupFunctionEntry` and `RtlVirtualUnwind` called by game code
  directly, for crash reporting or `RtlCaptureStackBackTrace`, still stop at Rust frames,
  because there is no `RUNTIME_FUNCTION` for Rust code. This only affects diagnostics. Do not
  fix it with a DWARF translator.
- **Callbacks.** Rust aw2-win32 APIs that call back into game code (`qsort`/`bsearch`
  comparators if implemented in Rust, thread start routines, WndProc) have the same problem
  without a shim. If one ever matters, give it the same fence: an assembly thunk with unwind
  info.
