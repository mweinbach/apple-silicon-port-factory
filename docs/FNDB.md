# Function Ownership Classifier (`tools/fndb.sh` — Alan Wake 2)

> **Scope:** Alan Wake 2–specific reverse-engineering tool for `games/alan-wake-2/crates/aw2-game`.
> Universal games running on `port-runner` do not use `fndb`.

`tools/fndb.sh build` assigns every one of the 244,796 functions in `AlanWake2.exe` an **owner**
(`remedy`, `thirdparty`, `crt`, or `unknown`), a **library**, and a **confidence score**. The goal
is knowing which functions are Remedy game code vs. third-party libraries (STL, rapidjson,
sparsehash, Wwise, Luau, mimalloc, …). For CLI commands and SQLite schema, see
[tools/README.md](../tools/README.md).

**Bottom line on accuracy:**
- `thirdparty` labels are high-precision (~100% on everything verifiable).
- Recall for *inlined C++ template code* (STL, rapidjson, sparsehash) compiled inside Remedy LTCG
  objects is low — no static heuristic fixes that reliably.
- A `remedy` label with `confidence < 0.5` means layout-only ("it sits between Remedy functions"),
  which covers 18.7 MB (~two-thirds of Remedy bytes).
- Functions with no evidence stay `unknown` rather than defaulting silently to `remedy`.
- Manual labels in `db/owner_overrides.tsv` always win (`confidence = 1.0`).

---

## 1. Toolchain Facts That Shaped the Classifier

- **MSVC 19.39 (VS 2022 17.9) LTCG:** Prebuilt non-LTCG static libraries live in contiguous
  address ranges where layout evidence works well. Template libraries instantiated inside Remedy
  LTCG translation units are interleaved directly with Remedy code.
- **DLL vs. statically linked middleware:** `protobuf`, `curl`, `PhysX`, and `cohtml` ship as DLLs,
  so only inline/generated glue lands in the `.exe`. `opentelemetry-cpp` 1.9.0, `nlohmann::json`,
  `fmt`, `Luau`, `lz4`, `zlib`, `mimalloc`, and `Wwise` are statically linked.
- **`__FILE__` paths:** Present only for middleware (Wwise, PhysX, OTel, `middleware/wwiseplugins`
  — `rmd_*` plugins are Remedy-written). No Remedy engine source paths appear in the binary.
- **Why whole-function cross-compiler hashing fails:** Compiling the exact MSVC 19.39 STL /
  rapidjson / sparsehash headers with `clang-cl` yielded **0 exact normalized-hash matches** for
  functions ≥ 10 instructions (MinHash Jaccard 0.10–0.38 even on identical source) because register
  allocation, instruction selection, and block layout differ. Reference builds are instead used for
  **distinctive strings, constants, and "value bags"** (struct field offsets + mask/compare
  immediates, which survive across compilers).

---

## 2. Classification Pipeline

```sh
tools/fndb.sh libcorpus all   # optional, once: pinned sources + string/const corpus + clang-cl refs
tools/fndb.sh build           # fnsig features -> evidence -> noisy-OR fusion -> db/functions.sqlite
tools/fndb.sh owner-eval      # check precision on overrides (with overrides off) + block hold-out
```

1. **`tools/fndb/fnsig`** (Rust, `iced-x86`, ~0.7 s over the whole exe) writes per-function features
   to `out/fndb/exe_feats.tsv`: sizes, exact/loose/mnemonic hashes, CFG summary, constants
   (`≥ 0x1000` and XMM pool), callees, strings, 32×MinHash, **value bag** (`m<size>@<disp>` field
   accesses + compare/test/mask/mov immediates), and mnemonic-class sequences (≤ 160 instructions).
2. **`tools/fndb/libcorpus.py`** *(optional)* clones pinned library sources (`microsoft/STL`
   17.9, `rapidjson`, `sparsehash`, `opentelemetry-cpp`, `nlohmann`, `zlib 1.2.8`, `protobuf`,
   `curl`, `lz4`, plus `Luau` and `PhysX` from `third_party/`), extracts unique strings/constants,
   and compiles reference objects with `clang-cl /O2 /Ob1` (`std` 760, `rapidjson` 281,
   `sparsehash` 494, `nlohmann` 930, `zlib` 110, `lz4` 151, `Luau` 4,306 functions).
3. **`tools/fndb/owner.py`** collects `(owner, lib, score, src, detail)` evidence rows per function
   (stored in `owner_evidence` and visible via `tools/fndb.sh show <va>`), combines same-owner
   scores via noisy-OR (capped at `0.98`), penalizes if a competing owner scores `≥ 0.5`, and marks
   margins `< 0.15` as `unknown` / `conflict`.

