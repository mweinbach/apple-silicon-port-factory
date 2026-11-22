//! Decompiled `rend` hot functions shard ZA19 (`0x142247710`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

struct ShaderParamSpec {
    guard_va: u64,
    id_va: u64,
    str_va: u64,
    str_len: u64,
    kind: u32,
    atexit_va: u64,
    inline_tls: bool,
}

const PARAMS_PRE: [ShaderParamSpec; 16] = [
    // 0: g_tJitterMap
    ShaderParamSpec {
        guard_va: 0x1_43cf_1814,
        id_va: 0x1_43cf_1670,
        str_va: 0x1_433f_1cb8,
        str_len: 0x0c,
        kind: 9,
        atexit_va: 0x1_4262_a9a0,
        inline_tls: true,
    },
    // 1: g_tGaussianNoiseMap
    ShaderParamSpec {
        guard_va: 0x1_43cf_16f8,
        id_va: 0x1_43cf_17b8,
        str_va: 0x1_433f_1ca0,
        str_len: 0x13,
        kind: 9,
        atexit_va: 0x1_4262_a990,
        inline_tls: true,
    },
    // 2: g_tUniformNoise64x64
    ShaderParamSpec {
        guard_va: 0x1_43cf_181c,
        id_va: 0x1_43cf_1770,
        str_va: 0x1_433f_1c48,
        str_len: 0x14,
        kind: 9,
        atexit_va: 0x1_4262_a980,
        inline_tls: true,
    },
    // 3: g_tInterleavedNoise
    ShaderParamSpec {
        guard_va: 0x1_43cf_1844,
        id_va: 0x1_43cf_16e8,
        str_va: 0x1_433f_1c30,
        str_len: 0x13,
        kind: 10,
        atexit_va: 0x1_4262_a970,
        inline_tls: true,
    },
    // 4: g_tInterleavedNoiseHQ
    ShaderParamSpec {
        guard_va: 0x1_43cf_1818,
        id_va: 0x1_43cf_1790,
        str_va: 0x1_433f_1c78,
        str_len: 0x15,
        kind: 10,
        atexit_va: 0x1_4262_a960,
        inline_tls: true,
    },
    // 5: g_tBlueNoiseSobol
    ShaderParamSpec {
        guard_va: 0x1_43cf_17a8,
        id_va: 0x1_43cf_1638,
        str_va: 0x1_433f_1c60,
        str_len: 0x11,
        kind: 10,
        atexit_va: 0x1_4262_a950,
        inline_tls: false,
    },
    // 6: g_tBlueNoiseScramblingTile
    ShaderParamSpec {
        guard_va: 0x1_43cf_1708,
        id_va: 0x1_43cf_1830,
        str_va: 0x1_433f_1be0,
        str_len: 0x1a,
        kind: 10,
        atexit_va: 0x1_4262_a940,
        inline_tls: false,
    },
    // 7: g_tBlueNoiseRankingTile
    ShaderParamSpec {
        guard_va: 0x1_43cf_1810,
        id_va: 0x1_43cf_1890,
        str_va: 0x1_433f_1bc8,
        str_len: 0x17,
        kind: 10,
        atexit_va: 0x1_4262_a930,
        inline_tls: false,
    },
    // 8: g_tSobol4Sequence
    ShaderParamSpec {
        guard_va: 0x1_43cf_1840,
        id_va: 0x1_43cf_17d8,
        str_va: 0x1_433f_1c18,
        str_len: 0x11,
        kind: 9,
        atexit_va: 0x1_4262_a920,
        inline_tls: false,
    },
    // 9: g_tHalton4Sequence
    ShaderParamSpec {
        guard_va: 0x1_43cf_17ac,
        id_va: 0x1_43cf_1718,
        str_va: 0x1_433f_1c00,
        str_len: 0x12,
        kind: 9,
        atexit_va: 0x1_4262_a910,
        inline_tls: false,
    },
    // 10: g_sbInterleavedSamples_16
    ShaderParamSpec {
        guard_va: 0x1_43cf_1920,
        id_va: 0x1_43cf_16d0,
        str_va: 0x1_433f_1b60,
        str_len: 0x19,
        kind: 10,
        atexit_va: 0x1_4262_a900,
        inline_tls: false,
    },
    // 11: g_sbInterleavedSamples_6
    ShaderParamSpec {
        guard_va: 0x1_43cf_15c0,
        id_va: 0x1_43cf_16b0,
        str_va: 0x1_433f_1b40,
        str_len: 0x18,
        kind: 10,
        atexit_va: 0x1_4262_a8f0,
        inline_tls: false,
    },
    // 12: g_sbInterleavedSamples_4
    ShaderParamSpec {
        guard_va: 0x1_43cf_15e8,
        id_va: 0x1_43cf_15f8,
        str_va: 0x1_433f_1ba8,
        str_len: 0x18,
        kind: 10,
        atexit_va: 0x1_4262_a8e0,
        inline_tls: false,
    },
    // 13: g_sbInterleavedSamples_16_temporal
    ShaderParamSpec {
        guard_va: 0x1_43cf_17f0,
        id_va: 0x1_43cf_15d8,
        str_va: 0x1_433f_1b80,
        str_len: 0x22,
        kind: 10,
        atexit_va: 0x1_4262_a8d0,
        inline_tls: false,
    },
    // 14: g_sbInterleavedSamples_6_temporal
    ShaderParamSpec {
        guard_va: 0x1_43cf_15cc,
        id_va: 0x1_43cf_1800,
        str_va: 0x1_433f_1ae0,
        str_len: 0x21,
        kind: 10,
        atexit_va: 0x1_4262_a8c0,
        inline_tls: false,
    },
    // 15: g_sbInterleavedSamples_4_temporal
    ShaderParamSpec {
        guard_va: 0x1_43cf_15c4,
        id_va: 0x1_43cf_1738,
        str_va: 0x1_433f_1ab8,
        str_len: 0x21,
        kind: 10,
        atexit_va: 0x1_4262_a8b0,
        inline_tls: false,
    },
];

