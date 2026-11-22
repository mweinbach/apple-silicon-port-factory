#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141c30fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1404b9990;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu128(r8, (xmm0) as u128);
    return crate::simd_ops::tail_fwd_xmm(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c31000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c316f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x68; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0xf0; const P11: u64 = 0xf4;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xf4u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xf8u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xf8u64), ((rax as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c31ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0xa0; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x61; const P11: u64 = 0x64; const P12: u64 = 0x1408265b0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P9))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P10))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P10), ((rax as u8)) as u8); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x64u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x70u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x74u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x74u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8cu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c32000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143265d08; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x64; const P11: u64 = 0x6c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x74u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x7cu64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x84u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x98u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xc0u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0xc8u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c32750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x80; const P4: u64 = 0x188; const P5: u64 = 0x18c; const P6: u64 = 0x190; const P7: u64 = 0x194;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c32800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143267550; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x80; const P10: u64 = 0x83; const P11: u64 = 0x84;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P9), (0xff000020u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P9), (0x20u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x20u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x88u64), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x190u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c32a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x80;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rdi = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64;
    r8 = ((rdi.wrapping_add(P3)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c32b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143263b10; const P1: u64 = 0x14020f270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_sub(0xffffffffffffff80u64)) as u64;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c32da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut rsi) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsi = 0xccccccfe; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c32e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x80;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = rdx;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rsi = r8;
    rcx = ((rbx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64;
    rdx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c33020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x80;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c330a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0xb; const P4: u64 = 0x38; const P5: u64 = 0x3c; const P6: u64 = 0x40; const P7: u64 = 0x143266988; const P8: u64 = 0x3b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x5u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P4), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P4), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    rax = P7;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x8u8) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c338e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143266078; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x38; const P6: u64 = 0x3c; const P7: u64 = 0x40; const P8: u64 = 0x3b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000005u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x5u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x5u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c34060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143264f68; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x38; const P6: u64 = 0x3c; const P7: u64 = 0x40; const P8: u64 = 0x3b; const P9: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000005u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x5u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x5u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c34760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x60; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x1408265b0; const P10: u64 = 0x90;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P10))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P10), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c34810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143266128; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60; const P11: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c35990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60; const P9: u64 = 0x64; const P10: u64 = 0x68;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c36130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x60; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0xe8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c361e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143266ca8; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60; const P11: u64 = 0x63;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P10), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P10), (0x10u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), (0x10u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(0x64u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x68u64), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0xe8u64), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0xe8u64), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(0xebu64), (0x10u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(0xecu64), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0xf0u64), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c36980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60; const P9: u64 = 0x68;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rax);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c37190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c379b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x68; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x64; const P11: u64 = 0xb0; const P12: u64 = 0x1408265b0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P11)) as u64) as u64; rcx = rdi.wrapping_add(0xb0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xf8u64))) as u64; rdx = rbx.wrapping_add(0x100); rcx = rdi.wrapping_add(0x100); crate::simd_ops::wu64(rdi.wrapping_add(0xf8u64), rax); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x130u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x130u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x134u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x134u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c38160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c38ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x68; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0x1408265b0; const P11: u64 = 0x98;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P9), rax);
    rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c38b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143265fc8; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x68; const P11: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x98u64), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c39d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x54; const P7: u64 = 0x58; const P8: u64 = 0x60;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c39da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143265f18; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x38; const P6: u64 = 0x3b; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000005u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x5u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000001u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c3a570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x60; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x140826310; const P10: u64 = 0x128; const P11: u64 = 0x12c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x12cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x130u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x130u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x134u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x134u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c3ad80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x38; const P3: u64 = 0x48; const P4: u64 = 0x68; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x58; const P9: u64 = 0x60; const P10: u64 = 0xf0; const P11: u64 = 0xf4;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((r8.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P8), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P9), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xf4u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c3b640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143264978; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x40; const P8: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c3bd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x30; const P2: u64 = 0x8; const P3: u64 = 0x14080c270; const P4: u64 = 0x40; const P5: u64 = 0x48; const P6: u64 = 0x140265ba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rdi.wrapping_add(P1)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32);
    rdx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c3bdb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x48; const P6: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000005u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x5u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x5u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c41910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x8c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rbx = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    rdi = r8;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P0)))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P0)))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c41970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x8c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rbx = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    rdi = r8;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c41bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x14240aa00; const P2: u64 = 0x8c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rbx = rdx;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c41c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x7; const P4: u64 = 0x8c; const P5: u64 = 0x90; const P6: u64 = 0x94; const P7: u64 = 0x8f;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P4), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P4), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P7), (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c42a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x48; const P2: u64 = 0x90; const P3: u64 = 0xd8; const P4: u64 = 0xd9;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P3), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c42ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x4b; const P7: u64 = 0x90; const P8: u64 = 0x94; const P9: u64 = 0x98; const P10: u64 = 0x93;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P7), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P7), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P9), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c430e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1403d79a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = ((rdx).wrapping_add(0x8u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu8(r8, ((rax as u8)) as u8);
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c43130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx, (0x1u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c446c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbb; const P1: u64 = 0xc0; const P2: u64 = 0xbc; const P3: u64 = 0xb8; const P4: u64 = 0x1424287f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r8 = ((rcx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x0u32) as u32);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; } else { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64;
    rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { r8 = (crate::simd_ops::ru64(r8)) as u64; }
    crate::simd_ops::wu8(r8, (0x0u8) as u8);
    return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c44730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xb8; const P2: u64 = 0x14240a960; const P3: u64 = 0x28; const P4: u64 = 0xc0; const P5: u64 = 0xbb; const P6: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(r14, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rax = 0x4; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P6), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c448f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142428d70; const P1: u64 = 0x14326a4b0; const P2: u64 = 0xb8; const P3: u64 = 0xbb; const P4: u64 = 0xbc; const P5: u64 = 0xc0;
    let (mut rax, mut r10, mut r11) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r10 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r11 = P1;
    rax = r10;
    crate::simd_ops::wu64(r10, r11);
    crate::simd_ops::lk_and_u32(r10.wrapping_add(P2), (0xff000002u32) as u32);
    crate::simd_ops::lk_or_u32(r10.wrapping_add(P2), (0x2u32) as u32);
    crate::simd_ops::wu8(r10.wrapping_add(P3), (0x2u8) as u8);
    crate::simd_ops::wu32(r10.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::wu8(r10.wrapping_add(P5), (0x0u8) as u8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c44d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = ((rdx).wrapping_add(0xb8u64)) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c44f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142428d70; const P1: u64 = 0x14326a4b0; const P2: u64 = 0xb8; const P3: u64 = 0xbb; const P4: u64 = 0xbc; const P5: u64 = 0xc0;
    let (mut rax, mut r10, mut r11) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = rdx;
    r10 = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r11 = P1;
    rax = r10;
    crate::simd_ops::wu64(r10, r11);
    crate::simd_ops::lk_and_u32(r10.wrapping_add(P2), (0xff000002u32) as u32);
    crate::simd_ops::lk_or_u32(r10.wrapping_add(P2), (0x2u32) as u32);
    crate::simd_ops::wu8(r10.wrapping_add(P3), (0x2u8) as u8);
    crate::simd_ops::wu32(r10.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::wu8(r10.wrapping_add(P5), (0x0u8) as u8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c461b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x1405a5010;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = ((rbx).wrapping_add(0x50u64)) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c47410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140853830;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = ((rdx).wrapping_add(0x50u64)) as u64;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4a700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x8; const P2: u64 = 0x14240aa00; const P3: u64 = 0x14028e490; const P4: u64 = 0x14183acbc;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rsi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rsi)) as u64;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4aa40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x50; const P2: u64 = 0x14240aa00; const P3: u64 = 0x14028e490; const P4: u64 = 0x14183acbc;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rsi = r8;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64;
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4aef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4b220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4b490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14326ad78; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x50; const P6: u64 = 0x53; const P7: u64 = 0x54; const P8: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000008u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x8u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4c670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rax = ((rax).wrapping_sub(1)) as u64;
    fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64;
    if (((fl_l as u64)) > ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rcx = ((rcx).wrapping_shl(3u32)) as u64;
    rax = ((rdx.wrapping_mul(8)) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P2), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4cf90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P2), (xmm0) as u128);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4d2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P2), (xmm0) as u128);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4d4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rdi.wrapping_add(P2), (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4d7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x58;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rbx).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(P3), ((rcx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4db20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50; const P3: u64 = 0x58;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rbx).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(P3), ((rcx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4dd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14326acc0; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x50; const P6: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000008u32) as u32);
    rax = P1;
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x8u32) as u32);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4e810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rdi).wrapping_add(P2), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4eb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = rdx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rbx = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdi).wrapping_add(P2), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c4ff20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rbx).wrapping_add(P2), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c50250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rbx).wrapping_add(P2), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c51410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x10; const P2: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(P1)))) as u32 as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = (if (((fl_l as u8)) != ((fl_r as u8))) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c51430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020c080; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = r8; rsi = rdx; rbx = rcx; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); r9 = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { bb = 12; continue; }
            10 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r8 = rdi; rdx = rsi; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c51560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14326e950; const P1: u64 = 0x142409b50; const P2: u64 = 0x1401f22d0; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10; const P5: u64 = 0x14326e970; const P6: u64 = 0x14326e998;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = rdx; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x10000010u64) as u64); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), ((rdi as u8)) as u8); rax = P0; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x1du64) as u64); r9 = ((rdi.wrapping_add(0x1du64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x0u8) as u8); rax = P5; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x24u64) as u64); r9 = 0x24; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x0u8) as u8); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x1cu64) as u64); r9 = 0x1c; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c519e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P0)))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c51a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c51c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    fl_l = ((rdx).wrapping_add(0xcu64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c51cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0xf; const P2: u64 = 0x10; const P3: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000004u32) as u32);
    rax = rdx;
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x4u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c53520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = ((r8.wrapping_add(P0)) as u64) as u64;
    rcx = ((rcx).wrapping_add(r8)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c53890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c53ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14326f968; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x13;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c53ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x48;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = rdi;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c54000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x48;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = rdi;
    rbx = ((rbx).wrapping_add(rdi)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c541f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x48;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rdi = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c54490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c547a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c54bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c54ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c57980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140728030;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c5eef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1408c2f90; const P2: u64 = 0x10; const P3: u64 = 0x1408b0590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    fl_l = r8; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c5ef50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x140267600; const P3: u64 = 0x1401e66f0; const P4: u64 = 0x1408b0810;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = ((rdi.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c5efb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x140267600; const P5: u64 = 0x1401e66f0; const P6: u64 = 0x1408b0810; const P7: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rsi = rbp; rsi = (((rsi) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = ((rsi.wrapping_add(P2)) as u64) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c5f050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x140267600; const P3: u64 = 0x1401e66f0; const P4: u64 = 0x1408b0810; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c612b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x11;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P0), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c61300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x11;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = rdx;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(r9.wrapping_add(rcx), rax);
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(r10.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    rdx = ((r10.wrapping_add(P2)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(r10.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c615b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu16(rdx.wrapping_add(P1), ((rax as u16)) as u16);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c62da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0xb;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P0), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P0), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c63470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x48; const P4: u64 = 0x4c; const P5: u64 = 0x50; const P6: u64 = 0x4b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u32(rdx, (0xff000008u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_or_u32(rdx, (0x8u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x8u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c63860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x6;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u16)) as u16);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c638a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x6;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r9 = rdx;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu16(r9.wrapping_add(rcx), ((rax as u16)) as u16);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64;
    crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c640b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); } else { crate::simd_ops::wu64(rdx.wrapping_add(P0), (0x0u64) as u64); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c64120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x8; const P3: u64 = 0x142442de0; const P4: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rdi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rsi.wrapping_add(P1))) as i64) as u64) as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(rsi).wrapping_add(P2))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(rsi).wrapping_add(P2), rbp); rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c641b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c65940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c66110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143275f10; const P1: u64 = 0x1426d01e0; const P2: u64 = 0x142409b50; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x160u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_l = rax; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 1; continue; }
            1 => { rdx = rax.wrapping_sub(0x8); r11 = rcx.wrapping_add(0x14); rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(0xfu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r11 = (crate::simd_ops::ru64(r11)) as u64; bb = 6; continue; }
            6 => { rcx = P0; r11 = ((r11).wrapping_sub(rcx)) as u64; fl_l = ((r11).wrapping_add(rdx)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; r8 = 0x8; rdx = rcx; r14 = P1; bb = 7; continue; }
            7 => { r8 = ((r8).wrapping_sub(1)) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(rdx))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 8; continue; }
            8 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 13; continue; }
            13 => { r11 = rbp.wrapping_add(0x14); r10 = ((crate::simd_ops::ru8(rbp.wrapping_add(0xfu64))) as u32) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((r10.wrapping_mul(8)) as u32) as u32 as u64; bb = 16; continue; }
            15 => { rdx = ((r10 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rsi = ((rax.wrapping_mul(8)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { r11 = (crate::simd_ops::ru64(r11)) as u64; bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_l = rdi; fl_r = (0x8u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = r11; fl_r = rcx; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = ((rdi as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rdx = ((rdx).wrapping_sub(1)) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(rcx))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 24; continue; } bb = 21; continue; }
            21 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 23; continue; }
            23 => { bb = 30; continue; }
            24 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x160u64), (0x1u32) as u32); rax = rbp.wrapping_add(0x14); fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { r10 = ((r10.wrapping_mul(8)) as u32) as u32 as u64; bb = 27; continue; }
            26 => { r10 = (((r10 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((r10 as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; bb = 27; continue; }
            27 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = ((r9).wrapping_add(0xfffffffffffffff8u64)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x30); rcx = rbp.wrapping_add(0xc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c662f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x13; const P2: u64 = 0x143275dd0; const P3: u64 = 0x1426d01e0; const P4: u64 = 0x143275dc8; const P5: u64 = 0x6; const P6: u64 = 0x142409b50;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r11 = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; r10 = r11.wrapping_add(0x8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r11)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru32(r11.wrapping_add(0x4u64))) as u32 as u64; fl_l = r8; fl_r = (0x6u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rdx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = r10; fl_r = rdx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rbx = P3; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rdx))) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rbx))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 8; continue; }
            8 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x6u64) as u64); r9 = ((r8.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); rcx = r11; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c663b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426d01e0; const P1: u64 = 0x1432127a8; const P2: u64 = 0x143275d8c; const P3: u64 = 0x143275d84; const P4: u64 = 0x143275d9c; const P5: u64 = 0x1401df700; const P6: u64 = 0x1432127b0; const P7: u64 = 0x143275d94; const P8: u64 = 0x143275dac; const P9: u64 = 0x143275da4; const P10: u64 = 0x1432127b4; const P11: u64 = 0x143275dc0; const P12: u64 = 0x1430f44f8; const P13: u64 = 0x143275db4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx.wrapping_add(0x9c); rbp = rcx; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdi = rbx.wrapping_add(0x8); fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((r9.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = ((r9 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rsi = ((rax.wrapping_mul(8)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; r14 = P0; fl_l = rdx; fl_r = (0x5u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { rcx = P1; r8 = ((rdx as u32)) as u32 as u64; fl_l = rdi; fl_r = rcx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(rcx))) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; r10 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 8; continue; }
            8 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { bb = 42; continue; }
            11 => { r10 = rbp.wrapping_add(0xa4); fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((r9.wrapping_mul(8)) as u32) as u32 as u64; bb = 14; continue; }
            13 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rdi = r10; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(r10)) as u64; bb = 16; continue; }
            16 => { fl_l = rdx; fl_r = (0x6u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 17; continue; }
            17 => { rcx = P2; r8 = rdx; fl_l = rdi; fl_r = rcx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 18; continue; }
            18 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(rcx))) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; r11 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r11 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 22; continue; } bb = 19; continue; }
            19 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 21; continue; }
            21 => { bb = 42; continue; }
            22 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { r9 = ((r9.wrapping_mul(8)) as u32) as u32 as u64; bb = 25; continue; }
            24 => { r9 = (((r9 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; bb = 25; continue; }
            25 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 27; continue; }
            27 => { fl_l = rdx; fl_r = (0x6u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 28; continue; }
            28 => { rcx = P3; fl_l = r10; fl_r = rcx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 29; continue; }
            29 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r14))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r14))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 33; continue; } bb = 30; continue; }
            30 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 33; continue; } bb = 31; continue; }
            31 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 32; continue; }
            32 => { bb = 42; continue; }
            33 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x6u64) as u64); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x3u64) as u64); rax = P6; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); rax = P9; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); rax = P10; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x7u64) as u64); rax = P11; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); rax = P12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x5u64) as u64); rax = P13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu32(rbp.wrapping_add(0x130u64), (0x1u32) as u32); bb = 43; continue; }
            43 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c66cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1408e89d0; const P2: u64 = 0x18; const P3: u64 = 0x1408b03b0; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x138u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c66f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rcx).wrapping_add(P0), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rcx).wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c67280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rdx).wrapping_add(P0), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rdx).wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c67460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P0), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c67620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143275eb0; const P1: u64 = 0x120; const P2: u64 = 0x1408e51e0; const P3: u64 = 0x130; const P4: u64 = 0x143275ea0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rbx = rcx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = rdi;
    r9 = P4;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c67770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143275eb0; const P1: u64 = 0x1408e51e0; const P2: u64 = 0x130; const P3: u64 = 0x143275ea0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    r9 = P0;
    rdx = ((rdx).wrapping_add(0x120u64)) as u64;
    rcx = r8;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = rdi;
    r9 = P3;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c67c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14240aa00; const P3: u64 = 0x94; const P4: u64 = 0x120; const P5: u64 = 0x1408e8390; const P6: u64 = 0x130;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(0xcu64)) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi.wrapping_add(P3)) as u64) as u64; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi.wrapping_add(P4)) as u64) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P6)) as u64) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c67cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432758d8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0xf; const P5: u64 = 0x94; const P6: u64 = 0x98; const P7: u64 = 0x9c; const P8: u64 = 0x97; const P9: u64 = 0x120; const P10: u64 = 0x128; const P11: u64 = 0x130;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x138u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c68380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x13; const P5: u64 = 0x98; const P6: u64 = 0x9c; const P7: u64 = 0xa0; const P8: u64 = 0x9b; const P9: u64 = 0x128; const P10: u64 = 0x12c; const P11: u64 = 0x130;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P9), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P9), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P11), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x12bu64), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(0x1b0u64), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(0x1b0u64), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(0x1b4u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x1b8u64), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x1b3u64), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x238u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c68420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x238; const P3: u64 = 0x142442de0; const P4: u64 = 0x1b0; const P5: u64 = 0x14240ab10; const P6: u64 = 0x128; const P7: u64 = 0x98; const P8: u64 = 0x10; const P9: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = 0; fl_l = 0; fl_r = 0; rbp = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = rbp; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), r14); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c68ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x13; const P5: u64 = 0x9c; const P6: u64 = 0xa0; const P7: u64 = 0xa4; const P8: u64 = 0x9f; const P9: u64 = 0x128; const P10: u64 = 0x130;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x10u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x10u8) as u8);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c68b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x128; const P3: u64 = 0x142442de0; const P4: u64 = 0x9c; const P5: u64 = 0x14240ab10; const P6: u64 = 0x10; const P7: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbp = 0; fl_l = 0; fl_r = 0; rsi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), rbp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c696c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64;
    if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c69980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0xc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c69be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x8; const P5: u64 = 0x143426fd0; const P6: u64 = 0x143427174; const P7: u64 = 0x4; const P8: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x7f7fffffu32) as u32); r8 = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0xff7fffffu32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; rcx = (((rdx) as i64).wrapping_mul((0x54u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(P6)) as u32) as u128; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rdx); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32) as u128; fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(r8.wrapping_add(P0), ((xmm0 as u32)) as u32); xmm4 = (xmm0) as u128; bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r8.wrapping_add(P1), ((xmm0 as u32)) as u32); xmm1 = (xmm0) as u128; bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r8.wrapping_add(P2), ((xmm0 as u32)) as u32); xmm3 = (xmm0) as u128; bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x14u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r8.wrapping_add(P3), ((xmm0 as u32)) as u32); xmm2 = (xmm0) as u128; bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x54u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c69cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x4; const P3: u64 = 0x18; const P4: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c69cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((0x54u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x54u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c69ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c69ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((rdx) as i64).wrapping_mul((0x54u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6a000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    rcx = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6a090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140940630; const P1: u64 = 0xc8; const P2: u64 = 0x1432778b0; const P3: u64 = 0x1406db330;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = rdi;
    r9 = P2;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6a1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140940630; const P1: u64 = 0xc8; const P2: u64 = 0x1432778b0; const P3: u64 = 0x1406db330;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rdi = r8;
    rcx = rbx;
    rdx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = rdi;
    r9 = P2;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6a680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140941970; const P1: u64 = 0xc0; const P2: u64 = 0xc4; const P3: u64 = 0xc8; const P4: u64 = 0x8; const P5: u64 = 0x1406f27c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rcx = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rbx = ((rbx).wrapping_add(0xc8u64)) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    fl_l = rcx; fl_r = rbx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6b0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14093f400; const P2: u64 = 0xc0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    r8 = ((rbx.wrapping_add(P2)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6b470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14093f400; const P2: u64 = 0xc0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rcx = rbx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6b880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143276a68; const P1: u64 = 0xcc; const P2: u64 = 0x143873de0; const P3: u64 = 0x143276a48; const P4: u64 = 0x143276a78; const P5: u64 = 0x1401de410;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0x3;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6ba40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143873de0; const P1: u64 = 0x143276a68; const P2: u64 = 0x143276a48; const P3: u64 = 0x143276a78; const P4: u64 = 0x1401de410;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0xccu64)) as u64;
    rcx = P3;
    r8 = 0x3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c6be90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140941970; const P1: u64 = 0xc0; const P2: u64 = 0xc8; const P3: u64 = 0xcc; const P4: u64 = 0xd0; const P5: u64 = 0xd4; const P6: u64 = 0xd8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = r8;
    rdi = r8;
    rbx = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rdi.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c85b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432a6b08; const P1: u64 = 0x1409ff500;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c86310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r9 = rcx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rcx).wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(P1), ((r8 as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c866f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P1), ((rcx as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c868e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c86940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143289e50; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3f800000u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x1u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x40c80000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c87550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x24; const P5: u64 = 0x28;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r10 = rdx; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; r9 = rcx; xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rcx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx).wrapping_add(P0), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(P1), ((r8 as u32)) as u32); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rcx = ((r9.wrapping_add(P3)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rax), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P0), ((rdx as u32)) as u32); rdx = ((r9.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r10))) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rcx = ((r9.wrapping_add(P5)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r10)) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rax), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P0), ((rdx as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c87970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x24; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    r9 = rdx;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P0), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P1), ((rcx as u32)) as u32);
    rdx = ((rdx).wrapping_add(0x14u64)) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = ((r9.wrapping_add(P3)) as u64) as u64;
    xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P0), ((rdx as u32)) as u32);
    rdx = ((r9.wrapping_add(P4)) as u64) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = ((r9.wrapping_add(P5)) as u64) as u64;
    xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P0), ((rdx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c87ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P6), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c87c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143288fc8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x2c; const P11: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3f800000u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x1u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x2u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x2u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), (0x2u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), (0xfa00u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), (0x41c80000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), (0x41c80000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), (0x41c80000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c87e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = ((crate::simd_ops::ru8(r9.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(r9).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P4), rax);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r9).wrapping_add(P5), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c88200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P3), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P4), rax);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rdx).wrapping_add(P5), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c88410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P5), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c88480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432941c8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3f800000u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x3f800000u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c88ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406f27c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x24u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x25u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x25u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x26u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x26u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x27u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x27u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x34u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x34u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x3cu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3du64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x3du64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3eu64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x3eu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x3fu64), ((rax as u8)) as u8); rax = rdx.wrapping_add(0x40); rcx = r8.wrapping_add(0x40); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x51u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x51u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x52u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x52u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x53u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x53u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x54u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x54u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x55u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x55u64), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c88cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432956e8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x38; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x24; const P9: u64 = 0x30; const P10: u64 = 0x34; const P11: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x3f800000u32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), (0x1010100u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), (0xbf800000u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), (0x10001u32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), ((rax as u32)) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(0x54u64), ((rax as u16)) as u16);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c895b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143288e70; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0xc; const P5: u64 = 0x14; const P6: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0xbu32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P4), (0x3e19999au64) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x3fe66666u32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), (0x3fb33333u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c897e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x1c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r11 = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r10 = rcx;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r11))) as u64;
    rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(P0), ((rcx as u32)) as u32);
    rdx = ((r10.wrapping_add(P3)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r11)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64;
    rax = ((crate::simd_ops::ru8(r10.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c89b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1c;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu32(r9.wrapping_add(rcx), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_add(P0), ((rcx as u32)) as u32);
    rdx = ((r10.wrapping_add(P2)) as u64) as u64;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(r10.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c89d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c89da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432996f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (0x42c80000u64) as u64);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8a520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x14240aa00; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x34; const P7: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rbx = rdx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8a5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14327fe18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x1b; const P7: u64 = 0x30; const P8: u64 = 0x38; const P9: u64 = 0x3c; const P10: u64 = 0x40; const P11: u64 = 0x3b;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000002u32) as u32);
    crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x2u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x2u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P8), (0xff000002u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P8), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu8(rdx.wrapping_add(P11), (0x2u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x50u64), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8ad20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x1406f27c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = ((r8.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8ad90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143297258; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0xbf800000u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8bb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu32(rdx, (0xbf800000u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0xbf800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0xbf800000u32) as u32);
    xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((xmm3 as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8e380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1c; const P3: u64 = 0x24; const P4: u64 = 0x28; const P5: u64 = 0x1406f27c0; const P6: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8); rax = ((rdx.wrapping_add(P4)) as u64) as u64; rcx = ((r8.wrapping_add(P4)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P6)) as u64) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8e440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426dd4; const P1: u64 = 0x8; const P2: u64 = 0x143298c50; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x14; const P6: u64 = 0x18; const P7: u64 = 0x20; const P8: u64 = 0x24; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x41200000u32) as u32);
    rax = P2;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0xc1200000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0xc1200000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0xc1200000u32) as u32);
    xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((xmm3 as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x0u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rax);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8ebb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143297100; const P1: u64 = 0x8; const P2: u64 = 0xb; const P3: u64 = 0xc; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000002u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x2u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8edd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rdx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r9))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8f0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rdx = ((rdx).wrapping_add(0xcu64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8f2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = ((rdx).wrapping_add(0xcu64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8f310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143281428; const P2: u64 = 0xc; const P3: u64 = 0xf; const P4: u64 = 0x10; const P5: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x0u8) as u8);
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P2), (0xff000002u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P2), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x2u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8fa20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f27c0; const P3: u64 = 0x20; const P4: u64 = 0x140265ba0; const P5: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c8fac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143280c38; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c90cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x20; const P3: u64 = 0x28;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rbx).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(P3), ((rcx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c90dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432a61a0; const P1: u64 = 0x14310b378; const P2: u64 = 0x1402c4e80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c91050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240aa00; const P2: u64 = 0x20; const P3: u64 = 0x28;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = ((rcx).wrapping_add(rdx)) as u64;
    rdi = r8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rbx).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(P3), ((rcx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c912c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143290bf8; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x20; const P6: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000002u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x2u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c92370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x2c; const P8: u64 = 0x30; const P9: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P7), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P9))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P9), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c92400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143296ba0; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x1c; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x2c; const P8: u64 = 0x34; const P9: u64 = 0x14; const P10: u64 = 0x18; const P11: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3d4ccccdu32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P9), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), (0x42c80000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c935f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432442c0; const P1: u64 = 0x1432a6268; const P2: u64 = 0x1402c4e80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c941a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140a35690;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c94820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x1409f9b00; const P2: u64 = 0x40; const P3: u64 = 0x1401de830; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x14240ab10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c94860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0xb; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x38; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx, (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P1), (0xff000002u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P1), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x2u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P5), (0x1u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu16(rdx.wrapping_add(P11), ((rax as u16)) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x54u64), (0x3dcccccdu32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x58u64), (0x42480000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x5cu64), (0xbf800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x64u64), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x68u64), (0x3u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x6cu64), (0x40000000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c948f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x78; const P3: u64 = 0x1409f9b00; const P4: u64 = 0x40; const P5: u64 = 0x1401de830; const P6: u64 = 0x28; const P7: u64 = 0x8; const P8: u64 = 0x14240ab10; const P9: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = rsi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c94970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x1409f9b00; const P2: u64 = 0x40; const P3: u64 = 0x1401de830; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x14240ab10; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c95110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x14; const P6: u64 = 0x14240aa00; const P7: u64 = 0x30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu8(r8, ((rax as u8)) as u8);
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdi = r8;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141c951a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x14240ab10; const P3: u64 = 0x142442c80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rbx = rdx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = rdi; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}
