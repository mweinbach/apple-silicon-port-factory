//! DirectWrite font collections, families, fonts, font faces, font files,
//! localized strings, rendering params, typography, GDI interop, and inline objects.

use super::abi::*;
use super::sfnt::{
    default_fallback_metrics, fallback_char_advance_du, fallback_cp_to_gid, fallback_gid_to_char,
    flatten_quad_contour, load_cached_file_bytes, parse_font_bytes_metadata,
    parse_font_file_metadata, rasterize_glyph, CustomFontFileRef, FontMeta, SfntFace,
};
use crate::com::{
    com_add_ref, com_call_add_ref, com_from_iface, com_release, qi_helper_bytes,
    register_vtable_once, write_null_out, ComObj, ComPtr,
};
use crate::strings::read_wide_z;
use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

// ============================================================================
// System Font Collection & Built-in Default Fallback Families
// ============================================================================

#[derive(Clone, Debug)]
pub(super) struct FontFamilyData {
    pub(super) name: String,
    pub(super) fonts: Vec<Arc<FontMeta>>,
}

pub(super) struct FontCollectionData {
    pub(super) families: Vec<Arc<FontFamilyData>>,
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
                    raw_bytes: None,
                    custom_file: None,
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

    pub(super) fn from_metas(metas: Vec<Arc<FontMeta>>) -> Self {
        let mut by_family: HashMap<String, (String, Vec<Arc<FontMeta>>)> = HashMap::new();
        for meta in metas {
            let key = meta.family_name.to_ascii_lowercase();
            by_family
                .entry(key)
                .or_insert_with(|| (meta.family_name.clone(), Vec::new()))
                .1
                .push(meta);
        }
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

    pub(super) fn find_family_index(&self, name: &str) -> Option<usize> {
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

    pub(super) fn resolve_face(
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

pub(super) fn system_font_collection_data(refresh: bool) -> Arc<FontCollectionData> {
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

pub(super) struct FontInner {
    collection: Arc<FontCollectionData>,
    family: Arc<FontFamilyData>,
    meta: Arc<FontMeta>,
}

pub(crate) struct FontFaceInner {
    pub(super) meta: Arc<FontMeta>,
    pub(super) simulations: u32,
    pub(super) sfnt: Option<Arc<SfntFace>>,
}

impl FontFaceInner {
    pub(super) fn from_meta(meta: Arc<FontMeta>, simulations: u32) -> Arc<Self> {
        let sfnt = meta
            .raw_bytes
            .clone()
            .or_else(|| meta.file_path.as_ref().and_then(|p| load_cached_file_bytes(p)))
            .and_then(|bytes| SfntFace::from_bytes(bytes, meta.face_index).map(Arc::new));
        Arc::new(Self { meta, simulations, sfnt })
    }

    pub(super) fn codepoint_to_glyph(&self, cp: u32) -> u16 {
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

    pub(super) fn glyph_metrics(&self, gid: u16) -> DWRITE_GLYPH_METRICS {
        let m = &self.meta.metrics;
        let default_adv_h = (m.ascent as u32) + (m.descent as u32);
        let default_v_origin = m.ascent as i32;

        if let Some(ref sfnt) = self.sfnt {
            if let Some((adv_w, lsb)) = sfnt.hmtx(gid) {
                let bbox = sfnt.glyph_bbox(gid);
                let rsb = if let Some((xmin, _, xmax, _)) = bbox {
                    let bw = (xmax as i32) - (xmin as i32);
                    (adv_w as i32) - (lsb as i32) - bw
                } else {
                    0
                };
                let (adv_h, tsb, bsb, v_origin) = if let Some((vm_adv_h, vm_tsb)) = sfnt.vmtx(gid) {
                    let ah = vm_adv_h as u32;
                    let t = vm_tsb as i32;
                    if let Some((_, ymin, _, ymax)) = bbox {
                        let bh = (ymax as i32) - (ymin as i32);
                        let b = (ah as i32) - t - bh;
                        let vo = (ymax as i32) + t;
                        (ah, t, b, vo)
                    } else {
                        (ah, t, (m.descent as i32) / 2, default_v_origin)
                    }
                } else if let Some((_, ymin, _, ymax)) = bbox {
                    let t = (m.ascent as i32) - (ymax as i32);
                    let b = (ymin as i32) + (m.descent as i32);
                    (default_adv_h, t, b, default_v_origin)
                } else {
                    (default_adv_h, (m.ascent as i32) / 5, (m.descent as i32) / 2, default_v_origin)
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
            advanceHeight: default_adv_h,
            bottomSideBearing: m.descent as i32,
            verticalOriginY: default_v_origin,
        }
    }

    pub(super) fn scaled_advance(&self, gid: u16, em_size: f32) -> f32 {
        let gm = self.glyph_metrics(gid);
        let upem = self.meta.metrics.designUnitsPerEm.max(1) as f32;
        (gm.advanceWidth as f32) * em_size / upem
    }

    pub(super) fn scaled_vertical_advance(&self, gid: u16, em_size: f32) -> f32 {
        let gm = self.glyph_metrics(gid);
        let upem = self.meta.metrics.designUnitsPerEm.max(1) as f32;
        (gm.advanceHeight as f32) * em_size / upem
    }
}

pub(super) enum FontFileSource {
    LocalPath {
        path: PathBuf,
        path_wide: Vec<u16>,
    },
    CustomBytes {
        bytes: Arc<Vec<u8>>,
        loader: Option<ComPtr>,
        key: Vec<u8>,
    },
}

pub(super) struct FontFileInner {
    source: FontFileSource,
}

struct RenderingParamsInner {
    gamma: f32,
    enhanced_contrast: f32,
    clear_type_level: f32,
    pixel_geometry: u32,
    rendering_mode: u32,
}

pub(super) struct TypographyInner {
    pub(super) features: Mutex<Vec<DWRITE_FONT_FEATURE>>,
}

pub(super) struct GdiInteropInner {
    pub(super) collection: Arc<FontCollectionData>,
}

pub(super) struct BitmapRenderTargetInner {
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

pub(super) struct EllipsisInlineObjectInner {
    pub(super) font_size: f32,
    pub(super) ascent: f32,
    pub(super) height: f32,
}

pub(super) const DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE: u32 = 0;
pub(super) const DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL: u32 = 1;
pub(super) const DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE: u32 = 2;
pub(super) const DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL: u32 = 3;
pub(super) const DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL: u32 = 4;

#[derive(Clone, Debug)]
pub(super) struct NumberSubstitutionInner {
    pub(super) method: u32,
    pub(super) locale: String,
    pub(super) ignore_user_override: bool,
}

// ============================================================================
// IDWriteLocalizedStrings Vtable (9 slots)
// ============================================================================

type PLocStr = *mut ComObj<LocalizedStringsInner>;

unsafe extern "win64" fn locstr_qi(this: PLocStr, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_LOCALIZED_STRINGS])
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
    let query = read_wide_z(locale_name, DW_MAX_STR).to_ascii_lowercase();
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
    write_wide_exact(loc, buf, size)
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
    write_wide_exact(val, buf, size)
}

fn localized_strings_vtbl() -> *const usize {
    static V: OnceLock<[usize; 9]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            locstr_qi as *const () as usize,
            com_add_ref::<LocalizedStringsInner> as *const () as usize,
            com_release::<LocalizedStringsInner> as *const () as usize,
            locstr_get_count as *const () as usize,
            locstr_find_locale as *const () as usize,
            locstr_get_locale_len as *const () as usize,
            locstr_get_locale as *const () as usize,
            locstr_get_str_len as *const () as usize,
            locstr_get_str as *const () as usize,
        ]
    })
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

pub(super) type PFontFile = *mut ComObj<FontFileInner>;

unsafe extern "win64" fn ff_file_qi(this: PFontFile, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_FONT_FILE])
}
unsafe extern "win64" fn ff_file_get_ref_key(
    this: PFontFile,
    key_ptr: *mut *const c_void,
    key_size: *mut u32,
) -> HRESULT {
    if this.is_null() || key_ptr.is_null() || key_size.is_null() {
        return E_POINTER;
    }
    match &ComObj::inner(this).source {
        FontFileSource::LocalPath { path_wide, .. } => {
            key_ptr.write_unaligned(path_wide.as_ptr() as *const c_void);
            key_size.write_unaligned((path_wide.len() * 2) as u32);
        }
        FontFileSource::CustomBytes { key, .. } => {
            key_ptr.write_unaligned(key.as_ptr() as *const c_void);
            key_size.write_unaligned(key.len() as u32);
        }
    }
    S_OK
}
unsafe extern "win64" fn ff_file_get_loader(this: PFontFile, loader: *mut *mut c_void) -> HRESULT {
    write_null_out(loader);
    if this.is_null() || loader.is_null() {
        return E_POINTER;
    }
    if let FontFileSource::CustomBytes { loader: Some(ref lp), .. } = ComObj::inner(this).source {
        com_call_add_ref(lp.as_ptr());
        loader.write_unaligned(lp.as_ptr());
        return S_OK;
    }
    E_NOTIMPL
}
pub(super) unsafe extern "win64" fn ff_file_analyze(
    this: PFontFile,
    is_supported: *mut BOOL,
    file_type: *mut u32,
    face_type: *mut u32,
    num_faces: *mut u32,
) -> HRESULT {
    if this.is_null() || is_supported.is_null() || file_type.is_null() || num_faces.is_null() {
        return E_POINTER;
    }
    let metas = match &ComObj::inner(this).source {
        FontFileSource::LocalPath { path, .. } => parse_font_file_metadata(path),
        FontFileSource::CustomBytes { bytes, .. } => parse_font_bytes_metadata(Arc::clone(bytes), "CustomFont"),
    };
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
    register_vtable_once(&V, || {
        [
            ff_file_qi as *const () as usize,
            com_add_ref::<FontFileInner> as *const () as usize,
            com_release::<FontFileInner> as *const () as usize,
            ff_file_get_ref_key as *const () as usize,
            ff_file_get_loader as *const () as usize,
            ff_file_analyze as *const () as usize,
        ]
    })
}

pub(super) fn new_font_file(path: PathBuf) -> *mut c_void {
    let path_wide: Vec<u16> = path.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
    ComObj::new(font_file_vtbl(), FontFileInner { source: FontFileSource::LocalPath { path, path_wide } })
        as *mut c_void
}

pub(super) unsafe fn new_custom_font_file(
    bytes: Arc<Vec<u8>>,
    loader: *mut c_void,
    key: Vec<u8>,
) -> *mut c_void {
    let lp = ComPtr::retain(loader);
    ComObj::new(
        font_file_vtbl(),
        FontFileInner { source: FontFileSource::CustomBytes { bytes, loader: lp, key } },
    ) as *mut c_void
}

pub(super) unsafe fn font_file_metadata_from_ptr(ptr: *mut c_void) -> Option<Vec<FontMeta>> {
    let obj = com_from_iface::<FontFileInner>(ptr, font_file_vtbl())?;
    Some(match &obj.inner.source {
        FontFileSource::LocalPath { path, .. } => parse_font_file_metadata(path),
        FontFileSource::CustomBytes { bytes, loader, key } => {
            let custom_ref = Arc::new(CustomFontFileRef {
                loader: loader.as_ref().and_then(|lp| ComPtr::retain(lp.as_ptr())),
                key: key.clone(),
            });
            let mut metas = parse_font_bytes_metadata(Arc::clone(bytes), "CustomFont");
            for m in &mut metas {
                m.custom_file = Some(Arc::clone(&custom_ref));
            }
            metas
        }
    })
}

// ============================================================================
// IDWriteFontFace Vtable (18 slots)
// ============================================================================

pub(super) type PFontFace = *mut ComObj<Arc<FontFaceInner>>;

unsafe extern "win64" fn face_qi(this: PFontFace, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_FONT_FACE])
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
    let meta = &ComObj::inner(this).meta;
    let has_file = meta.file_path.is_some() || (meta.raw_bytes.is_some() && meta.custom_file.is_some());
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
    if let Some(ref p) = meta.file_path {
        files.write_unaligned(new_font_file(p.clone()));
    } else if let (Some(ref bytes), Some(ref cf)) = (&meta.raw_bytes, &meta.custom_file) {
        let loader_ptr = cf.loader.as_ref().map_or(std::ptr::null_mut(), |lp| lp.as_ptr());
        files.write_unaligned(new_custom_font_file(Arc::clone(bytes), loader_ptr, cf.key.clone()));
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
pub(super) unsafe extern "win64" fn face_get_metrics(this: PFontFace, out: *mut DWRITE_FONT_METRICS) {
    if this.is_null() || out.is_null() {
        return;
    }
    out.write_unaligned(ComObj::inner(this).meta.metrics);
}
pub(super) unsafe extern "win64" fn face_get_glyph_count(this: PFontFace) -> u16 {
    if this.is_null() {
        return 0;
    }
    ComObj::inner(this).meta.glyph_count
}
pub(super) unsafe extern "win64" fn face_get_design_glyph_metrics(
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
pub(super) unsafe extern "win64" fn face_get_glyph_indices(
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

pub(super) unsafe extern "win64" fn face_try_get_font_table(
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
    write_null_out(table_context);
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
pub(super) unsafe extern "win64" fn face_release_font_table(_this: PFontFace, table_context: *mut c_void) {
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
    register_vtable_once(&V, || {
        [
            face_qi as *const () as usize,
            com_add_ref::<Arc<FontFaceInner>> as *const () as usize,
            com_release::<Arc<FontFaceInner>> as *const () as usize,
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
        ]
    })
}

pub(super) fn new_font_face(face: Arc<FontFaceInner>) -> *mut c_void {
    ComObj::new(font_face_vtbl(), face) as *mut c_void
}

pub(crate) unsafe fn font_face_from_ptr(ptr: *mut c_void) -> Option<Arc<FontFaceInner>> {
    com_from_iface::<Arc<FontFaceInner>>(ptr, font_face_vtbl()).map(|obj| Arc::clone(&obj.inner))
}

// ============================================================================
// IDWriteFont Vtable (14 slots)
// ============================================================================

pub(super) type PFont = *mut ComObj<FontInner>;

unsafe extern "win64" fn font_qi(this: PFont, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_FONT])
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
    write_null_out(out);
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
pub(super) unsafe extern "win64" fn font_create_face(this: PFont, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let face = FontFaceInner::from_meta(Arc::clone(&ComObj::inner(this).meta), DWRITE_FONT_SIMULATIONS_NONE);
    out.write_unaligned(new_font_face(face));
    S_OK
}

fn font_vtbl() -> *const usize {
    static V: OnceLock<[usize; 14]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            font_qi as *const () as usize,
            com_add_ref::<FontInner> as *const () as usize,
            com_release::<FontInner> as *const () as usize,
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
        ]
    })
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
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_FONT_LIST])
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
    write_null_out(out);
    let inner = ComObj::inner(this);
    let Some(meta) = inner.fonts.get(idx as usize) else {
        return E_INVALIDARG;
    };
    let fam = inner
        .collection
        .find_family_index(&meta.family_name)
        .map(|i| Arc::clone(&inner.collection.families[i]))
        .unwrap_or_else(|| {
            Arc::new(FontFamilyData { name: meta.family_name.clone(), fonts: vec![Arc::clone(meta)] })
        });
    out.write_unaligned(new_font(Arc::clone(&inner.collection), fam, Arc::clone(meta)));
    S_OK
}

