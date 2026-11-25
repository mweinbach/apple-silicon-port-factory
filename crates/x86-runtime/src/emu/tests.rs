//! Operand-form regression tests for the generic SSE/AVX/integer handlers in `emu_run_full`.
//!
//! Each snippet takes a scratch buffer in RCX (inputs at 0..0x100, outputs at 0x100..),
//! was assembled with `clang -target x86_64-apple-macos`, and its expected results match
//! real x86 hardware semantics.

use super::*;
use std::sync::Mutex;

#[rustfmt::skip]
const PXOR_FORMS: &[u8] = &[
    0x66, 0x0f, 0xef, 0xc1,  // pxor xmm0, xmm1
    0xf3, 0x0f, 0x7f, 0x81, 0x00, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x100], xmm0
    0xf3, 0x0f, 0x6f, 0x11,  // movdqu xmm2, xmmword ptr [rcx]
    0x66, 0x0f, 0xef, 0x51, 0x10,  // pxor xmm2, xmmword ptr [rcx + 0x10]
    0xf3, 0x0f, 0x7f, 0x91, 0x10, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x110], xmm2
    0xc5, 0xf1, 0xef, 0x19,  // vpxor xmm3, xmm1, xmmword ptr [rcx]
    0xf3, 0x0f, 0x7f, 0x99, 0x20, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x120], xmm3
    0xc3,  // ret
];

#[rustfmt::skip]
const SCALAR_ARITH: &[u8] = &[
    0x0f, 0x28, 0xe0,  // movaps xmm4, xmm0
    0xc5, 0xf2, 0x58, 0xc2,  // vaddss xmm0, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0x81, 0x00, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x100], xmm0
    0xf3, 0x0f, 0x58, 0xe2,  // addss xmm4, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x10, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x110], xmm4
    0x0f, 0x28, 0xe9,  // movaps xmm5, xmm1
    0xf3, 0x0f, 0x58, 0x29,  // addss xmm5, dword ptr [rcx]
    0xf3, 0x0f, 0x7f, 0xa9, 0x20, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x120], xmm5
    0xc5, 0xf3, 0x5c, 0xf2,  // vsubsd xmm6, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xb1, 0x30, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x130], xmm6
    0xc3,  // ret
];

#[rustfmt::skip]
const SCALAR_SPECIAL: &[u8] = &[
    0xf3, 0x0f, 0x10, 0x21,  // movss xmm4, dword ptr [rcx]
    0xf3, 0x0f, 0x5e, 0x61, 0x04,  // divss xmm4, dword ptr [rcx + 0x4]
    0x66, 0x0f, 0x7e, 0xa1, 0x00, 0x01, 0x00, 0x00,  // movd dword ptr [rcx + 0x100], xmm4
    0xf3, 0x0f, 0x10, 0x61, 0x04,  // movss xmm4, dword ptr [rcx + 0x4]
    0xf3, 0x0f, 0x5e, 0x61, 0x04,  // divss xmm4, dword ptr [rcx + 0x4]
    0x66, 0x0f, 0x7e, 0xa1, 0x04, 0x01, 0x00, 0x00,  // movd dword ptr [rcx + 0x104], xmm4
    0xf3, 0x0f, 0x51, 0x61, 0x08,  // sqrtss xmm4, dword ptr [rcx + 0x8]
    0x66, 0x0f, 0x7e, 0xa1, 0x08, 0x01, 0x00, 0x00,  // movd dword ptr [rcx + 0x108], xmm4
    0xf3, 0x0f, 0x10, 0x61, 0x0c,  // movss xmm4, dword ptr [rcx + 0xc]
    0xf3, 0x0f, 0x5d, 0x21,  // minss xmm4, dword ptr [rcx]
    0x66, 0x0f, 0x7e, 0xa1, 0x0c, 0x01, 0x00, 0x00,  // movd dword ptr [rcx + 0x10c], xmm4
    0xf3, 0x0f, 0x10, 0x21,  // movss xmm4, dword ptr [rcx]
    0xf3, 0x0f, 0x5d, 0x61, 0x0c,  // minss xmm4, dword ptr [rcx + 0xc]
    0x66, 0x0f, 0x7e, 0xa1, 0x10, 0x01, 0x00, 0x00,  // movd dword ptr [rcx + 0x110], xmm4
    0xf3, 0x0f, 0x10, 0x61, 0x0c,  // movss xmm4, dword ptr [rcx + 0xc]
    0xf3, 0x0f, 0x5f, 0x21,  // maxss xmm4, dword ptr [rcx]
    0x66, 0x0f, 0x7e, 0xa1, 0x14, 0x01, 0x00, 0x00,  // movd dword ptr [rcx + 0x114], xmm4
    0xc5, 0xf2, 0x51, 0xea,  // vsqrtss xmm5, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xa9, 0x20, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x120], xmm5
    0xf2, 0x0f, 0x10, 0x71, 0x10,  // movsd xmm6, qword ptr [rcx + 0x10]
    0xf2, 0x0f, 0x5e, 0x71, 0x18,  // divsd xmm6, qword ptr [rcx + 0x18]
    0x66, 0x0f, 0xd6, 0xb1, 0x30, 0x01, 0x00, 0x00,  // movq qword ptr [rcx + 0x130], xmm6
    0xc3,  // ret
];

#[rustfmt::skip]
const CVT_TO_INT: &[u8] = &[
    0xf3, 0x0f, 0x2c, 0x01,  // cvttss2si eax, dword ptr [rcx]
    0x89, 0x81, 0x00, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x100], eax
    0xf3, 0x0f, 0x2c, 0x41, 0x04,  // cvttss2si eax, dword ptr [rcx + 0x4]
    0x89, 0x81, 0x04, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x104], eax
    0xf3, 0x0f, 0x2c, 0x41, 0x08,  // cvttss2si eax, dword ptr [rcx + 0x8]
    0x89, 0x81, 0x08, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x108], eax
    0xf3, 0x0f, 0x2c, 0x41, 0x0c,  // cvttss2si eax, dword ptr [rcx + 0xc]
    0x89, 0x81, 0x0c, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x10c], eax
    0xf3, 0x0f, 0x2d, 0x41, 0x10,  // cvtss2si eax, dword ptr [rcx + 0x10]
    0x89, 0x81, 0x10, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x110], eax
    0xf3, 0x0f, 0x2d, 0x41, 0x14,  // cvtss2si eax, dword ptr [rcx + 0x14]
    0x89, 0x81, 0x14, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x114], eax
    0xf2, 0x48, 0x0f, 0x2c, 0x41, 0x18,  // cvttsd2si rax, qword ptr [rcx + 0x18]
    0x48, 0x89, 0x81, 0x18, 0x01, 0x00, 0x00,  // mov qword ptr [rcx + 0x118], rax
    0xf3, 0x0f, 0x10, 0x61, 0x04,  // movss xmm4, dword ptr [rcx + 0x4]
    0xf3, 0x48, 0x0f, 0x2c, 0xc4,  // cvttss2si rax, xmm4
    0x48, 0x89, 0x81, 0x20, 0x01, 0x00, 0x00,  // mov qword ptr [rcx + 0x120], rax
    0xf2, 0x0f, 0x2d, 0x41, 0x20,  // cvtsd2si eax, qword ptr [rcx + 0x20]
    0x89, 0x81, 0x28, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x128], eax
    0xc3,  // ret
];

#[rustfmt::skip]
const UCOMISS_FLAGS: &[u8] = &[
    0xb8, 0xff, 0xff, 0xff, 0x7f,  // mov eax, 0x7fffffff
    0x83, 0xc0, 0x01,  // add eax, 0x1
    0x0f, 0x2e, 0xc1,  // ucomiss xmm0, xmm1
    0x0f, 0x94, 0x81, 0x00, 0x01, 0x00, 0x00,  // sete byte ptr [rcx + 0x100]
    0x0f, 0x9a, 0x81, 0x01, 0x01, 0x00, 0x00,  // setp byte ptr [rcx + 0x101]
    0x0f, 0x92, 0x81, 0x02, 0x01, 0x00, 0x00,  // setb byte ptr [rcx + 0x102]
    0x0f, 0x90, 0x81, 0x03, 0x01, 0x00, 0x00,  // seto byte ptr [rcx + 0x103]
    0x0f, 0x98, 0x81, 0x04, 0x01, 0x00, 0x00,  // sets byte ptr [rcx + 0x104]
    0x0f, 0x2f, 0xd1,  // comiss xmm2, xmm1
    0x0f, 0x94, 0x81, 0x08, 0x01, 0x00, 0x00,  // sete byte ptr [rcx + 0x108]
    0x0f, 0x9a, 0x81, 0x09, 0x01, 0x00, 0x00,  // setp byte ptr [rcx + 0x109]
    0x0f, 0x92, 0x81, 0x0a, 0x01, 0x00, 0x00,  // setb byte ptr [rcx + 0x10a]
    0x66, 0x0f, 0x2e, 0x19,  // ucomisd xmm3, qword ptr [rcx]
    0x0f, 0x97, 0x81, 0x0c, 0x01, 0x00, 0x00,  // seta byte ptr [rcx + 0x10c]
    0xc3,  // ret
];

#[rustfmt::skip]
const CMPPS_PREDS: &[u8] = &[
    0xc5, 0xf0, 0xc2, 0xe2, 0x0e,  // vcmpgtps xmm4, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x00, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x100], xmm4
    0xc5, 0xf0, 0xc2, 0xe2, 0x0d,  // vcmpgeps xmm4, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x10, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x110], xmm4
    0xc5, 0xf0, 0xc2, 0xe2, 0x08,  // vcmpeq_uqps xmm4, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x20, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x120], xmm4
    0xc5, 0xf0, 0xc2, 0xe2, 0x0c,  // vcmpneq_oqps xmm4, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x30, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x130], xmm4
    0xc5, 0xf0, 0xc2, 0xe2, 0x09,  // vcmpngeps xmm4, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x40, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x140], xmm4
    0xc5, 0xf0, 0xc2, 0xe2, 0x0a,  // vcmpngtps xmm4, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x50, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x150], xmm4
    0x0f, 0x28, 0xe1,  // movaps xmm4, xmm1
    0x0f, 0xc2, 0xe2, 0x01,  // cmpltps xmm4, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x60, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x160], xmm4
    0x0f, 0x28, 0xe1,  // movaps xmm4, xmm1
    0x0f, 0xc2, 0xe2, 0x05,  // cmpnltps xmm4, xmm2
    0xf3, 0x0f, 0x7f, 0xa1, 0x70, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x170], xmm4
    0xc5, 0xf2, 0xc2, 0x21, 0x1e,  // vcmpgt_oqss xmm4, xmm1, dword ptr [rcx]
    0xf3, 0x0f, 0x7f, 0xa1, 0x80, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x180], xmm4
    0xc3,  // ret
];

#[rustfmt::skip]
const VEX128_ZERO_UPPER: &[u8] = &[
    0xc5, 0xfc, 0x10, 0x01,  // vmovups ymm0, ymmword ptr [rcx]
    0xc5, 0xfc, 0x10, 0x21,  // vmovups ymm4, ymmword ptr [rcx]
    0xc5, 0xfc, 0x10, 0x29,  // vmovups ymm5, ymmword ptr [rcx]
    0xc5, 0xfc, 0x10, 0x31,  // vmovups ymm6, ymmword ptr [rcx]
    0xc5, 0xfc, 0x10, 0x39,  // vmovups ymm7, ymmword ptr [rcx]
    0xc5, 0xf8, 0x28, 0xc1,  // vmovaps xmm0, xmm1
    0xc5, 0xfc, 0x11, 0x81, 0x00, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x100], ymm0
    0xc5, 0xf0, 0x58, 0xe2,  // vaddps xmm4, xmm1, xmm2
    0xc5, 0xfc, 0x11, 0xa1, 0x20, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x120], ymm4
    0xc5, 0xf0, 0x57, 0xea,  // vxorps xmm5, xmm1, xmm2
    0xc5, 0xfc, 0x11, 0xa9, 0x40, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x140], ymm5
    0xc5, 0xf2, 0x58, 0xf2,  // vaddss xmm6, xmm1, xmm2
    0xc5, 0xfc, 0x11, 0xb1, 0x60, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x160], ymm6
    0xc5, 0xfa, 0x10, 0x79, 0x40,  // vmovss xmm7, dword ptr [rcx + 0x40]
    0xc5, 0xfc, 0x11, 0xb9, 0x80, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x180], ymm7
    0xc5, 0xf8, 0x77,  // vzeroupper
    0xc3,  // ret
];

#[rustfmt::skip]
const YMM_PACKED: &[u8] = &[
    0xc5, 0xfc, 0x10, 0x09,  // vmovups ymm1, ymmword ptr [rcx]
    0xc5, 0xfc, 0x10, 0x51, 0x20,  // vmovups ymm2, ymmword ptr [rcx + 0x20]
    0xc5, 0xf4, 0x54, 0xc2,  // vandps ymm0, ymm1, ymm2
    0xc5, 0xfc, 0x11, 0x81, 0x00, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x100], ymm0
    0xc5, 0xf5, 0xeb, 0x41, 0x20,  // vpor ymm0, ymm1, ymmword ptr [rcx + 0x20]
    0xc5, 0xfc, 0x11, 0x81, 0x20, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x120], ymm0
    0xc5, 0xf5, 0xfe, 0xc2,  // vpaddd ymm0, ymm1, ymm2
    0xc5, 0xfc, 0x11, 0x81, 0x40, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x140], ymm0
    0xc5, 0xfd, 0x70, 0xc1, 0x1b,  // vpshufd ymm0, ymm1, 0x1b ## ymm0 = ymm1[3,2,1,0,7,6,5,4]
    0xc5, 0xfc, 0x11, 0x81, 0x60, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x160], ymm0
    0xc5, 0xf4, 0xc6, 0xc2, 0x44,  // vshufps ymm0, ymm1, ymm2, 0x44 ## ymm0 = ymm1[0,1],ymm2[0,1],ymm1[4,5],ymm2[4,5]
    0xc5, 0xfc, 0x11, 0x81, 0x80, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x180], ymm0
    0xc5, 0xf5, 0x6c, 0xc2,  // vpunpcklqdq ymm0, ymm1, ymm2 ## ymm0 = ymm1[0],ymm2[0],ymm1[2],ymm2[2]
    0xc5, 0xfc, 0x11, 0x81, 0xa0, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x1a0], ymm0
    0xc5, 0xf4, 0x55, 0xc2,  // vandnps ymm0, ymm1, ymm2
    0xc5, 0xfc, 0x11, 0x81, 0xc0, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x1c0], ymm0
    0xc5, 0xf5, 0xfb, 0xc2,  // vpsubq ymm0, ymm1, ymm2
    0xc5, 0xfc, 0x11, 0x81, 0xe0, 0x01, 0x00, 0x00,  // vmovups ymmword ptr [rcx + 0x1e0], ymm0
    0xc5, 0xf8, 0x77,  // vzeroupper
    0xc3,  // ret
];

#[rustfmt::skip]
const MOVSS_FORMS: &[u8] = &[
    0x66, 0x0f, 0x76, 0xe4,  // pcmpeqd xmm4, xmm4
    0xf3, 0x0f, 0x10, 0x21,  // movss xmm4, dword ptr [rcx]
    0xf3, 0x0f, 0x7f, 0xa1, 0x00, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x100], xmm4
    0x66, 0x0f, 0x76, 0xed,  // pcmpeqd xmm5, xmm5
    0xf3, 0x0f, 0x10, 0xea,  // movss xmm5, xmm2 ## xmm5 = xmm2[0],xmm5[1,2,3]
    0xf3, 0x0f, 0x7f, 0xa9, 0x10, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x110], xmm5
    0xc5, 0xf2, 0x10, 0xf2,  // vmovss xmm6, xmm1, xmm2 ## xmm6 = xmm2[0],xmm1[1,2,3]
    0xf3, 0x0f, 0x7f, 0xb1, 0x20, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x120], xmm6
    0x66, 0x0f, 0x76, 0xff,  // pcmpeqd xmm7, xmm7
    0xf2, 0x0f, 0x10, 0x39,  // movsd xmm7, qword ptr [rcx]
    0xf3, 0x0f, 0x7f, 0xb9, 0x30, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x130], xmm7
    0xb8, 0xf9, 0xff, 0xff, 0xff,  // mov eax, 0xfffffff9
    0xc5, 0xf2, 0x2a, 0xe0,  // vcvtsi2ss xmm4, xmm1, eax
    0xf3, 0x0f, 0x7f, 0xa1, 0x40, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x140], xmm4
    0x0f, 0x28, 0xe9,  // movaps xmm5, xmm1
    0xf2, 0x0f, 0x2a, 0xe8,  // cvtsi2sd xmm5, eax
    0xf3, 0x0f, 0x7f, 0xa9, 0x50, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x150], xmm5
    0xc5, 0xf2, 0x5a, 0xf2,  // vcvtss2sd xmm6, xmm1, xmm2
    0xf3, 0x0f, 0x7f, 0xb1, 0x60, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x160], xmm6
    0xc3,  // ret
];

#[rustfmt::skip]
const SIMD_INT_MISC: &[u8] = &[
    0xf3, 0x0f, 0x6f, 0x21,  // movdqu xmm4, xmmword ptr [rcx]
    0x66, 0x0f, 0x71, 0xd4, 0x04,  // psrlw xmm4, 0x4
    0xf3, 0x0f, 0x7f, 0xa1, 0x00, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x100], xmm4
    0xf3, 0x0f, 0x6f, 0x21,  // movdqu xmm4, xmmword ptr [rcx]
    0x66, 0x0f, 0x6e, 0x69, 0x10,  // movd xmm5, dword ptr [rcx + 0x10]
    0x66, 0x0f, 0xf2, 0xe5,  // pslld xmm4, xmm5
    0xf3, 0x0f, 0x7f, 0xa1, 0x10, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x110], xmm4
    0xf3, 0x0f, 0x6f, 0x21,  // movdqu xmm4, xmmword ptr [rcx]
    0x66, 0x0f, 0x72, 0xe4, 0x28,  // psrad xmm4, 0x28
    0xf3, 0x0f, 0x7f, 0xa1, 0x20, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x120], xmm4
    0xf3, 0x0f, 0x6f, 0x21,  // movdqu xmm4, xmmword ptr [rcx]
    0x66, 0x0f, 0xd7, 0xc4,  // pmovmskb eax, xmm4
    0x89, 0x81, 0x30, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x130], eax
    0x0f, 0x50, 0xc4,  // movmskps eax, xmm4
    0x89, 0x81, 0x34, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x134], eax
    0xb8, 0xef, 0xbe, 0x00, 0x00,  // mov eax, 0xbeef
    0x66, 0x0f, 0xc4, 0xe0, 0x03,  // pinsrw xmm4, eax, 0x3
    0x66, 0x0f, 0xc5, 0xd4, 0x03,  // pextrw edx, xmm4, 0x3
    0x89, 0x91, 0x38, 0x01, 0x00, 0x00,  // mov dword ptr [rcx + 0x138], edx
    0xf3, 0x0f, 0x7f, 0xa1, 0x40, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x140], xmm4
    0x0f, 0x5a, 0x71, 0x20,  // cvtps2pd xmm6, qword ptr [rcx + 0x20]
    0xf3, 0x0f, 0x7f, 0xb1, 0x50, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x150], xmm6
    0xc5, 0xfd, 0x5a, 0x79, 0x40,  // vcvtpd2ps xmm7, ymmword ptr [rcx + 0x40]
    0xf3, 0x0f, 0x7f, 0xb9, 0x60, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x160], xmm7
    0xf3, 0x0f, 0x6f, 0x21,  // movdqu xmm4, xmmword ptr [rcx]
    0x66, 0x0f, 0x38, 0x0a, 0x61, 0x60,  // psignd xmm4, xmmword ptr [rcx + 0x60]
    0xf3, 0x0f, 0x7f, 0xa1, 0x70, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x170], xmm4
    0xc3,  // ret
];

