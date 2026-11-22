# Full-Image Guest DLL Lift Roadmap (`g28` / `g29`)

**Scope:** Drop `--rva-list` and `--dataref-listed` globally across `cohtml.WindowsDesktop.dll` and `v8.dll` so all **8 of 8** guest PE DLLs are lifted across their entire `.text` image, plus close every remaining non-context-capture rejection across all 8 DLLs.

Reproducible standalone dry-run tool (writes only to `/tmp/aw2_dll_full_audit` with `AW2_DLL_PROGRESS=0` and auto-cleans on exit):
```sh
python3 tools/decomp_scale/audit_full_dlls.py
```

---

## 1. Executive Summary & Full-Image Baseline (Zero Generator Changes)

Without changing a single line of `gen_subsystems.py` or `dll_lift.py`, removing `--rva-list` and `--dataref-listed` today lifts **`99.93%`** of `cohtml.WindowsDesktop.dll` (`11,390 / 11,398`) and **`99.98%`** of Win64 C++ entries in `v8.dll` (`88,308 / 88,324` Win64 candidates, plus `1,616 / 1,643` `DF_REGFILE` builtins in `g27`):

| DLL (`--dataref-leaves`, no `--rva-list`) | `.pdata` Roots | Leaves (`internal`) | Win64 Lifted | `E9` Aliases | `tpl_*` Shared (`tpl` fns) | Standalone `fn_*` | Emitted Rust Fns | `t` / `e` Shards | Generated Win64 LOC (MiB) | Non-`v8_nonabi` Rejects |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **`cohtml.windowsdesktop.dll`** (now `--rva-list`: `4,653`) | `10,291` | `1,107` (`1,034`) | **`11,390`** (`+6,737`) | `21` | `3,118` (`530`) | `8,251` | `8,781` | `89` / `3` | `380,658` (`125.13`) | **`8`** (was `75`) |
| **`v8.dll`** (now `--rva-list`: `11,813` Win64) | `37,298` (`35,125` Win64 + `2,173` non-ABI) | `53,198` (`33,197`) | **`88,308`** (`+76,495`) | `39,604` | `21,983` (`2,879`) | `26,721` | `29,600` | `209` / `19` | `959,142` (`242.46` incl. P1) | **`16`** (was `1,892`) |
| `renoircore.windowsdesktop.dll` | `2,453` | `415` (`398`) | `2,865` | `23` | `257` (`92`) | `2,585` | `2,677` | `25` / `1` | `106,369` (`33.48`) | `3` |
| `msvcp140.dll` | `1,237` | `407` (`105`) | `1,641` | `50` | `481` (`131`) | `1,110` | `1,241` | `9` / `1` | `38,928` (`8.76`) | `3` |
| `v8_libbase.dll` (`g26` / `g27`) | `381` | `1,019` / `1,036` | `1,396` / `1,413` | `634` | `323` (`67` / `70`) | `439` | `506` | `3` / `1` | `13,108` (`2.43`) | `5` |
| `v8_libplatform.dll` (`g26` / `g27`) | `324` | `526` / `527` | `847` / `848` | `383` | `175` (`58`) | `289` | `347` | `3` / `1` | `9,467` (`1.61`) | `3` |
| `vcruntime140.dll` | `253` | `57` (`40`) | `289` | `3` | `31` (`13`) | `255` | `268` | `2` / `1` | `7,849` (`1.73`) | `7` (+`16` pre-skip) |
| `vcruntime140_1.dll` | `87` | `16` (`16`) | `97` | `0` | `4` (`2`) | `93` | `95` | `1` / `1` | `2,254` (`0.41`) | `3` (+`4` pre-skip) |
| **8-DLL Total (Win64 C++)** | **`52,324`** | **`56,745`** | **`106,833`** | **`40,718`** | **`26,372` (`3,772`)** | **`39,743`** | **`43,515`** | **`341` / `28`** | **`1,517,775` (`416.0` MiB)** | **`48`** |

### Key Takeaways from the Dry Run
1. **Zero `TRANSLATE_FAIL`s across all 106,881 Win64 candidates!** Every single function <= 32 KiB in full `cohtml` (`11,393` disassembled) and full `v8.dll` (`88,315` disassembled) translates through `translate_function` with **`0` `untranslatable insn`**, **`0` `cfg: blocks > 2048`**, **`0` `MAX_INSNS > 6000`** (for functions <= 32 KiB), **`0` `STRICT_FLAGS` failures**, and **`0` `seh_guard_fail`**. Only **1** function in the entire 8-DLL corpus (`cohtml+0x5a3ba0`) hits the precheck (`vpermd`).
2. **`--rva-list` was the sole cause of `99.7%` of thunk & `jcc` rejections:**
   - `cohtml`: `73` `thunk to guest-only target` rejections drop to **`0`**.
   - `v8.dll`: `1,885` `thunk to guest-only target` rejections drop to **`6`** (`4` of which disappear once the `MAX_FN_BYTES` and leaf-extent fixes below are applied, leaving only the `1` ILT thunk `+0x8ead0 -> +0xa07e00` `GetCurrentStackPosition`).
   - `v8.dll+0x5b2350` (`jcc to non-entry code at 0x7000c9e37e: jne 0x70007316a0`): lifts automatically in full-image mode (explained in §3.5).
3. **Massive deduplication in `v8.dll` (`66.5%` body elimination):**
   - Out of `88,308` lifted Win64 entries in `v8.dll`, **`39,604` (`44.8%`)** are incremental-link `E9 rel32` thunks aliased at zero codegen cost via `alias_of`, and **`21,983` (`24.9%`)** collapse into **`2,879` `tpl_*` const-generic templates** (`7.64x` sharing). Only **`29,600`** Rust functions are emitted for all `88,308` VAs (`0.335` Rust fns/entry).
   - In `cohtml.WindowsDesktop.dll`, `21` `E9` aliases + `3,118` entries across `530` `tpl_*` templates (`5.88x` sharing) + `8,251` standalone functions = **`8,781`** emitted Rust functions for `11,390` VAs (`0.771` Rust fns/entry).

---

