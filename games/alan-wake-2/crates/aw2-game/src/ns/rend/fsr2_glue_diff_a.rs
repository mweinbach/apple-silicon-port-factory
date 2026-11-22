//! Differential tests (`group14a`) for the first 30 `Status::Draft` functions in `fsr2_glue.rs`.
#![allow(static_mut_refs)]

use crate::difftest::{IatMock, Mock, Snapshot};
use crate::exe::{self, rd, wr};

const IAT_FFX_QUERY: u64 = 0x1_4264_6488;
const IAT_FFX_DISPATCH: u64 = 0x1_4264_6490;
const IAT_FFX_CREATE_CONTEXT: u64 = 0x1_4264_6498;
const IAT_FFX_CONFIGURE: u64 = 0x1_4264_64a8;
const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;
const IAT_ACQUIRE_SRW_EXCLUSIVE: u64 = 0x1_4264_55a8;
const IAT_GET_CURRENT_THREAD_ID: u64 = 0x1_4264_5790;

unsafe extern "win64" fn fake_srw_noop(_lock: usize) {}
unsafe extern "win64" fn fake_get_tid() -> u32 {
    0x1234
}

unsafe extern "win64" fn fake_d3d12_get_desc(this: usize, out_desc: usize) -> usize {
    for i in 0..7usize {
        wr::<u64>(out_desc + i * 8, rd::<u64>(this + 0x10 + i * 8));
    }
    out_desc
}

static mut TLS_FALLBACK: [u8; 0x47000] = [0u8; 0x47000];

struct TlsGuard {
    tls_slot: usize,
    saved_tls0: usize,
    tls0: usize,
    saved_flag70: u8,
    saved_ring6360: usize,
}

impl TlsGuard {
    unsafe fn new(ring_holder: usize) -> Self {
        let tls_slot = exe::tls_pointer();
        let saved_tls0 = rd::<usize>(tls_slot);
        let tls0 = if saved_tls0 != 0 {
            saved_tls0
        } else {
            let p = TLS_FALLBACK.as_mut_ptr() as usize;
            wr::<usize>(tls_slot, p);
            p
        };
        let saved_flag70 = rd::<u8>(tls0 + 0x70);
        let saved_ring6360 = rd::<usize>(tls0 + 0x6360);
        wr::<u8>(tls0 + 0x70, 1);
        wr::<usize>(tls0 + 0x6360, ring_holder);
        Self {
            tls_slot,
            saved_tls0,
            tls0,
            saved_flag70,
            saved_ring6360,
        }
    }
}

impl Drop for TlsGuard {
    fn drop(&mut self) {
        unsafe {
            wr::<u8>(self.tls0 + 0x70, self.saved_flag70);
            wr::<usize>(self.tls0 + 0x6360, self.saved_ring6360);
            if self.saved_tls0 == 0 {
                wr::<usize>(self.tls_slot, 0);
            }
        }
    }
}

static mut GPU_TS_CALLS: u32 = 0;

unsafe extern "win64" fn fake_142072ec0(_ctx: usize) {
    GPU_TS_CALLS = GPU_TS_CALLS.wrapping_add(1);
}

unsafe extern "win64" fn fake_1420734e0(out16: usize, flag: u32) -> usize {
    wr::<u64>(out16, 0x1122_3344_5566_7788 ^ (flag as u64));
    wr::<u64>(out16 + 8, 0x99aa_bbcc_ddee_ff00 ^ ((flag as u64) << 16));
    out16
}

static mut SAVED_UI_DESC: [u8; 64] = [0u8; 64];
static mut SAVED_FG_DISP: [u8; 0x138] = [0u8; 0x138];
static mut SAVED_PREP_DISP: [u8; 0xa8] = [0u8; 0xa8];
static mut SAVED_PREP_CAM: [u8; 0x40] = [0u8; 0x40];
static mut SAVED_UP_DISP: [u8; 0x1b0] = [0u8; 0x1b0];

unsafe extern "win64" fn fake_ffx_configure(_ctx: usize, desc: usize) -> u32 {
    let ui_ptr = rd::<usize>(desc + 0x08);
    if ui_ptr != 0 {
        std::ptr::copy_nonoverlapping(ui_ptr as *const u8, SAVED_UI_DESC.as_mut_ptr(), 64);
        wr::<usize>(desc + 0x08, 0x20008);
    } else {
        SAVED_UI_DESC = [0u8; 64];
    }
    0
}

unsafe extern "win64" fn fake_ffx_query_fg(_ctx: usize, q: usize) -> u32 {
    let ty = rd::<u64>(q);
    if ty == 0x30003 || ty == 0x30004 {
        let dst = rd::<usize>(q + 0x10);
        wr::<u64>(dst, 0x7777_0000_0000_0000 | ty);
    } else if ty == 0x10008 || ty == 0x20007 {
        let dst = rd::<usize>(q + 0x10);
        wr::<u64>(dst, 0x1000);
        wr::<u64>(dst + 8, 0x2000);
    }
    0
}

unsafe extern "win64" fn fake_ffx_dispatch(_ctx: usize, disp: usize) -> u32 {
    let ty = rd::<u64>(disp);
    if ty == 0x20003 {
        std::ptr::copy_nonoverlapping(disp as *const u8, SAVED_FG_DISP.as_mut_ptr(), 0x138);
    } else if ty == 0x20004 {
        let cam = rd::<usize>(disp + 0x08);
        if cam != 0 {
            std::ptr::copy_nonoverlapping(cam as *const u8, SAVED_PREP_CAM.as_mut_ptr(), 0x40);
        }
        std::ptr::copy_nonoverlapping(
            (disp + 0x10) as *const u8,
            SAVED_PREP_DISP.as_mut_ptr(),
            0xa8,
        );
    } else if ty == 0x10001 {
        std::ptr::copy_nonoverlapping(disp as *const u8, SAVED_UP_DISP.as_mut_ptr(), 0x1b0);
    }
    0
}

/// 1. `0x14205b7b0` — `d3d_fsr2_configure_and_dispatch_fg`
pub unsafe fn diff_d3d_fsr2_configure_and_dispatch_fg(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, usize, usize, u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _tls = TlsGuard::new(0);
    let _m_ts1 = Mock::new(0x1_4207_2ec0, fake_142072ec0 as *const ())?;
    let _m_ts2 = Mock::new(0x1_4207_34e0, fake_1420734e0 as *const ())?;
    let _im_cfg = IatMock::new(IAT_FFX_CONFIGURE, fake_ffx_configure as *const ())?;
    let _im_q = IatMock::new(IAT_FFX_QUERY, fake_ffx_query_fg as *const ())?;
    let _im_d = IatMock::new(IAT_FFX_DISPATCH, fake_ffx_dispatch as *const ())?;

    let snap_cfg = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ce0) as *mut u8, 0x180);
    let snap_sz = Snapshot::take(exe::ptr::<u8>(0x1_43a3_2fb8) as *mut u8, 8);
    let snap_ov = Snapshot::take(exe::ptr::<u8>(0x1_43a2_b11c) as *mut u8, 4);
    let snap_ts = Snapshot::take(exe::ptr::<u8>(0x1_43c4_8fc0) as *mut u8, 16);

    let mut vtbl = [0usize; 16];
    vtbl[10] = fake_d3d12_get_desc as *const () as usize;
    let mut d3d_res = [0u64; 10];
    wr::<usize>(d3d_res.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<u32>(d3d_res.as_mut_ptr() as usize + 0x10, 3);
    wr::<u64>(d3d_res.as_mut_ptr() as usize + 0x20, 1920);
    wr::<u32>(d3d_res.as_mut_ptr() as usize + 0x28, 1080);
    wr::<u16>(d3d_res.as_mut_ptr() as usize + 0x2c, 1);
    wr::<u16>(d3d_res.as_mut_ptr() as usize + 0x2e, 1);
    wr::<u32>(d3d_res.as_mut_ptr() as usize + 0x30, 0x1c);

    let mut tex_obj = [0u64; 16];
    tex_obj[14] = d3d_res.as_mut_ptr() as usize as u64; // +0x70

    let sc_inner: [usize; 3] = [0, 0xdead_0001, 0xdead_0002];
    let mut sc = [0u8; 0x70];
    let sc_ptr = sc.as_mut_ptr() as usize;
    wr::<usize>(sc_ptr, sc_inner.as_ptr() as usize);
    wr::<usize>(sc_ptr + 0x08, tex_obj.as_ptr() as usize);
    wr::<i32>(sc_ptr + 0x38, 0);
    wr::<u8>(sc_ptr + 0x4c, 1);
    wr::<f32>(sc_ptr + 0x50, 0.01);

    let mut cases = 0u32;
    for (fg_en, has_ui, hdr66, hdr67) in [(0u8, false, 0u8, 0u8), (1, false, 0, 1), (1, true, 1, 0)] {
        snap_cfg.restore();
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7dc8) as usize, fg_en);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7ce8) as usize, 1);
        wr::<u32>(exe::ptr::<u32>(0x1_43a3_2fb8) as usize, 1920);
        wr::<u32>(exe::ptr::<u32>(0x1_43a3_2fbc) as usize, 1080);
        wr::<u8>(sc_ptr + 0x66, hdr66);
        wr::<u8>(sc_ptr + 0x67, hdr67);
        let ui_arg = if has_ui { tex_obj.as_ptr() as usize } else { 0 };

        let pre_cfg = Snapshot::take(exe::ptr::<u8>(0x1_4385_7d10) as *mut u8, 0x90);
        SAVED_FG_DISP = [0u8; 0x138];
        SAVED_UI_DESC = [0u8; 64];
        o(sc_ptr, 42, tex_obj.as_ptr() as usize, ui_arg, 1);
        let cfg_o = pre_cfg.current();
        let disp_o = SAVED_FG_DISP;
        let ui_o = SAVED_UI_DESC;
        let ts_o = snap_ts.current();

        pre_cfg.restore();
        SAVED_FG_DISP = [0u8; 0x138];
        SAVED_UI_DESC = [0u8; 64];
        n(sc_ptr, 42, tex_obj.as_ptr() as usize, ui_arg, 1);
        let cfg_n = pre_cfg.current();
        let disp_n = SAVED_FG_DISP;
        let ui_n = SAVED_UI_DESC;
        let ts_n = snap_ts.current();

        crate::diff_eq!(cfg_o, cfg_n, "cfg fg_en={fg_en} ui={has_ui}");
        crate::diff_eq!(disp_o, disp_n, "disp fg_en={fg_en} ui={has_ui}");
        crate::diff_eq!(ui_o, ui_n, "ui fg_en={fg_en} ui={has_ui}");
        crate::diff_eq!(ts_o, ts_n, "ts fg_en={fg_en} ui={has_ui}");
        cases += 1;
    }

    snap_cfg.restore();
    snap_sz.restore();
    snap_ov.restore();
    snap_ts.restore();
    Ok(cases)
}

