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
use std::sync::{Arc, Mutex};

/// Premultiplied RGBA8 custom cursor bitmap and hotspot for native `NSCursor` rendering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomCursorImage {
    pub width: u32,
    pub height: u32,
    pub hotspot_x: u32,
    pub hotspot_y: u32,
    /// Premultiplied RGBA8 top-down pixels (`width * height * 4` bytes).
    pub rgba: Arc<[u8]>,
}

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
    /// Custom bitmap cursor rendered via native `NSCursor::initWithImage_hotSpot`.
    Custom(CustomCursorImage),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum SharedResKeyId {
    Int(u16),
    Str(String),
}

impl From<&IconResId> for SharedResKeyId {
    fn from(id: &IconResId) -> Self {
        match id {
            IconResId::Int(n) => Self::Int(*n),
            IconResId::Str(s) => Self::Str(s.to_ascii_uppercase()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SharedPeResourceKey {
    inst: usize,
    is_icon: bool,
    res_id: SharedResKeyId,
    want_w: u32,
    want_h: u32,
}

fn shared_pe_resources() -> &'static Mutex<HashMap<SharedPeResourceKey, usize>> {
    static S: std::sync::OnceLock<Mutex<HashMap<SharedPeResourceKey, usize>>> =
        std::sync::OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
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

const RT_CURSOR: u16 = 1;
const RT_GROUP_CURSOR: u16 = 12;

fn bgra_to_premul_rgba(bgra: &[u8]) -> Vec<u8> {
    let mut rgba = vec![0u8; bgra.len()];
    for (src, dst) in bgra.chunks_exact(4).zip(rgba.chunks_exact_mut(4)) {
        let a = src[3] as u16;
        dst[0] = ((src[2] as u16 * a + 127) / 255) as u8;
        dst[1] = ((src[1] as u16 * a + 127) / 255) as u8;
        dst[2] = ((src[0] as u16 * a + 127) / 255) as u8;
        dst[3] = src[3];
    }
    rgba
}

fn shape_from_icon_data(icon: &IconData) -> Shape {
    let all_transparent = !icon.bgra_pixels.is_empty()
        && icon.bgra_pixels.chunks_exact(4).all(|px| px[3] == 0);
    if all_transparent {
        return Shape::Blank;
    }
    let rgba = bgra_to_premul_rgba(&icon.bgra_pixels);
    Shape::Custom(CustomCursorImage {
        width: icon.width,
        height: icon.height,
        hotspot_x: icon.x_hotspot,
        hotspot_y: icon.y_hotspot,
        rgba: Arc::from(rgba),
    })
}

/// Resample decoded icon/cursor pixels and 1bpp DDB AND mask to `(new_w, new_h)`
/// and scale the hotspot proportionally when dimensions differ.
fn resize_icon_data(icon: &mut IconData, new_w: u32, new_h: u32) {
    let dst_w = new_w.clamp(1, 1024);
    let dst_h = new_h.clamp(1, 1024);
    let src_w = icon.width;
    let src_h = icon.height;
    if src_w == 0 || src_h == 0 || (src_w == dst_w && src_h == dst_h) {
        return;
    }

    let sw = src_w as usize;
    let sh = src_h as usize;
    let dw = dst_w as usize;
    let dh = dst_h as usize;

    let mut new_bgra = vec![0u8; dw * dh * 4];
    if icon.bgra_pixels.len() == sw * sh * 4 {
        if icon.bpp <= 1 {
            // Preserve crisp 1bpp monochrome edges (black/white/transparent/invert).
            for dy in 0..dh {
                let sy = (dy * sh) / dh;
                for dx in 0..dw {
                    let sx = (dx * sw) / dw;
                    let s = (sy * sw + sx) * 4;
                    let d = (dy * dw + dx) * 4;
                    new_bgra[d..d + 4].copy_from_slice(&icon.bgra_pixels[s..s + 4]);
                }
            }
        } else {
            // Premultiplied-alpha bilinear filtering avoids dark fringes at transparent edges.
            let sample = |x: usize, y: usize| -> [f64; 4] {
                let o = (y * sw + x) * 4;
                let a = icon.bgra_pixels[o + 3] as f64;
                let af = a / 255.0;
                [
                    icon.bgra_pixels[o] as f64 * af,
                    icon.bgra_pixels[o + 1] as f64 * af,
                    icon.bgra_pixels[o + 2] as f64 * af,
                    a,
                ]
            };
            for dy in 0..dh {
                let fy = ((dy as f64 + 0.5) * (sh as f64) / (dh as f64) - 0.5)
                    .clamp(0.0, (sh - 1) as f64);
                let y0 = fy.floor() as usize;
                let y1 = (y0 + 1).min(sh - 1);
                let wy = fy - y0 as f64;
                for dx in 0..dw {
                    let fx = ((dx as f64 + 0.5) * (sw as f64) / (dw as f64) - 0.5)
                        .clamp(0.0, (sw - 1) as f64);
                    let x0 = fx.floor() as usize;
                    let x1 = (x0 + 1).min(sw - 1);
                    let wx = fx - x0 as f64;

                    let p00 = sample(x0, y0);
                    let p10 = sample(x1, y0);
                    let p01 = sample(x0, y1);
                    let p11 = sample(x1, y1);
                    let w00 = (1.0 - wx) * (1.0 - wy);
                    let w10 = wx * (1.0 - wy);
                    let w01 = (1.0 - wx) * wy;
                    let w11 = wx * wy;

                    let pb = p00[0] * w00 + p10[0] * w10 + p01[0] * w01 + p11[0] * w11;
                    let pg = p00[1] * w00 + p10[1] * w10 + p01[1] * w01 + p11[1] * w11;
                    let pr = p00[2] * w00 + p10[2] * w10 + p01[2] * w01 + p11[2] * w11;
                    let a = p00[3] * w00 + p10[3] * w10 + p01[3] * w01 + p11[3] * w11;

                    let out_a = a.round().clamp(0.0, 255.0) as u8;
                    let d = (dy * dw + dx) * 4;
                    if out_a == 0 {
                        new_bgra[d..d + 4].copy_from_slice(&[0, 0, 0, 0]);
                    } else {
                        let inv = 255.0 / a;
                        new_bgra[d] = (pb * inv).round().clamp(0.0, 255.0) as u8;
                        new_bgra[d + 1] = (pg * inv).round().clamp(0.0, 255.0) as u8;
                        new_bgra[d + 2] = (pr * inv).round().clamp(0.0, 255.0) as u8;
                        new_bgra[d + 3] = out_a;
                    }
                }
            }
        }
    }

    let src_and_stride = ((sw + 15) & !15) / 8;
    let dst_and_stride = ((dw + 15) & !15) / 8;
    let mut new_and = vec![0u8; dst_and_stride * dh];
    if icon.and_mask.len() == src_and_stride * sh {
        for dy in 0..dh {
            let sy = (dy * sh) / dh;
            for dx in 0..dw {
                let sx = (dx * sw) / dw;
                let bit = (icon.and_mask[sy * src_and_stride + sx / 8] >> (7 - (sx % 8))) & 1;
                if bit != 0 {
                    new_and[dy * dst_and_stride + dx / 8] |= 1u8 << (7 - (dx % 8));
                }
            }
        }
    } else {
        for dy in 0..dh {
            for dx in 0..dw {
                if new_bgra[(dy * dw + dx) * 4 + 3] == 0 {
                    new_and[dy * dst_and_stride + dx / 8] |= 1u8 << (7 - (dx % 8));
                }
            }
        }
    }

    if icon.f_icon {
        icon.x_hotspot = dst_w / 2;
        icon.y_hotspot = dst_h / 2;
    } else {
        let scale_hot = |hot: u32, src_dim: u32, dst_dim: u32| -> u32 {
            let scaled =
                ((hot as u64) * (dst_dim as u64) + (src_dim as u64) / 2) / (src_dim as u64);
            (scaled as u32).min(dst_dim.saturating_sub(1))
        };
        icon.x_hotspot = scale_hot(icon.x_hotspot, src_w, dst_w);
        icon.y_hotspot = scale_hot(icon.y_hotspot, src_h, dst_h);
    }
    icon.width = dst_w;
    icon.height = dst_h;
    icon.bgra_pixels = new_bgra;
    icon.and_mask = new_and;
}

/// Parse an `RT_GROUP_CURSOR` directory (`NEWHEADER` + `RESDIR` entries, 14 bytes each).
fn parse_group_cursor_dir(bytes: &[u8]) -> Vec<GrpIconDirEntry> {
    if bytes.len() < 6 {
        return Vec::new();
    }
    let res_type = u16::from_le_bytes([bytes[2], bytes[3]]);
    let count = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
    if res_type != 2 || count == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(count.min(64));
    for i in 0..count {
        let off = 6 + i * 14;
        if off + 14 > bytes.len() {
            break;
        }
        // CURSORRESDIR: wWidth (u16), wHeight (u16; often 2*height for monochrome),
        // wPlanes (u16), wBitCount (u16), dwBytesInRes (u32), nID (u16).
        let w = u16::from_le_bytes([bytes[off], bytes[off + 1]]) as u32;
        let raw_h = u16::from_le_bytes([bytes[off + 2], bytes[off + 3]]) as u32;
        let h = if raw_h == w * 2 && w > 0 { w } else { raw_h.max(1) };
        let bpp = u16::from_le_bytes([bytes[off + 6], bytes[off + 7]]);
        let icon_id = u16::from_le_bytes([bytes[off + 12], bytes[off + 13]]);
        out.push(GrpIconDirEntry {
            width: w.max(1),
            height: h,
            bpp: if bpp == 0 { 1 } else { bpp },
            icon_id,
        });
    }
    out
}

fn load_cursor_impl(inst: usize, name: *const u16, ansi: bool, cx: i32, cy: i32, flags: u32) -> usize {
    let Some(res_id) = (unsafe { icon_res_id(name, ansi) }) else {
        set_error(e::INVALID_PARAMETER);
        return 0;
    };

    if inst == 0 {
        if let IconResId::Int(id) = res_id {
            if let Some(shape) = system_shape(id) {
                let h = SYSTEM_BASE + id as usize;
                cursors().lock().unwrap().entry(h).or_insert(Cursor { shape });
                return h;
            }
        }
        set_error(e::RESOURCE_NAME_NOT_FOUND);
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
    let shared = (flags & LR_SHARED) != 0;
    let cache_key = SharedPeResourceKey {
        inst,
        is_icon: false,
        res_id: SharedResKeyId::from(&res_id),
        want_w,
        want_h,
    };
    if shared {
        if let Some(&cached) = shared_pe_resources().lock().unwrap().get(&cache_key) {
            if cursors().lock().unwrap().contains_key(&cached) {
                return cached;
            }
        }
    }

    let Some(pe) = (unsafe { PeImageView::from_hinstance(inst) }) else {
        set_error(e::RESOURCE_TYPE_NOT_FOUND);
        return 0;
    };

    let Some(group_bytes) = (unsafe { pe.find_resource_data(RT_GROUP_CURSOR, &res_id) }) else {
        set_error(e::RESOURCE_NAME_NOT_FOUND);
        return 0;
    };
    let entries = parse_group_cursor_dir(group_bytes);
    if entries.is_empty() {
        set_error(1812); // ERROR_RESOURCE_DATA_NOT_FOUND
        return 0;
    }

    let ranked = rank_group_icon_entries(&entries, want_w, want_h);
    for entry in ranked {
        let Some(cur_bytes) = (unsafe { pe.find_resource_data(RT_CURSOR, &IconResId::Int(entry.icon_id)) }) else {
            continue;
        };
        // RT_CURSOR payload starts with 4-byte LOCALHEADER { WORD xHotSpot; WORD yHotSpot; }
        // followed by a DIB (BITMAPINFOHEADER + XOR/AND masks) or PNG image.
        if cur_bytes.len() < 8 {
            continue;
        }
        let x_hot = u16::from_le_bytes([cur_bytes[0], cur_bytes[1]]) as u32;
        let y_hot = u16::from_le_bytes([cur_bytes[2], cur_bytes[3]]) as u32;
        if let Some(mut icon_data) = decode_icon_resource_bytes(
            &cur_bytes[4..],
            false,
            entry.width,
            entry.height,
            entry.bpp,
            shared,
        ) {
            icon_data.x_hotspot = x_hot;
            icon_data.y_hotspot = y_hot;
            let target_w = if want_w > 0 { want_w } else { icon_data.width };
            let target_h = if want_h > 0 { want_h } else { icon_data.height };
            resize_icon_data(&mut icon_data, target_w, target_h);
            let shape = shape_from_icon_data(&icon_data);
            let final_w = icon_data.width;
            let final_h = icon_data.height;
            let final_x_hot = icon_data.x_hotspot;
            let final_y_hot = icon_data.y_hotspot;
            let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
            cursors().lock().unwrap().insert(hc, Cursor { shape });
            icons().lock().unwrap().insert(hc, icon_data);
            if shared {
                let winner = {
                    let mut cache = shared_pe_resources().lock().unwrap();
                    match cache.entry(cache_key) {
                        std::collections::hash_map::Entry::Occupied(e) => *e.get(),
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert(hc);
                            hc
                        }
                    }
                };
                if winner != hc {
                    cursors().lock().unwrap().remove(&hc);
                    icons().lock().unwrap().remove(&hc);
                    return winner;
                }
            }
            log::debug!(
                "user32: loaded PE .rsrc cursor from {inst:#x} ({final_w}x{final_h} hot=({final_x_hot},{final_y_hot})) -> {hc:#x}",
            );
            return hc;
        }
    }

    set_error(1812); // ERROR_RESOURCE_DATA_NOT_FOUND
    0
}

pub unsafe extern "win64" fn LoadCursorA(inst: HINSTANCE, name: LPCSTR) -> usize {
    load_cursor_impl(inst as usize, name as *const u16, true, 32, 32, LR_DEFAULTSIZE | LR_SHARED)
}

pub unsafe extern "win64" fn LoadCursorW(inst: HINSTANCE, name: LPCWSTR) -> usize {
    load_cursor_impl(inst as usize, name, false, 32, 32, LR_DEFAULTSIZE | LR_SHARED)
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

/// Decoded icon representation handed to the native UI backend for `STATIC` (`SS_ICON`) controls.
#[derive(Clone, Debug)]
pub enum IconPayload {
    /// Standard Windows system icon ID (`IDI_APPLICATION = 32512`..=`IDI_SHIELD = 32518`).
    Stock(u16),
    /// Raw PNG bytes from a Vista+ icon resource.
    Png(Vec<u8>),
    /// Decoded 32bpp top-down BGRA pixels (`width * height * 4`).
    Bgra { width: u32, height: u32, pixels: Vec<u8> },
}

/// Snapshot an `HICON` handle for native rendering (e.g. `SS_ICON` / `STM_SETICON`).
pub fn icon_payload(hicon: usize) -> Option<IconPayload> {
    if hicon == 0 {
        return None;
    }
    if hicon >= SYSTEM_ICON_BASE && hicon < SYSTEM_ICON_BASE + 0x8000 {
        let id = (hicon - SYSTEM_ICON_BASE) as u16;
        if is_stock_icon_id(id) {
            return Some(IconPayload::Stock(id));
        }
    }
    let tbl = icons().lock().unwrap_or_else(|e| e.into_inner());
    let data = tbl.get(&hicon)?;
    if data.is_png && !data.raw_data.is_empty() {
        Some(IconPayload::Png(data.raw_data.clone()))
    } else if data.width > 0 && data.height > 0 && data.bgra_pixels.len() >= (data.width as usize) * (data.height as usize) * 4 {
        Some(IconPayload::Bgra {
            width: data.width,
            height: data.height,
            pixels: data.bgra_pixels.clone(),
        })
    } else {
        None
    }
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

/// Decode a PNG image buffer into straight BGRA8 pixels and a 1bpp DDB AND mask
/// via Apple ImageIO + CoreGraphics (works on macOS and iOS, thread-safe).
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn decode_png_to_bgra(bytes: &[u8]) -> Option<(u32, u32, Vec<u8>, Vec<u8>)> {
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGPoint {
        x: f64,
        y: f64,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGSize {
        width: f64,
        height: f64,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGRect {
        origin: CGPoint,
        size: CGSize,
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFDataCreate(allocator: *const c_void, bytes: *const u8, length: isize) -> *const c_void;
        fn CFRelease(cf: *const c_void);
    }
    #[link(name = "ImageIO", kind = "framework")]
    extern "C" {
        fn CGImageSourceCreateWithData(data: *const c_void, options: *const c_void) -> *mut c_void;
        fn CGImageSourceCreateImageAtIndex(
            isrc: *mut c_void,
            index: usize,
            options: *const c_void,
        ) -> *mut c_void;
    }
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGImageGetWidth(img: *mut c_void) -> usize;
        fn CGImageGetHeight(img: *mut c_void) -> usize;
        fn CGImageRelease(img: *mut c_void);
        fn CGColorSpaceCreateDeviceRGB() -> *mut c_void;
        fn CGColorSpaceRelease(cs: *mut c_void);
        fn CGBitmapContextCreate(
            data: *mut c_void,
            width: usize,
            height: usize,
            bits_per_component: usize,
            bytes_per_row: usize,
            space: *mut c_void,
            bitmap_info: u32,
        ) -> *mut c_void;
        fn CGContextDrawImage(c: *mut c_void, rect: CGRect, image: *mut c_void);
        fn CGContextRelease(c: *mut c_void);
    }

    unsafe {
        let cf_data = CFDataCreate(std::ptr::null(), bytes.as_ptr(), bytes.len() as isize);
        if cf_data.is_null() {
            return None;
        }
        let src = CGImageSourceCreateWithData(cf_data, std::ptr::null());
        CFRelease(cf_data);
        if src.is_null() {
            return None;
        }
        let img = CGImageSourceCreateImageAtIndex(src, 0, std::ptr::null());
        CFRelease(src as *const c_void);
        if img.is_null() {
            return None;
        }
        let w = CGImageGetWidth(img);
        let h = CGImageGetHeight(img);
        if w == 0 || h == 0 || w > 4096 || h > 4096 {
            CGImageRelease(img);
            return None;
        }
        let byte_len = w.checked_mul(h)?.checked_mul(4)?;
        let mut bgra = vec![0u8; byte_len];
        let cs = CGColorSpaceCreateDeviceRGB();
        if cs.is_null() {
            CGImageRelease(img);
            return None;
        }
        // kCGBitmapByteOrder32Little (0x2000) | kCGImageAlphaPremultipliedFirst (2) -> BGRA8 premultiplied
        let bitmap_info: u32 = 0x2000 | 2;
        let ctx = CGBitmapContextCreate(
            bgra.as_mut_ptr() as *mut c_void,
            w,
            h,
            8,
            w * 4,
            cs,
            bitmap_info,
        );
        CGColorSpaceRelease(cs);
        if ctx.is_null() {
            CGImageRelease(img);
            return None;
        }
        CGContextDrawImage(
            ctx,
            CGRect {
                origin: CGPoint { x: 0.0, y: 0.0 },
                size: CGSize { width: w as f64, height: h as f64 },
            },
            img,
        );
        CGContextRelease(ctx);
        CGImageRelease(img);

        let ddb_and_stride = ((w + 15) & !15) / 8;
        let mut and_mask = vec![0u8; ddb_and_stride * h];
        for y in 0..h {
            for x in 0..w {
                let p = (y * w + x) * 4;
                let a = bgra[p + 3];
                if a == 0 {
                    bgra[p] = 0;
                    bgra[p + 1] = 0;
                    bgra[p + 2] = 0;
                    and_mask[y * ddb_and_stride + x / 8] |= 1u8 << (7 - (x % 8));
                } else if a < 255 {
                    let a32 = a as u32;
                    bgra[p] = (((bgra[p] as u32) * 255 + a32 / 2) / a32).min(255) as u8;
                    bgra[p + 1] = (((bgra[p + 1] as u32) * 255 + a32 / 2) / a32).min(255) as u8;
                    bgra[p + 2] = (((bgra[p + 2] as u32) * 255 + a32 / 2) / a32).min(255) as u8;
                }
            }
        }
        Some((w as u32, h as u32, bgra, and_mask))
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn decode_png_to_bgra(_bytes: &[u8]) -> Option<(u32, u32, Vec<u8>, Vec<u8>)> {
    None
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
    // PNG-encoded icon/cursor (Vista+ 256x256 icons or PNG-backed RT_CURSOR resources).
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
        let Some((dec_w, dec_h, bgra_pixels, and_mask)) = decode_png_to_bgra(bytes) else {
            log::warn!(
                "user32: failed to decode PNG {} resource ({width}x{height}, {} bytes)",
                if f_icon { "icon" } else { "cursor" },
                bytes.len(),
            );
            return None;
        };
        return Some(IconData {
            f_icon,
            x_hotspot: dec_w / 2,
            y_hotspot: dec_h / 2,
            width: dec_w,
            height: dec_h,
            bpp,
            is_png: true,
            raw_data: bytes.to_vec(),
            bgra_pixels,
            and_mask,
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
                        if and_bit != 0 {
                            if idx != 0 {
                                // AND=1, XOR=1: Win32 screen-invert pixel; render as visible
                                // opaque black (matching `decode_monochrome_cursor_bgra`) so
                                // invert-only cursors do not become `Shape::Blank`.
                                [0, 0, 0, 0xFF]
                            } else {
                                [0, 0, 0, 0]
                            }
                        } else {
                            let po = palette_off + idx * 4;
                            if idx < palette_entries && po + 4 <= bytes.len() {
                                [bytes[po], bytes[po + 1], bytes[po + 2], 0xFF]
                            } else if idx != 0 {
                                [0xFF, 0xFF, 0xFF, 0xFF]
                            } else {
                                [0, 0, 0, 0xFF]
                            }
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
    let shared = (flags & LR_SHARED) != 0;
    let cache_key = SharedPeResourceKey {
        inst,
        is_icon: true,
        res_id: SharedResKeyId::from(&res_id),
        want_w,
        want_h,
    };
    if shared {
        if let Some(&cached) = shared_pe_resources().lock().unwrap().get(&cache_key) {
            if icons().lock().unwrap().contains_key(&cached) {
                return cached;
            }
        }
    }

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
    for entry in ranked {
        if let Some(icon_bytes) = unsafe { pe.find_resource_data(RT_ICON, &IconResId::Int(entry.icon_id)) } {
            if let Some(mut icon_data) = decode_icon_resource_bytes(
                icon_bytes,
                true,
                entry.width,
                entry.height,
                entry.bpp,
                shared,
            ) {
                let target_w = if want_w > 0 { want_w } else { icon_data.width };
                let target_h = if want_h > 0 { want_h } else { icon_data.height };
                resize_icon_data(&mut icon_data, target_w, target_h);
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
                if shared {
                    let winner = {
                        let mut cache = shared_pe_resources().lock().unwrap();
                        match cache.entry(cache_key) {
                            std::collections::hash_map::Entry::Occupied(e) => *e.get(),
                            std::collections::hash_map::Entry::Vacant(v) => {
                                v.insert(handle);
                                handle
                            }
                        }
                    };
                    if winner != handle {
                        icons().lock().unwrap().remove(&handle);
                        return winner;
                    }
                }
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

fn destroy_icon_or_cursor(h: usize) -> BOOL {
    if h == 0 {
        set_error(e::INVALID_CURSOR_HANDLE);
        return FALSE;
    }
    if (SYSTEM_BASE..SYSTEM_BASE + 0x10000).contains(&h) {
        return TRUE; // shared system cursors are never destroyed
    }
    let mut found = false;
    {
        let mut tbl = icons().lock().unwrap();
        if let Some(icon) = tbl.get(&h) {
            if icon.shared {
                return TRUE; // shared icons/cursors are never destroyed
            }
            tbl.remove(&h);
            found = true;
        }
    }
    if cursors().lock().unwrap().remove(&h).is_some() {
        found = true;
    }
    if found {
        TRUE
    } else {
        set_error(e::INVALID_CURSOR_HANDLE);
        FALSE
    }
}

pub unsafe extern "win64" fn DestroyIcon(h: usize) -> BOOL {
    destroy_icon_or_cursor(h)
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
    let shape = shape_from_icon_data(&icon);
    let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
    cursors().lock().unwrap().insert(hc, Cursor { shape });
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
    let maybe_icon = icons().lock().unwrap().get(&hicon).cloned();
    let maybe_shape = cursors()
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
    if maybe_icon.is_none() && maybe_shape.is_none() {
        set_error(e::INVALID_CURSOR_HANDLE);
        return 0;
    }
    let handle = if maybe_shape.is_some() {
        NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed)
    } else {
        NEXT_ICON.fetch_add(4, Ordering::Relaxed)
    };
    if let Some(shape) = maybe_shape {
        cursors().lock().unwrap().insert(handle, Cursor { shape });
    }
    if let Some(mut icon) = maybe_icon {
        icon.shared = false;
        icons().lock().unwrap().insert(handle, icon);
    }
    handle
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
    let shared = (flags & LR_SHARED) != 0;
    let mut icon = decode_icon_resource_bytes(slice, f_icon != 0, fw, fh, 32, shared)
        .unwrap_or_else(|| {
            let mut m = make_solid_icon(fw, fh, shared);
            m.f_icon = f_icon != 0;
            m.raw_data = slice.to_vec();
            m
        });
    let target_w = if want_w > 0 { want_w } else { icon.width };
    let target_h = if want_h > 0 { want_h } else { icon.height };
    resize_icon_data(&mut icon, target_w, target_h);
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
        1 => load_icon_impl(inst as usize, name, false, cx, cy, flags),   // IMAGE_ICON
        2 => load_cursor_impl(inst as usize, name, false, cx, cy, flags), // IMAGE_CURSOR
        _ => {
            let icon = make_solid_icon(cx.max(1) as u32, cy.max(1) as u32, (flags & LR_SHARED) != 0);
            let shape = shape_from_icon_data(&icon);
            let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
            cursors().lock().unwrap().insert(hc, Cursor { shape });
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
        2 => load_cursor_impl(inst as usize, name as *const u16, true, cx, cy, flags),
        _ => {
            let icon = make_solid_icon(cx.max(1) as u32, cy.max(1) as u32, (flags & LR_SHARED) != 0);
            let shape = shape_from_icon_data(&icon);
            let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
            cursors().lock().unwrap().insert(hc, Cursor { shape });
            hc
        }
    }
}

pub unsafe extern "win64" fn CopyImage(h: usize, _ty: u32, _cx: i32, _cy: i32, _flags: u32) -> usize {
    CopyIcon(h)
}

/// Decode Win32 monochrome AND/XOR cursor planes (MSB-first, 16-bit row stride)
/// into top-down BGRA8 pixels.
///
/// Win32 monochrome cursor truth table:
/// * `AND=0, XOR=0` -> opaque black `(0, 0, 0, 255)`
/// * `AND=0, XOR=1` -> opaque white `(255, 255, 255, 255)`
/// * `AND=1, XOR=0` -> transparent `(0, 0, 0, 0)`
/// * `AND=1, XOR=1` -> screen-invert (rendered as opaque black on macOS `NSCursor`,
///   which has no hardware XOR cursor mode, matching Wine's `macdrv`).
fn decode_monochrome_cursor_bgra(w: usize, h: usize, and_plane: &[u8], xor_plane: &[u8]) -> Vec<u8> {
    let stride = ((w + 15) / 16) * 2;
    let mut bgra = vec![0u8; w * h * 4];
    for y in 0..h {
        let row = y * stride;
        for x in 0..w {
            let byte_idx = row + (x / 8);
            let shift = 7 - (x % 8);
            let and_bit = (and_plane[byte_idx] >> shift) & 1;
            let xor_bit = (xor_plane[byte_idx] >> shift) & 1;
            let px: [u8; 4] = match (and_bit, xor_bit) {
                (0, 0) => [0, 0, 0, 0xFF],       // black
                (0, 1) => [0xFF, 0xFF, 0xFF, 0xFF], // white
                (1, 0) => [0, 0, 0, 0],          // transparent
                _ => [0, 0, 0, 0xFF],            // invert fallback -> opaque black
            };
            let dst = (y * w + x) * 4;
            bgra[dst..dst + 4].copy_from_slice(&px);
        }
    }
    bgra
}

pub unsafe extern "win64" fn CreateCursor(
    _inst: HINSTANCE,
    xhot: i32,
    yhot: i32,
    w: i32,
    h: i32,
    and: *const u8,
    xor: *const u8,
) -> usize {
    if w <= 0 || h <= 0 || and.is_null() || xor.is_null() {
        set_error(e::INVALID_PARAMETER);
        return 0;
    }
    // Monochrome masks, rows padded to 16 bits.
    let wu = w as usize;
    let hu = h as usize;
    let stride = ((wu + 15) / 16) * 2;
    let n = stride * hu;
    let a = std::slice::from_raw_parts(and, n);
    let x = std::slice::from_raw_parts(xor, n);
    let blank = a.iter().all(|&b| b == 0xFF) && x.iter().all(|&b| b == 0);
    let bgra = decode_monochrome_cursor_bgra(wu, hu, a, x);
    let x_hotspot = xhot.clamp(0, w - 1) as u32;
    let y_hotspot = yhot.clamp(0, h - 1) as u32;
    let icon = IconData {
        f_icon: false,
        x_hotspot,
        y_hotspot,
        width: w as u32,
        height: h as u32,
        bpp: 1,
        is_png: false,
        raw_data: Vec::new(),
        bgra_pixels: bgra,
        and_mask: a.to_vec(),
        shared: false,
    };
    let shape = if blank {
        Shape::Blank
    } else {
        shape_from_icon_data(&icon)
    };
    let hc = NEXT_CUSTOM.fetch_add(4, Ordering::Relaxed);
    cursors().lock().unwrap().insert(hc, Cursor { shape });
    icons().lock().unwrap().insert(hc, icon);
    log::debug!(
        "CreateCursor {w}x{h} hot=({x_hotspot},{y_hotspot}) -> {hc:#x} ({})",
        if blank { "blank" } else { "custom" }
    );
    hc
}

pub unsafe extern "win64" fn DestroyCursor(h: usize) -> BOOL {
    destroy_icon_or_cursor(h)
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

pub fn has_clip_rect() -> bool {
    CLIP.lock()
        .unwrap()
        .is_some_and(|r| r.right > r.left && r.bottom > r.top)
}

/// Recompute the native cursor state (visibility, shape, relative mode).
pub fn update_state() {
    let shape = current_shape();
    let hidden = SHOW_COUNT.load(Ordering::Acquire) < 0 || matches!(shape, None | Some(Shape::Blank));
    let ours = super::input::app_active() && super::input::active_window() != 0;
    let in_client = crate::macos::fullscreen_takeover()
        || crate::macos::dxgi_fullscreen_active()
        || crate::macos::mouse_in_client();
    let can_capture = !crate::macos::cursor_capture_disallowed()
        && (crate::macos::lock_cursor_allowed() || (has_clip_rect() && in_client));
    let relative = hidden && ours && super::input::raw_mouse_registered() && can_capture;
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
    let now_active = v.is_some_and(|rc| rc.right > rc.left && rc.bottom > rc.top);
    let prev_active = {
        let mut guard = CLIP.lock().unwrap();
        let prev = guard.is_some_and(|rc| rc.right > rc.left && rc.bottom > rc.top);
        *guard = v;
        prev
    };
    log::debug!("ClipCursor({v:?})");
    if prev_active != now_active {
        update_state();
    }
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

    #[test]
    fn relative_mode_follows_dxgi_fullscreen_clip_and_override() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        crate::macos::with_lock_cursor_override(None, || unsafe {
            // Save baseline state.
            let prev_show = SHOW_COUNT.load(Ordering::Acquire);
            let prev_active = super::super::input::app_active();
            let prev_win = super::super::input::active_window();
            let prev_dxgi = crate::macos::dxgi_fullscreen_active();
            let prev_in_client = crate::macos::mouse_in_client();

            let dev_add = super::super::input::RAWINPUTDEVICE {
                usage_page: 1,
                usage: 2,
                flags: 0,
                target: 0,
            };
            let dev_rem = super::super::input::RAWINPUTDEVICE {
                usage_page: 1,
                usage: 2,
                flags: 1, // RIDEV_REMOVE
                target: 0,
            };
            assert_eq!(super::super::input::RegisterRawInputDevices(&dev_add, 1, 16), TRUE);
            super::super::input::set_active_window(0x1000, true);
            SHOW_COUNT.store(-1, Ordering::Release);
            crate::macos::set_dxgi_fullscreen_active(false);
            crate::macos::set_mouse_in_client(true);
            ClipCursor(std::ptr::null());
            update_state();

            // 1. Framed windowed mode without ClipCursor: relative mode stays off.
            assert!(!relative_mode());
            assert!(!crate::macos::lock_cursor_allowed());

            // 2. Clipped windowed gameplay with pointer inside client area: relative mode ON.
            let rect = RECT { left: 0, top: 0, right: 1280, bottom: 720 };
            assert_eq!(ClipCursor(&rect), TRUE);
            assert!(relative_mode());

            // 3. Pointer moves outside client area (e.g. title bar): relative mode turns off.
            crate::macos::set_mouse_in_client(false);
            update_state();
            assert!(!relative_mode());

            // 4. Unclipping in windowed mode keeps relative mode off even inside client.
            crate::macos::set_mouse_in_client(true);
            assert_eq!(ClipCursor(std::ptr::null()), TRUE);
            assert!(!relative_mode());

            // 5. Runtime DXGI borderless fullscreen enables lock_cursor_allowed + relative mode.
            crate::macos::set_dxgi_fullscreen_active(true);
            update_state();
            assert!(crate::macos::lock_cursor_allowed());
            assert!(relative_mode());

            // 6. Explicit disable override (PORT_LOCK_CURSOR=0 / AW2_LOCK_CURSOR=0) suppresses both.
            crate::macos::with_lock_cursor_override(Some(false), || {
                update_state();
                assert!(!crate::macos::lock_cursor_allowed());
                assert!(!relative_mode());
            });

            // 7. Deactivating the window (e.g. Cmd+Tab) immediately releases relative mode.
            super::super::input::native_deactivated(0x1000);
            assert!(!relative_mode());

            // Restore baseline state.
            crate::macos::set_dxgi_fullscreen_active(prev_dxgi);
            crate::macos::set_mouse_in_client(prev_in_client);
            ClipCursor(std::ptr::null());
            SHOW_COUNT.store(prev_show, Ordering::Release);
            if prev_active && prev_win != 0 {
                super::super::input::set_active_window(prev_win, true);
            }
            let _ = super::super::input::RegisterRawInputDevices(&dev_rem, 1, 16);
            update_state();
        });
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

    fn build_valid_png_rgba(w: u32, h: u32, rgba: [u8; 4]) -> Vec<u8> {
        fn crc32(data: &[u8]) -> u32 {
            let mut crc = 0xFFFF_FFFFu32;
            for &b in data {
                crc ^= b as u32;
                for _ in 0..8 {
                    crc = if (crc & 1) != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
                }
            }
            !crc
        }
        fn push_chunk(out: &mut Vec<u8>, tag: &[u8; 4], payload: &[u8]) {
            out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
            let start = out.len();
            out.extend_from_slice(tag);
            out.extend_from_slice(payload);
            let crc = crc32(&out[start..]);
            out.extend_from_slice(&crc.to_be_bytes());
        }
        // Build raw PNG scanlines (filter byte 0 + RGBA pixels per row) and deflate via system libz.
        #[link(name = "z")]
        extern "C" {
            fn compress(
                dest: *mut u8,
                dest_len: *mut libc::c_ulong,
                source: *const u8,
                source_len: libc::c_ulong,
            ) -> i32;
        }
        let mut raw = Vec::with_capacity(h as usize * (1 + w as usize * 4));
        for _ in 0..h {
            raw.push(0); // filter = None
            for _ in 0..w {
                raw.extend_from_slice(&rgba);
            }
        }
        let mut zlen: libc::c_ulong = (raw.len() + raw.len() / 100 + 64) as libc::c_ulong;
        let mut zbuf = vec![0u8; zlen as usize];
        let rc = unsafe { compress(zbuf.as_mut_ptr(), &mut zlen, raw.as_ptr(), raw.len() as libc::c_ulong) };
        assert_eq!(rc, 0, "libz compress failed");
        zbuf.truncate(zlen as usize);

        let mut ihdr = Vec::with_capacity(13);
        ihdr.extend_from_slice(&w.to_be_bytes());
        ihdr.extend_from_slice(&h.to_be_bytes());
        ihdr.push(8); // bit depth
        ihdr.push(6); // RGBA
        ihdr.extend_from_slice(&[0, 0, 0]);

        let mut out = Vec::new();
        out.extend_from_slice(b"\x89PNG\r\n\x1a\n");
        push_chunk(&mut out, b"IHDR", &ihdr);
        push_chunk(&mut out, b"IDAT", &zbuf);
        push_chunk(&mut out, b"IEND", &[]);
        out
    }

    fn build_png_stub(w: u32, h: u32) -> Vec<u8> {
        build_valid_png_rgba(w, h, [0x44, 0x88, 0xCC, 0xFF])
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
            //    LoadIconA uses LR_DEFAULTSIZE (32x32), resizing the 16x16-only MAINICON to 32x32;
            //    LoadImageA with (0, 0, 0) keeps the resource's native 16x16 dimensions.
            let h_named = LoadIconA(hinst, c"mainicon".as_ptr() as LPCSTR);
            assert_ne!(h_named, 0);
            assert_eq!(GetIconInfo(h_named, &mut ii), TRUE);
            assert_eq!(crate::misc::gdi32::GetObjectW(ii.hbm_color, 32, bm.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm[4..8].try_into().unwrap()), 32);
            assert_eq!(i32::from_le_bytes(bm[8..12].try_into().unwrap()), 32);
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);
            assert_eq!(DestroyIcon(h_named), TRUE);

            let h_named_native = LoadImageA(hinst, c"mainicon".as_ptr() as LPCSTR, 1, 0, 0, 0);
            assert_ne!(h_named_native, 0);
            assert_eq!(GetIconInfo(h_named_native, &mut ii), TRUE);
            assert_eq!(crate::misc::gdi32::GetObjectW(ii.hbm_color, 32, bm.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm[4..8].try_into().unwrap()), 16);
            assert_eq!(i32::from_le_bytes(bm[8..12].try_into().unwrap()), 16);
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);
            assert_eq!(DestroyIcon(h_named_native), TRUE);

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

    #[test]
    fn custom_bitmap_cursor_and_pe_cursor_resource() {
        let _g = super::super::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe {
            // 1. Monochrome CreateCursor: pixel (0,0) opaque black (AND=0,XOR=0),
            //    (1,0) opaque white (AND=0,XOR=1), (2,0) transparent (AND=1,XOR=0),
            //    (3,0) invert->black (AND=1,XOR=1).
            let mut and_plane = [0xFFu8; 4]; // 2x2 bytes for 4x2 (16-bit stride = 2)
            let mut xor_plane = [0x00u8; 4];
            // Row 0 MSB..: bit7=(0,0), bit6=(1,0), bit5=(2,0), bit4=(3,0)
            and_plane[0] = 0b0011_1111;
            xor_plane[0] = 0b0101_0000;
            let hc = CreateCursor(std::ptr::null_mut(), 1, 0, 4, 2, and_plane.as_ptr(), xor_plane.as_ptr());
            assert_ne!(hc, 0);
            let shape = cursors().lock().unwrap().get(&hc).unwrap().shape.clone();
            match shape {
                Shape::Custom(img) => {
                    assert_eq!((img.width, img.height, img.hotspot_x, img.hotspot_y), (4, 2, 1, 0));
                    assert_eq!(&img.rgba[0..4], &[0, 0, 0, 255]); // black
                    assert_eq!(&img.rgba[4..8], &[255, 255, 255, 255]); // white
                    assert_eq!(&img.rgba[8..12], &[0, 0, 0, 0]); // transparent
                    assert_eq!(&img.rgba[12..16], &[0, 0, 0, 255]); // invert -> black
                }
                other => panic!("expected Shape::Custom, got {other:?}"),
            }
            let mut ii = std::mem::zeroed::<ICONINFO>();
            assert_eq!(GetIconInfo(hc, &mut ii), TRUE);
            assert_eq!(ii.f_icon, FALSE);
            assert_eq!((ii.x_hotspot, ii.y_hotspot), (1, 0));
            crate::misc::gdi32::DeleteObject(ii.hbm_color);
            crate::misc::gdi32::DeleteObject(ii.hbm_mask);

            // CopyIcon / CopyImage on a CreateCursor handle must not deadlock and must copy
            // both the cursor shape and backing icon data into an independent non-shared handle.
            let hc_copy = CopyIcon(hc);
            assert_ne!(hc_copy, 0);
            assert_ne!(hc_copy, hc);
            assert!(cursors().lock().unwrap().contains_key(&hc_copy));
            assert!(icons().lock().unwrap().contains_key(&hc_copy));
            let mut ii_copy = std::mem::zeroed::<ICONINFO>();
            assert_eq!(GetIconInfo(hc_copy, &mut ii_copy), TRUE);
            assert_eq!((ii_copy.f_icon, ii_copy.x_hotspot, ii_copy.y_hotspot), (FALSE, 1, 0));
            crate::misc::gdi32::DeleteObject(ii_copy.hbm_color);
            crate::misc::gdi32::DeleteObject(ii_copy.hbm_mask);

            // DestroyCursor must release both cursors() and icons() for non-shared handles.
            assert_eq!(DestroyCursor(hc), TRUE);
            assert!(!cursors().lock().unwrap().contains_key(&hc));
            assert!(!icons().lock().unwrap().contains_key(&hc));
            assert_eq!(GetIconInfo(hc, &mut ii), FALSE);
            assert_eq!(DestroyCursor(hc), FALSE);

            // Copied cursor remains valid until destroyed, then releases both tables.
            assert!(cursors().lock().unwrap().contains_key(&hc_copy));
            assert!(icons().lock().unwrap().contains_key(&hc_copy));
            assert_eq!(DestroyCursor(hc_copy), TRUE);
            assert!(!cursors().lock().unwrap().contains_key(&hc_copy));
            assert!(!icons().lock().unwrap().contains_key(&hc_copy));

            // 2. Unknown system cursor ID returns 0 + ERROR_RESOURCE_NAME_NOT_FOUND.
            assert_eq!(LoadCursorW(std::ptr::null_mut(), 55555 as LPCWSTR), 0);
            assert_eq!(crate::teb::last_error(), e::RESOURCE_NAME_NOT_FOUND);

            // 3. Synthetic PE with RT_GROUP_CURSOR (12) + RT_CURSOR (1) containing hotspot (3,5) + 16x16 DIB.
            let mut img = vec![0u8; 0x3000];
            const RSRC_RVA: usize = 0x1000;
            img[0..2].copy_from_slice(&0x5A4Du16.to_le_bytes());
            img[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
            img[0x80..0x84].copy_from_slice(&0x0000_4550u32.to_le_bytes());
            img[0x84 + 16..0x84 + 18].copy_from_slice(&0xF0u16.to_le_bytes());
            let opt = 0x84 + 20;
            let img_len = img.len() as u32;
            img[opt..opt + 2].copy_from_slice(&0x20Bu16.to_le_bytes());
            img[opt + 56..opt + 60].copy_from_slice(&img_len.to_le_bytes());
            img[opt + 108..opt + 112].copy_from_slice(&16u32.to_le_bytes());
            img[opt + 112 + 16..opt + 112 + 20].copy_from_slice(&(RSRC_RVA as u32).to_le_bytes());
            img[opt + 112 + 20..opt + 112 + 24].copy_from_slice(&0x2000u32.to_le_bytes());

            let write_dir = |buf: &mut [u8], off: usize, entries: &[(u32, u32)]| {
                let b = RSRC_RVA + off;
                buf[b + 14..b + 16].copy_from_slice(&(entries.len() as u16).to_le_bytes());
                for (i, &(k, v)) in entries.iter().enumerate() {
                    let e = b + 16 + i * 8;
                    buf[e..e + 4].copy_from_slice(&k.to_le_bytes());
                    buf[e + 4..e + 8].copy_from_slice(&v.to_le_bytes());
                }
            };
            let mut dib = Vec::new();
            dib.extend_from_slice(&40u32.to_le_bytes());
            dib.extend_from_slice(&16i32.to_le_bytes());
            dib.extend_from_slice(&32i32.to_le_bytes()); // biHeight = 2*16
            dib.extend_from_slice(&1u16.to_le_bytes());
            dib.extend_from_slice(&32u16.to_le_bytes());
            dib.extend_from_slice(&[0u8; 24]);
            for _ in 0..(16 * 16) {
                dib.extend_from_slice(&[0x10, 0x20, 0x30, 0xFF]);
            }
            dib.extend_from_slice(&vec![0u8; 16 * 4]); // AND mask (32-bit padded rows)
            let mut cur_res = Vec::with_capacity(4 + dib.len());
            cur_res.extend_from_slice(&3u16.to_le_bytes()); // xHotSpot
            cur_res.extend_from_slice(&5u16.to_le_bytes()); // yHotSpot
            cur_res.extend_from_slice(&dib);
            let mut grp_res = Vec::new();
            grp_res.extend_from_slice(&0u16.to_le_bytes());
            grp_res.extend_from_slice(&2u16.to_le_bytes()); // RES_CURSOR
            grp_res.extend_from_slice(&1u16.to_le_bytes());
            grp_res.extend_from_slice(&16u16.to_le_bytes()); // wWidth
            grp_res.extend_from_slice(&32u16.to_le_bytes()); // wHeight (2*16)
            grp_res.extend_from_slice(&1u16.to_le_bytes());
            grp_res.extend_from_slice(&32u16.to_le_bytes());
            grp_res.extend_from_slice(&(cur_res.len() as u32).to_le_bytes());
            grp_res.extend_from_slice(&7u16.to_le_bytes()); // RT_CURSOR #7

            let cur_off = 0x300usize;
            img[RSRC_RVA + cur_off..RSRC_RVA + cur_off + cur_res.len()].copy_from_slice(&cur_res);
            let grp_off = 0xA00usize;
            img[RSRC_RVA + grp_off..RSRC_RVA + grp_off + grp_res.len()].copy_from_slice(&grp_res);

            write_dir(&mut img, 0x000, &[(RT_CURSOR as u32, 0x8000_0040), (RT_GROUP_CURSOR as u32, 0x8000_0080)]);
            write_dir(&mut img, 0x040, &[(7, 0x8000_0100)]);
            write_dir(&mut img, 0x080, &[(202, 0x8000_0120)]);
            write_dir(&mut img, 0x100, &[(0x0409, 0x200)]);
            write_dir(&mut img, 0x120, &[(0x0409, 0x210)]);
            img[RSRC_RVA + 0x200..RSRC_RVA + 0x204].copy_from_slice(&((RSRC_RVA + cur_off) as u32).to_le_bytes());
            img[RSRC_RVA + 0x204..RSRC_RVA + 0x208].copy_from_slice(&(cur_res.len() as u32).to_le_bytes());
            img[RSRC_RVA + 0x210..RSRC_RVA + 0x214].copy_from_slice(&((RSRC_RVA + grp_off) as u32).to_le_bytes());
            img[RSRC_RVA + 0x214..RSRC_RVA + 0x218].copy_from_slice(&(grp_res.len() as u32).to_le_bytes());

            let h_pe_cur = LoadCursorW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR);
            assert_ne!(h_pe_cur, 0);
            let cur_count_after_first = cursors().lock().unwrap().len();
            let icon_count_after_first = icons().lock().unwrap().len();
            // Repeated shared loads (LoadCursorW / LoadCursorA / LoadImageW with LR_SHARED) must
            // return the exact same cached handle without growing the cursor/icon tables.
            assert_eq!(LoadCursorW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR), h_pe_cur);
            assert_eq!(LoadCursorA(img.as_ptr() as HINSTANCE, 202 as LPCSTR), h_pe_cur);
            assert_eq!(
                LoadImageW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 32, 32, LR_DEFAULTSIZE | LR_SHARED),
                h_pe_cur
            );
            assert_eq!(cursors().lock().unwrap().len(), cur_count_after_first);
            assert_eq!(icons().lock().unwrap().len(), icon_count_after_first);
            // Non-shared LoadImageW allocates a distinct handle that DestroyCursor frees cleanly
            // without affecting the cached shared cursor.
            let h_nonshared_cur = LoadImageW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 32, 32, 0);
            assert_ne!(h_nonshared_cur, 0);
            assert_ne!(h_nonshared_cur, h_pe_cur);
            assert_eq!(DestroyCursor(h_nonshared_cur), TRUE);
            assert!(!cursors().lock().unwrap().contains_key(&h_nonshared_cur));
            assert!(!icons().lock().unwrap().contains_key(&h_nonshared_cur));

            // LoadCursorW requests LR_DEFAULTSIZE (32x32), scaling the 16x16 resource and its (3,5) hotspot to (6,10).
            match cursors().lock().unwrap().get(&h_pe_cur).unwrap().shape.clone() {
                Shape::Custom(ci) => {
                    assert_eq!((ci.width, ci.height, ci.hotspot_x, ci.hotspot_y), (32, 32, 6, 10));
                    assert_eq!(ci.rgba.len(), 32 * 32 * 4);
                    // BGRA [0x10, 0x20, 0x30, 0xFF] -> RGBA [0x30, 0x20, 0x10, 0xFF]
                    assert_eq!(&ci.rgba[0..4], &[0x30, 0x20, 0x10, 0xFF]);
                }
                other => panic!("expected Shape::Custom from RT_CURSOR, got {other:?}"),
            }
            // LoadImageW with (0, 0, 0) keeps native 16x16 dimensions and (3,5) hotspot.
            let h_native_cur = LoadImageW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 0, 0, 0);
            assert_ne!(h_native_cur, 0);
            match cursors().lock().unwrap().get(&h_native_cur).unwrap().shape.clone() {
                Shape::Custom(ci) => {
                    assert_eq!((ci.width, ci.height, ci.hotspot_x, ci.hotspot_y), (16, 16, 3, 5));
                    assert_eq!(ci.rgba.len(), 16 * 16 * 4);
                    assert_eq!(&ci.rgba[0..4], &[0x30, 0x20, 0x10, 0xFF]);
                }
                other => panic!("expected native 16x16 Shape::Custom, got {other:?}"),
            }
            assert_eq!(DestroyCursor(h_native_cur), TRUE);

            // LoadImageW requesting DPI-scaled 64x64 resizes pixels/mask and scales hotspot (3,5) -> (12,20).
            let h_dpi64_cur = LoadImageW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 64, 64, 0);
            assert_ne!(h_dpi64_cur, 0);
            match cursors().lock().unwrap().get(&h_dpi64_cur).unwrap().shape.clone() {
                Shape::Custom(ci) => {
                    assert_eq!((ci.width, ci.height, ci.hotspot_x, ci.hotspot_y), (64, 64, 12, 20));
                    assert_eq!(ci.rgba.len(), 64 * 64 * 4);
                    assert_eq!(&ci.rgba[0..4], &[0x30, 0x20, 0x10, 0xFF]);
                }
                other => panic!("expected 64x64 Shape::Custom, got {other:?}"),
            }
            let mut ii64 = std::mem::zeroed::<ICONINFO>();
            assert_eq!(GetIconInfo(h_dpi64_cur, &mut ii64), TRUE);
            assert_eq!((ii64.f_icon, ii64.x_hotspot, ii64.y_hotspot), (FALSE, 12, 20));
            let mut bm64 = [0u8; 32];
            assert_eq!(crate::misc::gdi32::GetObjectW(ii64.hbm_color, 32, bm64.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm64[4..8].try_into().unwrap()), 64);
            assert_eq!(i32::from_le_bytes(bm64[8..12].try_into().unwrap()), 64);
            assert_eq!(crate::misc::gdi32::GetObjectW(ii64.hbm_mask, 32, bm64.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm64[4..8].try_into().unwrap()), 64);
            assert_eq!(i32::from_le_bytes(bm64[8..12].try_into().unwrap()), 64);
            crate::misc::gdi32::DeleteObject(ii64.hbm_color);
            crate::misc::gdi32::DeleteObject(ii64.hbm_mask);
            assert_eq!(DestroyCursor(h_dpi64_cur), TRUE);

            // Downscaling 16x16 -> 8x8 scales hotspot (3,5) -> (2,3).
            let h_down8_cur = LoadImageW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 8, 8, 0);
            assert_ne!(h_down8_cur, 0);
            match cursors().lock().unwrap().get(&h_down8_cur).unwrap().shape.clone() {
                Shape::Custom(ci) => {
                    assert_eq!((ci.width, ci.height, ci.hotspot_x, ci.hotspot_y), (8, 8, 2, 3));
                    assert_eq!(ci.rgba.len(), 8 * 8 * 4);
                }
                other => panic!("expected 8x8 Shape::Custom, got {other:?}"),
            }
            assert_eq!(DestroyCursor(h_down8_cur), TRUE);

            assert_eq!(DestroyCursor(h_pe_cur), TRUE);
            // Shared cursor remains cached after DestroyCursor.
            assert_eq!(LoadCursorW(img.as_ptr() as HINSTANCE, 202 as LPCWSTR), h_pe_cur);

            // 4. Monochrome 1bpp PE RT_CURSOR: preserve visible invert (AND=1, XOR=1) pixels
            //    so invert-only cursors do not become Shape::Blank, and verify 4-state parity + resize.
            let build_mono_dib = |w: u32, h: u32, and_rows_top_down: &[u8], xor_rows_top_down: &[u8]| -> Vec<u8> {
                let mut d = Vec::new();
                d.extend_from_slice(&40u32.to_le_bytes());
                d.extend_from_slice(&(w as i32).to_le_bytes());
                d.extend_from_slice(&((h * 2) as i32).to_le_bytes()); // bottom-up XOR + AND
                d.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
                d.extend_from_slice(&1u16.to_le_bytes()); // biBitCount = 1bpp
                d.extend_from_slice(&[0u8; 24]);
                // 2-entry palette: 0=black, 1=white
                d.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
                d.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0x00]);
                let dib_stride = ((w as usize + 31) & !31) / 8;
                for y in (0..h as usize).rev() {
                    let mut row = vec![0u8; dib_stride];
                    row[0] = xor_rows_top_down[y];
                    d.extend_from_slice(&row);
                }
                for y in (0..h as usize).rev() {
                    let mut row = vec![0u8; dib_stride];
                    row[0] = and_rows_top_down[y];
                    d.extend_from_slice(&row);
                }
                d
            };
            // 4a. Invert-only cursor: all pixels have AND=1, XOR=1 -> must be Shape::Custom (not Shape::Blank).
            let inv_dib = build_mono_dib(4, 2, &[0xFF, 0xFF], &[0xF0, 0xF0]);
            let mut inv_cur_res = vec![1, 0, 0, 0]; // hotspot (1, 0)
            inv_cur_res.extend_from_slice(&inv_dib);
            let mut inv_img = img.clone();
            inv_img[RSRC_RVA + cur_off..RSRC_RVA + cur_off + inv_cur_res.len()].copy_from_slice(&inv_cur_res);
            inv_img[RSRC_RVA + 0x204..RSRC_RVA + 0x208].copy_from_slice(&(inv_cur_res.len() as u32).to_le_bytes());
            let h_inv = LoadImageW(inv_img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 0, 0, 0);
            assert_ne!(h_inv, 0);
            match cursors().lock().unwrap().get(&h_inv).unwrap().shape.clone() {
                Shape::Custom(ci) => {
                    assert_eq!((ci.width, ci.height, ci.hotspot_x, ci.hotspot_y), (4, 2, 1, 0));
                    for px in ci.rgba.chunks_exact(4) {
                        assert_eq!(px, &[0, 0, 0, 255], "AND=1,XOR=1 invert pixel must be visible opaque black");
                    }
                }
                other => panic!("invert-only monochrome cursor must not become Blank, got {other:?}"),
            }
            assert_eq!(DestroyCursor(h_inv), TRUE);

            // 4b. Mixed 4-state monochrome PE cursor: (0,0)=black, (1,0)=white, (2,0)=transparent, (3,0)=invert->black.
            //     Also verify upscaling to 8x4 preserves exact 4-state pixels and resamples AND mask.
            let mix_dib = build_mono_dib(4, 2, &[0b0011_1111, 0xFF], &[0b0101_0000, 0x00]);
            let mut mix_cur_res = vec![1, 0, 0, 0]; // hotspot (1, 0)
            mix_cur_res.extend_from_slice(&mix_dib);
            let mut mix_img = img.clone();
            mix_img[RSRC_RVA + cur_off..RSRC_RVA + cur_off + mix_cur_res.len()].copy_from_slice(&mix_cur_res);
            mix_img[RSRC_RVA + 0x204..RSRC_RVA + 0x208].copy_from_slice(&(mix_cur_res.len() as u32).to_le_bytes());
            let h_mix = LoadImageW(mix_img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 8, 4, 0);
            assert_ne!(h_mix, 0);
            match cursors().lock().unwrap().get(&h_mix).unwrap().shape.clone() {
                Shape::Custom(ci) => {
                    assert_eq!((ci.width, ci.height, ci.hotspot_x, ci.hotspot_y), (8, 4, 2, 0));
                    assert_eq!(&ci.rgba[0..4], &[0, 0, 0, 255]);       // (0,0) black
                    assert_eq!(&ci.rgba[8..12], &[255, 255, 255, 255]); // (2,0) white
                    assert_eq!(&ci.rgba[16..20], &[0, 0, 0, 0]);      // (4,0) transparent
                    assert_eq!(&ci.rgba[24..28], &[0, 0, 0, 255]);    // (6,0) invert -> black
                }
                other => panic!("expected resized 8x4 monochrome Shape::Custom, got {other:?}"),
            }
            // Verify resampled AND mask on 8x4 (stride 2 bytes): row 0 bits = 0000_1111 = 0x0F.
            let mix_and = icons().lock().unwrap().get(&h_mix).unwrap().and_mask.clone();
            assert_eq!(mix_and.len(), 2 * 4);
            assert_eq!(mix_and[0], 0b0000_1111);
            assert_eq!(DestroyCursor(h_mix), TRUE);

            // 4c. All-transparent monochrome PE cursor (AND=1, XOR=0) still produces Shape::Blank.
            let blank_dib = build_mono_dib(4, 2, &[0xFF, 0xFF], &[0x00, 0x00]);
            let mut blank_cur_res = vec![0, 0, 0, 0];
            blank_cur_res.extend_from_slice(&blank_dib);
            let mut blank_img = img.clone();
            blank_img[RSRC_RVA + cur_off..RSRC_RVA + cur_off + blank_cur_res.len()].copy_from_slice(&blank_cur_res);
            blank_img[RSRC_RVA + 0x204..RSRC_RVA + 0x208].copy_from_slice(&(blank_cur_res.len() as u32).to_le_bytes());
            let h_blank = LoadImageW(blank_img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 0, 0, 0);
            assert_ne!(h_blank, 0);
            assert_eq!(cursors().lock().unwrap().get(&h_blank).unwrap().shape, Shape::Blank);
            assert_eq!(DestroyCursor(h_blank), TRUE);

            // 5. PNG-backed RT_CURSOR must decode real pixels into Shape::Custom (never Shape::Blank),
            //    and a corrupt/undecodable PNG cursor resource must fail cleanly rather than returning a blank cursor.
            let png_bytes = build_valid_png_rgba(8, 8, [0x12, 0x34, 0x56, 0xFF]);
            let mut png_cur_res = Vec::with_capacity(4 + png_bytes.len());
            png_cur_res.extend_from_slice(&2u16.to_le_bytes()); // xHotSpot
            png_cur_res.extend_from_slice(&4u16.to_le_bytes()); // yHotSpot
            png_cur_res.extend_from_slice(&png_bytes);
            let mut png_img = img.clone();
            png_img[RSRC_RVA + cur_off..RSRC_RVA + cur_off + png_cur_res.len()].copy_from_slice(&png_cur_res);
            png_img[RSRC_RVA + 0x204..RSRC_RVA + 0x208].copy_from_slice(&(png_cur_res.len() as u32).to_le_bytes());
            let h_png_cur = LoadImageW(png_img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 0, 0, 0);
            assert_ne!(h_png_cur, 0);
            match cursors().lock().unwrap().get(&h_png_cur).unwrap().shape.clone() {
                Shape::Custom(ci) => {
                    assert_eq!((ci.width, ci.height, ci.hotspot_x, ci.hotspot_y), (8, 8, 2, 4));
                    assert_eq!(&ci.rgba[0..4], &[0x12, 0x34, 0x56, 0xFF]);
                }
                other => panic!("expected Shape::Custom for PNG RT_CURSOR, got {other:?}"),
            }
            assert_eq!(DestroyCursor(h_png_cur), TRUE);
            assert!(!cursors().lock().unwrap().contains_key(&h_png_cur));
            assert!(!icons().lock().unwrap().contains_key(&h_png_cur));

            // Truncated/corrupt PNG cursor resource must fail cleanly (return 0), never Shape::Blank.
            let mut bad_png_cur = vec![2, 0, 4, 0];
            bad_png_cur.extend_from_slice(&png_bytes[..28]); // header only, no IDAT/IEND
            let mut bad_img = img.clone();
            bad_img[RSRC_RVA + cur_off..RSRC_RVA + cur_off + bad_png_cur.len()].copy_from_slice(&bad_png_cur);
            bad_img[RSRC_RVA + 0x204..RSRC_RVA + 0x208].copy_from_slice(&(bad_png_cur.len() as u32).to_le_bytes());
            assert_eq!(LoadImageW(bad_img.as_ptr() as HINSTANCE, 202 as LPCWSTR, 2, 0, 0, 0), 0);
        }
    }
}