## 2. Complete Rejection Taxonomy Without `--rva-list`

### 2.1 `cohtml.WindowsDesktop.dll` — All `8` Rejections (Out of `11,398` Total)

| Category | Count | RVAs | Exact Reason | Action in `g28` |
|---|---:|---|---|---|
| `CAP:max_fn_bytes` | `3` | `+0x7bc0` (`33,992` B, `8,611` insns)<br>`+0x40cc50` (`42,041` B, `7,455` insns)<br>`+0x505150` (`56,998` B, `11,277` insns) | `sz > MAX_FN_BYTES (32768)` (currently dropped silently before `offset_map`) | **Lift (+3)**: raise DLL `MAX_FN_BYTES=131072`, `MAX_INSNS=25000` (verified: all 3 lift cleanly, §3.1) |
| `PRECHECK:unsupported_insn` | `1` | `+0x5a3ba0` | `unsupported instruction at 0x70005a3be0: vpermd ymm2, ymm3, ymmword ptr [rcx]` (`std::reverse` AVX2 32-bit element reversal) | **Lift (+1)**: add `vpermd` lowering (`simd_ops::vpermd` + `gen_subsystems.py`, §3.2) |
| `EXTENT:segment_spill_into_padding` | `1` | `+0x3d0640` | Pdata-less switch leaf (`11,808` B to next `.pdata` `+0x3d3460`, jump table at `+0x3d2ce0`) truncated mid-instruction at `+0x3d1640` by `min(nxt, x + 4096)` | **Lift (+1)**: raise internal/dataref leaf extent cap `4096 -> 16384` (verified: lifts cleanly, §3.3) |
| `OUTPUT:null_address_access` | `1` | `+0x30f00` | `mulss xmm0, dword ptr [0x0]` (`f3 0f 59 04 25 00 00 00 00` at `+0x312bf`, `+0x312c8`, `+0x3137d`, `+0x317a6`): Clang UB-poisoned dead branches lowered via `core::ptr::read_unaligned((0x0u64) as *const u32)` (`#[deny(invalid_null_arguments)]` in `rustc`) | **Lift (+1)**: route `read_xmm_f32` / `read_xmm_f64` through `crate::simd_ops::ru32` / `ru64` (§3.4) |
| `ABI:context_import` | `2` | `+0x5a47f8`, `+0x5a4bd8` | `calls RtlCaptureContext` (CRT `__scrt_fastfail` / GS failure reporting) | **Keep guest (`2`)**: intentional architectural boundary (captures guest context) |

**Result after `g28` fixes:** **`11,396 / 11,398` (`99.98%`) lifted**; only the `2` CRT `RtlCaptureContext` callers remain guest.

---

### 2.2 `v8.dll` — All Rejections (Out of `88,324` Win64 + `1,643` `DF_REGFILE`)

| Category | Count | RVAs | Exact Reason | Action in `g28` |
|---|---:|---|---|---|
| `CAP:max_fn_bytes` | `2` | `+0x6278c0` (`40,372` B, `8,094` insns)<br>`+0xbde070` (`97,404` B, `20,822` insns) | `sz > MAX_FN_BYTES (32768)` (plus `2` ILT thunks `+0x63cc0`, `+0x13e90` pointing to them) | **Lift (+2 roots, +2 thunks = +4)**: raise DLL `MAX_FN_BYTES=131072`, `MAX_INSNS=25000` (verified: both lift cleanly, §3.1) |
| `EXTENT:segment_spill_into_padding` | `3` | `+0x563f30` (`10,320` B, JT `+0x565a28`)<br>`+0xa9b1f0` (`11,568` B, JT `+0xa9c4e4`)<br>`+0xad73b0` (`11,552` B, JT `+0xad86a4`) | Pdata-less switch leaves truncated at `x + 4096` before their trailing jump tables (plus `2` ILT thunks `+0x49f80`, `+0xbb130`) | **Lift (+3 leaves, +2 thunks = +5)**: raise leaf extent cap `4096 -> 16384` (verified: all 3 lift cleanly, §3.3) |
| `ABI:stack_position_reader` (false positive!) | `1` (+`1` thunk) | `+0xce61f0` (and ILT thunk `+0xb3000`) | `stack_arg_info` misflags `std::small_vector` destructor (`sub rsp, 0x48; add rcx, 0x178; lea rax, [rsp+0x20]; cmp rcx, rax; je ...`) because `"rsp" in parts` didn't hit, wait: `if has_call: stack_pos = [s for s in stack_pos if s[1] == "compares rsp"]`? No — `+0xce632f` had `#raxlea` live at `ret` along the `je +0xce632b` empty-vector fast path where no `call` executed, wait: `has_call` IS `True` in `+0xce61f0`? Let's check §3.6! | **Lift (+1 root, +1 thunk = +2)**: narrow `stack_pos` check (§3.6) |
| `ABI:stack_position_reader` (true `GetCurrentStackPosition`) | `1` (+`1` thunk) | `+0xa07e00` (and ILT thunk `+0x8ead0`) | `lea rax, [rsp]; ret` — routed in **0 guest steps** by `emu::is_routed_dll_stack_pos` (`g26`) | **Already 0-step native** (`emu` route) |
| `ABI:context_import` | `3` | `+0x108e8e0`, `+0x108e968`, `+0x108fb34` | `calls RtlCaptureContext` (CRT `__scrt_fastfail` / unhandled-exception helpers) | **Keep guest (`3`)**: intentional architectural boundary |
| `V8_NONABI` (`CrashForExceptionInNonABICompliantCodeRange`) | `27` builtins in `g27` (`2,173` `.pdata` spans) | `[0x104e54, 0x24acc8)` | Embedded builtins: `1,616 / 1,643` lifted via `DF_REGFILE` in `g27` (+ final `27` x87/atomic/trap builtins handled on `v8_regfile.py` track). Note: `v8.dll` `.pdata` splits the `1,643` builtins into `2,173` `.pdata` root entries (`530` sub-span `.pdata` entries inside multi-fragment builtins); see §3.8 for deduplicating `pre_rejected` against `nonabi_range`. | Handled by `v8_regfile.py` (`DF_REGFILE`) |

