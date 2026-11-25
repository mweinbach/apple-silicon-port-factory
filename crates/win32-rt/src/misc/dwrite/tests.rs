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

#[test]
fn custom_font_file_and_collection_loaders_text_analyzer_and_glyph_run_analysis() {
    use super::analysis::{
        gra_create_alpha_texture, gra_get_alpha_blend_params, gra_get_alpha_texture_bounds,
        ta_get_gdi_compatible_glyph_placements, ta_get_glyph_placements, ta_get_glyphs,
        PGlyphRunAnalysis, PTextAnalyzer,
    };
    use crate::user32::RECT;

    // Mock IDWriteFontFileStream + IDWriteFontFileLoader + IDWriteFontFileEnumerator + IDWriteFontCollectionLoader
    #[repr(C)]
    struct MockStream {
        vtbl: *const [usize; 7],
        bytes: Vec<u8>,
    }
    unsafe extern "win64" fn ms_qi(_t: *mut MockStream, _r: *const u8, p: *mut *mut c_void) -> HRESULT {
        if !p.is_null() { p.write_unaligned(std::ptr::null_mut()); }
        E_NOINTERFACE
    }
    unsafe extern "win64" fn ms_addref(_t: *mut MockStream) -> u32 { 1 }
    unsafe extern "win64" fn ms_release(t: *mut MockStream) -> u32 {
        drop(Box::from_raw(t));
        0
    }
    unsafe extern "win64" fn ms_read_frag(
        t: *mut MockStream,
        frag_start: *mut *const c_void,
        off: u64,
        size: u64,
        frag_ctx: *mut *mut c_void,
    ) -> HRESULT {
        let s = &*t;
        if (off + size) as usize > s.bytes.len() {
            return E_INVALIDARG;
        }
        frag_start.write_unaligned(s.bytes.as_ptr().add(off as usize) as *const c_void);
        frag_ctx.write_unaligned(std::ptr::null_mut());
        S_OK
    }
    unsafe extern "win64" fn ms_rel_frag(_t: *mut MockStream, _ctx: *mut c_void) {}
    unsafe extern "win64" fn ms_get_size(t: *mut MockStream, sz: *mut u64) -> HRESULT {
        sz.write_unaligned((*t).bytes.len() as u64);
        S_OK
    }
    unsafe extern "win64" fn ms_get_time(_t: *mut MockStream, tm: *mut u64) -> HRESULT {
        tm.write_unaligned(0);
        S_OK
    }

    #[repr(C)]
    struct MockFileLoader {
        vtbl: *const [usize; 4],
        stream_vtbl: *const [usize; 7],
        ttf: Vec<u8>,
    }
    unsafe extern "win64" fn mfl_qi(_t: *mut MockFileLoader, _r: *const u8, p: *mut *mut c_void) -> HRESULT {
        if !p.is_null() { p.write_unaligned(std::ptr::null_mut()); }
        E_NOINTERFACE
    }
    unsafe extern "win64" fn mfl_addref(_t: *mut MockFileLoader) -> u32 { 1 }
    unsafe extern "win64" fn mfl_release(_t: *mut MockFileLoader) -> u32 { 1 }
    unsafe extern "win64" fn mfl_create_stream(
        t: *mut MockFileLoader,
        _key: *const c_void,
        _key_sz: u32,
        out_stream: *mut *mut c_void,
    ) -> HRESULT {
        let l = &*t;
        let st = Box::into_raw(Box::new(MockStream { vtbl: l.stream_vtbl, bytes: l.ttf.clone() }));
        out_stream.write_unaligned(st as *mut c_void);
        S_OK
    }

    #[repr(C)]
    struct MockEnum {
        vtbl: *const [usize; 5],
        fac: PFactory,
        file_loader: *mut c_void,
        idx: usize,
    }
    unsafe extern "win64" fn me_qi(_t: *mut MockEnum, _r: *const u8, p: *mut *mut c_void) -> HRESULT {
        if !p.is_null() { p.write_unaligned(std::ptr::null_mut()); }
        E_NOINTERFACE
    }
    unsafe extern "win64" fn me_addref(_t: *mut MockEnum) -> u32 { 1 }
    unsafe extern "win64" fn me_release(t: *mut MockEnum) -> u32 {
        drop(Box::from_raw(t));
        0
    }
    unsafe extern "win64" fn me_move_next(t: *mut MockEnum, has: *mut BOOL) -> HRESULT {
        let e = &mut *t;
        if e.idx == 0 {
            e.idx = 1;
            has.write_unaligned(TRUE);
        } else {
            has.write_unaligned(FALSE);
        }
        S_OK
    }
    unsafe extern "win64" fn me_get_cur(t: *mut MockEnum, out: *mut *mut c_void) -> HRESULT {
        let e = &*t;
        let key = 42u32;
        fac_create_custom_font_file_reference(
            e.fac,
            &key as *const u32 as *const c_void,
            4,
            e.file_loader,
            out,
        )
    }

    #[repr(C)]
    struct MockColLoader {
        vtbl: *const [usize; 4],
        enum_vtbl: *const [usize; 5],
        file_loader: *mut c_void,
    }
    unsafe extern "win64" fn mcl_qi(_t: *mut MockColLoader, _r: *const u8, p: *mut *mut c_void) -> HRESULT {
        if !p.is_null() { p.write_unaligned(std::ptr::null_mut()); }
        E_NOINTERFACE
    }
    unsafe extern "win64" fn mcl_addref(_t: *mut MockColLoader) -> u32 { 1 }
    unsafe extern "win64" fn mcl_release(_t: *mut MockColLoader) -> u32 { 1 }
    unsafe extern "win64" fn mcl_create_enum(
        t: *mut MockColLoader,
        fac: *mut c_void,
        _key: *const c_void,
        _key_sz: u32,
        out_enum: *mut *mut c_void,
    ) -> HRESULT {
        let l = &*t;
        let en = Box::into_raw(Box::new(MockEnum {
            vtbl: l.enum_vtbl,
            fac: fac as PFactory,
            file_loader: l.file_loader,
            idx: 0,
        }));
        out_enum.write_unaligned(en as *mut c_void);
        S_OK
    }

    // Mock IDWriteTextAnalysisSource + IDWriteTextAnalysisSink
    #[repr(C)]
    struct MockSource {
        vtbl: *const [usize; 8],
        text: Vec<u16>,
    }
    unsafe extern "win64" fn src_qi(_t: *mut MockSource, _r: *const u8, p: *mut *mut c_void) -> HRESULT {
        if !p.is_null() { p.write_unaligned(std::ptr::null_mut()); }
        E_NOINTERFACE
    }
    unsafe extern "win64" fn src_addref(_t: *mut MockSource) -> u32 { 1 }
    unsafe extern "win64" fn src_release(_t: *mut MockSource) -> u32 { 1 }
    unsafe extern "win64" fn src_get_at(t: *mut MockSource, pos: u32, ptr: *mut *const u16, len: *mut u32) -> HRESULT {
        let s = &*t;
        if pos as usize >= s.text.len() {
            ptr.write_unaligned(std::ptr::null());
            len.write_unaligned(0);
        } else {
            ptr.write_unaligned(s.text.as_ptr().add(pos as usize));
            len.write_unaligned((s.text.len() - pos as usize) as u32);
        }
        S_OK
    }
    unsafe extern "win64" fn src_get_before(_t: *mut MockSource, _pos: u32, ptr: *mut *const u16, len: *mut u32) -> HRESULT {
        ptr.write_unaligned(std::ptr::null());
        len.write_unaligned(0);
        S_OK
    }
    unsafe extern "win64" fn src_get_dir(_t: *mut MockSource) -> u32 { 0 }
    unsafe extern "win64" fn src_get_loc(_t: *mut MockSource, _pos: u32, len: *mut u32, loc: *mut *const u16) -> HRESULT {
        len.write_unaligned(0);
        loc.write_unaligned(std::ptr::null());
        S_OK
    }
    unsafe extern "win64" fn src_get_ns(_t: *mut MockSource, _pos: u32, len: *mut u32, ns: *mut *mut c_void) -> HRESULT {
        len.write_unaligned(0);
        ns.write_unaligned(std::ptr::null_mut());
        S_OK
    }

    #[repr(C)]
    struct MockSink {
        vtbl: *const [usize; 7],
        scripts: Vec<(u32, u32, u16)>,
        bps: Vec<u8>,
        bidis: Vec<(u32, u32, u8, u8)>,
    }
    unsafe extern "win64" fn snk_qi(_t: *mut MockSink, _r: *const u8, p: *mut *mut c_void) -> HRESULT {
        if !p.is_null() { p.write_unaligned(std::ptr::null_mut()); }
        E_NOINTERFACE
    }
    unsafe extern "win64" fn snk_addref(_t: *mut MockSink) -> u32 { 1 }
    unsafe extern "win64" fn snk_release(_t: *mut MockSink) -> u32 { 1 }
    unsafe extern "win64" fn snk_set_script(t: *mut MockSink, pos: u32, len: u32, sa: *const DWRITE_SCRIPT_ANALYSIS) -> HRESULT {
        (*t).scripts.push((pos, len, (*sa).script));
        S_OK
    }
    unsafe extern "win64" fn snk_set_bps(t: *mut MockSink, _pos: u32, len: u32, bps: *const DWRITE_LINE_BREAKPOINT) -> HRESULT {
        for i in 0..len as usize {
            (*t).bps.push(bps.add(i).read_unaligned().bits);
        }
        S_OK
    }
    unsafe extern "win64" fn snk_set_bidi(t: *mut MockSink, pos: u32, len: u32, exp: u8, res: u8) -> HRESULT {
        (*t).bidis.push((pos, len, exp, res));
        S_OK
    }
    unsafe extern "win64" fn snk_set_ns(_t: *mut MockSink, _pos: u32, _len: u32, _ns: *mut c_void) -> HRESULT {
        S_OK
    }

    unsafe {
        let stream_vtbl: [usize; 7] = [
            ms_qi as *const () as usize,
            ms_addref as *const () as usize,
            ms_release as *const () as usize,
            ms_read_frag as *const () as usize,
            ms_rel_frag as *const () as usize,
            ms_get_size as *const () as usize,
            ms_get_time as *const () as usize,
        ];
        let fl_vtbl: [usize; 4] = [
            mfl_qi as *const () as usize,
            mfl_addref as *const () as usize,
            mfl_release as *const () as usize,
            mfl_create_stream as *const () as usize,
        ];
        let en_vtbl: [usize; 5] = [
            me_qi as *const () as usize,
            me_addref as *const () as usize,
            me_release as *const () as usize,
            me_move_next as *const () as usize,
            me_get_cur as *const () as usize,
        ];
        let cl_vtbl: [usize; 4] = [
            mcl_qi as *const () as usize,
            mcl_addref as *const () as usize,
            mcl_release as *const () as usize,
            mcl_create_enum as *const () as usize,
        ];
        let src_vtbl: [usize; 8] = [
            src_qi as *const () as usize,
            src_addref as *const () as usize,
            src_release as *const () as usize,
            src_get_at as *const () as usize,
            src_get_before as *const () as usize,
            src_get_dir as *const () as usize,
            src_get_loc as *const () as usize,
            src_get_ns as *const () as usize,
        ];
        let snk_vtbl: [usize; 7] = [
            snk_qi as *const () as usize,
            snk_addref as *const () as usize,
            snk_release as *const () as usize,
            snk_set_script as *const () as usize,
            snk_set_bps as *const () as usize,
            snk_set_bidi as *const () as usize,
            snk_set_ns as *const () as usize,
        ];
        crate::com::register_vtable_ptrs(&stream_vtbl);
        crate::com::register_vtable_ptrs(&fl_vtbl);
        crate::com::register_vtable_ptrs(&en_vtbl);
        crate::com::register_vtable_ptrs(&cl_vtbl);
        crate::com::register_vtable_ptrs(&src_vtbl);
        crate::com::register_vtable_ptrs(&snk_vtbl);

        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &IID_IDWRITE_FACTORY as *const GUID as *const u8,
                &mut fac,
            ),
            S_OK
        );

        let ttf = build_synthetic_ttf();
        let mut fl = MockFileLoader { vtbl: &fl_vtbl, stream_vtbl: &stream_vtbl, ttf };
        assert_eq!(fac_register_font_file_loader(fac as PFactory, &mut fl as *mut _ as *mut c_void), S_OK);
        assert_eq!(
            fac_register_font_file_loader(fac as PFactory, &mut fl as *mut _ as *mut c_void),
            DWRITE_E_ALREADYREGISTERED
        );

        // 1. CreateCustomFontFileReference -> Analyze -> CreateFontFace
        let key = 0x1234u32;
        let mut ffile: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_custom_font_file_reference(
                fac as PFactory,
                &key as *const u32 as *const c_void,
                4,
                &mut fl as *mut _ as *mut c_void,
                &mut ffile,
            ),
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
        assert_eq!((supported, ftype, nfaces), (TRUE, DWRITE_FONT_FILE_TYPE_TRUETYPE, 1));

        let mut face: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_font_face(
                fac as PFactory,
                DWRITE_FONT_FACE_TYPE_TRUETYPE,
                1,
                &ffile,
                0,
                DWRITE_FONT_SIMULATIONS_NONE,
                &mut face,
            ),
            S_OK
        );
        assert_eq!(face_get_glyph_count(face as PFontFace), 2);

        // 1b. IDWriteFontFace::GetFiles must retain the custom backing file's loader & reference key.
        let face_get_files_fn: unsafe extern "win64" fn(*mut c_void, *mut u32, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_of(face).add(4).read_unaligned());
        let mut fcount = 0u32;
        assert_eq!(face_get_files_fn(face, &mut fcount, std::ptr::null_mut()), S_OK);
        assert_eq!(fcount, 1, "custom font face must report its backing font file");
        let mut out_file: *mut c_void = std::ptr::null_mut();
        assert_eq!(face_get_files_fn(face, &mut fcount, &mut out_file), S_OK);
        assert_eq!(fcount, 1);
        assert!(!out_file.is_null());
        let ff_get_key: unsafe extern "win64" fn(*mut c_void, *mut *const c_void, *mut u32) -> HRESULT =
            std::mem::transmute(vtbl_of(out_file).add(3).read_unaligned());
        let ff_get_loader: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_of(out_file).add(4).read_unaligned());
        let mut rk_ptr: *const c_void = std::ptr::null();
        let mut rk_sz = 0u32;
        assert_eq!(ff_get_key(out_file, &mut rk_ptr, &mut rk_sz), S_OK);
        assert_eq!(rk_sz, 4);
        assert_eq!(std::ptr::read_unaligned(rk_ptr as *const u32), key);
        let mut got_loader: *mut c_void = std::ptr::null_mut();
        assert_eq!(ff_get_loader(out_file, &mut got_loader), S_OK);
        assert_eq!(got_loader, &mut fl as *mut _ as *mut c_void);
        release_com(got_loader);
        release_com(out_file);

        // 2. CreateCustomFontCollection
        let mut cl = MockColLoader {
            vtbl: &cl_vtbl,
            enum_vtbl: &en_vtbl,
            file_loader: &mut fl as *mut _ as *mut c_void,
        };
        assert_eq!(fac_register_font_collection_loader(fac as PFactory, &mut cl as *mut _ as *mut c_void), S_OK);
        let mut col: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_custom_font_collection(
                fac as PFactory,
                &mut cl as *mut _ as *mut c_void,
                &key as *const u32 as *const c_void,
                4,
                &mut col,
            ),
            S_OK
        );
        let get_cnt: unsafe extern "win64" fn(*mut c_void) -> u32 =
            std::mem::transmute(vtbl_of(col).add(3).read_unaligned());
        assert_eq!(get_cnt(col), 1);

        // 3. CreateTextAnalyzer: AnalyzeScript, AnalyzeBidi, AnalyzeLineBreakpoints, GetGlyphs, GetGlyphPlacements
        let mut ta: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_text_analyzer(fac as PFactory, &mut ta), S_OK);
        let text_u16: Vec<u16> = "A \u{05D0}".encode_utf16().collect(); // Latin 'A', space, Hebrew Alef
        let mut src = MockSource { vtbl: &src_vtbl, text: text_u16.clone() };
        let mut snk = MockSink { vtbl: &snk_vtbl, scripts: Vec::new(), bps: Vec::new(), bidis: Vec::new() };

        let an_script: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_of(ta).add(3).read_unaligned());
        let an_bidi: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_of(ta).add(4).read_unaligned());
        let an_bp: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, *mut c_void) -> HRESULT =
            std::mem::transmute(vtbl_of(ta).add(6).read_unaligned());
        assert_eq!(an_script(ta, &mut src as *mut _ as *mut c_void, 0, text_u16.len() as u32, &mut snk as *mut _ as *mut c_void), S_OK);
        assert_eq!(snk.scripts.len(), 2); // Latin run + Hebrew run
        assert_eq!(an_bidi(ta, &mut src as *mut _ as *mut c_void, 0, text_u16.len() as u32, &mut snk as *mut _ as *mut c_void), S_OK);
        assert!(snk.bidis.iter().any(|&(_, _, _, res)| res == 1)); // Hebrew resolved level 1
        assert_eq!(an_bp(ta, &mut src as *mut _ as *mut c_void, 0, text_u16.len() as u32, &mut snk as *mut _ as *mut c_void), S_OK);
        assert_eq!(snk.bps.len(), 3);
        assert_ne!(snk.bps[1] & (1 << 4), 0); // space isWhitespace

        let a_u16 = [b'A' as u16];
        let mut cmap = [0u16; 1];
        let mut tprops = [DWRITE_SHAPING_TEXT_PROPERTIES::default(); 1];
        let mut gids = [0u16; 1];
        let mut gprops = [DWRITE_SHAPING_GLYPH_PROPERTIES::default(); 1];
        let mut actual = 0u32;
        assert_eq!(
            ta_get_glyphs(
                ta as PTextAnalyzer,
                a_u16.as_ptr(),
                1,
                face,
                FALSE,
                FALSE,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                0,
                1,
                cmap.as_mut_ptr(),
                tprops.as_mut_ptr(),
                gids.as_mut_ptr(),
                gprops.as_mut_ptr(),
                &mut actual,
            ),
            S_OK
        );
        assert_eq!((actual, gids[0], cmap[0]), (1, 1, 0));

        let mut adv = [0.0f32; 1];
        let mut offs = [DWRITE_GLYPH_OFFSET::default(); 1];
        assert_eq!(
            ta_get_glyph_placements(
                ta as PTextAnalyzer,
                a_u16.as_ptr(),
                cmap.as_ptr(),
                tprops.as_ptr(),
                1,
                gids.as_ptr(),
                gprops.as_ptr(),
                1,
                face,
                20.0f32.to_bits(),
                FALSE,
                FALSE,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                0,
                adv.as_mut_ptr(),
                offs.as_mut_ptr(),
            ),
            S_OK
        );
        assert!((adv[0] - 14.4).abs() < 1e-2, "expected 720*20/1000=14.4, got {}", adv[0]);

        let mut gdi_adv = [0.0f32; 1];
        assert_eq!(
            ta_get_gdi_compatible_glyph_placements(
                ta as PTextAnalyzer,
                a_u16.as_ptr(),
                cmap.as_ptr(),
                tprops.as_ptr(),
                1,
                gids.as_ptr(),
                gprops.as_ptr(),
                1,
                face,
                20.0f32.to_bits(),
                1.0f32.to_bits(),
                std::ptr::null(),
                FALSE,
                FALSE,
                FALSE,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                0,
                gdi_adv.as_mut_ptr(),
                offs.as_mut_ptr(),
            ),
            S_OK
        );
        assert_eq!(gdi_adv[0], 14.0);

        // 3b. Exercise the actual COM vtable entry thunks (slots 7, 8, 9) using the real
        //     guest-ABI path (16 u64 slots + Win64 incoming_rsp stack window for args 16..21).
        #[cfg(target_arch = "aarch64")]
        {
            use std::sync::atomic::{AtomicU64, Ordering};
            static TEST_RSP_LO: AtomicU64 = AtomicU64::new(0);
            static TEST_RSP_HI: AtomicU64 = AtomicU64::new(0);
            fn test_incoming_rsp() -> (u64, u64) {
                (TEST_RSP_LO.load(Ordering::Acquire), TEST_RSP_HI.load(Ordering::Acquire))
            }
            type Vtbl16Fn = unsafe extern "C" fn(
                u64, u64, u64, u64, u64, u64, u64, u64,
                u64, u64, u64, u64, u64, u64, u64, u64,
            ) -> HRESULT;

            let call_vtbl_win64 = |slot: usize, args: &[u64]| -> HRESULT {
                // Build a Win64 stack frame: [rsp+0x00..0x20] = shadow space,
                // [rsp+0x20 + (i-4)*8] = arg i for i >= 4.
                let mut stack = [0u64; 32]; // 0x100 bytes
                for (i, &v) in args.iter().enumerate().skip(4) {
                    stack[4 + (i - 4)] = v;
                }
                let lo = stack.as_ptr() as u64;
                let hi = lo + (stack.len() * 8) as u64;
                TEST_RSP_LO.store(lo, Ordering::Release);
                TEST_RSP_HI.store(hi, Ordering::Release);
                crate::set_incoming_rsp_reader(test_incoming_rsp);
                let a = |i: usize| args.get(i).copied().unwrap_or(0);
                let f: Vtbl16Fn = std::mem::transmute(vtbl_of(ta).add(slot).read_unaligned());
                let hr = f(
                    a(0), a(1), a(2), a(3), a(4), a(5), a(6), a(7),
                    a(8), a(9), a(10), a(11), a(12), a(13), a(14), a(15),
                );
                TEST_RSP_LO.store(0, Ordering::Release);
                TEST_RSP_HI.store(0, Ordering::Release);
                hr
            };

            // Slot 7: GetGlyphs (18 args: glyphProps @ #16, actualGlyphCount @ #17)
            let mut v_cmap = [0xFFFFu16; 1];
            let mut v_tprops = [DWRITE_SHAPING_TEXT_PROPERTIES::default(); 1];
            let mut v_gids = [0u16; 1];
            let mut v_gprops = [DWRITE_SHAPING_GLYPH_PROPERTIES::default(); 1];
            let mut v_actual = 0u32;
            assert_eq!(
                call_vtbl_win64(
                    7,
                    &[
                        ta as u64,
                        a_u16.as_ptr() as u64,
                        1,
                        face as u64,
                        FALSE as u64,
                        FALSE as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        1, // maxGlyphCount (#12)
                        v_cmap.as_mut_ptr() as u64,
                        v_tprops.as_mut_ptr() as u64,
                        v_gids.as_mut_ptr() as u64,
                        v_gprops.as_mut_ptr() as u64, // #16 -> [rsp+0x80]
                        &mut v_actual as *mut u32 as u64, // #17 -> [rsp+0x88]
                    ],
                ),
                S_OK
            );
            assert_eq!((v_actual, v_gids[0], v_cmap[0]), (1, 1, 0));
            assert_ne!(v_gprops[0].bits, 0);

            // Slot 8: GetGlyphPlacements (19 args: fontEmSize @ #9, glyphAdvances @ #17, glyphOffsets @ #18)
            let mut v_adv = [0.0f32; 1];
            let mut v_offs = [DWRITE_GLYPH_OFFSET { advanceOffset: 99.0, ascenderOffset: 99.0 }; 1];
            assert_eq!(
                call_vtbl_win64(
                    8,
                    &[
                        ta as u64,
                        a_u16.as_ptr() as u64,
                        v_cmap.as_ptr() as u64,
                        v_tprops.as_ptr() as u64,
                        1,
                        v_gids.as_ptr() as u64,
                        v_gprops.as_ptr() as u64,
                        1,
                        face as u64,
                        20.0f32.to_bits() as u64,
                        FALSE as u64,
                        FALSE as u64,
                        0,
                        0,
                        0,
                        0,
                        0, // featureRanges (#16)
                        v_adv.as_mut_ptr() as u64, // #17 -> [rsp+0x88]
                        v_offs.as_mut_ptr() as u64, // #18 -> [rsp+0x90]
                    ],
                ),
                S_OK
            );
            assert!((v_adv[0] - 14.4).abs() < 1e-2, "vtable GetGlyphPlacements adv={}", v_adv[0]);
            assert_eq!((v_offs[0].advanceOffset, v_offs[0].ascenderOffset), (0.0, 0.0));

            // Slot 9: GetGdiCompatibleGlyphPlacements (22 args: glyphAdvances @ #20, glyphOffsets @ #21)
            let mut v_gdi_adv = [0.0f32; 1];
            v_offs[0] = DWRITE_GLYPH_OFFSET { advanceOffset: 77.0, ascenderOffset: 77.0 };
            assert_eq!(
                call_vtbl_win64(
                    9,
                    &[
                        ta as u64,
                        a_u16.as_ptr() as u64,
                        v_cmap.as_ptr() as u64,
                        v_tprops.as_ptr() as u64,
                        1,
                        v_gids.as_ptr() as u64,
                        v_gprops.as_ptr() as u64,
                        1,
                        face as u64,
                        20.0f32.to_bits() as u64,
                        1.0f32.to_bits() as u64,
                        0,
                        FALSE as u64,
                        FALSE as u64,
                        FALSE as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        v_gdi_adv.as_mut_ptr() as u64, // #20 -> [rsp+0xA0]
                        v_offs.as_mut_ptr() as u64,    // #21 -> [rsp+0xA8]
                    ],
                ),
                S_OK
            );
            assert_eq!(v_gdi_adv[0], 14.0);
            assert_eq!((v_offs[0].advanceOffset, v_offs[0].ascenderOffset), (0.0, 0.0));
        }

        // 4. CreateGlyphRunAnalysis: GetAlphaTextureBounds + CreateAlphaTexture (1x1 & 3x1) + GetAlphaBlendParams
        let run = DWRITE_GLYPH_RUN {
            fontFace: face,
            fontEmSize: 32.0,
            glyphCount: 1,
            glyphIndices: gids.as_ptr(),
            glyphAdvances: adv.as_ptr(),
            glyphOffsets: offs.as_ptr(),
            isSideways: FALSE,
            bidiLevel: 0,
        };
        let mut gra: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run,
                1.0,
                std::ptr::null(),
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                0.0,
                32.0,
                &mut gra,
            ),
            S_OK
        );
        let mut bounds = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut bounds), S_OK);
        let w = (bounds.right - bounds.left) as usize;
        let h = (bounds.bottom - bounds.top) as usize;
        assert!(w >= 15 && h >= 15, "bounds={bounds:?}");

        // ClearType/Default run has empty bounds and zero pixels when queried as ALIASED_1X1.
        let mut empty_aliased = RECT { left: 1, top: 2, right: 3, bottom: 4 };
        assert_eq!(gra_get_alpha_texture_bounds(gra as PGlyphRunAnalysis, DWRITE_TEXTURE_ALIASED_1X1, &mut empty_aliased), S_OK);
        assert_eq!((empty_aliased.left, empty_aliased.top, empty_aliased.right, empty_aliased.bottom), (0, 0, 0, 0));
        let mut mismatched_tex1 = vec![0xAAu8; w * h];
        assert_eq!(
            gra_create_alpha_texture(gra as PGlyphRunAnalysis, DWRITE_TEXTURE_ALIASED_1X1, &bounds, mismatched_tex1.as_mut_ptr(), mismatched_tex1.len() as u32),
            S_OK
        );
        assert!(mismatched_tex1.iter().all(|&a| a == 0), "ClearType run must not emit pixels into ALIASED_1X1 texture");

        let mut tex3 = vec![0u8; w * h * 3];
        assert_eq!(
            gra_create_alpha_texture(gra as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &bounds, tex3.as_mut_ptr(), tex3.len() as u32),
            S_OK
        );
        assert!(tex3.iter().filter(|&&a| a >= 200).count() >= 180);

        // Aliased run reports nonempty ALIASED_1X1 bounds/binary pixels and empty CLEARTYPE_3X1 bounds/zeros.
        let mut gra_aliased: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run,
                1.0,
                std::ptr::null(),
                DWRITE_RENDERING_MODE_ALIASED,
                DWRITE_MEASURING_MODE_NATURAL,
                0.0,
                32.0,
                &mut gra_aliased,
            ),
            S_OK
        );
        let mut b_al = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_aliased as PGlyphRunAnalysis, DWRITE_TEXTURE_ALIASED_1X1, &mut b_al), S_OK);
        assert_eq!((b_al.left, b_al.top, b_al.right, b_al.bottom), (bounds.left, bounds.top, bounds.right, bounds.bottom));
        let mut empty_ct = RECT { left: 1, top: 2, right: 3, bottom: 4 };
        assert_eq!(gra_get_alpha_texture_bounds(gra_aliased as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut empty_ct), S_OK);
        assert_eq!((empty_ct.left, empty_ct.top, empty_ct.right, empty_ct.bottom), (0, 0, 0, 0));
        let mut tex1 = vec![0u8; w * h];
        assert_eq!(
            gra_create_alpha_texture(gra_aliased as PGlyphRunAnalysis, DWRITE_TEXTURE_ALIASED_1X1, &b_al, tex1.as_mut_ptr(), tex1.len() as u32),
            S_OK
        );
        assert!(tex1.iter().filter(|&&a| a == 255).count() >= 60);
        assert!(tex1.iter().all(|&a| a == 0 || a == 255), "aliased 1x1 texture must be binary");
        let mut mismatched_tex3 = vec![0xBBu8; w * h * 3];
        assert_eq!(
            gra_create_alpha_texture(gra_aliased as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &b_al, mismatched_tex3.as_mut_ptr(), mismatched_tex3.len() as u32),
            S_OK
        );
        assert!(mismatched_tex3.iter().all(|&a| a == 0), "aliased run must not emit pixels into CLEARTYPE_3X1 texture");
        release_com(gra_aliased);

        // 4b. Non-uniform scale (m11=2, m22=1) and 90-degree rotation must transform glyph
        //     coverage & bounds, not only glyph origins.
        let wide_mat = DWRITE_MATRIX { m11: 2.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 0.0, dy: 0.0 };
        let mut gra_wide: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run,
                1.0,
                &wide_mat,
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                0.0,
                32.0,
                &mut gra_wide,
            ),
            S_OK
        );
        let mut b_wide = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_wide as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut b_wide), S_OK);
        let w_wide = (b_wide.right - b_wide.left) as usize;
        let h_wide = (b_wide.bottom - b_wide.top) as usize;
        assert!(w_wide >= w * 2 - 4 && w_wide <= w * 2 + 4, "expected 2x width: base={w}, wide={w_wide}");
        assert!((h_wide as i32 - h as i32).abs() <= 2, "expected unchanged height: base={h}, wide={h_wide}");
        let mut tex_wide = vec![0u8; w_wide * h_wide * 3];
        assert_eq!(
            gra_create_alpha_texture(gra_wide as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &b_wide, tex_wide.as_mut_ptr(), tex_wide.len() as u32),
            S_OK
        );
        let lit_base = tex3.iter().filter(|&&a| a >= 200).count();
        let lit_wide = tex_wide.iter().filter(|&&a| a >= 200).count();
        assert!(lit_wide >= lit_base * 3 / 2, "expected ~2x lit pixels under 2x X-scale: base={lit_base}, wide={lit_wide}");

        // 90-degree CW rotation (m11=0, m12=1, m21=-1, m22=0) on a 2x-wide rectangle swaps aspect ratio.
        let rot90_wide = DWRITE_MATRIX { m11: 0.0, m12: 2.0, m21: -1.0, m22: 0.0, dx: 0.0, dy: 0.0 };
        let mut gra_rot: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run,
                1.0,
                &rot90_wide,
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                0.0,
                32.0,
                &mut gra_rot,
            ),
            S_OK
        );
        let mut b_rot = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_rot as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut b_rot), S_OK);
        let w_rot = (b_rot.right - b_rot.left) as usize;
        let h_rot = (b_rot.bottom - b_rot.top) as usize;
        assert!((w_rot as i32 - h_wide as i32).abs() <= 3, "rotated width {w_rot} should match unrotated height {h_wide}");
        assert!((h_rot as i32 - w_wide as i32).abs() <= 3, "rotated height {h_rot} should match unrotated width {w_wide}");
        release_com(gra_rot);
        release_com(gra_wide);

        // 4c. RTL glyph run (odd bidiLevel) must subtract each glyph's advance BEFORE placing
        //     its left-origin outline, so a glyph with baseline_x=100 sits to the LEFT of 100.
        let rtl_adv = [25.0f32];
        let rtl_run = DWRITE_GLYPH_RUN {
            fontFace: face,
            fontEmSize: 32.0,
            glyphCount: 1,
            glyphIndices: gids.as_ptr(),
            glyphAdvances: rtl_adv.as_ptr(),
            glyphOffsets: offs.as_ptr(),
            isSideways: FALSE,
            bidiLevel: 1,
        };
        let mut gra_rtl: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &rtl_run,
                1.0,
                std::ptr::null(),
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                100.0,
                32.0,
                &mut gra_rtl,
            ),
            S_OK
        );
        let mut b_rtl = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_rtl as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut b_rtl), S_OK);
        // LTR bounds at baseline_x=0 were [bounds.left..bounds.right]; RTL at 100 with adv=25
        // places the glyph origin at 100 - 25 = 75, so bounds shift by +75 (strictly left of 100+bounds.left).
        assert_eq!(b_rtl.left, bounds.left + 75, "RTL left bound: ltr={bounds:?}, rtl={b_rtl:?}");
        assert_eq!(b_rtl.right, bounds.right + 75, "RTL right bound: ltr={bounds:?}, rtl={b_rtl:?}");
        release_com(gra_rtl);

        let mut rp: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_custom_rendering_params(fac as PFactory, 2.2, 0.75, 0.9, DWRITE_PIXEL_GEOMETRY_RGB, DWRITE_RENDERING_MODE_DEFAULT, &mut rp), S_OK);
        let (mut g, mut ec, mut ct) = (0.0f32, 0.0f32, 0.0f32);
        assert_eq!(gra_get_alpha_blend_params(gra as PGlyphRunAnalysis, rp, &mut g, &mut ec, &mut ct), S_OK);
        assert_eq!((g, ec, ct), (2.2, 0.75, 0.9));

        assert_eq!(fac_unregister_font_collection_loader(fac as PFactory, &mut cl as *mut _ as *mut c_void), S_OK);
        assert_eq!(fac_unregister_font_file_loader(fac as PFactory, &mut fl as *mut _ as *mut c_void), S_OK);

        release_com(rp);
        release_com(gra);
        release_com(ta);
        release_com(col);
        release_com(face);
        release_com(ffile);
        release_com(fac);
    }
}

