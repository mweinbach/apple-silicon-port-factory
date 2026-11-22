//! Cursors: system/custom HCURSORs, SetCursor, ShowCursor's display
//! counter, GetCursorPos/SetCursorPos, ClipCursor, and "relative mode"
//! (mouselook).
//!
//! Relative mode is entered while the cursor is hidden (ShowCursor count
//! < 0 or a blank/NULL cursor is set), our window is active, and raw mouse
//! input is registered. The pointer is then detached from the mouse
//! (`CGAssociateMouseAndMouseCursorPosition(false)`), raw deltas keep
//! flowing, and `SetCursorPos` only moves a virtual cursor (no physical
//! warp, which would stall input for ~250 ms on macOS).

use super::*;
use crate::types::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::Mutex;

/// What a cursor handle looks like on the native side.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    Arrow,
    IBeam,
    Wait,
    Cross,
    Hand,
    SizeNWSE,
    SizeNESW,
    SizeWE,
    SizeNS,
    SizeAll,
    No,
    /// Fully transparent custom cursor (AW2 creates one to hide the pointer).
    Blank,
    /// Custom bitmap cursor (not rendered yet; shown as an arrow).
    Custom,
}

#[derive(Clone, Debug)]
struct Cursor {
    shape: Shape,
}

const SYSTEM_BASE: usize = 0x0006_0000;
static NEXT_CUSTOM: AtomicUsize = AtomicUsize::new(0x0007_0000);

fn cursors() -> &'static Mutex<HashMap<usize, Cursor>> {
    static C: std::sync::OnceLock<Mutex<HashMap<usize, Cursor>>> = std::sync::OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

fn system_shape(id: u16) -> Option<Shape> {
    Some(match id {
        32512 => Shape::Arrow,     // IDC_ARROW
        32513 => Shape::IBeam,     // IDC_IBEAM
        32514 => Shape::Wait,      // IDC_WAIT
        32515 => Shape::Cross,     // IDC_CROSS
        32516 => Shape::Arrow,     // IDC_UPARROW
        32642 => Shape::SizeNWSE,  // IDC_SIZENWSE
        32643 => Shape::SizeNESW,  // IDC_SIZENESW
        32644 => Shape::SizeWE,    // IDC_SIZEWE
        32645 => Shape::SizeNS,    // IDC_SIZENS
        32646 => Shape::SizeAll,   // IDC_SIZEALL
        32648 => Shape::No,        // IDC_NO
        32649 => Shape::Hand,      // IDC_HAND
        32650 => Shape::Wait,      // IDC_APPSTARTING
        32651 => Shape::Arrow,     // IDC_HELP
        _ => return None,
    })
}

fn load_cursor(inst: usize, name: *const u16, ansi: bool) -> usize {
    if inst == 0 && is_int_resource(name) {
        let id = name as usize as u16;
        if let Some(shape) = system_shape(id) {
            let h = SYSTEM_BASE + id as usize;
            cursors().lock().unwrap().entry(h).or_insert(Cursor { shape });
            return h;
        }
    }
    let n = if is_int_resource(name) {
        format!("#{}", name as usize)
    } else if ansi {
        unsafe { crate::strings::from_ansi(name as *const u8) }
    } else {
        unsafe { crate::strings::from_wide(name) }
    };
    stub!("LoadCursor({inst:#x}, {n:?}): cursor resources not supported -> NULL");
    set_error(e::RESOURCE_TYPE_NOT_FOUND);
    0
}

pub unsafe extern "win64" fn LoadCursorA(inst: HINSTANCE, name: LPCSTR) -> usize {
    load_cursor(inst as usize, name as *const u16, true)
}

pub unsafe extern "win64" fn LoadCursorW(inst: HINSTANCE, name: LPCWSTR) -> usize {
    load_cursor(inst as usize, name, false)
}

const RT_ICON: u16 = 3;
const RT_GROUP_ICON: u16 = 14;
const LR_DEFAULTSIZE: u32 = 0x0040;
const LR_SHARED: u32 = 0x8000;

const SYSTEM_ICON_BASE: usize = 0x0008_0000;
static NEXT_ICON: AtomicUsize = AtomicUsize::new(0x0008_8000);

#[derive(Clone, Debug)]
struct IconData {
    f_icon: bool,
    x_hotspot: u32,
    y_hotspot: u32,
    width: u32,
    height: u32,
    bpp: u16,
    /// True if `raw_data` holds PNG bytes rather than a DIB payload.
    is_png: bool,
    /// Raw resource payload (DIB starting with `BITMAPINFOHEADER`, or PNG bytes).
    raw_data: Vec<u8>,
    /// Decoded 32bpp top-down BGRA pixels (`width * height * 4` bytes).
    bgra_pixels: Vec<u8>,
    /// 1bpp top-down AND mask with 16-bit aligned rows (`((width + 15) & !15) / 8 * height`).
    and_mask: Vec<u8>,
    /// Shared system or `LR_SHARED` icons are not removed by `DestroyIcon`.
    shared: bool,
}

fn icons() -> &'static Mutex<HashMap<usize, IconData>> {
    static I: std::sync::OnceLock<Mutex<HashMap<usize, IconData>>> = std::sync::OnceLock::new();
    I.get_or_init(|| Mutex::new(HashMap::new()))
}

enum IconResId {
    Int(u16),
    Str(String),
}

unsafe fn icon_res_id(name: *const u16, ansi: bool) -> Option<IconResId> {
    if name.is_null() {
        return None;
    }
    if is_int_resource(name) {
        return Some(IconResId::Int(name as usize as u16));
    }
    let s = if ansi {
        crate::strings::from_ansi(name as *const u8)
    } else {
        crate::strings::from_wide(name)
    };
    if let Some(n) = s.strip_prefix('#').and_then(|v| v.parse::<u16>().ok()) {
        return Some(IconResId::Int(n));
    }
    Some(IconResId::Str(s))
}

fn is_stock_icon_id(id: u16) -> bool {
    (32512..=32518).contains(&id)
}

fn make_solid_icon(width: u32, height: u32, shared: bool) -> IconData {
    let w = width.clamp(1, 1024);
    let h = height.clamp(1, 1024);
    let bgra_pixels = vec![0xFFu8; (w as usize) * (h as usize) * 4];
    let and_stride = (((w as usize) + 15) & !15) / 8;
    let and_mask = vec![0u8; and_stride * (h as usize)];
    IconData {
        f_icon: true,
        x_hotspot: w / 2,
        y_hotspot: h / 2,
        width: w,
        height: h,
        bpp: 32,
        is_png: false,
        raw_data: Vec::new(),
        bgra_pixels,
        and_mask,
        shared,
    }
}

/// Bounds-checked reader over a mapped PE32/PE32+ image in guest memory.
struct PeImageView {
    base: usize,
    image_size: usize,
    rsrc_rva: usize,
    rsrc_size: usize,
}

