#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1409cc6b0, "unknown_1409cc6b0", fn_1409cc6b0 as *const ()), // sub_1409cc6b0
    Replacement::draft(0x1409cc790, "unknown_1409cc790", fn_1409cc790 as *const ()), // sub_1409cc790
    Replacement::draft(0x1409cc820, "unknown_1409cc820", fn_1409cc820 as *const ()), // sub_1409cc820
    Replacement::draft(0x1409cc8e0, "unknown_1409cc8e0", fn_1409cc8e0 as *const ()), // sub_1409cc8e0
    Replacement::draft(0x1409cc9b0, "unknown_1409cc9b0", fn_1409cc9b0 as *const ()), // sub_1409cc9b0
    Replacement::draft(0x1409cca90, "unknown_1409cca90", fn_1409cca90 as *const ()), // sub_1409cca90
    Replacement::draft_throws(0x1409ccc00, "unknown_1409ccc00", fn_1409ccc00 as *const (), Throws::Discard), // sub_1409ccc00
    Replacement::draft(0x1409ccd20, "unknown_1409ccd20", fn_1409ccd20 as *const ()), // sub_1409ccd20
    Replacement::draft_throws(0x1409cce00, "unknown_1409cce00", fn_1409cce00 as *const (), Throws::Discard), // sub_1409cce00
    Replacement::draft(0x1409ccf40, "unknown_1409ccf40", fn_1409ccf40 as *const ()), // sub_1409ccf40
    Replacement::draft_throws(0x1409ccff0, "unknown_1409ccff0", fn_1409ccff0 as *const (), Throws::Caught), // sub_1409ccff0
    Replacement::draft_throws(0x1409cd2b0, "unknown_1409cd2b0", fn_1409cd2b0 as *const (), Throws::Caught), // sub_1409cd2b0
    Replacement::draft_throws(0x1409cd530, "unknown_1409cd530", fn_1409cd530 as *const (), Throws::Caught), // sub_1409cd530
    Replacement::draft(0x1409cd7a0, "unknown_1409cd7a0", fn_1409cd7a0 as *const ()), // sub_1409cd7a0
    Replacement::draft(0x1409cd870, "unknown_1409cd870", fn_1409cd870 as *const ()), // sub_1409cd870
    Replacement::draft(0x1409cd940, "unknown_1409cd940", fn_1409cd940 as *const ()), // sub_1409cd940
    Replacement::draft_throws(0x1409cda30, "unknown_1409cda30", fn_1409cda30 as *const (), Throws::Discard), // sub_1409cda30
    Replacement::draft_throws(0x1409cdb00, "unknown_1409cdb00", fn_1409cdb00 as *const (), Throws::Discard), // sub_1409cdb00
    Replacement::draft(0x1409cdc50, "unknown_1409cdc50", tpl_70728::<0x28, 0x30, 0x1426cdcb0, 0x140a27d20> as *const ()), // sub_1409cdc50
    Replacement::draft(0x1409cdcc0, "unknown_1409cdcc0", tpl_70728::<0x28, 0x30, 0x1432a5038, 0x140a27d30> as *const ()), // sub_1409cdcc0
    Replacement::draft(0x1409cdd30, "unknown_1409cdd30", fn_1409cdd30 as *const ()), // sub_1409cdd30
    Replacement::draft(0x1409cddc0, "unknown_1409cddc0", fn_1409cddc0 as *const ()), // sub_1409cddc0
    Replacement::draft(0x1409cdee0, "unknown_1409cdee0", fn_1409cdee0 as *const ()), // sub_1409cdee0
    Replacement::draft_throws(0x1409ce050, "unknown_1409ce050", fn_1409ce050 as *const (), Throws::Discard), // sub_1409ce050
    Replacement::draft(0x1409ce170, "unknown_1409ce170", tpl_12267::<0x14387e138, 0x14243b0b0, 0x1408f8ff0, 0x8, 0x338, 0x10, 0x40, 0x18, 0x20, 0x24, 0x28, 0xfffffffffffffff8> as *const ()), // sub_1409ce170
    Replacement::draft_throws(0x1409ce230, "unknown_1409ce230", fn_1409ce230 as *const (), Throws::Discard), // sub_1409ce230
    Replacement::draft_throws(0x1409ce380, "unknown_1409ce380", fn_1409ce380 as *const (), Throws::Caught), // sub_1409ce380
    Replacement::draft_throws(0x1409ce5a0, "unknown_1409ce5a0", fn_1409ce5a0 as *const (), Throws::Caught), // sub_1409ce5a0
    Replacement::draft_throws(0x1409ce7c0, "unknown_1409ce7c0", fn_1409ce7c0 as *const (), Throws::Caught), // sub_1409ce7c0
    Replacement::draft(0x1409ce9e0, "unknown_1409ce9e0", fn_1409ce9e0 as *const ()), // sub_1409ce9e0
    Replacement::draft(0x1409ceaa0, "unknown_1409ceaa0", fn_1409ceaa0 as *const ()), // sub_1409ceaa0
    Replacement::draft(0x1409ceb60, "unknown_1409ceb60", tpl_12267::<0x14387e138, 0x14243b0b0, 0x1408f9ba0, 0x8, 0x338, 0x10, 0xe0, 0x18, 0x20, 0x24, 0x28, 0xfffffffffffffff8> as *const ()), // sub_1409ceb60
    Replacement::draft_throws(0x1409cec30, "unknown_1409cec30", fn_1409cec30 as *const (), Throws::Discard), // sub_1409cec30
    Replacement::draft_throws(0x1409ced10, "unknown_1409ced10", fn_1409ced10 as *const (), Throws::Discard), // sub_1409ced10
    Replacement::draft(0x1409cee90, "unknown_1409cee90", fn_1409cee90 as *const ()), // sub_1409cee90
    Replacement::draft(0x1409cf020, "unknown_1409cf020", fn_1409cf020 as *const ()), // sub_1409cf020
    Replacement::draft(0x1409cf0c0, "unknown_1409cf0c0", fn_1409cf0c0 as *const ()), // sub_1409cf0c0
    Replacement::draft(0x1409cf160, "unknown_1409cf160", fn_1409cf160 as *const ()), // sub_1409cf160
    Replacement::draft(0x1409cf230, "unknown_1409cf230", fn_1409cf230 as *const ()), // sub_1409cf230
    Replacement::draft_throws(0x1409cf440, "unknown_1409cf440", fn_1409cf440 as *const (), Throws::Discard), // sub_1409cf440
    Replacement::draft(0x1409cf5f0, "unknown_1409cf5f0", fn_1409cf5f0 as *const ()), // sub_1409cf5f0
    Replacement::draft_throws(0x1409cf6d0, "unknown_1409cf6d0", fn_1409cf6d0 as *const (), Throws::Discard), // sub_1409cf6d0
    Replacement::draft(0x1409cf8c0, "unknown_1409cf8c0", fn_1409cf8c0 as *const ()), // sub_1409cf8c0
    Replacement::draft_throws(0x1409cf960, "unknown_1409cf960", fn_1409cf960 as *const (), Throws::Caught), // sub_1409cf960
    Replacement::draft_throws(0x1409cff60, "unknown_1409cff60", fn_1409cff60 as *const (), Throws::Caught), // sub_1409cff60
    Replacement::draft_throws(0x1409d04b0, "unknown_1409d04b0", fn_1409d04b0 as *const (), Throws::Caught), // sub_1409d04b0
    Replacement::draft(0x1409d0a00, "unknown_1409d0a00", fn_1409d0a00 as *const ()), // sub_1409d0a00
    Replacement::draft(0x1409d0ab0, "unknown_1409d0ab0", fn_1409d0ab0 as *const ()), // sub_1409d0ab0
    Replacement::draft(0x1409d0bb0, "unknown_1409d0bb0", fn_1409d0bb0 as *const ()), // sub_1409d0bb0
    Replacement::draft_throws(0x1409d0cc0, "unknown_1409d0cc0", fn_1409d0cc0 as *const (), Throws::Discard), // sub_1409d0cc0
    Replacement::draft_throws(0x1409d0de0, "unknown_1409d0de0", fn_1409d0de0 as *const (), Throws::Discard), // sub_1409d0de0
    Replacement::draft(0x1409d1030, "unknown_1409d1030", fn_1409d1030 as *const ()), // sub_1409d1030
    Replacement::draft(0x1409d1120, "unknown_1409d1120", fn_1409d1120 as *const ()), // sub_1409d1120
    Replacement::draft(0x1409d1230, "unknown_1409d1230", fn_1409d1230 as *const ()), // sub_1409d1230
    Replacement::draft(0x1409d12a0, "unknown_1409d12a0", fn_1409d12a0 as *const ()), // sub_1409d12a0
    Replacement::draft(0x1409d1460, "unknown_1409d1460", fn_1409d1460 as *const ()), // sub_1409d1460
    Replacement::draft_throws(0x1409d18e0, "unknown_1409d18e0", fn_1409d18e0 as *const (), Throws::Discard), // sub_1409d18e0
    Replacement::draft(0x1409d1c70, "unknown_1409d1c70", fn_1409d1c70 as *const ()), // sub_1409d1c70
    Replacement::draft_throws(0x1409d1ed0, "unknown_1409d1ed0", fn_1409d1ed0 as *const (), Throws::Discard), // sub_1409d1ed0
    Replacement::draft(0x1409d2310, "unknown_1409d2310", fn_1409d2310 as *const ()), // sub_1409d2310
    Replacement::draft_throws(0x1409d2630, "unknown_1409d2630", fn_1409d2630 as *const (), Throws::Caught), // sub_1409d2630
    Replacement::draft_throws(0x1409d2c70, "unknown_1409d2c70", fn_1409d2c70 as *const (), Throws::Caught), // sub_1409d2c70
    Replacement::draft_throws(0x1409d3230, "unknown_1409d3230", fn_1409d3230 as *const (), Throws::Caught), // sub_1409d3230
    Replacement::draft(0x1409d37e0, "unknown_1409d37e0", fn_1409d37e0 as *const ()), // sub_1409d37e0
    Replacement::draft(0x1409d3990, "unknown_1409d3990", fn_1409d3990 as *const ()), // sub_1409d3990
    Replacement::draft(0x1409d3c00, "unknown_1409d3c00", fn_1409d3c00 as *const ()), // sub_1409d3c00
    Replacement::draft_throws(0x1409d3e90, "unknown_1409d3e90", fn_1409d3e90 as *const (), Throws::Discard), // sub_1409d3e90
    Replacement::draft_throws(0x1409d4160, "unknown_1409d4160", fn_1409d4160 as *const (), Throws::Discard), // sub_1409d4160
    Replacement::draft(0x1409d45c0, "unknown_1409d45c0", fn_1409d45c0 as *const ()), // sub_1409d45c0
    Replacement::draft(0x1409d46d0, "unknown_1409d46d0", fn_1409d46d0 as *const ()), // sub_1409d46d0
    Replacement::draft(0x1409d47d0, "unknown_1409d47d0", fn_1409d47d0 as *const ()), // sub_1409d47d0
    Replacement::draft(0x1409d48e0, "unknown_1409d48e0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143280288, 0x8> as *const ()), // sub_1409d48e0
    Replacement::draft(0x1409d4970, "unknown_1409d4970", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143280288, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409d4970
    Replacement::draft(0x1409d4a20, "unknown_1409d4a20", fn_1409d4a20 as *const ()), // sub_1409d4a20
    Replacement::draft(0x1409d4ac0, "unknown_1409d4ac0", fn_1409d4ac0 as *const ()), // sub_1409d4ac0
    Replacement::draft_throws(0x1409d4bb0, "unknown_1409d4bb0", tpl_6937::<0x14387e2f4, 0x14387e560, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140a00790, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409d4bb0
    Replacement::draft(0x1409d4c10, "unknown_1409d4c10", fn_1409d4c10 as *const ()), // sub_1409d4c10
    Replacement::draft_throws(0x1409d4cd0, "unknown_1409d4cd0", tpl_6937::<0x14387e2f4, 0x14387e560, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140a00940, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409d4cd0
    Replacement::draft(0x1409d4d30, "unknown_1409d4d30", fn_1409d4d30 as *const ()), // sub_1409d4d30
    Replacement::draft_throws(0x1409d4db0, "unknown_1409d4db0", fn_1409d4db0 as *const (), Throws::Caught), // sub_1409d4db0
    Replacement::draft_throws(0x1409d4e70, "unknown_1409d4e70", fn_1409d4e70 as *const (), Throws::Caught), // sub_1409d4e70
    Replacement::draft_throws(0x1409d4f30, "unknown_1409d4f30", fn_1409d4f30 as *const (), Throws::Caught), // sub_1409d4f30
    Replacement::draft(0x1409d4ff0, "unknown_1409d4ff0", fn_1409d4ff0 as *const ()), // sub_1409d4ff0
    Replacement::draft(0x1409d50b0, "unknown_1409d50b0", fn_1409d50b0 as *const ()), // sub_1409d50b0
    Replacement::draft(0x1409d5170, "unknown_1409d5170", fn_1409d5170 as *const ()), // sub_1409d5170
    Replacement::draft_throws(0x1409d5230, "unknown_1409d5230", fn_1409d5230 as *const (), Throws::Discard), // sub_1409d5230
    Replacement::draft_throws(0x1409d52e0, "unknown_1409d52e0", tpl_6941::<0x14387e560, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432a6588, 0x8, 0x1432a65a0, 0x140a00b30> as *const (), Throws::Discard), // sub_1409d52e0
    Replacement::draft(0x1409d53c0, "unknown_1409d53c0", tpl_70783::<0x8, 0x10, 0x1426cdcb0, 0x140a2c200> as *const ()), // sub_1409d53c0
    Replacement::draft(0x1409d5440, "unknown_1409d5440", tpl_70783::<0x8, 0x10, 0x1432a65a0, 0x140a2c210> as *const ()), // sub_1409d5440
    Replacement::draft(0x1409d54c0, "unknown_1409d54c0", tpl_12311::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x14328a820, 0x8> as *const ()), // sub_1409d54c0
    Replacement::draft(0x1409d5560, "unknown_1409d5560", tpl_12312::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328a820, 0xfffffffffffffff0, 0xfffffffffffffff8, 0xfffffffffffffffc, 0x18> as *const ()), // sub_1409d5560
    Replacement::draft(0x1409d5680, "unknown_1409d5680", tpl_11505::<0x14387e108, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d5680
    Replacement::draft_throws(0x1409d5720, "unknown_1409d5720", tpl_11506::<0x14387dd60, 0x14387e108, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x328, 0x338, 0x38, 0x4, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409d5720
    Replacement::draft(0x1409d57e0, "unknown_1409d57e0", tpl_11507::<0x14387e108, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d57e0
    Replacement::draft_throws(0x1409d5880, "unknown_1409d5880", tpl_11508::<0x14387dd60, 0x14387e108, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409d5880
    Replacement::draft_throws(0x1409d5920, "unknown_1409d5920", fn_1409d5920 as *const (), Throws::Caught), // sub_1409d5920
    Replacement::draft_throws(0x1409d5b20, "unknown_1409d5b20", fn_1409d5b20 as *const (), Throws::Caught), // sub_1409d5b20
    Replacement::draft_throws(0x1409d5cf0, "unknown_1409d5cf0", fn_1409d5cf0 as *const (), Throws::Caught), // sub_1409d5cf0
    Replacement::draft(0x1409d5ec0, "unknown_1409d5ec0", tpl_70013::<0x14387e108, 0x14243b0b0, 0x1406e01b0, 0x8, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d5ec0
    Replacement::draft(0x1409d5f40, "unknown_1409d5f40", tpl_11513::<0x14387dd60, 0x14387e108, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x1401e9860> as *const ()), // sub_1409d5f40
    Replacement::draft(0x1409d5fe0, "unknown_1409d5fe0", tpl_11507::<0x14387e108, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d5fe0
    Replacement::draft_throws(0x1409d6090, "unknown_1409d6090", tpl_70014::<0x14387dd60, 0x14387e108, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x10, 0x1401e9590> as *const (), Throws::Discard), // sub_1409d6090
    Replacement::draft_throws(0x1409d6110, "unknown_1409d6110", tpl_11515::<0x14387e108, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x4, 0x1432a6268, 0x1413c41a0> as *const (), Throws::Discard), // sub_1409d6110
    Replacement::draft(0x1409d6210, "unknown_1409d6210", tpl_12316::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14328d190, 0x8> as *const ()), // sub_1409d6210
    Replacement::draft(0x1409d6290, "unknown_1409d6290", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328d190> as *const ()), // sub_1409d6290
    Replacement::draft(0x1409d6390, "unknown_1409d6390", tpl_6896::<0x14387dde8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140a43ca0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d6390
    Replacement::draft_throws(0x1409d6400, "unknown_1409d6400", tpl_6897::<0x14387e1bc, 0x14387dde8, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1409fda10, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409d6400
    Replacement::draft(0x1409d6470, "unknown_1409d6470", tpl_6896::<0x14387dde8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140a441a0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d6470
    Replacement::draft_throws(0x1409d64e0, "unknown_1409d64e0", tpl_6897::<0x14387e1bc, 0x14387dde8, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1409fdb40, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409d64e0
    Replacement::draft_throws(0x1409d6550, "unknown_1409d6550", fn_1409d6550 as *const (), Throws::Caught), // sub_1409d6550
    Replacement::draft_throws(0x1409d6620, "unknown_1409d6620", fn_1409d6620 as *const (), Throws::Caught), // sub_1409d6620
    Replacement::draft_throws(0x1409d66f0, "unknown_1409d66f0", fn_1409d66f0 as *const (), Throws::Caught), // sub_1409d66f0
    Replacement::draft(0x1409d67c0, "unknown_1409d67c0", tpl_6896::<0x14387dde8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140a44010, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d67c0
    Replacement::draft(0x1409d6830, "unknown_1409d6830", tpl_6899::<0x14387e1bc, 0x14387dde8, 0x1401e9970, 0x1406e1480, 0x8, 0x140a46050, 0x1401e9860> as *const ()), // sub_1409d6830
    Replacement::draft(0x1409d6890, "unknown_1409d6890", tpl_6896::<0x14387dde8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140a461c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d6890
    Replacement::draft_throws(0x1409d6900, "unknown_1409d6900", tpl_6900::<0x14387e1bc, 0x14387dde8, 0x1401e9770, 0x1406e1570, 0x8, 0x140a46310, 0x1401e9590> as *const (), Throws::Discard), // sub_1409d6900
    Replacement::draft_throws(0x1409d6960, "unknown_1409d6960", tpl_11380::<0x14387dde8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432a6210, 0x1409fdcd0> as *const (), Throws::Discard), // sub_1409d6960
    Replacement::draft(0x1409d6a30, "unknown_1409d6a30", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143281740, 0x8> as *const ()), // sub_1409d6a30
    Replacement::draft(0x1409d6ac0, "unknown_1409d6ac0", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143281740, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409d6ac0
    Replacement::draft(0x1409d6be0, "unknown_1409d6be0", fn_1409d6be0 as *const ()), // sub_1409d6be0
    Replacement::draft_throws(0x1409d6cc0, "unknown_1409d6cc0", fn_1409d6cc0 as *const (), Throws::Discard), // sub_1409d6cc0
    Replacement::draft(0x1409d6d90, "unknown_1409d6d90", tpl_12319::<0x14387e180, 0x14243b0b0, 0x1406e01b0, 0x338, 0x48, 0x8, 0x14086b460, 0x140a441a0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d6d90
    Replacement::draft_throws(0x1409d6e50, "unknown_1409d6e50", fn_1409d6e50 as *const (), Throws::Discard), // sub_1409d6e50
    Replacement::draft(0x1409d6f20, "unknown_1409d6f20", tpl_12321::<0x8, 0x14086b460> as *const ()), // sub_1409d6f20
    Replacement::draft_throws(0x1409d6f80, "unknown_1409d6f80", fn_1409d6f80 as *const (), Throws::Caught), // sub_1409d6f80
    Replacement::draft_throws(0x1409d7040, "unknown_1409d7040", fn_1409d7040 as *const (), Throws::Caught), // sub_1409d7040
    Replacement::draft_throws(0x1409d7100, "unknown_1409d7100", fn_1409d7100 as *const (), Throws::Caught), // sub_1409d7100
    Replacement::draft(0x1409d71c0, "unknown_1409d71c0", tpl_12322::<0x14387e180, 0x14243b0b0, 0x1406e01b0, 0x338, 0x10, 0x8, 0x140a44010, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d71c0
    Replacement::draft(0x1409d7270, "unknown_1409d7270", tpl_70790::<0x14387e29c, 0x14387e180, 0x1401e9970, 0x1406e1480, 0x10, 0x338, 0xe8, 0x8, 0x140a46050, 0x1401e9860> as *const ()), // sub_1409d7270
    Replacement::draft(0x1409d7320, "unknown_1409d7320", tpl_70791::<0x14387e180, 0x14243b0b0, 0x1406e01b0, 0x10, 0x338, 0xe8, 0x8, 0x140a461c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409d7320
    Replacement::draft_throws(0x1409d73e0, "unknown_1409d73e0", tpl_70792::<0x14387e29c, 0x14387e180, 0x1401e9770, 0x1406e1570, 0x338, 0x10, 0x120, 0x8, 0x140a46310, 0x1401e9590> as *const (), Throws::Discard), // sub_1409d73e0
    Replacement::draft_throws(0x1409d7480, "unknown_1409d7480", tpl_11476::<0x14387e180, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432a6220, 0x8, 0x1432a62a0, 0x1432a62b0, 0x140a00bb0> as *const (), Throws::Discard), // sub_1409d7480
    Replacement::draft(0x1409d7570, "unknown_1409d7570", fn_1409d7570 as *const ()), // sub_1409d7570
    Replacement::draft(0x1409d7630, "unknown_1409d7630", fn_1409d7630 as *const ()), // sub_1409d7630
    Replacement::draft(0x1409d76a0, "unknown_1409d76a0", fn_1409d76a0 as *const ()), // sub_1409d76a0
    Replacement::draft(0x1409d7770, "unknown_1409d7770", fn_1409d7770 as *const ()), // sub_1409d7770
    Replacement::draft(0x1409d78a0, "unknown_1409d78a0", fn_1409d78a0 as *const ()), // sub_1409d78a0
    Replacement::draft_throws(0x1409d7a80, "unknown_1409d7a80", fn_1409d7a80 as *const (), Throws::Caught), // sub_1409d7a80
    Replacement::draft(0x1409d7d90, "unknown_1409d7d90", fn_1409d7d90 as *const ()), // sub_1409d7d90
    Replacement::draft_throws(0x1409d7ed0, "unknown_1409d7ed0", fn_1409d7ed0 as *const (), Throws::Caught), // sub_1409d7ed0
    Replacement::draft(0x1409d8230, "unknown_1409d8230", fn_1409d8230 as *const ()), // sub_1409d8230
    Replacement::draft_throws(0x1409d8300, "unknown_1409d8300", fn_1409d8300 as *const (), Throws::Caught), // sub_1409d8300
    Replacement::draft_throws(0x1409d8730, "unknown_1409d8730", fn_1409d8730 as *const (), Throws::Caught), // sub_1409d8730
    Replacement::draft_throws(0x1409d8af0, "unknown_1409d8af0", fn_1409d8af0 as *const (), Throws::Caught), // sub_1409d8af0
    Replacement::draft(0x1409d8eb0, "unknown_1409d8eb0", fn_1409d8eb0 as *const ()), // sub_1409d8eb0
    Replacement::draft(0x1409d8f60, "unknown_1409d8f60", fn_1409d8f60 as *const ()), // sub_1409d8f60
    Replacement::draft(0x1409d90a0, "unknown_1409d90a0", fn_1409d90a0 as *const ()), // sub_1409d90a0
    Replacement::draft_throws(0x1409d91f0, "unknown_1409d91f0", fn_1409d91f0 as *const (), Throws::Discard), // sub_1409d91f0
    Replacement::draft_throws(0x1409d9350, "unknown_1409d9350", fn_1409d9350 as *const (), Throws::Caught), // sub_1409d9350
    Replacement::draft_throws(0x1409d96d0, "unknown_1409d96d0", fn_1409d96d0 as *const (), Throws::Discard), // sub_1409d96d0
    Replacement::draft(0x1409d9770, "unknown_1409d9770", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143282248> as *const ()), // sub_1409d9770
    Replacement::draft(0x1409d97d0, "unknown_1409d97d0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143282248> as *const ()), // sub_1409d97d0
    Replacement::draft_throws(0x1409d98d0, "unknown_1409d98d0", fn_1409d98d0 as *const (), Throws::Caught), // sub_1409d98d0
    Replacement::draft_throws(0x1409d9960, "unknown_1409d9960", tpl_6762::<0x14387de60, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409d9960
    Replacement::draft(0x1409d9a10, "unknown_1409d9a10", fn_1409d9a10 as *const ()), // sub_1409d9a10
    Replacement::draft(0x1409d9ac0, "unknown_1409d9ac0", fn_1409d9ac0 as *const ()), // sub_1409d9ac0
    Replacement::draft(0x1409d9bf0, "unknown_1409d9bf0", fn_1409d9bf0 as *const ()), // sub_1409d9bf0
    Replacement::draft_throws(0x1409d9ce0, "unknown_1409d9ce0", fn_1409d9ce0 as *const (), Throws::Discard), // sub_1409d9ce0
    Replacement::draft(0x1409d9de0, "unknown_1409d9de0", fn_1409d9de0 as *const ()), // sub_1409d9de0
    Replacement::draft_throws(0x1409d9e90, "unknown_1409d9e90", fn_1409d9e90 as *const (), Throws::Discard), // sub_1409d9e90
    Replacement::draft_throws(0x1409d9f90, "unknown_1409d9f90", fn_1409d9f90 as *const (), Throws::Caught), // sub_1409d9f90
    Replacement::draft_throws(0x1409da340, "unknown_1409da340", fn_1409da340 as *const (), Throws::Caught), // sub_1409da340
    Replacement::draft_throws(0x1409da680, "unknown_1409da680", fn_1409da680 as *const (), Throws::Caught), // sub_1409da680
    Replacement::draft(0x1409da9a0, "unknown_1409da9a0", fn_1409da9a0 as *const ()), // sub_1409da9a0
    Replacement::draft(0x1409daa40, "unknown_1409daa40", fn_1409daa40 as *const ()), // sub_1409daa40
    Replacement::draft(0x1409daaf0, "unknown_1409daaf0", fn_1409daaf0 as *const ()), // sub_1409daaf0
    Replacement::draft_throws(0x1409dabb0, "unknown_1409dabb0", fn_1409dabb0 as *const (), Throws::Discard), // sub_1409dabb0
    Replacement::draft_throws(0x1409dac40, "unknown_1409dac40", fn_1409dac40 as *const (), Throws::Discard), // sub_1409dac40
    Replacement::draft(0x1409dad70, "unknown_1409dad70", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143288528, 0x8> as *const ()), // sub_1409dad70
    Replacement::draft(0x1409dadf0, "unknown_1409dadf0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143288528> as *const ()), // sub_1409dadf0
    Replacement::draft(0x1409daef0, "unknown_1409daef0", tpl_11667::<0x14387e530, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409daef0
    Replacement::draft_throws(0x1409daf90, "unknown_1409daf90", tpl_9063::<0x14387e1d4, 0x14387e530, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409daf90
    Replacement::draft(0x1409db020, "unknown_1409db020", tpl_11361::<0x14387e530, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409db020
    Replacement::draft_throws(0x1409db0a0, "unknown_1409db0a0", tpl_9065::<0x14387e1d4, 0x14387e530, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409db0a0
    Replacement::draft_throws(0x1409db130, "unknown_1409db130", fn_1409db130 as *const (), Throws::Caught), // sub_1409db130
    Replacement::draft_throws(0x1409db330, "unknown_1409db330", fn_1409db330 as *const (), Throws::Caught), // sub_1409db330
    Replacement::draft_throws(0x1409db500, "unknown_1409db500", fn_1409db500 as *const (), Throws::Caught), // sub_1409db500
    Replacement::draft(0x1409db6d0, "unknown_1409db6d0", fn_1409db6d0 as *const ()), // sub_1409db6d0
    Replacement::draft(0x1409db740, "unknown_1409db740", tpl_67834::<0x14387e1d4, 0x14387e530, 0x1401e9970, 0x1406e1480, 0x8, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_1409db740
    Replacement::draft(0x1409db7c0, "unknown_1409db7c0", tpl_11674::<0x14387e530, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409db7c0
    Replacement::draft_throws(0x1409db850, "unknown_1409db850", tpl_9073::<0x14387e1d4, 0x14387e530, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1409db850
    Replacement::draft_throws(0x1409db8c0, "unknown_1409db8c0", fn_1409db8c0 as *const (), Throws::Discard), // sub_1409db8c0
    Replacement::draft(0x1409db9a0, "unknown_1409db9a0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14329a128, 0x8> as *const ()), // sub_1409db9a0
    Replacement::draft(0x1409dba30, "unknown_1409dba30", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14329a128, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409dba30
    Replacement::draft(0x1409dbb50, "unknown_1409dbb50", fn_1409dbb50 as *const ()), // sub_1409dbb50
    Replacement::draft_throws(0x1409dbbc0, "unknown_1409dbbc0", tpl_6897::<0x14387e538, 0x14387de20, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140a00cf0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409dbbc0
    Replacement::draft(0x1409dbc30, "unknown_1409dbc30", fn_1409dbc30 as *const ()), // sub_1409dbc30
    Replacement::draft_throws(0x1409dbd00, "unknown_1409dbd00", tpl_6897::<0x14387e538, 0x14387de20, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140a00dc0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409dbd00
    Replacement::draft(0x1409dbd70, "unknown_1409dbd70", tpl_12118::<0x8, 0x140a03220, 0x14240aa00> as *const ()), // sub_1409dbd70
    Replacement::draft_throws(0x1409dbde0, "unknown_1409dbde0", fn_1409dbde0 as *const (), Throws::Caught), // sub_1409dbde0
    Replacement::draft_throws(0x1409dbea0, "unknown_1409dbea0", fn_1409dbea0 as *const (), Throws::Caught), // sub_1409dbea0
    Replacement::draft_throws(0x1409dbf60, "unknown_1409dbf60", fn_1409dbf60 as *const (), Throws::Caught), // sub_1409dbf60
    Replacement::draft(0x1409dc020, "unknown_1409dc020", fn_1409dc020 as *const ()), // sub_1409dc020
    Replacement::draft(0x1409dc100, "unknown_1409dc100", fn_1409dc100 as *const ()), // sub_1409dc100
    Replacement::draft(0x1409dc1c0, "unknown_1409dc1c0", fn_1409dc1c0 as *const ()), // sub_1409dc1c0
    Replacement::draft_throws(0x1409dc280, "unknown_1409dc280", fn_1409dc280 as *const (), Throws::Discard), // sub_1409dc280
    Replacement::draft_throws(0x1409dc330, "unknown_1409dc330", tpl_6941::<0x14387de20, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432a61d8, 0x8, 0x1432a61e4, 0x140a00ee0> as *const (), Throws::Discard), // sub_1409dc330
    Replacement::draft(0x1409dc410, "unknown_1409dc410", fn_1409dc410 as *const ()), // sub_1409dc410
    Replacement::draft(0x1409dc4b0, "unknown_1409dc4b0", fn_1409dc4b0 as *const ()), // sub_1409dc4b0
    Replacement::draft(0x1409dc5d0, "unknown_1409dc5d0", fn_1409dc5d0 as *const ()), // sub_1409dc5d0
    Replacement::draft_throws(0x1409dc670, "unknown_1409dc670", fn_1409dc670 as *const (), Throws::Discard), // sub_1409dc670
    Replacement::draft(0x1409dc7a0, "unknown_1409dc7a0", fn_1409dc7a0 as *const ()), // sub_1409dc7a0
    Replacement::draft_throws(0x1409dc920, "unknown_1409dc920", fn_1409dc920 as *const (), Throws::Discard), // sub_1409dc920
    Replacement::draft(0x1409dca10, "unknown_1409dca10", fn_1409dca10 as *const ()), // sub_1409dca10
    Replacement::draft_throws(0x1409dcb20, "unknown_1409dcb20", fn_1409dcb20 as *const (), Throws::Caught), // sub_1409dcb20
    Replacement::draft_throws(0x1409dcc60, "unknown_1409dcc60", fn_1409dcc60 as *const (), Throws::Caught), // sub_1409dcc60
    Replacement::draft_throws(0x1409dcda0, "unknown_1409dcda0", fn_1409dcda0 as *const (), Throws::Caught), // sub_1409dcda0
    Replacement::draft(0x1409dcee0, "unknown_1409dcee0", fn_1409dcee0 as *const ()), // sub_1409dcee0
    Replacement::draft(0x1409dcfd0, "unknown_1409dcfd0", fn_1409dcfd0 as *const ()), // sub_1409dcfd0
    Replacement::draft(0x1409dd110, "unknown_1409dd110", fn_1409dd110 as *const ()), // sub_1409dd110
    Replacement::draft_throws(0x1409dd260, "unknown_1409dd260", fn_1409dd260 as *const (), Throws::Discard), // sub_1409dd260
    Replacement::draft_throws(0x1409dd3b0, "unknown_1409dd3b0", fn_1409dd3b0 as *const (), Throws::Discard), // sub_1409dd3b0
    Replacement::draft(0x1409dd4e0, "unknown_1409dd4e0", fn_1409dd4e0 as *const ()), // sub_1409dd4e0
    Replacement::draft(0x1409dd580, "unknown_1409dd580", fn_1409dd580 as *const ()), // sub_1409dd580
    Replacement::draft(0x1409dd630, "unknown_1409dd630", fn_1409dd630 as *const ()), // sub_1409dd630
    Replacement::draft(0x1409dd770, "unknown_1409dd770", fn_1409dd770 as *const ()), // sub_1409dd770
    Replacement::draft_throws(0x1409dd850, "unknown_1409dd850", fn_1409dd850 as *const (), Throws::Discard), // sub_1409dd850
    Replacement::draft(0x1409dd930, "unknown_1409dd930", tpl_12375::<0x14387dda0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xc, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409dd930
    Replacement::draft_throws(0x1409dd9c0, "unknown_1409dd9c0", fn_1409dd9c0 as *const (), Throws::Discard), // sub_1409dd9c0
    Replacement::draft_throws(0x1409ddaa0, "unknown_1409ddaa0", fn_1409ddaa0 as *const (), Throws::Caught), // sub_1409ddaa0
    Replacement::draft_throws(0x1409ddd30, "unknown_1409ddd30", fn_1409ddd30 as *const (), Throws::Caught), // sub_1409ddd30
    Replacement::draft_throws(0x1409ddf80, "unknown_1409ddf80", fn_1409ddf80 as *const (), Throws::Caught), // sub_1409ddf80
    Replacement::draft(0x1409de1c0, "unknown_1409de1c0", fn_1409de1c0 as *const ()), // sub_1409de1c0
    Replacement::draft(0x1409de260, "unknown_1409de260", tpl_70858::<0x14387e098, 0x14387dda0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0xc, 0xc0, 0x1401e9860> as *const ()), // sub_1409de260
    Replacement::draft(0x1409de2e0, "unknown_1409de2e0", tpl_12375::<0x14387dda0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xc, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409de2e0
    Replacement::draft_throws(0x1409de370, "unknown_1409de370", fn_1409de370 as *const (), Throws::Discard), // sub_1409de370
    Replacement::draft_throws(0x1409de3f0, "unknown_1409de3f0", fn_1409de3f0 as *const (), Throws::Discard), // sub_1409de3f0
    Replacement::draft(0x1409de510, "unknown_1409de510", fn_1409de510 as *const ()), // sub_1409de510
    Replacement::draft(0x1409de5b0, "unknown_1409de5b0", fn_1409de5b0 as *const ()), // sub_1409de5b0
    Replacement::draft(0x1409de6e0, "unknown_1409de6e0", fn_1409de6e0 as *const ()), // sub_1409de6e0
    Replacement::draft_throws(0x1409de790, "unknown_1409de790", tpl_12387::<0x14387de6c, 0x14387dfd8, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409de790
    Replacement::draft(0x1409de840, "unknown_1409de840", tpl_11361::<0x14387dfd8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409de840
    Replacement::draft_throws(0x1409de8c0, "unknown_1409de8c0", tpl_12388::<0x14387de6c, 0x14387dfd8, 0x1401e9b60, 0x1406e03a0, 0x328, 0x8, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409de8c0
    Replacement::draft_throws(0x1409de970, "unknown_1409de970", fn_1409de970 as *const (), Throws::Caught), // sub_1409de970
    Replacement::draft_throws(0x1409debc0, "unknown_1409debc0", fn_1409debc0 as *const (), Throws::Caught), // sub_1409debc0
    Replacement::draft_throws(0x1409dede0, "unknown_1409dede0", fn_1409dede0 as *const (), Throws::Caught), // sub_1409dede0
    Replacement::draft(0x1409deff0, "unknown_1409deff0", fn_1409deff0 as *const ()), // sub_1409deff0
    Replacement::draft(0x1409df090, "unknown_1409df090", tpl_9203::<0x14387de6c, 0x14387dfd8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_1409df090
    Replacement::draft(0x1409df100, "unknown_1409df100", tpl_11361::<0x14387dfd8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409df100
    Replacement::draft_throws(0x1409df180, "unknown_1409df180", tpl_12393::<0x14387de6c, 0x14387dfd8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_1409df180
    Replacement::draft_throws(0x1409df1f0, "unknown_1409df1f0", fn_1409df1f0 as *const (), Throws::Discard), // sub_1409df1f0
    Replacement::draft(0x1409df2d0, "unknown_1409df2d0", fn_1409df2d0 as *const ()), // sub_1409df2d0
    Replacement::draft(0x1409df3b0, "unknown_1409df3b0", fn_1409df3b0 as *const ()), // sub_1409df3b0
    Replacement::draft(0x1409df4a0, "unknown_1409df4a0", fn_1409df4a0 as *const ()), // sub_1409df4a0
    Replacement::draft(0x1409df540, "unknown_1409df540", fn_1409df540 as *const ()), // sub_1409df540
    Replacement::draft_throws(0x1409df640, "unknown_1409df640", fn_1409df640 as *const (), Throws::Discard), // sub_1409df640
    Replacement::draft(0x1409df730, "unknown_1409df730", fn_1409df730 as *const ()), // sub_1409df730
    Replacement::draft_throws(0x1409df8a0, "unknown_1409df8a0", fn_1409df8a0 as *const (), Throws::Discard), // sub_1409df8a0
    Replacement::draft(0x1409df9a0, "unknown_1409df9a0", fn_1409df9a0 as *const ()), // sub_1409df9a0
    Replacement::draft_throws(0x1409dfab0, "unknown_1409dfab0", fn_1409dfab0 as *const (), Throws::Caught), // sub_1409dfab0
    Replacement::draft_throws(0x1409dfc30, "unknown_1409dfc30", fn_1409dfc30 as *const (), Throws::Caught), // sub_1409dfc30
    Replacement::draft_throws(0x1409dfdb0, "unknown_1409dfdb0", fn_1409dfdb0 as *const (), Throws::Caught), // sub_1409dfdb0
    Replacement::draft(0x1409dff30, "unknown_1409dff30", fn_1409dff30 as *const ()), // sub_1409dff30
    Replacement::draft(0x1409e00a0, "unknown_1409e00a0", fn_1409e00a0 as *const ()), // sub_1409e00a0
    Replacement::draft(0x1409e01e0, "unknown_1409e01e0", fn_1409e01e0 as *const ()), // sub_1409e01e0
    Replacement::draft_throws(0x1409e0340, "unknown_1409e0340", fn_1409e0340 as *const (), Throws::Discard), // sub_1409e0340
    Replacement::draft_throws(0x1409e0480, "unknown_1409e0480", fn_1409e0480 as *const (), Throws::Caught), // sub_1409e0480
    Replacement::draft(0x1409e0660, "unknown_1409e0660", fn_1409e0660 as *const ()), // sub_1409e0660
    Replacement::draft(0x1409e0730, "unknown_1409e0730", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432861e8> as *const ()), // sub_1409e0730
    Replacement::draft(0x1409e0790, "unknown_1409e0790", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432861e8> as *const ()), // sub_1409e0790
    Replacement::draft_throws(0x1409e0890, "unknown_1409e0890", fn_1409e0890 as *const (), Throws::Caught), // sub_1409e0890
    Replacement::draft_throws(0x1409e0920, "unknown_1409e0920", tpl_6762::<0x14387e0c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409e0920
    Replacement::draft(0x1409e09d0, "unknown_1409e09d0", fn_1409e09d0 as *const ()), // sub_1409e09d0
    Replacement::draft(0x1409e0a60, "unknown_1409e0a60", fn_1409e0a60 as *const ()), // sub_1409e0a60
    Replacement::draft(0x1409e0b70, "unknown_1409e0b70", tpl_12411::<0x14387dea8, 0x14243b0b0, 0x1406e01b0, 0xb, 0x10, 0xc, 0x8, 0x338, 0x1401ee7c0, 0x20, 0x1426d4580, 0x1> as *const ()), // sub_1409e0b70
    Replacement::draft_throws(0x1409e0c60, "unknown_1409e0c60", fn_1409e0c60 as *const (), Throws::Discard), // sub_1409e0c60
    Replacement::draft(0x1409e0d90, "unknown_1409e0d90", fn_1409e0d90 as *const ()), // sub_1409e0d90
    Replacement::draft_throws(0x1409e0e50, "unknown_1409e0e50", fn_1409e0e50 as *const (), Throws::Discard), // sub_1409e0e50
    Replacement::draft_throws(0x1409e0f60, "unknown_1409e0f60", fn_1409e0f60 as *const (), Throws::Caught), // sub_1409e0f60
    Replacement::draft_throws(0x1409e11c0, "unknown_1409e11c0", fn_1409e11c0 as *const (), Throws::Caught), // sub_1409e11c0
    Replacement::draft_throws(0x1409e13f0, "unknown_1409e13f0", fn_1409e13f0 as *const (), Throws::Caught), // sub_1409e13f0
    Replacement::draft(0x1409e1610, "unknown_1409e1610", fn_1409e1610 as *const ()), // sub_1409e1610
    Replacement::draft(0x1409e16c0, "unknown_1409e16c0", fn_1409e16c0 as *const ()), // sub_1409e16c0
    Replacement::draft(0x1409e1780, "unknown_1409e1780", fn_1409e1780 as *const ()), // sub_1409e1780
    Replacement::draft_throws(0x1409e1850, "unknown_1409e1850", tpl_70894::<0x14387e260, 0x14387dea8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x20, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1409e1850
    Replacement::draft_throws(0x1409e18d0, "unknown_1409e18d0", fn_1409e18d0 as *const (), Throws::Discard), // sub_1409e18d0
    Replacement::draft(0x1409e19f0, "unknown_1409e19f0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143299a08> as *const ()), // sub_1409e19f0
    Replacement::draft(0x1409e1a50, "unknown_1409e1a50", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143299a08> as *const ()), // sub_1409e1a50
    Replacement::draft_throws(0x1409e1b50, "unknown_1409e1b50", fn_1409e1b50 as *const (), Throws::Caught), // sub_1409e1b50
    Replacement::draft_throws(0x1409e1be0, "unknown_1409e1be0", tpl_6762::<0x14387e4b8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409e1be0
    Replacement::draft(0x1409e1c90, "unknown_1409e1c90", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143281898, 0x8> as *const ()), // sub_1409e1c90
    Replacement::draft(0x1409e1d10, "unknown_1409e1d10", fn_1409e1d10 as *const ()), // sub_1409e1d10
    Replacement::draft(0x1409e1e20, "unknown_1409e1e20", tpl_11781::<0x14387e490, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409e1e20
    Replacement::draft_throws(0x1409e1eb0, "unknown_1409e1eb0", tpl_11782::<0x14387de70, 0x14387e490, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409e1eb0
    Replacement::draft(0x1409e1f30, "unknown_1409e1f30", tpl_11361::<0x14387e490, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409e1f30
    Replacement::draft_throws(0x1409e1fb0, "unknown_1409e1fb0", tpl_11783::<0x14387de70, 0x14387e490, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409e1fb0
    Replacement::draft_throws(0x1409e2040, "unknown_1409e2040", fn_1409e2040 as *const (), Throws::Caught), // sub_1409e2040
    Replacement::draft_throws(0x1409e2110, "unknown_1409e2110", fn_1409e2110 as *const (), Throws::Caught), // sub_1409e2110
    Replacement::draft_throws(0x1409e21e0, "unknown_1409e21e0", fn_1409e21e0 as *const (), Throws::Caught), // sub_1409e21e0
    Replacement::draft(0x1409e22b0, "unknown_1409e22b0", fn_1409e22b0 as *const ()), // sub_1409e22b0
    Replacement::draft(0x1409e2330, "unknown_1409e2330", tpl_9203::<0x14387de70, 0x14387e490, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_1409e2330
    Replacement::draft(0x1409e23a0, "unknown_1409e23a0", tpl_11361::<0x14387e490, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409e23a0
    Replacement::draft_throws(0x1409e2420, "unknown_1409e2420", tpl_11786::<0x14387de70, 0x14387e490, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_1409e2420
    Replacement::draft_throws(0x1409e2490, "unknown_1409e2490", fn_1409e2490 as *const (), Throws::Discard), // sub_1409e2490
    Replacement::draft(0x1409e2580, "unknown_1409e2580", fn_1409e2580 as *const ()), // sub_1409e2580
    Replacement::draft(0x1409e2640, "unknown_1409e2640", tpl_5937::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140a01340> as *const ()), // sub_1409e2640
    Replacement::draft(0x1409e2740, "unknown_1409e2740", tpl_6868::<0x14387e048, 0x14243b0b0, 0x1438820c0, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_1409e2740
    Replacement::draft(0x1409e27d0, "unknown_1409e27d0", tpl_6869::<0x14387e048, 0x14243b0b0, 0x1406fe630, 0x338, 0x18, 0x1426d4580, 0x1, 0x24, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409e27d0
    Replacement::draft_throws(0x1409e28b0, "unknown_1409e28b0", tpl_65919::<0x14387e220, 0x14387e048, 0x1401e9fa0, 0x1406db430, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409e28b0
    Replacement::draft(0x1409e29d0, "unknown_1409e29d0", tpl_6870::<0x14387e048, 0x14243b0b0, 0x1406fe890, 0x338, 0x18, 0x38, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x8> as *const ()), // sub_1409e29d0
    Replacement::draft_throws(0x1409e2ab0, "unknown_1409e2ab0", tpl_6871::<0x14387e220, 0x14387e048, 0x1401e9b60, 0x1406db670, 0x328, 0x338, 0x120, 0x18, 0x8, 0x20, 0x369, 0x24, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409e2ab0
    Replacement::draft_throws(0x1409e2ba0, "unknown_1409e2ba0", fn_1409e2ba0 as *const (), Throws::Caught), // sub_1409e2ba0
    Replacement::draft_throws(0x1409e2f00, "unknown_1409e2f00", fn_1409e2f00 as *const (), Throws::Caught), // sub_1409e2f00
    Replacement::draft_throws(0x1409e3210, "unknown_1409e3210", fn_1409e3210 as *const (), Throws::Caught), // sub_1409e3210
    Replacement::draft(0x1409e3500, "unknown_1409e3500", tpl_6875::<0x14387e048, 0x14243b0b0, 0x1406fe800, 0x18, 0x20, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409e3500
    Replacement::draft(0x1409e3580, "unknown_1409e3580", tpl_6876::<0x14387e220, 0x14387e048, 0x1401e9970, 0x1406fea30, 0x18, 0xd8, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x1401e9860> as *const ()), // sub_1409e3580
    Replacement::draft(0x1409e3660, "unknown_1409e3660", tpl_6870::<0x14387e048, 0x14243b0b0, 0x1406feb00, 0x338, 0x18, 0xd8, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x8> as *const ()), // sub_1409e3660
    Replacement::draft_throws(0x1409e3750, "unknown_1409e3750", tpl_6877::<0x14387e220, 0x14387e048, 0x1401e9770, 0x1406febe0, 0x338, 0x18, 0x120, 0x20, 0x8, 0x24, 0x2c, 0x1401e9590> as *const (), Throws::Discard), // sub_1409e3750
    Replacement::draft_throws(0x1409e37f0, "unknown_1409e37f0", fn_1409e37f0 as *const (), Throws::Caught), // sub_1409e37f0
    Replacement::draft_throws(0x1409e3960, "unknown_1409e3960", tpl_6879::<0x18, 0x1432a5df8, 0x14310c398, 0x1402c4e80, 0x1432a5e08, 0x24, 0x143109418> as *const (), Throws::Discard), // sub_1409e3960
    Replacement::draft(0x1409e39e0, "unknown_1409e39e0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328d600, 0x8> as *const ()), // sub_1409e39e0
    Replacement::draft(0x1409e3a70, "unknown_1409e3a70", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328d600, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409e3a70
    Replacement::draft(0x1409e3b20, "unknown_1409e3b20", fn_1409e3b20 as *const ()), // sub_1409e3b20
    Replacement::draft(0x1409e3bc0, "unknown_1409e3bc0", fn_1409e3bc0 as *const ()), // sub_1409e3bc0
    Replacement::draft_throws(0x1409e3cb0, "unknown_1409e3cb0", tpl_12433::<0x14387e580, 0x14387df90, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x48, 0x8, 0x140a03340, 0x140a2ed80, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409e3cb0
    Replacement::draft(0x1409e3d60, "unknown_1409e3d60", fn_1409e3d60 as *const ()), // sub_1409e3d60
    Replacement::draft_throws(0x1409e3e20, "unknown_1409e3e20", tpl_12435::<0x14387e580, 0x14387df90, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x140a03340, 0x140a2f1a0, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_1409e3e20
    Replacement::draft(0x1409e3ee0, "unknown_1409e3ee0", fn_1409e3ee0 as *const ()), // sub_1409e3ee0
    Replacement::draft_throws(0x1409e3f70, "unknown_1409e3f70", fn_1409e3f70 as *const (), Throws::Caught), // sub_1409e3f70
    Replacement::draft_throws(0x1409e4030, "unknown_1409e4030", fn_1409e4030 as *const (), Throws::Caught), // sub_1409e4030
    Replacement::draft_throws(0x1409e40f0, "unknown_1409e40f0", fn_1409e40f0 as *const (), Throws::Caught), // sub_1409e40f0
    Replacement::draft(0x1409e41b0, "unknown_1409e41b0", fn_1409e41b0 as *const ()), // sub_1409e41b0
    Replacement::draft(0x1409e4260, "unknown_1409e4260", fn_1409e4260 as *const ()), // sub_1409e4260
    Replacement::draft(0x1409e4310, "unknown_1409e4310", fn_1409e4310 as *const ()), // sub_1409e4310
    Replacement::draft_throws(0x1409e43d0, "unknown_1409e43d0", fn_1409e43d0 as *const (), Throws::Discard), // sub_1409e43d0
    Replacement::draft_throws(0x1409e4470, "unknown_1409e4470", tpl_6941::<0x14387df90, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432a5de0, 0x8, 0x1432a5df0, 0x140a013e0> as *const (), Throws::Discard), // sub_1409e4470
    Replacement::draft(0x1409e4550, "unknown_1409e4550", fn_1409e4550 as *const ()), // sub_1409e4550
    Replacement::draft(0x1409e45e0, "unknown_1409e45e0", fn_1409e45e0 as *const ()), // sub_1409e45e0
    Replacement::draft(0x1409e4700, "unknown_1409e4700", fn_1409e4700 as *const ()), // sub_1409e4700
    Replacement::draft_throws(0x1409e47d0, "unknown_1409e47d0", fn_1409e47d0 as *const (), Throws::Discard), // sub_1409e47d0
    Replacement::draft(0x1409e48a0, "unknown_1409e48a0", fn_1409e48a0 as *const ()), // sub_1409e48a0
    Replacement::draft_throws(0x1409e49a0, "unknown_1409e49a0", fn_1409e49a0 as *const (), Throws::Discard), // sub_1409e49a0
    Replacement::draft(0x1409e4a80, "unknown_1409e4a80", fn_1409e4a80 as *const ()), // sub_1409e4a80
    Replacement::draft_throws(0x1409e4b10, "unknown_1409e4b10", fn_1409e4b10 as *const (), Throws::Caught), // sub_1409e4b10
    Replacement::draft_throws(0x1409e4c50, "unknown_1409e4c50", fn_1409e4c50 as *const (), Throws::Caught), // sub_1409e4c50
    Replacement::draft_throws(0x1409e4d90, "unknown_1409e4d90", fn_1409e4d90 as *const (), Throws::Caught), // sub_1409e4d90
    Replacement::draft(0x1409e4ed0, "unknown_1409e4ed0", fn_1409e4ed0 as *const ()), // sub_1409e4ed0
    Replacement::draft(0x1409e5010, "unknown_1409e5010", fn_1409e5010 as *const ()), // sub_1409e5010
    Replacement::draft(0x1409e50f0, "unknown_1409e50f0", fn_1409e50f0 as *const ()), // sub_1409e50f0
    Replacement::draft_throws(0x1409e51e0, "unknown_1409e51e0", fn_1409e51e0 as *const (), Throws::Discard), // sub_1409e51e0
    Replacement::draft_throws(0x1409e52d0, "unknown_1409e52d0", fn_1409e52d0 as *const (), Throws::Discard), // sub_1409e52d0
    Replacement::draft(0x1409e5440, "unknown_1409e5440", fn_1409e5440 as *const ()), // sub_1409e5440
    Replacement::draft(0x1409e54b0, "unknown_1409e54b0", fn_1409e54b0 as *const ()), // sub_1409e54b0
    Replacement::draft(0x1409e5580, "unknown_1409e5580", fn_1409e5580 as *const ()), // sub_1409e5580
    Replacement::draft(0x1409e56e0, "unknown_1409e56e0", fn_1409e56e0 as *const ()), // sub_1409e56e0
    Replacement::draft_throws(0x1409e58c0, "unknown_1409e58c0", fn_1409e58c0 as *const (), Throws::Caught), // sub_1409e58c0
    Replacement::draft(0x1409e5b70, "unknown_1409e5b70", fn_1409e5b70 as *const ()), // sub_1409e5b70
    Replacement::draft_throws(0x1409e5c50, "unknown_1409e5c50", fn_1409e5c50 as *const (), Throws::Caught), // sub_1409e5c50
    Replacement::draft(0x1409e5f20, "unknown_1409e5f20", fn_1409e5f20 as *const ()), // sub_1409e5f20
    Replacement::draft_throws(0x1409e5fc0, "unknown_1409e5fc0", fn_1409e5fc0 as *const (), Throws::Caught), // sub_1409e5fc0
    Replacement::draft_throws(0x1409e65c0, "unknown_1409e65c0", fn_1409e65c0 as *const (), Throws::Caught), // sub_1409e65c0
    Replacement::draft_throws(0x1409e6b00, "unknown_1409e6b00", fn_1409e6b00 as *const (), Throws::Caught), // sub_1409e6b00
    Replacement::draft(0x1409e7020, "unknown_1409e7020", fn_1409e7020 as *const ()), // sub_1409e7020
    Replacement::draft(0x1409e70f0, "unknown_1409e70f0", fn_1409e70f0 as *const ()), // sub_1409e70f0
    Replacement::draft(0x1409e71e0, "unknown_1409e71e0", fn_1409e71e0 as *const ()), // sub_1409e71e0
    Replacement::draft_throws(0x1409e72e0, "unknown_1409e72e0", fn_1409e72e0 as *const (), Throws::Discard), // sub_1409e72e0
    Replacement::draft_throws(0x1409e73d0, "unknown_1409e73d0", fn_1409e73d0 as *const (), Throws::Caught), // sub_1409e73d0
    Replacement::draft(0x1409e7670, "unknown_1409e7670", fn_1409e7670 as *const ()), // sub_1409e7670
    Replacement::draft(0x1409e7750, "unknown_1409e7750", fn_1409e7750 as *const ()), // sub_1409e7750
    Replacement::draft(0x1409e77c0, "unknown_1409e77c0", fn_1409e77c0 as *const ()), // sub_1409e77c0
    Replacement::draft(0x1409e7860, "unknown_1409e7860", fn_1409e7860 as *const ()), // sub_1409e7860
    Replacement::draft(0x1409e7980, "unknown_1409e7980", fn_1409e7980 as *const ()), // sub_1409e7980
    Replacement::draft_throws(0x1409e7ab0, "unknown_1409e7ab0", fn_1409e7ab0 as *const (), Throws::Discard), // sub_1409e7ab0
    Replacement::draft(0x1409e7be0, "unknown_1409e7be0", tpl_12468::<0x14387dea0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x80> as *const ()), // sub_1409e7be0
    Replacement::draft_throws(0x1409e7cc0, "unknown_1409e7cc0", fn_1409e7cc0 as *const (), Throws::Discard), // sub_1409e7cc0
    Replacement::draft_throws(0x1409e7df0, "unknown_1409e7df0", fn_1409e7df0 as *const (), Throws::Caught), // sub_1409e7df0
    Replacement::draft_throws(0x1409e80d0, "unknown_1409e80d0", fn_1409e80d0 as *const (), Throws::Caught), // sub_1409e80d0
    Replacement::draft_throws(0x1409e8380, "unknown_1409e8380", fn_1409e8380 as *const (), Throws::Caught), // sub_1409e8380
    Replacement::draft(0x1409e8620, "unknown_1409e8620", fn_1409e8620 as *const ()), // sub_1409e8620
    Replacement::draft(0x1409e86a0, "unknown_1409e86a0", fn_1409e86a0 as *const ()), // sub_1409e86a0
    Replacement::draft(0x1409e8780, "unknown_1409e8780", tpl_12468::<0x14387dea0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x120> as *const ()), // sub_1409e8780
    Replacement::draft_throws(0x1409e8870, "unknown_1409e8870", fn_1409e8870 as *const (), Throws::Discard), // sub_1409e8870
    Replacement::draft_throws(0x1409e8940, "unknown_1409e8940", fn_1409e8940 as *const (), Throws::Discard), // sub_1409e8940
    Replacement::draft_throws(0x1409e8b00, "unknown_1409e8b00", fn_1409e8b00 as *const (), Throws::Discard), // sub_1409e8b00
    Replacement::draft(0x1409e8bb0, "unknown_1409e8bb0", fn_1409e8bb0 as *const ()), // sub_1409e8bb0
    Replacement::draft(0x1409e8c60, "unknown_1409e8c60", fn_1409e8c60 as *const ()), // sub_1409e8c60
    Replacement::draft(0x1409e8d90, "unknown_1409e8d90", fn_1409e8d90 as *const ()), // sub_1409e8d90
    Replacement::draft_throws(0x1409e8fc0, "unknown_1409e8fc0", fn_1409e8fc0 as *const (), Throws::Discard), // sub_1409e8fc0
    Replacement::draft(0x1409e9170, "unknown_1409e9170", fn_1409e9170 as *const ()), // sub_1409e9170
    Replacement::draft_throws(0x1409e9270, "unknown_1409e9270", fn_1409e9270 as *const (), Throws::Discard), // sub_1409e9270
    Replacement::draft_throws(0x1409e9490, "unknown_1409e9490", fn_1409e9490 as *const (), Throws::Caught), // sub_1409e9490
    Replacement::draft_throws(0x1409e9760, "unknown_1409e9760", fn_1409e9760 as *const (), Throws::Caught), // sub_1409e9760
    Replacement::draft_throws(0x1409e9a30, "unknown_1409e9a30", fn_1409e9a30 as *const (), Throws::Caught), // sub_1409e9a30
    Replacement::draft(0x1409e9d00, "unknown_1409e9d00", fn_1409e9d00 as *const ()), // sub_1409e9d00
    Replacement::draft(0x1409e9d80, "unknown_1409e9d80", fn_1409e9d80 as *const ()), // sub_1409e9d80
    Replacement::draft(0x1409e9e90, "unknown_1409e9e90", fn_1409e9e90 as *const ()), // sub_1409e9e90
    Replacement::draft_throws(0x1409e9fb0, "unknown_1409e9fb0", fn_1409e9fb0 as *const (), Throws::Discard), // sub_1409e9fb0
    Replacement::draft_throws(0x1409ea110, "unknown_1409ea110", fn_1409ea110 as *const (), Throws::Discard), // sub_1409ea110
    Replacement::draft(0x1409ea3c0, "unknown_1409ea3c0", fn_1409ea3c0 as *const ()), // sub_1409ea3c0
    Replacement::draft(0x1409ea530, "unknown_1409ea530", fn_1409ea530 as *const ()), // sub_1409ea530
    Replacement::draft(0x1409ea610, "unknown_1409ea610", fn_1409ea610 as *const ()), // sub_1409ea610
    Replacement::draft(0x1409ea760, "unknown_1409ea760", fn_1409ea760 as *const ()), // sub_1409ea760
    Replacement::draft_throws(0x1409eaab0, "unknown_1409eaab0", fn_1409eaab0 as *const (), Throws::Caught), // sub_1409eaab0
    Replacement::draft(0x1409eaee0, "unknown_1409eaee0", fn_1409eaee0 as *const ()), // sub_1409eaee0
    Replacement::draft_throws(0x1409eb1c0, "unknown_1409eb1c0", fn_1409eb1c0 as *const (), Throws::Discard), // sub_1409eb1c0
    Replacement::draft(0x1409eb620, "unknown_1409eb620", fn_1409eb620 as *const ()), // sub_1409eb620
    Replacement::draft_throws(0x1409eb850, "unknown_1409eb850", fn_1409eb850 as *const (), Throws::Caught), // sub_1409eb850
    Replacement::draft_throws(0x1409ec220, "unknown_1409ec220", fn_1409ec220 as *const (), Throws::Caught), // sub_1409ec220
    Replacement::draft_throws(0x1409ecae0, "unknown_1409ecae0", fn_1409ecae0 as *const (), Throws::Caught), // sub_1409ecae0
    Replacement::draft(0x1409ed380, "unknown_1409ed380", fn_1409ed380 as *const ()), // sub_1409ed380
    Replacement::draft(0x1409ed5d0, "unknown_1409ed5d0", fn_1409ed5d0 as *const ()), // sub_1409ed5d0
    Replacement::draft(0x1409ed880, "unknown_1409ed880", fn_1409ed880 as *const ()), // sub_1409ed880
    Replacement::draft_throws(0x1409edb50, "unknown_1409edb50", fn_1409edb50 as *const (), Throws::Discard), // sub_1409edb50
    Replacement::draft_throws(0x1409ede70, "unknown_1409ede70", fn_1409ede70 as *const (), Throws::Discard), // sub_1409ede70
    Replacement::draft(0x1409ee390, "unknown_1409ee390", fn_1409ee390 as *const ()), // sub_1409ee390
    Replacement::draft_throws(0x1409ee4b0, "unknown_1409ee4b0", fn_1409ee4b0 as *const (), Throws::Discard), // sub_1409ee4b0
    Replacement::draft(0x1409ee5c0, "unknown_1409ee5c0", fn_1409ee5c0 as *const ()), // sub_1409ee5c0
    Replacement::draft(0x1409ee6c0, "unknown_1409ee6c0", fn_1409ee6c0 as *const ()), // sub_1409ee6c0
    Replacement::draft(0x1409ee770, "unknown_1409ee770", fn_1409ee770 as *const ()), // sub_1409ee770
    Replacement::draft(0x1409ee8b0, "unknown_1409ee8b0", fn_1409ee8b0 as *const ()), // sub_1409ee8b0
    Replacement::draft_throws(0x1409ee9f0, "unknown_1409ee9f0", fn_1409ee9f0 as *const (), Throws::Discard), // sub_1409ee9f0
    Replacement::draft(0x1409eeb10, "unknown_1409eeb10", tpl_12511::<0x14387e030, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x14, 0x80, 0x18, 0x140915830, 0x140914170> as *const ()), // sub_1409eeb10
    Replacement::draft_throws(0x1409eebd0, "unknown_1409eebd0", fn_1409eebd0 as *const (), Throws::Discard), // sub_1409eebd0
    Replacement::draft_throws(0x1409eed10, "unknown_1409eed10", fn_1409eed10 as *const (), Throws::Caught), // sub_1409eed10
    Replacement::draft_throws(0x1409eef40, "unknown_1409eef40", fn_1409eef40 as *const (), Throws::Caught), // sub_1409eef40
    Replacement::draft_throws(0x1409ef170, "unknown_1409ef170", fn_1409ef170 as *const (), Throws::Caught), // sub_1409ef170
    Replacement::draft(0x1409ef3a0, "unknown_1409ef3a0", fn_1409ef3a0 as *const ()), // sub_1409ef3a0
    Replacement::draft(0x1409ef460, "unknown_1409ef460", fn_1409ef460 as *const ()), // sub_1409ef460
    Replacement::draft(0x1409ef520, "unknown_1409ef520", tpl_12511::<0x14387e030, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14, 0x120, 0x18, 0x1409163e0, 0x140914d20> as *const ()), // sub_1409ef520
    Replacement::draft_throws(0x1409ef5f0, "unknown_1409ef5f0", fn_1409ef5f0 as *const (), Throws::Discard), // sub_1409ef5f0
    Replacement::draft_throws(0x1409ef6d0, "unknown_1409ef6d0", fn_1409ef6d0 as *const (), Throws::Discard), // sub_1409ef6d0
    Replacement::draft(0x1409ef8c0, "unknown_1409ef8c0", fn_1409ef8c0 as *const ()), // sub_1409ef8c0
    Replacement::draft(0x1409ef9e0, "unknown_1409ef9e0", fn_1409ef9e0 as *const ()), // sub_1409ef9e0
    Replacement::draft(0x1409efa90, "unknown_1409efa90", tpl_12520::<0x1432a6d48, 0x18, 0x1409fd860, 0x20, 0x1432a6d58, 0x1409fd850> as *const ()), // sub_1409efa90
    Replacement::draft(0x1409efb00, "unknown_1409efb00", fn_1409efb00 as *const ()), // sub_1409efb00
    Replacement::draft(0x1409efbe0, "unknown_1409efbe0", fn_1409efbe0 as *const ()), // sub_1409efbe0
    Replacement::draft(0x1409efd30, "unknown_1409efd30", fn_1409efd30 as *const ()), // sub_1409efd30
    Replacement::draft_throws(0x1409efeb0, "unknown_1409efeb0", fn_1409efeb0 as *const (), Throws::Discard), // sub_1409efeb0
    Replacement::draft(0x1409f0050, "unknown_1409f0050", fn_1409f0050 as *const ()), // sub_1409f0050
    Replacement::draft_throws(0x1409f0180, "unknown_1409f0180", fn_1409f0180 as *const (), Throws::Discard), // sub_1409f0180
    Replacement::draft_throws(0x1409f0300, "unknown_1409f0300", fn_1409f0300 as *const (), Throws::Caught), // sub_1409f0300
    Replacement::draft_throws(0x1409f0930, "unknown_1409f0930", fn_1409f0930 as *const (), Throws::Caught), // sub_1409f0930
    Replacement::draft_throws(0x1409f0e70, "unknown_1409f0e70", fn_1409f0e70 as *const (), Throws::Caught), // sub_1409f0e70
    Replacement::draft(0x1409f1380, "unknown_1409f1380", fn_1409f1380 as *const ()), // sub_1409f1380
    Replacement::draft(0x1409f1410, "unknown_1409f1410", fn_1409f1410 as *const ()), // sub_1409f1410
    Replacement::draft(0x1409f1560, "unknown_1409f1560", fn_1409f1560 as *const ()), // sub_1409f1560
    Replacement::draft_throws(0x1409f16c0, "unknown_1409f16c0", fn_1409f16c0 as *const (), Throws::Discard), // sub_1409f16c0
    Replacement::draft_throws(0x1409f17c0, "unknown_1409f17c0", fn_1409f17c0 as *const (), Throws::Discard), // sub_1409f17c0
    Replacement::draft_throws(0x1409f19a0, "unknown_1409f19a0", fn_1409f19a0 as *const (), Throws::Discard), // sub_1409f19a0
    Replacement::draft(0x1409f1a60, "unknown_1409f1a60", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432879b8> as *const ()), // sub_1409f1a60
    Replacement::draft(0x1409f1ac0, "unknown_1409f1ac0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432879b8> as *const ()), // sub_1409f1ac0
    Replacement::draft_throws(0x1409f1bc0, "unknown_1409f1bc0", fn_1409f1bc0 as *const (), Throws::Caught), // sub_1409f1bc0
    Replacement::draft_throws(0x1409f1c50, "unknown_1409f1c50", tpl_6762::<0x14387e540, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409f1c50
    Replacement::draft(0x1409f1d00, "unknown_1409f1d00", fn_1409f1d00 as *const ()), // sub_1409f1d00
    Replacement::draft(0x1409f1da0, "unknown_1409f1da0", fn_1409f1da0 as *const ()), // sub_1409f1da0
    Replacement::draft(0x1409f1ec0, "unknown_1409f1ec0", fn_1409f1ec0 as *const ()), // sub_1409f1ec0
    Replacement::draft_throws(0x1409f1fc0, "unknown_1409f1fc0", fn_1409f1fc0 as *const (), Throws::Discard), // sub_1409f1fc0
    Replacement::draft(0x1409f20e0, "unknown_1409f20e0", fn_1409f20e0 as *const ()), // sub_1409f20e0
    Replacement::draft_throws(0x1409f21b0, "unknown_1409f21b0", fn_1409f21b0 as *const (), Throws::Discard), // sub_1409f21b0
    Replacement::draft_throws(0x1409f22b0, "unknown_1409f22b0", fn_1409f22b0 as *const (), Throws::Caught), // sub_1409f22b0
    Replacement::draft_throws(0x1409f2660, "unknown_1409f2660", fn_1409f2660 as *const (), Throws::Caught), // sub_1409f2660
    Replacement::draft_throws(0x1409f2990, "unknown_1409f2990", fn_1409f2990 as *const (), Throws::Caught), // sub_1409f2990
    Replacement::draft(0x1409f2cb0, "unknown_1409f2cb0", fn_1409f2cb0 as *const ()), // sub_1409f2cb0
    Replacement::draft(0x1409f2d30, "unknown_1409f2d30", fn_1409f2d30 as *const ()), // sub_1409f2d30
    Replacement::draft(0x1409f2e10, "unknown_1409f2e10", fn_1409f2e10 as *const ()), // sub_1409f2e10
    Replacement::draft_throws(0x1409f2f00, "unknown_1409f2f00", fn_1409f2f00 as *const (), Throws::Discard), // sub_1409f2f00
    Replacement::draft_throws(0x1409f2fc0, "unknown_1409f2fc0", fn_1409f2fc0 as *const (), Throws::Discard), // sub_1409f2fc0
    Replacement::draft_throws(0x1409f3130, "unknown_1409f3130", fn_1409f3130 as *const (), Throws::Discard), // sub_1409f3130
    Replacement::draft(0x1409f31f0, "unknown_1409f31f0", tpl_12316::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14329acb8, 0x8> as *const ()), // sub_1409f31f0
    Replacement::draft(0x1409f3270, "unknown_1409f3270", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14329acb8> as *const ()), // sub_1409f3270
    Replacement::draft(0x1409f3370, "unknown_1409f3370", tpl_6896::<0x14387e528, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140911f40, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f3370
    Replacement::draft_throws(0x1409f33e0, "unknown_1409f33e0", tpl_6897::<0x14387e274, 0x14387e528, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1409ff1c0, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409f33e0
    Replacement::draft(0x1409f3450, "unknown_1409f3450", tpl_6896::<0x14387e528, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1409122b0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f3450
    Replacement::draft_throws(0x1409f34c0, "unknown_1409f34c0", tpl_6897::<0x14387e274, 0x14387e528, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1409ff2f0, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409f34c0
    Replacement::draft_throws(0x1409f3530, "unknown_1409f3530", fn_1409f3530 as *const (), Throws::Caught), // sub_1409f3530
    Replacement::draft_throws(0x1409f3600, "unknown_1409f3600", fn_1409f3600 as *const (), Throws::Caught), // sub_1409f3600
    Replacement::draft_throws(0x1409f36d0, "unknown_1409f36d0", fn_1409f36d0 as *const (), Throws::Caught), // sub_1409f36d0
    Replacement::draft(0x1409f37a0, "unknown_1409f37a0", fn_1409f37a0 as *const ()), // sub_1409f37a0
    Replacement::draft(0x1409f3820, "unknown_1409f3820", tpl_6899::<0x14387e274, 0x14387e528, 0x1401e9970, 0x1406e1480, 0x8, 0x140912cf0, 0x1401e9860> as *const ()), // sub_1409f3820
    Replacement::draft(0x1409f3880, "unknown_1409f3880", tpl_6896::<0x14387e528, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140912e60, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f3880
    Replacement::draft_throws(0x1409f38f0, "unknown_1409f38f0", tpl_6900::<0x14387e274, 0x14387e528, 0x1401e9770, 0x1406e1570, 0x8, 0x140912fb0, 0x1401e9590> as *const (), Throws::Discard), // sub_1409f38f0
    Replacement::draft_throws(0x1409f3950, "unknown_1409f3950", tpl_11380::<0x14387e528, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432a6b08, 0x1409ff420> as *const (), Throws::Discard), // sub_1409f3950
    Replacement::draft(0x1409f3a20, "unknown_1409f3a20", tpl_12311::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x143294070, 0x8> as *const ()), // sub_1409f3a20
    Replacement::draft(0x1409f3ac0, "unknown_1409f3ac0", tpl_12312::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143294070, 0xfffffffffffffff0, 0xfffffffffffffff8, 0xfffffffffffffffc, 0x18> as *const ()), // sub_1409f3ac0
    Replacement::draft(0x1409f3be0, "unknown_1409f3be0", tpl_11505::<0x14387def8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f3be0
    Replacement::draft_throws(0x1409f3c80, "unknown_1409f3c80", tpl_11506::<0x14387e40c, 0x14387def8, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x328, 0x338, 0x38, 0x4, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409f3c80
    Replacement::draft(0x1409f3d40, "unknown_1409f3d40", tpl_11507::<0x14387def8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f3d40
    Replacement::draft_throws(0x1409f3de0, "unknown_1409f3de0", tpl_11508::<0x14387e40c, 0x14387def8, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x10, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409f3de0
    Replacement::draft_throws(0x1409f3e80, "unknown_1409f3e80", fn_1409f3e80 as *const (), Throws::Caught), // sub_1409f3e80
    Replacement::draft_throws(0x1409f4080, "unknown_1409f4080", fn_1409f4080 as *const (), Throws::Caught), // sub_1409f4080
    Replacement::draft_throws(0x1409f4250, "unknown_1409f4250", fn_1409f4250 as *const (), Throws::Caught), // sub_1409f4250
    Replacement::draft(0x1409f4420, "unknown_1409f4420", tpl_70013::<0x14387def8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f4420
    Replacement::draft(0x1409f44a0, "unknown_1409f44a0", tpl_11513::<0x14387e40c, 0x14387def8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x1401e9860> as *const ()), // sub_1409f44a0
    Replacement::draft(0x1409f4540, "unknown_1409f4540", tpl_11507::<0x14387def8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f4540
    Replacement::draft_throws(0x1409f45f0, "unknown_1409f45f0", tpl_70014::<0x14387e40c, 0x14387def8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x10, 0x1401e9590> as *const (), Throws::Discard), // sub_1409f45f0
    Replacement::draft_throws(0x1409f4670, "unknown_1409f4670", tpl_11515::<0x14387def8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x4, 0x1432a60d8, 0x1413c41a0> as *const (), Throws::Discard), // sub_1409f4670
    Replacement::draft(0x1409f4770, "unknown_1409f4770", fn_1409f4770 as *const ()), // sub_1409f4770
    Replacement::draft(0x1409f47e0, "unknown_1409f47e0", fn_1409f47e0 as *const ()), // sub_1409f47e0
    Replacement::draft(0x1409f48f0, "unknown_1409f48f0", tpl_12555::<0x14387e4d0, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0xc, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f48f0
    Replacement::draft_throws(0x1409f49d0, "unknown_1409f49d0", tpl_71031::<0x14387df10, 0x14387e4d0, 0x1401e9fa0, 0x328, 0x338, 0x38, 0x4, 0x8, 0x369, 0xc, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409f49d0
    Replacement::draft(0x1409f4aa0, "unknown_1409f4aa0", tpl_12556::<0x14387e4d0, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f4aa0
    Replacement::draft_throws(0x1409f4b40, "unknown_1409f4b40", tpl_12557::<0x14387df10, 0x14387e4d0, 0x1401e9b60, 0x328, 0x338, 0x120, 0x8, 0x369, 0xc, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_1409f4b40
    Replacement::draft_throws(0x1409f4c10, "unknown_1409f4c10", fn_1409f4c10 as *const (), Throws::Caught), // sub_1409f4c10
    Replacement::draft_throws(0x1409f4e40, "unknown_1409f4e40", fn_1409f4e40 as *const (), Throws::Caught), // sub_1409f4e40
    Replacement::draft_throws(0x1409f5040, "unknown_1409f5040", fn_1409f5040 as *const (), Throws::Caught), // sub_1409f5040
    Replacement::draft(0x1409f5240, "unknown_1409f5240", fn_1409f5240 as *const ()), // sub_1409f5240
    Replacement::draft(0x1409f52b0, "unknown_1409f52b0", tpl_12562::<0x14387df10, 0x14387e4d0, 0x1401e9970, 0x338, 0xd8, 0x4, 0x8, 0xc, 0x1401e9860> as *const ()), // sub_1409f52b0
    Replacement::draft(0x1409f5350, "unknown_1409f5350", tpl_12556::<0x14387e4d0, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_1409f5350
    Replacement::draft_throws(0x1409f5400, "unknown_1409f5400", tpl_12563::<0x14387df10, 0x14387e4d0, 0x1401e9770, 0x338, 0x120, 0x8, 0xc, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_1409f5400
    Replacement::draft_throws(0x1409f5490, "unknown_1409f5490", fn_1409f5490 as *const (), Throws::Discard), // sub_1409f5490
    Replacement::draft_throws(0x1409f55b0, "unknown_1409f55b0", fn_1409f55b0 as *const (), Throws::Discard), // sub_1409f55b0
    Replacement::draft(0x1409f5630, "unknown_1409f5630", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143291068, 0x8> as *const ()), // sub_1409f5630
    Replacement::draft(0x1409f56c0, "unknown_1409f56c0", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143291068, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1409f56c0
    Replacement::draft(0x1409f5770, "unknown_1409f5770", fn_1409f5770 as *const ()), // sub_1409f5770
    Replacement::draft(0x1409f5850, "unknown_1409f5850", fn_1409f5850 as *const ()), // sub_1409f5850
    Replacement::draft_throws(0x1409f5940, "unknown_1409f5940", tpl_12433::<0x14387e340, 0x14387e240, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x48, 0x8, 0x140a03410, 0x140a31640, 0x1401e9e70> as *const (), Throws::Discard), // sub_1409f5940
    Replacement::draft(0x1409f59f0, "unknown_1409f59f0", fn_1409f59f0 as *const ()), // sub_1409f59f0
    Replacement::draft_throws(0x1409f5ab0, "unknown_1409f5ab0", tpl_12435::<0x14387e340, 0x14387e240, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x140a03410, 0x140a318f0, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_1409f5ab0
    Replacement::draft(0x1409f5b70, "unknown_1409f5b70", fn_1409f5b70 as *const ()), // sub_1409f5b70
    Replacement::draft_throws(0x1409f5ca0, "unknown_1409f5ca0", fn_1409f5ca0 as *const (), Throws::Caught), // sub_1409f5ca0
    Replacement::draft_throws(0x1409f5d60, "unknown_1409f5d60", fn_1409f5d60 as *const (), Throws::Caught), // sub_1409f5d60
    Replacement::draft_throws(0x1409f5e20, "unknown_1409f5e20", fn_1409f5e20 as *const (), Throws::Caught), // sub_1409f5e20
    Replacement::draft(0x1409f5ee0, "unknown_1409f5ee0", fn_1409f5ee0 as *const ()), // sub_1409f5ee0
    Replacement::draft(0x1409f5f90, "unknown_1409f5f90", fn_1409f5f90 as *const ()), // sub_1409f5f90
    Replacement::draft(0x1409f6040, "unknown_1409f6040", fn_1409f6040 as *const ()), // sub_1409f6040
    Replacement::draft_throws(0x1409f6100, "unknown_1409f6100", fn_1409f6100 as *const (), Throws::Discard), // sub_1409f6100
    Replacement::draft_throws(0x1409f61a0, "unknown_1409f61a0", tpl_6941::<0x14387e240, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432a6c40, 0x8, 0x1432a6c50, 0x140a01470> as *const (), Throws::Discard), // sub_1409f61a0
    Replacement::draft(0x1409f6280, "unknown_1409f6280", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143295e70> as *const ()), // sub_1409f6280
    Replacement::draft(0x1409f62e0, "unknown_1409f62e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143295e70> as *const ()), // sub_1409f62e0
    Replacement::draft_throws(0x1409f63e0, "unknown_1409f63e0", fn_1409f63e0 as *const (), Throws::Caught), // sub_1409f63e0
    Replacement::draft_throws(0x1409f6470, "unknown_1409f6470", tpl_6762::<0x14387e2c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409f6470
    Replacement::draft(0x1409f6520, "unknown_1409f6520", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328eb40> as *const ()), // sub_1409f6520
    Replacement::draft(0x1409f6580, "unknown_1409f6580", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328eb40> as *const ()), // sub_1409f6580
    Replacement::draft_throws(0x1409f6680, "unknown_1409f6680", fn_1409f6680 as *const (), Throws::Caught), // sub_1409f6680
    Replacement::draft_throws(0x1409f6710, "unknown_1409f6710", tpl_6762::<0x14387e230, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409f6710
    Replacement::draft(0x1409f67c0, "unknown_1409f67c0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14328dad8> as *const ()), // sub_1409f67c0
    Replacement::draft(0x1409f6820, "unknown_1409f6820", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14328dad8> as *const ()), // sub_1409f6820
    Replacement::draft_throws(0x1409f6920, "unknown_1409f6920", fn_1409f6920 as *const (), Throws::Caught), // sub_1409f6920
    Replacement::draft_throws(0x1409f69b0, "unknown_1409f69b0", tpl_6762::<0x14387e0e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_1409f69b0
    Replacement::draft(0x1409f6a60, "unknown_1409f6a60", fn_1409f6a60 as *const ()), // sub_1409f6a60
    Replacement::draft(0x1409f6c00, "unknown_1409f6c00", fn_1409f6c00 as *const ()), // sub_1409f6c00
    Replacement::draft(0x1409f6c90, "unknown_1409f6c90", fn_1409f6c90 as *const ()), // sub_1409f6c90
    Replacement::draft_throws(0x1409f6eb0, "unknown_1409f6eb0", fn_1409f6eb0 as *const (), Throws::Discard), // sub_1409f6eb0
    Replacement::draft(0x1409f7060, "unknown_1409f7060", fn_1409f7060 as *const ()), // sub_1409f7060
    Replacement::draft_throws(0x1409f7170, "unknown_1409f7170", fn_1409f7170 as *const (), Throws::Discard), // sub_1409f7170
    Replacement::draft_throws(0x1409f7370, "unknown_1409f7370", fn_1409f7370 as *const (), Throws::Caught), // sub_1409f7370
    Replacement::draft_throws(0x1409f7670, "unknown_1409f7670", fn_1409f7670 as *const (), Throws::Caught), // sub_1409f7670
    Replacement::draft_throws(0x1409f7970, "unknown_1409f7970", fn_1409f7970 as *const (), Throws::Caught), // sub_1409f7970
    Replacement::draft(0x1409f7c70, "unknown_1409f7c70", fn_1409f7c70 as *const ()), // sub_1409f7c70
    Replacement::draft(0x1409f7d30, "unknown_1409f7d30", fn_1409f7d30 as *const ()), // sub_1409f7d30
    Replacement::draft(0x1409f7e40, "unknown_1409f7e40", fn_1409f7e40 as *const ()), // sub_1409f7e40
    Replacement::draft_throws(0x1409f7f60, "unknown_1409f7f60", fn_1409f7f60 as *const (), Throws::Discard), // sub_1409f7f60
    Replacement::draft_throws(0x1409f80b0, "unknown_1409f80b0", fn_1409f80b0 as *const (), Throws::Discard), // sub_1409f80b0
    Replacement::draft(0x1409f8390, "unknown_1409f8390", fn_1409f8390 as *const ()), // sub_1409f8390
    Replacement::draft(0x1409f8520, "content_1409f8520", tpl_1953::<0x140a01810, 0x143084e28, 0x141834d30> as *const ()), // content::AudioSwitchesComponent::content::AudioSwitchesComponent::vfunc_0
    Replacement::draft(0x1409f8580, "content_1409f8580", fn_1409f8580 as *const ()), // content::BoneFoleyComponent::content::BoneFoleyComponent::vfunc_0
    Replacement::draft(0x1409f8620, "content_1409f8620", tpl_1967::<0x140a01eb0, 0x143084e28, 0x141834d30> as *const ()), // content::CharacterFootstepComponent::content::CharacterFootstepComponent::vfunc_0
    Replacement::draft(0x1409f8680, "content_1409f8680", fn_1409f8680 as *const ()), // content::CharacterLookAtComponent::content::CharacterLookAtComponent::vfunc_0
    Replacement::draft(0x1409f86d0, "content_1409f86d0", fn_1409f86d0 as *const ()), // sub_1409f86d0
    Replacement::draft(0x1409f87c0, "content_1409f87c0", fn_1409f87c0 as *const ()), // sub_1409f87c0
    Replacement::draft(0x1409f8900, "content_1409f8900", tpl_12588::<0x140a02140, 0x143084e28, 0x141834d30> as *const ()), // content::GfxGraphComponent::content::GfxGraphComponent::vfunc_0
    Replacement::draft(0x1409f8950, "content_1409f8950", fn_1409f8950 as *const ()), // sub_1409f8950
    Replacement::draft(0x1409f8990, "content_1409f8990", tpl_1953::<0x140a022e0, 0x143084e28, 0x141834d30> as *const ()), // content::GfxGraphSpawnerComponent::content::GfxGraphSpawnerComponent::vfunc_0
    Replacement::draft(0x1409f8a40, "content_1409f8a40", fn_1409f8a40 as *const ()), // sub_1409f8a40
    Replacement::draft(0x1409f8b10, "content_1409f8b10", fn_1409f8b10 as *const ()), // content::PointLightComponent::content::PointLightComponent::vfunc_0
    Replacement::draft(0x1409f8b60, "content_1409f8b60", fn_1409f8b60 as *const ()), // content::ScatterParamsComponent::content::ScatterParamsComponent::vfunc_0
    Replacement::draft(0x1409f8bb0, "content_1409f8bb0", tpl_6267::<0x8, 0xc, 0x140a5b830, 0x14183acc8> as *const ()), // sub_1409f8bb0
    Replacement::draft(0x1409f8c90, "content_1409f8c90", fn_1409f8c90 as *const ()), // content::ScatterPointsComponent::content::ScatterPointsComponent::vfunc_0
    Replacement::draft(0x1409f8d30, "content_1409f8d30", tpl_10125::<0x140a5b960, 0x14183acc8> as *const ()), // sub_1409f8d30
    Replacement::draft(0x1409f8e30, "content_1409f8e30", fn_1409f8e30 as *const ()), // content::SoundComponent::content::SoundComponent::vfunc_0
    Replacement::draft(0x1409f8ec0, "content_1409f8ec0", tpl_1953::<0x140236fa0, 0x143084e28, 0x141834d30> as *const ()), // content::SharedEntityCollectionComponent::content::SharedEntityCollectionComponent::vfunc_0
    Replacement::draft(0x1409f8f10, "content_1409f8f10", tpl_8643::<0x140a02a70, 0x18, 0x8, 0x143084e28, 0x141834d30> as *const ()), // content::StreamingStateComponent::content::StreamingStateComponent::vfunc_0
    Replacement::draft(0x1409f8f80, "unknown_1409f8f80", tpl_4889::<0x1401de830, 0x143084e28> as *const ()), // sub_1409f8f80
    Replacement::draft(0x1409f8fb0, "unknown_1409f8fb0", fn_1409f8fb0 as *const ()), // sub_1409f8fb0
    Replacement::draft(0x1409f9030, "unknown_1409f9030", fn_1409f9030 as *const ()), // sub_1409f9030
    Replacement::draft(0x1409f90f0, "unknown_1409f90f0", tpl_6943::<0x140a5bd90, 0x14183acc8> as *const ()), // sub_1409f90f0
    Replacement::draft(0x1409f91f0, "contentcore_1409f91f0", tpl_1953::<0x1409f9bd0, 0x143084e28, 0x141834d30> as *const ()), // contentcore::ContactSoundMaterialDatabaseComponent::contentcore::ContactSoundMaterialDatabaseComponent::vfunc_0
    Replacement::draft(0x1409f9250, "contentcore_1409f9250", tpl_1953::<0x1401fb9f0, 0x143084e28, 0x141834d30> as *const ()), // contentcore::TagComponent::contentcore::TagComponent::vfunc_0
    Replacement::draft(0x1409f92a0, "contentcore_1409f92a0", fn_1409f92a0 as *const ()), // sub_1409f92a0
    Replacement::draft(0x1409f9330, "contentcore_1409f9330", tpl_570::<0x1401de830, 0x143084e28, 0x141834d30> as *const ()), // contentcore::StreamingUnitComponent::contentcore::StreamingUnitComponent::vfunc_0
    Replacement::draft(0x1409f9380, "content_1409f9380", tpl_1967::<0x1401f2bc0, 0x143084e28, 0x141834d30> as *const ()), // content::AcousticsProbeGroupComponent::content::AcousticsProbeGroupComponent::vfunc_0
    Replacement::draft(0x1409f9450, "content_1409f9450", fn_1409f9450 as *const ()), // content::BoneAttachedWindAreaComponent::content::BoneAttachedWindAreaComponent::vfunc_0
    Replacement::draft(0x1409f9550, "content_1409f9550", fn_1409f9550 as *const ()), // content::DecalComponent::content::DecalComponent::vfunc_0
    Replacement::draft(0x1409f9590, "content_1409f9590", fn_1409f9590 as *const ()), // sub_1409f9590
    Replacement::draft(0x1409f95b0, "content_1409f95b0", tpl_650::<0x90> as *const ()), // sub_1409f95b0
    Replacement::draft(0x1409f9660, "content_1409f9660", fn_1409f9660 as *const ()), // content::HierarchyLODComponent::content::HierarchyLODComponent::vfunc_0
    Replacement::draft(0x1409f96d0, "content_1409f96d0", fn_1409f96d0 as *const ()), // content::LuaScriptComponent::content::LuaScriptComponent::vfunc_0
    Replacement::draft(0x1409f9760, "contentcore_1409f9760", fn_1409f9760 as *const ()), // contentcore::DialogueVoiceProfileComponent::contentcore::DialogueVoiceProfileComponent::vfunc_0
    Replacement::draft(0x1409f97b0, "contentcore_1409f97b0", fn_1409f97b0 as *const ()), // contentcore::ContactSoundOverrideComponent::contentcore::ContactSoundOverrideComponent::vfunc_0
    Replacement::draft(0x1409f9800, "contentcore_1409f9800", fn_1409f9800 as *const ()), // contentcore::TimelineComponent::contentcore::TimelineComponent::vfunc_0
    Replacement::draft(0x1409f9900, "contentcore_1409f9900", fn_1409f9900 as *const ()), // contentcore::HierarchyLODComponent::contentcore::HierarchyLODComponent::vfunc_0
    Replacement::draft(0x1409f99a0, "contentcore_1409f99a0", fn_1409f99a0 as *const ()), // contentcore::UIPageComponent::contentcore::UIPageComponent::vfunc_0
    Replacement::draft(0x1409f99f0, "contentcore_1409f99f0", fn_1409f99f0 as *const ()), // contentcore::NWEDMeshComponent::contentcore::NWEDMeshComponent::vfunc_0
    Replacement::draft(0x1409f9a40, "unknown_1409f9a40", fn_1409f9a40 as *const ()), // sub_1409f9a40
    Replacement::draft(0x1409f9b00, "unknown_1409f9b00", fn_1409f9b00 as *const ()), // sub_1409f9b00
    Replacement::draft(0x1409f9bd0, "unknown_1409f9bd0", tpl_12610::<0x8, 0x140a031c0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1409f9bd0
    Replacement::draft_throws(0x1409f9cc0, "unknown_1409f9cc0", fn_1409f9cc0 as *const (), Throws::Discard), // sub_1409f9cc0
    Replacement::draft_throws(0x1409f9d10, "unknown_1409f9d10", fn_1409f9d10 as *const (), Throws::Caught), // sub_1409f9d10
    Replacement::draft_throws(0x1409f9e40, "unknown_1409f9e40", fn_1409f9e40 as *const (), Throws::Caught), // sub_1409f9e40
    Replacement::draft_throws(0x1409f9f70, "unknown_1409f9f70", fn_1409f9f70 as *const (), Throws::Caught), // sub_1409f9f70
    Replacement::draft(0x1409fa060, "unknown_1409fa060", fn_1409fa060 as *const ()), // sub_1409fa060
    Replacement::draft(0x1409fa0e0, "unknown_1409fa0e0", fn_1409fa0e0 as *const ()), // sub_1409fa0e0
    Replacement::draft(0x1409fa190, "unknown_1409fa190", fn_1409fa190 as *const ()), // sub_1409fa190
    Replacement::draft_throws(0x1409fa290, "unknown_1409fa290", fn_1409fa290 as *const (), Throws::Caught), // sub_1409fa290
    Replacement::draft_throws(0x1409fa440, "unknown_1409fa440", fn_1409fa440 as *const (), Throws::Caught), // sub_1409fa440
    Replacement::draft_throws(0x1409fa630, "unknown_1409fa630", tpl_6967::<0x20, 0x8, 0x140a07e80> as *const (), Throws::Discard), // sub_1409fa630
    Replacement::draft_throws(0x1409fa680, "unknown_1409fa680", fn_1409fa680 as *const (), Throws::Caught), // sub_1409fa680
    Replacement::draft_throws(0x1409fa830, "unknown_1409fa830", fn_1409fa830 as *const (), Throws::Caught), // sub_1409fa830
    Replacement::draft_throws(0x1409fa9e0, "unknown_1409fa9e0", fn_1409fa9e0 as *const (), Throws::Caught), // sub_1409fa9e0
    Replacement::draft_throws(0x1409fab90, "unknown_1409fab90", fn_1409fab90 as *const (), Throws::Caught), // sub_1409fab90
    Replacement::draft_throws(0x1409facc0, "unknown_1409facc0", fn_1409facc0 as *const (), Throws::Caught), // sub_1409facc0
    Replacement::draft_throws(0x1409fae80, "unknown_1409fae80", fn_1409fae80 as *const (), Throws::Caught), // sub_1409fae80
    Replacement::draft_throws(0x1409fb040, "unknown_1409fb040", fn_1409fb040 as *const (), Throws::Caught), // sub_1409fb040
    Replacement::draft_throws(0x1409fb200, "unknown_1409fb200", fn_1409fb200 as *const (), Throws::Caught), // sub_1409fb200
    Replacement::draft_throws(0x1409fb3c0, "unknown_1409fb3c0", fn_1409fb3c0 as *const (), Throws::Discard), // sub_1409fb3c0
    Replacement::draft_throws(0x1409fb410, "unknown_1409fb410", fn_1409fb410 as *const (), Throws::Discard), // sub_1409fb410
    Replacement::draft_throws(0x1409fb460, "unknown_1409fb460", fn_1409fb460 as *const (), Throws::Discard), // sub_1409fb460
    Replacement::draft_throws(0x1409fb4b0, "unknown_1409fb4b0", fn_1409fb4b0 as *const (), Throws::Caught), // sub_1409fb4b0
    Replacement::draft(0x1409fb5a0, "unknown_1409fb5a0", fn_1409fb5a0 as *const ()), // sub_1409fb5a0
    Replacement::draft_throws(0x1409fb690, "unknown_1409fb690", fn_1409fb690 as *const (), Throws::Caught), // sub_1409fb690
    Replacement::draft_throws(0x1409fb840, "unknown_1409fb840", fn_1409fb840 as *const (), Throws::Caught), // sub_1409fb840
    Replacement::draft_throws(0x1409fba30, "unknown_1409fba30", tpl_12629::<0x20, 0x8, 0x140a0b2a0> as *const (), Throws::Discard), // sub_1409fba30
    Replacement::draft_throws(0x1409fbab0, "unknown_1409fbab0", fn_1409fbab0 as *const (), Throws::Discard), // sub_1409fbab0
    Replacement::draft_throws(0x1409fbb00, "unknown_1409fbb00", fn_1409fbb00 as *const (), Throws::Caught), // sub_1409fbb00
    Replacement::draft_throws(0x1409fbc40, "unknown_1409fbc40", fn_1409fbc40 as *const (), Throws::Caught), // sub_1409fbc40
    Replacement::draft_throws(0x1409fbd80, "unknown_1409fbd80", fn_1409fbd80 as *const (), Throws::Caught), // sub_1409fbd80
    Replacement::draft(0x1409fbec0, "unknown_1409fbec0", tpl_12631::<0xa0, 0x20, 0x8, 0xc, 0x14020bcb0> as *const ()), // sub_1409fbec0
    Replacement::draft(0x1409fbf20, "unknown_1409fbf20", fn_1409fbf20 as *const ()), // sub_1409fbf20
    Replacement::draft(0x1409fbf50, "unknown_1409fbf50", fn_1409fbf50 as *const ()), // sub_1409fbf50
    Replacement::draft(0x1409fbfb0, "unknown_1409fbfb0", tpl_9530::<0x8, 0x1426d4580, 0x338, 0x1, 0x1409337c0> as *const ()), // sub_1409fbfb0
    Replacement::draft_throws(0x1409fc050, "unknown_1409fc050", fn_1409fc050 as *const (), Throws::Caught), // sub_1409fc050
    Replacement::draft_throws(0x1409fc210, "unknown_1409fc210", fn_1409fc210 as *const (), Throws::Caught), // sub_1409fc210
    Replacement::draft_throws(0x1409fc430, "unknown_1409fc430", fn_1409fc430 as *const (), Throws::Caught), // sub_1409fc430
    Replacement::draft(0x1409fc720, "unknown_1409fc720", fn_1409fc720 as *const ()), // sub_1409fc720
    Replacement::draft(0x1409fc850, "unknown_1409fc850", fn_1409fc850 as *const ()), // sub_1409fc850
    Replacement::draft_throws(0x1409fc9f0, "unknown_1409fc9f0", fn_1409fc9f0 as *const (), Throws::Caught), // sub_1409fc9f0
    Replacement::draft_throws(0x1409fcad0, "unknown_1409fcad0", tpl_12639::<0x20, 0x8, 0x140a12a00> as *const (), Throws::Discard), // sub_1409fcad0
    Replacement::draft_throws(0x1409fcb50, "unknown_1409fcb50", tpl_12629::<0x20, 0x8, 0x140a13e10> as *const (), Throws::Discard), // sub_1409fcb50
    Replacement::draft(0x1409fcba0, "unknown_1409fcba0", fn_1409fcba0 as *const ()), // sub_1409fcba0
    Replacement::draft(0x1409fcc60, "unknown_1409fcc60", fn_1409fcc60 as *const ()), // sub_1409fcc60
    Replacement::draft_throws(0x1409fcd40, "unknown_1409fcd40", fn_1409fcd40 as *const (), Throws::Caught), // sub_1409fcd40
    Replacement::draft_throws(0x1409fce40, "unknown_1409fce40", fn_1409fce40 as *const (), Throws::Caught), // sub_1409fce40
    Replacement::draft_throws(0x1409fcf40, "unknown_1409fcf40", fn_1409fcf40 as *const (), Throws::Caught), // sub_1409fcf40
    Replacement::draft(0x1409fd120, "unknown_1409fd120", fn_1409fd120 as *const ()), // sub_1409fd120
    Replacement::draft_throws(0x1409fd1d0, "unknown_1409fd1d0", fn_1409fd1d0 as *const (), Throws::Caught), // sub_1409fd1d0
    Replacement::draft_throws(0x1409fd300, "unknown_1409fd300", fn_1409fd300 as *const (), Throws::Caught), // sub_1409fd300
    Replacement::draft_throws(0x1409fd430, "unknown_1409fd430", fn_1409fd430 as *const (), Throws::Caught), // sub_1409fd430
    Replacement::draft_throws(0x1409fd560, "unknown_1409fd560", fn_1409fd560 as *const (), Throws::Caught), // sub_1409fd560
    Replacement::draft_throws(0x1409fd690, "unknown_1409fd690", fn_1409fd690 as *const (), Throws::Caught), // sub_1409fd690
    Replacement::draft_throws(0x1409fd770, "unknown_1409fd770", fn_1409fd770 as *const (), Throws::Caught), // sub_1409fd770
    Replacement::draft(0x1409fd870, "unknown_1409fd870", fn_1409fd870 as *const ()), // sub_1409fd870
    Replacement::draft(0x1409fd960, "unknown_1409fd960", fn_1409fd960 as *const ()), // sub_1409fd960
    Replacement::draft_throws(0x1409fda10, "unknown_1409fda10", fn_1409fda10 as *const (), Throws::Caught), // sub_1409fda10
    Replacement::draft_throws(0x1409fdb40, "unknown_1409fdb40", fn_1409fdb40 as *const (), Throws::Caught), // sub_1409fdb40
    Replacement::draft_throws(0x1409fdcd0, "unknown_1409fdcd0", fn_1409fdcd0 as *const (), Throws::Caught), // sub_1409fdcd0
    Replacement::draft_throws(0x1409fde00, "unknown_1409fde00", tpl_12645::<0x20, 0x8, 0x140a1a140> as *const (), Throws::Discard), // sub_1409fde00
    Replacement::draft_throws(0x1409fde90, "unknown_1409fde90", tpl_2463::<0x20, 0x8, 0x140a1a9f0> as *const (), Throws::Discard), // sub_1409fde90
    Replacement::draft(0x1409fded0, "unknown_1409fded0", fn_1409fded0 as *const ()), // sub_1409fded0
    Replacement::draft_throws(0x1409fdf90, "unknown_1409fdf90", fn_1409fdf90 as *const (), Throws::Caught), // sub_1409fdf90
    Replacement::draft_throws(0x1409fe0c0, "unknown_1409fe0c0", fn_1409fe0c0 as *const (), Throws::Caught), // sub_1409fe0c0
    Replacement::draft_throws(0x1409fe1f0, "unknown_1409fe1f0", fn_1409fe1f0 as *const (), Throws::Caught), // sub_1409fe1f0
    Replacement::draft(0x1409fe2e0, "unknown_1409fe2e0", fn_1409fe2e0 as *const ()), // sub_1409fe2e0
    Replacement::draft_throws(0x1409fe390, "unknown_1409fe390", fn_1409fe390 as *const (), Throws::Caught), // sub_1409fe390
    Replacement::draft_throws(0x1409fe4c0, "unknown_1409fe4c0", fn_1409fe4c0 as *const (), Throws::Caught), // sub_1409fe4c0
    Replacement::draft_throws(0x1409fe5f0, "unknown_1409fe5f0", fn_1409fe5f0 as *const (), Throws::Caught), // sub_1409fe5f0
    Replacement::draft_throws(0x1409fe6d0, "unknown_1409fe6d0", fn_1409fe6d0 as *const (), Throws::Caught), // sub_1409fe6d0
    Replacement::draft_throws(0x1409fe880, "unknown_1409fe880", fn_1409fe880 as *const (), Throws::Caught), // sub_1409fe880
    Replacement::draft_throws(0x1409fea70, "unknown_1409fea70", tpl_9695::<0x20, 0x8, 0x140a1d3d0> as *const (), Throws::Discard), // sub_1409fea70
    Replacement::draft(0x1409feac0, "unknown_1409feac0", fn_1409feac0 as *const ()), // sub_1409feac0
    Replacement::draft(0x1409feb40, "unknown_1409feb40", fn_1409feb40 as *const ()), // sub_1409feb40
    Replacement::draft(0x1409febc0, "unknown_1409febc0", fn_1409febc0 as *const ()), // sub_1409febc0
    Replacement::draft(0x1409fec90, "unknown_1409fec90", tpl_2504::<0x8, 0x1426d4580, 0x338, 0x1, 0x1406fe630> as *const ()), // sub_1409fec90
    Replacement::draft(0x1409fed40, "unknown_1409fed40", fn_1409fed40 as *const ()), // sub_1409fed40
    Replacement::draft(0x1409fedf0, "unknown_1409fedf0", fn_1409fedf0 as *const ()), // sub_1409fedf0
    Replacement::draft_throws(0x1409feea0, "unknown_1409feea0", tpl_838::<0x140a1d7c0> as *const (), Throws::Discard), // sub_1409feea0
    Replacement::draft_throws(0x1409fef40, "unknown_1409fef40", tpl_838::<0x140a1f700> as *const (), Throws::Discard), // sub_1409fef40
    Replacement::draft_throws(0x1409fef90, "unknown_1409fef90", tpl_6985::<0x20, 0x8, 0x140a20680> as *const (), Throws::Discard), // sub_1409fef90
    Replacement::draft_throws(0x1409fefe0, "unknown_1409fefe0", tpl_2463::<0x20, 0x8, 0x140a20740> as *const (), Throws::Discard), // sub_1409fefe0
    Replacement::draft_throws(0x1409ff0c0, "unknown_1409ff0c0", tpl_2507::<0x20, 0x8, 0x140a20a80> as *const (), Throws::Discard), // sub_1409ff0c0
    Replacement::draft(0x1409ff110, "unknown_1409ff110", fn_1409ff110 as *const ()), // sub_1409ff110
    Replacement::draft_throws(0x1409ff1c0, "unknown_1409ff1c0", fn_1409ff1c0 as *const (), Throws::Caught), // sub_1409ff1c0
    Replacement::draft_throws(0x1409ff2f0, "unknown_1409ff2f0", fn_1409ff2f0 as *const (), Throws::Caught), // sub_1409ff2f0
    Replacement::draft_throws(0x1409ff420, "unknown_1409ff420", fn_1409ff420 as *const (), Throws::Caught), // sub_1409ff420
    Replacement::draft(0x1409ff5c0, "unknown_1409ff5c0", fn_1409ff5c0 as *const ()), // sub_1409ff5c0
    Replacement::draft_throws(0x1409ff6a0, "unknown_1409ff6a0", fn_1409ff6a0 as *const (), Throws::Caught), // sub_1409ff6a0
    Replacement::draft_throws(0x1409ff870, "unknown_1409ff870", fn_1409ff870 as *const (), Throws::Caught), // sub_1409ff870
    Replacement::draft(0x1409ffa40, "unknown_1409ffa40", fn_1409ffa40 as *const ()), // sub_1409ffa40
    Replacement::draft(0x1409ffad0, "unknown_1409ffad0", fn_1409ffad0 as *const ()), // sub_1409ffad0
    Replacement::draft(0x1409ffc20, "unknown_1409ffc20", fn_1409ffc20 as *const ()), // sub_1409ffc20
    Replacement::draft_throws(0x1409ffd00, "unknown_1409ffd00", tpl_12660::<0x20, 0x8, 0x140a24150> as *const (), Throws::Discard), // sub_1409ffd00
    Replacement::draft_throws(0x1409ffd50, "unknown_1409ffd50", fn_1409ffd50 as *const (), Throws::Caught), // sub_1409ffd50
    Replacement::draft_throws(0x1409ffe80, "unknown_1409ffe80", fn_1409ffe80 as *const (), Throws::Caught), // sub_1409ffe80
    Replacement::draft_throws(0x140a00030, "unknown_140a00030", fn_140a00030 as *const (), Throws::Caught), // sub_140a00030
    Replacement::draft_throws(0x140a00160, "unknown_140a00160", fn_140a00160 as *const (), Throws::Caught), // sub_140a00160
    Replacement::draft_throws(0x140a00350, "unknown_140a00350", fn_140a00350 as *const (), Throws::Caught), // sub_140a00350
    Replacement::draft_throws(0x140a00430, "unknown_140a00430", tpl_6985::<0x20, 0x8, 0x140a268f0> as *const (), Throws::Discard), // sub_140a00430
    Replacement::draft_throws(0x140a004a0, "unknown_140a004a0", tpl_838::<0x140a27000> as *const (), Throws::Discard), // sub_140a004a0
    Replacement::draft_throws(0x140a004d0, "unknown_140a004d0", tpl_2507::<0x20, 0x8, 0x140a27c50> as *const (), Throws::Discard), // sub_140a004d0
    Replacement::draft(0x140a00520, "unknown_140a00520", fn_140a00520 as *const ()), // sub_140a00520
    Replacement::draft_throws(0x140a006a0, "unknown_140a006a0", tpl_9695::<0x20, 0x8, 0x140a2b590> as *const (), Throws::Discard), // sub_140a006a0
    Replacement::draft_throws(0x140a006f0, "unknown_140a006f0", tpl_6985::<0x20, 0x8, 0x140a2b660> as *const (), Throws::Discard), // sub_140a006f0
    Replacement::draft_throws(0x140a00740, "unknown_140a00740", tpl_12664::<0x20, 0x8, 0x140a2b730> as *const (), Throws::Discard), // sub_140a00740
    Replacement::draft_throws(0x140a00790, "unknown_140a00790", fn_140a00790 as *const (), Throws::Caught), // sub_140a00790
    Replacement::draft_throws(0x140a00940, "unknown_140a00940", fn_140a00940 as *const (), Throws::Caught), // sub_140a00940
    Replacement::draft_throws(0x140a00b30, "unknown_140a00b30", tpl_11265::<0x20, 0x8, 0x140a2c130> as *const (), Throws::Discard), // sub_140a00b30
    Replacement::draft(0x140a00c10, "unknown_140a00c10", fn_140a00c10 as *const ()), // sub_140a00c10
    Replacement::draft(0x140a00cf0, "unknown_140a00cf0", fn_140a00cf0 as *const ()), // sub_140a00cf0
    Replacement::draft(0x140a00dc0, "unknown_140a00dc0", fn_140a00dc0 as *const ()), // sub_140a00dc0
    Replacement::draft_throws(0x140a00ee0, "unknown_140a00ee0", tpl_2507::<0x20, 0x8, 0x140a2e300> as *const (), Throws::Discard), // sub_140a00ee0
    Replacement::draft(0x140a00f30, "unknown_140a00f30", tpl_12669::<0x1406dcfb0, 0x1404b9a10, 0x142417470> as *const ()), // sub_140a00f30
    Replacement::draft(0x140a01080, "unknown_140a01080", tpl_12670::<0x1406dcfb0, 0x1404b9a10, 0x142417470> as *const ()), // sub_140a01080
    Replacement::draft(0x140a01220, "unknown_140a01220", fn_140a01220 as *const ()), // sub_140a01220
    Replacement::draft(0x140a01340, "unknown_140a01340", fn_140a01340 as *const ()), // sub_140a01340
    Replacement::draft_throws(0x140a013e0, "unknown_140a013e0", tpl_6985::<0x20, 0x8, 0x140a30660> as *const (), Throws::Discard), // sub_140a013e0
    Replacement::draft_throws(0x140a01470, "unknown_140a01470", tpl_9695::<0x20, 0x8, 0x140a327f0> as *const (), Throws::Discard), // sub_140a01470
    Replacement::draft(0x140a014c0, "unknown_140a014c0", fn_140a014c0 as *const ()), // sub_140a014c0
    Replacement::draft(0x140a01550, "unknown_140a01550", fn_140a01550 as *const ()), // sub_140a01550
    Replacement::draft(0x140a01730, "unknown_140a01730", tpl_71136::<0x8, 0xc, 0x140a5ae10, 0x140a32d80, 0x140a32b30> as *const ()), // sub_140a01730
    Replacement::draft(0x140a01810, "unknown_140a01810", fn_140a01810 as *const ()), // sub_140a01810
    Replacement::draft(0x140a018f0, "unknown_140a018f0", fn_140a018f0 as *const ()), // sub_140a018f0
    Replacement::draft(0x140a019d0, "unknown_140a019d0", tpl_4024::<0x8, 0x140a32b70, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140a019d0
    Replacement::draft(0x140a01a90, "unknown_140a01a90", fn_140a01a90 as *const ()), // sub_140a01a90
    Replacement::draft(0x140a01b70, "unknown_140a01b70", fn_140a01b70 as *const ()), // sub_140a01b70
    Replacement::draft(0x140a01c30, "unknown_140a01c30", fn_140a01c30 as *const ()), // sub_140a01c30
    Replacement::draft(0x140a01d10, "unknown_140a01d10", fn_140a01d10 as *const ()), // sub_140a01d10
    Replacement::draft(0x140a01dd0, "unknown_140a01dd0", fn_140a01dd0 as *const ()), // sub_140a01dd0
    Replacement::draft(0x140a01eb0, "unknown_140a01eb0", fn_140a01eb0 as *const ()), // sub_140a01eb0
    Replacement::draft(0x140a01f90, "unknown_140a01f90", fn_140a01f90 as *const ()), // sub_140a01f90
    Replacement::draft(0x140a02060, "unknown_140a02060", fn_140a02060 as *const ()), // sub_140a02060
    Replacement::draft(0x140a02140, "unknown_140a02140", tpl_12678::<0x8, 0x140a32cd0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140a02140
    Replacement::draft(0x140a02200, "unknown_140a02200", fn_140a02200 as *const ()), // sub_140a02200
    Replacement::draft(0x140a022e0, "unknown_140a022e0", fn_140a022e0 as *const ()), // sub_140a022e0
    Replacement::draft(0x140a023b0, "unknown_140a023b0", fn_140a023b0 as *const ()), // sub_140a023b0
    Replacement::draft(0x140a02630, "unknown_140a02630", tpl_71150::<0x8, 0xc, 0x140a5b830, 0x10> as *const ()), // sub_140a02630
    Replacement::draft(0x140a02700, "unknown_140a02700", fn_140a02700 as *const ()), // sub_140a02700
    Replacement::draft(0x140a02850, "unknown_140a02850", tpl_71152::<0x140a5b9d0> as *const ()), // sub_140a02850
    Replacement::draft(0x140a02990, "unknown_140a02990", fn_140a02990 as *const ()), // sub_140a02990
    Replacement::draft(0x140a02a70, "unknown_140a02a70", fn_140a02a70 as *const ()), // sub_140a02a70
    Replacement::draft(0x140a02b50, "unknown_140a02b50", fn_140a02b50 as *const ()), // sub_140a02b50
    Replacement::draft(0x140a02ca0, "unknown_140a02ca0", fn_140a02ca0 as *const ()), // sub_140a02ca0
    Replacement::draft(0x140a02dc0, "unknown_140a02dc0", tpl_71150::<0x8, 0xc, 0x14062ee30, 0x10> as *const ()), // sub_140a02dc0
    Replacement::draft(0x140a02e90, "unknown_140a02e90", fn_140a02e90 as *const ()), // sub_140a02e90
    Replacement::draft(0x140a02fb0, "unknown_140a02fb0", fn_140a02fb0 as *const ()), // sub_140a02fb0
    Replacement::draft(0x140a030e0, "unknown_140a030e0", fn_140a030e0 as *const ()), // sub_140a030e0
    Replacement::draft(0x140a031c0, "unknown_140a031c0", fn_140a031c0 as *const ()), // sub_140a031c0
    Replacement::draft(0x140a03220, "unknown_140a03220", fn_140a03220 as *const ()), // sub_140a03220
    Replacement::draft(0x140a03340, "unknown_140a03340", tpl_66027::<0x8, 0xc, 0x140701e10, 0x140a01340> as *const ()), // sub_140a03340
    Replacement::draft(0x140a03410, "unknown_140a03410", tpl_68372::<0x8, 0xc, 0x140a5b830, 0x10> as *const ()), // sub_140a03410
    Replacement::draft(0x140a034e0, "unknown_140a034e0", tpl_6995::<0x1426d4580, 0x140a32a60, 0x140a329c0, 0x1401ee7c0> as *const ()), // sub_140a034e0
    Replacement::draft_throws(0x140a03650, "unknown_140a03650", fn_140a03650 as *const (), Throws::Caught), // sub_140a03650
    Replacement::draft_throws(0x140a03f10, "unknown_140a03f10", fn_140a03f10 as *const (), Throws::Caught), // sub_140a03f10
    Replacement::draft_throws(0x140a04200, "unknown_140a04200", fn_140a04200 as *const (), Throws::Caught), // sub_140a04200
    Replacement::draft_throws(0x140a044c0, "unknown_140a044c0", fn_140a044c0 as *const (), Throws::Caught), // sub_140a044c0
    Replacement::draft_throws(0x140a04770, "unknown_140a04770", fn_140a04770 as *const (), Throws::Caught), // sub_140a04770
    Replacement::draft_throws(0x140a048a0, "unknown_140a048a0", fn_140a048a0 as *const (), Throws::Caught), // sub_140a048a0
    Replacement::draft_throws(0x140a04e20, "unknown_140a04e20", fn_140a04e20 as *const (), Throws::Caught), // sub_140a04e20
    Replacement::draft_throws(0x140a05370, "unknown_140a05370", fn_140a05370 as *const (), Throws::Caught), // sub_140a05370
    Replacement::draft_throws(0x140a057b0, "unknown_140a057b0", fn_140a057b0 as *const (), Throws::Caught), // sub_140a057b0
    Replacement::draft_throws(0x140a05d60, "unknown_140a05d60", fn_140a05d60 as *const (), Throws::Caught), // sub_140a05d60
    Replacement::draft_throws(0x140a062d0, "unknown_140a062d0", fn_140a062d0 as *const (), Throws::Caught), // sub_140a062d0
    Replacement::draft_throws(0x140a06710, "unknown_140a06710", fn_140a06710 as *const (), Throws::Caught), // sub_140a06710
    Replacement::draft_throws(0x140a06c90, "unknown_140a06c90", fn_140a06c90 as *const (), Throws::Caught), // sub_140a06c90
    Replacement::draft_throws(0x140a071e0, "unknown_140a071e0", fn_140a071e0 as *const (), Throws::Caught), // sub_140a071e0
    Replacement::draft_throws(0x140a07620, "unknown_140a07620", fn_140a07620 as *const (), Throws::Caught), // sub_140a07620
    Replacement::draft_throws(0x140a07920, "unknown_140a07920", fn_140a07920 as *const (), Throws::Caught), // sub_140a07920
    Replacement::draft_throws(0x140a07be0, "unknown_140a07be0", fn_140a07be0 as *const (), Throws::Caught), // sub_140a07be0
    Replacement::draft_throws(0x140a07e80, "unknown_140a07e80", fn_140a07e80 as *const (), Throws::Caught), // sub_140a07e80
    Replacement::draft_throws(0x140a07f50, "unknown_140a07f50", fn_140a07f50 as *const (), Throws::Caught), // sub_140a07f50
    Replacement::draft_throws(0x140a08250, "unknown_140a08250", fn_140a08250 as *const (), Throws::Caught), // sub_140a08250
    Replacement::draft_throws(0x140a08550, "unknown_140a08550", fn_140a08550 as *const (), Throws::Caught), // sub_140a08550
    Replacement::draft_throws(0x140a08850, "unknown_140a08850", fn_140a08850 as *const (), Throws::Caught), // sub_140a08850
    Replacement::draft_throws(0x140a08e00, "unknown_140a08e00", fn_140a08e00 as *const (), Throws::Caught), // sub_140a08e00
    Replacement::draft_throws(0x140a090c0, "unknown_140a090c0", fn_140a090c0 as *const (), Throws::Caught), // sub_140a090c0
    Replacement::draft_throws(0x140a09380, "unknown_140a09380", fn_140a09380 as *const (), Throws::Caught), // sub_140a09380
    Replacement::draft_throws(0x140a09640, "unknown_140a09640", fn_140a09640 as *const (), Throws::Caught), // sub_140a09640
    Replacement::draft_throws(0x140a09bb0, "unknown_140a09bb0", fn_140a09bb0 as *const (), Throws::Caught), // sub_140a09bb0
    Replacement::draft_throws(0x140a09e50, "unknown_140a09e50", fn_140a09e50 as *const (), Throws::Caught), // sub_140a09e50
    Replacement::draft_throws(0x140a0a0f0, "unknown_140a0a0f0", fn_140a0a0f0 as *const (), Throws::Caught), // sub_140a0a0f0
    Replacement::draft_throws(0x140a0a390, "unknown_140a0a390", fn_140a0a390 as *const (), Throws::Caught), // sub_140a0a390
    Replacement::draft_throws(0x140a0a7d0, "unknown_140a0a7d0", fn_140a0a7d0 as *const (), Throws::Caught), // sub_140a0a7d0
    Replacement::draft_throws(0x140a0a8a0, "unknown_140a0a8a0", fn_140a0a8a0 as *const (), Throws::Caught), // sub_140a0a8a0
    Replacement::draft_throws(0x140a0a970, "unknown_140a0a970", fn_140a0a970 as *const (), Throws::Caught), // sub_140a0a970
    Replacement::draft_throws(0x140a0aa40, "unknown_140a0aa40", fn_140a0aa40 as *const (), Throws::Caught), // sub_140a0aa40
    Replacement::draft_throws(0x140a0ad40, "unknown_140a0ad40", fn_140a0ad40 as *const (), Throws::Caught), // sub_140a0ad40
    Replacement::draft_throws(0x140a0b000, "unknown_140a0b000", fn_140a0b000 as *const (), Throws::Caught), // sub_140a0b000
    Replacement::draft_throws(0x140a0b2a0, "unknown_140a0b2a0", fn_140a0b2a0 as *const (), Throws::Caught), // sub_140a0b2a0
    Replacement::draft(0x140a0b380, "unknown_140a0b380", fn_140a0b380 as *const ()), // sub_140a0b380
    Replacement::draft_throws(0x140a0b3d0, "unknown_140a0b3d0", fn_140a0b3d0 as *const (), Throws::Caught), // sub_140a0b3d0
    Replacement::draft_throws(0x140a0b700, "unknown_140a0b700", fn_140a0b700 as *const (), Throws::Caught), // sub_140a0b700
    Replacement::draft(0x140a0ba30, "unknown_140a0ba30", fn_140a0ba30 as *const ()), // sub_140a0ba30
    Replacement::draft_throws(0x140a0be80, "unknown_140a0be80", fn_140a0be80 as *const (), Throws::Caught), // sub_140a0be80
    Replacement::draft_throws(0x140a0c320, "unknown_140a0c320", fn_140a0c320 as *const (), Throws::Caught), // sub_140a0c320
    Replacement::draft_throws(0x140a0c790, "unknown_140a0c790", fn_140a0c790 as *const (), Throws::Caught), // sub_140a0c790
    Replacement::draft_throws(0x140a0cbf0, "unknown_140a0cbf0", fn_140a0cbf0 as *const (), Throws::Caught), // sub_140a0cbf0
    Replacement::draft_throws(0x140a0ccc0, "unknown_140a0ccc0", fn_140a0ccc0 as *const (), Throws::Caught), // sub_140a0ccc0
    Replacement::draft_throws(0x140a0d270, "unknown_140a0d270", fn_140a0d270 as *const (), Throws::Caught), // sub_140a0d270
    Replacement::draft_throws(0x140a0d7e0, "unknown_140a0d7e0", fn_140a0d7e0 as *const (), Throws::Caught), // sub_140a0d7e0
    Replacement::draft_throws(0x140a0dc20, "unknown_140a0dc20", fn_140a0dc20 as *const (), Throws::Caught), // sub_140a0dc20
    Replacement::draft_throws(0x140a0def0, "unknown_140a0def0", fn_140a0def0 as *const (), Throws::Caught), // sub_140a0def0
    Replacement::draft_throws(0x140a0e1c0, "unknown_140a0e1c0", fn_140a0e1c0 as *const (), Throws::Caught), // sub_140a0e1c0
    Replacement::draft_throws(0x140a0e490, "unknown_140a0e490", fn_140a0e490 as *const (), Throws::Caught), // sub_140a0e490
    Replacement::draft_throws(0x140a0e730, "unknown_140a0e730", fn_140a0e730 as *const (), Throws::Caught), // sub_140a0e730
    Replacement::draft_throws(0x140a0e9d0, "unknown_140a0e9d0", fn_140a0e9d0 as *const (), Throws::Caught), // sub_140a0e9d0
    Replacement::draft_throws(0x140a0ec70, "unknown_140a0ec70", fn_140a0ec70 as *const (), Throws::Caught), // sub_140a0ec70
    Replacement::draft_throws(0x140a0ef00, "unknown_140a0ef00", fn_140a0ef00 as *const (), Throws::Caught), // sub_140a0ef00
    Replacement::draft_throws(0x140a0f190, "unknown_140a0f190", fn_140a0f190 as *const (), Throws::Caught), // sub_140a0f190
    Replacement::draft_throws(0x140a0f420, "unknown_140a0f420", fn_140a0f420 as *const (), Throws::Caught), // sub_140a0f420
    Replacement::draft_throws(0x140a0f550, "unknown_140a0f550", fn_140a0f550 as *const (), Throws::Caught), // sub_140a0f550
    Replacement::draft_throws(0x140a0f680, "unknown_140a0f680", fn_140a0f680 as *const (), Throws::Caught), // sub_140a0f680
    Replacement::draft_throws(0x140a0f7d0, "unknown_140a0f7d0", fn_140a0f7d0 as *const (), Throws::Caught), // sub_140a0f7d0
    Replacement::draft_throws(0x140a0fd50, "unknown_140a0fd50", fn_140a0fd50 as *const (), Throws::Caught), // sub_140a0fd50
    Replacement::draft_throws(0x140a102a0, "unknown_140a102a0", fn_140a102a0 as *const (), Throws::Caught), // sub_140a102a0
    Replacement::draft_throws(0x140a106e0, "unknown_140a106e0", fn_140a106e0 as *const (), Throws::Caught), // sub_140a106e0
    Replacement::draft_throws(0x140a10a00, "unknown_140a10a00", fn_140a10a00 as *const (), Throws::Caught), // sub_140a10a00
    Replacement::draft_throws(0x140a10d20, "unknown_140a10d20", fn_140a10d20 as *const (), Throws::Caught), // sub_140a10d20
    Replacement::draft_throws(0x140a112d0, "unknown_140a112d0", fn_140a112d0 as *const (), Throws::Caught), // sub_140a112d0
    Replacement::draft_throws(0x140a11780, "unknown_140a11780", fn_140a11780 as *const (), Throws::Caught), // sub_140a11780
    Replacement::draft_throws(0x140a11cf0, "unknown_140a11cf0", fn_140a11cf0 as *const (), Throws::Caught), // sub_140a11cf0
    Replacement::draft_throws(0x140a12160, "unknown_140a12160", fn_140a12160 as *const (), Throws::Caught), // sub_140a12160
    Replacement::draft_throws(0x140a125a0, "unknown_140a125a0", fn_140a125a0 as *const (), Throws::Caught), // sub_140a125a0
    Replacement::draft_throws(0x140a12a00, "unknown_140a12a00", fn_140a12a00 as *const (), Throws::Caught), // sub_140a12a00
    Replacement::draft_throws(0x140a12af0, "unknown_140a12af0", fn_140a12af0 as *const (), Throws::Caught), // sub_140a12af0
    Replacement::draft_throws(0x140a12df0, "unknown_140a12df0", fn_140a12df0 as *const (), Throws::Caught), // sub_140a12df0
    Replacement::draft_throws(0x140a130f0, "unknown_140a130f0", fn_140a130f0 as *const (), Throws::Caught), // sub_140a130f0
    Replacement::draft_throws(0x140a13580, "unknown_140a13580", fn_140a13580 as *const (), Throws::Caught), // sub_140a13580
    Replacement::draft_throws(0x140a139d0, "unknown_140a139d0", fn_140a139d0 as *const (), Throws::Caught), // sub_140a139d0
    Replacement::draft_throws(0x140a13e10, "unknown_140a13e10", fn_140a13e10 as *const (), Throws::Caught), // sub_140a13e10
    Replacement::draft_throws(0x140a13f00, "unknown_140a13f00", fn_140a13f00 as *const (), Throws::Caught), // sub_140a13f00
    Replacement::draft_throws(0x140a14480, "unknown_140a14480", fn_140a14480 as *const (), Throws::Caught), // sub_140a14480
    Replacement::draft_throws(0x140a149d0, "unknown_140a149d0", fn_140a149d0 as *const (), Throws::Caught), // sub_140a149d0
    Replacement::draft_throws(0x140a14e10, "unknown_140a14e10", fn_140a14e10 as *const (), Throws::Caught), // sub_140a14e10
    Replacement::draft_throws(0x140a153c0, "unknown_140a153c0", fn_140a153c0 as *const (), Throws::Caught), // sub_140a153c0
    Replacement::draft_throws(0x140a15970, "unknown_140a15970", fn_140a15970 as *const (), Throws::Caught), // sub_140a15970
    Replacement::draft_throws(0x140a15ee0, "unknown_140a15ee0", fn_140a15ee0 as *const (), Throws::Caught), // sub_140a15ee0
    Replacement::draft_throws(0x140a16450, "unknown_140a16450", fn_140a16450 as *const (), Throws::Caught), // sub_140a16450
    Replacement::draft_throws(0x140a16890, "unknown_140a16890", fn_140a16890 as *const (), Throws::Caught), // sub_140a16890
    Replacement::draft_throws(0x140a16cd0, "unknown_140a16cd0", fn_140a16cd0 as *const (), Throws::Caught), // sub_140a16cd0
    Replacement::draft_throws(0x140a16ff0, "unknown_140a16ff0", fn_140a16ff0 as *const (), Throws::Caught), // sub_140a16ff0
    Replacement::draft_throws(0x140a172e0, "unknown_140a172e0", fn_140a172e0 as *const (), Throws::Discard), // sub_140a172e0
    Replacement::draft_throws(0x140a17540, "unknown_140a17540", fn_140a17540 as *const (), Throws::Caught), // sub_140a17540
    Replacement::draft_throws(0x140a17860, "unknown_140a17860", fn_140a17860 as *const (), Throws::Caught), // sub_140a17860
    Replacement::draft_throws(0x140a17b50, "unknown_140a17b50", fn_140a17b50 as *const (), Throws::Caught), // sub_140a17b50
    Replacement::draft_throws(0x140a18100, "unknown_140a18100", fn_140a18100 as *const (), Throws::Caught), // sub_140a18100
    Replacement::draft_throws(0x140a183f0, "unknown_140a183f0", fn_140a183f0 as *const (), Throws::Caught), // sub_140a183f0
    Replacement::draft_throws(0x140a18890, "unknown_140a18890", fn_140a18890 as *const (), Throws::Caught), // sub_140a18890
    Replacement::draft_throws(0x140a18e00, "unknown_140a18e00", fn_140a18e00 as *const (), Throws::Caught), // sub_140a18e00
    Replacement::draft_throws(0x140a190c0, "unknown_140a190c0", fn_140a190c0 as *const (), Throws::Caught), // sub_140a190c0
    Replacement::draft_throws(0x140a19520, "unknown_140a19520", fn_140a19520 as *const (), Throws::Caught), // sub_140a19520
    Replacement::draft_throws(0x140a19960, "unknown_140a19960", fn_140a19960 as *const (), Throws::Caught), // sub_140a19960
    Replacement::draft_throws(0x140a19c20, "unknown_140a19c20", fn_140a19c20 as *const (), Throws::Caught), // sub_140a19c20
    Replacement::draft_throws(0x140a1a070, "unknown_140a1a070", fn_140a1a070 as *const (), Throws::Caught), // sub_140a1a070
    Replacement::draft_throws(0x140a1a140, "unknown_140a1a140", fn_140a1a140 as *const (), Throws::Caught), // sub_140a1a140
    Replacement::draft_throws(0x140a1a210, "unknown_140a1a210", fn_140a1a210 as *const (), Throws::Caught), // sub_140a1a210
    Replacement::draft_throws(0x140a1a4d0, "unknown_140a1a4d0", fn_140a1a4d0 as *const (), Throws::Caught), // sub_140a1a4d0
    Replacement::draft_throws(0x140a1a760, "unknown_140a1a760", fn_140a1a760 as *const (), Throws::Caught), // sub_140a1a760
    Replacement::draft_throws(0x140a1a9f0, "unknown_140a1a9f0", fn_140a1a9f0 as *const (), Throws::Caught), // sub_140a1a9f0
    Replacement::draft_throws(0x140a1aad0, "unknown_140a1aad0", fn_140a1aad0 as *const (), Throws::Caught), // sub_140a1aad0
    Replacement::draft_throws(0x140a1ac00, "unknown_140a1ac00", fn_140a1ac00 as *const (), Throws::Caught), // sub_140a1ac00
    Replacement::draft_throws(0x140a1ad30, "unknown_140a1ad30", fn_140a1ad30 as *const (), Throws::Caught), // sub_140a1ad30
    Replacement::draft_throws(0x140a1b2e0, "unknown_140a1b2e0", fn_140a1b2e0 as *const (), Throws::Caught), // sub_140a1b2e0
    Replacement::draft_throws(0x140a1b850, "unknown_140a1b850", fn_140a1b850 as *const (), Throws::Caught), // sub_140a1b850
    Replacement::draft_throws(0x140a1bc90, "unknown_140a1bc90", fn_140a1bc90 as *const (), Throws::Caught), // sub_140a1bc90
    Replacement::draft_throws(0x140a1c210, "unknown_140a1c210", fn_140a1c210 as *const (), Throws::Caught), // sub_140a1c210
    Replacement::draft_throws(0x140a1c760, "unknown_140a1c760", fn_140a1c760 as *const (), Throws::Caught), // sub_140a1c760
    Replacement::draft_throws(0x140a1cba0, "unknown_140a1cba0", fn_140a1cba0 as *const (), Throws::Caught), // sub_140a1cba0
    Replacement::draft_throws(0x140a1ce80, "unknown_140a1ce80", fn_140a1ce80 as *const (), Throws::Caught), // sub_140a1ce80
    Replacement::draft_throws(0x140a1d130, "unknown_140a1d130", fn_140a1d130 as *const (), Throws::Caught), // sub_140a1d130
    Replacement::draft_throws(0x140a1d3d0, "unknown_140a1d3d0", fn_140a1d3d0 as *const (), Throws::Caught), // sub_140a1d3d0
    Replacement::draft(0x140a1d4b0, "unknown_140a1d4b0", tpl_12741::<0x1432a71b0, 0x1432a71d0, 0x14088b360> as *const ()), // sub_140a1d4b0
    Replacement::draft_throws(0x140a1d4e0, "unknown_140a1d4e0", fn_140a1d4e0 as *const (), Throws::Caught), // sub_140a1d4e0
    Replacement::draft_throws(0x140a1d650, "unknown_140a1d650", fn_140a1d650 as *const (), Throws::Caught), // sub_140a1d650
    Replacement::draft_throws(0x140a1d7c0, "unknown_140a1d7c0", fn_140a1d7c0 as *const (), Throws::Caught), // sub_140a1d7c0
    Replacement::draft_throws(0x140a1db10, "unknown_140a1db10", fn_140a1db10 as *const (), Throws::Caught), // sub_140a1db10
    Replacement::draft_throws(0x140a1ddd0, "unknown_140a1ddd0", fn_140a1ddd0 as *const (), Throws::Caught), // sub_140a1ddd0
    Replacement::draft_throws(0x140a1e0a0, "unknown_140a1e0a0", fn_140a1e0a0 as *const (), Throws::Caught), // sub_140a1e0a0
    Replacement::draft_throws(0x140a1e370, "unknown_140a1e370", fn_140a1e370 as *const (), Throws::Caught), // sub_140a1e370
    Replacement::draft_throws(0x140a1e7e0, "unknown_140a1e7e0", fn_140a1e7e0 as *const (), Throws::Caught), // sub_140a1e7e0
    Replacement::draft_throws(0x140a1eaf0, "unknown_140a1eaf0", fn_140a1eaf0 as *const (), Throws::Caught), // sub_140a1eaf0
    Replacement::draft_throws(0x140a1ed80, "unknown_140a1ed80", fn_140a1ed80 as *const (), Throws::Caught), // sub_140a1ed80
    Replacement::draft_throws(0x140a1f020, "unknown_140a1f020", fn_140a1f020 as *const (), Throws::Caught), // sub_140a1f020
    Replacement::draft_throws(0x140a1f2c0, "unknown_140a1f2c0", fn_140a1f2c0 as *const (), Throws::Caught), // sub_140a1f2c0
    Replacement::draft_throws(0x140a1f700, "unknown_140a1f700", fn_140a1f700 as *const (), Throws::Caught), // sub_140a1f700
    Replacement::draft_throws(0x140a1fa00, "unknown_140a1fa00", fn_140a1fa00 as *const (), Throws::Caught), // sub_140a1fa00
    Replacement::draft_throws(0x140a1fc90, "unknown_140a1fc90", fn_140a1fc90 as *const (), Throws::Caught), // sub_140a1fc90
    Replacement::draft_throws(0x140a1ff20, "unknown_140a1ff20", fn_140a1ff20 as *const (), Throws::Caught), // sub_140a1ff20
    Replacement::draft_throws(0x140a201b0, "unknown_140a201b0", fn_140a201b0 as *const (), Throws::Caught), // sub_140a201b0
    Replacement::draft(0x140a205e0, "unknown_140a205e0", fn_140a205e0 as *const ()), // sub_140a205e0
    Replacement::draft_throws(0x140a20680, "unknown_140a20680", fn_140a20680 as *const (), Throws::Caught), // sub_140a20680
    Replacement::draft_throws(0x140a20740, "unknown_140a20740", fn_140a20740 as *const (), Throws::Caught), // sub_140a20740
    Replacement::draft_throws(0x140a20820, "unknown_140a20820", fn_140a20820 as *const (), Throws::Caught), // sub_140a20820
    Replacement::draft_throws(0x140a20950, "unknown_140a20950", fn_140a20950 as *const (), Throws::Caught), // sub_140a20950
    Replacement::draft_throws(0x140a20a80, "unknown_140a20a80", fn_140a20a80 as *const (), Throws::Caught), // sub_140a20a80
    Replacement::draft_throws(0x140a20b60, "unknown_140a20b60", fn_140a20b60 as *const (), Throws::Caught), // sub_140a20b60
    Replacement::draft_throws(0x140a21110, "unknown_140a21110", fn_140a21110 as *const (), Throws::Caught), // sub_140a21110
    Replacement::draft_throws(0x140a21680, "unknown_140a21680", fn_140a21680 as *const (), Throws::Caught), // sub_140a21680
    Replacement::draft_throws(0x140a21ac0, "unknown_140a21ac0", fn_140a21ac0 as *const (), Throws::Caught), // sub_140a21ac0
    Replacement::draft_throws(0x140a21d90, "unknown_140a21d90", fn_140a21d90 as *const (), Throws::Caught), // sub_140a21d90
    Replacement::draft_throws(0x140a22030, "unknown_140a22030", fn_140a22030 as *const (), Throws::Caught), // sub_140a22030
    Replacement::draft_throws(0x140a222c0, "unknown_140a222c0", fn_140a222c0 as *const (), Throws::Caught), // sub_140a222c0
    Replacement::draft_throws(0x140a223f0, "unknown_140a223f0", fn_140a223f0 as *const (), Throws::Caught), // sub_140a223f0
    Replacement::draft_throws(0x140a22770, "unknown_140a22770", fn_140a22770 as *const (), Throws::Caught), // sub_140a22770
    Replacement::draft_throws(0x140a22aa0, "unknown_140a22aa0", fn_140a22aa0 as *const (), Throws::Caught), // sub_140a22aa0
    Replacement::draft_throws(0x140a22db0, "unknown_140a22db0", fn_140a22db0 as *const (), Throws::Caught), // sub_140a22db0
    Replacement::draft_throws(0x140a230d0, "unknown_140a230d0", fn_140a230d0 as *const (), Throws::Caught), // sub_140a230d0
    Replacement::draft_throws(0x140a233f0, "unknown_140a233f0", fn_140a233f0 as *const (), Throws::Caught), // sub_140a233f0
    Replacement::draft_throws(0x140a23890, "unknown_140a23890", fn_140a23890 as *const (), Throws::Caught), // sub_140a23890
    Replacement::draft_throws(0x140a23cf0, "unknown_140a23cf0", fn_140a23cf0 as *const (), Throws::Caught), // sub_140a23cf0
    Replacement::draft_throws(0x140a24150, "unknown_140a24150", fn_140a24150 as *const (), Throws::Caught), // sub_140a24150
    Replacement::draft_throws(0x140a24220, "unknown_140a24220", fn_140a24220 as *const (), Throws::Caught), // sub_140a24220
    Replacement::draft_throws(0x140a247a0, "unknown_140a247a0", fn_140a247a0 as *const (), Throws::Caught), // sub_140a247a0
    Replacement::draft_throws(0x140a24d50, "unknown_140a24d50", fn_140a24d50 as *const (), Throws::Caught), // sub_140a24d50
    Replacement::draft_throws(0x140a25050, "unknown_140a25050", fn_140a25050 as *const (), Throws::Caught), // sub_140a25050
    Replacement::draft_throws(0x140a255a0, "unknown_140a255a0", fn_140a255a0 as *const (), Throws::Caught), // sub_140a255a0
    Replacement::draft_throws(0x140a25b10, "unknown_140a25b10", fn_140a25b10 as *const (), Throws::Caught), // sub_140a25b10
    Replacement::draft_throws(0x140a25dd0, "unknown_140a25dd0", fn_140a25dd0 as *const (), Throws::Caught), // sub_140a25dd0
    Replacement::draft_throws(0x140a26210, "unknown_140a26210", fn_140a26210 as *const (), Throws::Caught), // sub_140a26210
    Replacement::draft_throws(0x140a26650, "unknown_140a26650", fn_140a26650 as *const (), Throws::Caught), // sub_140a26650
    Replacement::draft_throws(0x140a268f0, "unknown_140a268f0", fn_140a268f0 as *const (), Throws::Caught), // sub_140a268f0
    Replacement::draft(0x140a269c0, "unknown_140a269c0", tpl_12759::<0x3, 0x4, 0x8, 0x1425fcb20, 0x141834e48> as *const ()), // sub_140a269c0
    Replacement::draft(0x140a26a20, "unknown_140a26a20", tpl_12760::<0x3, 0x4, 0x8, 0x1425fcae0, 0x141834e48> as *const ()), // sub_140a26a20
    Replacement::draft_throws(0x140a26a80, "unknown_140a26a80", fn_140a26a80 as *const (), Throws::Caught), // sub_140a26a80
    Replacement::draft_throws(0x140a26d40, "unknown_140a26d40", fn_140a26d40 as *const (), Throws::Caught), // sub_140a26d40
    Replacement::draft_throws(0x140a27000, "unknown_140a27000", fn_140a27000 as *const (), Throws::Caught), // sub_140a27000
    Replacement::draft_throws(0x140a27440, "unknown_140a27440", fn_140a27440 as *const (), Throws::Caught), // sub_140a27440
    Replacement::draft_throws(0x140a27850, "unknown_140a27850", fn_140a27850 as *const (), Throws::Caught), // sub_140a27850
    Replacement::draft_throws(0x140a27c50, "unknown_140a27c50", fn_140a27c50 as *const (), Throws::Caught), // sub_140a27c50
    Replacement::draft_throws(0x140a27d40, "unknown_140a27d40", fn_140a27d40 as *const (), Throws::Caught), // sub_140a27d40
    Replacement::draft_throws(0x140a28010, "unknown_140a28010", fn_140a28010 as *const (), Throws::Caught), // sub_140a28010
    Replacement::draft_throws(0x140a28310, "unknown_140a28310", fn_140a28310 as *const (), Throws::Caught), // sub_140a28310
    Replacement::draft_throws(0x140a28610, "unknown_140a28610", fn_140a28610 as *const (), Throws::Caught), // sub_140a28610
    Replacement::draft_throws(0x140a288e0, "unknown_140a288e0", fn_140a288e0 as *const (), Throws::Caught), // sub_140a288e0
    Replacement::draft_throws(0x140a28be0, "unknown_140a28be0", fn_140a28be0 as *const (), Throws::Caught), // sub_140a28be0
    Replacement::draft_throws(0x140a28ee0, "unknown_140a28ee0", fn_140a28ee0 as *const (), Throws::Caught), // sub_140a28ee0
    Replacement::draft_throws(0x140a29320, "unknown_140a29320", fn_140a29320 as *const (), Throws::Caught), // sub_140a29320
    Replacement::draft_throws(0x140a297b0, "unknown_140a297b0", fn_140a297b0 as *const (), Throws::Caught), // sub_140a297b0
    Replacement::draft_throws(0x140a29c40, "unknown_140a29c40", fn_140a29c40 as *const (), Throws::Caught), // sub_140a29c40
    Replacement::draft_throws(0x140a2a050, "unknown_140a2a050", fn_140a2a050 as *const (), Throws::Caught), // sub_140a2a050
    Replacement::draft_throws(0x140a2a4b0, "unknown_140a2a4b0", fn_140a2a4b0 as *const (), Throws::Caught), // sub_140a2a4b0
    Replacement::draft_throws(0x140a2a900, "unknown_140a2a900", fn_140a2a900 as *const (), Throws::Caught), // sub_140a2a900
    Replacement::draft_throws(0x140a2ad10, "unknown_140a2ad10", fn_140a2ad10 as *const (), Throws::Caught), // sub_140a2ad10
    Replacement::draft_throws(0x140a2b160, "unknown_140a2b160", fn_140a2b160 as *const (), Throws::Caught), // sub_140a2b160
    Replacement::draft_throws(0x140a2b590, "unknown_140a2b590", fn_140a2b590 as *const (), Throws::Caught), // sub_140a2b590
    Replacement::draft_throws(0x140a2b660, "unknown_140a2b660", fn_140a2b660 as *const (), Throws::Caught), // sub_140a2b660
    Replacement::draft_throws(0x140a2b730, "unknown_140a2b730", fn_140a2b730 as *const (), Throws::Caught), // sub_140a2b730
    Replacement::draft(0x140a2b810, "unknown_140a2b810", fn_140a2b810 as *const ()), // sub_140a2b810
    Replacement::draft(0x140a2b890, "unknown_140a2b890", tpl_12741::<0x1432a5168, 0x1432a51c0, 0x1409fb590> as *const ()), // sub_140a2b890
    Replacement::draft_throws(0x140a2b8d0, "unknown_140a2b8d0", fn_140a2b8d0 as *const (), Throws::Caught), // sub_140a2b8d0
    Replacement::draft_throws(0x140a2bbd0, "unknown_140a2bbd0", fn_140a2bbd0 as *const (), Throws::Caught), // sub_140a2bbd0
    Replacement::draft_throws(0x140a2be90, "unknown_140a2be90", fn_140a2be90 as *const (), Throws::Caught), // sub_140a2be90
    Replacement::draft_throws(0x140a2c130, "unknown_140a2c130", fn_140a2c130 as *const (), Throws::Caught), // sub_140a2c130
    Replacement::draft_throws(0x140a2c220, "unknown_140a2c220", fn_140a2c220 as *const (), Throws::Caught), // sub_140a2c220
    Replacement::draft_throws(0x140a2c4f0, "unknown_140a2c4f0", fn_140a2c4f0 as *const (), Throws::Caught), // sub_140a2c4f0
    Replacement::draft_throws(0x140a2c790, "unknown_140a2c790", fn_140a2c790 as *const (), Throws::Caught), // sub_140a2c790
    Replacement::draft_throws(0x140a2ca20, "unknown_140a2ca20", fn_140a2ca20 as *const (), Throws::Caught), // sub_140a2ca20
    Replacement::draft_throws(0x140a2cb50, "unknown_140a2cb50", fn_140a2cb50 as *const (), Throws::Caught), // sub_140a2cb50
    Replacement::draft_throws(0x140a2d0d0, "unknown_140a2d0d0", fn_140a2d0d0 as *const (), Throws::Caught), // sub_140a2d0d0
    Replacement::draft_throws(0x140a2d620, "unknown_140a2d620", fn_140a2d620 as *const (), Throws::Caught), // sub_140a2d620
    Replacement::draft_throws(0x140a2da60, "unknown_140a2da60", fn_140a2da60 as *const (), Throws::Caught), // sub_140a2da60
    Replacement::draft_throws(0x140a2dd70, "unknown_140a2dd70", fn_140a2dd70 as *const (), Throws::Caught), // sub_140a2dd70
    Replacement::draft_throws(0x140a2e040, "unknown_140a2e040", fn_140a2e040 as *const (), Throws::Caught), // sub_140a2e040
    Replacement::draft_throws(0x140a2e300, "unknown_140a2e300", fn_140a2e300 as *const (), Throws::Caught), // sub_140a2e300
    Replacement::draft_throws(0x140a2e3f0, "unknown_140a2e3f0", fn_140a2e3f0 as *const (), Throws::Caught), // sub_140a2e3f0
    Replacement::draft_throws(0x140a2e760, "unknown_140a2e760", fn_140a2e760 as *const (), Throws::Caught), // sub_140a2e760
    Replacement::draft_throws(0x140a2ea80, "unknown_140a2ea80", fn_140a2ea80 as *const (), Throws::Caught), // sub_140a2ea80
    Replacement::draft_throws(0x140a2ed80, "unknown_140a2ed80", fn_140a2ed80 as *const (), Throws::Caught), // sub_140a2ed80
    Replacement::draft_throws(0x140a2f1a0, "unknown_140a2f1a0", fn_140a2f1a0 as *const (), Throws::Caught), // sub_140a2f1a0
    Replacement::draft_throws(0x140a2f5c0, "unknown_140a2f5c0", fn_140a2f5c0 as *const (), Throws::Caught), // sub_140a2f5c0
    Replacement::draft_throws(0x140a2fb70, "unknown_140a2fb70", fn_140a2fb70 as *const (), Throws::Caught), // sub_140a2fb70
    Replacement::draft_throws(0x140a300f0, "unknown_140a300f0", fn_140a300f0 as *const (), Throws::Caught), // sub_140a300f0
    Replacement::draft_throws(0x140a30660, "unknown_140a30660", fn_140a30660 as *const (), Throws::Caught), // sub_140a30660
    Replacement::draft_throws(0x140a30730, "unknown_140a30730", fn_140a30730 as *const (), Throws::Caught), // sub_140a30730
    Replacement::draft_throws(0x140a30cb0, "unknown_140a30cb0", fn_140a30cb0 as *const (), Throws::Caught), // sub_140a30cb0
    Replacement::draft_throws(0x140a31200, "unknown_140a31200", fn_140a31200 as *const (), Throws::Caught), // sub_140a31200
    Replacement::draft_throws(0x140a31640, "unknown_140a31640", fn_140a31640 as *const (), Throws::Caught), // sub_140a31640
    Replacement::draft_throws(0x140a318f0, "unknown_140a318f0", fn_140a318f0 as *const (), Throws::Caught), // sub_140a318f0
    Replacement::draft_throws(0x140a31ba0, "unknown_140a31ba0", fn_140a31ba0 as *const (), Throws::Caught), // sub_140a31ba0
    Replacement::draft_throws(0x140a31fe0, "unknown_140a31fe0", fn_140a31fe0 as *const (), Throws::Caught), // sub_140a31fe0
    Replacement::draft_throws(0x140a323f0, "unknown_140a323f0", fn_140a323f0 as *const (), Throws::Caught), // sub_140a323f0
    Replacement::draft_throws(0x140a327f0, "unknown_140a327f0", fn_140a327f0 as *const (), Throws::Caught), // sub_140a327f0
    Replacement::draft(0x140a328e0, "unknown_140a328e0", fn_140a328e0 as *const ()), // sub_140a328e0
    Replacement::draft(0x140a32940, "unknown_140a32940", fn_140a32940 as *const ()), // sub_140a32940
    Replacement::draft(0x140a32980, "unknown_140a32980", fn_140a32980 as *const ()), // sub_140a32980
    Replacement::draft(0x140a329c0, "unknown_140a329c0", tpl_7074::<0x143c89fc8, 0x143c89fd0, 0x141834a78, 0x14260b650, 0x143c89fd4, 0x143c89fd8, 0x143c89fd3, 0x141834d18, 0x141834a0c> as *const ()), // sub_140a329c0
    Replacement::draft(0x140a32a60, "unknown_140a32a60", tpl_7075::<0x143c89fcc, 0x143c89f98, 0x141834a78, 0x143c89f9b, 0x143c89f9c, 0x143c89fa0, 0x143243df8, 0x142409b50, 0x14260b640, 0x141834d18, 0x141834a0c> as *const ()), // sub_140a32a60
    Replacement::draft(0x140a32b30, "unknown_140a32b30", fn_140a32b30 as *const ()), // sub_140a32b30
    Replacement::draft(0x140a32b70, "unknown_140a32b70", fn_140a32b70 as *const ()), // sub_140a32b70
    Replacement::draft(0x140a32bd0, "unknown_140a32bd0", fn_140a32bd0 as *const ()), // sub_140a32bd0
    Replacement::draft(0x140a32c30, "unknown_140a32c30", fn_140a32c30 as *const ()), // sub_140a32c30
    Replacement::draft(0x140a32c90, "unknown_140a32c90", fn_140a32c90 as *const ()), // sub_140a32c90
    Replacement::draft(0x140a32cd0, "unknown_140a32cd0", fn_140a32cd0 as *const ()), // sub_140a32cd0
    Replacement::draft(0x140a32d40, "unknown_140a32d40", fn_140a32d40 as *const ()), // sub_140a32d40
    Replacement::draft(0x140a32d80, "unknown_140a32d80", fn_140a32d80 as *const ()), // sub_140a32d80
    Replacement::draft(0x140a32df0, "unknown_140a32df0", fn_140a32df0 as *const ()), // sub_140a32df0
    Replacement::draft(0x140a32ed0, "unknown_140a32ed0", fn_140a32ed0 as *const ()), // sub_140a32ed0
    Replacement::draft(0x140a32fc0, "unknown_140a32fc0", fn_140a32fc0 as *const ()), // sub_140a32fc0
    Replacement::draft(0x140a330b0, "unknown_140a330b0", fn_140a330b0 as *const ()), // sub_140a330b0
    Replacement::draft(0x140a33140, "unknown_140a33140", fn_140a33140 as *const ()), // sub_140a33140
    Replacement::draft(0x140a331c0, "unknown_140a331c0", fn_140a331c0 as *const ()), // sub_140a331c0
    Replacement::draft(0x140a33220, "unknown_140a33220", fn_140a33220 as *const ()), // sub_140a33220
    Replacement::draft(0x140a33390, "unknown_140a33390", fn_140a33390 as *const ()), // sub_140a33390
    Replacement::draft(0x140a335c0, "unknown_140a335c0", fn_140a335c0 as *const ()), // sub_140a335c0
    Replacement::draft(0x140a33860, "unknown_140a33860", fn_140a33860 as *const ()), // sub_140a33860
    Replacement::draft(0x140a33b60, "unknown_140a33b60", fn_140a33b60 as *const ()), // sub_140a33b60
    Replacement::draft(0x140a33e80, "unknown_140a33e80", fn_140a33e80 as *const ()), // sub_140a33e80
    Replacement::draft_throws(0x140a340f0, "unknown_140a340f0", fn_140a340f0 as *const (), Throws::Discard), // sub_140a340f0
    Replacement::draft(0x140a34330, "unknown_140a34330", fn_140a34330 as *const ()), // sub_140a34330
    Replacement::draft_throws(0x140a34580, "unknown_140a34580", fn_140a34580 as *const (), Throws::Discard), // sub_140a34580
    Replacement::draft(0x140a34820, "unknown_140a34820", fn_140a34820 as *const ()), // sub_140a34820
    Replacement::draft(0x140a34aa0, "unknown_140a34aa0", fn_140a34aa0 as *const ()), // sub_140a34aa0
    Replacement::draft(0x140a34d40, "unknown_140a34d40", tpl_69838::<0x140a5b9d0, 0x1401e84a0> as *const ()), // sub_140a34d40
    Replacement::draft(0x140a34f60, "unknown_140a34f60", fn_140a34f60 as *const ()), // sub_140a34f60
    Replacement::draft(0x140a35240, "unknown_140a35240", fn_140a35240 as *const ()), // sub_140a35240
    Replacement::draft(0x140a35400, "unknown_140a35400", fn_140a35400 as *const ()), // sub_140a35400
    Replacement::draft(0x140a35690, "unknown_140a35690", fn_140a35690 as *const ()), // sub_140a35690
    Replacement::draft(0x140a359f0, "unknown_140a359f0", fn_140a359f0 as *const ()), // sub_140a359f0
    Replacement::draft_throws(0x140a35c00, "unknown_140a35c00", fn_140a35c00 as *const (), Throws::Caught), // sub_140a35c00
    Replacement::draft_throws(0x140a360c0, "unknown_140a360c0", fn_140a360c0 as *const (), Throws::Caught), // sub_140a360c0
    Replacement::draft_throws(0x140a36540, "unknown_140a36540", fn_140a36540 as *const (), Throws::Caught), // sub_140a36540
    Replacement::draft_throws(0x140a369c0, "unknown_140a369c0", fn_140a369c0 as *const (), Throws::Caught), // sub_140a369c0
    Replacement::draft_throws(0x140a36e90, "unknown_140a36e90", fn_140a36e90 as *const (), Throws::Caught), // sub_140a36e90
    Replacement::draft_throws(0x140a37320, "unknown_140a37320", fn_140a37320 as *const (), Throws::Caught), // sub_140a37320
    Replacement::draft_throws(0x140a377b0, "unknown_140a377b0", fn_140a377b0 as *const (), Throws::Caught), // sub_140a377b0
    Replacement::draft_throws(0x140a37c80, "unknown_140a37c80", fn_140a37c80 as *const (), Throws::Caught), // sub_140a37c80
    Replacement::draft_throws(0x140a38150, "unknown_140a38150", fn_140a38150 as *const (), Throws::Caught), // sub_140a38150
    Replacement::draft_throws(0x140a38620, "unknown_140a38620", fn_140a38620 as *const (), Throws::Caught), // sub_140a38620
    Replacement::draft_throws(0x140a38ab0, "unknown_140a38ab0", fn_140a38ab0 as *const (), Throws::Caught), // sub_140a38ab0
    Replacement::draft_throws(0x140a38f40, "unknown_140a38f40", fn_140a38f40 as *const (), Throws::Caught), // sub_140a38f40
    Replacement::draft_throws(0x140a393d0, "unknown_140a393d0", fn_140a393d0 as *const (), Throws::Caught), // sub_140a393d0
    Replacement::draft_throws(0x140a39860, "unknown_140a39860", fn_140a39860 as *const (), Throws::Caught), // sub_140a39860
    Replacement::draft_throws(0x140a39cf0, "unknown_140a39cf0", fn_140a39cf0 as *const (), Throws::Caught), // sub_140a39cf0
    Replacement::draft_throws(0x140a3a180, "unknown_140a3a180", fn_140a3a180 as *const (), Throws::Caught), // sub_140a3a180
    Replacement::draft_throws(0x140a3a650, "unknown_140a3a650", fn_140a3a650 as *const (), Throws::Caught), // sub_140a3a650
    Replacement::draft_throws(0x140a3aae0, "unknown_140a3aae0", fn_140a3aae0 as *const (), Throws::Caught), // sub_140a3aae0
    Replacement::draft_throws(0x140a3af70, "unknown_140a3af70", fn_140a3af70 as *const (), Throws::Caught), // sub_140a3af70
    Replacement::draft_throws(0x140a3b440, "unknown_140a3b440", fn_140a3b440 as *const (), Throws::Caught), // sub_140a3b440
    Replacement::draft_throws(0x140a3b8d0, "unknown_140a3b8d0", fn_140a3b8d0 as *const (), Throws::Caught), // sub_140a3b8d0
    Replacement::draft(0x140a3bd60, "unknown_140a3bd60", fn_140a3bd60 as *const ()), // sub_140a3bd60
    Replacement::draft(0x140a3c000, "unknown_140a3c000", fn_140a3c000 as *const ()), // sub_140a3c000
    Replacement::draft_throws(0x140a3c6d0, "unknown_140a3c6d0", fn_140a3c6d0 as *const (), Throws::Discard), // sub_140a3c6d0
    Replacement::draft(0x140a3cec0, "unknown_140a3cec0", fn_140a3cec0 as *const ()), // sub_140a3cec0
    Replacement::draft(0x140a3d3a0, "unknown_140a3d3a0", fn_140a3d3a0 as *const ()), // sub_140a3d3a0
    Replacement::draft_throws(0x140a3d8f0, "unknown_140a3d8f0", fn_140a3d8f0 as *const (), Throws::Caught), // sub_140a3d8f0
    Replacement::draft(0x140a3e250, "unknown_140a3e250", fn_140a3e250 as *const ()), // sub_140a3e250
    Replacement::draft(0x140a3e400, "unknown_140a3e400", fn_140a3e400 as *const ()), // sub_140a3e400
    Replacement::draft(0x140a3e8a0, "unknown_140a3e8a0", fn_140a3e8a0 as *const ()), // sub_140a3e8a0
    Replacement::draft_throws(0x140a3ed10, "unknown_140a3ed10", fn_140a3ed10 as *const (), Throws::Discard), // sub_140a3ed10
    Replacement::draft_throws(0x140a3f1f0, "unknown_140a3f1f0", fn_140a3f1f0 as *const (), Throws::Caught), // sub_140a3f1f0
    Replacement::draft_throws(0x140a3f320, "unknown_140a3f320", fn_140a3f320 as *const (), Throws::Caught), // sub_140a3f320
    Replacement::draft_throws(0x140a3f450, "unknown_140a3f450", fn_140a3f450 as *const (), Throws::Caught), // sub_140a3f450
    Replacement::draft_throws(0x140a3f960, "unknown_140a3f960", fn_140a3f960 as *const (), Throws::Caught), // sub_140a3f960
    Replacement::draft_throws(0x140a3fe70, "unknown_140a3fe70", fn_140a3fe70 as *const (), Throws::Caught), // sub_140a3fe70
    Replacement::draft_throws(0x140a40380, "unknown_140a40380", fn_140a40380 as *const (), Throws::Caught), // sub_140a40380
    Replacement::draft_throws(0x140a40840, "unknown_140a40840", fn_140a40840 as *const (), Throws::Caught), // sub_140a40840
    Replacement::draft_throws(0x140a40d00, "unknown_140a40d00", fn_140a40d00 as *const (), Throws::Caught), // sub_140a40d00
    Replacement::draft_throws(0x140a411c0, "unknown_140a411c0", fn_140a411c0 as *const (), Throws::Caught), // sub_140a411c0
    Replacement::draft_throws(0x140a41680, "unknown_140a41680", fn_140a41680 as *const (), Throws::Caught), // sub_140a41680
    Replacement::draft_throws(0x140a41b40, "unknown_140a41b40", fn_140a41b40 as *const (), Throws::Caught), // sub_140a41b40
    Replacement::draft(0x140a42000, "unknown_140a42000", fn_140a42000 as *const ()), // sub_140a42000
    Replacement::draft_throws(0x140a420c0, "unknown_140a420c0", fn_140a420c0 as *const (), Throws::Caught), // sub_140a420c0
    Replacement::draft_throws(0x140a42590, "unknown_140a42590", fn_140a42590 as *const (), Throws::Caught), // sub_140a42590
    Replacement::draft_throws(0x140a42a20, "unknown_140a42a20", fn_140a42a20 as *const (), Throws::Caught), // sub_140a42a20
    Replacement::draft_throws(0x140a42eb0, "unknown_140a42eb0", fn_140a42eb0 as *const (), Throws::Caught), // sub_140a42eb0
    Replacement::draft_throws(0x140a43380, "unknown_140a43380", fn_140a43380 as *const (), Throws::Caught), // sub_140a43380
    Replacement::draft_throws(0x140a43810, "unknown_140a43810", fn_140a43810 as *const (), Throws::Caught), // sub_140a43810
    Replacement::draft(0x140a43ca0, "unknown_140a43ca0", fn_140a43ca0 as *const ()), // sub_140a43ca0
    Replacement::draft_throws(0x140a43e60, "unknown_140a43e60", fn_140a43e60 as *const (), Throws::Caught), // sub_140a43e60
    Replacement::draft(0x140a44010, "unknown_140a44010", fn_140a44010 as *const ()), // sub_140a44010
    Replacement::draft(0x140a441a0, "unknown_140a441a0", fn_140a441a0 as *const ()), // sub_140a441a0
    Replacement::draft_throws(0x140a442e0, "unknown_140a442e0", fn_140a442e0 as *const (), Throws::Caught), // sub_140a442e0
    Replacement::draft_throws(0x140a44490, "unknown_140a44490", fn_140a44490 as *const (), Throws::Caught), // sub_140a44490
    Replacement::draft_throws(0x140a44960, "unknown_140a44960", fn_140a44960 as *const (), Throws::Caught), // sub_140a44960
    Replacement::draft_throws(0x140a44e30, "unknown_140a44e30", fn_140a44e30 as *const (), Throws::Caught), // sub_140a44e30
    Replacement::draft_throws(0x140a452b0, "unknown_140a452b0", fn_140a452b0 as *const (), Throws::Caught), // sub_140a452b0
    Replacement::draft_throws(0x140a45740, "unknown_140a45740", fn_140a45740 as *const (), Throws::Caught), // sub_140a45740
    Replacement::draft_throws(0x140a45bc0, "unknown_140a45bc0", fn_140a45bc0 as *const (), Throws::Caught), // sub_140a45bc0
    Replacement::draft(0x140a46050, "unknown_140a46050", fn_140a46050 as *const ()), // sub_140a46050
    Replacement::draft(0x140a461c0, "unknown_140a461c0", fn_140a461c0 as *const ()), // sub_140a461c0
    Replacement::draft_throws(0x140a46310, "unknown_140a46310", fn_140a46310 as *const (), Throws::Caught), // sub_140a46310
    Replacement::draft_throws(0x140a46440, "unknown_140a46440", fn_140a46440 as *const (), Throws::Caught), // sub_140a46440
    Replacement::draft(0x140a46640, "unknown_140a46640", fn_140a46640 as *const ()), // sub_140a46640
    Replacement::draft_throws(0x140a466e0, "unknown_140a466e0", fn_140a466e0 as *const (), Throws::Caught), // sub_140a466e0
    Replacement::draft_throws(0x140a46bd0, "unknown_140a46bd0", fn_140a46bd0 as *const (), Throws::Caught), // sub_140a46bd0
    Replacement::draft_throws(0x140a47080, "unknown_140a47080", fn_140a47080 as *const (), Throws::Caught), // sub_140a47080
    Replacement::draft_throws(0x140a47530, "unknown_140a47530", fn_140a47530 as *const (), Throws::Caught), // sub_140a47530
    Replacement::draft_throws(0x140a475d0, "unknown_140a475d0", fn_140a475d0 as *const (), Throws::Caught), // sub_140a475d0
    Replacement::draft_throws(0x140a47b50, "unknown_140a47b50", fn_140a47b50 as *const (), Throws::Caught), // sub_140a47b50
    Replacement::draft_throws(0x140a480a0, "unknown_140a480a0", fn_140a480a0 as *const (), Throws::Caught), // sub_140a480a0
    Replacement::draft_throws(0x140a484e0, "unknown_140a484e0", fn_140a484e0 as *const (), Throws::Caught), // sub_140a484e0
    Replacement::draft_throws(0x140a489b0, "unknown_140a489b0", fn_140a489b0 as *const (), Throws::Caught), // sub_140a489b0
    Replacement::draft_throws(0x140a48e40, "unknown_140a48e40", fn_140a48e40 as *const (), Throws::Caught), // sub_140a48e40
    Replacement::draft_throws(0x140a492d0, "unknown_140a492d0", fn_140a492d0 as *const (), Throws::Caught), // sub_140a492d0
    Replacement::draft_throws(0x140a49900, "unknown_140a49900", fn_140a49900 as *const (), Throws::Caught), // sub_140a49900
    Replacement::draft_throws(0x140a49df0, "unknown_140a49df0", fn_140a49df0 as *const (), Throws::Caught), // sub_140a49df0
    Replacement::draft_throws(0x140a4a300, "unknown_140a4a300", fn_140a4a300 as *const (), Throws::Caught), // sub_140a4a300
    Replacement::draft_throws(0x140a4a810, "unknown_140a4a810", fn_140a4a810 as *const (), Throws::Caught), // sub_140a4a810
    Replacement::draft_throws(0x140a4ace0, "unknown_140a4ace0", fn_140a4ace0 as *const (), Throws::Caught), // sub_140a4ace0
    Replacement::draft_throws(0x140a4b270, "unknown_140a4b270", fn_140a4b270 as *const (), Throws::Caught), // sub_140a4b270
    Replacement::draft_throws(0x140a4b720, "unknown_140a4b720", fn_140a4b720 as *const (), Throws::Caught), // sub_140a4b720
    Replacement::draft_throws(0x140a4bbe0, "unknown_140a4bbe0", fn_140a4bbe0 as *const (), Throws::Caught), // sub_140a4bbe0
    Replacement::draft_throws(0x140a4c0a0, "unknown_140a4c0a0", fn_140a4c0a0 as *const (), Throws::Caught), // sub_140a4c0a0
    Replacement::draft_throws(0x140a4c530, "unknown_140a4c530", fn_140a4c530 as *const (), Throws::Caught), // sub_140a4c530
    Replacement::draft_throws(0x140a4ca80, "unknown_140a4ca80", fn_140a4ca80 as *const (), Throws::Caught), // sub_140a4ca80
    Replacement::draft_throws(0x140a4cf30, "unknown_140a4cf30", fn_140a4cf30 as *const (), Throws::Caught), // sub_140a4cf30
    Replacement::draft_throws(0x140a4d3f0, "unknown_140a4d3f0", fn_140a4d3f0 as *const (), Throws::Caught), // sub_140a4d3f0
    Replacement::draft_throws(0x140a4d8b0, "unknown_140a4d8b0", fn_140a4d8b0 as *const (), Throws::Caught), // sub_140a4d8b0
    Replacement::draft_throws(0x140a4dd40, "unknown_140a4dd40", fn_140a4dd40 as *const (), Throws::Caught), // sub_140a4dd40
    Replacement::draft_throws(0x140a4dde0, "unknown_140a4dde0", fn_140a4dde0 as *const (), Throws::Caught), // sub_140a4dde0
    Replacement::draft_throws(0x140a4e2f0, "unknown_140a4e2f0", fn_140a4e2f0 as *const (), Throws::Caught), // sub_140a4e2f0
    Replacement::draft_throws(0x140a4e7b0, "unknown_140a4e7b0", fn_140a4e7b0 as *const (), Throws::Caught), // sub_140a4e7b0
    Replacement::draft_throws(0x140a4ec70, "unknown_140a4ec70", fn_140a4ec70 as *const (), Throws::Caught), // sub_140a4ec70
    Replacement::draft_throws(0x140a4f320, "unknown_140a4f320", fn_140a4f320 as *const (), Throws::Caught), // sub_140a4f320
    Replacement::draft_throws(0x140a4f950, "unknown_140a4f950", fn_140a4f950 as *const (), Throws::Caught), // sub_140a4f950
    Replacement::draft_throws(0x140a4ff00, "unknown_140a4ff00", fn_140a4ff00 as *const (), Throws::Caught), // sub_140a4ff00
    Replacement::draft_throws(0x140a503d0, "unknown_140a503d0", fn_140a503d0 as *const (), Throws::Caught), // sub_140a503d0
    Replacement::draft_throws(0x140a50860, "unknown_140a50860", fn_140a50860 as *const (), Throws::Caught), // sub_140a50860
    Replacement::draft_throws(0x140a50cf0, "unknown_140a50cf0", fn_140a50cf0 as *const (), Throws::Caught), // sub_140a50cf0
    Replacement::draft_throws(0x140a511c0, "unknown_140a511c0", fn_140a511c0 as *const (), Throws::Caught), // sub_140a511c0
    Replacement::draft_throws(0x140a51650, "unknown_140a51650", fn_140a51650 as *const (), Throws::Caught), // sub_140a51650
    Replacement::draft_throws(0x140a51ae0, "unknown_140a51ae0", fn_140a51ae0 as *const (), Throws::Caught), // sub_140a51ae0
    Replacement::draft_throws(0x140a51fb0, "unknown_140a51fb0", fn_140a51fb0 as *const (), Throws::Caught), // sub_140a51fb0
    Replacement::draft_throws(0x140a52440, "unknown_140a52440", fn_140a52440 as *const (), Throws::Caught), // sub_140a52440
    Replacement::draft_throws(0x140a528d0, "unknown_140a528d0", fn_140a528d0 as *const (), Throws::Caught), // sub_140a528d0
    Replacement::draft_throws(0x140a52da0, "unknown_140a52da0", fn_140a52da0 as *const (), Throws::Caught), // sub_140a52da0
    Replacement::draft_throws(0x140a53270, "unknown_140a53270", fn_140a53270 as *const (), Throws::Caught), // sub_140a53270
    Replacement::draft_throws(0x140a53740, "unknown_140a53740", fn_140a53740 as *const (), Throws::Caught), // sub_140a53740
    Replacement::draft_throws(0x140a53bd0, "unknown_140a53bd0", fn_140a53bd0 as *const (), Throws::Caught), // sub_140a53bd0
    Replacement::draft_throws(0x140a54060, "unknown_140a54060", fn_140a54060 as *const (), Throws::Caught), // sub_140a54060
    Replacement::draft_throws(0x140a544f0, "unknown_140a544f0", fn_140a544f0 as *const (), Throws::Caught), // sub_140a544f0
    Replacement::draft_throws(0x140a54980, "unknown_140a54980", fn_140a54980 as *const (), Throws::Caught), // sub_140a54980
    Replacement::draft_throws(0x140a54e10, "unknown_140a54e10", fn_140a54e10 as *const (), Throws::Caught), // sub_140a54e10
    Replacement::draft_throws(0x140a552a0, "unknown_140a552a0", fn_140a552a0 as *const (), Throws::Caught), // sub_140a552a0
    Replacement::draft_throws(0x140a55770, "unknown_140a55770", fn_140a55770 as *const (), Throws::Caught), // sub_140a55770
    Replacement::draft_throws(0x140a55c00, "unknown_140a55c00", fn_140a55c00 as *const (), Throws::Caught), // sub_140a55c00
    Replacement::draft_throws(0x140a56090, "unknown_140a56090", fn_140a56090 as *const (), Throws::Caught), // sub_140a56090
    Replacement::draft_throws(0x140a565a0, "unknown_140a565a0", fn_140a565a0 as *const (), Throws::Caught), // sub_140a565a0
    Replacement::draft_throws(0x140a56a60, "unknown_140a56a60", fn_140a56a60 as *const (), Throws::Caught), // sub_140a56a60
    Replacement::draft_throws(0x140a56f20, "unknown_140a56f20", fn_140a56f20 as *const (), Throws::Caught), // sub_140a56f20
    Replacement::draft_throws(0x140a574e0, "unknown_140a574e0", fn_140a574e0 as *const (), Throws::Caught), // sub_140a574e0
    Replacement::draft_throws(0x140a57a30, "unknown_140a57a30", fn_140a57a30 as *const (), Throws::Caught), // sub_140a57a30
    Replacement::draft_throws(0x140a57f70, "unknown_140a57f70", fn_140a57f70 as *const (), Throws::Caught), // sub_140a57f70
    Replacement::draft_throws(0x140a58630, "unknown_140a58630", fn_140a58630 as *const (), Throws::Caught), // sub_140a58630
    Replacement::draft_throws(0x140a58c70, "unknown_140a58c70", fn_140a58c70 as *const (), Throws::Caught), // sub_140a58c70
    Replacement::draft_throws(0x140a59230, "unknown_140a59230", fn_140a59230 as *const (), Throws::Caught), // sub_140a59230
    Replacement::draft_throws(0x140a59700, "unknown_140a59700", fn_140a59700 as *const (), Throws::Caught), // sub_140a59700
    Replacement::draft_throws(0x140a59b90, "unknown_140a59b90", fn_140a59b90 as *const (), Throws::Caught), // sub_140a59b90
    Replacement::draft_throws(0x140a5a020, "unknown_140a5a020", fn_140a5a020 as *const (), Throws::Caught), // sub_140a5a020
    Replacement::draft_throws(0x140a5a4f0, "unknown_140a5a4f0", fn_140a5a4f0 as *const (), Throws::Caught), // sub_140a5a4f0
    Replacement::draft_throws(0x140a5a980, "unknown_140a5a980", fn_140a5a980 as *const (), Throws::Caught), // sub_140a5a980
    Replacement::draft(0x140a5ae10, "unknown_140a5ae10", fn_140a5ae10 as *const ()), // sub_140a5ae10
    Replacement::draft(0x140a5aef0, "unknown_140a5aef0", fn_140a5aef0 as *const ()), // sub_140a5aef0
    Replacement::draft(0x140a5b020, "unknown_140a5b020", fn_140a5b020 as *const ()), // sub_140a5b020
    Replacement::draft(0x140a5b180, "unknown_140a5b180", fn_140a5b180 as *const ()), // sub_140a5b180
    Replacement::draft(0x140a5b2f0, "unknown_140a5b2f0", fn_140a5b2f0 as *const ()), // sub_140a5b2f0
    Replacement::draft(0x140a5b3e0, "unknown_140a5b3e0", fn_140a5b3e0 as *const ()), // sub_140a5b3e0
    Replacement::draft(0x140a5b440, "unknown_140a5b440", fn_140a5b440 as *const ()), // sub_140a5b440
    Replacement::draft(0x140a5b530, "unknown_140a5b530", fn_140a5b530 as *const ()), // sub_140a5b530
    Replacement::draft(0x140a5b610, "unknown_140a5b610", fn_140a5b610 as *const ()), // sub_140a5b610
    Replacement::draft(0x140a5b7c0, "unknown_140a5b7c0", tpl_12880::<0x8, 0x14024a9e0, 0x140a6e980, 0x1401de830, 0xc> as *const ()), // sub_140a5b7c0
    Replacement::draft(0x140a5b960, "unknown_140a5b960", fn_140a5b960 as *const ()), // sub_140a5b960
    Replacement::draft(0x140a5b9d0, "unknown_140a5b9d0", tpl_12882::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140a6ea10, 0x140236fa0> as *const ()), // sub_140a5b9d0
    Replacement::draft(0x140a5bb10, "unknown_140a5bb10", fn_140a5bb10 as *const ()), // sub_140a5bb10
    Replacement::draft(0x140a5bce0, "unknown_140a5bce0", fn_140a5bce0 as *const ()), // sub_140a5bce0
    Replacement::draft(0x140a5bd90, "unknown_140a5bd90", fn_140a5bd90 as *const ()), // sub_140a5bd90
    Replacement::draft(0x140a5be00, "unknown_140a5be00", fn_140a5be00 as *const ()), // sub_140a5be00
    Replacement::draft(0x140a5bef0, "unknown_140a5bef0", fn_140a5bef0 as *const ()), // sub_140a5bef0
    Replacement::draft(0x140a5bf60, "unknown_140a5bf60", tpl_12882::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14020b8e0, 0x1401fb9f0> as *const ()), // sub_140a5bf60
    Replacement::draft(0x140a5c0a0, "unknown_140a5c0a0", fn_140a5c0a0 as *const ()), // sub_140a5c0a0
    Replacement::draft_throws(0x140a5c170, "unknown_140a5c170", fn_140a5c170 as *const (), Throws::Caught), // sub_140a5c170
    Replacement::draft_throws(0x140a5c650, "unknown_140a5c650", fn_140a5c650 as *const (), Throws::Caught), // sub_140a5c650
    Replacement::draft_throws(0x140a5cb00, "unknown_140a5cb00", fn_140a5cb00 as *const (), Throws::Caught), // sub_140a5cb00
    Replacement::draft(0x140a5cfa0, "unknown_140a5cfa0", tpl_12891::<0x14387e8e8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x30, 0x2b, 0x2c, 0x28, 0xfffffffffffffff8> as *const ()), // sub_140a5cfa0
    Replacement::draft_throws(0x140a5d090, "unknown_140a5d090", fn_140a5d090 as *const (), Throws::Caught), // sub_140a5d090
    Replacement::draft_throws(0x140a5d1d0, "unknown_140a5d1d0", fn_140a5d1d0 as *const (), Throws::Caught), // sub_140a5d1d0
    Replacement::draft_throws(0x140a5d310, "unknown_140a5d310", fn_140a5d310 as *const (), Throws::Caught), // sub_140a5d310
    Replacement::draft_throws(0x140a5d450, "unknown_140a5d450", fn_140a5d450 as *const (), Throws::Caught), // sub_140a5d450
    Replacement::draft(0x140a5d6d0, "unknown_140a5d6d0", fn_140a5d6d0 as *const ()), // sub_140a5d6d0
    Replacement::draft(0x140a5d890, "unknown_140a5d890", fn_140a5d890 as *const ()), // sub_140a5d890
    Replacement::draft(0x140a5daa0, "unknown_140a5daa0", fn_140a5daa0 as *const ()), // sub_140a5daa0
    Replacement::draft_throws(0x140a5dcc0, "unknown_140a5dcc0", fn_140a5dcc0 as *const (), Throws::Caught), // sub_140a5dcc0
    Replacement::draft_throws(0x140a5de00, "unknown_140a5de00", fn_140a5de00 as *const (), Throws::Caught), // sub_140a5de00
    Replacement::draft_throws(0x140a5df40, "unknown_140a5df40", fn_140a5df40 as *const (), Throws::Caught), // sub_140a5df40
    Replacement::draft_throws(0x140a5e080, "unknown_140a5e080", fn_140a5e080 as *const (), Throws::Caught), // sub_140a5e080
    Replacement::draft_throws(0x140a5e1c0, "unknown_140a5e1c0", fn_140a5e1c0 as *const (), Throws::Caught), // sub_140a5e1c0
    Replacement::draft_throws(0x140a5e300, "unknown_140a5e300", fn_140a5e300 as *const (), Throws::Caught), // sub_140a5e300
    Replacement::draft_throws(0x140a5e440, "unknown_140a5e440", fn_140a5e440 as *const (), Throws::Caught), // sub_140a5e440
    Replacement::draft_throws(0x140a5e580, "unknown_140a5e580", fn_140a5e580 as *const (), Throws::Caught), // sub_140a5e580
    Replacement::draft_throws(0x140a5e6c0, "unknown_140a5e6c0", fn_140a5e6c0 as *const (), Throws::Caught), // sub_140a5e6c0
    Replacement::draft_throws(0x140a5e800, "unknown_140a5e800", fn_140a5e800 as *const (), Throws::Caught), // sub_140a5e800
    Replacement::draft_throws(0x140a5e9f0, "unknown_140a5e9f0", fn_140a5e9f0 as *const (), Throws::Caught), // sub_140a5e9f0
    Replacement::draft_throws(0x140a5ebe0, "unknown_140a5ebe0", fn_140a5ebe0 as *const (), Throws::Caught), // sub_140a5ebe0
    Replacement::draft(0x140a5edd0, "unknown_140a5edd0", fn_140a5edd0 as *const ()), // sub_140a5edd0
    Replacement::draft(0x140a5ee50, "unknown_140a5ee50", fn_140a5ee50 as *const ()), // sub_140a5ee50
    Replacement::draft_throws(0x140a5efd0, "unknown_140a5efd0", fn_140a5efd0 as *const (), Throws::Caught), // sub_140a5efd0
    Replacement::draft_throws(0x140a5f110, "unknown_140a5f110", fn_140a5f110 as *const (), Throws::Caught), // sub_140a5f110
    Replacement::draft_throws(0x140a5f250, "unknown_140a5f250", fn_140a5f250 as *const (), Throws::Caught), // sub_140a5f250
    Replacement::draft_throws(0x140a5f390, "unknown_140a5f390", fn_140a5f390 as *const (), Throws::Caught), // sub_140a5f390
    Replacement::draft(0x140a5f580, "unknown_140a5f580", tpl_11504::<0x50, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc, 0x14240ab10> as *const ()), // sub_140a5f580
    Replacement::draft_throws(0x140a5f630, "unknown_140a5f630", fn_140a5f630 as *const (), Throws::Caught), // sub_140a5f630
    Replacement::draft_throws(0x140a5f8e0, "unknown_140a5f8e0", fn_140a5f8e0 as *const (), Throws::Caught), // sub_140a5f8e0
    Replacement::draft_throws(0x140a5fb90, "unknown_140a5fb90", fn_140a5fb90 as *const (), Throws::Caught), // sub_140a5fb90
    Replacement::draft_throws(0x140a5fe40, "unknown_140a5fe40", fn_140a5fe40 as *const (), Throws::Caught), // sub_140a5fe40
    Replacement::draft_throws(0x140a60030, "unknown_140a60030", fn_140a60030 as *const (), Throws::Caught), // sub_140a60030
    Replacement::draft_throws(0x140a60ba0, "unknown_140a60ba0", fn_140a60ba0 as *const (), Throws::Caught), // sub_140a60ba0
    Replacement::draft_throws(0x140a61640, "unknown_140a61640", fn_140a61640 as *const (), Throws::Caught), // sub_140a61640
    Replacement::draft_throws(0x140a61fa0, "unknown_140a61fa0", fn_140a61fa0 as *const (), Throws::Discard), // sub_140a61fa0
    Replacement::draft_throws(0x140a62470, "unknown_140a62470", fn_140a62470 as *const (), Throws::Caught), // sub_140a62470
    Replacement::draft_throws(0x140a62660, "unknown_140a62660", fn_140a62660 as *const (), Throws::Caught), // sub_140a62660
    Replacement::draft_throws(0x140a62850, "unknown_140a62850", fn_140a62850 as *const (), Throws::Caught), // sub_140a62850
    Replacement::draft(0x140a62a40, "unknown_140a62a40", fn_140a62a40 as *const ()), // sub_140a62a40
    Replacement::draft(0x140a62ac0, "unknown_140a62ac0", fn_140a62ac0 as *const ()), // sub_140a62ac0
    Replacement::draft_throws(0x140a62bd0, "unknown_140a62bd0", fn_140a62bd0 as *const (), Throws::Caught), // sub_140a62bd0
    Replacement::draft_throws(0x140a62e50, "unknown_140a62e50", fn_140a62e50 as *const (), Throws::Caught), // sub_140a62e50
    Replacement::draft_throws(0x140a630d0, "unknown_140a630d0", fn_140a630d0 as *const (), Throws::Caught), // sub_140a630d0
    Replacement::draft_throws(0x140a63350, "unknown_140a63350", fn_140a63350 as *const (), Throws::Caught), // sub_140a63350
    Replacement::draft(0x140a63540, "unknown_140a63540", tpl_12906::<0x143264170, 0x88, 0x143874100, 0x1432a7320, 0x1432a72f0, 0x1401de410, 0x90, 0x18> as *const ()), // sub_140a63540
    Replacement::draft(0x140a635c0, "unknown_140a635c0", tpl_12906::<0x143264170, 0x88, 0x143874170, 0x1432a7320, 0x1432a72f0, 0x1401de410, 0x90, 0x8> as *const ()), // sub_140a635c0
    Replacement::draft(0x140a63640, "unknown_140a63640", fn_140a63640 as *const ()), // sub_140a63640
    Replacement::draft_throws(0x140a63720, "unknown_140a63720", fn_140a63720 as *const (), Throws::Caught), // sub_140a63720
    Replacement::draft_throws(0x140a63990, "unknown_140a63990", fn_140a63990 as *const (), Throws::Caught), // sub_140a63990
    Replacement::draft_throws(0x140a63bf0, "unknown_140a63bf0", fn_140a63bf0 as *const (), Throws::Caught), // sub_140a63bf0
    Replacement::draft_throws(0x140a63e50, "unknown_140a63e50", fn_140a63e50 as *const (), Throws::Caught), // sub_140a63e50
    Replacement::draft(0x140a64040, "unknown_140a64040", fn_140a64040 as *const ()), // sub_140a64040
    Replacement::draft(0x140a64160, "unknown_140a64160", fn_140a64160 as *const ()), // sub_140a64160
    Replacement::draft_throws(0x140a641e0, "unknown_140a641e0", fn_140a641e0 as *const (), Throws::Caught), // sub_140a641e0
    Replacement::draft_throws(0x140a64370, "unknown_140a64370", fn_140a64370 as *const (), Throws::Caught), // sub_140a64370
    Replacement::draft_throws(0x140a64e30, "unknown_140a64e30", fn_140a64e30 as *const (), Throws::Caught), // sub_140a64e30
    Replacement::draft_throws(0x140a65090, "unknown_140a65090", fn_140a65090 as *const (), Throws::Caught), // sub_140a65090
    Replacement::draft_throws(0x140a65220, "unknown_140a65220", fn_140a65220 as *const (), Throws::Caught), // sub_140a65220
    Replacement::draft_throws(0x140a65cb0, "unknown_140a65cb0", fn_140a65cb0 as *const (), Throws::Caught), // sub_140a65cb0
    Replacement::draft_throws(0x140a65f10, "unknown_140a65f10", fn_140a65f10 as *const (), Throws::Caught), // sub_140a65f10
    Replacement::draft_throws(0x140a660a0, "unknown_140a660a0", fn_140a660a0 as *const (), Throws::Caught), // sub_140a660a0
    Replacement::draft_throws(0x140a66b20, "unknown_140a66b20", fn_140a66b20 as *const (), Throws::Caught), // sub_140a66b20
    Replacement::draft_throws(0x140a66d80, "unknown_140a66d80", fn_140a66d80 as *const (), Throws::Caught), // sub_140a66d80
    Replacement::draft_throws(0x140a66f10, "unknown_140a66f10", fn_140a66f10 as *const (), Throws::Caught), // sub_140a66f10
    Replacement::draft_throws(0x140a67100, "unknown_140a67100", fn_140a67100 as *const (), Throws::Discard), // sub_140a67100
    Replacement::draft_throws(0x140a671b0, "unknown_140a671b0", fn_140a671b0 as *const (), Throws::Discard), // sub_140a671b0
    Replacement::draft_throws(0x140a67210, "unknown_140a67210", fn_140a67210 as *const (), Throws::Discard), // sub_140a67210
    Replacement::draft_throws(0x140a67250, "unknown_140a67250", fn_140a67250 as *const (), Throws::Caught), // sub_140a67250
    Replacement::draft(0x140a67400, "unknown_140a67400", fn_140a67400 as *const ()), // sub_140a67400
    Replacement::draft(0x140a67590, "unknown_140a67590", fn_140a67590 as *const ()), // sub_140a67590
    Replacement::draft_throws(0x140a676d0, "unknown_140a676d0", fn_140a676d0 as *const (), Throws::Caught), // sub_140a676d0
    Replacement::draft(0x140a67880, "unknown_140a67880", fn_140a67880 as *const ()), // sub_140a67880
    Replacement::draft_throws(0x140a67970, "unknown_140a67970", fn_140a67970 as *const (), Throws::Caught), // sub_140a67970
    Replacement::draft_throws(0x140a67ab0, "unknown_140a67ab0", fn_140a67ab0 as *const (), Throws::Caught), // sub_140a67ab0
    Replacement::draft_throws(0x140a67bf0, "unknown_140a67bf0", fn_140a67bf0 as *const (), Throws::Caught), // sub_140a67bf0
    Replacement::draft_throws(0x140a67d30, "unknown_140a67d30", fn_140a67d30 as *const (), Throws::Caught), // sub_140a67d30
    Replacement::draft(0x140a67f20, "unknown_140a67f20", fn_140a67f20 as *const ()), // sub_140a67f20
    Replacement::draft_throws(0x140a680e0, "unknown_140a680e0", tpl_2826::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2a90, 0x1426dd3fd, 0x1430b20f0, 0x1401f2430, 0x20, 0x1401eb150, 0x14240ab10> as *const (), Throws::Discard), // sub_140a680e0
    Replacement::draft_throws(0x140a681a0, "unknown_140a681a0", fn_140a681a0 as *const (), Throws::Caught), // sub_140a681a0
    Replacement::draft_throws(0x140a683f0, "unknown_140a683f0", tpl_2827::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2a90> as *const (), Throws::Discard), // sub_140a683f0
    Replacement::draft_throws(0x140a68450, "unknown_140a68450", fn_140a68450 as *const (), Throws::Caught), // sub_140a68450
    Replacement::draft_throws(0x140a686a0, "unknown_140a686a0", tpl_2828::<0x54, 0x48, 0x14330f110, 0x8, 0x50, 0x1413c2a90> as *const (), Throws::Discard), // sub_140a686a0
    Replacement::draft_throws(0x140a686f0, "unknown_140a686f0", fn_140a686f0 as *const (), Throws::Caught), // sub_140a686f0
    Replacement::draft_throws(0x140a68940, "unknown_140a68940", fn_140a68940 as *const (), Throws::Caught), // sub_140a68940
    Replacement::draft_throws(0x140a68b30, "unknown_140a68b30", fn_140a68b30 as *const (), Throws::Caught), // sub_140a68b30
    Replacement::draft_throws(0x140a68d20, "unknown_140a68d20", fn_140a68d20 as *const (), Throws::Caught), // sub_140a68d20
    Replacement::draft(0x140a68f10, "unknown_140a68f10", fn_140a68f10 as *const ()), // sub_140a68f10
    Replacement::draft_throws(0x140a68fb0, "unknown_140a68fb0", fn_140a68fb0 as *const (), Throws::Caught), // sub_140a68fb0
    Replacement::draft_throws(0x140a691a0, "unknown_140a691a0", fn_140a691a0 as *const (), Throws::Caught), // sub_140a691a0
    Replacement::draft_throws(0x140a69430, "unknown_140a69430", fn_140a69430 as *const (), Throws::Caught), // sub_140a69430
    Replacement::draft_throws(0x140a696b0, "unknown_140a696b0", fn_140a696b0 as *const (), Throws::Caught), // sub_140a696b0
    Replacement::draft_throws(0x140a69930, "unknown_140a69930", fn_140a69930 as *const (), Throws::Caught), // sub_140a69930
    Replacement::draft_throws(0x140a69bb0, "unknown_140a69bb0", fn_140a69bb0 as *const (), Throws::Caught), // sub_140a69bb0
    Replacement::draft_throws(0x140a69cf0, "unknown_140a69cf0", fn_140a69cf0 as *const (), Throws::Caught), // sub_140a69cf0
    Replacement::draft_throws(0x140a69e30, "unknown_140a69e30", fn_140a69e30 as *const (), Throws::Caught), // sub_140a69e30
    Replacement::draft_throws(0x140a69f70, "unknown_140a69f70", fn_140a69f70 as *const (), Throws::Caught), // sub_140a69f70
    Replacement::draft_throws(0x140a6a160, "unknown_140a6a160", fn_140a6a160 as *const (), Throws::Caught), // sub_140a6a160
    Replacement::draft_throws(0x140a6a3c0, "unknown_140a6a3c0", fn_140a6a3c0 as *const (), Throws::Caught), // sub_140a6a3c0
    Replacement::draft_throws(0x140a6a610, "unknown_140a6a610", fn_140a6a610 as *const (), Throws::Caught), // sub_140a6a610
    Replacement::draft_throws(0x140a6a860, "unknown_140a6a860", fn_140a6a860 as *const (), Throws::Caught), // sub_140a6a860
    Replacement::draft_throws(0x140a6aa50, "unknown_140a6aa50", fn_140a6aa50 as *const (), Throws::Caught), // sub_140a6aa50
    Replacement::draft_throws(0x140a6acb0, "unknown_140a6acb0", fn_140a6acb0 as *const (), Throws::Caught), // sub_140a6acb0
    Replacement::draft_throws(0x140a6af30, "unknown_140a6af30", fn_140a6af30 as *const (), Throws::Caught), // sub_140a6af30
    Replacement::draft_throws(0x140a6b1a0, "unknown_140a6b1a0", fn_140a6b1a0 as *const (), Throws::Caught), // sub_140a6b1a0
    Replacement::draft_throws(0x140a6b400, "unknown_140a6b400", fn_140a6b400 as *const (), Throws::Caught), // sub_140a6b400
    Replacement::draft_throws(0x140a6b680, "unknown_140a6b680", fn_140a6b680 as *const (), Throws::Caught), // sub_140a6b680
    Replacement::draft_throws(0x140a6b8f0, "unknown_140a6b8f0", fn_140a6b8f0 as *const (), Throws::Caught), // sub_140a6b8f0
    Replacement::draft_throws(0x140a6bb50, "unknown_140a6bb50", fn_140a6bb50 as *const (), Throws::Caught), // sub_140a6bb50
    Replacement::draft_throws(0x140a6bdd0, "unknown_140a6bdd0", fn_140a6bdd0 as *const (), Throws::Caught), // sub_140a6bdd0
    Replacement::draft_throws(0x140a6c040, "unknown_140a6c040", fn_140a6c040 as *const (), Throws::Caught), // sub_140a6c040
    Replacement::draft_throws(0x140a6c320, "unknown_140a6c320", fn_140a6c320 as *const (), Throws::Caught), // sub_140a6c320
    Replacement::draft_throws(0x140a6c510, "unknown_140a6c510", fn_140a6c510 as *const (), Throws::Caught), // sub_140a6c510
    Replacement::draft_throws(0x140a6c700, "unknown_140a6c700", fn_140a6c700 as *const (), Throws::Caught), // sub_140a6c700
    Replacement::draft_throws(0x140a6c840, "unknown_140a6c840", fn_140a6c840 as *const (), Throws::Caught), // sub_140a6c840
    Replacement::draft_throws(0x140a6c980, "unknown_140a6c980", fn_140a6c980 as *const (), Throws::Caught), // sub_140a6c980
    Replacement::draft_throws(0x140a6cac0, "unknown_140a6cac0", fn_140a6cac0 as *const (), Throws::Caught), // sub_140a6cac0
    Replacement::draft_throws(0x140a6ccb0, "unknown_140a6ccb0", fn_140a6ccb0 as *const (), Throws::Caught), // sub_140a6ccb0
    Replacement::draft_throws(0x140a6cea0, "unknown_140a6cea0", fn_140a6cea0 as *const (), Throws::Caught), // sub_140a6cea0
    Replacement::draft_throws(0x140a6d1e0, "unknown_140a6d1e0", fn_140a6d1e0 as *const (), Throws::Caught), // sub_140a6d1e0
    Replacement::draft_throws(0x140a6d510, "unknown_140a6d510", fn_140a6d510 as *const (), Throws::Caught), // sub_140a6d510
    Replacement::draft_throws(0x140a6d840, "unknown_140a6d840", fn_140a6d840 as *const (), Throws::Caught), // sub_140a6d840
    Replacement::draft_throws(0x140a6da30, "unknown_140a6da30", fn_140a6da30 as *const (), Throws::Caught), // sub_140a6da30
    Replacement::draft_throws(0x140a6dc60, "unknown_140a6dc60", fn_140a6dc60 as *const (), Throws::Caught), // sub_140a6dc60
    Replacement::draft_throws(0x140a6de90, "unknown_140a6de90", fn_140a6de90 as *const (), Throws::Caught), // sub_140a6de90
    Replacement::draft_throws(0x140a6e0c0, "unknown_140a6e0c0", fn_140a6e0c0 as *const (), Throws::Caught), // sub_140a6e0c0
    Replacement::draft(0x140a6e2b0, "unknown_140a6e2b0", fn_140a6e2b0 as *const ()), // sub_140a6e2b0
    Replacement::draft(0x140a6e320, "unknown_140a6e320", fn_140a6e320 as *const ()), // sub_140a6e320
    Replacement::draft(0x140a6e3a0, "unknown_140a6e3a0", tpl_12946::<0x8, 0x10, 0x14240ab10> as *const ()), // sub_140a6e3a0
    Replacement::draft(0x140a6e3e0, "unknown_140a6e3e0", fn_140a6e3e0 as *const ()), // sub_140a6e3e0
    Replacement::draft(0x140a6e450, "unknown_140a6e450", tpl_5227::<0x8, 0x28, 0x14240ab10> as *const ()), // sub_140a6e450
    Replacement::draft(0x140a6e490, "unknown_140a6e490", fn_140a6e490 as *const ()), // sub_140a6e490
    Replacement::draft(0x140a6e4f0, "unknown_140a6e4f0", fn_140a6e4f0 as *const ()), // sub_140a6e4f0
    Replacement::draft(0x140a6e550, "unknown_140a6e550", fn_140a6e550 as *const ()), // sub_140a6e550
    Replacement::draft(0x140a6e5a0, "unknown_140a6e5a0", fn_140a6e5a0 as *const ()), // sub_140a6e5a0
    Replacement::draft_throws(0x140a6e5e0, "unknown_140a6e5e0", tpl_65670::<0x142442de0, 0x143a3ade8, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x68> as *const (), Throws::Discard), // sub_140a6e5e0
    Replacement::draft_throws(0x140a6e6c0, "unknown_140a6e6c0", tpl_65670::<0x142442de0, 0x143a3dab0, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_140a6e6c0
    Replacement::draft(0x140a6ea10, "unknown_140a6ea10", fn_140a6ea10 as *const ()), // sub_140a6ea10
    Replacement::draft(0x140a6ea80, "unknown_140a6ea80", fn_140a6ea80 as *const ()), // sub_140a6ea80
    Replacement::draft(0x140a6eab0, "unknown_140a6eab0", fn_140a6eab0 as *const ()), // sub_140a6eab0
    Replacement::draft(0x140a6ebe0, "unknown_140a6ebe0", fn_140a6ebe0 as *const ()), // sub_140a6ebe0
    Replacement::draft(0x140a6ed10, "unknown_140a6ed10", fn_140a6ed10 as *const ()), // sub_140a6ed10
    Replacement::draft(0x140a6ee00, "unknown_140a6ee00", fn_140a6ee00 as *const ()), // sub_140a6ee00
    Replacement::draft(0x140a6eef0, "unknown_140a6eef0", fn_140a6eef0 as *const ()), // sub_140a6eef0
    Replacement::draft(0x140a6efd0, "unknown_140a6efd0", fn_140a6efd0 as *const ()), // sub_140a6efd0
    Replacement::draft_throws(0x140a6f0b0, "unknown_140a6f0b0", tpl_71382::<0x14387e7d0, 0x14387e8e8, 0x1401e9fa0, 0x1426cdcb0, 0x328, 0x338, 0x20, 0x369, 0x142409b50, 0x28, 0x1401e9e70> as *const (), Throws::Discard), // sub_140a6f0b0
    Replacement::draft(0x140a6f1b0, "unknown_140a6f1b0", fn_140a6f1b0 as *const ()), // sub_140a6f1b0
    Replacement::draft(0x140a6f260, "unknown_140a6f260", tpl_12961::<0x14387e8e8, 0x14243b0b0, 0x338, 0x20, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a6f260
    Replacement::draft_throws(0x140a6f2e0, "unknown_140a6f2e0", tpl_71384::<0x14387e7d0, 0x14387e8e8, 0x1401e9b60, 0x1426cdcb0, 0x328, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x28, 0x1401e9a20> as *const (), Throws::Discard), // sub_140a6f2e0
    Replacement::draft_throws(0x140a6f3e0, "unknown_140a6f3e0", fn_140a6f3e0 as *const (), Throws::Caught), // sub_140a6f3e0
    Replacement::draft_throws(0x140a6f850, "unknown_140a6f850", fn_140a6f850 as *const (), Throws::Caught), // sub_140a6f850
    Replacement::draft_throws(0x140a6fc80, "unknown_140a6fc80", fn_140a6fc80 as *const (), Throws::Caught), // sub_140a6fc80
    Replacement::draft(0x140a6ffd0, "unknown_140a6ffd0", tpl_12962::<0x14387e7d0, 0x14387e8e8, 0x1401e9970, 0x338, 0xc0, 0x28, 0x1401e9860> as *const ()), // sub_140a6ffd0
    Replacement::draft(0x140a70040, "unknown_140a70040", tpl_12961::<0x14387e8e8, 0x14243b0b0, 0x338, 0xc0, 0x28, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a70040
    Replacement::draft_throws(0x140a700c0, "unknown_140a700c0", tpl_12963::<0x14387e7d0, 0x14387e8e8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x28, 0x1401e9590> as *const (), Throws::Discard), // sub_140a700c0
    Replacement::draft_throws(0x140a70130, "unknown_140a70130", fn_140a70130 as *const (), Throws::Discard), // sub_140a70130
    Replacement::draft(0x140a702c0, "unknown_140a702c0", fn_140a702c0 as *const ()), // sub_140a702c0
    Replacement::draft_throws(0x140a703a0, "unknown_140a703a0", fn_140a703a0 as *const (), Throws::Discard), // sub_140a703a0
    Replacement::draft(0x140a70580, "unknown_140a70580", fn_140a70580 as *const ()), // sub_140a70580
    Replacement::draft_throws(0x140a70690, "unknown_140a70690", fn_140a70690 as *const (), Throws::Discard), // sub_140a70690
    Replacement::draft(0x140a708a0, "unknown_140a708a0", fn_140a708a0 as *const ()), // sub_140a708a0
    Replacement::draft(0x140a709c0, "unknown_140a709c0", tpl_12969::<0x14387e788, 0x14243b0b0, 0x338, 0x20, 0x48, 0x70, 0x38, 0x74, 0x78, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140a709c0
    Replacement::draft(0x140a70a70, "unknown_140a70a70", tpl_12970::<0x14387e800, 0x14243b0b0, 0x338, 0x20, 0x48, 0x70, 0x98, 0x38, 0x9c, 0xa0, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140a70a70
    Replacement::draft(0x140a70b40, "unknown_140a70b40", fn_140a70b40 as *const ()), // sub_140a70b40
    Replacement::draft_throws(0x140a70c50, "unknown_140a70c50", fn_140a70c50 as *const (), Throws::Discard), // sub_140a70c50
    Replacement::draft_throws(0x140a70df0, "unknown_140a70df0", fn_140a70df0 as *const (), Throws::Discard), // sub_140a70df0
    Replacement::draft_throws(0x140a70fe0, "unknown_140a70fe0", fn_140a70fe0 as *const (), Throws::Discard), // sub_140a70fe0
    Replacement::draft(0x140a711f0, "unknown_140a711f0", fn_140a711f0 as *const ()), // sub_140a711f0
    Replacement::draft(0x140a71260, "unknown_140a71260", fn_140a71260 as *const ()), // sub_140a71260
    Replacement::draft(0x140a71300, "unknown_140a71300", fn_140a71300 as *const ()), // sub_140a71300
    Replacement::draft_throws(0x140a713b0, "unknown_140a713b0", fn_140a713b0 as *const (), Throws::Caught), // sub_140a713b0
    Replacement::draft_throws(0x140a71820, "unknown_140a71820", fn_140a71820 as *const (), Throws::Caught), // sub_140a71820
    Replacement::draft_throws(0x140a71c90, "unknown_140a71c90", fn_140a71c90 as *const (), Throws::Caught), // sub_140a71c90
    Replacement::draft_throws(0x140a72100, "unknown_140a72100", fn_140a72100 as *const (), Throws::Caught), // sub_140a72100
    Replacement::draft_throws(0x140a72530, "unknown_140a72530", fn_140a72530 as *const (), Throws::Caught), // sub_140a72530
    Replacement::draft_throws(0x140a72960, "unknown_140a72960", fn_140a72960 as *const (), Throws::Caught), // sub_140a72960
    Replacement::draft_throws(0x140a72d90, "unknown_140a72d90", fn_140a72d90 as *const (), Throws::Caught), // sub_140a72d90
    Replacement::draft_throws(0x140a730e0, "unknown_140a730e0", fn_140a730e0 as *const (), Throws::Caught), // sub_140a730e0
    Replacement::draft_throws(0x140a73430, "unknown_140a73430", fn_140a73430 as *const (), Throws::Caught), // sub_140a73430
    Replacement::draft(0x140a73780, "unknown_140a73780", fn_140a73780 as *const ()), // sub_140a73780
    Replacement::draft(0x140a73840, "unknown_140a73840", fn_140a73840 as *const ()), // sub_140a73840
    Replacement::draft(0x140a73910, "unknown_140a73910", fn_140a73910 as *const ()), // sub_140a73910
    Replacement::draft(0x140a73a20, "unknown_140a73a20", tpl_12969::<0x14387e788, 0x14243b0b0, 0x338, 0xc0, 0x48, 0x70, 0xd8, 0x74, 0x78, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140a73a20
    Replacement::draft(0x140a73ae0, "unknown_140a73ae0", tpl_12970::<0x14387e800, 0x14243b0b0, 0x338, 0xc0, 0x48, 0x70, 0x98, 0xd8, 0x9c, 0xa0, 0x8, 0xfffffffffffffff8> as *const ()), // sub_140a73ae0
    Replacement::draft(0x140a73bc0, "unknown_140a73bc0", fn_140a73bc0 as *const ()), // sub_140a73bc0
    Replacement::draft_throws(0x140a73ce0, "unknown_140a73ce0", fn_140a73ce0 as *const (), Throws::Discard), // sub_140a73ce0
    Replacement::draft_throws(0x140a73da0, "unknown_140a73da0", fn_140a73da0 as *const (), Throws::Discard), // sub_140a73da0
    Replacement::draft_throws(0x140a73e80, "unknown_140a73e80", fn_140a73e80 as *const (), Throws::Discard), // sub_140a73e80
    Replacement::draft_throws(0x140a73f60, "unknown_140a73f60", fn_140a73f60 as *const (), Throws::Caught), // sub_140a73f60
    Replacement::draft(0x140a741d0, "unknown_140a741d0", fn_140a741d0 as *const ()), // sub_140a741d0
    Replacement::draft_throws(0x140a74370, "unknown_140a74370", fn_140a74370 as *const (), Throws::Caught), // sub_140a74370
    Replacement::draft_throws(0x140a74540, "unknown_140a74540", fn_140a74540 as *const (), Throws::Caught), // sub_140a74540
    Replacement::draft(0x140a74850, "unknown_140a74850", fn_140a74850 as *const ()), // sub_140a74850
    Replacement::draft_throws(0x140a748d0, "unknown_140a748d0", fn_140a748d0 as *const (), Throws::Caught), // sub_140a748d0
    Replacement::draft_throws(0x140a74d40, "unknown_140a74d40", fn_140a74d40 as *const (), Throws::Caught), // sub_140a74d40
    Replacement::draft_throws(0x140a75170, "unknown_140a75170", fn_140a75170 as *const (), Throws::Caught), // sub_140a75170
    Replacement::draft(0x140a754c0, "unknown_140a754c0", fn_140a754c0 as *const ()), // sub_140a754c0
    Replacement::draft(0x140a755b0, "unknown_140a755b0", fn_140a755b0 as *const ()), // sub_140a755b0
    Replacement::draft_throws(0x140a756b0, "unknown_140a756b0", fn_140a756b0 as *const (), Throws::Caught), // sub_140a756b0
    Replacement::draft_throws(0x140a75790, "unknown_140a75790", fn_140a75790 as *const (), Throws::Caught), // sub_140a75790
    Replacement::draft_throws(0x140a75d60, "unknown_140a75d60", fn_140a75d60 as *const (), Throws::Caught), // sub_140a75d60
    Replacement::draft_throws(0x140a762f0, "unknown_140a762f0", fn_140a762f0 as *const (), Throws::Caught), // sub_140a762f0
    Replacement::draft_throws(0x140a76640, "unknown_140a76640", fn_140a76640 as *const (), Throws::Caught), // sub_140a76640
    Replacement::draft_throws(0x140a76810, "unknown_140a76810", fn_140a76810 as *const (), Throws::Caught), // sub_140a76810
    Replacement::draft_throws(0x140a76c80, "unknown_140a76c80", fn_140a76c80 as *const (), Throws::Caught), // sub_140a76c80
    Replacement::draft_throws(0x140a770f0, "unknown_140a770f0", fn_140a770f0 as *const (), Throws::Caught), // sub_140a770f0
    Replacement::draft_throws(0x140a77560, "unknown_140a77560", fn_140a77560 as *const (), Throws::Caught), // sub_140a77560
    Replacement::draft_throws(0x140a77990, "unknown_140a77990", fn_140a77990 as *const (), Throws::Caught), // sub_140a77990
    Replacement::draft_throws(0x140a77dc0, "unknown_140a77dc0", fn_140a77dc0 as *const (), Throws::Caught), // sub_140a77dc0
    Replacement::draft_throws(0x140a781f0, "unknown_140a781f0", fn_140a781f0 as *const (), Throws::Caught), // sub_140a781f0
    Replacement::draft_throws(0x140a78540, "unknown_140a78540", fn_140a78540 as *const (), Throws::Caught), // sub_140a78540
    Replacement::draft_throws(0x140a78890, "unknown_140a78890", fn_140a78890 as *const (), Throws::Caught), // sub_140a78890
    Replacement::draft_throws(0x140a78be0, "unknown_140a78be0", fn_140a78be0 as *const (), Throws::Discard), // sub_140a78be0
    Replacement::draft(0x140a78cd0, "unknown_140a78cd0", fn_140a78cd0 as *const ()), // sub_140a78cd0
    Replacement::draft_throws(0x140a78da0, "unknown_140a78da0", fn_140a78da0 as *const (), Throws::Discard), // sub_140a78da0
    Replacement::draft_throws(0x140a78e30, "unknown_140a78e30", fn_140a78e30 as *const (), Throws::Discard), // sub_140a78e30
    Replacement::draft_throws(0x140a78f30, "unknown_140a78f30", fn_140a78f30 as *const (), Throws::Discard), // sub_140a78f30
    Replacement::draft_throws(0x140a79000, "unknown_140a79000", fn_140a79000 as *const (), Throws::Caught), // sub_140a79000
    Replacement::draft_throws(0x140a795e0, "unknown_140a795e0", fn_140a795e0 as *const (), Throws::Caught), // sub_140a795e0
    Replacement::draft_throws(0x140a79b90, "unknown_140a79b90", fn_140a79b90 as *const (), Throws::Caught), // sub_140a79b90
    Replacement::draft(0x140a79ee0, "unknown_140a79ee0", fn_140a79ee0 as *const ()), // sub_140a79ee0
    Replacement::draft(0x140a79ff0, "unknown_140a79ff0", fn_140a79ff0 as *const ()), // sub_140a79ff0
    Replacement::draft_throws(0x140a7a110, "unknown_140a7a110", fn_140a7a110 as *const (), Throws::Discard), // sub_140a7a110
    Replacement::draft_throws(0x140a7a210, "unknown_140a7a210", fn_140a7a210 as *const (), Throws::Discard), // sub_140a7a210
    Replacement::draft(0x140a7a310, "unknown_140a7a310", fn_140a7a310 as *const ()), // sub_140a7a310
    Replacement::draft(0x140a7a3b0, "unknown_140a7a3b0", tpl_13000::<0x14387e8f0, 0x14243b0b0, 0x14089d260, 0x8, 0x1408ede60, 0x338, 0x10, 0x20, 0x58, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140a7a3b0
    Replacement::draft_throws(0x140a7a450, "unknown_140a7a450", fn_140a7a450 as *const (), Throws::Discard), // sub_140a7a450
    Replacement::draft(0x140a7a550, "unknown_140a7a550", fn_140a7a550 as *const ()), // sub_140a7a550
    Replacement::draft_throws(0x140a7a5c0, "unknown_140a7a5c0", fn_140a7a5c0 as *const (), Throws::Caught), // sub_140a7a5c0
    Replacement::draft_throws(0x140a7ab90, "unknown_140a7ab90", fn_140a7ab90 as *const (), Throws::Caught), // sub_140a7ab90
    Replacement::draft_throws(0x140a7b120, "unknown_140a7b120", fn_140a7b120 as *const (), Throws::Caught), // sub_140a7b120
    Replacement::draft(0x140a7b470, "unknown_140a7b470", fn_140a7b470 as *const ()), // sub_140a7b470
    Replacement::draft(0x140a7b500, "unknown_140a7b500", tpl_13000::<0x14387e8f0, 0x14243b0b0, 0x14089d6c0, 0x8, 0x1408efaa0, 0x338, 0x10, 0xc0, 0x58, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140a7b500
    Replacement::draft_throws(0x140a7b5a0, "unknown_140a7b5a0", fn_140a7b5a0 as *const (), Throws::Discard), // sub_140a7b5a0
    Replacement::draft_throws(0x140a7b630, "unknown_140a7b630", fn_140a7b630 as *const (), Throws::Caught), // sub_140a7b630
    Replacement::draft_throws(0x140a7b770, "unknown_140a7b770", fn_140a7b770 as *const (), Throws::Caught), // sub_140a7b770
    Replacement::draft_throws(0x140a7bd30, "unknown_140a7bd30", fn_140a7bd30 as *const (), Throws::Caught), // sub_140a7bd30
    Replacement::draft_throws(0x140a7be70, "unknown_140a7be70", fn_140a7be70 as *const (), Throws::Caught), // sub_140a7be70
    Replacement::draft_throws(0x140a7c400, "unknown_140a7c400", fn_140a7c400 as *const (), Throws::Caught), // sub_140a7c400
    Replacement::draft_throws(0x140a7c540, "unknown_140a7c540", fn_140a7c540 as *const (), Throws::Caught), // sub_140a7c540
    Replacement::draft_throws(0x140a7c890, "unknown_140a7c890", fn_140a7c890 as *const (), Throws::Caught), // sub_140a7c890
    Replacement::draft_throws(0x140a7ca20, "unknown_140a7ca20", fn_140a7ca20 as *const (), Throws::Caught), // sub_140a7ca20
    Replacement::draft_throws(0x140a7cbb0, "unknown_140a7cbb0", fn_140a7cbb0 as *const (), Throws::Caught), // sub_140a7cbb0
    Replacement::draft_throws(0x140a7cd40, "unknown_140a7cd40", fn_140a7cd40 as *const (), Throws::Caught), // sub_140a7cd40
    Replacement::draft(0x140a7ce40, "unknown_140a7ce40", fn_140a7ce40 as *const ()), // sub_140a7ce40
    Replacement::draft(0x140a7cf70, "unknown_140a7cf70", fn_140a7cf70 as *const ()), // sub_140a7cf70
    Replacement::draft_throws(0x140a7d040, "unknown_140a7d040", fn_140a7d040 as *const (), Throws::Discard), // sub_140a7d040
    Replacement::draft_throws(0x140a7d0f0, "unknown_140a7d0f0", fn_140a7d0f0 as *const (), Throws::Caught), // sub_140a7d0f0
    Replacement::draft_throws(0x140a7d560, "unknown_140a7d560", fn_140a7d560 as *const (), Throws::Caught), // sub_140a7d560
    Replacement::draft_throws(0x140a7d990, "unknown_140a7d990", fn_140a7d990 as *const (), Throws::Caught), // sub_140a7d990
    Replacement::draft(0x140a7dce0, "unknown_140a7dce0", fn_140a7dce0 as *const ()), // sub_140a7dce0
    Replacement::draft(0x140a7dd90, "unknown_140a7dd90", fn_140a7dd90 as *const ()), // sub_140a7dd90
    Replacement::draft_throws(0x140a7de50, "unknown_140a7de50", fn_140a7de50 as *const (), Throws::Discard), // sub_140a7de50
    Replacement::draft_throws(0x140a7df00, "unknown_140a7df00", fn_140a7df00 as *const (), Throws::Caught), // sub_140a7df00
    Replacement::draft_throws(0x140a7e370, "unknown_140a7e370", fn_140a7e370 as *const (), Throws::Caught), // sub_140a7e370
    Replacement::draft_throws(0x140a7e7e0, "unknown_140a7e7e0", fn_140a7e7e0 as *const (), Throws::Caught), // sub_140a7e7e0
    Replacement::draft_throws(0x140a7ed60, "unknown_140a7ed60", fn_140a7ed60 as *const (), Throws::Caught), // sub_140a7ed60
    Replacement::draft_throws(0x140a7f190, "unknown_140a7f190", fn_140a7f190 as *const (), Throws::Caught), // sub_140a7f190
    Replacement::draft_throws(0x140a7f5c0, "unknown_140a7f5c0", fn_140a7f5c0 as *const (), Throws::Caught), // sub_140a7f5c0
    Replacement::draft_throws(0x140a7fb10, "unknown_140a7fb10", fn_140a7fb10 as *const (), Throws::Caught), // sub_140a7fb10
    Replacement::draft_throws(0x140a7fe60, "unknown_140a7fe60", fn_140a7fe60 as *const (), Throws::Caught), // sub_140a7fe60
    Replacement::draft_throws(0x140a801b0, "unknown_140a801b0", fn_140a801b0 as *const (), Throws::Caught), // sub_140a801b0
    Replacement::draft(0x140a80500, "unknown_140a80500", fn_140a80500 as *const ()), // sub_140a80500
    Replacement::draft(0x140a80670, "unknown_140a80670", fn_140a80670 as *const ()), // sub_140a80670
    Replacement::draft_throws(0x140a807c0, "unknown_140a807c0", fn_140a807c0 as *const (), Throws::Caught), // sub_140a807c0
    Replacement::draft_throws(0x140a808f0, "unknown_140a808f0", fn_140a808f0 as *const (), Throws::Caught), // sub_140a808f0
    Replacement::draft_throws(0x140a80d60, "unknown_140a80d60", fn_140a80d60 as *const (), Throws::Caught), // sub_140a80d60
    Replacement::draft_throws(0x140a81190, "unknown_140a81190", fn_140a81190 as *const (), Throws::Caught), // sub_140a81190
    Replacement::draft_throws(0x140a814e0, "unknown_140a814e0", fn_140a814e0 as *const (), Throws::Caught), // sub_140a814e0
    Replacement::draft_throws(0x140a81ab0, "unknown_140a81ab0", fn_140a81ab0 as *const (), Throws::Caught), // sub_140a81ab0
    Replacement::draft_throws(0x140a82050, "unknown_140a82050", fn_140a82050 as *const (), Throws::Caught), // sub_140a82050
    Replacement::draft_throws(0x140a823a0, "unknown_140a823a0", fn_140a823a0 as *const (), Throws::Caught), // sub_140a823a0
    Replacement::draft_throws(0x140a82810, "unknown_140a82810", fn_140a82810 as *const (), Throws::Caught), // sub_140a82810
    Replacement::draft_throws(0x140a82c40, "unknown_140a82c40", fn_140a82c40 as *const (), Throws::Caught), // sub_140a82c40
    Replacement::draft_throws(0x140a82f90, "unknown_140a82f90", fn_140a82f90 as *const (), Throws::Caught), // sub_140a82f90
    Replacement::draft_throws(0x140a83500, "unknown_140a83500", fn_140a83500 as *const (), Throws::Caught), // sub_140a83500
    Replacement::draft_throws(0x140a83a30, "unknown_140a83a30", fn_140a83a30 as *const (), Throws::Caught), // sub_140a83a30
    Replacement::draft_throws(0x140a83d80, "unknown_140a83d80", fn_140a83d80 as *const (), Throws::Caught), // sub_140a83d80
    Replacement::draft_throws(0x140a84410, "unknown_140a84410", fn_140a84410 as *const (), Throws::Caught), // sub_140a84410
    Replacement::draft_throws(0x140a849a0, "unknown_140a849a0", fn_140a849a0 as *const (), Throws::Caught), // sub_140a849a0
    Replacement::draft_throws(0x140a84f70, "unknown_140a84f70", fn_140a84f70 as *const (), Throws::Caught), // sub_140a84f70
    Replacement::draft_throws(0x140a855d0, "unknown_140a855d0", fn_140a855d0 as *const (), Throws::Caught), // sub_140a855d0
    Replacement::draft_throws(0x140a85b30, "unknown_140a85b30", fn_140a85b30 as *const (), Throws::Caught), // sub_140a85b30
    Replacement::draft_throws(0x140a860c0, "unknown_140a860c0", fn_140a860c0 as *const (), Throws::Caught), // sub_140a860c0
    Replacement::draft_throws(0x140a86500, "unknown_140a86500", fn_140a86500 as *const (), Throws::Caught), // sub_140a86500
    Replacement::draft_throws(0x140a86850, "unknown_140a86850", fn_140a86850 as *const (), Throws::Caught), // sub_140a86850
    Replacement::draft_throws(0x140a86ba0, "unknown_140a86ba0", fn_140a86ba0 as *const (), Throws::Caught), // sub_140a86ba0
    Replacement::draft_throws(0x140a87010, "unknown_140a87010", fn_140a87010 as *const (), Throws::Caught), // sub_140a87010
    Replacement::draft_throws(0x140a87440, "unknown_140a87440", fn_140a87440 as *const (), Throws::Caught), // sub_140a87440
    Replacement::draft_throws(0x140a87790, "unknown_140a87790", fn_140a87790 as *const (), Throws::Caught), // sub_140a87790
    Replacement::draft_throws(0x140a87c00, "unknown_140a87c00", fn_140a87c00 as *const (), Throws::Caught), // sub_140a87c00
    Replacement::draft_throws(0x140a88030, "unknown_140a88030", fn_140a88030 as *const (), Throws::Caught), // sub_140a88030
    Replacement::draft_throws(0x140a88380, "unknown_140a88380", fn_140a88380 as *const (), Throws::Caught), // sub_140a88380
    Replacement::draft_throws(0x140a889f0, "unknown_140a889f0", fn_140a889f0 as *const (), Throws::Caught), // sub_140a889f0
    Replacement::draft_throws(0x140a89030, "unknown_140a89030", fn_140a89030 as *const (), Throws::Caught), // sub_140a89030
    Replacement::draft_throws(0x140a89380, "unknown_140a89380", fn_140a89380 as *const (), Throws::Caught), // sub_140a89380
    Replacement::draft_throws(0x140a89b90, "unknown_140a89b90", fn_140a89b90 as *const (), Throws::Caught), // sub_140a89b90
    Replacement::draft_throws(0x140a8a330, "unknown_140a8a330", fn_140a8a330 as *const (), Throws::Caught), // sub_140a8a330
    Replacement::draft(0x140a8a870, "unknown_140a8a870", fn_140a8a870 as *const ()), // sub_140a8a870
    Replacement::draft_throws(0x140a8a9f0, "unknown_140a8a9f0", fn_140a8a9f0 as *const (), Throws::Discard), // sub_140a8a9f0
    Replacement::draft_throws(0x140a8aae0, "unknown_140a8aae0", fn_140a8aae0 as *const (), Throws::Discard), // sub_140a8aae0
    Replacement::draft_throws(0x140a8ac90, "unknown_140a8ac90", fn_140a8ac90 as *const (), Throws::Discard), // sub_140a8ac90
    Replacement::draft_throws(0x140a8ae80, "unknown_140a8ae80", fn_140a8ae80 as *const (), Throws::Discard), // sub_140a8ae80
    Replacement::draft_throws(0x140a8b070, "unknown_140a8b070", fn_140a8b070 as *const (), Throws::Caught), // sub_140a8b070
    Replacement::draft_throws(0x140a8b1d0, "unknown_140a8b1d0", fn_140a8b1d0 as *const (), Throws::Caught), // sub_140a8b1d0
    Replacement::draft_throws(0x140a8b4d0, "unknown_140a8b4d0", tpl_65672::<0x142442de0, 0x143a3dab0, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140a8b4d0
    Replacement::draft_throws(0x140a8b6f0, "unknown_140a8b6f0", fn_140a8b6f0 as *const (), Throws::Caught), // sub_140a8b6f0
    Replacement::draft_throws(0x140a8b9e0, "unknown_140a8b9e0", fn_140a8b9e0 as *const (), Throws::Discard), // sub_140a8b9e0
    Replacement::draft_throws(0x140a8baf0, "unknown_140a8baf0", fn_140a8baf0 as *const (), Throws::Discard), // sub_140a8baf0
    Replacement::draft_throws(0x140a8bc10, "unknown_140a8bc10", fn_140a8bc10 as *const (), Throws::Caught), // sub_140a8bc10
    Replacement::draft_throws(0x140a8c1c0, "unknown_140a8c1c0", fn_140a8c1c0 as *const (), Throws::Caught), // sub_140a8c1c0
    Replacement::draft_throws(0x140a8c740, "unknown_140a8c740", fn_140a8c740 as *const (), Throws::Caught), // sub_140a8c740
    Replacement::draft_throws(0x140a8cb90, "unknown_140a8cb90", fn_140a8cb90 as *const (), Throws::Discard), // sub_140a8cb90
    Replacement::draft_throws(0x140a8cca0, "unknown_140a8cca0", fn_140a8cca0 as *const (), Throws::Caught), // sub_140a8cca0
    Replacement::draft(0x140a8ce30, "unknown_140a8ce30", tpl_3643::<0x3, 0x4, 0x8, 0x14240aa00, 0x28> as *const ()), // sub_140a8ce30
    Replacement::draft(0x140a8ceb0, "unknown_140a8ceb0", fn_140a8ceb0 as *const ()), // sub_140a8ceb0
    Replacement::draft(0x140a8cf60, "unknown_140a8cf60", fn_140a8cf60 as *const ()), // sub_140a8cf60
    Replacement::draft(0x140a8d050, "unknown_140a8d050", fn_140a8d050 as *const ()), // sub_140a8d050
    Replacement::draft(0x140a8d150, "unknown_140a8d150", fn_140a8d150 as *const ()), // sub_140a8d150
    Replacement::draft_throws(0x140a8d1d0, "unknown_140a8d1d0", fn_140a8d1d0 as *const (), Throws::Discard), // sub_140a8d1d0
    Replacement::draft(0x140a8d240, "unknown_140a8d240", fn_140a8d240 as *const ()), // sub_140a8d240
    Replacement::draft(0x140a8d2b0, "unknown_140a8d2b0", fn_140a8d2b0 as *const ()), // sub_140a8d2b0
    Replacement::draft(0x140a8d330, "unknown_140a8d330", fn_140a8d330 as *const ()), // sub_140a8d330
    Replacement::draft_throws(0x140a8d5a0, "unknown_140a8d5a0", fn_140a8d5a0 as *const (), Throws::Caught), // sub_140a8d5a0
    Replacement::draft_throws(0x140a8d970, "unknown_140a8d970", fn_140a8d970 as *const (), Throws::Caught), // sub_140a8d970
    Replacement::draft_throws(0x140a8dce0, "unknown_140a8dce0", fn_140a8dce0 as *const (), Throws::Caught), // sub_140a8dce0
    Replacement::draft_throws(0x140a8e040, "unknown_140a8e040", fn_140a8e040 as *const (), Throws::Caught), // sub_140a8e040
    Replacement::draft_throws(0x140a8e4e0, "unknown_140a8e4e0", fn_140a8e4e0 as *const (), Throws::Caught), // sub_140a8e4e0
    Replacement::draft_throws(0x140a8eb10, "unknown_140a8eb10", fn_140a8eb10 as *const (), Throws::Caught), // sub_140a8eb10
    Replacement::draft_throws(0x140a8f210, "unknown_140a8f210", fn_140a8f210 as *const (), Throws::Caught), // sub_140a8f210
    Replacement::draft_throws(0x140a8f640, "unknown_140a8f640", fn_140a8f640 as *const (), Throws::Caught), // sub_140a8f640
    Replacement::draft_throws(0x140a8fbd0, "unknown_140a8fbd0", fn_140a8fbd0 as *const (), Throws::Caught), // sub_140a8fbd0
    Replacement::draft_throws(0x140a90210, "unknown_140a90210", fn_140a90210 as *const (), Throws::Caught), // sub_140a90210
    Replacement::draft_throws(0x140a90740, "unknown_140a90740", fn_140a90740 as *const (), Throws::Caught), // sub_140a90740
    Replacement::draft_throws(0x140a90d90, "unknown_140a90d90", fn_140a90d90 as *const (), Throws::Caught), // sub_140a90d90
    Replacement::draft_throws(0x140a913a0, "unknown_140a913a0", fn_140a913a0 as *const (), Throws::Caught), // sub_140a913a0
    Replacement::draft_throws(0x140a91a30, "unknown_140a91a30", fn_140a91a30 as *const (), Throws::Caught), // sub_140a91a30
    Replacement::draft_throws(0x140a92030, "unknown_140a92030", fn_140a92030 as *const (), Throws::Caught), // sub_140a92030
    Replacement::draft_throws(0x140a92600, "unknown_140a92600", fn_140a92600 as *const (), Throws::Caught), // sub_140a92600
    Replacement::draft_throws(0x140a929b0, "unknown_140a929b0", fn_140a929b0 as *const (), Throws::Caught), // sub_140a929b0
    Replacement::draft_throws(0x140a92d00, "unknown_140a92d00", fn_140a92d00 as *const (), Throws::Caught), // sub_140a92d00
    Replacement::draft_throws(0x140a93040, "unknown_140a93040", fn_140a93040 as *const (), Throws::Caught), // sub_140a93040
    Replacement::draft_throws(0x140a93240, "unknown_140a93240", fn_140a93240 as *const (), Throws::Caught), // sub_140a93240
    Replacement::draft_throws(0x140a933b0, "unknown_140a933b0", fn_140a933b0 as *const (), Throws::Caught), // sub_140a933b0
    Replacement::draft_throws(0x140a93520, "unknown_140a93520", fn_140a93520 as *const (), Throws::Caught), // sub_140a93520
    Replacement::draft_throws(0x140a93690, "unknown_140a93690", fn_140a93690 as *const (), Throws::Caught), // sub_140a93690
    Replacement::draft_throws(0x140a93910, "unknown_140a93910", fn_140a93910 as *const (), Throws::Caught), // sub_140a93910
    Replacement::draft_throws(0x140a93b50, "unknown_140a93b50", fn_140a93b50 as *const (), Throws::Caught), // sub_140a93b50
    Replacement::draft_throws(0x140a93d90, "unknown_140a93d90", fn_140a93d90 as *const (), Throws::Caught), // sub_140a93d90
    Replacement::draft_throws(0x140a94850, "unknown_140a94850", fn_140a94850 as *const (), Throws::Caught), // sub_140a94850
    Replacement::draft_throws(0x140a952e0, "unknown_140a952e0", fn_140a952e0 as *const (), Throws::Caught), // sub_140a952e0
    Replacement::draft_throws(0x140a95d60, "unknown_140a95d60", fn_140a95d60 as *const (), Throws::Caught), // sub_140a95d60
    Replacement::draft_throws(0x140a95ea0, "unknown_140a95ea0", fn_140a95ea0 as *const (), Throws::Caught), // sub_140a95ea0
    Replacement::draft(0x140a96480, "content_140a96480", fn_140a96480 as *const ()), // content::MovementModelPropertiesBase::content::MovementModelPropertiesBase::vfunc_0
    Replacement::draft(0x140a96620, "content_140a96620", fn_140a96620 as *const ()), // sub_140a96620
    Replacement::draft(0x140a966c0, "content_140a966c0", fn_140a966c0 as *const ()), // sub_140a966c0
    Replacement::draft_throws(0x140a968c0, "content_140a968c0", fn_140a968c0 as *const (), Throws::Discard), // sub_140a968c0
    Replacement::draft(0x140a96a90, "content_140a96a90", fn_140a96a90 as *const ()), // sub_140a96a90
    Replacement::draft_throws(0x140a96c00, "content_140a96c00", fn_140a96c00 as *const (), Throws::Discard), // sub_140a96c00
    Replacement::draft(0x140a96d90, "content_140a96d90", fn_140a96d90 as *const ()), // sub_140a96d90
    Replacement::draft_throws(0x140a96e20, "content_140a96e20", fn_140a96e20 as *const (), Throws::Caught), // sub_140a96e20
    Replacement::draft_throws(0x140a970c0, "content_140a970c0", fn_140a970c0 as *const (), Throws::Caught), // sub_140a970c0
    Replacement::draft_throws(0x140a97320, "content_140a97320", fn_140a97320 as *const (), Throws::Caught), // sub_140a97320
    Replacement::draft(0x140a97580, "content_140a97580", fn_140a97580 as *const ()), // sub_140a97580
    Replacement::draft(0x140a976a0, "content_140a976a0", fn_140a976a0 as *const ()), // sub_140a976a0
    Replacement::draft(0x140a97810, "content_140a97810", fn_140a97810 as *const ()), // sub_140a97810
    Replacement::draft_throws(0x140a97990, "content_140a97990", fn_140a97990 as *const (), Throws::Discard), // sub_140a97990
    Replacement::draft_throws(0x140a97b10, "content_140a97b10", fn_140a97b10 as *const (), Throws::Discard), // sub_140a97b10
    Replacement::draft(0x140a97c50, "content_140a97c50", fn_140a97c50 as *const ()), // sub_140a97c50
    Replacement::draft(0x140a97d00, "content_140a97d00", fn_140a97d00 as *const ()), // sub_140a97d00
    Replacement::draft(0x140a97e20, "content_140a97e20", fn_140a97e20 as *const ()), // sub_140a97e20
    Replacement::draft_throws(0x140a97fc0, "content_140a97fc0", fn_140a97fc0 as *const (), Throws::Discard), // sub_140a97fc0
    Replacement::draft(0x140a98140, "content_140a98140", fn_140a98140 as *const ()), // sub_140a98140
    Replacement::draft_throws(0x140a98230, "content_140a98230", fn_140a98230 as *const (), Throws::Discard), // sub_140a98230
    Replacement::draft(0x140a983e0, "content_140a983e0", fn_140a983e0 as *const ()), // sub_140a983e0
    Replacement::draft_throws(0x140a984a0, "content_140a984a0", fn_140a984a0 as *const (), Throws::Caught), // sub_140a984a0
    Replacement::draft_throws(0x140a987e0, "content_140a987e0", fn_140a987e0 as *const (), Throws::Caught), // sub_140a987e0
    Replacement::draft_throws(0x140a98af0, "content_140a98af0", fn_140a98af0 as *const (), Throws::Caught), // sub_140a98af0
    Replacement::draft(0x140a98e00, "content_140a98e00", fn_140a98e00 as *const ()), // sub_140a98e00
    Replacement::draft(0x140a98ee0, "content_140a98ee0", fn_140a98ee0 as *const ()), // sub_140a98ee0
    Replacement::draft(0x140a98fd0, "content_140a98fd0", fn_140a98fd0 as *const ()), // sub_140a98fd0
    Replacement::draft_throws(0x140a990d0, "content_140a990d0", fn_140a990d0 as *const (), Throws::Discard), // sub_140a990d0
    Replacement::draft_throws(0x140a991e0, "content_140a991e0", fn_140a991e0 as *const (), Throws::Discard), // sub_140a991e0
    Replacement::draft_throws(0x140a993b0, "content_140a993b0", fn_140a993b0 as *const (), Throws::Discard), // sub_140a993b0
    Replacement::draft(0x140a994e0, "content_140a994e0", fn_140a994e0 as *const ()), // sub_140a994e0
    Replacement::draft(0x140a995d0, "content_140a995d0", fn_140a995d0 as *const ()), // sub_140a995d0
    Replacement::draft(0x140a996c0, "content_140a996c0", tpl_13114::<0x14387e5f0, 0x14243b0b0, 0x140a96620, 0x48, 0x1408375a0, 0x78, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a996c0
    Replacement::draft(0x140a99740, "content_140a99740", fn_140a99740 as *const ()), // sub_140a99740
    Replacement::draft_throws(0x140a99840, "content_140a99840", fn_140a99840 as *const (), Throws::Discard), // sub_140a99840
    Replacement::draft(0x140a99950, "content_140a99950", fn_140a99950 as *const ()), // sub_140a99950
    Replacement::draft_throws(0x140a99a20, "content_140a99a20", fn_140a99a20 as *const (), Throws::Discard), // sub_140a99a20
    Replacement::draft(0x140a99b30, "content_140a99b30", fn_140a99b30 as *const ()), // sub_140a99b30
    Replacement::draft_throws(0x140a99ca0, "content_140a99ca0", fn_140a99ca0 as *const (), Throws::Caught), // sub_140a99ca0
    Replacement::draft_throws(0x140a99f40, "content_140a99f40", fn_140a99f40 as *const (), Throws::Caught), // sub_140a99f40
    Replacement::draft_throws(0x140a9a1b0, "content_140a9a1b0", fn_140a9a1b0 as *const (), Throws::Caught), // sub_140a9a1b0
    Replacement::draft(0x140a9a410, "content_140a9a410", fn_140a9a410 as *const ()), // sub_140a9a410
    Replacement::draft(0x140a9a4e0, "content_140a9a4e0", fn_140a9a4e0 as *const ()), // sub_140a9a4e0
    Replacement::draft(0x140a9a5a0, "content_140a9a5a0", fn_140a9a5a0 as *const ()), // sub_140a9a5a0
    Replacement::draft_throws(0x140a9a680, "content_140a9a680", fn_140a9a680 as *const (), Throws::Discard), // sub_140a9a680
    Replacement::draft_throws(0x140a9a730, "content_140a9a730", fn_140a9a730 as *const (), Throws::Discard), // sub_140a9a730
    Replacement::draft_throws(0x140a9a870, "content_140a9a870", tpl_13127::<0x14384e890, 0x1432a8c88, 0x30, 0x1432a8c70, 0x1, 0x1401de410, 0x38, 0x1432a8d38, 0x14020f270, 0x3c, 0x1432a8d28, 0x1402c61c0> as *const (), Throws::Discard), // sub_140a9a870
    Replacement::draft(0x140a9a910, "content_140a9a910", fn_140a9a910 as *const ()), // sub_140a9a910
    Replacement::draft(0x140a9aa00, "content_140a9aa00", fn_140a9aa00 as *const ()), // sub_140a9aa00
    Replacement::draft(0x140a9aae0, "content_140a9aae0", tpl_13114::<0x14387e608, 0x14243b0b0, 0x140a96620, 0x40, 0x1408375a0, 0x70, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9aae0
    Replacement::draft(0x140a9ab60, "content_140a9ab60", fn_140a9ab60 as *const ()), // sub_140a9ab60
    Replacement::draft_throws(0x140a9ac30, "content_140a9ac30", fn_140a9ac30 as *const (), Throws::Discard), // sub_140a9ac30
    Replacement::draft(0x140a9ace0, "content_140a9ace0", fn_140a9ace0 as *const ()), // sub_140a9ace0
    Replacement::draft_throws(0x140a9ad70, "content_140a9ad70", fn_140a9ad70 as *const (), Throws::Discard), // sub_140a9ad70
    Replacement::draft(0x140a9ae30, "content_140a9ae30", fn_140a9ae30 as *const ()), // sub_140a9ae30
    Replacement::draft_throws(0x140a9af90, "content_140a9af90", fn_140a9af90 as *const (), Throws::Caught), // sub_140a9af90
    Replacement::draft_throws(0x140a9b0a0, "content_140a9b0a0", fn_140a9b0a0 as *const (), Throws::Caught), // sub_140a9b0a0
    Replacement::draft_throws(0x140a9b1b0, "content_140a9b1b0", fn_140a9b1b0 as *const (), Throws::Caught), // sub_140a9b1b0
    Replacement::draft(0x140a9b2c0, "content_140a9b2c0", fn_140a9b2c0 as *const ()), // sub_140a9b2c0
    Replacement::draft(0x140a9b370, "content_140a9b370", fn_140a9b370 as *const ()), // sub_140a9b370
    Replacement::draft(0x140a9b400, "content_140a9b400", fn_140a9b400 as *const ()), // sub_140a9b400
    Replacement::draft_throws(0x140a9b4a0, "content_140a9b4a0", fn_140a9b4a0 as *const (), Throws::Discard), // sub_140a9b4a0
    Replacement::draft_throws(0x140a9b530, "content_140a9b530", fn_140a9b530 as *const (), Throws::Discard), // sub_140a9b530
    Replacement::draft(0x140a9b630, "content_140a9b630", tpl_13141::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410, 0x38, 0x1432a8d38, 0x14020f270> as *const ()), // sub_140a9b630
    Replacement::draft(0x140a9b6a0, "content_140a9b6a0", fn_140a9b6a0 as *const ()), // sub_140a9b6a0
    Replacement::draft(0x140a9b750, "content_140a9b750", fn_140a9b750 as *const ()), // sub_140a9b750
    Replacement::draft(0x140a9b810, "content_140a9b810", tpl_7312::<0x14387e5e0, 0x14243b0b0, 0x140a96620, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9b810
    Replacement::draft(0x140a9b880, "content_140a9b880", fn_140a9b880 as *const ()), // sub_140a9b880
    Replacement::draft_throws(0x140a9b980, "content_140a9b980", fn_140a9b980 as *const (), Throws::Discard), // sub_140a9b980
    Replacement::draft(0x140a9baa0, "content_140a9baa0", fn_140a9baa0 as *const ()), // sub_140a9baa0
    Replacement::draft_throws(0x140a9bb80, "content_140a9bb80", fn_140a9bb80 as *const (), Throws::Discard), // sub_140a9bb80
    Replacement::draft(0x140a9bc80, "content_140a9bc80", fn_140a9bc80 as *const ()), // sub_140a9bc80
    Replacement::draft_throws(0x140a9bd20, "content_140a9bd20", fn_140a9bd20 as *const (), Throws::Caught), // sub_140a9bd20
    Replacement::draft_throws(0x140a9bfa0, "content_140a9bfa0", fn_140a9bfa0 as *const (), Throws::Caught), // sub_140a9bfa0
    Replacement::draft_throws(0x140a9c1f0, "content_140a9c1f0", fn_140a9c1f0 as *const (), Throws::Caught), // sub_140a9c1f0
    Replacement::draft(0x140a9c440, "content_140a9c440", fn_140a9c440 as *const ()), // sub_140a9c440
    Replacement::draft(0x140a9c4e0, "content_140a9c4e0", fn_140a9c4e0 as *const ()), // sub_140a9c4e0
    Replacement::draft(0x140a9c5b0, "content_140a9c5b0", fn_140a9c5b0 as *const ()), // sub_140a9c5b0
    Replacement::draft_throws(0x140a9c690, "content_140a9c690", fn_140a9c690 as *const (), Throws::Discard), // sub_140a9c690
    Replacement::draft_throws(0x140a9c750, "content_140a9c750", fn_140a9c750 as *const (), Throws::Discard), // sub_140a9c750
    Replacement::draft_throws(0x140a9c8a0, "content_140a9c8a0", tpl_13127::<0x14384e890, 0x1432a8c88, 0x30, 0x1432a8c70, 0x1, 0x1401de410, 0x38, 0x1432a8d78, 0x1402c4e80, 0x44, 0x1432a8d88, 0x14020f270> as *const (), Throws::Discard), // sub_140a9c8a0
    Replacement::draft(0x140a9c940, "content_140a9c940", tpl_13156::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1432a7eb0, 0x34, 0x3c, 0x8, 0x20> as *const ()), // sub_140a9c940
    Replacement::draft(0x140a9c9e0, "content_140a9c9e0", tpl_13157::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432a7eb0, 0x4, 0x20, 0x28, 0xfffffffffffffff4> as *const ()), // sub_140a9c9e0
    Replacement::draft(0x140a9cb00, "content_140a9cb00", fn_140a9cb00 as *const ()), // sub_140a9cb00
    Replacement::draft_throws(0x140a9cbb0, "content_140a9cbb0", fn_140a9cbb0 as *const (), Throws::Discard), // sub_140a9cbb0
    Replacement::draft(0x140a9cc40, "content_140a9cc40", fn_140a9cc40 as *const ()), // sub_140a9cc40
    Replacement::draft_throws(0x140a9ccc0, "content_140a9ccc0", tpl_7853::<0x14387e5b0, 0x14387e598, 0x1401e9b60, 0x140a96c00, 0x328, 0x338, 0x120, 0x38, 0x4, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140a9ccc0
    Replacement::draft(0x140a9cd60, "content_140a9cd60", fn_140a9cd60 as *const ()), // sub_140a9cd60
    Replacement::draft_throws(0x140a9cde0, "content_140a9cde0", fn_140a9cde0 as *const (), Throws::Caught), // sub_140a9cde0
    Replacement::draft_throws(0x140a9ceb0, "content_140a9ceb0", fn_140a9ceb0 as *const (), Throws::Caught), // sub_140a9ceb0
    Replacement::draft_throws(0x140a9cf80, "content_140a9cf80", fn_140a9cf80 as *const (), Throws::Caught), // sub_140a9cf80
    Replacement::draft(0x140a9d050, "content_140a9d050", tpl_7855::<0x14387e598, 0x14243b0b0, 0x140a97580, 0x338, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9d050
    Replacement::draft(0x140a9d0d0, "content_140a9d0d0", tpl_7856::<0x14387e5b0, 0x14387e598, 0x1401e9970, 0x140a976a0, 0x338, 0x38, 0xd8, 0x1401e9860> as *const ()), // sub_140a9d0d0
    Replacement::draft(0x140a9d140, "content_140a9d140", tpl_7857::<0x14387e598, 0x14243b0b0, 0x140a97810, 0x338, 0x38, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9d140
    Replacement::draft_throws(0x140a9d1c0, "content_140a9d1c0", tpl_7858::<0x14387e5b0, 0x14387e598, 0x1401e9770, 0x140a97990, 0x338, 0x38, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140a9d1c0
    Replacement::draft_throws(0x140a9d240, "content_140a9d240", tpl_7859::<0x14387e598, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140a97b10, 0x38, 0x1432a8cd0, 0x1, 0x14020f240> as *const (), Throws::Discard), // sub_140a9d240
    Replacement::draft(0x140a9d310, "content_140a9d310", tpl_13141::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410, 0x38, 0x1432a8cd0, 0x14020f270> as *const ()), // sub_140a9d310
    Replacement::draft(0x140a9d380, "content_140a9d380", tpl_13156::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1432a7df8, 0x34, 0x3c, 0x8, 0x20> as *const ()), // sub_140a9d380
    Replacement::draft(0x140a9d420, "content_140a9d420", tpl_13157::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432a7df8, 0x4, 0x20, 0x28, 0xfffffffffffffff4> as *const ()), // sub_140a9d420
    Replacement::draft(0x140a9d540, "content_140a9d540", tpl_8125::<0x14387e590, 0x14243b0b0, 0x140a966c0, 0x38, 0x338, 0x1426d4580, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9d540
    Replacement::draft_throws(0x140a9d5e0, "content_140a9d5e0", tpl_8126::<0x14387e5c0, 0x14387e590, 0x1401e9fa0, 0x140a968c0, 0x328, 0x338, 0x38, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140a9d5e0
    Replacement::draft(0x140a9d670, "content_140a9d670", tpl_7857::<0x14387e590, 0x14243b0b0, 0x140a96a90, 0x338, 0x38, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9d670
    Replacement::draft_throws(0x140a9d6f0, "content_140a9d6f0", tpl_8127::<0x14387e5c0, 0x14387e590, 0x1401e9b60, 0x140a96c00, 0x328, 0x338, 0x120, 0x38, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140a9d6f0
    Replacement::draft(0x140a9d780, "content_140a9d780", fn_140a9d780 as *const ()), // sub_140a9d780
    Replacement::draft_throws(0x140a9d800, "content_140a9d800", fn_140a9d800 as *const (), Throws::Caught), // sub_140a9d800
    Replacement::draft_throws(0x140a9d8d0, "content_140a9d8d0", fn_140a9d8d0 as *const (), Throws::Caught), // sub_140a9d8d0
    Replacement::draft_throws(0x140a9d9a0, "content_140a9d9a0", fn_140a9d9a0 as *const (), Throws::Caught), // sub_140a9d9a0
    Replacement::draft(0x140a9da70, "content_140a9da70", tpl_8129::<0x14387e590, 0x14243b0b0, 0x140a97580, 0x338, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9da70
    Replacement::draft(0x140a9daf0, "content_140a9daf0", tpl_7856::<0x14387e5c0, 0x14387e590, 0x1401e9970, 0x140a976a0, 0x338, 0x38, 0x120, 0x1401e9860> as *const ()), // sub_140a9daf0
    Replacement::draft(0x140a9db60, "content_140a9db60", tpl_7857::<0x14387e590, 0x14243b0b0, 0x140a97810, 0x338, 0x38, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9db60
    Replacement::draft_throws(0x140a9dbe0, "content_140a9dbe0", tpl_8130::<0x14387e5c0, 0x14387e590, 0x1401e9770, 0x140a97990, 0x338, 0x38, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140a9dbe0
    Replacement::draft_throws(0x140a9dc60, "content_140a9dc60", tpl_8131::<0x14387e590, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x140a97b10, 0x38, 0x1432a8fd0, 0x1, 0x14020f0f0> as *const (), Throws::Discard), // sub_140a9dc60
    Replacement::draft(0x140a9dd30, "content_140a9dd30", fn_140a9dd30 as *const ()), // sub_140a9dd30
    Replacement::draft(0x140a9ddf0, "content_140a9ddf0", fn_140a9ddf0 as *const ()), // sub_140a9ddf0
    Replacement::draft(0x140a9df30, "content_140a9df30", tpl_8478::<0x14387e5d0, 0x14243b0b0, 0x140a966c0, 0x3b, 0x40, 0x3c, 0x38, 0x338, 0x1401ee7c0, 0x60, 0x1426d4580, 0x1> as *const ()), // sub_140a9df30
    Replacement::draft_throws(0x140a9e020, "content_140a9e020", fn_140a9e020 as *const (), Throws::Discard), // sub_140a9e020
    Replacement::draft(0x140a9e110, "content_140a9e110", tpl_13165::<0x14387e5d0, 0x14243b0b0, 0x140a96a90, 0x338, 0x38, 0x20, 0x60, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9e110
    Replacement::draft_throws(0x140a9e1a0, "content_140a9e1a0", fn_140a9e1a0 as *const (), Throws::Discard), // sub_140a9e1a0
    Replacement::draft(0x140a9e290, "content_140a9e290", fn_140a9e290 as *const ()), // sub_140a9e290
    Replacement::draft_throws(0x140a9e320, "content_140a9e320", fn_140a9e320 as *const (), Throws::Caught), // sub_140a9e320
    Replacement::draft_throws(0x140a9e5b0, "content_140a9e5b0", fn_140a9e5b0 as *const (), Throws::Caught), // sub_140a9e5b0
    Replacement::draft_throws(0x140a9e820, "content_140a9e820", fn_140a9e820 as *const (), Throws::Caught), // sub_140a9e820
    Replacement::draft(0x140a9ea80, "content_140a9ea80", fn_140a9ea80 as *const ()), // sub_140a9ea80
    Replacement::draft(0x140a9eb30, "content_140a9eb30", tpl_71593::<0x14387e5dc, 0x14387e5d0, 0x1401e9970, 0x140a976a0, 0x338, 0x38, 0xc0, 0x60, 0xe8, 0x1401e9860> as *const ()), // sub_140a9eb30
    Replacement::draft(0x140a9ebc0, "content_140a9ebc0", tpl_13165::<0x14387e5d0, 0x14243b0b0, 0x140a97810, 0x338, 0x38, 0xc0, 0x60, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140a9ebc0
    Replacement::draft_throws(0x140a9ec50, "content_140a9ec50", fn_140a9ec50 as *const (), Throws::Discard), // sub_140a9ec50
    Replacement::draft_throws(0x140a9ece0, "content_140a9ece0", fn_140a9ece0 as *const (), Throws::Discard), // sub_140a9ece0
    Replacement::draft(0x140a9ede0, "content_140a9ede0", fn_140a9ede0 as *const ()), // sub_140a9ede0
    Replacement::draft(0x140a9ee70, "content_140a9ee70", fn_140a9ee70 as *const ()), // sub_140a9ee70
    Replacement::draft(0x140a9ef90, "content_140a9ef90", fn_140a9ef90 as *const ()), // sub_140a9ef90
    Replacement::draft_throws(0x140a9f220, "content_140a9f220", fn_140a9f220 as *const (), Throws::Discard), // sub_140a9f220
    Replacement::draft(0x140a9f420, "content_140a9f420", fn_140a9f420 as *const ()), // sub_140a9f420
    Replacement::draft_throws(0x140a9f540, "content_140a9f540", fn_140a9f540 as *const (), Throws::Discard), // sub_140a9f540
    Replacement::draft(0x140a9f7c0, "content_140a9f7c0", fn_140a9f7c0 as *const ()), // sub_140a9f7c0
    Replacement::draft_throws(0x140a9f880, "content_140a9f880", fn_140a9f880 as *const (), Throws::Caught), // sub_140a9f880
    Replacement::draft_throws(0x140a9fcb0, "content_140a9fcb0", fn_140a9fcb0 as *const (), Throws::Caught), // sub_140a9fcb0
    Replacement::draft_throws(0x140aa0070, "content_140aa0070", fn_140aa0070 as *const (), Throws::Caught), // sub_140aa0070
    Replacement::draft(0x140aa0420, "content_140aa0420", fn_140aa0420 as *const ()), // sub_140aa0420
    Replacement::draft(0x140aa0530, "content_140aa0530", fn_140aa0530 as *const ()), // sub_140aa0530
    Replacement::draft(0x140aa0670, "content_140aa0670", fn_140aa0670 as *const ()), // sub_140aa0670
    Replacement::draft_throws(0x140aa07c0, "content_140aa07c0", fn_140aa07c0 as *const (), Throws::Discard), // sub_140aa07c0
    Replacement::draft_throws(0x140aa0960, "content_140aa0960", fn_140aa0960 as *const (), Throws::Discard), // sub_140aa0960
    Replacement::draft(0x140aa0bf0, "content_140aa0bf0", fn_140aa0bf0 as *const ()), // sub_140aa0bf0
    Replacement::draft(0x140aa0d70, "content_140aa0d70", tpl_13191::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x20, 0x28> as *const ()), // sub_140aa0d70
    Replacement::draft(0x140aa0e00, "content_140aa0e00", fn_140aa0e00 as *const ()), // sub_140aa0e00
    Replacement::draft(0x140aa0eb0, "content_140aa0eb0", fn_140aa0eb0 as *const ()), // sub_140aa0eb0
    Replacement::draft(0x140aa0f80, "content_140aa0f80", fn_140aa0f80 as *const ()), // sub_140aa0f80
    Replacement::draft_throws(0x140aa1090, "content_140aa1090", fn_140aa1090 as *const (), Throws::Discard), // sub_140aa1090
    Replacement::draft(0x140aa1160, "content_140aa1160", fn_140aa1160 as *const ()), // sub_140aa1160
    Replacement::draft_throws(0x140aa12d0, "content_140aa12d0", fn_140aa12d0 as *const (), Throws::Discard), // sub_140aa12d0
    Replacement::draft(0x140aa1350, "content_140aa1350", fn_140aa1350 as *const ()), // sub_140aa1350
    Replacement::draft_throws(0x140aa1420, "content_140aa1420", fn_140aa1420 as *const (), Throws::Caught), // sub_140aa1420
    Replacement::draft_throws(0x140aa1500, "content_140aa1500", fn_140aa1500 as *const (), Throws::Caught), // sub_140aa1500
    Replacement::draft_throws(0x140aa15e0, "content_140aa15e0", fn_140aa15e0 as *const (), Throws::Caught), // sub_140aa15e0
    Replacement::draft(0x140aa16c0, "content_140aa16c0", fn_140aa16c0 as *const ()), // sub_140aa16c0
    Replacement::draft(0x140aa1850, "content_140aa1850", fn_140aa1850 as *const ()), // sub_140aa1850
    Replacement::draft(0x140aa19a0, "content_140aa19a0", fn_140aa19a0 as *const ()), // sub_140aa19a0
    Replacement::draft_throws(0x140aa1b00, "content_140aa1b00", fn_140aa1b00 as *const (), Throws::Discard), // sub_140aa1b00
    Replacement::draft_throws(0x140aa1c50, "content_140aa1c50", fn_140aa1c50 as *const (), Throws::Discard), // sub_140aa1c50
    Replacement::draft(0x140aa1d20, "content_140aa1d20", fn_140aa1d20 as *const ()), // content::MovementModelFalling::content::MovementModelFalling::vfunc_0
    Replacement::draft(0x140aa1d70, "content_140aa1d70", fn_140aa1d70 as *const ()), // content::MovementModelForwardBias::content::MovementModelForwardBias::vfunc_0
    Replacement::draft(0x140aa1e00, "content_140aa1e00", fn_140aa1e00 as *const ()), // content::MovementModelStandard::content::MovementModelStandard::vfunc_0
    Replacement::draft(0x140aa1e90, "content_140aa1e90", fn_140aa1e90 as *const ()), // content::MovementModelImpulseMagnet::content::MovementModelImpulseMagnet::vfunc_0
    Replacement::draft(0x140aa1ee0, "content_140aa1ee0", fn_140aa1ee0 as *const ()), // content::MovementModelSpeedLimiter::content::MovementModelSpeedLimiter::vfunc_0
    Replacement::draft(0x140aa1f30, "content_140aa1f30", fn_140aa1f30 as *const ()), // content::MovementModelAnimatedMovement::content::MovementModelAnimatedMovement::vfunc_0
    Replacement::draft(0x140aa1f80, "content_140aa1f80", fn_140aa1f80 as *const ()), // sub_140aa1f80
    Replacement::draft_throws(0x140aa2070, "content_140aa2070", fn_140aa2070 as *const (), Throws::Caught), // sub_140aa2070
    Replacement::draft_throws(0x140aa2370, "content_140aa2370", fn_140aa2370 as *const (), Throws::Caught), // sub_140aa2370
    Replacement::draft_throws(0x140aa2670, "content_140aa2670", fn_140aa2670 as *const (), Throws::Caught), // sub_140aa2670
    Replacement::draft_throws(0x140aa2970, "content_140aa2970", fn_140aa2970 as *const (), Throws::Caught), // sub_140aa2970
    Replacement::draft_throws(0x140aa2b20, "content_140aa2b20", fn_140aa2b20 as *const (), Throws::Caught), // sub_140aa2b20
    Replacement::draft_throws(0x140aa2d10, "content_140aa2d10", tpl_13213::<0x20, 0x8, 0x140aa3d60> as *const (), Throws::Discard), // sub_140aa2d10
    Replacement::draft(0x140aa2d60, "content_140aa2d60", fn_140aa2d60 as *const ()), // sub_140aa2d60
    Replacement::draft(0x140aa2e70, "content_140aa2e70", fn_140aa2e70 as *const ()), // sub_140aa2e70
    Replacement::draft_throws(0x140aa3530, "content_140aa3530", fn_140aa3530 as *const (), Throws::Caught), // sub_140aa3530
    Replacement::draft_throws(0x140aa3810, "content_140aa3810", fn_140aa3810 as *const (), Throws::Caught), // sub_140aa3810
    Replacement::draft_throws(0x140aa3ac0, "content_140aa3ac0", fn_140aa3ac0 as *const (), Throws::Caught), // sub_140aa3ac0
    Replacement::draft_throws(0x140aa3d60, "content_140aa3d60", fn_140aa3d60 as *const (), Throws::Caught), // sub_140aa3d60
    Replacement::draft_throws(0x140aa3e30, "content_140aa3e30", fn_140aa3e30 as *const (), Throws::Caught), // sub_140aa3e30
    Replacement::draft_throws(0x140aa4300, "content_140aa4300", fn_140aa4300 as *const (), Throws::Caught), // sub_140aa4300
    Replacement::draft_throws(0x140aa4790, "content_140aa4790", fn_140aa4790 as *const (), Throws::Caught), // sub_140aa4790
    Replacement::draft_throws(0x140aa4d50, "content_140aa4d50", fn_140aa4d50 as *const (), Throws::Caught), // sub_140aa4d50
    Replacement::draft_throws(0x140aa4e90, "content_140aa4e90", fn_140aa4e90 as *const (), Throws::Caught), // sub_140aa4e90
    Replacement::draft_throws(0x140aa4fd0, "content_140aa4fd0", fn_140aa4fd0 as *const (), Throws::Caught), // sub_140aa4fd0
    Replacement::draft_throws(0x140aa5110, "content_140aa5110", fn_140aa5110 as *const (), Throws::Caught), // sub_140aa5110
    Replacement::draft_throws(0x140aa5300, "content_140aa5300", fn_140aa5300 as *const (), Throws::Caught), // sub_140aa5300
    Replacement::draft_throws(0x140aa5770, "content_140aa5770", fn_140aa5770 as *const (), Throws::Caught), // sub_140aa5770
    Replacement::draft_throws(0x140aa5ba0, "content_140aa5ba0", fn_140aa5ba0 as *const (), Throws::Caught), // sub_140aa5ba0
    Replacement::draft(0x140aa6100, "content_140aa6100", fn_140aa6100 as *const ()), // sub_140aa6100
    Replacement::draft(0x140aa61d0, "content_140aa61d0", fn_140aa61d0 as *const ()), // sub_140aa61d0
    Replacement::draft(0x140aa6240, "content_140aa6240", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432a97f8> as *const ()), // sub_140aa6240
    Replacement::draft(0x140aa62a0, "content_140aa62a0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432a97f8> as *const ()), // sub_140aa62a0
    Replacement::draft_throws(0x140aa6330, "content_140aa6330", fn_140aa6330 as *const (), Throws::Caught), // sub_140aa6330
    Replacement::draft_throws(0x140aa63c0, "content_140aa63c0", tpl_1908::<0x14387e648, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140aa63c0
    Replacement::draft(0x140aa6440, "content_140aa6440", tpl_9749::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432a98a8> as *const ()), // sub_140aa6440
    Replacement::draft(0x140aa64b0, "content_140aa64b0", tpl_9750::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432a98a8> as *const ()), // sub_140aa64b0
    Replacement::draft(0x140aa65e0, "content_140aa65e0", tpl_9760::<0x14387e650, 0x14243b0b0, 0x14387e648, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140aa65e0
    Replacement::draft_throws(0x140aa66c0, "content_140aa66c0", tpl_9761::<0x14387e678, 0x14387e650, 0x1401e9fa0, 0x14387e680, 0x14387e648, 0x1401e9e70, 0x328, 0x338, 0x8, 0x48, 0x369> as *const (), Throws::Discard), // sub_140aa66c0
    Replacement::draft(0x140aa6770, "content_140aa6770", tpl_9753::<0x14387e650, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x48> as *const ()), // sub_140aa6770
    Replacement::draft_throws(0x140aa6820, "content_140aa6820", tpl_9762::<0x14387e678, 0x14387e650, 0x1401e9b60, 0x14387e680, 0x14387e648, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369> as *const (), Throws::Discard), // sub_140aa6820
    Replacement::draft_throws(0x140aa68e0, "content_140aa68e0", fn_140aa68e0 as *const (), Throws::Caught), // sub_140aa68e0
    Replacement::draft_throws(0x140aa69c0, "content_140aa69c0", fn_140aa69c0 as *const (), Throws::Caught), // sub_140aa69c0
    Replacement::draft_throws(0x140aa6aa0, "content_140aa6aa0", fn_140aa6aa0 as *const (), Throws::Caught), // sub_140aa6aa0
    Replacement::draft(0x140aa6b80, "content_140aa6b80", tpl_13224::<0x14387e650, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140aa6b80
    Replacement::draft(0x140aa6c30, "content_140aa6c30", tpl_9757::<0x14387e678, 0x14387e650, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x338, 0x8, 0xe8> as *const ()), // sub_140aa6c30
    Replacement::draft(0x140aa6cd0, "content_140aa6cd0", tpl_9753::<0x14387e650, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xe8> as *const ()), // sub_140aa6cd0
    Replacement::draft_throws(0x140aa6d80, "content_140aa6d80", tpl_9765::<0x14387e678, 0x14387e650, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_140aa6d80
    Replacement::draft_throws(0x140aa6e20, "content_140aa6e20", fn_140aa6e20 as *const (), Throws::Discard), // sub_140aa6e20
    Replacement::draft(0x140aa6f20, "content_140aa6f20", tpl_9749::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432aa588> as *const ()), // sub_140aa6f20
    Replacement::draft(0x140aa6f90, "content_140aa6f90", tpl_9750::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432aa588> as *const ()), // sub_140aa6f90
    Replacement::draft(0x140aa70c0, "content_140aa70c0", tpl_9806::<0x14387e668, 0x14243b0b0, 0x14387e648, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140aa70c0
    Replacement::draft_throws(0x140aa71a0, "content_140aa71a0", tpl_9761::<0x14387e624, 0x14387e668, 0x1401e9fa0, 0x14387e680, 0x14387e648, 0x1401e9e70, 0x328, 0x338, 0x8, 0x38, 0x369> as *const (), Throws::Discard), // sub_140aa71a0
    Replacement::draft(0x140aa7250, "content_140aa7250", tpl_9753::<0x14387e668, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x38> as *const ()), // sub_140aa7250
    Replacement::draft_throws(0x140aa7300, "content_140aa7300", tpl_9807::<0x14387e624, 0x14387e668, 0x1401e9b60, 0x14387e680, 0x14387e648, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x4, 0x369> as *const (), Throws::Discard), // sub_140aa7300
    Replacement::draft_throws(0x140aa73c0, "content_140aa73c0", fn_140aa73c0 as *const (), Throws::Caught), // sub_140aa73c0
    Replacement::draft_throws(0x140aa74a0, "content_140aa74a0", fn_140aa74a0 as *const (), Throws::Caught), // sub_140aa74a0
    Replacement::draft_throws(0x140aa7580, "content_140aa7580", fn_140aa7580 as *const (), Throws::Caught), // sub_140aa7580
    Replacement::draft(0x140aa7660, "content_140aa7660", tpl_13224::<0x14387e668, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140aa7660
    Replacement::draft(0x140aa7710, "content_140aa7710", tpl_9757::<0x14387e624, 0x14387e668, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x338, 0x8, 0xd8> as *const ()), // sub_140aa7710
    Replacement::draft(0x140aa77b0, "content_140aa77b0", tpl_9753::<0x14387e668, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xd8> as *const ()), // sub_140aa77b0
    Replacement::draft_throws(0x140aa7860, "content_140aa7860", tpl_9809::<0x14387e624, 0x14387e668, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x338, 0x8, 0x120, 0x4> as *const (), Throws::Discard), // sub_140aa7860
    Replacement::draft_throws(0x140aa7900, "content_140aa7900", fn_140aa7900 as *const (), Throws::Discard), // sub_140aa7900
    Replacement::draft(0x140aa7a00, "content_140aa7a00", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432a9e38, 0x8> as *const ()), // sub_140aa7a00
    Replacement::draft(0x140aa7a80, "content_140aa7a80", tpl_9767::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432a9e38, 0xfffffffffffffff8> as *const ()), // sub_140aa7a80
    Replacement::draft(0x140aa7bb0, "content_140aa7bb0", tpl_9811::<0x14387e640, 0x14243b0b0, 0x14387e648, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140aa7bb0
    Replacement::draft_throws(0x140aa7c90, "content_140aa7c90", tpl_9812::<0x14387e620, 0x14387e640, 0x1401e9fa0, 0x14387e680, 0x14387e648, 0x1401e9e70, 0x328, 0x8, 0x338, 0x38, 0x4, 0x369> as *const (), Throws::Discard), // sub_140aa7c90
    Replacement::draft(0x140aa7d70, "content_140aa7d70", tpl_9770::<0x14387e640, 0x14243b0b0, 0x14387e648, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x38, 0xc> as *const ()), // sub_140aa7d70
    Replacement::draft_throws(0x140aa7e40, "content_140aa7e40", tpl_9813::<0x14387e620, 0x14387e640, 0x1401e9b60, 0x14387e680, 0x14387e648, 0x1401e9a20, 0x328, 0x338, 0x120, 0x8, 0x369> as *const (), Throws::Discard), // sub_140aa7e40
    Replacement::draft_throws(0x140aa7f10, "content_140aa7f10", fn_140aa7f10 as *const (), Throws::Caught), // sub_140aa7f10
    Replacement::draft_throws(0x140aa7fe0, "content_140aa7fe0", fn_140aa7fe0 as *const (), Throws::Caught), // sub_140aa7fe0
    Replacement::draft_throws(0x140aa81d0, "content_140aa81d0", fn_140aa81d0 as *const (), Throws::Caught), // sub_140aa81d0
    Replacement::draft(0x140aa83b0, "content_140aa83b0", fn_140aa83b0 as *const ()), // sub_140aa83b0
    Replacement::draft(0x140aa8470, "content_140aa8470", tpl_9776::<0x14387e620, 0x14387e640, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x338, 0x8, 0xd8, 0xc> as *const ()), // sub_140aa8470
    Replacement::draft(0x140aa8520, "content_140aa8520", tpl_9770::<0x14387e640, 0x14243b0b0, 0x14387e648, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xd8, 0xc> as *const ()), // sub_140aa8520
    Replacement::draft_throws(0x140aa85f0, "content_140aa85f0", tpl_9777::<0x14387e620, 0x14387e640, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_140aa85f0
    Replacement::draft_throws(0x140aa8690, "content_140aa8690", tpl_9818::<0x14387e640, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387e648, 0x20, 0x8, 0x4, 0x143251120, 0x1413c3e10> as *const (), Throws::Discard), // sub_140aa8690
    Replacement::draft(0x140aa8780, "content_140aa8780", tpl_9779::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432aa0a8, 0x8> as *const ()), // sub_140aa8780
    Replacement::draft(0x140aa8810, "content_140aa8810", tpl_9780::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432aa0a8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140aa8810
    Replacement::draft(0x140aa8960, "content_140aa8960", tpl_9819::<0x14387e638, 0x14243b0b0, 0x14387e648, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140aa8960
    Replacement::draft_throws(0x140aa8a40, "content_140aa8a40", fn_140aa8a40 as *const (), Throws::Discard), // sub_140aa8a40
    Replacement::draft(0x140aa8b20, "content_140aa8b20", tpl_9783::<0x14387e638, 0x14243b0b0, 0x14387e648, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x38, 0xc, 0x10> as *const ()), // sub_140aa8b20
    Replacement::draft_throws(0x140aa8c00, "content_140aa8c00", fn_140aa8c00 as *const (), Throws::Discard), // sub_140aa8c00
    Replacement::draft_throws(0x140aa8cd0, "content_140aa8cd0", fn_140aa8cd0 as *const (), Throws::Caught), // sub_140aa8cd0
    Replacement::draft_throws(0x140aa8db0, "content_140aa8db0", fn_140aa8db0 as *const (), Throws::Caught), // sub_140aa8db0
    Replacement::draft_throws(0x140aa8f90, "content_140aa8f90", fn_140aa8f90 as *const (), Throws::Caught), // sub_140aa8f90
    Replacement::draft(0x140aa9160, "content_140aa9160", fn_140aa9160 as *const ()), // sub_140aa9160
    Replacement::draft(0x140aa9210, "content_140aa9210", tpl_9789::<0x14387e614, 0x14387e638, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x338, 0x8, 0xd8, 0xc, 0x10> as *const ()), // sub_140aa9210
    Replacement::draft(0x140aa92d0, "content_140aa92d0", tpl_9783::<0x14387e638, 0x14243b0b0, 0x14387e648, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xd8, 0xc, 0x10> as *const ()), // sub_140aa92d0
    Replacement::draft_throws(0x140aa93b0, "content_140aa93b0", tpl_9790::<0x14387e614, 0x14387e638, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x338, 0x8, 0x120, 0x10> as *const (), Throws::Discard), // sub_140aa93b0
    Replacement::draft_throws(0x140aa9460, "content_140aa9460", tpl_9820::<0x14387e638, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387e648, 0x20, 0x8, 0x4, 0x143251120, 0x1413c41a0> as *const (), Throws::Discard), // sub_140aa9460
    Replacement::draft(0x140aa9560, "content_140aa9560", tpl_9792::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432aa9b8, 0x8> as *const ()), // sub_140aa9560
    Replacement::draft(0x140aa95f0, "content_140aa95f0", tpl_9793::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432aa9b8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140aa95f0
    Replacement::draft(0x140aa9740, "content_140aa9740", tpl_9821::<0x14387e660, 0x14243b0b0, 0x14387e648, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140aa9740
    Replacement::draft_throws(0x140aa9820, "content_140aa9820", fn_140aa9820 as *const (), Throws::Discard), // sub_140aa9820
    Replacement::draft(0x140aa9910, "content_140aa9910", tpl_9823::<0x14387e660, 0x14243b0b0, 0x14387e648, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x38, 0xc, 0x10, 0x14> as *const ()), // sub_140aa9910
    Replacement::draft_throws(0x140aa99f0, "content_140aa99f0", fn_140aa99f0 as *const (), Throws::Discard), // sub_140aa99f0
    Replacement::draft_throws(0x140aa9ab0, "content_140aa9ab0", fn_140aa9ab0 as *const (), Throws::Caught), // sub_140aa9ab0
    Replacement::draft_throws(0x140aa9b80, "content_140aa9b80", fn_140aa9b80 as *const (), Throws::Caught), // sub_140aa9b80
    Replacement::draft_throws(0x140aa9d50, "content_140aa9d50", fn_140aa9d50 as *const (), Throws::Caught), // sub_140aa9d50
    Replacement::draft(0x140aa9f10, "content_140aa9f10", fn_140aa9f10 as *const ()), // sub_140aa9f10
    Replacement::draft(0x140aa9fc0, "content_140aa9fc0", tpl_9824::<0x14387e688, 0x14387e660, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14> as *const ()), // sub_140aa9fc0
    Replacement::draft(0x140aaa090, "content_140aaa090", tpl_9825::<0x14387e660, 0x14243b0b0, 0x14387e648, 0x8, 0x338, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xd8, 0xc, 0x10, 0x14> as *const ()), // sub_140aaa090
    Replacement::draft_throws(0x140aaa180, "content_140aaa180", tpl_9804::<0x14387e688, 0x14387e660, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_140aaa180
    Replacement::draft_throws(0x140aaa220, "content_140aaa220", tpl_9826::<0x14387e660, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387e648, 0x20, 0x8, 0x4, 0xc, 0x143251120, 0x1413c4540> as *const (), Throws::Discard), // sub_140aaa220
    Replacement::draft(0x140aaa330, "content_140aaa330", fn_140aaa330 as *const ()), // sub_140aaa330
    Replacement::draft(0x140aaa3b0, "content_140aaa3b0", fn_140aaa3b0 as *const ()), // sub_140aaa3b0
    Replacement::draft(0x140aaa450, "content_140aaa450", fn_140aaa450 as *const ()), // sub_140aaa450
    Replacement::draft(0x140aaa4f0, "content_140aaa4f0", tpl_9760::<0x14387e630, 0x14243b0b0, 0x14387e648, 0x8, 0x1426d4580, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x1> as *const ()), // sub_140aaa4f0
    Replacement::draft_throws(0x140aaa5d0, "content_140aaa5d0", fn_140aaa5d0 as *const (), Throws::Discard), // sub_140aaa5d0
    Replacement::draft(0x140aaa6d0, "content_140aaa6d0", fn_140aaa6d0 as *const ()), // sub_140aaa6d0
    Replacement::draft_throws(0x140aaa7b0, "content_140aaa7b0", fn_140aaa7b0 as *const (), Throws::Discard), // sub_140aaa7b0
    Replacement::draft_throws(0x140aaa880, "content_140aaa880", fn_140aaa880 as *const (), Throws::Caught), // sub_140aaa880
    Replacement::draft_throws(0x140aaa950, "content_140aaa950", fn_140aaa950 as *const (), Throws::Caught), // sub_140aaa950
    Replacement::draft_throws(0x140aaaa20, "content_140aaaa20", fn_140aaaa20 as *const (), Throws::Caught), // sub_140aaaa20
    Replacement::draft(0x140aaaaf0, "content_140aaaaf0", fn_140aaaaf0 as *const ()), // sub_140aaaaf0
    Replacement::draft(0x140aaaba0, "content_140aaaba0", fn_140aaaba0 as *const ()), // sub_140aaaba0
    Replacement::draft(0x140aaac70, "content_140aaac70", fn_140aaac70 as *const ()), // sub_140aaac70
    Replacement::draft_throws(0x140aaad60, "content_140aaad60", tpl_9765::<0x14387e65c, 0x14387e630, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x338, 0x8, 0x120> as *const (), Throws::Discard), // sub_140aaad60
    Replacement::draft_throws(0x140aaae00, "content_140aaae00", fn_140aaae00 as *const (), Throws::Discard), // sub_140aaae00
    Replacement::draft(0x140aaaef0, "content_140aaaef0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432a9cd8, 0x8> as *const ()), // sub_140aaaef0
    Replacement::draft(0x140aaaf70, "content_140aaaf70", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432a9cd8> as *const ()), // sub_140aaaf70
    Replacement::draft(0x140aab0a0, "content_140aab0a0", tpl_9838::<0x14387e670, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x140839b60> as *const ()), // sub_140aab0a0
    Replacement::draft_throws(0x140aab150, "content_140aab150", tpl_9839::<0x14387e658, 0x14387e670, 0x1401e9fa0, 0x14387e680, 0x14387e648, 0x1401e9e70, 0x8, 0x14082b050> as *const (), Throws::Discard), // sub_140aab150
    Replacement::draft(0x140aab1f0, "content_140aab1f0", tpl_9838::<0x14387e670, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14083a060> as *const ()), // sub_140aab1f0
    Replacement::draft_throws(0x140aab2a0, "content_140aab2a0", tpl_9839::<0x14387e658, 0x14387e670, 0x1401e9b60, 0x14387e680, 0x14387e648, 0x1401e9a20, 0x8, 0x14082b180> as *const (), Throws::Discard), // sub_140aab2a0
    Replacement::draft_throws(0x140aab340, "content_140aab340", fn_140aab340 as *const (), Throws::Caught), // sub_140aab340
    Replacement::draft_throws(0x140aab410, "content_140aab410", fn_140aab410 as *const (), Throws::Caught), // sub_140aab410
    Replacement::draft_throws(0x140aab4e0, "content_140aab4e0", fn_140aab4e0 as *const (), Throws::Caught), // sub_140aab4e0
    Replacement::draft(0x140aab5b0, "content_140aab5b0", tpl_9838::<0x14387e670, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x140839ed0> as *const ()), // sub_140aab5b0
    Replacement::draft(0x140aab660, "content_140aab660", tpl_9842::<0x14387e658, 0x14387e670, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x8, 0x14083a350> as *const ()), // sub_140aab660
    Replacement::draft(0x140aab6f0, "content_140aab6f0", tpl_9838::<0x14387e670, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14083a4c0> as *const ()), // sub_140aab6f0
    Replacement::draft_throws(0x140aab7a0, "content_140aab7a0", tpl_9843::<0x14387e658, 0x14387e670, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x8, 0x14083a610> as *const (), Throws::Discard), // sub_140aab7a0
    Replacement::draft_throws(0x140aab830, "content_140aab830", fn_140aab830 as *const (), Throws::Caught), // sub_140aab830
    Replacement::draft(0x140aab9a0, "content_140aab9a0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432a9d88, 0x8> as *const ()), // sub_140aab9a0
    Replacement::draft(0x140aaba20, "content_140aaba20", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432a9d88> as *const ()), // sub_140aaba20
    Replacement::draft(0x140aabb50, "content_140aabb50", tpl_9838::<0x14387e628, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089c0e0> as *const ()), // sub_140aabb50
    Replacement::draft_throws(0x140aabc00, "content_140aabc00", tpl_9839::<0x14387e684, 0x14387e628, 0x1401e9fa0, 0x14387e680, 0x14387e648, 0x1401e9e70, 0x8, 0x14088b020> as *const (), Throws::Discard), // sub_140aabc00
    Replacement::draft(0x140aabca0, "content_140aabca0", tpl_9838::<0x14387e628, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089c5e0> as *const ()), // sub_140aabca0
    Replacement::draft_throws(0x140aabd50, "content_140aabd50", tpl_9839::<0x14387e684, 0x14387e628, 0x1401e9b60, 0x14387e680, 0x14387e648, 0x1401e9a20, 0x8, 0x14088b150> as *const (), Throws::Discard), // sub_140aabd50
    Replacement::draft_throws(0x140aabdf0, "content_140aabdf0", fn_140aabdf0 as *const (), Throws::Caught), // sub_140aabdf0
    Replacement::draft_throws(0x140aabec0, "content_140aabec0", fn_140aabec0 as *const (), Throws::Caught), // sub_140aabec0
    Replacement::draft_throws(0x140aabf90, "content_140aabf90", fn_140aabf90 as *const (), Throws::Caught), // sub_140aabf90
    Replacement::draft(0x140aac060, "content_140aac060", tpl_9838::<0x14387e628, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089c450> as *const ()), // sub_140aac060
    Replacement::draft(0x140aac110, "content_140aac110", tpl_9842::<0x14387e684, 0x14387e628, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x8, 0x14089c8d0> as *const ()), // sub_140aac110
    Replacement::draft(0x140aac1a0, "content_140aac1a0", tpl_9838::<0x14387e628, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x14089ca40> as *const ()), // sub_140aac1a0
    Replacement::draft_throws(0x140aac250, "content_140aac250", tpl_9843::<0x14387e684, 0x14387e628, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x8, 0x14089cb90> as *const (), Throws::Discard), // sub_140aac250
    Replacement::draft_throws(0x140aac2e0, "content_140aac2e0", tpl_9845::<0x14387e628, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387e648, 0x8, 0x143251120, 0x14088b280> as *const (), Throws::Discard), // sub_140aac2e0
    Replacement::draft(0x140aac3b0, "content_140aac3b0", tpl_6379::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1432aa318, 0x8, 0x1c> as *const ()), // sub_140aac3b0
    Replacement::draft(0x140aac440, "content_140aac440", tpl_9847::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x1432aa318, 0xffffffffffffffe8, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x4> as *const ()), // sub_140aac440
    Replacement::draft(0x140aac580, "content_140aac580", tpl_9848::<0x14387e690, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x1406dd2c0> as *const ()), // sub_140aac580
    Replacement::draft_throws(0x140aac630, "content_140aac630", tpl_9849::<0x14387e67c, 0x14387e690, 0x1401e9fa0, 0x14387e680, 0x14387e648, 0x1401e9e70, 0x328, 0x8, 0x338, 0x1403c7b30, 0x369, 0x1401e84a0> as *const (), Throws::Discard), // sub_140aac630
    Replacement::draft(0x140aac710, "content_140aac710", tpl_9850::<0x14387e690, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x1403c7b30> as *const ()), // sub_140aac710
    Replacement::draft_throws(0x140aac7c0, "content_140aac7c0", tpl_9851::<0x14387e67c, 0x14387e690, 0x1401e9b60, 0x14387e680, 0x14387e648, 0x1401e9a20, 0x328, 0x8, 0x338, 0x1406f1f30, 0x369, 0x1401e84a0> as *const (), Throws::Discard), // sub_140aac7c0
    Replacement::draft_throws(0x140aac890, "content_140aac890", fn_140aac890 as *const (), Throws::Caught), // sub_140aac890
    Replacement::draft_throws(0x140aac940, "content_140aac940", fn_140aac940 as *const (), Throws::Caught), // sub_140aac940
    Replacement::draft_throws(0x140aac9f0, "content_140aac9f0", fn_140aac9f0 as *const (), Throws::Caught), // sub_140aac9f0
    Replacement::draft(0x140aacaa0, "content_140aacaa0", fn_140aacaa0 as *const ()), // sub_140aacaa0
    Replacement::draft(0x140aacb80, "content_140aacb80", tpl_9853::<0x14387e67c, 0x14387e690, 0x1401e9970, 0x14387e680, 0x14387e648, 0x1401e9860, 0x338, 0x8, 0x88, 0xc0, 0x14240ab10> as *const ()), // sub_140aacb80
    Replacement::draft(0x140aacc70, "content_140aacc70", tpl_9854::<0x14387e690, 0x14243b0b0, 0x14387e648, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff, 0x338, 0x88, 0xc0, 0x14240ab10> as *const ()), // sub_140aacc70
    Replacement::draft_throws(0x140aacd70, "content_140aacd70", tpl_9846::<0x14387e67c, 0x14387e690, 0x1401e9770, 0x14387e680, 0x14387e648, 0x1401e9590, 0x8, 0x1406deba0> as *const (), Throws::Discard), // sub_140aacd70
    Replacement::draft_throws(0x140aace00, "content_140aace00", fn_140aace00 as *const (), Throws::Caught), // sub_140aace00
    Replacement::draft(0x140aacf40, "content_140aacf40", tpl_3383::<0x10, 0x1432a97f8, 0x141834d30> as *const ()), // content::GfxGraphParameterValue<r::GlobalIDRef<content::Curve>>::content::GfxGraphParameterValue<r::GlobalIDRef<content:
    Replacement::draft_throws(0x140aacfe0, "content_140aacfe0", fn_140aacfe0 as *const (), Throws::Caught), // sub_140aacfe0
    Replacement::draft_throws(0x140aad490, "content_140aad490", fn_140aad490 as *const (), Throws::Caught), // sub_140aad490
    Replacement::draft_throws(0x140aad8e0, "content_140aad8e0", fn_140aad8e0 as *const (), Throws::Caught), // sub_140aad8e0
    Replacement::draft(0x140aadcd0, "content_140aadcd0", fn_140aadcd0 as *const ()), // content::ActionSystemActorCollectorComponent::content::ActionSystemActorCollectorComponent::vfunc_18
    Replacement::draft(0x140aade00, "content_140aade00", fn_140aade00 as *const ()), // sub_140aade00
    Replacement::draft(0x140aade80, "content_140aade80", fn_140aade80 as *const ()), // content::ActionSystemActionTableResource::content::ActionSystemActionTableResource::vfunc_0
    Replacement::draft(0x140aadf30, "content_140aadf30", tpl_1149::<0x8, 0x140abc130, 0x1401e66f0> as *const ()), // sub_140aadf30
    Replacement::draft(0x140aadf60, "content_140aadf60", fn_140aadf60 as *const ()), // content::ActionSystemActorResource::content::ActionSystemActorResource::vfunc_0
    Replacement::draft(0x140aadfd0, "unknown_140aadfd0", tpl_2074::<0x14387e698, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140aadfd0
    Replacement::draft_throws(0x140aae070, "unknown_140aae070", fn_140aae070 as *const (), Throws::Discard), // sub_140aae070
    Replacement::draft(0x140aae0f0, "unknown_140aae0f0", tpl_2076::<0x14387e698, 0x14243b0b0, 0x338, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140aae0f0
    Replacement::draft_throws(0x140aae160, "unknown_140aae160", fn_140aae160 as *const (), Throws::Discard), // sub_140aae160
    Replacement::draft_throws(0x140aae1f0, "unknown_140aae1f0", fn_140aae1f0 as *const (), Throws::Caught), // sub_140aae1f0
    Replacement::draft_throws(0x140aae3e0, "unknown_140aae3e0", fn_140aae3e0 as *const (), Throws::Caught), // sub_140aae3e0
    Replacement::draft_throws(0x140aae5a0, "unknown_140aae5a0", fn_140aae5a0 as *const (), Throws::Caught), // sub_140aae5a0
    Replacement::draft(0x140aae760, "unknown_140aae760", fn_140aae760 as *const ()), // sub_140aae760
    Replacement::draft(0x140aae7d0, "unknown_140aae7d0", fn_140aae7d0 as *const ()), // sub_140aae7d0
    Replacement::draft(0x140aae840, "unknown_140aae840", fn_140aae840 as *const ()), // sub_140aae840
    Replacement::draft_throws(0x140aae8c0, "unknown_140aae8c0", tpl_2081::<0x14387e720, 0x14387e698, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140aae8c0
    Replacement::draft_throws(0x140aae930, "unknown_140aae930", fn_140aae930 as *const (), Throws::Discard), // sub_140aae930
    Replacement::draft(0x140aae9e0, "unknown_140aae9e0", tpl_13268::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x140abc3e0> as *const ()), // sub_140aae9e0
    Replacement::draft(0x140aaeaa0, "unknown_140aaeaa0", tpl_13269::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x140abc3e0, 0x18> as *const ()), // sub_140aaeaa0
    Replacement::draft(0x140aaeb80, "unknown_140aaeb80", fn_140aaeb80 as *const ()), // sub_140aaeb80
    Replacement::draft(0x140aaed00, "unknown_140aaed00", fn_140aaed00 as *const ()), // sub_140aaed00
    Replacement::draft_throws(0x140aaee40, "unknown_140aaee40", tpl_13272::<0x14387e71c, 0x14387e6e0, 0x1401e9fa0, 0x10, 0x140ab80b0, 0x328, 0x338, 0x48, 0x140ac91a0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140aaee40
    Replacement::draft_throws(0x140aaeef0, "unknown_140aaeef0", tpl_13273::<0x14387e6e0, 0x14243b0b0, 0x10, 0x140ab41b0, 0x338, 0x48, 0x140ac99b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140aaeef0
    Replacement::draft_throws(0x140aaefb0, "unknown_140aaefb0", tpl_13274::<0x14387e71c, 0x14387e6e0, 0x1401e9b60, 0x10, 0x140ab8350, 0x328, 0x338, 0x120, 0x140aca2c0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140aaefb0
    Replacement::draft_throws(0x140aaf050, "unknown_140aaf050", fn_140aaf050 as *const (), Throws::Caught), // sub_140aaf050
    Replacement::draft_throws(0x140aaf110, "unknown_140aaf110", fn_140aaf110 as *const (), Throws::Caught), // sub_140aaf110
    Replacement::draft_throws(0x140aaf1d0, "unknown_140aaf1d0", fn_140aaf1d0 as *const (), Throws::Caught), // sub_140aaf1d0
    Replacement::draft(0x140aaf290, "unknown_140aaf290", fn_140aaf290 as *const ()), // sub_140aaf290
    Replacement::draft(0x140aaf450, "unknown_140aaf450", fn_140aaf450 as *const ()), // sub_140aaf450
    Replacement::draft(0x140aaf560, "unknown_140aaf560", fn_140aaf560 as *const ()), // sub_140aaf560
    Replacement::draft_throws(0x140aaf680, "unknown_140aaf680", fn_140aaf680 as *const (), Throws::Discard), // sub_140aaf680
    Replacement::draft_throws(0x140aaf780, "unknown_140aaf780", tpl_13280::<0x14387e6e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x10, 0x140ab8d70, 0x1432aae00, 0x140ab8e40> as *const (), Throws::Discard), // sub_140aaf780
    Replacement::draft(0x140aaf830, "unknown_140aaf830", tpl_13268::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1401fc200> as *const ()), // sub_140aaf830
    Replacement::draft(0x140aaf8f0, "unknown_140aaf8f0", tpl_13269::<0x10, 0x1, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1401fc200, 0x18> as *const ()), // sub_140aaf8f0
    Replacement::draft(0x140aaf9d0, "unknown_140aaf9d0", fn_140aaf9d0 as *const ()), // sub_140aaf9d0
    Replacement::draft(0x140aafb10, "unknown_140aafb10", fn_140aafb10 as *const ()), // sub_140aafb10
    Replacement::draft_throws(0x140aafc40, "unknown_140aafc40", tpl_13272::<0x14387e6c8, 0x14387e700, 0x1401e9fa0, 0x10, 0x140ab80b0, 0x328, 0x338, 0x48, 0x140acc790, 0x1401e9e70> as *const (), Throws::Discard), // sub_140aafc40
    Replacement::draft_throws(0x140aafcf0, "unknown_140aafcf0", tpl_13273::<0x14387e700, 0x14243b0b0, 0x10, 0x140ab41b0, 0x338, 0x48, 0x140acced0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140aafcf0
    Replacement::draft_throws(0x140aafdb0, "unknown_140aafdb0", tpl_13274::<0x14387e6c8, 0x14387e700, 0x1401e9b60, 0x10, 0x140ab8350, 0x328, 0x338, 0x120, 0x140acd5d0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140aafdb0
    Replacement::draft_throws(0x140aafe50, "unknown_140aafe50", fn_140aafe50 as *const (), Throws::Caught), // sub_140aafe50
    Replacement::draft_throws(0x140aaff10, "unknown_140aaff10", fn_140aaff10 as *const (), Throws::Caught), // sub_140aaff10
    Replacement::draft_throws(0x140aaffd0, "unknown_140aaffd0", fn_140aaffd0 as *const (), Throws::Caught), // sub_140aaffd0
    Replacement::draft(0x140ab0090, "unknown_140ab0090", fn_140ab0090 as *const ()), // sub_140ab0090
    Replacement::draft(0x140ab01f0, "unknown_140ab01f0", fn_140ab01f0 as *const ()), // sub_140ab01f0
    Replacement::draft(0x140ab0300, "unknown_140ab0300", fn_140ab0300 as *const ()), // sub_140ab0300
    Replacement::draft_throws(0x140ab0420, "unknown_140ab0420", fn_140ab0420 as *const (), Throws::Discard), // sub_140ab0420
    Replacement::draft_throws(0x140ab0530, "unknown_140ab0530", tpl_13280::<0x14387e700, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x10, 0x140ab8d70, 0x1432aae00, 0x140ab8e90> as *const (), Throws::Discard), // sub_140ab0530
    Replacement::draft(0x140ab05e0, "unknown_140ab05e0", fn_140ab05e0 as *const ()), // sub_140ab05e0
    Replacement::draft(0x140ab0680, "unknown_140ab0680", fn_140ab0680 as *const ()), // sub_140ab0680
    Replacement::draft(0x140ab0700, "unknown_140ab0700", fn_140ab0700 as *const ()), // sub_140ab0700
    Replacement::draft_throws(0x140ab07d0, "unknown_140ab07d0", fn_140ab07d0 as *const (), Throws::Discard), // sub_140ab07d0
    Replacement::draft(0x140ab0840, "unknown_140ab0840", fn_140ab0840 as *const ()), // sub_140ab0840
    Replacement::draft_throws(0x140ab09d0, "unknown_140ab09d0", fn_140ab09d0 as *const (), Throws::Discard), // sub_140ab09d0
    Replacement::draft_throws(0x140ab0ad0, "unknown_140ab0ad0", fn_140ab0ad0 as *const (), Throws::Discard), // sub_140ab0ad0
    Replacement::draft_throws(0x140ab0bf0, "unknown_140ab0bf0", fn_140ab0bf0 as *const (), Throws::Discard), // sub_140ab0bf0
    Replacement::draft_throws(0x140ab0ce0, "unknown_140ab0ce0", fn_140ab0ce0 as *const (), Throws::Discard), // sub_140ab0ce0
    Replacement::draft_throws(0x140ab0d70, "unknown_140ab0d70", fn_140ab0d70 as *const (), Throws::Caught), // sub_140ab0d70
    Replacement::draft_throws(0x140ab0ff0, "unknown_140ab0ff0", fn_140ab0ff0 as *const (), Throws::Caught), // sub_140ab0ff0
    Replacement::draft_throws(0x140ab1240, "unknown_140ab1240", fn_140ab1240 as *const (), Throws::Caught), // sub_140ab1240
    Replacement::draft_throws(0x140ab1480, "unknown_140ab1480", fn_140ab1480 as *const (), Throws::Discard), // sub_140ab1480
    Replacement::draft(0x140ab1540, "unknown_140ab1540", fn_140ab1540 as *const ()), // sub_140ab1540
    Replacement::draft(0x140ab1660, "unknown_140ab1660", fn_140ab1660 as *const ()), // sub_140ab1660
    Replacement::draft_throws(0x140ab1790, "unknown_140ab1790", fn_140ab1790 as *const (), Throws::Discard), // sub_140ab1790
    Replacement::draft_throws(0x140ab18f0, "unknown_140ab18f0", fn_140ab18f0 as *const (), Throws::Discard), // sub_140ab18f0
    Replacement::draft(0x140ab19e0, "unknown_140ab19e0", fn_140ab19e0 as *const ()), // sub_140ab19e0
    Replacement::draft(0x140ab1ab0, "unknown_140ab1ab0", fn_140ab1ab0 as *const ()), // sub_140ab1ab0
    Replacement::draft(0x140ab1b90, "unknown_140ab1b90", fn_140ab1b90 as *const ()), // sub_140ab1b90
    Replacement::draft(0x140ab1c60, "unknown_140ab1c60", fn_140ab1c60 as *const ()), // sub_140ab1c60
    Replacement::draft_throws(0x140ab1de0, "unknown_140ab1de0", fn_140ab1de0 as *const (), Throws::Discard), // sub_140ab1de0
    Replacement::draft(0x140ab1f30, "unknown_140ab1f30", fn_140ab1f30 as *const ()), // sub_140ab1f30
    Replacement::draft_throws(0x140ab2010, "unknown_140ab2010", fn_140ab2010 as *const (), Throws::Discard), // sub_140ab2010
    Replacement::draft(0x140ab2170, "unknown_140ab2170", fn_140ab2170 as *const ()), // sub_140ab2170
    Replacement::draft_throws(0x140ab2230, "unknown_140ab2230", fn_140ab2230 as *const (), Throws::Caught), // sub_140ab2230
    Replacement::draft_throws(0x140ab2630, "unknown_140ab2630", fn_140ab2630 as *const (), Throws::Caught), // sub_140ab2630
    Replacement::draft_throws(0x140ab29d0, "unknown_140ab29d0", fn_140ab29d0 as *const (), Throws::Caught), // sub_140ab29d0
    Replacement::draft(0x140ab2d50, "unknown_140ab2d50", fn_140ab2d50 as *const ()), // sub_140ab2d50
    Replacement::draft(0x140ab2e60, "unknown_140ab2e60", fn_140ab2e60 as *const ()), // sub_140ab2e60
    Replacement::draft(0x140ab2f30, "unknown_140ab2f30", fn_140ab2f30 as *const ()), // sub_140ab2f30
    Replacement::draft_throws(0x140ab3010, "unknown_140ab3010", fn_140ab3010 as *const (), Throws::Discard), // sub_140ab3010
    Replacement::draft_throws(0x140ab30d0, "unknown_140ab30d0", fn_140ab30d0 as *const (), Throws::Discard), // sub_140ab30d0
    Replacement::draft(0x140ab31d0, "unknown_140ab31d0", fn_140ab31d0 as *const ()), // sub_140ab31d0
    Replacement::draft(0x140ab3270, "unknown_140ab3270", fn_140ab3270 as *const ()), // sub_140ab3270
    Replacement::draft(0x140ab3320, "unknown_140ab3320", tpl_11191::<0x14387e6d0, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x30, 0x1426d4580, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140ab3320
    Replacement::draft_throws(0x140ab3400, "unknown_140ab3400", tpl_71725::<0x14387e6f8, 0x14387e6d0, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x30, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140ab3400
    Replacement::draft(0x140ab34e0, "unknown_140ab34e0", tpl_2258::<0x14387e6d0, 0x14243b0b0, 0x338, 0x20, 0x30, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ab34e0
    Replacement::draft_throws(0x140ab3560, "unknown_140ab3560", fn_140ab3560 as *const (), Throws::Discard), // sub_140ab3560
    Replacement::draft_throws(0x140ab3650, "unknown_140ab3650", fn_140ab3650 as *const (), Throws::Caught), // sub_140ab3650
    Replacement::draft_throws(0x140ab38d0, "unknown_140ab38d0", fn_140ab38d0 as *const (), Throws::Caught), // sub_140ab38d0
    Replacement::draft_throws(0x140ab3b30, "unknown_140ab3b30", fn_140ab3b30 as *const (), Throws::Caught), // sub_140ab3b30
    Replacement::draft(0x140ab3d80, "unknown_140ab3d80", tpl_11195::<0x14387e6d0, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ab3d80
    Replacement::draft(0x140ab3e30, "unknown_140ab3e30", tpl_2262::<0x14387e6f8, 0x14387e6d0, 0x1401e9970, 0x338, 0xc0, 0x30, 0xd8, 0x1401e9860> as *const ()), // sub_140ab3e30
    Replacement::draft(0x140ab3eb0, "unknown_140ab3eb0", tpl_2258::<0x14387e6d0, 0x14243b0b0, 0x338, 0xc0, 0x30, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ab3eb0
    Replacement::draft_throws(0x140ab3f40, "unknown_140ab3f40", tpl_13321::<0x14387e6f8, 0x14387e6d0, 0x1401e9770, 0x338, 0x1424b4cb0, 0x30, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140ab3f40
    Replacement::draft_throws(0x140ab3fc0, "unknown_140ab3fc0", tpl_13322::<0x14387e6d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143263aa0, 0x1402c4730, 0x30, 0x1432aae28, 0x14020f240> as *const (), Throws::Discard), // sub_140ab3fc0
    Replacement::draft(0x140ab40b0, "unknown_140ab40b0", tpl_13323::<0x14387e6f0, 0x14243b0b0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ab40b0
    Replacement::draft(0x140ab4130, "unknown_140ab4130", tpl_13323::<0x14387e6f0, 0x14243b0b0, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ab4130
    Replacement::draft_throws(0x140ab41b0, "unknown_140ab41b0", fn_140ab41b0 as *const (), Throws::Discard), // sub_140ab41b0
    Replacement::draft_throws(0x140ab4220, "unknown_140ab4220", fn_140ab4220 as *const (), Throws::Discard), // sub_140ab4220
    Replacement::draft_throws(0x140ab42c0, "unknown_140ab42c0", fn_140ab42c0 as *const (), Throws::Caught), // sub_140ab42c0
    Replacement::draft_throws(0x140ab4370, "unknown_140ab4370", fn_140ab4370 as *const (), Throws::Caught), // sub_140ab4370
    Replacement::draft_throws(0x140ab4420, "unknown_140ab4420", fn_140ab4420 as *const (), Throws::Caught), // sub_140ab4420
    Replacement::draft(0x140ab44d0, "unknown_140ab44d0", fn_140ab44d0 as *const ()), // sub_140ab44d0
    Replacement::draft(0x140ab4570, "unknown_140ab4570", fn_140ab4570 as *const ()), // sub_140ab4570
    Replacement::draft(0x140ab4640, "unknown_140ab4640", fn_140ab4640 as *const ()), // sub_140ab4640
    Replacement::draft_throws(0x140ab4720, "unknown_140ab4720", fn_140ab4720 as *const (), Throws::Discard), // sub_140ab4720
    Replacement::draft_throws(0x140ab47e0, "unknown_140ab47e0", tpl_2091::<0x14387e6f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432aaeb0, 0x140abbb50> as *const (), Throws::Discard), // sub_140ab47e0
    Replacement::draft(0x140ab4880, "unknown_140ab4880", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432ab940, 0x8> as *const ()), // sub_140ab4880
    Replacement::draft(0x140ab4900, "unknown_140ab4900", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ab940, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140ab4900
    Replacement::draft_throws(0x140ab49a0, "unknown_140ab49a0", fn_140ab49a0 as *const (), Throws::Caught), // sub_140ab49a0
    Replacement::draft_throws(0x140ab4a30, "unknown_140ab4a30", tpl_1908::<0x14387e710, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140ab4a30
    Replacement::draft(0x140ab4ab0, "unknown_140ab4ab0", fn_140ab4ab0 as *const ()), // sub_140ab4ab0
    Replacement::draft(0x140ab4b50, "unknown_140ab4b50", fn_140ab4b50 as *const ()), // sub_140ab4b50
    Replacement::draft(0x140ab4c00, "unknown_140ab4c00", fn_140ab4c00 as *const ()), // sub_140ab4c00
    Replacement::draft_throws(0x140ab4cd0, "unknown_140ab4cd0", fn_140ab4cd0 as *const (), Throws::Discard), // sub_140ab4cd0
    Replacement::draft(0x140ab4e50, "unknown_140ab4e50", fn_140ab4e50 as *const ()), // sub_140ab4e50
    Replacement::draft_throws(0x140ab50f0, "unknown_140ab50f0", fn_140ab50f0 as *const (), Throws::Discard), // sub_140ab50f0
    Replacement::draft_throws(0x140ab52b0, "unknown_140ab52b0", fn_140ab52b0 as *const (), Throws::Discard), // sub_140ab52b0
    Replacement::draft_throws(0x140ab5450, "unknown_140ab5450", fn_140ab5450 as *const (), Throws::Discard), // sub_140ab5450
    Replacement::draft_throws(0x140ab5580, "unknown_140ab5580", fn_140ab5580 as *const (), Throws::Discard), // sub_140ab5580
    Replacement::draft_throws(0x140ab5610, "unknown_140ab5610", fn_140ab5610 as *const (), Throws::Caught), // sub_140ab5610
    Replacement::draft_throws(0x140ab56e0, "unknown_140ab56e0", fn_140ab56e0 as *const (), Throws::Caught), // sub_140ab56e0
    Replacement::draft_throws(0x140ab57b0, "unknown_140ab57b0", fn_140ab57b0 as *const (), Throws::Caught), // sub_140ab57b0
    Replacement::draft_throws(0x140ab5880, "unknown_140ab5880", fn_140ab5880 as *const (), Throws::Discard), // sub_140ab5880
    Replacement::draft(0x140ab5ab0, "unknown_140ab5ab0", fn_140ab5ab0 as *const ()), // sub_140ab5ab0
    Replacement::draft(0x140ab5cf0, "unknown_140ab5cf0", fn_140ab5cf0 as *const ()), // sub_140ab5cf0
    Replacement::draft_throws(0x140ab5f50, "unknown_140ab5f50", fn_140ab5f50 as *const (), Throws::Discard), // sub_140ab5f50
    Replacement::draft_throws(0x140ab61c0, "unknown_140ab61c0", fn_140ab61c0 as *const (), Throws::Discard), // sub_140ab61c0
    Replacement::draft(0x140ab62d0, "unknown_140ab62d0", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432abbe8, 0x8> as *const ()), // sub_140ab62d0
    Replacement::draft(0x140ab6350, "unknown_140ab6350", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432abbe8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140ab6350
    Replacement::draft_throws(0x140ab63f0, "unknown_140ab63f0", fn_140ab63f0 as *const (), Throws::Caught), // sub_140ab63f0
    Replacement::draft_throws(0x140ab6480, "unknown_140ab6480", tpl_1908::<0x14387e6d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140ab6480
    Replacement::draft(0x140ab6500, "unknown_140ab6500", tpl_7434::<0x130, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140ab6590> as *const ()), // sub_140ab6500
    Replacement::draft(0x140ab6590, "unknown_140ab6590", fn_140ab6590 as *const ()), // sub_140ab6590
    Replacement::draft(0x140ab6610, "unknown_140ab6610", fn_140ab6610 as *const ()), // sub_140ab6610
    Replacement::draft(0x140ab66c0, "unknown_140ab66c0", tpl_13342::<0x14387e6b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x140ab1b90> as *const ()), // sub_140ab66c0
    Replacement::draft(0x140ab6770, "unknown_140ab6770", tpl_13342::<0x14387e6b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x140ab1c60> as *const ()), // sub_140ab6770
    Replacement::draft_throws(0x140ab6820, "unknown_140ab6820", fn_140ab6820 as *const (), Throws::Caught), // sub_140ab6820
    Replacement::draft(0x140ab69d0, "unknown_140ab69d0", tpl_13342::<0x14387e6b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x140ab1f30> as *const ()), // sub_140ab69d0
    Replacement::draft_throws(0x140ab6a80, "unknown_140ab6a80", fn_140ab6a80 as *const (), Throws::Caught), // sub_140ab6a80
    Replacement::draft_throws(0x140ab6c30, "unknown_140ab6c30", fn_140ab6c30 as *const (), Throws::Caught), // sub_140ab6c30
    Replacement::draft_throws(0x140ab6cf0, "unknown_140ab6cf0", fn_140ab6cf0 as *const (), Throws::Caught), // sub_140ab6cf0
    Replacement::draft_throws(0x140ab6db0, "unknown_140ab6db0", fn_140ab6db0 as *const (), Throws::Caught), // sub_140ab6db0
    Replacement::draft(0x140ab6e70, "unknown_140ab6e70", tpl_13344::<0x14387e6b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xb8, 0x140ab2d50> as *const ()), // sub_140ab6e70
    Replacement::draft(0x140ab6f30, "unknown_140ab6f30", tpl_9842::<0x14387e6c0, 0x14387e6b8, 0x1401e9970, 0x143881f88, 0x143881f80, 0x1401e9860, 0xb8, 0x140ab2e60> as *const ()), // sub_140ab6f30
    Replacement::draft(0x140ab6fc0, "unknown_140ab6fc0", tpl_13342::<0x14387e6b8, 0x14243b0b0, 0x143881f80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0xb8, 0x140ab2f30> as *const ()), // sub_140ab6fc0
    Replacement::draft_throws(0x140ab7070, "unknown_140ab7070", tpl_9843::<0x14387e6c0, 0x14387e6b8, 0x1401e9770, 0x143881f88, 0x143881f80, 0x1401e9590, 0xb8, 0x140ab3010> as *const (), Throws::Discard), // sub_140ab7070
    Replacement::draft_throws(0x140ab7100, "unknown_140ab7100", fn_140ab7100 as *const (), Throws::Caught), // sub_140ab7100
    Replacement::draft(0x140ab7220, "unknown_140ab7220", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ab510, 0x8> as *const ()), // sub_140ab7220
    Replacement::draft(0x140ab72b0, "unknown_140ab72b0", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ab510, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140ab72b0
    Replacement::draft(0x140ab7360, "unknown_140ab7360", fn_140ab7360 as *const ()), // sub_140ab7360
    Replacement::draft(0x140ab7430, "unknown_140ab7430", tpl_11443::<0x14387e6a0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140853710, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ab7430
    Replacement::draft_throws(0x140ab74b0, "unknown_140ab74b0", tpl_9669::<0x14387e718, 0x14387e6a0, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x48, 0x8, 0x1406a55b0, 0x1408538c0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140ab74b0
    Replacement::draft(0x140ab7560, "unknown_140ab7560", fn_140ab7560 as *const ()), // sub_140ab7560
    Replacement::draft_throws(0x140ab7630, "unknown_140ab7630", tpl_9670::<0x14387e718, 0x14387e6a0, 0x1401e9b60, 0x1406e03a0, 0x8, 0x328, 0x338, 0x120, 0x1406a55b0, 0x140853b90, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_140ab7630
    Replacement::draft(0x140ab7700, "unknown_140ab7700", tpl_12321::<0x8, 0x1406a55b0> as *const ()), // sub_140ab7700
    Replacement::draft_throws(0x140ab7760, "unknown_140ab7760", fn_140ab7760 as *const (), Throws::Caught), // sub_140ab7760
    Replacement::draft_throws(0x140ab7810, "unknown_140ab7810", fn_140ab7810 as *const (), Throws::Caught), // sub_140ab7810
    Replacement::draft_throws(0x140ab78c0, "unknown_140ab78c0", fn_140ab78c0 as *const (), Throws::Caught), // sub_140ab78c0
    Replacement::draft(0x140ab7970, "unknown_140ab7970", fn_140ab7970 as *const ()), // sub_140ab7970
    Replacement::draft(0x140ab7a30, "unknown_140ab7a30", fn_140ab7a30 as *const ()), // sub_140ab7a30
    Replacement::draft(0x140ab7ad0, "unknown_140ab7ad0", fn_140ab7ad0 as *const ()), // sub_140ab7ad0
    Replacement::draft_throws(0x140ab7ba0, "unknown_140ab7ba0", fn_140ab7ba0 as *const (), Throws::Discard), // sub_140ab7ba0
    Replacement::draft_throws(0x140ab7c50, "unknown_140ab7c50", tpl_11858::<0x14387e6a0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432aae78, 0x1408537e0> as *const (), Throws::Discard), // sub_140ab7c50
    Replacement::draft_throws(0x140ab7d20, "unknown_140ab7d20", fn_140ab7d20 as *const (), Throws::Discard), // sub_140ab7d20
    Replacement::draft_throws(0x140ab7d90, "unknown_140ab7d90", fn_140ab7d90 as *const (), Throws::Discard), // sub_140ab7d90
    Replacement::draft_throws(0x140ab7e30, "unknown_140ab7e30", tpl_65670::<0x142442de0, 0x143a3da08, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x60> as *const (), Throws::Discard), // sub_140ab7e30
    Replacement::draft(0x140ab7f10, "unknown_140ab7f10", tpl_13347::<0x8, 0x14240ab10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140ab7f10
    Replacement::draft(0x140ab7fe0, "unknown_140ab7fe0", fn_140ab7fe0 as *const ()), // sub_140ab7fe0
    Replacement::draft_throws(0x140ab80b0, "unknown_140ab80b0", fn_140ab80b0 as *const (), Throws::Caught), // sub_140ab80b0
    Replacement::draft_throws(0x140ab8350, "unknown_140ab8350", fn_140ab8350 as *const (), Throws::Caught), // sub_140ab8350
    Replacement::draft_throws(0x140ab85f0, "unknown_140ab85f0", fn_140ab85f0 as *const (), Throws::Caught), // sub_140ab85f0
    Replacement::draft_throws(0x140ab8870, "unknown_140ab8870", fn_140ab8870 as *const (), Throws::Caught), // sub_140ab8870
    Replacement::draft_throws(0x140ab8af0, "unknown_140ab8af0", fn_140ab8af0 as *const (), Throws::Caught), // sub_140ab8af0
    Replacement::draft_throws(0x140ab8d70, "unknown_140ab8d70", fn_140ab8d70 as *const (), Throws::Caught), // sub_140ab8d70
    Replacement::draft_throws(0x140ab8e40, "unknown_140ab8e40", tpl_838::<0x140abeff0> as *const (), Throws::Discard), // sub_140ab8e40
    Replacement::draft(0x140ab8e50, "unknown_140ab8e50", tpl_13352::<0x18> as *const ()), // sub_140ab8e50
    Replacement::draft(0x140ab8e70, "unknown_140ab8e70", tpl_13352::<0x8> as *const ()), // sub_140ab8e70
    Replacement::draft_throws(0x140ab8e90, "unknown_140ab8e90", tpl_838::<0x140ac06b0> as *const (), Throws::Discard), // sub_140ab8e90
    Replacement::draft(0x140ab8ea0, "unknown_140ab8ea0", fn_140ab8ea0 as *const ()), // sub_140ab8ea0
    Replacement::draft(0x140ab8f80, "unknown_140ab8f80", fn_140ab8f80 as *const ()), // sub_140ab8f80
    Replacement::draft(0x140ab9060, "unknown_140ab9060", fn_140ab9060 as *const ()), // sub_140ab9060
    Replacement::draft_throws(0x140ab9140, "unknown_140ab9140", tpl_838::<0x140ac1460> as *const (), Throws::Discard), // sub_140ab9140
    Replacement::draft_throws(0x140ab9150, "unknown_140ab9150", tpl_838::<0x140ac1dc0> as *const (), Throws::Discard), // sub_140ab9150
    Replacement::draft_throws(0x140ab9160, "unknown_140ab9160", tpl_838::<0x140ac26e0> as *const (), Throws::Discard), // sub_140ab9160
    Replacement::draft(0x140ab9170, "unknown_140ab9170", fn_140ab9170 as *const ()), // sub_140ab9170
    Replacement::draft(0x140ab9210, "unknown_140ab9210", fn_140ab9210 as *const ()), // sub_140ab9210
    Replacement::draft_throws(0x140ab9290, "unknown_140ab9290", tpl_838::<0x140ac2c30> as *const (), Throws::Discard), // sub_140ab9290
    Replacement::draft_throws(0x140ab92a0, "unknown_140ab92a0", tpl_6985::<0x20, 0x8, 0x140ac2e90> as *const (), Throws::Discard), // sub_140ab92a0
    Replacement::draft_throws(0x140ab92f0, "unknown_140ab92f0", tpl_838::<0x140ac3590> as *const (), Throws::Discard), // sub_140ab92f0
    Replacement::draft_throws(0x140ab9300, "unknown_140ab9300", tpl_838::<0x140ac3a20> as *const (), Throws::Discard), // sub_140ab9300
    Replacement::draft_throws(0x140ab9310, "unknown_140ab9310", tpl_838::<0x140ac3e70> as *const (), Throws::Discard), // sub_140ab9310
    Replacement::draft_throws(0x140ab9320, "unknown_140ab9320", tpl_12664::<0x20, 0x8, 0x140ac42b0> as *const (), Throws::Discard), // sub_140ab9320
    Replacement::draft_throws(0x140ab9370, "unknown_140ab9370", fn_140ab9370 as *const (), Throws::Caught), // sub_140ab9370
    Replacement::draft_throws(0x140ab9670, "unknown_140ab9670", tpl_65672::<0x142442de0, 0x143a3da08, 0x1425371a0, 0x142533200, 0x1424b14c0, 0x1424b0d90, 0x1424b0060> as *const (), Throws::Discard), // sub_140ab9670
    Replacement::draft_throws(0x140ab9890, "unknown_140ab9890", fn_140ab9890 as *const (), Throws::Caught), // sub_140ab9890
    Replacement::draft_throws(0x140ab9b80, "unknown_140ab9b80", fn_140ab9b80 as *const (), Throws::Caught), // sub_140ab9b80
    Replacement::draft_throws(0x140aba640, "unknown_140aba640", fn_140aba640 as *const (), Throws::Caught), // sub_140aba640
    Replacement::draft_throws(0x140abb0d0, "unknown_140abb0d0", fn_140abb0d0 as *const (), Throws::Caught), // sub_140abb0d0
    Replacement::draft_throws(0x140abbb50, "unknown_140abbb50", fn_140abbb50 as *const (), Throws::Caught), // sub_140abbb50
    Replacement::draft_throws(0x140abbd50, "unknown_140abbd50", tpl_838::<0x140ac7300> as *const (), Throws::Discard), // sub_140abbd50
    Replacement::draft_throws(0x140abbd60, "unknown_140abbd60", tpl_838::<0x140ac7500> as *const (), Throws::Discard), // sub_140abbd60
    Replacement::draft_throws(0x140abbd70, "unknown_140abbd70", fn_140abbd70 as *const (), Throws::Caught), // sub_140abbd70
    Replacement::draft_throws(0x140abbeb0, "unknown_140abbeb0", fn_140abbeb0 as *const (), Throws::Caught), // sub_140abbeb0
    Replacement::draft_throws(0x140abbff0, "unknown_140abbff0", fn_140abbff0 as *const (), Throws::Caught), // sub_140abbff0
    Replacement::draft(0x140abc130, "unknown_140abc130", fn_140abc130 as *const ()), // sub_140abc130
    Replacement::draft(0x140abc1d0, "unknown_140abc1d0", tpl_60956::<0x19, 0x10, 0x140abc1d0, 0x20, 0x140ac8850, 0x1401e66f0> as *const ()), // sub_140abc1d0
    Replacement::draft(0x140abc300, "std_140abc300", tpl_1::<0x14388f120> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActorResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl
    Replacement::draft(0x140abc320, "std_140abc320", tpl_550::<0x1432ab4a0, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActorResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl
    Replacement::draft(0x140abc340, "std_140abc340", tpl_1::<0x14388f050> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActorResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_imp
    Replacement::draft(0x140abc350, "std_140abc350", tpl_550::<0x1432ab4d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActorResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_imp
    Replacement::draft(0x140abc370, "std_140abc370", tpl_1::<0x14388f0b0> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActionTableResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Fun
    Replacement::draft(0x140abc390, "std_140abc390", tpl_550::<0x1432abbb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActionTableResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Fun
    Replacement::draft(0x140abc3b0, "std_140abc3b0", tpl_1::<0x14388f180> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActionTableResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Fu
    Replacement::draft(0x140abc3c0, "std_140abc3c0", tpl_550::<0x1432ac830, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::ActionSystemActionTableResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Fu
    Replacement::draft(0x140abc3e0, "std_140abc3e0", fn_140abc3e0 as *const ()), // sub_140abc3e0
    Replacement::draft(0x140abc460, "std_140abc460", fn_140abc460 as *const ()), // sub_140abc460
    Replacement::draft(0x140abc590, "std_140abc590", fn_140abc590 as *const ()), // sub_140abc590
    Replacement::draft_throws(0x140abc6c0, "std_140abc6c0", fn_140abc6c0 as *const (), Throws::Discard), // sub_140abc6c0
    Replacement::draft_throws(0x140abc8b0, "std_140abc8b0", fn_140abc8b0 as *const (), Throws::Caught), // sub_140abc8b0
    Replacement::draft_throws(0x140abd050, "std_140abd050", fn_140abd050 as *const (), Throws::Caught), // sub_140abd050
    Replacement::draft_throws(0x140abd6f0, "std_140abd6f0", fn_140abd6f0 as *const (), Throws::Caught), // sub_140abd6f0
    Replacement::draft_throws(0x140abde60, "std_140abde60", fn_140abde60 as *const (), Throws::Caught), // sub_140abde60
    Replacement::draft_throws(0x140abe4b0, "std_140abe4b0", fn_140abe4b0 as *const (), Throws::Caught), // sub_140abe4b0
    Replacement::draft_throws(0x140abe9a0, "std_140abe9a0", fn_140abe9a0 as *const (), Throws::Caught), // sub_140abe9a0
    Replacement::draft_throws(0x140abeff0, "std_140abeff0", fn_140abeff0 as *const (), Throws::Caught), // sub_140abeff0
    Replacement::draft_throws(0x140abf240, "std_140abf240", fn_140abf240 as *const (), Throws::Discard), // sub_140abf240
    Replacement::draft_throws(0x140abf3d0, "unknown_140abf3d0", fn_140abf3d0 as *const (), Throws::Caught), // sub_140abf3d0
    Replacement::draft_throws(0x140abfa50, "unknown_140abfa50", fn_140abfa50 as *const (), Throws::Caught), // sub_140abfa50
    Replacement::draft_throws(0x140ac0080, "unknown_140ac0080", fn_140ac0080 as *const (), Throws::Caught), // sub_140ac0080
    Replacement::draft_throws(0x140ac06b0, "unknown_140ac06b0", fn_140ac06b0 as *const (), Throws::Caught), // sub_140ac06b0
    Replacement::draft_throws(0x140ac08a0, "unknown_140ac08a0", fn_140ac08a0 as *const (), Throws::Discard), // sub_140ac08a0
    Replacement::draft_throws(0x140ac0b30, "unknown_140ac0b30", fn_140ac0b30 as *const (), Throws::Discard), // sub_140ac0b30
    Replacement::draft_throws(0x140ac0dc0, "unknown_140ac0dc0", fn_140ac0dc0 as *const (), Throws::Caught), // sub_140ac0dc0
    Replacement::draft_throws(0x140ac1460, "unknown_140ac1460", fn_140ac1460 as *const (), Throws::Caught), // sub_140ac1460
    Replacement::draft_throws(0x140ac1770, "unknown_140ac1770", fn_140ac1770 as *const (), Throws::Caught), // sub_140ac1770
    Replacement::draft_throws(0x140ac1dc0, "unknown_140ac1dc0", fn_140ac1dc0 as *const (), Throws::Caught), // sub_140ac1dc0
    Replacement::draft_throws(0x140ac2090, "unknown_140ac2090", fn_140ac2090 as *const (), Throws::Caught), // sub_140ac2090
    Replacement::draft_throws(0x140ac26e0, "unknown_140ac26e0", fn_140ac26e0 as *const (), Throws::Caught), // sub_140ac26e0
    Replacement::draft_throws(0x140ac29a0, "unknown_140ac29a0", fn_140ac29a0 as *const (), Throws::Discard), // sub_140ac29a0
    Replacement::draft_throws(0x140ac2c30, "unknown_140ac2c30", fn_140ac2c30 as *const (), Throws::Caught), // sub_140ac2c30
    Replacement::draft_throws(0x140ac2e90, "unknown_140ac2e90", fn_140ac2e90 as *const (), Throws::Caught), // sub_140ac2e90
    Replacement::draft_throws(0x140ac2f90, "unknown_140ac2f90", fn_140ac2f90 as *const (), Throws::Caught), // sub_140ac2f90
    Replacement::draft_throws(0x140ac3290, "unknown_140ac3290", fn_140ac3290 as *const (), Throws::Caught), // sub_140ac3290
    Replacement::draft_throws(0x140ac3590, "unknown_140ac3590", fn_140ac3590 as *const (), Throws::Caught), // sub_140ac3590
    Replacement::draft_throws(0x140ac3a20, "unknown_140ac3a20", fn_140ac3a20 as *const (), Throws::Caught), // sub_140ac3a20
    Replacement::draft_throws(0x140ac3e70, "unknown_140ac3e70", fn_140ac3e70 as *const (), Throws::Caught), // sub_140ac3e70
    Replacement::draft_throws(0x140ac42b0, "unknown_140ac42b0", fn_140ac42b0 as *const (), Throws::Caught), // sub_140ac42b0
    Replacement::draft_throws(0x140ac4380, "unknown_140ac4380", fn_140ac4380 as *const (), Throws::Caught), // sub_140ac4380
    Replacement::draft_throws(0x140ac44c0, "unknown_140ac44c0", fn_140ac44c0 as *const (), Throws::Discard), // sub_140ac44c0
    Replacement::draft_throws(0x140ac4680, "unknown_140ac4680", fn_140ac4680 as *const (), Throws::Discard), // sub_140ac4680
    Replacement::draft_throws(0x140ac48f0, "unknown_140ac48f0", fn_140ac48f0 as *const (), Throws::Discard), // sub_140ac48f0
    Replacement::draft_throws(0x140ac4ab0, "unknown_140ac4ab0", fn_140ac4ab0 as *const (), Throws::Caught), // sub_140ac4ab0
    Replacement::draft_throws(0x140ac5150, "unknown_140ac5150", fn_140ac5150 as *const (), Throws::Caught), // sub_140ac5150
    Replacement::draft_throws(0x140ac57f0, "unknown_140ac57f0", fn_140ac57f0 as *const (), Throws::Caught), // sub_140ac57f0
    Replacement::draft_throws(0x140ac5e40, "unknown_140ac5e40", fn_140ac5e40 as *const (), Throws::Caught), // sub_140ac5e40
    Replacement::draft_throws(0x140ac6490, "unknown_140ac6490", fn_140ac6490 as *const (), Throws::Caught), // sub_140ac6490
    Replacement::draft_throws(0x140ac6ae0, "unknown_140ac6ae0", fn_140ac6ae0 as *const (), Throws::Caught), // sub_140ac6ae0
    Replacement::draft_throws(0x140ac7130, "unknown_140ac7130", fn_140ac7130 as *const (), Throws::Discard), // sub_140ac7130
    Replacement::draft_throws(0x140ac7300, "unknown_140ac7300", fn_140ac7300 as *const (), Throws::Caught), // sub_140ac7300
    Replacement::draft_throws(0x140ac7500, "unknown_140ac7500", fn_140ac7500 as *const (), Throws::Caught), // sub_140ac7500
    Replacement::draft_throws(0x140ac76a0, "unknown_140ac76a0", fn_140ac76a0 as *const (), Throws::Caught), // sub_140ac76a0
    Replacement::draft_throws(0x140ac7c50, "unknown_140ac7c50", fn_140ac7c50 as *const (), Throws::Caught), // sub_140ac7c50
    Replacement::draft_throws(0x140ac81c0, "unknown_140ac81c0", fn_140ac81c0 as *const (), Throws::Caught), // sub_140ac81c0
    Replacement::draft_throws(0x140ac8600, "unknown_140ac8600", fn_140ac8600 as *const (), Throws::Discard), // sub_140ac8600
    Replacement::draft(0x140ac86c0, "unknown_140ac86c0", fn_140ac86c0 as *const ()), // sub_140ac86c0
    Replacement::draft_throws(0x140ac87b0, "unknown_140ac87b0", fn_140ac87b0 as *const (), Throws::Discard), // sub_140ac87b0
    Replacement::draft(0x140ac8850, "unknown_140ac8850", fn_140ac8850 as *const ()), // sub_140ac8850
    Replacement::draft(0x140ac88c0, "unknown_140ac88c0", fn_140ac88c0 as *const ()), // sub_140ac88c0
    Replacement::draft(0x140ac8910, "unknown_140ac8910", fn_140ac8910 as *const ()), // sub_140ac8910
    Replacement::draft(0x140ac8950, "unknown_140ac8950", fn_140ac8950 as *const ()), // sub_140ac8950
    Replacement::draft(0x140ac8990, "unknown_140ac8990", fn_140ac8990 as *const ()), // sub_140ac8990
    Replacement::draft(0x140ac8a00, "unknown_140ac8a00", fn_140ac8a00 as *const ()), // sub_140ac8a00
    Replacement::draft(0x140ac8a70, "unknown_140ac8a70", tpl_1037::<0x8, 0x140aade00> as *const ()), // sub_140ac8a70
    Replacement::draft_throws(0x140ac8ab0, "unknown_140ac8ab0", fn_140ac8ab0 as *const (), Throws::Discard), // sub_140ac8ab0
    Replacement::draft_throws(0x140ac8bf0, "unknown_140ac8bf0", fn_140ac8bf0 as *const (), Throws::Discard), // sub_140ac8bf0
    Replacement::draft(0x140ac8d30, "unknown_140ac8d30", fn_140ac8d30 as *const ()), // sub_140ac8d30
    Replacement::draft(0x140ac8f50, "unknown_140ac8f50", fn_140ac8f50 as *const ()), // sub_140ac8f50
    Replacement::draft_throws(0x140ac91a0, "unknown_140ac91a0", fn_140ac91a0 as *const (), Throws::Caught), // sub_140ac91a0
    Replacement::draft_throws(0x140ac99b0, "unknown_140ac99b0", fn_140ac99b0 as *const (), Throws::Discard), // sub_140ac99b0
    Replacement::draft_throws(0x140aca2c0, "unknown_140aca2c0", fn_140aca2c0 as *const (), Throws::Caught), // sub_140aca2c0
    Replacement::draft_throws(0x140acab10, "unknown_140acab10", fn_140acab10 as *const (), Throws::Caught), // sub_140acab10
    Replacement::draft_throws(0x140acb490, "unknown_140acb490", fn_140acb490 as *const (), Throws::Caught), // sub_140acb490
    Replacement::draft_throws(0x140acbe10, "unknown_140acbe10", fn_140acbe10 as *const (), Throws::Caught), // sub_140acbe10
    Replacement::draft_throws(0x140acc790, "unknown_140acc790", fn_140acc790 as *const (), Throws::Caught), // sub_140acc790
    Replacement::draft_throws(0x140acced0, "unknown_140acced0", fn_140acced0 as *const (), Throws::Discard), // sub_140acced0
    Replacement::draft_throws(0x140acd5d0, "unknown_140acd5d0", fn_140acd5d0 as *const (), Throws::Caught), // sub_140acd5d0
    Replacement::draft_throws(0x140acdd20, "unknown_140acdd20", fn_140acdd20 as *const (), Throws::Caught), // sub_140acdd20
    Replacement::draft_throws(0x140ace520, "unknown_140ace520", fn_140ace520 as *const (), Throws::Caught), // sub_140ace520
    Replacement::draft_throws(0x140aced20, "unknown_140aced20", fn_140aced20 as *const (), Throws::Caught), // sub_140aced20
    Replacement::draft_throws(0x140acf520, "unknown_140acf520", fn_140acf520 as *const (), Throws::Caught), // sub_140acf520
    Replacement::draft_throws(0x140acfe90, "unknown_140acfe90", fn_140acfe90 as *const (), Throws::Discard), // sub_140acfe90
    Replacement::draft_throws(0x140ad0700, "unknown_140ad0700", fn_140ad0700 as *const (), Throws::Caught), // sub_140ad0700
    Replacement::draft_throws(0x140ad1160, "unknown_140ad1160", fn_140ad1160 as *const (), Throws::Caught), // sub_140ad1160
    Replacement::draft_throws(0x140ad1a40, "unknown_140ad1a40", fn_140ad1a40 as *const (), Throws::Caught), // sub_140ad1a40
    Replacement::draft_throws(0x140ad2010, "unknown_140ad2010", fn_140ad2010 as *const (), Throws::Caught), // sub_140ad2010
    Replacement::draft_throws(0x140ad28f0, "unknown_140ad28f0", fn_140ad28f0 as *const (), Throws::Caught), // sub_140ad28f0
    Replacement::draft_throws(0x140ad2e50, "unknown_140ad2e50", fn_140ad2e50 as *const (), Throws::Caught), // sub_140ad2e50
    Replacement::draft_throws(0x140ad3730, "unknown_140ad3730", fn_140ad3730 as *const (), Throws::Caught), // sub_140ad3730
    Replacement::draft_throws(0x140ad3c80, "unknown_140ad3c80", fn_140ad3c80 as *const (), Throws::Caught), // sub_140ad3c80
    Replacement::draft_throws(0x140ad4150, "unknown_140ad4150", fn_140ad4150 as *const (), Throws::Caught), // sub_140ad4150
    Replacement::draft_throws(0x140ad45e0, "unknown_140ad45e0", fn_140ad45e0 as *const (), Throws::Caught), // sub_140ad45e0
    Replacement::draft_throws(0x140ad4a70, "unknown_140ad4a70", fn_140ad4a70 as *const (), Throws::Caught), // sub_140ad4a70
    Replacement::draft_throws(0x140ad5020, "unknown_140ad5020", fn_140ad5020 as *const (), Throws::Caught), // sub_140ad5020
    Replacement::draft_throws(0x140ad58f0, "unknown_140ad58f0", fn_140ad58f0 as *const (), Throws::Discard), // sub_140ad58f0
    Replacement::draft_throws(0x140ad6080, "unknown_140ad6080", fn_140ad6080 as *const (), Throws::Discard), // sub_140ad6080
    Replacement::draft_throws(0x140ad67a0, "unknown_140ad67a0", fn_140ad67a0 as *const (), Throws::Discard), // sub_140ad67a0
    Replacement::draft_throws(0x140ad6e70, "unknown_140ad6e70", fn_140ad6e70 as *const (), Throws::Caught), // sub_140ad6e70
    Replacement::draft_throws(0x140ad7710, "unknown_140ad7710", fn_140ad7710 as *const (), Throws::Discard), // sub_140ad7710
    Replacement::draft_throws(0x140ad7e90, "unknown_140ad7e90", fn_140ad7e90 as *const (), Throws::Caught), // sub_140ad7e90
    Replacement::draft_throws(0x140ad8680, "unknown_140ad8680", fn_140ad8680 as *const (), Throws::Caught), // sub_140ad8680
    Replacement::draft_throws(0x140ad8e30, "unknown_140ad8e30", fn_140ad8e30 as *const (), Throws::Caught), // sub_140ad8e30
    Replacement::draft_throws(0x140ad9620, "unknown_140ad9620", fn_140ad9620 as *const (), Throws::Caught), // sub_140ad9620
    Replacement::draft_throws(0x140ad9dd0, "unknown_140ad9dd0", fn_140ad9dd0 as *const (), Throws::Caught), // sub_140ad9dd0
    Replacement::draft_throws(0x140ada5c0, "unknown_140ada5c0", fn_140ada5c0 as *const (), Throws::Caught), // sub_140ada5c0
    Replacement::draft_throws(0x140adaea0, "unknown_140adaea0", fn_140adaea0 as *const (), Throws::Discard), // sub_140adaea0
    Replacement::draft_throws(0x140adb030, "unknown_140adb030", tpl_13418::<0x140add020> as *const (), Throws::Discard), // sub_140adb030
    Replacement::draft_throws(0x140adb040, "unknown_140adb040", tpl_13418::<0x140add080> as *const (), Throws::Discard), // sub_140adb040
    Replacement::draft_throws(0x140adb050, "unknown_140adb050", tpl_13418::<0x140add0e0> as *const (), Throws::Discard), // sub_140adb050
    Replacement::draft_throws(0x140adb060, "unknown_140adb060", tpl_13418::<0x140add140> as *const (), Throws::Discard), // sub_140adb060
    Replacement::draft_throws(0x140adb070, "unknown_140adb070", tpl_13418::<0x140add1e0> as *const (), Throws::Discard), // sub_140adb070
    Replacement::draft(0x140adb080, "unknown_140adb080", tpl_13419::<0x140267a30, 0x1403f7860, 0x14240ab10, 0x1401e66f0> as *const ()), // sub_140adb080
    Replacement::draft(0x140adb220, "unknown_140adb220", tpl_13419::<0x140abc1d0, 0x1403f7860, 0x140ac8850, 0x1401e66f0> as *const ()), // sub_140adb220
    Replacement::draft(0x140adb3c0, "unknown_140adb3c0", fn_140adb3c0 as *const ()), // sub_140adb3c0
    Replacement::draft(0x140adb560, "unknown_140adb560", fn_140adb560 as *const ()), // sub_140adb560
    Replacement::draft(0x140adb6e0, "unknown_140adb6e0", tpl_12880::<0x8, 0x14024a9e0, 0x140add240, 0x140ab7f10, 0xc> as *const ()), // sub_140adb6e0
    Replacement::draft(0x140adb750, "unknown_140adb750", fn_140adb750 as *const ()), // sub_140adb750
    Replacement::draft_throws(0x140adb8f0, "unknown_140adb8f0", fn_140adb8f0 as *const (), Throws::Discard), // sub_140adb8f0
    Replacement::draft_throws(0x140adba40, "unknown_140adba40", fn_140adba40 as *const (), Throws::Discard), // sub_140adba40
    Replacement::draft_throws(0x140adbab0, "unknown_140adbab0", fn_140adbab0 as *const (), Throws::Caught), // sub_140adbab0
    Replacement::draft_throws(0x140adbbf0, "unknown_140adbbf0", fn_140adbbf0 as *const (), Throws::Discard), // sub_140adbbf0
    Replacement::draft_throws(0x140adbdd0, "unknown_140adbdd0", fn_140adbdd0 as *const (), Throws::Caught), // sub_140adbdd0
    Replacement::draft_throws(0x140adbf10, "unknown_140adbf10", fn_140adbf10 as *const (), Throws::Caught), // sub_140adbf10
    Replacement::draft_throws(0x140adc190, "unknown_140adc190", fn_140adc190 as *const (), Throws::Caught), // sub_140adc190
    Replacement::draft_throws(0x140adc400, "unknown_140adc400", fn_140adc400 as *const (), Throws::Caught), // sub_140adc400
    Replacement::draft_throws(0x140adc670, "unknown_140adc670", fn_140adc670 as *const (), Throws::Caught), // sub_140adc670
    Replacement::draft_throws(0x140adc900, "unknown_140adc900", fn_140adc900 as *const (), Throws::Caught), // sub_140adc900
    Replacement::draft_throws(0x140adca40, "unknown_140adca40", fn_140adca40 as *const (), Throws::Discard), // sub_140adca40
    Replacement::draft_throws(0x140adcbb0, "unknown_140adcbb0", fn_140adcbb0 as *const (), Throws::Discard), // sub_140adcbb0
    Replacement::draft_throws(0x140adcd00, "unknown_140adcd00", fn_140adcd00 as *const (), Throws::Caught), // sub_140adcd00
    Replacement::draft_throws(0x140adce40, "unknown_140adce40", fn_140adce40 as *const (), Throws::Caught), // sub_140adce40
    Replacement::draft(0x140adcfe0, "unknown_140adcfe0", tpl_13432::<0x8, 0x14240ab10> as *const ()), // sub_140adcfe0
    Replacement::draft_throws(0x140add020, "unknown_140add020", fn_140add020 as *const (), Throws::Discard), // sub_140add020
    Replacement::draft_throws(0x140add080, "unknown_140add080", fn_140add080 as *const (), Throws::Discard), // sub_140add080
    Replacement::draft_throws(0x140add0e0, "unknown_140add0e0", tpl_5147::<0x8, 0x140ae2970, 0x14241c190> as *const (), Throws::Discard), // sub_140add0e0
    Replacement::draft_throws(0x140add140, "unknown_140add140", fn_140add140 as *const (), Throws::Discard), // sub_140add140
    Replacement::draft_throws(0x140add1e0, "unknown_140add1e0", fn_140add1e0 as *const (), Throws::Discard), // sub_140add1e0
    Replacement::draft(0x140add240, "unknown_140add240", fn_140add240 as *const ()), // sub_140add240
    Replacement::draft_throws(0x140add2c0, "unknown_140add2c0", fn_140add2c0 as *const (), Throws::Caught), // sub_140add2c0
    Replacement::draft_throws(0x140add510, "unknown_140add510", fn_140add510 as *const (), Throws::Caught), // sub_140add510
    Replacement::draft_throws(0x140add760, "unknown_140add760", fn_140add760 as *const (), Throws::Caught), // sub_140add760
    Replacement::draft_throws(0x140add9b0, "unknown_140add9b0", fn_140add9b0 as *const (), Throws::Caught), // sub_140add9b0
    Replacement::draft_throws(0x140addc00, "unknown_140addc00", fn_140addc00 as *const (), Throws::Caught), // sub_140addc00
    Replacement::draft_throws(0x140adde50, "unknown_140adde50", fn_140adde50 as *const (), Throws::Caught), // sub_140adde50
    Replacement::draft_throws(0x140addf90, "unknown_140addf90", fn_140addf90 as *const (), Throws::Caught), // sub_140addf90
    Replacement::draft_throws(0x140ade0d0, "unknown_140ade0d0", fn_140ade0d0 as *const (), Throws::Caught), // sub_140ade0d0
    Replacement::draft_throws(0x140ade210, "unknown_140ade210", fn_140ade210 as *const (), Throws::Caught), // sub_140ade210
    Replacement::draft_throws(0x140ade7f0, "unknown_140ade7f0", fn_140ade7f0 as *const (), Throws::Caught), // sub_140ade7f0
    Replacement::draft_throws(0x140aded90, "unknown_140aded90", fn_140aded90 as *const (), Throws::Caught), // sub_140aded90
    Replacement::draft_throws(0x140adf0e0, "unknown_140adf0e0", fn_140adf0e0 as *const (), Throws::Caught), // sub_140adf0e0
    Replacement::draft_throws(0x140adf220, "unknown_140adf220", fn_140adf220 as *const (), Throws::Caught), // sub_140adf220
    Replacement::draft_throws(0x140adf360, "unknown_140adf360", fn_140adf360 as *const (), Throws::Caught), // sub_140adf360
    Replacement::draft_throws(0x140adf4a0, "unknown_140adf4a0", fn_140adf4a0 as *const (), Throws::Caught), // sub_140adf4a0
    Replacement::draft_throws(0x140adfcc0, "unknown_140adfcc0", fn_140adfcc0 as *const (), Throws::Caught), // sub_140adfcc0
    Replacement::draft_throws(0x140ae0470, "unknown_140ae0470", fn_140ae0470 as *const (), Throws::Caught), // sub_140ae0470
    Replacement::draft_throws(0x140ae0ab0, "unknown_140ae0ab0", fn_140ae0ab0 as *const (), Throws::Caught), // sub_140ae0ab0
    Replacement::draft_throws(0x140ae1060, "unknown_140ae1060", fn_140ae1060 as *const (), Throws::Caught), // sub_140ae1060
    Replacement::draft_throws(0x140ae15d0, "unknown_140ae15d0", fn_140ae15d0 as *const (), Throws::Caught), // sub_140ae15d0
    Replacement::draft_throws(0x140ae1a10, "unknown_140ae1a10", fn_140ae1a10 as *const (), Throws::Caught), // sub_140ae1a10
    Replacement::draft_throws(0x140ae1fc0, "unknown_140ae1fc0", fn_140ae1fc0 as *const (), Throws::Caught), // sub_140ae1fc0
    Replacement::draft_throws(0x140ae2530, "unknown_140ae2530", fn_140ae2530 as *const (), Throws::Caught), // sub_140ae2530
    Replacement::draft(0x140ae2970, "unknown_140ae2970", fn_140ae2970 as *const ()), // sub_140ae2970
    Replacement::draft(0x140ae2ad0, "unknown_140ae2ad0", fn_140ae2ad0 as *const ()), // sub_140ae2ad0
    Replacement::draft(0x140ae2b20, "unknown_140ae2b20", fn_140ae2b20 as *const ()), // sub_140ae2b20
    Replacement::draft(0x140ae2bb0, "unknown_140ae2bb0", fn_140ae2bb0 as *const ()), // sub_140ae2bb0
    Replacement::draft(0x140ae2c40, "unknown_140ae2c40", fn_140ae2c40 as *const ()), // sub_140ae2c40
    Replacement::draft(0x140ae2cf0, "unknown_140ae2cf0", fn_140ae2cf0 as *const ()), // sub_140ae2cf0
    Replacement::draft_throws(0x140ae2e20, "unknown_140ae2e20", fn_140ae2e20 as *const (), Throws::Discard), // sub_140ae2e20
    Replacement::draft(0x140ae2f10, "unknown_140ae2f10", tpl_13451::<0x14387e858, 0x14243b0b0, 0x338, 0x1401e8620, 0x18, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ae2f10
    Replacement::draft_throws(0x140ae2fb0, "unknown_140ae2fb0", fn_140ae2fb0 as *const (), Throws::Discard), // sub_140ae2fb0
    Replacement::draft_throws(0x140ae3040, "unknown_140ae3040", fn_140ae3040 as *const (), Throws::Caught), // sub_140ae3040
    Replacement::draft_throws(0x140ae3130, "unknown_140ae3130", fn_140ae3130 as *const (), Throws::Caught), // sub_140ae3130
    Replacement::draft_throws(0x140ae3220, "unknown_140ae3220", fn_140ae3220 as *const (), Throws::Caught), // sub_140ae3220
    Replacement::draft(0x140ae3310, "unknown_140ae3310", fn_140ae3310 as *const ()), // sub_140ae3310
    Replacement::draft(0x140ae3400, "unknown_140ae3400", tpl_2262::<0x14387e754, 0x14387e858, 0x1401e9970, 0x338, 0x88, 0x18, 0xe8, 0x1401e9860> as *const ()), // sub_140ae3400
    Replacement::draft(0x140ae3480, "unknown_140ae3480", tpl_2258::<0x14387e858, 0x14243b0b0, 0x338, 0x88, 0x18, 0xe8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ae3480
    Replacement::draft_throws(0x140ae3510, "unknown_140ae3510", tpl_13455::<0x14387e754, 0x14387e858, 0x1401e9770, 0x338, 0x120, 0x18, 0x1401e9590> as *const (), Throws::Discard), // sub_140ae3510
    Replacement::draft_throws(0x140ae35a0, "unknown_140ae35a0", fn_140ae35a0 as *const (), Throws::Discard), // sub_140ae35a0
    Replacement::draft(0x140ae36b0, "unknown_140ae36b0", tpl_13457::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x9, 0x1432add68, 0xd, 0xf, 0x8> as *const ()), // sub_140ae36b0
    Replacement::draft(0x140ae3750, "unknown_140ae3750", tpl_13458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432add68, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_140ae3750
    Replacement::draft(0x140ae3800, "unknown_140ae3800", fn_140ae3800 as *const ()), // sub_140ae3800
    Replacement::draft(0x140ae38c0, "unknown_140ae38c0", fn_140ae38c0 as *const ()), // sub_140ae38c0
    Replacement::draft_throws(0x140ae3930, "unknown_140ae3930", fn_140ae3930 as *const (), Throws::Discard), // sub_140ae3930
    Replacement::draft(0x140ae3a30, "unknown_140ae3a30", fn_140ae3a30 as *const ()), // sub_140ae3a30
    Replacement::draft_throws(0x140ae3b40, "unknown_140ae3b40", fn_140ae3b40 as *const (), Throws::Discard), // sub_140ae3b40
    Replacement::draft(0x140ae3c10, "unknown_140ae3c10", fn_140ae3c10 as *const ()), // sub_140ae3c10
    Replacement::draft_throws(0x140ae3cc0, "unknown_140ae3cc0", fn_140ae3cc0 as *const (), Throws::Caught), // sub_140ae3cc0
    Replacement::draft_throws(0x140ae3d80, "unknown_140ae3d80", fn_140ae3d80 as *const (), Throws::Caught), // sub_140ae3d80
    Replacement::draft_throws(0x140ae3e40, "unknown_140ae3e40", fn_140ae3e40 as *const (), Throws::Caught), // sub_140ae3e40
    Replacement::draft(0x140ae3f00, "unknown_140ae3f00", fn_140ae3f00 as *const ()), // sub_140ae3f00
    Replacement::draft(0x140ae4000, "unknown_140ae4000", fn_140ae4000 as *const ()), // sub_140ae4000
    Replacement::draft(0x140ae40c0, "unknown_140ae40c0", fn_140ae40c0 as *const ()), // sub_140ae40c0
    Replacement::draft_throws(0x140ae41c0, "unknown_140ae41c0", fn_140ae41c0 as *const (), Throws::Discard), // sub_140ae41c0
    Replacement::draft_throws(0x140ae42b0, "unknown_140ae42b0", fn_140ae42b0 as *const (), Throws::Discard), // sub_140ae42b0
    Replacement::draft(0x140ae4370, "unknown_140ae4370", fn_140ae4370 as *const ()), // sub_140ae4370
    Replacement::draft(0x140ae4400, "unknown_140ae4400", tpl_13469::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432b0278, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_140ae4400
    Replacement::draft(0x140ae44b0, "unknown_140ae44b0", fn_140ae44b0 as *const ()), // sub_140ae44b0
    Replacement::draft_throws(0x140ae4520, "unknown_140ae4520", fn_140ae4520 as *const (), Throws::Discard), // sub_140ae4520
    Replacement::draft(0x140ae45c0, "unknown_140ae45c0", fn_140ae45c0 as *const ()), // sub_140ae45c0
    Replacement::draft_throws(0x140ae4680, "unknown_140ae4680", fn_140ae4680 as *const (), Throws::Discard), // sub_140ae4680
    Replacement::draft(0x140ae4730, "unknown_140ae4730", fn_140ae4730 as *const ()), // sub_140ae4730
    Replacement::draft_throws(0x140ae47a0, "unknown_140ae47a0", fn_140ae47a0 as *const (), Throws::Caught), // sub_140ae47a0
    Replacement::draft_throws(0x140ae4850, "unknown_140ae4850", fn_140ae4850 as *const (), Throws::Caught), // sub_140ae4850
    Replacement::draft_throws(0x140ae4900, "unknown_140ae4900", fn_140ae4900 as *const (), Throws::Caught), // sub_140ae4900
    Replacement::draft(0x140ae49b0, "unknown_140ae49b0", fn_140ae49b0 as *const ()), // sub_140ae49b0
    Replacement::draft(0x140ae4a40, "unknown_140ae4a40", fn_140ae4a40 as *const ()), // sub_140ae4a40
    Replacement::draft(0x140ae4af0, "unknown_140ae4af0", fn_140ae4af0 as *const ()), // sub_140ae4af0
    Replacement::draft_throws(0x140ae4bb0, "unknown_140ae4bb0", fn_140ae4bb0 as *const (), Throws::Discard), // sub_140ae4bb0
    Replacement::draft_throws(0x140ae4c60, "unknown_140ae4c60", fn_140ae4c60 as *const (), Throws::Discard), // sub_140ae4c60
    Replacement::draft(0x140ae4d00, "unknown_140ae4d00", fn_140ae4d00 as *const ()), // sub_140ae4d00
    Replacement::draft(0x140ae4d70, "unknown_140ae4d70", fn_140ae4d70 as *const ()), // sub_140ae4d70
    Replacement::draft(0x140ae5030, "unknown_140ae5030", fn_140ae5030 as *const ()), // sub_140ae5030
    Replacement::draft_throws(0x140ae50e0, "unknown_140ae50e0", fn_140ae50e0 as *const (), Throws::Discard), // sub_140ae50e0
    Replacement::draft(0x140ae5300, "unknown_140ae5300", fn_140ae5300 as *const ()), // sub_140ae5300
    Replacement::draft_throws(0x140ae5540, "unknown_140ae5540", fn_140ae5540 as *const (), Throws::Discard), // sub_140ae5540
    Replacement::draft(0x140ae5820, "unknown_140ae5820", fn_140ae5820 as *const ()), // sub_140ae5820
    Replacement::draft_throws(0x140ae59e0, "unknown_140ae59e0", fn_140ae59e0 as *const (), Throws::Caught), // sub_140ae59e0
    Replacement::draft_throws(0x140ae5b10, "unknown_140ae5b10", fn_140ae5b10 as *const (), Throws::Caught), // sub_140ae5b10
    Replacement::draft_throws(0x140ae5c40, "unknown_140ae5c40", fn_140ae5c40 as *const (), Throws::Caught), // sub_140ae5c40
    Replacement::draft(0x140ae5d70, "unknown_140ae5d70", fn_140ae5d70 as *const ()), // sub_140ae5d70
    Replacement::draft(0x140ae5f90, "unknown_140ae5f90", fn_140ae5f90 as *const ()), // sub_140ae5f90
    Replacement::draft(0x140ae6120, "unknown_140ae6120", fn_140ae6120 as *const ()), // sub_140ae6120
    Replacement::draft_throws(0x140ae6350, "unknown_140ae6350", fn_140ae6350 as *const (), Throws::Discard), // sub_140ae6350
    Replacement::draft_throws(0x140ae6540, "unknown_140ae6540", fn_140ae6540 as *const (), Throws::Discard), // sub_140ae6540
    Replacement::draft(0x140ae6660, "unknown_140ae6660", fn_140ae6660 as *const ()), // sub_140ae6660
    Replacement::draft(0x140ae66d0, "unknown_140ae66d0", fn_140ae66d0 as *const ()), // sub_140ae66d0
    Replacement::draft(0x140ae6780, "unknown_140ae6780", fn_140ae6780 as *const ()), // sub_140ae6780
    Replacement::draft(0x140ae67f0, "unknown_140ae67f0", fn_140ae67f0 as *const ()), // sub_140ae67f0
    Replacement::draft(0x140ae68e0, "unknown_140ae68e0", fn_140ae68e0 as *const ()), // sub_140ae68e0
    Replacement::draft(0x140ae6950, "unknown_140ae6950", tpl_13492::<0x143426c70, 0x1432a62c0, 0x70, 0x1432a66a0, 0x14020f270, 0x143426a48, 0x1434269b0, 0x1432a66a8, 0x74, 0x1432a6678> as *const ()), // sub_140ae6950
    Replacement::draft(0x140ae6a00, "unknown_140ae6a00", fn_140ae6a00 as *const ()), // sub_140ae6a00
    Replacement::draft(0x140ae6b00, "unknown_140ae6b00", fn_140ae6b00 as *const ()), // sub_140ae6b00
    Replacement::draft(0x140ae6b80, "unknown_140ae6b80", tpl_13492::<0x143426c70, 0x1432a62c0, 0x98, 0x1432a66a0, 0x14020f270, 0x143426a48, 0x1434269b0, 0x1432a66a8, 0x9c, 0x1432a6678> as *const ()), // sub_140ae6b80
    Replacement::draft(0x140ae6c30, "unknown_140ae6c30", fn_140ae6c30 as *const ()), // sub_140ae6c30
    Replacement::draft(0x140ae6d30, "unknown_140ae6d30", fn_140ae6d30 as *const ()), // sub_140ae6d30
    Replacement::draft_throws(0x140ae6db0, "unknown_140ae6db0", fn_140ae6db0 as *const (), Throws::Discard), // sub_140ae6db0
    Replacement::draft(0x140ae6e70, "unknown_140ae6e70", fn_140ae6e70 as *const ()), // sub_140ae6e70
    Replacement::draft(0x140ae6f30, "unknown_140ae6f30", tpl_13499::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x140a330b0> as *const ()), // sub_140ae6f30
    Replacement::draft(0x140ae6fa0, "unknown_140ae6fa0", fn_140ae6fa0 as *const ()), // sub_140ae6fa0
    Replacement::draft(0x140ae7040, "unknown_140ae7040", fn_140ae7040 as *const ()), // sub_140ae7040
    Replacement::draft(0x140ae70d0, "unknown_140ae70d0", fn_140ae70d0 as *const ()), // sub_140ae70d0
    Replacement::draft_throws(0x140ae71f0, "unknown_140ae71f0", fn_140ae71f0 as *const (), Throws::Discard), // sub_140ae71f0
    Replacement::draft(0x140ae7280, "unknown_140ae7280", fn_140ae7280 as *const ()), // sub_140ae7280
    Replacement::draft(0x140ae7350, "unknown_140ae7350", fn_140ae7350 as *const ()), // sub_140ae7350
    Replacement::draft_throws(0x140ae7440, "unknown_140ae7440", tpl_8703::<0x14387e864, 0x14387e7f8, 0x1401e9fa0, 0x328, 0x338, 0x48, 0x140afe150, 0x140afe5b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140ae7440
    Replacement::draft(0x140ae74e0, "unknown_140ae74e0", fn_140ae74e0 as *const ()), // sub_140ae74e0
    Replacement::draft_throws(0x140ae75a0, "unknown_140ae75a0", tpl_8704::<0x14387e864, 0x14387e7f8, 0x1401e9b60, 0x328, 0x338, 0x120, 0x140afe150, 0x140afe880, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_140ae75a0
    Replacement::draft(0x140ae7650, "unknown_140ae7650", fn_140ae7650 as *const ()), // sub_140ae7650
    Replacement::draft_throws(0x140ae76c0, "unknown_140ae76c0", fn_140ae76c0 as *const (), Throws::Caught), // sub_140ae76c0
    Replacement::draft_throws(0x140ae7770, "unknown_140ae7770", fn_140ae7770 as *const (), Throws::Caught), // sub_140ae7770
    Replacement::draft_throws(0x140ae7820, "unknown_140ae7820", fn_140ae7820 as *const (), Throws::Caught), // sub_140ae7820
    Replacement::draft(0x140ae78d0, "unknown_140ae78d0", fn_140ae78d0 as *const ()), // sub_140ae78d0
    Replacement::draft(0x140ae7980, "unknown_140ae7980", fn_140ae7980 as *const ()), // sub_140ae7980
    Replacement::draft(0x140ae7a30, "unknown_140ae7a30", fn_140ae7a30 as *const ()), // sub_140ae7a30
    Replacement::draft_throws(0x140ae7af0, "unknown_140ae7af0", fn_140ae7af0 as *const (), Throws::Discard), // sub_140ae7af0
    Replacement::draft_throws(0x140ae7b90, "unknown_140ae7b90", tpl_2091::<0x14387e7f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432b13b8, 0x140afc940> as *const (), Throws::Discard), // sub_140ae7b90
    Replacement::draft(0x140ae7c30, "unknown_140ae7c30", fn_140ae7c30 as *const ()), // sub_140ae7c30
    Replacement::draft(0x140ae7dd0, "unknown_140ae7dd0", fn_140ae7dd0 as *const ()), // sub_140ae7dd0
    Replacement::draft_throws(0x140ae7ed0, "unknown_140ae7ed0", fn_140ae7ed0 as *const (), Throws::Discard), // sub_140ae7ed0
    Replacement::draft(0x140ae7fb0, "unknown_140ae7fb0", fn_140ae7fb0 as *const ()), // sub_140ae7fb0
    Replacement::draft_throws(0x140ae8040, "unknown_140ae8040", fn_140ae8040 as *const (), Throws::Discard), // sub_140ae8040
    Replacement::draft_throws(0x140ae8130, "unknown_140ae8130", fn_140ae8130 as *const (), Throws::Caught), // sub_140ae8130
    Replacement::draft_throws(0x140ae8260, "unknown_140ae8260", fn_140ae8260 as *const (), Throws::Caught), // sub_140ae8260
    Replacement::draft_throws(0x140ae8390, "unknown_140ae8390", fn_140ae8390 as *const (), Throws::Caught), // sub_140ae8390
    Replacement::draft(0x140ae84c0, "unknown_140ae84c0", fn_140ae84c0 as *const ()), // sub_140ae84c0
    Replacement::draft(0x140ae8550, "unknown_140ae8550", tpl_71920::<0x14387e870, 0x14387e7d8, 0x1401e9970, 0x338, 0x88, 0x8, 0x10, 0x1401e9860> as *const ()), // sub_140ae8550
    Replacement::draft(0x140ae85e0, "unknown_140ae85e0", fn_140ae85e0 as *const ()), // sub_140ae85e0
    Replacement::draft_throws(0x140ae8680, "unknown_140ae8680", fn_140ae8680 as *const (), Throws::Discard), // sub_140ae8680
    Replacement::draft_throws(0x140ae8730, "unknown_140ae8730", fn_140ae8730 as *const (), Throws::Discard), // sub_140ae8730
    Replacement::draft(0x140ae8850, "unknown_140ae8850", fn_140ae8850 as *const ()), // sub_140ae8850
    Replacement::draft(0x140ae8920, "unknown_140ae8920", fn_140ae8920 as *const ()), // sub_140ae8920
    Replacement::draft(0x140ae89a0, "unknown_140ae89a0", fn_140ae89a0 as *const ()), // sub_140ae89a0
    Replacement::draft(0x140ae8a30, "unknown_140ae8a30", fn_140ae8a30 as *const ()), // sub_140ae8a30
    Replacement::draft_throws(0x140ae8cf0, "unknown_140ae8cf0", fn_140ae8cf0 as *const (), Throws::Discard), // sub_140ae8cf0
    Replacement::draft(0x140ae8ff0, "unknown_140ae8ff0", fn_140ae8ff0 as *const ()), // sub_140ae8ff0
    Replacement::draft_throws(0x140ae91d0, "unknown_140ae91d0", fn_140ae91d0 as *const (), Throws::Discard), // sub_140ae91d0
    Replacement::draft(0x140ae9500, "unknown_140ae9500", fn_140ae9500 as *const ()), // sub_140ae9500
    Replacement::draft_throws(0x140ae9600, "unknown_140ae9600", fn_140ae9600 as *const (), Throws::Caught), // sub_140ae9600
    Replacement::draft_throws(0x140ae9d00, "unknown_140ae9d00", fn_140ae9d00 as *const (), Throws::Caught), // sub_140ae9d00
    Replacement::draft_throws(0x140aea360, "unknown_140aea360", fn_140aea360 as *const (), Throws::Caught), // sub_140aea360
    Replacement::draft(0x140aea9a0, "unknown_140aea9a0", fn_140aea9a0 as *const ()), // sub_140aea9a0
    Replacement::draft(0x140aeab70, "unknown_140aeab70", fn_140aeab70 as *const ()), // sub_140aeab70
    Replacement::draft(0x140aead60, "unknown_140aead60", fn_140aead60 as *const ()), // sub_140aead60
    Replacement::draft_throws(0x140aeaf60, "unknown_140aeaf60", fn_140aeaf60 as *const (), Throws::Discard), // sub_140aeaf60
    Replacement::draft_throws(0x140aeb120, "unknown_140aeb120", fn_140aeb120 as *const (), Throws::Caught), // sub_140aeb120
    Replacement::draft(0x140aeb4c0, "unknown_140aeb4c0", tpl_9044::<0x90, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc> as *const ()), // sub_140aeb4c0
    Replacement::draft_throws(0x140aeb560, "unknown_140aeb560", fn_140aeb560 as *const (), Throws::Discard), // sub_140aeb560
    Replacement::draft(0x140aeb700, "unknown_140aeb700", fn_140aeb700 as *const ()), // sub_140aeb700
    Replacement::draft(0x140aeb7a0, "unknown_140aeb7a0", fn_140aeb7a0 as *const ()), // sub_140aeb7a0
    Replacement::draft_throws(0x140aeb8c0, "unknown_140aeb8c0", fn_140aeb8c0 as *const (), Throws::Discard), // sub_140aeb8c0
    Replacement::draft(0x140aeb960, "unknown_140aeb960", fn_140aeb960 as *const ()), // sub_140aeb960
    Replacement::draft_throws(0x140aeba40, "unknown_140aeba40", fn_140aeba40 as *const (), Throws::Discard), // sub_140aeba40
    Replacement::draft(0x140aebaf0, "unknown_140aebaf0", fn_140aebaf0 as *const ()), // sub_140aebaf0
    Replacement::draft_throws(0x140aebb80, "unknown_140aebb80", fn_140aebb80 as *const (), Throws::Caught), // sub_140aebb80
    Replacement::draft_throws(0x140aebc40, "unknown_140aebc40", fn_140aebc40 as *const (), Throws::Caught), // sub_140aebc40
    Replacement::draft_throws(0x140aebd00, "unknown_140aebd00", fn_140aebd00 as *const (), Throws::Caught), // sub_140aebd00
    Replacement::draft(0x140aebdc0, "unknown_140aebdc0", fn_140aebdc0 as *const ()), // sub_140aebdc0
    Replacement::draft(0x140aebea0, "unknown_140aebea0", fn_140aebea0 as *const ()), // sub_140aebea0
    Replacement::draft(0x140aebf80, "unknown_140aebf80", fn_140aebf80 as *const ()), // sub_140aebf80
    Replacement::draft_throws(0x140aec060, "unknown_140aec060", fn_140aec060 as *const (), Throws::Discard), // sub_140aec060
    Replacement::draft_throws(0x140aec140, "unknown_140aec140", fn_140aec140 as *const (), Throws::Discard), // sub_140aec140
    Replacement::draft(0x140aec200, "unknown_140aec200", tpl_71952::<0x10, 0x18, 0x1426cdcb0, 0x140b00230> as *const ()), // sub_140aec200
    Replacement::draft(0x140aec280, "unknown_140aec280", tpl_71952::<0x10, 0x18, 0x1432b11b0, 0x140b00240> as *const ()), // sub_140aec280
    Replacement::draft(0x140aec300, "unknown_140aec300", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432aea58, 0x8> as *const ()), // sub_140aec300
    Replacement::draft(0x140aec380, "unknown_140aec380", tpl_8993::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432aea58, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const ()), // sub_140aec380
    Replacement::draft_throws(0x140aec420, "unknown_140aec420", fn_140aec420 as *const (), Throws::Caught), // sub_140aec420
    Replacement::draft_throws(0x140aec4b0, "unknown_140aec4b0", tpl_1908::<0x14387e810, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140aec4b0
    Replacement::draft(0x140aec530, "unknown_140aec530", fn_140aec530 as *const ()), // sub_140aec530
    Replacement::draft(0x140aec610, "unknown_140aec610", fn_140aec610 as *const ()), // sub_140aec610
    Replacement::draft_throws(0x140aec710, "unknown_140aec710", tpl_13544::<0x14387e888, 0x14387e818, 0x1401e9fa0, 0x9, 0x1406d7da0, 0x140a00f30, 0x4, 0x10, 0x2, 0x140afce20, 0x20, 0x140afd020, 0x1401e9e70> as *const (), Throws::Discard), // sub_140aec710
    Replacement::draft_throws(0x140aec810, "unknown_140aec810", fn_140aec810 as *const (), Throws::Discard), // sub_140aec810
    Replacement::draft_throws(0x140aec9d0, "unknown_140aec9d0", tpl_13544::<0x14387e888, 0x14387e818, 0x1401e9b60, 0x9, 0x1406d7da0, 0x140a01080, 0x4, 0x10, 0x2, 0x140afce20, 0x20, 0x140afd110, 0x1401e9a20> as *const (), Throws::Discard), // sub_140aec9d0
    Replacement::draft(0x140aecad0, "unknown_140aecad0", fn_140aecad0 as *const ()), // sub_140aecad0
    Replacement::draft_throws(0x140aecba0, "unknown_140aecba0", fn_140aecba0 as *const (), Throws::Caught), // sub_140aecba0
    Replacement::draft_throws(0x140aecc80, "unknown_140aecc80", fn_140aecc80 as *const (), Throws::Caught), // sub_140aecc80
    Replacement::draft_throws(0x140aecd60, "unknown_140aecd60", fn_140aecd60 as *const (), Throws::Caught), // sub_140aecd60
    Replacement::draft(0x140aece40, "unknown_140aece40", fn_140aece40 as *const ()), // sub_140aece40
    Replacement::draft(0x140aecf50, "unknown_140aecf50", fn_140aecf50 as *const ()), // sub_140aecf50
    Replacement::draft(0x140aed0e0, "unknown_140aed0e0", fn_140aed0e0 as *const ()), // sub_140aed0e0
    Replacement::draft_throws(0x140aed280, "unknown_140aed280", fn_140aed280 as *const (), Throws::Discard), // sub_140aed280
    Replacement::draft_throws(0x140aed410, "unknown_140aed410", fn_140aed410 as *const (), Throws::Discard), // sub_140aed410
    Replacement::draft(0x140aed4e0, "unknown_140aed4e0", fn_140aed4e0 as *const ()), // sub_140aed4e0
    Replacement::draft(0x140aed5a0, "unknown_140aed5a0", fn_140aed5a0 as *const ()), // sub_140aed5a0
    Replacement::draft(0x140aed670, "unknown_140aed670", fn_140aed670 as *const ()), // sub_140aed670
    Replacement::draft(0x140aed6f0, "unknown_140aed6f0", fn_140aed6f0 as *const ()), // sub_140aed6f0
    Replacement::draft(0x140aed810, "unknown_140aed810", fn_140aed810 as *const ()), // sub_140aed810
    Replacement::draft_throws(0x140aed990, "unknown_140aed990", fn_140aed990 as *const (), Throws::Caught), // sub_140aed990
    Replacement::draft_throws(0x140aedc20, "unknown_140aedc20", fn_140aedc20 as *const (), Throws::Discard), // sub_140aedc20
    Replacement::draft_throws(0x140aedd50, "unknown_140aedd50", fn_140aedd50 as *const (), Throws::Caught), // sub_140aedd50
    Replacement::draft_throws(0x140aee010, "unknown_140aee010", fn_140aee010 as *const (), Throws::Discard), // sub_140aee010
    Replacement::draft_throws(0x140aee190, "unknown_140aee190", fn_140aee190 as *const (), Throws::Caught), // sub_140aee190
    Replacement::draft_throws(0x140aee440, "unknown_140aee440", fn_140aee440 as *const (), Throws::Caught), // sub_140aee440
    Replacement::draft_throws(0x140aee6c0, "unknown_140aee6c0", fn_140aee6c0 as *const (), Throws::Caught), // sub_140aee6c0
    Replacement::draft(0x140aee930, "unknown_140aee930", fn_140aee930 as *const ()), // sub_140aee930
    Replacement::draft(0x140aeeab0, "unknown_140aeeab0", fn_140aeeab0 as *const ()), // sub_140aeeab0
    Replacement::draft(0x140aeec60, "unknown_140aeec60", fn_140aeec60 as *const ()), // sub_140aeec60
    Replacement::draft_throws(0x140aeee30, "unknown_140aeee30", fn_140aeee30 as *const (), Throws::Discard), // sub_140aeee30
    Replacement::draft_throws(0x140aeefd0, "unknown_140aeefd0", fn_140aeefd0 as *const (), Throws::Caught), // sub_140aeefd0
    Replacement::draft(0x140aef1d0, "unknown_140aef1d0", fn_140aef1d0 as *const ()), // sub_140aef1d0
    Replacement::draft(0x140aef270, "unknown_140aef270", fn_140aef270 as *const ()), // sub_140aef270
    Replacement::draft(0x140aef300, "unknown_140aef300", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432766b0> as *const ()), // sub_140aef300
    Replacement::draft(0x140aef360, "unknown_140aef360", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432766b0> as *const ()), // sub_140aef360
    Replacement::draft(0x140aef3f0, "unknown_140aef3f0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14324ab18> as *const ()), // sub_140aef3f0
    Replacement::draft(0x140aef450, "unknown_140aef450", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14324ab18> as *const ()), // sub_140aef450
    Replacement::draft(0x140aef4e0, "unknown_140aef4e0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143276120> as *const ()), // sub_140aef4e0
    Replacement::draft(0x140aef540, "unknown_140aef540", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143276120> as *const ()), // sub_140aef540
    Replacement::draft(0x140aef5d0, "unknown_140aef5d0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432b01c8> as *const ()), // sub_140aef5d0
    Replacement::draft(0x140aef630, "unknown_140aef630", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432b01c8> as *const ()), // sub_140aef630
    Replacement::draft_throws(0x140aef6c0, "unknown_140aef6c0", fn_140aef6c0 as *const (), Throws::Caught), // sub_140aef6c0
    Replacement::draft_throws(0x140aef750, "unknown_140aef750", tpl_1908::<0x14387e778, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140aef750
    Replacement::draft(0x140aef7d0, "unknown_140aef7d0", fn_140aef7d0 as *const ()), // sub_140aef7d0
    Replacement::draft(0x140aef850, "unknown_140aef850", fn_140aef850 as *const ()), // sub_140aef850
    Replacement::draft(0x140aef970, "unknown_140aef970", tpl_13566::<0x14387e7b8, 0x14243b0b0, 0x140839b60, 0x8, 0x10, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140aef970
    Replacement::draft_throws(0x140aefa00, "unknown_140aefa00", tpl_13567::<0x14387e7b0, 0x14387e7b8, 0x1401e9fa0, 0x14082b050, 0x8, 0x10, 0x18, 0x1401e9e70> as *const (), Throws::Discard), // sub_140aefa00
    Replacement::draft(0x140aefaa0, "unknown_140aefaa0", tpl_13566::<0x14387e7b8, 0x14243b0b0, 0x14083a060, 0x8, 0x10, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140aefaa0
    Replacement::draft_throws(0x140aefb30, "unknown_140aefb30", tpl_13567::<0x14387e7b0, 0x14387e7b8, 0x1401e9b60, 0x14082b180, 0x8, 0x10, 0x18, 0x1401e9a20> as *const (), Throws::Discard), // sub_140aefb30
    Replacement::draft_throws(0x140aefbd0, "unknown_140aefbd0", fn_140aefbd0 as *const (), Throws::Caught), // sub_140aefbd0
    Replacement::draft_throws(0x140aefd50, "unknown_140aefd50", fn_140aefd50 as *const (), Throws::Caught), // sub_140aefd50
    Replacement::draft_throws(0x140aefed0, "unknown_140aefed0", fn_140aefed0 as *const (), Throws::Caught), // sub_140aefed0
    Replacement::draft(0x140af0050, "unknown_140af0050", fn_140af0050 as *const ()), // sub_140af0050
    Replacement::draft(0x140af0110, "unknown_140af0110", fn_140af0110 as *const ()), // sub_140af0110
    Replacement::draft(0x140af0190, "unknown_140af0190", tpl_13566::<0x14387e7b8, 0x14243b0b0, 0x14083a4c0, 0x8, 0x10, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140af0190
    Replacement::draft_throws(0x140af0220, "unknown_140af0220", fn_140af0220 as *const (), Throws::Discard), // sub_140af0220
    Replacement::draft_throws(0x140af02a0, "unknown_140af02a0", fn_140af02a0 as *const (), Throws::Caught), // sub_140af02a0
    Replacement::draft(0x140af0590, "unknown_140af0590", fn_140af0590 as *const ()), // sub_140af0590
    Replacement::draft(0x140af0790, "unknown_140af0790", tpl_13574::<0x1432b1238, 0x1, 0x14082b2b0, 0x8, 0x1432b1228, 0x10, 0x1432b12b8, 0x18, 0x1432b12a0> as *const ()), // sub_140af0790
    Replacement::draft(0x140af0840, "unknown_140af0840", fn_140af0840 as *const ()), // sub_140af0840
    Replacement::draft(0x140af08f0, "unknown_140af08f0", fn_140af08f0 as *const ()), // sub_140af08f0
    Replacement::draft(0x140af09b0, "unknown_140af09b0", fn_140af09b0 as *const ()), // sub_140af09b0
    Replacement::draft(0x140af0a80, "unknown_140af0a80", fn_140af0a80 as *const ()), // sub_140af0a80
    Replacement::draft_throws(0x140af0c60, "unknown_140af0c60", fn_140af0c60 as *const (), Throws::Discard), // sub_140af0c60
    Replacement::draft(0x140af1060, "unknown_140af1060", fn_140af1060 as *const ()), // sub_140af1060
    Replacement::draft_throws(0x140af13a0, "unknown_140af13a0", fn_140af13a0 as *const (), Throws::Discard), // sub_140af13a0
    Replacement::draft(0x140af1740, "unknown_140af1740", fn_140af1740 as *const ()), // sub_140af1740
    Replacement::draft_throws(0x140af1840, "unknown_140af1840", fn_140af1840 as *const (), Throws::Caught), // sub_140af1840
    Replacement::draft_throws(0x140af1fb0, "unknown_140af1fb0", fn_140af1fb0 as *const (), Throws::Caught), // sub_140af1fb0
    Replacement::draft_throws(0x140af26c0, "unknown_140af26c0", fn_140af26c0 as *const (), Throws::Caught), // sub_140af26c0
    Replacement::draft(0x140af2db0, "unknown_140af2db0", fn_140af2db0 as *const ()), // sub_140af2db0
    Replacement::draft(0x140af2f10, "unknown_140af2f10", fn_140af2f10 as *const ()), // sub_140af2f10
    Replacement::draft(0x140af3090, "unknown_140af3090", fn_140af3090 as *const ()), // sub_140af3090
    Replacement::draft_throws(0x140af3220, "unknown_140af3220", fn_140af3220 as *const (), Throws::Discard), // sub_140af3220
    Replacement::draft_throws(0x140af3380, "unknown_140af3380", fn_140af3380 as *const (), Throws::Caught), // sub_140af3380
    Replacement::draft(0x140af3650, "unknown_140af3650", fn_140af3650 as *const ()), // sub_140af3650
    Replacement::draft_throws(0x140af37e0, "unknown_140af37e0", fn_140af37e0 as *const (), Throws::Discard), // sub_140af37e0
    Replacement::draft(0x140af38a0, "unknown_140af38a0", fn_140af38a0 as *const ()), // sub_140af38a0
    Replacement::draft(0x140af3940, "unknown_140af3940", fn_140af3940 as *const ()), // sub_140af3940
    Replacement::draft(0x140af39f0, "unknown_140af39f0", tpl_13499::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x140a331c0> as *const ()), // sub_140af39f0
    Replacement::draft(0x140af3a60, "unknown_140af3a60", fn_140af3a60 as *const ()), // sub_140af3a60
    Replacement::draft(0x140af3ba0, "unknown_140af3ba0", fn_140af3ba0 as *const ()), // sub_140af3ba0
    Replacement::draft(0x140af3cb0, "unknown_140af3cb0", fn_140af3cb0 as *const ()), // sub_140af3cb0
    Replacement::draft(0x140af3dc0, "unknown_140af3dc0", fn_140af3dc0 as *const ()), // sub_140af3dc0
    Replacement::draft(0x140af3e60, "unknown_140af3e60", fn_140af3e60 as *const ()), // sub_140af3e60
    Replacement::draft(0x140af3f80, "unknown_140af3f80", fn_140af3f80 as *const ()), // sub_140af3f80
    Replacement::draft(0x140af4100, "unknown_140af4100", fn_140af4100 as *const ()), // sub_140af4100
    Replacement::draft(0x140af4360, "unknown_140af4360", fn_140af4360 as *const ()), // sub_140af4360
    Replacement::draft_throws(0x140af4730, "unknown_140af4730", fn_140af4730 as *const (), Throws::Caught), // sub_140af4730
    Replacement::draft_throws(0x140af4b80, "unknown_140af4b80", fn_140af4b80 as *const (), Throws::Caught), // sub_140af4b80
    Replacement::draft_throws(0x140af4ec0, "unknown_140af4ec0", fn_140af4ec0 as *const (), Throws::Discard), // sub_140af4ec0
    Replacement::draft(0x140af5360, "unknown_140af5360", fn_140af5360 as *const ()), // sub_140af5360
    Replacement::draft_throws(0x140af5490, "unknown_140af5490", fn_140af5490 as *const (), Throws::Caught), // sub_140af5490
    Replacement::draft_throws(0x140af58e0, "unknown_140af58e0", fn_140af58e0 as *const (), Throws::Caught), // sub_140af58e0
    Replacement::draft_throws(0x140af5cd0, "unknown_140af5cd0", fn_140af5cd0 as *const (), Throws::Caught), // sub_140af5cd0
    Replacement::draft(0x140af6200, "unknown_140af6200", fn_140af6200 as *const ()), // sub_140af6200
    Replacement::draft(0x140af6670, "unknown_140af6670", fn_140af6670 as *const ()), // sub_140af6670
    Replacement::draft(0x140af6a00, "unknown_140af6a00", fn_140af6a00 as *const ()), // sub_140af6a00
    Replacement::draft_throws(0x140af6d30, "unknown_140af6d30", fn_140af6d30 as *const (), Throws::Discard), // sub_140af6d30
    Replacement::draft_throws(0x140af70d0, "unknown_140af70d0", fn_140af70d0 as *const (), Throws::Discard), // sub_140af70d0
    Replacement::draft(0x140af7410, "unknown_140af7410", fn_140af7410 as *const ()), // sub_140af7410
    Replacement::draft_throws(0x140af74c0, "unknown_140af74c0", fn_140af74c0 as *const (), Throws::Discard), // sub_140af74c0
    Replacement::draft(0x140af7570, "unknown_140af7570", tpl_13613::<0x14387e8d0, 0x14243b0b0, 0x1406da7e0, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140af7570
    Replacement::draft_throws(0x140af75f0, "unknown_140af75f0", tpl_13614::<0x14387e7c0, 0x14387e8d0, 0x1401e9fa0, 0x140afdb50, 0x10, 0x1401e9e70> as *const (), Throws::Discard), // sub_140af75f0
    Replacement::draft(0x140af7670, "unknown_140af7670", tpl_13613::<0x14387e8d0, 0x14243b0b0, 0x140afdca0, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140af7670
    Replacement::draft_throws(0x140af76f0, "unknown_140af76f0", tpl_13614::<0x14387e7c0, 0x14387e8d0, 0x1401e9b60, 0x140afdd40, 0x10, 0x1401e9a20> as *const (), Throws::Discard), // sub_140af76f0
    Replacement::draft(0x140af7770, "unknown_140af7770", fn_140af7770 as *const ()), // sub_140af7770
    Replacement::draft_throws(0x140af7840, "unknown_140af7840", fn_140af7840 as *const (), Throws::Caught), // sub_140af7840
    Replacement::draft_throws(0x140af7900, "unknown_140af7900", fn_140af7900 as *const (), Throws::Caught), // sub_140af7900
    Replacement::draft_throws(0x140af79c0, "unknown_140af79c0", fn_140af79c0 as *const (), Throws::Caught), // sub_140af79c0
    Replacement::draft(0x140af7a80, "unknown_140af7a80", tpl_13613::<0x14387e8d0, 0x14243b0b0, 0x140afdee0, 0x10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140af7a80
    Replacement::draft(0x140af7b00, "unknown_140af7b00", fn_140af7b00 as *const ()), // sub_140af7b00
    Replacement::draft(0x140af7c10, "unknown_140af7c10", fn_140af7c10 as *const ()), // sub_140af7c10
    Replacement::draft_throws(0x140af7d30, "unknown_140af7d30", fn_140af7d30 as *const (), Throws::Discard), // sub_140af7d30
    Replacement::draft_throws(0x140af7e30, "unknown_140af7e30", fn_140af7e30 as *const (), Throws::Discard), // sub_140af7e30
    Replacement::draft(0x140af7ef0, "unknown_140af7ef0", fn_140af7ef0 as *const ()), // sub_140af7ef0
    Replacement::draft(0x140af7fb0, "unknown_140af7fb0", fn_140af7fb0 as *const ()), // sub_140af7fb0
    Replacement::draft_throws(0x140af8050, "unknown_140af8050", fn_140af8050 as *const (), Throws::Discard), // sub_140af8050
    Replacement::draft(0x140af8140, "unknown_140af8140", tpl_13451::<0x14387e748, 0x14243b0b0, 0x338, 0x1401e8620, 0x18, 0x80, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140af8140
    Replacement::draft_throws(0x140af81e0, "unknown_140af81e0", fn_140af81e0 as *const (), Throws::Discard), // sub_140af81e0
    Replacement::draft_throws(0x140af8280, "unknown_140af8280", fn_140af8280 as *const (), Throws::Caught), // sub_140af8280
    Replacement::draft_throws(0x140af8360, "unknown_140af8360", fn_140af8360 as *const (), Throws::Caught), // sub_140af8360
    Replacement::draft_throws(0x140af8440, "unknown_140af8440", fn_140af8440 as *const (), Throws::Caught), // sub_140af8440
    Replacement::draft(0x140af8520, "unknown_140af8520", fn_140af8520 as *const ()), // sub_140af8520
    Replacement::draft(0x140af85a0, "unknown_140af85a0", tpl_2262::<0x14387e8bc, 0x14387e748, 0x1401e9970, 0x338, 0x88, 0x18, 0x120, 0x1401e9860> as *const ()), // sub_140af85a0
    Replacement::draft(0x140af8620, "unknown_140af8620", tpl_2258::<0x14387e748, 0x14243b0b0, 0x338, 0x88, 0x18, 0x120, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140af8620
    Replacement::draft_throws(0x140af86b0, "unknown_140af86b0", fn_140af86b0 as *const (), Throws::Discard), // sub_140af86b0
    Replacement::draft_throws(0x140af8740, "unknown_140af8740", tpl_13627::<0x14387e748, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x1432b0fb8, 0x1406db0a0, 0x18, 0x1432b0fa8, 0x14020f0f0> as *const (), Throws::Discard), // sub_140af8740
    Replacement::draft(0x140af8830, "unknown_140af8830", fn_140af8830 as *const ()), // sub_140af8830
    Replacement::draft(0x140af88e0, "unknown_140af88e0", fn_140af88e0 as *const ()), // sub_140af88e0
    Replacement::draft(0x140af8960, "unknown_140af8960", fn_140af8960 as *const ()), // sub_140af8960
    Replacement::draft(0x140af8a30, "unknown_140af8a30", fn_140af8a30 as *const ()), // sub_140af8a30
    Replacement::draft_throws(0x140af8b80, "unknown_140af8b80", fn_140af8b80 as *const (), Throws::Discard), // sub_140af8b80
    Replacement::draft_throws(0x140af8ca0, "unknown_140af8ca0", fn_140af8ca0 as *const (), Throws::Discard), // sub_140af8ca0
    Replacement::draft_throws(0x140af8d80, "unknown_140af8d80", fn_140af8d80 as *const (), Throws::Discard), // sub_140af8d80
    Replacement::draft(0x140af8eb0, "unknown_140af8eb0", fn_140af8eb0 as *const ()), // sub_140af8eb0
    Replacement::draft_throws(0x140af8f70, "unknown_140af8f70", fn_140af8f70 as *const (), Throws::Caught), // sub_140af8f70
    Replacement::draft_throws(0x140af9200, "unknown_140af9200", fn_140af9200 as *const (), Throws::Caught), // sub_140af9200
    Replacement::draft_throws(0x140af9460, "unknown_140af9460", fn_140af9460 as *const (), Throws::Caught), // sub_140af9460
    Replacement::draft(0x140af96b0, "unknown_140af96b0", fn_140af96b0 as *const ()), // sub_140af96b0
    Replacement::draft(0x140af97b0, "unknown_140af97b0", fn_140af97b0 as *const ()), // sub_140af97b0
    Replacement::draft(0x140af9880, "unknown_140af9880", fn_140af9880 as *const ()), // sub_140af9880
    Replacement::draft_throws(0x140af9960, "unknown_140af9960", fn_140af9960 as *const (), Throws::Discard), // sub_140af9960
    Replacement::draft_throws(0x140af9a30, "unknown_140af9a30", fn_140af9a30 as *const (), Throws::Discard), // sub_140af9a30
    Replacement::draft(0x140af9b20, "unknown_140af9b20", fn_140af9b20 as *const ()), // sub_140af9b20
    Replacement::draft(0x140af9ba0, "unknown_140af9ba0", fn_140af9ba0 as *const ()), // sub_140af9ba0
    Replacement::draft(0x140af9d40, "unknown_140af9d40", tpl_13639::<0x14387e878, 0x14243b0b0, 0x14139c4a0, 0x10, 0x1402c4ad0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140af9d40
    Replacement::draft_throws(0x140af9dc0, "unknown_140af9dc0", tpl_13640::<0x14387e8a0, 0x14387e878, 0x1401e9fa0, 0x328, 0x1432b1018, 0x14139c560, 0x10, 0x1432b1008, 0x142418680, 0x1401e9e70> as *const (), Throws::Discard), // sub_140af9dc0
    Replacement::draft_throws(0x140af9e60, "unknown_140af9e60", fn_140af9e60 as *const (), Throws::Discard), // sub_140af9e60
    Replacement::draft_throws(0x140af9ef0, "unknown_140af9ef0", tpl_13640::<0x14387e8a0, 0x14387e878, 0x1401e9b60, 0x328, 0x1432b1018, 0x14139c600, 0x10, 0x1432b1008, 0x142418720, 0x1401e9a20> as *const (), Throws::Discard), // sub_140af9ef0
    Replacement::draft_throws(0x140af9f90, "unknown_140af9f90", fn_140af9f90 as *const (), Throws::Caught), // sub_140af9f90
    Replacement::draft_throws(0x140afa050, "unknown_140afa050", fn_140afa050 as *const (), Throws::Caught), // sub_140afa050
    Replacement::draft_throws(0x140afa110, "unknown_140afa110", fn_140afa110 as *const (), Throws::Caught), // sub_140afa110
    Replacement::draft(0x140afa1d0, "unknown_140afa1d0", fn_140afa1d0 as *const ()), // sub_140afa1d0
    Replacement::draft(0x140afa280, "unknown_140afa280", fn_140afa280 as *const ()), // sub_140afa280
    Replacement::draft(0x140afa340, "unknown_140afa340", fn_140afa340 as *const ()), // sub_140afa340
    Replacement::draft_throws(0x140afa410, "unknown_140afa410", fn_140afa410 as *const (), Throws::Discard), // sub_140afa410
    Replacement::draft_throws(0x140afa4e0, "unknown_140afa4e0", tpl_10122::<0x14387e878, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432b1018, 0x140afdb40, 0x10, 0x1432b1008, 0x1402c4bf0> as *const (), Throws::Discard), // sub_140afa4e0
    Replacement::draft(0x140afa5a0, "unknown_140afa5a0", fn_140afa5a0 as *const ()), // sub_140afa5a0
    Replacement::draft(0x140afa610, "unknown_140afa610", tpl_13647::<0x14387e8d8, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140afa610
    Replacement::draft_throws(0x140afa6c0, "unknown_140afa6c0", tpl_13648::<0x14387e8e4, 0x14387e8d8, 0x1401e9fa0, 0x328, 0x338, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140afa6c0
    Replacement::draft(0x140afa760, "unknown_140afa760", tpl_2076::<0x14387e8d8, 0x14243b0b0, 0x338, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140afa760
    Replacement::draft_throws(0x140afa7d0, "unknown_140afa7d0", tpl_13649::<0x14387e8e4, 0x14387e8d8, 0x1401e9b60, 0x328, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140afa7d0
    Replacement::draft_throws(0x140afa870, "unknown_140afa870", fn_140afa870 as *const (), Throws::Caught), // sub_140afa870
    Replacement::draft_throws(0x140afaad0, "unknown_140afaad0", fn_140afaad0 as *const (), Throws::Caught), // sub_140afaad0
    Replacement::draft_throws(0x140afad00, "unknown_140afad00", fn_140afad00 as *const (), Throws::Caught), // sub_140afad00
    Replacement::draft(0x140afaf20, "unknown_140afaf20", tpl_13653::<0x14387e8d8, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140afaf20
    Replacement::draft(0x140afafb0, "unknown_140afafb0", tpl_2080::<0x14387e8e4, 0x14387e8d8, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // sub_140afafb0
    Replacement::draft(0x140afb020, "unknown_140afb020", tpl_2076::<0x14387e8d8, 0x14243b0b0, 0x338, 0xc0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140afb020
    Replacement::draft_throws(0x140afb090, "unknown_140afb090", tpl_13654::<0x14387e8d8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x14307eae0, 0x1402c4730> as *const (), Throws::Discard), // sub_140afb090
    Replacement::draft(0x140afb150, "unknown_140afb150", tpl_9963::<0x14387e828, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x38, 0x33, 0x34, 0x30, 0xfffffffffffffff8> as *const ()), // sub_140afb150
    Replacement::draft_throws(0x140afb240, "unknown_140afb240", fn_140afb240 as *const (), Throws::Discard), // sub_140afb240
    Replacement::draft(0x140afb340, "unknown_140afb340", tpl_9959::<0x14387e828, 0x14243b0b0, 0x338, 0x20, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140afb340
    Replacement::draft_throws(0x140afb3c0, "unknown_140afb3c0", fn_140afb3c0 as *const (), Throws::Discard), // sub_140afb3c0
    Replacement::draft_throws(0x140afb4c0, "unknown_140afb4c0", fn_140afb4c0 as *const (), Throws::Caught), // sub_140afb4c0
    Replacement::draft_throws(0x140afb8a0, "unknown_140afb8a0", fn_140afb8a0 as *const (), Throws::Caught), // sub_140afb8a0
    Replacement::draft_throws(0x140afbc20, "unknown_140afbc20", fn_140afbc20 as *const (), Throws::Caught), // sub_140afbc20
    Replacement::draft(0x140afbf80, "unknown_140afbf80", fn_140afbf80 as *const ()), // sub_140afbf80
    Replacement::draft(0x140afc030, "unknown_140afc030", tpl_9961::<0x14387e830, 0x14387e828, 0x1401e9970, 0x338, 0xc0, 0x30, 0x1401e9860> as *const ()), // sub_140afc030
    Replacement::draft(0x140afc0b0, "unknown_140afc0b0", tpl_9959::<0x14387e828, 0x14243b0b0, 0x338, 0xc0, 0x30, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140afc0b0
    Replacement::draft_throws(0x140afc140, "unknown_140afc140", tpl_9965::<0x14387e830, 0x14387e828, 0x1401e9770, 0x338, 0x1424b4cb0, 0x30, 0x1401e9590> as *const (), Throws::Discard), // sub_140afc140
    Replacement::draft_throws(0x140afc1b0, "unknown_140afc1b0", fn_140afc1b0 as *const (), Throws::Discard), // sub_140afc1b0
    Replacement::draft(0x140afc2b0, "physics_140afc2b0", tpl_13661::<0x1401de830, 0x10, 0x1430b6bd0, 0x141834d30> as *const ()), // physics::PhysXClothMetadata::physics::PhysXClothMetadata::vfunc_0
    Replacement::draft(0x140afc300, "content_140afc300", tpl_570::<0x14022a610, 0x1430b6bd0, 0x141834d30> as *const ()), // content::AnimationSourceDebugMetadata::content::AnimationSourceDebugMetadata::vfunc_0
    Replacement::draft(0x140afc350, "content_140afc350", fn_140afc350 as *const ()), // sub_140afc350
    Replacement::draft(0x140afc400, "content_140afc400", fn_140afc400 as *const ()), // sub_140afc400
    Replacement::draft_throws(0x140afc4c0, "content_140afc4c0", fn_140afc4c0 as *const (), Throws::Caught), // sub_140afc4c0
    Replacement::draft_throws(0x140afc5c0, "content_140afc5c0", fn_140afc5c0 as *const (), Throws::Caught), // sub_140afc5c0
    Replacement::draft_throws(0x140afc6c0, "content_140afc6c0", fn_140afc6c0 as *const (), Throws::Caught), // sub_140afc6c0
    Replacement::draft(0x140afc8a0, "content_140afc8a0", fn_140afc8a0 as *const ()), // sub_140afc8a0
    Replacement::draft_throws(0x140afc940, "content_140afc940", tpl_2507::<0x20, 0x8, 0x140aff830> as *const (), Throws::Discard), // sub_140afc940
    Replacement::draft(0x140afc990, "content_140afc990", fn_140afc990 as *const ()), // sub_140afc990
    Replacement::draft_throws(0x140afca30, "content_140afca30", fn_140afca30 as *const (), Throws::Caught), // sub_140afca30
    Replacement::draft_throws(0x140afcbe0, "content_140afcbe0", fn_140afcbe0 as *const (), Throws::Caught), // sub_140afcbe0
    Replacement::draft_throws(0x140afcdd0, "content_140afcdd0", fn_140afcdd0 as *const (), Throws::Discard), // sub_140afcdd0
    Replacement::draft(0x140afce20, "content_140afce20", fn_140afce20 as *const ()), // sub_140afce20
    Replacement::draft(0x140afcee0, "content_140afcee0", fn_140afcee0 as *const ()), // sub_140afcee0
    Replacement::draft(0x140afcf70, "content_140afcf70", fn_140afcf70 as *const ()), // sub_140afcf70
    Replacement::draft_throws(0x140afd020, "content_140afd020", fn_140afd020 as *const (), Throws::Discard), // sub_140afd020
    Replacement::draft_throws(0x140afd090, "content_140afd090", fn_140afd090 as *const (), Throws::Discard), // sub_140afd090
    Replacement::draft_throws(0x140afd110, "content_140afd110", fn_140afd110 as *const (), Throws::Discard), // sub_140afd110
    Replacement::draft(0x140afd1a0, "content_140afd1a0", fn_140afd1a0 as *const ()), // sub_140afd1a0
    Replacement::draft(0x140afd250, "content_140afd250", fn_140afd250 as *const ()), // sub_140afd250
    Replacement::draft(0x140afd2c0, "content_140afd2c0", tpl_72090::<0x8, 0x338, 0xe8, 0x140afa280> as *const ()), // sub_140afd2c0
    Replacement::draft(0x140afd340, "content_140afd340", tpl_72090::<0x8, 0x338, 0xe8, 0x140afa340> as *const ()), // sub_140afd340
    Replacement::draft_throws(0x140afd3c0, "content_140afd3c0", fn_140afd3c0 as *const (), Throws::Discard), // sub_140afd3c0
    Replacement::draft_throws(0x140afd430, "content_140afd430", tpl_6985::<0x20, 0x8, 0x140b01570> as *const (), Throws::Discard), // sub_140afd430
    Replacement::draft(0x140afd480, "content_140afd480", fn_140afd480 as *const ()), // sub_140afd480
    Replacement::draft_throws(0x140afd530, "content_140afd530", fn_140afd530 as *const (), Throws::Caught), // sub_140afd530
    Replacement::draft_throws(0x140afd6f0, "content_140afd6f0", fn_140afd6f0 as *const (), Throws::Caught), // sub_140afd6f0
    Replacement::draft_throws(0x140afd8a0, "content_140afd8a0", fn_140afd8a0 as *const (), Throws::Caught), // sub_140afd8a0
    Replacement::draft_throws(0x140afda90, "content_140afda90", tpl_12660::<0x20, 0x8, 0x140b04420> as *const (), Throws::Discard), // sub_140afda90
    Replacement::draft(0x140afdb50, "content_140afdb50", tpl_12669::<0x140afe4e0, 0x1404b9a10, 0x142417470> as *const ()), // sub_140afdb50
    Replacement::draft(0x140afdca0, "content_140afdca0", tpl_66008::<0x338, 0x48, 0x140afe4e0, 0x1404b9a10> as *const ()), // sub_140afdca0
    Replacement::draft(0x140afdd40, "content_140afdd40", tpl_12670::<0x140afe4e0, 0x1404b9a10, 0x142417470> as *const ()), // sub_140afdd40
    Replacement::draft(0x140afdee0, "content_140afdee0", fn_140afdee0 as *const ()), // sub_140afdee0
    Replacement::draft(0x140afdff0, "content_140afdff0", fn_140afdff0 as *const ()), // sub_140afdff0
    Replacement::draft_throws(0x140afe090, "content_140afe090", tpl_6980::<0x20, 0x8, 0x140b06370> as *const (), Throws::Discard), // sub_140afe090
    Replacement::draft(0x140afe0e0, "content_140afe0e0", fn_140afe0e0 as *const ()), // sub_140afe0e0
    Replacement::draft(0x140afe150, "content_140afe150", fn_140afe150 as *const ()), // sub_140afe150
    Replacement::draft(0x140afe210, "content_140afe210", tpl_72099::<0x8, 0xc, 0x140701e10, 0x14183acc2> as *const ()), // sub_140afe210
    Replacement::draft(0x140afe2d0, "content_140afe2d0", fn_140afe2d0 as *const ()), // sub_140afe2d0
    Replacement::draft(0x140afe400, "content_140afe400", fn_140afe400 as *const ()), // sub_140afe400
    Replacement::draft(0x140afe4e0, "content_140afe4e0", tpl_66026::<0x140b0eda0> as *const ()), // sub_140afe4e0
    Replacement::draft_throws(0x140afe5b0, "content_140afe5b0", fn_140afe5b0 as *const (), Throws::Caught), // sub_140afe5b0
    Replacement::draft_throws(0x140afe880, "content_140afe880", fn_140afe880 as *const (), Throws::Caught), // sub_140afe880
    Replacement::draft_throws(0x140afeb50, "content_140afeb50", fn_140afeb50 as *const (), Throws::Caught), // sub_140afeb50
    Replacement::draft_throws(0x140afefc0, "content_140afefc0", fn_140afefc0 as *const (), Throws::Caught), // sub_140afefc0
    Replacement::draft_throws(0x140aff400, "content_140aff400", fn_140aff400 as *const (), Throws::Caught), // sub_140aff400
    Replacement::draft_throws(0x140aff830, "content_140aff830", fn_140aff830 as *const (), Throws::Caught), // sub_140aff830
    Replacement::draft_throws(0x140aff900, "content_140aff900", fn_140aff900 as *const (), Throws::Caught), // sub_140aff900
    Replacement::draft_throws(0x140affc00, "content_140affc00", fn_140affc00 as *const (), Throws::Caught), // sub_140affc00
    Replacement::draft_throws(0x140affec0, "content_140affec0", fn_140affec0 as *const (), Throws::Caught), // sub_140affec0
    Replacement::draft_throws(0x140b00160, "content_140b00160", fn_140b00160 as *const (), Throws::Caught), // sub_140b00160
    Replacement::draft(0x140b00240, "content_140b00240", fn_140b00240 as *const ()), // sub_140b00240
    Replacement::draft_throws(0x140b00270, "content_140b00270", fn_140b00270 as *const (), Throws::Caught), // sub_140b00270
    Replacement::draft_throws(0x140b00560, "content_140b00560", fn_140b00560 as *const (), Throws::Caught), // sub_140b00560
    Replacement::draft_throws(0x140b00850, "content_140b00850", fn_140b00850 as *const (), Throws::Caught), // sub_140b00850
    Replacement::draft_throws(0x140b00cd0, "content_140b00cd0", fn_140b00cd0 as *const (), Throws::Caught), // sub_140b00cd0
    Replacement::draft_throws(0x140b01120, "content_140b01120", fn_140b01120 as *const (), Throws::Caught), // sub_140b01120
    Replacement::draft_throws(0x140b01570, "content_140b01570", fn_140b01570 as *const (), Throws::Caught), // sub_140b01570
    Replacement::draft_throws(0x140b01640, "content_140b01640", fn_140b01640 as *const (), Throws::Caught), // sub_140b01640
    Replacement::draft_throws(0x140b01960, "content_140b01960", fn_140b01960 as *const (), Throws::Discard), // sub_140b01960
    Replacement::draft_throws(0x140b01bc0, "content_140b01bc0", fn_140b01bc0 as *const (), Throws::Caught), // sub_140b01bc0
    Replacement::draft_throws(0x140b01ee0, "content_140b01ee0", fn_140b01ee0 as *const (), Throws::Caught), // sub_140b01ee0
    Replacement::draft_throws(0x140b02490, "content_140b02490", fn_140b02490 as *const (), Throws::Caught), // sub_140b02490
    Replacement::draft_throws(0x140b02780, "content_140b02780", fn_140b02780 as *const (), Throws::Caught), // sub_140b02780
    Replacement::draft_throws(0x140b02a40, "content_140b02a40", fn_140b02a40 as *const (), Throws::Caught), // sub_140b02a40
    Replacement::draft_throws(0x140b02d00, "content_140b02d00", fn_140b02d00 as *const (), Throws::Caught), // sub_140b02d00
    Replacement::draft_throws(0x140b02dd0, "content_140b02dd0", fn_140b02dd0 as *const (), Throws::Caught), // sub_140b02dd0
    Replacement::draft_throws(0x140b03090, "content_140b03090", fn_140b03090 as *const (), Throws::Caught), // sub_140b03090
    Replacement::draft_throws(0x140b03350, "content_140b03350", fn_140b03350 as *const (), Throws::Caught), // sub_140b03350
    Replacement::draft_throws(0x140b03620, "content_140b03620", fn_140b03620 as *const (), Throws::Caught), // sub_140b03620
    Replacement::draft_throws(0x140b03920, "content_140b03920", fn_140b03920 as *const (), Throws::Caught), // sub_140b03920
    Replacement::draft_throws(0x140b03bf0, "content_140b03bf0", fn_140b03bf0 as *const (), Throws::Caught), // sub_140b03bf0
    Replacement::draft_throws(0x140b03eb0, "content_140b03eb0", fn_140b03eb0 as *const (), Throws::Caught), // sub_140b03eb0
    Replacement::draft_throws(0x140b04180, "content_140b04180", fn_140b04180 as *const (), Throws::Caught), // sub_140b04180
    Replacement::draft_throws(0x140b04420, "content_140b04420", fn_140b04420 as *const (), Throws::Caught), // sub_140b04420
    Replacement::draft_throws(0x140b044f0, "content_140b044f0", fn_140b044f0 as *const (), Throws::Caught), // sub_140b044f0
    Replacement::draft_throws(0x140b04860, "content_140b04860", fn_140b04860 as *const (), Throws::Caught), // sub_140b04860
    Replacement::draft_throws(0x140b04b80, "content_140b04b80", fn_140b04b80 as *const (), Throws::Caught), // sub_140b04b80
    Replacement::draft_throws(0x140b04e80, "content_140b04e80", fn_140b04e80 as *const (), Throws::Caught), // sub_140b04e80
    Replacement::draft_throws(0x140b051e0, "content_140b051e0", fn_140b051e0 as *const (), Throws::Caught), // sub_140b051e0
    Replacement::draft_throws(0x140b05540, "content_140b05540", fn_140b05540 as *const (), Throws::Caught), // sub_140b05540
    Replacement::draft_throws(0x140b05a50, "content_140b05a50", fn_140b05a50 as *const (), Throws::Caught), // sub_140b05a50
    Replacement::draft_throws(0x140b05ef0, "content_140b05ef0", fn_140b05ef0 as *const (), Throws::Caught), // sub_140b05ef0
    Replacement::draft_throws(0x140b06370, "content_140b06370", fn_140b06370 as *const (), Throws::Caught), // sub_140b06370
    Replacement::draft(0x140b06480, "content_140b06480", tpl_13706::<0x8, 0x14240ab10> as *const ()), // sub_140b06480
    Replacement::draft_throws(0x140b064c0, "content_140b064c0", fn_140b064c0 as *const (), Throws::Discard), // sub_140b064c0
    Replacement::draft(0x140b06760, "content_140b06760", fn_140b06760 as *const ()), // sub_140b06760
    Replacement::draft(0x140b06960, "content_140b06960", fn_140b06960 as *const ()), // sub_140b06960
    Replacement::draft(0x140b06e20, "content_140b06e20", tpl_7081::<0x140b0eda0, 0x1404b9990, 0x1404b9a10, 0x142417470> as *const ()), // sub_140b06e20
    Replacement::draft_throws(0x140b07000, "content_140b07000", fn_140b07000 as *const (), Throws::Caught), // sub_140b07000
    Replacement::draft_throws(0x140b074d0, "content_140b074d0", fn_140b074d0 as *const (), Throws::Caught), // sub_140b074d0
    Replacement::draft_throws(0x140b07960, "content_140b07960", fn_140b07960 as *const (), Throws::Caught), // sub_140b07960
    Replacement::draft_throws(0x140b07df0, "content_140b07df0", fn_140b07df0 as *const (), Throws::Caught), // sub_140b07df0
    Replacement::draft_throws(0x140b082c0, "content_140b082c0", fn_140b082c0 as *const (), Throws::Caught), // sub_140b082c0
    Replacement::draft_throws(0x140b08750, "content_140b08750", fn_140b08750 as *const (), Throws::Caught), // sub_140b08750
    Replacement::draft(0x140b08be0, "content_140b08be0", fn_140b08be0 as *const ()), // sub_140b08be0
    Replacement::draft_throws(0x140b08ca0, "content_140b08ca0", fn_140b08ca0 as *const (), Throws::Caught), // sub_140b08ca0
    Replacement::draft_throws(0x140b09170, "content_140b09170", fn_140b09170 as *const (), Throws::Caught), // sub_140b09170
    Replacement::draft_throws(0x140b09600, "content_140b09600", fn_140b09600 as *const (), Throws::Caught), // sub_140b09600
    Replacement::draft_throws(0x140b09a90, "content_140b09a90", fn_140b09a90 as *const (), Throws::Caught), // sub_140b09a90
    Replacement::draft_throws(0x140b09f60, "content_140b09f60", fn_140b09f60 as *const (), Throws::Caught), // sub_140b09f60
    Replacement::draft_throws(0x140b0a3e0, "content_140b0a3e0", fn_140b0a3e0 as *const (), Throws::Caught), // sub_140b0a3e0
    Replacement::draft_throws(0x140b0a860, "content_140b0a860", fn_140b0a860 as *const (), Throws::Caught), // sub_140b0a860
    Replacement::draft_throws(0x140b0aa60, "content_140b0aa60", fn_140b0aa60 as *const (), Throws::Caught), // sub_140b0aa60
    Replacement::draft_throws(0x140b0b160, "content_140b0b160", fn_140b0b160 as *const (), Throws::Caught), // sub_140b0b160
    Replacement::draft_throws(0x140b0b630, "content_140b0b630", fn_140b0b630 as *const (), Throws::Caught), // sub_140b0b630
    Replacement::draft_throws(0x140b0bd00, "content_140b0bd00", fn_140b0bd00 as *const (), Throws::Caught), // sub_140b0bd00
    Replacement::draft_throws(0x140b0c190, "content_140b0c190", fn_140b0c190 as *const (), Throws::Caught), // sub_140b0c190
    Replacement::draft_throws(0x140b0c5d0, "content_140b0c5d0", fn_140b0c5d0 as *const (), Throws::Caught), // sub_140b0c5d0
    Replacement::draft_throws(0x140b0ca60, "content_140b0ca60", fn_140b0ca60 as *const (), Throws::Caught), // sub_140b0ca60
    Replacement::draft_throws(0x140b0d120, "content_140b0d120", fn_140b0d120 as *const (), Throws::Caught), // sub_140b0d120
    Replacement::draft_throws(0x140b0d760, "content_140b0d760", fn_140b0d760 as *const (), Throws::Caught), // sub_140b0d760
    Replacement::draft_throws(0x140b0dd20, "content_140b0dd20", fn_140b0dd20 as *const (), Throws::Caught), // sub_140b0dd20
    Replacement::draft_throws(0x140b0e1f0, "content_140b0e1f0", fn_140b0e1f0 as *const (), Throws::Caught), // sub_140b0e1f0
    Replacement::draft_throws(0x140b0e680, "content_140b0e680", fn_140b0e680 as *const (), Throws::Caught), // sub_140b0e680
    Replacement::draft(0x140b0eb10, "content_140b0eb10", fn_140b0eb10 as *const ()), // sub_140b0eb10
    Replacement::draft(0x140b0eb70, "content_140b0eb70", fn_140b0eb70 as *const ()), // sub_140b0eb70
    Replacement::draft(0x140b0eda0, "content_140b0eda0", fn_140b0eda0 as *const ()), // sub_140b0eda0
    Replacement::draft_throws(0x140b0ef10, "content_140b0ef10", fn_140b0ef10 as *const (), Throws::Caught), // sub_140b0ef10
    Replacement::draft_throws(0x140b0f170, "content_140b0f170", fn_140b0f170 as *const (), Throws::Caught), // sub_140b0f170
    Replacement::draft_throws(0x140b0f3d0, "content_140b0f3d0", fn_140b0f3d0 as *const (), Throws::Caught), // sub_140b0f3d0
    Replacement::draft_throws(0x140b0f630, "content_140b0f630", fn_140b0f630 as *const (), Throws::Caught), // sub_140b0f630
    Replacement::draft_throws(0x140b0f820, "content_140b0f820", fn_140b0f820 as *const (), Throws::Caught), // sub_140b0f820
    Replacement::draft_throws(0x140b0f960, "content_140b0f960", fn_140b0f960 as *const (), Throws::Caught), // sub_140b0f960
    Replacement::draft_throws(0x140b0faa0, "content_140b0faa0", fn_140b0faa0 as *const (), Throws::Caught), // sub_140b0faa0
    Replacement::draft_throws(0x140b0fbe0, "content_140b0fbe0", fn_140b0fbe0 as *const (), Throws::Caught), // sub_140b0fbe0
    Replacement::draft_throws(0x140b0fdd0, "content_140b0fdd0", fn_140b0fdd0 as *const (), Throws::Caught), // sub_140b0fdd0
    Replacement::draft_throws(0x140b10040, "content_140b10040", fn_140b10040 as *const (), Throws::Caught), // sub_140b10040
    Replacement::draft_throws(0x140b102a0, "content_140b102a0", fn_140b102a0 as *const (), Throws::Caught), // sub_140b102a0
    Replacement::draft_throws(0x140b10500, "content_140b10500", fn_140b10500 as *const (), Throws::Caught), // sub_140b10500
    Replacement::draft_throws(0x140b10750, "content_140b10750", fn_140b10750 as *const (), Throws::Caught), // sub_140b10750
    Replacement::draft_throws(0x140b11210, "content_140b11210", fn_140b11210 as *const (), Throws::Caught), // sub_140b11210
    Replacement::draft_throws(0x140b11ca0, "content_140b11ca0", fn_140b11ca0 as *const (), Throws::Caught), // sub_140b11ca0
    Replacement::draft_throws(0x140b12720, "content_140b12720", fn_140b12720 as *const (), Throws::Caught), // sub_140b12720
    Replacement::draft_throws(0x140b12860, "content_140b12860", fn_140b12860 as *const (), Throws::Caught), // sub_140b12860
    Replacement::draft_throws(0x140b129a0, "content_140b129a0", fn_140b129a0 as *const (), Throws::Caught), // sub_140b129a0
    Replacement::draft_throws(0x140b12ae0, "content_140b12ae0", fn_140b12ae0 as *const (), Throws::Caught), // sub_140b12ae0
    Replacement::draft_throws(0x140b12d70, "content_140b12d70", fn_140b12d70 as *const (), Throws::Caught), // sub_140b12d70
    Replacement::draft_throws(0x140b13040, "content_140b13040", fn_140b13040 as *const (), Throws::Caught), // sub_140b13040
    Replacement::draft_throws(0x140b13310, "content_140b13310", fn_140b13310 as *const (), Throws::Caught), // sub_140b13310
    Replacement::draft_throws(0x140b135e0, "content_140b135e0", fn_140b135e0 as *const (), Throws::Caught), // sub_140b135e0
    Replacement::draft_throws(0x140b13870, "content_140b13870", tpl_65670::<0x142442de0, 0x143a3b408, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x108> as *const (), Throws::Discard), // sub_140b13870
    Replacement::draft(0x140b13950, "content_140b13950", fn_140b13950 as *const ()), // sub_140b13950
    Replacement::draft_throws(0x140b139d0, "content_140b139d0", fn_140b139d0 as *const (), Throws::Caught), // sub_140b139d0
    Replacement::draft_throws(0x140b13f40, "content_140b13f40", fn_140b13f40 as *const (), Throws::Caught), // sub_140b13f40
    Replacement::draft_throws(0x140b14480, "content_140b14480", fn_140b14480 as *const (), Throws::Caught), // sub_140b14480
    Replacement::draft_throws(0x140b147d0, "content_140b147d0", fn_140b147d0 as *const (), Throws::Caught), // sub_140b147d0
    Replacement::draft_throws(0x140b14c40, "content_140b14c40", fn_140b14c40 as *const (), Throws::Caught), // sub_140b14c40
    Replacement::draft_throws(0x140b15070, "content_140b15070", fn_140b15070 as *const (), Throws::Caught), // sub_140b15070
    Replacement::draft_throws(0x140b153c0, "content_140b153c0", fn_140b153c0 as *const (), Throws::Caught), // sub_140b153c0
    Replacement::draft_throws(0x140b15960, "content_140b15960", fn_140b15960 as *const (), Throws::Caught), // sub_140b15960
    Replacement::draft_throws(0x140b15ef0, "content_140b15ef0", fn_140b15ef0 as *const (), Throws::Caught), // sub_140b15ef0
    Replacement::draft_throws(0x140b16240, "content_140b16240", fn_140b16240 as *const (), Throws::Caught), // sub_140b16240
    Replacement::draft_throws(0x140b16390, "content_140b16390", fn_140b16390 as *const (), Throws::Caught), // sub_140b16390
    Replacement::draft_throws(0x140b16800, "content_140b16800", fn_140b16800 as *const (), Throws::Caught), // sub_140b16800
    Replacement::draft_throws(0x140b16c30, "content_140b16c30", fn_140b16c30 as *const (), Throws::Caught), // sub_140b16c30
    Replacement::draft_throws(0x140b16f80, "content_140b16f80", fn_140b16f80 as *const (), Throws::Caught), // sub_140b16f80
    Replacement::draft_throws(0x140b175d0, "content_140b175d0", fn_140b175d0 as *const (), Throws::Caught), // sub_140b175d0
    Replacement::draft_throws(0x140b17be0, "content_140b17be0", fn_140b17be0 as *const (), Throws::Caught), // sub_140b17be0
    Replacement::draft_throws(0x140b17f30, "content_140b17f30", fn_140b17f30 as *const (), Throws::Caught), // sub_140b17f30
    Replacement::draft(0x140b184e0, "content_140b184e0", fn_140b184e0 as *const ()), // sub_140b184e0
    Replacement::draft(0x140b18ab0, "content_140b18ab0", fn_140b18ab0 as *const ()), // content::LightSensitiveDecalComponent::content::LightSensitiveDecalComponent::vfunc_18
    Replacement::draft(0x140b18c70, "content_140b18c70", fn_140b18c70 as *const ()), // content::MapAreaComponent::content::MapAreaComponent::vfunc_10
    Replacement::draft(0x140b18f90, "content_140b18f90", fn_140b18f90 as *const ()), // content::EffectsGameSpawnPool::content::EffectsGameSpawnPool::vfunc_1
    Replacement::draft(0x140b18fe0, "content_140b18fe0", tpl_586::<0x24> as *const ()), // content::EffectsGameSpawnPool::content::EffectsGameSpawnPool::vfunc_3
    Replacement::draft(0x140b190c0, "content_140b190c0", tpl_5719::<0x1432b2798, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::NLAIVisionSensorComponent::content::NLAIVisionSensorComponent::vfunc_18
    Replacement::draft(0x140b193c0, "content_140b193c0", fn_140b193c0 as *const ()), // sub_140b193c0
    Replacement::draft(0x140b19440, "content_140b19440", tpl_11829::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14320cbb8, 0x18, 0x20> as *const ()), // sub_140b19440
    Replacement::draft(0x140b194e0, "content_140b194e0", tpl_13738::<0x14387e978, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b194e0
    Replacement::draft(0x140b19550, "content_140b19550", fn_140b19550 as *const ()), // sub_140b19550
    Replacement::draft_throws(0x140b196e0, "content_140b196e0", fn_140b196e0 as *const (), Throws::Caught), // sub_140b196e0
    Replacement::draft(0x140b198a0, "content_140b198a0", fn_140b198a0 as *const ()), // sub_140b198a0
    Replacement::draft_throws(0x140b199e0, "content_140b199e0", fn_140b199e0 as *const (), Throws::Discard), // sub_140b199e0
    Replacement::draft_throws(0x140b19b30, "content_140b19b30", fn_140b19b30 as *const (), Throws::Caught), // sub_140b19b30
    Replacement::draft_throws(0x140b19f70, "content_140b19f70", fn_140b19f70 as *const (), Throws::Caught), // sub_140b19f70
    Replacement::draft_throws(0x140b1a330, "content_140b1a330", fn_140b1a330 as *const (), Throws::Caught), // sub_140b1a330
    Replacement::draft(0x140b1a6e0, "content_140b1a6e0", fn_140b1a6e0 as *const ()), // sub_140b1a6e0
    Replacement::draft(0x140b1a8e0, "content_140b1a8e0", fn_140b1a8e0 as *const ()), // sub_140b1a8e0
    Replacement::draft(0x140b1a9f0, "content_140b1a9f0", fn_140b1a9f0 as *const ()), // sub_140b1a9f0
    Replacement::draft_throws(0x140b1ab10, "content_140b1ab10", fn_140b1ab10 as *const (), Throws::Discard), // sub_140b1ab10
    Replacement::draft_throws(0x140b1ac40, "content_140b1ac40", fn_140b1ac40 as *const (), Throws::Discard), // sub_140b1ac40
    Replacement::draft(0x140b1ae30, "content_140b1ae30", fn_140b1ae30 as *const ()), // sub_140b1ae30
    Replacement::draft(0x140b1af10, "content_140b1af10", fn_140b1af10 as *const ()), // sub_140b1af10
    Replacement::draft(0x140b1afb0, "content_140b1afb0", tpl_13753::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x14320b7c0, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_140b1afb0
    Replacement::draft(0x140b1b060, "content_140b1b060", fn_140b1b060 as *const ()), // sub_140b1b060
    Replacement::draft(0x140b1b130, "content_140b1b130", fn_140b1b130 as *const ()), // sub_140b1b130
    Replacement::draft_throws(0x140b1b320, "content_140b1b320", fn_140b1b320 as *const (), Throws::Discard), // sub_140b1b320
    Replacement::draft(0x140b1b560, "content_140b1b560", fn_140b1b560 as *const ()), // sub_140b1b560
    Replacement::draft_throws(0x140b1b6e0, "content_140b1b6e0", fn_140b1b6e0 as *const (), Throws::Discard), // sub_140b1b6e0
    Replacement::draft_throws(0x140b1b8f0, "content_140b1b8f0", fn_140b1b8f0 as *const (), Throws::Caught), // sub_140b1b8f0
    Replacement::draft_throws(0x140b1bf80, "content_140b1bf80", fn_140b1bf80 as *const (), Throws::Caught), // sub_140b1bf80
    Replacement::draft_throws(0x140b1c4f0, "content_140b1c4f0", fn_140b1c4f0 as *const (), Throws::Caught), // sub_140b1c4f0
    Replacement::draft(0x140b1ca50, "content_140b1ca50", fn_140b1ca50 as *const ()), // sub_140b1ca50
    Replacement::draft(0x140b1cb80, "content_140b1cb80", fn_140b1cb80 as *const ()), // sub_140b1cb80
    Replacement::draft(0x140b1cd10, "content_140b1cd10", fn_140b1cd10 as *const ()), // sub_140b1cd10
    Replacement::draft_throws(0x140b1ceb0, "content_140b1ceb0", fn_140b1ceb0 as *const (), Throws::Discard), // sub_140b1ceb0
    Replacement::draft_throws(0x140b1d010, "content_140b1d010", fn_140b1d010 as *const (), Throws::Discard), // sub_140b1d010
    Replacement::draft_throws(0x140b1d1f0, "content_140b1d1f0", fn_140b1d1f0 as *const (), Throws::Discard), // sub_140b1d1f0
    Replacement::draft(0x140b1d310, "content_140b1d310", fn_140b1d310 as *const ()), // sub_140b1d310
    Replacement::draft(0x140b1d400, "content_140b1d400", fn_140b1d400 as *const ()), // sub_140b1d400
    Replacement::draft(0x140b1d520, "content_140b1d520", fn_140b1d520 as *const ()), // sub_140b1d520
    Replacement::draft(0x140b1d700, "content_140b1d700", fn_140b1d700 as *const ()), // sub_140b1d700
    Replacement::draft_throws(0x140b1dcd0, "content_140b1dcd0", fn_140b1dcd0 as *const (), Throws::Caught), // sub_140b1dcd0
    Replacement::draft(0x140b1e2e0, "content_140b1e2e0", fn_140b1e2e0 as *const ()), // sub_140b1e2e0
    Replacement::draft_throws(0x140b1e700, "content_140b1e700", fn_140b1e700 as *const (), Throws::Discard), // sub_140b1e700
    Replacement::draft(0x140b1ecb0, "content_140b1ecb0", fn_140b1ecb0 as *const ()), // sub_140b1ecb0
    Replacement::draft_throws(0x140b1ee10, "content_140b1ee10", fn_140b1ee10 as *const (), Throws::Caught), // sub_140b1ee10
    Replacement::draft_throws(0x140b1f7e0, "content_140b1f7e0", fn_140b1f7e0 as *const (), Throws::Caught), // sub_140b1f7e0
    Replacement::draft_throws(0x140b20090, "content_140b20090", fn_140b20090 as *const (), Throws::Caught), // sub_140b20090
    Replacement::draft(0x140b20940, "content_140b20940", fn_140b20940 as *const ()), // sub_140b20940
    Replacement::draft(0x140b20ed0, "content_140b20ed0", fn_140b20ed0 as *const ()), // sub_140b20ed0
    Replacement::draft(0x140b21310, "content_140b21310", fn_140b21310 as *const ()), // sub_140b21310
    Replacement::draft_throws(0x140b21760, "content_140b21760", fn_140b21760 as *const (), Throws::Discard), // sub_140b21760
    Replacement::draft_throws(0x140b21bb0, "content_140b21bb0", fn_140b21bb0 as *const (), Throws::Caught), // sub_140b21bb0
    Replacement::draft(0x140b221b0, "content_140b221b0", fn_140b221b0 as *const ()), // sub_140b221b0
    Replacement::draft(0x140b224c0, "content_140b224c0", fn_140b224c0 as *const ()), // sub_140b224c0
    Replacement::draft(0x140b22540, "content_140b22540", fn_140b22540 as *const ()), // sub_140b22540
    Replacement::draft(0x140b225e0, "content_140b225e0", tpl_13738::<0x14387f068, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b225e0
    Replacement::draft(0x140b22650, "content_140b22650", fn_140b22650 as *const ()), // sub_140b22650
    Replacement::draft_throws(0x140b22830, "content_140b22830", fn_140b22830 as *const (), Throws::Caught), // sub_140b22830
    Replacement::draft(0x140b22a80, "content_140b22a80", fn_140b22a80 as *const ()), // sub_140b22a80
    Replacement::draft_throws(0x140b22c30, "content_140b22c30", fn_140b22c30 as *const (), Throws::Discard), // sub_140b22c30
    Replacement::draft_throws(0x140b22d70, "content_140b22d70", fn_140b22d70 as *const (), Throws::Caught), // sub_140b22d70
    Replacement::draft_throws(0x140b23390, "content_140b23390", fn_140b23390 as *const (), Throws::Caught), // sub_140b23390
    Replacement::draft_throws(0x140b238b0, "content_140b238b0", fn_140b238b0 as *const (), Throws::Caught), // sub_140b238b0
    Replacement::draft(0x140b23dd0, "content_140b23dd0", fn_140b23dd0 as *const ()), // sub_140b23dd0
    Replacement::draft(0x140b24130, "content_140b24130", fn_140b24130 as *const ()), // sub_140b24130
    Replacement::draft(0x140b24290, "content_140b24290", fn_140b24290 as *const ()), // sub_140b24290
    Replacement::draft_throws(0x140b24400, "content_140b24400", fn_140b24400 as *const (), Throws::Discard), // sub_140b24400
    Replacement::draft_throws(0x140b24570, "content_140b24570", fn_140b24570 as *const (), Throws::Discard), // sub_140b24570
    Replacement::draft(0x140b24770, "content_140b24770", fn_140b24770 as *const ()), // sub_140b24770
    Replacement::draft(0x140b24830, "content_140b24830", fn_140b24830 as *const ()), // sub_140b24830
    Replacement::draft(0x140b24910, "content_140b24910", fn_140b24910 as *const ()), // sub_140b24910
    Replacement::draft(0x140b24a50, "content_140b24a50", fn_140b24a50 as *const ()), // sub_140b24a50
    Replacement::draft_throws(0x140b24ce0, "content_140b24ce0", fn_140b24ce0 as *const (), Throws::Discard), // sub_140b24ce0
    Replacement::draft(0x140b24e60, "content_140b24e60", fn_140b24e60 as *const ()), // sub_140b24e60
    Replacement::draft_throws(0x140b25000, "content_140b25000", fn_140b25000 as *const (), Throws::Discard), // sub_140b25000
    Replacement::draft(0x140b25190, "content_140b25190", fn_140b25190 as *const ()), // sub_140b25190
    Replacement::draft_throws(0x140b252b0, "content_140b252b0", fn_140b252b0 as *const (), Throws::Caught), // sub_140b252b0
    Replacement::draft_throws(0x140b255e0, "content_140b255e0", fn_140b255e0 as *const (), Throws::Caught), // sub_140b255e0
    Replacement::draft_throws(0x140b258e0, "content_140b258e0", fn_140b258e0 as *const (), Throws::Caught), // sub_140b258e0
    Replacement::draft(0x140b25be0, "content_140b25be0", fn_140b25be0 as *const ()), // sub_140b25be0
    Replacement::draft(0x140b25d70, "content_140b25d70", fn_140b25d70 as *const ()), // sub_140b25d70
    Replacement::draft(0x140b25f10, "content_140b25f10", fn_140b25f10 as *const ()), // sub_140b25f10
    Replacement::draft_throws(0x140b260b0, "content_140b260b0", fn_140b260b0 as *const (), Throws::Discard), // sub_140b260b0
    Replacement::draft_throws(0x140b26250, "content_140b26250", fn_140b26250 as *const (), Throws::Discard), // sub_140b26250
    Replacement::draft(0x140b26420, "content_140b26420", tpl_13807::<0x38, 0x1426cdcb0, 0x140c0c890> as *const ()), // sub_140b26420
    Replacement::draft(0x140b26490, "content_140b26490", tpl_13807::<0x38, 0x1432b4800, 0x140c0c890> as *const ()), // sub_140b26490
    Replacement::draft(0x140b26500, "content_140b26500", fn_140b26500 as *const ()), // sub_140b26500
    Replacement::draft(0x140b26620, "content_140b26620", fn_140b26620 as *const ()), // sub_140b26620
    Replacement::draft(0x140b26790, "content_140b26790", tpl_13809::<0x14387ea40, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b26790
    Replacement::draft(0x140b26800, "content_140b26800", fn_140b26800 as *const ()), // sub_140b26800
    Replacement::draft_throws(0x140b26ab0, "content_140b26ab0", fn_140b26ab0 as *const (), Throws::Discard), // sub_140b26ab0
    Replacement::draft(0x140b26cf0, "content_140b26cf0", fn_140b26cf0 as *const ()), // sub_140b26cf0
    Replacement::draft_throws(0x140b26e00, "content_140b26e00", fn_140b26e00 as *const (), Throws::Discard), // sub_140b26e00
    Replacement::draft(0x140b27070, "content_140b27070", fn_140b27070 as *const ()), // sub_140b27070
    Replacement::draft_throws(0x140b27130, "content_140b27130", fn_140b27130 as *const (), Throws::Caught), // sub_140b27130
    Replacement::draft_throws(0x140b27360, "content_140b27360", fn_140b27360 as *const (), Throws::Caught), // sub_140b27360
    Replacement::draft_throws(0x140b27590, "content_140b27590", fn_140b27590 as *const (), Throws::Caught), // sub_140b27590
    Replacement::draft(0x140b27c90, "content_140b27c90", fn_140b27c90 as *const ()), // sub_140b27c90
    Replacement::draft(0x140b27dc0, "content_140b27dc0", fn_140b27dc0 as *const ()), // sub_140b27dc0
    Replacement::draft(0x140b27ed0, "content_140b27ed0", fn_140b27ed0 as *const ()), // sub_140b27ed0
    Replacement::draft_throws(0x140b27ff0, "content_140b27ff0", fn_140b27ff0 as *const (), Throws::Discard), // sub_140b27ff0
    Replacement::draft_throws(0x140b28120, "content_140b28120", fn_140b28120 as *const (), Throws::Discard), // sub_140b28120
    Replacement::draft(0x140b283e0, "content_140b283e0", fn_140b283e0 as *const ()), // sub_140b283e0
    Replacement::draft(0x140b28510, "content_140b28510", fn_140b28510 as *const ()), // sub_140b28510
    Replacement::draft(0x140b285b0, "content_140b285b0", fn_140b285b0 as *const ()), // sub_140b285b0
    Replacement::draft(0x140b28660, "content_140b28660", tpl_13809::<0x14387efe0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b28660
    Replacement::draft(0x140b286d0, "content_140b286d0", fn_140b286d0 as *const ()), // sub_140b286d0
    Replacement::draft_throws(0x140b28930, "content_140b28930", fn_140b28930 as *const (), Throws::Caught), // sub_140b28930
    Replacement::draft(0x140b28b80, "content_140b28b80", fn_140b28b80 as *const ()), // sub_140b28b80
    Replacement::draft_throws(0x140b28d00, "content_140b28d00", fn_140b28d00 as *const (), Throws::Discard), // sub_140b28d00
    Replacement::draft_throws(0x140b28f20, "content_140b28f20", fn_140b28f20 as *const (), Throws::Caught), // sub_140b28f20
    Replacement::draft_throws(0x140b29580, "content_140b29580", fn_140b29580 as *const (), Throws::Caught), // sub_140b29580
    Replacement::draft_throws(0x140b29b20, "content_140b29b20", fn_140b29b20 as *const (), Throws::Caught), // sub_140b29b20
    Replacement::draft(0x140b2a0c0, "content_140b2a0c0", fn_140b2a0c0 as *const ()), // sub_140b2a0c0
    Replacement::draft(0x140b2a2e0, "content_140b2a2e0", fn_140b2a2e0 as *const ()), // sub_140b2a2e0
    Replacement::draft(0x140b2a450, "content_140b2a450", fn_140b2a450 as *const ()), // sub_140b2a450
    Replacement::draft_throws(0x140b2a5d0, "content_140b2a5d0", fn_140b2a5d0 as *const (), Throws::Discard), // sub_140b2a5d0
    Replacement::draft_throws(0x140b2a770, "content_140b2a770", fn_140b2a770 as *const (), Throws::Discard), // sub_140b2a770
    Replacement::draft(0x140b2aaa0, "content_140b2aaa0", fn_140b2aaa0 as *const ()), // sub_140b2aaa0
    Replacement::draft(0x140b2ac90, "content_140b2ac90", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432c4fa0, 0x8> as *const ()), // sub_140b2ac90
    Replacement::draft(0x140b2ad10, "content_140b2ad10", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c4fa0> as *const ()), // sub_140b2ad10
    Replacement::draft(0x140b2ae10, "content_140b2ae10", tpl_11781::<0x14387ee88, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b2ae10
    Replacement::draft_throws(0x140b2aea0, "content_140b2aea0", fn_140b2aea0 as *const (), Throws::Discard), // sub_140b2aea0
    Replacement::draft(0x140b2af20, "content_140b2af20", tpl_11361::<0x14387ee88, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b2af20
    Replacement::draft_throws(0x140b2afa0, "content_140b2afa0", fn_140b2afa0 as *const (), Throws::Discard), // sub_140b2afa0
    Replacement::draft_throws(0x140b2b030, "content_140b2b030", fn_140b2b030 as *const (), Throws::Caught), // sub_140b2b030
    Replacement::draft_throws(0x140b2b100, "content_140b2b100", fn_140b2b100 as *const (), Throws::Caught), // sub_140b2b100
    Replacement::draft_throws(0x140b2b1d0, "content_140b2b1d0", fn_140b2b1d0 as *const (), Throws::Caught), // sub_140b2b1d0
    Replacement::draft(0x140b2b2a0, "content_140b2b2a0", fn_140b2b2a0 as *const ()), // sub_140b2b2a0
    Replacement::draft(0x140b2b310, "content_140b2b310", tpl_9203::<0x14387ee00, 0x14387ee88, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140b2b310
    Replacement::draft(0x140b2b380, "content_140b2b380", tpl_11361::<0x14387ee88, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b2b380
    Replacement::draft_throws(0x140b2b400, "content_140b2b400", tpl_11786::<0x14387ee00, 0x14387ee88, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b2b400
    Replacement::draft_throws(0x140b2b470, "content_140b2b470", fn_140b2b470 as *const (), Throws::Discard), // sub_140b2b470
    Replacement::draft(0x140b2b570, "content_140b2b570", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431f8350, 0x8> as *const ()), // sub_140b2b570
    Replacement::draft(0x140b2b5f0, "content_140b2b5f0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431f8350> as *const ()), // sub_140b2b5f0
    Replacement::draft(0x140b2b680, "content_140b2b680", fn_140b2b680 as *const ()), // sub_140b2b680
    Replacement::draft(0x140b2b850, "content_140b2b850", fn_140b2b850 as *const ()), // sub_140b2b850
    Replacement::draft_throws(0x140b2ba60, "content_140b2ba60", fn_140b2ba60 as *const (), Throws::Discard), // sub_140b2ba60
    Replacement::draft(0x140b2bc50, "content_140b2bc50", fn_140b2bc50 as *const ()), // sub_140b2bc50
    Replacement::draft_throws(0x140b2be30, "content_140b2be30", fn_140b2be30 as *const (), Throws::Discard), // sub_140b2be30
    Replacement::draft_throws(0x140b2c030, "content_140b2c030", fn_140b2c030 as *const (), Throws::Caught), // sub_140b2c030
    Replacement::draft_throws(0x140b2c130, "content_140b2c130", fn_140b2c130 as *const (), Throws::Caught), // sub_140b2c130
    Replacement::draft_throws(0x140b2c230, "content_140b2c230", fn_140b2c230 as *const (), Throws::Caught), // sub_140b2c230
    Replacement::draft(0x140b2c330, "content_140b2c330", fn_140b2c330 as *const ()), // sub_140b2c330
    Replacement::draft(0x140b2c510, "content_140b2c510", fn_140b2c510 as *const ()), // sub_140b2c510
    Replacement::draft(0x140b2c6f0, "content_140b2c6f0", fn_140b2c6f0 as *const ()), // sub_140b2c6f0
    Replacement::draft_throws(0x140b2c8d0, "content_140b2c8d0", fn_140b2c8d0 as *const (), Throws::Discard), // sub_140b2c8d0
    Replacement::draft_throws(0x140b2cab0, "content_140b2cab0", fn_140b2cab0 as *const (), Throws::Discard), // sub_140b2cab0
    Replacement::draft(0x140b2cbc0, "content_140b2cbc0", fn_140b2cbc0 as *const ()), // sub_140b2cbc0
    Replacement::draft(0x140b2cc70, "content_140b2cc70", fn_140b2cc70 as *const ()), // sub_140b2cc70
    Replacement::draft(0x140b2cd20, "content_140b2cd20", fn_140b2cd20 as *const ()), // sub_140b2cd20
    Replacement::draft(0x140b2cf20, "content_140b2cf20", fn_140b2cf20 as *const ()), // sub_140b2cf20
    Replacement::draft_throws(0x140b2d4a0, "content_140b2d4a0", fn_140b2d4a0 as *const (), Throws::Caught), // sub_140b2d4a0
    Replacement::draft(0x140b2da10, "content_140b2da10", fn_140b2da10 as *const ()), // sub_140b2da10
    Replacement::draft_throws(0x140b2ddc0, "content_140b2ddc0", fn_140b2ddc0 as *const (), Throws::Discard), // sub_140b2ddc0
    Replacement::draft(0x140b2e370, "content_140b2e370", fn_140b2e370 as *const ()), // sub_140b2e370
    Replacement::draft_throws(0x140b2e490, "content_140b2e490", fn_140b2e490 as *const (), Throws::Caught), // sub_140b2e490
    Replacement::draft_throws(0x140b2ed20, "content_140b2ed20", fn_140b2ed20 as *const (), Throws::Caught), // sub_140b2ed20
    Replacement::draft_throws(0x140b2f4f0, "content_140b2f4f0", fn_140b2f4f0 as *const (), Throws::Caught), // sub_140b2f4f0
    Replacement::draft(0x140b2fcc0, "content_140b2fcc0", fn_140b2fcc0 as *const ()), // sub_140b2fcc0
    Replacement::draft(0x140b300e0, "content_140b300e0", fn_140b300e0 as *const ()), // sub_140b300e0
    Replacement::draft(0x140b304d0, "content_140b304d0", fn_140b304d0 as *const ()), // sub_140b304d0
    Replacement::draft_throws(0x140b308d0, "content_140b308d0", fn_140b308d0 as *const (), Throws::Discard), // sub_140b308d0
    Replacement::draft_throws(0x140b30cf0, "content_140b30cf0", fn_140b30cf0 as *const (), Throws::Caught), // sub_140b30cf0
    Replacement::draft(0x140b31370, "content_140b31370", fn_140b31370 as *const ()), // sub_140b31370
    Replacement::draft(0x140b317f0, "content_140b317f0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431f5318, 0x8> as *const ()), // sub_140b317f0
    Replacement::draft(0x140b31870, "content_140b31870", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431f5318> as *const ()), // sub_140b31870
    Replacement::draft(0x140b31900, "content_140b31900", tpl_9031::<0x14387ebb0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b31900
    Replacement::draft(0x140b31970, "content_140b31970", tpl_13868::<0x14387ebb0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b31970
    Replacement::draft_throws(0x140b31a20, "content_140b31a20", fn_140b31a20 as *const (), Throws::Discard), // sub_140b31a20
    Replacement::draft(0x140b31ab0, "content_140b31ab0", tpl_12080::<0x14387ebb0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b31ab0
    Replacement::draft_throws(0x140b31b30, "content_140b31b30", fn_140b31b30 as *const (), Throws::Discard), // sub_140b31b30
    Replacement::draft_throws(0x140b31bd0, "content_140b31bd0", fn_140b31bd0 as *const (), Throws::Caught), // sub_140b31bd0
    Replacement::draft_throws(0x140b31cb0, "content_140b31cb0", fn_140b31cb0 as *const (), Throws::Caught), // sub_140b31cb0
    Replacement::draft_throws(0x140b31d90, "content_140b31d90", fn_140b31d90 as *const (), Throws::Caught), // sub_140b31d90
    Replacement::draft(0x140b31e70, "content_140b31e70", fn_140b31e70 as *const ()), // sub_140b31e70
    Replacement::draft(0x140b31ee0, "content_140b31ee0", tpl_12084::<0x14387eee8, 0x14387ebb0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0x1401e9860> as *const ()), // sub_140b31ee0
    Replacement::draft(0x140b31f50, "content_140b31f50", tpl_12080::<0x14387ebb0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b31f50
    Replacement::draft_throws(0x140b31fd0, "content_140b31fd0", tpl_13873::<0x14387eee8, 0x14387ebb0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140b31fd0
    Replacement::draft_throws(0x140b32050, "content_140b32050", tpl_12133::<0x14387ebb0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1434266b8, 0x8, 0x1432b5b08, 0x1432b5bc0, 0x1432b5ac8, 0x14020f240> as *const (), Throws::Discard), // sub_140b32050
    Replacement::draft(0x140b32150, "content_140b32150", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430e2a48, 0x8> as *const ()), // sub_140b32150
    Replacement::draft(0x140b321d0, "content_140b321d0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430e2a48> as *const ()), // sub_140b321d0
    Replacement::draft(0x140b322d0, "content_140b322d0", tpl_13874::<0x14387ebe0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1426d4580, 0x338, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b322d0
    Replacement::draft_throws(0x140b32390, "content_140b32390", fn_140b32390 as *const (), Throws::Caught), // sub_140b32390
    Replacement::draft(0x140b32460, "content_140b32460", tpl_13876::<0x14387ebe0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x20, 0x40, 0x8, 0x14240ab10, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140b32460
    Replacement::draft_throws(0x140b32520, "content_140b32520", tpl_7839::<0x14387f0e8, 0x14387ebe0, 0x1401e9b60, 0x1406e03a0, 0x1426cdcb0, 0x8, 0x1402c1540, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b32520
    Replacement::draft_throws(0x140b325a0, "content_140b325a0", fn_140b325a0 as *const (), Throws::Caught), // sub_140b325a0
    Replacement::draft_throws(0x140b32790, "content_140b32790", fn_140b32790 as *const (), Throws::Caught), // sub_140b32790
    Replacement::draft_throws(0x140b32950, "content_140b32950", fn_140b32950 as *const (), Throws::Caught), // sub_140b32950
    Replacement::draft(0x140b32b10, "content_140b32b10", tpl_13880::<0x14387ebe0, 0x14243b0b0, 0x1406e01b0, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_140b32b10
    Replacement::draft(0x140b32c40, "content_140b32c40", tpl_7871::<0x14387f0e8, 0x14387ebe0, 0x1401e9970, 0x1406e1480, 0x338, 0x1426cdcb0, 0x8, 0xa0, 0xe0, 0x1401e9860> as *const ()), // sub_140b32c40
    Replacement::draft(0x140b32ce0, "content_140b32ce0", tpl_13881::<0x14387ebe0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426cdcb0, 0x8, 0xa0, 0xe0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b32ce0
    Replacement::draft_throws(0x140b32d80, "content_140b32d80", tpl_7873::<0x14387f0e8, 0x14387ebe0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b32d80
    Replacement::draft_throws(0x140b32e10, "content_140b32e10", tpl_13882::<0x14387ebe0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1426cdcb0, 0x1432b5b68, 0x1432b5ba0, 0x1432b5b80, 0x1402bf8c0> as *const (), Throws::Discard), // sub_140b32e10
    Replacement::draft(0x140b32f00, "content_140b32f00", fn_140b32f00 as *const ()), // sub_140b32f00
    Replacement::draft(0x140b32fb0, "content_140b32fb0", fn_140b32fb0 as *const ()), // sub_140b32fb0
    Replacement::draft(0x140b330c0, "content_140b330c0", fn_140b330c0 as *const ()), // sub_140b330c0
    Replacement::draft(0x140b33160, "content_140b33160", fn_140b33160 as *const ()), // sub_140b33160
    Replacement::draft_throws(0x140b332f0, "content_140b332f0", fn_140b332f0 as *const (), Throws::Discard), // sub_140b332f0
    Replacement::draft(0x140b33430, "content_140b33430", fn_140b33430 as *const ()), // sub_140b33430
    Replacement::draft_throws(0x140b33520, "content_140b33520", fn_140b33520 as *const (), Throws::Discard), // sub_140b33520
    Replacement::draft(0x140b33690, "content_140b33690", fn_140b33690 as *const ()), // sub_140b33690
    Replacement::draft_throws(0x140b33810, "content_140b33810", fn_140b33810 as *const (), Throws::Caught), // sub_140b33810
    Replacement::draft_throws(0x140b33ae0, "content_140b33ae0", fn_140b33ae0 as *const (), Throws::Caught), // sub_140b33ae0
    Replacement::draft_throws(0x140b33d90, "content_140b33d90", fn_140b33d90 as *const (), Throws::Caught), // sub_140b33d90
    Replacement::draft(0x140b34030, "content_140b34030", fn_140b34030 as *const ()), // sub_140b34030
    Replacement::draft(0x140b34120, "content_140b34120", fn_140b34120 as *const ()), // sub_140b34120
    Replacement::draft(0x140b34210, "content_140b34210", fn_140b34210 as *const ()), // sub_140b34210
    Replacement::draft_throws(0x140b34310, "content_140b34310", fn_140b34310 as *const (), Throws::Discard), // sub_140b34310
    Replacement::draft_throws(0x140b34400, "content_140b34400", fn_140b34400 as *const (), Throws::Discard), // sub_140b34400
    Replacement::draft(0x140b34570, "content_140b34570", fn_140b34570 as *const ()), // sub_140b34570
    Replacement::draft(0x140b345e0, "content_140b345e0", tpl_6721::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1430ab250> as *const ()), // sub_140b345e0
    Replacement::draft(0x140b34670, "content_140b34670", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430ab250, 0x18> as *const ()), // sub_140b34670
    Replacement::draft(0x140b34720, "content_140b34720", tpl_6766::<0x14387ec00, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b34720
    Replacement::draft(0x140b34790, "content_140b34790", fn_140b34790 as *const ()), // sub_140b34790
    Replacement::draft_throws(0x140b348a0, "content_140b348a0", fn_140b348a0 as *const (), Throws::Discard), // sub_140b348a0
    Replacement::draft(0x140b34990, "content_140b34990", tpl_13898::<0x14387ec00, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x48, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b34990
    Replacement::draft_throws(0x140b34a30, "content_140b34a30", fn_140b34a30 as *const (), Throws::Discard), // sub_140b34a30
    Replacement::draft_throws(0x140b34b30, "content_140b34b30", fn_140b34b30 as *const (), Throws::Caught), // sub_140b34b30
    Replacement::draft_throws(0x140b34ce0, "content_140b34ce0", fn_140b34ce0 as *const (), Throws::Caught), // sub_140b34ce0
    Replacement::draft_throws(0x140b34e90, "content_140b34e90", fn_140b34e90 as *const (), Throws::Caught), // sub_140b34e90
    Replacement::draft(0x140b35040, "content_140b35040", fn_140b35040 as *const ()), // sub_140b35040
    Replacement::draft(0x140b350c0, "content_140b350c0", fn_140b350c0 as *const ()), // sub_140b350c0
    Replacement::draft(0x140b35160, "content_140b35160", tpl_13898::<0x14387ec00, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0xe8, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b35160
    Replacement::draft_throws(0x140b35210, "content_140b35210", fn_140b35210 as *const (), Throws::Discard), // sub_140b35210
    Replacement::draft_throws(0x140b352c0, "content_140b352c0", fn_140b352c0 as *const (), Throws::Discard), // sub_140b352c0
    Replacement::draft(0x140b35480, "content_140b35480", fn_140b35480 as *const ()), // sub_140b35480
    Replacement::draft(0x140b35540, "content_140b35540", fn_140b35540 as *const ()), // sub_140b35540
    Replacement::draft(0x140b355e0, "content_140b355e0", fn_140b355e0 as *const ()), // sub_140b355e0
    Replacement::draft(0x140b35690, "content_140b35690", fn_140b35690 as *const ()), // sub_140b35690
    Replacement::draft(0x140b35730, "content_140b35730", fn_140b35730 as *const ()), // sub_140b35730
    Replacement::draft_throws(0x140b35910, "content_140b35910", fn_140b35910 as *const (), Throws::Discard), // sub_140b35910
    Replacement::draft(0x140b35b00, "content_140b35b00", fn_140b35b00 as *const ()), // sub_140b35b00
    Replacement::draft_throws(0x140b35c20, "content_140b35c20", fn_140b35c20 as *const (), Throws::Discard), // sub_140b35c20
    Replacement::draft(0x140b35e40, "content_140b35e40", fn_140b35e40 as *const ()), // sub_140b35e40
    Replacement::draft_throws(0x140b35ec0, "content_140b35ec0", fn_140b35ec0 as *const (), Throws::Caught), // sub_140b35ec0
    Replacement::draft_throws(0x140b362b0, "content_140b362b0", fn_140b362b0 as *const (), Throws::Caught), // sub_140b362b0
    Replacement::draft_throws(0x140b36670, "content_140b36670", fn_140b36670 as *const (), Throws::Caught), // sub_140b36670
    Replacement::draft(0x140b36a20, "content_140b36a20", fn_140b36a20 as *const ()), // sub_140b36a20
    Replacement::draft(0x140b36b20, "content_140b36b20", fn_140b36b20 as *const ()), // sub_140b36b20
    Replacement::draft(0x140b36ca0, "content_140b36ca0", fn_140b36ca0 as *const ()), // sub_140b36ca0
    Replacement::draft_throws(0x140b36e30, "content_140b36e30", fn_140b36e30 as *const (), Throws::Discard), // sub_140b36e30
    Replacement::draft_throws(0x140b36f80, "content_140b36f80", fn_140b36f80 as *const (), Throws::Caught), // sub_140b36f80
    Replacement::draft(0x140b372a0, "content_140b372a0", fn_140b372a0 as *const ()), // sub_140b372a0
    Replacement::draft(0x140b37460, "content_140b37460", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430fddd0, 0x8> as *const ()), // sub_140b37460
    Replacement::draft(0x140b374e0, "content_140b374e0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430fddd0> as *const ()), // sub_140b374e0
    Replacement::draft(0x140b37570, "content_140b37570", fn_140b37570 as *const ()), // sub_140b37570
    Replacement::draft(0x140b37670, "content_140b37670", fn_140b37670 as *const ()), // sub_140b37670
    Replacement::draft_throws(0x140b377f0, "content_140b377f0", fn_140b377f0 as *const (), Throws::Discard), // sub_140b377f0
    Replacement::draft(0x140b37940, "content_140b37940", fn_140b37940 as *const ()), // sub_140b37940
    Replacement::draft_throws(0x140b37a70, "content_140b37a70", fn_140b37a70 as *const (), Throws::Discard), // sub_140b37a70
    Replacement::draft_throws(0x140b37be0, "content_140b37be0", fn_140b37be0 as *const (), Throws::Caught), // sub_140b37be0
    Replacement::draft_throws(0x140b37d60, "content_140b37d60", fn_140b37d60 as *const (), Throws::Caught), // sub_140b37d60
    Replacement::draft_throws(0x140b37ee0, "content_140b37ee0", fn_140b37ee0 as *const (), Throws::Caught), // sub_140b37ee0
    Replacement::draft(0x140b38060, "content_140b38060", fn_140b38060 as *const ()), // sub_140b38060
    Replacement::draft(0x140b38170, "content_140b38170", fn_140b38170 as *const ()), // sub_140b38170
    Replacement::draft(0x140b38290, "content_140b38290", fn_140b38290 as *const ()), // sub_140b38290
    Replacement::draft_throws(0x140b383c0, "content_140b383c0", fn_140b383c0 as *const (), Throws::Discard), // sub_140b383c0
    Replacement::draft_throws(0x140b384f0, "content_140b384f0", fn_140b384f0 as *const (), Throws::Discard), // sub_140b384f0
    Replacement::draft(0x140b38690, "content_140b38690", fn_140b38690 as *const ()), // sub_140b38690
    Replacement::draft(0x140b38760, "content_140b38760", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431f4068, 0x8> as *const ()), // sub_140b38760
    Replacement::draft(0x140b387e0, "content_140b387e0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431f4068> as *const ()), // sub_140b387e0
    Replacement::draft(0x140b388e0, "content_140b388e0", tpl_12161::<0x14387e958, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b388e0
    Replacement::draft_throws(0x140b38990, "content_140b38990", tpl_7851::<0x14387ec80, 0x14387e958, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b38990
    Replacement::draft(0x140b38a20, "content_140b38a20", tpl_9969::<0x14387e958, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b38a20
    Replacement::draft_throws(0x140b38aa0, "content_140b38aa0", tpl_7972::<0x14387ec80, 0x14387e958, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b38aa0
    Replacement::draft_throws(0x140b38b30, "content_140b38b30", fn_140b38b30 as *const (), Throws::Caught), // sub_140b38b30
    Replacement::draft_throws(0x140b38d20, "content_140b38d20", fn_140b38d20 as *const (), Throws::Caught), // sub_140b38d20
    Replacement::draft_throws(0x140b38ee0, "content_140b38ee0", fn_140b38ee0 as *const (), Throws::Caught), // sub_140b38ee0
    Replacement::draft(0x140b390a0, "content_140b390a0", tpl_12163::<0x14387e958, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b390a0
    Replacement::draft(0x140b39110, "content_140b39110", tpl_8074::<0x14387ec80, 0x14387e958, 0x1401e9970, 0x1406e1480, 0x8, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_140b39110
    Replacement::draft(0x140b39190, "content_140b39190", tpl_13937::<0x14387e958, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b39190
    Replacement::draft_throws(0x140b39220, "content_140b39220", tpl_7974::<0x14387ec80, 0x14387e958, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b39220
    Replacement::draft_throws(0x140b392a0, "content_140b392a0", tpl_13938::<0x14387e958, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x1432b5510> as *const (), Throws::Discard), // sub_140b392a0
    Replacement::draft(0x140b39380, "content_140b39380", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432cba38> as *const ()), // sub_140b39380
    Replacement::draft(0x140b393e0, "content_140b393e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cba38> as *const ()), // sub_140b393e0
    Replacement::draft(0x140b39470, "content_140b39470", fn_140b39470 as *const ()), // sub_140b39470
    Replacement::draft_throws(0x140b39520, "content_140b39520", tpl_13940::<0x14387ece0, 0x14387eab0, 0x1401e9fa0, 0x1406e02b0, 0x4, 0x1406d7a10, 0x1, 0x3, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b39520
    Replacement::draft_throws(0x140b395c0, "content_140b395c0", tpl_13940::<0x14387ece0, 0x14387eab0, 0x1401e9b60, 0x1406e03a0, 0x4, 0x1406d7a10, 0x1, 0x3, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b395c0
    Replacement::draft_throws(0x140b39660, "content_140b39660", fn_140b39660 as *const (), Throws::Caught), // sub_140b39660
    Replacement::draft(0x140b396f0, "content_140b396f0", fn_140b396f0 as *const ()), // sub_140b396f0
    Replacement::draft_throws(0x140b39790, "content_140b39790", tpl_13940::<0x14387ece0, 0x14387eab0, 0x1401e9770, 0x1406e1570, 0x4, 0x1406d7a10, 0x1, 0x3, 0x1401e9590> as *const (), Throws::Discard), // sub_140b39790
    Replacement::draft_throws(0x140b39830, "content_140b39830", fn_140b39830 as *const (), Throws::Discard), // sub_140b39830
    Replacement::draft(0x140b398e0, "content_140b398e0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431f01a8, 0x8> as *const ()), // sub_140b398e0
    Replacement::draft(0x140b39970, "content_140b39970", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1431f01a8, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140b39970
    Replacement::draft(0x140b39a20, "content_140b39a20", fn_140b39a20 as *const ()), // sub_140b39a20
    Replacement::draft(0x140b39af0, "content_140b39af0", fn_140b39af0 as *const ()), // sub_140b39af0
    Replacement::draft_throws(0x140b39be0, "content_140b39be0", tpl_6937::<0x14387f0c8, 0x14387ed58, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140bfb680, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b39be0
    Replacement::draft(0x140b39c40, "content_140b39c40", fn_140b39c40 as *const ()), // sub_140b39c40
    Replacement::draft_throws(0x140b39d00, "content_140b39d00", tpl_6937::<0x14387f0c8, 0x14387ed58, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140bfb830, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b39d00
    Replacement::draft_throws(0x140b39d60, "content_140b39d60", fn_140b39d60 as *const (), Throws::Caught), // sub_140b39d60
    Replacement::draft_throws(0x140b39e20, "content_140b39e20", fn_140b39e20 as *const (), Throws::Caught), // sub_140b39e20
    Replacement::draft_throws(0x140b39ee0, "content_140b39ee0", fn_140b39ee0 as *const (), Throws::Caught), // sub_140b39ee0
    Replacement::draft(0x140b39fa0, "content_140b39fa0", fn_140b39fa0 as *const ()), // sub_140b39fa0
    Replacement::draft(0x140b3a050, "content_140b3a050", fn_140b3a050 as *const ()), // sub_140b3a050
    Replacement::draft(0x140b3a100, "content_140b3a100", fn_140b3a100 as *const ()), // sub_140b3a100
    Replacement::draft_throws(0x140b3a1c0, "content_140b3a1c0", fn_140b3a1c0 as *const (), Throws::Discard), // sub_140b3a1c0
    Replacement::draft_throws(0x140b3a260, "content_140b3a260", tpl_6941::<0x14387ed58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432b54f0, 0x8, 0x1432b54e0, 0x140bfba20> as *const (), Throws::Discard), // sub_140b3a260
    Replacement::draft(0x140b3a340, "content_140b3a340", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432c77b0> as *const ()), // sub_140b3a340
    Replacement::draft(0x140b3a3a0, "content_140b3a3a0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c77b0> as *const ()), // sub_140b3a3a0
    Replacement::draft(0x140b3a430, "content_140b3a430", fn_140b3a430 as *const ()), // sub_140b3a430
    Replacement::draft_throws(0x140b3a510, "content_140b3a510", tpl_13948::<0x14387f0b0, 0x14387f048, 0x1401e9fa0, 0x1406e02b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x140bfba60, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b3a510
    Replacement::draft_throws(0x140b3a5e0, "content_140b3a5e0", tpl_13948::<0x14387f0b0, 0x14387f048, 0x1401e9b60, 0x1406e03a0, 0x318, 0x328, 0x320, 0x1406e3600, 0x140bfba60, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b3a5e0
    Replacement::draft_throws(0x140b3a6b0, "content_140b3a6b0", fn_140b3a6b0 as *const (), Throws::Caught), // sub_140b3a6b0
    Replacement::draft(0x140b3a740, "content_140b3a740", fn_140b3a740 as *const ()), // sub_140b3a740
    Replacement::draft_throws(0x140b3a810, "content_140b3a810", tpl_13948::<0x14387f0b0, 0x14387f048, 0x1401e9770, 0x1406e1570, 0x318, 0x328, 0x320, 0x1406e3600, 0x140bfba60, 0x1401e9590> as *const (), Throws::Discard), // sub_140b3a810
    Replacement::draft_throws(0x140b3a8e0, "content_140b3a8e0", tpl_6737::<0x14387f048, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b3a8e0
    Replacement::draft(0x140b3a990, "content_140b3a990", fn_140b3a990 as *const ()), // sub_140b3a990
    Replacement::draft(0x140b3aa60, "content_140b3aa60", fn_140b3aa60 as *const ()), // sub_140b3aa60
    Replacement::draft(0x140b3ab10, "content_140b3ab10", fn_140b3ab10 as *const ()), // sub_140b3ab10
    Replacement::draft(0x140b3ae30, "content_140b3ae30", fn_140b3ae30 as *const ()), // sub_140b3ae30
    Replacement::draft_throws(0x140b3b1e0, "content_140b3b1e0", fn_140b3b1e0 as *const (), Throws::Discard), // sub_140b3b1e0
    Replacement::draft(0x140b3b6d0, "content_140b3b6d0", fn_140b3b6d0 as *const ()), // sub_140b3b6d0
    Replacement::draft_throws(0x140b3ba50, "content_140b3ba50", fn_140b3ba50 as *const (), Throws::Discard), // sub_140b3ba50
    Replacement::draft(0x140b3bf50, "content_140b3bf50", fn_140b3bf50 as *const ()), // sub_140b3bf50
    Replacement::draft_throws(0x140b3bfe0, "content_140b3bfe0", fn_140b3bfe0 as *const (), Throws::Caught), // sub_140b3bfe0
    Replacement::draft_throws(0x140b3c1c0, "content_140b3c1c0", fn_140b3c1c0 as *const (), Throws::Caught), // sub_140b3c1c0
    Replacement::draft_throws(0x140b3c3a0, "content_140b3c3a0", fn_140b3c3a0 as *const (), Throws::Caught), // sub_140b3c3a0
    Replacement::draft(0x140b3c580, "content_140b3c580", fn_140b3c580 as *const ()), // sub_140b3c580
    Replacement::draft(0x140b3c9a0, "content_140b3c9a0", fn_140b3c9a0 as *const ()), // sub_140b3c9a0
    Replacement::draft(0x140b3ce40, "content_140b3ce40", fn_140b3ce40 as *const ()), // sub_140b3ce40
    Replacement::draft_throws(0x140b3d2f0, "content_140b3d2f0", fn_140b3d2f0 as *const (), Throws::Discard), // sub_140b3d2f0
    Replacement::draft_throws(0x140b3d670, "content_140b3d670", fn_140b3d670 as *const (), Throws::Caught), // sub_140b3d670
    Replacement::draft(0x140b3da60, "content_140b3da60", fn_140b3da60 as *const ()), // sub_140b3da60
    Replacement::draft(0x140b3db40, "content_140b3db40", fn_140b3db40 as *const ()), // sub_140b3db40
    Replacement::draft(0x140b3dc10, "content_140b3dc10", fn_140b3dc10 as *const ()), // sub_140b3dc10
    Replacement::draft(0x140b3dd20, "content_140b3dd20", fn_140b3dd20 as *const ()), // sub_140b3dd20
    Replacement::draft(0x140b3de00, "content_140b3de00", fn_140b3de00 as *const ()), // sub_140b3de00
    Replacement::draft_throws(0x140b3df60, "content_140b3df60", fn_140b3df60 as *const (), Throws::Discard), // sub_140b3df60
    Replacement::draft(0x140b3e020, "content_140b3e020", fn_140b3e020 as *const ()), // sub_140b3e020
    Replacement::draft_throws(0x140b3e100, "content_140b3e100", fn_140b3e100 as *const (), Throws::Discard), // sub_140b3e100
    Replacement::draft(0x140b3e1c0, "content_140b3e1c0", fn_140b3e1c0 as *const ()), // sub_140b3e1c0
    Replacement::draft_throws(0x140b3e2a0, "content_140b3e2a0", fn_140b3e2a0 as *const (), Throws::Caught), // sub_140b3e2a0
    Replacement::draft_throws(0x140b3e540, "content_140b3e540", fn_140b3e540 as *const (), Throws::Caught), // sub_140b3e540
    Replacement::draft_throws(0x140b3e7a0, "content_140b3e7a0", fn_140b3e7a0 as *const (), Throws::Caught), // sub_140b3e7a0
    Replacement::draft(0x140b3ea00, "content_140b3ea00", fn_140b3ea00 as *const ()), // sub_140b3ea00
    Replacement::draft(0x140b3eaf0, "content_140b3eaf0", fn_140b3eaf0 as *const ()), // sub_140b3eaf0
    Replacement::draft(0x140b3ebc0, "content_140b3ebc0", fn_140b3ebc0 as *const ()), // sub_140b3ebc0
    Replacement::draft_throws(0x140b3eca0, "content_140b3eca0", fn_140b3eca0 as *const (), Throws::Discard), // sub_140b3eca0
    Replacement::draft_throws(0x140b3ed70, "content_140b3ed70", fn_140b3ed70 as *const (), Throws::Discard), // sub_140b3ed70
    Replacement::draft(0x140b3eea0, "content_140b3eea0", fn_140b3eea0 as *const ()), // sub_140b3eea0
    Replacement::draft(0x140b3ef80, "content_140b3ef80", fn_140b3ef80 as *const ()), // sub_140b3ef80
    Replacement::draft(0x140b3f0d0, "content_140b3f0d0", tpl_13978::<0x14387eeb8, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140b3f0d0
    Replacement::draft(0x140b3f170, "content_140b3f170", fn_140b3f170 as *const ()), // sub_140b3f170
    Replacement::draft_throws(0x140b3f330, "content_140b3f330", fn_140b3f330 as *const (), Throws::Discard), // sub_140b3f330
    Replacement::draft(0x140b3f500, "content_140b3f500", fn_140b3f500 as *const ()), // sub_140b3f500
    Replacement::draft_throws(0x140b3f610, "content_140b3f610", fn_140b3f610 as *const (), Throws::Discard), // sub_140b3f610
    Replacement::draft(0x140b3f7f0, "content_140b3f7f0", fn_140b3f7f0 as *const ()), // sub_140b3f7f0
    Replacement::draft_throws(0x140b3f880, "content_140b3f880", fn_140b3f880 as *const (), Throws::Caught), // sub_140b3f880
    Replacement::draft_throws(0x140b3fe40, "content_140b3fe40", fn_140b3fe40 as *const (), Throws::Caught), // sub_140b3fe40
    Replacement::draft_throws(0x140b40370, "content_140b40370", fn_140b40370 as *const (), Throws::Caught), // sub_140b40370
    Replacement::draft(0x140b40880, "content_140b40880", fn_140b40880 as *const ()), // sub_140b40880
    Replacement::draft(0x140b40990, "content_140b40990", fn_140b40990 as *const ()), // sub_140b40990
    Replacement::draft(0x140b40aa0, "content_140b40aa0", fn_140b40aa0 as *const ()), // sub_140b40aa0
    Replacement::draft_throws(0x140b40bc0, "content_140b40bc0", fn_140b40bc0 as *const (), Throws::Discard), // sub_140b40bc0
    Replacement::draft_throws(0x140b40cd0, "content_140b40cd0", fn_140b40cd0 as *const (), Throws::Discard), // sub_140b40cd0
    Replacement::draft(0x140b40eb0, "content_140b40eb0", tpl_8204::<0x128, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140b40f40> as *const ()), // sub_140b40eb0
    Replacement::draft(0x140b40f40, "content_140b40f40", fn_140b40f40 as *const ()), // sub_140b40f40
    Replacement::draft(0x140b41000, "content_140b41000", fn_140b41000 as *const ()), // sub_140b41000
    Replacement::draft(0x140b410b0, "content_140b410b0", fn_140b410b0 as *const ()), // sub_140b410b0
    Replacement::draft(0x140b41200, "content_140b41200", fn_140b41200 as *const ()), // sub_140b41200
    Replacement::draft_throws(0x140b414d0, "content_140b414d0", fn_140b414d0 as *const (), Throws::Discard), // sub_140b414d0
    Replacement::draft_throws(0x140b41860, "content_140b41860", fn_140b41860 as *const (), Throws::Discard), // sub_140b41860
    Replacement::draft_throws(0x140b41ac0, "content_140b41ac0", fn_140b41ac0 as *const (), Throws::Discard), // sub_140b41ac0
    Replacement::draft(0x140b41e70, "content_140b41e70", fn_140b41e70 as *const ()), // sub_140b41e70
    Replacement::draft_throws(0x140b41ff0, "content_140b41ff0", fn_140b41ff0 as *const (), Throws::Caught), // sub_140b41ff0
    Replacement::draft_throws(0x140b424b0, "content_140b424b0", fn_140b424b0 as *const (), Throws::Caught), // sub_140b424b0
    Replacement::draft_throws(0x140b428f0, "content_140b428f0", fn_140b428f0 as *const (), Throws::Caught), // sub_140b428f0
    Replacement::draft(0x140b42d20, "content_140b42d20", fn_140b42d20 as *const ()), // sub_140b42d20
    Replacement::draft(0x140b43130, "content_140b43130", fn_140b43130 as *const ()), // sub_140b43130
    Replacement::draft(0x140b434d0, "content_140b434d0", fn_140b434d0 as *const ()), // sub_140b434d0
    Replacement::draft_throws(0x140b43880, "content_140b43880", fn_140b43880 as *const (), Throws::Discard), // sub_140b43880
    Replacement::draft_throws(0x140b43b00, "content_140b43b00", fn_140b43b00 as *const (), Throws::Caught), // sub_140b43b00
    Replacement::draft(0x140b43f50, "content_140b43f50", fn_140b43f50 as *const ()), // sub_140b43f50
    Replacement::draft_throws(0x140b44000, "content_140b44000", fn_140b44000 as *const (), Throws::Discard), // sub_140b44000
    Replacement::draft(0x140b440d0, "content_140b440d0", fn_140b440d0 as *const ()), // sub_140b440d0
    Replacement::draft(0x140b44230, "content_140b44230", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431d1720, 0x8> as *const ()), // sub_140b44230
    Replacement::draft(0x140b442c0, "content_140b442c0", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1431d1720, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140b442c0
    Replacement::draft(0x140b44370, "content_140b44370", fn_140b44370 as *const ()), // sub_140b44370
    Replacement::draft(0x140b44440, "content_140b44440", fn_140b44440 as *const ()), // sub_140b44440
    Replacement::draft_throws(0x140b44540, "content_140b44540", tpl_14007::<0x14387eb78, 0x14387ee58, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140bfbf90, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b44540
    Replacement::draft(0x140b445b0, "content_140b445b0", fn_140b445b0 as *const ()), // sub_140b445b0
    Replacement::draft_throws(0x140b44680, "content_140b44680", tpl_14007::<0x14387eb78, 0x14387ee58, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140bfc140, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b44680
    Replacement::draft(0x140b446f0, "content_140b446f0", tpl_6939::<0x8, 0x140c0a570, 0x140c7c1b0> as *const ()), // sub_140b446f0
    Replacement::draft_throws(0x140b44770, "content_140b44770", fn_140b44770 as *const (), Throws::Caught), // sub_140b44770
    Replacement::draft_throws(0x140b44840, "content_140b44840", fn_140b44840 as *const (), Throws::Caught), // sub_140b44840
    Replacement::draft_throws(0x140b44910, "content_140b44910", fn_140b44910 as *const (), Throws::Caught), // sub_140b44910
    Replacement::draft(0x140b449e0, "content_140b449e0", fn_140b449e0 as *const ()), // sub_140b449e0
    Replacement::draft(0x140b44a90, "content_140b44a90", fn_140b44a90 as *const ()), // sub_140b44a90
    Replacement::draft(0x140b44b40, "content_140b44b40", fn_140b44b40 as *const ()), // sub_140b44b40
    Replacement::draft_throws(0x140b44c10, "content_140b44c10", fn_140b44c10 as *const (), Throws::Discard), // sub_140b44c10
    Replacement::draft_throws(0x140b44cc0, "content_140b44cc0", tpl_14010::<0x14387ee58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432b6f70, 0x8, 0x1432b7020, 0x1432b7010, 0x140bfc330> as *const (), Throws::Discard), // sub_140b44cc0
    Replacement::draft(0x140b44db0, "content_140b44db0", tpl_14011::<0x10, 0x1426cdcb0, 0x8, 0x140c116d0> as *const ()), // sub_140b44db0
    Replacement::draft(0x140b44e30, "content_140b44e30", tpl_14011::<0x10, 0x1432b7010, 0x8, 0x140c116e0> as *const ()), // sub_140b44e30
    Replacement::draft(0x140b44eb0, "content_140b44eb0", tpl_6676::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1431e3b00, 0x8> as *const ()), // sub_140b44eb0
    Replacement::draft(0x140b44f40, "content_140b44f40", tpl_8299::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1431e3b00, 0x4, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_140b44f40
    Replacement::draft(0x140b45050, "content_140b45050", fn_140b45050 as *const ()), // sub_140b45050
    Replacement::draft_throws(0x140b451f0, "content_140b451f0", fn_140b451f0 as *const (), Throws::Discard), // sub_140b451f0
    Replacement::draft(0x140b45300, "content_140b45300", fn_140b45300 as *const ()), // sub_140b45300
    Replacement::draft_throws(0x140b453f0, "content_140b453f0", fn_140b453f0 as *const (), Throws::Discard), // sub_140b453f0
    Replacement::draft(0x140b45530, "content_140b45530", fn_140b45530 as *const ()), // sub_140b45530
    Replacement::draft_throws(0x140b455c0, "content_140b455c0", fn_140b455c0 as *const (), Throws::Caught), // sub_140b455c0
    Replacement::draft_throws(0x140b45720, "content_140b45720", fn_140b45720 as *const (), Throws::Caught), // sub_140b45720
    Replacement::draft_throws(0x140b45880, "content_140b45880", fn_140b45880 as *const (), Throws::Caught), // sub_140b45880
    Replacement::draft(0x140b459e0, "content_140b459e0", fn_140b459e0 as *const ()), // sub_140b459e0
    Replacement::draft(0x140b45a80, "content_140b45a80", fn_140b45a80 as *const ()), // sub_140b45a80
    Replacement::draft(0x140b45b70, "content_140b45b70", fn_140b45b70 as *const ()), // sub_140b45b70
    Replacement::draft_throws(0x140b45c60, "content_140b45c60", fn_140b45c60 as *const (), Throws::Discard), // sub_140b45c60
    Replacement::draft_throws(0x140b45d50, "content_140b45d50", fn_140b45d50 as *const (), Throws::Discard), // sub_140b45d50
    Replacement::draft(0x140b45ed0, "content_140b45ed0", tpl_72438::<0x143426ac0, 0x1432b6fe8, 0x18, 0x1432b6fc0, 0x1432b70b0, 0x14020f270, 0x143426a48, 0x1432b7088, 0x1c, 0x1432b7058, 0x1432b7040> as *const ()), // sub_140b45ed0
    Replacement::draft(0x140b45f70, "content_140b45f70", tpl_11427::<0x58, 0x10, 0x18, 0xffffffffffffffc0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432c9318, 0x20, 0x28, 0x30> as *const ()), // sub_140b45f70
    Replacement::draft(0x140b46030, "content_140b46030", tpl_11428::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c9318, 0xfffffffffffffff4, 0x14, 0x34, 0xfffffffffffffffc, 0x4> as *const ()), // sub_140b46030
    Replacement::draft(0x140b46160, "content_140b46160", tpl_9966::<0x14387ea58, 0x14243b0b0, 0x1406e01b0, 0xb, 0x10, 0xc, 0x8, 0x338, 0x1401ee7c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b46160
    Replacement::draft_throws(0x140b46220, "content_140b46220", tpl_9629::<0x14387ee20, 0x14387ea58, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x20, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b46220
    Replacement::draft(0x140b462d0, "content_140b462d0", tpl_9969::<0x14387ea58, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b462d0
    Replacement::draft_throws(0x140b46350, "content_140b46350", tpl_9630::<0x14387ee20, 0x14387ea58, 0x1401e9b60, 0x1406e03a0, 0x328, 0x8, 0x338, 0x1424b4da0, 0x369, 0x1426cdcb0, 0x142409b50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b46350
    Replacement::draft_throws(0x140b46400, "content_140b46400", fn_140b46400 as *const (), Throws::Caught), // sub_140b46400
    Replacement::draft_throws(0x140b464b0, "content_140b464b0", fn_140b464b0 as *const (), Throws::Caught), // sub_140b464b0
    Replacement::draft_throws(0x140b46560, "content_140b46560", fn_140b46560 as *const (), Throws::Caught), // sub_140b46560
    Replacement::draft(0x140b46780, "content_140b46780", tpl_11589::<0x14387ea58, 0x14243b0b0, 0x1406e01b0, 0x1426cdcb0, 0x8, 0x142409b50, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b46780
    Replacement::draft(0x140b46810, "content_140b46810", tpl_7856::<0x14387ee20, 0x14387ea58, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x1401e9860> as *const ()), // sub_140b46810
    Replacement::draft(0x140b46880, "content_140b46880", tpl_9969::<0x14387ea58, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b46880
    Replacement::draft_throws(0x140b46900, "content_140b46900", tpl_9971::<0x14387ee20, 0x14387ea58, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // sub_140b46900
    Replacement::draft_throws(0x140b46970, "content_140b46970", tpl_11590::<0x14387ea58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1426cdcb0, 0x8, 0x1432b6cc0, 0x1430af080, 0x1432b6da8, 0x1402c4730> as *const (), Throws::Discard), // sub_140b46970
    Replacement::draft(0x140b46a70, "content_140b46a70", fn_140b46a70 as *const ()), // sub_140b46a70
    Replacement::draft(0x140b46b10, "content_140b46b10", fn_140b46b10 as *const ()), // sub_140b46b10
    Replacement::draft(0x140b46c40, "content_140b46c40", fn_140b46c40 as *const ()), // sub_140b46c40
    Replacement::draft_throws(0x140b46d50, "content_140b46d50", tpl_6799::<0x14387efe8, 0x14387eb10, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x1406da930, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b46d50
    Replacement::draft(0x140b46dc0, "content_140b46dc0", fn_140b46dc0 as *const ()), // sub_140b46dc0
    Replacement::draft_throws(0x140b46e50, "content_140b46e50", tpl_6799::<0x14387efe8, 0x14387eb10, 0x1401e9b60, 0x1406e03a0, 0x8, 0x1406daaa0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b46e50
    Replacement::draft_throws(0x140b46ec0, "content_140b46ec0", fn_140b46ec0 as *const (), Throws::Caught), // sub_140b46ec0
    Replacement::draft_throws(0x140b46f70, "content_140b46f70", fn_140b46f70 as *const (), Throws::Caught), // sub_140b46f70
    Replacement::draft_throws(0x140b47020, "content_140b47020", fn_140b47020 as *const (), Throws::Caught), // sub_140b47020
    Replacement::draft(0x140b470d0, "content_140b470d0", fn_140b470d0 as *const ()), // sub_140b470d0
    Replacement::draft(0x140b471c0, "content_140b471c0", tpl_7856::<0x14387efe8, 0x14387eb10, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x88, 0x1401e9860> as *const ()), // sub_140b471c0
    Replacement::draft(0x140b47230, "content_140b47230", tpl_9969::<0x14387eb10, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x88, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b47230
    Replacement::draft_throws(0x140b472b0, "content_140b472b0", tpl_9985::<0x14387efe8, 0x14387eb10, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b472b0
    Replacement::draft_throws(0x140b47330, "content_140b47330", fn_140b47330 as *const (), Throws::Discard), // sub_140b47330
    Replacement::draft(0x140b47420, "content_140b47420", fn_140b47420 as *const ()), // sub_140b47420
    Replacement::draft(0x140b474c0, "content_140b474c0", fn_140b474c0 as *const ()), // sub_140b474c0
    Replacement::draft(0x140b475e0, "content_140b475e0", fn_140b475e0 as *const ()), // sub_140b475e0
    Replacement::draft_throws(0x140b477b0, "content_140b477b0", fn_140b477b0 as *const (), Throws::Discard), // sub_140b477b0
    Replacement::draft(0x140b47960, "content_140b47960", fn_140b47960 as *const ()), // sub_140b47960
    Replacement::draft_throws(0x140b47ad0, "content_140b47ad0", fn_140b47ad0 as *const (), Throws::Discard), // sub_140b47ad0
    Replacement::draft(0x140b47c60, "content_140b47c60", fn_140b47c60 as *const ()), // sub_140b47c60
    Replacement::draft_throws(0x140b47d20, "content_140b47d20", fn_140b47d20 as *const (), Throws::Caught), // sub_140b47d20
    Replacement::draft_throws(0x140b48210, "content_140b48210", fn_140b48210 as *const (), Throws::Caught), // sub_140b48210
    Replacement::draft_throws(0x140b48650, "content_140b48650", fn_140b48650 as *const (), Throws::Caught), // sub_140b48650
    Replacement::draft(0x140b48a70, "content_140b48a70", fn_140b48a70 as *const ()), // sub_140b48a70
    Replacement::draft(0x140b48b20, "content_140b48b20", fn_140b48b20 as *const ()), // sub_140b48b20
    Replacement::draft(0x140b48c90, "content_140b48c90", fn_140b48c90 as *const ()), // sub_140b48c90
    Replacement::draft_throws(0x140b48e10, "content_140b48e10", fn_140b48e10 as *const (), Throws::Discard), // sub_140b48e10
    Replacement::draft_throws(0x140b48f40, "content_140b48f40", fn_140b48f40 as *const (), Throws::Discard), // sub_140b48f40
    Replacement::draft_throws(0x140b49100, "content_140b49100", fn_140b49100 as *const (), Throws::Discard), // sub_140b49100
    Replacement::draft(0x140b491b0, "content_140b491b0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431b6ef8, 0x8> as *const ()), // sub_140b491b0
    Replacement::draft(0x140b49230, "content_140b49230", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431b6ef8> as *const ()), // sub_140b49230
    Replacement::draft(0x140b492c0, "content_140b492c0", fn_140b492c0 as *const ()), // sub_140b492c0
    Replacement::draft(0x140b49390, "content_140b49390", fn_140b49390 as *const ()), // sub_140b49390
    Replacement::draft_throws(0x140b494e0, "content_140b494e0", fn_140b494e0 as *const (), Throws::Discard), // sub_140b494e0
    Replacement::draft(0x140b49650, "content_140b49650", fn_140b49650 as *const ()), // sub_140b49650
    Replacement::draft_throws(0x140b49770, "content_140b49770", fn_140b49770 as *const (), Throws::Discard), // sub_140b49770
    Replacement::draft_throws(0x140b498e0, "content_140b498e0", fn_140b498e0 as *const (), Throws::Caught), // sub_140b498e0
    Replacement::draft_throws(0x140b49a70, "content_140b49a70", fn_140b49a70 as *const (), Throws::Caught), // sub_140b49a70
    Replacement::draft_throws(0x140b49c00, "content_140b49c00", fn_140b49c00 as *const (), Throws::Caught), // sub_140b49c00
    Replacement::draft(0x140b49d90, "content_140b49d90", fn_140b49d90 as *const ()), // sub_140b49d90
    Replacement::draft(0x140b49e70, "content_140b49e70", fn_140b49e70 as *const ()), // sub_140b49e70
    Replacement::draft(0x140b49f70, "content_140b49f70", fn_140b49f70 as *const ()), // sub_140b49f70
    Replacement::draft_throws(0x140b4a080, "content_140b4a080", fn_140b4a080 as *const (), Throws::Discard), // sub_140b4a080
    Replacement::draft_throws(0x140b4a1a0, "content_140b4a1a0", fn_140b4a1a0 as *const (), Throws::Discard), // sub_140b4a1a0
    Replacement::draft(0x140b4a330, "content_140b4a330", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431b2810> as *const ()), // sub_140b4a330
    Replacement::draft(0x140b4a390, "content_140b4a390", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431b2810> as *const ()), // sub_140b4a390
    Replacement::draft_throws(0x140b4a490, "content_140b4a490", fn_140b4a490 as *const (), Throws::Caught), // sub_140b4a490
    Replacement::draft_throws(0x140b4a520, "content_140b4a520", tpl_6762::<0x14387eb58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b4a520
    Replacement::draft(0x140b4a5d0, "content_140b4a5d0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431b29c0, 0x8> as *const ()), // sub_140b4a5d0
    Replacement::draft(0x140b4a650, "content_140b4a650", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431b29c0> as *const ()), // sub_140b4a650
    Replacement::draft(0x140b4a750, "content_140b4a750", fn_140b4a750 as *const ()), // sub_140b4a750
    Replacement::draft_throws(0x140b4a830, "content_140b4a830", fn_140b4a830 as *const (), Throws::Discard), // sub_140b4a830
    Replacement::draft(0x140b4a8f0, "content_140b4a8f0", tpl_12226::<0x14387e940, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b4a8f0
    Replacement::draft_throws(0x140b4a980, "content_140b4a980", fn_140b4a980 as *const (), Throws::Discard), // sub_140b4a980
    Replacement::draft_throws(0x140b4aa60, "content_140b4aa60", fn_140b4aa60 as *const (), Throws::Caught), // sub_140b4aa60
    Replacement::draft_throws(0x140b4ab80, "content_140b4ab80", fn_140b4ab80 as *const (), Throws::Caught), // sub_140b4ab80
    Replacement::draft_throws(0x140b4aca0, "content_140b4aca0", fn_140b4aca0 as *const (), Throws::Caught), // sub_140b4aca0
    Replacement::draft(0x140b4adc0, "content_140b4adc0", fn_140b4adc0 as *const ()), // sub_140b4adc0
    Replacement::draft(0x140b4ae40, "content_140b4ae40", tpl_67902::<0x14387ef40, 0x14387e940, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x1401e9860> as *const ()), // sub_140b4ae40
    Replacement::draft(0x140b4aec0, "content_140b4aec0", tpl_12226::<0x14387e940, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b4aec0
    Replacement::draft_throws(0x140b4af50, "content_140b4af50", fn_140b4af50 as *const (), Throws::Discard), // sub_140b4af50
    Replacement::draft_throws(0x140b4afe0, "content_140b4afe0", fn_140b4afe0 as *const (), Throws::Discard), // sub_140b4afe0
    Replacement::draft(0x140b4b120, "content_140b4b120", fn_140b4b120 as *const ()), // sub_140b4b120
    Replacement::draft(0x140b4b1c0, "content_140b4b1c0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431aeee0, 0x8> as *const ()), // sub_140b4b1c0
    Replacement::draft(0x140b4b250, "content_140b4b250", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1431aeee0, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140b4b250
    Replacement::draft(0x140b4b300, "content_140b4b300", tpl_7312::<0x14387e918, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b4b300
    Replacement::draft(0x140b4b370, "content_140b4b370", fn_140b4b370 as *const ()), // sub_140b4b370
    Replacement::draft_throws(0x140b4b490, "content_140b4b490", fn_140b4b490 as *const (), Throws::Discard), // sub_140b4b490
    Replacement::draft(0x140b4b590, "content_140b4b590", fn_140b4b590 as *const ()), // sub_140b4b590
    Replacement::draft_throws(0x140b4b680, "content_140b4b680", fn_140b4b680 as *const (), Throws::Discard), // sub_140b4b680
    Replacement::draft(0x140b4b790, "content_140b4b790", fn_140b4b790 as *const ()), // sub_140b4b790
    Replacement::draft_throws(0x140b4b830, "content_140b4b830", fn_140b4b830 as *const (), Throws::Caught), // sub_140b4b830
    Replacement::draft_throws(0x140b4b900, "content_140b4b900", fn_140b4b900 as *const (), Throws::Caught), // sub_140b4b900
    Replacement::draft_throws(0x140b4b9d0, "content_140b4b9d0", fn_140b4b9d0 as *const (), Throws::Caught), // sub_140b4b9d0
    Replacement::draft(0x140b4baa0, "content_140b4baa0", fn_140b4baa0 as *const ()), // sub_140b4baa0
    Replacement::draft(0x140b4bb30, "content_140b4bb30", fn_140b4bb30 as *const ()), // sub_140b4bb30
    Replacement::draft(0x140b4bbd0, "content_140b4bbd0", fn_140b4bbd0 as *const ()), // sub_140b4bbd0
    Replacement::draft_throws(0x140b4bc70, "content_140b4bc70", fn_140b4bc70 as *const (), Throws::Discard), // sub_140b4bc70
    Replacement::draft_throws(0x140b4bd00, "content_140b4bd00", tpl_14010::<0x14387e918, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432b6858, 0x8, 0x1432b6838, 0x1432b6820, 0x140bfc3a0> as *const (), Throws::Discard), // sub_140b4bd00
    Replacement::draft(0x140b4bdf0, "content_140b4bdf0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431ad160, 0x8> as *const ()), // sub_140b4bdf0
    Replacement::draft(0x140b4be70, "content_140b4be70", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431ad160> as *const ()), // sub_140b4be70
    Replacement::draft(0x140b4bf70, "content_140b4bf70", tpl_14066::<0x14387ed20, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b4bf70
    Replacement::draft_throws(0x140b4c010, "content_140b4c010", tpl_8126::<0x14387e9d8, 0x14387ed20, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b4c010
    Replacement::draft(0x140b4c0a0, "content_140b4c0a0", tpl_9969::<0x14387ed20, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b4c0a0
    Replacement::draft_throws(0x140b4c120, "content_140b4c120", tpl_8127::<0x14387e9d8, 0x14387ed20, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b4c120
    Replacement::draft_throws(0x140b4c1b0, "content_140b4c1b0", fn_140b4c1b0 as *const (), Throws::Caught), // sub_140b4c1b0
    Replacement::draft_throws(0x140b4c290, "content_140b4c290", fn_140b4c290 as *const (), Throws::Caught), // sub_140b4c290
    Replacement::draft_throws(0x140b4c370, "content_140b4c370", fn_140b4c370 as *const (), Throws::Caught), // sub_140b4c370
    Replacement::draft(0x140b4c450, "content_140b4c450", tpl_14068::<0x14387ed20, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b4c450
    Replacement::draft(0x140b4c4c0, "content_140b4c4c0", tpl_7856::<0x14387e9d8, 0x14387ed20, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140b4c4c0
    Replacement::draft(0x140b4c530, "content_140b4c530", tpl_9969::<0x14387ed20, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b4c530
    Replacement::draft_throws(0x140b4c5b0, "content_140b4c5b0", tpl_8130::<0x14387e9d8, 0x14387ed20, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b4c5b0
    Replacement::draft_throws(0x140b4c630, "content_140b4c630", tpl_14069::<0x14387ed20, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432b67e0, 0x1430a1268, 0x1432b68f0, 0x14020f0f0> as *const (), Throws::Discard), // sub_140b4c630
    Replacement::draft(0x140b4c730, "content_140b4c730", fn_140b4c730 as *const ()), // sub_140b4c730
    Replacement::draft(0x140b4c810, "content_140b4c810", fn_140b4c810 as *const ()), // sub_140b4c810
    Replacement::draft(0x140b4c8e0, "content_140b4c8e0", fn_140b4c8e0 as *const ()), // sub_140b4c8e0
    Replacement::draft(0x140b4c9f0, "content_140b4c9f0", fn_140b4c9f0 as *const ()), // sub_140b4c9f0
    Replacement::draft_throws(0x140b4cd40, "content_140b4cd40", fn_140b4cd40 as *const (), Throws::Discard), // sub_140b4cd40
    Replacement::draft(0x140b4d080, "content_140b4d080", fn_140b4d080 as *const ()), // sub_140b4d080
    Replacement::draft_throws(0x140b4d2c0, "content_140b4d2c0", fn_140b4d2c0 as *const (), Throws::Discard), // sub_140b4d2c0
    Replacement::draft(0x140b4d600, "content_140b4d600", fn_140b4d600 as *const ()), // sub_140b4d600
    Replacement::draft_throws(0x140b4d7c0, "content_140b4d7c0", fn_140b4d7c0 as *const (), Throws::Caught), // sub_140b4d7c0
    Replacement::draft_throws(0x140b4e030, "content_140b4e030", fn_140b4e030 as *const (), Throws::Caught), // sub_140b4e030
    Replacement::draft_throws(0x140b4e7b0, "content_140b4e7b0", fn_140b4e7b0 as *const (), Throws::Caught), // sub_140b4e7b0
    Replacement::draft(0x140b4eef0, "content_140b4eef0", fn_140b4eef0 as *const ()), // sub_140b4eef0
    Replacement::draft(0x140b4f130, "content_140b4f130", fn_140b4f130 as *const ()), // sub_140b4f130
    Replacement::draft(0x140b4f3c0, "content_140b4f3c0", fn_140b4f3c0 as *const ()), // sub_140b4f3c0
    Replacement::draft_throws(0x140b4f620, "content_140b4f620", fn_140b4f620 as *const (), Throws::Discard), // sub_140b4f620
    Replacement::draft_throws(0x140b4f840, "content_140b4f840", fn_140b4f840 as *const (), Throws::Discard), // sub_140b4f840
    Replacement::draft_throws(0x140b4fbc0, "content_140b4fbc0", fn_140b4fbc0 as *const (), Throws::Discard), // sub_140b4fbc0
    Replacement::draft(0x140b4fda0, "content_140b4fda0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431ab0e0, 0x8> as *const ()), // sub_140b4fda0
    Replacement::draft(0x140b4fe30, "content_140b4fe30", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1431ab0e0, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140b4fe30
    Replacement::draft(0x140b4fee0, "content_140b4fee0", fn_140b4fee0 as *const ()), // sub_140b4fee0
    Replacement::draft(0x140b4ff80, "content_140b4ff80", fn_140b4ff80 as *const ()), // sub_140b4ff80
    Replacement::draft_throws(0x140b50080, "content_140b50080", tpl_8772::<0x14387e9c8, 0x14387ea88, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140bfc450, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b50080
    Replacement::draft(0x140b500f0, "content_140b500f0", fn_140b500f0 as *const ()), // sub_140b500f0
    Replacement::draft_throws(0x140b501c0, "content_140b501c0", tpl_8772::<0x14387e9c8, 0x14387ea88, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140bfc600, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b501c0
    Replacement::draft(0x140b50230, "content_140b50230", fn_140b50230 as *const ()), // sub_140b50230
    Replacement::draft_throws(0x140b502b0, "content_140b502b0", fn_140b502b0 as *const (), Throws::Caught), // sub_140b502b0
    Replacement::draft_throws(0x140b50380, "content_140b50380", fn_140b50380 as *const (), Throws::Caught), // sub_140b50380
    Replacement::draft_throws(0x140b50450, "content_140b50450", fn_140b50450 as *const (), Throws::Caught), // sub_140b50450
    Replacement::draft(0x140b50520, "content_140b50520", fn_140b50520 as *const ()), // sub_140b50520
    Replacement::draft(0x140b505d0, "content_140b505d0", fn_140b505d0 as *const ()), // sub_140b505d0
    Replacement::draft(0x140b50680, "content_140b50680", fn_140b50680 as *const ()), // sub_140b50680
    Replacement::draft_throws(0x140b50750, "content_140b50750", fn_140b50750 as *const (), Throws::Discard), // sub_140b50750
    Replacement::draft_throws(0x140b50800, "content_140b50800", tpl_14088::<0x14387ea88, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1426cdcb0, 0x8, 0x1432b62b8, 0x1432b62a0, 0x140bfc7f0> as *const (), Throws::Discard), // sub_140b50800
    Replacement::draft(0x140b508f0, "content_140b508f0", tpl_14089::<0x10, 0x1426cdcb0, 0x8, 0x140a1d4a0> as *const ()), // sub_140b508f0
    Replacement::draft(0x140b50970, "content_140b50970", tpl_14089::<0x10, 0x1432b62a0, 0x8, 0x140c13c10> as *const ()), // sub_140b50970
    Replacement::draft(0x140b509f0, "content_140b509f0", fn_140b509f0 as *const ()), // sub_140b509f0
    Replacement::draft(0x140b50a90, "content_140b50a90", tpl_9021::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x1430eb9e8, 0xffffffffffffffe8, 0x28, 0x38, 0x58> as *const ()), // sub_140b50a90
    Replacement::draft(0x140b50b40, "content_140b50b40", fn_140b50b40 as *const ()), // sub_140b50b40
    Replacement::draft(0x140b50d50, "content_140b50d50", fn_140b50d50 as *const ()), // sub_140b50d50
    Replacement::draft_throws(0x140b50f50, "content_140b50f50", fn_140b50f50 as *const (), Throws::Caught), // sub_140b50f50
    Replacement::draft(0x140b511a0, "content_140b511a0", fn_140b511a0 as *const ()), // sub_140b511a0
    Replacement::draft_throws(0x140b51360, "content_140b51360", fn_140b51360 as *const (), Throws::Discard), // sub_140b51360
    Replacement::draft(0x140b51550, "content_140b51550", fn_140b51550 as *const ()), // sub_140b51550
    Replacement::draft_throws(0x140b51640, "content_140b51640", fn_140b51640 as *const (), Throws::Caught), // sub_140b51640
    Replacement::draft_throws(0x140b519c0, "content_140b519c0", fn_140b519c0 as *const (), Throws::Caught), // sub_140b519c0
    Replacement::draft_throws(0x140b51d10, "content_140b51d10", fn_140b51d10 as *const (), Throws::Caught), // sub_140b51d10
    Replacement::draft(0x140b52060, "content_140b52060", fn_140b52060 as *const ()), // sub_140b52060
    Replacement::draft(0x140b52280, "content_140b52280", fn_140b52280 as *const ()), // sub_140b52280
    Replacement::draft(0x140b52420, "content_140b52420", fn_140b52420 as *const ()), // sub_140b52420
    Replacement::draft_throws(0x140b525d0, "content_140b525d0", fn_140b525d0 as *const (), Throws::Discard), // sub_140b525d0
    Replacement::draft_throws(0x140b52780, "content_140b52780", fn_140b52780 as *const (), Throws::Caught), // sub_140b52780
    Replacement::draft(0x140b52a70, "content_140b52a70", fn_140b52a70 as *const ()), // sub_140b52a70
    Replacement::draft(0x140b52b50, "content_140b52b50", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1431a1b00, 0x8> as *const ()), // sub_140b52b50
    Replacement::draft(0x140b52bd0, "content_140b52bd0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431a1b00> as *const ()), // sub_140b52bd0
    Replacement::draft(0x140b52cd0, "content_140b52cd0", tpl_12130::<0x14387f0a0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b52cd0
    Replacement::draft_throws(0x140b52d80, "content_140b52d80", fn_140b52d80 as *const (), Throws::Discard), // sub_140b52d80
    Replacement::draft(0x140b52e10, "content_140b52e10", tpl_9969::<0x14387f0a0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b52e10
    Replacement::draft_throws(0x140b52e90, "content_140b52e90", fn_140b52e90 as *const (), Throws::Discard), // sub_140b52e90
    Replacement::draft_throws(0x140b52f30, "content_140b52f30", fn_140b52f30 as *const (), Throws::Caught), // sub_140b52f30
    Replacement::draft_throws(0x140b53030, "content_140b53030", fn_140b53030 as *const (), Throws::Caught), // sub_140b53030
    Replacement::draft_throws(0x140b53130, "content_140b53130", fn_140b53130 as *const (), Throws::Caught), // sub_140b53130
    Replacement::draft(0x140b53230, "content_140b53230", fn_140b53230 as *const ()), // sub_140b53230
    Replacement::draft(0x140b532a0, "content_140b532a0", tpl_7856::<0x14387ecd4, 0x14387f0a0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0x1401e9860> as *const ()), // sub_140b532a0
    Replacement::draft(0x140b53310, "content_140b53310", tpl_9969::<0x14387f0a0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b53310
    Replacement::draft_throws(0x140b53390, "content_140b53390", tpl_7858::<0x14387ecd4, 0x14387f0a0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140b53390
    Replacement::draft_throws(0x140b53410, "content_140b53410", fn_140b53410 as *const (), Throws::Discard), // sub_140b53410
    Replacement::draft(0x140b53530, "content_140b53530", fn_140b53530 as *const ()), // sub_140b53530
    Replacement::draft(0x140b535a0, "content_140b535a0", fn_140b535a0 as *const ()), // sub_140b535a0
    Replacement::draft(0x140b53660, "content_140b53660", fn_140b53660 as *const ()), // sub_140b53660
    Replacement::draft(0x140b53730, "content_140b53730", fn_140b53730 as *const ()), // sub_140b53730
    Replacement::draft(0x140b53840, "content_140b53840", fn_140b53840 as *const ()), // sub_140b53840
    Replacement::draft_throws(0x140b539e0, "content_140b539e0", fn_140b539e0 as *const (), Throws::Discard), // sub_140b539e0
    Replacement::draft(0x140b53b10, "content_140b53b10", fn_140b53b10 as *const ()), // sub_140b53b10
    Replacement::draft_throws(0x140b53c50, "content_140b53c50", fn_140b53c50 as *const (), Throws::Discard), // sub_140b53c50
    Replacement::draft(0x140b53d80, "content_140b53d80", fn_140b53d80 as *const ()), // sub_140b53d80
    Replacement::draft_throws(0x140b53ed0, "content_140b53ed0", fn_140b53ed0 as *const (), Throws::Caught), // sub_140b53ed0
    Replacement::draft_throws(0x140b53ff0, "content_140b53ff0", fn_140b53ff0 as *const (), Throws::Caught), // sub_140b53ff0
    Replacement::draft_throws(0x140b54110, "content_140b54110", fn_140b54110 as *const (), Throws::Caught), // sub_140b54110
    Replacement::draft(0x140b54230, "content_140b54230", fn_140b54230 as *const ()), // sub_140b54230
    Replacement::draft(0x140b543a0, "content_140b543a0", fn_140b543a0 as *const ()), // sub_140b543a0
    Replacement::draft(0x140b54560, "content_140b54560", fn_140b54560 as *const ()), // sub_140b54560
    Replacement::draft_throws(0x140b54740, "content_140b54740", fn_140b54740 as *const (), Throws::Discard), // sub_140b54740
    Replacement::draft_throws(0x140b54850, "content_140b54850", fn_140b54850 as *const (), Throws::Caught), // sub_140b54850
    Replacement::draft(0x140b54a60, "content_140b54a60", fn_140b54a60 as *const ()), // sub_140b54a60
    Replacement::draft(0x140b54b30, "content_140b54b30", fn_140b54b30 as *const ()), // sub_140b54b30
    Replacement::draft(0x140b54c00, "content_140b54c00", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143199578> as *const ()), // sub_140b54c00
    Replacement::draft(0x140b54c60, "content_140b54c60", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143199578> as *const ()), // sub_140b54c60
    Replacement::draft_throws(0x140b54d60, "content_140b54d60", fn_140b54d60 as *const (), Throws::Caught), // sub_140b54d60
    Replacement::draft_throws(0x140b54df0, "content_140b54df0", tpl_6763::<0x14387eea8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b54df0
    Replacement::draft(0x140b54ea0, "content_140b54ea0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432cd048> as *const ()), // sub_140b54ea0
    Replacement::draft(0x140b54f00, "content_140b54f00", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cd048> as *const ()), // sub_140b54f00
    Replacement::draft_throws(0x140b55000, "content_140b55000", fn_140b55000 as *const (), Throws::Caught), // sub_140b55000
    Replacement::draft_throws(0x140b55090, "content_140b55090", tpl_6762::<0x14387ed98, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b55090
    Replacement::draft(0x140b55140, "content_140b55140", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143088f50, 0x8> as *const ()), // sub_140b55140
    Replacement::draft(0x140b551c0, "content_140b551c0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143088f50> as *const ()), // sub_140b551c0
    Replacement::draft(0x140b55250, "content_140b55250", fn_140b55250 as *const ()), // sub_140b55250
    Replacement::draft(0x140b552d0, "content_140b552d0", fn_140b552d0 as *const ()), // sub_140b552d0
    Replacement::draft_throws(0x140b553a0, "content_140b553a0", fn_140b553a0 as *const (), Throws::Caught), // sub_140b553a0
    Replacement::draft(0x140b55480, "content_140b55480", fn_140b55480 as *const ()), // sub_140b55480
    Replacement::draft_throws(0x140b55550, "content_140b55550", fn_140b55550 as *const (), Throws::Discard), // sub_140b55550
    Replacement::draft_throws(0x140b555e0, "content_140b555e0", fn_140b555e0 as *const (), Throws::Caught), // sub_140b555e0
    Replacement::draft_throws(0x140b557d0, "content_140b557d0", fn_140b557d0 as *const (), Throws::Caught), // sub_140b557d0
    Replacement::draft_throws(0x140b55990, "content_140b55990", fn_140b55990 as *const (), Throws::Caught), // sub_140b55990
    Replacement::draft(0x140b55b50, "content_140b55b50", fn_140b55b50 as *const ()), // sub_140b55b50
    Replacement::draft(0x140b55c90, "content_140b55c90", fn_140b55c90 as *const ()), // sub_140b55c90
    Replacement::draft(0x140b55d40, "content_140b55d40", fn_140b55d40 as *const ()), // sub_140b55d40
    Replacement::draft_throws(0x140b55df0, "content_140b55df0", fn_140b55df0 as *const (), Throws::Discard), // sub_140b55df0
    Replacement::draft_throws(0x140b55ea0, "content_140b55ea0", fn_140b55ea0 as *const (), Throws::Discard), // sub_140b55ea0
    Replacement::draft(0x140b55f90, "content_140b55f90", fn_140b55f90 as *const ()), // sub_140b55f90
    Replacement::draft(0x140b56060, "content_140b56060", fn_140b56060 as *const ()), // sub_140b56060
    Replacement::draft(0x140b56130, "content_140b56130", fn_140b56130 as *const ()), // sub_140b56130
    Replacement::draft(0x140b561e0, "content_140b561e0", fn_140b561e0 as *const ()), // sub_140b561e0
    Replacement::draft_throws(0x140b56570, "content_140b56570", fn_140b56570 as *const (), Throws::Discard), // sub_140b56570
    Replacement::draft(0x140b56900, "content_140b56900", fn_140b56900 as *const ()), // sub_140b56900
    Replacement::draft_throws(0x140b56af0, "content_140b56af0", fn_140b56af0 as *const (), Throws::Discard), // sub_140b56af0
    Replacement::draft(0x140b56ed0, "content_140b56ed0", fn_140b56ed0 as *const ()), // sub_140b56ed0
    Replacement::draft_throws(0x140b56fe0, "content_140b56fe0", fn_140b56fe0 as *const (), Throws::Caught), // sub_140b56fe0
    Replacement::draft_throws(0x140b578e0, "content_140b578e0", fn_140b578e0 as *const (), Throws::Caught), // sub_140b578e0
    Replacement::draft_throws(0x140b580f0, "content_140b580f0", fn_140b580f0 as *const (), Throws::Caught), // sub_140b580f0
    Replacement::draft(0x140b588f0, "content_140b588f0", fn_140b588f0 as *const ()), // sub_140b588f0
    Replacement::draft(0x140b58a60, "content_140b58a60", fn_140b58a60 as *const ()), // sub_140b58a60
    Replacement::draft(0x140b58cd0, "content_140b58cd0", fn_140b58cd0 as *const ()), // sub_140b58cd0
    Replacement::draft_throws(0x140b58f50, "content_140b58f50", fn_140b58f50 as *const (), Throws::Discard), // sub_140b58f50
    Replacement::draft_throws(0x140b59190, "content_140b59190", fn_140b59190 as *const (), Throws::Caught), // sub_140b59190
    Replacement::draft_throws(0x140b59680, "content_140b59680", fn_140b59680 as *const (), Throws::Discard), // sub_140b59680
    Replacement::draft(0x140b598d0, "content_140b598d0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431a3b50, 0x8> as *const ()), // sub_140b598d0
    Replacement::draft(0x140b59960, "content_140b59960", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1431a3b50, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140b59960
    Replacement::draft(0x140b59a10, "content_140b59a10", fn_140b59a10 as *const ()), // sub_140b59a10
    Replacement::draft(0x140b59ab0, "content_140b59ab0", fn_140b59ab0 as *const ()), // sub_140b59ab0
    Replacement::draft_throws(0x140b59ba0, "content_140b59ba0", fn_140b59ba0 as *const (), Throws::Discard), // sub_140b59ba0
    Replacement::draft(0x140b59c50, "content_140b59c50", fn_140b59c50 as *const ()), // sub_140b59c50
    Replacement::draft_throws(0x140b59d20, "content_140b59d20", fn_140b59d20 as *const (), Throws::Discard), // sub_140b59d20
    Replacement::draft(0x140b59de0, "content_140b59de0", fn_140b59de0 as *const ()), // sub_140b59de0
    Replacement::draft_throws(0x140b59e60, "content_140b59e60", fn_140b59e60 as *const (), Throws::Caught), // sub_140b59e60
    Replacement::draft_throws(0x140b59f20, "content_140b59f20", fn_140b59f20 as *const (), Throws::Caught), // sub_140b59f20
    Replacement::draft_throws(0x140b59fe0, "content_140b59fe0", fn_140b59fe0 as *const (), Throws::Caught), // sub_140b59fe0
    Replacement::draft(0x140b5a0a0, "content_140b5a0a0", fn_140b5a0a0 as *const ()), // sub_140b5a0a0
    Replacement::draft(0x140b5a150, "content_140b5a150", fn_140b5a150 as *const ()), // sub_140b5a150
    Replacement::draft(0x140b5a200, "content_140b5a200", fn_140b5a200 as *const ()), // sub_140b5a200
    Replacement::draft_throws(0x140b5a2d0, "content_140b5a2d0", fn_140b5a2d0 as *const (), Throws::Discard), // sub_140b5a2d0
    Replacement::draft_throws(0x140b5a380, "content_140b5a380", tpl_6745::<0x14387f0d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432b75d8, 0x8, 0x1432b75c8, 0x140bfd640> as *const (), Throws::Discard), // sub_140b5a380
    Replacement::draft(0x140b5a460, "content_140b5a460", tpl_14161::<0x10, 0x1426cdcb0, 0x8, 0x140c17250> as *const ()), // sub_140b5a460
    Replacement::draft(0x140b5a4e0, "content_140b5a4e0", tpl_14161::<0x10, 0x1432b75c8, 0x8, 0x140c17260> as *const ()), // sub_140b5a4e0
    Replacement::draft(0x140b5a560, "content_140b5a560", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143094930, 0x8> as *const ()), // sub_140b5a560
    Replacement::draft(0x140b5a5e0, "content_140b5a5e0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143094930> as *const ()), // sub_140b5a5e0
    Replacement::draft(0x140b5a6e0, "content_140b5a6e0", tpl_12161::<0x14387ec98, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5a6e0
    Replacement::draft_throws(0x140b5a790, "content_140b5a790", tpl_7851::<0x14387ec70, 0x14387ec98, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b5a790
    Replacement::draft(0x140b5a820, "content_140b5a820", tpl_9969::<0x14387ec98, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5a820
    Replacement::draft_throws(0x140b5a8a0, "content_140b5a8a0", tpl_7972::<0x14387ec70, 0x14387ec98, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b5a8a0
    Replacement::draft_throws(0x140b5a930, "content_140b5a930", fn_140b5a930 as *const (), Throws::Caught), // sub_140b5a930
    Replacement::draft_throws(0x140b5ab20, "content_140b5ab20", fn_140b5ab20 as *const (), Throws::Caught), // sub_140b5ab20
    Replacement::draft_throws(0x140b5ace0, "content_140b5ace0", fn_140b5ace0 as *const (), Throws::Caught), // sub_140b5ace0
    Replacement::draft(0x140b5aea0, "content_140b5aea0", tpl_12163::<0x14387ec98, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5aea0
    Replacement::draft(0x140b5af10, "content_140b5af10", tpl_8074::<0x14387ec70, 0x14387ec98, 0x1401e9970, 0x1406e1480, 0x8, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_140b5af10
    Replacement::draft(0x140b5af90, "content_140b5af90", tpl_13937::<0x14387ec98, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5af90
    Replacement::draft_throws(0x140b5b020, "content_140b5b020", tpl_7974::<0x14387ec70, 0x14387ec98, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b5b020
    Replacement::draft_throws(0x140b5b0a0, "content_140b5b0a0", tpl_13938::<0x14387ec98, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x1432b8b50> as *const (), Throws::Discard), // sub_140b5b0a0
    Replacement::draft(0x140b5b180, "content_140b5b180", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430a1868> as *const ()), // sub_140b5b180
    Replacement::draft(0x140b5b1e0, "content_140b5b1e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430a1868> as *const ()), // sub_140b5b1e0
    Replacement::draft(0x140b5b270, "content_140b5b270", tpl_9031::<0x14387eda0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5b270
    Replacement::draft_throws(0x140b5b2e0, "content_140b5b2e0", fn_140b5b2e0 as *const (), Throws::Caught), // sub_140b5b2e0
    Replacement::draft_throws(0x140b5b370, "content_140b5b370", tpl_6763::<0x14387eda0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b5b370
    Replacement::draft(0x140b5b420, "content_140b5b420", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430a7230, 0x8> as *const ()), // sub_140b5b420
    Replacement::draft(0x140b5b4a0, "content_140b5b4a0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430a7230> as *const ()), // sub_140b5b4a0
    Replacement::draft(0x140b5b530, "content_140b5b530", tpl_7925::<0x14387eb20, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140b5b530
    Replacement::draft(0x140b5b5d0, "content_140b5b5d0", fn_140b5b5d0 as *const ()), // sub_140b5b5d0
    Replacement::draft_throws(0x140b5b6b0, "content_140b5b6b0", fn_140b5b6b0 as *const (), Throws::Discard), // sub_140b5b6b0
    Replacement::draft(0x140b5b770, "content_140b5b770", fn_140b5b770 as *const ()), // sub_140b5b770
    Replacement::draft_throws(0x140b5b820, "content_140b5b820", fn_140b5b820 as *const (), Throws::Discard), // sub_140b5b820
    Replacement::draft_throws(0x140b5b8f0, "content_140b5b8f0", fn_140b5b8f0 as *const (), Throws::Caught), // sub_140b5b8f0
    Replacement::draft_throws(0x140b5bae0, "content_140b5bae0", fn_140b5bae0 as *const (), Throws::Caught), // sub_140b5bae0
    Replacement::draft_throws(0x140b5bca0, "content_140b5bca0", fn_140b5bca0 as *const (), Throws::Caught), // sub_140b5bca0
    Replacement::draft(0x140b5be60, "content_140b5be60", fn_140b5be60 as *const ()), // sub_140b5be60
    Replacement::draft(0x140b5bf10, "content_140b5bf10", fn_140b5bf10 as *const ()), // sub_140b5bf10
    Replacement::draft(0x140b5bfc0, "content_140b5bfc0", fn_140b5bfc0 as *const ()), // sub_140b5bfc0
    Replacement::draft_throws(0x140b5c080, "content_140b5c080", fn_140b5c080 as *const (), Throws::Discard), // sub_140b5c080
    Replacement::draft_throws(0x140b5c130, "content_140b5c130", fn_140b5c130 as *const (), Throws::Discard), // sub_140b5c130
    Replacement::draft(0x140b5c210, "content_140b5c210", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143175b40, 0x8> as *const ()), // sub_140b5c210
    Replacement::draft(0x140b5c290, "content_140b5c290", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143175b40> as *const ()), // sub_140b5c290
    Replacement::draft(0x140b5c390, "content_140b5c390", tpl_14066::<0x14387eda8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5c390
    Replacement::draft_throws(0x140b5c430, "content_140b5c430", tpl_8126::<0x14387eea0, 0x14387eda8, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x80, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b5c430
    Replacement::draft(0x140b5c4c0, "content_140b5c4c0", tpl_9969::<0x14387eda8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5c4c0
    Replacement::draft_throws(0x140b5c540, "content_140b5c540", tpl_8127::<0x14387eea0, 0x14387eda8, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b5c540
    Replacement::draft_throws(0x140b5c5d0, "content_140b5c5d0", fn_140b5c5d0 as *const (), Throws::Caught), // sub_140b5c5d0
    Replacement::draft_throws(0x140b5c6b0, "content_140b5c6b0", fn_140b5c6b0 as *const (), Throws::Caught), // sub_140b5c6b0
    Replacement::draft_throws(0x140b5c790, "content_140b5c790", fn_140b5c790 as *const (), Throws::Caught), // sub_140b5c790
    Replacement::draft(0x140b5c870, "content_140b5c870", tpl_14068::<0x14387eda8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5c870
    Replacement::draft(0x140b5c8e0, "content_140b5c8e0", tpl_7856::<0x14387eea0, 0x14387eda8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140b5c8e0
    Replacement::draft(0x140b5c950, "content_140b5c950", tpl_9969::<0x14387eda8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5c950
    Replacement::draft_throws(0x140b5c9d0, "content_140b5c9d0", tpl_8130::<0x14387eea0, 0x14387eda8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b5c9d0
    Replacement::draft_throws(0x140b5ca50, "content_140b5ca50", tpl_14069::<0x14387eda8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432b8900, 0x1430a1268, 0x1432b88d8, 0x14020f0f0> as *const (), Throws::Discard), // sub_140b5ca50
    Replacement::draft(0x140b5cb50, "content_140b5cb50", tpl_7618::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x1432d0b28, 0x20> as *const ()), // sub_140b5cb50
    Replacement::draft(0x140b5cbe0, "content_140b5cbe0", tpl_12018::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432d0b28, 0x18, 0x20, 0x28> as *const ()), // sub_140b5cbe0
    Replacement::draft(0x140b5cc90, "content_140b5cc90", tpl_14178::<0x14387f008, 0x14243b0b0, 0x1406e01b0, 0x18, 0x140c41840, 0x20, 0x1409ff110, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b5cc90
    Replacement::draft(0x140b5cd20, "content_140b5cd20", fn_140b5cd20 as *const ()), // sub_140b5cd20
    Replacement::draft_throws(0x140b5ce80, "content_140b5ce80", fn_140b5ce80 as *const (), Throws::Discard), // sub_140b5ce80
    Replacement::draft(0x140b5cfb0, "content_140b5cfb0", tpl_14181::<0x14387f008, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x14, 0x18, 0x140c41b60, 0x20, 0x1409ff110> as *const ()), // sub_140b5cfb0
    Replacement::draft_throws(0x140b5d080, "content_140b5d080", fn_140b5d080 as *const (), Throws::Discard), // sub_140b5d080
    Replacement::draft_throws(0x140b5d1e0, "content_140b5d1e0", fn_140b5d1e0 as *const (), Throws::Caught), // sub_140b5d1e0
    Replacement::draft_throws(0x140b5d470, "content_140b5d470", fn_140b5d470 as *const (), Throws::Caught), // sub_140b5d470
    Replacement::draft_throws(0x140b5d700, "content_140b5d700", fn_140b5d700 as *const (), Throws::Caught), // sub_140b5d700
    Replacement::draft(0x140b5d990, "content_140b5d990", fn_140b5d990 as *const ()), // sub_140b5d990
    Replacement::draft(0x140b5da40, "content_140b5da40", fn_140b5da40 as *const ()), // sub_140b5da40
    Replacement::draft(0x140b5db10, "content_140b5db10", tpl_14181::<0x14387f008, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x14, 0x18, 0x140c41dc0, 0x20, 0x1409ff110> as *const ()), // sub_140b5db10
    Replacement::draft_throws(0x140b5dbf0, "content_140b5dbf0", fn_140b5dbf0 as *const (), Throws::Discard), // sub_140b5dbf0
    Replacement::draft_throws(0x140b5dcf0, "content_140b5dcf0", fn_140b5dcf0 as *const (), Throws::Caught), // sub_140b5dcf0
    Replacement::draft(0x140b5dfc0, "content_140b5dfc0", fn_140b5dfc0 as *const ()), // sub_140b5dfc0
    Replacement::draft(0x140b5e140, "content_140b5e140", tpl_14189::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x14311eb70> as *const ()), // sub_140b5e140
    Replacement::draft(0x140b5e1c0, "content_140b5e1c0", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14311eb70, 0x18, 0x20> as *const ()), // sub_140b5e1c0
    Replacement::draft(0x140b5e260, "content_140b5e260", fn_140b5e260 as *const ()), // sub_140b5e260
    Replacement::draft(0x140b5e370, "content_140b5e370", fn_140b5e370 as *const ()), // sub_140b5e370
    Replacement::draft_throws(0x140b5e600, "content_140b5e600", fn_140b5e600 as *const (), Throws::Caught), // sub_140b5e600
    Replacement::draft(0x140b5e8a0, "content_140b5e8a0", fn_140b5e8a0 as *const ()), // sub_140b5e8a0
    Replacement::draft_throws(0x140b5ea90, "content_140b5ea90", fn_140b5ea90 as *const (), Throws::Discard), // sub_140b5ea90
    Replacement::draft_throws(0x140b5ed20, "content_140b5ed20", fn_140b5ed20 as *const (), Throws::Caught), // sub_140b5ed20
    Replacement::draft_throws(0x140b5f170, "content_140b5f170", fn_140b5f170 as *const (), Throws::Caught), // sub_140b5f170
    Replacement::draft_throws(0x140b5f570, "content_140b5f570", fn_140b5f570 as *const (), Throws::Caught), // sub_140b5f570
    Replacement::draft(0x140b5f970, "content_140b5f970", fn_140b5f970 as *const ()), // sub_140b5f970
    Replacement::draft(0x140b5fb70, "content_140b5fb70", fn_140b5fb70 as *const ()), // sub_140b5fb70
    Replacement::draft(0x140b5fd30, "content_140b5fd30", fn_140b5fd30 as *const ()), // sub_140b5fd30
    Replacement::draft_throws(0x140b5ff00, "content_140b5ff00", fn_140b5ff00 as *const (), Throws::Discard), // sub_140b5ff00
    Replacement::draft_throws(0x140b60100, "content_140b60100", fn_140b60100 as *const (), Throws::Discard), // sub_140b60100
    Replacement::draft(0x140b60470, "content_140b60470", fn_140b60470 as *const ()), // sub_140b60470
    Replacement::draft(0x140b605e0, "content_140b605e0", fn_140b605e0 as *const ()), // sub_140b605e0
    Replacement::draft(0x140b606c0, "content_140b606c0", fn_140b606c0 as *const ()), // sub_140b606c0
    Replacement::draft(0x140b607e0, "content_140b607e0", tpl_14178::<0x14387ed78, 0x14243b0b0, 0x1406e01b0, 0x50, 0x140c41f40, 0x78, 0x140bfe350, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b607e0
    Replacement::draft(0x140b60870, "content_140b60870", fn_140b60870 as *const ()), // sub_140b60870
    Replacement::draft_throws(0x140b60950, "content_140b60950", fn_140b60950 as *const (), Throws::Discard), // sub_140b60950
    Replacement::draft(0x140b60a20, "content_140b60a20", tpl_14207::<0x14387ed78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x50, 0x140c425c0, 0x78, 0x140bfe350, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140b60a20
    Replacement::draft_throws(0x140b60ac0, "content_140b60ac0", fn_140b60ac0 as *const (), Throws::Discard), // sub_140b60ac0
    Replacement::draft(0x140b60b90, "content_140b60b90", fn_140b60b90 as *const ()), // sub_140b60b90
    Replacement::draft_throws(0x140b60c50, "content_140b60c50", fn_140b60c50 as *const (), Throws::Caught), // sub_140b60c50
    Replacement::draft_throws(0x140b60d50, "content_140b60d50", fn_140b60d50 as *const (), Throws::Caught), // sub_140b60d50
    Replacement::draft_throws(0x140b60e50, "content_140b60e50", fn_140b60e50 as *const (), Throws::Caught), // sub_140b60e50
    Replacement::draft(0x140b610c0, "content_140b610c0", fn_140b610c0 as *const ()), // sub_140b610c0
    Replacement::draft(0x140b61170, "content_140b61170", fn_140b61170 as *const ()), // sub_140b61170
    Replacement::draft(0x140b61200, "content_140b61200", tpl_14207::<0x14387ed78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x50, 0x140c42b60, 0x78, 0x140bfe350, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140b61200
    Replacement::draft_throws(0x140b612a0, "content_140b612a0", fn_140b612a0 as *const (), Throws::Discard), // sub_140b612a0
    Replacement::draft_throws(0x140b61330, "content_140b61330", fn_140b61330 as *const (), Throws::Caught), // sub_140b61330
    Replacement::draft(0x140b614c0, "content_140b614c0", tpl_14216::<0x50, 0x1432b85c0, 0x1432b8a50, 0x140bff4d0, 0x1432b8590, 0x78, 0x1432b8638> as *const ()), // sub_140b614c0
    Replacement::draft(0x140b61540, "content_140b61540", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143174108, 0x8> as *const ()), // sub_140b61540
    Replacement::draft(0x140b615d0, "content_140b615d0", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143174108, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140b615d0
    Replacement::draft(0x140b61680, "content_140b61680", fn_140b61680 as *const ()), // sub_140b61680
    Replacement::draft(0x140b618d0, "content_140b618d0", fn_140b618d0 as *const ()), // sub_140b618d0
    Replacement::draft_throws(0x140b61b30, "content_140b61b30", tpl_14219::<0x14387efc0, 0x14387ef38, 0x1401e9fa0, 0x1406e02b0, 0x1406d7a10, 0x1403c7ab0, 0x140bff510, 0x140bff630, 0x1403c79b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b61b30
    Replacement::draft(0x140b61d10, "content_140b61d10", fn_140b61d10 as *const ()), // sub_140b61d10
    Replacement::draft_throws(0x140b61f50, "content_140b61f50", tpl_14219::<0x14387efc0, 0x14387ef38, 0x1401e9b60, 0x1406e03a0, 0x1406d7a10, 0x1403c7ab0, 0x140bff510, 0x140bff7e0, 0x1403c79b0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b61f50
    Replacement::draft(0x140b62130, "content_140b62130", fn_140b62130 as *const ()), // sub_140b62130
    Replacement::draft_throws(0x140b621d0, "content_140b621d0", fn_140b621d0 as *const (), Throws::Caught), // sub_140b621d0
    Replacement::draft_throws(0x140b622a0, "content_140b622a0", fn_140b622a0 as *const (), Throws::Caught), // sub_140b622a0
    Replacement::draft_throws(0x140b62370, "content_140b62370", fn_140b62370 as *const (), Throws::Caught), // sub_140b62370
    Replacement::draft(0x140b62440, "content_140b62440", fn_140b62440 as *const ()), // sub_140b62440
    Replacement::draft(0x140b62670, "content_140b62670", fn_140b62670 as *const ()), // sub_140b62670
    Replacement::draft(0x140b628a0, "content_140b628a0", fn_140b628a0 as *const ()), // sub_140b628a0
    Replacement::draft_throws(0x140b62ae0, "content_140b62ae0", fn_140b62ae0 as *const (), Throws::Discard), // sub_140b62ae0
    Replacement::draft_throws(0x140b62d00, "content_140b62d00", fn_140b62d00 as *const (), Throws::Discard), // sub_140b62d00
    Replacement::draft(0x140b62df0, "content_140b62df0", fn_140b62df0 as *const ()), // sub_140b62df0
    Replacement::draft(0x140b62e70, "content_140b62e70", fn_140b62e70 as *const ()), // sub_140b62e70
    Replacement::draft(0x140b62f50, "content_140b62f50", fn_140b62f50 as *const ()), // sub_140b62f50
    Replacement::draft(0x140b63060, "content_140b63060", fn_140b63060 as *const ()), // sub_140b63060
    Replacement::draft(0x140b63160, "content_140b63160", fn_140b63160 as *const ()), // sub_140b63160
    Replacement::draft_throws(0x140b63340, "content_140b63340", fn_140b63340 as *const (), Throws::Discard), // sub_140b63340
    Replacement::draft(0x140b63590, "content_140b63590", fn_140b63590 as *const ()), // sub_140b63590
    Replacement::draft_throws(0x140b636e0, "content_140b636e0", fn_140b636e0 as *const (), Throws::Discard), // sub_140b636e0
    Replacement::draft(0x140b63940, "content_140b63940", fn_140b63940 as *const ()), // sub_140b63940
    Replacement::draft_throws(0x140b639c0, "content_140b639c0", fn_140b639c0 as *const (), Throws::Caught), // sub_140b639c0
    Replacement::draft_throws(0x140b63b90, "content_140b63b90", fn_140b63b90 as *const (), Throws::Caught), // sub_140b63b90
    Replacement::draft_throws(0x140b63d60, "content_140b63d60", fn_140b63d60 as *const (), Throws::Caught), // sub_140b63d60
    Replacement::draft(0x140b64090, "content_140b64090", fn_140b64090 as *const ()), // sub_140b64090
    Replacement::draft(0x140b64230, "content_140b64230", fn_140b64230 as *const ()), // sub_140b64230
    Replacement::draft(0x140b64410, "content_140b64410", fn_140b64410 as *const ()), // sub_140b64410
    Replacement::draft_throws(0x140b64600, "content_140b64600", fn_140b64600 as *const (), Throws::Discard), // sub_140b64600
    Replacement::draft_throws(0x140b64760, "content_140b64760", fn_140b64760 as *const (), Throws::Caught), // sub_140b64760
    Replacement::draft(0x140b64a40, "content_140b64a40", fn_140b64a40 as *const ()), // sub_140b64a40
    Replacement::draft(0x140b64b20, "content_140b64b20", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14316dec8, 0x8> as *const ()), // sub_140b64b20
    Replacement::draft(0x140b64ba0, "content_140b64ba0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14316dec8> as *const ()), // sub_140b64ba0
    Replacement::draft(0x140b64ca0, "content_140b64ca0", tpl_12161::<0x14387ede8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b64ca0
    Replacement::draft_throws(0x140b64d50, "content_140b64d50", fn_140b64d50 as *const (), Throws::Discard), // sub_140b64d50
    Replacement::draft(0x140b64de0, "content_140b64de0", tpl_9969::<0x14387ede8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b64de0
    Replacement::draft_throws(0x140b64e60, "content_140b64e60", fn_140b64e60 as *const (), Throws::Discard), // sub_140b64e60
    Replacement::draft_throws(0x140b64ef0, "content_140b64ef0", fn_140b64ef0 as *const (), Throws::Caught), // sub_140b64ef0
    Replacement::draft_throws(0x140b64ff0, "content_140b64ff0", fn_140b64ff0 as *const (), Throws::Caught), // sub_140b64ff0
    Replacement::draft_throws(0x140b650f0, "content_140b650f0", fn_140b650f0 as *const (), Throws::Caught), // sub_140b650f0
    Replacement::draft(0x140b651f0, "content_140b651f0", fn_140b651f0 as *const ()), // sub_140b651f0
    Replacement::draft(0x140b65260, "content_140b65260", tpl_7856::<0x14387edb8, 0x14387ede8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xe8, 0x1401e9860> as *const ()), // sub_140b65260
    Replacement::draft(0x140b652d0, "content_140b652d0", tpl_9969::<0x14387ede8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b652d0
    Replacement::draft_throws(0x140b65350, "content_140b65350", tpl_7974::<0x14387edb8, 0x14387ede8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b65350
    Replacement::draft_throws(0x140b653d0, "content_140b653d0", fn_140b653d0 as *const (), Throws::Discard), // sub_140b653d0
    Replacement::draft(0x140b654e0, "content_140b654e0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431667e0> as *const ()), // sub_140b654e0
    Replacement::draft(0x140b65540, "content_140b65540", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431667e0> as *const ()), // sub_140b65540
    Replacement::draft(0x140b655d0, "content_140b655d0", fn_140b655d0 as *const ()), // sub_140b655d0
    Replacement::draft_throws(0x140b65840, "content_140b65840", tpl_14249::<0x14387f010, 0x14387eee0, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b65840
    Replacement::draft_throws(0x140b65aa0, "content_140b65aa0", tpl_14249::<0x14387f010, 0x14387eee0, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b65aa0
    Replacement::draft_throws(0x140b65d00, "content_140b65d00", fn_140b65d00 as *const (), Throws::Caught), // sub_140b65d00
    Replacement::draft(0x140b65d90, "content_140b65d90", fn_140b65d90 as *const ()), // sub_140b65d90
    Replacement::draft_throws(0x140b65ff0, "content_140b65ff0", tpl_14249::<0x14387f010, 0x14387eee0, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // sub_140b65ff0
    Replacement::draft_throws(0x140b66250, "content_140b66250", tpl_11903::<0x14387eee0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b66250
    Replacement::draft(0x140b66300, "content_140b66300", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143106928> as *const ()), // sub_140b66300
    Replacement::draft(0x140b66360, "content_140b66360", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143106928> as *const ()), // sub_140b66360
    Replacement::draft_throws(0x140b66460, "content_140b66460", fn_140b66460 as *const (), Throws::Caught), // sub_140b66460
    Replacement::draft_throws(0x140b664f0, "content_140b664f0", tpl_6763::<0x14387ea70, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b664f0
    Replacement::draft(0x140b665a0, "content_140b665a0", fn_140b665a0 as *const ()), // sub_140b665a0
    Replacement::draft(0x140b66680, "content_140b66680", fn_140b66680 as *const ()), // sub_140b66680
    Replacement::draft(0x140b66750, "content_140b66750", fn_140b66750 as *const ()), // sub_140b66750
    Replacement::draft(0x140b66870, "content_140b66870", fn_140b66870 as *const ()), // sub_140b66870
    Replacement::draft_throws(0x140b66990, "content_140b66990", fn_140b66990 as *const (), Throws::Discard), // sub_140b66990
    Replacement::draft(0x140b66b50, "content_140b66b50", fn_140b66b50 as *const ()), // sub_140b66b50
    Replacement::draft_throws(0x140b66c40, "content_140b66c40", fn_140b66c40 as *const (), Throws::Discard), // sub_140b66c40
    Replacement::draft(0x140b66e00, "content_140b66e00", fn_140b66e00 as *const ()), // sub_140b66e00
    Replacement::draft_throws(0x140b66e80, "content_140b66e80", fn_140b66e80 as *const (), Throws::Caught), // sub_140b66e80
    Replacement::draft_throws(0x140b66fb0, "content_140b66fb0", fn_140b66fb0 as *const (), Throws::Caught), // sub_140b66fb0
    Replacement::draft_throws(0x140b670e0, "content_140b670e0", fn_140b670e0 as *const (), Throws::Caught), // sub_140b670e0
    Replacement::draft(0x140b67380, "content_140b67380", fn_140b67380 as *const ()), // sub_140b67380
    Replacement::draft(0x140b67550, "content_140b67550", fn_140b67550 as *const ()), // sub_140b67550
    Replacement::draft(0x140b67750, "content_140b67750", fn_140b67750 as *const ()), // sub_140b67750
    Replacement::draft_throws(0x140b678e0, "content_140b678e0", fn_140b678e0 as *const (), Throws::Discard), // sub_140b678e0
    Replacement::draft_throws(0x140b679b0, "content_140b679b0", fn_140b679b0 as *const (), Throws::Caught), // sub_140b679b0
    Replacement::draft(0x140b67bf0, "content_140b67bf0", tpl_14216::<0x8, 0x1432b7fa0, 0x1432b7f88, 0x14088ba10, 0x1432b7f48, 0x20, 0x1432b80b0> as *const ()), // sub_140b67bf0
    Replacement::draft(0x140b67c70, "content_140b67c70", fn_140b67c70 as *const ()), // sub_140b67c70
    Replacement::draft(0x140b67d30, "content_140b67d30", fn_140b67d30 as *const ()), // sub_140b67d30
    Replacement::draft(0x140b67ea0, "content_140b67ea0", fn_140b67ea0 as *const ()), // sub_140b67ea0
    Replacement::draft_throws(0x140b680e0, "content_140b680e0", fn_140b680e0 as *const (), Throws::Discard), // sub_140b680e0
    Replacement::draft(0x140b68300, "content_140b68300", fn_140b68300 as *const ()), // sub_140b68300
    Replacement::draft_throws(0x140b68440, "content_140b68440", fn_140b68440 as *const (), Throws::Discard), // sub_140b68440
    Replacement::draft(0x140b68670, "content_140b68670", fn_140b68670 as *const ()), // sub_140b68670
    Replacement::draft_throws(0x140b68720, "content_140b68720", fn_140b68720 as *const (), Throws::Caught), // sub_140b68720
    Replacement::draft_throws(0x140b68bb0, "content_140b68bb0", fn_140b68bb0 as *const (), Throws::Caught), // sub_140b68bb0
    Replacement::draft_throws(0x140b69020, "content_140b69020", fn_140b69020 as *const (), Throws::Caught), // sub_140b69020
    Replacement::draft(0x140b69480, "content_140b69480", fn_140b69480 as *const ()), // sub_140b69480
    Replacement::draft(0x140b69550, "content_140b69550", fn_140b69550 as *const ()), // sub_140b69550
    Replacement::draft(0x140b696a0, "content_140b696a0", fn_140b696a0 as *const ()), // sub_140b696a0
    Replacement::draft_throws(0x140b69800, "content_140b69800", fn_140b69800 as *const (), Throws::Discard), // sub_140b69800
    Replacement::draft_throws(0x140b69930, "content_140b69930", fn_140b69930 as *const (), Throws::Discard), // sub_140b69930
    Replacement::draft(0x140b69c90, "content_140b69c90", fn_140b69c90 as *const ()), // sub_140b69c90
    Replacement::draft(0x140b69e70, "content_140b69e70", fn_140b69e70 as *const ()), // sub_140b69e70
    Replacement::draft(0x140b69f10, "content_140b69f10", fn_140b69f10 as *const ()), // sub_140b69f10
    Replacement::draft(0x140b69fc0, "content_140b69fc0", fn_140b69fc0 as *const ()), // sub_140b69fc0
    Replacement::draft(0x140b6a220, "content_140b6a220", fn_140b6a220 as *const ()), // sub_140b6a220
    Replacement::draft_throws(0x140b6a640, "content_140b6a640", fn_140b6a640 as *const (), Throws::Discard), // sub_140b6a640
    Replacement::draft(0x140b6aa20, "content_140b6aa20", fn_140b6aa20 as *const ()), // sub_140b6aa20
    Replacement::draft_throws(0x140b6ad50, "content_140b6ad50", fn_140b6ad50 as *const (), Throws::Discard), // sub_140b6ad50
    Replacement::draft(0x140b6b180, "content_140b6b180", fn_140b6b180 as *const ()), // sub_140b6b180
    Replacement::draft_throws(0x140b6b2b0, "content_140b6b2b0", fn_140b6b2b0 as *const (), Throws::Caught), // sub_140b6b2b0
    Replacement::draft_throws(0x140b6b840, "content_140b6b840", fn_140b6b840 as *const (), Throws::Caught), // sub_140b6b840
    Replacement::draft_throws(0x140b6bd40, "content_140b6bd40", fn_140b6bd40 as *const (), Throws::Caught), // sub_140b6bd40
    Replacement::draft(0x140b6c240, "content_140b6c240", fn_140b6c240 as *const ()), // sub_140b6c240
    Replacement::draft(0x140b6c4f0, "content_140b6c4f0", fn_140b6c4f0 as *const ()), // sub_140b6c4f0
    Replacement::draft(0x140b6c830, "content_140b6c830", fn_140b6c830 as *const ()), // sub_140b6c830
    Replacement::draft_throws(0x140b6cb80, "content_140b6cb80", fn_140b6cb80 as *const (), Throws::Discard), // sub_140b6cb80
    Replacement::draft_throws(0x140b6cee0, "content_140b6cee0", fn_140b6cee0 as *const (), Throws::Discard), // sub_140b6cee0
    Replacement::draft_throws(0x140b6d260, "content_140b6d260", fn_140b6d260 as *const (), Throws::Discard), // sub_140b6d260
    Replacement::draft(0x140b6d400, "content_140b6d400", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432cc5a8> as *const ()), // sub_140b6d400
    Replacement::draft(0x140b6d460, "content_140b6d460", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cc5a8> as *const ()), // sub_140b6d460
    Replacement::draft_throws(0x140b6d560, "content_140b6d560", fn_140b6d560 as *const (), Throws::Caught), // sub_140b6d560
    Replacement::draft_throws(0x140b6d5f0, "content_140b6d5f0", tpl_6763::<0x14387ec88, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b6d5f0
    Replacement::draft(0x140b6d6a0, "content_140b6d6a0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432c1658> as *const ()), // sub_140b6d6a0
    Replacement::draft(0x140b6d700, "content_140b6d700", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c1658> as *const ()), // sub_140b6d700
    Replacement::draft_throws(0x140b6d800, "content_140b6d800", fn_140b6d800 as *const (), Throws::Caught), // sub_140b6d800
    Replacement::draft_throws(0x140b6d890, "content_140b6d890", tpl_6763::<0x14387eb38, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b6d890
    Replacement::draft(0x140b6d940, "content_140b6d940", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432d06b8> as *const ()), // sub_140b6d940
    Replacement::draft(0x140b6d9a0, "content_140b6d9a0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432d06b8> as *const ()), // sub_140b6d9a0
    Replacement::draft_throws(0x140b6daa0, "content_140b6daa0", fn_140b6daa0 as *const (), Throws::Caught), // sub_140b6daa0
    Replacement::draft_throws(0x140b6db30, "content_140b6db30", tpl_6763::<0x14387e938, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b6db30
    Replacement::draft(0x140b6dbe0, "content_140b6dbe0", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430ebb50, 0x8> as *const ()), // sub_140b6dbe0
    Replacement::draft(0x140b6dc70, "content_140b6dc70", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430ebb50, 0x18> as *const ()), // sub_140b6dc70
    Replacement::draft(0x140b6dd20, "content_140b6dd20", fn_140b6dd20 as *const ()), // sub_140b6dd20
    Replacement::draft(0x140b6ddd0, "content_140b6ddd0", tpl_14295::<0x14387ebc8, 0x14243b0b0, 0x1406e01b0, 0x140c008c0, 0x8, 0x140c44e50, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b6ddd0
    Replacement::draft_throws(0x140b6de50, "content_140b6de50", fn_140b6de50 as *const (), Throws::Caught), // sub_140b6de50
    Replacement::draft(0x140b6dfe0, "content_140b6dfe0", tpl_14295::<0x14387ebc8, 0x14243b0b0, 0x1406e01b0, 0x140c008c0, 0x8, 0x140c451c0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b6dfe0
    Replacement::draft_throws(0x140b6e060, "content_140b6e060", fn_140b6e060 as *const (), Throws::Caught), // sub_140b6e060
    Replacement::draft_throws(0x140b6e1f0, "content_140b6e1f0", fn_140b6e1f0 as *const (), Throws::Caught), // sub_140b6e1f0
    Replacement::draft_throws(0x140b6e2b0, "content_140b6e2b0", fn_140b6e2b0 as *const (), Throws::Caught), // sub_140b6e2b0
    Replacement::draft_throws(0x140b6e370, "content_140b6e370", fn_140b6e370 as *const (), Throws::Caught), // sub_140b6e370
    Replacement::draft(0x140b6e430, "content_140b6e430", tpl_14295::<0x14387ebc8, 0x14243b0b0, 0x1406e01b0, 0x140c008c0, 0x8, 0x140c45090, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b6e430
    Replacement::draft(0x140b6e4b0, "content_140b6e4b0", fn_140b6e4b0 as *const ()), // sub_140b6e4b0
    Replacement::draft(0x140b6e520, "content_140b6e520", tpl_14295::<0x14387ebc8, 0x14243b0b0, 0x1406e01b0, 0x140c008c0, 0x8, 0x140c45440, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b6e520
    Replacement::draft_throws(0x140b6e5a0, "content_140b6e5a0", fn_140b6e5a0 as *const (), Throws::Discard), // sub_140b6e5a0
    Replacement::draft_throws(0x140b6e610, "content_140b6e610", fn_140b6e610 as *const (), Throws::Caught), // sub_140b6e610
    Replacement::draft(0x140b6e730, "content_140b6e730", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430d8740> as *const ()), // sub_140b6e730
    Replacement::draft(0x140b6e790, "content_140b6e790", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430d8740> as *const ()), // sub_140b6e790
    Replacement::draft_throws(0x140b6e890, "content_140b6e890", fn_140b6e890 as *const (), Throws::Caught), // sub_140b6e890
    Replacement::draft_throws(0x140b6e920, "content_140b6e920", tpl_6763::<0x14387f078, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b6e920
    Replacement::draft(0x140b6e9d0, "content_140b6e9d0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432c1180> as *const ()), // sub_140b6e9d0
    Replacement::draft(0x140b6ea30, "content_140b6ea30", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c1180> as *const ()), // sub_140b6ea30
    Replacement::draft_throws(0x140b6eb30, "content_140b6eb30", fn_140b6eb30 as *const (), Throws::Caught), // sub_140b6eb30
    Replacement::draft_throws(0x140b6ebc0, "content_140b6ebc0", tpl_6763::<0x14387ec18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b6ebc0
    Replacement::draft(0x140b6ec70, "content_140b6ec70", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430ebcc8> as *const ()), // sub_140b6ec70
    Replacement::draft(0x140b6ecd0, "content_140b6ecd0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430ebcc8> as *const ()), // sub_140b6ecd0
    Replacement::draft(0x140b6ed60, "content_140b6ed60", fn_140b6ed60 as *const ()), // sub_140b6ed60
    Replacement::draft_throws(0x140b6ee10, "content_140b6ee10", tpl_14302::<0x14387ea08, 0x14387f0c0, 0x1401e9fa0, 0x1406e02b0, 0x2, 0x1406d7a10, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b6ee10
    Replacement::draft_throws(0x140b6eeb0, "content_140b6eeb0", tpl_14302::<0x14387ea08, 0x14387f0c0, 0x1401e9b60, 0x1406e03a0, 0x2, 0x1406d7a10, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b6eeb0
    Replacement::draft_throws(0x140b6ef50, "content_140b6ef50", fn_140b6ef50 as *const (), Throws::Caught), // sub_140b6ef50
    Replacement::draft(0x140b6efe0, "content_140b6efe0", fn_140b6efe0 as *const ()), // sub_140b6efe0
    Replacement::draft_throws(0x140b6f080, "content_140b6f080", tpl_14302::<0x14387ea08, 0x14387f0c0, 0x1401e9770, 0x1406e1570, 0x2, 0x1406d7a10, 0x318, 0x328, 0x320, 0x8, 0x1, 0x38, 0x1401e9590> as *const (), Throws::Discard), // sub_140b6f080
    Replacement::draft_throws(0x140b6f120, "content_140b6f120", tpl_6737::<0x14387f0c0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b6f120
    Replacement::draft(0x140b6f1d0, "content_140b6f1d0", fn_140b6f1d0 as *const ()), // sub_140b6f1d0
    Replacement::draft(0x140b6f290, "content_140b6f290", fn_140b6f290 as *const ()), // sub_140b6f290
    Replacement::draft(0x140b6f350, "content_140b6f350", fn_140b6f350 as *const ()), // sub_140b6f350
    Replacement::draft(0x140b6f4c0, "content_140b6f4c0", fn_140b6f4c0 as *const ()), // sub_140b6f4c0
    Replacement::draft_throws(0x140b6f680, "content_140b6f680", fn_140b6f680 as *const (), Throws::Discard), // sub_140b6f680
    Replacement::draft(0x140b6f7a0, "content_140b6f7a0", fn_140b6f7a0 as *const ()), // sub_140b6f7a0
    Replacement::draft_throws(0x140b6f8f0, "content_140b6f8f0", fn_140b6f8f0 as *const (), Throws::Discard), // sub_140b6f8f0
    Replacement::draft(0x140b6fa10, "content_140b6fa10", fn_140b6fa10 as *const ()), // sub_140b6fa10
    Replacement::draft_throws(0x140b6fac0, "content_140b6fac0", fn_140b6fac0 as *const (), Throws::Caught), // sub_140b6fac0
    Replacement::draft_throws(0x140b6fba0, "content_140b6fba0", fn_140b6fba0 as *const (), Throws::Caught), // sub_140b6fba0
    Replacement::draft_throws(0x140b6fc80, "content_140b6fc80", fn_140b6fc80 as *const (), Throws::Caught), // sub_140b6fc80
    Replacement::draft(0x140b6fee0, "content_140b6fee0", fn_140b6fee0 as *const ()), // sub_140b6fee0
    Replacement::draft(0x140b70030, "content_140b70030", fn_140b70030 as *const ()), // sub_140b70030
    Replacement::draft(0x140b70170, "content_140b70170", fn_140b70170 as *const ()), // sub_140b70170
    Replacement::draft_throws(0x140b702c0, "content_140b702c0", fn_140b702c0 as *const (), Throws::Discard), // sub_140b702c0
    Replacement::draft_throws(0x140b703f0, "content_140b703f0", fn_140b703f0 as *const (), Throws::Discard), // sub_140b703f0
    Replacement::draft(0x140b70520, "content_140b70520", fn_140b70520 as *const ()), // sub_140b70520
    Replacement::draft(0x140b705a0, "content_140b705a0", tpl_11441::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1430e2be8, 0x8, 0x1c> as *const ()), // sub_140b705a0
    Replacement::draft(0x140b70630, "content_140b70630", tpl_11442::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x18, 0x1430e2be8, 0x5, 0x7, 0xffffffffffffffe8, 0xfffffffffffffff0> as *const ()), // sub_140b70630
    Replacement::draft(0x140b706e0, "content_140b706e0", tpl_9031::<0x14387efc8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b706e0
    Replacement::draft(0x140b70750, "content_140b70750", fn_140b70750 as *const ()), // sub_140b70750
    Replacement::draft_throws(0x140b707d0, "content_140b707d0", fn_140b707d0 as *const (), Throws::Discard), // sub_140b707d0
    Replacement::draft(0x140b70890, "content_140b70890", fn_140b70890 as *const ()), // sub_140b70890
    Replacement::draft_throws(0x140b70910, "content_140b70910", fn_140b70910 as *const (), Throws::Discard), // sub_140b70910
    Replacement::draft_throws(0x140b709d0, "content_140b709d0", fn_140b709d0 as *const (), Throws::Caught), // sub_140b709d0
    Replacement::draft_throws(0x140b70a80, "content_140b70a80", fn_140b70a80 as *const (), Throws::Caught), // sub_140b70a80
    Replacement::draft_throws(0x140b70b30, "content_140b70b30", fn_140b70b30 as *const (), Throws::Caught), // sub_140b70b30
    Replacement::draft(0x140b70be0, "content_140b70be0", fn_140b70be0 as *const ()), // sub_140b70be0
    Replacement::draft(0x140b70c80, "content_140b70c80", fn_140b70c80 as *const ()), // sub_140b70c80
    Replacement::draft(0x140b70d40, "content_140b70d40", fn_140b70d40 as *const ()), // sub_140b70d40
    Replacement::draft_throws(0x140b70e10, "content_140b70e10", fn_140b70e10 as *const (), Throws::Discard), // sub_140b70e10
    Replacement::draft_throws(0x140b70e80, "content_140b70e80", fn_140b70e80 as *const (), Throws::Caught), // sub_140b70e80
    Replacement::draft(0x140b70fc0, "content_140b70fc0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14311ee80, 0x8> as *const ()), // sub_140b70fc0
    Replacement::draft(0x140b71040, "content_140b71040", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14311ee80> as *const ()), // sub_140b71040
    Replacement::draft(0x140b710d0, "content_140b710d0", tpl_7312::<0x14387ef88, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b710d0
    Replacement::draft(0x140b71140, "content_140b71140", fn_140b71140 as *const ()), // sub_140b71140
    Replacement::draft_throws(0x140b71220, "content_140b71220", fn_140b71220 as *const (), Throws::Discard), // sub_140b71220
    Replacement::draft(0x140b71310, "content_140b71310", tpl_11480::<0x14387ef88, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0x9, 0xa, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b71310
    Replacement::draft_throws(0x140b713c0, "content_140b713c0", fn_140b713c0 as *const (), Throws::Discard), // sub_140b713c0
    Replacement::draft_throws(0x140b714b0, "content_140b714b0", fn_140b714b0 as *const (), Throws::Caught), // sub_140b714b0
    Replacement::draft_throws(0x140b71610, "content_140b71610", fn_140b71610 as *const (), Throws::Caught), // sub_140b71610
    Replacement::draft_throws(0x140b71770, "content_140b71770", fn_140b71770 as *const (), Throws::Caught), // sub_140b71770
    Replacement::draft(0x140b718d0, "content_140b718d0", fn_140b718d0 as *const ()), // sub_140b718d0
    Replacement::draft(0x140b71950, "content_140b71950", tpl_11485::<0x14387edd8, 0x14387ef88, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x9, 0xa, 0x1401e9860> as *const ()), // sub_140b71950
    Replacement::draft(0x140b719f0, "content_140b719f0", tpl_11480::<0x14387ef88, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0x9, 0xa, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b719f0
    Replacement::draft_throws(0x140b71aa0, "content_140b71aa0", fn_140b71aa0 as *const (), Throws::Discard), // sub_140b71aa0
    Replacement::draft_throws(0x140b71b50, "content_140b71b50", fn_140b71b50 as *const (), Throws::Discard), // sub_140b71b50
    Replacement::draft(0x140b71cd0, "content_140b71cd0", fn_140b71cd0 as *const ()), // sub_140b71cd0
    Replacement::draft(0x140b71d70, "content_140b71d70", fn_140b71d70 as *const ()), // sub_140b71d70
    Replacement::draft(0x140b71e20, "content_140b71e20", fn_140b71e20 as *const ()), // sub_140b71e20
    Replacement::draft(0x140b71fb0, "content_140b71fb0", fn_140b71fb0 as *const ()), // sub_140b71fb0
    Replacement::draft_throws(0x140b72270, "content_140b72270", fn_140b72270 as *const (), Throws::Discard), // sub_140b72270
    Replacement::draft(0x140b724d0, "content_140b724d0", fn_140b724d0 as *const ()), // sub_140b724d0
    Replacement::draft_throws(0x140b726b0, "content_140b726b0", fn_140b726b0 as *const (), Throws::Discard), // sub_140b726b0
    Replacement::draft(0x140b72990, "content_140b72990", fn_140b72990 as *const ()), // sub_140b72990
    Replacement::draft_throws(0x140b72a70, "content_140b72a70", fn_140b72a70 as *const (), Throws::Caught), // sub_140b72a70
    Replacement::draft_throws(0x140b72d40, "content_140b72d40", fn_140b72d40 as *const (), Throws::Caught), // sub_140b72d40
    Replacement::draft_throws(0x140b73010, "content_140b73010", fn_140b73010 as *const (), Throws::Caught), // sub_140b73010
    Replacement::draft(0x140b732e0, "content_140b732e0", fn_140b732e0 as *const ()), // sub_140b732e0
    Replacement::draft(0x140b73450, "content_140b73450", fn_140b73450 as *const ()), // sub_140b73450
    Replacement::draft(0x140b73620, "content_140b73620", fn_140b73620 as *const ()), // sub_140b73620
    Replacement::draft_throws(0x140b73800, "content_140b73800", fn_140b73800 as *const (), Throws::Discard), // sub_140b73800
    Replacement::draft_throws(0x140b73a10, "content_140b73a10", fn_140b73a10 as *const (), Throws::Discard), // sub_140b73a10
    Replacement::draft(0x140b73cd0, "content_140b73cd0", tpl_14339::<0x30, 0x1426cdcb0, 0x140c1f560> as *const ()), // sub_140b73cd0
    Replacement::draft(0x140b73d40, "content_140b73d40", fn_140b73d40 as *const ()), // sub_140b73d40
    Replacement::draft(0x140b73ed0, "content_140b73ed0", tpl_14339::<0x30, 0x1432b94d8, 0x140c1f560> as *const ()), // sub_140b73ed0
    Replacement::draft(0x140b73f40, "content_140b73f40", fn_140b73f40 as *const ()), // sub_140b73f40
    Replacement::draft(0x140b73fe0, "content_140b73fe0", fn_140b73fe0 as *const ()), // sub_140b73fe0
    Replacement::draft(0x140b74090, "content_140b74090", tpl_11830::<0x14387ee18, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b74090
    Replacement::draft(0x140b74100, "content_140b74100", fn_140b74100 as *const ()), // sub_140b74100
    Replacement::draft_throws(0x140b74420, "content_140b74420", fn_140b74420 as *const (), Throws::Discard), // sub_140b74420
    Replacement::draft(0x140b746c0, "content_140b746c0", fn_140b746c0 as *const ()), // sub_140b746c0
    Replacement::draft_throws(0x140b74820, "content_140b74820", fn_140b74820 as *const (), Throws::Discard), // sub_140b74820
    Replacement::draft(0x140b74b60, "content_140b74b60", fn_140b74b60 as *const ()), // sub_140b74b60
    Replacement::draft_throws(0x140b74c70, "content_140b74c70", fn_140b74c70 as *const (), Throws::Caught), // sub_140b74c70
    Replacement::draft_throws(0x140b750e0, "content_140b750e0", fn_140b750e0 as *const (), Throws::Caught), // sub_140b750e0
    Replacement::draft_throws(0x140b75550, "content_140b75550", fn_140b75550 as *const (), Throws::Caught), // sub_140b75550
    Replacement::draft(0x140b759c0, "content_140b759c0", fn_140b759c0 as *const ()), // sub_140b759c0
    Replacement::draft(0x140b75a80, "content_140b75a80", fn_140b75a80 as *const ()), // sub_140b75a80
    Replacement::draft(0x140b75c00, "content_140b75c00", fn_140b75c00 as *const ()), // sub_140b75c00
    Replacement::draft_throws(0x140b75d80, "content_140b75d80", fn_140b75d80 as *const (), Throws::Discard), // sub_140b75d80
    Replacement::draft_throws(0x140b75f80, "content_140b75f80", fn_140b75f80 as *const (), Throws::Discard), // sub_140b75f80
    Replacement::draft(0x140b763e0, "content_140b763e0", fn_140b763e0 as *const ()), // sub_140b763e0
    Replacement::draft(0x140b766e0, "content_140b766e0", tpl_11929::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432ce580> as *const ()), // sub_140b766e0
    Replacement::draft(0x140b76770, "content_140b76770", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ce580, 0x18, 0x20> as *const ()), // sub_140b76770
    Replacement::draft(0x140b76810, "content_140b76810", tpl_14355::<0x14387ed88, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140c471a0, 0x318, 0x328, 0x320, 0x4, 0x38, 0x2, 0x1403c79b0> as *const ()), // sub_140b76810
    Replacement::draft(0x140b768d0, "content_140b768d0", tpl_14355::<0x14387ed88, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140c47230, 0x318, 0x328, 0x320, 0x4, 0x38, 0x2, 0x1403c79b0> as *const ()), // sub_140b768d0
    Replacement::draft_throws(0x140b76990, "content_140b76990", tpl_14356::<0x14387e930, 0x14387ed88, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140c01260, 0x318, 0x328, 0x320, 0x4, 0x38, 0x2, 0x1403c79b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b76990
    Replacement::draft(0x140b76a40, "content_140b76a40", tpl_14355::<0x14387ed88, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140c47650, 0x318, 0x328, 0x320, 0x4, 0x38, 0x2, 0x1403c79b0> as *const ()), // sub_140b76a40
    Replacement::draft_throws(0x140b76b00, "content_140b76b00", tpl_14356::<0x14387e930, 0x14387ed88, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140c01500, 0x318, 0x328, 0x320, 0x4, 0x38, 0x2, 0x1403c79b0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b76b00
    Replacement::draft_throws(0x140b76bb0, "content_140b76bb0", fn_140b76bb0 as *const (), Throws::Caught), // sub_140b76bb0
    Replacement::draft_throws(0x140b76c70, "content_140b76c70", fn_140b76c70 as *const (), Throws::Caught), // sub_140b76c70
    Replacement::draft_throws(0x140b76d30, "content_140b76d30", fn_140b76d30 as *const (), Throws::Caught), // sub_140b76d30
    Replacement::draft(0x140b76df0, "content_140b76df0", tpl_14355::<0x14387ed88, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140c47590, 0x318, 0x328, 0x320, 0x4, 0x38, 0x2, 0x1403c79b0> as *const ()), // sub_140b76df0
    Replacement::draft(0x140b76eb0, "content_140b76eb0", fn_140b76eb0 as *const ()), // sub_140b76eb0
    Replacement::draft(0x140b76f60, "content_140b76f60", tpl_14355::<0x14387ed88, 0x14243b0b0, 0x1406e01b0, 0x8, 0x140c47a20, 0x318, 0x328, 0x320, 0x4, 0x38, 0x2, 0x1403c79b0> as *const ()), // sub_140b76f60
    Replacement::draft_throws(0x140b77020, "content_140b77020", fn_140b77020 as *const (), Throws::Discard), // sub_140b77020
    Replacement::draft_throws(0x140b770d0, "content_140b770d0", fn_140b770d0 as *const (), Throws::Caught), // sub_140b770d0
    Replacement::draft(0x140b771f0, "content_140b771f0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432c9000> as *const ()), // sub_140b771f0
    Replacement::draft(0x140b77250, "content_140b77250", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c9000> as *const ()), // sub_140b77250
    Replacement::draft_throws(0x140b77350, "content_140b77350", fn_140b77350 as *const (), Throws::Caught), // sub_140b77350
    Replacement::draft_throws(0x140b773e0, "content_140b773e0", tpl_6763::<0x14387ea20, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b773e0
    Replacement::draft(0x140b77490, "content_140b77490", fn_140b77490 as *const ()), // sub_140b77490
    Replacement::draft(0x140b77520, "content_140b77520", fn_140b77520 as *const ()), // sub_140b77520
    Replacement::draft(0x140b775d0, "content_140b775d0", fn_140b775d0 as *const ()), // sub_140b775d0
    Replacement::draft(0x140b776a0, "content_140b776a0", fn_140b776a0 as *const ()), // sub_140b776a0
    Replacement::draft_throws(0x140b77740, "content_140b77740", fn_140b77740 as *const (), Throws::Caught), // sub_140b77740
    Replacement::draft(0x140b77970, "content_140b77970", fn_140b77970 as *const ()), // sub_140b77970
    Replacement::draft_throws(0x140b77a30, "content_140b77a30", fn_140b77a30 as *const (), Throws::Caught), // sub_140b77a30
    Replacement::draft(0x140b77c60, "content_140b77c60", fn_140b77c60 as *const ()), // sub_140b77c60
    Replacement::draft_throws(0x140b77d50, "content_140b77d50", fn_140b77d50 as *const (), Throws::Caught), // sub_140b77d50
    Replacement::draft_throws(0x140b77ea0, "content_140b77ea0", fn_140b77ea0 as *const (), Throws::Caught), // sub_140b77ea0
    Replacement::draft_throws(0x140b77ff0, "content_140b77ff0", fn_140b77ff0 as *const (), Throws::Caught), // sub_140b77ff0
    Replacement::draft(0x140b78140, "content_140b78140", fn_140b78140 as *const ()), // sub_140b78140
    Replacement::draft(0x140b78210, "content_140b78210", fn_140b78210 as *const ()), // sub_140b78210
    Replacement::draft(0x140b78300, "content_140b78300", fn_140b78300 as *const ()), // sub_140b78300
    Replacement::draft_throws(0x140b78400, "content_140b78400", fn_140b78400 as *const (), Throws::Discard), // sub_140b78400
    Replacement::draft_throws(0x140b784e0, "content_140b784e0", fn_140b784e0 as *const (), Throws::Caught), // sub_140b784e0
    Replacement::draft(0x140b78790, "content_140b78790", fn_140b78790 as *const ()), // sub_140b78790
    Replacement::draft(0x140b78840, "content_140b78840", fn_140b78840 as *const ()), // sub_140b78840
    Replacement::draft(0x140b788e0, "content_140b788e0", fn_140b788e0 as *const ()), // sub_140b788e0
    Replacement::draft(0x140b789a0, "content_140b789a0", fn_140b789a0 as *const ()), // sub_140b789a0
    Replacement::draft(0x140b78b10, "content_140b78b10", fn_140b78b10 as *const ()), // sub_140b78b10
    Replacement::draft_throws(0x140b78db0, "content_140b78db0", fn_140b78db0 as *const (), Throws::Caught), // sub_140b78db0
    Replacement::draft(0x140b790b0, "content_140b790b0", fn_140b790b0 as *const ()), // sub_140b790b0
    Replacement::draft_throws(0x140b792f0, "content_140b792f0", fn_140b792f0 as *const (), Throws::Discard), // sub_140b792f0
    Replacement::draft(0x140b795b0, "content_140b795b0", fn_140b795b0 as *const ()), // sub_140b795b0
    Replacement::draft_throws(0x140b79640, "content_140b79640", fn_140b79640 as *const (), Throws::Caught), // sub_140b79640
    Replacement::draft_throws(0x140b79ac0, "content_140b79ac0", fn_140b79ac0 as *const (), Throws::Caught), // sub_140b79ac0
    Replacement::draft_throws(0x140b79ec0, "content_140b79ec0", fn_140b79ec0 as *const (), Throws::Caught), // sub_140b79ec0
    Replacement::draft(0x140b7a2b0, "content_140b7a2b0", fn_140b7a2b0 as *const ()), // sub_140b7a2b0
    Replacement::draft(0x140b7a530, "content_140b7a530", fn_140b7a530 as *const ()), // sub_140b7a530
    Replacement::draft(0x140b7a7a0, "content_140b7a7a0", fn_140b7a7a0 as *const ()), // sub_140b7a7a0
    Replacement::draft_throws(0x140b7aa20, "content_140b7aa20", fn_140b7aa20 as *const (), Throws::Discard), // sub_140b7aa20
    Replacement::draft_throws(0x140b7ac40, "content_140b7ac40", fn_140b7ac40 as *const (), Throws::Caught), // sub_140b7ac40
    Replacement::draft_throws(0x140b7af30, "content_140b7af30", fn_140b7af30 as *const (), Throws::Discard), // sub_140b7af30
    Replacement::draft(0x140b7b050, "content_140b7b050", fn_140b7b050 as *const ()), // sub_140b7b050
    Replacement::draft(0x140b7b120, "content_140b7b120", fn_140b7b120 as *const ()), // sub_140b7b120
    Replacement::draft(0x140b7b220, "content_140b7b220", fn_140b7b220 as *const ()), // sub_140b7b220
    Replacement::draft(0x140b7b410, "content_140b7b410", fn_140b7b410 as *const ()), // sub_140b7b410
    Replacement::draft_throws(0x140b7b750, "content_140b7b750", fn_140b7b750 as *const (), Throws::Discard), // sub_140b7b750
    Replacement::draft(0x140b7ba20, "content_140b7ba20", fn_140b7ba20 as *const ()), // sub_140b7ba20
    Replacement::draft_throws(0x140b7bc60, "content_140b7bc60", fn_140b7bc60 as *const (), Throws::Discard), // sub_140b7bc60
    Replacement::draft_throws(0x140b7bef0, "content_140b7bef0", fn_140b7bef0 as *const (), Throws::Caught), // sub_140b7bef0
    Replacement::draft_throws(0x140b7c1d0, "content_140b7c1d0", fn_140b7c1d0 as *const (), Throws::Caught), // sub_140b7c1d0
    Replacement::draft_throws(0x140b7c480, "content_140b7c480", fn_140b7c480 as *const (), Throws::Caught), // sub_140b7c480
    Replacement::draft(0x140b7c720, "content_140b7c720", fn_140b7c720 as *const ()), // sub_140b7c720
    Replacement::draft(0x140b7c9f0, "content_140b7c9f0", fn_140b7c9f0 as *const ()), // sub_140b7c9f0
    Replacement::draft(0x140b7cc10, "content_140b7cc10", fn_140b7cc10 as *const ()), // sub_140b7cc10
    Replacement::draft_throws(0x140b7ce40, "content_140b7ce40", fn_140b7ce40 as *const (), Throws::Discard), // sub_140b7ce40
    Replacement::draft_throws(0x140b7d070, "content_140b7d070", fn_140b7d070 as *const (), Throws::Discard), // sub_140b7d070
    Replacement::draft(0x140b7d200, "content_140b7d200", tpl_14406::<0x143426d70, 0x143426ce0, 0x1432bb128, 0xa8, 0x1432bb010, 0x1432bafe0, 0x14020f270> as *const ()), // sub_140b7d200
    Replacement::draft(0x140b7d270, "content_140b7d270", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432c3ad0> as *const ()), // sub_140b7d270
    Replacement::draft(0x140b7d2d0, "content_140b7d2d0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c3ad0> as *const ()), // sub_140b7d2d0
    Replacement::draft_throws(0x140b7d3d0, "content_140b7d3d0", fn_140b7d3d0 as *const (), Throws::Caught), // sub_140b7d3d0
    Replacement::draft_throws(0x140b7d460, "content_140b7d460", tpl_6763::<0x14387eb00, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b7d460
    Replacement::draft(0x140b7d510, "content_140b7d510", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14314f7f0> as *const ()), // sub_140b7d510
    Replacement::draft(0x140b7d570, "content_140b7d570", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14314f7f0> as *const ()), // sub_140b7d570
    Replacement::draft_throws(0x140b7d670, "content_140b7d670", fn_140b7d670 as *const (), Throws::Caught), // sub_140b7d670
    Replacement::draft_throws(0x140b7d700, "content_140b7d700", tpl_6762::<0x14387efb0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b7d700
    Replacement::draft(0x140b7d7b0, "content_140b7d7b0", fn_140b7d7b0 as *const ()), // sub_140b7d7b0
    Replacement::draft(0x140b7d850, "content_140b7d850", fn_140b7d850 as *const ()), // sub_140b7d850
    Replacement::draft(0x140b7d900, "content_140b7d900", fn_140b7d900 as *const ()), // sub_140b7d900
    Replacement::draft(0x140b7d9a0, "content_140b7d9a0", fn_140b7d9a0 as *const ()), // sub_140b7d9a0
    Replacement::draft_throws(0x140b7dbd0, "content_140b7dbd0", fn_140b7dbd0 as *const (), Throws::Discard), // sub_140b7dbd0
    Replacement::draft(0x140b7ddb0, "content_140b7ddb0", fn_140b7ddb0 as *const ()), // sub_140b7ddb0
    Replacement::draft_throws(0x140b7df20, "content_140b7df20", fn_140b7df20 as *const (), Throws::Discard), // sub_140b7df20
    Replacement::draft(0x140b7e140, "content_140b7e140", fn_140b7e140 as *const ()), // sub_140b7e140
    Replacement::draft_throws(0x140b7e290, "content_140b7e290", fn_140b7e290 as *const (), Throws::Caught), // sub_140b7e290
    Replacement::draft_throws(0x140b7e440, "content_140b7e440", fn_140b7e440 as *const (), Throws::Caught), // sub_140b7e440
    Replacement::draft_throws(0x140b7e5f0, "content_140b7e5f0", fn_140b7e5f0 as *const (), Throws::Caught), // sub_140b7e5f0
    Replacement::draft(0x140b7e7a0, "content_140b7e7a0", fn_140b7e7a0 as *const ()), // sub_140b7e7a0
    Replacement::draft(0x140b7e880, "content_140b7e880", fn_140b7e880 as *const ()), // sub_140b7e880
    Replacement::draft(0x140b7e9b0, "content_140b7e9b0", fn_140b7e9b0 as *const ()), // sub_140b7e9b0
    Replacement::draft_throws(0x140b7eaf0, "content_140b7eaf0", fn_140b7eaf0 as *const (), Throws::Discard), // sub_140b7eaf0
    Replacement::draft_throws(0x140b7ec30, "content_140b7ec30", fn_140b7ec30 as *const (), Throws::Discard), // sub_140b7ec30
    Replacement::draft(0x140b7ee00, "content_140b7ee00", tpl_14417::<0x20, 0x1426cdcb0, 0x140c252b0> as *const ()), // sub_140b7ee00
    Replacement::draft(0x140b7ee70, "content_140b7ee70", tpl_14417::<0x20, 0x1432bb108, 0x140c252b0> as *const ()), // sub_140b7ee70
    Replacement::draft(0x140b7eee0, "content_140b7eee0", fn_140b7eee0 as *const ()), // sub_140b7eee0
    Replacement::draft(0x140b7efe0, "content_140b7efe0", fn_140b7efe0 as *const ()), // sub_140b7efe0
    Replacement::draft(0x140b7f200, "content_140b7f200", fn_140b7f200 as *const ()), // sub_140b7f200
    Replacement::draft_throws(0x140b7f360, "content_140b7f360", fn_140b7f360 as *const (), Throws::Discard), // sub_140b7f360
    Replacement::draft(0x140b7f4c0, "content_140b7f4c0", tpl_14421::<0x14387f020, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x90, 0x118, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b7f4c0
    Replacement::draft_throws(0x140b7f560, "content_140b7f560", fn_140b7f560 as *const (), Throws::Discard), // sub_140b7f560
    Replacement::draft_throws(0x140b7f6b0, "content_140b7f6b0", fn_140b7f6b0 as *const (), Throws::Caught), // sub_140b7f6b0
    Replacement::draft_throws(0x140b7fc30, "content_140b7fc30", fn_140b7fc30 as *const (), Throws::Caught), // sub_140b7fc30
    Replacement::draft_throws(0x140b80120, "content_140b80120", fn_140b80120 as *const (), Throws::Caught), // sub_140b80120
    Replacement::draft(0x140b805e0, "content_140b805e0", fn_140b805e0 as *const ()), // sub_140b805e0
    Replacement::draft(0x140b806d0, "content_140b806d0", tpl_11949::<0x14387e9a8, 0x14387f020, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x90, 0x118, 0x1401e9860> as *const ()), // sub_140b806d0
    Replacement::draft(0x140b80770, "content_140b80770", tpl_14421::<0x14387f020, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x90, 0x118, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b80770
    Replacement::draft_throws(0x140b80820, "content_140b80820", fn_140b80820 as *const (), Throws::Discard), // sub_140b80820
    Replacement::draft_throws(0x140b808b0, "content_140b808b0", fn_140b808b0 as *const (), Throws::Discard), // sub_140b808b0
    Replacement::draft(0x140b80a20, "content_140b80a20", fn_140b80a20 as *const ()), // sub_140b80a20
    Replacement::draft(0x140b80b30, "content_140b80b30", fn_140b80b30 as *const ()), // sub_140b80b30
    Replacement::draft(0x140b80c30, "content_140b80c30", fn_140b80c30 as *const ()), // sub_140b80c30
    Replacement::draft(0x140b80e00, "content_140b80e00", fn_140b80e00 as *const ()), // sub_140b80e00
    Replacement::draft_throws(0x140b810f0, "content_140b810f0", fn_140b810f0 as *const (), Throws::Discard), // sub_140b810f0
    Replacement::draft(0x140b813a0, "content_140b813a0", fn_140b813a0 as *const ()), // sub_140b813a0
    Replacement::draft_throws(0x140b815c0, "content_140b815c0", fn_140b815c0 as *const (), Throws::Discard), // sub_140b815c0
    Replacement::draft(0x140b81880, "content_140b81880", fn_140b81880 as *const ()), // sub_140b81880
    Replacement::draft_throws(0x140b819c0, "content_140b819c0", fn_140b819c0 as *const (), Throws::Caught), // sub_140b819c0
    Replacement::draft_throws(0x140b81f30, "content_140b81f30", fn_140b81f30 as *const (), Throws::Caught), // sub_140b81f30
    Replacement::draft_throws(0x140b82400, "content_140b82400", fn_140b82400 as *const (), Throws::Caught), // sub_140b82400
    Replacement::draft(0x140b828c0, "content_140b828c0", fn_140b828c0 as *const ()), // sub_140b828c0
    Replacement::draft(0x140b82ad0, "content_140b82ad0", fn_140b82ad0 as *const ()), // sub_140b82ad0
    Replacement::draft(0x140b82d00, "content_140b82d00", fn_140b82d00 as *const ()), // sub_140b82d00
    Replacement::draft_throws(0x140b82f40, "content_140b82f40", fn_140b82f40 as *const (), Throws::Discard), // sub_140b82f40
    Replacement::draft_throws(0x140b83170, "content_140b83170", fn_140b83170 as *const (), Throws::Caught), // sub_140b83170
    Replacement::draft(0x140b83390, "content_140b83390", tpl_14442::<0x143877120, 0x1432baf00, 0x1432baf30, 0x8, 0x1432baf18, 0x1401de410, 0x143877170, 0xc, 0x1432bafc0, 0x1432baf98, 0x1432bafa8, 0x15, 0x1432bac00, 0x1432babe8, 0x140c05f40, 0x1432bacb8, 0x1432baca0> as *const ()), // sub_140b83390
    Replacement::draft(0x140b83470, "content_140b83470", tpl_14442::<0x1438770b0, 0x1432baf00, 0x1432baf30, 0x8, 0x1432baf18, 0x1401de410, 0x1438746d0, 0xc, 0x1432bafc0, 0x1432baf98, 0x1432bafa8, 0x15, 0x1432bac00, 0x1432babe8, 0x140c05f70, 0x1432bacb8, 0x1432baca0> as *const ()), // sub_140b83470
    Replacement::draft(0x140b83550, "content_140b83550", tpl_7434::<0x130, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140bf9f80> as *const ()), // sub_140b83550
    Replacement::draft(0x140b835e0, "content_140b835e0", tpl_66380::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x140bf9f80> as *const ()), // sub_140b835e0
    Replacement::draft(0x140b83690, "content_140b83690", fn_140b83690 as *const ()), // sub_140b83690
    Replacement::draft(0x140b83bc0, "content_140b83bc0", fn_140b83bc0 as *const ()), // sub_140b83bc0
    Replacement::draft_throws(0x140b84630, "content_140b84630", fn_140b84630 as *const (), Throws::Caught), // sub_140b84630
    Replacement::draft(0x140b84f90, "content_140b84f90", fn_140b84f90 as *const ()), // sub_140b84f90
    Replacement::draft_throws(0x140b85650, "content_140b85650", fn_140b85650 as *const (), Throws::Caught), // sub_140b85650
    Replacement::draft(0x140b85c70, "content_140b85c70", fn_140b85c70 as *const ()), // sub_140b85c70
    Replacement::draft_throws(0x140b85d90, "content_140b85d90", fn_140b85d90 as *const (), Throws::Caught), // sub_140b85d90
    Replacement::draft_throws(0x140b85f70, "content_140b85f70", fn_140b85f70 as *const (), Throws::Caught), // sub_140b85f70
    Replacement::draft_throws(0x140b86150, "content_140b86150", fn_140b86150 as *const (), Throws::Caught), // sub_140b86150
    Replacement::draft(0x140b86330, "content_140b86330", fn_140b86330 as *const ()), // sub_140b86330
    Replacement::draft(0x140b86c10, "content_140b86c10", fn_140b86c10 as *const ()), // sub_140b86c10
    Replacement::draft(0x140b87240, "content_140b87240", fn_140b87240 as *const ()), // sub_140b87240
    Replacement::draft_throws(0x140b87880, "content_140b87880", fn_140b87880 as *const (), Throws::Caught), // sub_140b87880
    Replacement::draft_throws(0x140b87f20, "content_140b87f20", fn_140b87f20 as *const (), Throws::Discard), // sub_140b87f20
    Replacement::draft(0x140b88230, "content_140b88230", fn_140b88230 as *const ()), // sub_140b88230
    Replacement::draft(0x140b88450, "content_140b88450", tpl_6513::<0xe8, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140bfa090> as *const ()), // sub_140b88450
    Replacement::draft(0x140b884e0, "content_140b884e0", fn_140b884e0 as *const ()), // sub_140b884e0
    Replacement::draft(0x140b88590, "content_140b88590", fn_140b88590 as *const ()), // sub_140b88590
    Replacement::draft(0x140b88cb0, "content_140b88cb0", fn_140b88cb0 as *const ()), // sub_140b88cb0
    Replacement::draft_throws(0x140b89880, "content_140b89880", fn_140b89880 as *const (), Throws::Discard), // sub_140b89880
    Replacement::draft(0x140b8a2f0, "content_140b8a2f0", fn_140b8a2f0 as *const ()), // sub_140b8a2f0
    Replacement::draft_throws(0x140b8ab30, "content_140b8ab30", fn_140b8ab30 as *const (), Throws::Discard), // sub_140b8ab30
    Replacement::draft(0x140b8b2f0, "content_140b8b2f0", fn_140b8b2f0 as *const ()), // sub_140b8b2f0
    Replacement::draft_throws(0x140b8b3d0, "content_140b8b3d0", fn_140b8b3d0 as *const (), Throws::Caught), // sub_140b8b3d0
    Replacement::draft_throws(0x140b8b560, "content_140b8b560", fn_140b8b560 as *const (), Throws::Caught), // sub_140b8b560
    Replacement::draft_throws(0x140b8b6f0, "content_140b8b6f0", fn_140b8b6f0 as *const (), Throws::Caught), // sub_140b8b6f0
    Replacement::draft(0x140b8b880, "content_140b8b880", fn_140b8b880 as *const ()), // sub_140b8b880
    Replacement::draft(0x140b8c2a0, "content_140b8c2a0", fn_140b8c2a0 as *const ()), // sub_140b8c2a0
    Replacement::draft(0x140b8ca70, "content_140b8ca70", fn_140b8ca70 as *const ()), // sub_140b8ca70
    Replacement::draft_throws(0x140b8d250, "content_140b8d250", fn_140b8d250 as *const (), Throws::Discard), // sub_140b8d250
    Replacement::draft_throws(0x140b8da70, "content_140b8da70", fn_140b8da70 as *const (), Throws::Discard), // sub_140b8da70
    Replacement::draft(0x140b8dce0, "content_140b8dce0", fn_140b8dce0 as *const ()), // sub_140b8dce0
    Replacement::draft(0x140b8de80, "content_140b8de80", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143201f88, 0x8> as *const ()), // sub_140b8de80
    Replacement::draft(0x140b8df00, "content_140b8df00", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143201f88> as *const ()), // sub_140b8df00
    Replacement::draft(0x140b8e000, "content_140b8e000", tpl_13874::<0x14387ed28, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1426d4580, 0x338, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b8e000
    Replacement::draft_throws(0x140b8e0c0, "content_140b8e0c0", fn_140b8e0c0 as *const (), Throws::Caught), // sub_140b8e0c0
    Replacement::draft(0x140b8e190, "content_140b8e190", tpl_13876::<0x14387ed28, 0x14243b0b0, 0x1406e01b0, 0x338, 0x20, 0x40, 0x8, 0x14240ab10, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140b8e190
    Replacement::draft_throws(0x140b8e250, "content_140b8e250", tpl_7839::<0x14387f070, 0x14387ed28, 0x1401e9b60, 0x1406e03a0, 0x1426cdcb0, 0x8, 0x1402c1540, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b8e250
    Replacement::draft_throws(0x140b8e2d0, "content_140b8e2d0", fn_140b8e2d0 as *const (), Throws::Caught), // sub_140b8e2d0
    Replacement::draft_throws(0x140b8e4c0, "content_140b8e4c0", fn_140b8e4c0 as *const (), Throws::Caught), // sub_140b8e4c0
    Replacement::draft_throws(0x140b8e680, "content_140b8e680", fn_140b8e680 as *const (), Throws::Caught), // sub_140b8e680
    Replacement::draft(0x140b8e840, "content_140b8e840", tpl_13880::<0x14387ed28, 0x14243b0b0, 0x1406e01b0, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_140b8e840
    Replacement::draft(0x140b8e970, "content_140b8e970", tpl_7871::<0x14387f070, 0x14387ed28, 0x1401e9970, 0x1406e1480, 0x338, 0x1426cdcb0, 0x8, 0xa0, 0xe0, 0x1401e9860> as *const ()), // sub_140b8e970
    Replacement::draft(0x140b8ea10, "content_140b8ea10", tpl_13881::<0x14387ed28, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426cdcb0, 0x8, 0xa0, 0xe0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b8ea10
    Replacement::draft_throws(0x140b8eab0, "content_140b8eab0", tpl_7873::<0x14387f070, 0x14387ed28, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b8eab0
    Replacement::draft_throws(0x140b8eb40, "content_140b8eb40", fn_140b8eb40 as *const (), Throws::Discard), // sub_140b8eb40
    Replacement::draft(0x140b8ec30, "content_140b8ec30", tpl_8296::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x14314dec0, 0x20, 0x28, 0x30> as *const ()), // sub_140b8ec30
    Replacement::draft(0x140b8ecd0, "content_140b8ecd0", fn_140b8ecd0 as *const ()), // sub_140b8ecd0
    Replacement::draft(0x140b8ed80, "content_140b8ed80", tpl_13809::<0x14387eb90, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b8ed80
    Replacement::draft(0x140b8edf0, "content_140b8edf0", fn_140b8edf0 as *const ()), // sub_140b8edf0
    Replacement::draft_throws(0x140b8f0b0, "content_140b8f0b0", fn_140b8f0b0 as *const (), Throws::Discard), // sub_140b8f0b0
    Replacement::draft(0x140b8f320, "content_140b8f320", fn_140b8f320 as *const ()), // sub_140b8f320
    Replacement::draft_throws(0x140b8f470, "content_140b8f470", fn_140b8f470 as *const (), Throws::Discard), // sub_140b8f470
    Replacement::draft(0x140b8f780, "content_140b8f780", fn_140b8f780 as *const ()), // sub_140b8f780
    Replacement::draft_throws(0x140b8f880, "content_140b8f880", fn_140b8f880 as *const (), Throws::Caught), // sub_140b8f880
    Replacement::draft_throws(0x140b8fca0, "content_140b8fca0", fn_140b8fca0 as *const (), Throws::Caught), // sub_140b8fca0
    Replacement::draft_throws(0x140b900c0, "content_140b900c0", fn_140b900c0 as *const (), Throws::Caught), // sub_140b900c0
    Replacement::draft(0x140b904e0, "content_140b904e0", fn_140b904e0 as *const ()), // sub_140b904e0
    Replacement::draft(0x140b905a0, "content_140b905a0", fn_140b905a0 as *const ()), // sub_140b905a0
    Replacement::draft(0x140b90700, "content_140b90700", fn_140b90700 as *const ()), // sub_140b90700
    Replacement::draft_throws(0x140b90870, "content_140b90870", fn_140b90870 as *const (), Throws::Discard), // sub_140b90870
    Replacement::draft_throws(0x140b90a50, "content_140b90a50", fn_140b90a50 as *const (), Throws::Discard), // sub_140b90a50
    Replacement::draft(0x140b90e70, "content_140b90e70", fn_140b90e70 as *const ()), // sub_140b90e70
    Replacement::draft(0x140b910c0, "content_140b910c0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431488f0> as *const ()), // sub_140b910c0
    Replacement::draft(0x140b91120, "content_140b91120", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431488f0> as *const ()), // sub_140b91120
    Replacement::draft(0x140b911b0, "content_140b911b0", fn_140b911b0 as *const ()), // sub_140b911b0
    Replacement::draft_throws(0x140b91260, "content_140b91260", tpl_14478::<0x14387eae0, 0x14387eaf8, 0x1401e9fa0, 0x1406e02b0, 0x140c06130, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b91260
    Replacement::draft_throws(0x140b91300, "content_140b91300", tpl_14478::<0x14387eae0, 0x14387eaf8, 0x1401e9b60, 0x1406e03a0, 0x140c06130, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b91300
    Replacement::draft_throws(0x140b913a0, "content_140b913a0", fn_140b913a0 as *const (), Throws::Caught), // sub_140b913a0
    Replacement::draft(0x140b91430, "content_140b91430", fn_140b91430 as *const ()), // sub_140b91430
    Replacement::draft_throws(0x140b914d0, "content_140b914d0", tpl_14478::<0x14387eae0, 0x14387eaf8, 0x1401e9770, 0x1406e1570, 0x140c06130, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0x1401e9590> as *const (), Throws::Discard), // sub_140b914d0
    Replacement::draft_throws(0x140b91570, "content_140b91570", tpl_11903::<0x14387eaf8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b91570
    Replacement::draft(0x140b91620, "content_140b91620", tpl_9792::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430bf490, 0x8> as *const ()), // sub_140b91620
    Replacement::draft(0x140b916b0, "content_140b916b0", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430bf490, 0x18> as *const ()), // sub_140b916b0
    Replacement::draft(0x140b91760, "content_140b91760", tpl_9031::<0x14387efd8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b91760
    Replacement::draft(0x140b917d0, "content_140b917d0", fn_140b917d0 as *const ()), // sub_140b917d0
    Replacement::draft_throws(0x140b918d0, "content_140b918d0", fn_140b918d0 as *const (), Throws::Caught), // sub_140b918d0
    Replacement::draft(0x140b91a30, "content_140b91a30", fn_140b91a30 as *const ()), // sub_140b91a30
    Replacement::draft_throws(0x140b91b20, "content_140b91b20", fn_140b91b20 as *const (), Throws::Discard), // sub_140b91b20
    Replacement::draft_throws(0x140b91c10, "content_140b91c10", fn_140b91c10 as *const (), Throws::Caught), // sub_140b91c10
    Replacement::draft_throws(0x140b91fb0, "content_140b91fb0", fn_140b91fb0 as *const (), Throws::Caught), // sub_140b91fb0
    Replacement::draft_throws(0x140b92300, "content_140b92300", fn_140b92300 as *const (), Throws::Caught), // sub_140b92300
    Replacement::draft(0x140b92630, "content_140b92630", fn_140b92630 as *const ()), // sub_140b92630
    Replacement::draft(0x140b92760, "content_140b92760", fn_140b92760 as *const ()), // sub_140b92760
    Replacement::draft(0x140b92830, "content_140b92830", fn_140b92830 as *const ()), // sub_140b92830
    Replacement::draft_throws(0x140b92900, "content_140b92900", fn_140b92900 as *const (), Throws::Discard), // sub_140b92900
    Replacement::draft_throws(0x140b929d0, "content_140b929d0", fn_140b929d0 as *const (), Throws::Discard), // sub_140b929d0
    Replacement::draft(0x140b92b30, "content_140b92b30", tpl_8298::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1430d88b8, 0x8> as *const ()), // sub_140b92b30
    Replacement::draft(0x140b92bc0, "content_140b92bc0", tpl_8299::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1430d88b8, 0xffffffffffffffec, 0xffffffffffffffe4, 0xfffffffffffffff4, 0xfffffffffffffffc> as *const ()), // sub_140b92bc0
    Replacement::draft(0x140b92c60, "content_140b92c60", fn_140b92c60 as *const ()), // sub_140b92c60
    Replacement::draft(0x140b92e30, "content_140b92e30", fn_140b92e30 as *const ()), // sub_140b92e30
    Replacement::draft_throws(0x140b93040, "content_140b93040", fn_140b93040 as *const (), Throws::Caught), // sub_140b93040
    Replacement::draft(0x140b93230, "content_140b93230", fn_140b93230 as *const ()), // sub_140b93230
    Replacement::draft_throws(0x140b93440, "content_140b93440", fn_140b93440 as *const (), Throws::Discard), // sub_140b93440
    Replacement::draft(0x140b935c0, "content_140b935c0", fn_140b935c0 as *const ()), // sub_140b935c0
    Replacement::draft_throws(0x140b93640, "content_140b93640", fn_140b93640 as *const (), Throws::Caught), // sub_140b93640
    Replacement::draft_throws(0x140b93890, "content_140b93890", fn_140b93890 as *const (), Throws::Caught), // sub_140b93890
    Replacement::draft_throws(0x140b93ab0, "content_140b93ab0", fn_140b93ab0 as *const (), Throws::Caught), // sub_140b93ab0
    Replacement::draft(0x140b93cd0, "content_140b93cd0", fn_140b93cd0 as *const ()), // sub_140b93cd0
    Replacement::draft(0x140b93f40, "content_140b93f40", fn_140b93f40 as *const ()), // sub_140b93f40
    Replacement::draft(0x140b94110, "content_140b94110", fn_140b94110 as *const ()), // sub_140b94110
    Replacement::draft_throws(0x140b94300, "content_140b94300", fn_140b94300 as *const (), Throws::Discard), // sub_140b94300
    Replacement::draft_throws(0x140b944d0, "content_140b944d0", fn_140b944d0 as *const (), Throws::Discard), // sub_140b944d0
    Replacement::draft(0x140b945f0, "content_140b945f0", tpl_14504::<0x18, 0x1426cdcb0, 0x140c26b60> as *const ()), // sub_140b945f0
    Replacement::draft(0x140b94660, "content_140b94660", tpl_14504::<0x18, 0x1432bc228, 0x140c26b60> as *const ()), // sub_140b94660
    Replacement::draft(0x140b946d0, "content_140b946d0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432d18c0> as *const ()), // sub_140b946d0
    Replacement::draft(0x140b94730, "content_140b94730", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432d18c0> as *const ()), // sub_140b94730
    Replacement::draft_throws(0x140b94830, "content_140b94830", fn_140b94830 as *const (), Throws::Caught), // sub_140b94830
    Replacement::draft_throws(0x140b948c0, "content_140b948c0", tpl_6763::<0x14387ef58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b948c0
    Replacement::draft(0x140b94970, "content_140b94970", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143141708, 0x8> as *const ()), // sub_140b94970
    Replacement::draft(0x140b94a00, "content_140b94a00", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x143141708, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140b94a00
    Replacement::draft(0x140b94ab0, "content_140b94ab0", fn_140b94ab0 as *const ()), // sub_140b94ab0
    Replacement::draft(0x140b94cc0, "content_140b94cc0", fn_140b94cc0 as *const ()), // sub_140b94cc0
    Replacement::draft_throws(0x140b94ef0, "content_140b94ef0", tpl_14506::<0x14387eb18, 0x14387ebd8, 0x1401e9fa0, 0x1406e02b0, 0x1403c79b0, 0x140c065f0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b94ef0
    Replacement::draft(0x140b95090, "content_140b95090", fn_140b95090 as *const ()), // sub_140b95090
    Replacement::draft_throws(0x140b95290, "content_140b95290", tpl_14506::<0x14387eb18, 0x14387ebd8, 0x1401e9b60, 0x1406e03a0, 0x1403c79b0, 0x140c067a0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b95290
    Replacement::draft(0x140b95430, "content_140b95430", fn_140b95430 as *const ()), // sub_140b95430
    Replacement::draft_throws(0x140b954b0, "content_140b954b0", fn_140b954b0 as *const (), Throws::Caught), // sub_140b954b0
    Replacement::draft_throws(0x140b95580, "content_140b95580", fn_140b95580 as *const (), Throws::Caught), // sub_140b95580
    Replacement::draft_throws(0x140b95650, "content_140b95650", fn_140b95650 as *const (), Throws::Caught), // sub_140b95650
    Replacement::draft(0x140b95720, "content_140b95720", fn_140b95720 as *const ()), // sub_140b95720
    Replacement::draft(0x140b95920, "content_140b95920", fn_140b95920 as *const ()), // sub_140b95920
    Replacement::draft(0x140b95b10, "content_140b95b10", fn_140b95b10 as *const ()), // sub_140b95b10
    Replacement::draft_throws(0x140b95d10, "content_140b95d10", fn_140b95d10 as *const (), Throws::Discard), // sub_140b95d10
    Replacement::draft_throws(0x140b95f00, "content_140b95f00", fn_140b95f00 as *const (), Throws::Discard), // sub_140b95f00
    Replacement::draft(0x140b95ff0, "content_140b95ff0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432ccd30, 0x8> as *const ()), // sub_140b95ff0
    Replacement::draft(0x140b96070, "content_140b96070", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ccd30> as *const ()), // sub_140b96070
    Replacement::draft(0x140b96100, "content_140b96100", tpl_7312::<0x14387e9d0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b96100
    Replacement::draft(0x140b96170, "content_140b96170", tpl_11821::<0x14387e9d0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b96170
    Replacement::draft_throws(0x140b96210, "content_140b96210", fn_140b96210 as *const (), Throws::Discard), // sub_140b96210
    Replacement::draft(0x140b962a0, "content_140b962a0", tpl_11823::<0x14387e9d0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b962a0
    Replacement::draft_throws(0x140b96320, "content_140b96320", fn_140b96320 as *const (), Throws::Discard), // sub_140b96320
    Replacement::draft_throws(0x140b963b0, "content_140b963b0", fn_140b963b0 as *const (), Throws::Caught), // sub_140b963b0
    Replacement::draft_throws(0x140b96490, "content_140b96490", fn_140b96490 as *const (), Throws::Caught), // sub_140b96490
    Replacement::draft_throws(0x140b96570, "content_140b96570", fn_140b96570 as *const (), Throws::Caught), // sub_140b96570
    Replacement::draft(0x140b96650, "content_140b96650", fn_140b96650 as *const ()), // sub_140b96650
    Replacement::draft(0x140b966c0, "content_140b966c0", tpl_7947::<0x14387e9f0, 0x14387e9d0, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140b966c0
    Replacement::draft(0x140b96730, "content_140b96730", tpl_11823::<0x14387e9d0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b96730
    Replacement::draft_throws(0x140b967b0, "content_140b967b0", tpl_7949::<0x14387e9f0, 0x14387e9d0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b967b0
    Replacement::draft_throws(0x140b96830, "content_140b96830", fn_140b96830 as *const (), Throws::Discard), // sub_140b96830
    Replacement::draft(0x140b96930, "content_140b96930", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143084cd0> as *const ()), // sub_140b96930
    Replacement::draft(0x140b96990, "content_140b96990", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143084cd0> as *const ()), // sub_140b96990
    Replacement::draft_throws(0x140b96a90, "content_140b96a90", fn_140b96a90 as *const (), Throws::Caught), // sub_140b96a90
    Replacement::draft_throws(0x140b96b20, "content_140b96b20", tpl_6762::<0x14387edb0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b96b20
    Replacement::draft(0x140b96bd0, "content_140b96bd0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432cb560, 0x8> as *const ()), // sub_140b96bd0
    Replacement::draft(0x140b96c50, "content_140b96c50", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cb560> as *const ()), // sub_140b96c50
    Replacement::draft(0x140b96d50, "content_140b96d50", tpl_12161::<0x14387f0a8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b96d50
    Replacement::draft_throws(0x140b96e00, "content_140b96e00", fn_140b96e00 as *const (), Throws::Discard), // sub_140b96e00
    Replacement::draft(0x140b96e90, "content_140b96e90", tpl_9969::<0x14387f0a8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b96e90
    Replacement::draft_throws(0x140b96f10, "content_140b96f10", fn_140b96f10 as *const (), Throws::Discard), // sub_140b96f10
    Replacement::draft_throws(0x140b96fa0, "content_140b96fa0", fn_140b96fa0 as *const (), Throws::Caught), // sub_140b96fa0
    Replacement::draft_throws(0x140b971a0, "content_140b971a0", fn_140b971a0 as *const (), Throws::Caught), // sub_140b971a0
    Replacement::draft_throws(0x140b97370, "content_140b97370", fn_140b97370 as *const (), Throws::Caught), // sub_140b97370
    Replacement::draft(0x140b97530, "content_140b97530", fn_140b97530 as *const ()), // sub_140b97530
    Replacement::draft(0x140b975a0, "content_140b975a0", tpl_8074::<0x14387ea68, 0x14387f0a8, 0x1401e9970, 0x1406e1480, 0x8, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_140b975a0
    Replacement::draft(0x140b97620, "content_140b97620", tpl_13937::<0x14387f0a8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b97620
    Replacement::draft_throws(0x140b976b0, "content_140b976b0", tpl_7974::<0x14387ea68, 0x14387f0a8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b976b0
    Replacement::draft_throws(0x140b97730, "content_140b97730", fn_140b97730 as *const (), Throws::Discard), // sub_140b97730
    Replacement::draft(0x140b97810, "content_140b97810", fn_140b97810 as *const ()), // sub_140b97810
    Replacement::draft(0x140b978d0, "content_140b978d0", fn_140b978d0 as *const ()), // sub_140b978d0
    Replacement::draft(0x140b979b0, "content_140b979b0", tpl_6766::<0x14387ee98, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b979b0
    Replacement::draft(0x140b97a20, "content_140b97a20", fn_140b97a20 as *const ()), // sub_140b97a20
    Replacement::draft_throws(0x140b97be0, "content_140b97be0", fn_140b97be0 as *const (), Throws::Discard), // sub_140b97be0
    Replacement::draft(0x140b97d20, "content_140b97d20", fn_140b97d20 as *const ()), // sub_140b97d20
    Replacement::draft_throws(0x140b97e10, "content_140b97e10", fn_140b97e10 as *const (), Throws::Discard), // sub_140b97e10
    Replacement::draft(0x140b97f60, "content_140b97f60", fn_140b97f60 as *const ()), // sub_140b97f60
    Replacement::draft_throws(0x140b97ff0, "content_140b97ff0", fn_140b97ff0 as *const (), Throws::Caught), // sub_140b97ff0
    Replacement::draft_throws(0x140b982b0, "content_140b982b0", fn_140b982b0 as *const (), Throws::Caught), // sub_140b982b0
    Replacement::draft_throws(0x140b98550, "content_140b98550", fn_140b98550 as *const (), Throws::Caught), // sub_140b98550
    Replacement::draft(0x140b987e0, "content_140b987e0", fn_140b987e0 as *const ()), // sub_140b987e0
    Replacement::draft(0x140b988b0, "content_140b988b0", fn_140b988b0 as *const ()), // sub_140b988b0
    Replacement::draft(0x140b989a0, "content_140b989a0", fn_140b989a0 as *const ()), // sub_140b989a0
    Replacement::draft_throws(0x140b98a90, "content_140b98a90", fn_140b98a90 as *const (), Throws::Discard), // sub_140b98a90
    Replacement::draft_throws(0x140b98b70, "content_140b98b70", fn_140b98b70 as *const (), Throws::Discard), // sub_140b98b70
    Replacement::draft(0x140b98cd0, "content_140b98cd0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432cff30> as *const ()), // sub_140b98cd0
    Replacement::draft(0x140b98d30, "content_140b98d30", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cff30> as *const ()), // sub_140b98d30
    Replacement::draft_throws(0x140b98e30, "content_140b98e30", fn_140b98e30 as *const (), Throws::Caught), // sub_140b98e30
    Replacement::draft_throws(0x140b98ec0, "content_140b98ec0", tpl_6763::<0x14387f0e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b98ec0
    Replacement::draft(0x140b98f70, "content_140b98f70", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143128510, 0x8> as *const ()), // sub_140b98f70
    Replacement::draft(0x140b98ff0, "content_140b98ff0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143128510> as *const ()), // sub_140b98ff0
    Replacement::draft(0x140b990f0, "content_140b990f0", tpl_11667::<0x14387e948, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b990f0
    Replacement::draft_throws(0x140b99190, "content_140b99190", tpl_11668::<0x14387ede0, 0x14387e948, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b99190
    Replacement::draft(0x140b99210, "content_140b99210", tpl_11361::<0x14387e948, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b99210
    Replacement::draft_throws(0x140b99290, "content_140b99290", tpl_11669::<0x14387ede0, 0x14387e948, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b99290
    Replacement::draft_throws(0x140b99320, "content_140b99320", fn_140b99320 as *const (), Throws::Caught), // sub_140b99320
    Replacement::draft_throws(0x140b99400, "content_140b99400", fn_140b99400 as *const (), Throws::Caught), // sub_140b99400
    Replacement::draft_throws(0x140b994e0, "content_140b994e0", fn_140b994e0 as *const (), Throws::Caught), // sub_140b994e0
    Replacement::draft(0x140b995c0, "content_140b995c0", tpl_11673::<0x14387e948, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b995c0
    Replacement::draft(0x140b99630, "content_140b99630", tpl_9203::<0x14387ede0, 0x14387e948, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xe8, 0x1401e9860> as *const ()), // sub_140b99630
    Replacement::draft(0x140b996a0, "content_140b996a0", tpl_11361::<0x14387e948, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b996a0
    Replacement::draft_throws(0x140b99720, "content_140b99720", tpl_9073::<0x14387ede0, 0x14387e948, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140b99720
    Replacement::draft_throws(0x140b99790, "content_140b99790", fn_140b99790 as *const (), Throws::Discard), // sub_140b99790
    Replacement::draft(0x140b99890, "content_140b99890", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143174260> as *const ()), // sub_140b99890
    Replacement::draft(0x140b998f0, "content_140b998f0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143174260> as *const ()), // sub_140b998f0
    Replacement::draft_throws(0x140b999f0, "content_140b999f0", fn_140b999f0 as *const (), Throws::Caught), // sub_140b999f0
    Replacement::draft_throws(0x140b99a80, "content_140b99a80", tpl_6763::<0x14387ed10, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b99a80
    Replacement::draft(0x140b99b30, "content_140b99b30", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431280a0> as *const ()), // sub_140b99b30
    Replacement::draft(0x140b99b90, "content_140b99b90", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431280a0> as *const ()), // sub_140b99b90
    Replacement::draft_throws(0x140b99c90, "content_140b99c90", fn_140b99c90 as *const (), Throws::Caught), // sub_140b99c90
    Replacement::draft_throws(0x140b99d20, "content_140b99d20", tpl_6763::<0x14387f038, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b99d20
    Replacement::draft(0x140b99dd0, "content_140b99dd0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143127f48> as *const ()), // sub_140b99dd0
    Replacement::draft(0x140b99e30, "content_140b99e30", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143127f48> as *const ()), // sub_140b99e30
    Replacement::draft_throws(0x140b99f30, "content_140b99f30", fn_140b99f30 as *const (), Throws::Caught), // sub_140b99f30
    Replacement::draft_throws(0x140b99fc0, "content_140b99fc0", tpl_6763::<0x14387eaa0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b99fc0
    Replacement::draft(0x140b9a070, "content_140b9a070", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143126360> as *const ()), // sub_140b9a070
    Replacement::draft(0x140b9a0d0, "content_140b9a0d0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143126360> as *const ()), // sub_140b9a0d0
    Replacement::draft(0x140b9a160, "content_140b9a160", fn_140b9a160 as *const ()), // sub_140b9a160
    Replacement::draft_throws(0x140b9a270, "content_140b9a270", tpl_14536::<0x14387ea00, 0x14387ea28, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140b9a270
    Replacement::draft_throws(0x140b9a370, "content_140b9a370", tpl_14536::<0x14387ea00, 0x14387ea28, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140b9a370
    Replacement::draft_throws(0x140b9a470, "content_140b9a470", fn_140b9a470 as *const (), Throws::Caught), // sub_140b9a470
    Replacement::draft(0x140b9a500, "content_140b9a500", fn_140b9a500 as *const ()), // sub_140b9a500
    Replacement::draft_throws(0x140b9a600, "content_140b9a600", tpl_14536::<0x14387ea00, 0x14387ea28, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // sub_140b9a600
    Replacement::draft_throws(0x140b9a700, "content_140b9a700", tpl_6737::<0x14387ea28, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b9a700
    Replacement::draft(0x140b9a7b0, "content_140b9a7b0", tpl_14189::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1431223a0> as *const ()), // sub_140b9a7b0
    Replacement::draft(0x140b9a830, "content_140b9a830", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431223a0, 0x18, 0x20> as *const ()), // sub_140b9a830
    Replacement::draft(0x140b9a8d0, "content_140b9a8d0", fn_140b9a8d0 as *const ()), // sub_140b9a8d0
    Replacement::draft(0x140b9aa50, "content_140b9aa50", fn_140b9aa50 as *const ()), // sub_140b9aa50
    Replacement::draft_throws(0x140b9ad00, "content_140b9ad00", fn_140b9ad00 as *const (), Throws::Caught), // sub_140b9ad00
    Replacement::draft(0x140b9af90, "content_140b9af90", fn_140b9af90 as *const ()), // sub_140b9af90
    Replacement::draft_throws(0x140b9b1b0, "content_140b9b1b0", fn_140b9b1b0 as *const (), Throws::Discard), // sub_140b9b1b0
    Replacement::draft_throws(0x140b9b420, "content_140b9b420", fn_140b9b420 as *const (), Throws::Caught), // sub_140b9b420
    Replacement::draft_throws(0x140b9b770, "content_140b9b770", fn_140b9b770 as *const (), Throws::Caught), // sub_140b9b770
    Replacement::draft_throws(0x140b9ba90, "content_140b9ba90", fn_140b9ba90 as *const (), Throws::Caught), // sub_140b9ba90
    Replacement::draft(0x140b9bdb0, "content_140b9bdb0", fn_140b9bdb0 as *const ()), // sub_140b9bdb0
    Replacement::draft(0x140b9c010, "content_140b9c010", fn_140b9c010 as *const ()), // sub_140b9c010
    Replacement::draft(0x140b9c210, "content_140b9c210", fn_140b9c210 as *const ()), // sub_140b9c210
    Replacement::draft_throws(0x140b9c420, "content_140b9c420", fn_140b9c420 as *const (), Throws::Discard), // sub_140b9c420
    Replacement::draft_throws(0x140b9c640, "content_140b9c640", fn_140b9c640 as *const (), Throws::Discard), // sub_140b9c640
    Replacement::draft(0x140b9c860, "content_140b9c860", fn_140b9c860 as *const ()), // sub_140b9c860
    Replacement::draft(0x140b9c9a0, "content_140b9c9a0", fn_140b9c9a0 as *const ()), // sub_140b9c9a0
    Replacement::draft(0x140b9ca50, "content_140b9ca50", fn_140b9ca50 as *const ()), // sub_140b9ca50
    Replacement::draft(0x140b9cb10, "content_140b9cb10", tpl_6766::<0x14387e9c0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140b9cb10
    Replacement::draft(0x140b9cb80, "content_140b9cb80", fn_140b9cb80 as *const ()), // sub_140b9cb80
    Replacement::draft_throws(0x140b9cd30, "content_140b9cd30", fn_140b9cd30 as *const (), Throws::Discard), // sub_140b9cd30
    Replacement::draft(0x140b9ced0, "content_140b9ced0", fn_140b9ced0 as *const ()), // sub_140b9ced0
    Replacement::draft_throws(0x140b9d000, "content_140b9d000", fn_140b9d000 as *const (), Throws::Discard), // sub_140b9d000
    Replacement::draft_throws(0x140b9d190, "content_140b9d190", fn_140b9d190 as *const (), Throws::Caught), // sub_140b9d190
    Replacement::draft_throws(0x140b9d4d0, "content_140b9d4d0", fn_140b9d4d0 as *const (), Throws::Caught), // sub_140b9d4d0
    Replacement::draft_throws(0x140b9d7e0, "content_140b9d7e0", fn_140b9d7e0 as *const (), Throws::Caught), // sub_140b9d7e0
    Replacement::draft(0x140b9daf0, "content_140b9daf0", fn_140b9daf0 as *const ()), // sub_140b9daf0
    Replacement::draft(0x140b9db90, "content_140b9db90", fn_140b9db90 as *const ()), // sub_140b9db90
    Replacement::draft(0x140b9dcd0, "content_140b9dcd0", fn_140b9dcd0 as *const ()), // sub_140b9dcd0
    Replacement::draft_throws(0x140b9de20, "content_140b9de20", fn_140b9de20 as *const (), Throws::Discard), // sub_140b9de20
    Replacement::draft_throws(0x140b9df20, "content_140b9df20", fn_140b9df20 as *const (), Throws::Caught), // sub_140b9df20
    Replacement::draft(0x140b9e1e0, "content_140b9e1e0", fn_140b9e1e0 as *const ()), // sub_140b9e1e0
    Replacement::draft_throws(0x140b9e280, "content_140b9e280", fn_140b9e280 as *const (), Throws::Discard), // sub_140b9e280
    Replacement::draft(0x140b9e380, "content_140b9e380", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14313e820> as *const ()), // sub_140b9e380
    Replacement::draft(0x140b9e3e0, "content_140b9e3e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14313e820> as *const ()), // sub_140b9e3e0
    Replacement::draft_throws(0x140b9e4e0, "content_140b9e4e0", fn_140b9e4e0 as *const (), Throws::Caught), // sub_140b9e4e0
    Replacement::draft_throws(0x140b9e570, "content_140b9e570", tpl_6762::<0x14387e9e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140b9e570
    Replacement::draft(0x140b9e620, "content_140b9e620", fn_140b9e620 as *const ()), // sub_140b9e620
    Replacement::draft(0x140b9e6d0, "content_140b9e6d0", fn_140b9e6d0 as *const ()), // sub_140b9e6d0
    Replacement::draft(0x140b9e800, "content_140b9e800", fn_140b9e800 as *const ()), // sub_140b9e800
    Replacement::draft_throws(0x140b9e8f0, "content_140b9e8f0", fn_140b9e8f0 as *const (), Throws::Discard), // sub_140b9e8f0
    Replacement::draft(0x140b9ea00, "content_140b9ea00", fn_140b9ea00 as *const ()), // sub_140b9ea00
    Replacement::draft_throws(0x140b9eab0, "content_140b9eab0", fn_140b9eab0 as *const (), Throws::Discard), // sub_140b9eab0
    Replacement::draft_throws(0x140b9ebc0, "content_140b9ebc0", fn_140b9ebc0 as *const (), Throws::Caught), // sub_140b9ebc0
    Replacement::draft_throws(0x140b9ef80, "content_140b9ef80", fn_140b9ef80 as *const (), Throws::Caught), // sub_140b9ef80
    Replacement::draft_throws(0x140b9f2e0, "content_140b9f2e0", fn_140b9f2e0 as *const (), Throws::Caught), // sub_140b9f2e0
    Replacement::draft(0x140b9f610, "content_140b9f610", fn_140b9f610 as *const ()), // sub_140b9f610
    Replacement::draft(0x140b9f6c0, "content_140b9f6c0", fn_140b9f6c0 as *const ()), // sub_140b9f6c0
    Replacement::draft(0x140b9f770, "content_140b9f770", fn_140b9f770 as *const ()), // sub_140b9f770
    Replacement::draft_throws(0x140b9f830, "content_140b9f830", fn_140b9f830 as *const (), Throws::Discard), // sub_140b9f830
    Replacement::draft_throws(0x140b9f8c0, "content_140b9f8c0", fn_140b9f8c0 as *const (), Throws::Discard), // sub_140b9f8c0
    Replacement::draft_throws(0x140b9fa00, "content_140b9fa00", tpl_14580::<0x1434268d0, 0x1432bb8d8, 0x30, 0x1432bbb90, 0x1432bbb80, 0x1402c4e80> as *const (), Throws::Discard), // sub_140b9fa00
    Replacement::draft(0x140b9fa70, "content_140b9fa70", fn_140b9fa70 as *const ()), // sub_140b9fa70
    Replacement::draft(0x140b9fb70, "content_140b9fb70", fn_140b9fb70 as *const ()), // sub_140b9fb70
    Replacement::draft(0x140b9fca0, "content_140b9fca0", fn_140b9fca0 as *const ()), // sub_140b9fca0
    Replacement::draft(0x140b9fd20, "content_140b9fd20", fn_140b9fd20 as *const ()), // sub_140b9fd20
    Replacement::draft_throws(0x140b9fed0, "content_140b9fed0", fn_140b9fed0 as *const (), Throws::Discard), // sub_140b9fed0
    Replacement::draft(0x140ba0200, "content_140ba0200", fn_140ba0200 as *const ()), // sub_140ba0200
    Replacement::draft_throws(0x140ba0440, "content_140ba0440", fn_140ba0440 as *const (), Throws::Discard), // sub_140ba0440
    Replacement::draft(0x140ba0780, "content_140ba0780", fn_140ba0780 as *const ()), // sub_140ba0780
    Replacement::draft_throws(0x140ba0800, "content_140ba0800", fn_140ba0800 as *const (), Throws::Caught), // sub_140ba0800
    Replacement::draft_throws(0x140ba0ed0, "content_140ba0ed0", fn_140ba0ed0 as *const (), Throws::Caught), // sub_140ba0ed0
    Replacement::draft_throws(0x140ba14f0, "content_140ba14f0", fn_140ba14f0 as *const (), Throws::Caught), // sub_140ba14f0
    Replacement::draft(0x140ba1af0, "content_140ba1af0", fn_140ba1af0 as *const ()), // sub_140ba1af0
    Replacement::draft(0x140ba1bf0, "content_140ba1bf0", fn_140ba1bf0 as *const ()), // sub_140ba1bf0
    Replacement::draft(0x140ba1cd0, "content_140ba1cd0", fn_140ba1cd0 as *const ()), // sub_140ba1cd0
    Replacement::draft_throws(0x140ba1dc0, "content_140ba1dc0", fn_140ba1dc0 as *const (), Throws::Discard), // sub_140ba1dc0
    Replacement::draft_throws(0x140ba1ea0, "content_140ba1ea0", fn_140ba1ea0 as *const (), Throws::Discard), // sub_140ba1ea0
    Replacement::draft(0x140ba2090, "content_140ba2090", fn_140ba2090 as *const ()), // sub_140ba2090
    Replacement::draft(0x140ba21d0, "content_140ba21d0", fn_140ba21d0 as *const ()), // sub_140ba21d0
    Replacement::draft_throws(0x140ba2310, "content_140ba2310", fn_140ba2310 as *const (), Throws::Discard), // sub_140ba2310
    Replacement::draft(0x140ba2470, "content_140ba2470", tpl_14593::<0x14387ef78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x50, 0x98, 0x38, 0x9c, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140ba2470
    Replacement::draft_throws(0x140ba2530, "content_140ba2530", fn_140ba2530 as *const (), Throws::Discard), // sub_140ba2530
    Replacement::draft_throws(0x140ba2680, "content_140ba2680", fn_140ba2680 as *const (), Throws::Caught), // sub_140ba2680
    Replacement::draft_throws(0x140ba28d0, "content_140ba28d0", fn_140ba28d0 as *const (), Throws::Caught), // sub_140ba28d0
    Replacement::draft_throws(0x140ba2af0, "content_140ba2af0", fn_140ba2af0 as *const (), Throws::Caught), // sub_140ba2af0
    Replacement::draft(0x140ba2f70, "content_140ba2f70", fn_140ba2f70 as *const ()), // sub_140ba2f70
    Replacement::draft(0x140ba3050, "content_140ba3050", fn_140ba3050 as *const ()), // sub_140ba3050
    Replacement::draft(0x140ba3110, "content_140ba3110", tpl_14593::<0x14387ef78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x50, 0x98, 0xd8, 0x9c, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140ba3110
    Replacement::draft_throws(0x140ba31e0, "content_140ba31e0", fn_140ba31e0 as *const (), Throws::Discard), // sub_140ba31e0
    Replacement::draft_throws(0x140ba3280, "content_140ba3280", fn_140ba3280 as *const (), Throws::Discard), // sub_140ba3280
    Replacement::draft(0x140ba33f0, "content_140ba33f0", fn_140ba33f0 as *const ()), // sub_140ba33f0
    Replacement::draft(0x140ba3540, "content_140ba3540", fn_140ba3540 as *const ()), // sub_140ba3540
    Replacement::draft_throws(0x140ba3650, "content_140ba3650", fn_140ba3650 as *const (), Throws::Discard), // sub_140ba3650
    Replacement::draft(0x140ba3770, "content_140ba3770", fn_140ba3770 as *const ()), // sub_140ba3770
    Replacement::draft_throws(0x140ba3810, "content_140ba3810", fn_140ba3810 as *const (), Throws::Discard), // sub_140ba3810
    Replacement::draft_throws(0x140ba3930, "content_140ba3930", fn_140ba3930 as *const (), Throws::Caught), // sub_140ba3930
    Replacement::draft_throws(0x140ba3a30, "content_140ba3a30", fn_140ba3a30 as *const (), Throws::Caught), // sub_140ba3a30
    Replacement::draft_throws(0x140ba3b30, "content_140ba3b30", fn_140ba3b30 as *const (), Throws::Caught), // sub_140ba3b30
    Replacement::draft(0x140ba3ee0, "content_140ba3ee0", fn_140ba3ee0 as *const ()), // sub_140ba3ee0
    Replacement::draft(0x140ba4000, "content_140ba4000", fn_140ba4000 as *const ()), // sub_140ba4000
    Replacement::draft(0x140ba40d0, "content_140ba40d0", fn_140ba40d0 as *const ()), // sub_140ba40d0
    Replacement::draft_throws(0x140ba41b0, "content_140ba41b0", fn_140ba41b0 as *const (), Throws::Discard), // sub_140ba41b0
    Replacement::draft_throws(0x140ba4290, "content_140ba4290", fn_140ba4290 as *const (), Throws::Discard), // sub_140ba4290
    Replacement::draft(0x140ba43f0, "content_140ba43f0", fn_140ba43f0 as *const ()), // sub_140ba43f0
    Replacement::draft(0x140ba44b0, "content_140ba44b0", tpl_7312::<0x14387ecf8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ba44b0
    Replacement::draft(0x140ba4520, "content_140ba4520", fn_140ba4520 as *const ()), // sub_140ba4520
    Replacement::draft_throws(0x140ba4620, "content_140ba4620", fn_140ba4620 as *const (), Throws::Discard), // sub_140ba4620
    Replacement::draft(0x140ba4750, "content_140ba4750", fn_140ba4750 as *const ()), // sub_140ba4750
    Replacement::draft_throws(0x140ba4820, "content_140ba4820", fn_140ba4820 as *const (), Throws::Discard), // sub_140ba4820
    Replacement::draft_throws(0x140ba4930, "content_140ba4930", fn_140ba4930 as *const (), Throws::Caught), // sub_140ba4930
    Replacement::draft_throws(0x140ba4b80, "content_140ba4b80", fn_140ba4b80 as *const (), Throws::Caught), // sub_140ba4b80
    Replacement::draft_throws(0x140ba4da0, "content_140ba4da0", fn_140ba4da0 as *const (), Throws::Caught), // sub_140ba4da0
    Replacement::draft(0x140ba5110, "content_140ba5110", fn_140ba5110 as *const ()), // sub_140ba5110
    Replacement::draft(0x140ba5210, "content_140ba5210", fn_140ba5210 as *const ()), // sub_140ba5210
    Replacement::draft(0x140ba5310, "content_140ba5310", fn_140ba5310 as *const ()), // sub_140ba5310
    Replacement::draft_throws(0x140ba5420, "content_140ba5420", fn_140ba5420 as *const (), Throws::Discard), // sub_140ba5420
    Replacement::draft_throws(0x140ba5510, "content_140ba5510", fn_140ba5510 as *const (), Throws::Discard), // sub_140ba5510
    Replacement::draft(0x140ba5670, "content_140ba5670", fn_140ba5670 as *const ()), // sub_140ba5670
    Replacement::draft(0x140ba5720, "content_140ba5720", fn_140ba5720 as *const ()), // sub_140ba5720
    Replacement::draft(0x140ba57f0, "content_140ba57f0", fn_140ba57f0 as *const ()), // sub_140ba57f0
    Replacement::draft(0x140ba5b70, "content_140ba5b70", fn_140ba5b70 as *const ()), // sub_140ba5b70
    Replacement::draft_throws(0x140ba5f50, "content_140ba5f50", fn_140ba5f50 as *const (), Throws::Caught), // sub_140ba5f50
    Replacement::draft(0x140ba6360, "content_140ba6360", fn_140ba6360 as *const ()), // sub_140ba6360
    Replacement::draft_throws(0x140ba6750, "content_140ba6750", fn_140ba6750 as *const (), Throws::Caught), // sub_140ba6750
    Replacement::draft(0x140ba6b70, "content_140ba6b70", fn_140ba6b70 as *const ()), // sub_140ba6b70
    Replacement::draft_throws(0x140ba6bf0, "content_140ba6bf0", fn_140ba6bf0 as *const (), Throws::Caught), // sub_140ba6bf0
    Replacement::draft_throws(0x140ba6e90, "content_140ba6e90", fn_140ba6e90 as *const (), Throws::Caught), // sub_140ba6e90
    Replacement::draft_throws(0x140ba70f0, "content_140ba70f0", fn_140ba70f0 as *const (), Throws::Caught), // sub_140ba70f0
    Replacement::draft(0x140ba7340, "content_140ba7340", fn_140ba7340 as *const ()), // sub_140ba7340
    Replacement::draft(0x140ba7700, "content_140ba7700", fn_140ba7700 as *const ()), // sub_140ba7700
    Replacement::draft(0x140ba7ae0, "content_140ba7ae0", fn_140ba7ae0 as *const ()), // sub_140ba7ae0
    Replacement::draft_throws(0x140ba7ed0, "content_140ba7ed0", fn_140ba7ed0 as *const (), Throws::Caught), // sub_140ba7ed0
    Replacement::draft_throws(0x140ba82a0, "content_140ba82a0", fn_140ba82a0 as *const (), Throws::Discard), // sub_140ba82a0
    Replacement::draft(0x140ba83d0, "content_140ba83d0", fn_140ba83d0 as *const ()), // sub_140ba83d0
    Replacement::draft(0x140ba8470, "content_140ba8470", fn_140ba8470 as *const ()), // sub_140ba8470
    Replacement::draft(0x140ba8520, "content_140ba8520", fn_140ba8520 as *const ()), // sub_140ba8520
    Replacement::draft(0x140ba94f0, "content_140ba94f0", fn_140ba94f0 as *const ()), // sub_140ba94f0
    Replacement::draft_throws(0x140baa730, "content_140baa730", fn_140baa730 as *const (), Throws::Caught), // sub_140baa730
    Replacement::draft(0x140baba60, "content_140baba60", fn_140baba60 as *const ()), // sub_140baba60
    Replacement::draft_throws(0x140bacbd0, "content_140bacbd0", fn_140bacbd0 as *const (), Throws::Discard), // sub_140bacbd0
    Replacement::draft(0x140badd80, "content_140badd80", fn_140badd80 as *const ()), // sub_140badd80
    Replacement::draft_throws(0x140bade50, "content_140bade50", fn_140bade50 as *const (), Throws::Caught), // sub_140bade50
    Replacement::draft_throws(0x140bae750, "content_140bae750", fn_140bae750 as *const (), Throws::Caught), // sub_140bae750
    Replacement::draft_throws(0x140baef30, "content_140baef30", fn_140baef30 as *const (), Throws::Caught), // sub_140baef30
    Replacement::draft(0x140baf6c0, "content_140baf6c0", fn_140baf6c0 as *const ()), // sub_140baf6c0
    Replacement::draft(0x140bb09e0, "content_140bb09e0", fn_140bb09e0 as *const ()), // sub_140bb09e0
    Replacement::draft(0x140bb1b10, "content_140bb1b10", fn_140bb1b10 as *const ()), // sub_140bb1b10
    Replacement::draft_throws(0x140bb2c50, "content_140bb2c50", fn_140bb2c50 as *const (), Throws::Discard), // sub_140bb2c50
    Replacement::draft_throws(0x140bb3dc0, "content_140bb3dc0", fn_140bb3dc0 as *const (), Throws::Discard), // sub_140bb3dc0
    Replacement::draft(0x140bb41d0, "content_140bb41d0", fn_140bb41d0 as *const ()), // sub_140bb41d0
    Replacement::draft(0x140bb4410, "content_140bb4410", fn_140bb4410 as *const ()), // sub_140bb4410
    Replacement::draft(0x140bb44c0, "content_140bb44c0", fn_140bb44c0 as *const ()), // sub_140bb44c0
    Replacement::draft(0x140bb4580, "content_140bb4580", tpl_11372::<0x14387eef8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1407de100, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb4580
    Replacement::draft(0x140bb4600, "content_140bb4600", tpl_11372::<0x14387eef8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1407de180, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb4600
    Replacement::draft_throws(0x140bb4680, "content_140bb4680", tpl_6799::<0x14387ebac, 0x14387eef8, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140c05920, 0x1401e9e70> as *const (), Throws::Discard), // sub_140bb4680
    Replacement::draft(0x140bb46f0, "content_140bb46f0", tpl_11372::<0x14387eef8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1407de580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb46f0
    Replacement::draft_throws(0x140bb4770, "content_140bb4770", tpl_6799::<0x14387ebac, 0x14387eef8, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140c05a50, 0x1401e9a20> as *const (), Throws::Discard), // sub_140bb4770
    Replacement::draft(0x140bb47e0, "content_140bb47e0", fn_140bb47e0 as *const ()), // sub_140bb47e0
    Replacement::draft_throws(0x140bb4860, "content_140bb4860", fn_140bb4860 as *const (), Throws::Caught), // sub_140bb4860
    Replacement::draft_throws(0x140bb4920, "content_140bb4920", fn_140bb4920 as *const (), Throws::Caught), // sub_140bb4920
    Replacement::draft_throws(0x140bb49e0, "content_140bb49e0", fn_140bb49e0 as *const (), Throws::Caught), // sub_140bb49e0
    Replacement::draft(0x140bb4aa0, "content_140bb4aa0", tpl_11372::<0x14387eef8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1407df4b0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb4aa0
    Replacement::draft(0x140bb4b20, "content_140bb4b20", tpl_6804::<0x14387ebac, 0x14387eef8, 0x1401e9970, 0x1406e1480, 0x8, 0x1407df670, 0x1401e9860> as *const ()), // sub_140bb4b20
    Replacement::draft(0x140bb4b90, "content_140bb4b90", tpl_11372::<0x14387eef8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1407df7d0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb4b90
    Replacement::draft_throws(0x140bb4c10, "content_140bb4c10", tpl_6805::<0x14387ebac, 0x14387eef8, 0x1401e9770, 0x1406e1570, 0x8, 0x1407df940, 0x1401e9590> as *const (), Throws::Discard), // sub_140bb4c10
    Replacement::draft_throws(0x140bb4c80, "content_140bb4c80", fn_140bb4c80 as *const (), Throws::Caught), // sub_140bb4c80
    Replacement::draft(0x140bb4da0, "content_140bb4da0", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430f6be8, 0x8> as *const ()), // sub_140bb4da0
    Replacement::draft(0x140bb4e30, "content_140bb4e30", tpl_11905::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430f6be8, 0x18> as *const ()), // sub_140bb4e30
    Replacement::draft(0x140bb4f50, "content_140bb4f50", fn_140bb4f50 as *const ()), // sub_140bb4f50
    Replacement::draft_throws(0x140bb5010, "content_140bb5010", fn_140bb5010 as *const (), Throws::Discard), // sub_140bb5010
    Replacement::draft(0x140bb50b0, "content_140bb50b0", tpl_14653::<0x14387e900, 0x14243b0b0, 0x1406e01b0, 0x10, 0x140555cc0, 0x338, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb50b0
    Replacement::draft_throws(0x140bb5140, "content_140bb5140", fn_140bb5140 as *const (), Throws::Discard), // sub_140bb5140
    Replacement::draft_throws(0x140bb51e0, "content_140bb51e0", fn_140bb51e0 as *const (), Throws::Caught), // sub_140bb51e0
    Replacement::draft_throws(0x140bb5310, "content_140bb5310", fn_140bb5310 as *const (), Throws::Caught), // sub_140bb5310
    Replacement::draft_throws(0x140bb5440, "content_140bb5440", fn_140bb5440 as *const (), Throws::Caught), // sub_140bb5440
    Replacement::draft(0x140bb5570, "content_140bb5570", fn_140bb5570 as *const ()), // sub_140bb5570
    Replacement::draft(0x140bb55f0, "content_140bb55f0", tpl_70397::<0x14387ecd8, 0x14387e900, 0x1401e9970, 0x1406e1480, 0x10, 0x140555fb0, 0x338, 0x8, 0xd8, 0x1401e9860> as *const ()), // sub_140bb55f0
    Replacement::draft(0x140bb5670, "content_140bb5670", tpl_14653::<0x14387e900, 0x14243b0b0, 0x1406e01b0, 0x10, 0x140556120, 0x338, 0x8, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb5670
    Replacement::draft_throws(0x140bb5700, "content_140bb5700", fn_140bb5700 as *const (), Throws::Discard), // sub_140bb5700
    Replacement::draft_throws(0x140bb5790, "content_140bb5790", fn_140bb5790 as *const (), Throws::Discard), // sub_140bb5790
    Replacement::draft(0x140bb58a0, "content_140bb58a0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430f6988, 0x8> as *const ()), // sub_140bb58a0
    Replacement::draft(0x140bb5920, "content_140bb5920", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430f6988> as *const ()), // sub_140bb5920
    Replacement::draft(0x140bb5a20, "content_140bb5a20", tpl_13874::<0x14387eeb0, 0x14243b0b0, 0x1406e01b0, 0x8, 0x1426d4580, 0x338, 0x1, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb5a20
    Replacement::draft_throws(0x140bb5ae0, "content_140bb5ae0", fn_140bb5ae0 as *const (), Throws::Caught), // sub_140bb5ae0
    Replacement::draft(0x140bb5bb0, "content_140bb5bb0", tpl_13876::<0x14387eeb0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x20, 0x40, 0x8, 0x14240ab10, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140bb5bb0
    Replacement::draft_throws(0x140bb5c70, "content_140bb5c70", tpl_7839::<0x14387ef08, 0x14387eeb0, 0x1401e9b60, 0x1406e03a0, 0x1426cdcb0, 0x8, 0x1402c1540, 0x1401e9a20> as *const (), Throws::Discard), // sub_140bb5c70
    Replacement::draft_throws(0x140bb5cf0, "content_140bb5cf0", fn_140bb5cf0 as *const (), Throws::Caught), // sub_140bb5cf0
    Replacement::draft_throws(0x140bb5ee0, "content_140bb5ee0", fn_140bb5ee0 as *const (), Throws::Caught), // sub_140bb5ee0
    Replacement::draft_throws(0x140bb60a0, "content_140bb60a0", fn_140bb60a0 as *const (), Throws::Caught), // sub_140bb60a0
    Replacement::draft(0x140bb6260, "content_140bb6260", tpl_13880::<0x14387eeb0, 0x14243b0b0, 0x1406e01b0, 0x1426cdcb0, 0x142409b50, 0x14240ab10> as *const ()), // sub_140bb6260
    Replacement::draft(0x140bb6390, "content_140bb6390", tpl_7871::<0x14387ef08, 0x14387eeb0, 0x1401e9970, 0x1406e1480, 0x338, 0x1426cdcb0, 0x8, 0xa0, 0xe0, 0x1401e9860> as *const ()), // sub_140bb6390
    Replacement::draft(0x140bb6430, "content_140bb6430", tpl_13881::<0x14387eeb0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426cdcb0, 0x8, 0xa0, 0xe0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb6430
    Replacement::draft_throws(0x140bb64d0, "content_140bb64d0", tpl_7873::<0x14387ef08, 0x14387eeb0, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140bb64d0
    Replacement::draft_throws(0x140bb6560, "content_140bb6560", tpl_13882::<0x14387eeb0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1426cdcb0, 0x1432bd220, 0x1432bd198, 0x1432bd180, 0x1402bf8c0> as *const (), Throws::Discard), // sub_140bb6560
    Replacement::draft(0x140bb6650, "content_140bb6650", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430f6638, 0x8> as *const ()), // sub_140bb6650
    Replacement::draft(0x140bb66e0, "content_140bb66e0", tpl_6934::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430f6638, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140bb66e0
    Replacement::draft(0x140bb6790, "content_140bb6790", fn_140bb6790 as *const ()), // sub_140bb6790
    Replacement::draft(0x140bb6840, "content_140bb6840", fn_140bb6840 as *const ()), // sub_140bb6840
    Replacement::draft_throws(0x140bb6940, "content_140bb6940", tpl_8772::<0x14387ef28, 0x14387ea98, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140c06df0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140bb6940
    Replacement::draft(0x140bb69b0, "content_140bb69b0", fn_140bb69b0 as *const ()), // sub_140bb69b0
    Replacement::draft_throws(0x140bb6a80, "content_140bb6a80", tpl_8772::<0x14387ef28, 0x14387ea98, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140c06fb0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140bb6a80
    Replacement::draft(0x140bb6af0, "content_140bb6af0", fn_140bb6af0 as *const ()), // sub_140bb6af0
    Replacement::draft_throws(0x140bb6b80, "content_140bb6b80", fn_140bb6b80 as *const (), Throws::Caught), // sub_140bb6b80
    Replacement::draft_throws(0x140bb6c50, "content_140bb6c50", fn_140bb6c50 as *const (), Throws::Caught), // sub_140bb6c50
    Replacement::draft_throws(0x140bb6d20, "content_140bb6d20", fn_140bb6d20 as *const (), Throws::Caught), // sub_140bb6d20
    Replacement::draft(0x140bb6df0, "content_140bb6df0", fn_140bb6df0 as *const ()), // sub_140bb6df0
    Replacement::draft(0x140bb6eb0, "content_140bb6eb0", fn_140bb6eb0 as *const ()), // sub_140bb6eb0
    Replacement::draft(0x140bb6f60, "content_140bb6f60", fn_140bb6f60 as *const ()), // sub_140bb6f60
    Replacement::draft_throws(0x140bb7030, "content_140bb7030", fn_140bb7030 as *const (), Throws::Discard), // sub_140bb7030
    Replacement::draft_throws(0x140bb70e0, "content_140bb70e0", tpl_14088::<0x14387ea98, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432bd208, 0x8, 0x1432bd1f8, 0x1432bd1e8, 0x140c07190> as *const (), Throws::Discard), // sub_140bb70e0
    Replacement::draft(0x140bb71d0, "content_140bb71d0", tpl_73097::<0x8, 0x10, 0x1426cdcb0, 0x140c28640> as *const ()), // sub_140bb71d0
    Replacement::draft(0x140bb7250, "content_140bb7250", tpl_73097::<0x8, 0x10, 0x1432bd1e8, 0x140c28650> as *const ()), // sub_140bb7250
    Replacement::draft(0x140bb72d0, "content_140bb72d0", fn_140bb72d0 as *const ()), // sub_140bb72d0
    Replacement::draft(0x140bb7370, "content_140bb7370", fn_140bb7370 as *const ()), // sub_140bb7370
    Replacement::draft(0x140bb7420, "content_140bb7420", fn_140bb7420 as *const ()), // sub_140bb7420
    Replacement::draft(0x140bb7500, "content_140bb7500", fn_140bb7500 as *const ()), // sub_140bb7500
    Replacement::draft_throws(0x140bb7740, "content_140bb7740", fn_140bb7740 as *const (), Throws::Caught), // sub_140bb7740
    Replacement::draft(0x140bb7940, "content_140bb7940", fn_140bb7940 as *const ()), // sub_140bb7940
    Replacement::draft_throws(0x140bb7b20, "content_140bb7b20", fn_140bb7b20 as *const (), Throws::Discard), // sub_140bb7b20
    Replacement::draft(0x140bb7c60, "content_140bb7c60", fn_140bb7c60 as *const ()), // sub_140bb7c60
    Replacement::draft_throws(0x140bb7dc0, "content_140bb7dc0", fn_140bb7dc0 as *const (), Throws::Caught), // sub_140bb7dc0
    Replacement::draft_throws(0x140bb8330, "content_140bb8330", fn_140bb8330 as *const (), Throws::Caught), // sub_140bb8330
    Replacement::draft_throws(0x140bb87e0, "content_140bb87e0", fn_140bb87e0 as *const (), Throws::Caught), // sub_140bb87e0
    Replacement::draft(0x140bb8c90, "content_140bb8c90", fn_140bb8c90 as *const ()), // sub_140bb8c90
    Replacement::draft(0x140bb8f80, "content_140bb8f80", fn_140bb8f80 as *const ()), // sub_140bb8f80
    Replacement::draft(0x140bb9100, "content_140bb9100", fn_140bb9100 as *const ()), // sub_140bb9100
    Replacement::draft_throws(0x140bb9290, "content_140bb9290", fn_140bb9290 as *const (), Throws::Discard), // sub_140bb9290
    Replacement::draft_throws(0x140bb9420, "content_140bb9420", fn_140bb9420 as *const (), Throws::Discard), // sub_140bb9420
    Replacement::draft(0x140bb9640, "content_140bb9640", fn_140bb9640 as *const ()), // sub_140bb9640
    Replacement::draft(0x140bb9700, "content_140bb9700", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432c37b8> as *const ()), // sub_140bb9700
    Replacement::draft(0x140bb9760, "content_140bb9760", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c37b8> as *const ()), // sub_140bb9760
    Replacement::draft_throws(0x140bb9860, "content_140bb9860", fn_140bb9860 as *const (), Throws::Caught), // sub_140bb9860
    Replacement::draft_throws(0x140bb98f0, "content_140bb98f0", tpl_6763::<0x14387ef10, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140bb98f0
    Replacement::draft(0x140bb99a0, "content_140bb99a0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430e82c0, 0x8> as *const ()), // sub_140bb99a0
    Replacement::draft(0x140bb9a20, "content_140bb9a20", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430e82c0> as *const ()), // sub_140bb9a20
    Replacement::draft(0x140bb9ab0, "content_140bb9ab0", tpl_6766::<0x14387ee48, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb9ab0
    Replacement::draft(0x140bb9b20, "content_140bb9b20", fn_140bb9b20 as *const ()), // sub_140bb9b20
    Replacement::draft_throws(0x140bb9bf0, "content_140bb9bf0", fn_140bb9bf0 as *const (), Throws::Discard), // sub_140bb9bf0
    Replacement::draft(0x140bb9cb0, "content_140bb9cb0", tpl_14682::<0x14387ee48, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bb9cb0
    Replacement::draft_throws(0x140bb9d40, "content_140bb9d40", fn_140bb9d40 as *const (), Throws::Discard), // sub_140bb9d40
    Replacement::draft_throws(0x140bb9e10, "content_140bb9e10", fn_140bb9e10 as *const (), Throws::Caught), // sub_140bb9e10
    Replacement::draft_throws(0x140bb9f30, "content_140bb9f30", fn_140bb9f30 as *const (), Throws::Caught), // sub_140bb9f30
    Replacement::draft_throws(0x140bba050, "content_140bba050", fn_140bba050 as *const (), Throws::Caught), // sub_140bba050
    Replacement::draft(0x140bba170, "content_140bba170", fn_140bba170 as *const ()), // sub_140bba170
    Replacement::draft(0x140bba1f0, "content_140bba1f0", fn_140bba1f0 as *const ()), // sub_140bba1f0
    Replacement::draft(0x140bba280, "content_140bba280", tpl_14682::<0x14387ee48, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bba280
    Replacement::draft_throws(0x140bba310, "content_140bba310", fn_140bba310 as *const (), Throws::Discard), // sub_140bba310
    Replacement::draft_throws(0x140bba3b0, "content_140bba3b0", fn_140bba3b0 as *const (), Throws::Discard), // sub_140bba3b0
    Replacement::draft(0x140bba4f0, "content_140bba4f0", fn_140bba4f0 as *const ()), // sub_140bba4f0
    Replacement::draft(0x140bba590, "content_140bba590", fn_140bba590 as *const ()), // sub_140bba590
    Replacement::draft(0x140bba670, "content_140bba670", fn_140bba670 as *const ()), // sub_140bba670
    Replacement::draft(0x140bbafe0, "content_140bbafe0", fn_140bbafe0 as *const ()), // sub_140bbafe0
    Replacement::draft_throws(0x140bbbda0, "content_140bbbda0", fn_140bbbda0 as *const (), Throws::Caught), // sub_140bbbda0
    Replacement::draft(0x140bbce10, "content_140bbce10", fn_140bbce10 as *const ()), // sub_140bbce10
    Replacement::draft_throws(0x140bbdaf0, "content_140bbdaf0", fn_140bbdaf0 as *const (), Throws::Discard), // sub_140bbdaf0
    Replacement::draft(0x140bbe710, "content_140bbe710", fn_140bbe710 as *const ()), // sub_140bbe710
    Replacement::draft_throws(0x140bbe900, "content_140bbe900", fn_140bbe900 as *const (), Throws::Caught), // sub_140bbe900
    Replacement::draft_throws(0x140bbf2c0, "content_140bbf2c0", fn_140bbf2c0 as *const (), Throws::Caught), // sub_140bbf2c0
    Replacement::draft_throws(0x140bbfb70, "content_140bbfb70", fn_140bbfb70 as *const (), Throws::Caught), // sub_140bbfb70
    Replacement::draft(0x140bc0400, "content_140bc0400", fn_140bc0400 as *const ()), // sub_140bc0400
    Replacement::draft(0x140bc1540, "content_140bc1540", fn_140bc1540 as *const ()), // sub_140bc1540
    Replacement::draft(0x140bc2180, "content_140bc2180", fn_140bc2180 as *const ()), // sub_140bc2180
    Replacement::draft_throws(0x140bc2dc0, "content_140bc2dc0", fn_140bc2dc0 as *const (), Throws::Discard), // sub_140bc2dc0
    Replacement::draft_throws(0x140bc3a40, "content_140bc3a40", fn_140bc3a40 as *const (), Throws::Discard), // sub_140bc3a40
    Replacement::draft(0x140bc3fb0, "content_140bc3fb0", tpl_14695::<0x58, 0x1426cdcb0, 0x140c2a200> as *const ()), // sub_140bc3fb0
    Replacement::draft(0x140bc4020, "content_140bc4020", fn_140bc4020 as *const ()), // sub_140bc4020
    Replacement::draft(0x140bc4250, "content_140bc4250", tpl_14695::<0x58, 0x1432bcb58, 0x140c2a240> as *const ()), // sub_140bc4250
    Replacement::draft(0x140bc42c0, "content_140bc42c0", fn_140bc42c0 as *const ()), // sub_140bc42c0
    Replacement::draft(0x140bc4370, "content_140bc4370", fn_140bc4370 as *const ()), // sub_140bc4370
    Replacement::draft(0x140bc4420, "content_140bc4420", tpl_6766::<0x14387eb30, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bc4420
    Replacement::draft(0x140bc4490, "content_140bc4490", fn_140bc4490 as *const ()), // sub_140bc4490
    Replacement::draft_throws(0x140bc4700, "content_140bc4700", fn_140bc4700 as *const (), Throws::Discard), // sub_140bc4700
    Replacement::draft(0x140bc4990, "content_140bc4990", fn_140bc4990 as *const ()), // sub_140bc4990
    Replacement::draft_throws(0x140bc4ab0, "content_140bc4ab0", fn_140bc4ab0 as *const (), Throws::Discard), // sub_140bc4ab0
    Replacement::draft(0x140bc4db0, "content_140bc4db0", fn_140bc4db0 as *const ()), // sub_140bc4db0
    Replacement::draft_throws(0x140bc4e80, "content_140bc4e80", fn_140bc4e80 as *const (), Throws::Caught), // sub_140bc4e80
    Replacement::draft_throws(0x140bc5210, "content_140bc5210", fn_140bc5210 as *const (), Throws::Caught), // sub_140bc5210
    Replacement::draft_throws(0x140bc55a0, "content_140bc55a0", fn_140bc55a0 as *const (), Throws::Caught), // sub_140bc55a0
    Replacement::draft(0x140bc5930, "content_140bc5930", fn_140bc5930 as *const ()), // sub_140bc5930
    Replacement::draft(0x140bc5a40, "content_140bc5a40", fn_140bc5a40 as *const ()), // sub_140bc5a40
    Replacement::draft(0x140bc5c10, "content_140bc5c10", fn_140bc5c10 as *const ()), // sub_140bc5c10
    Replacement::draft_throws(0x140bc5df0, "content_140bc5df0", fn_140bc5df0 as *const (), Throws::Discard), // sub_140bc5df0
    Replacement::draft_throws(0x140bc5f70, "content_140bc5f70", fn_140bc5f70 as *const (), Throws::Caught), // sub_140bc5f70
    Replacement::draft(0x140bc63b0, "content_140bc63b0", fn_140bc63b0 as *const ()), // sub_140bc63b0
    Replacement::draft(0x140bc6600, "content_140bc6600", tpl_14216::<0x30, 0x1432bc9b0, 0x1432bc998, 0x14088ba10, 0x1432bc960, 0x48, 0x1432bc618> as *const ()), // sub_140bc6600
    Replacement::draft(0x140bc6680, "content_140bc6680", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ce410> as *const ()), // sub_140bc6680
    Replacement::draft(0x140bc66e0, "content_140bc66e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ce410> as *const ()), // sub_140bc66e0
    Replacement::draft_throws(0x140bc67e0, "content_140bc67e0", fn_140bc67e0 as *const (), Throws::Caught), // sub_140bc67e0
    Replacement::draft_throws(0x140bc6870, "content_140bc6870", tpl_6763::<0x14387ec10, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140bc6870
    Replacement::draft(0x140bc6920, "content_140bc6920", tpl_14709::<0xa0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x28, 0x48, 0x68, 0x88, 0x1430b4dc8> as *const ()), // sub_140bc6920
    Replacement::draft(0x140bc69e0, "content_140bc69e0", tpl_14710::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430b4dc8, 0xfffffffffffffff4, 0x14, 0x34, 0x54, 0x74> as *const ()), // sub_140bc69e0
    Replacement::draft(0x140bc6ac0, "content_140bc6ac0", fn_140bc6ac0 as *const ()), // sub_140bc6ac0
    Replacement::draft(0x140bc6b60, "content_140bc6b60", fn_140bc6b60 as *const ()), // sub_140bc6b60
    Replacement::draft_throws(0x140bc6ce0, "content_140bc6ce0", fn_140bc6ce0 as *const (), Throws::Discard), // sub_140bc6ce0
    Replacement::draft(0x140bc6e30, "content_140bc6e30", tpl_14713::<0x14387eef0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x90, 0x38, 0x94, 0x318, 0x328, 0x320> as *const ()), // sub_140bc6e30
    Replacement::draft_throws(0x140bc6f10, "content_140bc6f10", fn_140bc6f10 as *const (), Throws::Discard), // sub_140bc6f10
    Replacement::draft_throws(0x140bc7080, "content_140bc7080", fn_140bc7080 as *const (), Throws::Caught), // sub_140bc7080
    Replacement::draft_throws(0x140bc73b0, "content_140bc73b0", fn_140bc73b0 as *const (), Throws::Caught), // sub_140bc73b0
    Replacement::draft_throws(0x140bc76b0, "content_140bc76b0", fn_140bc76b0 as *const (), Throws::Caught), // sub_140bc76b0
    Replacement::draft(0x140bc79b0, "content_140bc79b0", fn_140bc79b0 as *const ()), // sub_140bc79b0
    Replacement::draft(0x140bc7a90, "content_140bc7a90", fn_140bc7a90 as *const ()), // sub_140bc7a90
    Replacement::draft(0x140bc7b60, "content_140bc7b60", tpl_14713::<0x14387eef0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x90, 0xd8, 0x94, 0x318, 0x328, 0x320> as *const ()), // sub_140bc7b60
    Replacement::draft_throws(0x140bc7c40, "content_140bc7c40", fn_140bc7c40 as *const (), Throws::Discard), // sub_140bc7c40
    Replacement::draft_throws(0x140bc7d20, "content_140bc7d20", fn_140bc7d20 as *const (), Throws::Discard), // sub_140bc7d20
    Replacement::draft(0x140bc7f00, "content_140bc7f00", fn_140bc7f00 as *const ()), // sub_140bc7f00
    Replacement::draft(0x140bc7fe0, "content_140bc7fe0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430d36a8, 0x8> as *const ()), // sub_140bc7fe0
    Replacement::draft(0x140bc8070, "content_140bc8070", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1430d36a8, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140bc8070
    Replacement::draft(0x140bc8120, "content_140bc8120", fn_140bc8120 as *const ()), // sub_140bc8120
    Replacement::draft(0x140bc82a0, "content_140bc82a0", fn_140bc82a0 as *const ()), // sub_140bc82a0
    Replacement::draft_throws(0x140bc8440, "content_140bc8440", fn_140bc8440 as *const (), Throws::Discard), // sub_140bc8440
    Replacement::draft(0x140bc85a0, "content_140bc85a0", fn_140bc85a0 as *const ()), // sub_140bc85a0
    Replacement::draft_throws(0x140bc8710, "content_140bc8710", fn_140bc8710 as *const (), Throws::Discard), // sub_140bc8710
    Replacement::draft(0x140bc8880, "content_140bc8880", fn_140bc8880 as *const ()), // sub_140bc8880
    Replacement::draft_throws(0x140bc8910, "content_140bc8910", fn_140bc8910 as *const (), Throws::Caught), // sub_140bc8910
    Replacement::draft_throws(0x140bc89e0, "content_140bc89e0", fn_140bc89e0 as *const (), Throws::Caught), // sub_140bc89e0
    Replacement::draft_throws(0x140bc8ab0, "content_140bc8ab0", fn_140bc8ab0 as *const (), Throws::Caught), // sub_140bc8ab0
    Replacement::draft(0x140bc8b80, "content_140bc8b80", fn_140bc8b80 as *const ()), // sub_140bc8b80
    Replacement::draft(0x140bc8ce0, "content_140bc8ce0", fn_140bc8ce0 as *const ()), // sub_140bc8ce0
    Replacement::draft(0x140bc8e40, "content_140bc8e40", fn_140bc8e40 as *const ()), // sub_140bc8e40
    Replacement::draft_throws(0x140bc8fb0, "content_140bc8fb0", fn_140bc8fb0 as *const (), Throws::Discard), // sub_140bc8fb0
    Replacement::draft_throws(0x140bc9100, "content_140bc9100", tpl_14010::<0x14387e908, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432be460, 0x8, 0x1432be440, 0x1432be430, 0x140c07770> as *const (), Throws::Discard), // sub_140bc9100
    Replacement::draft(0x140bc91f0, "content_140bc91f0", fn_140bc91f0 as *const ()), // sub_140bc91f0
    Replacement::draft(0x140bc9270, "content_140bc9270", fn_140bc9270 as *const ()), // sub_140bc9270
    Replacement::draft(0x140bc92f0, "content_140bc92f0", fn_140bc92f0 as *const ()), // sub_140bc92f0
    Replacement::draft(0x140bc9400, "content_140bc9400", tpl_12161::<0x14387ec78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bc9400
    Replacement::draft_throws(0x140bc94b0, "content_140bc94b0", tpl_7851::<0x14387ef70, 0x14387ec78, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x8, 0x48, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140bc94b0
    Replacement::draft(0x140bc9540, "content_140bc9540", tpl_9969::<0x14387ec78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bc9540
    Replacement::draft_throws(0x140bc95c0, "content_140bc95c0", tpl_7972::<0x14387ef70, 0x14387ec78, 0x1401e9b60, 0x1406e03a0, 0x328, 0x338, 0x120, 0x8, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140bc95c0
    Replacement::draft_throws(0x140bc9650, "content_140bc9650", fn_140bc9650 as *const (), Throws::Caught), // sub_140bc9650
    Replacement::draft_throws(0x140bc9840, "content_140bc9840", fn_140bc9840 as *const (), Throws::Caught), // sub_140bc9840
    Replacement::draft_throws(0x140bc9a00, "content_140bc9a00", fn_140bc9a00 as *const (), Throws::Caught), // sub_140bc9a00
    Replacement::draft(0x140bc9bc0, "content_140bc9bc0", tpl_12163::<0x14387ec78, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bc9bc0
    Replacement::draft(0x140bc9c30, "content_140bc9c30", tpl_8074::<0x14387ef70, 0x14387ec78, 0x1401e9970, 0x1406e1480, 0x8, 0x338, 0xe8, 0x1401e9860> as *const ()), // sub_140bc9c30
    Replacement::draft(0x140bc9cb0, "content_140bc9cb0", tpl_13937::<0x14387ec78, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bc9cb0
    Replacement::draft_throws(0x140bc9d40, "content_140bc9d40", tpl_7974::<0x14387ef70, 0x14387ec78, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140bc9d40
    Replacement::draft_throws(0x140bc9dc0, "content_140bc9dc0", tpl_13938::<0x14387ec78, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x20, 0x8, 0x1432be4d0> as *const (), Throws::Discard), // sub_140bc9dc0
    Replacement::draft(0x140bc9ea0, "content_140bc9ea0", tpl_7909::<0x120, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140bc9f30> as *const ()), // sub_140bc9ea0
    Replacement::draft(0x140bc9f30, "content_140bc9f30", fn_140bc9f30 as *const ()), // sub_140bc9f30
    Replacement::draft(0x140bca020, "content_140bca020", tpl_66781::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x140bc9f30> as *const ()), // sub_140bca020
    Replacement::draft(0x140bca0d0, "content_140bca0d0", fn_140bca0d0 as *const ()), // sub_140bca0d0
    Replacement::draft(0x140bca5f0, "content_140bca5f0", fn_140bca5f0 as *const ()), // sub_140bca5f0
    Replacement::draft_throws(0x140bcac40, "content_140bcac40", fn_140bcac40 as *const (), Throws::Caught), // sub_140bcac40
    Replacement::draft(0x140bcb460, "content_140bcb460", fn_140bcb460 as *const ()), // sub_140bcb460
    Replacement::draft_throws(0x140bcbaf0, "content_140bcbaf0", fn_140bcbaf0 as *const (), Throws::Caught), // sub_140bcbaf0
    Replacement::draft(0x140bcc2c0, "content_140bcc2c0", fn_140bcc2c0 as *const ()), // sub_140bcc2c0
    Replacement::draft_throws(0x140bcc4c0, "content_140bcc4c0", fn_140bcc4c0 as *const (), Throws::Caught), // sub_140bcc4c0
    Replacement::draft_throws(0x140bccb50, "content_140bccb50", fn_140bccb50 as *const (), Throws::Caught), // sub_140bccb50
    Replacement::draft_throws(0x140bcd0f0, "content_140bcd0f0", fn_140bcd0f0 as *const (), Throws::Caught), // sub_140bcd0f0
    Replacement::draft(0x140bcd690, "content_140bcd690", fn_140bcd690 as *const ()), // sub_140bcd690
    Replacement::draft(0x140bcde60, "content_140bcde60", fn_140bcde60 as *const ()), // sub_140bcde60
    Replacement::draft(0x140bce640, "content_140bce640", fn_140bce640 as *const ()), // sub_140bce640
    Replacement::draft_throws(0x140bcee20, "content_140bcee20", fn_140bcee20 as *const (), Throws::Caught), // sub_140bcee20
    Replacement::draft_throws(0x140bcf470, "content_140bcf470", fn_140bcf470 as *const (), Throws::Caught), // sub_140bcf470
    Replacement::draft(0x140bcf990, "content_140bcf990", tpl_14750::<0x1432be0f0, 0x98, 0x1432be0d8, 0x13, 0x140c05f40, 0x143874640, 0x1432be0a0, 0xd0, 0x1432be080, 0x1401de410, 0x1432be180, 0xd8, 0x1432be168, 0x1432be138, 0x1432be120> as *const ()), // sub_140bcf990
    Replacement::draft(0x140bcfa70, "content_140bcfa70", tpl_14750::<0x1432be0f0, 0x98, 0x1432be0d8, 0x13, 0x140c05f70, 0x143874640, 0x1432be0a0, 0xd0, 0x1432be080, 0x1401de410, 0x1432be180, 0xd8, 0x1432be168, 0x1432be138, 0x1432be120> as *const ()), // sub_140bcfa70
    Replacement::draft(0x140bcfb50, "content_140bcfb50", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432ca7c8, 0x8> as *const ()), // sub_140bcfb50
    Replacement::draft(0x140bcfbe0, "content_140bcfbe0", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432ca7c8, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140bcfbe0
    Replacement::draft(0x140bcfc90, "content_140bcfc90", fn_140bcfc90 as *const ()), // sub_140bcfc90
    Replacement::draft(0x140bcfd70, "content_140bcfd70", fn_140bcfd70 as *const ()), // sub_140bcfd70
    Replacement::draft_throws(0x140bcfe70, "content_140bcfe70", tpl_9677::<0x14387ece4, 0x14387eba0, 0x1401e9fa0, 0x1406e02b0, 0x328, 0x338, 0x48, 0x8, 0x140c098e0, 0x140c2bf50, 0x1401e9e70> as *const (), Throws::Discard), // sub_140bcfe70
    Replacement::draft(0x140bcff20, "content_140bcff20", fn_140bcff20 as *const ()), // sub_140bcff20
    Replacement::draft_throws(0x140bcfff0, "content_140bcfff0", tpl_9678::<0x14387ece4, 0x14387eba0, 0x1401e9b60, 0x1406e03a0, 0x8, 0x328, 0x338, 0x120, 0x140c098e0, 0x140c2c240, 0x1401e9a20, 0x369> as *const (), Throws::Discard), // sub_140bcfff0
    Replacement::draft(0x140bd00b0, "content_140bd00b0", fn_140bd00b0 as *const ()), // sub_140bd00b0
    Replacement::draft_throws(0x140bd0140, "content_140bd0140", fn_140bd0140 as *const (), Throws::Caught), // sub_140bd0140
    Replacement::draft_throws(0x140bd0200, "content_140bd0200", fn_140bd0200 as *const (), Throws::Caught), // sub_140bd0200
    Replacement::draft_throws(0x140bd02c0, "content_140bd02c0", fn_140bd02c0 as *const (), Throws::Caught), // sub_140bd02c0
    Replacement::draft(0x140bd0380, "content_140bd0380", fn_140bd0380 as *const ()), // sub_140bd0380
    Replacement::draft(0x140bd0430, "content_140bd0430", fn_140bd0430 as *const ()), // sub_140bd0430
    Replacement::draft(0x140bd04e0, "content_140bd04e0", fn_140bd04e0 as *const ()), // sub_140bd04e0
    Replacement::draft_throws(0x140bd05b0, "content_140bd05b0", fn_140bd05b0 as *const (), Throws::Discard), // sub_140bd05b0
    Replacement::draft_throws(0x140bd0660, "content_140bd0660", tpl_6745::<0x14387eba0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432be210, 0x8, 0x1432be1f8, 0x140c079e0> as *const (), Throws::Discard), // sub_140bd0660
    Replacement::draft(0x140bd0740, "content_140bd0740", tpl_14161::<0x10, 0x1432be1f8, 0x8, 0x140c2d2d0> as *const ()), // sub_140bd0740
    Replacement::draft(0x140bd07c0, "content_140bd07c0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432c85e8> as *const ()), // sub_140bd07c0
    Replacement::draft(0x140bd0820, "content_140bd0820", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c85e8> as *const ()), // sub_140bd0820
    Replacement::draft(0x140bd08b0, "content_140bd08b0", fn_140bd08b0 as *const ()), // sub_140bd08b0
    Replacement::draft_throws(0x140bd0a70, "content_140bd0a70", tpl_14755::<0x14387efbc, 0x14387ef18, 0x1401e9fa0, 0x1406e02b0, 0x1406d7a10, 0x1401e9e70> as *const (), Throws::Discard), // sub_140bd0a70
    Replacement::draft_throws(0x140bd0c20, "content_140bd0c20", tpl_14755::<0x14387efbc, 0x14387ef18, 0x1401e9b60, 0x1406e03a0, 0x1406d7a10, 0x1401e9a20> as *const (), Throws::Discard), // sub_140bd0c20
    Replacement::draft_throws(0x140bd0dd0, "content_140bd0dd0", fn_140bd0dd0 as *const (), Throws::Caught), // sub_140bd0dd0
    Replacement::draft(0x140bd0e60, "content_140bd0e60", fn_140bd0e60 as *const ()), // sub_140bd0e60
    Replacement::draft_throws(0x140bd1010, "content_140bd1010", tpl_14755::<0x14387efbc, 0x14387ef18, 0x1401e9770, 0x1406e1570, 0x1406d7a10, 0x1401e9590> as *const (), Throws::Discard), // sub_140bd1010
    Replacement::draft_throws(0x140bd11c0, "content_140bd11c0", fn_140bd11c0 as *const (), Throws::Discard), // sub_140bd11c0
    Replacement::draft(0x140bd1270, "content_140bd1270", fn_140bd1270 as *const ()), // sub_140bd1270
    Replacement::draft(0x140bd1340, "content_140bd1340", fn_140bd1340 as *const ()), // sub_140bd1340
    Replacement::draft(0x140bd1420, "content_140bd1420", fn_140bd1420 as *const ()), // sub_140bd1420
    Replacement::draft(0x140bd14c0, "content_140bd14c0", fn_140bd14c0 as *const ()), // sub_140bd14c0
    Replacement::draft_throws(0x140bd1600, "content_140bd1600", fn_140bd1600 as *const (), Throws::Caught), // sub_140bd1600
    Replacement::draft(0x140bd1760, "content_140bd1760", fn_140bd1760 as *const ()), // sub_140bd1760
    Replacement::draft_throws(0x140bd1870, "content_140bd1870", fn_140bd1870 as *const (), Throws::Discard), // sub_140bd1870
    Replacement::draft_throws(0x140bd1970, "content_140bd1970", fn_140bd1970 as *const (), Throws::Caught), // sub_140bd1970
    Replacement::draft_throws(0x140bd1d10, "content_140bd1d10", fn_140bd1d10 as *const (), Throws::Caught), // sub_140bd1d10
    Replacement::draft_throws(0x140bd2050, "content_140bd2050", fn_140bd2050 as *const (), Throws::Caught), // sub_140bd2050
    Replacement::draft(0x140bd2380, "content_140bd2380", fn_140bd2380 as *const ()), // sub_140bd2380
    Replacement::draft(0x140bd2510, "content_140bd2510", fn_140bd2510 as *const ()), // sub_140bd2510
    Replacement::draft(0x140bd2600, "content_140bd2600", fn_140bd2600 as *const ()), // sub_140bd2600
    Replacement::draft_throws(0x140bd2700, "content_140bd2700", fn_140bd2700 as *const (), Throws::Discard), // sub_140bd2700
    Replacement::draft_throws(0x140bd27e0, "content_140bd27e0", fn_140bd27e0 as *const (), Throws::Discard), // sub_140bd27e0
    Replacement::draft(0x140bd2910, "content_140bd2910", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431391c8, 0x8> as *const ()), // sub_140bd2910
    Replacement::draft(0x140bd29a0, "content_140bd29a0", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1431391c8, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140bd29a0
    Replacement::draft(0x140bd2a50, "content_140bd2a50", fn_140bd2a50 as *const ()), // sub_140bd2a50
    Replacement::draft(0x140bd2d30, "content_140bd2d30", fn_140bd2d30 as *const ()), // sub_140bd2d30
    Replacement::draft_throws(0x140bd3000, "content_140bd3000", tpl_14775::<0x14387ea14, 0x14387ef68, 0x1401e9fa0, 0x1406e02b0, 0x1403c7a30, 0x1403c7ab0, 0x140c07a20, 0x140c07b00, 0x140c07be0, 0x1403c79b0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140bd3000
    Replacement::draft(0x140bd3250, "content_140bd3250", fn_140bd3250 as *const ()), // sub_140bd3250
    Replacement::draft_throws(0x140bd34f0, "content_140bd34f0", tpl_14775::<0x14387ea14, 0x14387ef68, 0x1401e9b60, 0x1406e03a0, 0x1403c7a30, 0x1403c7ab0, 0x140c07a20, 0x140c07b00, 0x140c07d90, 0x1403c79b0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140bd34f0
    Replacement::draft(0x140bd3740, "content_140bd3740", fn_140bd3740 as *const ()), // sub_140bd3740
    Replacement::draft_throws(0x140bd37b0, "content_140bd37b0", fn_140bd37b0 as *const (), Throws::Caught), // sub_140bd37b0
    Replacement::draft_throws(0x140bd3880, "content_140bd3880", fn_140bd3880 as *const (), Throws::Caught), // sub_140bd3880
    Replacement::draft_throws(0x140bd3950, "content_140bd3950", fn_140bd3950 as *const (), Throws::Caught), // sub_140bd3950
    Replacement::draft(0x140bd3a20, "content_140bd3a20", fn_140bd3a20 as *const ()), // sub_140bd3a20
    Replacement::draft(0x140bd3cc0, "content_140bd3cc0", fn_140bd3cc0 as *const ()), // sub_140bd3cc0
    Replacement::draft(0x140bd3f60, "content_140bd3f60", fn_140bd3f60 as *const ()), // sub_140bd3f60
    Replacement::draft_throws(0x140bd4200, "content_140bd4200", fn_140bd4200 as *const (), Throws::Discard), // sub_140bd4200
    Replacement::draft_throws(0x140bd4490, "content_140bd4490", fn_140bd4490 as *const (), Throws::Discard), // sub_140bd4490
    Replacement::draft(0x140bd4580, "content_140bd4580", fn_140bd4580 as *const ()), // sub_140bd4580
    Replacement::draft(0x140bd4610, "content_140bd4610", fn_140bd4610 as *const ()), // sub_140bd4610
    Replacement::draft(0x140bd46b0, "content_140bd46b0", tpl_7312::<0x14387ef00, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bd46b0
    Replacement::draft(0x140bd4720, "content_140bd4720", fn_140bd4720 as *const ()), // sub_140bd4720
    Replacement::draft_throws(0x140bd47e0, "content_140bd47e0", fn_140bd47e0 as *const (), Throws::Discard), // sub_140bd47e0
    Replacement::draft(0x140bd48d0, "content_140bd48d0", fn_140bd48d0 as *const ()), // sub_140bd48d0
    Replacement::draft_throws(0x140bd49c0, "content_140bd49c0", fn_140bd49c0 as *const (), Throws::Discard), // sub_140bd49c0
    Replacement::draft(0x140bd4ad0, "content_140bd4ad0", fn_140bd4ad0 as *const ()), // sub_140bd4ad0
    Replacement::draft_throws(0x140bd4b70, "content_140bd4b70", fn_140bd4b70 as *const (), Throws::Caught), // sub_140bd4b70
    Replacement::draft_throws(0x140bd4da0, "content_140bd4da0", fn_140bd4da0 as *const (), Throws::Caught), // sub_140bd4da0
    Replacement::draft_throws(0x140bd4fa0, "content_140bd4fa0", fn_140bd4fa0 as *const (), Throws::Caught), // sub_140bd4fa0
    Replacement::draft(0x140bd5190, "content_140bd5190", fn_140bd5190 as *const ()), // sub_140bd5190
    Replacement::draft(0x140bd5230, "content_140bd5230", fn_140bd5230 as *const ()), // sub_140bd5230
    Replacement::draft(0x140bd5310, "content_140bd5310", fn_140bd5310 as *const ()), // sub_140bd5310
    Replacement::draft_throws(0x140bd5400, "content_140bd5400", fn_140bd5400 as *const (), Throws::Discard), // sub_140bd5400
    Replacement::draft_throws(0x140bd54e0, "content_140bd54e0", fn_140bd54e0 as *const (), Throws::Discard), // sub_140bd54e0
    Replacement::draft(0x140bd55f0, "content_140bd55f0", tpl_6933::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430abdd0, 0x8> as *const ()), // sub_140bd55f0
    Replacement::draft(0x140bd5680, "content_140bd5680", tpl_9871::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1430abdd0, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x18> as *const ()), // sub_140bd5680
    Replacement::draft(0x140bd5730, "content_140bd5730", fn_140bd5730 as *const ()), // sub_140bd5730
    Replacement::draft(0x140bd5800, "content_140bd5800", fn_140bd5800 as *const ()), // sub_140bd5800
    Replacement::draft_throws(0x140bd58e0, "content_140bd58e0", tpl_6937::<0x14387ecf0, 0x14387eb48, 0x1401e9fa0, 0x1406e02b0, 0x8, 0x140c07fc0, 0x1401e9e70> as *const (), Throws::Discard), // sub_140bd58e0
    Replacement::draft(0x140bd5940, "content_140bd5940", tpl_12319::<0x14387eb48, 0x14243b0b0, 0x1406e01b0, 0x338, 0x48, 0x8, 0x140c097f0, 0x140c95a00, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bd5940
    Replacement::draft_throws(0x140bd5a00, "content_140bd5a00", tpl_6937::<0x14387ecf0, 0x14387eb48, 0x1401e9b60, 0x1406e03a0, 0x8, 0x140c08170, 0x1401e9a20> as *const (), Throws::Discard), // sub_140bd5a00
    Replacement::draft(0x140bd5a60, "content_140bd5a60", fn_140bd5a60 as *const ()), // sub_140bd5a60
    Replacement::draft_throws(0x140bd5ae0, "content_140bd5ae0", fn_140bd5ae0 as *const (), Throws::Caught), // sub_140bd5ae0
    Replacement::draft_throws(0x140bd5ba0, "content_140bd5ba0", fn_140bd5ba0 as *const (), Throws::Caught), // sub_140bd5ba0
    Replacement::draft_throws(0x140bd5c60, "content_140bd5c60", fn_140bd5c60 as *const (), Throws::Caught), // sub_140bd5c60
    Replacement::draft(0x140bd5d20, "content_140bd5d20", tpl_12322::<0x14387eb48, 0x14243b0b0, 0x1406e01b0, 0x338, 0x10, 0x8, 0x140c95820, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bd5d20
    Replacement::draft(0x140bd5dd0, "content_140bd5dd0", tpl_70790::<0x14387ecf0, 0x14387eb48, 0x1401e9970, 0x1406e1480, 0x10, 0x338, 0xe8, 0x8, 0x140c96770, 0x1401e9860> as *const ()), // sub_140bd5dd0
    Replacement::draft(0x140bd5e80, "content_140bd5e80", tpl_70791::<0x14387eb48, 0x14243b0b0, 0x1406e01b0, 0x10, 0x338, 0xe8, 0x8, 0x140c96830, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bd5e80
    Replacement::draft_throws(0x140bd5f40, "content_140bd5f40", tpl_70792::<0x14387ecf0, 0x14387eb48, 0x1401e9770, 0x1406e1570, 0x338, 0x10, 0x120, 0x8, 0x140c96900, 0x1401e9590> as *const (), Throws::Discard), // sub_140bd5f40
    Replacement::draft_throws(0x140bd5fe0, "content_140bd5fe0", tpl_6941::<0x14387eb48, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x1432be010, 0x8, 0x1432be000, 0x140c08360> as *const (), Throws::Discard), // sub_140bd5fe0
    Replacement::draft(0x140bd60c0, "content_140bd60c0", tpl_11529::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0x1432cc858> as *const ()), // sub_140bd60c0
    Replacement::draft(0x140bd6150, "content_140bd6150", tpl_11530::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cc858, 0x18, 0x20> as *const ()), // sub_140bd6150
    Replacement::draft(0x140bd61f0, "content_140bd61f0", tpl_11830::<0x14387ef30, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bd61f0
    Replacement::draft(0x140bd6260, "content_140bd6260", fn_140bd6260 as *const ()), // sub_140bd6260
    Replacement::draft_throws(0x140bd63e0, "content_140bd63e0", fn_140bd63e0 as *const (), Throws::Discard), // sub_140bd63e0
    Replacement::draft(0x140bd6520, "content_140bd6520", tpl_14795::<0x14387ef30, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x14, 0x10, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140bd6520
    Replacement::draft_throws(0x140bd65e0, "content_140bd65e0", fn_140bd65e0 as *const (), Throws::Discard), // sub_140bd65e0
    Replacement::draft_throws(0x140bd6750, "content_140bd6750", fn_140bd6750 as *const (), Throws::Caught), // sub_140bd6750
    Replacement::draft_throws(0x140bd69f0, "content_140bd69f0", fn_140bd69f0 as *const (), Throws::Caught), // sub_140bd69f0
    Replacement::draft_throws(0x140bd6c90, "content_140bd6c90", fn_140bd6c90 as *const (), Throws::Caught), // sub_140bd6c90
    Replacement::draft(0x140bd6f30, "content_140bd6f30", fn_140bd6f30 as *const ()), // sub_140bd6f30
    Replacement::draft(0x140bd6fc0, "content_140bd6fc0", fn_140bd6fc0 as *const ()), // sub_140bd6fc0
    Replacement::draft(0x140bd7080, "content_140bd7080", tpl_14795::<0x14387ef30, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x14, 0x10, 0x18, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140bd7080
    Replacement::draft_throws(0x140bd7150, "content_140bd7150", fn_140bd7150 as *const (), Throws::Discard), // sub_140bd7150
    Replacement::draft_throws(0x140bd7240, "content_140bd7240", fn_140bd7240 as *const (), Throws::Discard), // sub_140bd7240
    Replacement::draft(0x140bd7500, "content_140bd7500", fn_140bd7500 as *const ()), // sub_140bd7500
    Replacement::draft(0x140bd76d0, "content_140bd76d0", fn_140bd76d0 as *const ()), // sub_140bd76d0
    Replacement::draft(0x140bd7790, "content_140bd7790", fn_140bd7790 as *const ()), // sub_140bd7790
    Replacement::draft(0x140bd7840, "content_140bd7840", fn_140bd7840 as *const ()), // sub_140bd7840
    Replacement::draft(0x140bd78e0, "content_140bd78e0", fn_140bd78e0 as *const ()), // sub_140bd78e0
    Replacement::draft_throws(0x140bd7e00, "content_140bd7e00", fn_140bd7e00 as *const (), Throws::Caught), // sub_140bd7e00
    Replacement::draft(0x140bd8320, "content_140bd8320", fn_140bd8320 as *const ()), // sub_140bd8320
    Replacement::draft_throws(0x140bd8620, "content_140bd8620", fn_140bd8620 as *const (), Throws::Discard), // sub_140bd8620
    Replacement::draft(0x140bd8b80, "content_140bd8b80", fn_140bd8b80 as *const ()), // sub_140bd8b80
    Replacement::draft_throws(0x140bd8d00, "content_140bd8d00", fn_140bd8d00 as *const (), Throws::Caught), // sub_140bd8d00
    Replacement::draft_throws(0x140bd9690, "content_140bd9690", fn_140bd9690 as *const (), Throws::Caught), // sub_140bd9690
    Replacement::draft_throws(0x140bda000, "content_140bda000", fn_140bda000 as *const (), Throws::Caught), // sub_140bda000
    Replacement::draft(0x140bda960, "content_140bda960", fn_140bda960 as *const ()), // sub_140bda960
    Replacement::draft(0x140bdab70, "content_140bdab70", fn_140bdab70 as *const ()), // sub_140bdab70
    Replacement::draft(0x140bdaea0, "content_140bdaea0", fn_140bdaea0 as *const ()), // sub_140bdaea0
    Replacement::draft_throws(0x140bdb1e0, "content_140bdb1e0", fn_140bdb1e0 as *const (), Throws::Discard), // sub_140bdb1e0
    Replacement::draft_throws(0x140bdb530, "content_140bdb530", fn_140bdb530 as *const (), Throws::Discard), // sub_140bdb530
    Replacement::draft(0x140bdbd10, "content_140bdbd10", fn_140bdbd10 as *const ()), // sub_140bdbd10
    Replacement::draft(0x140bdc1e0, "content_140bdc1e0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430a9e60, 0x8> as *const ()), // sub_140bdc1e0
    Replacement::draft(0x140bdc260, "content_140bdc260", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430a9e60> as *const ()), // sub_140bdc260
    Replacement::draft(0x140bdc2f0, "content_140bdc2f0", tpl_7312::<0x14387f098, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdc2f0
    Replacement::draft(0x140bdc360, "content_140bdc360", tpl_11575::<0x14387f098, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x1426d4580, 0x1, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdc360
    Replacement::draft_throws(0x140bdc440, "content_140bdc440", fn_140bdc440 as *const (), Throws::Discard), // sub_140bdc440
    Replacement::draft(0x140bdc500, "content_140bdc500", tpl_11577::<0x14387f098, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdc500
    Replacement::draft_throws(0x140bdc590, "content_140bdc590", fn_140bdc590 as *const (), Throws::Discard), // sub_140bdc590
    Replacement::draft_throws(0x140bdc670, "content_140bdc670", fn_140bdc670 as *const (), Throws::Caught), // sub_140bdc670
    Replacement::draft_throws(0x140bdc7c0, "content_140bdc7c0", fn_140bdc7c0 as *const (), Throws::Caught), // sub_140bdc7c0
    Replacement::draft_throws(0x140bdc910, "content_140bdc910", fn_140bdc910 as *const (), Throws::Caught), // sub_140bdc910
    Replacement::draft(0x140bdca60, "content_140bdca60", fn_140bdca60 as *const ()), // sub_140bdca60
    Replacement::draft(0x140bdcae0, "content_140bdcae0", tpl_11581::<0x14387ef94, 0x14387f098, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x1401e9860> as *const ()), // sub_140bdcae0
    Replacement::draft(0x140bdcb70, "content_140bdcb70", tpl_11577::<0x14387f098, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdcb70
    Replacement::draft_throws(0x140bdcc00, "content_140bdcc00", tpl_11582::<0x14387ef94, 0x14387f098, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0xc, 0x1401e9590> as *const (), Throws::Discard), // sub_140bdcc00
    Replacement::draft_throws(0x140bdcca0, "content_140bdcca0", fn_140bdcca0 as *const (), Throws::Discard), // sub_140bdcca0
    Replacement::draft(0x140bdce00, "content_140bdce00", fn_140bdce00 as *const ()), // sub_140bdce00
    Replacement::draft(0x140bdcec0, "content_140bdcec0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430a81a8, 0x8> as *const ()), // sub_140bdcec0
    Replacement::draft(0x140bdcf40, "content_140bdcf40", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430a81a8> as *const ()), // sub_140bdcf40
    Replacement::draft(0x140bdd040, "content_140bdd040", tpl_14066::<0x14387ece8, 0x14243b0b0, 0x1406e01b0, 0x8, 0x338, 0x1426d4580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdd040
    Replacement::draft_throws(0x140bdd0e0, "content_140bdd0e0", fn_140bdd0e0 as *const (), Throws::Discard), // sub_140bdd0e0
    Replacement::draft(0x140bdd170, "content_140bdd170", tpl_9969::<0x14387ece8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdd170
    Replacement::draft_throws(0x140bdd1f0, "content_140bdd1f0", fn_140bdd1f0 as *const (), Throws::Discard), // sub_140bdd1f0
    Replacement::draft_throws(0x140bdd280, "content_140bdd280", fn_140bdd280 as *const (), Throws::Caught), // sub_140bdd280
    Replacement::draft_throws(0x140bdd360, "content_140bdd360", fn_140bdd360 as *const (), Throws::Caught), // sub_140bdd360
    Replacement::draft_throws(0x140bdd440, "content_140bdd440", fn_140bdd440 as *const (), Throws::Caught), // sub_140bdd440
    Replacement::draft(0x140bdd520, "content_140bdd520", fn_140bdd520 as *const ()), // sub_140bdd520
    Replacement::draft(0x140bdd590, "content_140bdd590", tpl_7856::<0x14387eb7c, 0x14387ece8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0x120, 0x1401e9860> as *const ()), // sub_140bdd590
    Replacement::draft(0x140bdd600, "content_140bdd600", tpl_9969::<0x14387ece8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdd600
    Replacement::draft_throws(0x140bdd680, "content_140bdd680", tpl_8130::<0x14387eb7c, 0x14387ece8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140bdd680
    Replacement::draft_throws(0x140bdd700, "content_140bdd700", tpl_12086::<0x14387ece8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x8, 0x1432bf438, 0x1432bf518, 0x1432bf418, 0x14020f0f0> as *const (), Throws::Discard), // sub_140bdd700
    Replacement::draft(0x140bdd800, "content_140bdd800", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1430a8050> as *const ()), // sub_140bdd800
    Replacement::draft(0x140bdd860, "content_140bdd860", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430a8050> as *const ()), // sub_140bdd860
    Replacement::draft_throws(0x140bdd960, "content_140bdd960", fn_140bdd960 as *const (), Throws::Caught), // sub_140bdd960
    Replacement::draft_throws(0x140bdd9f0, "content_140bdd9f0", tpl_6762::<0x14387ec68, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140bdd9f0
    Replacement::draft(0x140bddaa0, "content_140bddaa0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1430a4238, 0x8> as *const ()), // sub_140bddaa0
    Replacement::draft(0x140bddb20, "content_140bddb20", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1430a4238> as *const ()), // sub_140bddb20
    Replacement::draft(0x140bddbb0, "content_140bddbb0", tpl_7312::<0x14387f058, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bddbb0
    Replacement::draft(0x140bddc20, "content_140bddc20", tpl_11575::<0x14387f058, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x1426d4580, 0x1, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bddc20
    Replacement::draft_throws(0x140bddd00, "content_140bddd00", fn_140bddd00 as *const (), Throws::Discard), // sub_140bddd00
    Replacement::draft(0x140bdddc0, "content_140bdddc0", tpl_11577::<0x14387f058, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bdddc0
    Replacement::draft_throws(0x140bdde50, "content_140bdde50", fn_140bdde50 as *const (), Throws::Discard), // sub_140bdde50
    Replacement::draft_throws(0x140bddf30, "content_140bddf30", fn_140bddf30 as *const (), Throws::Caught), // sub_140bddf30
    Replacement::draft_throws(0x140bde0b0, "content_140bde0b0", fn_140bde0b0 as *const (), Throws::Caught), // sub_140bde0b0
    Replacement::draft_throws(0x140bde230, "content_140bde230", fn_140bde230 as *const (), Throws::Caught), // sub_140bde230
    Replacement::draft(0x140bde3b0, "content_140bde3b0", fn_140bde3b0 as *const ()), // sub_140bde3b0
    Replacement::draft(0x140bde430, "content_140bde430", tpl_11581::<0x14387eac8, 0x14387f058, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x1401e9860> as *const ()), // sub_140bde430
    Replacement::draft(0x140bde4c0, "content_140bde4c0", tpl_11577::<0x14387f058, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bde4c0
    Replacement::draft_throws(0x140bde550, "content_140bde550", tpl_11582::<0x14387eac8, 0x14387f058, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0xc, 0x1401e9590> as *const (), Throws::Discard), // sub_140bde550
    Replacement::draft_throws(0x140bde5f0, "content_140bde5f0", fn_140bde5f0 as *const (), Throws::Discard), // sub_140bde5f0
    Replacement::draft(0x140bde790, "content_140bde790", fn_140bde790 as *const ()), // sub_140bde790
    Replacement::draft(0x140bde870, "content_140bde870", tpl_11918::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1431fb5a0, 0x8> as *const ()), // sub_140bde870
    Replacement::draft(0x140bde900, "content_140bde900", tpl_11919::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1431fb5a0, 0x18> as *const ()), // sub_140bde900
    Replacement::draft(0x140bde9b0, "content_140bde9b0", tpl_13738::<0x14387ee08, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bde9b0
    Replacement::draft(0x140bdea20, "content_140bdea20", fn_140bdea20 as *const ()), // sub_140bdea20
    Replacement::draft_throws(0x140bdeb50, "content_140bdeb50", fn_140bdeb50 as *const (), Throws::Caught), // sub_140bdeb50
    Replacement::draft(0x140bdecc0, "content_140bdecc0", fn_140bdecc0 as *const ()), // sub_140bdecc0
    Replacement::draft_throws(0x140bdede0, "content_140bdede0", fn_140bdede0 as *const (), Throws::Discard), // sub_140bdede0
    Replacement::draft_throws(0x140bdeec0, "content_140bdeec0", fn_140bdeec0 as *const (), Throws::Caught), // sub_140bdeec0
    Replacement::draft_throws(0x140bdf270, "content_140bdf270", fn_140bdf270 as *const (), Throws::Caught), // sub_140bdf270
    Replacement::draft_throws(0x140bdf590, "content_140bdf590", fn_140bdf590 as *const (), Throws::Caught), // sub_140bdf590
    Replacement::draft(0x140bdf8b0, "content_140bdf8b0", fn_140bdf8b0 as *const ()), // sub_140bdf8b0
    Replacement::draft(0x140bdfaa0, "content_140bdfaa0", fn_140bdfaa0 as *const ()), // sub_140bdfaa0
    Replacement::draft(0x140bdfb90, "content_140bdfb90", fn_140bdfb90 as *const ()), // sub_140bdfb90
    Replacement::draft_throws(0x140bdfc80, "content_140bdfc80", fn_140bdfc80 as *const (), Throws::Discard), // sub_140bdfc80
    Replacement::draft_throws(0x140bdfd70, "content_140bdfd70", fn_140bdfd70 as *const (), Throws::Discard), // sub_140bdfd70
    Replacement::draft(0x140bdfed0, "content_140bdfed0", tpl_14848::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x28, 0x1432cf880, 0x8, 0x20> as *const ()), // sub_140bdfed0
    Replacement::draft(0x140bdff70, "content_140bdff70", tpl_13753::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432cf880, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_140bdff70
    Replacement::draft(0x140be0020, "content_140be0020", fn_140be0020 as *const ()), // sub_140be0020
    Replacement::draft(0x140be0190, "content_140be0190", fn_140be0190 as *const ()), // sub_140be0190
    Replacement::draft_throws(0x140be0350, "content_140be0350", fn_140be0350 as *const (), Throws::Discard), // sub_140be0350
    Replacement::draft(0x140be0480, "content_140be0480", fn_140be0480 as *const ()), // sub_140be0480
    Replacement::draft_throws(0x140be05b0, "content_140be05b0", fn_140be05b0 as *const (), Throws::Discard), // sub_140be05b0
    Replacement::draft(0x140be0730, "content_140be0730", fn_140be0730 as *const ()), // sub_140be0730
    Replacement::draft_throws(0x140be0840, "content_140be0840", fn_140be0840 as *const (), Throws::Caught), // sub_140be0840
    Replacement::draft_throws(0x140be0970, "content_140be0970", fn_140be0970 as *const (), Throws::Caught), // sub_140be0970
    Replacement::draft_throws(0x140be0aa0, "content_140be0aa0", fn_140be0aa0 as *const (), Throws::Caught), // sub_140be0aa0
    Replacement::draft(0x140be0bd0, "content_140be0bd0", fn_140be0bd0 as *const ()), // sub_140be0bd0
    Replacement::draft(0x140be0cd0, "content_140be0cd0", fn_140be0cd0 as *const ()), // sub_140be0cd0
    Replacement::draft(0x140be0df0, "content_140be0df0", fn_140be0df0 as *const ()), // sub_140be0df0
    Replacement::draft_throws(0x140be0f20, "content_140be0f20", fn_140be0f20 as *const (), Throws::Discard), // sub_140be0f20
    Replacement::draft_throws(0x140be1040, "content_140be1040", fn_140be1040 as *const (), Throws::Discard), // sub_140be1040
    Replacement::draft(0x140be1190, "content_140be1190", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143094798> as *const ()), // sub_140be1190
    Replacement::draft(0x140be11f0, "content_140be11f0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143094798> as *const ()), // sub_140be11f0
    Replacement::draft_throws(0x140be12f0, "content_140be12f0", fn_140be12f0 as *const (), Throws::Caught), // sub_140be12f0
    Replacement::draft_throws(0x140be1380, "content_140be1380", tpl_6762::<0x14387ee78, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140be1380
    Replacement::draft(0x140be1430, "content_140be1430", tpl_9792::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143088d68, 0x8> as *const ()), // sub_140be1430
    Replacement::draft(0x140be14c0, "content_140be14c0", fn_140be14c0 as *const ()), // sub_140be14c0
    Replacement::draft(0x140be1570, "content_140be1570", tpl_14861::<0x14387eca0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140be1570
    Replacement::draft(0x140be15e0, "content_140be15e0", fn_140be15e0 as *const ()), // sub_140be15e0
    Replacement::draft_throws(0x140be1750, "content_140be1750", fn_140be1750 as *const (), Throws::Caught), // sub_140be1750
    Replacement::draft(0x140be1920, "content_140be1920", fn_140be1920 as *const ()), // sub_140be1920
    Replacement::draft_throws(0x140be1a90, "content_140be1a90", fn_140be1a90 as *const (), Throws::Discard), // sub_140be1a90
    Replacement::draft_throws(0x140be1b80, "content_140be1b80", fn_140be1b80 as *const (), Throws::Caught), // sub_140be1b80
    Replacement::draft_throws(0x140be2060, "content_140be2060", fn_140be2060 as *const (), Throws::Caught), // sub_140be2060
    Replacement::draft_throws(0x140be24a0, "content_140be24a0", fn_140be24a0 as *const (), Throws::Caught), // sub_140be24a0
    Replacement::draft(0x140be28d0, "content_140be28d0", fn_140be28d0 as *const ()), // sub_140be28d0
    Replacement::draft(0x140be2b80, "content_140be2b80", fn_140be2b80 as *const ()), // sub_140be2b80
    Replacement::draft(0x140be2ca0, "content_140be2ca0", fn_140be2ca0 as *const ()), // sub_140be2ca0
    Replacement::draft_throws(0x140be2dd0, "content_140be2dd0", fn_140be2dd0 as *const (), Throws::Discard), // sub_140be2dd0
    Replacement::draft_throws(0x140be2ef0, "content_140be2ef0", fn_140be2ef0 as *const (), Throws::Discard), // sub_140be2ef0
    Replacement::draft(0x140be30a0, "content_140be30a0", tpl_14406::<0x143426d28, 0x1434269c8, 0x1432bf2a0, 0x8, 0x1432bf2f0, 0x1432bf2b8, 0x14020f270> as *const ()), // sub_140be30a0
    Replacement::draft(0x140be3110, "content_140be3110", fn_140be3110 as *const ()), // sub_140be3110
    Replacement::draft(0x140be31b0, "content_140be31b0", fn_140be31b0 as *const ()), // sub_140be31b0
    Replacement::draft(0x140be3260, "content_140be3260", fn_140be3260 as *const ()), // sub_140be3260
    Replacement::draft(0x140be3420, "content_140be3420", fn_140be3420 as *const ()), // sub_140be3420
    Replacement::draft_throws(0x140be3650, "content_140be3650", fn_140be3650 as *const (), Throws::Discard), // sub_140be3650
    Replacement::draft(0x140be3720, "content_140be3720", fn_140be3720 as *const ()), // sub_140be3720
    Replacement::draft_throws(0x140be38d0, "content_140be38d0", fn_140be38d0 as *const (), Throws::Discard), // sub_140be38d0
    Replacement::draft(0x140be39b0, "content_140be39b0", fn_140be39b0 as *const ()), // sub_140be39b0
    Replacement::draft_throws(0x140be3b60, "content_140be3b60", fn_140be3b60 as *const (), Throws::Caught), // sub_140be3b60
    Replacement::draft_throws(0x140be3cd0, "content_140be3cd0", fn_140be3cd0 as *const (), Throws::Caught), // sub_140be3cd0
    Replacement::draft_throws(0x140be3e40, "content_140be3e40", fn_140be3e40 as *const (), Throws::Caught), // sub_140be3e40
    Replacement::draft(0x140be3fb0, "content_140be3fb0", fn_140be3fb0 as *const ()), // sub_140be3fb0
    Replacement::draft(0x140be4130, "content_140be4130", fn_140be4130 as *const ()), // sub_140be4130
    Replacement::draft(0x140be42c0, "content_140be42c0", fn_140be42c0 as *const ()), // sub_140be42c0
    Replacement::draft_throws(0x140be4460, "content_140be4460", fn_140be4460 as *const (), Throws::Discard), // sub_140be4460
    Replacement::draft_throws(0x140be45e0, "content_140be45e0", fn_140be45e0 as *const (), Throws::Discard), // sub_140be45e0
    Replacement::draft(0x140be4770, "content_140be4770", fn_140be4770 as *const ()), // sub_140be4770
    Replacement::draft(0x140be4840, "content_140be4840", fn_140be4840 as *const ()), // sub_140be4840
    Replacement::draft(0x140be4930, "content_140be4930", fn_140be4930 as *const ()), // sub_140be4930
    Replacement::draft(0x140be49a0, "content_140be49a0", fn_140be49a0 as *const ()), // sub_140be49a0
    Replacement::draft_throws(0x140be4cc0, "content_140be4cc0", fn_140be4cc0 as *const (), Throws::Discard), // sub_140be4cc0
    Replacement::draft(0x140be4fd0, "content_140be4fd0", fn_140be4fd0 as *const ()), // sub_140be4fd0
    Replacement::draft_throws(0x140be5150, "content_140be5150", fn_140be5150 as *const (), Throws::Discard), // sub_140be5150
    Replacement::draft(0x140be54d0, "content_140be54d0", fn_140be54d0 as *const ()), // sub_140be54d0
    Replacement::draft_throws(0x140be55e0, "content_140be55e0", fn_140be55e0 as *const (), Throws::Caught), // sub_140be55e0
    Replacement::draft_throws(0x140be5e80, "content_140be5e80", fn_140be5e80 as *const (), Throws::Caught), // sub_140be5e80
    Replacement::draft_throws(0x140be6600, "content_140be6600", fn_140be6600 as *const (), Throws::Caught), // sub_140be6600
    Replacement::draft(0x140be6dd0, "content_140be6dd0", fn_140be6dd0 as *const ()), // sub_140be6dd0
    Replacement::draft(0x140be6ec0, "content_140be6ec0", fn_140be6ec0 as *const ()), // sub_140be6ec0
    Replacement::draft(0x140be7070, "content_140be7070", fn_140be7070 as *const ()), // sub_140be7070
    Replacement::draft_throws(0x140be7230, "content_140be7230", fn_140be7230 as *const (), Throws::Discard), // sub_140be7230
    Replacement::draft_throws(0x140be7410, "content_140be7410", fn_140be7410 as *const (), Throws::Discard), // sub_140be7410
    Replacement::draft(0x140be7800, "content_140be7800", tpl_14899::<0x1432bee10, 0x8, 0x143876290, 0x1432bee50, 0x1432bee28, 0x1401de410, 0x1432bec00, 0x143874600, 0x18, 0x1432bebc8, 0x1432bebe0, 0x1432beb80, 0x1432becc8, 0x1432bece0, 0x1438744f8, 0x1430b0324, 0x1432be8c0, 0x1432be8b0> as *const ()), // sub_140be7800
    Replacement::draft(0x140be7910, "content_140be7910", fn_140be7910 as *const ()), // sub_140be7910
    Replacement::draft(0x140be7a70, "content_140be7a70", tpl_14899::<0x1432bee10, 0x8, 0x143874680, 0x1432bee50, 0x1432bee28, 0x1401de410, 0x1432bec00, 0x1438754e0, 0x18, 0x1432bebc8, 0x1432bebe0, 0x1432beb80, 0x1432becc8, 0x1432bece0, 0x143874620, 0x1430b0324, 0x1432be8c0, 0x1432be8b0> as *const ()), // sub_140be7a70
    Replacement::draft(0x140be7b80, "content_140be7b80", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432d09d0> as *const ()), // sub_140be7b80
    Replacement::draft(0x140be7be0, "content_140be7be0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432d09d0> as *const ()), // sub_140be7be0
    Replacement::draft_throws(0x140be7ce0, "content_140be7ce0", fn_140be7ce0 as *const (), Throws::Caught), // sub_140be7ce0
    Replacement::draft_throws(0x140be7d70, "content_140be7d70", tpl_6763::<0x14387ed40, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140be7d70
    Replacement::draft(0x140be7e20, "content_140be7e20", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432cbf10, 0x8> as *const ()), // sub_140be7e20
    Replacement::draft(0x140be7ea0, "content_140be7ea0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cbf10> as *const ()), // sub_140be7ea0
    Replacement::draft(0x140be7f30, "content_140be7f30", tpl_9031::<0x14387f088, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140be7f30
    Replacement::draft(0x140be7fa0, "content_140be7fa0", tpl_13868::<0x14387f088, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140be7fa0
    Replacement::draft_throws(0x140be8050, "content_140be8050", fn_140be8050 as *const (), Throws::Discard), // sub_140be8050
    Replacement::draft(0x140be80e0, "content_140be80e0", tpl_12080::<0x14387f088, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140be80e0
    Replacement::draft_throws(0x140be8160, "content_140be8160", fn_140be8160 as *const (), Throws::Discard), // sub_140be8160
    Replacement::draft_throws(0x140be8200, "content_140be8200", fn_140be8200 as *const (), Throws::Caught), // sub_140be8200
    Replacement::draft_throws(0x140be82e0, "content_140be82e0", fn_140be82e0 as *const (), Throws::Caught), // sub_140be82e0
    Replacement::draft_throws(0x140be83c0, "content_140be83c0", fn_140be83c0 as *const (), Throws::Caught), // sub_140be83c0
    Replacement::draft(0x140be84a0, "content_140be84a0", fn_140be84a0 as *const ()), // sub_140be84a0
    Replacement::draft(0x140be8510, "content_140be8510", tpl_12084::<0x14387ebc0, 0x14387f088, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0x1401e9860> as *const ()), // sub_140be8510
    Replacement::draft(0x140be8580, "content_140be8580", tpl_12080::<0x14387f088, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140be8580
    Replacement::draft_throws(0x140be8600, "content_140be8600", tpl_13873::<0x14387ebc0, 0x14387f088, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140be8600
    Replacement::draft_throws(0x140be8680, "content_140be8680", tpl_12133::<0x14387f088, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x143426ee0, 0x8, 0x1432be6a0, 0x1432be668, 0x1432be678, 0x14020f240> as *const (), Throws::Discard), // sub_140be8680
    Replacement::draft(0x140be8780, "content_140be8780", fn_140be8780 as *const ()), // sub_140be8780
    Replacement::draft(0x140be8830, "content_140be8830", fn_140be8830 as *const ()), // sub_140be8830
    Replacement::draft(0x140be88f0, "content_140be88f0", tpl_6695::<0x14387ed50, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140be88f0
    Replacement::draft(0x140be8960, "content_140be8960", fn_140be8960 as *const ()), // sub_140be8960
    Replacement::draft_throws(0x140be8df0, "content_140be8df0", fn_140be8df0 as *const (), Throws::Caught), // sub_140be8df0
    Replacement::draft(0x140be9620, "content_140be9620", fn_140be9620 as *const ()), // sub_140be9620
    Replacement::draft_throws(0x140be9a60, "content_140be9a60", fn_140be9a60 as *const (), Throws::Discard), // sub_140be9a60
    Replacement::draft(0x140be9d40, "content_140be9d40", fn_140be9d40 as *const ()), // sub_140be9d40
    Replacement::draft_throws(0x140be9e80, "content_140be9e80", fn_140be9e80 as *const (), Throws::Caught), // sub_140be9e80
    Replacement::draft_throws(0x140bea810, "content_140bea810", fn_140bea810 as *const (), Throws::Caught), // sub_140bea810
    Replacement::draft_throws(0x140beb090, "content_140beb090", fn_140beb090 as *const (), Throws::Caught), // sub_140beb090
    Replacement::draft(0x140beb960, "content_140beb960", fn_140beb960 as *const ()), // sub_140beb960
    Replacement::draft(0x140bec230, "content_140bec230", fn_140bec230 as *const ()), // sub_140bec230
    Replacement::draft(0x140bec5a0, "content_140bec5a0", fn_140bec5a0 as *const ()), // sub_140bec5a0
    Replacement::draft_throws(0x140bec920, "content_140bec920", fn_140bec920 as *const (), Throws::Discard), // sub_140bec920
    Replacement::draft_throws(0x140becce0, "content_140becce0", fn_140becce0 as *const (), Throws::Discard), // sub_140becce0
    Replacement::draft(0x140bed230, "content_140bed230", fn_140bed230 as *const ()), // sub_140bed230
    Replacement::draft(0x140bed420, "content_140bed420", tpl_14848::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x28, 0x1432cd1a0, 0x8, 0x20> as *const ()), // sub_140bed420
    Replacement::draft(0x140bed4c0, "content_140bed4c0", tpl_13753::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14, 0x1432cd1a0, 0xffffffffffffffec, 0xfffffffffffffff4, 0xfffffffffffffffc, 0x4> as *const ()), // sub_140bed4c0
    Replacement::draft(0x140bed570, "content_140bed570", fn_140bed570 as *const ()), // sub_140bed570
    Replacement::draft(0x140bed610, "content_140bed610", fn_140bed610 as *const ()), // sub_140bed610
    Replacement::draft_throws(0x140bed840, "content_140bed840", fn_140bed840 as *const (), Throws::Discard), // sub_140bed840
    Replacement::draft(0x140beda20, "content_140beda20", fn_140beda20 as *const ()), // sub_140beda20
    Replacement::draft_throws(0x140bedb90, "content_140bedb90", fn_140bedb90 as *const (), Throws::Discard), // sub_140bedb90
    Replacement::draft_throws(0x140beddb0, "content_140beddb0", fn_140beddb0 as *const (), Throws::Caught), // sub_140beddb0
    Replacement::draft_throws(0x140bedf60, "content_140bedf60", fn_140bedf60 as *const (), Throws::Caught), // sub_140bedf60
    Replacement::draft_throws(0x140bee110, "content_140bee110", fn_140bee110 as *const (), Throws::Caught), // sub_140bee110
    Replacement::draft(0x140bee2c0, "content_140bee2c0", fn_140bee2c0 as *const ()), // sub_140bee2c0
    Replacement::draft(0x140bee3a0, "content_140bee3a0", fn_140bee3a0 as *const ()), // sub_140bee3a0
    Replacement::draft(0x140bee4d0, "content_140bee4d0", fn_140bee4d0 as *const ()), // sub_140bee4d0
    Replacement::draft_throws(0x140bee610, "content_140bee610", fn_140bee610 as *const (), Throws::Discard), // sub_140bee610
    Replacement::draft_throws(0x140bee750, "content_140bee750", fn_140bee750 as *const (), Throws::Discard), // sub_140bee750
    Replacement::draft(0x140bee920, "content_140bee920", tpl_14417::<0x20, 0x1426cdcb0, 0x140c339a0> as *const ()), // sub_140bee920
    Replacement::draft(0x140bee990, "content_140bee990", tpl_14417::<0x20, 0x1432d53a8, 0x140c339a0> as *const ()), // sub_140bee990
    Replacement::draft(0x140beea00, "content_140beea00", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432c4328, 0x8> as *const ()), // sub_140beea00
    Replacement::draft(0x140beea80, "content_140beea80", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c4328> as *const ()), // sub_140beea80
    Replacement::draft(0x140beeb10, "content_140beeb10", tpl_9031::<0x14387ec58, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140beeb10
    Replacement::draft(0x140beeb80, "content_140beeb80", tpl_13868::<0x14387ec58, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140beeb80
    Replacement::draft_throws(0x140beec30, "content_140beec30", fn_140beec30 as *const (), Throws::Discard), // sub_140beec30
    Replacement::draft(0x140beecc0, "content_140beecc0", tpl_12080::<0x14387ec58, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140beecc0
    Replacement::draft_throws(0x140beed40, "content_140beed40", fn_140beed40 as *const (), Throws::Discard), // sub_140beed40
    Replacement::draft_throws(0x140beede0, "content_140beede0", fn_140beede0 as *const (), Throws::Caught), // sub_140beede0
    Replacement::draft_throws(0x140beeec0, "content_140beeec0", fn_140beeec0 as *const (), Throws::Caught), // sub_140beeec0
    Replacement::draft_throws(0x140beefa0, "content_140beefa0", fn_140beefa0 as *const (), Throws::Caught), // sub_140beefa0
    Replacement::draft(0x140bef080, "content_140bef080", fn_140bef080 as *const ()), // sub_140bef080
    Replacement::draft(0x140bef0f0, "content_140bef0f0", tpl_12084::<0x14387ea50, 0x14387ec58, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0x1401e9860> as *const ()), // sub_140bef0f0
    Replacement::draft(0x140bef160, "content_140bef160", tpl_12080::<0x14387ec58, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bef160
    Replacement::draft_throws(0x140bef1e0, "content_140bef1e0", tpl_13873::<0x14387ea50, 0x14387ec58, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140bef1e0
    Replacement::draft_throws(0x140bef260, "content_140bef260", tpl_12133::<0x14387ec58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8, 0x143426d48, 0x8, 0x1432d50c8, 0x1432d5218, 0x1432d51f8, 0x14020f240> as *const (), Throws::Discard), // sub_140bef260
    Replacement::draft(0x140bef360, "content_140bef360", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432c26a0, 0x8> as *const ()), // sub_140bef360
    Replacement::draft(0x140bef3e0, "content_140bef3e0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c26a0> as *const ()), // sub_140bef3e0
    Replacement::draft(0x140bef470, "content_140bef470", tpl_13738::<0x14387edd0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bef470
    Replacement::draft(0x140bef4e0, "content_140bef4e0", fn_140bef4e0 as *const ()), // sub_140bef4e0
    Replacement::draft_throws(0x140bef5d0, "content_140bef5d0", fn_140bef5d0 as *const (), Throws::Discard), // sub_140bef5d0
    Replacement::draft(0x140bef6c0, "content_140bef6c0", tpl_14930::<0x14387edd0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xc, 0x80, 0xd, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bef6c0
    Replacement::draft_throws(0x140bef760, "content_140bef760", fn_140bef760 as *const (), Throws::Discard), // sub_140bef760
    Replacement::draft_throws(0x140bef860, "content_140bef860", fn_140bef860 as *const (), Throws::Caught), // sub_140bef860
    Replacement::draft_throws(0x140bef9c0, "content_140bef9c0", fn_140bef9c0 as *const (), Throws::Caught), // sub_140bef9c0
    Replacement::draft_throws(0x140befb20, "content_140befb20", fn_140befb20 as *const (), Throws::Caught), // sub_140befb20
    Replacement::draft(0x140befc80, "content_140befc80", fn_140befc80 as *const ()), // sub_140befc80
    Replacement::draft(0x140befd00, "content_140befd00", fn_140befd00 as *const ()), // sub_140befd00
    Replacement::draft(0x140befda0, "content_140befda0", tpl_14930::<0x14387edd0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xe8, 0xc, 0x120, 0xd, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140befda0
    Replacement::draft_throws(0x140befe50, "content_140befe50", fn_140befe50 as *const (), Throws::Discard), // sub_140befe50
    Replacement::draft_throws(0x140beff00, "content_140beff00", fn_140beff00 as *const (), Throws::Discard), // sub_140beff00
    Replacement::draft(0x140bf0080, "content_140bf0080", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432cc700> as *const ()), // sub_140bf0080
    Replacement::draft(0x140bf00e0, "content_140bf00e0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432cc700> as *const ()), // sub_140bf00e0
    Replacement::draft(0x140bf0170, "content_140bf0170", tpl_9031::<0x14387f0f8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf0170
    Replacement::draft_throws(0x140bf01e0, "content_140bf01e0", fn_140bf01e0 as *const (), Throws::Caught), // sub_140bf01e0
    Replacement::draft_throws(0x140bf0270, "content_140bf0270", tpl_6763::<0x14387f0f8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140bf0270
    Replacement::draft(0x140bf0320, "content_140bf0320", fn_140bf0320 as *const ()), // sub_140bf0320
    Replacement::draft(0x140bf03c0, "content_140bf03c0", fn_140bf03c0 as *const ()), // sub_140bf03c0
    Replacement::draft(0x140bf0480, "content_140bf0480", tpl_9031::<0x14387e9b0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf0480
    Replacement::draft(0x140bf04f0, "content_140bf04f0", fn_140bf04f0 as *const ()), // sub_140bf04f0
    Replacement::draft_throws(0x140bf0880, "content_140bf0880", fn_140bf0880 as *const (), Throws::Discard), // sub_140bf0880
    Replacement::draft(0x140bf0b70, "content_140bf0b70", fn_140bf0b70 as *const ()), // sub_140bf0b70
    Replacement::draft_throws(0x140bf0cf0, "content_140bf0cf0", fn_140bf0cf0 as *const (), Throws::Discard), // sub_140bf0cf0
    Replacement::draft(0x140bf10b0, "content_140bf10b0", fn_140bf10b0 as *const ()), // sub_140bf10b0
    Replacement::draft_throws(0x140bf11c0, "content_140bf11c0", fn_140bf11c0 as *const (), Throws::Caught), // sub_140bf11c0
    Replacement::draft_throws(0x140bf1940, "content_140bf1940", fn_140bf1940 as *const (), Throws::Caught), // sub_140bf1940
    Replacement::draft_throws(0x140bf20a0, "content_140bf20a0", fn_140bf20a0 as *const (), Throws::Caught), // sub_140bf20a0
    Replacement::draft(0x140bf27f0, "content_140bf27f0", fn_140bf27f0 as *const ()), // sub_140bf27f0
    Replacement::draft(0x140bf28d0, "content_140bf28d0", fn_140bf28d0 as *const ()), // sub_140bf28d0
    Replacement::draft(0x140bf2a80, "content_140bf2a80", fn_140bf2a80 as *const ()), // sub_140bf2a80
    Replacement::draft_throws(0x140bf2c40, "content_140bf2c40", fn_140bf2c40 as *const (), Throws::Discard), // sub_140bf2c40
    Replacement::draft_throws(0x140bf2e80, "content_140bf2e80", fn_140bf2e80 as *const (), Throws::Discard), // sub_140bf2e80
    Replacement::draft(0x140bf34a0, "content_140bf34a0", fn_140bf34a0 as *const ()), // sub_140bf34a0
    Replacement::draft(0x140bf39d0, "content_140bf39d0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1432d0560> as *const ()), // sub_140bf39d0
    Replacement::draft(0x140bf3a30, "content_140bf3a30", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432d0560> as *const ()), // sub_140bf3a30
    Replacement::draft(0x140bf3ac0, "content_140bf3ac0", tpl_9031::<0x14387eb60, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf3ac0
    Replacement::draft_throws(0x140bf3b30, "content_140bf3b30", fn_140bf3b30 as *const (), Throws::Caught), // sub_140bf3b30
    Replacement::draft_throws(0x140bf3bc0, "content_140bf3bc0", tpl_6763::<0x14387eb60, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140bf3bc0
    Replacement::draft(0x140bf3c70, "content_140bf3c70", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432c68c0, 0x8> as *const ()), // sub_140bf3c70
    Replacement::draft(0x140bf3cf0, "content_140bf3cf0", tpl_5000::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432c68c0> as *const ()), // sub_140bf3cf0
    Replacement::draft(0x140bf3d80, "content_140bf3d80", tpl_9031::<0x14387ed60, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf3d80
    Replacement::draft(0x140bf3df0, "content_140bf3df0", fn_140bf3df0 as *const ()), // sub_140bf3df0
    Replacement::draft_throws(0x140bf3ed0, "content_140bf3ed0", fn_140bf3ed0 as *const (), Throws::Discard), // sub_140bf3ed0
    Replacement::draft(0x140bf3f90, "content_140bf3f90", tpl_14955::<0x14387ed60, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf3f90
    Replacement::draft_throws(0x140bf4020, "content_140bf4020", fn_140bf4020 as *const (), Throws::Discard), // sub_140bf4020
    Replacement::draft_throws(0x140bf4100, "content_140bf4100", fn_140bf4100 as *const (), Throws::Caught), // sub_140bf4100
    Replacement::draft_throws(0x140bf4250, "content_140bf4250", fn_140bf4250 as *const (), Throws::Caught), // sub_140bf4250
    Replacement::draft_throws(0x140bf43a0, "content_140bf43a0", fn_140bf43a0 as *const (), Throws::Caught), // sub_140bf43a0
    Replacement::draft(0x140bf44f0, "content_140bf44f0", fn_140bf44f0 as *const ()), // sub_140bf44f0
    Replacement::draft(0x140bf4570, "content_140bf4570", fn_140bf4570 as *const ()), // sub_140bf4570
    Replacement::draft(0x140bf4600, "content_140bf4600", tpl_14955::<0x14387ed60, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf4600
    Replacement::draft_throws(0x140bf4690, "content_140bf4690", fn_140bf4690 as *const (), Throws::Discard), // sub_140bf4690
    Replacement::draft_throws(0x140bf4730, "content_140bf4730", fn_140bf4730 as *const (), Throws::Discard), // sub_140bf4730
    Replacement::draft(0x140bf4890, "content_140bf4890", fn_140bf4890 as *const ()), // sub_140bf4890
    Replacement::draft(0x140bf4950, "content_140bf4950", fn_140bf4950 as *const ()), // sub_140bf4950
    Replacement::draft(0x140bf49f0, "content_140bf49f0", fn_140bf49f0 as *const ()), // sub_140bf49f0
    Replacement::draft(0x140bf4ab0, "content_140bf4ab0", fn_140bf4ab0 as *const ()), // sub_140bf4ab0
    Replacement::draft(0x140bf4c80, "content_140bf4c80", fn_140bf4c80 as *const ()), // sub_140bf4c80
    Replacement::draft_throws(0x140bf5160, "content_140bf5160", fn_140bf5160 as *const (), Throws::Caught), // sub_140bf5160
    Replacement::draft(0x140bf5660, "content_140bf5660", fn_140bf5660 as *const ()), // sub_140bf5660
    Replacement::draft_throws(0x140bf59b0, "content_140bf59b0", fn_140bf59b0 as *const (), Throws::Discard), // sub_140bf59b0
    Replacement::draft(0x140bf5e70, "content_140bf5e70", fn_140bf5e70 as *const ()), // sub_140bf5e70
    Replacement::draft_throws(0x140bf5f90, "content_140bf5f90", fn_140bf5f90 as *const (), Throws::Caught), // sub_140bf5f90
    Replacement::draft_throws(0x140bf67b0, "content_140bf67b0", fn_140bf67b0 as *const (), Throws::Caught), // sub_140bf67b0
    Replacement::draft_throws(0x140bf6f70, "content_140bf6f70", fn_140bf6f70 as *const (), Throws::Caught), // sub_140bf6f70
    Replacement::draft(0x140bf7720, "content_140bf7720", fn_140bf7720 as *const ()), // sub_140bf7720
    Replacement::draft(0x140bf7ae0, "content_140bf7ae0", fn_140bf7ae0 as *const ()), // sub_140bf7ae0
    Replacement::draft(0x140bf7e20, "content_140bf7e20", fn_140bf7e20 as *const ()), // sub_140bf7e20
    Replacement::draft_throws(0x140bf8170, "content_140bf8170", fn_140bf8170 as *const (), Throws::Discard), // sub_140bf8170
    Replacement::draft_throws(0x140bf8520, "content_140bf8520", fn_140bf8520 as *const (), Throws::Discard), // sub_140bf8520
    Replacement::draft(0x140bf8a80, "content_140bf8a80", fn_140bf8a80 as *const ()), // sub_140bf8a80
    Replacement::draft(0x140bf8e80, "content_140bf8e80", fn_140bf8e80 as *const ()), // sub_140bf8e80
    Replacement::draft(0x140bf8f30, "content_140bf8f30", fn_140bf8f30 as *const ()), // sub_140bf8f30
    Replacement::draft(0x140bf9010, "content_140bf9010", tpl_9161::<0x14387f0f0, 0x14243b0b0, 0x1406e01b0, 0x10, 0x1407de100, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf9010
    Replacement::draft(0x140bf9090, "content_140bf9090", fn_140bf9090 as *const ()), // sub_140bf9090
    Replacement::draft_throws(0x140bf9180, "content_140bf9180", fn_140bf9180 as *const (), Throws::Discard), // sub_140bf9180
    Replacement::draft(0x140bf9250, "content_140bf9250", tpl_14984::<0x14387f0f0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0x1407de580, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf9250
    Replacement::draft_throws(0x140bf92f0, "content_140bf92f0", fn_140bf92f0 as *const (), Throws::Discard), // sub_140bf92f0
    Replacement::draft(0x140bf93e0, "content_140bf93e0", fn_140bf93e0 as *const ()), // sub_140bf93e0
    Replacement::draft_throws(0x140bf9480, "content_140bf9480", fn_140bf9480 as *const (), Throws::Caught), // sub_140bf9480
    Replacement::draft_throws(0x140bf95d0, "content_140bf95d0", fn_140bf95d0 as *const (), Throws::Caught), // sub_140bf95d0
    Replacement::draft_throws(0x140bf9720, "content_140bf9720", fn_140bf9720 as *const (), Throws::Caught), // sub_140bf9720
    Replacement::draft(0x140bf9870, "content_140bf9870", fn_140bf9870 as *const ()), // sub_140bf9870
    Replacement::draft(0x140bf9900, "content_140bf9900", fn_140bf9900 as *const ()), // sub_140bf9900
    Replacement::draft(0x140bf9990, "content_140bf9990", tpl_14984::<0x14387f0f0, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x1407df7d0, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140bf9990
];
