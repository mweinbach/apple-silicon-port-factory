//! ID3D12Resource backends: `MTLBuffer` (GPU VA = `gpuAddress`) or
//! `MTLTexture`. All allocations join the device residency set.

use crate::format;
use crate::types::*;
use objc2::rc::Retained;
use objc2::Message;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

static MIN_BUFFER_VA: AtomicU64 = AtomicU64::new(u64::MAX);
static MAX_BUFFER_VA: AtomicU64 = AtomicU64::new(0);

/// Record a D3D12 buffer's GPU virtual address range `[va, va + len)`.
pub fn record_buffer_va(va: u64, len: u64) {
    if va != 0 && len != 0 {
        MIN_BUFFER_VA.fetch_min(va & !3, Ordering::Relaxed);
        MAX_BUFFER_VA.fetch_max(va.saturating_add(len), Ordering::Relaxed);
    }
}

/// Span `[min_va, max_va)` covering all created D3D12 buffers on the device.
pub fn buffer_va_range() -> Option<(u64, u64)> {
    let lo = MIN_BUFFER_VA.load(Ordering::Relaxed);
    let hi = MAX_BUFFER_VA.load(Ordering::Relaxed);
    (lo < hi).then_some((lo, hi))
}

pub type MtlBuffer = ProtocolObject<dyn MTLBuffer>;
pub type MtlTexture = ProtocolObject<dyn MTLTexture>;

pub enum Backing {
    Buffer(Retained<MtlBuffer>),
    Texture(Retained<MtlTexture>),
}

/// Key of a cached texture view.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ViewKey {
    pub format: usize,
    pub ty: usize,
    pub mips: (u32, u32),
    pub slices: (u32, u32),
    /// `MTLTextureSwizzle` r,g,b,a packed in bytes (0x05040302 = identity).
    pub swizzle: u32,
    /// Canonical `f32::to_bits()` of Metal 4.1 `MTLTextureViewDescriptor.minLOD`
    /// (`0` when disabled or zero).
    pub min_lod_bits: u32,
}

pub const SWIZZLE_IDENTITY: u32 = 0x0504_0302;

fn unpack_swizzle(s: u32) -> MTLTextureSwizzleChannels {
    let c = |i: u32| MTLTextureSwizzle(((s >> (8 * i)) & 0xFF) as u8);
    MTLTextureSwizzleChannels { red: c(0), green: c(1), blue: c(2), alpha: c(3) }
}

/// Creates a Metal 4.1 (`macOS 27` / `iOS 27`) texture view with hardware `minLOD` via
/// `MTLTextureViewDescriptor` + `newTextureViewWithDescriptor:`. Returns `None` on older
/// OSes lacking the class/selectors so callers fall back cleanly to standard texture views.
unsafe fn create_texture_view_with_min_lod(
    tex: &MtlTexture,
    fmt: MTLPixelFormat,
    ty: MTLTextureType,
    levels: objc2_foundation::NSRange,
    slices: objc2_foundation::NSRange,
    swizzle: u32,
    abs_min_lod: f32,
) -> Option<Retained<MtlTexture>> {
    use objc2::runtime::{AnyClass, AnyObject, Bool};
    use objc2::{msg_send, sel};

    let cls = AnyClass::get(c"MTLTextureViewDescriptor")?;
    let has_new_view: Bool =
        msg_send![tex, respondsToSelector: sel!(newTextureViewWithDescriptor:)];
    if !has_new_view.as_bool() {
        return None;
    }
    let desc: Retained<AnyObject> = msg_send![cls, new];
    let has_min_lod: Bool = msg_send![&*desc, respondsToSelector: sel!(setMinLOD:)];
    if !has_min_lod.as_bool() {
        return None;
    }
    let _: () = msg_send![&*desc, setPixelFormat: fmt];
    let _: () = msg_send![&*desc, setTextureType: ty];
    let _: () = msg_send![&*desc, setLevelRange: levels];
    let _: () = msg_send![&*desc, setSliceRange: slices];
    if swizzle != SWIZZLE_IDENTITY {
        let _: () = msg_send![&*desc, setSwizzle: unpack_swizzle(swizzle)];
    }
    let _: () = msg_send![&*desc, setMinLOD: abs_min_lod];
    msg_send![tex, newTextureViewWithDescriptor: &*desc]
}

#[derive(Clone, Copy)]
struct BufRange {
    start: u64,
    end: u64,
    max_end: u64,
    key: usize,
}

/// Half-open GPU virtual address (or `[0, u64::MAX)` whole-resource) sub-range `[lo, hi)`
/// accessed within live `Resource` `r` (`*const Resource as usize`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubRange {
    pub r: usize,
    pub lo: u64,
    pub hi: u64,
}

impl SubRange {
    #[inline]
    pub const fn whole(r: usize) -> Self {
        Self { r, lo: 0, hi: u64::MAX }
    }

    #[inline]
    pub const fn overlaps(self, other: Self) -> bool {
        self.r == other.r && self.lo < other.hi && other.lo < self.hi
    }
}

/// Insert `s` into `spans`, coalescing any overlapping or adjacent ranges on `s.r`
/// so `spans` remains minimal and pairwise-disjoint per resource.
#[inline]
pub fn push_subrange(spans: &mut Vec<SubRange>, mut s: SubRange) {
    if s.lo >= s.hi {
        return;
    }
    let mut i = 0;
    while i < spans.len() {
        let e = spans[i];
        if e.r == s.r && e.lo <= s.hi && s.lo <= e.hi {
            if e.lo <= s.lo && s.hi <= e.hi {
                // Already fully covered by an existing disjoint span: keep hot
                // entries in place and exit immediately.
                return;
            }
            s.lo = s.lo.min(e.lo);
            s.hi = s.hi.max(e.hi);
            spans.swap_remove(i);
        } else {
            i += 1;
        }
    }
    spans.push(s);
}

/// Fast Fibonacci-multiply hasher for 64-bit integer `MTLResourceID` keys.
#[derive(Default)]
struct U64MixHasher(u64);

impl Hasher for U64MixHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = self.0.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(b as u64);
        }
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        let x = (self.0 ^ i).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        self.0 = x ^ (x >> 32);
    }
}

/// Per-command-list cache of strictly non-aliasing `[start, end) -> key` buffer ranges,
/// invalidated in O(1) whenever [`ResRegistry::buf_gen`] changes (resource destruction,
/// recreation at the same VA, or overlapping/aliasing placed-buffer insertion).
/// Only populated for buffers mathematically proven disjoint from all neighbors in
/// [`ResRegistry::buffers`].
///
/// Per-entry generation-tagged cache entry (32 bytes = power-of-2 stride so index
/// arithmetic compiles to a single shift and generation bumps require zero memset).
#[derive(Clone, Copy)]
struct BufVaEntry {
    start: u64,
    end: u64,
    key: usize,
    gen: u64,
}

impl BufVaEntry {
    const EMPTY: Self = Self { start: 0, end: 0, key: 0, gen: 0 };
}

pub(crate) struct BufVaCache {
    last: BufVaEntry,
    mru_cursor: usize,
    mru: [BufVaEntry; 4],
    page: [BufVaEntry; 512],
}

impl BufVaCache {
    pub(crate) const fn new() -> Self {
        Self {
            last: BufVaEntry::EMPTY,
            mru_cursor: 0,
            mru: [BufVaEntry::EMPTY; 4],
            page: [BufVaEntry::EMPTY; 512],
        }
    }

    pub(crate) fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }

    #[inline(always)]
    fn page_slot(va: u64) -> usize {
        (((va >> 12) ^ (va >> 21)) as usize) & 511
    }

    #[inline(always)]
    fn lookup(&mut self, buf_gen: u64, va: u64) -> Option<(u64, u64, usize)> {
        if buf_gen == 0 {
            return None;
        }
        // Fast 1-entry MRU front check for consecutive slots into the same buffer
        // (even across 64 KiB page boundaries) on the same cache line as `mru_cursor`.
        let l = self.last;
        if l.gen == buf_gen && va >= l.start && va < l.end {
            return Some((l.start, l.end, l.key));
        }
        // Check O(1) direct-mapped page table before the 4-entry collision MRU.
        let p = self.page[Self::page_slot(va)];
        if p.gen == buf_gen && va >= p.start && va < p.end {
            self.last = p;
            return Some((p.start, p.end, p.key));
        }
        for e in &self.mru {
            if e.gen == buf_gen && va >= e.start && va < e.end {
                let hit = *e;
                self.last = hit;
                return Some((hit.start, hit.end, hit.key));
            }
        }
        None
    }

    #[inline(always)]
    fn insert(&mut self, buf_gen: u64, va: u64, start: u64, end: u64, key: usize) {
        if buf_gen == 0 {
            return;
        }
        let entry = BufVaEntry { start, end, key, gen: buf_gen };
        self.last = entry;
        let slot = Self::page_slot(va);
        let prev = self.page[slot];
        self.page[slot] = entry;
        // Spill displaced live entry into the 4-entry MRU so hash collisions stay hot.
        if prev.gen == buf_gen
            && prev.key != key
            && !self.mru.iter().any(|e| e.gen == buf_gen && e.start == prev.start && e.end == prev.end)
        {
            self.mru[self.mru_cursor & 3] = prev;
            self.mru_cursor = self.mru_cursor.wrapping_add(1);
        }
    }
}

/// Live resource registry mapping buffer GPU virtual address ranges and
/// `MTLTexture` `gpuResourceID`s back to `&Resource as *const Resource as usize`.
#[derive(Default)]
pub struct ResRegistry {
    buffers: Vec<BufRange>,
    tex_ids: HashMap<u64, usize, BuildHasherDefault<U64MixHasher>>,
    /// Globally unique generation stamped on every `buffers` mutation (0 = empty/unmodified).
    buf_gen: u64,
}

static RES_REGISTRY_GEN: AtomicU64 = AtomicU64::new(1);