fn font_list_vtbl() -> *const usize {
    static V: OnceLock<[usize; 6]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            flist_qi as *const () as usize,
            com_add_ref::<FontListInner> as *const () as usize,
            com_release::<FontListInner> as *const () as usize,
            flist_get_collection as *const () as usize,
            flist_get_count as *const () as usize,
            flist_get_font as *const () as usize,
        ]
    })
}

type PFontFamily = *mut ComObj<FontFamilyInner>;

unsafe extern "win64" fn ffam_qi(this: PFontFamily, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_FONT_FAMILY, G_IDWRITE_FONT_LIST])
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
    write_null_out(out);
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
        write_null_out(out);
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
    register_vtable_once(&V, || {
        [
            ffam_qi as *const () as usize,
            com_add_ref::<FontFamilyInner> as *const () as usize,
            com_release::<FontFamilyInner> as *const () as usize,
            ffam_get_collection as *const () as usize,
            ffam_get_count as *const () as usize,
            ffam_get_font as *const () as usize,
            ffam_get_names as *const () as usize,
            ffam_get_first_matching as *const () as usize,
            ffam_get_matching_fonts as *const () as usize,
        ]
    })
}

fn new_font_family(collection: Arc<FontCollectionData>, family: Arc<FontFamilyData>) -> *mut c_void {
    ComObj::new(font_family_vtbl(), FontFamilyInner { collection, family }) as *mut c_void
}