static mut FAKE_CMD_LOCK_TARGET: usize = 0;

unsafe extern "win64" fn fake_transition_noop(_res: usize) {}

unsafe extern "win64" fn fake_get_cmd_lock(out_pair: usize) {
    wr::<usize>(out_pair, FAKE_CMD_LOCK_TARGET);
    wr::<usize>(out_pair + 8, 0xcafe_babe_0001);
}

/// 2. `0x14205bb70` — `d3d_fsr2_dispatch_fg_prepare`
pub unsafe fn diff_d3d_fsr2_dispatch_fg_prepare(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        usize,
        i32,
        usize,
        usize,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
    );
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _tls = TlsGuard::new(0);
    let _m_tr = Mock::new(0x1_4206_94a0, fake_transition_noop as *const ())?;
    let _m_cl = Mock::new(0x1_411d_5b00, fake_get_cmd_lock as *const ())?;
    let _m_ts1 = Mock::new(0x1_4207_2ec0, fake_142072ec0 as *const ())?;
    let _m_ts2 = Mock::new(0x1_4207_34e0, fake_1420734e0 as *const ())?;
    let _im_d = IatMock::new(IAT_FFX_DISPATCH, fake_ffx_dispatch as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_cfg = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ce0) as *mut u8, 0x180);
    let snap_ts = Snapshot::take(exe::ptr::<u8>(0x1_43c4_8fc0) as *mut u8, 16);

    let mut tex_obj = [0u64; 16];
    tex_obj[14] = 0; // null d3d_res at +0x70
    let cam_mat: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let sc_inner: [usize; 3] = [0, 0x1111, 0x2222];
    let sc: usize = sc_inner.as_ptr() as usize;

    for fg_en in [0u8, 1u8] {
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7dc8) as usize, fg_en);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7de0) as usize, 1);

        let mut lock_o = [0u32, 0, 0x1234, 1];
        FAKE_CMD_LOCK_TARGET = lock_o.as_mut_ptr() as usize;
        SAVED_PREP_DISP = [0u8; 0xa8];
        SAVED_PREP_CAM = [0u8; 0x40];
        o(
            cam_mat.as_ptr() as usize,
            &sc as *const usize as usize,
            7,
            tex_obj.as_ptr() as usize,
            tex_obj.as_ptr() as usize,
            0x3f00_0000_bf00_0000,
            (1080u64 << 32) | 1920,
            0.016f32.to_bits() as u64,
            0.1f32.to_bits() as u64,
            1000.0f32.to_bits() as u64,
            1.0f32.to_bits() as u64,
        );
        let disp_o = SAVED_PREP_DISP;
        let cam_o = SAVED_PREP_CAM;

        let mut lock_n = [0u32, 0, 0x1234, 1];
        FAKE_CMD_LOCK_TARGET = lock_n.as_mut_ptr() as usize;
        SAVED_PREP_DISP = [0u8; 0xa8];
        SAVED_PREP_CAM = [0u8; 0x40];
        n(
            cam_mat.as_ptr() as usize,
            &sc as *const usize as usize,
            7,
            tex_obj.as_ptr() as usize,
            tex_obj.as_ptr() as usize,
            0x3f00_0000_bf00_0000,
            (1080u64 << 32) | 1920,
            0.016f32.to_bits() as u64,
            0.1f32.to_bits() as u64,
            1000.0f32.to_bits() as u64,
            1.0f32.to_bits() as u64,
        );
        crate::diff_eq!(disp_o, SAVED_PREP_DISP, "prep disp fg_en={fg_en}");
        crate::diff_eq!(cam_o, SAVED_PREP_CAM, "prep cam fg_en={fg_en}");
        crate::diff_eq!(lock_o, lock_n, "prep lock fg_en={fg_en}");
    }

    snap_cfg.restore();
    snap_ts.restore();
    Ok(2)
}

/// 3. `0x14205bed0` — `d3d_fsr2_dispatch_upscale`
pub unsafe fn diff_d3d_fsr2_dispatch_upscale(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        usize,
    );
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _tls = TlsGuard::new(0);
    let _m_tr0 = Mock::new(0x1_4206_93a0, fake_transition_noop as *const ())?;
    let _m_tr1 = Mock::new(0x1_4206_94a0, fake_transition_noop as *const ())?;
    let _m_cl = Mock::new(0x1_411d_5b00, fake_get_cmd_lock as *const ())?;
    let _m_ts1 = Mock::new(0x1_4207_2ec0, fake_142072ec0 as *const ())?;
    let _m_ts2 = Mock::new(0x1_4207_34e0, fake_1420734e0 as *const ())?;
    let _im_d = IatMock::new(IAT_FFX_DISPATCH, fake_ffx_dispatch as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_sz = Snapshot::take(exe::ptr::<u8>(0x1_43a3_2fb8) as *mut u8, 8);
    let snap_fl = Snapshot::take(exe::ptr::<u8>(0x1_4385_7e70) as *mut u8, 4);
    let snap_ts = Snapshot::take(exe::ptr::<u8>(0x1_43c4_8fc0) as *mut u8, 16);

    wr::<u32>(exe::ptr::<u32>(0x1_43a3_2fb8) as usize, 1920);
    wr::<u32>(exe::ptr::<u32>(0x1_43a3_2fbc) as usize, 1080);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_7e70) as usize, 1);

    let mut tex = [0u64; 16];
    wr::<u32>(tex.as_mut_ptr() as usize, 1280);
    wr::<u32>(tex.as_mut_ptr() as usize + 4, 720);
    let tp = tex.as_ptr() as usize;

    for (has_opt, sharp) in [(false, 0.0f32), (true, 0.75f32)] {
        let sharp_cfg: [f32; 5] = [0.0, 0.0, 0.0, 0.0, sharp];
        let opt = if has_opt { tp } else { 0 };

        let mut lock_o = [0u32, 0, 0x1234, 1];
        FAKE_CMD_LOCK_TARGET = lock_o.as_mut_ptr() as usize;
        SAVED_UP_DISP = [0u8; 0x1b0];
        o(
            tp,
            tp,
            tp,
            tp,
            opt,
            opt,
            tp,
            0x1234_5678,
            0.016f32.to_bits() as u64,
            0.1f32.to_bits() as u64,
            100.0f32.to_bits() as u64,
            1.2f32.to_bits() as u64,
            1,
            0,
            sharp_cfg.as_ptr() as usize,
        );
        let disp_o = SAVED_UP_DISP;

        let mut lock_n = [0u32, 0, 0x1234, 1];
        FAKE_CMD_LOCK_TARGET = lock_n.as_mut_ptr() as usize;
        SAVED_UP_DISP = [0u8; 0x1b0];
        n(
            tp,
            tp,
            tp,
            tp,
            opt,
            opt,
            tp,
            0x1234_5678,
            0.016f32.to_bits() as u64,
            0.1f32.to_bits() as u64,
            100.0f32.to_bits() as u64,
            1.2f32.to_bits() as u64,
            1,
            0,
            sharp_cfg.as_ptr() as usize,
        );
        crate::diff_eq!(disp_o, SAVED_UP_DISP, "upscale disp opt={has_opt}");
        crate::diff_eq!(lock_o, lock_n, "upscale lock opt={has_opt}");
    }

    snap_sz.restore();
    snap_fl.restore();
    snap_ts.restore();
    Ok(2)
}

static mut SAVED_CREATE_UP: [u8; 40] = [0u8; 40];
static mut SAVED_CREATE_FG: [u8; 40] = [0u8; 40];
static mut SAVED_CREATE_HDR: [u64; 3] = [0u64; 3];

unsafe extern "win64" fn fake_ffx_create_ctx(_ctx: usize, desc: usize, _mem: usize) -> u32 {
    let ty = rd::<u64>(desc);
    if ty == 0x10000 {
        std::ptr::copy_nonoverlapping(desc as *const u8, SAVED_CREATE_UP.as_mut_ptr(), 40);
        wr::<u64>(SAVED_CREATE_UP.as_mut_ptr() as usize + 8, 0);
    } else if ty == 0x20001 {
        std::ptr::copy_nonoverlapping(desc as *const u8, SAVED_CREATE_FG.as_mut_ptr(), 40);
        let backend = rd::<usize>(desc + 8);
        let hdr = rd::<usize>(backend + 8);
        if hdr != 0 {
            for i in 0..3 {
                SAVED_CREATE_HDR[i] = rd::<u64>(hdr + i * 8);
            }
        }
        wr::<u64>(SAVED_CREATE_FG.as_mut_ptr() as usize + 8, 0);
    }
    0
}

static mut VER_NAMES_BUF: [usize; 4] = [0; 4];
static mut VER_IDS_BUF: [u64; 4] = [0; 4];
static mut VER_QUERY_COUNT: u64 = 0;
static mut VER_RESIZE_IDX: usize = 0;

unsafe extern "win64" fn fake_vec_resize_u64(vec: usize, count: u64) {
    if count == 0 {
        wr::<usize>(vec, 0);
    } else if VER_RESIZE_IDX & 1 == 0 {
        wr::<usize>(vec, VER_NAMES_BUF.as_mut_ptr() as usize);
    } else {
        wr::<usize>(vec, VER_IDS_BUF.as_mut_ptr() as usize);
    }
    VER_RESIZE_IDX += 1;
    wr::<u32>(vec + 8, count as u32);
    wr::<u32>(vec + 12, count as u32);
}

unsafe extern "win64" fn fake_ffx_query_versions(ctx: usize, q: usize) -> u32 {
    let ty = rd::<u64>(q);
    if ty == 4 {
        let count_ptr = rd::<usize>(q + 0x20);
        wr::<u64>(count_ptr, VER_QUERY_COUNT);
        let ids_out = rd::<usize>(q + 0x28);
        let names_out = rd::<usize>(q + 0x30);
        if ids_out != 0 && names_out != 0 {
            for i in 0..(VER_QUERY_COUNT as usize) {
                wr::<u64>(ids_out + i * 8, VER_IDS_BUF[i]);
                wr::<usize>(names_out + i * 8, VER_NAMES_BUF[i]);
            }
        }
        return 0;
    }
    fake_ffx_query_fg(ctx, q)
}

