//! Differential unit tests verifying `FastOp` lowering/execution matches `exec_generic_ins`
//! bit-for-bit across string ops, NEON SIMD, FMA, AES, F16C, PCMPSTR, BMI, atomics, and fusion.

use super::*;
use crate::emu::Cpu;

#[inline(always)]
fn pk<T: Copy, const N: usize>(a: [T; N]) -> u128 {
    debug_assert_eq!(core::mem::size_of::<[T; N]>(), 16);
    unsafe { core::mem::transmute_copy(&a) }
}

/// Allocate a guest-valid buffer aligned to `align` (always inside `[0x1_0000, 0x8000_0000_0000)`).
struct AlignedBuf {
    ptr: *mut u8,
    layout: std::alloc::Layout,
}

impl AlignedBuf {
    fn new(size: usize, align: usize) -> Self {
        let layout = std::alloc::Layout::from_size_align(size, align).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        assert!(!ptr.is_null());
        assert!(crate::emu::regfile_ext::is_valid_guest_range(ptr as u64, size));
        Self { ptr, layout }
    }
    fn addr(&self) -> u64 {
        self.ptr as u64
    }
    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.layout.size()) }
    }
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.layout.size()) }
    }
}

impl Drop for AlignedBuf {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr, self.layout) }
    }
}

fn base_cpu() -> Cpu {
    let mut cpu = Cpu::new_zeroed();
    for i in 0..16 {
        cpu.gpr[i] = 0x1111_2222_3333_4400u64.wrapping_add((i as u64) * 0x0102_0304_0506_0701);
        cpu.last_bits[i] = 64;
        cpu.xmm[i] = 0xDEAD_BEEF_CAFE_BABE_0123_4567_89AB_CDF0u128 ^ ((i as u128) * 0x0F1E_2D3C_4B5A_6978);
        cpu.ymm_hi[i] = 0xA5A5_5A5A_1234_5678_9ABC_DEF0_FEDC_BA98u128 ^ ((i as u128) * 0x1111_2222_3333_4444);
    }
    cpu.cf = true;
    cpu.zf = false;
    cpu.sf = true;
    cpu.of = true;
    cpu.set_pf(false);
    cpu.df = false;
    cpu
}

/// Execute `code` once through the `FastOp` path and once through `exec_generic_ins` on cloned
/// CPUs + memory buffers (using `rdi`/`rsi`/`rbx`/`rdx` mapped to `buf_a`/`buf_b` as configured
/// by `setup`), verifying every lowered op is non-`K_GENERIC` and asserting bit-exact equality
/// of all GPRs, `last_bits`, RFLAGS, XMM/YMM registers, and memory contents.
fn diff_seq(
    label: &str,
    code: &[u8],
    buf_size: usize,
    align: usize,
    mut setup: impl FnMut(&mut Cpu, &mut AlignedBuf, &mut AlignedBuf),
) {
    let mut buf_f_a = AlignedBuf::new(buf_size, align);
    let mut buf_f_b = AlignedBuf::new(buf_size, align);
    let mut buf_g_a = AlignedBuf::new(buf_size, align);
    let mut buf_g_b = AlignedBuf::new(buf_size, align);

    let mut cpu_f = base_cpu();
    let mut cpu_g = base_cpu();
    setup(&mut cpu_f, &mut buf_f_a, &mut buf_f_b);
    setup(&mut cpu_g, &mut buf_g_a, &mut buf_g_b);

    // Decode with a stable guest IP in `code`.
    let mut dec = iced_x86::Decoder::with_ip(64, code, 0x1000_0000, iced_x86::DecoderOptions::NONE);
    let mut insns = Vec::new();
    let mut ops = Vec::new();
    while dec.can_decode() {
        let ins = dec.decode();
        assert!(!ins.is_invalid(), "{label}: invalid test encoding");
        let op = lower(&ins);
        assert_ne!(op.kind, K_GENERIC, "{label}: {:?} fell back to K_GENERIC", ins.mnemonic());
        insns.push(ins);
        ops.push(op);
    }
    fuse(&mut ops);

    unsafe {
        for (ins, op) in insns.iter().zip(ops.iter()) {
            cpu_f.rip = op.ip.wrapping_add(op.len as u64);
            let _ = exec(&mut cpu_f, op);
            cpu_g.rip = ins.next_ip();
            let handled = cpu_g.exec_generic_ins(ins);
            assert!(handled, "{label}: generic handler did not handle {:?}", ins.mnemonic());
        }
    }

    // Normalize buffer base pointers in GPRs before comparing (since fast & generic use separate buffers).
    // Check exact [base, base + sz) first so adjacent heap allocations never alias into the +-64 slack window.
    let norm_gpr = |v: u64, ba: u64, bb: u64| -> u64 {
        let sz = buf_size as u64;
        if v >= ba && v < ba.wrapping_add(sz) {
            0xAAAA_0000_0000_0000u64.wrapping_add(v.wrapping_sub(ba))
        } else if v >= bb && v < bb.wrapping_add(sz) {
            0xBBBB_0000_0000_0000u64.wrapping_add(v.wrapping_sub(bb))
        } else if v >= ba.wrapping_sub(64) && v <= ba.wrapping_add(sz + 64) {
            0xAAAA_0000_0000_0000u64.wrapping_add(v.wrapping_sub(ba))
        } else if v >= bb.wrapping_sub(64) && v <= bb.wrapping_add(sz + 64) {
            0xBBBB_0000_0000_0000u64.wrapping_add(v.wrapping_sub(bb))
        } else {
            v
        }
    };
    for i in 0..16 {
        let gf = norm_gpr(cpu_f.gpr[i], buf_f_a.addr(), buf_f_b.addr());
        let gg = norm_gpr(cpu_g.gpr[i], buf_g_a.addr(), buf_g_b.addr());
        assert_eq!(gf, gg, "{label}: gpr[{i}] mismatch: fast={gf:#x} gen={gg:#x}");
        assert_eq!(
            cpu_f.last_bits[i], cpu_g.last_bits[i],
            "{label}: last_bits[{i}] mismatch: fast={} gen={}",
            cpu_f.last_bits[i], cpu_g.last_bits[i]
        );
    }
    assert_eq!(cpu_f.cf, cpu_g.cf, "{label}: CF mismatch");
    assert_eq!(cpu_f.zf, cpu_g.zf, "{label}: ZF mismatch");
    assert_eq!(cpu_f.sf, cpu_g.sf, "{label}: SF mismatch");
    assert_eq!(cpu_f.of, cpu_g.of, "{label}: OF mismatch");
    assert_eq!(cpu_f.pf(), cpu_g.pf(), "{label}: PF mismatch");
    assert_eq!(cpu_f.df, cpu_g.df, "{label}: DF mismatch");
    for i in 0..16 {
        assert_eq!(
            cpu_f.xmm[i], cpu_g.xmm[i],
            "{label}: xmm[{i}] mismatch: fast={:#034x} gen={:#034x}",
            cpu_f.xmm[i], cpu_g.xmm[i]
        );
        assert_eq!(
            cpu_f.ymm_hi[i], cpu_g.ymm_hi[i],
            "{label}: ymm_hi[{i}] mismatch: fast={:#034x} gen={:#034x}",
            cpu_f.ymm_hi[i], cpu_g.ymm_hi[i]
        );
    }
    assert_eq!(buf_f_a.as_slice(), buf_g_a.as_slice(), "{label}: buf_a memory mismatch");
    assert_eq!(buf_f_b.as_slice(), buf_g_b.as_slice(), "{label}: buf_b memory mismatch");
}

