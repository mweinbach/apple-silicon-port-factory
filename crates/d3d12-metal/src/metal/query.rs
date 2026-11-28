//! Query heaps.
//!
//! * Timestamps (`TIMESTAMP`, `COPY_QUEUE_TIMESTAMP`):
//!   - Heaps with `count <= MAX_DIRECT_COUNTER_HEAP_ENTRIES` (`4096`, Apple AGX's
//!     per-`MTL4CounterHeap` hardware limit) use a direct `MTL4CounterHeap`:
//!     `EndQuery` writes a GPU timestamp, `ResolveQueryData` resolves into the
//!     destination buffer as u64 ticks of [`timestamp_frequency`].
//!   - Heaps with `count > 4096` (e.g. 1M entries = 8 MiB) or created when the
//!     ~32-heap per-process hardware ceiling is reached use a GPU-resident
//!     backing `MTLBuffer` (`count * 8` bytes) fed by a lazily initialized shared
//!     scratch `MTL4CounterHeap` ring via real Metal 4 `writeTimestamp` +
//!     `resolveCounterHeap`, and `ResolveQueryData` copies the resolved u64
//!     ticks from the backing buffer into the destination buffer.
//! * Occlusion (`OCCLUSION`, supporting `D3D12_QUERY_TYPE_OCCLUSION` and
//!   `D3D12_QUERY_TYPE_BINARY_OCCLUSION`):
//!   - Backed by shared visibility-result `MTLBuffer` chunks (8 bytes per query,
//!     at most `MAX_VIS_CHUNK_ENTRIES` = 32,768 entries / 256 KiB per chunk so
//!     `MTL4RenderCommandEncoder::setVisibilityResultMode:offset:` stays within
//!     Apple AGX's 15-bit hardware slot index limit without wrap-around).
//!   - `BeginQuery` arms the slot with `MTLVisibilityResultMode::Counting` or
//!     `Boolean`; `EndQuery` disarms it (zero-filling 0-draw queries);
//!     `ResolveQueryData` copies u64 sample counts for `OCCLUSION` or normalizes
//!     each u64 slot to strictly `0` or `1` for `BINARY_OCCLUSION`.
//! * Pipeline statistics: not implemented (the callers trap).

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSRange;
use objc2_metal::*;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

pub type MtlCounterHeap = ProtocolObject<dyn MTL4CounterHeap>;

/// Maximum entries per visibility-result `MTLBuffer` chunk (`32,768 * 8 = 256 KiB`).
///
/// Apple AGX hardware truncates `setVisibilityResultMode:offset:` slot indices to
/// 15 bits (`offset < 262_144`), silently wrapping indices `>= 32_768` back into
/// `index % 32_768` with no Metal validation error. Chunking at 32,768 keeps
/// every query index from `0` to millions strictly in bounds.
pub const MAX_VIS_CHUNK_ENTRIES: u32 = 32_768;

/// Maximum entries Apple AGX supports in a single `MTL4CounterHeap`.
pub const MAX_DIRECT_COUNTER_HEAP_ENTRIES: u32 = 4096;

/// Number of entries per scratch `MTL4CounterHeap` in [`TimestampScratchPool`].
pub(crate) const SCRATCH_HEAP_ENTRIES: usize = MAX_DIRECT_COUNTER_HEAP_ENTRIES as usize;

/// Target number of shared scratch `MTL4CounterHeap`s (8,192 concurrent scratch
/// slots = 64 KiB, consuming only 2 of the ~32 process-wide AGX counter heaps).
const SCRATCH_HEAP_COUNT: usize = 2;

/// `D3D12_QUERY_HEAP_TYPE`.
pub mod heap_type {
    pub const OCCLUSION: u32 = 0;
    pub const TIMESTAMP: u32 = 1;
    pub const PIPELINE_STATISTICS: u32 = 2;
    pub const COPY_QUEUE_TIMESTAMP: u32 = 5;
    pub const PIPELINE_STATISTICS1: u32 = 7;
}

/// `D3D12_QUERY_TYPE`.
pub mod query_type {
    pub const OCCLUSION: u32 = 0;
    pub const BINARY_OCCLUSION: u32 = 1;
    pub const TIMESTAMP: u32 = 2;
}

/// Error returned by [`QueryHeap::try_new`], separating argument validation
/// (`E_INVALIDARG`) from real Metal allocation failures (`E_OUTOFMEMORY`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryHeapError {
    InvalidArg(String),
    OutOfMemory(String),
}

