# Advisor review of DXR.md

Reviewer: graphics advisor. Inputs: `docs/design/DXR.md`, `ADVICE_M3.md`, the MSC 4.0.1 headers
(`metal_irconverter.h`, `metal_irconverter_runtime.h`, `ir_raytracing.h`), and AlanWake2.exe.

Tags: **[V]** = verified against a header or the binary. **[A]** = from memory, needs a test.

**Verdict.** The core idea is right. Writing the 64-byte MSC header at the D3D VA and keeping the
real `MTLAccelerationStructure` in a side allocation is exactly the layout MSC shaders expect.
§5 matches what the headers say MSC does in `IntersectionFunctionCompilationVisibleFunction` mode.

Before coding P1, fix three things:
- copies must carry the instance-contributions array;
- null instances need a defined encoding;
- Metal AS object identity per VA must be kept stable.

**The biggest scheduling finding:** the game ships an inline-RayQuery variant of almost every
RT pass, and at least the sun-shadow pass selects inline by default. So **P1 + P2 (plus a stub
P3) should light up most RT settings.** Full DispatchRays is only needed for path-traced
indirect lighting.

---

## 0. Must-fix list (before or during P1)

| # | Item | Section |
|---|------|---------|
| M1 | `CLONE`/`COMPACT` (and refit with src≠dst) must give dst its **own contributions array**, copied from src, and a dst header pointing at it. As written, only the header is rewritten. | 1.3 |
| M2 | Null or inactive instances (`AccelerationStructure == 0`) → `mask = 0` plus a **dummy valid BLAS ID**. `accelerationStructureID = 0` is not a documented "skip". Never compact instances out: that shifts `InstanceIndex()` and the contribution indices. | 2 |
| M3 | Keep **one Metal AS object per (resource, VA) for as long as the size fits**. Reallocate only when growing, with deferred release. Pre-size from prebuild maxima so reallocation is rare. | 1.2 |
| M4 | `EmitRaytracingAccelerationStructurePostbuildInfo` / `CopyRaytracingAccelerationStructure` return `void`, so `E_NOTIMPL` is impossible. For `SERIALIZATION`/`TOOLS_VISUALIZATION`, write a defined value (header size, 0 blobs) and log once. Make `SERIALIZE`/`DESERIALIZE` copy modes a logged no-op. | 1.3 |
| M5 | Header writes don't depend on the build result, so encode them **at record time, before or alongside the build**. Don't put a barrier per build. Batch per encoder; see 5. | 1.4, 5 |
| M6 | Once tier 1.1 is reported, **`CreateStateObject` must succeed** (P3-stub) even if DispatchRays isn't implemented. Return a real `ID3D12StateObject`/`Properties` with unique, stable 32-byte identifiers and sane `GetShaderStackSize`. The engine likely creates DXR pipelines for passes it doesn't select [A, log it]. | 4 |
| M7 | Set `IRDispatchRaysArgument.IntersectionFunctionTables` (the plural field, `ir_raytracing.h`) explicitly: either 0 or a valid array. §5 doesn't mention it. | 3 |

---

## 1. The "header at VA, real AS elsewhere" scheme: every reference path

### 1.1 Reference paths: fine as designed
- **SRV descriptors**: `IRDescriptorTableSetAccelerationStructure` stores `gpuVA = Location` and
  zeroes `textureViewID`/`metadata` [V]. GPU-written descriptors and descriptor copies work
  unchanged.
- **Root SRVs**: the VA passes straight through. The shader dereferences the header [V layout:
  `accelerationStructureID` (as MSL `instance_acceleration_structure`),
  `addressOfInstanceContributions`, `pad0[4]`, `pad1`, 64 B].