unsafe extern "win64" fn fake_vec_free_noop(vec: usize) {
    wr::<usize>(vec, 0);
}

unsafe extern "win64" fn fake_vec_push_u64(vec: usize, val_ptr: usize) {
    let idx = rd::<u32>(vec + 8) as usize;
    let base = rd::<usize>(vec);
    wr::<u64>(base + idx * 8, rd::<u64>(val_ptr));
    wr::<u32>(vec + 8, (idx + 1) as u32);
}

unsafe extern "win64" fn fake_vec_reserve_u64(vec: usize, new_cap: usize) {
    wr::<u32>(vec + 12, new_cap as u32);
}

/// 4. `0x14205c4b0` — `d3d_fsr2_create_contexts`
pub unsafe fn diff_d3d_fsr2_create_contexts(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i8, u64, u64, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    VER_QUERY_COUNT = 0;
    let _m_rsz = Mock::new(0x1_408b_4ad0, fake_vec_resize_u64 as *const ())?;
    let _m_free = Mock::new(0x1_401d_e830, fake_vec_free_noop as *const ())?;
    let _m_ts2 = Mock::new(0x1_4207_34e0, fake_1420734e0 as *const ())?;
    let _im_q = IatMock::new(IAT_FFX_QUERY, fake_ffx_query_versions as *const ())?;
    let _im_c = IatMock::new(IAT_FFX_CREATE_CONTEXT, fake_ffx_create_ctx as *const ())?;

    let snap_sz = Snapshot::take(exe::ptr::<u8>(0x1_43a3_2fb8) as *mut u8, 8);
    let snap_fl = Snapshot::take(exe::ptr::<u8>(0x1_4385_7da0) as *mut u8, 0x100);
    let snap_ts = Snapshot::take(exe::ptr::<u8>(0x1_43c4_8fc0) as *mut u8, 16);

    for (fmt_idx, e28, e88, db0) in [(0i8, 0u8, 0u8, 0u8), (1, 1, 1, 1)] {
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7e28) as usize, e28);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7e88) as usize, e88);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7db0) as usize, db0);

        SAVED_CREATE_UP = [0u8; 40];
        SAVED_CREATE_FG = [0u8; 40];
        SAVED_CREATE_HDR = [0u64; 3];
        o(fmt_idx, 0, (720u64 << 32) | 1280, (1080u64 << 32) | 1920);
        let up_o = SAVED_CREATE_UP;
        let fg_o = SAVED_CREATE_FG;
        let hdr_o = SAVED_CREATE_HDR;

        SAVED_CREATE_UP = [0u8; 40];
        SAVED_CREATE_FG = [0u8; 40];
        SAVED_CREATE_HDR = [0u64; 3];
        n(fmt_idx, 0, (720u64 << 32) | 1280, (1080u64 << 32) | 1920);
        crate::diff_eq!(up_o, SAVED_CREATE_UP, "create up fmt={fmt_idx}");
        crate::diff_eq!(fg_o, SAVED_CREATE_FG, "create fg fmt={fmt_idx}");
        crate::diff_eq!(hdr_o, SAVED_CREATE_HDR, "create hdr fmt={fmt_idx}");
    }

    snap_sz.restore();
    snap_fl.restore();
    snap_ts.restore();
    Ok(2)
}

/// 5. `0x14205c820` — `d3d_fsr2_collect_supported_versions`
pub unsafe fn diff_d3d_fsr2_collect_supported_versions(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let s2 = b"2.2.1\0";
    let s3 = b"3.1.0\0";
    let s4 = b"4.0.0\0";
    let s1 = b"1.0.0\0";
    VER_NAMES_BUF = [
        s2.as_ptr() as usize,
        s3.as_ptr() as usize,
        s4.as_ptr() as usize,
        s1.as_ptr() as usize,
    ];
    VER_IDS_BUF = [0x202, 0x301, 0x400, 0x100];
    VER_QUERY_COUNT = 4;

    let _m_rsz = Mock::new(0x1_408b_4ad0, fake_vec_resize_u64 as *const ())?;
    let _m_push = Mock::new(0x1_403d_8aa0, fake_vec_push_u64 as *const ())?;
    let _m_resv = Mock::new(0x1_401d_eba0, fake_vec_reserve_u64 as *const ())?;
    let _m_free = Mock::new(0x1_401d_e830, fake_vec_free_noop as *const ())?;
    let _im_q = IatMock::new(IAT_FFX_QUERY, fake_ffx_query_versions as *const ())?;

    let snap_fl = Snapshot::take(exe::ptr::<u8>(0x1_4385_7df0) as *mut u8, 0xe0);
    for mask in [0b101u8, 0b111u8] {
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7df8) as usize, mask & 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7ea0) as usize, (mask >> 1) & 1);
        wr::<u8>(exe::ptr::<u8>(0x1_4385_7eb8) as usize, (mask >> 2) & 1);

        let mut names_o = [0usize; 4];
        let mut ids_o = [0u64; 4];
        let mut v_names_o = [names_o.as_mut_ptr() as u64, 0];
        let mut v_ids_o = [ids_o.as_mut_ptr() as u64, 0];
        VER_RESIZE_IDX = 0;
        o(v_names_o.as_mut_ptr() as usize, v_ids_o.as_mut_ptr() as usize);

        let mut names_n = [0usize; 4];
        let mut ids_n = [0u64; 4];
        let mut v_names_n = [names_n.as_mut_ptr() as u64, 0];
        let mut v_ids_n = [ids_n.as_mut_ptr() as u64, 0];
        VER_RESIZE_IDX = 0;
        n(v_names_n.as_mut_ptr() as usize, v_ids_n.as_mut_ptr() as usize);

        crate::diff_eq!(v_names_o[1], v_names_n[1], "names hdr mask={mask}");
        crate::diff_eq!(v_ids_o[1], v_ids_n[1], "ids hdr mask={mask}");
        crate::diff_eq!(names_o, names_n, "names mask={mask}");
        crate::diff_eq!(ids_o, ids_n, "ids mask={mask}");
    }
    snap_fl.restore();
    Ok(2)
}

/// 6. `0x14205c9e0` — `d3d_fsr2_query_upscale_versions`
pub unsafe fn diff_d3d_fsr2_query_upscale_versions(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    VER_NAMES_BUF = [0x1111, 0x2222, 0x3333, 0];
    VER_IDS_BUF = [0xaa, 0xbb, 0xcc, 0];
    VER_QUERY_COUNT = 3;

    let _m_rsz = Mock::new(0x1_408b_4ad0, fake_vec_resize_u64 as *const ())?;
    let _im_q = IatMock::new(IAT_FFX_QUERY, fake_ffx_query_versions as *const ())?;

    let mut v_names_o = [0u64; 2];
    let mut v_ids_o = [0u64; 2];
    VER_RESIZE_IDX = 0;
    o(v_names_o.as_mut_ptr() as usize, v_ids_o.as_mut_ptr() as usize);
    let names_o = VER_NAMES_BUF;
    let ids_o = VER_IDS_BUF;

    let mut v_names_n = [0u64; 2];
    let mut v_ids_n = [0u64; 2];
    VER_RESIZE_IDX = 0;
    n(v_names_n.as_mut_ptr() as usize, v_ids_n.as_mut_ptr() as usize);

    crate::diff_eq!(v_names_o, v_names_n, "names vec");
    crate::diff_eq!(v_ids_o, v_ids_n, "ids vec");
    crate::diff_eq!(names_o, VER_NAMES_BUF, "names data");
    crate::diff_eq!(ids_o, VER_IDS_BUF, "ids data");
    Ok(1)
}

static mut LAST_FATAL_ARGS: (usize, u32) = (0, 0);

unsafe extern "win64" fn fake_fatal_handler(rcx: usize, hr: u32) {
    LAST_FATAL_ARGS = (rcx, hr);
}

/// 7. `0x14205cdd0` — `d3d_fatal_device_removed`
pub unsafe fn diff_d3d_fatal_device_removed(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(0x1_4205_d410, fake_fatal_handler as *const ())?;
    for arg in [0usize, 0x1234_5678] {
        LAST_FATAL_ARGS = (0, 0);
        o(arg);
        let ao = LAST_FATAL_ARGS;
        LAST_FATAL_ARGS = (0, 0);
        n(arg);
        crate::diff_eq!(ao, LAST_FATAL_ARGS, "arg={arg:#x}");
    }
    Ok(2)
}

static mut FAKE_RING_PTR: usize = 0;
static mut FAKE_CPU_PTR: usize = 0;

unsafe extern "win64" fn fake_get_ring_2arg(_a0: usize, _a1: usize) -> usize {
    FAKE_RING_PTR
}

unsafe extern "win64" fn fake_ring_alloc_40(ring: usize, alloc_ptr: usize, size: usize) {
    wr::<u64>(ring + 0x30, size as u64);
    wr::<u64>(alloc_ptr, 0xaaaa_0000);
    wr::<u64>(alloc_ptr + 0x08, 0xbbbb_0000);
    wr::<u64>(alloc_ptr + 0x10, size as u64);
    wr::<usize>(alloc_ptr + 0x18, FAKE_CPU_PTR);
    wr::<u32>(alloc_ptr + 0x20, 0x11);
    wr::<u32>(alloc_ptr + 0x24, 0x22);
}