#[inline]
pub fn res_registry_gen() -> u64 {
    RES_REGISTRY_GEN.load(Ordering::Acquire)
}

#[inline]
fn bump_res_registry_gen() -> u64 {
    RES_REGISTRY_GEN.fetch_add(1, Ordering::Release).wrapping_add(1)
}

impl ResRegistry {
    fn recompute_max_end(&mut self, from: usize) {
        let mut cur = if from > 0 { self.buffers[from - 1].max_end } else { 0 };
        for b in &mut self.buffers[from..] {
            cur = cur.max(b.end);
            b.max_end = cur;
        }
    }

    pub(crate) fn insert_buffer(&mut self, start: u64, end: u64, key: usize) {
        if start == 0 || end <= start {
            return;
        }
        let idx = self.buffers.partition_point(|b| (b.start, b.end, b.key) < (start, end, key));
        if self.buffers.get(idx).is_some_and(|b| b.start == start && b.end == end && b.key == key) {
            return;
        }
        self.buffers.insert(idx, BufRange { start, end, max_end: end, key });
        self.recompute_max_end(idx);
        self.buf_gen = bump_res_registry_gen();
    }

    fn remove_buffer(&mut self, start: u64, key: usize) {
        if start == 0 {
            return;
        }
        let first = self.buffers.partition_point(|b| b.start < start);
        for i in first..self.buffers.len() {
            if self.buffers[i].start != start {
                break;
            }
            if self.buffers[i].key == key {
                self.buffers.remove(i);
                self.recompute_max_end(i);
                self.buf_gen = bump_res_registry_gen();
                break;
            }
        }
    }

    #[inline(always)]
    fn is_strictly_disjoint(&self, i: usize) -> bool {
        let b = &self.buffers[i];
        let prev_ok = i == 0 || self.buffers[i - 1].max_end <= b.start;
        let next_ok = i + 1 == self.buffers.len() || self.buffers[i + 1].start >= b.end;
        prev_ok && next_ok
    }

    #[inline]
    pub(crate) fn insert_tex_id(&mut self, id: u64, key: usize) {
        if self.tex_ids.insert(id, key) != Some(key) {
            bump_res_registry_gen();
        }
    }

    #[inline]
    fn remove_tex_id(&mut self, id: u64) {
        if self.tex_ids.remove(&id).is_some() {
            bump_res_registry_gen();
        }
    }

    /// Push all live buffer resource keys whose `[gpu_va, gpu_va + width)`
    /// contains `va`. Returns `true` if at least one live buffer matched.
    #[inline]
    pub fn push_buffer_va(&self, va: u64, out: &mut Vec<usize>) -> bool {
        let idx = self.buffers.partition_point(|b| b.start <= va);
        let mut match_count = 0usize;
        let mut i = idx;
        while i > 0 {
            i -= 1;
            let b = &self.buffers[i];
            if b.max_end <= va {
                break;
            }
            if va < b.end {
                match_count += 1;
                if out.last() != Some(&b.key) && !out.contains(&b.key) {
                    out.push(b.key);
                }
            }
        }
        match_count != 0
    }

    #[inline]
    pub(crate) fn push_buffer_va_cached(&self, cache: &mut BufVaCache, va: u64, out: &mut Vec<usize>) -> bool {
        let bg = self.buf_gen;
        if let Some((_, _, key)) = cache.lookup(bg, va) {
            if out.last() != Some(&key) && !out.contains(&key) {
                out.push(key);
            }
            return true;
        }
        let idx = self.buffers.partition_point(|b| b.start <= va);
        let mut match_count = 0usize;
        let mut last_idx = 0usize;
        let mut i = idx;
        while i > 0 {
            i -= 1;
            let b = &self.buffers[i];
            if b.max_end <= va {
                break;
            }
            if va < b.end {
                match_count += 1;
                last_idx = i;
                if out.last() != Some(&b.key) && !out.contains(&b.key) {
                    out.push(b.key);
                }
            }
        }
        if match_count == 1 && bg != 0 && self.is_strictly_disjoint(last_idx) {
            let b = &self.buffers[last_idx];
            cache.insert(bg, va, b.start, b.end, b.key);
        }
        match_count != 0
    }

    /// Resolve a buffer binding at `va` (`byte_len == 0` when unbounded / root VA,
    /// otherwise `IRDescriptorTableEntry` view byte size) into both deduplicated
    /// resource keys (`out`) and coalesced `[lo, hi)` GPU-VA sub-ranges (`spans`),
    /// accounting for any aliasing placed buffers overlapping the accessed span.
    #[inline]
    pub fn push_buffer_span(
        &self,
        va: u64,
        byte_len: u64,
        out: &mut Vec<usize>,
        spans: &mut Vec<SubRange>,
    ) -> bool {
        self.push_buffer_span_inner(None, va, byte_len, out, spans)
    }

    #[inline]
    pub(crate) fn push_buffer_span_cached(
        &self,
        cache: &mut BufVaCache,
        va: u64,
        byte_len: u64,
        out: &mut Vec<usize>,
        spans: &mut Vec<SubRange>,
    ) -> bool {
        let bg = self.buf_gen;
        if let Some((start, end, key)) = cache.lookup(bg, va) {
            let hi = if byte_len == 0 || (end - start) > u32::MAX as u64 {
                end
            } else {
                va.saturating_add(byte_len).min(end)
            };
            if out.last() != Some(&key) && !out.contains(&key) {
                out.push(key);
            }
            push_subrange(spans, SubRange { r: key, lo: va, hi });
            return true;
        }
        self.push_buffer_span_inner(Some(cache), va, byte_len, out, spans)
    }

    #[inline]
    fn push_buffer_span_inner(
        &self,
        cache: Option<&mut BufVaCache>,
        va: u64,
        byte_len: u64,
        out: &mut Vec<usize>,
        spans: &mut Vec<SubRange>,
    ) -> bool {
        let bg = self.buf_gen;
        let idx = self.buffers.partition_point(|b| b.start <= va);
        let mut eff_hi = 0u64;
        let mut match_count = 0usize;
        let mut single_idx = 0usize;
        let mut single = (0usize, 0u64, 0u64);
        let mut i = idx;
        while i > 0 {
            i -= 1;
            let b = &self.buffers[i];
            if b.max_end <= va {
                break;
            }
            if va < b.end {
                match_count += 1;
                single_idx = i;
                single = (b.key, b.start, b.end);
                let hi = if byte_len == 0 || (b.end - b.start) > u32::MAX as u64 {
                    b.end
                } else {
                    va.saturating_add(byte_len).min(b.end)
                };
                eff_hi = eff_hi.max(hi);
            }
        }
        if match_count == 0 {
            return false;
        }
        if match_count == 1 && self.buffers.get(idx).is_none_or(|next| next.start >= eff_hi) {
            let (key, start, end) = single;
            if let Some(c) = cache {
                if bg != 0 && self.is_strictly_disjoint(single_idx) {
                    c.insert(bg, va, start, end, key);
                }
            }
            if out.last() != Some(&key) && !out.contains(&key) {
                out.push(key);
            }
            push_subrange(
                spans,
                SubRange {
                    r: key,
                    lo: va.max(start),
                    hi: eff_hi.min(end),
                },
            );
            return true;
        }
        let end_idx = self.buffers.partition_point(|b| b.start < eff_hi);
        let mut j = end_idx;
        while j > 0 {
            j -= 1;
            let b = &self.buffers[j];
            if b.max_end <= va {
                break;
            }
            if va < b.end && b.start < eff_hi {
                if out.last() != Some(&b.key) && !out.contains(&b.key) {
                    out.push(b.key);
                }
                push_subrange(
                    spans,
                    SubRange {
                        r: b.key,
                        lo: va.max(b.start),
                        hi: eff_hi.min(b.end),
                    },
                );
            }
        }
        true
    }

    /// Resolve a texture view's `gpuResourceID._impl` to its owning resource key.
    #[inline]
    pub fn lookup_texture_id(&self, id: u64) -> Option<usize> {
        self.tex_ids.get(&id).copied()
    }
}

fn res_registry() -> &'static std::sync::RwLock<ResRegistry> {
    static R: std::sync::OnceLock<std::sync::RwLock<ResRegistry>> = std::sync::OnceLock::new();
    R.get_or_init(|| std::sync::RwLock::new(ResRegistry::default()))
}

#[inline]
pub fn with_resource_registry<R>(f: impl FnOnce(&ResRegistry) -> R) -> R {
    let g = res_registry().read().unwrap();
    f(&g)
}

#[inline]
fn mtl_tex_id(t: &MtlTexture) -> u64 {
    unsafe { std::mem::transmute::<MTLResourceID, u64>(t.gpuResourceID()) }
}

/// Key of a cached texture-buffer view of a buffer.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct BufferViewKey {
    format: usize,
    offset: u64,
    elements: u64,
    writable: bool,
}

/// 16-bit DXIL `@dx.op.annotateHandle` / D3D12 resource handle-kind bitmask (`g50_A`).
pub const HK_TEX1D_FLT: u32 = 1 << 0;
pub const HK_TEX1D_INT: u32 = 1 << 1;
pub const HK_TEX2D_FLT: u32 = 1 << 2;
pub const HK_TEX2D_INT: u32 = 1 << 3;
pub const HK_TEX2DMS_FLT: u32 = 1 << 4;
pub const HK_TEX2DMS_INT: u32 = 1 << 5;
pub const HK_TEX3D_FLT: u32 = 1 << 6;
pub const HK_TEX3D_INT: u32 = 1 << 7;
pub const HK_TEXCUBE_FLT: u32 = 1 << 8;
pub const HK_TEXCUBE_INT: u32 = 1 << 9;
pub const HK_FEEDBACK_TEX: u32 = 1 << 10;
pub const HK_TYPED_BUF: u32 = 1 << 11;
pub const HK_RAW_BUF: u32 = 1 << 12;
pub const HK_STRUCT_BUF: u32 = 1 << 13;
pub const HK_RAW_STRUCT_BUF: u32 = HK_RAW_BUF | HK_STRUCT_BUF;
pub const HK_ACCEL_STRUCT: u32 = 1 << 14;
pub const HK_CBV_BUF: u32 = 1 << 15;

