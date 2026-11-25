//! gdi32.dll: in-memory 2D DIB rasterizer, GDI object tracking (DCs, DIB/DDB bitmaps,
//! brushes, pens, fonts, regions), `GetObject`, font metrics, and per-DC gamma ramps.

use crate::types::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_INVALID_PARAMETER: u32 = 87;
const CLR_INVALID: u32 = 0xFFFF_FFFF;

// Stock object indices
const WHITE_BRUSH: u32 = 0;
const LTGRAY_BRUSH: u32 = 1;
const GRAY_BRUSH: u32 = 2;
const DKGRAY_BRUSH: u32 = 3;
const BLACK_BRUSH: u32 = 4;
const NULL_BRUSH: u32 = 5;
const WHITE_PEN: u32 = 6;
const BLACK_PEN: u32 = 7;
const NULL_PEN: u32 = 8;
const OEM_FIXED_FONT: u32 = 10;
const ANSI_FIXED_FONT: u32 = 11;
const ANSI_VAR_FONT: u32 = 12;
const SYSTEM_FONT: u32 = 13;
const DEVICE_DEFAULT_FONT: u32 = 14;
const DEFAULT_PALETTE: u32 = 15;
const SYSTEM_FIXED_FONT: u32 = 16;
const DEFAULT_GUI_FONT: u32 = 17;
const DC_BRUSH: u32 = 18;
const DC_PEN: u32 = 19;
/// Internal sentinel stock handle representing a memory DC's default 1x1 bitmap.
const DEFAULT_STOCK_BITMAP: u32 = 20;

// Brush / Pen styles
const BS_SOLID: u32 = 0;
const BS_NULL: u32 = 1;
const PS_SOLID: u32 = 0;
const PS_NULL: u32 = 5;

// GDI raster operations (ROPs)
const BLACKNESS: u32 = 0x0000_0042;
const NOTSRCCOPY: u32 = 0x0033_0008;
const DSTINVERT: u32 = 0x0055_0009;
const PATINVERT: u32 = 0x005A_0049;
const SRCINVERT: u32 = 0x0066_0046;
const SRCAND: u32 = 0x0088_00C6;
const SRCCOPY: u32 = 0x00CC_0020;
const SRCPAINT: u32 = 0x00EE_0086;
const PATCOPY: u32 = 0x00F0_0021;
const WHITENESS: u32 = 0x00FF_0062;

// GetObjectType return values
const OBJ_PEN: u32 = 1;
const OBJ_BRUSH: u32 = 2;
const OBJ_DC: u32 = 3;
const OBJ_PAL: u32 = 5;
const OBJ_FONT: u32 = 6;
const OBJ_BITMAP: u32 = 7;
const OBJ_REGION: u32 = 8;
const OBJ_MEMDC: u32 = 10;

// LOGFONT pitch bits
const FIXED_PITCH: u8 = 1;

#[derive(Clone, Debug)]
struct FontData {
    height: i32,
    width: i32,
    escapement: i32,
    orientation: i32,
    weight: i32,
    italic: u8,
    underline: u8,
    strike_out: u8,
    charset: u8,
    out_precision: u8,
    clip_precision: u8,
    quality: u8,
    pitch_and_family: u8,
    face: String,
}

impl FontData {
    fn is_monospace(&self) -> bool {
        if (self.pitch_and_family & 0x03) == FIXED_PITCH {
            return true;
        }
        let lower = self.face.to_ascii_lowercase();
        lower.contains("courier")
            || lower.contains("consolas")
            || lower.contains("mono")
            || lower.contains("fixed")
            || lower.contains("terminal")
            || lower.contains("menlo")
    }
}

#[derive(Clone, Debug)]
struct BrushData {
    style: u32,
    color: u32,
    hatch: usize,
}

#[derive(Clone, Debug)]
struct PenData {
    style: u32,
    width: i32,
    color: u32,
}

#[derive(Clone, Debug)]
struct BitmapData {
    width: usize,
    height: usize,
    bpp: u16,
    planes: u16,
    top_down: bool,
    is_dib: bool,
    bits_ptr: usize,
    stride: usize,
    byte_len: usize,
}

impl BitmapData {
    fn row_offset(&self, y: usize) -> Option<usize> {
        if y >= self.height || self.bits_ptr == 0 {
            return None;
        }
        let phys_row = if self.top_down { y } else { self.height - 1 - y };
        let off = phys_row.checked_mul(self.stride)?;
        (off < self.byte_len).then_some(off)
    }

    unsafe fn get_rgba(&self, x: i32, y: i32) -> Option<[u8; 4]> {
        if x < 0 || y < 0 {
            return None;
        }
        let ux = x as usize;
        let uy = y as usize;
        if ux >= self.width || uy >= self.height {
            return None;
        }
        let row = self.row_offset(uy)?;
        let base = self.bits_ptr as *const u8;
        match self.bpp {
            32 => {
                let off = row + ux * 4;
                if off + 4 > self.byte_len {
                    return None;
                }
                let p = base.add(off);
                let b = p.read_unaligned();
                let g = p.add(1).read_unaligned();
                let r = p.add(2).read_unaligned();
                let a = p.add(3).read_unaligned();
                Some([r, g, b, a])
            }
            24 => {
                let off = row + ux * 3;
                if off + 3 > self.byte_len {
                    return None;
                }
                let p = base.add(off);
                let b = p.read_unaligned();
                let g = p.add(1).read_unaligned();
                let r = p.add(2).read_unaligned();
                Some([r, g, b, 0xFF])
            }
            16 => {
                let off = row + ux * 2;
                if off + 2 > self.byte_len {
                    return None;
                }
                let v = (base.add(off) as *const u16).read_unaligned();
                let r = (((v >> 10) & 0x1F) * 255 / 31) as u8;
                let g = (((v >> 5) & 0x1F) * 255 / 31) as u8;
                let b = ((v & 0x1F) * 255 / 31) as u8;
                Some([r, g, b, 0xFF])
            }
            8 => {
                let off = row + ux;
                if off >= self.byte_len {
                    return None;
                }
                let v = base.add(off).read_unaligned();
                Some([v, v, v, 0xFF])
            }
            1 => {
                let off = row + ux / 8;
                if off >= self.byte_len {
                    return None;
                }
                let bit = (base.add(off).read_unaligned() >> (7 - (ux % 8))) & 1;
                let v = if bit != 0 { 0xFF } else { 0 };
                Some([v, v, v, 0xFF])
            }
            _ => None,
        }
    }