#[test]
fn fast_struct_size_is_32() {
    assert_eq!(core::mem::size_of::<FastOp>(), 32);
}

#[test]
fn fast_rep_movs_differential() {
    let forms: &[(&str, &[u8], usize)] = &[
        ("movsb", &[0xA4], 1),
        ("rep_movsb", &[0xF3, 0xA4], 1),
        ("rep_movsw", &[0xF3, 0x66, 0xA5], 2),
        ("rep_movsd", &[0xF3, 0xA5], 4),
        ("rep_movsq", &[0xF3, 0x48, 0xA5], 8),
    ];
    for &(name, code, elem) in forms {
        // Disjoint forward & backward, rcx=0, unaligned offsets
        for &df in &[false, true] {
            for &cnt in &[0usize, 1, 3, 17, 65] {
                for &off in &[0usize, 1, 3, 5] {
                    diff_seq(&format!("{name}_disjoint_df{df}_c{cnt}_o{off}"), code, 1024, 64, |cpu, a, b| {
                        for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                            *byte = (i.wrapping_mul(37).wrapping_add(13)) as u8;
                        }
                        b.as_mut_slice().fill(0xCC);
                        cpu.df = df;
                        cpu.gpr[1] = cnt as u64;
                        let base_off = if df { 600 + off } else { off };
                        cpu.gpr[6] = a.addr() + base_off as u64;
                        cpu.gpr[7] = b.addr() + base_off as u64;
                    });
                }
            }
        }
        // Overlapping cases within the same buffer (both DF=0 and DF=1, including dist < elem & non-multiples)
        for &(dist, df) in &[(0isize, false), (1, false), (3, false), (4, false), (6, false), (-5, false), (0, true), (1, true), (3, true), (6, true), (-1, true), (-3, true), (-6, true)] {
            let cnt = 24usize;
            diff_seq(&format!("{name}_ovlp_d{dist}_df{df}"), code, 1024, 64, |cpu, a, _| {
                for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                    *byte = (i.wrapping_mul(53).wrapping_add(7)) as u8;
                }
                cpu.df = df;
                cpu.gpr[1] = cnt as u64;
                let src_off = if df { 512isize } else { 128isize };
                let dst_off = src_off + dist;
                assert!(dst_off >= (cnt * elem) as isize || !df);
                cpu.gpr[6] = a.addr() + src_off as u64;
                cpu.gpr[7] = a.addr() + dst_off as u64;
            });
        }
    }
}

#[test]
fn fast_rep_stos_and_lods_differential() {
    let stos_forms: &[(&str, &[u8])] = &[
        ("stosb", &[0xAA]),
        ("rep_stosb", &[0xF3, 0xAA]),
        ("rep_stosw", &[0xF3, 0x66, 0xAB]),
        ("rep_stosd", &[0xF3, 0xAB]),
        ("rep_stosq", &[0xF3, 0x48, 0xAB]),
    ];
    let patterns = [0u64, 0xA5A5_A5A5_A5A5_A5A5, 0x1122_3344_5566_7788];
    for &(name, code) in stos_forms {
        for &pat in &patterns {
            for &df in &[false, true] {
                for &cnt in &[0usize, 1, 3, 7, 15, 16, 17, 65] {
                    for &off in &[0usize, 1, 3] {
                        diff_seq(&format!("{name}_{pat:#x}_df{df}_c{cnt}_o{off}"), code, 1024, 64, |cpu, a, _| {
                            a.as_mut_slice().fill(0xEE);
                            cpu.df = df;
                            cpu.gpr[0] = pat;
                            cpu.gpr[1] = cnt as u64;
                            cpu.gpr[7] = a.addr() + if df { 700 + off as u64 } else { off as u64 };
                        });
                    }
                }
            }
        }
    }
    let lods_forms: &[(&str, &[u8])] = &[
        ("lodsb", &[0xAC]),
        ("rep_lodsb", &[0xF3, 0xAC]),
        ("rep_lodsw", &[0xF3, 0x66, 0xAD]),
        ("rep_lodsd", &[0xF3, 0xAD]),
        ("rep_lodsq", &[0xF3, 0x48, 0xAD]),
    ];
    for &(name, code) in lods_forms {
        for &df in &[false, true] {
            for &cnt in &[0usize, 1, 5] {
                diff_seq(&format!("{name}_df{df}_c{cnt}"), code, 512, 64, |cpu, a, _| {
                    for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                        *byte = (i.wrapping_mul(19).wrapping_add(3)) as u8;
                    }
                    cpu.df = df;
                    cpu.gpr[1] = cnt as u64;
                    cpu.gpr[6] = a.addr() + if df { 256 } else { 5 };
                });
            }
        }
    }
}