pub const HK_ALL_TEX: u32 = (1 << 11) - 1;
pub const HK_ALL_SRV_UAV_BUF: u32 = HK_TYPED_BUF | HK_RAW_BUF | HK_STRUCT_BUF | HK_ACCEL_STRUCT;
pub const HK_ALL_BUF: u32 = HK_ALL_SRV_UAV_BUF | HK_CBV_BUF;
pub const HK_ALL: u32 = (1 << 16) - 1;

static ANY_PLACED_ALIAS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True if any two `CreatePlacedResource` allocations on the same `MTLHeap` have ever overlapped
/// in `[offset, offset + alloc_len)` (either concurrently live or within 2 submit epochs of drop).
#[inline]
pub fn any_placed_alias() -> bool {
    ANY_PLACED_ALIAS.load(Ordering::Relaxed)
}

#[cfg(test)]
#[inline]
pub(crate) fn swap_any_placed_alias_for_test(v: bool) -> bool {
    ANY_PLACED_ALIAS.swap(v, Ordering::Relaxed)
}

#[inline]
pub fn format_flt_int(dxgi: u32) -> (bool, bool) {
    match format::info(dxgi).num {
        format::Num::Float
        | format::Num::Unorm
        | format::Num::Snorm
        | format::Num::Srgb
        | format::Num::Depth => (true, false),
        format::Num::Uint | format::Num::Sint | format::Num::Stencil => (false, true),
        format::Num::Typeless | format::Num::DepthStencil | format::Num::Video => (true, true),
    }
}

pub fn desc_handle_kind_mask(desc: &ResourceDesc) -> u32 {
    if desc.is_buffer() {
        let mut m = HK_RAW_BUF | HK_STRUCT_BUF | HK_CBV_BUF;
        if (desc.Flags & resource_flags::ALLOW_UNORDERED_ACCESS) != 0 {
            m |= HK_ACCEL_STRUCT;
        }
        return m;
    }
    if desc.Format == 189 || desc.Format == 190 {
        return HK_FEEDBACK_TEX;
    }
    let (has_flt, has_int) = format_flt_int(desc.Format);
    let pick = |flt: u32, int: u32| -> u32 {
        (if has_flt { flt } else { 0 }) | (if has_int { int } else { 0 })
    };
    match desc.Dimension {
        dim::TEXTURE1D => pick(HK_TEX1D_FLT, HK_TEX1D_INT),
        dim::TEXTURE2D if desc.SampleCount > 1 => pick(HK_TEX2DMS_FLT, HK_TEX2DMS_INT),
        dim::TEXTURE2D => {
            let mut m = pick(HK_TEX2D_FLT, HK_TEX2D_INT);
            if desc.array_size() >= 6 {
                m |= pick(HK_TEXCUBE_FLT, HK_TEXCUBE_INT);
            }
            m
        }
        dim::TEXTURE3D => pick(HK_TEX3D_FLT, HK_TEX3D_INT),
        _ => HK_ALL_TEX,
    }
}

pub fn view_tex_handle_kind_mask(view_dxgi: u32, res_dxgi: u32, ty: MTLTextureType) -> u32 {
    let eff_dxgi = if view_dxgi == 0 { res_dxgi } else { view_dxgi };
    if eff_dxgi == 189 || eff_dxgi == 190 {
        return HK_FEEDBACK_TEX;
    }
    let (has_flt, has_int) = format_flt_int(eff_dxgi);
    let pick = |flt: u32, int: u32| -> u32 {
        (if has_flt { flt } else { 0 }) | (if has_int { int } else { 0 })
    };
    match ty {
        MTLTextureType::Type1D | MTLTextureType::Type1DArray => pick(HK_TEX1D_FLT, HK_TEX1D_INT),
        MTLTextureType::Type2D | MTLTextureType::Type2DArray => pick(HK_TEX2D_FLT, HK_TEX2D_INT),
        MTLTextureType::Type2DMultisample | MTLTextureType::Type2DMultisampleArray => {
            pick(HK_TEX2DMS_FLT, HK_TEX2DMS_INT)
        }
        MTLTextureType::Type3D => pick(HK_TEX3D_FLT, HK_TEX3D_INT),
        MTLTextureType::TypeCube | MTLTextureType::TypeCubeArray => {
            pick(HK_TEXCUBE_FLT, HK_TEXCUBE_INT)
        }
        _ => HK_ALL_TEX,
    }
}

/// Widens a handle-kind mask across families that can share an underlying D3D12 resource without
/// naming the resource in an unnamed `Barrier::Uav(0)` (e.g. typeless `FLT`/`INT` views, cube vs
/// 2D array views, or raw/structured/typed views of the same buffer).
#[inline]
pub const fn widen_uav0_alias_families(mut m: u32) -> u32 {
    if (m & HK_ALL_SRV_UAV_BUF) != 0 {
        m |= HK_ALL_SRV_UAV_BUF;
    }
    const TEX2D_FAMILY: u32 =
        HK_TEX2D_FLT | HK_TEX2D_INT | HK_TEXCUBE_FLT | HK_TEXCUBE_INT | HK_FEEDBACK_TEX;
    if (m & TEX2D_FAMILY) != 0 {
        m |= TEX2D_FAMILY;
    }
    if (m & (HK_TEX1D_FLT | HK_TEX1D_INT)) != 0 {
        m |= HK_TEX1D_FLT | HK_TEX1D_INT;
    }
    if (m & (HK_TEX2DMS_FLT | HK_TEX2DMS_INT)) != 0 {
        m |= HK_TEX2DMS_FLT | HK_TEX2DMS_INT;
    }
    if (m & (HK_TEX3D_FLT | HK_TEX3D_INT)) != 0 {
        m |= HK_TEX3D_FLT | HK_TEX3D_INT;
    }
    m
}

pub struct Resource {
    pub desc: ResourceDesc,
    pub heap: HeapProperties,
    pub backing: Backing,
    /// Buffer GPU virtual address (0 for textures).
    pub gpu_va: u64,
    /// Texture pixel format (Invalid for buffers).
    pub mtl_format: MTLPixelFormat,
    views: Mutex<HashMap<ViewKey, Retained<MtlTexture>>>,
    buffer_views: Mutex<HashMap<BufferViewKey, Retained<MtlTexture>>>,
    pooled_views: Mutex<HashMap<ViewKey, super::view_pool::PooledView>>,
    pooled_buffer_views: Mutex<HashMap<BufferViewKey, super::view_pool::PooledView>>,
    cpu_mirror: Mutex<Vec<u8>>,
    registered_key: std::sync::atomic::AtomicUsize,
    uav_tracked_key: std::sync::atomic::AtomicUsize,
    created_epoch: u64,
    view_epoch: std::sync::atomic::AtomicU64,
    gpu_used: std::sync::atomic::AtomicBool,
    base_handle_kinds: u32,
    extra_view_kinds: std::sync::atomic::AtomicU32,
    placed_uid: u64,
    placed_span: (u64, u64),
    placed_alias: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    /// Allocated GPU bytes for committed resources (`0` for placed resources, which are tracked by `Heap`).
    committed_bytes: u64,
    /// Heap of a placed resource (kept alive by the resource, as in D3D12;
    /// residency is the heap's).
    pub placed_in: Option<std::sync::Arc<super::heap::Heap>>,
}
unsafe impl Send for Resource {}
unsafe impl Sync for Resource {}

#[derive(Debug)]
pub enum CreateError {
    InvalidArg(String),
    OutOfMemory(String),
    Unsupported(String),
}

pub(crate) fn storage_options(heap: &HeapProperties) -> MTLResourceOptions {
    let base = MTLResourceOptions::HazardTrackingModeUntracked;
    match heap.cpu_access() {
        CpuAccess::None => base | MTLResourceOptions::StorageModePrivate,
        CpuAccess::WriteCombine => base | MTLResourceOptions::StorageModeShared | MTLResourceOptions::CPUCacheModeWriteCombined,
        CpuAccess::WriteBack => base | MTLResourceOptions::StorageModeShared,
    }
}

/// Metal texture descriptor for a D3D12 texture description.
pub fn texture_descriptor(desc: &ResourceDesc, extra_usage: MTLTextureUsage) -> Result<(Retained<MTLTextureDescriptor>, MTLPixelFormat), CreateError> {
    let flags = desc.Flags;
    let allow_depth = flags & resource_flags::ALLOW_DEPTH_STENCIL != 0;
    let fmt = format::resource_format(desc.Format, allow_depth);
    if fmt == MTLPixelFormat::Invalid {
        return Err(CreateError::Unsupported(format!("no Metal texture format for {}", format::name(desc.Format))));
    }
    let samples = desc.SampleCount.max(1) as usize;
    let arr = desc.array_size() as usize;
    let ty = match (desc.Dimension, arr > 1, samples > 1) {
        (dim::TEXTURE1D, false, _) => MTLTextureType::Type1D,
        (dim::TEXTURE1D, true, _) => MTLTextureType::Type1DArray,
        (dim::TEXTURE2D, false, false) => MTLTextureType::Type2D,
        (dim::TEXTURE2D, true, false) => MTLTextureType::Type2DArray,
        (dim::TEXTURE2D, false, true) => MTLTextureType::Type2DMultisample,
        (dim::TEXTURE2D, true, true) => MTLTextureType::Type2DMultisampleArray,
        (dim::TEXTURE3D, _, _) => MTLTextureType::Type3D,
        _ => return Err(CreateError::InvalidArg(format!("texture dimension {}", desc.Dimension))),
    };
    let td = MTLTextureDescriptor::new();
    td.setTextureType(ty);
    td.setPixelFormat(fmt);
    unsafe {
        td.setWidth(desc.Width.max(1) as usize);
        td.setHeight(desc.Height.max(1) as usize);
        td.setDepth(desc.depth() as usize);
        td.setArrayLength(arr);
        td.setMipmapLevelCount(desc.mip_levels() as usize);
        td.setSampleCount(samples);
    }
    td.setStorageMode(MTLStorageMode::Private);
    td.setHazardTrackingMode(MTLHazardTrackingMode::Untracked);
    let mut usage = MTLTextureUsage::ShaderRead | extra_usage;
    if flags & (resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_DEPTH_STENCIL) != 0 {
        usage |= MTLTextureUsage::RenderTarget;
    }
    if flags & resource_flags::ALLOW_UNORDERED_ACCESS != 0
        || (flags & resource_flags::ALLOW_RENDER_TARGET != 0
            && desc.Dimension == dim::TEXTURE2D
            && desc.SampleCount <= 1
            && super::gpunan::needs_rt_shader_write(fmt))
    {
        usage |= MTLTextureUsage::ShaderWrite;
    }
    let fi = format::info(desc.Format);
    // PixelFormatView disables lossless compression, and Metal needs it only
    // for views that change the pixel layout: sRGB/linear casts don't, and
    // the members of a D3D12 typeless colour family share one layout.
    // Depth/stencil resources keep it for plane and depth-as-colour views.
    if allow_depth || fi.has_stencil() || fmt == MTLPixelFormat::Depth32Float_Stencil8 {
        usage |= MTLTextureUsage::PixelFormatView;
    }
    td.setUsage(usage);
    Ok((td, fmt))
}

