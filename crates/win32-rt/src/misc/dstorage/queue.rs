//! DirectStorage `IDStorageFile`, `IDStorageStatusArray`, GPU copy uploader,
//! `IDStorageQueue` / `Queue1` / `Queue2`, and `IDStorageCustomDecompressionQueue` /
//! `Queue1` implementations.

use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::fs::File;
use std::os::unix::fs::{FileExt, MetadataExt};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::layout::{copyable_footprints, plane_format, PlacedFootprint};
use d3d12_metal::metal::cmdlist::{
    split_subresource, CommandAllocator as MtlCommandAllocator, CommandList as MtlCommandList,
    CopyLoc,
};
use d3d12_metal::metal::queue::{ListType, Queue as MtlQueue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};

use super::abi::*;
use super::factory::{factory_singleton, DEBUG_FLAGS, STAGING_BUFFER_SIZE};
use super::gdeflate::decompress_gdeflate_to_vec;
use crate::com::{
    com_add_ref, com_call_slot, com_from_iface, com_release, qi_helper, read_guid,
    register_vtable_once, write_null_out, ComObj, Guid, IID_IUnknown,
};
use crate::d3d12::com::Obj;
use crate::handles::Event;
use crate::strings::{le_u16, le_u32, le_u64, rd, read_ansi_bytes_z, wr};
use crate::types::{unix_to_filetime, FALSE};

// ============================================================================
// IDStorageFile (IUnknown + Close + GetFileInformation)
// ============================================================================

pub(super) struct DStorageFileState {
    pub(super) guest_path: String,
    pub(super) file: Mutex<Option<File>>,
}

struct FileObj {
    state: Arc<DStorageFileState>,
}

impl DStorageFileState {
    fn read_exact_at(&self, offset: u64, len: usize) -> Result<Vec<u8>, i32> {
        let guard = self.file.lock().unwrap();
        let Some(ref f) = *guard else {
            return Err(E_HANDLE);
        };
        if len == 0 {
            return Ok(Vec::new());
        }
        let end = offset.checked_add(len as u64).ok_or(E_DSTORAGE_END_OF_FILE)?;
        let file_len = f.metadata().map_err(|e| io_error_to_hresult(&e))?.len();
        if end > file_len {
            return Err(E_DSTORAGE_END_OF_FILE);
        }
        let mut buf = vec![0u8; len];
        f.read_exact_at(&mut buf, offset)
            .map_err(|e| io_error_to_hresult(&e))?;
        Ok(buf)
    }

    fn close(&self) {
        *self.file.lock().unwrap() = None;
    }
}

unsafe extern "win64" fn ds_file_qi(
    this: *mut ComObj<FileObj>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    qi_helper(this, riid, ppv, FILE_IIDS)
}

unsafe extern "win64" fn ds_file_close(this: *mut c_void) {
    if let Some(obj) = com_from_iface::<FileObj>(this, file_vtable()) {
        obj.inner.state.close();
    }
}

unsafe extern "win64" fn ds_file_get_info(this: *mut c_void, info_out: *mut u8) -> i32 {
    if info_out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = com_from_iface::<FileObj>(this, file_vtable()) else {
        return E_INVALIDARG;
    };
    let guard = obj.inner.state.file.lock().unwrap();
    let Some(ref f) = *guard else {
        return E_HANDLE;
    };
    let meta = match f.metadata() {
        Ok(m) => m,
        Err(e) => return io_error_to_hresult(&e),
    };
    let size = meta.len();
    let ft_create = unix_to_filetime(meta.ctime(), meta.ctime_nsec());
    let ft_access = unix_to_filetime(meta.atime(), meta.atime_nsec());
    let ft_write = unix_to_filetime(meta.mtime(), meta.mtime_nsec());

    // BY_HANDLE_FILE_INFORMATION is 52 bytes.
    std::ptr::write_bytes(info_out, 0, 52);
    wr(info_out as *mut u32, 0x80u32); // FILE_ATTRIBUTE_NORMAL
    wr(info_out.add(4) as *mut u32, ft_create as u32);
    wr(info_out.add(8) as *mut u32, (ft_create >> 32) as u32);
    wr(info_out.add(12) as *mut u32, ft_access as u32);
    wr(info_out.add(16) as *mut u32, (ft_access >> 32) as u32);
    wr(info_out.add(20) as *mut u32, ft_write as u32);
    wr(info_out.add(24) as *mut u32, (ft_write >> 32) as u32);
    wr(info_out.add(28) as *mut u32, meta.dev() as u32);
    wr(info_out.add(32) as *mut u32, (size >> 32) as u32);
    wr(info_out.add(36) as *mut u32, size as u32);
    wr(info_out.add(40) as *mut u32, meta.nlink() as u32);
    wr(info_out.add(44) as *mut u32, (meta.ino() >> 32) as u32);
    wr(info_out.add(48) as *mut u32, meta.ino() as u32);
    S_OK
}

fn file_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 5]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            ds_file_qi as *const () as usize,
            com_add_ref::<FileObj> as *const () as usize,
            com_release::<FileObj> as *const () as usize,
            ds_file_close as *const () as usize,
            ds_file_get_info as *const () as usize,
        ]
    })
}

pub(super) fn new_file_obj(guest_path: String, file: File) -> usize {
    let state = Arc::new(DStorageFileState {
        guest_path,
        file: Mutex::new(Some(file)),
    });
    ComObj::new(file_vtable(), FileObj { state }) as usize
}

// ============================================================================
// IDStorageStatusArray (IUnknown + IsComplete + GetHResult)
// ============================================================================

#[derive(Clone, Copy)]
struct StatusEntry {
    complete: bool,
    hresult: i32,
}

pub(super) struct StatusArrayState {
    entries: Mutex<Vec<StatusEntry>>,
}

struct StatusArrayObj {
    state: Arc<StatusArrayState>,
}

impl StatusArrayState {
    fn new(capacity: usize) -> Self {
        Self {
            entries: Mutex::new(vec![
                StatusEntry {
                    complete: true,
                    hresult: S_OK,
                };
                capacity
            ]),
        }
    }

    fn mark_pending(&self, index: u32) -> bool {
        let mut entries = self.entries.lock().unwrap();
        let Some(entry) = entries.get_mut(index as usize) else {
            return false;
        };
        entry.complete = false;
        entry.hresult = E_PENDING;
        true
    }

    fn complete(&self, index: u32, hresult: i32) -> bool {
        let mut entries = self.entries.lock().unwrap();
        let Some(entry) = entries.get_mut(index as usize) else {
            return false;
        };
        entry.hresult = hresult;
        entry.complete = true;
        true
    }

    fn is_complete(&self, index: u32) -> bool {
        self.entries
            .lock()
            .unwrap()
            .get(index as usize)
            .is_some_and(|e| e.complete)
    }

    fn get_hresult(&self, index: u32) -> i32 {
        match self.entries.lock().unwrap().get(index as usize) {
            Some(e) if e.complete => e.hresult,
            Some(_) => E_PENDING,
            None => E_INVALIDARG,
        }
    }
}

unsafe extern "win64" fn ds_status_qi(
    this: *mut ComObj<StatusArrayObj>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    qi_helper(this, riid, ppv, STATUS_ARRAY_IIDS)
}

unsafe extern "win64" fn ds_status_is_complete(this: *mut c_void, index: u32) -> u8 {
    let Some(obj) = com_from_iface::<StatusArrayObj>(this, status_vtable()) else {
        return 0;
    };
    obj.inner.state.is_complete(index) as u8
}

unsafe extern "win64" fn ds_status_get_hresult(this: *mut c_void, index: u32) -> i32 {
    let Some(obj) = com_from_iface::<StatusArrayObj>(this, status_vtable()) else {
        return E_INVALIDARG;
    };
    obj.inner.state.get_hresult(index)
}

fn status_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 5]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            ds_status_qi as *const () as usize,
            com_add_ref::<StatusArrayObj> as *const () as usize,
            com_release::<StatusArrayObj> as *const () as usize,
            ds_status_is_complete as *const () as usize,
            ds_status_get_hresult as *const () as usize,
        ]
    })
}

pub(super) fn new_status_array_obj(capacity: usize) -> usize {
    let state = Arc::new(StatusArrayState::new(capacity));
    ComObj::new(status_vtable(), StatusArrayObj { state }) as usize
}

// ============================================================================
// GPU Copy Uploader (via d3d12_metal safe Rust APIs — zero objc2-metal)
// ============================================================================

const MAX_STAGED_UPLOAD_BYTES: u64 = 64 * 1024 * 1024;

