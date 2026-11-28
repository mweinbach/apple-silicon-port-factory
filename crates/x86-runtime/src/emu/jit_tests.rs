//! Randomized differential tests: the x86->arm64 block translator must produce exactly the
//! interpreter's results (registers, flags, memory, `last_bits`-masked return value).
//!
//! Each program sets every register and the flags to random values, runs a random body of
//! translator-supported instructions mixed with generic ones (and forward `jcc`s, a loop,
//! unaligned and out-of-range memory accesses), then stores all registers and RFLAGS. It is
//! run once interpreted, then twice with the translator forced on (the first run translates
//! on first visit, the second runs fully linked code), from the same code/data addresses.

use super::*;
use iced_x86::code_asm::*;
use std::sync::Mutex;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
    fn imm32(&mut self) -> i32 {
        const I: [i32; 12] = [0, 1, -1, 2, 0x7F, 0x80, 0xFF, 0x7FFF, 0x8000, 0x7FFF_FFFF, i32::MIN, -128];
        if self.chance(50) {
            I[self.below(I.len() as u64) as usize]
        } else {
            self.next() as i32
        }
    }
    fn imm64(&mut self) -> i64 {
        const I: [i64; 8] = [0, 1, -1, 0x7FFF_FFFF, 0x8000_0000, 0xFFFF_FFFF, i64::MIN, i64::MAX];
        if self.chance(30) {
            I[self.below(I.len() as u64) as usize]
        } else {
            self.next() as i64
        }
    }
}

// Writable registers: everything except rsp, r13 (data base), r14 (bad base), r15 (loop index).
const W64: [AsmRegister64; 12] = [rax, rcx, rdx, rbx, rbp, rsi, rdi, r8, r9, r10, r11, r12];
const W32: [AsmRegister32; 12] = [eax, ecx, edx, ebx, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d];
const W16: [AsmRegister16; 12] = [ax, cx, dx, bx, bp, si, di, r8w, r9w, r10w, r11w, r12w];
const W8: [AsmRegister8; 12] = [al, cl, dl, bl, bpl, sil, dil, r8b, r9b, r10b, r11b, r12b];
const ALL64: [AsmRegister64; 15] = [rax, rcx, rdx, rbx, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15];

const DATA: usize = 0x400;
const OUT: usize = 0x300;

macro_rules! alu {
    ($a:expr, $op:expr, $d:expr, $s:expr) => {
        match $op {
            0 => $a.add($d, $s),
            1 => $a.sub($d, $s),
            2 => $a.and($d, $s),
            3 => $a.or($d, $s),
            4 => $a.xor($d, $s),
            5 => $a.cmp($d, $s),
            6 => $a.test($d, $s),
            7 => $a.adc($d, $s),
            _ => $a.sbb($d, $s),
        }
    };
}

// `test reg, mem` has no encoding: use cmp instead.
macro_rules! alu_rm {
    ($a:expr, $op:expr, $d:expr, $s:expr) => {
        match $op {
            0 => $a.add($d, $s),
            1 => $a.sub($d, $s),
            2 => $a.and($d, $s),
            3 => $a.or($d, $s),
            4 => $a.xor($d, $s),
            7 => $a.adc($d, $s),
            8 => $a.sbb($d, $s),
            _ => $a.cmp($d, $s),
        }
    };
}

macro_rules! cmov {
    ($a:expr, $cc:expr, $d:expr, $s:expr) => {
        match $cc {
            0 => $a.cmovo($d, $s),
            1 => $a.cmovno($d, $s),
            2 => $a.cmovb($d, $s),
            3 => $a.cmovae($d, $s),
            4 => $a.cmove($d, $s),
            5 => $a.cmovne($d, $s),
            6 => $a.cmovbe($d, $s),
            7 => $a.cmova($d, $s),
            8 => $a.cmovs($d, $s),
            9 => $a.cmovns($d, $s),
            10 => $a.cmovp($d, $s),
            11 => $a.cmovnp($d, $s),
            12 => $a.cmovl($d, $s),
            13 => $a.cmovge($d, $s),
            14 => $a.cmovle($d, $s),
            _ => $a.cmovg($d, $s),
        }
    };
}

macro_rules! setcc {
    ($a:expr, $cc:expr, $d:expr) => {
        match $cc {
            0 => $a.seto($d),
            1 => $a.setno($d),
            2 => $a.setb($d),
            3 => $a.setae($d),
            4 => $a.sete($d),
            5 => $a.setne($d),
            6 => $a.setbe($d),
            7 => $a.seta($d),
            8 => $a.sets($d),
            9 => $a.setns($d),
            10 => $a.setp($d),
            11 => $a.setnp($d),
            12 => $a.setl($d),
            13 => $a.setge($d),
            14 => $a.setle($d),
            _ => $a.setg($d),
        }
    };
}

macro_rules! jcc {
    ($a:expr, $cc:expr, $l:expr) => {
        match $cc {
            0 => $a.jo($l),
            1 => $a.jno($l),
            2 => $a.jb($l),
            3 => $a.jae($l),
            4 => $a.je($l),
            5 => $a.jne($l),
            6 => $a.jbe($l),
            7 => $a.ja($l),
            8 => $a.js($l),
            9 => $a.jns($l),
            10 => $a.jp($l),
            11 => $a.jnp($l),
            12 => $a.jl($l),
            13 => $a.jge($l),
            14 => $a.jle($l),
            _ => $a.jg($l),
        }
    };
}

/// Random memory operand inside the data area (`r13`-based, optionally `r15`-indexed,
/// sometimes unaligned) or, rarely, an out-of-range address (`r14`-based).
fn mem(r: &mut Rng, size: u32) -> AsmMemoryOperand {
    if r.chance(3) {
        return r14 + (r.below(64) as i32);
    }
    let align = if r.chance(25) { 1 } else { size as u64 };
    let d = ((r.below(0xC0) / align) * align) as i32;
    if r.chance(25) {
        r13 + r15 * 8 + d
    } else {
        r13 + d
    }
}

fn gen_insn(a: &mut CodeAssembler, r: &mut Rng, depth: &mut u32, stack_ok: bool) -> Result<(), IcedError> {
    let i = r.below(12) as usize;
    let j = r.below(12) as usize;
    let w = [8u32, 16, 32, 64, 32, 64][r.below(6) as usize];
    let op = r.below(9);
    match r.below(30) {
        0..=2 => match w {
            8 => a.mov(W8[i], W8[j]),
            16 => a.mov(W16[i], W16[j]),
            32 => a.mov(W32[i], W32[j]),
            _ => a.mov(W64[i], W64[j]),
        },
        3 => match w {
            8 => a.mov(W8[i], r.imm32() as i8 as i32),
            16 => a.mov(W16[i], r.imm32() as i16 as i32),
            32 => a.mov(W32[i], r.imm32() as u32),
            _ => a.mov(W64[i], r.imm64()),
        },
        4 => {
            let m = mem(r, w / 8);
            match w {
                8 => a.mov(W8[i], byte_ptr(m)),
                16 => a.mov(W16[i], word_ptr(m)),
                32 => a.mov(W32[i], dword_ptr(m)),
                _ => a.mov(W64[i], qword_ptr(m)),
            }
        }
        5 => {
            let m = mem(r, w / 8);
            match w {
                8 => a.mov(byte_ptr(m), W8[i]),
                16 => a.mov(word_ptr(m), W16[i]),
                32 => a.mov(dword_ptr(m), W32[i]),
                _ => a.mov(qword_ptr(m), W64[i]),
            }
        }
        6 => {
            let m = mem(r, w / 8);
            let v = r.imm32();
            match w {
                8 => a.mov(byte_ptr(m), v as i8 as i32),
                16 => a.mov(word_ptr(m), v as i16 as i32),
                32 => a.mov(dword_ptr(m), v),
                _ => a.mov(qword_ptr(m), v),
            }
        }
        7 => match r.below(8) {
            0 => a.movzx(W32[i], W8[j]),
            1 => a.movzx(W64[i], W16[j]),
            2 => a.movsx(W32[i], W16[j]),
            3 => a.movsx(W64[i], W8[j]),
            4 => a.movsxd(W64[i], W32[j]),
            5 => a.movzx(W16[i], W8[j]),
            6 => a.movsx(W16[i], W8[j]),
            _ => a.movzx(W64[i], W8[j]),
        },
        8 => match r.below(6) {
            0 => a.movzx(W32[i], byte_ptr(mem(r, 1))),
            1 => a.movzx(W64[i], word_ptr(mem(r, 2))),
            2 => a.movsx(W32[i], word_ptr(mem(r, 2))),
            3 => a.movsx(W64[i], byte_ptr(mem(r, 1))),
            4 => a.movsxd(W64[i], dword_ptr(mem(r, 4))),
            _ => a.movsx(W16[i], byte_ptr(mem(r, 1))),
        },
        9 => {
            let d = r.imm32() >> (r.below(31) as i32);
            let s = [1, 2, 4, 8][r.below(4) as usize];
            let m = match r.below(4) {
                0 => W64[j] + d,
                1 => W64[j] + W64[(j + 3) % 12] * s + d,
                2 => W64[j] * s + d,
                _ => W64[j] + W64[(j + 5) % 12],
            };
            if r.chance(30) {
                a.lea(W32[i], m)
            } else {
                a.lea(W64[i], m)
            }
        }
        10..=13 => match w {
            8 => alu!(a, op, W8[i], W8[j]),
            16 => alu!(a, op, W16[i], W16[j]),
            32 => alu!(a, op, W32[i], W32[j]),
            _ => alu!(a, op, W64[i], W64[j]),
        },
        14..=16 => {
            let v = r.imm32();
            match w {
                8 => alu!(a, op, W8[i], v as i8 as i32),
                16 => alu!(a, op, W16[i], v as i16 as i32),
                32 => alu!(a, op, W32[i], v),
                _ => alu!(a, op, W64[i], v),
            }
        }
        17 => {
            let m = mem(r, w / 8);
            match w {
                8 => alu_rm!(a, op, W8[i], byte_ptr(m)),
                16 => alu_rm!(a, op, W16[i], word_ptr(m)),
                32 => alu_rm!(a, op, W32[i], dword_ptr(m)),
                _ => alu_rm!(a, op, W64[i], qword_ptr(m)),
            }
        }
        18 => {
            let m = mem(r, w / 8);
            match w {
                8 => alu!(a, op, byte_ptr(m), W8[i]),
                16 => alu!(a, op, word_ptr(m), W16[i]),
                32 => alu!(a, op, dword_ptr(m), W32[i]),
                _ => alu!(a, op, qword_ptr(m), W64[i]),
            }
        }
        19 => {
            let m = mem(r, w / 8);
            let v = r.imm32();
            match w {
                8 => alu!(a, op, byte_ptr(m), v as i8 as i32),
                16 => alu!(a, op, word_ptr(m), v as i16 as i32),
                32 => alu!(a, op, dword_ptr(m), v),
                _ => alu!(a, op, qword_ptr(m), v),
            }
        }
        20 => {
            let inc = r.chance(50);
            match (w, inc) {
                (8, true) => a.inc(W8[i]),
                (8, false) => a.dec(W8[i]),
                (16, true) => a.inc(W16[i]),
                (16, false) => a.dec(W16[i]),
                (32, true) => a.inc(W32[i]),
                (32, false) => a.dec(W32[i]),
                (_, true) => a.inc(W64[i]),
                (_, false) => a.dec(W64[i]),
            }
        }
        21 | 22 => {
            let lim = if r.chance(20) { 256 } else { 34 };
            let c = r.below(lim) as i32;
            let k = r.below(3);
            macro_rules! sh {
                ($d:expr, $s:expr) => {
                    match k {
                        0 => a.shl($d, $s),
                        1 => a.shr($d, $s),
                        _ => a.sar($d, $s),
                    }
                };
            }
            if r.chance(35) {
                match w {
                    8 => sh!(W8[i], cl),
                    16 => sh!(W16[i], cl),
                    32 => sh!(W32[i], cl),
                    _ => sh!(W64[i], cl),
                }
            } else {
                match w {
                    8 => sh!(W8[i], c),
                    16 => sh!(W16[i], c),
                    32 => sh!(W32[i], c),
                    _ => sh!(W64[i], c),
                }
            }
        }
        23 => {
            let cc = r.below(16);
            if r.chance(30) {
                let m = mem(r, w / 8);
                match w {
                    8 | 16 => cmov!(a, cc, W16[i], word_ptr(m)),
                    32 => cmov!(a, cc, W32[i], dword_ptr(m)),
                    _ => cmov!(a, cc, W64[i], qword_ptr(m)),
                }
            } else {
                match w {
                    8 | 16 => cmov!(a, cc, W16[i], W16[j]),
                    32 => cmov!(a, cc, W32[i], W32[j]),
                    _ => cmov!(a, cc, W64[i], W64[j]),
                }
            }
        }
        24 => {
            if r.chance(30) {
                setcc!(a, r.below(16), byte_ptr(mem(r, 1)))
            } else {
                setcc!(a, r.below(16), W8[i])
            }
        }
        25 => {
            if !stack_ok {
                a.nop()
            } else if *depth > 0 && r.chance(50) {
                *depth -= 1;
                if r.chance(25) {
                    a.pop(qword_ptr(mem(r, 8)))
                } else {
                    a.pop(W64[i])
                }
            } else if *depth < 8 {
                *depth += 1;
                match r.below(3) {
                    0 => a.push(W64[i]),
                    1 => a.push(qword_ptr(mem(r, 8))),
                    _ => a.push(r.imm32()),
                }
            } else {
                a.nop()
            }
        }
        26 => {
            // Extended integer kinds and bit/multiply/atomic/BMI instructions.
            match r.below(58) {
                0 => a.imul_2(W32[i], W32[j]),
                1 => a.neg(W64[i]),
                2 => a.not(W32[i]),
                3 => a.bt(W64[i], W64[j]),
                4 => a.xchg(W64[i], W64[j]),
                5 => a.cdq(),
                6 => a.bsf(W32[i], W32[j]),
                7 => a.rol(W32[i], (1 + r.below(31)) as i32),
                8 => a.ror(W64[i], (1 + r.below(63)) as i32),
                9 => a.inc(dword_ptr(mem(r, 4))),
                10 => a.bswap(W32[i]),
                11 => a.bswap(W64[i]),
                12 => a.bsr(W64[i], W64[j]),
                13 => a.tzcnt(W32[i], W32[j]),
                14 => a.lzcnt(W64[i], W64[j]),
                15 => a.popcnt(W32[i], W32[j]),
                16 => a.bts(W64[i], W64[j]),
                17 => a.btr(W32[i], (r.below(32)) as i32),
                18 => a.btc(W64[i], (r.below(64)) as i32),
                19 => a.shld(W32[i], W32[j], (1 + r.below(31)) as i32),
                20 => a.shrd(W64[i], W64[j], cl),
                21 => a.rol(W16[i], cl),
                22 => a.ror(W32[i], cl),
                23 => a.mul(W64[i]),
                24 => a.imul(W32[i]),
                25 => a.imul_3(W64[i], W64[j], r.imm32()),
                26 => a.xadd(W32[i], W32[j]),
                27 => a.cmpxchg(W64[i], W64[j]),
                28 => a.cbw(),
                29 => a.cwde(),
                30 => a.cdqe(),
                31 => a.cqo(),
                32 => a.andn(W64[i], W64[j], W64[r.below(12) as usize]),
                33 => a.shlx(W32[i], W32[j], W32[r.below(12) as usize]),
                34 => a.shrx(W64[i], W64[j], W64[r.below(12) as usize]),
                35 => a.sarx(W32[i], W32[j], W32[r.below(12) as usize]),
                36 => a.rorx(W64[i], W64[j], r.below(64) as u32),
                37 => a.blsr(W32[i], W32[j]),
                38 => a.blsmsk(W64[i], W64[j]),
                39 => a.blsi(W32[i], W32[j]),
                40 => a.bzhi(W64[i], W64[j], W64[r.below(12) as usize]),
                41 => a.bextr(W32[i], W32[j], W32[r.below(12) as usize]),
                42 => a.mulx(W64[i], W64[j], W64[r.below(12) as usize]),
                43 => a.crc32(W32[i], W32[j]),
                44 => a.pdep(W64[i], W64[j], W64[r.below(12) as usize]),
                45 => a.pext(W32[i], W32[j], W32[r.below(12) as usize]),
                46 => a.rcl(W32[i], (1 + r.below(31)) as i32),
                47 => a.rcr(W64[i], cl),
                48 => a.lahf(),
                49 => a.sahf(),
                50 => a.add(ah, bh),
                51 => a.xor(ch, W8[i & 3]),
                52 => a.movzx(W32[i & 3], dh),
                53 => a.lock().add(dword_ptr(mem(r, 4)), W32[i]),
                54 => a.lock().xadd(qword_ptr(mem(r, 8)), W64[i]),
                55 => a.lock().cmpxchg(dword_ptr(mem(r, 4)), W32[i]),
                56 => a.mov(W64[i], qword_ptr(mem(r, 8)).gs()),
                _ => {
                    // 128/64-bit unsigned division (`rdx = 3`, divisor >= 0x10)
                    a.mov(rdx, 3u64)?;
                    a.or(r11, 0x10)?;
                    a.div(r11)
                }
            }
        }
        27 => match r.below(5) {
            0 => a.clc(),
            1 => a.stc(),
            2 => a.cmc(),
            3 => a.adcx(W64[i], W64[j]),
            _ => a.adox(W32[i], W32[j]),
        },
        28 | 29 => {
            const XMM: [AsmRegisterXmm; 4] = [xmm0, xmm1, xmm2, xmm3];
            const YMM: [AsmRegisterYmm; 4] = [ymm0, ymm1, ymm2, ymm3];
            let x = XMM[i & 3];
            let y = XMM[j & 3];
            let z_idx = r.below(4) as usize;
            let z = XMM[z_idx];
            let yx = YMM[i & 3];
            let yy = YMM[j & 3];
            let yz = YMM[z_idx];
            match r.below(58) {
                0 => a.movdqu(x, y),
                1 => a.movdqu(x, xmmword_ptr(mem(r, 16))),
                2 => a.movdqu(xmmword_ptr(mem(r, 16)), x),
                3 => a.pxor(x, y),
                4 => a.por(x, y),
                5 => a.pand(x, y),
                6 => a.paddd(x, y),
                7 => a.psubw(x, y),
                8 => a.movd(x, W32[i]),
                9 => a.movd(W32[i], x),
                10 => a.movss(x, y),
                11 => a.movq(x, y),
                12 => a.pandn(x, y),
                13 => a.punpckldq(x, y),
                14 => a.punpckhqdq(x, y),
                15 => a.pshufd(x, y, r.below(256) as i32),
                16 => a.pshufb(x, y),
                17 => a.pmullw(x, y),
                18 => a.pmaddwd(x, y),
                19 => a.pminub(x, y),
                20 => a.pavgb(x, y),
                21 => a.pcmpeqd(x, y),
                22 => a.cvtsi2ss(x, W32[i]),
                23 => a.ucomiss(x, y),
                24 => a.palignr(x, y, r.below(36) as i32),
                25 => a.pblendw(x, y, r.below(256) as i32),
                26 => a.blendps(x, y, r.below(16) as i32),
                27 => a.insertps(x, y, r.below(256) as i32),
                28 => a.movshdup(x, y),
                29 => a.movddup(x, y),
                30 => a.psadbw(x, y),
                31 => a.vpsllvd(x, y, z),
                32 => a.vpsravd(x, y, z),
                33 => a.vpbroadcastd(x, y),
                34 => a.pmovsxbw(x, y),
                35 => a.pmovzxwd(x, y),
                36 => a.vpblendvb(x, y, z, XMM[0]),
                37 => a.ptest(x, y),
                38 => a.pclmulqdq(x, y, r.below(32) as i32),
                39 => a.roundps(x, y, r.below(4) as i32),
                40 => a.cmpps(x, y, r.below(16) as i32),
                41 => a.cmpss(x, y, r.below(16) as i32),
                42 => a.addsubps(x, y),
                43 => a.haddps(x, y),
                44 => a.hsubpd(x, y),
                45 => a.cvtdq2pd(x, y),
                46 => {
                    if r.chance(50) {
                        a.cvttpd2dq(x, y)
                    } else {
                        a.cvtpd2dq(x, y)
                    }
                }
                47 => a.vfmadd213ps(x, y, z),
                48 => a.vfnmadd231sd(x, y, z),
                49 => a.vpermq(yx, yy, r.below(256) as u32),
                50 => a.vpermd(yx, yy, yz),
                51 => a.vinsertf128(yx, yy, z, r.below(2) as u32),
                52 => a.aesenc(x, y),
                53 => a.aesdec(x, y),
                54 => a.aesimc(x, y),
                55 => a.aeskeygenassist(x, y, r.below(256) as i32),
                56 => a.pcmpistri(x, y, r.below(128) as i32),
                _ => a.pcmpistrm(x, y, r.below(128) as i32),
            }
        }
        _ => a.nop(),
    }
}

fn gen_program(r: &mut Rng) -> Result<Vec<u8>, IcedError> {
    let mut a = CodeAssembler::new(64)?;
    a.mov(r13, rcx)?;
    for (k, &reg) in W64.iter().enumerate() {
        if k % 4 == 0 && r.chance(30) {
            a.mov(reg, r.imm32() as i64)?;
        } else {
            a.mov(reg, r.imm64())?;
        }
    }
    a.movdqu(xmm0, xmmword_ptr(r13))?;
    a.movdqu(xmm1, xmmword_ptr(r13 + 16))?;
    a.movdqu(xmm2, xmmword_ptr(r13 + 32))?;
    a.movdqu(xmm3, xmmword_ptr(r13 + 48))?;
    a.mov(r14, 0x1000 + r.below(0x8000) as i64)?;
    // Random CF/PF/ZF/SF/OF.
    let fl = (r.next() as u32) & 0x8C5;
    a.push(fl as i32)?;
    a.popfq()?;
    let iters = 1 + r.below(4) as i32;
    a.mov(r15d, iters)?;
    let mut top = a.create_label();
    a.set_label(&mut top)?;
    a.nop()?;
    let mut depth = 0u32;
    let n = 4 + r.below(40);
    let mut pending: Vec<(CodeLabel, u64)> = Vec::new();
    for _ in 0..n {
        if r.chance(10) {
            let mut l = a.create_label();
            jcc!(a, r.below(16), l)?;
            pending.push((l, 1 + r.below(3)));
            let _ = &mut l;
        }
        gen_insn(&mut a, r, &mut depth, pending.is_empty())?;
        for p in pending.iter_mut() {
            p.1 -= 1;
        }
        let mut k = 0;
        while k < pending.len() {
            if pending[k].1 == 0 {
                let (mut l, _) = pending.remove(k);
                a.set_label(&mut l)?;
                a.nop()?;
            } else {
                k += 1;
            }
        }
    }
    for (mut l, _) in pending.drain(..) {
        a.set_label(&mut l)?;
        a.nop()?;
    }
    while depth > 0 {
        a.pop(r12)?;
        depth -= 1;
    }
    a.dec(r15d)?;
    a.jnz(top)?;
    a.movdqu(xmmword_ptr(r13 + 0x200), xmm0)?;
    a.movdqu(xmmword_ptr(r13 + 0x210), xmm1)?;
    a.movdqu(xmmword_ptr(r13 + 0x220), xmm2)?;
    a.movdqu(xmmword_ptr(r13 + 0x230), xmm3)?;
    for (k, &reg) in ALL64.iter().enumerate() {
        a.mov(qword_ptr(r13 + (OUT + 8 * k) as i32), reg)?;
    }
    a.pushfq()?;
    a.pop(rdx)?;
    a.mov(qword_ptr(r13 + (OUT + 8 * 15) as i32), rdx)?;
    a.ret()?;
    a.assemble(0)
}

static ARENA: Mutex<(usize, usize)> = Mutex::new((0, 0));
const ARENA_SIZE: usize = 4 << 20;

pub(super) fn arena_alloc(len: usize, align: usize) -> *mut u8 {
    let mut a = ARENA.lock().unwrap();
    loop {
        if a.0 != 0 {
            let start = (a.0 + a.1 + align - 1) & !(align - 1);
            if start + len <= a.0 + ARENA_SIZE {
                a.1 = start + len - a.0;
                return start as *mut u8;
            }
        }
        let m: &'static mut [u8] = Box::leak(vec![0u8; ARENA_SIZE].into_boxed_slice());
        let p = m.as_mut_ptr() as u64;
        if p + ARENA_SIZE as u64 > 0x1_4000_0000 && p < 0x1_4500_0000 {
            continue;
        }
        *a = (p as usize, 0);
    }
}

fn disasm(code: &[u8], ip: u64) -> String {
    use iced_x86::{Formatter, IntelFormatter};
    let mut d = Decoder::with_ip(64, code, ip, DecoderOptions::NONE);
    let mut f = IntelFormatter::new();
    let mut s = String::new();
    for ins in &mut d {
        let mut t = String::new();
        f.format(&ins, &mut t);
        s += &format!("  {:#x}: {}\n", ins.ip(), t);
    }
    s
}

/// Run `code` at `c` with data at `d` (initialized from `init`); returns (rax, data).
unsafe fn run_mode(c: *mut u8, d: *mut u8, init: &[u8], mode: u8) -> (u64, Vec<u8>) {
    core::ptr::copy_nonoverlapping(init.as_ptr(), d, init.len());
    jit::force(mode);
    let (ret, _) = emu_run_full(c as u64, [d as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
    jit::force(0);
    (ret, core::slice::from_raw_parts(d, init.len()).to_vec())
}

fn fuzz(seed: u64, programs: usize) {
    let mut r = Rng(seed | 1);
    let jit_before = jit::test_executed();
    for p in 0..programs {
        let code = gen_program(&mut r).expect("assemble");
        let c = arena_alloc(code.len() + 16, 16);
        let d = arena_alloc(DATA, 64);
        let mut init = vec![0u8; DATA];
        for b in init.iter_mut().take(0x100) {
            *b = r.next() as u8;
        }
        unsafe {
            core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
            core::ptr::write_bytes(c.add(code.len()), 0xCC, 16);
            if let Some(path) = std::env::var_os("AW2_JIT_FUZZ_TRACE") {
                let _ = std::fs::write(path, format!("seed {seed:#x} program {p}\n{}", disasm(&code, c as u64)));
            }
            let interp = run_mode(c, d, &init, 2);
            let jit1 = run_mode(c, d, &init, 1);
            let jit2 = run_mode(c, d, &init, 1);
            for (name, got) in [("first", &jit1), ("linked", &jit2)] {
                if *got != interp {
                    let mut msg = format!("seed {seed:#x} program {p}: translator ({name} run) differs from interpreter\n");
                    if got.0 != interp.0 {
                        msg += &format!("  return value: jit {:#x} interp {:#x}\n", got.0, interp.0);
                    }
                    for k in 0..DATA / 8 {
                        let a = u64::from_le_bytes(got.1[8 * k..8 * k + 8].try_into().unwrap());
                        let b = u64::from_le_bytes(interp.1[8 * k..8 * k + 8].try_into().unwrap());
                        if a != b {
                            let what = if k * 8 >= OUT && k * 8 < OUT + 8 * 15 {
                                format!("reg {:?}", ALL64[(k * 8 - OUT) / 8])
                            } else if k * 8 == OUT + 8 * 15 {
                                "rflags".to_string()
                            } else {
                                format!("data+{:#x}", k * 8)
                            };
                            msg += &format!("  {what}: jit {a:#018x} interp {b:#018x}\n");
                        }
                    }
                    msg += &disasm(&code, c as u64);
                    panic!("{msg}");
                }
            }
        }
    }
    let ran = jit::test_executed() - jit_before;
    eprintln!("seed {seed:#x}: {ran} guest instructions executed in translated code");
    assert!(ran >= 20 * programs as u64, "translator barely ran ({ran} instructions)");
}

#[test]
fn translator_matches_interpreter_random() {
    fuzz(0x5EED_1234_ABCD_0001, 1500);
}

#[test]
fn translator_matches_interpreter_random_2() {
    fuzz(0x0BAD_C0DE_F00D_0002, 1500);
}

// --- host calls / tail jumps made from translated code -------------------------------------

/// Generic 8-argument host fn. Also defines `q0` (= the result): the host-call paths pass the
/// raw `q0` back as guest `xmm0`, and an integer-only Rust fn leaves `q0` unspecified. At
/// opt-level 0 (test profile) this body copies its `[u64; 8]` through vector registers and
/// leaves `q0` = `(a, b)` or `(c, d)` depending on the host `sp` mod 32 at the call, which
/// differs between the interpreter's and the translator's host-call paths.
unsafe extern "C" fn host_mix8(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64) -> u64 {
    let mut x = 0xcbf2_9ce4_8422_2325u64;
    for v in [a, b, c, d, e, f, g, h] {
        x = (x ^ v).wrapping_mul(0x100_0000_01b3);
    }
    #[cfg(target_arch = "aarch64")]
    core::arch::asm!("fmov d0, {x}", x = in(reg) x, out("v0") _, options(nomem, nostack, preserves_flags));
    x
}

/// Re-enters the emulator (nested `emu_run_full` on the same thread / translation cache).
unsafe extern "C" fn host_reenter(g: u64, x: u64) -> u64 {
    emu_run_full(g, [x, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0.wrapping_add(1)
}

/// The host-call / tail-jump program of `translator_host_calls_match_interpreter` at `c`
/// (16-byte slot, code at +16) calling the 6 registered host VAs `h`.
fn gen_host_calls(c: u64, h: [u64; 6]) -> Result<Vec<u8>, IcedError> {
    let [h_memset, h_mix, h_reent, h_ceil, h_malloc, h_free] = h;
    let mut a = CodeAssembler::new(64)?;
    let mut slot = a.create_label();
    let mut nested = a.create_label();
    let mut t1 = a.create_label();
    let mut t2 = a.create_label();
    let mut t3 = a.create_label();
    let mut t4 = a.create_label();
    let mut t5 = a.create_label();
    // 16-byte aligned import-style slot holding the plain builtin; code starts at +16.
    a.set_label(&mut slot)?;
    a.dq(&[h_memset, 0])?;
    a.push(rbx)?;
    a.sub(rsp, 0x40)?;
    a.mov(rbx, rcx)?;
    for (k, &r) in [rbp, rsi, rdi, r10, r11, r12, r13, r14, r15].iter().enumerate() {
        a.mov(r, 0x0101_0101_0101_0101i64.wrapping_mul(k as i64 + 3))?;
    }
    for k in 0..4 {
        a.mov(qword_ptr(rsp + 0x20 + 8 * k), 5 + k)?;
    }
    // Plain builtin via call rel32 (8-bit masked rdx).
    a.mov(rdx, -1i64)?;
    a.mov(dl, 0x5A)?;
    a.lea(rcx, ptr(rbx + 0x100))?;
    a.mov(r8d, 0x40)?;
    a.call(h_memset)?;
    a.mov(qword_ptr(rbx + 0x200), rax)?;
    a.movdqu(xmmword_ptr(rbx + 0x3C0), xmm0)?;
    // Float-ABI inline variant: ceil(xmm0).
    a.mov(rax, 0x4002_0000_0000_0000i64)?;
    a.movq(xmm0, rax)?;
    a.call(h_ceil)?;
    a.movq(qword_ptr(rbx + 0x258), xmm0)?;
    // Bail-out variant (malloc_try) and a plain free; the pointer itself differs per run.
    a.mov(ecx, 0x30)?;
    a.call(h_malloc)?;
    a.test(rax, rax)?;
    a.setne(byte_ptr(rbx + 0x260))?;
    a.mov(rcx, rax)?;
    a.call(h_free)?;
    // Generic host fn with 8 args.
    a.mov(rcx, 1i64)?;
    a.mov(rdx, 2i64)?;
    a.mov(r8, 3i64)?;
    a.mov(r9, 4i64)?;
    a.call(h_mix)?;
    a.mov(qword_ptr(rbx + 0x208), rax)?;
    // Partial-register args (abi_gpr masking).
    a.mov(rcx, -1i64)?;
    a.mov(cl, 7)?;
    a.mov(rdx, -1i64)?;
    a.mov(dx, 0x1234)?;
    a.mov(r8, -1i64)?;
    a.mov(r9, -1i64)?;
    a.mov(r9b, 9)?;
    a.call(h_mix)?;
    a.mov(qword_ptr(rbx + 0x210), rax)?;
    // Plain builtin via call [rip+slot].
    a.lea(rcx, ptr(rbx + 0x140))?;
    a.mov(edx, 0x33)?;
    a.mov(r8d, 0x20)?;
    a.call(qword_ptr(slot))?;
    a.mov(qword_ptr(rbx + 0x218), rax)?;
    // Host fn that re-enters guest code.
    a.lea(rcx, ptr(nested))?;
    a.mov(edx, 14)?;
    a.call(h_reent)?;
    a.mov(qword_ptr(rbx + 0x220), rax)?;
    // Tail jumps.
    a.call(t1)?;
    a.mov(qword_ptr(rbx + 0x228), rax)?;
    a.call(t2)?;
    a.mov(qword_ptr(rbx + 0x230), rax)?;
    a.call(t3)?;
    a.mov(qword_ptr(rbx + 0x238), rax)?;
    // Register-indirect calls.
    a.mov(rax, h_mix)?;
    a.mov(rcx, 9i64)?;
    a.call(rax)?;
    a.mov(qword_ptr(rbx + 0x240), rax)?;
    a.mov(rax, h_memset)?;
    a.lea(rcx, ptr(rbx + 0x1C0))?;
    a.mov(edx, 0x11)?;
    a.mov(r8d, 8)?;
    a.call(rax)?;
    a.mov(qword_ptr(rbx + 0x248), rax)?;
    a.call(t4)?;
    a.mov(qword_ptr(rbx + 0x250), rax)?;
    a.movdqu(xmmword_ptr(rbx + 0x3D0), xmm0)?;
    // Indirect float-ABI inline variant (host_xfer's plain fast path): ceil via call rax.
    a.mov(rax, 0x4004_cccc_cccc_cccdi64)?;
    a.movq(xmm0, rax)?;
    a.mov(rax, h_ceil)?;
    a.call(rax)?;
    a.movq(qword_ptr(rbx + 0x268), xmm0)?;
    // Plain builtin via jmp reg (fast path, tail).
    a.call(t5)?;
    a.mov(qword_ptr(rbx + 0x270), rax)?;
    for (k, &r) in [rbp, rsi, rdi, r10, r11, r12, r13, r14, r15].iter().enumerate() {
        a.mov(qword_ptr(rbx + (0x300 + 8 * k) as i32), r)?;
    }
    a.add(rsp, 0x40)?;
    a.pop(rbx)?;
    a.ret()?;
    // Nested guest function run by host_reenter.
    a.set_label(&mut nested)?;
    a.imul_3(rax, rcx, 3)?;
    a.ret()?;
    // Generic host fn via jmp rel32 (stack args are the caller's [rsp+0x20..]).
    a.set_label(&mut t1)?;
    a.mov(rcx, 11i64)?;
    a.mov(rdx, 22i64)?;
    a.mov(r8, 33i64)?;
    a.mov(r9, 44i64)?;
    a.jmp(h_mix)?;
    // Plain builtin via jmp [rip+slot].
    a.set_label(&mut t2)?;
    a.lea(rcx, ptr(rbx + 0x180))?;
    a.mov(edx, 0x77)?;
    a.mov(r8d, 0x10)?;
    a.jmp(qword_ptr(slot))?;
    // Plain builtin via jmp rel32.
    a.set_label(&mut t3)?;
    a.lea(rcx, ptr(rbx + 0x1A0))?;
    a.mov(edx, 0x66)?;
    a.mov(r8d, 0x18)?;
    a.jmp(h_memset)?;
    // Generic host fn via jmp reg.
    a.set_label(&mut t4)?;
    a.mov(rax, h_mix)?;
    a.mov(rcx, 0x55i64)?;
    a.jmp(rax)?;
    // Plain builtin via jmp reg.
    a.set_label(&mut t5)?;
    a.lea(rcx, ptr(rbx + 0x1E0))?;
    a.mov(edx, 0x44)?;
    a.mov(r8d, 0x10)?;
    a.mov(rax, h_memset)?;
    a.jmp(rax)?;
    a.assemble(c)
}

/// `call rel32` / `call [rip+slot]` / `call reg` / `jmp rel32` / `jmp [rip+slot]` / `jmp reg`
/// to registered host functions (a `DF_PLAIN` builtin, a generic 8-argument one and one that
/// re-enters guest code): the translator (`host_xfer` stub and inline plain calls) must match
/// the interpreter, including `last_bits`-masked arguments, `xmm0` and preserved registers.
#[test]
fn translator_host_calls_match_interpreter() {
    let h = arena_alloc(128, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 128) };
    let h_memset = h as u64;
    let h_mix = h_memset + 16;
    let h_reent = h_memset + 32;
    let h_ceil = h_memset + 48;
    let h_malloc = h_memset + 64;
    let h_free = h_memset + 80;
    let natives = [
        (h_memset, win32_rt::msvcrt::memset as *const () as usize),
        (h_mix, host_mix8 as *const () as usize),
        (h_reent, host_reenter as *const () as usize),
        (h_ceil, win32_rt::ucrt::math::ceil as *const () as usize),
        (h_malloc, win32_rt::ucrt::heap::malloc as *const () as usize),
        (h_free, win32_rt::ucrt::heap::free as *const () as usize),
    ];
    for &(va, f) in &natives {
        crate::exe::register_native_va(va, f);
    }

    let c = arena_alloc(0x1000, 16);
    let code = gen_host_calls(c as u64, [h_memset, h_mix, h_reent, h_ceil, h_malloc, h_free]).expect("assemble");
    assert!(code.len() <= 0x1000);
    let d = arena_alloc(DATA, 64);
    let mut init = vec![0u8; DATA];
    let mut r = Rng(0x7357_4057_CA11_0001);
    for b in init.iter_mut() {
        *b = r.next() as u8;
    }
    let jit_before = jit::test_executed();
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        let entry = c.add(16);
        let interp = run_mode(entry, d, &init, 2);
        let jit1 = run_mode(entry, d, &init, 1);
        let jit2 = run_mode(entry, d, &init, 1);
        for &(va, _) in &natives {
            crate::exe::unregister_native_va(va);
        }
        for (name, got) in [("first", &jit1), ("linked", &jit2)] {
            assert_eq!(got.0, interp.0, "{name} run: return value");
            for k in 0..DATA / 8 {
                let a = u64::from_le_bytes(got.1[8 * k..8 * k + 8].try_into().unwrap());
                let b = u64::from_le_bytes(interp.1[8 * k..8 * k + 8].try_into().unwrap());
                assert_eq!(a, b, "{name} run: data+{:#x}\n{}", 8 * k, disasm(&code[16..], c as u64 + 16));
            }
        }
    }
    let ran = jit::test_executed() - jit_before;
    assert!(ran >= 100, "translator barely ran ({ran} instructions)");
}

// --- guest -> native call: return-address slot ---------------------------------------------

/// Returns the guest's `[rsp-8]` at the call: the incoming rsp is the caller's `rsp` (nothing
/// is pushed for a host call), so a real `call`'s return address would be just below it.
unsafe extern "C" fn host_ret_slot() -> u64 {
    let caller_rsp = crate::callctx::ctx().incoming_rsp.get().0;
    if caller_rsp == 0 {
        return 1;
    }
    core::ptr::read((caller_rsp - 8) as *const u64)
}

/// A guest `call` to a host function stores the return address at `[rsp-8]` without moving
/// `rsp`, through `call reg` and `call rel32`, interpreted and translated (V8's `ExitFrame`
/// reads its pc from that slot).
#[test]
fn host_calls_store_the_return_address_below_rsp() {
    let h = arena_alloc(16, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 16) };
    let hv = h as u64;
    crate::exe::register_native_va(hv, host_ret_slot as *const () as usize);
    let c = arena_alloc(0x200, 16);
    let mut a = CodeAssembler::new(64).unwrap();
    let mut l1 = a.create_label();
    let mut l2 = a.create_label();
    a.push(rbx).unwrap();
    a.sub(rsp, 0x20).unwrap();
    a.mov(rbx, rcx).unwrap();
    a.mov(qword_ptr(rsp - 8), 0xBAD).unwrap();
    a.mov(rax, hv).unwrap();
    a.call(rax).unwrap();
    a.set_label(&mut l1).unwrap();
    a.mov(qword_ptr(rbx), rax).unwrap();
    a.lea(rax, ptr(l1)).unwrap();
    a.mov(qword_ptr(rbx + 8), rax).unwrap();
    a.mov(qword_ptr(rsp - 8), 0xBAD).unwrap();
    a.call(hv).unwrap();
    a.set_label(&mut l2).unwrap();
    a.mov(qword_ptr(rbx + 16), rax).unwrap();
    a.lea(rax, ptr(l2)).unwrap();
    a.mov(qword_ptr(rbx + 24), rax).unwrap();
    a.mov(qword_ptr(rbx + 32), rsp).unwrap();
    a.add(rsp, 0x20).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    let code = a.assemble(c as u64).expect("assemble");
    let d = arena_alloc(64, 16);
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        for (mode, name) in [(2u8, "interpreter"), (1u8, "translator")] {
            core::ptr::write_bytes(d, 0, 64);
            jit::force(mode);
            emu_run_full(c as u64, [d as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            jit::force(0);
            let q = |k: usize| core::ptr::read((d as *const u64).add(k));
            assert_eq!(q(0), q(1), "{name}: call reg: [rsp-8] = {:#x}, return address {:#x}", q(0), q(1));
            assert_eq!(q(2), q(3), "{name}: call rel32: [rsp-8] = {:#x}, return address {:#x}", q(2), q(3));
            assert_ne!(q(4), 0, "{name}: did not run");
        }
    }
    crate::exe::unregister_native_va(hv);
}

// --- call/ret: shadow RAS, flags dead at call/ret, read-only plain loads --------------------

const CR_DATA: usize = 0x1000;
const CR_TBL: i32 = 0x800;
const CR_SAVE: i32 = 0x900;
/// Bytes of read-only constant pool at the start of each call/ret program (marked read-only
/// for `AW2_JIT_RO_PLAIN` with `jit::test_set_ro`); 8 labels x 32 bytes.
const CR_POOL: usize = 0x100;
/// Offset of the host stub (`host_reenter`) inside each program's 0x1000-byte code area.
const CR_HOST: usize = 0xFF0;

/// After every call site: write all of CF/ZF/SF/OF/PF (flags are undefined after a call
/// under `AW2_JIT_FLAGS_DEAD_AT_CALL`), then maybe consume them.
fn cr_flags_after(a: &mut CodeAssembler, r: &mut Rng) -> Result<(), IcedError> {
    match r.below(4) {
        0 => a.add(rsi, rax)?,
        1 => a.cmp(rax, rbx)?,
        2 => a.sub(rdi, rsi)?,
        _ => a.add(rbx, rax)?,
    }
    let cc = r.below(16);
    match r.below(4) {
        0 => a.adc(rdi, 1),
        1 => setcc!(a, cc, byte_ptr(r13 + 0x40 + r.below(16) as i32)),
        2 => cmov!(a, cc, rbx, rsi),
        _ => Ok(()),
    }
}

/// A load (rarely a store) through a rip-relative pool operand.
fn cr_pool_op(a: &mut CodeAssembler, r: &mut Rng, pool: &[CodeLabel]) -> Result<(), IcedError> {
    let l = pool[r.below(pool.len() as u64) as usize];
    match r.below(12) {
        0 => a.mov(r8, qword_ptr(l)),
        1 => a.movzx(r9d, byte_ptr(l)),
        2 => a.add(r10, qword_ptr(l)),
        3 => a.cmp(dword_ptr(l), 0x1234),
        4 => a.movdqu(xmm0, xmmword_ptr(l)),
        5 => a.paddd(xmm1, xmmword_ptr(l)),
        6 => a.pxor(xmm2, xmmword_ptr(l)),
        7 => a.vpaddd(ymm3, ymm3, ymmword_ptr(l)),
        8 => a.movq(xmm3, qword_ptr(l)),
        9 => a.movd(xmm1, dword_ptr(l)),
        10 => a.imul_2(r11, qword_ptr(l)),
        // Store into "read-only" data (never expected in real code; must still be exact).
        _ => a.mov(dword_ptr(l), eax),
    }
}

/// Random call/ret program assembled at `base`: returns (code, entry offset). Covers direct,
/// register- and memory-indirect calls (incl. `call [rip+slot]`), `ret imm`, tail jumps,
/// return-address rewrites, frame-skipping returns, longjmp-style unwinds 3 and 5 frames
/// deep, recursion deeper than the RAS ring, and a host function re-entering guest code.
fn gen_callret(r: &mut Rng, base: u64, h_reent: u64) -> Result<(Vec<u8>, usize), IcedError> {
    let mut a = CodeAssembler::new(64)?;
    let mut pool: Vec<CodeLabel> = Vec::new();
    for _ in 0..7 {
        let mut l = a.create_label();
        a.set_label(&mut l)?;
        let q: Vec<u64> = (0..4).map(|_| r.next()).collect();
        a.dq(&q)?;
        pool.push(l);
    }
    // Import-style slot (patched with the address of leaf 0 after assembly).
    let mut slot = a.create_label();
    a.set_label(&mut slot)?;
    a.dq(&[0, 0, 0, 0])?;

    let mut entry = a.create_label();
    let mut leaves: Vec<CodeLabel> = (0..4).map(|_| a.create_label()).collect();
    let mut f_nest = a.create_label();
    let mut f_tail = a.create_label();
    let mut f_retn = a.create_label();
    let mut f_rw = a.create_label();
    let mut f_lj3 = [a.create_label(), a.create_label()];
    let mut f_lj5 = [a.create_label(), a.create_label(), a.create_label(), a.create_label()];
    let mut f_ljx = a.create_label();
    let mut f_rec = a.create_label();
    let mut f_mid = a.create_label();
    let mut f_skip = a.create_label();
    let mut f_host_g = a.create_label();

    // --- entry ---
    a.set_label(&mut entry)?;
    a.sub(rsp, 0x48)?;
    a.mov(r13, rcx)?;
    for &reg in &[rax, rbx, rsi, rdi, r8, r9, r10, r11, r12] {
        a.mov(reg, r.imm64())?;
    }
    a.movdqu(xmm0, xmmword_ptr(r13))?;
    a.movdqu(xmm1, xmmword_ptr(r13 + 16))?;
    a.movdqu(xmm2, xmmword_ptr(r13 + 32))?;
    a.movdqu(xmm3, xmmword_ptr(r13 + 48))?;
    let tbl = [leaves[0], leaves[1], leaves[2], leaves[3], f_nest, f_tail];
    for (k, &f) in tbl.iter().enumerate() {
        a.lea(r12, ptr(f))?;
        a.mov(qword_ptr(r13 + CR_TBL + 8 * k as i32), r12)?;
    }
    let fl = (r.next() as u32) & 0x8C5;
    a.push(fl as i32)?;
    a.popfq()?;
    a.mov(r15d, 1 + r.below(3) as u32)?;
    let mut top = a.create_label();
    a.set_label(&mut top)?;
    a.nop()?;
    for _ in 0..6 + r.below(14) {
        if r.chance(30) {
            cr_pool_op(&mut a, r, &pool)?;
        }
        if r.chance(30) {
            // Flags written right before the call: dead there.
            a.cmp(rsi, rdi)?;
        }
        match r.below(12) {
            0 | 1 => a.call(leaves[r.below(4) as usize])?,
            2 => {
                a.lea(r11, ptr(tbl[r.below(tbl.len() as u64) as usize]))?;
                a.call(r11)?;
            }
            3 => a.call(qword_ptr(r13 + CR_TBL + 8 * r.below(tbl.len() as u64) as i32))?,
            4 => a.call(qword_ptr(slot))?,
            5 => {
                a.push(r.imm32())?;
                a.call(f_retn)?;
            }
            6 => {
                // f_rw returns 5 bytes past its return address, skipping this mov.
                a.call(f_rw)?;
                a.mov(eax, 0xDEAD_0001u32)?;
            }
            7 => {
                a.mov(qword_ptr(r13 + CR_SAVE), rsp)?;
                a.call(if r.chance(50) { f_lj3[0] } else { f_lj5[0] })?;
            }
            8 => {
                let depth = match r.below(10) {
                    0..=5 => 1 + r.below(8) as u32,
                    6 | 7 => 50,
                    8 => 4200,
                    _ => 4500,
                };
                a.mov(ebp, depth)?;
                a.call(f_rec)?;
            }
            9 => {
                a.lea(rcx, ptr(f_host_g))?;
                a.mov(rdx, rsi)?;
                a.call(h_reent)?;
                // The host fn returns an integer: guest xmm0 then holds whatever the host left
                // in v0 (differs between the interpreter's and the translator's nested run).
                a.xorps(xmm0, xmm0)?;
            }
            10 => a.call(f_mid)?,
            _ => a.call(f_tail)?,
        }
        cr_flags_after(&mut a, r)?;
    }
    a.sub(r15d, 1)?;
    a.jnz(top)?;
    a.movdqu(xmmword_ptr(r13 + 0x200), xmm0)?;
    a.movdqu(xmmword_ptr(r13 + 0x210), xmm1)?;
    a.movdqu(xmmword_ptr(r13 + 0x220), xmm2)?;
    a.movdqu(xmmword_ptr(r13 + 0x230), xmm3)?;
    for (k, &reg) in ALL64.iter().enumerate() {
        a.mov(qword_ptr(r13 + (OUT + 8 * k) as i32), reg)?;
    }
    a.pushfq()?;
    a.pop(rdx)?;
    a.mov(qword_ptr(r13 + (OUT + 8 * 15) as i32), rdx)?;
    a.add(rsp, 0x48)?;
    a.ret()?;

    // --- functions (each writes all flags before reading any) ---
    for k in 0..4 {
        a.set_label(&mut leaves[k])?;
        a.add(rax, r.imm32())?;
        match k {
            1 => {
                a.cmp(rax, rbx)?;
                a.cmovl(rax, rsi)?;
                a.adc(rdi, 0)?;
            }
            3 => {
                a.mov(r12, qword_ptr(pool[3]))?;
                a.add(rax, r12)?;
            }
            _ => {}
        }
        a.lea(rsi, ptr(rsi + rax * 2 + k as i32))?;
        if k == 2 && r.chance(50) {
            a.ret_1(0)?;
        } else {
            a.ret()?;
        }
    }
    a.set_label(&mut f_nest)?;
    a.sub(rsp, 0x28)?;
    a.add(rdi, 3)?;
    a.call(leaves[0])?;
    a.add(rsi, rax)?;
    a.call(qword_ptr(r13 + CR_TBL + 8))?;
    a.xor(rbx, rax)?;
    a.add(rsp, 0x28)?;
    a.ret()?;

    a.set_label(&mut f_tail)?;
    a.add(rdi, 2)?;
    a.jmp(leaves[2])?;

    a.set_label(&mut f_retn)?;
    a.add(rax, qword_ptr(rsp + 8))?;
    a.ret_1(8)?;

    a.set_label(&mut f_rw)?;
    a.add(qword_ptr(rsp), 5)?;
    a.ret()?;

    // longjmp-style: the innermost frame restores the rsp saved at the call site.
    a.set_label(&mut f_lj3[0])?;
    a.add(rbx, 7)?;
    a.call(f_lj3[1])?;
    a.mov(eax, 0xDEAD_0002u32)?;
    a.ret()?;
    a.set_label(&mut f_lj3[1])?;
    a.add(rbx, 9)?;
    a.call(f_ljx)?;
    a.mov(eax, 0xDEAD_0003u32)?;
    a.ret()?;
    for k in 0..4 {
        a.set_label(&mut f_lj5[k])?;
        a.add(rdi, 13 + k as i32)?;
        a.call(if k < 3 { f_lj5[k + 1] } else { f_ljx })?;
        a.mov(eax, 0xDEAD_0010u32 + k as u32)?;
        a.ret()?;
    }
    a.set_label(&mut f_ljx)?;
    a.add(rbx, 11)?;
    a.mov(rsp, qword_ptr(r13 + CR_SAVE))?;
    a.sub(rsp, 8)?;
    a.ret()?;

    let mut done = a.create_label();
    a.set_label(&mut f_rec)?;
    a.sub(ebp, 1)?;
    a.jz(done)?;
    a.call(f_rec)?;
    a.add(rax, 1)?;
    a.set_label(&mut done)?;
    a.add(rdi, rbp)?;
    a.ret()?;

    // Skips its caller's frame: returns straight to f_mid's caller.
    a.set_label(&mut f_mid)?;
    a.add(rsi, 5)?;
    a.call(f_skip)?;
    a.mov(eax, 0xDEAD_0004u32)?;
    a.ret()?;
    a.set_label(&mut f_skip)?;
    a.add(rdi, 1)?;
    a.add(rsp, 8)?;
    a.ret()?;

    // Guest function run by `host_reenter` (nested emu_run_full; rcx = x, no r13 here).
    a.set_label(&mut f_host_g)?;
    a.mov(rax, rcx)?;
    a.add(rax, 7)?;
    a.sub(rsp, 0x28)?;
    a.call(leaves[1])?;
    a.add(rbx, rax)?;
    a.call(f_mid)?;
    a.add(rsi, 1)?;
    a.call(leaves[3])?;
    a.add(rsp, 0x28)?;
    a.add(rax, rsi)?;
    a.ret()?;

    let res = a.assemble_options(base, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)?;
    let mut code = res.inner.code_buffer.clone();
    let so = (res.label_ip(&slot)? - base) as usize;
    code[so..so + 8].copy_from_slice(&res.label_ip(&leaves[0])?.to_le_bytes());
    Ok((code, (res.label_ip(&entry)? - base) as usize))
}

/// Run a call/ret program (code re-copied: the pool may have been stored to).
unsafe fn cr_run(c: *mut u8, code: &[u8], entry: u64, d: *mut u8, init: &[u8], mode: u8) -> (u64, Vec<u8>) {
    core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
    core::ptr::copy_nonoverlapping(init.as_ptr(), d, init.len());
    jit::force(mode);
    let (ret, _) = emu_run_full(entry, [d as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
    jit::force(0);
    (ret, core::slice::from_raw_parts(d, init.len()).to_vec())
}

/// Call/ret differential fuzz on this thread with the given codegen knobs (must be a fresh
/// thread: the RAS decision is taken when its translation cache is created).
fn cr_fuzz(seed: u64, programs: usize, k: jit::TestKnobs) {
    jit::test_set_knobs(k);
    let ras = k.ras.unwrap_or_else(jit::ras_enabled);
    let knobs = format!(
        "RAS {ras}, FLAGS_DEAD_AT_CALL {:?}, RO_PLAIN {:?}, MEMCHK {:?}, PUSHPOP {:?}, BUDGET_BACKEDGE {:?}, budget cap {:?}, XMMREG {:?}, RETLIT {:?}, FLAGLAZY {:?}",
        k.flags_dead, k.ro_plain, k.memchk, k.pushpop, k.budget_be, k.budget_cap, k.xmmreg, k.retlit, k.flaglazy
    );
    let mut r = Rng(seed | 1);
    let jit_before = jit::test_executed();
    for p in 0..programs {
        let c = arena_alloc(0x1000, 64);
        let h = c as u64 + CR_HOST as u64;
        let (code, entry_off) = gen_callret(&mut r, c as u64, h).expect("assemble");
        assert!(code.len() <= CR_HOST, "call/ret program too large ({} bytes)", code.len());
        let d = arena_alloc(CR_DATA, 64);
        let mut init = vec![0u8; CR_DATA];
        for b in init.iter_mut().take(0x100) {
            *b = r.next() as u8;
        }
        let entry = c as u64 + entry_off as u64;
        unsafe {
            core::ptr::write_bytes(c.add(CR_HOST), 0xCC, 16);
            crate::exe::register_native_va(h, host_reenter as *const () as usize);
            jit::test_set_ro(vec![(c as u64, c as u64 + CR_POOL as u64)]);
            let interp = cr_run(c, &code, entry, d, &init, 2);
            let jit1 = cr_run(c, &code, entry, d, &init, 1);
            let jit2 = cr_run(c, &code, entry, d, &init, 1);
            crate::exe::unregister_native_va(h);
            for (name, got) in [("first", &jit1), ("linked", &jit2)] {
                if *got == interp {
                    continue;
                }
                let mut msg = format!("[{knobs}] seed {seed:#x} program {p}: translator ({name} run) differs from interpreter\n");
                if got.0 != interp.0 {
                    msg += &format!("  return value: jit {:#x} interp {:#x}\n", got.0, interp.0);
                }
                for k in 0..CR_DATA / 8 {
                    let x = u64::from_le_bytes(got.1[8 * k..8 * k + 8].try_into().unwrap());
                    let y = u64::from_le_bytes(interp.1[8 * k..8 * k + 8].try_into().unwrap());
                    if x != y {
                        let what = if k * 8 >= OUT && k * 8 < OUT + 8 * 15 {
                            format!("reg {:?}", ALL64[(k * 8 - OUT) / 8])
                        } else if k * 8 == OUT + 8 * 15 {
                            "rflags".to_string()
                        } else {
                            format!("data+{:#x}", k * 8)
                        };
                        msg += &format!("  {what}: jit {x:#018x} interp {y:#018x}\n");
                    }
                }
                msg += &disasm(&code[entry_off..], entry);
                panic!("{msg}");
            }
        }
    }
    jit::test_set_ro(Vec::new());
    let ran = jit::test_executed() - jit_before;
    eprintln!("call/ret fuzz [{knobs}]: {ran} guest instructions in translated code, {} RAS-moving runs", jit::test_ras_moves());
    assert!(ran >= 50 * programs as u64, "translator barely ran ({ran} instructions)");
    if ras {
        assert!(jit::test_ras_moves() > 0, "shadow RAS never used");
    } else {
        assert_eq!(jit::test_ras_moves(), 0);
    }
}

/// Calls/returns (shadow RAS incl. mismatches, unwinds, ring overflow and nested runs),
/// flags dead at call/ret and read-only plain loads, under each knob combination (each on a
/// fresh thread), must match the interpreter exactly.
#[test]
fn translator_call_ret_matches_interpreter() {
    let combos: [(bool, u8, bool); 6] =
        [(true, 1, true), (false, 0, false), (true, 3, true), (true, 0, false), (false, 1, true), (true, 3, false)];
    let n = jit::TestKnobs::NONE;
    let mut all: Vec<jit::TestKnobs> =
        combos.iter().map(|&(ras, fd, ro)| jit::TestKnobs { ras: Some(ras), flags_dead: Some(fd), ro_plain: Some(ro), ..n }).collect();
    // jit5 knobs: everything on with forced budget exits, each off, and the old codegen.
    let on = jit::TestKnobs { memchk: Some(15), pushpop: Some(true), budget_be: Some(true), ..n };
    all.push(jit::TestKnobs { budget_cap: Some(37), ..on });
    all.push(jit::TestKnobs { ras: Some(false), budget_cap: Some(5), ..on });
    all.push(jit::TestKnobs { memchk: Some(0), ..on });
    all.push(jit::TestKnobs { pushpop: Some(false), budget_cap: Some(200), ..on });
    all.push(jit::TestKnobs { budget_be: Some(false), budget_cap: Some(37), ..on });
    // jit6 knobs: return-address literals off (with budget exits), and the jit5 codegen.
    all.push(jit::TestKnobs { retlit: Some(false), budget_cap: Some(37), ..on });
    all.push(jit::TestKnobs { xmmreg: Some(false), retlit: Some(false), flaglazy: Some(false), ..on });
    // jit7 knobs: lazy flags/last_bits on with forced budget exits and flags dead at
    // call+ret, off (the jit6 codegen) with budget exits.
    all.push(jit::TestKnobs { flaglazy: Some(true), budget_cap: Some(37), flags_dead: Some(3), ..on });
    all.push(jit::TestKnobs { flaglazy: Some(false), budget_cap: Some(37), ..on });
    for (ci, &k) in all.iter().enumerate() {
        std::thread::Builder::new()
            .stack_size(16 << 20)
            .spawn(move || cr_fuzz(0xCA11_2E70_0000_0001 + ci as u64 * 0x1_0000, 150, k))
            .unwrap()
            .join()
            .unwrap_or_else(|_| panic!("call/ret fuzz failed (combo {ci})"));
    }
}

/// Dynamic `call rax` / tail `jmp rax` to DF_PLAIN builtins and inline variants through a
/// thunk VA registered with the *export-table* pointer (what the loader puts in the dispatch
/// table, not this crate's `fn as *const ()`): must be recognised as inline-callable (one
/// canonical copy per builtin) and served by `xfer_plain`, matching the interpreter.
#[test]
fn translator_dynamic_plain_builtins_take_xfer_plain() {
    let exp = |dll: &str, name: &str| win32_rt::lookup(dll, name).unwrap_or_else(|| panic!("{dll}!{name}")) as usize;
    let f_memcpy = exp("msvcrt.dll", "memcpy");
    let f_strcmp = exp("ucrtbase.dll", "strcmp");
    let f_ceil = exp("ucrtbase.dll", "ceil");
    let h = arena_alloc(64, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 64) };
    let (t_memcpy, t_strcmp, t_ceil) = (h as u64, h as u64 + 16, h as u64 + 32);
    let natives = [(t_memcpy, f_memcpy), (t_strcmp, f_strcmp), (t_ceil, f_ceil)];
    for &(va, f) in &natives {
        crate::exe::register_native_va(va, f);
    }
    let plain = jit::plaincall_enabled();
    for &(va, _) in &natives {
        assert!(!plain || jit::inline_host_fn(va).is_some(), "export-table builtin at thunk {va:#x} is not inline-callable");
    }

    let c = arena_alloc(0x400, 16);
    let code = (|| -> Result<Vec<u8>, IcedError> {
        let mut a = CodeAssembler::new(64)?;
        let mut tail = a.create_label();
        a.push(rbx)?;
        a.sub(rsp, 0x30)?;
        a.mov(rbx, rcx)?;
        // memcpy(rbx + 0x100, rbx, 0x40) via call rax.
        a.lea(rcx, ptr(rbx + 0x100))?;
        a.mov(rdx, rbx)?;
        a.mov(r8d, 0x40)?;
        a.mov(rax, t_memcpy)?;
        a.call(rax)?;
        a.mov(qword_ptr(rbx + 0x200), rax)?;
        // strcmp(rbx, rbx + 0x20) via a tail jmp rax.
        a.call(tail)?;
        // int result: the upper half of rax is unspecified.
        a.mov(dword_ptr(rbx + 0x208), eax)?;
        // ceil(2.5) via call rax (float inline variant).
        a.mov(rax, 0x4004_0000_0000_0000i64)?;
        a.movq(xmm0, rax)?;
        a.mov(rax, t_ceil)?;
        a.call(rax)?;
        a.movq(qword_ptr(rbx + 0x210), xmm0)?;
        a.xor(eax, eax)?;
        a.add(rsp, 0x30)?;
        a.pop(rbx)?;
        a.ret()?;
        a.set_label(&mut tail)?;
        a.mov(rcx, rbx)?;
        a.lea(rdx, ptr(rbx + 0x20))?;
        a.mov(rax, t_strcmp)?;
        a.jmp(rax)?;
        a.assemble(c as u64)
    })()
    .expect("assemble");
    let d = arena_alloc(DATA, 64);
    let mut init = vec![0u8; DATA];
    let mut r = Rng(0x0D1F_0B11_7A1E_0001);
    for b in init.iter_mut().take(0x40) {
        *b = b'a' + (r.next() % 3) as u8;
    }
    init[0x1F] = 0;
    init[0x3F] = 0;
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        let interp = run_mode(c, d, &init, 2);
        let x0 = super::test_xfer_plain();
        let jit1 = run_mode(c, d, &init, 1);
        let jit2 = run_mode(c, d, &init, 1);
        let served = super::test_xfer_plain() - x0;
        for &(va, _) in &natives {
            crate::exe::unregister_native_va(va);
        }
        for (name, got) in [("first", &jit1), ("linked", &jit2)] {
            assert_eq!(got.0, interp.0, "{name} run: return value");
            assert_eq!(got.1, interp.1, "{name} run: data\n{}", disasm(&code, c as u64));
        }
        assert_eq!(served, if plain { 6 } else { 0 }, "xfer_plain served {served} of the 6 dynamic builtin transfers");
    }
}

// --- memory checks / push-pop / budget (AW2_JIT_MEMCHK, AW2_JIT_PUSHPOP, AW2_JIT_BUDGET_BACKEDGE)

const MC_DATA: usize = 0x1000;
/// Pointer slots (valid bases into the data area, or invalid addresses); the body never
/// stores below 0x400 (bases stay in [0x470, 0x890], disps in [-64, 0xC8)).
const MC_PTRS: i32 = 0x380;
/// The final 0x100-byte stack frame (rsp-0x40 .. rsp+0xC0) is copied here.
const MC_FRAME: i32 = 0xB00;
const MC_BASES: [AsmRegister64; 4] = [rbx, rsi, rdi, rdx];
const MC_V64: [AsmRegister64; 6] = [rax, rcx, r8, r9, r10, r11];
const MC_V32: [AsmRegister32; 6] = [eax, ecx, r8d, r9d, r10d, r11d];
const MC_V16: [AsmRegister16; 6] = [ax, cx, r8w, r9w, r10w, r11w];
const MC_V8: [AsmRegister8; 6] = [al, cl, r8b, r9b, r10b, r11b];

/// Offset (from the data base) of a fresh valid base pointer, sometimes unaligned.
fn mc_off(r: &mut Rng) -> i32 {
    let o = r.below(0x100) as i32;
    0x600 + if r.chance(70) { o & !7 } else { o }
}

fn mc_val(r: &mut Rng, no_rcx: bool) -> usize {
    let v = r.below(6) as usize;
    if no_rcx && v == 1 {
        2
    } else {
        v
    }
}

/// One memory instruction on `m` of width `w`: load / store / store imm / extend / ALU
/// either way / ALU imm / imul / inc-dec / cmp (+ forward jcc when `pending` is given).
/// Writes only the value registers (never rcx with `no_rcx`).
fn mc_mem_op(
    a: &mut CodeAssembler,
    r: &mut Rng,
    m: AsmMemoryOperand,
    w: u32,
    no_rcx: bool,
    pending: Option<&mut Vec<(CodeLabel, u64)>>,
) -> Result<(), IcedError> {
    let v = mc_val(r, no_rcx);
    let op = r.below(9);
    let imm = r.imm32();
    match r.below(12) {
        0 | 1 => match w {
            8 => a.mov(MC_V8[v], byte_ptr(m)),
            16 => a.mov(MC_V16[v], word_ptr(m)),
            32 => a.mov(MC_V32[v], dword_ptr(m)),
            _ => a.mov(MC_V64[v], qword_ptr(m)),
        },
        2 | 3 => match w {
            8 => a.mov(byte_ptr(m), MC_V8[v]),
            16 => a.mov(word_ptr(m), MC_V16[v]),
            32 => a.mov(dword_ptr(m), MC_V32[v]),
            _ => a.mov(qword_ptr(m), MC_V64[v]),
        },
        4 => match w {
            8 => a.mov(byte_ptr(m), imm as i8 as i32),
            16 => a.mov(word_ptr(m), imm as i16 as i32),
            32 => a.mov(dword_ptr(m), imm),
            _ => a.mov(qword_ptr(m), imm),
        },
        5 => match w {
            8 => a.movzx(MC_V32[v], byte_ptr(m)),
            16 => a.movsx(MC_V64[v], word_ptr(m)),
            32 => a.movsxd(MC_V64[v], dword_ptr(m)),
            _ => a.movsx(MC_V32[v], byte_ptr(m)),
        },
        6 => match w {
            8 => alu_rm!(a, op, MC_V8[v], byte_ptr(m)),
            16 => alu_rm!(a, op, MC_V16[v], word_ptr(m)),
            32 => alu_rm!(a, op, MC_V32[v], dword_ptr(m)),
            _ => alu_rm!(a, op, MC_V64[v], qword_ptr(m)),
        },
        7 => match w {
            8 => alu!(a, op, byte_ptr(m), MC_V8[v]),
            16 => alu!(a, op, word_ptr(m), MC_V16[v]),
            32 => alu!(a, op, dword_ptr(m), MC_V32[v]),
            _ => alu!(a, op, qword_ptr(m), MC_V64[v]),
        },
        8 => match w {
            8 => alu!(a, op, byte_ptr(m), imm as i8 as i32),
            16 => alu!(a, op, word_ptr(m), imm as i16 as i32),
            32 => alu!(a, op, dword_ptr(m), imm),
            _ => alu!(a, op, qword_ptr(m), imm),
        },
        9 => {
            if w == 64 {
                a.imul_2(MC_V64[v], qword_ptr(m))
            } else {
                a.imul_3(MC_V32[v], dword_ptr(m), imm)
            }
        }
        10 => match (w, r.chance(50)) {
            (8, true) => a.inc(byte_ptr(m)),
            (8, false) => a.dec(byte_ptr(m)),
            (16, true) => a.inc(word_ptr(m)),
            (16, false) => a.dec(word_ptr(m)),
            (32, true) => a.inc(dword_ptr(m)),
            (32, false) => a.dec(dword_ptr(m)),
            (_, true) => a.inc(qword_ptr(m)),
            (_, false) => a.dec(qword_ptr(m)),
        },
        _ => {
            match w {
                8 => a.cmp(MC_V8[v], byte_ptr(m))?,
                16 => a.cmp(MC_V16[v], word_ptr(m))?,
                32 => a.cmp(MC_V32[v], dword_ptr(m))?,
                _ => a.cmp(MC_V64[v], qword_ptr(m))?,
            }
            if let Some(p) = pending {
                let l = a.create_label();
                jcc!(a, r.below(16), l)?;
                p.push((l, 1 + r.below(3)));
            }
            Ok(())
        }
    }
}

/// `[rsp+d]` access (d in [0, 0x48), often unaligned; the frame is initialised).
fn mc_rsp_op(a: &mut CodeAssembler, r: &mut Rng) -> Result<(), IcedError> {
    let w = [8u32, 16, 32, 64, 64, 32][r.below(6) as usize];
    let sz = (w / 8) as i32;
    let mut d = r.below(0x48) as i32;
    if r.chance(60) {
        d &= !(sz - 1);
    }
    let m = rsp + d;
    let v = mc_val(r, false);
    if r.chance(30) {
        return mc_mem_op(a, r, m, w, false, None);
    }
    match (r.below(3), w) {
        (0, 8) => a.mov(MC_V8[v], byte_ptr(m)),
        (0, 16) => a.mov(MC_V16[v], word_ptr(m)),
        (0, 32) => a.mov(MC_V32[v], dword_ptr(m)),
        (0, _) => a.mov(MC_V64[v], qword_ptr(m)),
        (1, 8) => a.mov(byte_ptr(m), MC_V8[v]),
        (1, 16) => a.mov(word_ptr(m), MC_V16[v]),
        (1, 32) => a.mov(dword_ptr(m), MC_V32[v]),
        (1, _) => a.mov(qword_ptr(m), MC_V64[v]),
        (_, 8) => a.mov(byte_ptr(m), r.imm32() as i8 as i32),
        (_, 16) => a.mov(word_ptr(m), r.imm32() as i16 as i32),
        (_, 32) => a.mov(dword_ptr(m), r.imm32()),
        (_, _) => a.mov(qword_ptr(m), r.imm32()),
    }
}

/// Random program for the memory-check / push-pop / budget translator paths, assembled at
/// `base`; returns (code, entry offset). A rip-relative RW pool (aligned and unaligned
/// slots) comes first. Bases rbx/rsi/rdi/rdx point into the data area (sometimes unaligned)
/// or at invalid addresses, and are rewritten mid-block explicitly (add/lea/mov/xchg/cmov,
/// loads of pointer slots, push+pop) and implicitly (`mul` writes rdx); `[rsp+d]` accesses
/// run with rsp 8-aligned, 16-aligned (`and rsp, -16`) and misaligned; pushes/pops come in
/// fusable pairs, same-register pops, `push rsp`/`pop rsp`, `push/pop [m]`; the body is a
/// `jnz` loop with inner `loop` loops, forward jccs, out-of-range absolute accesses.
fn gen_memchk(r: &mut Rng, base: u64) -> Result<(Vec<u8>, usize), IcedError> {
    let mut a = CodeAssembler::new(64)?;
    let mut pool: Vec<CodeLabel> = Vec::new();
    for k in 0..7 {
        if k >= 4 {
            // Unaligned slots.
            a.db(&[0x5A, 0xA5, 0x3C][..k - 3])?;
        }
        let mut l = a.create_label();
        a.set_label(&mut l)?;
        let q: Vec<u64> = (0..2).map(|_| r.next()).collect();
        a.dq(&q)?;
        pool.push(l);
    }
    let mut entry = a.create_label();
    a.set_label(&mut entry)?;
    a.mov(r13, rcx)?;
    a.mov(rbp, rsp)?;
    // Initialised frame: 0x40 bytes below the working rsp, 0xC0 above.
    a.sub(rsp, 0x100)?;
    for k in 0..32 {
        a.mov(rax, qword_ptr(r13 + 8 * k))?;
        a.mov(qword_ptr(rsp + 8 * k), rax)?;
    }
    a.add(rsp, 0x40)?;
    for j in 0..8 {
        if r.chance(25) {
            const BAD: [u64; 4] = [0x100, 0x8000, 0x8000_0002_0000, 0xFFFF_8000_0000_1000];
            a.mov(rax, (BAD[r.below(4) as usize] + r.below(0x40)) as i64)?;
        } else {
            a.lea(rax, ptr(r13 + mc_off(r)))?;
        }
        a.mov(qword_ptr(r13 + MC_PTRS + 8 * j), rax)?;
    }
    for &reg in &MC_V64 {
        a.mov(reg, r.imm64())?;
    }
    let fl = (r.next() as u32) & 0x8C5;
    a.push(fl as i32)?;
    a.popfq()?;
    a.mov(r15d, 1 + r.below(3) as u32)?;
    let mut top = a.create_label();
    a.set_label(&mut top)?;
    a.nop()?;
    for &b in &MC_BASES {
        if r.chance(10) {
            a.mov(b, 0x100 + r.below(0x100) as i64)?;
        } else {
            a.lea(b, ptr(r13 + mc_off(r)))?;
        }
    }
    let mut depth = 0u32;
    let mut pending: Vec<(CodeLabel, u64)> = Vec::new();
    for _ in 0..10 + r.below(40) {
        if r.chance(8) {
            let l = a.create_label();
            jcc!(a, r.below(16), l)?;
            pending.push((l, 1 + r.below(3)));
        }
        let w = [8u32, 16, 32, 64, 64, 32][r.below(6) as usize];
        let sz = (w / 8) as i32;
        let b = MC_BASES[r.below(4) as usize];
        let b2 = MC_BASES[r.below(4) as usize];
        let mut d = r.below(0x108) as i32 - 64;
        if r.chance(75) {
            d &= !(sz - 1);
        }
        let v = mc_val(&mut *r, false);
        let v2 = mc_val(&mut *r, false);
        match r.below(20) {
            0..=8 => mc_mem_op(&mut a, r, b + d, w, false, Some(&mut pending))?,
            // Base rewrites (drift at most 8 bytes per op).
            9 | 10 => match r.below(9) {
                0 => a.add(b, 8)?,
                1 => a.sub(b, 8)?,
                2 => a.lea(b, ptr(b2 + (r.below(17) as i32 - 8)))?,
                3 => a.xchg(b, b2)?,
                4 => cmov!(a, r.below(16), b, b2)?,
                5 => a.mov(b, qword_ptr(r13 + MC_PTRS + 8 * r.below(8) as i32))?,
                6 => {
                    // rdx:rax = 2^63 * 2p: rdx = p (implicit base write).
                    a.mov(rax, i64::MIN)?;
                    if r.chance(75) {
                        a.lea(rcx, ptr(r13 + mc_off(r)))?;
                    } else {
                        a.mov(ecx, 0x100 + r.below(0x100) as u32)?;
                    }
                    a.add(rcx, rcx)?;
                    a.mul(rcx)?;
                }
                7 => {
                    a.push(b2)?;
                    if r.chance(50) {
                        a.push(MC_V64[v])?;
                        a.pop(MC_V64[v2])?;
                    }
                    a.pop(b)?;
                }
                _ => a.mov(b, b2)?,
            },
            11 | 12 => mc_rsp_op(&mut a, r)?,
            // rsp windows (restored at the end of the unit).
            13 => {
                let undo: (u32, i32) = match r.below(4) {
                    0 => {
                        a.mov(r12, rsp)?;
                        a.and(rsp, -16)?;
                        (0, 0)
                    }
                    1 => {
                        let s = [1, 2, 4, 12][r.below(4) as usize];
                        a.sub(rsp, s)?;
                        (1, s)
                    }
                    2 => {
                        let s = [8, 16, 24][r.below(3) as usize];
                        a.sub(rsp, s)?;
                        (1, s)
                    }
                    _ => {
                        let s = [4, 8][r.below(2) as usize];
                        a.lea(rsp, ptr(rsp - s))?;
                        (2, s)
                    }
                };
                for _ in 0..1 + r.below(3) {
                    mc_rsp_op(&mut a, r)?;
                }
                if r.chance(30) {
                    a.push(MC_V64[v])?;
                    mc_rsp_op(&mut a, r)?;
                    a.pop(MC_V64[v2])?;
                }
                match undo {
                    (0, _) => a.mov(rsp, r12)?,
                    (1, s) => a.add(rsp, s)?,
                    (_, s) => a.lea(rsp, ptr(rsp + s))?,
                }
            }
            14 | 15 => {
                if !pending.is_empty() {
                    a.nop()?;
                } else if depth > 0 && r.chance(50) {
                    let mut pd = r.below(0x40) as i32;
                    if r.chance(70) {
                        pd &= !7;
                    }
                    match r.below(6) {
                        0 | 1 if depth >= 2 => {
                            // Fusable pair (or the same register twice: not fused).
                            a.pop(MC_V64[v])?;
                            a.pop(MC_V64[if r.chance(25) { v } else { v2 }])?;
                            depth -= 2;
                        }
                        2 => {
                            a.pop(qword_ptr(b + d))?;
                            depth -= 1;
                        }
                        3 => {
                            a.pop(qword_ptr(rsp + pd))?;
                            depth -= 1;
                        }
                        _ => {
                            a.pop(MC_V64[v])?;
                            depth -= 1;
                        }
                    }
                } else if depth < 8 {
                    let mut pd = r.below(0x40) as i32;
                    if r.chance(70) {
                        pd &= !7;
                    }
                    match r.below(7) {
                        0 | 1 => {
                            a.push(MC_V64[v])?;
                            a.push(if r.chance(30) { b } else { MC_V64[v2] })?;
                            depth += 2;
                        }
                        2 => {
                            a.push(r.imm32())?;
                            depth += 1;
                        }
                        3 => {
                            a.push(qword_ptr(b + d))?;
                            depth += 1;
                        }
                        4 => {
                            a.push(qword_ptr(rsp + pd))?;
                            depth += 1;
                        }
                        5 => {
                            // Balanced: rsp unchanged.
                            a.push(rsp)?;
                            a.pop(rsp)?;
                        }
                        _ => {
                            a.push(MC_V64[v])?;
                            depth += 1;
                        }
                    }
                } else {
                    a.nop()?;
                }
            }
            16 => {
                // Inner `loop` (backward transfer) over base-relative accesses.
                let mut l = a.create_label();
                a.mov(ecx, 1 + r.below(4) as u32)?;
                a.set_label(&mut l)?;
                for _ in 0..1 + r.below(3) {
                    let mut dd = r.below(0x108) as i32 - 64;
                    if r.chance(75) {
                        dd &= !(sz - 1);
                    }
                    let bb = MC_BASES[r.below(4) as usize];
                    mc_mem_op(&mut a, r, bb + dd, w, true, None)?;
                }
                a.loop_(l)?;
            }
            17 => {
                // rip-relative (absolute) RW pool operand.
                let l = pool[r.below(pool.len() as u64) as usize];
                match r.below(8) {
                    0 => a.mov(MC_V64[v], qword_ptr(l))?,
                    1 => a.mov(dword_ptr(l), MC_V32[v])?,
                    2 => a.add(MC_V64[v], qword_ptr(l))?,
                    3 => a.cmp(dword_ptr(l), r.imm32())?,
                    4 => a.inc(qword_ptr(l))?,
                    5 => a.movzx(MC_V32[v], word_ptr(l))?,
                    6 => a.mov(qword_ptr(l), r.imm32())?,
                    _ => a.add(qword_ptr(l), MC_V64[v])?,
                }
            }
            18 => {
                // Absolute operands outside [0x10000, 2^47) (bad accesses: side exits).
                match r.below(4) {
                    0 => a.mov(MC_V32[v], dword_ptr(0x2000u64 + r.below(0x100)))?,
                    1 => a.mov(qword_ptr(-16i64), MC_V64[v])?,
                    2 => a.add(MC_V64[v], qword_ptr(0x8000u64))?,
                    _ => a.mov(dword_ptr(0x100u64), r.imm32())?,
                }
            }
            _ => a.add(MC_V64[v], MC_V64[v2])?,
        }
        for p in pending.iter_mut() {
            p.1 -= 1;
        }
        let mut k = 0;
        while k < pending.len() {
            if pending[k].1 == 0 {
                let (mut l, _) = pending.remove(k);
                a.set_label(&mut l)?;
                a.nop()?;
            } else {
                k += 1;
            }
        }
    }
    for (mut l, _) in pending.drain(..) {
        a.set_label(&mut l)?;
        a.nop()?;
    }
    while depth > 0 {
        let v = mc_val(&mut *r, false);
        if depth >= 2 && r.chance(50) {
            a.pop(MC_V64[v])?;
            a.pop(MC_V64[(v + 1) % 6])?;
            depth -= 2;
        } else {
            a.pop(MC_V64[v])?;
            depth -= 1;
        }
    }
    a.dec(r15d)?;
    a.jnz(top)?;
    for k in 0..32 {
        a.mov(rax, qword_ptr(rsp + (8 * k - 0x40)))?;
        a.mov(qword_ptr(r13 + MC_FRAME + 8 * k), rax)?;
    }
    a.mov(rsp, rbp)?;
    // rbp / r12 hold guest stack addresses.
    a.xor(ebp, ebp)?;
    a.xor(r12d, r12d)?;
    for (k, &reg) in ALL64.iter().enumerate() {
        a.mov(qword_ptr(r13 + (OUT + 8 * k) as i32), reg)?;
    }
    a.pushfq()?;
    a.pop(rdx)?;
    a.mov(qword_ptr(r13 + (OUT + 8 * 15) as i32), rdx)?;
    a.ret()?;
    let res = a.assemble_options(base, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)?;
    Ok((res.inner.code_buffer.clone(), (res.label_ip(&entry)? - base) as usize))
}

/// Memory-check / push-pop / budget differential fuzz on this (fresh) thread.
fn mc_fuzz(seed: u64, programs: usize, k: jit::TestKnobs) {
    jit::test_set_knobs(k);
    let knobs = format!(
        "MEMCHK {:?}, PUSHPOP {:?}, BUDGET_BACKEDGE {:?}, budget cap {:?}, RAS {:?}, XMMREG {:?}, RETLIT {:?}, FLAGLAZY {:?}",
        k.memchk, k.pushpop, k.budget_be, k.budget_cap, k.ras, k.xmmreg, k.retlit, k.flaglazy
    );
    let mut r = Rng(seed | 1);
    let jit_before = jit::test_executed();
    for p in 0..programs {
        let c = arena_alloc(0x2000, 64);
        let (code, entry_off) = gen_memchk(&mut r, c as u64).expect("assemble");
        assert!(code.len() <= 0x2000, "memchk program too large ({} bytes)", code.len());
        let d = arena_alloc(MC_DATA, 64);
        let mut init = vec![0u8; MC_DATA];
        for (i, b) in init.iter_mut().enumerate() {
            if i < 0x100 || (0x400..0xA00).contains(&i) {
                *b = r.next() as u8;
            }
        }
        let entry = c as u64 + entry_off as u64;
        unsafe {
            if let Some(path) = std::env::var_os("AW2_JIT_FUZZ_TRACE") {
                let _ = std::fs::write(path, format!("[{knobs}] seed {seed:#x} program {p}\n{}", disasm(&code[entry_off..], entry)));
            }
            let interp = cr_run(c, &code, entry, d, &init, 2);
            let jit1 = cr_run(c, &code, entry, d, &init, 1);
            let jit2 = cr_run(c, &code, entry, d, &init, 1);
            for (name, got) in [("first", &jit1), ("linked", &jit2)] {
                if *got == interp {
                    continue;
                }
                let mut msg = format!("[{knobs}] seed {seed:#x} program {p}: translator ({name} run) differs from interpreter\n");
                if got.0 != interp.0 {
                    msg += &format!("  return value: jit {:#x} interp {:#x}\n", got.0, interp.0);
                }
                for q in 0..MC_DATA / 8 {
                    let x = u64::from_le_bytes(got.1[8 * q..8 * q + 8].try_into().unwrap());
                    let y = u64::from_le_bytes(interp.1[8 * q..8 * q + 8].try_into().unwrap());
                    if x != y {
                        let what = if q * 8 >= OUT && q * 8 < OUT + 8 * 15 {
                            format!("reg {:?}", ALL64[(q * 8 - OUT) / 8])
                        } else if q * 8 == OUT + 8 * 15 {
                            "rflags".to_string()
                        } else if q * 8 >= MC_FRAME as usize && q * 8 < MC_FRAME as usize + 0x100 {
                            format!("[rsp{:+#x}]", q as i64 * 8 - MC_FRAME as i64 - 0x40)
                        } else {
                            format!("data+{:#x}", q * 8)
                        };
                        msg += &format!("  {what}: jit {x:#018x} interp {y:#018x}\n");
                    }
                }
                msg += &disasm(&code[entry_off..], entry);
                panic!("{msg}");
            }
        }
    }
    let ran = jit::test_executed() - jit_before;
    eprintln!("memchk fuzz [{knobs}]: {ran} guest instructions in translated code");
    assert!(ran >= 40 * programs as u64, "translator barely ran ({ran} instructions)");
}

/// Same-base check groups (incl. implicit / conditional base writes and invalid bases),
/// the combined range+alignment mask, absolute operands, `[rsp+d]` ldapur/stlur with
/// 8-aligned / misaligned rsp, plain fused push/pop and budget tests only at back-edges /
/// indirect transfers (with forced budget exits), under each knob (each on a fresh thread),
/// must match the interpreter exactly.
#[test]
fn translator_memchk_pushpop_budget_match_interpreter() {
    let n = jit::TestKnobs::NONE;
    let on = jit::TestKnobs { memchk: Some(15), pushpop: Some(true), budget_be: Some(true), ..n };
    let combos = [
        on,
        jit::TestKnobs {
            memchk: Some(0),
            pushpop: Some(false),
            budget_be: Some(false),
            xmmreg: Some(false),
            retlit: Some(false),
            flaglazy: Some(false),
            ..n
        },
        jit::TestKnobs { memchk: Some(1), ..on },
        jit::TestKnobs { memchk: Some(2), ..on },
        jit::TestKnobs { memchk: Some(4), ..on },
        jit::TestKnobs { memchk: Some(8), ..on },
        jit::TestKnobs { memchk: Some(12), ..on },
        jit::TestKnobs { pushpop: Some(false), ..on },
        jit::TestKnobs { budget_cap: Some(7), ..on },
        jit::TestKnobs { budget_be: Some(false), budget_cap: Some(7), ..on },
        jit::TestKnobs { budget_cap: Some(2), ras: Some(false), ..on },
        jit::TestKnobs { retlit: Some(false), budget_cap: Some(7), ..on },
        jit::TestKnobs { xmmreg: Some(false), retlit: Some(false), flaglazy: Some(false), ..on },
        jit::TestKnobs { flaglazy: Some(false), budget_cap: Some(7), ..on },
        jit::TestKnobs { flaglazy: Some(true), budget_cap: Some(7), flags_dead: Some(3), ..on },
    ];
    for (ci, &k) in combos.iter().enumerate() {
        std::thread::Builder::new()
            .stack_size(16 << 20)
            .spawn(move || mc_fuzz(0x3E3C_4EC0_0000_0001 + ci as u64 * 0x1_0000, 300, k))
            .unwrap()
            .join()
            .unwrap_or_else(|_| panic!("memchk fuzz failed (combo {ci})"));
    }
}

// --- SSE: XMM register cache, out-of-line NaN fixups, return-address literals ----------------

const SX_DATA: usize = 0x800;
/// FP operand pool (special f32/f64 values and random ones) at data+0..0x200; vector stores
/// go anywhere in data+0..0x2F0, GPRs/RFLAGS at `OUT`, ymm0..15 at `SX_YMM`.
const SX_POOL: u64 = 0x200;
const SX_YMM: i32 = 0x400;
/// Offset of the host stub (`ceil`) inside each program's 0x2000-byte code area.
const SX_HOST: usize = 0x1FF0;

const SX_X: [AsmRegisterXmm; 16] =
    [xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7, xmm8, xmm9, xmm10, xmm11, xmm12, xmm13, xmm14, xmm15];
const SX_Y: [AsmRegisterYmm; 16] =
    [ymm0, ymm1, ymm2, ymm3, ymm4, ymm5, ymm6, ymm7, ymm8, ymm9, ymm10, ymm11, ymm12, ymm13, ymm14, ymm15];

/// f32: ±0, 1, -1.5, ±inf, ±qNaN, sNaN, negative sNaN with payload, smallest / largest
/// denormal, smallest normal, largest finite, pi, -123.456.
const SX_F32: [u32; 16] = [
    0, 0x8000_0000, 0x3F80_0000, 0xBFC0_0000, 0x7F80_0000, 0xFF80_0000, 0x7FC0_0000, 0xFFC0_0000, 0x7F80_0001,
    0xFFA0_1234, 0x0000_0001, 0x807F_FFFF, 0x0080_0000, 0x7F7F_FFFF, 0x4049_0FDB, 0xC2F6_E979,
];
const SX_F64: [u64; 16] = [
    0,
    0x8000_0000_0000_0000,
    0x3FF0_0000_0000_0000,
    0xC004_0000_0000_0000,
    0x7FF0_0000_0000_0000,
    0xFFF0_0000_0000_0000,
    0x7FF8_0000_0000_0000,
    0xFFF8_0000_0000_0000,
    0x7FF0_0000_0000_0001,
    0xFFF4_0000_0000_1234,
    0x0000_0000_0000_0001,
    0x800F_FFFF_FFFF_FFFF,
    0x0010_0000_0000_0000,
    0x7FEF_FFFF_FFFF_FFFF,
    0x4009_21FB_5444_2D18,
    0xC05E_DD2F_1A9F_BE77,
];

/// Random f32 / f64 of moderate magnitude (exact products / sums stay finite).
fn sx_f32m(r: &mut Rng) -> u32 {
    (r.next() as u32 & 0x8000_0000) | ((100 + r.below(55) as u32) << 23) | (r.next() as u32 & 0x7F_FFFF)
}

fn sx_f64m(r: &mut Rng) -> u64 {
    (r.next() & (1 << 63)) | ((1000 + r.below(47)) << 52) | (r.next() & 0xF_FFFF_FFFF_FFFF)
}

fn sx_fill_pool(r: &mut Rng, pool: &mut [u8]) {
    for q in pool.chunks_exact_mut(8) {
        let v: u64 = match r.below(6) {
            0 => SX_F32[r.below(16) as usize] as u64 | (SX_F32[r.below(16) as usize] as u64) << 32,
            1 => SX_F64[r.below(16) as usize],
            2 => sx_f32m(r) as u64 | (sx_f32m(r) as u64) << 32,
            3 => sx_f64m(r),
            4 => SX_F32[r.below(16) as usize] as u64 | (sx_f32m(r) as u64) << 32,
            _ => r.next(),
        };
        q.copy_from_slice(&v.to_le_bytes());
    }
}

/// XMM register index, biased to a few registers so values chain through the cache.
fn sx_xi(r: &mut Rng) -> usize {
    if r.chance(70) {
        r.below(6) as usize
    } else {
        r.below(16) as usize
    }
}

/// Load operand: the FP pool (sometimes misaligned, sometimes indexed by the loop counter)
/// or a bad address (side exit).
fn sx_mem(r: &mut Rng, size: i32) -> AsmMemoryOperand {
    if r.chance(4) {
        return r14 + (r.below(64) as i32);
    }
    let align = if r.chance(20) { 1 } else { size };
    let d = (r.below(SX_POOL - 0x40) as i32 / align) * align;
    if r.chance(20) {
        r13 + r15 * 8 + d
    } else {
        r13 + d
    }
}

/// Store operand: anywhere below `OUT` (incl. the pool) or a bad address (side exit).
fn sx_st(r: &mut Rng, size: i32) -> AsmMemoryOperand {
    if r.chance(4) {
        return r14 + (r.below(64) as i32);
    }
    let align = if r.chance(20) { 1 } else { size };
    r13 + (r.below(OUT as u64 - 0x20) as i32 / align) * align
}

fn sx_setcc(a: &mut CodeAssembler, r: &mut Rng, d: AsmRegister8) -> Result<(), IcedError> {
    match r.below(5) {
        0 => a.seta(d),
        1 => a.setb(d),
        2 => a.sete(d),
        3 => a.setp(d),
        _ => a.setbe(d),
    }
}

/// One random SSE / AVX / integer instruction; with `leaves` non-empty also guest calls and
/// `ceil` host calls (direct and via `call rax`) with live dirty XMM registers.
fn sx_insn(a: &mut CodeAssembler, r: &mut Rng, leaves: &[CodeLabel], h_ceil: u64) -> Result<(), IcedError> {
    let (xi, yi, zi) = (sx_xi(r), sx_xi(r), sx_xi(r));
    let (x, y, z) = (SX_X[xi], SX_X[yi], SX_X[zi]);
    let (yx, yy, yz) = (SX_Y[xi], SX_Y[yi], SX_Y[zi]);
    let i = r.below(12) as usize;
    let j = r.below(12) as usize;
    let sel = r.below(if leaves.is_empty() { 108 } else { 112 });
    match sel {
        // Scalar arithmetic (cache + out-of-line NaN fixup).
        0 => a.addss(x, y),
        1 => a.subss(x, y),
        2 => a.mulss(x, y),
        3 => a.divss(x, y),
        4 => a.minss(x, y),
        5 => a.maxss(x, y),
        6 => a.addsd(x, y),
        7 => a.subsd(x, y),
        8 => a.mulsd(x, y),
        9 => a.divsd(x, y),
        10 => a.minsd(x, y),
        11 => a.maxsd(x, y),
        12 => a.sqrtss(x, y),
        13 => a.sqrtsd(x, y),
        14 => a.cvtss2sd(x, y),
        15 => a.cvtsd2ss(x, y),
        16 => a.addss(x, dword_ptr(sx_mem(r, 4))),
        17 => a.mulsd(x, qword_ptr(sx_mem(r, 8))),
        18 => a.divss(x, dword_ptr(sx_mem(r, 4))),
        19 => a.maxsd(x, qword_ptr(sx_mem(r, 8))),
        20 => a.sqrtss(x, dword_ptr(sx_mem(r, 4))),
        21 => a.cvtss2sd(x, dword_ptr(sx_mem(r, 4))),
        22 => a.vaddss(x, y, z),
        23 => a.vmulsd(x, y, z),
        24 => a.vsubss(x, y, dword_ptr(sx_mem(r, 4))),
        25 => a.vminsd(x, y, z),
        26 => a.vsqrtss(x, y, z),
        27 => a.vdivsd(x, y, z),
        // Packed arithmetic.
        28 => a.addps(x, y),
        29 => a.subps(x, y),
        30 => a.mulps(x, y),
        31 => a.divps(x, y),
        32 => a.minps(x, y),
        33 => a.maxps(x, y),
        34 => a.addpd(x, y),
        35 => a.mulpd(x, y),
        36 => a.divpd(x, y),
        37 => a.minpd(x, y),
        38 => a.maxpd(x, y),
        39 => a.subpd(x, y),
        40 => a.vaddps(x, y, xmmword_ptr(sx_mem(r, 16))),
        41 => a.vmulps(x, y, z),
        42 => a.vmaxpd(x, y, z),
        43 => a.vaddps(yx, yy, yz),
        // Moves and loads.
        44 => a.movss(x, y),
        45 => a.movsd_2(x, y),
        46 => a.movss(x, dword_ptr(sx_mem(r, 4))),
        47 => a.movsd_2(x, qword_ptr(sx_mem(r, 8))),
        48 => a.movaps(x, y),
        49 => a.movups(x, xmmword_ptr(sx_mem(r, 16))),
        50 => a.movq(x, y),
        51 => a.movd(x, dword_ptr(sx_mem(r, 4))),
        52 => a.movq(x, qword_ptr(sx_mem(r, 8))),
        53 => a.vmovss_3(x, y, z),
        54 => a.vmovss(x, dword_ptr(sx_mem(r, 4))),
        55 => a.vmovaps(x, y),
        // Logic.
        56 => a.xorps(x, y),
        57 => a.xorps(x, x),
        58 => a.andps(x, y),
        59 => a.orps(x, y),
        60 => a.pxor(x, x),
        61 => a.vxorps(x, y, z),
        62 => a.andnps(x, y),
        // Stores / extraction.
        63 => a.movss(dword_ptr(sx_st(r, 4)), x),
        64 => a.movsd_2(qword_ptr(sx_st(r, 8)), x),
        65 => a.movups(xmmword_ptr(sx_st(r, 16)), x),
        66 => a.movq(qword_ptr(sx_st(r, 8)), x),
        67 => a.movhps(qword_ptr(sx_st(r, 8)), x),
        68 => a.movd(W32[i], x),
        69 => a.movq(W64[i], x),
        70 => a.movd(x, W32[i]),
        71 => a.movq(x, W64[i]),
        // Conversions.
        72 => a.cvtsi2ss(x, W32[i]),
        73 => a.cvtsi2sd(x, W64[i]),
        74 => a.cvtsi2ss(x, dword_ptr(sx_mem(r, 4))),
        75 => a.vcvtsi2ss(x, y, W32[i]),
        76 => a.cvttss2si(W32[i], x),
        77 => a.cvtsd2si(W64[i], x),
        78 => a.cvttsd2si(W32[i], qword_ptr(sx_mem(r, 8))),
        79 => a.cvtss2si(W64[i], x),
        // Compares (flags consumed).
        80 => {
            a.comiss(x, y)?;
            sx_setcc(a, r, W8[i])
        }
        81 => {
            a.ucomisd(x, qword_ptr(sx_mem(r, 8)))?;
            sx_setcc(a, r, W8[i])
        }
        // Generic vector path (operands from the cache, result written through).
        82 => a.shufps(x, y, r.below(256) as i32),
        83 => a.unpcklps(x, y),
        84 => a.pshufd(x, y, r.below(256) as i32),
        85 => a.paddd(x, y),
        86 => a.pmullw(x, y),
        87 => a.psrlq(x, r.below(70) as i32),
        88 => a.cmpps(x, y, r.below(8) as i32),
        89 => a.cvtdq2ps(x, y),
        // Horizontal / addsub / rsqrt (x86 NaN rules: both NaN -> first source quieted).
        90 => match r.below(9) {
            0 | 1 => a.haddps(x, y),
            2 => a.hsubps(x, y),
            3 => a.addsubps(x, y),
            4 => a.haddpd(x, y),
            5 => a.addsubpd(x, y),
            6 => a.rsqrtps(x, y),
            7 => a.vhaddps(yx, yy, yz),
            _ => a.punpckhqdq(x, y),
        },
        91 => a.roundss(x, y, r.below(4) as i32),
        92 => a.movhlps(x, y),
        93 => a.movlhps(x, y),
        94 => a.andnpd(x, y),
        // Cache barriers (read / write `Cpu::xmm` directly).
        95 => a.movmskps(W32[i], x),
        96 => a.pinsrd(x, W32[i], r.below(4) as i32),
        97 => a.pextrd(W32[i], x, r.below(4) as i32),
        98 => a.ptest(x, y),
        99 => a.vbroadcastss(x, y),
        100 => a.pmovzxbw(x, y),
        101 => a.vfmadd213ps(x, y, z),
        102 => a.vzeroupper(),
        // Integer (no XMM access).
        103 => a.add(W64[i], W64[j]),
        104 => a.imul_2(W32[i], W32[j]),
        105 => a.lea(W64[i], ptr(W64[j] + r.imm32())),
        106 => a.mov(W32[i], dword_ptr(sx_mem(r, 4))),
        107 => a.vmovdqu(ymmword_ptr(sx_st(r, 32)), yx),
        // Guest SSE leaf (flags are dead after calls: rewrite them).
        108 => {
            a.call(leaves[r.below(leaves.len() as u64) as usize])?;
            a.cmp(r15d, 2)
        }
        // ceil(xmm0) via call rel32 (inline plain call) or call rax (host_xfer).
        _ => {
            const CEIL_IN: [u64; 8] = [
                0x4002_0000_0000_0000,
                0xC004_0000_0000_0000,
                0x8000_0000_0000_0000,
                0x3FDF_5C28_F5C2_8F5C,
                0x7FEF_FFFF_FFFF_FFFF,
                0xBFE0_0000_0000_0000,
                0x4330_0000_0000_0001,
                0x7FF0_0000_0000_0000,
            ];
            a.mov(rax, CEIL_IN[r.below(8) as usize] as i64)?;
            a.movq(xmm0, rax)?;
            if r.chance(50) {
                // xmm0 dirty in its cache register at the call.
                a.addsd(xmm0, xmm0)?;
            }
            if sel == 109 {
                a.call(h_ceil)?;
            } else {
                a.mov(rax, h_ceil)?;
                a.call(rax)?;
            }
            // Only the f64 result is defined; volatile GPRs and flags are reset.
            a.movq(xmm0, xmm0)?;
            a.xor(eax, eax)?;
            a.mov(ecx, 0x11)?;
            a.mov(edx, 0x22)?;
            a.mov(r8d, 0x33)?;
            a.mov(r9d, 0x44)?;
            a.mov(r10d, 0x55)?;
            a.mov(r11d, 0x66)
        }
    }
}

/// SSE program: random GPRs/flags/xmm0..15 (from the pool), a loop over random SSE/AVX,
/// integer, barrier and call instructions with forward `jcc`s, then ymm0..15, the GPRs and
/// RFLAGS stored. Two guest leaf functions made of SSE instructions follow.
fn gen_sse(r: &mut Rng, base: u64, h_ceil: u64) -> Result<(Vec<u8>, usize), IcedError> {
    let mut a = CodeAssembler::new(64)?;
    let mut leaves = [a.create_label(), a.create_label()];
    let mut entry = a.create_label();
    a.set_label(&mut entry)?;
    a.mov(r13, rcx)?;
    a.vzeroupper()?;
    for (k, &reg) in W64.iter().enumerate() {
        if k % 4 == 0 && r.chance(30) {
            a.mov(reg, r.imm32() as i64)?;
        } else {
            a.mov(reg, r.imm64())?;
        }
    }
    for &x in SX_X.iter() {
        let off = r.below(SX_POOL - 15) as i32;
        a.movdqu(x, xmmword_ptr(r13 + off))?;
    }
    a.mov(r14, 0x1000 + r.below(0x8000) as i64)?;
    let fl = (r.next() as u32) & 0x8C5;
    a.push(fl as i32)?;
    a.popfq()?;
    a.mov(r15d, 1 + r.below(4) as i32)?;
    let mut top = a.create_label();
    a.set_label(&mut top)?;
    a.nop()?;
    let mut pending: Vec<(CodeLabel, u64)> = Vec::new();
    for _ in 0..8 + r.below(48) {
        if r.chance(8) {
            let l = a.create_label();
            jcc!(a, r.below(16), l)?;
            pending.push((l, 1 + r.below(4)));
        }
        sx_insn(&mut a, r, &leaves, h_ceil)?;
        for p in pending.iter_mut() {
            p.1 -= 1;
        }
        let mut k = 0;
        while k < pending.len() {
            if pending[k].1 == 0 {
                let (mut l, _) = pending.remove(k);
                a.set_label(&mut l)?;
                a.nop()?;
            } else {
                k += 1;
            }
        }
    }
    for (mut l, _) in pending.drain(..) {
        a.set_label(&mut l)?;
        a.nop()?;
    }
    a.dec(r15d)?;
    a.jnz(top)?;
    for (k, &y) in SX_Y.iter().enumerate() {
        a.vmovdqu(ymmword_ptr(r13 + (SX_YMM + 32 * k as i32)), y)?;
    }
    for (k, &reg) in ALL64.iter().enumerate() {
        a.mov(qword_ptr(r13 + (OUT + 8 * k) as i32), reg)?;
    }
    a.pushfq()?;
    a.pop(rdx)?;
    a.mov(qword_ptr(r13 + (OUT + 8 * 15) as i32), rdx)?;
    a.ret()?;
    for l in leaves.iter_mut() {
        a.set_label(l)?;
        for _ in 0..2 + r.below(6) {
            sx_insn(&mut a, r, &[], 0)?;
        }
        a.ret()?;
    }
    let res = a.assemble_options(base, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)?;
    Ok((res.inner.code_buffer.clone(), (res.label_ip(&entry)? - base) as usize))
}

/// SSE differential fuzz on this (fresh) thread with the given codegen knobs.
fn sx_fuzz(seed: u64, programs: usize, k: jit::TestKnobs) {
    jit::test_set_knobs(k);
    let knobs = format!(
        "XMMREG {:?}, RETLIT {:?}, MEMCHK {:?}, PUSHPOP {:?}, BUDGET_BACKEDGE {:?}, budget cap {:?}, RAS {:?}, FLAGLAZY {:?}",
        k.xmmreg, k.retlit, k.memchk, k.pushpop, k.budget_be, k.budget_cap, k.ras, k.flaglazy
    );
    let mut r = Rng(seed | 1);
    let jit_before = jit::test_executed();
    for p in 0..programs {
        let c = arena_alloc(0x2000, 64);
        let h = c as u64 + SX_HOST as u64;
        let (code, entry_off) = gen_sse(&mut r, c as u64, h).expect("assemble");
        assert!(code.len() <= SX_HOST, "SSE program too large ({} bytes)", code.len());
        let d = arena_alloc(SX_DATA, 64);
        let mut init = vec![0u8; SX_DATA];
        sx_fill_pool(&mut r, &mut init[..SX_POOL as usize]);
        for b in init[SX_POOL as usize..OUT].iter_mut() {
            *b = r.next() as u8;
        }
        let entry = c as u64 + entry_off as u64;
        unsafe {
            core::ptr::write_bytes(c.add(SX_HOST), 0xCC, 16);
            crate::exe::register_native_va(h, win32_rt::ucrt::math::ceil as *const () as usize);
            if let Some(path) = std::env::var_os("AW2_JIT_FUZZ_TRACE") {
                let _ = std::fs::write(path, format!("[{knobs}] seed {seed:#x} program {p}\n{}", disasm(&code[entry_off..], entry)));
            }
            let interp = cr_run(c, &code, entry, d, &init, 2);
            let jit1 = cr_run(c, &code, entry, d, &init, 1);
            let jit2 = cr_run(c, &code, entry, d, &init, 1);
            crate::exe::unregister_native_va(h);
            for (name, got) in [("first", &jit1), ("linked", &jit2)] {
                if *got == interp {
                    continue;
                }
                let mut msg = format!("[{knobs}] seed {seed:#x} program {p}: translator ({name} run) differs from interpreter\n");
                if got.0 != interp.0 {
                    msg += &format!("  return value: jit {:#x} interp {:#x}\n", got.0, interp.0);
                }
                for q in 0..SX_DATA / 8 {
                    let x = u64::from_le_bytes(got.1[8 * q..8 * q + 8].try_into().unwrap());
                    let y = u64::from_le_bytes(interp.1[8 * q..8 * q + 8].try_into().unwrap());
                    if x != y {
                        let o = q * 8;
                        let what = if o >= OUT && o < OUT + 8 * 15 {
                            format!("reg {:?}", ALL64[(o - OUT) / 8])
                        } else if o == OUT + 8 * 15 {
                            "rflags".to_string()
                        } else if o >= SX_YMM as usize && o < SX_YMM as usize + 0x200 {
                            let v = o - SX_YMM as usize;
                            format!("ymm{} qword {}", v / 32, (v % 32) / 8)
                        } else {
                            format!("data+{o:#x}")
                        };
                        msg += &format!("  {what}: jit {x:#018x} interp {y:#018x}\n");
                    }
                }
                msg += &disasm(&code[entry_off..], entry);
                panic!("{msg}");
            }
        }
    }
    let ran = jit::test_executed() - jit_before;
    eprintln!("SSE fuzz [{knobs}]: {ran} guest instructions in translated code");
    assert!(ran >= 30 * programs as u64, "translator barely ran ({ran} instructions)");
}

/// Scalar / packed SSE and AVX arithmetic on NaN / denormal / ±0 / ±inf inputs, mixed
/// ss/sd/ps/pd moves and merges, conversions, compares, stores, generic vector ops and cache
/// barriers, guest calls and `ceil` host calls (call rel32 and call rax) with dirty cached
/// registers, side exits (bad addresses) and forced budget exits: the XMM register cache
/// (`AW2_JIT_XMMREG`) and return-address literals (`AW2_JIT_RETLIT`), each on and off, must
/// match the interpreter exactly.
#[test]
fn translator_sse_xmm_cache_matches_interpreter() {
    let n = jit::TestKnobs::NONE;
    let on = jit::TestKnobs {
        memchk: Some(15),
        pushpop: Some(true),
        budget_be: Some(true),
        xmmreg: Some(true),
        retlit: Some(true),
        ..n
    };
    let combos = [
        on,
        jit::TestKnobs { xmmreg: Some(false), ..on },
        jit::TestKnobs { retlit: Some(false), ..on },
        // jit6 codegen.
        jit::TestKnobs { flaglazy: Some(false), ..on },
        // jit5 codegen.
        jit::TestKnobs { xmmreg: Some(false), retlit: Some(false), flaglazy: Some(false), ..on },
        // jit4 codegen.
        jit::TestKnobs {
            memchk: Some(0),
            pushpop: Some(false),
            budget_be: Some(false),
            xmmreg: Some(false),
            retlit: Some(false),
            flaglazy: Some(false),
            ..n
        },
        jit::TestKnobs { budget_cap: Some(7), ..on },
        jit::TestKnobs { budget_cap: Some(2), ras: Some(false), ..on },
        jit::TestKnobs { budget_be: Some(false), budget_cap: Some(37), ..on },
        jit::TestKnobs { memchk: Some(0), ..on },
        jit::TestKnobs { flaglazy: Some(true), flags_dead: Some(3), budget_cap: Some(37), ..on },
    ];
    // Debugging: AW2_JIT_SX_ONLY=<combo> runs one combo, AW2_JIT_SX_SEED=<hex> fixes the seed.
    let only: Option<usize> = std::env::var("AW2_JIT_SX_ONLY").ok().and_then(|s| s.parse().ok());
    let seed: Option<u64> =
        std::env::var("AW2_JIT_SX_SEED").ok().and_then(|s| u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok());
    for (ci, &k) in combos.iter().enumerate() {
        if only.is_some_and(|o| o != ci) {
            continue;
        }
        let s = seed.unwrap_or(0x55E0_C0DE_0000_0001 + ci as u64 * 0x1_0000);
        std::thread::Builder::new()
            .stack_size(16 << 20)
            .spawn(move || sx_fuzz(s, 300, k))
            .unwrap()
            .join()
            .unwrap_or_else(|_| panic!("SSE fuzz failed (combo {ci})"));
    }
}

// --- lazy flags / last_bits (AW2_JIT_FLAGLAZY) ----------------------------------------------

const LZ_DATA: usize = 0x400;
/// Offset of the `host_mix8` stub inside each program's 0x2000-byte code area.
const LZ_HOST: usize = 0x1FF0;
/// Tracked GPRs (`last_bits` read by `abi_gpr`) and some untracked sources.
const LZ_T64: [AsmRegister64; 5] = [rax, rcx, rdx, r8, r9];
const LZ_T32: [AsmRegister32; 5] = [eax, ecx, edx, r8d, r9d];
const LZ_T16: [AsmRegister16; 5] = [ax, cx, dx, r8w, r9w];
const LZ_T8: [AsmRegister8; 5] = [al, cl, dl, r8b, r9b];
const LZ_S64: [AsmRegister64; 5] = [rbx, rsi, rdi, r10, r11];
const LZ_S32: [AsmRegister32; 5] = [ebx, esi, edi, r10d, r11d];
const LZ_S16: [AsmRegister16; 5] = [bx, si, di, r10w, r11w];
const LZ_S8: [AsmRegister8; 5] = [bl, sil, dil, r10b, r11b];

/// Load operand in the random input area `[0, 0x100)`.
fn lz_ld(r: &mut Rng, size: i32) -> AsmMemoryOperand {
    let d = r.below(0x100 - 8) as i32;
    r13 + if r.chance(75) { d / size * size } else { d }
}

/// Store operand in `[0x100, 0x200)`.
fn lz_st(r: &mut Rng, size: i32) -> AsmMemoryOperand {
    let d = r.below(0x100 - 8) as i32;
    r13 + 0x100 + if r.chance(75) { d / size * size } else { d }
}

/// Write all of CF/ZF/SF/OF/PF (after calls: flags may be dead there).
fn lz_flags_all(a: &mut CodeAssembler, r: &mut Rng) -> Result<(), IcedError> {
    match r.below(5) {
        0 => a.cmp(rsi, rdi),
        1 => a.add(rbx, rsi),
        2 => a.sub(rdi, rbx),
        3 => a.add(r10d, r11d),
        _ => a.cmp(bl, r.below(256) as i32),
    }
}

/// One write to a tracked GPR (rax, rcx, rdx, r8, r9) of a random width and kind: some set
/// `last_bits` (in both engines), some don't (cdq / cqo / cwd, cmov, shifts, high bytes, ...).
fn lz_write(a: &mut CodeAssembler, r: &mut Rng) -> Result<(), IcedError> {
    let t = r.below(5) as usize;
    let s = r.below(5) as usize;
    let u = r.below(5) as usize;
    match r.below(24) {
        0 => a.mov(LZ_T64[t], r.imm64()),
        1 => a.mov(LZ_T32[t], r.imm32()),
        2 => a.mov(LZ_T16[t], r.below(0x10000) as i32),
        3 => a.mov(LZ_T8[t], r.below(0x100) as i32),
        4 => a.mov(LZ_T8[t], LZ_S8[s]),
        5 => a.mov(LZ_T16[t], LZ_S16[s]),
        6 => a.mov(LZ_T32[t], LZ_T32[u]),
        7 => a.movzx(LZ_T32[t], LZ_S8[s]),
        8 => a.movzx(LZ_T32[t], word_ptr(lz_ld(r, 2))),
        9 => a.movsx(LZ_T64[t], byte_ptr(lz_ld(r, 1))),
        10 => a.lea(LZ_T64[t], ptr(rbx + LZ_T64[u] * 2 + r.below(64) as i32)),
        11 => setcc!(a, r.below(16), LZ_T8[t]),
        12 => a.xor(LZ_T32[t], LZ_T32[t]),
        13 => {
            let op = r.below(9);
            match r.below(6) {
                0 => alu!(a, op, LZ_T8[t], LZ_S8[s]),
                1 => alu!(a, op, LZ_T8[t], r.imm32() as i8 as i32),
                2 => alu!(a, op, LZ_T16[t], r.imm32() as i16 as i32),
                3 => alu!(a, op, LZ_T32[t], LZ_T32[u]),
                4 => alu!(a, op, LZ_T64[t], r.imm32()),
                _ => alu!(a, op, LZ_T64[t], LZ_S64[s]),
            }
        }
        14 => match r.below(4) {
            0 => a.mov(LZ_T8[t], byte_ptr(lz_ld(r, 1))),
            1 => a.mov(LZ_T16[t], word_ptr(lz_ld(r, 2))),
            2 => a.mov(LZ_T32[t], dword_ptr(lz_ld(r, 4))),
            _ => a.mov(LZ_T64[t], qword_ptr(lz_ld(r, 8))),
        },
        15 => match r.below(4) {
            0 => a.inc(LZ_T8[t]),
            1 => a.dec(LZ_T16[t]),
            2 => a.inc(LZ_T32[t]),
            _ => a.dec(LZ_T64[t]),
        },
        // Write rdx without setting last_bits.
        16 => match r.below(3) {
            0 => a.cdq(),
            1 => a.cqo(),
            _ => a.cwd(),
        },
        17 => {
            if r.chance(50) {
                cmov!(a, r.below(16), LZ_T32[t], LZ_S32[s])
            } else {
                cmov!(a, r.below(16), LZ_T64[t], LZ_S64[s])
            }
        }
        18 => {
            a.push(LZ_S64[s])?;
            a.pop(LZ_T64[t])
        }
        19 => match r.below(4) {
            0 => a.shl(LZ_T32[t], 1 + r.below(31) as i32),
            1 => a.ror(LZ_T16[t], 3),
            2 => a.sar(LZ_T8[t], 1 + r.below(7) as i32),
            _ => a.shl(LZ_T64[t], cl),
        },
        20 => match r.below(4) {
            0 => a.imul_3(LZ_T32[t], LZ_S32[s], r.imm32()),
            1 => a.neg(LZ_T8[t]),
            2 => a.not(LZ_T16[t]),
            _ => a.bswap(LZ_T32[t]),
        },
        // High-byte writes (abi_gpr then sees the full register).
        21 => match r.below(3) {
            0 => a.mov(ch, r.below(0x100) as i32),
            1 => a.mov(dh, bl),
            _ => a.add(ah, 3),
        },
        22 => match r.below(3) {
            0 => a.movsx(LZ_T32[t], LZ_S16[s]),
            1 => a.movzx(LZ_T64[t], LZ_T8[u]),
            _ => a.movsxd(LZ_T64[t], LZ_T32[u]),
        },
        _ => {
            if r.chance(50) {
                a.adc(LZ_T8[t], r.below(0x80) as i32)
            } else {
                a.sbb(LZ_T32[t], LZ_S32[s])
            }
        }
    }
}

/// Final write to rax before the return to the host (`abi_gpr(0)` is the return value);
/// leaves the flags alone (they are dumped).
fn lz_rax_final(a: &mut CodeAssembler, r: &mut Rng) -> Result<(), IcedError> {
    match r.below(8) {
        0 => a.mov(al, r.below(0x100) as i32),
        1 => a.mov(ax, r.below(0x10000) as i32),
        2 => a.mov(eax, r.imm32()),
        3 => setcc!(a, r.below(16), al),
        4 => a.mov(al, byte_ptr(lz_ld(r, 1))),
        5 => a.mov(ax, si),
        6 => a.mov(ah, bl),
        _ => Ok(()),
    }
}

/// An ALU / inc / dec result whose parity is read later (same block, across a block split
/// or not at all), with ops in between that leave the result register alone.
fn lz_parity(a: &mut CodeAssembler, r: &mut Rng) -> Result<(), IcedError> {
    let t = r.below(5) as usize;
    let s = r.below(5) as usize;
    let op = r.below(5);
    match r.below(9) {
        0 => alu!(a, op, LZ_T32[t], LZ_S32[s])?,
        1 => alu!(a, op, LZ_S64[s], LZ_T64[t])?,
        2 => alu!(a, op, LZ_T8[t], r.imm32() as i8 as i32)?,
        3 => alu!(a, op, LZ_S16[s], LZ_T16[t])?,
        4 => a.inc(LZ_T32[t])?,
        5 => a.dec(LZ_S8[s])?,
        6 => a.adc(LZ_T64[t], LZ_S64[s])?,
        7 => a.add(ah, bl)?,
        _ => a.sub(LZ_T64[t], qword_ptr(lz_ld(r, 8)))?,
    }
    for _ in 0..r.below(3) {
        match r.below(5) {
            0 => a.mov(qword_ptr(lz_st(r, 8)), LZ_T64[r.below(5) as usize])?,
            1 => a.mov(dword_ptr(lz_st(r, 4)), r.imm32())?,
            2 => a.mov(LZ_S64[r.below(5) as usize], qword_ptr(lz_ld(r, 8)))?,
            3 => a.lea(r12, ptr(rbx + rsi + 8))?,
            _ => {
                // Block split with the parity pending.
                let mut l = a.create_label();
                a.jmp(l)?;
                a.set_label(&mut l)?;
                a.nop()?;
            }
        }
    }
    match r.below(6) {
        0 => a.setp(byte_ptr(lz_st(r, 1))),
        1 => a.setnp(LZ_T8[r.below(5) as usize]),
        2 => a.cmovp(LZ_S32[r.below(5) as usize], LZ_T32[r.below(5) as usize]),
        3 => {
            let mut l = a.create_label();
            if r.chance(50) {
                a.jp(l)?;
            } else {
                a.jnp(l)?;
            }
            lz_write(a, r)?;
            a.set_label(&mut l)?;
            a.nop()
        }
        _ => Ok(()),
    }
}

/// Host call (`host_mix8`, `last_bits`-masked rcx / rdx / r8 / r9) after a few tracked-register
/// writes; flags are rewritten afterwards.
fn lz_host_call(a: &mut CodeAssembler, r: &mut Rng, h_mix: u64) -> Result<(), IcedError> {
    for _ in 0..r.below(4) {
        lz_write(a, r)?;
    }
    if r.chance(70) {
        a.call(h_mix)?;
    } else {
        a.mov(r11, h_mix as i64)?;
        a.call(r11)?;
    }
    a.mov(qword_ptr(r13 + 0x200 + 8 * r.below(16) as i32), rax)?;
    lz_flags_all(a, r)
}

/// One random program segment; `depth` bounds nesting (loops only at depth 0, diamonds
/// below 2).
fn lz_seg(a: &mut CodeAssembler, r: &mut Rng, leaves: &[CodeLabel], h_mix: u64, depth: u32) -> Result<(), IcedError> {
    match r.below(12) {
        0..=2 => {
            for _ in 0..1 + r.below(4) {
                lz_write(a, r)?;
            }
        }
        3 | 4 => lz_parity(a, r)?,
        5 | 6 if depth < 2 => {
            // Forward diamond / skip: the two edges reach blocks writing different registers.
            if r.chance(70) {
                lz_parity(a, r)?;
            } else {
                lz_flags_all(a, r)?;
            }
            let mut join = a.create_label();
            let cc = r.below(16);
            if r.chance(50) {
                let mut els = a.create_label();
                jcc!(a, cc, els)?;
                for _ in 0..1 + r.below(3) {
                    lz_seg(a, r, leaves, h_mix, depth + 1)?;
                }
                a.jmp(join)?;
                a.set_label(&mut els)?;
                a.nop()?;
                for _ in 0..1 + r.below(3) {
                    lz_seg(a, r, leaves, h_mix, depth + 1)?;
                }
            } else {
                jcc!(a, cc, join)?;
                for _ in 0..1 + r.below(3) {
                    lz_seg(a, r, leaves, h_mix, depth + 1)?;
                }
            }
            a.set_label(&mut join)?;
            a.nop()?;
        }
        7 if depth == 0 => {
            // Bounded inner loop; the exit edge sometimes reads the flags of the counter.
            a.mov(r14d, 1 + r.below(4) as u32)?;
            let mut top = a.create_label();
            a.set_label(&mut top)?;
            a.nop()?;
            for _ in 0..1 + r.below(4) {
                lz_seg(a, r, leaves, h_mix, 1)?;
            }
            match r.below(3) {
                0 => a.sub(r14d, 1)?,
                1 => a.dec(r14d)?,
                _ => a.add(r14d, -1)?,
            }
            a.jnz(top)?;
            if r.chance(40) {
                setcc!(a, r.below(16), byte_ptr(lz_st(r, 1)))?;
            }
        }
        8 => lz_host_call(a, r, h_mix)?,
        9 if !leaves.is_empty() => {
            for _ in 0..r.below(3) {
                lz_write(a, r)?;
            }
            a.call(leaves[r.below(leaves.len() as u64) as usize])?;
            lz_flags_all(a, r)?;
        }
        10 => {
            for _ in 0..1 + r.below(3) {
                a.mov(qword_ptr(lz_st(r, 8)), LZ_T64[r.below(5) as usize])?;
            }
        }
        _ => {
            lz_write(a, r)?;
            let mut l = a.create_label();
            a.jmp(l)?;
            a.set_label(&mut l)?;
            a.nop()?;
        }
    }
    Ok(())
}

/// Random program for the lazy flags / `last_bits` fuzz assembled at `base`: partial and full
/// writes of rax / rcx / rdx / r8 / r9, forward branches whose edges write different
/// registers, parity read across block splits and branches, bounded loops, guest leaf calls
/// and `host_mix8` calls with partial-register arguments; ends with a partial rax write (the
/// return value is `abi_gpr(0)`) and a register / rflags dump at `OUT`.
fn gen_lazy(r: &mut Rng, base: u64, h_mix: u64) -> Result<(Vec<u8>, usize), IcedError> {
    let mut a = CodeAssembler::new(64)?;
    let mut leaves: Vec<CodeLabel> = (0..3).map(|_| a.create_label()).collect();
    let mut entry = a.create_label();
    a.set_label(&mut entry)?;
    a.sub(rsp, 0x58)?;
    a.mov(r13, rcx)?;
    for &reg in &[rax, rcx, rdx, rbx, rsi, rdi, r8, r9, r10, r11, r12] {
        a.mov(reg, r.imm64())?;
    }
    // Shadow space and stack arguments of host_mix8 (also as seen from a leaf).
    for k in 0..11i32 {
        a.mov(qword_ptr(rsp + 8 * k), r.imm32())?;
    }
    let fl = (r.next() as u32) & 0x8C5;
    a.push(fl as i32)?;
    a.popfq()?;
    a.mov(r15d, 1 + r.below(3) as u32)?;
    let mut top = a.create_label();
    a.set_label(&mut top)?;
    a.nop()?;
    for _ in 0..4 + r.below(10) {
        lz_seg(&mut a, r, &leaves, h_mix, 0)?;
    }
    a.sub(r15d, 1)?;
    a.jnz(top)?;
    lz_rax_final(&mut a, r)?;
    for (k, &reg) in ALL64.iter().enumerate() {
        a.mov(qword_ptr(r13 + (OUT + 8 * k) as i32), reg)?;
    }
    a.pushfq()?;
    a.pop(rdx)?;
    a.mov(qword_ptr(r13 + (OUT + 8 * 15) as i32), rdx)?;
    a.add(rsp, 0x58)?;
    a.ret()?;

    // Leaves (flags are dead at entry): no further calls to leaves.
    for l in leaves.iter_mut() {
        a.set_label(l)?;
        lz_flags_all(&mut a, r)?;
        for _ in 0..1 + r.below(4) {
            lz_seg(&mut a, r, &[], h_mix, 1)?;
        }
        if r.chance(50) {
            lz_rax_final(&mut a, r)?;
        }
        a.ret()?;
    }
    let code = a.assemble(base)?;
    Ok((code, 0))
}

/// Lazy flags / `last_bits` differential fuzz on this (fresh) thread.
fn lz_fuzz(seed: u64, programs: usize, k: jit::TestKnobs) {
    jit::test_set_knobs(k);
    let knobs = format!(
        "FLAGLAZY {:?}, FLAGS_DEAD_AT_CALL {:?}, MEMCHK {:?}, PUSHPOP {:?}, BUDGET_BACKEDGE {:?}, budget cap {:?}, RAS {:?}, XMMREG {:?}, RETLIT {:?}",
        k.flaglazy, k.flags_dead, k.memchk, k.pushpop, k.budget_be, k.budget_cap, k.ras, k.xmmreg, k.retlit
    );
    let mut r = Rng(seed | 1);
    let jit_before = jit::test_executed();
    for p in 0..programs {
        let c = arena_alloc(0x2000, 64);
        let h = c as u64 + LZ_HOST as u64;
        // Regenerate the (rare) programs that don't fit below the host stub.
        let (code, entry_off) = loop {
            let g = gen_lazy(&mut r, c as u64, h).expect("assemble");
            if g.0.len() <= LZ_HOST {
                break g;
            }
        };
        let d = arena_alloc(LZ_DATA, 64);
        let mut init = vec![0u8; LZ_DATA];
        for b in init[..OUT].iter_mut() {
            *b = r.next() as u8;
        }
        let entry = c as u64 + entry_off as u64;
        unsafe {
            core::ptr::write_bytes(c.add(LZ_HOST), 0xCC, 16);
            crate::exe::register_native_va(h, host_mix8 as *const () as usize);
            if let Some(path) = std::env::var_os("AW2_JIT_FUZZ_TRACE") {
                let _ = std::fs::write(path, format!("[{knobs}] seed {seed:#x} program {p}\n{}", disasm(&code[entry_off..], entry)));
            }
            let interp = cr_run(c, &code, entry, d, &init, 2);
            let jit1 = cr_run(c, &code, entry, d, &init, 1);
            let jit2 = cr_run(c, &code, entry, d, &init, 1);
            crate::exe::unregister_native_va(h);
            for (name, got) in [("first", &jit1), ("linked", &jit2)] {
                if *got == interp {
                    continue;
                }
                let mut msg = format!("[{knobs}] seed {seed:#x} program {p}: translator ({name} run) differs from interpreter\n");
                if got.0 != interp.0 {
                    msg += &format!("  return value (abi_gpr(0)): jit {:#x} interp {:#x}\n", got.0, interp.0);
                }
                for q in 0..LZ_DATA / 8 {
                    let x = u64::from_le_bytes(got.1[8 * q..8 * q + 8].try_into().unwrap());
                    let y = u64::from_le_bytes(interp.1[8 * q..8 * q + 8].try_into().unwrap());
                    if x != y {
                        let o = q * 8;
                        let what = if (OUT..OUT + 8 * 15).contains(&o) {
                            format!("reg {:?}", ALL64[(o - OUT) / 8])
                        } else if o == OUT + 8 * 15 {
                            "rflags".to_string()
                        } else if (0x200..0x280).contains(&o) {
                            format!("host_mix8 result slot {}", (o - 0x200) / 8)
                        } else {
                            format!("data+{o:#x}")
                        };
                        msg += &format!("  {what}: jit {x:#018x} interp {y:#018x}\n");
                    }
                }
                msg += &disasm(&code[entry_off..], entry);
                panic!("{msg}");
            }
        }
    }
    let ran = jit::test_executed() - jit_before;
    eprintln!("lazy-flags fuzz [{knobs}]: {ran} guest instructions in translated code");
    assert!(ran >= 30 * programs as u64, "translator barely ran ({ran} instructions)");
}

/// Partial / full writes of the `abi_gpr` registers feeding host calls and the return value,
/// per-edge flag and `last_bits` flushes of forward branches and loop back-edges (successor
/// blocks overwriting `last_bits` first), parity deferred across block splits / branches and
/// flushed by readers, guest leaf calls and forced budget exits: `AW2_JIT_FLAGLAZY` on and off
/// must match the interpreter exactly (return value, registers, rflags and memory).
#[test]
fn translator_lazy_flags_last_bits_match_interpreter() {
    let n = jit::TestKnobs::NONE;
    let on = jit::TestKnobs {
        memchk: Some(15),
        pushpop: Some(true),
        budget_be: Some(true),
        xmmreg: Some(true),
        retlit: Some(true),
        flaglazy: Some(true),
        ..n
    };
    let combos = [
        on,
        // jit6 codegen.
        jit::TestKnobs { flaglazy: Some(false), ..on },
        jit::TestKnobs { flags_dead: Some(3), ..on },
        jit::TestKnobs { flags_dead: Some(0), ..on },
        jit::TestKnobs { budget_cap: Some(7), ..on },
        jit::TestKnobs { budget_cap: Some(2), ras: Some(false), ..on },
        jit::TestKnobs { flaglazy: Some(false), flags_dead: Some(3), budget_cap: Some(37), ..on },
        jit::TestKnobs {
            memchk: Some(0),
            pushpop: Some(false),
            budget_be: Some(false),
            xmmreg: Some(false),
            retlit: Some(false),
            flaglazy: Some(true),
            ..n
        },
    ];
    // Debugging: AW2_JIT_LZ_ONLY=<combo> runs one combo, AW2_JIT_LZ_SEED=<hex> fixes the seed.
    let only: Option<usize> = std::env::var("AW2_JIT_LZ_ONLY").ok().and_then(|s| s.parse().ok());
    let seed: Option<u64> =
        std::env::var("AW2_JIT_LZ_SEED").ok().and_then(|s| u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok());
    for (ci, &k) in combos.iter().enumerate() {
        if only.is_some_and(|o| o != ci) {
            continue;
        }
        let s = seed.unwrap_or(0x1A2F_1A65_0000_0001 + ci as u64 * 0x1_0000);
        std::thread::Builder::new()
            .stack_size(16 << 20)
            .spawn(move || lz_fuzz(s, 300, k))
            .unwrap()
            .join()
            .unwrap_or_else(|_| panic!("lazy-flags fuzz failed (combo {ci})"));
    }
}

// --- AOT translation cache (`AW2_DLL_AOT`, aot.rs / jit_aot.rs) ----------------------------

/// First difference between a translated-code run and the interpreter's (None: identical).
fn first_diff(got: &(u64, Vec<u8>), want: &(u64, Vec<u8>)) -> Option<String> {
    if got.0 != want.0 {
        return Some(format!("return value: got {:#x} interp {:#x}", got.0, want.0));
    }
    (0..want.1.len() / 8).find_map(|k| {
        let x = u64::from_le_bytes(got.1[8 * k..8 * k + 8].try_into().unwrap());
        let y = u64::from_le_bytes(want.1[8 * k..8 * k + 8].try_into().unwrap());
        (x != y).then(|| format!("data+{:#x}: got {x:#018x} interp {y:#018x}", 8 * k))
    })
}

/// The host-call program, translated in AOT emission mode from the blocks the JIT
/// translated, round-tripped through the cache file format, installed (with load-time
/// verification) and run from the shared AOT code: must match the interpreter, with
/// host-fn relocations applied and no block invalidated by an unrelated dispatch change.
#[test]
fn aot_roundtrip_host_calls_match_interpreter() {
    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(aot_host_calls_default)
        .unwrap()
        .join()
        .unwrap_or_else(|_| panic!("AOT host-call round trip failed"));
}

/// [`aot_roundtrip_host_calls_match_interpreter`] with the `lookup_*` stubs probing the
/// shared AOT table (`AW2_DLL_AOT_PROBE`).
#[test]
fn aot_roundtrip_host_calls_probe_match_interpreter() {
    std::thread::Builder::new()
        .stack_size(16 << 20)
        .spawn(|| {
            jit::test_set_knobs(jit::TestKnobs { aot_probe: Some(true), ..jit::TestKnobs::NONE });
            aot_host_calls(SlotAtLoad::Moved)
        })
        .unwrap()
        .join()
        .unwrap_or_else(|_| panic!("AOT host-call round trip (probe) failed"));
}

/// [`aot_roundtrip_host_calls_match_interpreter`] with the import slot holding a value that
/// is not inline-callable at load (a guest thunk jumping to the builtin, like a delay-load
/// resolver before the first call): the blocks calling through it stay live (their guarded
/// call takes the generic path), whether the value recorded at build time is still
/// inline-callable or not, and stay correct once the slot changes again.
#[test]
fn aot_roundtrip_host_calls_held_slot_match_interpreter() {
    for (what, v) in [("recorded value callable", SlotAtLoad::HeldKeep), ("recorded value gone", SlotAtLoad::HeldDisabled)] {
        std::thread::Builder::new()
            .stack_size(16 << 20)
            .spawn(move || aot_host_calls(v))
            .unwrap()
            .join()
            .unwrap_or_else(|_| panic!("AOT host-call round trip with a held import slot failed ({what})"));
    }
}

/// What the host-call program's import slot holds when its AOT image is installed.
#[derive(Clone, Copy, PartialEq)]
enum SlotAtLoad {
    /// Another VA bound to the same builtin (relocations follow it).
    Moved,
    /// A guest thunk; the builtin VA recorded at build time is still inline-callable.
    HeldKeep,
    /// A guest thunk; the VA recorded at build time is no longer bound to anything.
    HeldDisabled,
}

fn aot_host_calls_default() {
    aot_host_calls(SlotAtLoad::Moved)
}

fn aot_host_calls(v: SlotAtLoad) {
    let h = arena_alloc(144, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 144) };
    let h_memset = h as u64;
    let (h_mix, h_reent, h_ceil, h_malloc, h_free, h_dummy) =
        (h_memset + 16, h_memset + 32, h_memset + 48, h_memset + 64, h_memset + 80, h_memset + 96);
    let natives = [
        (h_memset, win32_rt::msvcrt::memset as *const () as usize),
        (h_mix, host_mix8 as *const () as usize),
        (h_reent, host_reenter as *const () as usize),
        (h_ceil, win32_rt::ucrt::math::ceil as *const () as usize),
        (h_malloc, win32_rt::ucrt::heap::malloc as *const () as usize),
        (h_free, win32_rt::ucrt::heap::free as *const () as usize),
    ];
    for &(va, f) in &natives {
        crate::exe::register_native_va(va, f);
    }
    let c = arena_alloc(0x1000, 16);
    let code = gen_host_calls(c as u64, [h_memset, h_mix, h_reent, h_ceil, h_malloc, h_free]).expect("assemble");
    let d = arena_alloc(DATA, 64);
    let mut init = vec![0u8; DATA];
    let mut r = Rng(0x7357_4057_CA11_A070);
    for b in init.iter_mut() {
        *b = r.next() as u8;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        let entry = c.add(16);
        let interp = run_mode(entry, d, &init, 2);
        jit::test_dump_start();
        let jit1 = run_mode(entry, d, &init, 1);
        let ips: Vec<u64> = jit::test_dump_take().iter().map(|b| b.ip).collect();
        assert_eq!(first_diff(&jit1, &interp), None, "JIT run");
        let mut uniq = ips.clone();
        uniq.sort_unstable();
        uniq.dedup();
        // Moved: between "build" and "load" the import slot moves to another VA bound to the
        // same builtin (what a host-side thunk VA does across processes): its relocations
        // must follow the slot instead of dropping the blocks as stale.
        // Held*: at load the slot holds a guest thunk (`jmp [rip+0]` to the builtin): not
        // inline-callable, so the guarded call takes the generic path. HeldDisabled builds
        // with the slot on a builtin VA that is unbound before the load.
        let h_memset2 = h_memset + 112;
        let h_memset3 = h_memset + 128;
        crate::exe::register_native_va(h_memset2, win32_rt::msvcrt::memset as *const () as usize);
        let thunk = arena_alloc(16, 16);
        core::ptr::copy_nonoverlapping([0xFFu8, 0x25, 0, 0, 0, 0].as_ptr(), thunk, 6);
        core::ptr::write_unaligned(thunk.add(6) as *mut u64, h_memset);
        if v == SlotAtLoad::HeldDisabled {
            crate::exe::register_native_va(h_memset3, win32_rt::msvcrt::memset as *const () as usize);
            core::ptr::write_unaligned(c as *mut u64, h_memset3);
        }
        let (rebased0, held0) = (super::aot::test_rebased(), super::aot::test_held());
        let (live, relocs, links) = super::aot::test_install(&ips, || match v {
            SlotAtLoad::Moved => core::ptr::write_unaligned(c as *mut u64, h_memset2),
            SlotAtLoad::HeldKeep => core::ptr::write_unaligned(c as *mut u64, thunk as u64),
            SlotAtLoad::HeldDisabled => {
                crate::exe::unregister_native_va(h_memset3);
                core::ptr::write_unaligned(c as *mut u64, thunk as u64);
            }
        });
        let rebased = super::aot::test_rebased() - rebased0;
        let held = super::aot::test_held() - held0;
        eprintln!(
            "aot host calls: {} blocks translated by the JIT, {live} live AOT blocks, {relocs} relocations ({rebased} rebased to the moved import slot), {held} blocks with a held import slot, {links} link sites",
            uniq.len()
        );
        assert_eq!(live, uniq.len(), "AOT blocks dropped at install (verify mismatch / stale assumption / stale import slot)");
        if jit::aot_probe_enabled() {
            if let Err(e) = super::aot::test_probe_check() {
                panic!("probe table out of sync after install: {e}");
            }
        }
        assert!(!jit::plaincall_enabled() || relocs > 0, "no host-fn relocations");
        if v == SlotAtLoad::Moved {
            assert!(!jit::plaincall_enabled() || rebased >= 2, "import-slot relocations not rebased ({rebased})");
            assert_eq!(held, 0, "import slot held although its value moved to an equivalent builtin");
        } else {
            assert!(!jit::plaincall_enabled() || held >= 1, "import slot holding a thunk not held ({held})");
        }
        // Unrelated dispatch change: flushes this thread's cache (so the blocks now come
        // from the AOT image) and must not invalidate any of them.
        crate::exe::register_native_va(h_dummy, host_mix8 as *const () as usize);
        let (before, hits0) = (jit::test_aot_installs(), jit::test_probe_hits());
        let aot1 = run_mode(entry, d, &init, 1);
        let aot2 = run_mode(entry, d, &init, 1);
        // Held*: the slot is resolved to the builtin (the recorded one for HeldKeep: the
        // inline path applies again; HeldDisabled's guard never matches).
        let aot3 = if v == SlotAtLoad::Moved {
            None
        } else {
            core::ptr::write_unaligned(c as *mut u64, h_memset);
            Some(run_mode(entry, d, &init, 1))
        };
        let installs = jit::test_aot_installs() - before;
        let hits = jit::test_probe_hits() - hits0;
        eprintln!("aot host calls: {installs} AOT installs, {hits} probe-table hits");
        let dead = super::aot::test_dead();
        super::aot::test_uninstall();
        crate::exe::unregister_native_va(h_dummy);
        crate::exe::unregister_native_va(h_memset2);
        for &(va, _) in &natives {
            crate::exe::unregister_native_va(va);
        }
        for (name, got) in [("first AOT", Some(&aot1)), ("second AOT", Some(&aot2)), ("AOT after the slot changed", aot3.as_ref())] {
            if let Some(m) = got.and_then(|g| first_diff(g, &interp)) {
                panic!("{name} run: {m}\n{}", disasm(&code[16..], c as u64 + 16));
            }
        }
        assert_eq!(dead, 0, "AOT blocks invalidated by an unrelated dispatch change");
        assert!(installs > 0, "AOT code not used ({live} blocks)");
    }
}

/// Call/ret programs (see [`cr_fuzz`]) run from AOT images of their JIT-translated blocks,
/// then with one block's IP made host-callable (its assumption breaks: revalidation must
/// kill it and unpatch the AOT links into it) and restored (the dead block is JIT-translated
/// again, the rest still AOT): every run must match the interpreter.
fn aot_cr_fuzz(seed: u64, programs: usize, k: jit::TestKnobs) {
    jit::test_set_knobs(k);
    let knobs = format!(
        "RAS {:?}, FLAGS_DEAD_AT_CALL {:?}, RO_PLAIN {:?}, MEMCHK {:?}, PUSHPOP {:?}, BUDGET_BACKEDGE {:?}, budget cap {:?}, XMMREG {:?}, RETLIT {:?}, FLAGLAZY {:?}, AOT_PROBE {:?}",
        k.ras, k.flags_dead, k.ro_plain, k.memchk, k.pushpop, k.budget_be, k.budget_cap, k.xmmreg, k.retlit, k.flaglazy, k.aot_probe
    );
    let probe = jit::aot_probe_enabled();
    let probe_check = |what: &str, p: usize| {
        if probe {
            if let Err(e) = super::aot::test_probe_check() {
                panic!("[{knobs}] seed {seed:#x} program {p}: probe table out of sync {what}: {e}");
            }
        }
    };
    let probe_hits0 = jit::test_probe_hits();
    let mut r = Rng(seed | 1);
    let dummy = arena_alloc(16, 16);
    unsafe { core::ptr::write_bytes(dummy, 0xCC, 16) };
    let dummy = dummy as u64;
    let (mut blocks, mut links, mut installs, mut kills, mut kill_progs, mut victims, mut revived) = (0usize, 0usize, 0u64, 0usize, 0usize, 0usize, 0usize);
    let mut revive_kills = 0usize;
    for p in 0..programs {
        let c = arena_alloc(0x1000, 64);
        let h = c as u64 + CR_HOST as u64;
        let (code, entry_off) = gen_callret(&mut r, c as u64, h).expect("assemble");
        assert!(code.len() <= CR_HOST, "call/ret program too large ({} bytes)", code.len());
        let d = arena_alloc(CR_DATA, 64);
        let mut init = vec![0u8; CR_DATA];
        for b in init.iter_mut().take(0x100) {
            *b = r.next() as u8;
        }
        let entry = c as u64 + entry_off as u64;
        let fail = |what: &str, m: String| {
            format!("[{knobs}] seed {seed:#x} program {p}: {what} differs from the interpreter: {m}\n{}", disasm(&code[entry_off..], entry))
        };
        unsafe {
            core::ptr::write_bytes(c.add(CR_HOST), 0xCC, 16);
            crate::exe::register_native_va(h, host_reenter as *const () as usize);
            jit::test_set_ro(vec![(c as u64, c as u64 + CR_POOL as u64)]);
            let interp = cr_run(c, &code, entry, d, &init, 2);
            jit::test_dump_start();
            let jit1 = cr_run(c, &code, entry, d, &init, 1);
            let jit2 = cr_run(c, &code, entry, d, &init, 1);
            let ips: Vec<u64> = jit::test_dump_take().iter().map(|b| b.ip).collect();
            for (name, got) in [("JIT (first run)", &jit1), ("JIT (linked run)", &jit2)] {
                if let Some(m) = first_diff(got, &interp) {
                    panic!("{}", fail(name, m));
                }
            }
            let mut uniq = ips.clone();
            uniq.sort_unstable();
            uniq.dedup();
            let (live, _, nl) = super::aot::test_install(&uniq, || {});
            assert_eq!(live, uniq.len(), "[{knobs}] seed {seed:#x} program {p}: AOT blocks dropped at install");
            probe_check("after install", p);
            blocks += live;
            links += nl;
            // Flush this thread's cache: the blocks now come from the AOT image.
            crate::exe::register_native_va(dummy, host_reenter as *const () as usize);
            let before = jit::test_aot_installs();
            let aot1 = cr_run(c, &code, entry, d, &init, 1);
            let aot2 = cr_run(c, &code, entry, d, &init, 1);
            installs += jit::test_aot_installs() - before;
            for (name, got) in [("AOT (first run)", &aot1), ("AOT (second run)", &aot2)] {
                if let Some(m) = first_diff(got, &interp) {
                    panic!("{}", fail(name, m));
                }
            }
            assert_eq!(super::aot::test_dead(), 0, "[{knobs}] seed {seed:#x} program {p}: AOT blocks invalidated by an unrelated dispatch change");
            // Break one block's host-callable assumption, revalidate, restore.
            let cand: Vec<u64> = uniq.iter().copied().filter(|&ip| ip != entry).collect();
            if !cand.is_empty() {
                victims += 1;
                let victim = cand[r.below(cand.len() as u64) as usize];
                crate::exe::register_native_va(victim, host_reenter as *const () as usize);
                super::aot::revalidate(crate::exe::va_gen());
                let dead = super::aot::test_dead();
                kills += dead;
                kill_progs += (dead > 0) as usize;
                probe_check("after invalidation", p);
                // Odd programs: the killed blocks stay dead (revival off on this thread), so the
                // runs below mix JIT translations of them with AOT code. Even programs: the
                // assumption holds again after unregistering, so they must come back
                // (AW2_DLL_AOT_REVIVE) and the runs are AOT again.
                let keep_dead = p % 2 == 1;
                super::aot::test_set_revive(!keep_dead);
                crate::exe::unregister_native_va(victim);
                if !keep_dead {
                    super::aot::revalidate(crate::exe::va_gen());
                    let left = super::aot::test_dead();
                    probe_check("after revival", p);
                    revive_kills += dead;
                    revived += dead - left.min(dead);
                    if super::aot::revive_enabled() {
                        assert_eq!(left, 0, "[{knobs}] seed {seed:#x} program {p}: {left} of {dead} invalidated blocks not revived (victim {victim:#x})");
                    }
                }
                let aot3 = cr_run(c, &code, entry, d, &init, 1);
                let aot4 = cr_run(c, &code, entry, d, &init, 1);
                for (name, got) in [("AOT after invalidation (first run)", &aot3), ("AOT after invalidation (second run)", &aot4)] {
                    if let Some(m) = first_diff(got, &interp) {
                        panic!("{}", fail(name, format!("{m} (victim block {victim:#x})")));
                    }
                }
                super::aot::test_set_revive(true);
            }
            super::aot::test_uninstall();
            crate::exe::unregister_native_va(dummy);
            crate::exe::unregister_native_va(h);
        }
    }
    jit::test_set_ro(Vec::new());
    let probe_hits = jit::test_probe_hits() - probe_hits0;
    eprintln!(
        "aot call/ret fuzz [{knobs}]: {blocks} AOT blocks, {links} link sites, {installs} AOT installs, {probe_hits} probe-table hits, {kills} blocks invalidated in {kill_progs}/{victims} programs, {revived}/{revive_kills} revived"
    );
    // The stubs' probe serves index misses only with the knob on (shared code reached
    // without an install).
    assert!(probe || probe_hits == 0, "probe-table hits with AW2_DLL_AOT_PROBE off ({probe_hits})");
    assert!(!probe || probe_hits > 0, "probe table never served a lookup ({installs} installs of {blocks} blocks)");
    assert!(!super::aot::revive_enabled() || revived == revive_kills, "invalidated blocks not revived ({revived}/{revive_kills})");
    // AOT->AOT links are patched at install: linked successors run without an index miss.
    assert!(installs + probe_hits >= 2 * programs as u64, "AOT code barely used ({installs} installs + {probe_hits} probe-table hits of {blocks} blocks)");
    assert!(links > 0, "no AOT link sites");
    assert!(kills > 0 && 2 * kill_progs >= victims, "broken assumptions rarely invalidated AOT blocks ({kill_progs}/{victims} programs)");
}

/// [`aot_cr_fuzz`] under the default codegen and with each jit5-jit7 knob off (each on a
/// fresh thread: the RAS decision and the AOT key are per-thread here).
#[test]
fn aot_call_ret_matches_interpreter() {
    let n = jit::TestKnobs::NONE;
    let on = jit::TestKnobs { memchk: Some(15), pushpop: Some(true), budget_be: Some(true), ..n };
    let all = [
        n,
        jit::TestKnobs { ras: Some(false), ..n },
        jit::TestKnobs { retlit: Some(false), ..n },
        jit::TestKnobs { flaglazy: Some(false), ..n },
        jit::TestKnobs { memchk: Some(0), ..n },
        jit::TestKnobs { xmmreg: Some(false), ..n },
        jit::TestKnobs { budget_cap: Some(37), ..on },
        jit::TestKnobs { ras: Some(false), flags_dead: Some(0), ro_plain: Some(false), budget_cap: Some(5), ..on },
        // AW2_DLL_AOT_PROBE: index misses served from the shared table by the stubs.
        jit::TestKnobs { aot_probe: Some(true), ..n },
        jit::TestKnobs { aot_probe: Some(true), ras: Some(false), ..n },
        jit::TestKnobs { aot_probe: Some(true), budget_cap: Some(37), ..on },
    ];
    for (ci, &k) in all.iter().enumerate() {
        std::thread::Builder::new()
            .stack_size(16 << 20)
            .spawn(move || aot_cr_fuzz(0xA07C_2E70_0000_0001 + ci as u64 * 0x1_0000, 60, k))
            .unwrap()
            .join()
            .unwrap_or_else(|_| panic!("AOT call/ret fuzz failed (combo {ci})"));
    }
}

// --- mapped-DLL import stubs (g23): DF_IAT call-time dispatch + run-loop route --------------

unsafe extern "C" fn iat_host_a(x: u64) -> u64 {
    x.wrapping_add(100)
}

unsafe extern "C" fn iat_host_b(x: u64) -> u64 {
    x.wrapping_add(200)
}

/// Fake 4 KiB DLL image (registered with IAT directory `[+0x800, +0x840)`):
/// - `+0x100` stub A `FF 25` -> slot `+0x800`; `+0x110` stub B `48 FF 25` -> slot `+0x808`;
/// - `+0x120` stub C `FF 25` -> slot `+0x900` (outside the IAT directory: delay-load style);
/// - `+0x130` stub D `FF 25` -> slot `+0x810`; `+0x140` stub E `FF 25` -> slot `+0x818` (0);
/// - `+0x200` `mov eax, 7; ret`, `+0x208` `mov eax, 9; ret` (guest slot targets).
/// Returns the image base.
fn fake_iat_image() -> u64 {
    let img = arena_alloc(0x1000, 0x1000);
    let b = img as u64;
    unsafe {
        core::ptr::write_bytes(img, 0xCC, 0x800);
        core::ptr::write_bytes(img.add(0x800), 0, 0x800);
        let stub = |at: usize, rex: bool, slot: usize| {
            let mut p = img.add(at);
            if rex {
                *p = 0x48;
                p = p.add(1);
            }
            *p = 0xFF;
            *p.add(1) = 0x25;
            let next = p.add(6) as i64;
            let disp = (img.add(slot) as i64 - next) as i32;
            core::ptr::write_unaligned(p.add(2) as *mut i32, disp);
        };
        stub(0x100, false, 0x800);
        stub(0x110, true, 0x808);
        stub(0x120, false, 0x900);
        stub(0x130, false, 0x810);
        stub(0x140, false, 0x818);
        core::ptr::copy_nonoverlapping([0xB8u8, 7, 0, 0, 0, 0xC3].as_ptr(), img.add(0x200), 6);
        core::ptr::copy_nonoverlapping([0xB8u8, 9, 0, 0, 0, 0xC3].as_ptr(), img.add(0x208), 6);
    }
    crate::dll_native::register_iat_dir(b as usize, 0x1000, (b as usize + 0x800, b as usize + 0x840));
    b
}

unsafe fn set_slot(img: u64, off: u64, v: u64) {
    core::ptr::write_volatile((img + off) as *mut u64, v);
}

/// `resolve_dispatch` / `call_va8` through a DLL import stub use the slot's value at call
/// time: a rewritten slot is seen on the next call without any dispatch-generation bump (no
/// stale memo), the REX form resolves too, a slot outside the IAT directory is refused
/// (named), and the translator's bake-time lookup never sees through a stub.
#[test]
fn dll_iat_stub_dispatch_reads_the_slot_at_call_time() {
    let img = fake_iat_image();
    let h = arena_alloc(48, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 48) };
    let (hva, hvb, hmemset) = (h as u64, h as u64 + 16, h as u64 + 32);
    crate::exe::register_native_va(hva, iat_host_a as *const () as usize);
    crate::exe::register_native_va(hvb, iat_host_b as *const () as usize);
    crate::exe::register_native_va(hmemset, win32_rt::msvcrt::memset as *const () as usize);
    let (stub_a, stub_b, stub_c) = (img + 0x100, img + 0x110, img + 0x120);
    {
        let _hold = crate::exe::hold_va_gen();
        unsafe {
            set_slot(img, 0x800, hva);
            set_slot(img, 0x808, hva);
            let (ha, fa) = resolve_dispatch(stub_a);
            assert_eq!(ha, iat_host_a as *const () as usize, "stub A -> slot target A");
            assert!(fa & DF_HOST != 0 && fa & DF_IAT == 0, "flags {fa:#x}: the target's own");
            // The memo holds the slot address, not the target.
            let (hs, f_static) = resolve_dispatch_static(stub_a);
            assert_eq!((hs, f_static & DF_IAT), (img as usize + 0x800, DF_IAT));
            assert_eq!(call_va8(stub_a, 5, 0, 0, 0, 0, 0, 0, 0), 105);

            // Slot rewritten at run time: the next call follows it, with no generation bump.
            let g0 = crate::exe::va_gen();
            set_slot(img, 0x800, hvb);
            assert_eq!(resolve_dispatch(stub_a).0, iat_host_b as *const () as usize, "rewritten slot");
            assert_eq!(call_va8(stub_a, 5, 0, 0, 0, 0, 0, 0, 0), 205);
            set_slot(img, 0x800, hva);
            assert_eq!(call_va8(stub_a, 5, 0, 0, 0, 0, 0, 0, 0), 105, "rewritten back");
            assert_eq!(crate::exe::va_gen(), g0, "no dispatch-generation bump was needed");

            // `48 FF 25` form.
            assert_eq!(resolve_dispatch(stub_b).0, iat_host_a as *const () as usize, "REX stub");

            // Slot outside the IAT directory (delay-load style): refused, guest code.
            set_slot(img, 0x900, hva);
            assert_eq!(resolve_dispatch(stub_c).0, 0, "delay-load-style slot must not resolve");
            assert_eq!(
                crate::dll_native::iat_stub_slot_checked(stub_c),
                Err(crate::dll_native::StubRefusal::SlotOutsideIat)
            );
            // Not a stub at all.
            assert_eq!(crate::dll_native::iat_stub_slot_checked(img + 0x200), Ok(None));

            // The translator bakes inline host calls: never through a stub, even when the slot
            // currently holds an inline-callable builtin.
            set_slot(img, 0x800, hmemset);
            if jit::inline_host_fn(hmemset).is_some() {
                assert_eq!(jit::inline_host_fn(stub_a), None, "inline call baked through a stub");
            }
            assert_eq!(resolve_dispatch(stub_a).0, win32_rt::msvcrt::memset as *const () as usize);
        }
    }
    for va in [hva, hvb, hmemset] {
        crate::exe::unregister_native_va(va);
    }
}

/// A stub the emulator reaches is routed at its block start to the slot target read then
/// (guest target: plain jump; host target: host tail call), interpreted and translated; a
/// slot rewrite is seen by the next run; refused stubs (slot outside the IAT directory,
/// unbound slot) execute as guest code and are not counted.
#[test]
fn dll_iat_stub_guest_target_routes_in_the_run_loop() {
    let img = fake_iat_image();
    let h = arena_alloc(16, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 16) };
    let hva = h as u64;
    crate::exe::register_native_va(hva, iat_host_a as *const () as usize);
    let (stub_a, stub_c, stub_d, stub_e) = (img + 0x100, img + 0x120, img + 0x130, img + 0x140);
    unsafe {
        set_slot(img, 0x800, hva);
        set_slot(img, 0x810, img + 0x200);
        set_slot(img, 0x900, img + 0x200);
        set_slot(img, 0x818, 0);
        // Guest slot target: no host callable, the caller runs the emulator at the stub.
        assert_eq!(resolve_dispatch(stub_d).0, 0);
        for (mode, name) in [(2u8, "interpreter"), (1u8, "translator")] {
            jit::force(mode);
            set_slot(img, 0x810, img + 0x200);
            let r0 = dll_routed();
            let (r, _) = emu_run_full(stub_d, [0; 8], [0; 4], [0; 8]);
            assert_eq!(r as u32, 7, "{name}: guest slot target");
            assert_eq!(dll_routed() - r0, 1, "{name}: routed once");
            // Slot rewritten between runs.
            set_slot(img, 0x810, img + 0x208);
            assert_eq!(emu_run_full(stub_d, [0; 8], [0; 4], [0; 8]).0 as u32, 9, "{name}: rewritten slot");
            assert_eq!(call_va8(stub_d, 0, 0, 0, 0, 0, 0, 0, 0) as u32, 9, "{name}: call_va8 to a guest-target stub");
            // Host slot target reached by the emulator: host tail call.
            let r1 = dll_routed();
            assert_eq!(emu_run_full(stub_a, [5, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0, 105, "{name}: host target");
            assert_eq!(dll_routed() - r1, 1, "{name}: host target routed");
            // Refused: delay-load-style slot. The guest executes the `jmp` itself.
            let r2 = dll_routed();
            assert_eq!(emu_run_full(stub_c, [0; 8], [0; 4], [0; 8]).0 as u32, 7, "{name}: refused stub as guest code");
            assert_eq!(dll_routed(), r2, "{name}: refused stub not routed");
            jit::force(0);
        }
        // Unbound slot: not routed (named `slot_unbound`).
        assert_eq!(dll_stub_route(stub_e), None);
        // Ordinary DLL code: not a stub.
        assert_eq!(dll_stub_route(img + 0x200), None);
    }
    crate::exe::unregister_native_va(hva);
}

/// `pe_iat_dir` reads data directory 12 of a PE32+ header (and rejects a missing one).
#[test]
fn dll_pe_iat_dir_parses_data_directory_12() {
    let img = arena_alloc(0x1000, 0x1000);
    unsafe {
        core::ptr::write_bytes(img, 0, 0x1000);
        let w16 = |o: usize, v: u16| core::ptr::write_unaligned(img.add(o) as *mut u16, v);
        let w32 = |o: usize, v: u32| core::ptr::write_unaligned(img.add(o) as *mut u32, v);
        w16(0, 0x5A4D);
        w32(0x3C, 0x80);
        w32(0x80, 0x0000_4550);
        w16(0x80 + 20, 240); // SizeOfOptionalHeader
        let opt = 0x80 + 24;
        w16(opt, 0x20b);
        w32(opt + 108, 16); // NumberOfRvaAndSizes
        assert_eq!(crate::dll_native::pe_iat_dir(img as usize, 0x1000), None, "empty directory");
        w32(opt + 112 + 12 * 8, 0x800);
        w32(opt + 112 + 12 * 8 + 4, 0x40);
        assert_eq!(
            crate::dll_native::pe_iat_dir(img as usize, 0x1000),
            Some((img as usize + 0x800, img as usize + 0x840))
        );
        w32(opt + 112 + 12 * 8 + 4, 0x1000);
        assert_eq!(crate::dll_native::pe_iat_dir(img as usize, 0x1000), None, "outside the image");
    }
}

/// Routable DLL IAT stubs (`renoircore+0x185270`/`+0x185282`, `cohtml+0x5a4d38`/`+0x5a4d44`
/// `ceilf`/`floorf` shape) bind directly to native `win32-rt` exports (no guest x86 leaf
/// thunks) and execute **0** guest instructions whether invoked from lifted Rust callers
/// (`call_va8` / `call_va8_xmm`), entered directly via `emu_run_full`, or called/tail-jumped
/// from guest code in both interpreter and JIT translator modes.
#[test]
fn dll_iat_stub_executes_zero_guest_instructions() {
    let ucrt = win32_rt::find_builtin("ucrtbase.dll").expect("ucrtbase builtin");
    let k32 = win32_rt::find_builtin("kernel32.dll").expect("kernel32 builtin");
    let export_ptr = |d: &win32_rt::BuiltinDll, name: &str| -> usize {
        d.exports
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| panic!("missing builtin export {}!{name}", d.name))
            .ptr
            .0 as usize
    };
    // Every export formerly intercepted by `guest_leaf_import_stub` must exist as a real
    // native `win32-rt` export in the builtin table.
    for name in [
        "ceilf",
        "floorf",
        "truncf",
        "sqrtf",
        "sqrt",
        "fmax",
        "fmin",
        "fmaxf",
        "fminf",
        "_isnan",
        "isnan",
        "_isnanf",
        "_lock_locales",
        "_unlock_locales",
    ] {
        assert_ne!(export_ptr(ucrt, name), 0, "ucrtbase!{name} null");
    }
    let tid_ptr = export_ptr(k32, "GetCurrentThreadId");
    assert_ne!(tid_ptr, 0);

    let ceilf_ptr = export_ptr(ucrt, "ceilf");
    let floorf_ptr = export_ptr(ucrt, "floorf");
    for p in [ceilf_ptr, floorf_ptr, tid_ptr] {
        crate::exe::register_native_va(p as u64, p);
    }

    let img = fake_iat_image();
    // stub_a (`FF 25`) -> slot +0x800 (`ceilf`), stub_b (`48 FF 25`) -> slot +0x808 (`floorf`),
    // stub_d (`FF 25`) -> slot +0x810 (guest `mov eax, 7; ret`, 2 instructions).
    let (stub_ceilf, stub_floorf, stub_guest) = (img + 0x100, img + 0x110, img + 0x130);
    unsafe {
        set_slot(img, 0x800, ceilf_ptr as u64);
        set_slot(img, 0x808, floorf_ptr as u64);
        set_slot(img, 0x810, img + 0x200);

        // Guest callers inside the DLL image:
        // `+0x300` (4 guest instructions): `call stub_ceilf; addss xmm0, xmm1; call stub_floorf; ret`
        // `+0x340` (3 guest instructions): `call stub_ceilf; addss xmm0, xmm1; jmp stub_floorf`
        let mut a = CodeAssembler::new(64).unwrap();
        a.call(stub_ceilf).unwrap();
        a.addss(xmm0, xmm1).unwrap();
        a.call(stub_floorf).unwrap();
        a.ret().unwrap();
        let c1 = a.assemble(img + 0x300).unwrap();
        core::ptr::copy_nonoverlapping(c1.as_ptr(), (img + 0x300) as *mut u8, c1.len());

        let mut a2 = CodeAssembler::new(64).unwrap();
        a2.call(stub_ceilf).unwrap();
        a2.addss(xmm0, xmm1).unwrap();
        a2.jmp(stub_floorf).unwrap();
        let c2 = a2.assemble(img + 0x340).unwrap();
        core::ptr::copy_nonoverlapping(c2.as_ptr(), (img + 0x340) as *mut u8, c2.len());

        // 1. Lifted caller path (`call_va8_xmm` / `call_va8`): 0 guest instructions.
        let total0 = test_total_steps();
        let (_, x_ceil) = crate::simd_ops::call_va8_xmm(
            stub_ceilf,
            0, 0, 0, 0, 0, 0, 0, 0,
            [2.25f32.to_bits() as u128, 0, 0, 0],
        );
        let (_, x_floor) = crate::simd_ops::call_va8_xmm(
            stub_floorf,
            0, 0, 0, 0, 0, 0, 0, 0,
            [(-2.25f32).to_bits() as u128, 0, 0, 0],
        );
        assert_eq!(f32::from_bits(x_ceil as u32), 3.0f32);
        assert_eq!(f32::from_bits(x_floor as u32), -3.0f32);
        assert_eq!(test_total_steps() - total0, 0, "lifted calls must execute 0 guest instructions");

        // 2. Direct emulator runs, guest-target stub, and guest callers in interpreter & JIT.
        let xmm_in = [2.25f32.to_bits() as u128, (-5.5f32).to_bits() as u128, 0, 0];
        let want_bits = (-3.0f32).to_bits(); // ceilf(2.25)=3.0, +(-5.5)=-2.5, floorf(-2.5)=-3.0
        for (mode, name) in [(2u8, "interpreter"), (1u8, "translator")] {
            jit::force(mode);

            // Direct entry at host-bound DLL IAT stub: 0 guest steps (no phantom pre-route step).
            let r0 = dll_routed();
            let (_, x0) = emu_run_full(stub_ceilf, [0; 8], [2.25f32.to_bits() as u128, 0, 0, 0], [0; 8]);
            assert_eq!(x0 as u32, 3.0f32.to_bits(), "{name}: direct stub_ceilf result");
            assert_eq!(test_last_steps(), 0, "{name}: direct host-bound stub must execute 0 guest steps");
            assert_eq!(dll_routed() - r0, 1, "{name}: stub routed once");

            // Guest-target stub (`mov eax, 7; ret` = 2 guest instructions, not 3).
            let (rg, _) = emu_run_full(stub_guest, [0; 8], [0; 4], [0; 8]);
            assert_eq!(rg as u32, 7, "{name}: guest-target stub value");
            assert_eq!(test_last_steps(), 2, "{name}: guest-target stub must execute only target's 2 steps");

            // Guest caller (`call stub_ceilf; addss; call stub_floorf; ret` = 4 guest steps) and
            // tail-jumping caller (`call stub_ceilf; addss; jmp stub_floorf` = 3 guest steps).
            // Run twice so translator mode exercises both translation and warm linked execution.
            for pass in 0..2 {
                let jb = jit::test_executed();
                let (_, xc) = emu_run_full(img + 0x300, [0; 8], xmm_in, [0; 8]);
                assert_eq!(xc as u32, want_bits, "{name} pass {pass}: call caller result");
                assert_eq!(test_last_steps(), 4, "{name} pass {pass}: call caller must execute 4 guest steps (0 in stubs)");
                if mode == 1 {
                    assert_eq!(jit::test_executed() - jb, 4, "translator pass {pass}: 4 JIT insns");
                }

                let jb2 = jit::test_executed();
                let (_, xt) = emu_run_full(img + 0x340, [0; 8], xmm_in, [0; 8]);
                assert_eq!(xt as u32, want_bits, "{name} pass {pass}: tail caller result");
                assert_eq!(test_last_steps(), 3, "{name} pass {pass}: tail caller must execute 3 guest steps (0 in stubs)");
                if mode == 1 {
                    assert_eq!(jit::test_executed() - jb2, 3, "translator pass {pass}: 3 JIT insns");
                }
            }
            jit::force(0);
        }

        // Routable DLL IAT stubs are never compiled into guest JIT blocks.
        assert_eq!(test_translate(stub_ceilf), None, "routed DLL IAT stub must not translate as guest block");
    }

    for p in [ceilf_ptr, floorf_ptr, tid_ptr] {
        crate::exe::unregister_native_va(p as u64);
    }
}

/// Byte-pattern-detected DLL `__chkstk` (`dll_native::CHKSTK_SIG` + direct `E9` jump thunks):
/// - detected in executable PE sections only (non-executable `.rdata` copy and mutated copy ignored),
///   never exposed as an `extern "win64"` host callable;
/// - normal pass (direct `call`, ILT `E9` thunk `call`, and tail `jmp`, in interpreter & JIT):
///   executes 0 guest instructions in `__chkstk` and matches the x86 `CHKSTK_SIG` oracle
///   bit-for-bit across all 16 GPRs, all 16 XMMs, `rsp` delta, and `RFLAGS`;
/// - overflow (`rax` exceeding emulator stack floor, `caller_rsp - rax` unsigned borrow wrap,
///   and custom pivoted stack crossing `TEB.StackLimit`): raises noncontinuable
///   `STATUS_STACK_OVERFLOW` (`0xC00000FD`).
#[test]
fn dll_chkstk_byte_pattern_detection_normal_and_overflow() {
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(|| unsafe {
            // Build a synthetic 24 KiB PE32+ image with:
            // - `.text` (`+0x1000..+0x4000`, executable):
            //   - `+0x1100`: exact 78-byte `CHKSTK_SIG`
            //   - `+0x1200`: 5-byte `E9 rel32` jump thunk -> `+0x1100`
            //   - `+0x1300`: mutated 78-byte copy (1 byte altered)
            // - `.rdata` (`+0x4000..+0x5000`, read-only non-executable):
            //   - `+0x4100`: exact 78-byte `CHKSTK_SIG` (must NOT be detected)
            let img = arena_alloc(0x6000, 0x1000);
            core::ptr::write_bytes(img, 0, 0x6000);
            core::ptr::write_bytes(img.add(0x1000), 0xCC, 0x3000);
            let w16 = |o: usize, v: u16| core::ptr::write_unaligned(img.add(o) as *mut u16, v);
            let w32 = |o: usize, v: u32| core::ptr::write_unaligned(img.add(o) as *mut u32, v);
            w16(0, 0x5A4D);
            w32(0x3C, 0x80);
            w32(0x80, 0x0000_4550);
            w16(0x80 + 6, 2); // NumberOfSections = 2
            w16(0x80 + 20, 240); // SizeOfOptionalHeader
            w16(0x80 + 24, 0x20b); // PE32+
            w32(0x80 + 24 + 108, 16); // NumberOfRvaAndSizes
            let sec0 = 0x80 + 24 + 240;
            core::ptr::copy_nonoverlapping(b".text\0\0\0".as_ptr(), img.add(sec0), 8);
            w32(sec0 + 8, 0x3000); // VirtualSize
            w32(sec0 + 12, 0x1000); // VirtualAddress
            w32(sec0 + 36, 0x6000_0020); // CNT_CODE | MEM_EXECUTE | MEM_READ
            let sec1 = sec0 + 40;
            core::ptr::copy_nonoverlapping(b".rdata\0\0".as_ptr(), img.add(sec1), 8);
            w32(sec1 + 8, 0x1000); // VirtualSize
            w32(sec1 + 12, 0x4000); // VirtualAddress
            w32(sec1 + 36, 0x4000_0040); // CNT_INITIALIZED_DATA | MEM_READ (not executable)

            let sig = &crate::dll_native::CHKSTK_SIG;
            core::ptr::copy_nonoverlapping(sig.as_ptr(), img.add(0x1100), sig.len());
            *img.add(0x1200) = 0xE9;
            let rel_to_chk = ((0x1100isize) - (0x1200isize + 5)) as i32;
            core::ptr::write_unaligned(img.add(0x1201) as *mut i32, rel_to_chk);
            core::ptr::copy_nonoverlapping(sig.as_ptr(), img.add(0x1300), sig.len());
            *img.add(0x1300 + 20) ^= 0x08; // mutate `lea r10, [rsp+0x18]`
            core::ptr::copy_nonoverlapping(sig.as_ptr(), img.add(0x4100), sig.len());

            let base = img as u64;
            let chkstk_va = base + 0x1100;
            let thunk_va = base + 0x1200;
            let mutated_va = base + 0x1300;
            let rdata_va = base + 0x4100;

            crate::dll_native::install("fake_chkstk_test.dll", base as usize, 0x6000);
            assert!(crate::dll_native::is_dll_chkstk(chkstk_va), ".text CHKSTK_SIG must be detected");
            assert!(crate::dll_native::is_dll_chkstk(thunk_va), ".text E9 thunk to CHKSTK_SIG must be detected");
            assert!(!crate::dll_native::is_dll_chkstk(mutated_va), "mutated signature must not match");
            assert!(!crate::dll_native::is_dll_chkstk(rdata_va), "non-executable .rdata signature must not match");
            assert_eq!(resolve_host_callable(chkstk_va), None, "__chkstk must not expose extern win64 callable");
            assert_eq!(resolve_host_callable(thunk_va), None, "__chkstk thunk must not expose extern win64 callable");

            // Warm this thread's TEB + emulator state (`ls_gstack`).
            let teb = win32_rt::teb::current();
            let saved_teb_hi: usize = win32_rt::teb::field(teb, win32_rt::teb::off::STACK_BASE);
            let saved_teb_lo: usize = win32_rt::teb::field(teb, win32_rt::teb::off::STACK_LIMIT);
            let _ = emu_run_full(base + 0x1000, [0; 8], [0; 4], [0; 8]); // 0xCC returns in lenient test mode
            assert_eq!(test_translate(chkstk_va), None, "DLL __chkstk must not translate into a guest block");
            assert_eq!(test_translate(thunk_va), None, "DLL __chkstk thunk must not translate into a guest block");

            // Assemble a guest state-capture harness in `.text`:
            // loads all 16 XMMs + RFLAGS + all 15 non-rsp GPRs (`rax` = requested stack size from
            // `[init_ptr]`), invokes `target` (via direct `call` or via a `jmp chkstk_va` tail helper),
            // and dumps post-call `rax, rcx, rdx, rbx, rsp_delta, rbp, rsi, rdi, r8..r15`, `RFLAGS`,
            // and `xmm0..xmm15` to `out_buf`.
            #[repr(C, align(16))]
            #[derive(Clone, Copy, PartialEq, Eq, Debug)]
            struct Snap {
                gpr: [u64; 16], // gpr[4] stores `post_rsp - pre_rsp`
                rflags: u64,
                xmm: [u128; 16],
            }
            let init_xmm: [u128; 16] = core::array::from_fn(|i| {
                ((0xDEAD_0000_0000_0100u128 + i as u128 * 17) << 64)
                    | (0xBEEF_0000_0000_0200u128 + i as u128 * 31)
            });
            let init_gpr: [u64; 16] = [
                0x2468, // rax (overridden per case)
                0x1111_2222_3333_4441,
                0x1111_2222_3333_4442,
                0x1111_2222_3333_4443,
                0,
                0x1111_2222_3333_4445,
                0x1111_2222_3333_4446,
                0x1111_2222_3333_4447,
                0x8888_7777_6666_5508,
                0x8888_7777_6666_5509,
                0xAAAA_BBBB_CCCC_DD10, // r10 (saved/restored by __chkstk)
                0xEEEE_FFFF_1234_5611, // r11 (saved/restored by __chkstk)
                0x8888_7777_6666_5512,
                0x8888_7777_6666_5513,
                0x8888_7777_6666_5514,
                0x8888_7777_6666_5515,
            ];
            let xmm_buf = arena_alloc(256, 16) as *mut u128;
            core::ptr::copy_nonoverlapping(init_xmm.as_ptr(), xmm_buf, 16);
            let out_buf = arena_alloc(core::mem::size_of::<Snap>(), 16) as *mut Snap;
            let rax_slot = arena_alloc(16, 8) as *mut u64; // [0] = requested rax, [1] = pre_rsp

            // Tail-jump helper at `+0x1400`: `jmp chkstk_va`
            // (exercises `Fx::Jmp(chkstk_va)` / `jit_host_xfer` tail-jump path).
            let tail_helper = base + 0x1400;
            {
                let mut t = CodeAssembler::new(64).unwrap();
                t.jmp(chkstk_va).unwrap();
                let tc = t.assemble(tail_helper).unwrap();
                core::ptr::copy_nonoverlapping(tc.as_ptr(), tail_helper as *mut u8, tc.len());
            }

            let build_harness = |slot_off: u64, callee: u64| -> (u64, u64) {
                let code_va = base + slot_off;
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(rax, rax_slot.add(1) as u64).unwrap();
                a.mov(qword_ptr(rax), rsp).unwrap();
                a.mov(r11, xmm_buf as u64).unwrap();
                for (i, xm) in [
                    xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7,
                    xmm8, xmm9, xmm10, xmm11, xmm12, xmm13, xmm14, xmm15,
                ]
                .into_iter()
                .enumerate()
                {
                    a.movdqu(xm, xmmword_ptr(r11 + (i * 16) as i32)).unwrap();
                }
                // Seed arithmetic flags (`CF|PF|ZF|SF|OF` = 0x8D5) before the call so we prove
                // `__chkstk` overwrites them with exact `add rsp, 0x10` flags.
                a.mov(rax, 0x202u64 | 0x8D5).unwrap();
                a.push(rax).unwrap();
                a.popfq().unwrap();
                for (reg, val) in [
                    (rcx, init_gpr[1]),
                    (rdx, init_gpr[2]),
                    (rbx, init_gpr[3]),
                    (rbp, init_gpr[5]),
                    (rsi, init_gpr[6]),
                    (rdi, init_gpr[7]),
                    (r8, init_gpr[8]),
                    (r9, init_gpr[9]),
                    (r10, init_gpr[10]),
                    (r11, init_gpr[11]),
                    (r12, init_gpr[12]),
                    (r13, init_gpr[13]),
                    (r14, init_gpr[14]),
                    (r15, init_gpr[15]),
                ] {
                    a.mov(reg, val).unwrap();
                }
                a.mov(rax, rax_slot as u64).unwrap();
                a.mov(rax, qword_ptr(rax)).unwrap();
                a.call(callee).unwrap();
                // Save `rax` to `[rsp - 0x40]` without touching any flags or other registers.
                a.mov(qword_ptr(rsp - 0x40), rax).unwrap();
                a.mov(rax, out_buf as u64).unwrap();
                a.mov(qword_ptr(rax + 8), rcx).unwrap();
                a.mov(qword_ptr(rax + 16), rdx).unwrap();
                a.mov(qword_ptr(rax + 24), rbx).unwrap();
                a.mov(qword_ptr(rax + 32), rsp).unwrap();
                a.mov(qword_ptr(rax + 40), rbp).unwrap();
                a.mov(qword_ptr(rax + 48), rsi).unwrap();
                a.mov(qword_ptr(rax + 56), rdi).unwrap();
                a.mov(qword_ptr(rax + 64), r8).unwrap();
                a.mov(qword_ptr(rax + 72), r9).unwrap();
                a.mov(qword_ptr(rax + 80), r10).unwrap();
                a.mov(qword_ptr(rax + 88), r11).unwrap();
                a.mov(qword_ptr(rax + 96), r12).unwrap();
                a.mov(qword_ptr(rax + 104), r13).unwrap();
                a.mov(qword_ptr(rax + 112), r14).unwrap();
                a.mov(qword_ptr(rax + 120), r15).unwrap();
                a.pushfq().unwrap();
                a.pop(qword_ptr(rax + 128)).unwrap();
                a.mov(rdx, qword_ptr(rsp - 0x40)).unwrap();
                a.mov(qword_ptr(rax), rdx).unwrap();
                let xmm_off = core::mem::offset_of!(Snap, xmm) as i32;
                for (i, xm) in [
                    xmm0, xmm1, xmm2, xmm3, xmm4, xmm5, xmm6, xmm7,
                    xmm8, xmm9, xmm10, xmm11, xmm12, xmm13, xmm14, xmm15,
                ]
                .into_iter()
                .enumerate()
                {
                    a.movdqu(xmmword_ptr(rax + xmm_off + (i * 16) as i32), xm).unwrap();
                }
                a.xor(eax, eax).unwrap();
                a.ret().unwrap();
                let bytes = a.assemble(code_va).unwrap();
                assert!(bytes.len() < 0x380, "harness len {} exceeds 0x400 slot", bytes.len());
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), code_va as *mut u8, bytes.len());
                // Count instructions in the harness itself (`iced_x86::Decoder`).
                let mut dec = iced_x86::Decoder::with_ip(64, &bytes, code_va, iced_x86::DecoderOptions::NONE);
                let mut n_insns = 0u64;
                while dec.can_decode() {
                    let _ = dec.decode();
                    n_insns += 1;
                }
                (code_va, n_insns)
            };

            let (h_direct, n_direct) = build_harness(0x1800, chkstk_va);
            let (h_thunk, n_thunk) = build_harness(0x1c00, thunk_va);
            let (h_tail, n_tail) = build_harness(0x2000, tail_helper);

            let (floor, gtop) = crate::callctx::ctx().ls_gstack.get();
            assert!(floor > 0 && gtop > floor + 0x10000);

            let run_snap = |va: u64, req_rax: u64| -> (Snap, u64) {
                core::ptr::write_bytes(out_buf as *mut u8, 0, core::mem::size_of::<Snap>());
                *rax_slot = req_rax;
                *rax_slot.add(1) = 0;
                let _ = emu_run_full_rax(va, [0; 8], [0; 4], [0; 8], 0);
                let mut s = *out_buf;
                let pre_rsp = *rax_slot.add(1);
                s.gpr[4] = s.gpr[4].wrapping_sub(pre_rsp);
                (s, test_last_steps())
            };

            // Compare native DLL `__chkstk` routing against the real x86 `CHKSTK_SIG` oracle
            // across both small (`0x400`, no probe loop in x86) and multi-page (`0x2468`, enters
            // probe loop in x86) frame sizes, in interpreter and JIT translator modes.
            for req_rax in [0x0u64, 0x400, 0x2468] {
                // Oracle run (`DLL_CHKSTK_OFF = true`) with page-aligned `TEB.StackBase` and
                // `TEB.StackLimit = oracle_hi - 0x1000` so `0x2468` exercises the x86 probe loop too.
                let oracle_hi = (gtop & !0xFFFu64) as usize;
                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_BASE, oracle_hi);
                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_LIMIT, oracle_hi - 0x1000);
                jit::force(2);
                DLL_CHKSTK_OFF.with(|c| c.set(true));
                let (oracle, oracle_steps) = run_snap(h_direct, req_rax);
                DLL_CHKSTK_OFF.with(|c| c.set(false));
                jit::force(0);
                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_BASE, saved_teb_hi);
                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_LIMIT, saved_teb_lo);

                assert!(oracle_steps > n_direct, "oracle must execute guest __chkstk instructions");
                assert_eq!(oracle.gpr[0], req_rax, "rax preserved");
                assert_eq!(oracle.gpr[4], 0, "rsp restored to pre-call value");

                for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                    jit::force(mode);
                    for (label, hva, expect_steps) in [
                        ("direct", h_direct, n_direct),
                        ("ilt_thunk", h_thunk, n_thunk),
                        ("tail_jmp", h_tail, n_tail + 1), // +1 for `jmp chkstk_va` in tail_helper
                    ] {
                        for pass in 0..2 {
                            let c0 = dll_chkstk_routed();
                            let (got, steps) = run_snap(hva, req_rax);
                            assert_eq!(
                                dll_chkstk_routed() - c0,
                                1,
                                "{mname} {label} pass {pass} rax={req_rax:#x}: routed once"
                            );
                            assert_eq!(
                                steps, expect_steps,
                                "{mname} {label} pass {pass} rax={req_rax:#x}: 0 guest steps in __chkstk"
                            );
                            assert_eq!(
                                got, oracle,
                                "{mname} {label} pass {pass} rax={req_rax:#x}: bit-for-bit parity with x86 oracle"
                            );
                        }
                    }
                    jit::force(0);
                }
            }
            // Also verify direct entry at `chkstk_va` (`emu_run_full_rax(chkstk_va, ...)`): 0 steps.
            for mode in [2u8, 1u8] {
                jit::force(mode);
                let c0 = dll_chkstk_routed();
                let (r, _) = emu_run_full_rax(chkstk_va, [0; 8], [0; 4], [0; 8], 0x1000);
                assert_eq!(r, 0x1000);
                assert_eq!(dll_chkstk_routed() - c0, 1);
                assert_eq!(test_last_steps(), 0, "direct entry at __chkstk must execute 0 guest steps");
                jit::force(0);
            }

            // Overflow semantics: must raise noncontinuable `STATUS_STACK_OVERFLOW` (`0xC00000FD`).
            // Assemble minimal callers in `.text`:
            // - `ov_call` (`+0x2400`): `mov rax, rcx; call chkstk_va; mov eax, 0x77; ret`
            // - `ov_tail` (`+0x2500`): `mov rax, rcx; jmp thunk_va`
            let ov_call = base + 0x2400;
            {
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(rax, rcx).unwrap();
                a.call(chkstk_va).unwrap();
                a.mov(eax, 0x77u32).unwrap();
                a.ret().unwrap();
                let b = a.assemble(ov_call).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), ov_call as *mut u8, b.len());
            }
            let ov_tail = base + 0x2500;
            {
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(rax, rcx).unwrap();
                a.jmp(thunk_va).unwrap();
                let b = a.assemble(ov_tail).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), ov_tail as *mut u8, b.len());
            }

            let catch_ov = |f: &mut dyn FnMut() -> u64| {
                crate::eh_rt::host_try::catch_code_if(
                    crate::lstack::STATUS_STACK_OVERFLOW,
                    &mut |_| Some([0; 2]),
                    f,
                )
            };

            // Custom pivoted stack `[piv_lo, piv_hi)` outside `ThreadEmuState`, governed by
            // `TEB.StackBase = piv_hi` and `TEB.StackLimit = piv_lo + 0x4000`.
            let piv = arena_alloc(0x8000, 0x1000);
            core::ptr::write_bytes(piv, 0, 0x8000);
            let piv_lo = piv as u64;
            let piv_hi = piv_lo + 0x8000;
            let piv_sp = piv_hi - 0x108;
            core::ptr::write_unaligned(piv_sp as *mut u64, RET_SENTINEL);

            let piv_va = base + 0x2600;
            {
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(rbx, rsp).unwrap();
                a.mov(rsp, piv_sp).unwrap();
                a.mov(rax, rcx).unwrap();
                a.call(chkstk_va).unwrap();
                a.mov(rsp, rbx).unwrap();
                a.mov(eax, 0x42u32).unwrap();
                a.ret().unwrap();
                let b = a.assemble(piv_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), piv_va as *mut u8, b.len());
            }

            let too_big = (gtop - floor) + 0x2000;
            for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                jit::force(mode);
                // Sanity: small request succeeds.
                assert_eq!(
                    catch_ov(&mut || emu_run_full_rax(ov_call, [0x2000, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], 0).0)
                        .ok()
                        .expect("small frame ok"),
                    0x77
                );
                // (a) Exceeds emulator stack floor & (b) unsigned borrow wrap (`u64::MAX`),
                // across both `call __chkstk` and tail `jmp __chkstk_thunk`.
                for (va, kind) in [(ov_call, "call"), (ov_tail, "tail")] {
                    for (req, why) in [(too_big, "past_floor"), (u64::MAX, "borrow_wrap")] {
                        let err = catch_ov(&mut || {
                            emu_run_full_rax(va, [req, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], 0).0
                        })
                        .err()
                        .unwrap_or_else(|| panic!("{mname} {kind} {why}: expected STATUS_STACK_OVERFLOW"));
                        assert_eq!(err.rec.code, crate::lstack::STATUS_STACK_OVERFLOW);
                        assert_ne!(
                            err.rec.flags & win32_rt::unwind::EXCEPTION_NONCONTINUABLE,
                            0,
                            "{mname} {kind} {why}: overflow must be noncontinuable"
                        );
                    }
                }

                // (c) Custom pivoted stack `[piv_lo, piv_hi)` governed by `TEB`.
                core::ptr::write_unaligned(piv_sp as *mut u64, RET_SENTINEL);

                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_BASE, piv_hi as usize);
                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_LIMIT, (piv_lo + 0x4000) as usize);
                let ok_piv = catch_ov(&mut || {
                    emu_run_full_rax(piv_va, [0x1000, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], 0).0
                });
                let err_piv = catch_ov(&mut || {
                    emu_run_full_rax(piv_va, [0x5000, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], 0).0
                });
                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_BASE, saved_teb_hi);
                win32_rt::teb::set_field(teb, win32_rt::teb::off::STACK_LIMIT, saved_teb_lo);

                assert_eq!(ok_piv.ok().expect("pivoted in-bounds request ok"), 0x42, "{mname}: pivoted ok");
                let caught = err_piv.err().unwrap_or_else(|| panic!("{mname}: pivoted overflow must raise"));
                assert_eq!(caught.rec.code, crate::lstack::STATUS_STACK_OVERFLOW);
                assert_ne!(caught.rec.flags & win32_rt::unwind::EXCEPTION_NONCONTINUABLE, 0);

                jit::force(0);
            }

            crate::dll_native::register_chkstk_vas(base as usize, 0x6000, &[]);
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Multi-hop ILT `E9 rel32` -> `FF 25 [IAT]` chains inside a mapped DLL image
/// (`v8.dll+0xad7f0 -> +0x426ad0 -> +0x1090110` `[VCRUNTIME140.dll!memchr]` shape) resolve via
/// `DF_IAT` / `dll_stub_route` / `is_routed_dll_stub` in **0** guest instructions across
/// lifted callers, direct entry, interpreter, and JIT, while rejecting self-loops, >4 hops,
/// out-of-image `E9` targets, and ordinary in-image `E9` jumps.
#[test]
fn dll_ilt_e9_chain_to_iat_stub_routes_in_zero_guest_instructions() {
    let vcrt = win32_rt::find_builtin("vcruntime140.dll").expect("vcruntime140 builtin");
    let memchr_ptr = vcrt
        .exports
        .iter()
        .find(|e| e.name == "memchr")
        .expect("vcruntime140!memchr")
        .ptr
        .0 as usize;
    crate::exe::register_native_va(memchr_ptr as u64, memchr_ptr);

    let img = fake_iat_image();
    // Slot `+0x800` (`stub_a` at `+0x100`) -> native `memchr`.
    let stub_iat = img + 0x100;
    // Build a 2-hop ILT chain `+0x400: E9 -> +0x420: E9 -> +0x100 (FF 25 [+0x800])`,
    // a 5-hop chain `+0x440 -> +0x450 -> +0x460 -> +0x470 -> +0x480 -> +0x100` (must NOT route),
    // a self-loop `+0x490: E9 -5`, an out-of-image `E9` `+0x4a0: E9 +0x2000`,
    // and an ordinary `E9` to guest code `+0x4b0: E9 -> +0x200`.
    let write_e9 = |from_off: u64, to_va: u64| unsafe {
        let p = (img + from_off) as *mut u8;
        *p = 0xE9;
        let rel = (to_va as i64 - (img + from_off + 5) as i64) as i32;
        core::ptr::write_unaligned(p.add(1) as *mut i32, rel);
    };
    write_e9(0x420, stub_iat);
    write_e9(0x400, img + 0x420);
    write_e9(0x480, stub_iat);
    write_e9(0x470, img + 0x480);
    write_e9(0x460, img + 0x470);
    write_e9(0x450, img + 0x460);
    write_e9(0x440, img + 0x450);
    write_e9(0x490, img + 0x490);
    write_e9(0x4a0, img + 0x24a0);
    write_e9(0x4b0, img + 0x200);
    let ilt_2hop = img + 0x400;

    unsafe {
        set_slot(img, 0x800, memchr_ptr as u64);

        assert_eq!(crate::dll_native::iat_stub_slot_checked(ilt_2hop), Ok(Some(img as usize + 0x800)));
        assert_eq!(crate::dll_native::iat_stub_slot_checked(img + 0x450), Ok(Some(img as usize + 0x800)), "4 hops ok");
        assert_eq!(crate::dll_native::iat_stub_slot_checked(img + 0x440), Ok(None), ">4 hops must stop");
        assert_eq!(crate::dll_native::iat_stub_slot_checked(img + 0x490), Ok(None), "self-loop must stop");
        assert_eq!(crate::dll_native::iat_stub_slot_checked(img + 0x4a0), Ok(None), "out-of-image hop rejected");
        assert_eq!(crate::dll_native::iat_stub_slot_checked(img + 0x4b0), Ok(None), "ordinary E9 is not IAT");

        let hay = arena_alloc(32, 16);
        core::ptr::copy_nonoverlapping(b"hello!world\0".as_ptr(), hay, 12);
        let hay_va = hay as u64;
        let want = hay_va + 5; // '!'

        // 1. Lifted caller (`call_va8` -> `DF_IAT`): 0 guest steps.
        let t0 = test_total_steps();
        assert_eq!(call_va8(ilt_2hop, hay_va, b'!' as u64, 12, 0, 0, 0, 0, 0), want);
        assert_eq!(test_total_steps() - t0, 0);

        // 2. Guest indirect caller (`mov rax, ilt_2hop; jmp rax` = 2 guest insns, matching V8 JSEntry -> ExternalReference).
        let caller = img + 0x500;
        let mut a = CodeAssembler::new(64).unwrap();
        a.mov(rax, ilt_2hop).unwrap();
        a.jmp(rax).unwrap();
        let bytes = a.assemble(caller).unwrap();
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), caller as *mut u8, bytes.len());

        for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
            jit::force(mode);
            let r0 = dll_routed();
            let (direct_res, _) = emu_run_full(ilt_2hop, [hay_va, b'!' as u64, 12, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            assert_eq!(direct_res, want, "{mname}: direct ilt_2hop");
            assert_eq!(test_last_steps(), 0, "{mname}: direct ilt_2hop must execute 0 guest steps");
            assert_eq!(dll_routed() - r0, 1, "{mname}: routed once");

            for pass in 0..2 {
                let (ind_res, _) = emu_run_full(caller, [hay_va, b'!' as u64, 12, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                assert_eq!(ind_res, want, "{mname} pass {pass}: indirect jmp rax -> ilt_2hop");
                assert_eq!(test_last_steps(), 2, "{mname} pass {pass}: only caller's 2 insns execute");
            }
            jit::force(0);
        }

        assert_eq!(test_translate(ilt_2hop), None, "routed E9->IAT chain must not translate as guest block");
    }

    crate::exe::unregister_native_va(memchr_ptr as u64);
}

/// Byte-pattern detection of `lea rax, [rsp]; ret` (`STACK_POS_SIG` = `48 8d 04 24 c3`) and
/// standalone 16-byte-aligned `E9 rel32` thunks to it (`v8.dll+0x8ead0 -> +0xa07e00`
/// `GetCurrentStackPosition`), routed in **0** guest instructions across `call_va8`, direct
/// entry, guest `call`, and guest `jmp` in both interpreter and JIT modes with exact `rsp`,
/// `RFLAGS`, GPR, and XMM parity against the unrouted x86 oracle.
#[test]
fn dll_stack_pos_byte_pattern_zero_guest_steps_and_rflags_parity() {
    std::thread::Builder::new()
        .name("dll_stack_pos_unit".into())
        .spawn(|| unsafe {
            let img = arena_alloc(0x6000, 0x1000);
            core::ptr::write_bytes(img, 0, 0x1000);
            core::ptr::write_bytes(img.add(0x1000), 0xCC, 0x3000);
            core::ptr::write_bytes(img.add(0x4000), 0xCC, 0x1000);

            let w16 = |o: usize, v: u16| core::ptr::write_unaligned(img.add(o) as *mut u16, v);
            let w32 = |o: usize, v: u32| core::ptr::write_unaligned(img.add(o) as *mut u32, v);
            w16(0, 0x5A4D);
            w32(0x3C, 0x80);
            w32(0x80, 0x0000_4550);
            w16(0x80 + 6, 2);
            w16(0x80 + 20, 240);
            w16(0x80 + 24, 0x20b);
            w32(0x80 + 24 + 108, 16);
            let sec0 = 0x80 + 24 + 240;
            core::ptr::copy_nonoverlapping(b".text\0\0\0".as_ptr(), img.add(sec0), 8);
            w32(sec0 + 8, 0x3000);
            w32(sec0 + 12, 0x1000);
            w32(sec0 + 36, 0x6000_0020);
            let sec1 = sec0 + 40;
            core::ptr::copy_nonoverlapping(b".rdata\0\0".as_ptr(), img.add(sec1), 8);
            w32(sec1 + 8, 0x1000);
            w32(sec1 + 12, 0x4000);
            w32(sec1 + 36, 0x4000_0040);

            let sig = &crate::dll_native::STACK_POS_SIG;
            // Valid 16-aligned `.text` body at `+0x1100` and standalone `E9` thunk at `+0x1200`.
            core::ptr::copy_nonoverlapping(sig.as_ptr(), img.add(0x1100), sig.len());
            *img.add(0x1200) = 0xE9;
            let rel = (0x1100isize - (0x1200isize + 5)) as i32;
            core::ptr::write_unaligned(img.add(0x1201) as *mut i32, rel);

            // Unaligned copy at `+0x1304`: must NOT match.
            core::ptr::copy_nonoverlapping(sig.as_ptr(), img.add(0x1304), sig.len());
            // Mid-instruction / non-boundary copy at `+0x1400` preceded by `0x48`: must NOT match.
            *img.add(0x13FF) = 0x48;
            core::ptr::copy_nonoverlapping(sig.as_ptr(), img.add(0x1400), sig.len());
            // Non-executable `.rdata` copy at `+0x4100`: must NOT match.
            core::ptr::copy_nonoverlapping(sig.as_ptr(), img.add(0x4100), sig.len());

            let base = img as u64;
            let sp_va = base + 0x1100;
            let thunk_va = base + 0x1200;

            crate::dll_native::install("fake_stack_pos_test.dll", base as usize, 0x6000);
            assert!(crate::dll_native::is_dll_stack_pos(sp_va), "16-aligned .text STACK_POS_SIG must match");
            assert!(crate::dll_native::is_dll_stack_pos(thunk_va), "16-aligned E9 thunk must match");
            assert!(!crate::dll_native::is_dll_stack_pos(base + 0x1304), "unaligned copy must not match");
            assert!(!crate::dll_native::is_dll_stack_pos(base + 0x1400), "non-boundary copy must not match");
            assert!(!crate::dll_native::is_dll_stack_pos(base + 0x4100), ".rdata copy must not match");
            assert_eq!(resolve_host_callable(sp_va), None, "stack_pos must not expose extern win64 callable");

            // Warm emulator TLS on this thread.
            let _ = emu_run_full(base + 0x1000, [0; 8], [0; 4], [0; 8]);
            assert_eq!(test_translate(sp_va), None, "stack_pos must not translate as guest block");
            assert_eq!(test_translate(thunk_va), None, "stack_pos thunk must not translate as guest block");

            // 1. Lifted host caller (`call_va8`) parity vs unrouted x86 oracle, with and without
            //    a published `outgoing_rsp` window.
            for target in [sp_va, thunk_va] {
                DLL_STACK_POS_OFF.with(|c| c.set(true));
                let oracle_no_win = call_va8(target, 0, 0, 0, 0, 0, 0, 0, 0);
                DLL_STACK_POS_OFF.with(|c| c.set(false));

                let r0 = dll_stack_pos_routed();
                let t0 = test_total_steps();
                let got_no_win = call_va8(target, 0, 0, 0, 0, 0, 0, 0, 0);
                assert_eq!(got_no_win, oracle_no_win, "call_va8 no-window rsp match");
                assert_eq!(test_total_steps() - t0, 0, "call_va8 stack_pos must run 0 guest steps");
                assert_eq!(dll_stack_pos_routed() - r0, 1);

                let mut lf = crate::lstack::LiftedFrame::<64, 8>::new();
                let _ = crate::simd_ops::StackBuf::frame_base(&mut lf);
                let fake_frame = arena_alloc(0x100, 16) as u64;
                let ctx = crate::callctx::ctx();
                DLL_STACK_POS_OFF.with(|c| c.set(true));
                ctx.outgoing_rsp.set((fake_frame + 0x40, fake_frame + 0xc0));
                let oracle_win = call_va8(target, 0, 0, 0, 0, 0, 0, 0, 0);
                DLL_STACK_POS_OFF.with(|c| c.set(false));

                let r1 = dll_stack_pos_routed();
                let t1 = test_total_steps();
                ctx.outgoing_rsp.set((fake_frame + 0x40, fake_frame + 0xc0));
                let got_win = call_va8(target, 0, 0, 0, 0, 0, 0, 0, 0);
                assert!(got_win < got_no_win, "LiftedFrame must charge lifted_depth to guest stack pos");
                assert_eq!(got_win, oracle_win, "call_va8 LiftedFrame + outgoing_rsp window match");
                assert_eq!(test_total_steps() - t1, 0, "call_va8 with window must run 0 guest steps");
                assert_eq!(dll_stack_pos_routed() - r1, 1);
                assert_eq!(ctx.outgoing_rsp.get(), (0, 0), "outgoing_rsp consumed");
            }

            // 2. Guest `call` (checking live RFLAGS across `Term::Call`) and tail `jmp` in both
            //    interpreter and JIT translator.
            // Caller (`+0x1500` / `+0x1600`): sets live arithmetic flags via `sub ecx, edx`,
            // calls `target`, branches on `jb` / `je` from BEFORE the call (`lea rax,[rsp]; ret`
            // does not touch RFLAGS!), and returns `(rax - rsp) ^ flag_tag`.
            let out_sp = arena_alloc(16, 8) as *mut u64;
            let build_caller = |code_va: u64, target: u64| -> u64 {
                let mut a = CodeAssembler::new(64).unwrap();
                let mut lbl_b = a.create_label();
                let mut lbl_done = a.create_label();
                a.mov(r8, out_sp as u64).unwrap();
                a.mov(qword_ptr(r8), rsp).unwrap();
                a.sub(ecx, edx).unwrap(); // sets CF/ZF/SF/OF/PF
                a.call(target).unwrap();
                // Must still see `sub ecx, edx`'s CF after `call stack_pos` even in JIT!
                a.jb(lbl_b).unwrap();
                a.xor(rax, 0x1100i32).unwrap();
                a.jmp(lbl_done).unwrap();
                a.set_label(&mut lbl_b).unwrap();
                a.xor(rax, 0x2200i32).unwrap();
                a.set_label(&mut lbl_done).unwrap();
                a.ret().unwrap();
                let bytes = a.assemble(code_va).unwrap();
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), code_va as *mut u8, bytes.len());
                let mut dec = iced_x86::Decoder::with_ip(64, &bytes, code_va, iced_x86::DecoderOptions::NONE);
                let mut n = 0u64;
                while dec.can_decode() {
                    let _ = dec.decode();
                    n += 1;
                }
                // 2 branches terminate one path; compute exact executed steps from oracle minus target steps.
                let _ = n;
                code_va
            };
            let c_body = build_caller(base + 0x1500, sp_va);
            let c_thunk = build_caller(base + 0x1600, thunk_va);
            let tail_va = base + 0x1700;
            {
                let mut a = CodeAssembler::new(64).unwrap();
                a.jmp(thunk_va).unwrap();
                let b = a.assemble(tail_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), tail_va as *mut u8, b.len());
            }

            for &(c_va, target_steps) in &[(c_body, 2u64), (c_thunk, 3u64)] {
                for &(ecx_in, edx_in) in &[(3u64, 9u64), (12u64, 5u64)] {
                    // Oracle run on a separate code address so JIT doesn't cache unrouted blocks at `c_va`.
                    DLL_STACK_POS_OFF.with(|c| c.set(true));
                    jit::force(2);
                    let (want, _) = emu_run_full(c_va, [ecx_in, edx_in, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                    let want_sp = core::ptr::read_unaligned(out_sp);
                    let oracle_steps = test_last_steps();
                    DLL_STACK_POS_OFF.with(|c| c.set(false));

                    // `lea rax, [rsp]` inside callee sees `caller_rsp - 8`.
                    let tag = if ecx_in < edx_in { 0x2200 } else { 0x1100 };
                    assert_eq!(want, (want_sp - 8) ^ tag);

                    for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                        jit::force(mode);
                        for pass in 0..2 {
                            let r0 = dll_stack_pos_routed();
                            let (got, _) = emu_run_full(c_va, [ecx_in, edx_in, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                            assert_eq!(got, want, "{mname} pass {pass} ({ecx_in},{edx_in}): rax+flags parity");
                            assert_eq!(dll_stack_pos_routed() - r0, 1, "{mname} pass {pass}: routed once");
                            assert_eq!(
                                test_last_steps(),
                                oracle_steps - target_steps,
                                "{mname} pass {pass}: 0 guest steps inside stack_pos"
                            );
                        }
                        jit::force(0);
                    }
                }
            }

            // Direct entry and tail-jump entry in both modes.
            for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                jit::force(mode);
                let r0 = dll_stack_pos_routed();
                let (d_got, _) = emu_run_full(thunk_va, [0; 8], [0; 4], [0; 8]);
                assert_ne!(d_got, 0);
                assert_eq!(test_last_steps(), 0, "{mname}: direct entry at stack_pos thunk = 0 steps");
                assert_eq!(dll_stack_pos_routed() - r0, 1);

                for pass in 0..2 {
                    let r1 = dll_stack_pos_routed();
                    let (t_got, _) = emu_run_full(tail_va, [0; 8], [0; 4], [0; 8]);
                    assert_eq!(t_got, d_got, "{mname} pass {pass}: tail jmp rsp matches direct entry");
                    assert_eq!(test_last_steps(), 1, "{mname} pass {pass}: only caller's 1 jmp executed");
                    assert_eq!(dll_stack_pos_routed() - r1, 1);
                }
                jit::force(0);
            }

            crate::dll_native::register_stack_pos_vas(base as usize, 0x6000, &[]);
        })
        .unwrap()
        .join()
        .unwrap();
}

unsafe extern "C" fn test_rf_b1(raw: *mut u8, _blk: u32) -> regfile::RegfileExit {
    let mut c = regfile::ctx(raw);
    let Some(bc) = c.ld8(c.r64(12).wrapping_add(c.r64(9))) else {
        return regfile::RegfileExit::fault(c.r64(11));
    };
    c.w32(3, bc);
    let mixed = c.r64(0) ^ c.r64(13) ^ c.r64(14);
    let slot = c.r64(5).wrapping_add(c.r64(3).wrapping_mul(8)).wrapping_sub(0x20);
    if !c.st64(slot, mixed) {
        return regfile::RegfileExit::fault(c.r64(11));
    }
    c.arith_sd(1, 0, c.xmm_u64(1));
    let next_off = c.add(c.r64(9), 1, 64);
    c.w64(9, next_off);
    c.cmp(c.r64(9), 1, 64);
    let Some(next_fn) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else {
        return regfile::RegfileExit::fault(c.r64(11));
    };
    c.w64(1, next_fn);
    regfile::RegfileExit::jump(c.r64(1))
}

unsafe extern "C" fn test_rf_b2(raw: *mut u8, _blk: u32) -> regfile::RegfileExit {
    let mut c = regfile::ctx(raw);
    c.w8(2, c.cc_e() as u64);
    c.w64(10, 0xCAFE_BABE_1234_5678);
    if !c.push64(c.r64(10)) {
        return regfile::RegfileExit::fault(0);
    }
    if !c.pop64_reg(10) {
        return regfile::RegfileExit::fault(0);
    }
    let sum = c.add(c.r64(0), c.r64(10), 64);
    c.w64(0, sum);
    c.ret(0, 0)
}

unsafe extern "C" fn test_rf_fault_at_r11(raw: *mut u8, _blk: u32) -> regfile::RegfileExit {
    let mut c = regfile::ctx(raw);
    let r = c.add(c.r64(0), 0x55, 64);
    c.w64(0, r);
    // Simulate hitting an unmapped load / `int3` at address stored in `r11`:
    // leaves `rax = rax_in + 0x55` committed and hands `r11` to the guest interpreter via `SKIP_VA`.
    regfile::RegfileExit::fault(c.r64(11))
}

/// `DF_REGFILE` end-to-end dispatch, `run_chain` (`RF_JUMP -> RF_RET` Ignition handler chain),
/// `RF_FAULT` + one-shot `SKIP_VA` handoff without re-entry loop, and `set_regfile_off_for_test`
/// oracle parity across direct entry, guest `call`, guest `jmp`, interpreter, and JIT.
#[test]
fn dll_regfile_dispatch_chain_fault_and_toggle_match_oracle() {
    std::thread::Builder::new()
        .name("dll_regfile_unit".into())
        .spawn(|| unsafe {
            let code = arena_alloc(0x2000, 0x1000) as u64;
            core::ptr::write_bytes(code as *mut u8, 0xCC, 0x2000);
            let b1_va = code + 0x100;
            let b2_va = code + 0x200;
            let bf_va = code + 0x300;
            let caller_va = code + 0x400;

            // Assemble matching x86 bodies at `b1_va` and `b2_va` so both `run_oracle` and
            // unrouted `emu_run_full` execute the exact same architectural instructions.
            let b1_len = {
                let mut a = CodeAssembler::new(64).unwrap();
                a.movzx(ebx, byte_ptr(r12 + r9)).unwrap();
                a.mov(rcx, rax).unwrap();
                a.xor(rcx, r13).unwrap();
                a.xor(rcx, r14).unwrap();
                a.mov(qword_ptr(rbp + rbx * 8 - 0x20), rcx).unwrap();
                a.subsd(xmm0, xmm1).unwrap();
                a.add(r9, 1).unwrap();
                a.cmp(r9, 1).unwrap();
                a.mov(rcx, qword_ptr(r15 + rbx * 8)).unwrap();
                a.jmp(rcx).unwrap();
                let b = a.assemble(b1_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), b1_va as *mut u8, b.len());
                b.len()
            };
            let b2_len = {
                let mut a = CodeAssembler::new(64).unwrap();
                a.sete(dl).unwrap();
                a.mov(r10, 0xCAFE_BABE_1234_5678u64).unwrap();
                a.push(r10).unwrap();
                a.pop(r10).unwrap();
                a.add(rax, r10).unwrap();
                a.ret().unwrap();
                let b = a.assemble(b2_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), b2_va as *mut u8, b.len());
                b.len()
            };

            // `bf_va`: first instruction `add rax, 0x55`, second instruction at `bf_va + 4`: `add rax, 0x11; ret`.
            // When `test_rf_fault_at_r11` returns `RF_FAULT(bf_va)` (at entry!), `SKIP_VA` prevents
            // re-entering `test_rf_fault_at_r11` at `bf_va` and lets the guest execute `bf_va` cleanly!
            let bf_cont = {
                let mut a = CodeAssembler::new(64).unwrap();
                a.add(rax, 0x11).unwrap();
                a.ret().unwrap();
                let b = a.assemble(bf_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), bf_va as *mut u8, b.len());
                bf_va
            };

            // 1. Direct `run_oracle` vs `test_rf_b1` and `test_rf_b2` full-Cpu comparison.
            let mem = arena_alloc(0x1000, 16);
            core::ptr::write_bytes(mem, 0, 0x1000);
            let mem_va = mem as u64;
            *mem.add(0x100) = 3; // bytecode byte `rbx = 3`
            core::ptr::write_unaligned(mem.add(0x200 + 3 * 8) as *mut u64, b2_va);
            core::ptr::write_unaligned(mem.add(0x800) as *mut u64, 0xDEAD_BEEF_0000_0000);

            let mut cpu_o = Cpu::new_zeroed();
            cpu_o.gpr[0] = 0x1111_2222_3333_4444;
            cpu_o.gpr[2] = 0x9999_8888_7777_6600;
            cpu_o.gpr[4] = mem_va + 0x800;
            cpu_o.gpr[5] = mem_va + 0x400;
            cpu_o.gpr[9] = 0;
            cpu_o.gpr[11] = b1_va;
            cpu_o.gpr[12] = mem_va + 0x100;
            cpu_o.gpr[13] = 0x5555_0000_0000_0001;
            cpu_o.gpr[14] = 0x7000_0000_0000_0000;
            cpu_o.gpr[15] = mem_va + 0x200;
            cpu_o.xmm[0] = (0xABCDu128 << 64) | f64::to_bits(10.5) as u128;
            cpu_o.xmm[1] = (0x1234u128 << 64) | f64::to_bits(3.25) as u128;
            let mut cpu_r = cpu_o.clone();

            let ex_o1 = regfile::run_oracle(&mut cpu_o, b1_va, b1_va, b1_va + b1_len as u64);
            let slot_o = core::ptr::read_unaligned((mem_va + 0x400 + 24 - 0x20) as *const u64);
            core::ptr::write_unaligned((mem_va + 0x400 + 24 - 0x20) as *mut u64, 0);
            let ex_r1 = test_rf_b1(&mut cpu_r as *mut Cpu as *mut u8, 0);
            let slot_r = core::ptr::read_unaligned((mem_va + 0x400 + 24 - 0x20) as *const u64);
            assert_eq!(ex_o1, regfile::RegfileExit::jump(b2_va));
            assert_eq!(ex_r1, ex_o1);
            assert_eq!(slot_r, slot_o);
            assert_eq!(cpu_r.gpr, cpu_o.gpr);
            assert_eq!(cpu_r.xmm, cpu_o.xmm);
            assert_eq!(cpu_r.rflags(), cpu_o.rflags());
            assert_eq!(cpu_r.last_bits, cpu_o.last_bits);

            let ex_o2 = regfile::run_oracle(&mut cpu_o, b2_va, b2_va, b2_va + b2_len as u64);
            let ex_r2 = test_rf_b2(&mut cpu_r as *mut Cpu as *mut u8, 0);
            assert_eq!(ex_o2, regfile::RegfileExit::ret(0xDEAD_BEEF_0000_0000));
            assert_eq!(ex_r2, ex_o2);
            assert_eq!(cpu_r.gpr, cpu_o.gpr);
            assert_eq!(cpu_r.rflags(), cpu_o.rflags());
            assert_eq!(cpu_r.last_bits, cpu_o.last_bits);

            // 2. Register `b1_va`, `b2_va`, and `bf_va` in `REGFILE_TAB` and test chained execution
            //    (`b1 -> b2` in 0 guest instructions!) across interpreter and JIT.
            crate::dll_native::lift::register_regfile_vas(&[
                (b1_va, test_rf_b1, 0),
                (b2_va, test_rf_b2, 0),
                (bf_va, test_rf_fault_at_r11, 0),
            ]);
            let _ = emu_run_full(code, [0; 8], [0; 4], [0; 8]);
            assert_eq!(test_translate(b1_va), None, "DF_REGFILE entry must not translate into a guest block");

            // Assemble guest caller setting V8 registers and calling `b1_va`, then calling `bf_va`
            // (`r11 = bf_cont`) to exercise `RF_FAULT` + `SKIP_VA` fallback in the same flow!
            let caller_insns = {
                let mut a = CodeAssembler::new(64).unwrap();
                a.push(rbp).unwrap();
                a.push(rbx).unwrap();
                a.push(r12).unwrap();
                a.push(r13).unwrap();
                a.push(r14).unwrap();
                a.push(r15).unwrap();
                a.mov(rbp, mem_va + 0x400).unwrap();
                a.mov(r12, mem_va + 0x100).unwrap();
                a.mov(r13, 0x5555_0000_0000_0001u64).unwrap();
                a.mov(r14, 0x7000_0000_0000_0000u64).unwrap();
                a.mov(r15, mem_va + 0x200).unwrap();
                a.xor(r9d, r9d).unwrap();
                a.mov(rax, rcx).unwrap();
                a.mov(rdx, 0xABCD_EF00u64).unwrap();
                a.call(b1_va).unwrap();
                // Fold `dl` (`sete dl` = 1) into `rax`.
                a.movzx(ecx, dl).unwrap();
                a.add(rax, rcx).unwrap();
                a.pop(r15).unwrap();
                a.pop(r14).unwrap();
                a.pop(r13).unwrap();
                a.pop(r12).unwrap();
                a.pop(rbx).unwrap();
                a.pop(rbp).unwrap();
                a.ret().unwrap();
                let b = a.assemble(caller_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), caller_va as *mut u8, b.len());
                let mut dec = iced_x86::Decoder::with_ip(64, &b, caller_va, iced_x86::DecoderOptions::NONE);
                let mut n = 0u64;
                while dec.can_decode() {
                    let _ = dec.decode();
                    n += 1;
                }
                n
            };

            // Oracle with `set_regfile_off_for_test(true)`: runs all guest instructions of `b1` + `b2`.
            regfile::set_regfile_off_for_test(true);
            jit::force(2);
            core::ptr::write_unaligned((mem_va + 0x400 + 24 - 0x20) as *mut u64, 0);
            let (want_rax, want_xmm0) = emu_run_full(
                caller_va,
                [0x2222_3333_4444_5555, 0, 0, 0, 0, 0, 0, 0],
                [f64::to_bits(20.0) as u128, f64::to_bits(4.5) as u128, 0, 0],
                [0; 8],
            );
            let want_slot = core::ptr::read_unaligned((mem_va + 0x400 + 24 - 0x20) as *const u64);
            let unrouted_steps = test_last_steps();
            assert!(unrouted_steps > caller_insns + 10);
            regfile::set_regfile_off_for_test(false);

            for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                jit::force(mode);
                for pass in 0..2 {
                    core::ptr::write_unaligned((mem_va + 0x400 + 24 - 0x20) as *mut u64, 0);
                    let rf0 = regfile::regfile_runs();
                    let (got_rax, got_xmm0) = emu_run_full(
                        caller_va,
                        [0x2222_3333_4444_5555, 0, 0, 0, 0, 0, 0, 0],
                        [f64::to_bits(20.0) as u128, f64::to_bits(4.5) as u128, 0, 0],
                        [0; 8],
                    );
                    let got_slot = core::ptr::read_unaligned((mem_va + 0x400 + 24 - 0x20) as *const u64);
                    assert_eq!(got_rax, want_rax, "{mname} pass {pass}: chained rax parity");
                    assert_eq!(got_xmm0 as u64, want_xmm0 as u64, "{mname} pass {pass}: xmm0 subsd parity");
                    assert_eq!(got_slot, want_slot, "{mname} pass {pass}: frame slot store parity");
                    assert_eq!(
                        regfile::regfile_runs() - rf0,
                        2,
                        "{mname} pass {pass}: run_chain executed b1 -> b2 in 2 regfile hops"
                    );
                    assert_eq!(
                        test_last_steps(),
                        caller_insns,
                        "{mname} pass {pass}: 0 guest steps executed inside b1+b2"
                    );
                }
                jit::force(0);
            }

            // 3. `RF_FAULT` at entry (`bf_va`) arms one-shot `SKIP_VA` so the interpreter steps
            //    `bf_va` (`add rax, 0x11; ret`) cleanly without re-entering `test_rf_fault_at_r11`.
            let fault_caller = code + 0x600;
            {
                let mut a = CodeAssembler::new(64).unwrap();
                a.mov(rax, 0x100u64).unwrap();
                a.mov(r11, bf_cont).unwrap();
                a.call(bf_va).unwrap();
                a.ret().unwrap();
                let b = a.assemble(fault_caller).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), fault_caller as *mut u8, b.len());
            }
            for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                jit::force(mode);
                let rf0 = regfile::regfile_runs();
                let (got, _) = emu_run_full(fault_caller, [0; 8], [0; 4], [0; 8]);
                assert_eq!(got, 0x100 + 0x55 + 0x11, "{mname}: RF_FAULT + SKIP_VA fallback ran once");
                assert_eq!(regfile::regfile_runs() - rf0, 1, "{mname}: no infinite re-entry after RF_FAULT");
                jit::force(0);
            }

            crate::dll_native::lift::clear_regfile_va_for_test(b1_va);
            crate::dll_native::lift::clear_regfile_va_for_test(b2_va);
            crate::dll_native::lift::clear_regfile_va_for_test(bf_va);
        })
        .unwrap()
        .join()
        .unwrap();
}

unsafe extern "C" fn test_rf_p2_outer(raw: *mut u8, blk: u32) -> regfile::RegfileExit {
    let mut c = regfile::ctx(raw);
    loop {
        match blk {
            0 => {
                if !c.push64(c.r64(5)) {
                    return regfile::RegfileExit::fault(0);
                }
                c.w64(5, c.r64(4));
                let new_sp = c.sub(c.r64(4), 0x20, 64);
                c.w64(4, new_sp);
                if !c.st64(c.r64(5).wrapping_sub(8), c.r64(13)) {
                    return regfile::RegfileExit::fault(0);
                }
                // `r10` holds `inner_va`, `r11` holds `outer_cont_va`, `r12` holds `outer_handler_va`.
                if !c.st64(c.r64(5).wrapping_sub(16), c.r64(12)) {
                    return regfile::RegfileExit::fault(0);
                }
                return c.call(c.r64(10), c.r64(11), 0);
            }
            1 => {
                let Some(saved_r13) = c.ld64(c.r64(5).wrapping_sub(8)) else {
                    return regfile::RegfileExit::fault(0);
                };
                let sum = c.add(c.r64(0), saved_r13, 64);
                c.w64(0, sum);
                if !c.leave() {
                    return regfile::RegfileExit::fault(0);
                }
                return c.ret(0, 0);
            }
            2 => {
                // Exception handler sub-entry (`outer_handler_va`, `blk = 2`): reached directly when
                // `inner` unwinds its frame (`rsp = outer_rbp - 0x20`, `rbp = outer_rbp`) and jumps here.
                let Some(saved_r13) = c.ld64(c.r64(5).wrapping_sub(8)) else {
                    return regfile::RegfileExit::fault(0);
                };
                let v = c.add(saved_r13, 0x7777, 64);
                c.w64(0, v);
                if !c.leave() {
                    return regfile::RegfileExit::fault(0);
                }
                return c.ret(0, 0);
            }
            _ => unreachable!(),
        }
    }
}

unsafe extern "C" fn test_rf_p2_inner(raw: *mut u8, _blk: u32) -> regfile::RegfileExit {
    let mut c = regfile::ctx(raw);
    if !c.push64(c.r64(5)) {
        return regfile::RegfileExit::fault(0);
    }
    c.w64(5, c.r64(4));
    c.cmp(c.r64(1), 2, 64);
    if c.cc_e() {
        // Simulated V8 `UnwindAndFindHandler`: walk `rbp` back to outer frame, read `[outer_rbp - 16]`
        // (`outer_handler_va`), set `rsp = outer_rbp - 0x20`, `rbp = outer_rbp`, and jump to handler!
        let Some(outer_rbp) = c.ld64(c.r64(5)) else {
            return regfile::RegfileExit::fault(0);
        };
        let Some(handler_va) = c.ld64(outer_rbp.wrapping_sub(16)) else {
            return regfile::RegfileExit::fault(0);
        };
        c.w64(5, outer_rbp);
        c.w64(4, outer_rbp.wrapping_sub(0x20));
        return regfile::RegfileExit::jump(handler_va);
    }
    let v = c.add(c.r64(0), 0x42, 64);
    c.w64(0, v);
    if !c.leave() {
        return regfile::RegfileExit::fault(0);
    }
    c.ret(0, 0)
}

/// `DF_REGFILE` P2: builtin-to-builtin `RF_CALL` -> `RF_RET` continuation resumption (`(rf_outer, 1)`)
/// and guest-stack frame-walk unwind (`[rbp]` chain -> `(rf_outer, 2)` handler continuation) in 0 guest
/// instructions, verified bit-for-bit against the unrouted x86 guest oracle across interpreter and JIT.
#[test]
fn v8_regfile_p2_call_ret_stack_frame_and_unwind() {
    std::thread::Builder::new()
        .name("dll_regfile_p2".into())
        .spawn(|| unsafe {
            let code = arena_alloc(0x2000, 0x1000) as u64;
            core::ptr::write_bytes(code as *mut u8, 0xCC, 0x2000);
            let outer_va = code + 0x100;
            let inner_va = code + 0x300;
            let caller_va = code + 0x500;

            let (outer_cont_va, outer_handler_va) = {
                let mut a = CodeAssembler::new(64).unwrap();
                let mut cont_lbl = a.create_label();
                let mut hand_lbl = a.create_label();
                a.push(rbp).unwrap();
                a.mov(rbp, rsp).unwrap();
                a.sub(rsp, 0x20).unwrap();
                a.mov(qword_ptr(rbp - 8), r13).unwrap();
                a.mov(qword_ptr(rbp - 16), r12).unwrap();
                a.call(r10).unwrap();
                a.set_label(&mut cont_lbl).unwrap();
                a.add(rax, qword_ptr(rbp - 8)).unwrap();
                a.leave().unwrap();
                a.ret().unwrap();
                a.set_label(&mut hand_lbl).unwrap();
                a.mov(rax, qword_ptr(rbp - 8)).unwrap();
                a.add(rax, 0x7777).unwrap();
                a.leave().unwrap();
                a.ret().unwrap();
                let b = a.assemble(outer_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), outer_va as *mut u8, b.len());
                // Find exact offsets of `add rax, [rbp - 8]` and `mov rax, [rbp - 8]` after `ret`.
                let mut dec = iced_x86::Decoder::with_ip(64, &b, outer_va, iced_x86::DecoderOptions::NONE);
                let mut cont_ip = 0u64;
                let mut hand_ip = 0u64;
                let mut after_call = false;
                let mut after_ret = false;
                while dec.can_decode() {
                    let ins = dec.decode();
                    if after_call && cont_ip == 0 {
                        cont_ip = ins.ip();
                    }
                    if after_ret && hand_ip == 0 {
                        hand_ip = ins.ip();
                    }
                    if ins.mnemonic() == iced_x86::Mnemonic::Call {
                        after_call = true;
                    }
                    if ins.mnemonic() == iced_x86::Mnemonic::Ret {
                        after_ret = true;
                    }
                }
                assert!(cont_ip > outer_va && hand_ip > cont_ip);
                (cont_ip, hand_ip)
            };

            {
                let mut a = CodeAssembler::new(64).unwrap();
                let mut unwind_lbl = a.create_label();
                a.push(rbp).unwrap();
                a.mov(rbp, rsp).unwrap();
                a.cmp(rcx, 2).unwrap();
                a.je(unwind_lbl).unwrap();
                a.add(rax, 0x42).unwrap();
                a.leave().unwrap();
                a.ret().unwrap();
                a.set_label(&mut unwind_lbl).unwrap();
                a.mov(rbp, qword_ptr(rbp)).unwrap();
                a.mov(rdx, qword_ptr(rbp - 16)).unwrap();
                a.lea(rsp, ptr(rbp - 0x20)).unwrap();
                a.jmp(rdx).unwrap();
                let b = a.assemble(inner_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), inner_va as *mut u8, b.len());
            }

            let caller_insns = {
                let mut a = CodeAssembler::new(64).unwrap();
                a.push(rbp).unwrap();
                a.push(r12).unwrap();
                a.push(r13).unwrap();
                a.mov(rax, rdx).unwrap();
                a.mov(r10, inner_va).unwrap();
                a.mov(r11, outer_cont_va).unwrap();
                a.mov(r12, outer_handler_va).unwrap();
                a.mov(r13, 0x1000_0000_0005u64).unwrap();
                a.call(outer_va).unwrap();
                a.pop(r13).unwrap();
                a.pop(r12).unwrap();
                a.pop(rbp).unwrap();
                a.ret().unwrap();
                let b = a.assemble(caller_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), caller_va as *mut u8, b.len());
                let mut dec = iced_x86::Decoder::with_ip(64, &b, caller_va, iced_x86::DecoderOptions::NONE);
                let mut n = 0u64;
                while dec.can_decode() {
                    let _ = dec.decode();
                    n += 1;
                }
                n
            };

            crate::dll_native::lift::register_regfile_vas(&[
                (outer_va, test_rf_p2_outer, 0),
                (outer_cont_va, test_rf_p2_outer, 1),
                (outer_handler_va, test_rf_p2_outer, 2),
                (inner_va, test_rf_p2_inner, 0),
            ]);
            let _ = emu_run_full(code, [0; 8], [0; 4], [0; 8]);

            for kind in [1u64, 2u64] {
                regfile::set_regfile_off_for_test(true);
                jit::force(2);
                let (want, _) = emu_run_full(caller_va, [kind, 0x200, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                regfile::set_regfile_off_for_test(false);

                for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                    jit::force(mode);
                    let rf0 = regfile::regfile_runs();
                    let (got, _) = emu_run_full(caller_va, [kind, 0x200, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                    assert_eq!(got, want, "{mname} kind={kind}: P2 call/ret/unwind parity");
                    assert_eq!(
                        regfile::regfile_runs() - rf0,
                        3,
                        "{mname} kind={kind}: outer(0) -> inner(0) -> outer(cont) in 3 regfile hops"
                    );
                    assert_eq!(
                        test_last_steps(),
                        caller_insns,
                        "{mname} kind={kind}: 0 guest steps inside outer+inner"
                    );
                    jit::force(0);
                }
            }

            for va in [outer_va, outer_cont_va, outer_handler_va, inner_va] {
                crate::dll_native::lift::clear_regfile_va_for_test(va);
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

unsafe extern "C" fn test_p3_win64_runtime(a1: u64, a2: u64, nested_entry: u64, nested_arg: u64) -> u64 {
    let nested = if nested_entry != 0 {
        let (r, _) = emu_run_full(nested_entry, [nested_arg, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        r
    } else {
        0
    };
    a1.wrapping_add(a2).wrapping_add(nested)
}

unsafe extern "C" fn test_rf_p3_centry(raw: *mut u8, blk: u32) -> regfile::RegfileExit {
    let mut c = regfile::ctx(raw);
    loop {
        match blk {
            0 => {
                if !c.push64(c.r64(5)) {
                    return regfile::RegfileExit::fault(0);
                }
                c.w64(5, c.r64(4));
                // Allocate 0x20 Win64 shadow space (`rsp` 16B-aligned before `call`!).
                let new_sp = c.sub(c.r64(4), 0x20, 64);
                c.w64(4, new_sp);
                // `r10` = Win64 C++ runtime target, `r11` = `centry_cont_va`.
                return c.call(c.r64(10), c.r64(11), 0);
            }
            1 => {
                // Verify non-volatile V8 `r13` + `r14` survived Win64 callout AND nested `JSEntry`!
                let v = c.add(c.r64(0), c.r64(13) ^ c.r64(14), 64);
                c.w64(0, v);
                if !c.leave() {
                    return regfile::RegfileExit::fault(0);
                }
                return c.ret(0, 0);
            }
            _ => unreachable!(),
        }
    }
}

unsafe extern "C" fn test_rf_p3_nested_leaf(raw: *mut u8, _blk: u32) -> regfile::RegfileExit {
    let mut c = regfile::ctx(raw);
    // Clobbers `r13`/`r14` inside the *nested* `emu_run_full` Cpu — must NOT leak to outer Cpu!
    c.w64(13, 0xBAD0_BAD0_BAD0_BAD0);
    c.w64(14, 0xDEAD_DEAD_DEAD_DEAD);
    let r = c.imul2(c.r64(1), 7, 64);
    c.w64(0, r);
    c.ret(0, 0)
}

/// `DF_REGFILE` P3: Win64 C++ runtime callout (`CEntry` / `CallApiGetter`) via `RF_CALL -> host_tail_call`
/// with nested `JSEntry` (`emu_run_full`) re-entrancy into a second `DF_REGFILE` builtin, followed by
/// direct continuation resumption `(test_rf_p3_centry, 1)` without guest steps or register clobbers.
#[test]
fn v8_regfile_p3_win64_host_callout_and_nested_reentrancy() {
    std::thread::Builder::new()
        .name("dll_regfile_p3".into())
        .spawn(|| unsafe {
            let code = arena_alloc(0x2000, 0x1000) as u64;
            core::ptr::write_bytes(code as *mut u8, 0xCC, 0x2000);
            let centry_va = code + 0x100;
            let centry_cont_va = code + 0x120;
            let nested_va = code + 0x200;
            let host_va = code + 0x300;
            let caller_va = code + 0x400;

            crate::exe::register_native_va(host_va, test_p3_win64_runtime as *const () as usize);
            crate::dll_native::lift::register_regfile_vas(&[
                (centry_va, test_rf_p3_centry, 0),
                (centry_cont_va, test_rf_p3_centry, 1),
                (nested_va, test_rf_p3_nested_leaf, 0),
            ]);
            let _ = emu_run_full(code, [0; 8], [0; 4], [0; 8]);

            let caller_insns = {
                let mut a = CodeAssembler::new(64).unwrap();
                a.push(rbp).unwrap();
                a.push(r13).unwrap();
                a.push(r14).unwrap();
                a.sub(rsp, 0x20).unwrap();
                a.mov(r13, 0x1234_0000_0001u64).unwrap();
                a.mov(r14, 0x0234_0000_0000u64).unwrap();
                a.mov(r10, host_va).unwrap();
                a.mov(r11, centry_cont_va).unwrap();
                // Win64 args to `test_p3_win64_runtime(100, 200, nested_va, 9)` -> `300 + 63 = 363`,
                // then `test_rf_p3_centry` `blk = 1` adds `r13 ^ r14` (`0x1000_0000_0001`).
                a.mov(ecx, 100).unwrap();
                a.mov(edx, 200).unwrap();
                a.mov(r8, nested_va).unwrap();
                a.mov(r9d, 9).unwrap();
                a.call(centry_va).unwrap();
                a.add(rsp, 0x20).unwrap();
                a.pop(r14).unwrap();
                a.pop(r13).unwrap();
                a.pop(rbp).unwrap();
                a.ret().unwrap();
                let b = a.assemble(caller_va).unwrap();
                core::ptr::copy_nonoverlapping(b.as_ptr(), caller_va as *mut u8, b.len());
                let mut dec = iced_x86::Decoder::with_ip(64, &b, caller_va, iced_x86::DecoderOptions::NONE);
                let mut n = 0u64;
                while dec.can_decode() {
                    let _ = dec.decode();
                    n += 1;
                }
                n
            };

            let want = 363u64.wrapping_add(0x1234_0000_0001u64 ^ 0x0234_0000_0000u64);
            for (mode, mname) in [(2u8, "interpreter"), (1u8, "translator")] {
                jit::force(mode);
                let rf0 = regfile::regfile_runs();
                let (got, _) = emu_run_full(caller_va, [0; 8], [0; 4], [0; 8]);
                assert_eq!(got, want, "{mname}: P3 Win64 callout + nested JSEntry re-entrancy parity");
                assert_eq!(
                    regfile::regfile_runs() - rf0,
                    3,
                    "{mname}: centry(0) -> host -> nested_leaf(0) -> centry(1) = 3 regfile runs"
                );
                assert_eq!(
                    test_last_steps(),
                    caller_insns,
                    "{mname}: 0 guest steps inside centry or nested_leaf"
                );
                jit::force(0);
            }

            crate::exe::unregister_native_va(host_va);
            for va in [centry_va, centry_cont_va, nested_va] {
                crate::dll_native::lift::clear_regfile_va_for_test(va);
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

unsafe extern "C" fn capture_two_args(a: u64, b: u64) -> u64 {
    (a & 0xFFFF_FFFF) | ((b & 0xFFFF_FFFF) << 32)
}

/// Partial-register ops (`mov cl, cl`, `add dl, imm8`) before a standard Win32/CRT host call
/// (`DF_GAME_REPL == 0`) must preserve full 32/64-bit register values (matching real x86-64
/// hardware; regression for obfuscated callers passing `CP_UTF8 = 0xFDE9` to `WideCharToMultiByte`).
#[test]
fn partial_reg_ops_preserve_full_host_call_args() {
    use iced_x86::code_asm::*;

    let h = arena_alloc(64, 16) as u64;
    let code = arena_alloc(0x1000, 16) as u64;
    crate::exe::register_native_va(h, capture_two_args as *const () as usize);

    let mut a = CodeAssembler::new(64).unwrap();
    a.sub(rsp, 0x28).unwrap();
    a.mov(ecx, 0xFDE9u32).unwrap();
    a.mov(cl, cl).unwrap();
    a.mov(edx, 0xFD00u32).unwrap();
    a.add(dl, 0xE9i32).unwrap();
    a.call(h).unwrap();
    a.add(rsp, 0x28).unwrap();
    a.ret().unwrap();
    let bytes = a.assemble(code).unwrap();
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), code as *mut u8, bytes.len()) };

    let want = 0x0000_FDE9_0000_FDE9u64;
    for (mode, name) in [(2u8, "interpreter"), (1u8, "translator")] {
        jit::force(mode);
        let (got, _) = unsafe { emu_run_full(code, [0; 8], [0; 4], [0; 8]) };
        assert_eq!(got, want, "{name}: partial-reg ops must not truncate host-call arguments");
        jit::force(0);
    }
    crate::exe::unregister_native_va(h);
}

/// Entering `emu_run_full` on the emulator's heap guest stack updates `TEB.StackBase`/`StackLimit`
/// to cover that stack (so unhooked `__chkstk` never probes into the host pthread guard page)
/// and restores the outer TEB bounds on return.
#[test]
fn emu_stack_updates_teb_stack_bounds_for_unhooked_chkstk() {
    use iced_x86::code_asm::*;

    std::thread::Builder::new()
        .stack_size(8 << 20)
        .spawn(|| unsafe {
            let marker = 0u64;
            let host_sp = &marker as *const u64 as usize;
            let host_hi = (host_sp + 0x10000) & !0xFFF;
            let host_lo = (host_sp - 0x10000) & !0xFFF;
            let teb = win32_rt::teb::init_thread(host_hi, host_lo, 0x9990);

            let out = arena_alloc(32, 16) as u64;
            let code = arena_alloc(0x1000, 16) as u64;
            let mut a = CodeAssembler::new(64).unwrap();
            a.mov(rax, qword_ptr(0x08).gs()).unwrap(); // TEB.StackBase
            a.mov(qword_ptr(rcx), rax).unwrap();
            a.mov(rax, qword_ptr(0x10).gs()).unwrap(); // TEB.StackLimit
            a.mov(qword_ptr(rcx + 8), rax).unwrap();
            a.mov(qword_ptr(rcx + 16), rsp).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(code).unwrap();
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), code as *mut u8, bytes.len());

            for mode in [2u8, 1u8] {
                jit::force(mode);
                let _ = emu_run_full(code, [out, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
                let base = core::ptr::read_unaligned(out as *const u64);
                let limit = core::ptr::read_unaligned((out + 8) as *const u64);
                let guest_sp = core::ptr::read_unaligned((out + 16) as *const u64);
                assert!(limit < guest_sp && guest_sp <= base, "mode {mode}: limit={limit:#x} rsp={guest_sp:#x} base={base:#x}");
                assert_ne!(base as usize, host_hi, "mode {mode}: TEB.StackBase must switch off host pthread stack");
                jit::force(0);
            }

            assert_eq!(core::ptr::read_unaligned(teb.add(0x08) as *const u64) as usize, host_hi);
            assert_eq!(core::ptr::read_unaligned(teb.add(0x10) as *const u64) as usize, host_lo);
            win32_rt::teb::uninstall();
            win32_rt::teb::free_teb(teb);
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Regression tests for Phase 5/6 AVX/YMM optimizations:
/// - A block that reads live-in `ymm0[255:128]` (`vmovups [mem], ymm0`) before a VEX-128 write
///   to `xmm0` must not clobber the live-in upper half before the read.
/// - `vzeroupper` (out-of-line `clear_ymm_hi` via packed `XBUD` bit 0) must zero all upper YMM
///   halves while preserving every guest CPU flag (`CF/ZF/SF/OF`).
#[test]
fn ymm_hi_live_in_and_vzeroupper_preserves_flags() {
    use iced_x86::code_asm::*;

    let buf_in = arena_alloc(128, 32) as u64;
    let buf_out = arena_alloc(128, 32) as u64;
    let code = arena_alloc(0x1000, 16) as u64;

    // Input ymm0 = [0x11..; 32], ymm1 = [0x22..; 32]
    unsafe {
        for i in 0..32usize {
            *((buf_in as *mut u8).add(i)) = (0x10 + i) as u8;
            *((buf_in as *mut u8).add(32 + i)) = (0x80 + i) as u8;
        }
    }

    let mut a = CodeAssembler::new(64).unwrap();
    let mut b2 = a.create_label();
    // Block 1: load ymm0 and ymm1 (sets dirty ymm_hi + packed XBUD bit 0), then jump to Block 2.
    a.vmovups(ymm0, ymmword_ptr(rcx)).unwrap();
    a.vmovups(ymm1, ymmword_ptr(rcx + 32)).unwrap();
    a.jmp(b2).unwrap();

    // Block 2: read live-in ymm0 BEFORE VEX-128 vaddss writes xmm0, then test vzeroupper flag preservation.
    a.set_label(&mut b2).unwrap();
    a.vmovups(ymmword_ptr(rdx), ymm0).unwrap();
    a.vaddss(xmm0, xmm0, xmm1).unwrap();
    a.vmovups(ymmword_ptr(rdx + 32), ymm0).unwrap();

    // Set distinct NZCV flags via `sub` (r8d - r9d), invoke `vzeroupper`, then capture flags via setcc.
    a.mov(eax, r8d).unwrap();
    a.sub(eax, r9d).unwrap();
    a.vzeroupper().unwrap();
    a.sete(r10b).unwrap();
    a.setb(r11b).unwrap();
    a.setl(cl).unwrap();
    a.seto(r9b).unwrap();
    a.vmovups(ymmword_ptr(rdx + 64), ymm1).unwrap();
    a.movzx(eax, r10b).unwrap();
    a.movzx(r11d, r11b).unwrap();
    a.shl(r11d, 1).unwrap();
    a.or(eax, r11d).unwrap();
    a.movzx(r10d, cl).unwrap();
    a.shl(r10d, 2).unwrap();
    a.or(eax, r10d).unwrap();
    a.movzx(r10d, r9b).unwrap();
    a.shl(r10d, 3).unwrap();
    a.or(eax, r10d).unwrap();
    a.ret().unwrap();

    let bytes = a.assemble(code).unwrap();
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), code as *mut u8, bytes.len()) };

    for &(lhs, rhs) in &[(5u64, 5u64), (3, 10), (0x8000_0000, 1), (0x7fff_ffff, 0xffff_ffff)] {
        let mut snap = [[0u8; 96]; 2];
        let mut rets = [0u64; 2];
        for (idx, mode) in [(0, 2u8), (1, 1u8)] {
            unsafe { core::ptr::write_bytes(buf_out as *mut u8, 0xCC, 96) };
            jit::force(mode);
            let (ret, _) = unsafe { emu_run_full(code, [buf_in, buf_out, lhs, rhs, 0, 0, 0, 0], [0; 4], [0; 8]) };
            jit::force(0);
            rets[idx] = ret;
            unsafe { core::ptr::copy_nonoverlapping(buf_out as *const u8, snap[idx].as_mut_ptr(), 96) };
        }
        assert_eq!(rets[1], rets[0], "vzeroupper flags mismatch for ({lhs:#x}, {rhs:#x})");
        assert_eq!(snap[1], snap[0], "ymm live-in / vzeroupper memory mismatch for ({lhs:#x}, {rhs:#x})");
        // Explicitly check live-in ymm0[255:128] was preserved and ymm1[255:128] was zeroed after vzeroupper.
        assert_eq!(&snap[1][16..32], &(16..32u8).map(|i| 0x10 + i).collect::<Vec<_>>()[..]);
        assert_eq!(&snap[1][48..64], &[0u8; 16]);
        assert_eq!(&snap[1][80..96], &[0u8; 16]);
    }
}

/// Parity test for fused `Term::JccPair` (`ucomiss`/`ucomisd` + `jp`/`jnp` + `je`/`jne`) across
/// `<`, `==`, `>`, `+0.0/-0.0`, and `qNaN`.
#[test]
fn jcc_pair_ucomiss_ucomisd_parity() {
    use iced_x86::code_asm::*;

    let code = arena_alloc(0x1000, 16) as u64;
    let mut a = CodeAssembler::new(64).unwrap();
    let mut not_eq_f32 = a.create_label();
    let mut eq_f32 = a.create_label();
    let mut done_f32 = a.create_label();
    let mut unordered_or_ne_f64 = a.create_label();
    let mut eq_f64 = a.create_label();
    let mut done = a.create_label();

    // Test 1: f32 `a == b` idiom: ucomiss xmm0, xmm1; jp not_eq; je eq; jmp not_eq
    a.xor(eax, eax).unwrap();
    a.ucomiss(xmm0, xmm1).unwrap();
    a.jp(not_eq_f32).unwrap();
    a.je(eq_f32).unwrap();
    a.set_label(&mut not_eq_f32).unwrap();
    a.or(eax, 1).unwrap();
    a.jmp(done_f32).unwrap();
    a.set_label(&mut eq_f32).unwrap();
    a.or(eax, 2).unwrap();
    a.set_label(&mut done_f32).unwrap();

    // Test 2: f64 `a != b` idiom: ucomisd xmm2, xmm3; jp unord_ne; jne unord_ne; eq:
    a.ucomisd(xmm2, xmm3).unwrap();
    a.jp(unordered_or_ne_f64).unwrap();
    a.jne(unordered_or_ne_f64).unwrap();
    a.set_label(&mut eq_f64).unwrap();
    a.or(eax, 4).unwrap();
    a.jmp(done).unwrap();
    a.set_label(&mut unordered_or_ne_f64).unwrap();
    a.or(eax, 8).unwrap();
    a.set_label(&mut done).unwrap();
    a.ret().unwrap();

    let bytes = a.assemble(code).unwrap();
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), code as *mut u8, bytes.len()) };

    let f32_cases: [(f32, f32); 5] = [
        (1.5, 1.5),
        (0.0, -0.0),
        (1.0, 2.0),
        (3.0, -1.0),
        (f32::NAN, 1.0),
    ];
    let f64_cases: [(f64, f64); 5] = [
        (2.5, 2.5),
        (-0.0, 0.0),
        (-4.0, 4.0),
        (9.0, 1.0),
        (1.0, f64::NAN),
    ];
    for &(a32, b32) in &f32_cases {
        for &(a64, b64) in &f64_cases {
            let xmms = [
                a32.to_bits() as u128,
                b32.to_bits() as u128,
                a64.to_bits() as u128,
                b64.to_bits() as u128,
            ];
            jit::force(2);
            let (want, _) = unsafe { emu_run_full(code, [0; 8], xmms, [0; 8]) };
            jit::force(1);
            let (got, _) = unsafe { emu_run_full(code, [0; 8], xmms, [0; 8]) };
            jit::force(0);
            assert_eq!(got, want, "JccPair mismatch for f32=({a32},{b32}) f64=({a64},{b64})");
        }
    }
}

/// Parity test for Phase 6 batched `push`/`sub rsp` prologues, hoisted `add rsp`/`pop`/`ret`
/// epilogues (with shadow RAS `XRAS`), direct NEON `x_shuf_rr` (`vshufps` self-shuffles,
/// `vpshufd`, `vunpcklps`, `vunpckhps`, `vmovddup`), indexed `[base + idx*4 + disp]` XA reuse,
/// and 32-bit `lea`.
#[test]
fn batched_prologue_epilogue_shuf_and_indexed_lea_parity() {
    use iced_x86::code_asm::*;

    let data = arena_alloc(256, 16) as u64;
    let out = arena_alloc(256, 16) as u64;
    let code = arena_alloc(0x1000, 16) as u64;

    unsafe {
        for i in 0..32u32 {
            *((data as *mut f32).add(i as usize)) = (i as f32) * 1.25 - 7.5;
        }
    }

    let mut a = CodeAssembler::new(64).unwrap();
    let mut callee = a.create_label();

    // Entry: batched prologue -> call callee -> batched epilogue
    a.push(rbx).unwrap();
    a.push(rbp).unwrap();
    a.push(rsi).unwrap();
    a.push(rdi).unwrap();
    a.push(r12).unwrap();
    a.push(r13).unwrap();
    a.sub(rsp, 0x38).unwrap();
    a.mov(rbx, rcx).unwrap();
    a.mov(rsi, rdx).unwrap();
    a.mov(r12d, 3).unwrap();
    a.call(callee).unwrap();
    a.add(rsp, 0x38).unwrap();
    a.pop(r13).unwrap();
    a.pop(r12).unwrap();
    a.pop(rdi).unwrap();
    a.pop(rsi).unwrap();
    a.pop(rbp).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();

    // Callee: batched prologue, indexed vec3 loads (XA cache reuse), direct shuffles, 32-bit lea, batched epilogue.
    a.set_label(&mut callee).unwrap();
    a.push(rbx).unwrap();
    a.push(rdi).unwrap();
    a.push(r14).unwrap();
    a.sub(rsp, 0x20).unwrap();

    // Indexed vec3 scalar loads sharing base+idx*4
    a.vmovss(xmm0, dword_ptr(rbx + r12 * 4)).unwrap();
    a.vmovss(xmm1, dword_ptr(rbx + r12 * 4 + 4)).unwrap();
    a.vmovss(xmm2, dword_ptr(rbx + r12 * 4 + 8)).unwrap();
    a.vaddss(xmm0, xmm0, xmm1).unwrap();
    a.vaddss(xmm0, xmm0, xmm2).unwrap();
    a.vmovss(dword_ptr(rsi), xmm0).unwrap();

    // Direct NEON shuffles on 128-bit vectors
    a.vmovups(xmm3, xmmword_ptr(rbx)).unwrap();
    a.vmovups(xmm4, xmmword_ptr(rbx + 16)).unwrap();
    a.vshufps(xmm5, xmm3, xmm3, 0x00).unwrap();
    a.vmovups(xmmword_ptr(rsi + 16), xmm5).unwrap();
    a.vshufps(xmm5, xmm3, xmm3, 0x55).unwrap();
    a.vmovups(xmmword_ptr(rsi + 32), xmm5).unwrap();
    a.vshufps(xmm5, xmm3, xmm3, 0xAA).unwrap();
    a.vmovups(xmmword_ptr(rsi + 48), xmm5).unwrap();
    a.vshufps(xmm5, xmm3, xmm3, 0xFF).unwrap();
    a.vmovups(xmmword_ptr(rsi + 64), xmm5).unwrap();
    a.vshufps(xmm5, xmm3, xmm3, 0x4E).unwrap();
    a.vmovups(xmmword_ptr(rsi + 80), xmm5).unwrap();
    a.vshufps(xmm5, xmm3, xmm3, 0xB1).unwrap();
    a.vmovups(xmmword_ptr(rsi + 96), xmm5).unwrap();
    a.vpshufd(xmm5, xmm4, 0x1B).unwrap();
    a.vmovups(xmmword_ptr(rsi + 112), xmm5).unwrap();
    a.vunpcklps(xmm5, xmm3, xmm4).unwrap();
    a.vmovups(xmmword_ptr(rsi + 128), xmm5).unwrap();
    a.vunpckhps(xmm5, xmm3, xmm4).unwrap();
    a.vmovups(xmmword_ptr(rsi + 144), xmm5).unwrap();
    a.vmovddup(xmm5, xmm3).unwrap();
    a.vmovups(xmmword_ptr(rsi + 160), xmm5).unwrap();

    // 32-bit scaled LEA (zero-extends to 64-bit rax)
    a.mov(eax, 0x1234_5678u32).unwrap();
    a.lea(eax, dword_ptr(rax + r12 * 4 + 0x10)).unwrap();

    a.add(rsp, 0x20).unwrap();
    a.pop(r14).unwrap();
    a.pop(rdi).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();

    let bytes = a.assemble(code).unwrap();
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), code as *mut u8, bytes.len()) };

    let mut snap = [[0u8; 176]; 2];
    let mut rets = [0u64; 2];
    for (idx, mode) in [(0, 2u8), (1, 1u8)] {
        unsafe { core::ptr::write_bytes(out as *mut u8, 0x55, 176) };
        jit::force(mode);
        let (ret, _) = unsafe { emu_run_full(code, [data, out, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
        jit::force(0);
        rets[idx] = ret;
        unsafe { core::ptr::copy_nonoverlapping(out as *const u8, snap[idx].as_mut_ptr(), 176) };
    }
    assert_eq!(rets[1], rets[0], "batched prologue/epilogue + 32-bit LEA return value mismatch");
    assert_eq!(snap[1], snap[0], "shuf / indexed load memory mismatch");
}

/// Tail jumps (`jmp reg` and `jmp [mem]`) to `DF_PLAIN` builtins must cache into `table` on
/// call #1 and hit `lookup_jmp`'s `is_plain_jmp` fast path on calls #2+, preserving all
/// callee-saved GPRs and returning via the shadow RAS with balanced `XRAS`.
#[test]
fn lookup_jmp_plain_tail_call_fast_path_and_ras_preservation() {
    std::thread::Builder::new()
        .name("lookup_jmp_plain".into())
        .spawn(|| {
            let exp = |dll: &str, name: &str| {
                win32_rt::lookup(dll, name).unwrap_or_else(|| panic!("{dll}!{name}")) as usize
            };
            let f_memcpy = exp("msvcrt.dll", "memcpy");
            let f_memset = exp("msvcrt.dll", "memset");
            let h = arena_alloc(64, 16);
            unsafe { core::ptr::write_bytes(h, 0xCC, 64) };
            let (t_memcpy, t_memset) = (h as u64, h as u64 + 16);
            crate::exe::register_native_va(t_memcpy, f_memcpy);
            crate::exe::register_native_va(t_memset, f_memset);

            let c = arena_alloc(0x400, 16);
            let code = (|| -> Result<Vec<u8>, IcedError> {
                let mut a = CodeAssembler::new(64)?;
                let mut loop_lbl = a.create_label();
                let mut tail_reg = a.create_label();
                let mut tail_mem = a.create_label();
                a.push(rbx)?;
                a.push(rbp)?;
                a.push(rsi)?;
                a.push(rdi)?;
                a.push(r12)?;
                a.push(r13)?;
                a.push(r14)?;
                a.push(r15)?;
                a.sub(rsp, 0x48)?;
                a.mov(rbx, rcx)?;
                a.lea(r12, ptr(rbx + 0x280))?;
                a.mov(rax, t_memset)?;
                a.mov(qword_ptr(r12), rax)?;
                a.mov(rbp, 0x1111_2222_3333_4444u64)?;
                a.mov(rsi, 0x5555_6666_7777_8888u64)?;
                a.mov(rdi, 0x9999_AAAA_BBBB_CCCCu64)?;
                a.mov(r13, 0xDEAD_BEEF_CAFE_BABEu64)?;
                a.mov(r14, 0x0123_4567_89AB_CDEFu64)?;
                a.mov(r15d, 4u32)?;
                a.set_label(&mut loop_lbl)?;
                // Tail `jmp rax` -> memcpy(rbx + 0x100, rbx, 0x40)
                a.lea(rcx, ptr(rbx + 0x100))?;
                a.mov(rdx, rbx)?;
                a.mov(r8d, 0x40u32)?;
                a.call(tail_reg)?;
                a.mov(qword_ptr(rbx + 0x200), rax)?;
                // Tail `jmp [r12]` -> memset(rbx + 0x180, 0x5A, 0x40)
                a.lea(rcx, ptr(rbx + 0x180))?;
                a.mov(edx, 0x5Au32)?;
                a.mov(r8d, 0x40u32)?;
                a.call(tail_mem)?;
                a.mov(qword_ptr(rbx + 0x208), rax)?;
                a.dec(r15d)?;
                a.jne(loop_lbl)?;
                a.mov(qword_ptr(rbx + 0x300), rbp)?;
                a.mov(qword_ptr(rbx + 0x308), rsi)?;
                a.mov(qword_ptr(rbx + 0x310), rdi)?;
                a.mov(qword_ptr(rbx + 0x318), r13)?;
                a.mov(qword_ptr(rbx + 0x320), r14)?;
                a.mov(rax, qword_ptr(rbx + 0x200))?;
                a.xor(rax, qword_ptr(rbx + 0x208))?;
                a.add(rsp, 0x48)?;
                a.pop(r15)?;
                a.pop(r14)?;
                a.pop(r13)?;
                a.pop(r12)?;
                a.pop(rdi)?;
                a.pop(rsi)?;
                a.pop(rbp)?;
                a.pop(rbx)?;
                a.ret()?;
                a.set_label(&mut tail_reg)?;
                a.mov(rax, t_memcpy)?;
                a.jmp(rax)?;
                a.set_label(&mut tail_mem)?;
                a.jmp(qword_ptr(r12))?;
                a.assemble(c as u64)
            })()
            .expect("assemble");

            let d = arena_alloc(DATA, 64);
            let mut init = vec![0u8; DATA];
            for (i, b) in init.iter_mut().take(0x40).enumerate() {
                *b = (0xA0 + i) as u8;
            }
            let plain = jit::plaincall_enabled();
            unsafe {
                core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
                let interp = run_mode(c, d, &init, 2);

                let x1_0 = super::test_xfer_plain();
                let s1_0 = jit::test_stub_plain_hits();
                let jit1 = run_mode(c, d, &init, 1);
                let x1 = super::test_xfer_plain() - x1_0;
                let s1 = jit::test_stub_plain_hits() - s1_0;

                let x2_0 = super::test_xfer_plain();
                let s2_0 = jit::test_stub_plain_hits();
                let r2_0 = jit::test_ras_moves();
                let jit2 = run_mode(c, d, &init, 1);
                let x2 = super::test_xfer_plain() - x2_0;
                let s2 = jit::test_stub_plain_hits() - s2_0;
                let r2 = jit::test_ras_moves() - r2_0;

                let slot_memcpy = TLS_EMU.with(|cell| {
                    (*cell.get()).as_ref().and_then(|st| st.jit.test_slot_entry(t_memcpy))
                });
                let slot_memset = TLS_EMU.with(|cell| {
                    (*cell.get()).as_ref().and_then(|st| st.jit.test_slot_entry(t_memset))
                });

                crate::exe::unregister_native_va(t_memcpy);
                crate::exe::unregister_native_va(t_memset);

                assert_eq!(jit1, interp, "run 1 parity mismatch");
                assert_eq!(jit2, interp, "run 2 parity mismatch");
                assert_eq!(&jit2.1[0x100..0x140], &init[0x00..0x40], "memcpy result");
                assert_eq!(&jit2.1[0x180..0x1C0], &[0x5Au8; 0x40], "memset result");
                if plain {
                    assert_eq!(x1, 8, "all 8 tail calls served via xfer_plain/is_plain_jmp");
                    assert_eq!(s1, 6, "iterations 2..4 must hit lookup_jmp is_plain_jmp");
                    assert_eq!(x2, 8);
                    assert_eq!(s2, 8, "all 8 tail calls in run 2 must hit lookup_jmp is_plain_jmp");
                    assert_eq!(r2, 0, "is_plain_jmp RAS pops must stay balanced with caller pushes");
                    assert_eq!(slot_memcpy, Some((t_memcpy, (f_memcpy as u64) | 1, 0, u32::MAX, 0)));
                    assert_eq!(slot_memset, Some((t_memset, (f_memset as u64) | 1, 0, u32::MAX, 0)));
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Indirect `call reg` and indirect tail `jmp reg` to `pure_float_math_fns` builtins must
/// cache with bit 1 = 1 (`fast = host_fn | 3`), hit `is_plain` / `is_plain_jmp` with XMM0..3
/// arg/return marshalling, preserve non-arg pinned XMM registers, and match IEEE-754 parity.
#[test]
fn lookup_call_and_jmp_pure_float_math_fast_path() {
    std::thread::Builder::new()
        .name("lookup_float_math".into())
        .spawn(|| {
            let exp = |dll: &str, name: &str| {
                win32_rt::lookup(dll, name).unwrap_or_else(|| panic!("{dll}!{name}")) as usize
            };
            let f_fmaxf = exp("ucrtbase.dll", "fmaxf");
            let f_tanf = exp("ucrtbase.dll", "tanf");
            let f_ceil = exp("ucrtbase.dll", "ceil");
            let f_isnan = exp("ucrtbase.dll", "_isnan");

            let h = arena_alloc(64, 16);
            unsafe { core::ptr::write_bytes(h, 0xCC, 64) };
            let (t_fmaxf, t_tanf, t_ceil, t_isnan) = (h as u64, h as u64 + 16, h as u64 + 32, h as u64 + 48);
            let natives = [
                (t_fmaxf, f_fmaxf),
                (t_tanf, f_tanf),
                (t_ceil, f_ceil),
                (t_isnan, f_isnan),
            ];
            for &(va, f) in &natives {
                crate::exe::register_native_va(va, f);
            }

            let c = arena_alloc(0x400, 16);
            let code = (|| -> Result<Vec<u8>, IcedError> {
                let mut a = CodeAssembler::new(64)?;
                let mut tail_tanf = a.create_label();
                let mut tail_fmaxf = a.create_label();
                let mut tail_isnan = a.create_label();
                a.push(rbx)?;
                a.sub(rsp, 0x40)?;
                a.mov(rbx, rcx)?;
                // Seed non-arg XMM sentinels that must survive spill/reload.
                a.movdqu(xmm6, xmmword_ptr(rbx))?;
                a.movdqu(xmm15, xmmword_ptr(rbx + 0x10))?;
                // 1. Indirect call: fmaxf(-3.5f32, 7.25f32) == 7.25f32
                a.mov(eax, (-3.5f32).to_bits())?;
                a.movd(xmm0, eax)?;
                a.mov(eax, 7.25f32.to_bits())?;
                a.movd(xmm1, eax)?;
                a.mov(rax, t_fmaxf)?;
                a.call(rax)?;
                a.movd(eax, xmm0)?;
                a.mov(dword_ptr(rbx + 0x100), eax)?;
                // 2. Indirect tail jmp: tanf(0.75f32)
                a.mov(eax, 0.75f32.to_bits())?;
                a.movd(xmm0, eax)?;
                a.call(tail_tanf)?;
                a.movd(eax, xmm0)?;
                a.mov(dword_ptr(rbx + 0x104), eax)?;
                // 3. Indirect call: ceil(-2.25f64) == -2.0f64
                a.mov(rax, (-2.25f64).to_bits())?;
                a.movq(xmm0, rax)?;
                a.mov(rax, t_ceil)?;
                a.call(rax)?;
                a.movq(qword_ptr(rbx + 0x108), xmm0)?;
                // 4. Indirect tail jmp: fmaxf(12.5f32, -9.0f32) == 12.5f32
                a.mov(eax, 12.5f32.to_bits())?;
                a.movd(xmm0, eax)?;
                a.mov(eax, (-9.0f32).to_bits())?;
                a.movd(xmm1, eax)?;
                a.call(tail_fmaxf)?;
                a.movd(eax, xmm0)?;
                a.mov(dword_ptr(rbx + 0x110), eax)?;
                // 5. Indirect call: _isnan(qNaN) != 0 (integer return in eax)
                a.mov(rax, f64::NAN.to_bits())?;
                a.movq(xmm0, rax)?;
                a.mov(rax, t_isnan)?;
                a.call(rax)?;
                a.mov(dword_ptr(rbx + 0x114), eax)?;
                // 6. Indirect tail jmp: _isnan(1.5f64) == 0
                a.mov(rax, 1.5f64.to_bits())?;
                a.movq(xmm0, rax)?;
                a.call(tail_isnan)?;
                a.mov(dword_ptr(rbx + 0x118), eax)?;
                // Store non-arg XMM sentinels.
                a.movdqu(xmmword_ptr(rbx + 0x120), xmm6)?;
                a.movdqu(xmmword_ptr(rbx + 0x130), xmm15)?;
                a.xor(eax, eax)?;
                a.add(rsp, 0x40)?;
                a.pop(rbx)?;
                a.ret()?;
                a.set_label(&mut tail_tanf)?;
                a.mov(rax, t_tanf)?;
                a.jmp(rax)?;
                a.set_label(&mut tail_fmaxf)?;
                a.mov(rax, t_fmaxf)?;
                a.jmp(rax)?;
                a.set_label(&mut tail_isnan)?;
                a.mov(rax, t_isnan)?;
                a.jmp(rax)?;
                a.assemble(c as u64)
            })()
            .expect("assemble");

            let d = arena_alloc(DATA, 64);
            let mut init = vec![0u8; DATA];
            for (i, b) in init.iter_mut().take(0x20).enumerate() {
                *b = (0x31 + i * 3) as u8;
            }
            let plain = jit::plaincall_enabled();
            unsafe {
                core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
                let interp = run_mode(c, d, &init, 2);
                let jit1 = run_mode(c, d, &init, 1);

                let x2_0 = super::test_xfer_plain();
                let s2_0 = jit::test_stub_plain_hits();
                let r2_0 = jit::test_ras_moves();
                let jit2 = run_mode(c, d, &init, 1);
                let x2 = super::test_xfer_plain() - x2_0;
                let s2 = jit::test_stub_plain_hits() - s2_0;
                let r2 = jit::test_ras_moves() - r2_0;

                let slots: Vec<_> = natives
                    .iter()
                    .map(|&(va, _)| {
                        TLS_EMU.with(|cell| {
                            (*cell.get()).as_ref().and_then(|st| st.jit.test_slot_entry(va))
                        })
                    })
                    .collect();

                for &(va, _) in &natives {
                    crate::exe::unregister_native_va(va);
                }

                assert_eq!(jit1, interp, "float math run 1 parity mismatch");
                assert_eq!(jit2, interp, "float math run 2 parity mismatch");
                assert_eq!(&jit2.1[0x100..0x104], &7.25f32.to_bits().to_le_bytes());
                assert_eq!(&jit2.1[0x108..0x110], &(-2.0f64).to_bits().to_le_bytes());
                assert_eq!(&jit2.1[0x110..0x114], &12.5f32.to_bits().to_le_bytes());
                assert_ne!(u32::from_le_bytes(jit2.1[0x114..0x118].try_into().unwrap()), 0);
                assert_eq!(u32::from_le_bytes(jit2.1[0x118..0x11C].try_into().unwrap()), 0);
                assert_eq!(&jit2.1[0x120..0x140], &init[0x00..0x20], "xmm6/xmm15 preserved");

                if plain {
                    assert_eq!(x2, 6);
                    assert_eq!(s2, 6, "all 6 float math calls/jmps must hit lookup stub fast path");
                    assert_eq!(r2, 0, "float math tail jumps must return cleanly via RAS");
                    for (&(va, f), slot) in natives.iter().zip(slots.iter()) {
                        assert_eq!(
                            *slot,
                            Some((va, (f as u64) | 3, 0, u32::MAX, 0)),
                            "expected bit-1 tagged float math slot for {va:#x}"
                        );
                    }
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Indirect `call reg` and indirect tail `jmp reg` to `QueryPerformanceCounter` and
/// `RtlQueryPerformanceCounter` must cache bit-63-tagged `qpc_fast_state_ptr()`, hit the
/// zero-spill `is_qpc` / `is_qpc_jmp` fast path when `mode == 1`, and fall back cleanly to
/// `call_slow` / `jmp_slow` when `mode != 1` without clobbering any GPR or XMM state.
#[test]
fn lookup_call_and_jmp_qpc_zero_spill_fast_path_and_fallback() {
    use core::sync::atomic::{AtomicU64, Ordering};
    use win32_rt::kernel32::time::{qpc_fast_state_ptr, QpcFastState};

    std::thread::Builder::new()
        .name("lookup_qpc_fast".into())
        .spawn(|| {
            let exp = |dll: &str, name: &str| {
                win32_rt::lookup(dll, name).unwrap_or_else(|| panic!("{dll}!{name}")) as usize
            };
            let f_qpc = exp("kernel32.dll", "QueryPerformanceCounter");
            let f_rtl_qpc = exp("ntdll.dll", "RtlQueryPerformanceCounter");

            let h = arena_alloc(64, 16);
            unsafe { core::ptr::write_bytes(h, 0xCC, 64) };
            let (t_qpc, t_rtl_qpc) = (h as u64, h as u64 + 16);
            crate::exe::register_native_va(t_qpc, f_qpc);
            crate::exe::register_native_va(t_rtl_qpc, f_rtl_qpc);

            // Guest routine: fn(rcx = buf, rdx = target_va, r8d = is_tail) -> BOOL/BOOLEAN
            // Writes counter to [buf+0x00], copies xmm6/xmm15 sentinels from [buf+0x10..0x30]
            // to [buf+0x30..0x50], and saves callee-saved GPR sentinels to [buf+0x50..0x88].
            let c = arena_alloc(0x400, 16);
            let code = (|| -> Result<Vec<u8>, IcedError> {
                let mut a = CodeAssembler::new(64)?;
                let mut do_tail = a.create_label();
                let mut done = a.create_label();
                let mut tail_qpc = a.create_label();
                a.push(rbx)?;
                a.push(rbp)?;
                a.push(rsi)?;
                a.push(rdi)?;
                a.push(r12)?;
                a.push(r13)?;
                a.push(r14)?;
                a.push(r15)?;
                a.sub(rsp, 0x48)?;
                a.mov(rbx, rcx)?;
                a.mov(rax, rdx)?;
                a.mov(rbp, 0x1010_2020_3030_4040u64)?;
                a.mov(rsi, 0x5050_6060_7070_8080u64)?;
                a.mov(rdi, 0x9090_A0A0_B0B0_C0C0u64)?;
                a.mov(r12, 0xD0D0_E0E0_F0F0_0101u64)?;
                a.mov(r13, 0x2233_4455_6677_8899u64)?;
                a.mov(r14, 0xAABB_CCDD_EEFF_0011u64)?;
                a.mov(r15, 0xFACE_B00C_DEAD_BEEFu64)?;
                a.movdqu(xmm6, xmmword_ptr(rbx + 0x10))?;
                a.movdqu(xmm15, xmmword_ptr(rbx + 0x20))?;
                a.test(r8d, r8d)?;
                a.jne(do_tail)?;
                a.call(rax)?;
                a.jmp(done)?;
                a.set_label(&mut do_tail)?;
                a.call(tail_qpc)?;
                a.set_label(&mut done)?;
                a.movdqu(xmmword_ptr(rbx + 0x30), xmm6)?;
                a.movdqu(xmmword_ptr(rbx + 0x40), xmm15)?;
                a.mov(qword_ptr(rbx + 0x50), rbp)?;
                a.mov(qword_ptr(rbx + 0x58), rsi)?;
                a.mov(qword_ptr(rbx + 0x60), rdi)?;
                a.mov(qword_ptr(rbx + 0x68), r12)?;
                a.mov(qword_ptr(rbx + 0x70), r13)?;
                a.mov(qword_ptr(rbx + 0x78), r14)?;
                a.mov(qword_ptr(rbx + 0x80), r15)?;
                a.add(rsp, 0x48)?;
                a.pop(r15)?;
                a.pop(r14)?;
                a.pop(r13)?;
                a.pop(r12)?;
                a.pop(rdi)?;
                a.pop(rsi)?;
                a.pop(rbp)?;
                a.pop(rbx)?;
                a.ret()?;
                a.set_label(&mut tail_qpc)?;
                a.jmp(rax)?;
                a.assemble(c as u64)
            })()
            .expect("assemble");

            let buf = arena_alloc(0x100, 16);
            let sentinels_gpr: [u64; 7] = [
                0x1010_2020_3030_4040,
                0x5050_6060_7070_8080,
                0x9090_A0A0_B0B0_C0C0,
                0xD0D0_E0E0_F0F0_0101,
                0x2233_4455_6677_8899,
                0xAABB_CCDD_EEFF_0011,
                0xFACE_B00C_DEAD_BEEF,
            ];
            let plain = jit::plaincall_enabled();

            unsafe {
                core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
                let run_qpc = |target: u64, is_tail: u64| -> (u64, i64) {
                    core::ptr::write_bytes(buf, 0, 0x100);
                    for i in 0..0x20usize {
                        *buf.add(0x10 + i) = (0x41 + i) as u8;
                    }
                    jit::force(1);
                    let (ret, _) = emu_run_full(
                        c as u64,
                        [buf as u64, target, is_tail, 0, 0, 0, 0, 0],
                        [0; 4],
                        [0; 8],
                    );
                    jit::force(0);
                    assert_eq!(
                        core::slice::from_raw_parts(buf.add(0x30), 0x20),
                        core::slice::from_raw_parts(buf.add(0x10), 0x20),
                        "xmm6/xmm15 clobbered (tail={is_tail})"
                    );
                    for (i, &want) in sentinels_gpr.iter().enumerate() {
                        let got = core::ptr::read_unaligned(buf.add(0x50 + 8 * i) as *const u64);
                        assert_eq!(got, want, "callee-saved GPR #{i} clobbered (tail={is_tail})");
                    }
                    let ts = core::ptr::read_unaligned(buf as *const i64);
                    (ret, ts)
                };

                // Warm both QPC targets and the tail-jump block (call #1 caches bit-63-tagged
                // qpc_fast_state_ptr and links `do_tail -> tail_qpc`).
                let (r0, t0) = run_qpc(t_qpc, 0);
                let (r1, t1) = run_qpc(t_rtl_qpc, 0);
                let (rp, tp) = run_qpc(t_qpc, 1);
                assert_eq!((r0, r1, rp), (1, 1, 1));
                assert!(t0 > 0 && t1 >= t0 && tp >= t1, "monotonic real QPC: t0={t0}, t1={t1}, tp={tp}");

                let qpc_ptr = qpc_fast_state_ptr();
                let live_mode = (*(qpc_ptr as *const QpcFastState)).mode.load(Ordering::Acquire);
                let want_fast = (qpc_ptr as u64) | (1u64 << 63) | 1;
                let table = TLS_EMU.with(|cell| {
                    (*cell.get()).as_ref().map_or(0, |st| st.jit.test_table_ptr())
                });
                assert_ne!(table, 0);

                if plain {
                    for va in [t_qpc, t_rtl_qpc] {
                        let ent = TLS_EMU.with(|cell| {
                            (*cell.get()).as_ref().and_then(|st| st.jit.test_slot_entry(va))
                        });
                        assert_eq!(ent, Some((va, want_fast, 0, u32::MAX, 0)));
                    }

                    let s0 = jit::test_stub_plain_hits();
                    let rm0 = jit::test_ras_moves();
                    let (rc1, tc1) = run_qpc(t_qpc, 0);
                    let (rj1, tj1) = run_qpc(t_qpc, 1);
                    let (rc2, tc2) = run_qpc(t_rtl_qpc, 0);
                    let (rj2, tj2) = run_qpc(t_rtl_qpc, 1);
                    assert_eq!((rc1, rj1, rc2, rj2), (1, 1, 1, 1));
                    assert!(tc1 >= tp && tj1 >= tc1 && tc2 >= tj1 && tj2 >= tc2);
                    if live_mode == 1 {
                        assert_eq!(jit::test_stub_plain_hits() - s0, 4, "all 4 QPC calls/jmps hit is_qpc");
                        assert_eq!(jit::test_ras_moves() - rm0, 0, "is_qpc_jmp RAS pops stay balanced");
                    }
                }

                // Deterministic local QpcFastState test (never mutates global QPC_STATE):
                // 1) mode == 1, mult == 0, fixed offset -> proves inline ARM64 path executes directly.
                let local_fast = QpcFastState {
                    mode: AtomicU64::new(1),
                    mult: AtomicU64::new(0),
                    offset: AtomicU64::new(0x0123_4567_89AB_CDEF),
                };
                jit::JitCache::cache_qpc_host(table, t_qpc, &local_fast as *const QpcFastState as usize);
                let s_fast0 = jit::test_stub_plain_hits();
                let rm_fast0 = jit::test_ras_moves();
                let (rf_call, tf_call) = run_qpc(t_qpc, 0);
                let (rf_jmp, tf_jmp) = run_qpc(t_qpc, 1);
                assert_eq!((rf_call, rf_jmp), (1, 1));
                assert_eq!(tf_call, 0x0123_4567_89AB_CDEF);
                assert_eq!(tf_jmp, 0x0123_4567_89AB_CDEF);
                assert_eq!(jit::test_stub_plain_hits() - s_fast0, 2);
                assert_eq!(jit::test_ras_moves() - rm_fast0, 0);

                // 2) mode == 2 -> proves both `is_qpc` and `is_qpc_jmp` fall back cleanly to
                //    `call_slow` / `jmp_slow` -> `host_xfer` -> real `QueryPerformanceCounter`.
                let local_slow = QpcFastState {
                    mode: AtomicU64::new(2),
                    mult: AtomicU64::new(0),
                    offset: AtomicU64::new(0xDEAD_BEEF_1234_5678),
                };
                for is_tail in [0u64, 1u64] {
                    jit::JitCache::cache_qpc_host(table, t_qpc, &local_slow as *const QpcFastState as usize);
                    let s_slow0 = jit::test_stub_plain_hits();
                    let x_slow0 = super::test_xfer_plain();
                    let (rs, ts) = run_qpc(t_qpc, is_tail);
                    assert_eq!(rs, 1, "QPC fallback must return TRUE (tail={is_tail})");
                    assert_ne!(ts as u64, 0xDEAD_BEEF_1234_5678, "fallback must not read local offset");
                    assert!(ts >= t0, "fallback must return real monotonic ticks (tail={is_tail})");
                    assert_eq!(
                        jit::test_stub_plain_hits() - s_slow0,
                        0,
                        "mode != 1 must bail to slow path (tail={is_tail})"
                    );
                    if plain {
                        assert_eq!(
                            super::test_xfer_plain() - x_slow0,
                            1,
                            "mode != 1 fallback must execute via xfer_plain (tail={is_tail})"
                        );
                    }
                }

                crate::exe::unregister_native_va(t_qpc);
                crate::exe::unregister_native_va(t_rtl_qpc);
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// `cache_plain_host` and `cache_qpc_host` must unconditionally overwrite hash slots previously
/// occupied by guest blocks (`code != 0`, same or colliding `ip`), set `code = 0, hits = NOJIT`,
/// and reject unaligned / bit-63-set addresses or null tables defensively.
#[test]
fn cache_plain_host_overwrites_colliding_guest_slot() {
    // Modular inverse of `jit::JMUL` mod 2^64 so `target.wrapping_add(JMUL_INV)` hashes to the
    // exact same `jslot` whenever the low 46 bits of `target * JMUL` don't carry into bit 46.
    const JMUL_INV: u64 = 0xF1DE_83E1_9937_733D;
    assert_eq!(jit::JMUL.wrapping_mul(JMUL_INV), 1);

    let target = 0x7000_1234_0000u64;
    let colliding_guest = target.wrapping_add(JMUL_INV);
    assert_ne!(colliding_guest, target);

    let mut jc = jit::JitCache::new();
    jc.test_seed_guest_slot(colliding_guest, 0x2_0000_1000);
    // Confirm the seeded guest entry actually lands in `target`'s direct-mapped slot.
    assert_eq!(
        jc.test_slot_entry(target),
        Some((colliding_guest, 0x2_0000_1000, 0, 0, 0x2_0000_1000))
    );
    let table = jc.test_table_ptr();
    assert_ne!(table, 0);

    unsafe {
        // Pure-integer DF_PLAIN overwrites colliding guest block.
        jit::JitCache::cache_plain_host(table, target, 0x1_0000_4000, false);
        assert_eq!(
            jc.test_slot_entry(target),
            Some((target, 0x1_0000_4001, 0, u32::MAX, 0))
        );

        // Pure-float-math overwrites colliding guest block with bit 1 = 1.
        jc.test_seed_guest_slot(colliding_guest, 0x2_0000_2000);
        jit::JitCache::cache_plain_host(table, target, 0x1_0000_8000, true);
        assert_eq!(
            jc.test_slot_entry(target),
            Some((target, 0x1_0000_8003, 0, u32::MAX, 0))
        );

        // QPC fast-state entry overwrites colliding guest block with bit 63 | bit 0.
        jc.test_seed_guest_slot(colliding_guest, 0x2_0000_3000);
        jit::JitCache::cache_qpc_host(table, target, 0x1_0000_9000);
        assert_eq!(
            jc.test_slot_entry(target),
            Some((target, (1u64 << 63) | 0x1_0000_9001, 0, u32::MAX, 0))
        );

        // Defensive guards: misaligned, bit-63-set, and null table leave the slot untouched.
        jit::JitCache::cache_plain_host(0, target, 0x1_0000_A000, false);
        jit::JitCache::cache_plain_host(table, target, 0x1_0000_A001, false);
        jit::JitCache::cache_plain_host(table, target, 0x1_0000_A002, true);
        jit::JitCache::cache_plain_host(table, target, ((1u64 << 63) | 0x1_0000_A000) as usize, false);
        jit::JitCache::cache_qpc_host(table, target, 0x1_0000_B001);
        jit::JitCache::cache_qpc_host(table, target, ((1u64 << 63) | 0x1_0000_B000) as usize);
        assert_eq!(
            jc.test_slot_entry(target),
            Some((target, (1u64 << 63) | 0x1_0000_9001, 0, u32::MAX, 0))
        );
    }
}

/// Static Mach-O stubs + GOT veneer must match live stub offsets and fit comfortably inside
/// `AOT_STUBS_BYTES` (4096 bytes) with >1 KiB headroom even under worst-case knob combinations.
#[test]
fn static_stubs_with_veneer_fit_with_headroom() {
    const VBASE: usize = 0x2000_0000_0000;
    const MIN_HEADROOM_BYTES: usize = 1024;

    unsafe {
        // 1. Live default knobs via `aot_emit_stubs` + `emit_static_stubs_with_veneer`.
        let knobs = jit::aot_knob_bits();
        let mut live_buf = vec![0u32; jit::AOT_STUBS_BYTES / 4];
        let want = jit::aot_emit_stubs(live_buf.as_mut_ptr(), VBASE).expect("live stubs");
        let mut veneer_buf = vec![0u32; jit::AOT_STUBS_BYTES / 4];
        let (got, veneer_off) =
            jit::emit_static_stubs_with_veneer(veneer_buf.as_mut_ptr(), VBASE, knobs, &want)
                .expect("static stubs with veneer");
        assert_eq!(got, want);
        if let Some(vo) = veneer_off {
            assert_eq!(vo % 16, 0);
            assert!(
                vo + 16 + MIN_HEADROOM_BYTES <= jit::AOT_STUBS_BYTES,
                "live default stubs + veneer used {} / {} bytes",
                vo + 16,
                jit::AOT_STUBS_BYTES
            );
        }

        // 2. Worst-case and boundary knob combinations (including pushpop=0, xpin=1, lb_on=1,
        //    bud=1, bud_light=1, aot_on=1, ras=1, hx=1, count_probe=1, count_plain=1).
        let worst_knobs = (1u64 << 0) // ras
            | (1u64 << 9)  // bud
            | (1u64 << 13) // hx
            | (1u64 << 15) // tso
            | (1u64 << 16) // stack_tso
            | (1u64 << 27) // aot_on
            | (1u64 << 30) // xpin
            | (1u64 << 32) // lb_on
            | (1u64 << 34); // bud_light (pushpop bit 8 = 0 -> longest lookup_call!)
        let configs = [
            (worst_knobs, true, true),
            (worst_knobs, false, false),
            (worst_knobs & !(1u64 << 0), true, true),  // !ras
            (worst_knobs & !(1u64 << 30), true, true), // !xpin
            (worst_knobs & !(1u64 << 13), true, true), // !hx
            (worst_knobs & !(1u64 << 9), true, true),  // !bud
        ];
        for (k, cp, cpl) in configs {
            let (live_words, end_bytes) =
                jit::test_emit_stubs_and_veneer_for_knobs(k, cp, cpl).expect("stub emit");
            assert!(
                end_bytes + MIN_HEADROOM_BYTES <= jit::AOT_STUBS_BYTES,
                "knobs={k:#x} (probe={cp}, plain={cpl}): live={live_words} words, end={end_bytes} / {} bytes",
                jit::AOT_STUBS_BYTES
            );
        }
    }
}

/// Regression test for `emit_cvtpd2dq_lane` (`cvttpd2dq`/`vcvttpd2dq`/`cvtpd2dq`/`vcvtpd2dq`):
/// previously `vd == vn` with `trunc == true` let `v_fcvtzs_2d` overwrite `src` before the
/// `[-2^31, 2^31)` range check, corrupting every negative input `<= -1.0` to `0x8000_0000`
/// (and positive out-of-range inputs `[2^31, 2^63)` to low-32 wrapped garbage).
#[test]
fn cvttpd2dq_and_cvtpd2dq_all_ranges() {
    let inputs: [(f64, f64); 10] = [
        (0.0, -0.0),
        (0.5, -0.5),
        (1.0, -1.0),
        (3.14159, -3.14159),
        (2147483647.9, -2147483648.0),
        (2147483648.0, -2147483648.9),
        (3_000_000_000.0, -2147483649.0),
        (1e20, -1e20),
        (f64::INFINITY, f64::NEG_INFINITY),
        (f64::NAN, -f64::NAN),
    ];

    let mut a = CodeAssembler::new(64).unwrap();
    // rcx = data buffer: [0x00..0x100) holds inputs; write outputs starting at 0x100.
    // Test: (1) cvttpd2dq xmm0, xmm0 (in-place), (2) cvttpd2dq xmm1, xmm2 (two-reg),
    //       (3) cvtpd2dq xmm3, xmm3 (in-place), (4) vcvttpd2dq xmm4, ymm5 (256-bit),
    //       (5) vcvtpd2dq xmm6, ymm5 (256-bit), (6) cvttpd2dq xmm7, [rcx+0] (mem).
    a.movupd(xmm0, xmmword_ptr(rcx)).unwrap();
    a.cvttpd2dq(xmm0, xmm0).unwrap();
    a.movdqu(xmmword_ptr(rcx + 0x100), xmm0).unwrap();

    a.movupd(xmm2, xmmword_ptr(rcx)).unwrap();
    a.cvttpd2dq(xmm1, xmm2).unwrap();
    a.movdqu(xmmword_ptr(rcx + 0x110), xmm1).unwrap();

    a.movupd(xmm3, xmmword_ptr(rcx)).unwrap();
    a.cvtpd2dq(xmm3, xmm3).unwrap();
    a.movdqu(xmmword_ptr(rcx + 0x120), xmm3).unwrap();

    a.vmovupd(ymm5, ymmword_ptr(rcx)).unwrap();
    a.vcvttpd2dq(xmm4, ymm5).unwrap();
    a.movdqu(xmmword_ptr(rcx + 0x130), xmm4).unwrap();

    a.vcvtpd2dq(xmm6, ymm5).unwrap();
    a.movdqu(xmmword_ptr(rcx + 0x140), xmm6).unwrap();

    a.cvttpd2dq(xmm7, xmmword_ptr(rcx)).unwrap();
    a.movdqu(xmmword_ptr(rcx + 0x150), xmm7).unwrap();

    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let c = arena_alloc(code.len() + 16, 16);
    let d = arena_alloc(DATA, 64);
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        core::ptr::write_bytes(c.add(code.len()), 0xCC, 16);
    }

    for (idx, &(a0, a1)) in inputs.iter().enumerate() {
        let (b0, b1) = inputs[(idx + 3) % inputs.len()];
        let mut init = vec![0u8; DATA];
        init[0..8].copy_from_slice(&a0.to_le_bytes());
        init[8..16].copy_from_slice(&a1.to_le_bytes());
        init[16..24].copy_from_slice(&b0.to_le_bytes());
        init[24..32].copy_from_slice(&b1.to_le_bytes());

        let interp = unsafe { run_mode(c, d, &init, 2) };
        let jit1 = unsafe { run_mode(c, d, &init, 1) };
        let jit2 = unsafe { run_mode(c, d, &init, 1) };
        assert_eq!(
            jit1.1[0x100..0x160],
            interp.1[0x100..0x160],
            "case {idx} ({a0}, {a1}, {b0}, {b1}): JIT first run diverged from interpreter"
        );
        assert_eq!(
            jit2.1[0x100..0x160],
            interp.1[0x100..0x160],
            "case {idx} ({a0}, {a1}, {b0}, {b1}): JIT linked run diverged from interpreter"
        );
    }
    // Sanity-check exact expected truncated values for (-3.14159, 3_000_000_000.0):
    let mut init = vec![0u8; DATA];
    init[0..8].copy_from_slice(&(-3.14159f64).to_le_bytes());
    init[8..16].copy_from_slice(&(3_000_000_000.0f64).to_le_bytes());
    let jit_out = unsafe { run_mode(c, d, &init, 1) };
    let lane0 = i32::from_le_bytes(jit_out.1[0x100..0x104].try_into().unwrap());
    let lane1 = i32::from_le_bytes(jit_out.1[0x104..0x108].try_into().unwrap());
    let lane2 = u32::from_le_bytes(jit_out.1[0x108..0x10C].try_into().unwrap());
    let lane3 = u32::from_le_bytes(jit_out.1[0x10C..0x110].try_into().unwrap());
    assert_eq!(lane0, -3, "cvttpd2dq(-3.14159) must be -3");
    assert_eq!(lane1, i32::MIN, "cvttpd2dq(+3e9) must saturate to 0x8000_0000");
    assert_eq!((lane2, lane3), (0, 0), "cvttpd2dq upper 64 bits must be zero");
}

/// Regression tests for:
/// 1. `vcvtps2ph` (`f32 -> f16`) subnormal rounding (`1e-6 -> 0x0011`, `2^-25` ties-to-even `0`,
///    `> 2^-25 -> 0x0001`, subnormal-to-normal rollover `0x0400`, overflow `65520.0 -> 0x7C00`),
///    matching ARM64 `FCVTN` and IEEE-754 RNE bit-for-bit across Generic, FastOp, and JIT.
/// 2. `loope` / `loopne` in the Generic interpreter (`emu_run_region` previously let the `Jcc`
///    `FlowControl::ConditionalBranch` check hijack `Loope`/`Loopne` without decrementing `rcx`).
#[test]
fn vcvtps2ph_subnormals_and_loopcc_parity() {
    // Part 1: vcvtps2ph xmm0, ymm1, 0
    let f32_cases: [u32; 8] = [
        0x3586_37BD, // 1e-6 -> 16.777 * 2^-24 -> 0x0011
        0x3300_0000, // 2^-25 (exact tie between 0 and 2^-24) -> 0x0000
        0x3300_0001, // strictly > 2^-25 -> 0x0001
        0x387F_FFFF, // largest f32 < 2^-14 -> rounds up to smallest normal 0x0400
        0x477F_E000, // 65504.0 (max finite f16) -> 0x7BFF
        0x477F_F000, // 65520.0 (tie to Inf) -> 0x7C00
        0xBF80_0000, // -1.0 -> 0xBC00
        0x7FC0_0000, // +qNaN -> 0x7E00
    ];
    let expected_f16: [u16; 8] = [0x0011, 0x0000, 0x0001, 0x0400, 0x7BFF, 0x7C00, 0xBC00, 0x7E00];

    let mut a = CodeAssembler::new(64).unwrap();
    a.vmovups(ymm1, ymmword_ptr(rcx)).unwrap();
    a.vcvtps2ph(xmm0, ymm1, 0).unwrap();
    a.vmovdqu(xmmword_ptr(rcx + 0x100), xmm0).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let c = arena_alloc(code.len() + 16, 16);
    let d = arena_alloc(DATA, 64);
    let mut init = vec![0u8; DATA];
    for (i, &bits) in f32_cases.iter().enumerate() {
        init[i * 4..i * 4 + 4].copy_from_slice(&bits.to_le_bytes());
    }
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        core::ptr::write_bytes(c.add(code.len()), 0xCC, 16);
        let interp = run_mode(c, d, &init, 2);
        let jit1 = run_mode(c, d, &init, 1);
        assert_eq!(jit1.1[0x100..0x110], interp.1[0x100..0x110], "vcvtps2ph JIT vs interp");
        for (i, &want) in expected_f16.iter().enumerate() {
            let got = u16::from_le_bytes([interp.1[0x100 + i * 2], interp.1[0x100 + i * 2 + 1]]);
            assert_eq!(got, want, "vcvtps2ph lane {i}: got {got:#06x} want {want:#06x}");
        }
    }

    // Part 2: loopne / loope must decrement rcx identically in Generic, FastOp, and JIT.
    let mut b = CodeAssembler::new(64).unwrap();
    let mut l_ne = b.create_label();
    let mut l_eq = b.create_label();
    b.mov(rax, rcx).unwrap(); // save data ptr
    b.mov(rcx, 5u64).unwrap();
    b.xor(edx, edx).unwrap();
    b.set_label(&mut l_ne).unwrap();
    b.inc(edx).unwrap();
    b.cmp(edx, 3).unwrap(); // ZF=0 for edx=1,2 (loopne takes), ZF=1 for edx=3 (exits with rcx=2)
    b.loopne(l_ne).unwrap();
    b.mov(qword_ptr(rax + 0x100), rcx).unwrap();
    b.mov(dword_ptr(rax + 0x108), edx).unwrap();

    b.mov(rcx, 5u64).unwrap();
    b.xor(edx, edx).unwrap();
    b.set_label(&mut l_eq).unwrap();
    b.inc(edx).unwrap();
    b.cmp(edx, 1).unwrap(); // ZF=1 for edx=1 (loope takes), ZF=0 for edx=2 (exits with rcx=3)
    b.loope(l_eq).unwrap();
    b.mov(qword_ptr(rax + 0x110), rcx).unwrap();
    b.mov(dword_ptr(rax + 0x118), edx).unwrap();
    b.ret().unwrap();
    let code_b = b.assemble(0).unwrap();

    let cb = arena_alloc(code_b.len() + 16, 16);
    let db = arena_alloc(DATA, 64);
    let init_b = vec![0u8; DATA];
    unsafe {
        core::ptr::copy_nonoverlapping(code_b.as_ptr(), cb, code_b.len());
        core::ptr::write_bytes(cb.add(code_b.len()), 0xCC, 16);
        let interp = run_mode(cb, db, &init_b, 2);
        let jit1 = run_mode(cb, db, &init_b, 1);
        assert_eq!(jit1.1[0x100..0x120], interp.1[0x100..0x120], "loope/loopne JIT vs interp");
        let rcx_ne = u64::from_le_bytes(interp.1[0x100..0x108].try_into().unwrap());
        let edx_ne = u32::from_le_bytes(interp.1[0x108..0x10C].try_into().unwrap());
        let rcx_eq = u64::from_le_bytes(interp.1[0x110..0x118].try_into().unwrap());
        let edx_eq = u32::from_le_bytes(interp.1[0x118..0x11C].try_into().unwrap());
        assert_eq!((rcx_ne, edx_ne), (2, 3), "loopne expected rcx=2 edx=3");
        assert_eq!((rcx_eq, edx_eq), (3, 2), "loope expected rcx=3 edx=2");
    }
}

/// Guest `ldmxcsr`/`vldmxcsr`/`stmxcsr`/`vstmxcsr` must drive ARM64 hardware `FPCR`
/// (`FZ` for FTZ|DAZ and `RMode` for rounding control) identically in interpreter and JIT,
/// while strictly keeping host `FPCR` at IEEE-754 defaults across host calls and after return.
#[test]
fn mxcsr_ftz_daz_rounding_and_host_fpcr_isolation() {
    use iced_x86::code_asm::*;

    static HOST_SAW_FPCR: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(u64::MAX);
    static HOST_SAW_MXCSR: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
    unsafe extern "win64" fn observe_host_fpcr() -> u64 {
        HOST_SAW_FPCR.store(super::read_fpcr() & super::FPCR_GUEST_MASK, core::sync::atomic::Ordering::SeqCst);
        HOST_SAW_MXCSR.store(win32_rt::ucrt::runtime::guest_mxcsr(), core::sync::atomic::Ordering::SeqCst);
        // Modify MXCSR via UCRT `_controlfp` (`_DN_SAVE` = clear FTZ+DAZ -> 0x1F80) and verify
        // guest sees the update upon return.
        win32_rt::ucrt::runtime::_controlfp(0, 0x0300_0000);
        0xCAFE
    }
    const HOST_VA: u64 = 0x7FFF_EE01_1000;
    crate::exe::register_native_va(HOST_VA, observe_host_fpcr as *const () as usize);

    // Data layout at [rax]:
    //   +0x00: f32 2^-68 (0x1D80_0000) -> squared = 2^-136 (subnormal 0x0000_0080)
    //   +0x04: f32 subnormal 2^-136 (0x0000_0080)
    //   +0x08: f32 1.0f (0x3F80_0000)
    //   +0x0C: u32 0x9FC0 (FTZ | DAZ | all masked | RN)
    //   +0x10: u32 0x1F80 (default)
    //   +0x14: u32 0x7F80 (RZ rounding mode, no FTZ/DAZ)
    //   +0x18: f32 1.0f, +0x1C: f32 1e-20f (sum rounds down to 1.0f under RZ, 1.0f+ulp under RP)
    //   +0x20: u32 0x5F80 (RP rounding mode, toward +inf)
    // Outputs at [rax + 0x100..]:
    //   +0x100: initial stmxcsr (0x1F80)
    //   +0x104: (2^-68)^2 under 0x1F80 (subnormal 0x0000_0080)
    //   +0x108: stmxcsr after ldmxcsr 0x9FC0 (0x9FC0)
    //   +0x10C: (2^-68)^2 under 0x9FC0 (flushed to +0.0 = 0x0000_0000)
    //   +0x110: subnormal * 1.0f under 0x9FC0 (DAZ -> +0.0 = 0x0000_0000)
    //   +0x114: vstmxcsr after host _controlfp cleared DN (0x1F80)
    //   +0x118: 1.0f + 1e-20f under RP (0x5F80 -> 0x3F80_0001)
    //   +0x11C: 1.0f + 1e-20f under RZ (0x7F80 -> 0x3F80_0000)
    //   +0x120: (2^-68)^2 after restoring 0x1F80 (subnormal 0x0000_0080 again)
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(rax, rcx).unwrap();
    a.sub(rsp, 0x28).unwrap();
    a.mov(rbx, rax).unwrap();

    // Default MXCSR = 0x1F80
    a.stmxcsr(dword_ptr(rbx + 0x100)).unwrap();
    a.vmovss(xmm0, dword_ptr(rbx + 0x00)).unwrap();
    a.vmulss(xmm1, xmm0, xmm0).unwrap();
    a.vmovss(dword_ptr(rbx + 0x104), xmm1).unwrap();

    // Load 0x9FC0 (FTZ | DAZ)
    a.ldmxcsr(dword_ptr(rbx + 0x0C)).unwrap();
    a.stmxcsr(dword_ptr(rbx + 0x108)).unwrap();
    a.vmovss(xmm0, dword_ptr(rbx + 0x00)).unwrap();
    a.vmulss(xmm1, xmm0, xmm0).unwrap();
    a.vmovss(dword_ptr(rbx + 0x10C), xmm1).unwrap();
    a.vmovss(xmm2, dword_ptr(rbx + 0x04)).unwrap();
    a.vmovss(xmm3, dword_ptr(rbx + 0x08)).unwrap();
    a.vmulss(xmm2, xmm2, xmm3).unwrap();
    a.vmovss(dword_ptr(rbx + 0x110), xmm2).unwrap();

    // Call host fn while MXCSR = 0x9FC0; host verifies FPCR == 0 and clears FTZ/DAZ via _controlfp
    a.mov(r10, HOST_VA).unwrap();
    a.call(r10).unwrap();
    a.vstmxcsr(dword_ptr(rbx + 0x114)).unwrap();

    // Test RP (+inf) vs RZ (toward zero) rounding control via vldmxcsr
    a.vldmxcsr(dword_ptr(rbx + 0x20)).unwrap(); // 0x5F80 = RP
    a.vmovss(xmm4, dword_ptr(rbx + 0x18)).unwrap();
    a.vmovss(xmm5, dword_ptr(rbx + 0x1C)).unwrap();
    a.vaddss(xmm6, xmm4, xmm5).unwrap();
    a.vmovss(dword_ptr(rbx + 0x118), xmm6).unwrap();

    a.vldmxcsr(dword_ptr(rbx + 0x14)).unwrap(); // 0x7F80 = RZ
    a.vaddss(xmm6, xmm4, xmm5).unwrap();
    a.vmovss(dword_ptr(rbx + 0x11C), xmm6).unwrap();

    // Restore 0x1F80 and confirm subnormal preserved again
    a.ldmxcsr(dword_ptr(rbx + 0x10)).unwrap();
    a.vmovss(xmm0, dword_ptr(rbx + 0x00)).unwrap();
    a.vmulss(xmm1, xmm0, xmm0).unwrap();
    a.vmovss(dword_ptr(rbx + 0x120), xmm1).unwrap();

    a.add(rsp, 0x28).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    const DATA: usize = 0x200;
    let cptr = arena_alloc(code.len() + 16, 16);
    let dptr = arena_alloc(DATA, 64);
    let mut init = vec![0u8; DATA];
    let put32 = |buf: &mut [u8], off: usize, v: u32| {
        buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    };
    let get32 = |buf: &[u8], off: usize| u32::from_le_bytes(buf[off..off + 4].try_into().unwrap());
    put32(&mut init, 0x00, 0x1D80_0000); // 2^-68 (squared = 2^-136 = 0x0000_2000)
    put32(&mut init, 0x04, 0x0000_2000); // 2^-136 (subnormal)
    put32(&mut init, 0x08, 1.0f32.to_bits());
    put32(&mut init, 0x0C, 0x9FC0);
    put32(&mut init, 0x10, 0x1F80);
    put32(&mut init, 0x14, 0x7F80);
    put32(&mut init, 0x18, 1.0f32.to_bits());
    put32(&mut init, 0x1C, 1e-20f32.to_bits());
    put32(&mut init, 0x20, 0x5F80);

    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), cptr, code.len());
        core::ptr::write_bytes(cptr.add(code.len()), 0xCC, 16);

        for mode in [2u8, 1u8] {
            win32_rt::ucrt::runtime::set_guest_mxcsr(0x1F80);
            HOST_SAW_FPCR.store(u64::MAX, core::sync::atomic::Ordering::SeqCst);
            HOST_SAW_MXCSR.store(0, core::sync::atomic::Ordering::SeqCst);
            let (_, mem) = run_mode(cptr, dptr, &init, mode);

            // Hardware FPCR must be clean after guest returns!
            assert_eq!(
                super::read_fpcr() & super::FPCR_GUEST_MASK,
                0,
                "mode={mode}: host FPCR leaked after guest return"
            );
            assert_eq!(
                HOST_SAW_FPCR.load(core::sync::atomic::Ordering::SeqCst),
                0,
                "mode={mode}: host callback saw dirty FPCR"
            );
            assert_eq!(
                HOST_SAW_MXCSR.load(core::sync::atomic::Ordering::SeqCst),
                0x9FC0,
                "mode={mode}: host callback did not see synced guest MXCSR"
            );
            assert_eq!(get32(&mem, 0x100), 0x1F80, "mode={mode}: initial stmxcsr");
            assert_eq!(get32(&mem, 0x104), 0x0000_2000, "mode={mode}: subnormal under 0x1F80");
            assert_eq!(get32(&mem, 0x108), 0x9FC0, "mode={mode}: stmxcsr after ldmxcsr 0x9FC0");
            assert_eq!(get32(&mem, 0x10C), 0x0000_0000, "mode={mode}: FTZ (2^-68)^2 -> +0.0");
            assert_eq!(get32(&mem, 0x110), 0x0000_0000, "mode={mode}: DAZ subnormal*1 -> +0.0");
            assert_eq!(get32(&mem, 0x114), 0x1F80, "mode={mode}: host _controlfp update visible in guest");
            assert_eq!(get32(&mem, 0x118), 0x3F80_0001, "mode={mode}: RP rounds 1+1e-20 up by 1 ulp");
            assert_eq!(get32(&mem, 0x11C), 0x3F80_0000, "mode={mode}: RZ truncates 1+1e-20 to 1.0");
            assert_eq!(get32(&mem, 0x120), 0x0000_2000, "mode={mode}: subnormal restored after 0x1F80");
        }
        win32_rt::ucrt::runtime::set_guest_mxcsr(0x1F80);
        crate::exe::unregister_native_va(HOST_VA);
    }
}

/// Regression for `Tr::block_v128_mask`: blocks that start with non-zero `ymm_hi` and read it via
/// 256->128 narrowing ops (`vcvtpd2dq xmm, ymm`, `vcvtps2ph xmm, ymm, imm8`, `vmovmskps r32, ymm`)
/// before a VEX-128 op overwrites the same register must NOT bulk-clear `ymm_hi` at block entry.
#[test]
fn block_v128_mask_preserves_ymm_hi_on_narrowing_ops() {
    use iced_x86::code_asm::*;

    // Block 1 loads ymm0 (4 f64s: [1.0, -2.0, 3.0, -4.0]) and ymm1 (8 f32s: [-1, 2, -3, 4, -5, 6, -7, 8]),
    // then jumps to Block 2 so Block 2 starts with live-in ymm0_hi / ymm1_hi and a VEX-128 clear of xmm0/xmm1.
    let mut a = CodeAssembler::new(64).unwrap();
    let mut blk2 = a.create_label();
    a.mov(rax, rcx).unwrap();
    a.vmovupd(ymm0, ymmword_ptr(rax + 0x00)).unwrap();
    a.vmovups(ymm1, ymmword_ptr(rax + 0x20)).unwrap();
    a.jmp(blk2).unwrap();

    a.set_label(&mut blk2).unwrap();
    // Narrowing reads of ymm0_hi and ymm1_hi BEFORE VEX-128 writes to xmm0/xmm1:
    a.vcvtpd2dq(xmm2, ymm0).unwrap();            // [1i32, -2i32, 3i32, -4i32]
    a.vcvtps2ph(xmm3, ymm1, 0).unwrap();         // 8 f16s in xmm3
    a.vmovmskps(edx, ymm1).unwrap();             // sign bits of all 8 lanes = 0b0101_0101 = 0x55
    // VEX-128 writes to xmm0/xmm1 (would trigger block-entry bulk clear of ymm0_hi/ymm1_hi if bug present):
    a.vxorps(xmm0, xmm0, xmm0).unwrap();
    a.vxorps(xmm1, xmm1, xmm1).unwrap();
    a.vmovdqu(xmmword_ptr(rax + 0x100), xmm2).unwrap();
    a.vmovdqu(xmmword_ptr(rax + 0x110), xmm3).unwrap();
    a.mov(dword_ptr(rax + 0x120), edx).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    const DATA: usize = 0x200;
    let cptr = arena_alloc(code.len() + 16, 16);
    let dptr = arena_alloc(DATA, 64);
    let mut init = vec![0u8; DATA];
    for (i, &v) in [1.0f64, -2.0, 3.0, -4.0].iter().enumerate() {
        init[i * 8..i * 8 + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, &v) in [-1.0f32, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0].iter().enumerate() {
        init[0x20 + i * 4..0x20 + i * 4 + 4].copy_from_slice(&v.to_bits().to_le_bytes());
    }

    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), cptr, code.len());
        core::ptr::write_bytes(cptr.add(code.len()), 0xCC, 16);
        let interp = run_mode(cptr, dptr, &init, 2);
        let jit1 = run_mode(cptr, dptr, &init, 1);
        assert_eq!(jit1.1[0x100..0x124], interp.1[0x100..0x124], "narrowing ymm_hi JIT vs interp");
        let cvt: [i32; 4] = core::array::from_fn(|i| {
            i32::from_le_bytes(jit1.1[0x100 + i * 4..0x104 + i * 4].try_into().unwrap())
        });
        assert_eq!(cvt, [1, -2, 3, -4], "vcvtpd2dq must read upper 128 bits of ymm0");
        let mask = u32::from_le_bytes(jit1.1[0x120..0x124].try_into().unwrap());
        assert_eq!(mask, 0x55, "vmovmskps must read upper 128 bits of ymm1");
        // Lanes 4..7 of vcvtps2ph (-5, 6, -7, 8) must be non-zero in upper 64 bits of xmm3!
        let hi_f16 = u64::from_le_bytes(jit1.1[0x118..0x120].try_into().unwrap());
        assert_ne!(hi_f16, 0, "vcvtps2ph must read upper 128 bits of ymm1");
    }
}






/// Compile while frozen, then reuse the same machine code for live and frozen reads.
#[test]
fn jit_timestamp_blocks_switch_clock_mode_at_execution() {
    use std::time::{Duration, Instant};
    let mut asm = CodeAssembler::new(64).unwrap();
    asm.mov(r8, rcx).unwrap();
    asm.mov(rcx, u64::MAX).unwrap();
    asm.stc().unwrap();
    asm.rdtsc().unwrap();
    asm.mov(qword_ptr(r8), rax).unwrap();
    asm.mov(qword_ptr(r8 + 8), rdx).unwrap();
    asm.mov(qword_ptr(r8 + 16), rcx).unwrap();
    asm.rdtscp().unwrap();
    asm.mov(qword_ptr(r8 + 24), rax).unwrap();
    asm.mov(qword_ptr(r8 + 32), rdx).unwrap();
    asm.mov(qword_ptr(r8 + 40), rcx).unwrap();
    asm.setc(byte_ptr(r8 + 48)).unwrap();
    asm.ret().unwrap();
    let code = arena_alloc(128, 16);
    let data = arena_alloc(64, 16);
    let bytes = asm.assemble(code as u64).unwrap();
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), code, bytes.len()); }
    let sample = || {
        let before = jit::test_executed();
        let (_, out) = unsafe { run_mode(code, data, &[0xff; 64], 1) };
        assert_eq!(jit::test_executed() - before, 13, "timestamp block fell back to the interpreter");
        let word = |n: usize| u64::from_le_bytes(out[n..n + 8].try_into().unwrap());
        assert_eq!((word(16), word(40), out[48]), (u64::MAX, 0, 1));
        assert!([0, 8, 24, 32].iter().all(|&n| word(n) <= u32::MAX as u64));
        (word(8) << 32 | word(0), word(32) << 32 | word(24))
    };
    jit::test_dump_start();
    assert_eq!(with_frozen_tsc(sample), (GUEST_TSC, GUEST_TSC));
    assert!(!jit::test_dump_take().is_empty());
    jit::test_dump_start();
    let before = guest_rdtscp();
    let wall = Instant::now();
    let (first, first_p) = sample();
    let inner = Instant::now();
    std::thread::sleep(Duration::from_millis(5));
    let lower_ns = inner.elapsed().as_nanos() as u64;
    let (last, last_p) = sample();
    let upper_ns = wall.elapsed().as_nanos() as u64;
    assert!(before <= first && first <= first_p && first_p <= last && last <= last_p && last_p <= guest_rdtscp());
    if !tsc_is_frozen() {
        let elapsed = last - first;
        assert!(elapsed >= lower_ns.saturating_sub(10_000) && elapsed <= upper_ns + 10_000);
    }
    assert_eq!(with_frozen_tsc(sample), (GUEST_TSC, GUEST_TSC));
    assert!(jit::test_dump_take().is_empty(), "changing clock mode recompiled the block");
}

/// Multi-threaded contention test for unaligned LOCK operations:
/// verifies zero lost updates when N threads concurrently hammer LOCK ops on:
///   (a) a misaligned word inside a 16-byte granule (handled by FEAT_LSE2 single-insn atomics)
///   (b) a word crossing a 16-byte boundary (serialized via the sharded spinlock)
/// across all LOCK sub-ops (add, sub, and, or, xor, inc, dec, not, neg, xchg, cmpxchg, xadd,
/// bts/btr/btc, cmpxchg8b, cmpxchg16b) and mixed JIT + interpreter threads.
#[test]
fn unaligned_lock_ops_contention() {
    use std::sync::{Arc, Barrier};

    const THREADS: usize = 4;
    const ITERS: u64 = 20_000;

    fn assemble_fn(build: impl FnOnce(&mut CodeAssembler)) -> *mut u8 {
        let mut a = CodeAssembler::new(64).unwrap();
        build(&mut a);
        let code = arena_alloc(256, 16);
        let bytes = a.assemble(code as u64).unwrap();
        assert!(bytes.len() <= 256);
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), code, bytes.len());
            core::ptr::write_bytes(code.add(bytes.len()), 0xCC, 256 - bytes.len());
        }
        code
    }

    // Run `code(rcx = ptr, rdx = iters, r8 = tid)` concurrently on `THREADS` threads.
    // `modes` chooses `jit::force(mode)` per thread (1 = JIT, 2 = interpreter).
    fn run_contended(code: *mut u8, ptr: usize, iters: u64, modes: [u8; THREADS]) -> [u64; THREADS] {
        let barrier = Arc::new(Barrier::new(THREADS));
        let mut results = [0u64; THREADS];
        std::thread::scope(|s| {
            let mut handles = Vec::with_capacity(THREADS);
            for tid in 0..THREADS {
                let b = Arc::clone(&barrier);
                let mode = modes[tid];
                let code_addr = code as u64;
                handles.push(s.spawn(move || {
                    b.wait();
                    unsafe {
                        jit::force(mode);
                        let (ret, _) = emu_run_full(
                            code_addr,
                            [ptr as u64, iters, tid as u64, 0, 0, 0, 0, 0],
                            [0; 4],
                            [0; 8],
                        );
                        jit::force(0);
                        ret
                    }
                }));
            }
            for (tid, h) in handles.into_iter().enumerate() {
                results[tid] = h.join().expect("worker thread panicked");
            }
        });
        results
    }

    let buf = arena_alloc(128, 64);
    // Test offsets: misaligned inside 16B window AND crossing 16B boundary.
    let offsets_32: [(&str, usize); 2] = [("misaligned_in_16b", 1), ("crossing_16b", 14)];
    let offsets_64: [(&str, usize); 2] = [("misaligned_in_16b", 1), ("crossing_16b", 12)];
    let offsets_16: [(&str, usize); 2] = [("misaligned_in_16b", 1), ("crossing_16b", 15)];

    // 1. LOCK ADD + LOCK SUB (net +1 per iter: +3 then -2) on 32-bit and 64-bit.
    let lock_add_sub_32 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.set_label(&mut lp).unwrap();
        a.lock().add(dword_ptr(rcx), 3).unwrap();
        a.lock().sub(dword_ptr(rcx), 2).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    let lock_add_sub_64 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.set_label(&mut lp).unwrap();
        a.lock().add(qword_ptr(rcx), 3).unwrap();
        a.lock().sub(qword_ptr(rcx), 2).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_32 {
        for modes in [[1, 1, 1, 1], [1, 2, 1, 2]] {
            unsafe {
                core::ptr::write_bytes(buf, 0, 128);
                let p = buf.add(off);
                run_contended(lock_add_sub_32, p as usize, ITERS, modes);
                let got = core::ptr::read_unaligned(p as *const u32);
                assert_eq!(
                    got as u64,
                    THREADS as u64 * ITERS,
                    "lock add/sub u32 {label} modes={modes:?}"
                );
            }
        }
    }
    for &(label, off) in &offsets_64 {
        for modes in [[1, 1, 1, 1], [1, 2, 1, 2]] {
            unsafe {
                core::ptr::write_bytes(buf, 0, 128);
                let p = buf.add(off);
                run_contended(lock_add_sub_64, p as usize, ITERS, modes);
                let got = core::ptr::read_unaligned(p as *const u64);
                assert_eq!(
                    got,
                    THREADS as u64 * ITERS,
                    "lock add/sub u64 {label} modes={modes:?}"
                );
            }
        }
    }

    // 2. LOCK INC + LOCK DEC (2x inc, 1x dec -> net +1 per iter) on 16-bit and 32-bit.
    let lock_inc_dec_16 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.set_label(&mut lp).unwrap();
        a.lock().inc(word_ptr(rcx)).unwrap();
        a.lock().inc(word_ptr(rcx)).unwrap();
        a.lock().dec(word_ptr(rcx)).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    let lock_inc_dec_32 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.set_label(&mut lp).unwrap();
        a.lock().inc(dword_ptr(rcx)).unwrap();
        a.lock().inc(dword_ptr(rcx)).unwrap();
        a.lock().dec(dword_ptr(rcx)).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_16 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            run_contended(lock_inc_dec_16, p as usize, ITERS, [1, 1, 1, 1]);
            let got = core::ptr::read_unaligned(p as *const u16);
            assert_eq!(
                got,
                (THREADS as u64 * ITERS) as u16,
                "lock inc/dec u16 {label}"
            );
        }
    }
    for &(label, off) in &offsets_32 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            run_contended(lock_inc_dec_32, p as usize, ITERS, [1, 1, 1, 1]);
            let got = core::ptr::read_unaligned(p as *const u32);
            assert_eq!(got as u64, THREADS as u64 * ITERS, "lock inc/dec u32 {label}");
        }
    }

    // 3. LOCK XADD: sum all returned old values across all threads == N*(N-1)/2.
    let lock_xadd_64 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.xor(rax, rax).unwrap();
        a.set_label(&mut lp).unwrap();
        a.mov(r9, 1u64).unwrap();
        a.lock().xadd(qword_ptr(rcx), r9).unwrap();
        a.add(rax, r9).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_64 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            let sums = run_contended(lock_xadd_64, p as usize, ITERS, [1, 2, 1, 2]);
            let got = core::ptr::read_unaligned(p as *const u64);
            let total = THREADS as u64 * ITERS;
            assert_eq!(got, total, "lock xadd u64 final {label}");
            let sum_old: u64 = sums.iter().sum();
            assert_eq!(sum_old, total * (total - 1) / 2, "lock xadd u64 sum {label}");
        }
    }

    // 4. LOCK CMPXCHG (CAS increment loop) on 32-bit and 64-bit.
    let lock_cmpxchg_32 = assemble_fn(|a| {
        let mut lp = a.create_label();
        let mut retry = a.create_label();
        a.set_label(&mut lp).unwrap();
        a.mov(eax, dword_ptr(rcx)).unwrap();
        a.set_label(&mut retry).unwrap();
        a.lea(r9d, dword_ptr(rax + 1)).unwrap();
        a.lock().cmpxchg(dword_ptr(rcx), r9d).unwrap();
        a.jne(retry).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_32 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            run_contended(lock_cmpxchg_32, p as usize, ITERS, [1, 2, 1, 2]);
            let got = core::ptr::read_unaligned(p as *const u32);
            assert_eq!(got as u64, THREADS as u64 * ITERS, "lock cmpxchg u32 {label}");
        }
    }

    // 5. LOCK AND / OR / XOR / BTS / BTR / BTC on disjoint per-thread bit slices of the same word.
    let lock_bitwise_32 = assemble_fn(|a| {
        // Each thread tid in 0..4 owns bits: (tid*4 + 0..=3):
        //   bit tid*4 + 0: set via LOCK OR, cleared via LOCK AND, then left set via LOCK OR
        //   bit tid*4 + 1: toggled (2*iters + 1) times via LOCK XOR -> ends 1
        //   bit tid*4 + 2: set via LOCK BTS, cleared via LOCK BTR, set via LOCK BTS -> ends 1
        //   bit tid*4 + 3: toggled (2*iters + 1) times via LOCK BTC -> ends 1
        let mut lp = a.create_label();
        a.mov(r9, r8).unwrap();
        a.shl(r9, 2).unwrap(); // base_bit = tid * 4
        a.mov(r10d, 1u32).unwrap();
        a.mov(r11, rcx).unwrap();
        a.mov(rcx, r9).unwrap();
        a.shl(r10d, cl).unwrap(); // mask0 = 1 << base_bit
        a.mov(eax, r10d).unwrap();
        a.not(eax).unwrap(); // ~mask0
        a.lea(esi, dword_ptr(r10 + r10)).unwrap(); // mask1 = 1 << (base_bit + 1)
        a.mov(rcx, r11).unwrap();
        // Pre-toggle once so total toggles = 2*iters + 1 (odd -> ends 1).
        a.lock().xor(dword_ptr(rcx), esi).unwrap();
        a.lea(edi, dword_ptr(r9 + 3)).unwrap();
        a.lock().btc(dword_ptr(rcx), edi).unwrap();
        a.set_label(&mut lp).unwrap();
        a.lock().or(dword_ptr(rcx), r10d).unwrap();
        a.lock().and(dword_ptr(rcx), eax).unwrap();
        a.lock().or(dword_ptr(rcx), r10d).unwrap();
        a.lock().xor(dword_ptr(rcx), esi).unwrap();
        a.lock().xor(dword_ptr(rcx), esi).unwrap();
        a.lea(r11d, dword_ptr(r9 + 2)).unwrap();
        a.lock().bts(dword_ptr(rcx), r11d).unwrap();
        a.lock().btr(dword_ptr(rcx), r11d).unwrap();
        a.lock().bts(dword_ptr(rcx), r11d).unwrap();
        a.lock().btc(dword_ptr(rcx), edi).unwrap();
        a.lock().btc(dword_ptr(rcx), edi).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_32 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            run_contended(lock_bitwise_32, p as usize, ITERS / 4, [1, 1, 1, 1]);
            let got = core::ptr::read_unaligned(p as *const u32);
            assert_eq!(got, 0x0000_FFFF, "lock and/or/xor/bts/btr/btc u32 {label}");
        }
    }

    // 6. LOCK NOT and LOCK NEG: odd number of total ops flips/negates exact initial value.
    let lock_not_32 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.set_label(&mut lp).unwrap();
        a.lock().not(dword_ptr(rcx)).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    let lock_neg_32 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.set_label(&mut lp).unwrap();
        a.lock().neg(dword_ptr(rcx)).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_32 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            core::ptr::write_unaligned(p as *mut u32, 0xA55A_1234);
            // 4 threads * 5000 = 20000 (even) -> must end at 0xA55A_1234.
            run_contended(lock_not_32, p as usize, ITERS / 4, [1, 1, 1, 1]);
            assert_eq!(core::ptr::read_unaligned(p as *const u32), 0xA55A_1234, "lock not u32 {label}");
            // 4 threads * 5000 = 20000 (even) -> must end at 0xA55A_1234.
            run_contended(lock_neg_32, p as usize, ITERS / 4, [1, 1, 1, 1]);
            assert_eq!(core::ptr::read_unaligned(p as *const u32), 0xA55A_1234, "lock neg u32 {label}");
        }
    }

    // 7. XCHG (implicit LOCK): token conservation across all threads.
    let xchg_conserve_64 = assemble_fn(|a| {
        let mut lp = a.create_label();
        a.xor(rax, rax).unwrap(); // sum_out
        a.lea(r10, qword_ptr(r8 + 1)).unwrap(); // fixed token = tid + 1
        a.set_label(&mut lp).unwrap();
        a.mov(r9, r10).unwrap();
        a.xchg(qword_ptr(rcx), r9).unwrap();
        a.add(rax, r9).unwrap();
        a.dec(rdx).unwrap();
        a.jnz(lp).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_64 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            let init_tok = 999u64;
            core::ptr::write_unaligned(p as *mut u64, init_tok);
            let outs = run_contended(xchg_conserve_64, p as usize, ITERS, [1, 1, 1, 1]);
            let final_tok = core::ptr::read_unaligned(p as *const u64);
            let sum_in: u64 = init_tok + (1..=THREADS as u64).map(|t| t * ITERS).sum::<u64>();
            let sum_out: u64 = final_tok + outs.iter().sum::<u64>();
            assert_eq!(sum_out, sum_in, "xchg u64 token conservation {label}");
        }
    }

    // 8. LOCK CMPXCHG8B (64-bit CAS increment loop) on misaligned-in-16B and 16B-crossing.
    let lock_cmpxchg8b = assemble_fn(|a| {
        let mut lp = a.create_label();
        let mut retry = a.create_label();
        a.push(rbx).unwrap();
        a.mov(r9, rcx).unwrap();
        a.mov(r10, rdx).unwrap();
        a.set_label(&mut lp).unwrap();
        a.mov(eax, dword_ptr(r9)).unwrap();
        a.mov(edx, dword_ptr(r9 + 4)).unwrap();
        a.set_label(&mut retry).unwrap();
        a.mov(ebx, eax).unwrap();
        a.mov(ecx, edx).unwrap();
        a.add(ebx, 1).unwrap();
        a.adc(ecx, 0).unwrap();
        a.lock().cmpxchg8b(qword_ptr(r9)).unwrap();
        a.jne(retry).unwrap();
        a.dec(r10).unwrap();
        a.jnz(lp).unwrap();
        a.pop(rbx).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &offsets_64 {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            run_contended(lock_cmpxchg8b, p as usize, ITERS, [1, 2, 1, 2]);
            let got = core::ptr::read_unaligned(p as *const u64);
            assert_eq!(got, THREADS as u64 * ITERS, "lock cmpxchg8b {label}");
        }
    }

    // 9. LOCK CMPXCHG16B (128-bit CAS increment loop) on aligned AND misaligned (crossing 16B).
    let lock_cmpxchg16b = assemble_fn(|a| {
        let mut lp = a.create_label();
        let mut retry = a.create_label();
        a.push(rbx).unwrap();
        a.mov(r9, rcx).unwrap();
        a.mov(r10, rdx).unwrap();
        a.set_label(&mut lp).unwrap();
        a.mov(rax, qword_ptr(r9)).unwrap();
        a.mov(rdx, qword_ptr(r9 + 8)).unwrap();
        a.set_label(&mut retry).unwrap();
        a.mov(rbx, rax).unwrap();
        a.mov(rcx, rdx).unwrap();
        a.add(rbx, 1).unwrap();
        a.adc(rcx, 0).unwrap();
        a.lock().cmpxchg16b(xmmword_ptr(r9)).unwrap();
        a.jne(retry).unwrap();
        a.dec(r10).unwrap();
        a.jnz(lp).unwrap();
        a.pop(rbx).unwrap();
        a.xor(eax, eax).unwrap();
        a.ret().unwrap();
    });
    for &(label, off) in &[("aligned_16b", 0usize), ("crossing_16b", 4usize)] {
        unsafe {
            core::ptr::write_bytes(buf, 0, 128);
            let p = buf.add(off);
            run_contended(lock_cmpxchg16b, p as usize, ITERS / 2, [1, 2, 1, 2]);
            let lo = core::ptr::read_unaligned(p as *const u64);
            let hi = core::ptr::read_unaligned(p.add(8) as *const u64);
            assert_eq!((lo, hi), (THREADS as u64 * (ITERS / 2), 0), "lock cmpxchg16b {label}");
        }
    }
}

/// Differential parity test across Interpreter (FastOp), Live JIT, Disk AOT, and Static Mach-O AOT
/// covering every LOCK sub-op (add, sub, and, or, xor, inc, dec, not, neg, bt/bts/btr/btc, xchg,
/// xadd, cmpxchg, cmpxchg8b, cmpxchg16b) across widths (8/16/32/64/128) and alignments (aligned,
/// misaligned within 16B, and crossing 16B), including preserved and updated RFLAGS.
#[test]
fn unaligned_lock_ops_all_subops_tier_and_flag_parity() {
    use crate::emu::static_macho::{emit_macho_object_from_aot_bytes, MachoPlatform};

    let mut a = CodeAssembler::new(64).unwrap();
    // Input: rcx = 64B-aligned data buffer (512 bytes).
    // Output: writes results + flag snapshots (pushfq; pop) into [rcx + 0x100..].
    a.push(rbx).unwrap();
    a.mov(r10, rcx).unwrap();

    // Helper macro: run `body`, then snapshot RFLAGS (masked to CF|PF|ZF|SF|OF = 0x8C5) to [r10 + out_off].
    let mut snap_off = 0x100i32;
    let mut emit_snap = |a: &mut CodeAssembler| {
        a.pushfq().unwrap();
        a.pop(r11).unwrap();
        a.and(r11, 0x8C5).unwrap();
        a.mov(qword_ptr(r10 + snap_off), r11).unwrap();
        snap_off += 8;
    };

    // Exercise 3 offsets per width: aligned, misaligned inside 16B, crossing 16B.
    // Buffer layout [0x00..0x80]:
    //   8-bit:  0x00, 0x07, 0x0F
    //   16-bit: 0x10, 0x11, 0x1F (crosses 0x20)
    //   32-bit: 0x30, 0x31, 0x3E (crosses 0x40)
    //   64-bit: 0x50, 0x51, 0x5C (crosses 0x60)
    //   128-bit: 0x70 (aligned), 0x74 (crosses 0x80)
    for &off in &[0x00i32, 0x07, 0x0F] {
        a.mov(rax, 0xDEAD_BEEF_1234_56A5u64).unwrap();
        a.mov(rdx, 0xCAFE_BABE_9876_543Bu64).unwrap();
        a.lock().add(byte_ptr(r10 + off), dl).unwrap();
        emit_snap(&mut a);
        a.lock().sub(byte_ptr(r10 + off), 0x19).unwrap();
        emit_snap(&mut a);
        a.lock().and(byte_ptr(r10 + off), 0xF3).unwrap();
        emit_snap(&mut a);
        a.lock().or(byte_ptr(r10 + off), 0x24).unwrap();
        emit_snap(&mut a);
        a.lock().xor(byte_ptr(r10 + off), 0x5A).unwrap();
        emit_snap(&mut a);
        a.stc().unwrap();
        a.lock().inc(byte_ptr(r10 + off)).unwrap(); // must preserve CF=1
        emit_snap(&mut a);
        a.clc().unwrap();
        a.lock().dec(byte_ptr(r10 + off)).unwrap(); // must preserve CF=0
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.lock().not(byte_ptr(r10 + off)).unwrap(); // must preserve all flags
        emit_snap(&mut a);
        a.lock().neg(byte_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.xchg(byte_ptr(r10 + off), dl).unwrap(); // must preserve all flags
        emit_snap(&mut a);
        a.lock().xadd(byte_ptr(r10 + off), dl).unwrap();
        emit_snap(&mut a);
        // cmpxchg fail (al != mem) then succeed (al == mem)
        a.mov(al, 0xEE).unwrap();
        a.lock().cmpxchg(byte_ptr(r10 + off), dl).unwrap();
        emit_snap(&mut a);
        a.lock().cmpxchg(byte_ptr(r10 + off), dl).unwrap();
        emit_snap(&mut a);
    }

    for &off in &[0x10i32, 0x11, 0x1F] {
        a.mov(rax, 0x1122_3344_5566_7788u64).unwrap();
        a.mov(rdx, 0x99AA_BBCC_DDEE_1234u64).unwrap();
        a.lock().add(word_ptr(r10 + off), dx).unwrap();
        emit_snap(&mut a);
        a.lock().sub(word_ptr(r10 + off), 0x0555).unwrap();
        emit_snap(&mut a);
        a.lock().and(word_ptr(r10 + off), 0x7FFEu32).unwrap();
        emit_snap(&mut a);
        a.lock().or(word_ptr(r10 + off), 0x0041).unwrap();
        emit_snap(&mut a);
        a.lock().xor(word_ptr(r10 + off), 0x5A5A).unwrap();
        emit_snap(&mut a);
        a.stc().unwrap();
        a.lock().inc(word_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.clc().unwrap();
        a.lock().dec(word_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.lock().not(word_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.lock().neg(word_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.lock().bts(word_ptr(r10 + off), 5).unwrap();
        emit_snap(&mut a);
        a.lock().btr(word_ptr(r10 + off), 5).unwrap();
        emit_snap(&mut a);
        a.lock().btc(word_ptr(r10 + off), 3).unwrap();
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.xchg(word_ptr(r10 + off), dx).unwrap();
        emit_snap(&mut a);
        a.lock().xadd(word_ptr(r10 + off), dx).unwrap();
        emit_snap(&mut a);
        a.mov(ax, 0xBEEF).unwrap();
        a.lock().cmpxchg(word_ptr(r10 + off), dx).unwrap();
        emit_snap(&mut a);
        a.lock().cmpxchg(word_ptr(r10 + off), dx).unwrap();
        emit_snap(&mut a);
    }

    for &off in &[0x30i32, 0x31, 0x3E] {
        a.mov(rax, 0x8000_0001u64).unwrap();
        a.mov(rdx, 0x7FFF_1234u64).unwrap();
        a.lock().add(dword_ptr(r10 + off), edx).unwrap();
        emit_snap(&mut a);
        a.lock().sub(dword_ptr(r10 + off), 0x12345).unwrap();
        emit_snap(&mut a);
        a.lock().and(dword_ptr(r10 + off), 0x0FFF_FFF0u32).unwrap();
        emit_snap(&mut a);
        a.lock().or(dword_ptr(r10 + off), 0x0000_00A5u32).unwrap();
        emit_snap(&mut a);
        a.lock().xor(dword_ptr(r10 + off), 0x5A5A_A5A5u32).unwrap();
        emit_snap(&mut a);
        a.stc().unwrap();
        a.lock().inc(dword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.clc().unwrap();
        a.lock().dec(dword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.cmp(eax, edx).unwrap();
        a.lock().not(dword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.lock().neg(dword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.cmp(eax, edx).unwrap();
        a.lock().bts(dword_ptr(r10 + off), 17).unwrap();
        emit_snap(&mut a);
        a.lock().btr(dword_ptr(r10 + off), 17).unwrap();
        emit_snap(&mut a);
        a.lock().btc(dword_ptr(r10 + off), 9).unwrap();
        emit_snap(&mut a);
        a.cmp(eax, edx).unwrap();
        a.xchg(dword_ptr(r10 + off), edx).unwrap();
        emit_snap(&mut a);
        a.lock().xadd(dword_ptr(r10 + off), edx).unwrap();
        emit_snap(&mut a);
        a.mov(eax, 0x1111_2222u32).unwrap();
        a.lock().cmpxchg(dword_ptr(r10 + off), edx).unwrap();
        emit_snap(&mut a);
        a.lock().cmpxchg(dword_ptr(r10 + off), edx).unwrap();
        emit_snap(&mut a);
    }

    for &off in &[0x50i32, 0x51, 0x5C] {
        a.mov(rax, 0x0123_4567_89AB_CDEFu64).unwrap();
        a.mov(rdx, 0xFEDC_BA98_7654_3210u64).unwrap();
        a.lock().add(qword_ptr(r10 + off), rdx).unwrap();
        emit_snap(&mut a);
        a.lock().sub(qword_ptr(r10 + off), 0x13579).unwrap();
        emit_snap(&mut a);
        a.lock().and(qword_ptr(r10 + off), rdx).unwrap();
        emit_snap(&mut a);
        a.lock().or(qword_ptr(r10 + off), rax).unwrap();
        emit_snap(&mut a);
        a.lock().xor(qword_ptr(r10 + off), rdx).unwrap();
        emit_snap(&mut a);
        a.stc().unwrap();
        a.lock().inc(qword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.clc().unwrap();
        a.lock().dec(qword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.lock().not(qword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.lock().neg(qword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.lock().bts(qword_ptr(r10 + off), 41).unwrap();
        emit_snap(&mut a);
        a.lock().btr(qword_ptr(r10 + off), 41).unwrap();
        emit_snap(&mut a);
        a.lock().btc(qword_ptr(r10 + off), 33).unwrap();
        emit_snap(&mut a);
        a.cmp(rax, rdx).unwrap();
        a.xchg(qword_ptr(r10 + off), rdx).unwrap();
        emit_snap(&mut a);
        a.lock().xadd(qword_ptr(r10 + off), rdx).unwrap();
        emit_snap(&mut a);
        a.mov(rax, 0x5555_6666_7777_8888u64).unwrap();
        a.lock().cmpxchg(qword_ptr(r10 + off), rdx).unwrap();
        emit_snap(&mut a);
        a.lock().cmpxchg(qword_ptr(r10 + off), rdx).unwrap();
        emit_snap(&mut a);
        // cmpxchg8b mismatch (loads edx:eax) then match (stores ecx:ebx); preserves CF/SF/OF!
        a.mov(eax, 0x1234_5678u32).unwrap();
        a.mov(edx, 0x9ABC_DEF0u32).unwrap();
        a.mov(ebx, 0xAAAA_BBBBu32).unwrap();
        a.mov(ecx, 0xCCCC_DDDDu32).unwrap();
        a.stc().unwrap();
        a.lock().cmpxchg8b(qword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
        a.lock().cmpxchg8b(qword_ptr(r10 + off)).unwrap();
        emit_snap(&mut a);
    }

    for &off in &[0x70i32, 0x74] {
        a.mov(rax, 0x1111_2222_3333_4444u64).unwrap();
        a.mov(rdx, 0x5555_6666_7777_8888u64).unwrap();
        a.mov(rbx, 0xAAAA_BBBB_CCCC_DDDDu64).unwrap();
        a.mov(rcx, 0xEEEE_FFFF_0102_0304u64).unwrap();
        a.stc().unwrap();
        a.lock().cmpxchg16b(xmmword_ptr(r10 + off)).unwrap(); // mismatch -> loads rdx:rax
        emit_snap(&mut a);
        a.lock().cmpxchg16b(xmmword_ptr(r10 + off)).unwrap(); // match -> stores rcx:rbx
        emit_snap(&mut a);
    }

    a.mov(rax, rdx).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();

    let data_len = (snap_off as usize + 64).max(0x1000);
    let code_cap = 0x4000usize;
    let code = arena_alloc(code_cap, 16);
    let bytes = a.assemble(code as u64).unwrap();
    assert!(bytes.len() + 16 <= code_cap);
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), code, bytes.len());
        core::ptr::write_bytes(code.add(bytes.len()), 0xCC, 16);
    }

    let mut init = vec![0u8; data_len];
    for (i, b) in init.iter_mut().take(0x90).enumerate() {
        *b = ((i as u8).wrapping_mul(37)).wrapping_add(11);
    }
    let d_interp = arena_alloc(data_len, 64);
    let d_jit = arena_alloc(data_len, 64);

    let interp = unsafe { run_mode(code, d_interp, &init, 2) };
    let jit_before = jit::test_executed();
    let jit1 = unsafe { run_mode(code, d_jit, &init, 1) };
    assert!(jit::test_executed() > jit_before, "LOCK test block must execute in JIT");
    assert_eq!(jit1, interp, "Live JIT differs from FastOp interpreter on LOCK ops");

    // Also build a Disk AOT image AND Static Mach-O `.aot.o` for this exact x86-64 code and verify:
    //   1. Zero runtime relocations (`skipped_reloc_blocks == 0`, 100% emitted into static __TEXT).
    //   2. Executing the AOT image directly produces bit-for-bit identical output to interpreter/JIT.
    let mut ips = vec![code as u64];
    let mut dec = iced_x86::Decoder::with_ip(64, &bytes, code as u64, iced_x86::DecoderOptions::NONE);
    let mut ins_cnt = 0usize;
    while dec.can_decode() {
        let ins = dec.decode();
        ins_cnt += 1;
        if ins_cnt >= BLK_MAX_INSNS {
            ips.push(ins.next_ip());
            ins_cnt = 0;
        }
    }
    ips.retain(|&ip| ip >= code as u64 && ip < code as u64 + bytes.len() as u64);
    ips.sort_unstable();
    ips.dedup();

    let mut bc = Box::new(BlockCache::new());
    let img = unsafe { aot::build_image_for_test(&mut *bc, &ips) }.expect("build aot image for LOCK ops");
    assert!(!img.blocks.is_empty());
    let key = aot::AotKey {
        image_hash: [0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_FF00],
        image_base: code as u64,
        image_size: code_cap as u64,
        codegen_hash: aot::codegen_hash(),
        knobs: jit::aot_knob_bits(),
    };
    let aot_bytes = aot::encode_for_test("lock_ops_test.dll", &key, &img);
    for plat in [MachoPlatform::MacOs, MachoPlatform::Ios] {
        let (_obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
            .unwrap_or_else(|e| panic!("static Mach-O emit ({plat}) failed: {e}"));
        assert_eq!(rep.skipped_reloc_blocks, 0, "LOCK blocks must have zero runtime relocations for static AOT");
        assert_eq!(rep.emitted_blocks, img.blocks.len());
    }
}

#[test]
fn sys_state_ops_aot_image_and_shifted_base_reload() {
    use crate::emu::static_macho::{emit_macho_object_from_aot_bytes, MachoPlatform};
    use iced_x86::code_asm::*;

    let mut a = CodeAssembler::new(64).unwrap();
    a.push(rbx).unwrap();
    a.mov(r8, rcx).unwrap();
    // 1. CPUID leaf 0 & 0xd/0
    a.xor(eax, eax).unwrap();
    a.xor(ecx, ecx).unwrap();
    a.cpuid().unwrap();
    a.mov(dword_ptr(r8), eax).unwrap();
    a.mov(dword_ptr(r8 + 4), ebx).unwrap();
    a.mov(dword_ptr(r8 + 8), ecx).unwrap();
    a.mov(dword_ptr(r8 + 12), edx).unwrap();
    // 2. XGETBV(0)
    a.xor(ecx, ecx).unwrap();
    a.xgetbv().unwrap();
    a.mov(dword_ptr(r8 + 16), eax).unwrap();
    a.mov(dword_ptr(r8 + 20), edx).unwrap();
    // 3. FLDCW / FNSTCW / FSTCW
    a.fldcw(word_ptr(r8 + 24)).unwrap();
    a.fnstcw(word_ptr(r8 + 28)).unwrap();
    a.fstcw(word_ptr(r8 + 30)).unwrap();
    // 4. LDMXCSR / VLDMXCSR / STMXCSR / VSTMXCSR + FP arithmetic & cvt under RD+FTZ+DAZ
    a.ldmxcsr(dword_ptr(r8 + 32)).unwrap();
    a.stmxcsr(dword_ptr(r8 + 36)).unwrap();
    a.vmovdqu(ymm0, ymmword_ptr(r8 + 64)).unwrap();
    a.vmovdqu(ymm15, ymmword_ptr(r8 + 96)).unwrap();
    a.movss(xmm2, dword_ptr(r8 + 44)).unwrap(); // 1.0f
    a.divss(xmm2, dword_ptr(r8 + 48)).unwrap(); // 1.0 / 3.0 under RD -> 0x3eaa_aaaa
    a.movss(dword_ptr(r8 + 52), xmm2).unwrap();
    // 5. All 12 XSTATE ops in the built .aot image:
    //    XSAVE/XSAVE64/XSAVEOPT/XSAVEOPT64/XSAVEC/XSAVEC64/FXSAVE/FXSAVE64 -> clobber ->
    //    FXRSTOR/FXRSTOR64/XRSTOR/XRSTOR64
    a.mov(eax, 7).unwrap();
    a.xor(edx, edx).unwrap();
    a.xsave(ptr(r8 + 128)).unwrap();
    a.xsaveopt(ptr(r8 + 128)).unwrap();
    a.xsaveopt64(ptr(r8 + 128)).unwrap();
    a.xsave64(ptr(r8 + 128)).unwrap();
    a.xsavec(ptr(r8 + 1024)).unwrap();
    a.xsavec64(ptr(r8 + 1024)).unwrap();
    a.fxsave(ptr(r8 + 1920)).unwrap();
    a.fxsave64(ptr(r8 + 1920)).unwrap();
    a.vzeroall().unwrap();
    a.vldmxcsr(dword_ptr(r8 + 40)).unwrap(); // default 0x1f80
    a.vstmxcsr(dword_ptr(r8 + 56)).unwrap();
    a.fxrstor(ptr(r8 + 1920)).unwrap();
    a.vzeroall().unwrap();
    a.fxrstor64(ptr(r8 + 1920)).unwrap();
    a.vzeroall().unwrap();
    a.mov(eax, 7).unwrap();
    a.xor(edx, edx).unwrap();
    a.xrstor(ptr(r8 + 128)).unwrap();
    a.vzeroall().unwrap();
    a.xrstor64(ptr(r8 + 1024)).unwrap();
    a.vmovdqu(ymmword_ptr(r8 + 2432), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(r8 + 2464), ymm15).unwrap();
    a.vzeroupper().unwrap();
    // Restore default MXCSR/FCW
    a.ldmxcsr(dword_ptr(r8 + 40)).unwrap();
    a.fldcw(word_ptr(r8 + 26)).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    let bytes = a.assemble(0).unwrap();

    let mut init = vec![0u8; 2560];
    init[24..26].copy_from_slice(&0x027fu16.to_le_bytes()); // test FCW
    init[26..28].copy_from_slice(&0x037fu16.to_le_bytes()); // default FCW
    init[32..36].copy_from_slice(&0xbfc0u32.to_le_bytes()); // RD + FTZ + DAZ
    init[40..44].copy_from_slice(&0x1f80u32.to_le_bytes()); // default MXCSR
    init[44..48].copy_from_slice(&1.0f32.to_bits().to_le_bytes());
    init[48..52].copy_from_slice(&3.0f32.to_bits().to_le_bytes());
    for i in 0..64 {
        init[64 + i] = (i as u8).wrapping_mul(17).wrapping_add(3);
    }

    // Build and execute at two distinct guest bases (simulating ASLR / shifted-base reload),
    // verifying zero runtime relocations, macOS & iOS static Mach-O emission, and bit-exact
    // parity with the generic interpreter.
    let mut reference = None;
    for shift in [0usize, 0x1000] {
        let code_cap = ((bytes.len() + 64) + 15) & !15;
        let code_raw = arena_alloc(code_cap + shift, 64);
        let code = unsafe { code_raw.add(shift) };
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), code, bytes.len());
            core::ptr::write_bytes(code.add(bytes.len()), 0xcc, 16);
        }

        // Reference run in generic interpreter
        let data_interp = arena_alloc(init.len(), 64);
        unsafe { core::ptr::copy_nonoverlapping(init.as_ptr(), data_interp, init.len()) };
        let fast_off = TEST_FAST_OFF.with(|s| s.replace(true));
        jit::force(2);
        unsafe {
            emu_run_full(code as u64, [data_interp as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        }
        jit::force(0);
        TEST_FAST_OFF.with(|s| s.set(fast_off));
        let interp_out = unsafe { core::slice::from_raw_parts(data_interp, init.len()) }.to_vec();
        if let Some(ref first) = reference {
            assert_eq!(&interp_out, first);
        } else {
            reference = Some(interp_out.clone());
        }

        // Build .aot image via build_image_for_test
        let ips: Vec<u64> = iced_x86::Decoder::with_ip(64, &bytes, code as u64, iced_x86::DecoderOptions::NONE)
            .into_iter()
            .map(|ins| ins.ip())
            .collect();
        let mut bc = Box::new(BlockCache::new());
        let img = unsafe { aot::build_image_for_test(&mut *bc, &ips) }
            .expect("build .aot image for sys/state ops");
        assert!(!img.blocks.is_empty());
        assert!(img.blocks.iter().all(|b| b.reloc_n == 0), "sys/state ops must emit 0 relocs");

        let key = aot::AotKey {
            image_hash: [0xA1B2_C3D4_E5F6_0718 ^ shift as u64, 0x1928_3746_5564_7382],
            image_base: code as u64,
            image_size: code_cap as u64,
            codegen_hash: aot::codegen_hash(),
            knobs: jit::aot_knob_bits(),
        };
        let aot_bytes = aot::encode_for_test("sys_state_ops_test.dll", &key, &img);
        for plat in [MachoPlatform::MacOs, MachoPlatform::Ios] {
            let (_obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
                .unwrap_or_else(|e| panic!("static Mach-O emit ({plat}) failed: {e}"));
            assert_eq!(rep.skipped_reloc_blocks, 0, "{plat}: 0 skipped blocks");
            assert_eq!(rep.emitted_blocks, img.blocks.len(), "{plat}: all blocks emitted");
        }

        // Execute through installed disk AOT cache (reloaded at fresh host MAP_JIT base)
        let data_aot = arena_alloc(init.len(), 64);
        unsafe { core::ptr::copy_nonoverlapping(init.as_ptr(), data_aot, init.len()) };
        let before = jit::test_aot_installs();
        assert!(unsafe { aot::test_install(&ips, || {}).0 } > 0);
        jit::force(1);
        unsafe {
            emu_run_full(code as u64, [data_aot as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        }
        jit::force(0);
        aot::test_uninstall();
        assert!(jit::test_aot_installs() > before, "shift={shift}: AOT blocks must actually execute");
        let aot_out = unsafe { core::slice::from_raw_parts(data_aot, init.len()) };
        assert_eq!(aot_out, interp_out.as_slice(), "shift={shift}: AOT output must match generic interpreter");
        assert_eq!(u32::from_le_bytes(aot_out[16..20].try_into().unwrap()), 7, "xgetbv XCR0");
        assert_eq!(u16::from_le_bytes(aot_out[28..30].try_into().unwrap()), 0x027f, "fnstcw");
        assert_eq!(u16::from_le_bytes(aot_out[30..32].try_into().unwrap()), 0x027f, "fstcw");
        assert_eq!(u32::from_le_bytes(aot_out[36..40].try_into().unwrap()), 0xbfc0, "stmxcsr");
        assert_eq!(u32::from_le_bytes(aot_out[52..56].try_into().unwrap()), 0x3eaa_aaaa, "RD divss");
        assert_eq!(&aot_out[2432..2496], &init[64..128], "YMM0/YMM15 round-trip in AOT");
    }
}

/// AOT inclusion, shifted-base reload, and zero-relocation static Mach-O (macOS & iOS) test
/// covering all 3 Slice 4 families: `MMX_ALIASED`, `SIMD_LEGACY_FALLBACK`, and `AVX2_GATHER`.
#[test]
fn aot_mmx_legacy_simd_gather_inclusion_shifted_reload_and_static_macho() {
    use crate::emu::static_macho::{emit_macho_object_from_aot_bytes, MachoPlatform};
    use iced_x86::code_asm::*;

    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r11, rcx).unwrap();
    // 1. MMX_ALIASED: movq, pshufw, phaddsw, pshufb, palignr, psllw, cvtpi2ps, cvtps2pi, maskmovq, emms
    a.movq(mm0, qword_ptr(r11)).unwrap();
    a.movq(mm1, qword_ptr(r11 + 8)).unwrap();
    a.pshufw(mm2, mm0, 0x1B).unwrap();
    a.phaddsw(mm2, mm1).unwrap();
    a.pshufb(mm2, qword_ptr(r11 + 16)).unwrap();
    a.palignr(mm2, mm1, 3).unwrap();
    a.psllw(mm2, 2).unwrap();
    a.movq(qword_ptr(r11 + 128), mm2).unwrap();
    a.movups(xmm0, xmmword_ptr(r11 + 32)).unwrap();
    a.cvtpi2ps(xmm0, mm2).unwrap();
    a.movups(xmmword_ptr(r11 + 136), xmm0).unwrap();
    a.cvtps2pi(mm3, xmm0).unwrap();
    a.movq(qword_ptr(r11 + 152), mm3).unwrap();
    a.lea(rdi, ptr(r11 + 160)).unwrap();
    a.movq(mm4, qword_ptr(r11 + 24)).unwrap();
    a.db(&[0x0f, 0xf7, 0xd4]).unwrap(); // maskmovq mm2, mm4
    a.emms().unwrap();

    // 2. SIMD_LEGACY_FALLBACK: vphaddsw (256), vphsubsw (128), vphminposuw, vmpsadbw (256), vdppd, vpcmpestri64, vmaskmovdqu
    a.vmovdqu(ymm4, ymmword_ptr(r11)).unwrap();
    a.vmovdqu(ymm5, ymmword_ptr(r11 + 32)).unwrap();
    a.vphaddsw(ymm6, ymm4, ymm5).unwrap();
    a.vmovdqu(ymmword_ptr(r11 + 176), ymm6).unwrap();
    a.vphsubsw(xmm6, xmm4, xmm5).unwrap();
    a.vmovdqu(ymmword_ptr(r11 + 208), ymm6).unwrap();
    a.vphminposuw(xmm6, xmm4).unwrap();
    a.vmovdqu(ymmword_ptr(r11 + 240), ymm6).unwrap();
    a.vmpsadbw(ymm6, ymm4, ymm5, 0x2D).unwrap();
    a.vmovdqu(ymmword_ptr(r11 + 272), ymm6).unwrap();
    a.vdppd(xmm6, xmm4, xmm5, 0x33).unwrap();
    a.movupd(xmmword_ptr(r11 + 304), xmm6).unwrap();
    a.mov(rax, 0x0000_0001_0000_0004u64 as i64).unwrap();
    a.mov(rdx, 8u64 as i64).unwrap();
    a.movdqu(xmm0, xmm4).unwrap();
    a.movdqu(xmm1, xmm5).unwrap();
    a.db(&[0xc4, 0xe3, 0xf9, 0x61, 0xc1, 0x08]).unwrap(); // vpcmpestri64 xmm0, xmm1, 8
    a.mov(qword_ptr(r11 + 320), rcx).unwrap();
    a.lea(rdi, ptr(r11 + 336)).unwrap();
    a.db(&[0xc5, 0xf9, 0xf7, 0xc1]).unwrap(); // vmaskmovdqu xmm0, xmm1

    // 3. AVX2_GATHER: vpgatherdd (256) & vgatherqpd (256)
    a.lea(rdx, ptr(r11 + 64)).unwrap();
    a.vmovdqu(ymm1, ymmword_ptr(r11 + 96)).unwrap();
    a.vmovdqu(ymm2, ymmword_ptr(r11 + 32)).unwrap(); // mask (mixed MSBs)
    a.vmovdqu(ymm0, ymmword_ptr(r11)).unwrap();
    a.vpgatherdd_3(ymm0, dword_ptr(rdx + ymm1 * 4), ymm2).unwrap();
    a.vmovdqu(ymmword_ptr(r11 + 352), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(r11 + 384), ymm2).unwrap();

    a.vzeroupper().unwrap();
    a.mov(rax, qword_ptr(r11 + 128)).unwrap();
    a.ret().unwrap();

    let data_len = 512usize;
    let code_cap = 0x2000usize;
    let code_a = arena_alloc(code_cap, 16);
    let code_b = arena_alloc(code_cap, 16); // shifted guest base!
    assert_ne!(code_a, code_b);
    let bytes = a.assemble(code_a as u64).unwrap();
    assert!(bytes.len() + 16 <= code_cap);
    for c in [code_a, code_b] {
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), c, bytes.len());
            core::ptr::write_bytes(c.add(bytes.len()), 0xCC, 16);
        }
    }

    let mut init = vec![0u8; data_len];
    for (i, b) in init.iter_mut().take(96).enumerate() {
        *b = ((i as u8).wrapping_mul(53)).wrapping_add(17);
    }
    // Safe small indices [0, 1, 2, 3, 4, 5, 6, 7] at +96..128 for vpgatherdd into [r11 + 64..96]
    for i in 0..8i32 {
        init[96 + (i as usize) * 4..100 + (i as usize) * 4].copy_from_slice(&i.to_le_bytes());
    }

    let d_interp = arena_alloc(data_len, 64);
    let d_jit = arena_alloc(data_len, 64);
    let d_aot_a = arena_alloc(data_len, 64);
    let d_aot_b = arena_alloc(data_len, 64);

    let interp = unsafe { run_mode(code_a, d_interp, &init, 2) };

    // Collect block entry IPs for code_a and code_b
    let collect_ips = |base: u64| {
        let mut ips = vec![base];
        let mut dec = iced_x86::Decoder::with_ip(64, &bytes, base, iced_x86::DecoderOptions::NONE);
        let mut ins_cnt = 0usize;
        while dec.can_decode() {
            let ins = dec.decode();
            ins_cnt += 1;
            if ins_cnt >= BLK_MAX_INSNS {
                ips.push(ins.next_ip());
                ins_cnt = 0;
            }
        }
        ips.retain(|&ip| ip >= base && ip < base + bytes.len() as u64);
        ips.sort_unstable();
        ips.dedup();
        ips
    };
    let ips_a = collect_ips(code_a as u64);
    let ips_b = collect_ips(code_b as u64);

    // 1. Build AOT image, assert zero relocations, encode/decode round-trip, emit static Mach-O (macOS & iOS).
    let mut bc = Box::new(BlockCache::new());
    let img = unsafe { aot::build_image_for_test(&mut *bc, &ips_a) }.expect("build AOT image for Slice 4");
    assert!(!img.blocks.is_empty());
    assert!(img.relocs.is_empty(), "Slice 4 AOT blocks must have zero runtime relocations");
    let key = aot::AotKey {
        image_hash: [0x4433_2211_8877_6655, 0xCCBB_AA99_00FF_EEDD],
        image_base: code_a as u64,
        image_size: code_cap as u64,
        codegen_hash: aot::codegen_hash(),
        knobs: jit::aot_knob_bits(),
    };
    let aot_bytes = aot::encode_for_test("slice4_mmx_simd_gather.dll", &key, &img);
    let (dec_name, dec_key, dec_img, dec_flags) = aot::decode(&aot_bytes).expect("decode Slice 4 AOT bytes");
    assert_eq!(dec_name, "slice4_mmx_simd_gather.dll");
    assert_eq!(dec_key, key);
    assert_eq!(dec_flags, 0);
    assert_eq!(dec_img.blocks.len(), img.blocks.len());
    for plat in [MachoPlatform::MacOs, MachoPlatform::Ios] {
        let (_obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
            .unwrap_or_else(|e| panic!("static Mach-O emit ({plat}) failed: {e}"));
        assert_eq!(rep.skipped_reloc_blocks, 0, "Slice 4 blocks must have zero relocs for static Mach-O ({plat})");
        assert_eq!(rep.emitted_blocks, img.blocks.len());
    }

    // 2. Execute Disk AOT at base A and shifted base B (before live-JIT populates those IPs),
    // verifying AOT install hits and exact parity with the interpreter.
    for (c_ptr, d_ptr, ips, label) in [
        (code_a, d_aot_a, &ips_a, "base A"),
        (code_b, d_aot_b, &ips_b, "shifted base B"),
    ] {
        let before = jit::test_aot_installs();
        let (live, relocs, _links) = unsafe { aot::test_install(ips, || {}) };
        assert!(live > 0, "AOT must install live blocks at {label}");
        assert_eq!(relocs, 0, "AOT must have zero relocations at {label}");
        let aot_out = unsafe { run_mode(c_ptr, d_ptr, &init, 1) };
        aot::test_uninstall();
        assert!(jit::test_aot_installs() > before, "AOT must execute installed blocks at {label}");
        assert_eq!(aot_out, interp, "Disk AOT differs from interpreter at {label}");
    }

    // 3. Also run Live JIT on a fresh copy and verify parity.
    let code_c = arena_alloc(code_cap, 16);
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), code_c, bytes.len());
        core::ptr::write_bytes(code_c.add(bytes.len()), 0xCC, 16);
    }
    let jit_before = jit::test_executed();
    let jit_out = unsafe { run_mode(code_c, d_jit, &init, 1) };
    assert!(jit::test_executed() > jit_before, "Slice 4 test block must execute in Live JIT");
    assert_eq!(jit_out, interp, "Live JIT differs from FastOp interpreter on Slice 4 ops");
}

/// Dynamic `call rax` and tail `jmp rax` to `timeGetTime`, `GetTickCount`, `GetTickCount64`,
/// `EnterCriticalSection`, `LeaveCriticalSection`, `RtlEnterCriticalSection`,
/// `RtlLeaveCriticalSection`, `XInputGetState`, `XInputGetStateEx`, `SwitchToThread`, and
/// `_Thrd_yield` must take the zero-spill `lookup_call`/`lookup_jmp` inline fast thunks,
/// preserve all callee-saved GPRs and XMM registers, honour exact Win32 semantics (32-bit ms
/// wrap, CS layout + recursion + TryEnterCriticalSection consistency, XInput packet numbers +
/// GUIDE masking + ERROR_DEVICE_NOT_CONNECTED, SwitchToThread BOOL), and bail cleanly to the
/// slow path on edge cases.
#[test]
fn lookup_call_and_jmp_fast_thunks_semantics_and_fallback() {
    use core::sync::atomic::{AtomicU64, Ordering};
    use win32_rt::kernel32::time::QpcFastState;
    use win32_rt::misc::xinput1_4::{self, btn, DeviceInfo, Gamepad, State, XInputFastTable};
    use win32_rt::sync::{
        CriticalSection, InitializeCriticalSection, TryEnterCriticalSection,
    };

    std::thread::Builder::new()
        .name("fast_thunks_dyn".into())
        .spawn(|| {
            // Ensure this thread has a valid TEB so inline CS fast path sees a non-zero TID.
            let _teb = win32_rt::teb::current();
            let my_tid = win32_rt::teb::thread_id() as usize;
            assert_ne!(my_tid, 0);

            let exp = |dll: &str, name: &str| {
                win32_rt::lookup(dll, name).unwrap_or_else(|| panic!("{dll}!{name}")) as usize
            };
            let f_tgt = exp("winmm.dll", "timeGetTime");
            let f_gtc = exp("kernel32.dll", "GetTickCount");
            let f_gtc64 = exp("kernel32.dll", "GetTickCount64");
            let f_enter = exp("kernel32.dll", "EnterCriticalSection");
            let f_leave = exp("kernel32.dll", "LeaveCriticalSection");
            let f_rtl_enter = exp("ntdll.dll", "RtlEnterCriticalSection");
            let f_rtl_leave = exp("ntdll.dll", "RtlLeaveCriticalSection");
            let f_xinput = exp("xinput1_4.dll", "XInputGetState");
            let f_xinput_ex = exp("xinput1_4.dll", "XInputGetStateEx");
            let f_switch = exp("kernel32.dll", "SwitchToThread");
            let f_yield = exp("msvcp140.dll", "_Thrd_yield");

            let h = arena_alloc(256, 16);
            unsafe { core::ptr::write_bytes(h, 0xCC, 256) };
            let vas: [u64; 11] = core::array::from_fn(|i| h as u64 + (i as u64) * 16);
            let fns = [
                f_tgt, f_gtc, f_gtc64, f_enter, f_leave, f_rtl_enter, f_rtl_leave, f_xinput,
                f_xinput_ex, f_switch, f_yield,
            ];
            for (&va, &f) in vas.iter().zip(fns.iter()) {
                crate::exe::register_native_va(va, f);
            }
            let [
                t_tgt,
                t_gtc,
                t_gtc64,
                t_enter,
                t_leave,
                t_rtl_enter,
                t_rtl_leave,
                t_xinput,
                t_xinput_ex,
                t_switch,
                t_yield,
            ] = vas;

            // Guest harness: fn(rcx = arg0, rdx = arg1, r8 = target_va, r9 = is_tail, [rsp+0x28] = sentinel_buf) -> rax
            // Loads XMM6/XMM15 and callee-saved GPR sentinels, calls or tail-jumps target_va(arg0, arg1),
            // and writes XMM6/XMM15 and callee-saved GPRs to sentinel_buf[0x20..].
            let c = arena_alloc(0x400, 16);
            let code = (|| -> Result<Vec<u8>, IcedError> {
                let mut a = CodeAssembler::new(64)?;
                let mut do_tail = a.create_label();
                let mut done = a.create_label();
                let mut tail_stub = a.create_label();
                a.push(rbx)?;
                a.push(rbp)?;
                a.push(rsi)?;
                a.push(rdi)?;
                a.push(r12)?;
                a.push(r13)?;
                a.push(r14)?;
                a.push(r15)?;
                a.sub(rsp, 0x48)?;
                // 8 pushes (0x40) + 0x48 = 0x88; Win64 5th arg was at caller [rsp+0x28] -> [rsp+0xB0]
                a.mov(rbx, qword_ptr(rsp + 0xB0))?;
                a.mov(rax, r8)?;
                a.mov(r10, r9)?;
                a.mov(rbp, 0x1122_3344_5566_7788u64)?;
                a.mov(rsi, 0x8877_6655_4433_2211u64)?;
                a.mov(rdi, 0xAAAA_BBBB_CCCC_DDDDu64)?;
                a.mov(r12, 0x1234_5678_9ABC_DEF0u64)?;
                a.mov(r13, 0x0FED_CBA9_8765_4321u64)?;
                a.mov(r14, 0x5555_AAAA_3333_CCCCu64)?;
                a.mov(r15, 0xCAFE_BABE_DEAD_BEEFu64)?;
                a.movdqu(xmm6, xmmword_ptr(rbx + 0x00))?;
                a.movdqu(xmm15, xmmword_ptr(rbx + 0x10))?;
                a.test(r10d, r10d)?;
                a.jne(do_tail)?;
                a.call(rax)?;
                a.jmp(done)?;
                a.set_label(&mut do_tail)?;
                a.call(tail_stub)?;
                a.set_label(&mut done)?;
                a.movdqu(xmmword_ptr(rbx + 0x20), xmm6)?;
                a.movdqu(xmmword_ptr(rbx + 0x30), xmm15)?;
                a.mov(qword_ptr(rbx + 0x40), rbp)?;
                a.mov(qword_ptr(rbx + 0x48), rsi)?;
                a.mov(qword_ptr(rbx + 0x50), rdi)?;
                a.mov(qword_ptr(rbx + 0x58), r12)?;
                a.mov(qword_ptr(rbx + 0x60), r13)?;
                a.mov(qword_ptr(rbx + 0x68), r14)?;
                a.mov(qword_ptr(rbx + 0x70), r15)?;
                a.add(rsp, 0x48)?;
                a.pop(r15)?;
                a.pop(r14)?;
                a.pop(r13)?;
                a.pop(r12)?;
                a.pop(rdi)?;
                a.pop(rsi)?;
                a.pop(rbp)?;
                a.pop(rbx)?;
                a.ret()?;
                a.set_label(&mut tail_stub)?;
                a.jmp(rax)?;
                a.assemble(c as u64)
            })()
            .expect("assemble");

            let sbuf = arena_alloc(0x100, 16);
            let want_gprs: [u64; 7] = [
                0x1122_3344_5566_7788,
                0x8877_6655_4433_2211,
                0xAAAA_BBBB_CCCC_DDDD,
                0x1234_5678_9ABC_DEF0,
                0x0FED_CBA9_8765_4321,
                0x5555_AAAA_3333_CCCC,
                0xCAFE_BABE_DEAD_BEEF,
            ];
            let plain = jit::plaincall_enabled();

            unsafe {
                core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
                let invoke = |target: u64, arg0: u64, arg1: u64, is_tail: u64| -> u64 {
                    core::ptr::write_bytes(sbuf, 0, 0x100);
                    for i in 0..0x20usize {
                        *sbuf.add(i) = (0x30 + i) as u8;
                    }
                    jit::force(1);
                    let (ret, _) = emu_run_full(
                        c as u64,
                        [arg0, arg1, target, is_tail, sbuf as u64, 0, 0, 0],
                        [0; 4],
                        [0; 8],
                    );
                    jit::force(0);
                    assert_eq!(
                        core::slice::from_raw_parts(sbuf.add(0x20), 0x20),
                        core::slice::from_raw_parts(sbuf, 0x20),
                        "XMM6/XMM15 clobbered (target={target:#x}, tail={is_tail})"
                    );
                    for (i, &want) in want_gprs.iter().enumerate() {
                        let got = core::ptr::read_unaligned(sbuf.add(0x40 + 8 * i) as *const u64);
                        assert_eq!(got, want, "GPR #{i} clobbered (target={target:#x}, tail={is_tail})");
                    }
                    ret
                };

                // --- 1. timeGetTime / GetTickCount / GetTickCount64 ---
                // Warm both call and tail-jump paths so `do_tail -> tail_stub` is compiled & linked.
                let m0 = invoke(t_tgt, 0, 0, 0);
                let _ = invoke(t_tgt, 0, 0, 1);
                let m1 = invoke(t_gtc, 0, 0, 0);
                let m2 = invoke(t_gtc64, 0, 0, 0);
                assert!(m0 <= u32::MAX as u64 && m1 <= u32::MAX as u64);
                assert!(m2 >= m0);
                let table = TLS_EMU.with(|cell| (*cell.get()).as_ref().map_or(0, |st| st.jit.test_table_ptr()));
                assert_ne!(table, 0);
                if plain {
                    let s0 = jit::test_stub_plain_hits();
                    let rm0 = jit::test_ras_moves();
                    let a0 = invoke(t_tgt, 0, 0, 0);
                    let a1 = invoke(t_tgt, 0, 0, 1);
                    let a2 = invoke(t_gtc, 0, 0, 0);
                    let a3 = invoke(t_gtc64, 0, 0, 1);
                    assert!(a1.wrapping_sub(a0) < 10_000);
                    assert!(a2.wrapping_sub(a1) < 10_000);
                    assert!(a3 >= a2);
                    assert_eq!(jit::test_stub_plain_hits() - s0, 4, "all 4 timeGetTime/GetTickCount calls hit inline stub");
                    assert_eq!(jit::test_ras_moves() - rm0, 0, "RAS balanced on timeGetTime tail jump");

                    // Deterministic 32-bit wraparound at 2^32 ms + 123 ms (`offset = ((1<<32)+123)*10_000 + 9999`).
                    let wrap_100ns = (((1u64 << 32) + 123) * 10_000) + 9999;
                    let local_wrap = QpcFastState {
                        mode: AtomicU64::new(1),
                        mult: AtomicU64::new(0),
                        offset: AtomicU64::new(wrap_100ns),
                    };
                    jit::JitCache::cache_fast_thunk(
                        table,
                        t_tgt,
                        jit::FAST_KIND_TIME_MS,
                        false,
                        &local_wrap as *const QpcFastState as usize,
                    );
                    jit::JitCache::cache_fast_thunk(
                        table,
                        t_gtc64,
                        jit::FAST_KIND_TIME_MS,
                        true,
                        &local_wrap as *const QpcFastState as usize,
                    );
                    assert_eq!(invoke(t_tgt, 0, 0, 0), 123, "timeGetTime must wrap mod 2^32 ms");
                    assert_eq!(invoke(t_tgt, 0, 0, 1), 123, "timeGetTime tail must wrap mod 2^32 ms");
                    assert_eq!(
                        invoke(t_gtc64, 0, 0, 0),
                        (1u64 << 32) + 123,
                        "GetTickCount64 must keep full 64-bit ms"
                    );
                    // mode != 1 falls back cleanly to slow path (`xfer_plain`).
                    let local_slow = QpcFastState {
                        mode: AtomicU64::new(2),
                        mult: AtomicU64::new(0),
                        offset: AtomicU64::new(0),
                    };
                    jit::JitCache::cache_fast_thunk(
                        table,
                        t_tgt,
                        jit::FAST_KIND_TIME_MS,
                        false,
                        &local_slow as *const QpcFastState as usize,
                    );
                    let x0 = super::test_xfer_plain();
                    let s_slow = jit::test_stub_plain_hits();
                    let real_ms = invoke(t_tgt, 0, 0, 0);
                    assert!(real_ms <= u32::MAX as u64);
                    assert_eq!(jit::test_stub_plain_hits() - s_slow, 0);
                    assert_eq!(super::test_xfer_plain() - x0, 1);
                }

                // --- 2. EnterCriticalSection / LeaveCriticalSection / Rtl* + TryEnterCriticalSection ---
                let mut crit: CriticalSection = core::mem::zeroed();
                InitializeCriticalSection(&mut crit);
                let cs_va = &mut crit as *mut CriticalSection as u64;
                // Warm caches (call #1 via xfer_plain).
                invoke(t_enter, cs_va, 0, 0);
                invoke(t_leave, cs_va, 0, 0);
                assert_eq!(invoke(t_rtl_enter, cs_va, 0, 0), 0);
                assert_eq!(invoke(t_rtl_leave, cs_va, 0, 0), 0);

                if plain {
                    let s0 = jit::test_stub_plain_hits();
                    let rm0 = jit::test_ras_moves();
                    // Acquire via EnterCriticalSection fast thunk -> check exact Windows CS layout.
                    invoke(t_enter, cs_va, 0, 0);
                    assert_eq!(crit.lock_count.load(Ordering::Acquire), 0);
                    assert_eq!(crit.recursion_count.load(Ordering::Acquire), 1);
                    assert_eq!(crit.owning_thread.load(Ordering::Acquire), my_tid);
                    // Nested acquire via RtlEnterCriticalSection tail jump.
                    assert_eq!(invoke(t_rtl_enter, cs_va, 0, 1), 0);
                    assert_eq!(crit.recursion_count.load(Ordering::Acquire), 2);
                    // Consistency with TryEnterCriticalSection on same thread -> recursion = 3.
                    assert_eq!(TryEnterCriticalSection(&mut crit), 1);
                    assert_eq!(crit.recursion_count.load(Ordering::Acquire), 3);
                    // Another thread calling TryEnterCriticalSection sees FALSE.
                    let cs_addr = cs_va as usize;
                    let other_try = std::thread::spawn(move || TryEnterCriticalSection(cs_addr as *mut CriticalSection))
                        .join()
                        .unwrap();
                    assert_eq!(other_try, 0);
                    // Unwind 3 -> 2 -> 1 -> 0 via LeaveCriticalSection & RtlLeaveCriticalSection.
                    invoke(t_leave, cs_va, 0, 0);
                    assert_eq!(crit.recursion_count.load(Ordering::Acquire), 2);
                    assert_eq!(crit.owning_thread.load(Ordering::Acquire), my_tid);
                    assert_eq!(invoke(t_rtl_leave, cs_va, 0, 1), 0);
                    assert_eq!(crit.recursion_count.load(Ordering::Acquire), 1);
                    invoke(t_leave, cs_va, 0, 1);
                    assert_eq!(crit.lock_count.load(Ordering::Acquire), -1);
                    assert_eq!(crit.recursion_count.load(Ordering::Acquire), 0);
                    assert_eq!(crit.owning_thread.load(Ordering::Acquire), 0);
                    assert_eq!(jit::test_stub_plain_hits() - s0, 5, "all 5 CS ops hit inline fast thunk");
                    assert_eq!(jit::test_ras_moves() - rm0, 0);

                    // Unaligned CS falls back cleanly to slow path (`xfer_plain`).
                    #[repr(align(16))]
                    struct RawCs([u8; 64]);
                    let mut raw_cs = RawCs([0u8; 64]);
                    let unal_cs = raw_cs.0.as_mut_ptr().add(3) as *mut CriticalSection;
                    InitializeCriticalSection(unal_cs);
                    let x0 = super::test_xfer_plain();
                    invoke(t_enter, unal_cs as u64, 0, 0);
                    invoke(t_leave, unal_cs as u64, 0, 0);
                    assert_eq!(super::test_xfer_plain() - x0, 2, "unaligned CS must fall back to slow path");
                }

                // --- 3. XInputGetState / XInputGetStateEx ---
                let mut st = State {
                    packet: 0x1357_2468,
                    gamepad: Gamepad { buttons: 0xBEEF, ..Default::default() },
                };
                xinput1_4::backend_disconnect(1);
                // Warm both targets (call #1 initializes backend + caches FAST_KIND_XINPUT).
                assert_eq!(invoke(t_xinput, 1, &mut st as *mut State as u64, 0), 1167);
                assert_eq!(invoke(t_xinput_ex, 1, &mut st as *mut State as u64, 0), 1167);
                assert_eq!(st.packet, 0x1357_2468, "disconnected slot must not overwrite state");
                assert_eq!(st.gamepad.buttons, 0xBEEF);

                if plain {
                    // Disconnected fast path (call + tail jump) with upper-32-bit garbage in rcx!
                    let s0 = jit::test_stub_plain_hits();
                    assert_eq!(
                        invoke(t_xinput, 0xDEAD_0000_0000_0001, &mut st as *mut State as u64, 0),
                        1167
                    );
                    assert_eq!(invoke(t_xinput, 1, &mut st as *mut State as u64, 1), 1167);
                    assert_eq!(jit::test_stub_plain_hits() - s0, 2);

                    // Connect slot 1 with Guide bit set and verify GUIDE masking & packet numbers.
                    xinput1_4::backend_connect(
                        1,
                        DeviceInfo { name: "jit-pad".into(), wireless: false, rumble: false, battery: None },
                    );
                    let pad = Gamepad {
                        buttons: btn::A | btn::X | btn::GUIDE,
                        left_trigger: 120,
                        right_trigger: 240,
                        thumb_lx: -10000,
                        thumb_ly: 20000,
                        thumb_rx: -30000,
                        thumb_ry: 15000,
                    };
                    xinput1_4::backend_update(1, pad);
                    let s1 = jit::test_stub_plain_hits();
                    let rm0 = jit::test_ras_moves();
                    assert_eq!(invoke(t_xinput, 1, &mut st as *mut State as u64, 0), 0);
                    let pkt = st.packet;
                    assert_ne!(pkt, 0);
                    assert_eq!(st.gamepad.buttons, btn::A | btn::X, "XInputGetState must clear GUIDE");
                    assert_eq!((st.gamepad.left_trigger, st.gamepad.right_trigger), (120, 240));
                    assert_eq!((st.gamepad.thumb_lx, st.gamepad.thumb_ly), (-10000, 20000));
                    assert_eq!((st.gamepad.thumb_rx, st.gamepad.thumb_ry), (-30000, 15000));
                    assert_eq!(invoke(t_xinput_ex, 1, &mut st as *mut State as u64, 1), 0);
                    assert_eq!(st.packet, pkt);
                    assert_eq!(
                        st.gamepad.buttons,
                        btn::A | btn::X | btn::GUIDE,
                        "XInputGetStateEx must keep GUIDE"
                    );
                    assert_eq!(jit::test_stub_plain_hits() - s1, 2);
                    assert_eq!(jit::test_ras_moves() - rm0, 0);

                    // Bad args (null pointer / user >= 4) bail cleanly to slow returning 160.
                    let x0 = super::test_xfer_plain();
                    assert_eq!(invoke(t_xinput, 1, 0, 0), 160);
                    assert_eq!(invoke(t_xinput, 4, &mut st as *mut State as u64, 0), 160);
                    assert_eq!(super::test_xfer_plain() - x0, 2);

                    // Local unready table (`ready == 0`) bails cleanly to slow path.
                    let unready: XInputFastTable = core::mem::zeroed();
                    jit::JitCache::cache_fast_thunk(
                        table,
                        t_xinput,
                        jit::FAST_KIND_XINPUT,
                        false,
                        &unready as *const XInputFastTable as usize,
                    );
                    let x1 = super::test_xfer_plain();
                    assert_eq!(invoke(t_xinput, 1, &mut st as *mut State as u64, 0), 0);
                    assert_eq!(super::test_xfer_plain() - x1, 1);

                    xinput1_4::backend_disconnect(1);
                }

                // --- 4. SwitchToThread / _Thrd_yield ---
                assert_eq!(invoke(t_switch, 0, 0, 0), 1); // call #1 via xfer_plain -> caches FAST_KIND_SWITCH_TO_THREAD
                invoke(t_yield, 0, 0, 0);
                if plain {
                    // Run 64 consecutive SwitchToThread calls inside a single guest loop (so
                    // `cpu.jx.switch_yield_ctr` persists across iterations, matching real guest
                    // spin-wait loops): 63 inline fast yields (FALSE) + 1 OS yield fallback (TRUE).
                    let loop_code = arena_alloc(0x100, 16);
                    let loop_bytes = (|| -> Result<Vec<u8>, IcedError> {
                        let mut a = CodeAssembler::new(64)?;
                        let mut lp = a.create_label();
                        let mut use_tail = a.create_label();
                        let mut next = a.create_label();
                        let mut tail_sw = a.create_label();
                        a.push(rbx)?;
                        a.push(rsi)?;
                        a.push(rdi)?;
                        a.push(r12)?;
                        a.sub(rsp, 0x28)?;
                        a.mov(rbx, rcx)?; // target = t_switch
                        a.mov(rsi, rdx)?; // count = 64
                        a.xor(edi, edi)?; // falses
                        a.xor(r12d, r12d)?; // trues
                        a.set_label(&mut lp)?;
                        a.mov(rax, rbx)?;
                        a.test(esi, 1u32)?;
                        a.jnz(use_tail)?;
                        a.call(rax)?;
                        a.jmp(next)?;
                        a.set_label(&mut use_tail)?;
                        a.call(tail_sw)?;
                        a.set_label(&mut next)?;
                        a.test(eax, eax)?;
                        a.sete(cl)?;
                        a.movzx(ecx, cl)?;
                        a.add(edi, ecx)?;
                        a.add(r12d, eax)?;
                        a.sub(rsi, 1)?;
                        a.jnz(lp)?;
                        // Pack (trues << 32) | falses in rax.
                        a.mov(eax, r12d)?;
                        a.shl(rax, 32)?;
                        a.or(rax, rdi)?;
                        a.add(rsp, 0x28)?;
                        a.pop(r12)?;
                        a.pop(rdi)?;
                        a.pop(rsi)?;
                        a.pop(rbx)?;
                        a.ret()?;
                        a.set_label(&mut tail_sw)?;
                        a.jmp(rax)?;
                        a.assemble(loop_code as u64)
                    })()
                    .expect("assemble switch loop");
                    core::ptr::copy_nonoverlapping(loop_bytes.as_ptr(), loop_code, loop_bytes.len());
                    core::ptr::write_bytes(loop_code.add(loop_bytes.len()), 0xCC, 16);

                    let s0 = jit::test_stub_plain_hits();
                    let x0 = super::test_xfer_plain();
                    jit::force(1);
                    let (packed, _) = emu_run_full(
                        loop_code as u64,
                        [t_switch, 64, 0, 0, 0, 0, 0, 0],
                        [0; 4],
                        [0; 8],
                    );
                    jit::force(0);
                    let falses = (packed & 0xFFFF_FFFF) as u32;
                    let trues = (packed >> 32) as u32;
                    assert_eq!(falses, 63, "63 of 64 SwitchToThread calls must take inline fast yield (FALSE)");
                    assert_eq!(trues, 1, "1 of 64 SwitchToThread calls must fall back to OS yield (TRUE)");
                    assert_eq!(jit::test_stub_plain_hits() - s0, 63);
                    assert_eq!(
                        super::test_xfer_plain() - x0,
                        64,
                        "63 inline stub hits + 1 slow xfer_plain OS yield = 64 total"
                    );
                }

                for &va in &vas {
                    crate::exe::unregister_native_va(va);
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Static `plain_call` fast thunks for `timeGetTime`, `EnterCriticalSection`,
/// `LeaveCriticalSection`, `XInputGetState`, and `SwitchToThread` must execute inline in
/// both Live JIT and Disk AOT (with `RELOC_QPC_STATE` and `RELOC_XINPUT_STATE` round-tripping
/// cleanly through `.aot` encode/decode/install) with zero `host_xfer` transitions.
#[test]
fn static_plain_call_fast_thunks_jit_and_aot() {
    use core::sync::atomic::Ordering;
    use win32_rt::misc::xinput1_4::{self, btn, DeviceInfo, Gamepad, State};
    use win32_rt::sync::{CriticalSection, InitializeCriticalSection};

    if !jit::plaincall_enabled() {
        return;
    }

    std::thread::Builder::new()
        .name("fast_thunks_static_aot".into())
        .spawn(|| {
            let _teb = win32_rt::teb::current();
            let my_tid = win32_rt::teb::thread_id() as usize;

            let exp = |dll: &str, name: &str| {
                win32_rt::lookup(dll, name).unwrap_or_else(|| panic!("{dll}!{name}")) as usize
            };
            let h = arena_alloc(128, 16);
            unsafe { core::ptr::write_bytes(h, 0xCC, 128) };
            let t_enter = h as u64;
            let t_leave = h as u64 + 16;
            let t_tgt = h as u64 + 32;
            let t_xinput = h as u64 + 48;
            let t_switch = h as u64 + 64;
            crate::exe::register_native_va(t_enter, exp("kernel32.dll", "EnterCriticalSection"));
            crate::exe::register_native_va(t_leave, exp("kernel32.dll", "LeaveCriticalSection"));
            crate::exe::register_native_va(t_tgt, exp("winmm.dll", "timeGetTime"));
            crate::exe::register_native_va(t_xinput, exp("xinput1_4.dll", "XInputGetState"));
            crate::exe::register_native_va(t_switch, exp("kernel32.dll", "SwitchToThread"));

            // Ensure XInput backend is started & slot 2 connected so static XInputGetState takes fast path.
            xinput1_4::backend_connect(
                2,
                DeviceInfo { name: "aot-pad".into(), wireless: false, rumble: false, battery: None },
            );
            xinput1_4::backend_update(
                2,
                Gamepad { buttons: btn::B | btn::GUIDE, left_trigger: 55, ..Default::default() },
            );
            let mut warmup_st = State::default();
            unsafe {
                assert_eq!(xinput1_4::XInputGetState(2, &mut warmup_st), 0);
            }

            // Guest function: fn(rcx = cs, rdx = xinput_state, r8 = out_buf) -> u64
            // Calls EnterCriticalSection(cs), captures [cs+0x08..0x18], calls timeGetTime(),
            // calls XInputGetState(2, xinput_state), calls SwitchToThread(), calls LeaveCriticalSection(cs).
            let c_jit = arena_alloc(0x400, 16);
            let c_aot = arena_alloc(0x400, 16);
            let assemble_at = |base: u64| -> (Vec<u8>, Vec<u64>) {
                let mut a = CodeAssembler::new(64).unwrap();
                let mut l1 = a.create_label();
                let mut l2 = a.create_label();
                let mut l3 = a.create_label();
                let mut l4 = a.create_label();
                let mut l5 = a.create_label();
                a.push(rbx).unwrap();
                a.push(rsi).unwrap();
                a.push(rdi).unwrap();
                a.sub(rsp, 0x20).unwrap();
                a.mov(rbx, rcx).unwrap(); // cs
                a.mov(rsi, rdx).unwrap(); // xinput_state
                a.mov(rdi, r8).unwrap();  // out_buf
                // EnterCriticalSection(cs)
                a.mov(rcx, rbx).unwrap();
                a.call(t_enter).unwrap();
                a.set_label(&mut l1).unwrap();
                // Snapshot LockCount/RecursionCount/OwningThread while held
                a.mov(eax, dword_ptr(rbx + 0x08)).unwrap();
                a.mov(dword_ptr(rdi + 0x00), eax).unwrap();
                a.mov(eax, dword_ptr(rbx + 0x0c)).unwrap();
                a.mov(dword_ptr(rdi + 0x04), eax).unwrap();
                a.mov(rax, qword_ptr(rbx + 0x10)).unwrap();
                a.mov(qword_ptr(rdi + 0x08), rax).unwrap();
                // timeGetTime()
                a.call(t_tgt).unwrap();
                a.set_label(&mut l2).unwrap();
                a.mov(qword_ptr(rdi + 0x10), rax).unwrap();
                // XInputGetState(2, xinput_state)
                a.mov(ecx, 2u32).unwrap();
                a.mov(rdx, rsi).unwrap();
                a.call(t_xinput).unwrap();
                a.set_label(&mut l3).unwrap();
                a.mov(qword_ptr(rdi + 0x18), rax).unwrap();
                // SwitchToThread()
                a.call(t_switch).unwrap();
                a.set_label(&mut l4).unwrap();
                a.mov(qword_ptr(rdi + 0x20), rax).unwrap();
                // LeaveCriticalSection(cs)
                a.mov(rcx, rbx).unwrap();
                a.call(t_leave).unwrap();
                a.set_label(&mut l5).unwrap();
                a.mov(eax, 0x1234_5678u32).unwrap();
                a.add(rsp, 0x20).unwrap();
                a.pop(rdi).unwrap();
                a.pop(rsi).unwrap();
                a.pop(rbx).unwrap();
                a.ret().unwrap();
                let res = a
                    .assemble_options(base, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)
                    .unwrap();
                let mut ips = vec![base];
                for lbl in [l1, l2, l3, l4, l5] {
                    ips.push(res.label_ip(&lbl).unwrap());
                }
                (res.inner.code_buffer, ips)
            };

            let (bytes_jit, _ips_jit) = assemble_at(c_jit as u64);
            let (bytes_aot, ips_aot) = assemble_at(c_aot as u64);
            unsafe {
                core::ptr::copy_nonoverlapping(bytes_jit.as_ptr(), c_jit, bytes_jit.len());
                core::ptr::write_bytes(c_jit.add(bytes_jit.len()), 0xCC, 16);
                core::ptr::copy_nonoverlapping(bytes_aot.as_ptr(), c_aot, bytes_aot.len());
                core::ptr::write_bytes(c_aot.add(bytes_aot.len()), 0xCC, 16);

                let check_run = |entry: u64, label: &str| {
                    let mut crit: CriticalSection = core::mem::zeroed();
                    InitializeCriticalSection(&mut crit);
                    let mut st = State::default();
                    let mut out = [0u64; 6];
                    let x0 = super::test_xfer_plain();
                    jit::force(1);
                    let (ret, _) = emu_run_full(
                        entry,
                        [
                            &mut crit as *mut CriticalSection as u64,
                            &mut st as *mut State as u64,
                            out.as_mut_ptr() as u64,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ],
                        [0; 4],
                        [0; 8],
                    );
                    jit::force(0);
                    assert_eq!(ret, 0x1234_5678, "{label}: bad return");
                    let lc = (out[0] & 0xFFFF_FFFF) as i32;
                    let rc = (out[0] >> 32) as i32;
                    assert_eq!(lc, 0, "{label}: LockCount while held");
                    assert_eq!(rc, 1, "{label}: RecursionCount while held");
                    assert_eq!(out[1] as usize, my_tid, "{label}: OwningThread while held");
                    assert!(out[2] <= u32::MAX as u64, "{label}: timeGetTime u32");
                    assert_eq!(out[3], 0, "{label}: XInputGetState ERROR_SUCCESS");
                    assert_eq!(st.gamepad.buttons, btn::B, "{label}: GUIDE masked");
                    assert_eq!(st.gamepad.left_trigger, 55, "{label}: trigger preserved");
                    assert!(out[4] <= 1, "{label}: SwitchToThread BOOL");
                    assert_eq!(crit.lock_count.load(Ordering::Acquire), -1, "{label}: CS unlocked");
                    assert_eq!(crit.recursion_count.load(Ordering::Acquire), 0, "{label}: CS rec=0");
                    assert_eq!(crit.owning_thread.load(Ordering::Acquire), 0, "{label}: CS owner=0");
                    assert_eq!(
                        super::test_xfer_plain() - x0,
                        0,
                        "{label}: static plain_call fast thunks must not fall through to xfer_plain"
                    );
                };

                // 1. Live JIT static plain_call execution.
                check_run(c_jit as u64, "Live JIT");

                // 2. Disk AOT build -> encode -> decode -> install -> execute.
                let mut bc = Box::new(BlockCache::new());
                let img = aot::build_image_for_test(&mut *bc, &ips_aot).expect("build AOT image");
                assert!(
                    img.relocs.iter().any(|r| r.kind == jit::RELOC_QPC_STATE),
                    "AOT image must record RELOC_QPC_STATE for inline timeGetTime"
                );
                assert!(
                    img.relocs.iter().any(|r| r.kind == jit::RELOC_XINPUT_STATE),
                    "AOT image must record RELOC_XINPUT_STATE for inline XInputGetState"
                );
                let key = aot::AotKey {
                    image_hash: [0x1111_2222_3333_4444, 0x5555_6666_7777_8888],
                    image_base: c_aot as u64,
                    image_size: 0x400,
                    codegen_hash: aot::codegen_hash(),
                    knobs: jit::aot_knob_bits(),
                };
                let bytes = aot::encode_for_test("fast_thunks_static.dll", &key, &img);
                let (_name, _k, dec, _fl) = aot::decode(&bytes).expect("decode AOT with RELOC_XINPUT_STATE");
                assert_eq!(dec.relocs.len(), img.relocs.len());

                let aot_before = jit::test_aot_installs();
                let (live, _relocs, _links) = aot::test_install(&ips_aot, || {});
                assert!(live > 0, "AOT must install block");
                check_run(c_aot as u64, "Disk AOT");
                aot::test_uninstall();
                assert!(jit::test_aot_installs() > aot_before, "AOT block must have executed");

                xinput1_4::backend_disconnect(2);
                for va in [t_enter, t_leave, t_tgt, t_xinput, t_switch] {
                    crate::exe::unregister_native_va(va);
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Multi-threaded contention stress test for JIT-translated `EnterCriticalSection` /
/// `TryEnterCriticalSection` / `LeaveCriticalSection` fast thunks (covering both inline
/// uncontended acquire/release, nested recursion, and contended slow-path handoff + wakeup).
#[test]
fn fast_thunk_critical_section_contention_stress() {
    use core::sync::atomic::Ordering;
    use std::sync::{Arc, Barrier};
    use win32_rt::sync::{CriticalSection, InitializeCriticalSection};

    let exp = |dll: &str, name: &str| {
        win32_rt::lookup(dll, name).unwrap_or_else(|| panic!("{dll}!{name}")) as usize
    };
    let h = arena_alloc(64, 16);
    unsafe { core::ptr::write_bytes(h, 0xCC, 64) };
    let t_enter = h as u64;
    let t_try = h as u64 + 16;
    let t_leave = h as u64 + 32;
    crate::exe::register_native_va(t_enter, exp("kernel32.dll", "EnterCriticalSection"));
    crate::exe::register_native_va(t_try, exp("kernel32.dll", "TryEnterCriticalSection"));
    crate::exe::register_native_va(t_leave, exp("kernel32.dll", "LeaveCriticalSection"));

    // Guest worker: fn(rcx = cs, rdx = counter_ptr, r8 = iters)
    // For each iteration: Enter(cs), nested Enter(cs) (or TryEnter(cs)), non-atomic *counter += 1,
    // Leave(cs), Leave(cs).
    let code_buf = arena_alloc(0x200, 16);
    let bytes = (|| -> Result<Vec<u8>, IcedError> {
        let mut a = CodeAssembler::new(64)?;
        let mut lp = a.create_label();
        let mut use_enter = a.create_label();
        let mut inner_ok = a.create_label();
        a.push(rbx)?;
        a.push(rsi)?;
        a.push(rdi)?;
        a.sub(rsp, 0x20)?;
        a.mov(rbx, rcx)?; // cs
        a.mov(rsi, rdx)?; // counter
        a.mov(rdi, r8)?;  // iters
        a.set_label(&mut lp)?;
        a.mov(rcx, rbx)?;
        a.call(t_enter)?;
        a.test(edi, 1u32)?;
        a.jz(use_enter)?;
        a.mov(rcx, rbx)?;
        a.call(t_try)?;
        a.jmp(inner_ok)?;
        a.set_label(&mut use_enter)?;
        a.mov(rcx, rbx)?;
        a.call(t_enter)?;
        a.set_label(&mut inner_ok)?;
        // Non-atomic read-modify-write inside critical section.
        a.mov(rax, qword_ptr(rsi))?;
        a.add(rax, 1)?;
        a.mov(qword_ptr(rsi), rax)?;
        a.mov(rcx, rbx)?;
        a.call(t_leave)?;
        a.mov(rcx, rbx)?;
        a.call(t_leave)?;
        a.sub(rdi, 1)?;
        a.jnz(lp)?;
        a.add(rsp, 0x20)?;
        a.pop(rdi)?;
        a.pop(rsi)?;
        a.pop(rbx)?;
        a.ret()?;
        a.assemble(code_buf as u64)
    })()
    .expect("assemble");
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), code_buf, bytes.len());
        core::ptr::write_bytes(code_buf.add(bytes.len()), 0xCC, 16);
    }

    #[repr(align(64))]
    struct Shared {
        cs: CriticalSection,
        counter: core::cell::UnsafeCell<u64>,
    }
    unsafe impl Sync for Shared {}
    let shared = Arc::new(Shared {
        cs: unsafe { core::mem::zeroed() },
        counter: core::cell::UnsafeCell::new(0),
    });
    unsafe { InitializeCriticalSection(&shared.cs as *const CriticalSection as *mut CriticalSection) };

    const THREADS: usize = 6;
    const ITERS: u64 = 2000;
    let barrier = Arc::new(Barrier::new(THREADS));
    let entry = code_buf as u64;
    let mut handles = Vec::with_capacity(THREADS);
    for _ in 0..THREADS {
        let s = Arc::clone(&shared);
        let b = Arc::clone(&barrier);
        handles.push(
            std::thread::Builder::new()
                .name("cs_stress".into())
                .spawn(move || {
                    let _teb = win32_rt::teb::current();
                    b.wait();
                    unsafe {
                        jit::force(1);
                        emu_run_full(
                            entry,
                            [
                                &s.cs as *const CriticalSection as u64,
                                s.counter.get() as u64,
                                ITERS,
                                0,
                                0,
                                0,
                                0,
                                0,
                            ],
                            [0; 4],
                            [0; 8],
                        );
                        jit::force(0);
                    }
                })
                .unwrap(),
        );
    }
    for jh in handles {
        jh.join().unwrap();
    }

    assert_eq!(unsafe { *shared.counter.get() }, (THREADS as u64) * ITERS);
    assert_eq!(shared.cs.lock_count.load(Ordering::Acquire), -1);
    assert_eq!(shared.cs.recursion_count.load(Ordering::Acquire), 0);
    assert_eq!(shared.cs.owning_thread.load(Ordering::Acquire), 0);

    for va in [t_enter, t_try, t_leave] {
        crate::exe::unregister_native_va(va);
    }
}

/// Difftest (translator vs interpreter) for per-site 2-way indirect-branch inline caches:
/// covers `call rax`, `call [rcx]`, `jmp rax`, `jmp [rcx]`, and `ret` (both RAS-on, RAS-desync
/// `push target; ret`, and RAS-off), across monomorphic steady state, 2-way polymorphic
/// alternation, 4-way megamorphic rotation, SMC invalidation (`mem::note_code_region`), and
/// `va_gen` flush invalidation, with both `PORT_JIT_IC=1` and `PORT_JIT_IC=0`.
#[test]
fn indirect_branch_ic_difftest_mono_poly_mega_and_invalidation() {
    use iced_x86::code_asm::*;

    // Allocate a guest buffer containing 4 distinct callees (t0..t3) plus a driver routine that
    // iterates over a caller-supplied array of function pointers in `[rcx + i*8]`:
    //   call rax       (CallR)
    //   call [r10]     (CallM)
    //   call helper_jr -> jmp rax      (JmpR)
    //   call helper_jm -> jmp [r10]    (JmpM)
    //   push target; ret               (Ret bypassing RAS -> ras_miss_ic / ret IC)
    let code_buf = arena_alloc(8192, 16);
    let base = code_buf as u64;

    let mut a = CodeAssembler::new(64).unwrap();
    let mut lbl_entry = a.create_label();
    let mut lbl_loop = a.create_label();
    let mut lbl_done = a.create_label();
    let mut lbl_after_pushret = a.create_label();
    let mut lbl_jr = a.create_label();
    let mut lbl_jm = a.create_label();
    let mut lbl_t0 = a.create_label();
    let mut lbl_t1 = a.create_label();
    let mut lbl_t2 = a.create_label();
    let mut lbl_t3 = a.create_label();

    // Entry(rcx = ptr_array, rdx = count) -> rax = accumulated checksum
    a.set_label(&mut lbl_entry).unwrap();
    a.push(rbx).unwrap();
    a.push(rsi).unwrap();
    a.push(rdi).unwrap();
    a.push(r12).unwrap();
    a.mov(rsi, rcx).unwrap();
    a.mov(rdi, rdx).unwrap();
    a.xor(ebx, ebx).unwrap(); // acc = 0
    a.xor(r12d, r12d).unwrap(); // i = 0
    a.set_label(&mut lbl_loop).unwrap();
    a.cmp(r12, rdi).unwrap();
    a.jge(lbl_done).unwrap();
    a.lea(r10, ptr(rsi + r12 * 8)).unwrap();
    a.mov(rax, qword_ptr(r10)).unwrap();
    // 1. CallR
    a.mov(rcx, rbx).unwrap();
    a.call(rax).unwrap();
    a.mov(rbx, rax).unwrap();
    // 2. CallM
    a.mov(rcx, rbx).unwrap();
    a.call(qword_ptr(r10)).unwrap();
    a.mov(rbx, rax).unwrap();
    // 3. JmpR via helper
    a.mov(rax, qword_ptr(r10)).unwrap();
    a.mov(rcx, rbx).unwrap();
    a.call(lbl_jr).unwrap();
    a.mov(rbx, rax).unwrap();
    // 4. JmpM via helper
    a.mov(rcx, rbx).unwrap();
    a.call(lbl_jm).unwrap();
    a.mov(rbx, rax).unwrap();
    // 5. Desynced push/ret trampoline to target (exercises ras_miss_ic / ret IC)
    a.lea(r11, ptr(lbl_after_pushret)).unwrap();
    a.push(r11).unwrap();
    a.mov(rax, qword_ptr(r10)).unwrap();
    a.mov(rcx, rbx).unwrap();
    a.push(rax).unwrap();
    a.ret().unwrap();
    a.set_label(&mut lbl_after_pushret).unwrap();
    a.mov(rbx, rax).unwrap();

    a.inc(r12).unwrap();
    a.jmp(lbl_loop).unwrap();
    a.set_label(&mut lbl_done).unwrap();
    a.mov(rax, rbx).unwrap();
    a.pop(r12).unwrap();
    a.pop(rdi).unwrap();
    a.pop(rsi).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();

    a.set_label(&mut lbl_jr).unwrap();
    a.jmp(rax).unwrap();
    a.set_label(&mut lbl_jm).unwrap();
    a.jmp(qword_ptr(r10)).unwrap();

    // 4 distinct pure guest callees: f_k(rcx) -> rax
    a.set_label(&mut lbl_t0).unwrap();
    a.lea(rax, ptr(rcx * 2 + 3)).unwrap();
    a.ret().unwrap();
    a.set_label(&mut lbl_t1).unwrap();
    a.lea(rax, ptr(rcx * 4 + 7)).unwrap();
    a.ret().unwrap();
    a.set_label(&mut lbl_t2).unwrap();
    a.mov(rax, rcx).unwrap();
    a.xor(rax, 0x5A5A).unwrap();
    a.add(rax, 11).unwrap();
    a.ret().unwrap();
    a.set_label(&mut lbl_t3).unwrap();
    a.mov(rax, rcx).unwrap();
    a.ror(rax, 3).unwrap();
    a.add(rax, 19).unwrap();
    a.ret().unwrap();

    let res = a
        .assemble_options(base, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)
        .unwrap();
    let bytes = &res.inner.code_buffer;
    let entry_ip = res.label_ip(&lbl_entry).unwrap();
    let t0 = res.label_ip(&lbl_t0).unwrap();
    let t1 = res.label_ip(&lbl_t1).unwrap();
    let t2 = res.label_ip(&lbl_t2).unwrap();
    let t3 = res.label_ip(&lbl_t3).unwrap();
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), code_buf, bytes.len());
        core::ptr::write_bytes(code_buf.add(bytes.len()), 0xCC, 32);
    }

    // Build mono, 2-way poly, and 4-way mega target schedules.
    let mono = [t0; 12];
    let poly = [t0, t1, t0, t1, t0, t1, t0, t1, t1, t0, t1, t0];
    let mega = [t0, t1, t2, t3, t0, t2, t1, t3, t3, t2, t1, t0];

    let run_sched = move |sched: &[u64]| -> u64 {
        let mut args = [0u64; 8];
        args[0] = sched.as_ptr() as u64;
        args[1] = sched.len() as u64;
        unsafe { emu_run_full(entry_ip, args, [0; 4], [0; 8]).0 }
    };

    // Reference interpreter outputs.
    jit::force(2);
    let ref_mono = run_sched(&mono);
    let ref_poly = run_sched(&poly);
    let ref_mega = run_sched(&mega);
    jit::force(0);

    // Verify translator matches interpreter across (ic_on, ras_on) × (mono, poly, mega) on fresh threads.
    for (ic_on, ras_on) in [(true, true), (true, false), (false, true)] {
        let h = std::thread::spawn(move || {
            jit::test_set_knobs(jit::TestKnobs { ic: Some(ic_on), ras: Some(ras_on), ..jit::TestKnobs::NONE });
            jit::force(1);
            let r_mono = run_sched(&mono);
            let r_poly = run_sched(&poly);
            let r_mega = run_sched(&mega);
            // Bump va_gen -> forces IC + cache flush; re-run must still match identically.
            crate::exe::invalidate_dispatch();
            let r_after_flush = run_sched(&poly);
            jit::force(0);
            jit::test_set_knobs(jit::TestKnobs::NONE);
            (r_mono, r_poly, r_mega, r_after_flush)
        });
        let (r_mono, r_poly, r_mega, r_after_flush) = h.join().unwrap();
        assert_eq!(r_mono, ref_mono, "mono mismatch (ic={ic_on}, ras={ras_on})");
        assert_eq!(r_poly, ref_poly, "poly mismatch (ic={ic_on}, ras={ras_on})");
        assert_eq!(r_mega, ref_mega, "mega mismatch (ic={ic_on}, ras={ras_on})");
        assert_eq!(r_after_flush, ref_poly, "post-flush mismatch (ic={ic_on}, ras={ras_on})");
    }

    // SMC invalidation: place an indirect target inside a tracked guest JIT/SMC region,
    // call it through `call rax`, rewrite its immediate in-place + bump `note_code_region`,
    // and verify IC never serves the stale version.
    let smc_page = arena_alloc(4096, 4096);
    let smc_target = smc_page as u64;
    let smc_caller = smc_page as u64 + 0x100;
    unsafe {
        // target v1: mov eax, 0x1111; ret
        core::ptr::copy_nonoverlapping([0xB8u8, 0x11, 0x11, 0x00, 0x00, 0xC3].as_ptr(), smc_page, 6);
        // caller: mov rax, rcx; call rax; add eax, 1; ret
        let mut ca = CodeAssembler::new(64).unwrap();
        ca.mov(rax, rcx).unwrap();
        ca.call(rax).unwrap();
        ca.add(eax, 1).unwrap();
        ca.ret().unwrap();
        let cb = ca.assemble(smc_caller).unwrap();
        core::ptr::copy_nonoverlapping(cb.as_ptr(), smc_page.add(0x100), cb.len());
    }
    let h_smc = std::thread::spawn(move || {
        jit::force(1);
        let r1 = unsafe { emu_run_full(smc_caller, [smc_target, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
        assert_eq!(r1, 0x1112);
        // Now mark page as dynamic SMC code and rewrite target v2: mov eax, 0x2222; ret
        unsafe {
            core::ptr::copy_nonoverlapping([0xB8u8, 0x22, 0x22, 0x00, 0x00, 0xC3].as_ptr(), smc_target as *mut u8, 6);
        }
        mem::note_code_region(smc_target as usize, smc_target as usize + 4096);
        let r2 = unsafe { emu_run_full(smc_caller, [smc_target, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
        assert_eq!(r2, 0x2223, "SMC target after note_code_region must not hit stale IC entry");
        // Rewrite again inside already-tracked SMC region (tests gen != 0 skip in emit_ic_populate).
        unsafe {
            core::ptr::copy_nonoverlapping([0xB8u8, 0x33, 0x33, 0x00, 0x00, 0xC3].as_ptr(), smc_target as *mut u8, 6);
        }
        mem::note_code_region(smc_target as usize, smc_target as usize + 4096);
        let r3 = unsafe { emu_run_full(smc_caller, [smc_target, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
        assert_eq!(r3, 0x3334, "second SMC rewrite must never be cached in IC");
        jit::force(0);
    });
    h_smc.join().unwrap();
}

/// Direct-linked hot conditional exits (`EXIT_JCC` elimination + hot `b.cond` direct linking
/// and clean unlinking on invalidation/SMC).
#[test]
fn direct_linked_jcc_host_and_guest_unlinking() {
    use iced_x86::code_asm::*;

    // Part A: `jcc near host_fn` must NOT emit `EXIT_JCC` — it runs inline (`plain_call`) or
    // via `host_xfer` and matches the interpreter bit-for-bit.
    unsafe extern "C" fn leaf_add(a: u64, b: u64) -> u64 {
        a.wrapping_mul(3).wrapping_add(b)
    }
    let buf = arena_alloc(4096, 16);
    let entry_ip = buf as u64;
    // Place `host_va` > 256 bytes away so `jne host_va` assembles as a 6-byte near Jcc (`0F 85 rel32`).
    let host_va = entry_ip + 0x400;
    crate::exe::register_native_va(host_va, leaf_add as *const () as usize);
    crate::emu::routes::test_set_extra_plain_flags(leaf_add as *const () as usize, DF_PLAIN);

    // cmp r8, 0; jne near host_va; mov rax, rcx; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.cmp(r8, 0).unwrap();
    a.jne(host_va).unwrap();
    a.mov(rax, rcx).unwrap();
    a.ret().unwrap();
    let code = a.assemble(entry_ip).unwrap();
    assert!(code.len() >= 10, "expected 6-byte near jne encoding, got len={}", code.len());
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), buf, code.len());
        core::ptr::write_bytes(buf.add(code.len()), 0xCC, 16);
    }

    jit::force(2);
    let interp_taken = unsafe { emu_run_full(entry_ip, [10, 5, 1, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
    let interp_fall = unsafe { emu_run_full(entry_ip, [10, 5, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
    assert_eq!(interp_taken, 35);
    assert_eq!(interp_fall, 10);
    jit::force(0);

    let h = std::thread::spawn(move || {
        jit::force(1);
        jit::test_dump_start();
        let t_taken = unsafe { emu_run_full(entry_ip, [10, 5, 1, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
        let t_fall = unsafe { emu_run_full(entry_ip, [10, 5, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
        let dumped = jit::test_dump_take();
        jit::force(0);
        (t_taken, t_fall, dumped)
    });
    let (t_taken, t_fall, dumped) = h.join().unwrap();
    assert_eq!(t_taken, interp_taken);
    assert_eq!(t_fall, interp_fall);
    // `movz w0, #EXIT_JCC (3)` = 0x5280_0060 must NEVER appear in the translated block!
    let want_exit_jcc = 0x5280_0000 | ((jit::EXIT_JCC as u32) << 5);
    assert!(!dumped.is_empty());
    for blk in &dumped {
        assert!(
            !blk.words.contains(&want_exit_jcc),
            "jcc near host target must not emit EXIT_JCC (block {:#x})",
            blk.ip
        );
    }
    crate::emu::routes::test_set_extra_plain_flags(leaf_add as *const () as usize, 0);
    crate::exe::unregister_native_va(host_va);

    // Part B: Direct-linked hot conditional branch (`b.ne`) to a guest target block in JitCache,
    // followed by `unlink_target` restoring both `cond_site` and `site` back to their fallbacks.
    let gbuf = arena_alloc(4096, 16);
    let g_entry = gbuf as u64;
    let g_target = g_entry + 0x100;
    unsafe {
        // g_entry: cmp rcx, 0; jne g_target; mov eax, 10; ret
        let mut ca = CodeAssembler::new(64).unwrap();
        ca.cmp(rcx, 0).unwrap();
        ca.jne(g_target).unwrap();
        ca.mov(eax, 10).unwrap();
        ca.ret().unwrap();
        let cb = ca.assemble(g_entry).unwrap();
        core::ptr::copy_nonoverlapping(cb.as_ptr(), gbuf, cb.len());

        // g_target v1: mov eax, 111; ret
        core::ptr::copy_nonoverlapping([0xB8u8, 111, 0, 0, 0, 0xC3].as_ptr(), gbuf.add(0x100), 6);
    }

    let mut bc = Box::new(BlockCache::new());
    let mut jc = jit::JitCache::new();
    unsafe {
        jc.warm();
        // Translate entry first (registers pending cond_site -> g_target), then translate g_target
        // (patches both cold `b` site AND hot `b.ne` directly to g_target's code!).
        let entry_code = jc.lookup(g_entry, &mut *bc)
            .or_else(|| jc.lookup(g_entry, &mut *bc))
            .expect("translate g_entry");
        let target_code = jc.lookup(g_target, &mut *bc)
            .or_else(|| jc.lookup(g_target, &mut *bc))
            .expect("translate g_target");

        // Verify hot `b.ne` inside `entry_code` now directly targets `target_code`!
        let mut found_direct_cond = false;
        for i in 0..32usize {
            let site = entry_code + i * 4;
            let w = *(site as *const u32);
            if (w & 0xFF00_0010) == 0x5400_0000 {
                let imm19 = ((((w >> 5) & 0x7_FFFF) as i32) << 13) >> 13;
                let dest = (site as i64 + (imm19 as i64) * 4) as usize;
                if dest == target_code {
                    found_direct_cond = true;
                    break;
                }
            }
        }
        assert!(found_direct_cond, "expected hot b.cond directly patched to target_code {target_code:#x}");

        // Execute with rcx = 1 (taken): must return 111 via the direct-linked b.ne.
        let mut cpu = Cpu::new_zeroed();
        let mut stack = [0u64; 64];
        stack[60] = RET_SENTINEL;
        cpu.gpr[4] = &stack[60] as *const u64 as u64;
        cpu.gpr[1] = 1;
        cpu.rip = g_entry;
        let _ = jc.run(&mut cpu, entry_code, 100);
        assert_eq!(cpu.gpr[0], 111);

        // Unlink `g_target`: hot `b.ne` must no longer point directly at `target_code`!
        let unlinked = jc.test_unlink_target(g_target);
        assert!(unlinked >= 1, "expected >=1 direct link unlinked for g_target");
        for i in 0..32usize {
            let site = entry_code + i * 4;
            let w = *(site as *const u32);
            if (w & 0xFF00_0010) == 0x5400_0000 {
                let imm19 = ((((w >> 5) & 0x7_FFFF) as i32) << 13) >> 13;
                let dest = (site as i64 + (imm19 as i64) * 4) as usize;
                assert_ne!(dest, target_code, "b.cond still pointed at invalidated target_code after unlink_target!");
            }
        }
        assert_eq!(jc.test_ic_set(g_entry), Some((0, 0, 0, 0)));
    }
}

/// AOT image with indirect branches (`call rax`, `call [rcx]`, `jmp rax`, `ret`) must emit
/// zero runtime relocations, build clean static Mach-O `.o` objects, and execute with exact
/// interpreter parity at both original base A and a shifted guest+host base B.
#[test]
fn aot_indirect_branch_ic_shifted_base_reload_and_static_macho() {
    use crate::emu::static_macho::{emit_macho_object_from_aot_bytes, MachoPlatform};
    use iced_x86::code_asm::*;

    let build_prog = |buf: *mut u8| -> (u64, u64, u64, Vec<u64>) {
        let base = buf as u64;
        let mut a = CodeAssembler::new(64).unwrap();
        let mut l_entry = a.create_label();
        let mut l_jmp_tramp = a.create_label();
        let mut l_f0 = a.create_label();
        let mut l_f1 = a.create_label();

        // Entry(rcx = fn_ptr_slot_addr, rdx = seed) -> rax
        a.set_label(&mut l_entry).unwrap();
        a.push(rbx).unwrap();
        a.push(rsi).unwrap();
        a.mov(rsi, rcx).unwrap();
        a.mov(rbx, rdx).unwrap();
        // Indirect memory call
        a.mov(rcx, rbx).unwrap();
        a.call(qword_ptr(rsi)).unwrap();
        a.mov(rbx, rax).unwrap();
        // Indirect register call to second slot
        a.mov(rax, qword_ptr(rsi + 8)).unwrap();
        a.mov(rcx, rbx).unwrap();
        a.call(rax).unwrap();
        a.mov(rbx, rax).unwrap();
        // Indirect tail jump via tramp
        a.mov(rax, qword_ptr(rsi)).unwrap();
        a.mov(rcx, rbx).unwrap();
        a.call(l_jmp_tramp).unwrap();
        a.pop(rsi).unwrap();
        a.pop(rbx).unwrap();
        a.ret().unwrap();

        a.set_label(&mut l_jmp_tramp).unwrap();
        a.jmp(rax).unwrap();

        a.set_label(&mut l_f0).unwrap();
        a.lea(rax, ptr(rcx * 2 + 5)).unwrap();
        a.ret().unwrap();

        a.set_label(&mut l_f1).unwrap();
        a.lea(rax, ptr(rcx * 4 + 11)).unwrap();
        a.ret().unwrap();

        let enc = a
            .assemble_options(base, iced_x86::BlockEncoderOptions::RETURN_NEW_INSTRUCTION_OFFSETS)
            .unwrap();
        let bytes = &enc.inner.code_buffer;
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
            core::ptr::write_bytes(buf.add(bytes.len()), 0xCC, 32);
        }
        let entry = enc.label_ip(&l_entry).unwrap();
        let f0 = enc.label_ip(&l_f0).unwrap();
        let f1 = enc.label_ip(&l_f1).unwrap();
        let mut ips = vec![entry, enc.label_ip(&l_jmp_tramp).unwrap(), f0, f1];
        // Also collect every call fallthrough IP so 100% of blocks are AOT-compiled.
        let mut dec = iced_x86::Decoder::with_ip(64, &bytes, base, iced_x86::DecoderOptions::NONE);
        while dec.can_decode() {
            let ins = dec.decode();
            if matches!(
                ins.flow_control(),
                iced_x86::FlowControl::Call | iced_x86::FlowControl::IndirectCall
            ) {
                ips.push(ins.next_ip());
            }
        }
        ips.retain(|&ip| ip >= base && ip < base + bytes.len() as u64);
        ips.sort_unstable();
        ips.dedup();
        (entry, f0, f1, ips)
    };

    let code_a = arena_alloc(4096, 16);
    let code_b = arena_alloc(4096, 16);
    assert_ne!(code_a, code_b);
    let (entry_a, f0_a, f1_a, ips_a) = build_prog(code_a);
    let (entry_b, f0_b, f1_b, ips_b) = build_prog(code_b);

    // Build v8 AOT image and verify zero relocations + static Mach-O emission.
    let mut bc = Box::new(BlockCache::new());
    let img = unsafe { aot::build_image_for_test(&mut *bc, &ips_a) }.expect("build v8 IC AOT image");
    assert!(img.relocs.is_empty(), "IC AOT blocks must have zero runtime relocs");
    let key = aot::AotKey {
        image_hash: [0x1122_3344_5566_7788, 0x99AA_BBCC_DDEE_FF00],
        image_base: code_a as u64,
        image_size: 4096,
        codegen_hash: aot::codegen_hash(),
        knobs: jit::aot_knob_bits(),
    };
    let aot_bytes = aot::encode_for_test("ic_shifted_test.dll", &key, &img);
    let (_, _, dec_img, _) = aot::decode(&aot_bytes).expect("decode v8 IC AOT");
    assert_eq!(dec_img.stubs, img.stubs);
    for plat in [MachoPlatform::MacOs, MachoPlatform::Ios] {
        let (_obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
            .unwrap_or_else(|e| panic!("static Mach-O emit ({plat}) failed: {e}"));
        assert_eq!(rep.skipped_reloc_blocks, 0);
        assert_eq!(rep.emitted_blocks, img.blocks.len());
    }

    // Reference interpreter result.
    let slots_a = [f0_a, f1_a];
    let slots_b = [f0_b, f1_b];
    jit::force(2);
    let want = unsafe { emu_run_full(entry_a, [slots_a.as_ptr() as u64, 7, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
    jit::force(0);

    // Run AOT at original base A AND shifted guest+host base B on a fresh thread.
    let h = std::thread::spawn(move || {
        for (entry, slots, ips, label) in [
            (entry_a, slots_a, ips_a, "base A"),
            (entry_b, slots_b, ips_b, "shifted base B"),
        ] {
            let before = jit::test_aot_installs();
            let (live, relocs, _links) = unsafe { aot::test_install(&ips, || {}) };
            assert!(live >= 4, "expected >=4 live AOT blocks at {label}");
            assert_eq!(relocs, 0, "expected 0 relocs at {label}");
            jit::force(1);
            // Run twice so pass 2 hits the warm IC!
            let r1 = unsafe { emu_run_full(entry, [slots.as_ptr() as u64, 7, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
            let r2 = unsafe { emu_run_full(entry, [slots.as_ptr() as u64, 7, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
            jit::force(0);
            aot::test_uninstall();
            assert!(jit::test_aot_installs() > before, "AOT must install at {label}");
            assert_eq!(r1, want, "AOT pass 1 mismatch at {label}");
            assert_eq!(r2, want, "AOT pass 2 (warm IC) mismatch at {label}");
        }
    });
    h.join().unwrap();
}

/// Widened `xfer_plain` host-transfer ABI tests:
/// verifies int 1..8 (preserving `xmm0`), XMM args + XMM return (`DF_PLAIN | DF_PLAIN_XMM`),
/// wide stack args 9..16 (`DF_PLAIN | DF_PLAIN_STACK`), wide stack + XMM
/// (`DF_PLAIN | DF_PLAIN_XMM | DF_PLAIN_STACK`), and newly classified Win32/CRT leaf builtins
/// (`XInputGetState`, `HeapAlloc`/`HeapFree`, `sinf`), matching interpreter vs translator on
/// both indirect `call rax` and tail `jmp rax`.
#[test]
fn xfer_plain_widened_abi_shapes_int_xmm_stack_and_bail() {
    use iced_x86::code_asm::*;

    unsafe extern "C" fn fn_int8(
        a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64, a8: u64,
    ) -> u64 {
        a1.wrapping_add(a2 * 2)
            .wrapping_add(a3 * 3)
            .wrapping_add(a4 * 4)
            .wrapping_add(a5 * 5)
            .wrapping_add(a6 * 6)
            .wrapping_add(a7 * 7)
            .wrapping_add(a8 * 8)
    }

    unsafe extern "C" fn fn_xmm4(x0: f64, x1: f64, x2: f64, x3: f64) -> f64 {
        x0 * 2.0 + x1 * 3.0 + x2 * 5.0 + x3 * 7.0
    }

    unsafe extern "C" fn fn_wide16(
        a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64, a8: u64,
        a9: u64, a10: u64, a11: u64, a12: u64, a13: u64, a14: u64, a15: u64, a16: u64,
    ) -> u64 {
        let mut s = 0u64;
        for (i, v) in [a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16]
            .into_iter()
            .enumerate()
        {
            s = s.wrapping_add(v.wrapping_mul((i as u64) + 1));
        }
        s
    }

    unsafe extern "C" fn fn_wide16_xmm(
        a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64, a8: u64,
        a9: u64, a10: u64, a11: u64, a12: u64, a13: u64, a14: u64, a15: u64, a16: u64,
        x0: f64, x1: f64, x2: f64, x3: f64,
    ) -> f64 {
        let isum = a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12 + a13 + a14 + a15 + a16;
        (isum as f64) + x0 + x1 * 2.0 + x2 * 3.0 + x3 * 4.0
    }

    let va_int8 = arena_alloc(32, 16) as u64;
    let va_xmm4 = arena_alloc(32, 16) as u64;
    let va_wide16 = arena_alloc(32, 16) as u64;
    let va_wide16_xmm = arena_alloc(32, 16) as u64;

    let p_int8 = fn_int8 as *const () as usize;
    let p_xmm4 = fn_xmm4 as *const () as usize;
    let p_wide16 = fn_wide16 as *const () as usize;
    let p_wide16_xmm = fn_wide16_xmm as *const () as usize;

    crate::exe::register_native_va(va_int8, p_int8);
    crate::exe::register_native_va(va_xmm4, p_xmm4);
    crate::exe::register_native_va(va_wide16, p_wide16);
    crate::exe::register_native_va(va_wide16_xmm, p_wide16_xmm);
    crate::emu::routes::test_set_extra_plain_flags(p_int8, DF_PLAIN);
    crate::emu::routes::test_set_extra_plain_flags(p_xmm4, DF_PLAIN | DF_PLAIN_XMM);
    crate::emu::routes::test_set_extra_plain_flags(p_wide16, DF_PLAIN | DF_PLAIN_STACK);
    crate::emu::routes::test_set_extra_plain_flags(p_wide16_xmm, DF_PLAIN | DF_PLAIN_XMM | DF_PLAIN_STACK);

    // Also verify real Win32/CRT leaf builtins are classified with DF_PLAIN / DF_PLAIN_XMM.
    if let Some(xi) = win32_rt::lookup("xinput1_4.dll", "XInputGetState") {
        let va_xi = arena_alloc(32, 16) as u64;
        crate::exe::register_native_va(va_xi, xi as usize);
        let (_, fl) = resolve_dispatch(va_xi);
        assert_ne!(fl & DF_PLAIN, 0, "XInputGetState must have DF_PLAIN");
        crate::exe::unregister_native_va(va_xi);
    }
    if let Some(ha) = win32_rt::lookup("kernel32.dll", "HeapAlloc") {
        let va_ha = arena_alloc(32, 16) as u64;
        crate::exe::register_native_va(va_ha, ha as usize);
        let (_, fl) = resolve_dispatch(va_ha);
        assert_ne!(fl & DF_PLAIN, 0, "HeapAlloc must have DF_PLAIN");
        crate::exe::unregister_native_va(va_ha);
    }
    if let Some(sf) = win32_rt::lookup("ucrtbase.dll", "sinf") {
        let va_sf = arena_alloc(32, 16) as u64;
        crate::exe::register_native_va(va_sf, sf as usize);
        let (_, fl) = resolve_dispatch(va_sf);
        assert_eq!(fl & (DF_PLAIN | DF_PLAIN_XMM), DF_PLAIN | DF_PLAIN_XMM, "sinf must have DF_PLAIN | DF_PLAIN_XMM");
        crate::exe::unregister_native_va(va_sf);
    }

    // Build two tiny guest wrappers that invoke `[r11]` (passed in `rax` -> `call rax` and `jmp rax`)
    // preserving all 16 Win64 args (`rcx, rdx, r8, r9`, `[rsp+0x20..0x80]`) and `xmm0..3`.
    let wbuf = arena_alloc(4096, 16);
    let call_wrap = wbuf as u64;
    let tail_wrap = wbuf as u64 + 0x100;
    unsafe {
        // tail_wrap: target is in `[r10]`, jump straight to it (`jmp rax`) with untouched stack!
        // Caller passes target in `rax` via `emu_run_full_rax`, or simpler: put target in a global slot.
        let mut a = CodeAssembler::new(64).unwrap();
        // tail_wrap: mov rax, qword ptr [rip + slot]; jmp rax
        a.mov(rax, qword_ptr(wbuf as u64 + 0x200)).unwrap();
        a.jmp(rax).unwrap();
        let tb = a.assemble(tail_wrap).unwrap();
        core::ptr::copy_nonoverlapping(tb.as_ptr(), wbuf.add(0x100), tb.len());

        // call_wrap: copy stack args 5..16 from incoming `[rsp+0x28..0x88]` into outgoing frame,
        // call `qword ptr [slot]`, restore rsp, ret.
        let mut b = CodeAssembler::new(64).unwrap();
        b.sub(rsp, 0x88).unwrap();
        for i in 0..12i32 {
            b.mov(rax, qword_ptr(rsp + 0x88 + 0x28 + i * 8)).unwrap();
            b.mov(qword_ptr(rsp + 0x20 + i * 8), rax).unwrap();
        }
        b.mov(rax, qword_ptr(wbuf as u64 + 0x200)).unwrap();
        b.call(rax).unwrap();
        b.add(rsp, 0x88).unwrap();
        b.ret().unwrap();
        let cb = b.assemble(call_wrap).unwrap();
        core::ptr::copy_nonoverlapping(cb.as_ptr(), wbuf, cb.len());
    }
    let slot_ptr = unsafe { wbuf.add(0x200) as *mut u64 };

    let gpr = [1u64, 2, 3, 4, 5, 6, 7, 8];
    let stk = [9u64, 10, 11, 12, 13, 14, 15, 16];
    let xmm = [
        1.5f64.to_bits() as u128 | (0xDEAD_BEEFu128 << 64),
        2.25f64.to_bits() as u128,
        3.5f64.to_bits() as u128,
        4.75f64.to_bits() as u128,
    ];

    let want_int8 = unsafe { fn_int8(1, 2, 3, 4, 5, 6, 7, 8) };
    let want_xmm4 = unsafe { fn_xmm4(1.5, 2.25, 3.5, 4.75) }.to_bits() as u128;
    let want_wide16 = unsafe { fn_wide16(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16) };
    let want_wide16_xmm = unsafe {
        fn_wide16_xmm(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1.5, 2.25, 3.5, 4.75)
    }
    .to_bits() as u128;

    let slot_addr = slot_ptr as usize;
    let h = std::thread::spawn(move || {
        for (wrap, wname) in [(call_wrap, "call rax"), (tail_wrap, "jmp rax")] {
            for (va, label, check_rax, check_xmm) in [
                (va_int8, "int8", Some(want_int8), Some(xmm[0])),
                (va_xmm4, "xmm4", None, Some(want_xmm4)),
                (va_wide16, "wide16", Some(want_wide16), Some(xmm[0])),
                (va_wide16_xmm, "wide16_xmm", None, Some(want_wide16_xmm)),
            ] {
                unsafe { *(slot_addr as *mut u64) = va };
                jit::force(2);
                let (ir, ix) = unsafe { emu_run_full(wrap, gpr, xmm, stk) };
                jit::force(1);
                let xp0 = test_xfer_plain();
                let (jr1, jx1) = unsafe { emu_run_full(wrap, gpr, xmm, stk) };
                let (jr2, jx2) = unsafe { emu_run_full(wrap, gpr, xmm, stk) };
                let xp1 = test_xfer_plain();
                jit::force(0);

                if let Some(wr) = check_rax {
                    assert_eq!(ir, wr, "interp {wname}/{label} rax");
                    assert_eq!(jr1, wr, "jit pass1 {wname}/{label} rax");
                    assert_eq!(jr2, wr, "jit pass2 {wname}/{label} rax");
                }
                if let Some(wx) = check_xmm {
                    assert_eq!(ix, wx, "interp {wname}/{label} xmm0");
                    assert_eq!(jx1, wx, "jit pass1 {wname}/{label} xmm0");
                    assert_eq!(jx2, wx, "jit pass2 {wname}/{label} xmm0");
                }
                assert!(
                    xp1 >= xp0 + 2,
                    "{wname}/{label}: expected both passes served by widened xfer_plain/stub (delta={})",
                    xp1 - xp0
                );
            }
        }
    });
    h.join().unwrap();

    for p in [p_int8, p_xmm4, p_wide16, p_wide16_xmm] {
        crate::emu::routes::test_set_extra_plain_flags(p, 0);
    }
    for va in [va_int8, va_xmm4, va_wide16, va_wide16_xmm] {
        crate::exe::unregister_native_va(va);
    }
}

/// Cross-block pinned `ymm_hi[0..7]` in `v8..v15`, coalesced `ymm_dirty`, direct 256-bit
/// `vmovups`/`vfmadd213ps`/`vaddps`/`vminps`/`vpmovmskb`, `vzeroupper`, and preservation across
/// an inline `DF_PLAIN` host call (`spill_xmm`/`reload_xmm`) must match the interpreter bit-for-bit.
#[test]
fn ymm_hi_pinned_across_blocks_fma_movmsk_and_host_calls() {
    use iced_x86::code_asm::*;

    extern "win64" fn plain_add_u64(a: u64, b: u64) -> u64 {
        a.wrapping_add(b).wrapping_mul(3)
    }
    let host_va = 0x7FF9_9123_0000u64;
    let host_ptr = plain_add_u64 as *const () as usize;
    crate::exe::register_native_va(host_va, host_ptr);
    crate::emu::routes::test_set_extra_plain_flags(host_ptr, crate::emu::DF_PLAIN);

    let mut code = vec![0u8; 4096];
    let base = code.as_ptr() as u64;
    let mut mem = vec![0u8; 4096];
    // Populate 256-bit float arrays and byte pattern for vpmovmskb.
    for i in 0..64usize {
        let f = (i as f32) * 0.25 - 4.5;
        mem[i * 4..i * 4 + 4].copy_from_slice(&f.to_le_bytes());
    }
    for i in 0..32usize {
        mem[512 + i] = if i % 3 == 0 || i == 31 || i == 15 { 0x80 } else { 0x7F };
    }

    let mut a = CodeAssembler::new(64).unwrap();
    let mut lp = a.create_label();
    // rcx = mem_base
    a.vmovups(ymm4, ymmword_ptr(rcx + 128)).unwrap(); // loop-invariant pinned ymm_hi[4]
    a.vmovups(ymm9, ymmword_ptr(rcx + 160)).unwrap(); // unpinned ymm_hi[9]
    a.mov(r8d, 4).unwrap();
    a.set_label(&mut lp).unwrap();
    a.vmovups(ymm0, ymmword_ptr(rcx)).unwrap();
    a.vmovups(ymm1, ymmword_ptr(rcx + 32)).unwrap();
    a.vfmadd213ps(ymm0, ymm4, ymmword_ptr(rcx + 64)).unwrap();
    a.vfmadd213ps(ymm1, ymm9, ymmword_ptr(rcx + 96)).unwrap();
    a.vaddps(ymm2, ymm0, ymm1).unwrap();
    a.vminps(ymm3, ymm2, ymm4).unwrap();
    a.vmovups(ymmword_ptr(rcx + 256), ymm3).unwrap();
    a.dec(r8d).unwrap();
    a.jne(lp).unwrap();
    // Call a DF_PLAIN host function (must spill/reload pinned v8..v15 without losing ymm0..4!)
    a.push(rcx).unwrap();
    a.sub(rsp, 0x20).unwrap();
    a.mov(rcx, 11u64).unwrap();
    a.mov(rdx, 29u64).unwrap();
    a.mov(rax, host_va).unwrap();
    a.call(rax).unwrap();
    a.add(rsp, 0x20).unwrap();
    a.pop(rcx).unwrap();
    // Verify ymm0 and ymm4 upper halves survived the host call + test vpmovmskb 256-bit and vzeroupper.
    a.vaddps(ymm5, ymm0, ymm4).unwrap();
    a.vmovups(ymmword_ptr(rcx + 288), ymm5).unwrap();
    a.vmovdqu(ymm6, ymmword_ptr(rcx + 512)).unwrap();
    a.vpmovmskb(r9d, ymm6).unwrap();
    a.xor(rax, r9).unwrap();
    a.vzeroupper().unwrap();
    // Verify vzeroupper cleared ymm_hi[0..7] & ymm_hi[8..15] by storing ymm0 and ymm9 back to memory.
    a.vmovups(ymmword_ptr(rcx + 320), ymm0).unwrap();
    a.vmovups(ymmword_ptr(rcx + 352), ymm9).unwrap();
    a.ret().unwrap();
    let bytes = a.assemble(base).unwrap();
    code[..bytes.len()].copy_from_slice(&bytes);

    let run = |mode: u8| -> (u64, Vec<u8>) {
        let mut m = mem.clone();
        jit::force(mode);
        let r = unsafe { crate::emu::call_va8(base, m.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0) };
        jit::force(0);
        (r, m)
    };

    let (ir, im) = run(2);
    let (jr, jm) = run(1);
    crate::emu::routes::test_set_extra_plain_flags(host_ptr, 0);
    crate::exe::unregister_native_va(host_va);

    assert_eq!(jr, ir, "rax mismatch (vpmovmskb + plain_call)");
    assert_eq!(&jm[256..384], &im[256..384], "stored 256-bit YMM outputs mismatch");
    // Ensure vzeroupper actually cleared upper 128 bits of both pinned ymm0 and unpinned ymm9!
    assert_eq!(&jm[336..352], &[0u8; 16], "vzeroupper must clear pinned ymm_hi[0]");
    assert_eq!(&jm[368..384], &[0u8; 16], "vzeroupper must clear unpinned ymm_hi[9]");
}

/// Dead-flag elision across `FlowControl::Next` fallthrough, non-zero `K_SHIFT_RI`, and
/// Win64 host tail jumps must match the interpreter on both registers and live flags.
#[test]
fn dead_flag_elision_across_fallthrough_shift_and_host_jmp() {
    use iced_x86::code_asm::*;

    extern "win64" fn plain_tail_fn(a: u64) -> u64 {
        a ^ 0x55AA_1234
    }
    let host_va = 0x7FF9_9124_0000u64;
    let host_ptr = plain_tail_fn as *const () as usize;
    crate::exe::register_native_va(host_va, host_ptr);
    crate::emu::routes::test_set_extra_plain_flags(host_ptr, crate::emu::DF_PLAIN);

    let mut code = vec![0u8; 4096];
    let base = code.as_ptr() as u64;
    let mut a = CodeAssembler::new(64).unwrap();
    let mut next_blk = a.create_label();
    // Block 1: ends in jmp next_blk after an add whose flags are killed by `shl eax, 3` in block 2.
    a.mov(eax, ecx).unwrap();
    a.add(eax, edx).unwrap();
    a.jmp(next_blk).unwrap();
    // Block 2: starts with non-zero immediate shift (kills all flags), then computes flags for setb.
    a.set_label(&mut next_blk).unwrap();
    a.shl(eax, 3).unwrap();
    a.sub(eax, r8d).unwrap();
    a.setb(cl).unwrap();
    a.movzx(ecx, cl).unwrap();
    a.add(rcx, rax).unwrap();
    // Tail-jump to Win64 host function (flags dead on entry!).
    a.jmp(host_va).unwrap();
    let bytes = a.assemble(base).unwrap();
    code[..bytes.len()].copy_from_slice(&bytes);

    let run = |mode: u8, ia: u64, ib: u64, ic: u64| -> u64 {
        jit::force(mode);
        let r = unsafe { crate::emu::call_va8(base, ia, ib, ic, 0, 0, 0, 0, 0) };
        jit::force(0);
        r
    };

    for &(ia, ib, ic) in &[(10u64, 5u64, 200u64), (100, 3, 50), (0, 0, 1)] {
        assert_eq!(run(1, ia, ib, ic), run(2, ia, ib, ic), "mismatch for ({ia},{ib},{ic})");
    }
    crate::emu::routes::test_set_extra_plain_flags(host_ptr, 0);
    crate::exe::unregister_native_va(host_va);
}

/// Regression test: 256-bit `vminps`/`vmaxpd` writing to unpinned `ymm8..15` (and all
/// destinations when `xmm_pin` is off) with `MXCSR.DAZ` enabled must not alias
/// `flush_denorm_lanes`'s `v3`/`v4` scratch registers.
#[test]
fn ymm8_15_256bit_minmax_with_daz_matches_interpreter() {
    use iced_x86::code_asm::*;

    let mut code = vec![0u8; 4096];
    let base = code.as_ptr() as u64;
    let mut mem = vec![0u8; 512];
    // 8 x f32 in [0..32] and [32..64] (mix of finite values and subnormals in upper 128b).
    let a_f32: [u32; 8] = [
        1.5f32.to_bits(),
        (-2.0f32).to_bits(),
        0x0000_0042, // +subnormal
        3.25f32.to_bits(),
        4.5f32.to_bits(),
        0x8000_0015, // -subnormal
        (-7.0f32).to_bits(),
        9.0f32.to_bits(),
    ];
    let b_f32: [u32; 8] = [
        2.0f32.to_bits(),
        (-1.0f32).to_bits(),
        1.0f32.to_bits(),
        (-0.5f32).to_bits(),
        2.25f32.to_bits(),
        0.5f32.to_bits(),
        (-3.0f32).to_bits(),
        12.0f32.to_bits(),
    ];
    for (i, &w) in a_f32.iter().enumerate() {
        mem[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    for (i, &w) in b_f32.iter().enumerate() {
        mem[32 + i * 4..32 + i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    // 4 x f64 in [64..96] and [96..128] (finite + subnormal in upper 128b).
    let a_f64: [u64; 4] = [
        1.25f64.to_bits(),
        (-3.5f64).to_bits(),
        0x0000_0000_0000_0077, // +subnormal
        6.5f64.to_bits(),
    ];
    let b_f64: [u64; 4] = [
        0.5f64.to_bits(),
        (-4.0f64).to_bits(),
        (-1.0f64).to_bits(),
        8.25f64.to_bits(),
    ];
    for (i, &w) in a_f64.iter().enumerate() {
        mem[64 + i * 8..64 + i * 8 + 8].copy_from_slice(&w.to_le_bytes());
    }
    for (i, &w) in b_f64.iter().enumerate() {
        mem[96 + i * 8..96 + i * 8 + 8].copy_from_slice(&w.to_le_bytes());
    }
    // MXCSR with DAZ (bit 6 = 0x40) + default masked exceptions (0x1F80) at [256..260],
    // original MXCSR save slot at [260..264].
    mem[256..260].copy_from_slice(&0x1FC0u32.to_le_bytes());

    let mut a = CodeAssembler::new(64).unwrap();
    a.stmxcsr(dword_ptr(rcx + 260)).unwrap();
    a.ldmxcsr(dword_ptr(rcx + 256)).unwrap();
    a.vmovups(ymm8, ymmword_ptr(rcx)).unwrap();
    a.vmovups(ymm11, ymmword_ptr(rcx + 32)).unwrap();
    a.vminps(ymm9, ymm8, ymm11).unwrap();
    a.vmovups(ymmword_ptr(rcx + 128), ymm9).unwrap();
    a.vmovupd(ymm10, ymmword_ptr(rcx + 64)).unwrap();
    a.vmaxpd(ymm12, ymm10, ymmword_ptr(rcx + 96)).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 160), ymm12).unwrap();
    a.ldmxcsr(dword_ptr(rcx + 260)).unwrap();
    a.vzeroupper().unwrap();
    a.xor(eax, eax).unwrap();
    a.ret().unwrap();
    let bytes = a.assemble(base).unwrap();
    code[..bytes.len()].copy_from_slice(&bytes);

    let run = |mode: u8, xpin: bool| -> Vec<u8> {
        let mut m = mem.clone();
        std::thread::spawn(move || {
            jit::test_set_knobs(jit::TestKnobs { xmm_pin: Some(xpin), ..jit::TestKnobs::NONE });
            jit::force(mode);
            unsafe {
                crate::emu::call_va8(base, m.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0);
            }
            jit::force(0);
            jit::test_set_knobs(jit::TestKnobs::NONE);
            m
        })
        .join()
        .unwrap()
    };

    let interp = run(2, true);
    let jit_pinned = run(1, true);
    let jit_unpinned = run(1, false);
    assert_eq!(&jit_pinned[128..192], &interp[128..192], "pinned ymm9/ymm12 DAZ minmax mismatch");
    assert_eq!(&jit_unpinned[128..192], &interp[128..192], "unpinned ymm9/ymm12 DAZ minmax mismatch");
}

/// Zero-spill `FAST_KIND_*` inline thunks (`GetCurrentThreadId`, `GetLastError`, `TlsGetValue`,
/// `TlsSetValue` low + expansion + out-of-range, and all 6 `SRWLock` primitives) across both
/// indirect `call reg` (`lookup_call`) and tail `jmp reg` (`lookup_jmp`), plus bailouts on
/// contention, `SRW_PARKED`, unaligned lock words, and invalid TLS indices.
#[test]
fn inline_fast_thunks_tid_tls_and_srw_call_and_tail_jmp() {
    use core::sync::atomic::{AtomicUsize, Ordering};
    use iced_x86::code_asm::*;
    use win32_rt::kernel32::process::{GetLastError, SetLastError};
    use win32_rt::kernel32::threads::{GetCurrentThreadId, TlsGetValue, TlsSetValue};
    use win32_rt::sync::{
        AcquireSRWLockExclusive, AcquireSRWLockShared, InitializeSRWLock, ReleaseSRWLockExclusive,
        ReleaseSRWLockShared, TryAcquireSRWLockExclusive, TryAcquireSRWLockShared, SRW_PARKED,
        SRW_WAITING_WRITER, SRW_WRITER,
    };

    std::thread::Builder::new()
        .name("fast_thunks_tid_tls_srw".into())
        .spawn(|| {
            let my_tid = unsafe { GetCurrentThreadId() } as u64;
            assert_ne!(my_tid, 0);

            const BASE: u64 = 0x7FFF_E760_0000;
            let vas = [
                BASE + 0x100, // GetCurrentThreadId
                BASE + 0x110, // GetLastError
                BASE + 0x120, // TlsGetValue
                BASE + 0x130, // TlsSetValue
                BASE + 0x140, // AcquireSRWLockExclusive
                BASE + 0x150, // TryAcquireSRWLockExclusive
                BASE + 0x160, // ReleaseSRWLockExclusive
                BASE + 0x170, // AcquireSRWLockShared
                BASE + 0x180, // TryAcquireSRWLockShared
                BASE + 0x190, // ReleaseSRWLockShared
            ];
            let [t_tid, t_gle, t_tls_get, t_tls_set, t_srw_ae, t_srw_te, t_srw_re, t_srw_as, t_srw_ts, t_srw_rs] = vas;
            let fns: [usize; 10] = [
                GetCurrentThreadId as *const () as usize,
                GetLastError as *const () as usize,
                TlsGetValue as *const () as usize,
                TlsSetValue as *const () as usize,
                AcquireSRWLockExclusive as *const () as usize,
                TryAcquireSRWLockExclusive as *const () as usize,
                ReleaseSRWLockExclusive as *const () as usize,
                AcquireSRWLockShared as *const () as usize,
                TryAcquireSRWLockShared as *const () as usize,
                ReleaseSRWLockShared as *const () as usize,
            ];
            for (&va, &p) in vas.iter().zip(fns.iter()) {
                crate::exe::register_native_va(va, p);
            }

            // Guest harness: `invoke(arg0=rcx, arg1=rdx, target=r8, is_tail=r9, sbuf=[rsp+0x28])`
            let c = arena_alloc(0x200, 16);
            let code = (|| -> Result<Vec<u8>, IcedError> {
                let mut a = CodeAssembler::new(64)?;
                let mut do_tail = a.create_label();
                let mut done = a.create_label();
                let mut tail_stub = a.create_label();
                a.push(rbx)?;
                a.push(r12)?;
                a.push(r13)?;
                a.sub(rsp, 0x30)?;
                a.mov(rbx, qword_ptr(rsp + 0x70))?; // sbuf
                a.mov(rax, r8)?;
                a.mov(r10, r9)?;
                a.mov(r12, 0x1122_3344_5566_7788u64)?;
                a.mov(r13, 0xCAFE_BABE_DEAD_BEEFu64)?;
                a.movdqu(xmm6, xmmword_ptr(rbx + 0x00))?;
                a.movdqu(xmm15, xmmword_ptr(rbx + 0x10))?;
                a.test(r10d, r10d)?;
                a.jne(do_tail)?;
                a.call(rax)?;
                a.jmp(done)?;
                a.set_label(&mut do_tail)?;
                a.call(tail_stub)?;
                a.set_label(&mut done)?;
                a.movdqu(xmmword_ptr(rbx + 0x20), xmm6)?;
                a.movdqu(xmmword_ptr(rbx + 0x30), xmm15)?;
                a.mov(qword_ptr(rbx + 0x40), r12)?;
                a.mov(qword_ptr(rbx + 0x48), r13)?;
                a.add(rsp, 0x30)?;
                a.pop(r13)?;
                a.pop(r12)?;
                a.pop(rbx)?;
                a.ret()?;
                a.set_label(&mut tail_stub)?;
                a.jmp(rax)?;
                a.assemble(c as u64)
            })()
            .expect("assemble");

            let sbuf = arena_alloc(0x80, 16);
            let plain = jit::plaincall_enabled();

            unsafe {
                core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
                let invoke = |target: u64, arg0: u64, arg1: u64, is_tail: u64| -> u64 {
                    core::ptr::write_bytes(sbuf, 0, 0x80);
                    for i in 0..0x20usize {
                        *sbuf.add(i) = (0x50 + i) as u8;
                    }
                    jit::force(1);
                    let (ret, _) = emu_run_full(
                        c as u64,
                        [arg0, arg1, target, is_tail, sbuf as u64, 0, 0, 0],
                        [0; 4],
                        [0; 8],
                    );
                    jit::force(0);
                    assert_eq!(
                        core::slice::from_raw_parts(sbuf.add(0x20), 0x20),
                        core::slice::from_raw_parts(sbuf, 0x20),
                        "XMM6/XMM15 clobbered (target={target:#x}, tail={is_tail})"
                    );
                    assert_eq!(core::ptr::read_unaligned(sbuf.add(0x40) as *const u64), 0x1122_3344_5566_7788);
                    assert_eq!(core::ptr::read_unaligned(sbuf.add(0x48) as *const u64), 0xCAFE_BABE_DEAD_BEEF);
                    ret
                };

                // 1. GetCurrentThreadId & GetLastError
                // Warm both call and tail-jump paths so `do_tail -> tail_stub` is compiled & linked.
                assert_eq!(invoke(t_tid, 0, 0, 0), my_tid);
                assert_eq!(invoke(t_tid, 0, 0, 1), my_tid);
                SetLastError(0x42);
                assert_eq!(invoke(t_gle, 0, 0, 0), 0x42);
                if plain {
                    let s0 = jit::test_stub_plain_hits();
                    let rm0 = jit::test_ras_moves();
                    assert_eq!(invoke(t_tid, 0, 0, 0), my_tid);
                    assert_eq!(invoke(t_tid, 0, 0, 1), my_tid);
                    SetLastError(0x1234);
                    assert_eq!(invoke(t_gle, 0, 0, 0), 0x1234);
                    assert_eq!(invoke(t_gle, 0, 0, 1), 0x1234);
                    assert_eq!(jit::test_stub_plain_hits() - s0, 4);
                    assert_eq!(jit::test_ras_moves() - rm0, 0);
                }

                // 2. TlsSetValue & TlsGetValue (low slot < 64, expansion slot >= 64, invalid slot >= 1088)
                assert_eq!(invoke(t_tls_set, 5, 0x1111_2222_3333_4444, 0), 1);
                assert_eq!(invoke(t_tls_get, 5, 0, 0), 0x1111_2222_3333_4444);
                if plain {
                    let s0 = jit::test_stub_plain_hits();
                    SetLastError(77);
                    // Upper 32 bits garbage in rcx must be ignored; TlsSetValue keeps LastError=77.
                    assert_eq!(invoke(t_tls_set, 0xDEAD_0000_0000_0007, 0xABCD_1234_5678_90EF, 0), 1);
                    assert_eq!(GetLastError(), 77);
                    // TlsGetValue reads value AND clears LastError to 0.
                    assert_eq!(invoke(t_tls_get, 0xBEEF_0000_0000_0007, 0, 1), 0xABCD_1234_5678_90EF);
                    assert_eq!(GetLastError(), 0);
                    assert_eq!(jit::test_stub_plain_hits() - s0, 2);

                    // Expansion slot 70: first TlsSetValue bails to allocate TlsExpansionSlots,
                    // then subsequent set/get hit the inline fast thunk.
                    let x0 = super::test_xfer_plain();
                    assert_eq!(invoke(t_tls_set, 70, 0x9988_7766_5544_3322, 0), 1);
                    assert_eq!(super::test_xfer_plain() - x0, 1, "first expansion TlsSetValue allocates via slow path");
                    let s1 = jit::test_stub_plain_hits();
                    SetLastError(99);
                    assert_eq!(invoke(t_tls_get, 70, 0, 0), 0x9988_7766_5544_3322);
                    assert_eq!(GetLastError(), 0);
                    assert_eq!(invoke(t_tls_set, 70, 0x5555_6666_7777_8888, 1), 1);
                    assert_eq!(invoke(t_tls_get, 70, 0, 1), 0x5555_6666_7777_8888);
                    assert_eq!(jit::test_stub_plain_hits() - s1, 3);

                    // Invalid slot >= 1088 bails to slow path and sets ERROR_INVALID_PARAMETER (87).
                    let x1 = super::test_xfer_plain();
                    assert_eq!(invoke(t_tls_get, 1088, 0, 0), 0);
                    assert_eq!(GetLastError(), 87);
                    assert_eq!(invoke(t_tls_set, 2000, 1, 0), 0);
                    assert_eq!(GetLastError(), 87);
                    assert_eq!(super::test_xfer_plain() - x1, 2);
                }

                // 3. SRWLock (exclusive + shared + try + contention + parked + unaligned)
                let mut lock_word = AtomicUsize::new(0);
                let lp = (&mut lock_word as *mut AtomicUsize).cast::<core::ffi::c_void>();
                InitializeSRWLock(lp);
                let lva = lp as u64;

                // Warm all 6 targets via xfer_plain so their FAST_KIND_* entries are installed.
                invoke(t_srw_ae, lva, 0, 0);
                invoke(t_srw_re, lva, 0, 0);
                assert_eq!(invoke(t_srw_te, lva, 0, 0), 1);
                invoke(t_srw_re, lva, 0, 0);
                invoke(t_srw_as, lva, 0, 0);
                assert_eq!(invoke(t_srw_ts, lva, 0, 0), 1);
                invoke(t_srw_rs, lva, 0, 0);
                invoke(t_srw_rs, lva, 0, 0);
                assert_eq!(lock_word.load(Ordering::SeqCst), 0);

                if plain {
                    let s0 = jit::test_stub_plain_hits();
                    let rm0 = jit::test_ras_moves();
                    // Exclusive acquire -> TryAcquireExclusive fails inline (0) -> TryAcquireShared fails inline (0) -> ReleaseExclusive.
                    invoke(t_srw_ae, lva, 0, 0);
                    assert_eq!(lock_word.load(Ordering::Acquire), SRW_WRITER);
                    assert_eq!(invoke(t_srw_te, lva, 0, 0), 0);
                    assert_eq!(invoke(t_srw_ts, lva, 0, 1), 0);
                    invoke(t_srw_re, lva, 0, 1);
                    assert_eq!(lock_word.load(Ordering::Acquire), 0);

                    // Shared acquire (2 readers) -> TryAcquireExclusive fails inline (0) -> ReleaseShared twice.
                    invoke(t_srw_as, lva, 0, 0);
                    assert_eq!(invoke(t_srw_ts, lva, 0, 0), 1);
                    assert_eq!(lock_word.load(Ordering::Acquire), 16);
                    assert_eq!(invoke(t_srw_te, lva, 0, 1), 0);
                    invoke(t_srw_rs, lva, 0, 0);
                    assert_eq!(lock_word.load(Ordering::Acquire), 8);
                    invoke(t_srw_rs, lva, 0, 1);
                    assert_eq!(lock_word.load(Ordering::Acquire), 0);

                    // Waiting writer blocks TryAcquireShared inline (returns 0 without bailing).
                    lock_word.store(SRW_WAITING_WRITER, Ordering::SeqCst);
                    assert_eq!(invoke(t_srw_ts, lva, 0, 0), 0);
                    lock_word.store(0, Ordering::SeqCst);
                    assert_eq!(jit::test_stub_plain_hits() - s0, 10, "all 10 uncontended/try SRW ops must hit inline fast thunk");
                    assert_eq!(jit::test_ras_moves() - rm0, 0);

                    // Parked writer bit forces ReleaseSRWLockExclusive to bail cleanly to slow path without losing the lock.
                    lock_word.store(SRW_WRITER | SRW_PARKED, Ordering::SeqCst);
                    let x0 = super::test_xfer_plain();
                    invoke(t_srw_re, lva, 0, 0);
                    assert_eq!(super::test_xfer_plain() - x0, 1);
                    assert_eq!(lock_word.load(Ordering::SeqCst), 0);

                    // Unaligned SRWLock bails cleanly to sync.rs unaligned shard lock.
                    #[repr(align(16))]
                    struct RawBuf([u8; 32]);
                    let mut raw = RawBuf([0u8; 32]);
                    let unal = raw.0.as_mut_ptr().add(3) as u64;
                    let x1 = super::test_xfer_plain();
                    invoke(t_srw_ae, unal, 0, 0);
                    assert_eq!(invoke(t_srw_te, unal, 0, 0), 0);
                    invoke(t_srw_re, unal, 0, 0);
                    assert_eq!(super::test_xfer_plain() - x1, 3);
                }

                for &va in &vas {
                    crate::exe::unregister_native_va(va);
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// Verify conditional `ymm_hi` spill/reload (`XBUD` bit 0) and `call_with_host_fpcr` FPCR
/// elision/isolation:
/// 1. When `ymm_dirty == 0`, a `DF_PLAIN` callee that scribbles upper 64 bits of `q8..q15`
///    and all of `v16..v31` must still return clean zeros in `ymm_hi[0..7]` and intact `xmm0..15`.
/// 2. After spilling dirty `ymm_hi`, running `vzeroupper` (which clears `v8..v15` and `XBUD` bit 0),
///    and exiting JIT, `Cpu::ymm_hi[0..7]` in memory must be zero (no stale dirty state).
/// 3. `call_with_host_fpcr` preserves non-default guest `MXCSR` (`0x9FC0` FTZ|DAZ) while running
///    the host callee under IEEE-754 `host_fpcr`, AND works cleanly under default `0x1F80`.
#[test]
fn conditional_ymm_spill_and_fpcr_elision_across_plain_call() {
    use iced_x86::code_asm::*;

    static SEEN_FPCR: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(u64::MAX);
    unsafe extern "win64" fn clobber_vectors_and_read_fpcr(a: u64) -> u64 {
        let fpcr: u64;
        core::arch::asm!(
            "mrs {f}, fpcr",
            // Clobber upper 64 bits of q8..q15 (caller-saved under AAPCS64) while preserving d8..d15!
            "movi v0.16b, #0xaa",
            "ins v8.d[1], v0.d[0]",
            "ins v9.d[1], v0.d[0]",
            "ins v15.d[1], v0.d[0]",
            // Clobber caller-saved v16..v19.
            "movi v16.16b, #0x55",
            "movi v17.16b, #0x66",
            f = out(reg) fpcr,
            out("v0") _,
            out("v16") _,
            out("v17") _,
            options(nomem, nostack),
        );
        SEEN_FPCR.store(fpcr & super::FPCR_GUEST_MASK, core::sync::atomic::Ordering::SeqCst);
        a.wrapping_add(1)
    }

    const HOST_VA: u64 = 0x7FFF_E761_0000;
    const HOST_INT_VA: u64 = 0x7FFF_E761_0100;
    let host_ptr = clobber_vectors_and_read_fpcr as *const () as usize;
    unsafe extern "win64" fn clobber_vectors_int_only(a: u64) -> u64 {
        core::arch::asm!(
            "movi v0.16b, #0xbb",
            "ins v8.d[1], v0.d[0]",
            "movi v16.16b, #0x77",
            out("v0") _,
            out("v16") _,
            options(nomem, nostack),
        );
        a.wrapping_add(2)
    }
    let host_int_ptr = clobber_vectors_int_only as *const () as usize;
    // Float/FPCR-sensitive DF_PLAIN_XMM callee must observe host FPCR (0) under guest 0x9FC0.
    crate::emu::routes::test_set_extra_plain_flags(host_ptr, crate::emu::DF_PLAIN | crate::emu::DF_PLAIN_XMM);
    crate::emu::routes::test_set_extra_plain_flags(host_int_ptr, crate::emu::DF_PLAIN);
    crate::exe::register_native_va(HOST_VA, host_ptr);
    crate::exe::register_native_va(HOST_INT_VA, host_int_ptr);

    let code_buf = arena_alloc(0x200, 16);
    let data = arena_alloc(0x200, 32);
    unsafe {
        // [0x00..0x20]: 256-bit pattern for ymm0
        for i in 0..32usize {
            *data.add(i) = (0x10 + i) as u8;
        }
        // [0x20..0x24]: 0x9FC0 (FTZ|DAZ), [0x24..0x28]: 0x1F80 (default)
        core::ptr::write_unaligned(data.add(0x20) as *mut u32, 0x9FC0);
        core::ptr::write_unaligned(data.add(0x24) as *mut u32, 0x1F80);

        let bytes = (|| -> Result<Vec<u8>, IcedError> {
            let mut a = CodeAssembler::new(64)?;
            a.push(rbx)?;
            a.sub(rsp, 0x28)?;
            a.mov(rbx, rcx)?;
            // 1) Clean ymm_hi (ymm_dirty == 0) + default MXCSR (0x1F80): call pure-integer DF_PLAIN
            //    (`needs_xmm == 0` -> zero mrs/msr fpcr, preserves xmm0, re-zeros clobbered ymm_hi).
            a.vmovdqu(xmm0, xmmword_ptr(rbx + 0x00))?;
            a.mov(rcx, 10u64)?;
            a.mov(r10, HOST_INT_VA)?;
            a.call(r10)?;
            // Store full ymm0 (low MUST still be [0x00..0x10], upper 128 bits MUST be 0!).
            a.vmovdqu(ymmword_ptr(rbx + 0x40), ymm0)?;
            // 2) Dirty ymm0 + non-default MXCSR (0x9FC0): call DF_PLAIN_XMM host fn (`needs_xmm == 1`).
            a.ldmxcsr(dword_ptr(rbx + 0x20))?;
            a.vmovdqu(ymm0, ymmword_ptr(rbx + 0x00))?;
            a.vmovdqu(ymm1, ymmword_ptr(rbx + 0x00))?;
            a.mov(rcx, 20u64)?;
            a.mov(r10, HOST_VA)?;
            a.call(r10)?;
            // Store full ymm1 (both low and high 128 bits must match [0x00..0x20]) + stmxcsr.
            a.vmovdqu(ymmword_ptr(rbx + 0x60), ymm1)?;
            a.stmxcsr(dword_ptr(rbx + 0x80))?;
            // 3) Clear ymm_hi via vzeroupper after dirty spill, restore 0x1F80, and exit JIT.
            a.vzeroupper()?;
            a.ldmxcsr(dword_ptr(rbx + 0x24))?;
            a.add(rsp, 0x28)?;
            a.pop(rbx)?;
            a.ret()?;
            a.assemble(code_buf as u64)
        })()
        .expect("assemble");
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), code_buf, bytes.len());

        // Run twice so call #2 hits cached `is_plain` in `stubs.rs` (call #1 warms via `xfer_plain`).
        for iter in 0..2 {
            SEEN_FPCR.store(u64::MAX, core::sync::atomic::Ordering::SeqCst);
            jit::force(1);
            let (ret, _) = emu_run_full(code_buf as u64, [data as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            jit::force(0);
            assert_eq!(ret, 21, "iter {iter}: return value mismatch");
            assert_eq!(
                SEEN_FPCR.load(core::sync::atomic::Ordering::SeqCst),
                0,
                "iter {iter}: host callee must observe IEEE-754 host FPCR even under guest 0x9FC0"
            );
            assert_eq!(
                core::slice::from_raw_parts(data.add(0x40), 16),
                core::slice::from_raw_parts(data, 16),
                "iter {iter}: xmm0 low corrupted across clean plain call"
            );
            assert_eq!(
                core::slice::from_raw_parts(data.add(0x50), 16),
                &[0u8; 16],
                "iter {iter}: ymm0_hi corrupted by AAPCS64 callee upper-64b clobber when clean"
            );
            assert_eq!(
                core::slice::from_raw_parts(data.add(0x60), 32),
                core::slice::from_raw_parts(data, 32),
                "iter {iter}: dirty ymm1 corrupted across plain_xmm call"
            );
            assert_eq!(
                core::ptr::read_unaligned(data.add(0x80) as *const u32),
                0x9FC0,
                "iter {iter}: guest MXCSR not preserved across plain call"
            );
        }
        crate::exe::unregister_native_va(HOST_VA);
        crate::exe::unregister_native_va(HOST_INT_VA);
    }
}

/// Regression for Codex P1 finding: a 256-bit write followed in the same basic block by an x87
/// instruction that takes a deferred `X87Cold` helper path (e.g. `fld m32` of a subnormal) must
/// flush its pre-branch `ymm_dirty_pending` bits before `x87_trampoline` calls conditional
/// `spill_xmm`/`reload_xmm`, preserving pinned `ymm_hi[0..7]` (`v8..v15`).
#[test]
fn deferred_x87_cold_helper_preserves_pending_ymm_hi() {
    use iced_x86::code_asm::*;

    let mut a = CodeAssembler::new(64).unwrap();
    // Single block: ymm_dirty_pending is NOT flushed before `fld` branches to `X87ColdKind::FldF32`.
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 0x00)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 0x20)).unwrap();
    a.fninit().unwrap();
    a.fld(dword_ptr(rcx + 0x40)).unwrap();
    a.fstp(qword_ptr(rcx + 0x48)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x60), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x80), ymm3).unwrap();
    a.vzeroupper().unwrap();
    a.xor(eax, eax).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    const DATA: usize = 0x100;
    let cptr = arena_alloc(code.len() + 16, 16);
    let dptr = arena_alloc(DATA, 64);
    let mut init = vec![0u8; DATA];
    for i in 0..32usize {
        init[i] = (0x21 + i) as u8;
        init[0x20 + i] = (0xA1 + i) as u8;
    }
    // Smallest positive f32 subnormal forces `X87ColdKind::FldF32` cold helper path.
    init[0x40..0x44].copy_from_slice(&0x0000_0001u32.to_le_bytes());

    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), cptr, code.len());
        core::ptr::write_bytes(cptr.add(code.len()), 0xCC, 16);
        let interp = run_mode(cptr, dptr, &init, 2);
        let jit1 = run_mode(cptr, dptr, &init, 1);
        assert_eq!(jit1.1, interp.1, "deferred x87 cold helper JIT vs interpreter mismatch");
        assert_eq!(
            &jit1.1[0x60..0x80],
            &init[0x00..0x20],
            "ymm0 upper 128 bits lost across deferred x87 cold helper"
        );
        assert_eq!(
            &jit1.1[0x80..0xA0],
            &init[0x20..0x40],
            "ymm3 upper 128 bits lost across deferred x87 cold helper"
        );
        let f64_bits = u64::from_le_bytes(jit1.1[0x48..0x50].try_into().unwrap());
        assert_eq!(f64_bits, (f32::from_bits(1) as f64).to_bits(), "x87 subnormal fld/fstp mismatch");
    }
}

/// Compile-time constant (`Chk::Abs`) misaligned accesses must emit inline exact-TSO
/// (`ldr; dmb ishld` / `dmb ishst; str`) without `ldapr`/`stlr` traps or `SlowMem` stubs:
/// - Under `lse2_trap=1`: provably 16-byte-granule-crossing constants emit inline exact-TSO
///   (zero `SIGBUS` traps, zero LSE2 patch-slab use), while intra-16B-granule constants emit
///   single-instruction `ldapr`/`stlr`.
/// - Under `lse2_trap=0`: naturally aligned 2B/4B/8B constants emit `ldapr`/`stlr` with zero
///   alignment check, while naturally misaligned constants emit inline exact-TSO with zero
///   out-of-line `SlowMem` cold stubs.
/// - Execution parity (Generic == FastOp == Live JIT == AOT) holds across loads, stores, and
///   RMW instructions (`inc`/`add`) without any `SIGBUS` handler installed.
#[test]
fn inline_exact_tso_for_compile_time_misaligned_abs_accesses() {
    struct ResetKnobs;
    impl Drop for ResetKnobs {
        fn drop(&mut self) {
            jit::test_set_knobs(jit::TestKnobs::NONE);
            jit::TEST_STACK_TSO.with(|c| c.set(None));
            aot::test_uninstall();
        }
    }
    let _guard = ResetKnobs;

    let is_ldapr = |w: u32| (w & 0x3FFF_FC00) == 0x38BF_C000;
    let is_stlr = |w: u32| (w & 0x3FFF_FC00) == 0x089F_FC00;
    const DMB_ISHLD: u32 = 0xD503_39BF;
    const DMB_ISHST: u32 = 0xD503_3ABF;

    // Emit RIP-relative x86-64 sequences targeting `target_va` at runtime `code_va`.
    let push_rel = |buf: &mut Vec<u8>, code_va: u64, prefix: &[u8], target_va: u64, tail: &[u8]| {
        buf.extend_from_slice(prefix);
        let next_rip = code_va + buf.len() as u64 + 4 + tail.len() as u64;
        let disp = i32::try_from(target_va as i64 - next_rip as i64).expect("rip disp32");
        buf.extend_from_slice(&disp.to_le_bytes());
        buf.extend_from_slice(tail);
    };

    // Place code and 16-byte-aligned RW data close together in the test arena so disp32 fits.
    const ARENA_SZ: usize = 0x1000;
    const DATA_OFF: usize = 0x800; // 16-byte aligned relative to 64B-aligned base
    let base_ptr = arena_alloc(ARENA_SZ, 64);
    let base_va = base_ptr as u64;
    let data_va = base_va + DATA_OFF as u64;
    assert_eq!(data_va & 15, 0);

    let stubs: jit::AotStubOffs = [0; 16];
    let translate_hot_cold = |code_bytes: &[u8], lse2_trap: bool| -> (Vec<u32>, Vec<u32>) {
        let code_ptr = base_ptr;
        unsafe {
            core::ptr::copy_nonoverlapping(code_bytes.as_ptr(), code_ptr, code_bytes.len());
            core::ptr::write_bytes(code_ptr.add(code_bytes.len()), 0xCC, 16);
        }
        jit::test_set_knobs(jit::TestKnobs {
            memchk: Some(15),
            lse2_trap: Some(lse2_trap),
            ..jit::TestKnobs::NONE
        });
        let mut bc = Box::new(BlockCache::new());
        let mut scratch = Vec::new();
        let mut tr_scratch = jit::TrScratch::new();
        let t = unsafe {
            jit::aot_translate(
                &mut *bc,
                base_va,
                0x2000_0000_1000,
                0x2000_0000_0000,
                &stubs,
                &mut scratch,
                &mut tr_scratch,
            )
        }
        .expect("aot_translate");
        let main = t.main as usize;
        (t.words[..main].to_vec(), t.words[main..].to_vec())
    };

    // (label, width_bytes, data_offset, expect_inline_lse2_on, expect_inline_lse2_off)
    let cases: &[(&str, usize, usize, bool, bool)] = &[
        ("2B crossing 16B (+0x0F)", 2, 0x0F, true, true),
        ("4B crossing 16B (+0x0E)", 4, 0x0E, true, true),
        ("4B crossing 16B (+0x0D)", 4, 0x0D, true, true),
        ("8B crossing 16B (+0x0C)", 8, 0x0C, true, true),
        ("8B crossing 16B (+0x09)", 8, 0x09, true, true),
        ("4B intra-16B unaligned (+0x02)", 4, 0x22, false, true),
        ("8B intra-16B unaligned (+0x04)", 8, 0x34, false, true),
        ("2B naturally aligned (+0x02)", 2, 0x42, false, false),
        ("4B naturally aligned (+0x04)", 4, 0x54, false, false),
        ("8B naturally aligned (+0x00)", 8, 0x60, false, false),
    ];

    for &(label, bytes, off, expect_inline_on, expect_inline_off) in cases {
        let tgt = data_va + off as u64;
        // Build a single basic block exercising load, immediate/reg store, and RMW at `tgt`.
        let mut code = Vec::new();
        match bytes {
            2 => {
                // mov ax, word ptr [rip+disp] ; add ax, 7 ; mov word ptr [rip+disp], ax ; add word ptr [rip+disp], 3 ; ret
                push_rel(&mut code, base_va, &[0x66, 0x8B, 0x05], tgt, &[]);
                code.extend_from_slice(&[0x66, 0x83, 0xC0, 0x07]);
                push_rel(&mut code, base_va, &[0x66, 0x89, 0x05], tgt, &[]);
                push_rel(&mut code, base_va, &[0x66, 0x83, 0x05], tgt, &[0x03]);
            }
            4 => {
                // mov eax, dword ptr [rip+disp] ; mov dword ptr [rip+disp], 0xDEADBEEF ; inc dword ptr [rip+disp] ; ret
                push_rel(&mut code, base_va, &[0x8B, 0x05], tgt, &[]);
                push_rel(&mut code, base_va, &[0xC7, 0x05], tgt, &0xDEAD_BEEFu32.to_le_bytes());
                push_rel(&mut code, base_va, &[0xFF, 0x05], tgt, &[]);
            }
            8 => {
                // mov rax, qword ptr [rip+disp] ; add rax, 5 ; mov qword ptr [rip+disp], rax ; add qword ptr [rip+disp], rax ; ret
                push_rel(&mut code, base_va, &[0x48, 0x8B, 0x05], tgt, &[]);
                code.extend_from_slice(&[0x48, 0x83, 0xC0, 0x05]);
                push_rel(&mut code, base_va, &[0x48, 0x89, 0x05], tgt, &[]);
                push_rel(&mut code, base_va, &[0x48, 0x01, 0x05], tgt, &[]);
            }
            _ => unreachable!(),
        }
        code.push(0xC3); // ret

        // 1. Verify emitted ARM64 instruction sequences for both lse2_trap=true and lse2_trap=false.
        for (lse2_on, expect_inline) in [(true, expect_inline_on), (false, expect_inline_off)] {
            let (hot, cold) = translate_hot_cold(&code, lse2_on);
            // Neither mode should emit any out-of-line SlowMem dmb stubs for Chk::Abs.
            assert!(
                !cold.iter().any(|&w| w == DMB_ISHLD || w == DMB_ISHST),
                "{label} (lse2={lse2_on}): unexpected out-of-line SlowMem stub in cold code"
            );
            if expect_inline {
                assert!(
                    !hot.iter().any(|&w| is_ldapr(w) || is_stlr(w)),
                    "{label} (lse2={lse2_on}): must not emit trapping ldapr/stlr"
                );
                assert!(
                    hot.iter().any(|&w| w == DMB_ISHLD),
                    "{label} (lse2={lse2_on}): expected inline dmb ishld for misaligned load"
                );
                assert!(
                    hot.iter().any(|&w| w == DMB_ISHST),
                    "{label} (lse2={lse2_on}): expected inline dmb ishst for misaligned store/RMW"
                );
            } else {
                assert!(
                    hot.iter().any(|&w| is_ldapr(w)),
                    "{label} (lse2={lse2_on}): expected single-insn ldapr"
                );
                assert!(
                    hot.iter().any(|&w| is_stlr(w)),
                    "{label} (lse2={lse2_on}): expected single-insn stlr"
                );
                assert!(
                    !hot.iter().any(|&w| w == DMB_ISHLD || w == DMB_ISHST),
                    "{label} (lse2={lse2_on}): unexpected dmb barrier when ldapr/stlr is safe"
                );
            }
        }

        // 2. Verify live execution parity across Interpreter, Live JIT (lse2=0 & 1), and AOT
        // without any SIGBUS handler installed (proving zero hardware alignment traps).
        let init = vec![0xA5u8; 0x100];
        let run_at = |mode: u8, lse2_on: bool, use_aot: bool| -> (u64, Vec<u8>) {
            unsafe {
                core::ptr::copy_nonoverlapping(code.as_ptr(), base_ptr, code.len());
                core::ptr::write_bytes(base_ptr.add(code.len()), 0xCC, 16);
                jit::test_set_knobs(jit::TestKnobs {
                    memchk: Some(15),
                    lse2_trap: Some(lse2_on),
                    ..jit::TestKnobs::NONE
                });
                if use_aot {
                    aot::test_uninstall();
                    let (live, _, _) = aot::test_install(&[base_va], || {});
                    assert_eq!(live, 1, "{label}: AOT install failed");
                }
                let res = run_mode(base_ptr, base_ptr.add(DATA_OFF), &init, mode);
                if use_aot {
                    aot::test_uninstall();
                }
                res
            }
        };

        let interp = run_at(2, false, false);
        let jit_off = run_at(1, false, false);
        let jit_on = run_at(1, true, false);
        let aot_on = run_at(1, true, true);
        assert_eq!(jit_off, interp, "{label}: Live JIT (lse2=0) != interpreter");
        assert_eq!(jit_on, interp, "{label}: Live JIT (lse2=1) != interpreter");
        assert_eq!(aot_on, interp, "{label}: AOT (lse2=1) != interpreter");
    }

    // 3. Verify `store_checked_addr` respects `Chk::Stack` (`PORT_JIT_STACK_TSO=0`):
    // `add dword ptr [rsp + 8], 1; ret` must use plain `ldr`/`str` when stack TSO is off
    // and `ldapr`/`stlr` when stack TSO is on.
    let stack_rmw: &[u8] = &[0x83, 0x44, 0x24, 0x08, 0x01, 0xC3];
    for stack_tso_on in [false, true] {
        jit::TEST_STACK_TSO.with(|c| c.set(Some(stack_tso_on)));
        let (hot, _) = translate_hot_cold(stack_rmw, true);
        let has_tso_st = hot.iter().any(|&w| is_stlr(w));
        assert_eq!(
            has_tso_st, stack_tso_on,
            "stack RMW store_checked_addr (stack_tso={stack_tso_on}): expected stlr={stack_tso_on}"
        );
    }
}