impl fmt::Display for QueryHeapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArg(s) | Self::OutOfMemory(s) => f.write_str(s),
        }
    }
}

impl std::error::Error for QueryHeapError {}

impl From<QueryHeapError> for String {
    fn from(e: QueryHeapError) -> Self {
        e.to_string()
    }
}

/// Lazily initialized shared ring of hardware `MTL4CounterHeap`s feeding large
/// (`count > 4096`) or overflow timestamp query heaps.
pub(crate) struct TimestampScratchPool {
    heaps: Vec<Retained<MtlCounterHeap>>,
    next_slot: AtomicU64,
}
unsafe impl Send for TimestampScratchPool {}
unsafe impl Sync for TimestampScratchPool {}

impl TimestampScratchPool {
    fn init() -> Result<Self, String> {
        let d = super::device().ok_or_else(|| "no Metal 4 device".to_string())?;
        let entry = d.dev.sizeOfCounterHeapEntry(MTL4CounterHeapType::Timestamp);
        if entry != 8 {
            return Err(format!("timestamp heap entries are {entry} bytes, expected 8"));
        }
        let mut heaps = Vec::with_capacity(SCRATCH_HEAP_COUNT);
        let mut last_err = None;
        for i in 0..SCRATCH_HEAP_COUNT {
            let desc = MTL4CounterHeapDescriptor::new();
            desc.setType(MTL4CounterHeapType::Timestamp);
            unsafe { desc.setCount(SCRATCH_HEAP_ENTRIES) };
            match d.dev.newCounterHeapWithDescriptor_error(&desc) {
                Ok(h) => {
                    h.setLabel(Some(&objc2_foundation::NSString::from_str(&format!(
                        "port timestamp scratch {i}"
                    ))));
                    heaps.push(h);
                }
                Err(e) => {
                    last_err = Some(format!(
                        "newCounterHeap(timestamp scratch {i}, {SCRATCH_HEAP_ENTRIES}): {e}"
                    ));
                    break;
                }
            }
        }
        if heaps.is_empty() {
            return Err(last_err.unwrap_or_else(|| "failed to allocate timestamp scratch heap".into()));
        }
        Ok(Self { heaps, next_slot: AtomicU64::new(0) })
    }

    /// Claim the next scratch `(heap_idx, entry_idx)` from the lock-free ring.
    #[inline]
    pub(crate) fn alloc_slot(&self) -> (usize, usize) {
        let total = self.heaps.len() * SCRATCH_HEAP_ENTRIES;
        let raw = self.next_slot.fetch_add(1, Ordering::Relaxed) as usize;
        let slot = raw % total;
        (slot / SCRATCH_HEAP_ENTRIES, slot % SCRATCH_HEAP_ENTRIES)
    }

    #[inline]
    pub(crate) fn heap(&self, heap_idx: usize) -> &MtlCounterHeap {
        &self.heaps[heap_idx]
    }

    /// Flush pending staged timestamps well before a single command list can wrap
    /// around its own scratch slots.
    #[inline]
    pub(crate) fn flush_threshold(&self) -> usize {
        SCRATCH_HEAP_ENTRIES / 2
    }
}

pub(crate) fn timestamp_scratch_pool() -> Result<&'static TimestampScratchPool, String> {
    static POOL: OnceLock<Result<TimestampScratchPool, String>> = OnceLock::new();
    match POOL.get_or_init(TimestampScratchPool::init) {
        Ok(p) => Ok(p),
        Err(e) => Err(e.clone()),
    }
}

/// One staged GPU timestamp waiting to be resolved from the shared scratch
/// `MTL4CounterHeap` ring into its `QueryHeap`'s backing `MTLBuffer`.
pub(crate) struct PendingStagedTimestamp {
    pub heap_idx: u8,
    pub scratch_slot: u16,
    /// Retains the destination backing buffer until the resolve is encoded.
    pub _dst_buf: Retained<super::resource::MtlBuffer>,
    pub dst_gpu_va: u64,
}

/// Compute `(chunk_index, local_index)` for an occlusion query slot.
#[inline]
pub fn visibility_chunk_slot(index: u32) -> (usize, u32) {
    ((index / MAX_VIS_CHUNK_ENTRIES) as usize, index % MAX_VIS_CHUNK_ENTRIES)
}

/// Compute the number of visibility-result chunks needed for `count` occlusion queries.
#[inline]
pub fn visibility_chunk_count_for(count: u32) -> usize {
    if count == 0 {
        0
    } else {
        count.div_ceil(MAX_VIS_CHUNK_ENTRIES) as usize
    }
}

