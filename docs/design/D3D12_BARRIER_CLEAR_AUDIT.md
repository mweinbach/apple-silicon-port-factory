# D3D12 -> Metal Barrier, Encoder Split, and Clear Audit (`eprof26`)

Date: 2026-10-30  
Scope: `crates/aw2-d3d12/src/metal/{cmdlist.rs,clear.rs,encstats.rs}`

## 1. Baseline Telemetry (`eprof26`, frames 25,200–26,400, ~191 fps)

```text
draw_pass=40.7 clear_pass=4.0 compute_enc=27.5
q_barrier_all=7.5 q_barrier_frag=61.7 enc_barrier=124.1
app_barrier=689.4 app_barrier_split=35.7
clear_folded=7.0 clear_deferred=38.0 clear_cs=28.0
qb_list=4.5 qb_clear_uav=2.0 qb_written=1.0 qb_frag_to_vtx=16.0 qb_masked_other=45.7
cs_mid_qb=14.7 cs_keep_gfx=176.5 rb_keep=17.0 rb_mid_qb=8.0 cs_defer_gfx=41.7
cp_dsv=3.0 cp_rtv_nows=1.0 cf_barrier_uav0=4.0
```

Exact accounting identities verified against `cmdlist.rs` and `eprof26.log`:
- **Deferred clears (`clear_deferred = 38.0`)**:
  `clear_folded (7.0) + clear_cs (27.0 deferred + 1.0 multi-slice direct) + clear_pass (4.0) = 38.0`.
- **Standalone clear passes (`clear_pass = 4.0`)**:
  `cp_dsv (3.0) + cp_rtv_nows (1.0) + cp_rtv_fmt (0.0) = 4.0`, and `cf_barrier_uav0 = 4.0` while `cf_close = cf_barrier_res = cf_view = cf_copy = cf_rect = cf_resolve = 0.0`.
- **Full queue barriers (`q_barrier_all = 7.5`)**:
  `qb_list (4.5) + qb_clear_uav (2.0) + q_resolve/internal (~1.0) = 7.5` (`qb_app_full = 0.0`).
- **Stage-masked encoder-opening queue barriers (`q_barrier_frag = 61.7`)**:
  `qb_frag_to_vtx (16.0) + qb_masked_other (45.7) = 61.7` (with `qb_written = 1.0` folded into `qb_masked_other` in `q_barrier_frag`).

---

## 2. Exact Root Causes

### 2.1 Why `clear_pass = 4.0` (`cp_dsv = 3.0`, `cp_rtv_nows = 1.0`) and `cf_barrier_uav0 = 4.0` survive deferral
1. **Flush trigger (`cf_barrier_uav0 = 4.0`)**:
   In `CommandList::barrier_list` (`cmdlist.rs`), any `D3D12_RESOURCE_BARRIER_TYPE_UAV` with `pResource == NULL` (`Barrier::Uav(0)`) sets `uav_all = true` and maps `flush_all_why = Some(&stats::c::cf_barrier_uav0)`, calling `self.flush_clears(why, |_| true)`.
   - The game records a batch of whole-subresource RTV/DSV clears up front (`clear_deferred = 38.0`), then issues compute dispatches separated by global null UAV barriers (`Barrier::Uav(0)`) *before* opening the later render passes that attach those cleared targets.
   - Because `uav_all` flushes *every* `PendingClear` unconditionally—even though deferred RTV/DSV clears are in `RENDER_TARGET` / `DEPTH_WRITE` state rather than `UNORDERED_ACCESS`—all unconsumed clears are evicted early instead of surviving to fold into `draw_encoder()` (`MTLLoadAction::Clear`).
