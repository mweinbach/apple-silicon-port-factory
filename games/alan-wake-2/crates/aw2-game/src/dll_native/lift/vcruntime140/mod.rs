//! `vcruntime140.dll` functions lifted by `tools/decomp_scale/gen_subsystems.py --image` (generated; do not edit).
//! 289 lifted of 289 candidates (253 .pdata roots, 5 with handlers and 11 context exports skipped, 57 leaves of which 40 internal).
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

pub mod t00;
pub use t00::*;
pub mod t01;
pub use t01::*;
pub mod e00;

use super::eh::EhMeta;

/// 14 lifted functions with a `__CxxFrameHandler4` FuncInfo (`super::eh::EhMeta`).
pub static EH_META: &[EhMeta] = &[
    EhMeta { va: 0x7001ea6c38, funcinfo_rva: 0x15c00, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[0x7001ea6df9] },
    EhMeta { va: 0x7001ea6e1c, funcinfo_rva: 0x15c40, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[0x7001ea6fe0] },
    EhMeta { va: 0x7001ea7004, funcinfo_rva: 0x15c7c, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[0x7001ea70a9] },
    EhMeta { va: 0x7001ea70c8, funcinfo_rva: 0x15ca8, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[0x7001ea716e] },
    EhMeta { va: 0x7001ea89a0, funcinfo_rva: 0x15b98, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[] },
    EhMeta { va: 0x7001ea8c50, funcinfo_rva: 0x15ac0, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0xa8, conts: &[0x7001ea8d4a] },
    EhMeta { va: 0x7001ea8e40, funcinfo_rva: 0x15b4c, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0xd8, conts: &[0x7001ea8f91] },
    EhMeta { va: 0x7001ea92d8, funcinfo_rva: 0x15a68, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x68, conts: &[0x7001ea93f4] },
    EhMeta { va: 0x7001ea9464, funcinfo_rva: 0x15b04, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x138, conts: &[0x7001ea970a] },
    EhMeta { va: 0x7001eaa5f0, funcinfo_rva: 0x15e2c, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[0x7001eaa62a] },
    EhMeta { va: 0x7001eaa650, funcinfo_rva: 0x15dcc, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x78, conts: &[0x7001eaa76f] },
    EhMeta { va: 0x7001eaa7c0, funcinfo_rva: 0x15dfc, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x48, conts: &[0x7001eaa84b] },
    EhMeta { va: 0x7001eb32b0, funcinfo_rva: 0x15f0c, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0xd8, conts: &[] },
    EhMeta { va: 0x7001eb49b0, funcinfo_rva: 0x16304, handler: 3, shape: 0, frame_reg: 0, establisher_off: 0x28, conts: &[0x7001eb49f0] },
];

pub static IMAGE: super::LiftedImage = super::LiftedImage {
    name: "vcruntime140.dll",
    base: 0x7001ea4000,
    text_rva: 0x1000,
    text_len: 0x10f56,
    text_fnv: 0x96d11a0dcdeb4181,
    slices: &[e00::ENTRIES],
    regfile_slices: &[],
    eh_meta: EH_META,
};
