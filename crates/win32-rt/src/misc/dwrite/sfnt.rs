//! Pure-Rust TrueType / OpenType SFNT & TTC parser, built-in 8x8 fallback font,
//! and scanline glyph outline rasterizer.

use super::abi::*;
use super::font::FontFaceInner;
use crate::strings::{be_u16, be_u32};
use std::collections::HashMap;
use std::fs::{self, File};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

#[inline]
fn be_i16(b: &[u8], off: usize) -> Option<i16> {
    be_u16(b, off).map(|v| v as i16)
}

fn read_at_vec(file: &File, offset: u64, len: usize) -> Option<Vec<u8>> {
    if len == 0 || len > 16 * 1024 * 1024 {
        return None;
    }
    let mut buf = vec![0u8; len];
    file.read_exact_at(&mut buf, offset).ok()?;
    Some(buf)
}

pub(super) fn default_fallback_metrics() -> DWRITE_FONT_METRICS {
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

/// Provenance for an in-memory custom `IDWriteFontFile` created via
/// `CreateCustomFontFileReference`, retained on `FontMeta` so `IDWriteFontFace::GetFiles`
/// can return a real backing `IDWriteFontFile` carrying the caller's loader and key.
pub(super) struct CustomFontFileRef {
    pub(super) loader: Option<crate::com::ComPtr>,
    pub(super) key: Vec<u8>,
}

impl std::fmt::Debug for CustomFontFileRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomFontFileRef")
            .field("has_loader", &self.loader.is_some())
            .field("key_len", &self.key.len())
            .finish()
    }
}

#[derive(Clone, Debug)]
pub(super) struct FontMeta {
    pub(super) family_name: String,
    pub(super) face_name: String,
    pub(super) weight: u32,
    pub(super) stretch: u32,
    pub(super) style: u32,
    pub(super) is_symbol: bool,
    pub(super) is_monospace: bool,
    pub(super) face_type: u32,
    pub(super) face_index: u32,
    pub(super) glyph_count: u16,
    pub(super) metrics: DWRITE_FONT_METRICS,
    pub(super) info_strings: HashMap<u32, String>,
    pub(super) file_path: Option<PathBuf>,
    pub(super) raw_bytes: Option<Arc<Vec<u8>>>,
    pub(super) custom_file: Option<Arc<CustomFontFileRef>>,
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
    if lower_fam.contains("mono")
        || lower_fam.contains("courier")
        || lower_fam.contains("menlo")
        || lower_fam.contains("consolas")
    {
        is_monospace = true;
    }
    let is_symbol =
        lower_fam.contains("symbol") || lower_fam.contains("wingding") || lower_fam.contains("webding");

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
        raw_bytes: None,
        custom_file: None,
    })
}

