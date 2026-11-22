//! `rend::FSR2Glue`, `d3d::fsr2`, and `rend::FrameRateHistory` — FidelityFX Super Resolution / Frame Generation glue and frame-rate tracking.
#![allow(static_mut_refs)]

use std::sync::atomic::{AtomicU32, Ordering};

use crate::exe::{self, rd, wr};
pub use super::fsr2_glue_diff_a::*;
pub use super::fsr2_glue_diff_b::*;

// ============================================================================
// 1. Version packing & FidelityFX quality/format helpers
// ============================================================================

/// Pack a 64-bit version identifier `(((major as u64) << 16 | (minor as u64)) << 32) | (patch as u64)`.
///
/// Original: `0x14205b6f0` (24 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn pack_version_u64(minor: u16, patch: u32, major: u16) -> u64 {
    ((((major as u64) << 16) | (minor as u64)) << 32) | (patch as u64)
}

/// Map `DXGI_FORMAT` (`ecx`) to `FfxApiSurfaceFormat` (`eax`).
///
/// Original: `0x14205cbc0` (258 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_dxgi_to_ffx_format(dxgi_format: u32) -> u32 {
    match dxgi_format {
        1 => 1,
        2 => 3,
        3 => 2,
        9 => 0x1e,
        10 => 4,
        0x0f => 0x1f,
        0x10 => 6,
        0x13 | 0x14 | 0x15 | 0x28 | 0x29 => 0x1c,
        0x16 | 0x2f | 0x3e => 7,
        0x17 => 0x20,
        0x18 => 0x11,
        0x1a => 0x10,
        0x1b => 9,
        0x1c => 10,
        0x1d => 0x0c,
        0x1f => 0x0b,
        0x21 => 0x21,
        0x22 => 0x12,
        0x24 => 0x13,
        0x27 => 0x25,
        0x2a | 0x2c | 0x2d | 0x2e => 8,
        0x30 => 0x24,
        0x31 => 0x1a,
        0x32 => 0x1b,
        0x35 => 0x22,
        0x36 => 0x15,
        0x37 | 0x38 => 0x17,
        0x39 => 0x16,
        0x3a => 0x18,
        0x3c => 0x23,
        0x3d | 0x41 => 0x19,
        0x3f => 0x27,
        0x43 => 0x1d,
        0x57 => 0x0e,
        0x5a => 0x0d,
        0x5b => 0x0f,
        _ => 0,
    }
}

/// Build a 48-byte `FfxApiResource` at `out_res` (`rcx`) from `ID3D12Resource*` `d3d_res` (`rdx`)
/// and resource state `state` (`r8d`), returning `out_res`.
///
/// Original: `0x14205caa0` (274 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_get_ffx_resource(
    out_res: usize,
    d3d_res: usize,
    state: u32,
) -> usize {
    wr::<u64>(out_res + 0x08, 0);
    wr::<u64>(out_res + 0x10, 0);
    wr::<u64>(out_res + 0x18, 0);
    wr::<u64>(out_res + 0x20, 0);
    wr::<u32>(out_res + 0x2c, 0);
    wr::<usize>(out_res, d3d_res);
    wr::<u32>(out_res + 0x28, state);

    if d3d_res != 0 {
        let vtbl = rd::<usize>(d3d_res);
        let get_desc = rd::<usize>(vtbl + 0x50);
        let mut desc = [0u64; 8];
        let desc_ptr = desc.as_mut_ptr() as usize;
        let _ = exe::call_va!(get_desc, (d3d_res, desc_ptr) -> usize);
        let dimension = rd::<u32>(desc_ptr);
        if dimension != 0 {
            let format_dxgi = rd::<u32>(desc_ptr + 0x20);
            wr::<u32>(out_res + 0x20, 0);
            let mut usage: u32 = 0;
            if dimension == 1 {
                usage = 2;
                wr::<u32>(out_res + 0x10, rd::<u32>(desc_ptr + 0x10));
                wr::<u32>(out_res + 0x14, rd::<u32>(desc_ptr + 0x18));
                wr::<u32>(out_res + 0x08, 0);
            } else {
                if format_dxgi == 0x37 || format_dxgi == 0x28 {
                    usage = 4;
                } else if format_dxgi == 0x2d || format_dxgi == 0x14 {
                    usage = 0x24;
                }
                if (rd::<u8>(desc_ptr + 0x30) & 4) != 0 {
                    usage |= 2;
                }
                wr::<u32>(out_res + 0x10, rd::<u32>(desc_ptr + 0x10));
                wr::<u32>(out_res + 0x14, rd::<u32>(desc_ptr + 0x18));
                let depth_or_array = rd::<u16>(desc_ptr + 0x1c) as u32;
                let mip_levels = rd::<u16>(desc_ptr + 0x1e) as u32;
                wr::<u32>(out_res + 0x1c, mip_levels);
                wr::<u32>(out_res + 0x18, depth_or_array);
                match dimension {
                    2 => wr::<u32>(out_res + 0x08, 1),
                    3 => wr::<u32>(out_res + 0x08, if depth_or_array == 6 { 3 } else { 2 }),
                    4 => wr::<u32>(out_res + 0x08, 4),
                    _ => {}
                }
            }
            let ffx_fmt = d3d_fsr2_dxgi_to_ffx_format(format_dxgi);
            wr::<u32>(out_res + 0x0c, ffx_fmt);
            wr::<u32>(out_res + 0x24, usage);
        }
    }
    out_res
}

/// Compute the 5-bit FidelityFX frame-generation flags mask from renderer config globals.
///
/// Original: `0x14205be80` (69 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_get_fg_flags() -> u32 {
    let mut flags: u32 = 0;
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7ce8) as usize) != 0 {
        flags |= 1;
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7de0) as usize) != 0 {
        flags |= 2;
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7d00) as usize) != 0 {
        flags |= 4;
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7e40) as usize) != 0 {
        flags |= 0x10;
    }
    flags
}

const IAT_WCSTOMBS: u64 = 0x1_4264_64d0;

/// FidelityFX 2-parameter debug message callback (`type`, `message`).
///
/// Original: `0x14205c450` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_msg_callback_2(_msg_type: u32, wstr: usize) {
    let mut buf = [0u8; 0x88];
    let _ = exe::call_iat!(IAT_WCSTOMBS, (buf.as_mut_ptr() as usize, wstr, 0x80usize) -> usize);
}

/// FidelityFX 3-parameter debug message callback (`type`, `unk`, `message`).
///
/// Original: `0x14205c480` (35 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_msg_callback_3(_a0: u64, _a1: u64, wstr: usize) {
    let mut buf = [0u8; 0x88];
    let _ = exe::call_iat!(IAT_WCSTOMBS, (buf.as_mut_ptr() as usize, wstr, 0x80usize) -> usize);
}

const IAT_FFX_QUERY: u64 = 0x1_4264_6488;
const IAT_FFX_DISPATCH: u64 = 0x1_4264_6490;
const IAT_FFX_CREATE_CONTEXT: u64 = 0x1_4264_6498;
const IAT_FFX_CONFIGURE: u64 = 0x1_4264_64a8;
const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;
const UPSCALE_CTX_VA: u64 = 0x1_43a3_5b28;
const FG_CTX_VA: u64 = 0x1_43a3_5d08;

/// Query upscale jitter phase count (`0x10004`) and jitter offset (`0x10005`) for explicit `index`.
///
/// Original: `0x14205b710` (150 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_get_jitter_offset(
    render_w: u32,
    display_w: u32,
    index: u32,
    out_x: usize,
    out_y: usize,
) {
    let ctx = exe::ptr::<u8>(UPSCALE_CTX_VA) as usize;
    let mut phase_count: u32 = 0;
    let mut q_phase = [0u64; 4];
    q_phase[0] = 0x10004;
    q_phase[1] = 0;
    q_phase[2] = (render_w as u64) | ((display_w as u64) << 32);
    q_phase[3] = &mut phase_count as *mut u32 as usize as u64;
    let _ = exe::call_iat!(IAT_FFX_QUERY, (ctx, q_phase.as_mut_ptr() as usize) -> u32);

    let mut q_off = [0u64; 5];
    q_off[0] = 0x10005;
    q_off[1] = 0;
    q_off[2] = (index as u64) | ((phase_count as u64) << 32);
    q_off[3] = out_x as u64;
    q_off[4] = out_y as u64;
    let _ = exe::call_iat!(IAT_FFX_QUERY, (ctx, q_off.as_mut_ptr() as usize) -> u32);
}

/// Query upscale jitter phase count (`0x10004`) and jitter offset (`0x10005`) using `g_uCurrentFrame` (`0x143a00180`).
///
/// Original: `0x14238d5a0` (157 bytes, `Throws::No`).
pub unsafe extern "win64" fn fsr2_glue_get_jitter_offset(
    render_w: u32,
    display_w: u32,
    init_phase: u32,
    out_x: usize,
    out_y: usize,
) {
    let frame_idx = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
    let ctx = exe::ptr::<u8>(UPSCALE_CTX_VA) as usize;
    let mut phase_count: u32 = init_phase;
    let mut q_phase = [0u64; 4];
    q_phase[0] = 0x10004;
    q_phase[1] = 0;
    q_phase[2] = (render_w as u64) | ((display_w as u64) << 32);
    q_phase[3] = &mut phase_count as *mut u32 as usize as u64;
    let _ = exe::call_iat!(IAT_FFX_QUERY, (ctx, q_phase.as_mut_ptr() as usize) -> u32);

    let mut q_off = [0u64; 5];
    q_off[0] = 0x10005;
    q_off[1] = 0;
    q_off[2] = (frame_idx as u64) | ((phase_count as u64) << 32);
    q_off[3] = out_x as u64;
    q_off[4] = out_y as u64;
    let _ = exe::call_iat!(IAT_FFX_QUERY, (ctx, q_off.as_mut_ptr() as usize) -> u32);
}

/// `ffxFsr2GetUpscaleRatioFromQualityMode` (`0x142514680`).
///
/// Original: `0x142514680` (61 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn ffx_fsr2_get_upscale_ratio_from_quality_mode(mode: u32) -> f32 {
    match mode {
        1 => rd::<f32>(exe::ptr::<f32>(0x1_4342_6978) as usize),
        2 => rd::<f32>(exe::ptr::<f32>(0x1_4342_69d8) as usize),
        3 => rd::<f32>(exe::ptr::<f32>(0x1_4342_6abc) as usize),
        4 => rd::<f32>(exe::ptr::<f32>(0x1_4342_6bf8) as usize),
        _ => 0.0,
    }
}

/// Compute render resolution `(out_w, out_h)` from `(disp_w, disp_h)` and FSR2 quality `mode`.
///
/// Original: `0x14238d650` (277 bytes, `Throws::No`).
pub unsafe extern "win64" fn fsr2_glue_get_render_resolution(
    disp_w: u32,
    disp_h: u32,
    mode: u32,
    out_w: usize,
    out_h: usize,
) -> bool {
    if mode == 0 {
        wr::<u32>(out_w, disp_w);
        wr::<u32>(out_h, disp_h);
        return true;
    }
    let ffx_mode = match mode {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        _ => return false,
    };
    if out_w == 0 || out_h == 0 {
        return false;
    }
    let ratio = ffx_fsr2_get_upscale_ratio_from_quality_mode(ffx_mode);
    let rw = (((disp_w as i32 as i64) as f32) / ratio) as i64 as u32;
    wr::<u32>(out_w, rw);
    let rh = (((disp_h as i32 as i64) as f32) / ratio) as i64 as u32;
    wr::<u32>(out_h, rh);
    true
}

/// Query all upscale provider versions (`0x4`, `createDescType = 0x10000`) into `(out_names_vec, out_ids_vec)`.
///
/// Original: `0x14205c9e0` (166 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_query_upscale_versions(
    out_names_vec: usize,
    out_ids_vec: usize,
) {
    let dev = rd::<u64>(exe::ptr::<u64>(0x1_4397_fc80) as usize);
    let mut count: u64 = 0;
    let mut q = [0u64; 7];
    q[0] = 4;
    q[1] = 0;
    q[2] = 0x10000;
    q[3] = dev;
    q[4] = &mut count as *mut u64 as usize as u64;
    q[5] = 0;
    q[6] = 0;
    let _ = exe::call_iat!(IAT_FFX_QUERY, (0usize, q.as_mut_ptr() as usize) -> u32);
    exe::call_va!(0x1_408b_4ad0, (out_names_vec, count) -> ());
    exe::call_va!(0x1_408b_4ad0, (out_ids_vec, count) -> ());
    q[1] = 0;
    q[5] = rd::<u64>(out_ids_vec);
    q[6] = rd::<u64>(out_names_vec);
    let _ = exe::call_iat!(IAT_FFX_QUERY, (0usize, q.as_mut_ptr() as usize) -> u32);
}

/// Filter upscale provider versions by major version flags (`'2'`, `'3'`, `'4'`) into `(out_names_vec, out_ids_vec)`.
///
/// Original: `0x14205c820` (434 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_fsr2_collect_supported_versions(
    out_names_vec: usize,
    out_ids_vec: usize,
) {
    let mut names_vec = [0u64; 2];
    let mut ids_vec = [0u64; 2];
    d3d_fsr2_query_upscale_versions(
        names_vec.as_mut_ptr() as usize,
        ids_vec.as_mut_ptr() as usize,
    );
    let names_data = names_vec[0] as usize;
    let ids_data = ids_vec[0] as usize;
    let count = (ids_vec[1] as u32) as usize;
    for i in 0..count {
        let name_ptr = rd::<usize>(names_data + i * 8);
        let first_ch = if name_ptr != 0 { rd::<u8>(name_ptr) } else { 0 };
        let keep = (rd::<u8>(exe::ptr::<u8>(0x1_4385_7df8) as usize) != 0 && first_ch == b'2')
            || (rd::<u8>(exe::ptr::<u8>(0x1_4385_7ea0) as usize) != 0 && first_ch == b'3')
            || (rd::<u8>(exe::ptr::<u8>(0x1_4385_7eb8) as usize) != 0 && first_ch == b'4');
        if keep {
            exe::call_va!(0x1_403d_8aa0, (out_ids_vec, ids_data + i * 8) -> ());
            let cur_len = rd::<u32>(out_names_vec + 0x8) as usize;
            let cur_cap = rd::<u32>(out_names_vec + 0xc) as usize;
            if cur_len == cur_cap {
                let min_cap = cur_len + 1;
                let grown = cur_cap + (cur_cap >> 1);
                let new_cap = if grown < min_cap { min_cap } else { grown };
                exe::call_va!(0x1_401d_eba0, (out_names_vec, new_cap) -> ());
            }
            let len_now = rd::<u32>(out_names_vec + 0x8) as usize;
            let dst_base = rd::<usize>(out_names_vec);
            wr::<usize>(dst_base + len_now * 8, rd::<usize>(names_data + i * 8));
            wr::<u32>(out_names_vec + 0x8, (len_now + 1) as u32);
        }
    }
    exe::call_va!(0x1_401d_e830, (ids_vec.as_mut_ptr() as usize) -> ());
    exe::call_va!(0x1_401d_e830, (names_vec.as_mut_ptr() as usize) -> ());
}

unsafe fn update_gpu_timestamp_after_ffx(flag: u32) {
    let tls_ptr = exe::tls_pointer();
    if tls_ptr != 0 {
        let tls0 = rd::<usize>(tls_ptr);
        if tls0 != 0 {
            if rd::<u8>(tls0 + 0x70) == 0 {
                exe::call_va!(0x1_4183_49c0, () -> ());
            }
            exe::call_va!(0x1_4207_2ec0, (tls0 + 0x467e0) -> ());
            let mut out16 = [0u64; 2];
            let p = exe::call_va!(0x1_4207_34e0, (out16.as_mut_ptr() as usize, flag) -> usize);
            let dst = exe::ptr::<u64>(0x1_43c4_8fc0) as usize;
            wr::<u64>(dst, rd::<u64>(p));
            wr::<u64>(dst + 8, rd::<u64>(p + 8));
        }
    }
}

unsafe fn unlock_recursive_srw(lock: usize) {
    if lock != 0 {
        let cnt = rd::<u32>(lock + 0xc).wrapping_sub(1);
        wr::<u32>(lock + 0xc, cnt);
        if cnt == 0 {
            AtomicU32::from_ptr((lock + 0x8) as *mut u32).swap(u32::MAX, Ordering::SeqCst);
            let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
        }
    }
}

