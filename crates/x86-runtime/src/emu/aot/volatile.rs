//! PE `IMAGE_LOAD_CONFIG_DIRECTORY64.VolatileMetadataPointer` (`+0x100`) runtime registry.
//!
//! MSVC 2019+ (`/volatileMetadata`, used by Windows 11 on ARM `xtajit64` and FEX-Emu 2504)
//! emits an `IMAGE_VOLATILE_METADATA` table listing:
//! - `VolatileInfoRangeTable`: sorted half-open code RVA ranges `[start_rva, start_rva + len)`
//!   certified by the compiler as having complete volatile-access metadata;
//! - `VolatileAccessTable`: sorted instruction RVAs within those ranges that perform volatile
//!   or atomic memory accesses and therefore MUST retain x86 TSO acquire/release ordering.
//!
//! Outside covered ranges (or in PEs without volatile metadata), full TSO ordering is preserved.
//! Inside covered ranges, non-`LOCK` memory accesses whose instruction RVA is NOT in
//! `VolatileAccessTable` may safely emit plain ARM64 `ldr`/`str` instructions when
//! `PORT_JIT_VOLATILE_META=1`.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use super::cfg_discover::{data_dir, opt_image_base, pe_va_to_rva};

/// Parsed, relocation-invariant volatile metadata for one mapped PE module.
#[derive(Debug, Clone)]
pub(in crate::emu) struct ModuleVolatileMeta {
    pub(super) base: u64,
    pub(super) end: u64,
    /// Sorted, non-overlapping covered code RVA half-open ranges `[start_rva, end_rva)`.
    pub(super) ranges: Box<[(u32, u32)]>,
    /// Sorted, deduplicated volatile instruction RVAs.
    pub(super) access_rvas: Box<[u32]>,
}

impl ModuleVolatileMeta {
    /// True if `va` lies inside this module AND is explicitly listed in `VolatileAccessTable`.
    #[inline]
    pub(in crate::emu) fn is_volatile_access_va(&self, va: u64) -> bool {
        if va < self.base || va >= self.end {
            return false;
        }
        let rva = (va - self.base) as u32;
        self.access_rvas.binary_search(&rva).is_ok()
    }

    /// True if `va` lies inside a compiler-certified covered range AND is NOT listed in
    /// `VolatileAccessTable`.
    #[inline]
    pub(in crate::emu) fn can_elide_tso_va(&self, va: u64) -> bool {
        if va < self.base || va >= self.end {
            return false;
        }
        let rva = (va - self.base) as u32;
        let idx = self.ranges.partition_point(|&(s, _)| s <= rva);
        if idx == 0 || rva >= self.ranges[idx - 1].1 {
            return false;
        }
        self.access_rvas.binary_search(&rva).is_err()
    }
}

static HAS_VOLATILE_MODULES: AtomicBool = AtomicBool::new(false);
static VOLATILE_MODULES: RwLock<Vec<Arc<ModuleVolatileMeta>>> = RwLock::new(Vec::new());

/// Parse `IMAGE_VOLATILE_METADATA` directly from a mapped PE32+ image `[base, base + size)`.
pub(super) unsafe fn parse_mapped_volatile_metadata(
    base: usize,
    size: usize,
) -> Option<ModuleVolatileMeta> {
    let (dir_rva, _dir_len) = data_dir(base, size, 10)?;
    let size64 = size as u64;
    let fits = |off: u64, n: u64| off.checked_add(n).is_some_and(|e| e <= size64);
    let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
    let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);

    if !fits(dir_rva as u64, 4) {
        return None;
    }
    let avail = size64.saturating_sub(dir_rva as u64);
    let lc_size = (rd32(dir_rva as u64) as u64).min(avail);
    // `VolatileMetadataPointer` sits at offset +0x100 (8 bytes).
    if lc_size < 0x108 {
        return None;
    }
    let vm_va = rd64(dir_rva as u64 + 0x100);
    if vm_va == 0 {
        return None;
    }
    let opt_base = opt_image_base(base, size);
    let vm_rva = pe_va_to_rva(base, size, opt_base, vm_va)? as u64;
    if !fits(vm_rva, 24) {
        return None;
    }
    let vm_size = rd32(vm_rva);
    if vm_size < 24 {
        return None;
    }
    let access_rva = rd32(vm_rva + 8) as u64;
    let access_size = rd32(vm_rva + 12) as u64;
    let range_rva = rd32(vm_rva + 16) as u64;
    let range_size = rd32(vm_rva + 20) as u64;

    if range_rva == 0 || range_size < 8 || (range_size & 7) != 0 || !fits(range_rva, range_size) {
        return None;
    }
    if access_size != 0
        && (access_rva == 0 || (access_size & 3) != 0 || !fits(access_rva, access_size))
    {
        return None;
    }

    let n_ranges = (range_size / 8) as usize;
    let mut raw_ranges: Vec<(u32, u32)> = Vec::with_capacity(n_ranges);
    for i in 0..n_ranges {
        let o = range_rva + (i as u64) * 8;
        let start = rd32(o);
        let len = rd32(o + 4);
        if len == 0 {
            continue;
        }
        let end = start.saturating_add(len);
        if end > start {
            raw_ranges.push((start, end));
        }
    }
    if raw_ranges.is_empty() {
        return None;
    }
    raw_ranges.sort_unstable();
    let mut merged: Vec<(u32, u32)> = Vec::with_capacity(raw_ranges.len());
    for (s, e) in raw_ranges {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 {
                last.1 = last.1.max(e);
                continue;
            }
        }
        merged.push((s, e));
    }

    let n_access = (access_size / 4) as usize;
    let mut access_rvas: Vec<u32> = Vec::with_capacity(n_access);
    for i in 0..n_access {
        access_rvas.push(rd32(access_rva + (i as u64) * 4));
    }
    access_rvas.sort_unstable();
    access_rvas.dedup();

    Some(ModuleVolatileMeta {
        base: base as u64,
        end: (base as u64).saturating_add(size as u64),
        ranges: merged.into_boxed_slice(),
        access_rvas: access_rvas.into_boxed_slice(),
    })
}