pub struct QueryHeap {
    pub ty: u32,
    pub count: u32,
    /// Direct hardware counter heap for small `TIMESTAMP` / `COPY_QUEUE_TIMESTAMP` heaps (`count <= 4096`).
    pub timestamps: Option<Retained<MtlCounterHeap>>,
    /// GPU-resident backing buffer (`count * 8` bytes) for large (`count > 4096`) or overflow
    /// `TIMESTAMP` / `COPY_QUEUE_TIMESTAMP` heaps.
    pub timestamp_buffer: Option<Retained<super::resource::MtlBuffer>>,
    /// Chunked 256 KiB visibility-result buffers for `OCCLUSION` heaps (`<= 32,768` entries each).
    pub visibility_chunks: Vec<Retained<super::resource::MtlBuffer>>,
}
unsafe impl Send for QueryHeap {}
unsafe impl Sync for QueryHeap {}

/// GPU timestamp ticks per second (`GetTimestampFrequency`).
pub fn timestamp_frequency() -> u64 {
    super::device()
        .map(|d| d.dev.queryTimestampFrequency())
        .filter(|&f| f != 0)
        .unwrap_or(1_000_000_000)
}

impl QueryHeap {
    /// Returns `(chunk_buffer, local_index)` for logical occlusion query `index`.
    #[inline]
    pub fn visibility_slot(&self, index: u32) -> Option<(&Retained<super::resource::MtlBuffer>, u32)> {
        if index >= self.count {
            return None;
        }
        let (chunk, local) = visibility_chunk_slot(index);
        self.visibility_chunks.get(chunk).map(|b| (b, local))
    }

    /// True if this timestamp heap uses a GPU-resident backing buffer fed by the
    /// shared scratch `MTL4CounterHeap` ring.
    #[inline]
    pub fn is_staged_timestamp(&self) -> bool {
        self.timestamp_buffer.is_some()
    }

