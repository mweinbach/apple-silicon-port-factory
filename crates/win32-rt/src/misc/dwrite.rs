//! dwrite.dll: Native DirectWrite typography, TrueType/OpenType font parsing,
//! text layout, hit-testing, and glyph outline/bitmap rasterization.

mod abi;
mod analysis;
mod font;
mod layout;
mod sfnt;
mod shaping;

pub use abi::*;
pub(crate) use font::{font_face_from_ptr, FontFaceInner};
pub(crate) use layout::{extract_layout_runs, layout_text_from_format, DrawableGlyphRun};
pub(crate) use sfnt::rasterize_glyph;

use crate::com::{
    com_add_ref, com_call_release, com_release, qi_helper_bytes, read_guid_from_bytes,
    register_vtable_once, write_null_out, ComObj, IID_IUnknown,
};
use crate::strings::{read_wide_slice, read_wide_z};
use crate::types::*;
use analysis::{new_glyph_run_analysis, new_text_analyzer, GlyphRunAnalysisInner};
use font::{
    font_collection_data_from_ptr, font_file_metadata_from_ptr, gdi_interop_vtbl, inline_obj_vtbl,
    new_custom_font_file, new_font_collection, new_font_face, new_font_file, new_rendering_params,
    num_sub_vtbl, system_font_collection_data, typography_vtbl, EllipsisInlineObjectInner,
    FontCollectionData, GdiInteropInner, NumberSubstitutionInner, TypographyInner,
};
use layout::{
    line_metrics_for_face, text_format_state_from_ptr, text_format_vtbl, text_layout_vtbl,
    TextFormatInner, TextFormatState, TextLayoutInner, TextLayoutState,
};
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
    font_file_loaders: Mutex<Vec<usize>>,
    font_collection_loaders: Mutex<Vec<usize>>,
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
    this: PFactory,
    loader: *mut c_void,
    key: *const c_void,
    key_size: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    write_null_out(out);
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    if loader.is_null() || (key_size > 0 && key.is_null()) {
        return E_INVALIDARG;
    }
    let vtbl = (loader as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return E_INVALIDARG;
    }
    // IDWriteFontCollectionLoader::CreateEnumeratorFromKey (slot 3)
    let create_enum = vtbl.add(3).read_unaligned();
    if create_enum == 0 {
        return E_INVALIDARG;
    }
    let mut enumerator: *mut c_void = std::ptr::null_mut();
    let hr = crate::call_guest8(
        create_enum,
        loader as u64,
        this as u64,
        key as u64,
        key_size as u64,
        &mut enumerator as *mut *mut c_void as u64,
        0,
        0,
        0,
    ) as HRESULT;
    if hr < 0 {
        return hr;
    }
    if enumerator.is_null() {
        return E_FAIL;
    }
    let evtbl = (enumerator as *const *const usize).read_unaligned();
    if evtbl.is_null() {
        com_call_release(enumerator);
        return E_FAIL;
    }
    // IDWriteFontFileEnumerator: slot 3 = MoveNext, slot 4 = GetCurrentFontFile
    let move_next = evtbl.add(3).read_unaligned();
    let get_cur = evtbl.add(4).read_unaligned();
    let mut metas = Vec::new();
    for _ in 0..4096 {
        let mut has_current: BOOL = FALSE;
        let hr_next = crate::call_guest8(
            move_next,
            enumerator as u64,
            &mut has_current as *mut BOOL as u64,
            0,
            0,
            0,
            0,
            0,
            0,
        ) as HRESULT;
        if hr_next < 0 {
            com_call_release(enumerator);
            return hr_next;
        }
        if has_current == FALSE {
            break;
        }
        let mut ffile: *mut c_void = std::ptr::null_mut();
        let hr_file = crate::call_guest8(
            get_cur,
            enumerator as u64,
            &mut ffile as *mut *mut c_void as u64,
            0,
            0,
            0,
            0,
            0,
            0,
        ) as HRESULT;
        if hr_file < 0 {
            com_call_release(enumerator);
            return hr_file;
        }
        if !ffile.is_null() {
            if let Some(list) = font_file_metadata_from_ptr(ffile) {
                for m in list {
                    metas.push(Arc::new(m));
                }
            }
            com_call_release(ffile);
        }
    }
    com_call_release(enumerator);
    let data = Arc::new(FontCollectionData::from_metas(metas));
    out.write_unaligned(new_font_collection(data));
    S_OK
}
unsafe extern "win64" fn fac_register_font_collection_loader(this: PFactory, loader: *mut c_void) -> HRESULT {
    if this.is_null() || loader.is_null() {
        return E_INVALIDARG;
    }
    let mut list = ComObj::inner(this).font_collection_loaders.lock().unwrap();
    let addr = loader as usize;
    if list.contains(&addr) {
        return DWRITE_E_ALREADYREGISTERED;
    }
    list.push(addr);
    S_OK
}
unsafe extern "win64" fn fac_unregister_font_collection_loader(this: PFactory, loader: *mut c_void) -> HRESULT {
    if this.is_null() || loader.is_null() {
        return E_INVALIDARG;
    }
    let mut list = ComObj::inner(this).font_collection_loaders.lock().unwrap();
    let addr = loader as usize;
    if let Some(pos) = list.iter().position(|&p| p == addr) {
        list.remove(pos);
        S_OK
    } else {
        E_INVALIDARG
    }
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
    this: PFactory,
    key: *const c_void,
    key_size: u32,
    loader: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    write_null_out(out);
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    if loader.is_null() || (key_size > 0 && key.is_null()) {
        return E_INVALIDARG;
    }
    let lvtbl = (loader as *const *const usize).read_unaligned();
    if lvtbl.is_null() {
        return E_INVALIDARG;
    }
    // IDWriteFontFileLoader::CreateStreamFromKey (slot 3)
    let create_stream = lvtbl.add(3).read_unaligned();
    if create_stream == 0 {
        return E_INVALIDARG;
    }
    let mut stream: *mut c_void = std::ptr::null_mut();
    let hr = crate::call_guest8(
        create_stream,
        loader as u64,
        key as u64,
        key_size as u64,
        &mut stream as *mut *mut c_void as u64,
        0,
        0,
        0,
        0,
    ) as HRESULT;
    if hr < 0 {
        return hr;
    }
    if stream.is_null() {
        return DWRITE_E_FILENOTFOUND;
    }
    let svtbl = (stream as *const *const usize).read_unaligned();
    if svtbl.is_null() {
        com_call_release(stream);
        return E_FAIL;
    }
    // IDWriteFontFileStream: slot 3 = ReadFileFragment, slot 4 = ReleaseFileFragment, slot 5 = GetFileSize
    let read_frag = svtbl.add(3).read_unaligned();
    let release_frag = svtbl.add(4).read_unaligned();
    let get_size = svtbl.add(5).read_unaligned();

    let mut file_size: u64 = 0;
    let hr_sz = crate::call_guest8(
        get_size,
        stream as u64,
        &mut file_size as *mut u64 as u64,
        0,
        0,
        0,
        0,
        0,
        0,
    ) as HRESULT;
    if hr_sz < 0 {
        com_call_release(stream);
        return hr_sz;
    }
    if file_size == 0 || file_size > 64 * 1024 * 1024 {
        com_call_release(stream);
        return DWRITE_E_FILEFORMAT;
    }
    let mut frag_ptr: *const c_void = std::ptr::null();
    let mut frag_ctx: *mut c_void = std::ptr::null_mut();
    let hr_rd = crate::call_guest8(
        read_frag,
        stream as u64,
        &mut frag_ptr as *mut *const c_void as u64,
        0,
        file_size,
        &mut frag_ctx as *mut *mut c_void as u64,
        0,
        0,
        0,
    ) as HRESULT;
    if hr_rd < 0 || frag_ptr.is_null() {
        com_call_release(stream);
        return if hr_rd < 0 { hr_rd } else { DWRITE_E_FILEACCESS };
    }
    let mut bytes = vec![0u8; file_size as usize];
    std::ptr::copy_nonoverlapping(frag_ptr as *const u8, bytes.as_mut_ptr(), file_size as usize);
    crate::call_guest8(release_frag, stream as u64, frag_ctx as u64, 0, 0, 0, 0, 0, 0);
    com_call_release(stream);

    let mut key_bytes = vec![0u8; key_size as usize];
    if key_size > 0 {
        std::ptr::copy_nonoverlapping(key as *const u8, key_bytes.as_mut_ptr(), key_size as usize);
    }
    out.write_unaligned(new_custom_font_file(Arc::new(bytes), loader, key_bytes));
    S_OK
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
    let Some(metas) = font_file_metadata_from_ptr(first) else {
        return E_INVALIDARG;
    };
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

unsafe extern "win64" fn fac_register_font_file_loader(this: PFactory, loader: *mut c_void) -> HRESULT {
    if this.is_null() || loader.is_null() {
        return E_INVALIDARG;
    }
    let mut list = ComObj::inner(this).font_file_loaders.lock().unwrap();
    let addr = loader as usize;
    if list.contains(&addr) {
        return DWRITE_E_ALREADYREGISTERED;
    }
    list.push(addr);
    S_OK
}
unsafe extern "win64" fn fac_unregister_font_file_loader(this: PFactory, loader: *mut c_void) -> HRESULT {
    if this.is_null() || loader.is_null() {
        return E_INVALIDARG;
    }
    let mut list = ComObj::inner(this).font_file_loaders.lock().unwrap();
    let addr = loader as usize;
    if let Some(pos) = list.iter().position(|&p| p == addr) {
        list.remove(pos);
        S_OK
    } else {
        E_INVALIDARG
    }
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
unsafe extern "win64" fn fac_create_text_analyzer(this: PFactory, out: *mut *mut c_void) -> HRESULT {
    write_null_out(out);
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    out.write_unaligned(new_text_analyzer());
    S_OK
}
unsafe extern "win64" fn fac_create_number_substitution(
    this: PFactory,
    method: u32,
    locale: *const u16,
    ignore_override: BOOL,
    out: *mut *mut c_void,
) -> HRESULT {
    write_null_out(out);
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    if method > font::DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL {
        return E_INVALIDARG;
    }
    let locale_str = if locale.is_null() {
        String::new()
    } else {
        read_wide_z(locale, DW_MAX_STR)
    };
    let obj = ComObj::new(
        num_sub_vtbl(),
        NumberSubstitutionInner {
            method,
            locale: locale_str,
            ignore_user_override: ignore_override != FALSE,
        },
    );
    out.write_unaligned(obj as *mut c_void);
    S_OK
}
unsafe extern "win64" fn fac_create_glyph_run_analysis(
    this: PFactory,
    glyph_run: *const DWRITE_GLYPH_RUN,
    pixels_per_dip: f32,
    transform: *const DWRITE_MATRIX,
    rendering_mode: u32,
    _measuring_mode: u32,
    baseline_x: f32,
    baseline_y: f32,
    out: *mut *mut c_void,
) -> HRESULT {
    write_null_out(out);
    if this.is_null() || glyph_run.is_null() || out.is_null() {
        return E_POINTER;
    }
    let run = glyph_run.read_unaligned();
    let mat = (!transform.is_null()).then(|| transform.read_unaligned());
    match GlyphRunAnalysisInner::build(&run, pixels_per_dip, mat.as_ref(), rendering_mode, baseline_x, baseline_y) {
        Ok(inner) => {
            out.write_unaligned(new_glyph_run_analysis(inner));
            S_OK
        }
        Err(hr) => hr,
    }
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
    let obj = ComObj::new(
        factory_vtbl(),
        FactoryInner {
            _factory_type: factory_type,
            font_file_loaders: Mutex::new(Vec::new()),
            font_collection_loaders: Mutex::new(Vec::new()),
        },
    );
    factory.write_unaligned(obj as *mut c_void);
    S_OK
}

// ============================================================================
// Unit Tests (strictly at the bottom for build.rs export scanner)
// ============================================================================

#[cfg(test)]
mod tests;
