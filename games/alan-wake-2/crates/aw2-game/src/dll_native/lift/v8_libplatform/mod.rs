//! `v8_libplatform.dll` functions lifted by `tools/decomp_scale/gen_subsystems.py --image` (generated; do not edit).
//! 848 lifted of 848 candidates (324 .pdata roots, 0 with handlers and 0 context exports skipped, 527 leaves of which 343 internal).
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

pub mod t00;
pub use t00::*;
pub mod t01;
pub use t01::*;
pub mod t02;
pub use t02::*;
pub mod e00;

use super::eh::EhMeta;

/// 4 lifted functions with a `__CxxFrameHandler4` FuncInfo (`super::eh::EhMeta`).
pub static EH_META: &[EhMeta] = &[
    EhMeta { va: 0x7001ee8c04, funcinfo_rva: 0x22e5c, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[] },
    EhMeta { va: 0x7001ee8d60, funcinfo_rva: 0x22ea0, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x38, conts: &[] },
    EhMeta { va: 0x7001ee8e08, funcinfo_rva: 0x22f18, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x58, conts: &[0x7001ee8f23] },
    EhMeta { va: 0x7001ee9d58, funcinfo_rva: 0x23030, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x18, conts: &[0x7001ee9de9] },
];

pub static IMAGE: super::LiftedImage = super::LiftedImage {
    name: "v8_libplatform.dll",
    base: 0x7001ed0000,
    text_rva: 0x1000,
    text_len: 0x1cf84,
    text_fnv: 0xb56ed0726a404778,
    slices: &[e00::ENTRIES],
    regfile_slices: &[],
    eh_meta: EH_META,
};
