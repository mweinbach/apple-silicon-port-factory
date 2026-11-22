//! dwrite.dll: Native DirectWrite typography, TrueType/OpenType font parsing,
//! text layout, hit-testing, and glyph outline/bitmap rasterization.

use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fs::{self, File};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// DirectWrite Constants, Enums, GUIDs & HRESULTs (dwrite.h)
// ============================================================================

pub const DWRITE_E_FILEFORMAT: HRESULT = 0x8898_5000u32 as i32;
pub const DWRITE_E_NOFONT: HRESULT = 0x8898_5002u32 as i32;
pub const DWRITE_E_FILENOTFOUND: HRESULT = 0x8898_5003u32 as i32;
pub const DWRITE_E_FILEACCESS: HRESULT = 0x8898_5004u32 as i32;
pub const E_NOT_SUFFICIENT_BUFFER: HRESULT = 0x8007_007Au32 as i32;

// Factory types
pub const DWRITE_FACTORY_TYPE_SHARED: u32 = 0;
pub const DWRITE_FACTORY_TYPE_ISOLATED: u32 = 1;

// Font weights
pub const DWRITE_FONT_WEIGHT_THIN: u32 = 100;
pub const DWRITE_FONT_WEIGHT_NORMAL: u32 = 400;
pub const DWRITE_FONT_WEIGHT_BOLD: u32 = 700;

// Font stretch
pub const DWRITE_FONT_STRETCH_NORMAL: u32 = 5;

// Font styles
pub const DWRITE_FONT_STYLE_NORMAL: u32 = 0;
pub const DWRITE_FONT_STYLE_OBLIQUE: u32 = 1;
pub const DWRITE_FONT_STYLE_ITALIC: u32 = 2;

// Font simulations
pub const DWRITE_FONT_SIMULATIONS_NONE: u32 = 0;
pub const DWRITE_FONT_SIMULATIONS_BOLD: u32 = 1;
pub const DWRITE_FONT_SIMULATIONS_OBLIQUE: u32 = 2;

// Font face types
pub const DWRITE_FONT_FACE_TYPE_CFF: u32 = 0;
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE: u32 = 1;
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION: u32 = 2;
pub const DWRITE_FONT_FACE_TYPE_UNKNOWN: u32 = 7;

// Font file types
pub const DWRITE_FONT_FILE_TYPE_UNKNOWN: u32 = 0;
pub const DWRITE_FONT_FILE_TYPE_CFF: u32 = 1;
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE: u32 = 2;
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION: u32 = 3;

// Text & paragraph alignment
pub const DWRITE_TEXT_ALIGNMENT_LEADING: u32 = 0;
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: u32 = 1;
pub const DWRITE_TEXT_ALIGNMENT_CENTER: u32 = 2;
pub const DWRITE_TEXT_ALIGNMENT_JUSTIFIED: u32 = 3;

pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: u32 = 0;
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: u32 = 1;
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: u32 = 2;

// Word wrapping
pub const DWRITE_WORD_WRAPPING_WRAP: u32 = 0;
pub const DWRITE_WORD_WRAPPING_NO_WRAP: u32 = 1;
pub const DWRITE_WORD_WRAPPING_EMERGENCY_BREAK: u32 = 2;
pub const DWRITE_WORD_WRAPPING_WHOLE_WORD: u32 = 3;
pub const DWRITE_WORD_WRAPPING_CHARACTER: u32 = 4;

// Line spacing
pub const DWRITE_LINE_SPACING_METHOD_DEFAULT: u32 = 0;
pub const DWRITE_LINE_SPACING_METHOD_UNIFORM: u32 = 1;
pub const DWRITE_LINE_SPACING_METHOD_PROPORTIONAL: u32 = 2;

// Trimming granularity
pub const DWRITE_TRIMMING_GRANULARITY_NONE: u32 = 0;
pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: u32 = 1;
pub const DWRITE_TRIMMING_GRANULARITY_WORD: u32 = 2;

// Measuring / rendering modes
pub const DWRITE_MEASURING_MODE_NATURAL: u32 = 0;
pub const DWRITE_RENDERING_MODE_DEFAULT: u32 = 0;
pub const DWRITE_RENDERING_MODE_ALIASED: u32 = 1;
pub const DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC: u32 = 5;
pub const DWRITE_PIXEL_GEOMETRY_FLAT: u32 = 0;
pub const DWRITE_PIXEL_GEOMETRY_RGB: u32 = 1;

// Informational string IDs
const DWRITE_INFO_COPYRIGHT_NOTICE: u32 = 1;
const DWRITE_INFO_VERSION_STRINGS: u32 = 2;
const DWRITE_INFO_TRADEMARK: u32 = 3;
const DWRITE_INFO_MANUFACTURER: u32 = 4;
const DWRITE_INFO_DESIGNER: u32 = 5;
const DWRITE_INFO_DESCRIPTION: u32 = 8;
const DWRITE_INFO_LICENSE_DESCRIPTION: u32 = 10;
const DWRITE_INFO_FULL_NAME: u32 = 13;
const DWRITE_INFO_POSTSCRIPT_NAME: u32 = 14;