#[rustfmt::skip]
const INT_FLAGS: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0xf9,  // stc
    0xb8, 0x05, 0x00, 0x00, 0x00,  // mov eax, 0x5
    0xba, 0xff, 0xff, 0xff, 0xff,  // mov edx, 0xffffffff
    0x11, 0xd0,  // adc eax, edx
    0x41, 0x89, 0x80, 0x00, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x100], eax
    0x41, 0x0f, 0x92, 0x80, 0x04, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x104]
    0xf9,  // stc
    0xb8, 0x05, 0x00, 0x00, 0x00,  // mov eax, 0x5
    0x19, 0xd0,  // sbb eax, edx
    0x41, 0x89, 0x80, 0x08, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x108], eax
    0x41, 0x0f, 0x92, 0x80, 0x0c, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x10c]
    0xb8, 0x01, 0x00, 0x00, 0x00,  // mov eax, 0x1
    0xf3, 0x0f, 0xbc, 0xd0,  // tzcnt edx, eax
    0x41, 0x0f, 0x94, 0x80, 0x10, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x110]
    0x41, 0x0f, 0x92, 0x80, 0x11, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x111]
    0x31, 0xc0,  // xor eax, eax
    0xf3, 0x0f, 0xbc, 0xd0,  // tzcnt edx, eax
    0x41, 0x0f, 0x94, 0x80, 0x12, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x112]
    0x41, 0x0f, 0x92, 0x80, 0x13, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x113]
    0x41, 0x89, 0x90, 0x14, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x114], edx
    0xb8, 0x08, 0x00, 0x00, 0x00,  // mov eax, 0x8
    0xc4, 0xe2, 0x68, 0xf3, 0xd8,  // blsi edx, eax
    0x41, 0x0f, 0x92, 0x80, 0x18, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x118]
    0x31, 0xc0,  // xor eax, eax
    0xc4, 0xe2, 0x68, 0xf3, 0xd8,  // blsi edx, eax
    0x41, 0x0f, 0x92, 0x80, 0x19, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x119]
    0xf8,  // clc
    0xf5,  // cmc
    0x41, 0x0f, 0x92, 0x80, 0x1a, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x11a]
    0x31, 0xc0,  // xor eax, eax
    0xb0, 0xc8,  // mov al, -0x38
    0xba, 0x03, 0x00, 0x00, 0x00,  // mov edx, 0x3
    0xf6, 0xe2,  // mul dl
    0x49, 0x89, 0x80, 0x20, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x120], rax
    0x41, 0x0f, 0x92, 0x80, 0x1b, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x11b]
    0x31, 0xc0,  // xor eax, eax
    0xb0, 0xfd,  // mov al, -0x3
    0xba, 0x05, 0x00, 0x00, 0x00,  // mov edx, 0x5
    0xf6, 0xea,  // imul dl
    0x49, 0x89, 0x80, 0x28, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x128], rax
    0xb8, 0xe8, 0x03, 0x00, 0x00,  // mov eax, 0x3e8
    0x31, 0xd2,  // xor edx, edx
    0x41, 0xb9, 0x07, 0x00, 0x00, 0x00,  // mov r9d, 0x7
    0x66, 0x41, 0xf7, 0xf1,  // div r9w
    0x49, 0x89, 0x80, 0x30, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x130], rax
    0x49, 0x89, 0x90, 0x38, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x138], rdx
    0xb8, 0x64, 0x00, 0x00, 0x00,  // mov eax, 0x64
    0x41, 0xb9, 0x07, 0x00, 0x00, 0x00,  // mov r9d, 0x7
    0x41, 0xf6, 0xf1,  // div r9b
    0x49, 0x89, 0x80, 0x40, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x140], rax
    0xc3,  // ret
];

#[rustfmt::skip]
const REPE_CMPSB: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0x49, 0x8d, 0x30,  // lea rsi, [r8]
    0x49, 0x8d, 0x78, 0x10,  // lea rdi, [r8 + 0x10]
    0xb9, 0x08, 0x00, 0x00, 0x00,  // mov ecx, 0x8
    0xf3, 0xa6,  // rep cmpsb byte ptr [rsi], byte ptr es:[rdi]
    0x49, 0x89, 0x88, 0x00, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x100], rcx
    0x41, 0x0f, 0x94, 0x80, 0x08, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x108]
    0x41, 0x0f, 0x92, 0x80, 0x09, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x109]
    0x4c, 0x29, 0xc6,  // sub rsi, r8
    0x49, 0x89, 0xb0, 0x10, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x110], rsi
    0x49, 0x8d, 0x70, 0x20,  // lea rsi, [r8 + 0x20]
    0x49, 0x8d, 0x78, 0x28,  // lea rdi, [r8 + 0x28]
    0xa7,  // cmpsd dword ptr [rsi], dword ptr es:[rdi]
    0x41, 0x0f, 0x94, 0x80, 0x18, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x118]
    0x4c, 0x29, 0xc7,  // sub rdi, r8
    0x49, 0x89, 0xb8, 0x20, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x120], rdi
    0xc3,  // ret
];

#[rustfmt::skip]
const BITOPS_MISC: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0x41, 0x0f, 0x38, 0xf0, 0x00,  // movbe eax, dword ptr [r8]
    0x41, 0x89, 0x80, 0x00, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x100], eax
    0x48, 0xba, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,  // movabs rdx, 0x102030405060708
    0x49, 0x0f, 0x38, 0xf1, 0x90, 0x08, 0x01, 0x00, 0x00,  // movbe qword ptr [r8 + 0x108], rdx
    0x31, 0xc0,  // xor eax, eax
    0xf9,  // stc
    0x9f,  // lahf
    0x41, 0x89, 0x80, 0xa0, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x1a0], eax
    0xb4, 0x81,  // mov ah, -0x7f
    0x9e,  // sahf
    0x41, 0x0f, 0x98, 0x80, 0x11, 0x01, 0x00, 0x00,  // sets byte ptr [r8 + 0x111]
    0x41, 0x0f, 0x92, 0x80, 0x12, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x112]
    0x41, 0x0f, 0x94, 0x80, 0x13, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x113]
    0x41, 0x0f, 0x9a, 0x80, 0x14, 0x01, 0x00, 0x00,  // setp byte ptr [r8 + 0x114]
    0xf9,  // stc
    0xb0, 0x81,  // mov al, -0x7f
    0xd0, 0xd0,  // rcl al
    0x41, 0x88, 0x80, 0x18, 0x01, 0x00, 0x00,  // mov byte ptr [r8 + 0x118], al
    0x41, 0x0f, 0x92, 0x80, 0x19, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x119]
    0x41, 0x0f, 0x90, 0x80, 0x1a, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x11a]
    0xf8,  // clc
    0xb8, 0x03, 0x00, 0x00, 0x00,  // mov eax, 0x3
    0xc1, 0xd8, 0x02,  // rcr eax, 0x2
    0x41, 0x89, 0x80, 0x1c, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x11c], eax
    0x41, 0x0f, 0x92, 0x80, 0x20, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x120]
    0x48, 0xb8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,  // movabs rax, -0x7fffffffffffffff
    0xf9,  // stc
    0x48, 0xc1, 0xd0, 0x03,  // rcl rax, 0x3
    0x49, 0x89, 0x80, 0x28, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x128], rax
    0x41, 0x0f, 0x92, 0x80, 0x30, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x130]
    0xb0, 0x5a,  // mov al, 0x5a
    0xf8,  // clc
    0xb1, 0x09,  // mov cl, 0x9
    0xd2, 0xd0,  // rcl al, cl
    0x41, 0x88, 0x80, 0x31, 0x01, 0x00, 0x00,  // mov byte ptr [r8 + 0x131], al
    0xb8, 0x34, 0x12, 0xcd, 0xab,  // mov eax, 0xabcd1234
    0xba, 0x04, 0x08, 0x00, 0x00,  // mov edx, 0x804
    0xc4, 0x62, 0x68, 0xf7, 0xc8,  // bextr r9d, eax, edx
    0x45, 0x89, 0x88, 0x38, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x138], r9d
    0x41, 0x0f, 0x94, 0x80, 0x3c, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x13c]
    0xba, 0x40, 0x20, 0x00, 0x00,  // mov edx, 0x2040
    0xc4, 0x62, 0x68, 0xf7, 0xc8,  // bextr r9d, eax, edx
    0x45, 0x89, 0x88, 0x40, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x140], r9d
    0x41, 0x0f, 0x94, 0x80, 0x44, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x144]
    0x48, 0xc7, 0xc0, 0x78, 0x56, 0x34, 0x12,  // mov rax, 0x12345678
    0x48, 0xba, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff,  // movabs rdx, -0xff00ff00ff0100
    0xc4, 0x62, 0xfb, 0xf5, 0xca,  // pdep r9, rax, rdx
    0x4d, 0x89, 0x88, 0x48, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x148], r9
    0x48, 0xb8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11,  // movabs rax, 0x1122334455667788
    0xc4, 0x62, 0xfa, 0xf5, 0xca,  // pext r9, rax, rdx
    0x4d, 0x89, 0x88, 0x50, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x150], r9
    0x49, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff,  // mov r9, -0x1
    0xc4, 0x62, 0x7a, 0xf5, 0xca,  // pext r9d, eax, edx
    0x4d, 0x89, 0x88, 0x58, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x158], r9
    0x31, 0xc0,  // xor eax, eax
    0xf9,  // stc
    0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff,  // mov rax, -0x1
    0x48, 0xc7, 0xc2, 0x00, 0x00, 0x00, 0x00,  // mov rdx, 0x0
    0x66, 0x48, 0x0f, 0x38, 0xf6, 0xc2,  // adcx rax, rdx
    0x49, 0x89, 0x80, 0x60, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x160], rax
    0x41, 0x0f, 0x92, 0x80, 0x68, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x168]
    0x41, 0x0f, 0x90, 0x80, 0x69, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x169]
    0xb8, 0xff, 0xff, 0xff, 0x7f,  // mov eax, 0x7fffffff
    0x83, 0xc0, 0x01,  // add eax, 0x1
    0x48, 0xc7, 0xc0, 0x05, 0x00, 0x00, 0x00,  // mov rax, 0x5
    0x48, 0xc7, 0xc2, 0x06, 0x00, 0x00, 0x00,  // mov rdx, 0x6
    0xf3, 0x48, 0x0f, 0x38, 0xf6, 0xc2,  // adox rax, rdx
    0x49, 0x89, 0x80, 0x70, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x170], rax
    0x41, 0x0f, 0x90, 0x80, 0x78, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x178]
    0x41, 0x0f, 0x92, 0x80, 0x79, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x179]
    0xb8, 0x01, 0x00, 0x00, 0x00,  // mov eax, 0x1
    0x31, 0xd2,  // xor edx, edx
    0x0f, 0xac, 0xd0, 0x01,  // shrd eax, edx, 0x1
    0x41, 0x0f, 0x92, 0x80, 0x7a, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x17a]
    0xb8, 0xff, 0xff, 0xff, 0xff,  // mov eax, 0xffffffff
    0xf2, 0x49, 0x0f, 0x38, 0xf1, 0x40, 0x20,  // crc32 rax, qword ptr [r8 + 0x20]
    0xf2, 0x41, 0x0f, 0x38, 0xf0, 0x40, 0x28,  // crc32 eax, byte ptr [r8 + 0x28]
    0xf7, 0xd0,  // not eax
    0x41, 0x89, 0x80, 0x80, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x180], eax
    0x49, 0x8d, 0x58, 0x30,  // lea rbx, [r8 + 0x30]
    0xb8, 0x02, 0x45, 0x23, 0x01,  // mov eax, 0x1234502
    0xd7,  // xlatb
    0x41, 0x89, 0x80, 0x84, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x184], eax
    0x31, 0xc0,  // xor eax, eax
    0xb9, 0x05, 0x00, 0x00, 0x00,  // mov ecx, 0x5
    0x83, 0xc0, 0x02,  // add eax, 0x2
    0xe2, 0xfb,  // loop 0x1d5 <.Lloop>
    0x41, 0x89, 0x80, 0x88, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x188], eax
    0x31, 0xc9,  // xor ecx, ecx
    0xb8, 0x01, 0x00, 0x00, 0x00,  // mov eax, 0x1
    0xe3, 0x05,  // jrcxz +5 (skip the following mov eax, imm32)
    0xb8, 0x07, 0x00, 0x00, 0x00,  // mov eax, 0x7
    0x41, 0x89, 0x80, 0x8c, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x18c], eax
    0xb9, 0x01, 0x00, 0x00, 0x00,  // mov ecx, 0x1
    0xb8, 0x01, 0x00, 0x00, 0x00,  // mov eax, 0x1
    0xe3, 0x05,  // jrcxz +5 (skip the following mov eax, imm32)
    0xb8, 0x09, 0x00, 0x00, 0x00,  // mov eax, 0x9
    0x41, 0x89, 0x80, 0x90, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x190], eax
    0x0f, 0x01, 0xf9,  // rdtscp
    0x49, 0x89, 0x88, 0x98, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x198], rcx
    0xc3,  // ret
];

#[rustfmt::skip]
const STRING_DF: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0x49, 0x8d, 0x30,  // lea rsi, [r8]
    0xac,  // lodsb al, byte ptr [rsi]
    0x41, 0x88, 0x80, 0x00, 0x01, 0x00, 0x00,  // mov byte ptr [r8 + 0x100], al
    0x49, 0x8d, 0x30,  // lea rsi, [r8]
    0xb9, 0x03, 0x00, 0x00, 0x00,  // mov ecx, 0x3
    0xf3, 0xad,  // rep lodsd eax, dword ptr [rsi]
    0x41, 0x89, 0x80, 0x04, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x104], eax
    0x4c, 0x29, 0xc6,  // sub rsi, r8
    0x49, 0x89, 0xb0, 0x08, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x108], rsi
    0xfd,  // std
    0x49, 0x8d, 0xb8, 0x13, 0x01, 0x00, 0x00,  // lea rdi, [r8 + 0x113]
    0xb0, 0xab,  // mov al, -0x55
    0xb9, 0x04, 0x00, 0x00, 0x00,  // mov ecx, 0x4
    0xf3, 0xaa,  // rep stosb byte ptr es:[rdi], al
    0x4c, 0x29, 0xc7,  // sub rdi, r8
    0x49, 0x89, 0xb8, 0x18, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x118], rdi
    0x49, 0x8d, 0x70, 0x0c,  // lea rsi, [r8 + 0xc]
    0x49, 0x8d, 0xb8, 0x2c, 0x01, 0x00, 0x00,  // lea rdi, [r8 + 0x12c]
    0xb9, 0x04, 0x00, 0x00, 0x00,  // mov ecx, 0x4
    0xf3, 0xa5,  // rep movsd dword ptr es:[rdi], dword ptr [rsi]
    0x49, 0x8d, 0x70, 0x08,  // lea rsi, [r8 + 0x8]
    0x48, 0xad,  // lodsq rax, qword ptr [rsi]
    0x49, 0x89, 0x80, 0x30, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x130], rax
    0x4c, 0x29, 0xc6,  // sub rsi, r8
    0x49, 0x89, 0xb0, 0x38, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x138], rsi
    0x49, 0x8d, 0x78, 0x07,  // lea rdi, [r8 + 0x7]
    0x41, 0x8a, 0x40, 0x02,  // mov al, byte ptr [r8 + 0x2]
    0xb9, 0x08, 0x00, 0x00, 0x00,  // mov ecx, 0x8
    0xf2, 0xae,  // repne scasb al, byte ptr es:[rdi]
    0x49, 0x89, 0x88, 0x40, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x140], rcx
    0x4c, 0x29, 0xc7,  // sub rdi, r8
    0x49, 0x89, 0xb8, 0x48, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x148], rdi
    0xfc,  // cld
    0x49, 0x8d, 0xb8, 0x50, 0x01, 0x00, 0x00,  // lea rdi, [r8 + 0x150]
    0xb0, 0xcd,  // mov al, -0x33
    0xaa,  // stosb byte ptr es:[rdi], al
    0x4c, 0x29, 0xc7,  // sub rdi, r8
    0x49, 0x89, 0xb8, 0x58, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x158], rdi
    0xc3,  // ret
];

