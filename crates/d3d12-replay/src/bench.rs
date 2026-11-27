//! Repeatable GPU/frame benchmarking for `d3d12-replay --backend metal --bench`.
//!
//! Splits a trace into its one-time object setup prefix and its repeatable frame
//! body, warms up GPU/PSO caches, and measures `gpu_busy_ms` (hardware Metal 4
//! `MTL4CounterHeap` timestamps across executed command buffers) and `frame_ms`
//! (wall-clock iteration duration) over `N` steady-state iterations.

use std::collections::HashSet;
use std::time::Instant;

use d3d12_metal::trace::{Call, Reader, Record};

use crate::metal_backend::{MetalBackend, DEFAULT_MAX_TIMESTAMPS};
use crate::replay::{Backend, Handled, Objects};

#[derive(Debug, Clone)]
pub struct BenchOptions {
    pub iterations: usize,
    pub warmup: usize,
    pub until: Option<u64>,
}

impl Default for BenchOptions {
    fn default() -> Self {
        Self { iterations: 20, warmup: 5, until: None }
    }
}

/// Summary statistics for a series of timing samples (milliseconds).
#[derive(Debug, Clone, Default)]
pub struct TimingStats {
    pub samples: Vec<f64>,
    pub min_ms: f64,
    pub median_ms: f64,
    pub mean_ms: f64,
    pub p95_ms: f64,
    pub max_ms: f64,
    pub stddev_ms: f64,
    /// Coefficient of variation (`stddev / mean * 100.0`), in percent, over all samples.
    pub cv_pct: f64,
    /// Robust coefficient of variation (`1.4826 * MAD / median * 100.0`), in percent,
    /// matching Gaussian CV while remaining resilient to isolated OS/WindowServer GPU preemptions.
    pub mad_cv_pct: f64,
}

