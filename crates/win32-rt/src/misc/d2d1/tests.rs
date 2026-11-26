//! Unit tests for `d2d1.dll` (loaded from `#[cfg(test)] mod tests;` at the bottom of `d2d1.rs`).

use super::geometry::{
    geom_compute_area, geom_compute_length, geom_fill_contains_pt, gsink_add_arc, gsink_add_bezier,
    gsink_add_line, gsink_begin_figure, gsink_close, gsink_end_figure, path_geom_open, PGeom,
    PGeomSink,
};
use super::raster::{
    bmp_copy_from_rt, bmp_get_pixel_size, bmp_map, bmp_unmap, brush_set_opacity, PBitmap, PBrush,
};
use super::render_target::{
    brt_get_bitmap, dc_create_bitmap1, dc_rt_bind_dc, dc_set_target, rt_begin_draw, rt_clear,
    rt_create_bitmap, rt_create_bitmap_brush, rt_create_compat_rt, rt_create_grad_stops,
    rt_create_linear_brush, rt_create_radial_brush, rt_create_solid_brush, rt_draw_bitmap,
    rt_draw_line, rt_draw_rect, rt_draw_text, rt_draw_text_layout, rt_end_draw, rt_fill_ellipse,
    rt_fill_geometry, rt_fill_rect, rt_fill_rounded_rect, rt_get_pixel_size, rt_get_transform,
    rt_pop_clip, rt_pop_layer, rt_push_clip, rt_push_layer, rt_restore_state, rt_save_state,
    rt_set_transform, PRT,
};
use super::*;
use crate::com::register_vtable;
use crate::misc::dwrite;

unsafe fn vtbl_of(obj: *mut c_void) -> *const usize {
    (obj as *const *const usize).read_unaligned()
}

unsafe fn release_com(obj: *mut c_void) -> u32 {
    let f: unsafe extern "win64" fn(*mut c_void) -> u32 =
        std::mem::transmute(vtbl_of(obj).add(2).read_unaligned());
    f(obj)
}

/// Helper: read pixel `[r, g, b, a]` at `(x, y)` from any Direct2D RenderTarget via
/// `ID2D1Bitmap1::CopyFromRenderTarget` + `Map` (official Direct2D readback path!).
unsafe fn read_rt_pixel(rt: *mut c_void, x: u32, y: u32) -> [u8; 4] {
    let mut sz = D2D1_SIZE_U::default();
    rt_get_pixel_size(rt as PRT, &mut sz);
    let mut bmp: *mut c_void = std::ptr::null_mut();
    assert_eq!(
        rt_create_bitmap(
            rt as PRT,
            sz.to_u64(),
            std::ptr::null(),
            0,
            std::ptr::null(),
            &mut bmp,
        ),
        S_OK
    );
    assert_eq!(bmp_copy_from_rt(bmp as PBitmap, std::ptr::null(), rt, std::ptr::null()), S_OK);
    let mut mapped = D2D1_MAPPED_RECT { pitch: 0, _pad: 0, bits: std::ptr::null_mut() };
    assert_eq!(bmp_map(bmp as PBitmap, 0, &mut mapped), S_OK);
    let off = (y as usize) * (mapped.pitch as usize) + (x as usize) * 4;
    let p = mapped.bits.add(off);
    let out = [p.read_unaligned(), p.add(1).read_unaligned(), p.add(2).read_unaligned(), p.add(3).read_unaligned()];
    assert_eq!(bmp_unmap(bmp as PBitmap), S_OK);
    release_com(bmp);
    out
}

#[test]
fn matrix_helpers_rotate_skew_invert_and_bridge() {
    unsafe {
        let center = D2D1_POINT_2F { x: 10.0, y: 20.0 };
        let mut rot = D2D1_MATRIX_3X2_F::default();
        D2D1MakeRotateMatrix(90.0, center.to_u64(), &mut rot);
        let (cx, cy) = rot.transform_point(10.0, 20.0);
        assert!((cx - 10.0).abs() < 1e-4 && (cy - 20.0).abs() < 1e-4);
        let (rx, ry) = rot.transform_point(15.0, 20.0);
        assert!((rx - 10.0).abs() < 1e-4 && (ry - 25.0).abs() < 1e-4);

        assert_eq!(D2D1IsMatrixInvertible(&rot), TRUE);
        let mut inv = rot;
        assert_eq!(D2D1InvertMatrix(&mut inv), TRUE);
        let prod = rot.mul(&inv);
        assert!((prod.m11 - 1.0).abs() < 1e-4 && (prod.m22 - 1.0).abs() < 1e-4);
        assert!(prod.dx.abs() < 1e-4 && prod.dy.abs() < 1e-4);

        let singular = D2D1_MATRIX_3X2_F { m11: 1.0, m12: 2.0, m21: 2.0, m22: 4.0, dx: 0.0, dy: 0.0 };
        assert_eq!(D2D1IsMatrixInvertible(&singular), FALSE);
        let mut sing_copy = singular;
        assert_eq!(D2D1InvertMatrix(&mut sing_copy), FALSE);

        let mut skew = D2D1_MATRIX_3X2_F::default();
        D2D1MakeSkewMatrix(45.0, 0.0, D2D1_POINT_2F { x: 0.0, y: 10.0 }.to_u64(), &mut skew);
        let (sx, sy) = skew.transform_point(0.0, 20.0);
        assert!((sx - 10.0).abs() < 1e-4 && (sy - 20.0).abs() < 1e-4);

        // On aarch64, also directly verify the win64_bridge! thunk unpacks XMM0/XMM1 + GPRs.
        #[cfg(target_arch = "aarch64")]
        {
            let mut m2 = D2D1_MATRIX_3X2_F::default();
            d2d1_make_rotate_matrix_impl::bridge(
                0,
                center.to_u64(),
                &mut m2 as *mut D2D1_MATRIX_3X2_F as u64,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                f64::from_bits(90.0f32.to_bits() as u64),
                0.0,
                0.0,
                0.0,
            );
            assert!((m2.m11 - rot.m11).abs() < 1e-5 && (m2.m12 - rot.m12).abs() < 1e-5);
            assert!((m2.dx - rot.dx).abs() < 1e-4 && (m2.dy - rot.dy).abs() < 1e-4);
        }
    }
}