    /// Create a query heap with distinct validation (`InvalidArg`) vs allocation
    /// (`OutOfMemory`) errors.
    pub fn try_new(ty: u32, count: u32) -> Result<QueryHeap, QueryHeapError> {
        if !matches!(
            ty,
            heap_type::OCCLUSION
                | heap_type::TIMESTAMP
                | heap_type::PIPELINE_STATISTICS
                | heap_type::COPY_QUEUE_TIMESTAMP
                | heap_type::PIPELINE_STATISTICS1
        ) {
            return Err(QueryHeapError::InvalidArg(format!("unsupported query heap type {ty}")));
        }
        if count == 0 {
            return Err(QueryHeapError::InvalidArg("query heap count must be > 0".into()));
        }

        let mut timestamps = None;
        let mut timestamp_buffer = None;
        let mut visibility_chunks = Vec::new();

        match ty {
            heap_type::TIMESTAMP | heap_type::COPY_QUEUE_TIMESTAMP => {
                let d = super::device()
                    .ok_or_else(|| QueryHeapError::OutOfMemory("no Metal 4 device".into()))?;
                let mut direct_ok = false;
                if count <= MAX_DIRECT_COUNTER_HEAP_ENTRIES {
                    let desc = MTL4CounterHeapDescriptor::new();
                    desc.setType(MTL4CounterHeapType::Timestamp);
                    unsafe { desc.setCount(count as usize) };
                    if let Ok(h) = d.dev.newCounterHeapWithDescriptor_error(&desc) {
                        let entry = d.dev.sizeOfCounterHeapEntry(MTL4CounterHeapType::Timestamp);
                        if entry != 8 {
                            return Err(QueryHeapError::OutOfMemory(format!(
                                "timestamp heap entries are {entry} bytes, expected 8"
                            )));
                        }
                        timestamps = Some(h);
                        direct_ok = true;
                    }
                }
                if !direct_ok {
                    timestamp_scratch_pool().map_err(QueryHeapError::OutOfMemory)?;
                    let total_bytes = (count as u64)
                        .checked_mul(8)
                        .ok_or_else(|| QueryHeapError::OutOfMemory(format!("timestamp heap byte size overflow ({count})")))?;
                    let max_buf = d.dev.maxBufferLength() as u64;
                    if total_bytes > max_buf {
                        return Err(QueryHeapError::OutOfMemory(format!(
                            "timestamp query heap ({total_bytes} bytes) exceeds device maxBufferLength ({max_buf})"
                        )));
                    }
                    let bytes = total_bytes as usize;
                    let b = d
                        .dev
                        .newBufferWithLength_options(
                            bytes,
                            MTLResourceOptions::StorageModeShared
                                | MTLResourceOptions::HazardTrackingModeUntracked,
                        )
                        .ok_or_else(|| {
                            QueryHeapError::OutOfMemory(format!(
                                "newBuffer({bytes}) for a timestamp query heap ({count} entries)"
                            ))
                        })?;
                    unsafe { std::ptr::write_bytes(b.contents().as_ptr() as *mut u8, 0, bytes) };
                    b.setLabel(Some(&objc2_foundation::NSString::from_str("port timestamp queries")));
                    d.add_resident(ProtocolObject::from_ref(&*b));
                    log::debug!(
                        target: "d3d",
                        "CreateQueryHeap(timestamp ty={ty}, count={count}): staged backing buffer ({bytes} bytes)"
                    );
                    timestamp_buffer = Some(b);
                }
            }
            heap_type::OCCLUSION => {
                let d = super::device()
                    .ok_or_else(|| QueryHeapError::OutOfMemory("no Metal 4 device".into()))?;
                let total_bytes = (count as u64)
                    .checked_mul(8)
                    .ok_or_else(|| QueryHeapError::OutOfMemory(format!("occlusion heap byte size overflow ({count})")))?;
                let max_buf = d.dev.maxBufferLength() as u64;
                if total_bytes > max_buf {
                    return Err(QueryHeapError::OutOfMemory(format!(
                        "occlusion query heap ({total_bytes} bytes) exceeds device maxBufferLength ({max_buf})"
                    )));
                }
                let n_chunks = visibility_chunk_count_for(count);
                visibility_chunks.reserve(n_chunks);
                let mut remaining = count;
                while remaining > 0 {
                    let chunk_entries = remaining.min(MAX_VIS_CHUNK_ENTRIES);
                    let bytes = chunk_entries as usize * 8;
                    let b = d
                        .dev
                        .newBufferWithLength_options(
                            bytes,
                            MTLResourceOptions::StorageModeShared
                                | MTLResourceOptions::HazardTrackingModeUntracked,
                        )
                        .ok_or_else(|| {
                            QueryHeapError::OutOfMemory(format!(
                                "newBuffer({bytes}) for an occlusion query heap ({count} entries)"
                            ))
                        })?;
                    unsafe { std::ptr::write_bytes(b.contents().as_ptr() as *mut u8, 0, bytes) };
                    b.setLabel(Some(&objc2_foundation::NSString::from_str("port occlusion queries")));
                    visibility_chunks.push(b);
                    remaining -= chunk_entries;
                }
                for b in &visibility_chunks {
                    d.add_resident(ProtocolObject::from_ref(&**b));
                }
            }
            _ => {}
        }

        Ok(QueryHeap { ty, count, timestamps, timestamp_buffer, visibility_chunks })
    }

    pub fn new(ty: u32, count: u32) -> Result<QueryHeap, String> {
        Self::try_new(ty, count).map_err(Into::into)
    }
}

impl Drop for QueryHeap {
    fn drop(&mut self) {
        let Some(d) = super::device() else { return };
        if let Some(b) = &self.timestamp_buffer {
            d.remove_resident(ProtocolObject::from_ref(&**b));
        }
        for b in &self.visibility_chunks {
            d.remove_resident(ProtocolObject::from_ref(&**b));
        }
    }
}

/// The occlusion query a list is currently counting into.
#[derive(Clone)]
pub struct ActiveOcclusion {
    pub buffer: Retained<super::resource::MtlBuffer>,
    pub index: u32,
    pub binary: bool,
}

impl super::cmdlist::CommandList {
    /// `BeginQuery(OCCLUSION | BINARY_OCCLUSION)`.
    pub fn begin_occlusion(&mut self, heap: &QueryHeap, index: u32, binary: bool) -> Result<(), String> {
        if heap.visibility_chunks.is_empty() {
            return Err("not an occlusion heap".into());
        }
        let (b, local_index) = heap
            .visibility_slot(index)
            .ok_or_else(|| format!("query index {index} out of range ({})", heap.count))?;
        self.set_occlusion(Some(ActiveOcclusion { buffer: b.clone(), index: local_index, binary }))
    }

    /// `EndQuery(OCCLUSION | BINARY_OCCLUSION)`.
    pub fn end_occlusion(&mut self, heap: &QueryHeap, index: u32) -> Result<(), String> {
        if heap.visibility_chunks.is_empty() {
            return Err("not an occlusion heap".into());
        }
        if index >= heap.count {
            return Err(format!("query index {index} out of range ({})", heap.count));
        }
        self.set_occlusion(None)
    }

