//! Unit tests for `mfplat.dll`.

use super::attributes::{attr_set_double, clear_propvariant_local, MF_VIDEO_FORMAT_NV12};
use super::buffer::{buf_lock, buf_set_current_length, buf_unlock};
use super::*;
use crate::com::{com_call_qi, com_call_release, IID_IUnknown};
use crate::d3d11::types::{
    D3D11_TEXTURE2D_DESC, DXGI_SAMPLE_DESC, IID_ID3D11Texture2D, IID_IDXGISurface,
};
use crate::misc::ole32::CoTaskMemFree;
use crate::types::{E_INVALIDARG, FALSE, HANDLE, TRUE};

unsafe fn vslot<F: Copy>(obj: *mut c_void, idx: usize) -> F {
    let vt = std::ptr::read_unaligned(obj as *const *const usize);
    std::mem::transmute_copy(&std::ptr::read_unaligned(vt.add(idx)))
}

#[test]
fn startup_and_shutdown_refcounted() {
    unsafe {
        assert_eq!(MFStartup(0, 0), MF_E_BAD_STARTUP_VERSION);
        assert_eq!(MFStartup(0x0002_0001, 0), MF_E_BAD_STARTUP_VERSION);
        let before = startup_ref_count();
        assert_eq!(MFStartup(MF_VERSION, MFSTARTUP_FULL), S_OK);
        assert_eq!(startup_ref_count(), before + 1);
        assert_eq!(MFStartup((1 << 16) | MF_API_VERSION, MFSTARTUP_NOSOCKET), S_OK);
        assert_eq!(startup_ref_count(), before + 2);
        assert_eq!(MFShutdown(), S_OK);
        assert_eq!(startup_ref_count(), before + 1);
        assert_eq!(MFShutdown(), S_OK);
        assert_eq!(startup_ref_count(), before);
    }
}

#[test]
fn copy_image_positive_and_negative_strides() {
    unsafe {
        // 4x3 image with 2 bytes per row padding in src (stride=6) -> bottom-up dst (stride=-4).
        let src: [u8; 18] = [
            1, 2, 3, 4, 0xAA, 0xBB, // row 0
            5, 6, 7, 8, 0xCC, 0xDD, // row 1
            9, 10, 11, 12, 0xEE, 0xFF, // row 2
        ];
        let mut dst = [0u8; 12];
        // Top-down copy first
        assert_eq!(MFCopyImage(dst.as_mut_ptr(), 4, src.as_ptr(), 6, 4, 3), S_OK);
        assert_eq!(dst, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

        // Negative destination stride (bottom-up flip): row 0 goes to end, row 2 goes to start.
        let mut flipped = [0u8; 12];
        let dst_last_row = flipped.as_mut_ptr().add(8);
        assert_eq!(MFCopyImage(dst_last_row, -4, src.as_ptr(), 6, 4, 3), S_OK);
        assert_eq!(flipped, [9, 10, 11, 12, 5, 6, 7, 8, 1, 2, 3, 4]);

        // Negative source stride back to positive destination restores original order.
        let mut restored = [0u8; 12];
        assert_eq!(
            MFCopyImage(restored.as_mut_ptr(), 4, flipped.as_ptr().add(8), -4, 4, 3),
            S_OK
        );
        assert_eq!(restored, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }
}

#[test]
fn memory_buffer_and_2d_buffer_roundtrip() {
    unsafe {
        let mut buf: *mut c_void = std::ptr::null_mut();
        assert_eq!(MFCreateMemoryBuffer(64, &mut buf), S_OK);
        assert!(!buf.is_null());

        let lock: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut u32, *mut u32) -> HRESULT =
            vslot(buf, 3);
        let unlock: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(buf, 4);
        let get_cur: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(buf, 5);
        let set_cur: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT = vslot(buf, 6);
        let get_max: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(buf, 7);

        let mut max_len = 0u32;
        let mut cur_len = 99u32;
        assert_eq!(get_max(buf, &mut max_len), S_OK);
        assert_eq!(max_len, 64);
        assert_eq!(get_cur(buf, &mut cur_len), S_OK);
        assert_eq!(cur_len, 0);
        assert_eq!(set_cur(buf, 65), E_INVALIDARG);
        assert_eq!(set_cur(buf, 16), S_OK);

        let mut ptr: *mut u8 = std::ptr::null_mut();
        assert_eq!(lock(buf, &mut ptr, &mut max_len, &mut cur_len), S_OK);
        assert_eq!(max_len, 64);
        assert_eq!(cur_len, 16);
        for i in 0..16u8 {
            *ptr.add(i as usize) = i.wrapping_mul(3);
        }
        assert_eq!(unlock(buf), S_OK);

        // QI for IMF2DBuffer and verify COM identity round-trip + Lock2D/ContiguousCopyTo.
        let mut buf2d: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi(buf, &IID_IMF2DBuffer, &mut buf2d), S_OK);
        assert_ne!(buf2d, buf);
        let mut unk_from_2d: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi(buf2d, &IID_IUnknown, &mut unk_from_2d), S_OK);
        assert_eq!(unk_from_2d, buf);
        assert_eq!(com_call_release(unk_from_2d), 2);

        let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT =
            vslot(buf2d, 3);
        let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(buf2d, 4);
        let get_s0: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT =
            vslot(buf2d, 5);
        let copy_to: unsafe extern "win64" fn(*mut c_void, *mut u8, u32) -> HRESULT =
            vslot(buf2d, 8);

        let mut s0 = std::ptr::null_mut();
        let mut pitch = 0i32;
        assert_eq!(get_s0(buf2d, &mut s0, &mut pitch), MF_E_INVALIDREQUEST);
        assert_eq!(lock2d(buf2d, &mut s0, &mut pitch), S_OK);
        assert_eq!(pitch, 64);
        assert_eq!(*s0.add(5), 15);
        assert_eq!(get_s0(buf2d, &mut s0, &mut pitch), S_OK);
        assert_eq!(unlock2d(buf2d), S_OK);

        let mut out = [0u8; 64];
        assert_eq!(copy_to(buf2d, out.as_mut_ptr(), 64), S_OK);
        assert_eq!(out[5], 15);

        assert_eq!(com_call_release(buf2d), 1);
        assert_eq!(com_call_release(buf), 0);
    }
}

