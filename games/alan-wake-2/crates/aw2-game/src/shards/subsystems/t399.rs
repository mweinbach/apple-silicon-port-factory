#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ac0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x140fc37f0;
    let (mut rax, mut rbp, mut r13) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r13);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r13 = rdx;
    rbp = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbp = (((rbp) & (0xffffffffffffffc0u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    r13 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ac118(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x140fc3800;
    let (mut rax, mut rbp, mut r13) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r13);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r13 = rdx;
    rbp = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbp = (((rbp) & (0xffffffffffffffc0u64))) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64;
    rcx = ((rcx).wrapping_add(0x40u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    r13 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ac290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1b8; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x30; const P4: u64 = 0x28; const P5: u64 = 0x140571d50; const P6: u64 = 0x1a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425acfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140571d50; const P1: u64 = 0x14101f790; const P2: u64 = 0x14240ab10; const P3: u64 = 0x140c34930; const P4: u64 = 0x14243a340; const P5: u64 = 0x1401de3c0; const P6: u64 = 0x1413d1d70; const P7: u64 = 0x1424976f0; const P8: u64 = 0x14243bc40; const P9: u64 = 0x14249df20; const P10: u64 = 0x1401edac0; const P11: u64 = 0x1401ed720; const P12: u64 = 0x141021430; const P13: u64 = 0x141021bb0; const P14: u64 = 0x1403c3bd0; const P15: u64 = 0x140ff5db0; const P16: u64 = 0x141023260; const P17: u64 = 0x140ff5da0; const P18: u64 = 0x1403c4710; const P19: u64 = 0x141f79950; const P20: u64 = 0x140260f30; const P21: u64 = 0x14249e5d0; const P22: u64 = 0x14101f880;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x198u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x190u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xc40u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0xb0); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0xb0); fl_l = ((rcx).wrapping_add(0x110u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x410); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xf90u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0xb0); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0xb0); fl_l = ((rcx).wrapping_add(0x110u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x760); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x410); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x410); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x2c8); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x150); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x1d8); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x1d8); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x198u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x190u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0xc8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0xf8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x128u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x158u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x270u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x280u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x290u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x330u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x320u64))) as u64; fl_l = ((rcx).wrapping_add(0x2a4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ad290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x1401e1560; const P2: u64 = 0xb0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425aeca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020bdf0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x140fd52e0; const P4: u64 = 0x14109c910; const P5: u64 = 0x140280170; const P6: u64 = 0x1410a02f0; const P7: u64 = 0x141000da0; const P8: u64 = 0x14242ace0; const P9: u64 = 0x143cd9cd0; const P10: u64 = 0x1418349d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0xb8); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x100u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x100u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x180); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x188); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x20c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2188u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2330u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x24d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2680u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2828u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x29d0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2b78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2d20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2ec8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3070u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3218u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x33c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3568u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3710u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x38b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3a60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3c08u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3db0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3f58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4100u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x42a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4450u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x45f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x47a0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4948u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4af0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4c98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4e40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4fe8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5190u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5338u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x54e0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5688u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5830u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x59d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5b80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5d28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5ed0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6078u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6220u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x63c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6570u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6718u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x68c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6a68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6c10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6db8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6f60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7108u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x72b0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7458u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7600u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x77a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7950u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7af8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7ca0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7e48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7ff0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8198u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8340u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = P9; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425afc30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1e0; const P1: u64 = 0x140280170; const P2: u64 = 0x1410d7660; const P3: u64 = 0x828; const P4: u64 = 0x60; const P5: u64 = 0x68; const P6: u64 = 0x70; const P7: u64 = 0x78; const P8: u64 = 0x80; const P9: u64 = 0x88; const P10: u64 = 0x90; const P11: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b0310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x110; const P1: u64 = 0x1410ef180; const P2: u64 = 0x140226c00; const P3: u64 = 0x140226f40; const P4: u64 = 0x1401de400; const P5: u64 = 0x20; const P6: u64 = 0x1401e68b0; const P7: u64 = 0x30; const P8: u64 = 0x50; const P9: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xf0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x100u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b11d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411221e0; const P1: u64 = 0x1401f5a00; const P2: u64 = 0x1401f5a60; const P3: u64 = 0x1401f5a10; const P4: u64 = 0x141122220; const P5: u64 = 0x141122270; const P6: u64 = 0x1401f5ad0; const P7: u64 = 0x14240ab10; const P8: u64 = 0x1411222d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3d0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7b0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x9a0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0xa98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0xb90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0xc88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0xe98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0xfd8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1148u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1288u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x13c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1508u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1648u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1788u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x18c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1a38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1b78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1cb8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1df8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x1f68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x20a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x21e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2328u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2468u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x25a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x26e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2858u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2a68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2c78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2db8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2ef8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3038u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3178u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x32e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3428u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3638u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3848u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3a58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3b98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3cd8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3e18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x3f88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x40f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4238u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4448u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4588u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x46c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4808u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4948u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4a88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4bc8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4d08u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4e48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x4f88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x50c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5208u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5378u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x54b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x55f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5738u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5948u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5b58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5c98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5dd8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x5f48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c8u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2d0u64))) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6110u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6268u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6430u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6640u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6850u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6990u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6ad0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6c10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6d80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x6ec0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7000u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7140u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7280u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x73c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7500u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7640u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7808u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7948u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7a88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7bc8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7d08u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7e48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x7f88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8198u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x83a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x85b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x87c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x89d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8be8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8df8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x8ef0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx).wrapping_add(0x9100u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b1d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2f8; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x90; const P4: u64 = 0x1401e3aa0; const P5: u64 = 0x1f0; const P6: u64 = 0x78; const P7: u64 = 0x328; const P8: u64 = 0xb0; const P9: u64 = 0x220; const P10: u64 = 0x98; const P11: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b1e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1e0; const P1: u64 = 0x1401edac0; const P2: u64 = 0x1401ee260; const P3: u64 = 0xa0; const P4: u64 = 0x1401e3aa0; const P5: u64 = 0x20; const P6: u64 = 0x1401ff400; const P7: u64 = 0x30; const P8: u64 = 0x141152f10; const P9: u64 = 0x40; const P10: u64 = 0x58; const P11: u64 = 0x1401e1560; const P12: u64 = 0x1401de400;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x28); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b1e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x1401e5420; const P2: u64 = 0x1401e5380; const P3: u64 = 0x70; const P4: u64 = 0x14240ab10; const P5: u64 = 0x1401de3c0; const P6: u64 = 0xc0; const P7: u64 = 0x60; const P8: u64 = 0x1404b9d30; const P9: u64 = 0x58; const P10: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b2350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x820; const P1: u64 = 0x14243bc40; const P2: u64 = 0x230; const P3: u64 = 0x14249df20; const P4: u64 = 0x1070; const P5: u64 = 0x14240ab10; const P6: u64 = 0x2b0; const P7: u64 = 0x14243a340; const P8: u64 = 0x1a0; const P9: u64 = 0x1401de3c0; const P10: u64 = 0x60; const P11: u64 = 0x1413d1d70; const P12: u64 = 0x1424976f0; const P13: u64 = 0x1401edac0; const P14: u64 = 0x1401ed720; const P15: u64 = 0x141018740; const P16: u64 = 0x1401e68b0; const P17: u64 = 0x140571d50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x110u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x4d0); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x4d0); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x1070); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b24c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x260; const P1: u64 = 0x141196920; const P2: u64 = 0x1401de3c0; const P3: u64 = 0x1401e18a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xe8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x130u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b25b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208860; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x1;
    rdx = 0x40;
    rcx = rbp.wrapping_add(0x1b0);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b26d7(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203130; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x38;
    rcx = rbp.wrapping_add(0x1a0);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b2750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x50; const P3: u64 = 0x1401e7140; const P4: u64 = 0x68; const P5: u64 = 0x1401de3c0; const P6: u64 = 0x1401dff10; const P7: u64 = 0x180; const P8: u64 = 0x142483ab0; const P9: u64 = 0x70; const P10: u64 = 0x370; const P11: u64 = 0x1411d2d80; const P12: u64 = 0x1401e81c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x160u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b36b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140280170; const P1: u64 = 0x1401e5420; const P2: u64 = 0x1401fb4d0; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1401de3c0; const P5: u64 = 0x1411b7420; const P6: u64 = 0x140aade60; const P7: u64 = 0x1411a4f70; const P8: u64 = 0x1411b7400; const P9: u64 = 0x1401e5380; const P10: u64 = 0x1411b7160; const P11: u64 = 0x1404b9d30; const P12: u64 = 0x1401de400; const P13: u64 = 0x142028d90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x1a0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0xd8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x118u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b3dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x1411d2db0; const P2: u64 = 0x1401de400; const P3: u64 = 0x1411d2e50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b4150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1a8; const P1: u64 = 0x140571d50; const P2: u64 = 0xe0; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1a0; const P5: u64 = 0x40; const P6: u64 = 0x1401de3c0; const P7: u64 = 0x20; const P8: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b454c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403496f0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x18;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b457a(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403496f0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x18;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b45a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1403496f0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    rcx = ((rcx).wrapping_add(0x78u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x18;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b49e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d5bf0; const P1: u64 = 0x1401e68b0; const P2: u64 = 0x1411db860; const P3: u64 = 0x1411dd9e0; const P4: u64 = 0x1401de400; const P5: u64 = 0x1411d2b20; const P6: u64 = 0x140571d50; const P7: u64 = 0x1401dff10; const P8: u64 = 0x1423f0420; const P9: u64 = 0x140208fd0; const P10: u64 = 0x1401de3c0; const P11: u64 = 0x1401e18a0; const P12: u64 = 0x1411ddc00; const P13: u64 = 0x1411ddbd0; const P14: u64 = 0x1411ddba0; const P15: u64 = 0x1411ddb70; const P16: u64 = 0x1411ddb00; const P17: u64 = 0x1411dda10; const P18: u64 = 0x140226f40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x150); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x88); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0xd8); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0xf8); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x7a0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = rdx.wrapping_add(0x7a0); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = rdx.wrapping_add(0x270); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = rdx.wrapping_add(0x270); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x690); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x690); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0xe0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x140u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; fl_l = ((rcx).wrapping_add(0x148u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b4e37(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208fd0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64;
    r9 = P0;
    r8 = 0xc;
    rdx = 0x120;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b4e87(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208fd0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rcx = ((rcx).wrapping_add(0xff0u64)) as u64;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x120;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b4eb8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208fd0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rcx = ((rcx).wrapping_add(0x1350u64)) as u64;
    r9 = P0;
    r8 = 0xc;
    rdx = 0x120;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b4ee9(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208fd0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rcx = ((rcx).wrapping_add(0x20d0u64)) as u64;
    r9 = P0;
    r8 = 0xc;
    rdx = 0x120;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b4f1a(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208fd0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rcx = ((rcx).wrapping_add(0x2e50u64)) as u64;
    r9 = P0;
    r8 = 0xc;
    rdx = 0x120;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b4f71(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208fd0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    r9 = P0;
    r8 = 0xc;
    rdx = 0x120;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b5274(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d2b20; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x338u64))) as u64;
    r9 = P0;
    r8 = 0x5;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b5614(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d2db0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b5920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420888b0; const P1: u64 = 0x142089f90; const P2: u64 = 0x142085fa0; const P3: u64 = 0x142085f70; const P4: u64 = 0x142085f40; const P5: u64 = 0x1401e68b0; const P6: u64 = 0x1411d5bf0; const P7: u64 = 0x1401dff10; const P8: u64 = 0x1401e54f0; const P9: u64 = 0x143ce687c; const P10: u64 = 0x1418349d0; const P11: u64 = 0x1411dfa50; const P12: u64 = 0x142087400; const P13: u64 = 0x1420878a0; const P14: u64 = 0x1411db860; const P15: u64 = 0x1420878f0; const P16: u64 = 0x1411dfb80; const P17: u64 = 0x142088680; const P18: u64 = 0x142089030; const P19: u64 = 0x142089bf0; const P20: u64 = 0x1411dfb70; const P21: u64 = 0x1420851b0; const P22: u64 = 0x14208aa10; const P23: u64 = 0x142085430; const P24: u64 = 0x14208aa00; const P25: u64 = 0x14208a9d0; const P26: u64 = 0x1401e1560; const P27: u64 = 0x1420881c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0xc8); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0xb8); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x178); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x188); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P9; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0xb0); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x98); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xb8u64))) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = rdx.wrapping_add(0x228); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x28); return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P24, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0xc8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P25, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; return crate::simd_ops::call_fwd(P26, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; return crate::simd_ops::call_fwd(P26, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; return crate::simd_ops::call_fwd(P26, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = rdx.wrapping_add(0x88); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x28); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b5e62(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rbp = rdx;
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b6390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    r9 = P0;
    r8 = 0x8;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b63f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401df360; const P1: u64 = 0x14209a500; const P2: u64 = 0x14209a430; const P3: u64 = 0x8; const P4: u64 = 0x14397fd28; const P5: u64 = 0x1411e3d70; const P6: u64 = 0x1411e3c20; const P7: u64 = 0x142498f00; const P8: u64 = 0x14183acce; const P9: u64 = 0x60; const P10: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1425b63f0, funcinfo_rva: 0x379b7f4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rbp = (rdx) as u64; rcx = ((rbp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b6402); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0xf0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b640f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1d0u64))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b641e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rax as u32)) as u32 as u64; rdx = ((rbp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b6429); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0xf0u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1d8u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b643f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P4)) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b644d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x1c0u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b6460); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x1c8u64), (rax) as u64); rcx = ((rbp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b646e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm0) as u128); rax = ((rbp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbp.wrapping_add(0xf0u64)) as u64) as u64; r8 = ((rbp.wrapping_add(0x1c0u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x1c8u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b649e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b64a9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b64b2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rdx.wrapping_add(P9)) as u64) as u64; drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b67b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x268; const P1: u64 = 0x14240ab10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b68d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x1411f11c0; const P2: u64 = 0x160; const P3: u64 = 0x1420ba180; const P4: u64 = 0x40; const P5: u64 = 0x1401e68b0; const P6: u64 = 0x30; const P7: u64 = 0x20; const P8: u64 = 0x98; const P9: u64 = 0x1401de410; const P10: u64 = 0x88; const P11: u64 = 0x200; const P12: u64 = 0x1401dff10; const P13: u64 = 0x1401de400; const P14: u64 = 0x1420be2e0; const P15: u64 = 0x1420c3b10; const P16: u64 = 0x140348ab0; const P17: u64 = 0x1420b9d80; const P18: u64 = 0x1420b8780; const P19: u64 = 0x140571d50; const P20: u64 = 0x1401e4ed0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x158); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x690); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x328); return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x7e0); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x2e0); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x148); return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b6dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6930; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401e4ed0; const P3: u64 = 0x1420e6350; const P4: u64 = 0x14039f010;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0xf8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x170u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x1e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x260u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x2d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x350u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x3c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x440u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x4b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0xf8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x170u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x1e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x260u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x2d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x350u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x3c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x440u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x4b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x208u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0xb8); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x150); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x208u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0xe8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x1d8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0xf8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = rdx.wrapping_add(0x250); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x170u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = rdx.wrapping_add(0x2c8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x1e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = rdx.wrapping_add(0x340); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x260u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x3b8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x2d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = rdx.wrapping_add(0x430); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x350u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x4a8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x3c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = rdx.wrapping_add(0x520); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x440u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x598); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0xc0); fl_l = ((rcx).wrapping_add(0x4b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x610); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((rcx as u32)) as u32);
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7428(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210de90;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1425b7428, funcinfo_rva: 0x37a4a5c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x28, conts: &[0x1425b7452] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b743c); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1425b743c, ({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425b7452 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 1; continue; } _ => l.unknown(0x1425b7428) } }; rdx = (rax) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1425b7446); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1425b7446, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x1425b7452 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 1; continue; } _ => l.unknown(0x1425b7428) } }; crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x2u32) as u32); bb = 1; continue; }
            1 => { rax = (0x0u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7463(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x4u32) as u32);
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x1401de400; const P2: u64 = 0x30; const P3: u64 = 0x1411fa420; const P4: u64 = 0x148; const P5: u64 = 0x1411f9e70; const P6: u64 = 0x1411fa3d0; const P7: u64 = 0xb8; const P8: u64 = 0x1411f9e90; const P9: u64 = 0x1a8; const P10: u64 = 0x38; const P11: u64 = 0x80; const P12: u64 = 0x1411fada0; const P13: u64 = 0x1411fae70; const P14: u64 = 0x1401e1800;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b75f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x1411f9e90; const P2: u64 = 0x40; const P3: u64 = 0x141202090; const P4: u64 = 0x30; const P5: u64 = 0x141200250; const P6: u64 = 0x141202080; const P7: u64 = 0x50; const P8: u64 = 0x1411fb6c0; const P9: u64 = 0x141202260; const P10: u64 = 0x141202070;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf8; const P1: u64 = 0x1412323a0; const P2: u64 = 0xf0; const P3: u64 = 0x141232380; const P4: u64 = 0x1411f9e70; const P5: u64 = 0x40; const P6: u64 = 0x141232420; const P7: u64 = 0x20; const P8: u64 = 0x1412322f0; const P9: u64 = 0x80; const P10: u64 = 0x1401e68b0; const P11: u64 = 0x1c0; const P12: u64 = 0x1411fa150; const P13: u64 = 0x1411fb6c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7798(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411fb6c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = rbp.wrapping_add(0x40);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b77ee(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411fb6c0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = rbp.wrapping_add(0xa0);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b78f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x130; const P1: u64 = 0x14123a1d0; const P2: u64 = 0x1401e1590;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x200; const P1: u64 = 0x14123adc0; const P2: u64 = 0x20; const P3: u64 = 0x142126d80; const P4: u64 = 0x68; const P5: u64 = 0x1403e5be0; const P6: u64 = 0x168; const P7: u64 = 0x1403e6fe0; const P8: u64 = 0x90; const P9: u64 = 0x142126d50; const P10: u64 = 0x58; const P11: u64 = 0x1401e68b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b79e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x1411f9e90; const P2: u64 = 0x1411fa420; const P3: u64 = 0x30; const P4: u64 = 0x141202490; const P5: u64 = 0x141200610; const P6: u64 = 0x141232da0; const P7: u64 = 0x141232e20; const P8: u64 = 0x38; const P9: u64 = 0x1411f9e70; const P10: u64 = 0x141201310;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x110u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0x128u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x14213b180; const P2: u64 = 0xf0; const P3: u64 = 0x14216fdc0; const P4: u64 = 0x88; const P5: u64 = 0x1401e68b0; const P6: u64 = 0x40; const P7: u64 = 0xe0; const P8: u64 = 0x90; const P9: u64 = 0x100; const P10: u64 = 0x1421326f0; const P11: u64 = 0xa0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b7f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e68b0; const P1: u64 = 0x141248ab0; const P2: u64 = 0x14214e140; const P3: u64 = 0x141248da0; const P4: u64 = 0x14214e500; const P5: u64 = 0x141248db0; const P6: u64 = 0x14214dad0; const P7: u64 = 0x140571d50; const P8: u64 = 0x1401de400; const P9: u64 = 0x1421570e0; const P10: u64 = 0x1401de3c0; const P11: u64 = 0x1424b5180; const P12: u64 = 0x1401ee260; const P13: u64 = 0x1417fbd70; const P14: u64 = 0x141249790; const P15: u64 = 0x142150c90; const P16: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x1a0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0xc8); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x180); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b811c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401df680; const P1: u64 = 0x1401df9f0; const P2: u64 = 0x142490450; const P3: u64 = 0x142490500;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = rdx;
    rcx = rbp.wrapping_add(0xac);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rax;
    rcx = rbp.wrapping_add(0x1d0);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbp.wrapping_add(0x70u64), (xmm0) as u128);
    rcx = rbp.wrapping_add(0x70);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rbp.wrapping_add(0xac);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = rax;
    rcx = rbp.wrapping_add(0x50);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rbp.wrapping_add(0x80u64), (xmm0) as u128);
    rcx = rbp.wrapping_add(0x80);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b85b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x340; const P1: u64 = 0x1401e68b0; const P2: u64 = 0x338; const P3: u64 = 0x60; const P4: u64 = 0x330; const P5: u64 = 0x141289c20; const P6: u64 = 0x88; const P7: u64 = 0x110; const P8: u64 = 0x120; const P9: u64 = 0x180; const P10: u64 = 0x140; const P11: u64 = 0x540; const P12: u64 = 0x141289740; const P13: u64 = 0x141289780; const P14: u64 = 0x1412897c0; const P15: u64 = 0x1412891d0; const P16: u64 = 0x1412896b0; const P17: u64 = 0x141289190; const P18: u64 = 0x140571d50; const P19: u64 = 0x1401e1560; const P20: u64 = 0x14128cae0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x210); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x460u64))) as u64; return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x150); return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b8730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d0; const P1: u64 = 0x14128dcd0; const P2: u64 = 0x4d0; const P3: u64 = 0x14128dc90; const P4: u64 = 0x310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425b9123(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x5a0u64))) as u64;
    crate::simd_ops::wu32(rax, (0x29u32) as u32);
    crate::simd_ops::wu8(rax.wrapping_add(P1), (0x0u8) as u8);
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425ba570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x108; const P1: u64 = 0x14219d260; const P2: u64 = 0x40; const P3: u64 = 0x14020b2c0; const P4: u64 = 0xa0; const P5: u64 = 0x14240ab10; const P6: u64 = 0x1401de3c0; const P7: u64 = 0x50; const P8: u64 = 0x20; const P9: u64 = 0x1401de400; const P10: u64 = 0xf0; const P11: u64 = 0x1411d2d80; const P12: u64 = 0x1401fb4d0; const P13: u64 = 0x1402128c0; const P14: u64 = 0x1401e1800; const P15: u64 = 0x142498af0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x280u64))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x120); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x120); fl_l = ((rcx).wrapping_add(0x118u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x260); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x260); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x80); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x88); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bc120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x140260f30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bc65c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf770; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bc890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x14240ab10; const P2: u64 = 0x218; const P3: u64 = 0x1412cf2f0; const P4: u64 = 0x143cef4bc; const P5: u64 = 0x1418349d0; const P6: u64 = 0x143cef3ac; const P7: u64 = 0x143cef3d0; const P8: u64 = 0x143cef3a8; const P9: u64 = 0x143cef3b0; const P10: u64 = 0x143cef470; const P11: u64 = 0x143cef4b8; const P12: u64 = 0x1411d5bf0; const P13: u64 = 0x1420a0310;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P4; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P6; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P7; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P8; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P9; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P10; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P11; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bd1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x160; const P1: u64 = 0x1411db190; const P2: u64 = 0x70; const P3: u64 = 0x1412e6d50; const P4: u64 = 0x1401de400; const P5: u64 = 0x1401de3c0; const P6: u64 = 0x310; const P7: u64 = 0x14240ab10; const P8: u64 = 0xd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P6)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bd3a2(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411df630; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    rcx = ((rcx).wrapping_add(0xc8u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bd508(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x100; const P2: u64 = 0x108; const P3: u64 = 0x4; const P4: u64 = 0xe; const P5: u64 = 0x20; const P6: u64 = 0x18; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x2a8u64))) as u64; fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r9.wrapping_add(P0)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(P2)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P5), r9); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdx.wrapping_add(P6), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (0x0u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bd5b1(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412e8860; const P1: u64 = 0x14183acce; const P2: u64 = 0x20; const P3: u64 = 0x1401de400;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2a8u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rdx.wrapping_add(P2)) as u64) as u64;
    fl_l = ((rcx).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bd610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf2f0; const P1: u64 = 0x1412cf610; const P2: u64 = 0x143cf0b48; const P3: u64 = 0x1418349d0; const P4: u64 = 0x143cf0a64; const P5: u64 = 0x14052f2f0; const P6: u64 = 0x143cf0548; const P7: u64 = 0x143cf0690; const P8: u64 = 0x143cf0b88; const P9: u64 = 0x143cf0454; const P10: u64 = 0x143cf1008; const P11: u64 = 0x143cf0bb8; const P12: u64 = 0x143cf0c08; const P13: u64 = 0x143cf09f0; const P14: u64 = 0x143cf05b4; const P15: u64 = 0x143cf0bb4; const P16: u64 = 0x143cf0d40; const P17: u64 = 0x143cf0c30; const P18: u64 = 0x143cf0c3c; const P19: u64 = 0x143cf07c8; const P20: u64 = 0x143cf080c; const P21: u64 = 0x143cf0414; const P22: u64 = 0x143cf0cc0; const P23: u64 = 0x143cf0410; const P24: u64 = 0x143cf035c; const P25: u64 = 0x143cf0538; const P26: u64 = 0x143cf0808; const P27: u64 = 0x143cf0570; const P28: u64 = 0x143cf0b20; const P29: u64 = 0x143cf0398; const P30: u64 = 0x143cf0d48; const P31: u64 = 0x143cf0530; const P32: u64 = 0x143cf0540; const P33: u64 = 0x143cf0af8; const P34: u64 = 0x143cf0c80; const P35: u64 = 0x143cf0c40; const P36: u64 = 0x143cf04c8; const P37: u64 = 0x143cf0be0; const P38: u64 = 0x143cf0980; const P39: u64 = 0x143cf0a60; const P40: u64 = 0x143cf0830; const P41: u64 = 0x143cf1000; const P42: u64 = 0x143cf0508; const P43: u64 = 0x143cf0510; const P44: u64 = 0x143cf06f0; const P45: u64 = 0x143cf0ad0; const P46: u64 = 0x143cf0c38; const P47: u64 = 0x143cf04e8; const P48: u64 = 0x143cf0900; const P49: u64 = 0x143cf0a18; const P50: u64 = 0x143cf0844; const P51: u64 = 0x143cf083c; const P52: u64 = 0x143cf0c34; const P53: u64 = 0x143cf0c0c; const P54: u64 = 0x143cf0364; const P55: u64 = 0x143cf1028; const P56: u64 = 0x143cf0544; const P57: u64 = 0x143cf0360; const P58: u64 = 0x143cf0c2c; const P59: u64 = 0x143cf1050; const P60: u64 = 0x143cf05b0; const P61: u64 = 0x143cf09a8; const P62: u64 = 0x143cf1004; const P63: u64 = 0x143cf0358; const P64: u64 = 0x143cf0f38; const P65: u64 = 0x143cf0840; const P66: u64 = 0x143cf0ef0; const P67: u64 = 0x140571d50; const P68: u64 = 0x143cf0d44; const P69: u64 = 0x143cf0bb0; const P70: u64 = 0x143cf0948; const P71: u64 = 0x143cf0b24; const P72: u64 = 0x143cf0330; const P73: u64 = 0x143cf0d74; const P74: u64 = 0x143cf10c0; const P75: u64 = 0x143cf1058; const P76: u64 = 0x143cf053c; const P77: u64 = 0x143cf0860; const P78: u64 = 0x143cf1054; const P79: u64 = 0x143cf0450; const P80: u64 = 0x143cf0dc0; const P81: u64 = 0x142201e30; const P82: u64 = 0x143cf050c; const P83: u64 = 0x143cf0308; const P84: u64 = 0x143cf0d70; const P85: u64 = 0x143cf0880; const P86: u64 = 0x143cf105c; const P87: u64 = 0x143cf06f4; const P88: u64 = 0x143cf0838; const P89: u64 = 0x143cf0f3c; const P90: u64 = 0x143cf0834; const P91: u64 = 0x143cf0f40; const P92: u64 = 0x143cf10ec; const P93: u64 = 0x143cf0534; const P94: u64 = 0x143cf030c; const P95: u64 = 0x1411d5bf0; const P96: u64 = 0x1401de400; const P97: u64 = 0x1401e46d0; const P98: u64 = 0x1401de3c0; const P99: u64 = 0x142498af0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x330); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P2; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x2c0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P7; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P8; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P9; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P10; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P11; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P12; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P13; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P14; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = P15; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P16; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P17; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P18; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P19; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P20; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P21; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P22; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P23; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P24; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P25; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P26; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P27; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = P28; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = P29; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = P30; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = P31; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = P32; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = P33; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = P34; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = P35; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P36; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = P37; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = P38; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P39; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P40; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = P41; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P42; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P43; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = P44; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P45; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = P46; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = P47; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = P48; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = P49; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = P50; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = P51; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = P52; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = rdx.wrapping_add(0x2e8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = P53; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = P54; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = P55; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x2f8); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = P56; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = P57; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = P58; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = P59; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = P60; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = P61; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = P62; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = P63; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = P64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = P65; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = P66; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P67, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x650); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = P68; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = P69; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = P70; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = P71; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = P72; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = P73; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = P74; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = P75; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = P76; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = P77; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = P78; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = P79; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = P80; return crate::simd_ops::call_fwd(P81, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = P82; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = P83; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = P84; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = P85; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = P86; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = P87; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = P88; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = rdx.wrapping_add(0x41); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = P89; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = P90; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = P91; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = P92; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = P93; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = P94; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P95, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P96, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P97, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P97, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P67, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = rdx.wrapping_add(0x370); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P98, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = rdx.wrapping_add(0x370); return crate::simd_ops::call_fwd(P99, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = rdx.wrapping_add(0x220); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P98, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = rdx.wrapping_add(0x220); return crate::simd_ops::call_fwd(P99, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = rdx.wrapping_add(0xd0); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P98, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P99, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bdd3e(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x58u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bdd6f(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x78u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bdda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x1411db190;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x98u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xa8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bddd9(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rcx = ((rcx).wrapping_add(0xc8u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bde33(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x118u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bde7a(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x130u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bdf24(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x8;
    rcx = rbp.wrapping_add(0x190);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425be009(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ea880; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x2f0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425be868(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425be896(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rcx = ((rcx).wrapping_add(0x70u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425be8c4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rcx = ((rcx).wrapping_add(0x90u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bea00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x158; const P1: u64 = 0x1401dff10; const P2: u64 = 0x20; const P3: u64 = 0x14204db80; const P4: u64 = 0x24;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bed28(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x68u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bed59(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142256ee0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    rcx = ((rcx).wrapping_add(0xa8u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x48;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bed8d(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf770; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bee04(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f5b10; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x2c8u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425beea4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf770; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425befc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140280170; const P1: u64 = 0x1410d7660; const P2: u64 = 0x1422589c0; const P3: u64 = 0x1401e2110; const P4: u64 = 0x1412cf2f0; const P5: u64 = 0x1401e68b0; const P6: u64 = 0x1401de400; const P7: u64 = 0x1412f7d50; const P8: u64 = 0x140f5c7e0; const P9: u64 = 0x140d54540; const P10: u64 = 0x1412f7dc0; const P11: u64 = 0x1412f7ec0; const P12: u64 = 0x143cf2958; const P13: u64 = 0x1418349d0; const P14: u64 = 0x143cf2960; const P15: u64 = 0x143cf2954; const P16: u64 = 0x143cf28c0; const P17: u64 = 0x143cf28c4; const P18: u64 = 0x1412f7c10; const P19: u64 = 0x1412f7c40; const P20: u64 = 0x140246cd0; const P21: u64 = 0x1411db190; const P22: u64 = 0x14240ab10; const P23: u64 = 0x143cf2800; const P24: u64 = 0x143cf29b0; const P25: u64 = 0x143cf29d4; const P26: u64 = 0x143cf2950; const P27: u64 = 0x143cf29d0; const P28: u64 = 0x143cf28b8; const P29: u64 = 0x14052f2f0; const P30: u64 = 0x143cf28bc; const P31: u64 = 0x143cf2900; const P32: u64 = 0x143cf2880; const P33: u64 = 0x143cf28e0; const P34: u64 = 0x143cf295c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0xa8); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x130u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x140u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x150u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x160u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x180u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x2d0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x600u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x618u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x638u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x650u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x668u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x680u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x698u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x6b0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x6c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x6d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x6e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x60u64))) as u64; fl_l = ((rcx).wrapping_add(0x6f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = P12; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P14; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P15; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = P16; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P17; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0xd8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x180u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x220u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P19, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x2f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x308u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; fl_l = ((rcx).wrapping_add(0x318u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P22, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = P23; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = P24; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = P25; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = P26; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = P27; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = P28; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = P30; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = P31; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = P32; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = P33; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = P34; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bf778(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x8;
    rcx = rbp.wrapping_add(0x170);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bf7a5(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x8;
    rcx = rbp.wrapping_add(0x188);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425bf7d2(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x8;
    rcx = rbp.wrapping_add(0x1c0);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c0f7e(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f9f30; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x910u64))) as u64;
    rcx = ((rcx).wrapping_add(0x388u64)) as u64;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c0fb2(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412f9f30; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x910u64))) as u64;
    rcx = ((rcx).wrapping_add(0x3a0u64)) as u64;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c1290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x10;
    rcx = rbp.wrapping_add(0x60);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c12e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc70; const P1: u64 = 0x140fbd5e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c17c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x1401de400; const P2: u64 = 0x1411db190; const P3: u64 = 0x90; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rbp = (((((rbp as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx).wrapping_add(0x4c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x4d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x4e0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x4f0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x500u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x508u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x538u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c18b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x170; const P1: u64 = 0x1401de400; const P2: u64 = 0x70; const P3: u64 = 0x1401e68b0; const P4: u64 = 0xc0; const P5: u64 = 0x88; const P6: u64 = 0x1423306e0; const P7: u64 = 0x180; const P8: u64 = 0x141308a00; const P9: u64 = 0xa0; const P10: u64 = 0x1413089b0; const P11: u64 = 0x30; const P12: u64 = 0x1411d5bf0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xe0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xf0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P11))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c19c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cf5eb4; const P1: u64 = 0x1418349d0; const P2: u64 = 0x143cf5ff8; const P3: u64 = 0x143cf5c84; const P4: u64 = 0x143cf5e88; const P5: u64 = 0x143cf605c; const P6: u64 = 0x143cf58d4; const P7: u64 = 0x143cf5ed8; const P8: u64 = 0x143cf5ccc; const P9: u64 = 0x143cf5dc0; const P10: u64 = 0x143cf5f60; const P11: u64 = 0x143cf5eac; const P12: u64 = 0x143cf5b28; const P13: u64 = 0x143cf5c24; const P14: u64 = 0x1412cf2f0; const P15: u64 = 0x143cf60a4; const P16: u64 = 0x143cf5bdc; const P17: u64 = 0x143cf5818; const P18: u64 = 0x143cf5bf4; const P19: u64 = 0x143cf5a54; const P20: u64 = 0x143cf5eb8; const P21: u64 = 0x143cf6058; const P22: u64 = 0x143cf61c0; const P23: u64 = 0x143cf5ca0; const P24: u64 = 0x143cf5bf0; const P25: u64 = 0x143cf5bc8; const P26: u64 = 0x143cf5be0; const P27: u64 = 0x14052f2f0; const P28: u64 = 0x143cf58d0; const P29: u64 = 0x1401e1560; const P30: u64 = 0x143cf5d78; const P31: u64 = 0x143cf60a0; const P32: u64 = 0x143cf5c20; const P33: u64 = 0x143cf5be8; const P34: u64 = 0x143cf5c60; const P35: u64 = 0x143cf5da0; const P36: u64 = 0x143cf57b8; const P37: u64 = 0x143cf5b48; const P38: u64 = 0x143cf5f68; const P39: u64 = 0x143cf5cc8; const P40: u64 = 0x143cf61c4; const P41: u64 = 0x143cf5bd0; const P42: u64 = 0x143cf5a14; const P43: u64 = 0x143cf5d7c; const P44: u64 = 0x143cf5bfc; const P45: u64 = 0x143cf59b0; const P46: u64 = 0x143cf5c80; const P47: u64 = 0x143cf5be4; const P48: u64 = 0x143cf61a0; const P49: u64 = 0x143cf5bf8; const P50: u64 = 0x143cf60f8; const P51: u64 = 0x143cf5cd0; const P52: u64 = 0x143cf5e68; const P53: u64 = 0x143cf5e6c; const P54: u64 = 0x143cf5e8c; const P55: u64 = 0x143cf5a10; const P56: u64 = 0x143cf5a50; const P57: u64 = 0x143cf6060; const P58: u64 = 0x143cf5cf0; const P59: u64 = 0x143cf5de8; const P60: u64 = 0x143cf5bd4; const P61: u64 = 0x143cf61a4; const P62: u64 = 0x143cf5c00; const P63: u64 = 0x143cf5bc4; const P64: u64 = 0x143cf5ac8; const P65: u64 = 0x143cf5bc0; const P66: u64 = 0x143cf58a8; const P67: u64 = 0x143cf5ca4; const P68: u64 = 0x143cf5b80; const P69: u64 = 0x143cf5bec; const P70: u64 = 0x143cf5a30; const P71: u64 = 0x143cf5eb0; const P72: u64 = 0x143cf5f64; const P73: u64 = 0x143cf5bcc; const P74: u64 = 0x143cf5bd8; const P75: u64 = 0x143cf5ea8; const P76: u64 = 0x143cf5880; const P77: u64 = 0x143cf5884; const P78: u64 = 0x143cf5910; const P79: u64 = 0x143cf5cf4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = P0; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P8; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P9; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P10; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P11; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P12; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P13; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P15; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = P16; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P17; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P18; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P19; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P20; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P21; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P22; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P23; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P24; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P25; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P26; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x138); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = P28; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x188); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x170u64))) as u64; return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = P30; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P31; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = P32; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = P33; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P34; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P35; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = P36; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P37; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P38; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = P39; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P40; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = P41; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = P42; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = P43; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = rdx.wrapping_add(0xe0); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = P44; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = P45; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = P46; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = P47; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = P48; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = P49; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = P50; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = P51; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = P52; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = P53; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = P54; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = P55; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = P56; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = P57; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = P58; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = P59; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = P60; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = P61; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = P62; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = P63; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = P64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = P65; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = P66; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = P67; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = P68; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = P69; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = P70; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = P71; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = P72; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = P73; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = P74; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = P75; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = P76; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = P77; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = P78; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = P79; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1e0u64))) as u64; return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x1e0u64))) as u64; return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c2410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf2f0; const P1: u64 = 0x143cf6fc8; const P2: u64 = 0x1418349d0; const P3: u64 = 0x143cf6e24; const P4: u64 = 0x143cf7010; const P5: u64 = 0x143cf709c; const P6: u64 = 0x143cf6e18; const P7: u64 = 0x143cf6e68; const P8: u64 = 0x143cf6e90; const P9: u64 = 0x143cf7070; const P10: u64 = 0x143cf6f98; const P11: u64 = 0x143cf6f3c; const P12: u64 = 0x143cf6e20; const P13: u64 = 0x1401de3c0; const P14: u64 = 0x142498af0; const P15: u64 = 0x143cf6e1c; const P16: u64 = 0x143cf6ee0; const P17: u64 = 0x143cf6fc4; const P18: u64 = 0x143cf6f70; const P19: u64 = 0x143cf7068; const P20: u64 = 0x143cf6f74; const P21: u64 = 0x143cf6f38; const P22: u64 = 0x143cf7098; const P23: u64 = 0x143cf70a0; const P24: u64 = 0x143cf706c; const P25: u64 = 0x143cf6fc0; const P26: u64 = 0x1401de400; const P27: u64 = 0x1401e1560;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x390); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P1; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P8; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P9; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P10; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P11; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P12; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x1e0); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x1e0); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x90); fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P15; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P16; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P17; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P18; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P19; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P20; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P21; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P22; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P23; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P24; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P25; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P26, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P26, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c26d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x1401de400; const P2: u64 = 0x1411db190; const P3: u64 = 0x40; const P4: u64 = 0x1401e2110; const P5: u64 = 0x80; const P6: u64 = 0x1401e68b0; const P7: u64 = 0xb8; const P8: u64 = 0x98; const P9: u64 = 0x1423306e0; const P10: u64 = 0x178; const P11: u64 = 0x14130a340; const P12: u64 = 0x14130a300;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P7)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c2770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x140246cd0; const P2: u64 = 0x143cf73b8; const P3: u64 = 0x1418349d0; const P4: u64 = 0x143cf7424; const P5: u64 = 0x143cf73b0; const P6: u64 = 0x143cf7420; const P7: u64 = 0x143cf74ac; const P8: u64 = 0x14052f2f0; const P9: u64 = 0x1401de400; const P10: u64 = 0x14130b530; const P11: u64 = 0x14130b540; const P12: u64 = 0x1401e68b0; const P13: u64 = 0x1422e0200; const P14: u64 = 0x1412cf2f0; const P15: u64 = 0x143cf74cc; const P16: u64 = 0x143cf746c; const P17: u64 = 0x143cf742c; const P18: u64 = 0x143cf73b4; const P19: u64 = 0x143cf7428; const P20: u64 = 0x143cf7464; const P21: u64 = 0x143cf74a8; const P22: u64 = 0x143cf7348; const P23: u64 = 0x143cf7460; const P24: u64 = 0x143cf74c8; const P25: u64 = 0x143cf7470; const P26: u64 = 0x143cf7468; const P27: u64 = 0x1411d5bf0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P2; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_l = ((rcx).wrapping_add(0xb0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x400); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P15; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P16; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P17; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P18; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P19; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x408); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = P20; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = P21; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = P22; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = P23; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x250); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = P24; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P25; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = P26; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0x108); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P27, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c29f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4e0; const P1: u64 = 0x1403b30c0; const P2: u64 = 0x30; const P3: u64 = 0x1401e68b0; const P4: u64 = 0x143cf7644; const P5: u64 = 0x1418349d0; const P6: u64 = 0x143cf7624; const P7: u64 = 0x143cf7620;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P4; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P6; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P7; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c2a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cf79a8; const P1: u64 = 0x1418349d0; const P2: u64 = 0x143cf7ee0; const P3: u64 = 0x143cf7ff4; const P4: u64 = 0x143cf7870; const P5: u64 = 0x143cf7a98; const P6: u64 = 0x1412cf2f0; const P7: u64 = 0x143cf7ee8; const P8: u64 = 0x143cf7810; const P9: u64 = 0x143cf7aa0; const P10: u64 = 0x143cf7700; const P11: u64 = 0x143cf7798; const P12: u64 = 0x143cf7e9c; const P13: u64 = 0x143cf7b80; const P14: u64 = 0x143cf7854; const P15: u64 = 0x143cf80ec; const P16: u64 = 0x143cf7b84; const P17: u64 = 0x143cf7fc8; const P18: u64 = 0x143cf7be4; const P19: u64 = 0x143cf79ac; const P20: u64 = 0x143cf7da8; const P21: u64 = 0x143cf7948; const P22: u64 = 0x143cf7748; const P23: u64 = 0x143cf80f0; const P24: u64 = 0x143cf7c90; const P25: u64 = 0x143cf7b08; const P26: u64 = 0x143cf8018; const P27: u64 = 0x143cf7940; const P28: u64 = 0x143cf76a0; const P29: u64 = 0x14052f2f0; const P30: u64 = 0x143cf7818; const P31: u64 = 0x143cf7e58; const P32: u64 = 0x143cf7b14; const P33: u64 = 0x143cf7850; const P34: u64 = 0x143cf7ff0; const P35: u64 = 0x143cf7720; const P36: u64 = 0x143cf78b8; const P37: u64 = 0x143cf7f14; const P38: u64 = 0x143cf7898; const P39: u64 = 0x143cf7ee4; const P40: u64 = 0x143cf7fa8; const P41: u64 = 0x143cf76a8; const P42: u64 = 0x143cf80f4; const P43: u64 = 0x143cf8060; const P44: u64 = 0x143cf767c; const P45: u64 = 0x143cf7dd4; const P46: u64 = 0x143cf803c; const P47: u64 = 0x143cf7eb8; const P48: u64 = 0x143cf7a9c; const P49: u64 = 0x143cf7b38; const P50: u64 = 0x143cf80e8; const P51: u64 = 0x143cf7dcc; const P52: u64 = 0x143cf77c0; const P53: u64 = 0x143cf7aec; const P54: u64 = 0x143cf76c8; const P55: u64 = 0x143cf7b58; const P56: u64 = 0x143cf7eec; const P57: u64 = 0x143cf7cd4; const P58: u64 = 0x143cf7dc8; const P59: u64 = 0x143cf7d80; const P60: u64 = 0x143cf7e78; const P61: u64 = 0x143cf7944; const P62: u64 = 0x143cf7f48; const P63: u64 = 0x143cf7be0; const P64: u64 = 0x143cf7e7c; const P65: u64 = 0x143cf7ae8; const P66: u64 = 0x143cf76a4; const P67: u64 = 0x143cf7f10; const P68: u64 = 0x143cf7ac0; const P69: u64 = 0x143cf7680; const P70: u64 = 0x143cf7678; const P71: u64 = 0x143cf78bc; const P72: u64 = 0x143cf7c00; const P73: u64 = 0x143cf8038; const P74: u64 = 0x143cf79b0; const P75: u64 = 0x143cf7988; const P76: u64 = 0x143cf77e8; const P77: u64 = 0x143cf7cd0; const P78: u64 = 0x143cf7ef0; const P79: u64 = 0x143cf7814; const P80: u64 = 0x143cf7e98; const P81: u64 = 0x143cf7b0c; const P82: u64 = 0x143cf7d30; const P83: u64 = 0x143cf7b3c; const P84: u64 = 0x143cf7dd0; const P85: u64 = 0x143cf7b10; const P86: u64 = 0x1401dff10; const P87: u64 = 0x1411db860; const P88: u64 = 0x1401e1560; const P89: u64 = 0x1401edac0; const P90: u64 = 0x1401ee260;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = P0; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P7; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P8; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P9; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P10; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P11; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P12; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P13; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P14; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = P15; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P16; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P17; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P18; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P19; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P20; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P21; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P22; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P23; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P24; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P25; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P26; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P27; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = P28; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = P30; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = P31; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = P32; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = P33; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P34; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = P35; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = P36; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P37; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P38; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = P39; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P40; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P41; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = P42; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P43; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = P44; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = P45; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = P46; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = P47; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = P48; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = P49; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = P50; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = P51; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = P52; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = P53; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = P54; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = P55; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = P56; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = P57; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = P58; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = P59; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = P60; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = P61; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = P62; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = P63; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = P64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = P65; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = P66; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = P67; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = P68; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = P69; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = P70; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = P71; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = P72; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = P73; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = P74; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = P75; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = P76; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = P77; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = P78; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = P79; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = P80; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = P81; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = P82; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = P83; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = P84; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = P85; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P86, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P87, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; return crate::simd_ops::call_fwd(P88, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; return crate::simd_ops::call_fwd(P88, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P89, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P90, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c3380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x178; const P1: u64 = 0x140280170; const P2: u64 = 0x168; const P3: u64 = 0x30; const P4: u64 = 0x14230ce60; const P5: u64 = 0x142428ce0; const P6: u64 = 0x1413134f0; const P7: u64 = 0x1413134d0; const P8: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = ((rcx).wrapping_add(0xc0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; fl_l = ((rcx).wrapping_add(0xc0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c4250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e46d0; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x1401e2110; const P3: u64 = 0x140571d50; const P4: u64 = 0x1401df5a0; const P5: u64 = 0x1401de400; const P6: u64 = 0x141312a40; const P7: u64 = 0x140c34930; const P8: u64 = 0x1401e7140; const P9: u64 = 0x140280170; const P10: u64 = 0x1411a4f70; const P11: u64 = 0x14133c0f0; const P12: u64 = 0x14133c120; const P13: u64 = 0x14133c130; const P14: u64 = 0x14133c5c0; const P15: u64 = 0x14133c140; const P16: u64 = 0x1408a3bf0; const P17: u64 = 0x14133c150; const P18: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x108); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x220); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x118u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x128u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x130u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x150u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x160u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x170u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x230u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; fl_l = ((rcx).wrapping_add(0x2a0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P18, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0x78); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x80); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x70); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c4460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x143cf9d50; const P2: u64 = 0x1418349d0; const P3: u64 = 0x143cf9d48; const P4: u64 = 0x143cf9c84; const P5: u64 = 0x143cf9cfc; const P6: u64 = 0x143cf9cbc; const P7: u64 = 0x143cf9d88; const P8: u64 = 0x143cf9d84; const P9: u64 = 0x143cf9de8; const P10: u64 = 0x143cf9e28; const P11: u64 = 0x143cf9cb4; const P12: u64 = 0x143cf9d20; const P13: u64 = 0x143cf9cf0; const P14: u64 = 0x143cf9e2c; const P15: u64 = 0x143cf9cb0; const P16: u64 = 0x143cf9cec; const P17: u64 = 0x143cf9cb8; const P18: u64 = 0x143cf9d24; const P19: u64 = 0x143cf9ce8; const P20: u64 = 0x143cf9e80; const P21: u64 = 0x143cf9d00; const P22: u64 = 0x143cf9cf4; const P23: u64 = 0x143cf9e64; const P24: u64 = 0x143cf9d78; const P25: u64 = 0x143cf9e20; const P26: u64 = 0x143cf9e08; const P27: u64 = 0x143cf9ca8; const P28: u64 = 0x1412cf2f0; const P29: u64 = 0x1411dd9e0; const P30: u64 = 0x1401e68b0; const P31: u64 = 0x1401de400; const P32: u64 = 0x1411db190; const P33: u64 = 0x1412cf610;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P1; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P8; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P9; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P10; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P11; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P12; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P13; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P14; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P15; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x300); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P16; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P17; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P18; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P19; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P20; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P21; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P22; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P23; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P24; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P25; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P26; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P27; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0x270); return crate::simd_ops::call_fwd(P28, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x90); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P30, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x190); return crate::simd_ops::call_fwd(P29, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P31, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P31, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P31, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P31, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c46c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x1401e18a0; const P2: u64 = 0x2a0; const P3: u64 = 0x141341110;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c4833(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433fda50; const P1: u64 = 0x142409b50; const P2: u64 = 0x142119980;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = rdx;
    rax = P0;
    crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax);
    crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (0x18u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x40u64))) as u128;
    crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm0) as u128);
    r9 = 0x18;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rbp.wrapping_add(0x40);
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rdx = (((rdx as u32)).wrapping_add(0x2u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c4964(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf770; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    r9 = P0;
    r8 = 0x6;
    rdx = 0x8;
    rcx = rbp.wrapping_add(0x98);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c4a61(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf770; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    r9 = P0;
    r8 = 0x6;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c4e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cfad34; const P1: u64 = 0x1418349d0; const P2: u64 = 0x143cfa960; const P3: u64 = 0x143cfa1a8; const P4: u64 = 0x143cfab18; const P5: u64 = 0x143cfad7c; const P6: u64 = 0x143cfa290; const P7: u64 = 0x143cfa1c8; const P8: u64 = 0x143cfa37c; const P9: u64 = 0x143cfb090; const P10: u64 = 0x143cfb150; const P11: u64 = 0x143cfa2bc; const P12: u64 = 0x143cfa628; const P13: u64 = 0x143cfae18; const P14: u64 = 0x143cface8; const P15: u64 = 0x143cfae64; const P16: u64 = 0x143cfa0fc; const P17: u64 = 0x143cfab6c; const P18: u64 = 0x143cfab68; const P19: u64 = 0x143cfab10; const P20: u64 = 0x143cfae40; const P21: u64 = 0x143cfaddc; const P22: u64 = 0x143cfb030; const P23: u64 = 0x1412cf2f0; const P24: u64 = 0x143cfb08c; const P25: u64 = 0x143cfac88; const P26: u64 = 0x143cfab14; const P27: u64 = 0x143cfa7f0; const P28: u64 = 0x143cfaaf0; const P29: u64 = 0x143cfa910; const P30: u64 = 0x143cfab64; const P31: u64 = 0x143cfab20; const P32: u64 = 0x14052f2f0; const P33: u64 = 0x1411d5bf0; const P34: u64 = 0x143cfae60; const P35: u64 = 0x143cfa244; const P36: u64 = 0x143cfadd8; const P37: u64 = 0x143cfa378; const P38: u64 = 0x143cfa540; const P39: u64 = 0x142201e30; const P40: u64 = 0x143cfb134; const P41: u64 = 0x143cfae14; const P42: u64 = 0x143cfac20; const P43: u64 = 0x143cfab1c; const P44: u64 = 0x143cfab70; const P45: u64 = 0x143cfad0c; const P46: u64 = 0x143cfa0e8; const P47: u64 = 0x143cfb088; const P48: u64 = 0x143cfb128; const P49: u64 = 0x143cfaab4; const P50: u64 = 0x143cfab24; const P51: u64 = 0x143cfacdc; const P52: u64 = 0x143cfad30; const P53: u64 = 0x143cfadb8; const P54: u64 = 0x143cface0; const P55: u64 = 0x143cfabfc; const P56: u64 = 0x143cfa2b8; const P57: u64 = 0x143cfa830; const P58: u64 = 0x143cfb0b0; const P59: u64 = 0x143cfb108; const P60: u64 = 0x143cfab60; const P61: u64 = 0x143cfab28; const P62: u64 = 0x143cface4; const P63: u64 = 0x143cfad58; const P64: u64 = 0x143cfad80; const P65: u64 = 0x143cfa0ec; const P66: u64 = 0x143cfa0f4; const P67: u64 = 0x143cfaa54; const P68: u64 = 0x143cfa240; const P69: u64 = 0x143cfa8a4; const P70: u64 = 0x143cfa268; const P71: u64 = 0x143cfa2c4; const P72: u64 = 0x143cfb010; const P73: u64 = 0x1401e5030; const P74: u64 = 0x143cfb12c; const P75: u64 = 0x143cfa620; const P76: u64 = 0x143cfae3c; const P77: u64 = 0x143cfb008; const P78: u64 = 0x143cfac00; const P79: u64 = 0x143cfae38; const P80: u64 = 0x143cfa66c; const P81: u64 = 0x143cfa100; const P82: u64 = 0x143cfaab0; const P83: u64 = 0x143cfa2b4; const P84: u64 = 0x143cfb0d4; const P85: u64 = 0x143cfad78; const P86: u64 = 0x143cfb068; const P87: u64 = 0x143cfad50; const P88: u64 = 0x143cfa294; const P89: u64 = 0x143cfa668; const P90: u64 = 0x143cfb154; const P91: u64 = 0x143cfa634; const P92: u64 = 0x143cfa630; const P93: u64 = 0x143cfa62c; const P94: u64 = 0x143cfacd8; const P95: u64 = 0x143cfa9b0; const P96: u64 = 0x143cfafc8; const P97: u64 = 0x143cfad54; const P98: u64 = 0x143cfaa50; const P99: u64 = 0x1401e1560; const P100: u64 = 0x14235ab80; const P101: u64 = 0x142362190; const P102: u64 = 0x1401de400;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = P0; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P8; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P9; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P10; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P11; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P12; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P13; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P14; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P15; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = P16; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P17; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P18; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P19; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P20; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P21; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P22; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x44); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P24; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P25; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P26; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P27; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P28; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = P29; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = P30; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = P31; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = rdx.wrapping_add(0x330); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = rdx.wrapping_add(0x380); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = P34; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = P35; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = rdx.wrapping_add(0x280); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = P36; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P37; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P38; return crate::simd_ops::call_fwd(P39, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = P40; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P41; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P42; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P43; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = P44; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = P45; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = P46; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = P47; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = P48; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = P49; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = P50; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = P51; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = P52; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = P53; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = P54; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = P55; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = P56; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = P57; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = P58; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = P59; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = rdx.wrapping_add(0x5d0); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = P60; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = P61; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = P62; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = P63; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = P64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = P65; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = P66; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = P67; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0x270); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = rdx.wrapping_add(0x2c0); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = P68; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = P69; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = P70; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = P71; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = P72; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = rdx.wrapping_add(0x3d0); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x880); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P73, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x878); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = rdx.wrapping_add(0x700); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = P74; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = P75; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = P76; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = P77; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = rdx.wrapping_add(0x888); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = P78; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = P79; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = P80; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = P81; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = P82; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = P83; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = P84; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0x750); return crate::simd_ops::call_fwd(P32, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = P85; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = P86; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = P87; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = P88; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = P89; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = P90; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = P91; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = P92; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = P93; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = P94; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = rdx.wrapping_add(0x120); return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = rdx.wrapping_add(0x1f0); return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = rdx.wrapping_add(0x110); return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P33, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = rdx.wrapping_add(0x870); return crate::simd_ops::call_fwd(P23, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = P95; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = P96; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = P97; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = P98; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x958u64))) as u64; return crate::simd_ops::call_fwd(P99, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x958u64))) as u64; return crate::simd_ops::call_fwd(P100, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            119 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x950u64))) as u64; return crate::simd_ops::call_fwd(P99, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            120 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x950u64))) as u64; return crate::simd_ops::call_fwd(P99, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            121 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P101, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            122 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P101, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            123 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P101, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            124 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x560u64))) as u64; fl_l = ((rcx).wrapping_add(0x180000u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P102, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            125 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x560u64))) as u64; fl_l = ((rcx).wrapping_add(0x180010u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P102, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            126 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x560u64))) as u64; fl_l = ((rcx).wrapping_add(0x180020u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P102, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            127 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x560u64))) as u64; fl_l = ((rcx).wrapping_add(0x180030u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P102, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            128 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x560u64))) as u64; fl_l = ((rcx).wrapping_add(0x180040u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P102, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            129 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x560u64))) as u64; fl_l = ((rcx).wrapping_add(0x180050u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P102, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            130 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x560u64))) as u64; fl_l = ((rcx).wrapping_add(0x180060u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P102, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c5465(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x560u64))) as u64;
    rcx = ((rcx).wrapping_add(0x180070u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c5499(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x560u64))) as u64;
    rcx = ((rcx).wrapping_add(0x180080u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c5598(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140280170; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    r9 = P0;
    r8 = 0x100;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c58d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0x1412cf610; const P2: u64 = 0x143cfb1f4; const P3: u64 = 0x1418349d0; const P4: u64 = 0x143cfb1e8; const P5: u64 = 0x143cfb230; const P6: u64 = 0x143cfb1ec; const P7: u64 = 0x143cfb1f0; const P8: u64 = 0x143cfb158; const P9: u64 = 0x50; const P10: u64 = 0x1412cf2f0; const P11: u64 = 0x143cfb270; const P12: u64 = 0x143cfb2b0; const P13: u64 = 0x143cfb160; const P14: u64 = 0x143cfb234; const P15: u64 = 0x143cfb308; const P16: u64 = 0x143cfb15c; const P17: u64 = 0x1411d5bf0; const P18: u64 = 0x143cfb1c0; const P19: u64 = 0x143cfb274;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P4; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P5; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P6; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P7; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P8; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P11; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P12; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P13; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x51); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P14; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P15; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P16; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x538); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P18; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P19; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = rdx.wrapping_add(0x140); return crate::simd_ops::call_fwd(P17, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x528); return crate::simd_ops::call_fwd(P10, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c5df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cfb798; const P1: u64 = 0x1418349d0; const P2: u64 = 0x143cfbb54; const P3: u64 = 0x143cfb754; const P4: u64 = 0x143cfb824; const P5: u64 = 0x143cfb710; const P6: u64 = 0x143cfb7e8; const P7: u64 = 0x143cfbb28; const P8: u64 = 0x14052f2f0; const P9: u64 = 0x1411db190; const P10: u64 = 0x143cfbb98; const P11: u64 = 0x143cfb860; const P12: u64 = 0x143cfbb5c; const P13: u64 = 0x143cfba34; const P14: u64 = 0x143cfb5fc; const P15: u64 = 0x143cfbb08; const P16: u64 = 0x143cfbb4c; const P17: u64 = 0x143cfba78; const P18: u64 = 0x143cfb750; const P19: u64 = 0x143cfb920; const P20: u64 = 0x143cfb864; const P21: u64 = 0x143cfb9a8; const P22: u64 = 0x143cfb8b0; const P23: u64 = 0x143cfb828; const P24: u64 = 0x143cfbbbc; const P25: u64 = 0x143cfb928; const P26: u64 = 0x143cfbb48; const P27: u64 = 0x143cfb74c; const P28: u64 = 0x143cfb604; const P29: u64 = 0x143cfba50; const P30: u64 = 0x143cfbb50; const P31: u64 = 0x143cfb778; const P32: u64 = 0x143cfb774; const P33: u64 = 0x143cfb9b0; const P34: u64 = 0x143cfbb04; const P35: u64 = 0x143cfb820; const P36: u64 = 0x143cfba54; const P37: u64 = 0x1412cf2f0; const P38: u64 = 0x143cfbb60; const P39: u64 = 0x143cfb600; const P40: u64 = 0x143cfb924; const P41: u64 = 0x143cfbb58; const P42: u64 = 0x143cfb9b4; const P43: u64 = 0x143cfb748; const P44: u64 = 0x143cfba30; const P45: u64 = 0x143cfba58; const P46: u64 = 0x143cfbbb8; const P47: u64 = 0x143cfbb00; const P48: u64 = 0x143cfb714; const P49: u64 = 0x143cfb9ac; const P50: u64 = 0x143cfb8d0; const P51: u64 = 0x143cfbac8; const P52: u64 = 0x143cfba7c; const P53: u64 = 0x143cfb8b4; const P54: u64 = 0x143cfbb9c; const P55: u64 = 0x143cfb770; const P56: u64 = 0x1411d5bf0; const P57: u64 = 0x140280170; const P58: u64 = 0x1413525a0; const P59: u64 = 0x1412eb3b0; const P60: u64 = 0x1413525f0; const P61: u64 = 0x141352620; const P62: u64 = 0x141352680; const P63: u64 = 0x1413526e0; const P64: u64 = 0x1401de400; const P65: u64 = 0x141352740; const P66: u64 = 0x1410f6990; const P67: u64 = 0x141352750; const P68: u64 = 0x141337dd0; const P69: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = P0; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P10; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P11; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P12; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P13; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P14; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P15; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = P16; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P17; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P18; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P19; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P20; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P21; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P22; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P23; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P24; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P25; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P26; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P27; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = P28; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = P29; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = P30; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = P31; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = P32; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = P33; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = P34; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = P35; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P36; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = rdx.wrapping_add(0xc0); return crate::simd_ops::call_fwd(P37, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P38; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P39; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = P40; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P41; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P42; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = P43; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P44; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = P45; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = P46; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = P47; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = P48; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = P49; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = P50; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = P51; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = P52; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = P53; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = P54; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = P55; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P56, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P37, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P57, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P58, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0xf0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P59, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x1c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P60, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x268u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P61, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x350u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P62, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x438u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P63, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x528u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x538u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P65, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x548u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x560u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x578u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P66, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x5a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x5b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x5c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x5d8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x5e8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x5f8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P66, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x620u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P66, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x648u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P66, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x670u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P67, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x680u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x688u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x690u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x698u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x6a0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x6a8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x6b0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x6b8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x6c0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x6c8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P68, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x150u64))) as u64; fl_l = ((rcx).wrapping_add(0x6d0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P57, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P69, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P69, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P69, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P69, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P69, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P69, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c6470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3b8; const P1: u64 = 0x1412cf2f0; const P2: u64 = 0x143cfbdb0; const P3: u64 = 0x1412cf070; const P4: u64 = 0x143cfbdd0; const P5: u64 = 0x143cfbd40; const P6: u64 = 0x143cfbd60; const P7: u64 = 0x143cfbdac; const P8: u64 = 0x1418349d0; const P9: u64 = 0x143cfbda4; const P10: u64 = 0x40; const P11: u64 = 0x1401e68b0; const P12: u64 = 0x143cfbda8; const P13: u64 = 0x1411d5bf0; const P14: u64 = 0x1401e1560;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P4; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P5; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P6; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P7; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P9; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x60); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P12; return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x148); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x20); return crate::simd_ops::call_fwd(P11, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x100); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x90u64))) as u64; return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c6a34(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x170u64))) as u64;
    r9 = P0;
    r8 = 0x6;
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c6c36(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de400; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    r9 = P0;
    r8 = 0x4;
    rdx = 0x30;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c6c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cfc768; const P1: u64 = 0x1418349d0; const P2: u64 = 0x143cfcbc0; const P3: u64 = 0x143cfcf50; const P4: u64 = 0x143cfc798; const P5: u64 = 0x143cfce54; const P6: u64 = 0x143cfd128; const P7: u64 = 0x143cfc8f8; const P8: u64 = 0x1412cf610; const P9: u64 = 0x143cfce00; const P10: u64 = 0x143cfc900; const P11: u64 = 0x143cfc678; const P12: u64 = 0x143cfced8; const P13: u64 = 0x143cfcfc8; const P14: u64 = 0x143cfce50; const P15: u64 = 0x1412cf2f0; const P16: u64 = 0x143cfc62c; const P17: u64 = 0x143cfccb0; const P18: u64 = 0x143cfc764; const P19: u64 = 0x143cfcbbc; const P20: u64 = 0x143cfcacc; const P21: u64 = 0x143cfce70; const P22: u64 = 0x143cfcf98; const P23: u64 = 0x143cfc888; const P24: u64 = 0x143cfcc08; const P25: u64 = 0x14052f2f0; const P26: u64 = 0x143cfd0b4; const P27: u64 = 0x143cfc96c; const P28: u64 = 0x143cfcf4c; const P29: u64 = 0x143cfc628; const P30: u64 = 0x143cfd018; const P31: u64 = 0x143cfc8ec; const P32: u64 = 0x143cfcc30; const P33: u64 = 0x143cfcf48; const P34: u64 = 0x143cfc7c4; const P35: u64 = 0x143cfc668; const P36: u64 = 0x143cfd040; const P37: u64 = 0x143cfcaf4; const P38: u64 = 0x143cfcd18; const P39: u64 = 0x143cfc634; const P40: u64 = 0x143cfcba0; const P41: u64 = 0x143cfcbb8; const P42: u64 = 0x143cfca5c; const P43: u64 = 0x143cfc7f8; const P44: u64 = 0x143cfc968; const P45: u64 = 0x143cfc638; const P46: u64 = 0x143cfc790; const P47: u64 = 0x143cfc760; const P48: u64 = 0x143cfd254; const P49: u64 = 0x143cfd298; const P50: u64 = 0x143cfce28; const P51: u64 = 0x143cfc7f4; const P52: u64 = 0x143cfc6a0; const P53: u64 = 0x143cfc7c8; const P54: u64 = 0x143cfcd80; const P55: u64 = 0x143cfd29c; const P56: u64 = 0x143cfc820; const P57: u64 = 0x143cfcb98; const P58: u64 = 0x143cfc7f0; const P59: u64 = 0x143cfcf00; const P60: u64 = 0x143cfc8e8; const P61: u64 = 0x143cfd08c; const P62: u64 = 0x143cfcbac; const P63: u64 = 0x143cfcfc0; const P64: u64 = 0x143cfcbb4; const P65: u64 = 0x143cfcba8; const P66: u64 = 0x143cfd0f8; const P67: u64 = 0x143cfca80; const P68: u64 = 0x143cfcaf0; const P69: u64 = 0x143cfc5c4; const P70: u64 = 0x143cfcb9c; const P71: u64 = 0x143cfd278; const P72: u64 = 0x143cfc5c0; const P73: u64 = 0x143cfc664; const P74: u64 = 0x143cfc8fc; const P75: u64 = 0x143cfcfc4; const P76: u64 = 0x143cfc824; const P77: u64 = 0x143cfcac8; const P78: u64 = 0x143cfd0b0; const P79: u64 = 0x143cfc794; const P80: u64 = 0x143cfd088; const P81: u64 = 0x143cfcba4; const P82: u64 = 0x143cfd1c8; const P83: u64 = 0x143cfd12c; const P84: u64 = 0x143cfcf54; const P85: u64 = 0x143cfc7c0; const P86: u64 = 0x143cfcc04; const P87: u64 = 0x143cfc76c; const P88: u64 = 0x143cfcff0; const P89: u64 = 0x143cfc660; const P90: u64 = 0x143cfc8b4; const P91: u64 = 0x143cfcff4; const P92: u64 = 0x143cfcc00; const P93: u64 = 0x143cfca18; const P94: u64 = 0x143cfc630; const P95: u64 = 0x143cfd044; const P96: u64 = 0x143cfc66c; const P97: u64 = 0x143cfcbb0; const P98: u64 = 0x143cfca58; const P99: u64 = 0x143cfca84; const P100: u64 = 0x143cfc8b0; const P101: u64 = 0x143cfd250; const P102: u64 = 0x143cfc970;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = P0; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = rdx.wrapping_add(0x130); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = rdx.wrapping_add(0x128); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P9; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P10; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P11; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P12; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P13; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P14; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0x310); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P16; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P17; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P18; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P19; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0x310); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P20; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P21; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P22; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P23; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P24; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = rdx.wrapping_add(0x158); return crate::simd_ops::call_fwd(P25, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P26; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = rdx.wrapping_add(0x310); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = P27; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = P28; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = P29; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = P30; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = P31; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x310); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P32; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = P33; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = P34; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P35; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P36; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x1e8); return crate::simd_ops::call_fwd(P25, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P37; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P38; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = P39; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P40; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = P41; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = P42; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = P43; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = P44; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = P45; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = P46; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = P47; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = P48; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = P49; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = P50; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = P51; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = P52; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = P53; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = P54; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = P55; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = P56; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = P57; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = P58; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = P59; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = P60; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = P61; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = P62; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = P63; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = P64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = P65; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = P66; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = P67; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = P68; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = P69; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = P70; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = P71; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            82 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            83 => { rcx = P72; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            84 => { rcx = P73; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            85 => { rcx = P74; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            86 => { rcx = P75; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            87 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            88 => { rcx = P76; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            89 => { rcx = P77; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            90 => { rcx = P78; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            91 => { rcx = P79; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            92 => { rcx = P80; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            93 => { rcx = P81; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            94 => { rcx = P82; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            95 => { rcx = P83; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            96 => { rcx = rdx.wrapping_add(0xd0); return crate::simd_ops::call_fwd(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            97 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            98 => { rcx = P84; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            99 => { rcx = P85; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            100 => { rcx = P86; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            101 => { rcx = P87; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            102 => { rcx = P88; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            103 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            104 => { rcx = P89; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            105 => { rcx = P90; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            106 => { rcx = P91; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            107 => { rcx = P92; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            108 => { rcx = P93; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            109 => { rcx = P94; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            110 => { rcx = P95; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            111 => { rcx = P96; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            112 => { rcx = P97; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            113 => { rcx = rdx.wrapping_add(0x308); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            114 => { rcx = P98; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            115 => { rcx = P99; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            116 => { rcx = P100; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            117 => { rcx = P101; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            118 => { rcx = P102; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c7210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x290; const P1: u64 = 0x1403b30c0; const P2: u64 = 0x140; const P3: u64 = 0x30; const P4: u64 = 0x1401e68b0; const P5: u64 = 0x143cfdcd0; const P6: u64 = 0x1418349d0; const P7: u64 = 0x143cfd310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P3)) as u64) as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P5; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P7; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c724c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141364890; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    r9 = P0;
    r8 = 0x6;
    rdx = 0x4f0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c7560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf2f0; const P1: u64 = 0x1401de400; const P2: u64 = 0x143cfecb8; const P3: u64 = 0x1418349d0; const P4: u64 = 0x143cfee60; const P5: u64 = 0x142201e30; const P6: u64 = 0x143cfeb98; const P7: u64 = 0x143cfe830; const P8: u64 = 0x143cfeddc; const P9: u64 = 0x143cfef34; const P10: u64 = 0x143cfeee8; const P11: u64 = 0x143cfede0; const P12: u64 = 0x143cfeee0; const P13: u64 = 0x143cfef40; const P14: u64 = 0x143cfe82c; const P15: u64 = 0x143cfe974; const P16: u64 = 0x143cfea68; const P17: u64 = 0x143cfef3c; const P18: u64 = 0x143cfef38; const P19: u64 = 0x143cfea28; const P20: u64 = 0x143cfec90; const P21: u64 = 0x143cfeb78; const P22: u64 = 0x143cfec48; const P23: u64 = 0x143cfed64; const P24: u64 = 0x143cfece8; const P25: u64 = 0x143cfe954; const P26: u64 = 0x143cfedd0; const P27: u64 = 0x143cfea30; const P28: u64 = 0x143cfeec0; const P29: u64 = 0x143cfef30; const P30: u64 = 0x143cfed88; const P31: u64 = 0x143cfe9a8; const P32: u64 = 0x143cfe970; const P33: u64 = 0x143cfeac8; const P34: u64 = 0x143cfea34; const P35: u64 = 0x143cfe9e0; const P36: u64 = 0x143cfeee4; const P37: u64 = 0x143cfecc4; const P38: u64 = 0x143cfed60; const P39: u64 = 0x143cfe950; const P40: u64 = 0x143cfecc0; const P41: u64 = 0x143cfedd8; const P42: u64 = 0x143cfeaf4; const P43: u64 = 0x143cfeaa8; const P44: u64 = 0x143cfedb4; const P45: u64 = 0x143cfe9ac; const P46: u64 = 0x143cfec68; const P47: u64 = 0x1401e18d0; const P48: u64 = 0x1411db190; const P49: u64 = 0x1411df630; const P50: u64 = 0x1412cf610; const P51: u64 = 0x143cfe890; const P52: u64 = 0x143cfe8f0; const P53: u64 = 0x143cfeb20; const P54: u64 = 0x143cfe8b8; const P55: u64 = 0x143cfec10; const P56: u64 = 0x143cfeaf0; const P57: u64 = 0x143cfea2c; const P58: u64 = 0x143cfee08; const P59: u64 = 0x14052f2f0; const P60: u64 = 0x143cfed68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x4d8); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = rdx.wrapping_add(0x440); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = rdx.wrapping_add(0x108); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = rdx.wrapping_add(0x330); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P2; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P4; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P6; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P7; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P8; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P9; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P10; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P11; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P12; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = P13; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P14; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P15; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = P16; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P17; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = P18; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = P19; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = P20; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = P21; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P22; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P23; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P24; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = P25; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = P26; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = P27; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = P28; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = P29; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = P30; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = P31; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = P32; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = P33; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P34; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = P35; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = P36; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P37; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P38; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = P39; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P40; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = P41; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = P42; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = P43; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = P44; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = P45; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = P46; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = rdx.wrapping_add(0xf0); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            50 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            51 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            53 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P47, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            54 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            55 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            56 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x50u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            57 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P49, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            58 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            59 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            60 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x70u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            61 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x78u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P49, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            62 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x80u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            63 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            64 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0x90u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            65 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            66 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0xb0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            67 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            68 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = ((rcx).wrapping_add(0xc0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P48, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            69 => { rcx = rdx.wrapping_add(0x50); return crate::simd_ops::call_fwd(P50, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            70 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P50, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            71 => { rcx = rdx.wrapping_add(0x48); return crate::simd_ops::call_fwd(P50, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            72 => { rcx = P51; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            73 => { rcx = P52; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            74 => { rcx = P53; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            75 => { rcx = P54; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            76 => { rcx = P55; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            77 => { rcx = P56; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            78 => { rcx = P57; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            79 => { rcx = P58; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            80 => { rcx = rdx.wrapping_add(0x330); return crate::simd_ops::call_fwd(P59, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            81 => { rcx = P60; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c7d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x180; const P1: u64 = 0x1412cf2f0; const P2: u64 = 0x143cff550; const P3: u64 = 0x1418349d0; const P4: u64 = 0x143cff578; const P5: u64 = 0x40; const P6: u64 = 0x1401de400; const P7: u64 = 0x140571d50; const P8: u64 = 0x38; const P9: u64 = 0xf0; const P10: u64 = 0x30; const P11: u64 = 0x20; const P12: u64 = 0x1401e68b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P2; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c82f4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf610; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    r9 = P0;
    r8 = 0x7;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c849c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411db190; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    r9 = P0;
    r8 = 0x2;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c8620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141364890; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64;
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    r9 = P0;
    r8 = 0x6;
    rdx = 0x4f0;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c86e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412cf2f0; const P1: u64 = 0x143d006e0; const P2: u64 = 0x1418349d0; const P3: u64 = 0x143d004fc; const P4: u64 = 0x143d00fac; const P5: u64 = 0x143d00fa8; const P6: u64 = 0x143d004ac; const P7: u64 = 0x143d0069c; const P8: u64 = 0x143d004a8; const P9: u64 = 0x143d0055c; const P10: u64 = 0x143d003f0; const P11: u64 = 0x143d003f4; const P12: u64 = 0x143d00518; const P13: u64 = 0x143d00f18; const P14: u64 = 0x143d00638; const P15: u64 = 0x143d00f10; const P16: u64 = 0x143d00fa0; const P17: u64 = 0x143d00474; const P18: u64 = 0x143d006e4; const P19: u64 = 0x143d003f8; const P20: u64 = 0x1401e68b0; const P21: u64 = 0x14136e8b0; const P22: u64 = 0x143d00fb0; const P23: u64 = 0x143d004f8; const P24: u64 = 0x143d00564; const P25: u64 = 0x143d00fa4; const P26: u64 = 0x143d006a4; const P27: u64 = 0x143d00560; const P28: u64 = 0x143d00ec0; const P29: u64 = 0x143d00698; const P30: u64 = 0x1411d5bf0; const P31: u64 = 0x143d006a0; const P32: u64 = 0x143d005f0; const P33: u64 = 0x143d00558; const P34: u64 = 0x143d00418; const P35: u64 = 0x143d00470; const P36: u64 = 0x143d00f14; const P37: u64 = 0x1411df630; const P38: u64 = 0x14136e8c0; const P39: u64 = 0x1411db190; const P40: u64 = 0x1412cf770; const P41: u64 = 0x1412fa4a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = rdx.wrapping_add(0x200); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = P1; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = P3; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = P4; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = P5; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = P6; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = P7; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = P8; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = P9; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = P10; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = P11; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = P12; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = P13; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = P14; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = P15; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = P16; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = P17; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { rcx = P18; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { rcx = rdx.wrapping_add(0x3c); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { rcx = P19; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { rcx = rdx.wrapping_add(0xf88); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { rcx = rdx.wrapping_add(0x68); return crate::simd_ops::call_fwd(P20, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { rcx = rdx.wrapping_add(0x58); return crate::simd_ops::call_fwd(P21, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { rcx = rdx.wrapping_add(0xf90); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { rcx = P22; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { rcx = P23; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { rcx = P24; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { rcx = rdx.wrapping_add(0xf98); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { rcx = P25; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { rcx = P26; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            30 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            31 => { rcx = P27; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            32 => { rcx = P28; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            33 => { rcx = P29; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            34 => { rcx = rdx.wrapping_add(0x160); return crate::simd_ops::call_fwd(P30, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { rcx = rdx.wrapping_add(0x3d); return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            36 => { rcx = P31; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            37 => { rcx = P32; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            38 => { rcx = P33; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            39 => { rcx = P34; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            40 => { rcx = P35; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            41 => { rcx = rdx.wrapping_add(0x170); return crate::simd_ops::call_fwd(P30, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            42 => { rcx = P36; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            43 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P37, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            44 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P38, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            45 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P39, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            46 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P39, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            47 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P40, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            48 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P38, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            49 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; fl_l = ((rcx).wrapping_add(0x58u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P41, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c8954(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412eaec0; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rcx = ((rcx).wrapping_add(0x60u64)) as u64;
    r9 = P0;
    r8 = 0x6;
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c8d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x850; const P1: u64 = 0x14240ab10; const P2: u64 = 0x60; const P3: u64 = 0x140208fd0; const P4: u64 = 0x390; const P5: u64 = 0x1401de3c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c9c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x198; const P1: u64 = 0x14139eed0; const P2: u64 = 0x50; const P3: u64 = 0x1401de400; const P4: u64 = 0x40; const P5: u64 = 0x60; const P6: u64 = 0x140fc3800; const P7: u64 = 0x142425650; const P8: u64 = 0x20; const P9: u64 = 0x14241e8b0; const P10: u64 = 0xd0; const P11: u64 = 0x30; const P12: u64 = 0x140571d50; const P13: u64 = 0x14139eee0; const P14: u64 = 0x14106ace0; const P15: u64 = 0x1401e68b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { rcx = ((rdx.wrapping_add(P2)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { rcx = ((rdx.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = ((rdx.wrapping_add(P5)) as u64) as u64; return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = ((rdx.wrapping_add(P8)) as u64) as u64; return crate::simd_ops::call_fwd(P9, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = ((rdx.wrapping_add(P10)) as u64) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = ((rdx.wrapping_add(P11)) as u64) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { rcx = rdx.wrapping_add(0x38); return crate::simd_ops::call_fwd(P12, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P13, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { rcx = rdx.wrapping_add(0x30); fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; return crate::simd_ops::call_fwd(P6, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { rcx = rdx.wrapping_add(0x30); return crate::simd_ops::call_fwd(P7, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { rcx = rdx.wrapping_add(0xa0); return crate::simd_ops::call_fwd(P14, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { rcx = rdx.wrapping_add(0x40); return crate::simd_ops::call_fwd(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1425c9e16(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411e3350; const P1: u64 = 0x141834ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = rdx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64;
    rcx = ((rcx).wrapping_add(0xb0u64)) as u64;
    r9 = P0;
    r8 = 0x3;
    rdx = 0x40;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}