/// Create FidelityFX upscale (`0x10000`) and frame-generation (`0x20001`) contexts.
///
/// Original: `0x14205c4b0` (859 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fsr2_create_contexts(
    fmt_idx: i8,
    _unused: u64,
    max_render_size: u64,
    max_upscale_size: u64,
) {
    wr::<u64>(exe::ptr::<u64>(0x1_43a3_2fb8) as usize, max_upscale_size);
    let mut names_vec = [0u64; 2];
    let mut ids_vec = [0u64; 2];
    d3d_fsr2_collect_supported_versions(
        names_vec.as_mut_ptr() as usize,
        ids_vec.as_mut_ptr() as usize,
    );

    let dev = rd::<u64>(exe::ptr::<u64>(0x1_4397_fc80) as usize);
    let mut backend_desc = [0u64; 3];
    backend_desc[0] = 2; // FFX_API_CREATE_CONTEXT_DESC_TYPE_BACKEND_DX12
    backend_desc[1] = 0;
    backend_desc[2] = dev;

    let mut override_ver = [0u64; 3];
    override_ver[0] = 5; // FFX_API_CREATE_CONTEXT_DESC_TYPE_OVERRIDE_VERSION
    override_ver[1] = 0;
    override_ver[2] = 0;

    let ids_ptr = ids_vec[0] as usize;
    let ids_len = ids_vec[1] as u32;
    if ids_len != 0 && ids_ptr != 0 {
        override_ver[2] = rd::<u64>(ids_ptr);
        backend_desc[1] = override_ver.as_mut_ptr() as usize as u64;
    }

    let mut upscale_flags: u32 = if rd::<u8>(exe::ptr::<u8>(0x1_4385_7e28) as usize) != 0 {
        0x38
    } else {
        0x20
    };
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7e88) as usize) != 0 {
        upscale_flags |= 1;
    }
    let dbg_enabled = rd::<u8>(exe::ptr::<u8>(0x1_4385_7db0) as usize) != 0;
    let fp_msg = if dbg_enabled {
        upscale_flags |= 0x80;
        exe::ptr::<u8>(0x1_4205_c450) as usize as u64
    } else {
        0
    };

    #[repr(C)]
    struct CreateUpscaleDesc {
        ty: u64,
        next: u64,
        flags: u32,
        max_render_w: u32,
        max_render_h: u32,
        max_upscale_w: u32,
        max_upscale_h: u32,
        _pad: u32,
        fp_message: u64,
    }
    let mut up_desc = CreateUpscaleDesc {
        ty: 0x10000,
        next: backend_desc.as_mut_ptr() as usize as u64,
        flags: upscale_flags,
        max_render_w: max_render_size as u32,
        max_render_h: (max_render_size >> 32) as u32,
        max_upscale_w: max_upscale_size as u32,
        max_upscale_h: (max_upscale_size >> 32) as u32,
        _pad: 0,
        fp_message: fp_msg,
    };
    let up_ctx = exe::ptr::<u8>(UPSCALE_CTX_VA) as usize;
    let _ = exe::call_iat!(
        IAT_FFX_CREATE_CONTEXT,
        (up_ctx, &mut up_desc as *mut CreateUpscaleDesc as usize, 0usize) -> u32
    );

    let mut up_mem = [0u64; 2];
    let mut q_up_mem = [0x10008u64, 0, up_mem.as_mut_ptr() as usize as u64];
    let _ = exe::call_iat!(IAT_FFX_QUERY, (up_ctx, q_up_mem.as_mut_ptr() as usize) -> u32);

    let mut fg_flags: u32 = 1;
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7e88) as usize) != 0 {
        fg_flags |= 0x20;
    }
    let fmt_table = exe::ptr::<u8>(0x1_4297_eda0) as usize;
    let dxgi_fmt = rd::<u32>((fmt_table as isize + (fmt_idx as isize) * 0x14) as usize);
    let backbuffer_fmt = d3d_fsr2_dxgi_to_ffx_format(dxgi_fmt);

    let mut fg_hdr_desc = [0x20009u64, 0, backbuffer_fmt as u64];
    backend_desc[1] = fg_hdr_desc.as_mut_ptr() as usize as u64;

    #[repr(C)]
    struct CreateFgDesc {
        ty: u64,
        next: u64,
        flags: u32,
        display_w: u32,
        display_h: u32,
        max_render_w: u32,
        max_render_h: u32,
        backbuffer_format: u32,
    }
    let mut fg_desc = CreateFgDesc {
        ty: 0x20001,
        next: backend_desc.as_mut_ptr() as usize as u64,
        flags: fg_flags,
        display_w: max_upscale_size as u32,
        display_h: (max_upscale_size >> 32) as u32,
        max_render_w: max_render_size as u32,
        max_render_h: (max_render_size >> 32) as u32,
        backbuffer_format: backbuffer_fmt,
    };
    let fg_ctx = exe::ptr::<u8>(FG_CTX_VA) as usize;
    let _ = exe::call_iat!(
        IAT_FFX_CREATE_CONTEXT,
        (fg_ctx, &mut fg_desc as *mut CreateFgDesc as usize, 0usize) -> u64
    );

    let mut fg_mem = [0u64; 2];
    let mut q_fg_mem = [0x20007u64, 0, fg_mem.as_mut_ptr() as usize as u64];
    let _ = exe::call_iat!(IAT_FFX_QUERY, (fg_ctx, q_fg_mem.as_mut_ptr() as usize) -> u32);

    let mut out16 = [0u64; 2];
    let p = exe::call_va!(0x1_4207_34e0, (out16.as_mut_ptr() as usize, 0u32) -> usize);
    let dst = exe::ptr::<u64>(0x1_43c4_8fc0) as usize;
    wr::<u64>(dst, rd::<u64>(p));
    wr::<u64>(dst + 8, rd::<u64>(p + 8));

    exe::call_va!(0x1_401d_e830, (ids_vec.as_mut_ptr() as usize) -> ());
    exe::call_va!(0x1_401d_e830, (names_vec.as_mut_ptr() as usize) -> ());
}

/// Configure and dispatch FidelityFX frame generation (`0x20003`) for `swap_chain`.
///
/// Original: `0x14205b7b0` (947 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fsr2_configure_and_dispatch_fg(
    swap_chain: usize,
    frame_id: i32,
    hudless_res: usize,
    ui_tex_res: usize,
    reset: u8,
) {
    let sc_inner = rd::<usize>(swap_chain);
    let (has_fg, dxgi_sc, mut fg_sc_ctx) = if sc_inner != 0
        && rd::<usize>(sc_inner + 0x8) != 0
        && rd::<usize>(sc_inner + 0x10) != 0
    {
        (true, rd::<usize>(sc_inner + 0x8), rd::<usize>(sc_inner + 0x10))
    } else {
        (false, 0, 0)
    };

    let fg_enabled = rd::<u8>(exe::ptr::<u8>(0x1_4385_7dc8) as usize);
    if fg_enabled != 0 && has_fg {
        let cfg_base = exe::ptr::<u8>(0x1_4385_7d10) as usize;
        wr::<u8>(cfg_base + 0x38, fg_enabled);
        wr::<u64>(cfg_base + 0x18, 0);
        wr::<u64>(cfg_base + 0x20, 0);
        wr::<usize>(cfg_base + 0x10, dxgi_sc);
        wr::<u32>(cfg_base + 0x70, d3d_fsr2_get_fg_flags());

        let mut tmp_res = [0u64; 6];
        d3d_fsr2_get_ffx_resource(
            tmp_res.as_mut_ptr() as usize,
            rd::<usize>(hudless_res + 0x70),
            0xc,
        );
        for i in 0..6 {
            wr::<u64>(cfg_base + 0x40 + i * 8, tmp_res[i]);
        }

        wr::<u8>(
            cfg_base + 0x39,
            rd::<u8>(exe::ptr::<u8>(0x1_4385_7e58) as usize),
        );
        wr::<u32>(
            cfg_base + 0x80,
            rd::<u32>(exe::ptr::<u32>(0x1_43a3_2fb8) as usize),
        );
        wr::<u32>(
            cfg_base + 0x84,
            rd::<u32>(exe::ptr::<u32>(0x1_43a3_2fbc) as usize),
        );
        wr::<u8>(
            cfg_base + 0x74,
            rd::<u8>(exe::ptr::<u8>(0x1_4385_7e10) as usize),
        );
        wr::<u64>(cfg_base + 0x78, 0);
        wr::<u64>(cfg_base + 0x28, 0);
        wr::<u64>(cfg_base + 0x30, 0);
        wr::<i64>(cfg_base + 0x88, frame_id as i64);

        let mut ui_desc = [0u64; 8];
        if ui_tex_res != 0 {
            ui_desc[0] = 0x20008;
            ui_desc[1] = 0;
            d3d_fsr2_get_ffx_resource(
                (ui_desc.as_mut_ptr() as usize) + 0x10,
                rd::<usize>(ui_tex_res + 0x70),
                0xc,
            );
            wr::<usize>(cfg_base + 0x08, ui_desc.as_mut_ptr() as usize);
        } else {
            wr::<usize>(cfg_base + 0x08, 0);
        }

        let fg_ctx = exe::ptr::<u8>(FG_CTX_VA) as usize;
        let _ = exe::call_iat!(IAT_FFX_CONFIGURE, (fg_ctx, cfg_base) -> u32);

        let mut disp = [0u8; 0x140];
        let disp_ptr = disp.as_mut_ptr() as usize;
        wr::<u64>(disp_ptr, 0x20003);
        wr::<u64>(disp_ptr + 0x08, 0);

        let bb_idx = rd::<i32>(swap_chain + 0x38) as isize as usize;
        let bb_obj = rd::<usize>(swap_chain + 0x08 + bb_idx * 8);
        d3d_fsr2_get_ffx_resource(
            tmp_res.as_mut_ptr() as usize,
            rd::<usize>(bb_obj + 0x70),
            0xc,
        );
        for i in 0..6 {
            wr::<u64>(disp_ptr + 0x18 + i * 8, tmp_res[i]);
        }

        wr::<u32>(disp_ptr + 0x108, 1);
        wr::<u32>(disp_ptr + 0x10c, reset as u32);

        let xfer_fn = if rd::<u8>(swap_chain + 0x4c) == 0 {
            0u32
        } else if rd::<u8>(swap_chain + 0x66) == 0 {
            let mode_override = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
            let is_scrgb = if mode_override == 0 {
                rd::<u8>(swap_chain + 0x67) == 0
            } else {
                mode_override == 2
            };
            if is_scrgb { 2 } else { 1 }
        } else {
            2
        };
        wr::<u32>(disp_ptr + 0x110, xfer_fn);
        wr::<f32>(disp_ptr + 0x114, 0.0);
        wr::<f32>(disp_ptr + 0x118, rd::<f32>(swap_chain + 0x50));
        wr::<u64>(disp_ptr + 0x11c, 0);
        wr::<u32>(
            disp_ptr + 0x124,
            rd::<u32>(exe::ptr::<u32>(0x1_43a3_2fb8) as usize),
        );
        wr::<u32>(
            disp_ptr + 0x128,
            rd::<u32>(exe::ptr::<u32>(0x1_43a3_2fbc) as usize),
        );
        wr::<u64>(disp_ptr + 0x12c, 0);
        wr::<i64>(disp_ptr + 0x130, frame_id as i64);

        let sc_ctx_ptr = &mut fg_sc_ctx as *mut usize as usize;
        let mut q_cmd = [0x30003u64, 0, (disp_ptr + 0x10) as u64];
        let _ = exe::call_iat!(IAT_FFX_QUERY, (sc_ctx_ptr, q_cmd.as_mut_ptr() as usize) -> u32);

        let mut q_tex = [0x30004u64, 0, (disp_ptr + 0x48) as u64];
        let _ = exe::call_iat!(IAT_FFX_QUERY, (sc_ctx_ptr, q_tex.as_mut_ptr() as usize) -> u32);

        let _ = exe::call_iat!(IAT_FFX_DISPATCH, (fg_ctx, disp_ptr) -> u32);
    }

    update_gpu_timestamp_after_ffx(1);
}

/// Forwarding wrapper `rend::FSR2Glue::dispatchFrameGen` (`0x14238d400`) -> `0x14205b7b0`.
///
/// Original: `0x14238d400` (14 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn fsr2_glue_dispatch_fg(
    swap_chain: usize,
    frame_id: i32,
    hudless_res: usize,
    ui_tex_res: usize,
    reset: u8,
) {
    d3d_fsr2_configure_and_dispatch_fg(swap_chain, frame_id, hudless_res, ui_tex_res, reset);
}

/// Dispatch FidelityFX frame-generation prepare (`0x20004` + camera info `0x2000a`).
///
/// Original: `0x14205bb70` (776 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fsr2_dispatch_fg_prepare(
    cam_matrix: usize,
    swap_chain: usize,
    frame_id: i32,
    motion_vecs: usize,
    depth: usize,
    jitter: u64,
    render_size: u64,
    frame_time_delta_bits: u64,
    camera_near_bits: u64,
    camera_far_bits: u64,
    camera_fov_y_bits: u64,
) {
    let sc_inner = rd::<usize>(swap_chain);
    let has_fg = sc_inner != 0
        && rd::<usize>(sc_inner + 0x8) != 0
        && rd::<usize>(sc_inner + 0x10) != 0;
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7dc8) as usize) == 0 || !has_fg {
        return;
    }

    exe::call_va!(0x1_4206_94a0, (motion_vecs) -> ());
    exe::call_va!(0x1_4206_94a0, (depth) -> ());
    let mut cmd_lock = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> ());

    let mut cam_info = [0u32; 16];
    let cam_ptr = cam_info.as_mut_ptr() as usize;
    wr::<u64>(cam_ptr, 0x2000a);
    wr::<u64>(cam_ptr + 0x08, 0);
    // cameraPosition = column 3 (`+0x24..+0x30`), cameraUp = column 1 (`+0x0c..+0x18`),
    // cameraRight = column 0 (`+0x00..+0x0c`), cameraForward = column 2 (`+0x18..+0x24`).
    wr::<u32>(cam_ptr + 0x10, rd::<u32>(cam_matrix + 0x24));
    wr::<u32>(cam_ptr + 0x14, rd::<u32>(cam_matrix + 0x28));
    wr::<u32>(cam_ptr + 0x18, rd::<u32>(cam_matrix + 0x2c));
    wr::<u32>(cam_ptr + 0x1c, rd::<u32>(cam_matrix + 0x0c));
    wr::<u32>(cam_ptr + 0x20, rd::<u32>(cam_matrix + 0x10));
    wr::<u32>(cam_ptr + 0x24, rd::<u32>(cam_matrix + 0x14));
    wr::<u32>(cam_ptr + 0x28, rd::<u32>(cam_matrix));
    wr::<u32>(cam_ptr + 0x2c, rd::<u32>(cam_matrix + 0x04));
    wr::<u32>(cam_ptr + 0x30, rd::<u32>(cam_matrix + 0x08));
    wr::<u32>(cam_ptr + 0x34, rd::<u32>(cam_matrix + 0x18));
    wr::<u32>(cam_ptr + 0x38, rd::<u32>(cam_matrix + 0x1c));
    wr::<u32>(cam_ptr + 0x3c, rd::<u32>(cam_matrix + 0x20));

    let mut prep = [0u8; 0xb8];
    let prep_ptr = prep.as_mut_ptr() as usize;
    wr::<u64>(prep_ptr, 0x20004);
    wr::<usize>(prep_ptr + 0x08, cam_ptr);
    wr::<i64>(prep_ptr + 0x10, frame_id as i64);
    wr::<u32>(prep_ptr + 0x18, d3d_fsr2_get_fg_flags());
    wr::<u32>(prep_ptr + 0x1c, 0);
    wr::<usize>(prep_ptr + 0x20, cmd_lock[1]);

    let rw = render_size as u32;
    let rh = (render_size >> 32) as u32;
    wr::<u32>(prep_ptr + 0x28, rw);
    wr::<u32>(prep_ptr + 0x2c, rh);
    wr::<u64>(prep_ptr + 0x30, jitter);
    wr::<f32>(prep_ptr + 0x38, (rw as i32) as f32);
    wr::<f32>(prep_ptr + 0x3c, (rh as i32) as f32);

    let dt = f32::from_bits(frame_time_delta_bits as u32)
        * rd::<f32>(exe::ptr::<f32>(0x1_4342_6f34) as usize);
    wr::<f32>(prep_ptr + 0x40, dt);
    wr::<u32>(prep_ptr + 0x44, 0);
    wr::<u32>(prep_ptr + 0x48, camera_near_bits as u32);
    wr::<u32>(prep_ptr + 0x4c, camera_far_bits as u32);
    wr::<u32>(prep_ptr + 0x50, camera_fov_y_bits as u32);
    wr::<u32>(
        prep_ptr + 0x54,
        rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize),
    );

    let mut tmp_res = [0u64; 6];
    d3d_fsr2_get_ffx_resource(
        tmp_res.as_mut_ptr() as usize,
        rd::<usize>(depth + 0x70),
        0xc,
    );
    for i in 0..6 {
        wr::<u64>(prep_ptr + 0x58 + i * 8, tmp_res[i]);
    }
    d3d_fsr2_get_ffx_resource(
        tmp_res.as_mut_ptr() as usize,
        rd::<usize>(motion_vecs + 0x70),
        0xc,
    );
    for i in 0..6 {
        wr::<u64>(prep_ptr + 0x88 + i * 8, tmp_res[i]);
    }

    let fg_ctx = exe::ptr::<u8>(FG_CTX_VA) as usize;
    let _ = exe::call_iat!(IAT_FFX_DISPATCH, (fg_ctx, prep_ptr) -> u32);

    update_gpu_timestamp_after_ffx(1);
    unlock_recursive_srw(cmd_lock[0]);
}