/// 8. `0x14205d7d0` — `d3d_cbv_alloc_and_copy`
pub unsafe fn diff_d3d_cbv_alloc_and_copy(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_8150, fake_get_ring_2arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_7ec0, fake_ring_alloc_40 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let lock_va = exe::ptr::<usize>(0x1_43a3_5d58) as usize;
    let saved_lock = rd::<usize>(lock_va);
    let src_data = [0x42u8; 32];
    let mut dst_buf = [0u8; 32];

    for has_cpu in [false, true] {
        FAKE_CPU_PTR = if has_cpu { dst_buf.as_mut_ptr() as usize } else { 0 };
        let mut lock_o = [0u32, 0, u32::MAX, 0];
        let mut ring_o = [0u64; 8];
        ring_o[5] = 10;
        wr::<usize>(lock_va, lock_o.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        dst_buf = [0u8; 32];
        let mut view_o = [0xaa_u8; 96];
        let ro = o(view_o.as_mut_ptr() as usize, 32, src_data.as_ptr() as usize)
            - view_o.as_ptr() as usize;
        let copied_o = dst_buf;

        let mut lock_n = [0u32, 0, u32::MAX, 0];
        let mut ring_n = [0u64; 8];
        ring_n[5] = 10;
        wr::<usize>(lock_va, lock_n.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        dst_buf = [0u8; 32];
        let mut view_n = [0xaa_u8; 96];
        let rn = n(view_n.as_mut_ptr() as usize, 32, src_data.as_ptr() as usize)
            - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "ret has_cpu={has_cpu}");
        crate::diff_eq!(view_o, view_n, "view has_cpu={has_cpu}");
        crate::diff_eq!(ring_o, ring_n, "ring has_cpu={has_cpu}");
        crate::diff_eq!(copied_o, dst_buf, "copied has_cpu={has_cpu}");
    }
    wr::<usize>(lock_va, saved_lock);
    Ok(2)
}

/// 9. `0x14205da30` — `d3d_cbv_unmap`
pub unsafe fn diff_d3d_cbv_unmap(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_8150, fake_get_ring_2arg as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let lock_va = exe::ptr::<usize>(0x1_43a3_5d58) as usize;
    let saved_lock = rd::<usize>(lock_va);

    let mut lock_o = [0u32, 0, u32::MAX, 0];
    let mut ring_o = [0u64; 8];
    ring_o[5] = 7;
    wr::<usize>(lock_va, lock_o.as_mut_ptr() as usize);
    FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
    o(0x100, 0);

    let mut lock_n = [0u32, 0, u32::MAX, 0];
    let mut ring_n = [0u64; 8];
    ring_n[5] = 7;
    wr::<usize>(lock_va, lock_n.as_mut_ptr() as usize);
    FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
    n(0x100, 0);

    wr::<usize>(lock_va, saved_lock);
    crate::diff_eq!(ring_o, ring_n, "cbv_unmap ring");
    crate::diff_eq!(lock_o, lock_n, "cbv_unmap lock");
    Ok(1)
}

/// 10. `0x14205dab0` — `d3d_cbv_alloc`
pub unsafe fn diff_d3d_cbv_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_8150, fake_get_ring_2arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_7ec0, fake_ring_alloc_40 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let lock_va = exe::ptr::<usize>(0x1_43a3_5d58) as usize;
    let saved_lock = rd::<usize>(lock_va);

    for cpu_ptr in [0usize, 0x7777_8888] {
        FAKE_CPU_PTR = cpu_ptr;
        let mut lock_o = [0u32, 0, u32::MAX, 0];
        let mut ring_o = [0u64; 8];
        wr::<usize>(lock_va, lock_o.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        let mut view_o = [0xaa_u8; 96];
        let ro = o(view_o.as_mut_ptr() as usize, 256, 256) - view_o.as_ptr() as usize;

        let mut lock_n = [0u32, 0, u32::MAX, 0];
        let mut ring_n = [0u64; 8];
        wr::<usize>(lock_va, lock_n.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        let mut view_n = [0xaa_u8; 96];
        let rn = n(view_n.as_mut_ptr() as usize, 256, 256) - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "cbv_alloc ret cpu={cpu_ptr:#x}");
        crate::diff_eq!(view_o, view_n, "cbv_alloc view cpu={cpu_ptr:#x}");
        crate::diff_eq!(ring_o, ring_n, "cbv_alloc ring cpu={cpu_ptr:#x}");
    }
    wr::<usize>(lock_va, saved_lock);
    Ok(2)
}

unsafe extern "win64" fn fake_get_ring_1arg(_a0: usize) -> usize {
    FAKE_RING_PTR
}

unsafe extern "win64" fn fake_get_ring_0arg() -> usize {
    FAKE_RING_PTR
}

/// 11. `0x14205de40` — `d3d_vbv_unmap`
pub unsafe fn diff_d3d_vbv_unmap(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_8a00, fake_get_ring_1arg as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let lock_va = exe::ptr::<usize>(0x1_43a3_5e78) as usize;
    let saved_lock = rd::<usize>(lock_va);

    let mut lock_o = [0u32, 0, u32::MAX, 0];
    let mut ring_o = [0u64; 8];
    ring_o[5] = 9;
    wr::<usize>(lock_va, lock_o.as_mut_ptr() as usize);
    FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
    o(0x200, 0);

    let mut lock_n = [0u32, 0, u32::MAX, 0];
    let mut ring_n = [0u64; 8];
    ring_n[5] = 9;
    wr::<usize>(lock_va, lock_n.as_mut_ptr() as usize);
    FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
    n(0x200, 0);

    wr::<usize>(lock_va, saved_lock);
    crate::diff_eq!(ring_o, ring_n, "vbv_unmap ring");
    crate::diff_eq!(lock_o, lock_n, "vbv_unmap lock");
    Ok(1)
}

/// 12. `0x14205dec0` — `d3d_vbv_alloc_clamped`
pub unsafe fn diff_d3d_vbv_alloc_clamped(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_8a00, fake_get_ring_1arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_8770, fake_ring_alloc_40 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let lock_va = exe::ptr::<usize>(0x1_43a3_5e78) as usize;
    let saved_lock = rd::<usize>(lock_va);
    FAKE_CPU_PTR = 0x5555_6666;

    for used in [0x3f_ff80u64, 0x3f_fe00, 0] {
        let mut lock_o = [0u32, 0, u32::MAX, 0];
        let mut ring_o = [0u64; 8];
        ring_o[4] = used;
        wr::<usize>(lock_va, lock_o.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        let mut view_o = [0xaa_u8; 96];
        let ro = o(view_o.as_mut_ptr() as usize, 0x400, 0x100) - view_o.as_ptr() as usize;

        let mut lock_n = [0u32, 0, u32::MAX, 0];
        let mut ring_n = [0u64; 8];
        ring_n[4] = used;
        wr::<usize>(lock_va, lock_n.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        let mut view_n = [0xaa_u8; 96];
        let rn = n(view_n.as_mut_ptr() as usize, 0x400, 0x100) - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "vbv_clamped ret used={used:#x}");
        crate::diff_eq!(view_o, view_n, "vbv_clamped view used={used:#x}");
        crate::diff_eq!(ring_o, ring_n, "vbv_clamped ring used={used:#x}");
    }
    wr::<usize>(lock_va, saved_lock);
    Ok(3)
}

/// 13. `0x14205e050` — `d3d_vbv_alloc`
pub unsafe fn diff_d3d_vbv_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_8a00, fake_get_ring_1arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_8770, fake_ring_alloc_40 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let lock_va = exe::ptr::<usize>(0x1_43a3_5e78) as usize;
    let saved_lock = rd::<usize>(lock_va);

    for cpu_ptr in [0usize, 0x9999_aaaa] {
        FAKE_CPU_PTR = cpu_ptr;
        let mut lock_o = [0u32, 0, u32::MAX, 0];
        let mut ring_o = [0u64; 8];
        wr::<usize>(lock_va, lock_o.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        let mut view_o = [0xaa_u8; 96];
        let ro = o(view_o.as_mut_ptr() as usize, 512) - view_o.as_ptr() as usize;

        let mut lock_n = [0u32, 0, u32::MAX, 0];
        let mut ring_n = [0u64; 8];
        wr::<usize>(lock_va, lock_n.as_mut_ptr() as usize);
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        let mut view_n = [0xaa_u8; 96];
        let rn = n(view_n.as_mut_ptr() as usize, 512) - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "vbv_alloc ret cpu={cpu_ptr:#x}");
        crate::diff_eq!(view_o, view_n, "vbv_alloc view cpu={cpu_ptr:#x}");
        crate::diff_eq!(ring_o, ring_n, "vbv_alloc ring cpu={cpu_ptr:#x}");
    }
    wr::<usize>(lock_va, saved_lock);
    Ok(2)
}

static mut POOL_OP_ACCUM: u64 = 0;

unsafe extern "win64" fn fake_pool_free(_ptr: usize) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x101);
}

unsafe extern "win64" fn fake_pool_vec_free(_ptr: usize) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x1001);
}

unsafe extern "win64" fn fake_pool_qbuf_free(_ptr: usize) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x1_0001);
}

unsafe extern "win64" fn fake_pool_res_release(_res: usize, flag: u32) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x10_0001 + flag as u64);
}

unsafe extern "win64" fn fake_pool_srv_free(_heap: usize, srv: usize) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x100_0000 ^ (srv as u64));
}

unsafe extern "win64" fn fake_elem_dtor(_elem: usize, flags: u32) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x200_0000 + flags as u64);
}

unsafe extern "win64" fn fake_res_unmap(_res: usize, subres: u32, _range: usize) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x400_0000 + subres as u64);
}