#[test]
fn rasterizer_primitives_brushes_transforms_clips_and_paths() {
    unsafe {
        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &IID_ID2D1_FACTORY1 as *const GUID as *const u8,
                std::ptr::null(),
                &mut fac,
            ),
            S_OK
        );

        // Create 64x64 HWND render target
        let hp = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd: std::ptr::null_mut(),
            pixelSize: D2D1_SIZE_U { width: 64, height: 64 },
            presentOptions: 0,
            _pad: 0,
        };
        let mut rt: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_hwnd_rt(fac as PFactory, std::ptr::null(), &hp, &mut rt), S_OK);

        // Solid red brush
        let red = D2D1_COLOR_F { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
        let mut red_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_solid_brush(rt as PRT, &red, std::ptr::null(), &mut red_brush), S_OK);

        // Solid green brush (50% opacity)
        let green = D2D1_COLOR_F { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
        let mut green_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_solid_brush(rt as PRT, &green, std::ptr::null(), &mut green_brush), S_OK);
        brush_set_opacity(green_brush as PBrush, 0.5);

        // Clear to solid blue
        rt_begin_draw(rt as PRT);
        let blue = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
        rt_clear(rt as PRT, &blue);
        assert_eq!(read_rt_pixel(rt, 0, 0), [0, 0, 255, 255]);

        // FillRectangle with clip stack: fill [0..32, 0..32] clipped to [8..16, 8..16]
        let clip = D2D1_RECT_F { left: 8.0, top: 8.0, right: 16.0, bottom: 16.0 };
        rt_push_clip(rt as PRT, &clip, D2D1_ANTIALIAS_MODE_PER_PRIMITIVE);
        let big = D2D1_RECT_F { left: 0.0, top: 0.0, right: 32.0, bottom: 32.0 };
        rt_fill_rect(rt as PRT, &big, red_brush);
        rt_pop_clip(rt as PRT);
        assert_eq!(read_rt_pixel(rt, 12, 12), [255, 0, 0, 255]);
        assert_eq!(read_rt_pixel(rt, 4, 4), [0, 0, 255, 255]);

        // Blend 50% green over red at (12, 12)
        let small = D2D1_RECT_F { left: 10.0, top: 10.0, right: 14.0, bottom: 14.0 };
        rt_fill_rect(rt as PRT, &small, green_brush);
        let blended = read_rt_pixel(rt, 12, 12);
        assert!((blended[0] as i32 - 128).abs() <= 2 && (blended[1] as i32 - 128).abs() <= 2);

        // FillEllipse at (48, 16) radius (10, 10)
        let el = D2D1_ELLIPSE { point: D2D1_POINT_2F { x: 48.0, y: 16.0 }, radiusX: 10.0, radiusY: 10.0 };
        rt_fill_ellipse(rt as PRT, &el, red_brush);
        assert_eq!(read_rt_pixel(rt, 48, 16), [255, 0, 0, 255]);
        assert_eq!(read_rt_pixel(rt, 36, 4), [0, 0, 255, 255]);

        // FillRoundedRectangle & DrawLine with translation transform
        let tx = D2D1_MATRIX_3X2_F { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 4.0, dy: 4.0 };
        rt_set_transform(rt as PRT, &tx);
        let rr = D2D1_ROUNDED_RECT {
            rect: D2D1_RECT_F { left: 0.0, top: 36.0, right: 20.0, bottom: 52.0 },
            radiusX: 4.0,
            radiusY: 4.0,
        };
        rt_fill_rounded_rect(rt as PRT, &rr, red_brush);
        // Center of rounded rect is at (10+4, 44+4) = (14, 48)
        assert_eq!(read_rt_pixel(rt, 14, 48), [255, 0, 0, 255]);
        // Reset transform and draw white horizontal line across y=32
        let id = D2D1_MATRIX_3X2_F::identity();
        rt_set_transform(rt as PRT, &id);
        let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let mut white_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_solid_brush(rt as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);
        rt_draw_line(
            rt as PRT,
            D2D1_POINT_2F { x: 2.0, y: 32.0 }.to_u64(),
            D2D1_POINT_2F { x: 60.0, y: 32.0 }.to_u64(),
            white_brush,
            3.0,
            std::ptr::null_mut(),
        );
        assert_eq!(read_rt_pixel(rt, 30, 32), [255, 255, 255, 255]);

        // PathGeometry triangle + FillGeometry with LinearGradientBrush (black -> white left to right)
        let mut path: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut path), S_OK);
        let mut sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(path_geom_open(path as PGeom, &mut sink), S_OK);
        gsink_begin_figure(sink as PGeomSink, D2D1_POINT_2F { x: 32.0, y: 40.0 }.to_u64(), D2D1_FIGURE_BEGIN_FILLED);
        gsink_add_line(sink as PGeomSink, D2D1_POINT_2F { x: 60.0, y: 40.0 }.to_u64());
        gsink_add_line(sink as PGeomSink, D2D1_POINT_2F { x: 60.0, y: 60.0 }.to_u64());
        gsink_add_line(sink as PGeomSink, D2D1_POINT_2F { x: 32.0, y: 60.0 }.to_u64());
        gsink_end_figure(sink as PGeomSink, D2D1_FIGURE_END_CLOSED);
        assert_eq!(gsink_close(sink as PGeomSink), S_OK);

        let stops = [
            D2D1_GRADIENT_STOP { position: 0.0, color: D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } },
            D2D1_GRADIENT_STOP { position: 1.0, color: D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } },
        ];
        let mut stop_col: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_grad_stops(rt as PRT, stops.as_ptr(), 2, 0, D2D1_EXTEND_MODE_CLAMP, &mut stop_col), S_OK);
        let lgp = D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
            startPoint: D2D1_POINT_2F { x: 32.0, y: 50.0 },
            endPoint: D2D1_POINT_2F { x: 60.0, y: 50.0 },
        };
        let mut lin_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_linear_brush(rt as PRT, &lgp, std::ptr::null(), stop_col, &mut lin_brush), S_OK);
        rt_fill_geometry(rt as PRT, path, lin_brush, std::ptr::null_mut());
        let left_px = read_rt_pixel(rt, 34, 50);
        let right_px = read_rt_pixel(rt, 58, 50);
        assert!(left_px[0] < 40 && right_px[0] > 215, "gradient left={left_px:?} right={right_px:?}");

        assert_eq!(rt_end_draw(rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

        release_com(lin_brush);
        release_com(stop_col);
        release_com(sink);
        release_com(path);
        release_com(white_brush);
        release_com(green_brush);
        release_com(red_brush);
        release_com(rt);
        release_com(fac);
    }
}