| Source | Signal | Score |
|---|---|---|
| `override` | Manual row in `db/owner_overrides.tsv` (never overwritten) | `1.00` |
| `name` | Export, `__FUNCSIG__`, vftable class, CRT anchor, or Ghidra FID (`std::_Func_impl::vfunc_2` lambda bodies → `remedy 0.6`) | `0.55–0.99` |
| `srcfile` | `__FILE__` under `middleware/` (`wwiseplugins` → `remedy 0.6`) | `0.95` |
| `crt_region` | Contiguous static-CRT block around entry point | `0.90` |
| `string` | String literal unique to one library or known prefix (dropped if function also uses Remedy strings) | `0.40–0.95` |
| `const` | Low-df (`≤ 12`) constant unique to one library and absent from Remedy-named code | `0.50–0.80` |
| `refsig` | Identical normalized hash or matching const+string signature + size/loop count | `0.75–0.90` |
| `refvals` | Value-bag idf-weighted Jaccard `≥ 0.7` (weight `≥ 18` nats, matching leafness & size ratio) | `0.55–0.75` |
| `stl_helper` | `≤ 16`-instruction leaf calling `msvcp140` `std::_X*` throw helpers | `0.85` |
| `vftref` | `lea` of a non-`std` class vftable (inlined ctor/dtor) | `0.70` |
| `corefstr` | Strings referenced only by confidently owned functions of one owner | `0.55–0.70` |
| `dupgroup` / `neardup` | Identical loose hash or MinHash `≥ 0.75` against confident (`≥ 0.8`) functions | `≤ 0.75` |
| `smallsim` | 6–160 instructions matching value bag + CFG + mnemonic sequence (`≥ 0.75`) | `≤ 0.70` |
| `callgraph` | All callers belong to library X | `0.60–0.75` |
| `neighbor` / `weak_neighbor` | Nearest confident (`≥ 0.6`) functions on both sides agree (decays with distance) | `0.30–0.70` |
| *(none)* | No evidence → `unknown` (`confidence = 0`) | `0.00` |

`tools/fndb.sh next` returns both `remedy` and `unknown` functions by default; pass `--min-conf X`
for confident Remedy code only.

---

## 3. Measured Distribution & Precision (2026-10-05 Baseline)

| Owner | Functions | Bytes | High (`≥ 0.8`) | Mid (`≥ 0.5`) | Low (`< 0.5`) |
|---|---:|---:|---:|---:|---:|
| `remedy` | 209,848 (85.7%) | 27.85 MB (74.3%) | 96,832 / 3.34 MB | 27,156 / 5.80 MB | 85,860 / 18.70 MB |
| `thirdparty` | 17,784 (7.3%) | 5.13 MB (13.7%) | 4,719 / 1.08 MB | 7,072 / 2.36 MB | 5,993 / 1.69 MB |
| `crt` | 365 | 9.7 KB | 354 / 9.0 KB | 5 / 0.3 KB | 6 / 0.5 KB |
| `unknown` | 16,799 (6.9%) | 4.48 MB (12.0%) | — | — | 16,799 / 4.48 MB |

Largest identified third-party libraries by size: `AK` (Wwise, 1.18 MB), `Kaim` (0.92 MB),
unattributed `thirdparty` (0.89 MB), `Luau` (0.52 MB), `std` (0.52 MB), `physx` (0.37 MB),
`opentelemetry` (0.28 MB), `lz4` (0.13 MB), `d3d` (0.08 MB), `sparsehash` (0.06 MB), `cohtml`
(0.06 MB), `Concurrency` (0.04 MB), `rapidjson` (0.03 MB), `zlib` (0.02 MB).

### Verification highlights
- **8 boot-probe library positives** (previously 0/8 caught): automatic recall **4/8** with
  overrides disabled (`std::_Fnv1a_append_bytes`, `sparsehash::find_position`,
  `rapidjson::GenericValue::GetDouble`, `rapidjson` double-const helper), **0 wrong-library**
  predictions, 2 `unknown`, 2 mislabeled `remedy` by low-confidence `neighbor`. All 8 live in
  `db/owner_overrides.tsv`.
- **64 KB block name hold-out (`owner-eval`, 10% windows):**
  - `thirdparty` predictions: **61 / 61 (100% precision)**, **26.5% recall** (the misses are
    inlined `std` template vfuncs inside Remedy objects).
  - `remedy` predictions: **99.6%** precision in high/mid bins, **98.8%** in low bin (**99.4%**
    recall).
- **Non-name evidence vs. confident names:** `string` (64/64), `srcfile` (24/24), `const` (17/17),
  `refsig` (13/13), and `refvals` (9/9) all scored **100%** agreement; `vftref` scored **98.7%**
  (1,955/1,981); `crt_region` scored **98.5%** (65/66).

---

## 4. Known Limits & Manual Overrides

- Expect plenty of `remedy`-labeled functions in low-confidence bins to actually be inlined STL,
  rapidjson, or sparsehash instantiations. Always sanity-check generic container/hash/sort routines
  before hand-decompiling.
- Add verified manual classifications to `db/owner_overrides.tsv`:
  ```tsv
  # va	owner	lib	note
  0x1413c21b0	thirdparty	rapidjson	rapidjson GenericValue::GetDouble (verified by boot-time probe)
  ```
  Valid owners: `remedy`, `thirdparty`, `crt`, `unknown`. Every override is automatically
  cross-checked (with overrides off) by `tools/fndb.sh owner-eval`.
