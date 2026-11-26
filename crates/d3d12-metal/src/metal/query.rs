//! Query heaps.
//!
//! * Timestamps (`TIMESTAMP`, `COPY_QUEUE_TIMESTAMP`): an `MTL4CounterHeap`
//!   of type timestamp; `EndQuery` writes a GPU timestamp, `ResolveQueryData`
//!   resolves into the destination buffer as u64 ticks of
//!   [`timestamp_frequency`].
//! * Occlusion: a shared visibility-result buffer (one u64 per query).
//!   `BeginQuery` zeroes the slot and arms it; render passes opened while a
//!   query is active get the buffer (accumulate mode) and count into the
//!   slot; `ResolveQueryData` copies the slots. Binary occlusion uses
//!   Metal's boolean mode, which writes "nonzero" rather than exactly 1.
//! * Pipeline statistics: not implemented (the callers trap).

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSRange;
use objc2_metal::*;

pub type MtlCounterHeap = ProtocolObject<dyn MTL4CounterHeap>;

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

pub struct QueryHeap {
    pub ty: u32,
    pub count: u32,
    pub timestamps: Option<Retained<MtlCounterHeap>>,
    pub visibility: Option<Retained<super::resource::MtlBuffer>>,
}
unsafe impl Send for QueryHeap {}
unsafe impl Sync for QueryHeap {}

/// GPU timestamp ticks per second (`GetTimestampFrequency`).
pub fn timestamp_frequency() -> u64 {
    super::device().map(|d| d.dev.queryTimestampFrequency()).filter(|&f| f != 0).unwrap_or(1_000_000_000)
}

impl QueryHeap {
    pub fn new(ty: u32, count: u32) -> Result<QueryHeap, String> {
        let timestamps = match ty {
            heap_type::TIMESTAMP | heap_type::COPY_QUEUE_TIMESTAMP => {
                let d = super::device().ok_or("no device")?;
                let desc = MTL4CounterHeapDescriptor::new();
                desc.setType(MTL4CounterHeapType::Timestamp);
                unsafe { desc.setCount(count.max(1) as usize) };
                let h = d.dev.newCounterHeapWithDescriptor_error(&desc).map_err(|e| format!("newCounterHeap(timestamp, {count}): {e}"))?;
                let entry = d.dev.sizeOfCounterHeapEntry(MTL4CounterHeapType::Timestamp);
                if entry != 8 {
                    return Err(format!("timestamp heap entries are {entry} bytes, expected 8"));
                }
                Some(h)
            }
            _ => None,
        };
        let visibility = match ty {
            heap_type::OCCLUSION => {
                let d = super::device().ok_or("no device")?;
                let bytes = count.max(1) as usize * 8;
                let b = d
                    .dev
                    .newBufferWithLength_options(bytes, MTLResourceOptions::StorageModeShared | MTLResourceOptions::HazardTrackingModeUntracked)
                    .ok_or_else(|| format!("newBuffer({bytes}) for an occlusion query heap"))?;
                unsafe { std::ptr::write_bytes(b.contents().as_ptr() as *mut u8, 0, bytes) };
                b.setLabel(Some(&objc2_foundation::NSString::from_str("port occlusion queries")));
                d.add_resident(ProtocolObject::from_ref(&*b));
                Some(b)
            }
            _ => None,
        };
        Ok(QueryHeap { ty, count, timestamps, visibility })
    }
}

impl Drop for QueryHeap {
    fn drop(&mut self) {
        if let (Some(b), Some(d)) = (&self.visibility, super::device()) {
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
        let b = heap.visibility.as_ref().ok_or("not an occlusion heap")?;
        if index >= heap.count {
            return Err(format!("query index {index} out of range ({})", heap.count));
        }
        self.set_occlusion(Some(ActiveOcclusion { buffer: b.clone(), index, binary }))
    }

    /// `EndQuery(OCCLUSION | BINARY_OCCLUSION)`.
    pub fn end_occlusion(&mut self, heap: &QueryHeap, index: u32) -> Result<(), String> {
        if heap.visibility.is_none() {
            return Err("not an occlusion heap".into());
        }
        self.set_occlusion(None)?;
        let _ = index;
        Ok(())
    }

    /// `ResolveQueryData(OCCLUSION | BINARY_OCCLUSION)`.
    pub fn resolve_occlusion(&mut self, heap: &QueryHeap, start: u32, n: u32, dst: &super::resource::Resource, offset: u64) -> Result<(), String> {
        let v = heap.visibility.as_ref().ok_or("not an occlusion heap")?.clone();
        if start.checked_add(n).is_none_or(|e| e > heap.count) {
            return Err(format!("queries {start}+{n} out of range ({})", heap.count));
        }
        let d = dst.buffer().ok_or("resolve destination is not a buffer")?;
        if offset + n as u64 * 8 > d.length() as u64 {
            return Err("resolve destination too small".into());
        }
        self.copy_after_render(&v, start as u64 * 8, dst, offset, n as u64 * 8)
    }

    /// `EndQuery(TIMESTAMP)`.
    pub fn write_timestamp(&mut self, heap: &QueryHeap, index: u32) -> Result<(), String> {
        let h = heap.timestamps.as_deref().ok_or("not a timestamp heap")?;
        if index >= heap.count {
            return Err(format!("query index {index} out of range ({})", heap.count));
        }
        self.timestamp_into(h, index as usize);
        Ok(())
    }

    /// `ResolveQueryData(TIMESTAMP)`: u64 ticks into `dst` at `offset`.
    pub fn resolve_timestamps(&mut self, heap: &QueryHeap, start: u32, n: u32, dst: &super::resource::Resource, offset: u64) -> Result<(), String> {
        let h = heap.timestamps.as_deref().ok_or("not a timestamp heap")?;
        if start.checked_add(n).is_none_or(|e| e > heap.count) {
            return Err(format!("queries {start}+{n} out of range ({})", heap.count));
        }
        let b = dst.buffer().ok_or("resolve destination is not a buffer")?;
        if offset + n as u64 * 8 > b.length() as u64 {
            return Err("resolve destination too small".into());
        }
        self.resolve_counters(h, NSRange::new(start as usize, n as usize), MTL4BufferRange { bufferAddress: b.gpuAddress() + offset, length: n as u64 * 8 });
        Ok(())
    }
}
