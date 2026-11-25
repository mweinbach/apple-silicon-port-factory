//! Comprehensive lowering, differential (Generic vs FastOp vs Live ARM64 JIT vs AOT),
//! precision/rounding/exception, MMX-aliasing, static-TOP, and static Mach-O `.aot.o` tests
//! for the lowered x87 FPU instruction family (`K_X87`).

use super::*;
use crate::emu::aot;
use crate::emu::fast;
use crate::emu::jit;
use crate::emu::jit_tests::arena_alloc;
use crate::emu::static_macho::{emit_macho_object_from_aot_bytes, MachoPlatform};
use crate::emu::x87::{x87_reset, x87_restore, x87_snapshot, X87State};
use crate::emu::{emu_run_full, instruction_lowering, BlockCache, Cpu, InstructionLowering, BLK_MAX_INSNS};
use iced_x86::code_asm::*;
use iced_x86::{Decoder, DecoderOptions, Mnemonic};

struct X87Guard(X87State);
impl X87Guard {
    fn new() -> Self {
        let g = Self(x87_snapshot());
        x87_reset();
        g
    }
}
impl Drop for X87Guard {
    fn drop(&mut self) {
        x87_restore(self.0);
    }
}

#[derive(Debug, Clone)]
struct RunResult {
    rax: u64,
    data: Vec<u8>,
    x87: X87State,
}

fn assert_x87_eq(label: &str, got: &X87State, want: &X87State) {
    assert_eq!(got.cw, want.cw, "{label}: CW mismatch");
    assert_eq!(
        got.status_word(),
        want.status_word(),
        "{label}: SW mismatch (got={:#06x} want={:#06x})",
        got.status_word(),
        want.status_word()
    );
    assert_eq!(got.top, want.top, "{label}: TOP mismatch");
    assert_eq!(got.valid_mask, want.valid_mask, "{label}: valid_mask mismatch");
    for p in 0..8usize {
        if (want.valid_mask & (1 << p)) != 0 {
            assert_eq!(
                got.regs[p][..10],
                want.regs[p][..10],
                "{label}: physical slot[{p}] mismatch"
            );
        }
    }
}

/// Execute assembled x86-64 function `code` (Win64 ABI: `rcx = data_buf`) across:
///   1. Generic `iced-x86` interpreter (`exec_generic_ins` step loop),
///   2. FastOp interpreter (`jit::force(2)`),
///   3. Live ARM64 JIT (`jit::force(1)`).
/// Asserts all three produce bit-for-bit identical `rax`, memory buffer, and `X87State`,
/// and returns the verified result.
fn run_diff_x87(label: &str, bytes: &[u8], init_data: &[u8]) -> RunResult {
    let code_cap = (bytes.len() + 64).max(256);
    let code = arena_alloc(code_cap, 16);
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), code, bytes.len());
        core::ptr::write_bytes(code.add(bytes.len()), 0xCC, 16);
    }
    let data_len = init_data.len().max(64);
    let d_buf = arena_alloc(data_len, 64);
    run_diff_x87_at(label, code, d_buf, data_len, init_data)
}

