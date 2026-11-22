//! D3D12 call-trace hooks (`AW2_D3D12_TRACE=<path>`; format in
//! `d3d12_metal::trace`). Every hook checks [`on`] first, so tracing costs one
//! relaxed load when disabled.

pub use d3d12_metal::trace::{Call, U32s, U64s};

#[inline]
pub fn on() -> bool {
    d3d12_metal::trace::enabled()
}

#[inline]
pub fn record(c: &Call) {
    d3d12_metal::trace::record(c)
}

/// Raw Windows-layout bytes of `*p` (empty when null).
pub unsafe fn raw<'a, T>(p: *const T) -> &'a [u8] {
    raw_n(p, 1)
}

/// Raw bytes of `n` consecutive `T`s at `p` (empty when null or `n == 0`).
pub unsafe fn raw_n<'a, T>(p: *const T, n: usize) -> &'a [u8] {
    if p.is_null() || n == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(p as *const u8, std::mem::size_of::<T>() * n)
    }
}

/// `Some(bytes)` of an optional struct pointer.
pub unsafe fn opt<'a, T>(p: *const T) -> Option<&'a [u8]> {
    if p.is_null() {
        None
    } else {
        Some(raw(p))
    }
}
/// `Some(*out)` for a successful creation call (the interface pointer is
/// the trace id: every object has one superset vtable).
pub unsafe fn created(hr: crate::types::HRESULT, out: *mut *mut std::ffi::c_void) -> Option<u64> {
    (hr >= 0 && !out.is_null() && !(*out).is_null()).then(|| *out as u64)
}

static DEVICE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn set_device(id: u64) {
    DEVICE.store(id, std::sync::atomic::Ordering::Relaxed);
}

/// The (single) `ID3D12Device*`.
pub fn device() -> u64 {
    DEVICE.load(std::sync::atomic::Ordering::Relaxed)
}

/// GPU VA of a resource id (0 for textures / unknown).
pub unsafe fn resource_va(id: u64) -> u64 {
    super::resource::from_ptr(id as *mut std::ffi::c_void).map_or(0, |r| r.r.gpu_va)
}

// ---------------------------------------------------------------------------
// CPU writes into persistently mapped buffers
// ---------------------------------------------------------------------------
//
// While tracing, every mapped CPU-writable buffer is tracked with one hash
// per 4 KiB page. `sync_uploads` (at ExecuteCommandLists and Present) emits
// `WriteResource` for pages whose hash changed (adjacent pages merged), so
// the replayer sees the data each submission used. The first pass after
// Map counts as all-dirty.

const PAGE: usize = 4096;

struct Mapped {
    ptr: usize,
    size: usize,
    /// Empty until the first sync.
    hashes: Vec<u64>,
}

fn uploads() -> &'static std::sync::Mutex<std::collections::HashMap<u64, Mapped>> {
    static U: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<u64, Mapped>>> = std::sync::OnceLock::new();
    U.get_or_init(Default::default)
}

/// `Map` of a CPU-writable buffer while tracing.
pub fn track_mapped(id: u64, ptr: *mut u8, size: usize) {
    if !on() || ptr.is_null() || size == 0 {
        return;
    }
    let mut u = uploads().lock().unwrap();
    u.entry(id).or_insert(Mapped { ptr: ptr as usize, size, hashes: Vec::new() });
}

/// Final `Unmap` / `Release`.
pub fn untrack(id: u64) {
    if on() {
        uploads().lock().unwrap().remove(&id);
    }
}

fn page_hash(p: &[u8]) -> u64 {
    // 64-bit multiply-xorshift over 8-byte words (unaligned-safe).
    let mut h = 0x9E37_79B9_7F4A_7C15u64 ^ p.len() as u64;
    let mut it = p.chunks_exact(8);
    for c in &mut it {
        let w = u64::from_le_bytes(c.try_into().unwrap());
        h = (h ^ w).wrapping_mul(0xFF51_AFD7_ED55_8CCD);
        h ^= h >> 32;
    }
    for &b in it.remainder() {
        h = (h ^ b as u64).wrapping_mul(0xC4CE_B9FE_1A85_EC53);
    }
    h
}

/// Emit `WriteResource` records for changed pages of tracked buffers.
pub unsafe fn sync_uploads() {
    if !on() {
        return;
    }
    let mut u = uploads().lock().unwrap();
    for (&id, m) in u.iter_mut() {
        let data = std::slice::from_raw_parts(m.ptr as *const u8, m.size);
        let pages = m.size.div_ceil(PAGE);
        let first = m.hashes.is_empty();
        if first {
            m.hashes = vec![0; pages];
        }
        let mut run: Option<usize> = None; // first dirty page of the current run
        for i in 0..=pages {
            let dirty = i < pages && {
                let h = page_hash(&data[i * PAGE..((i + 1) * PAGE).min(m.size)]);
                let d = first || h != m.hashes[i];
                m.hashes[i] = h;
                d
            };
            match (dirty, run) {
                (true, None) => run = Some(i),
                (false, Some(s)) => {
                    let (b, e) = (s * PAGE, (i * PAGE).min(m.size));
                    record(&Call::WriteResource { resource: id, subresource: 0, offset: b as u64, data: &data[b..e] });
                    run = None;
                }
                _ => {}
            }
        }
    }
}

/// Records `$call` when tracing is on; the expression is only evaluated then.
/// Leading `let` bindings may hold temporaries the call borrows.
macro_rules! tr {
    ($(let $p:tt $(: $t:ty)? = $e:expr;)* $call:expr) => {
        if $crate::d3d12::tr::on() {
            #[allow(unused_imports)]
            use $crate::d3d12::tr::{Call, U32s, U64s};
            $(let $p $(: $t)? = $e;)*
            $crate::d3d12::tr::record(&$call);
        }
    };
}
pub(crate) use tr;
