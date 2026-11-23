//! Unit tests for `dwrite.dll` (loaded from `#[cfg(test)] mod tests;` at the bottom of `dwrite.rs`).

use super::font::{
    brt_draw_glyph_run, brt_get_memory_dc, face_get_design_glyph_metrics, face_get_glyph_count,
    face_get_glyph_indices, face_get_metrics, face_release_font_table, face_try_get_font_table,
    ff_file_analyze, font_create_face, gdi_convert_font_to_logfont, gdi_create_bitmap_rt,
    gdi_create_font_from_logfont, typo_add_feature, typo_get_count, typo_get_feature, PBmpRT,
    PFont, PFontFace, PFontFile, PGdiInterop, PTypography,
};
use super::layout::{
    tf_set_para_align, tf_set_text_align, tl_determine_min_width, tl_draw, tl_get_cluster_metrics,
    tl_get_line_metrics, tl_get_metrics, tl_get_overhang_metrics, tl_get_underline,
    tl_hit_test_point, tl_hit_test_text_position, tl_hit_test_text_range, tl_set_max_width,
    tl_set_underline, tl_set_word_wrap, PTextFormat, PTextLayout,
};
use super::*;
use crate::com::Guid;
use std::fs;

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

        let bogus = Guid::new(0xDEAD_BEEF, 1, 2, [3, 4, 5, 6, 7, 8, 9, 10]).to_win_guid();
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