// ============================================================================
// IDWriteFontCollection Vtable (7 slots)
// ============================================================================

type PFontCollection = *mut ComObj<FontCollectionInner>;

unsafe extern "win64" fn fcol_qi(this: PFontCollection, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_FONT_COLLECTION])
}
unsafe extern "win64" fn fcol_get_family_count(this: PFontCollection) -> u32 {
    if this.is_null() { return 0; }
    ComObj::inner(this).data.families.len() as u32
}
unsafe extern "win64" fn fcol_get_family(this: PFontCollection, idx: u32, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    write_null_out(out);
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
    let name = read_wide_z(family_name, DW_MAX_STR);
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
    write_null_out(out);
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
    register_vtable_once(&V, || {
        [
            fcol_qi as *const () as usize,
            com_add_ref::<FontCollectionInner> as *const () as usize,
            com_release::<FontCollectionInner> as *const () as usize,
            fcol_get_family_count as *const () as usize,
            fcol_get_family as *const () as usize,
            fcol_find_family_name as *const () as usize,
            fcol_get_font_from_face as *const () as usize,
        ]
    })
}

pub(super) fn new_font_collection(data: Arc<FontCollectionData>) -> *mut c_void {
    ComObj::new(font_collection_vtbl(), FontCollectionInner { data }) as *mut c_void
}