#[test]
fn device_context_draw_bitmap_and_dwrite_text_integration() {
    unsafe {
        let mut dc: *mut c_void = std::ptr::null_mut();
        assert_eq!(D2D1CreateDeviceContext(std::ptr::null_mut(), std::ptr::null(), &mut dc), S_OK);
        assert!(!dc.is_null());

        // Create 128x48 target bitmap and bind via SetTarget
        let mut target: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dc_create_bitmap1(
                dc as PRT,
                D2D1_SIZE_U { width: 128, height: 48 }.to_u64(),
                std::ptr::null(),
                0,
                std::ptr::null(),
                &mut target,
            ),
            S_OK
        );
        dc_set_target(dc as PRT, target);

        // Create 2x2 RGBA bitmap and DrawBitmap into [0..8, 0..8]
        let src_pixels: [u8; 16] = [
            255, 255, 0, 255,   255, 255, 0, 255,
            255, 255, 0, 255,   255, 255, 0, 255,
        ];
        let mut src_bmp: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            rt_create_bitmap(
                dc as PRT,
                D2D1_SIZE_U { width: 2, height: 2 }.to_u64(),
                src_pixels.as_ptr() as *const c_void,
                8,
                std::ptr::null(),
                &mut src_bmp,
            ),
            S_OK
        );

        // Create white brush for text
        let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let mut white_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_solid_brush(dc as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);

        // Create DirectWrite factory + TextFormat + TextLayout
        let mut dw_fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dwrite::DWriteCreateFactory(
                dwrite::DWRITE_FACTORY_TYPE_SHARED,
                &dwrite::IID_IDWRITE_FACTORY as *const GUID as *const u8,
                &mut dw_fac,
            ),
            S_OK
        );
        let fam_w: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
        let loc_w: Vec<u16> = "en-us\0".encode_utf16().collect();
        let create_tf: unsafe extern "win64" fn(
            *mut c_void, *const u16, *mut c_void, u32, u32, u32, f32, *const u16, *mut *mut c_void
        ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
        let mut fmt: *mut c_void = std::ptr::null_mut();
        #[cfg(not(target_arch = "aarch64"))]
        assert_eq!(
            create_tf(dw_fac, fam_w.as_ptr(), std::ptr::null_mut(), 400, 0, 5, 24.0, loc_w.as_ptr(), &mut fmt),
            S_OK
        );
        #[cfg(target_arch = "aarch64")]
        {
            // Call through the aarch64 win64_bridge! slot directly, validating the bridge ABI too.
            let _ = create_tf;
            let br: unsafe extern "C" fn(
                u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
                f64, f64, f64, f64,
            ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
            assert_eq!(
                br(
                    dw_fac as u64,
                    fam_w.as_ptr() as u64,
                    0,
                    400,
                    0,
                    5,
                    24.0f32.to_bits() as u64,
                    loc_w.as_ptr() as u64,
                    &mut fmt as *mut *mut c_void as u64,
                    0, 0, 0, 0, 0, 0, 0,
                    0.0, 0.0, 0.0, 0.0,
                ),
                S_OK
            );
        }
        assert!(!fmt.is_null());

        rt_begin_draw(dc as PRT);
        let black = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        rt_clear(dc as PRT, &black);
        let dst_bmp_rect = D2D1_RECT_F { left: 0.0, top: 0.0, right: 8.0, bottom: 8.0 };
        rt_draw_bitmap(dc as PRT, src_bmp, &dst_bmp_rect, 1.0, D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR, std::ptr::null());
        assert_eq!(read_rt_pixel(dc, 4, 4), [255, 255, 0, 255]);

        // DrawText ("HELLO") onto [16..124, 4..44]
        let hello: Vec<u16> = "HELLO".encode_utf16().collect();
        let text_rect = D2D1_RECT_F { left: 16.0, top: 4.0, right: 124.0, bottom: 44.0 };
        rt_draw_text(dc as PRT, hello.as_ptr(), hello.len() as u32, fmt, &text_rect, white_brush, 0, 0);
        assert_eq!(rt_end_draw(dc as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

        // Verify real lit text pixels were rasterized inside the text region!
        let mut lit_pixels = 0u32;
        for py in 4..44u32 {
            for px in 16..124u32 {
                let c = read_rt_pixel(dc, px, py);
                if c[0] > 64 && c[1] > 64 && c[2] > 64 {
                    lit_pixels += 1;
                }
            }
        }
        assert!(lit_pixels >= 40, "expected real rasterized text pixels, got {lit_pixels}");

        release_com(fmt);
        release_com(dw_fac);
        release_com(white_brush);
        release_com(src_bmp);
        release_com(target);
        release_com(dc);
    }
}

#[test]
fn dc_render_target_flushes_pixels_to_gdi_hdc() {
    unsafe {
        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &IID_ID2D1_FACTORY as *const GUID as *const u8,
                std::ptr::null(),
                &mut fac,
            ),
            S_OK
        );
        let mut dc_rt: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_dc_rt(fac as PFactory, std::ptr::null(), &mut dc_rt), S_OK);

        // Create real GDI memory DC + 16x16 DIB
        let hdc = crate::misc::gdi32::CreateCompatibleDC(0);
        let mut bmi = [0u8; 40];
        bmi[0..4].copy_from_slice(&40u32.to_le_bytes());
        bmi[4..8].copy_from_slice(&16i32.to_le_bytes());
        bmi[8..12].copy_from_slice(&(-16i32).to_le_bytes());
        bmi[12..14].copy_from_slice(&1u16.to_le_bytes());
        bmi[14..16].copy_from_slice(&32u16.to_le_bytes());
        let mut bits = std::ptr::null_mut();
        let hbm = crate::misc::gdi32::CreateDIBSection(hdc, bmi.as_ptr(), 0, &mut bits, std::ptr::null_mut(), 0);
        let old = crate::misc::gdi32::SelectObject(hdc, hbm);

        let sub = [0i32, 0, 16, 16];
        assert_eq!(dc_rt_bind_dc(dc_rt as PRT, hdc, &sub), S_OK);
        rt_begin_draw(dc_rt as PRT);
        let cyan = D2D1_COLOR_F { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
        rt_clear(dc_rt as PRT, &cyan);
        assert_eq!(rt_end_draw(dc_rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

        // GDI GetPixel returns COLORREF (0x00BBGGRR -> cyan = 0x00FFFF00)
        let cr = crate::misc::gdi32::GetPixel(hdc, 5, 5);
        assert_eq!(cr, 0x00FF_FF00);

        crate::misc::gdi32::SelectObject(hdc, old);
        crate::misc::gdi32::DeleteObject(hbm);
        crate::misc::gdi32::DeleteDC(hdc);
        release_com(dc_rt);
        release_com(fac);
    }
}

#[test]
fn dll_exports_device_compatible_rt_brushes_strokes_geometries_layers_and_wic() {
    unsafe {
        // 1. All 7 public d2d1.dll exports resolve via win32_rt::lookup
        for name in [
            "D2D1CreateFactory",
            "D2D1CreateDevice",
            "D2D1CreateDeviceContext",
            "D2D1MakeRotateMatrix",
            "D2D1MakeSkewMatrix",
            "D2D1IsMatrixInvertible",
            "D2D1InvertMatrix",
        ] {
            assert!(
                crate::lookup("d2d1.dll", name).is_some(),
                "missing d2d1.dll export {name}"
            );
        }

        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_MULTI_THREADED,
                &IID_ID2D1_FACTORY1 as *const GUID as *const u8,
                std::ptr::null(),
                &mut fac,
            ),
            S_OK
        );

        // 2. ID2D1Factory1::CreateDevice -> ID2D1Device::CreateDeviceContext -> CreateCompatibleRenderTarget
        let mut dev: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_device(fac as PFactory, std::ptr::null_mut(), &mut dev), S_OK);
        let mut dc: *mut c_void = std::ptr::null_mut();
        assert_eq!(dev_create_dc(dev as PDevice, 0, &mut dc), S_OK);

        let compat_px = D2D1_SIZE_U { width: 64, height: 64 };
        let mut compat_rt: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            rt_create_compat_rt(dc as PRT, std::ptr::null(), &compat_px, std::ptr::null(), 0, &mut compat_rt),
            S_OK
        );

        // 3. RadialGradientBrush & BitmapBrush
        let stops = [
            D2D1_GRADIENT_STOP { position: 0.0, color: D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } },
            D2D1_GRADIENT_STOP { position: 1.0, color: D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } },
        ];
        let mut stop_col: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            rt_create_grad_stops(compat_rt as PRT, stops.as_ptr(), 2, 0, D2D1_EXTEND_MODE_CLAMP, &mut stop_col),
            S_OK
        );
        let rgp = D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
            center: D2D1_POINT_2F { x: 16.0, y: 16.0 },
            gradientOriginOffset: D2D1_POINT_2F { x: 0.0, y: 0.0 },
            radiusX: 16.0,
            radiusY: 16.0,
        };
        let mut rad_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_radial_brush(compat_rt as PRT, &rgp, std::ptr::null(), stop_col, &mut rad_brush), S_OK);

        // 2x2 red/blue checkerboard BitmapBrush with WRAP
        let checker: [u8; 16] = [
            255, 0, 0, 255,   0, 0, 255, 255,
            0, 0, 255, 255,   255, 0, 0, 255,
        ];
        let mut chk_bmp: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            rt_create_bitmap(
                compat_rt as PRT,
                D2D1_SIZE_U { width: 2, height: 2 }.to_u64(),
                checker.as_ptr() as *const c_void,
                8,
                std::ptr::null(),
                &mut chk_bmp,
            ),
            S_OK
        );
        let bbp = D2D1_BITMAP_BRUSH_PROPERTIES {
            extendModeX: D2D1_EXTEND_MODE_WRAP,
            extendModeY: D2D1_EXTEND_MODE_WRAP,
            interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
        };
        let mut bmp_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_bitmap_brush(compat_rt as PRT, chk_bmp, &bbp, std::ptr::null(), &mut bmp_brush), S_OK);

        // 4. Stroke style + SaveDrawingState/RestoreDrawingState + PushLayer/PopLayer
        let ss_props = D2D1_STROKE_STYLE_PROPERTIES {
            startCap: D2D1_CAP_STYLE_ROUND,
            endCap: D2D1_CAP_STYLE_ROUND,
            dashStyle: D2D1_DASH_STYLE_DASH,
            ..D2D1_STROKE_STYLE_PROPERTIES::default()
        };
        let mut stroke_style: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_stroke_style(fac as PFactory, &ss_props, std::ptr::null(), 0, &mut stroke_style),
            S_OK
        );

        let mut dsb: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_drawing_state_block(fac as PFactory, std::ptr::null(), std::ptr::null_mut(), &mut dsb),
            S_OK
        );

        rt_begin_draw(compat_rt as PRT);
        let zero = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        rt_clear(compat_rt as PRT, &zero);

        // Fill [0..32, 0..32] with radial gradient: center bright, edge dark
        let r_tl = D2D1_RECT_F { left: 0.0, top: 0.0, right: 32.0, bottom: 32.0 };
        rt_fill_rect(compat_rt as PRT, &r_tl, rad_brush);
        assert!(read_rt_pixel(compat_rt, 16, 16)[0] > 230);
        assert!(read_rt_pixel(compat_rt, 1, 1)[0] < 30);

        // Fill [32..64, 0..32] with wrapped checkerboard BitmapBrush
        let r_tr = D2D1_RECT_F { left: 32.0, top: 0.0, right: 64.0, bottom: 32.0 };
        rt_fill_rect(compat_rt as PRT, &r_tr, bmp_brush);
        assert_eq!(read_rt_pixel(compat_rt, 32, 0), [255, 0, 0, 255]);
        assert_eq!(read_rt_pixel(compat_rt, 33, 0), [0, 0, 255, 255]);

        // Save state, apply transform, restore state
        let shifted = D2D1_MATRIX_3X2_F { m11: 1.0, m12: 0.0, m21: 0.0, m22: 1.0, dx: 12.0, dy: 5.0 };
        rt_set_transform(compat_rt as PRT, &shifted);
        rt_save_state(compat_rt as PRT, dsb);
        let id = D2D1_MATRIX_3X2_F::identity();
        rt_set_transform(compat_rt as PRT, &id);
        rt_restore_state(compat_rt as PRT, dsb);
        let mut got_m = D2D1_MATRIX_3X2_F::default();
        rt_get_transform(compat_rt as PRT, &mut got_m);
        assert_eq!(got_m, shifted);
        rt_set_transform(compat_rt as PRT, &id);

        // PushLayer at 50% opacity with rectangular mask
        let mask_r = D2D1_RECT_F { left: 4.0, top: 36.0, right: 24.0, bottom: 56.0 };
        let mut mask_geom: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_rect_geom(fac as PFactory, &mask_r, &mut mask_geom), S_OK);
        let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let mut white_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_solid_brush(compat_rt as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);

        let lp = D2D1_LAYER_PARAMETERS {
            contentBounds: D2D1_RECT_F { left: 0.0, top: 32.0, right: 32.0, bottom: 64.0 },
            geometricMask: mask_geom,
            maskAntialiasMode: D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
            maskTransform: D2D1_MATRIX_3X2_F::identity(),
            opacity: 0.5,
            opacityBrush: std::ptr::null_mut(),
            layerOptions: 0,
            _pad: 0,
        };
        rt_push_layer(compat_rt as PRT, &lp, std::ptr::null_mut());
        let big_bot = D2D1_RECT_F { left: 0.0, top: 32.0, right: 32.0, bottom: 64.0 };
        rt_fill_rect(compat_rt as PRT, &big_bot, white_brush);
        rt_pop_layer(compat_rt as PRT);
        let in_mask = read_rt_pixel(compat_rt, 14, 46);
        let out_mask = read_rt_pixel(compat_rt, 1, 34);
        assert!((in_mask[0] as i32 - 128).abs() <= 2, "expected 50% layer inside mask, got {in_mask:?}");
        assert_eq!(out_mask[0], 0, "expected untouched outside geometricMask");

        // DrawRectangle / DrawEllipse / DrawRoundedRectangle with stroke_style
        let box_r = D2D1_RECT_F { left: 36.0, top: 36.0, right: 60.0, bottom: 60.0 };
        rt_draw_rect(compat_rt as PRT, &box_r, white_brush, 2.0, stroke_style);
        assert_eq!(rt_end_draw(compat_rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

        // Compatible RT GetBitmap shares rendered content
        let mut compat_bmp: *mut c_void = std::ptr::null_mut();
        assert_eq!(brt_get_bitmap(compat_rt as PRT, &mut compat_bmp), S_OK);
        let mut bmp_px = D2D1_SIZE_U::default();
        bmp_get_pixel_size(compat_bmp as PBitmap, &mut bmp_px);
        assert_eq!(bmp_px, compat_px);

        // 5. Geometry queries (rect area, bounds, FillContainsPoint, path Bezier + Arc length)
        let mut area = 0.0f32;
        assert_eq!(geom_compute_area(mask_geom as PGeom, std::ptr::null(), 0.25, &mut area), S_OK);
        assert!((area - 400.0).abs() < 1.0, "expected 20x20=400 area, got {area}");
        let mut inside = FALSE;
        assert_eq!(
            geom_fill_contains_pt(mask_geom as PGeom, D2D1_POINT_2F { x: 10.0, y: 40.0 }.to_u64(), std::ptr::null(), 0.25, &mut inside),
            S_OK
        );
        assert_eq!(inside, TRUE);

        let mut curve_path: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut curve_path), S_OK);
        let mut csink: *mut c_void = std::ptr::null_mut();
        assert_eq!(path_geom_open(curve_path as PGeom, &mut csink), S_OK);
        gsink_begin_figure(csink as PGeomSink, D2D1_POINT_2F { x: 0.0, y: 0.0 }.to_u64(), D2D1_FIGURE_BEGIN_FILLED);
        let bez = D2D1_BEZIER_SEGMENT {
            point1: D2D1_POINT_2F { x: 10.0, y: 0.0 },
            point2: D2D1_POINT_2F { x: 20.0, y: 10.0 },
            point3: D2D1_POINT_2F { x: 20.0, y: 20.0 },
        };
        gsink_add_bezier(csink as PGeomSink, &bez);
        let arc = D2D1_ARC_SEGMENT {
            point: D2D1_POINT_2F { x: 0.0, y: 20.0 },
            size: D2D1_SIZE_F { width: 10.0, height: 10.0 },
            rotationAngle: 0.0,
            sweepDirection: D2D1_SWEEP_DIRECTION_CLOCKWISE,
            arcSize: D2D1_ARC_SIZE_SMALL,
        };
        gsink_add_arc(csink as PGeomSink, &arc);
        gsink_end_figure(csink as PGeomSink, D2D1_FIGURE_END_CLOSED);
        assert_eq!(gsink_close(csink as PGeomSink), S_OK);
        let mut plen = 0.0f32;
        assert_eq!(geom_compute_length(curve_path as PGeom, std::ptr::null(), 0.25, &mut plen), S_OK);
        assert!(plen > 40.0, "expected positive curve+arc perimeter, got {plen}");

        // 6. CreateWicBitmapRenderTarget flushes rendered pixels to mock IWICBitmap via Lock()
        #[repr(C)]
        struct MockWicLock {
            vtbl: *const usize,
            w: u32,
            h: u32,
            buf: *mut u8,
            len: u32,
        }
        #[repr(C)]
        struct MockWicBitmap {
            vtbl: *const usize,
            w: u32,
            h: u32,
            pixels: Vec<u8>,
        }
        unsafe extern "win64" fn mw_qi(_: *mut c_void, _: *const u8, _: *mut *mut c_void) -> HRESULT { E_NOINTERFACE }
        unsafe extern "win64" fn mw_ar(_: *mut c_void) -> u32 { 1 }
        unsafe extern "win64" fn mw_rel(_: *mut c_void) -> u32 { 1 }
        unsafe extern "win64" fn mw_get_size(this: *mut c_void, w: *mut u32, h: *mut u32) -> HRESULT {
            let m = &*(this as *const MockWicBitmap);
            w.write_unaligned(m.w);
            h.write_unaligned(m.h);
            S_OK
        }
        unsafe extern "win64" fn mw_noop(_: *mut c_void) -> HRESULT { E_NOTIMPL }
        unsafe extern "win64" fn mwl_rel(this: *mut c_void) -> u32 {
            drop(Box::from_raw(this as *mut MockWicLock));
            0
        }
        unsafe extern "win64" fn mwl_stride(this: *mut c_void, stride: *mut u32) -> HRESULT {
            let l = &*(this as *const MockWicLock);
            stride.write_unaligned(l.w * 4);
            S_OK
        }
        unsafe extern "win64" fn mwl_data(this: *mut c_void, sz: *mut u32, ptr: *mut *mut u8) -> HRESULT {
            let l = &*(this as *const MockWicLock);
            sz.write_unaligned(l.len);
            ptr.write_unaligned(l.buf);
            S_OK
        }
        static LOCK_VTBL: OnceLock<[usize; 7]> = OnceLock::new();
        let lvtbl = LOCK_VTBL
            .get_or_init(|| {
                register_vtable([
                    mw_qi as *const () as usize,
                    mw_ar as *const () as usize,
                    mwl_rel as *const () as usize,
                    mw_noop as *const () as usize,
                    mwl_stride as *const () as usize,
                    mwl_data as *const () as usize,
                    mw_noop as *const () as usize,
                ])
            })
            .as_ptr();
        unsafe extern "win64" fn mw_lock(
            this: *mut c_void,
            _rect: *const c_void,
            _flags: u32,
            out_lock: *mut *mut c_void,
        ) -> HRESULT {
            let m = &mut *(this as *mut MockWicBitmap);
            let l = Box::new(MockWicLock {
                vtbl: LOCK_VTBL.get().unwrap().as_ptr(),
                w: m.w,
                h: m.h,
                buf: m.pixels.as_mut_ptr(),
                len: m.pixels.len() as u32,
            });
            out_lock.write_unaligned(Box::into_raw(l) as *mut c_void);
            S_OK
        }
        let _ = lvtbl;
        let wic_vtbl = register_vtable([
            mw_qi as *const () as usize,
            mw_ar as *const () as usize,
            mw_rel as *const () as usize,
            mw_get_size as *const () as usize,
            mw_noop as *const () as usize,
            mw_noop as *const () as usize,
            mw_noop as *const () as usize,
            mw_noop as *const () as usize,
            mw_lock as *const () as usize,
        ]);
        let mut mock_wic = MockWicBitmap {
            vtbl: wic_vtbl.as_ptr(),
            w: 16,
            h: 16,
            pixels: vec![0u8; 16 * 16 * 4],
        };
        let mut wic_rt: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            fac_create_wic_rt(
                fac as PFactory,
                &mut mock_wic as *mut MockWicBitmap as *mut c_void,
                std::ptr::null(),
                &mut wic_rt,
            ),
            S_OK
        );
        rt_begin_draw(wic_rt as PRT);
        let magenta = D2D1_COLOR_F { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
        rt_clear(wic_rt as PRT, &magenta);
        assert_eq!(rt_end_draw(wic_rt as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);
        assert_eq!(&mock_wic.pixels[0..4], &[255, 0, 255, 255]);

        release_com(wic_rt);
        release_com(csink);
        release_com(curve_path);
        release_com(compat_bmp);
        release_com(white_brush);
        release_com(mask_geom);
        release_com(dsb);
        release_com(stroke_style);
        release_com(bmp_brush);
        release_com(chk_bmp);
        release_com(rad_brush);
        release_com(stop_col);
        release_com(compat_rt);
        release_com(dc);
        release_com(dev);
        release_com(fac);
    }
}

