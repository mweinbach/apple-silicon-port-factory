//! `renoircore.windowsdesktop.dll` functions lifted by `tools/decomp_scale/gen_subsystems.py --image` (generated; do not edit).
//! 2865 lifted of 2865 candidates (2453 .pdata roots, 0 with handlers and 0 context exports skipped, 415 leaves of which 398 internal).
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
pub mod e00;

use super::eh::EhMeta;

/// 7 lifted functions with a `__CxxFrameHandler4` FuncInfo (`super::eh::EhMeta`).
pub static EH_META: &[EhMeta] = &[
    EhMeta { va: 0x700206e820, funcinfo_rva: 0x0, handler: 0, shape: 0, frame_reg: 0, establisher_off: 0x1e8, conts: &[0x700206ea93] },
    EhMeta { va: 0x7002070c00, funcinfo_rva: 0x0, handler: 0, shape: 0, frame_reg: 0, establisher_off: 0x38, conts: &[0x7002070c1a] },
    EhMeta { va: 0x7002087f5c, funcinfo_rva: 0x1e2f74, handler: 1, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[] },
    EhMeta { va: 0x70020882ec, funcinfo_rva: 0x1e2f88, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x18, conts: &[0x700208837d] },
    EhMeta { va: 0x70020884b4, funcinfo_rva: 0x1e2fbc, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[] },
    EhMeta { va: 0x70020885cc, funcinfo_rva: 0x1e2ff0, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x38, conts: &[] },
    EhMeta { va: 0x700208864c, funcinfo_rva: 0x1e3048, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x58, conts: &[0x7002088767] },
];

pub static IMAGE: super::LiftedImage = super::LiftedImage {
    name: "renoircore.windowsdesktop.dll",
    base: 0x7001f04000,
    text_rva: 0x1000,
    text_len: 0x1843de,
    text_fnv: 0x0dc05fec8649dc4a,
    slices: &[e00::ENTRIES],
    regfile_slices: &[],
    eh_meta: EH_META,
};