fn run_diff_x87_at(
    label: &str,
    code: *mut u8,
    d_buf: *mut u8,
    data_len: usize,
    init_data: &[u8],
) -> RunResult {
    let _guard = X87Guard::new();

    // 1. Generic interpreter (pure `exec_generic_ins` + `record_x87_environment`, matching `interp.rs`)
    let gen_res = unsafe {
        core::ptr::write_bytes(d_buf, 0, data_len);
        core::ptr::copy_nonoverlapping(init_data.as_ptr(), d_buf, init_data.len());
        x87_reset();
        let stack = arena_alloc(0x4000, 16);
        let rsp_val = stack as u64 + 0x3F00;
        let sentinel = 0xDEAD_BEEF_CAFE_0000u64;
        core::ptr::write_unaligned(rsp_val as *mut u64, sentinel);
        let mut cpu = Cpu::new_zeroed();
        cpu.rip = code as u64;
        cpu.gpr[1] = d_buf as u64; // rcx
        cpu.gpr[4] = rsp_val; // rsp
        let mut steps = 0usize;
        while cpu.rip != sentinel && steps < 20_000 {
            let ip = cpu.rip;
            let slice = core::slice::from_raw_parts(ip as *const u8, 15);
            let mut dec = Decoder::with_ip(64, slice, ip, DecoderOptions::NONE);
            let ins = dec.decode();
            if ins.mnemonic() == Mnemonic::Ret {
                cpu.rip = sentinel;
                break;
            }
            cpu.rip = ins.next_ip();
            if ins.is_jmp_short_or_near() {
                cpu.rip = ins.near_branch_target();
            } else if ins.is_jcc_short_or_near() {
                if cpu.eval_cc(ins.condition_code()) {
                    cpu.rip = ins.near_branch_target();
                }
            } else {
                assert!(
                    cpu.exec_generic_ins(&ins),
                    "{label}: generic interpreter failed on {ins:?}"
                );
                crate::emu::regfile_ext::record_x87_environment(&cpu, &ins);
            }
            steps += 1;
        }
        assert_eq!(cpu.rip, sentinel, "{label}: generic interpreter did not return cleanly");
        RunResult {
            rax: cpu.gpr[0],
            data: core::slice::from_raw_parts(d_buf, init_data.len()).to_vec(),
            x87: x87_snapshot(),
        }
    };

    // 2. FastOp interpreter (`jit::force(2)`)
    let fast_res = unsafe {
        core::ptr::write_bytes(d_buf, 0, data_len);
        core::ptr::copy_nonoverlapping(init_data.as_ptr(), d_buf, init_data.len());
        x87_reset();
        jit::force(2);
        let (ret_rax, _) = emu_run_full(code as u64, [d_buf as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        jit::force(0);
        RunResult {
            rax: ret_rax,
            data: core::slice::from_raw_parts(d_buf, init_data.len()).to_vec(),
            x87: x87_snapshot(),
        }
    };

    // 3. Live ARM64 JIT (`jit::force(1)`)
    let jit_before = jit::test_executed();
    let jit_res = unsafe {
        core::ptr::write_bytes(d_buf, 0, data_len);
        core::ptr::copy_nonoverlapping(init_data.as_ptr(), d_buf, init_data.len());
        x87_reset();
        jit::force(1);
        let (ret_rax, _) = emu_run_full(code as u64, [d_buf as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        jit::force(0);
        RunResult {
            rax: ret_rax,
            data: core::slice::from_raw_parts(d_buf, init_data.len()).to_vec(),
            x87: x87_snapshot(),
        }
    };
    assert!(
        jit::test_executed() > jit_before,
        "{label}: expected block to execute in ARM64 JIT"
    );

    assert_eq!(fast_res.rax, gen_res.rax, "{label}: FastOp vs Generic RAX");
    assert_eq!(fast_res.data, gen_res.data, "{label}: FastOp vs Generic memory");
    assert_x87_eq(&format!("{label} (FastOp vs Generic)"), &fast_res.x87, &gen_res.x87);

    assert_eq!(jit_res.rax, gen_res.rax, "{label}: JIT vs Generic RAX");
    assert_eq!(jit_res.data, gen_res.data, "{label}: JIT vs Generic memory");
    assert_x87_eq(&format!("{label} (JIT vs Generic)"), &jit_res.x87, &gen_res.x87);

    jit_res
}

fn assemble(f: impl FnOnce(&mut CodeAssembler)) -> Vec<u8> {
    let mut a = CodeAssembler::new(64).unwrap();
    f(&mut a);
    a.ret().unwrap();
    a.assemble(0x10_0000).unwrap()
}

/// 1. Every single mnemonic in `X87_FPU_STATE_ARITH` (92 mnemonics) plus the 5 x87 control ops
/// (`fninit`, `finit`, `fldcw`, `fnstcw`, `fstcw`) across ALL operand forms must lower to
/// `InstructionLowering::Arm64` (zero `K_GENERIC` / interpreter fallbacks).
#[test]
fn x87_every_mnemonic_and_operand_form_lowers_to_arm64() {
    let mut a = CodeAssembler::new(64).unwrap();
    // Control / state / nop
    a.fnop().unwrap();
    a.fninit().unwrap();
    a.finit().unwrap();
    a.fnclex().unwrap();
    a.fclex().unwrap();
    a.fldcw(word_ptr(rcx)).unwrap();
    a.fnstcw(word_ptr(rcx)).unwrap();
    a.fstcw(word_ptr(rcx)).unwrap();
    a.fnstsw(ax).unwrap();
    a.fnstsw(word_ptr(rcx)).unwrap();
    a.fstsw(ax).unwrap();
    a.fstsw(word_ptr(rcx)).unwrap();
    a.fdecstp().unwrap();
    a.fincstp().unwrap();
    a.ffree(st2).unwrap();
    a.ffreep(st3).unwrap();
    a.fxch(st0, st1).unwrap();
    a.fxch(st0, st4).unwrap();
    a.fabs().unwrap();
    a.fchs().unwrap();
    a.fxam().unwrap();

    // Loads (all operand forms: ST(i), m32fp, m64fp, m80fp, m16int, m32int, m64int, m80bcd, 7 constants)
    a.fld(st1).unwrap();
    a.fld(dword_ptr(rcx)).unwrap();
    a.fld(qword_ptr(rcx)).unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fild(word_ptr(rcx)).unwrap();
    a.fild(dword_ptr(rcx)).unwrap();
    a.fild(qword_ptr(rcx)).unwrap();
    a.fbld(tbyte_ptr(rcx)).unwrap();
    a.fldz().unwrap();
    a.fld1().unwrap();
    a.fldpi().unwrap();
    a.fldl2e().unwrap();
    a.fldl2t().unwrap();
    a.fldlg2().unwrap();
    a.fldln2().unwrap();

    // Stores (all operand forms: ST(i), m32fp, m64fp, m80fp, m16int, m32int, m64int, m80bcd)
    a.fst(st2).unwrap();
    a.fst(dword_ptr(rcx)).unwrap();
    a.fst(qword_ptr(rcx)).unwrap();
    a.fstp(st3).unwrap();
    a.fstp(dword_ptr(rcx)).unwrap();
    a.fstp(qword_ptr(rcx)).unwrap();
    a.fstp(tbyte_ptr(rcx)).unwrap();
    a.fist(word_ptr(rcx)).unwrap();
    a.fist(dword_ptr(rcx)).unwrap();
    a.fistp(word_ptr(rcx)).unwrap();
    a.fistp(dword_ptr(rcx)).unwrap();
    a.fistp(qword_ptr(rcx)).unwrap();
    a.fisttp(word_ptr(rcx)).unwrap();
    a.fisttp(dword_ptr(rcx)).unwrap();
    a.fisttp(qword_ptr(rcx)).unwrap();
    a.fbstp(tbyte_ptr(rcx)).unwrap();

    // Binary arithmetic (all forms: ST0/STi, STi/ST0, pop, m32fp, m64fp, m16int, m32int)
    a.fadd_2(st0, st1).unwrap();
    a.fadd_2(st2, st0).unwrap();
    a.fadd(dword_ptr(rcx)).unwrap();
    a.fadd(qword_ptr(rcx)).unwrap();
    a.faddp(st1, st0).unwrap();
    a.faddp(st3, st0).unwrap();
    a.fiadd(word_ptr(rcx)).unwrap();
    a.fiadd(dword_ptr(rcx)).unwrap();

    a.fsub_2(st0, st1).unwrap();
    a.fsub_2(st2, st0).unwrap();
    a.fsub(dword_ptr(rcx)).unwrap();
    a.fsub(qword_ptr(rcx)).unwrap();
    a.fsubp(st1, st0).unwrap();
    a.fsubp(st3, st0).unwrap();
    a.fisub(word_ptr(rcx)).unwrap();
    a.fisub(dword_ptr(rcx)).unwrap();

    a.fsubr_2(st0, st1).unwrap();
    a.fsubr_2(st2, st0).unwrap();
    a.fsubr(dword_ptr(rcx)).unwrap();
    a.fsubr(qword_ptr(rcx)).unwrap();
    a.fsubrp(st1, st0).unwrap();
    a.fsubrp(st3, st0).unwrap();
    a.fisubr(word_ptr(rcx)).unwrap();
    a.fisubr(dword_ptr(rcx)).unwrap();

    a.fmul_2(st0, st1).unwrap();
    a.fmul_2(st2, st0).unwrap();
    a.fmul(dword_ptr(rcx)).unwrap();
    a.fmul(qword_ptr(rcx)).unwrap();
    a.fmulp(st1, st0).unwrap();
    a.fmulp(st3, st0).unwrap();
    a.fimul(word_ptr(rcx)).unwrap();
    a.fimul(dword_ptr(rcx)).unwrap();

    a.fdiv_2(st0, st1).unwrap();
    a.fdiv_2(st2, st0).unwrap();
    a.fdiv(dword_ptr(rcx)).unwrap();
    a.fdiv(qword_ptr(rcx)).unwrap();
    a.fdivp(st1, st0).unwrap();
    a.fdivp(st3, st0).unwrap();
    a.fidiv(word_ptr(rcx)).unwrap();
    a.fidiv(dword_ptr(rcx)).unwrap();

    a.fdivr_2(st0, st1).unwrap();
    a.fdivr_2(st2, st0).unwrap();
    a.fdivr(dword_ptr(rcx)).unwrap();
    a.fdivr(qword_ptr(rcx)).unwrap();
    a.fdivrp(st1, st0).unwrap();
    a.fdivrp(st3, st0).unwrap();
    a.fidivr(word_ptr(rcx)).unwrap();
    a.fidivr(dword_ptr(rcx)).unwrap();

    // Comparisons & test
    a.fcom_2(st0, st1).unwrap();
    a.fcom_2(st0, st2).unwrap();
    a.fcom(dword_ptr(rcx)).unwrap();
    a.fcom(qword_ptr(rcx)).unwrap();
    a.fcomp_2(st0, st1).unwrap();
    a.fcomp_2(st0, st2).unwrap();
    a.fcomp(dword_ptr(rcx)).unwrap();
    a.fcomp(qword_ptr(rcx)).unwrap();
    a.fcompp().unwrap();
    a.ficom(word_ptr(rcx)).unwrap();
    a.ficom(dword_ptr(rcx)).unwrap();
    a.ficomp(word_ptr(rcx)).unwrap();
    a.ficomp(dword_ptr(rcx)).unwrap();
    a.fucom(st0, st1).unwrap();
    a.fucom(st0, st2).unwrap();
    a.fucomp(st0, st1).unwrap();
    a.fucomp(st0, st2).unwrap();
    a.fucompp().unwrap();
    a.fcomi(st0, st2).unwrap();
    a.fcomip(st0, st2).unwrap();
    a.fucomi(st0, st2).unwrap();
    a.fucomip(st0, st2).unwrap();
    a.ftst().unwrap();

    // Conditional moves
    a.fcmovb(st0, st1).unwrap();
    a.fcmove(st0, st1).unwrap();
    a.fcmovbe(st0, st1).unwrap();
    a.fcmovu(st0, st1).unwrap();
    a.fcmovnb(st0, st1).unwrap();
    a.fcmovne(st0, st1).unwrap();
    a.fcmovnbe(st0, st1).unwrap();
    a.fcmovnu(st0, st1).unwrap();

    // Unary / transcendentals / remainder / scale / extract
    a.fsqrt().unwrap();
    a.frndint().unwrap();
    a.fsin().unwrap();
    a.fcos().unwrap();
    a.fsincos().unwrap();
    a.fptan().unwrap();
    a.fpatan().unwrap();
    a.f2xm1().unwrap();
    a.fyl2x().unwrap();
    a.fyl2xp1().unwrap();
    a.fscale().unwrap();
    a.fxtract().unwrap();
    a.fprem().unwrap();
    a.fprem1().unwrap();

    // Environment / save / restore (both 32-bit 28B/108B and 16-bit 14B/94B forms)
    a.fldenv(ptr(rcx)).unwrap();
    a.fnstenv(ptr(rcx)).unwrap();
    a.fstenv(ptr(rcx)).unwrap();
    a.frstor(ptr(rcx)).unwrap();
    a.fnsave(ptr(rcx)).unwrap();
    a.fsave(ptr(rcx)).unwrap();

    let mut bytes = a.assemble(0x10_0000).unwrap();
    // Append raw encodings for rare/16-bit/undocumented mnemonics in the 92-mnemonic catalog:
    // fstpnce st(1) (DF D9), fneni (DB E0), fndisi (DB E1), fnsetpm (DB E4),
    // 16-bit fldenv/fnstenv/frstor/fnsave (66 prefix).
    bytes.extend_from_slice(&[
        0xDF, 0xD9, // fstpnce st(1)
        0xDB, 0xE0, // fneni
        0xDB, 0xE1, // fndisi
        0xDB, 0xE4, // fnsetpm
        0x66, 0xD9, 0x21, // fldenv [rcx] (14B)
        0x66, 0xD9, 0x31, // fnstenv [rcx] (14B)
        0x66, 0xDD, 0x21, // frstor [rcx] (94B)
        0x66, 0xDD, 0x31, // fnsave [rcx] (94B)
    ]);

    let mut dec = Decoder::with_ip(64, &bytes, 0x10_0000, DecoderOptions::NONE);
    let mut seen = std::collections::HashSet::new();
    let mut total_forms = 0usize;
    while dec.can_decode() {
        let ins = dec.decode();
        assert!(!ins.is_invalid(), "invalid instruction decoded");
        let lowering = instruction_lowering(&ins);
        assert_eq!(
            lowering,
            InstructionLowering::Arm64,
            "x87 instruction {:?} ({ins}) lowered to {lowering:?}, expected Arm64",
            ins.mnemonic()
        );
        let fop = fast::lower(&ins);
        assert_ne!(fop.kind, fast::K_GENERIC, "x87 instruction {ins} stayed K_GENERIC");
        seen.insert(ins.mnemonic());
        total_forms += 1;
    }

    // Also explicitly test `Fstpnce` via iced-x86 `Code::Fstpnce_sti` (and wait-prefixed forms
    // `Finit`/`Fclex`/`Fstcw`/`Fstsw`/`Fstenv`/`Fsave` which `CodeAssembler` emits as Wait + Fn*).
    for ins in [
        iced_x86::Instruction::with1(iced_x86::Code::Fstpnce_sti, iced_x86::Register::ST1).unwrap(),
    ] {
        assert_eq!(
            instruction_lowering(&ins),
            InstructionLowering::Arm64,
            "x87 instruction {:?} must lower to Arm64",
            ins.mnemonic()
        );
        assert_ne!(fast::lower(&ins).kind, fast::K_GENERIC);
        seen.insert(ins.mnemonic());
        total_forms += 1;
    }

    // Verify all 92 catalog mnemonics (wait-prefixed forms like finit/fclex/fstcw/fstsw/fstenv/fsave
    // decode as Wait + Fn* under DecoderOptions::NONE, so check the 86 base mnemonics + Wait).
    for mn in [
        Mnemonic::Fnop,
        Mnemonic::Fneni,
        Mnemonic::Fndisi,
        Mnemonic::Fnsetpm,
        Mnemonic::Fninit,
        Mnemonic::Fnclex,
        Mnemonic::Fldcw,
        Mnemonic::Fnstcw,
        Mnemonic::Fnstsw,
        Mnemonic::Fdecstp,
        Mnemonic::Fincstp,
        Mnemonic::Ffree,
        Mnemonic::Ffreep,
        Mnemonic::Fxch,
        Mnemonic::Fabs,
        Mnemonic::Fchs,
        Mnemonic::Fxam,
        Mnemonic::Fld,
        Mnemonic::Fild,
        Mnemonic::Fbld,
        Mnemonic::Fldz,
        Mnemonic::Fld1,
        Mnemonic::Fldpi,
        Mnemonic::Fldl2e,
        Mnemonic::Fldl2t,
        Mnemonic::Fldlg2,
        Mnemonic::Fldln2,
        Mnemonic::Fst,
        Mnemonic::Fstp,
        Mnemonic::Fstpnce,
        Mnemonic::Fist,
        Mnemonic::Fistp,
        Mnemonic::Fisttp,
        Mnemonic::Fbstp,
        Mnemonic::Fadd,
        Mnemonic::Faddp,
        Mnemonic::Fiadd,
        Mnemonic::Fsub,
        Mnemonic::Fsubp,
        Mnemonic::Fisub,
        Mnemonic::Fsubr,
        Mnemonic::Fsubrp,
        Mnemonic::Fisubr,
        Mnemonic::Fmul,
        Mnemonic::Fmulp,
        Mnemonic::Fimul,
        Mnemonic::Fdiv,
        Mnemonic::Fdivp,
        Mnemonic::Fidiv,
        Mnemonic::Fdivr,
        Mnemonic::Fdivrp,
        Mnemonic::Fidivr,
        Mnemonic::Fcom,
        Mnemonic::Fcomp,
        Mnemonic::Fcompp,
        Mnemonic::Ficom,
        Mnemonic::Ficomp,
        Mnemonic::Fucom,
        Mnemonic::Fucomp,
        Mnemonic::Fucompp,
        Mnemonic::Fcomi,
        Mnemonic::Fcomip,
        Mnemonic::Fucomi,
        Mnemonic::Fucomip,
        Mnemonic::Ftst,
        Mnemonic::Fcmovb,
        Mnemonic::Fcmove,
        Mnemonic::Fcmovbe,
        Mnemonic::Fcmovu,
        Mnemonic::Fcmovnb,
        Mnemonic::Fcmovne,
        Mnemonic::Fcmovnbe,
        Mnemonic::Fcmovnu,
        Mnemonic::Fsqrt,
        Mnemonic::Frndint,
        Mnemonic::Fsin,
        Mnemonic::Fcos,
        Mnemonic::Fsincos,
        Mnemonic::Fptan,
        Mnemonic::Fpatan,
        Mnemonic::F2xm1,
        Mnemonic::Fyl2x,
        Mnemonic::Fyl2xp1,
        Mnemonic::Fscale,
        Mnemonic::Fxtract,
        Mnemonic::Fprem,
        Mnemonic::Fprem1,
        Mnemonic::Fldenv,
        Mnemonic::Fnstenv,
        Mnemonic::Frstor,
        Mnemonic::Fnsave,
    ] {
        assert!(seen.contains(&mn), "missing mnemonic {mn:?} in lowering audit");
    }
    assert!(total_forms >= 150, "expected >= 150 operand forms, got {total_forms}");
}

/// 2. Differential tests for loads, stores, integer loads, constants across all 4 RC modes,
/// stack management, overflow/underflow, and control/status ops.
#[test]
fn x87_differential_loads_stores_constants_and_stack_faults() {
    let mut init = vec![0u8; 512];
    // [0..4]: f32 = -13.75f32
    init[0..4].copy_from_slice(&(-13.75f32).to_bits().to_le_bytes());
    // [8..16]: f64 = 123456.789012345
    init[8..16].copy_from_slice(&123456.789012345f64.to_bits().to_le_bytes());
    // [16..26]: f80 = exact 1.0 + 2^-60 (sig = 0x8000_0000_0000_0008, se = 0x3FFF)
    init[16..24].copy_from_slice(&0x8000_0000_0000_0008u64.to_le_bytes());
    init[24..26].copy_from_slice(&0x3FFFu16.to_le_bytes());
    // [32..34]: i16::MIN
    init[32..34].copy_from_slice(&i16::MIN.to_le_bytes());
    // [36..40]: i32::MIN
    init[36..40].copy_from_slice(&i32::MIN.to_le_bytes());
    // [40..48]: i64::MIN
    init[40..48].copy_from_slice(&i64::MIN.to_le_bytes());
    // [48..56]: i64::MAX
    init[48..56].copy_from_slice(&i64::MAX.to_le_bytes());
    // [56..64]: f64 subnormal = 5e-324 (bits = 1)
    init[56..64].copy_from_slice(&1u64.to_le_bytes());
    // [64..68]: f32 sNaN = 0x7FA0_1234
    init[64..68].copy_from_slice(&0x7FA0_1234u32.to_le_bytes());
    // [72..74]: CW for each RC (RN=0x037F, RM=0x077F, RP=0x0B7F, RZ=0x0F7F)
    init[72..74].copy_from_slice(&0x037Fu16.to_le_bytes());
    init[74..76].copy_from_slice(&0x077Fu16.to_le_bytes());
    init[76..78].copy_from_slice(&0x0B7Fu16.to_le_bytes());
    init[78..80].copy_from_slice(&0x0F7Fu16.to_le_bytes());

    let code = assemble(|a| {
        a.fninit().unwrap();
        a.fld(dword_ptr(rcx)).unwrap();
        a.fabs().unwrap();
        a.fchs().unwrap();
        a.fst(dword_ptr(rcx + 128)).unwrap();
        a.fstp(qword_ptr(rcx + 136)).unwrap();

        a.fld(qword_ptr(rcx + 8)).unwrap();
        a.fld(st0).unwrap();
        a.fxch(st0, st1).unwrap();
        a.fst(st1).unwrap();
        a.fstp(st0).unwrap();
        a.fstp(tbyte_ptr(rcx + 144)).unwrap();

        // f80 lossless round-trip + fild all 3 widths (incl. MIN/MAX)
        a.fld(tbyte_ptr(rcx + 16)).unwrap();
        a.fstp(tbyte_ptr(rcx + 160)).unwrap();
        a.fild(word_ptr(rcx + 32)).unwrap();
        a.fstp(qword_ptr(rcx + 176)).unwrap();
        a.fild(dword_ptr(rcx + 36)).unwrap();
        a.fstp(qword_ptr(rcx + 184)).unwrap();
        a.fild(qword_ptr(rcx + 40)).unwrap();
        a.fstp(tbyte_ptr(rcx + 192)).unwrap();
        a.fild(qword_ptr(rcx + 48)).unwrap();
        a.fstp(tbyte_ptr(rcx + 208)).unwrap();

        // Subnormal + sNaN loads set DE/IE properly
        a.fld(qword_ptr(rcx + 56)).unwrap();
        a.fnstsw(word_ptr(rcx + 224)).unwrap();
        a.fnclex().unwrap();
        a.fld(dword_ptr(rcx + 64)).unwrap();
        a.fnstsw(ax).unwrap();
        a.mov(word_ptr(rcx + 226), ax).unwrap();
        a.ffree(st1).unwrap();
        a.ffreep(st0).unwrap();

        // fdecstp / fincstp
        a.fdecstp().unwrap();
        a.fnstsw(word_ptr(rcx + 228)).unwrap();
        a.fincstp().unwrap();
        a.fnstsw(word_ptr(rcx + 230)).unwrap();
    });
    let r = run_diff_x87("loads_stores_and_stack", &code, &init);
    assert_eq!(
        f32::from_bits(u32::from_le_bytes(r.data[128..132].try_into().unwrap())),
        -13.75f32
    );
    assert_eq!(&r.data[160..170], &init[16..26], "f80 round-trip must be exact");
    assert_eq!(
        f64::from_bits(u64::from_le_bytes(r.data[176..184].try_into().unwrap())),
        i16::MIN as f64
    );
    assert_eq!(
        f64::from_bits(u64::from_le_bytes(r.data[184..192].try_into().unwrap())),
        i32::MIN as f64
    );
    // DE flag set on subnormal load
    let sw_de = u16::from_le_bytes(r.data[224..226].try_into().unwrap());
    assert_ne!(sw_de & 0x0002, 0, "subnormal fld must set SW.DE");
    // IE flag set on sNaN load
    let sw_ie = u16::from_le_bytes(r.data[226..228].try_into().unwrap());
    assert_ne!(sw_ie & 0x0001, 0, "sNaN fld must set SW.IE");

    // All 7 constants across all 4 rounding modes
    for rc_i in 0..4usize {
        let code = assemble(|a| {
            a.fninit().unwrap();
            a.fldcw(word_ptr(rcx + 72 + (rc_i * 2) as i32)).unwrap();
            a.fnstcw(word_ptr(rcx + 240)).unwrap();
            a.fldz().unwrap();
            a.fstp(tbyte_ptr(rcx + 250)).unwrap();
            a.fld1().unwrap();
            a.fstp(tbyte_ptr(rcx + 260)).unwrap();
            a.fldpi().unwrap();
            a.fstp(tbyte_ptr(rcx + 270)).unwrap();
            a.fldl2e().unwrap();
            a.fstp(tbyte_ptr(rcx + 280)).unwrap();
            a.fldl2t().unwrap();
            a.fstp(tbyte_ptr(rcx + 290)).unwrap();
            a.fldlg2().unwrap();
            a.fstp(tbyte_ptr(rcx + 300)).unwrap();
            a.fldln2().unwrap();
            a.fstp(tbyte_ptr(rcx + 310)).unwrap();
        });
        run_diff_x87(&format!("constants_rc_{rc_i}"), &code, &init);
    }

    // Stack overflow (9 pushes) and stack underflow (pop/fabs/fchs/fxch on empty)
    let ov_code = assemble(|a| {
        a.fninit().unwrap();
        for _ in 0..9 {
            a.fld1().unwrap();
        }
        a.fnstsw(word_ptr(rcx + 320)).unwrap();
        a.fstp(tbyte_ptr(rcx + 322)).unwrap();
    });
    let ov = run_diff_x87("stack_overflow", &ov_code, &init);
    let sw_ov = u16::from_le_bytes(ov.data[320..322].try_into().unwrap());
    assert_eq!(sw_ov & 0x0241, 0x0241, "stack overflow must set IE|SF|C1");

    let uf_code = assemble(|a| {
        a.fninit().unwrap();
        a.fstp(qword_ptr(rcx + 336)).unwrap();
        a.fnstsw(word_ptr(rcx + 344)).unwrap();
        a.fninit().unwrap();
        a.fabs().unwrap();
        a.fnstsw(word_ptr(rcx + 346)).unwrap();
        a.fninit().unwrap();
        a.fchs().unwrap();
        a.fnstsw(word_ptr(rcx + 348)).unwrap();
        a.fninit().unwrap();
        a.fxch(st0, st2).unwrap();
        a.fnstsw(word_ptr(rcx + 350)).unwrap();
    });
    let uf = run_diff_x87("stack_underflow", &uf_code, &init);
    for off in [344, 346, 348, 350] {
        let sw = u16::from_le_bytes(uf.data[off..off + 2].try_into().unwrap());
        assert_eq!(sw & 0x0041, 0x0041, "underflow at +{off} must set IE|SF");
    }
}

/// 3. Differential tests for `fist`/`fistp`/`fisttp` across all widths (`m16`/`m32`/`m64`),
/// all 4 rounding modes (`RN`/`RM`/`RP`/`RZ`), >53-bit exact `i64`, `i64::MIN`, and overflow indefinite.
#[test]
fn x87_differential_fist_rounding_modes_and_overflow() {
    let vals: [f64; 14] = [
        0.0,
        -0.0,
        2.5,
        -2.5,
        3.5,
        -3.5,
        1.25,
        -1.25,
        32767.0,
        -32768.0,
        32768.0, // i16 overflow!
        2147483648.0, // i32 overflow!
        -9223372036854775808.0, // exact i64::MIN
        9223372036854775808.0, // +2^63 -> i64 overflow!
    ];
    let cws: [u16; 4] = [0x027F, 0x067F, 0x0A7F, 0x0E7F];
    for (rc_idx, &cw) in cws.iter().enumerate() {
        let mut init = vec![0u8; 512];
        init[0..2].copy_from_slice(&cw.to_le_bytes());
        for (i, &v) in vals.iter().enumerate() {
            init[8 + i * 8..16 + i * 8].copy_from_slice(&v.to_bits().to_le_bytes());
        }
        // Exact >53-bit i64 value at +128
        init[128..136].copy_from_slice(&0x1234_5678_9ABC_DEF1i64.to_le_bytes());

        let code = assemble(|a| {
            a.fninit().unwrap();
            a.fldcw(word_ptr(rcx)).unwrap();
            let mut out = 160i32;
            for i in 0..vals.len() {
                a.fld(qword_ptr(rcx + 8 + (i * 8) as i32)).unwrap();
                a.fist(word_ptr(rcx + out)).unwrap();
                a.fist(dword_ptr(rcx + out + 2)).unwrap();
                a.fistp(qword_ptr(rcx + out + 8)).unwrap();
                a.fld(qword_ptr(rcx + 8 + (i * 8) as i32)).unwrap();
                a.fisttp(qword_ptr(rcx + out + 16)).unwrap();
                out += 24;
            }
            // Exact >53-bit i64 round-trip via fild + fistp (must NOT lose low 11 bits!)
            a.fild(qword_ptr(rcx + 128)).unwrap();
            a.fistp(qword_ptr(rcx + 496)).unwrap();
        });
        let r = run_diff_x87(&format!("fist_rc_{rc_idx}"), &code, &init);
        let rt64 = i64::from_le_bytes(r.data[496..504].try_into().unwrap());
        assert_eq!(rt64, 0x1234_5678_9ABC_DEF1i64, "fistp m64 must preserve all 64 bits");
    }
}

/// 4. Differential tests for all 18 binary arithmetic ops, `fsqrt`, `frndint`,
/// `PC=24` vs `PC=53` vs `PC=64`, huge/tiny 15-bit exponents, and `MXCSR FTZ|DAZ` isolation.
#[test]
fn x87_differential_arith_precision_modes_and_mxcsr_isolation() {
    let mut init = vec![0u8; 512];
    init[0..8].copy_from_slice(&1.0f64.to_bits().to_le_bytes());
    init[8..16].copy_from_slice(&(2.0f64.powi(-60)).to_bits().to_le_bytes());
    init[16..24].copy_from_slice(&(2.0f64.powi(-30)).to_bits().to_le_bytes());
    init[24..32].copy_from_slice(&1e-200f64.to_bits().to_le_bytes());
    init[32..40].copy_from_slice(&1e200f64.to_bits().to_le_bytes());
    init[40..44].copy_from_slice(&3.5f32.to_bits().to_le_bytes());
    init[44..48].copy_from_slice(&(-7i32).to_le_bytes());
    init[48..50].copy_from_slice(&(-11i16).to_le_bytes());
    // CWs: PC=24 (0x007F), PC=53 (0x027F), PC=64 (0x037F)
    init[56..58].copy_from_slice(&0x007Fu16.to_le_bytes());
    init[58..60].copy_from_slice(&0x027Fu16.to_le_bytes());
    init[60..62].copy_from_slice(&0x037Fu16.to_le_bytes());
    // Hostile MXCSR: FTZ=1 | DAZ=1 | RC=RM (0xBF80 | 0x40 = 0xBFC0)
    init[64..68].copy_from_slice(&0xBFC0u32.to_le_bytes());
    // Default MXCSR (0x1F80)
    init[68..72].copy_from_slice(&0x1F80u32.to_le_bytes());
    // 2.0f64, 9.0f64, -1.0f64, -0.0f64
    init[72..80].copy_from_slice(&2.0f64.to_bits().to_le_bytes());
    init[80..88].copy_from_slice(&9.0f64.to_bits().to_le_bytes());
    init[88..96].copy_from_slice(&(-1.0f64).to_bits().to_le_bytes());
    init[96..104].copy_from_slice(&(-0.0f64).to_bits().to_le_bytes());

    // PC=24 vs PC=53 vs PC=64 exact precision proof
    let pc_code = assemble(|a| {
        a.ldmxcsr(dword_ptr(rcx + 64)).unwrap(); // Enable hostile MXCSR FTZ|DAZ|RM!
        for (i, cw_off) in [56i32, 58, 60].into_iter().enumerate() {
            a.fninit().unwrap();
            a.fldcw(word_ptr(rcx + cw_off)).unwrap();
            // 1.0 + 2^-60
            a.fld(qword_ptr(rcx)).unwrap();
            a.fadd(qword_ptr(rcx + 8)).unwrap();
            a.fstp(tbyte_ptr(rcx + 128 + (i * 20) as i32)).unwrap();
            // 1.0 + 2^-30
            a.fld(qword_ptr(rcx)).unwrap();
            a.fadd(qword_ptr(rcx + 16)).unwrap();
            a.fstp(tbyte_ptr(rcx + 138 + (i * 20) as i32)).unwrap();
        }
        // 15-bit exponent preservation under PC=53: 1e-200 * 1e-200 and 1e200 * 1e200!
        a.fninit().unwrap();
        a.fldcw(word_ptr(rcx + 58)).unwrap();
        a.fld(qword_ptr(rcx + 24)).unwrap();
        a.fmul(qword_ptr(rcx + 24)).unwrap();
        a.fstp(tbyte_ptr(rcx + 200)).unwrap();
        a.fld(qword_ptr(rcx + 32)).unwrap();
        a.fmul(qword_ptr(rcx + 32)).unwrap();
        a.fstp(tbyte_ptr(rcx + 210)).unwrap();
        a.ldmxcsr(dword_ptr(rcx + 68)).unwrap();
    });
    let r = run_diff_x87("pc24_pc53_pc64_and_mxcsr", &pc_code, &init);
    // Under PC=53, 1.0 + 2^-60 == 1.0 (sig = 0x8000_0000_0000_0000)
    let sig_53 = u64::from_le_bytes(r.data[148..156].try_into().unwrap());
    assert_eq!(sig_53, 0x8000_0000_0000_0000, "PC=53 must round 1+2^-60 to 1.0");
    // Under PC=64, 1.0 + 2^-60 keeps bit 3 (0x8000_0000_0000_0008)!
    let sig_64 = u64::from_le_bytes(r.data[168..176].try_into().unwrap());
    assert_eq!(sig_64, 0x8000_0000_0000_0008, "PC=64 must preserve 1+2^-60");
    // Under PC=24, 1.0 + 2^-30 rounds to 1.0; under PC=53 it preserves bit 33!
    let sig_24_30 = u64::from_le_bytes(r.data[138..146].try_into().unwrap());
    let sig_53_30 = u64::from_le_bytes(r.data[158..166].try_into().unwrap());
    assert_eq!(sig_24_30, 0x8000_0000_0000_0000, "PC=24 must round 1+2^-30 to 1.0");
    assert_eq!(sig_53_30, 0x8000_0002_0000_0000, "PC=53 must preserve 1+2^-30");
    // 1e-200 * 1e-200 is still non-zero normal in 80-bit!
    let exp_tiny = u16::from_le_bytes(r.data[208..210].try_into().unwrap()) & 0x7FFF;
    assert!(exp_tiny > 0 && exp_tiny < 16383, "1e-400 must stay normal in 80-bit");
    // 1e200 * 1e200 is still finite normal (not Inf!) in 80-bit!
    let exp_huge = u16::from_le_bytes(r.data[218..220].try_into().unwrap()) & 0x7FFF;
    assert!(exp_huge > 16383 && exp_huge < 0x7FFF, "1e+400 must stay finite in 80-bit");

    // All 18 binary arithmetic ops + fsqrt + frndint + exceptions (ZE, IE)
    let arith_code = assemble(|a| {
        a.fninit().unwrap();
        a.fldcw(word_ptr(rcx + 58)).unwrap(); // PC=53
        a.fld(qword_ptr(rcx + 80)).unwrap(); // 9.0
        a.fsqrt().unwrap(); // 3.0
        a.fiadd(word_ptr(rcx + 48)).unwrap(); // -8.0
        a.fisub(dword_ptr(rcx + 44)).unwrap(); // -1.0
        a.fisubr(word_ptr(rcx + 48)).unwrap(); // -10.0
        a.fimul(dword_ptr(rcx + 44)).unwrap(); // 70.0
        a.fidiv(dword_ptr(rcx + 44)).unwrap(); // -10.0
        a.fidivr(word_ptr(rcx + 48)).unwrap(); // 1.1
        a.fadd(dword_ptr(rcx + 40)).unwrap(); // 4.6
        a.fsub(dword_ptr(rcx + 40)).unwrap(); // 1.1
        a.fsubr(qword_ptr(rcx + 72)).unwrap(); // 0.9
        a.fmul(qword_ptr(rcx + 72)).unwrap(); // 1.8
        a.fdiv(dword_ptr(rcx + 40)).unwrap();
        a.fdivr(qword_ptr(rcx + 80)).unwrap();
        a.frndint().unwrap();
        // Register-register + pop forms
        a.fld(qword_ptr(rcx + 72)).unwrap();
        a.fadd_2(st0, st1).unwrap();
        a.fsub_2(st1, st0).unwrap();
        a.fsubr_2(st0, st1).unwrap();
        a.fmul_2(st1, st0).unwrap();
        a.fdiv_2(st0, st1).unwrap();
        a.fdivr_2(st1, st0).unwrap();
        a.faddp(st1, st0).unwrap();
        a.fld(qword_ptr(rcx + 72)).unwrap();
        a.fsubp(st1, st0).unwrap();
        a.fld(qword_ptr(rcx + 72)).unwrap();
        a.fsubrp(st1, st0).unwrap();
        a.fld(qword_ptr(rcx + 72)).unwrap();
        a.fmulp(st1, st0).unwrap();
        a.fld(qword_ptr(rcx + 72)).unwrap();
        a.fdivp(st1, st0).unwrap();
        a.fld(qword_ptr(rcx + 72)).unwrap();
        a.fdivrp(st1, st0).unwrap();
        a.fstp(qword_ptr(rcx + 240)).unwrap();
        // fsqrt(-0.0) == -0.0, fsqrt(-1.0) -> IE + indef, 1.0/0.0 -> ZE + Inf
        a.fld(qword_ptr(rcx + 96)).unwrap();
        a.fsqrt().unwrap();
        a.fstp(qword_ptr(rcx + 248)).unwrap();
        a.fld(qword_ptr(rcx + 88)).unwrap();
        a.fsqrt().unwrap();
        a.fstp(tbyte_ptr(rcx + 256)).unwrap();
        a.fld1().unwrap();
        a.fldz().unwrap();
        a.fdivp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 272)).unwrap();
        a.fstp(qword_ptr(rcx + 280)).unwrap();
    });
    let ar = run_diff_x87("all_18_binops_and_sqrt_rndint", &arith_code, &init);
    assert_eq!(
        u64::from_le_bytes(ar.data[248..256].try_into().unwrap()),
        (-0.0f64).to_bits(),
        "fsqrt(-0.0) must preserve -0.0"
    );
    let sw_ex = u16::from_le_bytes(ar.data[272..274].try_into().unwrap());
    assert_eq!(sw_ex & 0x0005, 0x0005, "must accumulate IE (sqrt(-1)) and ZE (1/0)");
}

/// 5. Differential tests for all comparison instructions (`fcom*`, `ficom*`, `fucom*`,
/// `fcomi*`, `fucomi*`, `ftst`), `fxam` across all classes, and all 8 `fcmov*` conditions.
#[test]
fn x87_differential_cmp_ftst_fxam_and_fcmov() {
    let mut init = vec![0u8; 512];
    init[0..8].copy_from_slice(&2.0f64.to_bits().to_le_bytes());
    init[8..16].copy_from_slice(&5.0f64.to_bits().to_le_bytes());
    init[16..24].copy_from_slice(&(-0.0f64).to_bits().to_le_bytes());
    init[24..32].copy_from_slice(&f64::NAN.to_bits().to_le_bytes()); // qNaN
    init[32..40].copy_from_slice(&0x7FF0_0000_0000_0001u64.to_le_bytes()); // sNaN
    init[40..42].copy_from_slice(&5i16.to_le_bytes());
    init[44..48].copy_from_slice(&2i32.to_le_bytes());
    init[48..52].copy_from_slice(&2.0f32.to_bits().to_le_bytes());
    // fxam raw 80-bit inputs (starting at +64, stride 10)
    let fxam_cases: [[u8; 10]; 11] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],                   // +0
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80],                // -0
        [0, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0x3F],          // +1.0 (normal)
        [0, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0xBF],          // -1.0 (normal)
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0],                   // +denormal
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0x80],                // -denormal
        [0, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0x7F],          // +Inf
        [0, 0, 0, 0, 0, 0, 0, 0x80, 0xFF, 0xFF],          // -Inf
        [0, 0, 0, 0, 0, 0, 0, 0xC0, 0xFF, 0x7F],          // +qNaN
        [0, 0, 0, 0, 0, 0, 0, 0xC0, 0xFF, 0xFF],          // -qNaN
        [1, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0x3F],             // unsupported unnormal
    ];
    for (i, c) in fxam_cases.iter().enumerate() {
        init[64 + i * 10..74 + i * 10].copy_from_slice(c);
    }

    let code = assemble(|a| {
        a.fninit().unwrap();
        // fxam empty first
        a.fxam().unwrap();
        a.fnstsw(word_ptr(rcx + 200)).unwrap();
        for i in 0..fxam_cases.len() {
            a.fld(tbyte_ptr(rcx + 64 + (i * 10) as i32)).unwrap();
            a.fxam().unwrap();
            a.fnstsw(word_ptr(rcx + 202 + (i * 2) as i32)).unwrap();
            a.fstp(st0).unwrap();
        }
        // fcom / fcomp / fcompp / ficom / ficomp / fucom / fucomp / fucompp / ftst
        a.fld(qword_ptr(rcx + 8)).unwrap(); // 5.0
        a.fld(qword_ptr(rcx)).unwrap(); // 2.0
        a.fcom_2(st0, st1).unwrap(); // 2 < 5
        a.fnstsw(word_ptr(rcx + 240)).unwrap();
        a.ficom(word_ptr(rcx + 40)).unwrap(); // 2 < 5
        a.fnstsw(word_ptr(rcx + 242)).unwrap();
        a.ficomp(dword_ptr(rcx + 44)).unwrap(); // 2 == 2, pop
        a.fnstsw(word_ptr(rcx + 244)).unwrap();
        a.fcomp(dword_ptr(rcx + 48)).unwrap(); // 5 > 2, pop
        a.fnstsw(word_ptr(rcx + 246)).unwrap();

        // qNaN: fucom must NOT set IE, fcom MUST set IE
        a.fninit().unwrap();
        a.fld(qword_ptr(rcx)).unwrap();
        a.fld(qword_ptr(rcx + 24)).unwrap(); // qNaN
        a.fucom(st0, st1).unwrap();
        a.fnstsw(word_ptr(rcx + 248)).unwrap();
        a.fcompp().unwrap();
        a.fnstsw(word_ptr(rcx + 250)).unwrap();

        // fcomi / fucomi / fcomip / fucomip (GT, LT, EQ, UNORD) -> capture RFLAGS via pushfq/pop
        let mut foff = 256i32;
        for (a_off, b_off) in [(8i32, 0i32), (0, 8), (0, 0), (0, 24), (0, 32)] {
            a.fninit().unwrap();
            a.fld(qword_ptr(rcx + b_off)).unwrap();
            a.fld(qword_ptr(rcx + a_off)).unwrap();
            a.fucomi(st0, st1).unwrap();
            a.pushfq().unwrap();
            a.pop(rax).unwrap();
            a.and(rax, 0x8D5).unwrap();
            a.mov(qword_ptr(rcx + foff), rax).unwrap();
            a.fcomip(st0, st1).unwrap();
            a.pushfq().unwrap();
            a.pop(rax).unwrap();
            a.and(rax, 0x8D5).unwrap();
            a.mov(qword_ptr(rcx + foff + 8), rax).unwrap();
            a.fstp(st0).unwrap();
            foff += 16;
        }
        // ftst on -0.0
        a.fninit().unwrap();
        a.fld(qword_ptr(rcx + 16)).unwrap();
        a.ftst().unwrap();
        a.fnstsw(word_ptr(rcx + 340)).unwrap();
        a.fstp(st0).unwrap();

        // All 8 fcmov* with flags set via fucomi (LT -> CF=1,ZF=0,PF=0; EQ -> CF=0,ZF=1,PF=0;
        // UNORD -> CF=1,ZF=1,PF=1; GT -> CF=0,ZF=0,PF=0)
        let mut cmov_off = 352i32;
        for (a_off, b_off) in [(0i32, 8i32), (0, 0), (0, 24), (8, 0)] {
            a.fninit().unwrap();
            a.fld(qword_ptr(rcx + b_off)).unwrap();
            a.fld(qword_ptr(rcx + a_off)).unwrap();
            a.fucomi(st0, st1).unwrap();
            // Test all 8 fcmov* using a fresh pair (10.0, 20.0) without clobbering EFLAGS
            a.fld(qword_ptr(rcx + 8)).unwrap(); // st1=5.0
            a.fld(qword_ptr(rcx)).unwrap(); // st0=2.0
            a.fcmovb(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off)).unwrap();
            a.fld(qword_ptr(rcx)).unwrap();
            a.fcmove(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off + 8)).unwrap();
            a.fld(qword_ptr(rcx)).unwrap();
            a.fcmovbe(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off + 16)).unwrap();
            a.fld(qword_ptr(rcx)).unwrap();
            a.fcmovu(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off + 24)).unwrap();
            a.fld(qword_ptr(rcx)).unwrap();
            a.fcmovnb(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off + 32)).unwrap();
            a.fld(qword_ptr(rcx)).unwrap();
            a.fcmovne(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off + 40)).unwrap();
            a.fld(qword_ptr(rcx)).unwrap();
            a.fcmovnbe(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off + 48)).unwrap();
            a.fld(qword_ptr(rcx)).unwrap();
            a.fcmovnu(st0, st1).unwrap();
            a.fstp(qword_ptr(rcx + cmov_off + 56)).unwrap();
            cmov_off += 64;
        }
    });
    let r = run_diff_x87("cmp_ftst_fxam_fcmov", &code, &init);
    // Verify fucom qNaN did NOT set IE, fcompp qNaN DID set IE
    let sw_fucom = u16::from_le_bytes(r.data[248..250].try_into().unwrap());
    let sw_fcom = u16::from_le_bytes(r.data[250..252].try_into().unwrap());
    assert_eq!(sw_fucom & 0x0001, 0, "fucom qNaN must not raise IE");
    assert_eq!(sw_fucom & 0x4500, 0x4500, "fucom qNaN must set C0|C2|C3");
    assert_ne!(sw_fcom & 0x0001, 0, "fcompp qNaN must raise IE");
    // Verify fxam empty = C3|C0 (0x4100)
    let sw_empty = u16::from_le_bytes(r.data[200..202].try_into().unwrap());
    assert_eq!(sw_empty & 0x4700, 0x4100, "fxam empty must set C3|C0");
}