const PARAMS_POST: [ShaderParamSpec; 7] = [
    // 0 (16): g_tBlueNoisesMSAA
    ShaderParamSpec {
        guard_va: 0x1_43cf_16c0,
        id_va: 0x1_43cf_1690,
        str_va: 0x1_433f_1b28,
        str_len: 0x11,
        kind: 9,
        atexit_va: 0x1_4262_a8a0,
        inline_tls: true,
    },
    // 1 (17): g_tBlueNoiseOutputDither
    ShaderParamSpec {
        guard_va: 0x1_43cf_17a0,
        id_va: 0x1_43cf_1908,
        str_va: 0x1_433f_1b08,
        str_len: 0x18,
        kind: 9,
        atexit_va: 0x1_4262_a890,
        inline_tls: true,
    },
    // 2 (18): g_tBlueNoiseShadowJitter
    ShaderParamSpec {
        guard_va: 0x1_43cf_18a0,
        id_va: 0x1_43cf_1750,
        str_va: 0x1_433f_1a60,
        str_len: 0x18,
        kind: 9,
        atexit_va: 0x1_4262_a880,
        inline_tls: true,
    },
    // 3 (19): g_tTemporalBlueNoiseRGBA
    ShaderParamSpec {
        guard_va: 0x1_43cf_1598,
        id_va: 0x1_43cf_1850,
        str_va: 0x1_433f_1a40,
        str_len: 0x18,
        kind: 9,
        atexit_va: 0x1_4262_a870,
        inline_tls: true,
    },
    // 4 (20): g_tRandomBlueNoiseRGBA
    ShaderParamSpec {
        guard_va: 0x1_43cf_17a4,
        id_va: 0x1_43cf_18c8,
        str_va: 0x1_433f_1aa0,
        str_len: 0x16,
        kind: 9,
        atexit_va: 0x1_4262_a860,
        inline_tls: false,
    },
    // 5 (21): g_tStaticBlueNoiseRGBA_0
    ShaderParamSpec {
        guard_va: 0x1_43cf_1820,
        id_va: 0x1_43cf_1650,
        str_va: 0x1_433f_1a80,
        str_len: 0x18,
        kind: 9,
        atexit_va: 0x1_4262_a850,
        inline_tls: false,
    },
    // 6 (22): g_tStaticBlueNoiseRGBA_1
    ShaderParamSpec {
        guard_va: 0x1_43cf_1608,
        id_va: 0x1_43cf_1618,
        str_va: 0x1_433f_1a08,
        str_len: 0x18,
        kind: 9,
        atexit_va: 0x1_4262_a840,
        inline_tls: false,
    },
];

