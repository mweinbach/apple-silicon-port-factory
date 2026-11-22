//! Decompiled hot functions shard ZA16 (`0x142202830`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn init_cvar_scalar(
    tls0: usize,
    guard_va: u64,
    id_va: u64,
    type_va: u64,
    ptr_va: u64,
    val_va: u64,
    name_va: u64,
    name_len: u64,
    cvar_type: u32,
    atexit_va: u64,
) {
    let guard_ptr = exe::ptr::<i32>(guard_va) as usize;
    if rd::<i32>(guard_ptr) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            let mut sv: [u64; 2] = [exe::ptr::<u8>(name_va) as u64, name_len];
            exe::call_va!(
                0x1_4220_ea70,
                (exe::ptr::<u8>(id_va) as usize, sv.as_mut_ptr() as usize)
            );
            wr::<u32>(exe::ptr::<u32>(type_va) as usize, cvar_type);
            wr::<usize>(
                exe::ptr::<usize>(ptr_va) as usize,
                exe::ptr::<u8>(val_va) as usize,
            );
            exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(atexit_va) as usize));
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

#[inline(always)]
unsafe fn write_tls_cvar_u32(tls0: usize, tbl_base: usize, id: isize, val_bits: u32) {
    let entry = tbl_base.wrapping_add((id as usize).wrapping_mul(16));
    let off = rd::<i32>(entry) as isize;
    if off >= 0 {
        let sz = rd::<i32>(entry + 0x08) as isize as usize;
        let dst = tls0.wrapping_add(0x63d0).wrapping_add(off as usize);
        let local_val: [u32; 2] = [val_bits, 0];
        exe::call_va!(
            0x1_4183_acbc,
            (dst, local_val.as_ptr() as usize, sz)
        );
        let cbuf = rd::<i32>(entry + 0x0c) as isize;
        if cbuf != -1 {
            let cnt_addr = tls0
                .wrapping_add(0x463e0)
                .wrapping_add((cbuf as usize).wrapping_mul(4));
            let next = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, if next == -1 { 0 } else { next });
        }
    }
}