/// D3D12 placement alignments.
pub const SMALL_ALIGNMENT: u64 = 4 << 10;
pub const DEFAULT_ALIGNMENT: u64 = 64 << 10;
pub const MSAA_ALIGNMENT: u64 = 4 << 20;

/// `GetResourceAllocationInfo` for one desc: (size, alignment). Sizes come
/// from Metal's heap placement requirements, rounded up to the D3D12
/// alignment so placed resources can be sub-allocated in an `MTLHeap`.
pub fn allocation_info(desc: &ResourceDesc) -> Result<(u64, u64), CreateError> {
    let d = super::device().ok_or_else(|| CreateError::Unsupported("no device".into()))?;
    if desc.is_buffer() {
        let a = DEFAULT_ALIGNMENT;
        return Ok((desc.Width.max(1).div_ceil(a) * a, a));
    }
    let (td, _) = texture_descriptor(desc, MTLTextureUsage::empty())?;
    let sa = d.dev.heapTextureSizeAndAlignWithDescriptor(&td);
    let req = match desc.Alignment {
        0 if desc.SampleCount > 1 => MSAA_ALIGNMENT,
        0 => DEFAULT_ALIGNMENT,
        a => a,
    };
    let align = req.max(sa.align as u64);
    Ok(((sa.size as u64).max(1).div_ceil(align) * align, align))
}

/// Diagnostic: a colour render target that is also a UAV gets ShaderWrite
/// usage, which may cost it lossless compression. Logs each one (debug) and
/// a running summary (info) at power-of-two counts.
fn note_rt_uav(desc: &ResourceDesc, t: &MtlTexture, label: &str, placed: bool) {
    let f = desc.Flags;
    if f & resource_flags::ALLOW_RENDER_TARGET == 0 || f & resource_flags::ALLOW_UNORDERED_ACCESS == 0 {
        return;
    }
    static COUNT: AtomicU64 = AtomicU64::new(0);
    static LARGE: AtomicU64 = AtomicU64::new(0);
    static BYTES: AtomicU64 = AtomicU64::new(0);
    let bytes = MTLResource::allocatedSize(t) as u64;
    let large = desc.Width * desc.Height as u64 >= 1 << 20;
    let n = COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let nl = LARGE.fetch_add(large as u64, Ordering::Relaxed) + large as u64;
    let total = BYTES.fetch_add(bytes, Ordering::Relaxed) + bytes;
    log::debug!(
        target: "d3d",
        "RT+UAV texture: {}x{}x{} mips {} samples {} {} {:.1} MB{} '{label}'",
        desc.Width,
        desc.Height,
        desc.DepthOrArraySize,
        desc.mip_levels(),
        desc.SampleCount.max(1),
        format::name(desc.Format),
        bytes as f64 / 1048576.0,
        if placed { " placed" } else { "" }
    );
    if n.is_power_of_two() {
        log::info!(target: "d3d", "RT+UAV textures so far: {n} ({nl} >= 1 Mpx), {:.1} MB", total as f64 / 1048576.0);
    }
}

impl Resource {
    /// `CreateCommittedResource` with default Metal texture usage flags.
    pub fn new_committed_default(desc: &ResourceDesc, heap: &HeapProperties, label: &str) -> Result<Resource, CreateError> {
        Self::new_committed(desc, heap, MTLTextureUsage::empty(), label)
    }

    /// `CreateCommittedResource`.
    pub fn new_committed(desc: &ResourceDesc, heap: &HeapProperties, extra_usage: MTLTextureUsage, label: &str) -> Result<Resource, CreateError> {
        objc2::rc::autoreleasepool(|_| Self::new_committed_inner(desc, heap, extra_usage, label))
    }

    fn new_committed_inner(desc: &ResourceDesc, heap: &HeapProperties, extra_usage: MTLTextureUsage, label: &str) -> Result<Resource, CreateError> {
        let d = super::device().ok_or_else(|| CreateError::Unsupported("no device".into()))?;
        let (backing, gpu_va, mtl_format, committed_bytes) = if desc.is_buffer() {
            if desc.Height != 1 || desc.DepthOrArraySize != 1 || desc.Format != 0 {
                return Err(CreateError::InvalidArg(format!("bad buffer desc {desc:?}")));
            }
            let len = (desc.Width as usize).max(1);
            let opts = match heap.cpu_access() {
                CpuAccess::None => MTLResourceOptions::StorageModeShared | MTLResourceOptions::HazardTrackingModeUntracked,
                _ => storage_options(heap),
            };
            let b = d
                .dev
                .newBufferWithLength_options(len, opts)
                .ok_or_else(|| CreateError::OutOfMemory(format!("newBuffer({len})")))?;
            let va = b.gpuAddress();
            let alloc = (MTLResource::allocatedSize(&*b) as u64).max(len as u64);
            super::device::note_cbuf_alloc(alloc);
            if cfg!(target_os = "ios") && alloc >= (16 << 20) {
                log::info!(target: "d3d", "large cbuf: {:.1} MiB heap_type={} label={label:?}", alloc as f64 / 1048576.0, heap.Type);
            }
            record_buffer_va(va, len as u64);
            (Backing::Buffer(b), va, MTLPixelFormat::Invalid, alloc)
        } else {
            if heap.cpu_access() != CpuAccess::None {
                return Err(CreateError::Unsupported(format!("texture on a CPU-visible heap ({:?})", heap)));
            }
            let (td, fmt) = texture_descriptor(desc, extra_usage)?;
            let t = d.dev.newTextureWithDescriptor(&td).ok_or_else(|| CreateError::OutOfMemory(format!("newTexture({desc:?})")))?;
            let alloc = MTLResource::allocatedSize(&*t) as u64;
            super::device::note_ctex_alloc(alloc);
            if cfg!(target_os = "ios") && alloc >= (16 << 20) {
                log::info!(
                    target: "d3d",
                    "large ctex: {:.1} MiB {}x{}x{} mips={} fmt={} flags={:#x} label={label:?}",
                    alloc as f64 / 1048576.0,
                    desc.Width,
                    desc.Height,
                    desc.DepthOrArraySize,
                    desc.mip_levels(),
                    format::name(desc.Format),
                    desc.Flags
                );
            }
            note_rt_uav(desc, &t, label, false);
            (Backing::Texture(t), 0, fmt, alloc)
        };
        let mut norm_desc = *desc;
        if !norm_desc.is_buffer() && norm_desc.MipLevels == 0 {
            norm_desc.MipLevels = norm_desc.mip_levels() as u16;
        }
        let base_handle_kinds = desc_handle_kind_mask(&norm_desc);
        let r = Resource {
            desc: norm_desc,
            heap: *heap,
            backing,
            gpu_va,
            mtl_format,
            views: Mutex::new(HashMap::new()),
            buffer_views: Mutex::new(HashMap::new()),
            pooled_views: Mutex::new(HashMap::new()),
            pooled_buffer_views: Mutex::new(HashMap::new()),
            cpu_mirror: Mutex::new(Vec::new()),
            registered_key: std::sync::atomic::AtomicUsize::new(0),
            uav_tracked_key: std::sync::atomic::AtomicUsize::new(0),
            created_epoch: super::queue::submit_epoch(),
            view_epoch: std::sync::atomic::AtomicU64::new(u64::MAX),
            gpu_used: std::sync::atomic::AtomicBool::new(false),
            base_handle_kinds,
            extra_view_kinds: std::sync::atomic::AtomicU32::new(0),
            placed_uid: 0,
            placed_span: (0, 0),
            placed_alias: None,
            committed_bytes,
            placed_in: None,
        };
        r.set_label(label);
        d.add_resident(r.allocation());
        Ok(r)
    }

    /// `CreatePlacedResource` with default Metal texture usage flags.
    pub fn new_placed_default(heap: &std::sync::Arc<super::heap::Heap>, offset: u64, desc: &ResourceDesc, label: &str) -> Result<Resource, CreateError> {
        Self::new_placed(heap, offset, desc, MTLTextureUsage::empty(), label)
    }

    /// `CreatePlacedResource`: sub-allocate from `heap` at `offset`.
    pub fn new_placed(heap: &std::sync::Arc<super::heap::Heap>, offset: u64, desc: &ResourceDesc, extra_usage: MTLTextureUsage, label: &str) -> Result<Resource, CreateError> {
        objc2::rc::autoreleasepool(|_| Self::new_placed_inner(heap, offset, desc, extra_usage, label))
    }