/// 6. Differential tests + known-answer checks for all transcendentals, `fprem`/`fprem1`,
/// `fscale`, `fxtract`, `fbld`/`fbstp` BCD, `fldenv`/`fnstenv`/`fnsave`/`frstor`, and MMX/`emms`.
#[test]
fn x87_differential_transcendentals_bcd_env_and_mmx_aliasing() {
    let mut init = vec![0u8; 768];
    init[0..8].copy_from_slice(&(core::f64::consts::FRAC_PI_6).to_bits().to_le_bytes()); // pi/6
    init[8..16].copy_from_slice(&(core::f64::consts::FRAC_PI_3).to_bits().to_le_bytes()); // pi/3
    init[16..24].copy_from_slice(&(core::f64::consts::FRAC_PI_4).to_bits().to_le_bytes()); // pi/4
    init[24..32].copy_from_slice(&0.5f64.to_bits().to_le_bytes());
    init[32..40].copy_from_slice(&3.0f64.to_bits().to_le_bytes());
    init[40..48].copy_from_slice(&8.0f64.to_bits().to_le_bytes());
    init[48..56].copy_from_slice(&6.0f64.to_bits().to_le_bytes());
    init[56..64].copy_from_slice(&10.0f64.to_bits().to_le_bytes());
    init[64..72].copy_from_slice(&5.0f64.to_bits().to_le_bytes());
    init[72..80].copy_from_slice(&0.75f64.to_bits().to_le_bytes()); // fprem1 d==-1 case!
    init[80..88].copy_from_slice(&(2.0f64.powi(64)).to_bits().to_le_bytes()); // out-of-range trig!
    // BCD +123456789012345678 at +96 (little-endian packed nibbles)
    init[96..106].copy_from_slice(&[0x78, 0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x00]);
    // BCD -987654321098765432 at +112
    init[112..122].copy_from_slice(&[0x32, 0x54, 0x76, 0x98, 0x10, 0x32, 0x54, 0x76, 0x98, 0x80]);
    // MMX data at +128
    init[128..136].copy_from_slice(&0x0001_0002_0003_0004u64.to_le_bytes());
    init[136..144].copy_from_slice(&0x0005_0006_0007_0008u64.to_le_bytes());

    let code = assemble(|a| {
        a.fninit().unwrap();
        // fsin(pi/6) = 0.5
        a.fld(qword_ptr(rcx)).unwrap();
        a.fsin().unwrap();
        a.fstp(qword_ptr(rcx + 160)).unwrap();
        // fcos(pi/3) = 0.5
        a.fld(qword_ptr(rcx + 8)).unwrap();
        a.fcos().unwrap();
        a.fstp(qword_ptr(rcx + 168)).unwrap();
        // fsincos(pi/4) -> ST(0)=cos, ST(1)=sin
        a.fld(qword_ptr(rcx + 16)).unwrap();
        a.fsincos().unwrap();
        a.fstp(qword_ptr(rcx + 176)).unwrap();
        a.fstp(qword_ptr(rcx + 184)).unwrap();
        // fptan(pi/4) -> ST(0)=1.0, ST(1)=tan(pi/4)=1.0
        a.fld(qword_ptr(rcx + 16)).unwrap();
        a.fptan().unwrap();
        a.fstp(qword_ptr(rcx + 192)).unwrap();
        a.fstp(qword_ptr(rcx + 200)).unwrap();
        // fpatan(1, 1) = pi/4
        a.fld1().unwrap();
        a.fld1().unwrap();
        a.fpatan().unwrap();
        a.fstp(qword_ptr(rcx + 208)).unwrap();
        // f2xm1(0.5) = sqrt(2) - 1
        a.fld(qword_ptr(rcx + 24)).unwrap();
        a.f2xm1().unwrap();
        a.fstp(qword_ptr(rcx + 216)).unwrap();
        // fyl2x: 3 * log2(8) = 9
        a.fld(qword_ptr(rcx + 32)).unwrap();
        a.fld(qword_ptr(rcx + 40)).unwrap();
        a.fyl2x().unwrap();
        a.fstp(qword_ptr(rcx + 224)).unwrap();
        // fyl2xp1: 3 * log2(1 + 0.5)
        a.fld(qword_ptr(rcx + 32)).unwrap();
        a.fld(qword_ptr(rcx + 24)).unwrap();
        a.fyl2xp1().unwrap();
        a.fstp(qword_ptr(rcx + 232)).unwrap();
        // fscale: 1.5 * 2^3 = 12.0 (ST(1)=3.0, ST(0)=0.5 + 1.0)
        a.fld(qword_ptr(rcx + 32)).unwrap();
        a.fld(qword_ptr(rcx + 24)).unwrap();
        a.fld1().unwrap();
        a.faddp(st1, st0).unwrap();
        a.fscale().unwrap();
        a.fstp(qword_ptr(rcx + 240)).unwrap();
        a.fstp(st0).unwrap();
        // fxtract(6.0): ST(0)=1.5, ST(1)=2.0
        a.fld(qword_ptr(rcx + 48)).unwrap();
        a.fxtract().unwrap();
        a.fstp(qword_ptr(rcx + 248)).unwrap();
        a.fstp(qword_ptr(rcx + 256)).unwrap();
        // fprem(10, 3) = 1.0
        a.fld(qword_ptr(rcx + 32)).unwrap();
        a.fld(qword_ptr(rcx + 56)).unwrap();
        a.fprem().unwrap();
        a.fstp(qword_ptr(rcx + 264)).unwrap();
        a.fstp(st0).unwrap();
        // fprem1(5, 3) = -1.0; fprem1(0.75, 1.0) = -0.25 (d == -1 path!)
        a.fld(qword_ptr(rcx + 32)).unwrap();
        a.fld(qword_ptr(rcx + 64)).unwrap();
        a.fprem1().unwrap();
        a.fstp(qword_ptr(rcx + 272)).unwrap();
        a.fstp(st0).unwrap();
        a.fld1().unwrap();
        a.fld(qword_ptr(rcx + 72)).unwrap();
        a.fprem1().unwrap();
        a.fstp(qword_ptr(rcx + 280)).unwrap();
        a.fstp(st0).unwrap();
        // Out-of-range fsin(2^64) sets C2 = 1 and preserves ST(0)
        a.fld(qword_ptr(rcx + 80)).unwrap();
        a.fsin().unwrap();
        a.fnstsw(word_ptr(rcx + 288)).unwrap();
        a.fstp(st0).unwrap();

        // BCD fbld + fbstp round-trips (positive and negative 18-digit integers)
        a.fbld(tbyte_ptr(rcx + 96)).unwrap();
        a.fbstp(tbyte_ptr(rcx + 300)).unwrap();
        a.fbld(tbyte_ptr(rcx + 112)).unwrap();
        a.fbstp(tbyte_ptr(rcx + 312)).unwrap();

        // fnsave (108B) + frstor (108B) + fnstenv (28B) + fldenv (28B) round-trip
        a.fninit().unwrap();
        a.fld(qword_ptr(rcx + 32)).unwrap(); // ST(1)=3.0
        a.fld(qword_ptr(rcx + 40)).unwrap(); // ST(0)=8.0
        a.fnsave(ptr(rcx + 336)).unwrap(); // saves 108B & re-inits FPU!
        a.fnstsw(word_ptr(rcx + 448)).unwrap(); // must be 0 after fnsave
        a.frstor(ptr(rcx + 336)).unwrap();
        a.fnstenv(ptr(rcx + 464)).unwrap();
        a.fldenv(ptr(rcx + 464)).unwrap();
        a.fstp(qword_ptr(rcx + 504)).unwrap(); // 8.0
        a.fstp(qword_ptr(rcx + 512)).unwrap(); // 3.0
    });
    let r = run_diff_x87("transcendentals_bcd_env", &code, &init);
    let rd = |off: usize| f64::from_bits(u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()));
    assert!((rd(160) - 0.5).abs() < 1e-15, "fsin(pi/6)={}", rd(160));
    assert!((rd(168) - 0.5).abs() < 1e-15, "fcos(pi/3)={}", rd(168));
    assert!((rd(176) - core::f64::consts::FRAC_1_SQRT_2).abs() < 1e-15);
    assert!((rd(184) - core::f64::consts::FRAC_1_SQRT_2).abs() < 1e-15);
    assert_eq!(rd(192), 1.0, "fptan pushes 1.0");
    assert!((rd(200) - 1.0).abs() < 1e-15, "tan(pi/4)={}", rd(200));
    assert!((rd(208) - core::f64::consts::FRAC_PI_4).abs() < 1e-15);
    assert!((rd(216) - (2.0f64.sqrt() - 1.0)).abs() < 1e-15);
    assert!((rd(224) - 9.0).abs() < 1e-14);
    assert!((rd(232) - 3.0 * (1.5f64).log2()).abs() < 1e-14);
    assert_eq!(rd(240), 12.0);
    assert_eq!(rd(248), 1.5);
    assert_eq!(rd(256), 2.0);
    assert_eq!(rd(264), 1.0);
    assert_eq!(rd(272), -1.0);
    assert_eq!(rd(280), -0.25);
    let sw_oor = u16::from_le_bytes(r.data[288..290].try_into().unwrap());
    assert_ne!(sw_oor & 0x0400, 0, "fsin(2^64) must set C2=1");
    assert_eq!(&r.data[300..310], &init[96..106], "positive BCD round-trip");
    assert_eq!(&r.data[312..322], &init[112..122], "negative BCD round-trip");
    assert_eq!(u16::from_le_bytes(r.data[448..450].try_into().unwrap()), 0, "fnsave must re-init FPU");
    assert_eq!(rd(504), 8.0, "frstor+fldenv ST(0)");
    assert_eq!(rd(512), 3.0, "frstor+fldenv ST(1)");

    // MMX aliasing + emms + subsequent x87 arithmetic
    let mmx_code = assemble(|a| {
        a.fninit().unwrap();
        a.movq(mm0, qword_ptr(rcx + 128)).unwrap();
        a.paddw(mm0, qword_ptr(rcx + 136)).unwrap();
        a.movq(qword_ptr(rcx + 544), mm0).unwrap();
        a.emms().unwrap();
        a.fld(qword_ptr(rcx + 32)).unwrap(); // 3.0
        a.fld(qword_ptr(rcx + 40)).unwrap(); // 8.0
        a.faddp(st1, st0).unwrap(); // 11.0
        a.fstp(qword_ptr(rcx + 552)).unwrap();
    });
    let mr = run_diff_x87("mmx_emms_then_x87", &mmx_code, &init);
    assert_eq!(
        u64::from_le_bytes(mr.data[544..552].try_into().unwrap()),
        0x0006_0008_000A_000Cu64
    );
    assert_eq!(
        f64::from_bits(u64::from_le_bytes(mr.data[552..560].try_into().unwrap())),
        11.0
    );
}