    fn resolve_occlusion_inner(
        &mut self,
        heap: &QueryHeap,
        start: u32,
        n: u32,
        dst: &super::resource::Resource,
        offset: u64,
        binary: bool,
    ) -> Result<(), String> {
        if heap.visibility_chunks.is_empty() {
            return Err("not an occlusion heap".into());
        }
        if start.checked_add(n).is_none_or(|e| e > heap.count) {
            return Err(format!("queries {start}+{n} out of range ({})", heap.count));
        }
        let d = dst.buffer().ok_or("resolve destination is not a buffer")?;
        let bytes = (n as u64).checked_mul(8).ok_or("resolve size overflow")?;
        if offset.checked_add(bytes).is_none_or(|e| e > d.length() as u64) {
            return Err("resolve destination too small".into());
        }
        if n == 0 {
            return Ok(());
        }
        let mut cur = start;
        let end = start + n;
        let mut dst_cur = offset;
        while cur < end {
            let (chunk_idx, local_start) = visibility_chunk_slot(cur);
            let take = (end - cur).min(MAX_VIS_CHUNK_ENTRIES - local_start);
            let v = heap.visibility_chunks[chunk_idx].clone();
            if binary {
                self.resolve_binary_after_render(&v, local_start as u64 * 8, dst, dst_cur, take)?;
            } else {
                self.copy_after_render(&v, local_start as u64 * 8, dst, dst_cur, take as u64 * 8)?;
            }
            cur += take;
            dst_cur += take as u64 * 8;
        }
        Ok(())
    }

    /// `ResolveQueryData(OCCLUSION)`: copies u64 sample counts into `dst` at `offset`.
    pub fn resolve_occlusion(
        &mut self,
        heap: &QueryHeap,
        start: u32,
        n: u32,
        dst: &super::resource::Resource,
        offset: u64,
    ) -> Result<(), String> {
        self.resolve_occlusion_inner(heap, start, n, dst, offset, false)
    }

    /// `ResolveQueryData(BINARY_OCCLUSION)`: writes strictly `0u64` or `1u64` per
    /// query into `dst` at `offset` (matching D3D12 binary-occlusion semantics).
    pub fn resolve_binary_occlusion(
        &mut self,
        heap: &QueryHeap,
        start: u32,
        n: u32,
        dst: &super::resource::Resource,
        offset: u64,
    ) -> Result<(), String> {
        self.resolve_occlusion_inner(heap, start, n, dst, offset, true)
    }

    /// `EndQuery(TIMESTAMP)`.
    pub fn write_timestamp(&mut self, heap: &QueryHeap, index: u32) -> Result<(), String> {
        if index >= heap.count {
            return Err(format!("query index {index} out of range ({})", heap.count));
        }
        if let Some(h) = heap.timestamps.as_deref() {
            self.timestamp_into(h, index as usize);
            return Ok(());
        }
        let buf = heap.timestamp_buffer.as_ref().ok_or("not a timestamp heap")?;
        let pool = timestamp_scratch_pool()?;
        if self.pending_staged_ts.len() >= pool.flush_threshold() {
            self.flush_staged_timestamps();
        }
        let (heap_idx, entry_idx) = pool.alloc_slot();
        self.timestamp_into(pool.heap(heap_idx), entry_idx);
        self.pending_staged_ts.push(PendingStagedTimestamp {
            heap_idx: heap_idx as u8,
            scratch_slot: entry_idx as u16,
            _dst_buf: buf.clone(),
            dst_gpu_va: buf.gpuAddress() + index as u64 * 8,
        });
        Ok(())
    }