**Result after `g28` fixes:** **`88,319 / 88,324` (`99.994%`) Win64 C++ entries lifted**; the remaining `5` Win64 entries are the `3` `RtlCaptureContext` CRT helpers and the `2` 0-step-routed `GetCurrentStackPosition` VAs (`+0xa07e00`, `+0x8ead0`).

---

### 2.3 Other 6 DLLs — Every Residual Non-`RtlCaptureContext` Rejection

| DLL | RVA | Current Rejection | Root Cause & Disposition |
|---|---|---|---|
| `msvcp140.dll` | `+0x536fd` | `truncated extent (no terminator)` | **FH4 rethrow catch funclet** (`[0x536fd, 0x53731)`) of `basic_ostream::write` (`+0x44f60`, `FuncInfo4` `+0x61db4`): ends `call +0x52444; nop` where `+0x52444` is `jmp [rip+0x2eae]` (`VCRUNTIME140.dll!_CxxThrowException`, noreturn), followed immediately at `+0x53731` by pdata-less FH4 unwind dtor funclets (`lea rcx, [rdx+0x38]; jmp ...`). Because `+0x53731` is neither `0xCC` nor in `fn_starts`, `tail_after_call_ok` is `False`. **Fix (+1 lift -> `0` non-`RtlCaptureContext` rejects in `msvcp140`)**: recognize noreturn import thunks (`_CxxThrowException`, `abort`, `_Exit`, ` terminate`, `_X*`) or FH4 unwind action targets in `fn_starts` (§3.7). |
| `v8_libbase.dll` | `+0x25ee0` (+ thunk `+0x1900`) | `stack-position reader: stores its own stack address through a pointer at +0x25ee4` | `Stack::ObtainCurrentThreadStackStart`: `lea rax, [rsp]; mov [rcx], rax; mov rax, rcx; ret` — **already routed in 0 guest steps** by `emu::is_routed_dll_stack_pos` (`g26`). Keep as-is. |
| `vcruntime140.dll` | `+0xf8b8`, `+0xf90c` | `seh_filter_reads_context` | CRT SEH helpers passing `EXCEPTION_POINTERS*` through `call qword ptr [rip + 0xc177]` (`_seh_filter_dll` / `_seh_filter_exe`). Intentional T3 SEH boundary; keep guest. |
| `vcruntime140_1.dll` | `+0x39b8` | `seh_filter_reads_context` | `__CxxFrameHandler4` internal SEH wrapper whose filter `+0x49c0` dispatches via jump table (`indirect jmp at +0x47b7`). Intentional EH runtime boundary; keep guest. |

---

## 3. Root-Cause Disassemblies & Surgical Fixes

### 3.1 Fix 1 — Explicit `MAX_FN_BYTES` Accounting + Raise DLL Caps (`+5` Roots, `+2` Thunks)

**Root cause:**
1. In `gen_subsystems.py` lines `4080` and `4097`, functions with `sz > MAX_FN_BYTES` (`32,768`) are skipped with `continue` *without* writing an entry to `reject_reason[va]`. As a result, `cohtml` (`+0x7bc0`, `+0x40cc50`, `+0x505150`) and `v8.dll` (`+0x6278c0`, `+0xbde070`) silently vanished from both `lifted_by_va` and `db/dll_progress.tsv` (causing `v8.dll` thunks `+0x63cc0 -> +0x6278c0` and `+0x13e90 -> +0xbde070` to drop as `thunk to guest-only target`).
2. All 5 oversize functions were tested directly with `AW2_GEN_MAX_BYTES=131072 AW2_GEN_MAX_INSNS=25000`:
   - `cohtml+0x7bc0` (`33,992` B, `8,611` insns) -> **lifts cleanly** (`1,652` Rust lines)
   - `cohtml+0x40cc50` (`42,041` B, `7,455` insns) -> **lifts cleanly** (`28` Rust lines — structured straight-line initializer)
   - `cohtml+0x505150` (`56,998` B, `11,277` insns) -> **lifts cleanly** (`93` Rust lines)
   - `v8.dll+0x6278c0` (`40,372` B, `8,094` insns) -> **lifts cleanly** (`272` Rust lines)
   - `v8.dll+0xbde070` (`97,404` B, `20,822` insns) -> **lifts cleanly** (`1,644` Rust lines, `< 2,048` basic blocks)

**Patch (`dll_lift.py` & `gen_subsystems.py`):**
```diff
--- a/tools/decomp_scale/dll_lift.py
+++ b/tools/decomp_scale/dll_lift.py
@@ -1220,9 +1220,9 @@ def load(args, gs):
     if "AW2_GEN_MAX_INSNS" not in os.environ:
-        gs.MAX_INSNS = 6000
+        gs.MAX_INSNS = 25000
     if "AW2_GEN_MAX_BYTES" not in os.environ:
-        gs.MAX_FN_BYTES = 32768
+        gs.MAX_FN_BYTES = 131072
```
And in `gen_subsystems.py`, record `reject_reason[va]` when `sz > MAX_FN_BYTES` or `sum(len(x[1]) for x in raw_segs) > MAX_FN_BYTES` so no candidate ever disappears from `dll_progress.tsv` silently:
```diff
--- a/tools/decomp_scale/gen_subsystems.py
+++ b/tools/decomp_scale/gen_subsystems.py
@@ -4080,4 +4080,8 @@ def main(argv=None):
-        if not b or sz > MAX_FN_BYTES:
+        if not b:
+            reject_reason[va] = "empty primary extent"
+            continue
+        if sz > MAX_FN_BYTES:
+            reject_reason[va] = f"exceeds MAX_FN_BYTES ({sz} > {MAX_FN_BYTES})"
             continue
...
         if sum(len(x[1]) for x in raw_segs) > MAX_FN_BYTES:
+            reject_reason[va] = f"exceeds MAX_FN_BYTES ({sum(len(x[1]) for x in raw_segs)} > {MAX_FN_BYTES})"
             continue
```

---