const BBD_TO_CF1_MAP: [(u64, u64); 17] = [
    (0x1_43bb_d040, 0x1_43cf_15a0),
    (0x1_43bb_d048, 0x1_43cf_15a8),
    (0x1_43bb_d050, 0x1_43cf_15b0),
    (0x1_43bb_d058, 0x1_43cf_15b8),
    (0x1_43bb_d060, 0x1_43cf_1868),
    (0x1_43bb_d068, 0x1_43cf_1870),
    (0x1_43bb_d070, 0x1_43cf_1878),
    (0x1_43bb_d078, 0x1_43cf_1880),
    (0x1_43bb_d080, 0x1_43cf_1928),
    (0x1_43bb_d088, 0x1_43cf_1930),
    (0x1_43bb_d090, 0x1_43cf_1938),
    (0x1_43bb_d098, 0x1_43cf_1940),
    (0x1_43bb_d0a0, 0x1_43cf_1700),
    (0x1_43bb_d0a8, 0x1_43cf_18e0),
    (0x1_43bb_d0b0, 0x1_43cf_18e8),
    (0x1_43bb_d0b8, 0x1_43cf_18f0),
    (0x1_43bb_d0c0, 0x1_43cf_18f8),
];

#[inline(always)]
unsafe fn init_static_param(spec: &ShaderParamSpec) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let guard_ptr = exe::ptr::<i32>(spec.guard_va) as usize;
    if rd::<i32>(guard_ptr) > rd::<i32>(tls0 + 0x74) {
        exe::call_va!(0x1_4183_4a78, (guard_ptr));
        if rd::<i32>(guard_ptr) == -1 {
            let mut view: [u64; 2] = [exe::ptr::<u8>(spec.str_va) as usize as u64, spec.str_len];
            let id_ptr = exe::ptr::<u8>(spec.id_va) as usize;
            exe::call_va!(0x1_4220_ea70, (id_ptr, view.as_mut_ptr() as usize));
            wr::<u32>(id_ptr + 0x04, spec.kind);
            wr::<usize>(id_ptr - 0x08, id_ptr + 0x08);
            wr::<u64>(id_ptr + 0x08, 0);
            if spec.kind == 9 {
                wr::<u64>(id_ptr + 0x10, 0);
            }
            let atexit_fn = exe::ptr::<u8>(spec.atexit_va) as usize;
            let _ = exe::call_va!(0x1_4183_4d18, (atexit_fn) -> i32);
            exe::call_va!(0x1_4183_4a0c, (guard_ptr));
        }
    }
}

#[inline(always)]
unsafe fn resolve_tex_handle(mode: i32, tex_obj: usize, fallback_base: usize) -> u64 {
    match mode {
        0 => {
            if tex_obj != 0 {
                rd::<u64>(tex_obj + 0x78)
            } else {
                rd::<u64>(fallback_base + 0x88)
            }
        }
        5 => {
            if tex_obj != 0 {
                rd::<u64>(tex_obj + 0x90)
            } else {
                rd::<u64>(fallback_base + 0x90)
            }
        }
        _ => rd::<u64>(fallback_base + 0x88),
    }
}