#[test]
fn attributes_and_media_type_all_value_types() {
    unsafe {
        let mut mt: *mut c_void = std::ptr::null_mut();
        assert_eq!(MFCreateMediaType(&mut mt), S_OK);

        let k_u32 = guid("10000001-0000-0000-0000-000000000001");
        let k_u64 = guid("10000002-0000-0000-0000-000000000002");
        let k_f64 = guid("10000003-0000-0000-0000-000000000003");
        let k_guid = guid("10000004-0000-0000-0000-000000000004");
        let k_str = guid("10000005-0000-0000-0000-000000000005");
        let k_blob = guid("10000006-0000-0000-0000-000000000006");
        let k_unk = guid("10000007-0000-0000-0000-000000000007");

        let get_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT =
            vslot(mt, 7);
        let get_u64: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u64) -> HRESULT =
            vslot(mt, 8);
        let get_f64: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut f64) -> HRESULT =
            vslot(mt, 9);
        let get_guid: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut Guid) -> HRESULT =
            vslot(mt, 10);
        let get_str: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u16, u32, *mut u32) -> HRESULT =
            vslot(mt, 12);
        let get_blob: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u8, u32, *mut u32) -> HRESULT =
            vslot(mt, 15);
        let get_unk: unsafe extern "win64" fn(*mut c_void, *const Guid, *const Guid, *mut *mut c_void) -> HRESULT =
            vslot(mt, 17);
        let set_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, u32) -> HRESULT =
            vslot(mt, 21);
        let set_u64: unsafe extern "win64" fn(*mut c_void, *const Guid, u64) -> HRESULT =
            vslot(mt, 22);
        let set_guid: unsafe extern "win64" fn(*mut c_void, *const Guid, *const Guid) -> HRESULT =
            vslot(mt, 24);
        let set_str: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u16) -> HRESULT =
            vslot(mt, 25);
        let set_blob: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8, u32) -> HRESULT =
            vslot(mt, 26);
        let set_unk: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut c_void) -> HRESULT =
            vslot(mt, 27);
        let lock_store: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(mt, 28);
        let unlock_store: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(mt, 29);
        let get_count: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(mt, 30);
        let copy_all: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT = vslot(mt, 32);
        let get_major: unsafe extern "win64" fn(*mut c_void, *mut Guid) -> HRESULT = vslot(mt, 33);
        let is_comp: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT = vslot(mt, 34);
        let is_eq: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut u32) -> HRESULT =
            vslot(mt, 35);

        // Reentrant LockStore should not deadlock nested writes.
        assert_eq!(lock_store(mt), S_OK);
        let slot23: usize = vslot(mt, 23);
        assert_eq!(slot23, crate::bridge_entry!(attr_set_double) as usize);
        assert_eq!(set_u32(mt, &k_u32, 0x1234_5678), S_OK);
        assert_eq!(set_u64(mt, &k_u64, 0xDEAD_BEEF_CAFE_BABE), S_OK);
        assert_eq!(attr_set_double(mt as usize, &k_f64, 3.25), S_OK);
        assert_eq!(set_guid(mt, &k_guid, &MF_VIDEO_FORMAT_NV12), S_OK);
        assert_eq!(set_guid(mt, &MF_MT_MAJOR_TYPE, &k_guid), S_OK);
        assert_eq!(set_guid(mt, &MF_MT_SUBTYPE, &MF_VIDEO_FORMAT_NV12), S_OK);
        let w: Vec<u16> = "HelloMF\0".encode_utf16().collect();
        assert_eq!(set_str(mt, &k_str, w.as_ptr()), S_OK);
        let blob = [10u8, 20, 30, 40, 50];
        assert_eq!(set_blob(mt, &k_blob, blob.as_ptr(), blob.len() as u32), S_OK);

        let mut sub_attr: *mut c_void = std::ptr::null_mut();
        assert_eq!(MFCreateAttributes(&mut sub_attr, 4), S_OK);
        assert_eq!(set_unk(mt, &k_unk, sub_attr), S_OK);
        assert_eq!(unlock_store(mt), S_OK);

        let mut v32 = 0u32;
        assert_eq!(get_u32(mt, &k_u32, &mut v32), S_OK);
        assert_eq!(v32, 0x1234_5678);
        let mut v64 = 0u64;
        assert_eq!(get_u64(mt, &k_u64, &mut v64), S_OK);
        assert_eq!(v64, 0xDEAD_BEEF_CAFE_BABE);
        let mut vf64 = 0.0f64;
        assert_eq!(get_f64(mt, &k_f64, &mut vf64), S_OK);
        assert!((vf64 - 3.25).abs() < 1e-12);
        let mut vg = Guid::default();
        assert_eq!(get_guid(mt, &k_guid, &mut vg), S_OK);
        assert_eq!(vg, MF_VIDEO_FORMAT_NV12);

        let mut sbuf = [0u16; 16];
        let mut slen = 0u32;
        assert_eq!(get_str(mt, &k_str, sbuf.as_mut_ptr(), 16, &mut slen), S_OK);
        assert_eq!(slen, 7);
        assert_eq!(String::from_utf16_lossy(&sbuf[..7]), "HelloMF");

        let mut bbuf = [0u8; 8];
        let mut blen = 0u32;
        assert_eq!(get_blob(mt, &k_blob, bbuf.as_mut_ptr(), 2, &mut blen), MF_E_BUFFERTOOSMALL);
        assert_eq!(blen, 5);
        assert_eq!(get_blob(mt, &k_blob, bbuf.as_mut_ptr(), 8, &mut blen), S_OK);
        assert_eq!(&bbuf[..5], &blob);

        let mut got_unk: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_unk(mt, &k_unk, &IID_IMFAttributes, &mut got_unk), S_OK);
        assert_eq!(got_unk, sub_attr);
        assert_eq!(com_call_release(got_unk), 2);

        // MediaType methods
        let mut major = Guid::default();
        assert_eq!(get_major(mt, &mut major), S_OK);
        assert_eq!(major, k_guid);
        let mut comp = TRUE;
        assert_eq!(is_comp(mt, &mut comp), S_OK);
        assert_eq!(comp, FALSE); // NV12 is uncompressed

        // CopyAllItems into another IMFMediaType + IsEqual
        let mut mt2: *mut c_void = std::ptr::null_mut();
        assert_eq!(MFCreateMediaType(&mut mt2), S_OK);
        assert_eq!(copy_all(mt, mt2), S_OK);
        let mut cnt = 0u32;
        assert_eq!(get_count(mt2, &mut cnt), S_OK);
        assert_eq!(cnt, 9);
        let mut eq_flags = 0u32;
        assert_eq!(is_eq(mt, mt2, &mut eq_flags), S_OK);
        assert_eq!(
            eq_flags,
            MF_MEDIATYPE_EQUAL_MAJOR_TYPES
                | MF_MEDIATYPE_EQUAL_FORMAT_TYPES
                | MF_MEDIATYPE_EQUAL_FORMAT_DATA
                | MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA
        );

        // Exercise GetItem/SetItem (PROPVARIANT), GetItemType, Compare, CompareItem,
        // GetAllocatedString, GetAllocatedBlob, GetItemByIndex, DeleteItem, DeleteAllItems.
        let get_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u8) -> HRESULT =
            vslot(mt, 3);
        let get_type: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT =
            vslot(mt, 4);
        let cmp_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8, *mut BOOL) -> HRESULT =
            vslot(mt, 5);
        let cmp_all: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *mut BOOL) -> HRESULT =
            vslot(mt, 6);
        let get_str_len: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT =
            vslot(mt, 11);
        let get_alloc_str: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut u16, *mut u32) -> HRESULT =
            vslot(mt, 13);
        let get_blob_sz: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT =
            vslot(mt, 14);
        let get_alloc_blob: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut u8, *mut u32) -> HRESULT =
            vslot(mt, 16);
        let set_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8) -> HRESULT =
            vslot(mt, 18);
        let del_item: unsafe extern "win64" fn(*mut c_void, *const Guid) -> HRESULT = vslot(mt, 19);
        let del_all: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(mt, 20);
        let get_by_idx: unsafe extern "win64" fn(*mut c_void, u32, *mut Guid, *mut u8) -> HRESULT =
            vslot(mt, 31);

        let mut ty = 0u32;
        assert_eq!(get_type(mt, &k_str, &mut ty), S_OK);
        assert_eq!(ty, MF_ATTRIBUTE_STRING);
        let mut pv = [0u8; 24];
        assert_eq!(get_item(mt, &k_str, pv.as_mut_ptr()), S_OK);
        let mut eq_b = FALSE;
        assert_eq!(cmp_item(mt, &k_str, pv.as_ptr(), &mut eq_b), S_OK);
        assert_eq!(eq_b, TRUE);
        let k_copy = guid("10000099-0000-0000-0000-000000000099");
        assert_eq!(set_item(mt2, &k_copy, pv.as_ptr()), S_OK);
        clear_propvariant_local(pv.as_mut_ptr());

        let mut match_b = FALSE;
        assert_eq!(cmp_all(mt, mt2, MF_ATTRIBUTES_MATCH_OUR_ITEMS, &mut match_b), S_OK);
        assert_eq!(match_b, TRUE);
        assert_eq!(cmp_all(mt, mt2, MF_ATTRIBUTES_MATCH_ALL_ITEMS, &mut match_b), S_OK);
        assert_eq!(match_b, FALSE);

        let mut slen2 = 0u32;
        assert_eq!(get_str_len(mt, &k_str, &mut slen2), S_OK);
        assert_eq!(slen2, 7);
        let mut alloc_s: *mut u16 = std::ptr::null_mut();
        let mut alloc_slen = 0u32;
        assert_eq!(get_alloc_str(mt, &k_str, &mut alloc_s, &mut alloc_slen), S_OK);
        assert_eq!(alloc_slen, 7);
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(alloc_s, 7)), "HelloMF");
        CoTaskMemFree(alloc_s as *mut c_void);

        let mut bsz = 0u32;
        assert_eq!(get_blob_sz(mt, &k_blob, &mut bsz), S_OK);
        assert_eq!(bsz, 5);
        let mut alloc_b: *mut u8 = std::ptr::null_mut();
        let mut alloc_bsz = 0u32;
        assert_eq!(get_alloc_blob(mt, &k_blob, &mut alloc_b, &mut alloc_bsz), S_OK);
        assert_eq!(alloc_bsz, 5);
        assert_eq!(std::slice::from_raw_parts(alloc_b, 5), &blob);
        CoTaskMemFree(alloc_b as *mut c_void);

        let mut idx_key = Guid::default();
        assert_eq!(get_by_idx(mt, 0, &mut idx_key, std::ptr::null_mut()), S_OK);
        assert_eq!(idx_key, k_u32);

        assert_eq!(del_item(mt2, &k_copy), S_OK);
        assert_eq!(get_type(mt2, &k_copy, &mut ty), MF_E_ATTRIBUTENOTFOUND);
        assert_eq!(del_all(mt2), S_OK);
        assert_eq!(get_count(mt2, &mut cnt), S_OK);
        assert_eq!(cnt, 0);

        assert_eq!(com_call_release(mt2), 0);
        assert_eq!(com_call_release(mt), 0);
        assert_eq!(com_call_release(sub_attr), 0);
    }
}