pub(super) unsafe fn font_collection_data_from_ptr(ptr: *mut c_void) -> Arc<FontCollectionData> {
    com_from_iface::<FontCollectionInner>(ptr, font_collection_vtbl())
        .map(|obj| Arc::clone(&obj.inner.data))
        .unwrap_or_else(|| system_font_collection_data(false))
}

// ============================================================================
// IDWriteRenderingParams Vtable (8 slots)
// ============================================================================

type PRenderParams = *mut ComObj<RenderingParamsInner>;

unsafe extern "win64" fn rp_qi(this: PRenderParams, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_RENDERING_PARAMS])
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
    register_vtable_once(&V, || {
        [
            rp_qi as *const () as usize,
            com_add_ref::<RenderingParamsInner> as *const () as usize,
            com_release::<RenderingParamsInner> as *const () as usize,
            rp_get_gamma as *const () as usize,
            rp_get_contrast as *const () as usize,
            rp_get_cleartype as *const () as usize,
            rp_get_pixel_geom as *const () as usize,
            rp_get_rendering_mode as *const () as usize,
        ]
    })
}

pub(super) fn new_rendering_params(gamma: f32, contrast: f32, ct: f32, pg: u32, rm: u32) -> *mut c_void {
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

pub(super) unsafe fn rendering_params_values_from_ptr(ptr: *mut c_void) -> Option<(f32, f32, f32)> {
    com_from_iface::<RenderingParamsInner>(ptr, rendering_params_vtbl())
        .map(|obj| (obj.inner.gamma, obj.inner.enhanced_contrast, obj.inner.clear_type_level))
}

// ============================================================================
// IDWriteTypography Vtable (6 slots)
// ============================================================================

pub(super) type PTypography = *mut ComObj<TypographyInner>;

unsafe extern "win64" fn typo_qi(this: PTypography, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_TYPOGRAPHY])
}
pub(super) unsafe extern "win64" fn typo_add_feature(this: PTypography, feature_packed: u64) -> HRESULT {
    if this.is_null() {
        return E_INVALIDARG;
    }
    ComObj::inner(this).features.lock().unwrap().push(DWRITE_FONT_FEATURE::from_u64(feature_packed));
    S_OK
}
pub(super) unsafe extern "win64" fn typo_get_count(this: PTypography) -> u32 {
    if this.is_null() { return 0; }
    ComObj::inner(this).features.lock().unwrap().len() as u32
}
pub(super) unsafe extern "win64" fn typo_get_feature(
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

pub(super) fn typography_vtbl() -> *const usize {
    static V: OnceLock<[usize; 6]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            typo_qi as *const () as usize,
            com_add_ref::<TypographyInner> as *const () as usize,
            com_release::<TypographyInner> as *const () as usize,
            typo_add_feature as *const () as usize,
            typo_get_count as *const () as usize,
            typo_get_feature as *const () as usize,
        ]
    })
}