/// Build `ffxDispatchDescUpscale` (`0x10001`) and call `ffxDispatch` against `0x143a35b28`.
///
/// Original: `0x14205bed0` (1007 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fsr2_dispatch_upscale(
    color_out: usize,
    color_in: usize,
    motion_vecs: usize,
    depth: usize,
    reactive: usize,
    trans_and_comp: usize,
    exposure: usize,
    jitter: u64,
    frame_time_delta_bits: u64,
    camera_near_bits: u64,
    camera_far_bits: u64,
    camera_fov_y_bits: u64,
    reset: u64,
    _unused14: u64,
    sharpness_cfg: usize,
) {
    exe::call_va!(0x1_4206_93a0, (color_out) -> ());
    exe::call_va!(0x1_4206_94a0, (color_in) -> ());
    exe::call_va!(0x1_4206_94a0, (motion_vecs) -> ());
    exe::call_va!(0x1_4206_94a0, (depth) -> ());

    let mut cmd_lock = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> ());

    let mut disp = [0u8; 0x1b0];
    let disp_ptr = disp.as_mut_ptr() as usize;
    wr::<u64>(disp_ptr, 0x10001);
    wr::<u64>(disp_ptr + 0x08, 0);
    wr::<usize>(disp_ptr + 0x10, cmd_lock[1]);

    let mut tmp_res = [0u64; 6];
    d3d_fsr2_get_ffx_resource(
        tmp_res.as_mut_ptr() as usize,
        rd::<usize>(color_in + 0x70),
        0xc,
    );
    for i in 0..6 {
        wr::<u64>(disp_ptr + 0x18 + i * 8, tmp_res[i]);
    }

    d3d_fsr2_get_ffx_resource(
        tmp_res.as_mut_ptr() as usize,
        rd::<usize>(depth + 0x70),
        0xc,
    );
    for i in 0..6 {
        wr::<u64>(disp_ptr + 0x48 + i * 8, tmp_res[i]);
    }

    d3d_fsr2_get_ffx_resource(
        tmp_res.as_mut_ptr() as usize,
        rd::<usize>(motion_vecs + 0x70),
        0xc,
    );
    for i in 0..6 {
        wr::<u64>(disp_ptr + 0x78 + i * 8, tmp_res[i]);
    }

    d3d_fsr2_get_ffx_resource(
        tmp_res.as_mut_ptr() as usize,
        rd::<usize>(exposure + 0x70),
        0xc,
    );
    for i in 0..6 {
        wr::<u64>(disp_ptr + 0xa8 + i * 8, tmp_res[i]);
    }

    if reactive != 0 {
        d3d_fsr2_get_ffx_resource(
            tmp_res.as_mut_ptr() as usize,
            rd::<usize>(reactive + 0x70),
            0xc,
        );
        for i in 0..6 {
            wr::<u64>(disp_ptr + 0xd8 + i * 8, tmp_res[i]);
        }
    }

    if trans_and_comp != 0 {
        d3d_fsr2_get_ffx_resource(
            tmp_res.as_mut_ptr() as usize,
            rd::<usize>(trans_and_comp + 0x70),
            0xc,
        );
        for i in 0..6 {
            wr::<u64>(disp_ptr + 0x108 + i * 8, tmp_res[i]);
        }
    }

    d3d_fsr2_get_ffx_resource(
        tmp_res.as_mut_ptr() as usize,
        rd::<usize>(color_out + 0x70),
        0x2,
    );
    for i in 0..6 {
        wr::<u64>(disp_ptr + 0x138 + i * 8, tmp_res[i]);
    }

    wr::<u64>(disp_ptr + 0x168, jitter);
    let rw = rd::<u32>(color_in);
    let rh = rd::<u32>(color_in + 0x04);
    wr::<f32>(disp_ptr + 0x170, (rw as u64) as f32);
    wr::<f32>(disp_ptr + 0x174, (rh as u64) as f32);
    wr::<u32>(disp_ptr + 0x178, rw);
    wr::<u32>(disp_ptr + 0x17c, rh);
    wr::<u32>(
        disp_ptr + 0x180,
        rd::<u32>(exe::ptr::<u32>(0x1_43a3_2fb8) as usize),
    );
    wr::<u32>(
        disp_ptr + 0x184,
        rd::<u32>(exe::ptr::<u32>(0x1_43a3_2fbc) as usize),
    );
    // Diagnostics: the actual render -> output size, logged once per change (host-only state).
    static LAST_FSR_SIZES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let (uw, uh) = (rd::<u32>(disp_ptr + 0x180), rd::<u32>(disp_ptr + 0x184));
    let key = (rw as u64 & 0xffff) | (rh as u64 & 0xffff) << 16 | (uw as u64 & 0xffff) << 32 | (uh as u64 & 0xffff) << 48;
    if LAST_FSR_SIZES.swap(key, Ordering::Relaxed) != key {
        log::info!(target: "fsr", "FSR upscale dispatch: render {rw}x{rh} -> output {uw}x{uh}");
    }

    let sharpness = if sharpness_cfg != 0 {
        rd::<f32>(sharpness_cfg + 0x10)
    } else {
        0.0
    };
    wr::<u8>(disp_ptr + 0x188, u8::from(sharpness > 0.0));
    wr::<f32>(disp_ptr + 0x18c, sharpness);

    let dt = f32::from_bits(frame_time_delta_bits as u32)
        * rd::<f32>(exe::ptr::<f32>(0x1_4342_6f34) as usize);
    wr::<f32>(disp_ptr + 0x190, dt);
    wr::<u32>(
        disp_ptr + 0x194,
        rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize),
    );
    wr::<u8>(disp_ptr + 0x198, reset as u8);
    wr::<u32>(disp_ptr + 0x19c, camera_near_bits as u32);
    wr::<u32>(disp_ptr + 0x1a0, camera_far_bits as u32);
    wr::<u32>(disp_ptr + 0x1a4, camera_fov_y_bits as u32);
    let flags = u32::from(rd::<u8>(exe::ptr::<u8>(0x1_4385_7e70) as usize) != 0);
    wr::<u32>(disp_ptr + 0x1ac, flags);

    let up_ctx = exe::ptr::<u8>(UPSCALE_CTX_VA) as usize;
    let _ = exe::call_iat!(IAT_FFX_DISPATCH, (up_ctx, disp_ptr) -> u32);

    update_gpu_timestamp_after_ffx(1);
    unlock_recursive_srw(cmd_lock[0]);
}

/// Wrapper around `d3d::fsr2::dispatchUpscale` (`0x14205bed0`) reading the 5 sharpness/tuning floats from globals.
///
/// Original: `0x14238ce70` (221 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn fsr2_glue_dispatch_upscale_wrapper(
    color_out: usize,
    color_in: usize,
    motion_vecs: usize,
    depth: usize,
    reactive: usize,
    trans_and_comp: usize,
    exposure: usize,
    jitter: u64,
    frame_time_delta_bits: u64,
    camera_near_bits: u64,
    camera_far_bits: u64,
    camera_fov_y_bits: u64,
    reset: u64,
) {
    let mut cfg = [0u32; 5];
    cfg[0] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cbc0) as usize);
    cfg[1] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cbe0) as usize);
    cfg[2] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cc00) as usize);
    cfg[3] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cba0) as usize);
    cfg[4] = rd::<u32>(exe::ptr::<u32>(0x1_4386_9af0) as usize);
    d3d_fsr2_dispatch_upscale(
        color_out,
        color_in,
        motion_vecs,
        depth,
        reactive,
        trans_and_comp,
        exposure,
        jitter,
        frame_time_delta_bits,
        camera_near_bits,
        camera_far_bits,
        camera_fov_y_bits,
        reset,
        0,
        cfg.as_mut_ptr() as usize,
    );
}

/// Construct `rend::FSR2Glue::execWrapper::<lambda_1>` capture struct (`0x14238cf60`).
///
/// Original: `0x14238cf60` (35 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn fsr2_glue_lambda_ctor(
    this: usize,
    reset_ptr: usize,
    no_masks_ptr: usize,
    jitter_ptr: usize,
    auto_exposure_ptr: usize,
) -> usize {
    wr::<u8>(this, rd::<u8>(reset_ptr));
    wr::<u8>(this + 0x01, rd::<u8>(no_masks_ptr));
    wr::<u64>(this + 0x04, rd::<u64>(jitter_ptr));
    wr::<u64>(this + 0x10, rd::<u64>(auto_exposure_ptr));
    this
}

#[inline(always)]
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= (b as i8 as i64) as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

#[inline(always)]
unsafe fn find_pass_resource_slot(pass_data: usize, name: &[u8]) -> usize {
    let target_hash = fnv1a64(name);
    let hashes = pass_data + 0x208;
    for i in 0..0x40usize {
        let h = rd::<u64>(hashes + i * 8);
        if h != 0 && h == target_hash {
            return rd::<usize>(pass_data + 0x08 + i * 8);
        }
    }
    0
}

/// Execute `rend::FSR2Glue::execWrapper::<lambda_1>::operator()(OrderSolverPassData*)` (`0x14238cf90`).
///
/// Original: `0x14238cf90` (907 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn fsr2_glue_lambda_call(this: usize, pass_data: usize) {
    let mut comp_slot = 0usize;
    let mut reactive_slot = 0usize;
    if rd::<u8>(this + 0x01) == 0 {
        comp_slot = find_pass_resource_slot(pass_data, b"fsr2_compmask");
        if rd::<u8>(exe::ptr::<u8>(0x1_4386_cb70) as usize) != 0 {
            reactive_slot = find_pass_resource_slot(pass_data, b"FSR2_InputReactiveMap");
        }
    }
    let reset = rd::<u8>(this) as u64;
    let min_dt = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
    let cur_dt = rd::<f32>(exe::ptr::<f32>(0x1_43a0_017c) as usize);
    let dt = if min_dt > cur_dt { min_dt } else { cur_dt };

    let jitter = rd::<u64>(this + 0x04);
    let mut frame_mod = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize)
        .wrapping_add(1)
        & 0x8000_0001;
    if (frame_mod as i32) < 0 {
        frame_mod = (frame_mod.wrapping_sub(1) | 0xffff_fffe).wrapping_add(1);
    }
    let ae_hist = rd::<usize>(this + 0x10);
    let exposure = rd::<usize>((ae_hist as isize + 0x20 + (frame_mod as i32 as isize) * 8) as usize);
    let trans_and_comp = if comp_slot != 0 { rd::<usize>(comp_slot) } else { 0 };
    let reactive = if reactive_slot != 0 {
        rd::<usize>(reactive_slot)
    } else {
        0
    };

    let depth_slot = exe::call_va!(0x1_412f_f1b0, (pass_data) -> usize);
    let depth = rd::<usize>(depth_slot);

    let vel_slot = find_pass_resource_slot(pass_data, b"velocity_geometry_buffer");
    let motion_vecs = rd::<usize>(vel_slot);

    let in_slot = find_pass_resource_slot(pass_data, b"colorbufferIN");
    let color_in = rd::<usize>(in_slot);

    let out_slot = find_pass_resource_slot(pass_data, b"colorbufferOUT");
    let color_out = rd::<usize>(out_slot);

    let mut cfg = [0u32; 5];
    cfg[0] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cbc0) as usize);
    cfg[1] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cbe0) as usize);
    cfg[2] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cc00) as usize);
    cfg[3] = rd::<u32>(exe::ptr::<u32>(0x1_4386_cba0) as usize);
    cfg[4] = rd::<u32>(exe::ptr::<u32>(0x1_4386_9af0) as usize);

    let cam_near = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0124) as usize) as u64;
    let cam_far = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0148) as usize) as u64;
    let cam_fov = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0128) as usize) as u64;

    d3d_fsr2_dispatch_upscale(
        color_out,
        color_in,
        motion_vecs,
        depth,
        reactive,
        trans_and_comp,
        exposure,
        jitter,
        dt.to_bits() as u64,
        cam_near,
        cam_far,
        cam_fov,
        reset,
        0,
        cfg.as_mut_ptr() as usize,
    );
}

/// Enqueue `rend::FSR2Glue::execWrapper`'s pass callback onto the current `OrderSolver` pass.
///
/// Original: `0x14238d330` (161 bytes, `Throws::No`).
pub unsafe extern "win64" fn fsr2_glue_exec_wrapper(
    _order_solver: usize,
    auto_exposure: u64,
    jitter: u64,
    reset: u8,
) {
    let mut func_buf = [0u64; 8];
    let f_ptr = func_buf.as_mut_ptr() as usize;
    let vtbl = exe::ptr::<u8>(0x1_4340_2888) as usize;
    wr::<usize>(f_ptr, vtbl);
    wr::<u8>(f_ptr + 0x08, reset);
    wr::<u8>(
        f_ptr + 0x09,
        rd::<u8>(exe::ptr::<u8>(0x1_4386_a2a0) as usize),
    );
    wr::<u64>(f_ptr + 0x0c, jitter);
    wr::<u64>(f_ptr + 0x18, auto_exposure);
    wr::<usize>(f_ptr + 0x38, f_ptr);

    let pass_idx = rd::<i32>(exe::ptr::<i32>(0x1_43bb_d168) as usize) as isize;
    let passes_base = rd::<usize>(exe::ptr::<usize>(0x1_43bb_d180) as usize);
    let pass_target = (passes_base as isize + pass_idx * 0x2e0 + 0x10) as usize;
    exe::call_va!(0x1_4128_cba0, (pass_target, f_ptr) -> ());

    let target_obj = rd::<usize>(f_ptr + 0x38);
    if target_obj != 0 {
        let obj_vtbl = rd::<usize>(target_obj);
        let destroy = rd::<usize>(obj_vtbl + 0x20);
        let is_heap = u8::from(target_obj != f_ptr);
        exe::call_va!(destroy, (target_obj, is_heap) -> ());
    }
}

/// Fatal `DXGI_ERROR_DEVICE_REMOVED` (`0x887a0005`) handler forwarding to `0x14205d410`.
///
/// Original: `0x14205cdd0` (10 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fatal_device_removed(rcx: usize) {
    exe::call_va!(0x1_4205_d410, (rcx, 0x887a_0005u32) -> ());
}

/// Show a GPU error dialog (`0x1423f2b40(msg, title, 0)`) when UI popups are enabled (`*0x143844045 != 0`).
///
/// Original: `0x14205d3d0` (18 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_show_error_message_box(msg: usize, title: usize) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4384_4045) as usize) != 0 {
        exe::call_va!(0x1_423f_2b40, (msg, title, 0u32) -> ());
    }
}

// ============================================================================
// 2. `rend::FrameRateHistory` (`0x14238d770`, `0x14238d7f0`, `0x14238d830`)
// ============================================================================

/// Compute the average FPS (`1.0 / mean(samples[0..count])`) over the ring buffer in `this`.
///
/// Original: `0x14238d770` (119 bytes, leaf, `Throws::No`, returns `f32` in `xmm0`).
pub unsafe extern "win64" fn frame_rate_history_get_avg_fps(this: usize) -> f32 {
    let count = rd::<u64>(this + 0x08);
    if count == 0 {
        return 0.0;
    }
    let buf = rd::<usize>(this + 0x10);
    let mut sum: f64 = 0.0;
    let mut i: u32 = 0;
    while ((i as i32 as i64) as u64) < count {
        let idx = (i as i32 as isize) as usize;
        sum += rd::<f64>(buf.wrapping_add(idx * 8));
        i = i.wrapping_add(1);
    }
    let denom: f64 = if (count as i64) < 0 {
        let half = ((count >> 1) | (count & 1)) as i64 as f64;
        half + half
    } else {
        (count as i64) as f64
    };
    let avg = sum / denom;
    if avg == 0.0 {
        0.0
    } else {
        let one = rd::<f64>(exe::ptr::<f64>(0x1_4342_6a48) as usize);
        (one / avg) as f32
    }
}

/// Push a frame-time sample `sample` (`xmm1`) into the 16-slot ring buffer at `this` (`rcx`).
///
/// Original: `0x14238d7f0` (51 bytes, leaf, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn frame_rate_history_push(this: usize, sample: f64) {
    frame_rate_history_push_impl(this, sample);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn frame_rate_history_push(this: usize, _xmm0: f64, sample: f64) {
    frame_rate_history_push_impl(this, sample);
}

#[inline(always)]
unsafe fn frame_rate_history_push_impl(this: usize, sample: f64) {
    let write_idx = rd::<u64>(this);
    wr::<u64>(this, write_idx.wrapping_add(1));
    let buf = rd::<usize>(this + 0x10);
    wr::<f64>(buf.wrapping_add((write_idx as usize) * 8), sample);
    let cap = rd::<u32>(this + 0x18) as u64;
    if rd::<u64>(this) >= cap {
        wr::<u64>(this, 0);
    }
    let len = rd::<u64>(this + 0x08);
    if len < cap {
        wr::<u64>(this + 0x08, len.wrapping_add(1));
    }
}

/// Initialize `FrameRateHistory` at `this` (`rcx`), allocating 16 `f64` slots via `0x1401dec00`.
///
/// Original: `0x14238d830` (141 bytes, `Throws::No`).
pub unsafe extern "win64" fn frame_rate_history_init(this: usize) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x08, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    let buf = exe::call_va!(0x1_401d_ec00, (this, 0x10usize) -> usize);
    wr::<usize>(this + 0x10, buf);
    wr::<u32>(this + 0x1c, 0x10);
    for i in 0..16usize {
        wr::<u64>(buf + i * 8, 0);
    }
    wr::<u32>(this + 0x18, 0x10);
    wr::<u64>(this + 0x08, 0);
    wr::<u64>(this, 0);
    this
}

// ============================================================================
// 3. `d3d` dynamic buffer views, input layouts, and shader wrappers (`0x14205d7d0..0x142060a20`)
// ============================================================================

const IAT_GET_CURRENT_THREAD_ID: u64 = 0x1_4264_5790;
const IAT_ACQUIRE_SRW_EXCLUSIVE: u64 = 0x1_4264_55a8;

#[inline(always)]
unsafe fn lock_recursive_srw(lock: usize) {
    let tid = exe::call_iat!(IAT_GET_CURRENT_THREAD_ID, () -> u32);
    if rd::<u32>(lock + 0x08) != tid {
        let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock) -> ());
        wr::<u32>(lock + 0x08, tid);
    }
    wr::<u32>(lock + 0x0c, rd::<u32>(lock + 0x0c).wrapping_add(1));
}

#[inline(always)]
unsafe fn init_buffer_view_from_alloc(
    this: usize,
    alloc: usize,
    stride_08: u64,
    page_sz_10: u64,
) -> usize {
    for off in (0..0x50usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    let cpu_ptr = rd::<u64>(alloc + 0x18);
    if cpu_ptr != 0 {
        wr::<u64>(this, rd::<u64>(alloc + 0x08));
        wr::<u64>(this + 0x40, rd::<u64>(alloc));
    }
    wr::<u64>(this + 0x08, stride_08);
    wr::<u64>(this + 0x10, page_sz_10);
    wr::<u32>(this + 0x48, 0xac3);
    wr::<u64>(this + 0x50, cpu_ptr);
    wr::<u32>(this + 0x58, rd::<u32>(alloc + 0x20));
    wr::<u32>(this + 0x5c, rd::<u32>(alloc + 0x24));
    this
}

/// Construct a 96-byte constant-buffer view (`page_size = 0x80000`) from a 40-byte ring allocation.
///
/// Original: `0x14205d970` (181 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_cbv_ctor_from_alloc(this: usize, alloc: usize) -> usize {
    init_buffer_view_from_alloc(this, alloc, 0, 0x80000)
}

/// Return the constant-buffer ring page size (`0x80000`).
///
/// Original: `0x14205dc30` (6 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_cbv_page_size() -> u32 {
    0x80000
}

/// Construct a 96-byte vertex-buffer view (`page_size = 0x400000`) from a 40-byte ring allocation.
///
/// Original: `0x14205e1c0` (181 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vbv_ctor_from_alloc(this: usize, alloc: usize) -> usize {
    init_buffer_view_from_alloc(this, alloc, 0, 0x40_0000)
}

/// Construct a 96-byte index-buffer view (`stride = 4`, `page_size = 0x80000`) from a 40-byte ring allocation.
///
/// Original: `0x14205e5f0` (189 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_ibv_ctor_from_alloc(this: usize, alloc: usize) -> usize {
    init_buffer_view_from_alloc(this, alloc, 4, 0x80000)
}

/// Construct a 96-byte structured-buffer view (`page_size = 0x100000`) from a 48-byte ring allocation.
///
/// Original: `0x14205f420` (185 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_sbv_ctor_from_alloc(this: usize, alloc: usize) -> usize {
    for off in (0..0x50usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    let cpu_ptr = rd::<u64>(alloc + 0x20);
    if cpu_ptr != 0 {
        wr::<u64>(this, rd::<u64>(alloc + 0x08));
        wr::<u64>(this + 0x10, 0x10_0000);
        wr::<u64>(this + 0x28, rd::<u64>(alloc + 0x18));
        wr::<u64>(this + 0x40, rd::<u64>(alloc));
        wr::<u32>(this + 0x48, 0xac3);
    }
    wr::<u64>(this + 0x50, cpu_ptr);
    wr::<u32>(this + 0x58, rd::<u32>(alloc + 0x28));
    wr::<u32>(this + 0x5c, rd::<u32>(alloc + 0x2c));
    this
}