const fn dw_guid(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> GUID {
    GUID { data1: d1, data2: d2, data3: d3, data4: d4 }
}

pub const IID_IUNKNOWN: GUID =
    dw_guid(0x0000_0000, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
pub const IID_IDWRITE_FACTORY: GUID =
    dw_guid(0xB859_EE5A, 0xD838, 0x4B5B, [0xA2, 0xE8, 0x1A, 0xDC, 0x7D, 0x93, 0xDB, 0x48]);
pub const IID_IDWRITE_FACTORY1: GUID =
    dw_guid(0x3057_2F99, 0xDAC6, 0x41DB, [0xA1, 0x6E, 0x04, 0x86, 0x30, 0x7E, 0x60, 0x6A]);
pub const IID_IDWRITE_FACTORY2: GUID =
    dw_guid(0x0439_FC60, 0xCA44, 0x4994, [0x8D, 0xEE, 0x3A, 0x9A, 0xF7, 0xB7, 0x32, 0xEC]);
pub const IID_IDWRITE_FACTORY3: GUID =
    dw_guid(0x9A1B_41C3, 0xD3BB, 0x466A, [0x87, 0xFC, 0xFE, 0x67, 0x55, 0x6A, 0x3B, 0x65]);
pub const IID_IDWRITE_FONT_COLLECTION: GUID =
    dw_guid(0xA84C_EE02, 0x3EEA, 0x4EEE, [0xA8, 0x27, 0x87, 0xC1, 0xA0, 0x2A, 0x0F, 0xCC]);
pub const IID_IDWRITE_FONT_LIST: GUID =
    dw_guid(0x1A0D_8438, 0x1D97, 0x4EC1, [0xAE, 0xF9, 0xA2, 0xFB, 0x86, 0xED, 0x6A, 0xCB]);
pub const IID_IDWRITE_FONT_FAMILY: GUID =
    dw_guid(0xDA20_D8EF, 0x812A, 0x4C43, [0x98, 0x02, 0x62, 0xEC, 0x4A, 0xBD, 0x7A, 0xDD]);
pub const IID_IDWRITE_FONT: GUID =
    dw_guid(0xACD1_6696, 0x8C14, 0x4F5D, [0x87, 0x7E, 0xFE, 0x3F, 0xC1, 0xD3, 0x27, 0x37]);
pub const IID_IDWRITE_FONT_FACE: GUID =
    dw_guid(0x5F49_804D, 0x7024, 0x4D43, [0xBF, 0xA9, 0xD2, 0x59, 0x84, 0xF5, 0x38, 0x49]);
pub const IID_IDWRITE_FONT_FILE: GUID =
    dw_guid(0x739D_886A, 0xCEF5, 0x47DC, [0x87, 0x69, 0x1A, 0x8B, 0x41, 0xBE, 0xBB, 0xB0]);
pub const IID_IDWRITE_LOCALIZED_STRINGS: GUID =
    dw_guid(0x0825_6209, 0x099A, 0x4B34, [0xB8, 0x6D, 0xC2, 0x2B, 0x11, 0x0E, 0x77, 0x71]);
pub const IID_IDWRITE_RENDERING_PARAMS: GUID =
    dw_guid(0x2F0D_A53A, 0x2ADD, 0x47CD, [0x82, 0xEE, 0xD9, 0xEC, 0x34, 0x68, 0x8E, 0x75]);
pub const IID_IDWRITE_TEXT_FORMAT: GUID =
    dw_guid(0x9C90_6818, 0x31D7, 0x4FD3, [0xA1, 0x51, 0x7C, 0x5E, 0x22, 0x5D, 0xB5, 0x5A]);
pub const IID_IDWRITE_TEXT_LAYOUT: GUID =
    dw_guid(0x5373_7037, 0x6D14, 0x410B, [0x9B, 0xFE, 0x0B, 0x18, 0x2B, 0xB7, 0x09, 0x61]);
pub const IID_IDWRITE_TYPOGRAPHY: GUID =
    dw_guid(0x55F1_112B, 0x1DC2, 0x4B3C, [0x95, 0x41, 0xF4, 0x68, 0x94, 0xED, 0x85, 0xB6]);
pub const IID_IDWRITE_GDI_INTEROP: GUID =
    dw_guid(0x1EDD_9491, 0x9853, 0x4299, [0x89, 0x8F, 0x64, 0x32, 0x98, 0x3B, 0x6F, 0x3A]);
pub const IID_IDWRITE_BITMAP_RENDER_TARGET: GUID =
    dw_guid(0x5E5A_32A3, 0x8DFF, 0x4773, [0x9F, 0xF6, 0x06, 0x96, 0xEA, 0xB7, 0x72, 0x67]);
pub const IID_IDWRITE_INLINE_OBJECT: GUID =
    dw_guid(0x8339_FDE3, 0x106F, 0x47AB, [0x83, 0x73, 0x1C, 0x62, 0x95, 0xEB, 0x10, 0xB3]);
pub const IID_IDWRITE_NUMBER_SUBSTITUTION: GUID =
    dw_guid(0x1488_5CC9, 0xBAB0, 0x4F90, [0xB6, 0xED, 0x5C, 0x36, 0x6A, 0x2C, 0xD0, 0x3D]);
pub const IID_IDWRITE_TEXT_ANALYZER: GUID =
    dw_guid(0xB7E6_163E, 0x7F46, 0x43B4, [0x84, 0xB3, 0xE4, 0xE6, 0x24, 0x9C, 0x36, 0x5D]);

// ============================================================================
// DirectWrite Win64 ABI Structs
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_FONT_METRICS {
    pub designUnitsPerEm: u16,
    pub ascent: u16,
    pub descent: u16,
    pub lineGap: i16,
    pub capHeight: u16,
    pub xHeight: u16,
    pub underlinePosition: i16,
    pub underlineThickness: u16,
    pub strikethroughPosition: i16,
    pub strikethroughThickness: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: i32,
    pub advanceWidth: u32,
    pub rightSideBearing: i32,
    pub topSideBearing: i32,
    pub advanceHeight: u32,
    pub bottomSideBearing: i32,
    pub verticalOriginY: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}

impl DWRITE_TEXT_RANGE {
    #[inline]
    fn from_u64(v: u64) -> Self {
        Self { startPosition: v as u32, length: (v >> 32) as u32 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_TRIMMING {
    pub granularity: u32,
    pub delimiter: u32,
    pub delimiterCount: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: u32,
    pub parameter: u32,
}

impl DWRITE_FONT_FEATURE {
    #[inline]
    fn from_u64(v: u64) -> Self {
        Self { nameTag: v as u32, parameter: (v >> 32) as u32 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Default for DWRITE_MATRIX {
    fn default() -> Self {
        Self { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 0.0, dy: 0.0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: BOOL,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_METRICS {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub widthIncludingTrailingWhitespace: f32,
    pub height: f32,
    pub layoutWidth: f32,
    pub layoutHeight: f32,
    pub maxBidiReorderingDepth: u32,
    pub lineCount: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: BOOL,
    pub isTrimmed: BOOL,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: *mut c_void,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *const u16,
    pub glyphAdvances: *const f32,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: BOOL,
    pub bidiLevel: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: *const u16,
    pub string: *const u16,
    pub stringLength: u32,
    pub _pad0: u32,
    pub clusterMap: *const u16,
    pub textPosition: u32,
    pub _pad1: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DWRITE_UNDERLINE {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub runHeight: f32,
    pub readingDirection: u32,
    pub flowDirection: u32,
    pub localeName: *const u16,
    pub measuringMode: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DWRITE_STRIKETHROUGH {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub runHeight: f32,
    pub readingDirection: u32,
    pub flowDirection: u32,
    pub localeName: *const u16,
    pub measuringMode: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_INLINE_OBJECT_METRICS {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supportsSideways: BOOL,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct LOGFONTW {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [u16; 32],
}

impl Default for LOGFONTW {
    fn default() -> Self {
        Self {
            lfHeight: 0,
            lfWidth: 0,
            lfEscapement: 0,
            lfOrientation: 0,
            lfWeight: 400,
            lfItalic: 0,
            lfUnderline: 0,
            lfStrikeOut: 0,
            lfCharSet: 1, // DEFAULT_CHARSET
            lfOutPrecision: 0,
            lfClipPrecision: 0,
            lfQuality: 0,
            lfPitchAndFamily: 0,
            lfFaceName: [0u16; 32],
        }
    }
}

// ============================================================================
// Unaligned Guest Memory & COM Object Infrastructure
// ============================================================================

#[inline]
unsafe fn read_wide_z(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut buf = Vec::new();
    let mut i = 0usize;
    loop {
        let u = p.add(i).read_unaligned();
        if u == 0 {
            break;
        }
        buf.push(u);
        i += 1;
        if i >= 65_536 {
            break;
        }
    }
    String::from_utf16_lossy(&buf)
}

#[inline]
unsafe fn read_wide_slice(p: *const u16, len: usize) -> Vec<u16> {
    if p.is_null() || len == 0 {
        return Vec::new();
    }
    let mut buf = Vec::with_capacity(len);
    for i in 0..len {
        buf.push(p.add(i).read_unaligned());
    }
    buf
}

#[inline]
unsafe fn write_wide_z(s: &str, dst: *mut u16, cap: u32) -> HRESULT {
    if dst.is_null() {
        return E_POINTER;
    }
    let units: Vec<u16> = s.encode_utf16().collect();
    if (cap as usize) < units.len() + 1 {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    for (i, &u) in units.iter().enumerate() {
        dst.add(i).write_unaligned(u);
    }
    dst.add(units.len()).write_unaligned(0);
    S_OK
}

#[repr(C)]
struct ComObj<T> {
    vtbl: *const usize,
    refs: AtomicU32,
    inner: T,
}

impl<T> ComObj<T> {
    fn new(vtbl: *const usize, inner: T) -> *mut Self {
        Box::into_raw(Box::new(Self { vtbl, refs: AtomicU32::new(1), inner }))
    }

    #[inline]
    unsafe fn inner<'a>(this: *mut Self) -> &'a T {
        &(&*this).inner
    }

    #[inline]
    unsafe fn add_ref(this: *mut Self) -> u32 {
        if this.is_null() {
            return 0;
        }
        (&*this).refs.fetch_add(1, Ordering::AcqRel) + 1
    }

    #[inline]
    unsafe fn release(this: *mut Self) -> u32 {
        if this.is_null() {
            return 0;
        }
        let prev = (&*this).refs.fetch_sub(1, Ordering::AcqRel);
        if prev == 1 {
            drop(Box::from_raw(this));
            0
        } else {
            prev - 1
        }
    }
}

fn register_vtable<const N: usize>(slots: [usize; N]) -> [usize; N] {
    for &addr in &slots {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
    slots
}

unsafe fn qi_helper<T>(
    this: *mut ComObj<T>,
    riid: *const u8,
    ppv: *mut *mut c_void,
    iids: &[GUID],
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    ppv.write_unaligned(std::ptr::null_mut());
    if this.is_null() || riid.is_null() {
        return E_INVALIDARG;
    }
    let iid = (riid as *const GUID).read_unaligned();
    if iid == IID_IUNKNOWN || iids.iter().any(|g| *g == iid) {
        ComObj::add_ref(this);
        ppv.write_unaligned(this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

/// Safe `IUnknown::AddRef` on an arbitrary guest/host COM pointer.
unsafe fn raw_com_add_ref(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let vtbl = (ptr as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return;
    }
    let f = vtbl.add(1).read_unaligned();
    if f != 0 {
        crate::call_guest8(f, ptr as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

/// Safe `IUnknown::Release` on an arbitrary guest/host COM pointer.
unsafe fn raw_com_release(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let vtbl = (ptr as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return;
    }
    let f = vtbl.add(2).read_unaligned();
    if f != 0 {
        crate::call_guest8(f, ptr as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

/// Wrapper keeping a COM interface pointer refcounted cleanly on Drop.
#[derive(Debug)]
struct ComPtr(*mut c_void);

unsafe impl Send for ComPtr {}
unsafe impl Sync for ComPtr {}

impl ComPtr {
    unsafe fn retain(p: *mut c_void) -> Option<Self> {
        if p.is_null() {
            None
        } else {
            raw_com_add_ref(p);
            Some(Self(p))
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.0
    }
}

impl Clone for ComPtr {
    fn clone(&self) -> Self {
        unsafe { raw_com_add_ref(self.0) };
        Self(self.0)
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        unsafe { raw_com_release(self.0) };
    }
}

// ============================================================================
// Pure-Rust TrueType / OpenType SFNT & TTC Parser + Glyph Outline Extractor
// ============================================================================

#[inline]
fn be_u16(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(u16::from_be_bytes([s[0], s[1]]))
}

#[inline]
fn be_i16(b: &[u8], off: usize) -> Option<i16> {
    be_u16(b, off).map(|v| v as i16)
}

#[inline]
fn be_u32(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}

fn read_at_vec(file: &File, offset: u64, len: usize) -> Option<Vec<u8>> {
    if len == 0 || len > 16 * 1024 * 1024 {
        return None;
    }
    let mut buf = vec![0u8; len];
    file.read_exact_at(&mut buf, offset).ok()?;
    Some(buf)
}

fn default_fallback_metrics() -> DWRITE_FONT_METRICS {
    DWRITE_FONT_METRICS {
        designUnitsPerEm: 2048,
        ascent: 1854,
        descent: 434,
        lineGap: 67,
        capHeight: 1434,
        xHeight: 1024,
        underlinePosition: -217,
        underlineThickness: 105,
        strikethroughPosition: 530,
        strikethroughThickness: 105,
    }
}

fn decode_name_record(data: &[u8], platform_id: u16) -> Option<String> {
    if platform_id == 3 || platform_id == 0 {
        if data.len() < 2 || (data.len() & 1) != 0 {
            return None;
        }
        let mut u16s = Vec::with_capacity(data.len() / 2);
        for chunk in data.chunks_exact(2) {
            u16s.push(u16::from_be_bytes([chunk[0], chunk[1]]));
        }
        let s = String::from_utf16_lossy(&u16s);
        let trimmed = s.trim_matches('\0').trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    } else if platform_id == 1 {
        let s: String = data.iter().map(|&c| c as char).collect();
        let trimmed = s.trim_matches('\0').trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    } else {
        None
    }
}

fn parse_name_table(tbl: &[u8]) -> HashMap<u16, String> {
    let mut best: HashMap<u16, (u8, String)> = HashMap::new();
    let Some(count) = be_u16(tbl, 2) else { return HashMap::new() };
    let Some(string_offset) = be_u16(tbl, 4) else { return HashMap::new() };
    let str_base = string_offset as usize;
    for i in 0..(count as usize) {
        let rec = 6 + i * 12;
        if rec + 12 > tbl.len() {
            break;
        }
        let platform_id = be_u16(tbl, rec).unwrap_or(0);
        let encoding_id = be_u16(tbl, rec + 2).unwrap_or(0);
        let language_id = be_u16(tbl, rec + 4).unwrap_or(0);
        let name_id = be_u16(tbl, rec + 6).unwrap_or(0);
        let length = be_u16(tbl, rec + 8).unwrap_or(0) as usize;
        let offset = be_u16(tbl, rec + 10).unwrap_or(0) as usize;
        let prio: u8 = match (platform_id, encoding_id, language_id) {
            (3, 1 | 10, 0x0409) => 5,
            (3, 1 | 10, _) => 4,
            (0, _, _) => 3,
            (1, 0, 0) => 2,
            (1, _, _) => 1,
            _ => continue,
        };
        let start = str_base.saturating_add(offset);
        let end = start.saturating_add(length);
        let Some(slice) = tbl.get(start..end) else { continue };
        let Some(decoded) = decode_name_record(slice, platform_id) else { continue };
        let replace = match best.get(&name_id) {
            Some((old_p, _)) => prio > *old_p,
            None => true,
        };
        if replace {
            best.insert(name_id, (prio, decoded));
        }
    }
    best.into_iter().map(|(k, (_, v))| (k, v)).collect()
}

#[derive(Clone, Debug)]
struct FontMeta {
    family_name: String,
    face_name: String,
    weight: u32,
    stretch: u32,
    style: u32,
    is_symbol: bool,
    is_monospace: bool,
    face_type: u32,
    face_index: u32,
    glyph_count: u16,
    metrics: DWRITE_FONT_METRICS,
    info_strings: HashMap<u32, String>,
    file_path: Option<PathBuf>,
}

fn parse_sfnt_face_meta(
    file: &File,
    sfnt_offset: u64,
    face_index: u32,
    is_ttc: bool,
    path: &Path,
) -> Option<FontMeta> {
    let hdr = read_at_vec(file, sfnt_offset, 12)?;
    let sfnt_ver = be_u32(&hdr, 0)?;
    let face_type = match sfnt_ver {
        0x0001_0000 | 0x7472_7565 => {
            if is_ttc {
                DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION
            } else {
                DWRITE_FONT_FACE_TYPE_TRUETYPE
            }
        }
        0x4F54_544F => DWRITE_FONT_FACE_TYPE_CFF, // "OTTO"
        _ => return None,
    };
    let num_tables = be_u16(&hdr, 4)? as usize;
    if num_tables == 0 || num_tables > 256 {
        return None;
    }
    let dir = read_at_vec(file, sfnt_offset + 12, num_tables * 16)?;
    let mut tables: HashMap<[u8; 4], (u64, usize)> = HashMap::with_capacity(num_tables);
    for i in 0..num_tables {
        let base = i * 16;
        let tag = [dir[base], dir[base + 1], dir[base + 2], dir[base + 3]];
        let off = be_u32(&dir, base + 8)? as u64;
        let len = be_u32(&dir, base + 12)? as usize;
        tables.insert(tag, (off, len));
    }

    let load_tbl = |tag: &[u8; 4]| -> Option<Vec<u8>> {
        let &(off, len) = tables.get(tag)?;
        read_at_vec(file, off, len.min(256 * 1024))
    };

    let head = load_tbl(b"head")?;
    let upem = be_u16(&head, 18).unwrap_or(2048).max(16);
    let mac_style = be_u16(&head, 44).unwrap_or(0);

    let hhea = load_tbl(b"hhea");
    let maxp = load_tbl(b"maxp");
    let os2 = load_tbl(b"OS/2");
    let post = load_tbl(b"post");
    let name_tbl = load_tbl(b"name");

    let glyph_count = maxp.as_deref().and_then(|b| be_u16(b, 4)).unwrap_or(256).max(1);

    let hhea_asc = hhea.as_deref().and_then(|b| be_i16(b, 4)).unwrap_or(((upem as i32 * 80) / 100) as i16);
    let hhea_desc = hhea.as_deref().and_then(|b| be_i16(b, 6)).unwrap_or(-((upem as i32 * 20) / 100) as i16);
    let hhea_gap = hhea.as_deref().and_then(|b| be_i16(b, 8)).unwrap_or(0);

    let mut weight = if (mac_style & 1) != 0 { DWRITE_FONT_WEIGHT_BOLD } else { DWRITE_FONT_WEIGHT_NORMAL };
    let mut stretch = DWRITE_FONT_STRETCH_NORMAL;
    let mut style = if (mac_style & 2) != 0 { DWRITE_FONT_STYLE_ITALIC } else { DWRITE_FONT_STYLE_NORMAL };
    let mut ascent = hhea_asc.max(0) as u16;
    let mut descent = hhea_desc.unsigned_abs();
    let mut line_gap = hhea_gap;
    let mut cap_height = ((upem as u32 * 70) / 100) as u16;
    let mut x_height = ((upem as u32 * 50) / 100) as u16;
    let mut strike_pos = ((upem as i32 * 26) / 100) as i16;
    let mut strike_thick = ((upem as u32 * 5) / 100).max(1) as u16;

    if let Some(ref b) = os2 {
        let ver = be_u16(b, 0).unwrap_or(0);
        if let Some(w) = be_u16(b, 4) {
            if (100..=999).contains(&w) {
                weight = w as u32;
            }
        }
        if let Some(st) = be_u16(b, 6) {
            if (1..=9).contains(&st) {
                stretch = st as u32;
            }
        }
        if let Some(ss) = be_i16(b, 26) {
            if ss > 0 {
                strike_thick = ss as u16;
            }
        }
        if let Some(sp) = be_i16(b, 28) {
            strike_pos = sp;
        }
        let fs_sel = be_u16(b, 62).unwrap_or(0);
        if (fs_sel & 1) != 0 {
            style = DWRITE_FONT_STYLE_ITALIC;
        } else if (fs_sel & (1 << 9)) != 0 {
            style = DWRITE_FONT_STYLE_OBLIQUE;
        }
        if (fs_sel & (1 << 5)) != 0 && weight < 600 {
            weight = DWRITE_FONT_WEIGHT_BOLD;
        }
        let use_typo = (fs_sel & (1 << 7)) != 0;
        if use_typo {
            if let Some(ta) = be_i16(b, 68) {
                ascent = ta.max(0) as u16;
            }
            if let Some(td) = be_i16(b, 70) {
                descent = td.unsigned_abs();
            }
            if let Some(tg) = be_i16(b, 72) {
                line_gap = tg;
            }
        } else {
            if let Some(wa) = be_u16(b, 74) {
                if wa > 0 {
                    ascent = wa;
                }
            }
            if let Some(wd) = be_u16(b, 76) {
                if wd > 0 {
                    descent = wd;
                }
            }
        }
        if ver >= 2 {
            if let Some(xh) = be_i16(b, 86) {
                if xh > 0 {
                    x_height = xh as u16;
                }
            }
            if let Some(ch) = be_i16(b, 88) {
                if ch > 0 {
                    cap_height = ch as u16;
                }
            }
        }
    }

    let mut underline_pos = -((upem as i32 * 10) / 100) as i16;
    let mut underline_thick = ((upem as u32 * 5) / 100).max(1) as u16;
    let mut is_monospace = false;
    if let Some(ref b) = post {
        if let Some(up) = be_i16(b, 8) {
            underline_pos = up;
        }
        if let Some(ut) = be_i16(b, 10) {
            if ut > 0 {
                underline_thick = ut as u16;
            }
        }
        if let Some(fp) = be_u32(b, 12) {
            is_monospace = fp != 0;
        }
    }

    let names = name_tbl.as_deref().map(parse_name_table).unwrap_or_default();
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown").to_string();
    let family_name = names
        .get(&16)
        .or_else(|| names.get(&1))
        .cloned()
        .unwrap_or(stem);
    let face_name = names
        .get(&17)
        .or_else(|| names.get(&2))
        .cloned()
        .unwrap_or_else(|| "Regular".to_string());

    let lower_fam = family_name.to_ascii_lowercase();
    if lower_fam.contains("mono") || lower_fam.contains("courier") || lower_fam.contains("menlo") || lower_fam.contains("consolas") {
        is_monospace = true;
    }
    let is_symbol = lower_fam.contains("symbol") || lower_fam.contains("wingding") || lower_fam.contains("webding");

    let mut info_strings = HashMap::new();
    for &(name_id, info_id) in &[
        (0u16, DWRITE_INFO_COPYRIGHT_NOTICE),
        (5, DWRITE_INFO_VERSION_STRINGS),
        (7, DWRITE_INFO_TRADEMARK),
        (8, DWRITE_INFO_MANUFACTURER),
        (9, DWRITE_INFO_DESIGNER),
        (10, DWRITE_INFO_DESCRIPTION),
        (13, DWRITE_INFO_LICENSE_DESCRIPTION),
        (4, DWRITE_INFO_FULL_NAME),
        (6, DWRITE_INFO_POSTSCRIPT_NAME),
    ] {
        if let Some(v) = names.get(&name_id) {
            info_strings.insert(info_id, v.clone());
        }
    }

    Some(FontMeta {
        family_name,
        face_name,
        weight,
        stretch,
        style,
        is_symbol,
        is_monospace,
        face_type,
        face_index,
        glyph_count,
        metrics: DWRITE_FONT_METRICS {
            designUnitsPerEm: upem,
            ascent,
            descent,
            lineGap: line_gap,
            capHeight: cap_height,
            xHeight: x_height,
            underlinePosition: underline_pos,
            underlineThickness: underline_thick,
            strikethroughPosition: strike_pos,
            strikethroughThickness: strike_thick,
        },
        info_strings,
        file_path: Some(path.to_path_buf()),
    })
}

fn parse_font_file_metadata(path: &Path) -> Vec<FontMeta> {
    let Ok(file) = File::open(path) else { return Vec::new() };
    let Some(hdr) = read_at_vec(&file, 0, 12) else { return Vec::new() };
    let Some(tag) = be_u32(&hdr, 0) else { return Vec::new() };
    if tag == 0x7474_6366 {
        // "ttcf" TrueType Collection
        let num_fonts = be_u32(&hdr, 8).unwrap_or(0).min(32) as usize;
        let Some(offs_buf) = read_at_vec(&file, 12, num_fonts * 4) else {
            return Vec::new();
        };
        let mut out = Vec::with_capacity(num_fonts);
        for i in 0..num_fonts {
            if let Some(off) = be_u32(&offs_buf, i * 4) {
                if let Some(meta) = parse_sfnt_face_meta(&file, off as u64, i as u32, true, path) {
                    out.push(meta);
                }
            }
        }
        out
    } else {
        parse_sfnt_face_meta(&file, 0, 0, false, path).into_iter().collect()
    }
}

/// Decoded SFNT tables for a loaded font face.
struct SfntFace {
    raw: Arc<Vec<u8>>,
    tables: HashMap<[u8; 4], (usize, usize)>,
    upem: u16,
    index_to_loc_format: i16,
    num_hmetrics: u16,
    cmap_subtable: Option<(u16, usize)>, // (format, byte offset into raw)
    /// Reverse map from glyph ID to Unicode codepoint for CFF fallback rendering.
    glyph_to_char: HashMap<u16, char>,
}

impl SfntFace {
    fn from_bytes(raw: Arc<Vec<u8>>, face_index: u32) -> Option<Self> {
        let b = raw.as_slice();
        let magic = be_u32(b, 0)?;
        let sfnt_off = if magic == 0x7474_6366 {
            let num = be_u32(b, 8)?;
            if face_index >= num {
                return None;
            }
            be_u32(b, 12 + (face_index as usize) * 4)? as usize
        } else {
            if face_index != 0 {
                return None;
            }
            0
        };

        let ver = be_u32(b, sfnt_off)?;
        if ver != 0x0001_0000 && ver != 0x7472_7565 && ver != 0x4F54_544F {
            return None;
        }
        let num_tables = be_u16(b, sfnt_off + 4)? as usize;
        if num_tables == 0 || num_tables > 256 {
            return None;
        }
        let mut tables = HashMap::with_capacity(num_tables);
        for i in 0..num_tables {
            let rec = sfnt_off + 12 + i * 16;
            if rec + 16 > b.len() {
                break;
            }
            let tag = [b[rec], b[rec + 1], b[rec + 2], b[rec + 3]];
            let off = be_u32(b, rec + 8)? as usize;
            let len = be_u32(b, rec + 12)? as usize;
            if off.saturating_add(len) <= b.len() {
                tables.insert(tag, (off, len));
            }
        }

        let head = tables.get(b"head").and_then(|&(o, l)| b.get(o..o + l))?;
        let upem = be_u16(head, 18).unwrap_or(2048).max(16);
        let index_to_loc_format = be_i16(head, 50).unwrap_or(1);

        let num_hmetrics = tables
            .get(b"hhea")
            .and_then(|&(o, l)| b.get(o..o + l))
            .and_then(|t| be_u16(t, 34))
            .unwrap_or(1)
            .max(1);

        let cmap_subtable = Self::pick_cmap(b, &tables);
        let mut face = Self {
            raw,
            tables,
            upem,
            index_to_loc_format,
            num_hmetrics,
            cmap_subtable,
            glyph_to_char: HashMap::new(),
        };
        // Populate ASCII reverse map so CFF fonts without `glyf` can render crisp fallback bitmaps.
        for cp in 0x20u32..=0x7Eu32 {
            let gid = face.codepoint_to_glyph(cp);
            if gid != 0 {
                if let Some(ch) = char::from_u32(cp) {
                    face.glyph_to_char.entry(gid).or_insert(ch);
                }
            }
        }
        Some(face)
    }

    fn pick_cmap(b: &[u8], tables: &HashMap<[u8; 4], (usize, usize)>) -> Option<(u16, usize)> {
        let &(cmap_off, cmap_len) = tables.get(b"cmap")?;
        let cmap = b.get(cmap_off..cmap_off + cmap_len)?;
        let num_recs = be_u16(cmap, 2)? as usize;
        let mut best: Option<(u8, u16, usize)> = None;
        for i in 0..num_recs {
            let rec = 4 + i * 8;
            if rec + 8 > cmap.len() {
                break;
            }
            let platform = be_u16(cmap, rec)?;
            let encoding = be_u16(cmap, rec + 2)?;
            let sub_rel = be_u32(cmap, rec + 4)? as usize;
            let sub_abs = cmap_off.checked_add(sub_rel)?;
            let fmt = be_u16(b, sub_abs)?;
            let prio = match (platform, encoding, fmt) {
                (3, 10, 12) => 10,
                (0, 4 | 6, 12) => 9,
                (_, _, 12) => 8,
                (3, 1, 4) => 7,
                (0, 3, 4) => 6,
                (3, 0, 4) => 5,
                (_, _, 4) => 4,
                (_, _, 0) => 2,
                _ => continue,
            };
            if best.is_none_or(|(bp, _, _)| prio > bp) {
                best = Some((prio, fmt, sub_abs));
            }
        }
        best.map(|(_, fmt, off)| (fmt, off))
    }

    fn codepoint_to_glyph(&self, cp: u32) -> u16 {
        let Some((fmt, off)) = self.cmap_subtable else { return 0 };
        let b = self.raw.as_slice();
        match fmt {
            12 => {
                let n_groups = be_u32(b, off + 12).unwrap_or(0) as usize;
                let mut lo = 0usize;
                let mut hi = n_groups;
                while lo < hi {
                    let mid = lo + (hi - lo) / 2;
                    let g = off + 16 + mid * 12;
                    let Some(start) = be_u32(b, g) else { break };
                    let Some(end) = be_u32(b, g + 4) else { break };
                    if cp < start {
                        hi = mid;
                    } else if cp > end {
                        lo = mid + 1;
                    } else {
                        let start_gid = be_u32(b, g + 8).unwrap_or(0);
                        return (start_gid.wrapping_add(cp - start)) as u16;
                    }
                }
                0
            }
            4 => {
                if cp > 0xFFFF {
                    return 0;
                }
                let c = cp as u16;
                let seg_count_x2 = be_u16(b, off + 6).unwrap_or(0) as usize;
                let seg_count = seg_count_x2 / 2;
                let end_base = off + 14;
                let start_base = end_base + seg_count_x2 + 2;
                let delta_base = start_base + seg_count_x2;
                let range_base = delta_base + seg_count_x2;
                for i in 0..seg_count {
                    let Some(end_c) = be_u16(b, end_base + i * 2) else { break };
                    if c > end_c {
                        continue;
                    }
                    let Some(start_c) = be_u16(b, start_base + i * 2) else { break };
                    if c < start_c {
                        return 0;
                    }
                    let delta = be_i16(b, delta_base + i * 2).unwrap_or(0);
                    let range_off_pos = range_base + i * 2;
                    let range_off = be_u16(b, range_off_pos).unwrap_or(0) as usize;
                    if range_off == 0 {
                        return (c as i32 + delta as i32) as u16;
                    } else {
                        let glyph_addr = range_off_pos + range_off + ((c - start_c) as usize) * 2;
                        let gid = be_u16(b, glyph_addr).unwrap_or(0);
                        if gid == 0 {
                            return 0;
                        }
                        return (gid as i32 + delta as i32) as u16;
                    }
                }
                0
            }
            0 => {
                if cp < 256 {
                    b.get(off + 6 + cp as usize).copied().unwrap_or(0) as u16
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn hmtx(&self, gid: u16) -> Option<(u16, i16)> {
        let &(off, len) = self.tables.get(b"hmtx")?;
        let tbl = self.raw.get(off..off + len)?;
        let nh = self.num_hmetrics as usize;
        let g = gid as usize;
        if g < nh {
            let adv = be_u16(tbl, g * 4)?;
            let lsb = be_i16(tbl, g * 4 + 2)?;
            Some((adv, lsb))
        } else {
            let adv = be_u16(tbl, nh.saturating_sub(1) * 4)?;
            let lsb_off = nh * 4 + (g - nh) * 2;
            let lsb = be_i16(tbl, lsb_off).unwrap_or(0);
            Some((adv, lsb))
        }
    }

    fn glyf_slice(&self, gid: u16) -> Option<&[u8]> {
        let &(loca_off, loca_len) = self.tables.get(b"loca")?;
        let &(glyf_off, glyf_len) = self.tables.get(b"glyf")?;
        let loca = self.raw.get(loca_off..loca_off + loca_len)?;
        let glyf = self.raw.get(glyf_off..glyf_off + glyf_len)?;
        let g = gid as usize;
        let (start, end) = if self.index_to_loc_format == 0 {
            let s = (be_u16(loca, g * 2)? as usize) * 2;
            let e = (be_u16(loca, (g + 1) * 2)? as usize) * 2;
            (s, e)
        } else {
            let s = be_u32(loca, g * 4)? as usize;
            let e = be_u32(loca, (g + 1) * 4)? as usize;
            (s, e)
        };
        if start >= end || end > glyf.len() {
            return None;
        }
        glyf.get(start..end)
    }

    fn glyph_bbox(&self, gid: u16) -> Option<(i16, i16, i16, i16)> {
        let g = self.glyf_slice(gid)?;
        if g.len() < 10 {
            return None;
        }
        Some((be_i16(g, 2)?, be_i16(g, 4)?, be_i16(g, 6)?, be_i16(g, 8)?))
    }

    fn glyph_contours(&self, gid: u16, depth: u32) -> Option<Vec<Vec<(f32, f32, bool)>>> {
        if depth > 6 {
            return None;
        }
        let g = self.glyf_slice(gid)?;
        if g.len() < 10 {
            return None;
        }
        let n_contours = be_i16(g, 0)?;
        if n_contours == 0 {
            return Some(Vec::new());
        }
        if n_contours > 0 {
            let nc = n_contours as usize;
            if 10 + nc * 2 + 2 > g.len() {
                return None;
            }
            let mut end_pts = Vec::with_capacity(nc);
            for i in 0..nc {
                end_pts.push(be_u16(g, 10 + i * 2)? as usize);
            }
            let num_pts = end_pts.last().copied()? + 1;
            if num_pts > 16_384 {
                return None;
            }
            let instr_len = be_u16(g, 10 + nc * 2)? as usize;
            let mut cur = 10 + nc * 2 + 2 + instr_len;
            if cur > g.len() {
                return None;
            }
            let mut flags = Vec::with_capacity(num_pts);
            while flags.len() < num_pts {
                let f = *g.get(cur)?;
                cur += 1;
                flags.push(f);
                if (f & 0x08) != 0 {
                    let repeat = *g.get(cur)? as usize;
                    cur += 1;
                    for _ in 0..repeat {
                        if flags.len() >= num_pts {
                            break;
                        }
                        flags.push(f);
                    }
                }
            }
            let mut xs = vec![0i32; num_pts];
            let mut x = 0i32;
            for i in 0..num_pts {
                let f = flags[i];
                if (f & 0x02) != 0 {
                    let dx = *g.get(cur)? as i32;
                    cur += 1;
                    x += if (f & 0x10) != 0 { dx } else { -dx };
                } else if (f & 0x10) == 0 {
                    let dx = be_i16(g, cur)? as i32;
                    cur += 2;
                    x += dx;
                }
                xs[i] = x;
            }
            let mut ys = vec![0i32; num_pts];
            let mut y = 0i32;
            for i in 0..num_pts {
                let f = flags[i];
                if (f & 0x04) != 0 {
                    let dy = *g.get(cur)? as i32;
                    cur += 1;
                    y += if (f & 0x20) != 0 { dy } else { -dy };
                } else if (f & 0x20) == 0 {
                    let dy = be_i16(g, cur)? as i32;
                    cur += 2;
                    y += dy;
                }
                ys[i] = y;
            }
            let mut contours = Vec::with_capacity(nc);
            let mut start = 0usize;
            for end in end_pts {
                if end < start || end >= num_pts {
                    return None;
                }
                let mut c = Vec::with_capacity(end - start + 1);
                for idx in start..=end {
                    c.push((xs[idx] as f32, ys[idx] as f32, (flags[idx] & 0x01) != 0));
                }
                contours.push(c);
                start = end + 1;
            }
            Some(contours)
        } else {
            // Compound glyph
            let mut cur = 10usize;
            let mut all_contours = Vec::new();
            loop {
                let comp_flags = be_u16(g, cur)?;
                let comp_gid = be_u16(g, cur + 2)?;
                cur += 4;
                let (arg1, arg2): (i32, i32) = if (comp_flags & 0x0001) != 0 {
                    let a1 = be_i16(g, cur)? as i32;
                    let a2 = be_i16(g, cur + 2)? as i32;
                    cur += 4;
                    (a1, a2)
                } else {
                    let a1 = (*g.get(cur)? as i8) as i32;
                    let a2 = (*g.get(cur + 1)? as i8) as i32;
                    cur += 2;
                    (a1, a2)
                };
                let (mut m00, mut m01, mut m10, mut m11) = (1.0f32, 0.0f32, 0.0f32, 1.0f32);
                if (comp_flags & 0x0008) != 0 {
                    let s = (be_i16(g, cur)? as f32) / 16384.0;
                    cur += 2;
                    m00 = s;
                    m11 = s;
                } else if (comp_flags & 0x0040) != 0 {
                    m00 = (be_i16(g, cur)? as f32) / 16384.0;
                    m11 = (be_i16(g, cur + 2)? as f32) / 16384.0;
                    cur += 4;
                } else if (comp_flags & 0x0080) != 0 {
                    m00 = (be_i16(g, cur)? as f32) / 16384.0;
                    m01 = (be_i16(g, cur + 2)? as f32) / 16384.0;
                    m10 = (be_i16(g, cur + 4)? as f32) / 16384.0;
                    m11 = (be_i16(g, cur + 6)? as f32) / 16384.0;
                    cur += 8;
                }
                let (dx, dy) = if (comp_flags & 0x0002) != 0 {
                    (arg1 as f32, arg2 as f32)
                } else {
                    (0.0, 0.0)
                };
                if let Some(sub) = self.glyph_contours(comp_gid, depth + 1) {
                    for contour in sub {
                        let transformed: Vec<(f32, f32, bool)> = contour
                            .into_iter()
                            .map(|(px, py, on)| (px * m00 + py * m10 + dx, px * m01 + py * m11 + dy, on))
                            .collect();
                        all_contours.push(transformed);
                    }
                }
                if (comp_flags & 0x0020) == 0 {
                    break;
                }
            }
            Some(all_contours)
        }
    }

    fn try_get_table(&self, open_type_tag: u32) -> Option<(usize, usize)> {
        let tag = open_type_tag.to_le_bytes();
        self.tables.get(&tag).copied()
    }
}

fn load_cached_file_bytes(path: &Path) -> Option<Arc<Vec<u8>>> {
    static CACHE: OnceLock<Mutex<HashMap<PathBuf, Arc<Vec<u8>>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(guard) = cache.lock() {
        if let Some(bytes) = guard.get(path) {
            return Some(Arc::clone(bytes));
        }
    }
    let data = Arc::new(fs::read(path).ok()?);
    if let Ok(mut guard) = cache.lock() {
        guard.insert(path.to_path_buf(), Arc::clone(&data));
    }
    Some(data)
}

// ============================================================================
// Built-in 8x8 ASCII Bitmap Font & TrueType Scanline Glyph Rasterizer
// ============================================================================

/// 8x8 monochrome bitmap glyphs for ASCII 0x20..=0x7E (MSB = leftmost pixel).
const FALLBACK_FONT_8X8: [[u8; 8]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ' '
    [0x18, 0x3C, 0x3C, 0x18, 0x18, 0x00, 0x18, 0x00], // '!'
    [0x66, 0x66, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00], // '"'
    [0x6C, 0x6C, 0xFE, 0x6C, 0xFE, 0x6C, 0x6C, 0x00], // '#'
    [0x18, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0x00], // '$'
    [0x00, 0xC6, 0xCC, 0x18, 0x30, 0x66, 0xC6, 0x00], // '%'
    [0x38, 0x6C, 0x38, 0x76, 0xDC, 0xCC, 0x76, 0x00], // '&'
    [0x18, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00], // '''
    [0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0x00], // '('
    [0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x00], // ')'
    [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00], // '*'
    [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00], // '+'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30], // ','
    [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00], // '-'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00], // '.'
    [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00], // '/'
    [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00], // '0'
    [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00], // '1'
    [0x3C, 0x66, 0x06, 0x1C, 0x30, 0x60, 0x7E, 0x00], // '2'
    [0x7E, 0x0C, 0x18, 0x1C, 0x06, 0x66, 0x3C, 0x00], // '3'
    [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x0C, 0x00], // '4'
    [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00], // '5'
    [0x3C, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x3C, 0x00], // '6'
    [0x7E, 0x66, 0x0C, 0x18, 0x18, 0x18, 0x18, 0x00], // '7'
    [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00], // '8'
    [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00], // '9'
    [0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x00], // ':'
    [0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x30, 0x00], // ';'
    [0x0C, 0x18, 0x30, 0x60, 0x30, 0x18, 0x0C, 0x00], // '<'
    [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00], // '='
    [0x30, 0x18, 0x0C, 0x06, 0x0C, 0x18, 0x30, 0x00], // '>'
    [0x3C, 0x66, 0x06, 0x1C, 0x18, 0x00, 0x18, 0x00], // '?'
    [0x3C, 0x66, 0x6E, 0x6E, 0x60, 0x62, 0x3C, 0x00], // '@'
    [0x18, 0x3C, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00], // 'A'
    [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00], // 'B'
    [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00], // 'C'
    [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00], // 'D'
    [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x7E, 0x00], // 'E'
    [0x7E, 0x60, 0x60, 0x78, 0x60, 0x60, 0x60, 0x00], // 'F'
    [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3E, 0x00], // 'G'
    [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00], // 'H'
    [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00], // 'I'
    [0x1E, 0x0C, 0x0C, 0x0C, 0x6C, 0x6C, 0x38, 0x00], // 'J'
    [0x66, 0x6C, 0x78, 0x70, 0x78, 0x6C, 0x66, 0x00], // 'K'
    [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00], // 'L'
    [0xC6, 0xEE, 0xFE, 0xD6, 0xC6, 0xC6, 0xC6, 0x00], // 'M'
    [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00], // 'N'
    [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00], // 'O'
    [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00], // 'P'
    [0x3C, 0x66, 0x66, 0x66, 0x6A, 0x6C, 0x36, 0x00], // 'Q'
    [0x7C, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0x66, 0x00], // 'R'
    [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00], // 'S'
    [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00], // 'T'
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00], // 'U'
    [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00], // 'V'
    [0xC6, 0xC6, 0xC6, 0xD6, 0xFE, 0xEE, 0xC6, 0x00], // 'W'
    [0x66, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x66, 0x00], // 'X'
    [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00], // 'Y'
    [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7E, 0x00], // 'Z'
    [0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00], // '['
    [0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x02, 0x00], // '\'
    [0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00], // ']'
    [0x18, 0x3C, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00], // '^'
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF], // '_'
    [0x30, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00], // '`'
    [0x00, 0x00, 0x3C, 0x06, 0x3E, 0x66, 0x3E, 0x00], // 'a'
    [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x7C, 0x00], // 'b'
    [0x00, 0x00, 0x3C, 0x60, 0x60, 0x60, 0x3C, 0x00], // 'c'
    [0x06, 0x06, 0x3E, 0x66, 0x66, 0x66, 0x3E, 0x00], // 'd'
    [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x00], // 'e'
    [0x1C, 0x30, 0x7C, 0x30, 0x30, 0x30, 0x30, 0x00], // 'f'
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x3C], // 'g'
    [0x60, 0x60, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00], // 'h'
    [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00], // 'i'
    [0x0C, 0x00, 0x1C, 0x0C, 0x0C, 0x0C, 0x6C, 0x38], // 'j'
    [0x60, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x00], // 'k'
    [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00], // 'l'
    [0x00, 0x00, 0xEC, 0xFE, 0xD6, 0xC6, 0xC6, 0x00], // 'm'
    [0x00, 0x00, 0x7C, 0x66, 0x66, 0x66, 0x66, 0x00], // 'n'
    [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x00], // 'o'
    [0x00, 0x00, 0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60], // 'p'
    [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x06], // 'q'
    [0x00, 0x00, 0x7C, 0x66, 0x60, 0x60, 0x60, 0x00], // 'r'
    [0x00, 0x00, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x00], // 's'
    [0x30, 0x30, 0x7C, 0x30, 0x30, 0x34, 0x18, 0x00], // 't'
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x00], // 'u'
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00], // 'v'
    [0x00, 0x00, 0xC6, 0xC6, 0xD6, 0xFE, 0x6C, 0x00], // 'w'
    [0x00, 0x00, 0x66, 0x3C, 0x18, 0x3C, 0x66, 0x00], // 'x'
    [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x3C], // 'y'
    [0x00, 0x00, 0x7E, 0x0C, 0x18, 0x30, 0x7E, 0x00], // 'z'
    [0x0E, 0x18, 0x18, 0x70, 0x18, 0x18, 0x0E, 0x00], // '{'
    [0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00], // '|'
    [0x70, 0x18, 0x18, 0x0E, 0x18, 0x18, 0x70, 0x00], // '}'
    [0x76, 0xDC, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // '~'
];

const FALLBACK_NOTDEF_8X8: [u8; 8] = [0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x00];

fn fallback_char_advance_du(ch: char, monospace: bool, weight: u32) -> u32 {
    if (ch as u32) < 0x20 {
        return 0;
    }
    let base: u32 = if monospace {
        1126
    } else {
        match ch {
            'i' | 'l' | '!' | '|' | '.' | ',' | ':' | ';' | '\'' | '`' | 'I' => 570,
            'f' | 'j' | 'r' | 't' | '(' | ')' | '[' | ']' | '{' | '}' | '/' | '\\' | '-' | ' ' => 680,
            'm' | 'w' => 1520,
            'M' | 'W' => 1740,
            'a'..='z' | '0'..='9' => 1040,
            'A'..='Z' => 1320,
            c if (c as u32) >= 0x1100 => 2048,
            _ => 1080,
        }
    };
    if weight >= 650 && !monospace {
        (base * 107) / 100
    } else {
        base
    }
}

fn fallback_cp_to_gid(cp: u32) -> u16 {
    if (0x20..=0x7E).contains(&cp) {
        (cp - 0x20 + 1) as u16
    } else if cp == 0x09 {
        1 // tab maps to space glyph
    } else if (0x80..=0xFFFE).contains(&cp) {
        // Stable non-zero synthetic glyph ID for non-ASCII BMP characters
        ((cp % 4000) + 96) as u16
    } else {
        0
    }
}

fn fallback_gid_to_char(gid: u16) -> Option<char> {
    if (1..=95).contains(&gid) {
        char::from_u32((gid as u32 - 1) + 0x20)
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RasterizedGlyph {
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: u32,
    pub height: u32,
    pub alpha: Vec<u8>,
}

fn flatten_quad_contour(contour: &[(f32, f32, bool)], scale: f32, italic_shear: f32) -> Vec<(f32, f32)> {
    let n = contour.len();
    if n < 2 {
        return Vec::new();
    }
    let tx = |x: f32, y: f32| -> (f32, f32) {
        let sx = (x + y * italic_shear) * scale;
        let sy = -y * scale;
        (sx, sy)
    };
    // Find an explicit or implied starting on-curve point.
    let first_on = contour.iter().position(|p| p.2);
    let mut expanded: Vec<(f32, f32, bool)> = Vec::with_capacity(n + 2);
    if let Some(idx) = first_on {
        for i in 0..n {
            expanded.push(contour[(idx + i) % n]);
        }
        expanded.push(contour[idx]);
    } else {
        // All points off-curve: start at midpoint between last and first.
        let p0 = contour[n - 1];
        let p1 = contour[0];
        let mid = ((p0.0 + p1.0) * 0.5, (p0.1 + p1.1) * 0.5, true);
        expanded.push(mid);
        expanded.extend_from_slice(contour);
        expanded.push(mid);
    }

    let mut poly = Vec::new();
    let mut i = 0usize;
    while i + 1 < expanded.len() {
        let p0 = tx(expanded[i].0, expanded[i].1);
        if poly.is_empty() {
            poly.push(p0);
        }
        if expanded[i + 1].2 {
            poly.push(tx(expanded[i + 1].0, expanded[i + 1].1));
            i += 1;
        } else {
            let ctrl = tx(expanded[i + 1].0, expanded[i + 1].1);
            let p2 = if i + 2 < expanded.len() && expanded[i + 2].2 {
                let pt = tx(expanded[i + 2].0, expanded[i + 2].1);
                i += 2;
                pt
            } else if i + 2 < expanded.len() {
                let next_off = tx(expanded[i + 2].0, expanded[i + 2].1);
                i += 1;
                ((ctrl.0 + next_off.0) * 0.5, (ctrl.1 + next_off.1) * 0.5)
            } else {
                break;
            };
            let steps = 6usize;
            for s in 1..=steps {
                let t = (s as f32) / (steps as f32);
                let mt = 1.0 - t;
                let qx = mt * mt * p0.0 + 2.0 * mt * t * ctrl.0 + t * t * p2.0;
                let qy = mt * mt * p0.1 + 2.0 * mt * t * ctrl.1 + t * t * p2.1;
                poly.push((qx, qy));
            }
        }
    }
    poly
}

fn rasterize_contours(contours: &[Vec<(f32, f32, bool)>], scale: f32, italic_shear: f32) -> RasterizedGlyph {
    let mut polys: Vec<Vec<(f32, f32)>> = Vec::with_capacity(contours.len());
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for c in contours {
        let poly = flatten_quad_contour(c, scale, italic_shear);
        if poly.len() >= 3 {
            for &(x, y) in &poly {
                if x < min_x { min_x = x; }
                if y < min_y { min_y = y; }
                if x > max_x { max_x = x; }
                if y > max_y { max_y = y; }
            }
            polys.push(poly);
        }
    }
    if polys.is_empty() || !min_x.is_finite() {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }

    let ix0 = min_x.floor() as i32 - 1;
    let iy0 = min_y.floor() as i32 - 1;
    let ix1 = max_x.ceil() as i32 + 1;
    let iy1 = max_y.ceil() as i32 + 1;
    let w = (ix1 - ix0).clamp(0, 512) as u32;
    let h = (iy1 - iy0).clamp(0, 512) as u32;
    if w == 0 || h == 0 {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }

    let mut alpha = vec![0u8; (w * h) as usize];
    let mut crossings: Vec<(f32, i32)> = Vec::new();
    for py in 0..h {
        let mut row_cov = vec![0.0f32; w as usize];
        for sub in 0..4u32 {
            let sy = (iy0 + py as i32) as f32 + (sub as f32 + 0.5) * 0.25;
            crossings.clear();
            for poly in &polys {
                for seg in poly.windows(2) {
                    let (x0, y0) = seg[0];
                    let (x1, y1) = seg[1];
                    if (y0 <= sy && y1 > sy) || (y1 <= sy && y0 > sy) {
                        let t = (sy - y0) / (y1 - y0);
                        let x = x0 + t * (x1 - x0);
                        let dir = if y1 > y0 { 1 } else { -1 };
                        crossings.push((x, dir));
                    }
                }
            }
            crossings.sort_by(|a, b| a.0.total_cmp(&b.0));
            let mut winding = 0i32;
            let mut prev_x = 0.0f32;
            for &(cx, dir) in &crossings {
                let was_inside = winding != 0;
                winding += dir;
                let is_inside = winding != 0;
                if was_inside {
                    let span_l = (prev_x - ix0 as f32).clamp(0.0, w as f32);
                    let span_r = (cx - ix0 as f32).clamp(0.0, w as f32);
                    if span_r > span_l {
                        let px_start = span_l.floor() as usize;
                        let px_end = (span_r.ceil() as usize).min(w as usize);
                        for px in px_start..px_end {
                            let l = span_l.max(px as f32);
                            let r = span_r.min((px + 1) as f32);
                            if r > l {
                                row_cov[px] += (r - l) * 0.25;
                            }
                        }
                    }
                }
                if !was_inside && is_inside {
                    prev_x = cx;
                } else if was_inside && is_inside {
                    prev_x = cx;
                }
            }
        }
        let row_base = (py * w) as usize;
        for px in 0..(w as usize) {
            alpha[row_base + px] = (row_cov[px].clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        }
    }

    RasterizedGlyph { offset_x: ix0, offset_y: iy0, width: w, height: h, alpha }
}

fn rasterize_fallback_8x8(ch: Option<char>, em_size: f32, bold: bool, italic: bool) -> RasterizedGlyph {
    if ch == Some(' ') || ch == Some('\t') || ch == Some('\n') || ch == Some('\r') {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }
    let bitmap = match ch {
        Some(c) if (0x20..=0x7E).contains(&(c as u32)) => &FALLBACK_FONT_8X8[(c as u32 - 0x20) as usize],
        _ => &FALLBACK_NOTDEF_8X8,
    };
    let gw = (em_size * 0.62).ceil().clamp(4.0, 256.0) as u32;
    let gh = (em_size * 0.88).ceil().clamp(4.0, 256.0) as u32;
    let shear_px = if italic { (gh as f32 * 0.20).ceil() as u32 } else { 0 };
    let out_w = (gw + shear_px).min(256);
    let out_h = gh.min(256);
    let mut alpha = vec![0u8; (out_w * out_h) as usize];

    let sample_bit = |u: f32, v: f32| -> f32 {
        let bx = (u * 8.0).floor() as i32;
        let by = (v * 8.0).floor() as i32;
        if !(0..8).contains(&bx) || !(0..8).contains(&by) {
            return 0.0;
        }
        let hit = (bitmap[by as usize] & (0x80u8 >> bx)) != 0;
        if hit {
            1.0
        } else if bold && bx > 0 && (bitmap[by as usize] & (0x80u8 >> (bx - 1))) != 0 {
            0.85
        } else {
            0.0
        }
    };

    for py in 0..out_h {
        let row_shear = if italic {
            (1.0 - (py as f32 / out_h as f32)) * (shear_px as f32)
        } else {
            0.0
        };
        for px in 0..out_w {
            let mut acc = 0.0f32;
            for sy in 0..2 {
                let v = (py as f32 + (sy as f32 + 0.5) * 0.5) / (out_h as f32);
                for sx in 0..2 {
                    let fx = (px as f32 + (sx as f32 + 0.5) * 0.5) - row_shear;
                    let u = fx / (gw as f32);
                    acc += sample_bit(u, v);
                }
            }
            alpha[(py * out_w + px) as usize] = ((acc * 0.25).clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        }
    }

    let ascent_px = (em_size * 0.78).round() as i32;
    RasterizedGlyph { offset_x: 0, offset_y: -ascent_px, width: out_w, height: out_h, alpha }
}

pub(crate) fn rasterize_glyph(face: &FontFaceInner, glyph_id: u16, em_size: f32) -> RasterizedGlyph {
    if em_size <= 0.5 || !em_size.is_finite() {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }
    let italic_shear = if (face.simulations & DWRITE_FONT_SIMULATIONS_OBLIQUE) != 0 { 0.21 } else { 0.0 };
    if let Some(ref sfnt) = face.sfnt {
        if let Some(contours) = sfnt.glyph_contours(glyph_id, 0) {
            if !contours.is_empty() {
                let scale = em_size / (sfnt.upem as f32);
                return rasterize_contours(&contours, scale, italic_shear);
            }
            // Space or empty TrueType glyph (e.g. ' ' has valid empty glyf slice, or no outline).
            if sfnt.glyph_to_char.get(&glyph_id) == Some(&' ') {
                return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
            }
        }
        // CFF font or glyph without `glyf` contours: fall back to built-in 8x8 bitmap for mapped char.
        let ch = sfnt.glyph_to_char.get(&glyph_id).copied();
        let bold = face.meta.weight >= 650 || (face.simulations & DWRITE_FONT_SIMULATIONS_BOLD) != 0;
        let italic = face.meta.style != DWRITE_FONT_STYLE_NORMAL
            || (face.simulations & DWRITE_FONT_SIMULATIONS_OBLIQUE) != 0;
        return rasterize_fallback_8x8(ch, em_size, bold, italic);
    }

    let ch = fallback_gid_to_char(glyph_id);
    let bold = face.meta.weight >= 650 || (face.simulations & DWRITE_FONT_SIMULATIONS_BOLD) != 0;
    let italic = face.meta.style != DWRITE_FONT_STYLE_NORMAL
        || (face.simulations & DWRITE_FONT_SIMULATIONS_OBLIQUE) != 0;
    rasterize_fallback_8x8(ch, em_size, bold, italic)
}

// ============================================================================
// System Font Collection & Built-in Default Fallback Families
// ============================================================================

#[derive(Clone, Debug)]
struct FontFamilyData {
    name: String,
    fonts: Vec<Arc<FontMeta>>,
}

struct FontCollectionData {
    families: Vec<Arc<FontFamilyData>>,
    /// Lowercase family name -> index in `families`.
    index_by_lower: HashMap<String, usize>,
}

impl FontCollectionData {
    fn build_system() -> Self {
        let mut by_family: HashMap<String, (String, Vec<Arc<FontMeta>>)> = HashMap::new();

        // Scan macOS system font directories (gracefully empty on iOS/sandboxes).
        let search_dirs = [
            "/System/Library/Fonts",
            "/System/Library/Fonts/Supplemental",
            "/Library/Fonts",
        ];
        for dir in search_dirs {
            let Ok(rd) = fs::read_dir(dir) else { continue };
            let mut paths: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
            paths.sort();
            for p in paths {
                let Some(ext) = p.extension().and_then(|e| e.to_str()) else { continue };
                if !ext.eq_ignore_ascii_case("ttf")
                    && !ext.eq_ignore_ascii_case("otf")
                    && !ext.eq_ignore_ascii_case("ttc")
                {
                    continue;
                }
                for meta in parse_font_file_metadata(&p) {
                    let key = meta.family_name.to_ascii_lowercase();
                    by_family
                        .entry(key)
                        .or_insert_with(|| (meta.family_name.clone(), Vec::new()))
                        .1
                        .push(Arc::new(meta));
                }
            }
        }

        // Always provide built-in default fallback families ("Segoe UI" & "Consolas").
        let ensure_fallback = |map: &mut HashMap<String, (String, Vec<Arc<FontMeta>>)>,
                               name: &str,
                               mono: bool| {
            let key = name.to_ascii_lowercase();
            if map.contains_key(&key) {
                return;
            }
            let variants = [
                ("Regular", DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_STYLE_NORMAL),
                ("Bold", DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_STYLE_NORMAL),
                ("Italic", DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_STYLE_ITALIC),
                ("Bold Italic", DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_STYLE_ITALIC),
            ];
            let mut list = Vec::with_capacity(variants.len());
            for (face_name, weight, style) in variants {
                let mut info = HashMap::new();
                info.insert(DWRITE_INFO_FULL_NAME, format!("{name} {face_name}"));
                info.insert(DWRITE_INFO_VERSION_STRINGS, "Version 1.0 (Built-in Fallback)".to_string());
                info.insert(DWRITE_INFO_MANUFACTURER, "DirectWrite Built-in Fallback".to_string());
                list.push(Arc::new(FontMeta {
                    family_name: name.to_string(),
                    face_name: face_name.to_string(),
                    weight,
                    stretch: DWRITE_FONT_STRETCH_NORMAL,
                    style,
                    is_symbol: false,
                    is_monospace: mono,
                    face_type: DWRITE_FONT_FACE_TYPE_TRUETYPE,
                    face_index: 0,
                    glyph_count: 4096,
                    metrics: default_fallback_metrics(),
                    info_strings: info,
                    file_path: None,
                }));
            }
            map.insert(key, (name.to_string(), list));
        };

        ensure_fallback(&mut by_family, "Segoe UI", false);
        ensure_fallback(&mut by_family, "Consolas", true);

        let mut families: Vec<Arc<FontFamilyData>> = by_family
            .into_values()
            .map(|(name, fonts)| Arc::new(FontFamilyData { name, fonts }))
            .collect();
        families.sort_by(|a, b| a.name.to_ascii_lowercase().cmp(&b.name.to_ascii_lowercase()));

        let mut index_by_lower = HashMap::with_capacity(families.len());
        for (i, fam) in families.iter().enumerate() {
            index_by_lower.insert(fam.name.to_ascii_lowercase(), i);
        }

        Self { families, index_by_lower }
    }

    fn find_family_index(&self, name: &str) -> Option<usize> {
        let lower = name.trim().to_ascii_lowercase();
        if let Some(&idx) = self.index_by_lower.get(&lower) {
            return Some(idx);
        }
        // Standard Windows font family aliases -> real enumerated macOS fonts or built-in fallback.
        let candidates: &[&str] = match lower.as_str() {
            "arial" | "helvetica" | "sans-serif" | "calibri" | "tahoma" | "verdana"
            | "ms shell dlg" | "ms shell dlg 2" | "system" | "default" | "" => &[
                "arial",
                "arial unicode ms",
                "helvetica",
                "helvetica neue",
                "geneva",
                "segoe ui",
            ],
            "times new roman" | "times" | "serif" | "georgia" | "cambria" => &[
                "times new roman",
                "times",
                "new york",
                "georgia",
                "segoe ui",
            ],
            "courier new" | "courier" | "monospace" | "lucida console" => &[
                "courier new",
                "courier",
                "menlo",
                "monaco",
                "consolas",
            ],
            _ => return None,
        };
        for &c in candidates {
            if let Some(&idx) = self.index_by_lower.get(c) {
                return Some(idx);
            }
        }
        None
    }

    fn resolve_face(
        &self,
        family_name: &str,
        weight: u32,
        stretch: u32,
        style: u32,
    ) -> Arc<FontFaceInner> {
        let idx = self
            .find_family_index(family_name)
            .or_else(|| self.find_family_index("Segoe UI"))
            .unwrap_or(0);
        let fam = &self.families[idx];
        let meta = pick_best_font(&fam.fonts, weight, stretch, style);
        FontFaceInner::from_meta(meta, DWRITE_FONT_SIMULATIONS_NONE)
    }
}

fn font_match_score(meta: &FontMeta, weight: u32, stretch: u32, style: u32) -> (u32, u32, u32) {
    let stretch_d = meta.stretch.abs_diff(stretch);
    let style_d = if meta.style == style {
        0
    } else if style != DWRITE_FONT_STYLE_NORMAL && meta.style != DWRITE_FONT_STYLE_NORMAL {
        1
    } else {
        2
    };
    let weight_d = meta.weight.abs_diff(weight);
    (stretch_d, style_d, weight_d)
}

fn pick_best_font(fonts: &[Arc<FontMeta>], weight: u32, stretch: u32, style: u32) -> Arc<FontMeta> {
    fonts
        .iter()
        .min_by_key(|m| font_match_score(m, weight, stretch, style))
        .cloned()
        .unwrap_or_else(|| fonts[0].clone())
}

fn system_font_collection_data(refresh: bool) -> Arc<FontCollectionData> {
    static SYS: OnceLock<Mutex<Option<Arc<FontCollectionData>>>> = OnceLock::new();
    let m = SYS.get_or_init(|| Mutex::new(None));
    let mut guard = m.lock().unwrap();
    if refresh || guard.is_none() {
        *guard = Some(Arc::new(FontCollectionData::build_system()));
    }
    Arc::clone(guard.as_ref().unwrap())
}

// ============================================================================
// Internal COM Object Payloads
// ============================================================================

struct FactoryInner {
    _factory_type: u32,
}

struct LocalizedStringsInner {
    entries: Vec<(String, String)>, // (locale, value)
}

struct FontCollectionInner {
    data: Arc<FontCollectionData>,
}

struct FontFamilyInner {
    collection: Arc<FontCollectionData>,
    family: Arc<FontFamilyData>,
}

struct FontListInner {
    collection: Arc<FontCollectionData>,
    fonts: Vec<Arc<FontMeta>>,
}

struct FontInner {
    collection: Arc<FontCollectionData>,
    family: Arc<FontFamilyData>,
    meta: Arc<FontMeta>,
}

pub(crate) struct FontFaceInner {
    meta: Arc<FontMeta>,
    simulations: u32,
    sfnt: Option<Arc<SfntFace>>,
}

impl FontFaceInner {
    fn from_meta(meta: Arc<FontMeta>, simulations: u32) -> Arc<Self> {
        let sfnt = meta.file_path.as_ref().and_then(|p| {
            let bytes = load_cached_file_bytes(p)?;
            SfntFace::from_bytes(bytes, meta.face_index).map(Arc::new)
        });
        Arc::new(Self { meta, simulations, sfnt })
    }

    fn codepoint_to_glyph(&self, cp: u32) -> u16 {
        if let Some(ref sfnt) = self.sfnt {
            sfnt.codepoint_to_glyph(cp)
        } else {
            fallback_cp_to_gid(cp)
        }
    }

    fn has_character(&self, cp: u32) -> bool {
        if let Some(ref sfnt) = self.sfnt {
            sfnt.codepoint_to_glyph(cp) != 0
        } else {
            (0x20..=0xFFFE).contains(&cp)
        }
    }

    fn glyph_metrics(&self, gid: u16) -> DWRITE_GLYPH_METRICS {
        let m = &self.meta.metrics;
        let adv_h = (m.ascent as u32) + (m.descent as u32);
        let v_origin = m.ascent as i32;

        if let Some(ref sfnt) = self.sfnt {
            if let Some((adv_w, lsb)) = sfnt.hmtx(gid) {
                let (rsb, tsb, bsb) = if let Some((xmin, ymin, xmax, ymax)) = sfnt.glyph_bbox(gid) {
                    let bw = (xmax as i32) - (xmin as i32);
                    let r = (adv_w as i32) - (lsb as i32) - bw;
                    let t = (m.ascent as i32) - (ymax as i32);
                    let b = (ymin as i32) + (m.descent as i32);
                    (r, t, b)
                } else {
                    (0, (m.ascent as i32) / 5, (m.descent as i32) / 2)
                };
                return DWRITE_GLYPH_METRICS {
                    leftSideBearing: lsb as i32,
                    advanceWidth: adv_w as u32,
                    rightSideBearing: rsb,
                    topSideBearing: tsb,
                    advanceHeight: adv_h,
                    bottomSideBearing: bsb,
                    verticalOriginY: v_origin,
                };
            }
        }

        let ch = fallback_gid_to_char(gid).unwrap_or('?');
        let adv_w = fallback_char_advance_du(ch, self.meta.is_monospace, self.meta.weight);
        let lsb = (adv_w as i32) / 14;
        let rsb = (adv_w as i32) / 14;
        DWRITE_GLYPH_METRICS {
            leftSideBearing: lsb,
            advanceWidth: adv_w,
            rightSideBearing: rsb,
            topSideBearing: (m.ascent as i32) - (m.capHeight as i32),
            advanceHeight: adv_h,
            bottomSideBearing: m.descent as i32,
            verticalOriginY: v_origin,
        }
    }

    fn scaled_advance(&self, gid: u16, em_size: f32) -> f32 {
        let gm = self.glyph_metrics(gid);
        let upem = self.meta.metrics.designUnitsPerEm.max(1) as f32;
        (gm.advanceWidth as f32) * em_size / upem
    }
}

struct FontFileInner {
    path: PathBuf,
    path_wide: Vec<u16>,
}

struct RenderingParamsInner {
    gamma: f32,
    enhanced_contrast: f32,
    clear_type_level: f32,
    pixel_geometry: u32,
    rendering_mode: u32,
}

struct TypographyInner {
    features: Mutex<Vec<DWRITE_FONT_FEATURE>>,
}

struct GdiInteropInner {
    collection: Arc<FontCollectionData>,
}

struct BitmapRenderTargetInner {
    state: Mutex<BitmapRenderTargetState>,
}

struct BitmapRenderTargetState {
    hdc: usize,
    hbm: usize,
    old_hbm: usize,
    width: u32,
    height: u32,
    pixels_per_dip: f32,
    transform: DWRITE_MATRIX,
}

impl Drop for BitmapRenderTargetInner {
    fn drop(&mut self) {
        let st = self.state.lock().unwrap();
        unsafe {
            if st.hdc != 0 {
                if st.old_hbm != 0 {
                    crate::misc::gdi32::SelectObject(st.hdc, st.old_hbm);
                }
                if st.hbm != 0 {
                    crate::misc::gdi32::DeleteObject(st.hbm);
                }
                crate::misc::gdi32::DeleteDC(st.hdc);
            }
        }
    }
}

struct EllipsisInlineObjectInner {
    font_size: f32,
    ascent: f32,
    height: f32,
}

struct NumberSubstitutionInner;

#[derive(Clone)]
struct TextFormatState {
    family_name: String,
    collection: Arc<FontCollectionData>,
    weight: u32,
    style: u32,
    stretch: u32,
    font_size: f32,
    locale_name: String,
    text_alignment: u32,
    paragraph_alignment: u32,
    word_wrapping: u32,
    reading_direction: u32,
    flow_direction: u32,
    incremental_tab_stop: f32,
    trimming: DWRITE_TRIMMING,
    trimming_sign: Option<ComPtr>,
    line_spacing_method: u32,
    line_spacing: f32,
    baseline: f32,
}

struct TextFormatInner {
    state: Mutex<TextFormatState>,
}

#[derive(Clone)]
struct CharRunProps {
    family_name: String,
    collection: Arc<FontCollectionData>,
    weight: u32,
    style: u32,
    stretch: u32,
    font_size: f32,
    locale_name: String,
    underline: bool,
    strikethrough: bool,
    drawing_effect: Option<ComPtr>,
    inline_object: Option<ComPtr>,
    typography: Option<ComPtr>,
}

#[derive(Clone)]
struct LayoutCluster {
    text_pos: u32,
    utf16_len: u16,
    gid: u16,
    width: f32,
    is_whitespace: bool,
    is_newline: bool,
    can_wrap_after: bool,
    face: Arc<FontFaceInner>,
    font_size: f32,
    underline: bool,
    strikethrough: bool,
    drawing_effect: Option<ComPtr>,
    locale_name: String,
}

#[derive(Clone)]
struct LayoutLine {
    cluster_start: usize,
    cluster_end: usize,
    text_start: u32,
    utf16_len: u32,
    trailing_ws_len: u32,
    newline_len: u32,
    left: f32,
    top: f32,
    width: f32,
    width_with_trailing_ws: f32,
    height: f32,
    baseline: f32,
    is_trimmed: bool,
    cluster_lefts: Vec<f32>,
    cluster_widths: Vec<f32>,
}

struct ComputedLayout {
    clusters: Vec<LayoutCluster>,
    lines: Vec<LayoutLine>,
    metrics: DWRITE_TEXT_METRICS,
    overhang: DWRITE_OVERHANG_METRICS,
    min_width: f32,
}

struct TextLayoutState {
    fmt: TextFormatState,
    text: Vec<u16>,
    max_width: f32,
    max_height: f32,
    char_props: Vec<CharRunProps>,
    dirty: bool,
    cached: Option<ComputedLayout>,
}

struct TextLayoutInner {
    state: Mutex<TextLayoutState>,
}

impl TextLayoutState {
    fn new(text: Vec<u16>, fmt: TextFormatState, max_width: f32, max_height: f32) -> Self {
        let base = CharRunProps {
            family_name: fmt.family_name.clone(),
            collection: Arc::clone(&fmt.collection),
            weight: fmt.weight,
            style: fmt.style,
            stretch: fmt.stretch,
            font_size: fmt.font_size,
            locale_name: fmt.locale_name.clone(),
            underline: false,
            strikethrough: false,
            drawing_effect: None,
            inline_object: None,
            typography: None,
        };
        let char_props = vec![base; text.len()];
        Self {
            fmt,
            text,
            max_width: if max_width.is_finite() && max_width >= 0.0 { max_width } else { f32::MAX },
            max_height: if max_height.is_finite() && max_height >= 0.0 { max_height } else { f32::MAX },
            char_props,
            dirty: true,
            cached: None,
        }
    }

    fn props_at(&self, pos: u32) -> (CharRunProps, DWRITE_TEXT_RANGE) {
        let len = self.char_props.len();
        if len == 0 {
            let base = CharRunProps {
                family_name: self.fmt.family_name.clone(),
                collection: Arc::clone(&self.fmt.collection),
                weight: self.fmt.weight,
                style: self.fmt.style,
                stretch: self.fmt.stretch,
                font_size: self.fmt.font_size,
                locale_name: self.fmt.locale_name.clone(),
                underline: false,
                strikethrough: false,
                drawing_effect: None,
                inline_object: None,
                typography: None,
            };
            return (base, DWRITE_TEXT_RANGE { startPosition: pos, length: 0 });
        }
        let idx = (pos as usize).min(len - 1);
        let p = self.char_props[idx].clone();
        let mut end = idx + 1;
        while end < len
            && self.char_props[end].family_name == p.family_name
            && self.char_props[end].weight == p.weight
            && self.char_props[end].style == p.style
            && self.char_props[end].stretch == p.stretch
            && (self.char_props[end].font_size - p.font_size).abs() < 1e-4
            && self.char_props[end].underline == p.underline
            && self.char_props[end].strikethrough == p.strikethrough
            && self.char_props[end].locale_name == p.locale_name
        {
            end += 1;
        }
        (
            p,
            DWRITE_TEXT_RANGE {
                startPosition: idx as u32,
                length: (end - idx) as u32,
            },
        )
    }

    fn apply_range(&mut self, range: DWRITE_TEXT_RANGE, mut f: impl FnMut(&mut CharRunProps)) {
        let start = (range.startPosition as usize).min(self.char_props.len());
        let end = start.saturating_add(range.length as usize).min(self.char_props.len());
        for p in &mut self.char_props[start..end] {
            f(p);
        }
        self.dirty = true;
    }

    fn ensure_layout(&mut self) -> &ComputedLayout {
        if self.dirty || self.cached.is_none() {
            let computed = compute_text_layout(self);
            self.cached = Some(computed);
            self.dirty = false;
        }
        self.cached.as_ref().unwrap()
    }
}

fn line_metrics_for_face(face: &FontFaceInner, em_size: f32, fmt: &TextFormatState) -> (f32, f32) {
    let m = &face.meta.metrics;
    let upem = m.designUnitsPerEm.max(1) as f32;
    let nat_ascent = (m.ascent as f32) * em_size / upem;
    let nat_descent = (m.descent as f32) * em_size / upem;
    let nat_gap = (m.lineGap.max(0) as f32) * em_size / upem;
    let nat_height = (nat_ascent + nat_descent + nat_gap).max(em_size);
    match fmt.line_spacing_method {
        DWRITE_LINE_SPACING_METHOD_UNIFORM if fmt.line_spacing > 0.0 => {
            (fmt.line_spacing, fmt.baseline.max(0.0))
        }
        DWRITE_LINE_SPACING_METHOD_PROPORTIONAL if fmt.line_spacing > 0.0 => {
            (nat_height * fmt.line_spacing, nat_ascent * fmt.baseline.max(0.5))
        }
        _ => (nat_height, nat_ascent),
    }
}

fn compute_text_layout(st: &TextLayoutState) -> ComputedLayout {
    // 1. Decode UTF-16 into clusters.
    let mut clusters: Vec<LayoutCluster> = Vec::new();
    let mut face_cache: HashMap<(String, u32, u32, u32), Arc<FontFaceInner>> = HashMap::new();
    let mut resolve_face = |col: &Arc<FontCollectionData>, fam: &str, w: u32, strch: u32, sty: u32| -> Arc<FontFaceInner> {
        let key = (fam.to_ascii_lowercase(), w, strch, sty);
        if let Some(f) = face_cache.get(&key) {
            return Arc::clone(f);
        }
        let f = col.resolve_face(fam, w, strch, sty);
        face_cache.insert(key, Arc::clone(&f));
        f
    };

    let mut i = 0usize;
    let n = st.text.len();
    while i < n {
        let u0 = st.text[i];
        let (cp, ulen, is_nl) = if u0 == 0x000D {
            if i + 1 < n && st.text[i + 1] == 0x000A {
                (0x000Au32, 2u16, true)
            } else {
                (0x000Du32, 1u16, true)
            }
        } else if u0 == 0x000A || u0 == 0x0085 || u0 == 0x2028 || u0 == 0x2029 {
            (u0 as u32, 1u16, true)
        } else if (0xD800..=0xDBFF).contains(&u0) && i + 1 < n && (0xDC00..=0xDFFF).contains(&st.text[i + 1]) {
            let hi = (u0 as u32) - 0xD800;
            let lo = (st.text[i + 1] as u32) - 0xDC00;
            (0x1_0000 + (hi << 10) + lo, 2u16, false)
        } else {
            (u0 as u32, 1u16, false)
        };

        let props = &st.char_props[i];
        let face = resolve_face(&props.collection, &props.family_name, props.weight, props.stretch, props.style);
        let ch = char::from_u32(cp).unwrap_or('\u{FFFD}');
        let is_ws = !is_nl && (ch == ' ' || ch == '\t' || ch.is_whitespace());
        let gid = if is_nl { 0 } else { face.codepoint_to_glyph(cp) };
        let width = if is_nl {
            0.0
        } else if ch == '\t' {
            if st.fmt.incremental_tab_stop > 0.0 {
                st.fmt.incremental_tab_stop
            } else {
                face.scaled_advance(face.codepoint_to_glyph(b' ' as u32), props.font_size) * 4.0
            }
        } else {
            face.scaled_advance(gid, props.font_size)
        };
        let can_wrap_after = is_ws || is_nl || ch == '-';

        clusters.push(LayoutCluster {
            text_pos: i as u32,
            utf16_len: ulen,
            gid,
            width,
            is_whitespace: is_ws,
            is_newline: is_nl,
            can_wrap_after,
            face,
            font_size: props.font_size,
            underline: props.underline,
            strikethrough: props.strikethrough,
            drawing_effect: props.drawing_effect.clone(),
            locale_name: props.locale_name.clone(),
        });
        i += ulen as usize;
    }

    // 2. Compute minimum unbreakable word width.
    let mut min_width = 0.0f32;
    let mut cur_word_w = 0.0f32;
    for c in &clusters {
        if c.is_whitespace || c.is_newline {
            if cur_word_w > min_width {
                min_width = cur_word_w;
            }
            cur_word_w = 0.0;
        } else {
            cur_word_w += c.width;
        }
    }
    if cur_word_w > min_width {
        min_width = cur_word_w;
    }

    // 3. Line breaking / word wrapping.
    let default_face = resolve_face(
        &st.fmt.collection,
        &st.fmt.family_name,
        st.fmt.weight,
        st.fmt.stretch,
        st.fmt.style,
    );
    let (default_line_h, default_baseline) = line_metrics_for_face(&default_face, st.fmt.font_size, &st.fmt);
    let max_w = st.max_width;
    let wrap_mode = st.fmt.word_wrapping;

    let mut raw_ranges: Vec<(usize, usize)> = Vec::new();
    if clusters.is_empty() {
        raw_ranges.push((0, 0));
    } else {
        let mut line_start = 0usize;
        while line_start < clusters.len() {
            let mut line_end = line_start;
            let mut cur_w = 0.0f32;
            let mut last_break: Option<usize> = None;
            let mut hit_newline = false;

            while line_end < clusters.len() {
                let c = &clusters[line_end];
                if c.is_newline {
                    line_end += 1;
                    hit_newline = true;
                    break;
                }
                let next_w = cur_w + c.width;
                let exceeds = wrap_mode != DWRITE_WORD_WRAPPING_NO_WRAP
                    && !c.is_whitespace
                    && line_end > line_start
                    && next_w > max_w + 1e-4;

                if exceeds {
                    if wrap_mode == DWRITE_WORD_WRAPPING_CHARACTER {
                        break;
                    }
                    if let Some(brk) = last_break {
                        if brk > line_start {
                            line_end = brk;
                            break;
                        }
                    }
                    if wrap_mode == DWRITE_WORD_WRAPPING_WHOLE_WORD {
                        // Continue until next wrap opportunity
                        cur_w = next_w;
                        line_end += 1;
                        if c.can_wrap_after {
                            break;
                        }
                        continue;
                    }
                    // WRAP / EMERGENCY_BREAK: break mid-word.
                    break;
                }

                cur_w = next_w;
                line_end += 1;
                if c.can_wrap_after {
                    last_break = Some(line_end);
                }
            }

            if line_end == line_start {
                line_end = (line_start + 1).min(clusters.len());
            }
            raw_ranges.push((line_start, line_end));
            line_start = line_end;

            // Trailing explicit newline creates an empty trailing line.
            if hit_newline && line_start == clusters.len() {
                raw_ranges.push((line_start, line_start));
            }
        }
    }

    // 4. Compute line geometries, vertical alignment, and horizontal alignment.
    let mut lines: Vec<LayoutLine> = Vec::with_capacity(raw_ranges.len());
    let mut y_cursor = 0.0f32;
    let total_lines = raw_ranges.len();

    for (line_idx, &(cs, ce)) in raw_ranges.iter().enumerate() {
        let mut h = default_line_h;
        let mut bl = default_baseline;
        let mut utf16_len = 0u32;
        let mut nl_len = 0u32;
        let mut w_all = 0.0f32;

        if cs < ce {
            h = 0.0;
            bl = 0.0;
            for c in &clusters[cs..ce] {
                let (ch_h, ch_bl) = line_metrics_for_face(&c.face, c.font_size, &st.fmt);
                if ch_h > h { h = ch_h; }
                if ch_bl > bl { bl = ch_bl; }
                utf16_len += c.utf16_len as u32;
                if c.is_newline {
                    nl_len += c.utf16_len as u32;
                }
                w_all += c.width;
            }
        }

        // Count trailing whitespace (before any newline).
        let mut vis_end = ce;
        while vis_end > cs && clusters[vis_end - 1].is_newline {
            vis_end -= 1;
        }
        let mut ws_end = vis_end;
        let mut tws_len = 0u32;
        while ws_end > cs && clusters[ws_end - 1].is_whitespace {
            tws_len += clusters[ws_end - 1].utf16_len as u32;
            ws_end -= 1;
        }
        let w_vis: f32 = clusters[cs..ws_end].iter().map(|c| c.width).sum();

        let is_trimmed = st.fmt.trimming.granularity != DWRITE_TRIMMING_GRANULARITY_NONE
            && w_vis > max_w + 1e-4;

        let mut cluster_widths: Vec<f32> = clusters[cs..ce].iter().map(|c| c.width).collect();
        let mut line_width_vis = w_vis;

        // Justify intermediate lines when DWRITE_TEXT_ALIGNMENT_JUSTIFIED is active.
        if st.fmt.text_alignment == DWRITE_TEXT_ALIGNMENT_JUSTIFIED
            && line_idx + 1 < total_lines
            && max_w.is_finite()
            && max_w < 1e15
            && nl_len == 0
            && w_vis > 0.0
            && w_vis < max_w
        {
            let ws_count = clusters[cs..ws_end].iter().filter(|c| c.is_whitespace).count();
            if ws_count > 0 {
                let extra = (max_w - w_vis) / (ws_count as f32);
                for (k, c) in clusters[cs..ws_end].iter().enumerate() {
                    if c.is_whitespace {
                        cluster_widths[k] += extra;
                    }
                }
                line_width_vis = max_w;
                w_all += extra * (ws_count as f32);
            }
        }

        let align_w = if max_w.is_finite() && max_w < 1e15 { max_w } else { line_width_vis };
        let line_left = match st.fmt.text_alignment {
            DWRITE_TEXT_ALIGNMENT_TRAILING => align_w - line_width_vis,
            DWRITE_TEXT_ALIGNMENT_CENTER => (align_w - line_width_vis) * 0.5,
            _ => 0.0,
        };

        let mut cluster_lefts = Vec::with_capacity(ce - cs);
        let mut x_cur = line_left;
        for &cw in &cluster_widths {
            cluster_lefts.push(x_cur);
            x_cur += cw;
        }

        let text_start = if cs < clusters.len() {
            clusters[cs].text_pos
        } else {
            st.text.len() as u32
        };

        lines.push(LayoutLine {
            cluster_start: cs,
            cluster_end: ce,
            text_start,
            utf16_len,
            trailing_ws_len: tws_len,
            newline_len: nl_len,
            left: line_left,
            top: y_cursor,
            width: line_width_vis,
            width_with_trailing_ws: w_all,
            height: h,
            baseline: bl,
            is_trimmed,
            cluster_lefts,
            cluster_widths,
        });
        y_cursor += h;
    }

    let total_height = y_cursor;
    let v_offset = if st.max_height.is_finite() && st.max_height < 1e15 {
        match st.fmt.paragraph_alignment {
            DWRITE_PARAGRAPH_ALIGNMENT_FAR => st.max_height - total_height,
            DWRITE_PARAGRAPH_ALIGNMENT_CENTER => (st.max_height - total_height) * 0.5,
            _ => 0.0,
        }
    } else {
        0.0
    };
    if v_offset != 0.0 {
        for line in &mut lines {
            line.top += v_offset;
        }
    }

    // 5. Aggregate overall bounding & overhang metrics.
    let mut min_left = f32::INFINITY;
    let mut max_vis_w = 0.0f32;
    let mut max_all_w = 0.0f32;
    let mut ink_left = f32::INFINITY;
    let mut ink_top = f32::INFINITY;
    let mut ink_right = f32::NEG_INFINITY;
    let mut ink_bottom = f32::NEG_INFINITY;

    for line in &lines {
        if line.left < min_left {
            min_left = line.left;
        }
        if line.width > max_vis_w {
            max_vis_w = line.width;
        }
        if line.width_with_trailing_ws > max_all_w {
            max_all_w = line.width_with_trailing_ws;
        }
        if line.width > 0.0 {
            ink_left = ink_left.min(line.left);
            ink_right = ink_right.max(line.left + line.width);
            ink_top = ink_top.min(line.top);
            ink_bottom = ink_bottom.max(line.top + line.height);
        }
    }
    if !min_left.is_finite() {
        min_left = 0.0;
    }
    let metrics = DWRITE_TEXT_METRICS {
        left: min_left,
        top: v_offset,
        width: max_vis_w,
        widthIncludingTrailingWhitespace: max_all_w,
        height: total_height,
        layoutWidth: st.max_width,
        layoutHeight: st.max_height,
        maxBidiReorderingDepth: 1,
        lineCount: lines.len() as u32,
    };

    let overhang = if ink_left.is_finite() {
        DWRITE_OVERHANG_METRICS {
            left: -ink_left,
            top: -ink_top,
            right: ink_right - st.max_width,
            bottom: ink_bottom - st.max_height,
        }
    } else {
        DWRITE_OVERHANG_METRICS {
            left: 0.0,
            top: 0.0,
            right: -st.max_width,
            bottom: -st.max_height,
        }
    };

    ComputedLayout { clusters, lines, metrics, overhang, min_width }
}

// ============================================================================
// IDWriteLocalizedStrings Vtable (9 slots)
// ============================================================================

type PLocStr = *mut ComObj<LocalizedStringsInner>;

unsafe extern "win64" fn locstr_qi(this: PLocStr, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_LOCALIZED_STRINGS])
}
unsafe extern "win64" fn locstr_add_ref(this: PLocStr) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn locstr_release(this: PLocStr) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn locstr_get_count(this: PLocStr) -> u32 {
    if this.is_null() {
        return 0;
    }
    ComObj::inner(this).entries.len() as u32
}
unsafe extern "win64" fn locstr_find_locale(
    this: PLocStr,
    locale_name: *const u16,
    index: *mut u32,
    exists: *mut BOOL,
) -> HRESULT {
    if this.is_null() || index.is_null() || exists.is_null() {
        return E_POINTER;
    }
    let query = read_wide_z(locale_name).to_ascii_lowercase();
    let entries = &ComObj::inner(this).entries;
    if let Some(pos) = entries.iter().position(|(l, _)| l.to_ascii_lowercase() == query) {
        index.write_unaligned(pos as u32);
        exists.write_unaligned(TRUE);
    } else {
        index.write_unaligned(u32::MAX);
        exists.write_unaligned(FALSE);
    }
    S_OK
}
unsafe extern "win64" fn locstr_get_locale_len(this: PLocStr, index: u32, length: *mut u32) -> HRESULT {
    if this.is_null() || length.is_null() {
        return E_POINTER;
    }
    let Some((loc, _)) = ComObj::inner(this).entries.get(index as usize) else {
        return E_INVALIDARG;
    };
    length.write_unaligned(loc.encode_utf16().count() as u32);
    S_OK
}
unsafe extern "win64" fn locstr_get_locale(this: PLocStr, index: u32, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() || buf.is_null() {
        return E_POINTER;
    }
    let Some((loc, _)) = ComObj::inner(this).entries.get(index as usize) else {
        return E_INVALIDARG;
    };
    write_wide_z(loc, buf, size)
}
unsafe extern "win64" fn locstr_get_str_len(this: PLocStr, index: u32, length: *mut u32) -> HRESULT {
    if this.is_null() || length.is_null() {
        return E_POINTER;
    }
    let Some((_, val)) = ComObj::inner(this).entries.get(index as usize) else {
        return E_INVALIDARG;
    };
    length.write_unaligned(val.encode_utf16().count() as u32);
    S_OK
}
unsafe extern "win64" fn locstr_get_str(this: PLocStr, index: u32, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() || buf.is_null() {
        return E_POINTER;
    }
    let Some((_, val)) = ComObj::inner(this).entries.get(index as usize) else {
        return E_INVALIDARG;
    };
    write_wide_z(val, buf, size)
}

fn localized_strings_vtbl() -> *const usize {
    static V: OnceLock<[usize; 9]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            locstr_qi as *const () as usize,
            locstr_add_ref as *const () as usize,
            locstr_release as *const () as usize,
            locstr_get_count as *const () as usize,
            locstr_find_locale as *const () as usize,
            locstr_get_locale_len as *const () as usize,
            locstr_get_locale as *const () as usize,
            locstr_get_str_len as *const () as usize,
            locstr_get_str as *const () as usize,
        ])
    })
    .as_ptr()
}

fn new_localized_strings(value: String) -> *mut c_void {
    ComObj::new(
        localized_strings_vtbl(),
        LocalizedStringsInner { entries: vec![("en-us".to_string(), value)] },
    ) as *mut c_void
}

// ============================================================================
// IDWriteFontFile Vtable (6 slots)
// ============================================================================

type PFontFile = *mut ComObj<FontFileInner>;

unsafe extern "win64" fn ff_file_qi(this: PFontFile, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_FONT_FILE])
}
unsafe extern "win64" fn ff_file_add_ref(this: PFontFile) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn ff_file_release(this: PFontFile) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn ff_file_get_ref_key(
    this: PFontFile,
    key_ptr: *mut *const c_void,
    key_size: *mut u32,
) -> HRESULT {
    if this.is_null() || key_ptr.is_null() || key_size.is_null() {
        return E_POINTER;
    }
    let w = &ComObj::inner(this).path_wide;
    key_ptr.write_unaligned(w.as_ptr() as *const c_void);
    key_size.write_unaligned((w.len() * 2) as u32);
    S_OK
}
unsafe extern "win64" fn ff_file_get_loader(_this: PFontFile, loader: *mut *mut c_void) -> HRESULT {
    if !loader.is_null() {
        loader.write_unaligned(std::ptr::null_mut());
    }
    E_NOTIMPL
}
unsafe extern "win64" fn ff_file_analyze(
    this: PFontFile,
    is_supported: *mut BOOL,
    file_type: *mut u32,
    face_type: *mut u32,
    num_faces: *mut u32,
) -> HRESULT {
    if this.is_null() || is_supported.is_null() || file_type.is_null() || num_faces.is_null() {
        return E_POINTER;
    }
    let metas = parse_font_file_metadata(&ComObj::inner(this).path);
    if metas.is_empty() {
        is_supported.write_unaligned(FALSE);
        file_type.write_unaligned(DWRITE_FONT_FILE_TYPE_UNKNOWN);
        if !face_type.is_null() {
            face_type.write_unaligned(DWRITE_FONT_FACE_TYPE_UNKNOWN);
        }
        num_faces.write_unaligned(0);
    } else {
        let ft = metas[0].face_type;
        let file_t = match ft {
            DWRITE_FONT_FACE_TYPE_CFF => DWRITE_FONT_FILE_TYPE_CFF,
            DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION => DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION,
            _ => DWRITE_FONT_FILE_TYPE_TRUETYPE,
        };
        is_supported.write_unaligned(TRUE);
        file_type.write_unaligned(file_t);
        if !face_type.is_null() {
            face_type.write_unaligned(ft);
        }
        num_faces.write_unaligned(metas.len() as u32);
    }
    S_OK
}

fn font_file_vtbl() -> *const usize {
    static V: OnceLock<[usize; 6]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            ff_file_qi as *const () as usize,
            ff_file_add_ref as *const () as usize,
            ff_file_release as *const () as usize,
            ff_file_get_ref_key as *const () as usize,
            ff_file_get_loader as *const () as usize,
            ff_file_analyze as *const () as usize,
        ])
    })
    .as_ptr()
}

fn new_font_file(path: PathBuf) -> *mut c_void {
    let path_wide: Vec<u16> = path.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
    ComObj::new(font_file_vtbl(), FontFileInner { path, path_wide }) as *mut c_void
}

// ============================================================================
// IDWriteFontFace Vtable (18 slots)
// ============================================================================

type PFontFace = *mut ComObj<Arc<FontFaceInner>>;

unsafe extern "win64" fn face_qi(this: PFontFace, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_FONT_FACE])
}
unsafe extern "win64" fn face_add_ref(this: PFontFace) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn face_release(this: PFontFace) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn face_get_type(this: PFontFace) -> u32 {
    if this.is_null() {
        return DWRITE_FONT_FACE_TYPE_UNKNOWN;
    }
    ComObj::inner(this).meta.face_type
}
unsafe extern "win64" fn face_get_files(
    this: PFontFace,
    num_files: *mut u32,
    files: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || num_files.is_null() {
        return E_POINTER;
    }
    let has_file = ComObj::inner(this).meta.file_path.is_some();
    let count = if has_file { 1u32 } else { 0u32 };
    if files.is_null() {
        num_files.write_unaligned(count);
        return S_OK;
    }
    let cap = num_files.read_unaligned();
    if cap < count {
        num_files.write_unaligned(count);
        return E_NOT_SUFFICIENT_BUFFER;
    }
    if let Some(ref p) = ComObj::inner(this).meta.file_path {
        files.write_unaligned(new_font_file(p.clone()));
    }
    num_files.write_unaligned(count);
    S_OK
}
unsafe extern "win64" fn face_get_index(this: PFontFace) -> u32 {
    if this.is_null() {
        return 0;
    }
    ComObj::inner(this).meta.face_index
}
unsafe extern "win64" fn face_get_simulations(this: PFontFace) -> u32 {
    if this.is_null() {
        return 0;
    }
    ComObj::inner(this).simulations
}
unsafe extern "win64" fn face_is_symbol(this: PFontFace) -> BOOL {
    if this.is_null() {
        return FALSE;
    }
    if ComObj::inner(this).meta.is_symbol { TRUE } else { FALSE }
}
unsafe extern "win64" fn face_get_metrics(this: PFontFace, out: *mut DWRITE_FONT_METRICS) {
    if this.is_null() || out.is_null() {
        return;
    }
    out.write_unaligned(ComObj::inner(this).meta.metrics);
}
unsafe extern "win64" fn face_get_glyph_count(this: PFontFace) -> u16 {
    if this.is_null() {
        return 0;
    }
    ComObj::inner(this).meta.glyph_count
}
unsafe extern "win64" fn face_get_design_glyph_metrics(
    this: PFontFace,
    glyph_indices: *const u16,
    glyph_count: u32,
    out_metrics: *mut DWRITE_GLYPH_METRICS,
    _is_sideways: BOOL,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    if glyph_count > 0 && (glyph_indices.is_null() || out_metrics.is_null()) {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    for i in 0..(glyph_count as usize) {
        let gid = glyph_indices.add(i).read_unaligned();
        out_metrics.add(i).write_unaligned(inner.glyph_metrics(gid));
    }
    S_OK
}
unsafe extern "win64" fn face_get_glyph_indices(
    this: PFontFace,
    code_points: *const u32,
    count: u32,
    out_indices: *mut u16,
) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    if count > 0 && (code_points.is_null() || out_indices.is_null()) {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    for i in 0..(count as usize) {
        let cp = code_points.add(i).read_unaligned();
        out_indices.add(i).write_unaligned(inner.codepoint_to_glyph(cp));
    }
    S_OK
}

struct FontTableLease {
    _keepalive: Arc<Vec<u8>>,
}

unsafe extern "win64" fn face_try_get_font_table(
    this: PFontFace,
    tag: u32,
    table_data: *mut *const c_void,
    table_size: *mut u32,
    table_context: *mut *mut c_void,
    exists: *mut BOOL,
) -> HRESULT {
    if this.is_null()
        || table_data.is_null()
        || table_size.is_null()
        || table_context.is_null()
        || exists.is_null()
    {
        return E_POINTER;
    }
    table_data.write_unaligned(std::ptr::null());
    table_size.write_unaligned(0);
    table_context.write_unaligned(std::ptr::null_mut());
    exists.write_unaligned(FALSE);

    if let Some(ref sfnt) = ComObj::inner(this).sfnt {
        if let Some((off, len)) = sfnt.try_get_table(tag) {
            let lease = Box::new(FontTableLease { _keepalive: Arc::clone(&sfnt.raw) });
            let ptr = sfnt.raw.as_ptr().add(off) as *const c_void;
            table_data.write_unaligned(ptr);
            table_size.write_unaligned(len as u32);
            table_context.write_unaligned(Box::into_raw(lease) as *mut c_void);
            exists.write_unaligned(TRUE);
        }
    }
    S_OK
}
unsafe extern "win64" fn face_release_font_table(_this: PFontFace, table_context: *mut c_void) {
    if !table_context.is_null() {
        drop(Box::from_raw(table_context as *mut FontTableLease));
    }
}
unsafe extern "win64" fn face_get_glyph_run_outline(
    this: PFontFace,
    em_size: f32,
    glyph_indices: *const u16,
    glyph_advances: *const f32,
    glyph_offsets: *const DWRITE_GLYPH_OFFSET,
    glyph_count: u32,
    _is_sideways: BOOL,
    _is_rtl: BOOL,
    geometry_sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || geometry_sink.is_null() {
        return E_POINTER;
    }
    if glyph_count > 0 && glyph_indices.is_null() {
        return E_POINTER;
    }
    let sink_vtbl = (geometry_sink as *const *const usize).read_unaligned();
    if sink_vtbl.is_null() {
        return E_INVALIDARG;
    }
    let fn_begin_fig = sink_vtbl.add(5).read_unaligned();
    let fn_add_lines = sink_vtbl.add(6).read_unaligned();
    let fn_end_fig = sink_vtbl.add(8).read_unaligned();

    let inner = ComObj::inner(this);
    let upem = inner.meta.metrics.designUnitsPerEm.max(1) as f32;
    let scale = em_size / upem;
    let mut pen_x = 0.0f32;

    for i in 0..(glyph_count as usize) {
        let gid = glyph_indices.add(i).read_unaligned();
        let adv = if !glyph_advances.is_null() {
            glyph_advances.add(i).read_unaligned()
        } else {
            inner.scaled_advance(gid, em_size)
        };
        let off = if !glyph_offsets.is_null() {
            glyph_offsets.add(i).read_unaligned()
        } else {
            DWRITE_GLYPH_OFFSET::default()
        };
        if let Some(ref sfnt) = inner.sfnt {
            if let Some(contours) = sfnt.glyph_contours(gid, 0) {
                for c in &contours {
                    let poly = flatten_quad_contour(c, scale, 0.0);
                    if poly.len() >= 2 {
                        let start_x = poly[0].0 + pen_x + off.advanceOffset;
                        let start_y = poly[0].1 - off.ascenderOffset;
                        let packed = (start_x.to_bits() as u64) | ((start_y.to_bits() as u64) << 32);
                        crate::call_guest8(fn_begin_fig, geometry_sink as u64, packed, 0, 0, 0, 0, 0, 0);
                        let pts: Vec<[f32; 2]> = poly[1..]
                            .iter()
                            .map(|&(px, py)| [px + pen_x + off.advanceOffset, py - off.ascenderOffset])
                            .collect();
                        crate::call_guest8(
                            fn_add_lines,
                            geometry_sink as u64,
                            pts.as_ptr() as u64,
                            pts.len() as u64,
                            0,
                            0,
                            0,
                            0,
                            0,
                        );
                        crate::call_guest8(fn_end_fig, geometry_sink as u64, 1, 0, 0, 0, 0, 0, 0);
                    }
                }
            }
        }
        pen_x += adv;
    }
    S_OK
}
crate::win64_bridge!(fn face_get_glyph_run_outline(
    PFontFace, f32, *const u16, *const f32, *const DWRITE_GLYPH_OFFSET, u32, BOOL, BOOL, *mut c_void
) -> HRESULT);

unsafe extern "win64" fn face_get_recommended_rendering_mode(
    this: PFontFace,
    em_size: f32,
    pixels_per_dip: f32,
    _measuring_mode: u32,
    _rendering_params: *mut c_void,
    out_mode: *mut u32,
) -> HRESULT {
    if this.is_null() || out_mode.is_null() {
        return E_POINTER;
    }
    let px = em_size * pixels_per_dip.max(1.0);
    let mode = if px < 6.0 {
        DWRITE_RENDERING_MODE_ALIASED
    } else {
        DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC
    };
    out_mode.write_unaligned(mode);
    S_OK
}
crate::win64_bridge!(fn face_get_recommended_rendering_mode(
    PFontFace, f32, f32, u32, *mut c_void, *mut u32
) -> HRESULT);

unsafe extern "win64" fn face_get_gdi_compatible_metrics(
    this: PFontFace,
    _em_size: f32,
    _pixels_per_dip: f32,
    _transform: *const DWRITE_MATRIX,
    out_metrics: *mut DWRITE_FONT_METRICS,
) -> HRESULT {
    if this.is_null() || out_metrics.is_null() {
        return E_POINTER;
    }
    out_metrics.write_unaligned(ComObj::inner(this).meta.metrics);
    S_OK
}
crate::win64_bridge!(fn face_get_gdi_compatible_metrics(
    PFontFace, f32, f32, *const DWRITE_MATRIX, *mut DWRITE_FONT_METRICS
) -> HRESULT);

unsafe extern "win64" fn face_get_gdi_compatible_glyph_metrics(
    this: PFontFace,
    _em_size: f32,
    _pixels_per_dip: f32,
    _transform: *const DWRITE_MATRIX,
    _use_gdi_natural: BOOL,
    glyph_indices: *const u16,
    glyph_count: u32,
    out_metrics: *mut DWRITE_GLYPH_METRICS,
    is_sideways: BOOL,
) -> HRESULT {
    face_get_design_glyph_metrics(this, glyph_indices, glyph_count, out_metrics, is_sideways)
}
crate::win64_bridge!(fn face_get_gdi_compatible_glyph_metrics(
    PFontFace, f32, f32, *const DWRITE_MATRIX, BOOL, *const u16, u32, *mut DWRITE_GLYPH_METRICS, BOOL
) -> HRESULT);

fn font_face_vtbl() -> *const usize {
    static V: OnceLock<[usize; 18]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            face_qi as *const () as usize,
            face_add_ref as *const () as usize,
            face_release as *const () as usize,
            face_get_type as *const () as usize,
            face_get_files as *const () as usize,
            face_get_index as *const () as usize,
            face_get_simulations as *const () as usize,
            face_is_symbol as *const () as usize,
            face_get_metrics as *const () as usize,
            face_get_glyph_count as *const () as usize,
            face_get_design_glyph_metrics as *const () as usize,
            face_get_glyph_indices as *const () as usize,
            face_try_get_font_table as *const () as usize,
            face_release_font_table as *const () as usize,
            crate::bridge_entry!(face_get_glyph_run_outline) as usize,
            crate::bridge_entry!(face_get_recommended_rendering_mode) as usize,
            crate::bridge_entry!(face_get_gdi_compatible_metrics) as usize,
            crate::bridge_entry!(face_get_gdi_compatible_glyph_metrics) as usize,
        ])
    })
    .as_ptr()
}

fn new_font_face(face: Arc<FontFaceInner>) -> *mut c_void {
    ComObj::new(font_face_vtbl(), face) as *mut c_void
}

pub(crate) unsafe fn font_face_from_ptr(ptr: *mut c_void) -> Option<Arc<FontFaceInner>> {
    if ptr.is_null() {
        return None;
    }
    let vtbl = (ptr as *const *const usize).read_unaligned();
    if vtbl == font_face_vtbl() {
        Some(Arc::clone(ComObj::inner(ptr as PFontFace)))
    } else {
        None
    }
}

// ============================================================================
// IDWriteFont Vtable (14 slots)
// ============================================================================

type PFont = *mut ComObj<FontInner>;

unsafe extern "win64" fn font_qi(this: PFont, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_FONT])
}
unsafe extern "win64" fn font_add_ref(this: PFont) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn font_release(this: PFont) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn font_get_family(this: PFont, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    out.write_unaligned(new_font_family(Arc::clone(&inner.collection), Arc::clone(&inner.family)));
    S_OK
}
unsafe extern "win64" fn font_get_weight(this: PFont) -> u32 {
    if this.is_null() { return DWRITE_FONT_WEIGHT_NORMAL; }
    ComObj::inner(this).meta.weight
}
unsafe extern "win64" fn font_get_stretch(this: PFont) -> u32 {
    if this.is_null() { return DWRITE_FONT_STRETCH_NORMAL; }
    ComObj::inner(this).meta.stretch
}
unsafe extern "win64" fn font_get_style(this: PFont) -> u32 {
    if this.is_null() { return DWRITE_FONT_STYLE_NORMAL; }
    ComObj::inner(this).meta.style
}
unsafe extern "win64" fn font_is_symbol(this: PFont) -> BOOL {
    if this.is_null() { return FALSE; }
    if ComObj::inner(this).meta.is_symbol { TRUE } else { FALSE }
}
unsafe extern "win64" fn font_get_face_names(this: PFont, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(new_localized_strings(ComObj::inner(this).meta.face_name.clone()));
    S_OK
}
unsafe extern "win64" fn font_get_info_strings(
    this: PFont,
    info_id: u32,
    out: *mut *mut c_void,
    exists: *mut BOOL,
) -> HRESULT {
    if this.is_null() || out.is_null() || exists.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    if let Some(s) = ComObj::inner(this).meta.info_strings.get(&info_id) {
        out.write_unaligned(new_localized_strings(s.clone()));
        exists.write_unaligned(TRUE);
    } else {
        exists.write_unaligned(FALSE);
    }
    S_OK
}
unsafe extern "win64" fn font_get_simulations(_this: PFont) -> u32 {
    DWRITE_FONT_SIMULATIONS_NONE
}
unsafe extern "win64" fn font_get_metrics(this: PFont, out: *mut DWRITE_FONT_METRICS) {
    if this.is_null() || out.is_null() {
        return;
    }
    out.write_unaligned(ComObj::inner(this).meta.metrics);
}
unsafe extern "win64" fn font_has_char(this: PFont, cp: u32, exists: *mut BOOL) -> HRESULT {
    if this.is_null() || exists.is_null() {
        return E_POINTER;
    }
    let face = FontFaceInner::from_meta(Arc::clone(&ComObj::inner(this).meta), DWRITE_FONT_SIMULATIONS_NONE);
    exists.write_unaligned(if face.has_character(cp) { TRUE } else { FALSE });
    S_OK
}
unsafe extern "win64" fn font_create_face(this: PFont, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let face = FontFaceInner::from_meta(Arc::clone(&ComObj::inner(this).meta), DWRITE_FONT_SIMULATIONS_NONE);
    out.write_unaligned(new_font_face(face));
    S_OK
}

fn font_vtbl() -> *const usize {
    static V: OnceLock<[usize; 14]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            font_qi as *const () as usize,
            font_add_ref as *const () as usize,
            font_release as *const () as usize,
            font_get_family as *const () as usize,
            font_get_weight as *const () as usize,
            font_get_stretch as *const () as usize,
            font_get_style as *const () as usize,
            font_is_symbol as *const () as usize,
            font_get_face_names as *const () as usize,
            font_get_info_strings as *const () as usize,
            font_get_simulations as *const () as usize,
            font_get_metrics as *const () as usize,
            font_has_char as *const () as usize,
            font_create_face as *const () as usize,
        ])
    })
    .as_ptr()
}

fn new_font(
    collection: Arc<FontCollectionData>,
    family: Arc<FontFamilyData>,
    meta: Arc<FontMeta>,
) -> *mut c_void {
    ComObj::new(font_vtbl(), FontInner { collection, family, meta }) as *mut c_void
}

// ============================================================================
// IDWriteFontList (6 slots) & IDWriteFontFamily (9 slots)
// ============================================================================

type PFontList = *mut ComObj<FontListInner>;

unsafe extern "win64" fn flist_qi(this: PFontList, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_FONT_LIST])
}
unsafe extern "win64" fn flist_add_ref(this: PFontList) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn flist_release(this: PFontList) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn flist_get_collection(this: PFontList, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(new_font_collection(Arc::clone(&ComObj::inner(this).collection)));
    S_OK
}
unsafe extern "win64" fn flist_get_count(this: PFontList) -> u32 {
    if this.is_null() { return 0; }
    ComObj::inner(this).fonts.len() as u32
}
unsafe extern "win64" fn flist_get_font(this: PFontList, idx: u32, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let inner = ComObj::inner(this);
    let Some(meta) = inner.fonts.get(idx as usize) else {
        return E_INVALIDARG;
    };
    let fam = inner
        .collection
        .find_family_index(&meta.family_name)
        .map(|i| Arc::clone(&inner.collection.families[i]))
        .unwrap_or_else(|| Arc::new(FontFamilyData { name: meta.family_name.clone(), fonts: vec![Arc::clone(meta)] }));
    out.write_unaligned(new_font(Arc::clone(&inner.collection), fam, Arc::clone(meta)));
    S_OK
}

fn font_list_vtbl() -> *const usize {
    static V: OnceLock<[usize; 6]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            flist_qi as *const () as usize,
            flist_add_ref as *const () as usize,
            flist_release as *const () as usize,
            flist_get_collection as *const () as usize,
            flist_get_count as *const () as usize,
            flist_get_font as *const () as usize,
        ])
    })
    .as_ptr()
}

type PFontFamily = *mut ComObj<FontFamilyInner>;

unsafe extern "win64" fn ffam_qi(this: PFontFamily, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_FONT_FAMILY, IID_IDWRITE_FONT_LIST])
}
unsafe extern "win64" fn ffam_add_ref(this: PFontFamily) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn ffam_release(this: PFontFamily) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn ffam_get_collection(this: PFontFamily, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(new_font_collection(Arc::clone(&ComObj::inner(this).collection)));
    S_OK
}
unsafe extern "win64" fn ffam_get_count(this: PFontFamily) -> u32 {
    if this.is_null() { return 0; }
    ComObj::inner(this).family.fonts.len() as u32
}
unsafe extern "win64" fn ffam_get_font(this: PFontFamily, idx: u32, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let inner = ComObj::inner(this);
    let Some(meta) = inner.family.fonts.get(idx as usize) else {
        return E_INVALIDARG;
    };
    out.write_unaligned(new_font(Arc::clone(&inner.collection), Arc::clone(&inner.family), Arc::clone(meta)));
    S_OK
}
unsafe extern "win64" fn ffam_get_names(this: PFontFamily, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(new_localized_strings(ComObj::inner(this).family.name.clone()));
    S_OK
}
unsafe extern "win64" fn ffam_get_first_matching(
    this: PFontFamily,
    weight: u32,
    stretch: u32,
    style: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    if inner.family.fonts.is_empty() {
        out.write_unaligned(std::ptr::null_mut());
        return DWRITE_E_NOFONT;
    }
    let meta = pick_best_font(&inner.family.fonts, weight, stretch, style);
    out.write_unaligned(new_font(Arc::clone(&inner.collection), Arc::clone(&inner.family), meta));
    S_OK
}
unsafe extern "win64" fn ffam_get_matching_fonts(
    this: PFontFamily,
    weight: u32,
    stretch: u32,
    style: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    let mut sorted = inner.family.fonts.clone();
    sorted.sort_by_key(|m| font_match_score(m, weight, stretch, style));
    let list = ComObj::new(
        font_list_vtbl(),
        FontListInner { collection: Arc::clone(&inner.collection), fonts: sorted },
    );
    out.write_unaligned(list as *mut c_void);
    S_OK
}