#[rustfmt::skip]
const SIMD_NEW: &[u8] = &[
    0xf3, 0x0f, 0x6f, 0x21,  // movdqu xmm4, xmmword ptr [rcx]
    0x66, 0x0f, 0x3a, 0x0e, 0x61, 0x10, 0xa5,  // pblendw xmm4, xmmword ptr [rcx + 0x10], 0xa5 ## xmm4 = mem[0],xmm4[1],mem[2],xmm4[3,4],mem[5],xmm4[6],mem[7]
    0xf3, 0x0f, 0x7f, 0xa1, 0x00, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x100], xmm4
    0xc5, 0xfe, 0x6f, 0x09,  // vmovdqu ymm1, ymmword ptr [rcx]
    0xc5, 0xfe, 0x6f, 0x51, 0x20,  // vmovdqu ymm2, ymmword ptr [rcx + 0x20]
    0xc4, 0xe3, 0x75, 0x0e, 0xda, 0x0f,  // vpblendw ymm3, ymm1, ymm2, 0xf ## ymm3 = ymm2[0,1,2,3],ymm1[4,5,6,7],ymm2[8,9,10,11],ymm1[12,13,14,15]
    0xc5, 0xfe, 0x7f, 0x99, 0x20, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x120], ymm3
    0xc4, 0xe3, 0x75, 0x02, 0xda, 0x5a,  // vpblendd ymm3, ymm1, ymm2, 0x5a ## ymm3 = ymm1[0],ymm2[1],ymm1[2],ymm2[3,4],ymm1[5],ymm2[6],ymm1[7]
    0xc5, 0xfe, 0x7f, 0x99, 0x40, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x140], ymm3
    0xf3, 0x0f, 0x6f, 0x29,  // movdqu xmm5, xmmword ptr [rcx]
    0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff,  // mov rax, -0x1
    0x66, 0x0f, 0x3a, 0x17, 0xe8, 0x02,  // extractps eax, xmm5, 0x2
    0x48, 0x89, 0x81, 0x60, 0x01, 0x00, 0x00,  // mov qword ptr [rcx + 0x160], rax
    0x66, 0x0f, 0x3a, 0x17, 0xa9, 0x68, 0x01, 0x00, 0x00, 0x03,  // extractps dword ptr [rcx + 0x168], xmm5, 0x3
    0xc5, 0xfe, 0x6f, 0x71, 0x40,  // vmovdqu ymm6, ymmword ptr [rcx + 0x40]
    0xc4, 0xe2, 0x4d, 0x36, 0xf9,  // vpermd ymm7, ymm6, ymm1
    0xc5, 0xfe, 0x7f, 0xb9, 0x80, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x180], ymm7
    0xc4, 0xe3, 0xfd, 0x00, 0xf9, 0x1b,  // vpermq ymm7, ymm1, 0x1b ## ymm7 = ymm1[3,2,1,0]
    0xc5, 0xfe, 0x7f, 0xb9, 0xa0, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x1a0], ymm7
    0xc5, 0xfe, 0x6f, 0x71, 0x60,  // vmovdqu ymm6, ymmword ptr [rcx + 0x60]
    0xc4, 0xe2, 0x75, 0x47, 0xfe,  // vpsllvd ymm7, ymm1, ymm6
    0xc5, 0xfe, 0x7f, 0xb9, 0xc0, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x1c0], ymm7
    0xc4, 0xe2, 0x71, 0x46, 0xfe,  // vpsravd xmm7, xmm1, xmm6
    0xc5, 0xfe, 0x7f, 0xb9, 0xe0, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x1e0], ymm7
    0xc5, 0xf8, 0x77,  // vzeroupper
    0xc3,  // ret
];

#[rustfmt::skip]
const SIMD_NEW2: &[u8] = &[
    0xc5, 0xfe, 0x6f, 0x09,  // vmovdqu ymm1, ymmword ptr [rcx]
    0xc5, 0xfe, 0x6f, 0x71, 0x60,  // vmovdqu ymm6, ymmword ptr [rcx + 0x60]
    0xc4, 0xe2, 0x75, 0x45, 0xfe,  // vpsrlvd ymm7, ymm1, ymm6
    0xc5, 0xfe, 0x7f, 0xb9, 0x00, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x100], ymm7
    0xc5, 0xfe, 0x6f, 0xb1, 0x80, 0x00, 0x00, 0x00,  // vmovdqu ymm6, ymmword ptr [rcx + 0x80]
    0xc4, 0xe2, 0xf5, 0x47, 0xfe,  // vpsllvq ymm7, ymm1, ymm6
    0xc5, 0xfe, 0x7f, 0xb9, 0x20, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x120], ymm7
    0xc4, 0xe2, 0xf5, 0x45, 0xb9, 0x80, 0x00, 0x00, 0x00,  // vpsrlvq ymm7, ymm1, ymmword ptr [rcx + 0x80]
    0xc5, 0xfe, 0x7f, 0xb9, 0x40, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x140], ymm7
    0xf2, 0x0f, 0xf0, 0x51, 0x10,  // lddqu xmm2, xmmword ptr [rcx + 0x10]
    0xf3, 0x0f, 0x7f, 0x91, 0x60, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [rcx + 0x160], xmm2
    0xc5, 0xfe, 0x6f, 0x19,  // vmovdqu ymm3, ymmword ptr [rcx]
    0xc4, 0xe3, 0x61, 0x0a, 0x99, 0xa0, 0x00, 0x00, 0x00, 0x01,  // vroundss xmm3, xmm3, dword ptr [rcx + 0xa0], 0x1
    0xc5, 0xfe, 0x7f, 0x99, 0x70, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x170], ymm3
    0xc4, 0xe3, 0xfd, 0x01, 0x21, 0xb1,  // vpermpd ymm4, ymmword ptr [rcx], 0xb1 ## ymm4 = mem[1,0,3,2]
    0xc5, 0xfe, 0x7f, 0xa1, 0x90, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x190], ymm4
    0xc5, 0xfe, 0x6f, 0x71, 0x40,  // vmovdqu ymm6, ymmword ptr [rcx + 0x40]
    0xc4, 0xe2, 0x4d, 0x16, 0x29,  // vpermps ymm5, ymm6, ymmword ptr [rcx]
    0xc5, 0xfe, 0x7f, 0xa9, 0xb0, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x1b0], ymm5
    0xc5, 0xf8, 0x77,  // vzeroupper
    0xc3,  // ret
];

#[rustfmt::skip]
const FLAGS_HADD_VTEST: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0x31, 0xc0,  // xor eax, eax
    0xf9,  // stc
    0xfd,  // std
    0x9c,  // pushfq
    0x58,  // pop rax
    0x49, 0x89, 0x80, 0x00, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x100], rax
    0xb8, 0x80, 0x08, 0x00, 0x00,  // mov eax, 0x880
    0x50,  // push rax
    0x9d,  // popfq
    0x41, 0x0f, 0x98, 0x80, 0x08, 0x01, 0x00, 0x00,  // sets byte ptr [r8 + 0x108]
    0x41, 0x0f, 0x90, 0x80, 0x09, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x109]
    0x41, 0x0f, 0x94, 0x80, 0x0a, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x10a]
    0x41, 0x0f, 0x92, 0x80, 0x0b, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x10b]
    0x49, 0x8d, 0xb8, 0x10, 0x01, 0x00, 0x00,  // lea rdi, [r8 + 0x110]
    0xaa,  // stosb byte ptr es:[rdi], al
    0x4c, 0x29, 0xc7,  // sub rdi, r8
    0x49, 0x89, 0xb8, 0x18, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x118], rdi
    0x66, 0x9c,  // pushf
    0x66, 0x58,  // pop ax
    0x66, 0x41, 0x89, 0x80, 0x20, 0x01, 0x00, 0x00,  // mov word ptr [r8 + 0x120], ax
    0xf3, 0x41, 0x0f, 0x6f, 0x08,  // movdqu xmm1, xmmword ptr [r8]
    0xf3, 0x41, 0x0f, 0x6f, 0x50, 0x10,  // movdqu xmm2, xmmword ptr [r8 + 0x10]
    0x66, 0x0f, 0x6f, 0xd9,  // movdqa xmm3, xmm1
    0x66, 0x0f, 0x38, 0x02, 0xda,  // phaddd xmm3, xmm2
    0xf3, 0x41, 0x0f, 0x7f, 0x98, 0x30, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [r8 + 0x130], xmm3
    0x66, 0x0f, 0x6f, 0xd9,  // movdqa xmm3, xmm1
    0x66, 0x41, 0x0f, 0x38, 0x05, 0x58, 0x10,  // phsubw xmm3, xmmword ptr [r8 + 0x10]
    0xf3, 0x41, 0x0f, 0x7f, 0x98, 0x40, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [r8 + 0x140], xmm3
    0xc4, 0xc1, 0x7e, 0x6f, 0x20,  // vmovdqu ymm4, ymmword ptr [r8]
    0xc4, 0xc1, 0x7e, 0x6f, 0x68, 0x20,  // vmovdqu ymm5, ymmword ptr [r8 + 0x20]
    0xc4, 0xe2, 0x5d, 0x01, 0xf5,  // vphaddw ymm6, ymm4, ymm5
    0xc4, 0xc1, 0x7e, 0x7f, 0xb0, 0x60, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [r8 + 0x160], ymm6
    0xc4, 0xe2, 0x59, 0x06, 0xf5,  // vphsubd xmm6, xmm4, xmm5
    0xc4, 0xc1, 0x7e, 0x7f, 0xb0, 0x80, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [r8 + 0x180], ymm6
    0xc4, 0xc2, 0x7d, 0x0e, 0x60, 0x20,  // vtestps ymm4, ymmword ptr [r8 + 0x20]
    0x41, 0x0f, 0x94, 0x80, 0xa0, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x1a0]
    0x41, 0x0f, 0x92, 0x80, 0xa1, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x1a1]
    0xc4, 0xe2, 0x79, 0x0f, 0xe4,  // vtestpd xmm4, xmm4
    0x41, 0x0f, 0x94, 0x80, 0xa2, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x1a2]
    0x41, 0x0f, 0x92, 0x80, 0xa3, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x1a3]
    0xc4, 0xc1, 0x7e, 0x6f, 0x78, 0x40,  // vmovdqu ymm7, ymmword ptr [r8 + 0x40]
    0xc4, 0xe2, 0x7d, 0x0e, 0xfc,  // vtestps ymm7, ymm4
    0x41, 0x0f, 0x94, 0x80, 0xa4, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x1a4]
    0x41, 0x0f, 0x92, 0x80, 0xa5, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x1a5]
    0xc5, 0xf8, 0x77,  // vzeroupper
    0xc3,  // ret
];

#[rustfmt::skip]
const MASKMOV: &[u8] = &[
    0xc5, 0xfe, 0x6f, 0x79, 0x40,  // vmovdqu ymm7, ymmword ptr [rcx + 0x40]
    0xc4, 0xe2, 0x45, 0x8c, 0x01,  // vpmaskmovd ymm0, ymm7, ymmword ptr [rcx]
    0xc5, 0xfe, 0x7f, 0x81, 0x00, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x100], ymm0
    0xc4, 0xe2, 0x41, 0x2d, 0x09,  // vmaskmovpd xmm1, xmm7, xmmword ptr [rcx]
    0xc5, 0xfe, 0x7f, 0x89, 0x20, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x120], ymm1
    0xc5, 0xfe, 0x6f, 0x51, 0x20,  // vmovdqu ymm2, ymmword ptr [rcx + 0x20]
    0xc4, 0xe2, 0x45, 0x8e, 0x91, 0x40, 0x01, 0x00, 0x00,  // vpmaskmovd ymmword ptr [rcx + 0x140], ymm7, ymm2
    0xc4, 0xe2, 0x45, 0x2f, 0x91, 0x60, 0x01, 0x00, 0x00,  // vmaskmovpd ymmword ptr [rcx + 0x160], ymm7, ymm2
    0xc4, 0xe2, 0xc1, 0x8e, 0x91, 0x80, 0x01, 0x00, 0x00,  // vpmaskmovq xmmword ptr [rcx + 0x180], xmm7, xmm2
    0xc4, 0xe2, 0x41, 0x2c, 0x59, 0x20,  // vmaskmovps xmm3, xmm7, xmmword ptr [rcx + 0x20]
    0xc5, 0xfe, 0x7f, 0x99, 0xa0, 0x01, 0x00, 0x00,  // vmovdqu ymmword ptr [rcx + 0x1a0], ymm3
    0xc5, 0xf8, 0x77,  // vzeroupper
    0xc3,  // ret
];

/// Bump arena for code + data. Never freed: the per-thread BlockCache is keyed by guest IP,
/// so addresses must never be reused. Must avoid the remapped image range.
static ARENA: Mutex<(usize, usize)> = Mutex::new((0, 0));
const ARENA_SIZE: usize = 1 << 20;

fn arena_alloc(len: usize, align: usize) -> *mut u8 {
    let mut a = ARENA.lock().unwrap();
    loop {
        if a.0 != 0 {
            let start = (a.0 + a.1 + align - 1) & !(align - 1);
            if start + len <= a.0 + ARENA_SIZE {
                a.1 = start + len - a.0;
                return start as *mut u8;
            }
        }
        let mem: &'static mut [u8] = Box::leak(vec![0u8; ARENA_SIZE].into_boxed_slice());
        let p = mem.as_mut_ptr() as u64;
        let end = p + ARENA_SIZE as u64;
        if end > 0x1_4000_0000 && p < 0x1_4500_0000 {
            continue; // overlaps the remapped image window; leak and retry
        }
        *a = (p as usize, 0);
    }
}

#[repr(C, align(32))]
struct Buf([u8; 512]);