2. **Why 4 clears become standalone render passes instead of batched compute (`clear_cs`)**:
   Inside `flush_clears()`:
   - **`cp_dsv = 3.0`**: `ClearKind::DepthStencil` is never eligible for `compute_clear_item()` because MSL kernels cannot `tex.write()` to `MTLPixelFormat::Depth32Float` / `Depth32Float_Stencil8` textures; Metal requires a render pass (`MTLLoadAction::Clear` on depth/stencil attachments).
   - **`cp_rtv_nows = 1.0`**: One colour target was created with `ALLOW_RENDER_TARGET` **without** `ALLOW_UNORDERED_ACCESS`, so `resource::texture_descriptor` did not set `MTLTextureUsage::ShaderWrite` (preserving Apple TBDR lossless bandwidth compression on pure render targets). Without `ShaderWrite`, `compute_clear_item` returns `None` and falls back to `encode_rtv_clear_pass`.
   - Note from commit `759ca1f`: if those 4 clears are never attached by a later draw pass in the *same* command list, keeping them across `Barrier::Uav(0)` moves their flush from `cf_barrier_uav0` to `cf_close` at `Close()`; any clear whose target *is* drawn later in the same command list now folds into `clear_folded` (`MTLLoadAction::Clear`) and eliminates its standalone render pass.

### 2.2 Why `qb_clear_uav = 2.0` fires every frame
- `CommandList::util_dispatch` (`cmdlist.rs`, called by `upscale_resource` and `clear_uav`) calls `self.need_full(FullWhy::ClearUav)` **twice** per invocation: once *before* `self.compute()` and once *after* the utility kernel dispatch.
- In `eprof26` (`1280x720 -> 1920x1080` FSR2 upscale path), `ffxDispatch` invokes `upscale_resource` once per frame (`1.0/frame`):
  1. If a compute encoder is already open, the pre-dispatch `need_full(ClearUav)` emits a full `MTLStages::All -> MTLStages::All` mid-encoder queue barrier + encoder barrier; if no compute encoder is open, it opens one with `qb_clear_uav` (+1).
  2. The post-dispatch `need_full(ClearUav)` forces the next render/compute encoder to open with a second `MTLStages::All -> MTLStages::All` queue barrier (`qb_clear_uav`, totaling `2.0/frame`), stalling vertex/tiler/dispatch overlap across the entire GPU pipeline.

### 2.3 Why `cs_mid_qb = 14.7`, `app_barrier_split = 35.7`, and `q_barrier_frag = 61.7` (`qb_frag_to_vtx = 16.0`, `qb_masked_other = 45.7`) fire so heavily
1. **Read-to-Read & No-Op D3D12 Transitions Are Treated as Full Execution Hazards**:
   - Out of `app_barrier = 689.4` barrier batches per frame, Remedy's render graph (`OrderSolver` / `GfxGraph`) transitions SRVs and depth buffers between read states (`PIXEL_SHADER_RESOURCE <-> NON_PIXEL_SHADER_RESOURCE`, `DEPTH_READ -> DEPTH_READ | NON_PIXEL_SHADER_RESOURCE`, `COPY_SOURCE <-> *_SHADER_RESOURCE`, etc.).
   - In D3D12, resources on unified Apple Silicon memory have **no layout transitions**—once a prior write-to-read barrier (`RENDER_TARGET -> PIXEL_SHADER_RESOURCE`, `UAV -> SRV`, `DEPTH_WRITE -> DEPTH_READ`) has completed, any subsequent read-to-read transition has **zero memory or execution hazard**.
   - Previously, `barrier_list` unconditionally accumulated `after |= state_stages(s0)` and `before |= state_stages(s1)` for every transition:
     - **`cs_mid_qb = 14.7`**: When a compute encoder opened after waiting only on `Dispatch` (`enc_sync_after = Dispatch`), any `PIXEL_SHADER_RESOURCE (0x80) -> NON_PIXEL_SHADER_RESOURCE (0x40)` transition added `after = Fragment` and `before = Dispatch`. Because `!enc_sync_after.contains(Fragment)`, the next dispatch emitted a mid-compute-encoder queue barrier waiting on `Fragment`!
     - **`app_barrier_split = 35.7`**: Inside an open draw pass, `render_barrier_keeps` rejected any transition touching an attached read-only DSV (`!self.gfx.dsv.is_some_and(|v| v.resource == r)` — e.g. `DEPTH_READ -> DEPTH_READ | NON_PIXEL_SHADER_RESOURCE` during lighting/decal passes), forcing `end_encoder()` and splitting the TBDR render pass.
     - **`qb_frag_to_vtx = 16.0` & `qb_masked_other = 45.7`**: Read-to-read transitions from `PIXEL_SHADER_RESOURCE` (`Fragment`) or `NON_PIXEL_SHADER_RESOURCE` (`Vertex|Object|Mesh|Dispatch|Accel`) inflated `pend_after` and `pend_before` across encoder boundaries:
       - `qb_masked_other` (`45.7/frame`) counts every stage-masked encoder-opening barrier that is not `Fragment -> PreRaster` (`Dispatch -> Fragment/PreRaster`, `Fragment -> Dispatch`, `Fragment -> Fragment`, `Blit -> *`). Notice `cs_keep_gfx = 176.5` and `cs_defer_gfx = 41.7` feed graphics stage masks into the next render pass's `begin_barrier`.
       - `qb_frag_to_vtx` (`16.0/frame`) fires whenever `after` contains `Fragment` and `before` contains hard `PRE_RASTER` bits—specifically transitions into `VERTEX_AND_CONSTANT_BUFFER (0x1)`, `INDEX_BUFFER (0x2)`, or `INDIRECT_ARGUMENT (0x200)` (which bypass `pend_before_soft`), or `ExecuteIndirect` expansion (`produced(Dispatch, PreRaster | Fragment)` combined with an unconsumed `pend_after = Fragment`).