/// Allocate and upload `size` (`rdx`) bytes from `src` (`r8`) into the constant-buffer ring (`0x143a35d58`),
/// writing the 96-byte view to `out_view` (`rcx`).
///
/// Original: `0x14205d7d0` (402 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_cbv_alloc_and_copy(
    out_view: usize,
    size: usize,
    src: usize,
) -> usize {
    let lock = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5d58) as usize);
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_8150, (0usize, (-2isize) as usize) -> usize);
    let mut alloc = [0u64; 5];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_7ec0, (ring, alloc_ptr, size) -> ());
    let cpu_ptr = rd::<usize>(alloc_ptr + 0x18);
    if cpu_ptr != 0 {
        let _ = exe::call_va!(0x1_4183_acbc, (cpu_ptr, src, size) -> usize);
        wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_sub(1));
    }
    d3d_cbv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Release a mapped constant-buffer ring allocation (`--ring->active_maps`).
///
/// Original: `0x14205da30` (114 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_cbv_unmap(rcx: usize, rdx: usize) {
    let lock = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5d58) as usize);
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_8150, (rcx, (-2isize) as usize) -> usize);
    let _ = rdx;
    wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_sub(1));
    unlock_recursive_srw(lock);
}

/// Allocate `size` (`rdx`) bytes with alignment `align` (`r8`) from the constant-buffer ring (`0x143a35d58`).
///
/// Original: `0x14205dab0` (367 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_cbv_alloc(
    out_view: usize,
    size: usize,
    align: usize,
) -> usize {
    let lock = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5d58) as usize);
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_8150, (out_view, align) -> usize);
    let mut alloc = [0u64; 5];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_7ec0, (ring, alloc_ptr, size) -> ());
    d3d_cbv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Release a mapped vertex-buffer ring allocation (`--ring->active_maps`).
///
/// Original: `0x14205de40` (107 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_vbv_unmap(rcx: usize, _rdx: usize) {
    let lock = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5e78) as usize);
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_8a00, (rcx) -> usize);
    wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_sub(1));
    unlock_recursive_srw(lock);
}

/// Allocate up to `max_size` (`rdx`, min `min_size` `r8`) bytes from the vertex-buffer ring (`0x143a35e78`).
///
/// Original: `0x14205dec0` (390 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_vbv_alloc_clamped(
    out_view: usize,
    max_size: usize,
    min_size: usize,
) -> usize {
    let lock = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5e78) as usize);
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_8a00, (out_view) -> usize);
    let rem = 0x40_0000usize.wrapping_sub(rd::<usize>(ring + 0x20));
    let req = if rem >= min_size && rem < max_size {
        rem
    } else {
        max_size
    };
    let mut alloc = [0u64; 5];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_8770, (ring, alloc_ptr, req) -> ());
    d3d_vbv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Allocate `size` (`rdx`) bytes from the vertex-buffer ring (`0x143a35e78`).
///
/// Original: `0x14205e050` (355 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_vbv_alloc(out_view: usize, size: usize) -> usize {
    let lock = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5e78) as usize);
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_8a00, (out_view) -> usize);
    let mut alloc = [0u64; 5];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_8770, (ring, alloc_ptr, size) -> ());
    d3d_vbv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Destroy the index-buffer ring pool (`0x143a35f98` and `0x14397fc20`).
///
/// Original: `0x14205e2a0` (354 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_ibv_pool_shutdown() {
    let list_va = exe::ptr::<usize>(0x1_43a3_5f98) as usize;
    let list = rd::<usize>(list_va);
    if list != 0 {
        let base = rd::<usize>(list);
        let count = rd::<u32>(list + 0x08) as usize;
        for i in 0..count {
            let elem = rd::<usize>(base + i * 8);
            if elem != 0 {
                let vtbl = rd::<usize>(elem);
                let dtor = rd::<usize>(vtbl);
                exe::call_va!(dtor, (elem, 0u32) -> ());
                exe::call_va!(0x1_4244_2de0, (elem) -> ());
            }
        }
        wr::<u32>(list + 0x08, 0);
        exe::call_va!(0x1_401d_e830, (list) -> ());
        exe::call_va!(0x1_4244_2de0, (list) -> ());
    }
    wr::<usize>(list_va, 0);

    let q_va = exe::ptr::<usize>(0x1_4397_fc20) as usize;
    let q = rd::<usize>(q_va);
    if q != 0 {
        lock_recursive_srw(q);
        while rd::<u32>(q + 0x20) != 0 {
            let arr = rd::<usize>(q + 0x10);
            let head = rd::<u32>(q + 0x18) as u64;
            let cap = rd::<u32>(q + 0x24) as u64;
            let slot = (head % cap) as usize;
            let item = rd::<usize>(arr + slot * 8);
            let res = rd::<usize>(item);
            let vtbl = rd::<usize>(res);
            let unmap = rd::<usize>(vtbl + 0x48);
            exe::call_va!(unmap, (res, 0u32, 0usize) -> ());
            let res2 = rd::<usize>(item);
            if res2 != 0 {
                exe::call_va!(0x1_4209_a1e0, (res2, 0u32) -> ());
            }
            wr::<usize>(item, 0);
            wr::<usize>(item + 0x10, 0);
            wr::<usize>(item + 0x08, 0);
            exe::call_va!(0x1_4244_2de0, (item) -> ());
            let cur_cnt = rd::<u32>(q + 0x20);
            if cur_cnt != 0 {
                let mut next_head = rd::<u32>(q + 0x18).wrapping_add(1);
                if next_head == rd::<u32>(q + 0x24) {
                    next_head = 0;
                }
                wr::<u32>(q + 0x18, next_head);
                wr::<u32>(q + 0x20, cur_cnt - 1);
            }
            wr::<u64>(q + 0x28, rd::<u64>(q + 0x28).wrapping_sub(1));
        }
        unlock_recursive_srw(q);
        exe::call_va!(0x1_4034_96f0, (q + 0x10) -> ());
        exe::call_va!(0x1_4244_2de0, (q) -> ());
    }
    wr::<usize>(q_va, 0);
}

/// Initialize the index-buffer ring pool (`0x143a35f98` and `0x14397fc20`).
///
/// Original: `0x14205e410` (37 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_ibv_pool_init() {
    let list = exe::call_va!(0x1_40fd_31f0, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_5f98) as usize, list);
    let q = exe::call_va!(0x1_411d_7e20, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc20) as usize, q);
    exe::call_va!(0x1_411d_93b0, () -> ());
}

/// Allocate and upload `count` (`rdx`) 32-bit indices from `src` (`r9`) into the index-buffer ring (`0x143857ec0`).
///
/// Original: `0x14205e440` (419 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_ibv_alloc_and_copy(
    out_view: usize,
    count: usize,
    _r8: usize,
    src: usize,
) -> usize {
    let lock = exe::ptr::<u8>(0x1_4385_7ec0) as usize;
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_9220, () -> usize);
    let byte_size = count * 4;
    let mut alloc = [0u64; 5];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_8f90, (ring, alloc_ptr, byte_size) -> ());
    let cpu_ptr = rd::<usize>(alloc_ptr + 0x18);
    if cpu_ptr != 0 {
        let _ = exe::call_va!(0x1_4183_acbc, (cpu_ptr, src, byte_size) -> usize);
        wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_sub(1));
    }
    d3d_ibv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Release a mapped thread-local ring allocation (`[[gs:0x58] + 0x6360]`).
///
/// Original: `0x14205e6d0` (39 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ring_unmap() {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let ring_holder = rd::<usize>(tls0 + 0x6360);
    let ring = exe::call_va!(0x1_411d_9920, (ring_holder) -> usize);
    wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_sub(1));
}

/// Allocate `size` (`rdx`) bytes from the thread-local ring (`[[gs:0x58] + 0x6360]`), writing 40 bytes to `out_alloc` (`rcx`).
///
/// Original: `0x14205e700` (93 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_tls_ring_alloc(out_alloc: usize, size: usize) -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let ring_holder = rd::<usize>(tls0 + 0x6360);
    let ring = exe::call_va!(0x1_411d_9920, (ring_holder) -> usize);
    let mut tmp = [0u64; 5];
    let p = exe::call_va!(0x1_411d_9690, (ring, tmp.as_mut_ptr() as usize, size) -> usize);
    for i in 0..5usize {
        wr::<u64>(out_alloc + i * 8, rd::<u64>(p + i * 8));
    }
    out_alloc
}

/// Destroy the structured-buffer ring pool (`0x143a35fe8` and `0x14397fc98`).
///
/// Original: `0x14205ec50` (375 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sbv_pool_shutdown() {
    let list_va = exe::ptr::<usize>(0x1_43a3_5fe8) as usize;
    let list = rd::<usize>(list_va);
    if list != 0 {
        let base = rd::<usize>(list);
        let count = rd::<u32>(list + 0x08) as usize;
        for i in 0..count {
            let elem = rd::<usize>(base + i * 8);
            if elem != 0 {
                let vtbl = rd::<usize>(elem);
                let dtor = rd::<usize>(vtbl);
                exe::call_va!(dtor, (elem, 0u32) -> ());
                exe::call_va!(0x1_4244_2de0, (elem) -> ());
            }
        }
        wr::<u32>(list + 0x08, 0);
        exe::call_va!(0x1_401d_e830, (list) -> ());
        exe::call_va!(0x1_4244_2de0, (list) -> ());
    }
    wr::<usize>(list_va, 0);

    let q_va = exe::ptr::<usize>(0x1_4397_fc98) as usize;
    let q = rd::<usize>(q_va);
    if q != 0 {
        lock_recursive_srw(q);
        while rd::<u32>(q + 0x20) != 0 {
            let arr = rd::<usize>(q + 0x10);
            let head = rd::<u32>(q + 0x18) as u64;
            let cap = rd::<u32>(q + 0x24) as u64;
            let slot = (head % cap) as usize;
            let item = rd::<usize>(arr + slot * 8);
            let res = rd::<usize>(item);
            let vtbl = rd::<usize>(res);
            let unmap = rd::<usize>(vtbl + 0x48);
            exe::call_va!(unmap, (res, 0u32, 0usize) -> ());
            let srv = rd::<usize>(item + 0x18);
            let srv_heap = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
            exe::call_va!(0x1_4209_6a90, (srv_heap, srv) -> ());
            wr::<usize>(item + 0x18, 0);
            let res2 = rd::<usize>(item);
            if res2 != 0 {
                exe::call_va!(0x1_4209_a1e0, (res2, 0u32) -> ());
            }
            wr::<usize>(item, 0);
            wr::<usize>(item + 0x10, 0);
            wr::<usize>(item + 0x08, 0);
            exe::call_va!(0x1_4244_2de0, (item) -> ());
            let cur_cnt = rd::<u32>(q + 0x20);
            if cur_cnt != 0 {
                let mut next_head = rd::<u32>(q + 0x18).wrapping_add(1);
                if next_head == rd::<u32>(q + 0x24) {
                    next_head = 0;
                }
                wr::<u32>(q + 0x18, next_head);
                wr::<u32>(q + 0x20, cur_cnt - 1);
            }
            wr::<u64>(q + 0x28, rd::<u64>(q + 0x28).wrapping_sub(1));
        }
        unlock_recursive_srw(q);
        exe::call_va!(0x1_4034_96f0, (q + 0x10) -> ());
        exe::call_va!(0x1_4244_2de0, (q) -> ());
    }
    wr::<usize>(q_va, 0);
}

/// Initialize the structured-buffer ring pool (`0x143a35fe8` and `0x14397fc98`) with 32 backing pages.
///
/// Original: `0x14205edd0` (223 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_sbv_pool_init() {
    let list = exe::call_va!(0x1_40fd_31f0, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_5fe8) as usize, list);
    let q = exe::call_va!(0x1_411d_7e20, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc98) as usize, q);
    lock_recursive_srw(q);
    for _ in 0..0x20 {
        let item = exe::call_va!(0x1_411d_a620, () -> usize);
        exe::call_va!(0x1_411d_a6a0, (item) -> ());
        let cnt = rd::<u32>(q + 0x20);
        if cnt == rd::<u32>(q + 0x24) {
            exe::call_va!(0x1_4034_97a0, (q + 0x10, cnt.wrapping_add(1)) -> ());
        }
        let tail = rd::<u32>(q + 0x1c);
        let arr = rd::<usize>(q + 0x10);
        wr::<usize>(arr + (tail as usize) * 8, item);
        let mut next_tail = tail.wrapping_add(1);
        if next_tail == rd::<u32>(q + 0x24) {
            next_tail = 0;
        }
        wr::<u32>(q + 0x1c, next_tail);
        wr::<u32>(q + 0x20, rd::<u32>(q + 0x20).wrapping_add(1));
        wr::<u64>(q + 0x28, rd::<u64>(q + 0x28).wrapping_add(1));
    }
    unlock_recursive_srw(q);
}

/// Allocate and upload `size` (`rdx`) bytes from `src` (`r8`) into the structured-buffer ring (`0x143857ee0`).
///
/// Original: `0x14205eec0` (423 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_sbv_alloc_and_copy(
    out_view: usize,
    size: usize,
    src: usize,
) -> usize {
    let lock = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_a370, (0usize, (-2isize) as usize) -> usize);
    let mut alloc = [0u64; 6];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_a080, (ring, alloc_ptr, size) -> ());
    let cpu_ptr = rd::<usize>(alloc_ptr + 0x20);
    if cpu_ptr != 0 {
        let _ = exe::call_va!(0x1_4183_acbc, (cpu_ptr, src, size) -> usize);
        wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_sub(1));
    }
    d3d_sbv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Release a mapped structured-buffer ring allocation (`--ring->active_maps`).
///
/// Original: `0x14205f070` (122 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sbv_unmap(rcx: usize, _rdx: usize) {
    let lock = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_a370, (rcx, (-2isize) as usize) -> usize);
    wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_sub(1));
    unlock_recursive_srw(lock);
}

/// Allocate up to `max_size` (`rdx`) bytes from the structured-buffer ring (`0x143857ee0`).
///
/// Original: `0x14205f0f0` (402 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_sbv_alloc_clamped(out_view: usize, max_size: usize) -> usize {
    let lock = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_a370, (out_view, (-2isize) as usize) -> usize);
    let rem = 0x10_0000usize.wrapping_sub(rd::<usize>(ring + 0x20));
    let req = if rem >= 1 && rem < max_size {
        rem
    } else {
        max_size
    };
    let mut alloc = [0u64; 6];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_a080, (ring, alloc_ptr, req) -> ());
    d3d_sbv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Allocate `size` (`rdx`) bytes with alignment `align` (`r8`) from the structured-buffer ring (`0x143857ee0`).
///
/// Original: `0x14205f290` (387 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_sbv_alloc(
    out_view: usize,
    size: usize,
    align: usize,
) -> usize {
    let lock = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
    lock_recursive_srw(lock);
    let ring = exe::call_va!(0x1_411d_a370, (out_view, align) -> usize);
    let mut alloc = [0u64; 6];
    let alloc_ptr = alloc.as_mut_ptr() as usize;
    exe::call_va!(0x1_411d_a080, (ring, alloc_ptr, size) -> ());
    d3d_sbv_ctor_from_alloc(out_view, alloc_ptr);
    unlock_recursive_srw(lock);
    out_view
}

/// Map vertex format `fmt` (`cl`, `0..=0x10`) to `DXGI_FORMAT` (`eax`).
///
/// Original: `0x14205f740` (200 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_format_to_dxgi(fmt: u8) -> u32 {
    match fmt {
        0 => 0x29,
        1 => 0x10,
        2 => 6,
        3 => 2,
        4 => 0x57,
        5 => 0x1e,
        6 => 0x1c,
        7 => 0x26,
        8 => 0x0e,
        9 => 0x25,
        10 => 0x0d,
        11 => 3,
        12 => 0x11,
        13 => 0x0c,
        14 => 0x3e,
        15 => 0x2a,
        16 => 0x39,
        _ => 0,
    }
}

/// Map vertex input-slot classification byte `cls` (`cl`) to `D3D12_INPUT_CLASSIFICATION` (`1 -> 1`, else `0`).
///
/// Original: `0x14205f810` (18 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_input_slot_class(cls: u8) -> u32 {
    if cls == 1 { 1 } else { 0 }
}

/// Build `D3D12_INPUT_ELEMENT_DESC` array (`0x20` bytes per element) at `out_descs` (`rdx`)
/// from the compact vertex layout at `layout` (`rcx`, count at `+0x40`).
///
/// Original: `0x14205f4e0` (588 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_build_input_element_descs(layout: usize, out_descs: usize) {
    let mut sem_indices = [0u32; 16];
    let mut slot_offsets = [0u64; 16];
    let count = rd::<u8>(layout + 0x40) as usize;
    if count == 0 {
        return;
    }
    let sizes_table = exe::ptr::<u8>(0x1_4297_89d0) as usize;
    for i in 0..count {
        let elem = layout + i * 4;
        let slot = rd::<u8>(elem) as usize;
        let fmt_idx = rd::<u8>(elem + 1);
        let sem_idx = rd::<u8>(elem + 2);
        let step_rate = rd::<u8>(elem + 3);

        let sem_va: u64 = match sem_idx {
            0 => 0x1_433d_bfd8, // "POSITION"
            1 => 0x1_433d_bf8c, // "NORMAL"
            2 => 0x1_433d_bf80, // "TEXCOORD"
            3 => 0x1_433d_bfa0, // "TANGENT"
            4 => 0x1_433d_bf94, // "COLOR"
            5 => 0x1_433d_bfb8, // "BLENDINDICES"
            6 => 0x1_433d_bfa8, // "BLENDWEIGHT"
            7 => 0x1_433d_bfc8, // "RenderInstance"
            _ => 0,
        };
        let sem_ptr = if sem_va != 0 {
            exe::ptr::<u8>(sem_va) as usize
        } else {
            0
        };
        let dst = out_descs + i * 0x20;
        wr::<usize>(dst, sem_ptr);
        let cur_sem_index = sem_indices[(sem_idx & 0x0f) as usize];
        wr::<u32>(dst + 0x08, cur_sem_index);
        sem_indices[(sem_idx & 0x0f) as usize] = cur_sem_index.wrapping_add(1);

        wr::<u32>(dst + 0x0c, d3d_vertex_format_to_dxgi(fmt_idx));
        wr::<u32>(dst + 0x10, slot as u32);
        wr::<u32>(dst + 0x14, slot_offsets[slot & 0x0f] as u32);
        wr::<u32>(dst + 0x18, d3d_vertex_input_slot_class(step_rate));
        wr::<u32>(dst + 0x1c, u32::from(step_rate != 0));

        let elem_bytes = rd::<u8>(sizes_table + (fmt_idx as usize)) as u64;
        slot_offsets[slot & 0x0f] = slot_offsets[slot & 0x0f].wrapping_add(elem_bytes);
    }
}