impl TimingStats {
    pub fn from_samples(samples: Vec<f64>) -> Self {
        if samples.is_empty() {
            return Self::default();
        }
        let mut sorted = samples.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let n = sorted.len();
        let min_ms = sorted[0];
        let max_ms = sorted[n - 1];
        let median_ms = Self::median_sorted(&sorted);
        let p95_idx = ((n as f64 * 0.95).ceil() as usize).saturating_sub(1).min(n - 1);
        let p95_ms = sorted[p95_idx];
        let mean_ms = sorted.iter().sum::<f64>() / n as f64;
        let variance = if n > 1 {
            sorted.iter().map(|&x| (x - mean_ms).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };
        let stddev_ms = variance.sqrt();
        let cv_pct = if mean_ms > 0.0 { (stddev_ms / mean_ms) * 100.0 } else { 0.0 };
        let mut abs_devs: Vec<f64> = sorted.iter().map(|&x| (x - median_ms).abs()).collect();
        abs_devs.sort_by(|a, b| a.total_cmp(b));
        let mad_ms = Self::median_sorted(&abs_devs);
        let mad_cv_pct = if median_ms > 0.0 { (1.4826 * mad_ms / median_ms) * 100.0 } else { 0.0 };
        Self { samples, min_ms, median_ms, mean_ms, p95_ms, max_ms, stddev_ms, cv_pct, mad_cv_pct }
    }

    fn median_sorted(sorted: &[f64]) -> f64 {
        let n = sorted.len();
        if n == 0 {
            0.0
        } else if n % 2 == 1 {
            sorted[n / 2]
        } else {
            0.5 * (sorted[n / 2 - 1] + sorted[n / 2])
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchReport {
    pub warmup: usize,
    pub iterations: usize,
    pub setup_records: usize,
    pub frame_records: usize,
    pub gpu_busy: TimingStats,
    pub frame_time: TimingStats,
    pub failures: Vec<String>,
}

impl std::fmt::Display for BenchReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "bench (metal backend): {} warmup + {} iterations (setup {} records, frame {} records)",
            self.warmup, self.iterations, self.setup_records, self.frame_records
        )?;
        writeln!(
            f,
            "  gpu_busy_ms : min={:.3} median={:.3} mean={:.3} p95={:.3} max={:.3} stddev={:.3} cv={:.2}% mad_cv={:.2}%",
            self.gpu_busy.min_ms,
            self.gpu_busy.median_ms,
            self.gpu_busy.mean_ms,
            self.gpu_busy.p95_ms,
            self.gpu_busy.max_ms,
            self.gpu_busy.stddev_ms,
            self.gpu_busy.cv_pct,
            self.gpu_busy.mad_cv_pct
        )?;
        writeln!(
            f,
            "  frame_ms    : min={:.3} median={:.3} mean={:.3} p95={:.3} max={:.3} stddev={:.3} cv={:.2}% mad_cv={:.2}%",
            self.frame_time.min_ms,
            self.frame_time.median_ms,
            self.frame_time.mean_ms,
            self.frame_time.p95_ms,
            self.frame_time.max_ms,
            self.frame_time.stddev_ms,
            self.frame_time.cv_pct,
            self.frame_time.mad_cv_pct
        )?;
        if !self.failures.is_empty() {
            writeln!(f, "  failures: {}", self.failures.len())?;
            for m in self.failures.iter().take(20) {
                writeln!(f, "    {m}")?;
            }
        }
        Ok(())
    }
}

fn is_setup_call(c: &Call) -> bool {
    matches!(
        c,
        Call::CreateDevice { .. }
            | Call::CreateCommandQueue { .. }
            | Call::CreateCommandAllocator { .. }
            | Call::CreateCommandList { .. }
            | Call::CreateDescriptorHeap { .. }
            | Call::CreateHeap { .. }
            | Call::CreateCommittedResource { .. }
            | Call::CreatePlacedResource { .. }
            | Call::CreateReservedResource { .. }
            | Call::CreateRootSignature { .. }
            | Call::CreateGraphicsPipelineState { .. }
            | Call::CreateComputePipelineState { .. }
            | Call::CreateMeshPipelineState { .. }
            | Call::CreateStateObject { .. }
            | Call::AddToStateObject { .. }
            | Call::CreateCommandSignature { .. }
            | Call::CreateFence { .. }
            | Call::CreateQueryHeap { .. }
            | Call::CreateSwapChain { .. }
            | Call::GetSwapChainBuffer { .. }
            | Call::WriteResource { .. }
            | Call::CreateShaderResourceView { .. }
            | Call::CreateUnorderedAccessView { .. }
            | Call::CreateRenderTargetView { .. }
            | Call::CreateDepthStencilView { .. }
            | Call::CreateConstantBufferView { .. }
            | Call::CreateSampler { .. }
    )
}

/// Partition `records` into `(frame_start, frame_end, reopen_lists)`:
/// - Multi-frame trace (>= 2 `Present`/`FrameEnd`): loop the last complete frame
///   `[prev_boundary..last_boundary]`, excluding any trailing partial frame or teardown records.
/// - Single-frame trace with 1 `Present`/`FrameEnd`: bound `frame_end` at that boundary so
///   trailing `Release` teardown records never run inside the repeated loop, and start at the
///   earliest non-setup call (ignoring empty setup `CreateCommandList`+`Close` pairs) so earlier
///   recording on another list is never skipped when a later list calls `Reset`.
/// - Traces with no `Present`/`FrameEnd`: trim trailing `Release` teardown calls and apply the
///   same earliest non-setup start rule.
/// - Returns an error if any command list used inside `[frame_start..frame_end]` without a
///   leading `Reset` already had commands recorded into it before `frame_start` (which would be
///   silently lost when `prepare_bench_iteration` reopens the list on subsequent iterations).
pub(crate) fn find_frame_slice(records: &[Record]) -> anyhow::Result<(usize, usize, Vec<u64>)> {
    let mut frame_boundaries: Vec<usize> = Vec::new();
    for (i, r) in records.iter().enumerate() {
        if matches!(r.call, Call::FrameEnd { .. }) {
            frame_boundaries.push(i + 1);
        }
    }
    if frame_boundaries.is_empty() {
        for (i, r) in records.iter().enumerate() {
            if matches!(r.call, Call::Present { .. }) {
                frame_boundaries.push(i + 1);
            }
        }
    }
    let frame_end = if let Some(&last) = frame_boundaries.last() {
        last
    } else {
        let mut end = records.len();
        while end > 0 && matches!(records[end - 1].call, Call::Release { .. }) {
            end -= 1;
        }
        end
    };
    let frame_start = if frame_boundaries.len() >= 2 {
        frame_boundaries[frame_boundaries.len() - 2]
    } else {
        // Earliest non-setup call before `frame_end`, skipping only empty `Close` calls that
        // immediately close a freshly created list with no commands recorded on it.
        let mut recorded_lists: HashSet<u64> = HashSet::new();
        let mut start = frame_end;
        for (i, r) in records[..frame_end].iter().enumerate() {
            if is_setup_call(&r.call) {
                continue;
            }
            if let Call::Close { list } = r.call {
                if !recorded_lists.contains(&list) {
                    continue;
                }
            }
            if let Some(list) = record_list_id(&r.call) {
                if !matches!(r.call, Call::Close { .. }) {
                    recorded_lists.insert(list);
                }
            }
            start = i;
            break;
        }
        start
    };

    // Track which lists carry unclosed recorded commands across `frame_start`.
    let mut dirty_open_before_start: HashSet<u64> = HashSet::new();
    for r in &records[..frame_start] {
        match r.call {
            Call::Reset { list, .. } | Call::Close { list } => {
                dirty_open_before_start.remove(&list);
            }
            _ => {
                if let Some(list) = record_list_id(&r.call) {
                    dirty_open_before_start.insert(list);
                }
            }
        }
    }

    // Determine which command lists are used inside `records[frame_start..frame_end]` without
    // first being `Reset` inside the slice, so `prepare_bench_iteration` re-opens them cleanly.
    let mut reset_first: HashSet<u64> = HashSet::new();
    let mut needs_reopen: Vec<u64> = Vec::new();
    let mut seen_reopen: HashSet<u64> = HashSet::new();
    for r in &records[frame_start..frame_end] {
        if let Call::Reset { list, .. } = r.call {
            reset_first.insert(list);
            continue;
        }
        if let Some(list) = record_list_id(&r.call) {
            if !reset_first.contains(&list) && seen_reopen.insert(list) {
                if dirty_open_before_start.contains(&list) {
                    anyhow::bail!(
                        "cannot safely benchmark frame slice [{frame_start}..{frame_end}]: \
                         command list {list:#x} has commands recorded before frame_start and is \
                         used without Reset inside the frame"
                    );
                }
                needs_reopen.push(list);
            }
        }
    }
    Ok((frame_start, frame_end, needs_reopen))
}

fn record_list_id(c: &Call) -> Option<u64> {
    Some(match *c {
        Call::Close { list }
        | Call::ClearState { list, .. }
        | Call::SetPredication { list, .. }
        | Call::ResourceBarrier { list, .. }
        | Call::SetPipelineState { list, .. }
        | Call::SetGraphicsRootSignature { list, .. }
        | Call::SetComputeRootSignature { list, .. }
        | Call::SetDescriptorHeaps { list, .. }
        | Call::SetRootDescriptorTable { list, .. }
        | Call::SetRoot32BitConstants { list, .. }
        | Call::SetRootView { list, .. }
        | Call::IASetPrimitiveTopology { list, .. }
        | Call::IASetVertexBuffers { list, .. }
        | Call::IASetIndexBuffer { list, .. }
        | Call::RSSetViewports { list, .. }
        | Call::RSSetScissorRects { list, .. }
        | Call::OMSetRenderTargets { list, .. }
        | Call::OMSetBlendFactor { list, .. }
        | Call::OMSetStencilRef { list, .. }
        | Call::OMSetFrontAndBackStencilRef { list, .. }
        | Call::OMSetDepthBounds { list, .. }
        | Call::ClearRenderTargetView { list, .. }
        | Call::ClearDepthStencilView { list, .. }
        | Call::ClearUnorderedAccessViewUint { list, .. }
        | Call::ClearUnorderedAccessViewFloat { list, .. }
        | Call::DiscardResource { list, .. }
        | Call::DrawInstanced { list, .. }
        | Call::DrawIndexedInstanced { list, .. }
        | Call::Dispatch { list, .. }
        | Call::DispatchMesh { list, .. }
        | Call::ExecuteIndirect { list, .. }
        | Call::CopyBufferRegion { list, .. }
        | Call::CopyTextureRegion { list, .. }
        | Call::CopyResource { list, .. }
        | Call::ResolveSubresource { list, .. }
        | Call::BeginQuery { list, .. }
        | Call::EndQuery { list, .. }
        | Call::ResolveQueryData { list, .. } => list,
        _ => return None,
    })
}

/// Run a Metal 4 benchmark on `data`, returning `(BenchReport, MetalBackend)` so callers can
/// optionally run `--golden` readback on the final rendered frame without replaying twice.
pub fn run_bench(data: &[u8], opts: &BenchOptions) -> anyhow::Result<(BenchReport, MetalBackend)> {
    run_bench_with_timer_capacity(data, opts, DEFAULT_MAX_TIMESTAMPS)
}

pub(crate) fn run_bench_with_timer_capacity(
    data: &[u8],
    opts: &BenchOptions,
    timer_capacity: u32,
) -> anyhow::Result<(BenchReport, MetalBackend)> {
    let reader = Reader::new(data).map_err(|e| anyhow::anyhow!("{}", e.0))?;
    let mut records = Vec::new();
    for r in reader {
        let rec = r.map_err(|e| anyhow::anyhow!("{}", e.0))?;
        if opts.until.is_some_and(|u| rec.index > u) {
            break;
        }
        records.push(rec);
    }
    if records.is_empty() {
        anyhow::bail!("trace contains no records");
    }

    let (frame_start, frame_end, reopen_lists) = find_frame_slice(&records)?;
    let mut backend = MetalBackend::new().map_err(|e| anyhow::anyhow!(e))?;
    let objs = Objects::default();
    let mut failures = Vec::new();

    // 1. Setup prefix (run once, untimed). Reject unsupported skipped calls honestly
    //    so `--bench` never reports timings for an incomplete workload.
    for rec in &records[..frame_start] {
        match backend.call(rec, &objs) {
            Ok(Handled::Done) => {}
            Ok(Handled::Skipped) => {
                anyhow::bail!(
                    "cannot benchmark incomplete workload: unsupported setup call #{} {} was skipped by metal backend",
                    rec.index,
                    rec.call.name()
                );
            }
            Err(e) => failures.push(format!("setup #{} {}: {e}", rec.index, rec.call.name())),
        }
    }
    let blocked = backend.blocked_queues();
    if !blocked.is_empty() {
        anyhow::bail!(
            "cannot benchmark trace: setup prefix [0..{frame_start}] leaves queue(s) {blocked:?} \
             blocked on fence signals not produced before frame_start"
        );
    }
    backend.wait_all_idle();
    let swapchain_indices = backend.snapshot_swapchain_indices();
    let fence_values = backend.snapshot_fence_values();

    // 2. Arm hardware GPU timing BEFORE warmup so the `MTL4CounterHeap` and timestamp
    //    paths are resident and warm before measured iteration 0 begins.
    backend
        .enable_gpu_timing_with_capacity(true, timer_capacity)
        .map_err(|e| anyhow::anyhow!(e))?;
    let mut max_slots_per_iter = 2u32;
    for iter in 0..opts.warmup {
        backend.reset_gpu_timer();
        objc2::rc::autoreleasepool(|_| -> anyhow::Result<()> {
            backend
                .prepare_bench_iteration(&reopen_lists, &swapchain_indices, &fence_values)
                .map_err(|e| anyhow::anyhow!("prepare warmup {iter}: {e}"))?;
            for rec in &records[frame_start..frame_end] {
                match backend.call(rec, &objs) {
                    Ok(Handled::Done) => {}
                    Ok(Handled::Skipped) => {
                        anyhow::bail!(
                            "cannot benchmark incomplete workload: unsupported frame call #{} {} was skipped by metal backend",
                            rec.index,
                            rec.call.name()
                        );
                    }
                    Err(e) if e.contains("repeated submission of closed command list") => {
                        anyhow::bail!("warmup {iter} #{} {}: {e}", rec.index, rec.call.name());
                    }
                    Err(e) => failures.push(format!("warmup {iter} #{} {}: {e}", rec.index, rec.call.name())),
                }
            }
            let blocked = backend.blocked_queues();
            if !blocked.is_empty() {
                anyhow::bail!(
                    "cannot benchmark trace: warmup {iter} leaves queue(s) {blocked:?} \
                     blocked on unsignaled cross-frame fence waits"
                );
            }
            backend.wait_all_idle();
            Ok(())
        })?;
        max_slots_per_iter = max_slots_per_iter.max(backend.gpu_timer_used_slots());
    }

    // 3. Measured steady-state iterations: accumulate hardware GPU timestamps across
    //    iterations and flush/recycle in bounded batches only if the counter heap would
    //    otherwise run out of slots, so normal runs resolve once at the end while huge
    //    multi-list traces never drop timestamps or report silent 0 ms samples.
    let measured = opts.iterations.max(1);
    backend.reset_gpu_timer();
    let mut frame_samples = Vec::with_capacity(measured);
    let mut gpu_samples = Vec::with_capacity(measured);
    let mut pair_ranges = Vec::with_capacity(measured);

    for iter in 0..measured {
        if !pair_ranges.is_empty() && backend.gpu_timer_remaining_slots() < max_slots_per_iter {
            let batch = backend
                .resolve_bench_gpu_busy_ms(&pair_ranges)
                .map_err(|e| anyhow::anyhow!("resolve gpu timestamps batch: {e}"))?;
            gpu_samples.extend(batch);
            pair_ranges.clear();
            backend.reset_gpu_timer();
        }
        let slots_before = backend.gpu_timer_used_slots();
        let (wall_ms, pair_lo, pair_hi) = objc2::rc::autoreleasepool(|_| -> anyhow::Result<(f64, usize, usize)> {
            backend
                .prepare_bench_iteration(&reopen_lists, &swapchain_indices, &fence_values)
                .map_err(|e| anyhow::anyhow!("prepare iteration {iter}: {e}"))?;
            let pair_lo = backend.gpu_timer_pair_count();
            let t0 = Instant::now();
            for rec in &records[frame_start..frame_end] {
                match backend.call(rec, &objs) {
                    Ok(Handled::Done) => {}
                    Ok(Handled::Skipped) => {
                        anyhow::bail!(
                            "cannot benchmark incomplete workload: unsupported frame call #{} {} was skipped by metal backend",
                            rec.index,
                            rec.call.name()
                        );
                    }
                    Err(e) if e.contains("repeated submission of closed command list") => {
                        anyhow::bail!("iter {iter} #{} {}: {e}", rec.index, rec.call.name());
                    }
                    Err(e) => failures.push(format!("iter {iter} #{} {}: {e}", rec.index, rec.call.name())),
                }
            }
            let blocked = backend.blocked_queues();
            if !blocked.is_empty() {
                anyhow::bail!(
                    "cannot benchmark trace: iteration {iter} leaves queue(s) {blocked:?} \
                     blocked on unsignaled cross-frame fence waits"
                );
            }
            backend.wait_all_idle();
            let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
            let pair_hi = backend.gpu_timer_pair_count();
            Ok((wall_ms, pair_lo, pair_hi))
        })?;
        let slots_used = backend.gpu_timer_used_slots().saturating_sub(slots_before);
        max_slots_per_iter = max_slots_per_iter.max(slots_used);
        pair_ranges.push((pair_lo, pair_hi));
        frame_samples.push(wall_ms);
    }

    if !pair_ranges.is_empty() {
        let batch = backend
            .resolve_bench_gpu_busy_ms(&pair_ranges)
            .map_err(|e| anyhow::anyhow!("resolve gpu timestamps: {e}"))?;
        gpu_samples.extend(batch);
    }
    backend.enable_gpu_timing(false).map_err(|e| anyhow::anyhow!(e))?;

    let report = BenchReport {
        warmup: opts.warmup,
        iterations: measured,
        setup_records: frame_start,
        frame_records: frame_end.saturating_sub(frame_start),
        gpu_busy: TimingStats::from_samples(gpu_samples),
        frame_time: TimingStats::from_samples(frame_samples),
        failures,
    };
    Ok((report, backend))
}