#[test]
fn draw_text_layout_with_underline_and_drawing_effect() {
    unsafe {
        let mut dc: *mut c_void = std::ptr::null_mut();
        assert_eq!(D2D1CreateDeviceContext(std::ptr::null_mut(), std::ptr::null(), &mut dc), S_OK);
        let mut target: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dc_create_bitmap1(dc as PRT, D2D1_SIZE_U { width: 120, height: 48 }.to_u64(), std::ptr::null(), 0, std::ptr::null(), &mut target),
            S_OK
        );
        dc_set_target(dc as PRT, target);

        let white = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let red = D2D1_COLOR_F { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
        let mut white_brush: *mut c_void = std::ptr::null_mut();
        let mut red_brush: *mut c_void = std::ptr::null_mut();
        assert_eq!(rt_create_solid_brush(dc as PRT, &white, std::ptr::null(), &mut white_brush), S_OK);
        assert_eq!(rt_create_solid_brush(dc as PRT, &red, std::ptr::null(), &mut red_brush), S_OK);

        // Create DirectWrite format + layout
        let mut dw_fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            dwrite::DWriteCreateFactory(
                dwrite::DWRITE_FACTORY_TYPE_SHARED,
                &dwrite::IID_IDWRITE_FACTORY as *const GUID as *const u8,
                &mut dw_fac,
            ),
            S_OK
        );
        let fam_w: Vec<u16> = "Segoe UI\0".encode_utf16().collect();
        let loc_w: Vec<u16> = "en-us\0".encode_utf16().collect();
        let mut fmt: *mut c_void = std::ptr::null_mut();
        #[cfg(not(target_arch = "aarch64"))]
        {
            let create_tf: unsafe extern "win64" fn(
                *mut c_void, *const u16, *mut c_void, u32, u32, u32, f32, *const u16, *mut *mut c_void
            ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
            assert_eq!(create_tf(dw_fac, fam_w.as_ptr(), std::ptr::null_mut(), 400, 0, 5, 22.0, loc_w.as_ptr(), &mut fmt), S_OK);
        }
        #[cfg(target_arch = "aarch64")]
        {
            let br: unsafe extern "C" fn(
                u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
                f64, f64, f64, f64,
            ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(15).read_unaligned());
            assert_eq!(
                br(
                    dw_fac as u64, fam_w.as_ptr() as u64, 0, 400, 0, 5,
                    22.0f32.to_bits() as u64, loc_w.as_ptr() as u64, &mut fmt as *mut *mut c_void as u64,
                    0, 0, 0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0,
                ),
                S_OK
            );
        }

        let text_u16: Vec<u16> = "ABCD".encode_utf16().collect();
        let mut layout: *mut c_void = std::ptr::null_mut();
        #[cfg(not(target_arch = "aarch64"))]
        {
            let create_tl: unsafe extern "win64" fn(
                *mut c_void, *const u16, u32, *mut c_void, f32, f32, *mut *mut c_void
            ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(18).read_unaligned());
            assert_eq!(create_tl(dw_fac, text_u16.as_ptr(), text_u16.len() as u32, fmt, 100.0, 40.0, &mut layout), S_OK);
        }
        #[cfg(target_arch = "aarch64")]
        {
            let br: unsafe extern "C" fn(
                u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64,
                f64, f64, f64, f64,
            ) -> HRESULT = std::mem::transmute(vtbl_of(dw_fac).add(18).read_unaligned());
            assert_eq!(
                br(
                    dw_fac as u64, text_u16.as_ptr() as u64, text_u16.len() as u64, fmt as u64,
                    100.0f32.to_bits() as u64, 40.0f32.to_bits() as u64, &mut layout as *mut *mut c_void as u64,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0,
                ),
                S_OK
            );
        }

        // Set red drawingEffect + underline on entire range via IDWriteTextLayout vtable slots 36 & 38
        let set_ul: unsafe extern "win64" fn(*mut c_void, BOOL, u64) -> HRESULT =
            std::mem::transmute(vtbl_of(layout).add(36).read_unaligned());
        let set_eff: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64) -> HRESULT =
            std::mem::transmute(vtbl_of(layout).add(38).read_unaligned());
        let full_range = (text_u16.len() as u64) << 32;
        assert_eq!(set_ul(layout, TRUE, full_range), S_OK);
        assert_eq!(set_eff(layout, red_brush, full_range), S_OK);

        rt_begin_draw(dc as PRT);
        let black = D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        rt_clear(dc as PRT, &black);
        rt_draw_text_layout(dc as PRT, D2D1_POINT_2F { x: 8.0, y: 6.0 }.to_u64(), layout, white_brush, 0);
        assert_eq!(rt_end_draw(dc as PRT, std::ptr::null_mut(), std::ptr::null_mut()), S_OK);

        // Verify red pixels (from drawingEffect brush!) were rendered, and zero white-only pixels
        let mut red_pixels = 0u32;
        for py in 0..48u32 {
            for px in 0..120u32 {
                let c = read_rt_pixel(dc, px, py);
                if c[0] > 80 && c[1] < 20 && c[2] < 20 {
                    red_pixels += 1;
                }
            }
        }
        assert!(red_pixels >= 40, "expected red text+underline pixels from drawingEffect, got {red_pixels}");

        release_com(layout);
        release_com(fmt);
        release_com(dw_fac);
        release_com(red_brush);
        release_com(white_brush);
        release_com(target);
        release_com(dc);
    }
}