#[inline(always)]
unsafe fn resolve_buf_handle(mode: i32, buf_obj: usize, fallback_base: usize) -> u64 {
    match mode {
        2 => {
            if buf_obj != 0 {
                let v = rd::<u64>(buf_obj + 0x18);
                if v != 0 {
                    return v;
                }
            }
            rd::<u64>(fallback_base + 0xa8)
        }
        3 => {
            if buf_obj != 0 {
                let v = rd::<u64>(buf_obj + 0x20);
                if v != 0 {
                    return v;
                }
            }
            rd::<u64>(fallback_base + 0x98)
        }
        4 => {
            if buf_obj != 0 {
                let v = rd::<u64>(buf_obj + 0x28);
                if v != 0 {
                    return v;
                }
            }
            rd::<u64>(fallback_base + 0x98)
        }
        6 => {
            if buf_obj != 0 {
                let v = rd::<u64>(buf_obj + 0x30);
                if v != 0 {
                    return v;
                }
            }
            rd::<u64>(fallback_base + 0xa0)
        }
        7 => {
            if buf_obj != 0 {
                let v = rd::<u64>(buf_obj + 0x38);
                if v != 0 {
                    return v;
                }
            }
            rd::<u64>(fallback_base + 0xa0)
        }
        _ => rd::<u64>(fallback_base + 0x98),
    }
}

#[inline(always)]
unsafe fn upload_tls_inline(id: isize, src_ptr: usize) {
    let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as isize;
    let entry = (tbl_base + id * 16) as usize;
    let copy_len = rd::<i32>(entry + 0x08) as isize as usize;
    let tls_off = rd::<i32>(entry);
    if tls_off >= 0 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        let dst = tls0
            .wrapping_add(0x63d0)
            .wrapping_add(tls_off as isize as usize);
        let _ = exe::call_va!(0x1_4183_acbc, (dst, src_ptr, copy_len) -> usize);
        let dirty_idx = rd::<i32>(entry + 0x0c) as isize;
        if dirty_idx != -1 {
            let cnt_addr = (tls0.wrapping_add(0x463e0) as isize + dirty_idx * 4) as usize;
            let next_cnt = rd::<i32>(cnt_addr).wrapping_add(1);
            wr::<i32>(cnt_addr, if next_cnt == -1 { 0 } else { next_cnt });
        }
    }
}

#[inline(always)]
unsafe fn upload_via_call(id: isize, src_ptr: usize) {
    let tbl_base = exe::ptr::<u8>(0x1_43a6_37b0) as isize;
    let entry = (tbl_base + id * 16) as usize;
    let copy_len = rd::<i32>(entry + 0x08) as i64 as u64;
    exe::call_va!(0x1_4209_a900, (id as i32 as u32, 0u32, copy_len, src_ptr));
}

#[inline(always)]
unsafe fn bind_shader_param(spec: &ShaderParamSpec, obj: usize) {
    let fallback_base = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let id_ptr = exe::ptr::<u8>(spec.id_va) as usize;
    let id = rd::<i32>(id_ptr) as isize;
    let mode_base = exe::ptr::<i32>(0x1_4385_8590) as isize;
    let mode = rd::<i32>((mode_base + id * 4) as usize);

    if spec.kind == 9 {
        let h1 = resolve_tex_handle(mode, obj, fallback_base);
        let mut pair: [u64; 2] = [h1, obj as u64];
        if spec.inline_tls {
            upload_tls_inline(id, pair.as_ptr() as usize);
        } else {
            upload_via_call(id, pair.as_mut_ptr() as usize);
        }
        wr::<u64>(id_ptr + 0x10, obj as u64);
        let h2 = resolve_tex_handle(mode, obj, fallback_base);
        wr::<u64>(id_ptr + 0x08, h2);
    } else {
        let h1 = resolve_buf_handle(mode, obj, fallback_base);
        let mut val: u64 = h1;
        if spec.inline_tls {
            upload_tls_inline(id, (&val as *const u64) as usize);
        } else {
            upload_via_call(id, (&mut val as *mut u64) as usize);
        }
        let h2 = resolve_buf_handle(mode, obj, fallback_base);
        wr::<u64>(id_ptr + 0x08, h2);
    }
}