/// 7. AOT inclusion, shifted-base reload execution, and signed static Mach-O `.aot.o`
/// emission for macOS & iOS (`skipped_reloc_blocks == 0`).
#[test]
fn x87_aot_inclusion_and_static_macho_zero_relocs() {
    let _guard = X87Guard::new();
    let mut a = CodeAssembler::new(64).unwrap();
    // Mix of inline ARM64 x87 ops + static-TOP chain + cold soft-f80 (PC=64) +
    // transcendental (fsincos) + BCD (fbld/fbstp) + fnsave/frstor!
    a.fninit().unwrap();
    a.fld(qword_ptr(rcx)).unwrap(); // 3.0
    a.fld(qword_ptr(rcx + 8)).unwrap(); // 4.0
    a.fmul_2(st0, st0).unwrap(); // 16.0
    a.fxch(st0, st1).unwrap();
    a.fmul_2(st0, st0).unwrap(); // 9.0
    a.faddp(st1, st0).unwrap(); // 25.0
    a.fsqrt().unwrap(); // 5.0
    a.fstp(qword_ptr(rcx + 64)).unwrap();

    // Helper-backed ops inside the same AOT block:
    a.fld(qword_ptr(rcx + 16)).unwrap(); // pi/4
    a.fsincos().unwrap();
    a.fstp(qword_ptr(rcx + 72)).unwrap();
    a.fstp(qword_ptr(rcx + 80)).unwrap();
    a.fbld(tbyte_ptr(rcx + 32)).unwrap();
    a.fbstp(tbyte_ptr(rcx + 96)).unwrap();
    a.fld1().unwrap();
    a.fnsave(ptr(rcx + 128)).unwrap();
    a.frstor(ptr(rcx + 128)).unwrap();
    a.fstp(qword_ptr(rcx + 240)).unwrap();
    a.mov(rax, 0x5838_3700u64).unwrap();
    a.ret().unwrap();

    let code_cap = 0x2000usize;
    let code = arena_alloc(code_cap, 16);
    let bytes = a.assemble(code as u64).unwrap();
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), code, bytes.len());
        core::ptr::write_bytes(code.add(bytes.len()), 0xCC, 16);
    }

    let mut ips = vec![code as u64];
    let mut dec = Decoder::with_ip(64, &bytes, code as u64, DecoderOptions::NONE);
    let mut cnt = 0usize;
    while dec.can_decode() {
        let ins = dec.decode();
        cnt += 1;
        if cnt >= BLK_MAX_INSNS {
            ips.push(ins.next_ip());
            cnt = 0;
        }
    }
    ips.retain(|&ip| ip >= code as u64 && ip < code as u64 + bytes.len() as u64);
    ips.sort_unstable();
    ips.dedup();

    let mut bc = Box::new(BlockCache::new());
    let img = unsafe { aot::build_image_for_test(&mut *bc, &ips) }.expect("build x87 AOT image");
    assert_eq!(img.blocks.len(), ips.len(), "every x87 block must compile into AOT");
    for blk in &img.blocks {
        assert_eq!(blk.reloc_n, 0, "x87 AOT blocks must emit zero runtime relocations");
    }

    let key = aot::AotKey {
        image_hash: [0x8787_1234_5678_9ABC, 0xDEF0_1122_3344_5566],
        image_base: code as u64,
        image_size: code_cap as u64,
        codegen_hash: aot::codegen_hash(),
        knobs: jit::aot_knob_bits(),
    };
    let aot_bytes = aot::encode_for_test("x87_aot_test.dll", &key, &img);

    // Emit static Mach-O `.aot.o` for both macOS and iOS with ZERO skipped reloc blocks!
    for plat in [MachoPlatform::MacOs, MachoPlatform::Ios] {
        let (_obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
            .unwrap_or_else(|e| panic!("static Mach-O emit ({plat}) failed: {e}"));
        assert_eq!(rep.skipped_reloc_blocks, 0, "x87 static Mach-O ({plat}) must skip 0 blocks");
        assert_eq!(rep.emitted_blocks, img.blocks.len());
    }

    // Execute via VerifyAotBatch (real split+rebased AOT image + AOT stubs) and compare vs JIT/interp!
    let mut init = vec![0u8; 320];
    init[0..8].copy_from_slice(&3.0f64.to_bits().to_le_bytes());
    init[8..16].copy_from_slice(&4.0f64.to_bits().to_le_bytes());
    init[16..24].copy_from_slice(&(core::f64::consts::FRAC_PI_4).to_bits().to_le_bytes());
    init[32..42].copy_from_slice(&[0x21, 0x43, 0x65, 0x87, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00]);
    let d_aot = arena_alloc(init.len(), 64);
    let ref_res = run_diff_x87_at("x87_aot_vs_jit_interp", code, d_aot, init.len(), &init);

    let batch = unsafe { aot::VerifyAotBatch::build(&ips, 1) }.expect("build VerifyAotBatch for x87");
    let (enter, entry_code) = batch.block_entry(code as u64).expect("x87 block entry in AOT batch");
    let stack = arena_alloc(0x4000, 16);
    let rsp_val = stack as u64 + 0x3F00;
    let sentinel = 0xDEAD_BEEF_CAFE_0000u64;
    let mut jc = JitCache::new();
    let aot_res = unsafe {
        core::ptr::copy_nonoverlapping(init.as_ptr(), d_aot, init.len());
        core::ptr::write_unaligned(rsp_val as *mut u64, sentinel);
        x87_reset();
        let mut cpu = Cpu::new_zeroed();
        cpu.rip = code as u64;
        cpu.gpr[1] = d_aot as u64;
        cpu.gpr[4] = rsp_val;
        jc.run_code_for_verify(&mut cpu, &mut *bc, Some(enter), entry_code, 256);
        assert_eq!(cpu.rip, sentinel, "AOT x87 block must return cleanly");
        RunResult {
            rax: cpu.gpr[0],
            data: core::slice::from_raw_parts(d_aot, init.len()).to_vec(),
            x87: x87_snapshot(),
        }
    };
    assert_eq!(aot_res.rax, ref_res.rax, "AOT vs Generic RAX");
    assert_eq!(aot_res.data, ref_res.data, "AOT vs Generic memory");
    assert_x87_eq("AOT vs Generic", &aot_res.x87, &ref_res.x87);
    assert_eq!(
        f64::from_bits(u64::from_le_bytes(aot_res.data[64..72].try_into().unwrap())),
        5.0
    );
    assert_eq!(&aot_res.data[96..106], &init[32..42]);
    assert_eq!(
        f64::from_bits(u64::from_le_bytes(aot_res.data[240..248].try_into().unwrap())),
        1.0
    );
}