#[test]
fn fast_neon_scasb_differential() {
    // Test repne scasb (F2 AE), repe scasb (F3 AE), single scasb (AE), and repne scasw/d/q.
    let cases: &[(&str, &[u8])] = &[
        ("repne_scasb", &[0xF2, 0xAE]),
        ("repe_scasb", &[0xF3, 0xAE]),
        ("scasb", &[0xAE]),
        ("repne_scasw", &[0xF2, 0x66, 0xAF]),
        ("repe_scasd", &[0xF3, 0xAF]),
        ("repne_scasq", &[0xF2, 0x48, 0xAF]),
    ];
    for &(name, code) in cases {
        for &cnt in &[0usize, 1, 7, 15, 16, 17, 32, 37, 80] {
            for &match_pos in &[None, Some(0usize), Some(7), Some(15), Some(16), Some(31), Some(36)] {
                for &off in &[0usize, 1, 3, 15] {
                    diff_seq(&format!("{name}_c{cnt}_m{match_pos:?}_o{off}"), code, 1024, 64, |cpu, a, _| {
                        let is_repe = name.starts_with("repe_");
                        let fill = if is_repe { 0x41u8 } else { 0x80u8 };
                        a.as_mut_slice().fill(fill);
                        if let Some(pos) = match_pos {
                            if off + pos * 8 + 8 <= 512 {
                                // Place differing or matching byte/element
                                let val = if is_repe { 0x01u8 } else { 0x41u8 };
                                if name.ends_with('b') {
                                    a.as_mut_slice()[off + pos] = val;
                                } else {
                                    for k in 0..8 {
                                        a.as_mut_slice()[off + pos * 8 + k] = val;
                                    }
                                }
                            }
                        }
                        cpu.df = false;
                        cpu.gpr[0] = 0x4141_4141_4141_4141;
                        cpu.gpr[1] = cnt as u64;
                        cpu.gpr[7] = a.addr() + off as u64;
                    });
                }
            }
        }
    }
    // 4 KiB page-boundary straddling test for NEON `repne`/`repe scasb` + DF=1 backward scan.
    for &(name, code, is_repe) in &[("repne_page", &[0xF2u8, 0xAE][..], false), ("repe_page", &[0xF3u8, 0xAE][..], true)] {
        for &start_off in &[4080usize, 4088, 4095, 4096] {
            for &hit_rel in &[None, Some(3usize), Some(8), Some(16), Some(35)] {
                diff_seq(&format!("{name}_s{start_off}_h{hit_rel:?}"), code, 8192, 4096, |cpu, a, _| {
                    a.as_mut_slice().fill(if is_repe { 0x55 } else { 0xAA });
                    if let Some(rel) = hit_rel {
                        a.as_mut_slice()[start_off + rel] = if is_repe { 0xFF } else { 0x55 };
                    }
                    cpu.df = false;
                    cpu.gpr[0] = 0x55;
                    cpu.gpr[1] = 48;
                    cpu.gpr[7] = a.addr() + start_off as u64;
                });
            }
        }
        // DF=1 backward scan
        diff_seq(&format!("{name}_df1"), code, 512, 64, |cpu, a, _| {
            a.as_mut_slice().fill(if is_repe { 0x55 } else { 0xAA });
            a.as_mut_slice()[180] = if is_repe { 0x11 } else { 0x55 };
            cpu.df = true;
            cpu.gpr[0] = 0x55;
            cpu.gpr[1] = 40;
            cpu.gpr[7] = a.addr() + 200;
        });
    }
}

#[test]
fn fast_neon_cmpsb_differential() {
    let cases: &[(&str, &[u8], bool)] = &[
        ("repe_cmpsb", &[0xF3, 0xA6], false),
        ("repne_cmpsb", &[0xF2, 0xA6], true),
        ("cmpsb", &[0xA6], false),
        ("repe_cmpsw", &[0xF3, 0x66, 0xA7], false),
        ("repe_cmpsd", &[0xF3, 0xA7], false),
        ("repe_cmpsq", &[0xF3, 0x48, 0xA7], false),
    ];
    for &(name, code, stop_on_eq) in cases {
        for &cnt in &[0usize, 1, 7, 15, 16, 17, 32, 45] {
            for &hit in &[None, Some(0usize), Some(1), Some(15), Some(16), Some(31), Some(40)] {
                diff_seq(&format!("{name}_c{cnt}_h{hit:?}"), code, 1024, 64, |cpu, a, b| {
                    for i in 0..1024 {
                        let v = ((i * 29 + 11) & 0xFF) as u8;
                        a.as_mut_slice()[i] = v;
                        b.as_mut_slice()[i] = if stop_on_eq { v ^ 0x55 } else { v };
                    }
                    let off_a = 1usize;
                    let off_b = 5usize;
                    // Re-sync relative offsets
                    for i in 0..512 {
                        let v = ((i * 29 + 11) & 0xFF) as u8;
                        a.as_mut_slice()[off_a + i] = v;
                        b.as_mut_slice()[off_b + i] = if stop_on_eq { v.wrapping_add(1) } else { v };
                    }
                    if let Some(pos) = hit {
                        if name.ends_with('b') {
                            b.as_mut_slice()[off_b + pos] = if stop_on_eq {
                                a.as_slice()[off_a + pos]
                            } else {
                                a.as_slice()[off_a + pos].wrapping_add(0x7F)
                            };
                        } else if off_b + pos * 8 + 8 <= 1024 {
                            b.as_mut_slice()[off_b + pos * 8] ^= 0x80;
                        }
                    }
                    cpu.df = false;
                    cpu.gpr[1] = cnt as u64;
                    cpu.gpr[6] = a.addr() + off_a as u64;
                    cpu.gpr[7] = b.addr() + off_b as u64;
                });
            }
        }
    }
    // Page-straddling with mismatched page offsets for `p1` and `p2` + DF=1.
    for &(name, code, stop_on_eq) in &[("repe_cmpsb_page", &[0xF3u8, 0xA6][..], false), ("repne_cmpsb_page", &[0xF2u8, 0xA6][..], true)] {
        for &hit in &[None, Some(4usize), Some(15), Some(16), Some(47)] {
            diff_seq(&format!("{name}_h{hit:?}"), code, 8192, 4096, |cpu, a, b| {
                let s1 = 4088usize;
                let s2 = 4092usize;
                for i in 0..64 {
                    let v = (i * 13 + 5) as u8;
                    a.as_mut_slice()[s1 + i] = v;
                    b.as_mut_slice()[s2 + i] = if stop_on_eq { v ^ 0xFF } else { v };
                }
                if let Some(pos) = hit {
                    b.as_mut_slice()[s2 + pos] = if stop_on_eq {
                        a.as_slice()[s1 + pos]
                    } else {
                        a.as_slice()[s1 + pos].wrapping_sub(3)
                    };
                }
                cpu.df = false;
                cpu.gpr[1] = 48;
                cpu.gpr[6] = a.addr() + s1 as u64;
                cpu.gpr[7] = b.addr() + s2 as u64;
            });
        }
        diff_seq(&format!("{name}_df1"), code, 512, 64, |cpu, a, b| {
            a.as_mut_slice().fill(0x22);
            b.as_mut_slice().fill(if stop_on_eq { 0x33 } else { 0x22 });
            b.as_mut_slice()[190] = if stop_on_eq { 0x22 } else { 0x99 };
            cpu.df = true;
            cpu.gpr[1] = 30;
            cpu.gpr[6] = a.addr() + 200;
            cpu.gpr[7] = b.addr() + 200;
        });
    }
}

