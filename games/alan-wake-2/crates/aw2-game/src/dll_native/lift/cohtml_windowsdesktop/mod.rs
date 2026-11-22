//! `cohtml.windowsdesktop.dll` functions lifted by `tools/decomp_scale/gen_subsystems.py --image` (generated; do not edit).
//! 11396 lifted of 11396 candidates (10291 .pdata roots, 0 with handlers and 0 context exports skipped, 1107 leaves of which 1034 internal).
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

pub mod t00;
pub use t00::*;
pub mod t01;
pub use t01::*;
pub mod t02;
pub use t02::*;
pub mod t03;
pub use t03::*;
pub mod t04;
pub use t04::*;
pub mod t05;
pub use t05::*;
pub mod t06;
pub use t06::*;
pub mod t07;
pub use t07::*;
pub mod t08;
pub use t08::*;
pub mod t09;
pub use t09::*;
pub mod t10;
pub use t10::*;
pub mod t11;
pub use t11::*;
pub mod t12;
pub use t12::*;
pub mod t13;
pub use t13::*;
pub mod t14;
pub use t14::*;
pub mod t15;
pub use t15::*;
pub mod t16;
pub use t16::*;
pub mod t17;
pub use t17::*;
pub mod t18;
pub use t18::*;
pub mod t19;
pub use t19::*;
pub mod t20;
pub use t20::*;
pub mod t21;
pub use t21::*;
pub mod t22;
pub use t22::*;
pub mod t23;
pub use t23::*;
pub mod t24;
pub use t24::*;
pub mod t25;
pub use t25::*;
pub mod t26;
pub use t26::*;
pub mod t27;
pub use t27::*;
pub mod t28;
pub use t28::*;
pub mod t29;
pub use t29::*;
pub mod t30;
pub use t30::*;
pub mod t31;
pub use t31::*;
pub mod t32;
pub use t32::*;
pub mod t33;
pub use t33::*;
pub mod t34;
pub use t34::*;
pub mod t35;
pub use t35::*;
pub mod t36;
pub use t36::*;
pub mod t37;
pub use t37::*;
pub mod t38;
pub use t38::*;
pub mod t39;
pub use t39::*;
pub mod t40;
pub use t40::*;
pub mod t41;
pub use t41::*;
pub mod t42;
pub use t42::*;
pub mod t43;
pub use t43::*;
pub mod t44;
pub use t44::*;
pub mod t45;
pub use t45::*;
pub mod t46;
pub use t46::*;
pub mod t47;
pub use t47::*;
pub mod t48;
pub use t48::*;
pub mod t49;
pub use t49::*;
pub mod t50;
pub use t50::*;
pub mod t51;
pub use t51::*;
pub mod t52;
pub use t52::*;
pub mod t53;
pub use t53::*;
pub mod t54;
pub use t54::*;
pub mod t55;
pub use t55::*;
pub mod t56;
pub use t56::*;
pub mod t57;
pub use t57::*;
pub mod t58;
pub use t58::*;
pub mod t59;
pub use t59::*;
pub mod t60;
pub use t60::*;
pub mod t61;
pub use t61::*;
pub mod t62;
pub use t62::*;
pub mod t63;
pub use t63::*;
pub mod t64;
pub use t64::*;
pub mod t65;
pub use t65::*;
pub mod t66;
pub use t66::*;
pub mod t67;
pub use t67::*;
pub mod t68;
pub use t68::*;
pub mod t69;
pub use t69::*;
pub mod t70;
pub use t70::*;
pub mod t71;
pub use t71::*;
pub mod t72;
pub use t72::*;
pub mod t73;
pub use t73::*;
pub mod t74;
pub use t74::*;
pub mod t75;
pub use t75::*;
pub mod t76;
pub use t76::*;
pub mod t77;
pub use t77::*;
pub mod t78;
pub use t78::*;
pub mod t79;
pub use t79::*;
pub mod t80;
pub use t80::*;
pub mod t81;
pub use t81::*;
pub mod t82;
pub use t82::*;
pub mod t83;
pub use t83::*;
pub mod t84;
pub use t84::*;
pub mod t85;
pub use t85::*;
pub mod t86;
pub use t86::*;
pub mod t87;
pub use t87::*;
pub mod t88;
pub use t88::*;
pub mod t89;
pub use t89::*;
pub mod e00;
pub mod e01;
pub mod e02;

use super::eh::EhMeta;

/// 5 lifted functions with a `__CxxFrameHandler4` FuncInfo (`super::eh::EhMeta`).
pub static EH_META: &[EhMeta] = &[
    EhMeta { va: 0x70005a3fc4, funcinfo_rva: 0x6831e8, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x18, conts: &[0x70005a4055] },
    EhMeta { va: 0x70005a41ec, funcinfo_rva: 0x683210, handler: 1, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[] },
    EhMeta { va: 0x70005a42e8, funcinfo_rva: 0x683230, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[] },
    EhMeta { va: 0x70005a4400, funcinfo_rva: 0x683264, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x38, conts: &[] },
    EhMeta { va: 0x70005a4480, funcinfo_rva: 0x6832bc, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x58, conts: &[0x70005a459b] },
];

pub static IMAGE: super::LiftedImage = super::LiftedImage {
    name: "cohtml.windowsdesktop.dll",
    base: 0x7000000000,
    text_rva: 0x1000,
    text_len: 0x5a6d53,
    text_fnv: 0x56591bf063615c35,
    slices: &[e00::ENTRIES, e01::ENTRIES, e02::ENTRIES],
    regfile_slices: &[],
    eh_meta: EH_META,
};