### 3.2 Fix 2 — `vpermd` AVX2 Lowering (`cohtml+0x5a3ba0`, `+1` Lift)

**Disassembly (`cohtml.WindowsDesktop.dll+0x5a3ba0`, MSVC `__std_reverse_trivially_swappable_4`):**
```asm
1805a3bb9: vmovdqu ymm3, ymmword ptr [rip + 0x5b5f] ; [7, 6, 5, 4, 3, 2, 1, 0]
...
1805a3be0: vpermd  ymm2, ymm3, ymmword ptr [rcx]
1805a3be5: vpermd  ymm1, ymm3, ymmword ptr [r8 - 0x20]
1805a3beb: sub     r8, 0x20
1805a3bef: vmovdqu ymmword ptr [rcx], ymm1
1805a3bf3: add     rcx, 0x20
1805a3bf7: vmovdqu ymmword ptr [r8], ymm2
1805a3bfc: cmp     rcx, rdx
1805a3bff: jne     0x1805a3be0
```
Note: the sibling 8-byte reversal function right below it at `+0x5a3c70` uses `vpermq ymm2, ymmword ptr [rcx], 0x1b` and already lifts! Adding `vpermd` (`VPERMD ymm1, ymm2, ymm3/m256`: for `i in 0..8`, `dst.dword[i] = src.dword[idx.dword[i] & 7]`) completes the pair.

**Patch (`crates/aw2-game/src/simd_ops.rs` + `tools/decomp_scale/gen_subsystems.py`):**
```rust
// crates/aw2-game/src/simd_ops.rs
#[inline(always)]
pub fn vpermd(idx_lo: u128, idx_hi: u128, src_lo: u128, src_hi: u128) -> (u128, u128) {
    let s = [
        src_lo as u32, (src_lo >> 32) as u32, (src_lo >> 64) as u32, (src_lo >> 96) as u32,
        src_hi as u32, (src_hi >> 32) as u32, (src_hi >> 64) as u32, (src_hi >> 96) as u32,
    ];
    let pick = |ctl: u128| -> u128 {
        (s[(ctl as usize) & 7] as u128)
            | ((s[((ctl >> 32) as usize) & 7] as u128) << 32)
            | ((s[((ctl >> 64) as usize) & 7] as u128) << 64)
            | ((s[((ctl >> 96) as usize) & 7] as u128) << 96)
    };
    (pick(idx_lo), pick(idx_hi))
}
```
```diff
--- a/tools/decomp_scale/gen_subsystems.py
+++ b/tools/decomp_scale/gen_subsystems.py
@@ -385,7 +385,7 @@ XMM_MISC_MNS = {
     "vrcpps", "rcpps", "vrsqrtps", "rsqrtps",
     "vinsertps", "insertps", "vpalignr", "palignr",
     "vroundps", "roundps", "vroundpd", "roundpd", "vroundss", "roundss", "vroundsd", "roundsd",
-    "vcvtps2ph", "vcvtph2ps", "vpermpd", "vpermq", "vstmxcsr", "stmxcsr", "vptest", "ptest",
+    "vcvtps2ph", "vcvtph2ps", "vpermpd", "vpermq", "vpermd", "vstmxcsr", "stmxcsr", "vptest", "ptest",
     "vblendvps", "blendvps", "vblendvpd", "blendvpd", "vpblendvb", "pblendvb",
     "vmovmskps", "movmskps", "vmovmskpd", "movmskpd", "vpmovmskb", "pmovmskb",
     "vbroadcastss", "vbroadcastsd", "vpbroadcastb", "vpbroadcastw", "vpbroadcastd", "vpbroadcastq",
@@ -1730,6 +1730,17 @@ def translate_linear_insns(
                     return None, None
                 d_idx = dst[1][3:]
                 lines.append(f"let (p_lo, p_hi) = crate::simd_ops::vpermpd({hs[0]}, {hs[1]}, 0x{imm[1] & 0xff:x}u8); {dst[1]} = p_lo; ymm{d_idx}_hi = p_hi;")
+            elif mn == "vpermd":
+                if len(ops) != 3:
+                    return None, None
+                dst, idx_op, src_op = parse_op(ops[0]), parse_op(ops[1]), parse_op(ops[2])
+                if not dst or not idx_op or not src_op or dst[0] != "reg" or dst[2] != 256:
+                    return None, None
+                hi = read_ymm_halves(idx_op)
+                hs = read_ymm_halves(src_op)
+                if not hi or not hs:
+                    return None, None
+                d_idx = dst[1][3:]
+                lines.append(f"let (p_lo, p_hi) = crate::simd_ops::vpermd({hi[0]}, {hi[1]}, {hs[0]}, {hs[1]}); {dst[1]} = p_lo; ymm{d_idx}_hi = p_hi;")
```

---

### 3.3 Fix 3 — Pdata-Less Switch Leaf Extent Cap (`4096 -> 16384`, `+4` Leaves, `+2` Thunks)

**Disassembly (`cohtml+0x3d0640`, `v8+0x563f30`, `v8+0xa9b1f0`, `v8+0xad73b0`):**
All 4 functions rejected for `segment decode spilled into padding` are large frameless/pdata-less C++ `switch` lookup tables (CSS keyword / Unicode / AST token tables) whose code is followed in `.text` by their RVA jump tables right before the next `.pdata` start:
- `cohtml+0x3d0640`: next `.pdata` start `+0x3d3460` (`11,808` B); jump table at `+0x3d2ce0` (`mov eax, [r9 + 4*rdx + 0x3d2ce0]`). Truncating at `+0x3d0640 + 4096 = +0x3d1640` slices right through the 5-byte `0f 85 92 16 00 00` (`jne 0x1803d2cdd`) instruction at `+0x3d1645` and omits the jump table, causing `(start_off + seg_len) not in insn_by_off` with `jt_in_seg == False`.
- `v8.dll+0x563f30`: next `.pdata` `+0x566780` (`10,320` B); jump table at `+0x565a28`. Truncated at `+0x564f30` mid-`lea rax, [rip + 0xbd177b]`.
- `v8.dll+0xa9b1f0`: next `.pdata` `+0xa9df20` (`11,568` B); jump table at `+0xa9c4e4`. Truncated at `+0xa9c1f0`.
- `v8.dll+0xad73b0`: next `.pdata` `+0xada0d0` (`11,552` B); jump table at `+0xad86a4`. Truncated at `+0xad83b0`.