#[inline(always)]
unsafe fn init_shader_wrapper_base(this: usize, src: usize, vtbl_va: u64, stage: u32) {
    let w0 = rd::<u64>(src + 0x50);
    let w1 = rd::<u64>(src + 0x48);
    let w2 = rd::<u64>(src + 0x60);
    wr::<usize>(this, exe::ptr::<u8>(vtbl_va) as usize);
    wr::<u64>(this + 0x08, w0);
    wr::<u64>(this + 0x10, w1);
    wr::<u32>(this + 0x18, stage);
    wr::<u64>(this + 0x20, w2);
}

/// Construct a `d3d::PixelShader` wrapper (`vtbl = 0x1433dc8b8`, `stage = 2`).
///
/// Original: `0x142060820` (72 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_pixel_shader_ctor(this: usize, src: usize) -> usize {
    init_shader_wrapper_base(this, src, 0x1_433d_c8b8, 2);
    wr::<u64>(this + 0x28, 0);
    this
}

/// Construct a `d3d::VertexShader` wrapper (`vtbl = 0x1433dc898`, `stage = 1`, `layout_id` at `+0x28`).
///
/// Original: `0x142060870` (68 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_shader_ctor(
    this: usize,
    src: usize,
    layout_id: u32,
) -> usize {
    init_shader_wrapper_base(this, src, 0x1_433d_c898, 1);
    wr::<u32>(this + 0x28, layout_id);
    this
}

/// Construct a `d3d::GeometryShader` wrapper (`vtbl = 0x1433dc888`, `stage = 4`).
///
/// Original: `0x1420608c0` (64 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_geometry_shader_ctor(this: usize, src: usize) -> usize {
    init_shader_wrapper_base(this, src, 0x1_433d_c888, 4);
    this
}

/// Construct a `d3d::HullShader` wrapper (`vtbl = 0x1433dc8a8`, `stage = 3`).
///
/// Original: `0x142060910` (64 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_hull_shader_ctor(this: usize, src: usize) -> usize {
    init_shader_wrapper_base(this, src, 0x1_433d_c8a8, 3);
    this
}

/// Construct a `d3d::DomainShader` wrapper (`vtbl = 0x1433dc8c8`, `stage = 5`).
///
/// Original: `0x142060960` (64 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_domain_shader_ctor(this: usize, src: usize) -> usize {
    init_shader_wrapper_base(this, src, 0x1_433d_c8c8, 5);
    this
}

/// Construct a `d3d::ComputeShader` wrapper (`vtbl = 0x1433dc878`, `stage = 0`, inline PSO cache at `+0x28`).
///
/// Original: `0x1420609b0` (103 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_compute_shader_ctor(this: usize, src: usize) -> usize {
    init_shader_wrapper_base(this, src, 0x1_433d_c878, 0);
    wr::<usize>(this + 0x28, this + 0x38);
    wr::<u32>(this + 0x30, 0);
    wr::<u32>(this + 0x34, 2);
    wr::<u64>(this + 0x38, 0);
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
    wr::<u8>(this + 0x58, 0);
    this
}

/// Populate a 16-byte `(bytecode_ptr, bytecode_len)` view at `out` (`rcx`) from shader object `shader` (`r8`).
///
/// Original: `0x142060a20` (49 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_copy_shader_bytecode_view(
    out: usize,
    _rdx: usize,
    shader: usize,
) {
    let (ptr, len) = if shader != 0 {
        (rd::<u64>(shader), rd::<u32>(shader + 0x10))
    } else {
        (0, 0)
    };
    wr::<u64>(out, ptr);
    wr::<u32>(out + 0x08, len);
    wr::<u32>(out + 0x0c, 0);
}

/// Populate a 16-byte `(bytecode_ptr, bytecode_len)` view at `out` (`rcx`) from shader object `shader` (`r8`) and return `out`.
///
/// Original: `0x142060a60` (52 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_copy_shader_bytecode_view_ret(
    out: usize,
    rdx: usize,
    shader: usize,
) -> usize {
    d3d_copy_shader_bytecode_view(out, rdx, shader);
    out
}

/// Zero a 16-byte shader bytecode view at `out` (`rcx`) and return `out`.
///
/// Original: `0x142060aa0` (31 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_zero_shader_bytecode_view(out: usize) -> usize {
    wr::<u64>(out, 0);
    wr::<u64>(out + 0x08, 0);
    out
}

/// Resolve an SRV descriptor handle into `[out]` (`rcx`) from view dimension `dim` (`edx`) and resource `res` (`r8`).
///
/// Original: `0x142060ad0` (164 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_resolve_srv_descriptor(out: usize, dim: u32, res: usize) {
    let _ = d3d_resolve_srv_descriptor_ret(out, dim, res);
}

/// Resolve an SRV descriptor handle into `[out]` (`rcx`) from view dimension `dim` (`edx`) and resource `res` (`r8`), returning `out`.
///
/// Original: `0x142060b80` (155 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_resolve_srv_descriptor_ret(
    out: usize,
    dim: u32,
    res: usize,
) -> usize {
    let default_mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let val = match dim {
        2 => {
            let v = if res != 0 { rd::<u64>(res + 0x18) } else { 0 };
            if v != 0 { v } else { rd::<u64>(default_mgr + 0xa8) }
        }
        3 => {
            let v = if res != 0 { rd::<u64>(res + 0x20) } else { 0 };
            if v != 0 { v } else { rd::<u64>(default_mgr + 0x98) }
        }
        4 => {
            let v = if res != 0 { rd::<u64>(res + 0x28) } else { 0 };
            if v != 0 { v } else { rd::<u64>(default_mgr + 0x98) }
        }
        6 => {
            let v = if res != 0 { rd::<u64>(res + 0x30) } else { 0 };
            if v != 0 { v } else { rd::<u64>(default_mgr + 0xa0) }
        }
        7 => {
            let v = if res != 0 { rd::<u64>(res + 0x38) } else { 0 };
            if v != 0 { v } else { rd::<u64>(default_mgr + 0xa0) }
        }
        _ => rd::<u64>(default_mgr + 0x98),
    };
    wr::<u64>(out, val);
    out
}

/// Resolve a UAV descriptor + resource pair into `out` (`rcx`, 16 bytes) from `dim` (`edx`) and `res` (`r8`).
///
/// Original: `0x142060c50` (96 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_resolve_uav_descriptor(out: usize, dim: u32, res: usize) {
    let _ = d3d_resolve_uav_descriptor_ret(out, dim, res);
}

/// Resolve a UAV descriptor + resource pair into `out` (`rcx`, 16 bytes) from `dim` (`edx`) and `res` (`r8`), returning `out`.
///
/// Original: `0x142060cc0` (111 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_resolve_uav_descriptor_ret(
    out: usize,
    dim: u32,
    res: usize,
) -> usize {
    wr::<usize>(out + 0x08, res);
    let default_mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let val = if dim == 0 {
        if res != 0 {
            rd::<u64>(res + 0x78)
        } else {
            rd::<u64>(default_mgr + 0x88)
        }
    } else if dim == 5 {
        if res != 0 {
            rd::<u64>(res + 0x90)
        } else {
            rd::<u64>(default_mgr + 0x90)
        }
    } else {
        rd::<u64>(default_mgr + 0x88)
    };
    wr::<u64>(out, val);
    out
}

/// Write `src != 0 ? *src : 0` to `*out` (`rcx`).
///
/// Original: `0x142060d60` (20 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_deref_or_zero(out: usize, src: usize) {
    wr::<u64>(out, if src != 0 { rd::<u64>(src) } else { 0 });
}

/// Write `src != 0 ? *src : 0` to `*out` (`rcx`) and return `out`.
///
/// Original: `0x142060d80` (26 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_deref_or_zero_ret(out: usize, src: usize) -> usize {
    d3d_deref_or_zero(out, src);
    out
}

/// Write `*0x143a34b68 != 0 ? **0x143a34b68 : 0` to `*out` (`rcx`) and return `out`.
///
/// Original: `0x142060da0` (33 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_global_34b68_deref(out: usize) -> usize {
    let p = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4b68) as usize);
    wr::<u64>(out, if p != 0 { rd::<u64>(p) } else { 0 });
    out
}

/// Scalar deleting destructor releasing COM pointer at `*(this + 0x08)` and freeing `0xb0` bytes when `flags & 1 != 0`.
///
/// Original: `0x1420610d0` (71 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_com_holder_b0_deleting_dtor(this: usize, flags: u32) -> usize {
    let com = rd::<usize>(this + 0x08);
    if com != 0 {
        let vtbl = rd::<usize>(com);
        let release = rd::<usize>(vtbl + 0x10);
        exe::call_va!(release, (com) -> u32);
    }
    wr::<usize>(this + 0x08, 0);
    if (flags & 1) != 0 {
        exe::call_va!(0x1_4183_4d30, (this, 0xb0u32) -> ());
    }
    this
}

/// Destroy the command-context queues (`0x14397fc30`, `0x14397fc38`, `0x143a36078`, `0x143a36148`).
///
/// Original: `0x142060060` (605 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_cmd_context_pool_shutdown() {
    for &(q_va, has_vec_at_0) in &[(0x1_4397_fc30u64, true), (0x1_4397_fc38, false)] {
        let va = exe::ptr::<usize>(q_va) as usize;
        let q = rd::<usize>(va);
        if q != 0 {
            lock_recursive_srw(q);
            while rd::<u32>(q + 0x20) != 0 {
                let arr = rd::<usize>(q + 0x10);
                let head = rd::<u32>(q + 0x18) as u64;
                let cap = rd::<u32>(q + 0x24) as u64;
                let item = rd::<usize>(arr + ((head % cap) as usize) * 8);
                if item != 0 {
                    if has_vec_at_0 {
                        exe::call_va!(0x1_401d_e830, (item) -> ());
                    }
                    exe::call_va!(0x1_4244_2de0, (item) -> ());
                }
                let cnt = rd::<u32>(q + 0x20);
                if cnt != 0 {
                    let mut next = rd::<u32>(q + 0x18).wrapping_add(1);
                    if next == rd::<u32>(q + 0x24) {
                        next = 0;
                    }
                    wr::<u32>(q + 0x18, next);
                    wr::<u32>(q + 0x20, cnt - 1);
                }
                wr::<u64>(q + 0x28, rd::<u64>(q + 0x28).wrapping_sub(1));
            }
            unlock_recursive_srw(q);
            exe::call_va!(0x1_4034_96f0, (q + 0x10) -> ());
            exe::call_va!(0x1_4244_2de0, (q) -> ());
        }
        wr::<usize>(va, 0);
    }

    let listeners_va = exe::ptr::<usize>(0x1_43a3_6078) as usize;
    let listeners = rd::<usize>(listeners_va);
    if listeners != 0 {
        exe::call_va!(0x1_401d_e830, (listeners) -> ());
        exe::call_va!(0x1_4244_2de0, (listeners) -> ());
    }
    wr::<usize>(listeners_va, 0);

    let q3_va = exe::ptr::<usize>(0x1_43a3_6148) as usize;
    let q3 = rd::<usize>(q3_va);
    if q3 != 0 {
        lock_recursive_srw(q3);
        while rd::<u32>(q3 + 0x20) != 0 {
            let arr = rd::<usize>(q3 + 0x10);
            let head = rd::<u32>(q3 + 0x18) as u64;
            let cap = rd::<u32>(q3 + 0x24) as u64;
            let item = rd::<usize>(arr + ((head % cap) as usize) * 8);
            if item != 0 {
                exe::call_va!(0x1_401d_e830, (item) -> ());
                exe::call_va!(0x1_4244_2de0, (item) -> ());
            }
            let cnt = rd::<u32>(q3 + 0x20);
            if cnt != 0 {
                let mut next = rd::<u32>(q3 + 0x18).wrapping_add(1);
                if next == rd::<u32>(q3 + 0x24) {
                    next = 0;
                }
                wr::<u32>(q3 + 0x18, next);
                wr::<u32>(q3 + 0x20, cnt - 1);
            }
            wr::<u64>(q3 + 0x28, rd::<u64>(q3 + 0x28).wrapping_sub(1));
        }
        unlock_recursive_srw(q3);
        exe::call_va!(0x1_4034_96f0, (q3 + 0x10) -> ());
        exe::call_va!(0x1_4244_2de0, (q3) -> ());
    }
    wr::<usize>(q3_va, 0);
}

/// Initialize the command-context queues (`0x14397fc38`, `0x14397fc30`, `0x143a36078`, `0x143a36148`).
///
/// Original: `0x1420602d0` (57 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_cmd_context_pool_init() {
    let q0 = exe::call_va!(0x1_411d_7e20, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc38) as usize, q0);
    let q1 = exe::call_va!(0x1_411d_7e20, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc30) as usize, q1);
    let list = exe::call_va!(0x1_40fd_31f0, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_6078) as usize, list);
    let q2 = exe::call_va!(0x1_411d_7e20, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_6148) as usize, q2);
}

/// Remove `listener` (`rcx`) from the command-listener vector `*0x143a36078` under lock `0x143857ef0`.
///
/// Original: `0x142060310` (246 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unregister_cmd_listener(listener: usize) {
    let lock = exe::ptr::<u8>(0x1_4385_7ef0) as usize;
    lock_recursive_srw(lock);
    let vec = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6078) as usize);
    let base = rd::<usize>(vec);
    let count = rd::<u32>(vec + 0x08) as usize;
    let end = base + count * 8;
    let mut write_ptr = base;
    while write_ptr != end && rd::<usize>(write_ptr) != listener {
        write_ptr += 8;
    }
    if write_ptr != end {
        let mut read_ptr = write_ptr + 8;
        while read_ptr != end {
            let val = rd::<usize>(read_ptr);
            if val != listener {
                wr::<usize>(write_ptr, val);
                write_ptr += 8;
            }
            read_ptr += 8;
        }
    }
    let base2 = rd::<usize>(vec);
    let count2 = rd::<u32>(vec + 0x08) as usize;
    let end2 = base2 + count2 * 8;
    if write_ptr != end2 {
        if write_ptr == base2 {
            wr::<u32>(vec + 0x08, 0);
        } else {
            let _ = exe::call_va!(0x1_4183_acc8, (write_ptr, end2, 0usize) -> usize);
            let removed = (end2 - write_ptr) >> 3;
            wr::<u32>(vec + 0x08, (count2 - removed) as u32);
        }
    }
    unlock_recursive_srw(lock);
}

/// Append `listener` (`rcx`) to the command-listener vector `*0x143a36078` under lock `0x143857ef0`.
///
/// Original: `0x142060410` (154 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_register_cmd_listener(listener: usize) {
    let lock = exe::ptr::<u8>(0x1_4385_7ef0) as usize;
    lock_recursive_srw(lock);
    let vec = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6078) as usize);
    let count = rd::<u32>(vec + 0x08);
    let cap = rd::<u32>(vec + 0x0c);
    if count == cap {
        exe::call_va!(0x1_4020_bcb0, (vec, (count as usize) + 1) -> ());
    }
    let idx = rd::<u32>(vec + 0x08) as usize;
    let base = rd::<usize>(vec);
    wr::<usize>(base + idx * 8, listener);
    wr::<u32>(vec + 0x08, (idx + 1) as u32);
    unlock_recursive_srw(lock);
}

/// Get or lazily create the `ID3D12PipelineState` for compute shader `cs` (`rcx`).
///
/// Original: `0x1420604d0` (300 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_get_or_create_compute_pso(cs: usize) -> usize {
    let cached = rd::<usize>(cs + 0x28);
    if cached != 0 {
        return cached;
    }
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc50) as usize);
    let mut key: u64 = rd::<u64>(cs + 0x20);
    let lock = mgr + 0x20;
    let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock) -> ());
    let mut iter_pair = [0usize; 2];
    exe::call_va!(
        0x1_411d_bd00,
        (mgr + 0x38, iter_pair.as_mut_ptr() as usize, &mut key as *mut u64 as usize) -> usize
    );
    let entry = iter_pair[1];
    if rd::<u8>(entry + 0x08) != 2 {
        let dev_ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
        let mut pso_desc = [0u64; 7];
        pso_desc[0] = rd::<u64>(dev_ctx + 0x30); // pRootSignature
        pso_desc[1] = rd::<u64>(cs + 0x10); // CS.pShaderBytecode
        pso_desc[2] = rd::<u64>(cs + 0x08); // CS.BytecodeLength
        let mut new_pso: usize = 0;
        let d3d_dev = rd::<usize>(dev_ctx);
        let vtbl = rd::<usize>(d3d_dev);
        let create_compute_pso = rd::<usize>(vtbl + 0x58);
        let riid = exe::ptr::<u8>(0x1_433d_cb10) as usize;
        let hr = exe::call_va!(
            create_compute_pso,
            (d3d_dev, pso_desc.as_ptr() as usize, riid, &mut new_pso as *mut usize as usize) -> u32
        );
        if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
            exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
        }
        let old_pso = rd::<usize>(entry + 0x10);
        if old_pso != 0 {
            let old_vtbl = rd::<usize>(old_pso);
            let release = rd::<usize>(old_vtbl + 0x10);
            exe::call_va!(release, (old_pso) -> u32);
        }
        wr::<usize>(entry + 0x10, new_pso);
        wr::<u8>(entry + 0x08, 2);
    }
    let pso = rd::<usize>(entry + 0x10);
    let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
    wr::<usize>(cs + 0x28, pso);
    pso
}