2. **`qb_written = 1.0` Precision**:
   - `self.written` tracks render targets / writable DSVs written since the last barrier that waited on `MTLStages::Fragment`. Because nearly every render pass already opens with an explicit D3D12 barrier containing `Fragment` (`q_barrier_frag = 61.7`), `self.written` is cleared almost every pass and only catches `1.0/frame` same-state RTV/DSV attachment reuse without an explicit barrier.
3. **`state_stages` Broadness**:
   - `VERTEX_AND_CONSTANT_BUFFER (0x1)` maps to `Vertex | Object | Mesh | Dispatch | Fragment` (`shader`) because D3D12 merges VB and CBV into one state bit, and `INDIRECT_ARGUMENT (0x200)` maps to `Vertex | Object | Mesh | Dispatch` (`pre_raster`), both bypassing `pend_before_soft` (`SRV_UAV = 0x8 | 0x40`). Whenever a read-only CBV/VB/indirect buffer transitions to another read state (e.g. `GENERIC_READ` or `NON_PIXEL_SHADER_RESOURCE`), it previously injected `Fragment | PreRaster | Dispatch` into `pend_after`! Eliding read-to-read transitions removes that false `Fragment` / `PreRaster` stage pollution at the source.

---

## 3. Implemented Default-Off (`AW2_*`) Optimizations

Both optimizations are **default OFF** so concurrent CPU benchmarks remain undisturbed until dedicated GPU A/B runs:

### Optimization 1: `AW2_BARRIER_R2R_ELIDE=1`
- **What it does**:
  Elides no-op transitions (`s0 == s1`) and pure read-to-read transitions (`is_read_to_read_transition(s0, s1)`) in `CommandList::barrier_list`, `render_barrier_keeps`, and intra-compute-encoder `ib_note` tracking.
- **Exact hazard safety rules**:
  - Neither `s0` nor `s1` may be `0` (`COMMON` / `PRESENT`, which can alias queue/copy/present promotion/decay).
  - Neither `s0` nor `s1` may contain any write-capable D3D12 state bit (`RENDER_TARGET 0x4`, `UNORDERED_ACCESS 0x8`, `DEPTH_WRITE 0x10`, `STREAM_OUT 0x100`, `COPY_DEST 0x400`, `RESOLVE_DEST 0x1000`, video write states) or unknown reserved bits.
  - **Deferred clear guardrail**: If `self.pending_clears` holds a deferred RTV/DSV clear for resource `r`, the transition is **never** elided, ensuring any pending clear is flushed before subsequent readers.
  - Inside an open draw pass (`render_barrier_keeps`), an elided read-to-read transition on an attached read-only DSV (e.g. `DEPTH_READ -> DEPTH_READ | NON_PIXEL_SHADER_RESOURCE`) no longer splits the TBDR render pass (`app_barrier_split`).
