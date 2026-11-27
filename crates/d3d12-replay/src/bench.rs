//! Repeatable GPU/frame benchmarking for `d3d12-replay --backend metal --bench`.
//!
//! Splits a trace into its one-time object setup prefix and its repeatable frame
//! body, warms up GPU/PSO caches, and measures `gpu_busy_ms` (hardware Metal 4
//! `MTL4CounterHeap` timestamps across executed command buffers) and `frame_ms`
//! (wall-clock iteration duration) over `N` steady-state iterations.

use std::collections::HashSet;
use std::time::Instant;

use d3d12_metal::trace::{Call, Reader, Record};

use crate::metal_backend::MetalBackend;
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

/// Partition `records` into `(setup_end, reopen_lists)`:
/// - Multi-frame trace (>= 2 `Present`/`FrameEnd`): loop the last complete frame.
/// - Single-frame trace with an explicit `Reset`: loop starting at the first `Reset`
///   (after any initial `CreateCommandList` + `Close` setup).
/// - Otherwise: loop starting at the first non-setup call, reopening any command list
///   that was left open by setup.
fn find_frame_slice(records: &[Record]) -> (usize, Vec<u64>) {
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
    let frame_start = if frame_boundaries.len() >= 2 {
        frame_boundaries[frame_boundaries.len() - 2]
    } else if let Some(first_reset) = records.iter().position(|r| matches!(r.call, Call::Reset { .. })) {
        first_reset
    } else {
        records.iter().position(|r| !is_setup_call(&r.call)).unwrap_or(0)
    };

    // Determine which command lists are used inside `records[frame_start..]` without first
    // being `Reset` inside the slice, so `prepare_bench_iteration` re-opens them cleanly.
    let mut reset_first: HashSet<u64> = HashSet::new();
    let mut needs_reopen: Vec<u64> = Vec::new();
    let mut seen_reopen: HashSet<u64> = HashSet::new();
    for r in &records[frame_start..] {
        if let Call::Reset { list, .. } = r.call {
            reset_first.insert(list);
            continue;
        }
        if let Some(list) = record_list_id(&r.call) {
            if !reset_first.contains(&list) && seen_reopen.insert(list) {
                needs_reopen.push(list);
            }
        }
    }
    (frame_start, needs_reopen)
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

    let (frame_start, reopen_lists) = find_frame_slice(&records);
    let mut backend = MetalBackend::new().map_err(|e| anyhow::anyhow!(e))?;
    let objs = Objects::default();
    let mut failures = Vec::new();

    // 1. Setup prefix (run once, untimed).
    for rec in &records[..frame_start] {
        if let Err(e) = backend.call(rec, &objs) {
            failures.push(format!("setup #{} {}: {e}", rec.index, rec.call.name()));
        }
    }
    backend.wait_all_idle();
    let swapchain_indices = backend.snapshot_swapchain_indices();

    // 2. Arm hardware GPU timing BEFORE warmup so the `MTL4CounterHeap` and timestamp
    //    paths are resident and warm before measured iteration 0 begins.
    backend.enable_gpu_timing(true).map_err(|e| anyhow::anyhow!(e))?;
    for iter in 0..opts.warmup {
        backend.reset_gpu_timer();
        objc2::rc::autoreleasepool(|_| -> anyhow::Result<()> {
            backend
                .prepare_bench_iteration(&reopen_lists, &swapchain_indices)
                .map_err(|e| anyhow::anyhow!("prepare warmup {iter}: {e}"))?;
            for rec in &records[frame_start..] {
                match backend.call(rec, &objs) {
                    Ok(Handled::Done | Handled::Skipped) => {}
                    Err(e) => failures.push(format!("warmup {iter} #{} {}: {e}", rec.index, rec.call.name())),
                }
            }
            backend.wait_all_idle();
            Ok(())
        })?;
    }

    // 3. Measured steady-state iterations: accumulate hardware GPU timestamps in one
    //    `MTL4CounterHeap` across all iterations and resolve once at the end so mid-loop
    //    counter resolves never perturb GPU frequency or inter-frame timing.
    let measured = opts.iterations.max(1);
    backend.reset_gpu_timer();
    let mut frame_samples = Vec::with_capacity(measured);
    let mut pair_ranges = Vec::with_capacity(measured);

    for iter in 0..measured {
        let (wall_ms, pair_lo, pair_hi) = objc2::rc::autoreleasepool(|_| -> anyhow::Result<(f64, usize, usize)> {
            backend
                .prepare_bench_iteration(&reopen_lists, &swapchain_indices)
                .map_err(|e| anyhow::anyhow!("prepare iteration {iter}: {e}"))?;
            let pair_lo = backend.gpu_timer_pair_count();
            let t0 = Instant::now();
            for rec in &records[frame_start..] {
                match backend.call(rec, &objs) {
                    Ok(Handled::Done | Handled::Skipped) => {}
                    Err(e) => failures.push(format!("iter {iter} #{} {}: {e}", rec.index, rec.call.name())),
                }
            }
            backend.wait_all_idle();
            let wall_ms = t0.elapsed().as_secs_f64() * 1000.0;
            let pair_hi = backend.gpu_timer_pair_count();
            Ok((wall_ms, pair_lo, pair_hi))
        })?;
        pair_ranges.push((pair_lo, pair_hi));
        frame_samples.push(wall_ms);
    }

    let gpu_samples = backend
        .resolve_bench_gpu_busy_ms(&pair_ranges)
        .map_err(|e| anyhow::anyhow!("resolve gpu timestamps: {e}"))?;
    backend.enable_gpu_timing(false).map_err(|e| anyhow::anyhow!(e))?;

    let report = BenchReport {
        warmup: opts.warmup,
        iterations: measured,
        setup_records: frame_start,
        frame_records: records.len() - frame_start,
        gpu_busy: TimingStats::from_samples(gpu_samples),
        frame_time: TimingStats::from_samples(frame_samples),
        failures,
    };
    Ok((report, backend))
}