Every leaf's extent is *already* bounded by `bounds[j]` (the sorted union of `.pdata` starts, exports, internal leaves, and `GuardCFFunctionTable` / relocation dataref starts). The `4096` constant in `min(nxt, x + 4096, pe.text_end)` was only a guard for the final function before `.text` end. Raising `4096 -> 16384` in `dll_lift.py` (`resync`, `work` scan, and `leaf_ext` assignment) was simulated directly against both PE images: all 4 switch leaves find their in-segment jump tables, cut cleanly at `min_jt`, end on `ret`, and translate with **0 errors** (`863`, `314`, `608`, and `608` Rust lines), and `v8.dll` thunks `+0x49f80` and `+0xbb130` lift alongside them.

**Patch (`tools/decomp_scale/dll_lift.py`):**
```diff
--- a/tools/decomp_scale/dll_lift.py
+++ b/tools/decomp_scale/dll_lift.py
@@ -1604,7 +1604,7 @@ def load(args, gs):
                 k = bisect.bisect_right(cstarts, t)
                 nxt = cstarts[k] if k < len(cstarts) else pe.text_end
-                if dis.resync(t, min(nxt, t + 4096)):
+                if dis.resync(t, min(nxt, t + 16384)):
                     dataref_resynced.append(t)
@@ -1669,7 +1669,7 @@ def load(args, gs):
             k = bisect.bisect_right(cstarts, t)
             nxt = cstarts[k] if k < len(cstarts) else pe.text_end
-            dis.resync(t, min(nxt, t + 4096))
+            dis.resync(t, min(nxt, t + 16384))
@@ -1766,7 +1766,7 @@ def load(args, gs):
             j = bisect.bisect_right(bounds, x)
             nxt = bounds[j] if j < len(bounds) else pe.text_end
-            ext = leaf_ext.get(x) or (min(nxt, x + 4096, pe.text_end) - x)
+            ext = leaf_ext.get(x) or (min(nxt, x + 16384, pe.text_end) - x)
             if ext > 0:
@@ -1780,7 +1780,7 @@ def load(args, gs):
             # Bounded by the next known start; the 16 KiB cap only guards the last function.
-            ext = min(nxt, x + 4096, pe.text_end) - x
+            ext = min(nxt, x + 16384, pe.text_end) - x
             if ext > 0:
```

---

### 3.4 Fix 4 — `cohtml+0x30f00` (`null-address memory access in output`, `+1` Lift)

**Disassembly (`cohtml.WindowsDesktop.dll+0x30f00`, `0x312bf..0x312dd`):**
```asm
1800312bf: f3 0f 59 04 25 00 00 00 00  mulss xmm0, dword ptr [0x0]
1800312c8: f3 0f 59 0c 25 00 00 00 00  mulss xmm1, dword ptr [0x0]
```
When Clang optimizes an unreachable/out-of-bounds C++ null-pointer branch (`esi >= 16` or `r8 == nullptr` at `+0x31368`), it folds the address `[0x0]` directly into `mulss xmm0, dword ptr [0x0]`.
- For GPR loads (`mov ecx, [0x8]` in `shards/subsystems/t373.rs:1617`), `gen_subsystems.py` emits `crate::simd_ops::ru32(0x8u64)` — inline `asm!` on `aarch64`, which never creates a Rust reference or calls `core::ptr::read_unaligned`, so `rustc` compiles it with 0 warnings/errors and hardware raises `SIGSEGV` at `0x0` if ever reached, matching x86.
- Only `read_xmm_f32` (`gen_subsystems.py:831`) and `read_xmm_f64` (`gen_subsystems.py:839`) still emitted `core::ptr::read_unaligned(({op[2]}) as *const u32)`, which triggers `rustc`'s default-deny `#[deny(invalid_null_arguments)]` compile error when `op[2] == "0x0u64"` — the exact reason line `4395` (`if any("(0x0u64) as *" in l for l in lines)`) had to reject `+0x30f00`!

**Patch (`tools/decomp_scale/gen_subsystems.py`):**
Routing `read_xmm_f32` / `read_xmm_f64` through `crate::simd_ops::ru32` / `ru64` eliminates the `core::ptr::read_unaligned` UB hazard, makes scalar FP loads TSO-consistent and SEH-guardable (`_guard_expr` already handles `simd_ops::ru32`/`ru64`!), and lets `+0x30f00` compile with 0 warnings:
```diff
--- a/tools/decomp_scale/gen_subsystems.py
+++ b/tools/decomp_scale/gen_subsystems.py
@@ -828,15 +828,15 @@ def read_xmm_f32(op):
     if op[0] == "reg" and op[2] == 128:
         return f"f32::from_bits({op[1]} as u32)"
     if op[0] == "mem" and op[1] in (0, 32):
-        return f"f32::from_bits(core::ptr::read_unaligned(({op[2]}) as *const u32))"
+        return f"f32::from_bits(crate::simd_ops::ru32({op[2]}))"
     return None
 
 
 def read_xmm_f64(op):
     if op[0] == "reg" and op[2] == 128:
         return f"f64::from_bits({op[1]} as u64)"
     if op[0] == "mem" and op[1] in (0, 64):
-        return f"f64::from_bits(core::ptr::read_unaligned(({op[2]}) as *const u64))"
+        return f"f64::from_bits(crate::simd_ops::ru64({op[2]}))"
     return None
```

---

### 3.5 Diagnosis of `v8.dll+0x5b2350` (`jcc to non-entry code at 0x7000c9e37e: jne 0x70007316a0`)