/// Parse and register `IMAGE_VOLATILE_METADATA` (if present) when a PE module is mapped.
/// Always clears any stale overlapping entries first so remapping a module without
/// `VolatileMetadata` at a reused address range never inherits an older module's table.
pub(super) unsafe fn register_mapped_module(name: &str, base: usize, size: usize) {
    let b = base as u64;
    let e = b.saturating_add(size as u64);
    let parsed = parse_mapped_volatile_metadata(base, size);
    let log_info = parsed.as_ref().map(|meta| {
        let covered_bytes: u64 = meta.ranges.iter().map(|&(s, en)| (en - s) as u64).sum();
        (meta.ranges.len(), covered_bytes, meta.access_rvas.len())
    });
    if let Ok(mut g) = VOLATILE_MODULES.write() {
        g.retain(|m| m.end <= b || m.base >= e);
        if let Some(meta) = parsed {
            g.push(Arc::new(meta));
            g.sort_unstable_by_key(|m| m.base);
        }
        HAS_VOLATILE_MODULES.store(!g.is_empty(), Ordering::Release);
    }
    if let Some((n_ranges, covered_bytes, n_access)) = log_info {
        log::info!(
            "emu aot: {name}: loaded PE VolatileMetadata ({n_ranges} ranges, {} KiB covered, {n_access} volatile access RVAs)",
            covered_bytes >> 10
        );
    }
}

/// Look up the registered [`ModuleVolatileMeta`] owning guest code address `ip`.
#[inline]
pub(in crate::emu) fn lookup_module_for_ip(ip: u64) -> Option<Arc<ModuleVolatileMeta>> {
    if !HAS_VOLATILE_MODULES.load(Ordering::Acquire) {
        return None;
    }
    let g = VOLATILE_MODULES.read().ok()?;
    let idx = g.partition_point(|m| m.base <= ip);
    let m = g.get(idx.checked_sub(1)?)?;
    (ip < m.end).then(|| Arc::clone(m))
}

#[cfg(test)]
pub(in crate::emu) fn test_register_volatile_meta(
    base: u64,
    size: u64,
    ranges: &[(u32, u32)],
    access_rvas: &[u32],
) {
    let mut r = ranges.to_vec();
    r.sort_unstable();
    let mut a = access_rvas.to_vec();
    a.sort_unstable();
    a.dedup();
    let arc = Arc::new(ModuleVolatileMeta {
        base,
        end: base.saturating_add(size),
        ranges: r.into_boxed_slice(),
        access_rvas: a.into_boxed_slice(),
    });
    if let Ok(mut g) = VOLATILE_MODULES.write() {
        g.retain(|m| m.base != base);
        g.push(arc);
        g.sort_unstable_by_key(|m| m.base);
        HAS_VOLATILE_MODULES.store(true, Ordering::Release);
    }
}

#[cfg(test)]
pub(in crate::emu) fn test_unregister_volatile_meta(base: u64) {
    if let Ok(mut g) = VOLATILE_MODULES.write() {
        g.retain(|m| m.base != base);
        if g.is_empty() {
            HAS_VOLATILE_MODULES.store(false, Ordering::Release);
        }
    }
}