/// Standard D3D12 64 KiB tile dimensions in texels `(tile_w, tile_h, tile_d)`.
/// Returns `None` for formats without a standard 64 KiB tile shape.
pub(super) fn standard_tile_shape(desc: &ResourceDesc, plane: u32) -> Option<(u32, u32, u32)> {
    let fi = d3d12_metal::format::info(desc.Format);
    if fi.bytes == 0 {
        return None;
    }
    let bw = fi.block_w.max(1);
    let bh = fi.block_h.max(1);
    let bpe = match (desc.Format, plane) {
        (44..=47, 0) | (19..=22, 0) => 4u32,
        (44..=47, _) | (19..=22, _) => 1u32,
        _ => fi.bytes,
    };
    let (mut tw, mut th, td) = match desc.Dimension {
        dim::TEXTURE3D => match bpe {
            1 => (64 * bw, 32 * bh, 32),
            2 => (32 * bw, 32 * bh, 32),
            4 => (32 * bw, 32 * bh, 16),
            8 => (32 * bw, 16 * bh, 16),
            16 => (16 * bw, 16 * bh, 16),
            _ => return None,
        },
        dim::TEXTURE2D => match bpe {
            1 => (256 * bw, 256 * bh, 1),
            2 => (256 * bw, 128 * bh, 1),
            4 => (128 * bw, 128 * bh, 1),
            8 => (128 * bw, 64 * bh, 1),
            16 => (64 * bw, 64 * bh, 1),
            _ => return None,
        },
        dim::TEXTURE1D => match bpe {
            1 | 2 | 4 | 8 | 16 => ((D3D12_TILE_SIZE_BYTES as u32 / bpe) * bw, 1, 1),
            _ => return None,
        },
        _ => return None,
    };
    if desc.Dimension == dim::TEXTURE2D {
        match desc.SampleCount.max(1) {
            1 => {}
            2 => tw = (tw / 2).max(bw),
            4 => {
                tw = (tw / 2).max(bw);
                th = (th / 2).max(bh);
            }
            8 => {
                tw = (tw / 4).max(bw);
                th = (th / 2).max(bh);
            }
            16 => {
                tw = (tw / 4).max(bw);
                th = (th / 4).max(bh);
            }
            _ => return None,
        }
    }
    Some((tw, th, td))
}

struct CopyUploader {
    queue: MtlQueue,
    alloc: MtlCommandAllocator,
    cl: MtlCommandList,
    open: bool,
    staging: Vec<Arc<MtlResource>>,
    staged_bytes: u64,
}

impl CopyUploader {
    fn new() -> Result<Self, i32> {
        let queue = MtlQueue::new(ListType::Copy, "DirectStorage copy").ok_or(E_FAIL)?;
        let alloc = MtlCommandAllocator::new(ListType::Copy).ok_or(E_OUTOFMEMORY)?;
        let cl = MtlCommandList::new(ListType::Copy).ok_or(E_OUTOFMEMORY)?;
        Ok(Self {
            queue,
            alloc,
            cl,
            open: false,
            staging: Vec::new(),
            staged_bytes: 0,
        })
    }