impl Buf {
    fn u32(&self, o: usize) -> u32 {
        u32::from_le_bytes(self.0[o..o + 4].try_into().unwrap())
    }
    fn u64(&self, o: usize) -> u64 {
        u64::from_le_bytes(self.0[o..o + 8].try_into().unwrap())
    }
    fn u128(&self, o: usize) -> u128 {
        u128::from_le_bytes(self.0[o..o + 16].try_into().unwrap())
    }
    fn set_u32(&mut self, o: usize, v: u32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn set_u64(&mut self, o: usize, v: u64) {
        self.0[o..o + 8].copy_from_slice(&v.to_le_bytes());
    }
    fn set_u128(&mut self, o: usize, v: u128) {
        self.0[o..o + 16].copy_from_slice(&v.to_le_bytes());
    }
    fn f32s(&self, o: usize) -> [f32; 4] {
        core::array::from_fn(|i| f32::from_bits(self.u32(o + 4 * i)))
    }
    fn u32s(&self, o: usize) -> [u32; 4] {
        core::array::from_fn(|i| self.u32(o + 4 * i))
    }
}

fn ps(v: [f32; 4]) -> u128 {
    v.iter().enumerate().fold(0u128, |acc, (i, f)| acc | ((f.to_bits() as u128) << (32 * i)))
}

fn dq(v: [u32; 4]) -> u128 {
    v.iter().enumerate().fold(0u128, |acc, (i, d)| acc | ((*d as u128) << (32 * i)))
}

/// Run `code` (must end in `ret`) with RCX = scratch buffer (copied in/out of the arena),
/// verifying that the interpreter (`jit::force(2)`) and the block translator (`jit::force(1)`)
/// produce identical output.
fn run(code: &[u8], buf: &mut Buf, xmm: [u128; 4]) {
    let c = arena_alloc(code.len() + 16, 16);
    let d_interp = arena_alloc(core::mem::size_of::<Buf>(), 32) as *mut Buf;
    let d_jit = arena_alloc(core::mem::size_of::<Buf>(), 32) as *mut Buf;
    unsafe {
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        core::ptr::write_bytes(c.add(code.len()), 0xCC, 16);
        core::ptr::copy_nonoverlapping(buf as *const Buf, d_interp, 1);
        core::ptr::copy_nonoverlapping(buf as *const Buf, d_jit, 1);
        #[cfg(target_arch = "aarch64")]
        jit::force(2);
        let r_interp = emu_run_full(c as u64, [d_interp as u64, 0, 0, 0, 0, 0, 0, 0], xmm, [0; 8]);
        #[cfg(target_arch = "aarch64")]
        {
            jit::force(1);
            let r_jit = emu_run_full(c as u64, [d_jit as u64, 0, 0, 0, 0, 0, 0, 0], xmm, [0; 8]);
            jit::force(0);
            assert_eq!(r_jit, r_interp, "jit return (rax, xmm0) differed from interpreter");
            assert_eq!((*d_jit).0, (*d_interp).0, "jit scratch buffer differed from interpreter");
        }
        core::ptr::copy_nonoverlapping(d_interp as *const Buf, buf as *mut Buf, 1);
    }
}

fn new_buf() -> Buf {
    Buf([0u8; 512])
}

#[test]
fn pxor_forms() {
    let mut b = new_buf();
    let a = 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210u128;
    let bb = 0xffff_0000_ffff_0000_1234_5678_9abc_def0u128;
    b.set_u128(0, a);
    b.set_u128(0x10, bb);
    let x0 = 0x1111_2222_3333_4444_5555_6666_7777_8888u128;
    let x1 = 0xaaaa_bbbb_cccc_dddd_eeee_ffff_0000_1111u128;
    run(PXOR_FORMS, &mut b, [x0, x1, 0, 0]);
    assert_eq!(b.u128(0x100), x0 ^ x1);
    assert_eq!(b.u128(0x110), a ^ bb);
    assert_eq!(b.u128(0x120), x1 ^ a);
}

#[test]
fn scalar_arith_forms() {
    let mut b = new_buf();
    b.set_u32(0, 0.5f32.to_bits());
    let x0 = ps([100.0, 7.0, 8.0, 9.0]);
    let x1 = ps([1.0, 10.0, 20.0, 30.0]);
    let x2 = ps([2.0, 99.0, 99.0, 99.0]);
    run(SCALAR_ARITH, &mut b, [x0, x1, x2, 0]);
    assert_eq!(b.f32s(0x100), [3.0, 10.0, 20.0, 30.0], "vaddss upper from src1");
    assert_eq!(b.f32s(0x110), [102.0, 7.0, 8.0, 9.0], "addss upper from dst");
    assert_eq!(b.f32s(0x120), [1.5, 10.0, 20.0, 30.0], "addss m32");
    let lo = f64::from_bits(x1 as u64) - f64::from_bits(x2 as u64);
    assert_eq!(b.u64(0x130), lo.to_bits(), "vsubsd low");
    assert_eq!(b.u64(0x138), (x1 >> 64) as u64, "vsubsd upper from src1");
}

#[test]
fn scalar_special_values() {
    let mut b = new_buf();
    b.set_u32(0, 1.0f32.to_bits());
    b.set_u32(4, 0.0f32.to_bits());
    b.set_u32(8, (-1.0f32).to_bits());
    b.set_u32(0xc, 0x7FC0_0001);
    b.set_u64(0x10, 1.0f64.to_bits());
    b.set_u64(0x18, 0.0f64.to_bits());
    let x1 = ps([1.0, 10.0, 20.0, 30.0]);
    let x2 = ps([4.0, 99.0, 99.0, 99.0]);
    run(SCALAR_SPECIAL, &mut b, [0, x1, x2, 0]);
    assert_eq!(b.u32(0x100), 0x7F80_0000, "1/0 = +inf");
    assert_eq!(b.u32(0x104), 0xFFC0_0000, "0/0 = default NaN");
    assert_eq!(b.u32(0x108), 0xFFC0_0000, "sqrt(-1) = default NaN");
    assert_eq!(b.u32(0x10c), 0x3F80_0000, "minss(NaN, 1) = src");
    assert_eq!(b.u32(0x110), 0x7FC0_0001, "minss(1, NaN) = src NaN");
    assert_eq!(b.u32(0x114), 0x3F80_0000, "maxss(NaN, 1) = src");
    assert_eq!(b.f32s(0x120), [2.0, 10.0, 20.0, 30.0], "vsqrtss upper from src1");
    assert_eq!(b.u64(0x130), 0x7FF0_0000_0000_0000, "1.0/0.0 sd = +inf");
}

#[test]
fn cvt_to_int_indefinite_and_rounding() {
    let mut b = new_buf();
    for (i, f) in [3.7f32, -3.7, 3e9, f32::NAN, 2.5, 3.5].iter().enumerate() {
        b.set_u32(4 * i, f.to_bits());
    }
    b.set_u64(0x18, 1e19f64.to_bits());
    b.set_u64(0x20, (-2.5f64).to_bits());
    run(CVT_TO_INT, &mut b, [0; 4]);
    assert_eq!(b.u32(0x100), 3);
    assert_eq!(b.u32(0x104), 0xFFFF_FFFD);
    assert_eq!(b.u32(0x108), 0x8000_0000, "out of range -> indefinite");
    assert_eq!(b.u32(0x10c), 0x8000_0000, "NaN -> indefinite");
    assert_eq!(b.u32(0x110), 2, "cvtss2si 2.5 rounds to even");
    assert_eq!(b.u32(0x114), 4, "cvtss2si 3.5 rounds to even");
    assert_eq!(b.u64(0x118), 0x8000_0000_0000_0000, "1e19 -> 64-bit indefinite");
    assert_eq!(b.u64(0x120), 0xFFFF_FFFF_FFFF_FFFD);
    assert_eq!(b.u32(0x128), 0xFFFF_FFFE, "cvtsd2si -2.5 rounds to even");
}

#[test]
fn ucomiss_flags() {
    let mut b = new_buf();
    b.set_u64(0, 1.0f64.to_bits());
    let x0 = f32::NAN.to_bits() as u128;
    let x1 = 1.0f32.to_bits() as u128;
    let x2 = 0.5f32.to_bits() as u128;
    let x3 = 2.0f64.to_bits() as u128;
    run(UCOMISS_FLAGS, &mut b, [x0, x1, x2, x3]);
    assert_eq!(&b.0[0x100..0x105], &[1, 1, 1, 0, 0], "unordered: ZF PF CF set, OF SF clear");
    assert_eq!(&b.0[0x108..0x10b], &[0, 0, 1], "0.5 < 1.0");
    assert_eq!(b.0[0x10c], 1, "ucomisd 2.0 > 1.0");
}

#[test]
fn cmpps_predicates() {
    let mut b = new_buf();
    b.set_u32(0, 2.0f32.to_bits());
    let x1 = ps([1.0, 2.0, 3.0, f32::NAN]);
    let x2 = ps([2.0; 4]);
    run(CMPPS_PREDS, &mut b, [0, x1, x2, 0]);
    let m = |bits: [u32; 4]| bits.map(|x| if x != 0 { u32::MAX } else { 0 });
    assert_eq!(b.u32s(0x100), m([0, 0, 1, 0]), "GT_OS");
    assert_eq!(b.u32s(0x110), m([0, 1, 1, 0]), "GE_OS");
    assert_eq!(b.u32s(0x120), m([0, 1, 0, 1]), "EQ_UQ");
    assert_eq!(b.u32s(0x130), m([1, 0, 1, 0]), "NEQ_OQ");
    assert_eq!(b.u32s(0x140), m([1, 0, 0, 1]), "NGE_US");
    assert_eq!(b.u32s(0x150), m([1, 1, 0, 1]), "NGT_US");
    assert_eq!(b.u32s(0x160), m([1, 0, 0, 0]), "LT_OS");
    assert_eq!(b.u32s(0x170), m([0, 1, 1, 1]), "NLT_US");
    let exp = [0, (x1 >> 32) as u32, (x1 >> 64) as u32, (x1 >> 96) as u32];
    assert_eq!(b.u32s(0x180), exp, "vcmpgt_oqss m32, upper from src1");
}

#[test]
fn vex128_zeroes_upper() {
    let mut b = new_buf();
    for i in 0..32 {
        b.0[i] = 0xFF;
    }
    b.set_u32(0x40, 5.0f32.to_bits());
    let x1 = ps([1.0, 2.0, 3.0, 4.0]);
    let x2 = ps([5.0, 6.0, 7.0, 8.0]);
    run(VEX128_ZERO_UPPER, &mut b, [0, x1, x2, 0]);
    for k in 0..5 {
        assert_eq!(b.u128(0x110 + 0x20 * k), 0, "ymm upper of output {k} not zeroed");
    }
    assert_eq!(b.u128(0x100), x1, "vmovaps");
    assert_eq!(b.f32s(0x120), [6.0, 8.0, 10.0, 12.0], "vaddps");
    assert_eq!(b.u128(0x140), x1 ^ x2, "vxorps");
    assert_eq!(b.f32s(0x160), [6.0, 2.0, 3.0, 4.0], "vaddss");
    assert_eq!(b.f32s(0x180), [5.0, 0.0, 0.0, 0.0], "vmovss load");
}

#[test]
fn ymm_packed_ops() {
    let mut b = new_buf();
    let y1: [u32; 8] = core::array::from_fn(|i| 0x0F0F_0F00 + i as u32);
    let y2: [u32; 8] = core::array::from_fn(|i| 0x00FF_00F0 + 16 * i as u32);
    for i in 0..8 {
        b.set_u32(4 * i, y1[i]);
        b.set_u32(0x20 + 4 * i, y2[i]);
    }
    run(YMM_PACKED, &mut b, [0; 4]);
    let out = |k: usize| -> [u32; 8] { core::array::from_fn(|i| b.u32(0x100 + 0x20 * k + 4 * i)) };
    let zip = |f: &dyn Fn(u32, u32) -> u32| -> [u32; 8] { core::array::from_fn(|i| f(y1[i], y2[i])) };
    assert_eq!(out(0), zip(&|a, c| a & c), "vandps ymm");
    assert_eq!(out(1), zip(&|a, c| a | c), "vpor ymm m256");
    assert_eq!(out(2), zip(&|a, c| a.wrapping_add(c)), "vpaddd ymm");
    let rev: [u32; 8] = [y1[3], y1[2], y1[1], y1[0], y1[7], y1[6], y1[5], y1[4]];
    assert_eq!(out(3), rev, "vpshufd ymm");
    let lohalf: [u32; 8] = [y1[0], y1[1], y2[0], y2[1], y1[4], y1[5], y2[4], y2[5]];
    assert_eq!(out(4), lohalf, "vshufps ymm 0x44");
    assert_eq!(out(5), lohalf, "vpunpcklqdq ymm");
    assert_eq!(out(6), zip(&|a, c| !a & c), "vandnps ymm");
    let q = |v: &[u32; 8], i: usize| (v[2 * i] as u64) | ((v[2 * i + 1] as u64) << 32);
    let o7 = out(7);
    for i in 0..4 {
        assert_eq!(q(&o7, i), q(&y1, i).wrapping_sub(q(&y2, i)), "vpsubq ymm lane {i}");
    }
}

#[test]
fn movss_and_scalar_cvt_forms() {
    let mut b = new_buf();
    b.set_u32(0, 7.0f32.to_bits());
    b.set_u32(4, 8.0f32.to_bits());
    let x1 = ps([1.0, 10.0, 20.0, 30.0]);
    let x2 = ps([4.0, 99.0, 99.0, 99.0]);
    run(MOVSS_FORMS, &mut b, [0, x1, x2, 0]);
    assert_eq!(b.f32s(0x100), [7.0, 0.0, 0.0, 0.0], "movss load zeroes upper");
    assert_eq!(b.u32s(0x110), [4.0f32.to_bits(), u32::MAX, u32::MAX, u32::MAX], "movss reg keeps upper");
    assert_eq!(b.f32s(0x120), [4.0, 10.0, 20.0, 30.0], "vmovss 3-op upper from src1");
    assert_eq!(b.u64(0x130), b.u64(0), "movsd load");
    assert_eq!(b.u64(0x138), 0, "movsd load zeroes upper");
    assert_eq!(b.f32s(0x140), [-7.0, 10.0, 20.0, 30.0], "vcvtsi2ss upper from src1");
    assert_eq!(b.u64(0x150), (-7.0f64).to_bits(), "cvtsi2sd");
    assert_eq!(b.u64(0x158), (x1 >> 64) as u64, "cvtsi2sd keeps upper");
    assert_eq!(b.u64(0x160), 4.0f64.to_bits(), "vcvtss2sd");
    assert_eq!(b.u64(0x168), (x1 >> 64) as u64, "vcvtss2sd upper from src1");
}

#[test]
fn simd_int_misc() {
    let mut b = new_buf();
    let v: [u32; 4] = [0x8000_1234, 0x7FFF_0F0F, 0xFFFF_FFFF, 0x0000_8001];
    b.set_u128(0, dq(v));
    b.set_u32(0x10, 3);
    b.set_u32(0x20, 1.5f32.to_bits());
    b.set_u32(0x24, (-2.0f32).to_bits());
    for (i, f) in [1.0f64, 2.0, 3.0, 4.0].iter().enumerate() {
        b.set_u64(0x40 + 8 * i, f.to_bits());
    }
    for (i, s) in [-1i32, 0, 5, -9].iter().enumerate() {
        b.set_u32(0x60 + 4 * i, *s as u32);
    }
    run(SIMD_INT_MISC, &mut b, [0; 4]);
    let w = |x: u32| ((x & 0xFFFF) >> 4) | ((((x >> 16) & 0xFFFF) >> 4) << 16);
    assert_eq!(b.u32s(0x100), v.map(w), "psrlw imm");
    assert_eq!(b.u32s(0x110), v.map(|x| x << 3), "pslld xmm count");
    assert_eq!(b.u32s(0x120), v.map(|x| ((x as i32) >> 31) as u32), "psrad count>31");
    assert_eq!(b.u32(0x130), 0x2F48, "pmovmskb");
    assert_eq!(b.u32(0x134), 5, "movmskps");
    assert_eq!(b.u32(0x138), 0xBEEF, "pextrw");
    assert_eq!(b.u32s(0x140), [v[0], 0xBEEF_0F0F, v[2], v[3]], "pinsrw");
    assert_eq!(b.u64(0x150), 1.5f64.to_bits(), "cvtps2pd m64 lo");
    assert_eq!(b.u64(0x158), (-2.0f64).to_bits(), "cvtps2pd m64 hi");
    assert_eq!(b.f32s(0x160), [1.0, 2.0, 3.0, 4.0], "vcvtpd2ps m256");
    assert_eq!(
        b.u32s(0x170),
        [v[0].wrapping_neg(), 0, v[2], v[3].wrapping_neg()],
        "psignd"
    );
}

#[test]
fn int_flags_and_narrow_muldiv() {
    let mut b = new_buf();
    run(INT_FLAGS, &mut b, [0; 4]);
    assert_eq!(b.u32(0x100), 5, "adc result");
    assert_eq!(b.0[0x104], 1, "adc CF when b+CF wraps");
    assert_eq!(b.u32(0x108), 5, "sbb result");
    assert_eq!(b.0[0x10c], 1, "sbb CF when b+CF wraps");
    assert_eq!(&b.0[0x110..0x112], &[1, 0], "tzcnt(1): ZF=1 CF=0");
    assert_eq!(&b.0[0x112..0x114], &[0, 1], "tzcnt(0): ZF=0 CF=1");
    assert_eq!(b.u32(0x114), 32, "tzcnt(0) = 32");
    assert_eq!(&b.0[0x118..0x11a], &[1, 0], "blsi CF = src!=0");
    assert_eq!(b.0[0x11a], 1, "clc; cmc");
    assert_eq!(b.u64(0x120), 0x258, "mul dl");
    assert_eq!(b.0[0x11b], 1, "mul dl CF");
    assert_eq!(b.u64(0x128), 0xFFF1, "imul dl");
    assert_eq!(b.u64(0x130), 0x8E, "div r9w quotient");
    assert_eq!(b.u64(0x138), 6, "div r9w remainder");
    assert_eq!(b.u64(0x140), 0x20E, "div r9b: AH=rem AL=quot");
}

#[test]
fn string_cmps() {
    let mut b = new_buf();
    b.0[0..8].copy_from_slice(b"abcdefgh");
    b.0[0x10..0x18].copy_from_slice(b"abcdXfgh");
    b.set_u32(0x20, 0x1122_3344);
    b.set_u32(0x28, 0x1122_3344);
    run(REPE_CMPSB, &mut b, [0; 4]);
    assert_eq!(b.u64(0x100), 3, "repe cmpsb remaining count");
    assert_eq!(&b.0[0x108..0x10a], &[0, 0], "ZF=0 CF=0 ('e' > 'X')");
    assert_eq!(b.u64(0x110), 5, "rsi advanced");
    assert_eq!(b.0[0x118], 1, "cmpsd equal");
    assert_eq!(b.u64(0x120), 0x2c, "rdi advanced by 4");
}

#[rustfmt::skip]
const SHIFT_FLAGS: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0xb8, 0x01, 0x00, 0x00, 0x80,  // mov eax, 0x80000001
    0xd1, 0xe0,  // shl eax, 1
    0x41, 0x0f, 0x92, 0x80, 0x00, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x100]
    0x41, 0x0f, 0x90, 0x80, 0x01, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x101]
    0x41, 0x89, 0x80, 0x04, 0x01, 0x00, 0x00,  // mov dword ptr [r8 + 0x104], eax
    0xb8, 0x03, 0x00, 0x00, 0x00,  // mov eax, 0x3
    0xd1, 0xe8,  // shr eax, 1
    0x41, 0x0f, 0x92, 0x80, 0x08, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x108]
    0x41, 0x0f, 0x90, 0x80, 0x09, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x109]
    0xb0, 0x80,  // mov al, -0x80
    0xc0, 0xf8, 0x09,  // sar al, 0x9
    0x41, 0x88, 0x80, 0x0c, 0x01, 0x00, 0x00,  // mov byte ptr [r8 + 0x10c], al
    0x41, 0x0f, 0x92, 0x80, 0x0d, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x10d]
    0xb8, 0x00, 0x00, 0x00, 0x40,  // mov eax, 0x40000000
    0xb1, 0x02,  // mov cl, 0x2
    0xd3, 0xe0,  // shl eax, cl
    0x41, 0x0f, 0x92, 0x80, 0x10, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x110]
    0x41, 0x0f, 0x94, 0x80, 0x11, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x111]
    0xb8, 0x00, 0x00, 0x00, 0x80,  // mov eax, 0x80000000
    0xd1, 0xc0,  // rol eax, 1
    0x41, 0x0f, 0x92, 0x80, 0x12, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x112]
    0x41, 0x0f, 0x90, 0x80, 0x13, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x113]
    0x41, 0xd1, 0x20,  // shl dword ptr [r8], 1
    0x41, 0x0f, 0x92, 0x80, 0x14, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x114]
    0x66, 0x41, 0xd1, 0x48, 0x08,  // ror word ptr [r8 + 0x8], 1
    0x41, 0x0f, 0x90, 0x80, 0x15, 0x01, 0x00, 0x00,  // seto byte ptr [r8 + 0x115]
    0x41, 0x0f, 0x92, 0x80, 0x16, 0x01, 0x00, 0x00,  // setb byte ptr [r8 + 0x116]
    0xc3,  // ret
];

#[test]
fn shift_rotate_flags() {
    let mut b = new_buf();
    b.set_u32(0, 0x8000_0000);
    b.set_u32(8, 1);
    run(SHIFT_FLAGS, &mut b, [0; 4]);
    assert_eq!(&b.0[0x100..0x102], &[1, 1], "shl eax,1 (0x80000001): CF=1 OF=1");
    assert_eq!(b.u32(0x104), 2, "shl result");
    assert_eq!(&b.0[0x108..0x10a], &[1, 0], "shr eax,1 (3): CF=1 OF=0");
    assert_eq!(&b.0[0x10c..0x10e], &[0xFF, 1], "sar al,9 (0x80): all sign bits, CF=sign");
    assert_eq!(&b.0[0x110..0x112], &[1, 1], "shl eax,cl=2 (0x40000000): CF=1 ZF=1");
    assert_eq!(&b.0[0x112..0x114], &[1, 1], "rol eax,1 (0x80000000): CF=1 OF=1");
    assert_eq!(b.0[0x114], 1, "shl m32,1 CF");
    assert_eq!(b.u32(0), 0, "shl m32 result");
    assert_eq!(&b.0[0x115..0x117], &[1, 1], "ror m16,1 (1): OF=1 CF=1");
    assert_eq!(b.u32(8) & 0xFFFF, 0x8000, "ror m16 result");
}

// ---------------------------------------------------------------------------------------------
// Follow-up: previously-unimplemented mnemonics.
// ---------------------------------------------------------------------------------------------