fn font_family_vtbl() -> *const usize {
    static V: OnceLock<[usize; 9]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            ffam_qi as *const () as usize,
            ffam_add_ref as *const () as usize,
            ffam_release as *const () as usize,
            ffam_get_collection as *const () as usize,
            ffam_get_count as *const () as usize,
            ffam_get_font as *const () as usize,
            ffam_get_names as *const () as usize,
            ffam_get_first_matching as *const () as usize,
            ffam_get_matching_fonts as *const () as usize,
        ])
    })
    .as_ptr()
}

fn new_font_family(collection: Arc<FontCollectionData>, family: Arc<FontFamilyData>) -> *mut c_void {
    ComObj::new(font_family_vtbl(), FontFamilyInner { collection, family }) as *mut c_void
}

// ============================================================================
// IDWriteFontCollection Vtable (7 slots)
// ============================================================================

type PFontCollection = *mut ComObj<FontCollectionInner>;

unsafe extern "win64" fn fcol_qi(this: PFontCollection, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_FONT_COLLECTION])
}
unsafe extern "win64" fn fcol_add_ref(this: PFontCollection) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn fcol_release(this: PFontCollection) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn fcol_get_family_count(this: PFontCollection) -> u32 {
    if this.is_null() { return 0; }
    ComObj::inner(this).data.families.len() as u32
}
unsafe extern "win64" fn fcol_get_family(this: PFontCollection, idx: u32, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let data = &ComObj::inner(this).data;
    let Some(fam) = data.families.get(idx as usize) else {
        return E_INVALIDARG;
    };
    out.write_unaligned(new_font_family(Arc::clone(data), Arc::clone(fam)));
    S_OK
}
unsafe extern "win64" fn fcol_find_family_name(
    this: PFontCollection,
    family_name: *const u16,
    index: *mut u32,
    exists: *mut BOOL,
) -> HRESULT {
    if this.is_null() || index.is_null() || exists.is_null() {
        return E_POINTER;
    }
    let name = read_wide_z(family_name);
    if let Some(idx) = ComObj::inner(this).data.find_family_index(&name) {
        index.write_unaligned(idx as u32);
        exists.write_unaligned(TRUE);
    } else {
        index.write_unaligned(u32::MAX);
        exists.write_unaligned(FALSE);
    }
    S_OK
}
unsafe extern "win64" fn fcol_get_font_from_face(
    this: PFontCollection,
    font_face: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let Some(face) = font_face_from_ptr(font_face) else {
        return DWRITE_E_NOFONT;
    };
    let data = &ComObj::inner(this).data;
    if let Some(idx) = data.find_family_index(&face.meta.family_name) {
        let fam = Arc::clone(&data.families[idx]);
        let meta = pick_best_font(&fam.fonts, face.meta.weight, face.meta.stretch, face.meta.style);
        out.write_unaligned(new_font(Arc::clone(data), fam, meta));
        return S_OK;
    }
    DWRITE_E_NOFONT
}