/// `unknown_142202830`
/// Original: `0x142202830` (894 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_142202830(
    _param_1: usize,
    param_2: u32,
    _param_3: usize,
    _param_4_r9: usize,
) {
    let [_, _, _, xmm3] = crate::simd_ops::take_xmm_args();
    let xmm6_in = xmm3 as u32;
    let dl = param_2 & 0xff;
    let c_cab = rd::<u8>(exe::ptr::<u8>(0x1_4382_3cab) as usize);
    let c_caa = rd::<u8>(exe::ptr::<u8>(0x1_4382_3caa) as usize);
    let tls0 = rd::<usize>(exe::tls_pointer());
    let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;

    // 1. g_fPaperWhite
    init_cvar_scalar(
        tls0,
        0x1_43cf_0028,
        0x1_43ce_fe30,
        0x1_43ce_fe34,
        0x1_43ce_fe28,
        0x1_43ce_fe38,
        0x1_433e_f8e8,
        0x0d,
        0,
        0x1_4262_7830,
    );
    let paper_white_bits = if dl == 0 {
        rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize)
    } else {
        xmm6_in
    };
    let id1 = rd::<i32>(exe::ptr::<i32>(0x1_43ce_fe30) as usize) as isize;
    write_tls_cvar_u32(tls0, tbl_base, id1, paper_white_bits);
    wr::<u32>(exe::ptr::<u32>(0x1_43ce_fe38) as usize, paper_white_bits);

    // 2. g_bApplyVignette
    init_cvar_scalar(
        tls0,
        0x1_43cf_00e8,
        0x1_43ce_ffd8,
        0x1_43ce_ffdc,
        0x1_43ce_ffd0,
        0x1_43ce_ffe0,
        0x1_433e_f938,
        0x10,
        0x0c,
        0x1_4262_7820,
    );
    let apply_vignette = u32::from(c_caa != 0);
    let id2 = rd::<i32>(exe::ptr::<i32>(0x1_43ce_ffd8) as usize) as isize;
    write_tls_cvar_u32(tls0, tbl_base, id2, apply_vignette);
    wr::<u32>(exe::ptr::<u32>(0x1_43ce_ffe0) as usize, apply_vignette);

    // 3. g_bApplyFilmGrain
    init_cvar_scalar(
        tls0,
        0x1_43ce_fea8,
        0x1_43ce_ff08,
        0x1_43ce_ff0c,
        0x1_43ce_ff00,
        0x1_43ce_ff10,
        0x1_433e_f950,
        0x11,
        0x0c,
        0x1_4262_7810,
    );
    let apply_film_grain = u32::from(c_cab != 0);
    let id3 = rd::<i32>(exe::ptr::<i32>(0x1_43ce_ff08) as usize) as isize;
    write_tls_cvar_u32(tls0, tbl_base, id3, apply_film_grain);
    wr::<u32>(exe::ptr::<u32>(0x1_43ce_ff10) as usize, apply_film_grain);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142202830(orig: usize, new: usize) -> Result<u32, String> {
    static mut INIT_TRACE: u64 = 0;

    unsafe extern "win64" fn mock_4a78(guard: usize) {
        wr::<i32>(guard, -1);
    }
    unsafe extern "win64" fn mock_ea70(id_ptr: usize, sv_ptr: usize) {
        let len = rd::<u64>(sv_ptr + 0x08);
        INIT_TRACE = INIT_TRACE.wrapping_mul(31).wrapping_add(len);
        let id = match len {
            0x0d => 0i32,
            0x10 => 1i32,
            _ => 2i32,
        };
        wr::<i32>(id_ptr, id);
    }
    unsafe extern "win64" fn mock_4d18(_atexit_fn: usize) {
        INIT_TRACE = INIT_TRACE.wrapping_add(0x100);
    }
    unsafe extern "win64" fn mock_4a0c(guard: usize) {
        wr::<i32>(guard, i32::MIN);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4183_4a78, mock_4a78 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4183_4d18, mock_4d18 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4183_4a0c, mock_4a0c as *const ())?;

    let snap_cvar_area =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43ce_fe20) as *mut u8, 0x2e0);
    let snap_tbl =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a6_37b0) as *mut u8, 0x30);
    let snap_flags =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4382_3caa) as *mut u8, 2);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut fake_tls = vec![0u8; 0x46500];
    let tp = fake_tls.as_mut_ptr() as usize;

    let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;

    let run = |fn_addr: usize,
               mode: u8,
               param_2: u32,
               caa: u8,
               cab: u8,
               xmm3_val: f32|
     -> (u64, [u32; 6], [u32; 3], [i32; 3], [bool; 3]) {
        INIT_TRACE = 0;

        // Entry 0: offset 0, size 4, cbuf 0
        wr::<i32>(tbl_base, if mode == 2 { -1 } else { 0 });
        wr::<i32>(tbl_base + 0x04, 0);
        wr::<i32>(tbl_base + 0x08, 4);
        wr::<i32>(tbl_base + 0x0c, 0);

        // Entry 1: offset 16, size 4, cbuf 1
        wr::<i32>(tbl_base + 0x10, 16);
        wr::<i32>(tbl_base + 0x14, 0);
        wr::<i32>(tbl_base + 0x18, 4);
        wr::<i32>(tbl_base + 0x1c, 1);

        // Entry 2: offset 32, size 4, cbuf (-1 on mode 1, else 2)
        wr::<i32>(tbl_base + 0x20, 32);
        wr::<i32>(tbl_base + 0x24, 0);
        wr::<i32>(tbl_base + 0x28, 4);
        wr::<i32>(tbl_base + 0x2c, if mode == 1 { -1 } else { 2 });

        let guard_init = if mode == 3 { 10i32 } else { i32::MIN };
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_0028) as usize, guard_init);
        wr::<i32>(exe::ptr::<i32>(0x1_43cf_00e8) as usize, guard_init);
        wr::<i32>(exe::ptr::<i32>(0x1_43ce_fea8) as usize, guard_init);

        wr::<i32>(exe::ptr::<i32>(0x1_43ce_fe30) as usize, 0);
        wr::<i32>(exe::ptr::<i32>(0x1_43ce_ffd8) as usize, 1);
        wr::<i32>(exe::ptr::<i32>(0x1_43ce_ff08) as usize, 2);

        wr::<u32>(exe::ptr::<u32>(0x1_43ce_fe34) as usize, 0x99);
        wr::<u32>(exe::ptr::<u32>(0x1_43ce_ffdc) as usize, 0x99);
        wr::<u32>(exe::ptr::<u32>(0x1_43ce_ff0c) as usize, 0x99);

        wr::<usize>(exe::ptr::<usize>(0x1_43ce_fe28) as usize, 0);
        wr::<usize>(exe::ptr::<usize>(0x1_43ce_ffd0) as usize, 0);
        wr::<usize>(exe::ptr::<usize>(0x1_43ce_ff00) as usize, 0);

        wr::<u32>(exe::ptr::<u32>(0x1_43ce_fe38) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_43ce_ffe0) as usize, 0);
        wr::<u32>(exe::ptr::<u32>(0x1_43ce_ff10) as usize, 0);

        wr::<u8>(exe::ptr::<u8>(0x1_4382_3caa) as usize, caa);
        wr::<u8>(exe::ptr::<u8>(0x1_4382_3cab) as usize, cab);

        wr::<i32>(tp + 0x74, 0);
        for off in [0usize, 16, 32] {
            wr::<u32>(tp + 0x63d0 + off, 0xdead_beef);
        }
        wr::<i32>(tp + 0x463e0, 5);
        wr::<i32>(tp + 0x463e0 + 4, -2); // increments to -1 -> wraps to 0
        wr::<i32>(tp + 0x463e0 + 8, -2); // increments to -1 -> wraps to 0

        wr::<usize>(tls_slot, tp);
        let _ = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            0,
            param_2 as u64,
            0,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, xmm3_val.to_bits() as u128],
        );
        wr::<usize>(tls_slot, saved_tls0);

        let globals = [
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_fe38) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_ffe0) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_ff10) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_fe34) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_ffdc) as usize),
            rd::<u32>(exe::ptr::<u32>(0x1_43ce_ff0c) as usize),
        ];
        let tls_vals = [
            rd::<u32>(tp + 0x63d0),
            rd::<u32>(tp + 0x63d0 + 16),
            rd::<u32>(tp + 0x63d0 + 32),
        ];
        let cnts = [
            rd::<i32>(tp + 0x463e0),
            rd::<i32>(tp + 0x463e0 + 4),
            rd::<i32>(tp + 0x463e0 + 8),
        ];
        let ptrs_ok = [
            rd::<usize>(exe::ptr::<usize>(0x1_43ce_fe28) as usize)
                == exe::ptr::<u8>(0x1_43ce_fe38) as usize,
            rd::<usize>(exe::ptr::<usize>(0x1_43ce_ffd0) as usize)
                == exe::ptr::<u8>(0x1_43ce_ffe0) as usize,
            rd::<usize>(exe::ptr::<usize>(0x1_43ce_ff00) as usize)
                == exe::ptr::<u8>(0x1_43ce_ff10) as usize,
        ];
        (INIT_TRACE, globals, tls_vals, cnts, ptrs_ok)
    };

    let res = (|| -> Result<u32, String> {
        let cases: [(u8, u32, u8, u8, f32); 4] = [
            (0, 0, 0, 1, 2.5),
            (1, 1, 3, 0, 4.25),
            (2, 0x100, 1, 1, 1.75),
            (3, 1, 0, 2, 8.0),
        ];
        for (idx, &(mode, p2, caa, cab, x3)) in cases.iter().enumerate() {
            let o = run(orig, mode, p2, caa, cab, x3);
            let n = run(new, mode, p2, caa, cab, x3);
            crate::diff_eq!(o, n, "142202830 case={}", idx);
        }
        Ok(4)
    })();

    wr::<usize>(tls_slot, saved_tls0);
    snap_flags.restore();
    snap_tbl.restore();
    snap_cvar_area.restore();
    res
}