impl PeImageView {
    unsafe fn from_hinstance(base: usize) -> Option<Self> {
        if base < 0x1000 {
            return None;
        }
        let mz = (base as *const u16).read_unaligned();
        if mz != 0x5A4D {
            return None;
        }
        let e_lfanew = ((base + 0x3C) as *const u32).read_unaligned() as usize;
        if !(0x40..=0x0010_0000).contains(&e_lfanew) {
            return None;
        }
        let nt = base.checked_add(e_lfanew)?;
        let sig = (nt as *const u32).read_unaligned();
        if sig != 0x0000_4550 {
            return None;
        }
        let file_hdr = nt + 4;
        let opt_size = ((file_hdr + 16) as *const u16).read_unaligned() as usize;
        let opt = file_hdr + 20;
        if opt_size < 96 {
            return None;
        }
        let magic = (opt as *const u16).read_unaligned();
        let (num_rva_off, data_dir_off) = match magic {
            0x20B => (108usize, 112usize), // PE32+
            0x10B => (92usize, 96usize),   // PE32
            _ => return None,
        };
        if opt_size < data_dir_off + 24 {
            return None;
        }
        let image_size = ((opt + 56) as *const u32).read_unaligned() as usize;
        let min_hdr = e_lfanew.checked_add(24)?.checked_add(opt_size)?;
        if image_size < min_hdr {
            return None;
        }
        let num_dirs = ((opt + num_rva_off) as *const u32).read_unaligned() as usize;
        if num_dirs <= 2 {
            return None;
        }
        let rsrc_dir_ptr = opt + data_dir_off + 2 * 8;
        let rsrc_rva = (rsrc_dir_ptr as *const u32).read_unaligned() as usize;
        let rsrc_size = ((rsrc_dir_ptr + 4) as *const u32).read_unaligned() as usize;
        if rsrc_rva == 0 || rsrc_size < 16 || rsrc_rva.checked_add(rsrc_size)? > image_size {
            return None;
        }
        Some(Self { base, image_size, rsrc_rva, rsrc_size })
    }

    unsafe fn slice_rva(&self, rva: usize, len: usize) -> Option<&[u8]> {
        let end = rva.checked_add(len)?;
        if end > self.image_size {
            return None;
        }
        Some(std::slice::from_raw_parts((self.base + rva) as *const u8, len))
    }

    unsafe fn rsrc_u16(&self, off: usize) -> Option<u16> {
        let end = off.checked_add(2)?;
        if end > self.rsrc_size {
            return None;
        }
        Some(((self.base + self.rsrc_rva + off) as *const u16).read_unaligned())
    }

    unsafe fn rsrc_u32(&self, off: usize) -> Option<u32> {
        let end = off.checked_add(4)?;
        if end > self.rsrc_size {
            return None;
        }
        Some(((self.base + self.rsrc_rva + off) as *const u32).read_unaligned())
    }

    unsafe fn rsrc_dir_find(&self, dir_off: usize, target: &IconResId) -> Option<u32> {
        let named = self.rsrc_u16(dir_off.checked_add(12)?)? as usize;
        let ids = self.rsrc_u16(dir_off.checked_add(14)?)? as usize;
        let total = named.checked_add(ids)?;
        let entries_off = dir_off.checked_add(16)?;
        if entries_off.checked_add(total.checked_mul(8)?)? > self.rsrc_size {
            return None;
        }
        for i in 0..total {
            let e = entries_off + i * 8;
            let name_or_id = self.rsrc_u32(e)?;
            let data_or_sub = self.rsrc_u32(e + 4)?;
            let matched = match target {
                IconResId::Int(n) => (name_or_id & 0x8000_0000) == 0 && (name_or_id as u16) == *n,
                IconResId::Str(want) => {
                    if (name_or_id & 0x8000_0000) == 0 {
                        false
                    } else {
                        let str_off = (name_or_id & 0x7FFF_FFFF) as usize;
                        let count = self.rsrc_u16(str_off)? as usize;
                        let chars_off = str_off.checked_add(2)?;
                        let byte_len = count.checked_mul(2)?;
                        if chars_off.checked_add(byte_len)? > self.rsrc_size {
                            continue;
                        }
                        let mut units = Vec::with_capacity(count);
                        for j in 0..count {
                            units.push(self.rsrc_u16(chars_off + j * 2)?);
                        }
                        String::from_utf16_lossy(&units).eq_ignore_ascii_case(want)
                    }
                }
            };
            if matched {
                return Some(data_or_sub);
            }
        }
        None
    }

    unsafe fn rsrc_dir_first_leaf(&self, dir_off: usize) -> Option<u32> {
        let named = self.rsrc_u16(dir_off.checked_add(12)?)? as usize;
        let ids = self.rsrc_u16(dir_off.checked_add(14)?)? as usize;
        let total = named.checked_add(ids)?;
        let entries_off = dir_off.checked_add(16)?;
        if entries_off.checked_add(total.checked_mul(8)?)? > self.rsrc_size {
            return None;
        }
        for i in 0..total {
            let data_or_sub = self.rsrc_u32(entries_off + i * 8 + 4)?;
            if (data_or_sub & 0x8000_0000) == 0 {
                return Some(data_or_sub);
            }
        }
        None
    }

    unsafe fn find_resource_data(&self, ty: u16, name: &IconResId) -> Option<&[u8]> {
        let t_off = self.rsrc_dir_find(0, &IconResId::Int(ty))?;
        if (t_off & 0x8000_0000) == 0 {
            return None;
        }
        let t_dir = (t_off & 0x7FFF_FFFF) as usize;
        let n_off = self.rsrc_dir_find(t_dir, name)?;
        if (n_off & 0x8000_0000) == 0 {
            return None;
        }
        let l_dir = (n_off & 0x7FFF_FFFF) as usize;
        let leaf = self
            .rsrc_dir_find(l_dir, &IconResId::Int(0x0409))
            .filter(|v| (v & 0x8000_0000) == 0)
            .or_else(|| self.rsrc_dir_find(l_dir, &IconResId::Int(0)).filter(|v| (v & 0x8000_0000) == 0))
            .or_else(|| self.rsrc_dir_first_leaf(l_dir))?;
        let leaf_off = leaf as usize;
        if leaf_off.checked_add(16)? > self.rsrc_size {
            return None;
        }
        let data_rva = self.rsrc_u32(leaf_off)? as usize;
        let data_size = self.rsrc_u32(leaf_off + 4)? as usize;
        if data_size == 0 {
            return None;
        }
        self.slice_rva(data_rva, data_size)
    }
}

#[derive(Clone, Copy, Debug)]
struct GrpIconDirEntry {
    width: u32,
    height: u32,
    bpp: u16,
    icon_id: u16,
}

fn parse_group_icon_dir(data: &[u8]) -> Vec<GrpIconDirEntry> {
    if data.len() < 6 {
        return Vec::new();
    }
    let reserved = u16::from_le_bytes([data[0], data[1]]);
    let ty = u16::from_le_bytes([data[2], data[3]]);
    let count = u16::from_le_bytes([data[4], data[5]]) as usize;
    if reserved != 0 || (ty != 1 && ty != 2) || count == 0 {
        return Vec::new();
    }
    let max_entries = (data.len() - 6) / 14;
    let n = count.min(max_entries);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let base = 6 + i * 14;
        let bw = data[base];
        let bh = data[base + 1];
        let color_count = data[base + 2];
        let bit_count = u16::from_le_bytes([data[base + 6], data[base + 7]]);
        let icon_id = u16::from_le_bytes([data[base + 12], data[base + 13]]);
        let width = if bw == 0 { 256 } else { bw as u32 };
        let height = if bh == 0 { 256 } else { bh as u32 };
        let bpp = if bit_count != 0 {
            bit_count
        } else if color_count != 0 {
            match color_count {
                0..=2 => 1,
                3..=16 => 4,
                _ => 8,
            }
        } else {
            32
        };
        out.push(GrpIconDirEntry { width, height, bpp, icon_id });
    }
    out
}