#[test]
fn fast_neon_simd_vshuf_vint_differential() {
    // Test edge cases: pabs*(MIN), pack* saturation, pshufb bits 6..4, pmulhrsw(-32768*-32768),
    // pmulhw/huw/udq/dq/maddwd, phadd/phsub, psadbw, pmovmskb/vpmovmskb/movmskps, pclmulqdq,
    // pmovsx/zx, vblendv*, and VEX.128 upper-lane zeroing vs legacy SSE preservation!
    let insns: &[(&str, &[u8])] = &[
        ("pabsb_leg", &[0x66, 0x0F, 0x38, 0x1C, 0xC1]),               // pabsb xmm0, xmm1
        ("vpabsb_128", &[0xC4, 0xE2, 0x79, 0x1C, 0xC1]),              // vpabsb xmm0, xmm1
        ("vpabsb_256", &[0xC4, 0xE2, 0x7D, 0x1C, 0xC1]),              // vpabsb ymm0, ymm1
        ("pabsw_leg", &[0x66, 0x0F, 0x38, 0x1D, 0xC1]),               // pabsw xmm0, xmm1
        ("pabsd_leg", &[0x66, 0x0F, 0x38, 0x1E, 0xC1]),               // pabsd xmm0, xmm1
        ("packsswb", &[0x66, 0x0F, 0x63, 0xC1]),                      // packsswb xmm0, xmm1
        ("packuswb", &[0x66, 0x0F, 0x67, 0xC1]),                      // packuswb xmm0, xmm1
        ("packssdw", &[0x66, 0x0F, 0x6B, 0xC1]),                      // packssdw xmm0, xmm1
        ("packusdw", &[0x66, 0x0F, 0x38, 0x2B, 0xC1]),                // packusdw xmm0, xmm1
        ("vpackuswb_128", &[0xC5, 0xF9, 0x67, 0xC2]),                 // vpackuswb xmm0, xmm0, xmm2
        ("pshufb", &[0x66, 0x0F, 0x38, 0x00, 0xC2]),                  // pshufb xmm0, xmm2
        ("vpshufb_128", &[0xC4, 0xE2, 0x79, 0x00, 0xC2]),             // vpshufb xmm0, xmm0, xmm2
        ("vpshufb_256", &[0xC4, 0xE2, 0x7D, 0x00, 0xC2]),             // vpshufb ymm0, ymm0, ymm2
        ("pmulhrsw", &[0x66, 0x0F, 0x38, 0x0B, 0xC1]),                // pmulhrsw xmm0, xmm1
        ("pmulhw", &[0x66, 0x0F, 0xE5, 0xC1]),                        // pmulhw xmm0, xmm1
        ("pmulhuw", &[0x66, 0x0F, 0xE4, 0xC1]),                       // pmulhuw xmm0, xmm1
        ("pmuludq", &[0x66, 0x0F, 0xF4, 0xC1]),                       // pmuludq xmm0, xmm1
        ("pmuldq", &[0x66, 0x0F, 0x38, 0x28, 0xC1]),                  // pmuldq xmm0, xmm1
        ("pmaddwd", &[0x66, 0x0F, 0xF5, 0xC1]),                       // pmaddwd xmm0, xmm1
        ("phaddw", &[0x66, 0x0F, 0x38, 0x01, 0xC1]),                  // phaddw xmm0, xmm1
        ("phaddd", &[0x66, 0x0F, 0x38, 0x02, 0xC1]),                  // phaddd xmm0, xmm1
        ("phsubw", &[0x66, 0x0F, 0x38, 0x05, 0xC1]),                  // phsubw xmm0, xmm1
        ("phsubd", &[0x66, 0x0F, 0x38, 0x06, 0xC1]),                  // phsubd xmm0, xmm1
        ("psadbw", &[0x66, 0x0F, 0xF6, 0xC1]),                        // psadbw xmm0, xmm1
        ("pavgb", &[0x66, 0x0F, 0xE0, 0xC1]),                         // pavgb xmm0, xmm1
        ("pavgw", &[0x66, 0x0F, 0xE3, 0xC1]),                         // pavgw xmm0, xmm1
        ("pcmpeqb", &[0x66, 0x0F, 0x74, 0xC1]),                       // pcmpeqb xmm0, xmm1
        ("vpcmpeqb_128", &[0xC5, 0xF9, 0x74, 0xC1]),                  // vpcmpeqb xmm0, xmm0, xmm1
        ("pcmpgtb", &[0x66, 0x0F, 0x64, 0xC1]),                       // pcmpgtb xmm0, xmm1
        ("pcmpgtq", &[0x66, 0x0F, 0x38, 0x37, 0xC1]),                 // pcmpgtq xmm0, xmm1
        ("pmovmskb", &[0x66, 0x0F, 0xD7, 0xC1]),                      // pmovmskb eax, xmm1
        ("vpmovmskb_256", &[0xC5, 0xFD, 0xD7, 0xC1]),                 // vpmovmskb eax, ymm1
        ("movmskps", &[0x0F, 0x50, 0xC1]),                            // movmskps eax, xmm1
        ("vmovmskps_256", &[0xC5, 0xFC, 0x50, 0xC1]),                 // vmovmskps eax, ymm1
        ("pclmulqdq_00", &[0x66, 0x0F, 0x3A, 0x44, 0xC1, 0x00]),      // pclmulqdq xmm0, xmm1, 0x00
        ("pclmulqdq_01", &[0x66, 0x0F, 0x3A, 0x44, 0xC1, 0x01]),      // pclmulqdq xmm0, xmm1, 0x01
        ("pclmulqdq_10", &[0x66, 0x0F, 0x3A, 0x44, 0xC1, 0x10]),      // pclmulqdq xmm0, xmm1, 0x10
        ("vpclmulqdq_11", &[0xC4, 0xE3, 0x79, 0x44, 0xC1, 0x11]),     // vpclmulqdq xmm0, xmm0, xmm1, 0x11
        ("pmovsxbw", &[0x66, 0x0F, 0x38, 0x20, 0xC1]),                // pmovsxbw xmm0, xmm1
        ("vpmovzxbw_256", &[0xC4, 0xE2, 0x7D, 0x30, 0xC1]),           // vpmovzxbw ymm0, xmm1
        ("vpmovsxbd_256", &[0xC4, 0xE2, 0x7D, 0x21, 0xC1]),           // vpmovsxbd ymm0, xmm1
        ("vpmovzxbq_256", &[0xC4, 0xE2, 0x7D, 0x32, 0xC1]),           // vpmovzxbq ymm0, xmm1
        ("vpmovsxwd_256", &[0xC4, 0xE2, 0x7D, 0x23, 0xC1]),           // vpmovsxwd ymm0, xmm1
        ("vpmovzxwq_256", &[0xC4, 0xE2, 0x7D, 0x34, 0xC1]),           // vpmovzxwq ymm0, xmm1
        ("vpmovsxdq_256", &[0xC4, 0xE2, 0x7D, 0x25, 0xC1]),           // vpmovsxdq ymm0, xmm1
        ("vpmovzxdq_256", &[0xC4, 0xE2, 0x7D, 0x35, 0xC1]),           // vpmovzxdq ymm0, xmm1
        ("pblendvb", &[0x66, 0x0F, 0x38, 0x10, 0xC1]),                // pblendvb xmm0, xmm1, xmm0
        ("vpblendvb_256", &[0xC4, 0xE3, 0x7D, 0x4C, 0xC1, 0x20]),     // vpblendvb ymm0, ymm0, ymm1, ymm2
        ("vblendvps_256", &[0xC4, 0xE3, 0x7D, 0x4A, 0xC1, 0x20]),     // vblendvps ymm0, ymm0, ymm1, ymm2
        ("vblendvpd_256", &[0xC4, 0xE3, 0x7D, 0x4B, 0xC1, 0x20]),     // vblendvpd ymm0, ymm0, ymm1, ymm2
        ("punpcklbw", &[0x66, 0x0F, 0x60, 0xC1]),                     // punpcklbw xmm0, xmm1
        ("punpckhwd", &[0x66, 0x0F, 0x69, 0xC1]),                     // punpckhwd xmm0, xmm1
        ("punpckldq", &[0x66, 0x0F, 0x62, 0xC1]),                     // punpckldq xmm0, xmm1
        ("punpckhdq", &[0x66, 0x0F, 0x6A, 0xC1]),                     // punpckhdq xmm0, xmm1
    ];
    for &(name, code) in insns {
        diff_seq(name, code, 128, 16, |cpu, _, _| {
            // Edge-case lanes: i8::MIN, i16::MIN, i32::MIN, saturation boundaries, control bytes with bits 6..4 set!
            cpu.xmm[0] = u128::from_le_bytes([
                0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0x00, 0x01, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x00, 0xFE, 0xFF,
            ]);
            cpu.xmm[1] = u128::from_le_bytes([
                0x00, 0x80, 0x80, 0x80, 0x00, 0x40, 0x01, 0x40, 0x00, 0x80, 0x7F, 0xFF, 0x00, 0x01, 0x80, 0x7F,
            ]);
            // pshufb control bytes with bits 6..4 set (`0x31`, `0x7F`, `0x52`, `0x80`, `0xFF`)
            cpu.xmm[2] = u128::from_le_bytes([
                0x00, 0x31, 0x7F, 0x80, 0x52, 0x63, 0xFF, 0x90, 0x1E, 0x2D, 0x4C, 0x85, 0x0A, 0x70, 0x81, 0x0F,
            ]);
            cpu.ymm_hi[0] = 0x8000_7FFF_FFFF_0001_8080_7F7F_0100_FFFE;
            cpu.ymm_hi[1] = 0x8000_8000_0001_FFFF_4001_4000_8000_7FFF;
            cpu.ymm_hi[2] = 0xFF80_7F31_6352_9000_4C2D_1E85_0F81_700A;
        });
    }
}