    fn ensure_open(&mut self) -> Result<(), i32> {
        if !self.open {
            self.alloc.reset();
            self.cl.begin(&self.alloc).map_err(|_| E_FAIL)?;
            self.open = true;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), i32> {
        if !self.open {
            return Ok(());
        }
        let close_res = self.cl.close();
        self.open = false;
        if close_res.is_err() {
            self.staging.clear();
            self.staged_bytes = 0;
            return Err(E_FAIL);
        }
        let v = self.queue.submit(&[&self.cl.cb]);
        self.queue.timeline.wait_cpu(v, u64::MAX);
        self.staging.clear();
        self.staged_bytes = 0;
        Ok(())
    }

    fn alloc_upload_buffer(size: u64, label: &str) -> Result<Arc<MtlResource>, i32> {
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: size.max(16),
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties {
            Type: heap_type::UPLOAD,
            ..Default::default()
        };
        let r = MtlResource::new_committed_default(&desc, &hp, label).map_err(|e| match e {
            d3d12_metal::metal::resource::CreateError::OutOfMemory(_) => E_OUTOFMEMORY,
            _ => E_FAIL,
        })?;
        let arc = Arc::new(r);
        arc.register_ptr();
        Ok(arc)
    }

    fn maybe_flush_staging(&mut self) -> Result<(), i32> {
        let limit = (STAGING_BUFFER_SIZE.load(Ordering::Relaxed) as u64)
            .clamp(1024 * 1024, MAX_STAGED_UPLOAD_BYTES);
        if self.staged_bytes >= limit {
            self.flush()?;
        }
        Ok(())
    }

    fn upload_buffer(&mut self, dst: &Arc<MtlResource>, offset: u64, data: &[u8]) -> Result<(), i32> {
        if data.is_empty() {
            return Ok(());
        }
        if let Some(cpu) = dst.cpu_ptr() {
            self.flush()?;
            unsafe {
                std::ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    cpu.add(offset as usize),
                    data.len(),
                );
            }
            return Ok(());
        }
        let up = Self::alloc_upload_buffer(data.len() as u64, "dstorage buf upload")?;
        let cpu = up.cpu_ptr().ok_or(E_FAIL)?;
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), cpu, data.len());
        }
        self.ensure_open()?;
        self.cl.barrier();
        self.cl
            .copy_buffer(dst, offset, &up, 0, data.len() as u64)
            .map_err(|_| E_FAIL)?;
        self.cl.barrier();
        self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
        self.staging.push(up);
        self.maybe_flush_staging()
    }

    fn upload_texture_region(
        &mut self,
        dst: &Arc<MtlResource>,
        subresource: u32,
        region: D3D12Box,
        data: &[u8],
    ) -> Result<(), i32> {
        if dst.desc.is_buffer() || subresource >= dst.desc.subresource_count() {
            return Err(E_INVALIDARG);
        }
        if region.right <= region.left
            || region.bottom <= region.top
            || region.back <= region.front
        {
            return Err(E_INVALIDARG);
        }
        let (mip, _slice, plane) = split_subresource(dst, subresource);
        let mw = (dst.desc.Width >> mip).max(1) as u32;
        let mh = (dst.desc.Height >> mip).max(1);
        let md = if dst.desc.Dimension == dim::TEXTURE3D {
            (dst.desc.depth() >> mip).max(1)
        } else {
            1
        };
        let fi = d3d12_metal::format::info(dst.desc.Format);
        if fi.bytes == 0 {
            return Err(E_INVALIDARG);
        }
        let bw = fi.block_w.max(1);
        let bh = fi.block_h.max(1);
        let bpe = match (dst.desc.Format, plane) {
            (44..=47, 0) | (19..=22, 0) => 4usize,
            (44..=47, _) | (19..=22, _) => 1usize,
            _ => fi.bytes as usize,
        };
        if region.left % bw != 0
            || region.top % bh != 0
            || region.right > mw.next_multiple_of(bw)
            || region.bottom > mh.next_multiple_of(bh)
            || region.back > md
        {
            return Err(E_INVALIDARG);
        }

        let w = region.right - region.left;
        let h = region.bottom - region.top;
        let d = region.back - region.front;
        let row_bytes = (w.div_ceil(bw) as usize)
            .checked_mul(bpe)
            .ok_or(E_INVALIDARG)?;
        let num_rows = h.div_ceil(bh) as usize;
        let slice_bytes = row_bytes.checked_mul(num_rows).ok_or(E_INVALIDARG)?;
        let needed_bytes = slice_bytes.checked_mul(d as usize).ok_or(E_INVALIDARG)?;
        if data.len() < needed_bytes {
            return Err(E_INVALIDARG);
        }

        let dst_row_pitch = row_bytes.next_multiple_of(256).max(256);
        let dst_slice_pitch = dst_row_pitch.checked_mul(num_rows).ok_or(E_OUTOFMEMORY)?;
        let total = dst_slice_pitch
            .checked_mul(d as usize)
            .ok_or(E_OUTOFMEMORY)?;
        let up = Self::alloc_upload_buffer(total as u64, "dstorage tex region upload")?;
        let cpu = up.cpu_ptr().ok_or(E_FAIL)?;
        for z in 0..d as usize {
            for y in 0..num_rows {
                let soff = z * slice_bytes + y * row_bytes;
                let doff = z * dst_slice_pitch + y * dst_row_pitch;
                unsafe {
                    std::ptr::copy_nonoverlapping(data.as_ptr().add(soff), cpu.add(doff), row_bytes);
                }
            }
        }
        let fp = PlacedFootprint {
            offset: 0,
            format: plane_format(dst.desc.Format, plane),
            width: w.next_multiple_of(bw),
            height: h.next_multiple_of(bh),
            depth: d,
            row_pitch: dst_row_pitch as u32,
            _pad: 0,
        };
        self.ensure_open()?;
        self.cl.barrier();
        self.cl
            .copy_texture_region(
                CopyLoc::Subresource(dst, subresource),
                [region.left, region.top, region.front],
                CopyLoc::Footprint(&up, fp),
                None,
            )
            .map_err(|_| E_FAIL)?;
        self.cl.barrier();
        self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
        self.staging.push(up);
        self.maybe_flush_staging()
    }

    fn upload_multiple_subresources(
        &mut self,
        dst: &Arc<MtlResource>,
        first: u32,
        count: u32,
        data: &[u8],
    ) -> Result<(), i32> {
        if dst.desc.is_buffer() {
            return Err(E_INVALIDARG);
        }
        if count == 0 {
            return Ok(());
        }
        let total_subs = dst.desc.subresource_count();
        if first.checked_add(count).is_none_or(|end| end > total_subs) {
            return Err(E_INVALIDARG);
        }
        let (layouts, total_aligned) =
            copyable_footprints(&dst.desc, first, count, 0).ok_or(E_INVALIDARG)?;

        let mut needed_bytes = 0usize;
        for lay in &layouts {
            let sub_bytes = (lay.row_size as usize)
                .checked_mul(lay.num_rows as usize)
                .and_then(|v| v.checked_mul(lay.footprint.depth.max(1) as usize))
                .ok_or(E_INVALIDARG)?;
            needed_bytes = needed_bytes.checked_add(sub_bytes).ok_or(E_INVALIDARG)?;
        }
        if data.len() < needed_bytes {
            return Err(E_INVALIDARG);
        }

        let padded_end = layouts
            .last()
            .and_then(|l| {
                let slice_sz = (l.footprint.row_pitch as u64)
                    .checked_mul(l.num_rows as u64)?
                    .checked_mul(l.footprint.depth.max(1) as u64)?;
                l.footprint.offset.checked_add(slice_sz)
            })
            .unwrap_or(total_aligned)
            .max(total_aligned);

        let up = Self::alloc_upload_buffer(padded_end, "dstorage multi-subresource upload")?;
        let cpu = up.cpu_ptr().ok_or(E_FAIL)?;

        let mut src_cursor = 0usize;
        for lay in &layouts {
            let row_bytes = lay.row_size as usize;
            let num_rows = lay.num_rows as usize;
            let depth = lay.footprint.depth.max(1) as usize;
            let dst_row_pitch = lay.footprint.row_pitch as usize;
            let dst_slice_pitch = dst_row_pitch * num_rows;
            let base_off = lay.footprint.offset as usize;
            for z in 0..depth {
                for y in 0..num_rows {
                    let doff = base_off + z * dst_slice_pitch + y * dst_row_pitch;
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            data.as_ptr().add(src_cursor),
                            cpu.add(doff),
                            row_bytes,
                        );
                    }
                    src_cursor += row_bytes;
                }
            }
        }

        self.ensure_open()?;
        self.cl.barrier();
        for (i, lay) in layouts.iter().enumerate() {
            let sub = first + i as u32;
            self.cl
                .copy_texture_region(
                    CopyLoc::Subresource(dst, sub),
                    [0, 0, 0],
                    CopyLoc::Footprint(&up, lay.footprint),
                    None,
                )
                .map_err(|_| E_FAIL)?;
        }
        self.cl.barrier();
        self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
        self.staging.push(up);
        self.maybe_flush_staging()
    }

    fn upload_tiles(
        &mut self,
        dst: &Arc<MtlResource>,
        coord: TileCoord,
        region: TileRegionSize,
        data: &[u8],
    ) -> Result<(), i32> {
        // Tiled buffers: linear 64 KiB tiles along X on subresource 0.
        if dst.desc.is_buffer() {
            if coord.y != 0 || coord.z != 0 || coord.subresource != 0 {
                return Err(E_INVALIDARG);
            }
            let num_tiles = if region.use_box {
                if region.num_tiles == 0
                    && region.width == 0
                    && region.height == 0
                    && region.depth == 0
                {
                    return Ok(());
                }
                if region.width == 0 || region.height != 1 || region.depth != 1 {
                    return Err(E_INVALIDARG);
                }
                if region.num_tiles != 0 && region.num_tiles != region.width {
                    return Err(E_INVALIDARG);
                }
                region.width
            } else {
                region.num_tiles
            };
            if num_tiles == 0 {
                return Ok(());
            }
            let offset = (coord.x as u64)
                .checked_mul(D3D12_TILE_SIZE_BYTES as u64)
                .ok_or(E_INVALIDARG)?;
            let total_bytes = (num_tiles as u64)
                .checked_mul(D3D12_TILE_SIZE_BYTES as u64)
                .ok_or(E_INVALIDARG)?;
            if offset >= dst.desc.Width {
                return Err(E_INVALIDARG);
            }
            let copy_len = total_bytes.min(dst.desc.Width - offset) as usize;
            if data.len() < copy_len {
                return Err(E_INVALIDARG);
            }
            return self.upload_buffer(dst, offset, &data[..copy_len]);
        }

        if coord.subresource >= dst.desc.subresource_count() {
            return Err(E_INVALIDARG);
        }
        let (mip, _slice, plane) = split_subresource(dst, coord.subresource);
        let mw = (dst.desc.Width >> mip).max(1) as u32;
        let mh = (dst.desc.Height >> mip).max(1);
        let md = if dst.desc.Dimension == dim::TEXTURE3D {
            (dst.desc.depth() >> mip).max(1)
        } else {
            1
        };
        let fi = d3d12_metal::format::info(dst.desc.Format);
        if fi.bytes == 0 {
            return Err(E_INVALIDARG);
        }
        let bw = fi.block_w.max(1);
        let bh = fi.block_h.max(1);
        let bpe = match (dst.desc.Format, plane) {
            (44..=47, 0) | (19..=22, 0) => 4usize,
            (44..=47, _) | (19..=22, _) => 1usize,
            _ => fi.bytes as usize,
        };
        let (tile_w, tile_h, tile_d) = standard_tile_shape(&dst.desc, plane).ok_or(E_INVALIDARG)?;
        let sub_tiles_w = mw.div_ceil(tile_w);
        let sub_tiles_h = mh.div_ceil(tile_h);
        let sub_tiles_d = md.div_ceil(tile_d);
        if coord.x >= sub_tiles_w || coord.y >= sub_tiles_h || coord.z >= sub_tiles_d {
            return Err(E_INVALIDARG);
        }

        // Resolve tile coordinate list in D3D12 CopyTiles traversal order (X, then Y, then Z).
        let mut tiles: Vec<(u32, u32, u32)> = Vec::new();
        if region.use_box {
            if region.num_tiles == 0
                && region.width == 0
                && region.height == 0
                && region.depth == 0
            {
                return Ok(());
            }
            let rw = region.width;
            let rh = region.height as u32;
            let rd_dim = region.depth as u32;
            if rw == 0 || rh == 0 || rd_dim == 0 {
                return Err(E_INVALIDARG);
            }
            let box_tiles = rw
                .checked_mul(rh)
                .and_then(|v| v.checked_mul(rd_dim))
                .ok_or(E_INVALIDARG)?;
            if region.num_tiles != 0 && region.num_tiles != box_tiles {
                return Err(E_INVALIDARG);
            }
            if coord.x.checked_add(rw).is_none_or(|e| e > sub_tiles_w)
                || coord.y.checked_add(rh).is_none_or(|e| e > sub_tiles_h)
                || coord.z.checked_add(rd_dim).is_none_or(|e| e > sub_tiles_d)
            {
                return Err(E_INVALIDARG);
            }
            tiles.reserve(box_tiles as usize);
            for dz in 0..rd_dim {
                for dy in 0..rh {
                    for dx in 0..rw {
                        tiles.push((coord.x + dx, coord.y + dy, coord.z + dz));
                    }
                }
            }
        } else {
            if region.num_tiles == 0 {
                return Ok(());
            }
            tiles.reserve(region.num_tiles as usize);
            let (mut tx, mut ty, mut tz) = (coord.x, coord.y, coord.z);
            for _ in 0..region.num_tiles {
                if tz >= sub_tiles_d {
                    return Err(E_INVALIDARG);
                }
                tiles.push((tx, ty, tz));
                tx += 1;
                if tx >= sub_tiles_w {
                    tx = 0;
                    ty += 1;
                    if ty >= sub_tiles_h {
                        ty = 0;
                        tz += 1;
                    }
                }
            }
        }

        let needed_bytes = tiles
            .len()
            .checked_mul(D3D12_TILE_SIZE_BYTES)
            .ok_or(E_INVALIDARG)?;
        if data.len() < needed_bytes {
            return Err(E_INVALIDARG);
        }

        let tile_row_bytes = (tile_w / bw) as usize * bpe;
        let tile_rows = (tile_h / bh) as usize;
        let tile_slice_bytes = tile_row_bytes * tile_rows;
        let mw_aligned = mw.next_multiple_of(bw);
        let mh_aligned = mh.next_multiple_of(bh);
        let copy_fmt = plane_format(dst.desc.Format, plane);

        struct TileCopyPlan {
            tile_idx: usize,
            dst_box: D3D12Box,
            copy_row_bytes: usize,
            copy_rows: usize,
            dst_row_pitch: usize,
            dst_slice_pitch: usize,
            stage_off: usize,
        }

        let mut cursor = 0usize;
        while cursor < tiles.len() {
            let mut plans: Vec<TileCopyPlan> = Vec::new();
            let mut stage_total = 0usize;

            while cursor < tiles.len() {
                let (tx, ty, tz) = tiles[cursor];
                let left = tx * tile_w;
                let top = ty * tile_h;
                let front = tz * tile_d;
                let right = (left + tile_w).min(mw_aligned);
                let bottom = (top + tile_h).min(mh_aligned);
                let back = (front + tile_d).min(md);
                let w = right - left;
                let h = bottom - top;
                let d = back - front;
                let copy_row_bytes = (w.div_ceil(bw) as usize)
                    .checked_mul(bpe)
                    .ok_or(E_INVALIDARG)?;
                let copy_rows = h.div_ceil(bh) as usize;
                let dst_row_pitch = copy_row_bytes.next_multiple_of(256).max(256);
                let dst_slice_pitch = dst_row_pitch.checked_mul(copy_rows).ok_or(E_OUTOFMEMORY)?;
                let tile_staged = dst_slice_pitch
                    .checked_mul(d as usize)
                    .ok_or(E_OUTOFMEMORY)?;
                let stage_off = stage_total.next_multiple_of(512);
                let next_total = stage_off.checked_add(tile_staged).ok_or(E_OUTOFMEMORY)?;
                if !plans.is_empty() && next_total as u64 > MAX_STAGED_UPLOAD_BYTES {
                    break;
                }
                plans.push(TileCopyPlan {
                    tile_idx: cursor,
                    dst_box: D3D12Box {
                        left,
                        top,
                        front,
                        right,
                        bottom,
                        back,
                    },
                    copy_row_bytes,
                    copy_rows,
                    dst_row_pitch,
                    dst_slice_pitch,
                    stage_off,
                });
                stage_total = next_total;
                cursor += 1;
            }

            let up = Self::alloc_upload_buffer(stage_total as u64, "dstorage tiles upload")?;
            let cpu = up.cpu_ptr().ok_or(E_FAIL)?;

            for plan in &plans {
                let tile_src = &data[plan.tile_idx * D3D12_TILE_SIZE_BYTES
                    ..(plan.tile_idx + 1) * D3D12_TILE_SIZE_BYTES];
                let depth = (plan.dst_box.back - plan.dst_box.front) as usize;
                for z in 0..depth {
                    for y in 0..plan.copy_rows {
                        let soff = z * tile_slice_bytes + y * tile_row_bytes;
                        let doff = plan.stage_off + z * plan.dst_slice_pitch + y * plan.dst_row_pitch;
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                tile_src.as_ptr().add(soff),
                                cpu.add(doff),
                                plan.copy_row_bytes,
                            );
                        }
                    }
                }
            }

            self.ensure_open()?;
            self.cl.barrier();
            for plan in &plans {
                let b = plan.dst_box;
                let fp = PlacedFootprint {
                    offset: plan.stage_off as u64,
                    format: copy_fmt,
                    width: b.right - b.left,
                    height: b.bottom - b.top,
                    depth: b.back - b.front,
                    row_pitch: plan.dst_row_pitch as u32,
                    _pad: 0,
                };
                self.cl
                    .copy_texture_region(
                        CopyLoc::Subresource(dst, coord.subresource),
                        [b.left, b.top, b.front],
                        CopyLoc::Footprint(&up, fp),
                        None,
                    )
                    .map_err(|_| E_FAIL)?;
            }
            self.cl.barrier();
            self.staged_bytes = self.staged_bytes.saturating_add(up.desc.Width);
            self.staging.push(up);
            self.maybe_flush_staging()?;
        }

        Ok(())
    }
}

