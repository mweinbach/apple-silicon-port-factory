//! Render-thread pass-recording, culling, light-grid and dirty-property hot helpers.
//!
//! Hand-decompiled replacements for hot `Throws::No` leaves and `Throws::Discard` wrappers
//! on the render-thread critical path (`perf/cpu-frame-critical-path`):
//!
//! | VA | Role | Throws |
//! |---|---|---|
//! | `0x140274e20` | `vec4_mul_mat4_inplace(v, m)` (`v = v * M`, row-vector `float4 * float4x4`) | `No` |
//! | `0x1403b3de0` | `mat4_d2f(dst, src)` (`double[16] -> float[16]`, returns `dst`) | `No` |
//! | `0x1412ff130` | `pass_data_find_cstr(pass, cstr)` (FNV-1a of C string in `pass+0x208[64]`) | `No` |
//! | `0x14220e860` | `shader_variant_bsearch(table, mask)` (`(mask | +0xb8) & ~+0xbc` in `table[i]+0x90`) | `No` |
//! | `0x14220e960` | `hash_string_view_flagged(out, view, flag)` (`{fnv1a64(view), flag}`) | `No` |
//! | `0x14222e1d0` | `skinned_tree_has_active_node(tree)` (RB-tree scan for `+0x548/+0x5b0/+0x5cc > 0`) | `No` |
//! | `0x14224d860` | `bind_mesh_geometry_streams_a(geom)` (`tls+0x467e0` -> `0x14206f680`) | `Discard` |
//! | `0x14224d920` | `bind_mesh_geometry_streams_b(geom)` (`tls+0x467e0` -> `0x14206f680`) | `Discard` |
//! | `0x142263e80` | `renderWorldInternal::<lambda_2>::<lambda_1>::vfunc_1` (pass-1 dirty bounds job) | `No` |
//! | `0x142264530` | `renderWorldInternal::<lambda_6>::?bref1::<lambda_1>::vfunc_1` (pass-5 upload job) | `Discard` |
//! | `0x1422645d0` | `renderWorldInternal::<lambda_6>::<lambda_1>::vfunc_1` (pass-5 dirty prop job) | `Discard` |
//! | `0x142326d00` | `eval_draw_pass_feature_flags(state, ptr)` (pass feature flags `+0xe0..+0xe6`) | `No` |
//! | `0x142326e60` | `find_draw_pass_bucket_nocase(table, view)` (0x38-stride bucket lookup) | `No` |
//! | `0x142398a00` | `find_named_entry_nocase(container, view)` (`*ptr` array, `InplaceString` at `+0x68`) | `No` |
//! | `0x1423c3030` | `cmd_node_key_less(a, b)` (`a->key (+8) < b->key (+8)`) | `No` |
//! | `0x1423fd3c0` | `frustum_from_view_proj(view, frustum)` (`double[16]` @ `view+0x1d0` -> 6 planes) | `No` |

use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::exe::{self, rd, wr};

/// Case-folding table in `.rdata` used by Remedy's case-insensitive string comparisons.
const CASE_FOLD_TABLE_VA: u64 = 0x1_426d_01e0;

// ---------------------------------------------------------------------------
// Exact x86 SSE/AVX scalar & packed f32 helpers (preserves NaN payload & sign)
// ---------------------------------------------------------------------------

#[inline(always)]
fn x86_nan32(a: f32, b: f32, r: f32) -> f32 {
    if !r.is_nan() {
        r
    } else if a.is_nan() {
        f32::from_bits(a.to_bits() | 0x0040_0000)
    } else if b.is_nan() {
        f32::from_bits(b.to_bits() | 0x0040_0000)
    } else {
        f32::from_bits(0xffc0_0000)
    }
}

#[inline(always)]
fn sse_add(a: f32, b: f32) -> f32 {
    x86_nan32(a, b, a + b)
}

#[inline(always)]
fn sse_sub(a: f32, b: f32) -> f32 {
    x86_nan32(a, b, a - b)
}

#[inline(always)]
fn sse_mul(a: f32, b: f32) -> f32 {
    x86_nan32(a, b, a * b)
}

#[inline(always)]
fn sse_cvt_f64_f32(x: f64) -> f32 {
    if x.is_nan() {
        let bits = x.to_bits();
        let sign = ((bits >> 63) as u32) << 31;
        let payload = ((bits >> 29) as u32) & 0x003f_ffff;
        f32::from_bits(sign | 0x7fc0_0000 | payload)
    } else {
        x as f32
    }
}

#[inline(always)]
fn sse_rsqrt_lane(x: f32) -> f32 {
    let s = x86_nan32(x, x, x.sqrt());
    x86_nan32(1.0, s, 1.0f32 / s)
}

/// Extract `(data_ptr, len)` from an `r::InplaceString` header at `s`.
#[inline(always)]
unsafe fn inplace_str_ptr_len(s: usize) -> (usize, usize) {
    let b3 = rd::<u8>(s + 0x03) as u32;
    let inline_cap = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cap_bits = (rd::<u32>(s) & 0x00ff_ffff) << 3;
    let ptr = if cap_bits > inline_cap {
        rd::<usize>(s + 0x08)
    } else {
        s + 0x08
    };
    let len = rd::<u32>(s + 0x04) as usize;
    (ptr, len)
}

