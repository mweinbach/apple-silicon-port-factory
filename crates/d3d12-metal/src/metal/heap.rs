//! ID3D12Heap on a placement `MTLHeap`.

use super::resource::{storage_options, CreateError};
use crate::types::HeapDesc;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

pub type MtlHeap = ProtocolObject<dyn MTLHeap>;

#[derive(Default)]
pub(crate) struct PlacedHeapTracker {
    pub next_uid: u64,
    /// `(uid, lo, hi, may_alias_flag)`
    pub live: Vec<(u64, u64, u64, std::sync::Arc<std::sync::atomic::AtomicBool>)>,
    /// `(lo, hi, retire_submit_epoch)`
    pub retired: Vec<(u64, u64, u64)>,
}

pub struct Heap {
    pub heap: Retained<MtlHeap>,
    pub desc: HeapDesc,
    pub(crate) placed_spans: std::sync::Mutex<PlacedHeapTracker>,
    alloc_bytes: u64,
}
unsafe impl Send for Heap {}
unsafe impl Sync for Heap {}

impl Heap {
    pub fn new(desc: &HeapDesc, label: &str) -> Result<Heap, CreateError> {
        let d = super::device().ok_or_else(|| CreateError::Unsupported("no device".into()))?;
        if desc.SizeInBytes == 0 {
            return Err(CreateError::InvalidArg("heap of size 0".into()));
        }
        let opts = if (desc.Flags & 0xc0) == 0xc0 && desc.Properties.cpu_access() == crate::types::CpuAccess::None {
            MTLResourceOptions::StorageModeShared | MTLResourceOptions::HazardTrackingModeUntracked
        } else {
            storage_options(&desc.Properties)
        };
        let heap = objc2::rc::autoreleasepool(|_| {
            let hd = MTLHeapDescriptor::new();
            hd.setType(MTLHeapType::Placement);
            hd.setSize(desc.SizeInBytes as usize);
            hd.setResourceOptions(opts);
            let heap = d.dev.newHeapWithDescriptor(&hd).ok_or_else(|| CreateError::OutOfMemory(format!("newHeap({:#x})", desc.SizeInBytes)))?;
            if !label.is_empty() {
                heap.setLabel(Some(&objc2_foundation::NSString::from_str(label)));
            }
            Ok::<_, CreateError>(heap)
        })?;
        let alloc_bytes = (heap.size() as u64).max(desc.SizeInBytes);
        super::device::note_heap_alloc(alloc_bytes);
        d.add_resident(ProtocolObject::from_ref(&*heap));
        Ok(Heap {
            heap,
            desc: *desc,
            placed_spans: std::sync::Mutex::new(PlacedHeapTracker::default()),
            alloc_bytes,
        })
    }

    pub fn set_label(&self, name: &str) {
        if name.is_empty() {
            return;
        }
        self.heap.setLabel(Some(&objc2_foundation::NSString::from_str(name)));
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        super::device::note_heap_free(self.alloc_bytes);
        if let Some(d) = super::device() {
            d.remove_resident(ProtocolObject::from_ref(&*self.heap));
        }
    }
}