#[test]
fn fast_neon_vfp_and_fma_differential() {
    let insns: &[(&str, &[u8])] = &[
        ("addps", &[0x0F, 0x58, 0xC1]),
        ("subps", &[0x0F, 0x5C, 0xC1]),
        ("mulps", &[0x0F, 0x59, 0xC1]),
        ("divps", &[0x0F, 0x5E, 0xC1]),
        ("minps", &[0x0F, 0x5D, 0xC1]),
        ("maxps", &[0x0F, 0x5F, 0xC1]),
        ("sqrtps", &[0x0F, 0x51, 0xC1]),
        ("addpd", &[0x66, 0x0F, 0x58, 0xC1]),
        ("subpd", &[0x66, 0x0F, 0x5C, 0xC1]),
        ("mulpd", &[0x66, 0x0F, 0x59, 0xC1]),
        ("divpd", &[0x66, 0x0F, 0x5E, 0xC1]),
        ("minpd", &[0x66, 0x0F, 0x5D, 0xC1]),
        ("maxpd", &[0x66, 0x0F, 0x5F, 0xC1]),
        ("sqrtpd", &[0x66, 0x0F, 0x51, 0xC1]),
        ("cvtdq2ps", &[0x0F, 0x5B, 0xC1]),
        ("roundps_0", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x00]),
        ("roundps_1", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x01]),
        ("roundps_2", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x02]),
        ("roundps_3", &[0x66, 0x0F, 0x3A, 0x08, 0xC1, 0x03]),
        ("roundpd_0", &[0x66, 0x0F, 0x3A, 0x09, 0xC1, 0x00]),
        ("haddps", &[0xF2, 0x0F, 0x7C, 0xC1]),
        ("hsubps", &[0xF2, 0x0F, 0x7D, 0xC1]),
        ("vaddps_128", &[0xC5, 0xF8, 0x58, 0xC1]),
        ("vfmadd213ps_256", &[0xC4, 0xE2, 0x75, 0xA8, 0xC2]),
        ("vfmsub213ps_256", &[0xC4, 0xE2, 0x75, 0xAA, 0xC2]),
        ("vfnmadd213ps_256", &[0xC4, 0xE2, 0x75, 0xAC, 0xC2]),
        ("vfnmsub213ps_256", &[0xC4, 0xE2, 0x75, 0xAE, 0xC2]),
        ("vfmaddsub213ps_256", &[0xC4, 0xE2, 0x75, 0xA6, 0xC2]),
        ("vfmsubadd213ps_256", &[0xC4, 0xE2, 0x75, 0xA7, 0xC2]),
        ("vfmadd213pd_256", &[0xC4, 0xE2, 0xF5, 0xA8, 0xC2]),
        ("vfmaddsub213pd_256", &[0xC4, 0xE2, 0xF5, 0xA6, 0xC2]),
    ];
    // Test 1: finite normal/negative/zero/half values
    let f32v = |arr: [f32; 4]| -> u128 { pk::<u32, 4>(arr.map(f32::to_bits)) };
    let f64v = |arr: [f64; 2]| -> u128 { pk::<u64, 2>(arr.map(f64::to_bits)) };
    let datasets: &[(&str, u128, u128, u128)] = &[
        ("finite", f32v([1.5, -2.5, 0.0, -0.0]), f32v([-0.0, 0.0, 3.25, -4.75]), f32v([0.5, -1.5, 2.5, -3.5])),
        (
            "inf_nan",
            pk::<u32, 4>([f32::INFINITY.to_bits(), 0x7FC0_0011, 0x7F80_0005, (-1.0f32).to_bits()]),
            pk::<u32, 4>([f32::INFINITY.to_bits(), 0x7FC0_0022, 1.0f32.to_bits(), (-0.0f32).to_bits()]),
            f32v([1.0, 2.0, -3.0, 4.0]),
        ),
        (
            "f64_special",
            f64v([0.0, f64::NEG_INFINITY]),
            pk::<u64, 2>([(-0.0f64).to_bits(), 0x7FF8_0000_0000_00AA]),
            f64v([2.5, -3.5]),
        ),
    ];
    for &(dname, a, b, c) in datasets {
        for &(name, code) in insns {
            diff_seq(&format!("{name}_{dname}"), code, 128, 16, |cpu, _, _| {
                cpu.xmm[0] = a;
                cpu.xmm[1] = b;
                cpu.xmm[2] = c;
                cpu.ymm_hi[0] = b;
                cpu.ymm_hi[1] = c;
                cpu.ymm_hi[2] = a;
            });
        }
    }
}