#[test]
fn text_analyzer_shapes_complex_arabic_and_combining_marks_and_rejects_unshapeable_fallback() {
    use super::analysis::{ta_get_glyph_placements, ta_get_glyphs, PTextAnalyzer};
    use super::sfnt::FontMeta;
    use std::collections::HashMap;
    unsafe {
        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &G_IDWRITE_FACTORY as *const _ as *const u8,
                &mut fac,
            ),
            S_OK
        );
        let mut ta: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_text_analyzer(fac as PFactory, &mut ta), S_OK);

        // 1. Find a real system font with Arabic contextual shaping (e.g. Geeza Pro or Arial on macOS).
        let sys = system_font_collection_data(false);
        let arabic_meta = ["geeza pro", "arial", "damascus", "baghdad", "times new roman"]
            .iter()
            .find_map(|&fam_name| {
                let idx = sys.find_family_index(fam_name)?;
                sys.families[idx]
                    .fonts
                    .iter()
                    .find(|m| m.file_path.is_some())
                    .cloned()
            });

        if let Some(meta) = arabic_meta {
            let face_inner = FontFaceInner::from_meta(meta, 0);
            // Connected Arabic word "سلام" (Seen + Lam + Alef + Meem): contextual joining + Lam-Alef ligature.
            let arabic_u16: Vec<u16> = "\u{0633}\u{0644}\u{0627}\u{0645}".encode_utf16().collect();
            let raw_cmap_gids: Vec<u16> = [0x0633u32, 0x0644, 0x0627, 0x0645]
                .iter()
                .map(|&cp| face_inner.codepoint_to_glyph(cp))
                .collect();
            let face = new_font_face(face_inner);

            let mut cmap = [0xFFFFu16; 4];
            let mut tprops = [DWRITE_SHAPING_TEXT_PROPERTIES::default(); 4];
            let mut gids = [0u16; 16];
            let mut gprops = [DWRITE_SHAPING_GLYPH_PROPERTIES::default(); 16];
            let mut actual = 0u32;
            let sa = DWRITE_SCRIPT_ANALYSIS { script: 1, shapes: 0 };
            assert_eq!(
                ta_get_glyphs(
                    ta as PTextAnalyzer,
                    arabic_u16.as_ptr(),
                    arabic_u16.len() as u32,
                    face,
                    FALSE,
                    TRUE, // isRightToLeft
                    &sa,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    gids.len() as u32,
                    cmap.as_mut_ptr(),
                    tprops.as_mut_ptr(),
                    gids.as_mut_ptr(),
                    gprops.as_mut_ptr(),
                    &mut actual,
                ),
                S_OK
            );
            assert!(actual > 0 && actual <= 4, "expected shaped Arabic glyphs, got {actual}");
            let shaped_slice = &gids[..actual as usize];
            assert_ne!(
                shaped_slice,
                raw_cmap_gids.as_slice(),
                "Arabic run must undergo contextual shaping rather than returning isolated cmap glyphs"
            );
            // Monotonic logical clusterMap covering all 4 UTF-16 code units.
            for w in cmap.windows(2) {
                assert!(w[0] <= w[1], "clusterMap must be monotonically non-decreasing: {cmap:?}");
            }
            assert!((cmap[3] as u32) < actual);
            // If Lam+Alef formed a ligature (3 glyphs for 4 chars), indices 1 & 2 share one cluster.
            if actual == 3 {
                assert_eq!(cmap[1], cmap[2], "Lam+Alef ligature should share one cluster: {cmap:?}");
            }

            let mut adv = [0.0f32; 16];
            let mut offs = [DWRITE_GLYPH_OFFSET::default(); 16];
            assert_eq!(
                ta_get_glyph_placements(
                    ta as PTextAnalyzer,
                    arabic_u16.as_ptr(),
                    cmap.as_ptr(),
                    tprops.as_ptr(),
                    arabic_u16.len() as u32,
                    gids.as_ptr(),
                    gprops.as_ptr(),
                    actual,
                    face,
                    24.0f32.to_bits(),
                    FALSE,
                    TRUE,
                    &sa,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    adv.as_mut_ptr(),
                    offs.as_mut_ptr(),
                ),
                S_OK
            );
            assert!(adv[..actual as usize].iter().any(|&a| a > 0.0));
            release_com(face);
        }

        // 2. Combining mark ("e" + U+0301 combining acute) with a real Latin system font.
        let latin_meta = ["arial", "helvetica", "times new roman"]
            .iter()
            .find_map(|&fam_name| {
                let idx = sys.find_family_index(fam_name)?;
                sys.families[idx]
                    .fonts
                    .iter()
                    .find(|m| m.file_path.is_some())
                    .cloned()
            });
        if let Some(meta) = latin_meta {
            let face_inner = FontFaceInner::from_meta(meta, 0);
            let face = new_font_face(face_inner);
            let comb_u16: Vec<u16> = "e\u{0301}".encode_utf16().collect();
            let mut cmap = [0xFFFFu16; 2];
            let mut tprops = [DWRITE_SHAPING_TEXT_PROPERTIES::default(); 2];
            let mut gids = [0u16; 8];
            let mut gprops = [DWRITE_SHAPING_GLYPH_PROPERTIES::default(); 8];
            let mut actual = 0u32;
            assert_eq!(
                ta_get_glyphs(
                    ta as PTextAnalyzer,
                    comb_u16.as_ptr(),
                    comb_u16.len() as u32,
                    face,
                    FALSE,
                    FALSE,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    gids.len() as u32,
                    cmap.as_mut_ptr(),
                    tprops.as_mut_ptr(),
                    gids.as_mut_ptr(),
                    gprops.as_mut_ptr(),
                    &mut actual,
                ),
                S_OK
            );
            let mut adv = [0.0f32; 8];
            let mut offs = [DWRITE_GLYPH_OFFSET::default(); 8];
            assert_eq!(
                ta_get_glyph_placements(
                    ta as PTextAnalyzer,
                    comb_u16.as_ptr(),
                    cmap.as_ptr(),
                    tprops.as_ptr(),
                    comb_u16.len() as u32,
                    gids.as_ptr(),
                    gprops.as_ptr(),
                    actual,
                    face,
                    24.0f32.to_bits(),
                    FALSE,
                    FALSE,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    adv.as_mut_ptr(),
                    offs.as_mut_ptr(),
                ),
                S_OK
            );
            // Either composed into 1 glyph (clusterMap=[0,0]) or 2 glyphs with zero-advance combining mark.
            if actual == 1 {
                assert_eq!(cmap, [0, 0]);
                assert!(adv[0] > 0.0);
            } else {
                assert_eq!(actual, 2);
                assert!(adv[0] > 0.0);
                assert!(adv[1].abs() < 1e-3, "combining mark advance should be 0, got {}", adv[1]);
            }
            release_com(face);
        }

        // 3. Built-in fallback face (no backing SFNT bytes) must honestly return E_NOTIMPL for
        //    complex Arabic runs rather than returning S_OK with isolated cmap glyphs.
        let fallback_meta = Arc::new(FontMeta {
            family_name: "FallbackOnly".to_string(),
            face_name: "Regular".to_string(),
            weight: DWRITE_FONT_WEIGHT_NORMAL,
            stretch: DWRITE_FONT_STRETCH_NORMAL,
            style: DWRITE_FONT_STYLE_NORMAL,
            is_symbol: false,
            is_monospace: false,
            face_type: DWRITE_FONT_FACE_TYPE_TRUETYPE,
            face_index: 0,
            glyph_count: 256,
            metrics: super::sfnt::default_fallback_metrics(),
            info_strings: HashMap::new(),
            file_path: None,
            raw_bytes: None,
            custom_file: None,
        });
        let fb_face = new_font_face(FontFaceInner::from_meta(fallback_meta, 0));
        let arabic_u16: Vec<u16> = "\u{0633}\u{0644}\u{0627}\u{0645}".encode_utf16().collect();
        let mut cmap = [0u16; 4];
        let mut tprops = [DWRITE_SHAPING_TEXT_PROPERTIES::default(); 4];
        let mut gids = [0u16; 8];
        let mut gprops = [DWRITE_SHAPING_GLYPH_PROPERTIES::default(); 8];
        let mut actual = 99u32;
        assert_eq!(
            ta_get_glyphs(
                ta as PTextAnalyzer,
                arabic_u16.as_ptr(),
                arabic_u16.len() as u32,
                fb_face,
                FALSE,
                TRUE,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                0,
                gids.len() as u32,
                cmap.as_mut_ptr(),
                tprops.as_mut_ptr(),
                gids.as_mut_ptr(),
                gprops.as_mut_ptr(),
                &mut actual,
            ),
            E_NOTIMPL
        );
        assert_eq!(actual, 0);
        release_com(fb_face);

        // 4. Bidi numeric resolution (UAX #9 I2): "אב12ג" in an RTL paragraph must resolve
        //    to levels [1, 1, 2, 2, 1] rather than assigning digits the odd base level.
        #[repr(C)]
        struct RtlSource {
            vtbl: *const [usize; 8],
            text: Vec<u16>,
        }
        #[repr(C)]
        struct BidiSink {
            vtbl: *const [usize; 7],
            bidis: Vec<(u32, u32, u8, u8)>,
        }
        unsafe extern "win64" fn rtl_get_text(this: *mut RtlSource, pos: u32, out_str: *mut *const u16, out_len: *mut u32) -> HRESULT {
            let p = pos as usize;
            if p >= (*this).text.len() {
                out_str.write_unaligned(std::ptr::null());
                out_len.write_unaligned(0);
            } else {
                out_str.write_unaligned((*this).text.as_ptr().add(p));
                out_len.write_unaligned(((*this).text.len() - p) as u32);
            }
            S_OK
        }
        unsafe extern "win64" fn rtl_get_dir(_this: *mut RtlSource) -> u32 {
            1 // DWRITE_READING_DIRECTION_RIGHT_TO_LEFT
        }
        unsafe extern "win64" fn bidi_set_level(this: *mut BidiSink, pos: u32, len: u32, expl: u8, res: u8) -> HRESULT {
            (*this).bidis.push((pos, len, expl, res));
            S_OK
        }
        let rtl_vtbl = [0, 0, 0, rtl_get_text as *const () as usize, 0, rtl_get_dir as *const () as usize, 0, 0];
        let bidi_vtbl = [0, 0, 0, 0, 0, bidi_set_level as *const () as usize, 0];
        let hebrew_digits: Vec<u16> = "\u{05D0}\u{05D1}12\u{05D2}".encode_utf16().collect();
        let mut rsrc = RtlSource { vtbl: &rtl_vtbl, text: hebrew_digits.clone() };
        let mut bsink = BidiSink { vtbl: &bidi_vtbl, bidis: Vec::new() };
        let an_bidi: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, *mut c_void) -> HRESULT =
            std::mem::transmute((*(ta as *const *const usize)).add(4).read_unaligned());
        assert_eq!(
            an_bidi(
                ta,
                &mut rsrc as *mut _ as *mut c_void,
                0,
                hebrew_digits.len() as u32,
                &mut bsink as *mut _ as *mut c_void,
            ),
            S_OK
        );
        assert_eq!(bsink.bidis, vec![(0, 2, 1, 1), (2, 2, 1, 2), (4, 1, 1, 1)]);

        // Devanagari ("אב कख ג") and Hangul ("אב 한글 ג") in RTL paragraphs must resolve to
        // [1, 1, 1, 2, 2, 1, 1] -> runs (0, 3, 1, 1), (3, 2, 1, 2), (5, 2, 1, 1).
        for sample in ["\u{05D0}\u{05D1} \u{0915}\u{0916} \u{05D2}", "\u{05D0}\u{05D1} \u{D55C}\u{AE00} \u{05D2}"] {
            let u16s: Vec<u16> = sample.encode_utf16().collect();
            rsrc.text = u16s.clone();
            bsink.bidis.clear();
            assert_eq!(
                an_bidi(
                    ta,
                    &mut rsrc as *mut _ as *mut c_void,
                    0,
                    u16s.len() as u32,
                    &mut bsink as *mut _ as *mut c_void,
                ),
                S_OK
            );
            assert_eq!(bsink.bidis, vec![(0, 3, 1, 1), (3, 2, 1, 2), (5, 2, 1, 1)], "sample={sample:?}");
        }

        // Explicit bidi controls (RLO, LRO, LRE, RLI, FSI, paragraph reset) in an LTR paragraph.
        unsafe extern "win64" fn ltr_get_dir(_this: *mut RtlSource) -> u32 {
            0 // DWRITE_READING_DIRECTION_LEFT_TO_RIGHT
        }
        let ltr_vtbl = [0, 0, 0, rtl_get_text as *const () as usize, 0, ltr_get_dir as *const () as usize, 0, 0];
        let mut lsrc = RtlSource { vtbl: &ltr_vtbl, text: Vec::new() };
        let expand_levels = |runs: &[(u32, u32, u8, u8)], total: usize| -> (Vec<u8>, Vec<u8>) {
            let mut exp = vec![0xFFu8; total];
            let mut res = vec![0xFFu8; total];
            for &(pos, len, e, r) in runs {
                for k in (pos as usize)..((pos + len) as usize) {
                    exp[k] = e;
                    res[k] = r;
                }
            }
            (exp, res)
        };

        // 4a. RLO override: "abc\u202Edef\u202C" -> "def" has explicit=1, resolved=1.
        let rlo_u16: Vec<u16> = "abc\u{202E}def\u{202C}".encode_utf16().collect();
        lsrc.text = rlo_u16.clone();
        bsink.bidis.clear();
        assert_eq!(
            an_bidi(ta, &mut lsrc as *mut _ as *mut c_void, 0, rlo_u16.len() as u32, &mut bsink as *mut _ as *mut c_void),
            S_OK
        );
        let (rlo_exp, rlo_res) = expand_levels(&bsink.bidis, rlo_u16.len());
        assert_eq!(rlo_exp, vec![0, 0, 0, 1, 1, 1, 1, 0]);
        assert_eq!(rlo_res, vec![0, 0, 0, 1, 1, 1, 1, 0]);

        // 4b. LRE embedding with Hebrew: "abc\u202Aאב\u202C" -> Hebrew has explicit=2, resolved=3.
        let lre_u16: Vec<u16> = "abc\u{202A}\u{05D0}\u{05D1}\u{202C}".encode_utf16().collect();
        lsrc.text = lre_u16.clone();
        bsink.bidis.clear();
        assert_eq!(
            an_bidi(ta, &mut lsrc as *mut _ as *mut c_void, 0, lre_u16.len() as u32, &mut bsink as *mut _ as *mut c_void),
            S_OK
        );
        let (lre_exp, lre_res) = expand_levels(&bsink.bidis, lre_u16.len());
        assert_eq!(lre_exp, vec![0, 0, 0, 2, 2, 2, 0]);
        assert_eq!(lre_res, vec![0, 0, 0, 3, 3, 3, 0]);

        // 4c. RLI isolate with Hebrew + digits: "hi \u2067אב 12\u2069 ok" -> אב at level 1, 12 at level 2.
        let rli_u16: Vec<u16> = "hi \u{2067}\u{05D0}\u{05D1} 12\u{2069} ok".encode_utf16().collect();
        lsrc.text = rli_u16.clone();
        bsink.bidis.clear();
        assert_eq!(
            an_bidi(ta, &mut lsrc as *mut _ as *mut c_void, 0, rli_u16.len() as u32, &mut bsink as *mut _ as *mut c_void),
            S_OK
        );
        let (rli_exp, rli_res) = expand_levels(&bsink.bidis, rli_u16.len());
        assert_eq!(rli_exp, vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0]);
        assert_eq!(rli_res, vec![0, 0, 0, 0, 1, 1, 1, 2, 2, 0, 0, 0, 0]);

        // 4d. FSI isolate auto-detecting RTL Hebrew + newline resetting RLO stack.
        let fsi_u16: Vec<u16> = "a\u{2068}\u{05D0}\u{05D1}\u{2069}b".encode_utf16().collect();
        lsrc.text = fsi_u16.clone();
        bsink.bidis.clear();
        assert_eq!(
            an_bidi(ta, &mut lsrc as *mut _ as *mut c_void, 0, fsi_u16.len() as u32, &mut bsink as *mut _ as *mut c_void),
            S_OK
        );
        let (fsi_exp, fsi_res) = expand_levels(&bsink.bidis, fsi_u16.len());
        assert_eq!(fsi_exp, vec![0, 0, 1, 1, 0, 0]);
        assert_eq!(fsi_res, vec![0, 0, 1, 1, 0, 0]);

        let nl_u16: Vec<u16> = "a\u{202E}bc\nde".encode_utf16().collect();
        lsrc.text = nl_u16.clone();
        bsink.bidis.clear();
        assert_eq!(
            an_bidi(ta, &mut lsrc as *mut _ as *mut c_void, 0, nl_u16.len() as u32, &mut bsink as *mut _ as *mut c_void),
            S_OK
        );
        let (nl_exp, nl_res) = expand_levels(&bsink.bidis, nl_u16.len());
        assert_eq!(nl_exp, vec![0, 1, 1, 1, 0, 0, 0]);
        assert_eq!(nl_res, vec![0, 1, 1, 1, 0, 0, 0]);

        // 5. Per-range OpenType features: "fifi" with range 0 (len 2) disabling `liga` and
        //    range 1 (len 2) enabling `liga` on a font with an `fi` ligature produces 2 glyphs
        //    for range 0 and 1 ligature glyph for range 1 (total 3 glyphs, clusterMap [0,1,2,2]).
        #[repr(C)]
        struct DWriteFontFeature {
            name_tag: u32,
            parameter: u32,
        }
        #[repr(C)]
        struct DWriteTypographicFeatures {
            features: *const DWriteFontFeature,
            feature_count: u32,
        }
        let liga_tag = u32::from_le_bytes(*b"liga");
        let f_off = DWriteFontFeature { name_tag: liga_tag, parameter: 0 };
        let f_on = DWriteFontFeature { name_tag: liga_tag, parameter: 1 };
        let set_off = DWriteTypographicFeatures { features: &f_off, feature_count: 1 };
        let set_on = DWriteTypographicFeatures { features: &f_on, feature_count: 1 };
        let feat_ptrs = [
            &set_off as *const _ as *const c_void,
            &set_on as *const _ as *const c_void,
        ];
        let feat_lens = [2u32, 2u32];

        let lig_meta = ["georgia", "times new roman", "helvetica", "palatino", "baskerville"]
            .iter()
            .find_map(|&fam_name| {
                let idx = sys.find_family_index(fam_name)?;
                let meta = sys.families[idx].fonts.iter().find(|m| m.file_path.is_some())?.clone();
                // Probe whether this font supports the standard `fi` ligature when `liga=1`.
                let probe_face = new_font_face(FontFaceInner::from_meta(meta.clone(), 0));
                let fi_u16 = [b'f' as u16, b'i' as u16];
                let mut cm = [0u16; 2];
                let mut tp = [DWRITE_SHAPING_TEXT_PROPERTIES::default(); 2];
                let mut gi = [0u16; 4];
                let mut gp = [DWRITE_SHAPING_GLYPH_PROPERTIES::default(); 4];
                let mut cnt = 0u32;
                let on_ptr = [&set_on as *const _ as *const c_void];
                let on_len = [2u32];
                let hr = ta_get_glyphs(
                    ta as PTextAnalyzer,
                    fi_u16.as_ptr(),
                    2,
                    probe_face,
                    FALSE,
                    FALSE,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    on_ptr.as_ptr(),
                    on_len.as_ptr(),
                    1,
                    4,
                    cm.as_mut_ptr(),
                    tp.as_mut_ptr(),
                    gi.as_mut_ptr(),
                    gp.as_mut_ptr(),
                    &mut cnt,
                );
                release_com(probe_face);
                (hr == S_OK && cnt == 1).then_some(meta)
            });

        if let Some(meta) = lig_meta {
            let lig_face = new_font_face(FontFaceInner::from_meta(meta, 0));
            let fifi_u16 = [b'f' as u16, b'i' as u16, b'f' as u16, b'i' as u16];
            let mut cm = [0xFFFFu16; 4];
            let mut tp = [DWRITE_SHAPING_TEXT_PROPERTIES::default(); 4];
            let mut gi = [0u16; 8];
            let mut gp = [DWRITE_SHAPING_GLYPH_PROPERTIES::default(); 8];
            let mut cnt = 0u32;
            assert_eq!(
                ta_get_glyphs(
                    ta as PTextAnalyzer,
                    fifi_u16.as_ptr(),
                    4,
                    lig_face,
                    FALSE,
                    FALSE,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    feat_ptrs.as_ptr(),
                    feat_lens.as_ptr(),
                    2,
                    8,
                    cm.as_mut_ptr(),
                    tp.as_mut_ptr(),
                    gi.as_mut_ptr(),
                    gp.as_mut_ptr(),
                    &mut cnt,
                ),
                S_OK
            );
            assert_eq!(cnt, 3, "range 0 (liga=0) -> 2 glyphs, range 1 (liga=1) -> 1 ligature glyph");
            assert_eq!(cm, [0, 1, 2, 2]);
            let mut adv = [0.0f32; 8];
            let mut offs = [DWRITE_GLYPH_OFFSET::default(); 8];
            assert_eq!(
                ta_get_glyph_placements(
                    ta as PTextAnalyzer,
                    fifi_u16.as_ptr(),
                    cm.as_ptr(),
                    tp.as_ptr(),
                    4,
                    gi.as_ptr(),
                    gp.as_ptr(),
                    cnt,
                    lig_face,
                    20.0f32.to_bits(),
                    FALSE,
                    FALSE,
                    std::ptr::null(),
                    std::ptr::null(),
                    feat_ptrs.as_ptr(),
                    feat_lens.as_ptr(),
                    2,
                    adv.as_mut_ptr(),
                    offs.as_mut_ptr(),
                ),
                S_OK
            );
            assert!(adv[0] > 0.0 && adv[1] > 0.0 && adv[2] > 0.0);
            release_com(lig_face);
        }

        release_com(ta);
        release_com(fac);
    }
}