/// 14. `0x14205e2a0` — `d3d_ibv_pool_shutdown`
pub unsafe fn diff_d3d_ibv_pool_shutdown(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_free = Mock::new(0x1_4244_2de0, fake_pool_free as *const ())?;
    let _m_vfree = Mock::new(0x1_401d_e830, fake_pool_vec_free as *const ())?;
    let _m_rel = Mock::new(0x1_4209_a1e0, fake_pool_res_release as *const ())?;
    let _m_qfree = Mock::new(0x1_4034_96f0, fake_pool_qbuf_free as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let list_va = exe::ptr::<usize>(0x1_43a3_5f98) as usize;
    let q_va = exe::ptr::<usize>(0x1_4397_fc20) as usize;
    let saved_list = rd::<usize>(list_va);
    let saved_q = rd::<usize>(q_va);

    let mut elem_vtbl = [0usize; 2];
    elem_vtbl[0] = fake_elem_dtor as *const () as usize;
    let mut res_vtbl = [0usize; 12];
    res_vtbl[9] = fake_res_unmap as *const () as usize; // +0x48
    let res_obj: usize = res_vtbl.as_ptr() as usize;

    for populated in [false, true] {
        let run_once = |f: F| -> (u64, [u64; 4], [u64; 6]) {
            let elem_obj: usize = elem_vtbl.as_ptr() as usize;
            let elems: [usize; 2] = [&elem_obj as *const usize as usize, 0];
            let mut list_hdr = [elems.as_ptr() as u64, 2u64];
            let mut item0 = [&res_obj as *const usize as u64, 0x11, 0x22, 0];
            let mut item1 = [&res_obj as *const usize as u64, 0x33, 0x44, 0];
            let q_arr: [usize; 2] = [item0.as_mut_ptr() as usize, item1.as_mut_ptr() as usize];
            let mut q = [0u64; 6];
            wr::<u32>(q.as_mut_ptr() as usize + 0x08, u32::MAX);
            wr::<usize>(q.as_mut_ptr() as usize + 0x10, q_arr.as_ptr() as usize);
            wr::<u32>(q.as_mut_ptr() as usize + 0x18, 1); // head = 1
            wr::<u32>(q.as_mut_ptr() as usize + 0x20, 2); // count = 2
            wr::<u32>(q.as_mut_ptr() as usize + 0x24, 2); // cap = 2
            wr::<u64>(q.as_mut_ptr() as usize + 0x28, 5);
            if populated {
                wr::<usize>(list_va, list_hdr.as_mut_ptr() as usize);
                wr::<usize>(q_va, q.as_mut_ptr() as usize);
            } else {
                wr::<usize>(list_va, 0);
                wr::<usize>(q_va, 0);
            }
            POOL_OP_ACCUM = 0;
            f();
            item0[0] = 0;
            q[2] = 0; // clear stack pointer at +0x10 before comparing
            (POOL_OP_ACCUM, item0, q)
        };
        let (acc_o, it_o, q_o) = run_once(o);
        let lo = (rd::<usize>(list_va), rd::<usize>(q_va));
        let (acc_n, it_n, q_n) = run_once(n);
        let ln = (rd::<usize>(list_va), rd::<usize>(q_va));
        crate::diff_eq!(acc_o, acc_n, "ibv_shutdown acc pop={populated}");
        crate::diff_eq!(it_o, it_n, "ibv_shutdown item pop={populated}");
        crate::diff_eq!(q_o, q_n, "ibv_shutdown q pop={populated}");
        crate::diff_eq!(lo, ln, "ibv_shutdown globals pop={populated}");
    }

    wr::<usize>(list_va, saved_list);
    wr::<usize>(q_va, saved_q);
    Ok(2)
}

static mut INIT_ALLOC_SEQ: usize = 0;

unsafe extern "win64" fn fake_alloc_list() -> usize {
    INIT_ALLOC_SEQ += 1;
    0xaaaa_0000 + INIT_ALLOC_SEQ
}

unsafe extern "win64" fn fake_alloc_queue() -> usize {
    INIT_ALLOC_SEQ += 1;
    0xbbbb_0000 + INIT_ALLOC_SEQ
}

unsafe extern "win64" fn fake_ibv_init_pages() {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(0x777);
}

/// 15. `0x14205e410` — `d3d_ibv_pool_init`
pub unsafe fn diff_d3d_ibv_pool_init(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_l = Mock::new(0x1_40fd_31f0, fake_alloc_list as *const ())?;
    let _m_q = Mock::new(0x1_411d_7e20, fake_alloc_queue as *const ())?;
    let _m_p = Mock::new(0x1_411d_93b0, fake_ibv_init_pages as *const ())?;

    let list_va = exe::ptr::<usize>(0x1_43a3_5f98) as usize;
    let q_va = exe::ptr::<usize>(0x1_4397_fc20) as usize;
    let saved_list = rd::<usize>(list_va);
    let saved_q = rd::<usize>(q_va);

    INIT_ALLOC_SEQ = 0;
    POOL_OP_ACCUM = 0;
    o();
    let res_o = (rd::<usize>(list_va), rd::<usize>(q_va), POOL_OP_ACCUM);

    INIT_ALLOC_SEQ = 0;
    POOL_OP_ACCUM = 0;
    n();
    let res_n = (rd::<usize>(list_va), rd::<usize>(q_va), POOL_OP_ACCUM);

    wr::<usize>(list_va, saved_list);
    wr::<usize>(q_va, saved_q);
    crate::diff_eq!(res_o, res_n, "ibv_pool_init");
    Ok(1)
}

/// 16. `0x14205e440` — `d3d_ibv_alloc_and_copy`
pub unsafe fn diff_d3d_ibv_alloc_and_copy(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_9220, fake_get_ring_0arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_8f90, fake_ring_alloc_40 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_lock = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ec0) as *mut u8, 16);
    let src_indices = [0x1122_3344u32; 8];
    let mut dst_buf = [0u8; 32];

    for has_cpu in [false, true] {
        FAKE_CPU_PTR = if has_cpu { dst_buf.as_mut_ptr() as usize } else { 0 };
        snap_lock.restore();
        let mut ring_o = [0u64; 8];
        ring_o[5] = 4;
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        dst_buf = [0u8; 32];
        let mut view_o = [0xaa_u8; 96];
        let ro = o(
            view_o.as_mut_ptr() as usize,
            8,
            0,
            src_indices.as_ptr() as usize,
        ) - view_o.as_ptr() as usize;
        let copied_o = dst_buf;

        snap_lock.restore();
        let mut ring_n = [0u64; 8];
        ring_n[5] = 4;
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        dst_buf = [0u8; 32];
        let mut view_n = [0xaa_u8; 96];
        let rn = n(
            view_n.as_mut_ptr() as usize,
            8,
            0,
            src_indices.as_ptr() as usize,
        ) - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "ibv_copy ret has_cpu={has_cpu}");
        crate::diff_eq!(view_o, view_n, "ibv_copy view has_cpu={has_cpu}");
        crate::diff_eq!(ring_o, ring_n, "ibv_copy ring has_cpu={has_cpu}");
        crate::diff_eq!(copied_o, dst_buf, "ibv_copy bytes has_cpu={has_cpu}");
    }
    snap_lock.restore();
    Ok(2)
}

/// 17. `0x14205e6d0` — `d3d_tls_ring_unmap`
pub unsafe fn diff_d3d_tls_ring_unmap(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _tls = TlsGuard::new(0x1234_0000);
    let _m_get = Mock::new(0x1_411d_9920, fake_get_ring_1arg as *const ())?;

    let mut ring_o = [0u64; 8];
    ring_o[5] = 12;
    FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
    o();

    let mut ring_n = [0u64; 8];
    ring_n[5] = 12;
    FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
    n();

    crate::diff_eq!(ring_o, ring_n, "tls_ring_unmap");
    Ok(1)
}

unsafe extern "win64" fn fake_tls_ring_alloc_fn(ring: usize, tmp: usize, size: usize) -> usize {
    wr::<u64>(ring + 0x30, size as u64);
    for i in 0..5usize {
        wr::<u64>(tmp + i * 8, 0x1000_0000 + (i as u64) * 0x111 + (size as u64));
    }
    tmp
}

/// 18. `0x14205e700` — `d3d_tls_ring_alloc`
pub unsafe fn diff_d3d_tls_ring_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _tls = TlsGuard::new(0x5678_0000);
    let _m_get = Mock::new(0x1_411d_9920, fake_get_ring_1arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_9690, fake_tls_ring_alloc_fn as *const ())?;

    let mut ring_o = [0u64; 8];
    FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
    let mut out_o = [0xaa_u8; 40];
    let ro = o(out_o.as_mut_ptr() as usize, 128) - out_o.as_ptr() as usize;

    let mut ring_n = [0u64; 8];
    FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
    let mut out_n = [0xaa_u8; 40];
    let rn = n(out_n.as_mut_ptr() as usize, 128) - out_n.as_ptr() as usize;

    crate::diff_eq!(ro, rn, "tls_ring_alloc ret");
    crate::diff_eq!(out_o, out_n, "tls_ring_alloc out");
    crate::diff_eq!(ring_o, ring_n, "tls_ring_alloc ring");
    Ok(1)
}

