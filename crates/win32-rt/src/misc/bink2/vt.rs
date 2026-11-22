//! Apple `VideoToolbox` hardware HEVC decoder for `KB2v` streams (macOS + iOS).
//!
//! Decodes 4-byte-length-prefixed HVCC access units synchronously on the Apple Silicon
//! Media Engine directly into Bink's caller-registered planar `Y`, `Cr` (V), and `Cb` (U)
//! frame buffers with zero CPU video decode overhead.

use std::ffi::c_void;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple {
    use super::*;

    type OSStatus = i32;
    type CFAllocatorRef = *const c_void;
    type CFDictionaryRef = *const c_void;
    type CFTypeRef = *const c_void;
    type CFStringRef = *const c_void;
    type CFNumberRef = *const c_void;
    type CMFormatDescriptionRef = *const c_void;
    type CMBlockBufferRef = *mut c_void;
    type CMSampleBufferRef = *mut c_void;
    type VTDecompressionSessionRef = *mut c_void;
    type CVImageBufferRef = *mut c_void;
    type CVPixelBufferRef = *mut c_void;
    type VTDecodeFrameFlags = u32;
    type VTDecodeInfoFlags = u32;
    type CMItemCount = isize;

    const K_CF_NUMBER_SINT32_TYPE: isize = 3;
    const K_CV_PIXEL_BUFFER_LOCK_READ_ONLY: u64 = 1;
    // Planar 4:2:0 YCbCr (3 planes: Y, Cb, Cr)
    const K_CV_PIXEL_FORMAT_420_PLANAR: i32 = u32::from_be_bytes(*b"y420") as i32;
    const K_CV_PIXEL_FORMAT_420_PLANAR_FULL: i32 = u32::from_be_bytes(*b"f420") as i32;
    // Bi-planar NV12 4:2:0 YCbCr (2 planes: Y, interleaved CbCr) fallback
    const K_CV_PIXEL_FORMAT_420_BIPLANAR: i32 = u32::from_be_bytes(*b"420v") as i32;
    const K_CV_PIXEL_FORMAT_420_BIPLANAR_FULL: i32 = u32::from_be_bytes(*b"420f") as i32;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CMTime {
        value: i64,
        timescale: i32,
        flags: u32,
        epoch: i64,
    }

    const K_CM_TIME_INVALID: CMTime = CMTime {
        value: 0,
        timescale: 0,
        flags: 0,
        epoch: 0,
    };

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CMSampleTimingInfo {
        duration: CMTime,
        presentation_time_stamp: CMTime,
        decode_time_stamp: CMTime,
    }

    type VTDecompressionOutputCallback = unsafe extern "C" fn(
        decompression_output_ref_con: *mut c_void,
        source_frame_ref_con: *mut c_void,
        status: OSStatus,
        info_flags: VTDecodeInfoFlags,
        image_buffer: CVImageBufferRef,
        presentation_time_stamp: CMTime,
        presentation_duration: CMTime,
    );

    #[repr(C)]
    struct VTDecompressionOutputCallbackRecord {
        decompression_output_callback: VTDecompressionOutputCallback,
        decompression_output_ref_con: *mut c_void,
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFAllocatorDefault: CFAllocatorRef;
        static kCFAllocatorNull: CFAllocatorRef;
        static kCFTypeDictionaryKeyCallBacks: u8;
        static kCFTypeDictionaryValueCallBacks: u8;

        fn CFRelease(cf: CFTypeRef);
        fn CFNumberCreate(
            allocator: CFAllocatorRef,
            the_type: isize,
            value_ptr: *const c_void,
        ) -> CFNumberRef;
        fn CFDictionaryCreate(
            allocator: CFAllocatorRef,
            keys: *const *const c_void,
            values: *const *const c_void,
            num_values: isize,
            key_callbacks: *const u8,
            value_callbacks: *const u8,
        ) -> CFDictionaryRef;
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        static kCVPixelBufferPixelFormatTypeKey: CFStringRef;

        fn CVPixelBufferLockBaseAddress(pixel_buffer: CVPixelBufferRef, lock_flags: u64) -> i32;
        fn CVPixelBufferUnlockBaseAddress(pixel_buffer: CVPixelBufferRef, unlock_flags: u64) -> i32;
        fn CVPixelBufferGetPlaneCount(pixel_buffer: CVPixelBufferRef) -> usize;
        fn CVPixelBufferGetWidthOfPlane(pixel_buffer: CVPixelBufferRef, plane_index: usize) -> usize;
        fn CVPixelBufferGetHeightOfPlane(pixel_buffer: CVPixelBufferRef, plane_index: usize) -> usize;
        fn CVPixelBufferGetBytesPerRowOfPlane(
            pixel_buffer: CVPixelBufferRef,
            plane_index: usize,
        ) -> usize;
        fn CVPixelBufferGetBaseAddressOfPlane(
            pixel_buffer: CVPixelBufferRef,
            plane_index: usize,
        ) -> *mut c_void;
    }

    #[link(name = "CoreMedia", kind = "framework")]
    extern "C" {
        fn CMVideoFormatDescriptionCreateFromHEVCParameterSets(
            allocator: CFAllocatorRef,
            parameter_set_count: usize,
            parameter_set_pointers: *const *const u8,
            parameter_set_sizes: *const usize,
            nal_unit_header_length: i32,
            extensions: CFDictionaryRef,
            format_description_out: *mut CMFormatDescriptionRef,
        ) -> OSStatus;

        fn CMBlockBufferCreateWithMemoryBlock(
            structure_allocator: CFAllocatorRef,
            memory_block: *mut c_void,
            block_length: usize,
            block_allocator: CFAllocatorRef,
            custom_block_source: *const c_void,
            offset_to_data: usize,
            data_length: usize,
            flags: u32,
            block_buffer_out: *mut CMBlockBufferRef,
        ) -> OSStatus;

        fn CMSampleBufferCreateReady(
            allocator: CFAllocatorRef,
            data_buffer: CMBlockBufferRef,
            format_description: CMFormatDescriptionRef,
            num_samples: CMItemCount,
            num_sample_timing_entries: CMItemCount,
            sample_timing_array: *const CMSampleTimingInfo,
            num_sample_size_entries: CMItemCount,
            sample_size_array: *const usize,
            sample_buffer_out: *mut CMSampleBufferRef,
        ) -> OSStatus;
    }

    #[link(name = "VideoToolbox", kind = "framework")]
    extern "C" {
        fn VTDecompressionSessionCreate(
            allocator: CFAllocatorRef,
            video_format_description: CMFormatDescriptionRef,
            video_decoder_specification: CFDictionaryRef,
            destination_image_buffer_attributes: CFDictionaryRef,
            output_callback: *const VTDecompressionOutputCallbackRecord,
            decompression_session_out: *mut VTDecompressionSessionRef,
        ) -> OSStatus;

        fn VTDecompressionSessionDecodeFrame(
            session: VTDecompressionSessionRef,
            sample_buffer: CMSampleBufferRef,
            decode_flags: VTDecodeFrameFlags,
            source_frame_ref_con: *mut c_void,
            info_flags_out: *mut VTDecodeInfoFlags,
        ) -> OSStatus;

        fn VTDecompressionSessionInvalidate(session: VTDecompressionSessionRef);
    }

    struct DecodeTarget<'a> {
        ya_width: usize,
        ya_height: usize,
        ya_pitch: usize,
        crcb_pitch: usize,
        dst_y: &'a mut [u8],
        dst_cr: &'a mut [u8],
        dst_cb: &'a mut [u8],
        ok: bool,
        status: OSStatus,
    }

    /// Copy one plane with edge-pixel replication into Bink's aligned padding rows/cols.
    unsafe fn copy_plane_with_pad(
        src: *const u8,
        src_w: usize,
        src_h: usize,
        src_stride: usize,
        dst: &mut [u8],
        dst_w: usize,
        dst_h: usize,
        dst_stride: usize,
    ) {
        if src.is_null() || src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
            return;
        }
        let copy_w = src_w.min(dst_w).min(dst_stride);
        let copy_h = src_h.min(dst_h);
        for row in 0..copy_h {
            let d_off = row * dst_stride;
            if d_off + copy_w > dst.len() {
                break;
            }
            let s_row = std::slice::from_raw_parts(src.add(row * src_stride), copy_w);
            dst[d_off..d_off + copy_w].copy_from_slice(s_row);
            if copy_w < dst_w {
                let pad_end = dst_w.min(dst_stride);
                if d_off + pad_end <= dst.len() {
                    let edge = s_row[copy_w - 1];
                    dst[d_off + copy_w..d_off + pad_end].fill(edge);
                }
            }
        }
        if copy_h > 0 && copy_h < dst_h {
            let last_off = (copy_h - 1) * dst_stride;
            let row_len = dst_w.min(dst_stride);
            if last_off + row_len <= dst.len() {
                for row in copy_h..dst_h {
                    let d_off = row * dst_stride;
                    if d_off + row_len > dst.len() {
                        break;
                    }
                    dst.copy_within(last_off..last_off + row_len, d_off);
                }
            }
        }
    }

    /// De-interleave NV12 bi-planar chroma (`[Cb0, Cr0, Cb1, Cr1, ...]`) into separate
    /// `dst_cb` and `dst_cr` planes with edge-pixel padding.
    unsafe fn deinterleave_nv12_chroma(
        src: *const u8,
        src_w: usize,
        src_h: usize,
        src_stride: usize,
        dst_cb: &mut [u8],
        dst_cr: &mut [u8],
        dst_w: usize,
        dst_h: usize,
        dst_stride: usize,
    ) {
        if src.is_null() || src_w == 0 || src_h == 0 || dst_w == 0 || dst_h == 0 {
            return;
        }
        let copy_w = src_w.min(dst_w).min(dst_stride);
        let copy_h = src_h.min(dst_h);
        for row in 0..copy_h {
            let d_off = row * dst_stride;
            if d_off + copy_w > dst_cb.len() || d_off + copy_w > dst_cr.len() {
                break;
            }
            let s_row = std::slice::from_raw_parts(src.add(row * src_stride), copy_w * 2);
            for x in 0..copy_w {
                dst_cb[d_off + x] = s_row[x * 2];
                dst_cr[d_off + x] = s_row[x * 2 + 1];
            }
            if copy_w < dst_w {
                let pad_end = dst_w.min(dst_stride);
                if d_off + pad_end <= dst_cb.len() && d_off + pad_end <= dst_cr.len() {
                    let edge_cb = dst_cb[d_off + copy_w - 1];
                    let edge_cr = dst_cr[d_off + copy_w - 1];
                    dst_cb[d_off + copy_w..d_off + pad_end].fill(edge_cb);
                    dst_cr[d_off + copy_w..d_off + pad_end].fill(edge_cr);
                }
            }
        }
        if copy_h > 0 && copy_h < dst_h {
            let last_off = (copy_h - 1) * dst_stride;
            let row_len = dst_w.min(dst_stride);
            if last_off + row_len <= dst_cb.len() && last_off + row_len <= dst_cr.len() {
                for row in copy_h..dst_h {
                    let d_off = row * dst_stride;
                    if d_off + row_len > dst_cb.len() || d_off + row_len > dst_cr.len() {
                        break;
                    }
                    dst_cb.copy_within(last_off..last_off + row_len, d_off);
                    dst_cr.copy_within(last_off..last_off + row_len, d_off);
                }
            }
        }
    }

    unsafe extern "C" fn vt_output_callback(
        _decompression_output_ref_con: *mut c_void,
        source_frame_ref_con: *mut c_void,
        status: OSStatus,
        _info_flags: VTDecodeInfoFlags,
        image_buffer: CVImageBufferRef,
        _pts: CMTime,
        _duration: CMTime,
    ) {
        if source_frame_ref_con.is_null() {
            return;
        }
        let target = &mut *(source_frame_ref_con as *mut DecodeTarget<'_>);
        target.status = status;
        if status != 0 || image_buffer.is_null() {
            return;
        }
        if CVPixelBufferLockBaseAddress(image_buffer, K_CV_PIXEL_BUFFER_LOCK_READ_ONLY) != 0 {
            return;
        }
        let plane_count = CVPixelBufferGetPlaneCount(image_buffer);
        let crcb_w = target.ya_width >> 1;
        let crcb_h = target.ya_height >> 1;

        if plane_count >= 3 {
            // 3-plane planar 4:2:0: plane 0 = Y, plane 1 = Cb (U), plane 2 = Cr (V)
            let y_ptr = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0) as *const u8;
            let y_w = CVPixelBufferGetWidthOfPlane(image_buffer, 0);
            let y_h = CVPixelBufferGetHeightOfPlane(image_buffer, 0);
            let y_s = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
            copy_plane_with_pad(
                y_ptr,
                y_w,
                y_h,
                y_s,
                target.dst_y,
                target.ya_width,
                target.ya_height,
                target.ya_pitch,
            );

            let cb_ptr = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1) as *const u8;
            let cb_w = CVPixelBufferGetWidthOfPlane(image_buffer, 1);
            let cb_h = CVPixelBufferGetHeightOfPlane(image_buffer, 1);
            let cb_s = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
            copy_plane_with_pad(
                cb_ptr,
                cb_w,
                cb_h,
                cb_s,
                target.dst_cb,
                crcb_w,
                crcb_h,
                target.crcb_pitch,
            );

            let cr_ptr = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 2) as *const u8;
            let cr_w = CVPixelBufferGetWidthOfPlane(image_buffer, 2);
            let cr_h = CVPixelBufferGetHeightOfPlane(image_buffer, 2);
            let cr_s = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 2);
            copy_plane_with_pad(
                cr_ptr,
                cr_w,
                cr_h,
                cr_s,
                target.dst_cr,
                crcb_w,
                crcb_h,
                target.crcb_pitch,
            );
            target.ok = true;
        } else if plane_count == 2 {
            // 2-plane NV12 4:2:0: plane 0 = Y, plane 1 = interleaved CbCr
            let y_ptr = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 0) as *const u8;
            let y_w = CVPixelBufferGetWidthOfPlane(image_buffer, 0);
            let y_h = CVPixelBufferGetHeightOfPlane(image_buffer, 0);
            let y_s = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 0);
            copy_plane_with_pad(
                y_ptr,
                y_w,
                y_h,
                y_s,
                target.dst_y,
                target.ya_width,
                target.ya_height,
                target.ya_pitch,
            );

            let uv_ptr = CVPixelBufferGetBaseAddressOfPlane(image_buffer, 1) as *const u8;
            let uv_w = CVPixelBufferGetWidthOfPlane(image_buffer, 1);
            let uv_h = CVPixelBufferGetHeightOfPlane(image_buffer, 1);
            let uv_s = CVPixelBufferGetBytesPerRowOfPlane(image_buffer, 1);
            deinterleave_nv12_chroma(
                uv_ptr,
                uv_w,
                uv_h,
                uv_s,
                target.dst_cb,
                target.dst_cr,
                crcb_w,
                crcb_h,
                target.crcb_pitch,
            );
            target.ok = true;
        }

        CVPixelBufferUnlockBaseAddress(image_buffer, K_CV_PIXEL_BUFFER_LOCK_READ_ONLY);
    }

    pub struct VtHevcDecoder {
        format_desc: CMFormatDescriptionRef,
        session: VTDecompressionSessionRef,
        full_range: bool,
        pts_counter: i64,
    }

    unsafe impl Send for VtHevcDecoder {}

    impl VtHevcDecoder {
        unsafe fn create_session(
            format_desc: CMFormatDescriptionRef,
            full_range: bool,
        ) -> Result<VTDecompressionSessionRef, String> {
            let preferred_fmts = if full_range {
                [
                    K_CV_PIXEL_FORMAT_420_PLANAR_FULL,
                    K_CV_PIXEL_FORMAT_420_BIPLANAR_FULL,
                ]
            } else {
                [
                    K_CV_PIXEL_FORMAT_420_PLANAR,
                    K_CV_PIXEL_FORMAT_420_BIPLANAR,
                ]
            };

            let cb = VTDecompressionOutputCallbackRecord {
                decompression_output_callback: vt_output_callback,
                decompression_output_ref_con: std::ptr::null_mut(),
            };

            let mut session: VTDecompressionSessionRef = std::ptr::null_mut();
            let mut last_st = -1;
            for fmt_val in preferred_fmts {
                let num = CFNumberCreate(
                    kCFAllocatorDefault,
                    K_CF_NUMBER_SINT32_TYPE,
                    &fmt_val as *const i32 as *const c_void,
                );
                let keys = [kCVPixelBufferPixelFormatTypeKey as *const c_void];
                let vals = [num as *const c_void];
                let attrs = CFDictionaryCreate(
                    kCFAllocatorDefault,
                    keys.as_ptr(),
                    vals.as_ptr(),
                    1,
                    &kCFTypeDictionaryKeyCallBacks as *const u8,
                    &kCFTypeDictionaryValueCallBacks as *const u8,
                );
                last_st = VTDecompressionSessionCreate(
                    kCFAllocatorDefault,
                    format_desc,
                    std::ptr::null(),
                    attrs,
                    &cb,
                    &mut session,
                );
                if !attrs.is_null() {
                    CFRelease(attrs);
                }
                if !num.is_null() {
                    CFRelease(num);
                }
                if last_st == 0 && !session.is_null() {
                    return Ok(session);
                }
            }
            Err(format!("VTDecompressionSessionCreate failed (OSStatus {last_st})"))
        }

        unsafe fn recreate_session(&mut self) -> Result<(), String> {
            if !self.session.is_null() {
                VTDecompressionSessionInvalidate(self.session);
                CFRelease(self.session as CFTypeRef);
                self.session = std::ptr::null_mut();
            }
            self.session = Self::create_session(self.format_desc, self.full_range)?;
            Ok(())
        }

        pub fn new(
            vps: &[u8],
            sps: &[u8],
            pps: &[u8],
            full_range: bool,
        ) -> Result<Self, String> {
            unsafe {
                let ptrs = [vps.as_ptr(), sps.as_ptr(), pps.as_ptr()];
                let sizes = [vps.len(), sps.len(), pps.len()];
                let mut format_desc: CMFormatDescriptionRef = std::ptr::null();
                let st = CMVideoFormatDescriptionCreateFromHEVCParameterSets(
                    kCFAllocatorDefault,
                    3,
                    ptrs.as_ptr(),
                    sizes.as_ptr(),
                    4,
                    std::ptr::null(),
                    &mut format_desc,
                );
                if st != 0 || format_desc.is_null() {
                    return Err(format!(
                        "CMVideoFormatDescriptionCreateFromHEVCParameterSets failed (OSStatus {st})"
                    ));
                }

                let session = match Self::create_session(format_desc, full_range) {
                    Ok(s) => s,
                    Err(e) => {
                        CFRelease(format_desc);
                        return Err(e);
                    }
                };

                Ok(Self {
                    format_desc,
                    session,
                    full_range,
                    pts_counter: 1,
                })
            }
        }

        pub fn decode_hvcc_frame(
            &mut self,
            hvcc_packet: &[u8],
            ya_width: usize,
            ya_height: usize,
            ya_pitch: usize,
            crcb_pitch: usize,
            dst_y: &mut [u8],
            dst_cr: &mut [u8],
            dst_cb: &mut [u8],
        ) -> Result<(), String> {
            if hvcc_packet.len() < 6 {
                return Err("HVCC packet too short".to_string());
            }
            unsafe {
                let mut block_buf: CMBlockBufferRef = std::ptr::null_mut();
                let st = CMBlockBufferCreateWithMemoryBlock(
                    kCFAllocatorDefault,
                    hvcc_packet.as_ptr() as *mut c_void,
                    hvcc_packet.len(),
                    kCFAllocatorNull,
                    std::ptr::null(),
                    0,
                    hvcc_packet.len(),
                    0,
                    &mut block_buf,
                );
                if st != 0 || block_buf.is_null() {
                    return Err(format!(
                        "CMBlockBufferCreateWithMemoryBlock failed (OSStatus {st})"
                    ));
                }

                let pts = self.pts_counter;
                self.pts_counter = self.pts_counter.wrapping_add(1).max(1);
                let timing = CMSampleTimingInfo {
                    duration: CMTime {
                        value: 1,
                        timescale: 1000,
                        flags: 1,
                        epoch: 0,
                    },
                    presentation_time_stamp: CMTime {
                        value: pts,
                        timescale: 1000,
                        flags: 1,
                        epoch: 0,
                    },
                    decode_time_stamp: K_CM_TIME_INVALID,
                };
                let sample_size = hvcc_packet.len();
                let mut sample_buf: CMSampleBufferRef = std::ptr::null_mut();
                let st2 = CMSampleBufferCreateReady(
                    kCFAllocatorDefault,
                    block_buf,
                    self.format_desc,
                    1,
                    1,
                    &timing,
                    1,
                    &sample_size,
                    &mut sample_buf,
                );
                if st2 != 0 || sample_buf.is_null() {
                    CFRelease(block_buf as CFTypeRef);
                    return Err(format!(
                        "CMSampleBufferCreateReady failed (OSStatus {st2})"
                    ));
                }

                let mut target = DecodeTarget {
                    ya_width,
                    ya_height,
                    ya_pitch,
                    crcb_pitch,
                    dst_y,
                    dst_cr,
                    dst_cb,
                    ok: false,
                    status: 0,
                };
                let mut info_flags: VTDecodeInfoFlags = 0;
                // decodeFlags = 0 => synchronous decode (callback runs before return).
                // On iOS, backgrounding / Control Center invalidates hardware VT sessions
                // (`kVTInvalidSessionErr = -12903` or `kVTVideoDecoderMalfunctionErr = -12911`);
                // recreate the session once and retry seamlessly.
                let mut st3 = VTDecompressionSessionDecodeFrame(
                    self.session,
                    sample_buf,
                    0,
                    (&mut target as *mut DecodeTarget<'_>) as *mut c_void,
                    &mut info_flags,
                );
                let session_invalid = matches!(st3, -12903 | -12911)
                    || (!target.ok && matches!(target.status, -12903 | -12911));
                if session_invalid && self.recreate_session().is_ok() {
                    target.ok = false;
                    target.status = 0;
                    info_flags = 0;
                    st3 = VTDecompressionSessionDecodeFrame(
                        self.session,
                        sample_buf,
                        0,
                        (&mut target as *mut DecodeTarget<'_>) as *mut c_void,
                        &mut info_flags,
                    );
                }

                CFRelease(sample_buf as CFTypeRef);
                CFRelease(block_buf as CFTypeRef);

                if st3 != 0 {
                    return Err(format!(
                        "VTDecompressionSessionDecodeFrame failed (OSStatus {st3})"
                    ));
                }
                if !target.ok {
                    return Err(format!(
                        "VideoToolbox output callback did not produce a planar buffer (status {})",
                        target.status
                    ));
                }
                Ok(())
            }
        }
    }

    impl Drop for VtHevcDecoder {
        fn drop(&mut self) {
            unsafe {
                if !self.session.is_null() {
                    VTDecompressionSessionInvalidate(self.session);
                    CFRelease(self.session as CFTypeRef);
                    self.session = std::ptr::null_mut();
                }
                if !self.format_desc.is_null() {
                    CFRelease(self.format_desc);
                    self.format_desc = std::ptr::null();
                }
            }
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use apple::VtHevcDecoder;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub struct VtHevcDecoder;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
impl VtHevcDecoder {
    pub fn new(_vps: &[u8], _sps: &[u8], _pps: &[u8], _full_range: bool) -> Result<Self, String> {
        Err("VideoToolbox HEVC decoding requires macOS or iOS".to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn decode_hvcc_frame(
        &mut self,
        _hvcc_packet: &[u8],
        _ya_width: usize,
        _ya_height: usize,
        _ya_pitch: usize,
        _crcb_pitch: usize,
        _dst_y: &mut [u8],
        _dst_cr: &mut [u8],
        _dst_cb: &mut [u8],
    ) -> Result<(), String> {
        Err("VideoToolbox HEVC decoding requires macOS or iOS".to_string())
    }
}