/// `unknown_142247710`
/// Original: `0x142247710` (10744 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_142247710(param_1: usize) {
    exe::call_va!(0x1_4224_5590, (param_1));
    exe::call_va!(0x1_4224_a7d0, (0u32, 0u32));

    // 0..15: pre-table bindings from param_1
    for (idx, spec) in PARAMS_PRE.iter().enumerate() {
        init_static_param(spec);
        let obj = match idx {
            0 => rd::<usize>(rd::<usize>(param_1)),
            1 => rd::<usize>(rd::<usize>(param_1 + 0x08)),
            2 => rd::<usize>(rd::<usize>(param_1 + 0x10)),
            3 => rd::<usize>(param_1 + 0x18),
            4 => rd::<usize>(param_1 + 0x20),
            5 => rd::<usize>(param_1 + 0x28),
            6 => rd::<usize>(param_1 + 0x30),
            7 => rd::<usize>(param_1 + 0x38),
            8 => rd::<usize>(rd::<usize>(param_1 + 0x40)),
            9 => rd::<usize>(rd::<usize>(param_1 + 0x48)),
            10 => rd::<usize>(param_1 + 0x50),
            11 => rd::<usize>(param_1 + 0x70),
            12 => rd::<usize>(param_1 + 0x90),
            13 => {
                let t = rd::<i32>(exe::ptr::<i32>(0x1_43a0_014c) as usize) as isize;
                rd::<usize>((param_1 as isize + t * 8 + 0x50) as usize)
            }
            14 => {
                let t = rd::<i32>(exe::ptr::<i32>(0x1_43a0_014c) as usize) as isize;
                rd::<usize>((param_1 as isize + t * 8 + 0x70) as usize)
            }
            _ => {
                let t = rd::<i32>(exe::ptr::<i32>(0x1_43a0_014c) as usize) as isize;
                rd::<usize>((param_1 as isize + t * 8 + 0x90) as usize)
            }
        };
        bind_shader_param(spec, obj);
    }

    // Populate 17 blue-noise texture pointers from 0x143bbd040..0x143bbd0c0
    for &(src_va, dst_va) in &BBD_TO_CF1_MAP {
        let wrapper = rd::<usize>(exe::ptr::<usize>(src_va) as usize);
        let tex_obj = if wrapper != 0 {
            rd::<usize>(wrapper + 0x1a0)
        } else {
            0
        };
        wr::<usize>(exe::ptr::<usize>(dst_va) as usize, tex_obj);
    }

    // 16..22: post-table blue-noise bindings
    for (idx, spec) in PARAMS_POST.iter().enumerate() {
        init_static_param(spec);
        let obj = match idx {
            0 => {
                let t = rd::<i32>(exe::ptr::<i32>(0x1_43a0_014c) as usize) as isize;
                let base = exe::ptr::<usize>(0x1_43cf_15a0) as isize;
                rd::<usize>((base + t * 8) as usize)
            }
            1 => rd::<usize>(exe::ptr::<usize>(0x1_43cf_1700) as usize),
            2 => {
                let t = rd::<i32>(exe::ptr::<i32>(0x1_43a0_014c) as usize) as isize;
                let base = exe::ptr::<usize>(0x1_43cf_18e0) as isize;
                rd::<usize>((base + t * 8) as usize)
            }
            3 => {
                let t = rd::<i32>(exe::ptr::<i32>(0x1_43a0_014c) as usize)
                    .wrapping_add(0x11) as isize;
                let base = exe::ptr::<usize>(0x1_43bb_d040) as isize;
                let wrapper = rd::<usize>((base + t * 8) as usize);
                if wrapper != 0 {
                    rd::<usize>(wrapper + 0x1a0)
                } else {
                    0
                }
            }
            4 => {
                let raw = rd::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize) as u32;
                let mut rem = raw & 0x8000_0007;
                if (rem as i32) < 0 {
                    rem = (rem.wrapping_sub(1) | 0xffff_fff8).wrapping_add(1);
                }
                let base = exe::ptr::<usize>(0x1_43bb_d0c8) as isize;
                let wrapper = rd::<usize>((base + (rem as i32 as isize) * 8) as usize);
                if wrapper != 0 {
                    rd::<usize>(wrapper + 0x1a0)
                } else {
                    0
                }
            }
            5 => {
                let wrapper = rd::<usize>(exe::ptr::<usize>(0x1_43bb_d0c8) as usize);
                if wrapper != 0 {
                    rd::<usize>(wrapper + 0x1a0)
                } else {
                    0
                }
            }
            _ => {
                let wrapper = rd::<usize>(exe::ptr::<usize>(0x1_43bb_d0d0) as usize);
                if wrapper != 0 {
                    rd::<usize>(wrapper + 0x1a0)
                } else {
                    0
                }
            }
        };
        bind_shader_param(spec, obj);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142247710(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;
    static mut NEXT_ID: i32 = 0;
    static mut FAKE_TLS: [u8; 0x47000] = [0; 0x47000];
    static mut FCB8_OBJ: [u8; 0xc0] = [0; 0xc0];
    static mut PARAM1_ARR: [usize; 24] = [0; 24];
    static mut IND_PTRS: [usize; 5] = [0; 5];
    static mut TEX_OBJS: [[u8; 0xa0]; 16] = [[0; 0xa0]; 16];
    static mut BUF_OBJS: [[u8; 0x40]; 20] = [[0; 0x40]; 20];
    static mut BBD_WRAPPERS: [[u8; 0x1b0]; 26] = [[0; 0x1b0]; 26];

    #[inline(always)]
    unsafe fn mix(v: u64) {
        TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
    }

    unsafe extern "win64" fn mock_5590(p1: usize) {
        let base = core::ptr::addr_of!(PARAM1_ARR) as usize;
        mix(0x5590 ^ (p1.wrapping_sub(base) as u64));
    }

    unsafe extern "win64" fn mock_a7d0(a: u32, b: u32) {
        mix(0xa7d0 ^ (a as u64) ^ ((b as u64) << 32));
    }

    unsafe extern "win64" fn mock_init_hdr(guard_ptr: usize) {
        wr::<i32>(guard_ptr, -1);
    }

    unsafe extern "win64" fn mock_init_ftr(guard_ptr: usize) {
        wr::<i32>(guard_ptr, -2);
    }

    unsafe extern "win64" fn mock_ea70(id_ptr: usize, view_ptr: usize) {
        let str_ptr = rd::<usize>(view_ptr);
        let str_len = rd::<u64>(view_ptr + 0x08);
        let first_byte = if str_ptr != 0 && str_len > 0 {
            rd::<u8>(str_ptr) as u64
        } else {
            0
        };
        let id = NEXT_ID;
        NEXT_ID += 1;
        wr::<i32>(id_ptr, id);
        mix(0xea70 ^ (id as u64) ^ (str_len << 8) ^ (first_byte << 16));
    }

    unsafe extern "win64" fn mock_atexit(fn_ptr: usize) -> i32 {
        let exe_base = exe::ptr::<u8>(0x1_4000_0000) as usize;
        mix(0xd180 ^ (fn_ptr.wrapping_sub(exe_base) as u64));
        0
    }

    unsafe extern "win64" fn mock_a900(id: u32, edx: u32, sz: u64, src_ptr: usize) {
        mix(0xa900 ^ (id as u64) ^ ((edx as u64) << 16) ^ (sz << 32));
        let n = (sz as usize).min(16);
        for i in 0..n {
            mix(rd::<u8>(src_ptr + i) as u64);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_5590, mock_5590 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4224_a7d0, mock_a7d0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4183_4a78, mock_init_hdr as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4183_4a0c, mock_init_ftr as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4183_4d18, mock_atexit as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4209_a900, mock_a900 as *const ())?;

    let snap_cf1 =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_1590) as *mut u8, 0x3c0);
    let snap_mode =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_8590) as *mut u8, 0x80);
    let snap_cb =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a6_37b0) as *mut u8, 0x200);
    let snap_fcb8 =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fcb8) as *mut u8, 0x08);
    let snap_idx =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_0148) as *mut u8, 0x40);
    let snap_bbd =
        crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43bb_d040) as *mut u8, 0xd0);

    let saved_tls0 = rd::<usize>(exe::tls_pointer());

    let run = |f: F,
               mode_bias: i32,
               init_guards: bool,
               null_every_other: bool,
               frame_idx: i32|
     -> (u64, Vec<u8>, Vec<u8>, Vec<u8>) {
        TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        NEXT_ID = 0;

        let tp = core::ptr::addr_of_mut!(FAKE_TLS) as usize;
        unsafe {
            core::ptr::write_bytes(tp as *mut u8, 0, 0x47000);
        }
        wr::<i32>(tp + 0x74, 0);
        wr::<usize>(exe::tls_pointer(), tp);

        let cf1_base = exe::ptr::<u8>(0x1_43cf_1590) as usize;
        unsafe {
            core::ptr::write_bytes(cf1_base as *mut u8, 0, 0x3c0);
        }

        for (i, spec) in PARAMS_PRE.iter().chain(PARAMS_POST.iter()).enumerate() {
            let g_ptr = exe::ptr::<i32>(spec.guard_va) as usize;
            let id_ptr = exe::ptr::<i32>(spec.id_va) as usize;
            if init_guards {
                wr::<i32>(g_ptr, 1); // > tls epoch (0), triggers _Init_thread_header
            } else {
                wr::<i32>(g_ptr, -2); // <= tls epoch (0), already initialized
                wr::<i32>(id_ptr, i as i32);
            }
        }

        let mode_base = exe::ptr::<i32>(0x1_4385_8590) as usize;
        let tex_modes = [0, 5, 1];
        let buf_modes = [2, 3, 4, 6, 7, 1];
        for (i, spec) in PARAMS_PRE.iter().chain(PARAMS_POST.iter()).enumerate() {
            let m = if spec.kind == 9 {
                tex_modes[((i as i32 + mode_bias).rem_euclid(3)) as usize]
            } else {
                buf_modes[((i as i32 + mode_bias).rem_euclid(6)) as usize]
            };
            wr::<i32>(mode_base + i * 4, m);
        }

        let cb_base = exe::ptr::<u8>(0x1_43a6_37b0) as usize;
        for (i, spec) in PARAMS_PRE.iter().chain(PARAMS_POST.iter()).enumerate() {
            let sz: i32 = if spec.kind == 9 { 16 } else { 8 };
            let off: i32 = if i == 2 { -1 } else { (i * 0x10) as i32 };
            let dirty: i32 = if i == 1 { -1 } else { (i & 7) as i32 };
            wr::<i32>(cb_base + i * 16, off);
            wr::<i32>(cb_base + i * 16 + 0x08, sz);
            wr::<i32>(cb_base + i * 16 + 0x0c, dirty);
        }
        // Seed one dirty counter to -2 so +1 reaches -1 and wraps to 0
        wr::<i32>(tp + 0x463e0, -2);

        let fcb8_p = core::ptr::addr_of_mut!(FCB8_OBJ) as usize;
        wr::<u64>(fcb8_p + 0x88, 0xfcb8_0088_1111_2222);
        wr::<u64>(fcb8_p + 0x90, 0xfcb8_0090_3333_4444);
        wr::<u64>(fcb8_p + 0x98, 0xfcb8_0098_5555_6666);
        wr::<u64>(fcb8_p + 0xa0, 0xfcb8_00a0_7777_8888);
        wr::<u64>(fcb8_p + 0xa8, 0xfcb8_00a8_9999_aaaa);
        wr::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize, fcb8_p);

        wr::<i32>(exe::ptr::<i32>(0x1_43a0_014c) as usize, mode_bias & 3);
        wr::<i32>(exe::ptr::<i32>(0x1_43a0_0180) as usize, frame_idx);

        for i in 0..16usize {
            let tp_obj = core::ptr::addr_of_mut!(TEX_OBJS[i]) as usize;
            wr::<u64>(tp_obj + 0x78, 0x7800_0000_0000_0000 | (i as u64 + 1));
            wr::<u64>(tp_obj + 0x90, 0x9000_0000_0000_0000 | (i as u64 + 1));
        }

        for i in 0..20usize {
            let bp_obj = core::ptr::addr_of_mut!(BUF_OBJS[i]) as usize;
            let z = null_every_other && (i & 1) == 1;
            wr::<u64>(bp_obj + 0x18, if z { 0 } else { 0x1800_0000 | (i as u64 + 1) });
            wr::<u64>(bp_obj + 0x20, if z { 0 } else { 0x2000_0000 | (i as u64 + 1) });
            wr::<u64>(bp_obj + 0x28, if z { 0 } else { 0x2800_0000 | (i as u64 + 1) });
            wr::<u64>(bp_obj + 0x30, if z { 0 } else { 0x3000_0000 | (i as u64 + 1) });
            wr::<u64>(bp_obj + 0x38, if z { 0 } else { 0x3800_0000 | (i as u64 + 1) });
        }

        // Set up param_1 slots
        let ind_tex_slots = [0usize, 1, 2, 8, 9];
        for (k, &slot) in ind_tex_slots.iter().enumerate() {
            let tex_p = if null_every_other && (k & 1) == 1 {
                0
            } else {
                core::ptr::addr_of_mut!(TEX_OBJS[k]) as usize
            };
            IND_PTRS[k] = tex_p;
            PARAM1_ARR[slot] = core::ptr::addr_of_mut!(IND_PTRS[k]) as usize;
        }
        for slot in [3usize, 4, 5, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21] {
            let buf_p = if null_every_other && (slot & 1) == 0 {
                0
            } else {
                core::ptr::addr_of_mut!(BUF_OBJS[slot - 3]) as usize
            };
            PARAM1_ARR[slot] = buf_p;
        }

        let bbd_base = exe::ptr::<usize>(0x1_43bb_d040) as usize;
        for i in 0..26usize {
            let wp = core::ptr::addr_of_mut!(BBD_WRAPPERS[i]) as usize;
            let tex_p = core::ptr::addr_of_mut!(TEX_OBJS[(i + 5) & 15]) as usize;
            wr::<usize>(wp + 0x1a0, tex_p);
            let wrapper_ptr = if null_every_other && (i & 1) == 1 {
                0
            } else {
                wp
            };
            wr::<usize>(bbd_base + i * 8, wrapper_ptr);
        }

        f(core::ptr::addr_of_mut!(PARAM1_ARR) as usize);

        let mut cf1_out = vec![0u8; 0x3c0];
        let mut tls_cb_out = vec![0u8; 0x180];
        let mut tls_dirty_out = vec![0u8; 0x20];
        unsafe {
            core::ptr::copy_nonoverlapping(cf1_base as *const u8, cf1_out.as_mut_ptr(), 0x3c0);
            core::ptr::copy_nonoverlapping(
                (tp + 0x63d0) as *const u8,
                tls_cb_out.as_mut_ptr(),
                0x180,
            );
            core::ptr::copy_nonoverlapping(
                (tp + 0x463e0) as *const u8,
                tls_dirty_out.as_mut_ptr(),
                0x20,
            );
        }
        (TRACE_HASH, cf1_out, tls_cb_out, tls_dirty_out)
    };

    let cases = [
        (0i32, true, false, 3i32),
        (1i32, false, false, -5i32),
        (2i32, false, true, -8i32),
        (3i32, true, true, 14i32),
        (4i32, false, false, -1i32),
        (5i32, false, true, 0i32),
    ];

    for (idx, &(mode_bias, init_guards, null_every_other, frame_idx)) in cases.iter().enumerate() {
        let o = run(f_o, mode_bias, init_guards, null_every_other, frame_idx);
        let n = run(f_n, mode_bias, init_guards, null_every_other, frame_idx);
        if o != n {
            wr::<usize>(exe::tls_pointer(), saved_tls0);
            snap_bbd.restore();
            snap_idx.restore();
            snap_fcb8.restore();
            snap_cb.restore();
            snap_mode.restore();
            snap_cf1.restore();
        }
        crate::diff_eq!(o, n, "142247710 case={}", idx);
    }

    wr::<usize>(exe::tls_pointer(), saved_tls0);
    snap_bbd.restore();
    snap_idx.restore();
    snap_fcb8.restore();
    snap_cb.restore();
    snap_mode.restore();
    snap_cf1.restore();
    Ok(cases.len() as u32)
}