#[inline(always)]
unsafe fn eq_nocase_raw(p1: usize, p2: usize, len: usize, fold: usize) -> bool {
    for i in 0..len {
        let c1 = rd::<u8>(p1.wrapping_add(i)) as usize;
        let c2 = rd::<u8>(p2.wrapping_add(i)) as usize;
        if rd::<i8>(fold + c1) != rd::<i8>(fold + c2) {
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// 1. 0x140274e20 — vec4_mul_mat4_inplace
// ---------------------------------------------------------------------------

/// `vec4_mul_mat4_inplace(vec, mat)` — row-vector `float[4]` multiplied in-place by `float[16]`.
///
/// Original: `0x140274e20` (235 bytes, leaf, `Throws::No`). Exact SSE `vmulss`/`vaddss`
/// association `(((v0 * m[0,c] + v1 * m[1,c]) + v2 * m[2,c]) + v3 * m[3,c])`; returns `vec`
/// in `rax` (ignored at call sites per `ret_liveness.tsv`).
pub unsafe extern "win64" fn vec4_mul_mat4_inplace(vec: usize, mat: usize) -> usize {
    let v0 = rd::<f32>(vec);
    let v1 = rd::<f32>(vec + 0x04);
    let v2 = rd::<f32>(vec + 0x08);
    let v3 = rd::<f32>(vec + 0x0c);

    let r1 = sse_add(
        sse_add(
            sse_add(sse_mul(v0, rd::<f32>(mat + 0x04)), sse_mul(v1, rd::<f32>(mat + 0x14))),
            sse_mul(v2, rd::<f32>(mat + 0x24)),
        ),
        sse_mul(v3, rd::<f32>(mat + 0x34)),
    );
    let r2 = sse_add(
        sse_add(
            sse_add(sse_mul(v0, rd::<f32>(mat + 0x08)), sse_mul(v1, rd::<f32>(mat + 0x18))),
            sse_mul(v2, rd::<f32>(mat + 0x28)),
        ),
        sse_mul(v3, rd::<f32>(mat + 0x38)),
    );
    let r3 = sse_add(
        sse_add(
            sse_add(sse_mul(v0, rd::<f32>(mat + 0x0c)), sse_mul(v1, rd::<f32>(mat + 0x1c))),
            sse_mul(v2, rd::<f32>(mat + 0x2c)),
        ),
        sse_mul(v3, rd::<f32>(mat + 0x3c)),
    );
    // Note: column 0 computes `v1 * m[1,0] + v0 * m[0,0]` (`vaddss xmm2, xmm1, xmm0` @ 0x140274ecc).
    let r0 = sse_add(
        sse_add(
            sse_add(sse_mul(v1, rd::<f32>(mat + 0x10)), sse_mul(v0, rd::<f32>(mat + 0x00))),
            sse_mul(v2, rd::<f32>(mat + 0x20)),
        ),
        sse_mul(v3, rd::<f32>(mat + 0x30)),
    );

    wr::<f32>(vec + 0x04, r1);
    wr::<f32>(vec, r0);
    wr::<f32>(vec + 0x08, r2);
    wr::<f32>(vec + 0x0c, r3);
    vec
}

// ---------------------------------------------------------------------------
// 2. 0x1403b3de0 — mat4_d2f
// ---------------------------------------------------------------------------

/// `mat4_d2f(dst, src)` — converts 16 `f64` matrix elements at `src` into 16 `f32`s at `dst`
/// (loads all 16 `f64`s first so `dst == src` aliasing is safe), returning `dst`.
///
/// Original: `0x1403b3de0` (368 bytes, leaf, `Throws::No`, 67 callers incl. `light_grid_update`,
/// `0x14228b130`, `0x14227bd90`).
pub unsafe extern "win64" fn mat4_d2f(dst: usize, src: usize) -> usize {
    let mut s = [0.0f64; 16];
    for (i, slot) in s.iter_mut().enumerate() {
        *slot = rd::<f64>(src + i * 8);
    }
    for (i, &val) in s.iter().enumerate() {
        wr::<f32>(dst + i * 4, sse_cvt_f64_f32(val));
    }
    dst
}

// ---------------------------------------------------------------------------
// 3. 0x1412ff130 — pass_data_find_cstr
// ---------------------------------------------------------------------------

/// `pass_data_find_cstr(pass, cstr)` — FNV-1a 64-bit hash (sign-extended `i8` bytes) of NUL-
/// terminated `cstr`, looked up in the 64-slot key array at `pass + 0x208`; returns
/// `[pass + 8 + i*8]` on the first non-zero matching key, else `0`.
///
/// Original: `0x1412ff130` (115 bytes, leaf, `Throws::No`, 90 call sites across render passes
/// incl. `0x142272de0` and `0x14227bd90`).
pub unsafe extern "win64" fn pass_data_find_cstr(pass: usize, cstr: usize) -> u64 {
    let mut len = 0usize;
    while rd::<u8>(cstr.wrapping_add(len)) != 0 {
        len += 1;
    }
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for i in 0..len {
        let b = rd::<i8>(cstr.wrapping_add(i)) as i64 as u64;
        hash = (hash ^ b).wrapping_mul(0x0100_0000_01b3);
    }
    for i in 0..0x40usize {
        let k = rd::<u64>(pass.wrapping_add(0x208 + i * 8));
        if k != 0 && k == hash {
            return rd::<u64>(pass.wrapping_add(0x08 + i * 8));
        }
    }
    0
}

// ---------------------------------------------------------------------------
// 4. 0x14220e860 — shader_variant_bsearch
// ---------------------------------------------------------------------------

/// `shader_variant_bsearch(table, mask)` — computes `(mask | table->or_mask) & !table->andn_mask`
/// (`+0xb8`, `+0xbc`) and binary-searches `table->entries[0..count]` (`+0x00`, `i32` count `+0x08`)
/// by `u32` key at `entry + 0x90`. Returns the matching entry pointer or `0`.
///
/// Original: `0x14220e860` (77 bytes, leaf, `Throws::No`, 37 callers / 75 sites).
/// Uses 32-bit signed `lo`/`hi` with `(lo.wrapping_add(hi)) >> 1` (`lea eax, [rdx+r8]; sar eax, 1`).
pub unsafe extern "win64" fn shader_variant_bsearch(table: usize, mask: u32) -> usize {
    let eff = (mask | rd::<u32>(table + 0xb8)) & !rd::<u32>(table + 0xbc);
    let mut hi = (rd::<u32>(table + 0x08) as i32).wrapping_sub(1);
    if hi < 0 {
        return 0;
    }
    let arr = rd::<usize>(table);
    let mut lo: i32 = 0;
    loop {
        let mid = lo.wrapping_add(hi) >> 1;
        let entry = rd::<usize>(arr.wrapping_add((mid as isize as usize).wrapping_mul(8)));
        let key = rd::<u32>(entry + 0x90);
        if eff < key {
            hi = mid.wrapping_sub(1);
        } else if eff <= key {
            return entry;
        } else {
            lo = mid.wrapping_add(1);
        }
        if lo > hi {
            return 0;
        }
    }
}

// ---------------------------------------------------------------------------
// 5. 0x14220e960 — hash_string_view_flagged
// ---------------------------------------------------------------------------

/// `hash_string_view_flagged(out, view, flag)` — writes `{fnv1a64_signed(view), flag as u64}` to
/// `out` and returns `out`.
///
/// Original: `0x14220e960` (82 bytes, leaf, `Throws::No`). `view` is `{const char* ptr, u64 len}`.
pub unsafe extern "win64" fn hash_string_view_flagged(out: usize, view: usize, flag: u8) -> usize {
    let mut p = rd::<usize>(view);
    let len = rd::<usize>(view + 0x08);
    let end = p.wrapping_add(len);
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    while p != end {
        let b = rd::<i8>(p) as i64 as u64;
        p = p.wrapping_add(1);
        hash = (hash ^ b).wrapping_mul(0x0100_0000_01b3);
    }
    wr::<u64>(out, hash);
    wr::<u64>(out + 0x08, flag as u64);
    out
}

// ---------------------------------------------------------------------------
// 6. 0x14222e1d0 — skinned_tree_has_active_node
// ---------------------------------------------------------------------------

/// `skinned_tree_has_active_node(tree)` — in-order traversal of MSVC `std::_Tree` starting at
/// `**tree` (leftmost node; sentinel has `byte [node + 0x19] != 0`), returning `1` as soon as a
/// node has `u32`s at `+0x548`, `+0x5b0`, and `+0x5cc` all non-zero (`> 0`), else `0`.
///
/// Original: `0x14222e1d0` (139 bytes, leaf, `Throws::No`, gates `0x14222dca0` every frame).
pub unsafe extern "win64" fn skinned_tree_has_active_node(tree: usize) -> u8 {
    let head = rd::<usize>(tree);
    let mut node = rd::<usize>(head);
    if rd::<u8>(node + 0x19) != 0 {
        return 0;
    }
    loop {
        if rd::<u32>(node + 0x548) > 0
            && rd::<u32>(node + 0x5b0) > 0
            && rd::<u32>(node + 0x5cc) > 0
        {
            return 1;
        }
        let right = rd::<usize>(node + 0x10);
        if rd::<u8>(right + 0x19) == 0 {
            let mut cur = right;
            let mut left = rd::<usize>(cur);
            while rd::<u8>(left + 0x19) == 0 {
                cur = left;
                left = rd::<usize>(cur);
            }
            node = cur;
        } else {
            let mut parent = rd::<usize>(node + 0x08);
            while rd::<u8>(parent + 0x19) == 0 && node == rd::<usize>(parent + 0x10) {
                node = parent;
                parent = rd::<usize>(parent + 0x08);
            }
            node = parent;
        }
        if rd::<u8>(node + 0x19) != 0 {
            return 0;
        }
    }
}

// ---------------------------------------------------------------------------
// 7 & 8. 0x14224d860 / 0x14224d920 — bind_mesh_geometry_streams_{a,b}
// ---------------------------------------------------------------------------

const DYN_TLS_INIT_VA: u64 = 0x1_4183_49c0;
const D3D_BIND_STREAMS_VA: u64 = 0x1_4206_f680;

/// `bind_mesh_geometry_streams_a(geom)` — builds the 6-slot stream descriptor `{0, geom+0x38, 0,
/// geom+0x48, geom+0x50, 0}` and 6-slot handle array `{0, geom+0x68, 0, geom+0x78, geom+0x80, 0}`
/// on the stack, ensures TLS slot 0 is initialized, and calls `d3d_14206f680(tls0 + 0x467e0, ...)`.
///
/// Original: `0x14224d860` (186 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn bind_mesh_geometry_streams_a(geom: usize) {
    let flag = rd::<u8>(geom + 0x97);
    let mut src = [0u64; 6];
    let mut handles = [0u64; 6];
    src[1] = rd::<u64>(geom + 0x38);
    src[3] = rd::<u64>(geom + 0x48);
    src[4] = rd::<u64>(geom + 0x50);
    handles[1] = rd::<u64>(geom + 0x68);
    handles[3] = rd::<u64>(geom + 0x78);
    handles[4] = rd::<u64>(geom + 0x80);

    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(DYN_TLS_INIT_VA, () -> ());
    }
    let ctx = tls0.wrapping_add(0x467e0);
    let s_ptr = src.as_mut_ptr() as usize;
    let h_ptr = handles.as_mut_ptr() as usize;
    exe::call_native!(
        D3D_BIND_STREAMS_VA,
        crate::ns::rend::d3d_rem_a::d3d_14206f680,
        (ctx, s_ptr, h_ptr, flag) -> ()
    );
}

/// `bind_mesh_geometry_streams_b(geom)` — builds `{geom+0x30, geom+0x38, 0, 0, 0, 0}` and
/// `{geom+0x60, geom+0x68, 0, 0, 0, 0}` and calls `d3d_14206f680(tls0 + 0x467e0, ...)`.
///
/// Original: `0x14224d920` (162 bytes, `Throws::Discard`, called in `0x142324af0` inner draw loop).
pub unsafe extern "win64" fn bind_mesh_geometry_streams_b(geom: usize) {
    let flag = rd::<u8>(geom + 0x97);
    let mut src = [0u64; 6];
    let mut handles = [0u64; 6];
    src[0] = rd::<u64>(geom + 0x30);
    src[1] = rd::<u64>(geom + 0x38);
    handles[0] = rd::<u64>(geom + 0x60);
    handles[1] = rd::<u64>(geom + 0x68);

    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(DYN_TLS_INIT_VA, () -> ());
    }
    let ctx = tls0.wrapping_add(0x467e0);
    let s_ptr = src.as_mut_ptr() as usize;
    let h_ptr = handles.as_mut_ptr() as usize;
    exe::call_native!(
        D3D_BIND_STREAMS_VA,
        crate::ns::rend::d3d_rem_a::d3d_14206f680,
        (ctx, s_ptr, h_ptr, flag) -> ()
    );
}

// ---------------------------------------------------------------------------
// 9a. 0x142263e80 — renderWorldInternal::<lambda_2>::operator()::<lambda_1>::vfunc_1
// ---------------------------------------------------------------------------

const UPDATE_DIRTY_BOUNDS_VA: u64 = 0x1_4226_1520;
const UPDATE_LIGHT_BOUNDS_VA: u64 = 0x1_4234_18c0;

/// `r::JobCallable<rend::RendererInterface::renderWorldInternal::<lambda_2>::operator()::<lambda_1>>::vfunc_1`
/// — Pass-1 gating worker job (~113 µs/frame): updates dirty mesh instance bounds (`0x142261520`
/// via `call_native!`), updates dirty light/volume bounds (`0x1423418c0`), and atomically
/// decrements the pass completion counter at `[this + 0x08]`.
///
/// Original: `0x142263e80` (43 bytes, `Throws::No`).
pub unsafe extern "win64" fn pass1_dirty_bounds_job_vfunc1(this: usize) {
    let inner = rd::<usize>(this + 0x10);
    let obj_mgr = rd::<usize>(inner + 0x1a8);
    exe::call_native!(
        UPDATE_DIRTY_BOUNDS_VA,
        crate::ns::rend::culling::update_dirty_instance_bounds,
        (obj_mgr as *mut u8) -> ()
    );
    exe::call_va!(UPDATE_LIGHT_BOUNDS_VA, () -> ());
    let cnt_ptr = rd::<usize>(this + 0x08);
    let _ = crate::simd_ops::atomic_xadd_u32(cnt_ptr as u64, (-1i32) as u32);
}

// ---------------------------------------------------------------------------
// 9b. 0x142264530 — renderWorldInternal::<lambda_6>::operator()::?bref1::operator()::<lambda_1>::vfunc_1
// ---------------------------------------------------------------------------