// ============================================================================
// IDWriteBitmapRenderTarget (11 slots) & IDWriteGdiInterop (8 slots)
// ============================================================================

pub(super) type PBmpRT = *mut ComObj<BitmapRenderTargetInner>;

unsafe extern "win64" fn brt_qi(this: PBmpRT, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_BITMAP_RENDER_TARGET])
}
pub(super) unsafe extern "win64" fn brt_draw_glyph_run(
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

pub(super) unsafe extern "win64" fn brt_get_memory_dc(this: PBmpRT) -> usize {
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
    register_vtable_once(&V, || {
        [
            brt_qi as *const () as usize,
            com_add_ref::<BitmapRenderTargetInner> as *const () as usize,
            com_release::<BitmapRenderTargetInner> as *const () as usize,
            crate::bridge_entry!(brt_draw_glyph_run) as usize,
            brt_get_memory_dc as *const () as usize,
            brt_get_ppd as *const () as usize,
            crate::bridge_entry!(brt_set_ppd) as usize,
            brt_get_transform as *const () as usize,
            brt_set_transform as *const () as usize,
            brt_get_size as *const () as usize,
            brt_resize as *const () as usize,
        ]
    })
}

pub(super) type PGdiInterop = *mut ComObj<GdiInteropInner>;

unsafe extern "win64" fn gdi_qi(this: PGdiInterop, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_GDI_INTEROP])
}
pub(super) unsafe extern "win64" fn gdi_create_font_from_logfont(
    this: PGdiInterop,
    lf: *const LOGFONTW,
    out: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || lf.is_null() || out.is_null() {
        return E_POINTER;
    }
    write_null_out(out);
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

pub(super) unsafe extern "win64" fn gdi_convert_font_to_logfont(
    this: PGdiInterop,
    font: *mut c_void,
    lf: *mut LOGFONTW,
    is_system_font: *mut BOOL,
) -> HRESULT {
    if this.is_null() || font.is_null() || lf.is_null() {
        return E_POINTER;
    }
    let Some(fobj) = com_from_iface::<FontInner>(font, font_vtbl()) else {
        return E_INVALIDARG;
    };
    lf.write_unaligned(fill_logfont_from_meta(&fobj.inner.meta));
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
pub(super) unsafe extern "win64" fn gdi_create_bitmap_rt(
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

pub(super) fn gdi_interop_vtbl() -> *const usize {
    static V: OnceLock<[usize; 8]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            gdi_qi as *const () as usize,
            com_add_ref::<GdiInteropInner> as *const () as usize,
            com_release::<GdiInteropInner> as *const () as usize,
            gdi_create_font_from_logfont as *const () as usize,
            gdi_convert_font_to_logfont as *const () as usize,
            gdi_convert_face_to_logfont as *const () as usize,
            gdi_create_face_from_hdc as *const () as usize,
            gdi_create_bitmap_rt as *const () as usize,
        ]
    })
}