fn rank_group_icon_entries(entries: &[GrpIconDirEntry], want_w: u32, want_h: u32) -> Vec<GrpIconDirEntry> {
    let mut ranked = entries.to_vec();
    if want_w == 0 && want_h == 0 {
        // No explicit dimension requested: prefer highest BPP (up to 32), then largest area.
        ranked.sort_by(|a, b| {
            let ba = a.bpp.min(32);
            let bb = b.bpp.min(32);
            bb.cmp(&ba).then_with(|| (b.width * b.height).cmp(&(a.width * a.height)))
        });
    } else {
        let tw = if want_w > 0 { want_w } else { want_h };
        let th = if want_h > 0 { want_h } else { want_w };
        ranked.sort_by(|a, b| {
            let score = |e: &GrpIconDirEntry| -> (u32, u32, u16) {
                let exact = u32::from(!(e.width == tw && e.height == th));
                let dw = e.width.abs_diff(tw);
                let dh = e.height.abs_diff(th);
                // Prefer scaling down slightly over scaling up at equal distance.
                let upscale_penalty = u32::from(e.width < tw || e.height < th);
                let dist = (dw + dh).saturating_mul(2).saturating_add(upscale_penalty);
                let bpp_inv = 32u16.saturating_sub(e.bpp.min(32));
                (exact, dist, bpp_inv)
            };
            score(a).cmp(&score(b))
        });
    }
    ranked
}

fn decode_icon_resource_bytes(
    bytes: &[u8],
    f_icon: bool,
    fallback_w: u32,
    fallback_h: u32,
    fallback_bpp: u16,
    shared: bool,
) -> Option<IconData> {
    if bytes.is_empty() {
        return None;
    }
    // PNG-encoded icon (Vista+ 256x256 or arbitrary PNG resource).
    if bytes.len() >= 26 && &bytes[0..8] == b"\x89PNG\r\n\x1a\n" && &bytes[12..16] == b"IHDR" {
        let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
        let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
        if width == 0 || height == 0 || width > 4096 || height > 4096 {
            return None;
        }
        let bit_depth = bytes[24] as u16;
        let color_type = bytes[25];
        let bpp = match color_type {
            6 => bit_depth.saturating_mul(4),
            2 => bit_depth.saturating_mul(3),
            4 => bit_depth.saturating_mul(2),
            _ => bit_depth,
        }
        .max(1);
        let and_stride = (((width as usize) + 15) & !15) / 8;
        return Some(IconData {
            f_icon,
            x_hotspot: width / 2,
            y_hotspot: height / 2,
            width,
            height,
            bpp,
            is_png: true,
            raw_data: bytes.to_vec(),
            bgra_pixels: vec![0u8; (width as usize).saturating_mul(height as usize).saturating_mul(4)],
            and_mask: vec![0u8; and_stride.saturating_mul(height as usize)],
            shared,
        });
    }
    // DIB-encoded icon: BITMAPINFOHEADER (+ optional palette) + XOR color + 1bpp AND mask.
    if bytes.len() < 40 {
        return None;
    }
    let bi_size = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
    if bi_size < 40 || bi_size > bytes.len() {
        return None;
    }
    let raw_w = i32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let raw_h = i32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let bi_bpp = u16::from_le_bytes([bytes[14], bytes[15]]);
    let bi_clr_used = u32::from_le_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]) as usize;

    let width = {
        let w = raw_w.unsigned_abs();
        if w > 0 { w } else { fallback_w.max(1) }
    };
    let top_down = raw_h < 0;
    let abs_h = raw_h.unsigned_abs();
    // In RT_ICON DIBs, biHeight is 2 * icon height (XOR + AND bitmaps stacked).
    let height = if abs_h >= 2 {
        abs_h / 2
    } else if abs_h == 1 {
        1
    } else {
        fallback_h.max(1)
    };
    if width == 0 || height == 0 || width > 4096 || height > 4096 {
        return None;
    }
    let bpp = if bi_bpp != 0 { bi_bpp } else { fallback_bpp.max(32) };
    let w_u = width as usize;
    let h_u = height as usize;

    let palette_entries = if bpp <= 8 {
        if bi_clr_used != 0 {
            bi_clr_used.min(256)
        } else {
            1usize << (bpp as usize)
        }
    } else {
        0
    };
    let palette_off = bi_size;
    let palette_bytes = palette_entries.saturating_mul(4);
    let xor_off = palette_off.saturating_add(palette_bytes);
    let xor_stride = ((w_u.saturating_mul(bpp as usize).saturating_add(31)) & !31) / 8;
    let xor_bytes = xor_stride.saturating_mul(h_u);
    let and_off = xor_off.saturating_add(xor_bytes);
    let dib_and_stride = ((w_u.saturating_add(31)) & !31) / 8;

    let ddb_and_stride = ((w_u.saturating_add(15)) & !15) / 8;
    let mut and_mask = vec![0u8; ddb_and_stride.saturating_mul(h_u)];
    let has_and = and_off.saturating_add(dib_and_stride.saturating_mul(h_u)) <= bytes.len();
    if has_and {
        for y in 0..h_u {
            let src_row = if top_down { y } else { h_u - 1 - y };
            let src_start = and_off + src_row * dib_and_stride;
            let dst_start = y * ddb_and_stride;
            let copy_n = dib_and_stride.min(ddb_and_stride);
            and_mask[dst_start..dst_start + copy_n].copy_from_slice(&bytes[src_start..src_start + copy_n]);
        }
    }

    let mut bgra = vec![0u8; w_u.saturating_mul(h_u).saturating_mul(4)];
    if xor_off.saturating_add(xor_bytes) <= bytes.len() {
        let mut any_alpha = false;
        for y in 0..h_u {
            let src_row = if top_down { y } else { h_u - 1 - y };
            let row = &bytes[xor_off + src_row * xor_stride..xor_off + (src_row + 1) * xor_stride];
            for x in 0..w_u {
                let and_bit = if has_and {
                    (and_mask[y * ddb_and_stride + x / 8] >> (7 - (x % 8))) & 1
                } else {
                    0
                };
                let mask_alpha = if and_bit != 0 { 0u8 } else { 0xFFu8 };
                let px: [u8; 4] = match bpp {
                    32 => {
                        let o = x * 4;
                        let a = row[o + 3];
                        if a != 0 {
                            any_alpha = true;
                        }
                        [row[o], row[o + 1], row[o + 2], a]
                    }
                    24 => {
                        let o = x * 3;
                        [row[o], row[o + 1], row[o + 2], mask_alpha]
                    }
                    8 => {
                        let idx = row[x] as usize;
                        let po = palette_off + idx * 4;
                        if idx < palette_entries && po + 4 <= bytes.len() {
                            [bytes[po], bytes[po + 1], bytes[po + 2], mask_alpha]
                        } else {
                            [0, 0, 0, mask_alpha]
                        }
                    }
                    4 => {
                        let b = row[x / 2];
                        let idx = if x % 2 == 0 { (b >> 4) & 0xF } else { b & 0xF } as usize;
                        let po = palette_off + idx * 4;
                        if idx < palette_entries && po + 4 <= bytes.len() {
                            [bytes[po], bytes[po + 1], bytes[po + 2], mask_alpha]
                        } else {
                            [0, 0, 0, mask_alpha]
                        }
                    }
                    1 => {
                        let idx = ((row[x / 8] >> (7 - (x % 8))) & 1) as usize;
                        let po = palette_off + idx * 4;
                        if idx < palette_entries && po + 4 <= bytes.len() {
                            [bytes[po], bytes[po + 1], bytes[po + 2], mask_alpha]
                        } else if idx != 0 {
                            [0xFF, 0xFF, 0xFF, mask_alpha]
                        } else {
                            [0, 0, 0, mask_alpha]
                        }
                    }
                    _ => [0, 0, 0, mask_alpha],
                };
                let d = (y * w_u + x) * 4;
                bgra[d..d + 4].copy_from_slice(&px);
            }
        }
        // Legacy 32bpp icons with all-zero alpha channel fall back to the AND mask.
        if bpp == 32 && !any_alpha && has_and {
            for y in 0..h_u {
                for x in 0..w_u {
                    let and_bit = (and_mask[y * ddb_and_stride + x / 8] >> (7 - (x % 8))) & 1;
                    bgra[(y * w_u + x) * 4 + 3] = if and_bit != 0 { 0 } else { 0xFF };
                }
            }
        }
    }

    Some(IconData {
        f_icon,
        x_hotspot: width / 2,
        y_hotspot: height / 2,
        width,
        height,
        bpp,
        is_png: false,
        raw_data: bytes.to_vec(),
        bgra_pixels: bgra,
        and_mask,
        shared,
    })
}