const D3D_BEGIN_UPLOAD_CTX_VA: u64 = 0x1_4205_fdc0;
const D3D_LOCK_UPLOAD_RING_VA: u64 = 0x1_4209_3110;
const FLUSH_MATERIAL_BUFFERS_VA: u64 = 0x1_4233_6540;
const FLUSH_DECAL_BUFFERS_VA: u64 = 0x1_423c_ff40;
const D3D_UNLOCK_UPLOAD_RING_VA: u64 = 0x1_4209_3200;
const D3D_END_UPLOAD_CTX_VA: u64 = 0x1_4205_fad0;
const G_D3D_QUEUE_FC60_VA: u64 = 0x1_4397_fc60;
const G_D3D_RING_FCB8_VA: u64 = 0x1_4397_fcb8;
const G_PASS5_CMD_OUT_VA: u64 = 0x1_43c8_c908;

/// `r::JobCallable<rend::RendererInterface::renderWorldInternal::<lambda_6>::operator()::?bref1::operator()::<lambda_1>>::vfunc_1`
/// — Pass-5 nested upload worker job (~95 µs/frame, spawned by `0x1422645d0`): opens an upload
/// command context, locks the upload ring, flushes dirty material/decal GPU buffers (`0x142336540`,
/// `0x1423cff40`), unlocks, closes the upload context, and stores the command handle at
/// `0x143c8c908`.
///
/// Original: `0x142264530` (115 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn pass5_upload_materials_job_vfunc1(this: usize) {
    let g_fc60 = rd::<u64>(exe::ptr::<u64>(G_D3D_QUEUE_FC60_VA) as usize);
    let mut desc = [0u64; 3];
    let dp = desc.as_mut_ptr() as usize;
    wr::<u64>(dp, 0);
    wr::<u32>(dp + 0x08, 1);
    wr::<u64>(dp + 0x10, g_fc60);
    exe::call_va!(D3D_BEGIN_UPLOAD_CTX_VA, (dp) -> ());
    exe::call_native!(
        D3D_LOCK_UPLOAD_RING_VA,
        crate::ns::rend::d3d_rem_c::d3d_142093110,
        (0usize) -> ()
    );
    let mgr_98 = rd::<usize>(rd::<usize>(this + 0x08) + 0x98);
    exe::call_native!(
        FLUSH_MATERIAL_BUFFERS_VA,
        crate::ns::rend::cauldron_hot_x6::rend_142336540,
        (mgr_98) -> ()
    );
    let mgr_a0 = rd::<usize>(rd::<usize>(this + 0x08) + 0xa0);
    exe::call_va!(FLUSH_DECAL_BUFFERS_VA, (mgr_a0) -> ());
    let g_fcb8 = rd::<usize>(exe::ptr::<usize>(G_D3D_RING_FCB8_VA) as usize);
    exe::call_va!(D3D_UNLOCK_UPLOAD_RING_VA, (g_fcb8) -> ());
    let res = exe::call_va!(D3D_END_UPLOAD_CTX_VA, () -> u64);
    wr::<u64>(exe::ptr::<u64>(G_PASS5_CMD_OUT_VA) as usize, res);
}

// ---------------------------------------------------------------------------
// 9c. 0x1422645d0 — renderWorldInternal::<lambda_6>::operator()::<lambda_1>::vfunc_1
// ---------------------------------------------------------------------------

const DIRTY_MATERIAL_UPDATE_VA: u64 = 0x1_4233_6b40;
const WORKER_INDEX_VA: u64 = 0x1_4240_7c80;
const JOB_NODE_ALLOC_VA: u64 = 0x1_4248_5aa0;
const SCHEDULER_ENQUEUE_JOB_VA: u64 = 0x1_4248_6460;
const G_JOB_SYSTEM_VA: u64 = 0x1_43a0_82f8;
const NESTED_CALLABLE_VTABLE_VA: u64 = 0x1_433f_38f8;
const G_NESTED_JOB_HANDLE_VA: u64 = 0x1_43c8_c930;
const IAT_RELEASE_SEMAPHORE: u64 = 0x1_4264_5580;

/// `r::JobCallable<rend::RendererInterface::renderWorldInternal::<lambda_6>::operator()::<lambda_1>>::vfunc_1`
/// — Pass-5 gating worker job: runs dirty material/property updates (`0x142336b40`), allocates and
/// enqueues the follow-up job (`vftable 0x1433f38f8` -> `0x142264530`) into `0x143c8c930`, and
/// decrements the pass completion counter at `[this + 0x08]`.
///
/// Original: `0x1422645d0` (213 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn pass5_dirty_materials_job_vfunc1(this: usize) {
    let inner = rd::<usize>(this + 0x10);
    let prop_mgr = rd::<usize>(inner + 0x98);
    exe::call_native!(
        DIRTY_MATERIAL_UPDATE_VA,
        crate::ns::rend::cauldron_hot_w6b::rend_142336b40,
        (prop_mgr) -> ()
    );

    let g_sys_slot = exe::ptr::<usize>(G_JOB_SYSTEM_VA) as usize;
    let rbx_payload = rd::<u64>(this + 0x10);
    let sched0 = rd::<usize>(rd::<usize>(g_sys_slot) + 0x10);
    let w = exe::call_native!(WORKER_INDEX_VA, crate::ns::r::thread::worker_index, () -> u32) as usize;
    let pool = rd::<usize>(sched0 + (w << 6) + 0x48).wrapping_add(0x40);
    let mut handle = [0u64; 2];
    let _ = exe::call_native!(
        JOB_NODE_ALLOC_VA,
        crate::ns::r::thread::job_node_alloc,
        (pool, handle.as_mut_ptr() as usize) -> usize
    );

    let node = handle[0] as usize;
    let _ = crate::simd_ops::atomic_xchg_u32(node as u64, 1);
    let vtbl = exe::ptr::<u8>(NESTED_CALLABLE_VTABLE_VA) as u64;
    let h0 = handle[0];
    let h1 = handle[1];
    wr::<u64>(node + 0x08, 0);
    wr::<u64>(node + 0x18, rbx_payload);
    let sys2 = rd::<usize>(g_sys_slot);
    wr::<u64>(node + 0x10, vtbl);
    let g_handle = exe::ptr::<u64>(G_NESTED_JOB_HANDLE_VA) as usize;
    wr::<u64>(g_handle, h0);
    wr::<u64>(g_handle + 0x08, h1);

    let state = if (node & 3) == 0 {
        AtomicU32::from_ptr(node as *mut u32).load(Ordering::Acquire)
    } else {
        rd::<u32>(node)
    };
    if state == 1 {
        let sched = rd::<usize>(sys2 + 0x10);
        let jn = rd::<usize>(g_handle);
        exe::call_native!(
            SCHEDULER_ENQUEUE_JOB_VA,
            crate::ns::r::thread::scheduler_enqueue_job,
            (sched, jn) -> ()
        );
        let sem = rd::<usize>(rd::<usize>(sched) + 0x1008);
        let _ = exe::call_iat_native!(
            IAT_RELEASE_SEMAPHORE,
            win32_rt::kernel32::threads::ReleaseSemaphore,
            (sem as *mut core::ffi::c_void, 1i32, core::ptr::null_mut::<i32>()) -> i32
        );
    }

    let cnt_ptr = rd::<usize>(this + 0x08);
    let _ = crate::simd_ops::atomic_xadd_u32(cnt_ptr as u64, (-1i32) as u32);
}

// ---------------------------------------------------------------------------
// 10. 0x142326d00 — eval_draw_pass_feature_flags
// ---------------------------------------------------------------------------

const G_FEAT_D8_VA: u64 = 0x1_4386_c4d8;
const G_FRAME_ID_VA: u64 = 0x1_43a0_0180;
const G_FEAT_A8_VA: u64 = 0x1_4386_c4a8;
const G_FEAT_C0_VA: u64 = 0x1_4386_c4c0;
const G_FEAT_78_VA: u64 = 0x1_4386_c478;
const G_FEAT_90_VA: u64 = 0x1_4386_c490;

/// `eval_draw_pass_feature_flags(state, ptr)` — evaluates per-pass boolean/u32 feature flags at
/// `state + 0xe0 .. +0xe8` from the pass's bitmask (`[state]`), frame counter (`0x143a00180`),
/// frustum plane count (`[state+0x10]+0x180 <= 13`), and renderer CVars.
///
/// Original: `0x142326d00` (316 bytes, leaf, `Throws::No`, called in `0x142286b30` and draw passes).
pub unsafe extern "win64" fn eval_draw_pass_feature_flags(state: usize, ptr: usize) {
    let p30 = rd::<usize>(state + 0x30);
    let p38 = rd::<usize>(state + 0x38);
    let (r9b, seq) = if rd::<u32>(p30 + 0x10) == 1
        && rd::<u8>(exe::ptr::<u8>(G_FEAT_D8_VA) as usize) != 0
    {
        wr::<u8>(state + 0xe0, 1);
        wr::<u32>(state + 0xe8, 1);
        (1u8, rd::<u64>(p38 + 0x80))
    } else {
        wr::<u8>(state + 0xe0, 0);
        wr::<u32>(state + 0xe8, 0);
        (0u8, rd::<u64>(p38 + 0x18))
    };

    let frame = rd::<i32>(exe::ptr::<i32>(G_FRAME_ID_VA) as usize) as i64 as u64;
    let bits0 = rd::<u8>(state);
    let e3_init = u8::from(
        seq.wrapping_add(1) == frame
            && (bits0 & 0x04) != 0
            && rd::<u64>(ptr) != 0
            && rd::<u8>(exe::ptr::<u8>(G_FEAT_A8_VA) as usize) != 0,
    );
    wr::<u8>(state + 0xe3, e3_init);

    let e1 = if rd::<u64>(state + 0x08) == 0 {
        wr::<u8>(state + 0xe3, 0);
        0u8
    } else if e3_init != 0 && (bits0 & 0x08) != 0 && rd::<i32>(state + 0x40) != -1 {
        1u8
    } else {
        0u8
    };
    wr::<u8>(state + 0xe1, e1);

    let e2 = u8::from(
        rd::<u8>(exe::ptr::<u8>(G_FEAT_C0_VA) as usize) != 0 && (rd::<u8>(state) & 0x01) != 0,
    );
    wr::<u8>(state + 0xe2, e2);

    let p10 = rd::<usize>(state + 0x10);
    let keep_e2 = (p10 != 0 && rd::<i32>(p10 + 0x180) <= 0x0d)
        || rd::<u64>(state + 0x18) != 0
        || rd::<u32>(state + 0x20) <= 0x0d;
    if !keep_e2 {
        wr::<u8>(state + 0xe2, 0);
    }

    let e4 = u8::from(
        rd::<u8>(exe::ptr::<u8>(G_FEAT_78_VA) as usize) != 0 && (rd::<u8>(state) & 0x02) != 0,
    );
    wr::<u8>(state + 0xe4, e4);

    let e5 = u8::from(rd::<u8>(exe::ptr::<u8>(G_FEAT_90_VA) as usize) != 0 && r9b != 0);
    wr::<u8>(state + 0xe5, e5);

    let e6 = ((rd::<u32>(state) >> 4) as u8) & 1;
    wr::<u8>(state + 0xe6, e6);
}