    fn new_placed_inner(heap: &std::sync::Arc<super::heap::Heap>, offset: u64, desc: &ResourceDesc, extra_usage: MTLTextureUsage, label: &str) -> Result<Resource, CreateError> {
        let hp = heap.desc.Properties;
        let (backing, gpu_va, mtl_format, alloc_len) = if desc.is_buffer() {
            if desc.Height != 1 || desc.DepthOrArraySize != 1 || desc.Format != 0 {
                return Err(CreateError::InvalidArg(format!("bad buffer desc {desc:?}")));
            }
            let len = (desc.Width as usize).max(1);
            if offset + len as u64 > heap.desc.SizeInBytes {
                return Err(CreateError::InvalidArg(format!("buffer [{offset:#x}, +{len:#x}) outside heap of {:#x}", heap.desc.SizeInBytes)));
            }
            let b = unsafe { heap.heap.newBufferWithLength_options_offset(len, heap.heap.resourceOptions(), offset as usize) }
                .ok_or_else(|| CreateError::OutOfMemory(format!("heap newBuffer({len}) at {offset:#x}")))?;
            let va = b.gpuAddress();
            let alen = (MTLResource::allocatedSize(&*b) as u64).max(len as u64);
            record_buffer_va(va, len as u64);
            let retired = crate::rt::accel::evict_range(va, len as u64);
            if !retired.is_empty() {
                super::queue::defer_release_all(retired);
            }
            (Backing::Buffer(b), va, MTLPixelFormat::Invalid, alen)
        } else {
            if hp.cpu_access() != CpuAccess::None {
                return Err(CreateError::Unsupported(format!("texture on a CPU-visible heap ({hp:?})")));
            }
            let (td, fmt) = texture_descriptor(desc, extra_usage)?;
            let t = unsafe { heap.heap.newTextureWithDescriptor_offset(&td, offset as usize) }
                .ok_or_else(|| CreateError::OutOfMemory(format!("heap newTexture({desc:?}) at {offset:#x}")))?;
            let alen = (MTLResource::allocatedSize(&*t) as u64).max(1);
            note_rt_uav(desc, &t, label, true);
            (Backing::Texture(t), 0, fmt, alen)
        };
        let mut norm_desc = *desc;
        if !norm_desc.is_buffer() && norm_desc.MipLevels == 0 {
            norm_desc.MipLevels = norm_desc.mip_levels() as u16;
        }
        let base_handle_kinds = desc_handle_kind_mask(&norm_desc);
        let lo = offset;
        let hi = offset.saturating_add(alloc_len.max(1));
        let cur_epoch = super::queue::submit_epoch();
        let (placed_uid, placed_alias) = {
            let mut tr = heap.placed_spans.lock().unwrap();
            tr.next_uid = tr.next_uid.wrapping_add(1).max(1);
            let uid = tr.next_uid;
            let mut my_alias = false;
            for &(_, o_lo, o_hi, ref o_flag) in &tr.live {
                if lo < o_hi && o_lo < hi {
                    my_alias = true;
                    o_flag.store(true, Ordering::Release);
                }
            }
            if !my_alias {
                for &(r_lo, r_hi, r_epoch) in &tr.retired {
                    if cur_epoch <= r_epoch.saturating_add(2) && lo < r_hi && r_lo < hi {
                        my_alias = true;
                        break;
                    }
                }
            }
            if my_alias {
                ANY_PLACED_ALIAS.store(true, Ordering::Relaxed);
            }
            let flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(my_alias));
            tr.live.push((uid, lo, hi, flag.clone()));
            (uid, flag)
        };
        let r = Resource {
            desc: norm_desc,
            heap: hp,
            backing,
            gpu_va,
            mtl_format,
            views: Mutex::new(HashMap::new()),
            buffer_views: Mutex::new(HashMap::new()),
            pooled_views: Mutex::new(HashMap::new()),
            pooled_buffer_views: Mutex::new(HashMap::new()),
            cpu_mirror: Mutex::new(Vec::new()),
            registered_key: std::sync::atomic::AtomicUsize::new(0),
            uav_tracked_key: std::sync::atomic::AtomicUsize::new(0),
            created_epoch: 0,
            view_epoch: std::sync::atomic::AtomicU64::new(0),
            gpu_used: std::sync::atomic::AtomicBool::new(true),
            base_handle_kinds,
            extra_view_kinds: std::sync::atomic::AtomicU32::new(0),
            placed_uid,
            placed_span: (lo, hi),
            placed_alias: Some(placed_alias),
            committed_bytes: 0,
            placed_in: Some(heap.clone()),
        };
        r.set_label(label);
        Ok(r)
    }

    /// True if this resource is placed on an `MTLHeap` AND overlaps in `[offset, offset + size)`
    /// with another live (or recently retired within 2 submit epochs) placed resource on the same heap.
    /// Non-overlapping sub-allocated placed resources return `false`.
    #[inline]
    pub fn may_alias_placed(&self) -> bool {
        self.placed_alias
            .as_ref()
            .is_some_and(|f| f.load(Ordering::Acquire))
    }

    #[inline]
    pub fn handle_kind_mask(&self) -> u32 {
        self.base_handle_kinds | self.extra_view_kinds.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn note_view_kind(&self, mask: u32) {
        if (mask & !self.base_handle_kinds) != 0 {
            self.extra_view_kinds.fetch_or(mask, Ordering::Relaxed);
        }
    }

    #[inline]
    pub fn mark_gpu_used(&self) {
        if !self.gpu_used.load(Ordering::Relaxed) {
            self.gpu_used.store(true, Ordering::Relaxed);
        }
    }

    /// True when this standalone committed resource has never been recorded into prior
    /// GPU work and could not have been accessed by any prior submitted `Fragment` stage.
    #[inline]
    pub fn is_fresh_nonfrag_blit_safe(&self) -> bool {
        if self.placed_in.is_some() || self.gpu_used.load(Ordering::Relaxed) {
            return false;
        }
        let cur = super::queue::submit_epoch();
        if self.created_epoch == cur {
            return true;
        }
        !self.desc.is_buffer()
            && (self.desc.Flags
                & (resource_flags::ALLOW_RENDER_TARGET
                    | resource_flags::ALLOW_DEPTH_STENCIL
                    | resource_flags::ALLOW_UNORDERED_ACCESS))
                == 0
            && self.view_epoch.load(Ordering::Relaxed) >= cur
    }

    #[inline]
    pub(crate) fn note_uav_tracked_key(&self, key: usize) -> usize {
        self.uav_tracked_key.swap(key, Ordering::Relaxed)
    }

    /// Record this `Resource`'s stable heap address (`&*Arc<Resource> as *const Resource as usize`)
    /// in the process-wide `ResRegistry` so compute descriptor-table / root-VA bindings can resolve
    /// `gpu_va` and `MTLResourceID` back to D3D12 resource identity keys (`r_key`).
    #[inline]
    pub fn register_ptr(&self) {
        let key = self as *const Resource as usize;
        if self.registered_key.load(Ordering::Relaxed) == key {
            return;
        }
        let prev = self.registered_key.swap(key, Ordering::Relaxed);
        if prev == key {
            return;
        }
        let views = self.views.lock().unwrap();
        let bviews = self.buffer_views.lock().unwrap();
        let pviews = self.pooled_views.lock().unwrap();
        let pbviews = self.pooled_buffer_views.lock().unwrap();
        let mut reg = res_registry().write().unwrap();
        if prev != 0 && self.gpu_va != 0 {
            reg.remove_buffer(self.gpu_va, prev);
        }
        if self.gpu_va != 0 {
            reg.insert_buffer(self.gpu_va, self.gpu_va + self.desc.Width.max(1), key);
        }
        if let Some(t) = self.texture() {
            reg.insert_tex_id(mtl_tex_id(t), key);
        }
        for tv in views.values() {
            reg.insert_tex_id(mtl_tex_id(tv), key);
        }
        for tv in bviews.values() {
            reg.insert_tex_id(mtl_tex_id(tv), key);
        }
        for pv in pviews.values() {
            reg.insert_tex_id(pv.id, key);
        }
        for pv in pbviews.values() {
            reg.insert_tex_id(pv.id, key);
        }
    }

    pub fn allocation(&self) -> &ProtocolObject<dyn MTLAllocation> {
        match &self.backing {
            Backing::Buffer(b) => ProtocolObject::from_ref(&**b),
            Backing::Texture(t) => ProtocolObject::from_ref(&**t),
        }
    }

    /// Allocated backing size in bytes as reported by Metal (`MTLAllocation::allocatedSize`).
    pub fn allocated_bytes(&self) -> u64 {
        match &self.backing {
            Backing::Buffer(b) => MTLAllocation::allocatedSize(&**b) as u64,
            Backing::Texture(t) => MTLAllocation::allocatedSize(&**t) as u64,
        }
    }

    pub fn set_label(&self, s: &str) {
        if s.is_empty() {
            return;
        }
        let ns = objc2_foundation::NSString::from_str(s);
        match &self.backing {
            Backing::Buffer(b) => b.setLabel(Some(&ns)),
            Backing::Texture(t) => t.setLabel(Some(&ns)),
        }
    }

    pub fn buffer(&self) -> Option<&MtlBuffer> {
        match &self.backing {
            Backing::Buffer(b) => Some(b),
            _ => None,
        }
    }

    pub fn texture(&self) -> Option<&MtlTexture> {
        match &self.backing {
            Backing::Texture(t) => Some(t),
            _ => None,
        }
    }

    /// CPU pointer of a mappable buffer.
    pub fn cpu_ptr(&self) -> Option<*mut u8> {
        match &self.backing {
            Backing::Buffer(b) if self.heap.cpu_access() != CpuAccess::None => Some(b.contents().as_ptr() as *mut u8),
            _ => None,
        }
    }

    /// Record a CPU-visible upload into `self.cpu_mirror` so CPU-staged private
    /// buffers (such as indirect argument buffers) can be inspected on the CPU.
    pub fn record_cpu_copy(&self, dst_off: u64, src_bytes: &[u8]) {
        if self.cpu_ptr().is_some() || src_bytes.is_empty() {
            return;
        }
        let end = (dst_off as usize).saturating_add(src_bytes.len());
        if end > (16 << 20) {
            return;
        }
        let mut m = self.cpu_mirror.lock().unwrap();
        if m.len() < end {
            m.resize(end, 0);
        }
        m[dst_off as usize..end].copy_from_slice(src_bytes);
    }

    /// Zero-copy CPU slice of a `StorageModeShared` buffer on unified memory.
    pub fn shared_slice(&self, offset: u64, len: usize) -> Option<&[u8]> {
        if let Backing::Buffer(b) = &self.backing {
            if b.storageMode() == MTLStorageMode::Shared {
                let p = b.contents().as_ptr() as *const u8;
                let total = b.length();
                let off = offset as usize;
                if off >= total {
                    return None;
                }
                let avail = (total - off).min(len);
                return Some(unsafe { std::slice::from_raw_parts(p.add(off), avail) });
            }
        }
        None
    }

    /// Read up to `len` bytes starting at `offset` from a mappable or shared
    /// buffer, or its `cpu_mirror` if populated via `CopyBufferRegion` / `CopyResource`.
    pub fn cpu_bytes(&self, offset: u64, len: usize) -> Option<Vec<u8>> {
        if let Some(s) = self.shared_slice(offset, len) {
            return Some(s.to_vec());
        }
        let m = self.cpu_mirror.lock().unwrap();
        let off = offset as usize;
        if off >= m.len() {
            return None;
        }
        let avail = (m.len() - off).min(len);
        Some(m[off..off + avail].to_vec())
    }

    /// A texture (view) with `fmt`, mip range and slice range, cached on the
    /// resource. Returns the base texture when nothing differs.
    pub fn texture_view(&self, fmt: MTLPixelFormat, ty: MTLTextureType, mips: (u32, u32), slices: (u32, u32)) -> Option<Retained<MtlTexture>> {
        self.texture_view_swizzled(fmt, ty, mips, slices, SWIZZLE_IDENTITY)
    }

    /// [`Self::texture_view`] with a component swizzle (packed
    /// `MTLTextureSwizzle` bytes r,g,b,a).
    pub fn texture_view_swizzled(&self, fmt: MTLPixelFormat, ty: MTLTextureType, mips: (u32, u32), slices: (u32, u32), swizzle: u32) -> Option<Retained<MtlTexture>> {
        self.texture_view_swizzled_min_lod(fmt, ty, mips, slices, swizzle, 0.0)
    }

    /// [`Self::texture_view_swizzled`] with an optional Metal 4.1 (`macOS 27` / `iOS 27`)
    /// `MTLTextureViewDescriptor.minLOD` clamp (`min_lod` is resource-relative, matching
    /// D3D12 `ResourceMinLODClamp` and D3D11 `SetResourceMinLOD`).
    ///
    /// Active when `min_lod > mips.0` and `Device::supports_texture_view_min_lod()` (`Apple10`+
    /// with `PORT_METAL41_VIEW_MIN_LOD` / `PORT_METAL41`); falls back cleanly to standard
    /// texture views on `Apple9` and earlier, on macOS 26, or when disabled.
    pub fn texture_view_swizzled_min_lod(
        &self,
        fmt: MTLPixelFormat,
        ty: MTLTextureType,
        mips: (u32, u32),
        slices: (u32, u32),
        swizzle: u32,
        min_lod: f32,
    ) -> Option<Retained<MtlTexture>> {
        self.view_epoch.fetch_min(super::queue::submit_epoch(), Ordering::Relaxed);
        let t = self.texture()?;
        let full_mips = (0, t.mipmapLevelCount() as u32);
        let full_slices = (0, (t.arrayLength() as u32).max(1) * if t.textureType() == MTLTextureType::TypeCube || t.textureType() == MTLTextureType::TypeCubeArray { 6 } else { 1 });
        // Metal 4.1 applies `MTLTextureViewDescriptor.minLOD` after adding `levelRange.location`
        // (`mips.0`), so both D3D `ResourceMinLODClamp` and Metal `minLOD` are resource-relative.
        let abs_min_lod = if super::device().is_some_and(|d| d.supports_texture_view_min_lod()) {
            resolve_view_min_lod(mips, min_lod)
        } else {
            0.0
        };
        let min_lod_bits = if abs_min_lod > 0.0 { abs_min_lod.to_bits() } else { 0 };
        if fmt == self.mtl_format
            && ty == t.textureType()
            && mips == full_mips
            && slices == full_slices
            && swizzle == SWIZZLE_IDENTITY
            && min_lod_bits == 0
        {
            return Some(t.retain());
        }
        let key = ViewKey { format: fmt.0, ty: ty.0, mips, slices, swizzle, min_lod_bits };
        let mut v = self.views.lock().unwrap();
        if let Some(x) = v.get(&key) {
            return Some(x.clone());
        }
        let levels = objc2_foundation::NSRange::new(mips.0 as usize, mips.1 as usize);
        let sl = objc2_foundation::NSRange::new(slices.0 as usize, slices.1 as usize);
        let view_with_min_lod = if min_lod_bits != 0 {
            unsafe {
                create_texture_view_with_min_lod(t, fmt, ty, levels, sl, swizzle, abs_min_lod)
            }
        } else {
            None
        };
        let view = match view_with_min_lod {
            Some(tv) => tv,
            None => unsafe {
                if swizzle == SWIZZLE_IDENTITY {
                    t.newTextureViewWithPixelFormat_textureType_levels_slices(fmt, ty, levels, sl)
                } else {
                    t.newTextureViewWithPixelFormat_textureType_levels_slices_swizzle(fmt, ty, levels, sl, unpack_swizzle(swizzle))
                }
            }?,
        };
        let rk = self.registered_key.load(Ordering::Relaxed);
        if rk != 0 {
            res_registry().write().unwrap().insert_tex_id(mtl_tex_id(&view), rk);
        }
        v.insert(key, view.clone());
        Some(view)
    }

    /// A texture-buffer view (typed buffer SRV/UAV) over `elements`
    /// elements of `fmt` starting at byte `offset` (which must satisfy the
    /// device's texture-buffer alignment).
    pub fn buffer_texture_view(&self, fmt: MTLPixelFormat, offset: u64, elements: u64, writable: bool) -> Option<Retained<MtlTexture>> {
        self.view_epoch.fetch_min(super::queue::submit_epoch(), Ordering::Relaxed);
        let b = self.buffer()?;
        let key = BufferViewKey { format: fmt.0, offset, elements, writable };
        let mut v = self.buffer_views.lock().unwrap();
        if let Some(x) = v.get(&key) {
            return Some(x.clone());
        }
        let td = unsafe { MTLTextureDescriptor::textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage(
            fmt,
            elements.max(1) as usize,
            b.resourceOptions(),
            if writable {
                MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite | MTLTextureUsage::PixelFormatView
            } else {
                MTLTextureUsage::ShaderRead
            },
        ) };
        let bpe = texture_buffer_bpe(fmt);
        let t = b.newTextureWithDescriptor_offset_bytesPerRow(&td, offset as usize, (elements.max(1) * bpe) as usize)?;
        let rk = self.registered_key.load(Ordering::Relaxed);
        if rk != 0 {
            res_registry().write().unwrap().insert_tex_id(mtl_tex_id(&t), rk);
        }
        v.insert(key, t.clone());
        Some(t)
    }

    /// Lightweight `MTLResourceID` for a shader-visible texture view.
    ///
    /// Returns the base texture's `gpuResourceID` for identity views, reuses an
    /// already-cached view, or allocates from the Metal 4 [`super::view_pool`]
    /// without constructing an Objective-C `MTLTexture` object (falling back to
    /// [`Self::texture_view_swizzled`] when pooling is disabled or unavailable).
    pub fn texture_view_id(&self, fmt: MTLPixelFormat, ty: MTLTextureType, mips: (u32, u32), slices: (u32, u32)) -> Option<u64> {
        self.texture_view_id_swizzled(fmt, ty, mips, slices, SWIZZLE_IDENTITY)
    }

    /// [`Self::texture_view_id`] with a packed `MTLTextureSwizzle` (r,g,b,a bytes).
    pub fn texture_view_id_swizzled(&self, fmt: MTLPixelFormat, ty: MTLTextureType, mips: (u32, u32), slices: (u32, u32), swizzle: u32) -> Option<u64> {
        self.texture_view_id_swizzled_min_lod(fmt, ty, mips, slices, swizzle, 0.0)
    }

    /// [`Self::texture_view_id_swizzled`] with an optional Metal 4.1 (`Apple10`+)
    /// `MTLTextureViewDescriptor.minLOD` clamp (`min_lod` is resource-relative, matching
    /// D3D12 `ResourceMinLODClamp` and D3D11 `SetResourceMinLOD`).
    ///
    /// When `PORT_METAL41_VIEW_MIN_LOD` is off or the GPU is `< Apple10`, `min_lod_bits == 0`
    /// and this takes the exact baseline `MTLTextureViewPool` path.
    pub fn texture_view_id_swizzled_min_lod(
        &self,
        fmt: MTLPixelFormat,
        ty: MTLTextureType,
        mips: (u32, u32),
        slices: (u32, u32),
        swizzle: u32,
        min_lod: f32,
    ) -> Option<u64> {
        self.view_epoch.fetch_min(super::queue::submit_epoch(), Ordering::Relaxed);
        let t = self.texture()?;
        let full_mips = (0, t.mipmapLevelCount() as u32);
        let full_slices = (0, (t.arrayLength() as u32).max(1) * if t.textureType() == MTLTextureType::TypeCube || t.textureType() == MTLTextureType::TypeCubeArray { 6 } else { 1 });
        let abs_min_lod = if super::device().is_some_and(|d| d.supports_texture_view_min_lod()) {
            resolve_view_min_lod(mips, min_lod)
        } else {
            0.0
        };
        let min_lod_bits = if abs_min_lod > 0.0 { abs_min_lod.to_bits() } else { 0 };
        if fmt == self.mtl_format
            && ty == t.textureType()
            && mips == full_mips
            && slices == full_slices
            && swizzle == SWIZZLE_IDENTITY
            && min_lod_bits == 0
        {
            return Some(mtl_tex_id(t));
        }
        let key = ViewKey { format: fmt.0, ty: ty.0, mips, slices, swizzle, min_lod_bits };
        if let Some(existing) = self.views.lock().unwrap().get(&key) {
            return Some(mtl_tex_id(existing));
        }
        {
            let mut pv_map = self.pooled_views.lock().unwrap();
            if let Some(pv) = pv_map.get(&key) {
                return Some(pv.id);
            }
            if let Some(pv) = super::view_pool::create_texture_view_with_min_lod(
                t,
                fmt,
                ty,
                mips,
                slices,
                unpack_swizzle(swizzle),
                abs_min_lod,
            ) {
                let rk = self.registered_key.load(Ordering::Relaxed);
                if rk != 0 {
                    res_registry().write().unwrap().insert_tex_id(pv.id, rk);
                }
                pv_map.insert(key, pv);
                return Some(pv.id);
            }
        }
        self.texture_view_swizzled_min_lod(fmt, ty, mips, slices, swizzle, min_lod).map(|v| mtl_tex_id(&v))
    }

    /// Lightweight `MTLResourceID` for a shader-visible typed-buffer or UAV
    /// counter texture-buffer view, backed by Metal 4 [`super::view_pool`] when
    /// enabled (falling back to [`Self::buffer_texture_view`]).
    pub fn buffer_texture_view_id(&self, fmt: MTLPixelFormat, offset: u64, elements: u64, writable: bool) -> Option<u64> {
        self.view_epoch.fetch_min(super::queue::submit_epoch(), Ordering::Relaxed);
        let b = self.buffer()?;
        let key = BufferViewKey { format: fmt.0, offset, elements, writable };
        if let Some(existing) = self.buffer_views.lock().unwrap().get(&key) {
            return Some(mtl_tex_id(existing));
        }
        {
            let mut pbv_map = self.pooled_buffer_views.lock().unwrap();
            if let Some(pv) = pbv_map.get(&key) {
                return Some(pv.id);
            }
            if super::view_pool::enabled() {
                let td = unsafe { MTLTextureDescriptor::textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage(
                    fmt,
                    elements.max(1) as usize,
                    b.resourceOptions(),
                    if writable {
                        MTLTextureUsage::ShaderRead | MTLTextureUsage::ShaderWrite | MTLTextureUsage::PixelFormatView
                    } else {
                        MTLTextureUsage::ShaderRead
                    },
                ) };
                let bpe = texture_buffer_bpe(fmt);
                let bytes_per_row = elements.max(1) * bpe;
                if let Some(pv) = super::view_pool::create_buffer_texture_view(b, &td, offset, bytes_per_row) {
                    let rk = self.registered_key.load(Ordering::Relaxed);
                    if rk != 0 {
                        res_registry().write().unwrap().insert_tex_id(pv.id, rk);
                    }
                    pbv_map.insert(key, pv);
                    return Some(pv.id);
                }
            }
        }
        self.buffer_texture_view(fmt, offset, elements, writable).map(|v| mtl_tex_id(&v))
    }
}