#[test]
fn fast_aes_f16c_vmaskmov_crc32_differential() {
    let insns: &[(&str, &[u8])] = &[
        // Legacy AES (must preserve ymm_hi[0])
        ("aesenc", &[0x66, 0x0F, 0x38, 0xDC, 0xC1]),
        ("aesenclast", &[0x66, 0x0F, 0x38, 0xDD, 0xC1]),
        ("aesdec", &[0x66, 0x0F, 0x38, 0xDE, 0xC1]),
        ("aesdeclast", &[0x66, 0x0F, 0x38, 0xDF, 0xC1]),
        ("aesimc", &[0x66, 0x0F, 0x38, 0xDB, 0xC1]),
        ("aeskeygenassist", &[0x66, 0x0F, 0x3A, 0xDF, 0xC1, 0x36]),
        // VEX AES (must zero ymm_hi[0]) + memory form
        ("vaesenc", &[0xC4, 0xE2, 0x71, 0xDC, 0xC2]),              // vaesenc xmm0, xmm1, xmm2
        ("vaesenc_m", &[0xC4, 0xE2, 0x71, 0xDC, 0x07]),            // vaesenc xmm0, xmm1, [rdi]
        ("vaesenclast", &[0xC4, 0xE2, 0x71, 0xDD, 0xC2]),
        ("vaesdec", &[0xC4, 0xE2, 0x71, 0xDE, 0xC2]),
        ("vaesdeclast", &[0xC4, 0xE2, 0x71, 0xDF, 0xC2]),
        ("vaesimc", &[0xC4, 0xE2, 0x79, 0xDB, 0xC1]),
        ("vaeskeygenassist", &[0xC4, 0xE3, 0x79, 0xDF, 0xC1, 0x1B]),
        // F16C
        ("vcvtph2ps_128", &[0xC4, 0xE2, 0x79, 0x13, 0xC1]),
        ("vcvtph2ps_256", &[0xC4, 0xE2, 0x7D, 0x13, 0xC1]),
        ("vcvtph2ps_128_m", &[0xC4, 0xE2, 0x79, 0x13, 0x07]),
        ("vcvtps2ph_128", &[0xC4, 0xE3, 0x79, 0x1D, 0xC8, 0x00]),  // vcvtps2ph xmm0, xmm1, 0
        ("vcvtps2ph_256", &[0xC4, 0xE3, 0x7D, 0x1D, 0xC8, 0x00]),  // vcvtps2ph xmm0, ymm1, 0
        ("vcvtps2ph_256_m", &[0xC4, 0xE3, 0x7D, 0x1D, 0x0F, 0x00]), // vcvtps2ph [rdi], ymm1, 0
        // VMASKMOV (loads & stores, 128 & 256)
        ("vpmaskmovd_ld_128", &[0xC4, 0xE2, 0x71, 0x8C, 0x07]),    // vpmaskmovd xmm0, xmm1, [rdi]
        ("vpmaskmovd_ld_256", &[0xC4, 0xE2, 0x75, 0x8C, 0x07]),    // vpmaskmovd ymm0, ymm1, [rdi]
        ("vpmaskmovq_st_256", &[0xC4, 0xE2, 0xF5, 0x8E, 0x17]),    // vpmaskmovq [rdi], ymm1, ymm2
        // CRC32 (8/16/32/64-bit src, reg & mem)
        ("crc32_r32_r8", &[0xF2, 0x0F, 0x38, 0xF0, 0xC3]),         // crc32 eax, bl
        ("crc32_r32_r16", &[0x66, 0xF2, 0x0F, 0x38, 0xF1, 0xC3]),  // crc32 eax, bx
        ("crc32_r32_r32", &[0xF2, 0x0F, 0x38, 0xF1, 0xC3]),        // crc32 eax, ebx
        ("crc32_r64_r64", &[0xF2, 0x48, 0x0F, 0x38, 0xF1, 0xC3]),  // crc32 rax, rbx
        ("crc32_r64_m64", &[0xF2, 0x48, 0x0F, 0x38, 0xF1, 0x07]),  // crc32 rax, [rdi]
    ];
    for &(name, code) in insns {
        diff_seq(name, code, 256, 32, |cpu, a, _| {
            for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                *byte = (i.wrapping_mul(41).wrapping_add(9)) as u8;
            }
            cpu.gpr[7] = a.addr() + 3; // unaligned!
            // Half-float test values: +0, -0, 1.0 (0x3C00), subnormal (0x0001), -Inf (0xFC00), qNaN (0x7E01), 65504 (0x7BFF), -2.5 (0xC100)
            cpu.xmm[1] = u128::from_le_bytes([
                0x00, 0x00, 0x00, 0x80, 0x00, 0x3C, 0x01, 0x00, 0x00, 0xFC, 0x01, 0x7E, 0xFF, 0x7B, 0x00, 0xC1,
            ]);
            cpu.ymm_hi[1] = 0x8000_0000_0000_0000_8000_0000_0000_0000;
        });
    }
}