/// 8. Differential tests for 16-bit `fnstenv`/`fldenv` (14B) and `fnsave`/`frstor` (94B),
/// fused `fcomi + jcc` branches (incl. unordered NaN), and static TOP tracking across
/// mid-block cold soft-f80 fallbacks.
#[test]
fn x87_differential_env16_fcomi_jcc_and_static_top_cold_fallback() {
    let mut init = vec![0u8; 320];
    init[0..8].copy_from_slice(&1.0f64.to_bits().to_le_bytes());
    init[8..16].copy_from_slice(&(2.0f64.powi(-60)).to_bits().to_le_bytes());
    init[16..24].copy_from_slice(&7.0f64.to_bits().to_le_bytes());
    init[24..32].copy_from_slice(&f64::NAN.to_bits().to_le_bytes());
    init[32..34].copy_from_slice(&0x037Fu16.to_le_bytes()); // PC=64

    // Static TOP chain starting from fninit, hitting cold soft-f80 (PC=64) in the middle,
    // then continuing with inline ops and storing final stack + SW!
    let top_code = assemble(|a| {
        a.fninit().unwrap();
        a.fldcw(word_ptr(rcx + 32)).unwrap(); // PC=64
        a.fld(qword_ptr(rcx + 16)).unwrap(); // TOP=7: 7.0
        a.fld(qword_ptr(rcx)).unwrap(); // TOP=6: 1.0
        a.fld(qword_ptr(rcx + 8)).unwrap(); // TOP=5: 2^-60
        a.faddp(st1, st0).unwrap(); // cold soft-f80! TOP=6: 1 + 2^-60
        a.fstp(tbyte_ptr(rcx + 64)).unwrap(); // TOP=7
        a.fld1().unwrap(); // TOP=6: 1.0
        a.faddp(st1, st0).unwrap(); // TOP=7: 8.0
        a.fstp(qword_ptr(rcx + 80)).unwrap(); // TOP=0
        a.fnstsw(word_ptr(rcx + 88)).unwrap();
    });
    let r = run_diff_x87("static_top_mid_chain_cold", &top_code, &init);
    assert_eq!(
        u64::from_le_bytes(r.data[64..72].try_into().unwrap()),
        0x8000_0000_0000_0008,
        "PC=64 cold faddp mid-chain must keep exact 64-bit significand"
    );
    assert_eq!(
        f64::from_bits(u64::from_le_bytes(r.data[80..88].try_into().unwrap())),
        8.0
    );

    // 16-bit 14B/94B env & save/restore round-trip (0x66 prefix)
    let mut env16_bytes = Vec::new();
    let mut a = CodeAssembler::new(64).unwrap();
    a.fninit().unwrap();
    a.fld(qword_ptr(rcx + 16)).unwrap(); // 7.0
    a.fld1().unwrap(); // 1.0
    a.add(rcx, 128).unwrap();
    env16_bytes.extend_from_slice(&a.assemble(0x10_0000).unwrap());
    // 66 DD 31 = fnsave16 [rcx] (94B); 66 DD 21 = frstor16 [rcx] (94B)
    // 66 D9 31 = fnstenv16 [rcx] (14B); 66 D9 21 = fldenv16 [rcx] (14B)
    env16_bytes.extend_from_slice(&[
        0x66, 0xDD, 0x31, // fnsave [rcx] (94B)
        0x66, 0xDD, 0x21, // frstor [rcx] (94B)
        0x66, 0xD9, 0x31, // fnstenv [rcx] (14B)
        0x66, 0xD9, 0x21, // fldenv [rcx] (14B)
    ]);
    let mut b = CodeAssembler::new(64).unwrap();
    b.fstp(qword_ptr(rcx + 112)).unwrap(); // 1.0 -> offset 240
    b.fstp(qword_ptr(rcx + 120)).unwrap(); // 7.0 -> offset 248
    b.ret().unwrap();
    env16_bytes.extend_from_slice(&b.assemble(0x10_0000).unwrap());
    let er = run_diff_x87("env16_fnsave94_frstor94", &env16_bytes, &init);
    assert_eq!(f64::from_bits(u64::from_le_bytes(er.data[240..248].try_into().unwrap())), 1.0);
    assert_eq!(f64::from_bits(u64::from_le_bytes(er.data[248..256].try_into().unwrap())), 7.0);

    // Fused fcomi + jcc (jp/jb/je/ja) for ordered and unordered (NaN) comparisons!
    for (case_i, (a_off, b_off, want_rax)) in [
        (16i32, 0i32, 1u64), // 7.0 > 1.0 -> ja
        (0, 16, 2),          // 1.0 < 7.0 -> jb
        (0, 0, 3),           // 1.0 == 1.0 -> je
        (0, 24, 4),          // 1.0 vs NaN -> jp
    ]
    .into_iter()
    .enumerate()
    {
        let mut fa = CodeAssembler::new(64).unwrap();
        let mut l_unord = fa.create_label();
        let mut l_above = fa.create_label();
        let mut l_below = fa.create_label();
        let mut l_done = fa.create_label();
        fa.fninit().unwrap();
        fa.fld(qword_ptr(rcx + b_off)).unwrap();
        fa.fld(qword_ptr(rcx + a_off)).unwrap();
        fa.fucomip(st0, st1).unwrap();
        fa.jp(l_unord).unwrap();
        fa.ja(l_above).unwrap();
        fa.jb(l_below).unwrap();
        fa.mov(rax, 3u64).unwrap();
        fa.jmp(l_done).unwrap();
        fa.set_label(&mut l_unord).unwrap();
        fa.mov(rax, 4u64).unwrap();
        fa.jmp(l_done).unwrap();
        fa.set_label(&mut l_above).unwrap();
        fa.mov(rax, 1u64).unwrap();
        fa.jmp(l_done).unwrap();
        fa.set_label(&mut l_below).unwrap();
        fa.mov(rax, 2u64).unwrap();
        fa.set_label(&mut l_done).unwrap();
        fa.fstp(st0).unwrap();
        fa.ret().unwrap();
        let fbytes = fa.assemble(0x10_0000).unwrap();
        let fr = run_diff_x87(&format!("fcomi_jcc_case_{case_i}"), &fbytes, &init);
        assert_eq!(fr.rax, want_rax, "fcomi_jcc case {case_i}");
    }
}