    unsafe fn set_rgba(&self, x: i32, y: i32, rgba: [u8; 4]) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let ux = x as usize;
        let uy = y as usize;
        if ux >= self.width || uy >= self.height {
            return false;
        }
        let Some(row) = self.row_offset(uy) else {
            return false;
        };
        let base = self.bits_ptr as *mut u8;
        match self.bpp {
            32 => {
                let off = row + ux * 4;
                if off + 4 > self.byte_len {
                    return false;
                }
                let p = base.add(off);
                p.write_unaligned(rgba[2]);
                p.add(1).write_unaligned(rgba[1]);
                p.add(2).write_unaligned(rgba[0]);
                p.add(3).write_unaligned(rgba[3]);
                true
            }
            24 => {
                let off = row + ux * 3;
                if off + 3 > self.byte_len {
                    return false;
                }
                let p = base.add(off);
                p.write_unaligned(rgba[2]);
                p.add(1).write_unaligned(rgba[1]);
                p.add(2).write_unaligned(rgba[0]);
                true
            }
            16 => {
                let off = row + ux * 2;
                if off + 2 > self.byte_len {
                    return false;
                }
                let r5 = (rgba[0] as u16) >> 3;
                let g5 = (rgba[1] as u16) >> 3;
                let b5 = (rgba[2] as u16) >> 3;
                let v = (r5 << 10) | (g5 << 5) | b5;
                (base.add(off) as *mut u16).write_unaligned(v);
                true
            }
            8 => {
                let off = row + ux;
                if off >= self.byte_len {
                    return false;
                }
                let lum = ((rgba[0] as u32 * 77 + rgba[1] as u32 * 150 + rgba[2] as u32 * 29) >> 8) as u8;
                base.add(off).write_unaligned(lum);
                true
            }
            1 => {
                let off = row + ux / 8;
                if off >= self.byte_len {
                    return false;
                }
                let mask = 1u8 << (7 - (ux % 8));
                let cur = base.add(off).read_unaligned();
                let lit = (rgba[0] | rgba[1] | rgba[2]) >= 128;
                let next = if lit { cur | mask } else { cur & !mask };
                base.add(off).write_unaligned(next);
                true
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
struct DcData {
    window: usize,
    font: usize,
    bitmap: usize,
    brush: usize,
    pen: usize,
    text_color: u32,
    bk_color: u32,
    bk_mode: i32,
    gamma_ramp: Option<Box<[u16; 768]>>,
}

impl DcData {
    fn new(window: usize) -> Self {
        Self {
            window,
            font: stock_handle(SYSTEM_FONT),
            bitmap: stock_handle(DEFAULT_STOCK_BITMAP),
            brush: stock_handle(WHITE_BRUSH),
            pen: stock_handle(BLACK_PEN),
            text_color: 0x0000_0000,
            bk_color: 0x00FF_FFFF,
            bk_mode: 2, // OPAQUE
            gamma_ramp: None,
        }
    }
}

#[derive(Clone, Debug)]
enum Obj {
    Dc(DcData),
    Font(FontData),
    Brush(BrushData),
    Pen(PenData),
    Bitmap(BitmapData),
    Region,
}

const STOCK_BASE: usize = 0x0090_0000;
const WINDOW_DC_BASE: usize = 0x0300_0000;
static NEXT: AtomicUsize = AtomicUsize::new(0x0400_0000);

fn objs() -> &'static Mutex<HashMap<usize, Obj>> {
    static M: OnceLock<Mutex<HashMap<usize, Obj>>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(HashMap::new()))
}

fn stock_handle(i: u32) -> usize {
    STOCK_BASE + 4 * i as usize
}

fn is_stock(h: usize) -> bool {
    (STOCK_BASE..STOCK_BASE + 4 * 32).contains(&h) && ((h - STOCK_BASE) % 4 == 0)
}

fn stock_index(h: usize) -> Option<u32> {
    is_stock(h).then_some(((h - STOCK_BASE) / 4) as u32)
}

fn stock_brush(idx: u32) -> Option<BrushData> {
    let (style, color) = match idx {
        WHITE_BRUSH | DC_BRUSH => (BS_SOLID, 0x00FF_FFFF),
        LTGRAY_BRUSH => (BS_SOLID, 0x00C0_C0C0),
        GRAY_BRUSH => (BS_SOLID, 0x0080_8080),
        DKGRAY_BRUSH => (BS_SOLID, 0x0040_4040),
        BLACK_BRUSH => (BS_SOLID, 0x0000_0000),
        NULL_BRUSH => (BS_NULL, 0x0000_0000),
        _ => return None,
    };
    Some(BrushData { style, color, hatch: 0 })
}

fn stock_pen(idx: u32) -> Option<PenData> {
    let (style, width, color) = match idx {
        WHITE_PEN => (PS_SOLID, 1, 0x00FF_FFFF),
        BLACK_PEN | DC_PEN => (PS_SOLID, 1, 0x0000_0000),
        NULL_PEN => (PS_NULL, 0, 0x0000_0000),
        _ => return None,
    };
    Some(PenData { style, width, color })
}

fn stock_font(idx: u32) -> Option<FontData> {
    let (height, pitch, face) = match idx {
        OEM_FIXED_FONT => (12, FIXED_PITCH, "Terminal"),
        ANSI_FIXED_FONT => (12, FIXED_PITCH, "Courier"),
        ANSI_VAR_FONT => (12, 2, "MS Sans Serif"),
        SYSTEM_FONT | DEVICE_DEFAULT_FONT => (16, 2, "System"),
        SYSTEM_FIXED_FONT => (16, FIXED_PITCH, "Fixedsys"),
        DEFAULT_GUI_FONT => (12, 2, "Segoe UI"),
        _ => return None,
    };
    Some(FontData {
        height,
        width: 0,
        escapement: 0,
        orientation: 0,
        weight: 400,
        italic: 0,
        underline: 0,
        strike_out: 0,
        charset: 0,
        out_precision: 0,
        clip_precision: 0,
        quality: 0,
        pitch_and_family: pitch,
        face: face.to_string(),
    })
}

fn resolve_brush(m: &HashMap<usize, Obj>, h: usize) -> BrushData {
    if let Some(idx) = stock_index(h) {
        if let Some(b) = stock_brush(idx) {
            return b;
        }
    }
    match m.get(&h) {
        Some(Obj::Brush(b)) => b.clone(),
        _ => BrushData { style: BS_SOLID, color: 0x00FF_FFFF, hatch: 0 },
    }
}

fn resolve_pen(m: &HashMap<usize, Obj>, h: usize) -> PenData {
    if let Some(idx) = stock_index(h) {
        if let Some(p) = stock_pen(idx) {
            return p;
        }
    }
    match m.get(&h) {
        Some(Obj::Pen(p)) => p.clone(),
        _ => PenData { style: PS_SOLID, width: 1, color: 0x0000_0000 },
    }
}

fn resolve_font(m: &HashMap<usize, Obj>, h: usize) -> FontData {
    if let Some(idx) = stock_index(h) {
        if let Some(f) = stock_font(idx) {
            return f;
        }
    }
    match m.get(&h) {
        Some(Obj::Font(f)) => f.clone(),
        _ => stock_font(SYSTEM_FONT).unwrap(),
    }
}

fn colorref_to_rgba(c: u32) -> [u8; 4] {
    [(c & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, ((c >> 16) & 0xFF) as u8, 0xFF]
}

fn rgba_to_colorref(rgba: [u8; 4]) -> u32 {
    (rgba[0] as u32) | ((rgba[1] as u32) << 8) | ((rgba[2] as u32) << 16)
}

fn dib_stride(width: usize, bpp: u16) -> usize {
    ((width.saturating_mul(bpp as usize).saturating_add(31)) & !31) / 8
}

/// The (shared, never freed) DC of a window, used by user32 GetDC/BeginPaint.
pub fn window_dc(hwnd: usize) -> usize {
    let h = WINDOW_DC_BASE | (hwnd & 0x00FF_FFFC);
    objs().lock().unwrap().entry(h).or_insert_with(|| Obj::Dc(DcData::new(hwnd)));
    h
}

pub unsafe extern "win64" fn GetStockObject(i: i32) -> usize {
    // WHITE_BRUSH..DC_PEN (0..=19), 9 is unused.
    if !(0..=19).contains(&i) || i == 9 {
        return 0;
    }
    stock_handle(i as u32)
}

pub unsafe extern "win64" fn CreateCompatibleDC(_hdc: usize) -> usize {
    let h = NEXT.fetch_add(4, Ordering::Relaxed);
    objs().lock().unwrap().insert(h, Obj::Dc(DcData::new(0)));
    h
}

pub unsafe extern "win64" fn DeleteDC(hdc: usize) -> BOOL {
    let mut m = objs().lock().unwrap();
    match m.get(&hdc) {
        Some(Obj::Dc(dc)) if dc.window == 0 => {
            m.remove(&hdc);
            TRUE
        }
        _ => FALSE,
    }
}

fn insert_font(f: FontData) -> usize {
    let h = NEXT.fetch_add(4, Ordering::Relaxed);
    log::debug!("CreateFont {:?} height {} -> {h:#x}", f.face, f.height);
    objs().lock().unwrap().insert(h, Obj::Font(f));
    h
}

#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn CreateFontA(
    height: i32,
    width: i32,
    esc: i32,
    orient: i32,
    weight: i32,
    italic: u32,
    underline: u32,
    strike: u32,
    charset: usize,
    out_prec: usize,
    clip_prec: usize,
    quality: usize,
    pitch: usize,
    face: LPCSTR,
) -> usize {
    insert_font(FontData {
        height,
        width,
        escapement: esc,
        orientation: orient,
        weight,
        italic: (italic != 0) as u8,
        underline: (underline != 0) as u8,
        strike_out: (strike != 0) as u8,
        charset: charset as u8,
        out_precision: out_prec as u8,
        clip_precision: clip_prec as u8,
        quality: quality as u8,
        pitch_and_family: pitch as u8,
        face: crate::strings::from_ansi_opt(face).unwrap_or_default(),
    })
}

#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn CreateFontW(
    height: i32,
    width: i32,
    esc: i32,
    orient: i32,
    weight: i32,
    italic: u32,
    underline: u32,
    strike: u32,
    charset: usize,
    out_prec: usize,
    clip_prec: usize,
    quality: usize,
    pitch: usize,
    face: LPCWSTR,
) -> usize {
    insert_font(FontData {
        height,
        width,
        escapement: esc,
        orientation: orient,
        weight,
        italic: (italic != 0) as u8,
        underline: (underline != 0) as u8,
        strike_out: (strike != 0) as u8,
        charset: charset as u8,
        out_precision: out_prec as u8,
        clip_precision: clip_prec as u8,
        quality: quality as u8,
        pitch_and_family: pitch as u8,
        face: crate::strings::from_wide_opt(face).unwrap_or_default(),
    })
}

/// LOGFONTA: fields @0..28, lfFaceName CHAR[32] @28 (60 bytes total).
pub unsafe extern "win64" fn CreateFontIndirectA(lf: *const u8) -> usize {
    if lf.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let mut face_buf = [0u8; 32];
    std::ptr::copy_nonoverlapping(lf.add(28), face_buf.as_mut_ptr(), 32);
    let n = face_buf.iter().position(|&c| c == 0).unwrap_or(32);
    insert_font(FontData {
        height: (lf.add(0) as *const i32).read_unaligned(),
        width: (lf.add(4) as *const i32).read_unaligned(),
        escapement: (lf.add(8) as *const i32).read_unaligned(),
        orientation: (lf.add(12) as *const i32).read_unaligned(),
        weight: (lf.add(16) as *const i32).read_unaligned(),
        italic: lf.add(20).read_unaligned(),
        underline: lf.add(21).read_unaligned(),
        strike_out: lf.add(22).read_unaligned(),
        charset: lf.add(23).read_unaligned(),
        out_precision: lf.add(24).read_unaligned(),
        clip_precision: lf.add(25).read_unaligned(),
        quality: lf.add(26).read_unaligned(),
        pitch_and_family: lf.add(27).read_unaligned(),
        face: String::from_utf8_lossy(&face_buf[..n]).into_owned(),
    })
}

/// LOGFONTW: fields @0..28, lfFaceName WCHAR[32] @28 (92 bytes total).
pub unsafe extern "win64" fn CreateFontIndirectW(lf: *const u8) -> usize {
    if lf.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let mut face_u16 = [0u16; 32];
    for i in 0..32usize {
        face_u16[i] = (lf.add(28 + i * 2) as *const u16).read_unaligned();
    }
    let n = face_u16.iter().position(|&c| c == 0).unwrap_or(32);
    insert_font(FontData {
        height: (lf.add(0) as *const i32).read_unaligned(),
        width: (lf.add(4) as *const i32).read_unaligned(),
        escapement: (lf.add(8) as *const i32).read_unaligned(),
        orientation: (lf.add(12) as *const i32).read_unaligned(),
        weight: (lf.add(16) as *const i32).read_unaligned(),
        italic: lf.add(20).read_unaligned(),
        underline: lf.add(21).read_unaligned(),
        strike_out: lf.add(22).read_unaligned(),
        charset: lf.add(23).read_unaligned(),
        out_precision: lf.add(24).read_unaligned(),
        clip_precision: lf.add(25).read_unaligned(),
        quality: lf.add(26).read_unaligned(),
        pitch_and_family: lf.add(27).read_unaligned(),
        face: String::from_utf16_lossy(&face_u16[..n]),
    })
}