#[test]
fn bitops_misc_new_mnemonics() {
    let mut b = new_buf();
    b.set_u32(0, 0x1122_3344);
    b.0[0x20..0x29].copy_from_slice(b"123456789");
    for i in 0..16 {
        b.0[0x30 + i] = 0xA0 + i as u8;
    }
    with_frozen_tsc(|| run(BITOPS_MISC, &mut b, [0; 4]));
    assert_eq!(b.u32(0x100), 0x4433_2211, "movbe load");
    assert_eq!(b.u64(0x108), 0x0807_0605_0403_0201, "movbe store");
    assert_eq!(b.u32(0x1a0), 0x4700, "lahf: SF=0 ZF=1 PF=1 CF=1 + reserved bit 1");
    assert_eq!(&b.0[0x111..0x115], &[1, 1, 0, 0], "sahf 0x81: SF CF set, ZF PF clear");
    assert_eq!(&b.0[0x118..0x11b], &[0x03, 1, 1], "rcl al,1 (CF=1): value, CF, OF");
    assert_eq!(b.u32(0x11c), 0x8000_0000, "rcr eax,2 (CF=0)");
    assert_eq!(b.0[0x120], 1, "rcr CF");
    assert_eq!(b.u64(0x128), 0xE, "rcl rax,3 (CF=1)");
    assert_eq!(b.0[0x130], 0, "rcl rax,3 CF");
    assert_eq!(b.0[0x131], 0x5a, "rcl r8 by 9 is a no-op (mod 9)");
    assert_eq!(b.u32(0x138), 0x23, "bextr start 4 len 8");
    assert_eq!(b.0[0x13c], 0, "bextr ZF");
    assert_eq!(b.u32(0x140), 0, "bextr start >= 32");
    assert_eq!(b.0[0x144], 1, "bextr ZF on zero");
    assert_eq!(b.u64(0x148), 0x1200_3400_5600_7800, "pdep r64");
    assert_eq!(b.u64(0x150), 0x1133_5577, "pext r64");
    assert_eq!(b.u64(0x158), 0x5577, "pext r32 zero-extends");
    assert_eq!(b.u64(0x160), 0, "adcx -1 + 0 + CF");
    assert_eq!(&b.0[0x168..0x16a], &[1, 0], "adcx sets CF only");
    assert_eq!(b.u64(0x170), 12, "adox 5 + 6 + OF");
    assert_eq!(&b.0[0x178..0x17a], &[0, 0], "adox OF out, CF untouched");
    assert_eq!(b.0[0x17a], 1, "shrd CF = last bit shifted out");
    assert_eq!(b.u32(0x180), 0xE306_9283, "crc32c(\"123456789\")");
    assert_eq!(b.u32(0x184), 0x0123_45A2, "xlatb");
    assert_eq!(b.u32(0x188), 10, "loop x5");
    assert_eq!(b.u32(0x18c), 1, "jrcxz taken");
    assert_eq!(b.u32(0x190), 9, "jrcxz not taken");
    assert_eq!(b.u64(0x198), 0, "rdtscp ecx = TSC_AUX = 0");
}

#[test]
fn string_ops_lods_and_direction_flag() {
    let mut b = new_buf();
    for i in 0..16 {
        b.0[i] = 0x10 + i as u8;
    }
    let src: [u8; 16] = b.0[0..16].try_into().unwrap();
    run(STRING_DF, &mut b, [0; 4]);
    assert_eq!(b.0[0x100], 0x10, "lodsb");
    assert_eq!(b.u32(0x104), 0x1B1A_1918, "rep lodsd keeps last element");
    assert_eq!(b.u64(0x108), 12, "rep lodsd advances rsi");
    assert_eq!(&b.0[0x10f..0x115], &[0, 0xAB, 0xAB, 0xAB, 0xAB, 0], "std; rep stosb writes downwards");
    assert_eq!(b.u64(0x118), 0x10F, "std; rep stosb rdi");
    assert_eq!(&b.0[0x120..0x130], &src, "std; rep movsd copies downwards");
    assert_eq!(b.u64(0x130), 0x1F1E_1D1C_1B1A_1918, "std; lodsq");
    assert_eq!(b.u64(0x138), 0, "std; lodsq decrements rsi");
    assert_eq!(b.u64(0x140), 2, "std; repne scasb rcx");
    assert_eq!(b.u64(0x148), 1, "std; repne scasb rdi");
    assert_eq!(b.0[0x150], 0xCD, "cld; stosb");
    assert_eq!(b.u64(0x158), 0x151, "cld restores forward direction");
}

fn simd_new_inputs() -> Buf {
    let mut b = new_buf();
    for i in 0..0x40 {
        b.0[i] = (i * 7 + 3) as u8;
    }
    for (i, v) in [7u32, 0, 5, 2, 9, 3, 6, 12].iter().enumerate() {
        b.set_u32(0x40 + 4 * i, *v);
    }
    for (i, v) in [0u32, 1, 4, 31, 32, 33, 8, 100].iter().enumerate() {
        b.set_u32(0x60 + 4 * i, *v);
    }
    for (i, v) in [0u64, 63, 64, 5].iter().enumerate() {
        b.set_u64(0x80 + 8 * i, *v);
    }
    b.set_u32(0xa0, 2.7f32.to_bits());
    b
}

fn d8(b: &Buf, o: usize) -> [u32; 8] {
    core::array::from_fn(|i| b.u32(o + 4 * i))
}

fn q4(b: &Buf, o: usize) -> [u64; 4] {
    core::array::from_fn(|i| b.u64(o + 8 * i))
}

#[test]
fn simd_new_mnemonics() {
    let mut b = simd_new_inputs();
    let inp = simd_new_inputs();
    run(SIMD_NEW, &mut b, [0; 4]);
    let w = |o: usize| -> [u16; 16] { core::array::from_fn(|i| u16::from_le_bytes([inp.0[o + 2 * i], inp.0[o + 2 * i + 1]])) };
    let (y1w, y2w) = (w(0), w(0x20));
    // pblendw xmm (words 0..8 of A/B = y1w[0..8] / y1w[8..16]).
    let pb: Vec<u16> = (0..8).map(|i| if (0xA5 >> i) & 1 != 0 { y1w[8 + i] } else { y1w[i] }).collect();
    let got: Vec<u16> = (0..8).map(|i| u16::from_le_bytes([b.0[0x100 + 2 * i], b.0[0x101 + 2 * i]])).collect();
    assert_eq!(got, pb, "pblendw m128");
    let vpbw: Vec<u16> = (0..16).map(|i| if (0x0F >> (i % 8)) & 1 != 0 { y2w[i] } else { y1w[i] }).collect();
    let got: Vec<u16> = (0..16).map(|i| u16::from_le_bytes([b.0[0x120 + 2 * i], b.0[0x121 + 2 * i]])).collect();
    assert_eq!(got, vpbw, "vpblendw ymm (imm per lane)");
    let (y1, y2) = (d8(&inp, 0), d8(&inp, 0x20));
    let vpbd: [u32; 8] = core::array::from_fn(|i| if (0x5A >> i) & 1 != 0 { y2[i] } else { y1[i] });
    assert_eq!(d8(&b, 0x140), vpbd, "vpblendd ymm");
    assert_eq!(b.u64(0x160), y1[2] as u64, "extractps r32 zero-extends");
    assert_eq!(b.u32(0x168), y1[3], "extractps m32");
    let idx = d8(&inp, 0x40);
    let perm: [u32; 8] = core::array::from_fn(|i| y1[(idx[i] & 7) as usize]);
    assert_eq!(d8(&b, 0x180), perm, "vpermd");
    let q = q4(&inp, 0);
    assert_eq!(q4(&b, 0x1a0), [q[3], q[2], q[1], q[0]], "vpermq 0x1b");
    let cnt = d8(&inp, 0x60);
    let sllv: [u32; 8] = core::array::from_fn(|i| if cnt[i] > 31 { 0 } else { y1[i] << cnt[i] });
    assert_eq!(d8(&b, 0x1c0), sllv, "vpsllvd ymm");
    let srav: [u32; 8] =
        core::array::from_fn(|i| if i < 4 { ((y1[i] as i32) >> cnt[i].min(31)) as u32 } else { 0 });
    assert_eq!(d8(&b, 0x1e0), srav, "vpsravd xmm (upper zeroed)");
}

#[test]
fn simd_new_mnemonics_2() {
    let mut b = simd_new_inputs();
    let inp = simd_new_inputs();
    run(SIMD_NEW2, &mut b, [0; 4]);
    let y1 = d8(&inp, 0);
    let cnt = d8(&inp, 0x60);
    let srlv: [u32; 8] = core::array::from_fn(|i| if cnt[i] > 31 { 0 } else { y1[i] >> cnt[i] });
    assert_eq!(d8(&b, 0x100), srlv, "vpsrlvd ymm");
    let q = q4(&inp, 0);
    let qc = q4(&inp, 0x80);
    let sllq: [u64; 4] = core::array::from_fn(|i| if qc[i] > 63 { 0 } else { q[i] << qc[i] });
    let srlq: [u64; 4] = core::array::from_fn(|i| if qc[i] > 63 { 0 } else { q[i] >> qc[i] });
    assert_eq!(q4(&b, 0x120), sllq, "vpsllvq ymm");
    assert_eq!(q4(&b, 0x140), srlq, "vpsrlvq ymm m256");
    assert_eq!(b.u128(0x160), inp.u128(0x10), "lddqu");
    assert_eq!(
        d8(&b, 0x170),
        [2.0f32.to_bits(), y1[1], y1[2], y1[3], 0, 0, 0, 0],
        "vroundss floor, upper from src1, ymm upper zeroed"
    );
    assert_eq!(q4(&b, 0x190), [q[1], q[0], q[3], q[2]], "vpermpd m256 0xb1");
    let idx = d8(&inp, 0x40);
    let perm: [u32; 8] = core::array::from_fn(|i| y1[(idx[i] & 7) as usize]);
    assert_eq!(d8(&b, 0x1b0), perm, "vpermps m256");
}

// ---------------------------------------------------------------------------------------------
// Follow-up 2: PUSHF/POPF, PHADD/PHSUB, VTEST, masked moves.
// ---------------------------------------------------------------------------------------------

fn pattern_buf() -> Buf {
    let mut b = new_buf();
    for i in 0..0x40 {
        b.0[i] = (i * 37 + 11) as u8;
    }
    b
}

#[test]
fn flags_push_pop_hadd_vtest() {
    let mut b = pattern_buf();
    let inp = pattern_buf();
    run(FLAGS_HADD_VTEST, &mut b, [0; 4]);
    assert_eq!(b.u64(0x100), 0x647, "pushfq: 0x202 | CF | PF | ZF | DF");
    assert_eq!(&b.0[0x108..0x10c], &[1, 1, 0, 0], "popfq 0x880: SF OF set, ZF CF clear");
    assert_eq!(b.u64(0x118), 0x111, "popfq cleared DF");
    assert_eq!(u16::from_le_bytes([b.0[0x120], b.0[0x121]]), 0x206, "pushfw (16-bit) + pop ax: flags from `sub rdi, r8` = 0x111 (PF only)");

    let y1 = d8(&inp, 0);
    let y2 = d8(&inp, 0x20);
    let w = |o: usize| -> [u16; 16] { core::array::from_fn(|i| u16::from_le_bytes([inp.0[o + 2 * i], inp.0[o + 2 * i + 1]])) };
    let (w1, w2) = (w(0), w(0x20));
    // phaddd xmm1(A=y1[0..4]), xmm2(B=y1[4..8])
    let phaddd = [y1[0].wrapping_add(y1[1]), y1[2].wrapping_add(y1[3]), y1[4].wrapping_add(y1[5]), y1[6].wrapping_add(y1[7])];
    assert_eq!(b.u32s(0x130), phaddd, "phaddd");
    // phsubw xmm(A=w1[0..8]), m128(B=w1[8..16])
    let phsubw: Vec<u16> = (0..8).map(|i| { let base = if i < 4 { 2 * i } else { 8 + 2 * (i - 4) }; w1[base].wrapping_sub(w1[base + 1]) }).collect();
    let got: Vec<u16> = (0..8).map(|i| u16::from_le_bytes([b.0[0x140 + 2 * i], b.0[0x141 + 2 * i]])).collect();
    assert_eq!(got, phsubw, "phsubw m128");
    // vphaddw ymm: per 128-bit lane, 4 sums from src1 lane then 4 from src2 lane.
    let mut vphaddw = Vec::new();
    for lane in 0..2 {
        for src in [&w1, &w2] {
            for k in 0..4 {
                let j = lane * 8 + 2 * k;
                vphaddw.push(src[j].wrapping_add(src[j + 1]));
            }
        }
    }
    let got: Vec<u16> = (0..16).map(|i| u16::from_le_bytes([b.0[0x160 + 2 * i], b.0[0x161 + 2 * i]])).collect();
    assert_eq!(got, vphaddw, "vphaddw ymm");
    let vphsubd = [y1[0].wrapping_sub(y1[1]), y1[2].wrapping_sub(y1[3]), y2[0].wrapping_sub(y2[1]), y2[2].wrapping_sub(y2[3]), 0, 0, 0, 0];
    assert_eq!(d8(&b, 0x180), vphsubd, "vphsubd xmm (upper zeroed)");

    let sign32 = 0x8000_0000u32;
    let zf = (0..8).all(|i| y1[i] & y2[i] & sign32 == 0) as u8;
    let cf = (0..8).all(|i| !y1[i] & y2[i] & sign32 == 0) as u8;
    assert_eq!(&b.0[0x1a0..0x1a2], &[zf, cf], "vtestps ymm, m256");
    let q = q4(&inp, 0);
    let zf = (0..2).all(|i| q[i] >> 63 == 0) as u8;
    assert_eq!(&b.0[0x1a2..0x1a4], &[zf, 1], "vtestpd xmm, xmm (self)");
    // ymm7 = 0x40.. = zero in pattern_buf; ZF=1, CF = no sign bits in ymm4.
    let cf = (0..8).all(|i| y1[i] & sign32 == 0) as u8;
    assert_eq!(&b.0[0x1a4..0x1a6], &[1, cf], "vtestps zero mask");
}

#[test]
fn masked_moves() {
    let mut b = pattern_buf();
    let m: [u32; 8] = [0x8000_0000, 0, 0xFFFF_FFFF, 0x8FFF_FFFF, 0, 0x8000_0001, 0, 0xF000_0000];
    for (i, v) in m.iter().enumerate() {
        b.set_u32(0x40 + 4 * i, *v);
    }
    for i in 0x140..0x1a0 {
        b.0[i] = 0xEE;
    }
    let inp = Buf(b.0);
    run(MASKMOV, &mut b, [0; 4]);
    let (a, bb) = (d8(&inp, 0), d8(&inp, 0x20));
    let (aq, bq) = (q4(&inp, 0), q4(&inp, 0x20));
    let ms: [bool; 8] = core::array::from_fn(|i| m[i] >> 31 != 0);
    let mq = q4(&inp, 0x40);
    let mqs: [bool; 4] = core::array::from_fn(|i| mq[i] >> 63 != 0);
    let e32 = 0xEEEE_EEEEu32;
    let e64 = 0xEEEE_EEEE_EEEE_EEEEu64;
    let exp: [u32; 8] = core::array::from_fn(|i| if ms[i] { a[i] } else { 0 });
    assert_eq!(d8(&b, 0x100), exp, "vpmaskmovd ymm load");
    let exp = [if mqs[0] { aq[0] } else { 0 }, if mqs[1] { aq[1] } else { 0 }, 0, 0];
    assert_eq!(q4(&b, 0x120), exp, "vmaskmovpd xmm load (upper zeroed)");
    let exp: [u32; 8] = core::array::from_fn(|i| if ms[i] { bb[i] } else { e32 });
    assert_eq!(d8(&b, 0x140), exp, "vpmaskmovd ymm store leaves masked-off memory");
    let exp: [u64; 4] = core::array::from_fn(|i| if mqs[i] { bq[i] } else { e64 });
    assert_eq!(q4(&b, 0x160), exp, "vmaskmovpd ymm store");
    let exp = [if mqs[0] { bq[0] } else { e64 }, if mqs[1] { bq[1] } else { e64 }, e64, e64];
    assert_eq!(q4(&b, 0x180), exp, "vpmaskmovq xmm store");
    let exp: [u32; 8] = core::array::from_fn(|i| if i < 4 && ms[i] { bb[i] } else { 0 });
    assert_eq!(d8(&b, 0x1a0), exp, "vmaskmovps xmm load");
}

// ---- string-move overlap and CMPXCHG16B (V8 LoopFinder crash regression) ----

#[rustfmt::skip]
const REP_MOVSQ_FILL: &[u8] = &[
    0x48, 0x89, 0xce,                          // mov rsi, rcx
    0x48, 0x8d, 0x79, 0x18,                    // lea rdi, [rcx + 0x18]
    0x48, 0xc7, 0xc1, 0x09, 0x00, 0x00, 0x00,  // mov rcx, 9
    0xf3, 0x48, 0xa5,                          // rep movsq
    0xc3,                                      // ret
];

#[rustfmt::skip]
const REP_MOVSD_DIST2: &[u8] = &[
    0x48, 0x89, 0xce,                          // mov rsi, rcx
    0x48, 0x8d, 0x79, 0x02,                    // lea rdi, [rcx + 0x2]
    0xb9, 0x04, 0x00, 0x00, 0x00,              // mov ecx, 4
    0xf3, 0xa5,                                // rep movsd
    0xc3,                                      // ret
];

#[rustfmt::skip]
const CMPXCHG16B_FORM: &[u8] = &[
    0x49, 0x89, 0xc8,                          // mov r8, rcx
    0x49, 0x8b, 0x00,                          // mov rax, [r8]
    0x49, 0x8b, 0x50, 0x08,                    // mov rdx, [r8 + 0x8]
    0x49, 0x8b, 0x58, 0x10,                    // mov rbx, [r8 + 0x10]
    0x49, 0x8b, 0x48, 0x18,                    // mov rcx, [r8 + 0x18]
    0xf0, 0x49, 0x0f, 0xc7, 0x48, 0x20,        // lock cmpxchg16b [r8 + 0x20]
    0x41, 0x0f, 0x94, 0x80, 0x00, 0x01, 0x00, 0x00,  // sete byte ptr [r8 + 0x100]
    0x49, 0x89, 0x80, 0x08, 0x01, 0x00, 0x00,  // mov [r8 + 0x108], rax
    0x49, 0x89, 0x90, 0x10, 0x01, 0x00, 0x00,  // mov [r8 + 0x110], rdx
    0xc3,                                      // ret
];

/// x86 `rep movs` copies forward element by element, so an overlapping destination inside
/// the source replicates the prefix (MSVC's `std::vector(n, value)` fill idiom). A plain
/// memmove here left V8's `LoopFinderImpl::info_` with stale heap contents.
#[test]
fn rep_movs_overlap_replicates() {
    let mut b = new_buf();
    for i in 0..0x80 {
        b.0[i] = 0xa0 ^ i as u8;
    }
    let mut want = b.0;
    run(REP_MOVSQ_FILL, &mut b, [0; 4]);
    for i in 0..9 * 8 {
        want[0x18 + i] = want[i];
    }
    assert_eq!(&b.0[..0x80], &want[..0x80], "rep movsq fill idiom");

    let mut b = new_buf();
    for i in 0..0x40 {
        b.0[i] = 0x30 + i as u8;
    }
    let mut want = b.0;
    run(REP_MOVSD_DIST2, &mut b, [0; 4]);
    for e in 0..4 {
        let v: [u8; 4] = want[e * 4..e * 4 + 4].try_into().unwrap();
        want[2 + e * 4..2 + e * 4 + 4].copy_from_slice(&v);
    }
    assert_eq!(&b.0[..0x40], &want[..0x40], "rep movsd with dist < element size");
}