fn parse_sfnt_face_meta_slice(
    raw: &Arc<Vec<u8>>,
    sfnt_offset: usize,
    face_index: u32,
    is_ttc: bool,
    default_stem: &str,
) -> Option<FontMeta> {
    let b = raw.as_slice();
    let hdr = b.get(sfnt_offset..sfnt_offset.checked_add(12)?)?;
    let sfnt_ver = be_u32(hdr, 0)?;
    let face_type = match sfnt_ver {
        0x0001_0000 | 0x7472_7565 => {
            if is_ttc {
                DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION
            } else {
                DWRITE_FONT_FACE_TYPE_TRUETYPE
            }
        }
        0x4F54_544F => DWRITE_FONT_FACE_TYPE_CFF,
        _ => return None,
    };
    let num_tables = be_u16(hdr, 4)? as usize;
    if num_tables == 0 || num_tables > 256 {
        return None;
    }
    let dir = b.get(sfnt_offset + 12..sfnt_offset + 12 + num_tables * 16)?;
    let mut tables: HashMap<[u8; 4], (usize, usize)> = HashMap::with_capacity(num_tables);
    for i in 0..num_tables {
        let base = i * 16;
        let tag = [dir[base], dir[base + 1], dir[base + 2], dir[base + 3]];
        let off = be_u32(dir, base + 8)? as usize;
        let len = be_u32(dir, base + 12)? as usize;
        if off.saturating_add(len) <= b.len() {
            tables.insert(tag, (off, len));
        }
    }
    let load_tbl = |tag: &[u8; 4]| -> Option<&[u8]> {
        let &(off, len) = tables.get(tag)?;
        b.get(off..off + len.min(256 * 1024))
    };

    let head = load_tbl(b"head")?;
    let upem = be_u16(head, 18).unwrap_or(2048).max(16);
    let mac_style = be_u16(head, 44).unwrap_or(0);
    let hhea = load_tbl(b"hhea");
    let maxp = load_tbl(b"maxp");
    let os2 = load_tbl(b"OS/2");
    let post = load_tbl(b"post");
    let name_tbl = load_tbl(b"name");

    let glyph_count = maxp.and_then(|t| be_u16(t, 4)).unwrap_or(256).max(1);
    let hhea_asc = hhea.and_then(|t| be_i16(t, 4)).unwrap_or(((upem as i32 * 80) / 100) as i16);
    let hhea_desc = hhea.and_then(|t| be_i16(t, 6)).unwrap_or(-((upem as i32 * 20) / 100) as i16);
    let hhea_gap = hhea.and_then(|t| be_i16(t, 8)).unwrap_or(0);

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

    if let Some(t) = os2 {
        let ver = be_u16(t, 0).unwrap_or(0);
        if let Some(w) = be_u16(t, 4) {
            if (100..=999).contains(&w) { weight = w as u32; }
        }
        if let Some(st) = be_u16(t, 6) {
            if (1..=9).contains(&st) { stretch = st as u32; }
        }
        if let Some(ss) = be_i16(t, 26) { if ss > 0 { strike_thick = ss as u16; } }
        if let Some(sp) = be_i16(t, 28) { strike_pos = sp; }
        let fs_sel = be_u16(t, 62).unwrap_or(0);
        if (fs_sel & 1) != 0 {
            style = DWRITE_FONT_STYLE_ITALIC;
        } else if (fs_sel & (1 << 9)) != 0 {
            style = DWRITE_FONT_STYLE_OBLIQUE;
        }
        if (fs_sel & (1 << 5)) != 0 && weight < 600 {
            weight = DWRITE_FONT_WEIGHT_BOLD;
        }
        if (fs_sel & (1 << 7)) != 0 {
            if let Some(ta) = be_i16(t, 68) { ascent = ta.max(0) as u16; }
            if let Some(td) = be_i16(t, 70) { descent = td.unsigned_abs(); }
            if let Some(tg) = be_i16(t, 72) { line_gap = tg; }
        } else {
            if let Some(wa) = be_u16(t, 74) { if wa > 0 { ascent = wa; } }
            if let Some(wd) = be_u16(t, 76) { if wd > 0 { descent = wd; } }
        }
        if ver >= 2 {
            if let Some(xh) = be_i16(t, 86) { if xh > 0 { x_height = xh as u16; } }
            if let Some(ch) = be_i16(t, 88) { if ch > 0 { cap_height = ch as u16; } }
        }
    }

    let mut underline_pos = -((upem as i32 * 10) / 100) as i16;
    let mut underline_thick = ((upem as u32 * 5) / 100).max(1) as u16;
    let mut is_monospace = false;
    if let Some(t) = post {
        if let Some(up) = be_i16(t, 8) { underline_pos = up; }
        if let Some(ut) = be_i16(t, 10) { if ut > 0 { underline_thick = ut as u16; } }
        if let Some(fp) = be_u32(t, 12) { is_monospace = fp != 0; }
    }

    let names = name_tbl.map(parse_name_table).unwrap_or_default();
    let family_name = names
        .get(&16)
        .or_else(|| names.get(&1))
        .cloned()
        .unwrap_or_else(|| default_stem.to_string());
    let face_name = names
        .get(&17)
        .or_else(|| names.get(&2))
        .cloned()
        .unwrap_or_else(|| "Regular".to_string());
    let lower_fam = family_name.to_ascii_lowercase();
    if lower_fam.contains("mono")
        || lower_fam.contains("courier")
        || lower_fam.contains("menlo")
        || lower_fam.contains("consolas")
    {
        is_monospace = true;
    }
    let is_symbol = lower_fam.contains("symbol")
        || lower_fam.contains("wingding")
        || lower_fam.contains("webding");

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
        file_path: None,
        raw_bytes: Some(Arc::clone(raw)),
        custom_file: None,
    })
}