/// Resolves a Direct3D resource-relative `ResourceMinLODClamp` / `SetResourceMinLOD`
/// into a Metal 4.1 `MTLTextureViewDescriptor.minLOD` (also resource-relative, applied
/// after adding `levelRange.location` = `mips.0`).
///
/// Returns `0.0` when `min_lod` does not restrict beyond `mips.0` (`MostDetailedMip`),
/// otherwise clamps to `mips.0 + mips.1` (`levelRange.location + levelRange.length`).
pub(crate) fn resolve_view_min_lod(mips: (u32, u32), min_lod: f32) -> f32 {
    if !min_lod.is_finite() || min_lod <= mips.0 as f32 || mips.1 == 0 {
        return 0.0;
    }
    let max_abs_lod = mips.0.saturating_add(mips.1) as f32;
    min_lod.min(max_abs_lod)
}

fn texture_buffer_bpe(fmt: MTLPixelFormat) -> u64 {
    match fmt.0 {
        120 | 121 | 122 => 12u64, // Metal 4.1 Apple10+ MTLPixelFormatRGB32{Uint,Sint,Float}
        95..=99 => 6u64,          // Metal 4.1 Apple10+ MTLPixelFormatRGB16*
        45..=48 => 3u64,          // Metal 4.1 Apple10+ MTLPixelFormatRGB8*
        _ => crate::format::TABLE.iter().find(|f| f.mtl == fmt).map(|f| f.bytes).unwrap_or(4) as u64,
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        let uav_key = self.uav_tracked_key.load(Ordering::Relaxed);
        if uav_key != 0 {
            super::descriptor::forget_uav_resource(uav_key);
        }
        super::descriptor::forget_uav_resource(self as *const Resource as usize);
        let rk = self.registered_key.load(Ordering::Relaxed);
        let pviews = self.pooled_views.get_mut().unwrap();
        let pbviews = self.pooled_buffer_views.get_mut().unwrap();
        let mut retired_slots = Vec::with_capacity(pviews.len() + pbviews.len());
        for pv in pviews.values() {
            retired_slots.push(pv.slot);
        }
        for pv in pbviews.values() {
            retired_slots.push(pv.slot);
        }
        if rk != 0 {
            let views = self.views.get_mut().unwrap();
            let bviews = self.buffer_views.get_mut().unwrap();
            let mut reg = res_registry().write().unwrap();
            if self.gpu_va != 0 {
                reg.remove_buffer(self.gpu_va, rk);
            }
            if let Backing::Texture(t) = &self.backing {
                reg.remove_tex_id(mtl_tex_id(t));
            }
            for tv in views.values() {
                reg.remove_tex_id(mtl_tex_id(tv));
            }
            for tv in bviews.values() {
                reg.remove_tex_id(mtl_tex_id(tv));
            }
            for pv in pviews.values() {
                reg.remove_tex_id(pv.id);
            }
            for pv in pbviews.values() {
                reg.remove_tex_id(pv.id);
            }
        }
        if !retired_slots.is_empty() {
            super::view_pool::retire_slots(retired_slots);
        }
        if self.gpu_va != 0 {
            let retired = crate::rt::accel::evict_range(self.gpu_va, self.desc.Width);
            if !retired.is_empty() {
                super::queue::defer_release_all(retired);
            }
        }
        if let Some(h) = &self.placed_in {
            if self.placed_uid != 0 {
                let cur_epoch = super::queue::submit_epoch();
                let mut tr = h.placed_spans.lock().unwrap();
                tr.live.retain(|&(uid, _, _, _)| uid != self.placed_uid);
                let (lo, hi) = self.placed_span;
                if lo < hi {
                    tr.retired.push((lo, hi, cur_epoch));
                }
                if tr.retired.len() > 256 {
                    tr.retired.retain(|&(_, _, ep)| ep.saturating_add(4) >= cur_epoch);
                }
            }
            return;
        }
        if self.committed_bytes != 0 {
            match &self.backing {
                Backing::Buffer(_) => super::device::note_cbuf_free(self.committed_bytes),
                Backing::Texture(_) => super::device::note_ctex_free(self.committed_bytes),
            }
        }
        if let Some(d) = super::device() {
            d.remove_resident(self.allocation());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Brute-force reference matching the exact `push_buffer_span` contract
    /// (without the single-buffer fast path) for differential verification.
    fn push_buffer_span_ref(
        reg: &ResRegistry,
        va: u64,
        byte_len: u64,
        out: &mut Vec<usize>,
        spans: &mut Vec<SubRange>,
    ) -> bool {
        let mut eff_hi = 0u64;
        let mut found = false;
        for b in &reg.buffers {
            if b.start <= va && va < b.end {
                found = true;
                let hi = if byte_len == 0 || (b.end - b.start) > u32::MAX as u64 {
                    b.end
                } else {
                    va.saturating_add(byte_len).min(b.end)
                };
                eff_hi = eff_hi.max(hi);
            }
        }
        if !found {
            return false;
        }
        for b in &reg.buffers {
            if va < b.end && b.start < eff_hi {
                if !out.contains(&b.key) {
                    out.push(b.key);
                }
                push_subrange(
                    spans,
                    SubRange {
                        r: b.key,
                        lo: va.max(b.start),
                        hi: eff_hi.min(b.end),
                    },
                );
            }
        }
        true
    }

    fn assert_span_matches_ref(reg: &ResRegistry, va: u64, byte_len: u64, seed_out: &[usize], seed_spans: &[SubRange]) {
        let mut out_fast = seed_out.to_vec();
        let mut spans_fast = seed_spans.to_vec();
        let ok_fast = reg.push_buffer_span(va, byte_len, &mut out_fast, &mut spans_fast);

        let mut out_ref = seed_out.to_vec();
        let mut spans_ref = seed_spans.to_vec();
        let ok_ref = push_buffer_span_ref(reg, va, byte_len, &mut out_ref, &mut spans_ref);

        assert_eq!(ok_fast, ok_ref, "match status mismatch at va={va:#x} len={byte_len:#x}");
        out_fast.sort_unstable();
        out_ref.sort_unstable();
        assert_eq!(out_fast, out_ref, "keys mismatch at va={va:#x} len={byte_len:#x}");
        spans_fast.sort_unstable_by_key(|s| (s.r, s.lo, s.hi));
        spans_ref.sort_unstable_by_key(|s| (s.r, s.lo, s.hi));
        assert_eq!(spans_fast, spans_ref, "spans mismatch at va={va:#x} len={byte_len:#x}");
    }

    #[test]
    fn push_buffer_span_fast_and_aliasing_paths_match_reference() {
        let mut reg = ResRegistry::default();
        // Disjoint non-aliasing buffers (fast path).
        reg.insert_buffer(0x1000, 0x2000, 1);
        reg.insert_buffer(0x2000, 0x3000, 2);
        // Overlapping/aliasing placed buffers:
        //   b3: [0x4000, 0x5000) key=3
        //   b4: [0x4000, 0x4400) key=4  (same start -> match_count > 1 at 0x4100)
        //   b5: [0x4600, 0x4c00) key=5  (starts strictly after va=0x4500 inside eff_hi=0x4900)
        //   b6: [0x4900, 0x4f00) key=6  (starts exactly at eff_hi=0x4900 -> excluded for [0x4500, 0x4900))
        reg.insert_buffer(0x4000, 0x5000, 3);
        reg.insert_buffer(0x4000, 0x4400, 4);
        reg.insert_buffer(0x4600, 0x4c00, 5);
        reg.insert_buffer(0x4900, 0x4f00, 6);

        // 1) Misses (before first, gap between 0x3000 and 0x4000, after last).
        assert_span_matches_ref(&reg, 0x0800, 0x100, &[], &[]);
        assert_span_matches_ref(&reg, 0x3500, 0x100, &[], &[]);
        assert_span_matches_ref(&reg, 0x6000, 0x100, &[], &[]);

        // 2) Single-buffer fast path: bounded, unbounded (byte_len == 0), clamped to end,
        //    adjacent next buffer starting right at eff_hi (0x2000), and coalescing with existing spans.
        assert_span_matches_ref(&reg, 0x1100, 0x200, &[], &[]);
        assert_span_matches_ref(&reg, 0x1100, 0, &[], &[]);
        assert_span_matches_ref(&reg, 0x1f00, 0x500, &[], &[]);
        assert_span_matches_ref(
            &reg,
            0x1200,
            0x300,
            &[1],
            &[SubRange { r: 1, lo: 0x1000, hi: 0x1200 }],
        );
        assert_span_matches_ref(&reg, 0x2000, 0x1000, &[], &[]);

        // 3) Aliasing placed buffers with match_count > 1 at va (b3 + b4).
        assert_span_matches_ref(&reg, 0x4100, 0x100, &[], &[]);
        assert_span_matches_ref(&reg, 0x4100, 0, &[], &[]);

        // 4) Single match at va=0x4500 (only b3 contains 0x4500), with b5 starting at
        //    va < 0x4600 < eff_hi: must fall back to slow path and include both b3 and b5,
        //    while excluding b6 starting at == eff_hi (0x4900).
        let mut out = Vec::new();
        let mut spans = Vec::new();
        assert!(reg.push_buffer_span(0x4500, 0x400, &mut out, &mut spans));
        out.sort_unstable();
        spans.sort_unstable_by_key(|s| (s.r, s.lo, s.hi));
        assert_eq!(out, vec![3, 5]);
        assert_eq!(
            spans,
            vec![
                SubRange { r: 3, lo: 0x4500, hi: 0x4900 },
                SubRange { r: 5, lo: 0x4600, hi: 0x4900 },
            ]
        );
        assert_span_matches_ref(&reg, 0x4500, 0x400, &[], &[]);
        // Tighter span [0x4500, 0x4600) stops right at b5.start -> fast path, only b3.
        assert_span_matches_ref(&reg, 0x4500, 0x100, &[], &[]);
        // Wider span [0x4500, 0x4a00) reaches into b6 -> includes b3, b5, b6.
        assert_span_matches_ref(&reg, 0x4500, 0x500, &[], &[]);
        // Unbounded root VA at 0x4500 -> covers [0x4500, 0x5000) -> includes b3, b5, b6.
        assert_span_matches_ref(&reg, 0x4500, 0, &[], &[]);
    }

    #[test]
    fn tex_id_lookup_with_u64_mix_hasher() {
        let mut reg = ResRegistry::default();
        let ids: [u64; 6] = [
            1,
            2,
            0x100,
            0x200,
            0x0001_0000_0000_0000,
            0xDEAD_BEEF_CAFE_0040,
        ];
        for (i, &id) in ids.iter().enumerate() {
            reg.insert_tex_id(id, i + 10);
        }
        for (i, &id) in ids.iter().enumerate() {
            assert_eq!(reg.lookup_texture_id(id), Some(i + 10));
        }
        assert_eq!(reg.lookup_texture_id(0x9999), None);
        reg.remove_tex_id(ids[2]);
        assert_eq!(reg.lookup_texture_id(ids[2]), None);
        assert_eq!(reg.lookup_texture_id(ids[3]), Some(13));
    }

    #[test]
    fn buf_va_cache_hits_disjoint_and_invalidates_on_alias_or_remove() {
        let mut reg = ResRegistry::default();
        reg.insert_buffer(0x10_0000, 0x12_0000, 101);
        reg.insert_buffer(0x20_0000, 0x24_0000, 202);

        // Prime cache and verify repeated hits across distinct offsets inside the same buffer.
        let mut out = Vec::new();
        let mut spans = Vec::new();
        assert!(reg.push_buffer_va(0x10_0100, &mut out));
        assert_eq!(out, vec![101]);
        out.clear();
        assert!(reg.push_buffer_span(0x11_F000, 0x2000, &mut out, &mut spans));
        assert_eq!(out, vec![101]);
        assert_eq!(spans, vec![SubRange { r: 101, lo: 0x11_F000, hi: 0x12_0000 }]);

        // Inserting an overlapping placed buffer MUST invalidate the cached disjoint entry.
        reg.insert_buffer(0x11_0000, 0x11_8000, 303);
        out.clear();
        spans.clear();
        assert!(reg.push_buffer_va(0x11_1000, &mut out));
        out.sort_unstable();
        assert_eq!(out, vec![101, 303]);
        out.clear();
        assert!(reg.push_buffer_span(0x10_8000, 0xA000, &mut out, &mut spans));
        out.sort_unstable();
        assert_eq!(out, vec![101, 303]);

        // Removing buffer 202 MUST invalidate its cached entry immediately.
        out.clear();
        assert!(reg.push_buffer_va(0x21_0000, &mut out));
        assert_eq!(out, vec![202]);
        reg.remove_buffer(0x20_0000, 202);
        out.clear();
        assert!(!reg.push_buffer_va(0x21_0000, &mut out));
        assert!(out.is_empty());
    }

    #[test]
    fn buf_va_cache_per_cmdlist_invalidation_on_recreate_same_va_and_aliasing() {
        let mut reg = ResRegistry::default();
        let mut cache_a = BufVaCache::new();
        let mut cache_b = BufVaCache::new();
        reg.insert_buffer(0x10_0000, 0x12_0000, 101);
        reg.insert_buffer(0x20_0000, 0x24_0000, 202);

        let mut out = Vec::new();
        let mut spans = Vec::new();

        // Prime both per-command-list caches (tests `last` MRU hit, `page` table hit, and `mru[4]`).
        assert!(reg.push_buffer_va_cached(&mut cache_a, 0x10_0040, &mut out));
        assert_eq!(out, vec![101]);
        assert!(reg.push_buffer_va_cached(&mut cache_a, 0x10_0800, &mut out));
        assert_eq!(out, vec![101], "duplicate key should dedup via out.last() fast check");
        out.clear();
        assert!(reg.push_buffer_span_cached(&mut cache_a, 0x11_F000, 0x2000, &mut out, &mut spans));
        assert_eq!(out, vec![101]);
        assert_eq!(spans, vec![SubRange { r: 101, lo: 0x11_F000, hi: 0x12_0000 }]);

        out.clear();
        assert!(reg.push_buffer_va_cached(&mut cache_b, 0x20_0100, &mut out));
        assert_eq!(out, vec![202]);

        // Destroy resource 101 and recreate a DIFFERENT resource 909 at the EXACT same VA range.
        // Both `last` and `page`/`mru` entries in `cache_a` MUST invalidate via `buf_gen`.
        reg.remove_buffer(0x10_0000, 101);
        reg.insert_buffer(0x10_0000, 0x12_0000, 909);
        out.clear();
        spans.clear();
        assert!(reg.push_buffer_va_cached(&mut cache_a, 0x10_0040, &mut out));
        assert_eq!(out, vec![909], "recreated resource at identical VA must not return stale key 101");
        out.clear();
        assert!(reg.push_buffer_span_cached(&mut cache_a, 0x10_1000, 0x400, &mut out, &mut spans));
        assert_eq!(out, vec![909]);
        assert_eq!(spans, vec![SubRange { r: 909, lo: 0x10_1000, hi: 0x10_1400 }]);

        // Insert an overlapping/aliasing placed buffer inside [0x20_0000, 0x24_0000).
        // `cache_b` (primed with disjoint 202) MUST invalidate and return BOTH 202 and 404.
        reg.insert_buffer(0x20_0000, 0x22_0000, 404);
        out.clear();
        spans.clear();
        assert!(reg.push_buffer_va_cached(&mut cache_b, 0x20_0100, &mut out));
        out.sort_unstable();
        assert_eq!(out, vec![202, 404]);
        out.clear();
        assert!(reg.push_buffer_span_cached(&mut cache_b, 0x21_8000, 0x1_0000, &mut out, &mut spans));
        out.sort_unstable();
        spans.sort_unstable_by_key(|s| (s.r, s.lo, s.hi));
        assert_eq!(out, vec![202, 404]);
        assert_eq!(
            spans,
            vec![
                SubRange { r: 202, lo: 0x21_8000, hi: 0x22_8000 },
                SubRange { r: 404, lo: 0x21_8000, hi: 0x22_0000 },
            ]
        );

        // Removing the aliasing resource restores single-buffer disjoint caching for 202.
        reg.remove_buffer(0x20_0000, 404);
        out.clear();
        assert!(reg.push_buffer_va_cached(&mut cache_b, 0x20_0100, &mut out));
        assert_eq!(out, vec![202]);
    }
}