#[test]
fn aligned_and_2d_media_buffer_top_down_and_bottom_up() {
    unsafe {
        let mut abuf = std::ptr::null_mut();
        assert_eq!(MFCreateAlignedMemoryBuffer(128, 255, &mut abuf), S_OK);
        let (aptr, amax, _) = buf_lock(abuf).unwrap();
        assert_eq!(amax, 128);
        assert_eq!((aptr as usize) % 256, 0);
        assert_eq!(buf_unlock(abuf), S_OK);
        assert_eq!(com_call_release(abuf), 0);

        // 4x3 RGBA8 bottom-up 2D buffer (row_bytes = 16, height = 3)
        let mut bu_m = std::ptr::null_mut();
        let mut td_m = std::ptr::null_mut();
        assert_eq!(MFCreate2DMediaBuffer(4, 3, 21, TRUE, &mut bu_m), S_OK);
        assert_eq!(MFCreate2DMediaBuffer(4, 3, 21, FALSE, &mut td_m), S_OK);

        let mut bu2d = std::ptr::null_mut();
        let mut td2d = std::ptr::null_mut();
        assert_eq!(com_call_qi(bu_m, &IID_IMF2DBuffer2, &mut bu2d), S_OK);
        assert_eq!(com_call_qi(td_m, &IID_IMF2DBuffer2, &mut td2d), S_OK);

        let is_contig: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT = vslot(bu2d, 6);
        let get_clen: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(bu2d, 7);
        let copy_to: unsafe extern "win64" fn(*mut c_void, *mut u8, u32) -> HRESULT =
            vslot(bu2d, 8);
        let copy_from: unsafe extern "win64" fn(*mut c_void, *const u8, u32) -> HRESULT =
            vslot(bu2d, 9);
        let lock2d_sz: unsafe extern "win64" fn(
            *mut c_void,
            u32,
            *mut *mut u8,
            *mut i32,
            *mut *mut u8,
            *mut u32,
        ) -> HRESULT = vslot(bu2d, 10);
        let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(bu2d, 4);
        let copy2d_to: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT =
            vslot(bu2d, 11);

        let mut cflag = TRUE;
        assert_eq!(is_contig(bu2d, &mut cflag), S_OK);
        assert_eq!(cflag, FALSE);
        assert_eq!(is_contig(td2d, &mut cflag), S_OK);
        assert_eq!(cflag, TRUE);

        let mut clen = 0u32;
        assert_eq!(get_clen(bu2d, &mut clen), S_OK);
        assert_eq!(clen, 48);

        let src: Vec<u8> = (0u8..48).collect();
        assert_eq!(copy_from(bu2d, src.as_ptr(), 48), S_OK);

        let mut scan0 = std::ptr::null_mut();
        let mut pitch = 0i32;
        let mut start = std::ptr::null_mut();
        let mut blen = 0u32;
        assert_eq!(lock2d_sz(bu2d, 0, &mut scan0, &mut pitch, &mut start, &mut blen), S_OK);
        assert_eq!(pitch, -16);
        assert_eq!(blen, 48);
        assert_eq!(scan0, start.add(32));
        assert_eq!(std::slice::from_raw_parts(scan0, 16), &src[0..16]);
        assert_eq!(std::slice::from_raw_parts(start, 16), &src[32..48]);
        assert_eq!(unlock2d(bu2d), S_OK);

        // Copy2DTo from bottom-up -> top-down, then ContiguousCopyTo back out.
        assert_eq!(copy2d_to(bu2d, td2d), S_OK);
        let mut out = [0u8; 48];
        assert_eq!(copy_to(td2d, out.as_mut_ptr(), 48), S_OK);
        assert_eq!(out.as_slice(), src.as_slice());

        assert_eq!(com_call_release(bu2d), 1);
        assert_eq!(com_call_release(bu_m), 0);
        assert_eq!(com_call_release(td2d), 1);
        assert_eq!(com_call_release(td_m), 0);
    }
}

