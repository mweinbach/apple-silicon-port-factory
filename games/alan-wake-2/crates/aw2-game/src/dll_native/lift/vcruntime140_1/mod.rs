//! `vcruntime140_1.dll` functions lifted by `tools/decomp_scale/gen_subsystems.py --image` (generated; do not edit).
//! 97 lifted of 97 candidates (87 .pdata roots, 1 with handlers and 3 context exports skipped, 16 leaves of which 16 internal).
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

pub mod t00;
pub use t00::*;
pub mod e00;

use super::eh::EhMeta;

/// 6 lifted functions with a `__CxxFrameHandler4` FuncInfo (`super::eh::EhMeta`).
pub static EH_META: &[EhMeta] = &[
    EhMeta { va: 0x7001ec51a0, funcinfo_rva: 0x694c, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[0x7001ec51e8] },
    EhMeta { va: 0x7001ec52d0, funcinfo_rva: 0x6acc, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[0x7001ec5494] },
    EhMeta { va: 0x7001ec54b8, funcinfo_rva: 0x6b08, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[0x7001ec555e] },
    EhMeta { va: 0x7001ec6354, funcinfo_rva: 0x6a64, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[] },
    EhMeta { va: 0x7001ec6600, funcinfo_rva: 0x69fc, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0xd8, conts: &[0x7001ec6751] },
    EhMeta { va: 0x7001ec6a94, funcinfo_rva: 0x69b4, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x138, conts: &[0x7001ec6d36] },
];

pub static IMAGE: super::LiftedImage = super::LiftedImage {
    name: "vcruntime140_1.dll",
    base: 0x7001ec4000,
    text_rva: 0x1000,
    text_len: 0x3a70,
    text_fnv: 0x4f904ce882a4a54f,
    slices: &[e00::ENTRIES],
    regfile_slices: &[],
    eh_meta: EH_META,
};