#[test]
fn cmpxchg16b_success_and_failure() {
    let mut b = new_buf();
    let (cur, new) = (0x1111_2222_3333_4444_5555_6666_7777_8888u128, 0x9999_aaaa_bbbb_cccc_dddd_eeee_ffff_0001u128);
    b.set_u128(0x00, cur);
    b.set_u128(0x10, new);
    b.set_u128(0x20, cur);
    run(CMPXCHG16B_FORM, &mut b, [0; 4]);
    assert_eq!(b.0[0x100], 1, "zf on success");
    assert_eq!(b.u128(0x20), new, "stored desired");

    let mut b = new_buf();
    b.set_u128(0x00, cur ^ 1);
    b.set_u128(0x10, new);
    b.set_u128(0x20, cur);
    run(CMPXCHG16B_FORM, &mut b, [0; 4]);
    assert_eq!(b.0[0x100], 0, "zf on failure");
    assert_eq!(b.u128(0x20), cur, "memory unchanged on failure");
    assert_eq!(b.u64(0x108) as u128 | (b.u64(0x110) as u128) << 64, cur, "rdx:rax = actual");
}

// ---- SSE/AVX fast-path regression vectors (kinds >= K_VSHUF_RR in emu_fast.rs) ----

mod sse_vec {
    use super::*;
    include!("sse_vectors.rs");

    const BIG: usize = 0x4000;

    /// Run a vector snippet over a fresh 16 KiB buffer (inputs, then 0xEE fill, like the Rosetta
    /// harness) with the fast path on or forced off, and the translator forced on (1) or off (2).
    fn run_big(code: &[u8], fast: bool, jit_mode: u8) -> Vec<u8> {
        let c = arena_alloc(code.len() + 16, 16);
        let d = arena_alloc(BIG, 64);
        let prev = TEST_FAST_OFF.with(|f| f.replace(!fast));
        jit::force(jit_mode);
        let out = unsafe {
            core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
            core::ptr::write_bytes(c.add(code.len()), 0xCC, 16);
            core::ptr::write_bytes(d, 0xEE, BIG);
            core::ptr::copy_nonoverlapping(SSE_VEC_INPUT.as_ptr(), d, 256);
            emu_run_full(c as u64, [d as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            core::slice::from_raw_parts(d, BIG).to_vec()
        };
        TEST_FAST_OFF.with(|f| f.set(prev));
        jit::force(0);
        out
    }

    fn label(notes: &[(usize, usize, &str)], off: usize) -> String {
        notes
            .iter()
            .find(|(o, n, _)| off >= *o && off < o + n)
            .map_or_else(|| format!("{off:#x} (unlabelled)"), |(o, _, l)| format!("{l} @ {o:#x}+{}", off - o))
    }

    /// Compare `got[0x100..end]` against `want`, listing every mismatching labelled range.
    fn diff(what: &str, got: &[u8], want: &[u8], end: usize, notes: &[(usize, usize, &str)]) -> Vec<String> {
        let mut bad = Vec::new();
        let mut last = String::new();
        for off in 0x100..end {
            if got[off] != want[off - 0x100] {
                let l = label(notes, off);
                let key = l.split(" @ ").next().unwrap_or("").to_string();
                if key != last {
                    let (o, n) = notes
                        .iter()
                        .find(|(o, n, _)| off >= *o && off < o + n)
                        .map_or((off, 1), |(o, n, _)| (*o, *n));
                    bad.push(format!(
                        "{what}: {l}: got {} want {}",
                        hex(&got[o..o + n]),
                        hex(&want[o - 0x100..o - 0x100 + n])
                    ));
                    last = key;
                }
            }
        }
        bad
    }

    fn hex(b: &[u8]) -> String {
        b.iter().map(|x| format!("{x:02x}")).collect()
    }

    fn unhex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    /// Block translator, fast path, generic interpreter and native x86 (Rosetta) must all agree.
    fn check(name: &str, code: &[u8], end: usize, exp: &str, notes: &[(usize, usize, &str)]) {
        let want = unhex(exp);
        assert_eq!(want.len(), end - 0x100, "{name}: expected-bytes length");
        let fast = run_big(code, true, 2);
        let generic = run_big(code, false, 2);
        let jit = run_big(code, true, 1);
        let mut bad = diff("generic vs native", &generic, &want, end, notes);
        bad.extend(diff("fast vs native", &fast, &want, end, notes));
        bad.extend(diff("jit vs native", &jit, &want, end, notes));
        // Nothing may be written outside the output range (scratch at 0x3ff0 excepted).
        for (tag, buf) in [("fast", &fast), ("generic", &generic), ("jit", &jit)] {
            if let Some(o) = (end..0x3ff0).find(|&o| buf[o] != 0xEE) {
                bad.push(format!("{tag}: stray write at {o:#x}"));
            }
        }
        assert!(bad.is_empty(), "{name}: {} mismatches:\n{}", bad.len(), bad.join("\n"));
    }

    /// Every instruction in the snippet that the fast path is expected to cover must lower to a
    /// native fast kind (never `K_GENERIC` or `K_INSN`).
    fn coverage(name: &str, code: &[u8], allow_generic: &[iced_x86::Mnemonic]) {
        let mut dec = iced_x86::Decoder::with_ip(64, code, 0x1000, iced_x86::DecoderOptions::NONE);
        let mut missing = Vec::new();
        while dec.can_decode() {
            let ins = dec.decode();
            let k = fast::lower(&ins).kind;
            if (k == fast::K_GENERIC || k == fast::K_INSN) && !allow_generic.contains(&ins.mnemonic()) {
                missing.push(format!("{ins} (kind={k})"));
            }
        }
        missing.dedup();
        assert!(missing.is_empty(), "{name}: not lowered to a native fast kind:\n{}", missing.join("\n"));
    }

    #[test]
    fn sse_int_legacy() {
        check("SSE_INT_LEGACY", SSE_INT_LEGACY, SSE_INT_LEGACY_END, SSE_INT_LEGACY_EXP, SSE_INT_LEGACY_NOTES);
    }

    #[test]
    fn sse_shuf_shift() {
        check("SSE_SHUF_SHIFT", SSE_SHUF_SHIFT, SSE_SHUF_SHIFT_END, SSE_SHUF_SHIFT_EXP, SSE_SHUF_SHIFT_NOTES);
    }

    #[test]
    fn sse_fp_legacy() {
        check("SSE_FP_LEGACY", SSE_FP_LEGACY, SSE_FP_LEGACY_END, SSE_FP_LEGACY_EXP, SSE_FP_LEGACY_NOTES);
    }

    #[test]
    fn sse_moves_misc() {
        check("SSE_MOVES_MISC", SSE_MOVES_MISC, SSE_MOVES_MISC_END, SSE_MOVES_MISC_EXP, SSE_MOVES_MISC_NOTES);
    }

    #[test]
    fn avx_forms() {
        check("AVX_FORMS", AVX_FORMS, AVX_FORMS_END, AVX_FORMS_EXP, AVX_FORMS_NOTES);
    }

    #[test]
    fn sse_vectors_lower_to_fast_kinds() {
        let allow: &[iced_x86::Mnemonic] = &[];
        coverage("SSE_INT_LEGACY", SSE_INT_LEGACY, allow);
        coverage("SSE_SHUF_SHIFT", SSE_SHUF_SHIFT, allow);
        coverage("SSE_FP_LEGACY", SSE_FP_LEGACY, allow);
        coverage("SSE_MOVES_MISC", SSE_MOVES_MISC, allow);
        coverage("AVX_FORMS", AVX_FORMS, allow);
        coverage("PXOR_FORMS", super::PXOR_FORMS, allow);
        coverage("SCALAR_ARITH", super::SCALAR_ARITH, allow);
        coverage("SCALAR_SPECIAL", super::SCALAR_SPECIAL, allow);
        coverage("CVT_TO_INT", super::CVT_TO_INT, allow);
        coverage("UCOMISS_FLAGS", super::UCOMISS_FLAGS, allow);
        coverage("CMPPS_PREDS", super::CMPPS_PREDS, allow);
        coverage("VEX128_ZERO_UPPER", super::VEX128_ZERO_UPPER, allow);
        coverage("YMM_PACKED", super::YMM_PACKED, allow);
        coverage("MOVSS_FORMS", super::MOVSS_FORMS, allow);
        coverage("SIMD_INT_MISC", super::SIMD_INT_MISC, allow);
        coverage("INT_FLAGS", super::INT_FLAGS, allow);
        coverage("REPE_CMPSB", super::REPE_CMPSB, allow);
        coverage("BITOPS_MISC", super::BITOPS_MISC, allow);
        coverage("STRING_DF", super::STRING_DF, allow);
        coverage("SIMD_NEW", super::SIMD_NEW, allow);
        coverage("SIMD_NEW2", super::SIMD_NEW2, allow);
        coverage("FLAGS_HADD_VTEST", super::FLAGS_HADD_VTEST, allow);
        coverage("MASKMOV", super::MASKMOV, allow);
        coverage("SHIFT_FLAGS", super::SHIFT_FLAGS, allow);
        coverage("REP_MOVSQ_FILL", super::REP_MOVSQ_FILL, allow);
        coverage("REP_MOVSD_DIST2", super::REP_MOVSD_DIST2, allow);
        coverage("CMPXCHG16B_FORM", super::CMPXCHG16B_FORM, allow);
    }
}

/// A native (lifted) caller's rax reaches guest code it calls through `exe::call_va8` /
/// `simd_ops::call_va8_xmm` (`emu_run_full_rax`), as a real x86 `call` leaves it: guest code
/// that returns rax untouched (directly, or through a `jmp [rip+slot]` import thunk to guest
/// code) gives the caller its own rax back. Guest code that sets rax is unaffected, the
/// caller's `rax_in` is restored, and a null target still yields 0.
#[test]
fn native_to_guest_call_passes_rax() {
    // ret
    let leaf = arena_alloc(16, 16);
    // mov eax, 5; ret
    let sets = arena_alloc(16, 16);
    // jmp qword ptr [rip+0]; dq leaf
    let thunk = arena_alloc(32, 16);
    unsafe {
        core::ptr::write_bytes(leaf, 0xCC, 16);
        *leaf = 0xC3;
        core::ptr::write_bytes(sets, 0xCC, 16);
        core::ptr::copy_nonoverlapping([0xB8u8, 5, 0, 0, 0, 0xC3].as_ptr(), sets, 6);
        core::ptr::write_bytes(thunk, 0xCC, 32);
        core::ptr::copy_nonoverlapping([0xFFu8, 0x25, 0, 0, 0, 0].as_ptr(), thunk, 6);
        core::ptr::write_unaligned(thunk.add(6) as *mut u64, leaf as u64);
    }
    const RAX: u64 = 0x7001_e6b3_78;
    for force in [2u8, 1] {
        #[cfg(target_arch = "aarch64")]
        jit::force(force);
        unsafe {
            for (code, want) in [(leaf, RAX), (thunk, RAX), (sets, 5)] {
                let prev = crate::simd_ops::replace_rax_in(RAX);
                let r = crate::exe::call_va8(code as u64, 1, 2, 3, 4, 0, 0, 0, 0);
                assert_eq!(r, want, "call_va8 {code:p} force {force}");
                assert_eq!(crate::simd_ops::get_rax_in(), RAX, "rax_in restored");
                let (rx, _) = crate::simd_ops::call_va8_xmm(code as u64, 1, 2, 3, 4, 0, 0, 0, 0, [0; 4]);
                assert_eq!(rx, want, "call_va8_xmm {code:p} force {force}");
                crate::simd_ops::replace_rax_in(prev);
            }
            assert_eq!(emu_run_full_rax(0, [0; 8], [0; 4], [0; 8], RAX).0, 0, "null target");
        }
    }
    #[cfg(target_arch = "aarch64")]
    jit::force(0);
}

/// Store both halves of each timestamp and the AUX value for tier comparison.
#[rustfmt::skip]
const RDTSC_FORMS: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0x0f, 0x31,  // rdtsc
    0x49, 0x89, 0x80, 0x00, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x100], rax
    0x49, 0x89, 0x90, 0x08, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x108], rdx
    0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff,  // mov rcx, -1
    0x0f, 0x01, 0xf9,  // rdtscp
    0x49, 0x89, 0x80, 0x10, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x110], rax
    0x49, 0x89, 0x90, 0x18, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x118], rdx
    0x49, 0x89, 0x88, 0x20, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x120], rcx
    0xc3,  // ret
];

#[test]
fn rdtsc_reads_the_verifier_clock_on_every_path() {
    with_frozen_tsc(|| {
        let mut b = new_buf();
        b.0[0x100..0x128].fill(0xff);
        run(RDTSC_FORMS, &mut b, [0; 4]);
        for off in [0x100, 0x110] {
            assert_eq!(b.u64(off), GUEST_TSC);
            assert_eq!(b.u64(off + 8), 0);
        }
        assert_eq!(b.u64(0x120), 0);
    });
}

/// `paddb` / `pmaxuw`: the emulator matches `simd_ops::vpaddb` / `vpmaxuw` (the lifted
/// lowering, frontier_plan.md §3).
#[rustfmt::skip]
const PADDB_PMAXUW: &[u8] = &[
    0x49, 0x89, 0xc8,  // mov r8, rcx
    0xf3, 0x41, 0x0f, 0x6f, 0x00,  // movdqu xmm0, xmmword ptr [r8]
    0xf3, 0x41, 0x0f, 0x6f, 0x48, 0x10,  // movdqu xmm1, xmmword ptr [r8 + 0x10]
    0x66, 0x0f, 0xfc, 0xc1,  // paddb xmm0, xmm1
    0xf3, 0x41, 0x0f, 0x7f, 0x80, 0x00, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [r8 + 0x100], xmm0
    0xf3, 0x41, 0x0f, 0x6f, 0x00,  // movdqu xmm0, xmmword ptr [r8]
    0x66, 0x0f, 0x38, 0x3e, 0xc1,  // pmaxuw xmm0, xmm1
    0xf3, 0x41, 0x0f, 0x7f, 0x80, 0x10, 0x01, 0x00, 0x00,  // movdqu xmmword ptr [r8 + 0x110], xmm0
    0xc3,  // ret
];

struct XorShift(u64);
impl XorShift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[test]
fn paddb_pmaxuw_match_simd_ops() {
    let mut r = XorShift(0x9e37_79b9_7f4a_7c15);
    let mut cases = vec![
        (0u128, 0u128),
        (u128::MAX, 1),
        (0x8000_7fff_ffff_0001_8000_7fff_ffff_0001, 0x7fff_8000_0001_ffff_7fff_8000_0001_ffff),
    ];
    for _ in 0..32 {
        let a = (r.next() as u128) << 64 | r.next() as u128;
        let b = (r.next() as u128) << 64 | r.next() as u128;
        cases.push((a, b));
    }
    for (a, bv) in cases {
        let mut b = new_buf();
        b.set_u128(0, a);
        b.set_u128(0x10, bv);
        run(PADDB_PMAXUW, &mut b, [0; 4]);
        assert_eq!(b.u128(0x100), crate::simd_ops::vpaddb(a, bv), "paddb {a:#x} {bv:#x}");
        assert_eq!(b.u128(0x110), crate::simd_ops::vpmaxuw(a, bv), "pmaxuw {a:#x} {bv:#x}");
    }
}

/// `pcmpistri xmm0, xmm1/m128, imm` code for one imm: register form -> [0x100] rcx, [0x108]
/// rflags; memory form -> [0x110], [0x118]. rcx = -1 before each (ecx write zero-extends).
fn pcmpistri_code(imm: u8) -> Vec<u8> {
    #[rustfmt::skip]
    let v = vec![
        0x49, 0x89, 0xc8,  // mov r8, rcx
        0xf3, 0x41, 0x0f, 0x6f, 0x00,  // movdqu xmm0, xmmword ptr [r8]
        0xf3, 0x41, 0x0f, 0x6f, 0x48, 0x10,  // movdqu xmm1, xmmword ptr [r8 + 0x10]
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff,  // mov rcx, -1
        0x66, 0x0f, 0x3a, 0x63, 0xc1, imm,  // pcmpistri xmm0, xmm1, imm
        0x49, 0x89, 0x88, 0x00, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x100], rcx
        0x9c,  // pushfq
        0x58,  // pop rax
        0x49, 0x89, 0x80, 0x08, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x108], rax
        0x48, 0xc7, 0xc1, 0xff, 0xff, 0xff, 0xff,  // mov rcx, -1
        0x66, 0x41, 0x0f, 0x3a, 0x63, 0x40, 0x10, imm,  // pcmpistri xmm0, xmmword ptr [r8 + 0x10], imm
        0x49, 0x89, 0x88, 0x10, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x110], rcx
        0x9c,  // pushfq
        0x58,  // pop rax
        0x49, 0x89, 0x80, 0x18, 0x01, 0x00, 0x00,  // mov qword ptr [r8 + 0x118], rax
        0xc3,  // ret
    ];
    v
}

/// czso (simd_ops::pcmpistri) -> the rflags bits CF ZF SF OF (AF = PF = 0).
fn czso_rflags(f: u8) -> u64 {
    (f & 1) as u64 | (((f >> 1) & 1) as u64) << 6 | (((f >> 2) & 1) as u64) << 7 | (((f >> 3) & 1) as u64) << 11
}

const STATUS_FLAGS: u64 = 0x8d5; // CF PF AF ZF SF OF