fn load_icon_impl(inst: usize, name: *const u16, ansi: bool, cx: i32, cy: i32, flags: u32) -> usize {
    let Some(res_id) = (unsafe { icon_res_id(name, ansi) }) else {
        set_error(e::INVALID_PARAMETER);
        return 0;
    };

    // System stock icons (`inst == 0`, `IDI_*` 32512..=32518).
    if inst == 0 {
        if let IconResId::Int(id) = res_id {
            if is_stock_icon_id(id) {
                let w = if cx > 0 { cx as u32 } else { 32 };
                let h = if cy > 0 { cy as u32 } else { 32 };
                if w == 32 && h == 32 {
                    let handle = SYSTEM_ICON_BASE + id as usize;
                    icons().lock().unwrap().entry(handle).or_insert_with(|| make_solid_icon(32, 32, true));
                    return handle;
                }
                let handle = NEXT_ICON.fetch_add(4, Ordering::Relaxed);
                icons()
                    .lock()
                    .unwrap()
                    .insert(handle, make_solid_icon(w, h, (flags & LR_SHARED) != 0));
                return handle;
            }
        }
        set_error(1814); // ERROR_RESOURCE_NAME_NOT_FOUND
        return 0;
    }

    let want_w = if cx > 0 {
        cx as u32
    } else if (flags & LR_DEFAULTSIZE) != 0 {
        32
    } else {
        0
    };
    let want_h = if cy > 0 {
        cy as u32
    } else if (flags & LR_DEFAULTSIZE) != 0 {
        32
    } else {
        0
    };

    let Some(pe) = (unsafe { PeImageView::from_hinstance(inst) }) else {
        set_error(e::RESOURCE_TYPE_NOT_FOUND);
        return 0;
    };

    let Some(group_bytes) = (unsafe { pe.find_resource_data(RT_GROUP_ICON, &res_id) }) else {
        set_error(1814); // ERROR_RESOURCE_NAME_NOT_FOUND
        return 0;
    };
    let entries = parse_group_icon_dir(group_bytes);
    if entries.is_empty() {
        set_error(1812); // ERROR_RESOURCE_DATA_NOT_FOUND
        return 0;
    }

    let ranked = rank_group_icon_entries(&entries, want_w, want_h);
    let shared = (flags & LR_SHARED) != 0;
    for entry in ranked {
        if let Some(icon_bytes) = unsafe { pe.find_resource_data(RT_ICON, &IconResId::Int(entry.icon_id)) } {
            if let Some(icon_data) = decode_icon_resource_bytes(
                icon_bytes,
                true,
                entry.width,
                entry.height,
                entry.bpp,
                shared,
            ) {
                let handle = NEXT_ICON.fetch_add(4, Ordering::Relaxed);
                log::debug!(
                    "user32: loaded PE .rsrc icon from {inst:#x} ({}x{} {}bpp, png={}, {}B) -> {handle:#x}",
                    icon_data.width,
                    icon_data.height,
                    icon_data.bpp,
                    icon_data.is_png,
                    icon_data.raw_data.len(),
                );
                icons().lock().unwrap().insert(handle, icon_data);
                return handle;
            }
        }
    }

    set_error(1812); // ERROR_RESOURCE_DATA_NOT_FOUND
    0
}

pub unsafe extern "win64" fn LoadIconA(inst: HINSTANCE, name: LPCSTR) -> usize {
    load_icon_impl(inst as usize, name as *const u16, true, 32, 32, LR_DEFAULTSIZE)
}

pub unsafe extern "win64" fn LoadIconW(inst: HINSTANCE, name: LPCWSTR) -> usize {
    load_icon_impl(inst as usize, name, false, 32, 32, LR_DEFAULTSIZE)
}

pub unsafe extern "win64" fn DestroyIcon(h: usize) -> BOOL {
    if h == 0 {
        set_error(e::INVALID_CURSOR_HANDLE);
        return FALSE;
    }
    if (SYSTEM_BASE..SYSTEM_BASE + 0x10000).contains(&h) {
        return TRUE;
    }
    {
        let mut tbl = icons().lock().unwrap();
        if let Some(icon) = tbl.get(&h) {
            if icon.shared {
                return TRUE;
            }
            tbl.remove(&h);
            return TRUE;
        }
    }
    if cursors().lock().unwrap().remove(&h).is_some() {
        return TRUE;
    }
    set_error(e::INVALID_CURSOR_HANDLE);
    FALSE
}

#[repr(C)]
pub struct ICONINFO {
    pub f_icon: BOOL,
    pub x_hotspot: u32,
    pub y_hotspot: u32,
    pub hbm_mask: usize,
    pub hbm_color: usize,
}

unsafe fn create_icon_color_dib(w: u32, h: u32, bgra: &[u8]) -> usize {
    let mut bmi = [0u8; 40];
    bmi[0..4].copy_from_slice(&40u32.to_le_bytes());
    bmi[4..8].copy_from_slice(&(w as i32).to_le_bytes());
    bmi[8..12].copy_from_slice(&(-(h as i32)).to_le_bytes()); // top-down 32bpp
    bmi[12..14].copy_from_slice(&1u16.to_le_bytes());
    bmi[14..16].copy_from_slice(&32u16.to_le_bytes());
    let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
    let hbm = crate::misc::gdi32::CreateDIBSection(0, bmi.as_ptr(), 0, &mut bits, std::ptr::null_mut(), 0);
    if hbm != 0 && !bits.is_null() && !bgra.is_empty() {
        let cap = (w as usize).saturating_mul(h as usize).saturating_mul(4);
        let n = bgra.len().min(cap);
        std::ptr::copy_nonoverlapping(bgra.as_ptr(), bits as *mut u8, n);
    }
    hbm
}

pub unsafe extern "win64" fn CreateIconIndirect(info: *const ICONINFO) -> usize {
    if info.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let ii = info.read_unaligned();
    let mut width = 32u32;
    let mut height = 32u32;
    let mut bpp = 32u16;
    let probe_bm = if ii.hbm_color != 0 { ii.hbm_color } else { ii.hbm_mask };
    if probe_bm != 0 {
        let mut bm_buf = [0u8; 32];
        if crate::misc::gdi32::GetObjectW(probe_bm, 32, bm_buf.as_mut_ptr()) >= 20 {
            let bw = i32::from_le_bytes([bm_buf[4], bm_buf[5], bm_buf[6], bm_buf[7]]).unsigned_abs();
            let bh = i32::from_le_bytes([bm_buf[8], bm_buf[9], bm_buf[10], bm_buf[11]]).unsigned_abs();
            let bb = u16::from_le_bytes([bm_buf[18], bm_buf[19]]);
            if bw > 0 && bh > 0 {
                width = bw;
                height = if ii.hbm_color == 0 && bh >= 2 { bh / 2 } else { bh };
            }
            if bb > 0 {
                bpp = bb;
            }
        }
    }
    let mut icon = make_solid_icon(width, height, false);
    icon.f_icon = ii.f_icon != 0;
    icon.x_hotspot = ii.x_hotspot;
    icon.y_hotspot = ii.y_hotspot;
    icon.bpp = bpp;
    let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
    cursors().lock().unwrap().insert(hc, Cursor { shape: Shape::Custom });
    icons().lock().unwrap().insert(hc, icon);
    hc
}