/// 19. `0x14205ec50` — `d3d_sbv_pool_shutdown`
pub unsafe fn diff_d3d_sbv_pool_shutdown(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_free = Mock::new(0x1_4244_2de0, fake_pool_free as *const ())?;
    let _m_vfree = Mock::new(0x1_401d_e830, fake_pool_vec_free as *const ())?;
    let _m_sfree = Mock::new(0x1_4209_6a90, fake_pool_srv_free as *const ())?;
    let _m_rel = Mock::new(0x1_4209_a1e0, fake_pool_res_release as *const ())?;
    let _m_qfree = Mock::new(0x1_4034_96f0, fake_pool_qbuf_free as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let list_va = exe::ptr::<usize>(0x1_43a3_5fe8) as usize;
    let q_va = exe::ptr::<usize>(0x1_4397_fc98) as usize;
    let saved_list = rd::<usize>(list_va);
    let saved_q = rd::<usize>(q_va);

    let mut elem_vtbl = [0usize; 2];
    elem_vtbl[0] = fake_elem_dtor as *const () as usize;
    let mut res_vtbl = [0usize; 12];
    res_vtbl[9] = fake_res_unmap as *const () as usize;
    let res_obj: usize = res_vtbl.as_ptr() as usize;

    for populated in [false, true] {
        let run_once = |f: F| -> (u64, [u64; 4], [u64; 6]) {
            let elem_obj: usize = elem_vtbl.as_ptr() as usize;
            let elems: [usize; 2] = [&elem_obj as *const usize as usize, 0];
            let mut list_hdr = [elems.as_ptr() as u64, 2u64];
            let mut item0 = [&res_obj as *const usize as u64, 0x11, 0x22, 0x777];
            let mut item1 = [&res_obj as *const usize as u64, 0x33, 0x44, 0x888];
            let q_arr: [usize; 2] = [item0.as_mut_ptr() as usize, item1.as_mut_ptr() as usize];
            let mut q = [0u64; 6];
            wr::<u32>(q.as_mut_ptr() as usize + 0x08, u32::MAX);
            wr::<usize>(q.as_mut_ptr() as usize + 0x10, q_arr.as_ptr() as usize);
            wr::<u32>(q.as_mut_ptr() as usize + 0x18, 1);
            wr::<u32>(q.as_mut_ptr() as usize + 0x20, 2);
            wr::<u32>(q.as_mut_ptr() as usize + 0x24, 2);
            wr::<u64>(q.as_mut_ptr() as usize + 0x28, 5);
            if populated {
                wr::<usize>(list_va, list_hdr.as_mut_ptr() as usize);
                wr::<usize>(q_va, q.as_mut_ptr() as usize);
            } else {
                wr::<usize>(list_va, 0);
                wr::<usize>(q_va, 0);
            }
            POOL_OP_ACCUM = 0;
            f();
            item0[0] = 0;
            q[2] = 0;
            (POOL_OP_ACCUM, item0, q)
        };
        let (acc_o, it_o, q_o) = run_once(o);
        let lo = (rd::<usize>(list_va), rd::<usize>(q_va));
        let (acc_n, it_n, q_n) = run_once(n);
        let ln = (rd::<usize>(list_va), rd::<usize>(q_va));
        crate::diff_eq!(acc_o, acc_n, "sbv_shutdown acc pop={populated}");
        crate::diff_eq!(it_o, it_n, "sbv_shutdown item pop={populated}");
        crate::diff_eq!(q_o, q_n, "sbv_shutdown q pop={populated}");
        crate::diff_eq!(lo, ln, "sbv_shutdown globals pop={populated}");
    }

    wr::<usize>(list_va, saved_list);
    wr::<usize>(q_va, saved_q);
    Ok(2)
}

static mut SBV_INIT_Q_PTR: usize = 0;

unsafe extern "win64" fn fake_sbv_alloc_q() -> usize {
    SBV_INIT_Q_PTR
}

unsafe extern "win64" fn fake_sbv_alloc_item() -> usize {
    INIT_ALLOC_SEQ += 1;
    0x5000 + INIT_ALLOC_SEQ
}

unsafe extern "win64" fn fake_sbv_init_item(item: usize) {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(item as u64);
}

unsafe extern "win64" fn fake_sbv_grow_q(q_plus_10: usize, min_cap: u32) {
    wr::<u32>(q_plus_10 + 0x14, min_cap.max(32));
}

/// 20. `0x14205edd0` — `d3d_sbv_pool_init`
pub unsafe fn diff_d3d_sbv_pool_init(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_l = Mock::new(0x1_40fd_31f0, fake_alloc_list as *const ())?;
    let _m_q = Mock::new(0x1_411d_7e20, fake_sbv_alloc_q as *const ())?;
    let _m_ai = Mock::new(0x1_411d_a620, fake_sbv_alloc_item as *const ())?;
    let _m_ii = Mock::new(0x1_411d_a6a0, fake_sbv_init_item as *const ())?;
    let _m_gr = Mock::new(0x1_4034_97a0, fake_sbv_grow_q as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let list_va = exe::ptr::<usize>(0x1_43a3_5fe8) as usize;
    let q_va = exe::ptr::<usize>(0x1_4397_fc98) as usize;
    let saved_list = rd::<usize>(list_va);
    let saved_q = rd::<usize>(q_va);

    let run_once = |f: F| -> ([usize; 32], [u64; 6], u64, usize) {
        let mut arr = [0usize; 32];
        let mut q = [0u64; 6];
        wr::<u32>(q.as_mut_ptr() as usize + 0x08, u32::MAX);
        wr::<usize>(q.as_mut_ptr() as usize + 0x10, arr.as_mut_ptr() as usize);
        wr::<u32>(q.as_mut_ptr() as usize + 0x24, 16); // initial cap 16 -> grows to 32
        SBV_INIT_Q_PTR = q.as_mut_ptr() as usize;
        INIT_ALLOC_SEQ = 0;
        POOL_OP_ACCUM = 0;
        f();
        q[2] = 0;
        (arr, q, POOL_OP_ACCUM, rd::<usize>(list_va))
    };

    let (arr_o, q_o, acc_o, l_o) = run_once(o);
    let (arr_n, q_n, acc_n, l_n) = run_once(n);

    wr::<usize>(list_va, saved_list);
    wr::<usize>(q_va, saved_q);
    crate::diff_eq!(arr_o, arr_n, "sbv_pool_init arr");
    crate::diff_eq!(q_o, q_n, "sbv_pool_init q");
    crate::diff_eq!(acc_o, acc_n, "sbv_pool_init acc");
    crate::diff_eq!(l_o, l_n, "sbv_pool_init list");
    Ok(1)
}

unsafe extern "win64" fn fake_ring_alloc_48(ring: usize, alloc_ptr: usize, size: usize) {
    wr::<u64>(ring + 0x30, size as u64);
    wr::<u64>(alloc_ptr, 0xaaaa_1111);
    wr::<u64>(alloc_ptr + 0x08, 0xbbbb_2222);
    wr::<u64>(alloc_ptr + 0x10, size as u64);
    wr::<u64>(alloc_ptr + 0x18, 0xcccc_3333);
    wr::<usize>(alloc_ptr + 0x20, FAKE_CPU_PTR);
    wr::<u32>(alloc_ptr + 0x28, 0x33);
    wr::<u32>(alloc_ptr + 0x2c, 0x44);
}

/// 21. `0x14205eec0` — `d3d_sbv_alloc_and_copy`
pub unsafe fn diff_d3d_sbv_alloc_and_copy(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_a370, fake_get_ring_2arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_a080, fake_ring_alloc_48 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_lock = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ee0) as *mut u8, 16);
    let src_data = [0x5au8; 32];
    let mut dst_buf = [0u8; 32];

    for has_cpu in [false, true] {
        FAKE_CPU_PTR = if has_cpu { dst_buf.as_mut_ptr() as usize } else { 0 };
        snap_lock.restore();
        let mut ring_o = [0u64; 8];
        ring_o[5] = 6;
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        dst_buf = [0u8; 32];
        let mut view_o = [0xaa_u8; 96];
        let ro = o(view_o.as_mut_ptr() as usize, 32, src_data.as_ptr() as usize)
            - view_o.as_ptr() as usize;
        let copied_o = dst_buf;

        snap_lock.restore();
        let mut ring_n = [0u64; 8];
        ring_n[5] = 6;
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        dst_buf = [0u8; 32];
        let mut view_n = [0xaa_u8; 96];
        let rn = n(view_n.as_mut_ptr() as usize, 32, src_data.as_ptr() as usize)
            - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "sbv_copy ret has_cpu={has_cpu}");
        crate::diff_eq!(view_o, view_n, "sbv_copy view has_cpu={has_cpu}");
        crate::diff_eq!(ring_o, ring_n, "sbv_copy ring has_cpu={has_cpu}");
        crate::diff_eq!(copied_o, dst_buf, "sbv_copy bytes has_cpu={has_cpu}");
    }
    snap_lock.restore();
    Ok(2)
}

/// 22. `0x14205f070` — `d3d_sbv_unmap`
pub unsafe fn diff_d3d_sbv_unmap(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_a370, fake_get_ring_2arg as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_lock = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ee0) as *mut u8, 16);

    snap_lock.restore();
    let mut ring_o = [0u64; 8];
    ring_o[5] = 11;
    FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
    o(0x300, 0);

    snap_lock.restore();
    let mut ring_n = [0u64; 8];
    ring_n[5] = 11;
    FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
    n(0x300, 0);

    snap_lock.restore();
    crate::diff_eq!(ring_o, ring_n, "sbv_unmap ring");
    Ok(1)
}

/// 23. `0x14205f0f0` — `d3d_sbv_alloc_clamped`
pub unsafe fn diff_d3d_sbv_alloc_clamped(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_a370, fake_get_ring_2arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_a080, fake_ring_alloc_48 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_lock = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ee0) as *mut u8, 16);
    FAKE_CPU_PTR = 0x1234_5678;

    for used in [0x10_0000u64, 0x0f_ff00, 0] {
        snap_lock.restore();
        let mut ring_o = [0u64; 8];
        ring_o[4] = used;
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        let mut view_o = [0xaa_u8; 96];
        let ro = o(view_o.as_mut_ptr() as usize, 0x400) - view_o.as_ptr() as usize;

        snap_lock.restore();
        let mut ring_n = [0u64; 8];
        ring_n[4] = used;
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        let mut view_n = [0xaa_u8; 96];
        let rn = n(view_n.as_mut_ptr() as usize, 0x400) - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "sbv_clamped ret used={used:#x}");
        crate::diff_eq!(view_o, view_n, "sbv_clamped view used={used:#x}");
        crate::diff_eq!(ring_o, ring_n, "sbv_clamped ring used={used:#x}");
    }
    snap_lock.restore();
    Ok(3)
}

/// 24. `0x14205f290` — `d3d_sbv_alloc`
pub unsafe fn diff_d3d_sbv_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_get = Mock::new(0x1_411d_a370, fake_get_ring_2arg as *const ())?;
    let _m_al = Mock::new(0x1_411d_a080, fake_ring_alloc_48 as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_lock = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ee0) as *mut u8, 16);

    for cpu_ptr in [0usize, 0xdead_beef] {
        FAKE_CPU_PTR = cpu_ptr;
        snap_lock.restore();
        let mut ring_o = [0u64; 8];
        FAKE_RING_PTR = ring_o.as_mut_ptr() as usize;
        let mut view_o = [0xaa_u8; 96];
        let ro = o(view_o.as_mut_ptr() as usize, 256, 16) - view_o.as_ptr() as usize;

        snap_lock.restore();
        let mut ring_n = [0u64; 8];
        FAKE_RING_PTR = ring_n.as_mut_ptr() as usize;
        let mut view_n = [0xaa_u8; 96];
        let rn = n(view_n.as_mut_ptr() as usize, 256, 16) - view_n.as_ptr() as usize;

        crate::diff_eq!(ro, rn, "sbv_alloc ret cpu={cpu_ptr:#x}");
        crate::diff_eq!(view_o, view_n, "sbv_alloc view cpu={cpu_ptr:#x}");
        crate::diff_eq!(ring_o, ring_n, "sbv_alloc ring cpu={cpu_ptr:#x}");
    }
    snap_lock.restore();
    Ok(2)
}