#[test]
fn glyph_run_analysis_applies_sideways_rotation_and_vertical_metrics() {
    use super::analysis::{
        gra_create_alpha_texture, gra_get_alpha_texture_bounds, ta_get_glyph_placements,
        PGlyphRunAnalysis, PTextAnalyzer,
    };
    use crate::user32::RECT;
    unsafe {
        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &G_IDWRITE_FACTORY as *const _ as *const u8,
                &mut fac,
            ),
            S_OK
        );
        // Use a non-square glyph ('I' or built-in fallback 'H' / 'I') whose height != width
        // so 90-degree rotation is unmistakably observable in alpha-texture bounds.
        let sys = system_font_collection_data(false);
        let face_inner = sys.resolve_face(
            "Segoe UI",
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
        );
        let gid_i = face_inner.codepoint_to_glyph('I' as u32);
        let em_size = 40.0f32;
        let horiz_adv = face_inner.scaled_advance(gid_i, em_size);
        let vert_adv = face_inner.scaled_vertical_advance(gid_i, em_size);
        assert!(vert_adv > horiz_adv, "expected vertical advance > horizontal advance for 'I': h={horiz_adv}, v={vert_adv}");

        let face = new_font_face(face_inner);

        // Verify GetGlyphPlacements returns vertical advance when isSideways == TRUE.
        let mut ta: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_text_analyzer(fac as PFactory, &mut ta), S_OK);
        let text_i = ['I' as u16];
        let cmap = [0u16];
        let tprops = [DWRITE_SHAPING_TEXT_PROPERTIES::default()];
        let gids1 = [gid_i];
        let gprops = [DWRITE_SHAPING_GLYPH_PROPERTIES::default()];
        let mut placed_adv = [0.0f32; 1];
        let mut placed_off = [DWRITE_GLYPH_OFFSET::default(); 1];
        assert_eq!(
            ta_get_glyph_placements(
                ta as PTextAnalyzer,
                text_i.as_ptr(),
                cmap.as_ptr(),
                tprops.as_ptr(),
                1,
                gids1.as_ptr(),
                gprops.as_ptr(),
                1,
                face,
                em_size.to_bits(),
                TRUE, // isSideways
                FALSE,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                0,
                placed_adv.as_mut_ptr(),
                placed_off.as_mut_ptr(),
            ),
            S_OK
        );
        assert!(
            (placed_adv[0] - vert_adv).abs() < 1e-3,
            "sideways GetGlyphPlacements should return vertical advance {vert_adv}, got {}",
            placed_adv[0]
        );

        // 1. Baseline horizontal upright single glyph (isSideways = FALSE, identity).
        let run_horiz = DWRITE_GLYPH_RUN {
            fontFace: face,
            fontEmSize: em_size,
            glyphCount: 1,
            glyphIndices: gids1.as_ptr(),
            glyphAdvances: std::ptr::null(),
            glyphOffsets: std::ptr::null(),
            isSideways: FALSE,
            bidiLevel: 0,
        };
        let mut gra_h: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run_horiz,
                1.0,
                std::ptr::null(),
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                0.0,
                40.0,
                &mut gra_h,
            ),
            S_OK
        );
        let mut b_h = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_h as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut b_h), S_OK);
        let w_h = b_h.right - b_h.left;
        let h_h = b_h.bottom - b_h.top;
        assert!(h_h > w_h + 4, "upright 'I' should be taller than wide: {b_h:?}");

        // 2. Sideways run with identity transform: rotated 90° left (width <-> height swapped).
        let run_sw = DWRITE_GLYPH_RUN { isSideways: TRUE, ..run_horiz };
        let mut gra_sw: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run_sw,
                1.0,
                std::ptr::null(),
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                0.0,
                40.0,
                &mut gra_sw,
            ),
            S_OK
        );
        let mut b_sw = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_sw as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut b_sw), S_OK);
        let w_sw = b_sw.right - b_sw.left;
        let h_sw = b_sw.bottom - b_sw.top;
        assert!((w_sw - h_h).abs() <= 3, "sideways width {w_sw} should match upright height {h_h}");
        assert!((h_sw - w_h).abs() <= 3, "sideways height {h_sw} should match upright width {w_h}");

        // 3. Documented vertical writing: isSideways = TRUE + 90° CW compensating world transform
        //    (m11=0, m12=1, m21=-1, m22=0) restores upright orientation AND stacks 2 glyphs vertically.
        let rot_cw = DWRITE_MATRIX { m11: 0.0, m12: 1.0, m21: -1.0, m22: 0.0, dx: 0.0, dy: 0.0 };
        let mut gra_v1: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run_sw,
                1.0,
                &rot_cw,
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                10.0,
                -50.0, // maps under rot_cw to x = +50, y = +10
                &mut gra_v1,
            ),
            S_OK
        );
        let mut b_v1 = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_v1 as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut b_v1), S_OK);
        let w_v1 = b_v1.right - b_v1.left;
        let h_v1 = b_v1.bottom - b_v1.top;
        assert!((w_v1 - w_h).abs() <= 3, "compensated vertical width {w_v1} should match upright width {w_h}");
        assert!((h_v1 - h_h).abs() <= 3, "compensated vertical height {h_v1} should match upright height {h_h}");

        let gids2 = [gid_i, gid_i];
        let run_sw2 = DWRITE_GLYPH_RUN {
            glyphCount: 2,
            glyphIndices: gids2.as_ptr(),
            ..run_sw
        };
        let mut gra_v2: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_glyph_run_analysis(
                fac as PFactory,
                &run_sw2,
                1.0,
                &rot_cw,
                DWRITE_RENDERING_MODE_DEFAULT,
                DWRITE_MEASURING_MODE_NATURAL,
                10.0,
                -50.0,
                &mut gra_v2,
            ),
            S_OK
        );
        let mut b_v2 = RECT::default();
        assert_eq!(gra_get_alpha_texture_bounds(gra_v2 as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut b_v2), S_OK);
        assert_eq!(b_v2.left, b_v1.left, "vertical 2-glyph run must stay horizontally aligned");
        assert_eq!(b_v2.right, b_v1.right, "vertical 2-glyph run must stay horizontally aligned");
        assert!(
            b_v2.bottom >= b_v1.bottom + (vert_adv.round() as i32) - 2,
            "second vertical glyph must advance downward by vertical advance {vert_adv}: v1={b_v1:?}, v2={b_v2:?}"
        );
        // Ensure alpha texture actually renders non-zero pixels for the vertical run.
        let tex_w = (b_v2.right - b_v2.left) as usize;
        let tex_h = (b_v2.bottom - b_v2.top) as usize;
        let mut tex = vec![0u8; tex_w * tex_h * 3];
        assert_eq!(
            gra_create_alpha_texture(gra_v2 as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &b_v2, tex.as_mut_ptr(), tex.len() as u32),
            S_OK
        );
        assert!(tex.iter().any(|&a| a >= 128), "vertical alpha texture must contain lit glyph pixels");

        release_com(gra_v2);
        release_com(gra_v1);
        release_com(gra_sw);
        release_com(gra_h);
        release_com(ta);
        release_com(face);
        release_com(fac);
    }
}