#[test]
fn fast_pcmpstr_differential() {
    // Test all 4 agg modes (0..4), unsigned/signed bytes & words, polarities (0x00, 0x10, 0x30),
    // imm bit 6 (0/1), implicit & explicit, index & mask!
    let imms: &[u8] = &[
        0x00, 0x02, 0x04, 0x06, 0x08, 0x0C, 0x10, 0x18, 0x38, 0x40, 0x48, 0x4C, 0x58, 0x6E, 0x78, // bytes
        0x01, 0x05, 0x07, 0x09, 0x0D, 0x19, 0x39, 0x49, 0x6F, 0x79, // words
    ];
    for &imm in imms {
        let forms: &[(&str, [u8; 6])] = &[
            ("pcmpistri", [0x66, 0x0F, 0x3A, 0x63, 0xC1, imm]),
            ("pcmpistrm", [0x66, 0x0F, 0x3A, 0x62, 0xC1, imm]),
            ("pcmpestri", [0x66, 0x0F, 0x3A, 0x61, 0xC1, imm]),
            ("pcmpestrm", [0x66, 0x0F, 0x3A, 0x60, 0xC1, imm]),
        ];
        for &(name, code) in forms {
            diff_seq(&format!("{name}_{imm:#04x}"), &code, 128, 16, |cpu, _, _| {
                cpu.gpr[0] = 5;  // eax = explicit len a
                cpu.gpr[2] = 11; // edx = explicit len b
                cpu.xmm[0] = u128::from_le_bytes(*b"a\x7F\x80\xFEz\0bcdefghijk");
                cpu.xmm[1] = u128::from_le_bytes(*b"hello a\x7F\x80w\0xyz12");
            });
            // Empty needle (`a_len == 0`) edge case (especially Equal Ordered with polarity/MSB)
            diff_seq(&format!("{name}_empty_{imm:#04x}"), &code, 128, 16, |cpu, _, _| {
                cpu.gpr[0] = 0;
                cpu.gpr[2] = 0;
                cpu.xmm[0] = 0;
                cpu.xmm[1] = 0;
            });
        }
    }
}

#[test]
fn fast_hi8_sys_misc_bit_ext_atomic_differential() {
    let cases: &[(&str, &[u8])] = &[
        // K_HI8
        ("hi8_mov_ri", &[0xB4, 0x7A]),                         // mov ah, 0x7A
        ("hi8_mov_rr", &[0x88, 0xFC]),                         // mov ah, bh
        ("hi8_mov_rm", &[0x8A, 0x27]),                         // mov ah, [rdi]
        ("hi8_mov_mr", &[0x88, 0x27]),                         // mov [rdi], ah
        ("hi8_movzx", &[0x0F, 0xB6, 0xC4]),                    // movzx eax, ah
        ("hi8_movsx", &[0x0F, 0xBE, 0xC4]),                    // movsx eax, ah
        ("hi8_add_rr", &[0x00, 0xFC]),                         // add ah, bh
        ("hi8_sub_ri", &[0x80, 0xEC, 0x55]),                   // sub ah, 0x55
        ("hi8_cmp_rm", &[0x3A, 0x27]),                         // cmp ah, [rdi]
        ("hi8_test_mr", &[0x84, 0x27]),                        // test [rdi], ah
        ("hi8_inc", &[0xFE, 0xC4]),                            // inc ah
        ("hi8_dec", &[0xFE, 0xCC]),                            // dec ah
        ("hi8_neg", &[0xF6, 0xDC]),                            // neg ah
        ("hi8_not", &[0xF6, 0xD4]),                            // not ah
        ("hi8_shl", &[0xC0, 0xE4, 0x03]),                      // shl ah, 3
        ("hi8_sar", &[0xC0, 0xFC, 0x02]),                      // sar ah, 2
        ("hi8_rol", &[0xC0, 0xC4, 0x05]),                      // rol ah, 5
        ("hi8_ror", &[0xC0, 0xCC, 0x03]),                      // ror ah, 3
        ("hi8_setc", &[0x0F, 0x92, 0xC4]),                     // setc ah
        ("hi8_xchg", &[0x86, 0xE7]),                           // xchg ah, bh
        ("hi8_xadd", &[0x0F, 0xC0, 0xFC]),                     // xadd ah, bh
        ("hi8_cmpxchg", &[0x0F, 0xB0, 0xFC]),                  // cmpxchg ah, bh
        // K_SYS_MISC
        ("lahf", &[0x9F]),
        ("sahf", &[0x9E]),
        ("pushfq_popfq", &[0x9C, 0xF8, 0x9D]),                 // pushfq; clc; popfq
        ("pushf_popf", &[0x66, 0x9C, 0xF9, 0x66, 0x9D]),       // pushf; stc; popf
        ("stmxcsr", &[0x0F, 0xAE, 0x1F]),                      // stmxcsr [rdi]
        ("fnstcw", &[0x9B, 0xD9, 0x3F]),                       // fnstcw [rdi]
        ("xlatb", &[0xD7]),
        ("xgetbv", &[0x0F, 0x01, 0xD0]),
        ("rdtsc", &[0x0F, 0x31]),
        ("rdtscp", &[0x0F, 0x01, 0xF9]),
        ("cpuid_0", &[0x31, 0xC0, 0x0F, 0xA2]),
        ("cpuid_1", &[0xB8, 0x01, 0x00, 0x00, 0x00, 0x0F, 0xA2]),
        ("cpuid_7", &[0xB8, 0x07, 0x00, 0x00, 0x00, 0x0F, 0xA2]),
        ("push16_pop16", &[0x66, 0x50, 0x66, 0x5B]),           // push ax; pop bx
        // K_BIT_EXT
        ("pdep_64", &[0xC4, 0xE2, 0xF3, 0xF5, 0xC3]),          // pdep rax, rcx, rbx
        ("pext_64", &[0xC4, 0xE2, 0xF2, 0xF5, 0xC3]),          // pext rax, rcx, rbx
        ("pdep_32_m", &[0xC4, 0xE2, 0x73, 0xF5, 0x07]),        // pdep eax, ecx, [rdi]
        ("rcl_64_i", &[0x48, 0xC1, 0xD0, 0x07]),               // rcl rax, 7
        ("rcr_8_i", &[0xC0, 0xD8, 0x0B]),                      // rcr al, 11 (tests % 9!)
        ("rcl_32_m", &[0xD3, 0x17]),                           // rcl dword ptr [rdi], cl
        // K_ATOMIC
        ("lock_add", &[0xF0, 0x48, 0x01, 0x07]),               // lock add [rdi], rax
        ("lock_sub", &[0xF0, 0x48, 0x29, 0x07]),               // lock sub [rdi], rax
        ("lock_and", &[0xF0, 0x48, 0x21, 0x07]),               // lock and [rdi], rax
        ("lock_or", &[0xF0, 0x48, 0x09, 0x07]),                // lock or [rdi], rax
        ("lock_xor", &[0xF0, 0x48, 0x31, 0x07]),               // lock xor [rdi], rax
        ("lock_inc", &[0xF0, 0x48, 0xFF, 0x07]),               // lock inc qword ptr [rdi]
        ("lock_dec", &[0xF0, 0x48, 0xFF, 0x0F]),               // lock dec qword ptr [rdi]
        ("lock_not", &[0xF0, 0x48, 0xF7, 0x17]),               // lock not qword ptr [rdi]
        ("lock_neg", &[0xF0, 0x48, 0xF7, 0x1F]),               // lock neg qword ptr [rdi]
        ("xchg_mr", &[0x48, 0x87, 0x07]),                      // xchg [rdi], rax
        ("lock_xadd", &[0xF0, 0x48, 0x0F, 0xC1, 0x07]),        // lock xadd [rdi], rax
        ("lock_cmpxchg_ok", &[0xF0, 0x48, 0x0F, 0xB1, 0x0F]),  // lock cmpxchg [rdi], rcx
        ("lock_cmpxchg8b", &[0xF0, 0x0F, 0xC7, 0x0F]),         // lock cmpxchg8b [rdi]
        ("lock_cmpxchg16b", &[0xF0, 0x48, 0x0F, 0xC7, 0x0F]),  // lock cmpxchg16b [rdi]
        ("bt_m_imm", &[0x48, 0x0F, 0xBA, 0x27, 0x25]),         // bt qword ptr [rdi], 37
        ("bts_m_neg", &[0x48, 0x0F, 0xAB, 0x17]),              // bts qword ptr [rdi], rdx (rdx = -19!)
        ("btr_m_pos", &[0x48, 0x0F, 0xB3, 0x0F]),              // btr qword ptr [rdi], rcx
        ("btc_m_neg", &[0x48, 0x0F, 0xBB, 0x17]),              // btc qword ptr [rdi], rdx
    ];
    for &(name, code) in cases {
        diff_seq(name, code, 512, 64, |cpu, a, b| {
            for (i, byte) in a.as_mut_slice().iter_mut().enumerate() {
                *byte = (i.wrapping_mul(17).wrapping_add(5)) as u8;
            }
            for (i, byte) in b.as_mut_slice().iter_mut().enumerate() {
                *byte = (i.wrapping_mul(23).wrapping_add(1)) as u8;
            }
            cpu.gpr[4] = b.addr() + 256; // rsp
            cpu.gpr[7] = a.addr() + 64;  // rdi (16-byte aligned, with room below for negative `bts`!)
            if name == "xlatb" {
                cpu.gpr[3] = a.addr();   // rbx = table base
                cpu.gpr[0] = 0x1234_002A;
            } else if name == "lock_cmpxchg_ok" {
                let cur = u64::from_le_bytes(a.as_slice()[64..72].try_into().unwrap());
                cpu.gpr[0] = cur;
            } else if name == "lock_cmpxchg8b" {
                let cur = u64::from_le_bytes(a.as_slice()[64..72].try_into().unwrap());
                cpu.gpr[0] = cur as u32 as u64;
                cpu.gpr[2] = cur >> 32;
            } else if name == "lock_cmpxchg16b" {
                let lo = u64::from_le_bytes(a.as_slice()[64..72].try_into().unwrap());
                let hi = u64::from_le_bytes(a.as_slice()[72..80].try_into().unwrap());
                cpu.gpr[0] = lo;
                cpu.gpr[2] = hi;
            } else if name == "bts_m_neg" || name == "btc_m_neg" {
                cpu.gpr[2] = (-19i64) as u64;
            } else if name == "btr_m_pos" {
                cpu.gpr[1] = 73;
            } else if name == "rcl_32_m" {
                cpu.gpr[1] = 5;
            }
        });
    }
}