impl Drop for CopyUploader {
    fn drop(&mut self) {
        if self.open {
            let _ = self.cl.close();
            self.open = false;
        }
        self.queue.wait_idle();
        self.staging.clear();
        self.staged_bytes = 0;
    }
}

// ============================================================================
// IDStorageQueue / IDStorageQueue1 / IDStorageQueue2
// ============================================================================

enum QueuedSource {
    Memory {
        ptr: usize,
        size: u32,
    },
    File {
        file: Option<Arc<DStorageFileState>>,
        offset: u64,
        size: u32,
    },
    Invalid,
}

enum QueuedDest {
    Memory {
        ptr: usize,
        size: u32,
    },
    Buffer {
        resource: Option<Arc<MtlResource>>,
        offset: u64,
        size: u32,
    },
    TextureRegion {
        resource: Option<Arc<MtlResource>>,
        subresource_index: u32,
        region: D3D12Box,
    },
    MultipleSubresources {
        resource: Option<Arc<MtlResource>>,
        first_subresource: u32,
    },
    MultipleSubresourcesRange {
        resource: Option<Arc<MtlResource>>,
        first_subresource: u32,
        num_subresources: u32,
    },
    Tiles {
        resource: Option<Arc<MtlResource>>,
        coord: TileCoord,
        region: TileRegionSize,
    },
    Invalid,
}

struct QueuedRequest {
    compression_format: u8,
    source: QueuedSource,
    dest: QueuedDest,
    uncompressed_size: u32,
    cancellation_tag: u64,
    request_name: Vec<u8>,
    filename: String,
    raw_request: [u8; std::mem::size_of::<DSTORAGE_REQUEST>()],
}

impl QueuedRequest {
    fn read_source_bytes(&self) -> Result<Vec<u8>, i32> {
        match &self.source {
            QueuedSource::Memory { ptr, size } => {
                let sz = *size as usize;
                if sz == 0 {
                    Ok(Vec::new())
                } else if *ptr == 0 {
                    Err(E_INVALIDARG)
                } else {
                    let slice = unsafe { std::slice::from_raw_parts(*ptr as *const u8, sz) };
                    Ok(slice.to_vec())
                }
            }
            QueuedSource::File { file, offset, size } => {
                let Some(ref f) = file else {
                    return Err(E_INVALIDARG);
                };
                f.read_exact_at(*offset, *size as usize)
            }
            QueuedSource::Invalid => Err(E_INVALIDARG),
        }
    }

    fn expected_uncompressed_size(&self) -> Result<usize, i32> {
        if self.uncompressed_size > 0 {
            return Ok(self.uncompressed_size as usize);
        }
        match &self.dest {
            QueuedDest::Memory { size, .. } | QueuedDest::Buffer { size, .. } => Ok(*size as usize),
            QueuedDest::Tiles { region, .. } => {
                let n = if region.use_box {
                    if region.num_tiles != 0 {
                        region.num_tiles
                    } else {
                        region
                            .width
                            .checked_mul(region.height as u32)
                            .and_then(|v| v.checked_mul(region.depth as u32))
                            .ok_or(E_INVALIDARG)?
                    }
                } else {
                    region.num_tiles
                };
                (n as usize)
                    .checked_mul(D3D12_TILE_SIZE_BYTES)
                    .ok_or(E_INVALIDARG)
            }
            _ => Ok(0),
        }
    }
}

struct OwnedD3D12Fence {
    ptr: usize,
    owned: bool,
}

impl Drop for OwnedD3D12Fence {
    fn drop(&mut self) {
        if self.owned && self.ptr != 0 {
            unsafe {
                Obj::release(self.ptr as *mut Obj<crate::d3d12::fence::Fence>);
            }
            self.ptr = 0;
        }
    }
}

enum QueueItem {
    Request(QueuedRequest),
    Status {
        array_ptr: usize,
        array: Option<Arc<StatusArrayState>>,
        index: u32,
        valid_slot: bool,
    },
    Signal {
        fence: OwnedD3D12Fence,
        value: u64,
    },
    Event {
        handle: usize,
    },
}

enum CustomDecompCompletion {
    Pending,
    Done {
        hresult: i32,
        decompressed: Vec<u8>,
    },
}

struct PendingCustomRequest {
    req: QueuedRequest,
    completion: Arc<Mutex<CustomDecompCompletion>>,
}

enum SubmittedEntry {
    Item(QueueItem),
    Dispatching,
    InFlightCustom(PendingCustomRequest),
}

enum FrontAction {
    Item(QueueItem),
    CustomDone(QueuedRequest, i32, Vec<u8>),
}

enum FirstFailureDetail {
    Request {
        filename: String,
        request_name: Vec<u8>,
        raw_request: [u8; std::mem::size_of::<DSTORAGE_REQUEST>()],
    },
    Status {
        status_array: usize,
        index: u32,
    },
    Signal {
        fence: usize,
        value: u64,
    },
    Event {
        handle: usize,
    },
}

struct FirstFailureRecord {
    hresult: i32,
    command_type: i32,
    detail: FirstFailureDetail,
}

struct QueueInner {
    pending: VecDeque<QueueItem>,
    submitted: VecDeque<SubmittedEntry>,
    in_flight: usize,
    closed: bool,
    failure_count: u32,
    first_failure: Option<FirstFailureRecord>,
    batch_first_error: Option<i32>,
}

struct QueueState {
    source_type: u64,
    capacity: u16,
    auto_submit_threshold: u16,
    priority: i8,
    name_buf: Option<Box<[u8]>>,
    device: usize,
    error_event: Arc<Event>,
    inner: Mutex<QueueInner>,
    exec_lock: Mutex<()>,
    uploader: Mutex<Option<CopyUploader>>,
}

struct QueueObj {
    state: Arc<QueueState>,
}

impl QueueState {
    fn desc(&self) -> DSTORAGE_QUEUE_DESC {
        DSTORAGE_QUEUE_DESC {
            source_type: self.source_type,
            capacity: self.capacity,
            priority: self.priority,
            _pad0: [0; 5],
            name: self
                .name_buf
                .as_ref()
                .map_or(std::ptr::null(), |b| b.as_ptr()),
            device: self.device,
        }
    }

    fn record_error(&self, hresult: i32, command_type: i32, detail: FirstFailureDetail) {
        if DEBUG_FLAGS.load(Ordering::Relaxed) & DSTORAGE_DEBUG_SHOW_ERRORS != 0 {
            log::warn!(
                target: "dstorage",
                "DirectStorage queue error: cmd_type={command_type} hr={hresult:#010x}"
            );
        }
        let mut st = self.inner.lock().unwrap();
        st.failure_count = st.failure_count.saturating_add(1);
        if st.first_failure.is_none() {
            st.first_failure = Some(FirstFailureRecord {
                hresult,
                command_type,
                detail,
            });
        }
        if st.batch_first_error.is_none() {
            st.batch_first_error = Some(hresult);
        }
        drop(st);
        self.error_event.set();
    }