fn font_collection_vtbl() -> *const usize {
    static V: OnceLock<[usize; 7]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            fcol_qi as *const () as usize,
            fcol_add_ref as *const () as usize,
            fcol_release as *const () as usize,
            fcol_get_family_count as *const () as usize,
            fcol_get_family as *const () as usize,
            fcol_find_family_name as *const () as usize,
            fcol_get_font_from_face as *const () as usize,
        ])
    })
    .as_ptr()
}

fn new_font_collection(data: Arc<FontCollectionData>) -> *mut c_void {
    ComObj::new(font_collection_vtbl(), FontCollectionInner { data }) as *mut c_void
}

unsafe fn font_collection_data_from_ptr(ptr: *mut c_void) -> Arc<FontCollectionData> {
    if !ptr.is_null() && (ptr as *const *const usize).read_unaligned() == font_collection_vtbl() {
        Arc::clone(&ComObj::inner(ptr as PFontCollection).data)
    } else {
        system_font_collection_data(false)
    }
}

// ============================================================================
// IDWriteRenderingParams Vtable (8 slots)
// ============================================================================

type PRenderParams = *mut ComObj<RenderingParamsInner>;

unsafe extern "win64" fn rp_qi(this: PRenderParams, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_RENDERING_PARAMS])
}
unsafe extern "win64" fn rp_add_ref(this: PRenderParams) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn rp_release(this: PRenderParams) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn rp_get_gamma(this: PRenderParams) -> f32 {
    if this.is_null() { 1.8 } else { ComObj::inner(this).gamma }
}
unsafe extern "win64" fn rp_get_contrast(this: PRenderParams) -> f32 {
    if this.is_null() { 0.5 } else { ComObj::inner(this).enhanced_contrast }
}
unsafe extern "win64" fn rp_get_cleartype(this: PRenderParams) -> f32 {
    if this.is_null() { 0.5 } else { ComObj::inner(this).clear_type_level }
}
unsafe extern "win64" fn rp_get_pixel_geom(this: PRenderParams) -> u32 {
    if this.is_null() { DWRITE_PIXEL_GEOMETRY_FLAT } else { ComObj::inner(this).pixel_geometry }
}
unsafe extern "win64" fn rp_get_rendering_mode(this: PRenderParams) -> u32 {
    if this.is_null() { DWRITE_RENDERING_MODE_DEFAULT } else { ComObj::inner(this).rendering_mode }
}

