//! The Metal device behind our single DXGI adapter / D3D12 device (lives in
//! `d3d12-metal`; re-exported for the dxgi shims).

pub use d3d12_metal::metal::device::{
    available_headroom_bytes, device, ios_jetsam_headroom_bytes, log_memory_snapshot,
    physical_memory, process_phys_footprint, Device as MetalDevice,
};

/// Bytes currently allocated on the device (for QueryVideoMemoryInfo).
pub fn current_allocated() -> u64 {
    device().map(|d| d.current_allocated()).unwrap_or(0)
}

/// Live `(budget, current_usage)` for `IDXGIAdapter3::QueryVideoMemoryInfo`.
pub fn dynamic_vram_budget() -> (u64, u64) {
    device().map(|d| d.dynamic_vram_budget()).unwrap_or((0, 0))
}

/// Log Darwin footprint + Metal GPU breakdown plus guest `VirtualAlloc` committed bytes.
pub fn log_memory_snapshot_full(tag: &str) {
    log_memory_snapshot(tag);
    if cfg!(target_os = "ios") {
        let va_mb = crate::kernel32::memory::committed_private_bytes() as f64 / (1024.0 * 1024.0);
        log::info!(target: "d3d", "mem[{tag}]: guest_va_committed={va_mb:.1} MiB");
    }
}

/// Handle a host low-memory warning (e.g. `UIApplicationDidReceiveMemoryWarningNotification`):
/// log footprint/headroom, evict transient shader/library caches, flush pending PSO archives,
/// purge Darwin libmalloc magazine caches back to XNU, and signal any registered DXGI video
/// memory budget change events.
pub fn handle_memory_warning() {
    log_memory_snapshot_full("memory-warning");
    d3d12_metal::metal::pipeline::trim_caches();
    #[cfg(target_vendor = "apple")]
    {
        extern "C" {
            fn malloc_zone_pressure_relief(zone: *mut std::ffi::c_void, goal: usize) -> usize;
        }
        unsafe {
            malloc_zone_pressure_relief(std::ptr::null_mut(), 0);
        }
    }
    crate::dxgi::adapter::notify_video_memory_budget_change();
}

/// Arm the one-frame GPU `NaN`/`Inf`/HDR-spike pass scanner (`d3d12_metal::metal::gpunan`).
pub fn arm_gpu_nan_scan() {
    d3d12_metal::metal::gpunan::arm_next_frame();
}