#[test]
fn text_analyzer_honors_number_substitution_for_localized_digits() {
    use super::analysis::{
        ta_get_gdi_compatible_glyph_placements, ta_get_glyph_placements, ta_get_glyphs,
        PTextAnalyzer,
    };
    use super::font::{
        DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL,
        DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL,
        DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE,
    };
    unsafe {
        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &G_IDWRITE_FACTORY as *const _ as *const u8,
                &mut fac,
            ),
            S_OK
        );
        let mut ta: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_text_analyzer(fac as PFactory, &mut ta), S_OK);

        // Find a system font that has both ASCII '0'..'9' and Arabic-Indic '٠'..'٩' (U+0660..U+0669).
        let sys = system_font_collection_data(false);
        let font_meta = ["geeza pro", "arial", "times new roman", "damascus"]
            .iter()
            .find_map(|&fam_name| {
                let idx = sys.find_family_index(fam_name)?;
                sys.families[idx]
                    .fonts
                    .iter()
                    .find(|m| {
                        if m.file_path.is_none() {
                            return false;
                        }
                        let fi = FontFaceInner::from_meta((*m).clone(), 0);
                        fi.codepoint_to_glyph('1' as u32) != 0
                            && fi.codepoint_to_glyph(0x0661) != 0
                            && fi.codepoint_to_glyph('1' as u32) != fi.codepoint_to_glyph(0x0661)
                    })
                    .cloned()
            })
            .expect("expected a macOS system font with distinct Latin and Arabic-Indic digits");

        let face_inner = FontFaceInner::from_meta(font_meta, 0);
        let latin_123 = [
            face_inner.codepoint_to_glyph('1' as u32),
            face_inner.codepoint_to_glyph('2' as u32),
            face_inner.codepoint_to_glyph('3' as u32),
        ];
        let arabic_123 = [
            face_inner.codepoint_to_glyph(0x0661),
            face_inner.codepoint_to_glyph(0x0662),
            face_inner.codepoint_to_glyph(0x0663),
        ];
        assert_ne!(latin_123, arabic_123);
        let face = new_font_face(face_inner);

        let ar_sa = wide("ar-SA");
        let mut ns_national: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_number_substitution(
                fac as PFactory,
                DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL,
                ar_sa.as_ptr(),
                TRUE,
                &mut ns_national,
            ),
            S_OK
        );
        let mut ns_none: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_number_substitution(
                fac as PFactory,
                DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE,
                ar_sa.as_ptr(),
                TRUE,
                &mut ns_none,
            ),
            S_OK
        );
        let mut ns_ctx: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_number_substitution(
                fac as PFactory,
                DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL,
                ar_sa.as_ptr(),
                TRUE,
                &mut ns_ctx,
            ),
            S_OK
        );

        let shape_with_ns = |text: &str, rtl: BOOL, ns: *mut c_void| -> (HRESULT, Vec<u16>) {
            let u16s: Vec<u16> = text.encode_utf16().collect();
            let mut cm = vec![0u16; u16s.len()];
            let mut tp = vec![DWRITE_SHAPING_TEXT_PROPERTIES::default(); u16s.len()];
            let mut gi = vec![0u16; u16s.len() * 2 + 4];
            let mut gp = vec![DWRITE_SHAPING_GLYPH_PROPERTIES::default(); gi.len()];
            let mut actual = 0u32;
            let hr = ta_get_glyphs(
                ta as PTextAnalyzer,
                u16s.as_ptr(),
                u16s.len() as u32,
                face,
                FALSE,
                rtl,
                std::ptr::null(),
                std::ptr::null(),
                ns,
                std::ptr::null(),
                std::ptr::null(),
                0,
                gi.len() as u32,
                cm.as_mut_ptr(),
                tp.as_mut_ptr(),
                gi.as_mut_ptr(),
                gp.as_mut_ptr(),
                &mut actual,
            );
            (hr, gi[..actual as usize].to_vec())
        };

        // 1. NATIONAL ar-SA converts ASCII "123" to Arabic-Indic glyphs.
        let (hr_nat, g_nat) = shape_with_ns("123", FALSE, ns_national);
        assert_eq!(hr_nat, S_OK);
        assert_eq!(g_nat, arabic_123, "NATIONAL ar-SA must produce Arabic-Indic digit glyphs");

        // 2. NONE ar-SA keeps nominal Latin digits.
        let (hr_none, g_none) = shape_with_ns("123", FALSE, ns_none);
        assert_eq!(hr_none, S_OK);
        assert_eq!(g_none, latin_123, "NONE must keep nominal Latin digit glyphs");

        // 3. CONTEXTUAL ar-SA: keeps Latin digits after Latin letters, substitutes after Arabic letters.
        let (hr_ctx_ltr, g_ctx_ltr) = shape_with_ns("A123", FALSE, ns_ctx);
        assert_eq!(hr_ctx_ltr, S_OK);
        assert_eq!(&g_ctx_ltr[1..], &latin_123, "CONTEXTUAL after Latin must keep Latin digits");

        let (hr_ctx_ar, g_ctx_ar) = shape_with_ns("\u{0633}123", TRUE, ns_ctx);
        assert_eq!(hr_ctx_ar, S_OK);
        assert!(
            arabic_123.iter().all(|g| g_ctx_ar.contains(g)),
            "CONTEXTUAL after Arabic must substitute Arabic-Indic digits: got={g_ctx_ar:?}, expected={arabic_123:?}"
        );

        // 4. GetGlyphPlacements and GetGdiCompatibleGlyphPlacements must preserve CoreText
        //    shaped positioning (combining-mark offsets / zero-advance marks) when GetGlyphs
        //    substituted localized digits and placement is called with the original text.
        let orig_ar_mark = "\u{0633}\u{064E}12"; // Seen + Fatha + ASCII "12"
        let pre_sub_ar_mark = "\u{0633}\u{064E}\u{0661}\u{0662}"; // Seen + Fatha + Arabic-Indic "١٢"
        let (hr_m_sub, g_m_sub) = shape_with_ns(orig_ar_mark, TRUE, ns_national);
        let (hr_m_ref, g_m_ref) = shape_with_ns(pre_sub_ar_mark, TRUE, std::ptr::null_mut());
        assert_eq!(hr_m_sub, S_OK);
        assert_eq!(hr_m_ref, S_OK);
        assert_eq!(g_m_sub, g_m_ref);

        let place_run = |text: &str, gids: &[u16], gdi: bool| -> (Vec<f32>, Vec<DWRITE_GLYPH_OFFSET>) {
            let u16s: Vec<u16> = text.encode_utf16().collect();
            let cm = vec![0u16; u16s.len()];
            let tp = vec![DWRITE_SHAPING_TEXT_PROPERTIES::default(); u16s.len()];
            let gp = vec![DWRITE_SHAPING_GLYPH_PROPERTIES::default(); gids.len()];
            let mut adv = vec![0.0f32; gids.len()];
            let mut off = vec![DWRITE_GLYPH_OFFSET::default(); gids.len()];
            let hr = if gdi {
                ta_get_gdi_compatible_glyph_placements(
                    ta as PTextAnalyzer,
                    u16s.as_ptr(),
                    cm.as_ptr(),
                    tp.as_ptr(),
                    u16s.len() as u32,
                    gids.as_ptr(),
                    gp.as_ptr(),
                    gids.len() as u32,
                    face,
                    24.0f32.to_bits(),
                    1.0f32.to_bits(),
                    std::ptr::null(),
                    TRUE,
                    FALSE,
                    TRUE,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    adv.as_mut_ptr(),
                    off.as_mut_ptr(),
                )
            } else {
                ta_get_glyph_placements(
                    ta as PTextAnalyzer,
                    u16s.as_ptr(),
                    cm.as_ptr(),
                    tp.as_ptr(),
                    u16s.len() as u32,
                    gids.as_ptr(),
                    gp.as_ptr(),
                    gids.len() as u32,
                    face,
                    24.0f32.to_bits(),
                    FALSE,
                    TRUE,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    adv.as_mut_ptr(),
                    off.as_mut_ptr(),
                )
            };
            assert_eq!(hr, S_OK);
            (adv, off)
        };
        let (adv_sub, off_sub) = place_run(orig_ar_mark, &g_m_sub, false);
        let (adv_ref, off_ref) = place_run(pre_sub_ar_mark, &g_m_ref, false);
        assert_eq!(adv_sub, adv_ref, "GetGlyphPlacements must preserve shaped advances after digit substitution");
        for (a, b) in off_sub.iter().zip(off_ref.iter()) {
            assert_eq!((a.advanceOffset, a.ascenderOffset), (b.advanceOffset, b.ascenderOffset));
        }
        let (gadv_sub, goff_sub) = place_run(orig_ar_mark, &g_m_sub, true);
        let (gadv_ref, goff_ref) = place_run(pre_sub_ar_mark, &g_m_ref, true);
        assert_eq!(gadv_sub, gadv_ref, "GetGdiCompatibleGlyphPlacements must preserve shaped advances");
        for (a, b) in goff_sub.iter().zip(goff_ref.iter()) {
            assert_eq!((a.advanceOffset, a.ascenderOffset), (b.advanceOffset, b.ascenderOffset));
        }

        // 5. Unrecognized/bogus non-null number_substitution pointer must fail instead of silently ignoring.
        let mut fake_vtbl = [0usize; 3];
        let mut fake_obj = fake_vtbl.as_mut_ptr();
        let (hr_bad, _) = shape_with_ns("123", FALSE, &mut fake_obj as *mut _ as *mut c_void);
        assert_eq!(hr_bad, E_INVALIDARG);

        // 6. AnalyzeLineBreakpoints must report mandatory breaks (MUST_BREAK + isWhitespace)
        //    for Unicode line/paragraph separators U+2028, U+2029, VT, FF, and NEL.
        #[repr(C)]
        struct BpSource {
            vtbl: *const [usize; 8],
            text: Vec<u16>,
        }
        #[repr(C)]
        struct BpSink {
            vtbl: *const [usize; 7],
            bps: Vec<u8>,
        }
        unsafe extern "win64" fn bp_get_text(this: *mut BpSource, pos: u32, out_str: *mut *const u16, out_len: *mut u32) -> HRESULT {
            let p = pos as usize;
            if p >= (*this).text.len() {
                out_str.write_unaligned(std::ptr::null());
                out_len.write_unaligned(0);
            } else {
                out_str.write_unaligned((*this).text.as_ptr().add(p));
                out_len.write_unaligned(((*this).text.len() - p) as u32);
            }
            S_OK
        }
        unsafe extern "win64" fn bp_set_bps(this: *mut BpSink, _pos: u32, len: u32, bps: *const DWRITE_LINE_BREAKPOINT) -> HRESULT {
            for i in 0..len as usize {
                (*this).bps.push(bps.add(i).read_unaligned().bits);
            }
            S_OK
        }
        let bp_src_vtbl = [0, 0, 0, bp_get_text as *const () as usize, 0, 0, 0, 0];
        let bp_snk_vtbl = [0, 0, 0, 0, bp_set_bps as *const () as usize, 0, 0];
        let sample_u16: Vec<u16> = "a\u{2028}b\u{2029}c\u{000B}d\u{000C}e\u{0085}f".encode_utf16().collect();
        let mut bp_src = BpSource { vtbl: &bp_src_vtbl, text: sample_u16.clone() };
        let mut bp_snk = BpSink { vtbl: &bp_snk_vtbl, bps: Vec::new() };
        let an_lb_fn: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, *mut c_void) -> HRESULT =
            std::mem::transmute((*(ta as *const *const usize)).add(6).read_unaligned());
        assert_eq!(
            an_lb_fn(ta, &mut bp_src as *mut _ as *mut c_void, 0, sample_u16.len() as u32, &mut bp_snk as *mut _ as *mut c_void),
            S_OK
        );
        assert_eq!(bp_snk.bps.len(), sample_u16.len());
        for &sep_idx in &[1usize, 3, 5, 7, 9] {
            let bits = bp_snk.bps[sep_idx];
            let before = bits & 0x3;
            let after = (bits >> 2) & 0x3;
            let is_ws = (bits >> 4) & 0x1;
            assert_eq!((before, after, is_ws), (2, 3, 1), "mandatory separator at idx {sep_idx} (u={:#x}) had bits={bits:#x}", sample_u16[sep_idx]);
        }

        // 7. Word-internal boundaries must be marked MAY_NOT_BREAK (2), while space/hyphen/CJK
        //    boundaries allow breaking (CAN_BREAK = 1) and CR+LF breaks only after LF (MUST_BREAK = 3).
        let analyze_bps = |s: &str| -> Vec<(u8, u8, u8, u8)> {
            let u16s: Vec<u16> = s.encode_utf16().collect();
            let mut src = BpSource { vtbl: &bp_src_vtbl, text: u16s.clone() };
            let mut snk = BpSink { vtbl: &bp_snk_vtbl, bps: Vec::new() };
            assert_eq!(
                an_lb_fn(ta, &mut src as *mut _ as *mut c_void, 0, u16s.len() as u32, &mut snk as *mut _ as *mut c_void),
                S_OK
            );
            snk.bps
                .into_iter()
                .map(|b| (b & 0x3, (b >> 2) & 0x3, (b >> 4) & 0x1, (b >> 5) & 0x1))
                .collect()
        };
        let hello_bps = analyze_bps("a hello");
        assert_eq!(hello_bps.len(), 7);
        // 'a' -> ' ' prohibited; ' ' -> 'h' allowed; all boundaries inside "hello" prohibited.
        assert_eq!(hello_bps[0].1, 2, "must not break between 'a' and space: {hello_bps:?}");
        assert_eq!((hello_bps[1].0, hello_bps[1].1, hello_bps[1].2), (2, 1, 1), "space must allow break after: {hello_bps:?}");
        assert_eq!(hello_bps[2].0, 1, "'h' after space allows break before: {hello_bps:?}");
        for idx in 2..6 {
            assert_eq!(hello_bps[idx].1, 2, "word-internal break after idx {idx} in 'hello' must be MAY_NOT_BREAK: {hello_bps:?}");
            assert_eq!(hello_bps[idx + 1].0, 2, "word-internal break before idx {} in 'hello' must be MAY_NOT_BREAK: {hello_bps:?}", idx + 1);
        }

        let hyp_nbsp = analyze_bps("foo-bar\u{00A0}baz");
        // Break allowed after '-' (idx 3 -> 4), prohibited around NBSP (idx 7) and inside words.
        assert_eq!((hyp_nbsp[3].1, hyp_nbsp[4].0), (1, 1), "hyphen allows break after: {hyp_nbsp:?}");
        assert_eq!((hyp_nbsp[6].1, hyp_nbsp[7].0, hyp_nbsp[7].1, hyp_nbsp[8].0), (2, 2, 2, 2), "NBSP prohibits break before and after: {hyp_nbsp:?}");
        assert_eq!((hyp_nbsp[0].1, hyp_nbsp[1].0), (2, 2), "word-internal 'fo' must prohibit break: {hyp_nbsp:?}");

        let crlf_bps = analyze_bps("a\r\nb");
        assert_eq!((crlf_bps[1].1, crlf_bps[2].0, crlf_bps[2].1), (2, 2, 3), "CR+LF must not break between CR and LF and must break after LF: {crlf_bps:?}");

        let cjk_bps = analyze_bps("日本語");
        assert_eq!((cjk_bps[0].1, cjk_bps[1].0, cjk_bps[1].1, cjk_bps[2].0), (1, 1, 1, 1), "CJK ideographs allow breaks between chars: {cjk_bps:?}");

        release_com(ns_ctx);
        release_com(ns_none);
        release_com(ns_national);
        release_com(face);
        release_com(ta);
        release_com(fac);
    }
}