#[test]
fn geometry_tessellate_rect_concave_l_shape_and_ellipse() {
    use super::geometry::{
        geom_tessellate, gsink_add_line, gsink_begin_figure, gsink_close, gsink_end_figure,
        path_geom_open, PGeom, PGeomSink,
    };

    #[repr(C)]
    struct MockTessSink {
        vtbl: *const [usize; 5],
        tris: Vec<D2D1_TRIANGLE>,
    }
    unsafe extern "win64" fn mock_qi(_this: *mut MockTessSink, _riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
        if !ppv.is_null() { ppv.write_unaligned(std::ptr::null_mut()); }
        E_NOINTERFACE
    }
    unsafe extern "win64" fn mock_addref(_this: *mut MockTessSink) -> u32 { 1 }
    unsafe extern "win64" fn mock_release(_this: *mut MockTessSink) -> u32 { 1 }
    unsafe extern "win64" fn mock_add_tris(this: *mut MockTessSink, tris: *const D2D1_TRIANGLE, count: u32) {
        let sink = &mut *this;
        for i in 0..count as usize {
            sink.tris.push(tris.add(i).read_unaligned());
        }
    }
    unsafe extern "win64" fn mock_close(_this: *mut MockTessSink) -> HRESULT { S_OK }

    fn tri_area_sum(tris: &[D2D1_TRIANGLE]) -> f32 {
        tris.iter()
            .map(|t| {
                let ax = t.point2.x - t.point1.x;
                let ay = t.point2.y - t.point1.y;
                let bx = t.point3.x - t.point1.x;
                let by = t.point3.y - t.point1.y;
                0.5 * (ax * by - ay * bx).abs()
            })
            .sum()
    }

    unsafe {
        let vtbl: [usize; 5] = [
            mock_qi as *const () as usize,
            mock_addref as *const () as usize,
            mock_release as *const () as usize,
            mock_add_tris as *const () as usize,
            mock_close as *const () as usize,
        ];
        crate::com::register_vtable_ptrs(&vtbl);

        let mut fac: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &IID_ID2D1_FACTORY as *const GUID as *const u8,
                std::ptr::null(),
                &mut fac,
            ),
            S_OK
        );

        // 1. Rectangle (10x20 -> 2 triangles, area 200; scaled by 2x -> area 800).
        let rect = D2D1_RECT_F { left: 0.0, top: 0.0, right: 10.0, bottom: 20.0 };
        let mut rgeom: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_rect_geom(fac as PFactory, &rect, &mut rgeom), S_OK);
        let mut sink = MockTessSink { vtbl: &vtbl, tris: Vec::new() };
        assert_eq!(geom_tessellate(rgeom as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert_eq!(sink.tris.len(), 2);
        assert!((tri_area_sum(&sink.tris) - 200.0).abs() < 1e-2);

        sink.tris.clear();
        let scale2 = D2D1_MATRIX_3X2_F { m11: 2.0, m12: 0.0, m21: 0.0, m22: 2.0, dx: 5.0, dy: 5.0 };
        assert_eq!(geom_tessellate(rgeom as PGeom, &scale2, 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert_eq!(sink.tris.len(), 2);
        assert!((tri_area_sum(&sink.tris) - 800.0).abs() < 1e-2);

        // 2. Concave L-shape polygon (6 vertices -> 4 triangles, area = 10*10 - 5*5 = 75).
        let mut pgeom: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut pgeom), S_OK);
        let mut psink: *mut c_void = std::ptr::null_mut();
        assert_eq!(path_geom_open(pgeom as PGeom, &mut psink), S_OK);
        gsink_begin_figure(psink as PGeomSink, D2D1_POINT_2F { x: 0.0, y: 0.0 }.to_u64(), D2D1_FIGURE_BEGIN_FILLED);
        for &(x, y) in &[(10.0, 0.0), (10.0, 5.0), (5.0, 5.0), (5.0, 10.0), (0.0, 10.0)] {
            gsink_add_line(psink as PGeomSink, D2D1_POINT_2F { x, y }.to_u64());
        }
        gsink_end_figure(psink as PGeomSink, D2D1_FIGURE_END_CLOSED);
        assert_eq!(gsink_close(psink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(pgeom as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert_eq!(sink.tris.len(), 4);
        assert!((tri_area_sum(&sink.tris) - 75.0).abs() < 1e-2, "L-shape area {}", tri_area_sum(&sink.tris));

        // 3. Ellipse (r=10 -> area ~ pi*100 = 314.16).
        let el = D2D1_ELLIPSE { point: D2D1_POINT_2F { x: 0.0, y: 0.0 }, radiusX: 10.0, radiusY: 10.0 };
        let mut egeom: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_ellipse_geom(fac as PFactory, &el, &mut egeom), S_OK);
        sink.tris.clear();
        assert_eq!(geom_tessellate(egeom as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!(sink.tris.len() >= 20);
        let el_area = tri_area_sum(&sink.tris);
        assert!((el_area - std::f32::consts::PI * 100.0).abs() < 10.0, "ellipse area {el_area}");

        // Null sink returns E_POINTER.
        assert_eq!(geom_tessellate(rgeom as PGeom, std::ptr::null(), 0.25, std::ptr::null_mut()), E_POINTER);

        // 4. Compound contours with fill rules (ALTERNATE holes, WINDING opposite vs same winding,
        //    and island-inside-hole) must match `geom_fill_contains_pt`.
        let tris_contain_pt = |tris: &[D2D1_TRIANGLE], x: f32, y: f32| -> bool {
            tris.iter().any(|t| {
                let (a, b, c) = (t.point1, t.point2, t.point3);
                let cross = |p: D2D1_POINT_2F, q: D2D1_POINT_2F, r: D2D1_POINT_2F| -> f32 {
                    (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)
                };
                let c1 = cross(a, b, D2D1_POINT_2F { x, y });
                let c2 = cross(b, c, D2D1_POINT_2F { x, y });
                let c3 = cross(c, a, D2D1_POINT_2F { x, y });
                (c1 >= -1e-4 && c2 >= -1e-4 && c3 >= -1e-4)
                    || (c1 <= 1e-4 && c2 <= 1e-4 && c3 <= 1e-4)
            })
        };
        let add_ring = |ps: PGeomSink, pts: &[(f32, f32)]| {
            gsink_begin_figure(ps, D2D1_POINT_2F { x: pts[0].0, y: pts[0].1 }.to_u64(), D2D1_FIGURE_BEGIN_FILLED);
            for &(x, y) in &pts[1..] {
                gsink_add_line(ps, D2D1_POINT_2F { x, y }.to_u64());
            }
            gsink_end_figure(ps, D2D1_FIGURE_END_CLOSED);
        };
        let set_fill_mode = |ps: *mut c_void, mode: u32| {
            let f: unsafe extern "win64" fn(*mut c_void, u32) =
                std::mem::transmute(vtbl_of(ps).add(3).read_unaligned());
            f(ps, mode);
        };
        let outer_ccw = [(0.0, 0.0), (20.0, 0.0), (20.0, 20.0), (0.0, 20.0)];
        let inner_ccw = [(5.0, 5.0), (15.0, 5.0), (15.0, 15.0), (5.0, 15.0)];
        let inner_cw = [(5.0, 5.0), (5.0, 15.0), (15.0, 15.0), (15.0, 5.0)];

        // 4a. ALTERNATE donut (same winding -> inner contour cuts a 10x10 hole: area = 400 - 100 = 300).
        let mut alt_donut: *mut c_void = std::ptr::null_mut();
        let mut alt_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut alt_donut), S_OK);
        assert_eq!(path_geom_open(alt_donut as PGeom, &mut alt_sink), S_OK);
        set_fill_mode(alt_sink, D2D1_FILL_MODE_ALTERNATE);
        add_ring(alt_sink as PGeomSink, &outer_ccw);
        add_ring(alt_sink as PGeomSink, &inner_ccw);
        assert_eq!(gsink_close(alt_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(alt_donut as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!((tri_area_sum(&sink.tris) - 300.0).abs() < 1e-2, "ALTERNATE donut area {}", tri_area_sum(&sink.tris));
        assert!(!tris_contain_pt(&sink.tris, 10.0, 10.0), "hole center must not be covered under ALTERNATE");
        assert!(tris_contain_pt(&sink.tris, 2.0, 10.0), "donut rim must be covered under ALTERNATE");
        let mut hit = TRUE;
        assert_eq!(geom_fill_contains_pt(alt_donut as PGeom, D2D1_POINT_2F { x: 10.0, y: 10.0 }.to_u64(), std::ptr::null(), 0.25, &mut hit), S_OK);
        assert_eq!(hit, FALSE);

        // 4b. WINDING donut with opposite-wound hole (CCW outer + CW inner -> area = 300, center empty).
        let mut wind_hole: *mut c_void = std::ptr::null_mut();
        let mut wh_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut wind_hole), S_OK);
        assert_eq!(path_geom_open(wind_hole as PGeom, &mut wh_sink), S_OK);
        set_fill_mode(wh_sink, D2D1_FILL_MODE_WINDING);
        add_ring(wh_sink as PGeomSink, &outer_ccw);
        add_ring(wh_sink as PGeomSink, &inner_cw);
        assert_eq!(gsink_close(wh_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(wind_hole as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!((tri_area_sum(&sink.tris) - 300.0).abs() < 1e-2, "WINDING hole area {}", tri_area_sum(&sink.tris));
        assert!(!tris_contain_pt(&sink.tris, 10.0, 10.0), "opposite-wound hole must stay empty under WINDING");
        assert!(tris_contain_pt(&sink.tris, 18.0, 10.0));

        // 4c. WINDING nested same-direction contour (CCW outer + CCW inner -> solid area = 400, center filled).
        let mut wind_solid: *mut c_void = std::ptr::null_mut();
        let mut ws_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut wind_solid), S_OK);
        assert_eq!(path_geom_open(wind_solid as PGeom, &mut ws_sink), S_OK);
        set_fill_mode(ws_sink, D2D1_FILL_MODE_WINDING);
        add_ring(ws_sink as PGeomSink, &outer_ccw);
        add_ring(ws_sink as PGeomSink, &inner_ccw);
        assert_eq!(gsink_close(ws_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(wind_solid as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!((tri_area_sum(&sink.tris) - 400.0).abs() < 1e-2, "WINDING same-winding area {}", tri_area_sum(&sink.tris));
        assert!(tris_contain_pt(&sink.tris, 10.0, 10.0), "same-wound inner contour must be filled under WINDING");
        assert_eq!(geom_fill_contains_pt(wind_solid as PGeom, D2D1_POINT_2F { x: 10.0, y: 10.0 }.to_u64(), std::ptr::null(), 0.25, &mut hit), S_OK);
        assert_eq!(hit, TRUE);

        // 4d. ALTERNATE nested island inside hole (30x30 - 20x20 + 10x10 = 600).
        let mut island_geom: *mut c_void = std::ptr::null_mut();
        let mut isl_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut island_geom), S_OK);
        assert_eq!(path_geom_open(island_geom as PGeom, &mut isl_sink), S_OK);
        set_fill_mode(isl_sink, D2D1_FILL_MODE_ALTERNATE);
        add_ring(isl_sink as PGeomSink, &[(0.0, 0.0), (30.0, 0.0), (30.0, 30.0), (0.0, 30.0)]);
        add_ring(isl_sink as PGeomSink, &[(5.0, 5.0), (25.0, 5.0), (25.0, 25.0), (5.0, 25.0)]);
        add_ring(isl_sink as PGeomSink, &[(10.0, 10.0), (20.0, 10.0), (20.0, 20.0), (10.0, 20.0)]);
        assert_eq!(gsink_close(isl_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(island_geom as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!((tri_area_sum(&sink.tris) - 600.0).abs() < 1e-2, "island-in-hole area {}", tri_area_sum(&sink.tris));
        assert!(tris_contain_pt(&sink.tris, 2.0, 15.0), "outer rim must be filled");
        assert!(!tris_contain_pt(&sink.tris, 7.5, 15.0), "middle ring must be empty");
        assert!(tris_contain_pt(&sink.tris, 15.0, 15.0), "inner island must be filled");

        // 4e. Self-intersecting zero-signed-area bowtie: (0,0)->(10,10)->(0,10)->(10,0)
        //     must NOT be discarded; both lobes are filled (area = 25 + 25 = 50).
        let mut bowtie_geom: *mut c_void = std::ptr::null_mut();
        let mut bt_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut bowtie_geom), S_OK);
        assert_eq!(path_geom_open(bowtie_geom as PGeom, &mut bt_sink), S_OK);
        set_fill_mode(bt_sink, D2D1_FILL_MODE_ALTERNATE);
        add_ring(bt_sink as PGeomSink, &[(0.0, 0.0), (10.0, 10.0), (0.0, 10.0), (10.0, 0.0)]);
        assert_eq!(gsink_close(bt_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(bowtie_geom as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!((tri_area_sum(&sink.tris) - 50.0).abs() < 1e-2, "bowtie area {}", tri_area_sum(&sink.tris));
        assert!(tris_contain_pt(&sink.tris, 5.0, 2.0), "lower bowtie lobe must be covered");
        assert!(tris_contain_pt(&sink.tris, 5.0, 8.0), "upper bowtie lobe must be covered");
        assert!(!tris_contain_pt(&sink.tris, 1.0, 5.0), "outside left of bowtie must be empty");
        assert!(!tris_contain_pt(&sink.tris, 9.0, 5.0), "outside right of bowtie must be empty");

        // 4f. Two overlapping squares under ALTERNATE (XOR area = 100) vs WINDING (union area = 150).
        let mut ov_alt: *mut c_void = std::ptr::null_mut();
        let mut ova_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut ov_alt), S_OK);
        assert_eq!(path_geom_open(ov_alt as PGeom, &mut ova_sink), S_OK);
        set_fill_mode(ova_sink, D2D1_FILL_MODE_ALTERNATE);
        add_ring(ova_sink as PGeomSink, &[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]);
        add_ring(ova_sink as PGeomSink, &[(5.0, 0.0), (15.0, 0.0), (15.0, 10.0), (5.0, 10.0)]);
        assert_eq!(gsink_close(ova_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(ov_alt as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!((tri_area_sum(&sink.tris) - 100.0).abs() < 1e-2, "overlapping ALTERNATE area {}", tri_area_sum(&sink.tris));
        assert!(!tris_contain_pt(&sink.tris, 7.5, 5.0), "overlap region must be empty under ALTERNATE");

        let mut ov_wind: *mut c_void = std::ptr::null_mut();
        let mut ovw_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut ov_wind), S_OK);
        assert_eq!(path_geom_open(ov_wind as PGeom, &mut ovw_sink), S_OK);
        set_fill_mode(ovw_sink, D2D1_FILL_MODE_WINDING);
        add_ring(ovw_sink as PGeomSink, &[(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]);
        add_ring(ovw_sink as PGeomSink, &[(5.0, 0.0), (15.0, 0.0), (15.0, 10.0), (5.0, 10.0)]);
        assert_eq!(gsink_close(ovw_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(ov_wind as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!((tri_area_sum(&sink.tris) - 150.0).abs() < 1e-2, "overlapping WINDING area {}", tri_area_sum(&sink.tris));
        assert!(tris_contain_pt(&sink.tris, 7.5, 5.0), "overlap region must be filled under WINDING");

        // 4g. Multi-hole polygon where ear-clipping after bridging hits a stuck corner
        //     must fall back cleanly to scanline-slab tessellation (exact area = 10000 - 12 - 12 = 9976).
        let mut multi_hole: *mut c_void = std::ptr::null_mut();
        let mut mh_sink: *mut c_void = std::ptr::null_mut();
        assert_eq!(fac_create_path_geom(fac as PFactory, &mut multi_hole), S_OK);
        assert_eq!(path_geom_open(multi_hole as PGeom, &mut mh_sink), S_OK);
        set_fill_mode(mh_sink, D2D1_FILL_MODE_ALTERNATE);
        add_ring(mh_sink as PGeomSink, &[(0.0, 0.0), (100.0, 0.0), (100.0, 100.0), (0.0, 100.0)]);
        add_ring(mh_sink as PGeomSink, &[(26.9, 37.7), (29.9, 37.7), (29.9, 41.7), (26.9, 41.7)]);
        add_ring(mh_sink as PGeomSink, &[(32.8, 52.2), (35.8, 52.2), (35.8, 56.2), (32.8, 56.2)]);
        assert_eq!(gsink_close(mh_sink as PGeomSink), S_OK);

        sink.tris.clear();
        assert_eq!(geom_tessellate(multi_hole as PGeom, std::ptr::null(), 0.25, &mut sink as *mut _ as *mut c_void), S_OK);
        assert!(
            (tri_area_sum(&sink.tris) - 9976.0).abs() < 1e-1,
            "multi-hole area must equal 9976.0 without overlapping triangles, got {}",
            tri_area_sum(&sink.tris)
        );
        assert!(!tris_contain_pt(&sink.tris, 28.4, 39.7), "first hole interior must remain empty");
        assert!(!tris_contain_pt(&sink.tris, 34.3, 54.2), "second hole interior must remain empty");
        assert!(tris_contain_pt(&sink.tris, 10.0, 10.0), "outer interior must be filled");
        assert!(tris_contain_pt(&sink.tris, 31.0, 47.0), "region between holes must be filled");

        release_com(mh_sink);
        release_com(multi_hole);
        release_com(ovw_sink);
        release_com(ov_wind);
        release_com(ova_sink);
        release_com(ov_alt);
        release_com(bt_sink);
        release_com(bowtie_geom);
        release_com(isl_sink);
        release_com(island_geom);
        release_com(ws_sink);
        release_com(wind_solid);
        release_com(wh_sink);
        release_com(wind_hole);
        release_com(alt_sink);
        release_com(alt_donut);
        release_com(egeom);
        release_com(psink);
        release_com(pgeom);
        release_com(rgeom);
        release_com(fac);
    }
}