#[test]
fn sample_buffers_flatten_timestamps_and_attributes() {
    unsafe {
        let mut sample: *mut c_void = std::ptr::null_mut();
        assert_eq!(MFCreateSample(&mut sample), S_OK);

        let get_flags: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
            vslot(sample, 33);
        let set_flags: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT = vslot(sample, 34);
        let get_time: unsafe extern "win64" fn(*mut c_void, *mut i64) -> HRESULT =
            vslot(sample, 35);
        let set_time: unsafe extern "win64" fn(*mut c_void, i64) -> HRESULT = vslot(sample, 36);
        let get_dur: unsafe extern "win64" fn(*mut c_void, *mut i64) -> HRESULT = vslot(sample, 37);
        let set_dur: unsafe extern "win64" fn(*mut c_void, i64) -> HRESULT = vslot(sample, 38);
        let get_cnt: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(sample, 39);
        let get_by_idx: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT =
            vslot(sample, 40);
        let convert: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
            vslot(sample, 41);
        let add_buf: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT =
            vslot(sample, 42);
        let rem_by_idx: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT = vslot(sample, 43);
        let rem_all: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(sample, 44);
        let get_total: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
            vslot(sample, 45);
        let copy_to_buf: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT =
            vslot(sample, 46);
        let set_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, u32) -> HRESULT =
            vslot(sample, 21);
        let get_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT =
            vslot(sample, 7);

        let mut flags = 99u32;
        assert_eq!(get_flags(sample, &mut flags), S_OK);
        assert_eq!(flags, 0);
        assert_eq!(set_flags(sample, 0x5A), S_OK);
        assert_eq!(get_flags(sample, &mut flags), S_OK);
        assert_eq!(flags, 0x5A);

        let mut t = 0i64;
        assert_eq!(get_time(sample, &mut t), MF_E_NO_SAMPLE_TIMESTAMP);
        assert_eq!(get_dur(sample, &mut t), MF_E_NO_SAMPLE_DURATION);
        assert_eq!(set_time(sample, 100_000), S_OK);
        assert_eq!(set_dur(sample, 333_333), S_OK);
        assert_eq!(get_time(sample, &mut t), S_OK);
        assert_eq!(t, 100_000);
        assert_eq!(get_dur(sample, &mut t), S_OK);
        assert_eq!(t, 333_333);

        // Inherited IMFAttributes on IMFSample
        let k = guid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee");
        assert_eq!(set_u32(sample, &k, 77), S_OK);
        let mut v = 0u32;
        assert_eq!(get_u32(sample, &k, &mut v), S_OK);
        assert_eq!(v, 77);

        // Create two buffers and flatten via ConvertToContiguousBuffer
        let mut b1 = std::ptr::null_mut();
        let mut b2 = std::ptr::null_mut();
        assert_eq!(MFCreateMemoryBuffer(8, &mut b1), S_OK);
        assert_eq!(MFCreateMemoryBuffer(8, &mut b2), S_OK);
        let (p1, _, _) = buf_lock(b1).unwrap();
        std::ptr::copy_nonoverlapping([1u8, 2, 3, 4].as_ptr(), p1, 4);
        assert_eq!(buf_unlock(b1), S_OK);
        assert_eq!(buf_set_current_length(b1, 4), S_OK);

        let (p2, _, _) = buf_lock(b2).unwrap();
        std::ptr::copy_nonoverlapping([5u8, 6, 7].as_ptr(), p2, 3);
        assert_eq!(buf_unlock(b2), S_OK);
        assert_eq!(buf_set_current_length(b2, 3), S_OK);

        assert_eq!(add_buf(sample, b1), S_OK);
        assert_eq!(add_buf(sample, b2), S_OK);
        assert_eq!(com_call_release(b1), 1);
        assert_eq!(com_call_release(b2), 1);

        let mut cnt = 0u32;
        assert_eq!(get_cnt(sample, &mut cnt), S_OK);
        assert_eq!(cnt, 2);
        let mut total = 0u32;
        assert_eq!(get_total(sample, &mut total), S_OK);
        assert_eq!(total, 7);

        // GetBufferByIndex + CopyToBuffer (small buffer fails, 8-byte succeeds)
        let mut got0 = std::ptr::null_mut();
        assert_eq!(get_by_idx(sample, 0, &mut got0), S_OK);
        assert_eq!(got0, b1);
        assert_eq!(com_call_release(got0), 1);

        let mut small_dst = std::ptr::null_mut();
        let mut ok_dst = std::ptr::null_mut();
        assert_eq!(MFCreateMemoryBuffer(4, &mut small_dst), S_OK);
        assert_eq!(MFCreateMemoryBuffer(8, &mut ok_dst), S_OK);
        assert_eq!(copy_to_buf(sample, small_dst), MF_E_BUFFERTOOSMALL);
        assert_eq!(copy_to_buf(sample, ok_dst), S_OK);
        let (po, _, cur_o) = buf_lock(ok_dst).unwrap();
        assert_eq!(cur_o, 7);
        assert_eq!(std::slice::from_raw_parts(po, 7), &[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(buf_unlock(ok_dst), S_OK);
        assert_eq!(com_call_release(small_dst), 0);
        assert_eq!(com_call_release(ok_dst), 0);

        let mut flat = std::ptr::null_mut();
        assert_eq!(convert(sample, &mut flat), S_OK);
        assert_eq!(get_cnt(sample, &mut cnt), S_OK);
        assert_eq!(cnt, 1);

        let (pf, _, cur) = buf_lock(flat).unwrap();
        assert_eq!(cur, 7);
        assert_eq!(std::slice::from_raw_parts(pf, 7), &[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(buf_unlock(flat), S_OK);

        assert_eq!(rem_by_idx(sample, 0), S_OK);
        assert_eq!(get_cnt(sample, &mut cnt), S_OK);
        assert_eq!(cnt, 0);
        assert_eq!(add_buf(sample, flat), S_OK);
        assert_eq!(rem_all(sample), S_OK);
        assert_eq!(get_cnt(sample, &mut cnt), S_OK);
        assert_eq!(cnt, 0);

        assert_eq!(com_call_release(flat), 0);
        assert_eq!(com_call_release(sample), 0);
    }
}

#[test]
fn dxgi_device_manager_and_surface_buffer() {
    unsafe {
        // Create a real D3D11 device + Texture2D (skipped if no Metal device).
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let mut d3d11_dev: *mut c_void = std::ptr::null_mut();
        let hr = crate::d3d11::D3D11CreateDevice(
            std::ptr::null_mut(),
            1, // D3D_DRIVER_TYPE_HARDWARE
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
            0,
            7,
            &mut d3d11_dev,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(hr, S_OK);

        // IMFDXGIDeviceManager lifecycle
        let mut token = 0u32;
        let mut mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!(MFCreateDXGIDeviceManager(&mut token, &mut mgr), S_OK);
        assert_ne!(token, 0);

        let close_h: unsafe extern "win64" fn(*mut c_void, HANDLE) -> HRESULT = vslot(mgr, 3);
        let get_svc: unsafe extern "win64" fn(
            *mut c_void,
            HANDLE,
            *const Guid,
            *mut *mut c_void,
        ) -> HRESULT = vslot(mgr, 4);
        let lock_dev: unsafe extern "win64" fn(
            *mut c_void,
            HANDLE,
            *const Guid,
            *mut *mut c_void,
            BOOL,
        ) -> HRESULT = vslot(mgr, 5);
        let open_h: unsafe extern "win64" fn(*mut c_void, *mut HANDLE) -> HRESULT = vslot(mgr, 6);
        let reset_dev: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) -> HRESULT =
            vslot(mgr, 7);
        let test_dev: unsafe extern "win64" fn(*mut c_void, HANDLE) -> HRESULT = vslot(mgr, 8);
        let unlock_dev: unsafe extern "win64" fn(*mut c_void, HANDLE, BOOL) -> HRESULT =
            vslot(mgr, 9);

        let mut h1: HANDLE = std::ptr::null_mut();
        assert_eq!(open_h(mgr, &mut h1), MF_E_DXGI_DEVICE_NOT_INITIALIZED);
        assert_eq!(reset_dev(mgr, d3d11_dev, token ^ 0xFF), E_INVALIDARG);
        assert_eq!(reset_dev(mgr, d3d11_dev, token), S_OK);
        assert_eq!(open_h(mgr, &mut h1), S_OK);
        assert_eq!(test_dev(mgr, h1), S_OK);

        let mut svc: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_svc(mgr, h1, &IID_IUnknown, &mut svc), S_OK);
        assert!(!svc.is_null());
        com_call_release(svc);

        let mut locked_dev: *mut c_void = std::ptr::null_mut();
        assert_eq!(lock_dev(mgr, h1, &IID_IUnknown, &mut locked_dev, FALSE), S_OK);
        assert!(!locked_dev.is_null());

        // Second handle cannot non-blocking lock while h1 holds it.
        let mut h2: HANDLE = std::ptr::null_mut();
        assert_eq!(open_h(mgr, &mut h2), S_OK);
        let mut locked2: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            lock_dev(mgr, h2, &IID_IUnknown, &mut locked2, FALSE),
            MF_E_DXGI_VIDEO_DEVICE_LOCKED
        );

        assert_eq!(unlock_dev(mgr, h1, FALSE), S_OK);
        assert_eq!(com_call_release(locked_dev), 2); // d3d11_dev + mgr

        // ResetDevice invalidates existing handles.
        assert_eq!(reset_dev(mgr, d3d11_dev, token), S_OK);
        assert_eq!(test_dev(mgr, h1), MF_E_DXGI_NEW_VIDEO_DEVICE);
        assert_eq!(close_h(mgr, h1), S_OK);
        assert_eq!(close_h(mgr, h2), S_OK);
        assert_eq!(close_h(mgr, h1), E_HANDLE);

        // Create a real 4x4 RGBA8 D3D11 Texture2D and wrap with MFCreateDXGISurfaceBuffer.
        let create_tex2d: unsafe extern "win64" fn(
            *mut c_void,
            *const D3D11_TEXTURE2D_DESC,
            *const c_void,
            *mut *mut c_void,
        ) -> HRESULT = vslot(d3d11_dev, 5);
        let tex_desc = D3D11_TEXTURE2D_DESC {
            Width: 4,
            Height: 4,
            MipLevels: 1,
            ArraySize: 1,
            Format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: 0,
            BindFlags: 8, // SHADER_RESOURCE
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let mut tex: *mut c_void = std::ptr::null_mut();
        assert_eq!(create_tex2d(d3d11_dev, &tex_desc, std::ptr::null(), &mut tex), S_OK);

        let mut mf_buf: *mut c_void = std::ptr::null_mut();
        assert_eq!(
            MFCreateDXGISurfaceBuffer(&IID_ID3D11Texture2D, tex, 0, FALSE, &mut mf_buf),
            S_OK
        );

        let mut dxgi_buf: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi(mf_buf, &IID_IMFDXGIBuffer, &mut dxgi_buf), S_OK);

        let get_res: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT =
            vslot(dxgi_buf, 3);
        let get_sub: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
            vslot(dxgi_buf, 4);
        let get_unk: unsafe extern "win64" fn(
            *mut c_void,
            *const Guid,
            *const Guid,
            *mut *mut c_void,
        ) -> HRESULT = vslot(dxgi_buf, 5);
        let set_unk: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut c_void) -> HRESULT =
            vslot(dxgi_buf, 6);

        let mut got_surf: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_res(dxgi_buf, &IID_IDXGISurface, &mut got_surf), S_OK);
        assert!(!got_surf.is_null());
        com_call_release(got_surf);

        let mut sub_idx = 99u32;
        assert_eq!(get_sub(dxgi_buf, &mut sub_idx), S_OK);
        assert_eq!(sub_idx, 0);

        let uk = guid("22222222-3333-4444-5555-666666666666");
        assert_eq!(set_unk(dxgi_buf, &uk, mgr), S_OK);
        let mut got_mgr: *mut c_void = std::ptr::null_mut();
        assert_eq!(get_unk(dxgi_buf, &uk, &IID_IMFDXGIDeviceManager, &mut got_mgr), S_OK);
        assert_eq!(got_mgr, mgr);
        com_call_release(got_mgr);

        // Lock2D through real IDXGISurface::Map/Unmap
        let mut buf2d: *mut c_void = std::ptr::null_mut();
        assert_eq!(com_call_qi(mf_buf, &IID_IMF2DBuffer, &mut buf2d), S_OK);
        let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT =
            vslot(buf2d, 3);
        let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(buf2d, 4);
        let mut scan0 = std::ptr::null_mut();
        let mut pitch = 0i32;
        assert_eq!(lock2d(buf2d, &mut scan0, &mut pitch), S_OK);
        assert!(!scan0.is_null());
        assert!(pitch >= 16);
        *scan0 = 0xAB;
        assert_eq!(unlock2d(buf2d), S_OK);

        com_call_release(buf2d);
        com_call_release(dxgi_buf);
        com_call_release(mf_buf);
        com_call_release(tex);
        com_call_release(mgr);
        com_call_release(d3d11_dev);
    }
}