    fn flush_gpu_uploads(&self) -> Result<(), i32> {
        let mut up_guard = self.uploader.lock().unwrap();
        if let Some(ref mut uploader) = *up_guard {
            uploader.flush()?;
        }
        Ok(())
    }

    fn with_uploader<F>(&self, f: F) -> Result<(), i32>
    where
        F: FnOnce(&mut CopyUploader) -> Result<(), i32>,
    {
        let mut up_guard = self.uploader.lock().unwrap();
        if up_guard.is_none() {
            *up_guard = Some(CopyUploader::new()?);
        }
        let uploader = up_guard.as_mut().unwrap();
        f(uploader)
    }

    fn deliver_payload(&self, req: &QueuedRequest, payload: &[u8]) -> Result<(), i32> {
        match &req.dest {
            QueuedDest::Memory { ptr, size } => {
                let dst_size = *size as usize;
                if payload.len() > dst_size {
                    return Err(E_INVALIDARG);
                }
                if !payload.is_empty() {
                    if *ptr == 0 {
                        return Err(E_INVALIDARG);
                    }
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            payload.as_ptr(),
                            *ptr as *mut u8,
                            payload.len(),
                        );
                    }
                }
                Ok(())
            }
            QueuedDest::Buffer {
                resource,
                offset,
                size,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                if !mtl_res.desc.is_buffer() {
                    return Err(E_INVALIDARG);
                }
                let copy_len = *size as usize;
                let end = offset.checked_add(copy_len as u64).ok_or(E_INVALIDARG)?;
                if end > mtl_res.desc.Width || payload.len() < copy_len {
                    return Err(E_INVALIDARG);
                }
                if copy_len == 0 {
                    return Ok(());
                }
                if let Some(cpu) = mtl_res.cpu_ptr() {
                    self.flush_gpu_uploads()?;
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            payload.as_ptr(),
                            cpu.add(*offset as usize),
                            copy_len,
                        );
                    }
                    Ok(())
                } else {
                    self.with_uploader(|up| up.upload_buffer(mtl_res, *offset, &payload[..copy_len]))
                }
            }
            QueuedDest::TextureRegion {
                resource,
                subresource_index,
                region,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                self.with_uploader(|up| {
                    up.upload_texture_region(mtl_res, *subresource_index, *region, payload)
                })
            }
            QueuedDest::MultipleSubresources {
                resource,
                first_subresource,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                if mtl_res.desc.is_buffer() {
                    return Err(E_INVALIDARG);
                }
                let total = mtl_res.desc.subresource_count();
                if *first_subresource >= total {
                    return Err(E_INVALIDARG);
                }
                let count = total - *first_subresource;
                self.with_uploader(|up| {
                    up.upload_multiple_subresources(mtl_res, *first_subresource, count, payload)
                })
            }
            QueuedDest::MultipleSubresourcesRange {
                resource,
                first_subresource,
                num_subresources,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                self.with_uploader(|up| {
                    up.upload_multiple_subresources(
                        mtl_res,
                        *first_subresource,
                        *num_subresources,
                        payload,
                    )
                })
            }
            QueuedDest::Tiles {
                resource,
                coord,
                region,
            } => {
                let Some(ref mtl_res) = resource else {
                    return Err(E_INVALIDARG);
                };
                self.with_uploader(|up| up.upload_tiles(mtl_res, *coord, *region, payload))
            }
            QueuedDest::Invalid => Err(E_INVALIDARG),
        }
    }

    fn execute_non_custom_request(&self, req: &QueuedRequest) -> Result<(), i32> {
        // 1. Read source bytes
        let raw_bytes = req.read_source_bytes()?;

        // 2. Decompress payload (never silently memcpy on decompression failure!)
        let payload: Vec<u8> = match req.compression_format {
            DSTORAGE_COMPRESSION_FORMAT_NONE => raw_bytes,
            DSTORAGE_COMPRESSION_FORMAT_GDEFLATE => {
                let decompressed = decompress_gdeflate_to_vec(&raw_bytes)?;
                if req.uncompressed_size > 0
                    && decompressed.len() != req.uncompressed_size as usize
                {
                    return Err(E_DSTORAGE_DECOMPRESSION_ERROR);
                }
                decompressed
            }
            _ => return Err(E_INVALIDARG),
        };

        // 3. Deliver payload to destination
        self.deliver_payload(req, &payload)
    }

    fn execute_item(&self, item: QueueItem) {
        match &item {
            QueueItem::Request(req) => {
                if let Err(hr) = self.execute_non_custom_request(req) {
                    self.record_error(
                        hr,
                        DSTORAGE_COMMAND_TYPE_REQUEST,
                        FirstFailureDetail::Request {
                            filename: req.filename.clone(),
                            request_name: req.request_name.clone(),
                            raw_request: req.raw_request,
                        },
                    );
                }
            }
            QueueItem::Status {
                array_ptr,
                array,
                index,
                valid_slot,
            } => {
                let array_ptr = *array_ptr;
                let index = *index;
                let valid_slot = *valid_slot;
                let flush_err = self.flush_gpu_uploads().err();
                let batch_hr = {
                    let mut st = self.inner.lock().unwrap();
                    let prev = st.batch_first_error.take();
                    prev.or(flush_err).unwrap_or(S_OK)
                };
                if let Some(err_hr) = flush_err {
                    self.record_error(
                        err_hr,
                        DSTORAGE_COMMAND_TYPE_STATUS,
                        FirstFailureDetail::Status {
                            status_array: array_ptr,
                            index,
                        },
                    );
                }
                if valid_slot {
                    if let Some(sa) = array {
                        if !sa.complete(index, batch_hr) {
                            self.record_error(
                                E_INVALIDARG,
                                DSTORAGE_COMMAND_TYPE_STATUS,
                                FirstFailureDetail::Status {
                                    status_array: array_ptr,
                                    index,
                                },
                            );
                        }
                    }
                } else {
                    self.record_error(
                        E_INVALIDARG,
                        DSTORAGE_COMMAND_TYPE_STATUS,
                        FirstFailureDetail::Status {
                            status_array: array_ptr,
                            index,
                        },
                    );
                }
            }
            QueueItem::Signal { fence, value } => {
                let fence_ptr = fence.ptr;
                let value = *value;
                if let Err(hr) = self.flush_gpu_uploads() {
                    self.record_error(
                        hr,
                        DSTORAGE_COMMAND_TYPE_SIGNAL,
                        FirstFailureDetail::Signal {
                            fence: fence_ptr,
                            value,
                        },
                    );
                }
                let sig_hr = unsafe { signal_d3d12_fence(fence_ptr, value) };
                if sig_hr != S_OK {
                    self.record_error(
                        sig_hr,
                        DSTORAGE_COMMAND_TYPE_SIGNAL,
                        FirstFailureDetail::Signal {
                            fence: fence_ptr,
                            value,
                        },
                    );
                }
            }
            QueueItem::Event { handle } => {
                let handle = *handle;
                if let Err(hr) = self.flush_gpu_uploads() {
                    self.record_error(
                        hr,
                        DSTORAGE_COMMAND_TYPE_EVENT,
                        FirstFailureDetail::Event { handle },
                    );
                }
                if handle == 0 {
                    self.record_error(
                        E_HANDLE,
                        DSTORAGE_COMMAND_TYPE_EVENT,
                        FirstFailureDetail::Event { handle },
                    );
                } else {
                    let ok = unsafe { crate::kernel32::threads::SetEvent(handle as HANDLE) };
                    if ok == FALSE {
                        self.record_error(
                            E_HANDLE,
                            DSTORAGE_COMMAND_TYPE_EVENT,
                            FirstFailureDetail::Event { handle },
                        );
                    }
                }
            }
        }
    }

    fn drain_submitted(self: &Arc<Self>) {
        let _exec = self.exec_lock.lock().unwrap();
        let mut did_work = false;

        loop {
            // Phase 1: dispatch any contiguous leading custom-decompression requests
            // so caller worker threads can pull an entire batch via GetRequests/GetRequests1.
            loop {
                let next_custom = {
                    let mut st = self.inner.lock().unwrap();
                    if st.closed {
                        return;
                    }
                    let mut idx = 0usize;
                    while idx < st.submitted.len() {
                        match &st.submitted[idx] {
                            SubmittedEntry::InFlightCustom(_) => idx += 1,
                            SubmittedEntry::Item(QueueItem::Request(req))
                                if req.compression_format >= DSTORAGE_CUSTOM_COMPRESSION_0 =>
                            {
                                break;
                            }
                            _ => {
                                idx = usize::MAX;
                                break;
                            }
                        }
                    }
                    if idx < st.submitted.len() {
                        let slot = std::mem::replace(
                            &mut st.submitted[idx],
                            SubmittedEntry::Dispatching,
                        );
                        match slot {
                            SubmittedEntry::Item(QueueItem::Request(req)) => Some((idx, req)),
                            _ => unreachable!(),
                        }
                    } else {
                        None
                    }
                };
                let Some((idx, req)) = next_custom else {
                    break;
                };

                let prep = req.read_source_bytes().and_then(|raw| {
                    let dst_size = req.expected_uncompressed_size()?;
                    Ok((raw, dst_size))
                });

                let mut st = self.inner.lock().unwrap();
                if st.closed {
                    return;
                }
                match prep {
                    Ok((raw_bytes, dst_size)) => {
                        let completion = Arc::new(Mutex::new(CustomDecompCompletion::Pending));
                        custom_decomp_state().enqueue(
                            req.compression_format,
                            raw_bytes,
                            dst_size,
                            Arc::clone(self),
                            Arc::clone(&completion),
                        );
                        st.submitted[idx] = SubmittedEntry::InFlightCustom(PendingCustomRequest {
                            req,
                            completion,
                        });
                    }
                    Err(err_hr) => {
                        let completion = Arc::new(Mutex::new(CustomDecompCompletion::Done {
                            hresult: err_hr,
                            decompressed: Vec::new(),
                        }));
                        st.submitted[idx] = SubmittedEntry::InFlightCustom(PendingCustomRequest {
                            req,
                            completion,
                        });
                    }
                }
            }

            // Phase 2: pop and execute the front entry if ready (FIFO order).
            let front_action = {
                let mut st = self.inner.lock().unwrap();
                if st.closed {
                    return;
                }
                match st.submitted.front() {
                    None => break,
                    Some(SubmittedEntry::InFlightCustom(pending)) => {
                        let mut guard = pending.completion.lock().unwrap();
                        if matches!(*guard, CustomDecompCompletion::Pending) {
                            break;
                        }
                        let done = std::mem::replace(&mut *guard, CustomDecompCompletion::Pending);
                        drop(guard);
                        let Some(SubmittedEntry::InFlightCustom(p)) = st.submitted.pop_front()
                        else {
                            unreachable!();
                        };
                        match done {
                            CustomDecompCompletion::Done {
                                hresult,
                                decompressed,
                            } => FrontAction::CustomDone(p.req, hresult, decompressed),
                            CustomDecompCompletion::Pending => unreachable!(),
                        }
                    }
                    Some(SubmittedEntry::Item(_)) => {
                        let Some(SubmittedEntry::Item(item)) = st.submitted.pop_front() else {
                            unreachable!();
                        };
                        FrontAction::Item(item)
                    }
                    Some(SubmittedEntry::Dispatching) => unreachable!(),
                }
            };

            did_work = true;
            match front_action {
                FrontAction::CustomDone(req, hr, decompressed) => {
                    if hr < 0 {
                        self.record_error(
                            hr,
                            DSTORAGE_COMMAND_TYPE_REQUEST,
                            FirstFailureDetail::Request {
                                filename: req.filename.clone(),
                                request_name: req.request_name.clone(),
                                raw_request: req.raw_request,
                            },
                        );
                    } else if let Err(dest_hr) = self.deliver_payload(&req, &decompressed) {
                        self.record_error(
                            dest_hr,
                            DSTORAGE_COMMAND_TYPE_REQUEST,
                            FirstFailureDetail::Request {
                                filename: req.filename.clone(),
                                request_name: req.request_name.clone(),
                                raw_request: req.raw_request,
                            },
                        );
                    }
                }
                FrontAction::Item(item) => {
                    self.execute_item(item);
                }
            }

            let mut st = self.inner.lock().unwrap();
            st.in_flight = st.in_flight.saturating_sub(1);
        }

        // Ensure any trailing GPU uploads committed in this pass are landed before returning.
        if did_work {
            if let Err(hr) = self.flush_gpu_uploads() {
                log::warn!(target: "dstorage", "DirectStorage trailing GPU copy flush failed: {hr:#010x}");
            }
        }
    }

    fn execute_submit(self: &Arc<Self>) {
        let moved = {
            let mut st = self.inner.lock().unwrap();
            if st.closed || st.pending.is_empty() {
                false
            } else {
                let count = st.pending.len();
                let drained: Vec<QueueItem> = st.pending.drain(..).collect();
                for item in drained {
                    st.submitted.push_back(SubmittedEntry::Item(item));
                }
                st.in_flight = st.in_flight.saturating_add(count);
                true
            }
        };
        if moved {
            self.drain_submitted();
        }
    }

    fn enqueue_item(self: &Arc<Self>, item: QueueItem) {
        let should_auto_submit = {
            let mut st = self.inner.lock().unwrap();
            if st.closed {
                return;
            }
            st.pending.push_back(item);
            st.pending.len() >= self.auto_submit_threshold as usize
        };
        if should_auto_submit {
            self.execute_submit();
        }
    }
}