fn rendering_params_vtbl() -> *const usize {
    static V: OnceLock<[usize; 8]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            rp_qi as *const () as usize,
            rp_add_ref as *const () as usize,
            rp_release as *const () as usize,
            rp_get_gamma as *const () as usize,
            rp_get_contrast as *const () as usize,
            rp_get_cleartype as *const () as usize,
            rp_get_pixel_geom as *const () as usize,
            rp_get_rendering_mode as *const () as usize,
        ])
    })
    .as_ptr()
}

fn new_rendering_params(gamma: f32, contrast: f32, ct: f32, pg: u32, rm: u32) -> *mut c_void {
    ComObj::new(
        rendering_params_vtbl(),
        RenderingParamsInner {
            gamma,
            enhanced_contrast: contrast,
            clear_type_level: ct,
            pixel_geometry: pg,
            rendering_mode: rm,
        },
    ) as *mut c_void
}

// ============================================================================
// IDWriteTypography Vtable (6 slots)
// ============================================================================

type PTypography = *mut ComObj<TypographyInner>;

unsafe extern "win64" fn typo_qi(this: PTypography, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_TYPOGRAPHY])
}
unsafe extern "win64" fn typo_add_ref(this: PTypography) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn typo_release(this: PTypography) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn typo_add_feature(this: PTypography, feature_packed: u64) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    ComObj::inner(this).features.lock().unwrap().push(DWRITE_FONT_FEATURE::from_u64(feature_packed));
    S_OK
}
unsafe extern "win64" fn typo_get_count(this: PTypography) -> u32 {
    if this.is_null() { return 0; }
    ComObj::inner(this).features.lock().unwrap().len() as u32
}
unsafe extern "win64" fn typo_get_feature(
    this: PTypography,
    index: u32,
    out: *mut DWRITE_FONT_FEATURE,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let feats = ComObj::inner(this).features.lock().unwrap();
    let Some(&f) = feats.get(index as usize) else {
        return E_INVALIDARG;
    };
    out.write_unaligned(f);
    S_OK
}

fn typography_vtbl() -> *const usize {
    static V: OnceLock<[usize; 6]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            typo_qi as *const () as usize,
            typo_add_ref as *const () as usize,
            typo_release as *const () as usize,
            typo_add_feature as *const () as usize,
            typo_get_count as *const () as usize,
            typo_get_feature as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// IDWriteBitmapRenderTarget (11 slots) & IDWriteGdiInterop (8 slots)
// ============================================================================

type PBmpRT = *mut ComObj<BitmapRenderTargetInner>;

unsafe extern "win64" fn brt_qi(this: PBmpRT, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_BITMAP_RENDER_TARGET])
}
unsafe extern "win64" fn brt_add_ref(this: PBmpRT) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn brt_release(this: PBmpRT) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn brt_draw_glyph_run(
    this: PBmpRT,
    baseline_x: f32,
    baseline_y: f32,
    _measuring_mode: u32,
    glyph_run: *const DWRITE_GLYPH_RUN,
    _rendering_params: *mut c_void,
    text_color: u32,
    black_box_rect: *mut [i32; 4],
) -> HRESULT {
    if this.is_null() || glyph_run.is_null() {
        return E_POINTER;
    }
    let run = glyph_run.read_unaligned();
    let st = ComObj::inner(this).state.lock().unwrap();
    let face = font_face_from_ptr(run.fontFace).unwrap_or_else(|| {
        system_font_collection_data(false).resolve_face(
            "Segoe UI",
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
        )
    });

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut pen_x = baseline_x;

    for i in 0..(run.glyphCount as usize) {
        let gid = if !run.glyphIndices.is_null() {
            run.glyphIndices.add(i).read_unaligned()
        } else {
            0
        };
        let adv = if !run.glyphAdvances.is_null() {
            run.glyphAdvances.add(i).read_unaligned()
        } else {
            face.scaled_advance(gid, run.fontEmSize)
        };
        let off = if !run.glyphOffsets.is_null() {
            run.glyphOffsets.add(i).read_unaligned()
        } else {
            DWRITE_GLYPH_OFFSET::default()
        };
        let glyph = rasterize_glyph(&face, gid, run.fontEmSize * st.pixels_per_dip.max(1.0));
        let gx0 = (pen_x + off.advanceOffset).round() as i32 + glyph.offset_x;
        let gy0 = (baseline_y - off.ascenderOffset).round() as i32 + glyph.offset_y;
        for py in 0..glyph.height {
            for px in 0..glyph.width {
                let a = glyph.alpha[(py * glyph.width + px) as usize];
                if a >= 64 {
                    let dx = gx0 + px as i32;
                    let dy = gy0 + py as i32;
                    if dx >= 0 && dy >= 0 && (dx as u32) < st.width && (dy as u32) < st.height {
                        if st.hdc != 0 {
                            crate::misc::gdi32::SetPixel(st.hdc, dx, dy, text_color);
                        }
                        min_x = min_x.min(dx);
                        min_y = min_y.min(dy);
                        max_x = max_x.max(dx + 1);
                        max_y = max_y.max(dy + 1);
                    }
                }
            }
        }
        pen_x += adv;
    }

    if !black_box_rect.is_null() {
        let rect = if min_x <= max_x && min_y <= max_y {
            [min_x, min_y, max_x, max_y]
        } else {
            [0, 0, 0, 0]
        };
        black_box_rect.write_unaligned(rect);
    }
    S_OK
}
crate::win64_bridge!(fn brt_draw_glyph_run(
    PBmpRT, f32, f32, u32, *const DWRITE_GLYPH_RUN, *mut c_void, u32, *mut [i32; 4]
) -> HRESULT);

unsafe extern "win64" fn brt_get_memory_dc(this: PBmpRT) -> usize {
    if this.is_null() { return 0; }
    ComObj::inner(this).state.lock().unwrap().hdc
}
unsafe extern "win64" fn brt_get_ppd(this: PBmpRT) -> f32 {
    if this.is_null() { return 1.0; }
    ComObj::inner(this).state.lock().unwrap().pixels_per_dip
}
unsafe extern "win64" fn brt_set_ppd(this: PBmpRT, ppd: f32) -> HRESULT {
    if this.is_null() || !ppd.is_finite() || ppd <= 0.0 {
        return E_INVALIDARG;
    }
    ComObj::inner(this).state.lock().unwrap().pixels_per_dip = ppd;
    S_OK
}
crate::win64_bridge!(fn brt_set_ppd(PBmpRT, f32) -> HRESULT);

unsafe extern "win64" fn brt_get_transform(this: PBmpRT, out: *mut DWRITE_MATRIX) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(ComObj::inner(this).state.lock().unwrap().transform);
    S_OK
}
unsafe extern "win64" fn brt_set_transform(this: PBmpRT, m: *const DWRITE_MATRIX) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    let mat = if m.is_null() { DWRITE_MATRIX::default() } else { m.read_unaligned() };
    ComObj::inner(this).state.lock().unwrap().transform = mat;
    S_OK
}
unsafe extern "win64" fn brt_get_size(this: PBmpRT, size: *mut [i32; 2]) -> HRESULT {
    if this.is_null() || size.is_null() {
        return E_POINTER;
    }
    let st = ComObj::inner(this).state.lock().unwrap();
    size.write_unaligned([st.width as i32, st.height as i32]);
    S_OK
}
unsafe extern "win64" fn brt_resize(this: PBmpRT, width: u32, height: u32) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let w = width.max(1);
    let h = height.max(1);
    let mut bmi = [0u8; 40];
    bmi[0..4].copy_from_slice(&40u32.to_le_bytes());
    bmi[4..8].copy_from_slice(&(w as i32).to_le_bytes());
    bmi[8..12].copy_from_slice(&(-(h as i32)).to_le_bytes());
    bmi[12..14].copy_from_slice(&1u16.to_le_bytes());
    bmi[14..16].copy_from_slice(&32u16.to_le_bytes());
    let mut bits = std::ptr::null_mut();
    let new_hbm = crate::misc::gdi32::CreateDIBSection(st.hdc, bmi.as_ptr(), 0, &mut bits, std::ptr::null_mut(), 0);
    if new_hbm != 0 {
        crate::misc::gdi32::SelectObject(st.hdc, new_hbm);
        if st.hbm != 0 {
            crate::misc::gdi32::DeleteObject(st.hbm);
        }
        st.hbm = new_hbm;
        st.width = w;
        st.height = h;
    }
    S_OK
}

fn bitmap_rt_vtbl() -> *const usize {
    static V: OnceLock<[usize; 11]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            brt_qi as *const () as usize,
            brt_add_ref as *const () as usize,
            brt_release as *const () as usize,
            crate::bridge_entry!(brt_draw_glyph_run) as usize,
            brt_get_memory_dc as *const () as usize,
            brt_get_ppd as *const () as usize,
            crate::bridge_entry!(brt_set_ppd) as usize,
            brt_get_transform as *const () as usize,
            brt_set_transform as *const () as usize,
            brt_get_size as *const () as usize,
            brt_resize as *const () as usize,
        ])
    })
    .as_ptr()
}

type PGdiInterop = *mut ComObj<GdiInteropInner>;

unsafe extern "win64" fn gdi_qi(this: PGdiInterop, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_GDI_INTEROP])
}
unsafe extern "win64" fn gdi_add_ref(this: PGdiInterop) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn gdi_release(this: PGdiInterop) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn gdi_create_font_from_logfont(
    this: PGdiInterop,
    lf: *const LOGFONTW,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || lf.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let l = lf.read_unaligned();
    let n = l.lfFaceName.iter().position(|&c| c == 0).unwrap_or(32);
    let name = String::from_utf16_lossy(&l.lfFaceName[..n]);
    let weight = if l.lfWeight > 0 { l.lfWeight as u32 } else { DWRITE_FONT_WEIGHT_NORMAL };
    let style = if l.lfItalic != 0 { DWRITE_FONT_STYLE_ITALIC } else { DWRITE_FONT_STYLE_NORMAL };
    let col = &ComObj::inner(this).collection;
    let idx = col
        .find_family_index(&name)
        .or_else(|| col.find_family_index("Segoe UI"))
        .unwrap_or(0);
    let fam = Arc::clone(&col.families[idx]);
    let meta = pick_best_font(&fam.fonts, weight, DWRITE_FONT_STRETCH_NORMAL, style);
    out.write_unaligned(new_font(Arc::clone(col), fam, meta));
    S_OK
}

fn fill_logfont_from_meta(meta: &FontMeta) -> LOGFONTW {
    let mut lf = LOGFONTW {
        lfHeight: 0,
        lfWeight: meta.weight as i32,
        lfItalic: if meta.style != DWRITE_FONT_STYLE_NORMAL { 1 } else { 0 },
        lfPitchAndFamily: if meta.is_monospace { 1 } else { 2 },
        ..LOGFONTW::default()
    };
    for (i, u) in meta.family_name.encode_utf16().take(31).enumerate() {
        lf.lfFaceName[i] = u;
    }
    lf
}

unsafe extern "win64" fn gdi_convert_font_to_logfont(
    this: PGdiInterop,
    font: *mut c_void,
    lf: *mut LOGFONTW,
    is_system_font: *mut BOOL,
) -> HRESULT {
    if this.is_null() || font.is_null() || lf.is_null() {
        return E_POINTER;
    }
    if (font as *const *const usize).read_unaligned() != font_vtbl() {
        return E_INVALIDARG;
    }
    let meta = &ComObj::inner(font as PFont).meta;
    lf.write_unaligned(fill_logfont_from_meta(meta));
    if !is_system_font.is_null() {
        is_system_font.write_unaligned(TRUE);
    }
    S_OK
}
unsafe extern "win64" fn gdi_convert_face_to_logfont(
    this: PGdiInterop,
    face: *mut c_void,
    lf: *mut LOGFONTW,
) -> HRESULT {
    if this.is_null() || face.is_null() || lf.is_null() {
        return E_POINTER;
    }
    let Some(f) = font_face_from_ptr(face) else {
        return E_INVALIDARG;
    };
    lf.write_unaligned(fill_logfont_from_meta(&f.meta));
    S_OK
}
unsafe extern "win64" fn gdi_create_face_from_hdc(
    this: PGdiInterop,
    _hdc: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let face = ComObj::inner(this).collection.resolve_face(
        "Segoe UI",
        DWRITE_FONT_WEIGHT_NORMAL,
        DWRITE_FONT_STRETCH_NORMAL,
        DWRITE_FONT_STYLE_NORMAL,
    );
    out.write_unaligned(new_font_face(face));
    S_OK
}
unsafe extern "win64" fn gdi_create_bitmap_rt(
    this: PGdiInterop,
    hdc: usize,
    width: u32,
    height: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let w = width.max(1);
    let h = height.max(1);
    let mem_dc = crate::misc::gdi32::CreateCompatibleDC(hdc);
    let mut bmi = [0u8; 40];
    bmi[0..4].copy_from_slice(&40u32.to_le_bytes());
    bmi[4..8].copy_from_slice(&(w as i32).to_le_bytes());
    bmi[8..12].copy_from_slice(&(-(h as i32)).to_le_bytes());
    bmi[12..14].copy_from_slice(&1u16.to_le_bytes());
    bmi[14..16].copy_from_slice(&32u16.to_le_bytes());
    let mut bits = std::ptr::null_mut();
    let hbm = crate::misc::gdi32::CreateDIBSection(mem_dc, bmi.as_ptr(), 0, &mut bits, std::ptr::null_mut(), 0);
    let old_hbm = if hbm != 0 {
        crate::misc::gdi32::SelectObject(mem_dc, hbm)
    } else {
        0
    };
    let rt = ComObj::new(
        bitmap_rt_vtbl(),
        BitmapRenderTargetInner {
            state: Mutex::new(BitmapRenderTargetState {
                hdc: mem_dc,
                hbm,
                old_hbm,
                width: w,
                height: h,
                pixels_per_dip: 1.0,
                transform: DWRITE_MATRIX::default(),
            }),
        },
    );
    out.write_unaligned(rt as *mut c_void);
    S_OK
}

fn gdi_interop_vtbl() -> *const usize {
    static V: OnceLock<[usize; 8]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            gdi_qi as *const () as usize,
            gdi_add_ref as *const () as usize,
            gdi_release as *const () as usize,
            gdi_create_font_from_logfont as *const () as usize,
            gdi_convert_font_to_logfont as *const () as usize,
            gdi_convert_face_to_logfont as *const () as usize,
            gdi_create_face_from_hdc as *const () as usize,
            gdi_create_bitmap_rt as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// IDWriteInlineObject (7 slots) & IDWriteNumberSubstitution (3 slots)
// ============================================================================

type PInlineObj = *mut ComObj<EllipsisInlineObjectInner>;

unsafe extern "win64" fn inline_qi(this: PInlineObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_INLINE_OBJECT])
}
unsafe extern "win64" fn inline_add_ref(this: PInlineObj) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn inline_release(this: PInlineObj) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn inline_draw(
    _this: PInlineObj,
    _ctx: *mut c_void,
    _renderer: *mut c_void,
    _ox: f32,
    _oy: f32,
    _sideways: BOOL,
    _rtl: BOOL,
    _effect: *mut c_void,
) -> HRESULT {
    S_OK
}
crate::win64_bridge!(fn inline_draw(
    PInlineObj, *mut c_void, *mut c_void, f32, f32, BOOL, BOOL, *mut c_void
) -> HRESULT);

unsafe extern "win64" fn inline_get_metrics(this: PInlineObj, out: *mut DWRITE_INLINE_OBJECT_METRICS) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let inner = ComObj::inner(this);
    out.write_unaligned(DWRITE_INLINE_OBJECT_METRICS {
        width: inner.font_size * 0.9,
        height: inner.height,
        baseline: inner.ascent,
        supportsSideways: FALSE,
    });
    S_OK
}
unsafe extern "win64" fn inline_get_overhang(_this: PInlineObj, out: *mut DWRITE_OVERHANG_METRICS) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(DWRITE_OVERHANG_METRICS::default());
    S_OK
}
unsafe extern "win64" fn inline_get_break(_this: PInlineObj, before: *mut u32, after: *mut u32) -> HRESULT {
    if !before.is_null() { before.write_unaligned(0); }
    if !after.is_null() { after.write_unaligned(0); }
    S_OK
}

fn inline_obj_vtbl() -> *const usize {
    static V: OnceLock<[usize; 7]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            inline_qi as *const () as usize,
            inline_add_ref as *const () as usize,
            inline_release as *const () as usize,
            crate::bridge_entry!(inline_draw) as usize,
            inline_get_metrics as *const () as usize,
            inline_get_overhang as *const () as usize,
            inline_get_break as *const () as usize,
        ])
    })
    .as_ptr()
}

type PNumSub = *mut ComObj<NumberSubstitutionInner>;

unsafe extern "win64" fn numsub_qi(this: PNumSub, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_NUMBER_SUBSTITUTION])
}
unsafe extern "win64" fn numsub_add_ref(this: PNumSub) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn numsub_release(this: PNumSub) -> u32 {
    ComObj::release(this)
}

fn num_sub_vtbl() -> *const usize {
    static V: OnceLock<[usize; 3]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            numsub_qi as *const () as usize,
            numsub_add_ref as *const () as usize,
            numsub_release as *const () as usize,
        ])
    })
    .as_ptr()
}

// ============================================================================
// IDWriteTextFormat Vtable (28 slots)
// ============================================================================

type PTextFormat = *mut ComObj<TextFormatInner>;

unsafe extern "win64" fn tf_qi(this: PTextFormat, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_TEXT_FORMAT])
}
unsafe extern "win64" fn tf_add_ref(this: PTextFormat) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn tf_release(this: PTextFormat) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn tf_set_text_align(this: PTextFormat, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_TEXT_ALIGNMENT_JUSTIFIED { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().text_alignment = align;
    S_OK
}
unsafe extern "win64" fn tf_set_para_align(this: PTextFormat, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_PARAGRAPH_ALIGNMENT_CENTER { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().paragraph_alignment = align;
    S_OK
}
unsafe extern "win64" fn tf_set_word_wrap(this: PTextFormat, wrap: u32) -> HRESULT {
    if this.is_null() || wrap > DWRITE_WORD_WRAPPING_CHARACTER { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().word_wrapping = wrap;
    S_OK
}
unsafe extern "win64" fn tf_set_reading_dir(this: PTextFormat, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().reading_direction = dir;
    S_OK
}
unsafe extern "win64" fn tf_set_flow_dir(this: PTextFormat, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().flow_direction = dir;
    S_OK
}
unsafe extern "win64" fn tf_set_tab_stop(this: PTextFormat, tab: f32) -> HRESULT {
    if this.is_null() || !tab.is_finite() || tab <= 0.0 { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().incremental_tab_stop = tab;
    S_OK
}
crate::win64_bridge!(fn tf_set_tab_stop(PTextFormat, f32) -> HRESULT);

unsafe extern "win64" fn tf_set_trimming(
    this: PTextFormat,
    opts: *const DWRITE_TRIMMING,
    sign: *mut c_void,
) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.trimming = opts.read_unaligned();
    st.trimming_sign = ComPtr::retain(sign);
    S_OK
}
unsafe extern "win64" fn tf_set_line_spacing(
    this: PTextFormat,
    method: u32,
    spacing: f32,
    baseline: f32,
) -> HRESULT {
    if this.is_null() || method > DWRITE_LINE_SPACING_METHOD_PROPORTIONAL || !spacing.is_finite() || spacing < 0.0 {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.line_spacing_method = method;
    st.line_spacing = spacing;
    st.baseline = baseline;
    S_OK
}
crate::win64_bridge!(fn tf_set_line_spacing(PTextFormat, u32, f32, f32) -> HRESULT);

unsafe extern "win64" fn tf_get_text_align(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().text_alignment }
}
unsafe extern "win64" fn tf_get_para_align(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().paragraph_alignment }
}
unsafe extern "win64" fn tf_get_word_wrap(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().word_wrapping }
}
unsafe extern "win64" fn tf_get_reading_dir(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().reading_direction }
}
unsafe extern "win64" fn tf_get_flow_dir(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().flow_direction }
}
unsafe extern "win64" fn tf_get_tab_stop(this: PTextFormat) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().incremental_tab_stop }
}
unsafe extern "win64" fn tf_get_trimming(
    this: PTextFormat,
    opts: *mut DWRITE_TRIMMING,
    sign: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    opts.write_unaligned(st.trimming);
    if !sign.is_null() {
        if let Some(ref p) = st.trimming_sign {
            raw_com_add_ref(p.as_ptr());
            sign.write_unaligned(p.as_ptr());
        } else {
            sign.write_unaligned(std::ptr::null_mut());
        }
    }
    S_OK
}
unsafe extern "win64" fn tf_get_line_spacing(
    this: PTextFormat,
    method: *mut u32,
    spacing: *mut f32,
    baseline: *mut f32,
) -> HRESULT {
    if this.is_null() || method.is_null() || spacing.is_null() || baseline.is_null() {
        return E_POINTER;
    }
    let st = ComObj::inner(this).state.lock().unwrap();
    method.write_unaligned(st.line_spacing_method);
    spacing.write_unaligned(st.line_spacing);
    baseline.write_unaligned(st.baseline);
    S_OK
}
unsafe extern "win64" fn tf_get_font_collection(this: PTextFormat, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    out.write_unaligned(new_font_collection(Arc::clone(&st.collection)));
    S_OK
}
unsafe extern "win64" fn tf_get_family_len(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().family_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tf_get_family_name(this: PTextFormat, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let st = ComObj::inner(this).state.lock().unwrap();
    write_wide_z(&st.family_name, buf, size)
}
unsafe extern "win64" fn tf_get_weight(this: PTextFormat) -> u32 {
    if this.is_null() { DWRITE_FONT_WEIGHT_NORMAL } else { ComObj::inner(this).state.lock().unwrap().weight }
}
unsafe extern "win64" fn tf_get_style(this: PTextFormat) -> u32 {
    if this.is_null() { DWRITE_FONT_STYLE_NORMAL } else { ComObj::inner(this).state.lock().unwrap().style }
}
unsafe extern "win64" fn tf_get_stretch(this: PTextFormat) -> u32 {
    if this.is_null() { DWRITE_FONT_STRETCH_NORMAL } else { ComObj::inner(this).state.lock().unwrap().stretch }
}
unsafe extern "win64" fn tf_get_font_size(this: PTextFormat) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().font_size }
}
unsafe extern "win64" fn tf_get_locale_len(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().locale_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tf_get_locale_name(this: PTextFormat, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let st = ComObj::inner(this).state.lock().unwrap();
    write_wide_z(&st.locale_name, buf, size)
}

fn text_format_vtbl() -> *const usize {
    static V: OnceLock<[usize; 28]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            tf_qi as *const () as usize,
            tf_add_ref as *const () as usize,
            tf_release as *const () as usize,
            tf_set_text_align as *const () as usize,
            tf_set_para_align as *const () as usize,
            tf_set_word_wrap as *const () as usize,
            tf_set_reading_dir as *const () as usize,
            tf_set_flow_dir as *const () as usize,
            crate::bridge_entry!(tf_set_tab_stop) as usize,
            tf_set_trimming as *const () as usize,
            crate::bridge_entry!(tf_set_line_spacing) as usize,
            tf_get_text_align as *const () as usize,
            tf_get_para_align as *const () as usize,
            tf_get_word_wrap as *const () as usize,
            tf_get_reading_dir as *const () as usize,
            tf_get_flow_dir as *const () as usize,
            tf_get_tab_stop as *const () as usize,
            tf_get_trimming as *const () as usize,
            tf_get_line_spacing as *const () as usize,
            tf_get_font_collection as *const () as usize,
            tf_get_family_len as *const () as usize,
            tf_get_family_name as *const () as usize,
            tf_get_weight as *const () as usize,
            tf_get_style as *const () as usize,
            tf_get_stretch as *const () as usize,
            tf_get_font_size as *const () as usize,
            tf_get_locale_len as *const () as usize,
            tf_get_locale_name as *const () as usize,
        ])
    })
    .as_ptr()
}