/// 9. Regression: x87 lowering must invalidate `Tr::xa_key` whenever it overwrites `XA`
/// (both when `load_op` populates `xa_key` before `load_x87_state` and when an earlier
/// ordinary memory access populated `xa_key` before `fnstcw`/`fnstsw`/`fstp`/`fistp`).
#[test]
fn x87_xa_key_invalidation_with_indexed_mem() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 512];
    let f_val: f32 = 12.5;
    let f_add: f32 = 3.25;
    let i_val: i32 = 42;
    let cw_val: u16 = 0x027F;
    init[16..20].copy_from_slice(&f_val.to_bits().to_le_bytes()); // [rcx + 4*4]
    init[32..36].copy_from_slice(&f_add.to_bits().to_le_bytes()); // [rcx + 8*4]
    init[48..52].copy_from_slice(&i_val.to_le_bytes());           // [rcx + 12*4]
    init[64..66].copy_from_slice(&cw_val.to_le_bytes());          // [rcx + 16*4]
    init[80..84].copy_from_slice(&0x1122_3344u32.to_le_bytes());  // [rcx + 20*4]
    init[96..100].copy_from_slice(&0x5566_7788u32.to_le_bytes()); // [rcx + 24*4]

    let code = assemble(|a| {
        a.fninit().unwrap();
        // Pattern 1: fld dword ptr [rcx + rdx*4] populates xa_key, clobbers XA with X87State*,
        // then `mov eax, [rcx + rdx*4]` MUST still read guest memory, NOT X87State!
        a.mov(rdx, 4u64).unwrap();
        a.fld(dword_ptr(rcx + rdx * 4)).unwrap();
        a.mov(eax, dword_ptr(rcx + rdx * 4)).unwrap();
        a.mov(dword_ptr(rcx + 128), eax).unwrap();

        // Pattern 2: fadd dword ptr [rcx + rdx*4] followed by mov eax, [rcx + rdx*4]
        a.mov(rdx, 8u64).unwrap();
        a.fadd(dword_ptr(rcx + rdx * 4)).unwrap();
        a.mov(eax, dword_ptr(rcx + rdx * 4)).unwrap();
        a.mov(dword_ptr(rcx + 132), eax).unwrap();

        // Pattern 3: fild dword ptr [rcx + rdx*4] followed by mov eax, [rcx + rdx*4]
        a.mov(rdx, 12u64).unwrap();
        a.fild(dword_ptr(rcx + rdx * 4)).unwrap();
        a.mov(eax, dword_ptr(rcx + rdx * 4)).unwrap();
        a.mov(dword_ptr(rcx + 136), eax).unwrap();
        a.faddp(st1, st0).unwrap(); // ST(0) = 12.5 + 3.25 + 42.0 = 57.75

        // Pattern 4: fldcw word ptr [rcx + rdx*4] followed by movzx eax, word ptr [rcx + rdx*4]
        a.mov(rdx, 16u64).unwrap();
        a.fldcw(word_ptr(rcx + rdx * 4)).unwrap();
        a.movzx(eax, word_ptr(rcx + rdx * 4)).unwrap();
        a.mov(dword_ptr(rcx + 140), eax).unwrap();

        // Pattern 5: ordinary load sets xa_key, then fnstcw/fnstsw store to the SAME indexed address!
        a.mov(rdx, 20u64).unwrap();
        a.mov(eax, dword_ptr(rcx + rdx * 4)).unwrap();
        a.fnstcw(word_ptr(rcx + rdx * 4)).unwrap();
        a.mov(rdx, 24u64).unwrap();
        a.mov(eax, dword_ptr(rcx + rdx * 4)).unwrap();
        a.fnstsw(word_ptr(rcx + rdx * 4)).unwrap();

        // Pattern 6: ordinary load sets xa_key, then fstp stores to the same indexed address!
        a.mov(rdx, 40u64).unwrap(); // [rcx + 160]
        a.mov(eax, dword_ptr(rcx + rdx * 4)).unwrap();
        a.fstp(dword_ptr(rcx + rdx * 4)).unwrap();
    });

    let r = run_diff_x87("xa_key_invalidation", &code, &init);
    assert_eq!(u32::from_le_bytes(r.data[128..132].try_into().unwrap()), f_val.to_bits());
    assert_eq!(u32::from_le_bytes(r.data[132..136].try_into().unwrap()), f_add.to_bits());
    assert_eq!(i32::from_le_bytes(r.data[136..140].try_into().unwrap()), i_val);
    assert_eq!(u32::from_le_bytes(r.data[140..144].try_into().unwrap()), cw_val as u32);
    assert_eq!(u16::from_le_bytes(r.data[80..82].try_into().unwrap()), cw_val);
    assert_eq!(
        f32::from_bits(u32::from_le_bytes(r.data[160..164].try_into().unwrap())),
        57.75f32
    );
}

/// 10. Regression: FADD/FSUB with a zero operand must still apply CW.PC precision rounding
/// and raise SW.PE when the nonzero operand has extra significand bits beyond CW.PC.
#[test]
fn x87_add_sub_zero_precision_rounding() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 512];
    // m80 value = 1 + 2^-60: sig = 0x8000_0000_0000_0008, exp_biased = 0x3FFF
    init[0..8].copy_from_slice(&0x8000_0000_0000_0008u64.to_le_bytes());
    init[8..10].copy_from_slice(&0x3FFFu16.to_le_bytes());
    // Control words:
    init[16..18].copy_from_slice(&0x027Fu16.to_le_bytes()); // PC=53, RN
    init[18..20].copy_from_slice(&0x0A7Fu16.to_le_bytes()); // PC=53, RP (+inf)
    init[20..22].copy_from_slice(&0x007Fu16.to_le_bytes()); // PC=24, RN
    init[22..24].copy_from_slice(&0x037Fu16.to_le_bytes()); // PC=64, RN

    let code = assemble(|a| {
        // Case 1: PC=53, RN: (1 + 2^-60) + 0.0 -> 1.0 (sig = 0x8000_0000_0000_0000), SW.PE = 1
        a.fninit().unwrap();
        a.fldcw(word_ptr(rcx + 16)).unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fldz().unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 64)).unwrap();
        a.fstp(tbyte_ptr(rcx + 80)).unwrap();

        // Case 2: PC=53, RN: 0.0 + (1 + 2^-60) -> reversed operand order
        a.fnclex().unwrap();
        a.fldz().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 66)).unwrap();
        a.fstp(tbyte_ptr(rcx + 96)).unwrap();

        // Case 3: PC=53, RN: 0.0 - (1 + 2^-60) -> -1.0 (sig = 0x8000_0000_0000_0000, se = 0xBFFF), SW.PE = 1
        a.fnclex().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fldz().unwrap();
        a.fsubp(st1, st0).unwrap(); // ST(1) - ST(0) = (1 + 2^-60) - 0.0
        a.fnstsw(word_ptr(rcx + 68)).unwrap();
        a.fstp(tbyte_ptr(rcx + 112)).unwrap();

        a.fnclex().unwrap();
        a.fldz().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fsubp(st1, st0).unwrap(); // ST(1) - ST(0) = 0.0 - (1 + 2^-60)
        a.fnstsw(word_ptr(rcx + 70)).unwrap();
        a.fstp(tbyte_ptr(rcx + 128)).unwrap();

        // Case 4: PC=53, RP (+inf): (1 + 2^-60) + 0.0 -> 1 + 2^-52 (sig = 0x8000_0000_0000_0800), SW.PE = 1
        a.fnclex().unwrap();
        a.fldcw(word_ptr(rcx + 18)).unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fldz().unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 72)).unwrap();
        a.fstp(tbyte_ptr(rcx + 144)).unwrap();

        // Case 5: PC=24, RN: (1 + 2^-60) + 0.0 -> 1.0 (sig = 0x8000_0000_0000_0000), SW.PE = 1
        a.fnclex().unwrap();
        a.fldcw(word_ptr(rcx + 20)).unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fldz().unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 74)).unwrap();
        a.fstp(tbyte_ptr(rcx + 160)).unwrap();

        // Case 6: PC=64, RN: (1 + 2^-60) + 0.0 -> exact (sig = 0x8000_0000_0000_0008), SW.PE = 0
        a.fnclex().unwrap();
        a.fldcw(word_ptr(rcx + 22)).unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fldz().unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 76)).unwrap();
        a.fstp(tbyte_ptr(rcx + 176)).unwrap();
    });

    let r = run_diff_x87("add_sub_zero_pc_rounding", &code, &init);
    let rd_m80 = |off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(r.data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    let rd_sw = |off: usize| u16::from_le_bytes(r.data[off..off + 2].try_into().unwrap());
    const SW_PE: u16 = 0x0020;

    assert_eq!(rd_m80(80), (0x8000_0000_0000_0000, 0x3FFF));
    assert_ne!(rd_sw(64) & SW_PE, 0, "PC=53 RN (x + 0) must set PE");

    assert_eq!(rd_m80(96), (0x8000_0000_0000_0000, 0x3FFF));
    assert_ne!(rd_sw(66) & SW_PE, 0, "PC=53 RN (0 + x) must set PE");

    assert_eq!(rd_m80(112), (0x8000_0000_0000_0000, 0x3FFF));
    assert_ne!(rd_sw(68) & SW_PE, 0, "PC=53 RN (x - 0) must set PE");

    assert_eq!(rd_m80(128), (0x8000_0000_0000_0000, 0xBFFF));
    assert_ne!(rd_sw(70) & SW_PE, 0, "PC=53 RN (0 - x) must set PE");

    assert_eq!(rd_m80(144), (0x8000_0000_0000_0800, 0x3FFF));
    assert_ne!(rd_sw(72) & SW_PE, 0, "PC=53 RP (x + 0) must round up and set PE");

    assert_eq!(rd_m80(160), (0x8000_0000_0000_0000, 0x3FFF));
    assert_ne!(rd_sw(74) & SW_PE, 0, "PC=24 RN (x + 0) must round and set PE");

    assert_eq!(rd_m80(176), (0x8000_0000_0000_0008, 0x3FFF));
    assert_eq!(rd_sw(76) & SW_PE, 0, "PC=64 RN (x + 0) must stay exact with PE=0");
}