**Disassembly:**
```asm
; v8.dll +0x5b2350 (.pdata extent accidentally spans [0x5b2350, 0x5b2390) across two 25-byte twins):
1805b2350: mov rax, [rcx + 0x18]; mov edx, [rax + 0x8]; shr rdx, 0xa; test dl, 0x1
1805b235e: jne 0x180099580     ; ILT E9 -> +0x5b5d80
1805b2364: jmp 0x1800ad450     ; ILT E9 -> +0x5b6090
1805b2369..1805b236f: int3 (x7)
1805b2370: mov rax, [rcx + 0x18]; mov edx, [rax + 0x8]; shr rdx, 0xa; test dl, 0x1
1805b237e: jne 0x1800456a0     ; ILT E9 -> +0x5b5d60 (frameless leaf!)
1805b2384: jmp 0x18006a020     ; ILT E9 -> +0x5b6080

; v8.dll +0x456a0 (ILT thunk):
1800456a0: e9 bb 06 57 00      jmp 0x1805b5d60
```
**Why `--rva-list` rejected it & why full-image mode lifts it automatically:**
`+0x456a0` redirects (`CALL_REDIRECT`) to frameless leaf `+0x5b5d60`. Under `--rva-list --dataref-listed`, `+0x5b5d60` was in `GuardCFFunctionTable` (`dataref`) and in `callers[0x5b5d60] = {0x5b2350}`, wait: why wasn't it in `LEAF_STARTS`? Because `gs.LEAF_STARTS` is built *before* `--rva-list` filtering (`line 1791`), EXCEPT `dataref` was filtered early at `line 1599` (`if rva_listed_set is not None and t not in rva_listed_set: continue`) AND in `scan(0x5b2350)` `mn` was `"jne"`, so `scan()` checked `pe.data[to - 1] != 0xCC` on `t = 0x5b5d60` where `+0x5b5d5f` was the `ret` (`0xC3`) of the preceding function without `int3` padding!
When `--dataref-listed` is dropped, `GuardCFFunctionTable` registers `+0x5b5d60` in `dataref` -> `leaf_ext` -> `gs.LEAF_STARTS`, so `t2 in LEAF_STARTS` succeeds immediately and `+0x5b2350` lifts with zero code changes!

---

### 3.6 Fix 5 — `v8.dll+0xce61f0` (`stack-position reader` False Positive, `+1` Root, `+1` Thunk)

**Disassembly (`v8.dll+0xce61f0`):**
```asm
180ce61f0: sub rsp, 0x48
180ce61f4: add rcx, 0x178          ; &this->inline_storage_
180ce61fb: lea rax, [rsp + 0x20]
180ce6200: cmp rcx, rax
180ce6203: je  0x180ce632b         ; -> add rsp, 0x48; ret (rax still holds #raxlea!)
...
180ce6261: call qword ptr [rax + 0x8]
```
**Root cause:**
1. `+0xce61f0` is a `void` destructor (`~ZoneVector` / small-buffer-optimized container cleanup, `rcx = this`). On the fast path (`je +0xce632b`), `rax` happens to still contain `lea rax, [rsp + 0x20]` at `ret` (`+0xce632f`), even though `rax` is dead (void return) and the function contains `3` `call`s in its body (`+0xce6261`, `+0xce629a`, `+0xce630d`).
2. Look at `dll_lift.py` lines `488-489`:
   ```python
   if has_call:
       stack_pos = [s for s in stack_pos if s[1] == "compares rsp"]
   ```
   Wait — why wasn't `+0xce61f0` cleared by `if has_call:`?! Look at `fn_frags[0xce61f0]`:
   `[('0xce61f0', '0xce6209'), ('0xce6209', '0xce623f'), ('0xce623f', '0xce6317'), ('0xce6317', '0xce632b'), ('0xce632b', '0xce6330')]`.
   In `+0xce6239` (the end of fragment 2), `0f 84 d8 00 00 00` is `je 0x180ce6317`, and its fall-through into fragment 3 (`0xce623f`) crosses a `.pdata` fragment boundary! Look at `stack_arg_info` lines `318-320`:
   ```python
   for b, e in frags:
       seg = dis.range(b, e)
       for i, (va, mn, ops, parts) in enumerate(seg):
           lst.append((va, mn, ops, parts, i + 1 < len(seg)))
   ```
   `ft` (`i + 1 < len(seg)`) is **`False`** at the end of a `.pdata` fragment even when `e == next_frag_b` (`0xce6209 == 0xce6209` and `0xce623f == 0xce623f`)! Because `ft` was `False` at `+0xce6203` (`je +0xce632b`), the dataflow worklist in `stack_arg_info` **never walked into fragment 2 (`+0xce6209`) or fragment 3 (`+0xce623f`)**, leaving `state[i] is None` on all `3` `call` instructions so `has_call` stayed `False`!