// ---------------------------------------------------------------------------
// 11. 0x142326e60 — find_draw_pass_bucket_nocase
// ---------------------------------------------------------------------------

/// `find_draw_pass_bucket_nocase(table, view)` — searches `table->entries[0..count]` (`+0x18` base,
/// `u32` count `+0x20`, stride `0x38`, `InplaceString` at entry `+0x10`) for a case-insensitive
/// match against `string_view` at `view` (`{ptr, u64 len}`), returning the `u32` bucket index or
/// `u32::MAX` (`-1`).
///
/// Original: `0x142326e60` (240 bytes, leaf, `Throws::No`, called by `GfxGraphDrawVisibleSetCommand`
/// `0x1423736a0` and `0x142286b30`).
pub unsafe extern "win64" fn find_draw_pass_bucket_nocase(table: usize, view: usize) -> u32 {
    let count = rd::<u32>(table + 0x20) as usize;
    if count == 0 {
        return u32::MAX;
    }
    let v_ptr = rd::<usize>(view);
    let v_len = rd::<u64>(view + 0x08);
    let mut entry_str = rd::<usize>(table + 0x18).wrapping_add(0x10);
    let fold = exe::ptr::<u8>(CASE_FOLD_TABLE_VA) as usize;
    for idx in 0..count {
        let (s_ptr, s_len) = inplace_str_ptr_len(entry_str);
        if (s_len as u64) == v_len
            && (s_len == 0 || eq_nocase_raw(s_ptr, v_ptr, s_len, fold))
        {
            return idx as u32;
        }
        entry_str = entry_str.wrapping_add(0x38);
    }
    u32::MAX
}

// ---------------------------------------------------------------------------
// 12. 0x142398a00 — find_named_entry_nocase
// ---------------------------------------------------------------------------

/// `find_named_entry_nocase(container, view)` — scans the pointer array `[container + 0x60]` of
/// `u32` length `[container + 0x68]` for the first object whose `InplaceString` at `obj + 0x68`
/// matches `view` (`{ptr, u64 len}`) case-insensitively, returning `obj` or `0`.
///
/// Original: `0x142398a00` (242 bytes, leaf, `Throws::No`, 139 callers / 313 call sites).
pub unsafe extern "win64" fn find_named_entry_nocase(container: usize, view: usize) -> usize {
    let mut cur = rd::<usize>(container + 0x60);
    let count = rd::<u32>(container + 0x68) as usize;
    let end = cur.wrapping_add(count * 8);
    if cur == end {
        return 0;
    }
    let v_ptr = rd::<usize>(view);
    let v_len = rd::<u64>(view + 0x08);
    let fold = exe::ptr::<u8>(CASE_FOLD_TABLE_VA) as usize;
    while cur != end {
        let obj = rd::<usize>(cur);
        let (s_ptr, s_len) = inplace_str_ptr_len(obj.wrapping_add(0x68));
        if (s_len as u64) == v_len
            && (s_len == 0 || eq_nocase_raw(s_ptr, v_ptr, s_len, fold))
        {
            return obj;
        }
        cur = cur.wrapping_add(8);
    }
    0
}

// ---------------------------------------------------------------------------
// 13. 0x1423c3030 — cmd_node_key_less
// ---------------------------------------------------------------------------

/// `cmd_node_key_less(a, b)` — sort predicate passed by address to `std::sort` in
/// `run_command_list` (`0x1423c3e30`): unsigned 32-bit `a->key (+8) < b->key (+8)`.
///
/// Original: `0x1423c3030` (10 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn cmd_node_key_less(a: usize, b: usize) -> u8 {
    u8::from(rd::<u32>(a + 0x08) < rd::<u32>(b + 0x08))
}

// ---------------------------------------------------------------------------
// 14. 0x1423fd3c0 — frustum_from_view_proj
// ---------------------------------------------------------------------------

const K_SIGN_MASK_VA: u64 = 0x1_4342_8760; // 4 x -0.0f
const K_THREE_VA: u64 = 0x1_4342_8460; // 4 x 3.0f
const K_HALF_VEC_VA: u64 = 0x1_4342_8120; // 4 x 0.5f

/// `frustum_from_view_proj(view, frustum)` — converts the row-major `double[16]` view-projection
/// matrix at `view + 0x1d0` to `float[16]`, extracts the 6 Gribb-Hartmann frustum planes (LH `[0,1]`
/// depth: `c3+c0`, `-c0+c3`, `-c1+c3`, `c3+c1`, `c2`, `-c2+c3`), writes `count = 6` to
/// `frustum + 0x180`, and normalizes each plane with `vdpps 0x7f` + `vrsqrtps` + one Newton-Raphson
/// step `(3.0 - d*r*r) * (0.5 * r)`.
///
/// Original: `0x1423fd3c0` (302 bytes, leaf, `Throws::No`, 15 callers incl. `light_grid_update` and
/// `GfxGraphExecuteCullingCommand`).
pub unsafe extern "win64" fn frustum_from_view_proj(view: usize, frustum: usize) {
    let mut m = [0.0f32; 16];
    for (i, slot) in m.iter_mut().enumerate() {
        *slot = sse_cvt_f64_f32(rd::<f64>(view + 0x1d0 + i * 8));
    }
    let c0 = [m[0], m[4], m[8], m[12]];
    let c1 = [m[1], m[5], m[9], m[13]];
    let c2 = [m[2], m[6], m[10], m[14]];
    let c3 = [m[3], m[7], m[11], m[15]];

    let sign_mask: [u32; 4] = rd(exe::ptr::<u8>(K_SIGN_MASK_VA) as usize);
    let three: [f32; 4] = rd(exe::ptr::<u8>(K_THREE_VA) as usize);
    let half: [f32; 4] = rd(exe::ptr::<u8>(K_HALF_VEC_VA) as usize);

    let neg = |v: [f32; 4]| -> [f32; 4] {
        [
            f32::from_bits(v[0].to_bits() ^ sign_mask[0]),
            f32::from_bits(v[1].to_bits() ^ sign_mask[1]),
            f32::from_bits(v[2].to_bits() ^ sign_mask[2]),
            f32::from_bits(v[3].to_bits() ^ sign_mask[3]),
        ]
    };
    let add4 = |a: [f32; 4], b: [f32; 4]| -> [f32; 4] {
        [
            sse_add(a[0], b[0]),
            sse_add(a[1], b[1]),
            sse_add(a[2], b[2]),
            sse_add(a[3], b[3]),
        ]
    };

    wr::<i32>(frustum + 0x180, 6);
    let planes = [
        add4(c3, c0),
        add4(neg(c0), c3),
        add4(neg(c1), c3),
        add4(c3, c1),
        c2,
        add4(neg(c2), c3),
    ];
    for (idx, p) in planes.iter().enumerate() {
        wr::<[f32; 4]>(frustum + idx * 0x10, *p);
    }

    let mut i = 0i32;
    let mut p_addr = frustum;
    let count_addr = frustum + 0x180;
    while i < rd::<i32>(count_addr) {
        let p: [f32; 4] = rd(p_addr);
        // `vdpps xmm0, xmm4, xmm4, 0x7f`: `(p0*p0 + p1*p1) + (p2*p2 + +0.0f)` broadcast to 4 lanes.
        let p00 = sse_mul(p[0], p[0]);
        let p11 = sse_mul(p[1], p[1]);
        let p22 = sse_mul(p[2], p[2]);
        let dot = sse_add(sse_add(p00, p11), sse_add(p22, 0.0));
        let mut out = [0.0f32; 4];
        for lane in 0..4 {
            let r = sse_rsqrt_lane(dot);
            let dr = sse_mul(dot, r);
            let drr = sse_mul(dr, r);
            let sub = sse_sub(three[lane], drr);
            let hr = sse_mul(half[lane], r);
            let scale = sse_mul(sub, hr);
            out[lane] = sse_mul(scale, p[lane]);
        }
        wr::<[f32; 4]>(p_addr, out);
        i += 1;
        p_addr = p_addr.wrapping_add(0x10);
    }
}

// ===========================================================================
// Differential tests (`--diff-test`)
// ===========================================================================

struct SavedGlobals(Vec<(usize, u64)>);

impl SavedGlobals {
    unsafe fn save(vas: &[u64]) -> Self {
        Self(
            vas.iter()
                .map(|&va| {
                    let a = exe::ptr::<u8>(va) as usize;
                    (a, rd::<u64>(a))
                })
                .collect(),
        )
    }
}

impl Drop for SavedGlobals {
    fn drop(&mut self) {
        for &(a, v) in self.0.iter().rev() {
            unsafe { wr::<u64>(a, v) };
        }
    }
}