- **Shader records / local root args holding AS VAs**: also fine, because they're just VAs.
- **Instance descs**: the BLAS VA points at the BLAS header; the kernel reads `->accelerationStructureID`. Fine (see 2).
- MSC's own CPU helper `IRRaytracingSetAccelerationStructure(headerBuf, asResourceID,
  contribBuf, contribGPUVA, contribs, count)` [V] produces the same bytes. Use it in a unit test
  as the oracle for the header encoder.

### 1.2 Identity and lifetime (M3)
- A TLAS stores **BLAS resource IDs**, copied at TLAS build time. If a BLAS VA gets a *new*
  Metal object (it grew on rebuild, or was compacted into a new object), every TLAS built
  earlier still holds the old ID until the app rebuilds it.
  - D3D requires TLAS rebuild/refit after a BLAS rebuild [A, spec recall], so a well-behaved
    app is OK.
  - The *old* object must stay alive until the GPU is done, which `defer_release` handles.
  - Never destroy and recreate on same-size rebuilds.
- **Pool reuse.** Northlight almost certainly suballocates BLASes from large pool buffers [A].
  The app sizes its pool from our tiny reported sizes (64/256 B), so the real Metal memory is
  entirely outside its accounting.
  - Ranges abandoned by the app (no API call marks a BLAS as dead) keep their Metal object
    alive until a later build overlaps them, or the pool buffer is released.
  - Add telemetry: live entries, total private AS bytes, and evictions by overlap vs by
    release. Fold total private bytes into `QueryVideoMemoryInfo` usage from day one, not
    "if it matters". Otherwise the streaming budget logic is blind.
- **Map key.** Key on `(resource id, offset)`, or VA plus a resource generation. A bare VA is
  not enough, because placed resources can reuse a VA range after the old resource dies and
  before the deferred eviction runs.
  - Also evict on *creation* of a placed resource that overlaps a live entry's range, not only
    on build. D3D says aliased AS contents are undefined, so eviction is legal.

### 1.3 Copy, compaction, serialization (M1, M4)
- **Contributions.**
  - `CLONE`: blit `4·N` bytes from src's contributions to dst+64. Write the dst header with
    dst's own `addressOfInstanceContributions`.
  - `COMPACT`: same.
  - Refit with src≠dst: same (or rerun the conversion kernel, which already writes
    contributions).
  - Without this, dst silently aliases src's array. Once src's buffer is reused, hit-group
    indexing for dst is garbage.
  - Store `instance_count` in `AsEntry` (DXR.md already does).
- **Compacted size.** Encode `writeCompactedAccelerationStructureSize` into a private slot **in
  the same command buffer** as the app's postbuild emit.
  - The app can only record `COMPACT` after it has CPU-read *its* postbuild result, i.e. after
    that command buffer completed. So at `COMPACT` record time, check that command buffer's
    completion. If it's done, read the slot and allocate exactly. If not, fall back to a
    same-size copy.
  - This is never "one frame late" in practice, and removes a frame of double memory.
- **Reported compacted size** = `64 + 4N` (rounded to 256): correct, matching our
  `ResultDataMaxSizeInBytes`.

### 1.4 Ordering, barriers, queues
- **Single queue.** The header write and the build share a timeline. The header content
  (`resourceID`, contributions address) is known **at record time**, so write it with an
  upload-ring blit *before* the build. The app's own UAV barrier then orders it for consumers
  for free (M5).
  - The only internal barrier we must add: **conversion dispatch → TLAS build/refit**
    (Dispatch → AccelerationStructure stage).
  - Batch all TLAS conversions of one barrier interval into one dispatch, followed by one
    barrier.
- **Map a D3D UAV barrier on an AS** to `AccelerationStructure|Dispatch → AccelerationStructure|Dispatch`.
  - The intra-pass barrier is right [A exact MTL4 enum names: `MTLStageAccelerationStructure`,
    `MTLStageDispatch`].
  - Also add `Blit` to the source stages, because the header writes are blits.
- **Async compute.** If the game builds an AS on a compute queue, the CPU-side map update happens
  at *record* time, but the GPU contents become valid at *execute* time on another queue.
  - That's fine for the map, since objects are created at record time.
  - Header visibility across queues is covered by the app's fence → our `MTLSharedEvent`. Make
    sure that event wait covers the blit stage.
- **Refit of a TLAS.** Metal refits an instance AS by re-reading the instance buffer [A]. So the
  conversion kernel must also run before every TLAS **refit**, not only before builds.

---

## 2. Instance-conversion kernel

Correct in outline. Fix or test the following:

- **Null BLAS (M2).** D3D defines `AccelerationStructure == 0` as an inactive instance. Encode it
  as `mask = 0`, `accelerationStructureID = dummyBLAS.gpuResourceID` (a 1-degenerate-triangle
  BLAS kept resident forever), contribution 0.
  - Keep the array dense. Metal `instance_id` = array index, and MSC indexes
    `addressOfInstanceContributions[instance_id]` [A].
- **Garbage VAs.** A stale BLAS VA in an unused-but-counted descriptor faults the GPU when the
  kernel dereferences it. D3D has the same UB, but Metal faults are harder to debug.
  - In debug builds, validate: keep a GPU-visible sorted array of live header VAs, binary-search
    it, and write a poison flag on a miss.
- **Field mapping, to test against a CPU reference and with RayQuery readback:**
  - `InstanceID` (24 bits) → `userID`. `InstanceIndex()` → Metal `instance_id`.
    `InstanceID()` → `user_instance_id` [A MSC lowering. Test both intrinsics].
  - Transform: requires the MTL4 instance descriptor's `instanceTransformationMatrixLayout =
    RowMajor` [A, macOS 15+/MTL4]. Otherwise transpose into packed column-major.
  - Winding: D3D's default front face is clockwise in object space, *with D3D's left-handed ray
    convention*. Metal's default is clockwise too [A]. Test `HitKind()`/front-face with and
    without `TRIANGLE_FRONT_COUNTERCLOCKWISE` and a negative-determinant transform.
  - `FORCE_OPAQUE`/`FORCE_NON_OPAQUE` → `Opaque`/`NonOpaque` options.
  - `intersectionFunctionTableOffset = 0`. Fine for the 2-wrapper IFT, since the geometry offset
    selects the wrapper.
- **`ARRAY_OF_POINTERS`**: supported in the kernel; one indirection, also null-checked.
- MSC ships `IRRaytracingUpdateInstanceContributions(header, instDescs, index)` [V] as a GPU
  helper. Mirror its semantics exactly: index = TLAS array index.
- **Instance count is CPU-known**: correct, no indirect build needed. Size the converted buffer
  from the prebuild `NumDescs` and reuse it per TLAS.

---

## 3. MSC DispatchRays model (§5)

Header evidence [V], all from `ir_raytracing.h` and `metal_irconverter.h`:
- `kIRRayDispatchIndirectionKernelName = "RaygenIndirection"`, bind point 3.
- `IRDispatchRaysArgument` = { `IRDispatchRaysDescriptor`, `GRS`, `ResDescHeap`,
  `SmpDescHeap`, `VisibleFunctionTable`, `IntersectionFunctionTable`,
  `IntersectionFunctionTables` }.
- `IRShaderIdentifier` (32 B): `shaderHandle` is a VFT index. `intersectionShaderHandle` depends
  on the mode:
  - **VisibleFunction mode** → VFT index;
  - IntersectionFunction mode → IFT index;
  - **IntersectionFunctionBufferFunction mode** → "intersection function handle resource ID".
- Wrapper names: `irconverter.wrapper.intersection.function.triangle` / `.procedural`
  (`metal_irconverter_runtime.h:741-742`).
- `IRMetalLibSynthesizeIndirectIntersectionFunction` says codegen depends on the current hit-group
  type *and* RT config (`metal_irconverter.h:793`). So synthesize one wrapper per hit-group type
  per config, and put the config in the key.

**So §5's model = `IRRayGenerationCompilationVisibleFunction` +
`IRIntersectionFunctionCompilationVisibleFunction` + 2 wrappers + VFT indices.** It is
consistent with the headers. Notes:

1. **Consider IFB mode later.** `IntersectionFunctionBufferFunction` (Metal 4 intersection
   function buffers, macOS 26 [A]) is the native SBT-like path. It likely avoids wrapper → VFT
   double indirection. Don't block P3 on it, but keep the mode a single enum in
   `RayTracingConfig`.
2. **Intrinsic masks** must be identical across the pipeline (`metal_irconverter.h` ~611-696
   [V]). Start with the defaults (`IRIntrinsicMask*All`) for every shader. That avoids
   recompiling the whole pipeline on `AddToStateObject` (P4) and avoids cache-key explosion.
   Narrow the masks only once profiling says so.
3. **Recursion.** Pass the real `MaxTraceRecursionDepth` (`SetMaxRecursiveDepth`; default -1 =
   unlimited [V]). AW2's passes are almost certainly depth 1 [A].
4. **Opaque matters for performance.** The triangle wrapper runs only for non-opaque triangle
   hits [A]. If BLAS geometry flags or instance `FORCE_OPAQUE` are mistranslated, every hit pays
   a wrapper + VFT call.
5. **Local root signature layout** [A]: assume the D3D layout, verify with a test.
6. **Pipeline flags.** `SkipTriangles 0x100` / `SkipProcedural 0x200` are only for
   `RAYTRACING_PIPELINE_CONFIG1`. Also set `SetMaxAttributeSize` from the shader config.
7. `EnableDirectStateAccess` requires macOS 15+ [V comment]. Leave it off initially.

**Test-first order for P3**, each test being our own HLSL, compared with a CPU reference:
1. Raygen-only writing `DispatchRaysIndex` + a miss shader. This validates the indirection
   kernel, the arg buffer, SBT start/stride and the threadgroup shape.
2. Triangle hit group with closest-hit only, opaque.
3. Non-opaque triangle with any-hit `IgnoreHit`/`AcceptHitAndEndSearch` through the wrapper.
4. Procedural hit group: intersection + any-hit + `ReportHit` attributes up to the max attribute
   size.
5. Local root args: constants, a descriptor table and a root CBV in the record; plus static
   samplers via `localRootSignatureSamplersBuffer`.
6. Multiple raygen records selected by `RayGenerationShaderRecord.StartAddress`; per-instance
   `InstanceContributionToHitGroupIndex` plus `MultiplierForGeometryContributionToHitGroupIndex`.
7. Recursion depth 2 (closest-hit traces), and callable shaders.
8. Mixed intrinsic usage across shaders with All masks (no mismatch errors).

---

## 4. What the game actually requires, and phasing

### 4.1 Evidence [V]
- RT is gated on `OPTIONS5.RaytracingTier > 10`, i.e. 1.1 (`DAT_14397fc58+0xac`).
  - Tier 1.2 → `+0xad`.
  - SER requires SM 6.9 + `OPTIONS22` → `+0xae`.
  - Keep `HighestShaderModel` at 6.6 and `OPTIONS22` zeroed. The 642 `lib_6_9` raygens don't
    convert anyway.
- **Every RT pass except path tracing has both a DXR and an inline variant:**

  | Pass | Function | DXR variant | Inline variant |
  |---|---|---|---|
  | Sun shadow | `0x1422b08b0` | `rt_SunShadow` | `rt_SunShadowInline` |
  | Reflection | `0x1422c4ae0` | `rt_reflectionDeferredShading` | `rt_reflection_inline(_clusters)` |
  | Diffuse GI | `0x1422d86d0` | `rt_diffuseGIDeferredShading` | `rt_diffuseGIInline` |
  | ReSTIR shading | `0x1422b70e0` | `rt_restir_shading` | `rt_restir_shading_ray_query` |
  | Spot | `0x1423b6e10` | `rt_spot` | `rt_spot_inline` |
  | Transparent | `0x1422c7600` | `rt_transparent_reflection_pt` | `rt_transparent_reflection_inline` |
  | Path tracing | `0x1422eb2b0`/`0x1422eebe0`/`0x1422f0800` | `rt_pathTracing(_clusters)` | **none** |

- **The sun-shadow selector is a console variable that defaults to inline.**
  - The render-graph lambda (functor vtable `0x1433f6a38`, built at `0x1422b17b0`) captures the
    byte `DAT_14386a920` at `0x1422b16a8`.
  - `FUN_1422b08b0` takes the DXR path when the captured byte is 0 (`param_1+0x14`).
  - `DAT_14386a920` is the value field (+0x10) of a statically-initialized cvar object at
    `0x14386a910` (vtable `0x1430795f8`, registered into the cvar list at `0x1401b6a76`). Its
    static image value is `0x0101`, so the **default is 1 → `rt_SunShadowInline`**.
  - The other passes almost certainly use sibling cvars [A]. Verify each one the same way (find
    the `movzx r, byte [rip+…]` captured into the pass lambda) before declaring P3 optional for
    it.

### 4.2 Recommended phasing
1. **P1** AS (with M1–M5).
2. **P3-stub**:
   - `CreateStateObject` parses the subobjects and returns identifiers; no compilation needed
     yet. Optionally kick off background compiles.
   - `SetPipelineState1` + `DispatchRays` → log once and skip. Never crash.
   - This is required as soon as tier 1.1 is reported (M6).
3. **P2** inline:
   - Report tier 1.1 only behind `AW2_RAYTRACING=1` and only once the P2 fixture passes.
   - Then run the game with RT Low/Medium and confirm via trace which `*Inline`/`*ray_query`
     programs dispatch.
   - If any default pass selects DXR, force its cvar to inline via the game's config/console
     path as a stopgap, rather than pulling P3 forward.
4. **P3 full**, only for path-traced indirect lighting (the highest preset) and any pass whose
   cvar can't be forced. `rt_pathtracing` would need the `lib_6_5` variants. Whether the game
   picks a 6.5 PT variant when `+0xae` is false is [A]; confirm with a trace once P3-stub logs
   which state objects get created.
5. **P4** (collections, `AddToStateObject`, ExecuteIndirect `DispatchRays`): only on evidence.

---

## 5. Performance

- **Header blits.** Header writes per build cost nothing *if* there's no barrier per build.
  D3D apps batch N BLAS builds and then issue one UAV barrier.
  - Encode all builds back-to-back in one MTL4 compute encoder, with record-time header blits
    and no internal barriers between independent builds.
  - Inserting barriers per build serializes the GPU, and that would be the first big performance
    bug.
- **Allocation.**
  - Thousands of BLASes as individual `newAccelerationStructureWithSize:` allocations means
    thousands of residency-set entries and a commit per change.
  - Suballocate from `MTLHeap`s (`heapAccelerationStructureSizeAndAlignWithSize`,
    `newAccelerationStructureWithSize:offset:` [A MTL4 equivalents]) in size classes. Put the
    *heaps* in the residency set.
  - Commit the residency set at most once per submit.
- **Refit vs rebuild.** Honor `ALLOW_UPDATE`/`PERFORM_UPDATE` (refit) for skinned/animated
  BLASes and the per-frame TLAS. Honor `PREFER_FAST_BUILD`.
  - The `Refit` usage flag on a BLAS that is never refit costs trace performance on Apple GPUs
    [A]. So don't set it unless `ALLOW_UPDATE` is set.
  - Log per-frame counts (builds, refits, compactions, triangles) in the trace to spot rebuild
    storms.
- **Compaction.** Apply it to static BLASes as the app requests, with an exact size via 1.3. Free
  the uncompacted object on the deferred list.
- **Scratch.** max(build, refit) is fine. Check that Metal's scratch alignment requirement holds
  for the app's scratch VA (D3D guarantees 256 B) [A].
- **Hardware.** M1/M2 have no hardware RT, so default off there (agree with §6.6). On M3+, inline
  RayQuery in compute is the fast path anyway. That's another reason to prefer the inline
  variants.
- **Instance kernel.** One dispatch per barrier interval, covering all TLASes. Not per TLAS.

---

## 6. Smaller notes on DXR.md
- §3.1 "Entries are dropped… when another build targets an overlapping range": the overlap must
  use the *reported* size (256 B granularity) and the resource identity (1.2).
- §3.2 header write "a tiny compute pass, or a blit from an upload copy": prefer the blit, at
  record time, before the build (M5).
- §6.3 ordering argument: correct for one queue. Add the cross-queue note (1.4).
- §5 DispatchRays: `GRS` = compute root signature: correct for D3D. Use `dispatchThreads`
  (non-uniform) for W×H×D. Zero `IntersectionFunctionTables` (M7).
- §7: add the "P3-stub" item between items 4 and 6, and a header-encoder unit test against
  `IRRaytracingSetAccelerationStructure`.