/// 25. `0x142060060` — `d3d_cmd_context_pool_shutdown`
pub unsafe fn diff_d3d_cmd_context_pool_shutdown(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_free = Mock::new(0x1_4244_2de0, fake_pool_free as *const ())?;
    let _m_vfree = Mock::new(0x1_401d_e830, fake_pool_vec_free as *const ())?;
    let _m_qfree = Mock::new(0x1_4034_96f0, fake_pool_qbuf_free as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let va0 = exe::ptr::<usize>(0x1_4397_fc30) as usize;
    let va1 = exe::ptr::<usize>(0x1_4397_fc38) as usize;
    let va2 = exe::ptr::<usize>(0x1_43a3_6078) as usize;
    let va3 = exe::ptr::<usize>(0x1_43a3_6148) as usize;
    let saved = (rd::<usize>(va0), rd::<usize>(va1), rd::<usize>(va2), rd::<usize>(va3));

    for populated in [false, true] {
        let run_once = |f: F| -> (u64, [[u64; 6]; 3], [usize; 4]) {
            let arr0: [usize; 2] = [0x1000, 0];
            let arr1: [usize; 2] = [0x2000, 0x3000];
            let arr3: [usize; 2] = [0x4000, 0];
            let mut qs = [[0u64; 6]; 3];
            for (k, arr_ptr) in [arr0.as_ptr(), arr1.as_ptr(), arr3.as_ptr()].into_iter().enumerate() {
                let qp = qs[k].as_mut_ptr() as usize;
                wr::<u32>(qp + 0x08, u32::MAX);
                wr::<usize>(qp + 0x10, arr_ptr as usize);
                wr::<u32>(qp + 0x18, 1);
                wr::<u32>(qp + 0x20, 2);
                wr::<u32>(qp + 0x24, 2);
                wr::<u64>(qp + 0x28, 3);
            }
            let mut list_hdr = [0u64; 2];
            if populated {
                wr::<usize>(va0, qs[0].as_mut_ptr() as usize);
                wr::<usize>(va1, qs[1].as_mut_ptr() as usize);
                wr::<usize>(va2, list_hdr.as_mut_ptr() as usize);
                wr::<usize>(va3, qs[2].as_mut_ptr() as usize);
            } else {
                wr::<usize>(va0, 0);
                wr::<usize>(va1, 0);
                wr::<usize>(va2, 0);
                wr::<usize>(va3, 0);
            }
            POOL_OP_ACCUM = 0;
            f();
            for q in &mut qs {
                q[2] = 0;
            }
            (
                POOL_OP_ACCUM,
                qs,
                [rd::<usize>(va0), rd::<usize>(va1), rd::<usize>(va2), rd::<usize>(va3)],
            )
        };
        let (acc_o, qs_o, g_o) = run_once(o);
        let (acc_n, qs_n, g_n) = run_once(n);
        crate::diff_eq!(acc_o, acc_n, "cmd_pool_shutdown acc pop={populated}");
        crate::diff_eq!(qs_o, qs_n, "cmd_pool_shutdown qs pop={populated}");
        crate::diff_eq!(g_o, g_n, "cmd_pool_shutdown globals pop={populated}");
    }

    wr::<usize>(va0, saved.0);
    wr::<usize>(va1, saved.1);
    wr::<usize>(va2, saved.2);
    wr::<usize>(va3, saved.3);
    Ok(2)
}

/// 26. `0x1420602d0` — `d3d_cmd_context_pool_init`
pub unsafe fn diff_d3d_cmd_context_pool_init(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_l = Mock::new(0x1_40fd_31f0, fake_alloc_list as *const ())?;
    let _m_q = Mock::new(0x1_411d_7e20, fake_alloc_queue as *const ())?;

    let va0 = exe::ptr::<usize>(0x1_4397_fc38) as usize;
    let va1 = exe::ptr::<usize>(0x1_4397_fc30) as usize;
    let va2 = exe::ptr::<usize>(0x1_43a3_6078) as usize;
    let va3 = exe::ptr::<usize>(0x1_43a3_6148) as usize;
    let saved = (rd::<usize>(va0), rd::<usize>(va1), rd::<usize>(va2), rd::<usize>(va3));

    INIT_ALLOC_SEQ = 0;
    o();
    let go = [rd::<usize>(va0), rd::<usize>(va1), rd::<usize>(va2), rd::<usize>(va3)];

    INIT_ALLOC_SEQ = 0;
    n();
    let gn = [rd::<usize>(va0), rd::<usize>(va1), rd::<usize>(va2), rd::<usize>(va3)];

    wr::<usize>(va0, saved.0);
    wr::<usize>(va1, saved.1);
    wr::<usize>(va2, saved.2);
    wr::<usize>(va3, saved.3);
    crate::diff_eq!(go, gn, "cmd_context_pool_init");
    Ok(1)
}

/// 27. `0x142060310` — `d3d_unregister_cmd_listener`
pub unsafe fn diff_d3d_unregister_cmd_listener(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_lock = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ef0) as *mut u8, 16);
    let vec_va = exe::ptr::<usize>(0x1_43a3_6078) as usize;
    let saved_vec = rd::<usize>(vec_va);

    for (init_arr, len, target) in [
        ([0x10usize, 0x20, 0x30, 0], 3u32, 0x99usize),
        ([0x10, 0x20, 0x10, 0x30], 4, 0x10),
        ([0x10, 0x10, 0, 0], 2, 0x10),
    ] {
        snap_lock.restore();
        let mut arr_o = init_arr;
        let mut vec_o = [arr_o.as_mut_ptr() as u64, (4u64 << 32) | (len as u64)];
        wr::<usize>(vec_va, vec_o.as_mut_ptr() as usize);
        o(target);

        snap_lock.restore();
        let mut arr_n = init_arr;
        let mut vec_n = [arr_n.as_mut_ptr() as u64, (4u64 << 32) | (len as u64)];
        wr::<usize>(vec_va, vec_n.as_mut_ptr() as usize);
        n(target);

        crate::diff_eq!(vec_o[1], vec_n[1], "unregister hdr target={target:#x}");
        crate::diff_eq!(arr_o, arr_n, "unregister arr target={target:#x}");
    }

    wr::<usize>(vec_va, saved_vec);
    snap_lock.restore();
    Ok(3)
}

unsafe extern "win64" fn fake_listener_vec_grow(vec: usize, min_cap: usize) {
    wr::<u32>(vec + 0x0c, min_cap.max(4) as u32);
}