pub(super) fn parse_font_bytes_metadata(raw: Arc<Vec<u8>>, default_stem: &str) -> Vec<FontMeta> {
    let b = raw.as_slice();
    let Some(tag) = be_u32(b, 0) else { return Vec::new() };
    if tag == 0x7474_6366 {
        let num_fonts = be_u32(b, 8).unwrap_or(0).min(32) as usize;
        let mut out = Vec::with_capacity(num_fonts);
        for i in 0..num_fonts {
            if let Some(off) = be_u32(b, 12 + i * 4) {
                if let Some(meta) = parse_sfnt_face_meta_slice(&raw, off as usize, i as u32, true, default_stem) {
                    out.push(meta);
                }
            }
        }
        out
    } else {
        parse_sfnt_face_meta_slice(&raw, 0, 0, false, default_stem).into_iter().collect()
    }
}

pub(super) fn parse_font_file_metadata(path: &Path) -> Vec<FontMeta> {
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
pub(super) struct SfntFace {
    pub(super) raw: Arc<Vec<u8>>,
    tables: HashMap<[u8; 4], (usize, usize)>,
    upem: u16,
    index_to_loc_format: i16,
    num_hmetrics: u16,
    num_vmetrics: u16,
    cmap_subtable: Option<(u16, usize)>, // (format, byte offset into raw)
    /// Reverse map from glyph ID to Unicode codepoint for CFF fallback rendering.
    glyph_to_char: HashMap<u16, char>,
}

impl SfntFace {
    pub(super) fn from_bytes(raw: Arc<Vec<u8>>, face_index: u32) -> Option<Self> {
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
        let num_vmetrics = tables
            .get(b"vhea")
            .and_then(|&(o, l)| b.get(o..o + l))
            .and_then(|t| be_u16(t, 34))
            .unwrap_or(0);

        let cmap_subtable = Self::pick_cmap(b, &tables);
        let mut face = Self {
            raw,
            tables,
            upem,
            index_to_loc_format,
            num_hmetrics,
            num_vmetrics,
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

    pub(super) fn codepoint_to_glyph(&self, cp: u32) -> u16 {
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

    pub(super) fn hmtx(&self, gid: u16) -> Option<(u16, i16)> {
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

    pub(super) fn vmtx(&self, gid: u16) -> Option<(u16, i16)> {
        if self.num_vmetrics == 0 {
            return None;
        }
        let &(off, len) = self.tables.get(b"vmtx")?;
        let tbl = self.raw.get(off..off + len)?;
        let nv = self.num_vmetrics as usize;
        let g = gid as usize;
        if g < nv {
            let adv = be_u16(tbl, g * 4)?;
            let tsb = be_i16(tbl, g * 4 + 2)?;
            Some((adv, tsb))
        } else {
            let adv = be_u16(tbl, nv.saturating_sub(1) * 4)?;
            let tsb_off = nv * 4 + (g - nv) * 2;
            let tsb = be_i16(tbl, tsb_off).unwrap_or(0);
            Some((adv, tsb))
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

    pub(super) fn glyph_bbox(&self, gid: u16) -> Option<(i16, i16, i16, i16)> {
        let g = self.glyf_slice(gid)?;
        if g.len() < 10 {
            return None;
        }
        Some((be_i16(g, 2)?, be_i16(g, 4)?, be_i16(g, 6)?, be_i16(g, 8)?))
    }

    pub(super) fn glyph_contours(&self, gid: u16, depth: u32) -> Option<Vec<Vec<(f32, f32, bool)>>> {
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

    pub(super) fn try_get_table(&self, open_type_tag: u32) -> Option<(usize, usize)> {
        let tag = open_type_tag.to_le_bytes();
        self.tables.get(&tag).copied()
    }
}

pub(super) fn load_cached_file_bytes(path: &Path) -> Option<Arc<Vec<u8>>> {
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

pub(super) fn fallback_char_advance_du(ch: char, monospace: bool, weight: u32) -> u32 {
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

pub(super) fn fallback_cp_to_gid(cp: u32) -> u16 {
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

pub(super) fn fallback_gid_to_char(gid: u16) -> Option<char> {
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

pub(super) fn flatten_quad_contour(
    contour: &[(f32, f32, bool)],
    scale: f32,
    italic_shear: f32,
) -> Vec<(f32, f32)> {
    flatten_quad_contour_transformed(contour, scale, italic_shear, (1.0, 0.0, 0.0, 1.0), (0.0, 0.0))
}

fn flatten_quad_contour_transformed(
    contour: &[(f32, f32, bool)],
    scale: f32,
    italic_shear: f32,
    mat: (f32, f32, f32, f32),
    subpixel_offset: (f32, f32),
) -> Vec<(f32, f32)> {
    let n = contour.len();
    if n < 2 {
        return Vec::new();
    }
    let tx = |x: f32, y: f32| -> (f32, f32) {
        let dx = (x + y * italic_shear) * scale;
        let dy = -y * scale;
        (
            dx * mat.0 + dy * mat.2 + subpixel_offset.0,
            dx * mat.1 + dy * mat.3 + subpixel_offset.1,
        )
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

fn rasterize_contours(
    contours: &[Vec<(f32, f32, bool)>],
    scale: f32,
    italic_shear: f32,
    mat: (f32, f32, f32, f32),
    subpixel_offset: (f32, f32),
) -> RasterizedGlyph {
    let mut polys: Vec<Vec<(f32, f32)>> = Vec::with_capacity(contours.len());
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for c in contours {
        let poly = flatten_quad_contour_transformed(c, scale, italic_shear, mat, subpixel_offset);
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

fn rasterize_fallback_8x8_transformed(
    ch: Option<char>,
    em_size: f32,
    bold: bool,
    italic: bool,
    mat: (f32, f32, f32, f32),
    subpixel_offset: (f32, f32),
) -> RasterizedGlyph {
    let is_identity = (mat.0 - 1.0).abs() < 1e-5
        && mat.1.abs() < 1e-5
        && mat.2.abs() < 1e-5
        && (mat.3 - 1.0).abs() < 1e-5
        && subpixel_offset.0.abs() < 1e-5
        && subpixel_offset.1.abs() < 1e-5;
    if is_identity {
        return rasterize_fallback_8x8(ch, em_size, bold, italic);
    }
    let det = mat.0 * mat.3 - mat.1 * mat.2;
    if !det.is_finite() || det.abs() < 1e-6 {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }
    if ch == Some(' ') || ch == Some('\t') || ch == Some('\r') || ch == Some('\n') {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }
    let bitmap = match ch {
        Some(c) if (0x20..=0x7E).contains(&(c as u32)) => &FALLBACK_FONT_8X8[(c as u32 - 0x20) as usize],
        _ => &FALLBACK_NOTDEF_8X8,
    };
    let gw = (em_size * 0.62).max(1.0);
    let gh = (em_size * 0.88).max(1.0);
    let shear_dip = if italic { em_size * 0.16 } else { 0.0 };
    let ascent_dip = em_size * 0.78;
    let local_w = gw + shear_dip;
    let corners = [
        (0.0f32, -ascent_dip),
        (local_w, -ascent_dip),
        (0.0, gh - ascent_dip),
        (local_w, gh - ascent_dip),
    ];
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for &(lx, ly) in &corners {
        let px = lx * mat.0 + ly * mat.2 + subpixel_offset.0;
        let py = lx * mat.1 + ly * mat.3 + subpixel_offset.1;
        min_x = min_x.min(px);
        min_y = min_y.min(py);
        max_x = max_x.max(px);
        max_y = max_y.max(py);
    }
    let ix0 = min_x.floor() as i32 - 1;
    let iy0 = min_y.floor() as i32 - 1;
    let ix1 = max_x.ceil() as i32 + 1;
    let iy1 = max_y.ceil() as i32 + 1;
    let out_w = (ix1 - ix0).clamp(0, 512) as u32;
    let out_h = (iy1 - iy0).clamp(0, 512) as u32;
    if out_w == 0 || out_h == 0 {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }
    // Inverse of row-vector transform (lx*m11 + ly*m21 + sub_x, lx*m12 + ly*m22 + sub_y):
    // lx = ((px - sub_x)*m22 - (py - sub_y)*m21) / det,
    // ly = (-(px - sub_x)*m12 + (py - sub_y)*m11) / det.
    let inv_det = 1.0 / det;
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
    let mut alpha = vec![0u8; (out_w * out_h) as usize];
    for py in 0..out_h {
        for px in 0..out_w {
            let mut acc = 0.0f32;
            for sy in 0..4 {
                let oy = (iy0 + py as i32) as f32 + (sy as f32 + 0.5) * 0.25 - subpixel_offset.1;
                for sx in 0..4 {
                    let ox = (ix0 + px as i32) as f32 + (sx as f32 + 0.5) * 0.25 - subpixel_offset.0;
                    let lx = (ox * mat.3 - oy * mat.2) * inv_det;
                    let ly = (-ox * mat.1 + oy * mat.0) * inv_det;
                    let ly_box = ly + ascent_dip;
                    let v = ly_box / gh;
                    let row_shear = if italic { (1.0 - v) * shear_dip } else { 0.0 };
                    let u = (lx - row_shear) / gw;
                    acc += sample_bit(u, v);
                }
            }
            alpha[(py * out_w + px) as usize] = ((acc * 0.0625).clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
        }
    }
    RasterizedGlyph { offset_x: ix0, offset_y: iy0, width: out_w, height: out_h, alpha }
}

pub(crate) fn rasterize_glyph(face: &FontFaceInner, glyph_id: u16, em_size: f32) -> RasterizedGlyph {
    rasterize_glyph_transformed(face, glyph_id, em_size, 1.0, &DWRITE_MATRIX::default(), (0.0, 0.0))
}

pub(crate) fn rasterize_glyph_transformed(
    face: &FontFaceInner,
    glyph_id: u16,
    em_size: f32,
    pixels_per_dip: f32,
    m: &DWRITE_MATRIX,
    subpixel_offset: (f32, f32),
) -> RasterizedGlyph {
    if em_size <= 0.0 || !em_size.is_finite() || pixels_per_dip <= 0.0 || !pixels_per_dip.is_finite() {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }
    let mat = (
        m.m11 * pixels_per_dip,
        m.m12 * pixels_per_dip,
        m.m21 * pixels_per_dip,
        m.m22 * pixels_per_dip,
    );
    let max_scale = mat.0.hypot(mat.1).max(mat.2.hypot(mat.3));
    if !max_scale.is_finite() || em_size * max_scale <= 0.5 {
        return RasterizedGlyph { offset_x: 0, offset_y: 0, width: 0, height: 0, alpha: Vec::new() };
    }
    let italic_shear = if (face.simulations & DWRITE_FONT_SIMULATIONS_OBLIQUE) != 0 { 0.21 } else { 0.0 };
    if let Some(ref sfnt) = face.sfnt {
        if let Some(contours) = sfnt.glyph_contours(glyph_id, 0) {
            if !contours.is_empty() {
                let scale = em_size / (sfnt.upem as f32);
                return rasterize_contours(&contours, scale, italic_shear, mat, subpixel_offset);
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
        return rasterize_fallback_8x8_transformed(ch, em_size, bold, italic, mat, subpixel_offset);
    }

    let ch = fallback_gid_to_char(glyph_id);
    let bold = face.meta.weight >= 650 || (face.simulations & DWRITE_FONT_SIMULATIONS_BOLD) != 0;
    let italic = face.meta.style != DWRITE_FONT_STYLE_NORMAL
        || (face.simulations & DWRITE_FONT_SIMULATIONS_OBLIQUE) != 0;
    rasterize_fallback_8x8_transformed(ch, em_size, bold, italic, mat, subpixel_offset)
}