/// Build the `D3D12_INPUT_LAYOUT_DESC` at `*(0x1411dad70())` for vertex shader `vs` (`rdx`).
///
/// Original: `0x142060720` (94 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_build_vertex_shader_input_layout(rcx: usize, vs: usize) {
    let descs = exe::call_va!(0x1_4206_0610, (rcx, vs, 0usize, 0x10u32) -> usize);
    let layout_id = rd::<u32>(vs) as usize;
    let layouts_base = exe::ptr::<u8>(0x1_43a4_9130) as usize;
    d3d_build_input_element_descs(layouts_base + layout_id * 0x68, descs);
    let layout_id2 = rd::<u32>(vs) as usize;
    let count = rd::<u8>(layouts_base + layout_id2 * 0x68 + 0x40) as u32;
    let slot = exe::call_va!(0x1_411d_ad70, (layout_id2) -> usize);
    wr::<u32>(slot + 0x08, count);
    wr::<usize>(slot, descs);
}

/// Destroy a `d3d::ComputeShader` (`rcx`, vtbl `0x1433dc878`), freeing its inline PSO cache at `+0x28`.
///
/// Original: `0x142060790` (123 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_compute_shader_dtor(this: usize) {
    wr::<usize>(this, exe::ptr::<u8>(0x1_433d_c878) as usize);
    let cache = this + 0x28;
    let base = rd::<usize>(cache);
    let count = rd::<u32>(this + 0x30) as usize;
    for i in 0..count {
        let node = rd::<usize>(base + i * 0x10 + 0x08);
        if node != 0 {
            let inner = rd::<usize>(node);
            if inner != 0 {
                exe::call_va!(0x1_4244_2de0, (inner) -> ());
            }
            exe::call_va!(0x1_4244_2de0, (node) -> ());
        }
    }
    exe::call_va!(0x1_40ec_18c0, (cache) -> ());
}

/// Query `(x, y, z)` thread-group size via `ID3D12ShaderReflection::GetThreadGroupSize` (`vtbl + 0xa0`).
///
/// Original: `0x142061120` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_shader_reflection_get_thread_group_size(
    this: usize,
    out_xyz: usize,
) -> usize {
    let refl = rd::<usize>(this + 0x08);
    let vtbl = rd::<usize>(refl);
    let get_tg = rd::<usize>(vtbl + 0xa0);
    let (mut x, mut y, mut z) = (0u32, 0u32, 0u32);
    exe::call_va!(
        get_tg,
        (
            refl,
            &mut x as *mut u32 as usize,
            &mut y as *mut u32 as usize,
            &mut z as *mut u32 as usize
        ) -> u32
    );
    wr::<u32>(out_xyz, x);
    wr::<u32>(out_xyz + 0x04, y);
    wr::<u32>(out_xyz + 0x08, z);
    out_xyz
}

/// Query parameter description via `ID3D12ShaderReflection::GetInputParameterDesc` (`vtbl + 0x30`).
///
/// Original: `0x142061180` (62 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_shader_reflection_get_input_param_desc(
    this: usize,
    out_desc: usize,
    param_idx: u32,
) -> usize {
    let refl = rd::<usize>(this + 0x08);
    let vtbl = rd::<usize>(refl);
    let get_param = rd::<usize>(vtbl + 0x30);
    let hr = exe::call_va!(get_param, (refl, param_idx, out_desc) -> u32);
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
    }
    out_desc
}

/// Destroy `d3d::ShaderReflection` (`rcx`), freeing vectors at `+0x20` and `+0x10` and releasing COM at `+0x08`.
///
/// Original: `0x142061270` (58 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_shader_reflection_dtor(this: usize) {
    exe::call_va!(0x1_401d_e830, (this + 0x20) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x10) -> ());
    let refl = rd::<usize>(this + 0x08);
    if refl != 0 {
        let vtbl = rd::<usize>(refl);
        let release = rd::<usize>(vtbl + 0x10);
        exe::call_va!(release, (refl) -> u32);
    }
    wr::<usize>(this + 0x08, 0);
}

/// Scalar deleting destructor for `d3d::ShaderReflection` (`size = 0x30`).
///
/// Original: `0x1420612b0` (91 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_shader_reflection_deleting_dtor(
    this: usize,
    flags: u32,
) -> usize {
    d3d_shader_reflection_dtor(this);
    if (flags & 1) != 0 {
        exe::call_va!(0x1_4183_4d30, (this, 0x30u32) -> ());
    }
    this
}

/// Copy a 48-byte entry at index `idx` (`r8d`) from the array at `*(this + 0x10)` into `out` (`rdx`).
///
/// Original: `0x142061350` (43 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_copy_entry_48_at_10(
    this: usize,
    out: usize,
    idx: i32,
) -> usize {
    let base = rd::<usize>(this + 0x10);
    let src = (base as isize + (idx as isize) * 48) as usize;
    for i in 0..6usize {
        wr::<u64>(out + i * 8, rd::<u64>(src + i * 8));
    }
    out
}

/// Return the `u64` at index `idx` (`edx`) from the array at `*(this + 0x20)`.
///
/// Original: `0x142061390` (12 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_qword_at_20(this: usize, idx: i32) -> u64 {
    let base = rd::<usize>(this + 0x20);
    rd::<u64>((base as isize + (idx as isize) * 8) as usize)
}

/// Recreate the sampler descriptor for `sampler` (`rcx`) with `mip_lod_bias` (`xmm1`) via `ID3D12Device::CreateSampler` (`vtbl + 0xb0`).
///
/// Original: `0x1420613d0` (74 bytes, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn d3d_recreate_sampler_descriptor(sampler: usize, mip_lod_bias: f32) {
    d3d_recreate_sampler_descriptor_impl(sampler, mip_lod_bias);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn d3d_recreate_sampler_descriptor(
    sampler: usize,
    _xmm0: f32,
    mip_lod_bias: f32,
) {
    d3d_recreate_sampler_descriptor_impl(sampler, mip_lod_bias);
}

#[inline(always)]
unsafe fn d3d_recreate_sampler_descriptor_impl(sampler: usize, mip_lod_bias: f32) {
    let mut desc = [0u32; 13];
    for i in 0..13usize {
        desc[i] = rd::<u32>(sampler + 0x08 + i * 4);
    }
    desc[4] = mip_lod_bias.to_bits();
    let handle_ptr = rd::<usize>(sampler);
    let cpu_handle = rd::<u64>(handle_ptr);
    let d3d_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let vtbl = rd::<usize>(d3d_dev);
    let create_sampler = rd::<usize>(vtbl + 0xb0);
    exe::call_va!(create_sampler, (d3d_dev, desc.as_ptr() as usize, cpu_handle) -> ());
}

/// Dispatch rays variant 1 (`mode = 1`), forwarding to `0x142061a90`.
///
/// Original: `0x142061690` (67 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_rays_1(
    _rcx: usize,
    w: u32,
    h: u32,
    _r9: usize,
    arg5: u64,
    arg6: usize,
) {
    let local = arg5;
    d3d_dispatch_rays_common(w, h, 1, 0, &local as *const u64 as usize, arg6);
}

/// Dispatch rays variant 4 (`mode = 4`), forwarding 32-byte payload `*arg5` to `0x142061a90`.
///
/// Original: `0x1420616e0` (75 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_rays_4(
    _rcx: usize,
    w: u32,
    h: u32,
    _r9: usize,
    arg5: usize,
    arg6: usize,
) {
    let mut local = [0u64; 4];
    for i in 0..4usize {
        local[i] = rd::<u64>(arg5 + i * 8);
    }
    d3d_dispatch_rays_common(w, h, 4, 0, local.as_ptr() as usize, arg6);
}

/// Dispatch rays variant 3 (`mode = 3`), forwarding 32-byte payload `*arg5` to `0x142061a90`.
///
/// Original: `0x142061740` (75 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_rays_3(
    _rcx: usize,
    w: u32,
    h: u32,
    _r9: usize,
    arg5: usize,
    arg6: usize,
) {
    let mut local = [0u64; 4];
    for i in 0..4usize {
        local[i] = rd::<u64>(arg5 + i * 8);
    }
    d3d_dispatch_rays_common(w, h, 3, 0, local.as_ptr() as usize, arg6);
}

/// Common ray-dispatch / resource-state command packet builder (`0x142061a90`).
///
/// Original: `0x142061a90` (224 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_rays_common(
    w: u32,
    h: u32,
    mode: u32,
    _r9: usize,
    geom_src: usize,
    state_obj: usize,
) {
    let mut geom = [0u32; 12];
    geom[0] = w;
    geom[1] = h;
    geom[2] = mode;
    geom[3] = 1;
    d3d_fill_raytracing_geom_desc(geom.as_mut_ptr() as usize, geom_src);

    let mut cmd = [0u64; 18];
    let cmd_ptr = cmd.as_mut_ptr() as usize;
    wr::<u32>(cmd_ptr, 0x10010);
    wr::<usize>(cmd_ptr + 0x08, geom.as_ptr() as usize);
    wr::<u32>(cmd_ptr + 0x40, 0);
    wr::<u64>(cmd_ptr + 0x48, rd::<u64>(state_obj));
    wr::<u64>(cmd_ptr + 0x50, rd::<u64>(state_obj + 0x08));
    wr::<u64>(cmd_ptr + 0x58, rd::<u64>(state_obj + 0x10));
    let w18 = rd::<u64>(state_obj + 0x18);
    wr::<u64>(cmd_ptr + 0x60, w18);
    wr::<u64>(cmd_ptr + 0x68, rd::<u64>(state_obj + 0x20));
    wr::<u64>(cmd_ptr + 0x70, w18);
    wr::<u64>(cmd_ptr + 0x78, rd::<u64>(state_obj + 0x30));
    wr::<u64>(cmd_ptr + 0x80, rd::<u64>(state_obj + 0x38));
    wr::<u64>(cmd_ptr + 0x88, rd::<u64>(state_obj + 0x40));

    let mut lock_guard = [0usize; 2];
    let ctx = exe::call_va!(0x1_411d_b050, (lock_guard.as_mut_ptr() as usize) -> usize);
    exe::call_va!(0x1_4000_1870, (rd::<usize>(ctx + 0x08), cmd_ptr) -> ());
    if lock_guard[0] != 0 {
        unlock_recursive_srw(lock_guard[0]);
    }
}

/// Map index-buffer stride byte `stride` (`cl`) to `DXGI_FORMAT` (`2 -> 6`, `10 -> 0xd`, else `0`).
///
/// Original: `0x142061db0` (25 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_index_stride_to_dxgi_format(stride: u8) -> u32 {
    if stride == 2 {
        6
    } else if stride == 10 {
        0x0d
    } else {
        0
    }
}

/// Convert a 32-byte compact vertex/index view at `src` (`rdx`) into `out` (`rcx`) and return `out`.
///
/// Original: `0x142061d50` (81 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_convert_vertex_and_index_views_32(
    out: usize,
    src: usize,
) -> usize {
    wr::<u32>(out, d3d_index_stride_to_dxgi_format(rd::<u8>(src)));
    for i in 1..8usize {
        wr::<u32>(out + i * 4, rd::<u32>(src + i * 4));
    }
    out
}

/// Populate the geometry payload at `desc + 0x10` (`rcx`) from `src` (`rdx`) based on `*(desc + 0x08)`.
///
/// Original: `0x142061c40` (173 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_fill_raytracing_geom_desc(desc: usize, src: usize) {
    match rd::<u32>(desc + 0x08) {
        0 | 1 => {
            wr::<u32>(desc + 0x10, rd::<u32>(src));
            wr::<u32>(desc + 0x14, rd::<u32>(src + 0x04));
        }
        2..=4 => {
            let _ = d3d_convert_vertex_and_index_views_32(desc + 0x10, src);
        }
        _ => {}
    }
}

/// Pack two 32-bit words at `[rcx]` and `[rcx + 4]` into a `u64` (`(hi << 32) | lo`).
///
/// Original: `0x142061d30` (13 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_pack_two_dwords_u64(rcx: usize) -> u64 {
    let lo = rd::<u32>(rcx) as u64;
    let hi = rd::<u32>(rcx + 0x04) as u64;
    (hi << 32) | lo
}

/// Unmap the active compaction readback buffer at `this + 8 * (*(this + 0x20) & 1) + 8` and clear `*(this + 0x18)`.
///
/// Original: `0x142061df0` (41 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_rt_compaction_pool_unmap(this: usize) {
    let idx = (rd::<u32>(this + 0x20) & 1) as usize;
    let buf = rd::<usize>(this + 0x08 + idx * 8);
    exe::call_va!(0x1_4209_d6f0, (buf, 0u32) -> ());
    wr::<usize>(this + 0x18, 0);
}

/// Map the compaction readback buffer for frame `frame_idx` (`edx`) and store the mapped pointer at `this + 0x18`.
///
/// Original: `0x142061e20` (44 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_rt_compaction_pool_map(this: usize, frame_idx: u32) {
    wr::<u32>(this + 0x20, frame_idx);
    let idx = (frame_idx & 1) as usize;
    let buf = rd::<usize>(this + 0x08 + idx * 8);
    let byte_len = rd::<u64>(buf + 0x10);
    let ptr = exe::call_va!(0x1_4209_c560, (buf, frame_idx, byte_len, 0u32) -> usize);
    wr::<usize>(this + 0x18, ptr);
}

/// Copy `*(this + 0x28)` compaction query results into the current readback buffer and advance `*(this + 0x30)`.
///
/// Original: `0x142061e60` (90 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_rt_compaction_pool_resolve(this: usize) {
    let count = rd::<u32>(this + 0x28);
    if count > 0 {
        let src_buf = rd::<usize>(this);
        exe::call_va!(0x1_4207_5850, (src_buf) -> ());
        let dst_idx = (rd::<u32>(this + 0x30) & 1) as usize;
        let dst_buf = rd::<usize>(this + 0x08 + dst_idx * 8);
        let byte_size = (count as u64) << 3;
        exe::call_va!(0x1_4207_5bc0, (dst_buf, 0u64, rd::<usize>(this), 0u64, byte_size) -> ());
    }
    wr::<u32>(this + 0x28, 0);
    wr::<u32>(this + 0x30, rd::<u32>(this + 0x30).wrapping_add(1));
}

/// Return `*((*(this + 0x18)) + (*(rdx + 0x0c)) * 8)`.
///
/// Original: `0x142062780` (12 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_indexed_ptr_at_18(this: usize, rdx: usize) -> usize {
    let idx = rd::<u32>(rdx + 0x0c) as usize;
    let arr = rd::<usize>(this + 0x18);
    rd::<usize>(arr + idx * 8)
}

/// Return whether query `this` (`rcx`) belongs to an older generation than its backing block (`*(this + 0x0c) < *(*this + 0x30)`).
///
/// Original: `0x1420640e0` (13 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_query_is_stale(this: usize) -> bool {
    let block = rd::<usize>(this);
    let block_gen = rd::<i32>(block + 0x30);
    rd::<i32>(this + 0x0c) < block_gen
}

/// Map `d3d::QueryType` (`ecx`) to `D3D12_QUERY_TYPE` (`0 -> 0`, non-zero -> `2`).
///
/// Original: `0x142064100` (13 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_query_type_to_d3d12(query_type: u32) -> u32 {
    if query_type != 0 { 2 } else { 0 }
}

/// Check whether the GPU fence on query `this`'s backing block has reached the block's fence value.
///
/// Original: `0x142064090` (65 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_query_is_fence_complete(this: usize) -> bool {
    let block = rd::<usize>(this);
    let fence = rd::<usize>(block + 0x10);
    let vtbl = rd::<usize>(fence);
    let get_completed = rd::<usize>(vtbl + 0x40);
    let completed = exe::call_va!(get_completed, (fence) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
    }
    let block2 = rd::<usize>(this);
    completed >= rd::<u64>(block2)
}

/// Read slot `slot` (`r8d`) from query block `block` (`rcx`) into `*out_val` (`rdx`) via `ID3D12Resource::Map`/`Unmap`.
///
/// Original: `0x142063b40` (159 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_query_block_read_slot(
    block: usize,
    out_val: usize,
    slot: i32,
) -> i32 {
    let begin_byte = ((slot as isize) * 8) as u64;
    let end_byte = (((slot + 1) as isize) * 8) as u64;
    let read_range: [u64; 2] = [begin_byte, end_byte];
    let mut mapped: usize = 0;
    let res = rd::<usize>(block + 0x28);
    let vtbl = rd::<usize>(res);
    let map_fn = rd::<usize>(vtbl + 0x40);
    let hr = exe::call_va!(
        map_fn,
        (res, 0u32, read_range.as_ptr() as usize, &mut mapped as *mut usize as usize) -> i32
    );
    if hr >= 0 {
        wr::<u64>(out_val, rd::<u64>(mapped.wrapping_add(begin_byte as usize)));
    }
    let res2 = rd::<usize>(block + 0x28);
    let vtbl2 = rd::<usize>(res2);
    let unmap_fn = rd::<usize>(vtbl2 + 0x48);
    let empty_range: [u64; 2] = [0, 0];
    exe::call_va!(unmap_fn, (res2, 0u32, empty_range.as_ptr() as usize) -> ());
    hr
}

/// Read the 64-bit result of query `this` (`rcx`) into `*out_val` (`rdx`).
///
/// Original: `0x142063fe0` (161 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_query_read_slot(this: usize, out_val: usize) -> i32 {
    let slot = rd::<i32>(this + 0x08);
    let block = rd::<usize>(this);
    d3d_query_block_read_slot(block, out_val, slot)
}

/// Release the `ID3D12Resource` (`+0x28`) and `ID3D12QueryHeap` (`+0x18`) owned by query block `this` (`rcx`).
///
/// Original: `0x142063bf0` (61 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_query_block_dtor(this: usize) {
    let res = rd::<usize>(this + 0x28);
    if res != 0 {
        let vtbl = rd::<usize>(res);
        let release = rd::<usize>(vtbl + 0x10);
        exe::call_va!(release, (res) -> u32);
    }
    wr::<usize>(this + 0x28, 0);
    let heap = rd::<usize>(this + 0x18);
    if heap != 0 {
        let vtbl = rd::<usize>(heap);
        let release = rd::<usize>(vtbl + 0x10);
        exe::call_va!(release, (heap) -> u32);
    }
    wr::<usize>(this + 0x18, 0);
}

