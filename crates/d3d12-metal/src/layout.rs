//! Subresource layout math: `GetCopyableFootprints` (D3D12 linear layouts
//! for buffer↔texture copies).

use crate::format;
use crate::types::{dim, ResourceDesc};

/// D3D12_TEXTURE_DATA_PITCH_ALIGNMENT / PLACEMENT_ALIGNMENT.
pub const PITCH_ALIGN: u64 = 256;
pub const PLACEMENT_ALIGN: u64 = 512;

/// `D3D12_PLACED_SUBRESOURCE_FOOTPRINT` (32 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlacedFootprint {
    pub offset: u64,
    pub format: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub row_pitch: u32,
    pub _pad: u32,
}
const _: () = assert!(std::mem::size_of::<PlacedFootprint>() == 32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubresourceLayout {
    pub footprint: PlacedFootprint,
    pub num_rows: u32,
    pub row_size: u64,
}

fn align(v: u64, a: u64) -> u64 {
    v.div_ceil(a) * a
}

/// Copy format of plane `plane` of `fmt` (depth/stencil planes split).
pub fn plane_format(fmt: u32, plane: u32) -> u32 {
    match (fmt, plane) {
        // D24S8 family: R24G8_TYPELESS depth plane, R8_TYPELESS stencil.
        (44..=47, 0) => 44,
        (44..=47, _) => 60,
        // D32S8 family: R32_TYPELESS depth plane, R8_TYPELESS stencil.
        (19..=22, 0) => 39,
        (19..=22, _) => 60,
        _ => fmt,
    }
}

fn plane_bytes(fmt: u32, plane: u32) -> u32 {
    match (fmt, plane) {
        (44..=47, 0) | (19..=22, 0) => 4,
        (44..=47, _) | (19..=22, _) => 1,
        _ => format::info(fmt).bytes,
    }
}

/// Layouts of subresources `first..first+n` starting at `base`; returns
/// (layouts, total bytes). `None` = invalid desc/range.
pub fn copyable_footprints(desc: &ResourceDesc, first: u32, n: u32, base: u64) -> Option<(Vec<SubresourceLayout>, u64)> {
    if desc.is_buffer() {
        if first != 0 || n > 1 {
            return None;
        }
        let w = desc.Width;
        let l = SubresourceLayout {
            footprint: PlacedFootprint { offset: base, format: 0, width: w as u32, height: 1, depth: 1, row_pitch: align(w, PITCH_ALIGN) as u32, _pad: 0 },
            num_rows: 1,
            row_size: w,
        };
        return Some((if n == 1 { vec![l] } else { vec![] }, w));
    }
    if !matches!(desc.Dimension, dim::TEXTURE1D | dim::TEXTURE2D | dim::TEXTURE3D) {
        return None;
    }
    let fi = format::info(desc.Format);
    if fi.bytes == 0 {
        return None;
    }
    let mips = desc.mip_levels();
    let arr = desc.array_size();
    let planes = fi.planes.max(1);
    if first.checked_add(n)? > mips * arr * planes {
        return None;
    }
    let mut out = Vec::with_capacity(n as usize);
    let mut off = base;
    let mut total = 0u64;
    for s in first..first + n {
        let mip = s % mips;
        let plane = s / (mips * arr);
        let w = (desc.Width >> mip).max(1);
        let h = ((desc.Height as u64) >> mip).max(1);
        let d = ((desc.depth() as u64) >> mip).max(1);
        let (bw, bh) = (fi.block_w as u64, fi.block_h as u64);
        let bytes = plane_bytes(desc.Format, plane) as u64;
        let blocks_w = w.div_ceil(bw);
        let rows = h.div_ceil(bh);
        let row_size = blocks_w * bytes;
        let pitch = align(row_size, PITCH_ALIGN);
        off = align(off, PLACEMENT_ALIGN);
        let fp = PlacedFootprint {
            offset: off,
            format: plane_format(desc.Format, plane),
            width: align(w, bw) as u32,
            height: align(h, bh) as u32,
            depth: d as u32,
            row_pitch: pitch as u32,
            _pad: 0,
        };
        out.push(SubresourceLayout { footprint: fp, num_rows: rows as u32, row_size });
        // The last row of the last slice need not be padded.
        let size = pitch * rows * (d - 1) + pitch * (rows - 1) + row_size;
        total = off + size - base;
        off += pitch * rows * d;
    }
    Some((out, total))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tex2d(w: u64, h: u32, fmt: u32, mips: u16, arr: u16) -> ResourceDesc {
        ResourceDesc { Dimension: dim::TEXTURE2D, Width: w, Height: h, DepthOrArraySize: arr, MipLevels: mips, Format: fmt, SampleCount: 1, ..Default::default() }
    }

    #[test]
    fn rgba8_mip_chain() {
        // 100x60 RGBA8, 3 mips: pitches 512/256/256, offsets aligned to 512.
        let (l, total) = copyable_footprints(&tex2d(100, 60, 28, 3, 1), 0, 3, 0).unwrap();
        assert_eq!(l[0].footprint, PlacedFootprint { offset: 0, format: 28, width: 100, height: 60, depth: 1, row_pitch: 512, _pad: 0 });
        assert_eq!((l[0].num_rows, l[0].row_size), (60, 400));
        assert_eq!(l[1].footprint.offset, 512 * 60);
        assert_eq!((l[1].footprint.width, l[1].footprint.row_pitch, l[1].row_size), (50, 256, 200));
        assert_eq!(l[2].footprint.offset, 512 * 60 + 256 * 30);
        assert_eq!(total, 512 * 60 + 256 * 30 + 256 * 14 + 100);
    }

    #[test]
    fn bc1_rounds_to_blocks() {
        // BC1 (71) 10x10: 3x3 blocks of 8 bytes.
        let (l, total) = copyable_footprints(&tex2d(10, 10, 71, 1, 1), 0, 1, 0).unwrap();
        assert_eq!((l[0].footprint.width, l[0].footprint.height, l[0].num_rows, l[0].row_size), (12, 12, 3, 24));
        assert_eq!(total, 256 * 2 + 24);
    }

    #[test]
    fn buffer_and_base_offset() {
        let b = ResourceDesc { Dimension: dim::BUFFER, Width: 1000, Height: 1, DepthOrArraySize: 1, MipLevels: 1, SampleCount: 1, ..Default::default() };
        let (l, total) = copyable_footprints(&b, 0, 1, 64).unwrap();
        assert_eq!((l[0].footprint.offset, l[0].footprint.row_pitch, total), (64, 1024, 1000));
        // Texture subresources start on 512-byte boundaries after `base`.
        let (l, _) = copyable_footprints(&tex2d(4, 4, 28, 1, 2), 1, 1, 100).unwrap();
        assert_eq!(l[0].footprint.offset, 512);
    }

    #[test]
    fn depth_stencil_planes() {
        // D24S8 (45), 1 mip, 1 slice: subresource 1 = stencil plane.
        let (l, _) = copyable_footprints(&tex2d(64, 64, 45, 1, 1), 0, 2, 0).unwrap();
        assert_eq!((l[0].footprint.format, l[0].row_size), (44, 256));
        assert_eq!((l[1].footprint.format, l[1].row_size), (60, 64));
    }
}
