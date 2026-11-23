//! dwrite.dll: Native DirectWrite typography, TrueType/OpenType font parsing,
//! text layout, hit-testing, and glyph outline/bitmap rasterization.

mod abi;
mod font;
mod layout;
mod sfnt;

pub use abi::*;
pub(crate) use font::{font_face_from_ptr, FontFaceInner};
pub(crate) use layout::{extract_layout_runs, layout_text_from_format, DrawableGlyphRun};
pub(crate) use sfnt::rasterize_glyph;

use crate::com::{
    com_add_ref, com_release, qi_helper_bytes, read_guid_from_bytes, register_vtable_once,
    write_null_out, ComObj, IID_IUnknown,
};
use crate::strings::{read_wide_slice, read_wide_z};
use crate::types::*;
use font::{
    font_collection_data_from_ptr, font_file_path_from_ptr, gdi_interop_vtbl, inline_obj_vtbl,
    new_font_collection, new_font_face, new_font_file, new_rendering_params, num_sub_vtbl,
    system_font_collection_data, typography_vtbl, EllipsisInlineObjectInner, GdiInteropInner,
    NumberSubstitutionInner, TypographyInner,
};
use layout::{
    line_metrics_for_face, text_format_state_from_ptr, text_format_vtbl, text_layout_vtbl,
    TextFormatInner, TextFormatState, TextLayoutInner, TextLayoutState,
};
use sfnt::parse_font_file_metadata;
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// IDWriteFactory Vtable (24 slots)
// ============================================================================

struct FactoryInner {
    _factory_type: u32,
}

type PFactory = *mut ComObj<FactoryInner>;

unsafe extern "win64" fn fac_qi(this: PFactory, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(
        this,
        riid,
        ppv,
        &[G_IDWRITE_FACTORY, G_IDWRITE_FACTORY1, G_IDWRITE_FACTORY2, G_IDWRITE_FACTORY3],
    )
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
    write_null_out(out);
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
    write_null_out(out);
    let win_path = read_wide_z(file_path, DW_MAX_STR);
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
    write_null_out(out);
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
    write_null_out(out);
    let first = font_files.read_unaligned();
    let Some(path) = font_file_path_from_ptr(first) else {
        return E_INVALIDARG;
    };
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
    write_null_out(out);
    if !font_size.is_finite() || font_size <= 0.0 {
        return E_INVALIDARG;
    }
    let fam = read_wide_z(family_name, DW_MAX_STR);
    let loc = if locale_name.is_null() { "en-us".to_string() } else { read_wide_z(locale_name, DW_MAX_STR) };
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
    write_null_out(out);
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
    write_null_out(out);
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
    write_null_out(out);
    crate::stub!("IDWriteFactory::CreateGlyphRunAnalysis -> E_NOTIMPL");
    E_NOTIMPL
}
crate::win64_bridge!(fn fac_create_glyph_run_analysis(
    PFactory, *const DWRITE_GLYPH_RUN, f32, *const DWRITE_MATRIX, u32, u32, f32, f32, *mut *mut c_void
) -> HRESULT);

fn factory_vtbl() -> *const usize {
    static V: OnceLock<[usize; 24]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            fac_qi as *const () as usize,                                         // 0
            com_add_ref::<FactoryInner> as *const () as usize,                    // 1
            com_release::<FactoryInner> as *const () as usize,                    // 2
            fac_get_system_font_collection as *const () as usize,                 // 3
            fac_create_custom_font_collection as *const () as usize,              // 4
            fac_register_font_collection_loader as *const () as usize,            // 5
            fac_unregister_font_collection_loader as *const () as usize,          // 6
            fac_create_font_file_reference as *const () as usize,                 // 7
            fac_create_custom_font_file_reference as *const () as usize,          // 8
            fac_create_font_face as *const () as usize,                           // 9
            fac_create_rendering_params as *const () as usize,                    // 10
            fac_create_monitor_rendering_params as *const () as usize,            // 11
            crate::bridge_entry!(fac_create_custom_rendering_params) as usize,    // 12
            fac_register_font_file_loader as *const () as usize,                  // 13
            fac_unregister_font_file_loader as *const () as usize,                // 14
            crate::bridge_entry!(fac_create_text_format) as usize,                // 15
            fac_create_typography as *const () as usize,                          // 16
            fac_get_gdi_interop as *const () as usize,                            // 17
            crate::bridge_entry!(fac_create_text_layout) as usize,                // 18
            crate::bridge_entry!(fac_create_gdi_compatible_text_layout) as usize, // 19
            fac_create_ellipsis_trimming_sign as *const () as usize,              // 20
            fac_create_text_analyzer as *const () as usize,                       // 21
            fac_create_number_substitution as *const () as usize,                 // 22
            crate::bridge_entry!(fac_create_glyph_run_analysis) as usize,         // 23
        ]
    })
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
    write_null_out(factory);
    if factory_type != DWRITE_FACTORY_TYPE_SHARED && factory_type != DWRITE_FACTORY_TYPE_ISOLATED {
        return E_INVALIDARG;
    }
    let Some(req) = read_guid_from_bytes(iid) else {
        return E_INVALIDARG;
    };
    if req != IID_IUnknown
        && req != G_IDWRITE_FACTORY
        && req != G_IDWRITE_FACTORY1
        && req != G_IDWRITE_FACTORY2
        && req != G_IDWRITE_FACTORY3
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
mod tests;