**Two-for-one insight & patch (`tools/decomp_scale/dll_lift.py`):**
Fixing `ft` across contiguous `.pdata` fragments (`e == next_b`, which MSVC hot/cold & unwind splitting produces whenever prolog/epilog unwind info is chained) makes `stack_arg_info` accurately walk multi-fragment functions (`has_call = True` on `+0xce61f0`, lifting both `+0xce61f0` and its ILT thunk `+0xb3000` AND improving `max_slot`/`escapes` accuracy across all multi-fragment functions):
```diff
--- a/tools/decomp_scale/dll_lift.py
+++ b/tools/decomp_scale/dll_lift.py
@@ -314,10 +314,13 @@ def stack_arg_info(dis, frags):
     every memory operand on a tracked register yields entry offset -> slot. Conservative only
     in the sense that matters here: a spurious hit keeps a function as guest code."""
     lst = []
+    frag_starts = {b for b, _ in frags}
     for b, e in frags:
         seg = dis.range(b, e)
         for i, (va, mn, ops, parts) in enumerate(seg):
-            lst.append((va, mn, ops, parts, i + 1 < len(seg)))
+            lst.append((va, mn, ops, parts, (i + 1 < len(seg)) or (e in frag_starts)))
```
*(Note: what is `+0xce61f0` actually comparing at `+0xce6200`? `rcx + 0x178 == rsp + 0x20` — wait: could `rcx` ever be stack-allocated in a caller so `rcx + 0x178` happens to equal `rsp + 0x20` of the callee's own local `[rsp+0x20]`? `[rsp + 0x20]` is inside `+0xce61f0`'s own freshly allocated `0x48`-byte stack frame (`[entry_rsp - 0x28]`), which cannot alias a caller's valid object in either guest or lifted execution! Fixing `ft` lets `has_call` filter it cleanly.)*

---

### 3.7 Fix 6 — `msvcp140.dll+0x536fd` (`truncated extent` on FH4 Rethrow Catch Funclet, `+1` Lift)

**Disassembly (`msvcp140.dll+0x536fd`, catch handler of `std::basic_ostream<wchar_t>::write` `+0x44f60`):**
```asm
1800536fd: mov  qword ptr [rsp + 0x10], rdx
180053702: push rbp
180053703: sub  rsp, 0x20
180053707: mov  rbp, rdx
...
180053722: call 0x180008e20    ; basic_ios::setstate
180053727: xor  edx, edx
180053729: xor  ecx, ecx
18005372b: call 0x180052444    ; jmp [rip+0x2eae] -> VCRUNTIME140.dll!_CxxThrowException (noreturn!)
180053730: nop                 ; end of .pdata [0x536fd, 0x53731)
180053731: lea  rcx, [rdx + 0x38]  ; next function (frameless FH4 dtor thunk, no .pdata)
180053738: jmp  0x1800412e4
```
**Root cause:**
`+0x536fd` ends with `call +0x52444` (`_CxxThrowException;`), padded with a 1-byte `nop` (`0x90`) to `+0x53731`, where MSVC immediately placed a 12-byte frameless FH4 unwind cleanup thunk (`lea rcx, [rdx+0x38]; jmp +0x412e4`). Because `+0x53731` has no `.pdata` entry (12-byte leaf thunk) and `0x90` is popped as `nop`, `trimmed[-1]` is `call 0x7001e66444` and `seg_end_va` (`0x7001e67731`) is neither `0xCC` nor in `fn_starts`, so `tail_after_call_ok` (`gen_subsystems.py:4318`) evaluates to `False`.

**Patch (`tools/decomp_scale/dll_lift.py`):**
Add FH4 `UnwindMap` action RVAs (`typ in (1, 2, 3)`) to `fn_starts` (or mark calls to noreturn import stubs `_CxxThrowException` / `abort` / `terminate` as `tail_after_call_ok = True` in `gen_subsystems.py`). Checking noreturn import stubs at `trimmed[-1]` in `gen_subsystems.py` is 6 lines and handles every DLL/exe noreturn-call tail without `int3` padding:
```diff
--- a/tools/decomp_scale/dll_lift.py
+++ b/tools/decomp_scale/dll_lift.py
@@ -76,6 +76,9 @@ SETJMP_IMPORT_RE = re.compile(r"^(__intrinsic_setjmp|__intrinsic_setjmpex|_setjm
 BACKTRACE_IMPORT_RE = re.compile(r"^(RtlCaptureStackBackTrace|RtlWalkFrameChain|CaptureStackBackTrace)$")
+NORETURN_IMPORT_RE = re.compile(
+    r"^(_CxxThrowException|abort|_Exit|exit|quick_exit|terminate|_invalid_parameter_noinfo_noreturn)$"
+)
```
(Or record `gs.NORETURN_CALL_VAS` from import stubs matching `NORETURN_IMPORT_RE` in `dll_lift.load`, and set `tail_after_call_ok = True` when `CALL_REDIRECT.get(t_va, t_va) in NORETURN_CALL_VAS`.)

---

### 3.8 Cosmetic / Bookkeeping Cleanup — `v8.dll` Sub-Builtin `.pdata` Spans in `pre_rejected`

In `v8.dll`, `EmbeddedData::LayoutDescription` has **`1,643`** embedded builtins inside `[0x104e54, 0x24acc8)`, whereas `.pdata` has **`2,173`** root entries with `CrashForExceptionInNonABICompliantCodeRange` (`530` extra `.pdata` entries are interior sub-ranges of large builtins that `v8_regfile` already lifts as part of their parent builtin + `conts`).
In `dll_lift.py` line `1325`:
```python
if "v8_nonabi" in kinds:
    if r not in regfile_lifted_rvas:
        pre_rejected[base + r] = "v8_nonabi: ..."
    continue
```
Because `regfile_lifted_rvas` only holds the `1,616` *entry* RVAs from `LayoutDescription`, the `530` interior `.pdata` spans whose parent builtin *is* lifted by `v8_regfile` get recorded in `pre_rejected` as `v8_nonabi` rejects in `db/dll_progress.tsv`!
**Patch (`tools/decomp_scale/dll_lift.py`):**
```diff
-        regfile_lifted_rvas = {rva for _va, rva, _ilen, _bid, _code, _conts in regfile_res["lifted"]}
+        regfile_spans = [(rva, rva + ilen) for _va, rva, ilen, *_ in regfile_res["lifted"]]
+        regfile_all_rvas = {rva for rva, _ in regfile_spans} | {rva for _, rva, *_ in regfile_res["rejected"]}
```
Skip any `.pdata` root `r` inside `nonabi_range` that is covered by `regfile_spans` (or not a `LayoutDescription` builtin start), so `db/dll_progress.tsv` has an exact 1-to-1 row per genuine `v8_builtin_<id>` (`1,643` rows total, zero phantom sub-span `sub_10xxxx` rejects).

---

## 4. Compile-Time, Shard Count & Runtime Table Capacity Projections

### 4.1 Source Size & Shard Delta (`aw2-game`)

| Metric | Current (`g27` w/ `--rva-list`) | Full 8-DLL (`g28`, zero `--rva-list`) | Delta |
|---|---:|---:|---:|
| `cohtml` lifted entries / Rust fns | `4,653` / `3,962` | **`11,396`** / **`8,787`** | `+6,743` entries / `+4,825` fns |
| `cohtml` shards / LOC / MiB | `44 t + 1 e` / `185.8k` / `61.4 MiB` | **`89 t + 3 e`** / **`383.2k`** / **`126.0 MiB`** | `+47` files / `+197.4k` LOC / `+64.6 MiB` |
| `v8.dll` Win64 entries / Rust fns | `11,813` / `9,108` | **`88,319`** / **`29,606`** | `+76,506` entries / `+20,498` fns |
| `v8.dll` Win64 `t`+`e` shards / LOC / MiB | `53 t + 3 e` / `231.5k` / `61.5 MiB` | **`210 t + 19 e`** / **`961.1k`** / **`240.2 MiB`** | `+173` files / `+729.6k` LOC / `+178.7 MiB` |
| `v8.dll` `DF_REGFILE` (`g27` 1,616 builtins) | `36 r + 2 re` / `667.0k` / `28.6 MiB` | `36 r + 2 re` / `667.0k` / `28.6 MiB` | unchanged |
| Other 6 DLLs (`msvcp`, `renoir`, `vcrt*`, `libbase`, `libplat`) | `43 t + 6 e` / `177.9k` / `48.4 MiB` | `43 t + 6 e` / `178.0k` / `48.4 MiB` | `+1` fn (`msvcp140+0x536fd`) |
| **Total `aw2-game` crate (incl. `AlanWake2.exe` shards)** | `991` files / `3.85M` LOC / `788 MiB` | **`1,211` files / `4.78M` LOC / `1,031 MiB`** | **`+24.1%` LOC / `+30.8%` source bytes** |

### 4.2 Generator, Compile-Time, Binary-Size & Hash-Table Checks
1. **Generator runtime**: Measured wall-clock in `audit_full_dlls.py` is **`12.8 s`** for full `cohtml` and **`97.0 s`** for full `v8.dll` (`~115 s` total for all 8 DLLs).
2. **`cargo build --release -p aw2-loader` compile time & binary size**:
   - `aw2-game` compiles at `opt-level = 0` with `codegen-units = 32` (`g25` built `3.20M` LOC -> `455 MiB` binary in `14m 22s`; `g27` built `3.85M` LOC + debug tests in `~25m`).
   - Adding `+927k` LOC (`+24.1%`) across `204` new `4,200`-LOC `t*.rs` modules distributes evenly across all 32 CGUs, projecting **`+3.5 to +4.5 minutes`** incremental release build time (`~18–20 min` clean release build) and **`+95 to +115 MiB`** binary size (`~560–575 MiB` `aw2-loader` Mach-O, well below the `2 GiB` Mach-O / ARM64 `ADRP` limit).
3. **`VA_DISPATCH` capacity check (`CRITICAL` — must bump in `g28`!)**:
   - `crates/aw2-game/src/exe.rs:127` currently sets `VA_TABLE_BITS: u32 = 19` (`VA_TABLE_CAP = 524,288` slots).
   - Current load: `244,796` (`AlanWake2.exe`) + `23,601` (DLLs) = `268,397` entries (`51.2%` load factor).
   - With full `cohtml` + full `v8.dll` (`106,849` DLL Win64 entries + `244,796` exe entries = **`351,645` entries**), `VA_TABLE_BITS = 19` reaches **`67.1%` load factor** (longer linear-probe clusters on hot `call_va8` lookups!).
   - **Action in `g28`**: Bump `VA_TABLE_BITS` from `19` (`524,288`, `8 MiB`) to **`20`** (`1,048,576`, `16 MiB`) in `crates/aw2-game/src/exe.rs:127`, bringing load factor down to **`33.5%`**.
4. **`cov::stub` (`AW2_DLL_LIFT_COV=1`) check**:
   - Allocates `48`-byte `MAP_JIT` stubs in `192 KiB` chunks on demand (`106,849` entries = `5.13 MiB` across `27` `mmap` calls, only when `AW2_DLL_LIFT_COV=1` is set). Zero impact when unset.
5. **`retlive_dll.py` & difftest scaling**:
   - `retlive_dll.py` already analyses the full PE `.pdata` + exports of all 8 DLLs (`41 s` total in `g27`); only the target filter `Lifted::new` grows.
   - For `dll_lift` difftest (`AW2_DLL_DIFF_SHARD=k/N`), `88,319` `v8.dll` entries include `39,604` `E9` aliases that share the exact `fn_expr` of their target; skipping or 1-trial-checking `E9` aliases whose target `fn_expr` is already verified in the same image cuts full-`v8.dll` fuzz wall time by **`45%`** without losing any body coverage.

---

## 5. Recommended Execution Sequence (`g28`)

1. **Wait for `g27` to fast-forward `main`** (`1,616` `DF_REGFILE` builtins + `v8_libbase` `.data`-indirect `import_stub` fix).
2. **Apply the 6 surgical fixes in §3 + bump `VA_TABLE_BITS = 20`**:
   - `crates/aw2-game/src/exe.rs`: `VA_TABLE_BITS: u32 = 20`.
   - `crates/aw2-game/src/simd_ops.rs`: add `pub fn vpermd(...)`.
   - `tools/decomp_scale/gen_subsystems.py`: `vpermd` lowering, `read_xmm_f32`/`read_xmm_f64` -> `simd_ops::ru32`/`ru64`, explicit `MAX_FN_BYTES` `reject_reason`.
   - `tools/decomp_scale/dll_lift.py`: `MAX_INSNS = 25000`, `MAX_FN_BYTES = 131072`, leaf extent cap `4096 -> 16384`, `stack_arg_info` contiguous-fragment `ft`, `NORETURN_IMPORT_RE` tail-call check, and `regfile_spans` `pre_rejected` dedup.
3. **Re-run `python3 tools/decomp_scale/audit_full_dlls.py`** to verify in `< 2 min` that non-`RtlCaptureContext` / non-0-step-routed Win64 rejections drop to **0** in `cohtml`, **0** in `v8.dll`, **0** in `msvcp140`, **0** in `renoircore`, **0** in `v8_libbase`, and **0** in `v8_libplatform`.
4. **Relift all 8 DLLs without `--rva-list` or `--dataref-listed`**, run `tools/fndb/retlive_dll.py`, build release + unit tests (`0` warnings), and run the sharded 8-DLL difftest gate + gameplay capture gate (`errors: 0`, `black%: 0`).