#[test]
fn glyph_run_analysis_preserves_fractional_subpixel_origins() {
    use super::analysis::{gra_create_alpha_texture, gra_get_alpha_texture_bounds, PGlyphRunAnalysis};
    use super::sfnt::{fallback_cp_to_gid, parse_font_bytes_metadata, SfntFace};
    use crate::user32::RECT;

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

            // Build a TrueType contour font face (gid 1 = solid rectangle outline).
            let ttf = build_synthetic_ttf();
            let raw = Arc::new(ttf);
            let metas = parse_font_bytes_metadata(Arc::clone(&raw), "SubpixelTest");
            assert!(!metas.is_empty());
            let sfnt = SfntFace::from_bytes(Arc::clone(&raw), 0).map(Arc::new);
            let tt_face = new_font_face(Arc::new(FontFaceInner {
                meta: Arc::new(metas[0].clone()),
                simulations: 0,
                sfnt,
            }));

            let render_centroid = |face_ptr: *mut c_void, mode: u32, tex_ty: u32, bx: f32, by: f32| -> (RECT, Vec<u8>, f64, f64) {
                let gids = [1u16];
                let adv = [20.0f32];
                let offs = [DWRITE_GLYPH_OFFSET::default()];
                let run = DWRITE_GLYPH_RUN {
                    fontFace: face_ptr,
                    fontEmSize: 32.0,
                    glyphCount: 1,
                    glyphIndices: gids.as_ptr(),
                    glyphAdvances: adv.as_ptr(),
                    glyphOffsets: offs.as_ptr(),
                    isSideways: FALSE,
                    bidiLevel: 0,
                };
                let mut gra: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    fac_create_glyph_run_analysis(
                        fac as PFactory,
                        &run,
                        1.0,
                        std::ptr::null(),
                        mode,
                        DWRITE_MEASURING_MODE_NATURAL,
                        bx,
                        by,
                        &mut gra,
                    ),
                    S_OK
                );
                let mut rc = RECT::default();
                assert_eq!(gra_get_alpha_texture_bounds(gra as PGlyphRunAnalysis, tex_ty, &mut rc), S_OK);
                let w = (rc.right - rc.left) as usize;
                let h = (rc.bottom - rc.top) as usize;
                let bpp = if tex_ty == DWRITE_TEXTURE_ALIASED_1X1 { 1usize } else { 3usize };
                let mut buf = vec![0u8; w * h * bpp];
                assert_eq!(
                    gra_create_alpha_texture(gra as PGlyphRunAnalysis, tex_ty, &rc, buf.as_mut_ptr(), buf.len() as u32),
                    S_OK
                );
                release_com(gra);

                let mut sum = 0.0f64;
                let mut sum_x = 0.0f64;
                let mut sum_y = 0.0f64;
                for y in 0..h {
                    for x in 0..w {
                        let cov = buf[(y * w + x) * bpp] as f64;
                        sum += cov;
                        sum_x += cov * (rc.left as f64 + x as f64 + 0.5);
                        sum_y += cov * (rc.top as f64 + y as f64 + 0.5);
                    }
                }
                assert!(sum > 0.0, "expected non-empty coverage for mode={mode}, bx={bx}, by={by}");
                (rc, buf, sum_x / sum, sum_y / sum)
            };

            // 1. Natural mode preserves fractional X origins: 0.0 -> 0.4 -> 0.6 shifts coverage smoothly,
            //    never producing identical coverage for 0.0 and 0.4.
            let (_, tex_0, cx_0, cy_0) = render_centroid(tt_face, DWRITE_RENDERING_MODE_NATURAL, DWRITE_TEXTURE_CLEARTYPE_3X1, 0.0, 32.0);
            let (_, tex_04, cx_04, _) = render_centroid(tt_face, DWRITE_RENDERING_MODE_NATURAL, DWRITE_TEXTURE_CLEARTYPE_3X1, 0.4, 32.0);
            let (_, _, cx_06, _) = render_centroid(tt_face, DWRITE_RENDERING_MODE_NATURAL, DWRITE_TEXTURE_CLEARTYPE_3X1, 0.6, 32.0);
            assert_ne!(tex_0, tex_04, "NATURAL mode must produce distinct coverage at baseline_x 0.0 vs 0.4");
            assert!(((cx_04 - cx_0) - 0.4).abs() < 0.06, "expected +0.4px X shift: cx_0={cx_0}, cx_04={cx_04}");
            assert!(((cx_06 - cx_0) - 0.6).abs() < 0.06, "expected +0.6px X shift: cx_0={cx_0}, cx_06={cx_06}");

            // NATURAL snaps vertical baseline to whole pixel; NATURAL_SYMMETRIC preserves fractional Y too.
            let (_, _, _, cy_nat_04) = render_centroid(tt_face, DWRITE_RENDERING_MODE_NATURAL, DWRITE_TEXTURE_CLEARTYPE_3X1, 0.0, 32.4);
            assert!((cy_nat_04 - cy_0).abs() < 1e-4, "NATURAL mode snaps baseline Y: cy_0={cy_0}, cy_nat_04={cy_nat_04}");
            let (_, _, _, cy_sym_04) = render_centroid(tt_face, DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC, DWRITE_TEXTURE_CLEARTYPE_3X1, 0.0, 32.4);
            assert!(((cy_sym_04 - cy_0) - 0.4).abs() < 0.15, "NATURAL_SYMMETRIC must preserve fractional Y: cy_0={cy_0}, cy_sym_04={cy_sym_04}");

            // 2. ALIASED and GDI_CLASSIC snap origins to whole pixels (0.0 and 0.4 match).
            let (rc_al_0, tex_al_0, _, _) = render_centroid(tt_face, DWRITE_RENDERING_MODE_ALIASED, DWRITE_TEXTURE_ALIASED_1X1, 0.0, 32.0);
            let (rc_al_04, tex_al_04, _, _) = render_centroid(tt_face, DWRITE_RENDERING_MODE_ALIASED, DWRITE_TEXTURE_ALIASED_1X1, 0.4, 32.0);
            assert_eq!((rc_al_0.left, rc_al_0.top, rc_al_0.right, rc_al_0.bottom), (rc_al_04.left, rc_al_04.top, rc_al_04.right, rc_al_04.bottom));
            assert_eq!(tex_al_0, tex_al_04, "ALIASED mode must snap 0.0 and 0.4 to the same whole pixel");

            let (rc_gdi_0, tex_gdi_0, _, _) = render_centroid(tt_face, DWRITE_RENDERING_MODE_GDI_CLASSIC, DWRITE_TEXTURE_CLEARTYPE_3X1, 0.0, 32.0);
            let (rc_gdi_04, tex_gdi_04, _, _) = render_centroid(tt_face, DWRITE_RENDERING_MODE_GDI_CLASSIC, DWRITE_TEXTURE_CLEARTYPE_3X1, 0.4, 32.0);
            assert_eq!((rc_gdi_0.left, rc_gdi_0.top, rc_gdi_0.right, rc_gdi_0.bottom), (rc_gdi_04.left, rc_gdi_04.top, rc_gdi_04.right, rc_gdi_04.bottom));
            assert_eq!(tex_gdi_0, tex_gdi_04, "GDI_CLASSIC mode must snap 0.0 and 0.4 to the same whole pixel");

            // 3. Fallback 8x8 bitmap rasterizer also preserves fractional X origins under NATURAL mode.
            let fb_face = new_font_face(Arc::new(FontFaceInner {
                meta: Arc::new(metas[0].clone()),
                simulations: 0,
                sfnt: None,
            }));
            let gids_h = [fallback_cp_to_gid('H' as u32)];
            let adv_h = [20.0f32];
            let offs_h = [DWRITE_GLYPH_OFFSET::default()];
            let render_fb = |bx: f32| -> (Vec<u8>, f64) {
                let run = DWRITE_GLYPH_RUN {
                    fontFace: fb_face,
                    fontEmSize: 32.0,
                    glyphCount: 1,
                    glyphIndices: gids_h.as_ptr(),
                    glyphAdvances: adv_h.as_ptr(),
                    glyphOffsets: offs_h.as_ptr(),
                    isSideways: FALSE,
                    bidiLevel: 0,
                };
                let mut gra: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    fac_create_glyph_run_analysis(
                        fac as PFactory,
                        &run,
                        1.0,
                        std::ptr::null(),
                        DWRITE_RENDERING_MODE_NATURAL,
                        DWRITE_MEASURING_MODE_NATURAL,
                        bx,
                        32.0,
                        &mut gra,
                    ),
                    S_OK
                );
                let mut rc = RECT::default();
                assert_eq!(gra_get_alpha_texture_bounds(gra as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &mut rc), S_OK);
                let w = (rc.right - rc.left) as usize;
                let h = (rc.bottom - rc.top) as usize;
                let mut buf = vec![0u8; w * h * 3];
                assert_eq!(
                    gra_create_alpha_texture(gra as PGlyphRunAnalysis, DWRITE_TEXTURE_CLEARTYPE_3X1, &rc, buf.as_mut_ptr(), buf.len() as u32),
                    S_OK
                );
                release_com(gra);
                let mut sum = 0.0f64;
                let mut sum_x = 0.0f64;
                for y in 0..h {
                    for x in 0..w {
                        let cov = buf[(y * w + x) * 3] as f64;
                        sum += cov;
                        sum_x += cov * (rc.left as f64 + x as f64 + 0.5);
                    }
                }
                (buf, sum_x / sum)
            };
            let (fb_tex_0, fb_cx_0) = render_fb(0.0);
            let (fb_tex_04, fb_cx_04) = render_fb(0.4);
            assert_ne!(fb_tex_0, fb_tex_04, "fallback 8x8 glyph must differ at baseline_x 0.0 vs 0.4");
            assert!(((fb_cx_04 - fb_cx_0) - 0.4).abs() < 0.2, "fallback X shift expected ~0.4: 0={fb_cx_0}, 0.4={fb_cx_04}");

            release_com(fb_face);
            release_com(tt_face);
            release_com(fac);
    }
}