unsafe fn signal_d3d12_fence(fence_ptr: usize, value: u64) -> i32 {
    if fence_ptr == 0 {
        return E_INVALIDARG;
    }
    if let Some(fence) = crate::d3d12::fence::from_ptr(fence_ptr as *mut c_void) {
        fence.f.signal_cpu(value);
        return S_OK;
    }
    // Fallback for proxy / foreign COM fence wrappers: ID3D12Fence::Signal is vtable slot 10.
    com_call_slot(fence_ptr as *mut c_void, 10, value, 0, 0)
}

unsafe extern "win64" fn ds_queue_qi(
    this: *mut ComObj<QueueObj>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    qi_helper(this, riid, ppv, QUEUE_IIDS)
}

unsafe extern "win64" fn ds_queue_enqueue_request(this: *mut c_void, req_ptr: usize) {
    if req_ptr == 0 {
        return;
    }
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return;
    };
    let q = &obj.inner.state;
    let raw_request = rd(req_ptr as *const [u8; std::mem::size_of::<DSTORAGE_REQUEST>()]);
    let req = rd(req_ptr as *const DSTORAGE_REQUEST);

    let compression_format = req.options.compression_format;
    let source_type = req.options.source_type();
    let dest_type = req.options.destination_type();

    if source_type as u64 != q.source_type {
        log::debug!(
            target: "dstorage",
            "EnqueueRequest source_type ({source_type}) differs from queue source_type ({})",
            q.source_type
        );
    }

    let request_name = read_ansi_bytes_z(req.name, DSTORAGE_REQUEST_MAX_NAME - 1);
    let mut filename = String::new();

    let source = match source_type {
        DSTORAGE_REQUEST_SOURCE_MEMORY => QueuedSource::Memory {
            ptr: le_u64(&req.source, 0).unwrap_or(0) as usize,
            size: le_u32(&req.source, 8).unwrap_or(0),
        },
        DSTORAGE_REQUEST_SOURCE_FILE => {
            let file_com = le_u64(&req.source, 0).unwrap_or(0) as usize;
            let offset = le_u64(&req.source, 8).unwrap_or(0);
            let size = le_u32(&req.source, 16).unwrap_or(0);
            let file_arc = com_from_iface::<FileObj>(file_com as *mut c_void, file_vtable())
                .map(|f| f.inner.state.clone());
            if let Some(ref f) = file_arc {
                filename = f.guest_path.clone();
            }
            QueuedSource::File {
                file: file_arc,
                offset,
                size,
            }
        }
        _ => QueuedSource::Invalid,
    };

    let dest = match dest_type {
        DSTORAGE_REQUEST_DESTINATION_MEMORY => QueuedDest::Memory {
            ptr: le_u64(&req.destination, 0).unwrap_or(0) as usize,
            size: le_u32(&req.destination, 8).unwrap_or(0),
        },
        DSTORAGE_REQUEST_DESTINATION_BUFFER => {
            let res_ptr = le_u64(&req.destination, 0).unwrap_or(0) as usize;
            let offset = le_u64(&req.destination, 8).unwrap_or(0);
            let size = le_u32(&req.destination, 16).unwrap_or(0);
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::Buffer {
                resource,
                offset,
                size,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_TEXTURE_REGION => {
            let res_ptr = le_u64(&req.destination, 0).unwrap_or(0) as usize;
            let subresource_index = le_u32(&req.destination, 8).unwrap_or(0);
            let region = D3D12Box {
                left: le_u32(&req.destination, 12).unwrap_or(0),
                top: le_u32(&req.destination, 16).unwrap_or(0),
                front: le_u32(&req.destination, 20).unwrap_or(0),
                right: le_u32(&req.destination, 24).unwrap_or(0),
                bottom: le_u32(&req.destination, 28).unwrap_or(0),
                back: le_u32(&req.destination, 32).unwrap_or(0),
            };
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::TextureRegion {
                resource,
                subresource_index,
                region,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES => {
            let res_ptr = le_u64(&req.destination, 0).unwrap_or(0) as usize;
            let first_subresource = le_u32(&req.destination, 8).unwrap_or(0);
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::MultipleSubresources {
                resource,
                first_subresource,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_TILES => {
            let res_ptr = le_u64(&req.destination, 0).unwrap_or(0) as usize;
            let coord = TileCoord {
                x: le_u32(&req.destination, 8).unwrap_or(0),
                y: le_u32(&req.destination, 12).unwrap_or(0),
                z: le_u32(&req.destination, 16).unwrap_or(0),
                subresource: le_u32(&req.destination, 20).unwrap_or(0),
            };
            let region = TileRegionSize {
                num_tiles: le_u32(&req.destination, 24).unwrap_or(0),
                use_box: le_u32(&req.destination, 28).unwrap_or(0) != 0,
                width: le_u32(&req.destination, 32).unwrap_or(0),
                height: le_u16(&req.destination, 36).unwrap_or(0),
                depth: le_u16(&req.destination, 38).unwrap_or(0),
            };
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::Tiles {
                resource,
                coord,
                region,
            }
        }
        DSTORAGE_REQUEST_DESTINATION_MULTIPLE_SUBRESOURCES_RANGE => {
            let res_ptr = le_u64(&req.destination, 0).unwrap_or(0) as usize;
            let first_subresource = le_u32(&req.destination, 8).unwrap_or(0);
            let num_subresources = le_u32(&req.destination, 12).unwrap_or(0);
            let resource = crate::d3d12::resource::from_ptr(res_ptr as *mut c_void)
                .map(|r| r.r.clone());
            QueuedDest::MultipleSubresourcesRange {
                resource,
                first_subresource,
                num_subresources,
            }
        }
        _ => QueuedDest::Invalid,
    };

    q.enqueue_item(QueueItem::Request(QueuedRequest {
        compression_format,
        source,
        dest,
        uncompressed_size: req.uncompressed_size,
        cancellation_tag: req.cancellation_tag,
        request_name,
        filename,
        raw_request,
    }));
}

unsafe extern "win64" fn ds_queue_enqueue_status(
    this: *mut c_void,
    status_array: usize,
    index: u32,
) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return;
    };
    let array = com_from_iface::<StatusArrayObj>(status_array as *mut c_void, status_vtable())
        .map(|s| s.inner.state.clone());
    let valid_slot = array
        .as_ref()
        .is_some_and(|sa| sa.mark_pending(index));
    obj.inner.state.enqueue_item(QueueItem::Status {
        array_ptr: status_array,
        array,
        index,
        valid_slot,
    });
}

unsafe extern "win64" fn ds_queue_enqueue_signal(this: *mut c_void, fence_ptr: usize, value: u64) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return;
    };
    let owned_d3d12_fence = if fence_ptr != 0 {
        if let Some(f) = Obj::<crate::d3d12::fence::Fence>::from_iface(fence_ptr as *mut c_void) {
            Obj::add_ref(f);
            true
        } else {
            false
        }
    } else {
        false
    };
    obj.inner.state.enqueue_item(QueueItem::Signal {
        fence: OwnedD3D12Fence {
            ptr: fence_ptr,
            owned: owned_d3d12_fence,
        },
        value,
    });
}

unsafe extern "win64" fn ds_queue_submit(this: *mut c_void) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return;
    };
    obj.inner.state.execute_submit();
}