pub unsafe extern "win64" fn CreateSolidBrush(color: u32) -> usize {
    let h = NEXT.fetch_add(4, Ordering::Relaxed);
    objs()
        .lock()
        .unwrap()
        .insert(h, Obj::Brush(BrushData { style: BS_SOLID, color, hatch: 0 }));
    h
}

pub unsafe extern "win64" fn CreatePen(style: i32, width: i32, color: u32) -> usize {
    let h = NEXT.fetch_add(4, Ordering::Relaxed);
    objs()
        .lock()
        .unwrap()
        .insert(h, Obj::Pen(PenData { style: style as u32, width, color }));
    h
}

/// BITMAPINFOHEADER: biSize @0, biWidth @4 (i32), biHeight @8 (i32), biPlanes @12 (u16), biBitCount @14 (u16).
pub unsafe extern "win64" fn CreateDIBSection(
    _hdc: usize,
    pbmi: *const u8,
    _usage: u32,
    ppv_bits: *mut *mut std::ffi::c_void,
    _section: HANDLE,
    _offset: u32,
) -> usize {
    if !ppv_bits.is_null() {
        ppv_bits.write_unaligned(std::ptr::null_mut());
    }
    if pbmi.is_null() {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let raw_w = (pbmi.add(4) as *const i32).read_unaligned();
    let raw_h = (pbmi.add(8) as *const i32).read_unaligned();
    let width = raw_w.unsigned_abs() as usize;
    let height = raw_h.unsigned_abs() as usize;
    if width == 0 || height == 0 {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return 0;
    }
    let planes = (pbmi.add(12) as *const u16).read_unaligned().max(1);
    let raw_bpp = (pbmi.add(14) as *const u16).read_unaligned();
    let bpp = if raw_bpp == 0 { 32 } else { raw_bpp };
    let top_down = raw_h < 0;
    let stride = dib_stride(width, bpp);
    let total = stride.saturating_mul(height).max(4);
    let bits = crate::heap::alloc(total, true);
    if bits.is_null() {
        return 0;
    }
    if !ppv_bits.is_null() {
        ppv_bits.write_unaligned(bits);
    }
    let h = NEXT.fetch_add(4, Ordering::Relaxed);
    objs().lock().unwrap().insert(
        h,
        Obj::Bitmap(BitmapData {
            width,
            height,
            bpp,
            planes,
            top_down,
            is_dib: true,
            bits_ptr: bits as usize,
            stride,
            byte_len: total,
        }),
    );
    h
}

pub unsafe extern "win64" fn DeleteObject(h: usize) -> BOOL {
    if is_stock(h) {
        return TRUE; // deleting a stock object is a harmless no-op
    }
    let mut m = objs().lock().unwrap();
    match m.remove(&h) {
        Some(Obj::Font(_) | Obj::Brush(_) | Obj::Pen(_) | Obj::Region) => TRUE,
        Some(Obj::Bitmap(bm)) => {
            drop(m);
            if bm.bits_ptr != 0 {
                crate::heap::free(bm.bits_ptr as *mut std::ffi::c_void);
            }
            TRUE
        }
        Some(dc @ Obj::Dc(_)) => {
            m.insert(h, dc);
            FALSE
        }
        None => FALSE,
    }
}

pub unsafe extern "win64" fn SelectObject(hdc: usize, obj: usize) -> usize {
    let mut m = objs().lock().unwrap();
    let kind = if let Some(idx) = stock_index(obj) {
        match idx {
            WHITE_BRUSH..=NULL_BRUSH | DC_BRUSH => OBJ_BRUSH,
            WHITE_PEN..=NULL_PEN | DC_PEN => OBJ_PEN,
            OEM_FIXED_FONT..=DEVICE_DEFAULT_FONT | SYSTEM_FIXED_FONT | DEFAULT_GUI_FONT => OBJ_FONT,
            DEFAULT_STOCK_BITMAP => OBJ_BITMAP,
            _ => 0,
        }
    } else {
        match m.get(&obj) {
            Some(Obj::Font(_)) => OBJ_FONT,
            Some(Obj::Brush(_)) => OBJ_BRUSH,
            Some(Obj::Pen(_)) => OBJ_PEN,
            Some(Obj::Bitmap(_)) => OBJ_BITMAP,
            Some(Obj::Region) => OBJ_REGION,
            _ => 0,
        }
    };
    if kind == 0 {
        return 0;
    }
    match m.get_mut(&hdc) {
        Some(Obj::Dc(dc)) => match kind {
            OBJ_FONT => std::mem::replace(&mut dc.font, obj),
            OBJ_BRUSH => std::mem::replace(&mut dc.brush, obj),
            OBJ_PEN => std::mem::replace(&mut dc.pen, obj),
            OBJ_BITMAP => std::mem::replace(&mut dc.bitmap, obj),
            OBJ_REGION => 2, // SIMPLEREGION
            _ => 0,
        },
        _ => 0,
    }
}

pub unsafe extern "win64" fn GetObjectType(h: usize) -> u32 {
    if let Some(idx) = stock_index(h) {
        return match idx {
            WHITE_BRUSH..=NULL_BRUSH | DC_BRUSH => OBJ_BRUSH,
            WHITE_PEN..=NULL_PEN | DC_PEN => OBJ_PEN,
            OEM_FIXED_FONT..=DEVICE_DEFAULT_FONT | SYSTEM_FIXED_FONT | DEFAULT_GUI_FONT => OBJ_FONT,
            DEFAULT_PALETTE => OBJ_PAL,
            DEFAULT_STOCK_BITMAP => OBJ_BITMAP,
            _ => 0,
        };
    }
    match objs().lock().unwrap().get(&h) {
        Some(Obj::Dc(dc)) if dc.window == 0 => OBJ_MEMDC,
        Some(Obj::Dc(_)) => OBJ_DC,
        Some(Obj::Font(_)) => OBJ_FONT,
        Some(Obj::Brush(_)) => OBJ_BRUSH,
        Some(Obj::Pen(_)) => OBJ_PEN,
        Some(Obj::Bitmap(_)) => OBJ_BITMAP,
        Some(Obj::Region) => OBJ_REGION,
        None => 0,
    }
}

#[repr(C)]
pub struct SIZE {
    pub cx: i32,
    pub cy: i32,
}

fn char_width_hundredths(ch: char, monospace: bool) -> i64 {
    if (ch as u32) < 0x20 {
        return 0;
    }
    if monospace {
        return 58;
    }
    match ch {
        'i' | 'l' | '!' | '|' | '.' | ',' | ':' | ';' | '\'' | '`' | 'I' => 28,
        'f' | 'j' | 'r' | 't' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' | '-' | ' ' => 34,
        'm' | 'w' => 75,
        'M' | 'W' => 85,
        'a'..='z' | '0'..='9' => 50,
        'A'..='Z' => 65,
        c if (c as u32) >= 0x1100 => 100, // CJK / fullwidth
        _ => 52,
    }
}

fn measure_text(font: &FontData, text: &str) -> SIZE {
    let height = if font.height != 0 { font.height.unsigned_abs() as i32 } else { 16 };
    if text.is_empty() {
        return SIZE { cx: 0, cy: height };
    }
    let mono = font.is_monospace();
    let mut total_h: i64 = 0;
    for ch in text.chars() {
        total_h += char_width_hundredths(ch, mono);
    }
    if font.weight >= 700 {
        total_h = (total_h * 108 + 50) / 100;
    }
    let cx = if font.width > 0 {
        let base_w = if mono { 58 } else { 50 };
        ((total_h * (font.width as i64) + (base_w / 2)) / base_w) as i32
    } else {
        ((total_h * (height as i64) + 50) / 100) as i32
    };
    SIZE { cx, cy: height }
}

pub unsafe extern "win64" fn GetTextExtentPoint32W(hdc: usize, s: LPCWSTR, n: i32, out: *mut SIZE) -> BOOL {
    if out.is_null() || (s.is_null() && n > 0) || n < 0 {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let font = {
        let m = objs().lock().unwrap();
        let Some(Obj::Dc(dc)) = m.get(&hdc) else {
            crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
            return FALSE;
        };
        resolve_font(&m, dc.font)
    };
    let mut units = Vec::with_capacity(n as usize);
    for i in 0..(n as usize) {
        units.push(s.add(i).read_unaligned());
    }
    let text = String::from_utf16_lossy(&units);
    out.write_unaligned(measure_text(&font, &text));
    TRUE
}

pub unsafe extern "win64" fn GetTextExtentPoint32A(hdc: usize, s: LPCSTR, n: i32, out: *mut SIZE) -> BOOL {
    if out.is_null() || (s.is_null() && n > 0) || n < 0 {
        crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let font = {
        let m = objs().lock().unwrap();
        let Some(Obj::Dc(dc)) = m.get(&hdc) else {
            crate::teb::set_last_error(ERROR_INVALID_PARAMETER);
            return FALSE;
        };
        resolve_font(&m, dc.font)
    };
    let bytes = if n > 0 { std::slice::from_raw_parts(s, n as usize) } else { &[] };
    let text = String::from_utf8_lossy(bytes);
    out.write_unaligned(measure_text(&font, &text));
    TRUE
}

pub unsafe extern "win64" fn GetDeviceCaps(hdc: usize, index: i32) -> i32 {
    let _ = hdc;
    let mon = crate::user32::monitor::primary_rect();
    let dpi = crate::user32::window::dpi().max(1) as i32;
    let refresh = crate::macos::display::monitors()
        .first()
        .map(|m| m.refresh_hz as i32)
        .unwrap_or(60);

    // Per Microsoft Learn GetDeviceCaps: HORZRES/VERTRES/HORZSIZE/VERTSIZE/BITSPIXEL/PLANES
    // describe the display device capabilities (inherited by compatible memory DCs), never
    // the currently selected bitmap object (which is queried via GetObject).
    let (w, h, bpp, planes) = (mon.width(), mon.height(), 32, 1);
    match index {
        0 => 0x4000,                                                 // DRIVERVERSION (4.0)
        2 => 1,                                                      // TECHNOLOGY (DT_RASDISPLAY)
        4 => ((w as f64) / (dpi as f64) * 25.4).round() as i32,      // HORZSIZE (mm)
        6 => ((h as f64) / (dpi as f64) * 25.4).round() as i32,      // VERTSIZE (mm)
        8 => w,                                                      // HORZRES
        10 => h,                                                     // VERTRES
        12 => bpp,                                                   // BITSPIXEL
        14 => planes,                                                // PLANES
        16 => -1,                                                    // NUMBRUSHES
        18 => -1,                                                    // NUMPENS
        20 => 0,                                                     // NUMMARKERS
        22 => 0,                                                     // NUMFONTS
        24 => if bpp <= 8 { 1 << bpp } else { -1 },                  // NUMCOLORS
        26 => 0,                                                     // PDEVICESIZE
        28 => 0x01FF,                                                // CURVECAPS
        30 => 0x00FE,                                                // LINECAPS
        32 => 0x00FF,                                                // POLYGONALCAPS
        34 => 0x7804,                                                // TEXTCAPS
        36 => 1,                                                     // CLIPCAPS (CP_RECTANGLE)
        38 => 0x7E99,                                                // RASTERCAPS
        40 => 36,                                                    // ASPECTX
        42 => 36,                                                    // ASPECTY
        44 => 51,                                                    // ASPECTXY
        45 => 3,                                                     // SHADEBLENDCAPS (SB_CONST_ALPHA|SB_PIXEL_ALPHA)
        88 | 90 => dpi,                                              // LOGPIXELSX / LOGPIXELSY
        104 => 0,                                                    // SIZEPALETTE
        106 => 20,                                                   // NUMRESERVED
        108 => bpp.min(24),                                          // COLORRES
        110..=115 => 0,                                              // PHYSICAL*, SCALINGFACTOR*
        116 => refresh,                                              // VREFRESH
        117 => mon.height(),                                         // DESKTOPVERTRES
        118 => mon.width(),                                          // DESKTOPHORZRES
        119 => 0,                                                    // BLTALIGNMENT
        121 => 0,                                                    // COLORMGMTCAPS (CM_NONE)
        _ => 0,                                                      // Windows returns 0 for unknown indices
    }
}

pub unsafe extern "win64" fn CreateCompatibleBitmap(_hdc: usize, cx: i32, cy: i32) -> usize {
    let w = cx.max(1) as usize;
    let h = cy.max(1) as usize;
    let stride = w.saturating_mul(4);
    let total = stride.saturating_mul(h).max(4);
    let bits = crate::heap::alloc(total, true);
    if bits.is_null() {
        return 0;
    }
    let handle = NEXT.fetch_add(4, Ordering::Relaxed);
    objs().lock().unwrap().insert(
        handle,
        Obj::Bitmap(BitmapData {
            width: w,
            height: h,
            bpp: 32,
            planes: 1,
            top_down: true,
            is_dib: false,
            bits_ptr: bits as usize,
            stride,
            byte_len: total,
        }),
    );
    handle
}

pub unsafe extern "win64" fn CreateICW(
    _driver: LPCWSTR,
    _device: LPCWSTR,
    _output: LPCWSTR,
    _init_data: *const u8,
) -> usize {
    CreateCompatibleDC(0)
}

pub unsafe extern "win64" fn AddFontMemResourceEx(
    _file_view: *const u8,
    _size: u32,
    _reserved: *const u8,
    num_fonts: *mut u32,
) -> usize {
    if !num_fonts.is_null() {
        num_fonts.write_unaligned(1);
    }
    NEXT.fetch_add(4, Ordering::Relaxed)
}

pub unsafe extern "win64" fn RemoveFontMemResourceEx(_handle: usize) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetTextColor(hdc: usize, color: u32) -> u32 {
    let mut m = objs().lock().unwrap();
    match m.get_mut(&hdc) {
        Some(Obj::Dc(dc)) => std::mem::replace(&mut dc.text_color, color),
        _ => CLR_INVALID,
    }
}

pub unsafe extern "win64" fn SetBkColor(hdc: usize, color: u32) -> u32 {
    let mut m = objs().lock().unwrap();
    match m.get_mut(&hdc) {
        Some(Obj::Dc(dc)) => std::mem::replace(&mut dc.bk_color, color),
        _ => CLR_INVALID,
    }
}

pub unsafe extern "win64" fn SetBkMode(hdc: usize, mode: i32) -> i32 {
    let mut m = objs().lock().unwrap();
    match m.get_mut(&hdc) {
        Some(Obj::Dc(dc)) => std::mem::replace(&mut dc.bk_mode, mode),
        _ => 0,
    }
}

pub unsafe extern "win64" fn TextOutW(
    _hdc: usize,
    _x: i32,
    _y: i32,
    _str: LPCWSTR,
    _count: i32,
) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn ChoosePixelFormat(_hdc: usize, _ppfd: *const u8) -> i32 {
    1
}

pub unsafe extern "win64" fn SetPixelFormat(_hdc: usize, _fmt: i32, _ppfd: *const u8) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SwapBuffers(_hdc: usize) -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn SetPixel(hdc: usize, x: i32, y: i32, color: u32) -> u32 {
    let m = objs().lock().unwrap();
    let Some(Obj::Dc(dc)) = m.get(&hdc) else {
        return CLR_INVALID;
    };
    let Some(Obj::Bitmap(bm)) = m.get(&dc.bitmap) else {
        return CLR_INVALID;
    };
    let rgba = colorref_to_rgba(color);
    if bm.set_rgba(x, y, rgba) {
        color & 0x00FF_FFFF
    } else {
        CLR_INVALID
    }
}

pub unsafe extern "win64" fn GetPixel(hdc: usize, x: i32, y: i32) -> u32 {
    let m = objs().lock().unwrap();
    let Some(Obj::Dc(dc)) = m.get(&hdc) else {
        return CLR_INVALID;
    };
    let Some(Obj::Bitmap(bm)) = m.get(&dc.bitmap) else {
        return CLR_INVALID;
    };
    bm.get_rgba(x, y).map_or(CLR_INVALID, rgba_to_colorref)
}

pub unsafe extern "win64" fn CreateBitmap(
    cx: i32,
    cy: i32,
    planes: u32,
    bpp: u32,
    init_bits: *const u8,
) -> usize {
    let w = cx.max(1) as usize;
    let h = cy.max(1) as usize;
    let pl = planes.max(1) as u16;
    let bc = bpp.max(1) as u16;
    let stride = ((w.saturating_mul(bc as usize).saturating_mul(pl as usize).saturating_add(15)) & !15) / 8;
    let total = stride.saturating_mul(h).max(4);
    let bits = crate::heap::alloc(total, true);
    if bits.is_null() {
        return 0;
    }
    if !init_bits.is_null() {
        std::ptr::copy_nonoverlapping(init_bits, bits as *mut u8, total);
    }
    let handle = NEXT.fetch_add(4, Ordering::Relaxed);
    objs().lock().unwrap().insert(
        handle,
        Obj::Bitmap(BitmapData {
            width: w,
            height: h,
            bpp: bc,
            planes: pl,
            top_down: true,
            is_dib: false,
            bits_ptr: bits as usize,
            stride,
            byte_len: total,
        }),
    );
    handle
}

pub unsafe extern "win64" fn CreateDCW(
    _driver: LPCWSTR,
    _device: LPCWSTR,
    _port: LPCWSTR,
    _dm: *const u8,
) -> usize {
    CreateCompatibleDC(0)
}

pub unsafe extern "win64" fn CreateDCA(
    _driver: LPCSTR,
    _device: LPCSTR,
    _port: LPCSTR,
    _dm: *const u8,
) -> usize {
    CreateCompatibleDC(0)
}

fn is_pattern_only_rop(rop: u32) -> bool {
    matches!(rop, BLACKNESS | WHITENESS | PATCOPY | PATINVERT | DSTINVERT)
}

unsafe fn apply_pat_rop(bm: &BitmapData, brush: &BrushData, x: i32, y: i32, w: i32, h: i32, rop: u32) {
    if w == 0 || h == 0 {
        return;
    }
    let (x0, x1) = if w >= 0 { (x, x.saturating_add(w)) } else { (x.saturating_add(w), x) };
    let (y0, y1) = if h >= 0 { (y, y.saturating_add(h)) } else { (y.saturating_add(h), y) };
    let cx0 = x0.max(0).min(bm.width as i32);
    let cx1 = x1.max(0).min(bm.width as i32);
    let cy0 = y0.max(0).min(bm.height as i32);
    let cy1 = y1.max(0).min(bm.height as i32);
    if cx0 >= cx1 || cy0 >= cy1 {
        return;
    }
    let pat = colorref_to_rgba(brush.color);
    for py in cy0..cy1 {
        for px in cx0..cx1 {
            match rop {
                BLACKNESS => {
                    bm.set_rgba(px, py, [0, 0, 0, 0xFF]);
                }
                WHITENESS => {
                    bm.set_rgba(px, py, [0xFF, 0xFF, 0xFF, 0xFF]);
                }
                PATCOPY => {
                    if brush.style != BS_NULL {
                        bm.set_rgba(px, py, pat);
                    }
                }
                PATINVERT => {
                    if brush.style != BS_NULL {
                        if let Some(d) = bm.get_rgba(px, py) {
                            bm.set_rgba(px, py, [d[0] ^ pat[0], d[1] ^ pat[1], d[2] ^ pat[2], d[3]]);
                        }
                    }
                }
                DSTINVERT => {
                    if let Some(d) = bm.get_rgba(px, py) {
                        bm.set_rgba(px, py, [!d[0], !d[1], !d[2], d[3]]);
                    }
                }
                _ => {}
            }
        }
    }
}

fn combine_src_rop(dst: [u8; 4], src: [u8; 4], rop: u32) -> [u8; 4] {
    match rop {
        SRCCOPY => src,
        SRCINVERT => [dst[0] ^ src[0], dst[1] ^ src[1], dst[2] ^ src[2], dst[3]],
        SRCPAINT => [dst[0] | src[0], dst[1] | src[1], dst[2] | src[2], dst[3]],
        SRCAND => [dst[0] & src[0], dst[1] & src[1], dst[2] & src[2], dst[3]],
        NOTSRCCOPY => [!src[0], !src[1], !src[2], src[3]],
        _ => src,
    }
}

pub unsafe extern "win64" fn BitBlt(
    dst: usize,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    src: usize,
    x1: i32,
    y1: i32,
    rop: u32,
) -> BOOL {
    let m = objs().lock().unwrap();
    let Some(Obj::Dc(dst_dc)) = m.get(&dst) else {
        return FALSE;
    };
    let Some(Obj::Bitmap(dst_bm)) = m.get(&dst_dc.bitmap) else {
        return TRUE;
    };
    if is_pattern_only_rop(rop) {
        let brush = resolve_brush(&m, dst_dc.brush);
        apply_pat_rop(dst_bm, &brush, x, y, cx, cy, rop);
        return TRUE;
    }
    if cx <= 0 || cy <= 0 {
        return TRUE;
    }
    let Some(Obj::Dc(src_dc)) = m.get(&src) else {
        return FALSE;
    };
    let Some(Obj::Bitmap(src_bm)) = m.get(&src_dc.bitmap) else {
        return TRUE;
    };

    // Snapshot the source region first so overlapping self-blits never clobber unread pixels.
    let w = cx as usize;
    let h = cy as usize;
    let mut tmp = vec![None; w.saturating_mul(h)];
    for row in 0..h {
        let sy = y1.saturating_add(row as i32);
        let dy = y.saturating_add(row as i32);
        if dy < 0 || (dy as usize) >= dst_bm.height {
            continue;
        }
        for col in 0..w {
            let sx = x1.saturating_add(col as i32);
            let dx = x.saturating_add(col as i32);
            if dx < 0 || (dx as usize) >= dst_bm.width {
                continue;
            }
            tmp[row * w + col] = src_bm.get_rgba(sx, sy);
        }
    }
    for row in 0..h {
        let dy = y.saturating_add(row as i32);
        if dy < 0 || (dy as usize) >= dst_bm.height {
            continue;
        }
        for col in 0..w {
            let dx = x.saturating_add(col as i32);
            if let Some(sp) = tmp[row * w + col] {
                let dp = dst_bm.get_rgba(dx, dy).unwrap_or([0, 0, 0, 0xFF]);
                dst_bm.set_rgba(dx, dy, combine_src_rop(dp, sp, rop));
            }
        }
    }
    TRUE
}

pub unsafe extern "win64" fn StretchBlt(
    dst: usize,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    src: usize,
    x1: i32,
    y1: i32,
    w1: i32,
    h1: i32,
    rop: u32,
) -> BOOL {
    let m = objs().lock().unwrap();
    let Some(Obj::Dc(dst_dc)) = m.get(&dst) else {
        return FALSE;
    };
    let Some(Obj::Bitmap(dst_bm)) = m.get(&dst_dc.bitmap) else {
        return TRUE;
    };
    if is_pattern_only_rop(rop) {
        let brush = resolve_brush(&m, dst_dc.brush);
        apply_pat_rop(dst_bm, &brush, x, y, cx, cy, rop);
        return TRUE;
    }
    if cx == 0 || cy == 0 || w1 == 0 || h1 == 0 {
        return TRUE;
    }
    let Some(Obj::Dc(src_dc)) = m.get(&src) else {
        return FALSE;
    };
    let Some(Obj::Bitmap(src_bm)) = m.get(&src_dc.bitmap) else {
        return TRUE;
    };

    let dst_w = cx.unsigned_abs() as i64;
    let dst_h = cy.unsigned_abs() as i64;
    let src_w = w1.unsigned_abs() as i64;
    let src_h = h1.unsigned_abs() as i64;
    let flip_x = (cx < 0) != (w1 < 0);
    let flip_y = (cy < 0) != (h1 < 0);
    let dx_start = if cx >= 0 { x } else { x.saturating_add(cx) };
    let dy_start = if cy >= 0 { y } else { y.saturating_add(cy) };
    let sx_start = if w1 >= 0 { x1 } else { x1.saturating_add(w1) };
    let sy_start = if h1 >= 0 { y1 } else { y1.saturating_add(h1) };

    let tw = dst_w as usize;
    let th = dst_h as usize;
    let mut tmp = vec![None; tw.saturating_mul(th)];
    for row in 0..dst_h {
        let dy = dy_start.saturating_add(row as i32);
        if dy < 0 || (dy as usize) >= dst_bm.height {
            continue;
        }
        let sample_y = if flip_y { dst_h - 1 - row } else { row };
        let sy = sy_start.saturating_add(((sample_y * src_h) / dst_h) as i32);
        for col in 0..dst_w {
            let dx = dx_start.saturating_add(col as i32);
            if dx < 0 || (dx as usize) >= dst_bm.width {
                continue;
            }
            let sample_x = if flip_x { dst_w - 1 - col } else { col };
            let sx = sx_start.saturating_add(((sample_x * src_w) / dst_w) as i32);
            tmp[(row as usize) * tw + (col as usize)] = src_bm.get_rgba(sx, sy);
        }
    }
    for row in 0..th {
        let dy = dy_start.saturating_add(row as i32);
        if dy < 0 || (dy as usize) >= dst_bm.height {
            continue;
        }
        for col in 0..tw {
            let dx = dx_start.saturating_add(col as i32);
            if let Some(sp) = tmp[row * tw + col] {
                let dp = dst_bm.get_rgba(dx, dy).unwrap_or([0, 0, 0, 0xFF]);
                dst_bm.set_rgba(dx, dy, combine_src_rop(dp, sp, rop));
            }
        }
    }
    TRUE
}

pub unsafe extern "win64" fn PatBlt(hdc: usize, x: i32, y: i32, w: i32, h: i32, rop: u32) -> BOOL {
    let m = objs().lock().unwrap();
    let Some(Obj::Dc(dc)) = m.get(&hdc) else {
        return FALSE;
    };
    let Some(Obj::Bitmap(bm)) = m.get(&dc.bitmap) else {
        return TRUE;
    };
    let brush = resolve_brush(&m, dc.brush);
    apply_pat_rop(bm, &brush, x, y, w, h, rop);
    TRUE
}

pub unsafe extern "win64" fn Rectangle(hdc: usize, l: i32, t: i32, r: i32, b: i32) -> BOOL {
    let m = objs().lock().unwrap();
    let Some(Obj::Dc(dc)) = m.get(&hdc) else {
        return FALSE;
    };
    let Some(Obj::Bitmap(bm)) = m.get(&dc.bitmap) else {
        return TRUE;
    };
    let x0 = l.min(r);
    let x1 = l.max(r);
    let y0 = t.min(b);
    let y1 = t.max(b);
    if x0 >= x1 || y0 >= y1 {
        return TRUE;
    }

    let brush = resolve_brush(&m, dc.brush);
    let pen = resolve_pen(&m, dc.pen);

    // Fill interior with selected brush (if not BS_NULL).
    if brush.style != BS_NULL {
        apply_pat_rop(bm, &brush, x0, y0, x1 - x0, y1 - y0, PATCOPY);
    }

    // Stroke border with selected pen (if not PS_NULL).
    if pen.style != PS_NULL {
        let pw = pen.width.max(1);
        let pen_rgba = colorref_to_rgba(pen.color);
        let cx0 = x0.max(0).min(bm.width as i32);
        let cx1 = x1.max(0).min(bm.width as i32);
        let cy0 = y0.max(0).min(bm.height as i32);
        let cy1 = y1.max(0).min(bm.height as i32);
        for py in cy0..cy1 {
            let on_y_edge = py < y0.saturating_add(pw) || py >= y1.saturating_sub(pw);
            for px in cx0..cx1 {
                let on_x_edge = px < x0.saturating_add(pw) || px >= x1.saturating_sub(pw);
                if on_y_edge || on_x_edge {
                    bm.set_rgba(px, py, pen_rgba);
                }
            }
        }
    }
    TRUE
}

pub unsafe extern "win64" fn TextOutA(
    _hdc: usize,
    _x: i32,
    _y: i32,
    _str: LPCSTR,
    _count: i32,
) -> BOOL {
    TRUE
}

unsafe fn write_dib_pixel(row_ptr: *mut u8, x: usize, bpp: u16, rgba: [u8; 4]) {
    match bpp {
        32 => {
            let p = row_ptr.add(x * 4);
            p.write_unaligned(rgba[2]);
            p.add(1).write_unaligned(rgba[1]);
            p.add(2).write_unaligned(rgba[0]);
            p.add(3).write_unaligned(rgba[3]);
        }
        24 => {
            let p = row_ptr.add(x * 3);
            p.write_unaligned(rgba[2]);
            p.add(1).write_unaligned(rgba[1]);
            p.add(2).write_unaligned(rgba[0]);
        }
        16 => {
            let v = (((rgba[0] as u16) >> 3) << 10)
                | (((rgba[1] as u16) >> 3) << 5)
                | ((rgba[2] as u16) >> 3);
            (row_ptr.add(x * 2) as *mut u16).write_unaligned(v);
        }
        8 => {
            let lum = ((rgba[0] as u32 * 77 + rgba[1] as u32 * 150 + rgba[2] as u32 * 29) >> 8) as u8;
            row_ptr.add(x).write_unaligned(lum);
        }
        _ => {}
    }
}

unsafe fn read_dib_pixel(row_ptr: *const u8, x: usize, bpp: u16) -> [u8; 4] {
    match bpp {
        32 => {
            let p = row_ptr.add(x * 4);
            let b = p.read_unaligned();
            let g = p.add(1).read_unaligned();
            let r = p.add(2).read_unaligned();
            let a = p.add(3).read_unaligned();
            [r, g, b, a]
        }
        24 => {
            let p = row_ptr.add(x * 3);
            let b = p.read_unaligned();
            let g = p.add(1).read_unaligned();
            let r = p.add(2).read_unaligned();
            [r, g, b, 0xFF]
        }
        16 => {
            let v = (row_ptr.add(x * 2) as *const u16).read_unaligned();
            let r = (((v >> 10) & 0x1F) * 255 / 31) as u8;
            let g = (((v >> 5) & 0x1F) * 255 / 31) as u8;
            let b = ((v & 0x1F) * 255 / 31) as u8;
            [r, g, b, 0xFF]
        }
        8 => {
            let v = row_ptr.add(x).read_unaligned();
            [v, v, v, 0xFF]
        }
        _ => [0, 0, 0, 0xFF],
    }
}

pub unsafe extern "win64" fn GetDIBits(
    _hdc: usize,
    hbm: usize,
    start: u32,
    lines: u32,
    bits: *mut u8,
    bmi: *mut u8,
    _usage: u32,
) -> i32 {
    if bmi.is_null() {
        return 0;
    }
    let bi_size = (bmi as *const u32).read_unaligned();
    if bi_size < 40 {
        return 0;
    }
    let m = objs().lock().unwrap();
    let Some(Obj::Bitmap(bm)) = m.get(&hbm) else {
        return 0;
    };

    if bits.is_null() {
        let out_bpp = {
            let req = (bmi.add(14) as *const u16).read_unaligned();
            if req == 0 { bm.bpp } else { req }
        };
        let stride = dib_stride(bm.width, out_bpp);
        (bmi.add(4) as *mut i32).write_unaligned(bm.width as i32);
        (bmi.add(8) as *mut i32).write_unaligned(bm.height as i32);
        (bmi.add(12) as *mut u16).write_unaligned(1);
        (bmi.add(14) as *mut u16).write_unaligned(out_bpp);
        (bmi.add(16) as *mut u32).write_unaligned(0); // BI_RGB
        (bmi.add(20) as *mut u32).write_unaligned((stride * bm.height) as u32);
        return 1;
    }

    let req_w = (bmi.add(4) as *const i32).read_unaligned();
    let req_h = (bmi.add(8) as *const i32).read_unaligned();
    let req_bpp = (bmi.add(14) as *const u16).read_unaligned();
    let dst_bpp = if req_bpp == 0 { bm.bpp } else { req_bpp };
    let dst_w = if req_w != 0 {
        bm.width.min(req_w.unsigned_abs() as usize)
    } else {
        bm.width
    };
    let dst_top_down = req_h < 0;
    let dst_stride = dib_stride(dst_w, dst_bpp);

    let start_u = start as usize;
    if start_u >= bm.height || lines == 0 {
        return 0;
    }
    let count = (lines as usize).min(bm.height - start_u);
    for i in 0..count {
        let y = if dst_top_down {
            start_u + i
        } else {
            bm.height - 1 - (start_u + i)
        };
        let row_dst = bits.add(i * dst_stride);
        std::ptr::write_bytes(row_dst, 0, dst_stride);
        for x in 0..dst_w {
            let rgba = bm.get_rgba(x as i32, y as i32).unwrap_or([0, 0, 0, 0xFF]);
            write_dib_pixel(row_dst, x, dst_bpp, rgba);
        }
    }
    (bmi.add(20) as *mut u32).write_unaligned((dst_stride * bm.height) as u32);
    count as i32
}

pub unsafe extern "win64" fn SetDIBits(
    _hdc: usize,
    hbm: usize,
    start: u32,
    lines: u32,
    bits: *const u8,
    bmi: *const u8,
    _usage: u32,
) -> i32 {
    if bits.is_null() || bmi.is_null() {
        return 0;
    }
    let bi_size = (bmi as *const u32).read_unaligned();
    if bi_size < 40 {
        return 0;
    }
    let m = objs().lock().unwrap();
    let Some(Obj::Bitmap(bm)) = m.get(&hbm) else {
        return 0;
    };

    let src_w = (bmi.add(4) as *const i32).read_unaligned().unsigned_abs() as usize;
    let raw_h = (bmi.add(8) as *const i32).read_unaligned();
    let src_h = raw_h.unsigned_abs() as usize;
    let src_top_down = raw_h < 0;
    let src_bpp = (bmi.add(14) as *const u16).read_unaligned();
    if src_w == 0 || src_h == 0 || src_bpp == 0 {
        return 0;
    }
    let src_stride = dib_stride(src_w, src_bpp);
    let copy_w = src_w.min(bm.width);
    let start_u = start as usize;
    if start_u >= bm.height || lines == 0 {
        return 0;
    }
    let count = (lines as usize).min(bm.height - start_u).min(src_h);
    for i in 0..count {
        let y = if src_top_down {
            start_u + i
        } else {
            bm.height - 1 - (start_u + i)
        };
        let row_src = bits.add(i * src_stride);
        for x in 0..copy_w {
            let rgba = read_dib_pixel(row_src, x, src_bpp);
            bm.set_rgba(x as i32, y as i32, rgba);
        }
    }
    count as i32
}

fn serialize_bitmap(bm: &BitmapData, want_dibsection: bool) -> Vec<u8> {
    let len = if want_dibsection && bm.is_dib { 104 } else { 32 };
    let mut out = vec![0u8; len];
    // BITMAP (32 bytes)
    out[0..4].copy_from_slice(&0i32.to_le_bytes()); // bmType
    out[4..8].copy_from_slice(&(bm.width as i32).to_le_bytes()); // bmWidth
    out[8..12].copy_from_slice(&(bm.height as i32).to_le_bytes()); // bmHeight
    out[12..16].copy_from_slice(&(bm.stride as i32).to_le_bytes()); // bmWidthBytes
    out[16..18].copy_from_slice(&bm.planes.to_le_bytes()); // bmPlanes
    out[18..20].copy_from_slice(&bm.bpp.to_le_bytes()); // bmBitsPixel
    let exposed_bits: usize = if bm.is_dib { bm.bits_ptr } else { 0 };
    out[24..32].copy_from_slice(&exposed_bits.to_le_bytes()); // bmBits
    if len == 104 {
        // DIBSECTION.dsBmih (BITMAPINFOHEADER @0x20, 40 bytes)
        let h_signed = if bm.top_down { -(bm.height as i32) } else { bm.height as i32 };
        out[0x20..0x24].copy_from_slice(&40u32.to_le_bytes()); // biSize
        out[0x24..0x28].copy_from_slice(&(bm.width as i32).to_le_bytes()); // biWidth
        out[0x28..0x2C].copy_from_slice(&h_signed.to_le_bytes()); // biHeight
        out[0x2C..0x2E].copy_from_slice(&bm.planes.to_le_bytes()); // biPlanes
        out[0x2E..0x30].copy_from_slice(&bm.bpp.to_le_bytes()); // biBitCount
        out[0x30..0x34].copy_from_slice(&0u32.to_le_bytes()); // biCompression = BI_RGB
        out[0x34..0x38].copy_from_slice(&((bm.stride * bm.height) as u32).to_le_bytes()); // biSizeImage
    }
    out
}

fn serialize_brush(b: &BrushData) -> Vec<u8> {
    let mut out = vec![0u8; 16];
    out[0..4].copy_from_slice(&b.style.to_le_bytes());
    out[4..8].copy_from_slice(&b.color.to_le_bytes());
    out[8..16].copy_from_slice(&b.hatch.to_le_bytes());
    out
}

fn serialize_pen(p: &PenData) -> Vec<u8> {
    let mut out = vec![0u8; 16];
    out[0..4].copy_from_slice(&p.style.to_le_bytes());
    out[4..8].copy_from_slice(&p.width.to_le_bytes()); // lopnWidth.x
    out[8..12].copy_from_slice(&0i32.to_le_bytes()); // lopnWidth.y
    out[12..16].copy_from_slice(&p.color.to_le_bytes());
    out
}

fn serialize_logfont(f: &FontData, wide: bool) -> Vec<u8> {
    let len = if wide { 92 } else { 60 };
    let mut out = vec![0u8; len];
    out[0..4].copy_from_slice(&f.height.to_le_bytes());
    out[4..8].copy_from_slice(&f.width.to_le_bytes());
    out[8..12].copy_from_slice(&f.escapement.to_le_bytes());
    out[12..16].copy_from_slice(&f.orientation.to_le_bytes());
    out[16..20].copy_from_slice(&f.weight.to_le_bytes());
    out[20] = f.italic;
    out[21] = f.underline;
    out[22] = f.strike_out;
    out[23] = f.charset;
    out[24] = f.out_precision;
    out[25] = f.clip_precision;
    out[26] = f.quality;
    out[27] = f.pitch_and_family;
    if wide {
        for (i, u) in f.face.encode_utf16().take(31).enumerate() {
            out[28 + i * 2..30 + i * 2].copy_from_slice(&u.to_le_bytes());
        }
    } else {
        let bytes = f.face.as_bytes();
        let n = bytes.len().min(31);
        out[28..28 + n].copy_from_slice(&bytes[..n]);
    }
    out
}

unsafe fn get_object_impl(h: usize, cb: i32, pv: *mut u8, wide: bool) -> i32 {
    let data: Option<Vec<u8>> = if let Some(idx) = stock_index(h) {
        if let Some(b) = stock_brush(idx) {
            Some(serialize_brush(&b))
        } else if let Some(p) = stock_pen(idx) {
            Some(serialize_pen(&p))
        } else if let Some(f) = stock_font(idx) {
            Some(serialize_logfont(&f, wide))
        } else if idx == DEFAULT_STOCK_BITMAP {
            let def = BitmapData {
                width: 1,
                height: 1,
                bpp: 1,
                planes: 1,
                top_down: true,
                is_dib: false,
                bits_ptr: 0,
                stride: 2,
                byte_len: 2,
            };
            Some(serialize_bitmap(&def, false))
        } else {
            None
        }
    } else {
        let m = objs().lock().unwrap();
        match m.get(&h) {
            Some(Obj::Bitmap(bm)) => Some(serialize_bitmap(bm, cb >= 104)),
            Some(Obj::Brush(b)) => Some(serialize_brush(b)),
            Some(Obj::Pen(p)) => Some(serialize_pen(p)),
            Some(Obj::Font(f)) => Some(serialize_logfont(f, wide)),
            _ => None,
        }
    };

    let Some(bytes) = data else {
        return 0;
    };
    if pv.is_null() {
        return bytes.len() as i32;
    }
    if cb <= 0 {
        return 0;
    }
    let copy_len = (cb as usize).min(bytes.len());
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), pv, copy_len);
    copy_len as i32
}

pub unsafe extern "win64" fn GetObjectW(h: usize, cb: i32, pv: *mut u8) -> i32 {
    get_object_impl(h, cb, pv, true)
}

pub unsafe extern "win64" fn GetObjectA(h: usize, cb: i32, pv: *mut u8) -> i32 {
    get_object_impl(h, cb, pv, false)
}

pub unsafe extern "win64" fn CreateRectRgn(_l: i32, _t: i32, _r: i32, _b: i32) -> usize {
    let h = NEXT.fetch_add(4, Ordering::Relaxed);
    objs().lock().unwrap().insert(h, Obj::Region);
    h
}

pub unsafe extern "win64" fn CombineRgn(_dst: usize, _s1: usize, _s2: usize, _mode: i32) -> i32 {
    2 // SIMPLEREGION
}

pub unsafe extern "win64" fn GetDeviceGammaRamp(hdc: usize, ramp: *mut u16) -> BOOL {
    if ramp.is_null() {
        return FALSE;
    }
    let m = objs().lock().unwrap();
    let Some(Obj::Dc(dc)) = m.get(&hdc) else {
        return FALSE;
    };
    if let Some(ref stored) = dc.gamma_ramp {
        for (i, &val) in stored.iter().enumerate() {
            ramp.add(i).write_unaligned(val);
        }
    } else {
        for c in 0..3usize {
            for i in 0..256usize {
                ramp.add(c * 256 + i).write_unaligned((i as u16) << 8);
            }
        }
    }
    TRUE
}

pub unsafe extern "win64" fn SetDeviceGammaRamp(hdc: usize, ramp: *const u16) -> BOOL {
    if ramp.is_null() {
        return FALSE;
    }
    let mut table = Box::new([0u16; 768]);
    for i in 0..768usize {
        table[i] = ramp.add(i).read_unaligned();
    }
    // Validate each of the 3 RGB channels (256 entries each) is monotonically non-decreasing.
    for c in 0..3usize {
        let ch = &table[c * 256..(c + 1) * 256];
        if ch.windows(2).any(|w| w[0] > w[1]) {
            return FALSE;
        }
    }
    let mut m = objs().lock().unwrap();
    let Some(Obj::Dc(dc)) = m.get_mut(&hdc) else {
        return FALSE;
    };
    dc.gamma_ramp = Some(table);
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_bmi_header(w: i32, h: i32, bpp: u16) -> [u8; 40] {
        let mut bmi = [0u8; 40];
        bmi[0..4].copy_from_slice(&40u32.to_le_bytes());
        bmi[4..8].copy_from_slice(&w.to_le_bytes());
        bmi[8..12].copy_from_slice(&h.to_le_bytes());
        bmi[12..14].copy_from_slice(&1u16.to_le_bytes());
        bmi[14..16].copy_from_slice(&bpp.to_le_bytes());
        bmi
    }

    #[test]
    fn dc_font_extent() {
        unsafe {
            let dc = CreateCompatibleDC(0);
            assert_ne!(dc, 0);
            let f = CreateFontA(-20, 0, 0, 0, 400, 0, 0, 0, 0, 0, 0, 0, 0, c"Arial".as_ptr() as LPCSTR);
            let old = SelectObject(dc, f);
            assert_eq!(old, stock_handle(SYSTEM_FONT));
            let s: Vec<u16> = "abcd".encode_utf16().collect();
            let mut sz = SIZE { cx: 0, cy: 0 };
            assert_eq!(GetTextExtentPoint32W(dc, s.as_ptr(), 4, &mut sz), TRUE);
            assert_eq!((sz.cx, sz.cy), (40, 20));

            // Proportional width: "iiii" is narrower than "WWWW", and A matches W.
            let mut sz_narrow = SIZE { cx: 0, cy: 0 };
            let mut sz_wide = SIZE { cx: 0, cy: 0 };
            assert_eq!(GetTextExtentPoint32A(dc, b"iiii".as_ptr(), 4, &mut sz_narrow), TRUE);
            assert_eq!(GetTextExtentPoint32A(dc, b"WWWW".as_ptr(), 4, &mut sz_wide), TRUE);
            assert!(sz_narrow.cx < sz_wide.cx);

            // Monospace font gives equal widths for "iiii" and "WWWW".
            let mono = CreateFontA(-20, 0, 0, 0, 400, 0, 0, 0, 0, 0, 0, 0, FIXED_PITCH as usize, c"Consolas".as_ptr() as LPCSTR);
            SelectObject(dc, mono);
            assert_eq!(GetTextExtentPoint32A(dc, b"iiii".as_ptr(), 4, &mut sz_narrow), TRUE);
            assert_eq!(GetTextExtentPoint32A(dc, b"WWWW".as_ptr(), 4, &mut sz_wide), TRUE);
            assert_eq!(sz_narrow.cx, sz_wide.cx);

            assert_eq!(SelectObject(dc, old), mono);
            assert_eq!(DeleteObject(mono), TRUE);
            assert_eq!(DeleteObject(f), TRUE);
            assert_eq!(DeleteDC(dc), TRUE);
            assert_eq!(DeleteDC(dc), FALSE);
            assert_eq!(GetStockObject(4), stock_handle(4));
            assert_eq!(GetObjectType(stock_handle(DEFAULT_GUI_FONT)), OBJ_FONT);
            assert_eq!(GetObjectType(stock_handle(BLACK_PEN)), OBJ_PEN);
        }
    }

    #[test]
    fn dib_rasterizer_pixels_patblt_rectangle_bitblt_stretchblt() {
        unsafe {
            let dc1 = CreateCompatibleDC(0);
            let dc2 = CreateCompatibleDC(0);
            let bmi = make_bmi_header(8, -8, 32); // 8x8 top-down 32bpp
            let mut bits1: *mut std::ffi::c_void = std::ptr::null_mut();
            let mut bits2: *mut std::ffi::c_void = std::ptr::null_mut();
            let bm1 = CreateDIBSection(dc1, bmi.as_ptr(), 0, &mut bits1, std::ptr::null_mut(), 0);
            let bm2 = CreateDIBSection(dc2, bmi.as_ptr(), 0, &mut bits2, std::ptr::null_mut(), 0);
            assert_ne!(bm1, 0);
            assert_ne!(bm2, 0);
            assert!(!bits1.is_null() && !bits2.is_null());

            let old1 = SelectObject(dc1, bm1);
            let old2 = SelectObject(dc2, bm2);
            assert_eq!(old1, stock_handle(DEFAULT_STOCK_BITMAP));

            // SetPixel / GetPixel roundtrip (COLORREF 0x00BBGGRR)
            let red = 0x0000_00FF;
            let green = 0x0000_FF00;
            let blue = 0x00FF_0000;
            assert_eq!(SetPixel(dc1, 1, 2, red), red);
            assert_eq!(GetPixel(dc1, 1, 2), red);
            assert_eq!(GetPixel(dc1, 99, 99), CLR_INVALID);

            // PatBlt with custom brush, WHITENESS, BLACKNESS
            let br = CreateSolidBrush(green);
            SelectObject(dc1, br);
            assert_eq!(PatBlt(dc1, 0, 0, 4, 4, PATCOPY), TRUE);
            assert_eq!(GetPixel(dc1, 2, 2), green);
            assert_eq!(PatBlt(dc1, 0, 0, 2, 2, WHITENESS), TRUE);
            assert_eq!(GetPixel(dc1, 0, 0), 0x00FF_FFFF);
            assert_eq!(PatBlt(dc1, 0, 0, 1, 1, BLACKNESS), TRUE);
            assert_eq!(GetPixel(dc1, 0, 0), 0x0000_0000);

            // Rectangle: blue border, green fill
            let pen = CreatePen(PS_SOLID as i32, 1, blue);
            SelectObject(dc1, pen);
            assert_eq!(Rectangle(dc1, 1, 1, 6, 6), TRUE);
            assert_eq!(GetPixel(dc1, 1, 1), blue); // border
            assert_eq!(GetPixel(dc1, 5, 5), blue); // bottom-right border
            assert_eq!(GetPixel(dc1, 3, 3), green); // interior

            // BitBlt SRCCOPY and SRCINVERT
            assert_eq!(BitBlt(dc2, 0, 0, 8, 8, dc1, 0, 0, SRCCOPY), TRUE);
            assert_eq!(GetPixel(dc2, 1, 1), blue);
            assert_eq!(GetPixel(dc2, 3, 3), green);
            assert_eq!(BitBlt(dc2, 0, 0, 8, 8, dc1, 0, 0, SRCINVERT), TRUE);
            assert_eq!(GetPixel(dc2, 3, 3), 0x0000_0000); // XOR with self -> 0

            // StretchBlt 2x nearest-neighbor upscale
            assert_eq!(SetPixel(dc1, 0, 0, red), red);
            assert_eq!(SetPixel(dc1, 1, 0, blue), blue);
            assert_eq!(StretchBlt(dc2, 0, 0, 4, 4, dc1, 0, 0, 2, 2, SRCCOPY), TRUE);
            assert_eq!(GetPixel(dc2, 0, 0), red);
            assert_eq!(GetPixel(dc2, 1, 1), red);
            assert_eq!(GetPixel(dc2, 2, 0), blue);
            assert_eq!(GetPixel(dc2, 3, 1), blue);

            SelectObject(dc1, old1);
            SelectObject(dc2, old2);
            DeleteObject(pen);
            DeleteObject(br);
            DeleteObject(bm1);
            DeleteObject(bm2);
            DeleteDC(dc1);
            DeleteDC(dc2);
        }
    }

    #[test]
    fn get_set_dibits_32_and_24_conversion_and_get_object() {
        unsafe {
            let dc = CreateCompatibleDC(0);
            let bmi32 = make_bmi_header(2, 2, 32); // bottom-up 2x2 32bpp
            let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
            let hbm = CreateDIBSection(dc, bmi32.as_ptr(), 0, &mut bits, std::ptr::null_mut(), 0);
            assert_ne!(hbm, 0);
            SelectObject(dc, hbm);

            // SetDIBits with 24-bit BGR top-down buffer (2px * 3B = 6B -> padded to 8B stride!)
            let bmi24 = make_bmi_header(2, -2, 24);
            let src24: [u8; 16] = [
                // Row 0 (top): px(0,0)=R(0x11,0x22,0x33 -> BGR 33,22,11), px(1,0)=(44,55,66 -> BGR 66,55,44)
                0x33, 0x22, 0x11, 0x66, 0x55, 0x44, 0x00, 0x00,
                // Row 1 (bottom): px(0,1)=(77,88,99 -> BGR 99,88,77), px(1,1)=(AA,BB,CC -> BGR CC,BB,AA)
                0x99, 0x88, 0x77, 0xCC, 0xBB, 0xAA, 0x00, 0x00,
            ];
            assert_eq!(SetDIBits(dc, hbm, 0, 2, src24.as_ptr(), bmi24.as_ptr(), 0), 2);
            assert_eq!(GetPixel(dc, 0, 0), 0x0033_2211);
            assert_eq!(GetPixel(dc, 1, 0), 0x0066_5544);
            assert_eq!(GetPixel(dc, 0, 1), 0x0099_8877);
            assert_eq!(GetPixel(dc, 1, 1), 0x00CC_BBAA);

            // GetDIBits back out as 24-bit BGR bottom-up
            let mut bmi24_out = make_bmi_header(2, 2, 24);
            let mut dst24 = [0u8; 16];
            assert_eq!(GetDIBits(dc, hbm, 0, 2, dst24.as_mut_ptr(), bmi24_out.as_mut_ptr(), 0), 2);
            // Bottom-up row 0 is logical y=1, row 1 is logical y=0!
            assert_eq!(&dst24[0..6], &[0x99, 0x88, 0x77, 0xCC, 0xBB, 0xAA]);
            assert_eq!(&dst24[8..14], &[0x33, 0x22, 0x11, 0x66, 0x55, 0x44]);

            // GetObject: BITMAP (32B), DIBSECTION (104B), LOGBRUSH (16B), LOGPEN (16B), LOGFONTA/W
            let mut bm_buf = [0u8; 32];
            assert_eq!(GetObjectW(hbm, 32, bm_buf.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm_buf[4..8].try_into().unwrap()), 2);
            assert_eq!(i32::from_le_bytes(bm_buf[8..12].try_into().unwrap()), 2);
            assert_eq!(u16::from_le_bytes(bm_buf[18..20].try_into().unwrap()), 32);
            assert_eq!(usize::from_le_bytes(bm_buf[24..32].try_into().unwrap()), bits as usize);

            let mut ds_buf = [0u8; 104];
            assert_eq!(GetObjectW(hbm, 104, ds_buf.as_mut_ptr()), 104);
            assert_eq!(u32::from_le_bytes(ds_buf[0x20..0x24].try_into().unwrap()), 40);

            let br = CreateSolidBrush(0x0012_3456);
            let mut lb = [0u8; 16];
            assert_eq!(GetObjectA(br, 16, lb.as_mut_ptr()), 16);
            assert_eq!(u32::from_le_bytes(lb[4..8].try_into().unwrap()), 0x0012_3456);

            let pen = CreatePen(PS_SOLID as i32, 3, 0x0065_4321);
            let mut lp = [0u8; 16];
            assert_eq!(GetObjectA(pen, 16, lp.as_mut_ptr()), 16);
            assert_eq!(i32::from_le_bytes(lp[4..8].try_into().unwrap()), 3);
            assert_eq!(u32::from_le_bytes(lp[12..16].try_into().unwrap()), 0x0065_4321);

            let font = CreateFontA(-18, 0, 0, 0, 700, 1, 0, 0, 0, 0, 0, 0, 0, c"Verdana".as_ptr() as LPCSTR);
            let mut lfa = [0u8; 60];
            assert_eq!(GetObjectA(font, 60, lfa.as_mut_ptr()), 60);
            assert_eq!(i32::from_le_bytes(lfa[0..4].try_into().unwrap()), -18);
            assert_eq!(i32::from_le_bytes(lfa[16..20].try_into().unwrap()), 700);
            assert_eq!(&lfa[28..35], b"Verdana");

            DeleteObject(font);
            DeleteObject(pen);
            DeleteObject(br);
            DeleteObject(hbm);
            DeleteDC(dc);
        }
    }

    #[test]
    fn gamma_ramp_per_dc_roundtrip_and_validation() {
        unsafe {
            let dc = CreateCompatibleDC(0);
            let mut got = [0u16; 768];
            assert_eq!(GetDeviceGammaRamp(dc, got.as_mut_ptr()), TRUE);
            assert_eq!(got[0], 0);
            assert_eq!(got[255], 255u16 << 8);

            // Set custom half-brightness monotonic ramp.
            let mut custom = [0u16; 768];
            for c in 0..3usize {
                for i in 0..256usize {
                    custom[c * 256 + i] = (i as u16) * 128;
                }
            }
            assert_eq!(SetDeviceGammaRamp(dc, custom.as_ptr()), TRUE);
            assert_eq!(GetDeviceGammaRamp(dc, got.as_mut_ptr()), TRUE);
            assert_eq!(got, custom);

            // Non-monotonic ramp must be rejected and preserve previous ramp.
            let mut bad = custom;
            bad[10] = 50000;
            bad[11] = 100;
            assert_eq!(SetDeviceGammaRamp(dc, bad.as_ptr()), FALSE);
            assert_eq!(GetDeviceGammaRamp(dc, got.as_mut_ptr()), TRUE);
            assert_eq!(got, custom);

            // Invalid DC returns FALSE.
            assert_eq!(GetDeviceGammaRamp(0, got.as_mut_ptr()), FALSE);
            assert_eq!(SetDeviceGammaRamp(0, custom.as_ptr()), FALSE);

            DeleteDC(dc);
        }
    }

    #[test]
    fn get_device_caps_display_and_memdc_indices() {
        unsafe {
            assert_eq!(GetDeviceCaps(0, 0), 0x4000); // DRIVERVERSION
            assert_eq!(GetDeviceCaps(0, 2), 1); // TECHNOLOGY == DT_RASDISPLAY
            assert!(GetDeviceCaps(0, 8) > 0); // HORZRES
            assert!(GetDeviceCaps(0, 10) > 0); // VERTRES
            assert_eq!(GetDeviceCaps(0, 12), 32); // BITSPIXEL
            assert_eq!(GetDeviceCaps(0, 14), 1); // PLANES
            assert_eq!(GetDeviceCaps(0, 24), -1); // NUMCOLORS
            assert_eq!(GetDeviceCaps(0, 36), 1); // CLIPCAPS
            assert_ne!(GetDeviceCaps(0, 38), 0); // RASTERCAPS
            assert_eq!(GetDeviceCaps(0, 45), 3); // SHADEBLENDCAPS
            assert!(GetDeviceCaps(0, 88) >= 96); // LOGPIXELSX
            assert_eq!(GetDeviceCaps(0, 108), 24); // COLORRES
            assert!(GetDeviceCaps(0, 116) >= 30); // VREFRESH
            assert_eq!(GetDeviceCaps(0, 9999), 0); // unknown index -> 0

            // Selecting a 64x32 24bpp DIB into a compatible memory DC must NOT override the
            // display device capabilities reported by GetDeviceCaps (bitmap dimensions come
            // from GetObject instead).
            let screen_w = GetDeviceCaps(0, 8);
            let screen_h = GetDeviceCaps(0, 10);
            let screen_mm_w = GetDeviceCaps(0, 4);
            let screen_mm_h = GetDeviceCaps(0, 6);
            let dc = CreateCompatibleDC(0);
            let bmi = make_bmi_header(64, -32, 24);
            let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
            let hbm = CreateDIBSection(dc, bmi.as_ptr(), 0, &mut bits, std::ptr::null_mut(), 0);
            let old = SelectObject(dc, hbm);
            assert_eq!(GetDeviceCaps(dc, 8), screen_w);
            assert_eq!(GetDeviceCaps(dc, 10), screen_h);
            assert_eq!(GetDeviceCaps(dc, 4), screen_mm_w);
            assert_eq!(GetDeviceCaps(dc, 6), screen_mm_h);
            assert_eq!(GetDeviceCaps(dc, 12), 32);
            assert_eq!(GetDeviceCaps(dc, 14), 1);
            assert_eq!(GetDeviceCaps(dc, 108), 24);
            let mut bm_buf = [0u8; 32];
            assert_eq!(GetObjectW(hbm, 32, bm_buf.as_mut_ptr()), 32);
            assert_eq!(i32::from_le_bytes(bm_buf[4..8].try_into().unwrap()), 64);
            assert_eq!(i32::from_le_bytes(bm_buf[8..12].try_into().unwrap()), 32);
            assert_eq!(u16::from_le_bytes(bm_buf[18..20].try_into().unwrap()), 24);
            SelectObject(dc, old);
            DeleteObject(hbm);
            DeleteDC(dc);
        }
    }
}