/// 11. Regressions for:
///   (a) `BinopMem` producing an exact `f64` subnormal without setting `FPSR.UFC` (e.g.
///       `2^-1022 * 0.5 = 2^-1023`, `2^-1022 / 2.0 = 2^-1023`, `(2^-1022 + 2^-1023) - 2^-1022`):
///       must preserve the loaded RHS in `T6` when branching to the cold helper!
///   (b) x87 environment bookkeeping (`FIP`, `FDP`, `FOP`) across Generic, FastOp, Live JIT,
///       `fnstenv`, and `fxsave`: data instructions update `FIP`/`FOP` (and `FDP` if memory),
///       while control instructions (`fnstcw`) leave `FIP`/`FDP`/`FOP` untouched.
#[test]
fn x87_env_fip_fdp_fop_and_exact_subnormal_mul_mem() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 768];
    let min_norm: f64 = f64::from_bits(0x0010_0000_0000_0000); // 2^-1022
    let half: f64 = 0.5;
    let two: f64 = 2.0;
    let one_and_half_min: f64 = f64::from_bits(0x0018_0000_0000_0000); // 1.5 * 2^-1022
    init[0..8].copy_from_slice(&min_norm.to_bits().to_le_bytes());
    init[8..16].copy_from_slice(&half.to_bits().to_le_bytes());
    init[16..24].copy_from_slice(&two.to_bits().to_le_bytes());
    init[24..32].copy_from_slice(&one_and_half_min.to_bits().to_le_bytes());
    init[32..34].copy_from_slice(&0x027Fu16.to_le_bytes()); // CW: PC=53, RN

    // (a) Exact f64 subnormal results in BinopMem (PC=53, FPSR.UFC == 0)
    let subnorm_code = assemble(|a| {
        a.fninit().unwrap();
        a.fldcw(word_ptr(rcx + 32)).unwrap();
        // 2^-1022 * 0.5 = 2^-1023 (exact normal 80-bit: sig=0x8000..0, se=0x3C00)
        a.fld(qword_ptr(rcx)).unwrap();
        a.fmul(qword_ptr(rcx + 8)).unwrap();
        a.fstp(tbyte_ptr(rcx + 64)).unwrap();

        // 2^-1022 / 2.0 = 2^-1023
        a.fld(qword_ptr(rcx)).unwrap();
        a.fdiv(qword_ptr(rcx + 16)).unwrap();
        a.fstp(tbyte_ptr(rcx + 80)).unwrap();

        // 1.5 * 2^-1022 - 2^-1022 = 2^-1023
        a.fld(qword_ptr(rcx + 24)).unwrap();
        a.fsub(qword_ptr(rcx)).unwrap();
        a.fstp(tbyte_ptr(rcx + 96)).unwrap();
    });
    let sr = run_diff_x87("exact_subnormal_binop_mem", &subnorm_code, &init);
    let rd_m80 = |data: &[u8], off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    // 2^-1023 in 80-bit extended: unbiased exp = -1023 -> biased = 16383 - 1023 = 15360 = 0x3C00
    assert_eq!(rd_m80(&sr.data, 64), (0x8000_0000_0000_0000, 0x3C00), "2^-1022 * 0.5");
    assert_eq!(rd_m80(&sr.data, 80), (0x8000_0000_0000_0000, 0x3C00), "2^-1022 / 2.0");
    assert_eq!(rd_m80(&sr.data, 96), (0x8000_0000_0000_0000, 0x3C00), "1.5*2^-1022 - 2^-1022");

    // (b) x87 environment bookkeeping (FIP/FDP/FOP) via fnstenv AND fxsave
    let env_code = assemble(|a| {
        a.fninit().unwrap();
        // Memory data op: sets FIP, FDP = rcx + 8, FOP = DD 41 -> ((5 << 8) | 0x41) = 0x0541
        a.fld(qword_ptr(rcx + 8)).unwrap();
        a.fnstenv(ptr(rcx + 64)).unwrap();
        a.fxsave64(ptr(rcx + 192)).unwrap(); // 16-byte aligned (d_buf is 64-byte aligned)
        // Non-memory data op: updates FIP & FOP (D8 C0 -> 0x00C0), preserves FDP = rcx + 8
        a.fadd_2(st0, st0).unwrap();
        a.fnstenv(ptr(rcx + 96)).unwrap();
        // Control op: must NOT alter FIP, FDP, or FOP!
        a.fnstcw(word_ptr(rcx + 128)).unwrap();
        a.fnstenv(ptr(rcx + 144)).unwrap();
    });
    let code_cap = (env_code.len() + 64).max(256);
    let code = arena_alloc(code_cap, 16);
    unsafe {
        core::ptr::copy_nonoverlapping(env_code.as_ptr(), code, env_code.len());
        core::ptr::write_bytes(code.add(env_code.len()), 0xCC, 16);
    }
    let d_buf = arena_alloc(init.len(), 64);
    let er = run_diff_x87_at("x87_env_fip_fdp_fop", code, d_buf, init.len(), &init);

    // Verify exact non-zero FIP/FDP/FOP recorded by fld and fadd, preserved across fnstcw!
    let fip_after_fld = u32::from_le_bytes(er.data[64 + 12..64 + 16].try_into().unwrap());
    let fop_after_fld = u16::from_le_bytes(er.data[64 + 18..64 + 20].try_into().unwrap());
    let fdp_after_fld = u32::from_le_bytes(er.data[64 + 20..64 + 24].try_into().unwrap());
    assert_ne!(fip_after_fld, 0, "fnstenv after fld must record non-zero FIP");
    assert_eq!(fdp_after_fld, (d_buf as u64 + 8) as u32, "fnstenv after fld must record FDP = rcx + 8");
    assert_ne!(fop_after_fld, 0, "fnstenv after fld must record non-zero FOP");
    // fxsave at +192: FOP at +6, FIP (64-bit) at +8, FDP (64-bit) at +16
    assert_eq!(u16::from_le_bytes(er.data[192 + 6..192 + 8].try_into().unwrap()), fop_after_fld);
    assert_eq!(u64::from_le_bytes(er.data[192 + 8..192 + 16].try_into().unwrap()) as u32, fip_after_fld);
    assert_eq!(u64::from_le_bytes(er.data[192 + 16..192 + 24].try_into().unwrap()), d_buf as u64 + 8);
    // After fadd st0, st0: FIP moved forward, FOP updated, FDP preserved!
    let fip_after_fadd = u32::from_le_bytes(er.data[96 + 12..96 + 16].try_into().unwrap());
    let fop_after_fadd = u16::from_le_bytes(er.data[96 + 18..96 + 20].try_into().unwrap());
    let fdp_after_fadd = u32::from_le_bytes(er.data[96 + 20..96 + 24].try_into().unwrap());
    assert_ne!(fip_after_fadd, fip_after_fld, "fadd must advance FIP");
    assert_eq!(fop_after_fadd, 0x00C0, "fadd st0, st0 FOP must be 0x00C0");
    assert_eq!(fdp_after_fadd, fdp_after_fld, "non-memory fadd must preserve FDP");
    // After fnstcw (control op): FIP, FOP, FDP all unchanged!
    assert_eq!(&er.data[144 + 12..144 + 24], &er.data[96 + 12..96 + 24], "fnstcw must not alter FIP/FOP/FDP");
}

/// 12. Regression: mixed QNaN + SNaN x87 arithmetic must prefer the quiet NaN regardless of
/// 62-bit payload magnitude, while still raising `SW.IE`.
#[test]
fn x87_mixed_qnan_snan_prefers_qnan() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 256];
    // QNaN with small payload: sig = 0xC000_0000_0000_0001, se = 0x7FFF
    init[0..8].copy_from_slice(&0xC000_0000_0000_0001u64.to_le_bytes());
    init[8..10].copy_from_slice(&0x7FFFu16.to_le_bytes());
    // SNaN with maximal 62-bit payload: sig = 0xBFFF_FFFF_FFFF_FFFF, se = 0x7FFF
    init[16..24].copy_from_slice(&0xBFFF_FFFF_FFFF_FFFFu64.to_le_bytes());
    init[24..26].copy_from_slice(&0x7FFFu16.to_le_bytes());
    // SNaN with smaller payload: sig = 0x8000_0000_0000_0002, se = 0x7FFF
    init[32..40].copy_from_slice(&0x8000_0000_0000_0002u64.to_le_bytes());
    init[40..42].copy_from_slice(&0x7FFFu16.to_le_bytes());

    let code = assemble(|a| {
        // Order 1: QNaN + SNaN -> QNaN unchanged, SW.IE set
        a.fninit().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fld(tbyte_ptr(rcx + 16)).unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 64)).unwrap();
        a.fstp(tbyte_ptr(rcx + 80)).unwrap();

        // Order 2: SNaN + QNaN -> QNaN unchanged, SW.IE set
        a.fnclex().unwrap();
        a.fld(tbyte_ptr(rcx + 16)).unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 66)).unwrap();
        a.fstp(tbyte_ptr(rcx + 96)).unwrap();

        // Both SNaN: larger payload quieted (0xFFFF_FFFF_FFFF_FFFF), SW.IE set
        a.fnclex().unwrap();
        a.fld(tbyte_ptr(rcx + 32)).unwrap();
        a.fld(tbyte_ptr(rcx + 16)).unwrap();
        a.faddp(st1, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 68)).unwrap();
        a.fstp(tbyte_ptr(rcx + 112)).unwrap();
    });

    let r = run_diff_x87("mixed_qnan_snan", &code, &init);
    let rd_m80 = |off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(r.data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    let rd_sw = |off: usize| u16::from_le_bytes(r.data[off..off + 2].try_into().unwrap());
    const SW_IE: u16 = 0x0001;

    assert_eq!(rd_m80(80), (0xC000_0000_0000_0001, 0x7FFF));
    assert_ne!(rd_sw(64) & SW_IE, 0);
    assert_eq!(rd_m80(96), (0xC000_0000_0000_0001, 0x7FFF));
    assert_ne!(rd_sw(66) & SW_IE, 0);
    assert_eq!(rd_m80(112), (0xFFFF_FFFF_FFFF_FFFF, 0x7FFF));
    assert_ne!(rd_sw(68) & SW_IE, 0);
}

/// 13. Regression: `fcmov*` must validate both `ST(0)` and `ST(i)` BEFORE checking the
/// condition, raising `SW.IE | SW.SF` and storing indefinite QNaN to `ST(0)` on any empty slot.
#[test]
fn x87_fcmov_validates_both_operands_before_condition() {
    let _guard = X87Guard::new();
    let init = vec![0u8; 256];
    let code = assemble(|a| {
        // Case 1: ST(0) empty, ST(7) valid (1.0), CF=1 (cond true) -> IE|SF + QNAN_INDEF
        a.fninit().unwrap();
        a.fld1().unwrap();
        a.fincstp().unwrap(); // ST(0) is now empty, ST(7) holds 1.0
        a.stc().unwrap();
        a.fcmovb(st0, st7).unwrap();
        a.fnstsw(word_ptr(rcx)).unwrap();
        a.fstp(tbyte_ptr(rcx + 16)).unwrap();

        // Case 2: both empty, CF=0 (cond false) -> still raises IE|SF + QNAN_INDEF
        a.fninit().unwrap();
        a.clc().unwrap();
        a.fcmovb(st0, st1).unwrap();
        a.fnstsw(word_ptr(rcx + 2)).unwrap();
        a.fstp(tbyte_ptr(rcx + 32)).unwrap();

        // Case 3: ST(0) valid (1.0), ST(1) empty, CF=0 (cond false) -> raises IE|SF + QNAN_INDEF
        a.fninit().unwrap();
        a.fld1().unwrap();
        a.clc().unwrap();
        a.fcmovb(st0, st1).unwrap();
        a.fnstsw(word_ptr(rcx + 4)).unwrap();
        a.fstp(tbyte_ptr(rcx + 48)).unwrap();

        // Case 4: ST(0) empty, ST(7) valid (1.0), CF=0 (cond false) -> raises IE|SF + QNAN_INDEF
        a.fninit().unwrap();
        a.fld1().unwrap();
        a.fincstp().unwrap();
        a.clc().unwrap();
        a.fcmovb(st0, st7).unwrap();
        a.fnstsw(word_ptr(rcx + 6)).unwrap();
        a.fstp(tbyte_ptr(rcx + 64)).unwrap();
    });

    let r = run_diff_x87("fcmov_operand_validation", &code, &init);
    let rd_sw = |off: usize| u16::from_le_bytes(r.data[off..off + 2].try_into().unwrap());
    let rd_m80 = |off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(r.data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    const SW_IE_SF: u16 = 0x0041;
    const INDEF: (u64, u16) = (0xC000_0000_0000_0000, 0xFFFF);

    for (idx, (sw_off, m80_off)) in [(0, 16), (2, 32), (4, 48), (6, 64)].into_iter().enumerate() {
        assert_eq!(rd_sw(sw_off) & SW_IE_SF, SW_IE_SF, "case {idx} must set IE|SF");
        assert_eq!(rd_m80(m80_off), INDEF, "case {idx} must write QNAN_INDEF to ST(0)");
    }
}

/// 14. Regression: `frndint` rounding up across a power-of-two boundary (e.g. `1.5 -> 2.0`,
/// `-1.5 -> -2.0`, `3.75 -> 4.0`, `1.5 + 2^-60 -> 2.0`, `2^63 - 0.5 -> 2^63`) must keep the
/// J-bit normalized (`sig = 0x8000_0000_0000_0000`) and never produce an unnormal encoding.
#[test]
fn x87_frndint_power_of_two_carry() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 256];
    init[0..8].copy_from_slice(&1.5f64.to_bits().to_le_bytes());
    init[8..16].copy_from_slice(&(-1.5f64).to_bits().to_le_bytes());
    init[16..24].copy_from_slice(&3.75f64.to_bits().to_le_bytes());
    // m80: 1.5 + 2^-60 -> forces cold soft-f80 frndint path even under JIT!
    init[32..40].copy_from_slice(&0xC000_0000_0000_0008u64.to_le_bytes());
    init[40..42].copy_from_slice(&0x3FFFu16.to_le_bytes());
    // m80: 2^63 - 0.5 (exp = 62 -> biased 0x403D, sig = u64::MAX) -> rounds to 2^63 (0x403E:0x8000..0)
    init[48..56].copy_from_slice(&u64::MAX.to_le_bytes());
    init[56..58].copy_from_slice(&0x403Du16.to_le_bytes());

    let code = assemble(|a| {
        a.fninit().unwrap();
        // 1.5 -> 2.0, then fadd st0, st0 -> 4.0 (verifies normalized J-bit; unnormal would raise IE!)
        a.fld(qword_ptr(rcx)).unwrap();
        a.frndint().unwrap();
        a.fld(st0).unwrap();
        a.fstp(tbyte_ptr(rcx + 64)).unwrap();
        a.fadd_2(st0, st0).unwrap();
        a.fnstsw(word_ptr(rcx + 200)).unwrap();
        a.fstp(qword_ptr(rcx + 144)).unwrap();

        // -1.5 -> -2.0
        a.fld(qword_ptr(rcx + 8)).unwrap();
        a.frndint().unwrap();
        a.fstp(tbyte_ptr(rcx + 80)).unwrap();

        // 3.75 -> 4.0
        a.fld(qword_ptr(rcx + 16)).unwrap();
        a.frndint().unwrap();
        a.fstp(tbyte_ptr(rcx + 96)).unwrap();

        // m80 (1.5 + 2^-60) -> 2.0 (cold path)
        a.fld(tbyte_ptr(rcx + 32)).unwrap();
        a.frndint().unwrap();
        a.fstp(tbyte_ptr(rcx + 112)).unwrap();

        // m80 (2^63 - 0.5) -> 2^63 (cold path, exp = 62 -> 63 boundary)
        a.fld(tbyte_ptr(rcx + 48)).unwrap();
        a.frndint().unwrap();
        a.fstp(tbyte_ptr(rcx + 128)).unwrap();
    });

    let r = run_diff_x87("frndint_pow2_carry", &code, &init);
    let rd_m80 = |off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(r.data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    assert_eq!(rd_m80(64), (0x8000_0000_0000_0000, 0x4000), "frndint(1.5) == 2.0");
    assert_eq!(
        f64::from_bits(u64::from_le_bytes(r.data[144..152].try_into().unwrap())),
        4.0,
        "2.0 + 2.0 == 4.0"
    );
    let sw = u16::from_le_bytes(r.data[200..202].try_into().unwrap());
    assert_eq!(sw & 0x0001, 0, "SW.IE must be 0 after frndint(1.5) + fadd");
    assert_eq!(rd_m80(80), (0x8000_0000_0000_0000, 0xC000), "frndint(-1.5) == -2.0");
    assert_eq!(rd_m80(96), (0x8000_0000_0000_0000, 0x4001), "frndint(3.75) == 4.0");
    assert_eq!(rd_m80(112), (0x8000_0000_0000_0000, 0x4000), "frndint(1.5+2^-60) == 2.0");
    assert_eq!(rd_m80(128), (0x8000_0000_0000_0000, 0x403E), "frndint(2^63-0.5) == 2^63");
}