#[test]
fn fast_cmpj_fusion_and_run_chain_end_to_end() {
    // End-to-end guest function exercising fused `cmp`/`test` + `jcc` (RR, RI, RM, taken &
    // fall-through chains) plus a 200-iteration loop, verified against `TEST_FAST_OFF = true`.
    //
    // Args (Win64): rcx = n (200), rdx = ptr to u64 threshold (50)
    //   0x00: xor eax, eax              ; 31 C0
    //   0x02: test rcx, rcx             ; 48 85 C9  (K_CMPJ_RR, not taken)
    //   0x05: jz done (+0x1E -> 0x25)   ; 74 1E
    // loop (0x07):
    //   0x07: cmp rcx, [rdx]            ; 48 3B 0A  (K_CMPJ_RM)
    //   0x0A: jb below (+0x04 -> 0x10)  ; 72 04
    //   0x0C: add rax, 2                ; 48 83 C0 02
    // below (0x10):
    //   0x10: test cl, 1                ; F6 C1 01  (K_CMPJ_RI)
    //   0x13: jz even (+0x04 -> 0x19)   ; 74 04
    //   0x15: add rax, rcx              ; 48 01 C8
    // even (0x18):
    //   0x18: dec rcx                   ; 48 FF C9
    //   0x1B: cmp rcx, 0                ; 48 83 F9 00 (K_CMPJ_RI)
    //   0x1F: jne loop (-0x1A -> 0x07)  ; 75 E6
    // done (0x21):
    //   0x21: ret                       ; C3
    #[rustfmt::skip]
    let code: [u8; 34] = [
        0x31, 0xC0,
        0x48, 0x85, 0xC9,
        0x74, 0x1A,
        // 0x07: loop
        0x48, 0x3B, 0x0A,
        0x72, 0x04,
        0x48, 0x83, 0xC0, 0x02,
        // 0x10: below
        0xF6, 0xC1, 0x01,
        0x74, 0x03,
        0x48, 0x01, 0xC8,
        // 0x18: even
        0x48, 0xFF, 0xC9,
        0x48, 0x83, 0xF9, 0x00,
        0x75, 0xE6,
        // 0x21: done
        0xC3,
    ];
    let mut code_buf = AlignedBuf::new(64, 16);
    code_buf.as_mut_slice()[..code.len()].copy_from_slice(&code);
    let mut thresh = AlignedBuf::new(16, 16);
    thresh.as_mut_slice()[..8].copy_from_slice(&50u64.to_le_bytes());

    let run_mode = |fast_on: bool| -> u64 {
        let prev = crate::emu::TEST_FAST_OFF.with(|f| f.replace(!fast_on));
        crate::emu::jit::force(2);
        let (res, _) = unsafe {
            crate::emu::emu_run_full(code_buf.addr(), [200, thresh.addr(), 0, 0, 0, 0, 0, 0], [0; 4], [0; 8])
        };
        crate::emu::TEST_FAST_OFF.with(|f| f.set(prev));
        crate::emu::jit::force(0);
        res
    };
    let fast_res = run_mode(true);
    let gen_res = run_mode(false);
    // Expected: for k in 1..=200: (+2 if k >= 50 => 151*2 = 302) + (sum of odd k in 1..=200 => 100^2 = 10000) = 10302.
    assert_eq!(fast_res, 10302);
    assert_eq!(fast_res, gen_res);
}