pub unsafe fn diff_vec4_mul_mat4_inplace(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1402_74e2);
    let mut vbuf_o = [0u8; 32];
    let mut vbuf_n = [0u8; 32];
    let mut mbuf = [0u8; 80];
    let mut cases = 0u32;

    for round in 0..256u32 {
        let voff = if round % 2 == 0 { 0 } else { 4 };
        let moff = if round % 3 == 0 { 0 } else { 4 };
        let vo = vbuf_o.as_mut_ptr() as usize + voff;
        let vn = vbuf_n.as_mut_ptr() as usize + voff;
        let mp = mbuf.as_mut_ptr() as usize + moff;

        let mut v = [0.0f32; 4];
        let mut m = [0.0f32; 16];
        for x in &mut v {
            *x = rng.f32_any();
        }
        for x in &mut m {
            *x = rng.f32_any();
        }
        if round % 17 == 5 {
            v[(round as usize) % 4] = f32::NAN;
        }
        if round % 19 == 7 {
            m[(round as usize) % 16] = f32::INFINITY;
        }
        wr::<[f32; 4]>(vo, v);
        wr::<[f32; 4]>(vn, v);
        wr::<[f32; 16]>(mp, m);

        fo(vo, mp);
        fn_(vn, mp);
        let out_o = rd::<[u32; 4]>(vo);
        let out_n = rd::<[u32; 4]>(vn);
        crate::diff_eq!(out_o, out_n, "vec4_mul_mat4 round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_mat4_d2f(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1403_b3de);
    let mut src_buf = [0u8; 144];
    let mut dst_o = [0u8; 80];
    let mut dst_n = [0u8; 80];
    let mut cases = 0u32;

    for round in 0..200u32 {
        let soff = if round % 2 == 0 { 0 } else { 4 };
        let doff = if round % 3 == 0 { 0 } else { 4 };
        let sp = src_buf.as_mut_ptr() as usize + soff;
        let dpo = dst_o.as_mut_ptr() as usize + doff;
        let dpn = dst_n.as_mut_ptr() as usize + doff;

        for i in 0..16 {
            let val = match (round + i as u32) % 13 {
                0 => 0.0f64,
                1 => -0.0f64,
                2 => f64::INFINITY,
                3 => f64::NEG_INFINITY,
                4 => f64::from_bits(0x7ff8_0012_3456_7890),
                _ => (rng.f32_any() as f64) * 1.000_000_119_209_289_6,
            };
            wr::<f64>(sp + i * 8, val);
        }
        dst_o.fill(0xaa);
        dst_n.fill(0xaa);

        let ro = fo(dpo, sp);
        let rn = fn_(dpn, sp);
        crate::diff_eq!(ro, dpo, "mat4_d2f orig ret round {round}");
        crate::diff_eq!(rn, dpn, "mat4_d2f new ret round {round}");
        crate::diff_eq!(dst_o, dst_n, "mat4_d2f bytes round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_pass_data_find_cstr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1412_ff13);
    let mut pass_buf = vec![0u8; 0x418];
    let mut str_buf = [0u8; 80];
    let mut cases = 0u32;

    for round in 0..160u32 {
        let poff = if round % 2 == 0 { 0 } else { 4 };
        let soff = (round as usize) % 7;
        let pp = pass_buf.as_mut_ptr() as usize + poff;
        let sp = str_buf.as_mut_ptr() as usize + soff;

        let slen = match round {
            0 => 0,
            1 => 1,
            _ => (rng.below(40) as usize) + 1,
        };
        for i in 0..slen {
            let mut b = rng.next_u32() as u8;
            if b == 0 {
                b = 0x80 | ((i as u8) + 1); // exercise sign-extended high bytes too
            }
            wr::<u8>(sp + i, b);
        }
        wr::<u8>(sp + slen, 0);

        // Compute target hash to plant hits in some rounds.
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for i in 0..slen {
            h = (h ^ (rd::<i8>(sp + i) as i64 as u64)).wrapping_mul(0x0100_0000_01b3);
        }
        for i in 0..0x40usize {
            let k = if round % 5 == 0 {
                0
            } else if round % 3 != 0 && i == (round as usize % 0x40) {
                h
            } else {
                rng.next_u64()
            };
            wr::<u64>(pp + 0x208 + i * 8, k);
            wr::<u64>(pp + 0x08 + i * 8, rng.next_u64() | 1);
        }

        let ro = fo(pp, sp);
        let rn = fn_(pp, sp);
        crate::diff_eq!(ro, rn, "pass_data_find_cstr round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_shader_variant_bsearch(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1422_0e86);
    let mut tbl_buf = [0u8; 0xd0];
    let mut entries_mem = vec![0u8; 64 * 0xa0 + 8];
    let mut ptrs = [0usize; 64];
    let mut cases = 0u32;

    for round in 0..200u32 {
        let count = match round {
            0 => 0usize,
            1 => 1,
            2 => 2,
            _ => (rng.below(48) as usize) + 1,
        };
        let toff = if round % 2 == 0 { 0 } else { 4 };
        let tp = tbl_buf.as_mut_ptr() as usize + toff;
        let ebase = entries_mem.as_mut_ptr() as usize + if round % 3 == 0 { 0 } else { 4 };

        let mut keys: Vec<u32> = (0..count).map(|_| rng.next_u32()).collect();
        keys.sort_unstable();
        keys.dedup();
        let count = keys.len();
        for (i, &k) in keys.iter().enumerate() {
            let ep = ebase + i * 0xa0;
            wr::<u32>(ep + 0x90, k);
            ptrs[i] = ep;
        }

        let or_mask = if round % 4 == 0 { 0 } else { rng.next_u32() & 0x00ff_00ff };
        let andn_mask = if round % 5 == 0 { 0 } else { rng.next_u32() & 0x0f00_0f00 };
        wr::<usize>(tp, ptrs.as_ptr() as usize);
        wr::<u32>(tp + 0x08, count as u32);
        wr::<u32>(tp + 0xb8, or_mask);
        wr::<u32>(tp + 0xbc, andn_mask);

        let query = if count > 0 && round % 2 == 0 {
            let target = keys[(round as usize) % count];
            // Try passing a mask that maps onto or near `target`.
            target ^ (or_mask & !andn_mask)
        } else {
            rng.next_u32()
        };
        // Also test directly when `or_mask == 0 && andn_mask == 0` for guaranteed exact hits.
        let ro = fo(tp, query);
        let rn = fn_(tp, query);
        crate::diff_eq!(ro, rn, "shader_variant_bsearch round {round} count={count}");
        cases += 1;

        if count > 0 {
            wr::<u32>(tp + 0xb8, 0);
            wr::<u32>(tp + 0xbc, 0);
            let hit_k = keys[rng.below(count as u64) as usize];
            crate::diff_eq!(fo(tp, hit_k), fn_(tp, hit_k), "shader_variant_bsearch exact round {round}");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_hash_string_view_flagged(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> usize;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1422_0e96);
    let mut out_o = [0u8; 24];
    let mut out_n = [0u8; 24];
    let mut view_buf = [0u8; 24];
    let mut data_buf = [0u8; 96];
    let mut cases = 0u32;

    for round in 0..160u32 {
        let off = if round % 2 == 0 { 0 } else { 4 };
        let oo = out_o.as_mut_ptr() as usize + off;
        let on = out_n.as_mut_ptr() as usize + off;
        let vp = view_buf.as_mut_ptr() as usize + off;
        let dp = data_buf.as_mut_ptr() as usize + ((round as usize) % 5);

        let len = if round == 0 { 0 } else { rng.below(64) as usize };
        for i in 0..len {
            wr::<u8>(dp + i, rng.next_u32() as u8);
        }
        wr::<usize>(vp, dp);
        wr::<u64>(vp + 0x08, len as u64);
        let flag = rng.next_u32() as u8;

        out_o.fill(0xcc);
        out_n.fill(0xcc);
        let ro = fo(oo, vp, flag);
        let rn = fn_(on, vp, flag);
        crate::diff_eq!(ro, oo, "hash_string_view_flagged orig ret round {round}");
        crate::diff_eq!(rn, on, "hash_string_view_flagged new ret round {round}");
        crate::diff_eq!(out_o, out_n, "hash_string_view_flagged out round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_skinned_tree_has_active_node(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1422_2e1d);
    let mut arena = vec![0u8; 16 * 0x600 + 16];
    let mut cases = 0u32;

    // Build a valid BST of `n` nodes (indices 1..=n; 0 = sentinel head).
    unsafe fn build_bst(
        base: usize,
        head: usize,
        lo: usize,
        hi: usize,
        parent: usize,
    ) -> usize {
        if lo > hi {
            return head;
        }
        let mid = (lo + hi) / 2;
        let nd = base + mid * 0x600;
        wr::<u8>(nd + 0x19, 0);
        wr::<usize>(nd + 0x08, parent);
        let l = if mid > lo { build_bst(base, head, lo, mid - 1, nd) } else { head };
        let r = if mid < hi { build_bst(base, head, mid + 1, hi, nd) } else { head };
        wr::<usize>(nd, l);
        wr::<usize>(nd + 0x10, r);
        nd
    }

    for round in 0..160u32 {
        arena.fill(0);
        let base = arena.as_mut_ptr() as usize + if round % 2 == 0 { 0 } else { 4 };
        let head = base;
        wr::<u8>(head + 0x19, 1);
        let n = match round {
            0 => 0usize,
            1 => 1,
            2 => 2,
            _ => (rng.below(14) as usize) + 1,
        };
        if n == 0 {
            wr::<usize>(head, head);
            wr::<usize>(head + 0x08, head);
            wr::<usize>(head + 0x10, head);
        } else {
            let root = build_bst(base, head, 1, n, head);
            wr::<usize>(head, base + 0x600); // leftmost = node 1
            wr::<usize>(head + 0x08, root);
            wr::<usize>(head + 0x10, base + n * 0x600);
            for k in 1..=n {
                let nd = base + k * 0x600;
                let a = if rng.below(3) == 0 { 0 } else { rng.next_u32() | 1 };
                let b = if rng.below(3) == 0 { 0 } else { rng.next_u32() | 1 };
                let c = if round % 4 == 0 {
                    0 // force full traversal to sentinel
                } else if rng.below(4) == 0 {
                    rng.next_u32() | 1
                } else {
                    0
                };
                wr::<u32>(nd + 0x548, a);
                wr::<u32>(nd + 0x5b0, b);
                wr::<u32>(nd + 0x5cc, c);
            }
        }
        let mut tree_holder = [0u8; 16];
        let tp = tree_holder.as_mut_ptr() as usize + if round % 3 == 0 { 0 } else { 4 };
        wr::<usize>(tp, head);

        let ro = fo(tp);
        let rn = fn_(tp);
        crate::diff_eq!(ro, rn, "skinned_tree_has_active_node round {round} n={n}");
        cases += 1;
    }
    Ok(cases)
}

thread_local! {
    static STREAM_LOG: RefCell<Vec<([u64; 6], [u64; 6], usize, u8, u8)>> = const { RefCell::new(Vec::new()) };
    static TLS_INIT_CNT: RefCell<u32> = const { RefCell::new(0) };
}

unsafe extern "win64" fn mock_tls_init() {
    let tls0 = rd::<usize>(exe::tls_pointer());
    wr::<u8>(tls0 + 0x70, 1);
    TLS_INIT_CNT.with(|c| *c.borrow_mut() += 1);
}

unsafe extern "win64" fn mock_bind_streams(ctx: usize, src: usize, handles: usize, flag: u8) {
    let s = rd::<[u64; 6]>(src);
    let h = rd::<[u64; 6]>(handles);
    let init_cnt = TLS_INIT_CNT.with(|c| *c.borrow()) as u8;
    STREAM_LOG.with(|l| l.borrow_mut().push((s, h, ctx, flag, init_cnt)));
}

unsafe fn diff_bind_streams_common(orig: usize, new: usize, seed: u64) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = crate::difftest::Mock::new(DYN_TLS_INIT_VA, mock_tls_init as *const ())?;
    let _m2 = crate::difftest::Mock::new(D3D_BIND_STREAMS_VA, mock_bind_streams as *const ())?;

    let tls_arr = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_arr);
    let mut fake_tls = vec![0u8; 0x46900];
    let tls_base = fake_tls.as_mut_ptr() as usize;
    wr::<usize>(tls_arr, tls_base);

    let mut rng = crate::difftest::Rng::new(seed);
    let mut geom_buf = [0u8; 0xb0];
    let mut cases = 0u32;

    let res = (|| -> Result<u32, String> {
        for round in 0..96u32 {
            let goff = if round % 2 == 0 { 0 } else { 4 };
            let gp = geom_buf.as_mut_ptr() as usize + goff;
            for i in 0..0xa0 {
                wr::<u8>(gp + i, rng.next_u32() as u8);
            }
            let init_b = u8::from(round % 3 != 0);

            STREAM_LOG.with(|l| l.borrow_mut().clear());
            TLS_INIT_CNT.with(|c| *c.borrow_mut() = 0);
            wr::<u8>(tls_base + 0x70, init_b);
            fo(gp);
            let log_o = STREAM_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
            let tls70_o = rd::<u8>(tls_base + 0x70);

            STREAM_LOG.with(|l| l.borrow_mut().clear());
            TLS_INIT_CNT.with(|c| *c.borrow_mut() = 0);
            wr::<u8>(tls_base + 0x70, init_b);
            fn_(gp);
            let log_n = STREAM_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
            let tls70_n = rd::<u8>(tls_base + 0x70);

            crate::diff_eq!(log_o, log_n, "bind_streams log round {round}");
            crate::diff_eq!(tls70_o, tls70_n, "bind_streams tls+0x70 round {round}");
            cases += 1;
        }
        Ok(cases)
    })();

    wr::<usize>(tls_arr, saved_tls0);
    res
}

pub unsafe fn diff_bind_mesh_geometry_streams_a(orig: usize, new: usize) -> Result<u32, String> {
    diff_bind_streams_common(orig, new, 0x1422_4d86)
}

pub unsafe fn diff_bind_mesh_geometry_streams_b(orig: usize, new: usize) -> Result<u32, String> {
    diff_bind_streams_common(orig, new, 0x1422_4d92)
}

thread_local! {
    static PASS1_LOG: RefCell<Vec<[u64; 2]>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn mock_update_dirty_bounds(mgr: usize) {
    PASS1_LOG.with(|l| l.borrow_mut().push([1, mgr as u64]));
}

unsafe extern "win64" fn mock_update_light_bounds() {
    PASS1_LOG.with(|l| l.borrow_mut().push([2, 0]));
}

pub unsafe fn diff_pass1_dirty_bounds_job_vfunc1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = crate::difftest::Mock::new(UPDATE_DIRTY_BOUNDS_VA, mock_update_dirty_bounds as *const ())?;
    let _m2 = crate::difftest::Mock::new(UPDATE_LIGHT_BOUNDS_VA, mock_update_light_bounds as *const ())?;

    let mut rng = crate::difftest::Rng::new(0x1422_63e8);
    let mut inner = [0u8; 0x1c0];
    let mut callable = [0u8; 0x28];
    let mut cases = 0u32;

    for round in 0..64u32 {
        let off = if round % 2 == 0 { 0 } else { 4 };
        let ip = inner.as_mut_ptr() as usize + off;
        let cp = callable.as_mut_ptr() as usize + off;
        let mgr = (rng.next_u64() | 0x4000) as usize;
        wr::<usize>(ip + 0x1a8, mgr);
        wr::<usize>(cp + 0x10, ip);

        let init_cnt = (rng.below(32) as u32) + 1;
        let mut cnt_o = init_cnt;
        let mut cnt_n = init_cnt;

        wr::<usize>(cp + 0x08, (&mut cnt_o as *mut u32) as usize);
        PASS1_LOG.with(|l| l.borrow_mut().clear());
        fo(cp);
        let log_o = PASS1_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));

        wr::<usize>(cp + 0x08, (&mut cnt_n as *mut u32) as usize);
        PASS1_LOG.with(|l| l.borrow_mut().clear());
        fn_(cp);
        let log_n = PASS1_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));

        crate::diff_eq!(log_o, log_n, "pass1_dirty_bounds_job log round {round}");
        crate::diff_eq!(cnt_o, cnt_n, "pass1_dirty_bounds_job cnt round {round}");
        cases += 1;
    }
    Ok(cases)
}

thread_local! {
    static PASS5_UP_LOG: RefCell<Vec<[u64; 4]>> = const { RefCell::new(Vec::new()) };
    static PASS5_END_RET: RefCell<u64> = const { RefCell::new(0) };
}

unsafe extern "win64" fn mock_begin_upload(desc: usize) {
    PASS5_UP_LOG.with(|l| {
        l.borrow_mut()
            .push([1, rd::<u64>(desc), rd::<u32>(desc + 0x08) as u64, rd::<u64>(desc + 0x10)]);
    });
}

unsafe extern "win64" fn mock_lock_ring(_unused: usize) {
    PASS5_UP_LOG.with(|l| l.borrow_mut().push([2, 0, 0, 0]));
}

unsafe extern "win64" fn mock_flush_mat(mgr: usize) {
    PASS5_UP_LOG.with(|l| l.borrow_mut().push([3, mgr as u64, 0, 0]));
}

unsafe extern "win64" fn mock_flush_decal(mgr: usize) {
    PASS5_UP_LOG.with(|l| l.borrow_mut().push([4, mgr as u64, 0, 0]));
}

unsafe extern "win64" fn mock_unlock_ring(ring: usize) {
    PASS5_UP_LOG.with(|l| l.borrow_mut().push([5, ring as u64, 0, 0]));
}

unsafe extern "win64" fn mock_end_upload() -> u64 {
    let r = PASS5_END_RET.with(|v| *v.borrow());
    PASS5_UP_LOG.with(|l| l.borrow_mut().push([6, r, 0, 0]));
    r
}

pub unsafe fn diff_pass5_upload_materials_job_vfunc1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _saved = SavedGlobals::save(&[G_D3D_QUEUE_FC60_VA, G_D3D_RING_FCB8_VA, G_PASS5_CMD_OUT_VA]);
    let _m1 = crate::difftest::Mock::new(D3D_BEGIN_UPLOAD_CTX_VA, mock_begin_upload as *const ())?;
    let _m2 = crate::difftest::Mock::new(D3D_LOCK_UPLOAD_RING_VA, mock_lock_ring as *const ())?;
    let _m3 = crate::difftest::Mock::new(FLUSH_MATERIAL_BUFFERS_VA, mock_flush_mat as *const ())?;
    let _m4 = crate::difftest::Mock::new(FLUSH_DECAL_BUFFERS_VA, mock_flush_decal as *const ())?;
    let _m5 = crate::difftest::Mock::new(D3D_UNLOCK_UPLOAD_RING_VA, mock_unlock_ring as *const ())?;
    let _m6 = crate::difftest::Mock::new(D3D_END_UPLOAD_CTX_VA, mock_end_upload as *const ())?;

    let mut rng = crate::difftest::Rng::new(0x1422_6453);
    let mut inner = [0u8; 0xc0];
    let mut callable = [0u8; 0x20];
    let mut cases = 0u32;

    for round in 0..64u32 {
        let off = if round % 2 == 0 { 0 } else { 4 };
        let ip = inner.as_mut_ptr() as usize + off;
        let cp = callable.as_mut_ptr() as usize + off;

        wr::<u64>(exe::ptr::<u8>(G_D3D_QUEUE_FC60_VA) as usize, rng.next_u64() | 1);
        wr::<u64>(exe::ptr::<u8>(G_D3D_RING_FCB8_VA) as usize, rng.next_u64() | 2);
        wr::<usize>(ip + 0x98, (rng.next_u64() | 0x10) as usize);
        wr::<usize>(ip + 0xa0, (rng.next_u64() | 0x20) as usize);
        wr::<usize>(cp + 0x08, ip);

        let end_ret = rng.next_u64();
        PASS5_END_RET.with(|v| *v.borrow_mut() = end_ret);

        wr::<u64>(exe::ptr::<u8>(G_PASS5_CMD_OUT_VA) as usize, 0);
        PASS5_UP_LOG.with(|l| l.borrow_mut().clear());
        fo(cp);
        let log_o = PASS5_UP_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
        let out_o = rd::<u64>(exe::ptr::<u8>(G_PASS5_CMD_OUT_VA) as usize);

        wr::<u64>(exe::ptr::<u8>(G_PASS5_CMD_OUT_VA) as usize, 0);
        PASS5_UP_LOG.with(|l| l.borrow_mut().clear());
        fn_(cp);
        let log_n = PASS5_UP_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
        let out_n = rd::<u64>(exe::ptr::<u8>(G_PASS5_CMD_OUT_VA) as usize);

        crate::diff_eq!(log_o, log_n, "pass5_upload_materials_job log round {round}");
        crate::diff_eq!(out_o, out_n, "pass5_upload_materials_job out round {round}");
        cases += 1;
    }
    Ok(cases)
}

thread_local! {
    static PASS5_LOG: RefCell<Vec<[u64; 3]>> = const { RefCell::new(Vec::new()) };
    static PASS5_NODE_PTR: RefCell<(usize, u64, u32)> = const { RefCell::new((0, 0, 1)) };
}

unsafe extern "win64" fn mock_dirty_mat_update(mgr: usize) {
    PASS5_LOG.with(|l| l.borrow_mut().push([1, mgr as u64, 0]));
}

unsafe extern "win64" fn mock_worker_index() -> u32 {
    PASS5_LOG.with(|l| l.borrow_mut().push([2, 0, 0]));
    3
}

unsafe extern "win64" fn mock_job_node_alloc(pool: usize, out_handle: usize) -> usize {
    let (node, gen, post_state) = PASS5_NODE_PTR.with(|p| *p.borrow());
    wr::<u64>(out_handle, node as u64);
    wr::<u64>(out_handle + 0x08, gen);
    // Note: caller subsequently `xchg [node], 1`; to also test the `state != 1` branch
    // (concurrently modified before `mov eax, [rax]`), we only preset `[node]` here, while a
    // dedicated round has no race (state == 1).
    wr::<u32>(node, post_state);
    PASS5_LOG.with(|l| l.borrow_mut().push([3, pool as u64, node as u64]));
    out_handle
}

unsafe extern "win64" fn mock_enqueue_job(sched: usize, node: usize) {
    PASS5_LOG.with(|l| l.borrow_mut().push([4, sched as u64, node as u64]));
}

unsafe extern "win64" fn mock_rel_sem(h: usize, count: i32, prev: usize) -> i32 {
    PASS5_LOG.with(|l| l.borrow_mut().push([5, h as u64, (count as u32 as u64) | ((prev as u64) << 32)]));
    1
}

pub unsafe fn diff_pass5_dirty_materials_job_vfunc1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _saved = SavedGlobals::save(&[
        G_JOB_SYSTEM_VA,
        G_NESTED_JOB_HANDLE_VA,
        G_NESTED_JOB_HANDLE_VA + 8,
    ]);
    let _m1 = crate::difftest::Mock::new(DIRTY_MATERIAL_UPDATE_VA, mock_dirty_mat_update as *const ())?;
    let _m2 = crate::difftest::Mock::new(WORKER_INDEX_VA, mock_worker_index as *const ())?;
    let _m3 = crate::difftest::Mock::new(JOB_NODE_ALLOC_VA, mock_job_node_alloc as *const ())?;
    let _m4 = crate::difftest::Mock::new(SCHEDULER_ENQUEUE_JOB_VA, mock_enqueue_job as *const ())?;
    let _iat = crate::difftest::IatMock::new(IAT_RELEASE_SEMAPHORE, mock_rel_sem as *const ())?;

    let mut rng = crate::difftest::Rng::new(0x1422_645d);
    let mut sched_inner = vec![0u8; 0x1020];
    let mut sched = vec![0u8; 0x200];
    let mut sys_obj = [0u8; 0x30];
    let mut inner_obj = [0u8; 0xb0];
    let mut job_callable = [0u8; 0x30];
    let mut node_o = [0u8; 0x40];
    let mut node_n = [0u8; 0x40];
    let mut cases = 0u32;

    let sip = sched_inner.as_mut_ptr() as usize;
    let sp = sched.as_mut_ptr() as usize;
    let syp = sys_obj.as_mut_ptr() as usize;
    wr::<usize>(sp, sip);
    wr::<usize>(syp + 0x10, sp);
    wr::<usize>(exe::ptr::<u8>(G_JOB_SYSTEM_VA) as usize, syp);

    for round in 0..64u32 {
        let sem_handle = rng.next_u64() | 0x1000;
        let pool_base = (rng.next_u64() & 0x0000_ffff_ffff_fff0) as usize;
        wr::<u64>(sip + 0x1008, sem_handle);
        wr::<usize>(sp + (3 << 6) + 0x48, pool_base);

        let ip = inner_obj.as_mut_ptr() as usize + if round % 2 == 0 { 0 } else { 4 };
        let prop_mgr = (rng.next_u64() | 0x2000) as usize;
        wr::<usize>(ip + 0x98, prop_mgr);

        let gen = rng.next_u64();
        let init_cnt = (rng.below(16) as u32) + 1;
        let mut cnt_o = init_cnt;
        let mut cnt_n = init_cnt;

        let off = if round % 3 == 0 { 0 } else { 4 };
        let no_p = node_o.as_mut_ptr() as usize + off;
        let nn_p = node_n.as_mut_ptr() as usize + off;
        node_o.fill(0xdd);
        node_n.fill(0xdd);

        let jcp = job_callable.as_mut_ptr() as usize + off;
        wr::<usize>(jcp + 0x10, ip);

        // Run original
        wr::<usize>(jcp + 0x08, (&mut cnt_o as *mut u32) as usize);
        PASS5_NODE_PTR.with(|p| *p.borrow_mut() = (no_p, gen, 0x77));
        PASS5_LOG.with(|l| l.borrow_mut().clear());
        fo(jcp);
        let mut log_o = PASS5_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
        let gh_o = rd::<[u64; 2]>(exe::ptr::<u8>(G_NESTED_JOB_HANDLE_VA) as usize);

        // Run replacement
        wr::<usize>(jcp + 0x08, (&mut cnt_n as *mut u32) as usize);
        PASS5_NODE_PTR.with(|p| *p.borrow_mut() = (nn_p, gen, 0x77));
        PASS5_LOG.with(|l| l.borrow_mut().clear());
        fn_(jcp);
        let mut log_n = PASS5_LOG.with(|l| std::mem::take(&mut *l.borrow_mut()));
        let gh_n = rd::<[u64; 2]>(exe::ptr::<u8>(G_NESTED_JOB_HANDLE_VA) as usize);

        // Normalize the node buffer address inside log/global handle before comparing.
        for e in &mut log_o {
            if e[0] == 3 || e[0] == 4 {
                e[2] = e[2].wrapping_sub(no_p as u64);
            }
        }
        for e in &mut log_n {
            if e[0] == 3 || e[0] == 4 {
                e[2] = e[2].wrapping_sub(nn_p as u64);
            }
        }
        crate::diff_eq!(log_o, log_n, "pass5 job log round {round}");
        crate::diff_eq!(cnt_o, cnt_n, "pass5 job counter round {round}");
        crate::diff_eq!(node_o, node_n, "pass5 job node bytes round {round}");
        crate::diff_eq!(gh_o[0].wrapping_sub(no_p as u64), gh_n[0].wrapping_sub(nn_p as u64), "pass5 handle ptr round {round}");
        crate::diff_eq!(gh_o[1], gh_n[1], "pass5 handle gen round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_eval_draw_pass_feature_flags(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _saved = SavedGlobals::save(&[
        G_FEAT_D8_VA,
        G_FRAME_ID_VA,
        G_FEAT_A8_VA,
        G_FEAT_C0_VA,
        G_FEAT_78_VA,
        G_FEAT_90_VA,
    ]);
    let mut rng = crate::difftest::Rng::new(0x1423_26d0);
    let mut st_o = [0u8; 0x100];
    let mut st_n = [0u8; 0x100];
    let mut sub30 = [0u8; 0x30];
    let mut sub38 = [0u8; 0xa0];
    let mut sub10 = [0u8; 0x198];
    let mut cases = 0u32;

    for round in 0..512u32 {
        let off = if round % 2 == 0 { 0 } else { 4 };
        let so = st_o.as_mut_ptr() as usize + off;
        let sn = st_n.as_mut_ptr() as usize + off;
        let p30 = sub30.as_mut_ptr() as usize + off;
        let p38 = sub38.as_mut_ptr() as usize + off;
        let p10 = sub10.as_mut_ptr() as usize + off;

        let frame: i32 = match round % 4 {
            0 => 0,
            1 => -5,
            _ => (rng.below(1000) as i32) + 1,
        };
        wr::<i32>(exe::ptr::<u8>(G_FRAME_ID_VA) as usize, frame);
        wr::<u8>(exe::ptr::<u8>(G_FEAT_D8_VA) as usize, (rng.next_u32() & 1) as u8);
        wr::<u8>(exe::ptr::<u8>(G_FEAT_A8_VA) as usize, (rng.next_u32() & 1) as u8);
        wr::<u8>(exe::ptr::<u8>(G_FEAT_C0_VA) as usize, (rng.next_u32() & 1) as u8);
        wr::<u8>(exe::ptr::<u8>(G_FEAT_78_VA) as usize, (rng.next_u32() & 1) as u8);
        wr::<u8>(exe::ptr::<u8>(G_FEAT_90_VA) as usize, (rng.next_u32() & 1) as u8);

        wr::<u32>(p30 + 0x10, if rng.below(2) == 0 { 1 } else { rng.below(5) as u32 });
        let seq_match = (frame as i64).wrapping_sub(1) as u64;
        wr::<u64>(p38 + 0x80, if rng.below(2) == 0 { seq_match } else { rng.next_u64() });
        wr::<u64>(p38 + 0x18, if rng.below(2) == 0 { seq_match } else { rng.next_u64() });
        wr::<i32>(p10 + 0x180, (rng.below(30) as i32) - 4);

        let mut base_state = [0u8; 0xf0];
        for b in &mut base_state {
            *b = rng.next_u32() as u8;
        }
        wr::<u32>(base_state.as_mut_ptr() as usize, rng.next_u32() & 0xff);
        wr::<u64>(
            base_state.as_mut_ptr() as usize + 0x08,
            if rng.below(3) == 0 { 0 } else { rng.next_u64() | 1 },
        );
        wr::<usize>(
            base_state.as_mut_ptr() as usize + 0x10,
            if rng.below(3) == 0 { 0 } else { p10 },
        );
        wr::<u64>(
            base_state.as_mut_ptr() as usize + 0x18,
            if rng.below(2) == 0 { 0 } else { rng.next_u64() | 1 },
        );
        wr::<u32>(base_state.as_mut_ptr() as usize + 0x20, rng.below(28) as u32);
        wr::<usize>(base_state.as_mut_ptr() as usize + 0x30, p30);
        wr::<usize>(base_state.as_mut_ptr() as usize + 0x38, p38);
        wr::<i32>(
            base_state.as_mut_ptr() as usize + 0x40,
            if rng.below(3) == 0 { -1 } else { rng.below(16) as i32 },
        );

        st_o[off..off + 0xf0].copy_from_slice(&base_state);
        st_n[off..off + 0xf0].copy_from_slice(&base_state);

        let ptr_val: u64 = if rng.below(3) == 0 { 0 } else { rng.next_u64() | 1 };
        let p_arg = (&ptr_val as *const u64) as usize;

        fo(so, p_arg);
        fn_(sn, p_arg);
        crate::diff_eq!(
            &st_o[off..off + 0xf0],
            &st_n[off..off + 0xf0],
            "eval_draw_pass_feature_flags round {round}"
        );
        cases += 1;
    }
    Ok(cases)
}

unsafe fn write_test_inplace_string(s: usize, bytes: &[u8], heap_buf: usize, force_heap: bool) {
    let len = bytes.len();
    wr::<u32>(s + 0x04, len as u32);
    if force_heap || len > 15 {
        // Inline cap = 2 * 8 = 16 bits, cap_bits = 32 * 8 = 256 > 16 -> heap pointer at +0x08.
        wr::<u32>(s, (0x02u32 << 24) | 32u32);
        for (i, &b) in bytes.iter().enumerate() {
            wr::<u8>(heap_buf + i, b);
        }
        wr::<usize>(s + 0x08, heap_buf);
    } else {
        // Inline cap = 16 * 8 = 128 bits, cap_bits = 16 * 8 = 128 <= 128 -> inline at +0x08.
        wr::<u32>(s, (16u32 << 24) | 16u32);
        for (i, &b) in bytes.iter().enumerate() {
            wr::<u8>(s + 0x08 + i, b);
        }
    }
}

pub unsafe fn diff_find_draw_pass_bucket_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1423_26e6);
    let mut tbl = [0u8; 0x30];
    let mut entries = vec![0u8; 16 * 0x38 + 8];
    let mut heap_strs = vec![0u8; 16 * 64 + 8];
    let mut query_buf = [0u8; 64];
    let mut view_buf = [0u8; 24];
    let mut cases = 0u32;

    const SAMPLE_NAMES: &[&[u8]] = &[
        b"",
        b"depth",
        b"Opaque",
        b"shadow_cascade_0",
        b"TransparentMomentPassLongBucketName",
        b"AlphaTest_TwoSided_01",
    ];

    for round in 0..160u32 {
        let off = if round % 2 == 0 { 0 } else { 4 };
        let tp = tbl.as_mut_ptr() as usize + off;
        let ep = entries.as_mut_ptr() as usize + off;
        let hp = heap_strs.as_mut_ptr() as usize + off;
        let vp = view_buf.as_mut_ptr() as usize + off;

        let count = match round {
            0 => 0usize,
            1 => 1,
            _ => (rng.below(12) as usize) + 1,
        };
        entries.fill(0);
        for i in 0..count {
            let name = SAMPLE_NAMES[(i + round as usize) % SAMPLE_NAMES.len()];
            write_test_inplace_string(ep + i * 0x38 + 0x10, name, hp + i * 64, (i + round as usize) % 2 == 1);
        }
        wr::<usize>(tp + 0x18, ep);
        wr::<u32>(tp + 0x20, count as u32);

        let qsrc: &[u8] = if count > 0 && round % 3 != 0 {
            SAMPLE_NAMES[((rng.below(count as u64) as usize) + round as usize) % SAMPLE_NAMES.len()]
        } else {
            b"nonexistent_bucket"
        };
        let qp = query_buf.as_mut_ptr() as usize + ((round as usize) % 3);
        for (i, &b) in qsrc.iter().enumerate() {
            // Flip ASCII case on alternating bytes to exercise case-folding table.
            let c = if i % 2 == 0 { b.to_ascii_uppercase() } else { b.to_ascii_lowercase() };
            wr::<u8>(qp + i, c);
        }
        wr::<usize>(vp, qp);
        wr::<u64>(vp + 0x08, qsrc.len() as u64);

        let ro = fo(tp, vp);
        let rn = fn_(tp, vp);
        crate::diff_eq!(ro, rn, "find_draw_pass_bucket_nocase round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_find_named_entry_nocase(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1423_98a0);
    let mut cont = [0u8; 0x80];
    let mut objs = vec![0u8; 16 * 0xa0 + 8];
    let mut ptr_arr = [0usize; 16];
    let mut heap_strs = vec![0u8; 16 * 64 + 8];
    let mut query_buf = [0u8; 64];
    let mut view_buf = [0u8; 24];
    let mut cases = 0u32;

    const NAMES: &[&[u8]] = &[
        b"",
        b"g_vProjectionConstants",
        b"lightbuffer_diffuse",
        b"g_bPerDraw",
        b"MeshletBoundsBuffer_0",
    ];

    for round in 0..160u32 {
        let off = if round % 2 == 0 { 0 } else { 4 };
        let cp = cont.as_mut_ptr() as usize + off;
        let ob = objs.as_mut_ptr() as usize + off;
        let hp = heap_strs.as_mut_ptr() as usize + off;
        let vp = view_buf.as_mut_ptr() as usize + off;

        let count = match round {
            0 => 0usize,
            1 => 1,
            _ => (rng.below(12) as usize) + 1,
        };
        objs.fill(0);
        for i in 0..count {
            let op = ob + i * 0xa0;
            let name = NAMES[(i + round as usize) % NAMES.len()];
            write_test_inplace_string(op + 0x68, name, hp + i * 64, i % 2 == 0);
            ptr_arr[i] = op;
        }
        wr::<usize>(cp + 0x60, ptr_arr.as_ptr() as usize);
        wr::<u32>(cp + 0x68, count as u32);

        let qsrc: &[u8] = if count > 0 && round % 3 != 0 {
            NAMES[((rng.below(count as u64) as usize) + round as usize) % NAMES.len()]
        } else {
            b"unknown_prop_entry"
        };
        let qp = query_buf.as_mut_ptr() as usize + ((round as usize) % 3);
        for (i, &b) in qsrc.iter().enumerate() {
            let c = if (i + round as usize) % 2 == 0 { b.to_ascii_uppercase() } else { b.to_ascii_lowercase() };
            wr::<u8>(qp + i, c);
        }
        wr::<usize>(vp, qp);
        wr::<u64>(vp + 0x08, qsrc.len() as u64);

        let ro = fo(cp, vp);
        let rn = fn_(cp, vp);
        crate::diff_eq!(ro, rn, "find_named_entry_nocase round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_cmd_node_key_less(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1423_c303);
    let mut a_buf = [0u8; 24];
    let mut b_buf = [0u8; 24];
    let mut cases = 0u32;

    let special = [0u32, 1, 0x7fff_ffff, 0x8000_0000, u32::MAX];
    for &ka in &special {
        for &kb in &special {
            let ap = a_buf.as_mut_ptr() as usize + 1;
            let bp = b_buf.as_mut_ptr() as usize + 3;
            wr::<u32>(ap + 8, ka);
            wr::<u32>(bp + 8, kb);
            crate::diff_eq!(fo(ap, bp), fn_(ap, bp), "cmd_node_key_less special {ka:#x} vs {kb:#x}");
            cases += 1;
        }
    }
    for round in 0..100u32 {
        let ap = a_buf.as_mut_ptr() as usize + ((round as usize) % 5);
        let bp = b_buf.as_mut_ptr() as usize + ((round as usize + 2) % 5);
        wr::<u32>(ap + 8, rng.next_u32());
        wr::<u32>(bp + 8, rng.next_u32());
        crate::diff_eq!(fo(ap, bp), fn_(ap, bp), "cmd_node_key_less round {round}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_frustum_from_view_proj(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (fo, fn_): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1423_fd3c);
    let mut view_buf = vec![0u8; 0x260];
    let mut fr_o = vec![0u8; 0x1a0];
    let mut fr_n = vec![0u8; 0x1a0];
    let mut cases = 0u32;

    for round in 0..200u32 {
        let voff = if round % 2 == 0 { 0 } else { 4 };
        let foff = if round % 3 == 0 { 0 } else { 4 };
        let vp = view_buf.as_mut_ptr() as usize + voff;
        let fpo = fr_o.as_mut_ptr() as usize + foff;
        let fpn = fr_n.as_mut_ptr() as usize + foff;

        for i in 0..16 {
            let v = match round {
                0 => if i % 5 == 0 { 1.0 } else { 0.0 },
                1 => 0.0, // zero-length plane normal edge case (dot == 0 -> rsqrt = inf -> NR = NaN)
                _ => (rng.f32_any() as f64) * 0.75,
            };
            wr::<f64>(vp + 0x1d0 + i * 8, v);
        }
        fr_o.fill(0x55);
        fr_n.fill(0x55);

        fo(vp, fpo);
        fn_(vp, fpn);
        crate::diff_eq!(fr_o, fr_n, "frustum_from_view_proj round {round}");
        cases += 1;
    }
    Ok(cases)
}