/// Resolve all active queries in `block` (`rcx`) via `ID3D12GraphicsCommandList::ResolveQueryData` (`vtbl + 0x1b0`) and record `fence_val` (`rdx`).
///
/// Original: `0x142063ab0` (133 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_query_block_resolve(block: usize, fence_val: u64) {
    let mut cmd_guard = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_guard.as_mut_ptr() as usize) -> usize);
    let cmd_list = cmd_guard[1];
    let vtbl = rd::<usize>(cmd_list);
    let resolve_fn = rd::<usize>(vtbl + 0x1b0);
    let dst_buf = rd::<usize>(block + 0x28);
    let count = rd::<u32>(block + 0x34);
    let qtype = rd::<u32>(block + 0x20);
    let heap = rd::<usize>(block + 0x18);
    exe::call_va!(
        resolve_fn,
        (cmd_list, heap, qtype, 0u32, count, dst_buf, 0u64) -> ()
    );
    if cmd_guard[0] != 0 {
        unlock_recursive_srw(cmd_guard[0]);
    }
    wr::<u64>(block, fence_val);
}

/// Poll query `this` (`rcx`) and write its result to `*out_val` (`rdx`), returning `0` (ready), `1` (pending), or `2` (stale/error).
///
/// Original: `0x142063180` (109 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_query_try_get_result(this: usize, out_val: usize) -> u32 {
    let block = rd::<usize>(this);
    if block == 0 || rd::<i32>(this + 0x0c) < rd::<i32>(block + 0x30) {
        return 2;
    }
    if !d3d_query_is_fence_complete(this) {
        return 1;
    }
    let hr = d3d_query_read_slot(this, out_val);
    if hr >= 0 { 0 } else { 2 }
}

/// Allocate a query slot of type `qtype` (`r8d`) from `*0x14397fc48` into `out_slot` (`rdx`, 16 bytes).
///
/// Original: `0x142063e60` (376 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_query_heap_mgr_alloc_slot(
    _rcx: usize,
    out_slot: usize,
    qtype: i32,
) -> usize {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc48) as usize);
    let lock = mgr + 0x08;
    lock_recursive_srw(lock);
    let t = qtype as isize;
    let cur_slot_va = (mgr as isize + t * 8 + 0xa8) as usize;
    let mut block = rd::<usize>(cur_slot_va);
    if rd::<u32>(block + 0x34) == 0x80 {
        let active_q = (mgr as isize + (t + 3) * 24) as usize;
        let cnt = rd::<u32>(active_q + 0x10);
        if cnt == rd::<u32>(active_q + 0x14) {
            exe::call_va!(0x1_411d_aad0, (active_q, cnt.wrapping_add(1)) -> ());
        }
        let tail = rd::<u32>(active_q + 0x0c);
        wr::<usize>(rd::<usize>(active_q) + (tail as usize) * 8, rd::<usize>(cur_slot_va));
        let mut next_tail = tail.wrapping_add(1);
        if next_tail == rd::<u32>(active_q + 0x14) {
            next_tail = 0;
        }
        wr::<u32>(active_q + 0x0c, next_tail);
        wr::<u32>(active_q + 0x10, rd::<u32>(active_q + 0x10).wrapping_add(1));

        let free_q = (mgr as isize + t * 24) as usize;
        let free_cnt = rd::<u32>(free_q + 0x28);
        if free_cnt == 0 {
            let dev_ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
            let fence_pair: [u64; 2] = [rd::<u64>(dev_ctx + 0x40), rd::<u64>(dev_ctx + 0x38)];
            let mut qtype_local = qtype;
            block = exe::call_va!(
                0x1_411d_b770,
                (
                    mgr,
                    0usize,
                    &mut qtype_local as *mut i32 as usize,
                    fence_pair.as_ptr() as usize
                ) -> usize
            );
            wr::<usize>(cur_slot_va, block);
        } else {
            let arr = rd::<usize>(free_q + 0x18);
            let cap = rd::<u32>(free_q + 0x2c) as u64;
            let head = rd::<u32>(free_q + 0x20) as u64;
            block = rd::<usize>(arr + ((head % cap) as usize) * 8);
            let mut next_head = (head as u32).wrapping_add(1);
            if next_head == rd::<u32>(free_q + 0x2c) {
                next_head = 0;
            }
            wr::<u32>(free_q + 0x20, next_head);
            wr::<u32>(free_q + 0x28, free_cnt - 1);
            wr::<usize>(cur_slot_va, block);
        }
    }
    wr::<usize>(out_slot, block);
    let slot_idx = rd::<u32>(block + 0x34);
    wr::<u32>(out_slot + 0x08, slot_idx);
    wr::<u32>(block + 0x34, slot_idx.wrapping_add(1));
    let cur_block = rd::<usize>(cur_slot_va);
    wr::<u32>(out_slot + 0x0c, rd::<u32>(cur_block + 0x30));
    unlock_recursive_srw(lock);
    out_slot
}

/// Begin an occlusion query (`*(this + 0x10) == 0`) via `ID3D12GraphicsCommandList::BeginQuery` (`vtbl + 0x1a0`).
///
/// Original: `0x142063200` (150 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_query_begin(this: usize) {
    if rd::<u32>(this + 0x10) != 0 {
        return;
    }
    let mut slot = [0u64; 2];
    let _ = d3d_query_heap_mgr_alloc_slot(this, slot.as_mut_ptr() as usize, 0);
    wr::<u64>(this, slot[0]);
    wr::<u64>(this + 0x08, slot[1]);
    let idx = rd::<u32>(this + 0x08);
    let mut cmd_guard = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_guard.as_mut_ptr() as usize) -> usize);
    let cmd_list = cmd_guard[1];
    let vtbl = rd::<usize>(cmd_list);
    let begin_query = rd::<usize>(vtbl + 0x1a0);
    let d3d_qtype = d3d_query_type_to_d3d12(rd::<u32>(this + 0x10));
    let block = rd::<usize>(this);
    let heap = rd::<usize>(block + 0x18);
    exe::call_va!(begin_query, (cmd_list, heap, d3d_qtype, idx) -> ());
    if cmd_guard[0] != 0 {
        unlock_recursive_srw(cmd_guard[0]);
    }
}

/// End a query (`this` at `rcx`, allocating a slot first if timestamp `*(this + 0x10) != 0`) via `ID3D12GraphicsCommandList::EndQuery` (`vtbl + 0x1a8`).
///
/// Original: `0x1420632b0` (150 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_query_end(this: usize) {
    let qtype = rd::<i32>(this + 0x10);
    if qtype != 0 {
        let mut slot = [0u64; 2];
        let _ = d3d_query_heap_mgr_alloc_slot(this, slot.as_mut_ptr() as usize, qtype);
        wr::<u64>(this, slot[0]);
        wr::<u64>(this + 0x08, slot[1]);
    }
    let idx = rd::<u32>(this + 0x08);
    let mut cmd_guard = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_guard.as_mut_ptr() as usize) -> usize);
    let cmd_list = cmd_guard[1];
    let vtbl = rd::<usize>(cmd_list);
    let end_query = rd::<usize>(vtbl + 0x1a8);
    let d3d_qtype = d3d_query_type_to_d3d12(rd::<u32>(this + 0x10));
    let block = rd::<usize>(this);
    let heap = rd::<usize>(block + 0x18);
    exe::call_va!(end_query, (cmd_list, heap, d3d_qtype, idx) -> ());
    if cmd_guard[0] != 0 {
        unlock_recursive_srw(cmd_guard[0]);
    }
}

/// Construct a `d3d::Query` (`this` at `rcx`, `qtype` at `edx`), querying `ID3D12CommandQueue::GetTimestampFrequency` into `*0x14397fc40` if 0.
///
/// Original: `0x142063360` (110 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_query_ctor(this: usize, qtype: u32) -> usize {
    wr::<u64>(this + 0x08, u64::MAX);
    wr::<u64>(this, 0);
    wr::<u32>(this + 0x10, qtype);
    let freq_va = exe::ptr::<u64>(0x1_4397_fc40) as usize;
    if rd::<u64>(freq_va) == 0 {
        let mut freq: u64 = 0;
        let queue_holder = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
        let queue = rd::<usize>(queue_holder + 0x10);
        let vtbl = rd::<usize>(queue);
        let get_freq = rd::<usize>(vtbl + 0x80);
        let hr = exe::call_va!(get_freq, (queue, &mut freq as *mut u64 as usize) -> u32);
        if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
            exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
        }
        wr::<u64>(freq_va, freq);
    }
    this
}

/// `d3d::QueryHeapManager::vfunc_1`: flush both occlusion (`0`) and timestamp (`1`) query pools.
///
/// Original: `0x142063410` (34 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_query_heap_mgr_vfunc_1(this: usize) {
    exe::call_va!(0x1_4206_3760, (this, 0u32) -> ());
    exe::call_va!(0x1_4206_3760, (this, 1u32) -> ());
}

// ============================================================================
// 4. Differential tests
// ============================================================================

pub unsafe fn diff_pack_version_u64(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u16, u32, u16) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(0x1420_5b6f);
    for case in 0..64 {
        let minor = r.next_u64() as u16;
        let patch = r.next_u64() as u32;
        let major = r.next_u64() as u16;
        let ra = o(minor, patch, major);
        let rb = n(minor, patch, major);
        crate::diff_eq!(ra, rb, "case {case}");
    }
    Ok(64)
}

pub unsafe fn diff_d3d_fsr2_dxgi_to_ffx_format(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for fmt in (0..=0x70u32).chain([0x80, 0xff, u32::MAX]) {
        let ra = o(fmt);
        let rb = n(fmt);
        crate::diff_eq!(ra, rb, "dxgi_format={fmt:#x}");
        cases += 1;
    }
    Ok(cases)
}

unsafe extern "win64" fn fake_d3d12_get_desc(this: usize, out_desc: usize) -> usize {
    for i in 0..7usize {
        wr::<u64>(out_desc + i * 8, rd::<u64>(this + 0x10 + i * 8));
    }
    out_desc
}