// ============================================================================
// IDWriteInlineObject (7 slots) & IDWriteNumberSubstitution (3 slots)
// ============================================================================

type PInlineObj = *mut ComObj<EllipsisInlineObjectInner>;

unsafe extern "win64" fn inline_qi(this: PInlineObj, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_INLINE_OBJECT])
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

pub(super) fn inline_obj_vtbl() -> *const usize {
    static V: OnceLock<[usize; 7]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            inline_qi as *const () as usize,
            com_add_ref::<EllipsisInlineObjectInner> as *const () as usize,
            com_release::<EllipsisInlineObjectInner> as *const () as usize,
            crate::bridge_entry!(inline_draw) as usize,
            inline_get_metrics as *const () as usize,
            inline_get_overhang as *const () as usize,
            inline_get_break as *const () as usize,
        ]
    })
}

type PNumSub = *mut ComObj<NumberSubstitutionInner>;

unsafe extern "win64" fn numsub_qi(this: PNumSub, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_NUMBER_SUBSTITUTION])
}

pub(super) fn num_sub_vtbl() -> *const usize {
    static V: OnceLock<[usize; 3]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            numsub_qi as *const () as usize,
            com_add_ref::<NumberSubstitutionInner> as *const () as usize,
            com_release::<NumberSubstitutionInner> as *const () as usize,
        ]
    })
}

pub(super) unsafe fn number_substitution_from_ptr(p: *mut c_void) -> Option<NumberSubstitutionInner> {
    if p.is_null() {
        return None;
    }
    let obj = com_from_iface::<NumberSubstitutionInner>(p, num_sub_vtbl())?;
    Some(obj.inner.clone())
}