/// 15. Regression: `fyl2x` and `fyl2xp1` must preserve extended 80-bit exponents outside the
/// `f64` range (e.g. `ST(0) = 2^1024, ST(1) = 1 -> 1024.0`, `2^-2000 * 3 -> -6000.0`,
/// `y = 2^5000 * log2(2^1024) -> 2^5010`, and `fyl2xp1(2^-5000, 2^5000) -> log2(e)`).
#[test]
fn x87_fyl2x_extended_exponent() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 256];
    let wr_m80 = |buf: &mut [u8], off: usize, sig: u64, se: u16| {
        buf[off..off + 8].copy_from_slice(&sig.to_le_bytes());
        buf[off + 8..off + 10].copy_from_slice(&se.to_le_bytes());
    };
    wr_m80(&mut init, 0, 0x8000_0000_0000_0000, (16383 + 1024) as u16); // 2^1024
    wr_m80(&mut init, 16, 0x8000_0000_0000_0000, (16383 - 2000) as u16); // 2^-2000
    wr_m80(&mut init, 32, 0xC000_0000_0000_0000, (16383 + 1024) as u16); // 1.5 * 2^1024
    wr_m80(&mut init, 48, 0x8000_0000_0000_0000, (16383 + 5000) as u16); // 2^5000
    wr_m80(&mut init, 64, 0x8000_0000_0000_0000, (16383 - 5000) as u16); // 2^-5000
    init[80..88].copy_from_slice(&3.0f64.to_bits().to_le_bytes());

    let code = assemble(|a| {
        a.fninit().unwrap();
        // 1 * log2(2^1024) = 1024.0
        a.fld1().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fyl2x().unwrap();
        a.fstp(qword_ptr(rcx + 96)).unwrap();

        // 3 * log2(2^-2000) = -6000.0
        a.fld(qword_ptr(rcx + 80)).unwrap();
        a.fld(tbyte_ptr(rcx + 16)).unwrap();
        a.fyl2x().unwrap();
        a.fstp(qword_ptr(rcx + 104)).unwrap();

        // 1 * log2(1.5 * 2^1024) = 1024 + log2(1.5)
        a.fld1().unwrap();
        a.fld(tbyte_ptr(rcx + 32)).unwrap();
        a.fyl2x().unwrap();
        a.fstp(qword_ptr(rcx + 112)).unwrap();

        // 2^5000 * log2(2^1024) = 2^5010 (m80: sig=0x8000..0, se=16383+5010)
        a.fld(tbyte_ptr(rcx + 48)).unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fyl2x().unwrap();
        a.fstp(tbyte_ptr(rcx + 128)).unwrap();

        // 2^5000 * log2(1 + 2^-5000) ~= log2(e)
        a.fld(tbyte_ptr(rcx + 48)).unwrap();
        a.fld(tbyte_ptr(rcx + 64)).unwrap();
        a.fyl2xp1().unwrap();
        a.fstp(qword_ptr(rcx + 144)).unwrap();
    });

    let r = run_diff_x87("fyl2x_extended_exponent", &code, &init);
    let rd_f64 = |off: usize| f64::from_bits(u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()));
    let rd_m80 = |off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(r.data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    assert_eq!(rd_f64(96), 1024.0, "1 * log2(2^1024)");
    assert_eq!(rd_f64(104), -6000.0, "3 * log2(2^-2000)");
    assert!((rd_f64(112) - (1024.0 + 1.5f64.log2())).abs() < 1e-12, "1 * log2(1.5 * 2^1024)");
    assert_eq!(rd_m80(128), (0x8000_0000_0000_0000, (16383 + 5010) as u16), "2^5000 * log2(2^1024)");
    assert!((rd_f64(144) - core::f64::consts::LOG2_E).abs() < 1e-12, "2^5000 * log2(1 + 2^-5000)");
}

/// 16. Regressions for:
///   (a) `fyl2x` just below 1.0 (`0x3FFE:0xFFFF_FFFF_FFFF_FFFF` = `1 - 2^-64`): must not cancel
///       to 0.0 (exact value ~= `-7.820865487829389e-20`).
///   (b) `fpatan` preserving ratios and quadrants across extended exponents (`2^1024` vs `2^1025`,
///       `2^-5000` vs `2^-4999`, `0.0` vs `2^-5000`, `2^-5000` vs `1.0`).
#[test]
fn x87_fyl2x_near_one_and_fpatan_extended_ratio() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 256];
    let wr_m80 = |buf: &mut [u8], off: usize, sig: u64, se: u16| {
        buf[off..off + 8].copy_from_slice(&sig.to_le_bytes());
        buf[off + 8..off + 10].copy_from_slice(&se.to_le_bytes());
    };
    wr_m80(&mut init, 0, u64::MAX, 0x3FFE);                             // 1 - 2^-64
    wr_m80(&mut init, 16, 0x8000_0000_0000_0000, (16383 + 1024) as u16); // 2^1024
    wr_m80(&mut init, 32, 0x8000_0000_0000_0000, (16383 + 1025) as u16); // 2^1025
    wr_m80(&mut init, 48, 0x8000_0000_0000_0000, (16383 - 5000) as u16); // 2^-5000
    wr_m80(&mut init, 64, 0x8000_0000_0000_0000, (16383 - 4999) as u16); // 2^-4999

    let code = assemble(|a| {
        a.fninit().unwrap();
        // (a) 1 * log2(1 - 2^-64) ~= -2^-64 * LOG2_E ~= -7.820865487829389e-20
        a.fld1().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fyl2x().unwrap();
        a.fstp(qword_ptr(rcx + 96)).unwrap();

        // (b1) atan2(2^1025, 2^1024) = atan(2) ~= 1.1071487177940905
        a.fld(tbyte_ptr(rcx + 32)).unwrap(); // ST(1) = y = 2^1025
        a.fld(tbyte_ptr(rcx + 16)).unwrap(); // ST(0) = x = 2^1024
        a.fpatan().unwrap();
        a.fstp(qword_ptr(rcx + 104)).unwrap();

        // (b2) atan2(2^-4999, 2^-5000) = atan(2) ~= 1.1071487177940905
        a.fld(tbyte_ptr(rcx + 64)).unwrap(); // ST(1) = y = 2^-4999
        a.fld(tbyte_ptr(rcx + 48)).unwrap(); // ST(0) = x = 2^-5000
        a.fpatan().unwrap();
        a.fstp(qword_ptr(rcx + 112)).unwrap();

        // (b3) atan2(2^-5000, 0.0) = +pi/2
        a.fld(tbyte_ptr(rcx + 48)).unwrap(); // ST(1) = y = 2^-5000
        a.fldz().unwrap();                   // ST(0) = x = 0.0
        a.fpatan().unwrap();
        a.fstp(qword_ptr(rcx + 120)).unwrap();

        // (b4) atan2(2^-5000, 1.0) = 2^-5000 (extended m80!)
        a.fld(tbyte_ptr(rcx + 48)).unwrap(); // ST(1) = y = 2^-5000
        a.fld1().unwrap();                   // ST(0) = x = 1.0
        a.fpatan().unwrap();
        a.fstp(tbyte_ptr(rcx + 128)).unwrap();
    });

    let r = run_diff_x87("fyl2x_near_one_fpatan_ratio", &code, &init);
    let rd_f64 = |off: usize| f64::from_bits(u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()));
    let rd_m80 = |off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(r.data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    let want_yl2x = -(2.0f64.powi(-64)) * core::f64::consts::LOG2_E;
    assert!((rd_f64(96) - want_yl2x).abs() < 1e-32, "fyl2x(1-2^-64) = {}", rd_f64(96));
    let want_atan2 = 2.0f64.atan();
    assert!((rd_f64(104) - want_atan2).abs() < 1e-15, "atan2(2^1025, 2^1024) = {}", rd_f64(104));
    assert!((rd_f64(112) - want_atan2).abs() < 1e-15, "atan2(2^-4999, 2^-5000) = {}", rd_f64(112));
    assert!((rd_f64(120) - core::f64::consts::FRAC_PI_2).abs() < 1e-15, "atan2(2^-5000, 0) = {}", rd_f64(120));
    assert_eq!(rd_m80(128), (0x8000_0000_0000_0000, (16383 - 5000) as u16), "atan2(2^-5000, 1)");
}

/// 17. Regression: memory arithmetic (`m64`/`m32`) with a signaling NaN operand must preserve
///     its sNaN classification into `select_nan` so an existing QNaN in `ST(0)` wins even when
///     the memory sNaN has a larger payload, while still raising `SW.IE` (and `m64` denormal
///     does not raise `SW.DE` when `ST(0)` is NaN).
#[test]
fn x87_mem_arith_preserves_snan_classification() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 256];
    let wr_m80 = |buf: &mut [u8], off: usize, sig: u64, se: u16| {
        buf[off..off + 8].copy_from_slice(&sig.to_le_bytes());
        buf[off + 8..off + 10].copy_from_slice(&se.to_le_bytes());
    };
    // ST(0) QNaN with small payload 0x7FFF:0xC000_0000_0000_0001
    wr_m80(&mut init, 0, 0xC000_0000_0000_0001, 0x7FFF);
    // ST(0) sNaN with small payload 0x7FFF:0x8000_0000_0000_0001
    wr_m80(&mut init, 16, 0x8000_0000_0000_0001, 0x7FFF);
    // m64 sNaN with max payload: 0x7FF7_FFFF_FFFF_FFFF
    init[32..40].copy_from_slice(&0x7FF7_FFFF_FFFF_FFFFu64.to_le_bytes());
    // m32 sNaN with max payload: 0x7FBF_FFFF
    init[40..44].copy_from_slice(&0x7FBF_FFFFu32.to_le_bytes());
    // m32 QNaN with small payload: 0x7FC0_0001
    init[44..48].copy_from_slice(&0x7FC0_0001u32.to_le_bytes());
    // m64 subnormal: 1u64
    init[48..56].copy_from_slice(&1u64.to_le_bytes());

    let code = assemble(|a| {
        // (a) QNaN(ST0) + sNaN(m64, larger payload) -> QNaN(ST0) preserved, SW.IE set
        a.fninit().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fadd(qword_ptr(rcx + 32)).unwrap();
        a.fnstsw(word_ptr(rcx + 64)).unwrap();
        a.fstp(tbyte_ptr(rcx + 80)).unwrap();

        // (b) QNaN(ST0) + sNaN(m32, larger payload) -> QNaN(ST0) preserved, SW.IE set
        a.fninit().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fadd(dword_ptr(rcx + 40)).unwrap();
        a.fnstsw(word_ptr(rcx + 66)).unwrap();
        a.fstp(tbyte_ptr(rcx + 96)).unwrap();

        // (c) sNaN(ST0) + QNaN(m32) -> QNaN(m32) wins, SW.IE set
        a.fninit().unwrap();
        a.fld(tbyte_ptr(rcx + 16)).unwrap();
        a.fadd(dword_ptr(rcx + 44)).unwrap();
        a.fnstsw(word_ptr(rcx + 68)).unwrap();
        a.fstp(tbyte_ptr(rcx + 112)).unwrap();

        // (d) QNaN(ST0) + subnormal(m64) -> QNaN(ST0), SW.DE NOT raised
        a.fninit().unwrap();
        a.fld(tbyte_ptr(rcx)).unwrap();
        a.fadd(qword_ptr(rcx + 48)).unwrap();
        a.fnstsw(word_ptr(rcx + 70)).unwrap();
        a.fstp(tbyte_ptr(rcx + 128)).unwrap();
    });

    let r = run_diff_x87("mem_arith_snan_classification", &code, &init);
    let rd_u16 = |off: usize| u16::from_le_bytes(r.data[off..off + 2].try_into().unwrap());
    let rd_m80 = |off: usize| -> (u64, u16) {
        (
            u64::from_le_bytes(r.data[off..off + 8].try_into().unwrap()),
            u16::from_le_bytes(r.data[off + 8..off + 10].try_into().unwrap()),
        )
    };
    assert_eq!(rd_m80(80), (0xC000_0000_0000_0001, 0x7FFF), "QNaN + m64 sNaN preserves ST(0)");
    assert_ne!(rd_u16(64) & 0x0001, 0, "QNaN + m64 sNaN raises SW.IE");
    assert_eq!(rd_m80(96), (0xC000_0000_0000_0001, 0x7FFF), "QNaN + m32 sNaN preserves ST(0)");
    assert_ne!(rd_u16(66) & 0x0001, 0, "QNaN + m32 sNaN raises SW.IE");
    assert_eq!(rd_m80(112), (0xC000_0100_0000_0000, 0x7FFF), "sNaN + m32 QNaN selects m32 QNaN");
    assert_ne!(rd_u16(68) & 0x0001, 0, "sNaN + m32 QNaN raises SW.IE");
    assert_eq!(rd_m80(128), (0xC000_0000_0000_0001, 0x7FFF), "QNaN + m64 denormal preserves ST(0)");
    assert_eq!(rd_u16(70) & 0x0002, 0, "QNaN + m64 denormal suppresses SW.DE");
}

/// 18. Regression: `fldcw` must normalize reserved bits (15:13, 7 -> 0) and bit 6 (-> 1)
///     identically across Generic, FastOp, JIT, and AOT (`0x023F -> 0x027F`, `0xFFFF -> 0x1F7F`,
///     `0x0000 -> 0x0040`) without clobbering EFLAGS.
#[test]
fn x87_fldcw_normalizes_reserved_bits() {
    let _guard = X87Guard::new();
    let mut init = vec![0u8; 64];
    init[0..2].copy_from_slice(&0x023Fu16.to_le_bytes());
    init[2..4].copy_from_slice(&0xFFFFu16.to_le_bytes());
    init[4..6].copy_from_slice(&0x0000u16.to_le_bytes());

    let code = assemble(|a| {
        a.fninit().unwrap();
        a.fldcw(word_ptr(rcx)).unwrap();
        a.fnstcw(word_ptr(rcx + 16)).unwrap();
        a.fldcw(word_ptr(rcx + 2)).unwrap();
        a.fnstcw(word_ptr(rcx + 18)).unwrap();
        a.fldcw(word_ptr(rcx + 4)).unwrap();
        a.fnstcw(word_ptr(rcx + 20)).unwrap();
    });

    let r = run_diff_x87("fldcw_normalize", &code, &init);
    let rd_u16 = |off: usize| u16::from_le_bytes(r.data[off..off + 2].try_into().unwrap());
    assert_eq!(rd_u16(16), 0x027F, "0x023F -> 0x027F");
    assert_eq!(rd_u16(18), 0x1F7F, "0xFFFF -> 0x1F7F");
    assert_eq!(rd_u16(20), 0x0040, "0x0000 -> 0x0040");
}