unsafe extern "win64" fn ds_queue_cancel_requests(this: *mut c_void, mask: u64, value: u64) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return;
    };
    let mut st = obj.inner.state.inner.lock().unwrap();
    st.pending.retain(|item| match item {
        QueueItem::Request(req) => (req.cancellation_tag & mask) != value,
        _ => true,
    });
}

unsafe extern "win64" fn ds_queue_close(this: *mut c_void) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return;
    };
    let q = &obj.inner.state;
    {
        let mut st = q.inner.lock().unwrap();
        st.closed = true;
        st.pending.clear();
        st.submitted.clear();
        st.in_flight = 0;
    }
    custom_decomp_state().cancel_pending_for_queue(q);
    let _exec = q.exec_lock.lock().unwrap();
    *q.uploader.lock().unwrap() = None;
}

unsafe extern "win64" fn ds_queue_get_error_event(this: *mut c_void) -> HANDLE {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return std::ptr::null_mut();
    };
    crate::handles::insert(obj.inner.state.error_event.clone())
}

unsafe extern "win64" fn ds_queue_retrieve_error_record(
    this: *mut c_void,
    record_out: *mut DSTORAGE_ERROR_RECORD,
) {
    if record_out.is_null() {
        return;
    }
    let mut rec: DSTORAGE_ERROR_RECORD = std::mem::zeroed();
    rec.first_failure.command_type = DSTORAGE_COMMAND_TYPE_NONE;
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        wr(record_out, rec);
        return;
    };
    let q = &obj.inner.state;

    let (failure_count, first_failure) = {
        let mut st = q.inner.lock().unwrap();
        let count = st.failure_count;
        st.failure_count = 0;
        let first = st.first_failure.take();
        (count, first)
    };
    q.error_event.reset();

    rec.failure_count = failure_count;
    if let Some(first) = first_failure {
        rec.first_failure.hresult = first.hresult;
        rec.first_failure.command_type = first.command_type;
        match first.detail {
            FirstFailureDetail::Request {
                filename,
                request_name,
                raw_request,
            } => {
                let mut req_params: DSTORAGE_ERROR_PARAMETERS_REQUEST = std::mem::zeroed();
                for (i, unit) in filename
                    .encode_utf16()
                    .take(DSTORAGE_MAX_PATH - 1)
                    .enumerate()
                {
                    req_params.filename[i] = unit;
                }
                let copy_name = request_name.len().min(DSTORAGE_REQUEST_MAX_NAME - 1);
                req_params.request_name[..copy_name].copy_from_slice(&request_name[..copy_name]);
                req_params.request = rd(raw_request.as_ptr() as *const DSTORAGE_REQUEST);
                rec.first_failure.params.request = req_params;
            }
            FirstFailureDetail::Status {
                status_array,
                index,
            } => {
                rec.first_failure.params.status = DSTORAGE_ERROR_PARAMETERS_STATUS {
                    status_array,
                    index,
                    _pad0: 0,
                };
            }
            FirstFailureDetail::Signal { fence, value } => {
                rec.first_failure.params.signal = DSTORAGE_ERROR_PARAMETERS_SIGNAL { fence, value };
            }
            FirstFailureDetail::Event { handle } => {
                rec.first_failure.params.event = DSTORAGE_ERROR_PARAMETERS_EVENT {
                    handle: handle as HANDLE,
                };
            }
        }
    }
    wr(record_out, rec);
}

unsafe extern "win64" fn ds_queue_query(this: *mut c_void, info_out: *mut DSTORAGE_QUEUE_INFO) {
    if info_out.is_null() {
        return;
    }
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        std::ptr::write_bytes(info_out as *mut u8, 0, std::mem::size_of::<DSTORAGE_QUEUE_INFO>());
        return;
    };
    let q = &obj.inner.state;
    let (used, pending_len) = {
        let st = q.inner.lock().unwrap();
        let p = st.pending.len();
        (p.saturating_add(st.in_flight), p)
    };
    let cap = q.capacity as usize;
    let empty_slots = cap.saturating_sub(used).min(u16::MAX as usize) as u16;
    let until_auto = (q.auto_submit_threshold as usize)
        .saturating_sub(pending_len)
        .min(empty_slots as usize) as u16;
    let info = DSTORAGE_QUEUE_INFO {
        desc: q.desc(),
        empty_slot_count: empty_slots,
        request_count_until_auto_submit: until_auto,
        _pad0: 0,
    };
    wr(info_out, info);
}

unsafe extern "win64" fn ds_queue_enqueue_set_event(this: *mut c_void, handle: HANDLE) {
    let Some(obj) = com_from_iface::<QueueObj>(this, queue_vtable()) else {
        return;
    };
    obj.inner.state.enqueue_item(QueueItem::Event {
        handle: handle as usize,
    });
}

extern "win64" fn ds_queue_get_compression_support(_this: *mut c_void, format: u8) -> u32 {
    match format {
        DSTORAGE_COMPRESSION_FORMAT_GDEFLATE => DSTORAGE_COMPRESSION_SUPPORT_CPU_FALLBACK,
        _ => DSTORAGE_COMPRESSION_SUPPORT_NONE,
    }
}

fn queue_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 14]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            ds_queue_qi as *const () as usize,
            com_add_ref::<QueueObj> as *const () as usize,
            com_release::<QueueObj> as *const () as usize,
            ds_queue_enqueue_request as *const () as usize,
            ds_queue_enqueue_status as *const () as usize,
            ds_queue_enqueue_signal as *const () as usize,
            ds_queue_submit as *const () as usize,
            ds_queue_cancel_requests as *const () as usize,
            ds_queue_close as *const () as usize,
            ds_queue_get_error_event as *const () as usize,
            ds_queue_retrieve_error_record as *const () as usize,
            ds_queue_query as *const () as usize,
            ds_queue_enqueue_set_event as *const () as usize,
            ds_queue_get_compression_support as *const () as usize,
        ]
    })
}

pub(super) fn new_queue_obj(desc: DSTORAGE_QUEUE_DESC, name_buf: Option<Box<[u8]>>) -> usize {
    let auto_submit_threshold = (desc.capacity / 2).max(64).min(desc.capacity);
    let state = Arc::new(QueueState {
        source_type: desc.source_type,
        capacity: desc.capacity,
        auto_submit_threshold,
        priority: desc.priority,
        name_buf,
        device: desc.device,
        error_event: Arc::new(Event::new(true, false, None)),
        inner: Mutex::new(QueueInner {
            pending: VecDeque::new(),
            submitted: VecDeque::new(),
            in_flight: 0,
            closed: false,
            failure_count: 0,
            first_failure: None,
            batch_first_error: None,
        }),
        exec_lock: Mutex::new(()),
        uploader: Mutex::new(None),
    });
    ComObj::new(queue_vtable(), QueueObj { state }) as usize
}

// ============================================================================
// IDStorageCustomDecompressionQueue / IDStorageCustomDecompressionQueue1
// Vtable order (dstorage.h):
//   0: QueryInterface, 1: AddRef, 2: Release,
//   3: GetEvent, 4: GetRequests, 5: SetRequestResults,
//   6: GetRequests1 (IDStorageCustomDecompressionQueue1)
// ============================================================================