- **New `AW2_ENC_STATS` counters** (counted even when the knob is off so baseline runs measure exact opportunity):
  - `rb_r2r`: pure read-to-read transition items seen per frame.
  - `rb_r2r_elided`: read-to-read transition items elided per frame.
  - `rb_noop`: `s0 == s1` transition items seen per frame.

### Optimization 2: `AW2_BARRIER_TIGHT_STAGES=1`
- **What it does**:
  1. **Stage-masked utility dispatches (`upscale_resource` & `clear_uav`)**: Replaces the two `need_full(FullWhy::ClearUav)` (`MTLStages::All -> MTLStages::All`) barriers in `util_dispatch_with` with stage-masked `pend_after` / `pend_before` (`Shader|Blit -> Dispatch` before, `Dispatch -> Shader|Blit` after) plus per-resource intra-compute-encoder tracking (`ib_note(r)` and `compute_touch(Some(touch))`). Eliminates `qb_clear_uav = 2.0` full pipeline stalls per frame while preserving 100% same-state UAV write->write and write->read ordering.
  2. **Keep deferred RTV/DSV clears across null `Barrier::Uav(0)`**: Stops `Barrier::Uav(0)` from force-flushing `self.pending_clears` (`cf_barrier_uav0 = 4.0/frame`). Deferred clears represent `ClearRenderTargetView` / `ClearDepthStencilView` on resources in `RENDER_TARGET` / `DEPTH_WRITE` state—not `UNORDERED_ACCESS`—and already flush on any transition of `r`, `clear_uav(r)`, `copy_*(r)`, `upscale_resource(r)`, `resolve_subresource(r)`, conflicting view draw, or `Close()`. Surviving `Barrier::Uav(0)` lets deferred DSV and non-ShaderWrite RTV clears fold into `draw_encoder()` (`MTLLoadAction::Clear`) instead of opening standalone `cp_dsv` / `cp_rtv_nows` clear render passes.
- **New `AW2_ENC_STATS` counters**:
  - `qb_upscale_masked`: utility dispatches ordered via stage-masked barriers instead of `qb_clear_uav`.
  - `cf_uav0_kept`: `Barrier::Uav(0)` calls where non-empty `pending_clears` were preserved instead of flushed.

---

## 4. 3-Pair Interleaved A/B Benchmark Commands

Once the `launchd` benchmark queue is free:

1. **Build release loader and save binary for reproducible A/B**:
   ```sh
   cargo build --release -p aw2-loader
   cp target/release/aw2-loader target/review/bins/bclear1
   ```

2. **Quick encoder-stats + correctness gate check (`AW2_ENC_STATS=1`)**:
   ```sh
   # Verify rb_r2r, rb_r2r_elided, qb_clear_uav -> 0, cf_barrier_uav0 -> 0, clear_pass reduction, errors: 0, black% 0
   target/review/ab_run.sh bclear1 bclear1_diag AW2_ENC_STATS=1 AW2_BARRIER_R2R_ELIDE=1 AW2_BARRIER_TIGHT_STAGES=1
   python3 target/review/pngstat.py target/review/cap_bclear1_diag_24000.png
   grep "enc/frame:" target/review/bclear1_diag.log | tail -n 5
   ```

3. **3-Pair Interleaved Gameplay FPS A/B (1280x720 render & native 2560x1440 GPU-bound)**:
   ```sh
   # Pin resolution per protocol before each pair (target/review/seq_rr.sh pattern), run via lrun.sh:
   # Pair i = 1..3:
   #   Baseline (off): target/review/ab_run.sh bclear1 bc_off_${i}
   #   R2R only:       target/review/ab_run.sh bclear1 bc_r2r_${i}   AW2_BARRIER_R2R_ELIDE=1
   #   Combined:       target/review/ab_run.sh bclear1 bc_both_${i}  AW2_BARRIER_R2R_ELIDE=1 AW2_BARRIER_TIGHT_STAGES=1

   python3 target/review/fpstime.py \
     target/review/bc_off_{1,2,3}.log \
     target/review/bc_r2r_{1,2,3}.log \
     target/review/bc_both_{1,2,3}.log \
     --from 125 --to 230 --step 15
   ```