pub unsafe extern "win64" fn GetIconInfo(hicon: usize, out: *mut ICONINFO) -> BOOL {
    if hicon == 0 || out.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    let maybe_icon = icons().lock().unwrap().get(&hicon).cloned();
    if let Some(icon) = maybe_icon {
        let w = icon.width.max(1);
        let h = icon.height.max(1);
        let mask_ptr = if icon.and_mask.is_empty() {
            std::ptr::null()
        } else {
            icon.and_mask.as_ptr()
        };
        let hbm_mask = crate::misc::gdi32::CreateBitmap(w as i32, h as i32, 1, 1, mask_ptr);
        let hbm_color = create_icon_color_dib(w, h, &icon.bgra_pixels);
        std::ptr::write_unaligned(
            out,
            ICONINFO {
                f_icon: if icon.f_icon { TRUE } else { FALSE },
                x_hotspot: icon.x_hotspot,
                y_hotspot: icon.y_hotspot,
                hbm_mask,
                hbm_color,
            },
        );
        return TRUE;
    }
    let is_cursor = (SYSTEM_BASE..SYSTEM_BASE + 0x10000).contains(&hicon)
        || cursors().lock().unwrap().contains_key(&hicon);
    if is_cursor {
        let hbm_mask = crate::misc::gdi32::CreateBitmap(32, 32, 1, 1, std::ptr::null());
        let hbm_color = create_icon_color_dib(32, 32, &[]);
        std::ptr::write_unaligned(
            out,
            ICONINFO { f_icon: FALSE, x_hotspot: 0, y_hotspot: 0, hbm_mask, hbm_color },
        );
        return TRUE;
    }
    set_error(e::INVALID_CURSOR_HANDLE);
    FALSE
}

pub unsafe extern "win64" fn CopyIcon(hicon: usize) -> usize {
    if hicon == 0 {
        set_error(e::INVALID_CURSOR_HANDLE);
        return 0;
    }
    if let Some(mut icon) = icons().lock().unwrap().get(&hicon).cloned() {
        icon.shared = false;
        let handle = NEXT_ICON.fetch_add(4, Ordering::Relaxed);
        icons().lock().unwrap().insert(handle, icon);
        return handle;
    }
    let shape = cursors()
        .lock()
        .unwrap()
        .get(&hicon)
        .map(|c| c.shape.clone())
        .or_else(|| {
            (SYSTEM_BASE..SYSTEM_BASE + 0x10000)
                .contains(&hicon)
                .then(|| system_shape((hicon - SYSTEM_BASE) as u16))
                .flatten()
        });
    let Some(shape) = shape else {
        set_error(e::INVALID_CURSOR_HANDLE);
        return 0;
    };
    let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
    cursors().lock().unwrap().insert(hc, Cursor { shape });
    hc
}

pub unsafe extern "win64" fn CreateIcon(
    _inst: HINSTANCE,
    w: i32,
    h: i32,
    _planes: u8,
    bpp: u8,
    and: *const u8,
    _xor: *const u8,
) -> usize {
    if w <= 0 || h <= 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let mut icon = make_solid_icon(w as u32, h as u32, false);
    icon.bpp = if bpp != 0 { bpp as u16 } else { 32 };
    if !and.is_null() {
        let stride = (((w as usize) + 15) & !15) / 8;
        let len = stride.saturating_mul(h as usize);
        icon.and_mask = std::slice::from_raw_parts(and, len).to_vec();
    }
    let hc = NEXT_ICON.fetch_add(4, Ordering::Relaxed);
    icons().lock().unwrap().insert(hc, icon);
    hc
}