    /// `ResolveQueryData(TIMESTAMP)`: u64 ticks into `dst` at `offset`.
    pub fn resolve_timestamps(
        &mut self,
        heap: &QueryHeap,
        start: u32,
        n: u32,
        dst: &super::resource::Resource,
        offset: u64,
    ) -> Result<(), String> {
        if heap.timestamps.is_none() && heap.timestamp_buffer.is_none() {
            return Err("not a timestamp heap".into());
        }
        if start.checked_add(n).is_none_or(|e| e > heap.count) {
            return Err(format!("queries {start}+{n} out of range ({})", heap.count));
        }
        let b = dst.buffer().ok_or("resolve destination is not a buffer")?;
        let bytes = (n as u64).checked_mul(8).ok_or("resolve size overflow")?;
        if offset.checked_add(bytes).is_none_or(|e| e > b.length() as u64) {
            return Err("resolve destination too small".into());
        }
        if n == 0 {
            return Ok(());
        }
        if let Some(h) = heap.timestamps.as_deref() {
            self.resolve_counters(
                h,
                NSRange::new(start as usize, n as usize),
                MTL4BufferRange { bufferAddress: b.gpuAddress() + offset, length: bytes },
            );
            return Ok(());
        }
        let v = heap.timestamp_buffer.as_ref().unwrap().clone();
        self.flush_staged_timestamps();
        self.copy_timestamp_buffer(&v, start as u64 * 8, dst, offset, bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visibility_chunk_mapping_stays_within_15_bit_agx_limit() {
        assert_eq!(visibility_chunk_count_for(0), 0);
        assert_eq!(visibility_chunk_count_for(1), 1);
        assert_eq!(visibility_chunk_count_for(MAX_VIS_CHUNK_ENTRIES), 1);
        assert_eq!(visibility_chunk_count_for(MAX_VIS_CHUNK_ENTRIES + 1), 2);
        assert_eq!(visibility_chunk_count_for(1_048_576), 32);

        assert_eq!(visibility_chunk_slot(0), (0, 0));
        assert_eq!(visibility_chunk_slot(MAX_VIS_CHUNK_ENTRIES - 1), (0, MAX_VIS_CHUNK_ENTRIES - 1));
        assert_eq!(visibility_chunk_slot(MAX_VIS_CHUNK_ENTRIES), (1, 0));
        assert_eq!(visibility_chunk_slot(1_048_575), (31, MAX_VIS_CHUNK_ENTRIES - 1));
    }

    #[test]
    fn query_heap_validation_rejects_zero_count_and_unknown_type() {
        assert!(matches!(
            QueryHeap::try_new(heap_type::OCCLUSION, 0),
            Err(QueryHeapError::InvalidArg(_))
        ));
        assert!(matches!(
            QueryHeap::try_new(heap_type::TIMESTAMP, 0),
            Err(QueryHeapError::InvalidArg(_))
        ));
        assert!(matches!(
            QueryHeap::try_new(99, 16),
            Err(QueryHeapError::InvalidArg(_))
        ));
    }

    #[test]
    fn large_1m_occlusion_and_timestamp_heaps_allocate_and_pipeline_stats_preserved() {
        if super::super::device().is_none() {
            return;
        }
        let occ = QueryHeap::try_new(heap_type::OCCLUSION, 1_048_576)
            .expect("1M occlusion query heap (8 MiB) must succeed");
        assert_eq!(occ.visibility_chunks.len(), 32);
        assert!(occ.timestamps.is_none());
        assert!(occ.timestamp_buffer.is_none());
        let (c0, s0) = occ.visibility_slot(0).unwrap();
        let (c31, s31) = occ.visibility_slot(1_048_575).unwrap();
        assert_eq!(s0, 0);
        assert_eq!(s31, MAX_VIS_CHUNK_ENTRIES - 1);
        assert_ne!(c0.gpuAddress(), c31.gpuAddress());
        assert!(occ.visibility_slot(1_048_576).is_none());

        let ts_small = QueryHeap::try_new(heap_type::TIMESTAMP, 128)
            .expect("small timestamp query heap must keep direct counter-heap path");
        assert!(ts_small.timestamps.is_some());
        assert!(!ts_small.is_staged_timestamp());

        let ts_large = QueryHeap::try_new(heap_type::TIMESTAMP, 1_048_576)
            .expect("1M timestamp query heap (8 MiB) must succeed via staged backing buffer");
        assert!(ts_large.timestamps.is_none());
        assert!(ts_large.is_staged_timestamp());
        assert_eq!(ts_large.timestamp_buffer.as_ref().unwrap().length(), 1_048_576 * 8);

        let ts_copy_large = QueryHeap::try_new(heap_type::COPY_QUEUE_TIMESTAMP, 65_536)
            .expect("large COPY_QUEUE_TIMESTAMP heap must succeed");
        assert!(ts_copy_large.is_staged_timestamp());

        let ps = QueryHeap::try_new(heap_type::PIPELINE_STATISTICS, 64)
            .expect("pipeline statistics heap creation keeps current path");
        assert!(ps.timestamps.is_none());
        assert!(ps.timestamp_buffer.is_none());
        assert!(ps.visibility_chunks.is_empty());
    }
}
