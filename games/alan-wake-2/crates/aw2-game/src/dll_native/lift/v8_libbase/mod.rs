//! `v8_libbase.dll` functions lifted by `tools/decomp_scale/gen_subsystems.py --image` (generated; do not edit).
//! 1413 lifted of 1413 candidates (381 .pdata roots, 0 with handlers and 0 context exports skipped, 1036 leaves of which 506 internal).
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

pub mod t00;
pub use t00::*;
pub mod t01;
pub use t01::*;
pub mod t02;
pub use t02::*;
pub mod e00;

use super::eh::EhMeta;

/// 5 lifted functions with a `__CxxFrameHandler4` FuncInfo (`super::eh::EhMeta`).
pub static EH_META: &[EhMeta] = &[
    EhMeta { va: 0x7001deca50, funcinfo_rva: 0x0, handler: 0, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[] },
    EhMeta { va: 0x7001dee804, funcinfo_rva: 0x37878, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x18, conts: &[0x7001dee895] },
    EhMeta { va: 0x7001deeaa4, funcinfo_rva: 0x37930, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[] },
    EhMeta { va: 0x7001deec00, funcinfo_rva: 0x37974, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x38, conts: &[] },
    EhMeta { va: 0x7001deeca8, funcinfo_rva: 0x379ec, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x58, conts: &[0x7001deedc3] },
];

pub static IMAGE: super::LiftedImage = super::LiftedImage {
    name: "v8_libbase.dll",
    base: 0x7001dc4000,
    text_rva: 0x1000,
    text_len: 0x2f9d0,
    text_fnv: 0x8f9ffed2f263aeb2,
    slices: &[e00::ENTRIES],
    regfile_slices: &[],
    eh_meta: EH_META,
};