unsafe fn text_format_state_from_ptr(ptr: *mut c_void) -> Option<TextFormatState> {
    if ptr.is_null() {
        return None;
    }
    let vtbl = (ptr as *const *const usize).read_unaligned();
    if vtbl == text_format_vtbl() {
        Some(ComObj::inner(ptr as PTextFormat).state.lock().unwrap().clone())
    } else if vtbl == text_layout_vtbl() {
        Some(ComObj::inner(ptr as PTextLayout).state.lock().unwrap().fmt.clone())
    } else {
        None
    }
}

// ============================================================================
// IDWriteTextLayout Vtable (67 slots: 28 inherited from IDWriteTextFormat + 39)
// ============================================================================

type PTextLayout = *mut ComObj<TextLayoutInner>;

unsafe extern "win64" fn tl_qi(this: PTextLayout, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(this, riid, ppv, &[IID_IDWRITE_TEXT_LAYOUT, IID_IDWRITE_TEXT_FORMAT])
}
unsafe extern "win64" fn tl_add_ref(this: PTextLayout) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn tl_release(this: PTextLayout) -> u32 {
    ComObj::release(this)
}
// Inherited IDWriteTextFormat methods (slots 3..27) forwarding to `st.fmt` + marking dirty on write.
unsafe extern "win64" fn tl_set_text_align(this: PTextLayout, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_TEXT_ALIGNMENT_JUSTIFIED { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.text_alignment = align;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_para_align(this: PTextLayout, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_PARAGRAPH_ALIGNMENT_CENTER { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.paragraph_alignment = align;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_word_wrap(this: PTextLayout, wrap: u32) -> HRESULT {
    if this.is_null() || wrap > DWRITE_WORD_WRAPPING_CHARACTER { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.word_wrapping = wrap;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_reading_dir(this: PTextLayout, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.reading_direction = dir;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_flow_dir(this: PTextLayout, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.flow_direction = dir;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_tab_stop(this: PTextLayout, tab: f32) -> HRESULT {
    if this.is_null() || !tab.is_finite() || tab <= 0.0 { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.incremental_tab_stop = tab;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_tab_stop(PTextLayout, f32) -> HRESULT);

unsafe extern "win64" fn tl_set_trimming(this: PTextLayout, opts: *const DWRITE_TRIMMING, sign: *mut c_void) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.trimming = opts.read_unaligned();
    st.fmt.trimming_sign = ComPtr::retain(sign);
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_line_spacing(this: PTextLayout, method: u32, spacing: f32, baseline: f32) -> HRESULT {
    if this.is_null() || method > DWRITE_LINE_SPACING_METHOD_PROPORTIONAL || !spacing.is_finite() || spacing < 0.0 {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.line_spacing_method = method;
    st.fmt.line_spacing = spacing;
    st.fmt.baseline = baseline;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_line_spacing(PTextLayout, u32, f32, f32) -> HRESULT);

unsafe extern "win64" fn tl_get_text_align(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.text_alignment }
}
unsafe extern "win64" fn tl_get_para_align(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.paragraph_alignment }
}
unsafe extern "win64" fn tl_get_word_wrap(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.word_wrapping }
}
unsafe extern "win64" fn tl_get_reading_dir(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.reading_direction }
}
unsafe extern "win64" fn tl_get_flow_dir(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.flow_direction }
}
unsafe extern "win64" fn tl_get_tab_stop(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().fmt.incremental_tab_stop }
}
unsafe extern "win64" fn tl_get_trimming(this: PTextLayout, opts: *mut DWRITE_TRIMMING, sign: *mut *mut c_void) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    opts.write_unaligned(st.fmt.trimming);
    if !sign.is_null() {
        if let Some(ref p) = st.fmt.trimming_sign {
            raw_com_add_ref(p.as_ptr());
            sign.write_unaligned(p.as_ptr());
        } else {
            sign.write_unaligned(std::ptr::null_mut());
        }
    }
    S_OK
}
unsafe extern "win64" fn tl_get_line_spacing(this: PTextLayout, method: *mut u32, spacing: *mut f32, baseline: *mut f32) -> HRESULT {
    if this.is_null() || method.is_null() || spacing.is_null() || baseline.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    method.write_unaligned(st.fmt.line_spacing_method);
    spacing.write_unaligned(st.fmt.line_spacing);
    baseline.write_unaligned(st.fmt.baseline);
    S_OK
}
unsafe extern "win64" fn tl_fmt_get_collection(this: PTextLayout, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    out.write_unaligned(new_font_collection(Arc::clone(&ComObj::inner(this).state.lock().unwrap().fmt.collection)));
    S_OK
}
unsafe extern "win64" fn tl_fmt_get_family_len(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.family_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tl_fmt_get_family_name(this: PTextLayout, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    write_wide_z(&ComObj::inner(this).state.lock().unwrap().fmt.family_name, buf, size)
}
unsafe extern "win64" fn tl_fmt_get_weight(this: PTextLayout) -> u32 {
    if this.is_null() { DWRITE_FONT_WEIGHT_NORMAL } else { ComObj::inner(this).state.lock().unwrap().fmt.weight }
}
unsafe extern "win64" fn tl_fmt_get_style(this: PTextLayout) -> u32 {
    if this.is_null() { DWRITE_FONT_STYLE_NORMAL } else { ComObj::inner(this).state.lock().unwrap().fmt.style }
}
unsafe extern "win64" fn tl_fmt_get_stretch(this: PTextLayout) -> u32 {
    if this.is_null() { DWRITE_FONT_STRETCH_NORMAL } else { ComObj::inner(this).state.lock().unwrap().fmt.stretch }
}
unsafe extern "win64" fn tl_fmt_get_font_size(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().fmt.font_size }
}
unsafe extern "win64" fn tl_fmt_get_locale_len(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.locale_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tl_fmt_get_locale_name(this: PTextLayout, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    write_wide_z(&ComObj::inner(this).state.lock().unwrap().fmt.locale_name, buf, size)
}

// IDWriteTextLayout-specific setters (slots 28..41)
unsafe extern "win64" fn tl_set_max_width(this: PTextLayout, max_w: f32) -> HRESULT {
    if this.is_null() || max_w.is_nan() || max_w < 0.0 { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.max_width = max_w;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_max_width(PTextLayout, f32) -> HRESULT);

unsafe extern "win64" fn tl_set_max_height(this: PTextLayout, max_h: f32) -> HRESULT {
    if this.is_null() || max_h.is_nan() || max_h < 0.0 { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.max_height = max_h;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_max_height(PTextLayout, f32) -> HRESULT);

unsafe extern "win64" fn tl_set_font_collection(this: PTextLayout, col: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let data = font_collection_data_from_ptr(col);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.collection = Arc::clone(&data);
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_family(this: PTextLayout, name: *const u16, range: u64) -> HRESULT {
    if this.is_null() || name.is_null() { return E_POINTER; }
    let fam = read_wide_z(name);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.family_name = fam.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_weight(this: PTextLayout, weight: u32, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.weight = weight;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_style(this: PTextLayout, style: u32, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.style = style;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_stretch(this: PTextLayout, stretch: u32, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.stretch = stretch;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_size(this: PTextLayout, size: f32, range: u64) -> HRESULT {
    if this.is_null() || !size.is_finite() || size <= 0.0 { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.font_size = size;
    });
    S_OK
}
crate::win64_bridge!(fn tl_set_font_size(PTextLayout, f32, u64) -> HRESULT);

unsafe extern "win64" fn tl_set_underline(this: PTextLayout, has: BOOL, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let v = has != 0;
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.underline = v;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_strikethrough(this: PTextLayout, has: BOOL, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let v = has != 0;
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.strikethrough = v;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_drawing_effect(this: PTextLayout, effect: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let retained = ComPtr::retain(effect);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.drawing_effect = retained.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_inline_object(this: PTextLayout, obj: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let retained = ComPtr::retain(obj);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.inline_object = retained.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_typography(this: PTextLayout, typo: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let retained = ComPtr::retain(typo);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.typography = retained.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_locale_name(this: PTextLayout, loc: *const u16, range: u64) -> HRESULT {
    if this.is_null() || loc.is_null() { return E_POINTER; }
    let s = read_wide_z(loc);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.locale_name = s.clone();
    });
    S_OK
}

// IDWriteTextLayout-specific getters (slots 42..57)
unsafe extern "win64" fn tl_get_max_width(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().max_width }
}
unsafe extern "win64" fn tl_get_max_height(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().max_height }
}
unsafe extern "win64" fn tl_get_font_collection(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(new_font_collection(p.collection));
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_family_len(
    this: PTextLayout,
    pos: u32,
    len: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || len.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    len.write_unaligned(p.family_name.encode_utf16().count() as u32);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_family_name(
    this: PTextLayout,
    pos: u32,
    buf: *mut u16,
    size: u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || buf.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    let hr = write_wide_z(&p.family_name, buf, size);
    if hr == S_OK && !range.is_null() { range.write_unaligned(r); }
    hr
}
unsafe extern "win64" fn tl_get_weight(
    this: PTextLayout,
    pos: u32,
    out: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.weight);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_style(
    this: PTextLayout,
    pos: u32,
    out: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.style);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_stretch(
    this: PTextLayout,
    pos: u32,
    out: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.stretch);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_font_size(
    this: PTextLayout,
    pos: u32,
    out: *mut f32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.font_size);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_underline(
    this: PTextLayout,
    pos: u32,
    out: *mut BOOL,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(if p.underline { TRUE } else { FALSE });
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_strikethrough(
    this: PTextLayout,
    pos: u32,
    out: *mut BOOL,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(if p.strikethrough { TRUE } else { FALSE });
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_drawing_effect(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    if let Some(ref e) = p.drawing_effect {
        raw_com_add_ref(e.as_ptr());
        out.write_unaligned(e.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_inline_object(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    if let Some(ref o) = p.inline_object {
        raw_com_add_ref(o.as_ptr());
        out.write_unaligned(o.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_typography(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    if let Some(ref t) = p.typography {
        raw_com_add_ref(t.as_ptr());
        out.write_unaligned(t.as_ptr());
    } else {
        out.write_unaligned(std::ptr::null_mut());
    }
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_locale_len(
    this: PTextLayout,
    pos: u32,
    len: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || len.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    len.write_unaligned(p.locale_name.encode_utf16().count() as u32);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_locale_name(
    this: PTextLayout,
    pos: u32,
    buf: *mut u16,
    size: u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || buf.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    let hr = write_wide_z(&p.locale_name, buf, size);
    if hr == S_OK && !range.is_null() { range.write_unaligned(r); }
    hr
}

// Helper to build positioned drawable glyph runs from a computed layout.
pub(crate) struct DrawableGlyphRun {
    pub font_face: Arc<FontFaceInner>,
    pub em_size: f32,
    pub baseline_x: f32,
    pub baseline_y: f32,
    pub line_height: f32,
    pub glyph_indices: Vec<u16>,
    pub glyph_advances: Vec<f32>,
    pub glyph_offsets: Vec<DWRITE_GLYPH_OFFSET>,
    pub utf16_slice: Vec<u16>,
    pub cluster_map: Vec<u16>,
    pub text_position: u32,
    pub locale_wide: Vec<u16>,
    pub drawing_effect: *mut c_void,
    pub underline: Option<(f32, f32, f32)>,     // (width, offset, thickness)
    pub strikethrough: Option<(f32, f32, f32)>, // (width, offset, thickness)
}

fn build_drawable_runs(st: &mut TextLayoutState) -> Vec<DrawableGlyphRun> {
    st.ensure_layout();
    let layout = st.cached.as_ref().unwrap();
    let mut out = Vec::new();

    for line in &layout.lines {
        let cs = line.cluster_start;
        let ce = line.cluster_end;
        let mut idx = cs;
        while idx < ce {
            let c0 = &layout.clusters[idx];
            if c0.is_newline {
                idx += 1;
                continue;
            }
            let run_start = idx;
            let eff0 = c0.drawing_effect.as_ref().map_or(std::ptr::null_mut(), |p| p.as_ptr());
            let mut run_end = idx + 1;
            while run_end < ce {
                let cn = &layout.clusters[run_end];
                if cn.is_newline {
                    break;
                }
                let effn = cn.drawing_effect.as_ref().map_or(std::ptr::null_mut(), |p| p.as_ptr());
                if !Arc::ptr_eq(&cn.face, &c0.face)
                    || (cn.font_size - c0.font_size).abs() > 1e-4
                    || cn.underline != c0.underline
                    || cn.strikethrough != c0.strikethrough
                    || effn != eff0
                {
                    break;
                }
                run_end += 1;
            }

            let count = run_end - run_start;
            let mut gids = Vec::with_capacity(count);
            let mut advs = Vec::with_capacity(count);
            let mut offs = Vec::with_capacity(count);
            let mut cmap = Vec::new();
            let mut run_w = 0.0f32;
            let text_pos_start = layout.clusters[run_start].text_pos;
            let last_c = &layout.clusters[run_end - 1];
            let text_pos_end = (last_c.text_pos + last_c.utf16_len as u32) as usize;

            for (gi, ci) in (run_start..run_end).enumerate() {
                let c = &layout.clusters[ci];
                let w = line.cluster_widths[ci - cs];
                gids.push(c.gid);
                advs.push(w);
                offs.push(DWRITE_GLYPH_OFFSET::default());
                run_w += w;
                for _ in 0..c.utf16_len {
                    cmap.push(gi as u16);
                }
            }

            let u_start = (text_pos_start as usize).min(st.text.len());
            let u_end = text_pos_end.min(st.text.len());
            let utf16_slice = st.text[u_start..u_end].to_vec();
            let locale_wide: Vec<u16> = c0.locale_name.encode_utf16().chain(std::iter::once(0)).collect();

            let m = &c0.face.meta.metrics;
            let upem = m.designUnitsPerEm.max(1) as f32;
            let scale = c0.font_size / upem;
            let underline = if c0.underline {
                Some((run_w, -(m.underlinePosition as f32) * scale, (m.underlineThickness as f32 * scale).max(1.0)))
            } else {
                None
            };
            let strikethrough = if c0.strikethrough {
                Some((run_w, -(m.strikethroughPosition as f32) * scale, (m.strikethroughThickness as f32 * scale).max(1.0)))
            } else {
                None
            };

            out.push(DrawableGlyphRun {
                font_face: Arc::clone(&c0.face),
                em_size: c0.font_size,
                baseline_x: line.cluster_lefts[run_start - cs],
                baseline_y: line.top + line.baseline,
                line_height: line.height,
                glyph_indices: gids,
                glyph_advances: advs,
                glyph_offsets: offs,
                utf16_slice,
                cluster_map: cmap,
                text_position: text_pos_start,
                locale_wide,
                drawing_effect: eff0,
                underline,
                strikethrough,
            });
            idx = run_end;
        }
    }
    out
}

pub(crate) unsafe fn extract_layout_runs(layout_ptr: *mut c_void) -> Option<Vec<DrawableGlyphRun>> {
    if layout_ptr.is_null() {
        return None;
    }
    if (layout_ptr as *const *const usize).read_unaligned() != text_layout_vtbl() {
        return None;
    }
    let mut st = ComObj::inner(layout_ptr as PTextLayout).state.lock().unwrap();
    Some(build_drawable_runs(&mut st))
}

pub(crate) unsafe fn layout_text_from_format(
    text: &[u16],
    format_ptr: *mut c_void,
    max_w: f32,
    max_h: f32,
) -> Option<Vec<DrawableGlyphRun>> {
    let fmt = text_format_state_from_ptr(format_ptr)?;
    let mut st = TextLayoutState::new(text.to_vec(), fmt, max_w, max_h);
    Some(build_drawable_runs(&mut st))
}

// Layout / Draw / Metrics / HitTesting (slots 58..66)
unsafe extern "win64" fn tl_draw(
    this: PTextLayout,
    client_ctx: *mut c_void,
    renderer: *mut c_void,
    origin_x: f32,
    origin_y: f32,
) -> HRESULT {
    if this.is_null() || renderer.is_null() {
        return E_POINTER;
    }
    let vtbl = (renderer as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return E_INVALIDARG;
    }
    let fn_draw_glyph_run = vtbl.add(6).read_unaligned();
    let fn_draw_underline = vtbl.add(7).read_unaligned();
    let fn_draw_strikethrough = vtbl.add(8).read_unaligned();

    let (runs, reading_dir, flow_dir) = {
        let mut st = ComObj::inner(this).state.lock().unwrap();
        let rd = st.fmt.reading_direction;
        let fd = st.fmt.flow_direction;
        (build_drawable_runs(&mut st), rd, fd)
    };

    type DrawGlyphRunFn = unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        f32,
        f32,
        u32,
        *const DWRITE_GLYPH_RUN,
        *const DWRITE_GLYPH_RUN_DESCRIPTION,
        *mut c_void,
    ) -> HRESULT;
    type DrawUnderlineFn = unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        f32,
        f32,
        *const DWRITE_UNDERLINE,
        *mut c_void,
    ) -> HRESULT;
    type DrawStrikeFn = unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        f32,
        f32,
        *const DWRITE_STRIKETHROUGH,
        *mut c_void,
    ) -> HRESULT;

    for run in &runs {
        let face_com = new_font_face(Arc::clone(&run.font_face));
        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: face_com,
            fontEmSize: run.em_size,
            glyphCount: run.glyph_indices.len() as u32,
            glyphIndices: run.glyph_indices.as_ptr(),
            glyphAdvances: run.glyph_advances.as_ptr(),
            glyphOffsets: run.glyph_offsets.as_ptr(),
            isSideways: FALSE,
            bidiLevel: 0,
        };
        let desc = DWRITE_GLYPH_RUN_DESCRIPTION {
            localeName: run.locale_wide.as_ptr(),
            string: run.utf16_slice.as_ptr(),
            stringLength: run.utf16_slice.len() as u32,
            _pad0: 0,
            clusterMap: run.cluster_map.as_ptr(),
            textPosition: run.text_position,
            _pad1: 0,
        };
        let bx = origin_x + run.baseline_x;
        let by = origin_y + run.baseline_y;

        let hr = if fn_draw_glyph_run != 0 {
            let f: DrawGlyphRunFn = std::mem::transmute(fn_draw_glyph_run);
            f(
                renderer,
                client_ctx,
                bx,
                by,
                DWRITE_MEASURING_MODE_NATURAL,
                &glyph_run,
                &desc,
                run.drawing_effect,
            )
        } else {
            S_OK
        };
        raw_com_release(face_com);
        if hr < 0 {
            return hr;
        }

        if let Some((w, off, thick)) = run.underline {
            if fn_draw_underline != 0 {
                let ul = DWRITE_UNDERLINE {
                    width: w,
                    thickness: thick,
                    offset: off,
                    runHeight: run.line_height,
                    readingDirection: reading_dir,
                    flowDirection: flow_dir,
                    localeName: run.locale_wide.as_ptr(),
                    measuringMode: DWRITE_MEASURING_MODE_NATURAL,
                    _pad: 0,
                };
                let f: DrawUnderlineFn = std::mem::transmute(fn_draw_underline);
                let hr2 = f(renderer, client_ctx, bx, by, &ul, run.drawing_effect);
                if hr2 < 0 {
                    return hr2;
                }
            }
        }
        if let Some((w, off, thick)) = run.strikethrough {
            if fn_draw_strikethrough != 0 {
                let sk = DWRITE_STRIKETHROUGH {
                    width: w,
                    thickness: thick,
                    offset: off,
                    runHeight: run.line_height,
                    readingDirection: reading_dir,
                    flowDirection: flow_dir,
                    localeName: run.locale_wide.as_ptr(),
                    measuringMode: DWRITE_MEASURING_MODE_NATURAL,
                    _pad: 0,
                };
                let f: DrawStrikeFn = std::mem::transmute(fn_draw_strikethrough);
                let hr2 = f(renderer, client_ctx, bx, by, &sk, run.drawing_effect);
                if hr2 < 0 {
                    return hr2;
                }
            }
        }
    }
    S_OK
}
crate::win64_bridge!(fn tl_draw(PTextLayout, *mut c_void, *mut c_void, f32, f32) -> HRESULT);

unsafe extern "win64" fn tl_get_line_metrics(
    this: PTextLayout,
    line_metrics: *mut DWRITE_LINE_METRICS,
    max_count: u32,
    actual_count: *mut u32,
) -> HRESULT {
    if this.is_null() || actual_count.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    let n = layout.lines.len() as u32;
    actual_count.write_unaligned(n);
    if max_count < n || line_metrics.is_null() {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    for (i, line) in layout.lines.iter().enumerate() {
        line_metrics.add(i).write_unaligned(DWRITE_LINE_METRICS {
            length: line.utf16_len,
            trailingWhitespaceLength: line.trailing_ws_len,
            newlineLength: line.newline_len,
            height: line.height,
            baseline: line.baseline,
            isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
        });
    }
    S_OK
}
unsafe extern "win64" fn tl_get_metrics(this: PTextLayout, out: *mut DWRITE_TEXT_METRICS) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    out.write_unaligned(layout.metrics);
    S_OK
}
unsafe extern "win64" fn tl_get_overhang_metrics(
    this: PTextLayout,
    out: *mut DWRITE_OVERHANG_METRICS,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    out.write_unaligned(layout.overhang);
    S_OK
}
unsafe extern "win64" fn tl_get_cluster_metrics(
    this: PTextLayout,
    out: *mut DWRITE_CLUSTER_METRICS,
    max_count: u32,
    actual_count: *mut u32,
) -> HRESULT {
    if this.is_null() || actual_count.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    let n = layout.clusters.len() as u32;
    actual_count.write_unaligned(n);
    if max_count < n || (n > 0 && out.is_null()) {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    for (i, c) in layout.clusters.iter().enumerate() {
        let mut flags = 0u16;
        if c.can_wrap_after { flags |= 1 << 0; }
        if c.is_whitespace { flags |= 1 << 1; }
        if c.is_newline { flags |= 1 << 2; }
        out.add(i).write_unaligned(DWRITE_CLUSTER_METRICS {
            width: c.width,
            length: c.utf16_len,
            flags,
        });
    }
    S_OK
}
unsafe extern "win64" fn tl_determine_min_width(this: PTextLayout, min_width: *mut f32) -> HRESULT {
    if this.is_null() || min_width.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    min_width.write_unaligned(layout.min_width);
    S_OK
}
unsafe extern "win64" fn tl_hit_test_point(
    this: PTextLayout,
    point_x: f32,
    point_y: f32,
    is_trailing_hit: *mut BOOL,
    is_inside: *mut BOOL,
    metrics: *mut DWRITE_HIT_TEST_METRICS,
) -> HRESULT {
    if this.is_null() || is_trailing_hit.is_null() || is_inside.is_null() || metrics.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();

    let first_line = &layout.lines[0];
    let last_line = layout.lines.last().unwrap();
    let y_inside = point_y >= first_line.top && point_y < last_line.top + last_line.height;

    let line = if point_y < first_line.top {
        first_line
    } else if point_y >= last_line.top + last_line.height {
        last_line
    } else {
        layout
            .lines
            .iter()
            .find(|l| point_y >= l.top && point_y < l.top + l.height)
            .unwrap_or(last_line)
    };

    let cs = line.cluster_start;
    let ce = line.cluster_end;
    let mut vis_end = ce;
    while vis_end > cs && layout.clusters[vis_end - 1].is_newline {
        vis_end -= 1;
    }

    if cs == vis_end {
        is_trailing_hit.write_unaligned(FALSE);
        is_inside.write_unaligned(FALSE);
        metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
            textPosition: line.text_start,
            length: if cs < ce { layout.clusters[cs].utf16_len as u32 } else { 0 },
            left: line.left,
            top: line.top,
            width: 0.0,
            height: line.height,
            bidiLevel: 0,
            isText: TRUE,
            isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
        });
        return S_OK;
    }

    let line_right = line.cluster_lefts[vis_end - 1 - cs] + line.cluster_widths[vis_end - 1 - cs];
    let x_inside = point_x >= line.left && point_x < line_right;
    is_inside.write_unaligned(if y_inside && x_inside { TRUE } else { FALSE });

    let (chosen_ci, trailing) = if point_x < line.left {
        (cs, false)
    } else if point_x >= line_right {
        (vis_end - 1, true)
    } else {
        let mut found = (vis_end - 1, true);
        for ci in cs..vis_end {
            let cl = line.cluster_lefts[ci - cs];
            let cw = line.cluster_widths[ci - cs];
            if point_x < cl + cw {
                found = (ci, point_x >= cl + cw * 0.5);
                break;
            }
        }
        found
    };

    let c = &layout.clusters[chosen_ci];
    is_trailing_hit.write_unaligned(if trailing { TRUE } else { FALSE });
    metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
        textPosition: c.text_pos,
        length: c.utf16_len as u32,
        left: line.cluster_lefts[chosen_ci - cs],
        top: line.top,
        width: line.cluster_widths[chosen_ci - cs],
        height: line.height,
        bidiLevel: 0,
        isText: TRUE,
        isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
    });
    S_OK
}
crate::win64_bridge!(fn tl_hit_test_point(
    PTextLayout, f32, f32, *mut BOOL, *mut BOOL, *mut DWRITE_HIT_TEST_METRICS
) -> HRESULT);

unsafe extern "win64" fn tl_hit_test_text_position(
    this: PTextLayout,
    text_pos: u32,
    is_trailing_hit: BOOL,
    point_x: *mut f32,
    point_y: *mut f32,
    metrics: *mut DWRITE_HIT_TEST_METRICS,
) -> HRESULT {
    if this.is_null() || point_x.is_null() || point_y.is_null() || metrics.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let text_len = st.text.len() as u32;
    let layout = st.ensure_layout();

    for line in &layout.lines {
        let cs = line.cluster_start;
        let ce = line.cluster_end;
        for ci in cs..ce {
            let c = &layout.clusters[ci];
            if text_pos >= c.text_pos && text_pos < c.text_pos + c.utf16_len as u32 {
                let cl = line.cluster_lefts[ci - cs];
                let cw = line.cluster_widths[ci - cs];
                let px = if is_trailing_hit != 0 { cl + cw } else { cl };
                point_x.write_unaligned(px);
                point_y.write_unaligned(line.top);
                metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
                    textPosition: c.text_pos,
                    length: c.utf16_len as u32,
                    left: cl,
                    top: line.top,
                    width: cw,
                    height: line.height,
                    bidiLevel: 0,
                    isText: TRUE,
                    isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
                });
                return S_OK;
            }
        }
    }

    // Past end of text (or empty text): return caret at end of last line.
    let last = layout.lines.last().unwrap();
    let end_x = last.left + last.width_with_trailing_ws;
    point_x.write_unaligned(end_x);
    point_y.write_unaligned(last.top);
    metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
        textPosition: text_len,
        length: 0,
        left: end_x,
        top: last.top,
        width: 0.0,
        height: last.height,
        bidiLevel: 0,
        isText: TRUE,
        isTrimmed: if last.is_trimmed { TRUE } else { FALSE },
    });
    S_OK
}

unsafe extern "win64" fn tl_hit_test_text_range(
    this: PTextLayout,
    text_pos: u32,
    text_len: u32,
    origin_x: f32,
    origin_y: f32,
    out_metrics: *mut DWRITE_HIT_TEST_METRICS,
    max_count: u32,
    actual_count: *mut u32,
) -> HRESULT {
    if this.is_null() || actual_count.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    let range_end = text_pos.saturating_add(text_len);
    let mut rects: Vec<DWRITE_HIT_TEST_METRICS> = Vec::new();

    if text_len == 0 {
        // Zero-length range returns a single zero-width caret metric on the containing line.
        let last = layout.lines.last().unwrap();
        let mut found = DWRITE_HIT_TEST_METRICS {
            textPosition: text_pos,
            length: 0,
            left: origin_x + last.left + last.width_with_trailing_ws,
            top: origin_y + last.top,
            width: 0.0,
            height: last.height,
            bidiLevel: 0,
            isText: TRUE,
            isTrimmed: FALSE,
        };
        'outer: for line in &layout.lines {
            for ci in line.cluster_start..line.cluster_end {
                let c = &layout.clusters[ci];
                if text_pos >= c.text_pos && text_pos < c.text_pos + c.utf16_len as u32 {
                    found = DWRITE_HIT_TEST_METRICS {
                        textPosition: c.text_pos,
                        length: 0,
                        left: origin_x + line.cluster_lefts[ci - line.cluster_start],
                        top: origin_y + line.top,
                        width: 0.0,
                        height: line.height,
                        bidiLevel: 0,
                        isText: TRUE,
                        isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
                    };
                    break 'outer;
                }
            }
        }
        rects.push(found);
    } else {
        for line in &layout.lines {
            let cs = line.cluster_start;
            let ce = line.cluster_end;
            let mut first_ci: Option<usize> = None;
            let mut last_ci: Option<usize> = None;
            for ci in cs..ce {
                let c = &layout.clusters[ci];
                let c_end = c.text_pos + c.utf16_len as u32;
                if c_end > text_pos && c.text_pos < range_end {
                    if first_ci.is_none() {
                        first_ci = Some(ci);
                    }
                    last_ci = Some(ci);
                }
            }
            if let (Some(fci), Some(lci)) = (first_ci, last_ci) {
                let left = line.cluster_lefts[fci - cs];
                let right = line.cluster_lefts[lci - cs] + line.cluster_widths[lci - cs];
                let start_pos = layout.clusters[fci].text_pos;
                let end_pos = layout.clusters[lci].text_pos + layout.clusters[lci].utf16_len as u32;
                rects.push(DWRITE_HIT_TEST_METRICS {
                    textPosition: start_pos,
                    length: end_pos - start_pos,
                    left: origin_x + left,
                    top: origin_y + line.top,
                    width: right - left,
                    height: line.height,
                    bidiLevel: 0,
                    isText: TRUE,
                    isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
                });
            }
        }
    }

    let n = rects.len() as u32;
    actual_count.write_unaligned(n);
    if max_count < n || (n > 0 && out_metrics.is_null()) {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    for (i, r) in rects.iter().enumerate() {
        out_metrics.add(i).write_unaligned(*r);
    }
    S_OK
}
crate::win64_bridge!(fn tl_hit_test_text_range(
    PTextLayout, u32, u32, f32, f32, *mut DWRITE_HIT_TEST_METRICS, u32, *mut u32
) -> HRESULT);

fn text_layout_vtbl() -> *const usize {
    static V: OnceLock<[usize; 67]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            tl_qi as *const () as usize,                           // 0
            tl_add_ref as *const () as usize,                      // 1
            tl_release as *const () as usize,                      // 2
            tl_set_text_align as *const () as usize,               // 3
            tl_set_para_align as *const () as usize,               // 4
            tl_set_word_wrap as *const () as usize,                // 5
            tl_set_reading_dir as *const () as usize,              // 6
            tl_set_flow_dir as *const () as usize,                 // 7
            crate::bridge_entry!(tl_set_tab_stop) as usize,        // 8
            tl_set_trimming as *const () as usize,                 // 9
            crate::bridge_entry!(tl_set_line_spacing) as usize,    // 10
            tl_get_text_align as *const () as usize,               // 11
            tl_get_para_align as *const () as usize,               // 12
            tl_get_word_wrap as *const () as usize,                // 13
            tl_get_reading_dir as *const () as usize,              // 14
            tl_get_flow_dir as *const () as usize,                 // 15
            tl_get_tab_stop as *const () as usize,                 // 16
            tl_get_trimming as *const () as usize,                 // 17
            tl_get_line_spacing as *const () as usize,             // 18
            tl_fmt_get_collection as *const () as usize,           // 19
            tl_fmt_get_family_len as *const () as usize,           // 20
            tl_fmt_get_family_name as *const () as usize,          // 21
            tl_fmt_get_weight as *const () as usize,               // 22
            tl_fmt_get_style as *const () as usize,                // 23
            tl_fmt_get_stretch as *const () as usize,              // 24
            tl_fmt_get_font_size as *const () as usize,            // 25
            tl_fmt_get_locale_len as *const () as usize,           // 26
            tl_fmt_get_locale_name as *const () as usize,          // 27
            crate::bridge_entry!(tl_set_max_width) as usize,       // 28
            crate::bridge_entry!(tl_set_max_height) as usize,      // 29
            tl_set_font_collection as *const () as usize,          // 30
            tl_set_font_family as *const () as usize,              // 31
            tl_set_font_weight as *const () as usize,              // 32
            tl_set_font_style as *const () as usize,               // 33
            tl_set_font_stretch as *const () as usize,             // 34
            crate::bridge_entry!(tl_set_font_size) as usize,       // 35
            tl_set_underline as *const () as usize,                // 36
            tl_set_strikethrough as *const () as usize,            // 37
            tl_set_drawing_effect as *const () as usize,           // 38
            tl_set_inline_object as *const () as usize,            // 39
            tl_set_typography as *const () as usize,               // 40
            tl_set_locale_name as *const () as usize,              // 41
            tl_get_max_width as *const () as usize,                // 42
            tl_get_max_height as *const () as usize,               // 43
            tl_get_font_collection as *const () as usize,          // 44
            tl_get_family_len as *const () as usize,               // 45
            tl_get_family_name as *const () as usize,              // 46
            tl_get_weight as *const () as usize,                   // 47
            tl_get_style as *const () as usize,                    // 48
            tl_get_stretch as *const () as usize,                  // 49
            tl_get_font_size as *const () as usize,                // 50
            tl_get_underline as *const () as usize,                // 51
            tl_get_strikethrough as *const () as usize,            // 52
            tl_get_drawing_effect as *const () as usize,           // 53
            tl_get_inline_object as *const () as usize,            // 54
            tl_get_typography as *const () as usize,               // 55
            tl_get_locale_len as *const () as usize,               // 56
            tl_get_locale_name as *const () as usize,              // 57
            crate::bridge_entry!(tl_draw) as usize,                // 58
            tl_get_line_metrics as *const () as usize,             // 59
            tl_get_metrics as *const () as usize,                  // 60
            tl_get_overhang_metrics as *const () as usize,         // 61
            tl_get_cluster_metrics as *const () as usize,          // 62
            tl_determine_min_width as *const () as usize,          // 63
            crate::bridge_entry!(tl_hit_test_point) as usize,      // 64
            tl_hit_test_text_position as *const () as usize,       // 65
            crate::bridge_entry!(tl_hit_test_text_range) as usize, // 66
        ])
    })
    .as_ptr()
}

// ============================================================================
// IDWriteFactory Vtable (24 slots)
// ============================================================================

type PFactory = *mut ComObj<FactoryInner>;

unsafe extern "win64" fn fac_qi(this: PFactory, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper(
        this,
        riid,
        ppv,
        &[IID_IDWRITE_FACTORY, IID_IDWRITE_FACTORY1, IID_IDWRITE_FACTORY2, IID_IDWRITE_FACTORY3],
    )
}
unsafe extern "win64" fn fac_add_ref(this: PFactory) -> u32 {
    ComObj::add_ref(this)
}
unsafe extern "win64" fn fac_release(this: PFactory) -> u32 {
    ComObj::release(this)
}
unsafe extern "win64" fn fac_get_system_font_collection(
    this: PFactory,
    out: *mut *mut c_void,
    check_for_updates: BOOL,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let data = system_font_collection_data(check_for_updates != 0);
    out.write_unaligned(new_font_collection(data));
    S_OK
}
unsafe extern "win64" fn fac_create_custom_font_collection(
    _this: PFactory,
    _loader: *mut c_void,
    _key: *const c_void,
    _key_size: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        out.write_unaligned(std::ptr::null_mut());
    }
    crate::stub!("IDWriteFactory::CreateCustomFontCollection -> E_NOTIMPL");
    E_NOTIMPL
}
unsafe extern "win64" fn fac_register_font_collection_loader(_this: PFactory, loader: *mut c_void) -> HRESULT {
    if loader.is_null() { return E_INVALIDARG; }
    S_OK
}
unsafe extern "win64" fn fac_unregister_font_collection_loader(_this: PFactory, loader: *mut c_void) -> HRESULT {
    if loader.is_null() { return E_INVALIDARG; }
    S_OK
}
unsafe extern "win64" fn fac_create_font_file_reference(
    this: PFactory,
    file_path: *const u16,
    _last_write_time: *const FILETIME,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || file_path.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let win_path = read_wide_z(file_path);
    let host_path = if Path::new(&win_path).exists() {
        PathBuf::from(&win_path)
    } else if let Some(p) = crate::path::to_host(&win_path) {
        p
    } else {
        return DWRITE_E_FILENOTFOUND;
    };
    if !host_path.is_file() {
        return DWRITE_E_FILENOTFOUND;
    }
    out.write_unaligned(new_font_file(host_path));
    S_OK
}
unsafe extern "win64" fn fac_create_custom_font_file_reference(
    _this: PFactory,
    _key: *const c_void,
    _key_size: u32,
    _loader: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        out.write_unaligned(std::ptr::null_mut());
    }
    crate::stub!("IDWriteFactory::CreateCustomFontFileReference -> E_NOTIMPL");
    E_NOTIMPL
}
unsafe extern "win64" fn fac_create_font_face(
    this: PFactory,
    _face_type: u32,
    num_files: u32,
    font_files: *const *mut c_void,
    face_index: u32,
    simulations: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() || font_files.is_null() || num_files == 0 {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    let first = font_files.read_unaligned();
    if first.is_null() || (first as *const *const usize).read_unaligned() != font_file_vtbl() {
        return E_INVALIDARG;
    }
    let path = ComObj::inner(first as PFontFile).path.clone();
    let metas = parse_font_file_metadata(&path);
    let Some(meta) = metas.into_iter().nth(face_index as usize) else {
        return DWRITE_E_FILEFORMAT;
    };
    let face = FontFaceInner::from_meta(Arc::new(meta), simulations);
    out.write_unaligned(new_font_face(face));
    S_OK
}
unsafe extern "win64" fn fac_create_rendering_params(this: PFactory, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    out.write_unaligned(new_rendering_params(
        1.8,
        0.5,
        0.5,
        DWRITE_PIXEL_GEOMETRY_RGB,
        DWRITE_RENDERING_MODE_DEFAULT,
    ));
    S_OK
}
unsafe extern "win64" fn fac_create_monitor_rendering_params(
    this: PFactory,
    _monitor: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    fac_create_rendering_params(this, out)
}
unsafe extern "win64" fn fac_create_custom_rendering_params(
    this: PFactory,
    gamma: f32,
    enhanced_contrast: f32,
    clear_type_level: f32,
    pixel_geometry: u32,
    rendering_mode: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    if !gamma.is_finite() || gamma <= 0.0 || !enhanced_contrast.is_finite() || enhanced_contrast < 0.0 {
        return E_INVALIDARG;
    }
    out.write_unaligned(new_rendering_params(
        gamma,
        enhanced_contrast,
        clear_type_level.clamp(0.0, 1.0),
        pixel_geometry,
        rendering_mode,
    ));
    S_OK
}
crate::win64_bridge!(fn fac_create_custom_rendering_params(
    PFactory, f32, f32, f32, u32, u32, *mut *mut c_void
) -> HRESULT);

unsafe extern "win64" fn fac_register_font_file_loader(_this: PFactory, loader: *mut c_void) -> HRESULT {
    if loader.is_null() { return E_INVALIDARG; }
    S_OK
}
unsafe extern "win64" fn fac_unregister_font_file_loader(_this: PFactory, loader: *mut c_void) -> HRESULT {
    if loader.is_null() { return E_INVALIDARG; }
    S_OK
}
unsafe extern "win64" fn fac_create_text_format(
    this: PFactory,
    family_name: *const u16,
    font_collection: *mut c_void,
    weight: u32,
    style: u32,
    stretch: u32,
    font_size: f32,
    locale_name: *const u16,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    if !font_size.is_finite() || font_size <= 0.0 {
        return E_INVALIDARG;
    }
    let fam = read_wide_z(family_name);
    let loc = if locale_name.is_null() { "en-us".to_string() } else { read_wide_z(locale_name) };
    let col = font_collection_data_from_ptr(font_collection);
    let st = TextFormatState {
        family_name: if fam.is_empty() { "Segoe UI".to_string() } else { fam },
        collection: col,
        weight: if weight == 0 { DWRITE_FONT_WEIGHT_NORMAL } else { weight },
        style,
        stretch: if stretch == 0 { DWRITE_FONT_STRETCH_NORMAL } else { stretch },
        font_size,
        locale_name: loc,
        text_alignment: DWRITE_TEXT_ALIGNMENT_LEADING,
        paragraph_alignment: DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
        word_wrapping: DWRITE_WORD_WRAPPING_WRAP,
        reading_direction: 0,
        flow_direction: 0,
        incremental_tab_stop: font_size * 4.0,
        trimming: DWRITE_TRIMMING::default(),
        trimming_sign: None,
        line_spacing_method: DWRITE_LINE_SPACING_METHOD_DEFAULT,
        line_spacing: 0.0,
        baseline: 0.0,
    };
    let obj = ComObj::new(text_format_vtbl(), TextFormatInner { state: Mutex::new(st) });
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
crate::win64_bridge!(fn fac_create_text_format(
    PFactory, *const u16, *mut c_void, u32, u32, u32, f32, *const u16, *mut *mut c_void
) -> HRESULT);

unsafe extern "win64" fn fac_create_typography(this: PFactory, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let obj = ComObj::new(typography_vtbl(), TypographyInner { features: Mutex::new(Vec::new()) });
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_get_gdi_interop(this: PFactory, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let obj = ComObj::new(
        gdi_interop_vtbl(),
        GdiInteropInner { collection: system_font_collection_data(false) },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_text_layout(
    this: PFactory,
    string: *const u16,
    string_len: u32,
    text_format: *mut c_void,
    max_width: f32,
    max_height: f32,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() || text_format.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(std::ptr::null_mut());
    if string_len > 0 && string.is_null() {
        return E_POINTER;
    }
    let Some(fmt) = text_format_state_from_ptr(text_format) else {
        return E_INVALIDARG;
    };
    let units = read_wide_slice(string, string_len as usize);
    let st = TextLayoutState::new(units, fmt, max_width, max_height);
    let obj = ComObj::new(text_layout_vtbl(), TextLayoutInner { state: Mutex::new(st) });
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
crate::win64_bridge!(fn fac_create_text_layout(
    PFactory, *const u16, u32, *mut c_void, f32, f32, *mut *mut c_void
) -> HRESULT);

unsafe extern "win64" fn fac_create_gdi_compatible_text_layout(
    this: PFactory,
    string: *const u16,
    string_len: u32,
    text_format: *mut c_void,
    layout_width: f32,
    layout_height: f32,
    _pixels_per_dip: f32,
    _transform: *const DWRITE_MATRIX,
    _use_gdi_natural: BOOL,
    out: *mut *mut c_void,
) -> HRESULT {
    fac_create_text_layout(this, string, string_len, text_format, layout_width, layout_height, out)
}
crate::win64_bridge!(fn fac_create_gdi_compatible_text_layout(
    PFactory, *const u16, u32, *mut c_void, f32, f32, f32, *const DWRITE_MATRIX, BOOL, *mut *mut c_void
) -> HRESULT);

unsafe extern "win64" fn fac_create_ellipsis_trimming_sign(
    this: PFactory,
    text_format: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || text_format.is_null() || out.is_null() {
        return E_POINTER;
    }
    let Some(fmt) = text_format_state_from_ptr(text_format) else {
        return E_INVALIDARG;
    };
    let face = fmt.collection.resolve_face(&fmt.family_name, fmt.weight, fmt.stretch, fmt.style);
    let (h, bl) = line_metrics_for_face(&face, fmt.font_size, &fmt);
    let obj = ComObj::new(
        inline_obj_vtbl(),
        EllipsisInlineObjectInner { font_size: fmt.font_size, ascent: bl, height: h },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_text_analyzer(_this: PFactory, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        out.write_unaligned(std::ptr::null_mut());
    }
    crate::stub!("IDWriteFactory::CreateTextAnalyzer -> E_NOTIMPL");
    E_NOTIMPL
}
unsafe extern "win64" fn fac_create_number_substitution(
    this: PFactory,
    _method: u32,
    _locale: *const u16,
    _ignore_override: BOOL,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let obj = ComObj::new(num_sub_vtbl(), NumberSubstitutionInner);
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_glyph_run_analysis(
    _this: PFactory,
    _glyph_run: *const DWRITE_GLYPH_RUN,
    _pixels_per_dip: f32,
    _transform: *const DWRITE_MATRIX,
    _rendering_mode: u32,
    _measuring_mode: u32,
    _baseline_x: f32,
    _baseline_y: f32,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        out.write_unaligned(std::ptr::null_mut());
    }
    crate::stub!("IDWriteFactory::CreateGlyphRunAnalysis -> E_NOTIMPL");
    E_NOTIMPL
}
crate::win64_bridge!(fn fac_create_glyph_run_analysis(
    PFactory, *const DWRITE_GLYPH_RUN, f32, *const DWRITE_MATRIX, u32, u32, f32, f32, *mut *mut c_void
) -> HRESULT);

fn factory_vtbl() -> *const usize {
    static V: OnceLock<[usize; 24]> = OnceLock::new();
    V.get_or_init(|| {
        register_vtable([
            fac_qi as *const () as usize,                                           // 0
            fac_add_ref as *const () as usize,                                      // 1
            fac_release as *const () as usize,                                      // 2
            fac_get_system_font_collection as *const () as usize,                   // 3
            fac_create_custom_font_collection as *const () as usize,                // 4
            fac_register_font_collection_loader as *const () as usize,              // 5
            fac_unregister_font_collection_loader as *const () as usize,            // 6
            fac_create_font_file_reference as *const () as usize,                   // 7
            fac_create_custom_font_file_reference as *const () as usize,            // 8
            fac_create_font_face as *const () as usize,                             // 9
            fac_create_rendering_params as *const () as usize,                      // 10
            fac_create_monitor_rendering_params as *const () as usize,              // 11
            crate::bridge_entry!(fac_create_custom_rendering_params) as usize,      // 12
            fac_register_font_file_loader as *const () as usize,                    // 13
            fac_unregister_font_file_loader as *const () as usize,                  // 14
            crate::bridge_entry!(fac_create_text_format) as usize,                  // 15
            fac_create_typography as *const () as usize,                            // 16
            fac_get_gdi_interop as *const () as usize,                              // 17
            crate::bridge_entry!(fac_create_text_layout) as usize,                  // 18
            crate::bridge_entry!(fac_create_gdi_compatible_text_layout) as usize,   // 19
            fac_create_ellipsis_trimming_sign as *const () as usize,                // 20
            fac_create_text_analyzer as *const () as usize,                         // 21
            fac_create_number_substitution as *const () as usize,                   // 22
            crate::bridge_entry!(fac_create_glyph_run_analysis) as usize,           // 23
        ])
    })
    .as_ptr()
}

// ============================================================================
// Public DLL Export
// ============================================================================

pub unsafe extern "win64" fn DWriteCreateFactory(
    factory_type: u32,
    iid: *const u8,
    factory: *mut *mut c_void,
) -> HRESULT {
    if factory.is_null() {
        return E_POINTER;
    }
    factory.write_unaligned(std::ptr::null_mut());
    if factory_type != DWRITE_FACTORY_TYPE_SHARED && factory_type != DWRITE_FACTORY_TYPE_ISOLATED {
        return E_INVALIDARG;
    }
    if iid.is_null() {
        return E_INVALIDARG;
    }
    let req = (iid as *const GUID).read_unaligned();
    if req != IID_IUNKNOWN
        && req != IID_IDWRITE_FACTORY
        && req != IID_IDWRITE_FACTORY1
        && req != IID_IDWRITE_FACTORY2
        && req != IID_IDWRITE_FACTORY3
    {
        return E_NOINTERFACE;
    }
    let obj = ComObj::new(factory_vtbl(), FactoryInner { _factory_type: factory_type });
    factory.write_unaligned(obj as *mut c_void);
    S_OK
}

// ============================================================================
// Unit Tests (strictly at the bottom for build.rs export scanner)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    unsafe fn vtbl_of(obj: *mut c_void) -> *const usize {
        (obj as *const *const usize).read_unaligned()
    }

    unsafe fn release_com(obj: *mut c_void) -> u32 {
        let f: unsafe extern "win64" fn(*mut c_void) -> u32 =
            std::mem::transmute(vtbl_of(obj).add(2).read_unaligned());
        f(obj)
    }

    #[test]
    fn create_factory_and_qi_refcount() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            let hr = DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &IID_IDWRITE_FACTORY as *const GUID as *const u8,
                &mut fac,
            );
            assert_eq!(hr, S_OK);
            assert!(!fac.is_null());

            // QI for IDWriteFactory1 succeeds, random GUID returns E_NOINTERFACE.
            let qi: unsafe extern "win64" fn(*mut c_void, *const u8, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_of(fac).add(0).read_unaligned());
            let mut fac1: *mut c_void = std::ptr::null_mut();
            assert_eq!(qi(fac, &IID_IDWRITE_FACTORY1 as *const GUID as *const u8, &mut fac1), S_OK);
            assert_eq!(fac1, fac);

            let bogus = dw_guid(0xDEAD_BEEF, 1, 2, [3, 4, 5, 6, 7, 8, 9, 10]);
            let mut bad: *mut c_void = std::ptr::null_mut();
            assert_eq!(qi(fac, &bogus as *const GUID as *const u8, &mut bad), E_NOINTERFACE);
            assert!(bad.is_null());

            assert_eq!(release_com(fac1), 1);
            assert_eq!(release_com(fac), 0);
        }
    }

    #[test]
    fn font_collection_family_font_and_face_metrics() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DWriteCreateFactory(
                    DWRITE_FACTORY_TYPE_SHARED,
                    &IID_IDWRITE_FACTORY as *const GUID as *const u8,
                    &mut fac,
                ),
                S_OK
            );

            let get_col: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void, BOOL) -> HRESULT =
                std::mem::transmute(vtbl_of(fac).add(3).read_unaligned());
            let mut col: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_col(fac, &mut col, FALSE), S_OK);
            assert!(!col.is_null());

            let get_count: unsafe extern "win64" fn(*mut c_void) -> u32 =
                std::mem::transmute(vtbl_of(col).add(3).read_unaligned());
            let fam_count = get_count(col);
            assert!(fam_count >= 2, "expected system fonts + built-in fallback families, got {fam_count}");

            let find_fam: unsafe extern "win64" fn(*mut c_void, *const u16, *mut u32, *mut BOOL) -> HRESULT =
                std::mem::transmute(vtbl_of(col).add(5).read_unaligned());
            let mut idx = 0u32;
            let mut exists = FALSE;
            assert_eq!(find_fam(col, wide("Segoe UI").as_ptr(), &mut idx, &mut exists), S_OK);
            assert_eq!(exists, TRUE);

            let mut missing_idx = 0u32;
            let mut missing_exists = TRUE;
            assert_eq!(
                find_fam(col, wide("DefinitelyNotARealFont999").as_ptr(), &mut missing_idx, &mut missing_exists),
                S_OK
            );
            assert_eq!(missing_exists, FALSE);

            let get_fam: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_of(col).add(4).read_unaligned());
            let mut fam: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_fam(col, idx, &mut fam), S_OK);

            // Family name localized strings
            let get_names: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_of(fam).add(6).read_unaligned());
            let mut names: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_names(fam, &mut names), S_OK);
            let str_len_fn: unsafe extern "win64" fn(*mut c_void, u32, *mut u32) -> HRESULT =
                std::mem::transmute(vtbl_of(names).add(7).read_unaligned());
            let str_get_fn: unsafe extern "win64" fn(*mut c_void, u32, *mut u16, u32) -> HRESULT =
                std::mem::transmute(vtbl_of(names).add(8).read_unaligned());
            let mut nlen = 0u32;
            assert_eq!(str_len_fn(names, 0, &mut nlen), S_OK);
            let mut nbuf = vec![0u16; (nlen + 1) as usize];
            assert_eq!(str_get_fn(names, 0, nbuf.as_mut_ptr(), nlen + 1), S_OK);
            assert_eq!(String::from_utf16_lossy(&nbuf[..nlen as usize]), "Segoe UI");

            // First matching bold font + face + UTF-32 glyph mapping & metrics
            let get_match: unsafe extern "win64" fn(*mut c_void, u32, u32, u32, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_of(fam).add(7).read_unaligned());
            let mut font: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                get_match(fam, DWRITE_FONT_WEIGHT_BOLD, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, &mut font),
                S_OK
            );
            let get_weight: unsafe extern "win64" fn(*mut c_void) -> u32 =
                std::mem::transmute(vtbl_of(font).add(4).read_unaligned());
            assert_eq!(get_weight(font), DWRITE_FONT_WEIGHT_BOLD);

            let create_face: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_of(font).add(13).read_unaligned());
            let mut face: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_face(font, &mut face), S_OK);

            let get_metrics: unsafe extern "win64" fn(*mut c_void, *mut DWRITE_FONT_METRICS) =
                std::mem::transmute(vtbl_of(face).add(8).read_unaligned());
            let mut fm = DWRITE_FONT_METRICS::default();
            get_metrics(face, &mut fm);
            assert!(fm.designUnitsPerEm >= 64);
            assert!(fm.ascent > 0 && fm.descent > 0);

            let get_gids: unsafe extern "win64" fn(*mut c_void, *const u32, u32, *mut u16) -> HRESULT =
                std::mem::transmute(vtbl_of(face).add(11).read_unaligned());
            let cps = [b'H' as u32, b'i' as u32, b'!' as u32];
            let mut gids = [0u16; 3];
            assert_eq!(get_gids(face, cps.as_ptr(), 3, gids.as_mut_ptr()), S_OK);
            assert!(gids.iter().all(|&g| g != 0));

            let get_gm: unsafe extern "win64" fn(*mut c_void, *const u16, u32, *mut DWRITE_GLYPH_METRICS, BOOL) -> HRESULT =
                std::mem::transmute(vtbl_of(face).add(10).read_unaligned());
            let mut gms = [DWRITE_GLYPH_METRICS::default(); 3];
            assert_eq!(get_gm(face, gids.as_ptr(), 3, gms.as_mut_ptr(), FALSE), S_OK);
            assert!(gms[0].advanceWidth > 0);
            assert!(gms[0].advanceWidth > gms[1].advanceWidth, "'H' should be wider than 'i'");

            release_com(face);
            release_com(font);
            release_com(names);
            release_com(fam);
            release_com(col);
            release_com(fac);
        }
    }

    #[test]
    fn text_layout_wrapping_metrics_hit_testing_and_draw_callback() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DWriteCreateFactory(
                    DWRITE_FACTORY_TYPE_SHARED,
                    &IID_IDWRITE_FACTORY as *const GUID as *const u8,
                    &mut fac,
                ),
                S_OK
            );

            // Call impl directly for float-bearing factory methods in unit tests (also verify bridge below).
            let mut fmt: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_text_format(
                    fac as PFactory,
                    wide("Segoe UI").as_ptr(),
                    std::ptr::null_mut(),
                    DWRITE_FONT_WEIGHT_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    16.0,
                    wide("en-us").as_ptr(),
                    &mut fmt,
                ),
                S_OK
            );

            let text_u16: Vec<u16> = "Hello world from DirectWrite".encode_utf16().collect();
            let mut layout: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_text_layout(
                    fac as PFactory,
                    text_u16.as_ptr(),
                    text_u16.len() as u32,
                    fmt,
                    90.0,
                    200.0,
                    &mut layout,
                ),
                S_OK
            );

            // Apply underline on "Hello"
            let range_hello = 0u64 | (5u64 << 32);
            assert_eq!(tl_set_underline(layout as PTextLayout, TRUE, range_hello), S_OK);
            let mut has_ul = FALSE;
            let mut out_range = DWRITE_TEXT_RANGE::default();
            assert_eq!(tl_get_underline(layout as PTextLayout, 2, &mut has_ul, &mut out_range), S_OK);
            assert_eq!(has_ul, TRUE);
            assert_eq!(out_range.startPosition, 2);
            assert_eq!(out_range.length, 3);

            // Metrics & line wrapping: 90 DIPs at 16px should wrap into multiple lines
            let mut tm = DWRITE_TEXT_METRICS::default();
            assert_eq!(tl_get_metrics(layout as PTextLayout, &mut tm), S_OK);
            assert!(tm.lineCount >= 2, "expected wrapped lines, got {}", tm.lineCount);
            assert!(tm.width > 0.0 && tm.width <= 95.0);
            assert!(tm.height > 16.0);

            let mut actual_lines = 0u32;
            let hr_small = tl_get_line_metrics(layout as PTextLayout, std::ptr::null_mut(), 0, &mut actual_lines);
            assert_eq!(hr_small, E_NOT_SUFFICIENT_BUFFER);
            assert_eq!(actual_lines, tm.lineCount);
            let mut lms = vec![DWRITE_LINE_METRICS::default(); actual_lines as usize];
            assert_eq!(
                tl_get_line_metrics(layout as PTextLayout, lms.as_mut_ptr(), actual_lines, &mut actual_lines),
                S_OK
            );
            let sum_chars: u32 = lms.iter().map(|l| l.length).sum();
            assert_eq!(sum_chars, text_u16.len() as u32);

            // Overhang metrics
            let mut oh = DWRITE_OVERHANG_METRICS::default();
            assert_eq!(tl_get_overhang_metrics(layout as PTextLayout, &mut oh), S_OK);
            assert!(oh.left.is_finite() && oh.right.is_finite());

            // Hit-testing round-trip: position -> point -> hit_test_point
            let mut px = 0.0f32;
            let mut py = 0.0f32;
            let mut htm_pos = DWRITE_HIT_TEST_METRICS::default();
            assert_eq!(tl_hit_test_text_position(layout as PTextLayout, 1, FALSE, &mut px, &mut py, &mut htm_pos), S_OK);
            assert!(px > 0.0);

            let mut trailing = FALSE;
            let mut inside = FALSE;
            let mut htm_pt = DWRITE_HIT_TEST_METRICS::default();
            assert_eq!(
                tl_hit_test_point(layout as PTextLayout, px + 1.0, py + 4.0, &mut trailing, &mut inside, &mut htm_pt),
                S_OK
            );
            assert_eq!(inside, TRUE);
            assert_eq!(htm_pt.textPosition, 1);

            // Hit-test range spanning whole text should produce one rect per line
            let mut range_cnt = 0u32;
            assert_eq!(
                tl_hit_test_text_range(
                    layout as PTextLayout,
                    0,
                    text_u16.len() as u32,
                    10.0,
                    20.0,
                    std::ptr::null_mut(),
                    0,
                    &mut range_cnt,
                ),
                E_NOT_SUFFICIENT_BUFFER
            );
            assert_eq!(range_cnt, tm.lineCount);
            let mut range_rects = vec![DWRITE_HIT_TEST_METRICS::default(); range_cnt as usize];
            assert_eq!(
                tl_hit_test_text_range(
                    layout as PTextLayout,
                    0,
                    text_u16.len() as u32,
                    10.0,
                    20.0,
                    range_rects.as_mut_ptr(),
                    range_cnt,
                    &mut range_cnt,
                ),
                S_OK
            );
            assert!((range_rects[0].left - 10.0).abs() < 0.01);
            assert!((range_rects[0].top - 20.0).abs() < 0.01);

            // Draw invokes IDWriteTextRenderer::DrawGlyphRun and DrawUnderline
            #[repr(C)]
            struct MockRenderer {
                vtbl: *const usize,
                glyph_runs: u32,
                total_glyphs: u32,
                underlines: u32,
            }
            unsafe extern "win64" fn mr_qi(_: *mut c_void, _: *const u8, _: *mut *mut c_void) -> HRESULT { E_NOINTERFACE }
            unsafe extern "win64" fn mr_ar(_: *mut c_void) -> u32 { 1 }
            unsafe extern "win64" fn mr_rel(_: *mut c_void) -> u32 { 1 }
            unsafe extern "win64" fn mr_snap(_: *mut c_void, _: *mut c_void, _: *mut BOOL) -> HRESULT { S_OK }
            unsafe extern "win64" fn mr_xform(_: *mut c_void, _: *mut c_void, _: *mut DWRITE_MATRIX) -> HRESULT { S_OK }
            unsafe extern "win64" fn mr_ppd(_: *mut c_void, _: *mut c_void, _: *mut f32) -> HRESULT { S_OK }
            unsafe extern "win64" fn mr_draw_gr(
                this: *mut c_void,
                _ctx: *mut c_void,
                _bx: f32,
                _by: f32,
                _mm: u32,
                run: *const DWRITE_GLYPH_RUN,
                _desc: *const DWRITE_GLYPH_RUN_DESCRIPTION,
                _eff: *mut c_void,
            ) -> HRESULT {
                let m = &mut *(this as *mut MockRenderer);
                m.glyph_runs += 1;
                m.total_glyphs += (*run).glyphCount;
                S_OK
            }
            unsafe extern "win64" fn mr_draw_ul(
                this: *mut c_void,
                _ctx: *mut c_void,
                _bx: f32,
                _by: f32,
                _ul: *const DWRITE_UNDERLINE,
                _eff: *mut c_void,
            ) -> HRESULT {
                let m = &mut *(this as *mut MockRenderer);
                m.underlines += 1;
                S_OK
            }
            unsafe extern "win64" fn mr_draw_sk(
                _: *mut c_void, _: *mut c_void, _: f32, _: f32, _: *const DWRITE_STRIKETHROUGH, _: *mut c_void
            ) -> HRESULT { S_OK }
            unsafe extern "win64" fn mr_draw_io(
                _: *mut c_void, _: *mut c_void, _: f32, _: f32, _: *mut c_void, _: BOOL, _: BOOL, _: *mut c_void
            ) -> HRESULT { S_OK }

            let mr_vtbl: [usize; 10] = [
                mr_qi as *const () as usize,
                mr_ar as *const () as usize,
                mr_rel as *const () as usize,
                mr_snap as *const () as usize,
                mr_xform as *const () as usize,
                mr_ppd as *const () as usize,
                mr_draw_gr as *const () as usize,
                mr_draw_ul as *const () as usize,
                mr_draw_sk as *const () as usize,
                mr_draw_io as *const () as usize,
            ];
            let mut renderer = MockRenderer {
                vtbl: mr_vtbl.as_ptr(),
                glyph_runs: 0,
                total_glyphs: 0,
                underlines: 0,
            };
            assert_eq!(
                tl_draw(
                    layout as PTextLayout,
                    std::ptr::null_mut(),
                    &mut renderer as *mut MockRenderer as *mut c_void,
                    5.0,
                    10.0,
                ),
                S_OK
            );
            assert!(renderer.glyph_runs >= 2);
            assert_eq!(renderer.total_glyphs, text_u16.len() as u32);
            assert_eq!(renderer.underlines, 1);

            release_com(layout);
            release_com(fmt);
            release_com(fac);
        }
    }

    #[test]
    fn typography_and_gdi_interop_and_sfnt_rasterize() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DWriteCreateFactory(
                    DWRITE_FACTORY_TYPE_SHARED,
                    &IID_IDWRITE_FACTORY as *const GUID as *const u8,
                    &mut fac,
                ),
                S_OK
            );

            // Typography features
            let create_typo: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_of(fac).add(16).read_unaligned());
            let mut typo: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_typo(fac, &mut typo), S_OK);
            let tag_liga = u32::from_le_bytes(*b"liga");
            let feat_packed = (tag_liga as u64) | (1u64 << 32);
            assert_eq!(typo_add_feature(typo as PTypography, feat_packed), S_OK);
            assert_eq!(typo_get_count(typo as PTypography), 1);
            let mut read_feat = DWRITE_FONT_FEATURE::default();
            assert_eq!(typo_get_feature(typo as PTypography, 0, &mut read_feat), S_OK);
            assert_eq!(read_feat.nameTag, tag_liga);
            assert_eq!(read_feat.parameter, 1);

            // GdiInterop: LOGFONTW <-> IDWriteFont & BitmapRenderTarget
            let get_gdi: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_of(fac).add(17).read_unaligned());
            let mut gdi: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_gdi(fac, &mut gdi), S_OK);

            let mut lf = LOGFONTW { lfWeight: 700, ..LOGFONTW::default() };
            for (i, u) in "Consolas".encode_utf16().enumerate() {
                lf.lfFaceName[i] = u;
            }
            let mut font: *mut c_void = std::ptr::null_mut();
            assert_eq!(gdi_create_font_from_logfont(gdi as PGdiInterop, &lf, &mut font), S_OK);
            let mut lf_out = LOGFONTW::default();
            let mut is_sys = FALSE;
            assert_eq!(gdi_convert_font_to_logfont(gdi as PGdiInterop, font, &mut lf_out, &mut is_sys), S_OK);
            assert_eq!(is_sys, TRUE);
            assert_eq!(lf_out.lfWeight, 700);

            // Glyph rasterization produces non-empty alpha coverage for 'A'
            let mut face_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(font_create_face(font as PFont, &mut face_ptr), S_OK);
            let face = font_face_from_ptr(face_ptr).unwrap();
            let gid_a = face.codepoint_to_glyph(b'A' as u32);
            let bmp = rasterize_glyph(&face, gid_a, 24.0);
            assert!(bmp.width > 0 && bmp.height > 0);
            assert!(bmp.alpha.iter().any(|&a| a > 128), "expected lit pixels in rasterized 'A'");

            // BitmapRenderTarget draws onto a real GDI memory DC
            let mut brt: *mut c_void = std::ptr::null_mut();
            assert_eq!(gdi_create_bitmap_rt(gdi as PGdiInterop, 0, 64, 32, &mut brt), S_OK);
            let dc = brt_get_memory_dc(brt as PBmpRT);
            assert_ne!(dc, 0);
            let gids = [gid_a];
            let advs = [16.0f32];
            let run = DWRITE_GLYPH_RUN {
                fontFace: face_ptr,
                fontEmSize: 20.0,
                glyphCount: 1,
                glyphIndices: gids.as_ptr(),
                glyphAdvances: advs.as_ptr(),
                glyphOffsets: std::ptr::null(),
                isSideways: FALSE,
                bidiLevel: 0,
            };
            let mut bbox = [0i32; 4];
            assert_eq!(
                brt_draw_glyph_run(brt as PBmpRT, 4.0, 24.0, 0, &run, std::ptr::null_mut(), 0x0000_00FF, &mut bbox),
                S_OK
            );
            assert!(bbox[2] > bbox[0] && bbox[3] > bbox[1], "expected non-empty blackBoxRect: {bbox:?}");

            release_com(brt);
            release_com(face_ptr);
            release_com(font);
            release_com(gdi);
            release_com(typo);
            release_com(fac);
        }
    }

    /// Build a minimal, spec-compliant TrueType (.ttf) binary in memory with
    /// `head`, `hhea`, `maxp`, `OS/2`, `post`, `name`, `cmap` (format 4), `hmtx`, `loca`, `glyf`
    /// containing a real triangle glyph outline for 'A' (glyph 1).
    fn build_synthetic_ttf() -> Vec<u8> {
        let mut tables: Vec<([u8; 4], Vec<u8>)> = Vec::new();

        // head (54 bytes): unitsPerEm = 1000, indexToLocFormat = 1 (long u32)
        let mut head = vec![0u8; 54];
        head[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        head[12..16].copy_from_slice(&0x5F0F_3CF5u32.to_be_bytes()); // magic
        head[18..20].copy_from_slice(&1000u16.to_be_bytes()); // unitsPerEm
        head[36..38].copy_from_slice(&0i16.to_be_bytes());
        head[38..40].copy_from_slice(&(-200i16).to_be_bytes());
        head[40..42].copy_from_slice(&800i16.to_be_bytes());
        head[42..44].copy_from_slice(&800i16.to_be_bytes());
        head[50..52].copy_from_slice(&1i16.to_be_bytes()); // long loca
        tables.push((*b"head", head));

        // hhea (36 bytes): ascent=800, descent=-200, lineGap=100, numberOfHMetrics=2
        let mut hhea = vec![0u8; 36];
        hhea[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        hhea[4..6].copy_from_slice(&800i16.to_be_bytes());
        hhea[6..8].copy_from_slice(&(-200i16).to_be_bytes());
        hhea[8..10].copy_from_slice(&100i16.to_be_bytes());
        hhea[34..36].copy_from_slice(&2u16.to_be_bytes());
        tables.push((*b"hhea", hhea));

        // maxp (6 bytes): numGlyphs = 2 (.notdef + 'A')
        let mut maxp = vec![0u8; 6];
        maxp[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        maxp[4..6].copy_from_slice(&2u16.to_be_bytes());
        tables.push((*b"maxp", maxp));

        // OS/2 (96 bytes, version 2)
        let mut os2 = vec![0u8; 96];
        os2[0..2].copy_from_slice(&2u16.to_be_bytes());
        os2[4..6].copy_from_slice(&600u16.to_be_bytes()); // SemiBold weight
        os2[6..8].copy_from_slice(&5u16.to_be_bytes());   // Normal stretch
        os2[26..28].copy_from_slice(&50i16.to_be_bytes());
        os2[28..30].copy_from_slice(&300i16.to_be_bytes());
        os2[62..64].copy_from_slice(&0x0080u16.to_be_bytes()); // USE_TYPO_METRICS
        os2[68..70].copy_from_slice(&820i16.to_be_bytes());
        os2[70..72].copy_from_slice(&(-180i16).to_be_bytes());
        os2[72..74].copy_from_slice(&50i16.to_be_bytes());
        os2[86..88].copy_from_slice(&520i16.to_be_bytes()); // xHeight
        os2[88..90].copy_from_slice(&720i16.to_be_bytes()); // capHeight
        tables.push((*b"OS/2", os2));

        // post (32 bytes)
        let mut post = vec![0u8; 32];
        post[0..4].copy_from_slice(&0x0003_0000u32.to_be_bytes());
        post[8..10].copy_from_slice(&(-100i16).to_be_bytes());
        post[10..12].copy_from_slice(&50i16.to_be_bytes());
        tables.push((*b"post", post));

        // name table with Platform 3 family="SynthSans" (id 1) and subfamily="SemiBold" (id 2)
        let fam_u16: Vec<u8> = "SynthSans".encode_utf16().flat_map(|u| u.to_be_bytes()).collect();
        let sub_u16: Vec<u8> = "SemiBold".encode_utf16().flat_map(|u| u.to_be_bytes()).collect();
        let mut name = Vec::new();
        name.extend_from_slice(&0u16.to_be_bytes());
        name.extend_from_slice(&2u16.to_be_bytes()); // 2 records
        name.extend_from_slice(&(6u16 + 24).to_be_bytes()); // stringOffset
        // Record 1: family
        for v in [3u16, 1, 0x0409, 1, fam_u16.len() as u16, 0] {
            name.extend_from_slice(&v.to_be_bytes());
        }
        // Record 2: subfamily
        for v in [3u16, 1, 0x0409, 2, sub_u16.len() as u16, fam_u16.len() as u16] {
            name.extend_from_slice(&v.to_be_bytes());
        }
        name.extend_from_slice(&fam_u16);
        name.extend_from_slice(&sub_u16);
        tables.push((*b"name", name));

        // cmap format 4 mapping 'A' (0x41) -> glyph 1
        let mut cmap = Vec::new();
        cmap.extend_from_slice(&0u16.to_be_bytes());
        cmap.extend_from_slice(&1u16.to_be_bytes()); // 1 encoding record
        cmap.extend_from_slice(&3u16.to_be_bytes()); // Windows
        cmap.extend_from_slice(&1u16.to_be_bytes()); // Unicode BMP
        cmap.extend_from_slice(&12u32.to_be_bytes()); // subtable offset
        // Format 4 subtable (2 segments: 'A'..'A' and 0xFFFF..0xFFFF)
        cmap.extend_from_slice(&4u16.to_be_bytes()); // format
        cmap.extend_from_slice(&32u16.to_be_bytes()); // length
        cmap.extend_from_slice(&0u16.to_be_bytes()); // language
        cmap.extend_from_slice(&4u16.to_be_bytes()); // segCountX2 = 2*2
        cmap.extend_from_slice(&4u16.to_be_bytes());
        cmap.extend_from_slice(&1u16.to_be_bytes());
        cmap.extend_from_slice(&0u16.to_be_bytes());
        cmap.extend_from_slice(&0x0041u16.to_be_bytes()); // endCode[0]
        cmap.extend_from_slice(&0xFFFFu16.to_be_bytes()); // endCode[1]
        cmap.extend_from_slice(&0u16.to_be_bytes()); // reservedPad
        cmap.extend_from_slice(&0x0041u16.to_be_bytes()); // startCode[0]
        cmap.extend_from_slice(&0xFFFFu16.to_be_bytes()); // startCode[1]
        cmap.extend_from_slice(&(-0x0040i16).to_be_bytes()); // idDelta[0]: 0x41 - 0x40 = 1
        cmap.extend_from_slice(&1i16.to_be_bytes()); // idDelta[1]
        cmap.extend_from_slice(&0u16.to_be_bytes()); // idRangeOffset[0]
        cmap.extend_from_slice(&0u16.to_be_bytes()); // idRangeOffset[1]
        tables.push((*b"cmap", cmap));

        // hmtx: glyph 0 (adv=500, lsb=0), glyph 1 'A' (adv=720, lsb=40)
        let mut hmtx = Vec::new();
        hmtx.extend_from_slice(&500u16.to_be_bytes());
        hmtx.extend_from_slice(&0i16.to_be_bytes());
        hmtx.extend_from_slice(&720u16.to_be_bytes());
        hmtx.extend_from_slice(&40i16.to_be_bytes());
        tables.push((*b"hmtx", hmtx));

        // glyf: glyph 0 empty, glyph 1 is a filled triangle (50,0) -> (650,0) -> (350,700)
        let mut glyf = Vec::new();
        glyf.extend_from_slice(&1i16.to_be_bytes()); // 1 contour
        glyf.extend_from_slice(&50i16.to_be_bytes()); // xMin
        glyf.extend_from_slice(&0i16.to_be_bytes()); // yMin
        glyf.extend_from_slice(&650i16.to_be_bytes()); // xMax
        glyf.extend_from_slice(&700i16.to_be_bytes()); // yMax
        glyf.extend_from_slice(&2u16.to_be_bytes()); // endPtsOfContours[0] = 2 (3 points)
        glyf.extend_from_slice(&0u16.to_be_bytes()); // instructionLength
        glyf.extend_from_slice(&[0x01, 0x01, 0x01]); // all 3 on-curve, 16-bit signed deltas
        // dx deltas: +50, +600, -300
        glyf.extend_from_slice(&50i16.to_be_bytes());
        glyf.extend_from_slice(&600i16.to_be_bytes());
        glyf.extend_from_slice(&(-300i16).to_be_bytes());
        // dy deltas: 0, 0, +700
        glyf.extend_from_slice(&0i16.to_be_bytes());
        glyf.extend_from_slice(&0i16.to_be_bytes());
        glyf.extend_from_slice(&700i16.to_be_bytes());
        let glyf_len = glyf.len() as u32;
        tables.push((*b"glyf", glyf));

        // loca (long format, 3 entries for 2 glyphs): [0, 0, glyf_len]
        let mut loca = Vec::new();
        loca.extend_from_slice(&0u32.to_be_bytes());
        loca.extend_from_slice(&0u32.to_be_bytes());
        loca.extend_from_slice(&glyf_len.to_be_bytes());
        tables.push((*b"loca", loca));

        tables.sort_by_key(|(tag, _)| *tag);
        let num_tables = tables.len() as u16;
        let mut out = Vec::new();
        out.extend_from_slice(&0x0001_0000u32.to_be_bytes());
        out.extend_from_slice(&num_tables.to_be_bytes());
        out.extend_from_slice(&0u16.to_be_bytes());
        out.extend_from_slice(&0u16.to_be_bytes());
        out.extend_from_slice(&0u16.to_be_bytes());
        let dir_end = 12 + (tables.len() * 16);
        let mut cur_off = dir_end;
        for (tag, data) in &tables {
            out.extend_from_slice(tag);
            out.extend_from_slice(&0u32.to_be_bytes());
            out.extend_from_slice(&(cur_off as u32).to_be_bytes());
            out.extend_from_slice(&(data.len() as u32).to_be_bytes());
            cur_off += (data.len() + 3) & !3;
        }
        for (_, data) in &tables {
            out.extend_from_slice(data);
            while out.len() % 4 != 0 {
                out.push(0);
            }
        }
        out
    }

    #[test]
    fn truetype_font_file_parsing_tables_metrics_and_outline_rasterization() {
        unsafe {
            assert!(crate::lookup("dwrite.dll", "DWriteCreateFactory").is_some());

            let ttf_bytes = build_synthetic_ttf();
            let tmp_path = std::env::temp_dir().join(format!(
                "win32_rt_dwrite_synth_{}.ttf",
                std::process::id()
            ));
            fs::write(&tmp_path, &ttf_bytes).expect("write synthetic ttf");

            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DWriteCreateFactory(
                    DWRITE_FACTORY_TYPE_SHARED,
                    &IID_IDWRITE_FACTORY as *const GUID as *const u8,
                    &mut fac,
                ),
                S_OK
            );

            // CreateFontFileReference + Analyze
            let wpath = wide(&tmp_path.to_string_lossy());
            let mut ffile: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_font_file_reference(fac as PFactory, wpath.as_ptr(), std::ptr::null(), &mut ffile),
                S_OK
            );
            let mut supported = FALSE;
            let mut ftype = 0u32;
            let mut face_type = 0u32;
            let mut nfaces = 0u32;
            assert_eq!(
                ff_file_analyze(ffile as PFontFile, &mut supported, &mut ftype, &mut face_type, &mut nfaces),
                S_OK
            );
            assert_eq!(supported, TRUE);
            assert_eq!(ftype, DWRITE_FONT_FILE_TYPE_TRUETYPE);
            assert_eq!(face_type, DWRITE_FONT_FACE_TYPE_TRUETYPE);
            assert_eq!(nfaces, 1);

            // CreateFontFace from the synthetic TrueType file
            let mut face_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_font_face(
                    fac as PFactory,
                    DWRITE_FONT_FACE_TYPE_TRUETYPE,
                    1,
                    &ffile,
                    0,
                    DWRITE_FONT_SIMULATIONS_NONE,
                    &mut face_ptr,
                ),
                S_OK
            );

            // Verify parsed design metrics from OS/2 + head + post
            let mut fm = DWRITE_FONT_METRICS::default();
            face_get_metrics(face_ptr as PFontFace, &mut fm);
            assert_eq!(fm.designUnitsPerEm, 1000);
            assert_eq!(fm.ascent, 820);
            assert_eq!(fm.descent, 180);
            assert_eq!(fm.capHeight, 720);
            assert_eq!(fm.xHeight, 520);
            assert_eq!(face_get_glyph_count(face_ptr as PFontFace), 2);

            // Verify UTF-32 cmap mapping ('A' -> 1, unmapped 'Z' -> 0) and exact hmtx + glyf metrics
            let cps = [b'A' as u32, b'Z' as u32];
            let mut gids = [0u16; 2];
            assert_eq!(face_get_glyph_indices(face_ptr as PFontFace, cps.as_ptr(), 2, gids.as_mut_ptr()), S_OK);
            assert_eq!(gids, [1, 0]);

            let mut gm = DWRITE_GLYPH_METRICS::default();
            assert_eq!(face_get_design_glyph_metrics(face_ptr as PFontFace, &gids[0], 1, &mut gm, FALSE), S_OK);
            assert_eq!(gm.advanceWidth, 720);
            assert_eq!(gm.leftSideBearing, 40);
            assert_eq!(gm.rightSideBearing, 720 - 40 - 600); // bbox width = 650 - 50 = 600

            // TryGetFontTable("head") returns real bytes and ReleaseFontTable cleans up
            let tag_head = u32::from_le_bytes(*b"head");
            let mut tdata: *const c_void = std::ptr::null();
            let mut tsize = 0u32;
            let mut tctx: *mut c_void = std::ptr::null_mut();
            let mut texists = FALSE;
            assert_eq!(
                face_try_get_font_table(
                    face_ptr as PFontFace,
                    tag_head,
                    &mut tdata,
                    &mut tsize,
                    &mut tctx,
                    &mut texists,
                ),
                S_OK
            );
            assert_eq!(texists, TRUE);
            assert_eq!(tsize, 54);
            assert!(!tdata.is_null());
            face_release_font_table(face_ptr as PFontFace, tctx);

            // Rasterize real TrueType triangle glyph 'A' via scanline contour rasterizer
            let face_arc = font_face_from_ptr(face_ptr).unwrap();
            let glyph = rasterize_glyph(&face_arc, 1, 32.0);
            assert!(glyph.width >= 15 && glyph.height >= 15, "w={} h={}", glyph.width, glyph.height);
            let solid_px = glyph.alpha.iter().filter(|&&a| a >= 200).count();
            assert!(solid_px >= 60, "expected filled TrueType triangle pixels, got {solid_px}");

            release_com(face_ptr);
            release_com(ffile);
            release_com(fac);
            let _ = fs::remove_file(&tmp_path);
        }
    }

    #[test]
    fn text_layout_alignments_nowrap_min_width_and_clusters() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DWriteCreateFactory(
                    DWRITE_FACTORY_TYPE_SHARED,
                    &IID_IDWRITE_FACTORY as *const GUID as *const u8,
                    &mut fac,
                ),
                S_OK
            );
            let mut fmt: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_text_format(
                    fac as PFactory,
                    wide("Consolas").as_ptr(),
                    std::ptr::null_mut(),
                    DWRITE_FONT_WEIGHT_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    16.0,
                    wide("en-us").as_ptr(),
                    &mut fmt,
                ),
                S_OK
            );
            assert_eq!(tf_set_text_align(fmt as PTextFormat, DWRITE_TEXT_ALIGNMENT_CENTER), S_OK);
            assert_eq!(tf_set_para_align(fmt as PTextFormat, DWRITE_PARAGRAPH_ALIGNMENT_CENTER), S_OK);

            let text_u16: Vec<u16> = "Alpha Beta".encode_utf16().collect();
            let mut layout: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                fac_create_text_layout(
                    fac as PFactory,
                    text_u16.as_ptr(),
                    text_u16.len() as u32,
                    fmt,
                    400.0,
                    100.0,
                    &mut layout,
                ),
                S_OK
            );

            let mut tm = DWRITE_TEXT_METRICS::default();
            assert_eq!(tl_get_metrics(layout as PTextLayout, &mut tm), S_OK);
            assert_eq!(tm.lineCount, 1);
            // Centered horizontally inside 400 and vertically inside 100
            assert!(tm.left > 100.0 && tm.left < 200.0, "expected centered left, got {}", tm.left);
            assert!(tm.top > 20.0 && tm.top < 50.0, "expected centered top, got {}", tm.top);

            let mut min_w = 0.0f32;
            assert_eq!(tl_determine_min_width(layout as PTextLayout, &mut min_w), S_OK);
            assert!(min_w > 0.0 && min_w < tm.width);

            // Cluster metrics
            let mut cc = 0u32;
            assert_eq!(tl_get_cluster_metrics(layout as PTextLayout, std::ptr::null_mut(), 0, &mut cc), E_NOT_SUFFICIENT_BUFFER);
            assert_eq!(cc, text_u16.len() as u32);
            let mut cms = vec![DWRITE_CLUSTER_METRICS::default(); cc as usize];
            assert_eq!(tl_get_cluster_metrics(layout as PTextLayout, cms.as_mut_ptr(), cc, &mut cc), S_OK);
            assert_ne!(cms[5].flags & 2, 0, "cluster 5 (' ') should have isWhitespace flag set");

            // NO_WRAP keeps single line even when maxWidth is tiny
            assert_eq!(tl_set_word_wrap(layout as PTextLayout, DWRITE_WORD_WRAPPING_NO_WRAP), S_OK);
            assert_eq!(tl_set_max_width(layout as PTextLayout, 20.0), S_OK);
            assert_eq!(tl_get_metrics(layout as PTextLayout, &mut tm), S_OK);
            assert_eq!(tm.lineCount, 1);

            release_com(layout);
            release_com(fmt);
            release_com(fac);
        }
    }
}