pub unsafe extern "win64" fn CreateIconFromResourceEx(
    bits: *const u8,
    size: u32,
    f_icon: BOOL,
    _ver: u32,
    cx: i32,
    cy: i32,
    flags: u32,
) -> usize {
    if bits.is_null() || size == 0 {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    let slice = std::slice::from_raw_parts(bits, size as usize);
    let fw = if cx > 0 { cx as u32 } else { 32 };
    let fh = if cy > 0 { cy as u32 } else { 32 };
    let shared = (flags & LR_SHARED) != 0;
    let icon = decode_icon_resource_bytes(slice, f_icon != 0, fw, fh, 32, shared)
        .unwrap_or_else(|| {
            let mut m = make_solid_icon(fw, fh, shared);
            m.f_icon = f_icon != 0;
            m.raw_data = slice.to_vec();
            m
        });
    let hc = NEXT_ICON.fetch_add(4, Ordering::Relaxed);
    icons().lock().unwrap().insert(hc, icon);
    hc
}

pub unsafe extern "win64" fn LoadImageW(
    inst: HINSTANCE,
    name: LPCWSTR,
    ty: u32,
    cx: i32,
    cy: i32,
    flags: u32,
) -> usize {
    match ty {
        1 => load_icon_impl(inst as usize, name, false, cx, cy, flags), // IMAGE_ICON
        2 => LoadCursorW(inst, name),                                   // IMAGE_CURSOR
        _ => {
            let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
            cursors().lock().unwrap().insert(hc, Cursor { shape: Shape::Custom });
            hc
        }
    }
}

pub unsafe extern "win64" fn LoadImageA(
    inst: HINSTANCE,
    name: LPCSTR,
    ty: u32,
    cx: i32,
    cy: i32,
    flags: u32,
) -> usize {
    match ty {
        1 => load_icon_impl(inst as usize, name as *const u16, true, cx, cy, flags),
        2 => LoadCursorA(inst, name),
        _ => {
            let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
            cursors().lock().unwrap().insert(hc, Cursor { shape: Shape::Custom });
            hc
        }
    }
}

pub unsafe extern "win64" fn CopyImage(h: usize, _ty: u32, _cx: i32, _cy: i32, _flags: u32) -> usize {
    CopyIcon(h)
}

pub unsafe extern "win64" fn CreateCursor(_inst: HINSTANCE, _xhot: i32, _yhot: i32, w: i32, h: i32, and: *const u8, xor: *const u8) -> usize {
    if w <= 0 || h <= 0 || and.is_null() || xor.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    // Monochrome masks, rows padded to 16 bits.
    let stride = (((w + 15) / 16) * 2) as usize;
    let n = stride * h as usize;
    let a = std::slice::from_raw_parts(and, n);
    let x = std::slice::from_raw_parts(xor, n);
    let blank = a.iter().all(|&b| b == 0xFF) && x.iter().all(|&b| b == 0);
    let shape = if blank { Shape::Blank } else { Shape::Custom };
    if !blank {
        stub!("CreateCursor {w}x{h}: custom bitmap cursors are shown as the arrow");
    }
    let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
    cursors().lock().unwrap().insert(hc, Cursor { shape });
    log::debug!("CreateCursor {w}x{h} -> {hc:#x} ({})", if blank { "blank" } else { "custom" });
    hc
}

pub unsafe extern "win64" fn DestroyCursor(h: usize) -> BOOL {
    if h >= SYSTEM_BASE && h < SYSTEM_BASE + 0x10000 {
        return TRUE; // shared system cursors are never destroyed
    }
    if cursors().lock().unwrap().remove(&h).is_some() {
        TRUE
    } else {
        set_error(e::INVALID_CURSOR_HANDLE);
        FALSE
    }
}

// ---------------------------------------------------------------------------
// Current cursor, display counter, relative mode
// ---------------------------------------------------------------------------

static CURRENT: AtomicUsize = AtomicUsize::new(SYSTEM_BASE + 32512);
static SHOW_COUNT: AtomicI32 = AtomicI32::new(0);
static RELATIVE: AtomicBool = AtomicBool::new(false);
static POS_X: AtomicI32 = AtomicI32::new(0);
static POS_Y: AtomicI32 = AtomicI32::new(0);
static CLIP: Mutex<Option<RECT>> = Mutex::new(None);

pub fn relative_mode() -> bool {
    RELATIVE.load(Ordering::Acquire)
}

pub fn cursor_pos() -> POINT {
    POINT { x: POS_X.load(Ordering::Relaxed), y: POS_Y.load(Ordering::Relaxed) }
}

/// Updated by the backend from native mouse events (not in relative mode).
pub fn set_cached_pos(p: POINT) {
    let p = clip_point(p);
    POS_X.store(p.x, Ordering::Relaxed);
    POS_Y.store(p.y, Ordering::Relaxed);
}

fn clip_point(p: POINT) -> POINT {
    match *CLIP.lock().unwrap() {
        Some(r) if r.right > r.left && r.bottom > r.top => {
            POINT { x: p.x.clamp(r.left, r.right - 1), y: p.y.clamp(r.top, r.bottom - 1) }
        }
        _ => p,
    }
}

fn current_shape() -> Option<Shape> {
    let h = CURRENT.load(Ordering::Acquire);
    if h == 0 {
        return None;
    }
    if let Some(c) = cursors().lock().unwrap().get(&h) {
        return Some(c.shape.clone());
    }
    // System cursors are valid before anyone called LoadCursor (the
    // initial cursor is IDC_ARROW).
    if (SYSTEM_BASE..SYSTEM_BASE + 0x10000).contains(&h) {
        return system_shape((h - SYSTEM_BASE) as u16);
    }
    None
}

/// Recompute the native cursor state (visibility, shape, relative mode).
pub fn update_state() {
    let shape = current_shape();
    let hidden = SHOW_COUNT.load(Ordering::Acquire) < 0 || matches!(shape, None | Some(Shape::Blank));
    let ours = super::input::app_active() && super::input::active_window() != 0;
    let relative = hidden && ours && super::input::raw_mouse_registered() && crate::macos::lock_cursor_allowed();
    let was = RELATIVE.swap(relative, Ordering::AcqRel);
    if was != relative {
        log::info!("cursor relative mode {}", if relative { "ON" } else { "off" });
    }
    crate::macos::cursor_update(crate::macos::CursorState { hidden: hidden && ours, shape: shape.unwrap_or(Shape::Arrow), relative });
}

pub fn set_cursor(h: usize) -> usize {
    let old = CURRENT.swap(h, Ordering::AcqRel);
    if old != h {
        update_state();
    }
    old
}

pub unsafe extern "win64" fn SetCursor(h: usize) -> usize {
    set_cursor(h)
}

pub unsafe extern "win64" fn GetCursor() -> usize {
    CURRENT.load(Ordering::Acquire)
}

/// Process-wide display counter (Windows keeps one per thread; AW2 funnels
/// every call through its pump thread). Returns the new count.
pub unsafe extern "win64" fn ShowCursor(show: BOOL) -> i32 {
    let n = if show != 0 { SHOW_COUNT.fetch_add(1, Ordering::AcqRel) + 1 } else { SHOW_COUNT.fetch_sub(1, Ordering::AcqRel) - 1 };
    if n == 0 || n == -1 {
        update_state();
    }
    n
}

pub unsafe extern "win64" fn GetCursorPos(p: *mut POINT) -> BOOL {
    if p.is_null() {
        set_error(e::INVALID_PARAMETER);
        return FALSE;
    }
    *p = cursor_pos();
    TRUE
}

pub unsafe extern "win64" fn GetPhysicalCursorPos(p: *mut POINT) -> BOOL {
    GetCursorPos(p)
}

pub unsafe extern "win64" fn SetCursorPos(x: i32, y: i32) -> BOOL {
    let p = clip_point(POINT { x, y });
    let old_x = POS_X.swap(p.x, Ordering::Relaxed);
    let old_y = POS_Y.swap(p.y, Ordering::Relaxed);
    if relative_mode() || !crate::macos::lock_cursor_allowed() {
        // Virtual move only; Windows generates a WM_MOUSEMOVE for it, but only when the
        // cursor actually moves (the game re-centers every frame, many times).
        if old_x == p.x && old_y == p.y {
            return TRUE;
        }
        let target = {
            let cap = super::input::GetCapture() as usize;
            if cap != 0 {
                cap
            } else {
                super::window::WindowFromPoint(p) as usize
            }
        };
        if target != 0 {
            if let (Some(o), Some(w)) = (super::window::client_origin_screen(target), super::window::get(target)) {
                if let Some(q) = super::message::queue_of(w.tid) {
                    q.post_input(super::message::InputMsg {
                        msg: MSG {
                            hwnd: target,
                            message: wm::MOUSEMOVE,
                            wparam: 0,
                            lparam: make_long(p.x - o.x, p.y - o.y),
                            time: tick_ms(),
                            pt: p,
                            l_private: 0,
                        },
                        chars: Vec::new(),
                        ll_checked: false,
                        seq: 0,
                    });
                }
            }
        }
    } else {
        crate::macos::cursor_warp(p);
    }
    TRUE
}

pub unsafe extern "win64" fn SetPhysicalCursorPos(x: i32, y: i32) -> BOOL {
    SetCursorPos(x, y)
}

/// Confines the cursor (Windows clips the real pointer):
/// * the reported position is always clamped;
/// * when physical cursor locking is allowed (`AW2_FULLSCREEN=1` or
///   `AW2_LOCK_CURSOR=1`), while the app is active and not in relative mode,
///   the macOS pointer is warped back inside on every move that leaves the
///   rectangle (see [`clip_violation`]);
/// * in framed window mode (default), only the virtual Win32 cursor position
///   is clamped so the macOS cursor can freely reach the title bar, Dock,
///   menu bar, or other windows.
pub unsafe extern "win64" fn ClipCursor(r: *const RECT) -> BOOL {
    let v = if r.is_null() { None } else { Some(*r) };
    *CLIP.lock().unwrap() = v;
    log::debug!("ClipCursor({v:?})");
    if v.is_some() && super::input::app_active() && !relative_mode() {
        let p = cursor_pos();
        let c = clip_point(p);
        if c != p {
            set_cached_pos(c);
            if crate::macos::lock_cursor_allowed() {
                crate::macos::cursor_warp(c);
            }
        }
    }
    TRUE
}

/// Main thread, on native pointer motion: if a clip rectangle is set, the
/// app is active and physical cursor locking is enabled, return the clamped
/// point the real pointer must be warped to.
pub fn clip_violation(screen: POINT) -> Option<POINT> {
    if !crate::macos::lock_cursor_allowed() || relative_mode() || !super::input::app_active() {
        return None;
    }
    let c = clip_point(screen);
    (c != screen).then_some(c)
}

pub unsafe extern "win64" fn GetClipCursor(r: *mut RECT) -> BOOL {
    if r.is_null() {
        return FALSE;
    }
    *r = CLIP.lock().unwrap().unwrap_or_else(super::misc::virtual_screen);
    TRUE
}

#[repr(C)]
pub struct CURSORINFO {
    pub cb_size: u32,
    pub flags: u32,
    pub cursor: usize,
    pub pos: POINT,
}

pub unsafe extern "win64" fn GetCursorInfo(ci: *mut CURSORINFO) -> BOOL {
    if ci.is_null() {
        return FALSE;
    }
    (*ci).flags = (SHOW_COUNT.load(Ordering::Acquire) >= 0) as u32; // CURSOR_SHOWING
    (*ci).cursor = CURRENT.load(Ordering::Acquire);
    (*ci).pos = cursor_pos();
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn show_cursor_counter_and_handles() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            let base = ShowCursor(TRUE) - 1;
            assert_eq!(ShowCursor(FALSE), base);
            assert_eq!(ShowCursor(FALSE), base - 1);
            assert_eq!(ShowCursor(TRUE), base);
            let a = LoadCursorA(std::ptr::null_mut(), 32512 as LPCSTR);
            assert_ne!(a, 0);
            assert_eq!(a, LoadCursorW(std::ptr::null_mut(), 32512 as LPCWSTR));
            let and = [0xFFu8; 8];
            let xor = [0u8; 8];
            let c = CreateCursor(std::ptr::null_mut(), 0, 0, 16, 4, and.as_ptr(), xor.as_ptr());
            assert_ne!(c, 0);
            assert_eq!(DestroyCursor(c), TRUE);
        }
    }

    fn build_dib_icon_32(w: u32, h: u32, bgra_fill: [u8; 4]) -> Vec<u8> {
        let mut out = Vec::new();
        // BITMAPINFOHEADER (40 bytes): biHeight = 2 * h (XOR + AND)
        out.extend_from_slice(&40u32.to_le_bytes());
        out.extend_from_slice(&(w as i32).to_le_bytes());
        out.extend_from_slice(&((h * 2) as i32).to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
        out.extend_from_slice(&32u16.to_le_bytes()); // biBitCount
        out.extend_from_slice(&0u32.to_le_bytes()); // biCompression = BI_RGB
        out.extend_from_slice(&0u32.to_le_bytes()); // biSizeImage
        out.extend_from_slice(&[0u8; 16]); // pels/clr
        for _ in 0..(w * h) {
            out.extend_from_slice(&bgra_fill);
        }
        let and_stride = ((w as usize + 31) & !31) / 8;
        out.extend(std::iter::repeat_n(0u8, and_stride * h as usize));
        out
    }

    fn build_png_stub(w: u32, h: u32) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"\x89PNG\r\n\x1a\n");
        out.extend_from_slice(&13u32.to_be_bytes());
        out.extend_from_slice(b"IHDR");
        out.extend_from_slice(&w.to_be_bytes());
        out.extend_from_slice(&h.to_be_bytes());
        out.push(8); // bit depth
        out.push(6); // RGBA
        out.extend_from_slice(&[0, 0, 0]);
        out.extend_from_slice(&[0u8; 4]); // CRC placeholder
        out
    }

    /// Build a minimal valid PE32+ image in memory containing `.rsrc` with:
    /// - `RT_GROUP_ICON` #101 (entries: 16x16 32bpp DIB -> RT_ICON #1, 32x32 32bpp DIB -> RT_ICON #2, 256x256 PNG -> RT_ICON #3)
    /// - `RT_GROUP_ICON` "MAINICON" (entry: 16x16 32bpp DIB -> RT_ICON #1)
    fn build_test_pe_with_icons() -> Vec<u8> {
        let mut img = vec![0u8; 0x10000];
        // DOS header
        img[0..2].copy_from_slice(&0x5A4Du16.to_le_bytes());
        let pe_off = 0x80usize;
        img[0x3C..0x40].copy_from_slice(&(pe_off as u32).to_le_bytes());
        // PE signature + COFF header
        img[pe_off..pe_off + 4].copy_from_slice(&0x0000_4550u32.to_le_bytes());
        let file_hdr = pe_off + 4;
        let opt_size = 240u16;
        img[file_hdr + 16..file_hdr + 18].copy_from_slice(&opt_size.to_le_bytes());
        // Optional header (PE32+)
        let opt = file_hdr + 20;
        img[opt..opt + 2].copy_from_slice(&0x20Bu16.to_le_bytes());
        let img_len = img.len() as u32;
        img[opt + 56..opt + 60].copy_from_slice(&img_len.to_le_bytes()); // SizeOfImage
        img[opt + 108..opt + 112].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
        // Data Directory[2] = Resource table at RVA 0x1000, size 0xE000
        let rsrc_rva = 0x1000usize;
        let rsrc_size = 0xE000usize;
        let dd2 = opt + 112 + 2 * 8;
        img[dd2..dd2 + 4].copy_from_slice(&(rsrc_rva as u32).to_le_bytes());
        img[dd2 + 4..dd2 + 8].copy_from_slice(&(rsrc_size as u32).to_le_bytes());

        // Payloads placed at RVA 0x4000+
        let icon16 = build_dib_icon_32(16, 16, [0x11, 0x22, 0x33, 0xFF]);
        let icon32 = build_dib_icon_32(32, 32, [0xAA, 0xBB, 0xCC, 0xFF]);
        let icon256 = build_png_stub(256, 256);

        let mut grp101 = Vec::new();
        grp101.extend_from_slice(&0u16.to_le_bytes()); // idReserved
        grp101.extend_from_slice(&1u16.to_le_bytes()); // idType = icon
        grp101.extend_from_slice(&3u16.to_le_bytes()); // idCount = 3
        for (bw, bh, sz, nid) in [
            (16u8, 16u8, icon16.len() as u32, 1u16),
            (32u8, 32u8, icon32.len() as u32, 2u16),
            (0u8, 0u8, icon256.len() as u32, 3u16),
        ] {
            grp101.extend_from_slice(&[bw, bh, 0, 0]);
            grp101.extend_from_slice(&1u16.to_le_bytes()); // wPlanes
            grp101.extend_from_slice(&32u16.to_le_bytes()); // wBitCount
            grp101.extend_from_slice(&sz.to_le_bytes());
            grp101.extend_from_slice(&nid.to_le_bytes());
        }

        let mut grp_named = Vec::new();
        grp_named.extend_from_slice(&0u16.to_le_bytes());
        grp_named.extend_from_slice(&1u16.to_le_bytes());
        grp_named.extend_from_slice(&1u16.to_le_bytes());
        grp_named.extend_from_slice(&[16u8, 16u8, 0, 0]);
        grp_named.extend_from_slice(&1u16.to_le_bytes());
        grp_named.extend_from_slice(&32u16.to_le_bytes());
        grp_named.extend_from_slice(&(icon16.len() as u32).to_le_bytes());
        grp_named.extend_from_slice(&1u16.to_le_bytes());

        // Helper to write bytes into rsrc and return offset relative to rsrc_rva.
        let mut cursor_rva = 0x4000usize;
        let mut alloc_payload = |bytes: &[u8]| -> (u32, u32) {
            let rva = cursor_rva;
            img[rva..rva + bytes.len()].copy_from_slice(bytes);
            cursor_rva = (cursor_rva + bytes.len() + 15) & !15;
            (rva as u32, bytes.len() as u32)
        };
        let p_icon16 = alloc_payload(&icon16);
        let p_icon32 = alloc_payload(&icon32);
        let p_icon256 = alloc_payload(&icon256);
        let p_grp101 = alloc_payload(&grp101);
        let p_grp_named = alloc_payload(&grp_named);

        let write_dir = |img: &mut [u8], off: usize, named: u16, ids: u16, entries: &[(u32, u32)]| {
            let base = rsrc_rva + off;
            img[base + 12..base + 14].copy_from_slice(&named.to_le_bytes());
            img[base + 14..base + 16].copy_from_slice(&ids.to_le_bytes());
            for (i, &(k, v)) in entries.iter().enumerate() {
                let e = base + 16 + i * 8;
                img[e..e + 4].copy_from_slice(&k.to_le_bytes());
                img[e + 4..e + 8].copy_from_slice(&v.to_le_bytes());
            }
        };
        let write_data_entry = |img: &mut [u8], off: usize, (rva, sz): (u32, u32)| {
            let base = rsrc_rva + off;
            img[base..base + 4].copy_from_slice(&rva.to_le_bytes());
            img[base + 4..base + 8].copy_from_slice(&sz.to_le_bytes());
        };

        // String "MAINICON" at rsrc offset 0x400
        let str_off = 0x400usize;
        let name_u16: Vec<u16> = "MAINICON".encode_utf16().collect();
        img[rsrc_rva + str_off..rsrc_rva + str_off + 2].copy_from_slice(&(name_u16.len() as u16).to_le_bytes());
        for (i, u) in name_u16.iter().enumerate() {
            let p = rsrc_rva + str_off + 2 + i * 2;
            img[p..p + 2].copy_from_slice(&u.to_le_bytes());
        }

        // Root directory @ 0x000: RT_ICON (3) -> dir 0x040, RT_GROUP_ICON (14) -> dir 0x080
        write_dir(
            &mut img,
            0x000,
            0,
            2,
            &[(RT_ICON as u32, 0x8000_0040), (RT_GROUP_ICON as u32, 0x8000_0080)],
        );
        // RT_ICON name dir @ 0x040: #1 -> lang 0x100, #2 -> lang 0x120, #3 -> lang 0x140
        write_dir(
            &mut img,
            0x040,
            0,
            3,
            &[(1, 0x8000_0100), (2, 0x8000_0120), (3, 0x8000_0140)],
        );
        // RT_GROUP_ICON name dir @ 0x080: "MAINICON" -> lang 0x180, #101 -> lang 0x160
        write_dir(
            &mut img,
            0x080,
            1,
            1,
            &[(0x8000_0000 | str_off as u32, 0x8000_0180), (101, 0x8000_0160)],
        );

        // Language directories -> Data entries at 0x200..0x240
        write_dir(&mut img, 0x100, 0, 1, &[(0x0409, 0x200)]);
        write_dir(&mut img, 0x120, 0, 1, &[(0x0409, 0x210)]);
        write_dir(&mut img, 0x140, 0, 1, &[(0x0409, 0x220)]);
        write_dir(&mut img, 0x160, 0, 1, &[(0x0409, 0x230)]);
        write_dir(&mut img, 0x180, 0, 1, &[(0x0409, 0x240)]);

        write_data_entry(&mut img, 0x200, p_icon16);
        write_data_entry(&mut img, 0x210, p_icon32);
        write_data_entry(&mut img, 0x220, p_icon256);
        write_data_entry(&mut img, 0x230, p_grp101);
        write_data_entry(&mut img, 0x240, p_grp_named);

        img
    }

    #[test]
    fn pe_rsrc_icon_loading_dimensions_pixels_and_lifecycle() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let pe = build_test_pe_with_icons();
        let hinst = pe.as_ptr() as HINSTANCE;

        unsafe {
            // 1. LoadIconW by integer ID (#101) selects best 32x32 entry.
            let h32 = LoadIconW(hinst, 101 as LPCWSTR);
            assert_ne!(h32, 0);
            let mut ii = std::mem::zeroed::<ICONINFO>();
            assert_eq!(GetIconInfo(h32, &mut ii), TRUE);
            assert_eq!(ii.f_icon, TRUE);
            assert_eq!((ii.x_hotspot, ii.y_hotspot), (16, 16));
            assert_ne!(ii.hbm_color, 0);
            assert_ne!(ii.hbm_mask, 0);
            let mut bm = [0u8; 32];
            assert_eq!(crate::misc::gdi32::GetObjectW(ii.hbm_color, 32, bm.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm[4..8].try_into().unwrap()), 32);
            assert_eq!(i32::from_le_bytes(bm[8..12].try_into().unwrap()), 32);
            assert_eq!(u16::from_le_bytes(bm[18..20].try_into().unwrap()), 32);
            // Verify pixel data copied into hbmColor matches 32x32 DIB ([B=0xAA, G=0xBB, R=0xCC, A=0xFF]).
            let bits_ptr = usize::from_le_bytes(bm[24..32].try_into().unwrap()) as *const u8;
            assert!(!bits_ptr.is_null());
            assert_eq!(std::slice::from_raw_parts(bits_ptr, 4), &[0xAA, 0xBB, 0xCC, 0xFF]);
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);
            assert_eq!(DestroyIcon(h32), TRUE);
            assert_eq!(GetIconInfo(h32, &mut ii), FALSE);
            assert_eq!(DestroyIcon(h32), FALSE);

            // 2. LoadImageW requesting 16x16 selects the 16x16 DIB entry (#1).
            let h16 = LoadImageW(hinst, 101 as LPCWSTR, 1, 16, 16, 0);
            assert_ne!(h16, 0);
            assert_eq!(GetIconInfo(h16, &mut ii), TRUE);
            assert_eq!(crate::misc::gdi32::GetObjectW(ii.hbm_color, 32, bm.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm[4..8].try_into().unwrap()), 16);
            assert_eq!(i32::from_le_bytes(bm[8..12].try_into().unwrap()), 16);
            let bits16 = usize::from_le_bytes(bm[24..32].try_into().unwrap()) as *const u8;
            assert_eq!(std::slice::from_raw_parts(bits16, 4), &[0x11, 0x22, 0x33, 0xFF]);
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);
            assert_eq!(DestroyIcon(h16), TRUE);

            // 3. LoadImageW requesting 256x256 selects the PNG entry (#3) with 256x256 dimensions.
            let h256 = LoadImageW(hinst, 101 as LPCWSTR, 1, 256, 256, 0);
            assert_ne!(h256, 0);
            assert_eq!(GetIconInfo(h256, &mut ii), TRUE);
            assert_eq!(crate::misc::gdi32::GetObjectW(ii.hbm_color, 32, bm.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm[4..8].try_into().unwrap()), 256);
            assert_eq!(i32::from_le_bytes(bm[8..12].try_into().unwrap()), 256);
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);
            assert_eq!(DestroyIcon(h256), TRUE);

            // 4. Case-insensitive string resource name lookup via LoadIconA / LoadImageA.
            let h_named = LoadIconA(hinst, c"mainicon".as_ptr() as LPCSTR);
            assert_ne!(h_named, 0);
            assert_eq!(GetIconInfo(h_named, &mut ii), TRUE);
            assert_eq!(crate::misc::gdi32::GetObjectW(ii.hbm_color, 32, bm.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm[4..8].try_into().unwrap()), 16);
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);
            assert_eq!(DestroyIcon(h_named), TRUE);

            // 5. Missing resource returns NULL; stock system IDI_APPLICATION succeeds and stays shared.
            assert_eq!(LoadIconW(hinst, 999 as LPCWSTR), 0);
            let h_stock = LoadIconW(std::ptr::null_mut(), 32512 as LPCWSTR);
            assert_ne!(h_stock, 0);
            assert_eq!(DestroyIcon(h_stock), TRUE);
            assert_eq!(GetIconInfo(h_stock, &mut ii), TRUE);
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);
        }
    }
}