/// Owned heap byte buffer whose raw address remains valid across moves / HashMap
/// rehashes (no `&mut` aliasing while caller worker threads write into `DstBuffer`).
struct RawHeapBuf {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

unsafe impl Send for RawHeapBuf {}

impl RawHeapBuf {
    fn from_vec(mut v: Vec<u8>) -> Self {
        let ptr = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();
        std::mem::forget(v);
        Self { ptr, len, cap }
    }

    fn into_vec(self) -> Vec<u8> {
        let v = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
        std::mem::forget(self);
        v
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn as_const_ptr(&self) -> *const c_void {
        if self.len == 0 {
            std::ptr::null()
        } else {
            self.ptr as *const c_void
        }
    }

    #[inline]
    fn as_mut_ptr(&self) -> *mut c_void {
        if self.len == 0 {
            std::ptr::null_mut()
        } else {
            self.ptr as *mut c_void
        }
    }
}

impl Drop for RawHeapBuf {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(self.ptr, self.len, self.cap));
        }
    }
}

struct CustomDecompJob {
    id: u64,
    compression_format: u8,
    src_buf: RawHeapBuf,
    dst_buf: RawHeapBuf,
    queue: Arc<QueueState>,
    completion: Arc<Mutex<CustomDecompCompletion>>,
}

struct CustomDecompInner {
    pending: VecDeque<CustomDecompJob>,
    in_flight: HashMap<u64, CustomDecompJob>,
}

struct CustomDecompQueueState {
    next_id: AtomicU64,
    event: Arc<Event>,
    inner: Mutex<CustomDecompInner>,
}

impl CustomDecompQueueState {
    fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            event: Arc::new(Event::new(true, false, None)),
            inner: Mutex::new(CustomDecompInner {
                pending: VecDeque::new(),
                in_flight: HashMap::new(),
            }),
        }
    }

    fn enqueue(
        &self,
        compression_format: u8,
        src_bytes: Vec<u8>,
        dst_size: usize,
        queue: Arc<QueueState>,
        completion: Arc<Mutex<CustomDecompCompletion>>,
    ) {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let job = CustomDecompJob {
            id,
            compression_format,
            src_buf: RawHeapBuf::from_vec(src_bytes),
            dst_buf: RawHeapBuf::from_vec(vec![0u8; dst_size]),
            queue,
            completion,
        };
        let mut st = self.inner.lock().unwrap();
        st.pending.push_back(job);
        self.event.set();
    }

    fn cancel_pending_for_queue(&self, queue: &Arc<QueueState>) {
        let mut st = self.inner.lock().unwrap();
        st.pending.retain(|job| !Arc::ptr_eq(&job.queue, queue));
        if st.pending.is_empty() {
            self.event.reset();
        }
    }

    unsafe fn get_requests(
        &self,
        flags: u32,
        max_requests: u32,
        requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
        num_requests: *mut u32,
    ) -> i32 {
        if num_requests.is_null() {
            return E_INVALIDARG;
        }
        wr(num_requests, 0);
        if max_requests > 0 && requests.is_null() {
            return E_INVALIDARG;
        }
        if flags == 0 || (flags & !DSTORAGE_GET_REQUEST_FLAG_SELECT_ALL) != 0 {
            return E_INVALIDARG;
        }
        if max_requests == 0 || (flags & DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM) == 0 {
            return S_OK;
        }

        let mut st = self.inner.lock().unwrap();
        let mut count = 0u32;
        while count < max_requests {
            let Some(job) = st.pending.pop_front() else {
                break;
            };
            let raw = DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST {
                id: job.id,
                compression_format: job.compression_format,
                reserved: [0; 3],
                flags: DSTORAGE_CUSTOM_DECOMPRESSION_FLAG_NONE,
                src_size: job.src_buf.len() as u64,
                src_buffer: job.src_buf.as_const_ptr(),
                dst_size: job.dst_buf.len() as u64,
                dst_buffer: job.dst_buf.as_mut_ptr(),
            };
            st.in_flight.insert(job.id, job);
            wr(requests.add(count as usize), raw);
            count += 1;
        }
        if st.pending.is_empty() {
            self.event.reset();
        }
        drop(st);
        wr(num_requests, count);
        S_OK
    }

    unsafe fn set_request_results(
        &self,
        num_results: u32,
        results: *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
    ) -> i32 {
        if num_results == 0 {
            return S_OK;
        }
        if results.is_null() {
            return E_INVALIDARG;
        }

        let mut parsed = Vec::with_capacity(num_results as usize);
        for i in 0..num_results as usize {
            parsed.push(rd(results.add(i)));
        }

        let mut affected_queues: Vec<Arc<QueueState>> = Vec::new();
        {
            let mut st = self.inner.lock().unwrap();
            // Validate all IDs exist and are distinct before mutating state.
            for (idx, r) in parsed.iter().enumerate() {
                if !st.in_flight.contains_key(&r.id)
                    || parsed[..idx].iter().any(|prev| prev.id == r.id)
                {
                    return E_INVALIDARG;
                }
            }
            for r in parsed {
                let job = st.in_flight.remove(&r.id).unwrap();
                let decompressed = if r.result >= 0 {
                    job.dst_buf.into_vec()
                } else {
                    Vec::new()
                };
                *job.completion.lock().unwrap() = CustomDecompCompletion::Done {
                    hresult: r.result,
                    decompressed,
                };
                if !affected_queues
                    .iter()
                    .any(|q| Arc::ptr_eq(q, &job.queue))
                {
                    affected_queues.push(job.queue);
                }
            }
        }

        // Resume each affected queue outside custom_queue's mutex to preserve lock order.
        for q in affected_queues {
            q.drain_submitted();
        }
        S_OK
    }
}

fn custom_decomp_state() -> &'static CustomDecompQueueState {
    static STATE: OnceLock<CustomDecompQueueState> = OnceLock::new();
    STATE.get_or_init(CustomDecompQueueState::new)
}

struct CustomDecompQueueObj;

unsafe extern "win64" fn ds_custom_decomp_qi(
    this: *mut ComObj<CustomDecompQueueObj>,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    if CUSTOM_DECOMP_IIDS.contains(&iid) {
        ComObj::add_ref(this);
        wr(ppv, this as *mut c_void);
        S_OK
    } else if iid == IID_IUnknown || iid == IID_IDSTORAGE_FACTORY {
        wr(ppv, factory_singleton() as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn ds_custom_decomp_release(this: *mut ComObj<CustomDecompQueueObj>) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    let mut cur = obj.refs.load(Ordering::Acquire);
    while cur > 1 {
        match obj.refs.compare_exchange_weak(
            cur,
            cur - 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return cur - 1,
            Err(v) => cur = v,
        }
    }
    1
}

unsafe extern "win64" fn ds_custom_decomp_get_event(this: *mut c_void) -> HANDLE {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable()).is_none() {
        return std::ptr::null_mut();
    }
    crate::handles::insert(custom_decomp_state().event.clone())
}

unsafe extern "win64" fn ds_custom_decomp_get_requests(
    this: *mut c_void,
    max_requests: u32,
    requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
    num_requests: *mut u32,
) -> i32 {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable()).is_none() {
        return E_INVALIDARG;
    }
    custom_decomp_state().get_requests(
        DSTORAGE_GET_REQUEST_FLAG_SELECT_CUSTOM,
        max_requests,
        requests,
        num_requests,
    )
}

unsafe extern "win64" fn ds_custom_decomp_set_request_results(
    this: *mut c_void,
    num_results: u32,
    results: *const DSTORAGE_CUSTOM_DECOMPRESSION_RESULT,
) -> i32 {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable()).is_none() {
        return E_INVALIDARG;
    }
    custom_decomp_state().set_request_results(num_results, results)
}

unsafe extern "win64" fn ds_custom_decomp_get_requests1(
    this: *mut c_void,
    flags: u32,
    max_requests: u32,
    requests: *mut DSTORAGE_CUSTOM_DECOMPRESSION_REQUEST,
    num_requests: *mut u32,
) -> i32 {
    if com_from_iface::<CustomDecompQueueObj>(this, custom_decomp_vtable()).is_none() {
        return E_INVALIDARG;
    }
    custom_decomp_state().get_requests(flags, max_requests, requests, num_requests)
}

fn custom_decomp_vtable() -> *const usize {
    static VTBL: OnceLock<[usize; 7]> = OnceLock::new();
    register_vtable_once(&VTBL, || {
        [
            ds_custom_decomp_qi as *const () as usize,
            com_add_ref::<CustomDecompQueueObj> as *const () as usize,
            ds_custom_decomp_release as *const () as usize,
            ds_custom_decomp_get_event as *const () as usize,
            ds_custom_decomp_get_requests as *const () as usize,
            ds_custom_decomp_set_request_results as *const () as usize,
            ds_custom_decomp_get_requests1 as *const () as usize,
        ]
    })
}

pub(super) fn custom_decomp_singleton() -> usize {
    static QUEUE: OnceLock<usize> = OnceLock::new();
    let ptr = *QUEUE.get_or_init(|| {
        let _ = custom_decomp_state();
        ComObj::new(custom_decomp_vtable(), CustomDecompQueueObj) as usize
    });
    unsafe {
        ComObj::add_ref(ptr as *mut ComObj<CustomDecompQueueObj>);
    }
    ptr
}