#[test]
fn pcmpistri_known_values() {
    // Hardware semantics for the vcruntime140 forms (frontier_plan.md §2).
    let s = |t: &[u8]| t.iter().enumerate().fold(0u128, |acc, (i, &c)| acc | (c as u128) << (8 * i));
    let w = |t: &[u16]| t.iter().enumerate().fold(0u128, |acc, (i, &c)| acc | (c as u128) << (16 * i));
    use crate::simd_ops::pcmpistri;
    // strrchr: equal any, ub, msb index. Needle "a", hay "xaya" -> last match 3; CF, ZF
    // (hay has NUL), SF (needle has NUL), OF = bit 0 of IntRes2 = 0.
    assert_eq!(pcmpistri(s(b"a"), s(b"xaya"), 0x40), (3, 0b0111));
    // Same, lsb index (imm 0x00): 1.
    assert_eq!(pcmpistri(s(b"a"), s(b"xaya"), 0x00), (1, 0b0111));
    // No match: index 16, CF = 0.
    assert_eq!(pcmpistri(s(b"q"), s(b"xaya"), 0x40), (16, 0b0110));
    // strstr: equal ordered (0x0c), needle "ab", hay "zabab": first match 1.
    assert_eq!(pcmpistri(s(b"ab"), s(b"zabab"), 0x0c), (1, 0b0111));
    // Equal ordered partial match at the end of a full (NUL-free) hay: candidate 15.
    let hay: Vec<u8> = b"xxxxxxxxxxxxxxxa".to_vec();
    assert_eq!(pcmpistri(s(b"ab"), s(&hay), 0x0c), (15, 0b0101));
    // wcsrchr (0x41: equal any, uw, msb): needle L"b", hay L"abcb" -> 3.
    assert_eq!(pcmpistri(w(&[b'b' as u16]), w(&[97, 98, 99, 98]), 0x41), (3, 0b0111));
    // wcsstr (0x0d: equal ordered, uw): needle L"cb", hay L"abcb" -> 2.
    assert_eq!(pcmpistri(w(&[99, 98]), w(&[97, 98, 99, 98]), 0x0d), (2, 0b0111));
    // 0x15: equal any | negative polarity (bytes not in set): set "ab", hay "abxa" -> first 2.
    assert_eq!(pcmpistri(s(b"ab"), s(b"abxa"), 0x14), (2, 0b0111));
    // Negative polarity covers the invalid (past-NUL) elements too: hay "ab" all in set ->
    // first non-member is index 2 (past the end).
    assert_eq!(pcmpistri(s(b"ab"), s(b"ab"), 0x14), (2, 0b0111));
    // Masked negative (0x34): only valid elements negate -> no match, index 16.
    assert_eq!(pcmpistri(s(b"ab"), s(b"ab"), 0x34), (16, 0b0110));
    // Both empty, equal each: IntRes1 all ones (invalid == invalid), OF = 1.
    assert_eq!(pcmpistri(0, 0, 0x08), (0, 0b1111));
}

#[test]
fn pcmpistri_emulator_matches_simd_ops() {
    let mut r = XorShift(0x2545_f491_4f6c_dd1d);
    const ALPHA: [u16; 8] = [0x61, 0x62, 0x63, 0x01, 0x7f, 0x80, 0xff, 0x8000];
    let gen = |r: &mut XorShift, words: bool| -> u128 {
        let n = if words { 8 } else { 16 };
        let mut v = 0u128;
        let nul = (r.next() % (n as u64 + 4)) as usize; // >= n: no NUL
        for i in 0..n {
            let mut e = ALPHA[(r.next() % 8) as usize] as u128;
            if r.next() % 8 == 0 {
                e = r.next() as u128 & 0xffff;
            }
            if i == nul {
                e = 0;
            }
            if words {
                v |= (e & 0xffff) << (16 * i);
            } else {
                v |= (e & 0xff) << (8 * i);
            }
        }
        v
    };
    for imm in 0u8..0x80 {
        let code = pcmpistri_code(imm);
        let words = imm & 1 != 0;
        let n = if words { 8 } else { 16 };
        // NUL at every position of each operand, then random pairs.
        let mut cases = Vec::new();
        for k in 0..=n {
            let a = gen(&mut r, words);
            let b = gen(&mut r, words);
            let sh = if words { 16 * k } else { 8 * k };
            let m = if words { 0xffffu128 } else { 0xff };
            let cut = |v: u128| if k < n { v & !(m << sh) } else { v };
            cases.push((cut(a), b));
            cases.push((a, cut(b)));
        }
        for _ in 0..12 {
            cases.push((gen(&mut r, words), gen(&mut r, words)));
        }
        for (a, bv) in cases {
            let mut b = new_buf();
            b.set_u128(0, a);
            b.set_u128(0x10, bv);
            run(&code, &mut b, [0; 4]);
            let (idx, f) = crate::simd_ops::pcmpistri(a, bv, imm);
            for (o, form) in [(0x100, "reg"), (0x110, "mem")] {
                assert_eq!(b.u64(o), idx as u64, "imm {imm:#x} {form} a {a:#x} b {bv:#x}: ecx");
                assert_eq!(
                    b.u64(o + 8) & STATUS_FLAGS,
                    czso_rflags(f),
                    "imm {imm:#x} {form} a {a:#x} b {bv:#x}: flags"
                );
            }
        }
    }
}

/// Run assembled x86-64 guest code (which receives `rcx = buf_ptr`) on ALL THREE emulator paths:
/// generic interpreter (`fast=false, jit=off`), fast interpreter (`fast=true, jit=off`), and
/// native ARM64 JIT (`fast=true, jit=on`), asserting every path produces byte-identical output.
fn run_all_3_paths(code: &[u8], init: &[u8]) -> Vec<u8> {
    let n = init.len();
    let mut results: Vec<(&str, Vec<u8>)> = Vec::new();
    #[cfg(target_arch = "aarch64")]
    let modes: &[(&str, bool, u8)] = &[
        ("generic", false, 2),
        ("fast", true, 2),
        ("jit", true, 1),
    ];
    #[cfg(not(target_arch = "aarch64"))]
    let modes: &[(&str, bool, u8)] = &[("generic", false, 2), ("fast", true, 2)];

    for &(tag, fast_on, jf) in modes {
        let c = arena_alloc(code.len() + 16, 16);
        let d = arena_alloc(n.max(64), 32);
        let prev = TEST_FAST_OFF.with(|f| f.replace(!fast_on));
        #[cfg(target_arch = "aarch64")]
        jit::force(jf);
        #[cfg(not(target_arch = "aarch64"))]
        let _ = jf;
        let out = unsafe {
            core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
            core::ptr::write_bytes(c.add(code.len()), 0xCC, 16);
            core::ptr::copy_nonoverlapping(init.as_ptr(), d, n);
            emu_run_full(c as u64, [d as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            core::slice::from_raw_parts(d, n).to_vec()
        };
        TEST_FAST_OFF.with(|f| f.set(prev));
        #[cfg(target_arch = "aarch64")]
        jit::force(0);
        results.push((tag, out));
    }
    let first = &results[0].1;
    for (tag, got) in &results[1..] {
        assert_eq!(got, first, "{tag} diverged from generic interpreter");
    }
    results.into_iter().next().unwrap().1
}

/// Assert every instruction in `code` lowers to a JIT-supported native `FastOp` (never `K_GENERIC`
/// or `K_INSN`).
fn assert_all_jit_supported(name: &str, code: &[u8]) {
    let mut dec = iced_x86::Decoder::with_ip(64, code, 0x1000, iced_x86::DecoderOptions::NONE);
    let mut bad = Vec::new();
    while dec.can_decode() {
        let ins = dec.decode();
        let op = fast::lower(&ins);
        let ok = op.kind != fast::K_GENERIC && op.kind != fast::K_INSN && {
            #[cfg(target_arch = "aarch64")]
            {
                jit::supported(&op)
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                true
            }
        };
        if !ok {
            bad.push(format!("{ins} (kind={})", op.kind));
        }
    }
    assert!(bad.is_empty(), "{name}: fell back to generic/unsupported:\n{}", bad.join("\n"));
}

#[test]
fn sha_ni_all_7_ops_parity_reg_and_mem() {
    use iced_x86::code_asm::*;
    // Buffer layout (0x400 bytes):
    //   [0x000..0x010]: xmm0 (WK for sha256rnds2)
    //   [0x010..0x020]: src1
    //   [0x023..0x033]: src2 (intentionally misaligned by +3 to verify unaligned guest loads!)
    //   [0x040..0x050]: ymm_hi sentinel (must be preserved by all legacy SSE SHA-NI instructions)
    // Outputs (each 16 bytes, reg form then mem form, plus ymm_hi check):
    //   0x080..0x0A0: sha1rnds4 imm=0 (reg, mem)
    //   0x0A0..0x0C0: sha1rnds4 imm=1 (reg, mem)
    //   0x0C0..0x0E0: sha1rnds4 imm=2 (reg, mem)
    //   0x0E0..0x100: sha1rnds4 imm=3 (reg, mem)
    //   0x100..0x120: sha1nexte (reg, mem)
    //   0x120..0x140: sha1msg1 (reg, mem)
    //   0x140..0x160: sha1msg2 (reg, mem)
    //   0x160..0x180: sha256rnds2 (reg, mem)
    //   0x180..0x1A0: sha256msg1 (reg, mem)
    //   0x1A0..0x1C0: sha256msg2 (reg, mem)
    //   0x1C0..0x1D0: sha256rnds2 xmm0, xmm0, xmm0 (dst == src2 == xmm0 aliasing)
    //   0x1D0..0x1E0: sha1nexte xmm1, xmm1 (dst == src2 aliasing)
    //   0x1E0..0x1F0: ymm1 upper 128-bit lane after sha256rnds2 (must equal sentinel)
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, rcx).unwrap();
    a.vmovdqu(xmm0, xmmword_ptr(r8 + 0x00)).unwrap();
    a.vmovdqu(xmm2, xmmword_ptr(r8 + 0x23)).unwrap();
    a.vmovdqu(xmm5, xmmword_ptr(r8 + 0x40)).unwrap(); // upper-lane sentinel

    // Helper: load src1 into xmm1 with sentinel in ymm1_hi, run op, store xmm1.
    macro_rules! check_pair {
        ($off:expr, $reg_op:expr, $mem_op:expr) => {
            a.vmovdqu(xmm1, xmmword_ptr(r8 + 0x10)).unwrap();
            a.vinserti128(ymm1, ymm1, xmm5, 1).unwrap();
            $reg_op;
            a.movdqu(xmmword_ptr(r8 + $off), xmm1).unwrap();

            a.vmovdqu(xmm1, xmmword_ptr(r8 + 0x10)).unwrap();
            a.vinserti128(ymm1, ymm1, xmm5, 1).unwrap();
            $mem_op;
            a.movdqu(xmmword_ptr(r8 + ($off + 0x10)), xmm1).unwrap();
        };
    }

    check_pair!(0x080, a.sha1rnds4(xmm1, xmm2, 0).unwrap(), a.sha1rnds4(xmm1, xmmword_ptr(r8 + 0x23), 0).unwrap());
    check_pair!(0x0A0, a.sha1rnds4(xmm1, xmm2, 1).unwrap(), a.sha1rnds4(xmm1, xmmword_ptr(r8 + 0x23), 1).unwrap());
    check_pair!(0x0C0, a.sha1rnds4(xmm1, xmm2, 2).unwrap(), a.sha1rnds4(xmm1, xmmword_ptr(r8 + 0x23), 2).unwrap());
    check_pair!(0x0E0, a.sha1rnds4(xmm1, xmm2, 3).unwrap(), a.sha1rnds4(xmm1, xmmword_ptr(r8 + 0x23), 3).unwrap());
    check_pair!(0x100, a.sha1nexte(xmm1, xmm2).unwrap(), a.sha1nexte(xmm1, xmmword_ptr(r8 + 0x23)).unwrap());
    check_pair!(0x120, a.sha1msg1(xmm1, xmm2).unwrap(), a.sha1msg1(xmm1, xmmword_ptr(r8 + 0x23)).unwrap());
    check_pair!(0x140, a.sha1msg2(xmm1, xmm2).unwrap(), a.sha1msg2(xmm1, xmmword_ptr(r8 + 0x23)).unwrap());
    check_pair!(0x160, a.sha256rnds2(xmm1, xmm2).unwrap(), a.sha256rnds2(xmm1, xmmword_ptr(r8 + 0x23)).unwrap());
    // Capture ymm1 upper 128-bit lane right after sha256rnds2 (must still equal sentinel!).
    a.vextracti128(xmmword_ptr(r8 + 0x1E0), ymm1, 1).unwrap();
    check_pair!(0x180, a.sha256msg1(xmm1, xmm2).unwrap(), a.sha256msg1(xmm1, xmmword_ptr(r8 + 0x23)).unwrap());
    check_pair!(0x1A0, a.sha256msg2(xmm1, xmm2).unwrap(), a.sha256msg2(xmm1, xmmword_ptr(r8 + 0x23)).unwrap());

    // Self-aliasing edge cases:
    a.vmovdqu(xmm1, xmmword_ptr(r8 + 0x10)).unwrap();
    a.sha1nexte(xmm1, xmm1).unwrap();
    a.movdqu(xmmword_ptr(r8 + 0x1D0), xmm1).unwrap();

    a.sha256rnds2(xmm0, xmm0).unwrap();
    a.movdqu(xmmword_ptr(r8 + 0x1C0), xmm0).unwrap();
    a.ret().unwrap();

    let code = a.assemble(0x1000).unwrap();
    assert_all_jit_supported("sha_ni_all_7_ops", &code);

    let mut rng = XorShift(0x5A82_7999_6ED9_EBA1);
    for case in 0..32 {
        let mut init = vec![0u8; 0x240];
        let x0 = if case == 0 { 0 } else if case == 1 { u128::MAX } else { ((rng.next() as u128) << 64) | rng.next() as u128 };
        let s1 = if case == 0 { 0 } else if case == 1 { u128::MAX } else { ((rng.next() as u128) << 64) | rng.next() as u128 };
        let s2 = if case == 0 { 0 } else if case == 1 { u128::MAX } else { ((rng.next() as u128) << 64) | rng.next() as u128 };
        let sent = 0xCAFE_BABE_DEAD_BEEF_0123_4567_89AB_CDEFu128;
        init[0x00..0x10].copy_from_slice(&x0.to_le_bytes());
        init[0x10..0x20].copy_from_slice(&s1.to_le_bytes());
        init[0x23..0x33].copy_from_slice(&s2.to_le_bytes());
        init[0x40..0x50].copy_from_slice(&sent.to_le_bytes());

        let out = run_all_3_paths(&code, &init);
        let rd = |off: usize| u128::from_le_bytes(out[off..off + 16].try_into().unwrap());
        // Reg form must equal mem form for every op, and ymm_hi must be untouched.
        for off in (0x080..0x1C0).step_by(0x20) {
            assert_eq!(rd(off), rd(off + 0x10), "case {case} off {off:#x}: reg vs mem");
        }
        assert_eq!(rd(0x1E0), sent, "case {case}: legacy SHA-NI clobbered ymm_hi");
        assert_eq!(rd(0x1C0), simd_fp::sha256rnds2(x0, x0, x0), "case {case}: sha256rnds2 self-alias");
        assert_eq!(rd(0x1D0), simd_fp::sha1nexte(s1, s1), "case {case}: sha1nexte self-alias");
    }

    // Verify XMM register cache (`AW2_JIT_XMMREG=1`) barrier coherence: dirty `xmm0`, `xmm1`, `xmm2`
    // via cache-aware `XClass::Aware` ops (`movaps`/`xorps`) immediately before `sha256rnds2`, then
    // consume `xmm1` immediately afterwards via another cache-aware op (`xorps`) in the same block!
    let mut c = CodeAssembler::new(64).unwrap();
    c.mov(r8, rcx).unwrap();
    c.movups(xmm0, xmmword_ptr(r8 + 0x00)).unwrap();
    c.movups(xmm1, xmmword_ptr(r8 + 0x10)).unwrap();
    c.movups(xmm2, xmmword_ptr(r8 + 0x20)).unwrap();
    c.xorps(xmm0, xmm2).unwrap();       // XClass::Aware: dirties cached v16 (xmm0)
    c.xorps(xmm1, xmm0).unwrap();       // XClass::Aware: dirties cached v17 (xmm1)
    c.sha256rnds2(xmm1, xmm2).unwrap(); // XClass::Barrier: must flush v16..v18 before reading Cpu::xmm
    c.xorps(xmm1, xmm0).unwrap();       // XClass::Aware: must reload updated Cpu::xmm[1]
    c.movups(xmmword_ptr(r8 + 0x30), xmm1).unwrap();
    c.ret().unwrap();
    let cache_code = c.assemble(0x1000).unwrap();
    assert_all_jit_supported("sha_ni_xmm_cache_barrier", &cache_code);
    let mut init = vec![0u8; 0x40];
    let v0 = 0x1122_3344_5566_7788_99AA_BBCC_DDEE_FF01u128;
    let v1 = 0xCAFE_BABE_1234_5678_9ABC_DEF0_FEDC_BA98u128;
    let v2 = 0x0F1E_2D3C_4B5A_6978_8796_A5B4_C3D2_E1F0u128;
    init[0x00..0x10].copy_from_slice(&v0.to_le_bytes());
    init[0x10..0x20].copy_from_slice(&v1.to_le_bytes());
    init[0x20..0x30].copy_from_slice(&v2.to_le_bytes());
    let out = run_all_3_paths(&cache_code, &init);
    let x0_d = v0 ^ v2;
    let x1_d = v1 ^ x0_d;
    let want = simd_fp::sha256rnds2(x1_d, v2, x0_d) ^ x0_d;
    assert_eq!(u128::from_le_bytes(out[0x30..0x40].try_into().unwrap()), want);
}

/// Assemble a full x86-64 Intel SHA-NI SHA-256 single-block compression routine (`rcx` = work buffer):
/// - `[rcx + 0x00..0x20]`: 8 x `u32` state `[A,B,C,D,E,F,G,H]` (read & updated in place)
/// - `[rcx + 0x20..0x60]`: 64-byte big-endian message block
/// - `[rcx + 0x60..0x160]`: 64 x `u32` FIPS 180-4 round constants `K[0..64]`
/// - `[rcx + 0x160..0x170]`: `pshufb` byte-swap mask
fn build_sha256_ni_compress() -> Vec<u8> {
    use iced_x86::code_asm::*;
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, rcx).unwrap();
    // Load [A,B,C,D] into xmm1 and [E,F,G,H] into xmm2, then pack into Intel SHA-NI layout:
    //   STATE0 (xmm1) = [F, E, B, A]
    //   STATE1 (xmm2) = [H, G, D, C]
    a.movdqu(xmm3, xmmword_ptr(r8 + 0x00)).unwrap(); // [A, B, C, D]
    a.movdqu(xmm4, xmmword_ptr(r8 + 0x10)).unwrap(); // [E, F, G, H]
    a.pshufd(xmm3, xmm3, 0x1B).unwrap(); // [D, C, B, A]
    a.pshufd(xmm4, xmm4, 0x1B).unwrap(); // [H, G, F, E]
    a.movdqa(xmm1, xmm4).unwrap();
    a.punpckhqdq(xmm1, xmm3).unwrap(); // STATE0 = [F, E, B, A]
    a.movdqa(xmm2, xmm4).unwrap();
    a.punpcklqdq(xmm2, xmm3).unwrap(); // STATE1 = [H, G, D, C]
    a.movdqa(xmm8, xmm1).unwrap();     // ABEF_SAVE
    a.movdqa(xmm9, xmm2).unwrap();     // CDGH_SAVE

    // Load 4 big-endian message vectors into xmm4..xmm7 = MSG0..MSG3
    a.movdqu(xmm10, xmmword_ptr(r8 + 0x160)).unwrap(); // bswap mask
    let mut msgs = [xmm4, xmm5, xmm6, xmm7];
    for (i, &m) in msgs.iter().enumerate() {
        a.movdqu(m, xmmword_ptr(r8 + 0x20 + (i as i32) * 16)).unwrap();
        a.pshufb(m, xmm10).unwrap();
    }

    // 16 groups of 4 rounds = 64 rounds
    for g in 0..16i32 {
        a.movdqa(xmm0, msgs[0]).unwrap();
        a.paddd(xmm0, xmmword_ptr(r8 + 0x60 + g * 16)).unwrap();
        a.sha256rnds2(xmm2, xmm1).unwrap();
        a.pshufd(xmm0, xmm0, 0x0E).unwrap();
        a.sha256rnds2(xmm1, xmm2).unwrap();

        if g < 12 {
            a.sha256msg1(msgs[0], msgs[1]).unwrap();
            a.movdqa(xmm3, msgs[3]).unwrap();
            a.palignr(xmm3, msgs[2], 4).unwrap();
            a.paddd(msgs[0], xmm3).unwrap();
            a.sha256msg2(msgs[0], msgs[3]).unwrap();
        }
        msgs.rotate_left(1);
    }

    a.paddd(xmm1, xmm8).unwrap();
    a.paddd(xmm2, xmm9).unwrap();
    // Unpack STATE0=[F,E,B,A], STATE1=[H,G,D,C] back to [A,B,C,D], [E,F,G,H]
    a.movdqa(xmm3, xmm2).unwrap();
    a.punpckhqdq(xmm3, xmm1).unwrap(); // [D, C, B, A]
    a.pshufd(xmm3, xmm3, 0x1B).unwrap(); // [A, B, C, D]
    a.movdqa(xmm4, xmm2).unwrap();
    a.punpcklqdq(xmm4, xmm1).unwrap(); // [H, G, F, E]
    a.pshufd(xmm4, xmm4, 0x1B).unwrap(); // [E, F, G, H]
    a.movdqu(xmmword_ptr(r8 + 0x00), xmm3).unwrap();
    a.movdqu(xmmword_ptr(r8 + 0x10), xmm4).unwrap();
    a.ret().unwrap();
    a.assemble(0x1000).unwrap()
}

