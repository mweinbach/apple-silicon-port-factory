//! DirectWrite ABI constants, HRESULTs, IIDs, and `#[repr(C)]` structures (`dwrite.h`).

use crate::com::{guid, Guid, IID_IUnknown};
use crate::types::*;
use std::ffi::c_void;

// ============================================================================
// DirectWrite Constants, Enums, GUIDs & HRESULTs (dwrite.h)
// ============================================================================

pub const DWRITE_E_FILEFORMAT: HRESULT = 0x8898_5000u32 as i32;
pub const DWRITE_E_NOFONT: HRESULT = 0x8898_5002u32 as i32;
pub const DWRITE_E_FILENOTFOUND: HRESULT = 0x8898_5003u32 as i32;
pub const DWRITE_E_FILEACCESS: HRESULT = 0x8898_5004u32 as i32;
pub const DWRITE_E_ALREADYREGISTERED: HRESULT = 0x8898_5006u32 as i32;
pub const E_NOT_SUFFICIENT_BUFFER: HRESULT = 0x8007_007Au32 as i32;

// Texture types (IDWriteGlyphRunAnalysis)
pub const DWRITE_TEXTURE_ALIASED_1X1: u32 = 0;
pub const DWRITE_TEXTURE_CLEARTYPE_3X1: u32 = 1;

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
pub(super) const DWRITE_INFO_COPYRIGHT_NOTICE: u32 = 1;
pub(super) const DWRITE_INFO_VERSION_STRINGS: u32 = 2;
pub(super) const DWRITE_INFO_TRADEMARK: u32 = 3;
pub(super) const DWRITE_INFO_MANUFACTURER: u32 = 4;
pub(super) const DWRITE_INFO_DESIGNER: u32 = 5;
pub(super) const DWRITE_INFO_DESCRIPTION: u32 = 8;
pub(super) const DWRITE_INFO_LICENSE_DESCRIPTION: u32 = 10;
pub(super) const DWRITE_INFO_FULL_NAME: u32 = 13;
pub(super) const DWRITE_INFO_POSTSCRIPT_NAME: u32 = 14;

pub(super) const G_IDWRITE_FACTORY: Guid = guid("b859ee5a-d838-4b5b-a2e8-1adc7d93db48");
pub(super) const G_IDWRITE_FACTORY1: Guid = guid("30572f99-dac6-41db-a16e-0486307e606a");
pub(super) const G_IDWRITE_FACTORY2: Guid = guid("0439fc60-ca44-4994-8dee-3a9af7b732ec");
pub(super) const G_IDWRITE_FACTORY3: Guid = guid("9a1b41c3-d3bb-466a-87fc-fe67556a3b65");
pub(super) const G_IDWRITE_FONT_COLLECTION: Guid = guid("a84cee02-3eea-4eee-a827-87c1a02a0fcc");
pub(super) const G_IDWRITE_FONT_LIST: Guid = guid("1a0d8438-1d97-4ec1-aef9-a2fb86ed6acb");
pub(super) const G_IDWRITE_FONT_FAMILY: Guid = guid("da20d8ef-812a-4c43-9802-62ec4abd7add");
pub(super) const G_IDWRITE_FONT: Guid = guid("acd16696-8c14-4f5d-877e-fe3fc1d32737");
pub(super) const G_IDWRITE_FONT_FACE: Guid = guid("5f49804d-7024-4d43-bfa9-d25984f53849");
pub(super) const G_IDWRITE_FONT_FILE: Guid = guid("739d886a-cef5-47dc-8769-1a8b41bebbb0");
pub(super) const G_IDWRITE_LOCALIZED_STRINGS: Guid = guid("08256209-099a-4b34-b86d-c22b110e7771");
pub(super) const G_IDWRITE_RENDERING_PARAMS: Guid = guid("2f0da53a-2add-47cd-82ee-d9ec34688e75");
pub(super) const G_IDWRITE_TEXT_FORMAT: Guid = guid("9c906818-31d7-4fd3-a151-7c5e225db55a");
pub(super) const G_IDWRITE_TEXT_LAYOUT: Guid = guid("53737037-6d14-410b-9bfe-0b182bb70961");
pub(super) const G_IDWRITE_TYPOGRAPHY: Guid = guid("55f1112b-1dc2-4b3c-9541-f46894ed85b6");
pub(super) const G_IDWRITE_GDI_INTEROP: Guid = guid("1edd9491-9853-4299-898f-6432983b6f3a");
pub(super) const G_IDWRITE_BITMAP_RENDER_TARGET: Guid = guid("5e5a32a3-8dff-4773-9ff6-0696eab77267");
pub(super) const G_IDWRITE_INLINE_OBJECT: Guid = guid("8339fde3-106f-47ab-8373-1c6295eb10b3");
pub(super) const G_IDWRITE_NUMBER_SUBSTITUTION: Guid = guid("14885cc9-bab0-4f90-b6ed-5c366a2cd03d");
pub(super) const G_IDWRITE_TEXT_ANALYZER: Guid = guid("b7e6163e-7f46-43b4-84b3-e4e6249c365d");
pub(super) const G_IDWRITE_GLYPH_RUN_ANALYSIS: Guid = guid("7d97dbf7-e085-42d4-81e3-6a883bdef118");