/// 28. `0x142060410` — `d3d_register_cmd_listener`
pub unsafe fn diff_d3d_register_cmd_listener(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_gr = Mock::new(0x1_4020_bcb0, fake_listener_vec_grow as *const ())?;
    let _im_tid = IatMock::new(IAT_GET_CURRENT_THREAD_ID, fake_get_tid as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let snap_lock = Snapshot::take(exe::ptr::<u8>(0x1_4385_7ef0) as *mut u8, 16);
    let vec_va = exe::ptr::<usize>(0x1_43a3_6078) as usize;
    let saved_vec = rd::<usize>(vec_va);

    for (len, cap) in [(1u32, 4u32), (2u32, 2u32)] {
        snap_lock.restore();
        let mut arr_o = [0x11usize, 0x22, 0, 0];
        let mut vec_o = [arr_o.as_mut_ptr() as u64, ((cap as u64) << 32) | (len as u64)];
        wr::<usize>(vec_va, vec_o.as_mut_ptr() as usize);
        o(0x7777);

        snap_lock.restore();
        let mut arr_n = [0x11usize, 0x22, 0, 0];
        let mut vec_n = [arr_n.as_mut_ptr() as u64, ((cap as u64) << 32) | (len as u64)];
        wr::<usize>(vec_va, vec_n.as_mut_ptr() as usize);
        n(0x7777);

        crate::diff_eq!(vec_o[1], vec_n[1], "register hdr len={len} cap={cap}");
        crate::diff_eq!(arr_o, arr_n, "register arr len={len} cap={cap}");
    }

    wr::<usize>(vec_va, saved_vec);
    snap_lock.restore();
    Ok(2)
}

static mut FAKE_PSO_ENTRY_PTR: usize = 0;
static mut SAVED_PSO_DESC: [u64; 3] = [0; 3];

unsafe extern "win64" fn fake_pso_map_lookup(_map: usize, out_pair: usize, key_ptr: usize) -> usize {
    wr::<u64>(FAKE_PSO_ENTRY_PTR, rd::<u64>(key_ptr));
    wr::<usize>(out_pair, 0);
    wr::<usize>(out_pair + 8, FAKE_PSO_ENTRY_PTR);
    out_pair
}

unsafe extern "win64" fn fake_create_compute_pso(
    _dev: usize,
    desc: usize,
    _riid: usize,
    out_pso: usize,
) -> u32 {
    SAVED_PSO_DESC = [rd::<u64>(desc), rd::<u64>(desc + 8), rd::<u64>(desc + 16)];
    wr::<usize>(out_pso, 0xbeef_cafe);
    0
}

unsafe extern "win64" fn fake_com_release(_this: usize) -> u32 {
    POOL_OP_ACCUM = POOL_OP_ACCUM.wrapping_add(1);
    0
}

/// 29. `0x1420604d0` — `d3d_get_or_create_compute_pso`
pub unsafe fn diff_d3d_get_or_create_compute_pso(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_lk = Mock::new(0x1_411d_bd00, fake_pso_map_lookup as *const ())?;
    let _im_acq = IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im_rel = IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;

    let mgr_va = exe::ptr::<usize>(0x1_4397_fc50) as usize;
    let ctx_va = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_mgr = rd::<usize>(mgr_va);
    let saved_ctx = rd::<usize>(ctx_va);

    let mut dev_vtbl = [0usize; 14];
    dev_vtbl[11] = fake_create_compute_pso as *const () as usize; // +0x58
    let dev_obj: usize = dev_vtbl.as_ptr() as usize;

    let mut old_vtbl = [0usize; 4];
    old_vtbl[2] = fake_com_release as *const () as usize; // +0x10
    let old_pso_obj: usize = old_vtbl.as_ptr() as usize;

    let mut dev_ctx = [0u64; 8];
    dev_ctx[0] = &dev_obj as *const usize as u64;
    dev_ctx[6] = 0x5555_0030; // root_sig at +0x30
    let mgr_buf = [0u64; 16];
    wr::<usize>(mgr_va, mgr_buf.as_ptr() as usize);
    wr::<usize>(ctx_va, dev_ctx.as_ptr() as usize);

    for (init_cached, entry_state) in [(0x9999usize, 2u8), (0, 2), (0, 0)] {
        let mut cs_o = [0u64, 0x100, 0xabc0_0000, 0, 0x777, init_cached as u64];
        let mut entry_o = [0u64, entry_state as u64, &old_pso_obj as *const usize as u64];
        FAKE_PSO_ENTRY_PTR = entry_o.as_mut_ptr() as usize;
        SAVED_PSO_DESC = [0; 3];
        POOL_OP_ACCUM = 0;
        let ro = o(cs_o.as_mut_ptr() as usize);
        let desc_o = SAVED_PSO_DESC;
        let rel_o = POOL_OP_ACCUM;

        let mut cs_n = [0u64, 0x100, 0xabc0_0000, 0, 0x777, init_cached as u64];
        let mut entry_n = [0u64, entry_state as u64, &old_pso_obj as *const usize as u64];
        FAKE_PSO_ENTRY_PTR = entry_n.as_mut_ptr() as usize;
        SAVED_PSO_DESC = [0; 3];
        POOL_OP_ACCUM = 0;
        let rn = n(cs_n.as_mut_ptr() as usize);

        crate::diff_eq!(ro, rn, "pso ret cached={init_cached:#x} st={entry_state}");
        crate::diff_eq!(cs_o, cs_n, "pso cs cached={init_cached:#x} st={entry_state}");
        crate::diff_eq!(entry_o, entry_n, "pso entry cached={init_cached:#x} st={entry_state}");
        crate::diff_eq!(desc_o, SAVED_PSO_DESC, "pso desc st={entry_state}");
        crate::diff_eq!(rel_o, POOL_OP_ACCUM, "pso rel st={entry_state}");
    }

    wr::<usize>(mgr_va, saved_mgr);
    wr::<usize>(ctx_va, saved_ctx);
    Ok(3)
}

static mut FAKE_DESCS_BUF_PTR: usize = 0;
static mut FAKE_LAYOUT_SLOT_PTR: usize = 0;

unsafe extern "win64" fn fake_alloc_input_descs(
    _rcx: usize,
    _vs: usize,
    _a2: usize,
    _a3: u32,
) -> usize {
    FAKE_DESCS_BUF_PTR
}

unsafe extern "win64" fn fake_get_input_layout_slot(_layout_id: usize) -> usize {
    FAKE_LAYOUT_SLOT_PTR
}

/// 30. `0x142060720` — `d3d_build_vertex_shader_input_layout`
pub unsafe fn diff_d3d_build_vertex_shader_input_layout(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_al = Mock::new(0x1_4206_0610, fake_alloc_input_descs as *const ())?;
    let _m_sl = Mock::new(0x1_411d_ad70, fake_get_input_layout_slot as *const ())?;

    let snap_layouts = Snapshot::take(exe::ptr::<u8>(0x1_43a4_9130) as *mut u8, 2 * 0x68);
    let l1 = exe::ptr::<u8>(0x1_43a4_9130) as usize + 0x68;
    wr::<u32>(l1, 0x0000_0200); // slot 0, fmt 2, POSITION
    wr::<u32>(l1 + 4, 0x0002_0100); // slot 0, fmt 1, TEXCOORD
    wr::<u32>(l1 + 8, 0x0107_0301); // slot 1, fmt 3, RenderInstance, step 1
    wr::<u8>(l1 + 0x40, 3);

    let vs: u32 = 1;
    let mut descs_buf = [0u8; 3 * 0x20];
    FAKE_DESCS_BUF_PTR = descs_buf.as_mut_ptr() as usize;

    let mut slot_o = [0u64; 2];
    FAKE_LAYOUT_SLOT_PTR = slot_o.as_mut_ptr() as usize;
    descs_buf = [0xaa_u8; 3 * 0x20];
    o(0, &vs as *const u32 as usize);
    let descs_o = descs_buf;

    let mut slot_n = [0u64; 2];
    FAKE_LAYOUT_SLOT_PTR = slot_n.as_mut_ptr() as usize;
    descs_buf = [0xaa_u8; 3 * 0x20];
    n(0, &vs as *const u32 as usize);

    snap_layouts.restore();
    crate::diff_eq!(descs_o, descs_buf, "vs_input_layout descs");
    crate::diff_eq!(slot_o, slot_n, "vs_input_layout slot");
    Ok(1)
}

// REGISTRY: Replacement::verified_throws(0x1_4205_b7b0, "d3d::fsr2::configureAndDispatchFg_14205b7b0", rend::fsr2_glue::d3d_fsr2_configure_and_dispatch_fg as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_fsr2_configure_and_dispatch_fg),
// REGISTRY: Replacement::verified_throws(0x1_4205_bb70, "d3d::fsr2::dispatchFgPrepare_14205bb70", rend::fsr2_glue::d3d_fsr2_dispatch_fg_prepare as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_fsr2_dispatch_fg_prepare),
// REGISTRY: Replacement::verified_throws(0x1_4205_bed0, "d3d::fsr2::dispatchUpscale_14205bed0", rend::fsr2_glue::d3d_fsr2_dispatch_upscale as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_fsr2_dispatch_upscale),
// REGISTRY: Replacement::verified_throws(0x1_4205_c4b0, "d3d::fsr2::createContexts_14205c4b0", rend::fsr2_glue::d3d_fsr2_create_contexts as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_fsr2_create_contexts),
// REGISTRY: Replacement::verified(0x1_4205_c820, "d3d::fsr2::collectSupportedVersions_14205c820", rend::fsr2_glue::d3d_fsr2_collect_supported_versions as *const (), rend::fsr2_glue::diff_d3d_fsr2_collect_supported_versions),
// REGISTRY: Replacement::verified(0x1_4205_c9e0, "d3d::fsr2::queryUpscaleVersions_14205c9e0", rend::fsr2_glue::d3d_fsr2_query_upscale_versions as *const (), rend::fsr2_glue::diff_d3d_fsr2_query_upscale_versions),
// REGISTRY: Replacement::verified_throws(0x1_4205_cdd0, "d3d::fatalDeviceRemoved_14205cdd0", rend::fsr2_glue::d3d_fatal_device_removed as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_fatal_device_removed),
// REGISTRY: Replacement::verified_throws(0x1_4205_d7d0, "d3d::cbvAllocAndCopy_14205d7d0", rend::fsr2_glue::d3d_cbv_alloc_and_copy as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_cbv_alloc_and_copy),
// REGISTRY: Replacement::verified(0x1_4205_da30, "d3d::cbvUnmap_14205da30", rend::fsr2_glue::d3d_cbv_unmap as *const (), rend::fsr2_glue::diff_d3d_cbv_unmap),
// REGISTRY: Replacement::verified_throws(0x1_4205_dab0, "d3d::cbvAlloc_14205dab0", rend::fsr2_glue::d3d_cbv_alloc as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_cbv_alloc),
// REGISTRY: Replacement::verified(0x1_4205_de40, "d3d::vbvUnmap_14205de40", rend::fsr2_glue::d3d_vbv_unmap as *const (), rend::fsr2_glue::diff_d3d_vbv_unmap),
// REGISTRY: Replacement::verified_throws(0x1_4205_dec0, "d3d::vbvAllocClamped_14205dec0", rend::fsr2_glue::d3d_vbv_alloc_clamped as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_vbv_alloc_clamped),
// REGISTRY: Replacement::verified_throws(0x1_4205_e050, "d3d::vbvAlloc_14205e050", rend::fsr2_glue::d3d_vbv_alloc as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_vbv_alloc),
// REGISTRY: Replacement::verified(0x1_4205_e2a0, "d3d::ibvPoolShutdown_14205e2a0", rend::fsr2_glue::d3d_ibv_pool_shutdown as *const (), rend::fsr2_glue::diff_d3d_ibv_pool_shutdown),
// REGISTRY: Replacement::verified_throws(0x1_4205_e410, "d3d::ibvPoolInit_14205e410", rend::fsr2_glue::d3d_ibv_pool_init as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_ibv_pool_init),
// REGISTRY: Replacement::verified_throws(0x1_4205_e440, "d3d::ibvAllocAndCopy_14205e440", rend::fsr2_glue::d3d_ibv_alloc_and_copy as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_ibv_alloc_and_copy),
// REGISTRY: Replacement::verified(0x1_4205_e6d0, "d3d::tlsRingUnmap_14205e6d0", rend::fsr2_glue::d3d_tls_ring_unmap as *const (), rend::fsr2_glue::diff_d3d_tls_ring_unmap),
// REGISTRY: Replacement::verified_throws(0x1_4205_e700, "d3d::tlsRingAlloc_14205e700", rend::fsr2_glue::d3d_tls_ring_alloc as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_tls_ring_alloc),
// REGISTRY: Replacement::verified(0x1_4205_ec50, "d3d::sbvPoolShutdown_14205ec50", rend::fsr2_glue::d3d_sbv_pool_shutdown as *const (), rend::fsr2_glue::diff_d3d_sbv_pool_shutdown),
// REGISTRY: Replacement::verified_throws(0x1_4205_edd0, "d3d::sbvPoolInit_14205edd0", rend::fsr2_glue::d3d_sbv_pool_init as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_sbv_pool_init),
// REGISTRY: Replacement::verified_throws(0x1_4205_eec0, "d3d::sbvAllocAndCopy_14205eec0", rend::fsr2_glue::d3d_sbv_alloc_and_copy as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_sbv_alloc_and_copy),
// REGISTRY: Replacement::verified(0x1_4205_f070, "d3d::sbvUnmap_14205f070", rend::fsr2_glue::d3d_sbv_unmap as *const (), rend::fsr2_glue::diff_d3d_sbv_unmap),
// REGISTRY: Replacement::verified_throws(0x1_4205_f0f0, "d3d::sbvAllocClamped_14205f0f0", rend::fsr2_glue::d3d_sbv_alloc_clamped as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_sbv_alloc_clamped),
// REGISTRY: Replacement::verified_throws(0x1_4205_f290, "d3d::sbvAlloc_14205f290", rend::fsr2_glue::d3d_sbv_alloc as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_sbv_alloc),
// REGISTRY: Replacement::verified(0x1_4206_0060, "d3d::cmdContextPoolShutdown_142060060", rend::fsr2_glue::d3d_cmd_context_pool_shutdown as *const (), rend::fsr2_glue::diff_d3d_cmd_context_pool_shutdown),
// REGISTRY: Replacement::verified(0x1_4206_02d0, "d3d::cmdContextPoolInit_1420602d0", rend::fsr2_glue::d3d_cmd_context_pool_init as *const (), rend::fsr2_glue::diff_d3d_cmd_context_pool_init),
// REGISTRY: Replacement::verified(0x1_4206_0310, "d3d::unregisterCmdListener_142060310", rend::fsr2_glue::d3d_unregister_cmd_listener as *const (), rend::fsr2_glue::diff_d3d_unregister_cmd_listener),
// REGISTRY: Replacement::verified(0x1_4206_0410, "d3d::registerCmdListener_142060410", rend::fsr2_glue::d3d_register_cmd_listener as *const (), rend::fsr2_glue::diff_d3d_register_cmd_listener),
// REGISTRY: Replacement::verified_throws(0x1_4206_04d0, "d3d::getOrCreateComputePso_1420604d0", rend::fsr2_glue::d3d_get_or_create_compute_pso as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_get_or_create_compute_pso),
// REGISTRY: Replacement::verified(0x1_4206_0720, "d3d::buildVertexShaderInputLayout_142060720", rend::fsr2_glue::d3d_build_vertex_shader_input_layout as *const (), rend::fsr2_glue::diff_d3d_build_vertex_shader_input_layout),