fn pad_sha_blocks(msg: &[u8]) -> Vec<[u8; 64]> {
    let mut buf = msg.to_vec();
    buf.push(0x80);
    while (buf.len() % 64) != 56 {
        buf.push(0);
    }
    let bit_len = (msg.len() as u64) * 8;
    buf.extend_from_slice(&bit_len.to_be_bytes());
    buf.chunks_exact(64).map(|c| c.try_into().unwrap()).collect()
}

#[test]
fn sha256_ni_nist_fips_180_4_vectors() {
    #[rustfmt::skip]
    const K256: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    const IV256: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    const BSWAP_DWORD: [u8; 16] = [3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12];

    let code = build_sha256_ni_compress();
    assert_all_jit_supported("sha256_ni_compress", &code);

    let vectors: &[(&[u8], &str)] = &[
        (b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
        (b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        ),
    ];

    for &(msg, want_hex) in vectors {
        let mut state = IV256;
        for blk in pad_sha_blocks(msg) {
            let mut work = vec![0u8; 0x180];
            for (i, &w) in state.iter().enumerate() {
                work[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
            }
            work[0x20..0x60].copy_from_slice(&blk);
            for (i, &k) in K256.iter().enumerate() {
                work[0x60 + i * 4..0x60 + i * 4 + 4].copy_from_slice(&k.to_le_bytes());
            }
            work[0x160..0x170].copy_from_slice(&BSWAP_DWORD);

            let out = run_all_3_paths(&code, &work);
            for i in 0..8 {
                state[i] = u32::from_le_bytes(out[i * 4..i * 4 + 4].try_into().unwrap());
            }
        }
        let got_hex: String = state.iter().map(|w| format!("{w:08x}")).collect();
        assert_eq!(got_hex, want_hex, "SHA-256 NIST vector {:?}", core::str::from_utf8(msg));
    }
}

/// Assemble a full x86-64 Intel SHA-NI SHA-1 single-block compression routine (`rcx` = work buffer):
/// - `[rcx + 0x00..0x14]`: 5 x `u32` state `[A,B,C,D,E]` (read & updated in place)
/// - `[rcx + 0x20..0x60]`: 64-byte big-endian message block
/// - `[rcx + 0x60..0x70]`: `pshufb` full 16-byte reverse mask (`[15,14,...,0]`)
fn build_sha1_ni_compress() -> Vec<u8> {
    use iced_x86::code_asm::*;
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, rcx).unwrap();
    // ABCD (xmm0) = [D, C, B, A]; E0 (xmm1) = [0, 0, 0, E]
    a.movdqu(xmm0, xmmword_ptr(r8 + 0x00)).unwrap(); // [A, B, C, D]
    a.pshufd(xmm0, xmm0, 0x1B).unwrap();             // [D, C, B, A]
    a.pxor(xmm1, xmm1).unwrap();
    a.pinsrd(xmm1, dword_ptr(r8 + 0x10), 3).unwrap(); // [0, 0, 0, E]
    a.movdqa(xmm8, xmm0).unwrap();                    // ABCD_SAVE
    a.movdqa(xmm9, xmm1).unwrap();                    // E0_SAVE

    // Load MSG0..MSG3 (xmm4..xmm7) in big-endian reversed-dword order: MSG0 = [W3, W2, W1, W0]
    a.movdqu(xmm10, xmmword_ptr(r8 + 0x60)).unwrap();
    let mut msgs = [xmm4, xmm5, xmm6, xmm7];
    for (i, &m) in msgs.iter().enumerate() {
        a.movdqu(m, xmmword_ptr(r8 + 0x20 + (i as i32) * 16)).unwrap();
        a.pshufb(m, xmm10).unwrap();
    }

    // Group 0 (rounds 0..3)
    a.paddd(xmm1, msgs[0]).unwrap();
    a.movdqa(xmm2, xmm0).unwrap(); // E1 holds pre-round ABCD for next sha1nexte
    a.sha1rnds4(xmm0, xmm1, 0).unwrap();

    // Groups 1..19 (rounds 4..79)
    let mut e_cur = xmm2;
    let mut e_nxt = xmm1;
    for g in 1..20usize {
        let cur_msg = if g < 4 {
            msgs[g]
        } else {
            a.sha1msg1(msgs[0], msgs[1]).unwrap();
            a.pxor(msgs[0], msgs[2]).unwrap();
            a.sha1msg2(msgs[0], msgs[3]).unwrap();
            msgs.rotate_left(1);
            msgs[3]
        };
        a.sha1nexte(e_cur, cur_msg).unwrap();
        a.movdqa(e_nxt, xmm0).unwrap();
        a.sha1rnds4(xmm0, e_cur, (g / 5) as i32).unwrap();
        core::mem::swap(&mut e_cur, &mut e_nxt);
    }

    // Finalize: H4 += ROL32(A_76, 30); [H0..H3] += [A..D]
    a.sha1nexte(e_cur, xmm9).unwrap();
    a.paddd(xmm0, xmm8).unwrap();
    a.pshufd(xmm0, xmm0, 0x1B).unwrap(); // [A, B, C, D]
    a.movdqu(xmmword_ptr(r8 + 0x00), xmm0).unwrap();
    a.pextrd(dword_ptr(r8 + 0x10), e_cur, 3).unwrap();
    a.ret().unwrap();
    a.assemble(0x1000).unwrap()
}

#[test]
fn sha1_ni_nist_fips_180_4_vectors() {
    const IV1: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
    const REV16: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    let code = build_sha1_ni_compress();
    assert_all_jit_supported("sha1_ni_compress", &code);

    let vectors: &[(&[u8], &str)] = &[
        (b"", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
        (b"abc", "a9993e364706816aba3e25717850c26c9cd0d89d"),
        (
            b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            "84983e441c3bd26ebaae4aa1f95129e5e54670f1",
        ),
    ];

    for &(msg, want_hex) in vectors {
        let mut state = IV1;
        for blk in pad_sha_blocks(msg) {
            let mut work = vec![0u8; 0x80];
            for (i, &w) in state.iter().enumerate() {
                work[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
            }
            work[0x20..0x60].copy_from_slice(&blk);
            work[0x60..0x70].copy_from_slice(&REV16);

            let out = run_all_3_paths(&code, &work);
            for i in 0..5 {
                state[i] = u32::from_le_bytes(out[i * 4..i * 4 + 4].try_into().unwrap());
            }
        }
        let got_hex: String = state.iter().map(|w| format!("{w:08x}")).collect();
        assert_eq!(got_hex, want_hex, "SHA-1 NIST vector {:?}", core::str::from_utf8(msg));
    }
}

#[test]
fn cpuid_leaf7_advertises_sha_ni_on_all_paths() {
    use iced_x86::code_asm::*;
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, rcx).unwrap();
    a.mov(eax, 7).unwrap();
    a.xor(ecx, ecx).unwrap();
    a.cpuid().unwrap();
    a.mov(dword_ptr(r8 + 0x00), eax).unwrap();
    a.mov(dword_ptr(r8 + 0x04), ebx).unwrap();
    a.mov(dword_ptr(r8 + 0x08), ecx).unwrap();
    a.mov(dword_ptr(r8 + 0x0C), edx).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0x1000).unwrap();


    let out = run_all_3_paths(&code, &[0u8; 64]);
    let ebx_val = u32::from_le_bytes(out[4..8].try_into().unwrap());
    let host_sha = super::features::host().has(super::features::HostFeatures::SHA1 | super::features::HostFeatures::SHA256);
    assert_eq!(ebx_val & (1 << 29) != 0, host_sha);
    assert_eq!(ebx_val & ((1 << 7) | (1 << 10)), 0);
    assert_eq!(ebx_val as u64, crate::simd_ops::cpuid_x64(7, 0).1);
}

#[test]
fn promoted_hot_generic_ops_jit_parity() {
    use iced_x86::code_asm::*;
    // 1. 32-bit address-size override LEA (MSVC CRT strlen/memchr pattern `67 8d 04 0a`),
    //    movnti (32/64-bit aligned & unaligned), cwd (positive/negative/zero preserving RDX[63:16]),
    //    rdsspq / wait (NOPs preserving regs & flags).
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, rcx).unwrap();

    // Test 32-bit address-size LEA wraparound & zero-extension:
    // edx = 0xFFFF_FFFE, ecx = 5 -> (edx + ecx) mod 2^32 = 3; rdx/rcx upper 32 bits non-zero!
    a.mov(rdx, 0x1111_2222_FFFF_FFFEu64 as i64).unwrap();
    a.mov(rcx, 0x3333_4444_0000_0005u64 as i64).unwrap();
    a.lea(eax, ptr(edx + ecx)).unwrap();            // 67 8d 04 0a -> 3
    a.mov(qword_ptr(r8 + 0x00), rax).unwrap();
    a.mov(rax, 0xAAAA_BBBB_CCCC_DDDDu64 as i64).unwrap();
    a.lea(rax, ptr(edx + ecx)).unwrap();            // 67 48 8d 04 0a -> 3 (zero-extended!)
    a.mov(qword_ptr(r8 + 0x08), rax).unwrap();
    a.mov(rax, 0xAAAA_BBBB_CCCC_DDDDu64 as i64).unwrap();
    a.lea(rax, ptr(edx)).unwrap();                  // 67 48 8d 02 -> 0xFFFF_FFFE (zero-extended!)
    a.mov(qword_ptr(r8 + 0x10), rax).unwrap();
    a.lea(eax, ptr(edx + ecx * 4 - 16)).unwrap();   // 67 8d 44 8a f0 -> (0xFFFFFFFE + 20 - 16) mod 2^32 = 2
    a.mov(qword_ptr(r8 + 0x18), rax).unwrap();
    a.mov(rax, 0xAAAA_BBBB_CCCC_0000u64 as i64).unwrap();
    a.lea(ax, ptr(edx + ecx)).unwrap();             // 67 66 8d 04 0a -> 0xAAAA_BBBB_CCCC_0003
    a.mov(qword_ptr(r8 + 0x20), rax).unwrap();

    // Test movnti (32-bit & 64-bit, aligned and misaligned by +3):
    a.mov(r9, 0x0123_4567_89AB_CDEFu64 as i64).unwrap();
    a.movnti(qword_ptr(r8 + 0x28), r9).unwrap();
    a.movnti(qword_ptr(r8 + 0x33), r9).unwrap();
    a.mov(r10d, 0xDEAD_BEEFu32 as i32).unwrap();
    a.movnti(dword_ptr(r8 + 0x40), r10d).unwrap();
    a.movnti(dword_ptr(r8 + 0x45), r10d).unwrap();

    // Test cwd: positive, negative, boundary 0x7FFF & 0x8000, preserving RDX[63:16]
    a.mov(rdx, 0x1234_5678_9ABC_5555u64 as i64).unwrap();
    a.mov(rax, 0xFFFF_FFFF_FFFF_7FFFu64 as i64).unwrap();
    a.cwd().unwrap();
    a.mov(qword_ptr(r8 + 0x50), rdx).unwrap(); // 0x1234_5678_9ABC_0000
    a.mov(rdx, 0x1234_5678_9ABC_0000u64 as i64).unwrap();
    a.mov(rax, 0x0000_0000_0000_8000u64 as i64).unwrap();
    a.cwd().unwrap();
    a.mov(qword_ptr(r8 + 0x58), rdx).unwrap(); // 0x1234_5678_9ABC_FFFF

    // Test rdsspq + wait (must leave r10 untouched as NOPs)
    a.mov(r10, 0x5555_6666_7777_8888u64 as i64).unwrap();
    a.rdsspq(r10).unwrap();
    a.wait().unwrap();
    a.mov(qword_ptr(r8 + 0x60), r10).unwrap();
    a.ret().unwrap();

    let code = a.assemble(0x1000).unwrap();
    assert_all_jit_supported("promoted_hot_gp_ops", &code);

    let out = run_all_3_paths(&code, &[0u8; 0x80]);
    let rd64 = |off: usize| u64::from_le_bytes(out[off..off + 8].try_into().unwrap());
    let rd32 = |off: usize| u32::from_le_bytes(out[off..off + 4].try_into().unwrap());
    assert_eq!(rd64(0x00), 3);
    assert_eq!(rd64(0x08), 3);
    assert_eq!(rd64(0x10), 0xFFFF_FFFE);
    assert_eq!(rd64(0x18), 2);
    assert_eq!(rd64(0x20), 0xAAAA_BBBB_CCCC_0003);
    assert_eq!(rd64(0x28), 0x0123_4567_89AB_CDEF);
    assert_eq!(rd64(0x33), 0x0123_4567_89AB_CDEF);
    assert_eq!(rd32(0x40), 0xDEAD_BEEF);
    assert_eq!(rd32(0x45), 0xDEAD_BEEF);
    assert_eq!(rd64(0x50), 0x1234_5678_9ABC_0000);
    assert_eq!(rd64(0x58), 0x1234_5678_9ABC_FFFF);
    assert_eq!(rd64(0x60), 0x5555_6666_7777_8888);

    // 2. AVX2 256-bit register-count & memory-count packed shifts (`vpsllw/d/q`, `vpsrlw/d/q`, `vpsraw/d`):
    //    Buffer: [0x00..0x20] = ymm_src, [0x20..0x30] = xmm_cnt (also read from mem),
    //            [0x40..0x240] = 16 x 32-byte outputs (8 reg-count + 8 mem-count).
    let mut b = CodeAssembler::new(64).unwrap();
    b.mov(r8, rcx).unwrap();
    b.vmovdqu(ymm2, ymmword_ptr(r8 + 0x00)).unwrap();
    b.vmovdqu(xmm1, xmmword_ptr(r8 + 0x20)).unwrap();

    b.vpsllw(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x040), ymm3).unwrap();
    b.vpsrlw(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x060), ymm3).unwrap();
    b.vpsraw(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x080), ymm3).unwrap();
    b.vpslld(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x0A0), ymm3).unwrap();
    b.vpsrld(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x0C0), ymm3).unwrap();
    b.vpsrad(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x0E0), ymm3).unwrap();
    b.vpsllq(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x100), ymm3).unwrap();
    b.vpsrlq(ymm3, ymm2, xmm1).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x120), ymm3).unwrap();

    b.vpsllw(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x140), ymm3).unwrap();
    b.vpsrlw(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x160), ymm3).unwrap();
    b.vpsraw(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x180), ymm3).unwrap();
    b.vpslld(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x1A0), ymm3).unwrap();
    b.vpsrld(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x1C0), ymm3).unwrap();
    b.vpsrad(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x1E0), ymm3).unwrap();
    b.vpsllq(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x200), ymm3).unwrap();
    b.vpsrlq(ymm3, ymm2, xmmword_ptr(r8 + 0x20)).unwrap();
    b.vmovdqu(ymmword_ptr(r8 + 0x220), ymm3).unwrap();
    b.vzeroupper().unwrap();
    b.ret().unwrap();

    let vcode = b.assemble(0x1000).unwrap();
    assert_all_jit_supported("avx2_256_reg_shifts", &vcode);

    for cnt in [0u64, 1, 7, 15, 16, 31, 32, 63, 64, 128, u64::MAX] {
        let mut init = vec![0u8; 0x240];
        let lo: [u64; 2] = [0x8001_7FFF_FEDC_BA98, 0x0123_4567_89AB_CDEF];
        let hi: [u64; 2] = [0xFFFF_0000_8000_0001, 0x7654_3210_CAFE_BABE];
        init[0x00..0x08].copy_from_slice(&lo[0].to_le_bytes());
        init[0x08..0x10].copy_from_slice(&lo[1].to_le_bytes());
        init[0x10..0x18].copy_from_slice(&hi[0].to_le_bytes());
        init[0x18..0x20].copy_from_slice(&hi[1].to_le_bytes());
        init[0x20..0x28].copy_from_slice(&cnt.to_le_bytes());
        // Deliberately put junk in upper 64 bits of xmm_cnt (must be ignored by x86!).
        init[0x28..0x30].copy_from_slice(&0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes());

        let out = run_all_3_paths(&vcode, &init);
        // Reg-count output must match mem-count output for all 8 ops!
        assert_eq!(&out[0x040..0x140], &out[0x140..0x240], "cnt={cnt}: reg vs mem mismatch");
    }
}