pub const IID_IUNKNOWN: GUID = IID_IUnknown.to_win_guid();
pub const IID_IDWRITE_FACTORY: GUID = G_IDWRITE_FACTORY.to_win_guid();
pub const IID_IDWRITE_FACTORY1: GUID = G_IDWRITE_FACTORY1.to_win_guid();
pub const IID_IDWRITE_FACTORY2: GUID = G_IDWRITE_FACTORY2.to_win_guid();
pub const IID_IDWRITE_FACTORY3: GUID = G_IDWRITE_FACTORY3.to_win_guid();
pub const IID_IDWRITE_FONT_COLLECTION: GUID = G_IDWRITE_FONT_COLLECTION.to_win_guid();
pub const IID_IDWRITE_FONT_LIST: GUID = G_IDWRITE_FONT_LIST.to_win_guid();
pub const IID_IDWRITE_FONT_FAMILY: GUID = G_IDWRITE_FONT_FAMILY.to_win_guid();
pub const IID_IDWRITE_FONT: GUID = G_IDWRITE_FONT.to_win_guid();
pub const IID_IDWRITE_FONT_FACE: GUID = G_IDWRITE_FONT_FACE.to_win_guid();
pub const IID_IDWRITE_FONT_FILE: GUID = G_IDWRITE_FONT_FILE.to_win_guid();
pub const IID_IDWRITE_LOCALIZED_STRINGS: GUID = G_IDWRITE_LOCALIZED_STRINGS.to_win_guid();
pub const IID_IDWRITE_RENDERING_PARAMS: GUID = G_IDWRITE_RENDERING_PARAMS.to_win_guid();
pub const IID_IDWRITE_TEXT_FORMAT: GUID = G_IDWRITE_TEXT_FORMAT.to_win_guid();
pub const IID_IDWRITE_TEXT_LAYOUT: GUID = G_IDWRITE_TEXT_LAYOUT.to_win_guid();
pub const IID_IDWRITE_TYPOGRAPHY: GUID = G_IDWRITE_TYPOGRAPHY.to_win_guid();
pub const IID_IDWRITE_GDI_INTEROP: GUID = G_IDWRITE_GDI_INTEROP.to_win_guid();
pub const IID_IDWRITE_BITMAP_RENDER_TARGET: GUID = G_IDWRITE_BITMAP_RENDER_TARGET.to_win_guid();
pub const IID_IDWRITE_INLINE_OBJECT: GUID = G_IDWRITE_INLINE_OBJECT.to_win_guid();
pub const IID_IDWRITE_NUMBER_SUBSTITUTION: GUID = G_IDWRITE_NUMBER_SUBSTITUTION.to_win_guid();
pub const IID_IDWRITE_TEXT_ANALYZER: GUID = G_IDWRITE_TEXT_ANALYZER.to_win_guid();
pub const IID_IDWRITE_GLYPH_RUN_ANALYSIS: GUID = G_IDWRITE_GLYPH_RUN_ANALYSIS.to_win_guid();

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_SCRIPT_ANALYSIS {
    pub script: u16,
    pub shapes: u32,
}
const _: () = assert!(std::mem::size_of::<DWRITE_SCRIPT_ANALYSIS>() == 8);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_LINE_BREAKPOINT {
    pub bits: u8,
}
const _: () = assert!(std::mem::size_of::<DWRITE_LINE_BREAKPOINT>() == 1);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_SHAPING_TEXT_PROPERTIES {
    pub bits: u16,
}
const _: () = assert!(std::mem::size_of::<DWRITE_SHAPING_TEXT_PROPERTIES>() == 2);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    pub bits: u16,
}
const _: () = assert!(std::mem::size_of::<DWRITE_SHAPING_GLYPH_PROPERTIES>() == 2);

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
    pub(super) fn from_u64(v: u64) -> Self {
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
    pub(super) fn from_u64(v: u64) -> Self {
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

/// Upper bound for NUL-terminated UTF-16 string reads in DirectWrite APIs.
pub(super) const DW_MAX_STR: usize = 65_536;

/// Write NUL-terminated UTF-16 string into `dst` (up to `cap` code units),
/// returning `E_POINTER` on null or `E_NOT_SUFFICIENT_BUFFER` without writing
/// if the buffer cannot hold the entire string plus NUL.
#[inline]
pub(super) unsafe fn write_wide_exact(s: &str, dst: *mut u16, cap: u32) -> HRESULT {
    if dst.is_null() {
        return E_POINTER;
    }
    let needed = s.encode_utf16().count() + 1;
    if (cap as usize) < needed {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    crate::strings::write_wide_z(dst, cap as usize, s);
    S_OK
}