pub unsafe fn diff_d3d_fsr2_get_ffx_resource(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut vtbl = [0usize; 16];
    vtbl[10] = fake_d3d12_get_desc as *const () as usize; // +0x50
    let mut cases = 0;

    // Case 0: null resource pointer
    {
        let mut out_a = [0xaa_u8; 48];
        let mut out_b = [0xaa_u8; 48];
        let ra = o(out_a.as_mut_ptr() as usize, 0, 0xc) - out_a.as_ptr() as usize;
        let rb = n(out_b.as_mut_ptr() as usize, 0, 0xc) - out_b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "null ret");
        crate::diff_eq!(out_a, out_b, "null bytes");
        cases += 1;
    }

    for &(dim, fmt, depth_arr, mips, flags, state) in &[
        (0u32, 0u32, 1u16, 1u16, 0u32, 0xc_u32),
        (1, 0, 1, 1, 0, 0x2),
        (2, 0x0a, 1, 4, 0, 0xc),
        (3, 0x1c, 1, 1, 4, 0x2),
        (3, 0x28, 6, 1, 0, 0xc),
        (3, 0x2d, 1, 8, 4, 0xc),
        (4, 0x02, 16, 3, 4, 0xc),
    ] {
        let mut fake_res = [0u64; 10];
        let res_ptr = fake_res.as_mut_ptr() as usize;
        wr::<usize>(res_ptr, vtbl.as_ptr() as usize);
        let desc_ptr = res_ptr + 0x10;
        wr::<u32>(desc_ptr, dim);
        wr::<u64>(desc_ptr + 0x10, 1920);
        wr::<u32>(desc_ptr + 0x18, 1080);
        wr::<u16>(desc_ptr + 0x1c, depth_arr);
        wr::<u16>(desc_ptr + 0x1e, mips);
        wr::<u32>(desc_ptr + 0x20, fmt);
        wr::<u32>(desc_ptr + 0x30, flags);

        let mut out_a = [0x55_u8; 48];
        let mut out_b = [0x55_u8; 48];
        let ra = o(out_a.as_mut_ptr() as usize, res_ptr, state) - out_a.as_ptr() as usize;
        let rb = n(out_b.as_mut_ptr() as usize, res_ptr, state) - out_b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret dim={dim} fmt={fmt:#x}");
        crate::diff_eq!(out_a, out_b, "out dim={dim} fmt={fmt:#x}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_fsr2_get_fg_flags(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let p0 = exe::ptr::<u8>(0x1_4385_7ce8) as usize;
    let p1 = exe::ptr::<u8>(0x1_4385_7de0) as usize;
    let p2 = exe::ptr::<u8>(0x1_4385_7d00) as usize;
    let p3 = exe::ptr::<u8>(0x1_4385_7e40) as usize;
    let saved = (rd::<u8>(p0), rd::<u8>(p1), rd::<u8>(p2), rd::<u8>(p3));
    for mask in 0..16u8 {
        wr::<u8>(p0, if (mask & 1) != 0 { 1 } else { 0 });
        wr::<u8>(p1, if (mask & 2) != 0 { 5 } else { 0 });
        wr::<u8>(p2, if (mask & 4) != 0 { 255 } else { 0 });
        wr::<u8>(p3, if (mask & 8) != 0 { 1 } else { 0 });
        let ra = o();
        let rb = n();
        crate::diff_eq!(ra, rb, "mask={mask:#x}");
    }
    wr::<u8>(p0, saved.0);
    wr::<u8>(p1, saved.1);
    wr::<u8>(p2, saved.2);
    wr::<u8>(p3, saved.3);
    Ok(16)
}

pub unsafe fn diff_d3d_fsr2_msg_callback_2(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let wstr: [u16; 6] = [b'F' as u16, b'S' as u16, b'R' as u16, b'2' as u16, b'!' as u16, 0];
    for t in [0u32, 1, 2] {
        o(t, wstr.as_ptr() as usize);
        n(t, wstr.as_ptr() as usize);
    }
    Ok(3)
}

pub unsafe fn diff_d3d_fsr2_msg_callback_3(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let wstr: [u16; 6] = [b'F' as u16, b'S' as u16, b'R' as u16, b'3' as u16, b'!' as u16, 0];
    for t in [0u64, 1, 2] {
        o(t, 0, wstr.as_ptr() as usize);
        n(t, 0, wstr.as_ptr() as usize);
    }
    Ok(3)
}

pub unsafe fn diff_d3d_fsr2_get_jitter_offset(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, u32, u32, usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for (rw, dw, idx) in [(1280u32, 1920u32, 0u32), (960, 1920, 3), (1414, 1414, 7)] {
        let (mut xa, mut ya) = (-1.0f32, -1.0f32);
        let (mut xb, mut yb) = (-1.0f32, -1.0f32);
        o(rw, dw, idx, &mut xa as *mut f32 as usize, &mut ya as *mut f32 as usize);
        n(rw, dw, idx, &mut xb as *mut f32 as usize, &mut yb as *mut f32 as usize);
        crate::diff_eq!(xa.to_bits(), xb.to_bits(), "x rw={rw} dw={dw} idx={idx}");
        crate::diff_eq!(ya.to_bits(), yb.to_bits(), "y rw={rw} dw={dw} idx={idx}");
    }
    Ok(3)
}

pub unsafe fn diff_fsr2_glue_get_jitter_offset(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, u32, u32, usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let frame_va = exe::ptr::<u32>(0x1_43a0_0180) as usize;
    let saved_frame = rd::<u32>(frame_va);
    for (rw, dw, frame) in [(1280u32, 1920u32, 0u32), (960, 1920, 5), (1280, 1414, 11)] {
        wr::<u32>(frame_va, frame);
        let (mut xa, mut ya) = (-1.0f32, -1.0f32);
        let (mut xb, mut yb) = (-1.0f32, -1.0f32);
        o(rw, dw, 0, &mut xa as *mut f32 as usize, &mut ya as *mut f32 as usize);
        n(rw, dw, 0, &mut xb as *mut f32 as usize, &mut yb as *mut f32 as usize);
        crate::diff_eq!(xa.to_bits(), xb.to_bits(), "x rw={rw} dw={dw} frame={frame}");
        crate::diff_eq!(ya.to_bits(), yb.to_bits(), "y rw={rw} dw={dw} frame={frame}");
    }
    wr::<u32>(frame_va, saved_frame);
    Ok(3)
}

pub unsafe fn diff_ffx_fsr2_get_upscale_ratio_from_quality_mode(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> f32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for mode in 0..8u32 {
        let ra = o(mode).to_bits();
        let rb = n(mode).to_bits();
        crate::diff_eq!(ra, rb, "mode={mode}");
    }
    Ok(8)
}

pub unsafe fn diff_fsr2_glue_get_render_resolution(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, u32, u32, usize, usize) -> bool;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for mode in 0..=6u32 {
        let (mut wa, mut ha) = (0xdead_u32, 0xbeef_u32);
        let (mut wb, mut hb) = (0xdead_u32, 0xbeef_u32);
        let ra = o(
            1920,
            1080,
            mode,
            &mut wa as *mut u32 as usize,
            &mut ha as *mut u32 as usize,
        );
        let rb = n(
            1920,
            1080,
            mode,
            &mut wb as *mut u32 as usize,
            &mut hb as *mut u32 as usize,
        );
        crate::diff_eq!(ra, rb, "ret mode={mode}");
        crate::diff_eq!(wa, wb, "w mode={mode}");
        crate::diff_eq!(ha, hb, "h mode={mode}");
        cases += 1;
    }
    // Null output pointer checks for mode 1..=4
    for (ow, oh) in [(0usize, 1usize), (1usize, 0usize)] {
        let mut dummy_a = 0u32;
        let mut dummy_b = 0u32;
        let pa = if ow == 0 { 0 } else { &mut dummy_a as *mut u32 as usize };
        let qa = if oh == 0 { 0 } else { &mut dummy_a as *mut u32 as usize };
        let pb = if ow == 0 { 0 } else { &mut dummy_b as *mut u32 as usize };
        let qb = if oh == 0 { 0 } else { &mut dummy_b as *mut u32 as usize };
        crate::diff_eq!(o(1920, 1080, 1, pa, qa), n(1920, 1080, 1, pb, qb), "null ptr");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_fsr2_glue_lambda_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(0x1423_8cf6);
    for case in 0..16 {
        let b0 = r.next_u64() as u8;
        let b1 = r.next_u64() as u8;
        let q0 = r.next_u64();
        let q1 = r.next_u64();
        let mut a = [0xaa_u8; 24];
        let mut b = [0xaa_u8; 24];
        let ra = o(
            a.as_mut_ptr() as usize,
            &b0 as *const u8 as usize,
            &b1 as *const u8 as usize,
            &q0 as *const u64 as usize,
            &q1 as *const u64 as usize,
        ) - a.as_ptr() as usize;
        let rb = n(
            b.as_mut_ptr() as usize,
            &b0 as *const u8 as usize,
            &b1 as *const u8 as usize,
            &q0 as *const u64 as usize,
            &q1 as *const u64 as usize,
        ) - b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret case {case}");
        crate::diff_eq!(a, b, "mem case {case}");
    }
    Ok(16)
}

pub unsafe fn diff_d3d_show_error_message_box(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let flag_ptr = exe::ptr::<u8>(0x1_4384_4045) as usize;
    let saved = rd::<u8>(flag_ptr);
    wr::<u8>(flag_ptr, 0);
    o(0x1234, 0x5678);
    n(0x1234, 0x5678);
    wr::<u8>(flag_ptr, saved);
    Ok(1)
}

pub unsafe fn diff_frame_rate_history_get_avg_fps(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> f32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let samples: [f64; 8] = [
        0.016666666666666666,
        0.016,
        0.017,
        0.0165,
        0.0168,
        0.0162,
        0.0159,
        0.0171,
    ];
    for count in [0u64, 1, 4, 8] {
        let mut hist = [0u64; 4];
        hist[1] = count;
        hist[2] = samples.as_ptr() as usize as u64;
        hist[3] = 16;
        let ra = o(hist.as_ptr() as usize).to_bits();
        let rb = n(hist.as_ptr() as usize).to_bits();
        crate::diff_eq!(ra, rb, "count={count}");
    }
    // Zero-sum edge case
    let zeros = [0.0f64; 4];
    let hist_z = [0u64, 4, zeros.as_ptr() as usize as u64, 16];
    crate::diff_eq!(
        o(hist_z.as_ptr() as usize).to_bits(),
        n(hist_z.as_ptr() as usize).to_bits(),
        "zero sum"
    );
    Ok(5)
}

pub unsafe fn diff_frame_rate_history_push(orig: usize, new: usize) -> Result<u32, String> {
    #[cfg(not(target_arch = "aarch64"))]
    type F = unsafe extern "win64" fn(usize, f64);
    #[cfg(target_arch = "aarch64")]
    type F = unsafe extern "win64" fn(usize, f64, f64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut buf_a = [0.0f64; 4];
    let mut buf_b = [0.0f64; 4];
    let mut hist_a = [0u64, 0, buf_a.as_mut_ptr() as usize as u64, 4];
    let mut hist_b = [0u64, 0, buf_b.as_mut_ptr() as usize as u64, 4];

    for step in 0..7 {
        let val = 0.010 + (step as f64) * 0.002;
        #[cfg(not(target_arch = "aarch64"))]
        {
            o(hist_a.as_mut_ptr() as usize, val);
            n(hist_b.as_mut_ptr() as usize, val);
        }
        #[cfg(target_arch = "aarch64")]
        {
            o(hist_a.as_mut_ptr() as usize, 0.0, val);
            n(hist_b.as_mut_ptr() as usize, 0.0, val);
        }
        crate::diff_eq!(hist_a[0], hist_b[0], "write_idx step={step}");
        crate::diff_eq!(hist_a[1], hist_b[1], "len step={step}");
        for k in 0..4 {
            crate::diff_eq!(
                buf_a[k].to_bits(),
                buf_b[k].to_bits(),
                "buf[{k}] step={step}"
            );
        }
    }
    Ok(7)
}

pub unsafe fn diff_d3d_buffer_view_ctor_from_alloc(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(0x1420_5d97);
    for case in 0..8 {
        let mut alloc = [0u64; 6];
        for w in &mut alloc {
            *w = r.next_u64();
        }
        if case % 2 == 0 {
            alloc[3] = 0; // test null cpu_ptr at +0x18
            alloc[4] = 0; // test null cpu_ptr at +0x20 (for sbv)
        }
        let mut out_a = [0xaa_u8; 96];
        let mut out_b = [0xaa_u8; 96];
        let ra = o(out_a.as_mut_ptr() as usize, alloc.as_ptr() as usize) - out_a.as_ptr() as usize;
        let rb = n(out_b.as_mut_ptr() as usize, alloc.as_ptr() as usize) - out_b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret case={case}");
        crate::diff_eq!(out_a, out_b, "mem case={case}");
    }
    Ok(8)
}

pub unsafe fn diff_d3d_get_cbv_page_size(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    crate::diff_eq!(o(), n(), "page_size");
    Ok(1)
}

pub unsafe fn diff_d3d_vertex_format_to_dxgi(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for fmt in 0..=32u8 {
        crate::diff_eq!(o(fmt), n(fmt), "fmt={fmt}");
    }
    Ok(33)
}

pub unsafe fn diff_d3d_vertex_input_slot_class(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for cls in [0u8, 1, 2, 0xff] {
        crate::diff_eq!(o(cls), n(cls), "cls={cls}");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_build_input_element_descs(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    // Case 0: empty layout
    {
        let layout = [0u8; 68];
        let mut out_a = [0xaa_u8; 64];
        let mut out_b = [0xaa_u8; 64];
        o(layout.as_ptr() as usize, out_a.as_mut_ptr() as usize);
        n(layout.as_ptr() as usize, out_b.as_mut_ptr() as usize);
        crate::diff_eq!(out_a, out_b, "empty layout");
    }
    // Case 1: 5 elements across multiple slots, semantics, and step rates
    {
        let mut layout = [0u8; 68];
        layout[0..4].copy_from_slice(&[0, 2, 0, 0]); // slot 0, float3, POSITION0, vertex
        layout[4..8].copy_from_slice(&[0, 1, 2, 0]); // slot 0, float2, TEXCOORD0, vertex
        layout[8..12].copy_from_slice(&[0, 4, 2, 0]); // slot 0, rgba8, TEXCOORD1, vertex
        layout[12..16].copy_from_slice(&[1, 3, 7, 1]); // slot 1, float4, RenderInstance0, instance
        layout[16..20].copy_from_slice(&[1, 16, 9, 2]); // slot 1, fmt 16, unknown sem 9, step 2
        layout[0x40] = 5;
        let mut out_a = [0x55_u8; 5 * 0x20];
        let mut out_b = [0x55_u8; 5 * 0x20];
        o(layout.as_ptr() as usize, out_a.as_mut_ptr() as usize);
        n(layout.as_ptr() as usize, out_b.as_mut_ptr() as usize);
        crate::diff_eq!(out_a, out_b, "5-elem layout");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_shader_ctor_2arg(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(0x1420_6082);
    for case in 0..4 {
        let mut src = [0u64; 16];
        for w in &mut src {
            *w = r.next_u64();
        }
        let mut a = [0xaa_u8; 96];
        let mut b = [0xaa_u8; 96];
        let ra = o(a.as_mut_ptr() as usize, src.as_ptr() as usize) - a.as_ptr() as usize;
        let rb = n(b.as_mut_ptr() as usize, src.as_ptr() as usize) - b.as_ptr() as usize;
        // ComputeShader stores a self-relative pointer `this + 0x38` at `+0x28`; normalize if set
        let ptr_a = rd::<usize>(a.as_ptr() as usize + 0x28);
        let ptr_b = rd::<usize>(b.as_ptr() as usize + 0x28);
        if ptr_a == a.as_ptr() as usize + 0x38 && ptr_b == b.as_ptr() as usize + 0x38 {
            wr::<usize>(a.as_mut_ptr() as usize + 0x28, 0x38);
            wr::<usize>(b.as_mut_ptr() as usize + 0x28, 0x38);
        }
        crate::diff_eq!(ra, rb, "ret case={case}");
        crate::diff_eq!(a, b, "mem case={case}");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_vertex_shader_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = crate::difftest::Rng::new(0x1420_6087);
    for case in 0..4 {
        let mut src = [0u64; 16];
        for w in &mut src {
            *w = r.next_u64();
        }
        let layout_id = r.next_u64() as u32;
        let mut a = [0xaa_u8; 64];
        let mut b = [0xaa_u8; 64];
        let ra = o(a.as_mut_ptr() as usize, src.as_ptr() as usize, layout_id) - a.as_ptr() as usize;
        let rb = n(b.as_mut_ptr() as usize, src.as_ptr() as usize, layout_id) - b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret case={case}");
        crate::diff_eq!(a, b, "mem case={case}");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_copy_shader_bytecode_view(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    // Null shader
    {
        let mut a = [0xaa_u8; 16];
        let mut b = [0xaa_u8; 16];
        o(a.as_mut_ptr() as usize, 0, 0);
        n(b.as_mut_ptr() as usize, 0, 0);
        crate::diff_eq!(a[..12], b[..12], "null shader");
    }
    // Non-null shader
    {
        let shader: [u64; 4] = [0x1234_5678_9abc_def0, 0, 0x0000_0001_0000_4321, 0];
        let mut a = [0xaa_u8; 16];
        let mut b = [0xaa_u8; 16];
        o(a.as_mut_ptr() as usize, 0, shader.as_ptr() as usize);
        n(b.as_mut_ptr() as usize, 0, shader.as_ptr() as usize);
        crate::diff_eq!(a[..12], b[..12], "valid shader");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_copy_shader_bytecode_view_ret(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for has_shader in [false, true] {
        let shader: [u64; 4] = [0xfeed_face_cafe_babe, 0, 0x0000_0002_0000_1234, 0];
        let s_ptr = if has_shader { shader.as_ptr() as usize } else { 0 };
        let mut a = [0xaa_u8; 16];
        let mut b = [0xaa_u8; 16];
        let ra = o(a.as_mut_ptr() as usize, 0, s_ptr) - a.as_ptr() as usize;
        let rb = n(b.as_mut_ptr() as usize, 0, s_ptr) - b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret has_shader={has_shader}");
        crate::diff_eq!(a[..12], b[..12], "mem has_shader={has_shader}");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_zero_shader_bytecode_view(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut a = [0xaa_u8; 16];
    let mut b = [0xaa_u8; 16];
    let ra = o(a.as_mut_ptr() as usize) - a.as_ptr() as usize;
    let rb = n(b.as_mut_ptr() as usize) - b.as_ptr() as usize;
    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(a[..12], b[..12], "mem");
    Ok(1)
}

pub unsafe fn diff_d3d_resolve_srv_descriptor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mgr_va = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let saved_mgr = rd::<usize>(mgr_va);
    let mut fake_mgr = [0u64; 24];
    fake_mgr[19] = 0xdef0_0098; // +0x98
    fake_mgr[20] = 0xdef0_00a0; // +0xa0
    fake_mgr[21] = 0xdef0_00a8; // +0xa8
    wr::<usize>(mgr_va, fake_mgr.as_ptr() as usize);

    let mut fake_res = [0u64; 10];
    fake_res[3] = 0x1111_0018; // +0x18
    fake_res[4] = 0x2222_0020; // +0x20
    fake_res[5] = 0x3333_0028; // +0x28
    fake_res[6] = 0x4444_0030; // +0x30
    fake_res[7] = 0x5555_0038; // +0x38
    let zero_res = [0u64; 10];

    let mut cases = 0;
    for dim in 0..=9u32 {
        for r_ptr in [0usize, zero_res.as_ptr() as usize, fake_res.as_ptr() as usize] {
            let (mut oa, mut ob) = (0xdead_beef_u64, 0xdead_beef_u64);
            let _ = o(&mut oa as *mut u64 as usize, dim, r_ptr);
            let _ = n(&mut ob as *mut u64 as usize, dim, r_ptr);
            crate::diff_eq!(oa, ob, "dim={dim} r_ptr={r_ptr:#x}");
            cases += 1;
        }
    }
    wr::<usize>(mgr_va, saved_mgr);
    Ok(cases)
}

pub unsafe fn diff_d3d_resolve_uav_descriptor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mgr_va = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let saved_mgr = rd::<usize>(mgr_va);
    let mut fake_mgr = [0u64; 24];
    fake_mgr[17] = 0xdef0_0088; // +0x88
    fake_mgr[18] = 0xdef0_0090; // +0x90
    wr::<usize>(mgr_va, fake_mgr.as_ptr() as usize);

    let mut fake_res = [0u64; 20];
    fake_res[15] = 0x7777_0078; // +0x78
    fake_res[18] = 0x9999_0090; // +0x90

    let mut cases = 0;
    for dim in [0u32, 1, 4, 5, 6] {
        for r_ptr in [0usize, fake_res.as_ptr() as usize] {
            let mut oa = [0xaa_u8; 16];
            let mut ob = [0xaa_u8; 16];
            let _ = o(oa.as_mut_ptr() as usize, dim, r_ptr);
            let _ = n(ob.as_mut_ptr() as usize, dim, r_ptr);
            crate::diff_eq!(oa, ob, "dim={dim} r_ptr={r_ptr:#x}");
            cases += 1;
        }
    }
    wr::<usize>(mgr_va, saved_mgr);
    Ok(cases)
}

pub unsafe fn diff_d3d_deref_or_zero(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let val: u64 = 0x1234_5678_9abc_def0;
    for src in [0usize, &val as *const u64 as usize] {
        let (mut a, mut b) = (0xdead_u64, 0xdead_u64);
        let _ = o(&mut a as *mut u64 as usize, src);
        let _ = n(&mut b as *mut u64 as usize, src);
        crate::diff_eq!(a, b, "src={src:#x}");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_get_global_34b68_deref(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let g_va = exe::ptr::<usize>(0x1_43a3_4b68) as usize;
    let saved = rd::<usize>(g_va);
    let val: u64 = 0xfedc_ba98_7654_3210;
    for ptr in [0usize, &val as *const u64 as usize] {
        wr::<usize>(g_va, ptr);
        let (mut a, mut b) = (0xdead_u64, 0xdead_u64);
        let ra = o(&mut a as *mut u64 as usize) - (&a as *const u64 as usize);
        let rb = n(&mut b as *mut u64 as usize) - (&b as *const u64 as usize);
        crate::diff_eq!(ra, rb, "ret ptr={ptr:#x}");
        crate::diff_eq!(a, b, "val ptr={ptr:#x}");
    }
    wr::<usize>(g_va, saved);
    Ok(2)
}

pub unsafe fn diff_d3d_copy_entry_48_at_10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut entries = [0u64; 18]; // 3 * 48B
    for (i, w) in entries.iter_mut().enumerate() {
        *w = 0x1000 + i as u64;
    }
    let holder: [usize; 4] = [0, 0, entries.as_ptr() as usize, 0];
    for idx in 0..3i32 {
        let mut a = [0xaa_u8; 48];
        let mut b = [0xaa_u8; 48];
        let ra = o(holder.as_ptr() as usize, a.as_mut_ptr() as usize, idx) - a.as_ptr() as usize;
        let rb = n(holder.as_ptr() as usize, b.as_mut_ptr() as usize, idx) - b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret idx={idx}");
        crate::diff_eq!(a, b, "mem idx={idx}");
    }
    Ok(3)
}

pub unsafe fn diff_d3d_get_qword_at_20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let arr: [u64; 4] = [0x11, 0x22, 0x33, 0x44];
    let holder: [usize; 5] = [0, 0, 0, 0, arr.as_ptr() as usize];
    for idx in 0..4i32 {
        crate::diff_eq!(o(holder.as_ptr() as usize, idx), n(holder.as_ptr() as usize, idx), "idx={idx}");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_index_stride_to_dxgi_format(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for s in 0..=16u8 {
        crate::diff_eq!(o(s), n(s), "stride={s}");
    }
    Ok(17)
}

pub unsafe fn diff_d3d_convert_vertex_and_index_views_32(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for b0 in [0u8, 2, 4, 10] {
        let mut src = [0u32; 8];
        for i in 0..8 {
            src[i] = 0x100 + i as u32;
        }
        wr::<u8>(src.as_mut_ptr() as usize, b0);
        let mut a = [0xaa_u8; 32];
        let mut b = [0xaa_u8; 32];
        let ra = o(a.as_mut_ptr() as usize, src.as_ptr() as usize) - a.as_ptr() as usize;
        let rb = n(b.as_mut_ptr() as usize, src.as_ptr() as usize) - b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret b0={b0}");
        crate::diff_eq!(a, b, "mem b0={b0}");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_fill_raytracing_geom_desc(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for mode in 0..=5u32 {
        for b0 in [0u8, 2, 10] {
            let mut src = [0u32; 8];
            for i in 0..8 {
                src[i] = 0x200 + i as u32;
            }
            wr::<u8>(src.as_mut_ptr() as usize, b0);
            let mut a = [0u32; 12];
            let mut b = [0u32; 12];
            a[2] = mode;
            b[2] = mode;
            o(a.as_mut_ptr() as usize, src.as_ptr() as usize);
            n(b.as_mut_ptr() as usize, src.as_ptr() as usize);
            crate::diff_eq!(a, b, "mode={mode} b0={b0}");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_pack_two_dwords_u64(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let pair: [u32; 2] = [0x89ab_cdef, 0x0123_4567];
    crate::diff_eq!(o(pair.as_ptr() as usize), n(pair.as_ptr() as usize), "pack");
    Ok(1)
}

pub unsafe fn diff_d3d_get_indexed_ptr_at_18(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let arr: [usize; 4] = [0x1111, 0x2222, 0x3333, 0x4444];
    let this_obj: [usize; 4] = [0, 0, 0, arr.as_ptr() as usize];
    for idx in 0..4u32 {
        let rdx_obj: [u32; 4] = [0, 0, 0, idx];
        crate::diff_eq!(
            o(this_obj.as_ptr() as usize, rdx_obj.as_ptr() as usize),
            n(this_obj.as_ptr() as usize, rdx_obj.as_ptr() as usize),
            "idx={idx}"
        );
    }
    Ok(4)
}

pub unsafe fn diff_d3d_query_is_stale(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> bool;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for (q_gen, b_gen) in [(0i32, 0i32), (0, 1), (2, 1), (-1, 0)] {
        let mut block = [0u32; 16];
        block[12] = b_gen as u32; // +0x30
        let mut query = [0u64; 2];
        query[0] = block.as_ptr() as usize as u64;
        wr::<i32>((query.as_mut_ptr() as usize) + 0x0c, q_gen);
        crate::diff_eq!(
            o(query.as_ptr() as usize),
            n(query.as_ptr() as usize),
            "q_gen={q_gen} b_gen={b_gen}"
        );
    }
    Ok(4)
}

pub unsafe fn diff_d3d_query_type_to_d3d12(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    for qt in [0u32, 1, 2, 0xff] {
        crate::diff_eq!(o(qt), n(qt), "qt={qt}");
    }
    Ok(4)
}
